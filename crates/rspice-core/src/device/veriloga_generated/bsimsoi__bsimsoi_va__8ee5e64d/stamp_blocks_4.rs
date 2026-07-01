#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        locals: &mut StampLocals,
    ) {
        let assign25110_e19112: f64 = if locals.var_ehlis < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1614 = assign25110_e19112;

        let (assign25120_e19121, assign25120_e19121_d_n3, assign25120_e19121_d_n4, assign25120_e19121_d_n5, assign25120_e19121_d_n6, assign25120_e19121_d_n7, assign25120_e19121_d_n8, assign25120_e19121_d_n9, assign25120_e19121_d_n10, assign25120_e19121_d_n11, assign25120_e19121_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1614 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlis, locals.var_ehlis_dn3, locals.var_ehlis_dn4, locals.var_ehlis_dn5, locals.var_ehlis_dn6, locals.var_ehlis_dn7, locals.var_ehlis_dn8, locals.var_ehlis_dn9, locals.var_ehlis_dn10, locals.var_ehlis_dn11, locals.var_ehlis_dn12,)
    }
};
        locals.var_ehlis = assign25120_e19121;
        locals.var_ehlis_dn3 = assign25120_e19121_d_n3;
        locals.var_ehlis_dn4 = assign25120_e19121_d_n4;
        locals.var_ehlis_dn5 = assign25120_e19121_d_n5;
        locals.var_ehlis_dn6 = assign25120_e19121_d_n6;
        locals.var_ehlis_dn7 = assign25120_e19121_d_n7;
        locals.var_ehlis_dn8 = assign25120_e19121_d_n8;
        locals.var_ehlis_dn9 = assign25120_e19121_d_n9;
        locals.var_ehlis_dn10 = assign25120_e19121_d_n10;
        locals.var_ehlis_dn11 = assign25120_e19121_d_n11;
        locals.var_ehlis_dn12 = assign25120_e19121_d_n12;

        let (assign25130_e19130, assign25130_e19130_d_n3, assign25130_e19130_d_n4, assign25130_e19130_d_n5, assign25130_e19130_d_n6, assign25130_e19130_d_n7, assign25130_e19130_d_n8, assign25130_e19130_d_n9, assign25130_e19130_d_n10, assign25130_e19130_d_n11, assign25130_e19130_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1614 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlisfactor, locals.var_ehlisfactor_dn3, locals.var_ehlisfactor_dn4, locals.var_ehlisfactor_dn5, locals.var_ehlisfactor_dn6, locals.var_ehlisfactor_dn7, locals.var_ehlisfactor_dn8, locals.var_ehlisfactor_dn9, locals.var_ehlisfactor_dn10, locals.var_ehlisfactor_dn11, locals.var_ehlisfactor_dn12,)
    }
};
        locals.var_ehlisfactor = assign25130_e19130;
        locals.var_ehlisfactor_dn3 = assign25130_e19130_d_n3;
        locals.var_ehlisfactor_dn4 = assign25130_e19130_d_n4;
        locals.var_ehlisfactor_dn5 = assign25130_e19130_d_n5;
        locals.var_ehlisfactor_dn6 = assign25130_e19130_d_n6;
        locals.var_ehlisfactor_dn7 = assign25130_e19130_d_n7;
        locals.var_ehlisfactor_dn8 = assign25130_e19130_d_n8;
        locals.var_ehlisfactor_dn9 = assign25130_e19130_d_n9;
        locals.var_ehlisfactor_dn10 = assign25130_e19130_d_n10;
        locals.var_ehlisfactor_dn11 = assign25130_e19130_d_n11;
        locals.var_ehlisfactor_dn12 = assign25130_e19130_d_n12;

        let (assign25140_e19145, assign25140_e19145_d_n3, assign25140_e19145_d_n4, assign25140_e19145_d_n5, assign25140_e19145_d_n6, assign25140_e19145_d_n7, assign25140_e19145_d_n8, assign25140_e19145_d_n9, assign25140_e19145_d_n10, assign25140_e19145_d_n11, assign25140_e19145_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1614 == 0.0)) {
        let assign25140_e19141: f64 = (1.0 + locals.var_ehlis);
        let assign25140_e19142: f64 = (assign25140_e19141).sqrt();
        let assign25140_e19143: f64 = (1.0 / assign25140_e19142);
        (assign25140_e19143, (-((locals.var_ehlis_dn3 / (2.0 * assign25140_e19142)) / (assign25140_e19142 * assign25140_e19142))), (-((locals.var_ehlis_dn4 / (2.0 * assign25140_e19142)) / (assign25140_e19142 * assign25140_e19142))), (-((locals.var_ehlis_dn5 / (2.0 * assign25140_e19142)) / (assign25140_e19142 * assign25140_e19142))), (-((locals.var_ehlis_dn6 / (2.0 * assign25140_e19142)) / (assign25140_e19142 * assign25140_e19142))), (-((locals.var_ehlis_dn7 / (2.0 * assign25140_e19142)) / (assign25140_e19142 * assign25140_e19142))), (-((locals.var_ehlis_dn8 / (2.0 * assign25140_e19142)) / (assign25140_e19142 * assign25140_e19142))), (-((locals.var_ehlis_dn9 / (2.0 * assign25140_e19142)) / (assign25140_e19142 * assign25140_e19142))), (-((locals.var_ehlis_dn10 / (2.0 * assign25140_e19142)) / (assign25140_e19142 * assign25140_e19142))), (-((locals.var_ehlis_dn11 / (2.0 * assign25140_e19142)) / (assign25140_e19142 * assign25140_e19142))), (-((locals.var_ehlis_dn12 / (2.0 * assign25140_e19142)) / (assign25140_e19142 * assign25140_e19142))),)
    } else {
        (locals.var_ehlisfactor, locals.var_ehlisfactor_dn3, locals.var_ehlisfactor_dn4, locals.var_ehlisfactor_dn5, locals.var_ehlisfactor_dn6, locals.var_ehlisfactor_dn7, locals.var_ehlisfactor_dn8, locals.var_ehlisfactor_dn9, locals.var_ehlisfactor_dn10, locals.var_ehlisfactor_dn11, locals.var_ehlisfactor_dn12,)
    }
};
        locals.var_ehlisfactor = assign25140_e19145;
        locals.var_ehlisfactor_dn3 = assign25140_e19145_d_n3;
        locals.var_ehlisfactor_dn4 = assign25140_e19145_d_n4;
        locals.var_ehlisfactor_dn5 = assign25140_e19145_d_n5;
        locals.var_ehlisfactor_dn6 = assign25140_e19145_d_n6;
        locals.var_ehlisfactor_dn7 = assign25140_e19145_d_n7;
        locals.var_ehlisfactor_dn8 = assign25140_e19145_d_n8;
        locals.var_ehlisfactor_dn9 = assign25140_e19145_d_n9;
        locals.var_ehlisfactor_dn10 = assign25140_e19145_d_n10;
        locals.var_ehlisfactor_dn11 = assign25140_e19145_d_n11;
        locals.var_ehlisfactor_dn12 = assign25140_e19145_d_n12;

        let (assign25150_e19156, assign25150_e19156_d_n3, assign25150_e19156_d_n4, assign25150_e19156_d_n5, assign25150_e19156_d_n6, assign25150_e19156_d_n7, assign25150_e19156_d_n8, assign25150_e19156_d_n9, assign25150_e19156_d_n10, assign25150_e19156_d_n11, assign25150_e19156_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign25150_e19153: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign25150_e19154: f64 = (locals.var_ahlid * assign25150_e19153);
        (assign25150_e19154, ((locals.var_ahlid_dn3 * assign25150_e19153) + (locals.var_ahlid * locals.var_expvbdnvtm_dn3)), ((locals.var_ahlid_dn4 * assign25150_e19153) + (locals.var_ahlid * locals.var_expvbdnvtm_dn4)), ((locals.var_ahlid_dn5 * assign25150_e19153) + (locals.var_ahlid * locals.var_expvbdnvtm_dn5)), ((locals.var_ahlid_dn6 * assign25150_e19153) + (locals.var_ahlid * locals.var_expvbdnvtm_dn6)), ((locals.var_ahlid_dn7 * assign25150_e19153) + (locals.var_ahlid * locals.var_expvbdnvtm_dn7)), ((locals.var_ahlid_dn8 * assign25150_e19153) + (locals.var_ahlid * locals.var_expvbdnvtm_dn8)), ((locals.var_ahlid_dn9 * assign25150_e19153) + (locals.var_ahlid * locals.var_expvbdnvtm_dn9)), ((locals.var_ahlid_dn10 * assign25150_e19153) + (locals.var_ahlid * locals.var_expvbdnvtm_dn10)), ((locals.var_ahlid_dn11 * assign25150_e19153) + (locals.var_ahlid * locals.var_expvbdnvtm_dn11)), ((locals.var_ahlid_dn12 * assign25150_e19153) + (locals.var_ahlid * locals.var_expvbdnvtm_dn12)),)
    } else {
        (locals.var_ehlid, locals.var_ehlid_dn3, locals.var_ehlid_dn4, locals.var_ehlid_dn5, locals.var_ehlid_dn6, locals.var_ehlid_dn7, locals.var_ehlid_dn8, locals.var_ehlid_dn9, locals.var_ehlid_dn10, locals.var_ehlid_dn11, locals.var_ehlid_dn12,)
    }
};
        locals.var_ehlid = assign25150_e19156;
        locals.var_ehlid_dn3 = assign25150_e19156_d_n3;
        locals.var_ehlid_dn4 = assign25150_e19156_d_n4;
        locals.var_ehlid_dn5 = assign25150_e19156_d_n5;
        locals.var_ehlid_dn6 = assign25150_e19156_d_n6;
        locals.var_ehlid_dn7 = assign25150_e19156_d_n7;
        locals.var_ehlid_dn8 = assign25150_e19156_d_n8;
        locals.var_ehlid_dn9 = assign25150_e19156_d_n9;
        locals.var_ehlid_dn10 = assign25150_e19156_d_n10;
        locals.var_ehlid_dn11 = assign25150_e19156_d_n11;
        locals.var_ehlid_dn12 = assign25150_e19156_d_n12;

        let assign25160_e19159: f64 = if locals.var_ehlid < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1615 = assign25160_e19159;

        let (assign25170_e19168, assign25170_e19168_d_n3, assign25170_e19168_d_n4, assign25170_e19168_d_n5, assign25170_e19168_d_n6, assign25170_e19168_d_n7, assign25170_e19168_d_n8, assign25170_e19168_d_n9, assign25170_e19168_d_n10, assign25170_e19168_d_n11, assign25170_e19168_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1615 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlid, locals.var_ehlid_dn3, locals.var_ehlid_dn4, locals.var_ehlid_dn5, locals.var_ehlid_dn6, locals.var_ehlid_dn7, locals.var_ehlid_dn8, locals.var_ehlid_dn9, locals.var_ehlid_dn10, locals.var_ehlid_dn11, locals.var_ehlid_dn12,)
    }
};
        locals.var_ehlid = assign25170_e19168;
        locals.var_ehlid_dn3 = assign25170_e19168_d_n3;
        locals.var_ehlid_dn4 = assign25170_e19168_d_n4;
        locals.var_ehlid_dn5 = assign25170_e19168_d_n5;
        locals.var_ehlid_dn6 = assign25170_e19168_d_n6;
        locals.var_ehlid_dn7 = assign25170_e19168_d_n7;
        locals.var_ehlid_dn8 = assign25170_e19168_d_n8;
        locals.var_ehlid_dn9 = assign25170_e19168_d_n9;
        locals.var_ehlid_dn10 = assign25170_e19168_d_n10;
        locals.var_ehlid_dn11 = assign25170_e19168_d_n11;
        locals.var_ehlid_dn12 = assign25170_e19168_d_n12;

        let (assign25180_e19177, assign25180_e19177_d_n3, assign25180_e19177_d_n4, assign25180_e19177_d_n5, assign25180_e19177_d_n6, assign25180_e19177_d_n7, assign25180_e19177_d_n8, assign25180_e19177_d_n9, assign25180_e19177_d_n10, assign25180_e19177_d_n11, assign25180_e19177_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1615 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11, locals.var_ehlidfactor_dn12,)
    }
};
        locals.var_ehlidfactor = assign25180_e19177;
        locals.var_ehlidfactor_dn3 = assign25180_e19177_d_n3;
        locals.var_ehlidfactor_dn4 = assign25180_e19177_d_n4;
        locals.var_ehlidfactor_dn5 = assign25180_e19177_d_n5;
        locals.var_ehlidfactor_dn6 = assign25180_e19177_d_n6;
        locals.var_ehlidfactor_dn7 = assign25180_e19177_d_n7;
        locals.var_ehlidfactor_dn8 = assign25180_e19177_d_n8;
        locals.var_ehlidfactor_dn9 = assign25180_e19177_d_n9;
        locals.var_ehlidfactor_dn10 = assign25180_e19177_d_n10;
        locals.var_ehlidfactor_dn11 = assign25180_e19177_d_n11;
        locals.var_ehlidfactor_dn12 = assign25180_e19177_d_n12;

        let (assign25190_e19192, assign25190_e19192_d_n3, assign25190_e19192_d_n4, assign25190_e19192_d_n5, assign25190_e19192_d_n6, assign25190_e19192_d_n7, assign25190_e19192_d_n8, assign25190_e19192_d_n9, assign25190_e19192_d_n10, assign25190_e19192_d_n11, assign25190_e19192_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1615 == 0.0)) {
        let assign25190_e19188: f64 = (1.0 + locals.var_ehlid);
        let assign25190_e19189: f64 = (assign25190_e19188).sqrt();
        let assign25190_e19190: f64 = (1.0 / assign25190_e19189);
        (assign25190_e19190, (-((locals.var_ehlid_dn3 / (2.0 * assign25190_e19189)) / (assign25190_e19189 * assign25190_e19189))), (-((locals.var_ehlid_dn4 / (2.0 * assign25190_e19189)) / (assign25190_e19189 * assign25190_e19189))), (-((locals.var_ehlid_dn5 / (2.0 * assign25190_e19189)) / (assign25190_e19189 * assign25190_e19189))), (-((locals.var_ehlid_dn6 / (2.0 * assign25190_e19189)) / (assign25190_e19189 * assign25190_e19189))), (-((locals.var_ehlid_dn7 / (2.0 * assign25190_e19189)) / (assign25190_e19189 * assign25190_e19189))), (-((locals.var_ehlid_dn8 / (2.0 * assign25190_e19189)) / (assign25190_e19189 * assign25190_e19189))), (-((locals.var_ehlid_dn9 / (2.0 * assign25190_e19189)) / (assign25190_e19189 * assign25190_e19189))), (-((locals.var_ehlid_dn10 / (2.0 * assign25190_e19189)) / (assign25190_e19189 * assign25190_e19189))), (-((locals.var_ehlid_dn11 / (2.0 * assign25190_e19189)) / (assign25190_e19189 * assign25190_e19189))), (-((locals.var_ehlid_dn12 / (2.0 * assign25190_e19189)) / (assign25190_e19189 * assign25190_e19189))),)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11, locals.var_ehlidfactor_dn12,)
    }
};
        locals.var_ehlidfactor = assign25190_e19192;
        locals.var_ehlidfactor_dn3 = assign25190_e19192_d_n3;
        locals.var_ehlidfactor_dn4 = assign25190_e19192_d_n4;
        locals.var_ehlidfactor_dn5 = assign25190_e19192_d_n5;
        locals.var_ehlidfactor_dn6 = assign25190_e19192_d_n6;
        locals.var_ehlidfactor_dn7 = assign25190_e19192_d_n7;
        locals.var_ehlidfactor_dn8 = assign25190_e19192_d_n8;
        locals.var_ehlidfactor_dn9 = assign25190_e19192_d_n9;
        locals.var_ehlidfactor_dn10 = assign25190_e19192_d_n10;
        locals.var_ehlidfactor_dn11 = assign25190_e19192_d_n11;
        locals.var_ehlidfactor_dn12 = assign25190_e19192_d_n12;

        let (assign25200_e19201, assign25200_e19201_d_n3, assign25200_e19201_d_n4, assign25200_e19201_d_n5, assign25200_e19201_d_n6, assign25200_e19201_d_n7, assign25200_e19201_d_n8, assign25200_e19201_d_n9, assign25200_e19201_d_n10, assign25200_e19201_d_n11, assign25200_e19201_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign25200_e19199: f64 = (1.0 - locals.var_pparam_b4soiarfabjt);
        (assign25200_e19199, (-locals.var_pparam_b4soiarfabjt_dn3), (-locals.var_pparam_b4soiarfabjt_dn4), (-locals.var_pparam_b4soiarfabjt_dn5), (-locals.var_pparam_b4soiarfabjt_dn6), (-locals.var_pparam_b4soiarfabjt_dn7), (-locals.var_pparam_b4soiarfabjt_dn8), (-locals.var_pparam_b4soiarfabjt_dn9), (-locals.var_pparam_b4soiarfabjt_dn10), (-locals.var_pparam_b4soiarfabjt_dn11), (-locals.var_pparam_b4soiarfabjt_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign25200_e19201;
        locals.var_t0__blk1144_dn3 = assign25200_e19201_d_n3;
        locals.var_t0__blk1144_dn4 = assign25200_e19201_d_n4;
        locals.var_t0__blk1144_dn5 = assign25200_e19201_d_n5;
        locals.var_t0__blk1144_dn6 = assign25200_e19201_d_n6;
        locals.var_t0__blk1144_dn7 = assign25200_e19201_d_n7;
        locals.var_t0__blk1144_dn8 = assign25200_e19201_d_n8;
        locals.var_t0__blk1144_dn9 = assign25200_e19201_d_n9;
        locals.var_t0__blk1144_dn10 = assign25200_e19201_d_n10;
        locals.var_t0__blk1144_dn11 = assign25200_e19201_d_n11;
        locals.var_t0__blk1144_dn12 = assign25200_e19201_d_n12;

        let (assign25210_e19212, assign25210_e19212_d_n3, assign25210_e19212_d_n4, assign25210_e19212_d_n5, assign25210_e19212_d_n6, assign25210_e19212_d_n7, assign25210_e19212_d_n8, assign25210_e19212_d_n9, assign25210_e19212_d_n10, assign25210_e19212_d_n11, assign25210_e19212_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign25210_e19208: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign25210_e19210: f64 = (assign25210_e19208 * locals.var_pparam_b4soilratio);
        (assign25210_e19210, ((((locals.var_wtsi_dn3 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn3)) * locals.var_pparam_b4soilratio) + (assign25210_e19208 * locals.var_pparam_b4soilratio_dn3)), ((((locals.var_wtsi_dn4 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn4)) * locals.var_pparam_b4soilratio) + (assign25210_e19208 * locals.var_pparam_b4soilratio_dn4)), ((((locals.var_wtsi_dn5 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn5)) * locals.var_pparam_b4soilratio) + (assign25210_e19208 * locals.var_pparam_b4soilratio_dn5)), ((((locals.var_wtsi_dn6 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn6)) * locals.var_pparam_b4soilratio) + (assign25210_e19208 * locals.var_pparam_b4soilratio_dn6)), ((((locals.var_wtsi_dn7 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn7)) * locals.var_pparam_b4soilratio) + (assign25210_e19208 * locals.var_pparam_b4soilratio_dn7)), ((((locals.var_wtsi_dn8 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn8)) * locals.var_pparam_b4soilratio) + (assign25210_e19208 * locals.var_pparam_b4soilratio_dn8)), ((((locals.var_wtsi_dn9 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn9)) * locals.var_pparam_b4soilratio) + (assign25210_e19208 * locals.var_pparam_b4soilratio_dn9)), ((((locals.var_wtsi_dn10 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn10)) * locals.var_pparam_b4soilratio) + (assign25210_e19208 * locals.var_pparam_b4soilratio_dn10)), ((((locals.var_wtsi_dn11 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn11)) * locals.var_pparam_b4soilratio) + (assign25210_e19208 * locals.var_pparam_b4soilratio_dn11)), ((((locals.var_wtsi_dn12 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn12)) * locals.var_pparam_b4soilratio) + (assign25210_e19208 * locals.var_pparam_b4soilratio_dn12)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11, locals.var_ien_dn12,)
    }
};
        locals.var_ien = assign25210_e19212;
        locals.var_ien_dn3 = assign25210_e19212_d_n3;
        locals.var_ien_dn4 = assign25210_e19212_d_n4;
        locals.var_ien_dn5 = assign25210_e19212_d_n5;
        locals.var_ien_dn6 = assign25210_e19212_d_n6;
        locals.var_ien_dn7 = assign25210_e19212_d_n7;
        locals.var_ien_dn8 = assign25210_e19212_d_n8;
        locals.var_ien_dn9 = assign25210_e19212_d_n9;
        locals.var_ien_dn10 = assign25210_e19212_d_n10;
        locals.var_ien_dn11 = assign25210_e19212_d_n11;
        locals.var_ien_dn12 = assign25210_e19212_d_n12;

        let (assign25220_e19221, assign25220_e19221_d_n3, assign25220_e19221_d_n4, assign25220_e19221_d_n5, assign25220_e19221_d_n6, assign25220_e19221_d_n7, assign25220_e19221_d_n8, assign25220_e19221_d_n9, assign25220_e19221_d_n10, assign25220_e19221_d_n11, assign25220_e19221_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign25220_e19219: f64 = (locals.var_t0__blk1144 * locals.var_ien);
        (assign25220_e19219, ((locals.var_t0__blk1144_dn3 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn12)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25220_e19221;
        locals.var_t1__blk1145_dn3 = assign25220_e19221_d_n3;
        locals.var_t1__blk1145_dn4 = assign25220_e19221_d_n4;
        locals.var_t1__blk1145_dn5 = assign25220_e19221_d_n5;
        locals.var_t1__blk1145_dn6 = assign25220_e19221_d_n6;
        locals.var_t1__blk1145_dn7 = assign25220_e19221_d_n7;
        locals.var_t1__blk1145_dn8 = assign25220_e19221_d_n8;
        locals.var_t1__blk1145_dn9 = assign25220_e19221_d_n9;
        locals.var_t1__blk1145_dn10 = assign25220_e19221_d_n10;
        locals.var_t1__blk1145_dn11 = assign25220_e19221_d_n11;
        locals.var_t1__blk1145_dn12 = assign25220_e19221_d_n12;

        let (assign25230_e19234, assign25230_e19234_d_n3, assign25230_e19234_d_n4, assign25230_e19234_d_n5, assign25230_e19234_d_n6, assign25230_e19234_d_n7, assign25230_e19234_d_n8, assign25230_e19234_d_n9, assign25230_e19234_d_n10, assign25230_e19234_d_n11, assign25230_e19234_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign25230_e19229: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign25230_e19230: f64 = (locals.var_t1__blk1145 * assign25230_e19229);
        let assign25230_e19232: f64 = (assign25230_e19230 * locals.var_ehlisfactor);
        (assign25230_e19232, ((((locals.var_t1__blk1145_dn3 * assign25230_e19229) + (locals.var_t1__blk1145 * locals.var_expvbsnvtm_dn3)) * locals.var_ehlisfactor) + (assign25230_e19230 * locals.var_ehlisfactor_dn3)), ((((locals.var_t1__blk1145_dn4 * assign25230_e19229) + (locals.var_t1__blk1145 * locals.var_expvbsnvtm_dn4)) * locals.var_ehlisfactor) + (assign25230_e19230 * locals.var_ehlisfactor_dn4)), ((((locals.var_t1__blk1145_dn5 * assign25230_e19229) + (locals.var_t1__blk1145 * locals.var_expvbsnvtm_dn5)) * locals.var_ehlisfactor) + (assign25230_e19230 * locals.var_ehlisfactor_dn5)), ((((locals.var_t1__blk1145_dn6 * assign25230_e19229) + (locals.var_t1__blk1145 * locals.var_expvbsnvtm_dn6)) * locals.var_ehlisfactor) + (assign25230_e19230 * locals.var_ehlisfactor_dn6)), ((((locals.var_t1__blk1145_dn7 * assign25230_e19229) + (locals.var_t1__blk1145 * locals.var_expvbsnvtm_dn7)) * locals.var_ehlisfactor) + (assign25230_e19230 * locals.var_ehlisfactor_dn7)), ((((locals.var_t1__blk1145_dn8 * assign25230_e19229) + (locals.var_t1__blk1145 * locals.var_expvbsnvtm_dn8)) * locals.var_ehlisfactor) + (assign25230_e19230 * locals.var_ehlisfactor_dn8)), ((((locals.var_t1__blk1145_dn9 * assign25230_e19229) + (locals.var_t1__blk1145 * locals.var_expvbsnvtm_dn9)) * locals.var_ehlisfactor) + (assign25230_e19230 * locals.var_ehlisfactor_dn9)), ((((locals.var_t1__blk1145_dn10 * assign25230_e19229) + (locals.var_t1__blk1145 * locals.var_expvbsnvtm_dn10)) * locals.var_ehlisfactor) + (assign25230_e19230 * locals.var_ehlisfactor_dn10)), ((((locals.var_t1__blk1145_dn11 * assign25230_e19229) + (locals.var_t1__blk1145 * locals.var_expvbsnvtm_dn11)) * locals.var_ehlisfactor) + (assign25230_e19230 * locals.var_ehlisfactor_dn11)), ((((locals.var_t1__blk1145_dn12 * assign25230_e19229) + (locals.var_t1__blk1145 * locals.var_expvbsnvtm_dn12)) * locals.var_ehlisfactor) + (assign25230_e19230 * locals.var_ehlisfactor_dn12)),)
    } else {
        (locals.var_ibs3, locals.var_ibs3_dn3, locals.var_ibs3_dn4, locals.var_ibs3_dn5, locals.var_ibs3_dn6, locals.var_ibs3_dn7, locals.var_ibs3_dn8, locals.var_ibs3_dn9, locals.var_ibs3_dn10, locals.var_ibs3_dn11, locals.var_ibs3_dn12,)
    }
};
        locals.var_ibs3 = assign25230_e19234;
        locals.var_ibs3_dn3 = assign25230_e19234_d_n3;
        locals.var_ibs3_dn4 = assign25230_e19234_d_n4;
        locals.var_ibs3_dn5 = assign25230_e19234_d_n5;
        locals.var_ibs3_dn6 = assign25230_e19234_d_n6;
        locals.var_ibs3_dn7 = assign25230_e19234_d_n7;
        locals.var_ibs3_dn8 = assign25230_e19234_d_n8;
        locals.var_ibs3_dn9 = assign25230_e19234_d_n9;
        locals.var_ibs3_dn10 = assign25230_e19234_d_n10;
        locals.var_ibs3_dn11 = assign25230_e19234_d_n11;
        locals.var_ibs3_dn12 = assign25230_e19234_d_n12;

        let (assign25240_e19245, assign25240_e19245_d_n3, assign25240_e19245_d_n4, assign25240_e19245_d_n5, assign25240_e19245_d_n6, assign25240_e19245_d_n7, assign25240_e19245_d_n8, assign25240_e19245_d_n9, assign25240_e19245_d_n10, assign25240_e19245_d_n11, assign25240_e19245_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign25240_e19241: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign25240_e19243: f64 = (assign25240_e19241 * locals.var_pparam_b4soilratio);
        (assign25240_e19243, ((((locals.var_wtsi_dn3 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn3)) * locals.var_pparam_b4soilratio) + (assign25240_e19241 * locals.var_pparam_b4soilratio_dn3)), ((((locals.var_wtsi_dn4 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn4)) * locals.var_pparam_b4soilratio) + (assign25240_e19241 * locals.var_pparam_b4soilratio_dn4)), ((((locals.var_wtsi_dn5 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn5)) * locals.var_pparam_b4soilratio) + (assign25240_e19241 * locals.var_pparam_b4soilratio_dn5)), ((((locals.var_wtsi_dn6 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn6)) * locals.var_pparam_b4soilratio) + (assign25240_e19241 * locals.var_pparam_b4soilratio_dn6)), ((((locals.var_wtsi_dn7 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn7)) * locals.var_pparam_b4soilratio) + (assign25240_e19241 * locals.var_pparam_b4soilratio_dn7)), ((((locals.var_wtsi_dn8 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn8)) * locals.var_pparam_b4soilratio) + (assign25240_e19241 * locals.var_pparam_b4soilratio_dn8)), ((((locals.var_wtsi_dn9 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn9)) * locals.var_pparam_b4soilratio) + (assign25240_e19241 * locals.var_pparam_b4soilratio_dn9)), ((((locals.var_wtsi_dn10 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn10)) * locals.var_pparam_b4soilratio) + (assign25240_e19241 * locals.var_pparam_b4soilratio_dn10)), ((((locals.var_wtsi_dn11 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn11)) * locals.var_pparam_b4soilratio) + (assign25240_e19241 * locals.var_pparam_b4soilratio_dn11)), ((((locals.var_wtsi_dn12 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn12)) * locals.var_pparam_b4soilratio) + (assign25240_e19241 * locals.var_pparam_b4soilratio_dn12)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11, locals.var_ien_dn12,)
    }
};
        locals.var_ien = assign25240_e19245;
        locals.var_ien_dn3 = assign25240_e19245_d_n3;
        locals.var_ien_dn4 = assign25240_e19245_d_n4;
        locals.var_ien_dn5 = assign25240_e19245_d_n5;
        locals.var_ien_dn6 = assign25240_e19245_d_n6;
        locals.var_ien_dn7 = assign25240_e19245_d_n7;
        locals.var_ien_dn8 = assign25240_e19245_d_n8;
        locals.var_ien_dn9 = assign25240_e19245_d_n9;
        locals.var_ien_dn10 = assign25240_e19245_d_n10;
        locals.var_ien_dn11 = assign25240_e19245_d_n11;
        locals.var_ien_dn12 = assign25240_e19245_d_n12;

        let (assign25250_e19254, assign25250_e19254_d_n3, assign25250_e19254_d_n4, assign25250_e19254_d_n5, assign25250_e19254_d_n6, assign25250_e19254_d_n7, assign25250_e19254_d_n8, assign25250_e19254_d_n9, assign25250_e19254_d_n10, assign25250_e19254_d_n11, assign25250_e19254_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign25250_e19252: f64 = (locals.var_t0__blk1144 * locals.var_ien);
        (assign25250_e19252, ((locals.var_t0__blk1144_dn3 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_ien) + (locals.var_t0__blk1144 * locals.var_ien_dn12)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25250_e19254;
        locals.var_t1__blk1145_dn3 = assign25250_e19254_d_n3;
        locals.var_t1__blk1145_dn4 = assign25250_e19254_d_n4;
        locals.var_t1__blk1145_dn5 = assign25250_e19254_d_n5;
        locals.var_t1__blk1145_dn6 = assign25250_e19254_d_n6;
        locals.var_t1__blk1145_dn7 = assign25250_e19254_d_n7;
        locals.var_t1__blk1145_dn8 = assign25250_e19254_d_n8;
        locals.var_t1__blk1145_dn9 = assign25250_e19254_d_n9;
        locals.var_t1__blk1145_dn10 = assign25250_e19254_d_n10;
        locals.var_t1__blk1145_dn11 = assign25250_e19254_d_n11;
        locals.var_t1__blk1145_dn12 = assign25250_e19254_d_n12;

        let (assign25260_e19267, assign25260_e19267_d_n3, assign25260_e19267_d_n4, assign25260_e19267_d_n5, assign25260_e19267_d_n6, assign25260_e19267_d_n7, assign25260_e19267_d_n8, assign25260_e19267_d_n9, assign25260_e19267_d_n10, assign25260_e19267_d_n11, assign25260_e19267_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign25260_e19262: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign25260_e19263: f64 = (locals.var_t1__blk1145 * assign25260_e19262);
        let assign25260_e19265: f64 = (assign25260_e19263 * locals.var_ehlidfactor);
        (assign25260_e19265, ((((locals.var_t1__blk1145_dn3 * assign25260_e19262) + (locals.var_t1__blk1145 * locals.var_expvbdnvtm_dn3)) * locals.var_ehlidfactor) + (assign25260_e19263 * locals.var_ehlidfactor_dn3)), ((((locals.var_t1__blk1145_dn4 * assign25260_e19262) + (locals.var_t1__blk1145 * locals.var_expvbdnvtm_dn4)) * locals.var_ehlidfactor) + (assign25260_e19263 * locals.var_ehlidfactor_dn4)), ((((locals.var_t1__blk1145_dn5 * assign25260_e19262) + (locals.var_t1__blk1145 * locals.var_expvbdnvtm_dn5)) * locals.var_ehlidfactor) + (assign25260_e19263 * locals.var_ehlidfactor_dn5)), ((((locals.var_t1__blk1145_dn6 * assign25260_e19262) + (locals.var_t1__blk1145 * locals.var_expvbdnvtm_dn6)) * locals.var_ehlidfactor) + (assign25260_e19263 * locals.var_ehlidfactor_dn6)), ((((locals.var_t1__blk1145_dn7 * assign25260_e19262) + (locals.var_t1__blk1145 * locals.var_expvbdnvtm_dn7)) * locals.var_ehlidfactor) + (assign25260_e19263 * locals.var_ehlidfactor_dn7)), ((((locals.var_t1__blk1145_dn8 * assign25260_e19262) + (locals.var_t1__blk1145 * locals.var_expvbdnvtm_dn8)) * locals.var_ehlidfactor) + (assign25260_e19263 * locals.var_ehlidfactor_dn8)), ((((locals.var_t1__blk1145_dn9 * assign25260_e19262) + (locals.var_t1__blk1145 * locals.var_expvbdnvtm_dn9)) * locals.var_ehlidfactor) + (assign25260_e19263 * locals.var_ehlidfactor_dn9)), ((((locals.var_t1__blk1145_dn10 * assign25260_e19262) + (locals.var_t1__blk1145 * locals.var_expvbdnvtm_dn10)) * locals.var_ehlidfactor) + (assign25260_e19263 * locals.var_ehlidfactor_dn10)), ((((locals.var_t1__blk1145_dn11 * assign25260_e19262) + (locals.var_t1__blk1145 * locals.var_expvbdnvtm_dn11)) * locals.var_ehlidfactor) + (assign25260_e19263 * locals.var_ehlidfactor_dn11)), ((((locals.var_t1__blk1145_dn12 * assign25260_e19262) + (locals.var_t1__blk1145 * locals.var_expvbdnvtm_dn12)) * locals.var_ehlidfactor) + (assign25260_e19263 * locals.var_ehlidfactor_dn12)),)
    } else {
        (locals.var_ibd3, locals.var_ibd3_dn3, locals.var_ibd3_dn4, locals.var_ibd3_dn5, locals.var_ibd3_dn6, locals.var_ibd3_dn7, locals.var_ibd3_dn8, locals.var_ibd3_dn9, locals.var_ibd3_dn10, locals.var_ibd3_dn11, locals.var_ibd3_dn12,)
    }
};
        locals.var_ibd3 = assign25260_e19267;
        locals.var_ibd3_dn3 = assign25260_e19267_d_n3;
        locals.var_ibd3_dn4 = assign25260_e19267_d_n4;
        locals.var_ibd3_dn5 = assign25260_e19267_d_n5;
        locals.var_ibd3_dn6 = assign25260_e19267_d_n6;
        locals.var_ibd3_dn7 = assign25260_e19267_d_n7;
        locals.var_ibd3_dn8 = assign25260_e19267_d_n8;
        locals.var_ibd3_dn9 = assign25260_e19267_d_n9;
        locals.var_ibd3_dn10 = assign25260_e19267_d_n10;
        locals.var_ibd3_dn11 = assign25260_e19267_d_n11;
        locals.var_ibd3_dn12 = assign25260_e19267_d_n12;

        let (assign25270_e19278, assign25270_e19278_d_n3, assign25270_e19278_d_n4, assign25270_e19278_d_n5, assign25270_e19278_d_n6, assign25270_e19278_d_n7, assign25270_e19278_d_n8, assign25270_e19278_d_n9, assign25270_e19278_d_n10, assign25270_e19278_d_n11, assign25270_e19278_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign25270_e19274: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign25270_e19276: f64 = (assign25270_e19274 * locals.var_pparam_b4soilratiodif);
        (assign25270_e19276, ((((locals.var_wtsi_dn3 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn3)) * locals.var_pparam_b4soilratiodif) + (assign25270_e19274 * locals.var_pparam_b4soilratiodif_dn3)), ((((locals.var_wtsi_dn4 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn4)) * locals.var_pparam_b4soilratiodif) + (assign25270_e19274 * locals.var_pparam_b4soilratiodif_dn4)), ((((locals.var_wtsi_dn5 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn5)) * locals.var_pparam_b4soilratiodif) + (assign25270_e19274 * locals.var_pparam_b4soilratiodif_dn5)), ((((locals.var_wtsi_dn6 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn6)) * locals.var_pparam_b4soilratiodif) + (assign25270_e19274 * locals.var_pparam_b4soilratiodif_dn6)), ((((locals.var_wtsi_dn7 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn7)) * locals.var_pparam_b4soilratiodif) + (assign25270_e19274 * locals.var_pparam_b4soilratiodif_dn7)), ((((locals.var_wtsi_dn8 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn8)) * locals.var_pparam_b4soilratiodif) + (assign25270_e19274 * locals.var_pparam_b4soilratiodif_dn8)), ((((locals.var_wtsi_dn9 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn9)) * locals.var_pparam_b4soilratiodif) + (assign25270_e19274 * locals.var_pparam_b4soilratiodif_dn9)), ((((locals.var_wtsi_dn10 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn10)) * locals.var_pparam_b4soilratiodif) + (assign25270_e19274 * locals.var_pparam_b4soilratiodif_dn10)), ((((locals.var_wtsi_dn11 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn11)) * locals.var_pparam_b4soilratiodif) + (assign25270_e19274 * locals.var_pparam_b4soilratiodif_dn11)), ((((locals.var_wtsi_dn12 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn12)) * locals.var_pparam_b4soilratiodif) + (assign25270_e19274 * locals.var_pparam_b4soilratiodif_dn12)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11, locals.var_iendif_dn12,)
    }
};
        locals.var_iendif = assign25270_e19278;
        locals.var_iendif_dn3 = assign25270_e19278_d_n3;
        locals.var_iendif_dn4 = assign25270_e19278_d_n4;
        locals.var_iendif_dn5 = assign25270_e19278_d_n5;
        locals.var_iendif_dn6 = assign25270_e19278_d_n6;
        locals.var_iendif_dn7 = assign25270_e19278_d_n7;
        locals.var_iendif_dn8 = assign25270_e19278_d_n8;
        locals.var_iendif_dn9 = assign25270_e19278_d_n9;
        locals.var_iendif_dn10 = assign25270_e19278_d_n10;
        locals.var_iendif_dn11 = assign25270_e19278_d_n11;
        locals.var_iendif_dn12 = assign25270_e19278_d_n12;

        let (assign25280_e19291, assign25280_e19291_d_n3, assign25280_e19291_d_n4, assign25280_e19291_d_n5, assign25280_e19291_d_n6, assign25280_e19291_d_n7, assign25280_e19291_d_n8, assign25280_e19291_d_n9, assign25280_e19291_d_n10, assign25280_e19291_d_n11, assign25280_e19291_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign25280_e19286: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign25280_e19287: f64 = (locals.var_iendif * assign25280_e19286);
        let assign25280_e19289: f64 = (assign25280_e19287 * locals.var_ehlisfactor);
        (assign25280_e19289, ((((locals.var_iendif_dn3 * assign25280_e19286) + (locals.var_iendif * locals.var_expvbsnvtm_dn3)) * locals.var_ehlisfactor) + (assign25280_e19287 * locals.var_ehlisfactor_dn3)), ((((locals.var_iendif_dn4 * assign25280_e19286) + (locals.var_iendif * locals.var_expvbsnvtm_dn4)) * locals.var_ehlisfactor) + (assign25280_e19287 * locals.var_ehlisfactor_dn4)), ((((locals.var_iendif_dn5 * assign25280_e19286) + (locals.var_iendif * locals.var_expvbsnvtm_dn5)) * locals.var_ehlisfactor) + (assign25280_e19287 * locals.var_ehlisfactor_dn5)), ((((locals.var_iendif_dn6 * assign25280_e19286) + (locals.var_iendif * locals.var_expvbsnvtm_dn6)) * locals.var_ehlisfactor) + (assign25280_e19287 * locals.var_ehlisfactor_dn6)), ((((locals.var_iendif_dn7 * assign25280_e19286) + (locals.var_iendif * locals.var_expvbsnvtm_dn7)) * locals.var_ehlisfactor) + (assign25280_e19287 * locals.var_ehlisfactor_dn7)), ((((locals.var_iendif_dn8 * assign25280_e19286) + (locals.var_iendif * locals.var_expvbsnvtm_dn8)) * locals.var_ehlisfactor) + (assign25280_e19287 * locals.var_ehlisfactor_dn8)), ((((locals.var_iendif_dn9 * assign25280_e19286) + (locals.var_iendif * locals.var_expvbsnvtm_dn9)) * locals.var_ehlisfactor) + (assign25280_e19287 * locals.var_ehlisfactor_dn9)), ((((locals.var_iendif_dn10 * assign25280_e19286) + (locals.var_iendif * locals.var_expvbsnvtm_dn10)) * locals.var_ehlisfactor) + (assign25280_e19287 * locals.var_ehlisfactor_dn10)), ((((locals.var_iendif_dn11 * assign25280_e19286) + (locals.var_iendif * locals.var_expvbsnvtm_dn11)) * locals.var_ehlisfactor) + (assign25280_e19287 * locals.var_ehlisfactor_dn11)), ((((locals.var_iendif_dn12 * assign25280_e19286) + (locals.var_iendif * locals.var_expvbsnvtm_dn12)) * locals.var_ehlisfactor) + (assign25280_e19287 * locals.var_ehlisfactor_dn12)),)
    } else {
        (locals.var_ibsdif, locals.var_ibsdif_dn3, locals.var_ibsdif_dn4, locals.var_ibsdif_dn5, locals.var_ibsdif_dn6, locals.var_ibsdif_dn7, locals.var_ibsdif_dn8, locals.var_ibsdif_dn9, locals.var_ibsdif_dn10, locals.var_ibsdif_dn11, locals.var_ibsdif_dn12,)
    }
};
        locals.var_ibsdif = assign25280_e19291;
        locals.var_ibsdif_dn3 = assign25280_e19291_d_n3;
        locals.var_ibsdif_dn4 = assign25280_e19291_d_n4;
        locals.var_ibsdif_dn5 = assign25280_e19291_d_n5;
        locals.var_ibsdif_dn6 = assign25280_e19291_d_n6;
        locals.var_ibsdif_dn7 = assign25280_e19291_d_n7;
        locals.var_ibsdif_dn8 = assign25280_e19291_d_n8;
        locals.var_ibsdif_dn9 = assign25280_e19291_d_n9;
        locals.var_ibsdif_dn10 = assign25280_e19291_d_n10;
        locals.var_ibsdif_dn11 = assign25280_e19291_d_n11;
        locals.var_ibsdif_dn12 = assign25280_e19291_d_n12;

        let (assign25290_e19302, assign25290_e19302_d_n3, assign25290_e19302_d_n4, assign25290_e19302_d_n5, assign25290_e19302_d_n6, assign25290_e19302_d_n7, assign25290_e19302_d_n8, assign25290_e19302_d_n9, assign25290_e19302_d_n10, assign25290_e19302_d_n11, assign25290_e19302_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign25290_e19298: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign25290_e19300: f64 = (assign25290_e19298 * locals.var_pparam_b4soilratiodif);
        (assign25290_e19300, ((((locals.var_wtsi_dn3 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn3)) * locals.var_pparam_b4soilratiodif) + (assign25290_e19298 * locals.var_pparam_b4soilratiodif_dn3)), ((((locals.var_wtsi_dn4 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn4)) * locals.var_pparam_b4soilratiodif) + (assign25290_e19298 * locals.var_pparam_b4soilratiodif_dn4)), ((((locals.var_wtsi_dn5 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn5)) * locals.var_pparam_b4soilratiodif) + (assign25290_e19298 * locals.var_pparam_b4soilratiodif_dn5)), ((((locals.var_wtsi_dn6 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn6)) * locals.var_pparam_b4soilratiodif) + (assign25290_e19298 * locals.var_pparam_b4soilratiodif_dn6)), ((((locals.var_wtsi_dn7 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn7)) * locals.var_pparam_b4soilratiodif) + (assign25290_e19298 * locals.var_pparam_b4soilratiodif_dn7)), ((((locals.var_wtsi_dn8 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn8)) * locals.var_pparam_b4soilratiodif) + (assign25290_e19298 * locals.var_pparam_b4soilratiodif_dn8)), ((((locals.var_wtsi_dn9 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn9)) * locals.var_pparam_b4soilratiodif) + (assign25290_e19298 * locals.var_pparam_b4soilratiodif_dn9)), ((((locals.var_wtsi_dn10 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn10)) * locals.var_pparam_b4soilratiodif) + (assign25290_e19298 * locals.var_pparam_b4soilratiodif_dn10)), ((((locals.var_wtsi_dn11 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn11)) * locals.var_pparam_b4soilratiodif) + (assign25290_e19298 * locals.var_pparam_b4soilratiodif_dn11)), ((((locals.var_wtsi_dn12 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn12)) * locals.var_pparam_b4soilratiodif) + (assign25290_e19298 * locals.var_pparam_b4soilratiodif_dn12)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11, locals.var_iendif_dn12,)
    }
};
        locals.var_iendif = assign25290_e19302;
        locals.var_iendif_dn3 = assign25290_e19302_d_n3;
        locals.var_iendif_dn4 = assign25290_e19302_d_n4;
        locals.var_iendif_dn5 = assign25290_e19302_d_n5;
        locals.var_iendif_dn6 = assign25290_e19302_d_n6;
        locals.var_iendif_dn7 = assign25290_e19302_d_n7;
        locals.var_iendif_dn8 = assign25290_e19302_d_n8;
        locals.var_iendif_dn9 = assign25290_e19302_d_n9;
        locals.var_iendif_dn10 = assign25290_e19302_d_n10;
        locals.var_iendif_dn11 = assign25290_e19302_d_n11;
        locals.var_iendif_dn12 = assign25290_e19302_d_n12;

        let (assign25300_e19315, assign25300_e19315_d_n3, assign25300_e19315_d_n4, assign25300_e19315_d_n5, assign25300_e19315_d_n6, assign25300_e19315_d_n7, assign25300_e19315_d_n8, assign25300_e19315_d_n9, assign25300_e19315_d_n10, assign25300_e19315_d_n11, assign25300_e19315_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign25300_e19310: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign25300_e19311: f64 = (locals.var_iendif * assign25300_e19310);
        let assign25300_e19313: f64 = (assign25300_e19311 * locals.var_ehlidfactor);
        (assign25300_e19313, ((((locals.var_iendif_dn3 * assign25300_e19310) + (locals.var_iendif * locals.var_expvbdnvtm_dn3)) * locals.var_ehlidfactor) + (assign25300_e19311 * locals.var_ehlidfactor_dn3)), ((((locals.var_iendif_dn4 * assign25300_e19310) + (locals.var_iendif * locals.var_expvbdnvtm_dn4)) * locals.var_ehlidfactor) + (assign25300_e19311 * locals.var_ehlidfactor_dn4)), ((((locals.var_iendif_dn5 * assign25300_e19310) + (locals.var_iendif * locals.var_expvbdnvtm_dn5)) * locals.var_ehlidfactor) + (assign25300_e19311 * locals.var_ehlidfactor_dn5)), ((((locals.var_iendif_dn6 * assign25300_e19310) + (locals.var_iendif * locals.var_expvbdnvtm_dn6)) * locals.var_ehlidfactor) + (assign25300_e19311 * locals.var_ehlidfactor_dn6)), ((((locals.var_iendif_dn7 * assign25300_e19310) + (locals.var_iendif * locals.var_expvbdnvtm_dn7)) * locals.var_ehlidfactor) + (assign25300_e19311 * locals.var_ehlidfactor_dn7)), ((((locals.var_iendif_dn8 * assign25300_e19310) + (locals.var_iendif * locals.var_expvbdnvtm_dn8)) * locals.var_ehlidfactor) + (assign25300_e19311 * locals.var_ehlidfactor_dn8)), ((((locals.var_iendif_dn9 * assign25300_e19310) + (locals.var_iendif * locals.var_expvbdnvtm_dn9)) * locals.var_ehlidfactor) + (assign25300_e19311 * locals.var_ehlidfactor_dn9)), ((((locals.var_iendif_dn10 * assign25300_e19310) + (locals.var_iendif * locals.var_expvbdnvtm_dn10)) * locals.var_ehlidfactor) + (assign25300_e19311 * locals.var_ehlidfactor_dn10)), ((((locals.var_iendif_dn11 * assign25300_e19310) + (locals.var_iendif * locals.var_expvbdnvtm_dn11)) * locals.var_ehlidfactor) + (assign25300_e19311 * locals.var_ehlidfactor_dn11)), ((((locals.var_iendif_dn12 * assign25300_e19310) + (locals.var_iendif * locals.var_expvbdnvtm_dn12)) * locals.var_ehlidfactor) + (assign25300_e19311 * locals.var_ehlidfactor_dn12)),)
    } else {
        (locals.var_ibddif, locals.var_ibddif_dn3, locals.var_ibddif_dn4, locals.var_ibddif_dn5, locals.var_ibddif_dn6, locals.var_ibddif_dn7, locals.var_ibddif_dn8, locals.var_ibddif_dn9, locals.var_ibddif_dn10, locals.var_ibddif_dn11, locals.var_ibddif_dn12,)
    }
};
        locals.var_ibddif = assign25300_e19315;
        locals.var_ibddif_dn3 = assign25300_e19315_d_n3;
        locals.var_ibddif_dn4 = assign25300_e19315_d_n4;
        locals.var_ibddif_dn5 = assign25300_e19315_d_n5;
        locals.var_ibddif_dn6 = assign25300_e19315_d_n6;
        locals.var_ibddif_dn7 = assign25300_e19315_d_n7;
        locals.var_ibddif_dn8 = assign25300_e19315_d_n8;
        locals.var_ibddif_dn9 = assign25300_e19315_d_n9;
        locals.var_ibddif_dn10 = assign25300_e19315_d_n10;
        locals.var_ibddif_dn11 = assign25300_e19315_d_n11;
        locals.var_ibddif_dn12 = assign25300_e19315_d_n12;

        let assign25310_e19318: f64 = if locals.var_b4soibjtoff == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1616 = assign25310_e19318;

        let (assign25320_e19327, assign25320_e19327_d_n3, assign25320_e19327_d_n4, assign25320_e19327_d_n5, assign25320_e19327_d_n6, assign25320_e19327_d_n7, assign25320_e19327_d_n8, assign25320_e19327_d_n9, assign25320_e19327_d_n10, assign25320_e19327_d_n11, assign25320_e19327_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1616 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ic_1, locals.var_ic_1_dn3, locals.var_ic_1_dn4, locals.var_ic_1_dn5, locals.var_ic_1_dn6, locals.var_ic_1_dn7, locals.var_ic_1_dn8, locals.var_ic_1_dn9, locals.var_ic_1_dn10, locals.var_ic_1_dn11, locals.var_ic_1_dn12,)
    }
};
        locals.var_ic_1 = assign25320_e19327;
        locals.var_ic_1_dn3 = assign25320_e19327_d_n3;
        locals.var_ic_1_dn4 = assign25320_e19327_d_n4;
        locals.var_ic_1_dn5 = assign25320_e19327_d_n5;
        locals.var_ic_1_dn6 = assign25320_e19327_d_n6;
        locals.var_ic_1_dn7 = assign25320_e19327_d_n7;
        locals.var_ic_1_dn8 = assign25320_e19327_d_n8;
        locals.var_ic_1_dn9 = assign25320_e19327_d_n9;
        locals.var_ic_1_dn10 = assign25320_e19327_d_n10;
        locals.var_ic_1_dn11 = assign25320_e19327_d_n11;
        locals.var_ic_1_dn12 = assign25320_e19327_d_n12;

        let (assign25330_e19343, assign25330_e19343_d_n3, assign25330_e19343_d_n4, assign25330_e19343_d_n5, assign25330_e19343_d_n6, assign25330_e19343_d_n7, assign25330_e19343_d_n8, assign25330_e19343_d_n9, assign25330_e19343_d_n10, assign25330_e19343_d_n11, assign25330_e19343_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1616 == 0.0)) {
        let assign25330_e19338: f64 = (locals.var_vsbs + locals.var_vdbd);
        let assign25330_e19340: f64 = (assign25330_e19338 / locals.var_pparam_b4soivearly);
        let assign25330_e19341: f64 = (1.0 + assign25330_e19340);
        (assign25330_e19341, (-((assign25330_e19338 * locals.var_pparam_b4soivearly_dn3) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly))), (-((assign25330_e19338 * locals.var_pparam_b4soivearly_dn4) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly))), (-((assign25330_e19338 * locals.var_pparam_b4soivearly_dn5) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly))), (-((assign25330_e19338 * locals.var_pparam_b4soivearly_dn6) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly))), (((locals.var_vdbd_dn7 * locals.var_pparam_b4soivearly) - (assign25330_e19338 * locals.var_pparam_b4soivearly_dn7)) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly)), (((locals.var_vsbs_dn8 * locals.var_pparam_b4soivearly) - (assign25330_e19338 * locals.var_pparam_b4soivearly_dn8)) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly)), (-((assign25330_e19338 * locals.var_pparam_b4soivearly_dn9) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly))), (-((assign25330_e19338 * locals.var_pparam_b4soivearly_dn10) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly))), (((locals.var_vsbs_dn11 * locals.var_pparam_b4soivearly) - (assign25330_e19338 * locals.var_pparam_b4soivearly_dn11)) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly)), (((locals.var_vdbd_dn12 * locals.var_pparam_b4soivearly) - (assign25330_e19338 * locals.var_pparam_b4soivearly_dn12)) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign25330_e19343;
        locals.var_t0__blk1144_dn3 = assign25330_e19343_d_n3;
        locals.var_t0__blk1144_dn4 = assign25330_e19343_d_n4;
        locals.var_t0__blk1144_dn5 = assign25330_e19343_d_n5;
        locals.var_t0__blk1144_dn6 = assign25330_e19343_d_n6;
        locals.var_t0__blk1144_dn7 = assign25330_e19343_d_n7;
        locals.var_t0__blk1144_dn8 = assign25330_e19343_d_n8;
        locals.var_t0__blk1144_dn9 = assign25330_e19343_d_n9;
        locals.var_t0__blk1144_dn10 = assign25330_e19343_d_n10;
        locals.var_t0__blk1144_dn11 = assign25330_e19343_d_n11;
        locals.var_t0__blk1144_dn12 = assign25330_e19343_d_n12;

        let (assign25340_e19355, assign25340_e19355_d_n3, assign25340_e19355_d_n4, assign25340_e19355_d_n5, assign25340_e19355_d_n6, assign25340_e19355_d_n7, assign25340_e19355_d_n8, assign25340_e19355_d_n9, assign25340_e19355_d_n10, assign25340_e19355_d_n11, assign25340_e19355_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1616 == 0.0)) {
        let assign25340_e19353: f64 = (locals.var_ehlis + locals.var_ehlid);
        (assign25340_e19353, (locals.var_ehlis_dn3 + locals.var_ehlid_dn3), (locals.var_ehlis_dn4 + locals.var_ehlid_dn4), (locals.var_ehlis_dn5 + locals.var_ehlid_dn5), (locals.var_ehlis_dn6 + locals.var_ehlid_dn6), (locals.var_ehlis_dn7 + locals.var_ehlid_dn7), (locals.var_ehlis_dn8 + locals.var_ehlid_dn8), (locals.var_ehlis_dn9 + locals.var_ehlid_dn9), (locals.var_ehlis_dn10 + locals.var_ehlid_dn10), (locals.var_ehlis_dn11 + locals.var_ehlid_dn11), (locals.var_ehlis_dn12 + locals.var_ehlid_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25340_e19355;
        locals.var_t1__blk1145_dn3 = assign25340_e19355_d_n3;
        locals.var_t1__blk1145_dn4 = assign25340_e19355_d_n4;
        locals.var_t1__blk1145_dn5 = assign25340_e19355_d_n5;
        locals.var_t1__blk1145_dn6 = assign25340_e19355_d_n6;
        locals.var_t1__blk1145_dn7 = assign25340_e19355_d_n7;
        locals.var_t1__blk1145_dn8 = assign25340_e19355_d_n8;
        locals.var_t1__blk1145_dn9 = assign25340_e19355_d_n9;
        locals.var_t1__blk1145_dn10 = assign25340_e19355_d_n10;
        locals.var_t1__blk1145_dn11 = assign25340_e19355_d_n11;
        locals.var_t1__blk1145_dn12 = assign25340_e19355_d_n12;

        let (assign25350_e19372, assign25350_e19372_d_n3, assign25350_e19372_d_n4, assign25350_e19372_d_n5, assign25350_e19372_d_n6, assign25350_e19372_d_n7, assign25350_e19372_d_n8, assign25350_e19372_d_n9, assign25350_e19372_d_n10, assign25350_e19372_d_n11, assign25350_e19372_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1616 == 0.0)) {
        let assign25350_e19365: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign25350_e19368: f64 = (4.0 * locals.var_t1__blk1145);
        let assign25350_e19369: f64 = (assign25350_e19365 + assign25350_e19368);
        let assign25350_e19370: f64 = (assign25350_e19369).sqrt();
        (assign25350_e19370, ((((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) + (4.0 * locals.var_t1__blk1145_dn3)) / (2.0 * assign25350_e19370)), ((((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) + (4.0 * locals.var_t1__blk1145_dn4)) / (2.0 * assign25350_e19370)), ((((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) + (4.0 * locals.var_t1__blk1145_dn5)) / (2.0 * assign25350_e19370)), ((((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) + (4.0 * locals.var_t1__blk1145_dn6)) / (2.0 * assign25350_e19370)), ((((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) + (4.0 * locals.var_t1__blk1145_dn7)) / (2.0 * assign25350_e19370)), ((((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) + (4.0 * locals.var_t1__blk1145_dn8)) / (2.0 * assign25350_e19370)), ((((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) + (4.0 * locals.var_t1__blk1145_dn9)) / (2.0 * assign25350_e19370)), ((((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) + (4.0 * locals.var_t1__blk1145_dn10)) / (2.0 * assign25350_e19370)), ((((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) + (4.0 * locals.var_t1__blk1145_dn11)) / (2.0 * assign25350_e19370)), ((((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) + (4.0 * locals.var_t1__blk1145_dn12)) / (2.0 * assign25350_e19370)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign25350_e19372;
        locals.var_t3__blk1147_dn3 = assign25350_e19372_d_n3;
        locals.var_t3__blk1147_dn4 = assign25350_e19372_d_n4;
        locals.var_t3__blk1147_dn5 = assign25350_e19372_d_n5;
        locals.var_t3__blk1147_dn6 = assign25350_e19372_d_n6;
        locals.var_t3__blk1147_dn7 = assign25350_e19372_d_n7;
        locals.var_t3__blk1147_dn8 = assign25350_e19372_d_n8;
        locals.var_t3__blk1147_dn9 = assign25350_e19372_d_n9;
        locals.var_t3__blk1147_dn10 = assign25350_e19372_d_n10;
        locals.var_t3__blk1147_dn11 = assign25350_e19372_d_n11;
        locals.var_t3__blk1147_dn12 = assign25350_e19372_d_n12;

        let (assign25360_e19386, assign25360_e19386_d_n3, assign25360_e19386_d_n4, assign25360_e19386_d_n5, assign25360_e19386_d_n6, assign25360_e19386_d_n7, assign25360_e19386_d_n8, assign25360_e19386_d_n9, assign25360_e19386_d_n10, assign25360_e19386_d_n11, assign25360_e19386_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1616 == 0.0)) {
        let assign25360_e19382: f64 = (locals.var_t0__blk1144 + locals.var_t3__blk1147);
        let assign25360_e19384: f64 = (assign25360_e19382 / 2.0);
        (assign25360_e19384, ((locals.var_t0__blk1144_dn3 + locals.var_t3__blk1147_dn3) / 2.0), ((locals.var_t0__blk1144_dn4 + locals.var_t3__blk1147_dn4) / 2.0), ((locals.var_t0__blk1144_dn5 + locals.var_t3__blk1147_dn5) / 2.0), ((locals.var_t0__blk1144_dn6 + locals.var_t3__blk1147_dn6) / 2.0), ((locals.var_t0__blk1144_dn7 + locals.var_t3__blk1147_dn7) / 2.0), ((locals.var_t0__blk1144_dn8 + locals.var_t3__blk1147_dn8) / 2.0), ((locals.var_t0__blk1144_dn9 + locals.var_t3__blk1147_dn9) / 2.0), ((locals.var_t0__blk1144_dn10 + locals.var_t3__blk1147_dn10) / 2.0), ((locals.var_t0__blk1144_dn11 + locals.var_t3__blk1147_dn11) / 2.0), ((locals.var_t0__blk1144_dn12 + locals.var_t3__blk1147_dn12) / 2.0),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign25360_e19386;
        locals.var_t2__blk1146_dn3 = assign25360_e19386_d_n3;
        locals.var_t2__blk1146_dn4 = assign25360_e19386_d_n4;
        locals.var_t2__blk1146_dn5 = assign25360_e19386_d_n5;
        locals.var_t2__blk1146_dn6 = assign25360_e19386_d_n6;
        locals.var_t2__blk1146_dn7 = assign25360_e19386_d_n7;
        locals.var_t2__blk1146_dn8 = assign25360_e19386_d_n8;
        locals.var_t2__blk1146_dn9 = assign25360_e19386_d_n9;
        locals.var_t2__blk1146_dn10 = assign25360_e19386_d_n10;
        locals.var_t2__blk1146_dn11 = assign25360_e19386_d_n11;
        locals.var_t2__blk1146_dn12 = assign25360_e19386_d_n12;

        let assign25370_e19389: f64 = if locals.var_t2__blk1146 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard1617 = assign25370_e19389;

        let (assign25380_e19401, assign25380_e19401_d_n3, assign25380_e19401_d_n4, assign25380_e19401_d_n5, assign25380_e19401_d_n6, assign25380_e19401_d_n7, assign25380_e19401_d_n8, assign25380_e19401_d_n9, assign25380_e19401_d_n10, assign25380_e19401_d_n11, assign25380_e19401_d_n12,) = {
    if ((((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1616 == 0.0)) && (locals.var_guard1617 != 0.0)) {
        (10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_e2ndfactor, locals.var_e2ndfactor_dn3, locals.var_e2ndfactor_dn4, locals.var_e2ndfactor_dn5, locals.var_e2ndfactor_dn6, locals.var_e2ndfactor_dn7, locals.var_e2ndfactor_dn8, locals.var_e2ndfactor_dn9, locals.var_e2ndfactor_dn10, locals.var_e2ndfactor_dn11, locals.var_e2ndfactor_dn12,)
    }
};
        locals.var_e2ndfactor = assign25380_e19401;
        locals.var_e2ndfactor_dn3 = assign25380_e19401_d_n3;
        locals.var_e2ndfactor_dn4 = assign25380_e19401_d_n4;
        locals.var_e2ndfactor_dn5 = assign25380_e19401_d_n5;
        locals.var_e2ndfactor_dn6 = assign25380_e19401_d_n6;
        locals.var_e2ndfactor_dn7 = assign25380_e19401_d_n7;
        locals.var_e2ndfactor_dn8 = assign25380_e19401_d_n8;
        locals.var_e2ndfactor_dn9 = assign25380_e19401_d_n9;
        locals.var_e2ndfactor_dn10 = assign25380_e19401_d_n10;
        locals.var_e2ndfactor_dn11 = assign25380_e19401_d_n11;
        locals.var_e2ndfactor_dn12 = assign25380_e19401_d_n12;

    }

    pub(super) fn stamp_transient_block_65(
        locals: &mut StampLocals,
    ) {
        let (assign25390_e19416, assign25390_e19416_d_n3, assign25390_e19416_d_n4, assign25390_e19416_d_n5, assign25390_e19416_d_n6, assign25390_e19416_d_n7, assign25390_e19416_d_n8, assign25390_e19416_d_n9, assign25390_e19416_d_n10, assign25390_e19416_d_n11, assign25390_e19416_d_n12,) = {
    if ((((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1616 == 0.0)) && (locals.var_guard1617 == 0.0)) {
        let assign25390_e19414: f64 = (1.0 / locals.var_t2__blk1146);
        (assign25390_e19414, (-(locals.var_t2__blk1146_dn3 / (locals.var_t2__blk1146 * locals.var_t2__blk1146))), (-(locals.var_t2__blk1146_dn4 / (locals.var_t2__blk1146 * locals.var_t2__blk1146))), (-(locals.var_t2__blk1146_dn5 / (locals.var_t2__blk1146 * locals.var_t2__blk1146))), (-(locals.var_t2__blk1146_dn6 / (locals.var_t2__blk1146 * locals.var_t2__blk1146))), (-(locals.var_t2__blk1146_dn7 / (locals.var_t2__blk1146 * locals.var_t2__blk1146))), (-(locals.var_t2__blk1146_dn8 / (locals.var_t2__blk1146 * locals.var_t2__blk1146))), (-(locals.var_t2__blk1146_dn9 / (locals.var_t2__blk1146 * locals.var_t2__blk1146))), (-(locals.var_t2__blk1146_dn10 / (locals.var_t2__blk1146 * locals.var_t2__blk1146))), (-(locals.var_t2__blk1146_dn11 / (locals.var_t2__blk1146 * locals.var_t2__blk1146))), (-(locals.var_t2__blk1146_dn12 / (locals.var_t2__blk1146 * locals.var_t2__blk1146))),)
    } else {
        (locals.var_e2ndfactor, locals.var_e2ndfactor_dn3, locals.var_e2ndfactor_dn4, locals.var_e2ndfactor_dn5, locals.var_e2ndfactor_dn6, locals.var_e2ndfactor_dn7, locals.var_e2ndfactor_dn8, locals.var_e2ndfactor_dn9, locals.var_e2ndfactor_dn10, locals.var_e2ndfactor_dn11, locals.var_e2ndfactor_dn12,)
    }
};
        locals.var_e2ndfactor = assign25390_e19416;
        locals.var_e2ndfactor_dn3 = assign25390_e19416_d_n3;
        locals.var_e2ndfactor_dn4 = assign25390_e19416_d_n4;
        locals.var_e2ndfactor_dn5 = assign25390_e19416_d_n5;
        locals.var_e2ndfactor_dn6 = assign25390_e19416_d_n6;
        locals.var_e2ndfactor_dn7 = assign25390_e19416_d_n7;
        locals.var_e2ndfactor_dn8 = assign25390_e19416_d_n8;
        locals.var_e2ndfactor_dn9 = assign25390_e19416_d_n9;
        locals.var_e2ndfactor_dn10 = assign25390_e19416_d_n10;
        locals.var_e2ndfactor_dn11 = assign25390_e19416_d_n11;
        locals.var_e2ndfactor_dn12 = assign25390_e19416_d_n12;

        let (assign25400_e19428, assign25400_e19428_d_n3, assign25400_e19428_d_n4, assign25400_e19428_d_n5, assign25400_e19428_d_n6, assign25400_e19428_d_n7, assign25400_e19428_d_n8, assign25400_e19428_d_n9, assign25400_e19428_d_n10, assign25400_e19428_d_n11, assign25400_e19428_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1616 == 0.0)) {
        let assign25400_e19426: f64 = (locals.var_pparam_b4soiarfabjt * locals.var_ien);
        (assign25400_e19426, ((locals.var_pparam_b4soiarfabjt_dn3 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn3)), ((locals.var_pparam_b4soiarfabjt_dn4 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn4)), ((locals.var_pparam_b4soiarfabjt_dn5 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn5)), ((locals.var_pparam_b4soiarfabjt_dn6 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn6)), ((locals.var_pparam_b4soiarfabjt_dn7 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn7)), ((locals.var_pparam_b4soiarfabjt_dn8 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn8)), ((locals.var_pparam_b4soiarfabjt_dn9 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn9)), ((locals.var_pparam_b4soiarfabjt_dn10 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn10)), ((locals.var_pparam_b4soiarfabjt_dn11 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn11)), ((locals.var_pparam_b4soiarfabjt_dn12 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign25400_e19428;
        locals.var_t0__blk1144_dn3 = assign25400_e19428_d_n3;
        locals.var_t0__blk1144_dn4 = assign25400_e19428_d_n4;
        locals.var_t0__blk1144_dn5 = assign25400_e19428_d_n5;
        locals.var_t0__blk1144_dn6 = assign25400_e19428_d_n6;
        locals.var_t0__blk1144_dn7 = assign25400_e19428_d_n7;
        locals.var_t0__blk1144_dn8 = assign25400_e19428_d_n8;
        locals.var_t0__blk1144_dn9 = assign25400_e19428_d_n9;
        locals.var_t0__blk1144_dn10 = assign25400_e19428_d_n10;
        locals.var_t0__blk1144_dn11 = assign25400_e19428_d_n11;
        locals.var_t0__blk1144_dn12 = assign25400_e19428_d_n12;

        let (assign25410_e19444, assign25410_e19444_d_n3, assign25410_e19444_d_n4, assign25410_e19444_d_n5, assign25410_e19444_d_n6, assign25410_e19444_d_n7, assign25410_e19444_d_n8, assign25410_e19444_d_n9, assign25410_e19444_d_n10, assign25410_e19444_d_n11, assign25410_e19444_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1613 == 0.0)) && (locals.var_guard1616 == 0.0)) {
        let assign25410_e19439: f64 = (locals.var_expvbsnvtm - locals.var_expvbdnvtm);
        let assign25410_e19440: f64 = (locals.var_t0__blk1144 * assign25410_e19439);
        let assign25410_e19442: f64 = (assign25410_e19440 * locals.var_e2ndfactor);
        (assign25410_e19442, ((((locals.var_t0__blk1144_dn3 * assign25410_e19439) + (locals.var_t0__blk1144 * (locals.var_expvbsnvtm_dn3 - locals.var_expvbdnvtm_dn3))) * locals.var_e2ndfactor) + (assign25410_e19440 * locals.var_e2ndfactor_dn3)), ((((locals.var_t0__blk1144_dn4 * assign25410_e19439) + (locals.var_t0__blk1144 * (locals.var_expvbsnvtm_dn4 - locals.var_expvbdnvtm_dn4))) * locals.var_e2ndfactor) + (assign25410_e19440 * locals.var_e2ndfactor_dn4)), ((((locals.var_t0__blk1144_dn5 * assign25410_e19439) + (locals.var_t0__blk1144 * (locals.var_expvbsnvtm_dn5 - locals.var_expvbdnvtm_dn5))) * locals.var_e2ndfactor) + (assign25410_e19440 * locals.var_e2ndfactor_dn5)), ((((locals.var_t0__blk1144_dn6 * assign25410_e19439) + (locals.var_t0__blk1144 * (locals.var_expvbsnvtm_dn6 - locals.var_expvbdnvtm_dn6))) * locals.var_e2ndfactor) + (assign25410_e19440 * locals.var_e2ndfactor_dn6)), ((((locals.var_t0__blk1144_dn7 * assign25410_e19439) + (locals.var_t0__blk1144 * (locals.var_expvbsnvtm_dn7 - locals.var_expvbdnvtm_dn7))) * locals.var_e2ndfactor) + (assign25410_e19440 * locals.var_e2ndfactor_dn7)), ((((locals.var_t0__blk1144_dn8 * assign25410_e19439) + (locals.var_t0__blk1144 * (locals.var_expvbsnvtm_dn8 - locals.var_expvbdnvtm_dn8))) * locals.var_e2ndfactor) + (assign25410_e19440 * locals.var_e2ndfactor_dn8)), ((((locals.var_t0__blk1144_dn9 * assign25410_e19439) + (locals.var_t0__blk1144 * (locals.var_expvbsnvtm_dn9 - locals.var_expvbdnvtm_dn9))) * locals.var_e2ndfactor) + (assign25410_e19440 * locals.var_e2ndfactor_dn9)), ((((locals.var_t0__blk1144_dn10 * assign25410_e19439) + (locals.var_t0__blk1144 * (locals.var_expvbsnvtm_dn10 - locals.var_expvbdnvtm_dn10))) * locals.var_e2ndfactor) + (assign25410_e19440 * locals.var_e2ndfactor_dn10)), ((((locals.var_t0__blk1144_dn11 * assign25410_e19439) + (locals.var_t0__blk1144 * (locals.var_expvbsnvtm_dn11 - locals.var_expvbdnvtm_dn11))) * locals.var_e2ndfactor) + (assign25410_e19440 * locals.var_e2ndfactor_dn11)), ((((locals.var_t0__blk1144_dn12 * assign25410_e19439) + (locals.var_t0__blk1144 * (locals.var_expvbsnvtm_dn12 - locals.var_expvbdnvtm_dn12))) * locals.var_e2ndfactor) + (assign25410_e19440 * locals.var_e2ndfactor_dn12)),)
    } else {
        (locals.var_ic_1, locals.var_ic_1_dn3, locals.var_ic_1_dn4, locals.var_ic_1_dn5, locals.var_ic_1_dn6, locals.var_ic_1_dn7, locals.var_ic_1_dn8, locals.var_ic_1_dn9, locals.var_ic_1_dn10, locals.var_ic_1_dn11, locals.var_ic_1_dn12,)
    }
};
        locals.var_ic_1 = assign25410_e19444;
        locals.var_ic_1_dn3 = assign25410_e19444_d_n3;
        locals.var_ic_1_dn4 = assign25410_e19444_d_n4;
        locals.var_ic_1_dn5 = assign25410_e19444_d_n5;
        locals.var_ic_1_dn6 = assign25410_e19444_d_n6;
        locals.var_ic_1_dn7 = assign25410_e19444_d_n7;
        locals.var_ic_1_dn8 = assign25410_e19444_d_n8;
        locals.var_ic_1_dn9 = assign25410_e19444_d_n9;
        locals.var_ic_1_dn10 = assign25410_e19444_d_n10;
        locals.var_ic_1_dn11 = assign25410_e19444_d_n11;
        locals.var_ic_1_dn12 = assign25410_e19444_d_n12;

        let assign25420_e19451: f64 = if ((locals.var_jtuns == 0.0) && (locals.var_jtund == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1618 = assign25420_e19451;

        let (assign25430_e19457, assign25430_e19457_d_n3, assign25430_e19457_d_n4, assign25430_e19457_d_n5, assign25430_e19457_d_n6, assign25430_e19457_d_n7, assign25430_e19457_d_n8, assign25430_e19457_d_n9, assign25430_e19457_d_n10, assign25430_e19457_d_n11, assign25430_e19457_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1618 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd4, locals.var_ibd4_dn3, locals.var_ibd4_dn4, locals.var_ibd4_dn5, locals.var_ibd4_dn6, locals.var_ibd4_dn7, locals.var_ibd4_dn8, locals.var_ibd4_dn9, locals.var_ibd4_dn10, locals.var_ibd4_dn11, locals.var_ibd4_dn12,)
    }
};
        locals.var_ibd4 = assign25430_e19457;
        locals.var_ibd4_dn3 = assign25430_e19457_d_n3;
        locals.var_ibd4_dn4 = assign25430_e19457_d_n4;
        locals.var_ibd4_dn5 = assign25430_e19457_d_n5;
        locals.var_ibd4_dn6 = assign25430_e19457_d_n6;
        locals.var_ibd4_dn7 = assign25430_e19457_d_n7;
        locals.var_ibd4_dn8 = assign25430_e19457_d_n8;
        locals.var_ibd4_dn9 = assign25430_e19457_d_n9;
        locals.var_ibd4_dn10 = assign25430_e19457_d_n10;
        locals.var_ibd4_dn11 = assign25430_e19457_d_n11;
        locals.var_ibd4_dn12 = assign25430_e19457_d_n12;

        let (assign25440_e19463, assign25440_e19463_d_n3, assign25440_e19463_d_n4, assign25440_e19463_d_n5, assign25440_e19463_d_n6, assign25440_e19463_d_n7, assign25440_e19463_d_n8, assign25440_e19463_d_n9, assign25440_e19463_d_n10, assign25440_e19463_d_n11, assign25440_e19463_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1618 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs4, locals.var_ibs4_dn3, locals.var_ibs4_dn4, locals.var_ibs4_dn5, locals.var_ibs4_dn6, locals.var_ibs4_dn7, locals.var_ibs4_dn8, locals.var_ibs4_dn9, locals.var_ibs4_dn10, locals.var_ibs4_dn11, locals.var_ibs4_dn12,)
    }
};
        locals.var_ibs4 = assign25440_e19463;
        locals.var_ibs4_dn3 = assign25440_e19463_d_n3;
        locals.var_ibs4_dn4 = assign25440_e19463_d_n4;
        locals.var_ibs4_dn5 = assign25440_e19463_d_n5;
        locals.var_ibs4_dn6 = assign25440_e19463_d_n6;
        locals.var_ibs4_dn7 = assign25440_e19463_d_n7;
        locals.var_ibs4_dn8 = assign25440_e19463_d_n8;
        locals.var_ibs4_dn9 = assign25440_e19463_d_n9;
        locals.var_ibs4_dn10 = assign25440_e19463_d_n10;
        locals.var_ibs4_dn11 = assign25440_e19463_d_n11;
        locals.var_ibs4_dn12 = assign25440_e19463_d_n12;

        let (assign25450_e19472, assign25450_e19472_d_n3, assign25450_e19472_d_n4, assign25450_e19472_d_n5, assign25450_e19472_d_n6, assign25450_e19472_d_n7, assign25450_e19472_d_n8, assign25450_e19472_d_n9, assign25450_e19472_d_n10, assign25450_e19472_d_n11, assign25450_e19472_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) {
        let assign25450_e19470: f64 = (locals.var_vtm00 * locals.var_pparam_b4sointun);
        (assign25450_e19470, (locals.var_vtm00 * locals.var_pparam_b4sointun_dn3), (locals.var_vtm00 * locals.var_pparam_b4sointun_dn4), (locals.var_vtm00 * locals.var_pparam_b4sointun_dn5), (locals.var_vtm00 * locals.var_pparam_b4sointun_dn6), (locals.var_vtm00 * locals.var_pparam_b4sointun_dn7), (locals.var_vtm00 * locals.var_pparam_b4sointun_dn8), (locals.var_vtm00 * locals.var_pparam_b4sointun_dn9), (locals.var_vtm00 * locals.var_pparam_b4sointun_dn10), (locals.var_vtm00 * locals.var_pparam_b4sointun_dn11), (locals.var_vtm00 * locals.var_pparam_b4sointun_dn12),)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn3, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5, locals.var_nvtm2_dn6, locals.var_nvtm2_dn7, locals.var_nvtm2_dn8, locals.var_nvtm2_dn9, locals.var_nvtm2_dn10, locals.var_nvtm2_dn11, locals.var_nvtm2_dn12,)
    }
};
        locals.var_nvtm2 = assign25450_e19472;
        locals.var_nvtm2_dn3 = assign25450_e19472_d_n3;
        locals.var_nvtm2_dn4 = assign25450_e19472_d_n4;
        locals.var_nvtm2_dn5 = assign25450_e19472_d_n5;
        locals.var_nvtm2_dn6 = assign25450_e19472_d_n6;
        locals.var_nvtm2_dn7 = assign25450_e19472_d_n7;
        locals.var_nvtm2_dn8 = assign25450_e19472_d_n8;
        locals.var_nvtm2_dn9 = assign25450_e19472_d_n9;
        locals.var_nvtm2_dn10 = assign25450_e19472_d_n10;
        locals.var_nvtm2_dn11 = assign25450_e19472_d_n11;
        locals.var_nvtm2_dn12 = assign25450_e19472_d_n12;

        let assign25460_e19475: f64 = (locals.var_pparam_b4soivtun0 - locals.var_vsbs);
        let assign25460_e19477: f64 = if assign25460_e19475 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1619 = assign25460_e19477;

        let (assign25470_e19486, assign25470_e19486_d_n3, assign25470_e19486_d_n4, assign25470_e19486_d_n5, assign25470_e19486_d_n6, assign25470_e19486_d_n7, assign25470_e19486_d_n8, assign25470_e19486_d_n9, assign25470_e19486_d_n10, assign25470_e19486_d_n11, assign25470_e19486_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25470_e19486;
        locals.var_t1__blk1145_dn3 = assign25470_e19486_d_n3;
        locals.var_t1__blk1145_dn4 = assign25470_e19486_d_n4;
        locals.var_t1__blk1145_dn5 = assign25470_e19486_d_n5;
        locals.var_t1__blk1145_dn6 = assign25470_e19486_d_n6;
        locals.var_t1__blk1145_dn7 = assign25470_e19486_d_n7;
        locals.var_t1__blk1145_dn8 = assign25470_e19486_d_n8;
        locals.var_t1__blk1145_dn9 = assign25470_e19486_d_n9;
        locals.var_t1__blk1145_dn10 = assign25470_e19486_d_n10;
        locals.var_t1__blk1145_dn11 = assign25470_e19486_d_n11;
        locals.var_t1__blk1145_dn12 = assign25470_e19486_d_n12;

        let (assign25480_e19502, assign25480_e19502_d_n3, assign25480_e19502_d_n4, assign25480_e19502_d_n5, assign25480_e19502_d_n6, assign25480_e19502_d_n7, assign25480_e19502_d_n8, assign25480_e19502_d_n9, assign25480_e19502_d_n10, assign25480_e19502_d_n11, assign25480_e19502_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 != 0.0)) {
        let assign25480_e19494: f64 = (-locals.var_vsbs);
        let assign25480_e19496: f64 = (assign25480_e19494 / locals.var_nvtm2);
        let assign25480_e19498: f64 = (assign25480_e19496 * locals.var_pparam_b4soivtun0);
        let assign25480_e19500: f64 = (assign25480_e19498 * locals.var_t1__blk1145);
        (assign25480_e19500, (((((-((assign25480_e19494 * locals.var_nvtm2_dn3) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25480_e19496 * locals.var_pparam_b4soivtun0_dn3)) * locals.var_t1__blk1145) + (assign25480_e19498 * locals.var_t1__blk1145_dn3)), (((((-((assign25480_e19494 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25480_e19496 * locals.var_pparam_b4soivtun0_dn4)) * locals.var_t1__blk1145) + (assign25480_e19498 * locals.var_t1__blk1145_dn4)), (((((-((assign25480_e19494 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25480_e19496 * locals.var_pparam_b4soivtun0_dn5)) * locals.var_t1__blk1145) + (assign25480_e19498 * locals.var_t1__blk1145_dn5)), (((((-((assign25480_e19494 * locals.var_nvtm2_dn6) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25480_e19496 * locals.var_pparam_b4soivtun0_dn6)) * locals.var_t1__blk1145) + (assign25480_e19498 * locals.var_t1__blk1145_dn6)), (((((-((assign25480_e19494 * locals.var_nvtm2_dn7) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25480_e19496 * locals.var_pparam_b4soivtun0_dn7)) * locals.var_t1__blk1145) + (assign25480_e19498 * locals.var_t1__blk1145_dn7)), ((((((((-locals.var_vsbs_dn8) * locals.var_nvtm2) - (assign25480_e19494 * locals.var_nvtm2_dn8)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0) + (assign25480_e19496 * locals.var_pparam_b4soivtun0_dn8)) * locals.var_t1__blk1145) + (assign25480_e19498 * locals.var_t1__blk1145_dn8)), (((((-((assign25480_e19494 * locals.var_nvtm2_dn9) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25480_e19496 * locals.var_pparam_b4soivtun0_dn9)) * locals.var_t1__blk1145) + (assign25480_e19498 * locals.var_t1__blk1145_dn9)), (((((-((assign25480_e19494 * locals.var_nvtm2_dn10) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25480_e19496 * locals.var_pparam_b4soivtun0_dn10)) * locals.var_t1__blk1145) + (assign25480_e19498 * locals.var_t1__blk1145_dn10)), ((((((((-locals.var_vsbs_dn11) * locals.var_nvtm2) - (assign25480_e19494 * locals.var_nvtm2_dn11)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0) + (assign25480_e19496 * locals.var_pparam_b4soivtun0_dn11)) * locals.var_t1__blk1145) + (assign25480_e19498 * locals.var_t1__blk1145_dn11)), (((((-((assign25480_e19494 * locals.var_nvtm2_dn12) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25480_e19496 * locals.var_pparam_b4soivtun0_dn12)) * locals.var_t1__blk1145) + (assign25480_e19498 * locals.var_t1__blk1145_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign25480_e19502;
        locals.var_t0__blk1144_dn3 = assign25480_e19502_d_n3;
        locals.var_t0__blk1144_dn4 = assign25480_e19502_d_n4;
        locals.var_t0__blk1144_dn5 = assign25480_e19502_d_n5;
        locals.var_t0__blk1144_dn6 = assign25480_e19502_d_n6;
        locals.var_t0__blk1144_dn7 = assign25480_e19502_d_n7;
        locals.var_t0__blk1144_dn8 = assign25480_e19502_d_n8;
        locals.var_t0__blk1144_dn9 = assign25480_e19502_d_n9;
        locals.var_t0__blk1144_dn10 = assign25480_e19502_d_n10;
        locals.var_t0__blk1144_dn11 = assign25480_e19502_d_n11;
        locals.var_t0__blk1144_dn12 = assign25480_e19502_d_n12;

        let assign25490_e19505: f64 = if locals.var_t0__blk1144 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1620 = assign25490_e19505;

        let (assign25500_e19522, assign25500_e19522_d_n3, assign25500_e19522_d_n4, assign25500_e19522_d_n5, assign25500_e19522_d_n6, assign25500_e19522_d_n7, assign25500_e19522_d_n8, assign25500_e19522_d_n9, assign25500_e19522_d_n10, assign25500_e19522_d_n11, assign25500_e19522_d_n12,) = {
    if ((((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 != 0.0)) && (locals.var_guard1620 != 0.0)) {
        let assign25500_e19517: f64 = (1.0 + locals.var_t0__blk1144);
        let assign25500_e19519: f64 = (assign25500_e19517 - 100.0);
        let assign25500_e19520: f64 = (2.688117142e43 * assign25500_e19519);
        (assign25500_e19520, (2.688117142e43 * locals.var_t0__blk1144_dn3), (2.688117142e43 * locals.var_t0__blk1144_dn4), (2.688117142e43 * locals.var_t0__blk1144_dn5), (2.688117142e43 * locals.var_t0__blk1144_dn6), (2.688117142e43 * locals.var_t0__blk1144_dn7), (2.688117142e43 * locals.var_t0__blk1144_dn8), (2.688117142e43 * locals.var_t0__blk1144_dn9), (2.688117142e43 * locals.var_t0__blk1144_dn10), (2.688117142e43 * locals.var_t0__blk1144_dn11), (2.688117142e43 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25500_e19522;
        locals.var_t1__blk1145_dn3 = assign25500_e19522_d_n3;
        locals.var_t1__blk1145_dn4 = assign25500_e19522_d_n4;
        locals.var_t1__blk1145_dn5 = assign25500_e19522_d_n5;
        locals.var_t1__blk1145_dn6 = assign25500_e19522_d_n6;
        locals.var_t1__blk1145_dn7 = assign25500_e19522_d_n7;
        locals.var_t1__blk1145_dn8 = assign25500_e19522_d_n8;
        locals.var_t1__blk1145_dn9 = assign25500_e19522_d_n9;
        locals.var_t1__blk1145_dn10 = assign25500_e19522_d_n10;
        locals.var_t1__blk1145_dn11 = assign25500_e19522_d_n11;
        locals.var_t1__blk1145_dn12 = assign25500_e19522_d_n12;

        let assign25510_e19525: f64 = (-100.0);
        let assign25510_e19526: f64 = if locals.var_t0__blk1144 < assign25510_e19525 { 1.0 } else { 0.0 };
        locals.var_guard1621 = assign25510_e19526;

        let (assign25520_e19540, assign25520_e19540_d_n3, assign25520_e19540_d_n4, assign25520_e19540_d_n5, assign25520_e19540_d_n6, assign25520_e19540_d_n7, assign25520_e19540_d_n8, assign25520_e19540_d_n9, assign25520_e19540_d_n10, assign25520_e19540_d_n11, assign25520_e19540_d_n12,) = {
    if (((((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 != 0.0)) && (locals.var_guard1620 == 0.0)) && (locals.var_guard1621 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25520_e19540;
        locals.var_t1__blk1145_dn3 = assign25520_e19540_d_n3;
        locals.var_t1__blk1145_dn4 = assign25520_e19540_d_n4;
        locals.var_t1__blk1145_dn5 = assign25520_e19540_d_n5;
        locals.var_t1__blk1145_dn6 = assign25520_e19540_d_n6;
        locals.var_t1__blk1145_dn7 = assign25520_e19540_d_n7;
        locals.var_t1__blk1145_dn8 = assign25520_e19540_d_n8;
        locals.var_t1__blk1145_dn9 = assign25520_e19540_d_n9;
        locals.var_t1__blk1145_dn10 = assign25520_e19540_d_n10;
        locals.var_t1__blk1145_dn11 = assign25520_e19540_d_n11;
        locals.var_t1__blk1145_dn12 = assign25520_e19540_d_n12;

        let (assign25530_e19556, assign25530_e19556_d_n3, assign25530_e19556_d_n4, assign25530_e19556_d_n5, assign25530_e19556_d_n6, assign25530_e19556_d_n7, assign25530_e19556_d_n8, assign25530_e19556_d_n9, assign25530_e19556_d_n10, assign25530_e19556_d_n11, assign25530_e19556_d_n12,) = {
    if (((((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 != 0.0)) && (locals.var_guard1620 == 0.0)) && (locals.var_guard1621 == 0.0)) {
        let assign25530_e19554: f64 = (locals.var_t0__blk1144).exp();
        (assign25530_e19554, (assign25530_e19554 * locals.var_t0__blk1144_dn3), (assign25530_e19554 * locals.var_t0__blk1144_dn4), (assign25530_e19554 * locals.var_t0__blk1144_dn5), (assign25530_e19554 * locals.var_t0__blk1144_dn6), (assign25530_e19554 * locals.var_t0__blk1144_dn7), (assign25530_e19554 * locals.var_t0__blk1144_dn8), (assign25530_e19554 * locals.var_t0__blk1144_dn9), (assign25530_e19554 * locals.var_t0__blk1144_dn10), (assign25530_e19554 * locals.var_t0__blk1144_dn11), (assign25530_e19554 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25530_e19556;
        locals.var_t1__blk1145_dn3 = assign25530_e19556_d_n3;
        locals.var_t1__blk1145_dn4 = assign25530_e19556_d_n4;
        locals.var_t1__blk1145_dn5 = assign25530_e19556_d_n5;
        locals.var_t1__blk1145_dn6 = assign25530_e19556_d_n6;
        locals.var_t1__blk1145_dn7 = assign25530_e19556_d_n7;
        locals.var_t1__blk1145_dn8 = assign25530_e19556_d_n8;
        locals.var_t1__blk1145_dn9 = assign25530_e19556_d_n9;
        locals.var_t1__blk1145_dn10 = assign25530_e19556_d_n10;
        locals.var_t1__blk1145_dn11 = assign25530_e19556_d_n11;
        locals.var_t1__blk1145_dn12 = assign25530_e19556_d_n12;

        let (assign25540_e19567, assign25540_e19567_d_n3, assign25540_e19567_d_n4, assign25540_e19567_d_n5, assign25540_e19567_d_n6, assign25540_e19567_d_n7, assign25540_e19567_d_n8, assign25540_e19567_d_n9, assign25540_e19567_d_n10, assign25540_e19567_d_n11, assign25540_e19567_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 != 0.0)) {
        let assign25540_e19565: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign25540_e19565, ((locals.var_wstsi_dn3 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn3)), ((locals.var_wstsi_dn4 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn4)), ((locals.var_wstsi_dn5 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn5)), ((locals.var_wstsi_dn6 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn6)), ((locals.var_wstsi_dn7 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn7)), ((locals.var_wstsi_dn8 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn8)), ((locals.var_wstsi_dn9 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn9)), ((locals.var_wstsi_dn10 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn10)), ((locals.var_wstsi_dn11 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn11)), ((locals.var_wstsi_dn12 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn12)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign25540_e19567;
        locals.var_t3__blk1147_dn3 = assign25540_e19567_d_n3;
        locals.var_t3__blk1147_dn4 = assign25540_e19567_d_n4;
        locals.var_t3__blk1147_dn5 = assign25540_e19567_d_n5;
        locals.var_t3__blk1147_dn6 = assign25540_e19567_d_n6;
        locals.var_t3__blk1147_dn7 = assign25540_e19567_d_n7;
        locals.var_t3__blk1147_dn8 = assign25540_e19567_d_n8;
        locals.var_t3__blk1147_dn9 = assign25540_e19567_d_n9;
        locals.var_t3__blk1147_dn10 = assign25540_e19567_d_n10;
        locals.var_t3__blk1147_dn11 = assign25540_e19567_d_n11;
        locals.var_t3__blk1147_dn12 = assign25540_e19567_d_n12;

        let (assign25550_e19580, assign25550_e19580_d_n3, assign25550_e19580_d_n4, assign25550_e19580_d_n5, assign25550_e19580_d_n6, assign25550_e19580_d_n7, assign25550_e19580_d_n8, assign25550_e19580_d_n9, assign25550_e19580_d_n10, assign25550_e19580_d_n11, assign25550_e19580_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 != 0.0)) {
        let assign25550_e19577: f64 = (1.0 - locals.var_t1__blk1145);
        let assign25550_e19578: f64 = (locals.var_t3__blk1147 * assign25550_e19577);
        (assign25550_e19578, ((locals.var_t3__blk1147_dn3 * assign25550_e19577) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn3))), ((locals.var_t3__blk1147_dn4 * assign25550_e19577) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn4))), ((locals.var_t3__blk1147_dn5 * assign25550_e19577) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn5))), ((locals.var_t3__blk1147_dn6 * assign25550_e19577) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn6))), ((locals.var_t3__blk1147_dn7 * assign25550_e19577) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn7))), ((locals.var_t3__blk1147_dn8 * assign25550_e19577) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn8))), ((locals.var_t3__blk1147_dn9 * assign25550_e19577) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn9))), ((locals.var_t3__blk1147_dn10 * assign25550_e19577) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn10))), ((locals.var_t3__blk1147_dn11 * assign25550_e19577) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn11))), ((locals.var_t3__blk1147_dn12 * assign25550_e19577) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn12))),)
    } else {
        (locals.var_ibs4, locals.var_ibs4_dn3, locals.var_ibs4_dn4, locals.var_ibs4_dn5, locals.var_ibs4_dn6, locals.var_ibs4_dn7, locals.var_ibs4_dn8, locals.var_ibs4_dn9, locals.var_ibs4_dn10, locals.var_ibs4_dn11, locals.var_ibs4_dn12,)
    }
};
        locals.var_ibs4 = assign25550_e19580;
        locals.var_ibs4_dn3 = assign25550_e19580_d_n3;
        locals.var_ibs4_dn4 = assign25550_e19580_d_n4;
        locals.var_ibs4_dn5 = assign25550_e19580_d_n5;
        locals.var_ibs4_dn6 = assign25550_e19580_d_n6;
        locals.var_ibs4_dn7 = assign25550_e19580_d_n7;
        locals.var_ibs4_dn8 = assign25550_e19580_d_n8;
        locals.var_ibs4_dn9 = assign25550_e19580_d_n9;
        locals.var_ibs4_dn10 = assign25550_e19580_d_n10;
        locals.var_ibs4_dn11 = assign25550_e19580_d_n11;
        locals.var_ibs4_dn12 = assign25550_e19580_d_n12;

        let (assign25560_e19594, assign25560_e19594_d_n3, assign25560_e19594_d_n4, assign25560_e19594_d_n5, assign25560_e19594_d_n6, assign25560_e19594_d_n7, assign25560_e19594_d_n8, assign25560_e19594_d_n9, assign25560_e19594_d_n10, assign25560_e19594_d_n11, assign25560_e19594_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 == 0.0)) {
        let assign25560_e19591: f64 = (locals.var_pparam_b4soivtun0 - locals.var_vsbs);
        let assign25560_e19592: f64 = (1.0 / assign25560_e19591);
        (assign25560_e19592, (-(locals.var_pparam_b4soivtun0_dn3 / (assign25560_e19591 * assign25560_e19591))), (-(locals.var_pparam_b4soivtun0_dn4 / (assign25560_e19591 * assign25560_e19591))), (-(locals.var_pparam_b4soivtun0_dn5 / (assign25560_e19591 * assign25560_e19591))), (-(locals.var_pparam_b4soivtun0_dn6 / (assign25560_e19591 * assign25560_e19591))), (-(locals.var_pparam_b4soivtun0_dn7 / (assign25560_e19591 * assign25560_e19591))), (-((locals.var_pparam_b4soivtun0_dn8 - locals.var_vsbs_dn8) / (assign25560_e19591 * assign25560_e19591))), (-(locals.var_pparam_b4soivtun0_dn9 / (assign25560_e19591 * assign25560_e19591))), (-(locals.var_pparam_b4soivtun0_dn10 / (assign25560_e19591 * assign25560_e19591))), (-((locals.var_pparam_b4soivtun0_dn11 - locals.var_vsbs_dn11) / (assign25560_e19591 * assign25560_e19591))), (-(locals.var_pparam_b4soivtun0_dn12 / (assign25560_e19591 * assign25560_e19591))),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25560_e19594;
        locals.var_t1__blk1145_dn3 = assign25560_e19594_d_n3;
        locals.var_t1__blk1145_dn4 = assign25560_e19594_d_n4;
        locals.var_t1__blk1145_dn5 = assign25560_e19594_d_n5;
        locals.var_t1__blk1145_dn6 = assign25560_e19594_d_n6;
        locals.var_t1__blk1145_dn7 = assign25560_e19594_d_n7;
        locals.var_t1__blk1145_dn8 = assign25560_e19594_d_n8;
        locals.var_t1__blk1145_dn9 = assign25560_e19594_d_n9;
        locals.var_t1__blk1145_dn10 = assign25560_e19594_d_n10;
        locals.var_t1__blk1145_dn11 = assign25560_e19594_d_n11;
        locals.var_t1__blk1145_dn12 = assign25560_e19594_d_n12;

        let (assign25570_e19611, assign25570_e19611_d_n3, assign25570_e19611_d_n4, assign25570_e19611_d_n5, assign25570_e19611_d_n6, assign25570_e19611_d_n7, assign25570_e19611_d_n8, assign25570_e19611_d_n9, assign25570_e19611_d_n10, assign25570_e19611_d_n11, assign25570_e19611_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 == 0.0)) {
        let assign25570_e19603: f64 = (-locals.var_vsbs);
        let assign25570_e19605: f64 = (assign25570_e19603 / locals.var_nvtm2);
        let assign25570_e19607: f64 = (assign25570_e19605 * locals.var_pparam_b4soivtun0);
        let assign25570_e19609: f64 = (assign25570_e19607 * locals.var_t1__blk1145);
        (assign25570_e19609, (((((-((assign25570_e19603 * locals.var_nvtm2_dn3) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25570_e19605 * locals.var_pparam_b4soivtun0_dn3)) * locals.var_t1__blk1145) + (assign25570_e19607 * locals.var_t1__blk1145_dn3)), (((((-((assign25570_e19603 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25570_e19605 * locals.var_pparam_b4soivtun0_dn4)) * locals.var_t1__blk1145) + (assign25570_e19607 * locals.var_t1__blk1145_dn4)), (((((-((assign25570_e19603 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25570_e19605 * locals.var_pparam_b4soivtun0_dn5)) * locals.var_t1__blk1145) + (assign25570_e19607 * locals.var_t1__blk1145_dn5)), (((((-((assign25570_e19603 * locals.var_nvtm2_dn6) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25570_e19605 * locals.var_pparam_b4soivtun0_dn6)) * locals.var_t1__blk1145) + (assign25570_e19607 * locals.var_t1__blk1145_dn6)), (((((-((assign25570_e19603 * locals.var_nvtm2_dn7) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25570_e19605 * locals.var_pparam_b4soivtun0_dn7)) * locals.var_t1__blk1145) + (assign25570_e19607 * locals.var_t1__blk1145_dn7)), ((((((((-locals.var_vsbs_dn8) * locals.var_nvtm2) - (assign25570_e19603 * locals.var_nvtm2_dn8)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0) + (assign25570_e19605 * locals.var_pparam_b4soivtun0_dn8)) * locals.var_t1__blk1145) + (assign25570_e19607 * locals.var_t1__blk1145_dn8)), (((((-((assign25570_e19603 * locals.var_nvtm2_dn9) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25570_e19605 * locals.var_pparam_b4soivtun0_dn9)) * locals.var_t1__blk1145) + (assign25570_e19607 * locals.var_t1__blk1145_dn9)), (((((-((assign25570_e19603 * locals.var_nvtm2_dn10) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25570_e19605 * locals.var_pparam_b4soivtun0_dn10)) * locals.var_t1__blk1145) + (assign25570_e19607 * locals.var_t1__blk1145_dn10)), ((((((((-locals.var_vsbs_dn11) * locals.var_nvtm2) - (assign25570_e19603 * locals.var_nvtm2_dn11)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0) + (assign25570_e19605 * locals.var_pparam_b4soivtun0_dn11)) * locals.var_t1__blk1145) + (assign25570_e19607 * locals.var_t1__blk1145_dn11)), (((((-((assign25570_e19603 * locals.var_nvtm2_dn12) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign25570_e19605 * locals.var_pparam_b4soivtun0_dn12)) * locals.var_t1__blk1145) + (assign25570_e19607 * locals.var_t1__blk1145_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign25570_e19611;
        locals.var_t0__blk1144_dn3 = assign25570_e19611_d_n3;
        locals.var_t0__blk1144_dn4 = assign25570_e19611_d_n4;
        locals.var_t0__blk1144_dn5 = assign25570_e19611_d_n5;
        locals.var_t0__blk1144_dn6 = assign25570_e19611_d_n6;
        locals.var_t0__blk1144_dn7 = assign25570_e19611_d_n7;
        locals.var_t0__blk1144_dn8 = assign25570_e19611_d_n8;
        locals.var_t0__blk1144_dn9 = assign25570_e19611_d_n9;
        locals.var_t0__blk1144_dn10 = assign25570_e19611_d_n10;
        locals.var_t0__blk1144_dn11 = assign25570_e19611_d_n11;
        locals.var_t0__blk1144_dn12 = assign25570_e19611_d_n12;

        let assign25580_e19614: f64 = if locals.var_t0__blk1144 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1622 = assign25580_e19614;

        let (assign25590_e19632, assign25590_e19632_d_n3, assign25590_e19632_d_n4, assign25590_e19632_d_n5, assign25590_e19632_d_n6, assign25590_e19632_d_n7, assign25590_e19632_d_n8, assign25590_e19632_d_n9, assign25590_e19632_d_n10, assign25590_e19632_d_n11, assign25590_e19632_d_n12,) = {
    if ((((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 == 0.0)) && (locals.var_guard1622 != 0.0)) {
        let assign25590_e19627: f64 = (1.0 + locals.var_t0__blk1144);
        let assign25590_e19629: f64 = (assign25590_e19627 - 100.0);
        let assign25590_e19630: f64 = (2.688117142e43 * assign25590_e19629);
        (assign25590_e19630, (2.688117142e43 * locals.var_t0__blk1144_dn3), (2.688117142e43 * locals.var_t0__blk1144_dn4), (2.688117142e43 * locals.var_t0__blk1144_dn5), (2.688117142e43 * locals.var_t0__blk1144_dn6), (2.688117142e43 * locals.var_t0__blk1144_dn7), (2.688117142e43 * locals.var_t0__blk1144_dn8), (2.688117142e43 * locals.var_t0__blk1144_dn9), (2.688117142e43 * locals.var_t0__blk1144_dn10), (2.688117142e43 * locals.var_t0__blk1144_dn11), (2.688117142e43 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25590_e19632;
        locals.var_t1__blk1145_dn3 = assign25590_e19632_d_n3;
        locals.var_t1__blk1145_dn4 = assign25590_e19632_d_n4;
        locals.var_t1__blk1145_dn5 = assign25590_e19632_d_n5;
        locals.var_t1__blk1145_dn6 = assign25590_e19632_d_n6;
        locals.var_t1__blk1145_dn7 = assign25590_e19632_d_n7;
        locals.var_t1__blk1145_dn8 = assign25590_e19632_d_n8;
        locals.var_t1__blk1145_dn9 = assign25590_e19632_d_n9;
        locals.var_t1__blk1145_dn10 = assign25590_e19632_d_n10;
        locals.var_t1__blk1145_dn11 = assign25590_e19632_d_n11;
        locals.var_t1__blk1145_dn12 = assign25590_e19632_d_n12;

        let assign25600_e19635: f64 = (-100.0);
        let assign25600_e19636: f64 = if locals.var_t0__blk1144 < assign25600_e19635 { 1.0 } else { 0.0 };
        locals.var_guard1623 = assign25600_e19636;

        let (assign25610_e19651, assign25610_e19651_d_n3, assign25610_e19651_d_n4, assign25610_e19651_d_n5, assign25610_e19651_d_n6, assign25610_e19651_d_n7, assign25610_e19651_d_n8, assign25610_e19651_d_n9, assign25610_e19651_d_n10, assign25610_e19651_d_n11, assign25610_e19651_d_n12,) = {
    if (((((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 == 0.0)) && (locals.var_guard1622 == 0.0)) && (locals.var_guard1623 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25610_e19651;
        locals.var_t1__blk1145_dn3 = assign25610_e19651_d_n3;
        locals.var_t1__blk1145_dn4 = assign25610_e19651_d_n4;
        locals.var_t1__blk1145_dn5 = assign25610_e19651_d_n5;
        locals.var_t1__blk1145_dn6 = assign25610_e19651_d_n6;
        locals.var_t1__blk1145_dn7 = assign25610_e19651_d_n7;
        locals.var_t1__blk1145_dn8 = assign25610_e19651_d_n8;
        locals.var_t1__blk1145_dn9 = assign25610_e19651_d_n9;
        locals.var_t1__blk1145_dn10 = assign25610_e19651_d_n10;
        locals.var_t1__blk1145_dn11 = assign25610_e19651_d_n11;
        locals.var_t1__blk1145_dn12 = assign25610_e19651_d_n12;

        let (assign25620_e19668, assign25620_e19668_d_n3, assign25620_e19668_d_n4, assign25620_e19668_d_n5, assign25620_e19668_d_n6, assign25620_e19668_d_n7, assign25620_e19668_d_n8, assign25620_e19668_d_n9, assign25620_e19668_d_n10, assign25620_e19668_d_n11, assign25620_e19668_d_n12,) = {
    if (((((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 == 0.0)) && (locals.var_guard1622 == 0.0)) && (locals.var_guard1623 == 0.0)) {
        let assign25620_e19666: f64 = (locals.var_t0__blk1144).exp();
        (assign25620_e19666, (assign25620_e19666 * locals.var_t0__blk1144_dn3), (assign25620_e19666 * locals.var_t0__blk1144_dn4), (assign25620_e19666 * locals.var_t0__blk1144_dn5), (assign25620_e19666 * locals.var_t0__blk1144_dn6), (assign25620_e19666 * locals.var_t0__blk1144_dn7), (assign25620_e19666 * locals.var_t0__blk1144_dn8), (assign25620_e19666 * locals.var_t0__blk1144_dn9), (assign25620_e19666 * locals.var_t0__blk1144_dn10), (assign25620_e19666 * locals.var_t0__blk1144_dn11), (assign25620_e19666 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25620_e19668;
        locals.var_t1__blk1145_dn3 = assign25620_e19668_d_n3;
        locals.var_t1__blk1145_dn4 = assign25620_e19668_d_n4;
        locals.var_t1__blk1145_dn5 = assign25620_e19668_d_n5;
        locals.var_t1__blk1145_dn6 = assign25620_e19668_d_n6;
        locals.var_t1__blk1145_dn7 = assign25620_e19668_d_n7;
        locals.var_t1__blk1145_dn8 = assign25620_e19668_d_n8;
        locals.var_t1__blk1145_dn9 = assign25620_e19668_d_n9;
        locals.var_t1__blk1145_dn10 = assign25620_e19668_d_n10;
        locals.var_t1__blk1145_dn11 = assign25620_e19668_d_n11;
        locals.var_t1__blk1145_dn12 = assign25620_e19668_d_n12;

        let (assign25630_e19680, assign25630_e19680_d_n3, assign25630_e19680_d_n4, assign25630_e19680_d_n5, assign25630_e19680_d_n6, assign25630_e19680_d_n7, assign25630_e19680_d_n8, assign25630_e19680_d_n9, assign25630_e19680_d_n10, assign25630_e19680_d_n11, assign25630_e19680_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 == 0.0)) {
        let assign25630_e19678: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign25630_e19678, ((locals.var_wstsi_dn3 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn3)), ((locals.var_wstsi_dn4 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn4)), ((locals.var_wstsi_dn5 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn5)), ((locals.var_wstsi_dn6 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn6)), ((locals.var_wstsi_dn7 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn7)), ((locals.var_wstsi_dn8 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn8)), ((locals.var_wstsi_dn9 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn9)), ((locals.var_wstsi_dn10 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn10)), ((locals.var_wstsi_dn11 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn11)), ((locals.var_wstsi_dn12 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn12)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign25630_e19680;
        locals.var_t3__blk1147_dn3 = assign25630_e19680_d_n3;
        locals.var_t3__blk1147_dn4 = assign25630_e19680_d_n4;
        locals.var_t3__blk1147_dn5 = assign25630_e19680_d_n5;
        locals.var_t3__blk1147_dn6 = assign25630_e19680_d_n6;
        locals.var_t3__blk1147_dn7 = assign25630_e19680_d_n7;
        locals.var_t3__blk1147_dn8 = assign25630_e19680_d_n8;
        locals.var_t3__blk1147_dn9 = assign25630_e19680_d_n9;
        locals.var_t3__blk1147_dn10 = assign25630_e19680_d_n10;
        locals.var_t3__blk1147_dn11 = assign25630_e19680_d_n11;
        locals.var_t3__blk1147_dn12 = assign25630_e19680_d_n12;

        let (assign25640_e19694, assign25640_e19694_d_n3, assign25640_e19694_d_n4, assign25640_e19694_d_n5, assign25640_e19694_d_n6, assign25640_e19694_d_n7, assign25640_e19694_d_n8, assign25640_e19694_d_n9, assign25640_e19694_d_n10, assign25640_e19694_d_n11, assign25640_e19694_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1619 == 0.0)) {
        let assign25640_e19691: f64 = (1.0 - locals.var_t1__blk1145);
        let assign25640_e19692: f64 = (locals.var_t3__blk1147 * assign25640_e19691);
        (assign25640_e19692, ((locals.var_t3__blk1147_dn3 * assign25640_e19691) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn3))), ((locals.var_t3__blk1147_dn4 * assign25640_e19691) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn4))), ((locals.var_t3__blk1147_dn5 * assign25640_e19691) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn5))), ((locals.var_t3__blk1147_dn6 * assign25640_e19691) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn6))), ((locals.var_t3__blk1147_dn7 * assign25640_e19691) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn7))), ((locals.var_t3__blk1147_dn8 * assign25640_e19691) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn8))), ((locals.var_t3__blk1147_dn9 * assign25640_e19691) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn9))), ((locals.var_t3__blk1147_dn10 * assign25640_e19691) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn10))), ((locals.var_t3__blk1147_dn11 * assign25640_e19691) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn11))), ((locals.var_t3__blk1147_dn12 * assign25640_e19691) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn12))),)
    } else {
        (locals.var_ibs4, locals.var_ibs4_dn3, locals.var_ibs4_dn4, locals.var_ibs4_dn5, locals.var_ibs4_dn6, locals.var_ibs4_dn7, locals.var_ibs4_dn8, locals.var_ibs4_dn9, locals.var_ibs4_dn10, locals.var_ibs4_dn11, locals.var_ibs4_dn12,)
    }
};
        locals.var_ibs4 = assign25640_e19694;
        locals.var_ibs4_dn3 = assign25640_e19694_d_n3;
        locals.var_ibs4_dn4 = assign25640_e19694_d_n4;
        locals.var_ibs4_dn5 = assign25640_e19694_d_n5;
        locals.var_ibs4_dn6 = assign25640_e19694_d_n6;
        locals.var_ibs4_dn7 = assign25640_e19694_d_n7;
        locals.var_ibs4_dn8 = assign25640_e19694_d_n8;
        locals.var_ibs4_dn9 = assign25640_e19694_d_n9;
        locals.var_ibs4_dn10 = assign25640_e19694_d_n10;
        locals.var_ibs4_dn11 = assign25640_e19694_d_n11;
        locals.var_ibs4_dn12 = assign25640_e19694_d_n12;

        let (assign25650_e19703, assign25650_e19703_d_n3, assign25650_e19703_d_n4, assign25650_e19703_d_n5, assign25650_e19703_d_n6, assign25650_e19703_d_n7, assign25650_e19703_d_n8, assign25650_e19703_d_n9, assign25650_e19703_d_n10, assign25650_e19703_d_n11, assign25650_e19703_d_n12,) = {
    if ((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) {
        let assign25650_e19701: f64 = (locals.var_vtm00 * locals.var_pparam_b4sointund);
        (assign25650_e19701, (locals.var_vtm00 * locals.var_pparam_b4sointund_dn3), (locals.var_vtm00 * locals.var_pparam_b4sointund_dn4), (locals.var_vtm00 * locals.var_pparam_b4sointund_dn5), (locals.var_vtm00 * locals.var_pparam_b4sointund_dn6), (locals.var_vtm00 * locals.var_pparam_b4sointund_dn7), (locals.var_vtm00 * locals.var_pparam_b4sointund_dn8), (locals.var_vtm00 * locals.var_pparam_b4sointund_dn9), (locals.var_vtm00 * locals.var_pparam_b4sointund_dn10), (locals.var_vtm00 * locals.var_pparam_b4sointund_dn11), (locals.var_vtm00 * locals.var_pparam_b4sointund_dn12),)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn3, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5, locals.var_nvtm2_dn6, locals.var_nvtm2_dn7, locals.var_nvtm2_dn8, locals.var_nvtm2_dn9, locals.var_nvtm2_dn10, locals.var_nvtm2_dn11, locals.var_nvtm2_dn12,)
    }
};
        locals.var_nvtm2 = assign25650_e19703;
        locals.var_nvtm2_dn3 = assign25650_e19703_d_n3;
        locals.var_nvtm2_dn4 = assign25650_e19703_d_n4;
        locals.var_nvtm2_dn5 = assign25650_e19703_d_n5;
        locals.var_nvtm2_dn6 = assign25650_e19703_d_n6;
        locals.var_nvtm2_dn7 = assign25650_e19703_d_n7;
        locals.var_nvtm2_dn8 = assign25650_e19703_d_n8;
        locals.var_nvtm2_dn9 = assign25650_e19703_d_n9;
        locals.var_nvtm2_dn10 = assign25650_e19703_d_n10;
        locals.var_nvtm2_dn11 = assign25650_e19703_d_n11;
        locals.var_nvtm2_dn12 = assign25650_e19703_d_n12;

        let assign25660_e19706: f64 = (locals.var_pparam_b4soivtun0d - locals.var_vdbd);
        let assign25660_e19708: f64 = if assign25660_e19706 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1624 = assign25660_e19708;

        let (assign25670_e19717, assign25670_e19717_d_n3, assign25670_e19717_d_n4, assign25670_e19717_d_n5, assign25670_e19717_d_n6, assign25670_e19717_d_n7, assign25670_e19717_d_n8, assign25670_e19717_d_n9, assign25670_e19717_d_n10, assign25670_e19717_d_n11, assign25670_e19717_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25670_e19717;
        locals.var_t1__blk1145_dn3 = assign25670_e19717_d_n3;
        locals.var_t1__blk1145_dn4 = assign25670_e19717_d_n4;
        locals.var_t1__blk1145_dn5 = assign25670_e19717_d_n5;
        locals.var_t1__blk1145_dn6 = assign25670_e19717_d_n6;
        locals.var_t1__blk1145_dn7 = assign25670_e19717_d_n7;
        locals.var_t1__blk1145_dn8 = assign25670_e19717_d_n8;
        locals.var_t1__blk1145_dn9 = assign25670_e19717_d_n9;
        locals.var_t1__blk1145_dn10 = assign25670_e19717_d_n10;
        locals.var_t1__blk1145_dn11 = assign25670_e19717_d_n11;
        locals.var_t1__blk1145_dn12 = assign25670_e19717_d_n12;

        let (assign25680_e19733, assign25680_e19733_d_n3, assign25680_e19733_d_n4, assign25680_e19733_d_n5, assign25680_e19733_d_n6, assign25680_e19733_d_n7, assign25680_e19733_d_n8, assign25680_e19733_d_n9, assign25680_e19733_d_n10, assign25680_e19733_d_n11, assign25680_e19733_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 != 0.0)) {
        let assign25680_e19725: f64 = (-locals.var_vdbd);
        let assign25680_e19727: f64 = (assign25680_e19725 / locals.var_nvtm2);
        let assign25680_e19729: f64 = (assign25680_e19727 * locals.var_pparam_b4soivtun0d);
        let assign25680_e19731: f64 = (assign25680_e19729 * locals.var_t1__blk1145);
        (assign25680_e19731, (((((-((assign25680_e19725 * locals.var_nvtm2_dn3) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25680_e19727 * locals.var_pparam_b4soivtun0d_dn3)) * locals.var_t1__blk1145) + (assign25680_e19729 * locals.var_t1__blk1145_dn3)), (((((-((assign25680_e19725 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25680_e19727 * locals.var_pparam_b4soivtun0d_dn4)) * locals.var_t1__blk1145) + (assign25680_e19729 * locals.var_t1__blk1145_dn4)), (((((-((assign25680_e19725 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25680_e19727 * locals.var_pparam_b4soivtun0d_dn5)) * locals.var_t1__blk1145) + (assign25680_e19729 * locals.var_t1__blk1145_dn5)), (((((-((assign25680_e19725 * locals.var_nvtm2_dn6) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25680_e19727 * locals.var_pparam_b4soivtun0d_dn6)) * locals.var_t1__blk1145) + (assign25680_e19729 * locals.var_t1__blk1145_dn6)), ((((((((-locals.var_vdbd_dn7) * locals.var_nvtm2) - (assign25680_e19725 * locals.var_nvtm2_dn7)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0d) + (assign25680_e19727 * locals.var_pparam_b4soivtun0d_dn7)) * locals.var_t1__blk1145) + (assign25680_e19729 * locals.var_t1__blk1145_dn7)), (((((-((assign25680_e19725 * locals.var_nvtm2_dn8) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25680_e19727 * locals.var_pparam_b4soivtun0d_dn8)) * locals.var_t1__blk1145) + (assign25680_e19729 * locals.var_t1__blk1145_dn8)), (((((-((assign25680_e19725 * locals.var_nvtm2_dn9) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25680_e19727 * locals.var_pparam_b4soivtun0d_dn9)) * locals.var_t1__blk1145) + (assign25680_e19729 * locals.var_t1__blk1145_dn9)), (((((-((assign25680_e19725 * locals.var_nvtm2_dn10) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25680_e19727 * locals.var_pparam_b4soivtun0d_dn10)) * locals.var_t1__blk1145) + (assign25680_e19729 * locals.var_t1__blk1145_dn10)), (((((-((assign25680_e19725 * locals.var_nvtm2_dn11) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25680_e19727 * locals.var_pparam_b4soivtun0d_dn11)) * locals.var_t1__blk1145) + (assign25680_e19729 * locals.var_t1__blk1145_dn11)), ((((((((-locals.var_vdbd_dn12) * locals.var_nvtm2) - (assign25680_e19725 * locals.var_nvtm2_dn12)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0d) + (assign25680_e19727 * locals.var_pparam_b4soivtun0d_dn12)) * locals.var_t1__blk1145) + (assign25680_e19729 * locals.var_t1__blk1145_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign25680_e19733;
        locals.var_t0__blk1144_dn3 = assign25680_e19733_d_n3;
        locals.var_t0__blk1144_dn4 = assign25680_e19733_d_n4;
        locals.var_t0__blk1144_dn5 = assign25680_e19733_d_n5;
        locals.var_t0__blk1144_dn6 = assign25680_e19733_d_n6;
        locals.var_t0__blk1144_dn7 = assign25680_e19733_d_n7;
        locals.var_t0__blk1144_dn8 = assign25680_e19733_d_n8;
        locals.var_t0__blk1144_dn9 = assign25680_e19733_d_n9;
        locals.var_t0__blk1144_dn10 = assign25680_e19733_d_n10;
        locals.var_t0__blk1144_dn11 = assign25680_e19733_d_n11;
        locals.var_t0__blk1144_dn12 = assign25680_e19733_d_n12;

        let assign25690_e19736: f64 = if locals.var_t0__blk1144 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1625 = assign25690_e19736;

    }

    pub(super) fn stamp_transient_block_66(
        locals: &mut StampLocals,
    ) {
        let (assign25700_e19753, assign25700_e19753_d_n3, assign25700_e19753_d_n4, assign25700_e19753_d_n5, assign25700_e19753_d_n6, assign25700_e19753_d_n7, assign25700_e19753_d_n8, assign25700_e19753_d_n9, assign25700_e19753_d_n10, assign25700_e19753_d_n11, assign25700_e19753_d_n12,) = {
    if ((((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 != 0.0)) && (locals.var_guard1625 != 0.0)) {
        let assign25700_e19748: f64 = (1.0 + locals.var_t0__blk1144);
        let assign25700_e19750: f64 = (assign25700_e19748 - 100.0);
        let assign25700_e19751: f64 = (2.688117142e43 * assign25700_e19750);
        (assign25700_e19751, (2.688117142e43 * locals.var_t0__blk1144_dn3), (2.688117142e43 * locals.var_t0__blk1144_dn4), (2.688117142e43 * locals.var_t0__blk1144_dn5), (2.688117142e43 * locals.var_t0__blk1144_dn6), (2.688117142e43 * locals.var_t0__blk1144_dn7), (2.688117142e43 * locals.var_t0__blk1144_dn8), (2.688117142e43 * locals.var_t0__blk1144_dn9), (2.688117142e43 * locals.var_t0__blk1144_dn10), (2.688117142e43 * locals.var_t0__blk1144_dn11), (2.688117142e43 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25700_e19753;
        locals.var_t1__blk1145_dn3 = assign25700_e19753_d_n3;
        locals.var_t1__blk1145_dn4 = assign25700_e19753_d_n4;
        locals.var_t1__blk1145_dn5 = assign25700_e19753_d_n5;
        locals.var_t1__blk1145_dn6 = assign25700_e19753_d_n6;
        locals.var_t1__blk1145_dn7 = assign25700_e19753_d_n7;
        locals.var_t1__blk1145_dn8 = assign25700_e19753_d_n8;
        locals.var_t1__blk1145_dn9 = assign25700_e19753_d_n9;
        locals.var_t1__blk1145_dn10 = assign25700_e19753_d_n10;
        locals.var_t1__blk1145_dn11 = assign25700_e19753_d_n11;
        locals.var_t1__blk1145_dn12 = assign25700_e19753_d_n12;

        let assign25710_e19756: f64 = (-100.0);
        let assign25710_e19757: f64 = if locals.var_t0__blk1144 < assign25710_e19756 { 1.0 } else { 0.0 };
        locals.var_guard1626 = assign25710_e19757;

        let (assign25720_e19771, assign25720_e19771_d_n3, assign25720_e19771_d_n4, assign25720_e19771_d_n5, assign25720_e19771_d_n6, assign25720_e19771_d_n7, assign25720_e19771_d_n8, assign25720_e19771_d_n9, assign25720_e19771_d_n10, assign25720_e19771_d_n11, assign25720_e19771_d_n12,) = {
    if (((((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 != 0.0)) && (locals.var_guard1625 == 0.0)) && (locals.var_guard1626 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25720_e19771;
        locals.var_t1__blk1145_dn3 = assign25720_e19771_d_n3;
        locals.var_t1__blk1145_dn4 = assign25720_e19771_d_n4;
        locals.var_t1__blk1145_dn5 = assign25720_e19771_d_n5;
        locals.var_t1__blk1145_dn6 = assign25720_e19771_d_n6;
        locals.var_t1__blk1145_dn7 = assign25720_e19771_d_n7;
        locals.var_t1__blk1145_dn8 = assign25720_e19771_d_n8;
        locals.var_t1__blk1145_dn9 = assign25720_e19771_d_n9;
        locals.var_t1__blk1145_dn10 = assign25720_e19771_d_n10;
        locals.var_t1__blk1145_dn11 = assign25720_e19771_d_n11;
        locals.var_t1__blk1145_dn12 = assign25720_e19771_d_n12;

        let (assign25730_e19787, assign25730_e19787_d_n3, assign25730_e19787_d_n4, assign25730_e19787_d_n5, assign25730_e19787_d_n6, assign25730_e19787_d_n7, assign25730_e19787_d_n8, assign25730_e19787_d_n9, assign25730_e19787_d_n10, assign25730_e19787_d_n11, assign25730_e19787_d_n12,) = {
    if (((((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 != 0.0)) && (locals.var_guard1625 == 0.0)) && (locals.var_guard1626 == 0.0)) {
        let assign25730_e19785: f64 = (locals.var_t0__blk1144).exp();
        (assign25730_e19785, (assign25730_e19785 * locals.var_t0__blk1144_dn3), (assign25730_e19785 * locals.var_t0__blk1144_dn4), (assign25730_e19785 * locals.var_t0__blk1144_dn5), (assign25730_e19785 * locals.var_t0__blk1144_dn6), (assign25730_e19785 * locals.var_t0__blk1144_dn7), (assign25730_e19785 * locals.var_t0__blk1144_dn8), (assign25730_e19785 * locals.var_t0__blk1144_dn9), (assign25730_e19785 * locals.var_t0__blk1144_dn10), (assign25730_e19785 * locals.var_t0__blk1144_dn11), (assign25730_e19785 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25730_e19787;
        locals.var_t1__blk1145_dn3 = assign25730_e19787_d_n3;
        locals.var_t1__blk1145_dn4 = assign25730_e19787_d_n4;
        locals.var_t1__blk1145_dn5 = assign25730_e19787_d_n5;
        locals.var_t1__blk1145_dn6 = assign25730_e19787_d_n6;
        locals.var_t1__blk1145_dn7 = assign25730_e19787_d_n7;
        locals.var_t1__blk1145_dn8 = assign25730_e19787_d_n8;
        locals.var_t1__blk1145_dn9 = assign25730_e19787_d_n9;
        locals.var_t1__blk1145_dn10 = assign25730_e19787_d_n10;
        locals.var_t1__blk1145_dn11 = assign25730_e19787_d_n11;
        locals.var_t1__blk1145_dn12 = assign25730_e19787_d_n12;

        let (assign25740_e19798, assign25740_e19798_d_n3, assign25740_e19798_d_n4, assign25740_e19798_d_n5, assign25740_e19798_d_n6, assign25740_e19798_d_n7, assign25740_e19798_d_n8, assign25740_e19798_d_n9, assign25740_e19798_d_n10, assign25740_e19798_d_n11, assign25740_e19798_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 != 0.0)) {
        let assign25740_e19796: f64 = (locals.var_wdtsi * locals.var_jtund);
        (assign25740_e19796, ((locals.var_wdtsi_dn3 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn3)), ((locals.var_wdtsi_dn4 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn4)), ((locals.var_wdtsi_dn5 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn5)), ((locals.var_wdtsi_dn6 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn6)), ((locals.var_wdtsi_dn7 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn7)), ((locals.var_wdtsi_dn8 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn8)), ((locals.var_wdtsi_dn9 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn9)), ((locals.var_wdtsi_dn10 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn10)), ((locals.var_wdtsi_dn11 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn11)), ((locals.var_wdtsi_dn12 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn12)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign25740_e19798;
        locals.var_t3__blk1147_dn3 = assign25740_e19798_d_n3;
        locals.var_t3__blk1147_dn4 = assign25740_e19798_d_n4;
        locals.var_t3__blk1147_dn5 = assign25740_e19798_d_n5;
        locals.var_t3__blk1147_dn6 = assign25740_e19798_d_n6;
        locals.var_t3__blk1147_dn7 = assign25740_e19798_d_n7;
        locals.var_t3__blk1147_dn8 = assign25740_e19798_d_n8;
        locals.var_t3__blk1147_dn9 = assign25740_e19798_d_n9;
        locals.var_t3__blk1147_dn10 = assign25740_e19798_d_n10;
        locals.var_t3__blk1147_dn11 = assign25740_e19798_d_n11;
        locals.var_t3__blk1147_dn12 = assign25740_e19798_d_n12;

        let (assign25750_e19811, assign25750_e19811_d_n3, assign25750_e19811_d_n4, assign25750_e19811_d_n5, assign25750_e19811_d_n6, assign25750_e19811_d_n7, assign25750_e19811_d_n8, assign25750_e19811_d_n9, assign25750_e19811_d_n10, assign25750_e19811_d_n11, assign25750_e19811_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 != 0.0)) {
        let assign25750_e19808: f64 = (1.0 - locals.var_t1__blk1145);
        let assign25750_e19809: f64 = (locals.var_t3__blk1147 * assign25750_e19808);
        (assign25750_e19809, ((locals.var_t3__blk1147_dn3 * assign25750_e19808) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn3))), ((locals.var_t3__blk1147_dn4 * assign25750_e19808) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn4))), ((locals.var_t3__blk1147_dn5 * assign25750_e19808) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn5))), ((locals.var_t3__blk1147_dn6 * assign25750_e19808) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn6))), ((locals.var_t3__blk1147_dn7 * assign25750_e19808) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn7))), ((locals.var_t3__blk1147_dn8 * assign25750_e19808) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn8))), ((locals.var_t3__blk1147_dn9 * assign25750_e19808) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn9))), ((locals.var_t3__blk1147_dn10 * assign25750_e19808) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn10))), ((locals.var_t3__blk1147_dn11 * assign25750_e19808) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn11))), ((locals.var_t3__blk1147_dn12 * assign25750_e19808) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn12))),)
    } else {
        (locals.var_ibd4, locals.var_ibd4_dn3, locals.var_ibd4_dn4, locals.var_ibd4_dn5, locals.var_ibd4_dn6, locals.var_ibd4_dn7, locals.var_ibd4_dn8, locals.var_ibd4_dn9, locals.var_ibd4_dn10, locals.var_ibd4_dn11, locals.var_ibd4_dn12,)
    }
};
        locals.var_ibd4 = assign25750_e19811;
        locals.var_ibd4_dn3 = assign25750_e19811_d_n3;
        locals.var_ibd4_dn4 = assign25750_e19811_d_n4;
        locals.var_ibd4_dn5 = assign25750_e19811_d_n5;
        locals.var_ibd4_dn6 = assign25750_e19811_d_n6;
        locals.var_ibd4_dn7 = assign25750_e19811_d_n7;
        locals.var_ibd4_dn8 = assign25750_e19811_d_n8;
        locals.var_ibd4_dn9 = assign25750_e19811_d_n9;
        locals.var_ibd4_dn10 = assign25750_e19811_d_n10;
        locals.var_ibd4_dn11 = assign25750_e19811_d_n11;
        locals.var_ibd4_dn12 = assign25750_e19811_d_n12;

        let (assign25760_e19825, assign25760_e19825_d_n3, assign25760_e19825_d_n4, assign25760_e19825_d_n5, assign25760_e19825_d_n6, assign25760_e19825_d_n7, assign25760_e19825_d_n8, assign25760_e19825_d_n9, assign25760_e19825_d_n10, assign25760_e19825_d_n11, assign25760_e19825_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 == 0.0)) {
        let assign25760_e19822: f64 = (locals.var_pparam_b4soivtun0d - locals.var_vdbd);
        let assign25760_e19823: f64 = (1.0 / assign25760_e19822);
        (assign25760_e19823, (-(locals.var_pparam_b4soivtun0d_dn3 / (assign25760_e19822 * assign25760_e19822))), (-(locals.var_pparam_b4soivtun0d_dn4 / (assign25760_e19822 * assign25760_e19822))), (-(locals.var_pparam_b4soivtun0d_dn5 / (assign25760_e19822 * assign25760_e19822))), (-(locals.var_pparam_b4soivtun0d_dn6 / (assign25760_e19822 * assign25760_e19822))), (-((locals.var_pparam_b4soivtun0d_dn7 - locals.var_vdbd_dn7) / (assign25760_e19822 * assign25760_e19822))), (-(locals.var_pparam_b4soivtun0d_dn8 / (assign25760_e19822 * assign25760_e19822))), (-(locals.var_pparam_b4soivtun0d_dn9 / (assign25760_e19822 * assign25760_e19822))), (-(locals.var_pparam_b4soivtun0d_dn10 / (assign25760_e19822 * assign25760_e19822))), (-(locals.var_pparam_b4soivtun0d_dn11 / (assign25760_e19822 * assign25760_e19822))), (-((locals.var_pparam_b4soivtun0d_dn12 - locals.var_vdbd_dn12) / (assign25760_e19822 * assign25760_e19822))),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25760_e19825;
        locals.var_t1__blk1145_dn3 = assign25760_e19825_d_n3;
        locals.var_t1__blk1145_dn4 = assign25760_e19825_d_n4;
        locals.var_t1__blk1145_dn5 = assign25760_e19825_d_n5;
        locals.var_t1__blk1145_dn6 = assign25760_e19825_d_n6;
        locals.var_t1__blk1145_dn7 = assign25760_e19825_d_n7;
        locals.var_t1__blk1145_dn8 = assign25760_e19825_d_n8;
        locals.var_t1__blk1145_dn9 = assign25760_e19825_d_n9;
        locals.var_t1__blk1145_dn10 = assign25760_e19825_d_n10;
        locals.var_t1__blk1145_dn11 = assign25760_e19825_d_n11;
        locals.var_t1__blk1145_dn12 = assign25760_e19825_d_n12;

        let (assign25770_e19842, assign25770_e19842_d_n3, assign25770_e19842_d_n4, assign25770_e19842_d_n5, assign25770_e19842_d_n6, assign25770_e19842_d_n7, assign25770_e19842_d_n8, assign25770_e19842_d_n9, assign25770_e19842_d_n10, assign25770_e19842_d_n11, assign25770_e19842_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 == 0.0)) {
        let assign25770_e19834: f64 = (-locals.var_vdbd);
        let assign25770_e19836: f64 = (assign25770_e19834 / locals.var_nvtm2);
        let assign25770_e19838: f64 = (assign25770_e19836 * locals.var_pparam_b4soivtun0d);
        let assign25770_e19840: f64 = (assign25770_e19838 * locals.var_t1__blk1145);
        (assign25770_e19840, (((((-((assign25770_e19834 * locals.var_nvtm2_dn3) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25770_e19836 * locals.var_pparam_b4soivtun0d_dn3)) * locals.var_t1__blk1145) + (assign25770_e19838 * locals.var_t1__blk1145_dn3)), (((((-((assign25770_e19834 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25770_e19836 * locals.var_pparam_b4soivtun0d_dn4)) * locals.var_t1__blk1145) + (assign25770_e19838 * locals.var_t1__blk1145_dn4)), (((((-((assign25770_e19834 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25770_e19836 * locals.var_pparam_b4soivtun0d_dn5)) * locals.var_t1__blk1145) + (assign25770_e19838 * locals.var_t1__blk1145_dn5)), (((((-((assign25770_e19834 * locals.var_nvtm2_dn6) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25770_e19836 * locals.var_pparam_b4soivtun0d_dn6)) * locals.var_t1__blk1145) + (assign25770_e19838 * locals.var_t1__blk1145_dn6)), ((((((((-locals.var_vdbd_dn7) * locals.var_nvtm2) - (assign25770_e19834 * locals.var_nvtm2_dn7)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0d) + (assign25770_e19836 * locals.var_pparam_b4soivtun0d_dn7)) * locals.var_t1__blk1145) + (assign25770_e19838 * locals.var_t1__blk1145_dn7)), (((((-((assign25770_e19834 * locals.var_nvtm2_dn8) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25770_e19836 * locals.var_pparam_b4soivtun0d_dn8)) * locals.var_t1__blk1145) + (assign25770_e19838 * locals.var_t1__blk1145_dn8)), (((((-((assign25770_e19834 * locals.var_nvtm2_dn9) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25770_e19836 * locals.var_pparam_b4soivtun0d_dn9)) * locals.var_t1__blk1145) + (assign25770_e19838 * locals.var_t1__blk1145_dn9)), (((((-((assign25770_e19834 * locals.var_nvtm2_dn10) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25770_e19836 * locals.var_pparam_b4soivtun0d_dn10)) * locals.var_t1__blk1145) + (assign25770_e19838 * locals.var_t1__blk1145_dn10)), (((((-((assign25770_e19834 * locals.var_nvtm2_dn11) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign25770_e19836 * locals.var_pparam_b4soivtun0d_dn11)) * locals.var_t1__blk1145) + (assign25770_e19838 * locals.var_t1__blk1145_dn11)), ((((((((-locals.var_vdbd_dn12) * locals.var_nvtm2) - (assign25770_e19834 * locals.var_nvtm2_dn12)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0d) + (assign25770_e19836 * locals.var_pparam_b4soivtun0d_dn12)) * locals.var_t1__blk1145) + (assign25770_e19838 * locals.var_t1__blk1145_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign25770_e19842;
        locals.var_t0__blk1144_dn3 = assign25770_e19842_d_n3;
        locals.var_t0__blk1144_dn4 = assign25770_e19842_d_n4;
        locals.var_t0__blk1144_dn5 = assign25770_e19842_d_n5;
        locals.var_t0__blk1144_dn6 = assign25770_e19842_d_n6;
        locals.var_t0__blk1144_dn7 = assign25770_e19842_d_n7;
        locals.var_t0__blk1144_dn8 = assign25770_e19842_d_n8;
        locals.var_t0__blk1144_dn9 = assign25770_e19842_d_n9;
        locals.var_t0__blk1144_dn10 = assign25770_e19842_d_n10;
        locals.var_t0__blk1144_dn11 = assign25770_e19842_d_n11;
        locals.var_t0__blk1144_dn12 = assign25770_e19842_d_n12;

        let assign25780_e19845: f64 = if locals.var_t0__blk1144 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1627 = assign25780_e19845;

        let (assign25790_e19863, assign25790_e19863_d_n3, assign25790_e19863_d_n4, assign25790_e19863_d_n5, assign25790_e19863_d_n6, assign25790_e19863_d_n7, assign25790_e19863_d_n8, assign25790_e19863_d_n9, assign25790_e19863_d_n10, assign25790_e19863_d_n11, assign25790_e19863_d_n12,) = {
    if ((((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 == 0.0)) && (locals.var_guard1627 != 0.0)) {
        let assign25790_e19858: f64 = (1.0 + locals.var_t0__blk1144);
        let assign25790_e19860: f64 = (assign25790_e19858 - 100.0);
        let assign25790_e19861: f64 = (2.688117142e43 * assign25790_e19860);
        (assign25790_e19861, (2.688117142e43 * locals.var_t0__blk1144_dn3), (2.688117142e43 * locals.var_t0__blk1144_dn4), (2.688117142e43 * locals.var_t0__blk1144_dn5), (2.688117142e43 * locals.var_t0__blk1144_dn6), (2.688117142e43 * locals.var_t0__blk1144_dn7), (2.688117142e43 * locals.var_t0__blk1144_dn8), (2.688117142e43 * locals.var_t0__blk1144_dn9), (2.688117142e43 * locals.var_t0__blk1144_dn10), (2.688117142e43 * locals.var_t0__blk1144_dn11), (2.688117142e43 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25790_e19863;
        locals.var_t1__blk1145_dn3 = assign25790_e19863_d_n3;
        locals.var_t1__blk1145_dn4 = assign25790_e19863_d_n4;
        locals.var_t1__blk1145_dn5 = assign25790_e19863_d_n5;
        locals.var_t1__blk1145_dn6 = assign25790_e19863_d_n6;
        locals.var_t1__blk1145_dn7 = assign25790_e19863_d_n7;
        locals.var_t1__blk1145_dn8 = assign25790_e19863_d_n8;
        locals.var_t1__blk1145_dn9 = assign25790_e19863_d_n9;
        locals.var_t1__blk1145_dn10 = assign25790_e19863_d_n10;
        locals.var_t1__blk1145_dn11 = assign25790_e19863_d_n11;
        locals.var_t1__blk1145_dn12 = assign25790_e19863_d_n12;

        let assign25800_e19866: f64 = (-100.0);
        let assign25800_e19867: f64 = if locals.var_t0__blk1144 < assign25800_e19866 { 1.0 } else { 0.0 };
        locals.var_guard1628 = assign25800_e19867;

        let (assign25810_e19882, assign25810_e19882_d_n3, assign25810_e19882_d_n4, assign25810_e19882_d_n5, assign25810_e19882_d_n6, assign25810_e19882_d_n7, assign25810_e19882_d_n8, assign25810_e19882_d_n9, assign25810_e19882_d_n10, assign25810_e19882_d_n11, assign25810_e19882_d_n12,) = {
    if (((((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 == 0.0)) && (locals.var_guard1627 == 0.0)) && (locals.var_guard1628 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25810_e19882;
        locals.var_t1__blk1145_dn3 = assign25810_e19882_d_n3;
        locals.var_t1__blk1145_dn4 = assign25810_e19882_d_n4;
        locals.var_t1__blk1145_dn5 = assign25810_e19882_d_n5;
        locals.var_t1__blk1145_dn6 = assign25810_e19882_d_n6;
        locals.var_t1__blk1145_dn7 = assign25810_e19882_d_n7;
        locals.var_t1__blk1145_dn8 = assign25810_e19882_d_n8;
        locals.var_t1__blk1145_dn9 = assign25810_e19882_d_n9;
        locals.var_t1__blk1145_dn10 = assign25810_e19882_d_n10;
        locals.var_t1__blk1145_dn11 = assign25810_e19882_d_n11;
        locals.var_t1__blk1145_dn12 = assign25810_e19882_d_n12;

        let (assign25820_e19899, assign25820_e19899_d_n3, assign25820_e19899_d_n4, assign25820_e19899_d_n5, assign25820_e19899_d_n6, assign25820_e19899_d_n7, assign25820_e19899_d_n8, assign25820_e19899_d_n9, assign25820_e19899_d_n10, assign25820_e19899_d_n11, assign25820_e19899_d_n12,) = {
    if (((((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 == 0.0)) && (locals.var_guard1627 == 0.0)) && (locals.var_guard1628 == 0.0)) {
        let assign25820_e19897: f64 = (locals.var_t0__blk1144).exp();
        (assign25820_e19897, (assign25820_e19897 * locals.var_t0__blk1144_dn3), (assign25820_e19897 * locals.var_t0__blk1144_dn4), (assign25820_e19897 * locals.var_t0__blk1144_dn5), (assign25820_e19897 * locals.var_t0__blk1144_dn6), (assign25820_e19897 * locals.var_t0__blk1144_dn7), (assign25820_e19897 * locals.var_t0__blk1144_dn8), (assign25820_e19897 * locals.var_t0__blk1144_dn9), (assign25820_e19897 * locals.var_t0__blk1144_dn10), (assign25820_e19897 * locals.var_t0__blk1144_dn11), (assign25820_e19897 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign25820_e19899;
        locals.var_t1__blk1145_dn3 = assign25820_e19899_d_n3;
        locals.var_t1__blk1145_dn4 = assign25820_e19899_d_n4;
        locals.var_t1__blk1145_dn5 = assign25820_e19899_d_n5;
        locals.var_t1__blk1145_dn6 = assign25820_e19899_d_n6;
        locals.var_t1__blk1145_dn7 = assign25820_e19899_d_n7;
        locals.var_t1__blk1145_dn8 = assign25820_e19899_d_n8;
        locals.var_t1__blk1145_dn9 = assign25820_e19899_d_n9;
        locals.var_t1__blk1145_dn10 = assign25820_e19899_d_n10;
        locals.var_t1__blk1145_dn11 = assign25820_e19899_d_n11;
        locals.var_t1__blk1145_dn12 = assign25820_e19899_d_n12;

        let (assign25830_e19911, assign25830_e19911_d_n3, assign25830_e19911_d_n4, assign25830_e19911_d_n5, assign25830_e19911_d_n6, assign25830_e19911_d_n7, assign25830_e19911_d_n8, assign25830_e19911_d_n9, assign25830_e19911_d_n10, assign25830_e19911_d_n11, assign25830_e19911_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 == 0.0)) {
        let assign25830_e19909: f64 = (locals.var_wdtsi * locals.var_jtund);
        (assign25830_e19909, ((locals.var_wdtsi_dn3 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn3)), ((locals.var_wdtsi_dn4 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn4)), ((locals.var_wdtsi_dn5 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn5)), ((locals.var_wdtsi_dn6 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn6)), ((locals.var_wdtsi_dn7 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn7)), ((locals.var_wdtsi_dn8 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn8)), ((locals.var_wdtsi_dn9 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn9)), ((locals.var_wdtsi_dn10 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn10)), ((locals.var_wdtsi_dn11 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn11)), ((locals.var_wdtsi_dn12 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn12)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign25830_e19911;
        locals.var_t3__blk1147_dn3 = assign25830_e19911_d_n3;
        locals.var_t3__blk1147_dn4 = assign25830_e19911_d_n4;
        locals.var_t3__blk1147_dn5 = assign25830_e19911_d_n5;
        locals.var_t3__blk1147_dn6 = assign25830_e19911_d_n6;
        locals.var_t3__blk1147_dn7 = assign25830_e19911_d_n7;
        locals.var_t3__blk1147_dn8 = assign25830_e19911_d_n8;
        locals.var_t3__blk1147_dn9 = assign25830_e19911_d_n9;
        locals.var_t3__blk1147_dn10 = assign25830_e19911_d_n10;
        locals.var_t3__blk1147_dn11 = assign25830_e19911_d_n11;
        locals.var_t3__blk1147_dn12 = assign25830_e19911_d_n12;

        let (assign25840_e19925, assign25840_e19925_d_n3, assign25840_e19925_d_n4, assign25840_e19925_d_n5, assign25840_e19925_d_n6, assign25840_e19925_d_n7, assign25840_e19925_d_n8, assign25840_e19925_d_n9, assign25840_e19925_d_n10, assign25840_e19925_d_n11, assign25840_e19925_d_n12,) = {
    if (((locals.var_guard1578 != 0.0) && (locals.var_guard1618 == 0.0)) && (locals.var_guard1624 == 0.0)) {
        let assign25840_e19922: f64 = (1.0 - locals.var_t1__blk1145);
        let assign25840_e19923: f64 = (locals.var_t3__blk1147 * assign25840_e19922);
        (assign25840_e19923, ((locals.var_t3__blk1147_dn3 * assign25840_e19922) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn3))), ((locals.var_t3__blk1147_dn4 * assign25840_e19922) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn4))), ((locals.var_t3__blk1147_dn5 * assign25840_e19922) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn5))), ((locals.var_t3__blk1147_dn6 * assign25840_e19922) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn6))), ((locals.var_t3__blk1147_dn7 * assign25840_e19922) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn7))), ((locals.var_t3__blk1147_dn8 * assign25840_e19922) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn8))), ((locals.var_t3__blk1147_dn9 * assign25840_e19922) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn9))), ((locals.var_t3__blk1147_dn10 * assign25840_e19922) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn10))), ((locals.var_t3__blk1147_dn11 * assign25840_e19922) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn11))), ((locals.var_t3__blk1147_dn12 * assign25840_e19922) + (locals.var_t3__blk1147 * (-locals.var_t1__blk1145_dn12))),)
    } else {
        (locals.var_ibd4, locals.var_ibd4_dn3, locals.var_ibd4_dn4, locals.var_ibd4_dn5, locals.var_ibd4_dn6, locals.var_ibd4_dn7, locals.var_ibd4_dn8, locals.var_ibd4_dn9, locals.var_ibd4_dn10, locals.var_ibd4_dn11, locals.var_ibd4_dn12,)
    }
};
        locals.var_ibd4 = assign25840_e19925;
        locals.var_ibd4_dn3 = assign25840_e19925_d_n3;
        locals.var_ibd4_dn4 = assign25840_e19925_d_n4;
        locals.var_ibd4_dn5 = assign25840_e19925_d_n5;
        locals.var_ibd4_dn6 = assign25840_e19925_d_n6;
        locals.var_ibd4_dn7 = assign25840_e19925_d_n7;
        locals.var_ibd4_dn8 = assign25840_e19925_d_n8;
        locals.var_ibd4_dn9 = assign25840_e19925_d_n9;
        locals.var_ibd4_dn10 = assign25840_e19925_d_n10;
        locals.var_ibd4_dn11 = assign25840_e19925_d_n11;
        locals.var_ibd4_dn12 = assign25840_e19925_d_n12;

        let (assign25850_e19935, assign25850_e19935_d_n3, assign25850_e19935_d_n4, assign25850_e19935_d_n5, assign25850_e19935_d_n6, assign25850_e19935_d_n7, assign25850_e19935_d_n8, assign25850_e19935_d_n9, assign25850_e19935_d_n10, assign25850_e19935_d_n11, assign25850_e19935_d_n12,) = {
    if (locals.var_guard1578 != 0.0) {
        let assign25850_e19929: f64 = (locals.var_ibs1 + locals.var_ibs2);
        let assign25850_e19931: f64 = (assign25850_e19929 + locals.var_ibs3);
        let assign25850_e19933: f64 = (assign25850_e19931 + locals.var_ibs4);
        (assign25850_e19933, (((locals.var_ibs1_dn3 + locals.var_ibs2_dn3) + locals.var_ibs3_dn3) + locals.var_ibs4_dn3), (((locals.var_ibs1_dn4 + locals.var_ibs2_dn4) + locals.var_ibs3_dn4) + locals.var_ibs4_dn4), (((locals.var_ibs1_dn5 + locals.var_ibs2_dn5) + locals.var_ibs3_dn5) + locals.var_ibs4_dn5), (((locals.var_ibs1_dn6 + locals.var_ibs2_dn6) + locals.var_ibs3_dn6) + locals.var_ibs4_dn6), (((locals.var_ibs1_dn7 + locals.var_ibs2_dn7) + locals.var_ibs3_dn7) + locals.var_ibs4_dn7), (((locals.var_ibs1_dn8 + locals.var_ibs2_dn8) + locals.var_ibs3_dn8) + locals.var_ibs4_dn8), (((locals.var_ibs1_dn9 + locals.var_ibs2_dn9) + locals.var_ibs3_dn9) + locals.var_ibs4_dn9), (((locals.var_ibs1_dn10 + locals.var_ibs2_dn10) + locals.var_ibs3_dn10) + locals.var_ibs4_dn10), (((locals.var_ibs1_dn11 + locals.var_ibs2_dn11) + locals.var_ibs3_dn11) + locals.var_ibs4_dn11), (((locals.var_ibs1_dn12 + locals.var_ibs2_dn12) + locals.var_ibs3_dn12) + locals.var_ibs4_dn12),)
    } else {
        (locals.var_ibs_1, locals.var_ibs_1_dn3, locals.var_ibs_1_dn4, locals.var_ibs_1_dn5, locals.var_ibs_1_dn6, locals.var_ibs_1_dn7, locals.var_ibs_1_dn8, locals.var_ibs_1_dn9, locals.var_ibs_1_dn10, locals.var_ibs_1_dn11, locals.var_ibs_1_dn12,)
    }
};
        locals.var_ibs_1 = assign25850_e19935;
        locals.var_ibs_1_dn3 = assign25850_e19935_d_n3;
        locals.var_ibs_1_dn4 = assign25850_e19935_d_n4;
        locals.var_ibs_1_dn5 = assign25850_e19935_d_n5;
        locals.var_ibs_1_dn6 = assign25850_e19935_d_n6;
        locals.var_ibs_1_dn7 = assign25850_e19935_d_n7;
        locals.var_ibs_1_dn8 = assign25850_e19935_d_n8;
        locals.var_ibs_1_dn9 = assign25850_e19935_d_n9;
        locals.var_ibs_1_dn10 = assign25850_e19935_d_n10;
        locals.var_ibs_1_dn11 = assign25850_e19935_d_n11;
        locals.var_ibs_1_dn12 = assign25850_e19935_d_n12;

        let (assign25860_e19945, assign25860_e19945_d_n3, assign25860_e19945_d_n4, assign25860_e19945_d_n5, assign25860_e19945_d_n6, assign25860_e19945_d_n7, assign25860_e19945_d_n8, assign25860_e19945_d_n9, assign25860_e19945_d_n10, assign25860_e19945_d_n11, assign25860_e19945_d_n12,) = {
    if (locals.var_guard1578 != 0.0) {
        let assign25860_e19939: f64 = (locals.var_ibd1 + locals.var_ibd2);
        let assign25860_e19941: f64 = (assign25860_e19939 + locals.var_ibd3);
        let assign25860_e19943: f64 = (assign25860_e19941 + locals.var_ibd4);
        (assign25860_e19943, (((locals.var_ibd1_dn3 + locals.var_ibd2_dn3) + locals.var_ibd3_dn3) + locals.var_ibd4_dn3), (((locals.var_ibd1_dn4 + locals.var_ibd2_dn4) + locals.var_ibd3_dn4) + locals.var_ibd4_dn4), (((locals.var_ibd1_dn5 + locals.var_ibd2_dn5) + locals.var_ibd3_dn5) + locals.var_ibd4_dn5), (((locals.var_ibd1_dn6 + locals.var_ibd2_dn6) + locals.var_ibd3_dn6) + locals.var_ibd4_dn6), (((locals.var_ibd1_dn7 + locals.var_ibd2_dn7) + locals.var_ibd3_dn7) + locals.var_ibd4_dn7), (((locals.var_ibd1_dn8 + locals.var_ibd2_dn8) + locals.var_ibd3_dn8) + locals.var_ibd4_dn8), (((locals.var_ibd1_dn9 + locals.var_ibd2_dn9) + locals.var_ibd3_dn9) + locals.var_ibd4_dn9), (((locals.var_ibd1_dn10 + locals.var_ibd2_dn10) + locals.var_ibd3_dn10) + locals.var_ibd4_dn10), (((locals.var_ibd1_dn11 + locals.var_ibd2_dn11) + locals.var_ibd3_dn11) + locals.var_ibd4_dn11), (((locals.var_ibd1_dn12 + locals.var_ibd2_dn12) + locals.var_ibd3_dn12) + locals.var_ibd4_dn12),)
    } else {
        (locals.var_ibd_1, locals.var_ibd_1_dn3, locals.var_ibd_1_dn4, locals.var_ibd_1_dn5, locals.var_ibd_1_dn6, locals.var_ibd_1_dn7, locals.var_ibd_1_dn8, locals.var_ibd_1_dn9, locals.var_ibd_1_dn10, locals.var_ibd_1_dn11, locals.var_ibd_1_dn12,)
    }
};
        locals.var_ibd_1 = assign25860_e19945;
        locals.var_ibd_1_dn3 = assign25860_e19945_d_n3;
        locals.var_ibd_1_dn4 = assign25860_e19945_d_n4;
        locals.var_ibd_1_dn5 = assign25860_e19945_d_n5;
        locals.var_ibd_1_dn6 = assign25860_e19945_d_n6;
        locals.var_ibd_1_dn7 = assign25860_e19945_d_n7;
        locals.var_ibd_1_dn8 = assign25860_e19945_d_n8;
        locals.var_ibd_1_dn9 = assign25860_e19945_d_n9;
        locals.var_ibd_1_dn10 = assign25860_e19945_d_n10;
        locals.var_ibd_1_dn11 = assign25860_e19945_d_n11;
        locals.var_ibd_1_dn12 = assign25860_e19945_d_n12;

        let (assign25870_e19950, assign25870_e19950_d_n3, assign25870_e19950_d_n4, assign25870_e19950_d_n5, assign25870_e19950_d_n6, assign25870_e19950_d_n7, assign25870_e19950_d_n8, assign25870_e19950_d_n9, assign25870_e19950_d_n10, assign25870_e19950_d_n11, assign25870_e19950_d_n12,) = {
    if (locals.var_guard1578 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl_1, locals.var_igidl_1_dn3, locals.var_igidl_1_dn4, locals.var_igidl_1_dn5, locals.var_igidl_1_dn6, locals.var_igidl_1_dn7, locals.var_igidl_1_dn8, locals.var_igidl_1_dn9, locals.var_igidl_1_dn10, locals.var_igidl_1_dn11, locals.var_igidl_1_dn12,)
    }
};
        locals.var_igidl_1 = assign25870_e19950;
        locals.var_igidl_1_dn3 = assign25870_e19950_d_n3;
        locals.var_igidl_1_dn4 = assign25870_e19950_d_n4;
        locals.var_igidl_1_dn5 = assign25870_e19950_d_n5;
        locals.var_igidl_1_dn6 = assign25870_e19950_d_n6;
        locals.var_igidl_1_dn7 = assign25870_e19950_d_n7;
        locals.var_igidl_1_dn8 = assign25870_e19950_d_n8;
        locals.var_igidl_1_dn9 = assign25870_e19950_d_n9;
        locals.var_igidl_1_dn10 = assign25870_e19950_d_n10;
        locals.var_igidl_1_dn11 = assign25870_e19950_d_n11;
        locals.var_igidl_1_dn12 = assign25870_e19950_d_n12;

        let (assign25880_e19955, assign25880_e19955_d_n3, assign25880_e19955_d_n4, assign25880_e19955_d_n5, assign25880_e19955_d_n6, assign25880_e19955_d_n7, assign25880_e19955_d_n8, assign25880_e19955_d_n9, assign25880_e19955_d_n10, assign25880_e19955_d_n11, assign25880_e19955_d_n12,) = {
    if (locals.var_guard1578 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl_1, locals.var_igisl_1_dn3, locals.var_igisl_1_dn4, locals.var_igisl_1_dn5, locals.var_igisl_1_dn6, locals.var_igisl_1_dn7, locals.var_igisl_1_dn8, locals.var_igisl_1_dn9, locals.var_igisl_1_dn10, locals.var_igisl_1_dn11, locals.var_igisl_1_dn12,)
    }
};
        locals.var_igisl_1 = assign25880_e19955;
        locals.var_igisl_1_dn3 = assign25880_e19955_d_n3;
        locals.var_igisl_1_dn4 = assign25880_e19955_d_n4;
        locals.var_igisl_1_dn5 = assign25880_e19955_d_n5;
        locals.var_igisl_1_dn6 = assign25880_e19955_d_n6;
        locals.var_igisl_1_dn7 = assign25880_e19955_d_n7;
        locals.var_igisl_1_dn8 = assign25880_e19955_d_n8;
        locals.var_igisl_1_dn9 = assign25880_e19955_d_n9;
        locals.var_igisl_1_dn10 = assign25880_e19955_d_n10;
        locals.var_igisl_1_dn11 = assign25880_e19955_d_n11;
        locals.var_igisl_1_dn12 = assign25880_e19955_d_n12;

        let (assign25890_e19960, assign25890_e19960_d_n3, assign25890_e19960_d_n4, assign25890_e19960_d_n5, assign25890_e19960_d_n6, assign25890_e19960_d_n7, assign25890_e19960_d_n8, assign25890_e19960_d_n9, assign25890_e19960_d_n10, assign25890_e19960_d_n11, assign25890_e19960_d_n12,) = {
    if (locals.var_guard1578 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_1, locals.var_ibs_1_dn3, locals.var_ibs_1_dn4, locals.var_ibs_1_dn5, locals.var_ibs_1_dn6, locals.var_ibs_1_dn7, locals.var_ibs_1_dn8, locals.var_ibs_1_dn9, locals.var_ibs_1_dn10, locals.var_ibs_1_dn11, locals.var_ibs_1_dn12,)
    }
};
        locals.var_ibs_1 = assign25890_e19960;
        locals.var_ibs_1_dn3 = assign25890_e19960_d_n3;
        locals.var_ibs_1_dn4 = assign25890_e19960_d_n4;
        locals.var_ibs_1_dn5 = assign25890_e19960_d_n5;
        locals.var_ibs_1_dn6 = assign25890_e19960_d_n6;
        locals.var_ibs_1_dn7 = assign25890_e19960_d_n7;
        locals.var_ibs_1_dn8 = assign25890_e19960_d_n8;
        locals.var_ibs_1_dn9 = assign25890_e19960_d_n9;
        locals.var_ibs_1_dn10 = assign25890_e19960_d_n10;
        locals.var_ibs_1_dn11 = assign25890_e19960_d_n11;
        locals.var_ibs_1_dn12 = assign25890_e19960_d_n12;

        let (assign25900_e19965, assign25900_e19965_d_n3, assign25900_e19965_d_n4, assign25900_e19965_d_n5, assign25900_e19965_d_n6, assign25900_e19965_d_n7, assign25900_e19965_d_n8, assign25900_e19965_d_n9, assign25900_e19965_d_n10, assign25900_e19965_d_n11, assign25900_e19965_d_n12,) = {
    if (locals.var_guard1578 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd_1, locals.var_ibd_1_dn3, locals.var_ibd_1_dn4, locals.var_ibd_1_dn5, locals.var_ibd_1_dn6, locals.var_ibd_1_dn7, locals.var_ibd_1_dn8, locals.var_ibd_1_dn9, locals.var_ibd_1_dn10, locals.var_ibd_1_dn11, locals.var_ibd_1_dn12,)
    }
};
        locals.var_ibd_1 = assign25900_e19965;
        locals.var_ibd_1_dn3 = assign25900_e19965_d_n3;
        locals.var_ibd_1_dn4 = assign25900_e19965_d_n4;
        locals.var_ibd_1_dn5 = assign25900_e19965_d_n5;
        locals.var_ibd_1_dn6 = assign25900_e19965_d_n6;
        locals.var_ibd_1_dn7 = assign25900_e19965_d_n7;
        locals.var_ibd_1_dn8 = assign25900_e19965_d_n8;
        locals.var_ibd_1_dn9 = assign25900_e19965_d_n9;
        locals.var_ibd_1_dn10 = assign25900_e19965_d_n10;
        locals.var_ibd_1_dn11 = assign25900_e19965_d_n11;
        locals.var_ibd_1_dn12 = assign25900_e19965_d_n12;

        let (assign25910_e19970, assign25910_e19970_d_n3, assign25910_e19970_d_n4, assign25910_e19970_d_n5, assign25910_e19970_d_n6, assign25910_e19970_d_n7, assign25910_e19970_d_n8, assign25910_e19970_d_n9, assign25910_e19970_d_n10, assign25910_e19970_d_n11, assign25910_e19970_d_n12,) = {
    if (locals.var_guard1578 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibsdif, locals.var_ibsdif_dn3, locals.var_ibsdif_dn4, locals.var_ibsdif_dn5, locals.var_ibsdif_dn6, locals.var_ibsdif_dn7, locals.var_ibsdif_dn8, locals.var_ibsdif_dn9, locals.var_ibsdif_dn10, locals.var_ibsdif_dn11, locals.var_ibsdif_dn12,)
    }
};
        locals.var_ibsdif = assign25910_e19970;
        locals.var_ibsdif_dn3 = assign25910_e19970_d_n3;
        locals.var_ibsdif_dn4 = assign25910_e19970_d_n4;
        locals.var_ibsdif_dn5 = assign25910_e19970_d_n5;
        locals.var_ibsdif_dn6 = assign25910_e19970_d_n6;
        locals.var_ibsdif_dn7 = assign25910_e19970_d_n7;
        locals.var_ibsdif_dn8 = assign25910_e19970_d_n8;
        locals.var_ibsdif_dn9 = assign25910_e19970_d_n9;
        locals.var_ibsdif_dn10 = assign25910_e19970_d_n10;
        locals.var_ibsdif_dn11 = assign25910_e19970_d_n11;
        locals.var_ibsdif_dn12 = assign25910_e19970_d_n12;

        let (assign25920_e19975, assign25920_e19975_d_n3, assign25920_e19975_d_n4, assign25920_e19975_d_n5, assign25920_e19975_d_n6, assign25920_e19975_d_n7, assign25920_e19975_d_n8, assign25920_e19975_d_n9, assign25920_e19975_d_n10, assign25920_e19975_d_n11, assign25920_e19975_d_n12,) = {
    if (locals.var_guard1578 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibddif, locals.var_ibddif_dn3, locals.var_ibddif_dn4, locals.var_ibddif_dn5, locals.var_ibddif_dn6, locals.var_ibddif_dn7, locals.var_ibddif_dn8, locals.var_ibddif_dn9, locals.var_ibddif_dn10, locals.var_ibddif_dn11, locals.var_ibddif_dn12,)
    }
};
        locals.var_ibddif = assign25920_e19975;
        locals.var_ibddif_dn3 = assign25920_e19975_d_n3;
        locals.var_ibddif_dn4 = assign25920_e19975_d_n4;
        locals.var_ibddif_dn5 = assign25920_e19975_d_n5;
        locals.var_ibddif_dn6 = assign25920_e19975_d_n6;
        locals.var_ibddif_dn7 = assign25920_e19975_d_n7;
        locals.var_ibddif_dn8 = assign25920_e19975_d_n8;
        locals.var_ibddif_dn9 = assign25920_e19975_d_n9;
        locals.var_ibddif_dn10 = assign25920_e19975_d_n10;
        locals.var_ibddif_dn11 = assign25920_e19975_d_n11;
        locals.var_ibddif_dn12 = assign25920_e19975_d_n12;

        let (assign25930_e19980, assign25930_e19980_d_n3, assign25930_e19980_d_n4, assign25930_e19980_d_n5, assign25930_e19980_d_n6, assign25930_e19980_d_n7, assign25930_e19980_d_n8, assign25930_e19980_d_n9, assign25930_e19980_d_n10, assign25930_e19980_d_n11, assign25930_e19980_d_n12,) = {
    if (locals.var_guard1578 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ic_1, locals.var_ic_1_dn3, locals.var_ic_1_dn4, locals.var_ic_1_dn5, locals.var_ic_1_dn6, locals.var_ic_1_dn7, locals.var_ic_1_dn8, locals.var_ic_1_dn9, locals.var_ic_1_dn10, locals.var_ic_1_dn11, locals.var_ic_1_dn12,)
    }
};
        locals.var_ic_1 = assign25930_e19980;
        locals.var_ic_1_dn3 = assign25930_e19980_d_n3;
        locals.var_ic_1_dn4 = assign25930_e19980_d_n4;
        locals.var_ic_1_dn5 = assign25930_e19980_d_n5;
        locals.var_ic_1_dn6 = assign25930_e19980_d_n6;
        locals.var_ic_1_dn7 = assign25930_e19980_d_n7;
        locals.var_ic_1_dn8 = assign25930_e19980_d_n8;
        locals.var_ic_1_dn9 = assign25930_e19980_d_n9;
        locals.var_ic_1_dn10 = assign25930_e19980_d_n10;
        locals.var_ic_1_dn11 = assign25930_e19980_d_n11;
        locals.var_ic_1_dn12 = assign25930_e19980_d_n12;

        let assign25940_e19987: f64 = if ((locals.var_b4soiigbmod != 0.0) || (locals.var_b4soiigcmod != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1629 = assign25940_e19987;

        let (assign25950_e19993, assign25950_e19993_d_n3, assign25950_e19993_d_n4, assign25950_e19993_d_n5, assign25950_e19993_d_n6, assign25950_e19993_d_n7, assign25950_e19993_d_n8, assign25950_e19993_d_n9, assign25950_e19993_d_n10, assign25950_e19993_d_n11, assign25950_e19993_d_n12,) = {
    if (locals.var_guard1629 != 0.0) {
        let assign25950_e19991: f64 = (locals.var_vgs_eff__blk1126 - locals.var_vbs_1);
        (assign25950_e19991, (locals.var_vgs_eff__blk1126_dn3 - locals.var_vbs_1_dn3), (locals.var_vgs_eff__blk1126_dn4 - locals.var_vbs_1_dn4), (locals.var_vgs_eff__blk1126_dn5 - locals.var_vbs_1_dn5), (locals.var_vgs_eff__blk1126_dn6 - locals.var_vbs_1_dn6), (locals.var_vgs_eff__blk1126_dn7 - locals.var_vbs_1_dn7), (locals.var_vgs_eff__blk1126_dn8 - locals.var_vbs_1_dn8), (locals.var_vgs_eff__blk1126_dn9 - locals.var_vbs_1_dn9), (locals.var_vgs_eff__blk1126_dn10 - locals.var_vbs_1_dn10), (locals.var_vgs_eff__blk1126_dn11 - locals.var_vbs_1_dn11), (locals.var_vgs_eff__blk1126_dn12 - locals.var_vbs_1_dn12),)
    } else {
        (locals.var_vgb, locals.var_vgb_dn3, locals.var_vgb_dn4, locals.var_vgb_dn5, locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8, locals.var_vgb_dn9, locals.var_vgb_dn10, locals.var_vgb_dn11, locals.var_vgb_dn12,)
    }
};
        locals.var_vgb = assign25950_e19993;
        locals.var_vgb_dn3 = assign25950_e19993_d_n3;
        locals.var_vgb_dn4 = assign25950_e19993_d_n4;
        locals.var_vgb_dn5 = assign25950_e19993_d_n5;
        locals.var_vgb_dn6 = assign25950_e19993_d_n6;
        locals.var_vgb_dn7 = assign25950_e19993_d_n7;
        locals.var_vgb_dn8 = assign25950_e19993_d_n8;
        locals.var_vgb_dn9 = assign25950_e19993_d_n9;
        locals.var_vgb_dn10 = assign25950_e19993_d_n10;
        locals.var_vgb_dn11 = assign25950_e19993_d_n11;
        locals.var_vgb_dn12 = assign25950_e19993_d_n12;

        let (assign25960_e20005, assign25960_e20005_d_n3, assign25960_e20005_d_n4, assign25960_e20005_d_n5, assign25960_e20005_d_n6, assign25960_e20005_d_n7, assign25960_e20005_d_n8, assign25960_e20005_d_n9, assign25960_e20005_d_n10, assign25960_e20005_d_n11, assign25960_e20005_d_n12,) = {
    if (locals.var_guard1629 != 0.0) {
        let assign25960_e19997: f64 = (locals.var_b4soitype * locals.var_here_b4soivth0);
        let assign25960_e19999: f64 = (assign25960_e19997 - locals.var_phi);
        let assign25960_e20002: f64 = (locals.var_pparam_b4soik1eff * locals.var_sqrtphi);
        let assign25960_e20003: f64 = (assign25960_e19999 - assign25960_e20002);
        (assign25960_e20003, (((locals.var_b4soitype * locals.var_here_b4soivth0_dn3) - locals.var_phi_dn3) - ((locals.var_pparam_b4soik1eff_dn3 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn3))), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn4) - locals.var_phi_dn4) - ((locals.var_pparam_b4soik1eff_dn4 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn4))), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn5) - locals.var_phi_dn5) - ((locals.var_pparam_b4soik1eff_dn5 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn5))), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn6) - locals.var_phi_dn6) - ((locals.var_pparam_b4soik1eff_dn6 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn6))), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn7) - locals.var_phi_dn7) - ((locals.var_pparam_b4soik1eff_dn7 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn7))), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn8) - locals.var_phi_dn8) - ((locals.var_pparam_b4soik1eff_dn8 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn8))), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn9) - locals.var_phi_dn9) - ((locals.var_pparam_b4soik1eff_dn9 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn9))), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn10) - locals.var_phi_dn10) - ((locals.var_pparam_b4soik1eff_dn10 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn10))), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn11) - locals.var_phi_dn11) - ((locals.var_pparam_b4soik1eff_dn11 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn11))), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn12) - locals.var_phi_dn12) - ((locals.var_pparam_b4soik1eff_dn12 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn12))),)
    } else {
        (locals.var_vfb, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11, locals.var_vfb_dn12,)
    }
};
        locals.var_vfb = assign25960_e20005;
        locals.var_vfb_dn3 = assign25960_e20005_d_n3;
        locals.var_vfb_dn4 = assign25960_e20005_d_n4;
        locals.var_vfb_dn5 = assign25960_e20005_d_n5;
        locals.var_vfb_dn6 = assign25960_e20005_d_n6;
        locals.var_vfb_dn7 = assign25960_e20005_d_n7;
        locals.var_vfb_dn8 = assign25960_e20005_d_n8;
        locals.var_vfb_dn9 = assign25960_e20005_d_n9;
        locals.var_vfb_dn10 = assign25960_e20005_d_n10;
        locals.var_vfb_dn11 = assign25960_e20005_d_n11;
        locals.var_vfb_dn12 = assign25960_e20005_d_n12;

        let (assign25970_e20015, assign25970_e20015_d_n3, assign25970_e20015_d_n4, assign25970_e20015_d_n5, assign25970_e20015_d_n6, assign25970_e20015_d_n7, assign25970_e20015_d_n8, assign25970_e20015_d_n9, assign25970_e20015_d_n10, assign25970_e20015_d_n11, assign25970_e20015_d_n12,) = {
    if (locals.var_guard1629 != 0.0) {
        let assign25970_e20009: f64 = (locals.var_vfb - locals.var_vgs_eff__blk1126);
        let assign25970_e20011: f64 = (assign25970_e20009 + locals.var_vbs_1);
        let assign25970_e20013: f64 = (assign25970_e20011 - 0.02);
        (assign25970_e20013, ((locals.var_vfb_dn3 - locals.var_vgs_eff__blk1126_dn3) + locals.var_vbs_1_dn3), ((locals.var_vfb_dn4 - locals.var_vgs_eff__blk1126_dn4) + locals.var_vbs_1_dn4), ((locals.var_vfb_dn5 - locals.var_vgs_eff__blk1126_dn5) + locals.var_vbs_1_dn5), ((locals.var_vfb_dn6 - locals.var_vgs_eff__blk1126_dn6) + locals.var_vbs_1_dn6), ((locals.var_vfb_dn7 - locals.var_vgs_eff__blk1126_dn7) + locals.var_vbs_1_dn7), ((locals.var_vfb_dn8 - locals.var_vgs_eff__blk1126_dn8) + locals.var_vbs_1_dn8), ((locals.var_vfb_dn9 - locals.var_vgs_eff__blk1126_dn9) + locals.var_vbs_1_dn9), ((locals.var_vfb_dn10 - locals.var_vgs_eff__blk1126_dn10) + locals.var_vbs_1_dn10), ((locals.var_vfb_dn11 - locals.var_vgs_eff__blk1126_dn11) + locals.var_vbs_1_dn11), ((locals.var_vfb_dn12 - locals.var_vgs_eff__blk1126_dn12) + locals.var_vbs_1_dn12),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign25970_e20015;
        locals.var_t3__blk1147_dn3 = assign25970_e20015_d_n3;
        locals.var_t3__blk1147_dn4 = assign25970_e20015_d_n4;
        locals.var_t3__blk1147_dn5 = assign25970_e20015_d_n5;
        locals.var_t3__blk1147_dn6 = assign25970_e20015_d_n6;
        locals.var_t3__blk1147_dn7 = assign25970_e20015_d_n7;
        locals.var_t3__blk1147_dn8 = assign25970_e20015_d_n8;
        locals.var_t3__blk1147_dn9 = assign25970_e20015_d_n9;
        locals.var_t3__blk1147_dn10 = assign25970_e20015_d_n10;
        locals.var_t3__blk1147_dn11 = assign25970_e20015_d_n11;
        locals.var_t3__blk1147_dn12 = assign25970_e20015_d_n12;

        let assign25980_e20018: f64 = if locals.var_vfb <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1630 = assign25980_e20018;

    }

    pub(super) fn stamp_transient_block_67(
        locals: &mut StampLocals,
    ) {
        let (assign25990_e20033, assign25990_e20033_d_n3, assign25990_e20033_d_n4, assign25990_e20033_d_n5, assign25990_e20033_d_n6, assign25990_e20033_d_n7, assign25990_e20033_d_n8, assign25990_e20033_d_n9, assign25990_e20033_d_n10, assign25990_e20033_d_n11, assign25990_e20033_d_n12,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1630 != 0.0)) {
        let assign25990_e20024: f64 = (locals.var_t3__blk1147 * locals.var_t3__blk1147);
        let assign25990_e20027: f64 = (4.0 * 0.02);
        let assign25990_e20029: f64 = (assign25990_e20027 * locals.var_vfb);
        let assign25990_e20030: f64 = (assign25990_e20024 - assign25990_e20029);
        let assign25990_e20031: f64 = (assign25990_e20030).sqrt();
        (assign25990_e20031, ((((locals.var_t3__blk1147_dn3 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn3)) - (assign25990_e20027 * locals.var_vfb_dn3)) / (2.0 * assign25990_e20031)), ((((locals.var_t3__blk1147_dn4 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn4)) - (assign25990_e20027 * locals.var_vfb_dn4)) / (2.0 * assign25990_e20031)), ((((locals.var_t3__blk1147_dn5 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn5)) - (assign25990_e20027 * locals.var_vfb_dn5)) / (2.0 * assign25990_e20031)), ((((locals.var_t3__blk1147_dn6 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn6)) - (assign25990_e20027 * locals.var_vfb_dn6)) / (2.0 * assign25990_e20031)), ((((locals.var_t3__blk1147_dn7 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn7)) - (assign25990_e20027 * locals.var_vfb_dn7)) / (2.0 * assign25990_e20031)), ((((locals.var_t3__blk1147_dn8 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn8)) - (assign25990_e20027 * locals.var_vfb_dn8)) / (2.0 * assign25990_e20031)), ((((locals.var_t3__blk1147_dn9 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn9)) - (assign25990_e20027 * locals.var_vfb_dn9)) / (2.0 * assign25990_e20031)), ((((locals.var_t3__blk1147_dn10 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn10)) - (assign25990_e20027 * locals.var_vfb_dn10)) / (2.0 * assign25990_e20031)), ((((locals.var_t3__blk1147_dn11 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn11)) - (assign25990_e20027 * locals.var_vfb_dn11)) / (2.0 * assign25990_e20031)), ((((locals.var_t3__blk1147_dn12 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn12)) - (assign25990_e20027 * locals.var_vfb_dn12)) / (2.0 * assign25990_e20031)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign25990_e20033;
        locals.var_t0__blk1144_dn3 = assign25990_e20033_d_n3;
        locals.var_t0__blk1144_dn4 = assign25990_e20033_d_n4;
        locals.var_t0__blk1144_dn5 = assign25990_e20033_d_n5;
        locals.var_t0__blk1144_dn6 = assign25990_e20033_d_n6;
        locals.var_t0__blk1144_dn7 = assign25990_e20033_d_n7;
        locals.var_t0__blk1144_dn8 = assign25990_e20033_d_n8;
        locals.var_t0__blk1144_dn9 = assign25990_e20033_d_n9;
        locals.var_t0__blk1144_dn10 = assign25990_e20033_d_n10;
        locals.var_t0__blk1144_dn11 = assign25990_e20033_d_n11;
        locals.var_t0__blk1144_dn12 = assign25990_e20033_d_n12;

        let (assign26000_e20049, assign26000_e20049_d_n3, assign26000_e20049_d_n4, assign26000_e20049_d_n5, assign26000_e20049_d_n6, assign26000_e20049_d_n7, assign26000_e20049_d_n8, assign26000_e20049_d_n9, assign26000_e20049_d_n10, assign26000_e20049_d_n11, assign26000_e20049_d_n12,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1630 == 0.0)) {
        let assign26000_e20040: f64 = (locals.var_t3__blk1147 * locals.var_t3__blk1147);
        let assign26000_e20043: f64 = (4.0 * 0.02);
        let assign26000_e20045: f64 = (assign26000_e20043 * locals.var_vfb);
        let assign26000_e20046: f64 = (assign26000_e20040 + assign26000_e20045);
        let assign26000_e20047: f64 = (assign26000_e20046).sqrt();
        (assign26000_e20047, ((((locals.var_t3__blk1147_dn3 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn3)) + (assign26000_e20043 * locals.var_vfb_dn3)) / (2.0 * assign26000_e20047)), ((((locals.var_t3__blk1147_dn4 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn4)) + (assign26000_e20043 * locals.var_vfb_dn4)) / (2.0 * assign26000_e20047)), ((((locals.var_t3__blk1147_dn5 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn5)) + (assign26000_e20043 * locals.var_vfb_dn5)) / (2.0 * assign26000_e20047)), ((((locals.var_t3__blk1147_dn6 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn6)) + (assign26000_e20043 * locals.var_vfb_dn6)) / (2.0 * assign26000_e20047)), ((((locals.var_t3__blk1147_dn7 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn7)) + (assign26000_e20043 * locals.var_vfb_dn7)) / (2.0 * assign26000_e20047)), ((((locals.var_t3__blk1147_dn8 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn8)) + (assign26000_e20043 * locals.var_vfb_dn8)) / (2.0 * assign26000_e20047)), ((((locals.var_t3__blk1147_dn9 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn9)) + (assign26000_e20043 * locals.var_vfb_dn9)) / (2.0 * assign26000_e20047)), ((((locals.var_t3__blk1147_dn10 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn10)) + (assign26000_e20043 * locals.var_vfb_dn10)) / (2.0 * assign26000_e20047)), ((((locals.var_t3__blk1147_dn11 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn11)) + (assign26000_e20043 * locals.var_vfb_dn11)) / (2.0 * assign26000_e20047)), ((((locals.var_t3__blk1147_dn12 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn12)) + (assign26000_e20043 * locals.var_vfb_dn12)) / (2.0 * assign26000_e20047)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign26000_e20049;
        locals.var_t0__blk1144_dn3 = assign26000_e20049_d_n3;
        locals.var_t0__blk1144_dn4 = assign26000_e20049_d_n4;
        locals.var_t0__blk1144_dn5 = assign26000_e20049_d_n5;
        locals.var_t0__blk1144_dn6 = assign26000_e20049_d_n6;
        locals.var_t0__blk1144_dn7 = assign26000_e20049_d_n7;
        locals.var_t0__blk1144_dn8 = assign26000_e20049_d_n8;
        locals.var_t0__blk1144_dn9 = assign26000_e20049_d_n9;
        locals.var_t0__blk1144_dn10 = assign26000_e20049_d_n10;
        locals.var_t0__blk1144_dn11 = assign26000_e20049_d_n11;
        locals.var_t0__blk1144_dn12 = assign26000_e20049_d_n12;

        let (assign26010_e20059, assign26010_e20059_d_n3, assign26010_e20059_d_n4, assign26010_e20059_d_n5, assign26010_e20059_d_n6, assign26010_e20059_d_n7, assign26010_e20059_d_n8, assign26010_e20059_d_n9, assign26010_e20059_d_n10, assign26010_e20059_d_n11, assign26010_e20059_d_n12,) = {
    if (locals.var_guard1629 != 0.0) {
        let assign26010_e20055: f64 = (locals.var_t3__blk1147 + locals.var_t0__blk1144);
        let assign26010_e20056: f64 = (0.5 * assign26010_e20055);
        let assign26010_e20057: f64 = (locals.var_vfb - assign26010_e20056);
        (assign26010_e20057, (locals.var_vfb_dn3 - (0.5 * (locals.var_t3__blk1147_dn3 + locals.var_t0__blk1144_dn3))), (locals.var_vfb_dn4 - (0.5 * (locals.var_t3__blk1147_dn4 + locals.var_t0__blk1144_dn4))), (locals.var_vfb_dn5 - (0.5 * (locals.var_t3__blk1147_dn5 + locals.var_t0__blk1144_dn5))), (locals.var_vfb_dn6 - (0.5 * (locals.var_t3__blk1147_dn6 + locals.var_t0__blk1144_dn6))), (locals.var_vfb_dn7 - (0.5 * (locals.var_t3__blk1147_dn7 + locals.var_t0__blk1144_dn7))), (locals.var_vfb_dn8 - (0.5 * (locals.var_t3__blk1147_dn8 + locals.var_t0__blk1144_dn8))), (locals.var_vfb_dn9 - (0.5 * (locals.var_t3__blk1147_dn9 + locals.var_t0__blk1144_dn9))), (locals.var_vfb_dn10 - (0.5 * (locals.var_t3__blk1147_dn10 + locals.var_t0__blk1144_dn10))), (locals.var_vfb_dn11 - (0.5 * (locals.var_t3__blk1147_dn11 + locals.var_t0__blk1144_dn11))), (locals.var_vfb_dn12 - (0.5 * (locals.var_t3__blk1147_dn12 + locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_vfbeff, locals.var_vfbeff_dn3, locals.var_vfbeff_dn4, locals.var_vfbeff_dn5, locals.var_vfbeff_dn6, locals.var_vfbeff_dn7, locals.var_vfbeff_dn8, locals.var_vfbeff_dn9, locals.var_vfbeff_dn10, locals.var_vfbeff_dn11, locals.var_vfbeff_dn12,)
    }
};
        locals.var_vfbeff = assign26010_e20059;
        locals.var_vfbeff_dn3 = assign26010_e20059_d_n3;
        locals.var_vfbeff_dn4 = assign26010_e20059_d_n4;
        locals.var_vfbeff_dn5 = assign26010_e20059_d_n5;
        locals.var_vfbeff_dn6 = assign26010_e20059_d_n6;
        locals.var_vfbeff_dn7 = assign26010_e20059_d_n7;
        locals.var_vfbeff_dn8 = assign26010_e20059_d_n8;
        locals.var_vfbeff_dn9 = assign26010_e20059_d_n9;
        locals.var_vfbeff_dn10 = assign26010_e20059_d_n10;
        locals.var_vfbeff_dn11 = assign26010_e20059_d_n11;
        locals.var_vfbeff_dn12 = assign26010_e20059_d_n12;

        let (assign26020_e20065, assign26020_e20065_d_n3, assign26020_e20065_d_n4, assign26020_e20065_d_n5, assign26020_e20065_d_n6, assign26020_e20065_d_n7, assign26020_e20065_d_n8, assign26020_e20065_d_n9, assign26020_e20065_d_n10, assign26020_e20065_d_n11, assign26020_e20065_d_n12,) = {
    if (locals.var_guard1629 != 0.0) {
        let assign26020_e20063: f64 = (locals.var_vfb - locals.var_vfbeff);
        (assign26020_e20063, (locals.var_vfb_dn3 - locals.var_vfbeff_dn3), (locals.var_vfb_dn4 - locals.var_vfbeff_dn4), (locals.var_vfb_dn5 - locals.var_vfbeff_dn5), (locals.var_vfb_dn6 - locals.var_vfbeff_dn6), (locals.var_vfb_dn7 - locals.var_vfbeff_dn7), (locals.var_vfb_dn8 - locals.var_vfbeff_dn8), (locals.var_vfb_dn9 - locals.var_vfbeff_dn9), (locals.var_vfb_dn10 - locals.var_vfbeff_dn10), (locals.var_vfb_dn11 - locals.var_vfbeff_dn11), (locals.var_vfb_dn12 - locals.var_vfbeff_dn12),)
    } else {
        (locals.var_voxacc, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8, locals.var_voxacc_dn9, locals.var_voxacc_dn10, locals.var_voxacc_dn11, locals.var_voxacc_dn12,)
    }
};
        locals.var_voxacc = assign26020_e20065;
        locals.var_voxacc_dn3 = assign26020_e20065_d_n3;
        locals.var_voxacc_dn4 = assign26020_e20065_d_n4;
        locals.var_voxacc_dn5 = assign26020_e20065_d_n5;
        locals.var_voxacc_dn6 = assign26020_e20065_d_n6;
        locals.var_voxacc_dn7 = assign26020_e20065_d_n7;
        locals.var_voxacc_dn8 = assign26020_e20065_d_n8;
        locals.var_voxacc_dn9 = assign26020_e20065_d_n9;
        locals.var_voxacc_dn10 = assign26020_e20065_d_n10;
        locals.var_voxacc_dn11 = assign26020_e20065_d_n11;
        locals.var_voxacc_dn12 = assign26020_e20065_d_n12;

        let assign26030_e20068: f64 = if locals.var_voxacc < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1631 = assign26030_e20068;

        let (assign26040_e20074, assign26040_e20074_d_n3, assign26040_e20074_d_n4, assign26040_e20074_d_n5, assign26040_e20074_d_n6, assign26040_e20074_d_n7, assign26040_e20074_d_n8, assign26040_e20074_d_n9, assign26040_e20074_d_n10, assign26040_e20074_d_n11, assign26040_e20074_d_n12,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_voxacc, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8, locals.var_voxacc_dn9, locals.var_voxacc_dn10, locals.var_voxacc_dn11, locals.var_voxacc_dn12,)
    }
};
        locals.var_voxacc = assign26040_e20074;
        locals.var_voxacc_dn3 = assign26040_e20074_d_n3;
        locals.var_voxacc_dn4 = assign26040_e20074_d_n4;
        locals.var_voxacc_dn5 = assign26040_e20074_d_n5;
        locals.var_voxacc_dn6 = assign26040_e20074_d_n6;
        locals.var_voxacc_dn7 = assign26040_e20074_d_n7;
        locals.var_voxacc_dn8 = assign26040_e20074_d_n8;
        locals.var_voxacc_dn9 = assign26040_e20074_d_n9;
        locals.var_voxacc_dn10 = assign26040_e20074_d_n10;
        locals.var_voxacc_dn11 = assign26040_e20074_d_n11;
        locals.var_voxacc_dn12 = assign26040_e20074_d_n12;

        let assign26050_e20077: f64 = if locals.var_pparam_b4soik1ox == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1632 = assign26050_e20077;

        let (assign26060_e20083, assign26060_e20083_d_n3, assign26060_e20083_d_n4, assign26060_e20083_d_n5, assign26060_e20083_d_n6, assign26060_e20083_d_n7, assign26060_e20083_d_n8, assign26060_e20083_d_n9, assign26060_e20083_d_n10, assign26060_e20083_d_n11, assign26060_e20083_d_n12,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1632 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_voxdepinv, locals.var_voxdepinv_dn3, locals.var_voxdepinv_dn4, locals.var_voxdepinv_dn5, locals.var_voxdepinv_dn6, locals.var_voxdepinv_dn7, locals.var_voxdepinv_dn8, locals.var_voxdepinv_dn9, locals.var_voxdepinv_dn10, locals.var_voxdepinv_dn11, locals.var_voxdepinv_dn12,)
    }
};
        locals.var_voxdepinv = assign26060_e20083;
        locals.var_voxdepinv_dn3 = assign26060_e20083_d_n3;
        locals.var_voxdepinv_dn4 = assign26060_e20083_d_n4;
        locals.var_voxdepinv_dn5 = assign26060_e20083_d_n5;
        locals.var_voxdepinv_dn6 = assign26060_e20083_d_n6;
        locals.var_voxdepinv_dn7 = assign26060_e20083_d_n7;
        locals.var_voxdepinv_dn8 = assign26060_e20083_d_n8;
        locals.var_voxdepinv_dn9 = assign26060_e20083_d_n9;
        locals.var_voxdepinv_dn10 = assign26060_e20083_d_n10;
        locals.var_voxdepinv_dn11 = assign26060_e20083_d_n11;
        locals.var_voxdepinv_dn12 = assign26060_e20083_d_n12;

        let (assign26070_e20096, assign26070_e20096_d_n3, assign26070_e20096_d_n4, assign26070_e20096_d_n5, assign26070_e20096_d_n6, assign26070_e20096_d_n7, assign26070_e20096_d_n8, assign26070_e20096_d_n9, assign26070_e20096_d_n10, assign26070_e20096_d_n11, assign26070_e20096_d_n12,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1632 == 0.0)) {
        let assign26070_e20090: f64 = (locals.var_vgs_eff__blk1126 - locals.var_vgsteff__blk1175);
        let assign26070_e20092: f64 = (assign26070_e20090 - locals.var_vfbeff);
        let assign26070_e20094: f64 = (assign26070_e20092 - locals.var_vbseff);
        (assign26070_e20094, (((locals.var_vgs_eff__blk1126_dn3 - locals.var_vgsteff__blk1175_dn3) - locals.var_vfbeff_dn3) - locals.var_vbseff_dn3), (((locals.var_vgs_eff__blk1126_dn4 - locals.var_vgsteff__blk1175_dn4) - locals.var_vfbeff_dn4) - locals.var_vbseff_dn4), (((locals.var_vgs_eff__blk1126_dn5 - locals.var_vgsteff__blk1175_dn5) - locals.var_vfbeff_dn5) - locals.var_vbseff_dn5), (((locals.var_vgs_eff__blk1126_dn6 - locals.var_vgsteff__blk1175_dn6) - locals.var_vfbeff_dn6) - locals.var_vbseff_dn6), (((locals.var_vgs_eff__blk1126_dn7 - locals.var_vgsteff__blk1175_dn7) - locals.var_vfbeff_dn7) - locals.var_vbseff_dn7), (((locals.var_vgs_eff__blk1126_dn8 - locals.var_vgsteff__blk1175_dn8) - locals.var_vfbeff_dn8) - locals.var_vbseff_dn8), (((locals.var_vgs_eff__blk1126_dn9 - locals.var_vgsteff__blk1175_dn9) - locals.var_vfbeff_dn9) - locals.var_vbseff_dn9), (((locals.var_vgs_eff__blk1126_dn10 - locals.var_vgsteff__blk1175_dn10) - locals.var_vfbeff_dn10) - locals.var_vbseff_dn10), (((locals.var_vgs_eff__blk1126_dn11 - locals.var_vgsteff__blk1175_dn11) - locals.var_vfbeff_dn11) - locals.var_vbseff_dn11), (((locals.var_vgs_eff__blk1126_dn12 - locals.var_vgsteff__blk1175_dn12) - locals.var_vfbeff_dn12) - locals.var_vbseff_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign26070_e20096;
        locals.var_t0__blk1144_dn3 = assign26070_e20096_d_n3;
        locals.var_t0__blk1144_dn4 = assign26070_e20096_d_n4;
        locals.var_t0__blk1144_dn5 = assign26070_e20096_d_n5;
        locals.var_t0__blk1144_dn6 = assign26070_e20096_d_n6;
        locals.var_t0__blk1144_dn7 = assign26070_e20096_d_n7;
        locals.var_t0__blk1144_dn8 = assign26070_e20096_d_n8;
        locals.var_t0__blk1144_dn9 = assign26070_e20096_d_n9;
        locals.var_t0__blk1144_dn10 = assign26070_e20096_d_n10;
        locals.var_t0__blk1144_dn11 = assign26070_e20096_d_n11;
        locals.var_t0__blk1144_dn12 = assign26070_e20096_d_n12;

        let assign26080_e20099: f64 = if locals.var_t0__blk1144 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1633 = assign26080_e20099;

        let (assign26090_e20110, assign26090_e20110_d_n3, assign26090_e20110_d_n4, assign26090_e20110_d_n5, assign26090_e20110_d_n6, assign26090_e20110_d_n7, assign26090_e20110_d_n8, assign26090_e20110_d_n9, assign26090_e20110_d_n10, assign26090_e20110_d_n11, assign26090_e20110_d_n12,) = {
    if (((locals.var_guard1629 != 0.0) && (locals.var_guard1632 == 0.0)) && (locals.var_guard1633 != 0.0)) {
        let assign26090_e20108: f64 = (locals.var_t0__blk1144 / locals.var_pparam_b4soik1ox);
        (assign26090_e20108, (((locals.var_t0__blk1144_dn3 * locals.var_pparam_b4soik1ox) - (locals.var_t0__blk1144 * locals.var_pparam_b4soik1ox_dn3)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)), (((locals.var_t0__blk1144_dn4 * locals.var_pparam_b4soik1ox) - (locals.var_t0__blk1144 * locals.var_pparam_b4soik1ox_dn4)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)), (((locals.var_t0__blk1144_dn5 * locals.var_pparam_b4soik1ox) - (locals.var_t0__blk1144 * locals.var_pparam_b4soik1ox_dn5)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)), (((locals.var_t0__blk1144_dn6 * locals.var_pparam_b4soik1ox) - (locals.var_t0__blk1144 * locals.var_pparam_b4soik1ox_dn6)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)), (((locals.var_t0__blk1144_dn7 * locals.var_pparam_b4soik1ox) - (locals.var_t0__blk1144 * locals.var_pparam_b4soik1ox_dn7)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)), (((locals.var_t0__blk1144_dn8 * locals.var_pparam_b4soik1ox) - (locals.var_t0__blk1144 * locals.var_pparam_b4soik1ox_dn8)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)), (((locals.var_t0__blk1144_dn9 * locals.var_pparam_b4soik1ox) - (locals.var_t0__blk1144 * locals.var_pparam_b4soik1ox_dn9)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)), (((locals.var_t0__blk1144_dn10 * locals.var_pparam_b4soik1ox) - (locals.var_t0__blk1144 * locals.var_pparam_b4soik1ox_dn10)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)), (((locals.var_t0__blk1144_dn11 * locals.var_pparam_b4soik1ox) - (locals.var_t0__blk1144 * locals.var_pparam_b4soik1ox_dn11)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)), (((locals.var_t0__blk1144_dn12 * locals.var_pparam_b4soik1ox) - (locals.var_t0__blk1144 * locals.var_pparam_b4soik1ox_dn12)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign26090_e20110;
        locals.var_t1__blk1145_dn3 = assign26090_e20110_d_n3;
        locals.var_t1__blk1145_dn4 = assign26090_e20110_d_n4;
        locals.var_t1__blk1145_dn5 = assign26090_e20110_d_n5;
        locals.var_t1__blk1145_dn6 = assign26090_e20110_d_n6;
        locals.var_t1__blk1145_dn7 = assign26090_e20110_d_n7;
        locals.var_t1__blk1145_dn8 = assign26090_e20110_d_n8;
        locals.var_t1__blk1145_dn9 = assign26090_e20110_d_n9;
        locals.var_t1__blk1145_dn10 = assign26090_e20110_d_n10;
        locals.var_t1__blk1145_dn11 = assign26090_e20110_d_n11;
        locals.var_t1__blk1145_dn12 = assign26090_e20110_d_n12;

        let (assign26100_e20136, assign26100_e20136_d_n3, assign26100_e20136_d_n4, assign26100_e20136_d_n5, assign26100_e20136_d_n6, assign26100_e20136_d_n7, assign26100_e20136_d_n8, assign26100_e20136_d_n9, assign26100_e20136_d_n10, assign26100_e20136_d_n11, assign26100_e20136_d_n12,) = {
    if (((locals.var_guard1629 != 0.0) && (locals.var_guard1632 == 0.0)) && (locals.var_guard1633 == 0.0)) {
        let assign26100_e20120: f64 = (locals.var_pparam_b4soik1ox / 2.0);
        let assign26100_e20122: f64 = (-1.0);
        let assign26100_e20126: f64 = (4.0 * locals.var_t0__blk1144);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_pparam_b4soik1ox;
        let assign26100_e20128: f64 = (assign26100_e20126 * __rspice_inv_cse_0);
        let assign26100_e20130: f64 = (assign26100_e20128 * __rspice_inv_cse_0);
        let assign26100_e20131: f64 = (1.0 + assign26100_e20130);
        let assign26100_e20132: f64 = (assign26100_e20131).sqrt();
        let assign26100_e20133: f64 = (assign26100_e20122 + assign26100_e20132);
        let assign26100_e20134: f64 = (assign26100_e20120 * assign26100_e20133);
        (assign26100_e20134, (((locals.var_pparam_b4soik1ox_dn3 / 2.0) * assign26100_e20133) + (assign26100_e20120 * ((((((((4.0 * locals.var_t0__blk1144_dn3) * locals.var_pparam_b4soik1ox) - (assign26100_e20126 * locals.var_pparam_b4soik1ox_dn3)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) * locals.var_pparam_b4soik1ox) - (assign26100_e20128 * locals.var_pparam_b4soik1ox_dn3)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) / (2.0 * assign26100_e20132)))), (((locals.var_pparam_b4soik1ox_dn4 / 2.0) * assign26100_e20133) + (assign26100_e20120 * ((((((((4.0 * locals.var_t0__blk1144_dn4) * locals.var_pparam_b4soik1ox) - (assign26100_e20126 * locals.var_pparam_b4soik1ox_dn4)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) * locals.var_pparam_b4soik1ox) - (assign26100_e20128 * locals.var_pparam_b4soik1ox_dn4)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) / (2.0 * assign26100_e20132)))), (((locals.var_pparam_b4soik1ox_dn5 / 2.0) * assign26100_e20133) + (assign26100_e20120 * ((((((((4.0 * locals.var_t0__blk1144_dn5) * locals.var_pparam_b4soik1ox) - (assign26100_e20126 * locals.var_pparam_b4soik1ox_dn5)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) * locals.var_pparam_b4soik1ox) - (assign26100_e20128 * locals.var_pparam_b4soik1ox_dn5)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) / (2.0 * assign26100_e20132)))), (((locals.var_pparam_b4soik1ox_dn6 / 2.0) * assign26100_e20133) + (assign26100_e20120 * ((((((((4.0 * locals.var_t0__blk1144_dn6) * locals.var_pparam_b4soik1ox) - (assign26100_e20126 * locals.var_pparam_b4soik1ox_dn6)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) * locals.var_pparam_b4soik1ox) - (assign26100_e20128 * locals.var_pparam_b4soik1ox_dn6)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) / (2.0 * assign26100_e20132)))), (((locals.var_pparam_b4soik1ox_dn7 / 2.0) * assign26100_e20133) + (assign26100_e20120 * ((((((((4.0 * locals.var_t0__blk1144_dn7) * locals.var_pparam_b4soik1ox) - (assign26100_e20126 * locals.var_pparam_b4soik1ox_dn7)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) * locals.var_pparam_b4soik1ox) - (assign26100_e20128 * locals.var_pparam_b4soik1ox_dn7)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) / (2.0 * assign26100_e20132)))), (((locals.var_pparam_b4soik1ox_dn8 / 2.0) * assign26100_e20133) + (assign26100_e20120 * ((((((((4.0 * locals.var_t0__blk1144_dn8) * locals.var_pparam_b4soik1ox) - (assign26100_e20126 * locals.var_pparam_b4soik1ox_dn8)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) * locals.var_pparam_b4soik1ox) - (assign26100_e20128 * locals.var_pparam_b4soik1ox_dn8)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) / (2.0 * assign26100_e20132)))), (((locals.var_pparam_b4soik1ox_dn9 / 2.0) * assign26100_e20133) + (assign26100_e20120 * ((((((((4.0 * locals.var_t0__blk1144_dn9) * locals.var_pparam_b4soik1ox) - (assign26100_e20126 * locals.var_pparam_b4soik1ox_dn9)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) * locals.var_pparam_b4soik1ox) - (assign26100_e20128 * locals.var_pparam_b4soik1ox_dn9)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) / (2.0 * assign26100_e20132)))), (((locals.var_pparam_b4soik1ox_dn10 / 2.0) * assign26100_e20133) + (assign26100_e20120 * ((((((((4.0 * locals.var_t0__blk1144_dn10) * locals.var_pparam_b4soik1ox) - (assign26100_e20126 * locals.var_pparam_b4soik1ox_dn10)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) * locals.var_pparam_b4soik1ox) - (assign26100_e20128 * locals.var_pparam_b4soik1ox_dn10)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) / (2.0 * assign26100_e20132)))), (((locals.var_pparam_b4soik1ox_dn11 / 2.0) * assign26100_e20133) + (assign26100_e20120 * ((((((((4.0 * locals.var_t0__blk1144_dn11) * locals.var_pparam_b4soik1ox) - (assign26100_e20126 * locals.var_pparam_b4soik1ox_dn11)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) * locals.var_pparam_b4soik1ox) - (assign26100_e20128 * locals.var_pparam_b4soik1ox_dn11)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) / (2.0 * assign26100_e20132)))), (((locals.var_pparam_b4soik1ox_dn12 / 2.0) * assign26100_e20133) + (assign26100_e20120 * ((((((((4.0 * locals.var_t0__blk1144_dn12) * locals.var_pparam_b4soik1ox) - (assign26100_e20126 * locals.var_pparam_b4soik1ox_dn12)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) * locals.var_pparam_b4soik1ox) - (assign26100_e20128 * locals.var_pparam_b4soik1ox_dn12)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox)) / (2.0 * assign26100_e20132)))),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign26100_e20136;
        locals.var_t1__blk1145_dn3 = assign26100_e20136_d_n3;
        locals.var_t1__blk1145_dn4 = assign26100_e20136_d_n4;
        locals.var_t1__blk1145_dn5 = assign26100_e20136_d_n5;
        locals.var_t1__blk1145_dn6 = assign26100_e20136_d_n6;
        locals.var_t1__blk1145_dn7 = assign26100_e20136_d_n7;
        locals.var_t1__blk1145_dn8 = assign26100_e20136_d_n8;
        locals.var_t1__blk1145_dn9 = assign26100_e20136_d_n9;
        locals.var_t1__blk1145_dn10 = assign26100_e20136_d_n10;
        locals.var_t1__blk1145_dn11 = assign26100_e20136_d_n11;
        locals.var_t1__blk1145_dn12 = assign26100_e20136_d_n12;

        let (assign26110_e20151, assign26110_e20151_d_n3, assign26110_e20151_d_n4, assign26110_e20151_d_n5, assign26110_e20151_d_n6, assign26110_e20151_d_n7, assign26110_e20151_d_n8, assign26110_e20151_d_n9, assign26110_e20151_d_n10, assign26110_e20151_d_n11, assign26110_e20151_d_n12,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1632 == 0.0)) {
        let assign26110_e20144: f64 = (locals.var_t1__blk1145 * locals.var_t1__blk1145);
        let assign26110_e20146: f64 = (assign26110_e20144 + locals.var_vbs_1);
        let assign26110_e20147: f64 = (locals.var_vgs_eff__blk1126 - assign26110_e20146);
        let assign26110_e20149: f64 = (assign26110_e20147 - locals.var_vfb);
        (assign26110_e20149, ((locals.var_vgs_eff__blk1126_dn3 - (((locals.var_t1__blk1145_dn3 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn3)) + locals.var_vbs_1_dn3)) - locals.var_vfb_dn3), ((locals.var_vgs_eff__blk1126_dn4 - (((locals.var_t1__blk1145_dn4 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn4)) + locals.var_vbs_1_dn4)) - locals.var_vfb_dn4), ((locals.var_vgs_eff__blk1126_dn5 - (((locals.var_t1__blk1145_dn5 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn5)) + locals.var_vbs_1_dn5)) - locals.var_vfb_dn5), ((locals.var_vgs_eff__blk1126_dn6 - (((locals.var_t1__blk1145_dn6 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn6)) + locals.var_vbs_1_dn6)) - locals.var_vfb_dn6), ((locals.var_vgs_eff__blk1126_dn7 - (((locals.var_t1__blk1145_dn7 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn7)) + locals.var_vbs_1_dn7)) - locals.var_vfb_dn7), ((locals.var_vgs_eff__blk1126_dn8 - (((locals.var_t1__blk1145_dn8 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn8)) + locals.var_vbs_1_dn8)) - locals.var_vfb_dn8), ((locals.var_vgs_eff__blk1126_dn9 - (((locals.var_t1__blk1145_dn9 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn9)) + locals.var_vbs_1_dn9)) - locals.var_vfb_dn9), ((locals.var_vgs_eff__blk1126_dn10 - (((locals.var_t1__blk1145_dn10 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn10)) + locals.var_vbs_1_dn10)) - locals.var_vfb_dn10), ((locals.var_vgs_eff__blk1126_dn11 - (((locals.var_t1__blk1145_dn11 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn11)) + locals.var_vbs_1_dn11)) - locals.var_vfb_dn11), ((locals.var_vgs_eff__blk1126_dn12 - (((locals.var_t1__blk1145_dn12 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn12)) + locals.var_vbs_1_dn12)) - locals.var_vfb_dn12),)
    } else {
        (locals.var_voxdepinv, locals.var_voxdepinv_dn3, locals.var_voxdepinv_dn4, locals.var_voxdepinv_dn5, locals.var_voxdepinv_dn6, locals.var_voxdepinv_dn7, locals.var_voxdepinv_dn8, locals.var_voxdepinv_dn9, locals.var_voxdepinv_dn10, locals.var_voxdepinv_dn11, locals.var_voxdepinv_dn12,)
    }
};
        locals.var_voxdepinv = assign26110_e20151;
        locals.var_voxdepinv_dn3 = assign26110_e20151_d_n3;
        locals.var_voxdepinv_dn4 = assign26110_e20151_d_n4;
        locals.var_voxdepinv_dn5 = assign26110_e20151_d_n5;
        locals.var_voxdepinv_dn6 = assign26110_e20151_d_n6;
        locals.var_voxdepinv_dn7 = assign26110_e20151_d_n7;
        locals.var_voxdepinv_dn8 = assign26110_e20151_d_n8;
        locals.var_voxdepinv_dn9 = assign26110_e20151_d_n9;
        locals.var_voxdepinv_dn10 = assign26110_e20151_d_n10;
        locals.var_voxdepinv_dn11 = assign26110_e20151_d_n11;
        locals.var_voxdepinv_dn12 = assign26110_e20151_d_n12;

        let (assign26120_e20156, assign26120_e20156_d_n3, assign26120_e20156_d_n4, assign26120_e20156_d_n5, assign26120_e20156_d_n6, assign26120_e20156_d_n7, assign26120_e20156_d_n8, assign26120_e20156_d_n9, assign26120_e20156_d_n10, assign26120_e20156_d_n11, assign26120_e20156_d_n12,) = {
    if (locals.var_guard1629 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfb, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11, locals.var_vfb_dn12,)
    }
};
        locals.var_vfb = assign26120_e20156;
        locals.var_vfb_dn3 = assign26120_e20156_d_n3;
        locals.var_vfb_dn4 = assign26120_e20156_d_n4;
        locals.var_vfb_dn5 = assign26120_e20156_d_n5;
        locals.var_vfb_dn6 = assign26120_e20156_d_n6;
        locals.var_vfb_dn7 = assign26120_e20156_d_n7;
        locals.var_vfb_dn8 = assign26120_e20156_d_n8;
        locals.var_vfb_dn9 = assign26120_e20156_d_n9;
        locals.var_vfb_dn10 = assign26120_e20156_d_n10;
        locals.var_vfb_dn11 = assign26120_e20156_d_n11;
        locals.var_vfb_dn12 = assign26120_e20156_d_n12;

        let (assign26130_e20161, assign26130_e20161_d_n3, assign26130_e20161_d_n4, assign26130_e20161_d_n5, assign26130_e20161_d_n6, assign26130_e20161_d_n7, assign26130_e20161_d_n8, assign26130_e20161_d_n9, assign26130_e20161_d_n10, assign26130_e20161_d_n11, assign26130_e20161_d_n12,) = {
    if (locals.var_guard1629 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgb, locals.var_vgb_dn3, locals.var_vgb_dn4, locals.var_vgb_dn5, locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8, locals.var_vgb_dn9, locals.var_vgb_dn10, locals.var_vgb_dn11, locals.var_vgb_dn12,)
    }
};
        locals.var_vgb = assign26130_e20161;
        locals.var_vgb_dn3 = assign26130_e20161_d_n3;
        locals.var_vgb_dn4 = assign26130_e20161_d_n4;
        locals.var_vgb_dn5 = assign26130_e20161_d_n5;
        locals.var_vgb_dn6 = assign26130_e20161_d_n6;
        locals.var_vgb_dn7 = assign26130_e20161_d_n7;
        locals.var_vgb_dn8 = assign26130_e20161_d_n8;
        locals.var_vgb_dn9 = assign26130_e20161_d_n9;
        locals.var_vgb_dn10 = assign26130_e20161_d_n10;
        locals.var_vgb_dn11 = assign26130_e20161_d_n11;
        locals.var_vgb_dn12 = assign26130_e20161_d_n12;

        let (assign26140_e20166, assign26140_e20166_d_n3, assign26140_e20166_d_n4, assign26140_e20166_d_n5, assign26140_e20166_d_n6, assign26140_e20166_d_n7, assign26140_e20166_d_n8, assign26140_e20166_d_n9, assign26140_e20166_d_n10, assign26140_e20166_d_n11, assign26140_e20166_d_n12,) = {
    if (locals.var_guard1629 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_voxacc, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8, locals.var_voxacc_dn9, locals.var_voxacc_dn10, locals.var_voxacc_dn11, locals.var_voxacc_dn12,)
    }
};
        locals.var_voxacc = assign26140_e20166;
        locals.var_voxacc_dn3 = assign26140_e20166_d_n3;
        locals.var_voxacc_dn4 = assign26140_e20166_d_n4;
        locals.var_voxacc_dn5 = assign26140_e20166_d_n5;
        locals.var_voxacc_dn6 = assign26140_e20166_d_n6;
        locals.var_voxacc_dn7 = assign26140_e20166_d_n7;
        locals.var_voxacc_dn8 = assign26140_e20166_d_n8;
        locals.var_voxacc_dn9 = assign26140_e20166_d_n9;
        locals.var_voxacc_dn10 = assign26140_e20166_d_n10;
        locals.var_voxacc_dn11 = assign26140_e20166_d_n11;
        locals.var_voxacc_dn12 = assign26140_e20166_d_n12;

        let (assign26150_e20171, assign26150_e20171_d_n3, assign26150_e20171_d_n4, assign26150_e20171_d_n5, assign26150_e20171_d_n6, assign26150_e20171_d_n7, assign26150_e20171_d_n8, assign26150_e20171_d_n9, assign26150_e20171_d_n10, assign26150_e20171_d_n11, assign26150_e20171_d_n12,) = {
    if (locals.var_guard1629 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_voxdepinv, locals.var_voxdepinv_dn3, locals.var_voxdepinv_dn4, locals.var_voxdepinv_dn5, locals.var_voxdepinv_dn6, locals.var_voxdepinv_dn7, locals.var_voxdepinv_dn8, locals.var_voxdepinv_dn9, locals.var_voxdepinv_dn10, locals.var_voxdepinv_dn11, locals.var_voxdepinv_dn12,)
    }
};
        locals.var_voxdepinv = assign26150_e20171;
        locals.var_voxdepinv_dn3 = assign26150_e20171_d_n3;
        locals.var_voxdepinv_dn4 = assign26150_e20171_d_n4;
        locals.var_voxdepinv_dn5 = assign26150_e20171_d_n5;
        locals.var_voxdepinv_dn6 = assign26150_e20171_d_n6;
        locals.var_voxdepinv_dn7 = assign26150_e20171_d_n7;
        locals.var_voxdepinv_dn8 = assign26150_e20171_d_n8;
        locals.var_voxdepinv_dn9 = assign26150_e20171_d_n9;
        locals.var_voxdepinv_dn10 = assign26150_e20171_d_n10;
        locals.var_voxdepinv_dn11 = assign26150_e20171_d_n11;
        locals.var_voxdepinv_dn12 = assign26150_e20171_d_n12;

        let (assign26160_e20177, assign26160_e20177_d_n3, assign26160_e20177_d_n4, assign26160_e20177_d_n5, assign26160_e20177_d_n6, assign26160_e20177_d_n7, assign26160_e20177_d_n8, assign26160_e20177_d_n9, assign26160_e20177_d_n10, assign26160_e20177_d_n11, assign26160_e20177_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26160_e20175: f64 = (locals.var_vtm * locals.var_pparam_b4soinigc);
        (assign26160_e20175, (locals.var_vtm * locals.var_pparam_b4soinigc_dn3), (locals.var_vtm * locals.var_pparam_b4soinigc_dn4), (locals.var_vtm * locals.var_pparam_b4soinigc_dn5), ((locals.var_vtm_dn6 * locals.var_pparam_b4soinigc) + (locals.var_vtm * locals.var_pparam_b4soinigc_dn6)), (locals.var_vtm * locals.var_pparam_b4soinigc_dn7), (locals.var_vtm * locals.var_pparam_b4soinigc_dn8), (locals.var_vtm * locals.var_pparam_b4soinigc_dn9), (locals.var_vtm * locals.var_pparam_b4soinigc_dn10), (locals.var_vtm * locals.var_pparam_b4soinigc_dn11), (locals.var_vtm * locals.var_pparam_b4soinigc_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign26160_e20177;
        locals.var_t0__blk1144_dn3 = assign26160_e20177_d_n3;
        locals.var_t0__blk1144_dn4 = assign26160_e20177_d_n4;
        locals.var_t0__blk1144_dn5 = assign26160_e20177_d_n5;
        locals.var_t0__blk1144_dn6 = assign26160_e20177_d_n6;
        locals.var_t0__blk1144_dn7 = assign26160_e20177_d_n7;
        locals.var_t0__blk1144_dn8 = assign26160_e20177_d_n8;
        locals.var_t0__blk1144_dn9 = assign26160_e20177_d_n9;
        locals.var_t0__blk1144_dn10 = assign26160_e20177_d_n10;
        locals.var_t0__blk1144_dn11 = assign26160_e20177_d_n11;
        locals.var_t0__blk1144_dn12 = assign26160_e20177_d_n12;

        let (assign26170_e20187, assign26170_e20187_d_n3, assign26170_e20187_d_n4, assign26170_e20187_d_n5, assign26170_e20187_d_n6, assign26170_e20187_d_n7, assign26170_e20187_d_n8, assign26170_e20187_d_n9, assign26170_e20187_d_n10, assign26170_e20187_d_n11, assign26170_e20187_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26170_e20182: f64 = (locals.var_b4soitype * locals.var_here_b4soivth0);
        let assign26170_e20183: f64 = (locals.var_vgs_eff__blk1126 - assign26170_e20182);
        let assign26170_e20185: f64 = (assign26170_e20183 / locals.var_t0__blk1144);
        (assign26170_e20185, ((((locals.var_vgs_eff__blk1126_dn3 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn3)) * locals.var_t0__blk1144) - (assign26170_e20183 * locals.var_t0__blk1144_dn3)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), ((((locals.var_vgs_eff__blk1126_dn4 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn4)) * locals.var_t0__blk1144) - (assign26170_e20183 * locals.var_t0__blk1144_dn4)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), ((((locals.var_vgs_eff__blk1126_dn5 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn5)) * locals.var_t0__blk1144) - (assign26170_e20183 * locals.var_t0__blk1144_dn5)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), ((((locals.var_vgs_eff__blk1126_dn6 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn6)) * locals.var_t0__blk1144) - (assign26170_e20183 * locals.var_t0__blk1144_dn6)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), ((((locals.var_vgs_eff__blk1126_dn7 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn7)) * locals.var_t0__blk1144) - (assign26170_e20183 * locals.var_t0__blk1144_dn7)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), ((((locals.var_vgs_eff__blk1126_dn8 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn8)) * locals.var_t0__blk1144) - (assign26170_e20183 * locals.var_t0__blk1144_dn8)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), ((((locals.var_vgs_eff__blk1126_dn9 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn9)) * locals.var_t0__blk1144) - (assign26170_e20183 * locals.var_t0__blk1144_dn9)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), ((((locals.var_vgs_eff__blk1126_dn10 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn10)) * locals.var_t0__blk1144) - (assign26170_e20183 * locals.var_t0__blk1144_dn10)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), ((((locals.var_vgs_eff__blk1126_dn11 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn11)) * locals.var_t0__blk1144) - (assign26170_e20183 * locals.var_t0__blk1144_dn11)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), ((((locals.var_vgs_eff__blk1126_dn12 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn12)) * locals.var_t0__blk1144) - (assign26170_e20183 * locals.var_t0__blk1144_dn12)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)),)
    } else {
        (locals.var_vxnvt, locals.var_vxnvt_dn3, locals.var_vxnvt_dn4, locals.var_vxnvt_dn5, locals.var_vxnvt_dn6, locals.var_vxnvt_dn7, locals.var_vxnvt_dn8, locals.var_vxnvt_dn9, locals.var_vxnvt_dn10, locals.var_vxnvt_dn11, locals.var_vxnvt_dn12,)
    }
};
        locals.var_vxnvt = assign26170_e20187;
        locals.var_vxnvt_dn3 = assign26170_e20187_d_n3;
        locals.var_vxnvt_dn4 = assign26170_e20187_d_n4;
        locals.var_vxnvt_dn5 = assign26170_e20187_d_n5;
        locals.var_vxnvt_dn6 = assign26170_e20187_d_n6;
        locals.var_vxnvt_dn7 = assign26170_e20187_d_n7;
        locals.var_vxnvt_dn8 = assign26170_e20187_d_n8;
        locals.var_vxnvt_dn9 = assign26170_e20187_d_n9;
        locals.var_vxnvt_dn10 = assign26170_e20187_d_n10;
        locals.var_vxnvt_dn11 = assign26170_e20187_d_n11;
        locals.var_vxnvt_dn12 = assign26170_e20187_d_n12;

        let assign26180_e20190: f64 = if locals.var_vxnvt > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1634 = assign26180_e20190;

        let (assign26190_e20200, assign26190_e20200_d_n3, assign26190_e20200_d_n4, assign26190_e20200_d_n5, assign26190_e20200_d_n6, assign26190_e20200_d_n7, assign26190_e20200_d_n8, assign26190_e20200_d_n9, assign26190_e20200_d_n10, assign26190_e20200_d_n11, assign26190_e20200_d_n12,) = {
    if ((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1634 != 0.0)) {
        let assign26190_e20197: f64 = (locals.var_b4soitype * locals.var_here_b4soivth0);
        let assign26190_e20198: f64 = (locals.var_vgs_eff__blk1126 - assign26190_e20197);
        (assign26190_e20198, (locals.var_vgs_eff__blk1126_dn3 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn3)), (locals.var_vgs_eff__blk1126_dn4 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn4)), (locals.var_vgs_eff__blk1126_dn5 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn5)), (locals.var_vgs_eff__blk1126_dn6 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn6)), (locals.var_vgs_eff__blk1126_dn7 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn7)), (locals.var_vgs_eff__blk1126_dn8 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn8)), (locals.var_vgs_eff__blk1126_dn9 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn9)), (locals.var_vgs_eff__blk1126_dn10 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn10)), (locals.var_vgs_eff__blk1126_dn11 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn11)), (locals.var_vgs_eff__blk1126_dn12 - (locals.var_b4soitype * locals.var_here_b4soivth0_dn12)),)
    } else {
        (locals.var_vaux, locals.var_vaux_dn3, locals.var_vaux_dn4, locals.var_vaux_dn5, locals.var_vaux_dn6, locals.var_vaux_dn7, locals.var_vaux_dn8, locals.var_vaux_dn9, locals.var_vaux_dn10, locals.var_vaux_dn11, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign26190_e20200;
        locals.var_vaux_dn3 = assign26190_e20200_d_n3;
        locals.var_vaux_dn4 = assign26190_e20200_d_n4;
        locals.var_vaux_dn5 = assign26190_e20200_d_n5;
        locals.var_vaux_dn6 = assign26190_e20200_d_n6;
        locals.var_vaux_dn7 = assign26190_e20200_d_n7;
        locals.var_vaux_dn8 = assign26190_e20200_d_n8;
        locals.var_vaux_dn9 = assign26190_e20200_d_n9;
        locals.var_vaux_dn10 = assign26190_e20200_d_n10;
        locals.var_vaux_dn11 = assign26190_e20200_d_n11;
        locals.var_vaux_dn12 = assign26190_e20200_d_n12;

        let assign26200_e20203: f64 = (-100.0);
        let assign26200_e20204: f64 = if locals.var_vxnvt < assign26200_e20203 { 1.0 } else { 0.0 };
        locals.var_guard1635 = assign26200_e20204;

        let (assign26210_e20218, assign26210_e20218_d_n3, assign26210_e20218_d_n4, assign26210_e20218_d_n5, assign26210_e20218_d_n6, assign26210_e20218_d_n7, assign26210_e20218_d_n8, assign26210_e20218_d_n9, assign26210_e20218_d_n10, assign26210_e20218_d_n11, assign26210_e20218_d_n12,) = {
    if (((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1634 == 0.0)) && (locals.var_guard1635 != 0.0)) {
        let assign26210_e20214: f64 = (1.0 + 3.720075976e-44);
        let assign26210_e20215: f64 = (assign26210_e20214).ln();
        let assign26210_e20216: f64 = (locals.var_t0__blk1144 * assign26210_e20215);
        (assign26210_e20216, (locals.var_t0__blk1144_dn3 * assign26210_e20215), (locals.var_t0__blk1144_dn4 * assign26210_e20215), (locals.var_t0__blk1144_dn5 * assign26210_e20215), (locals.var_t0__blk1144_dn6 * assign26210_e20215), (locals.var_t0__blk1144_dn7 * assign26210_e20215), (locals.var_t0__blk1144_dn8 * assign26210_e20215), (locals.var_t0__blk1144_dn9 * assign26210_e20215), (locals.var_t0__blk1144_dn10 * assign26210_e20215), (locals.var_t0__blk1144_dn11 * assign26210_e20215), (locals.var_t0__blk1144_dn12 * assign26210_e20215),)
    } else {
        (locals.var_vaux, locals.var_vaux_dn3, locals.var_vaux_dn4, locals.var_vaux_dn5, locals.var_vaux_dn6, locals.var_vaux_dn7, locals.var_vaux_dn8, locals.var_vaux_dn9, locals.var_vaux_dn10, locals.var_vaux_dn11, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign26210_e20218;
        locals.var_vaux_dn3 = assign26210_e20218_d_n3;
        locals.var_vaux_dn4 = assign26210_e20218_d_n4;
        locals.var_vaux_dn5 = assign26210_e20218_d_n5;
        locals.var_vaux_dn6 = assign26210_e20218_d_n6;
        locals.var_vaux_dn7 = assign26210_e20218_d_n7;
        locals.var_vaux_dn8 = assign26210_e20218_d_n8;
        locals.var_vaux_dn9 = assign26210_e20218_d_n9;
        locals.var_vaux_dn10 = assign26210_e20218_d_n10;
        locals.var_vaux_dn11 = assign26210_e20218_d_n11;
        locals.var_vaux_dn12 = assign26210_e20218_d_n12;

        let (assign26220_e20229, assign26220_e20229_d_n3, assign26220_e20229_d_n4, assign26220_e20229_d_n5, assign26220_e20229_d_n6, assign26220_e20229_d_n7, assign26220_e20229_d_n8, assign26220_e20229_d_n9, assign26220_e20229_d_n10, assign26220_e20229_d_n11, assign26220_e20229_d_n12,) = {
    if (((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1634 == 0.0)) && (locals.var_guard1635 == 0.0)) {
        let assign26220_e20227: f64 = (locals.var_vxnvt).exp();
        (assign26220_e20227, (assign26220_e20227 * locals.var_vxnvt_dn3), (assign26220_e20227 * locals.var_vxnvt_dn4), (assign26220_e20227 * locals.var_vxnvt_dn5), (assign26220_e20227 * locals.var_vxnvt_dn6), (assign26220_e20227 * locals.var_vxnvt_dn7), (assign26220_e20227 * locals.var_vxnvt_dn8), (assign26220_e20227 * locals.var_vxnvt_dn9), (assign26220_e20227 * locals.var_vxnvt_dn10), (assign26220_e20227 * locals.var_vxnvt_dn11), (assign26220_e20227 * locals.var_vxnvt_dn12),)
    } else {
        (locals.var_expvxnvt, locals.var_expvxnvt_dn3, locals.var_expvxnvt_dn4, locals.var_expvxnvt_dn5, locals.var_expvxnvt_dn6, locals.var_expvxnvt_dn7, locals.var_expvxnvt_dn8, locals.var_expvxnvt_dn9, locals.var_expvxnvt_dn10, locals.var_expvxnvt_dn11, locals.var_expvxnvt_dn12,)
    }
};
        locals.var_expvxnvt = assign26220_e20229;
        locals.var_expvxnvt_dn3 = assign26220_e20229_d_n3;
        locals.var_expvxnvt_dn4 = assign26220_e20229_d_n4;
        locals.var_expvxnvt_dn5 = assign26220_e20229_d_n5;
        locals.var_expvxnvt_dn6 = assign26220_e20229_d_n6;
        locals.var_expvxnvt_dn7 = assign26220_e20229_d_n7;
        locals.var_expvxnvt_dn8 = assign26220_e20229_d_n8;
        locals.var_expvxnvt_dn9 = assign26220_e20229_d_n9;
        locals.var_expvxnvt_dn10 = assign26220_e20229_d_n10;
        locals.var_expvxnvt_dn11 = assign26220_e20229_d_n11;
        locals.var_expvxnvt_dn12 = assign26220_e20229_d_n12;

        let (assign26230_e20244, assign26230_e20244_d_n3, assign26230_e20244_d_n4, assign26230_e20244_d_n5, assign26230_e20244_d_n6, assign26230_e20244_d_n7, assign26230_e20244_d_n8, assign26230_e20244_d_n9, assign26230_e20244_d_n10, assign26230_e20244_d_n11, assign26230_e20244_d_n12,) = {
    if (((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1634 == 0.0)) && (locals.var_guard1635 == 0.0)) {
        let assign26230_e20240: f64 = (1.0 + locals.var_expvxnvt);
        let assign26230_e20241: f64 = (assign26230_e20240).ln();
        let assign26230_e20242: f64 = (locals.var_t0__blk1144 * assign26230_e20241);
        (assign26230_e20242, ((locals.var_t0__blk1144_dn3 * assign26230_e20241) + (locals.var_t0__blk1144 * (locals.var_expvxnvt_dn3 / assign26230_e20240))), ((locals.var_t0__blk1144_dn4 * assign26230_e20241) + (locals.var_t0__blk1144 * (locals.var_expvxnvt_dn4 / assign26230_e20240))), ((locals.var_t0__blk1144_dn5 * assign26230_e20241) + (locals.var_t0__blk1144 * (locals.var_expvxnvt_dn5 / assign26230_e20240))), ((locals.var_t0__blk1144_dn6 * assign26230_e20241) + (locals.var_t0__blk1144 * (locals.var_expvxnvt_dn6 / assign26230_e20240))), ((locals.var_t0__blk1144_dn7 * assign26230_e20241) + (locals.var_t0__blk1144 * (locals.var_expvxnvt_dn7 / assign26230_e20240))), ((locals.var_t0__blk1144_dn8 * assign26230_e20241) + (locals.var_t0__blk1144 * (locals.var_expvxnvt_dn8 / assign26230_e20240))), ((locals.var_t0__blk1144_dn9 * assign26230_e20241) + (locals.var_t0__blk1144 * (locals.var_expvxnvt_dn9 / assign26230_e20240))), ((locals.var_t0__blk1144_dn10 * assign26230_e20241) + (locals.var_t0__blk1144 * (locals.var_expvxnvt_dn10 / assign26230_e20240))), ((locals.var_t0__blk1144_dn11 * assign26230_e20241) + (locals.var_t0__blk1144 * (locals.var_expvxnvt_dn11 / assign26230_e20240))), ((locals.var_t0__blk1144_dn12 * assign26230_e20241) + (locals.var_t0__blk1144 * (locals.var_expvxnvt_dn12 / assign26230_e20240))),)
    } else {
        (locals.var_vaux, locals.var_vaux_dn3, locals.var_vaux_dn4, locals.var_vaux_dn5, locals.var_vaux_dn6, locals.var_vaux_dn7, locals.var_vaux_dn8, locals.var_vaux_dn9, locals.var_vaux_dn10, locals.var_vaux_dn11, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign26230_e20244;
        locals.var_vaux_dn3 = assign26230_e20244_d_n3;
        locals.var_vaux_dn4 = assign26230_e20244_d_n4;
        locals.var_vaux_dn5 = assign26230_e20244_d_n5;
        locals.var_vaux_dn6 = assign26230_e20244_d_n6;
        locals.var_vaux_dn7 = assign26230_e20244_d_n7;
        locals.var_vaux_dn8 = assign26230_e20244_d_n8;
        locals.var_vaux_dn9 = assign26230_e20244_d_n9;
        locals.var_vaux_dn10 = assign26230_e20244_d_n10;
        locals.var_vaux_dn11 = assign26230_e20244_d_n11;
        locals.var_vaux_dn12 = assign26230_e20244_d_n12;

        let (assign26240_e20250, assign26240_e20250_d_n3, assign26240_e20250_d_n4, assign26240_e20250_d_n5, assign26240_e20250_d_n6, assign26240_e20250_d_n7, assign26240_e20250_d_n8, assign26240_e20250_d_n9, assign26240_e20250_d_n10, assign26240_e20250_d_n11, assign26240_e20250_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26240_e20248: f64 = (locals.var_vgs_eff__blk1126 * locals.var_vaux);
        (assign26240_e20248, ((locals.var_vgs_eff__blk1126_dn3 * locals.var_vaux) + (locals.var_vgs_eff__blk1126 * locals.var_vaux_dn3)), ((locals.var_vgs_eff__blk1126_dn4 * locals.var_vaux) + (locals.var_vgs_eff__blk1126 * locals.var_vaux_dn4)), ((locals.var_vgs_eff__blk1126_dn5 * locals.var_vaux) + (locals.var_vgs_eff__blk1126 * locals.var_vaux_dn5)), ((locals.var_vgs_eff__blk1126_dn6 * locals.var_vaux) + (locals.var_vgs_eff__blk1126 * locals.var_vaux_dn6)), ((locals.var_vgs_eff__blk1126_dn7 * locals.var_vaux) + (locals.var_vgs_eff__blk1126 * locals.var_vaux_dn7)), ((locals.var_vgs_eff__blk1126_dn8 * locals.var_vaux) + (locals.var_vgs_eff__blk1126 * locals.var_vaux_dn8)), ((locals.var_vgs_eff__blk1126_dn9 * locals.var_vaux) + (locals.var_vgs_eff__blk1126 * locals.var_vaux_dn9)), ((locals.var_vgs_eff__blk1126_dn10 * locals.var_vaux) + (locals.var_vgs_eff__blk1126 * locals.var_vaux_dn10)), ((locals.var_vgs_eff__blk1126_dn11 * locals.var_vaux) + (locals.var_vgs_eff__blk1126 * locals.var_vaux_dn11)), ((locals.var_vgs_eff__blk1126_dn12 * locals.var_vaux) + (locals.var_vgs_eff__blk1126 * locals.var_vaux_dn12)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign26240_e20250;
        locals.var_t2__blk1146_dn3 = assign26240_e20250_d_n3;
        locals.var_t2__blk1146_dn4 = assign26240_e20250_d_n4;
        locals.var_t2__blk1146_dn5 = assign26240_e20250_d_n5;
        locals.var_t2__blk1146_dn6 = assign26240_e20250_d_n6;
        locals.var_t2__blk1146_dn7 = assign26240_e20250_d_n7;
        locals.var_t2__blk1146_dn8 = assign26240_e20250_d_n8;
        locals.var_t2__blk1146_dn9 = assign26240_e20250_d_n9;
        locals.var_t2__blk1146_dn10 = assign26240_e20250_d_n10;
        locals.var_t2__blk1146_dn11 = assign26240_e20250_d_n11;
        locals.var_t2__blk1146_dn12 = assign26240_e20250_d_n12;

        let (assign26250_e20254, assign26250_e20254_d_n3, assign26250_e20254_d_n4, assign26250_e20254_d_n5, assign26250_e20254_d_n6, assign26250_e20254_d_n7, assign26250_e20254_d_n8, assign26250_e20254_d_n9, assign26250_e20254_d_n10, assign26250_e20254_d_n11, assign26250_e20254_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        (locals.var_pparam_b4soiaechvb, locals.var_pparam_b4soiaechvb_dn3, locals.var_pparam_b4soiaechvb_dn4, locals.var_pparam_b4soiaechvb_dn5, locals.var_pparam_b4soiaechvb_dn6, locals.var_pparam_b4soiaechvb_dn7, locals.var_pparam_b4soiaechvb_dn8, locals.var_pparam_b4soiaechvb_dn9, locals.var_pparam_b4soiaechvb_dn10, locals.var_pparam_b4soiaechvb_dn11, locals.var_pparam_b4soiaechvb_dn12,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign26250_e20254;
        locals.var_t11_dn3 = assign26250_e20254_d_n3;
        locals.var_t11_dn4 = assign26250_e20254_d_n4;
        locals.var_t11_dn5 = assign26250_e20254_d_n5;
        locals.var_t11_dn6 = assign26250_e20254_d_n6;
        locals.var_t11_dn7 = assign26250_e20254_d_n7;
        locals.var_t11_dn8 = assign26250_e20254_d_n8;
        locals.var_t11_dn9 = assign26250_e20254_d_n9;
        locals.var_t11_dn10 = assign26250_e20254_d_n10;
        locals.var_t11_dn11 = assign26250_e20254_d_n11;
        locals.var_t11_dn12 = assign26250_e20254_d_n12;

        let (assign26260_e20258, assign26260_e20258_d_n3, assign26260_e20258_d_n4, assign26260_e20258_d_n5, assign26260_e20258_d_n6, assign26260_e20258_d_n7, assign26260_e20258_d_n8, assign26260_e20258_d_n9, assign26260_e20258_d_n10, assign26260_e20258_d_n11, assign26260_e20258_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        (locals.var_pparam_b4soibechvb, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign26260_e20258;
        locals.var_t12_dn3 = assign26260_e20258_d_n3;
        locals.var_t12_dn4 = assign26260_e20258_d_n4;
        locals.var_t12_dn5 = assign26260_e20258_d_n5;
        locals.var_t12_dn6 = assign26260_e20258_d_n6;
        locals.var_t12_dn7 = assign26260_e20258_d_n7;
        locals.var_t12_dn8 = assign26260_e20258_d_n8;
        locals.var_t12_dn9 = assign26260_e20258_d_n9;
        locals.var_t12_dn10 = assign26260_e20258_d_n10;
        locals.var_t12_dn11 = assign26260_e20258_d_n11;
        locals.var_t12_dn12 = assign26260_e20258_d_n12;

    }

    pub(super) fn stamp_transient_block_68(
        locals: &mut StampLocals,
    ) {
        let (assign26270_e20266, assign26270_e20266_d_n3, assign26270_e20266_d_n4, assign26270_e20266_d_n5, assign26270_e20266_d_n6, assign26270_e20266_d_n7, assign26270_e20266_d_n8, assign26270_e20266_d_n9, assign26270_e20266_d_n10, assign26270_e20266_d_n11, assign26270_e20266_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26270_e20262: f64 = (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc);
        let assign26270_e20264: f64 = (assign26270_e20262 - locals.var_pparam_b4soibigc);
        (assign26270_e20264, (((locals.var_pparam_b4soiaigc_dn3 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn3)) - locals.var_pparam_b4soibigc_dn3), (((locals.var_pparam_b4soiaigc_dn4 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn4)) - locals.var_pparam_b4soibigc_dn4), (((locals.var_pparam_b4soiaigc_dn5 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn5)) - locals.var_pparam_b4soibigc_dn5), (((locals.var_pparam_b4soiaigc_dn6 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn6)) - locals.var_pparam_b4soibigc_dn6), (((locals.var_pparam_b4soiaigc_dn7 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn7)) - locals.var_pparam_b4soibigc_dn7), (((locals.var_pparam_b4soiaigc_dn8 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn8)) - locals.var_pparam_b4soibigc_dn8), (((locals.var_pparam_b4soiaigc_dn9 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn9)) - locals.var_pparam_b4soibigc_dn9), (((locals.var_pparam_b4soiaigc_dn10 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn10)) - locals.var_pparam_b4soibigc_dn10), (((locals.var_pparam_b4soiaigc_dn11 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn11)) - locals.var_pparam_b4soibigc_dn11), (((locals.var_pparam_b4soiaigc_dn12 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn12)) - locals.var_pparam_b4soibigc_dn12),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign26270_e20266;
        locals.var_t3__blk1147_dn3 = assign26270_e20266_d_n3;
        locals.var_t3__blk1147_dn4 = assign26270_e20266_d_n4;
        locals.var_t3__blk1147_dn5 = assign26270_e20266_d_n5;
        locals.var_t3__blk1147_dn6 = assign26270_e20266_d_n6;
        locals.var_t3__blk1147_dn7 = assign26270_e20266_d_n7;
        locals.var_t3__blk1147_dn8 = assign26270_e20266_d_n8;
        locals.var_t3__blk1147_dn9 = assign26270_e20266_d_n9;
        locals.var_t3__blk1147_dn10 = assign26270_e20266_d_n10;
        locals.var_t3__blk1147_dn11 = assign26270_e20266_d_n11;
        locals.var_t3__blk1147_dn12 = assign26270_e20266_d_n12;

        let (assign26280_e20272, assign26280_e20272_d_n3, assign26280_e20272_d_n4, assign26280_e20272_d_n5, assign26280_e20272_d_n6, assign26280_e20272_d_n7, assign26280_e20272_d_n8, assign26280_e20272_d_n9, assign26280_e20272_d_n10, assign26280_e20272_d_n11, assign26280_e20272_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26280_e20270: f64 = (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc);
        (assign26280_e20270, ((locals.var_pparam_b4soibigc_dn3 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn3)), ((locals.var_pparam_b4soibigc_dn4 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn4)), ((locals.var_pparam_b4soibigc_dn5 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn5)), ((locals.var_pparam_b4soibigc_dn6 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn6)), ((locals.var_pparam_b4soibigc_dn7 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn7)), ((locals.var_pparam_b4soibigc_dn8 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn8)), ((locals.var_pparam_b4soibigc_dn9 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn9)), ((locals.var_pparam_b4soibigc_dn10 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn10)), ((locals.var_pparam_b4soibigc_dn11 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn11)), ((locals.var_pparam_b4soibigc_dn12 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn12)),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign26280_e20272;
        locals.var_t4__blk1148_dn3 = assign26280_e20272_d_n3;
        locals.var_t4__blk1148_dn4 = assign26280_e20272_d_n4;
        locals.var_t4__blk1148_dn5 = assign26280_e20272_d_n5;
        locals.var_t4__blk1148_dn6 = assign26280_e20272_d_n6;
        locals.var_t4__blk1148_dn7 = assign26280_e20272_d_n7;
        locals.var_t4__blk1148_dn8 = assign26280_e20272_d_n8;
        locals.var_t4__blk1148_dn9 = assign26280_e20272_d_n9;
        locals.var_t4__blk1148_dn10 = assign26280_e20272_d_n10;
        locals.var_t4__blk1148_dn11 = assign26280_e20272_d_n11;
        locals.var_t4__blk1148_dn12 = assign26280_e20272_d_n12;

        let (assign26290_e20288, assign26290_e20288_d_n3, assign26290_e20288_d_n4, assign26290_e20288_d_n5, assign26290_e20288_d_n6, assign26290_e20288_d_n7, assign26290_e20288_d_n8, assign26290_e20288_d_n9, assign26290_e20288_d_n10, assign26290_e20288_d_n11, assign26290_e20288_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26290_e20278: f64 = (locals.var_t3__blk1147 * locals.var_voxdepinv);
        let assign26290_e20279: f64 = (locals.var_pparam_b4soiaigc + assign26290_e20278);
        let assign26290_e20282: f64 = (locals.var_t4__blk1148 * locals.var_voxdepinv);
        let assign26290_e20284: f64 = (assign26290_e20282 * locals.var_voxdepinv);
        let assign26290_e20285: f64 = (assign26290_e20279 - assign26290_e20284);
        let assign26290_e20286: f64 = (locals.var_t12 * assign26290_e20285);
        (assign26290_e20286, ((locals.var_t12_dn3 * assign26290_e20285) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn3 + ((locals.var_t3__blk1147_dn3 * locals.var_voxdepinv) + (locals.var_t3__blk1147 * locals.var_voxdepinv_dn3))) - ((((locals.var_t4__blk1148_dn3 * locals.var_voxdepinv) + (locals.var_t4__blk1148 * locals.var_voxdepinv_dn3)) * locals.var_voxdepinv) + (assign26290_e20282 * locals.var_voxdepinv_dn3))))), ((locals.var_t12_dn4 * assign26290_e20285) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn4 + ((locals.var_t3__blk1147_dn4 * locals.var_voxdepinv) + (locals.var_t3__blk1147 * locals.var_voxdepinv_dn4))) - ((((locals.var_t4__blk1148_dn4 * locals.var_voxdepinv) + (locals.var_t4__blk1148 * locals.var_voxdepinv_dn4)) * locals.var_voxdepinv) + (assign26290_e20282 * locals.var_voxdepinv_dn4))))), ((locals.var_t12_dn5 * assign26290_e20285) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn5 + ((locals.var_t3__blk1147_dn5 * locals.var_voxdepinv) + (locals.var_t3__blk1147 * locals.var_voxdepinv_dn5))) - ((((locals.var_t4__blk1148_dn5 * locals.var_voxdepinv) + (locals.var_t4__blk1148 * locals.var_voxdepinv_dn5)) * locals.var_voxdepinv) + (assign26290_e20282 * locals.var_voxdepinv_dn5))))), ((locals.var_t12_dn6 * assign26290_e20285) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn6 + ((locals.var_t3__blk1147_dn6 * locals.var_voxdepinv) + (locals.var_t3__blk1147 * locals.var_voxdepinv_dn6))) - ((((locals.var_t4__blk1148_dn6 * locals.var_voxdepinv) + (locals.var_t4__blk1148 * locals.var_voxdepinv_dn6)) * locals.var_voxdepinv) + (assign26290_e20282 * locals.var_voxdepinv_dn6))))), ((locals.var_t12_dn7 * assign26290_e20285) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn7 + ((locals.var_t3__blk1147_dn7 * locals.var_voxdepinv) + (locals.var_t3__blk1147 * locals.var_voxdepinv_dn7))) - ((((locals.var_t4__blk1148_dn7 * locals.var_voxdepinv) + (locals.var_t4__blk1148 * locals.var_voxdepinv_dn7)) * locals.var_voxdepinv) + (assign26290_e20282 * locals.var_voxdepinv_dn7))))), ((locals.var_t12_dn8 * assign26290_e20285) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn8 + ((locals.var_t3__blk1147_dn8 * locals.var_voxdepinv) + (locals.var_t3__blk1147 * locals.var_voxdepinv_dn8))) - ((((locals.var_t4__blk1148_dn8 * locals.var_voxdepinv) + (locals.var_t4__blk1148 * locals.var_voxdepinv_dn8)) * locals.var_voxdepinv) + (assign26290_e20282 * locals.var_voxdepinv_dn8))))), ((locals.var_t12_dn9 * assign26290_e20285) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn9 + ((locals.var_t3__blk1147_dn9 * locals.var_voxdepinv) + (locals.var_t3__blk1147 * locals.var_voxdepinv_dn9))) - ((((locals.var_t4__blk1148_dn9 * locals.var_voxdepinv) + (locals.var_t4__blk1148 * locals.var_voxdepinv_dn9)) * locals.var_voxdepinv) + (assign26290_e20282 * locals.var_voxdepinv_dn9))))), ((locals.var_t12_dn10 * assign26290_e20285) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn10 + ((locals.var_t3__blk1147_dn10 * locals.var_voxdepinv) + (locals.var_t3__blk1147 * locals.var_voxdepinv_dn10))) - ((((locals.var_t4__blk1148_dn10 * locals.var_voxdepinv) + (locals.var_t4__blk1148 * locals.var_voxdepinv_dn10)) * locals.var_voxdepinv) + (assign26290_e20282 * locals.var_voxdepinv_dn10))))), ((locals.var_t12_dn11 * assign26290_e20285) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn11 + ((locals.var_t3__blk1147_dn11 * locals.var_voxdepinv) + (locals.var_t3__blk1147 * locals.var_voxdepinv_dn11))) - ((((locals.var_t4__blk1148_dn11 * locals.var_voxdepinv) + (locals.var_t4__blk1148 * locals.var_voxdepinv_dn11)) * locals.var_voxdepinv) + (assign26290_e20282 * locals.var_voxdepinv_dn11))))), ((locals.var_t12_dn12 * assign26290_e20285) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn12 + ((locals.var_t3__blk1147_dn12 * locals.var_voxdepinv) + (locals.var_t3__blk1147 * locals.var_voxdepinv_dn12))) - ((((locals.var_t4__blk1148_dn12 * locals.var_voxdepinv) + (locals.var_t4__blk1148 * locals.var_voxdepinv_dn12)) * locals.var_voxdepinv) + (assign26290_e20282 * locals.var_voxdepinv_dn12))))),)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign26290_e20288;
        locals.var_t5__blk1149_dn3 = assign26290_e20288_d_n3;
        locals.var_t5__blk1149_dn4 = assign26290_e20288_d_n4;
        locals.var_t5__blk1149_dn5 = assign26290_e20288_d_n5;
        locals.var_t5__blk1149_dn6 = assign26290_e20288_d_n6;
        locals.var_t5__blk1149_dn7 = assign26290_e20288_d_n7;
        locals.var_t5__blk1149_dn8 = assign26290_e20288_d_n8;
        locals.var_t5__blk1149_dn9 = assign26290_e20288_d_n9;
        locals.var_t5__blk1149_dn10 = assign26290_e20288_d_n10;
        locals.var_t5__blk1149_dn11 = assign26290_e20288_d_n11;
        locals.var_t5__blk1149_dn12 = assign26290_e20288_d_n12;

        let assign26300_e20291: f64 = if locals.var_t5__blk1149 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1636 = assign26300_e20291;

        let (assign26310_e20297, assign26310_e20297_d_n3, assign26310_e20297_d_n4, assign26310_e20297_d_n5, assign26310_e20297_d_n6, assign26310_e20297_d_n7, assign26310_e20297_d_n8, assign26310_e20297_d_n9, assign26310_e20297_d_n10, assign26310_e20297_d_n11, assign26310_e20297_d_n12,) = {
    if ((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1636 != 0.0)) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign26310_e20297;
        locals.var_t6__blk1150_dn3 = assign26310_e20297_d_n3;
        locals.var_t6__blk1150_dn4 = assign26310_e20297_d_n4;
        locals.var_t6__blk1150_dn5 = assign26310_e20297_d_n5;
        locals.var_t6__blk1150_dn6 = assign26310_e20297_d_n6;
        locals.var_t6__blk1150_dn7 = assign26310_e20297_d_n7;
        locals.var_t6__blk1150_dn8 = assign26310_e20297_d_n8;
        locals.var_t6__blk1150_dn9 = assign26310_e20297_d_n9;
        locals.var_t6__blk1150_dn10 = assign26310_e20297_d_n10;
        locals.var_t6__blk1150_dn11 = assign26310_e20297_d_n11;
        locals.var_t6__blk1150_dn12 = assign26310_e20297_d_n12;

        let assign26320_e20300: f64 = (-100.0);
        let assign26320_e20301: f64 = if locals.var_t5__blk1149 < assign26320_e20300 { 1.0 } else { 0.0 };
        locals.var_guard1637 = assign26320_e20301;

        let (assign26330_e20310, assign26330_e20310_d_n3, assign26330_e20310_d_n4, assign26330_e20310_d_n5, assign26330_e20310_d_n6, assign26330_e20310_d_n7, assign26330_e20310_d_n8, assign26330_e20310_d_n9, assign26330_e20310_d_n10, assign26330_e20310_d_n11, assign26330_e20310_d_n12,) = {
    if (((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1636 == 0.0)) && (locals.var_guard1637 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign26330_e20310;
        locals.var_t6__blk1150_dn3 = assign26330_e20310_d_n3;
        locals.var_t6__blk1150_dn4 = assign26330_e20310_d_n4;
        locals.var_t6__blk1150_dn5 = assign26330_e20310_d_n5;
        locals.var_t6__blk1150_dn6 = assign26330_e20310_d_n6;
        locals.var_t6__blk1150_dn7 = assign26330_e20310_d_n7;
        locals.var_t6__blk1150_dn8 = assign26330_e20310_d_n8;
        locals.var_t6__blk1150_dn9 = assign26330_e20310_d_n9;
        locals.var_t6__blk1150_dn10 = assign26330_e20310_d_n10;
        locals.var_t6__blk1150_dn11 = assign26330_e20310_d_n11;
        locals.var_t6__blk1150_dn12 = assign26330_e20310_d_n12;

        let (assign26340_e20321, assign26340_e20321_d_n3, assign26340_e20321_d_n4, assign26340_e20321_d_n5, assign26340_e20321_d_n6, assign26340_e20321_d_n7, assign26340_e20321_d_n8, assign26340_e20321_d_n9, assign26340_e20321_d_n10, assign26340_e20321_d_n11, assign26340_e20321_d_n12,) = {
    if (((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1636 == 0.0)) && (locals.var_guard1637 == 0.0)) {
        let assign26340_e20319: f64 = (locals.var_t5__blk1149).exp();
        (assign26340_e20319, (assign26340_e20319 * locals.var_t5__blk1149_dn3), (assign26340_e20319 * locals.var_t5__blk1149_dn4), (assign26340_e20319 * locals.var_t5__blk1149_dn5), (assign26340_e20319 * locals.var_t5__blk1149_dn6), (assign26340_e20319 * locals.var_t5__blk1149_dn7), (assign26340_e20319 * locals.var_t5__blk1149_dn8), (assign26340_e20319 * locals.var_t5__blk1149_dn9), (assign26340_e20319 * locals.var_t5__blk1149_dn10), (assign26340_e20319 * locals.var_t5__blk1149_dn11), (assign26340_e20319 * locals.var_t5__blk1149_dn12),)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign26340_e20321;
        locals.var_t6__blk1150_dn3 = assign26340_e20321_d_n3;
        locals.var_t6__blk1150_dn4 = assign26340_e20321_d_n4;
        locals.var_t6__blk1150_dn5 = assign26340_e20321_d_n5;
        locals.var_t6__blk1150_dn6 = assign26340_e20321_d_n6;
        locals.var_t6__blk1150_dn7 = assign26340_e20321_d_n7;
        locals.var_t6__blk1150_dn8 = assign26340_e20321_d_n8;
        locals.var_t6__blk1150_dn9 = assign26340_e20321_d_n9;
        locals.var_t6__blk1150_dn10 = assign26340_e20321_d_n10;
        locals.var_t6__blk1150_dn11 = assign26340_e20321_d_n11;
        locals.var_t6__blk1150_dn12 = assign26340_e20321_d_n12;

        let (assign26350_e20329, assign26350_e20329_d_n3, assign26350_e20329_d_n4, assign26350_e20329_d_n5, assign26350_e20329_d_n6, assign26350_e20329_d_n7, assign26350_e20329_d_n8, assign26350_e20329_d_n9, assign26350_e20329_d_n10, assign26350_e20329_d_n11, assign26350_e20329_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26350_e20325: f64 = (locals.var_t11 * locals.var_t2__blk1146);
        let assign26350_e20327: f64 = (assign26350_e20325 * locals.var_t6__blk1150);
        (assign26350_e20327, ((((locals.var_t11_dn3 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn3)) * locals.var_t6__blk1150) + (assign26350_e20325 * locals.var_t6__blk1150_dn3)), ((((locals.var_t11_dn4 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn4)) * locals.var_t6__blk1150) + (assign26350_e20325 * locals.var_t6__blk1150_dn4)), ((((locals.var_t11_dn5 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn5)) * locals.var_t6__blk1150) + (assign26350_e20325 * locals.var_t6__blk1150_dn5)), ((((locals.var_t11_dn6 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn6)) * locals.var_t6__blk1150) + (assign26350_e20325 * locals.var_t6__blk1150_dn6)), ((((locals.var_t11_dn7 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn7)) * locals.var_t6__blk1150) + (assign26350_e20325 * locals.var_t6__blk1150_dn7)), ((((locals.var_t11_dn8 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn8)) * locals.var_t6__blk1150) + (assign26350_e20325 * locals.var_t6__blk1150_dn8)), ((((locals.var_t11_dn9 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn9)) * locals.var_t6__blk1150) + (assign26350_e20325 * locals.var_t6__blk1150_dn9)), ((((locals.var_t11_dn10 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn10)) * locals.var_t6__blk1150) + (assign26350_e20325 * locals.var_t6__blk1150_dn10)), ((((locals.var_t11_dn11 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn11)) * locals.var_t6__blk1150) + (assign26350_e20325 * locals.var_t6__blk1150_dn11)), ((((locals.var_t11_dn12 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn12)) * locals.var_t6__blk1150) + (assign26350_e20325 * locals.var_t6__blk1150_dn12)),)
    } else {
        (locals.var_igc, locals.var_igc_dn3, locals.var_igc_dn4, locals.var_igc_dn5, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8, locals.var_igc_dn9, locals.var_igc_dn10, locals.var_igc_dn11, locals.var_igc_dn12,)
    }
};
        locals.var_igc = assign26350_e20329;
        locals.var_igc_dn3 = assign26350_e20329_d_n3;
        locals.var_igc_dn4 = assign26350_e20329_d_n4;
        locals.var_igc_dn5 = assign26350_e20329_d_n5;
        locals.var_igc_dn6 = assign26350_e20329_d_n6;
        locals.var_igc_dn7 = assign26350_e20329_d_n7;
        locals.var_igc_dn8 = assign26350_e20329_d_n8;
        locals.var_igc_dn9 = assign26350_e20329_d_n9;
        locals.var_igc_dn10 = assign26350_e20329_d_n10;
        locals.var_igc_dn11 = assign26350_e20329_d_n11;
        locals.var_igc_dn12 = assign26350_e20329_d_n12;

        let (assign26360_e20336, assign26360_e20336_d_n3, assign26360_e20336_d_n4, assign26360_e20336_d_n5, assign26360_e20336_d_n6, assign26360_e20336_d_n7, assign26360_e20336_d_n8, assign26360_e20336_d_n9, assign26360_e20336_d_n10, assign26360_e20336_d_n11, assign26360_e20336_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26360_e20332: f64 = (-locals.var_pparam_b4soipigcd);
        let assign26360_e20334: f64 = (assign26360_e20332 * locals.var_vds_1);
        (assign26360_e20334, ((-locals.var_pparam_b4soipigcd_dn3) * locals.var_vds_1), ((-locals.var_pparam_b4soipigcd_dn4) * locals.var_vds_1), ((-locals.var_pparam_b4soipigcd_dn5) * locals.var_vds_1), ((-locals.var_pparam_b4soipigcd_dn6) * locals.var_vds_1), (((-locals.var_pparam_b4soipigcd_dn7) * locals.var_vds_1) + (assign26360_e20332 * locals.var_vds_1_dn7)), (((-locals.var_pparam_b4soipigcd_dn8) * locals.var_vds_1) + (assign26360_e20332 * locals.var_vds_1_dn8)), ((-locals.var_pparam_b4soipigcd_dn9) * locals.var_vds_1), ((-locals.var_pparam_b4soipigcd_dn10) * locals.var_vds_1), ((-locals.var_pparam_b4soipigcd_dn11) * locals.var_vds_1), ((-locals.var_pparam_b4soipigcd_dn12) * locals.var_vds_1),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign26360_e20336;
        locals.var_t7__blk1151_dn3 = assign26360_e20336_d_n3;
        locals.var_t7__blk1151_dn4 = assign26360_e20336_d_n4;
        locals.var_t7__blk1151_dn5 = assign26360_e20336_d_n5;
        locals.var_t7__blk1151_dn6 = assign26360_e20336_d_n6;
        locals.var_t7__blk1151_dn7 = assign26360_e20336_d_n7;
        locals.var_t7__blk1151_dn8 = assign26360_e20336_d_n8;
        locals.var_t7__blk1151_dn9 = assign26360_e20336_d_n9;
        locals.var_t7__blk1151_dn10 = assign26360_e20336_d_n10;
        locals.var_t7__blk1151_dn11 = assign26360_e20336_d_n11;
        locals.var_t7__blk1151_dn12 = assign26360_e20336_d_n12;

        let (assign26370_e20344, assign26370_e20344_d_n3, assign26370_e20344_d_n4, assign26370_e20344_d_n5, assign26370_e20344_d_n6, assign26370_e20344_d_n7, assign26370_e20344_d_n8, assign26370_e20344_d_n9, assign26370_e20344_d_n10, assign26370_e20344_d_n11, assign26370_e20344_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26370_e20340: f64 = (locals.var_t7__blk1151 * locals.var_t7__blk1151);
        let assign26370_e20342: f64 = (assign26370_e20340 + 0.0002);
        (assign26370_e20342, ((locals.var_t7__blk1151_dn3 * locals.var_t7__blk1151) + (locals.var_t7__blk1151 * locals.var_t7__blk1151_dn3)), ((locals.var_t7__blk1151_dn4 * locals.var_t7__blk1151) + (locals.var_t7__blk1151 * locals.var_t7__blk1151_dn4)), ((locals.var_t7__blk1151_dn5 * locals.var_t7__blk1151) + (locals.var_t7__blk1151 * locals.var_t7__blk1151_dn5)), ((locals.var_t7__blk1151_dn6 * locals.var_t7__blk1151) + (locals.var_t7__blk1151 * locals.var_t7__blk1151_dn6)), ((locals.var_t7__blk1151_dn7 * locals.var_t7__blk1151) + (locals.var_t7__blk1151 * locals.var_t7__blk1151_dn7)), ((locals.var_t7__blk1151_dn8 * locals.var_t7__blk1151) + (locals.var_t7__blk1151 * locals.var_t7__blk1151_dn8)), ((locals.var_t7__blk1151_dn9 * locals.var_t7__blk1151) + (locals.var_t7__blk1151 * locals.var_t7__blk1151_dn9)), ((locals.var_t7__blk1151_dn10 * locals.var_t7__blk1151) + (locals.var_t7__blk1151 * locals.var_t7__blk1151_dn10)), ((locals.var_t7__blk1151_dn11 * locals.var_t7__blk1151) + (locals.var_t7__blk1151 * locals.var_t7__blk1151_dn11)), ((locals.var_t7__blk1151_dn12 * locals.var_t7__blk1151) + (locals.var_t7__blk1151 * locals.var_t7__blk1151_dn12)),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12,)
    }
};
        locals.var_t8 = assign26370_e20344;
        locals.var_t8_dn3 = assign26370_e20344_d_n3;
        locals.var_t8_dn4 = assign26370_e20344_d_n4;
        locals.var_t8_dn5 = assign26370_e20344_d_n5;
        locals.var_t8_dn6 = assign26370_e20344_d_n6;
        locals.var_t8_dn7 = assign26370_e20344_d_n7;
        locals.var_t8_dn8 = assign26370_e20344_d_n8;
        locals.var_t8_dn9 = assign26370_e20344_d_n9;
        locals.var_t8_dn10 = assign26370_e20344_d_n10;
        locals.var_t8_dn11 = assign26370_e20344_d_n11;
        locals.var_t8_dn12 = assign26370_e20344_d_n12;

        let assign26380_e20347: f64 = if locals.var_t7__blk1151 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1638 = assign26380_e20347;

        let (assign26390_e20353, assign26390_e20353_d_n3, assign26390_e20353_d_n4, assign26390_e20353_d_n5, assign26390_e20353_d_n6, assign26390_e20353_d_n7, assign26390_e20353_d_n8, assign26390_e20353_d_n9, assign26390_e20353_d_n10, assign26390_e20353_d_n11, assign26390_e20353_d_n12,) = {
    if ((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1638 != 0.0)) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign26390_e20353;
        locals.var_t9_dn3 = assign26390_e20353_d_n3;
        locals.var_t9_dn4 = assign26390_e20353_d_n4;
        locals.var_t9_dn5 = assign26390_e20353_d_n5;
        locals.var_t9_dn6 = assign26390_e20353_d_n6;
        locals.var_t9_dn7 = assign26390_e20353_d_n7;
        locals.var_t9_dn8 = assign26390_e20353_d_n8;
        locals.var_t9_dn9 = assign26390_e20353_d_n9;
        locals.var_t9_dn10 = assign26390_e20353_d_n10;
        locals.var_t9_dn11 = assign26390_e20353_d_n11;
        locals.var_t9_dn12 = assign26390_e20353_d_n12;

        let assign26400_e20356: f64 = (-100.0);
        let assign26400_e20357: f64 = if locals.var_t7__blk1151 < assign26400_e20356 { 1.0 } else { 0.0 };
        locals.var_guard1639 = assign26400_e20357;

        let (assign26410_e20366, assign26410_e20366_d_n3, assign26410_e20366_d_n4, assign26410_e20366_d_n5, assign26410_e20366_d_n6, assign26410_e20366_d_n7, assign26410_e20366_d_n8, assign26410_e20366_d_n9, assign26410_e20366_d_n10, assign26410_e20366_d_n11, assign26410_e20366_d_n12,) = {
    if (((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1638 == 0.0)) && (locals.var_guard1639 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign26410_e20366;
        locals.var_t9_dn3 = assign26410_e20366_d_n3;
        locals.var_t9_dn4 = assign26410_e20366_d_n4;
        locals.var_t9_dn5 = assign26410_e20366_d_n5;
        locals.var_t9_dn6 = assign26410_e20366_d_n6;
        locals.var_t9_dn7 = assign26410_e20366_d_n7;
        locals.var_t9_dn8 = assign26410_e20366_d_n8;
        locals.var_t9_dn9 = assign26410_e20366_d_n9;
        locals.var_t9_dn10 = assign26410_e20366_d_n10;
        locals.var_t9_dn11 = assign26410_e20366_d_n11;
        locals.var_t9_dn12 = assign26410_e20366_d_n12;

        let (assign26420_e20377, assign26420_e20377_d_n3, assign26420_e20377_d_n4, assign26420_e20377_d_n5, assign26420_e20377_d_n6, assign26420_e20377_d_n7, assign26420_e20377_d_n8, assign26420_e20377_d_n9, assign26420_e20377_d_n10, assign26420_e20377_d_n11, assign26420_e20377_d_n12,) = {
    if (((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1638 == 0.0)) && (locals.var_guard1639 == 0.0)) {
        let assign26420_e20375: f64 = (locals.var_t7__blk1151).exp();
        (assign26420_e20375, (assign26420_e20375 * locals.var_t7__blk1151_dn3), (assign26420_e20375 * locals.var_t7__blk1151_dn4), (assign26420_e20375 * locals.var_t7__blk1151_dn5), (assign26420_e20375 * locals.var_t7__blk1151_dn6), (assign26420_e20375 * locals.var_t7__blk1151_dn7), (assign26420_e20375 * locals.var_t7__blk1151_dn8), (assign26420_e20375 * locals.var_t7__blk1151_dn9), (assign26420_e20375 * locals.var_t7__blk1151_dn10), (assign26420_e20375 * locals.var_t7__blk1151_dn11), (assign26420_e20375 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign26420_e20377;
        locals.var_t9_dn3 = assign26420_e20377_d_n3;
        locals.var_t9_dn4 = assign26420_e20377_d_n4;
        locals.var_t9_dn5 = assign26420_e20377_d_n5;
        locals.var_t9_dn6 = assign26420_e20377_d_n6;
        locals.var_t9_dn7 = assign26420_e20377_d_n7;
        locals.var_t9_dn8 = assign26420_e20377_d_n8;
        locals.var_t9_dn9 = assign26420_e20377_d_n9;
        locals.var_t9_dn10 = assign26420_e20377_d_n10;
        locals.var_t9_dn11 = assign26420_e20377_d_n11;
        locals.var_t9_dn12 = assign26420_e20377_d_n12;

        let (assign26430_e20385, assign26430_e20385_d_n3, assign26430_e20385_d_n4, assign26430_e20385_d_n5, assign26430_e20385_d_n6, assign26430_e20385_d_n7, assign26430_e20385_d_n8, assign26430_e20385_d_n9, assign26430_e20385_d_n10, assign26430_e20385_d_n11, assign26430_e20385_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26430_e20381: f64 = (locals.var_t9 - 1.0);
        let assign26430_e20383: f64 = (assign26430_e20381 + 0.0001);
        (assign26430_e20383, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign26430_e20385;
        locals.var_t1__blk1145_dn3 = assign26430_e20385_d_n3;
        locals.var_t1__blk1145_dn4 = assign26430_e20385_d_n4;
        locals.var_t1__blk1145_dn5 = assign26430_e20385_d_n5;
        locals.var_t1__blk1145_dn6 = assign26430_e20385_d_n6;
        locals.var_t1__blk1145_dn7 = assign26430_e20385_d_n7;
        locals.var_t1__blk1145_dn8 = assign26430_e20385_d_n8;
        locals.var_t1__blk1145_dn9 = assign26430_e20385_d_n9;
        locals.var_t1__blk1145_dn10 = assign26430_e20385_d_n10;
        locals.var_t1__blk1145_dn11 = assign26430_e20385_d_n11;
        locals.var_t1__blk1145_dn12 = assign26430_e20385_d_n12;

        let (assign26440_e20393, assign26440_e20393_d_n3, assign26440_e20393_d_n4, assign26440_e20393_d_n5, assign26440_e20393_d_n6, assign26440_e20393_d_n7, assign26440_e20393_d_n8, assign26440_e20393_d_n9, assign26440_e20393_d_n10, assign26440_e20393_d_n11, assign26440_e20393_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26440_e20389: f64 = (locals.var_t1__blk1145 - locals.var_t7__blk1151);
        let assign26440_e20391: f64 = (assign26440_e20389 / locals.var_t8);
        (assign26440_e20391, ((((locals.var_t1__blk1145_dn3 - locals.var_t7__blk1151_dn3) * locals.var_t8) - (assign26440_e20389 * locals.var_t8_dn3)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk1145_dn4 - locals.var_t7__blk1151_dn4) * locals.var_t8) - (assign26440_e20389 * locals.var_t8_dn4)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk1145_dn5 - locals.var_t7__blk1151_dn5) * locals.var_t8) - (assign26440_e20389 * locals.var_t8_dn5)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk1145_dn6 - locals.var_t7__blk1151_dn6) * locals.var_t8) - (assign26440_e20389 * locals.var_t8_dn6)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk1145_dn7 - locals.var_t7__blk1151_dn7) * locals.var_t8) - (assign26440_e20389 * locals.var_t8_dn7)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk1145_dn8 - locals.var_t7__blk1151_dn8) * locals.var_t8) - (assign26440_e20389 * locals.var_t8_dn8)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk1145_dn9 - locals.var_t7__blk1151_dn9) * locals.var_t8) - (assign26440_e20389 * locals.var_t8_dn9)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk1145_dn10 - locals.var_t7__blk1151_dn10) * locals.var_t8) - (assign26440_e20389 * locals.var_t8_dn10)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk1145_dn11 - locals.var_t7__blk1151_dn11) * locals.var_t8) - (assign26440_e20389 * locals.var_t8_dn11)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk1145_dn12 - locals.var_t7__blk1151_dn12) * locals.var_t8) - (assign26440_e20389 * locals.var_t8_dn12)) / (locals.var_t8 * locals.var_t8)),)
    } else {
        (locals.var_t10__blk1154, locals.var_t10__blk1154_dn3, locals.var_t10__blk1154_dn4, locals.var_t10__blk1154_dn5, locals.var_t10__blk1154_dn6, locals.var_t10__blk1154_dn7, locals.var_t10__blk1154_dn8, locals.var_t10__blk1154_dn9, locals.var_t10__blk1154_dn10, locals.var_t10__blk1154_dn11, locals.var_t10__blk1154_dn12,)
    }
};
        locals.var_t10__blk1154 = assign26440_e20393;
        locals.var_t10__blk1154_dn3 = assign26440_e20393_d_n3;
        locals.var_t10__blk1154_dn4 = assign26440_e20393_d_n4;
        locals.var_t10__blk1154_dn5 = assign26440_e20393_d_n5;
        locals.var_t10__blk1154_dn6 = assign26440_e20393_d_n6;
        locals.var_t10__blk1154_dn7 = assign26440_e20393_d_n7;
        locals.var_t10__blk1154_dn8 = assign26440_e20393_d_n8;
        locals.var_t10__blk1154_dn9 = assign26440_e20393_d_n9;
        locals.var_t10__blk1154_dn10 = assign26440_e20393_d_n10;
        locals.var_t10__blk1154_dn11 = assign26440_e20393_d_n11;
        locals.var_t10__blk1154_dn12 = assign26440_e20393_d_n12;

        let (assign26450_e20399, assign26450_e20399_d_n3, assign26450_e20399_d_n4, assign26450_e20399_d_n5, assign26450_e20399_d_n6, assign26450_e20399_d_n7, assign26450_e20399_d_n8, assign26450_e20399_d_n9, assign26450_e20399_d_n10, assign26450_e20399_d_n11, assign26450_e20399_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26450_e20397: f64 = (locals.var_igc * locals.var_t10__blk1154);
        (assign26450_e20397, ((locals.var_igc_dn3 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn3)), ((locals.var_igc_dn4 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn4)), ((locals.var_igc_dn5 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn5)), ((locals.var_igc_dn6 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn6)), ((locals.var_igc_dn7 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn7)), ((locals.var_igc_dn8 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn8)), ((locals.var_igc_dn9 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn9)), ((locals.var_igc_dn10 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn10)), ((locals.var_igc_dn11 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn11)), ((locals.var_igc_dn12 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn12)),)
    } else {
        (locals.var_igcs_1, locals.var_igcs_1_dn3, locals.var_igcs_1_dn4, locals.var_igcs_1_dn5, locals.var_igcs_1_dn6, locals.var_igcs_1_dn7, locals.var_igcs_1_dn8, locals.var_igcs_1_dn9, locals.var_igcs_1_dn10, locals.var_igcs_1_dn11, locals.var_igcs_1_dn12,)
    }
};
        locals.var_igcs_1 = assign26450_e20399;
        locals.var_igcs_1_dn3 = assign26450_e20399_d_n3;
        locals.var_igcs_1_dn4 = assign26450_e20399_d_n4;
        locals.var_igcs_1_dn5 = assign26450_e20399_d_n5;
        locals.var_igcs_1_dn6 = assign26450_e20399_d_n6;
        locals.var_igcs_1_dn7 = assign26450_e20399_d_n7;
        locals.var_igcs_1_dn8 = assign26450_e20399_d_n8;
        locals.var_igcs_1_dn9 = assign26450_e20399_d_n9;
        locals.var_igcs_1_dn10 = assign26450_e20399_d_n10;
        locals.var_igcs_1_dn11 = assign26450_e20399_d_n11;
        locals.var_igcs_1_dn12 = assign26450_e20399_d_n12;

        let (assign26460_e20407, assign26460_e20407_d_n3, assign26460_e20407_d_n4, assign26460_e20407_d_n5, assign26460_e20407_d_n6, assign26460_e20407_d_n7, assign26460_e20407_d_n8, assign26460_e20407_d_n9, assign26460_e20407_d_n10, assign26460_e20407_d_n11, assign26460_e20407_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26460_e20403: f64 = (locals.var_t9 - 1.0);
        let assign26460_e20405: f64 = (assign26460_e20403 - 0.0001);
        (assign26460_e20405, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign26460_e20407;
        locals.var_t1__blk1145_dn3 = assign26460_e20407_d_n3;
        locals.var_t1__blk1145_dn4 = assign26460_e20407_d_n4;
        locals.var_t1__blk1145_dn5 = assign26460_e20407_d_n5;
        locals.var_t1__blk1145_dn6 = assign26460_e20407_d_n6;
        locals.var_t1__blk1145_dn7 = assign26460_e20407_d_n7;
        locals.var_t1__blk1145_dn8 = assign26460_e20407_d_n8;
        locals.var_t1__blk1145_dn9 = assign26460_e20407_d_n9;
        locals.var_t1__blk1145_dn10 = assign26460_e20407_d_n10;
        locals.var_t1__blk1145_dn11 = assign26460_e20407_d_n11;
        locals.var_t1__blk1145_dn12 = assign26460_e20407_d_n12;

        let (assign26470_e20417, assign26470_e20417_d_n3, assign26470_e20417_d_n4, assign26470_e20417_d_n5, assign26470_e20417_d_n6, assign26470_e20417_d_n7, assign26470_e20417_d_n8, assign26470_e20417_d_n9, assign26470_e20417_d_n10, assign26470_e20417_d_n11, assign26470_e20417_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26470_e20411: f64 = (locals.var_t7__blk1151 * locals.var_t9);
        let assign26470_e20413: f64 = (assign26470_e20411 - locals.var_t1__blk1145);
        let assign26470_e20415: f64 = (assign26470_e20413 / locals.var_t8);
        (assign26470_e20415, ((((((locals.var_t7__blk1151_dn3 * locals.var_t9) + (locals.var_t7__blk1151 * locals.var_t9_dn3)) - locals.var_t1__blk1145_dn3) * locals.var_t8) - (assign26470_e20413 * locals.var_t8_dn3)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk1151_dn4 * locals.var_t9) + (locals.var_t7__blk1151 * locals.var_t9_dn4)) - locals.var_t1__blk1145_dn4) * locals.var_t8) - (assign26470_e20413 * locals.var_t8_dn4)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk1151_dn5 * locals.var_t9) + (locals.var_t7__blk1151 * locals.var_t9_dn5)) - locals.var_t1__blk1145_dn5) * locals.var_t8) - (assign26470_e20413 * locals.var_t8_dn5)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk1151_dn6 * locals.var_t9) + (locals.var_t7__blk1151 * locals.var_t9_dn6)) - locals.var_t1__blk1145_dn6) * locals.var_t8) - (assign26470_e20413 * locals.var_t8_dn6)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk1151_dn7 * locals.var_t9) + (locals.var_t7__blk1151 * locals.var_t9_dn7)) - locals.var_t1__blk1145_dn7) * locals.var_t8) - (assign26470_e20413 * locals.var_t8_dn7)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk1151_dn8 * locals.var_t9) + (locals.var_t7__blk1151 * locals.var_t9_dn8)) - locals.var_t1__blk1145_dn8) * locals.var_t8) - (assign26470_e20413 * locals.var_t8_dn8)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk1151_dn9 * locals.var_t9) + (locals.var_t7__blk1151 * locals.var_t9_dn9)) - locals.var_t1__blk1145_dn9) * locals.var_t8) - (assign26470_e20413 * locals.var_t8_dn9)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk1151_dn10 * locals.var_t9) + (locals.var_t7__blk1151 * locals.var_t9_dn10)) - locals.var_t1__blk1145_dn10) * locals.var_t8) - (assign26470_e20413 * locals.var_t8_dn10)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk1151_dn11 * locals.var_t9) + (locals.var_t7__blk1151 * locals.var_t9_dn11)) - locals.var_t1__blk1145_dn11) * locals.var_t8) - (assign26470_e20413 * locals.var_t8_dn11)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk1151_dn12 * locals.var_t9) + (locals.var_t7__blk1151 * locals.var_t9_dn12)) - locals.var_t1__blk1145_dn12) * locals.var_t8) - (assign26470_e20413 * locals.var_t8_dn12)) / (locals.var_t8 * locals.var_t8)),)
    } else {
        (locals.var_t10__blk1154, locals.var_t10__blk1154_dn3, locals.var_t10__blk1154_dn4, locals.var_t10__blk1154_dn5, locals.var_t10__blk1154_dn6, locals.var_t10__blk1154_dn7, locals.var_t10__blk1154_dn8, locals.var_t10__blk1154_dn9, locals.var_t10__blk1154_dn10, locals.var_t10__blk1154_dn11, locals.var_t10__blk1154_dn12,)
    }
};
        locals.var_t10__blk1154 = assign26470_e20417;
        locals.var_t10__blk1154_dn3 = assign26470_e20417_d_n3;
        locals.var_t10__blk1154_dn4 = assign26470_e20417_d_n4;
        locals.var_t10__blk1154_dn5 = assign26470_e20417_d_n5;
        locals.var_t10__blk1154_dn6 = assign26470_e20417_d_n6;
        locals.var_t10__blk1154_dn7 = assign26470_e20417_d_n7;
        locals.var_t10__blk1154_dn8 = assign26470_e20417_d_n8;
        locals.var_t10__blk1154_dn9 = assign26470_e20417_d_n9;
        locals.var_t10__blk1154_dn10 = assign26470_e20417_d_n10;
        locals.var_t10__blk1154_dn11 = assign26470_e20417_d_n11;
        locals.var_t10__blk1154_dn12 = assign26470_e20417_d_n12;

        let (assign26480_e20423, assign26480_e20423_d_n3, assign26480_e20423_d_n4, assign26480_e20423_d_n5, assign26480_e20423_d_n6, assign26480_e20423_d_n7, assign26480_e20423_d_n8, assign26480_e20423_d_n9, assign26480_e20423_d_n10, assign26480_e20423_d_n11, assign26480_e20423_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26480_e20421: f64 = (locals.var_igc * locals.var_t10__blk1154);
        (assign26480_e20421, ((locals.var_igc_dn3 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn3)), ((locals.var_igc_dn4 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn4)), ((locals.var_igc_dn5 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn5)), ((locals.var_igc_dn6 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn6)), ((locals.var_igc_dn7 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn7)), ((locals.var_igc_dn8 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn8)), ((locals.var_igc_dn9 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn9)), ((locals.var_igc_dn10 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn10)), ((locals.var_igc_dn11 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn11)), ((locals.var_igc_dn12 * locals.var_t10__blk1154) + (locals.var_igc * locals.var_t10__blk1154_dn12)),)
    } else {
        (locals.var_igcd_1, locals.var_igcd_1_dn3, locals.var_igcd_1_dn4, locals.var_igcd_1_dn5, locals.var_igcd_1_dn6, locals.var_igcd_1_dn7, locals.var_igcd_1_dn8, locals.var_igcd_1_dn9, locals.var_igcd_1_dn10, locals.var_igcd_1_dn11, locals.var_igcd_1_dn12,)
    }
};
        locals.var_igcd_1 = assign26480_e20423;
        locals.var_igcd_1_dn3 = assign26480_e20423_d_n3;
        locals.var_igcd_1_dn4 = assign26480_e20423_d_n4;
        locals.var_igcd_1_dn5 = assign26480_e20423_d_n5;
        locals.var_igcd_1_dn6 = assign26480_e20423_d_n6;
        locals.var_igcd_1_dn7 = assign26480_e20423_d_n7;
        locals.var_igcd_1_dn8 = assign26480_e20423_d_n8;
        locals.var_igcd_1_dn9 = assign26480_e20423_d_n9;
        locals.var_igcd_1_dn10 = assign26480_e20423_d_n10;
        locals.var_igcd_1_dn11 = assign26480_e20423_d_n11;
        locals.var_igcd_1_dn12 = assign26480_e20423_d_n12;

        let (assign26490_e20429, assign26490_e20429_d_n3, assign26490_e20429_d_n4, assign26490_e20429_d_n5, assign26490_e20429_d_n6, assign26490_e20429_d_n7, assign26490_e20429_d_n8, assign26490_e20429_d_n9, assign26490_e20429_d_n10, assign26490_e20429_d_n11, assign26490_e20429_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26490_e20427: f64 = (locals.var_vgs - locals.var_pparam_b4soivfbsd);
        (assign26490_e20427, (-locals.var_pparam_b4soivfbsd_dn3), (-locals.var_pparam_b4soivfbsd_dn4), (-locals.var_pparam_b4soivfbsd_dn5), (-locals.var_pparam_b4soivfbsd_dn6), (-locals.var_pparam_b4soivfbsd_dn7), (locals.var_vgs_dn8 - locals.var_pparam_b4soivfbsd_dn8), (locals.var_vgs_dn9 - locals.var_pparam_b4soivfbsd_dn9), (-locals.var_pparam_b4soivfbsd_dn10), (-locals.var_pparam_b4soivfbsd_dn11), (-locals.var_pparam_b4soivfbsd_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign26490_e20429;
        locals.var_t0__blk1144_dn3 = assign26490_e20429_d_n3;
        locals.var_t0__blk1144_dn4 = assign26490_e20429_d_n4;
        locals.var_t0__blk1144_dn5 = assign26490_e20429_d_n5;
        locals.var_t0__blk1144_dn6 = assign26490_e20429_d_n6;
        locals.var_t0__blk1144_dn7 = assign26490_e20429_d_n7;
        locals.var_t0__blk1144_dn8 = assign26490_e20429_d_n8;
        locals.var_t0__blk1144_dn9 = assign26490_e20429_d_n9;
        locals.var_t0__blk1144_dn10 = assign26490_e20429_d_n10;
        locals.var_t0__blk1144_dn11 = assign26490_e20429_d_n11;
        locals.var_t0__blk1144_dn12 = assign26490_e20429_d_n12;

        let (assign26500_e20438, assign26500_e20438_d_n3, assign26500_e20438_d_n4, assign26500_e20438_d_n5, assign26500_e20438_d_n6, assign26500_e20438_d_n7, assign26500_e20438_d_n8, assign26500_e20438_d_n9, assign26500_e20438_d_n10, assign26500_e20438_d_n11, assign26500_e20438_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26500_e20433: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign26500_e20435: f64 = (assign26500_e20433 + 0.0001);
        let assign26500_e20436: f64 = (assign26500_e20435).sqrt();
        (assign26500_e20436, (((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) / (2.0 * assign26500_e20436)), (((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) / (2.0 * assign26500_e20436)), (((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) / (2.0 * assign26500_e20436)), (((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) / (2.0 * assign26500_e20436)), (((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) / (2.0 * assign26500_e20436)), (((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) / (2.0 * assign26500_e20436)), (((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) / (2.0 * assign26500_e20436)), (((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) / (2.0 * assign26500_e20436)), (((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) / (2.0 * assign26500_e20436)), (((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) / (2.0 * assign26500_e20436)),)
    } else {
        (locals.var_vgs_eff_1, locals.var_vgs_eff_1_dn3, locals.var_vgs_eff_1_dn4, locals.var_vgs_eff_1_dn5, locals.var_vgs_eff_1_dn6, locals.var_vgs_eff_1_dn7, locals.var_vgs_eff_1_dn8, locals.var_vgs_eff_1_dn9, locals.var_vgs_eff_1_dn10, locals.var_vgs_eff_1_dn11, locals.var_vgs_eff_1_dn12,)
    }
};
        locals.var_vgs_eff_1 = assign26500_e20438;
        locals.var_vgs_eff_1_dn3 = assign26500_e20438_d_n3;
        locals.var_vgs_eff_1_dn4 = assign26500_e20438_d_n4;
        locals.var_vgs_eff_1_dn5 = assign26500_e20438_d_n5;
        locals.var_vgs_eff_1_dn6 = assign26500_e20438_d_n6;
        locals.var_vgs_eff_1_dn7 = assign26500_e20438_d_n7;
        locals.var_vgs_eff_1_dn8 = assign26500_e20438_d_n8;
        locals.var_vgs_eff_1_dn9 = assign26500_e20438_d_n9;
        locals.var_vgs_eff_1_dn10 = assign26500_e20438_d_n10;
        locals.var_vgs_eff_1_dn11 = assign26500_e20438_d_n11;
        locals.var_vgs_eff_1_dn12 = assign26500_e20438_d_n12;

        let (assign26510_e20444, assign26510_e20444_d_n3, assign26510_e20444_d_n4, assign26510_e20444_d_n5, assign26510_e20444_d_n6, assign26510_e20444_d_n7, assign26510_e20444_d_n8, assign26510_e20444_d_n9, assign26510_e20444_d_n10, assign26510_e20444_d_n11, assign26510_e20444_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26510_e20442: f64 = (locals.var_vgs * locals.var_vgs_eff_1);
        (assign26510_e20442, (locals.var_vgs * locals.var_vgs_eff_1_dn3), (locals.var_vgs * locals.var_vgs_eff_1_dn4), (locals.var_vgs * locals.var_vgs_eff_1_dn5), (locals.var_vgs * locals.var_vgs_eff_1_dn6), (locals.var_vgs * locals.var_vgs_eff_1_dn7), ((locals.var_vgs_dn8 * locals.var_vgs_eff_1) + (locals.var_vgs * locals.var_vgs_eff_1_dn8)), ((locals.var_vgs_dn9 * locals.var_vgs_eff_1) + (locals.var_vgs * locals.var_vgs_eff_1_dn9)), (locals.var_vgs * locals.var_vgs_eff_1_dn10), (locals.var_vgs * locals.var_vgs_eff_1_dn11), (locals.var_vgs * locals.var_vgs_eff_1_dn12),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign26510_e20444;
        locals.var_t2__blk1146_dn3 = assign26510_e20444_d_n3;
        locals.var_t2__blk1146_dn4 = assign26510_e20444_d_n4;
        locals.var_t2__blk1146_dn5 = assign26510_e20444_d_n5;
        locals.var_t2__blk1146_dn6 = assign26510_e20444_d_n6;
        locals.var_t2__blk1146_dn7 = assign26510_e20444_d_n7;
        locals.var_t2__blk1146_dn8 = assign26510_e20444_d_n8;
        locals.var_t2__blk1146_dn9 = assign26510_e20444_d_n9;
        locals.var_t2__blk1146_dn10 = assign26510_e20444_d_n10;
        locals.var_t2__blk1146_dn11 = assign26510_e20444_d_n11;
        locals.var_t2__blk1146_dn12 = assign26510_e20444_d_n12;

        let (assign26520_e20448, assign26520_e20448_d_n3, assign26520_e20448_d_n4, assign26520_e20448_d_n5, assign26520_e20448_d_n6, assign26520_e20448_d_n7, assign26520_e20448_d_n8, assign26520_e20448_d_n9, assign26520_e20448_d_n10, assign26520_e20448_d_n11, assign26520_e20448_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        (locals.var_pparam_b4soiaechvbedges, locals.var_pparam_b4soiaechvbedges_dn3, locals.var_pparam_b4soiaechvbedges_dn4, locals.var_pparam_b4soiaechvbedges_dn5, locals.var_pparam_b4soiaechvbedges_dn6, locals.var_pparam_b4soiaechvbedges_dn7, locals.var_pparam_b4soiaechvbedges_dn8, locals.var_pparam_b4soiaechvbedges_dn9, locals.var_pparam_b4soiaechvbedges_dn10, locals.var_pparam_b4soiaechvbedges_dn11, locals.var_pparam_b4soiaechvbedges_dn12,)
    } else {
        (locals.var_t13, locals.var_t13_dn3, locals.var_t13_dn4, locals.var_t13_dn5, locals.var_t13_dn6, locals.var_t13_dn7, locals.var_t13_dn8, locals.var_t13_dn9, locals.var_t13_dn10, locals.var_t13_dn11, locals.var_t13_dn12,)
    }
};
        locals.var_t13 = assign26520_e20448;
        locals.var_t13_dn3 = assign26520_e20448_d_n3;
        locals.var_t13_dn4 = assign26520_e20448_d_n4;
        locals.var_t13_dn5 = assign26520_e20448_d_n5;
        locals.var_t13_dn6 = assign26520_e20448_d_n6;
        locals.var_t13_dn7 = assign26520_e20448_d_n7;
        locals.var_t13_dn8 = assign26520_e20448_d_n8;
        locals.var_t13_dn9 = assign26520_e20448_d_n9;
        locals.var_t13_dn10 = assign26520_e20448_d_n10;
        locals.var_t13_dn11 = assign26520_e20448_d_n11;
        locals.var_t13_dn12 = assign26520_e20448_d_n12;

        let (assign26530_e20452, assign26530_e20452_d_n3, assign26530_e20452_d_n4, assign26530_e20452_d_n5, assign26530_e20452_d_n6, assign26530_e20452_d_n7, assign26530_e20452_d_n8, assign26530_e20452_d_n9, assign26530_e20452_d_n10, assign26530_e20452_d_n11, assign26530_e20452_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        (locals.var_pparam_b4soiaechvbedged, locals.var_pparam_b4soiaechvbedged_dn3, locals.var_pparam_b4soiaechvbedged_dn4, locals.var_pparam_b4soiaechvbedged_dn5, locals.var_pparam_b4soiaechvbedged_dn6, locals.var_pparam_b4soiaechvbedged_dn7, locals.var_pparam_b4soiaechvbedged_dn8, locals.var_pparam_b4soiaechvbedged_dn9, locals.var_pparam_b4soiaechvbedged_dn10, locals.var_pparam_b4soiaechvbedged_dn11, locals.var_pparam_b4soiaechvbedged_dn12,)
    } else {
        (locals.var_t14, locals.var_t14_dn3, locals.var_t14_dn4, locals.var_t14_dn5, locals.var_t14_dn6, locals.var_t14_dn7, locals.var_t14_dn8, locals.var_t14_dn9, locals.var_t14_dn10, locals.var_t14_dn11, locals.var_t14_dn12,)
    }
};
        locals.var_t14 = assign26530_e20452;
        locals.var_t14_dn3 = assign26530_e20452_d_n3;
        locals.var_t14_dn4 = assign26530_e20452_d_n4;
        locals.var_t14_dn5 = assign26530_e20452_d_n5;
        locals.var_t14_dn6 = assign26530_e20452_d_n6;
        locals.var_t14_dn7 = assign26530_e20452_d_n7;
        locals.var_t14_dn8 = assign26530_e20452_d_n8;
        locals.var_t14_dn9 = assign26530_e20452_d_n9;
        locals.var_t14_dn10 = assign26530_e20452_d_n10;
        locals.var_t14_dn11 = assign26530_e20452_d_n11;
        locals.var_t14_dn12 = assign26530_e20452_d_n12;

        let (assign26540_e20456, assign26540_e20456_d_n3, assign26540_e20456_d_n4, assign26540_e20456_d_n5, assign26540_e20456_d_n6, assign26540_e20456_d_n7, assign26540_e20456_d_n8, assign26540_e20456_d_n9, assign26540_e20456_d_n10, assign26540_e20456_d_n11, assign26540_e20456_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        (locals.var_pparam_b4soibechvbedge, locals.var_pparam_b4soibechvbedge_dn3, locals.var_pparam_b4soibechvbedge_dn4, locals.var_pparam_b4soibechvbedge_dn5, locals.var_pparam_b4soibechvbedge_dn6, locals.var_pparam_b4soibechvbedge_dn7, locals.var_pparam_b4soibechvbedge_dn8, locals.var_pparam_b4soibechvbedge_dn9, locals.var_pparam_b4soibechvbedge_dn10, locals.var_pparam_b4soibechvbedge_dn11, locals.var_pparam_b4soibechvbedge_dn12,)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign26540_e20456;
        locals.var_t12_dn3 = assign26540_e20456_d_n3;
        locals.var_t12_dn4 = assign26540_e20456_d_n4;
        locals.var_t12_dn5 = assign26540_e20456_d_n5;
        locals.var_t12_dn6 = assign26540_e20456_d_n6;
        locals.var_t12_dn7 = assign26540_e20456_d_n7;
        locals.var_t12_dn8 = assign26540_e20456_d_n8;
        locals.var_t12_dn9 = assign26540_e20456_d_n9;
        locals.var_t12_dn10 = assign26540_e20456_d_n10;
        locals.var_t12_dn11 = assign26540_e20456_d_n11;
        locals.var_t12_dn12 = assign26540_e20456_d_n12;

    }

    pub(super) fn stamp_transient_block_69(
        locals: &mut StampLocals,
    ) {
        let (assign26550_e20464, assign26550_e20464_d_n3, assign26550_e20464_d_n4, assign26550_e20464_d_n5, assign26550_e20464_d_n6, assign26550_e20464_d_n7, assign26550_e20464_d_n8, assign26550_e20464_d_n9, assign26550_e20464_d_n10, assign26550_e20464_d_n11, assign26550_e20464_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26550_e20460: f64 = (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd);
        let assign26550_e20462: f64 = (assign26550_e20460 - locals.var_pparam_b4soibigsd);
        (assign26550_e20462, (((locals.var_pparam_b4soiaigsd_dn3 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn3)) - locals.var_pparam_b4soibigsd_dn3), (((locals.var_pparam_b4soiaigsd_dn4 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn4)) - locals.var_pparam_b4soibigsd_dn4), (((locals.var_pparam_b4soiaigsd_dn5 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn5)) - locals.var_pparam_b4soibigsd_dn5), (((locals.var_pparam_b4soiaigsd_dn6 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn6)) - locals.var_pparam_b4soibigsd_dn6), (((locals.var_pparam_b4soiaigsd_dn7 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn7)) - locals.var_pparam_b4soibigsd_dn7), (((locals.var_pparam_b4soiaigsd_dn8 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn8)) - locals.var_pparam_b4soibigsd_dn8), (((locals.var_pparam_b4soiaigsd_dn9 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn9)) - locals.var_pparam_b4soibigsd_dn9), (((locals.var_pparam_b4soiaigsd_dn10 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn10)) - locals.var_pparam_b4soibigsd_dn10), (((locals.var_pparam_b4soiaigsd_dn11 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn11)) - locals.var_pparam_b4soibigsd_dn11), (((locals.var_pparam_b4soiaigsd_dn12 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn12)) - locals.var_pparam_b4soibigsd_dn12),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign26550_e20464;
        locals.var_t3__blk1147_dn3 = assign26550_e20464_d_n3;
        locals.var_t3__blk1147_dn4 = assign26550_e20464_d_n4;
        locals.var_t3__blk1147_dn5 = assign26550_e20464_d_n5;
        locals.var_t3__blk1147_dn6 = assign26550_e20464_d_n6;
        locals.var_t3__blk1147_dn7 = assign26550_e20464_d_n7;
        locals.var_t3__blk1147_dn8 = assign26550_e20464_d_n8;
        locals.var_t3__blk1147_dn9 = assign26550_e20464_d_n9;
        locals.var_t3__blk1147_dn10 = assign26550_e20464_d_n10;
        locals.var_t3__blk1147_dn11 = assign26550_e20464_d_n11;
        locals.var_t3__blk1147_dn12 = assign26550_e20464_d_n12;

        let (assign26560_e20470, assign26560_e20470_d_n3, assign26560_e20470_d_n4, assign26560_e20470_d_n5, assign26560_e20470_d_n6, assign26560_e20470_d_n7, assign26560_e20470_d_n8, assign26560_e20470_d_n9, assign26560_e20470_d_n10, assign26560_e20470_d_n11, assign26560_e20470_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26560_e20468: f64 = (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd);
        (assign26560_e20468, ((locals.var_pparam_b4soibigsd_dn3 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn3)), ((locals.var_pparam_b4soibigsd_dn4 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn4)), ((locals.var_pparam_b4soibigsd_dn5 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn5)), ((locals.var_pparam_b4soibigsd_dn6 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn6)), ((locals.var_pparam_b4soibigsd_dn7 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn7)), ((locals.var_pparam_b4soibigsd_dn8 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn8)), ((locals.var_pparam_b4soibigsd_dn9 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn9)), ((locals.var_pparam_b4soibigsd_dn10 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn10)), ((locals.var_pparam_b4soibigsd_dn11 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn11)), ((locals.var_pparam_b4soibigsd_dn12 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn12)),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign26560_e20470;
        locals.var_t4__blk1148_dn3 = assign26560_e20470_d_n3;
        locals.var_t4__blk1148_dn4 = assign26560_e20470_d_n4;
        locals.var_t4__blk1148_dn5 = assign26560_e20470_d_n5;
        locals.var_t4__blk1148_dn6 = assign26560_e20470_d_n6;
        locals.var_t4__blk1148_dn7 = assign26560_e20470_d_n7;
        locals.var_t4__blk1148_dn8 = assign26560_e20470_d_n8;
        locals.var_t4__blk1148_dn9 = assign26560_e20470_d_n9;
        locals.var_t4__blk1148_dn10 = assign26560_e20470_d_n10;
        locals.var_t4__blk1148_dn11 = assign26560_e20470_d_n11;
        locals.var_t4__blk1148_dn12 = assign26560_e20470_d_n12;

        let (assign26570_e20486, assign26570_e20486_d_n3, assign26570_e20486_d_n4, assign26570_e20486_d_n5, assign26570_e20486_d_n6, assign26570_e20486_d_n7, assign26570_e20486_d_n8, assign26570_e20486_d_n9, assign26570_e20486_d_n10, assign26570_e20486_d_n11, assign26570_e20486_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26570_e20476: f64 = (locals.var_t3__blk1147 * locals.var_vgs_eff_1);
        let assign26570_e20477: f64 = (locals.var_pparam_b4soiaigsd + assign26570_e20476);
        let assign26570_e20480: f64 = (locals.var_t4__blk1148 * locals.var_vgs_eff_1);
        let assign26570_e20482: f64 = (assign26570_e20480 * locals.var_vgs_eff_1);
        let assign26570_e20483: f64 = (assign26570_e20477 - assign26570_e20482);
        let assign26570_e20484: f64 = (locals.var_t12 * assign26570_e20483);
        (assign26570_e20484, ((locals.var_t12_dn3 * assign26570_e20483) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn3 + ((locals.var_t3__blk1147_dn3 * locals.var_vgs_eff_1) + (locals.var_t3__blk1147 * locals.var_vgs_eff_1_dn3))) - ((((locals.var_t4__blk1148_dn3 * locals.var_vgs_eff_1) + (locals.var_t4__blk1148 * locals.var_vgs_eff_1_dn3)) * locals.var_vgs_eff_1) + (assign26570_e20480 * locals.var_vgs_eff_1_dn3))))), ((locals.var_t12_dn4 * assign26570_e20483) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn4 + ((locals.var_t3__blk1147_dn4 * locals.var_vgs_eff_1) + (locals.var_t3__blk1147 * locals.var_vgs_eff_1_dn4))) - ((((locals.var_t4__blk1148_dn4 * locals.var_vgs_eff_1) + (locals.var_t4__blk1148 * locals.var_vgs_eff_1_dn4)) * locals.var_vgs_eff_1) + (assign26570_e20480 * locals.var_vgs_eff_1_dn4))))), ((locals.var_t12_dn5 * assign26570_e20483) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn5 + ((locals.var_t3__blk1147_dn5 * locals.var_vgs_eff_1) + (locals.var_t3__blk1147 * locals.var_vgs_eff_1_dn5))) - ((((locals.var_t4__blk1148_dn5 * locals.var_vgs_eff_1) + (locals.var_t4__blk1148 * locals.var_vgs_eff_1_dn5)) * locals.var_vgs_eff_1) + (assign26570_e20480 * locals.var_vgs_eff_1_dn5))))), ((locals.var_t12_dn6 * assign26570_e20483) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn6 + ((locals.var_t3__blk1147_dn6 * locals.var_vgs_eff_1) + (locals.var_t3__blk1147 * locals.var_vgs_eff_1_dn6))) - ((((locals.var_t4__blk1148_dn6 * locals.var_vgs_eff_1) + (locals.var_t4__blk1148 * locals.var_vgs_eff_1_dn6)) * locals.var_vgs_eff_1) + (assign26570_e20480 * locals.var_vgs_eff_1_dn6))))), ((locals.var_t12_dn7 * assign26570_e20483) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn7 + ((locals.var_t3__blk1147_dn7 * locals.var_vgs_eff_1) + (locals.var_t3__blk1147 * locals.var_vgs_eff_1_dn7))) - ((((locals.var_t4__blk1148_dn7 * locals.var_vgs_eff_1) + (locals.var_t4__blk1148 * locals.var_vgs_eff_1_dn7)) * locals.var_vgs_eff_1) + (assign26570_e20480 * locals.var_vgs_eff_1_dn7))))), ((locals.var_t12_dn8 * assign26570_e20483) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn8 + ((locals.var_t3__blk1147_dn8 * locals.var_vgs_eff_1) + (locals.var_t3__blk1147 * locals.var_vgs_eff_1_dn8))) - ((((locals.var_t4__blk1148_dn8 * locals.var_vgs_eff_1) + (locals.var_t4__blk1148 * locals.var_vgs_eff_1_dn8)) * locals.var_vgs_eff_1) + (assign26570_e20480 * locals.var_vgs_eff_1_dn8))))), ((locals.var_t12_dn9 * assign26570_e20483) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn9 + ((locals.var_t3__blk1147_dn9 * locals.var_vgs_eff_1) + (locals.var_t3__blk1147 * locals.var_vgs_eff_1_dn9))) - ((((locals.var_t4__blk1148_dn9 * locals.var_vgs_eff_1) + (locals.var_t4__blk1148 * locals.var_vgs_eff_1_dn9)) * locals.var_vgs_eff_1) + (assign26570_e20480 * locals.var_vgs_eff_1_dn9))))), ((locals.var_t12_dn10 * assign26570_e20483) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn10 + ((locals.var_t3__blk1147_dn10 * locals.var_vgs_eff_1) + (locals.var_t3__blk1147 * locals.var_vgs_eff_1_dn10))) - ((((locals.var_t4__blk1148_dn10 * locals.var_vgs_eff_1) + (locals.var_t4__blk1148 * locals.var_vgs_eff_1_dn10)) * locals.var_vgs_eff_1) + (assign26570_e20480 * locals.var_vgs_eff_1_dn10))))), ((locals.var_t12_dn11 * assign26570_e20483) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn11 + ((locals.var_t3__blk1147_dn11 * locals.var_vgs_eff_1) + (locals.var_t3__blk1147 * locals.var_vgs_eff_1_dn11))) - ((((locals.var_t4__blk1148_dn11 * locals.var_vgs_eff_1) + (locals.var_t4__blk1148 * locals.var_vgs_eff_1_dn11)) * locals.var_vgs_eff_1) + (assign26570_e20480 * locals.var_vgs_eff_1_dn11))))), ((locals.var_t12_dn12 * assign26570_e20483) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn12 + ((locals.var_t3__blk1147_dn12 * locals.var_vgs_eff_1) + (locals.var_t3__blk1147 * locals.var_vgs_eff_1_dn12))) - ((((locals.var_t4__blk1148_dn12 * locals.var_vgs_eff_1) + (locals.var_t4__blk1148 * locals.var_vgs_eff_1_dn12)) * locals.var_vgs_eff_1) + (assign26570_e20480 * locals.var_vgs_eff_1_dn12))))),)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign26570_e20486;
        locals.var_t5__blk1149_dn3 = assign26570_e20486_d_n3;
        locals.var_t5__blk1149_dn4 = assign26570_e20486_d_n4;
        locals.var_t5__blk1149_dn5 = assign26570_e20486_d_n5;
        locals.var_t5__blk1149_dn6 = assign26570_e20486_d_n6;
        locals.var_t5__blk1149_dn7 = assign26570_e20486_d_n7;
        locals.var_t5__blk1149_dn8 = assign26570_e20486_d_n8;
        locals.var_t5__blk1149_dn9 = assign26570_e20486_d_n9;
        locals.var_t5__blk1149_dn10 = assign26570_e20486_d_n10;
        locals.var_t5__blk1149_dn11 = assign26570_e20486_d_n11;
        locals.var_t5__blk1149_dn12 = assign26570_e20486_d_n12;

        let assign26580_e20489: f64 = if locals.var_t5__blk1149 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1640 = assign26580_e20489;

        let (assign26590_e20495, assign26590_e20495_d_n3, assign26590_e20495_d_n4, assign26590_e20495_d_n5, assign26590_e20495_d_n6, assign26590_e20495_d_n7, assign26590_e20495_d_n8, assign26590_e20495_d_n9, assign26590_e20495_d_n10, assign26590_e20495_d_n11, assign26590_e20495_d_n12,) = {
    if ((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1640 != 0.0)) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign26590_e20495;
        locals.var_t6__blk1150_dn3 = assign26590_e20495_d_n3;
        locals.var_t6__blk1150_dn4 = assign26590_e20495_d_n4;
        locals.var_t6__blk1150_dn5 = assign26590_e20495_d_n5;
        locals.var_t6__blk1150_dn6 = assign26590_e20495_d_n6;
        locals.var_t6__blk1150_dn7 = assign26590_e20495_d_n7;
        locals.var_t6__blk1150_dn8 = assign26590_e20495_d_n8;
        locals.var_t6__blk1150_dn9 = assign26590_e20495_d_n9;
        locals.var_t6__blk1150_dn10 = assign26590_e20495_d_n10;
        locals.var_t6__blk1150_dn11 = assign26590_e20495_d_n11;
        locals.var_t6__blk1150_dn12 = assign26590_e20495_d_n12;

        let assign26600_e20498: f64 = (-100.0);
        let assign26600_e20499: f64 = if locals.var_t5__blk1149 < assign26600_e20498 { 1.0 } else { 0.0 };
        locals.var_guard1641 = assign26600_e20499;

        let (assign26610_e20508, assign26610_e20508_d_n3, assign26610_e20508_d_n4, assign26610_e20508_d_n5, assign26610_e20508_d_n6, assign26610_e20508_d_n7, assign26610_e20508_d_n8, assign26610_e20508_d_n9, assign26610_e20508_d_n10, assign26610_e20508_d_n11, assign26610_e20508_d_n12,) = {
    if (((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1640 == 0.0)) && (locals.var_guard1641 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign26610_e20508;
        locals.var_t6__blk1150_dn3 = assign26610_e20508_d_n3;
        locals.var_t6__blk1150_dn4 = assign26610_e20508_d_n4;
        locals.var_t6__blk1150_dn5 = assign26610_e20508_d_n5;
        locals.var_t6__blk1150_dn6 = assign26610_e20508_d_n6;
        locals.var_t6__blk1150_dn7 = assign26610_e20508_d_n7;
        locals.var_t6__blk1150_dn8 = assign26610_e20508_d_n8;
        locals.var_t6__blk1150_dn9 = assign26610_e20508_d_n9;
        locals.var_t6__blk1150_dn10 = assign26610_e20508_d_n10;
        locals.var_t6__blk1150_dn11 = assign26610_e20508_d_n11;
        locals.var_t6__blk1150_dn12 = assign26610_e20508_d_n12;

        let (assign26620_e20519, assign26620_e20519_d_n3, assign26620_e20519_d_n4, assign26620_e20519_d_n5, assign26620_e20519_d_n6, assign26620_e20519_d_n7, assign26620_e20519_d_n8, assign26620_e20519_d_n9, assign26620_e20519_d_n10, assign26620_e20519_d_n11, assign26620_e20519_d_n12,) = {
    if (((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1640 == 0.0)) && (locals.var_guard1641 == 0.0)) {
        let assign26620_e20517: f64 = (locals.var_t5__blk1149).exp();
        (assign26620_e20517, (assign26620_e20517 * locals.var_t5__blk1149_dn3), (assign26620_e20517 * locals.var_t5__blk1149_dn4), (assign26620_e20517 * locals.var_t5__blk1149_dn5), (assign26620_e20517 * locals.var_t5__blk1149_dn6), (assign26620_e20517 * locals.var_t5__blk1149_dn7), (assign26620_e20517 * locals.var_t5__blk1149_dn8), (assign26620_e20517 * locals.var_t5__blk1149_dn9), (assign26620_e20517 * locals.var_t5__blk1149_dn10), (assign26620_e20517 * locals.var_t5__blk1149_dn11), (assign26620_e20517 * locals.var_t5__blk1149_dn12),)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign26620_e20519;
        locals.var_t6__blk1150_dn3 = assign26620_e20519_d_n3;
        locals.var_t6__blk1150_dn4 = assign26620_e20519_d_n4;
        locals.var_t6__blk1150_dn5 = assign26620_e20519_d_n5;
        locals.var_t6__blk1150_dn6 = assign26620_e20519_d_n6;
        locals.var_t6__blk1150_dn7 = assign26620_e20519_d_n7;
        locals.var_t6__blk1150_dn8 = assign26620_e20519_d_n8;
        locals.var_t6__blk1150_dn9 = assign26620_e20519_d_n9;
        locals.var_t6__blk1150_dn10 = assign26620_e20519_d_n10;
        locals.var_t6__blk1150_dn11 = assign26620_e20519_d_n11;
        locals.var_t6__blk1150_dn12 = assign26620_e20519_d_n12;

        let (assign26630_e20527, assign26630_e20527_d_n3, assign26630_e20527_d_n4, assign26630_e20527_d_n5, assign26630_e20527_d_n6, assign26630_e20527_d_n7, assign26630_e20527_d_n8, assign26630_e20527_d_n9, assign26630_e20527_d_n10, assign26630_e20527_d_n11, assign26630_e20527_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26630_e20523: f64 = (locals.var_t13 * locals.var_t2__blk1146);
        let assign26630_e20525: f64 = (assign26630_e20523 * locals.var_t6__blk1150);
        (assign26630_e20525, ((((locals.var_t13_dn3 * locals.var_t2__blk1146) + (locals.var_t13 * locals.var_t2__blk1146_dn3)) * locals.var_t6__blk1150) + (assign26630_e20523 * locals.var_t6__blk1150_dn3)), ((((locals.var_t13_dn4 * locals.var_t2__blk1146) + (locals.var_t13 * locals.var_t2__blk1146_dn4)) * locals.var_t6__blk1150) + (assign26630_e20523 * locals.var_t6__blk1150_dn4)), ((((locals.var_t13_dn5 * locals.var_t2__blk1146) + (locals.var_t13 * locals.var_t2__blk1146_dn5)) * locals.var_t6__blk1150) + (assign26630_e20523 * locals.var_t6__blk1150_dn5)), ((((locals.var_t13_dn6 * locals.var_t2__blk1146) + (locals.var_t13 * locals.var_t2__blk1146_dn6)) * locals.var_t6__blk1150) + (assign26630_e20523 * locals.var_t6__blk1150_dn6)), ((((locals.var_t13_dn7 * locals.var_t2__blk1146) + (locals.var_t13 * locals.var_t2__blk1146_dn7)) * locals.var_t6__blk1150) + (assign26630_e20523 * locals.var_t6__blk1150_dn7)), ((((locals.var_t13_dn8 * locals.var_t2__blk1146) + (locals.var_t13 * locals.var_t2__blk1146_dn8)) * locals.var_t6__blk1150) + (assign26630_e20523 * locals.var_t6__blk1150_dn8)), ((((locals.var_t13_dn9 * locals.var_t2__blk1146) + (locals.var_t13 * locals.var_t2__blk1146_dn9)) * locals.var_t6__blk1150) + (assign26630_e20523 * locals.var_t6__blk1150_dn9)), ((((locals.var_t13_dn10 * locals.var_t2__blk1146) + (locals.var_t13 * locals.var_t2__blk1146_dn10)) * locals.var_t6__blk1150) + (assign26630_e20523 * locals.var_t6__blk1150_dn10)), ((((locals.var_t13_dn11 * locals.var_t2__blk1146) + (locals.var_t13 * locals.var_t2__blk1146_dn11)) * locals.var_t6__blk1150) + (assign26630_e20523 * locals.var_t6__blk1150_dn11)), ((((locals.var_t13_dn12 * locals.var_t2__blk1146) + (locals.var_t13 * locals.var_t2__blk1146_dn12)) * locals.var_t6__blk1150) + (assign26630_e20523 * locals.var_t6__blk1150_dn12)),)
    } else {
        (locals.var_igs_1, locals.var_igs_1_dn3, locals.var_igs_1_dn4, locals.var_igs_1_dn5, locals.var_igs_1_dn6, locals.var_igs_1_dn7, locals.var_igs_1_dn8, locals.var_igs_1_dn9, locals.var_igs_1_dn10, locals.var_igs_1_dn11, locals.var_igs_1_dn12,)
    }
};
        locals.var_igs_1 = assign26630_e20527;
        locals.var_igs_1_dn3 = assign26630_e20527_d_n3;
        locals.var_igs_1_dn4 = assign26630_e20527_d_n4;
        locals.var_igs_1_dn5 = assign26630_e20527_d_n5;
        locals.var_igs_1_dn6 = assign26630_e20527_d_n6;
        locals.var_igs_1_dn7 = assign26630_e20527_d_n7;
        locals.var_igs_1_dn8 = assign26630_e20527_d_n8;
        locals.var_igs_1_dn9 = assign26630_e20527_d_n9;
        locals.var_igs_1_dn10 = assign26630_e20527_d_n10;
        locals.var_igs_1_dn11 = assign26630_e20527_d_n11;
        locals.var_igs_1_dn12 = assign26630_e20527_d_n12;

        let (assign26640_e20533, assign26640_e20533_d_n3, assign26640_e20533_d_n4, assign26640_e20533_d_n5, assign26640_e20533_d_n6, assign26640_e20533_d_n7, assign26640_e20533_d_n8, assign26640_e20533_d_n9, assign26640_e20533_d_n10, assign26640_e20533_d_n11, assign26640_e20533_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26640_e20531: f64 = (locals.var_vgd - locals.var_pparam_b4soivfbsd);
        (assign26640_e20531, (-locals.var_pparam_b4soivfbsd_dn3), (-locals.var_pparam_b4soivfbsd_dn4), (-locals.var_pparam_b4soivfbsd_dn5), (-locals.var_pparam_b4soivfbsd_dn6), (locals.var_vgd_dn7 - locals.var_pparam_b4soivfbsd_dn7), (locals.var_vgd_dn8 - locals.var_pparam_b4soivfbsd_dn8), (locals.var_vgd_dn9 - locals.var_pparam_b4soivfbsd_dn9), (-locals.var_pparam_b4soivfbsd_dn10), (-locals.var_pparam_b4soivfbsd_dn11), (-locals.var_pparam_b4soivfbsd_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign26640_e20533;
        locals.var_t0__blk1144_dn3 = assign26640_e20533_d_n3;
        locals.var_t0__blk1144_dn4 = assign26640_e20533_d_n4;
        locals.var_t0__blk1144_dn5 = assign26640_e20533_d_n5;
        locals.var_t0__blk1144_dn6 = assign26640_e20533_d_n6;
        locals.var_t0__blk1144_dn7 = assign26640_e20533_d_n7;
        locals.var_t0__blk1144_dn8 = assign26640_e20533_d_n8;
        locals.var_t0__blk1144_dn9 = assign26640_e20533_d_n9;
        locals.var_t0__blk1144_dn10 = assign26640_e20533_d_n10;
        locals.var_t0__blk1144_dn11 = assign26640_e20533_d_n11;
        locals.var_t0__blk1144_dn12 = assign26640_e20533_d_n12;

        let (assign26650_e20542, assign26650_e20542_d_n3, assign26650_e20542_d_n4, assign26650_e20542_d_n5, assign26650_e20542_d_n6, assign26650_e20542_d_n7, assign26650_e20542_d_n8, assign26650_e20542_d_n9, assign26650_e20542_d_n10, assign26650_e20542_d_n11, assign26650_e20542_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26650_e20537: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign26650_e20539: f64 = (assign26650_e20537 + 0.0001);
        let assign26650_e20540: f64 = (assign26650_e20539).sqrt();
        (assign26650_e20540, (((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) / (2.0 * assign26650_e20540)), (((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) / (2.0 * assign26650_e20540)), (((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) / (2.0 * assign26650_e20540)), (((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) / (2.0 * assign26650_e20540)), (((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) / (2.0 * assign26650_e20540)), (((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) / (2.0 * assign26650_e20540)), (((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) / (2.0 * assign26650_e20540)), (((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) / (2.0 * assign26650_e20540)), (((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) / (2.0 * assign26650_e20540)), (((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) / (2.0 * assign26650_e20540)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11, locals.var_vgd_eff_dn12,)
    }
};
        locals.var_vgd_eff = assign26650_e20542;
        locals.var_vgd_eff_dn3 = assign26650_e20542_d_n3;
        locals.var_vgd_eff_dn4 = assign26650_e20542_d_n4;
        locals.var_vgd_eff_dn5 = assign26650_e20542_d_n5;
        locals.var_vgd_eff_dn6 = assign26650_e20542_d_n6;
        locals.var_vgd_eff_dn7 = assign26650_e20542_d_n7;
        locals.var_vgd_eff_dn8 = assign26650_e20542_d_n8;
        locals.var_vgd_eff_dn9 = assign26650_e20542_d_n9;
        locals.var_vgd_eff_dn10 = assign26650_e20542_d_n10;
        locals.var_vgd_eff_dn11 = assign26650_e20542_d_n11;
        locals.var_vgd_eff_dn12 = assign26650_e20542_d_n12;

        let (assign26660_e20548, assign26660_e20548_d_n3, assign26660_e20548_d_n4, assign26660_e20548_d_n5, assign26660_e20548_d_n6, assign26660_e20548_d_n7, assign26660_e20548_d_n8, assign26660_e20548_d_n9, assign26660_e20548_d_n10, assign26660_e20548_d_n11, assign26660_e20548_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26660_e20546: f64 = (locals.var_vgd * locals.var_vgd_eff);
        (assign26660_e20546, (locals.var_vgd * locals.var_vgd_eff_dn3), (locals.var_vgd * locals.var_vgd_eff_dn4), (locals.var_vgd * locals.var_vgd_eff_dn5), (locals.var_vgd * locals.var_vgd_eff_dn6), ((locals.var_vgd_dn7 * locals.var_vgd_eff) + (locals.var_vgd * locals.var_vgd_eff_dn7)), ((locals.var_vgd_dn8 * locals.var_vgd_eff) + (locals.var_vgd * locals.var_vgd_eff_dn8)), ((locals.var_vgd_dn9 * locals.var_vgd_eff) + (locals.var_vgd * locals.var_vgd_eff_dn9)), (locals.var_vgd * locals.var_vgd_eff_dn10), (locals.var_vgd * locals.var_vgd_eff_dn11), (locals.var_vgd * locals.var_vgd_eff_dn12),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign26660_e20548;
        locals.var_t2__blk1146_dn3 = assign26660_e20548_d_n3;
        locals.var_t2__blk1146_dn4 = assign26660_e20548_d_n4;
        locals.var_t2__blk1146_dn5 = assign26660_e20548_d_n5;
        locals.var_t2__blk1146_dn6 = assign26660_e20548_d_n6;
        locals.var_t2__blk1146_dn7 = assign26660_e20548_d_n7;
        locals.var_t2__blk1146_dn8 = assign26660_e20548_d_n8;
        locals.var_t2__blk1146_dn9 = assign26660_e20548_d_n9;
        locals.var_t2__blk1146_dn10 = assign26660_e20548_d_n10;
        locals.var_t2__blk1146_dn11 = assign26660_e20548_d_n11;
        locals.var_t2__blk1146_dn12 = assign26660_e20548_d_n12;

        let (assign26670_e20564, assign26670_e20564_d_n3, assign26670_e20564_d_n4, assign26670_e20564_d_n5, assign26670_e20564_d_n6, assign26670_e20564_d_n7, assign26670_e20564_d_n8, assign26670_e20564_d_n9, assign26670_e20564_d_n10, assign26670_e20564_d_n11, assign26670_e20564_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26670_e20554: f64 = (locals.var_t3__blk1147 * locals.var_vgd_eff);
        let assign26670_e20555: f64 = (locals.var_pparam_b4soiaigsd + assign26670_e20554);
        let assign26670_e20558: f64 = (locals.var_t4__blk1148 * locals.var_vgd_eff);
        let assign26670_e20560: f64 = (assign26670_e20558 * locals.var_vgd_eff);
        let assign26670_e20561: f64 = (assign26670_e20555 - assign26670_e20560);
        let assign26670_e20562: f64 = (locals.var_t12 * assign26670_e20561);
        (assign26670_e20562, ((locals.var_t12_dn3 * assign26670_e20561) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn3 + ((locals.var_t3__blk1147_dn3 * locals.var_vgd_eff) + (locals.var_t3__blk1147 * locals.var_vgd_eff_dn3))) - ((((locals.var_t4__blk1148_dn3 * locals.var_vgd_eff) + (locals.var_t4__blk1148 * locals.var_vgd_eff_dn3)) * locals.var_vgd_eff) + (assign26670_e20558 * locals.var_vgd_eff_dn3))))), ((locals.var_t12_dn4 * assign26670_e20561) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn4 + ((locals.var_t3__blk1147_dn4 * locals.var_vgd_eff) + (locals.var_t3__blk1147 * locals.var_vgd_eff_dn4))) - ((((locals.var_t4__blk1148_dn4 * locals.var_vgd_eff) + (locals.var_t4__blk1148 * locals.var_vgd_eff_dn4)) * locals.var_vgd_eff) + (assign26670_e20558 * locals.var_vgd_eff_dn4))))), ((locals.var_t12_dn5 * assign26670_e20561) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn5 + ((locals.var_t3__blk1147_dn5 * locals.var_vgd_eff) + (locals.var_t3__blk1147 * locals.var_vgd_eff_dn5))) - ((((locals.var_t4__blk1148_dn5 * locals.var_vgd_eff) + (locals.var_t4__blk1148 * locals.var_vgd_eff_dn5)) * locals.var_vgd_eff) + (assign26670_e20558 * locals.var_vgd_eff_dn5))))), ((locals.var_t12_dn6 * assign26670_e20561) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn6 + ((locals.var_t3__blk1147_dn6 * locals.var_vgd_eff) + (locals.var_t3__blk1147 * locals.var_vgd_eff_dn6))) - ((((locals.var_t4__blk1148_dn6 * locals.var_vgd_eff) + (locals.var_t4__blk1148 * locals.var_vgd_eff_dn6)) * locals.var_vgd_eff) + (assign26670_e20558 * locals.var_vgd_eff_dn6))))), ((locals.var_t12_dn7 * assign26670_e20561) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn7 + ((locals.var_t3__blk1147_dn7 * locals.var_vgd_eff) + (locals.var_t3__blk1147 * locals.var_vgd_eff_dn7))) - ((((locals.var_t4__blk1148_dn7 * locals.var_vgd_eff) + (locals.var_t4__blk1148 * locals.var_vgd_eff_dn7)) * locals.var_vgd_eff) + (assign26670_e20558 * locals.var_vgd_eff_dn7))))), ((locals.var_t12_dn8 * assign26670_e20561) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn8 + ((locals.var_t3__blk1147_dn8 * locals.var_vgd_eff) + (locals.var_t3__blk1147 * locals.var_vgd_eff_dn8))) - ((((locals.var_t4__blk1148_dn8 * locals.var_vgd_eff) + (locals.var_t4__blk1148 * locals.var_vgd_eff_dn8)) * locals.var_vgd_eff) + (assign26670_e20558 * locals.var_vgd_eff_dn8))))), ((locals.var_t12_dn9 * assign26670_e20561) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn9 + ((locals.var_t3__blk1147_dn9 * locals.var_vgd_eff) + (locals.var_t3__blk1147 * locals.var_vgd_eff_dn9))) - ((((locals.var_t4__blk1148_dn9 * locals.var_vgd_eff) + (locals.var_t4__blk1148 * locals.var_vgd_eff_dn9)) * locals.var_vgd_eff) + (assign26670_e20558 * locals.var_vgd_eff_dn9))))), ((locals.var_t12_dn10 * assign26670_e20561) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn10 + ((locals.var_t3__blk1147_dn10 * locals.var_vgd_eff) + (locals.var_t3__blk1147 * locals.var_vgd_eff_dn10))) - ((((locals.var_t4__blk1148_dn10 * locals.var_vgd_eff) + (locals.var_t4__blk1148 * locals.var_vgd_eff_dn10)) * locals.var_vgd_eff) + (assign26670_e20558 * locals.var_vgd_eff_dn10))))), ((locals.var_t12_dn11 * assign26670_e20561) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn11 + ((locals.var_t3__blk1147_dn11 * locals.var_vgd_eff) + (locals.var_t3__blk1147 * locals.var_vgd_eff_dn11))) - ((((locals.var_t4__blk1148_dn11 * locals.var_vgd_eff) + (locals.var_t4__blk1148 * locals.var_vgd_eff_dn11)) * locals.var_vgd_eff) + (assign26670_e20558 * locals.var_vgd_eff_dn11))))), ((locals.var_t12_dn12 * assign26670_e20561) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn12 + ((locals.var_t3__blk1147_dn12 * locals.var_vgd_eff) + (locals.var_t3__blk1147 * locals.var_vgd_eff_dn12))) - ((((locals.var_t4__blk1148_dn12 * locals.var_vgd_eff) + (locals.var_t4__blk1148 * locals.var_vgd_eff_dn12)) * locals.var_vgd_eff) + (assign26670_e20558 * locals.var_vgd_eff_dn12))))),)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign26670_e20564;
        locals.var_t5__blk1149_dn3 = assign26670_e20564_d_n3;
        locals.var_t5__blk1149_dn4 = assign26670_e20564_d_n4;
        locals.var_t5__blk1149_dn5 = assign26670_e20564_d_n5;
        locals.var_t5__blk1149_dn6 = assign26670_e20564_d_n6;
        locals.var_t5__blk1149_dn7 = assign26670_e20564_d_n7;
        locals.var_t5__blk1149_dn8 = assign26670_e20564_d_n8;
        locals.var_t5__blk1149_dn9 = assign26670_e20564_d_n9;
        locals.var_t5__blk1149_dn10 = assign26670_e20564_d_n10;
        locals.var_t5__blk1149_dn11 = assign26670_e20564_d_n11;
        locals.var_t5__blk1149_dn12 = assign26670_e20564_d_n12;

        let assign26680_e20567: f64 = if locals.var_t5__blk1149 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1642 = assign26680_e20567;

        let (assign26690_e20573, assign26690_e20573_d_n3, assign26690_e20573_d_n4, assign26690_e20573_d_n5, assign26690_e20573_d_n6, assign26690_e20573_d_n7, assign26690_e20573_d_n8, assign26690_e20573_d_n9, assign26690_e20573_d_n10, assign26690_e20573_d_n11, assign26690_e20573_d_n12,) = {
    if ((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1642 != 0.0)) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign26690_e20573;
        locals.var_t6__blk1150_dn3 = assign26690_e20573_d_n3;
        locals.var_t6__blk1150_dn4 = assign26690_e20573_d_n4;
        locals.var_t6__blk1150_dn5 = assign26690_e20573_d_n5;
        locals.var_t6__blk1150_dn6 = assign26690_e20573_d_n6;
        locals.var_t6__blk1150_dn7 = assign26690_e20573_d_n7;
        locals.var_t6__blk1150_dn8 = assign26690_e20573_d_n8;
        locals.var_t6__blk1150_dn9 = assign26690_e20573_d_n9;
        locals.var_t6__blk1150_dn10 = assign26690_e20573_d_n10;
        locals.var_t6__blk1150_dn11 = assign26690_e20573_d_n11;
        locals.var_t6__blk1150_dn12 = assign26690_e20573_d_n12;

        let assign26700_e20576: f64 = (-100.0);
        let assign26700_e20577: f64 = if locals.var_t5__blk1149 < assign26700_e20576 { 1.0 } else { 0.0 };
        locals.var_guard1643 = assign26700_e20577;

        let (assign26710_e20586, assign26710_e20586_d_n3, assign26710_e20586_d_n4, assign26710_e20586_d_n5, assign26710_e20586_d_n6, assign26710_e20586_d_n7, assign26710_e20586_d_n8, assign26710_e20586_d_n9, assign26710_e20586_d_n10, assign26710_e20586_d_n11, assign26710_e20586_d_n12,) = {
    if (((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1642 == 0.0)) && (locals.var_guard1643 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign26710_e20586;
        locals.var_t6__blk1150_dn3 = assign26710_e20586_d_n3;
        locals.var_t6__blk1150_dn4 = assign26710_e20586_d_n4;
        locals.var_t6__blk1150_dn5 = assign26710_e20586_d_n5;
        locals.var_t6__blk1150_dn6 = assign26710_e20586_d_n6;
        locals.var_t6__blk1150_dn7 = assign26710_e20586_d_n7;
        locals.var_t6__blk1150_dn8 = assign26710_e20586_d_n8;
        locals.var_t6__blk1150_dn9 = assign26710_e20586_d_n9;
        locals.var_t6__blk1150_dn10 = assign26710_e20586_d_n10;
        locals.var_t6__blk1150_dn11 = assign26710_e20586_d_n11;
        locals.var_t6__blk1150_dn12 = assign26710_e20586_d_n12;

        let (assign26720_e20597, assign26720_e20597_d_n3, assign26720_e20597_d_n4, assign26720_e20597_d_n5, assign26720_e20597_d_n6, assign26720_e20597_d_n7, assign26720_e20597_d_n8, assign26720_e20597_d_n9, assign26720_e20597_d_n10, assign26720_e20597_d_n11, assign26720_e20597_d_n12,) = {
    if (((locals.var_b4soiigcmod != 0.0) && (locals.var_guard1642 == 0.0)) && (locals.var_guard1643 == 0.0)) {
        let assign26720_e20595: f64 = (locals.var_t5__blk1149).exp();
        (assign26720_e20595, (assign26720_e20595 * locals.var_t5__blk1149_dn3), (assign26720_e20595 * locals.var_t5__blk1149_dn4), (assign26720_e20595 * locals.var_t5__blk1149_dn5), (assign26720_e20595 * locals.var_t5__blk1149_dn6), (assign26720_e20595 * locals.var_t5__blk1149_dn7), (assign26720_e20595 * locals.var_t5__blk1149_dn8), (assign26720_e20595 * locals.var_t5__blk1149_dn9), (assign26720_e20595 * locals.var_t5__blk1149_dn10), (assign26720_e20595 * locals.var_t5__blk1149_dn11), (assign26720_e20595 * locals.var_t5__blk1149_dn12),)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign26720_e20597;
        locals.var_t6__blk1150_dn3 = assign26720_e20597_d_n3;
        locals.var_t6__blk1150_dn4 = assign26720_e20597_d_n4;
        locals.var_t6__blk1150_dn5 = assign26720_e20597_d_n5;
        locals.var_t6__blk1150_dn6 = assign26720_e20597_d_n6;
        locals.var_t6__blk1150_dn7 = assign26720_e20597_d_n7;
        locals.var_t6__blk1150_dn8 = assign26720_e20597_d_n8;
        locals.var_t6__blk1150_dn9 = assign26720_e20597_d_n9;
        locals.var_t6__blk1150_dn10 = assign26720_e20597_d_n10;
        locals.var_t6__blk1150_dn11 = assign26720_e20597_d_n11;
        locals.var_t6__blk1150_dn12 = assign26720_e20597_d_n12;

        let (assign26730_e20605, assign26730_e20605_d_n3, assign26730_e20605_d_n4, assign26730_e20605_d_n5, assign26730_e20605_d_n6, assign26730_e20605_d_n7, assign26730_e20605_d_n8, assign26730_e20605_d_n9, assign26730_e20605_d_n10, assign26730_e20605_d_n11, assign26730_e20605_d_n12,) = {
    if (locals.var_b4soiigcmod != 0.0) {
        let assign26730_e20601: f64 = (locals.var_t14 * locals.var_t2__blk1146);
        let assign26730_e20603: f64 = (assign26730_e20601 * locals.var_t6__blk1150);
        (assign26730_e20603, ((((locals.var_t14_dn3 * locals.var_t2__blk1146) + (locals.var_t14 * locals.var_t2__blk1146_dn3)) * locals.var_t6__blk1150) + (assign26730_e20601 * locals.var_t6__blk1150_dn3)), ((((locals.var_t14_dn4 * locals.var_t2__blk1146) + (locals.var_t14 * locals.var_t2__blk1146_dn4)) * locals.var_t6__blk1150) + (assign26730_e20601 * locals.var_t6__blk1150_dn4)), ((((locals.var_t14_dn5 * locals.var_t2__blk1146) + (locals.var_t14 * locals.var_t2__blk1146_dn5)) * locals.var_t6__blk1150) + (assign26730_e20601 * locals.var_t6__blk1150_dn5)), ((((locals.var_t14_dn6 * locals.var_t2__blk1146) + (locals.var_t14 * locals.var_t2__blk1146_dn6)) * locals.var_t6__blk1150) + (assign26730_e20601 * locals.var_t6__blk1150_dn6)), ((((locals.var_t14_dn7 * locals.var_t2__blk1146) + (locals.var_t14 * locals.var_t2__blk1146_dn7)) * locals.var_t6__blk1150) + (assign26730_e20601 * locals.var_t6__blk1150_dn7)), ((((locals.var_t14_dn8 * locals.var_t2__blk1146) + (locals.var_t14 * locals.var_t2__blk1146_dn8)) * locals.var_t6__blk1150) + (assign26730_e20601 * locals.var_t6__blk1150_dn8)), ((((locals.var_t14_dn9 * locals.var_t2__blk1146) + (locals.var_t14 * locals.var_t2__blk1146_dn9)) * locals.var_t6__blk1150) + (assign26730_e20601 * locals.var_t6__blk1150_dn9)), ((((locals.var_t14_dn10 * locals.var_t2__blk1146) + (locals.var_t14 * locals.var_t2__blk1146_dn10)) * locals.var_t6__blk1150) + (assign26730_e20601 * locals.var_t6__blk1150_dn10)), ((((locals.var_t14_dn11 * locals.var_t2__blk1146) + (locals.var_t14 * locals.var_t2__blk1146_dn11)) * locals.var_t6__blk1150) + (assign26730_e20601 * locals.var_t6__blk1150_dn11)), ((((locals.var_t14_dn12 * locals.var_t2__blk1146) + (locals.var_t14 * locals.var_t2__blk1146_dn12)) * locals.var_t6__blk1150) + (assign26730_e20601 * locals.var_t6__blk1150_dn12)),)
    } else {
        (locals.var_igd_1, locals.var_igd_1_dn3, locals.var_igd_1_dn4, locals.var_igd_1_dn5, locals.var_igd_1_dn6, locals.var_igd_1_dn7, locals.var_igd_1_dn8, locals.var_igd_1_dn9, locals.var_igd_1_dn10, locals.var_igd_1_dn11, locals.var_igd_1_dn12,)
    }
};
        locals.var_igd_1 = assign26730_e20605;
        locals.var_igd_1_dn3 = assign26730_e20605_d_n3;
        locals.var_igd_1_dn4 = assign26730_e20605_d_n4;
        locals.var_igd_1_dn5 = assign26730_e20605_d_n5;
        locals.var_igd_1_dn6 = assign26730_e20605_d_n6;
        locals.var_igd_1_dn7 = assign26730_e20605_d_n7;
        locals.var_igd_1_dn8 = assign26730_e20605_d_n8;
        locals.var_igd_1_dn9 = assign26730_e20605_d_n9;
        locals.var_igd_1_dn10 = assign26730_e20605_d_n10;
        locals.var_igd_1_dn11 = assign26730_e20605_d_n11;
        locals.var_igd_1_dn12 = assign26730_e20605_d_n12;

        let (assign26740_e20610, assign26740_e20610_d_n3, assign26740_e20610_d_n4, assign26740_e20610_d_n5, assign26740_e20610_d_n6, assign26740_e20610_d_n7, assign26740_e20610_d_n8, assign26740_e20610_d_n9, assign26740_e20610_d_n10, assign26740_e20610_d_n11, assign26740_e20610_d_n12,) = {
    if (locals.var_b4soiigcmod == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igd_1, locals.var_igd_1_dn3, locals.var_igd_1_dn4, locals.var_igd_1_dn5, locals.var_igd_1_dn6, locals.var_igd_1_dn7, locals.var_igd_1_dn8, locals.var_igd_1_dn9, locals.var_igd_1_dn10, locals.var_igd_1_dn11, locals.var_igd_1_dn12,)
    }
};
        locals.var_igd_1 = assign26740_e20610;
        locals.var_igd_1_dn3 = assign26740_e20610_d_n3;
        locals.var_igd_1_dn4 = assign26740_e20610_d_n4;
        locals.var_igd_1_dn5 = assign26740_e20610_d_n5;
        locals.var_igd_1_dn6 = assign26740_e20610_d_n6;
        locals.var_igd_1_dn7 = assign26740_e20610_d_n7;
        locals.var_igd_1_dn8 = assign26740_e20610_d_n8;
        locals.var_igd_1_dn9 = assign26740_e20610_d_n9;
        locals.var_igd_1_dn10 = assign26740_e20610_d_n10;
        locals.var_igd_1_dn11 = assign26740_e20610_d_n11;
        locals.var_igd_1_dn12 = assign26740_e20610_d_n12;

        let (assign26750_e20615, assign26750_e20615_d_n3, assign26750_e20615_d_n4, assign26750_e20615_d_n5, assign26750_e20615_d_n6, assign26750_e20615_d_n7, assign26750_e20615_d_n8, assign26750_e20615_d_n9, assign26750_e20615_d_n10, assign26750_e20615_d_n11, assign26750_e20615_d_n12,) = {
    if (locals.var_b4soiigcmod == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igs_1, locals.var_igs_1_dn3, locals.var_igs_1_dn4, locals.var_igs_1_dn5, locals.var_igs_1_dn6, locals.var_igs_1_dn7, locals.var_igs_1_dn8, locals.var_igs_1_dn9, locals.var_igs_1_dn10, locals.var_igs_1_dn11, locals.var_igs_1_dn12,)
    }
};
        locals.var_igs_1 = assign26750_e20615;
        locals.var_igs_1_dn3 = assign26750_e20615_d_n3;
        locals.var_igs_1_dn4 = assign26750_e20615_d_n4;
        locals.var_igs_1_dn5 = assign26750_e20615_d_n5;
        locals.var_igs_1_dn6 = assign26750_e20615_d_n6;
        locals.var_igs_1_dn7 = assign26750_e20615_d_n7;
        locals.var_igs_1_dn8 = assign26750_e20615_d_n8;
        locals.var_igs_1_dn9 = assign26750_e20615_d_n9;
        locals.var_igs_1_dn10 = assign26750_e20615_d_n10;
        locals.var_igs_1_dn11 = assign26750_e20615_d_n11;
        locals.var_igs_1_dn12 = assign26750_e20615_d_n12;

        let (assign26760_e20620, assign26760_e20620_d_n3, assign26760_e20620_d_n4, assign26760_e20620_d_n5, assign26760_e20620_d_n6, assign26760_e20620_d_n7, assign26760_e20620_d_n8, assign26760_e20620_d_n9, assign26760_e20620_d_n10, assign26760_e20620_d_n11, assign26760_e20620_d_n12,) = {
    if (locals.var_b4soiigcmod == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igcd_1, locals.var_igcd_1_dn3, locals.var_igcd_1_dn4, locals.var_igcd_1_dn5, locals.var_igcd_1_dn6, locals.var_igcd_1_dn7, locals.var_igcd_1_dn8, locals.var_igcd_1_dn9, locals.var_igcd_1_dn10, locals.var_igcd_1_dn11, locals.var_igcd_1_dn12,)
    }
};
        locals.var_igcd_1 = assign26760_e20620;
        locals.var_igcd_1_dn3 = assign26760_e20620_d_n3;
        locals.var_igcd_1_dn4 = assign26760_e20620_d_n4;
        locals.var_igcd_1_dn5 = assign26760_e20620_d_n5;
        locals.var_igcd_1_dn6 = assign26760_e20620_d_n6;
        locals.var_igcd_1_dn7 = assign26760_e20620_d_n7;
        locals.var_igcd_1_dn8 = assign26760_e20620_d_n8;
        locals.var_igcd_1_dn9 = assign26760_e20620_d_n9;
        locals.var_igcd_1_dn10 = assign26760_e20620_d_n10;
        locals.var_igcd_1_dn11 = assign26760_e20620_d_n11;
        locals.var_igcd_1_dn12 = assign26760_e20620_d_n12;

        let (assign26770_e20625, assign26770_e20625_d_n3, assign26770_e20625_d_n4, assign26770_e20625_d_n5, assign26770_e20625_d_n6, assign26770_e20625_d_n7, assign26770_e20625_d_n8, assign26770_e20625_d_n9, assign26770_e20625_d_n10, assign26770_e20625_d_n11, assign26770_e20625_d_n12,) = {
    if (locals.var_b4soiigcmod == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igcs_1, locals.var_igcs_1_dn3, locals.var_igcs_1_dn4, locals.var_igcs_1_dn5, locals.var_igcs_1_dn6, locals.var_igcs_1_dn7, locals.var_igcs_1_dn8, locals.var_igcs_1_dn9, locals.var_igcs_1_dn10, locals.var_igcs_1_dn11, locals.var_igcs_1_dn12,)
    }
};
        locals.var_igcs_1 = assign26770_e20625;
        locals.var_igcs_1_dn3 = assign26770_e20625_d_n3;
        locals.var_igcs_1_dn4 = assign26770_e20625_d_n4;
        locals.var_igcs_1_dn5 = assign26770_e20625_d_n5;
        locals.var_igcs_1_dn6 = assign26770_e20625_d_n6;
        locals.var_igcs_1_dn7 = assign26770_e20625_d_n7;
        locals.var_igcs_1_dn8 = assign26770_e20625_d_n8;
        locals.var_igcs_1_dn9 = assign26770_e20625_d_n9;
        locals.var_igcs_1_dn10 = assign26770_e20625_d_n10;
        locals.var_igcs_1_dn11 = assign26770_e20625_d_n11;
        locals.var_igcs_1_dn12 = assign26770_e20625_d_n12;

        let assign26780_e20632: f64 = if ((locals.var_b4soiigbmod != 0.0) && (locals.var_b4soisoimod != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard1644 = assign26780_e20632;

        let (assign26790_e20636,) = {
    if (locals.var_guard1644 != 0.0) {
        (locals.var_pparam_b4soioxideratio,)
    } else {
        (locals.var_oxideratio,)
    }
};
        locals.var_oxideratio = assign26790_e20636;

        let (assign26800_e20640, assign26800_e20640_d_n3, assign26800_e20640_d_n4, assign26800_e20640_d_n5, assign26800_e20640_d_n6, assign26800_e20640_d_n7, assign26800_e20640_d_n8, assign26800_e20640_d_n9, assign26800_e20640_d_n10, assign26800_e20640_d_n11, assign26800_e20640_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        (locals.var_voxdepinv, locals.var_voxdepinv_dn3, locals.var_voxdepinv_dn4, locals.var_voxdepinv_dn5, locals.var_voxdepinv_dn6, locals.var_voxdepinv_dn7, locals.var_voxdepinv_dn8, locals.var_voxdepinv_dn9, locals.var_voxdepinv_dn10, locals.var_voxdepinv_dn11, locals.var_voxdepinv_dn12,)
    } else {
        (locals.var_vox, locals.var_vox_dn3, locals.var_vox_dn4, locals.var_vox_dn5, locals.var_vox_dn6, locals.var_vox_dn7, locals.var_vox_dn8, locals.var_vox_dn9, locals.var_vox_dn10, locals.var_vox_dn11, locals.var_vox_dn12,)
    }
};
        locals.var_vox = assign26800_e20640;
        locals.var_vox_dn3 = assign26800_e20640_d_n3;
        locals.var_vox_dn4 = assign26800_e20640_d_n4;
        locals.var_vox_dn5 = assign26800_e20640_d_n5;
        locals.var_vox_dn6 = assign26800_e20640_d_n6;
        locals.var_vox_dn7 = assign26800_e20640_d_n7;
        locals.var_vox_dn8 = assign26800_e20640_d_n8;
        locals.var_vox_dn9 = assign26800_e20640_d_n9;
        locals.var_vox_dn10 = assign26800_e20640_d_n10;
        locals.var_vox_dn11 = assign26800_e20640_d_n11;
        locals.var_vox_dn12 = assign26800_e20640_d_n12;

        let (assign26810_e20644, assign26810_e20644_d_n3, assign26810_e20644_d_n4, assign26810_e20644_d_n5, assign26810_e20644_d_n6, assign26810_e20644_d_n7, assign26810_e20644_d_n8, assign26810_e20644_d_n9, assign26810_e20644_d_n10, assign26810_e20644_d_n11, assign26810_e20644_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        (locals.var_b4soivoxh, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign26810_e20644;
        locals.var_t0__blk1144_dn3 = assign26810_e20644_d_n3;
        locals.var_t0__blk1144_dn4 = assign26810_e20644_d_n4;
        locals.var_t0__blk1144_dn5 = assign26810_e20644_d_n5;
        locals.var_t0__blk1144_dn6 = assign26810_e20644_d_n6;
        locals.var_t0__blk1144_dn7 = assign26810_e20644_d_n7;
        locals.var_t0__blk1144_dn8 = assign26810_e20644_d_n8;
        locals.var_t0__blk1144_dn9 = assign26810_e20644_d_n9;
        locals.var_t0__blk1144_dn10 = assign26810_e20644_d_n10;
        locals.var_t0__blk1144_dn11 = assign26810_e20644_d_n11;
        locals.var_t0__blk1144_dn12 = assign26810_e20644_d_n12;

        let (assign26820_e20652, assign26820_e20652_d_n3, assign26820_e20652_d_n4, assign26820_e20652_d_n5, assign26820_e20652_d_n6, assign26820_e20652_d_n7, assign26820_e20652_d_n8, assign26820_e20652_d_n9, assign26820_e20652_d_n10, assign26820_e20652_d_n11, assign26820_e20652_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign26820_e20648: f64 = (locals.var_t0__blk1144 - locals.var_vox);
        let assign26820_e20650: f64 = (assign26820_e20648 - locals.var_b4soideltavox);
        (assign26820_e20650, (locals.var_t0__blk1144_dn3 - locals.var_vox_dn3), (locals.var_t0__blk1144_dn4 - locals.var_vox_dn4), (locals.var_t0__blk1144_dn5 - locals.var_vox_dn5), (locals.var_t0__blk1144_dn6 - locals.var_vox_dn6), (locals.var_t0__blk1144_dn7 - locals.var_vox_dn7), (locals.var_t0__blk1144_dn8 - locals.var_vox_dn8), (locals.var_t0__blk1144_dn9 - locals.var_vox_dn9), (locals.var_t0__blk1144_dn10 - locals.var_vox_dn10), (locals.var_t0__blk1144_dn11 - locals.var_vox_dn11), (locals.var_t0__blk1144_dn12 - locals.var_vox_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign26820_e20652;
        locals.var_t1__blk1145_dn3 = assign26820_e20652_d_n3;
        locals.var_t1__blk1145_dn4 = assign26820_e20652_d_n4;
        locals.var_t1__blk1145_dn5 = assign26820_e20652_d_n5;
        locals.var_t1__blk1145_dn6 = assign26820_e20652_d_n6;
        locals.var_t1__blk1145_dn7 = assign26820_e20652_d_n7;
        locals.var_t1__blk1145_dn8 = assign26820_e20652_d_n8;
        locals.var_t1__blk1145_dn9 = assign26820_e20652_d_n9;
        locals.var_t1__blk1145_dn10 = assign26820_e20652_d_n10;
        locals.var_t1__blk1145_dn11 = assign26820_e20652_d_n11;
        locals.var_t1__blk1145_dn12 = assign26820_e20652_d_n12;

        let (assign26830_e20665, assign26830_e20665_d_n3, assign26830_e20665_d_n4, assign26830_e20665_d_n5, assign26830_e20665_d_n6, assign26830_e20665_d_n7, assign26830_e20665_d_n8, assign26830_e20665_d_n9, assign26830_e20665_d_n10, assign26830_e20665_d_n11, assign26830_e20665_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign26830_e20656: f64 = (locals.var_t1__blk1145 * locals.var_t1__blk1145);
        let assign26830_e20659: f64 = (4.0 * locals.var_b4soideltavox);
        let assign26830_e20661: f64 = (assign26830_e20659 * locals.var_t0__blk1144);
        let assign26830_e20662: f64 = (assign26830_e20656 + assign26830_e20661);
        let assign26830_e20663: f64 = (assign26830_e20662).sqrt();
        (assign26830_e20663, ((((locals.var_t1__blk1145_dn3 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn3)) + (assign26830_e20659 * locals.var_t0__blk1144_dn3)) / (2.0 * assign26830_e20663)), ((((locals.var_t1__blk1145_dn4 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn4)) + (assign26830_e20659 * locals.var_t0__blk1144_dn4)) / (2.0 * assign26830_e20663)), ((((locals.var_t1__blk1145_dn5 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn5)) + (assign26830_e20659 * locals.var_t0__blk1144_dn5)) / (2.0 * assign26830_e20663)), ((((locals.var_t1__blk1145_dn6 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn6)) + (assign26830_e20659 * locals.var_t0__blk1144_dn6)) / (2.0 * assign26830_e20663)), ((((locals.var_t1__blk1145_dn7 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn7)) + (assign26830_e20659 * locals.var_t0__blk1144_dn7)) / (2.0 * assign26830_e20663)), ((((locals.var_t1__blk1145_dn8 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn8)) + (assign26830_e20659 * locals.var_t0__blk1144_dn8)) / (2.0 * assign26830_e20663)), ((((locals.var_t1__blk1145_dn9 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn9)) + (assign26830_e20659 * locals.var_t0__blk1144_dn9)) / (2.0 * assign26830_e20663)), ((((locals.var_t1__blk1145_dn10 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn10)) + (assign26830_e20659 * locals.var_t0__blk1144_dn10)) / (2.0 * assign26830_e20663)), ((((locals.var_t1__blk1145_dn11 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn11)) + (assign26830_e20659 * locals.var_t0__blk1144_dn11)) / (2.0 * assign26830_e20663)), ((((locals.var_t1__blk1145_dn12 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn12)) + (assign26830_e20659 * locals.var_t0__blk1144_dn12)) / (2.0 * assign26830_e20663)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign26830_e20665;
        locals.var_t3__blk1147_dn3 = assign26830_e20665_d_n3;
        locals.var_t3__blk1147_dn4 = assign26830_e20665_d_n4;
        locals.var_t3__blk1147_dn5 = assign26830_e20665_d_n5;
        locals.var_t3__blk1147_dn6 = assign26830_e20665_d_n6;
        locals.var_t3__blk1147_dn7 = assign26830_e20665_d_n7;
        locals.var_t3__blk1147_dn8 = assign26830_e20665_d_n8;
        locals.var_t3__blk1147_dn9 = assign26830_e20665_d_n9;
        locals.var_t3__blk1147_dn10 = assign26830_e20665_d_n10;
        locals.var_t3__blk1147_dn11 = assign26830_e20665_d_n11;
        locals.var_t3__blk1147_dn12 = assign26830_e20665_d_n12;

    }

    pub(super) fn stamp_transient_block_70(
        locals: &mut StampLocals,
    ) {
        let (assign26840_e20675, assign26840_e20675_d_n3, assign26840_e20675_d_n4, assign26840_e20675_d_n5, assign26840_e20675_d_n6, assign26840_e20675_d_n7, assign26840_e20675_d_n8, assign26840_e20675_d_n9, assign26840_e20675_d_n10, assign26840_e20675_d_n11, assign26840_e20675_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign26840_e20671: f64 = (locals.var_t1__blk1145 + locals.var_t3__blk1147);
        let assign26840_e20672: f64 = (0.5 * assign26840_e20671);
        let assign26840_e20673: f64 = (locals.var_t0__blk1144 - assign26840_e20672);
        (assign26840_e20673, (locals.var_t0__blk1144_dn3 - (0.5 * (locals.var_t1__blk1145_dn3 + locals.var_t3__blk1147_dn3))), (locals.var_t0__blk1144_dn4 - (0.5 * (locals.var_t1__blk1145_dn4 + locals.var_t3__blk1147_dn4))), (locals.var_t0__blk1144_dn5 - (0.5 * (locals.var_t1__blk1145_dn5 + locals.var_t3__blk1147_dn5))), (locals.var_t0__blk1144_dn6 - (0.5 * (locals.var_t1__blk1145_dn6 + locals.var_t3__blk1147_dn6))), (locals.var_t0__blk1144_dn7 - (0.5 * (locals.var_t1__blk1145_dn7 + locals.var_t3__blk1147_dn7))), (locals.var_t0__blk1144_dn8 - (0.5 * (locals.var_t1__blk1145_dn8 + locals.var_t3__blk1147_dn8))), (locals.var_t0__blk1144_dn9 - (0.5 * (locals.var_t1__blk1145_dn9 + locals.var_t3__blk1147_dn9))), (locals.var_t0__blk1144_dn10 - (0.5 * (locals.var_t1__blk1145_dn10 + locals.var_t3__blk1147_dn10))), (locals.var_t0__blk1144_dn11 - (0.5 * (locals.var_t1__blk1145_dn11 + locals.var_t3__blk1147_dn11))), (locals.var_t0__blk1144_dn12 - (0.5 * (locals.var_t1__blk1145_dn12 + locals.var_t3__blk1147_dn12))),)
    } else {
        (locals.var_voxeff, locals.var_voxeff_dn3, locals.var_voxeff_dn4, locals.var_voxeff_dn5, locals.var_voxeff_dn6, locals.var_voxeff_dn7, locals.var_voxeff_dn8, locals.var_voxeff_dn9, locals.var_voxeff_dn10, locals.var_voxeff_dn11, locals.var_voxeff_dn12,)
    }
};
        locals.var_voxeff = assign26840_e20675;
        locals.var_voxeff_dn3 = assign26840_e20675_d_n3;
        locals.var_voxeff_dn4 = assign26840_e20675_d_n4;
        locals.var_voxeff_dn5 = assign26840_e20675_d_n5;
        locals.var_voxeff_dn6 = assign26840_e20675_d_n6;
        locals.var_voxeff_dn7 = assign26840_e20675_d_n7;
        locals.var_voxeff_dn8 = assign26840_e20675_d_n8;
        locals.var_voxeff_dn9 = assign26840_e20675_d_n9;
        locals.var_voxeff_dn10 = assign26840_e20675_d_n10;
        locals.var_voxeff_dn11 = assign26840_e20675_d_n11;
        locals.var_voxeff_dn12 = assign26840_e20675_d_n12;

        let (assign26850_e20679, assign26850_e20679_d_n3, assign26850_e20679_d_n4, assign26850_e20679_d_n5, assign26850_e20679_d_n6, assign26850_e20679_d_n7, assign26850_e20679_d_n8, assign26850_e20679_d_n9, assign26850_e20679_d_n10, assign26850_e20679_d_n11, assign26850_e20679_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        (locals.var_voxeff, locals.var_voxeff_dn3, locals.var_voxeff_dn4, locals.var_voxeff_dn5, locals.var_voxeff_dn6, locals.var_voxeff_dn7, locals.var_voxeff_dn8, locals.var_voxeff_dn9, locals.var_voxeff_dn10, locals.var_voxeff_dn11, locals.var_voxeff_dn12,)
    } else {
        (locals.var_vox, locals.var_vox_dn3, locals.var_vox_dn4, locals.var_vox_dn5, locals.var_vox_dn6, locals.var_vox_dn7, locals.var_vox_dn8, locals.var_vox_dn9, locals.var_vox_dn10, locals.var_vox_dn11, locals.var_vox_dn12,)
    }
};
        locals.var_vox = assign26850_e20679;
        locals.var_vox_dn3 = assign26850_e20679_d_n3;
        locals.var_vox_dn4 = assign26850_e20679_d_n4;
        locals.var_vox_dn5 = assign26850_e20679_d_n5;
        locals.var_vox_dn6 = assign26850_e20679_d_n6;
        locals.var_vox_dn7 = assign26850_e20679_d_n7;
        locals.var_vox_dn8 = assign26850_e20679_d_n8;
        locals.var_vox_dn9 = assign26850_e20679_d_n9;
        locals.var_vox_dn10 = assign26850_e20679_d_n10;
        locals.var_vox_dn11 = assign26850_e20679_d_n11;
        locals.var_vox_dn12 = assign26850_e20679_d_n12;

        let (assign26860_e20687, assign26860_e20687_d_n3, assign26860_e20687_d_n4, assign26860_e20687_d_n5, assign26860_e20687_d_n6, assign26860_e20687_d_n7, assign26860_e20687_d_n8, assign26860_e20687_d_n9, assign26860_e20687_d_n10, assign26860_e20687_d_n11, assign26860_e20687_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign26860_e20683: f64 = (locals.var_vox - locals.var_b4soiebg);
        let assign26860_e20685: f64 = (assign26860_e20683 / locals.var_b4soivevb);
        (assign26860_e20685, (locals.var_vox_dn3 / locals.var_b4soivevb), (locals.var_vox_dn4 / locals.var_b4soivevb), (locals.var_vox_dn5 / locals.var_b4soivevb), (locals.var_vox_dn6 / locals.var_b4soivevb), (locals.var_vox_dn7 / locals.var_b4soivevb), (locals.var_vox_dn8 / locals.var_b4soivevb), (locals.var_vox_dn9 / locals.var_b4soivevb), (locals.var_vox_dn10 / locals.var_b4soivevb), (locals.var_vox_dn11 / locals.var_b4soivevb), (locals.var_vox_dn12 / locals.var_b4soivevb),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign26860_e20687;
        locals.var_t0__blk1144_dn3 = assign26860_e20687_d_n3;
        locals.var_t0__blk1144_dn4 = assign26860_e20687_d_n4;
        locals.var_t0__blk1144_dn5 = assign26860_e20687_d_n5;
        locals.var_t0__blk1144_dn6 = assign26860_e20687_d_n6;
        locals.var_t0__blk1144_dn7 = assign26860_e20687_d_n7;
        locals.var_t0__blk1144_dn8 = assign26860_e20687_d_n8;
        locals.var_t0__blk1144_dn9 = assign26860_e20687_d_n9;
        locals.var_t0__blk1144_dn10 = assign26860_e20687_d_n10;
        locals.var_t0__blk1144_dn11 = assign26860_e20687_d_n11;
        locals.var_t0__blk1144_dn12 = assign26860_e20687_d_n12;

        let assign26870_e20690: f64 = if locals.var_t0__blk1144 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1645 = assign26870_e20690;

        let (assign26880_e20702, assign26880_e20702_d_n3, assign26880_e20702_d_n4, assign26880_e20702_d_n5, assign26880_e20702_d_n6, assign26880_e20702_d_n7, assign26880_e20702_d_n8, assign26880_e20702_d_n9, assign26880_e20702_d_n10, assign26880_e20702_d_n11, assign26880_e20702_d_n12,) = {
    if ((locals.var_guard1644 != 0.0) && (locals.var_guard1645 != 0.0)) {
        let assign26880_e20697: f64 = (1.0 + locals.var_t0__blk1144);
        let assign26880_e20699: f64 = (assign26880_e20697 - 100.0);
        let assign26880_e20700: f64 = (2.688117142e43 * assign26880_e20699);
        (assign26880_e20700, (2.688117142e43 * locals.var_t0__blk1144_dn3), (2.688117142e43 * locals.var_t0__blk1144_dn4), (2.688117142e43 * locals.var_t0__blk1144_dn5), (2.688117142e43 * locals.var_t0__blk1144_dn6), (2.688117142e43 * locals.var_t0__blk1144_dn7), (2.688117142e43 * locals.var_t0__blk1144_dn8), (2.688117142e43 * locals.var_t0__blk1144_dn9), (2.688117142e43 * locals.var_t0__blk1144_dn10), (2.688117142e43 * locals.var_t0__blk1144_dn11), (2.688117142e43 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign26880_e20702;
        locals.var_t1__blk1145_dn3 = assign26880_e20702_d_n3;
        locals.var_t1__blk1145_dn4 = assign26880_e20702_d_n4;
        locals.var_t1__blk1145_dn5 = assign26880_e20702_d_n5;
        locals.var_t1__blk1145_dn6 = assign26880_e20702_d_n6;
        locals.var_t1__blk1145_dn7 = assign26880_e20702_d_n7;
        locals.var_t1__blk1145_dn8 = assign26880_e20702_d_n8;
        locals.var_t1__blk1145_dn9 = assign26880_e20702_d_n9;
        locals.var_t1__blk1145_dn10 = assign26880_e20702_d_n10;
        locals.var_t1__blk1145_dn11 = assign26880_e20702_d_n11;
        locals.var_t1__blk1145_dn12 = assign26880_e20702_d_n12;

        let assign26890_e20705: f64 = (-100.0);
        let assign26890_e20706: f64 = if locals.var_t0__blk1144 < assign26890_e20705 { 1.0 } else { 0.0 };
        locals.var_guard1646 = assign26890_e20706;

        let (assign26900_e20715, assign26900_e20715_d_n3, assign26900_e20715_d_n4, assign26900_e20715_d_n5, assign26900_e20715_d_n6, assign26900_e20715_d_n7, assign26900_e20715_d_n8, assign26900_e20715_d_n9, assign26900_e20715_d_n10, assign26900_e20715_d_n11, assign26900_e20715_d_n12,) = {
    if (((locals.var_guard1644 != 0.0) && (locals.var_guard1645 == 0.0)) && (locals.var_guard1646 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign26900_e20715;
        locals.var_t1__blk1145_dn3 = assign26900_e20715_d_n3;
        locals.var_t1__blk1145_dn4 = assign26900_e20715_d_n4;
        locals.var_t1__blk1145_dn5 = assign26900_e20715_d_n5;
        locals.var_t1__blk1145_dn6 = assign26900_e20715_d_n6;
        locals.var_t1__blk1145_dn7 = assign26900_e20715_d_n7;
        locals.var_t1__blk1145_dn8 = assign26900_e20715_d_n8;
        locals.var_t1__blk1145_dn9 = assign26900_e20715_d_n9;
        locals.var_t1__blk1145_dn10 = assign26900_e20715_d_n10;
        locals.var_t1__blk1145_dn11 = assign26900_e20715_d_n11;
        locals.var_t1__blk1145_dn12 = assign26900_e20715_d_n12;

        let (assign26910_e20726, assign26910_e20726_d_n3, assign26910_e20726_d_n4, assign26910_e20726_d_n5, assign26910_e20726_d_n6, assign26910_e20726_d_n7, assign26910_e20726_d_n8, assign26910_e20726_d_n9, assign26910_e20726_d_n10, assign26910_e20726_d_n11, assign26910_e20726_d_n12,) = {
    if (((locals.var_guard1644 != 0.0) && (locals.var_guard1645 == 0.0)) && (locals.var_guard1646 == 0.0)) {
        let assign26910_e20724: f64 = (locals.var_t0__blk1144).exp();
        (assign26910_e20724, (assign26910_e20724 * locals.var_t0__blk1144_dn3), (assign26910_e20724 * locals.var_t0__blk1144_dn4), (assign26910_e20724 * locals.var_t0__blk1144_dn5), (assign26910_e20724 * locals.var_t0__blk1144_dn6), (assign26910_e20724 * locals.var_t0__blk1144_dn7), (assign26910_e20724 * locals.var_t0__blk1144_dn8), (assign26910_e20724 * locals.var_t0__blk1144_dn9), (assign26910_e20724 * locals.var_t0__blk1144_dn10), (assign26910_e20724 * locals.var_t0__blk1144_dn11), (assign26910_e20724 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign26910_e20726;
        locals.var_t1__blk1145_dn3 = assign26910_e20726_d_n3;
        locals.var_t1__blk1145_dn4 = assign26910_e20726_d_n4;
        locals.var_t1__blk1145_dn5 = assign26910_e20726_d_n5;
        locals.var_t1__blk1145_dn6 = assign26910_e20726_d_n6;
        locals.var_t1__blk1145_dn7 = assign26910_e20726_d_n7;
        locals.var_t1__blk1145_dn8 = assign26910_e20726_d_n8;
        locals.var_t1__blk1145_dn9 = assign26910_e20726_d_n9;
        locals.var_t1__blk1145_dn10 = assign26910_e20726_d_n10;
        locals.var_t1__blk1145_dn11 = assign26910_e20726_d_n11;
        locals.var_t1__blk1145_dn12 = assign26910_e20726_d_n12;

        let (assign26920_e20735, assign26920_e20735_d_n3, assign26920_e20735_d_n4, assign26920_e20735_d_n5, assign26920_e20735_d_n6, assign26920_e20735_d_n7, assign26920_e20735_d_n8, assign26920_e20735_d_n9, assign26920_e20735_d_n10, assign26920_e20735_d_n11, assign26920_e20735_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign26920_e20731: f64 = (1.0 + locals.var_t1__blk1145);
        let assign26920_e20732: f64 = (assign26920_e20731).ln();
        let assign26920_e20733: f64 = (locals.var_b4soivevb * assign26920_e20732);
        (assign26920_e20733, (locals.var_b4soivevb * (locals.var_t1__blk1145_dn3 / assign26920_e20731)), (locals.var_b4soivevb * (locals.var_t1__blk1145_dn4 / assign26920_e20731)), (locals.var_b4soivevb * (locals.var_t1__blk1145_dn5 / assign26920_e20731)), (locals.var_b4soivevb * (locals.var_t1__blk1145_dn6 / assign26920_e20731)), (locals.var_b4soivevb * (locals.var_t1__blk1145_dn7 / assign26920_e20731)), (locals.var_b4soivevb * (locals.var_t1__blk1145_dn8 / assign26920_e20731)), (locals.var_b4soivevb * (locals.var_t1__blk1145_dn9 / assign26920_e20731)), (locals.var_b4soivevb * (locals.var_t1__blk1145_dn10 / assign26920_e20731)), (locals.var_b4soivevb * (locals.var_t1__blk1145_dn11 / assign26920_e20731)), (locals.var_b4soivevb * (locals.var_t1__blk1145_dn12 / assign26920_e20731)),)
    } else {
        (locals.var_vaux, locals.var_vaux_dn3, locals.var_vaux_dn4, locals.var_vaux_dn5, locals.var_vaux_dn6, locals.var_vaux_dn7, locals.var_vaux_dn8, locals.var_vaux_dn9, locals.var_vaux_dn10, locals.var_vaux_dn11, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign26920_e20735;
        locals.var_vaux_dn3 = assign26920_e20735_d_n3;
        locals.var_vaux_dn4 = assign26920_e20735_d_n4;
        locals.var_vaux_dn5 = assign26920_e20735_d_n5;
        locals.var_vaux_dn6 = assign26920_e20735_d_n6;
        locals.var_vaux_dn7 = assign26920_e20735_d_n7;
        locals.var_vaux_dn8 = assign26920_e20735_d_n8;
        locals.var_vaux_dn9 = assign26920_e20735_d_n9;
        locals.var_vaux_dn10 = assign26920_e20735_d_n10;
        locals.var_vaux_dn11 = assign26920_e20735_d_n11;
        locals.var_vaux_dn12 = assign26920_e20735_d_n12;

        let assign26930_e20738: f64 = if locals.var_b4soivgb1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1647 = assign26930_e20738;

        let (assign26940_e20748, assign26940_e20748_d_n3, assign26940_e20748_d_n4, assign26940_e20748_d_n5, assign26940_e20748_d_n6, assign26940_e20748_d_n7, assign26940_e20748_d_n8, assign26940_e20748_d_n9, assign26940_e20748_d_n10, assign26940_e20748_d_n11, assign26940_e20748_d_n12,) = {
    if ((locals.var_guard1644 != 0.0) && (locals.var_guard1647 != 0.0)) {
        let assign26940_e20745: f64 = (locals.var_vox / locals.var_b4soivgb1);
        let assign26940_e20746: f64 = (1.0 - assign26940_e20745);
        (assign26940_e20746, (-(locals.var_vox_dn3 / locals.var_b4soivgb1)), (-(locals.var_vox_dn4 / locals.var_b4soivgb1)), (-(locals.var_vox_dn5 / locals.var_b4soivgb1)), (-(locals.var_vox_dn6 / locals.var_b4soivgb1)), (-(locals.var_vox_dn7 / locals.var_b4soivgb1)), (-(locals.var_vox_dn8 / locals.var_b4soivgb1)), (-(locals.var_vox_dn9 / locals.var_b4soivgb1)), (-(locals.var_vox_dn10 / locals.var_b4soivgb1)), (-(locals.var_vox_dn11 / locals.var_b4soivgb1)), (-(locals.var_vox_dn12 / locals.var_b4soivgb1)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign26940_e20748;
        locals.var_t0__blk1144_dn3 = assign26940_e20748_d_n3;
        locals.var_t0__blk1144_dn4 = assign26940_e20748_d_n4;
        locals.var_t0__blk1144_dn5 = assign26940_e20748_d_n5;
        locals.var_t0__blk1144_dn6 = assign26940_e20748_d_n6;
        locals.var_t0__blk1144_dn7 = assign26940_e20748_d_n7;
        locals.var_t0__blk1144_dn8 = assign26940_e20748_d_n8;
        locals.var_t0__blk1144_dn9 = assign26940_e20748_d_n9;
        locals.var_t0__blk1144_dn10 = assign26940_e20748_d_n10;
        locals.var_t0__blk1144_dn11 = assign26940_e20748_d_n11;
        locals.var_t0__blk1144_dn12 = assign26940_e20748_d_n12;

        let (assign26950_e20755, assign26950_e20755_d_n3, assign26950_e20755_d_n4, assign26950_e20755_d_n5, assign26950_e20755_d_n6, assign26950_e20755_d_n7, assign26950_e20755_d_n8, assign26950_e20755_d_n9, assign26950_e20755_d_n10, assign26950_e20755_d_n11, assign26950_e20755_d_n12,) = {
    if ((locals.var_guard1644 != 0.0) && (locals.var_guard1647 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign26950_e20755;
        locals.var_t0__blk1144_dn3 = assign26950_e20755_d_n3;
        locals.var_t0__blk1144_dn4 = assign26950_e20755_d_n4;
        locals.var_t0__blk1144_dn5 = assign26950_e20755_d_n5;
        locals.var_t0__blk1144_dn6 = assign26950_e20755_d_n6;
        locals.var_t0__blk1144_dn7 = assign26950_e20755_d_n7;
        locals.var_t0__blk1144_dn8 = assign26950_e20755_d_n8;
        locals.var_t0__blk1144_dn9 = assign26950_e20755_d_n9;
        locals.var_t0__blk1144_dn10 = assign26950_e20755_d_n10;
        locals.var_t0__blk1144_dn11 = assign26950_e20755_d_n11;
        locals.var_t0__blk1144_dn12 = assign26950_e20755_d_n12;

        let assign26960_e20758: f64 = if locals.var_t0__blk1144 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1648 = assign26960_e20758;

        let (assign26970_e20764, assign26970_e20764_d_n3, assign26970_e20764_d_n4, assign26970_e20764_d_n5, assign26970_e20764_d_n6, assign26970_e20764_d_n7, assign26970_e20764_d_n8, assign26970_e20764_d_n9, assign26970_e20764_d_n10, assign26970_e20764_d_n11, assign26970_e20764_d_n12,) = {
    if ((locals.var_guard1644 != 0.0) && (locals.var_guard1648 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign26970_e20764;
        locals.var_t0__blk1144_dn3 = assign26970_e20764_d_n3;
        locals.var_t0__blk1144_dn4 = assign26970_e20764_d_n4;
        locals.var_t0__blk1144_dn5 = assign26970_e20764_d_n5;
        locals.var_t0__blk1144_dn6 = assign26970_e20764_d_n6;
        locals.var_t0__blk1144_dn7 = assign26970_e20764_d_n7;
        locals.var_t0__blk1144_dn8 = assign26970_e20764_d_n8;
        locals.var_t0__blk1144_dn9 = assign26970_e20764_d_n9;
        locals.var_t0__blk1144_dn10 = assign26970_e20764_d_n10;
        locals.var_t0__blk1144_dn11 = assign26970_e20764_d_n11;
        locals.var_t0__blk1144_dn12 = assign26970_e20764_d_n12;

        let (assign26980_e20780, assign26980_e20780_d_n3, assign26980_e20780_d_n4, assign26980_e20780_d_n5, assign26980_e20780_d_n6, assign26980_e20780_d_n7, assign26980_e20780_d_n8, assign26980_e20780_d_n9, assign26980_e20780_d_n10, assign26980_e20780_d_n11, assign26980_e20780_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign26980_e20768: f64 = (locals.var_leff * locals.var_weff);
        let assign26980_e20770: f64 = (assign26980_e20768 / locals.var_b4soinseg);
        let assign26980_e20773: f64 = (locals.var_b4soiagbcpd / locals.var_b4soinf);
        let assign26980_e20774: f64 = (assign26980_e20770 + assign26980_e20773);
        let assign26980_e20776: f64 = (assign26980_e20774 * locals.var_agb1);
        let assign26980_e20778: f64 = (assign26980_e20776 * locals.var_oxideratio);
        (assign26980_e20778, (((((locals.var_leff_dn3 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn3)) / locals.var_b4soinseg) * locals.var_agb1) * locals.var_oxideratio), (((((locals.var_leff_dn4 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn4)) / locals.var_b4soinseg) * locals.var_agb1) * locals.var_oxideratio), (((((locals.var_leff_dn5 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn5)) / locals.var_b4soinseg) * locals.var_agb1) * locals.var_oxideratio), (((((locals.var_leff_dn6 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn6)) / locals.var_b4soinseg) * locals.var_agb1) * locals.var_oxideratio), (((((locals.var_leff_dn7 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn7)) / locals.var_b4soinseg) * locals.var_agb1) * locals.var_oxideratio), (((((locals.var_leff_dn8 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn8)) / locals.var_b4soinseg) * locals.var_agb1) * locals.var_oxideratio), (((((locals.var_leff_dn9 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn9)) / locals.var_b4soinseg) * locals.var_agb1) * locals.var_oxideratio), (((((locals.var_leff_dn10 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn10)) / locals.var_b4soinseg) * locals.var_agb1) * locals.var_oxideratio), (((((locals.var_leff_dn11 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn11)) / locals.var_b4soinseg) * locals.var_agb1) * locals.var_oxideratio), (((((locals.var_leff_dn12 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn12)) / locals.var_b4soinseg) * locals.var_agb1) * locals.var_oxideratio),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign26980_e20780;
        locals.var_t1__blk1145_dn3 = assign26980_e20780_d_n3;
        locals.var_t1__blk1145_dn4 = assign26980_e20780_d_n4;
        locals.var_t1__blk1145_dn5 = assign26980_e20780_d_n5;
        locals.var_t1__blk1145_dn6 = assign26980_e20780_d_n6;
        locals.var_t1__blk1145_dn7 = assign26980_e20780_d_n7;
        locals.var_t1__blk1145_dn8 = assign26980_e20780_d_n8;
        locals.var_t1__blk1145_dn9 = assign26980_e20780_d_n9;
        locals.var_t1__blk1145_dn10 = assign26980_e20780_d_n10;
        locals.var_t1__blk1145_dn11 = assign26980_e20780_d_n11;
        locals.var_t1__blk1145_dn12 = assign26980_e20780_d_n12;

        let (assign26990_e20786, assign26990_e20786_d_n3, assign26990_e20786_d_n4, assign26990_e20786_d_n5, assign26990_e20786_d_n6, assign26990_e20786_d_n7, assign26990_e20786_d_n8, assign26990_e20786_d_n9, assign26990_e20786_d_n10, assign26990_e20786_d_n11, assign26990_e20786_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign26990_e20784: f64 = (locals.var_bgb1 * locals.var_b4soitoxqm);
        (assign26990_e20784, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign26990_e20786;
        locals.var_t2__blk1146_dn3 = assign26990_e20786_d_n3;
        locals.var_t2__blk1146_dn4 = assign26990_e20786_d_n4;
        locals.var_t2__blk1146_dn5 = assign26990_e20786_d_n5;
        locals.var_t2__blk1146_dn6 = assign26990_e20786_d_n6;
        locals.var_t2__blk1146_dn7 = assign26990_e20786_d_n7;
        locals.var_t2__blk1146_dn8 = assign26990_e20786_d_n8;
        locals.var_t2__blk1146_dn9 = assign26990_e20786_d_n9;
        locals.var_t2__blk1146_dn10 = assign26990_e20786_d_n10;
        locals.var_t2__blk1146_dn11 = assign26990_e20786_d_n11;
        locals.var_t2__blk1146_dn12 = assign26990_e20786_d_n12;

        let (assign27000_e20790, assign27000_e20790_d_n3, assign27000_e20790_d_n4, assign27000_e20790_d_n5, assign27000_e20790_d_n6, assign27000_e20790_d_n7, assign27000_e20790_d_n8, assign27000_e20790_d_n9, assign27000_e20790_d_n10, assign27000_e20790_d_n11, assign27000_e20790_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        (locals.var_pparam_b4soialphagb1, locals.var_pparam_b4soialphagb1_dn3, locals.var_pparam_b4soialphagb1_dn4, locals.var_pparam_b4soialphagb1_dn5, locals.var_pparam_b4soialphagb1_dn6, locals.var_pparam_b4soialphagb1_dn7, locals.var_pparam_b4soialphagb1_dn8, locals.var_pparam_b4soialphagb1_dn9, locals.var_pparam_b4soialphagb1_dn10, locals.var_pparam_b4soialphagb1_dn11, locals.var_pparam_b4soialphagb1_dn12,)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign27000_e20790;
        locals.var_t3__blk1147_dn3 = assign27000_e20790_d_n3;
        locals.var_t3__blk1147_dn4 = assign27000_e20790_d_n4;
        locals.var_t3__blk1147_dn5 = assign27000_e20790_d_n5;
        locals.var_t3__blk1147_dn6 = assign27000_e20790_d_n6;
        locals.var_t3__blk1147_dn7 = assign27000_e20790_d_n7;
        locals.var_t3__blk1147_dn8 = assign27000_e20790_d_n8;
        locals.var_t3__blk1147_dn9 = assign27000_e20790_d_n9;
        locals.var_t3__blk1147_dn10 = assign27000_e20790_d_n10;
        locals.var_t3__blk1147_dn11 = assign27000_e20790_d_n11;
        locals.var_t3__blk1147_dn12 = assign27000_e20790_d_n12;

        let (assign27010_e20794, assign27010_e20794_d_n3, assign27010_e20794_d_n4, assign27010_e20794_d_n5, assign27010_e20794_d_n6, assign27010_e20794_d_n7, assign27010_e20794_d_n8, assign27010_e20794_d_n9, assign27010_e20794_d_n10, assign27010_e20794_d_n11, assign27010_e20794_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        (locals.var_pparam_b4soibetagb1, locals.var_pparam_b4soibetagb1_dn3, locals.var_pparam_b4soibetagb1_dn4, locals.var_pparam_b4soibetagb1_dn5, locals.var_pparam_b4soibetagb1_dn6, locals.var_pparam_b4soibetagb1_dn7, locals.var_pparam_b4soibetagb1_dn8, locals.var_pparam_b4soibetagb1_dn9, locals.var_pparam_b4soibetagb1_dn10, locals.var_pparam_b4soibetagb1_dn11, locals.var_pparam_b4soibetagb1_dn12,)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign27010_e20794;
        locals.var_t4__blk1148_dn3 = assign27010_e20794_d_n3;
        locals.var_t4__blk1148_dn4 = assign27010_e20794_d_n4;
        locals.var_t4__blk1148_dn5 = assign27010_e20794_d_n5;
        locals.var_t4__blk1148_dn6 = assign27010_e20794_d_n6;
        locals.var_t4__blk1148_dn7 = assign27010_e20794_d_n7;
        locals.var_t4__blk1148_dn8 = assign27010_e20794_d_n8;
        locals.var_t4__blk1148_dn9 = assign27010_e20794_d_n9;
        locals.var_t4__blk1148_dn10 = assign27010_e20794_d_n10;
        locals.var_t4__blk1148_dn11 = assign27010_e20794_d_n11;
        locals.var_t4__blk1148_dn12 = assign27010_e20794_d_n12;

        let (assign27020_e20806, assign27020_e20806_d_n3, assign27020_e20806_d_n4, assign27020_e20806_d_n5, assign27020_e20806_d_n6, assign27020_e20806_d_n7, assign27020_e20806_d_n8, assign27020_e20806_d_n9, assign27020_e20806_d_n10, assign27020_e20806_d_n11, assign27020_e20806_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign27020_e20800: f64 = (locals.var_t4__blk1148 * locals.var_vox);
        let assign27020_e20801: f64 = (locals.var_t3__blk1147 - assign27020_e20800);
        let assign27020_e20802: f64 = (locals.var_t2__blk1146 * assign27020_e20801);
        let assign27020_e20804: f64 = (assign27020_e20802 / locals.var_t0__blk1144);
        (assign27020_e20804, (((((locals.var_t2__blk1146_dn3 * assign27020_e20801) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn3 - ((locals.var_t4__blk1148_dn3 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn3))))) * locals.var_t0__blk1144) - (assign27020_e20802 * locals.var_t0__blk1144_dn3)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn4 * assign27020_e20801) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn4 - ((locals.var_t4__blk1148_dn4 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn4))))) * locals.var_t0__blk1144) - (assign27020_e20802 * locals.var_t0__blk1144_dn4)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn5 * assign27020_e20801) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn5 - ((locals.var_t4__blk1148_dn5 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn5))))) * locals.var_t0__blk1144) - (assign27020_e20802 * locals.var_t0__blk1144_dn5)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn6 * assign27020_e20801) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn6 - ((locals.var_t4__blk1148_dn6 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn6))))) * locals.var_t0__blk1144) - (assign27020_e20802 * locals.var_t0__blk1144_dn6)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn7 * assign27020_e20801) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn7 - ((locals.var_t4__blk1148_dn7 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn7))))) * locals.var_t0__blk1144) - (assign27020_e20802 * locals.var_t0__blk1144_dn7)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn8 * assign27020_e20801) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn8 - ((locals.var_t4__blk1148_dn8 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn8))))) * locals.var_t0__blk1144) - (assign27020_e20802 * locals.var_t0__blk1144_dn8)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn9 * assign27020_e20801) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn9 - ((locals.var_t4__blk1148_dn9 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn9))))) * locals.var_t0__blk1144) - (assign27020_e20802 * locals.var_t0__blk1144_dn9)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn10 * assign27020_e20801) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn10 - ((locals.var_t4__blk1148_dn10 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn10))))) * locals.var_t0__blk1144) - (assign27020_e20802 * locals.var_t0__blk1144_dn10)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn11 * assign27020_e20801) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn11 - ((locals.var_t4__blk1148_dn11 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn11))))) * locals.var_t0__blk1144) - (assign27020_e20802 * locals.var_t0__blk1144_dn11)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn12 * assign27020_e20801) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn12 - ((locals.var_t4__blk1148_dn12 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn12))))) * locals.var_t0__blk1144) - (assign27020_e20802 * locals.var_t0__blk1144_dn12)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)),)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign27020_e20806;
        locals.var_t6__blk1150_dn3 = assign27020_e20806_d_n3;
        locals.var_t6__blk1150_dn4 = assign27020_e20806_d_n4;
        locals.var_t6__blk1150_dn5 = assign27020_e20806_d_n5;
        locals.var_t6__blk1150_dn6 = assign27020_e20806_d_n6;
        locals.var_t6__blk1150_dn7 = assign27020_e20806_d_n7;
        locals.var_t6__blk1150_dn8 = assign27020_e20806_d_n8;
        locals.var_t6__blk1150_dn9 = assign27020_e20806_d_n9;
        locals.var_t6__blk1150_dn10 = assign27020_e20806_d_n10;
        locals.var_t6__blk1150_dn11 = assign27020_e20806_d_n11;
        locals.var_t6__blk1150_dn12 = assign27020_e20806_d_n12;

        let assign27030_e20809: f64 = if locals.var_t6__blk1150 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1649 = assign27030_e20809;

        let (assign27040_e20821, assign27040_e20821_d_n3, assign27040_e20821_d_n4, assign27040_e20821_d_n5, assign27040_e20821_d_n6, assign27040_e20821_d_n7, assign27040_e20821_d_n8, assign27040_e20821_d_n9, assign27040_e20821_d_n10, assign27040_e20821_d_n11, assign27040_e20821_d_n12,) = {
    if ((locals.var_guard1644 != 0.0) && (locals.var_guard1649 != 0.0)) {
        let assign27040_e20816: f64 = (1.0 + locals.var_t6__blk1150);
        let assign27040_e20818: f64 = (assign27040_e20816 - 100.0);
        let assign27040_e20819: f64 = (2.688117142e43 * assign27040_e20818);
        (assign27040_e20819, (2.688117142e43 * locals.var_t6__blk1150_dn3), (2.688117142e43 * locals.var_t6__blk1150_dn4), (2.688117142e43 * locals.var_t6__blk1150_dn5), (2.688117142e43 * locals.var_t6__blk1150_dn6), (2.688117142e43 * locals.var_t6__blk1150_dn7), (2.688117142e43 * locals.var_t6__blk1150_dn8), (2.688117142e43 * locals.var_t6__blk1150_dn9), (2.688117142e43 * locals.var_t6__blk1150_dn10), (2.688117142e43 * locals.var_t6__blk1150_dn11), (2.688117142e43 * locals.var_t6__blk1150_dn12),)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign27040_e20821;
        locals.var_t5__blk1149_dn3 = assign27040_e20821_d_n3;
        locals.var_t5__blk1149_dn4 = assign27040_e20821_d_n4;
        locals.var_t5__blk1149_dn5 = assign27040_e20821_d_n5;
        locals.var_t5__blk1149_dn6 = assign27040_e20821_d_n6;
        locals.var_t5__blk1149_dn7 = assign27040_e20821_d_n7;
        locals.var_t5__blk1149_dn8 = assign27040_e20821_d_n8;
        locals.var_t5__blk1149_dn9 = assign27040_e20821_d_n9;
        locals.var_t5__blk1149_dn10 = assign27040_e20821_d_n10;
        locals.var_t5__blk1149_dn11 = assign27040_e20821_d_n11;
        locals.var_t5__blk1149_dn12 = assign27040_e20821_d_n12;

        let assign27050_e20824: f64 = (-100.0);
        let assign27050_e20825: f64 = if locals.var_t6__blk1150 < assign27050_e20824 { 1.0 } else { 0.0 };
        locals.var_guard1650 = assign27050_e20825;

        let (assign27060_e20834, assign27060_e20834_d_n3, assign27060_e20834_d_n4, assign27060_e20834_d_n5, assign27060_e20834_d_n6, assign27060_e20834_d_n7, assign27060_e20834_d_n8, assign27060_e20834_d_n9, assign27060_e20834_d_n10, assign27060_e20834_d_n11, assign27060_e20834_d_n12,) = {
    if (((locals.var_guard1644 != 0.0) && (locals.var_guard1649 == 0.0)) && (locals.var_guard1650 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign27060_e20834;
        locals.var_t5__blk1149_dn3 = assign27060_e20834_d_n3;
        locals.var_t5__blk1149_dn4 = assign27060_e20834_d_n4;
        locals.var_t5__blk1149_dn5 = assign27060_e20834_d_n5;
        locals.var_t5__blk1149_dn6 = assign27060_e20834_d_n6;
        locals.var_t5__blk1149_dn7 = assign27060_e20834_d_n7;
        locals.var_t5__blk1149_dn8 = assign27060_e20834_d_n8;
        locals.var_t5__blk1149_dn9 = assign27060_e20834_d_n9;
        locals.var_t5__blk1149_dn10 = assign27060_e20834_d_n10;
        locals.var_t5__blk1149_dn11 = assign27060_e20834_d_n11;
        locals.var_t5__blk1149_dn12 = assign27060_e20834_d_n12;

        let (assign27070_e20845, assign27070_e20845_d_n3, assign27070_e20845_d_n4, assign27070_e20845_d_n5, assign27070_e20845_d_n6, assign27070_e20845_d_n7, assign27070_e20845_d_n8, assign27070_e20845_d_n9, assign27070_e20845_d_n10, assign27070_e20845_d_n11, assign27070_e20845_d_n12,) = {
    if (((locals.var_guard1644 != 0.0) && (locals.var_guard1649 == 0.0)) && (locals.var_guard1650 == 0.0)) {
        let assign27070_e20843: f64 = (locals.var_t6__blk1150).exp();
        (assign27070_e20843, (assign27070_e20843 * locals.var_t6__blk1150_dn3), (assign27070_e20843 * locals.var_t6__blk1150_dn4), (assign27070_e20843 * locals.var_t6__blk1150_dn5), (assign27070_e20843 * locals.var_t6__blk1150_dn6), (assign27070_e20843 * locals.var_t6__blk1150_dn7), (assign27070_e20843 * locals.var_t6__blk1150_dn8), (assign27070_e20843 * locals.var_t6__blk1150_dn9), (assign27070_e20843 * locals.var_t6__blk1150_dn10), (assign27070_e20843 * locals.var_t6__blk1150_dn11), (assign27070_e20843 * locals.var_t6__blk1150_dn12),)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign27070_e20845;
        locals.var_t5__blk1149_dn3 = assign27070_e20845_d_n3;
        locals.var_t5__blk1149_dn4 = assign27070_e20845_d_n4;
        locals.var_t5__blk1149_dn5 = assign27070_e20845_d_n5;
        locals.var_t5__blk1149_dn6 = assign27070_e20845_d_n6;
        locals.var_t5__blk1149_dn7 = assign27070_e20845_d_n7;
        locals.var_t5__blk1149_dn8 = assign27070_e20845_d_n8;
        locals.var_t5__blk1149_dn9 = assign27070_e20845_d_n9;
        locals.var_t5__blk1149_dn10 = assign27070_e20845_d_n10;
        locals.var_t5__blk1149_dn11 = assign27070_e20845_d_n11;
        locals.var_t5__blk1149_dn12 = assign27070_e20845_d_n12;

        let (assign27080_e20855, assign27080_e20855_d_n3, assign27080_e20855_d_n4, assign27080_e20855_d_n5, assign27080_e20855_d_n6, assign27080_e20855_d_n7, assign27080_e20855_d_n8, assign27080_e20855_d_n9, assign27080_e20855_d_n10, assign27080_e20855_d_n11, assign27080_e20855_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign27080_e20849: f64 = (locals.var_t1__blk1145 * locals.var_vgb);
        let assign27080_e20851: f64 = (assign27080_e20849 * locals.var_vaux);
        let assign27080_e20853: f64 = (assign27080_e20851 * locals.var_t5__blk1149);
        (assign27080_e20853, ((((((locals.var_t1__blk1145_dn3 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn3)) * locals.var_vaux) + (assign27080_e20849 * locals.var_vaux_dn3)) * locals.var_t5__blk1149) + (assign27080_e20851 * locals.var_t5__blk1149_dn3)), ((((((locals.var_t1__blk1145_dn4 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn4)) * locals.var_vaux) + (assign27080_e20849 * locals.var_vaux_dn4)) * locals.var_t5__blk1149) + (assign27080_e20851 * locals.var_t5__blk1149_dn4)), ((((((locals.var_t1__blk1145_dn5 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn5)) * locals.var_vaux) + (assign27080_e20849 * locals.var_vaux_dn5)) * locals.var_t5__blk1149) + (assign27080_e20851 * locals.var_t5__blk1149_dn5)), ((((((locals.var_t1__blk1145_dn6 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn6)) * locals.var_vaux) + (assign27080_e20849 * locals.var_vaux_dn6)) * locals.var_t5__blk1149) + (assign27080_e20851 * locals.var_t5__blk1149_dn6)), ((((((locals.var_t1__blk1145_dn7 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn7)) * locals.var_vaux) + (assign27080_e20849 * locals.var_vaux_dn7)) * locals.var_t5__blk1149) + (assign27080_e20851 * locals.var_t5__blk1149_dn7)), ((((((locals.var_t1__blk1145_dn8 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn8)) * locals.var_vaux) + (assign27080_e20849 * locals.var_vaux_dn8)) * locals.var_t5__blk1149) + (assign27080_e20851 * locals.var_t5__blk1149_dn8)), ((((((locals.var_t1__blk1145_dn9 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn9)) * locals.var_vaux) + (assign27080_e20849 * locals.var_vaux_dn9)) * locals.var_t5__blk1149) + (assign27080_e20851 * locals.var_t5__blk1149_dn9)), ((((((locals.var_t1__blk1145_dn10 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn10)) * locals.var_vaux) + (assign27080_e20849 * locals.var_vaux_dn10)) * locals.var_t5__blk1149) + (assign27080_e20851 * locals.var_t5__blk1149_dn10)), ((((((locals.var_t1__blk1145_dn11 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn11)) * locals.var_vaux) + (assign27080_e20849 * locals.var_vaux_dn11)) * locals.var_t5__blk1149) + (assign27080_e20851 * locals.var_t5__blk1149_dn11)), ((((((locals.var_t1__blk1145_dn12 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn12)) * locals.var_vaux) + (assign27080_e20849 * locals.var_vaux_dn12)) * locals.var_t5__blk1149) + (assign27080_e20851 * locals.var_t5__blk1149_dn12)),)
    } else {
        (locals.var_igb1, locals.var_igb1_dn3, locals.var_igb1_dn4, locals.var_igb1_dn5, locals.var_igb1_dn6, locals.var_igb1_dn7, locals.var_igb1_dn8, locals.var_igb1_dn9, locals.var_igb1_dn10, locals.var_igb1_dn11, locals.var_igb1_dn12,)
    }
};
        locals.var_igb1 = assign27080_e20855;
        locals.var_igb1_dn3 = assign27080_e20855_d_n3;
        locals.var_igb1_dn4 = assign27080_e20855_d_n4;
        locals.var_igb1_dn5 = assign27080_e20855_d_n5;
        locals.var_igb1_dn6 = assign27080_e20855_d_n6;
        locals.var_igb1_dn7 = assign27080_e20855_d_n7;
        locals.var_igb1_dn8 = assign27080_e20855_d_n8;
        locals.var_igb1_dn9 = assign27080_e20855_d_n9;
        locals.var_igb1_dn10 = assign27080_e20855_d_n10;
        locals.var_igb1_dn11 = assign27080_e20855_d_n11;
        locals.var_igb1_dn12 = assign27080_e20855_d_n12;

        let (assign27090_e20859, assign27090_e20859_d_n3, assign27090_e20859_d_n4, assign27090_e20859_d_n5, assign27090_e20859_d_n6, assign27090_e20859_d_n7, assign27090_e20859_d_n8, assign27090_e20859_d_n9, assign27090_e20859_d_n10, assign27090_e20859_d_n11, assign27090_e20859_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        (locals.var_voxacc, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8, locals.var_voxacc_dn9, locals.var_voxacc_dn10, locals.var_voxacc_dn11, locals.var_voxacc_dn12,)
    } else {
        (locals.var_vox, locals.var_vox_dn3, locals.var_vox_dn4, locals.var_vox_dn5, locals.var_vox_dn6, locals.var_vox_dn7, locals.var_vox_dn8, locals.var_vox_dn9, locals.var_vox_dn10, locals.var_vox_dn11, locals.var_vox_dn12,)
    }
};
        locals.var_vox = assign27090_e20859;
        locals.var_vox_dn3 = assign27090_e20859_d_n3;
        locals.var_vox_dn4 = assign27090_e20859_d_n4;
        locals.var_vox_dn5 = assign27090_e20859_d_n5;
        locals.var_vox_dn6 = assign27090_e20859_d_n6;
        locals.var_vox_dn7 = assign27090_e20859_d_n7;
        locals.var_vox_dn8 = assign27090_e20859_d_n8;
        locals.var_vox_dn9 = assign27090_e20859_d_n9;
        locals.var_vox_dn10 = assign27090_e20859_d_n10;
        locals.var_vox_dn11 = assign27090_e20859_d_n11;
        locals.var_vox_dn12 = assign27090_e20859_d_n12;

        let (assign27100_e20863, assign27100_e20863_d_n3, assign27100_e20863_d_n4, assign27100_e20863_d_n5, assign27100_e20863_d_n6, assign27100_e20863_d_n7, assign27100_e20863_d_n8, assign27100_e20863_d_n9, assign27100_e20863_d_n10, assign27100_e20863_d_n11, assign27100_e20863_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        (locals.var_b4soivoxh, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27100_e20863;
        locals.var_t0__blk1144_dn3 = assign27100_e20863_d_n3;
        locals.var_t0__blk1144_dn4 = assign27100_e20863_d_n4;
        locals.var_t0__blk1144_dn5 = assign27100_e20863_d_n5;
        locals.var_t0__blk1144_dn6 = assign27100_e20863_d_n6;
        locals.var_t0__blk1144_dn7 = assign27100_e20863_d_n7;
        locals.var_t0__blk1144_dn8 = assign27100_e20863_d_n8;
        locals.var_t0__blk1144_dn9 = assign27100_e20863_d_n9;
        locals.var_t0__blk1144_dn10 = assign27100_e20863_d_n10;
        locals.var_t0__blk1144_dn11 = assign27100_e20863_d_n11;
        locals.var_t0__blk1144_dn12 = assign27100_e20863_d_n12;

        let (assign27110_e20871, assign27110_e20871_d_n3, assign27110_e20871_d_n4, assign27110_e20871_d_n5, assign27110_e20871_d_n6, assign27110_e20871_d_n7, assign27110_e20871_d_n8, assign27110_e20871_d_n9, assign27110_e20871_d_n10, assign27110_e20871_d_n11, assign27110_e20871_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign27110_e20867: f64 = (locals.var_t0__blk1144 - locals.var_vox);
        let assign27110_e20869: f64 = (assign27110_e20867 - locals.var_b4soideltavox);
        (assign27110_e20869, (locals.var_t0__blk1144_dn3 - locals.var_vox_dn3), (locals.var_t0__blk1144_dn4 - locals.var_vox_dn4), (locals.var_t0__blk1144_dn5 - locals.var_vox_dn5), (locals.var_t0__blk1144_dn6 - locals.var_vox_dn6), (locals.var_t0__blk1144_dn7 - locals.var_vox_dn7), (locals.var_t0__blk1144_dn8 - locals.var_vox_dn8), (locals.var_t0__blk1144_dn9 - locals.var_vox_dn9), (locals.var_t0__blk1144_dn10 - locals.var_vox_dn10), (locals.var_t0__blk1144_dn11 - locals.var_vox_dn11), (locals.var_t0__blk1144_dn12 - locals.var_vox_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign27110_e20871;
        locals.var_t1__blk1145_dn3 = assign27110_e20871_d_n3;
        locals.var_t1__blk1145_dn4 = assign27110_e20871_d_n4;
        locals.var_t1__blk1145_dn5 = assign27110_e20871_d_n5;
        locals.var_t1__blk1145_dn6 = assign27110_e20871_d_n6;
        locals.var_t1__blk1145_dn7 = assign27110_e20871_d_n7;
        locals.var_t1__blk1145_dn8 = assign27110_e20871_d_n8;
        locals.var_t1__blk1145_dn9 = assign27110_e20871_d_n9;
        locals.var_t1__blk1145_dn10 = assign27110_e20871_d_n10;
        locals.var_t1__blk1145_dn11 = assign27110_e20871_d_n11;
        locals.var_t1__blk1145_dn12 = assign27110_e20871_d_n12;

        let (assign27120_e20884, assign27120_e20884_d_n3, assign27120_e20884_d_n4, assign27120_e20884_d_n5, assign27120_e20884_d_n6, assign27120_e20884_d_n7, assign27120_e20884_d_n8, assign27120_e20884_d_n9, assign27120_e20884_d_n10, assign27120_e20884_d_n11, assign27120_e20884_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign27120_e20875: f64 = (locals.var_t1__blk1145 * locals.var_t1__blk1145);
        let assign27120_e20878: f64 = (4.0 * locals.var_b4soideltavox);
        let assign27120_e20880: f64 = (assign27120_e20878 * locals.var_t0__blk1144);
        let assign27120_e20881: f64 = (assign27120_e20875 + assign27120_e20880);
        let assign27120_e20882: f64 = (assign27120_e20881).sqrt();
        (assign27120_e20882, ((((locals.var_t1__blk1145_dn3 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn3)) + (assign27120_e20878 * locals.var_t0__blk1144_dn3)) / (2.0 * assign27120_e20882)), ((((locals.var_t1__blk1145_dn4 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn4)) + (assign27120_e20878 * locals.var_t0__blk1144_dn4)) / (2.0 * assign27120_e20882)), ((((locals.var_t1__blk1145_dn5 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn5)) + (assign27120_e20878 * locals.var_t0__blk1144_dn5)) / (2.0 * assign27120_e20882)), ((((locals.var_t1__blk1145_dn6 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn6)) + (assign27120_e20878 * locals.var_t0__blk1144_dn6)) / (2.0 * assign27120_e20882)), ((((locals.var_t1__blk1145_dn7 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn7)) + (assign27120_e20878 * locals.var_t0__blk1144_dn7)) / (2.0 * assign27120_e20882)), ((((locals.var_t1__blk1145_dn8 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn8)) + (assign27120_e20878 * locals.var_t0__blk1144_dn8)) / (2.0 * assign27120_e20882)), ((((locals.var_t1__blk1145_dn9 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn9)) + (assign27120_e20878 * locals.var_t0__blk1144_dn9)) / (2.0 * assign27120_e20882)), ((((locals.var_t1__blk1145_dn10 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn10)) + (assign27120_e20878 * locals.var_t0__blk1144_dn10)) / (2.0 * assign27120_e20882)), ((((locals.var_t1__blk1145_dn11 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn11)) + (assign27120_e20878 * locals.var_t0__blk1144_dn11)) / (2.0 * assign27120_e20882)), ((((locals.var_t1__blk1145_dn12 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn12)) + (assign27120_e20878 * locals.var_t0__blk1144_dn12)) / (2.0 * assign27120_e20882)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign27120_e20884;
        locals.var_t3__blk1147_dn3 = assign27120_e20884_d_n3;
        locals.var_t3__blk1147_dn4 = assign27120_e20884_d_n4;
        locals.var_t3__blk1147_dn5 = assign27120_e20884_d_n5;
        locals.var_t3__blk1147_dn6 = assign27120_e20884_d_n6;
        locals.var_t3__blk1147_dn7 = assign27120_e20884_d_n7;
        locals.var_t3__blk1147_dn8 = assign27120_e20884_d_n8;
        locals.var_t3__blk1147_dn9 = assign27120_e20884_d_n9;
        locals.var_t3__blk1147_dn10 = assign27120_e20884_d_n10;
        locals.var_t3__blk1147_dn11 = assign27120_e20884_d_n11;
        locals.var_t3__blk1147_dn12 = assign27120_e20884_d_n12;

    }

    pub(super) fn stamp_transient_block_71(
        locals: &mut StampLocals,
    ) {
        let (assign27130_e20894, assign27130_e20894_d_n3, assign27130_e20894_d_n4, assign27130_e20894_d_n5, assign27130_e20894_d_n6, assign27130_e20894_d_n7, assign27130_e20894_d_n8, assign27130_e20894_d_n9, assign27130_e20894_d_n10, assign27130_e20894_d_n11, assign27130_e20894_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign27130_e20890: f64 = (locals.var_t1__blk1145 + locals.var_t3__blk1147);
        let assign27130_e20891: f64 = (0.5 * assign27130_e20890);
        let assign27130_e20892: f64 = (locals.var_t0__blk1144 - assign27130_e20891);
        (assign27130_e20892, (locals.var_t0__blk1144_dn3 - (0.5 * (locals.var_t1__blk1145_dn3 + locals.var_t3__blk1147_dn3))), (locals.var_t0__blk1144_dn4 - (0.5 * (locals.var_t1__blk1145_dn4 + locals.var_t3__blk1147_dn4))), (locals.var_t0__blk1144_dn5 - (0.5 * (locals.var_t1__blk1145_dn5 + locals.var_t3__blk1147_dn5))), (locals.var_t0__blk1144_dn6 - (0.5 * (locals.var_t1__blk1145_dn6 + locals.var_t3__blk1147_dn6))), (locals.var_t0__blk1144_dn7 - (0.5 * (locals.var_t1__blk1145_dn7 + locals.var_t3__blk1147_dn7))), (locals.var_t0__blk1144_dn8 - (0.5 * (locals.var_t1__blk1145_dn8 + locals.var_t3__blk1147_dn8))), (locals.var_t0__blk1144_dn9 - (0.5 * (locals.var_t1__blk1145_dn9 + locals.var_t3__blk1147_dn9))), (locals.var_t0__blk1144_dn10 - (0.5 * (locals.var_t1__blk1145_dn10 + locals.var_t3__blk1147_dn10))), (locals.var_t0__blk1144_dn11 - (0.5 * (locals.var_t1__blk1145_dn11 + locals.var_t3__blk1147_dn11))), (locals.var_t0__blk1144_dn12 - (0.5 * (locals.var_t1__blk1145_dn12 + locals.var_t3__blk1147_dn12))),)
    } else {
        (locals.var_voxeff, locals.var_voxeff_dn3, locals.var_voxeff_dn4, locals.var_voxeff_dn5, locals.var_voxeff_dn6, locals.var_voxeff_dn7, locals.var_voxeff_dn8, locals.var_voxeff_dn9, locals.var_voxeff_dn10, locals.var_voxeff_dn11, locals.var_voxeff_dn12,)
    }
};
        locals.var_voxeff = assign27130_e20894;
        locals.var_voxeff_dn3 = assign27130_e20894_d_n3;
        locals.var_voxeff_dn4 = assign27130_e20894_d_n4;
        locals.var_voxeff_dn5 = assign27130_e20894_d_n5;
        locals.var_voxeff_dn6 = assign27130_e20894_d_n6;
        locals.var_voxeff_dn7 = assign27130_e20894_d_n7;
        locals.var_voxeff_dn8 = assign27130_e20894_d_n8;
        locals.var_voxeff_dn9 = assign27130_e20894_d_n9;
        locals.var_voxeff_dn10 = assign27130_e20894_d_n10;
        locals.var_voxeff_dn11 = assign27130_e20894_d_n11;
        locals.var_voxeff_dn12 = assign27130_e20894_d_n12;

        let (assign27140_e20898, assign27140_e20898_d_n3, assign27140_e20898_d_n4, assign27140_e20898_d_n5, assign27140_e20898_d_n6, assign27140_e20898_d_n7, assign27140_e20898_d_n8, assign27140_e20898_d_n9, assign27140_e20898_d_n10, assign27140_e20898_d_n11, assign27140_e20898_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        (locals.var_voxeff, locals.var_voxeff_dn3, locals.var_voxeff_dn4, locals.var_voxeff_dn5, locals.var_voxeff_dn6, locals.var_voxeff_dn7, locals.var_voxeff_dn8, locals.var_voxeff_dn9, locals.var_voxeff_dn10, locals.var_voxeff_dn11, locals.var_voxeff_dn12,)
    } else {
        (locals.var_vox, locals.var_vox_dn3, locals.var_vox_dn4, locals.var_vox_dn5, locals.var_vox_dn6, locals.var_vox_dn7, locals.var_vox_dn8, locals.var_vox_dn9, locals.var_vox_dn10, locals.var_vox_dn11, locals.var_vox_dn12,)
    }
};
        locals.var_vox = assign27140_e20898;
        locals.var_vox_dn3 = assign27140_e20898_d_n3;
        locals.var_vox_dn4 = assign27140_e20898_d_n4;
        locals.var_vox_dn5 = assign27140_e20898_d_n5;
        locals.var_vox_dn6 = assign27140_e20898_d_n6;
        locals.var_vox_dn7 = assign27140_e20898_d_n7;
        locals.var_vox_dn8 = assign27140_e20898_d_n8;
        locals.var_vox_dn9 = assign27140_e20898_d_n9;
        locals.var_vox_dn10 = assign27140_e20898_d_n10;
        locals.var_vox_dn11 = assign27140_e20898_d_n11;
        locals.var_vox_dn12 = assign27140_e20898_d_n12;

        let (assign27150_e20907, assign27150_e20907_d_n3, assign27150_e20907_d_n4, assign27150_e20907_d_n5, assign27150_e20907_d_n6, assign27150_e20907_d_n7, assign27150_e20907_d_n8, assign27150_e20907_d_n9, assign27150_e20907_d_n10, assign27150_e20907_d_n11, assign27150_e20907_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign27150_e20901: f64 = (-locals.var_vgb);
        let assign27150_e20903: f64 = (assign27150_e20901 + locals.var_vfb);
        let assign27150_e20905: f64 = (assign27150_e20903 / locals.var_b4soivecb);
        (assign27150_e20905, (((-locals.var_vgb_dn3) + locals.var_vfb_dn3) / locals.var_b4soivecb), (((-locals.var_vgb_dn4) + locals.var_vfb_dn4) / locals.var_b4soivecb), (((-locals.var_vgb_dn5) + locals.var_vfb_dn5) / locals.var_b4soivecb), (((-locals.var_vgb_dn6) + locals.var_vfb_dn6) / locals.var_b4soivecb), (((-locals.var_vgb_dn7) + locals.var_vfb_dn7) / locals.var_b4soivecb), (((-locals.var_vgb_dn8) + locals.var_vfb_dn8) / locals.var_b4soivecb), (((-locals.var_vgb_dn9) + locals.var_vfb_dn9) / locals.var_b4soivecb), (((-locals.var_vgb_dn10) + locals.var_vfb_dn10) / locals.var_b4soivecb), (((-locals.var_vgb_dn11) + locals.var_vfb_dn11) / locals.var_b4soivecb), (((-locals.var_vgb_dn12) + locals.var_vfb_dn12) / locals.var_b4soivecb),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27150_e20907;
        locals.var_t0__blk1144_dn3 = assign27150_e20907_d_n3;
        locals.var_t0__blk1144_dn4 = assign27150_e20907_d_n4;
        locals.var_t0__blk1144_dn5 = assign27150_e20907_d_n5;
        locals.var_t0__blk1144_dn6 = assign27150_e20907_d_n6;
        locals.var_t0__blk1144_dn7 = assign27150_e20907_d_n7;
        locals.var_t0__blk1144_dn8 = assign27150_e20907_d_n8;
        locals.var_t0__blk1144_dn9 = assign27150_e20907_d_n9;
        locals.var_t0__blk1144_dn10 = assign27150_e20907_d_n10;
        locals.var_t0__blk1144_dn11 = assign27150_e20907_d_n11;
        locals.var_t0__blk1144_dn12 = assign27150_e20907_d_n12;

        let assign27160_e20910: f64 = if locals.var_t0__blk1144 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1651 = assign27160_e20910;

        let (assign27170_e20922, assign27170_e20922_d_n3, assign27170_e20922_d_n4, assign27170_e20922_d_n5, assign27170_e20922_d_n6, assign27170_e20922_d_n7, assign27170_e20922_d_n8, assign27170_e20922_d_n9, assign27170_e20922_d_n10, assign27170_e20922_d_n11, assign27170_e20922_d_n12,) = {
    if ((locals.var_guard1644 != 0.0) && (locals.var_guard1651 != 0.0)) {
        let assign27170_e20917: f64 = (1.0 + locals.var_t0__blk1144);
        let assign27170_e20919: f64 = (assign27170_e20917 - 100.0);
        let assign27170_e20920: f64 = (2.688117142e43 * assign27170_e20919);
        (assign27170_e20920, (2.688117142e43 * locals.var_t0__blk1144_dn3), (2.688117142e43 * locals.var_t0__blk1144_dn4), (2.688117142e43 * locals.var_t0__blk1144_dn5), (2.688117142e43 * locals.var_t0__blk1144_dn6), (2.688117142e43 * locals.var_t0__blk1144_dn7), (2.688117142e43 * locals.var_t0__blk1144_dn8), (2.688117142e43 * locals.var_t0__blk1144_dn9), (2.688117142e43 * locals.var_t0__blk1144_dn10), (2.688117142e43 * locals.var_t0__blk1144_dn11), (2.688117142e43 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign27170_e20922;
        locals.var_t1__blk1145_dn3 = assign27170_e20922_d_n3;
        locals.var_t1__blk1145_dn4 = assign27170_e20922_d_n4;
        locals.var_t1__blk1145_dn5 = assign27170_e20922_d_n5;
        locals.var_t1__blk1145_dn6 = assign27170_e20922_d_n6;
        locals.var_t1__blk1145_dn7 = assign27170_e20922_d_n7;
        locals.var_t1__blk1145_dn8 = assign27170_e20922_d_n8;
        locals.var_t1__blk1145_dn9 = assign27170_e20922_d_n9;
        locals.var_t1__blk1145_dn10 = assign27170_e20922_d_n10;
        locals.var_t1__blk1145_dn11 = assign27170_e20922_d_n11;
        locals.var_t1__blk1145_dn12 = assign27170_e20922_d_n12;

        let assign27180_e20925: f64 = (-100.0);
        let assign27180_e20926: f64 = if locals.var_t0__blk1144 < assign27180_e20925 { 1.0 } else { 0.0 };
        locals.var_guard1652 = assign27180_e20926;

        let (assign27190_e20935, assign27190_e20935_d_n3, assign27190_e20935_d_n4, assign27190_e20935_d_n5, assign27190_e20935_d_n6, assign27190_e20935_d_n7, assign27190_e20935_d_n8, assign27190_e20935_d_n9, assign27190_e20935_d_n10, assign27190_e20935_d_n11, assign27190_e20935_d_n12,) = {
    if (((locals.var_guard1644 != 0.0) && (locals.var_guard1651 == 0.0)) && (locals.var_guard1652 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign27190_e20935;
        locals.var_t1__blk1145_dn3 = assign27190_e20935_d_n3;
        locals.var_t1__blk1145_dn4 = assign27190_e20935_d_n4;
        locals.var_t1__blk1145_dn5 = assign27190_e20935_d_n5;
        locals.var_t1__blk1145_dn6 = assign27190_e20935_d_n6;
        locals.var_t1__blk1145_dn7 = assign27190_e20935_d_n7;
        locals.var_t1__blk1145_dn8 = assign27190_e20935_d_n8;
        locals.var_t1__blk1145_dn9 = assign27190_e20935_d_n9;
        locals.var_t1__blk1145_dn10 = assign27190_e20935_d_n10;
        locals.var_t1__blk1145_dn11 = assign27190_e20935_d_n11;
        locals.var_t1__blk1145_dn12 = assign27190_e20935_d_n12;

        let (assign27200_e20946, assign27200_e20946_d_n3, assign27200_e20946_d_n4, assign27200_e20946_d_n5, assign27200_e20946_d_n6, assign27200_e20946_d_n7, assign27200_e20946_d_n8, assign27200_e20946_d_n9, assign27200_e20946_d_n10, assign27200_e20946_d_n11, assign27200_e20946_d_n12,) = {
    if (((locals.var_guard1644 != 0.0) && (locals.var_guard1651 == 0.0)) && (locals.var_guard1652 == 0.0)) {
        let assign27200_e20944: f64 = (locals.var_t0__blk1144).exp();
        (assign27200_e20944, (assign27200_e20944 * locals.var_t0__blk1144_dn3), (assign27200_e20944 * locals.var_t0__blk1144_dn4), (assign27200_e20944 * locals.var_t0__blk1144_dn5), (assign27200_e20944 * locals.var_t0__blk1144_dn6), (assign27200_e20944 * locals.var_t0__blk1144_dn7), (assign27200_e20944 * locals.var_t0__blk1144_dn8), (assign27200_e20944 * locals.var_t0__blk1144_dn9), (assign27200_e20944 * locals.var_t0__blk1144_dn10), (assign27200_e20944 * locals.var_t0__blk1144_dn11), (assign27200_e20944 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign27200_e20946;
        locals.var_t1__blk1145_dn3 = assign27200_e20946_d_n3;
        locals.var_t1__blk1145_dn4 = assign27200_e20946_d_n4;
        locals.var_t1__blk1145_dn5 = assign27200_e20946_d_n5;
        locals.var_t1__blk1145_dn6 = assign27200_e20946_d_n6;
        locals.var_t1__blk1145_dn7 = assign27200_e20946_d_n7;
        locals.var_t1__blk1145_dn8 = assign27200_e20946_d_n8;
        locals.var_t1__blk1145_dn9 = assign27200_e20946_d_n9;
        locals.var_t1__blk1145_dn10 = assign27200_e20946_d_n10;
        locals.var_t1__blk1145_dn11 = assign27200_e20946_d_n11;
        locals.var_t1__blk1145_dn12 = assign27200_e20946_d_n12;

        let (assign27210_e20955, assign27210_e20955_d_n3, assign27210_e20955_d_n4, assign27210_e20955_d_n5, assign27210_e20955_d_n6, assign27210_e20955_d_n7, assign27210_e20955_d_n8, assign27210_e20955_d_n9, assign27210_e20955_d_n10, assign27210_e20955_d_n11, assign27210_e20955_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign27210_e20951: f64 = (1.0 + locals.var_t1__blk1145);
        let assign27210_e20952: f64 = (assign27210_e20951).ln();
        let assign27210_e20953: f64 = (locals.var_b4soivecb * assign27210_e20952);
        (assign27210_e20953, (locals.var_b4soivecb * (locals.var_t1__blk1145_dn3 / assign27210_e20951)), (locals.var_b4soivecb * (locals.var_t1__blk1145_dn4 / assign27210_e20951)), (locals.var_b4soivecb * (locals.var_t1__blk1145_dn5 / assign27210_e20951)), (locals.var_b4soivecb * (locals.var_t1__blk1145_dn6 / assign27210_e20951)), (locals.var_b4soivecb * (locals.var_t1__blk1145_dn7 / assign27210_e20951)), (locals.var_b4soivecb * (locals.var_t1__blk1145_dn8 / assign27210_e20951)), (locals.var_b4soivecb * (locals.var_t1__blk1145_dn9 / assign27210_e20951)), (locals.var_b4soivecb * (locals.var_t1__blk1145_dn10 / assign27210_e20951)), (locals.var_b4soivecb * (locals.var_t1__blk1145_dn11 / assign27210_e20951)), (locals.var_b4soivecb * (locals.var_t1__blk1145_dn12 / assign27210_e20951)),)
    } else {
        (locals.var_vaux, locals.var_vaux_dn3, locals.var_vaux_dn4, locals.var_vaux_dn5, locals.var_vaux_dn6, locals.var_vaux_dn7, locals.var_vaux_dn8, locals.var_vaux_dn9, locals.var_vaux_dn10, locals.var_vaux_dn11, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign27210_e20955;
        locals.var_vaux_dn3 = assign27210_e20955_d_n3;
        locals.var_vaux_dn4 = assign27210_e20955_d_n4;
        locals.var_vaux_dn5 = assign27210_e20955_d_n5;
        locals.var_vaux_dn6 = assign27210_e20955_d_n6;
        locals.var_vaux_dn7 = assign27210_e20955_d_n7;
        locals.var_vaux_dn8 = assign27210_e20955_d_n8;
        locals.var_vaux_dn9 = assign27210_e20955_d_n9;
        locals.var_vaux_dn10 = assign27210_e20955_d_n10;
        locals.var_vaux_dn11 = assign27210_e20955_d_n11;
        locals.var_vaux_dn12 = assign27210_e20955_d_n12;

        let assign27220_e20958: f64 = if locals.var_b4soivgb2 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1653 = assign27220_e20958;

        let (assign27230_e20968, assign27230_e20968_d_n3, assign27230_e20968_d_n4, assign27230_e20968_d_n5, assign27230_e20968_d_n6, assign27230_e20968_d_n7, assign27230_e20968_d_n8, assign27230_e20968_d_n9, assign27230_e20968_d_n10, assign27230_e20968_d_n11, assign27230_e20968_d_n12,) = {
    if ((locals.var_guard1644 != 0.0) && (locals.var_guard1653 != 0.0)) {
        let assign27230_e20965: f64 = (locals.var_vox / locals.var_b4soivgb2);
        let assign27230_e20966: f64 = (1.0 - assign27230_e20965);
        (assign27230_e20966, (-(locals.var_vox_dn3 / locals.var_b4soivgb2)), (-(locals.var_vox_dn4 / locals.var_b4soivgb2)), (-(locals.var_vox_dn5 / locals.var_b4soivgb2)), (-(locals.var_vox_dn6 / locals.var_b4soivgb2)), (-(locals.var_vox_dn7 / locals.var_b4soivgb2)), (-(locals.var_vox_dn8 / locals.var_b4soivgb2)), (-(locals.var_vox_dn9 / locals.var_b4soivgb2)), (-(locals.var_vox_dn10 / locals.var_b4soivgb2)), (-(locals.var_vox_dn11 / locals.var_b4soivgb2)), (-(locals.var_vox_dn12 / locals.var_b4soivgb2)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27230_e20968;
        locals.var_t0__blk1144_dn3 = assign27230_e20968_d_n3;
        locals.var_t0__blk1144_dn4 = assign27230_e20968_d_n4;
        locals.var_t0__blk1144_dn5 = assign27230_e20968_d_n5;
        locals.var_t0__blk1144_dn6 = assign27230_e20968_d_n6;
        locals.var_t0__blk1144_dn7 = assign27230_e20968_d_n7;
        locals.var_t0__blk1144_dn8 = assign27230_e20968_d_n8;
        locals.var_t0__blk1144_dn9 = assign27230_e20968_d_n9;
        locals.var_t0__blk1144_dn10 = assign27230_e20968_d_n10;
        locals.var_t0__blk1144_dn11 = assign27230_e20968_d_n11;
        locals.var_t0__blk1144_dn12 = assign27230_e20968_d_n12;

        let (assign27240_e20975, assign27240_e20975_d_n3, assign27240_e20975_d_n4, assign27240_e20975_d_n5, assign27240_e20975_d_n6, assign27240_e20975_d_n7, assign27240_e20975_d_n8, assign27240_e20975_d_n9, assign27240_e20975_d_n10, assign27240_e20975_d_n11, assign27240_e20975_d_n12,) = {
    if ((locals.var_guard1644 != 0.0) && (locals.var_guard1653 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27240_e20975;
        locals.var_t0__blk1144_dn3 = assign27240_e20975_d_n3;
        locals.var_t0__blk1144_dn4 = assign27240_e20975_d_n4;
        locals.var_t0__blk1144_dn5 = assign27240_e20975_d_n5;
        locals.var_t0__blk1144_dn6 = assign27240_e20975_d_n6;
        locals.var_t0__blk1144_dn7 = assign27240_e20975_d_n7;
        locals.var_t0__blk1144_dn8 = assign27240_e20975_d_n8;
        locals.var_t0__blk1144_dn9 = assign27240_e20975_d_n9;
        locals.var_t0__blk1144_dn10 = assign27240_e20975_d_n10;
        locals.var_t0__blk1144_dn11 = assign27240_e20975_d_n11;
        locals.var_t0__blk1144_dn12 = assign27240_e20975_d_n12;

        let assign27250_e20978: f64 = if locals.var_t0__blk1144 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1654 = assign27250_e20978;

        let (assign27260_e20984, assign27260_e20984_d_n3, assign27260_e20984_d_n4, assign27260_e20984_d_n5, assign27260_e20984_d_n6, assign27260_e20984_d_n7, assign27260_e20984_d_n8, assign27260_e20984_d_n9, assign27260_e20984_d_n10, assign27260_e20984_d_n11, assign27260_e20984_d_n12,) = {
    if ((locals.var_guard1644 != 0.0) && (locals.var_guard1654 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27260_e20984;
        locals.var_t0__blk1144_dn3 = assign27260_e20984_d_n3;
        locals.var_t0__blk1144_dn4 = assign27260_e20984_d_n4;
        locals.var_t0__blk1144_dn5 = assign27260_e20984_d_n5;
        locals.var_t0__blk1144_dn6 = assign27260_e20984_d_n6;
        locals.var_t0__blk1144_dn7 = assign27260_e20984_d_n7;
        locals.var_t0__blk1144_dn8 = assign27260_e20984_d_n8;
        locals.var_t0__blk1144_dn9 = assign27260_e20984_d_n9;
        locals.var_t0__blk1144_dn10 = assign27260_e20984_d_n10;
        locals.var_t0__blk1144_dn11 = assign27260_e20984_d_n11;
        locals.var_t0__blk1144_dn12 = assign27260_e20984_d_n12;

        let (assign27270_e21000, assign27270_e21000_d_n3, assign27270_e21000_d_n4, assign27270_e21000_d_n5, assign27270_e21000_d_n6, assign27270_e21000_d_n7, assign27270_e21000_d_n8, assign27270_e21000_d_n9, assign27270_e21000_d_n10, assign27270_e21000_d_n11, assign27270_e21000_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign27270_e20988: f64 = (locals.var_leff * locals.var_weff);
        let assign27270_e20990: f64 = (assign27270_e20988 / locals.var_b4soinseg);
        let assign27270_e20993: f64 = (locals.var_b4soiagbcpd / locals.var_b4soinf);
        let assign27270_e20994: f64 = (assign27270_e20990 + assign27270_e20993);
        let assign27270_e20996: f64 = (assign27270_e20994 * locals.var_agb2);
        let assign27270_e20998: f64 = (assign27270_e20996 * locals.var_oxideratio);
        (assign27270_e20998, (((((locals.var_leff_dn3 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn3)) / locals.var_b4soinseg) * locals.var_agb2) * locals.var_oxideratio), (((((locals.var_leff_dn4 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn4)) / locals.var_b4soinseg) * locals.var_agb2) * locals.var_oxideratio), (((((locals.var_leff_dn5 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn5)) / locals.var_b4soinseg) * locals.var_agb2) * locals.var_oxideratio), (((((locals.var_leff_dn6 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn6)) / locals.var_b4soinseg) * locals.var_agb2) * locals.var_oxideratio), (((((locals.var_leff_dn7 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn7)) / locals.var_b4soinseg) * locals.var_agb2) * locals.var_oxideratio), (((((locals.var_leff_dn8 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn8)) / locals.var_b4soinseg) * locals.var_agb2) * locals.var_oxideratio), (((((locals.var_leff_dn9 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn9)) / locals.var_b4soinseg) * locals.var_agb2) * locals.var_oxideratio), (((((locals.var_leff_dn10 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn10)) / locals.var_b4soinseg) * locals.var_agb2) * locals.var_oxideratio), (((((locals.var_leff_dn11 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn11)) / locals.var_b4soinseg) * locals.var_agb2) * locals.var_oxideratio), (((((locals.var_leff_dn12 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn12)) / locals.var_b4soinseg) * locals.var_agb2) * locals.var_oxideratio),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign27270_e21000;
        locals.var_t1__blk1145_dn3 = assign27270_e21000_d_n3;
        locals.var_t1__blk1145_dn4 = assign27270_e21000_d_n4;
        locals.var_t1__blk1145_dn5 = assign27270_e21000_d_n5;
        locals.var_t1__blk1145_dn6 = assign27270_e21000_d_n6;
        locals.var_t1__blk1145_dn7 = assign27270_e21000_d_n7;
        locals.var_t1__blk1145_dn8 = assign27270_e21000_d_n8;
        locals.var_t1__blk1145_dn9 = assign27270_e21000_d_n9;
        locals.var_t1__blk1145_dn10 = assign27270_e21000_d_n10;
        locals.var_t1__blk1145_dn11 = assign27270_e21000_d_n11;
        locals.var_t1__blk1145_dn12 = assign27270_e21000_d_n12;

        let (assign27280_e21006, assign27280_e21006_d_n3, assign27280_e21006_d_n4, assign27280_e21006_d_n5, assign27280_e21006_d_n6, assign27280_e21006_d_n7, assign27280_e21006_d_n8, assign27280_e21006_d_n9, assign27280_e21006_d_n10, assign27280_e21006_d_n11, assign27280_e21006_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign27280_e21004: f64 = (locals.var_bgb2 * locals.var_b4soitoxqm);
        (assign27280_e21004, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign27280_e21006;
        locals.var_t2__blk1146_dn3 = assign27280_e21006_d_n3;
        locals.var_t2__blk1146_dn4 = assign27280_e21006_d_n4;
        locals.var_t2__blk1146_dn5 = assign27280_e21006_d_n5;
        locals.var_t2__blk1146_dn6 = assign27280_e21006_d_n6;
        locals.var_t2__blk1146_dn7 = assign27280_e21006_d_n7;
        locals.var_t2__blk1146_dn8 = assign27280_e21006_d_n8;
        locals.var_t2__blk1146_dn9 = assign27280_e21006_d_n9;
        locals.var_t2__blk1146_dn10 = assign27280_e21006_d_n10;
        locals.var_t2__blk1146_dn11 = assign27280_e21006_d_n11;
        locals.var_t2__blk1146_dn12 = assign27280_e21006_d_n12;

        let (assign27290_e21010, assign27290_e21010_d_n3, assign27290_e21010_d_n4, assign27290_e21010_d_n5, assign27290_e21010_d_n6, assign27290_e21010_d_n7, assign27290_e21010_d_n8, assign27290_e21010_d_n9, assign27290_e21010_d_n10, assign27290_e21010_d_n11, assign27290_e21010_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        (locals.var_pparam_b4soialphagb2, locals.var_pparam_b4soialphagb2_dn3, locals.var_pparam_b4soialphagb2_dn4, locals.var_pparam_b4soialphagb2_dn5, locals.var_pparam_b4soialphagb2_dn6, locals.var_pparam_b4soialphagb2_dn7, locals.var_pparam_b4soialphagb2_dn8, locals.var_pparam_b4soialphagb2_dn9, locals.var_pparam_b4soialphagb2_dn10, locals.var_pparam_b4soialphagb2_dn11, locals.var_pparam_b4soialphagb2_dn12,)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign27290_e21010;
        locals.var_t3__blk1147_dn3 = assign27290_e21010_d_n3;
        locals.var_t3__blk1147_dn4 = assign27290_e21010_d_n4;
        locals.var_t3__blk1147_dn5 = assign27290_e21010_d_n5;
        locals.var_t3__blk1147_dn6 = assign27290_e21010_d_n6;
        locals.var_t3__blk1147_dn7 = assign27290_e21010_d_n7;
        locals.var_t3__blk1147_dn8 = assign27290_e21010_d_n8;
        locals.var_t3__blk1147_dn9 = assign27290_e21010_d_n9;
        locals.var_t3__blk1147_dn10 = assign27290_e21010_d_n10;
        locals.var_t3__blk1147_dn11 = assign27290_e21010_d_n11;
        locals.var_t3__blk1147_dn12 = assign27290_e21010_d_n12;

        let (assign27300_e21014, assign27300_e21014_d_n3, assign27300_e21014_d_n4, assign27300_e21014_d_n5, assign27300_e21014_d_n6, assign27300_e21014_d_n7, assign27300_e21014_d_n8, assign27300_e21014_d_n9, assign27300_e21014_d_n10, assign27300_e21014_d_n11, assign27300_e21014_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        (locals.var_pparam_b4soibetagb2, locals.var_pparam_b4soibetagb2_dn3, locals.var_pparam_b4soibetagb2_dn4, locals.var_pparam_b4soibetagb2_dn5, locals.var_pparam_b4soibetagb2_dn6, locals.var_pparam_b4soibetagb2_dn7, locals.var_pparam_b4soibetagb2_dn8, locals.var_pparam_b4soibetagb2_dn9, locals.var_pparam_b4soibetagb2_dn10, locals.var_pparam_b4soibetagb2_dn11, locals.var_pparam_b4soibetagb2_dn12,)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign27300_e21014;
        locals.var_t4__blk1148_dn3 = assign27300_e21014_d_n3;
        locals.var_t4__blk1148_dn4 = assign27300_e21014_d_n4;
        locals.var_t4__blk1148_dn5 = assign27300_e21014_d_n5;
        locals.var_t4__blk1148_dn6 = assign27300_e21014_d_n6;
        locals.var_t4__blk1148_dn7 = assign27300_e21014_d_n7;
        locals.var_t4__blk1148_dn8 = assign27300_e21014_d_n8;
        locals.var_t4__blk1148_dn9 = assign27300_e21014_d_n9;
        locals.var_t4__blk1148_dn10 = assign27300_e21014_d_n10;
        locals.var_t4__blk1148_dn11 = assign27300_e21014_d_n11;
        locals.var_t4__blk1148_dn12 = assign27300_e21014_d_n12;

        let (assign27310_e21026, assign27310_e21026_d_n3, assign27310_e21026_d_n4, assign27310_e21026_d_n5, assign27310_e21026_d_n6, assign27310_e21026_d_n7, assign27310_e21026_d_n8, assign27310_e21026_d_n9, assign27310_e21026_d_n10, assign27310_e21026_d_n11, assign27310_e21026_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign27310_e21020: f64 = (locals.var_t4__blk1148 * locals.var_vox);
        let assign27310_e21021: f64 = (locals.var_t3__blk1147 - assign27310_e21020);
        let assign27310_e21022: f64 = (locals.var_t2__blk1146 * assign27310_e21021);
        let assign27310_e21024: f64 = (assign27310_e21022 / locals.var_t0__blk1144);
        (assign27310_e21024, (((((locals.var_t2__blk1146_dn3 * assign27310_e21021) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn3 - ((locals.var_t4__blk1148_dn3 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn3))))) * locals.var_t0__blk1144) - (assign27310_e21022 * locals.var_t0__blk1144_dn3)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn4 * assign27310_e21021) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn4 - ((locals.var_t4__blk1148_dn4 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn4))))) * locals.var_t0__blk1144) - (assign27310_e21022 * locals.var_t0__blk1144_dn4)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn5 * assign27310_e21021) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn5 - ((locals.var_t4__blk1148_dn5 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn5))))) * locals.var_t0__blk1144) - (assign27310_e21022 * locals.var_t0__blk1144_dn5)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn6 * assign27310_e21021) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn6 - ((locals.var_t4__blk1148_dn6 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn6))))) * locals.var_t0__blk1144) - (assign27310_e21022 * locals.var_t0__blk1144_dn6)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn7 * assign27310_e21021) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn7 - ((locals.var_t4__blk1148_dn7 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn7))))) * locals.var_t0__blk1144) - (assign27310_e21022 * locals.var_t0__blk1144_dn7)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn8 * assign27310_e21021) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn8 - ((locals.var_t4__blk1148_dn8 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn8))))) * locals.var_t0__blk1144) - (assign27310_e21022 * locals.var_t0__blk1144_dn8)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn9 * assign27310_e21021) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn9 - ((locals.var_t4__blk1148_dn9 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn9))))) * locals.var_t0__blk1144) - (assign27310_e21022 * locals.var_t0__blk1144_dn9)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn10 * assign27310_e21021) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn10 - ((locals.var_t4__blk1148_dn10 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn10))))) * locals.var_t0__blk1144) - (assign27310_e21022 * locals.var_t0__blk1144_dn10)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn11 * assign27310_e21021) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn11 - ((locals.var_t4__blk1148_dn11 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn11))))) * locals.var_t0__blk1144) - (assign27310_e21022 * locals.var_t0__blk1144_dn11)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)), (((((locals.var_t2__blk1146_dn12 * assign27310_e21021) + (locals.var_t2__blk1146 * (locals.var_t3__blk1147_dn12 - ((locals.var_t4__blk1148_dn12 * locals.var_vox) + (locals.var_t4__blk1148 * locals.var_vox_dn12))))) * locals.var_t0__blk1144) - (assign27310_e21022 * locals.var_t0__blk1144_dn12)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144)),)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign27310_e21026;
        locals.var_t6__blk1150_dn3 = assign27310_e21026_d_n3;
        locals.var_t6__blk1150_dn4 = assign27310_e21026_d_n4;
        locals.var_t6__blk1150_dn5 = assign27310_e21026_d_n5;
        locals.var_t6__blk1150_dn6 = assign27310_e21026_d_n6;
        locals.var_t6__blk1150_dn7 = assign27310_e21026_d_n7;
        locals.var_t6__blk1150_dn8 = assign27310_e21026_d_n8;
        locals.var_t6__blk1150_dn9 = assign27310_e21026_d_n9;
        locals.var_t6__blk1150_dn10 = assign27310_e21026_d_n10;
        locals.var_t6__blk1150_dn11 = assign27310_e21026_d_n11;
        locals.var_t6__blk1150_dn12 = assign27310_e21026_d_n12;

        let assign27320_e21029: f64 = if locals.var_t6__blk1150 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1655 = assign27320_e21029;

        let (assign27330_e21041, assign27330_e21041_d_n3, assign27330_e21041_d_n4, assign27330_e21041_d_n5, assign27330_e21041_d_n6, assign27330_e21041_d_n7, assign27330_e21041_d_n8, assign27330_e21041_d_n9, assign27330_e21041_d_n10, assign27330_e21041_d_n11, assign27330_e21041_d_n12,) = {
    if ((locals.var_guard1644 != 0.0) && (locals.var_guard1655 != 0.0)) {
        let assign27330_e21036: f64 = (1.0 + locals.var_t6__blk1150);
        let assign27330_e21038: f64 = (assign27330_e21036 - 100.0);
        let assign27330_e21039: f64 = (2.688117142e43 * assign27330_e21038);
        (assign27330_e21039, (2.688117142e43 * locals.var_t6__blk1150_dn3), (2.688117142e43 * locals.var_t6__blk1150_dn4), (2.688117142e43 * locals.var_t6__blk1150_dn5), (2.688117142e43 * locals.var_t6__blk1150_dn6), (2.688117142e43 * locals.var_t6__blk1150_dn7), (2.688117142e43 * locals.var_t6__blk1150_dn8), (2.688117142e43 * locals.var_t6__blk1150_dn9), (2.688117142e43 * locals.var_t6__blk1150_dn10), (2.688117142e43 * locals.var_t6__blk1150_dn11), (2.688117142e43 * locals.var_t6__blk1150_dn12),)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign27330_e21041;
        locals.var_t5__blk1149_dn3 = assign27330_e21041_d_n3;
        locals.var_t5__blk1149_dn4 = assign27330_e21041_d_n4;
        locals.var_t5__blk1149_dn5 = assign27330_e21041_d_n5;
        locals.var_t5__blk1149_dn6 = assign27330_e21041_d_n6;
        locals.var_t5__blk1149_dn7 = assign27330_e21041_d_n7;
        locals.var_t5__blk1149_dn8 = assign27330_e21041_d_n8;
        locals.var_t5__blk1149_dn9 = assign27330_e21041_d_n9;
        locals.var_t5__blk1149_dn10 = assign27330_e21041_d_n10;
        locals.var_t5__blk1149_dn11 = assign27330_e21041_d_n11;
        locals.var_t5__blk1149_dn12 = assign27330_e21041_d_n12;

        let assign27340_e21044: f64 = (-100.0);
        let assign27340_e21045: f64 = if locals.var_t6__blk1150 < assign27340_e21044 { 1.0 } else { 0.0 };
        locals.var_guard1656 = assign27340_e21045;

        let (assign27350_e21054, assign27350_e21054_d_n3, assign27350_e21054_d_n4, assign27350_e21054_d_n5, assign27350_e21054_d_n6, assign27350_e21054_d_n7, assign27350_e21054_d_n8, assign27350_e21054_d_n9, assign27350_e21054_d_n10, assign27350_e21054_d_n11, assign27350_e21054_d_n12,) = {
    if (((locals.var_guard1644 != 0.0) && (locals.var_guard1655 == 0.0)) && (locals.var_guard1656 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign27350_e21054;
        locals.var_t5__blk1149_dn3 = assign27350_e21054_d_n3;
        locals.var_t5__blk1149_dn4 = assign27350_e21054_d_n4;
        locals.var_t5__blk1149_dn5 = assign27350_e21054_d_n5;
        locals.var_t5__blk1149_dn6 = assign27350_e21054_d_n6;
        locals.var_t5__blk1149_dn7 = assign27350_e21054_d_n7;
        locals.var_t5__blk1149_dn8 = assign27350_e21054_d_n8;
        locals.var_t5__blk1149_dn9 = assign27350_e21054_d_n9;
        locals.var_t5__blk1149_dn10 = assign27350_e21054_d_n10;
        locals.var_t5__blk1149_dn11 = assign27350_e21054_d_n11;
        locals.var_t5__blk1149_dn12 = assign27350_e21054_d_n12;

        let (assign27360_e21065, assign27360_e21065_d_n3, assign27360_e21065_d_n4, assign27360_e21065_d_n5, assign27360_e21065_d_n6, assign27360_e21065_d_n7, assign27360_e21065_d_n8, assign27360_e21065_d_n9, assign27360_e21065_d_n10, assign27360_e21065_d_n11, assign27360_e21065_d_n12,) = {
    if (((locals.var_guard1644 != 0.0) && (locals.var_guard1655 == 0.0)) && (locals.var_guard1656 == 0.0)) {
        let assign27360_e21063: f64 = (locals.var_t6__blk1150).exp();
        (assign27360_e21063, (assign27360_e21063 * locals.var_t6__blk1150_dn3), (assign27360_e21063 * locals.var_t6__blk1150_dn4), (assign27360_e21063 * locals.var_t6__blk1150_dn5), (assign27360_e21063 * locals.var_t6__blk1150_dn6), (assign27360_e21063 * locals.var_t6__blk1150_dn7), (assign27360_e21063 * locals.var_t6__blk1150_dn8), (assign27360_e21063 * locals.var_t6__blk1150_dn9), (assign27360_e21063 * locals.var_t6__blk1150_dn10), (assign27360_e21063 * locals.var_t6__blk1150_dn11), (assign27360_e21063 * locals.var_t6__blk1150_dn12),)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign27360_e21065;
        locals.var_t5__blk1149_dn3 = assign27360_e21065_d_n3;
        locals.var_t5__blk1149_dn4 = assign27360_e21065_d_n4;
        locals.var_t5__blk1149_dn5 = assign27360_e21065_d_n5;
        locals.var_t5__blk1149_dn6 = assign27360_e21065_d_n6;
        locals.var_t5__blk1149_dn7 = assign27360_e21065_d_n7;
        locals.var_t5__blk1149_dn8 = assign27360_e21065_d_n8;
        locals.var_t5__blk1149_dn9 = assign27360_e21065_d_n9;
        locals.var_t5__blk1149_dn10 = assign27360_e21065_d_n10;
        locals.var_t5__blk1149_dn11 = assign27360_e21065_d_n11;
        locals.var_t5__blk1149_dn12 = assign27360_e21065_d_n12;

        let (assign27370_e21075, assign27370_e21075_d_n3, assign27370_e21075_d_n4, assign27370_e21075_d_n5, assign27370_e21075_d_n6, assign27370_e21075_d_n7, assign27370_e21075_d_n8, assign27370_e21075_d_n9, assign27370_e21075_d_n10, assign27370_e21075_d_n11, assign27370_e21075_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign27370_e21069: f64 = (locals.var_t1__blk1145 * locals.var_vgb);
        let assign27370_e21071: f64 = (assign27370_e21069 * locals.var_vaux);
        let assign27370_e21073: f64 = (assign27370_e21071 * locals.var_t5__blk1149);
        (assign27370_e21073, ((((((locals.var_t1__blk1145_dn3 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn3)) * locals.var_vaux) + (assign27370_e21069 * locals.var_vaux_dn3)) * locals.var_t5__blk1149) + (assign27370_e21071 * locals.var_t5__blk1149_dn3)), ((((((locals.var_t1__blk1145_dn4 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn4)) * locals.var_vaux) + (assign27370_e21069 * locals.var_vaux_dn4)) * locals.var_t5__blk1149) + (assign27370_e21071 * locals.var_t5__blk1149_dn4)), ((((((locals.var_t1__blk1145_dn5 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn5)) * locals.var_vaux) + (assign27370_e21069 * locals.var_vaux_dn5)) * locals.var_t5__blk1149) + (assign27370_e21071 * locals.var_t5__blk1149_dn5)), ((((((locals.var_t1__blk1145_dn6 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn6)) * locals.var_vaux) + (assign27370_e21069 * locals.var_vaux_dn6)) * locals.var_t5__blk1149) + (assign27370_e21071 * locals.var_t5__blk1149_dn6)), ((((((locals.var_t1__blk1145_dn7 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn7)) * locals.var_vaux) + (assign27370_e21069 * locals.var_vaux_dn7)) * locals.var_t5__blk1149) + (assign27370_e21071 * locals.var_t5__blk1149_dn7)), ((((((locals.var_t1__blk1145_dn8 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn8)) * locals.var_vaux) + (assign27370_e21069 * locals.var_vaux_dn8)) * locals.var_t5__blk1149) + (assign27370_e21071 * locals.var_t5__blk1149_dn8)), ((((((locals.var_t1__blk1145_dn9 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn9)) * locals.var_vaux) + (assign27370_e21069 * locals.var_vaux_dn9)) * locals.var_t5__blk1149) + (assign27370_e21071 * locals.var_t5__blk1149_dn9)), ((((((locals.var_t1__blk1145_dn10 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn10)) * locals.var_vaux) + (assign27370_e21069 * locals.var_vaux_dn10)) * locals.var_t5__blk1149) + (assign27370_e21071 * locals.var_t5__blk1149_dn10)), ((((((locals.var_t1__blk1145_dn11 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn11)) * locals.var_vaux) + (assign27370_e21069 * locals.var_vaux_dn11)) * locals.var_t5__blk1149) + (assign27370_e21071 * locals.var_t5__blk1149_dn11)), ((((((locals.var_t1__blk1145_dn12 * locals.var_vgb) + (locals.var_t1__blk1145 * locals.var_vgb_dn12)) * locals.var_vaux) + (assign27370_e21069 * locals.var_vaux_dn12)) * locals.var_t5__blk1149) + (assign27370_e21071 * locals.var_t5__blk1149_dn12)),)
    } else {
        (locals.var_igb2, locals.var_igb2_dn3, locals.var_igb2_dn4, locals.var_igb2_dn5, locals.var_igb2_dn6, locals.var_igb2_dn7, locals.var_igb2_dn8, locals.var_igb2_dn9, locals.var_igb2_dn10, locals.var_igb2_dn11, locals.var_igb2_dn12,)
    }
};
        locals.var_igb2 = assign27370_e21075;
        locals.var_igb2_dn3 = assign27370_e21075_d_n3;
        locals.var_igb2_dn4 = assign27370_e21075_d_n4;
        locals.var_igb2_dn5 = assign27370_e21075_d_n5;
        locals.var_igb2_dn6 = assign27370_e21075_d_n6;
        locals.var_igb2_dn7 = assign27370_e21075_d_n7;
        locals.var_igb2_dn8 = assign27370_e21075_d_n8;
        locals.var_igb2_dn9 = assign27370_e21075_d_n9;
        locals.var_igb2_dn10 = assign27370_e21075_d_n10;
        locals.var_igb2_dn11 = assign27370_e21075_d_n11;
        locals.var_igb2_dn12 = assign27370_e21075_d_n12;

        let assign27380_e21078: f64 = if locals.var_vgb >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1657 = assign27380_e21078;

        let (assign27390_e21084, assign27390_e21084_d_n3, assign27390_e21084_d_n4, assign27390_e21084_d_n5, assign27390_e21084_d_n6, assign27390_e21084_d_n7, assign27390_e21084_d_n8, assign27390_e21084_d_n9, assign27390_e21084_d_n10, assign27390_e21084_d_n11, assign27390_e21084_d_n12,) = {
    if ((locals.var_guard1644 != 0.0) && (locals.var_guard1657 != 0.0)) {
        (locals.var_igb1, locals.var_igb1_dn3, locals.var_igb1_dn4, locals.var_igb1_dn5, locals.var_igb1_dn6, locals.var_igb1_dn7, locals.var_igb1_dn8, locals.var_igb1_dn9, locals.var_igb1_dn10, locals.var_igb1_dn11, locals.var_igb1_dn12,)
    } else {
        (locals.var_igb_1, locals.var_igb_1_dn3, locals.var_igb_1_dn4, locals.var_igb_1_dn5, locals.var_igb_1_dn6, locals.var_igb_1_dn7, locals.var_igb_1_dn8, locals.var_igb_1_dn9, locals.var_igb_1_dn10, locals.var_igb_1_dn11, locals.var_igb_1_dn12,)
    }
};
        locals.var_igb_1 = assign27390_e21084;
        locals.var_igb_1_dn3 = assign27390_e21084_d_n3;
        locals.var_igb_1_dn4 = assign27390_e21084_d_n4;
        locals.var_igb_1_dn5 = assign27390_e21084_d_n5;
        locals.var_igb_1_dn6 = assign27390_e21084_d_n6;
        locals.var_igb_1_dn7 = assign27390_e21084_d_n7;
        locals.var_igb_1_dn8 = assign27390_e21084_d_n8;
        locals.var_igb_1_dn9 = assign27390_e21084_d_n9;
        locals.var_igb_1_dn10 = assign27390_e21084_d_n10;
        locals.var_igb_1_dn11 = assign27390_e21084_d_n11;
        locals.var_igb_1_dn12 = assign27390_e21084_d_n12;

        let (assign27400_e21091, assign27400_e21091_d_n3, assign27400_e21091_d_n4, assign27400_e21091_d_n5, assign27400_e21091_d_n6, assign27400_e21091_d_n7, assign27400_e21091_d_n8, assign27400_e21091_d_n9, assign27400_e21091_d_n10, assign27400_e21091_d_n11, assign27400_e21091_d_n12,) = {
    if ((locals.var_guard1644 != 0.0) && (locals.var_guard1657 == 0.0)) {
        (locals.var_igb2, locals.var_igb2_dn3, locals.var_igb2_dn4, locals.var_igb2_dn5, locals.var_igb2_dn6, locals.var_igb2_dn7, locals.var_igb2_dn8, locals.var_igb2_dn9, locals.var_igb2_dn10, locals.var_igb2_dn11, locals.var_igb2_dn12,)
    } else {
        (locals.var_igb_1, locals.var_igb_1_dn3, locals.var_igb_1_dn4, locals.var_igb_1_dn5, locals.var_igb_1_dn6, locals.var_igb_1_dn7, locals.var_igb_1_dn8, locals.var_igb_1_dn9, locals.var_igb_1_dn10, locals.var_igb_1_dn11, locals.var_igb_1_dn12,)
    }
};
        locals.var_igb_1 = assign27400_e21091;
        locals.var_igb_1_dn3 = assign27400_e21091_d_n3;
        locals.var_igb_1_dn4 = assign27400_e21091_d_n4;
        locals.var_igb_1_dn5 = assign27400_e21091_d_n5;
        locals.var_igb_1_dn6 = assign27400_e21091_d_n6;
        locals.var_igb_1_dn7 = assign27400_e21091_d_n7;
        locals.var_igb_1_dn8 = assign27400_e21091_d_n8;
        locals.var_igb_1_dn9 = assign27400_e21091_d_n9;
        locals.var_igb_1_dn10 = assign27400_e21091_d_n10;
        locals.var_igb_1_dn11 = assign27400_e21091_d_n11;
        locals.var_igb_1_dn12 = assign27400_e21091_d_n12;

        let (assign27410_e21097, assign27410_e21097_d_n3, assign27410_e21097_d_n4, assign27410_e21097_d_n5, assign27410_e21097_d_n6, assign27410_e21097_d_n7, assign27410_e21097_d_n8, assign27410_e21097_d_n9, assign27410_e21097_d_n10, assign27410_e21097_d_n11, assign27410_e21097_d_n12,) = {
    if (locals.var_guard1644 != 0.0) {
        let assign27410_e21095: f64 = (locals.var_vfb + locals.var_eggbcp2);
        (assign27410_e21095, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11, locals.var_vfb_dn12,)
    } else {
        (locals.var_vfb2, locals.var_vfb2_dn3, locals.var_vfb2_dn4, locals.var_vfb2_dn5, locals.var_vfb2_dn6, locals.var_vfb2_dn7, locals.var_vfb2_dn8, locals.var_vfb2_dn9, locals.var_vfb2_dn10, locals.var_vfb2_dn11, locals.var_vfb2_dn12,)
    }
};
        locals.var_vfb2 = assign27410_e21097;
        locals.var_vfb2_dn3 = assign27410_e21097_d_n3;
        locals.var_vfb2_dn4 = assign27410_e21097_d_n4;
        locals.var_vfb2_dn5 = assign27410_e21097_d_n5;
        locals.var_vfb2_dn6 = assign27410_e21097_d_n6;
        locals.var_vfb2_dn7 = assign27410_e21097_d_n7;
        locals.var_vfb2_dn8 = assign27410_e21097_d_n8;
        locals.var_vfb2_dn9 = assign27410_e21097_d_n9;
        locals.var_vfb2_dn10 = assign27410_e21097_d_n10;
        locals.var_vfb2_dn11 = assign27410_e21097_d_n11;
        locals.var_vfb2_dn12 = assign27410_e21097_d_n12;

        let (assign27420_e21102, assign27420_e21102_d_n3, assign27420_e21102_d_n4, assign27420_e21102_d_n5, assign27420_e21102_d_n6, assign27420_e21102_d_n7, assign27420_e21102_d_n8, assign27420_e21102_d_n9, assign27420_e21102_d_n10, assign27420_e21102_d_n11, assign27420_e21102_d_n12,) = {
    if (locals.var_guard1644 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igb_1, locals.var_igb_1_dn3, locals.var_igb_1_dn4, locals.var_igb_1_dn5, locals.var_igb_1_dn6, locals.var_igb_1_dn7, locals.var_igb_1_dn8, locals.var_igb_1_dn9, locals.var_igb_1_dn10, locals.var_igb_1_dn11, locals.var_igb_1_dn12,)
    }
};
        locals.var_igb_1 = assign27420_e21102;
        locals.var_igb_1_dn3 = assign27420_e21102_d_n3;
        locals.var_igb_1_dn4 = assign27420_e21102_d_n4;
        locals.var_igb_1_dn5 = assign27420_e21102_d_n5;
        locals.var_igb_1_dn6 = assign27420_e21102_d_n6;
        locals.var_igb_1_dn7 = assign27420_e21102_d_n7;
        locals.var_igb_1_dn8 = assign27420_e21102_d_n8;
        locals.var_igb_1_dn9 = assign27420_e21102_d_n9;
        locals.var_igb_1_dn10 = assign27420_e21102_d_n10;
        locals.var_igb_1_dn11 = assign27420_e21102_d_n11;
        locals.var_igb_1_dn12 = assign27420_e21102_d_n12;

        let assign27430_e21105: f64 = (locals.var_b4soitype * locals.var_igb_1);
        locals.var_b4soiig = assign27430_e21105;
        locals.var_b4soiig_dn3 = (locals.var_b4soitype * locals.var_igb_1_dn3);
        locals.var_b4soiig_dn4 = (locals.var_b4soitype * locals.var_igb_1_dn4);
        locals.var_b4soiig_dn5 = (locals.var_b4soitype * locals.var_igb_1_dn5);
        locals.var_b4soiig_dn6 = (locals.var_b4soitype * locals.var_igb_1_dn6);
        locals.var_b4soiig_dn7 = (locals.var_b4soitype * locals.var_igb_1_dn7);
        locals.var_b4soiig_dn8 = (locals.var_b4soitype * locals.var_igb_1_dn8);
        locals.var_b4soiig_dn9 = (locals.var_b4soitype * locals.var_igb_1_dn9);
        locals.var_b4soiig_dn10 = (locals.var_b4soitype * locals.var_igb_1_dn10);
        locals.var_b4soiig_dn11 = (locals.var_b4soitype * locals.var_igb_1_dn11);
        locals.var_b4soiig_dn12 = (locals.var_b4soitype * locals.var_igb_1_dn12);

        let assign27440_e21124: f64 = if (((((locals.var_b4soiigbmod != 0.0) && (locals.var_b4soisoimod != 2.0)) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) && (locals.var_vgp < locals.var_vfb2)) { 1.0 } else { 0.0 };
        locals.var_guard1658 = assign27440_e21124;

    }

    pub(super) fn stamp_transient_block_72(
        locals: &mut StampLocals,
    ) {
        let (assign27450_e21130, assign27450_e21130_d_n3, assign27450_e21130_d_n4, assign27450_e21130_d_n5, assign27450_e21130_d_n6, assign27450_e21130_d_n7, assign27450_e21130_d_n8, assign27450_e21130_d_n9, assign27450_e21130_d_n10, assign27450_e21130_d_n11, assign27450_e21130_d_n12,) = {
    if (locals.var_guard1658 != 0.0) {
        let assign27450_e21128: f64 = (locals.var_vgp - locals.var_vfb2);
        (assign27450_e21128, (-locals.var_vfb2_dn3), (locals.var_vgp_dn4 - locals.var_vfb2_dn4), (-locals.var_vfb2_dn5), (-locals.var_vfb2_dn6), (-locals.var_vfb2_dn7), (-locals.var_vfb2_dn8), (locals.var_vgp_dn9 - locals.var_vfb2_dn9), (-locals.var_vfb2_dn10), (-locals.var_vfb2_dn11), (-locals.var_vfb2_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27450_e21130;
        locals.var_t0__blk1144_dn3 = assign27450_e21130_d_n3;
        locals.var_t0__blk1144_dn4 = assign27450_e21130_d_n4;
        locals.var_t0__blk1144_dn5 = assign27450_e21130_d_n5;
        locals.var_t0__blk1144_dn6 = assign27450_e21130_d_n6;
        locals.var_t0__blk1144_dn7 = assign27450_e21130_d_n7;
        locals.var_t0__blk1144_dn8 = assign27450_e21130_d_n8;
        locals.var_t0__blk1144_dn9 = assign27450_e21130_d_n9;
        locals.var_t0__blk1144_dn10 = assign27450_e21130_d_n10;
        locals.var_t0__blk1144_dn11 = assign27450_e21130_d_n11;
        locals.var_t0__blk1144_dn12 = assign27450_e21130_d_n12;

        let (assign27460_e21139, assign27460_e21139_d_n3, assign27460_e21139_d_n4, assign27460_e21139_d_n5, assign27460_e21139_d_n6, assign27460_e21139_d_n7, assign27460_e21139_d_n8, assign27460_e21139_d_n9, assign27460_e21139_d_n10, assign27460_e21139_d_n11, assign27460_e21139_d_n12,) = {
    if (locals.var_guard1658 != 0.0) {
        let assign27460_e21134: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign27460_e21136: f64 = (assign27460_e21134 + 0.0001);
        let assign27460_e21137: f64 = (assign27460_e21136).sqrt();
        (assign27460_e21137, (((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) / (2.0 * assign27460_e21137)), (((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) / (2.0 * assign27460_e21137)), (((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) / (2.0 * assign27460_e21137)), (((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) / (2.0 * assign27460_e21137)), (((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) / (2.0 * assign27460_e21137)), (((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) / (2.0 * assign27460_e21137)), (((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) / (2.0 * assign27460_e21137)), (((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) / (2.0 * assign27460_e21137)), (((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) / (2.0 * assign27460_e21137)), (((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) / (2.0 * assign27460_e21137)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign27460_e21139;
        locals.var_t1__blk1145_dn3 = assign27460_e21139_d_n3;
        locals.var_t1__blk1145_dn4 = assign27460_e21139_d_n4;
        locals.var_t1__blk1145_dn5 = assign27460_e21139_d_n5;
        locals.var_t1__blk1145_dn6 = assign27460_e21139_d_n6;
        locals.var_t1__blk1145_dn7 = assign27460_e21139_d_n7;
        locals.var_t1__blk1145_dn8 = assign27460_e21139_d_n8;
        locals.var_t1__blk1145_dn9 = assign27460_e21139_d_n9;
        locals.var_t1__blk1145_dn10 = assign27460_e21139_d_n10;
        locals.var_t1__blk1145_dn11 = assign27460_e21139_d_n11;
        locals.var_t1__blk1145_dn12 = assign27460_e21139_d_n12;

        let (assign27470_e21150, assign27470_e21150_d_n3, assign27470_e21150_d_n4, assign27470_e21150_d_n5, assign27470_e21150_d_n6, assign27470_e21150_d_n7, assign27470_e21150_d_n8, assign27470_e21150_d_n9, assign27470_e21150_d_n10, assign27470_e21150_d_n11, assign27470_e21150_d_n12,) = {
    if (locals.var_guard1658 != 0.0) {
        let assign27470_e21143: f64 = (-locals.var_t0__blk1144);
        let assign27470_e21145: f64 = (assign27470_e21143 + locals.var_t1__blk1145);
        let assign27470_e21147: f64 = (assign27470_e21145 - 0.01);
        let assign27470_e21148: f64 = (0.5 * assign27470_e21147);
        (assign27470_e21148, (0.5 * ((-locals.var_t0__blk1144_dn3) + locals.var_t1__blk1145_dn3)), (0.5 * ((-locals.var_t0__blk1144_dn4) + locals.var_t1__blk1145_dn4)), (0.5 * ((-locals.var_t0__blk1144_dn5) + locals.var_t1__blk1145_dn5)), (0.5 * ((-locals.var_t0__blk1144_dn6) + locals.var_t1__blk1145_dn6)), (0.5 * ((-locals.var_t0__blk1144_dn7) + locals.var_t1__blk1145_dn7)), (0.5 * ((-locals.var_t0__blk1144_dn8) + locals.var_t1__blk1145_dn8)), (0.5 * ((-locals.var_t0__blk1144_dn9) + locals.var_t1__blk1145_dn9)), (0.5 * ((-locals.var_t0__blk1144_dn10) + locals.var_t1__blk1145_dn10)), (0.5 * ((-locals.var_t0__blk1144_dn11) + locals.var_t1__blk1145_dn11)), (0.5 * ((-locals.var_t0__blk1144_dn12) + locals.var_t1__blk1145_dn12)),)
    } else {
        (locals.var_vgp_eff, locals.var_vgp_eff_dn3, locals.var_vgp_eff_dn4, locals.var_vgp_eff_dn5, locals.var_vgp_eff_dn6, locals.var_vgp_eff_dn7, locals.var_vgp_eff_dn8, locals.var_vgp_eff_dn9, locals.var_vgp_eff_dn10, locals.var_vgp_eff_dn11, locals.var_vgp_eff_dn12,)
    }
};
        locals.var_vgp_eff = assign27470_e21150;
        locals.var_vgp_eff_dn3 = assign27470_e21150_d_n3;
        locals.var_vgp_eff_dn4 = assign27470_e21150_d_n4;
        locals.var_vgp_eff_dn5 = assign27470_e21150_d_n5;
        locals.var_vgp_eff_dn6 = assign27470_e21150_d_n6;
        locals.var_vgp_eff_dn7 = assign27470_e21150_d_n7;
        locals.var_vgp_eff_dn8 = assign27470_e21150_d_n8;
        locals.var_vgp_eff_dn9 = assign27470_e21150_d_n9;
        locals.var_vgp_eff_dn10 = assign27470_e21150_d_n10;
        locals.var_vgp_eff_dn11 = assign27470_e21150_d_n11;
        locals.var_vgp_eff_dn12 = assign27470_e21150_d_n12;

        let (assign27480_e21159, assign27480_e21159_d_n3, assign27480_e21159_d_n4, assign27480_e21159_d_n5, assign27480_e21159_d_n6, assign27480_e21159_d_n7, assign27480_e21159_d_n8, assign27480_e21159_d_n9, assign27480_e21159_d_n10, assign27480_e21159_d_n11, assign27480_e21159_d_n12,) = {
    if (locals.var_guard1658 != 0.0) {
        let (assign27480_e21157,) = {
            if (locals.var_b4soitype == 1.0) {
                (locals.var_agbc2n,)
            } else {
                (locals.var_agbc2p,)
            }
        };
        (assign27480_e21157, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign27480_e21159;
        locals.var_t11_dn3 = assign27480_e21159_d_n3;
        locals.var_t11_dn4 = assign27480_e21159_d_n4;
        locals.var_t11_dn5 = assign27480_e21159_d_n5;
        locals.var_t11_dn6 = assign27480_e21159_d_n6;
        locals.var_t11_dn7 = assign27480_e21159_d_n7;
        locals.var_t11_dn8 = assign27480_e21159_d_n8;
        locals.var_t11_dn9 = assign27480_e21159_d_n9;
        locals.var_t11_dn10 = assign27480_e21159_d_n10;
        locals.var_t11_dn11 = assign27480_e21159_d_n11;
        locals.var_t11_dn12 = assign27480_e21159_d_n12;

        let (assign27490_e21168, assign27490_e21168_d_n3, assign27490_e21168_d_n4, assign27490_e21168_d_n5, assign27490_e21168_d_n6, assign27490_e21168_d_n7, assign27490_e21168_d_n8, assign27490_e21168_d_n9, assign27490_e21168_d_n10, assign27490_e21168_d_n11, assign27490_e21168_d_n12,) = {
    if (locals.var_guard1658 != 0.0) {
        let (assign27490_e21166,) = {
            if (locals.var_b4soitype == 1.0) {
                (locals.var_bgbc2n,)
            } else {
                (locals.var_bgbc2p,)
            }
        };
        (assign27490_e21166, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign27490_e21168;
        locals.var_t12_dn3 = assign27490_e21168_d_n3;
        locals.var_t12_dn4 = assign27490_e21168_d_n4;
        locals.var_t12_dn5 = assign27490_e21168_d_n5;
        locals.var_t12_dn6 = assign27490_e21168_d_n6;
        locals.var_t12_dn7 = assign27490_e21168_d_n7;
        locals.var_t12_dn8 = assign27490_e21168_d_n8;
        locals.var_t12_dn9 = assign27490_e21168_d_n9;
        locals.var_t12_dn10 = assign27490_e21168_d_n10;
        locals.var_t12_dn11 = assign27490_e21168_d_n11;
        locals.var_t12_dn12 = assign27490_e21168_d_n12;

        let (assign27500_e21174, assign27500_e21174_d_n3, assign27500_e21174_d_n4, assign27500_e21174_d_n5, assign27500_e21174_d_n6, assign27500_e21174_d_n7, assign27500_e21174_d_n8, assign27500_e21174_d_n9, assign27500_e21174_d_n10, assign27500_e21174_d_n11, assign27500_e21174_d_n12,) = {
    if (locals.var_guard1658 != 0.0) {
        let assign27500_e21172: f64 = (locals.var_vgp * locals.var_vgp_eff);
        (assign27500_e21172, (locals.var_vgp * locals.var_vgp_eff_dn3), ((locals.var_vgp_dn4 * locals.var_vgp_eff) + (locals.var_vgp * locals.var_vgp_eff_dn4)), (locals.var_vgp * locals.var_vgp_eff_dn5), (locals.var_vgp * locals.var_vgp_eff_dn6), (locals.var_vgp * locals.var_vgp_eff_dn7), (locals.var_vgp * locals.var_vgp_eff_dn8), ((locals.var_vgp_dn9 * locals.var_vgp_eff) + (locals.var_vgp * locals.var_vgp_eff_dn9)), (locals.var_vgp * locals.var_vgp_eff_dn10), (locals.var_vgp * locals.var_vgp_eff_dn11), (locals.var_vgp * locals.var_vgp_eff_dn12),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign27500_e21174;
        locals.var_t2__blk1146_dn3 = assign27500_e21174_d_n3;
        locals.var_t2__blk1146_dn4 = assign27500_e21174_d_n4;
        locals.var_t2__blk1146_dn5 = assign27500_e21174_d_n5;
        locals.var_t2__blk1146_dn6 = assign27500_e21174_d_n6;
        locals.var_t2__blk1146_dn7 = assign27500_e21174_d_n7;
        locals.var_t2__blk1146_dn8 = assign27500_e21174_d_n8;
        locals.var_t2__blk1146_dn9 = assign27500_e21174_d_n9;
        locals.var_t2__blk1146_dn10 = assign27500_e21174_d_n10;
        locals.var_t2__blk1146_dn11 = assign27500_e21174_d_n11;
        locals.var_t2__blk1146_dn12 = assign27500_e21174_d_n12;

        let (assign27510_e21182, assign27510_e21182_d_n3, assign27510_e21182_d_n4, assign27510_e21182_d_n5, assign27510_e21182_d_n6, assign27510_e21182_d_n7, assign27510_e21182_d_n8, assign27510_e21182_d_n9, assign27510_e21182_d_n10, assign27510_e21182_d_n11, assign27510_e21182_d_n12,) = {
    if (locals.var_guard1658 != 0.0) {
        let assign27510_e21178: f64 = (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2);
        let assign27510_e21180: f64 = (assign27510_e21178 - locals.var_pparam_b4soibigbcp2);
        (assign27510_e21180, (((locals.var_pparam_b4soiaigbcp2_dn3 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn3)) - locals.var_pparam_b4soibigbcp2_dn3), (((locals.var_pparam_b4soiaigbcp2_dn4 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn4)) - locals.var_pparam_b4soibigbcp2_dn4), (((locals.var_pparam_b4soiaigbcp2_dn5 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn5)) - locals.var_pparam_b4soibigbcp2_dn5), (((locals.var_pparam_b4soiaigbcp2_dn6 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn6)) - locals.var_pparam_b4soibigbcp2_dn6), (((locals.var_pparam_b4soiaigbcp2_dn7 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn7)) - locals.var_pparam_b4soibigbcp2_dn7), (((locals.var_pparam_b4soiaigbcp2_dn8 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn8)) - locals.var_pparam_b4soibigbcp2_dn8), (((locals.var_pparam_b4soiaigbcp2_dn9 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn9)) - locals.var_pparam_b4soibigbcp2_dn9), (((locals.var_pparam_b4soiaigbcp2_dn10 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn10)) - locals.var_pparam_b4soibigbcp2_dn10), (((locals.var_pparam_b4soiaigbcp2_dn11 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn11)) - locals.var_pparam_b4soibigbcp2_dn11), (((locals.var_pparam_b4soiaigbcp2_dn12 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn12)) - locals.var_pparam_b4soibigbcp2_dn12),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign27510_e21182;
        locals.var_t3__blk1147_dn3 = assign27510_e21182_d_n3;
        locals.var_t3__blk1147_dn4 = assign27510_e21182_d_n4;
        locals.var_t3__blk1147_dn5 = assign27510_e21182_d_n5;
        locals.var_t3__blk1147_dn6 = assign27510_e21182_d_n6;
        locals.var_t3__blk1147_dn7 = assign27510_e21182_d_n7;
        locals.var_t3__blk1147_dn8 = assign27510_e21182_d_n8;
        locals.var_t3__blk1147_dn9 = assign27510_e21182_d_n9;
        locals.var_t3__blk1147_dn10 = assign27510_e21182_d_n10;
        locals.var_t3__blk1147_dn11 = assign27510_e21182_d_n11;
        locals.var_t3__blk1147_dn12 = assign27510_e21182_d_n12;

        let (assign27520_e21188, assign27520_e21188_d_n3, assign27520_e21188_d_n4, assign27520_e21188_d_n5, assign27520_e21188_d_n6, assign27520_e21188_d_n7, assign27520_e21188_d_n8, assign27520_e21188_d_n9, assign27520_e21188_d_n10, assign27520_e21188_d_n11, assign27520_e21188_d_n12,) = {
    if (locals.var_guard1658 != 0.0) {
        let assign27520_e21186: f64 = (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2);
        (assign27520_e21186, ((locals.var_pparam_b4soibigbcp2_dn3 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn3)), ((locals.var_pparam_b4soibigbcp2_dn4 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn4)), ((locals.var_pparam_b4soibigbcp2_dn5 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn5)), ((locals.var_pparam_b4soibigbcp2_dn6 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn6)), ((locals.var_pparam_b4soibigbcp2_dn7 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn7)), ((locals.var_pparam_b4soibigbcp2_dn8 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn8)), ((locals.var_pparam_b4soibigbcp2_dn9 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn9)), ((locals.var_pparam_b4soibigbcp2_dn10 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn10)), ((locals.var_pparam_b4soibigbcp2_dn11 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn11)), ((locals.var_pparam_b4soibigbcp2_dn12 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn12)),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign27520_e21188;
        locals.var_t4__blk1148_dn3 = assign27520_e21188_d_n3;
        locals.var_t4__blk1148_dn4 = assign27520_e21188_d_n4;
        locals.var_t4__blk1148_dn5 = assign27520_e21188_d_n5;
        locals.var_t4__blk1148_dn6 = assign27520_e21188_d_n6;
        locals.var_t4__blk1148_dn7 = assign27520_e21188_d_n7;
        locals.var_t4__blk1148_dn8 = assign27520_e21188_d_n8;
        locals.var_t4__blk1148_dn9 = assign27520_e21188_d_n9;
        locals.var_t4__blk1148_dn10 = assign27520_e21188_d_n10;
        locals.var_t4__blk1148_dn11 = assign27520_e21188_d_n11;
        locals.var_t4__blk1148_dn12 = assign27520_e21188_d_n12;

        let (assign27530_e21207, assign27530_e21207_d_n3, assign27530_e21207_d_n4, assign27530_e21207_d_n5, assign27530_e21207_d_n6, assign27530_e21207_d_n7, assign27530_e21207_d_n8, assign27530_e21207_d_n9, assign27530_e21207_d_n10, assign27530_e21207_d_n11, assign27530_e21207_d_n12,) = {
    if (locals.var_guard1658 != 0.0) {
        let assign27530_e21191: f64 = (-locals.var_t12);
        let assign27530_e21193: f64 = (assign27530_e21191 * locals.var_b4soitoxqm);
        let assign27530_e21197: f64 = (locals.var_t3__blk1147 * locals.var_vgp_eff);
        let assign27530_e21198: f64 = (locals.var_pparam_b4soiaigbcp2 + assign27530_e21197);
        let assign27530_e21201: f64 = (locals.var_t4__blk1148 * locals.var_vgp_eff);
        let assign27530_e21203: f64 = (assign27530_e21201 * locals.var_vgp_eff);
        let assign27530_e21204: f64 = (assign27530_e21198 - assign27530_e21203);
        let assign27530_e21205: f64 = (assign27530_e21193 * assign27530_e21204);
        (assign27530_e21205, ((((-locals.var_t12_dn3) * locals.var_b4soitoxqm) * assign27530_e21204) + (assign27530_e21193 * ((locals.var_pparam_b4soiaigbcp2_dn3 + ((locals.var_t3__blk1147_dn3 * locals.var_vgp_eff) + (locals.var_t3__blk1147 * locals.var_vgp_eff_dn3))) - ((((locals.var_t4__blk1148_dn3 * locals.var_vgp_eff) + (locals.var_t4__blk1148 * locals.var_vgp_eff_dn3)) * locals.var_vgp_eff) + (assign27530_e21201 * locals.var_vgp_eff_dn3))))), ((((-locals.var_t12_dn4) * locals.var_b4soitoxqm) * assign27530_e21204) + (assign27530_e21193 * ((locals.var_pparam_b4soiaigbcp2_dn4 + ((locals.var_t3__blk1147_dn4 * locals.var_vgp_eff) + (locals.var_t3__blk1147 * locals.var_vgp_eff_dn4))) - ((((locals.var_t4__blk1148_dn4 * locals.var_vgp_eff) + (locals.var_t4__blk1148 * locals.var_vgp_eff_dn4)) * locals.var_vgp_eff) + (assign27530_e21201 * locals.var_vgp_eff_dn4))))), ((((-locals.var_t12_dn5) * locals.var_b4soitoxqm) * assign27530_e21204) + (assign27530_e21193 * ((locals.var_pparam_b4soiaigbcp2_dn5 + ((locals.var_t3__blk1147_dn5 * locals.var_vgp_eff) + (locals.var_t3__blk1147 * locals.var_vgp_eff_dn5))) - ((((locals.var_t4__blk1148_dn5 * locals.var_vgp_eff) + (locals.var_t4__blk1148 * locals.var_vgp_eff_dn5)) * locals.var_vgp_eff) + (assign27530_e21201 * locals.var_vgp_eff_dn5))))), ((((-locals.var_t12_dn6) * locals.var_b4soitoxqm) * assign27530_e21204) + (assign27530_e21193 * ((locals.var_pparam_b4soiaigbcp2_dn6 + ((locals.var_t3__blk1147_dn6 * locals.var_vgp_eff) + (locals.var_t3__blk1147 * locals.var_vgp_eff_dn6))) - ((((locals.var_t4__blk1148_dn6 * locals.var_vgp_eff) + (locals.var_t4__blk1148 * locals.var_vgp_eff_dn6)) * locals.var_vgp_eff) + (assign27530_e21201 * locals.var_vgp_eff_dn6))))), ((((-locals.var_t12_dn7) * locals.var_b4soitoxqm) * assign27530_e21204) + (assign27530_e21193 * ((locals.var_pparam_b4soiaigbcp2_dn7 + ((locals.var_t3__blk1147_dn7 * locals.var_vgp_eff) + (locals.var_t3__blk1147 * locals.var_vgp_eff_dn7))) - ((((locals.var_t4__blk1148_dn7 * locals.var_vgp_eff) + (locals.var_t4__blk1148 * locals.var_vgp_eff_dn7)) * locals.var_vgp_eff) + (assign27530_e21201 * locals.var_vgp_eff_dn7))))), ((((-locals.var_t12_dn8) * locals.var_b4soitoxqm) * assign27530_e21204) + (assign27530_e21193 * ((locals.var_pparam_b4soiaigbcp2_dn8 + ((locals.var_t3__blk1147_dn8 * locals.var_vgp_eff) + (locals.var_t3__blk1147 * locals.var_vgp_eff_dn8))) - ((((locals.var_t4__blk1148_dn8 * locals.var_vgp_eff) + (locals.var_t4__blk1148 * locals.var_vgp_eff_dn8)) * locals.var_vgp_eff) + (assign27530_e21201 * locals.var_vgp_eff_dn8))))), ((((-locals.var_t12_dn9) * locals.var_b4soitoxqm) * assign27530_e21204) + (assign27530_e21193 * ((locals.var_pparam_b4soiaigbcp2_dn9 + ((locals.var_t3__blk1147_dn9 * locals.var_vgp_eff) + (locals.var_t3__blk1147 * locals.var_vgp_eff_dn9))) - ((((locals.var_t4__blk1148_dn9 * locals.var_vgp_eff) + (locals.var_t4__blk1148 * locals.var_vgp_eff_dn9)) * locals.var_vgp_eff) + (assign27530_e21201 * locals.var_vgp_eff_dn9))))), ((((-locals.var_t12_dn10) * locals.var_b4soitoxqm) * assign27530_e21204) + (assign27530_e21193 * ((locals.var_pparam_b4soiaigbcp2_dn10 + ((locals.var_t3__blk1147_dn10 * locals.var_vgp_eff) + (locals.var_t3__blk1147 * locals.var_vgp_eff_dn10))) - ((((locals.var_t4__blk1148_dn10 * locals.var_vgp_eff) + (locals.var_t4__blk1148 * locals.var_vgp_eff_dn10)) * locals.var_vgp_eff) + (assign27530_e21201 * locals.var_vgp_eff_dn10))))), ((((-locals.var_t12_dn11) * locals.var_b4soitoxqm) * assign27530_e21204) + (assign27530_e21193 * ((locals.var_pparam_b4soiaigbcp2_dn11 + ((locals.var_t3__blk1147_dn11 * locals.var_vgp_eff) + (locals.var_t3__blk1147 * locals.var_vgp_eff_dn11))) - ((((locals.var_t4__blk1148_dn11 * locals.var_vgp_eff) + (locals.var_t4__blk1148 * locals.var_vgp_eff_dn11)) * locals.var_vgp_eff) + (assign27530_e21201 * locals.var_vgp_eff_dn11))))), ((((-locals.var_t12_dn12) * locals.var_b4soitoxqm) * assign27530_e21204) + (assign27530_e21193 * ((locals.var_pparam_b4soiaigbcp2_dn12 + ((locals.var_t3__blk1147_dn12 * locals.var_vgp_eff) + (locals.var_t3__blk1147 * locals.var_vgp_eff_dn12))) - ((((locals.var_t4__blk1148_dn12 * locals.var_vgp_eff) + (locals.var_t4__blk1148 * locals.var_vgp_eff_dn12)) * locals.var_vgp_eff) + (assign27530_e21201 * locals.var_vgp_eff_dn12))))),)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign27530_e21207;
        locals.var_t5__blk1149_dn3 = assign27530_e21207_d_n3;
        locals.var_t5__blk1149_dn4 = assign27530_e21207_d_n4;
        locals.var_t5__blk1149_dn5 = assign27530_e21207_d_n5;
        locals.var_t5__blk1149_dn6 = assign27530_e21207_d_n6;
        locals.var_t5__blk1149_dn7 = assign27530_e21207_d_n7;
        locals.var_t5__blk1149_dn8 = assign27530_e21207_d_n8;
        locals.var_t5__blk1149_dn9 = assign27530_e21207_d_n9;
        locals.var_t5__blk1149_dn10 = assign27530_e21207_d_n10;
        locals.var_t5__blk1149_dn11 = assign27530_e21207_d_n11;
        locals.var_t5__blk1149_dn12 = assign27530_e21207_d_n12;

        let assign27540_e21210: f64 = if locals.var_t5__blk1149 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1659 = assign27540_e21210;

        let (assign27550_e21216, assign27550_e21216_d_n3, assign27550_e21216_d_n4, assign27550_e21216_d_n5, assign27550_e21216_d_n6, assign27550_e21216_d_n7, assign27550_e21216_d_n8, assign27550_e21216_d_n9, assign27550_e21216_d_n10, assign27550_e21216_d_n11, assign27550_e21216_d_n12,) = {
    if ((locals.var_guard1658 != 0.0) && (locals.var_guard1659 != 0.0)) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign27550_e21216;
        locals.var_t6__blk1150_dn3 = assign27550_e21216_d_n3;
        locals.var_t6__blk1150_dn4 = assign27550_e21216_d_n4;
        locals.var_t6__blk1150_dn5 = assign27550_e21216_d_n5;
        locals.var_t6__blk1150_dn6 = assign27550_e21216_d_n6;
        locals.var_t6__blk1150_dn7 = assign27550_e21216_d_n7;
        locals.var_t6__blk1150_dn8 = assign27550_e21216_d_n8;
        locals.var_t6__blk1150_dn9 = assign27550_e21216_d_n9;
        locals.var_t6__blk1150_dn10 = assign27550_e21216_d_n10;
        locals.var_t6__blk1150_dn11 = assign27550_e21216_d_n11;
        locals.var_t6__blk1150_dn12 = assign27550_e21216_d_n12;

        let assign27560_e21219: f64 = (-100.0);
        let assign27560_e21220: f64 = if locals.var_t5__blk1149 < assign27560_e21219 { 1.0 } else { 0.0 };
        locals.var_guard1660 = assign27560_e21220;

        let (assign27570_e21229, assign27570_e21229_d_n3, assign27570_e21229_d_n4, assign27570_e21229_d_n5, assign27570_e21229_d_n6, assign27570_e21229_d_n7, assign27570_e21229_d_n8, assign27570_e21229_d_n9, assign27570_e21229_d_n10, assign27570_e21229_d_n11, assign27570_e21229_d_n12,) = {
    if (((locals.var_guard1658 != 0.0) && (locals.var_guard1659 == 0.0)) && (locals.var_guard1660 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign27570_e21229;
        locals.var_t6__blk1150_dn3 = assign27570_e21229_d_n3;
        locals.var_t6__blk1150_dn4 = assign27570_e21229_d_n4;
        locals.var_t6__blk1150_dn5 = assign27570_e21229_d_n5;
        locals.var_t6__blk1150_dn6 = assign27570_e21229_d_n6;
        locals.var_t6__blk1150_dn7 = assign27570_e21229_d_n7;
        locals.var_t6__blk1150_dn8 = assign27570_e21229_d_n8;
        locals.var_t6__blk1150_dn9 = assign27570_e21229_d_n9;
        locals.var_t6__blk1150_dn10 = assign27570_e21229_d_n10;
        locals.var_t6__blk1150_dn11 = assign27570_e21229_d_n11;
        locals.var_t6__blk1150_dn12 = assign27570_e21229_d_n12;

        let (assign27580_e21240, assign27580_e21240_d_n3, assign27580_e21240_d_n4, assign27580_e21240_d_n5, assign27580_e21240_d_n6, assign27580_e21240_d_n7, assign27580_e21240_d_n8, assign27580_e21240_d_n9, assign27580_e21240_d_n10, assign27580_e21240_d_n11, assign27580_e21240_d_n12,) = {
    if (((locals.var_guard1658 != 0.0) && (locals.var_guard1659 == 0.0)) && (locals.var_guard1660 == 0.0)) {
        let assign27580_e21238: f64 = (locals.var_t5__blk1149).exp();
        (assign27580_e21238, (assign27580_e21238 * locals.var_t5__blk1149_dn3), (assign27580_e21238 * locals.var_t5__blk1149_dn4), (assign27580_e21238 * locals.var_t5__blk1149_dn5), (assign27580_e21238 * locals.var_t5__blk1149_dn6), (assign27580_e21238 * locals.var_t5__blk1149_dn7), (assign27580_e21238 * locals.var_t5__blk1149_dn8), (assign27580_e21238 * locals.var_t5__blk1149_dn9), (assign27580_e21238 * locals.var_t5__blk1149_dn10), (assign27580_e21238 * locals.var_t5__blk1149_dn11), (assign27580_e21238 * locals.var_t5__blk1149_dn12),)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign27580_e21240;
        locals.var_t6__blk1150_dn3 = assign27580_e21240_d_n3;
        locals.var_t6__blk1150_dn4 = assign27580_e21240_d_n4;
        locals.var_t6__blk1150_dn5 = assign27580_e21240_d_n5;
        locals.var_t6__blk1150_dn6 = assign27580_e21240_d_n6;
        locals.var_t6__blk1150_dn7 = assign27580_e21240_d_n7;
        locals.var_t6__blk1150_dn8 = assign27580_e21240_d_n8;
        locals.var_t6__blk1150_dn9 = assign27580_e21240_d_n9;
        locals.var_t6__blk1150_dn10 = assign27580_e21240_d_n10;
        locals.var_t6__blk1150_dn11 = assign27580_e21240_d_n11;
        locals.var_t6__blk1150_dn12 = assign27580_e21240_d_n12;

        let (assign27590_e21248, assign27590_e21248_d_n3, assign27590_e21248_d_n4, assign27590_e21248_d_n5, assign27590_e21248_d_n6, assign27590_e21248_d_n7, assign27590_e21248_d_n8, assign27590_e21248_d_n9, assign27590_e21248_d_n10, assign27590_e21248_d_n11, assign27590_e21248_d_n12,) = {
    if (locals.var_guard1658 != 0.0) {
        let assign27590_e21244: f64 = (locals.var_t11 * locals.var_b4soiagbcp2);
        let assign27590_e21246: f64 = (assign27590_e21244 * locals.var_pparam_b4soioxideratio);
        (assign27590_e21246, ((locals.var_t11_dn3 * locals.var_b4soiagbcp2) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn4 * locals.var_b4soiagbcp2) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn5 * locals.var_b4soiagbcp2) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn6 * locals.var_b4soiagbcp2) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn7 * locals.var_b4soiagbcp2) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn8 * locals.var_b4soiagbcp2) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn9 * locals.var_b4soiagbcp2) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn10 * locals.var_b4soiagbcp2) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn11 * locals.var_b4soiagbcp2) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn12 * locals.var_b4soiagbcp2) * locals.var_pparam_b4soioxideratio),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign27590_e21248;
        locals.var_t11_dn3 = assign27590_e21248_d_n3;
        locals.var_t11_dn4 = assign27590_e21248_d_n4;
        locals.var_t11_dn5 = assign27590_e21248_d_n5;
        locals.var_t11_dn6 = assign27590_e21248_d_n6;
        locals.var_t11_dn7 = assign27590_e21248_d_n7;
        locals.var_t11_dn8 = assign27590_e21248_d_n8;
        locals.var_t11_dn9 = assign27590_e21248_d_n9;
        locals.var_t11_dn10 = assign27590_e21248_d_n10;
        locals.var_t11_dn11 = assign27590_e21248_d_n11;
        locals.var_t11_dn12 = assign27590_e21248_d_n12;

        let (assign27600_e21256, assign27600_e21256_d_n3, assign27600_e21256_d_n4, assign27600_e21256_d_n5, assign27600_e21256_d_n6, assign27600_e21256_d_n7, assign27600_e21256_d_n8, assign27600_e21256_d_n9, assign27600_e21256_d_n10, assign27600_e21256_d_n11, assign27600_e21256_d_n12,) = {
    if (locals.var_guard1658 != 0.0) {
        let assign27600_e21252: f64 = (locals.var_t11 * locals.var_t2__blk1146);
        let assign27600_e21254: f64 = (assign27600_e21252 * locals.var_t6__blk1150);
        (assign27600_e21254, ((((locals.var_t11_dn3 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn3)) * locals.var_t6__blk1150) + (assign27600_e21252 * locals.var_t6__blk1150_dn3)), ((((locals.var_t11_dn4 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn4)) * locals.var_t6__blk1150) + (assign27600_e21252 * locals.var_t6__blk1150_dn4)), ((((locals.var_t11_dn5 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn5)) * locals.var_t6__blk1150) + (assign27600_e21252 * locals.var_t6__blk1150_dn5)), ((((locals.var_t11_dn6 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn6)) * locals.var_t6__blk1150) + (assign27600_e21252 * locals.var_t6__blk1150_dn6)), ((((locals.var_t11_dn7 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn7)) * locals.var_t6__blk1150) + (assign27600_e21252 * locals.var_t6__blk1150_dn7)), ((((locals.var_t11_dn8 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn8)) * locals.var_t6__blk1150) + (assign27600_e21252 * locals.var_t6__blk1150_dn8)), ((((locals.var_t11_dn9 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn9)) * locals.var_t6__blk1150) + (assign27600_e21252 * locals.var_t6__blk1150_dn9)), ((((locals.var_t11_dn10 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn10)) * locals.var_t6__blk1150) + (assign27600_e21252 * locals.var_t6__blk1150_dn10)), ((((locals.var_t11_dn11 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn11)) * locals.var_t6__blk1150) + (assign27600_e21252 * locals.var_t6__blk1150_dn11)), ((((locals.var_t11_dn12 * locals.var_t2__blk1146) + (locals.var_t11 * locals.var_t2__blk1146_dn12)) * locals.var_t6__blk1150) + (assign27600_e21252 * locals.var_t6__blk1150_dn12)),)
    } else {
        (locals.var_ig_agbcp2, locals.var_ig_agbcp2_dn3, locals.var_ig_agbcp2_dn4, locals.var_ig_agbcp2_dn5, locals.var_ig_agbcp2_dn6, locals.var_ig_agbcp2_dn7, locals.var_ig_agbcp2_dn8, locals.var_ig_agbcp2_dn9, locals.var_ig_agbcp2_dn10, locals.var_ig_agbcp2_dn11, locals.var_ig_agbcp2_dn12,)
    }
};
        locals.var_ig_agbcp2 = assign27600_e21256;
        locals.var_ig_agbcp2_dn3 = assign27600_e21256_d_n3;
        locals.var_ig_agbcp2_dn4 = assign27600_e21256_d_n4;
        locals.var_ig_agbcp2_dn5 = assign27600_e21256_d_n5;
        locals.var_ig_agbcp2_dn6 = assign27600_e21256_d_n6;
        locals.var_ig_agbcp2_dn7 = assign27600_e21256_d_n7;
        locals.var_ig_agbcp2_dn8 = assign27600_e21256_d_n8;
        locals.var_ig_agbcp2_dn9 = assign27600_e21256_d_n9;
        locals.var_ig_agbcp2_dn10 = assign27600_e21256_d_n10;
        locals.var_ig_agbcp2_dn11 = assign27600_e21256_d_n11;
        locals.var_ig_agbcp2_dn12 = assign27600_e21256_d_n12;

        let (assign27610_e21261, assign27610_e21261_d_n3, assign27610_e21261_d_n4, assign27610_e21261_d_n5, assign27610_e21261_d_n6, assign27610_e21261_d_n7, assign27610_e21261_d_n8, assign27610_e21261_d_n9, assign27610_e21261_d_n10, assign27610_e21261_d_n11, assign27610_e21261_d_n12,) = {
    if (locals.var_guard1658 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ig_agbcp2, locals.var_ig_agbcp2_dn3, locals.var_ig_agbcp2_dn4, locals.var_ig_agbcp2_dn5, locals.var_ig_agbcp2_dn6, locals.var_ig_agbcp2_dn7, locals.var_ig_agbcp2_dn8, locals.var_ig_agbcp2_dn9, locals.var_ig_agbcp2_dn10, locals.var_ig_agbcp2_dn11, locals.var_ig_agbcp2_dn12,)
    }
};
        locals.var_ig_agbcp2 = assign27610_e21261;
        locals.var_ig_agbcp2_dn3 = assign27610_e21261_d_n3;
        locals.var_ig_agbcp2_dn4 = assign27610_e21261_d_n4;
        locals.var_ig_agbcp2_dn5 = assign27610_e21261_d_n5;
        locals.var_ig_agbcp2_dn6 = assign27610_e21261_d_n6;
        locals.var_ig_agbcp2_dn7 = assign27610_e21261_d_n7;
        locals.var_ig_agbcp2_dn8 = assign27610_e21261_d_n8;
        locals.var_ig_agbcp2_dn9 = assign27610_e21261_d_n9;
        locals.var_ig_agbcp2_dn10 = assign27610_e21261_d_n10;
        locals.var_ig_agbcp2_dn11 = assign27610_e21261_d_n11;
        locals.var_ig_agbcp2_dn12 = assign27610_e21261_d_n12;

        let assign27620_e21264: f64 = (locals.var_b4soitype * locals.var_ig_agbcp2);
        locals.var_b4soiigp = assign27620_e21264;
        locals.var_b4soiigp_dn3 = (locals.var_b4soitype * locals.var_ig_agbcp2_dn3);
        locals.var_b4soiigp_dn4 = (locals.var_b4soitype * locals.var_ig_agbcp2_dn4);
        locals.var_b4soiigp_dn5 = (locals.var_b4soitype * locals.var_ig_agbcp2_dn5);
        locals.var_b4soiigp_dn6 = (locals.var_b4soitype * locals.var_ig_agbcp2_dn6);
        locals.var_b4soiigp_dn7 = (locals.var_b4soitype * locals.var_ig_agbcp2_dn7);
        locals.var_b4soiigp_dn8 = (locals.var_b4soitype * locals.var_ig_agbcp2_dn8);
        locals.var_b4soiigp_dn9 = (locals.var_b4soitype * locals.var_ig_agbcp2_dn9);
        locals.var_b4soiigp_dn10 = (locals.var_b4soitype * locals.var_ig_agbcp2_dn10);
        locals.var_b4soiigp_dn11 = (locals.var_b4soitype * locals.var_ig_agbcp2_dn11);
        locals.var_b4soiigp_dn12 = (locals.var_b4soitype * locals.var_ig_agbcp2_dn12);

        let assign27630_e21267: f64 = if locals.var_b4soisoimod != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1661 = assign27630_e21267;

        let assign27640_e21270: f64 = if locals.var_b4soiiiimod == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1662 = assign27640_e21270;

        let assign27650_e21273: f64 = if locals.var_pparam_b4soialpha0 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1663 = assign27650_e21273;

        let (assign27660_e21281, assign27660_e21281_d_n3, assign27660_e21281_d_n4, assign27660_e21281_d_n5, assign27660_e21281_d_n6, assign27660_e21281_d_n7, assign27660_e21281_d_n8, assign27660_e21281_d_n9, assign27660_e21281_d_n10, assign27660_e21281_d_n11, assign27660_e21281_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11, locals.var_iii_dn12,)
    }
};
        locals.var_iii = assign27660_e21281;
        locals.var_iii_dn3 = assign27660_e21281_d_n3;
        locals.var_iii_dn4 = assign27660_e21281_d_n4;
        locals.var_iii_dn5 = assign27660_e21281_d_n5;
        locals.var_iii_dn6 = assign27660_e21281_d_n6;
        locals.var_iii_dn7 = assign27660_e21281_d_n7;
        locals.var_iii_dn8 = assign27660_e21281_d_n8;
        locals.var_iii_dn9 = assign27660_e21281_d_n9;
        locals.var_iii_dn10 = assign27660_e21281_d_n10;
        locals.var_iii_dn11 = assign27660_e21281_d_n11;
        locals.var_iii_dn12 = assign27660_e21281_d_n12;

        let (assign27670_e21302, assign27670_e21302_d_n3, assign27670_e21302_d_n4, assign27670_e21302_d_n5, assign27670_e21302_d_n6, assign27670_e21302_d_n7, assign27670_e21302_d_n8, assign27670_e21302_d_n9, assign27670_e21302_d_n10, assign27670_e21302_d_n11, assign27670_e21302_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27670_e21293: f64 = (locals.var_tempratio - 1.0);
        let assign27670_e21294: f64 = (locals.var_b4soitii * assign27670_e21293);
        let assign27670_e21295: f64 = (1.0 + assign27670_e21294);
        let assign27670_e21296: f64 = (locals.var_pparam_b4soivdsatii0 * assign27670_e21295);
        let assign27670_e21299: f64 = (locals.var_pparam_b4soilii / locals.var_leff);
        let assign27670_e21300: f64 = (assign27670_e21296 - assign27670_e21299);
        (assign27670_e21300, ((locals.var_pparam_b4soivdsatii0_dn3 * assign27670_e21295) - (((locals.var_pparam_b4soilii_dn3 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn4 * assign27670_e21295) - (((locals.var_pparam_b4soilii_dn4 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn5 * assign27670_e21295) - (((locals.var_pparam_b4soilii_dn5 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff))), (((locals.var_pparam_b4soivdsatii0_dn6 * assign27670_e21295) + (locals.var_pparam_b4soivdsatii0 * (locals.var_b4soitii * locals.var_tempratio_dn6))) - (((locals.var_pparam_b4soilii_dn6 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn7 * assign27670_e21295) - (((locals.var_pparam_b4soilii_dn7 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn8 * assign27670_e21295) - (((locals.var_pparam_b4soilii_dn8 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn9 * assign27670_e21295) - (((locals.var_pparam_b4soilii_dn9 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn10 * assign27670_e21295) - (((locals.var_pparam_b4soilii_dn10 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn11 * assign27670_e21295) - (((locals.var_pparam_b4soilii_dn11 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn12 * assign27670_e21295) - (((locals.var_pparam_b4soilii_dn12 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff))),)
    } else {
        (locals.var_vdsatii0, locals.var_vdsatii0_dn3, locals.var_vdsatii0_dn4, locals.var_vdsatii0_dn5, locals.var_vdsatii0_dn6, locals.var_vdsatii0_dn7, locals.var_vdsatii0_dn8, locals.var_vdsatii0_dn9, locals.var_vdsatii0_dn10, locals.var_vdsatii0_dn11, locals.var_vdsatii0_dn12,)
    }
};
        locals.var_vdsatii0 = assign27670_e21302;
        locals.var_vdsatii0_dn3 = assign27670_e21302_d_n3;
        locals.var_vdsatii0_dn4 = assign27670_e21302_d_n4;
        locals.var_vdsatii0_dn5 = assign27670_e21302_d_n5;
        locals.var_vdsatii0_dn6 = assign27670_e21302_d_n6;
        locals.var_vdsatii0_dn7 = assign27670_e21302_d_n7;
        locals.var_vdsatii0_dn8 = assign27670_e21302_d_n8;
        locals.var_vdsatii0_dn9 = assign27670_e21302_d_n9;
        locals.var_vdsatii0_dn10 = assign27670_e21302_d_n10;
        locals.var_vdsatii0_dn11 = assign27670_e21302_d_n11;
        locals.var_vdsatii0_dn12 = assign27670_e21302_d_n12;

        let (assign27680_e21313, assign27680_e21313_d_n3, assign27680_e21313_d_n4, assign27680_e21313_d_n5, assign27680_e21313_d_n6, assign27680_e21313_d_n7, assign27680_e21313_d_n8, assign27680_e21313_d_n9, assign27680_e21313_d_n10, assign27680_e21313_d_n11, assign27680_e21313_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27680_e21311: f64 = (locals.var_pparam_b4soiesatii * locals.var_leff);
        (assign27680_e21311, ((locals.var_pparam_b4soiesatii_dn3 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn3)), ((locals.var_pparam_b4soiesatii_dn4 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn4)), ((locals.var_pparam_b4soiesatii_dn5 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn5)), ((locals.var_pparam_b4soiesatii_dn6 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn6)), ((locals.var_pparam_b4soiesatii_dn7 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn7)), ((locals.var_pparam_b4soiesatii_dn8 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn8)), ((locals.var_pparam_b4soiesatii_dn9 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn9)), ((locals.var_pparam_b4soiesatii_dn10 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn10)), ((locals.var_pparam_b4soiesatii_dn11 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn11)), ((locals.var_pparam_b4soiesatii_dn12 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27680_e21313;
        locals.var_t0__blk1144_dn3 = assign27680_e21313_d_n3;
        locals.var_t0__blk1144_dn4 = assign27680_e21313_d_n4;
        locals.var_t0__blk1144_dn5 = assign27680_e21313_d_n5;
        locals.var_t0__blk1144_dn6 = assign27680_e21313_d_n6;
        locals.var_t0__blk1144_dn7 = assign27680_e21313_d_n7;
        locals.var_t0__blk1144_dn8 = assign27680_e21313_d_n8;
        locals.var_t0__blk1144_dn9 = assign27680_e21313_d_n9;
        locals.var_t0__blk1144_dn10 = assign27680_e21313_d_n10;
        locals.var_t0__blk1144_dn11 = assign27680_e21313_d_n11;
        locals.var_t0__blk1144_dn12 = assign27680_e21313_d_n12;

        let (assign27690_e21328, assign27690_e21328_d_n3, assign27690_e21328_d_n4, assign27690_e21328_d_n5, assign27690_e21328_d_n6, assign27690_e21328_d_n7, assign27690_e21328_d_n8, assign27690_e21328_d_n9, assign27690_e21328_d_n10, assign27690_e21328_d_n11, assign27690_e21328_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27690_e21322: f64 = (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144);
        let assign27690_e21325: f64 = (1.0 + locals.var_t0__blk1144);
        let assign27690_e21326: f64 = (assign27690_e21322 / assign27690_e21325);
        (assign27690_e21326, (((((locals.var_pparam_b4soisii0_dn3 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn3)) * assign27690_e21325) - (assign27690_e21322 * locals.var_t0__blk1144_dn3)) / (assign27690_e21325 * assign27690_e21325)), (((((locals.var_pparam_b4soisii0_dn4 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn4)) * assign27690_e21325) - (assign27690_e21322 * locals.var_t0__blk1144_dn4)) / (assign27690_e21325 * assign27690_e21325)), (((((locals.var_pparam_b4soisii0_dn5 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn5)) * assign27690_e21325) - (assign27690_e21322 * locals.var_t0__blk1144_dn5)) / (assign27690_e21325 * assign27690_e21325)), (((((locals.var_pparam_b4soisii0_dn6 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn6)) * assign27690_e21325) - (assign27690_e21322 * locals.var_t0__blk1144_dn6)) / (assign27690_e21325 * assign27690_e21325)), (((((locals.var_pparam_b4soisii0_dn7 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn7)) * assign27690_e21325) - (assign27690_e21322 * locals.var_t0__blk1144_dn7)) / (assign27690_e21325 * assign27690_e21325)), (((((locals.var_pparam_b4soisii0_dn8 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn8)) * assign27690_e21325) - (assign27690_e21322 * locals.var_t0__blk1144_dn8)) / (assign27690_e21325 * assign27690_e21325)), (((((locals.var_pparam_b4soisii0_dn9 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn9)) * assign27690_e21325) - (assign27690_e21322 * locals.var_t0__blk1144_dn9)) / (assign27690_e21325 * assign27690_e21325)), (((((locals.var_pparam_b4soisii0_dn10 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn10)) * assign27690_e21325) - (assign27690_e21322 * locals.var_t0__blk1144_dn10)) / (assign27690_e21325 * assign27690_e21325)), (((((locals.var_pparam_b4soisii0_dn11 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn11)) * assign27690_e21325) - (assign27690_e21322 * locals.var_t0__blk1144_dn11)) / (assign27690_e21325 * assign27690_e21325)), (((((locals.var_pparam_b4soisii0_dn12 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn12)) * assign27690_e21325) - (assign27690_e21322 * locals.var_t0__blk1144_dn12)) / (assign27690_e21325 * assign27690_e21325)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign27690_e21328;
        locals.var_t1__blk1145_dn3 = assign27690_e21328_d_n3;
        locals.var_t1__blk1145_dn4 = assign27690_e21328_d_n4;
        locals.var_t1__blk1145_dn5 = assign27690_e21328_d_n5;
        locals.var_t1__blk1145_dn6 = assign27690_e21328_d_n6;
        locals.var_t1__blk1145_dn7 = assign27690_e21328_d_n7;
        locals.var_t1__blk1145_dn8 = assign27690_e21328_d_n8;
        locals.var_t1__blk1145_dn9 = assign27690_e21328_d_n9;
        locals.var_t1__blk1145_dn10 = assign27690_e21328_d_n10;
        locals.var_t1__blk1145_dn11 = assign27690_e21328_d_n11;
        locals.var_t1__blk1145_dn12 = assign27690_e21328_d_n12;

        let (assign27700_e21343, assign27700_e21343_d_n3, assign27700_e21343_d_n4, assign27700_e21343_d_n5, assign27700_e21343_d_n6, assign27700_e21343_d_n7, assign27700_e21343_d_n8, assign27700_e21343_d_n9, assign27700_e21343_d_n10, assign27700_e21343_d_n11, assign27700_e21343_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27700_e21339: f64 = (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175);
        let assign27700_e21340: f64 = (1.0 + assign27700_e21339);
        let assign27700_e21341: f64 = (1.0 / assign27700_e21340);
        (assign27700_e21341, (-(((locals.var_pparam_b4soisii1_dn3 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn3)) / (assign27700_e21340 * assign27700_e21340))), (-(((locals.var_pparam_b4soisii1_dn4 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn4)) / (assign27700_e21340 * assign27700_e21340))), (-(((locals.var_pparam_b4soisii1_dn5 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn5)) / (assign27700_e21340 * assign27700_e21340))), (-(((locals.var_pparam_b4soisii1_dn6 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn6)) / (assign27700_e21340 * assign27700_e21340))), (-(((locals.var_pparam_b4soisii1_dn7 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn7)) / (assign27700_e21340 * assign27700_e21340))), (-(((locals.var_pparam_b4soisii1_dn8 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn8)) / (assign27700_e21340 * assign27700_e21340))), (-(((locals.var_pparam_b4soisii1_dn9 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn9)) / (assign27700_e21340 * assign27700_e21340))), (-(((locals.var_pparam_b4soisii1_dn10 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn10)) / (assign27700_e21340 * assign27700_e21340))), (-(((locals.var_pparam_b4soisii1_dn11 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn11)) / (assign27700_e21340 * assign27700_e21340))), (-(((locals.var_pparam_b4soisii1_dn12 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn12)) / (assign27700_e21340 * assign27700_e21340))),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27700_e21343;
        locals.var_t0__blk1144_dn3 = assign27700_e21343_d_n3;
        locals.var_t0__blk1144_dn4 = assign27700_e21343_d_n4;
        locals.var_t0__blk1144_dn5 = assign27700_e21343_d_n5;
        locals.var_t0__blk1144_dn6 = assign27700_e21343_d_n6;
        locals.var_t0__blk1144_dn7 = assign27700_e21343_d_n7;
        locals.var_t0__blk1144_dn8 = assign27700_e21343_d_n8;
        locals.var_t0__blk1144_dn9 = assign27700_e21343_d_n9;
        locals.var_t0__blk1144_dn10 = assign27700_e21343_d_n10;
        locals.var_t0__blk1144_dn11 = assign27700_e21343_d_n11;
        locals.var_t0__blk1144_dn12 = assign27700_e21343_d_n12;

        let (assign27710_e21354, assign27710_e21354_d_n3, assign27710_e21354_d_n4, assign27710_e21354_d_n5, assign27710_e21354_d_n6, assign27710_e21354_d_n7, assign27710_e21354_d_n8, assign27710_e21354_d_n9, assign27710_e21354_d_n10, assign27710_e21354_d_n11, assign27710_e21354_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27710_e21352: f64 = (locals.var_t0__blk1144 + locals.var_pparam_b4soisii2);
        (assign27710_e21352, (locals.var_t0__blk1144_dn3 + locals.var_pparam_b4soisii2_dn3), (locals.var_t0__blk1144_dn4 + locals.var_pparam_b4soisii2_dn4), (locals.var_t0__blk1144_dn5 + locals.var_pparam_b4soisii2_dn5), (locals.var_t0__blk1144_dn6 + locals.var_pparam_b4soisii2_dn6), (locals.var_t0__blk1144_dn7 + locals.var_pparam_b4soisii2_dn7), (locals.var_t0__blk1144_dn8 + locals.var_pparam_b4soisii2_dn8), (locals.var_t0__blk1144_dn9 + locals.var_pparam_b4soisii2_dn9), (locals.var_t0__blk1144_dn10 + locals.var_pparam_b4soisii2_dn10), (locals.var_t0__blk1144_dn11 + locals.var_pparam_b4soisii2_dn11), (locals.var_t0__blk1144_dn12 + locals.var_pparam_b4soisii2_dn12),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign27710_e21354;
        locals.var_t3__blk1147_dn3 = assign27710_e21354_d_n3;
        locals.var_t3__blk1147_dn4 = assign27710_e21354_d_n4;
        locals.var_t3__blk1147_dn5 = assign27710_e21354_d_n5;
        locals.var_t3__blk1147_dn6 = assign27710_e21354_d_n6;
        locals.var_t3__blk1147_dn7 = assign27710_e21354_d_n7;
        locals.var_t3__blk1147_dn8 = assign27710_e21354_d_n8;
        locals.var_t3__blk1147_dn9 = assign27710_e21354_d_n9;
        locals.var_t3__blk1147_dn10 = assign27710_e21354_d_n10;
        locals.var_t3__blk1147_dn11 = assign27710_e21354_d_n11;
        locals.var_t3__blk1147_dn12 = assign27710_e21354_d_n12;

        let (assign27720_e21365, assign27720_e21365_d_n3, assign27720_e21365_d_n4, assign27720_e21365_d_n5, assign27720_e21365_d_n6, assign27720_e21365_d_n7, assign27720_e21365_d_n8, assign27720_e21365_d_n9, assign27720_e21365_d_n10, assign27720_e21365_d_n11, assign27720_e21365_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27720_e21363: f64 = (locals.var_vgst__blk1131 * locals.var_t3__blk1147);
        (assign27720_e21363, ((locals.var_vgst__blk1131_dn3 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn3)), ((locals.var_vgst__blk1131_dn4 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn4)), ((locals.var_vgst__blk1131_dn5 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn5)), ((locals.var_vgst__blk1131_dn6 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn6)), ((locals.var_vgst__blk1131_dn7 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn7)), ((locals.var_vgst__blk1131_dn8 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn8)), ((locals.var_vgst__blk1131_dn9 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn9)), ((locals.var_vgst__blk1131_dn10 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn10)), ((locals.var_vgst__blk1131_dn11 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn11)), ((locals.var_vgst__blk1131_dn12 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn12)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign27720_e21365;
        locals.var_t2__blk1146_dn3 = assign27720_e21365_d_n3;
        locals.var_t2__blk1146_dn4 = assign27720_e21365_d_n4;
        locals.var_t2__blk1146_dn5 = assign27720_e21365_d_n5;
        locals.var_t2__blk1146_dn6 = assign27720_e21365_d_n6;
        locals.var_t2__blk1146_dn7 = assign27720_e21365_d_n7;
        locals.var_t2__blk1146_dn8 = assign27720_e21365_d_n8;
        locals.var_t2__blk1146_dn9 = assign27720_e21365_d_n9;
        locals.var_t2__blk1146_dn10 = assign27720_e21365_d_n10;
        locals.var_t2__blk1146_dn11 = assign27720_e21365_d_n11;
        locals.var_t2__blk1146_dn12 = assign27720_e21365_d_n12;

    }

    pub(super) fn stamp_transient_block_73(
        locals: &mut StampLocals,
    ) {
        let (assign27730_e21380, assign27730_e21380_d_n3, assign27730_e21380_d_n4, assign27730_e21380_d_n5, assign27730_e21380_d_n6, assign27730_e21380_d_n7, assign27730_e21380_d_n8, assign27730_e21380_d_n9, assign27730_e21380_d_n10, assign27730_e21380_d_n11, assign27730_e21380_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27730_e21376: f64 = (locals.var_pparam_b4soisiid * locals.var_vds_1);
        let assign27730_e21377: f64 = (1.0 + assign27730_e21376);
        let assign27730_e21378: f64 = (1.0 / assign27730_e21377);
        (assign27730_e21378, (-((locals.var_pparam_b4soisiid_dn3 * locals.var_vds_1) / (assign27730_e21377 * assign27730_e21377))), (-((locals.var_pparam_b4soisiid_dn4 * locals.var_vds_1) / (assign27730_e21377 * assign27730_e21377))), (-((locals.var_pparam_b4soisiid_dn5 * locals.var_vds_1) / (assign27730_e21377 * assign27730_e21377))), (-((locals.var_pparam_b4soisiid_dn6 * locals.var_vds_1) / (assign27730_e21377 * assign27730_e21377))), (-(((locals.var_pparam_b4soisiid_dn7 * locals.var_vds_1) + (locals.var_pparam_b4soisiid * locals.var_vds_1_dn7)) / (assign27730_e21377 * assign27730_e21377))), (-(((locals.var_pparam_b4soisiid_dn8 * locals.var_vds_1) + (locals.var_pparam_b4soisiid * locals.var_vds_1_dn8)) / (assign27730_e21377 * assign27730_e21377))), (-((locals.var_pparam_b4soisiid_dn9 * locals.var_vds_1) / (assign27730_e21377 * assign27730_e21377))), (-((locals.var_pparam_b4soisiid_dn10 * locals.var_vds_1) / (assign27730_e21377 * assign27730_e21377))), (-((locals.var_pparam_b4soisiid_dn11 * locals.var_vds_1) / (assign27730_e21377 * assign27730_e21377))), (-((locals.var_pparam_b4soisiid_dn12 * locals.var_vds_1) / (assign27730_e21377 * assign27730_e21377))),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign27730_e21380;
        locals.var_t3__blk1147_dn3 = assign27730_e21380_d_n3;
        locals.var_t3__blk1147_dn4 = assign27730_e21380_d_n4;
        locals.var_t3__blk1147_dn5 = assign27730_e21380_d_n5;
        locals.var_t3__blk1147_dn6 = assign27730_e21380_d_n6;
        locals.var_t3__blk1147_dn7 = assign27730_e21380_d_n7;
        locals.var_t3__blk1147_dn8 = assign27730_e21380_d_n8;
        locals.var_t3__blk1147_dn9 = assign27730_e21380_d_n9;
        locals.var_t3__blk1147_dn10 = assign27730_e21380_d_n10;
        locals.var_t3__blk1147_dn11 = assign27730_e21380_d_n11;
        locals.var_t3__blk1147_dn12 = assign27730_e21380_d_n12;

        let (assign27740_e21393, assign27740_e21393_d_n3, assign27740_e21393_d_n4, assign27740_e21393_d_n5, assign27740_e21393_d_n6, assign27740_e21393_d_n7, assign27740_e21393_d_n8, assign27740_e21393_d_n9, assign27740_e21393_d_n10, assign27740_e21393_d_n11, assign27740_e21393_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27740_e21389: f64 = (locals.var_t1__blk1145 * locals.var_t2__blk1146);
        let assign27740_e21391: f64 = (assign27740_e21389 * locals.var_t3__blk1147);
        (assign27740_e21391, ((((locals.var_t1__blk1145_dn3 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn3)) * locals.var_t3__blk1147) + (assign27740_e21389 * locals.var_t3__blk1147_dn3)), ((((locals.var_t1__blk1145_dn4 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn4)) * locals.var_t3__blk1147) + (assign27740_e21389 * locals.var_t3__blk1147_dn4)), ((((locals.var_t1__blk1145_dn5 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn5)) * locals.var_t3__blk1147) + (assign27740_e21389 * locals.var_t3__blk1147_dn5)), ((((locals.var_t1__blk1145_dn6 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn6)) * locals.var_t3__blk1147) + (assign27740_e21389 * locals.var_t3__blk1147_dn6)), ((((locals.var_t1__blk1145_dn7 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn7)) * locals.var_t3__blk1147) + (assign27740_e21389 * locals.var_t3__blk1147_dn7)), ((((locals.var_t1__blk1145_dn8 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn8)) * locals.var_t3__blk1147) + (assign27740_e21389 * locals.var_t3__blk1147_dn8)), ((((locals.var_t1__blk1145_dn9 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn9)) * locals.var_t3__blk1147) + (assign27740_e21389 * locals.var_t3__blk1147_dn9)), ((((locals.var_t1__blk1145_dn10 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn10)) * locals.var_t3__blk1147) + (assign27740_e21389 * locals.var_t3__blk1147_dn10)), ((((locals.var_t1__blk1145_dn11 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn11)) * locals.var_t3__blk1147) + (assign27740_e21389 * locals.var_t3__blk1147_dn11)), ((((locals.var_t1__blk1145_dn12 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn12)) * locals.var_t3__blk1147) + (assign27740_e21389 * locals.var_t3__blk1147_dn12)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11, locals.var_vgsstep_dn12,)
    }
};
        locals.var_vgsstep = assign27740_e21393;
        locals.var_vgsstep_dn3 = assign27740_e21393_d_n3;
        locals.var_vgsstep_dn4 = assign27740_e21393_d_n4;
        locals.var_vgsstep_dn5 = assign27740_e21393_d_n5;
        locals.var_vgsstep_dn6 = assign27740_e21393_d_n6;
        locals.var_vgsstep_dn7 = assign27740_e21393_d_n7;
        locals.var_vgsstep_dn8 = assign27740_e21393_d_n8;
        locals.var_vgsstep_dn9 = assign27740_e21393_d_n9;
        locals.var_vgsstep_dn10 = assign27740_e21393_d_n10;
        locals.var_vgsstep_dn11 = assign27740_e21393_d_n11;
        locals.var_vgsstep_dn12 = assign27740_e21393_d_n12;

        let (assign27750_e21404, assign27750_e21404_d_n3, assign27750_e21404_d_n4, assign27750_e21404_d_n5, assign27750_e21404_d_n6, assign27750_e21404_d_n7, assign27750_e21404_d_n8, assign27750_e21404_d_n9, assign27750_e21404_d_n10, assign27750_e21404_d_n11, assign27750_e21404_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27750_e21402: f64 = (locals.var_vdsatii0 + locals.var_vgsstep);
        (assign27750_e21402, (locals.var_vdsatii0_dn3 + locals.var_vgsstep_dn3), (locals.var_vdsatii0_dn4 + locals.var_vgsstep_dn4), (locals.var_vdsatii0_dn5 + locals.var_vgsstep_dn5), (locals.var_vdsatii0_dn6 + locals.var_vgsstep_dn6), (locals.var_vdsatii0_dn7 + locals.var_vgsstep_dn7), (locals.var_vdsatii0_dn8 + locals.var_vgsstep_dn8), (locals.var_vdsatii0_dn9 + locals.var_vgsstep_dn9), (locals.var_vdsatii0_dn10 + locals.var_vgsstep_dn10), (locals.var_vdsatii0_dn11 + locals.var_vgsstep_dn11), (locals.var_vdsatii0_dn12 + locals.var_vgsstep_dn12),)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11, locals.var_vdsatii_dn12,)
    }
};
        locals.var_vdsatii = assign27750_e21404;
        locals.var_vdsatii_dn3 = assign27750_e21404_d_n3;
        locals.var_vdsatii_dn4 = assign27750_e21404_d_n4;
        locals.var_vdsatii_dn5 = assign27750_e21404_d_n5;
        locals.var_vdsatii_dn6 = assign27750_e21404_d_n6;
        locals.var_vdsatii_dn7 = assign27750_e21404_d_n7;
        locals.var_vdsatii_dn8 = assign27750_e21404_d_n8;
        locals.var_vdsatii_dn9 = assign27750_e21404_d_n9;
        locals.var_vdsatii_dn10 = assign27750_e21404_d_n10;
        locals.var_vdsatii_dn11 = assign27750_e21404_d_n11;
        locals.var_vdsatii_dn12 = assign27750_e21404_d_n12;

        let (assign27760_e21415, assign27760_e21415_d_n3, assign27760_e21415_d_n4, assign27760_e21415_d_n5, assign27760_e21415_d_n6, assign27760_e21415_d_n7, assign27760_e21415_d_n8, assign27760_e21415_d_n9, assign27760_e21415_d_n10, assign27760_e21415_d_n11, assign27760_e21415_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27760_e21413: f64 = (locals.var_vds_1 - locals.var_vdsatii);
        (assign27760_e21413, (-locals.var_vdsatii_dn3), (-locals.var_vdsatii_dn4), (-locals.var_vdsatii_dn5), (-locals.var_vdsatii_dn6), (locals.var_vds_1_dn7 - locals.var_vdsatii_dn7), (locals.var_vds_1_dn8 - locals.var_vdsatii_dn8), (-locals.var_vdsatii_dn9), (-locals.var_vdsatii_dn10), (-locals.var_vdsatii_dn11), (-locals.var_vdsatii_dn12),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11, locals.var_vdiff_dn12,)
    }
};
        locals.var_vdiff = assign27760_e21415;
        locals.var_vdiff_dn3 = assign27760_e21415_d_n3;
        locals.var_vdiff_dn4 = assign27760_e21415_d_n4;
        locals.var_vdiff_dn5 = assign27760_e21415_d_n5;
        locals.var_vdiff_dn6 = assign27760_e21415_d_n6;
        locals.var_vdiff_dn7 = assign27760_e21415_d_n7;
        locals.var_vdiff_dn8 = assign27760_e21415_d_n8;
        locals.var_vdiff_dn9 = assign27760_e21415_d_n9;
        locals.var_vdiff_dn10 = assign27760_e21415_d_n10;
        locals.var_vdiff_dn11 = assign27760_e21415_d_n11;
        locals.var_vdiff_dn12 = assign27760_e21415_d_n12;

        let (assign27770_e21434, assign27770_e21434_d_n3, assign27770_e21434_d_n4, assign27770_e21434_d_n5, assign27770_e21434_d_n6, assign27770_e21434_d_n7, assign27770_e21434_d_n8, assign27770_e21434_d_n9, assign27770_e21434_d_n10, assign27770_e21434_d_n11, assign27770_e21434_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27770_e21425: f64 = (locals.var_pparam_b4soibeta1 * locals.var_vdiff);
        let assign27770_e21426: f64 = (locals.var_pparam_b4soibeta2 + assign27770_e21425);
        let assign27770_e21429: f64 = (locals.var_pparam_b4soibeta0 * locals.var_vdiff);
        let assign27770_e21431: f64 = (assign27770_e21429 * locals.var_vdiff);
        let assign27770_e21432: f64 = (assign27770_e21426 + assign27770_e21431);
        (assign27770_e21432, ((locals.var_pparam_b4soibeta2_dn3 + ((locals.var_pparam_b4soibeta1_dn3 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn3))) + ((((locals.var_pparam_b4soibeta0_dn3 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn3)) * locals.var_vdiff) + (assign27770_e21429 * locals.var_vdiff_dn3))), ((locals.var_pparam_b4soibeta2_dn4 + ((locals.var_pparam_b4soibeta1_dn4 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn4))) + ((((locals.var_pparam_b4soibeta0_dn4 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn4)) * locals.var_vdiff) + (assign27770_e21429 * locals.var_vdiff_dn4))), ((locals.var_pparam_b4soibeta2_dn5 + ((locals.var_pparam_b4soibeta1_dn5 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn5))) + ((((locals.var_pparam_b4soibeta0_dn5 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn5)) * locals.var_vdiff) + (assign27770_e21429 * locals.var_vdiff_dn5))), ((locals.var_pparam_b4soibeta2_dn6 + ((locals.var_pparam_b4soibeta1_dn6 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn6))) + ((((locals.var_pparam_b4soibeta0_dn6 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn6)) * locals.var_vdiff) + (assign27770_e21429 * locals.var_vdiff_dn6))), ((locals.var_pparam_b4soibeta2_dn7 + ((locals.var_pparam_b4soibeta1_dn7 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn7))) + ((((locals.var_pparam_b4soibeta0_dn7 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn7)) * locals.var_vdiff) + (assign27770_e21429 * locals.var_vdiff_dn7))), ((locals.var_pparam_b4soibeta2_dn8 + ((locals.var_pparam_b4soibeta1_dn8 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn8))) + ((((locals.var_pparam_b4soibeta0_dn8 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn8)) * locals.var_vdiff) + (assign27770_e21429 * locals.var_vdiff_dn8))), ((locals.var_pparam_b4soibeta2_dn9 + ((locals.var_pparam_b4soibeta1_dn9 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn9))) + ((((locals.var_pparam_b4soibeta0_dn9 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn9)) * locals.var_vdiff) + (assign27770_e21429 * locals.var_vdiff_dn9))), ((locals.var_pparam_b4soibeta2_dn10 + ((locals.var_pparam_b4soibeta1_dn10 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn10))) + ((((locals.var_pparam_b4soibeta0_dn10 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn10)) * locals.var_vdiff) + (assign27770_e21429 * locals.var_vdiff_dn10))), ((locals.var_pparam_b4soibeta2_dn11 + ((locals.var_pparam_b4soibeta1_dn11 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn11))) + ((((locals.var_pparam_b4soibeta0_dn11 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn11)) * locals.var_vdiff) + (assign27770_e21429 * locals.var_vdiff_dn11))), ((locals.var_pparam_b4soibeta2_dn12 + ((locals.var_pparam_b4soibeta1_dn12 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn12))) + ((((locals.var_pparam_b4soibeta0_dn12 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn12)) * locals.var_vdiff) + (assign27770_e21429 * locals.var_vdiff_dn12))),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27770_e21434;
        locals.var_t0__blk1144_dn3 = assign27770_e21434_d_n3;
        locals.var_t0__blk1144_dn4 = assign27770_e21434_d_n4;
        locals.var_t0__blk1144_dn5 = assign27770_e21434_d_n5;
        locals.var_t0__blk1144_dn6 = assign27770_e21434_d_n6;
        locals.var_t0__blk1144_dn7 = assign27770_e21434_d_n7;
        locals.var_t0__blk1144_dn8 = assign27770_e21434_d_n8;
        locals.var_t0__blk1144_dn9 = assign27770_e21434_d_n9;
        locals.var_t0__blk1144_dn10 = assign27770_e21434_d_n10;
        locals.var_t0__blk1144_dn11 = assign27770_e21434_d_n11;
        locals.var_t0__blk1144_dn12 = assign27770_e21434_d_n12;

        let assign27780_e21437: f64 = if locals.var_t0__blk1144 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1664 = assign27780_e21437;

        let (assign27790_e21448, assign27790_e21448_d_n3, assign27790_e21448_d_n4, assign27790_e21448_d_n5, assign27790_e21448_d_n6, assign27790_e21448_d_n7, assign27790_e21448_d_n8, assign27790_e21448_d_n9, assign27790_e21448_d_n10, assign27790_e21448_d_n11, assign27790_e21448_d_n12,) = {
    if ((((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) && (locals.var_guard1664 != 0.0)) {
        (1e-5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27790_e21448;
        locals.var_t0__blk1144_dn3 = assign27790_e21448_d_n3;
        locals.var_t0__blk1144_dn4 = assign27790_e21448_d_n4;
        locals.var_t0__blk1144_dn5 = assign27790_e21448_d_n5;
        locals.var_t0__blk1144_dn6 = assign27790_e21448_d_n6;
        locals.var_t0__blk1144_dn7 = assign27790_e21448_d_n7;
        locals.var_t0__blk1144_dn8 = assign27790_e21448_d_n8;
        locals.var_t0__blk1144_dn9 = assign27790_e21448_d_n9;
        locals.var_t0__blk1144_dn10 = assign27790_e21448_d_n10;
        locals.var_t0__blk1144_dn11 = assign27790_e21448_d_n11;
        locals.var_t0__blk1144_dn12 = assign27790_e21448_d_n12;

        let assign27800_e21452: f64 = (locals.var_vdiff / 100.0);
        let assign27800_e21457: f64 = if ((locals.var_t0__blk1144 < assign27800_e21452) && (locals.var_vdiff > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1665 = assign27800_e21457;

        let (assign27810_e21470, assign27810_e21470_d_n3, assign27810_e21470_d_n4, assign27810_e21470_d_n5, assign27810_e21470_d_n6, assign27810_e21470_d_n7, assign27810_e21470_d_n8, assign27810_e21470_d_n9, assign27810_e21470_d_n10, assign27810_e21470_d_n11, assign27810_e21470_d_n12,) = {
    if ((((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) && (locals.var_guard1665 != 0.0)) {
        let assign27810_e21468: f64 = (locals.var_pparam_b4soialpha0 * 2.688117142e43);
        (assign27810_e21468, (locals.var_pparam_b4soialpha0_dn3 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn4 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn5 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn6 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn7 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn8 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn9 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn10 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn11 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn12 * 2.688117142e43),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign27810_e21470;
        locals.var_ratio_dn3 = assign27810_e21470_d_n3;
        locals.var_ratio_dn4 = assign27810_e21470_d_n4;
        locals.var_ratio_dn5 = assign27810_e21470_d_n5;
        locals.var_ratio_dn6 = assign27810_e21470_d_n6;
        locals.var_ratio_dn7 = assign27810_e21470_d_n7;
        locals.var_ratio_dn8 = assign27810_e21470_d_n8;
        locals.var_ratio_dn9 = assign27810_e21470_d_n9;
        locals.var_ratio_dn10 = assign27810_e21470_d_n10;
        locals.var_ratio_dn11 = assign27810_e21470_d_n11;
        locals.var_ratio_dn12 = assign27810_e21470_d_n12;

        let assign27820_e21473: f64 = (-locals.var_vdiff);
        let assign27820_e21475: f64 = (assign27820_e21473 / 100.0);
        let assign27820_e21480: f64 = if ((locals.var_t0__blk1144 < assign27820_e21475) && (locals.var_vdiff < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1666 = assign27820_e21480;

        let (assign27830_e21496, assign27830_e21496_d_n3, assign27830_e21496_d_n4, assign27830_e21496_d_n5, assign27830_e21496_d_n6, assign27830_e21496_d_n7, assign27830_e21496_d_n8, assign27830_e21496_d_n9, assign27830_e21496_d_n10, assign27830_e21496_d_n11, assign27830_e21496_d_n12,) = {
    if (((((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1666 != 0.0)) {
        let assign27830_e21494: f64 = (locals.var_pparam_b4soialpha0 * 3.720075976e-44);
        (assign27830_e21494, (locals.var_pparam_b4soialpha0_dn3 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn4 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn5 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn6 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn7 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn8 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn9 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn10 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn11 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn12 * 3.720075976e-44),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign27830_e21496;
        locals.var_ratio_dn3 = assign27830_e21496_d_n3;
        locals.var_ratio_dn4 = assign27830_e21496_d_n4;
        locals.var_ratio_dn5 = assign27830_e21496_d_n5;
        locals.var_ratio_dn6 = assign27830_e21496_d_n6;
        locals.var_ratio_dn7 = assign27830_e21496_d_n7;
        locals.var_ratio_dn8 = assign27830_e21496_d_n8;
        locals.var_ratio_dn9 = assign27830_e21496_d_n9;
        locals.var_ratio_dn10 = assign27830_e21496_d_n10;
        locals.var_ratio_dn11 = assign27830_e21496_d_n11;
        locals.var_ratio_dn12 = assign27830_e21496_d_n12;

        let (assign27840_e21516, assign27840_e21516_d_n3, assign27840_e21516_d_n4, assign27840_e21516_d_n5, assign27840_e21516_d_n6, assign27840_e21516_d_n7, assign27840_e21516_d_n8, assign27840_e21516_d_n9, assign27840_e21516_d_n10, assign27840_e21516_d_n11, assign27840_e21516_d_n12,) = {
    if (((((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1666 == 0.0)) {
        let assign27840_e21512: f64 = (locals.var_vdiff / locals.var_t0__blk1144);
        let assign27840_e21513: f64 = (assign27840_e21512).exp();
        let assign27840_e21514: f64 = (locals.var_pparam_b4soialpha0 * assign27840_e21513);
        (assign27840_e21514, ((locals.var_pparam_b4soialpha0_dn3 * assign27840_e21513) + (locals.var_pparam_b4soialpha0 * (assign27840_e21513 * (((locals.var_vdiff_dn3 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn3)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn4 * assign27840_e21513) + (locals.var_pparam_b4soialpha0 * (assign27840_e21513 * (((locals.var_vdiff_dn4 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn4)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn5 * assign27840_e21513) + (locals.var_pparam_b4soialpha0 * (assign27840_e21513 * (((locals.var_vdiff_dn5 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn5)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn6 * assign27840_e21513) + (locals.var_pparam_b4soialpha0 * (assign27840_e21513 * (((locals.var_vdiff_dn6 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn6)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn7 * assign27840_e21513) + (locals.var_pparam_b4soialpha0 * (assign27840_e21513 * (((locals.var_vdiff_dn7 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn7)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn8 * assign27840_e21513) + (locals.var_pparam_b4soialpha0 * (assign27840_e21513 * (((locals.var_vdiff_dn8 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn8)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn9 * assign27840_e21513) + (locals.var_pparam_b4soialpha0 * (assign27840_e21513 * (((locals.var_vdiff_dn9 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn9)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn10 * assign27840_e21513) + (locals.var_pparam_b4soialpha0 * (assign27840_e21513 * (((locals.var_vdiff_dn10 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn10)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn11 * assign27840_e21513) + (locals.var_pparam_b4soialpha0 * (assign27840_e21513 * (((locals.var_vdiff_dn11 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn11)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn12 * assign27840_e21513) + (locals.var_pparam_b4soialpha0 * (assign27840_e21513 * (((locals.var_vdiff_dn12 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn12)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign27840_e21516;
        locals.var_ratio_dn3 = assign27840_e21516_d_n3;
        locals.var_ratio_dn4 = assign27840_e21516_d_n4;
        locals.var_ratio_dn5 = assign27840_e21516_d_n5;
        locals.var_ratio_dn6 = assign27840_e21516_d_n6;
        locals.var_ratio_dn7 = assign27840_e21516_d_n7;
        locals.var_ratio_dn8 = assign27840_e21516_d_n8;
        locals.var_ratio_dn9 = assign27840_e21516_d_n9;
        locals.var_ratio_dn10 = assign27840_e21516_d_n10;
        locals.var_ratio_dn11 = assign27840_e21516_d_n11;
        locals.var_ratio_dn12 = assign27840_e21516_d_n12;

        let assign27850_e21519: f64 = if locals.var_ratio > 10.0 { 1.0 } else { 0.0 };
        locals.var_guard1667 = assign27850_e21519;

        let (assign27860_e21530, assign27860_e21530_d_n3, assign27860_e21530_d_n4, assign27860_e21530_d_n5, assign27860_e21530_d_n6, assign27860_e21530_d_n7, assign27860_e21530_d_n8, assign27860_e21530_d_n9, assign27860_e21530_d_n10, assign27860_e21530_d_n11, assign27860_e21530_d_n12,) = {
    if ((((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) && (locals.var_guard1667 != 0.0)) {
        (10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign27860_e21530;
        locals.var_ratio_dn3 = assign27860_e21530_d_n3;
        locals.var_ratio_dn4 = assign27860_e21530_d_n4;
        locals.var_ratio_dn5 = assign27860_e21530_d_n5;
        locals.var_ratio_dn6 = assign27860_e21530_d_n6;
        locals.var_ratio_dn7 = assign27860_e21530_d_n7;
        locals.var_ratio_dn8 = assign27860_e21530_d_n8;
        locals.var_ratio_dn9 = assign27860_e21530_d_n9;
        locals.var_ratio_dn10 = assign27860_e21530_d_n10;
        locals.var_ratio_dn11 = assign27860_e21530_d_n11;
        locals.var_ratio_dn12 = assign27860_e21530_d_n12;

        let (assign27870_e21545, assign27870_e21545_d_n3, assign27870_e21545_d_n4, assign27870_e21545_d_n5, assign27870_e21545_d_n6, assign27870_e21545_d_n7, assign27870_e21545_d_n8, assign27870_e21545_d_n9, assign27870_e21545_d_n10, assign27870_e21545_d_n11, assign27870_e21545_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27870_e21540: f64 = (locals.var_pparam_b4soifbjtii * locals.var_b4soimode);
        let assign27870_e21542: f64 = (assign27870_e21540 * locals.var_ic_1);
        let assign27870_e21543: f64 = (locals.var_ids_1 + assign27870_e21542);
        (assign27870_e21543, (locals.var_ids_1_dn3 + (((locals.var_pparam_b4soifbjtii_dn3 * locals.var_b4soimode) * locals.var_ic_1) + (assign27870_e21540 * locals.var_ic_1_dn3))), (locals.var_ids_1_dn4 + (((locals.var_pparam_b4soifbjtii_dn4 * locals.var_b4soimode) * locals.var_ic_1) + (assign27870_e21540 * locals.var_ic_1_dn4))), (locals.var_ids_1_dn5 + (((locals.var_pparam_b4soifbjtii_dn5 * locals.var_b4soimode) * locals.var_ic_1) + (assign27870_e21540 * locals.var_ic_1_dn5))), (locals.var_ids_1_dn6 + (((locals.var_pparam_b4soifbjtii_dn6 * locals.var_b4soimode) * locals.var_ic_1) + (assign27870_e21540 * locals.var_ic_1_dn6))), (locals.var_ids_1_dn7 + (((locals.var_pparam_b4soifbjtii_dn7 * locals.var_b4soimode) * locals.var_ic_1) + (assign27870_e21540 * locals.var_ic_1_dn7))), (locals.var_ids_1_dn8 + (((locals.var_pparam_b4soifbjtii_dn8 * locals.var_b4soimode) * locals.var_ic_1) + (assign27870_e21540 * locals.var_ic_1_dn8))), (locals.var_ids_1_dn9 + (((locals.var_pparam_b4soifbjtii_dn9 * locals.var_b4soimode) * locals.var_ic_1) + (assign27870_e21540 * locals.var_ic_1_dn9))), (locals.var_ids_1_dn10 + (((locals.var_pparam_b4soifbjtii_dn10 * locals.var_b4soimode) * locals.var_ic_1) + (assign27870_e21540 * locals.var_ic_1_dn10))), (locals.var_ids_1_dn11 + (((locals.var_pparam_b4soifbjtii_dn11 * locals.var_b4soimode) * locals.var_ic_1) + (assign27870_e21540 * locals.var_ic_1_dn11))), (locals.var_ids_1_dn12 + (((locals.var_pparam_b4soifbjtii_dn12 * locals.var_b4soimode) * locals.var_ic_1) + (assign27870_e21540 * locals.var_ic_1_dn12))),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27870_e21545;
        locals.var_t0__blk1144_dn3 = assign27870_e21545_d_n3;
        locals.var_t0__blk1144_dn4 = assign27870_e21545_d_n4;
        locals.var_t0__blk1144_dn5 = assign27870_e21545_d_n5;
        locals.var_t0__blk1144_dn6 = assign27870_e21545_d_n6;
        locals.var_t0__blk1144_dn7 = assign27870_e21545_d_n7;
        locals.var_t0__blk1144_dn8 = assign27870_e21545_d_n8;
        locals.var_t0__blk1144_dn9 = assign27870_e21545_d_n9;
        locals.var_t0__blk1144_dn10 = assign27870_e21545_d_n10;
        locals.var_t0__blk1144_dn11 = assign27870_e21545_d_n11;
        locals.var_t0__blk1144_dn12 = assign27870_e21545_d_n12;

        let (assign27880_e21556, assign27880_e21556_d_n3, assign27880_e21556_d_n4, assign27880_e21556_d_n5, assign27880_e21556_d_n6, assign27880_e21556_d_n7, assign27880_e21556_d_n8, assign27880_e21556_d_n9, assign27880_e21556_d_n10, assign27880_e21556_d_n11, assign27880_e21556_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign27880_e21554: f64 = (locals.var_ratio * locals.var_t0__blk1144);
        (assign27880_e21554, ((locals.var_ratio_dn3 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn3)), ((locals.var_ratio_dn4 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn4)), ((locals.var_ratio_dn5 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn5)), ((locals.var_ratio_dn6 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn6)), ((locals.var_ratio_dn7 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn7)), ((locals.var_ratio_dn8 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn8)), ((locals.var_ratio_dn9 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn9)), ((locals.var_ratio_dn10 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn10)), ((locals.var_ratio_dn11 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn11)), ((locals.var_ratio_dn12 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11, locals.var_iii_dn12,)
    }
};
        locals.var_iii = assign27880_e21556;
        locals.var_iii_dn3 = assign27880_e21556_d_n3;
        locals.var_iii_dn4 = assign27880_e21556_d_n4;
        locals.var_iii_dn5 = assign27880_e21556_d_n5;
        locals.var_iii_dn6 = assign27880_e21556_d_n6;
        locals.var_iii_dn7 = assign27880_e21556_d_n7;
        locals.var_iii_dn8 = assign27880_e21556_d_n8;
        locals.var_iii_dn9 = assign27880_e21556_d_n9;
        locals.var_iii_dn10 = assign27880_e21556_d_n10;
        locals.var_iii_dn11 = assign27880_e21556_d_n11;
        locals.var_iii_dn12 = assign27880_e21556_d_n12;

        let assign27890_e21559: f64 = if locals.var_pparam_b4soialpha0 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1668 = assign27890_e21559;

        let (assign27900_e21568, assign27900_e21568_d_n3, assign27900_e21568_d_n4, assign27900_e21568_d_n5, assign27900_e21568_d_n6, assign27900_e21568_d_n7, assign27900_e21568_d_n8, assign27900_e21568_d_n9, assign27900_e21568_d_n10, assign27900_e21568_d_n11, assign27900_e21568_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idsmosfet, locals.var_idsmosfet_dn3, locals.var_idsmosfet_dn4, locals.var_idsmosfet_dn5, locals.var_idsmosfet_dn6, locals.var_idsmosfet_dn7, locals.var_idsmosfet_dn8, locals.var_idsmosfet_dn9, locals.var_idsmosfet_dn10, locals.var_idsmosfet_dn11, locals.var_idsmosfet_dn12,)
    }
};
        locals.var_idsmosfet = assign27900_e21568;
        locals.var_idsmosfet_dn3 = assign27900_e21568_d_n3;
        locals.var_idsmosfet_dn4 = assign27900_e21568_d_n4;
        locals.var_idsmosfet_dn5 = assign27900_e21568_d_n5;
        locals.var_idsmosfet_dn6 = assign27900_e21568_d_n6;
        locals.var_idsmosfet_dn7 = assign27900_e21568_d_n7;
        locals.var_idsmosfet_dn8 = assign27900_e21568_d_n8;
        locals.var_idsmosfet_dn9 = assign27900_e21568_d_n9;
        locals.var_idsmosfet_dn10 = assign27900_e21568_d_n10;
        locals.var_idsmosfet_dn11 = assign27900_e21568_d_n11;
        locals.var_idsmosfet_dn12 = assign27900_e21568_d_n12;

        let (assign27910_e21590, assign27910_e21590_d_n3, assign27910_e21590_d_n4, assign27910_e21590_d_n5, assign27910_e21590_d_n6, assign27910_e21590_d_n7, assign27910_e21590_d_n8, assign27910_e21590_d_n9, assign27910_e21590_d_n10, assign27910_e21590_d_n11, assign27910_e21590_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign27910_e21581: f64 = (locals.var_tempratio - 1.0);
        let assign27910_e21582: f64 = (locals.var_b4soitii * assign27910_e21581);
        let assign27910_e21583: f64 = (1.0 + assign27910_e21582);
        let assign27910_e21584: f64 = (locals.var_pparam_b4soivdsatii0 * assign27910_e21583);
        let assign27910_e21587: f64 = (locals.var_pparam_b4soilii / locals.var_leff);
        let assign27910_e21588: f64 = (assign27910_e21584 - assign27910_e21587);
        (assign27910_e21588, ((locals.var_pparam_b4soivdsatii0_dn3 * assign27910_e21583) - (((locals.var_pparam_b4soilii_dn3 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn4 * assign27910_e21583) - (((locals.var_pparam_b4soilii_dn4 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn5 * assign27910_e21583) - (((locals.var_pparam_b4soilii_dn5 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff))), (((locals.var_pparam_b4soivdsatii0_dn6 * assign27910_e21583) + (locals.var_pparam_b4soivdsatii0 * (locals.var_b4soitii * locals.var_tempratio_dn6))) - (((locals.var_pparam_b4soilii_dn6 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn7 * assign27910_e21583) - (((locals.var_pparam_b4soilii_dn7 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn8 * assign27910_e21583) - (((locals.var_pparam_b4soilii_dn8 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn9 * assign27910_e21583) - (((locals.var_pparam_b4soilii_dn9 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn10 * assign27910_e21583) - (((locals.var_pparam_b4soilii_dn10 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn11 * assign27910_e21583) - (((locals.var_pparam_b4soilii_dn11 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn12 * assign27910_e21583) - (((locals.var_pparam_b4soilii_dn12 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff))),)
    } else {
        (locals.var_vdsatii0, locals.var_vdsatii0_dn3, locals.var_vdsatii0_dn4, locals.var_vdsatii0_dn5, locals.var_vdsatii0_dn6, locals.var_vdsatii0_dn7, locals.var_vdsatii0_dn8, locals.var_vdsatii0_dn9, locals.var_vdsatii0_dn10, locals.var_vdsatii0_dn11, locals.var_vdsatii0_dn12,)
    }
};
        locals.var_vdsatii0 = assign27910_e21590;
        locals.var_vdsatii0_dn3 = assign27910_e21590_d_n3;
        locals.var_vdsatii0_dn4 = assign27910_e21590_d_n4;
        locals.var_vdsatii0_dn5 = assign27910_e21590_d_n5;
        locals.var_vdsatii0_dn6 = assign27910_e21590_d_n6;
        locals.var_vdsatii0_dn7 = assign27910_e21590_d_n7;
        locals.var_vdsatii0_dn8 = assign27910_e21590_d_n8;
        locals.var_vdsatii0_dn9 = assign27910_e21590_d_n9;
        locals.var_vdsatii0_dn10 = assign27910_e21590_d_n10;
        locals.var_vdsatii0_dn11 = assign27910_e21590_d_n11;
        locals.var_vdsatii0_dn12 = assign27910_e21590_d_n12;

        let (assign27920_e21602, assign27920_e21602_d_n3, assign27920_e21602_d_n4, assign27920_e21602_d_n5, assign27920_e21602_d_n6, assign27920_e21602_d_n7, assign27920_e21602_d_n8, assign27920_e21602_d_n9, assign27920_e21602_d_n10, assign27920_e21602_d_n11, assign27920_e21602_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign27920_e21600: f64 = (locals.var_pparam_b4soiesatii * locals.var_leff);
        (assign27920_e21600, ((locals.var_pparam_b4soiesatii_dn3 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn3)), ((locals.var_pparam_b4soiesatii_dn4 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn4)), ((locals.var_pparam_b4soiesatii_dn5 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn5)), ((locals.var_pparam_b4soiesatii_dn6 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn6)), ((locals.var_pparam_b4soiesatii_dn7 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn7)), ((locals.var_pparam_b4soiesatii_dn8 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn8)), ((locals.var_pparam_b4soiesatii_dn9 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn9)), ((locals.var_pparam_b4soiesatii_dn10 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn10)), ((locals.var_pparam_b4soiesatii_dn11 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn11)), ((locals.var_pparam_b4soiesatii_dn12 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27920_e21602;
        locals.var_t0__blk1144_dn3 = assign27920_e21602_d_n3;
        locals.var_t0__blk1144_dn4 = assign27920_e21602_d_n4;
        locals.var_t0__blk1144_dn5 = assign27920_e21602_d_n5;
        locals.var_t0__blk1144_dn6 = assign27920_e21602_d_n6;
        locals.var_t0__blk1144_dn7 = assign27920_e21602_d_n7;
        locals.var_t0__blk1144_dn8 = assign27920_e21602_d_n8;
        locals.var_t0__blk1144_dn9 = assign27920_e21602_d_n9;
        locals.var_t0__blk1144_dn10 = assign27920_e21602_d_n10;
        locals.var_t0__blk1144_dn11 = assign27920_e21602_d_n11;
        locals.var_t0__blk1144_dn12 = assign27920_e21602_d_n12;

        let (assign27930_e21618, assign27930_e21618_d_n3, assign27930_e21618_d_n4, assign27930_e21618_d_n5, assign27930_e21618_d_n6, assign27930_e21618_d_n7, assign27930_e21618_d_n8, assign27930_e21618_d_n9, assign27930_e21618_d_n10, assign27930_e21618_d_n11, assign27930_e21618_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign27930_e21612: f64 = (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144);
        let assign27930_e21615: f64 = (1.0 + locals.var_t0__blk1144);
        let assign27930_e21616: f64 = (assign27930_e21612 / assign27930_e21615);
        (assign27930_e21616, (((((locals.var_pparam_b4soisii0_dn3 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn3)) * assign27930_e21615) - (assign27930_e21612 * locals.var_t0__blk1144_dn3)) / (assign27930_e21615 * assign27930_e21615)), (((((locals.var_pparam_b4soisii0_dn4 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn4)) * assign27930_e21615) - (assign27930_e21612 * locals.var_t0__blk1144_dn4)) / (assign27930_e21615 * assign27930_e21615)), (((((locals.var_pparam_b4soisii0_dn5 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn5)) * assign27930_e21615) - (assign27930_e21612 * locals.var_t0__blk1144_dn5)) / (assign27930_e21615 * assign27930_e21615)), (((((locals.var_pparam_b4soisii0_dn6 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn6)) * assign27930_e21615) - (assign27930_e21612 * locals.var_t0__blk1144_dn6)) / (assign27930_e21615 * assign27930_e21615)), (((((locals.var_pparam_b4soisii0_dn7 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn7)) * assign27930_e21615) - (assign27930_e21612 * locals.var_t0__blk1144_dn7)) / (assign27930_e21615 * assign27930_e21615)), (((((locals.var_pparam_b4soisii0_dn8 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn8)) * assign27930_e21615) - (assign27930_e21612 * locals.var_t0__blk1144_dn8)) / (assign27930_e21615 * assign27930_e21615)), (((((locals.var_pparam_b4soisii0_dn9 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn9)) * assign27930_e21615) - (assign27930_e21612 * locals.var_t0__blk1144_dn9)) / (assign27930_e21615 * assign27930_e21615)), (((((locals.var_pparam_b4soisii0_dn10 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn10)) * assign27930_e21615) - (assign27930_e21612 * locals.var_t0__blk1144_dn10)) / (assign27930_e21615 * assign27930_e21615)), (((((locals.var_pparam_b4soisii0_dn11 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn11)) * assign27930_e21615) - (assign27930_e21612 * locals.var_t0__blk1144_dn11)) / (assign27930_e21615 * assign27930_e21615)), (((((locals.var_pparam_b4soisii0_dn12 * locals.var_t0__blk1144) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk1144_dn12)) * assign27930_e21615) - (assign27930_e21612 * locals.var_t0__blk1144_dn12)) / (assign27930_e21615 * assign27930_e21615)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign27930_e21618;
        locals.var_t1__blk1145_dn3 = assign27930_e21618_d_n3;
        locals.var_t1__blk1145_dn4 = assign27930_e21618_d_n4;
        locals.var_t1__blk1145_dn5 = assign27930_e21618_d_n5;
        locals.var_t1__blk1145_dn6 = assign27930_e21618_d_n6;
        locals.var_t1__blk1145_dn7 = assign27930_e21618_d_n7;
        locals.var_t1__blk1145_dn8 = assign27930_e21618_d_n8;
        locals.var_t1__blk1145_dn9 = assign27930_e21618_d_n9;
        locals.var_t1__blk1145_dn10 = assign27930_e21618_d_n10;
        locals.var_t1__blk1145_dn11 = assign27930_e21618_d_n11;
        locals.var_t1__blk1145_dn12 = assign27930_e21618_d_n12;

        let (assign27940_e21634, assign27940_e21634_d_n3, assign27940_e21634_d_n4, assign27940_e21634_d_n5, assign27940_e21634_d_n6, assign27940_e21634_d_n7, assign27940_e21634_d_n8, assign27940_e21634_d_n9, assign27940_e21634_d_n10, assign27940_e21634_d_n11, assign27940_e21634_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign27940_e21630: f64 = (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175);
        let assign27940_e21631: f64 = (1.0 + assign27940_e21630);
        let assign27940_e21632: f64 = (1.0 / assign27940_e21631);
        (assign27940_e21632, (-(((locals.var_pparam_b4soisii1_dn3 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn3)) / (assign27940_e21631 * assign27940_e21631))), (-(((locals.var_pparam_b4soisii1_dn4 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn4)) / (assign27940_e21631 * assign27940_e21631))), (-(((locals.var_pparam_b4soisii1_dn5 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn5)) / (assign27940_e21631 * assign27940_e21631))), (-(((locals.var_pparam_b4soisii1_dn6 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn6)) / (assign27940_e21631 * assign27940_e21631))), (-(((locals.var_pparam_b4soisii1_dn7 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn7)) / (assign27940_e21631 * assign27940_e21631))), (-(((locals.var_pparam_b4soisii1_dn8 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn8)) / (assign27940_e21631 * assign27940_e21631))), (-(((locals.var_pparam_b4soisii1_dn9 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn9)) / (assign27940_e21631 * assign27940_e21631))), (-(((locals.var_pparam_b4soisii1_dn10 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn10)) / (assign27940_e21631 * assign27940_e21631))), (-(((locals.var_pparam_b4soisii1_dn11 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn11)) / (assign27940_e21631 * assign27940_e21631))), (-(((locals.var_pparam_b4soisii1_dn12 * locals.var_vgsteff__blk1175) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk1175_dn12)) / (assign27940_e21631 * assign27940_e21631))),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign27940_e21634;
        locals.var_t0__blk1144_dn3 = assign27940_e21634_d_n3;
        locals.var_t0__blk1144_dn4 = assign27940_e21634_d_n4;
        locals.var_t0__blk1144_dn5 = assign27940_e21634_d_n5;
        locals.var_t0__blk1144_dn6 = assign27940_e21634_d_n6;
        locals.var_t0__blk1144_dn7 = assign27940_e21634_d_n7;
        locals.var_t0__blk1144_dn8 = assign27940_e21634_d_n8;
        locals.var_t0__blk1144_dn9 = assign27940_e21634_d_n9;
        locals.var_t0__blk1144_dn10 = assign27940_e21634_d_n10;
        locals.var_t0__blk1144_dn11 = assign27940_e21634_d_n11;
        locals.var_t0__blk1144_dn12 = assign27940_e21634_d_n12;

        let (assign27950_e21646, assign27950_e21646_d_n3, assign27950_e21646_d_n4, assign27950_e21646_d_n5, assign27950_e21646_d_n6, assign27950_e21646_d_n7, assign27950_e21646_d_n8, assign27950_e21646_d_n9, assign27950_e21646_d_n10, assign27950_e21646_d_n11, assign27950_e21646_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign27950_e21644: f64 = (locals.var_t0__blk1144 + locals.var_pparam_b4soisii2);
        (assign27950_e21644, (locals.var_t0__blk1144_dn3 + locals.var_pparam_b4soisii2_dn3), (locals.var_t0__blk1144_dn4 + locals.var_pparam_b4soisii2_dn4), (locals.var_t0__blk1144_dn5 + locals.var_pparam_b4soisii2_dn5), (locals.var_t0__blk1144_dn6 + locals.var_pparam_b4soisii2_dn6), (locals.var_t0__blk1144_dn7 + locals.var_pparam_b4soisii2_dn7), (locals.var_t0__blk1144_dn8 + locals.var_pparam_b4soisii2_dn8), (locals.var_t0__blk1144_dn9 + locals.var_pparam_b4soisii2_dn9), (locals.var_t0__blk1144_dn10 + locals.var_pparam_b4soisii2_dn10), (locals.var_t0__blk1144_dn11 + locals.var_pparam_b4soisii2_dn11), (locals.var_t0__blk1144_dn12 + locals.var_pparam_b4soisii2_dn12),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign27950_e21646;
        locals.var_t3__blk1147_dn3 = assign27950_e21646_d_n3;
        locals.var_t3__blk1147_dn4 = assign27950_e21646_d_n4;
        locals.var_t3__blk1147_dn5 = assign27950_e21646_d_n5;
        locals.var_t3__blk1147_dn6 = assign27950_e21646_d_n6;
        locals.var_t3__blk1147_dn7 = assign27950_e21646_d_n7;
        locals.var_t3__blk1147_dn8 = assign27950_e21646_d_n8;
        locals.var_t3__blk1147_dn9 = assign27950_e21646_d_n9;
        locals.var_t3__blk1147_dn10 = assign27950_e21646_d_n10;
        locals.var_t3__blk1147_dn11 = assign27950_e21646_d_n11;
        locals.var_t3__blk1147_dn12 = assign27950_e21646_d_n12;

        let (assign27960_e21658, assign27960_e21658_d_n3, assign27960_e21658_d_n4, assign27960_e21658_d_n5, assign27960_e21658_d_n6, assign27960_e21658_d_n7, assign27960_e21658_d_n8, assign27960_e21658_d_n9, assign27960_e21658_d_n10, assign27960_e21658_d_n11, assign27960_e21658_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign27960_e21656: f64 = (locals.var_vgst__blk1131 * locals.var_t3__blk1147);
        (assign27960_e21656, ((locals.var_vgst__blk1131_dn3 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn3)), ((locals.var_vgst__blk1131_dn4 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn4)), ((locals.var_vgst__blk1131_dn5 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn5)), ((locals.var_vgst__blk1131_dn6 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn6)), ((locals.var_vgst__blk1131_dn7 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn7)), ((locals.var_vgst__blk1131_dn8 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn8)), ((locals.var_vgst__blk1131_dn9 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn9)), ((locals.var_vgst__blk1131_dn10 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn10)), ((locals.var_vgst__blk1131_dn11 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn11)), ((locals.var_vgst__blk1131_dn12 * locals.var_t3__blk1147) + (locals.var_vgst__blk1131 * locals.var_t3__blk1147_dn12)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign27960_e21658;
        locals.var_t2__blk1146_dn3 = assign27960_e21658_d_n3;
        locals.var_t2__blk1146_dn4 = assign27960_e21658_d_n4;
        locals.var_t2__blk1146_dn5 = assign27960_e21658_d_n5;
        locals.var_t2__blk1146_dn6 = assign27960_e21658_d_n6;
        locals.var_t2__blk1146_dn7 = assign27960_e21658_d_n7;
        locals.var_t2__blk1146_dn8 = assign27960_e21658_d_n8;
        locals.var_t2__blk1146_dn9 = assign27960_e21658_d_n9;
        locals.var_t2__blk1146_dn10 = assign27960_e21658_d_n10;
        locals.var_t2__blk1146_dn11 = assign27960_e21658_d_n11;
        locals.var_t2__blk1146_dn12 = assign27960_e21658_d_n12;

        let (assign27970_e21674, assign27970_e21674_d_n3, assign27970_e21674_d_n4, assign27970_e21674_d_n5, assign27970_e21674_d_n6, assign27970_e21674_d_n7, assign27970_e21674_d_n8, assign27970_e21674_d_n9, assign27970_e21674_d_n10, assign27970_e21674_d_n11, assign27970_e21674_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign27970_e21670: f64 = (locals.var_pparam_b4soisiid * locals.var_vds_1);
        let assign27970_e21671: f64 = (1.0 + assign27970_e21670);
        let assign27970_e21672: f64 = (1.0 / assign27970_e21671);
        (assign27970_e21672, (-((locals.var_pparam_b4soisiid_dn3 * locals.var_vds_1) / (assign27970_e21671 * assign27970_e21671))), (-((locals.var_pparam_b4soisiid_dn4 * locals.var_vds_1) / (assign27970_e21671 * assign27970_e21671))), (-((locals.var_pparam_b4soisiid_dn5 * locals.var_vds_1) / (assign27970_e21671 * assign27970_e21671))), (-((locals.var_pparam_b4soisiid_dn6 * locals.var_vds_1) / (assign27970_e21671 * assign27970_e21671))), (-(((locals.var_pparam_b4soisiid_dn7 * locals.var_vds_1) + (locals.var_pparam_b4soisiid * locals.var_vds_1_dn7)) / (assign27970_e21671 * assign27970_e21671))), (-(((locals.var_pparam_b4soisiid_dn8 * locals.var_vds_1) + (locals.var_pparam_b4soisiid * locals.var_vds_1_dn8)) / (assign27970_e21671 * assign27970_e21671))), (-((locals.var_pparam_b4soisiid_dn9 * locals.var_vds_1) / (assign27970_e21671 * assign27970_e21671))), (-((locals.var_pparam_b4soisiid_dn10 * locals.var_vds_1) / (assign27970_e21671 * assign27970_e21671))), (-((locals.var_pparam_b4soisiid_dn11 * locals.var_vds_1) / (assign27970_e21671 * assign27970_e21671))), (-((locals.var_pparam_b4soisiid_dn12 * locals.var_vds_1) / (assign27970_e21671 * assign27970_e21671))),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign27970_e21674;
        locals.var_t3__blk1147_dn3 = assign27970_e21674_d_n3;
        locals.var_t3__blk1147_dn4 = assign27970_e21674_d_n4;
        locals.var_t3__blk1147_dn5 = assign27970_e21674_d_n5;
        locals.var_t3__blk1147_dn6 = assign27970_e21674_d_n6;
        locals.var_t3__blk1147_dn7 = assign27970_e21674_d_n7;
        locals.var_t3__blk1147_dn8 = assign27970_e21674_d_n8;
        locals.var_t3__blk1147_dn9 = assign27970_e21674_d_n9;
        locals.var_t3__blk1147_dn10 = assign27970_e21674_d_n10;
        locals.var_t3__blk1147_dn11 = assign27970_e21674_d_n11;
        locals.var_t3__blk1147_dn12 = assign27970_e21674_d_n12;

        let (assign27980_e21688, assign27980_e21688_d_n3, assign27980_e21688_d_n4, assign27980_e21688_d_n5, assign27980_e21688_d_n6, assign27980_e21688_d_n7, assign27980_e21688_d_n8, assign27980_e21688_d_n9, assign27980_e21688_d_n10, assign27980_e21688_d_n11, assign27980_e21688_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign27980_e21684: f64 = (locals.var_t1__blk1145 * locals.var_t2__blk1146);
        let assign27980_e21686: f64 = (assign27980_e21684 * locals.var_t3__blk1147);
        (assign27980_e21686, ((((locals.var_t1__blk1145_dn3 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn3)) * locals.var_t3__blk1147) + (assign27980_e21684 * locals.var_t3__blk1147_dn3)), ((((locals.var_t1__blk1145_dn4 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn4)) * locals.var_t3__blk1147) + (assign27980_e21684 * locals.var_t3__blk1147_dn4)), ((((locals.var_t1__blk1145_dn5 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn5)) * locals.var_t3__blk1147) + (assign27980_e21684 * locals.var_t3__blk1147_dn5)), ((((locals.var_t1__blk1145_dn6 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn6)) * locals.var_t3__blk1147) + (assign27980_e21684 * locals.var_t3__blk1147_dn6)), ((((locals.var_t1__blk1145_dn7 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn7)) * locals.var_t3__blk1147) + (assign27980_e21684 * locals.var_t3__blk1147_dn7)), ((((locals.var_t1__blk1145_dn8 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn8)) * locals.var_t3__blk1147) + (assign27980_e21684 * locals.var_t3__blk1147_dn8)), ((((locals.var_t1__blk1145_dn9 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn9)) * locals.var_t3__blk1147) + (assign27980_e21684 * locals.var_t3__blk1147_dn9)), ((((locals.var_t1__blk1145_dn10 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn10)) * locals.var_t3__blk1147) + (assign27980_e21684 * locals.var_t3__blk1147_dn10)), ((((locals.var_t1__blk1145_dn11 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn11)) * locals.var_t3__blk1147) + (assign27980_e21684 * locals.var_t3__blk1147_dn11)), ((((locals.var_t1__blk1145_dn12 * locals.var_t2__blk1146) + (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn12)) * locals.var_t3__blk1147) + (assign27980_e21684 * locals.var_t3__blk1147_dn12)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11, locals.var_vgsstep_dn12,)
    }
};
        locals.var_vgsstep = assign27980_e21688;
        locals.var_vgsstep_dn3 = assign27980_e21688_d_n3;
        locals.var_vgsstep_dn4 = assign27980_e21688_d_n4;
        locals.var_vgsstep_dn5 = assign27980_e21688_d_n5;
        locals.var_vgsstep_dn6 = assign27980_e21688_d_n6;
        locals.var_vgsstep_dn7 = assign27980_e21688_d_n7;
        locals.var_vgsstep_dn8 = assign27980_e21688_d_n8;
        locals.var_vgsstep_dn9 = assign27980_e21688_d_n9;
        locals.var_vgsstep_dn10 = assign27980_e21688_d_n10;
        locals.var_vgsstep_dn11 = assign27980_e21688_d_n11;
        locals.var_vgsstep_dn12 = assign27980_e21688_d_n12;

        let (assign27990_e21700, assign27990_e21700_d_n3, assign27990_e21700_d_n4, assign27990_e21700_d_n5, assign27990_e21700_d_n6, assign27990_e21700_d_n7, assign27990_e21700_d_n8, assign27990_e21700_d_n9, assign27990_e21700_d_n10, assign27990_e21700_d_n11, assign27990_e21700_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign27990_e21698: f64 = (locals.var_vdsatii0 + locals.var_vgsstep);
        (assign27990_e21698, (locals.var_vdsatii0_dn3 + locals.var_vgsstep_dn3), (locals.var_vdsatii0_dn4 + locals.var_vgsstep_dn4), (locals.var_vdsatii0_dn5 + locals.var_vgsstep_dn5), (locals.var_vdsatii0_dn6 + locals.var_vgsstep_dn6), (locals.var_vdsatii0_dn7 + locals.var_vgsstep_dn7), (locals.var_vdsatii0_dn8 + locals.var_vgsstep_dn8), (locals.var_vdsatii0_dn9 + locals.var_vgsstep_dn9), (locals.var_vdsatii0_dn10 + locals.var_vgsstep_dn10), (locals.var_vdsatii0_dn11 + locals.var_vgsstep_dn11), (locals.var_vdsatii0_dn12 + locals.var_vgsstep_dn12),)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11, locals.var_vdsatii_dn12,)
    }
};
        locals.var_vdsatii = assign27990_e21700;
        locals.var_vdsatii_dn3 = assign27990_e21700_d_n3;
        locals.var_vdsatii_dn4 = assign27990_e21700_d_n4;
        locals.var_vdsatii_dn5 = assign27990_e21700_d_n5;
        locals.var_vdsatii_dn6 = assign27990_e21700_d_n6;
        locals.var_vdsatii_dn7 = assign27990_e21700_d_n7;
        locals.var_vdsatii_dn8 = assign27990_e21700_d_n8;
        locals.var_vdsatii_dn9 = assign27990_e21700_d_n9;
        locals.var_vdsatii_dn10 = assign27990_e21700_d_n10;
        locals.var_vdsatii_dn11 = assign27990_e21700_d_n11;
        locals.var_vdsatii_dn12 = assign27990_e21700_d_n12;

        let (assign28000_e21712, assign28000_e21712_d_n3, assign28000_e21712_d_n4, assign28000_e21712_d_n5, assign28000_e21712_d_n6, assign28000_e21712_d_n7, assign28000_e21712_d_n8, assign28000_e21712_d_n9, assign28000_e21712_d_n10, assign28000_e21712_d_n11, assign28000_e21712_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign28000_e21710: f64 = (locals.var_vds_1 - locals.var_vdsatii);
        (assign28000_e21710, (-locals.var_vdsatii_dn3), (-locals.var_vdsatii_dn4), (-locals.var_vdsatii_dn5), (-locals.var_vdsatii_dn6), (locals.var_vds_1_dn7 - locals.var_vdsatii_dn7), (locals.var_vds_1_dn8 - locals.var_vdsatii_dn8), (-locals.var_vdsatii_dn9), (-locals.var_vdsatii_dn10), (-locals.var_vdsatii_dn11), (-locals.var_vdsatii_dn12),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11, locals.var_vdiff_dn12,)
    }
};
        locals.var_vdiff = assign28000_e21712;
        locals.var_vdiff_dn3 = assign28000_e21712_d_n3;
        locals.var_vdiff_dn4 = assign28000_e21712_d_n4;
        locals.var_vdiff_dn5 = assign28000_e21712_d_n5;
        locals.var_vdiff_dn6 = assign28000_e21712_d_n6;
        locals.var_vdiff_dn7 = assign28000_e21712_d_n7;
        locals.var_vdiff_dn8 = assign28000_e21712_d_n8;
        locals.var_vdiff_dn9 = assign28000_e21712_d_n9;
        locals.var_vdiff_dn10 = assign28000_e21712_d_n10;
        locals.var_vdiff_dn11 = assign28000_e21712_d_n11;
        locals.var_vdiff_dn12 = assign28000_e21712_d_n12;

    }

    pub(super) fn stamp_transient_block_74(
        locals: &mut StampLocals,
    ) {
        let (assign28010_e21732, assign28010_e21732_d_n3, assign28010_e21732_d_n4, assign28010_e21732_d_n5, assign28010_e21732_d_n6, assign28010_e21732_d_n7, assign28010_e21732_d_n8, assign28010_e21732_d_n9, assign28010_e21732_d_n10, assign28010_e21732_d_n11, assign28010_e21732_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign28010_e21723: f64 = (locals.var_pparam_b4soibeta1 * locals.var_vdiff);
        let assign28010_e21724: f64 = (locals.var_pparam_b4soibeta2 + assign28010_e21723);
        let assign28010_e21727: f64 = (locals.var_pparam_b4soibeta0 * locals.var_vdiff);
        let assign28010_e21729: f64 = (assign28010_e21727 * locals.var_vdiff);
        let assign28010_e21730: f64 = (assign28010_e21724 + assign28010_e21729);
        (assign28010_e21730, ((locals.var_pparam_b4soibeta2_dn3 + ((locals.var_pparam_b4soibeta1_dn3 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn3))) + ((((locals.var_pparam_b4soibeta0_dn3 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn3)) * locals.var_vdiff) + (assign28010_e21727 * locals.var_vdiff_dn3))), ((locals.var_pparam_b4soibeta2_dn4 + ((locals.var_pparam_b4soibeta1_dn4 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn4))) + ((((locals.var_pparam_b4soibeta0_dn4 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn4)) * locals.var_vdiff) + (assign28010_e21727 * locals.var_vdiff_dn4))), ((locals.var_pparam_b4soibeta2_dn5 + ((locals.var_pparam_b4soibeta1_dn5 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn5))) + ((((locals.var_pparam_b4soibeta0_dn5 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn5)) * locals.var_vdiff) + (assign28010_e21727 * locals.var_vdiff_dn5))), ((locals.var_pparam_b4soibeta2_dn6 + ((locals.var_pparam_b4soibeta1_dn6 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn6))) + ((((locals.var_pparam_b4soibeta0_dn6 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn6)) * locals.var_vdiff) + (assign28010_e21727 * locals.var_vdiff_dn6))), ((locals.var_pparam_b4soibeta2_dn7 + ((locals.var_pparam_b4soibeta1_dn7 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn7))) + ((((locals.var_pparam_b4soibeta0_dn7 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn7)) * locals.var_vdiff) + (assign28010_e21727 * locals.var_vdiff_dn7))), ((locals.var_pparam_b4soibeta2_dn8 + ((locals.var_pparam_b4soibeta1_dn8 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn8))) + ((((locals.var_pparam_b4soibeta0_dn8 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn8)) * locals.var_vdiff) + (assign28010_e21727 * locals.var_vdiff_dn8))), ((locals.var_pparam_b4soibeta2_dn9 + ((locals.var_pparam_b4soibeta1_dn9 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn9))) + ((((locals.var_pparam_b4soibeta0_dn9 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn9)) * locals.var_vdiff) + (assign28010_e21727 * locals.var_vdiff_dn9))), ((locals.var_pparam_b4soibeta2_dn10 + ((locals.var_pparam_b4soibeta1_dn10 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn10))) + ((((locals.var_pparam_b4soibeta0_dn10 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn10)) * locals.var_vdiff) + (assign28010_e21727 * locals.var_vdiff_dn10))), ((locals.var_pparam_b4soibeta2_dn11 + ((locals.var_pparam_b4soibeta1_dn11 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn11))) + ((((locals.var_pparam_b4soibeta0_dn11 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn11)) * locals.var_vdiff) + (assign28010_e21727 * locals.var_vdiff_dn11))), ((locals.var_pparam_b4soibeta2_dn12 + ((locals.var_pparam_b4soibeta1_dn12 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn12))) + ((((locals.var_pparam_b4soibeta0_dn12 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn12)) * locals.var_vdiff) + (assign28010_e21727 * locals.var_vdiff_dn12))),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign28010_e21732;
        locals.var_t0__blk1144_dn3 = assign28010_e21732_d_n3;
        locals.var_t0__blk1144_dn4 = assign28010_e21732_d_n4;
        locals.var_t0__blk1144_dn5 = assign28010_e21732_d_n5;
        locals.var_t0__blk1144_dn6 = assign28010_e21732_d_n6;
        locals.var_t0__blk1144_dn7 = assign28010_e21732_d_n7;
        locals.var_t0__blk1144_dn8 = assign28010_e21732_d_n8;
        locals.var_t0__blk1144_dn9 = assign28010_e21732_d_n9;
        locals.var_t0__blk1144_dn10 = assign28010_e21732_d_n10;
        locals.var_t0__blk1144_dn11 = assign28010_e21732_d_n11;
        locals.var_t0__blk1144_dn12 = assign28010_e21732_d_n12;

        let assign28020_e21735: f64 = if locals.var_t0__blk1144 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1669 = assign28020_e21735;

        let (assign28030_e21747, assign28030_e21747_d_n3, assign28030_e21747_d_n4, assign28030_e21747_d_n5, assign28030_e21747_d_n6, assign28030_e21747_d_n7, assign28030_e21747_d_n8, assign28030_e21747_d_n9, assign28030_e21747_d_n10, assign28030_e21747_d_n11, assign28030_e21747_d_n12,) = {
    if ((((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        (1e-5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign28030_e21747;
        locals.var_t0__blk1144_dn3 = assign28030_e21747_d_n3;
        locals.var_t0__blk1144_dn4 = assign28030_e21747_d_n4;
        locals.var_t0__blk1144_dn5 = assign28030_e21747_d_n5;
        locals.var_t0__blk1144_dn6 = assign28030_e21747_d_n6;
        locals.var_t0__blk1144_dn7 = assign28030_e21747_d_n7;
        locals.var_t0__blk1144_dn8 = assign28030_e21747_d_n8;
        locals.var_t0__blk1144_dn9 = assign28030_e21747_d_n9;
        locals.var_t0__blk1144_dn10 = assign28030_e21747_d_n10;
        locals.var_t0__blk1144_dn11 = assign28030_e21747_d_n11;
        locals.var_t0__blk1144_dn12 = assign28030_e21747_d_n12;

        let assign28040_e21751: f64 = (locals.var_vdiff / 100.0);
        let assign28040_e21756: f64 = if ((locals.var_t0__blk1144 < assign28040_e21751) && (locals.var_vdiff > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1670 = assign28040_e21756;

        let (assign28050_e21770, assign28050_e21770_d_n3, assign28050_e21770_d_n4, assign28050_e21770_d_n5, assign28050_e21770_d_n6, assign28050_e21770_d_n7, assign28050_e21770_d_n8, assign28050_e21770_d_n9, assign28050_e21770_d_n10, assign28050_e21770_d_n11, assign28050_e21770_d_n12,) = {
    if ((((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign28050_e21768: f64 = (locals.var_pparam_b4soialpha0 * 2.688117142e43);
        (assign28050_e21768, (locals.var_pparam_b4soialpha0_dn3 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn4 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn5 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn6 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn7 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn8 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn9 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn10 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn11 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn12 * 2.688117142e43),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign28050_e21770;
        locals.var_ratio_dn3 = assign28050_e21770_d_n3;
        locals.var_ratio_dn4 = assign28050_e21770_d_n4;
        locals.var_ratio_dn5 = assign28050_e21770_d_n5;
        locals.var_ratio_dn6 = assign28050_e21770_d_n6;
        locals.var_ratio_dn7 = assign28050_e21770_d_n7;
        locals.var_ratio_dn8 = assign28050_e21770_d_n8;
        locals.var_ratio_dn9 = assign28050_e21770_d_n9;
        locals.var_ratio_dn10 = assign28050_e21770_d_n10;
        locals.var_ratio_dn11 = assign28050_e21770_d_n11;
        locals.var_ratio_dn12 = assign28050_e21770_d_n12;

        let assign28060_e21773: f64 = (-locals.var_vdiff);
        let assign28060_e21775: f64 = (assign28060_e21773 / 100.0);
        let assign28060_e21780: f64 = if ((locals.var_t0__blk1144 < assign28060_e21775) && (locals.var_vdiff < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1671 = assign28060_e21780;

        let (assign28070_e21797, assign28070_e21797_d_n3, assign28070_e21797_d_n4, assign28070_e21797_d_n5, assign28070_e21797_d_n6, assign28070_e21797_d_n7, assign28070_e21797_d_n8, assign28070_e21797_d_n9, assign28070_e21797_d_n10, assign28070_e21797_d_n11, assign28070_e21797_d_n12,) = {
    if (((((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) && (locals.var_guard1670 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let assign28070_e21795: f64 = (locals.var_pparam_b4soialpha0 * 3.720075976e-44);
        (assign28070_e21795, (locals.var_pparam_b4soialpha0_dn3 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn4 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn5 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn6 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn7 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn8 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn9 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn10 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn11 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn12 * 3.720075976e-44),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign28070_e21797;
        locals.var_ratio_dn3 = assign28070_e21797_d_n3;
        locals.var_ratio_dn4 = assign28070_e21797_d_n4;
        locals.var_ratio_dn5 = assign28070_e21797_d_n5;
        locals.var_ratio_dn6 = assign28070_e21797_d_n6;
        locals.var_ratio_dn7 = assign28070_e21797_d_n7;
        locals.var_ratio_dn8 = assign28070_e21797_d_n8;
        locals.var_ratio_dn9 = assign28070_e21797_d_n9;
        locals.var_ratio_dn10 = assign28070_e21797_d_n10;
        locals.var_ratio_dn11 = assign28070_e21797_d_n11;
        locals.var_ratio_dn12 = assign28070_e21797_d_n12;

        let (assign28080_e21818, assign28080_e21818_d_n3, assign28080_e21818_d_n4, assign28080_e21818_d_n5, assign28080_e21818_d_n6, assign28080_e21818_d_n7, assign28080_e21818_d_n8, assign28080_e21818_d_n9, assign28080_e21818_d_n10, assign28080_e21818_d_n11, assign28080_e21818_d_n12,) = {
    if (((((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) && (locals.var_guard1670 == 0.0)) && (locals.var_guard1671 == 0.0)) {
        let assign28080_e21814: f64 = (locals.var_vdiff / locals.var_t0__blk1144);
        let assign28080_e21815: f64 = (assign28080_e21814).exp();
        let assign28080_e21816: f64 = (locals.var_pparam_b4soialpha0 * assign28080_e21815);
        (assign28080_e21816, ((locals.var_pparam_b4soialpha0_dn3 * assign28080_e21815) + (locals.var_pparam_b4soialpha0 * (assign28080_e21815 * (((locals.var_vdiff_dn3 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn3)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn4 * assign28080_e21815) + (locals.var_pparam_b4soialpha0 * (assign28080_e21815 * (((locals.var_vdiff_dn4 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn4)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn5 * assign28080_e21815) + (locals.var_pparam_b4soialpha0 * (assign28080_e21815 * (((locals.var_vdiff_dn5 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn5)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn6 * assign28080_e21815) + (locals.var_pparam_b4soialpha0 * (assign28080_e21815 * (((locals.var_vdiff_dn6 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn6)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn7 * assign28080_e21815) + (locals.var_pparam_b4soialpha0 * (assign28080_e21815 * (((locals.var_vdiff_dn7 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn7)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn8 * assign28080_e21815) + (locals.var_pparam_b4soialpha0 * (assign28080_e21815 * (((locals.var_vdiff_dn8 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn8)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn9 * assign28080_e21815) + (locals.var_pparam_b4soialpha0 * (assign28080_e21815 * (((locals.var_vdiff_dn9 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn9)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn10 * assign28080_e21815) + (locals.var_pparam_b4soialpha0 * (assign28080_e21815 * (((locals.var_vdiff_dn10 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn10)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn11 * assign28080_e21815) + (locals.var_pparam_b4soialpha0 * (assign28080_e21815 * (((locals.var_vdiff_dn11 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn11)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))), ((locals.var_pparam_b4soialpha0_dn12 * assign28080_e21815) + (locals.var_pparam_b4soialpha0 * (assign28080_e21815 * (((locals.var_vdiff_dn12 * locals.var_t0__blk1144) - (locals.var_vdiff * locals.var_t0__blk1144_dn12)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))))),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign28080_e21818;
        locals.var_ratio_dn3 = assign28080_e21818_d_n3;
        locals.var_ratio_dn4 = assign28080_e21818_d_n4;
        locals.var_ratio_dn5 = assign28080_e21818_d_n5;
        locals.var_ratio_dn6 = assign28080_e21818_d_n6;
        locals.var_ratio_dn7 = assign28080_e21818_d_n7;
        locals.var_ratio_dn8 = assign28080_e21818_d_n8;
        locals.var_ratio_dn9 = assign28080_e21818_d_n9;
        locals.var_ratio_dn10 = assign28080_e21818_d_n10;
        locals.var_ratio_dn11 = assign28080_e21818_d_n11;
        locals.var_ratio_dn12 = assign28080_e21818_d_n12;

        let assign28090_e21821: f64 = if locals.var_ratio > 10.0 { 1.0 } else { 0.0 };
        locals.var_guard1672 = assign28090_e21821;

        let (assign28100_e21833, assign28100_e21833_d_n3, assign28100_e21833_d_n4, assign28100_e21833_d_n5, assign28100_e21833_d_n6, assign28100_e21833_d_n7, assign28100_e21833_d_n8, assign28100_e21833_d_n9, assign28100_e21833_d_n10, assign28100_e21833_d_n11, assign28100_e21833_d_n12,) = {
    if ((((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) && (locals.var_guard1672 != 0.0)) {
        (10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign28100_e21833;
        locals.var_ratio_dn3 = assign28100_e21833_d_n3;
        locals.var_ratio_dn4 = assign28100_e21833_d_n4;
        locals.var_ratio_dn5 = assign28100_e21833_d_n5;
        locals.var_ratio_dn6 = assign28100_e21833_d_n6;
        locals.var_ratio_dn7 = assign28100_e21833_d_n7;
        locals.var_ratio_dn8 = assign28100_e21833_d_n8;
        locals.var_ratio_dn9 = assign28100_e21833_d_n9;
        locals.var_ratio_dn10 = assign28100_e21833_d_n10;
        locals.var_ratio_dn11 = assign28100_e21833_d_n11;
        locals.var_ratio_dn12 = assign28100_e21833_d_n12;

        let (assign28110_e21843, assign28110_e21843_d_n3, assign28110_e21843_d_n4, assign28110_e21843_d_n5, assign28110_e21843_d_n6, assign28110_e21843_d_n7, assign28110_e21843_d_n8, assign28110_e21843_d_n9, assign28110_e21843_d_n10, assign28110_e21843_d_n11, assign28110_e21843_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        (locals.var_ids_1, locals.var_ids_1_dn3, locals.var_ids_1_dn4, locals.var_ids_1_dn5, locals.var_ids_1_dn6, locals.var_ids_1_dn7, locals.var_ids_1_dn8, locals.var_ids_1_dn9, locals.var_ids_1_dn10, locals.var_ids_1_dn11, locals.var_ids_1_dn12,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign28110_e21843;
        locals.var_t0__blk1144_dn3 = assign28110_e21843_d_n3;
        locals.var_t0__blk1144_dn4 = assign28110_e21843_d_n4;
        locals.var_t0__blk1144_dn5 = assign28110_e21843_d_n5;
        locals.var_t0__blk1144_dn6 = assign28110_e21843_d_n6;
        locals.var_t0__blk1144_dn7 = assign28110_e21843_d_n7;
        locals.var_t0__blk1144_dn8 = assign28110_e21843_d_n8;
        locals.var_t0__blk1144_dn9 = assign28110_e21843_d_n9;
        locals.var_t0__blk1144_dn10 = assign28110_e21843_d_n10;
        locals.var_t0__blk1144_dn11 = assign28110_e21843_d_n11;
        locals.var_t0__blk1144_dn12 = assign28110_e21843_d_n12;

        let (assign28120_e21855, assign28120_e21855_d_n3, assign28120_e21855_d_n4, assign28120_e21855_d_n5, assign28120_e21855_d_n6, assign28120_e21855_d_n7, assign28120_e21855_d_n8, assign28120_e21855_d_n9, assign28120_e21855_d_n10, assign28120_e21855_d_n11, assign28120_e21855_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign28120_e21853: f64 = (locals.var_ratio * locals.var_t0__blk1144);
        (assign28120_e21853, ((locals.var_ratio_dn3 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn3)), ((locals.var_ratio_dn4 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn4)), ((locals.var_ratio_dn5 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn5)), ((locals.var_ratio_dn6 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn6)), ((locals.var_ratio_dn7 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn7)), ((locals.var_ratio_dn8 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn8)), ((locals.var_ratio_dn9 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn9)), ((locals.var_ratio_dn10 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn10)), ((locals.var_ratio_dn11 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn11)), ((locals.var_ratio_dn12 * locals.var_t0__blk1144) + (locals.var_ratio * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_idsmosfet, locals.var_idsmosfet_dn3, locals.var_idsmosfet_dn4, locals.var_idsmosfet_dn5, locals.var_idsmosfet_dn6, locals.var_idsmosfet_dn7, locals.var_idsmosfet_dn8, locals.var_idsmosfet_dn9, locals.var_idsmosfet_dn10, locals.var_idsmosfet_dn11, locals.var_idsmosfet_dn12,)
    }
};
        locals.var_idsmosfet = assign28120_e21855;
        locals.var_idsmosfet_dn3 = assign28120_e21855_d_n3;
        locals.var_idsmosfet_dn4 = assign28120_e21855_d_n4;
        locals.var_idsmosfet_dn5 = assign28120_e21855_d_n5;
        locals.var_idsmosfet_dn6 = assign28120_e21855_d_n6;
        locals.var_idsmosfet_dn7 = assign28120_e21855_d_n7;
        locals.var_idsmosfet_dn8 = assign28120_e21855_d_n8;
        locals.var_idsmosfet_dn9 = assign28120_e21855_d_n9;
        locals.var_idsmosfet_dn10 = assign28120_e21855_d_n10;
        locals.var_idsmosfet_dn11 = assign28120_e21855_d_n11;
        locals.var_idsmosfet_dn12 = assign28120_e21855_d_n12;

        let (assign28130_e21868, assign28130_e21868_d_n3, assign28130_e21868_d_n4, assign28130_e21868_d_n5, assign28130_e21868_d_n6, assign28130_e21868_d_n7, assign28130_e21868_d_n8, assign28130_e21868_d_n9, assign28130_e21868_d_n10, assign28130_e21868_d_n11, assign28130_e21868_d_n12,) = {
    if ((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) {
        let assign28130_e21863: f64 = (locals.var_pparam_b4soiebjtii * locals.var_leff);
        let assign28130_e21864: f64 = (locals.var_pparam_b4soicbjtii + assign28130_e21863);
        let assign28130_e21866: f64 = (assign28130_e21864 / locals.var_leff);
        (assign28130_e21866, ((((locals.var_pparam_b4soicbjtii_dn3 + ((locals.var_pparam_b4soiebjtii_dn3 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn3))) * locals.var_leff) - (assign28130_e21864 * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn4 + ((locals.var_pparam_b4soiebjtii_dn4 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn4))) * locals.var_leff) - (assign28130_e21864 * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn5 + ((locals.var_pparam_b4soiebjtii_dn5 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn5))) * locals.var_leff) - (assign28130_e21864 * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn6 + ((locals.var_pparam_b4soiebjtii_dn6 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn6))) * locals.var_leff) - (assign28130_e21864 * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn7 + ((locals.var_pparam_b4soiebjtii_dn7 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn7))) * locals.var_leff) - (assign28130_e21864 * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn8 + ((locals.var_pparam_b4soiebjtii_dn8 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn8))) * locals.var_leff) - (assign28130_e21864 * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn9 + ((locals.var_pparam_b4soiebjtii_dn9 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn9))) * locals.var_leff) - (assign28130_e21864 * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn10 + ((locals.var_pparam_b4soiebjtii_dn10 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn10))) * locals.var_leff) - (assign28130_e21864 * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn11 + ((locals.var_pparam_b4soiebjtii_dn11 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn11))) * locals.var_leff) - (assign28130_e21864 * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn12 + ((locals.var_pparam_b4soiebjtii_dn12 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn12))) * locals.var_leff) - (assign28130_e21864 * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign28130_e21868;
        locals.var_t0__blk1144_dn3 = assign28130_e21868_d_n3;
        locals.var_t0__blk1144_dn4 = assign28130_e21868_d_n4;
        locals.var_t0__blk1144_dn5 = assign28130_e21868_d_n5;
        locals.var_t0__blk1144_dn6 = assign28130_e21868_d_n6;
        locals.var_t0__blk1144_dn7 = assign28130_e21868_d_n7;
        locals.var_t0__blk1144_dn8 = assign28130_e21868_d_n8;
        locals.var_t0__blk1144_dn9 = assign28130_e21868_d_n9;
        locals.var_t0__blk1144_dn10 = assign28130_e21868_d_n10;
        locals.var_t0__blk1144_dn11 = assign28130_e21868_d_n11;
        locals.var_t0__blk1144_dn12 = assign28130_e21868_d_n12;

        let (assign28140_e21883, assign28140_e21883_d_n3, assign28140_e21883_d_n4, assign28140_e21883_d_n5, assign28140_e21883_d_n6, assign28140_e21883_d_n7, assign28140_e21883_d_n8, assign28140_e21883_d_n9, assign28140_e21883_d_n10, assign28140_e21883_d_n11, assign28140_e21883_d_n12,) = {
    if ((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) {
        let assign28140_e21878: f64 = (locals.var_tempratio - 1.0);
        let assign28140_e21879: f64 = (locals.var_b4soitvbci * assign28140_e21878);
        let assign28140_e21880: f64 = (1.0 + assign28140_e21879);
        let assign28140_e21881: f64 = (locals.var_pparam_b4soivbci * assign28140_e21880);
        (assign28140_e21881, (locals.var_pparam_b4soivbci_dn3 * assign28140_e21880), (locals.var_pparam_b4soivbci_dn4 * assign28140_e21880), (locals.var_pparam_b4soivbci_dn5 * assign28140_e21880), ((locals.var_pparam_b4soivbci_dn6 * assign28140_e21880) + (locals.var_pparam_b4soivbci * (locals.var_b4soitvbci * locals.var_tempratio_dn6))), (locals.var_pparam_b4soivbci_dn7 * assign28140_e21880), (locals.var_pparam_b4soivbci_dn8 * assign28140_e21880), (locals.var_pparam_b4soivbci_dn9 * assign28140_e21880), (locals.var_pparam_b4soivbci_dn10 * assign28140_e21880), (locals.var_pparam_b4soivbci_dn11 * assign28140_e21880), (locals.var_pparam_b4soivbci_dn12 * assign28140_e21880),)
    } else {
        (locals.var_vbci, locals.var_vbci_dn3, locals.var_vbci_dn4, locals.var_vbci_dn5, locals.var_vbci_dn6, locals.var_vbci_dn7, locals.var_vbci_dn8, locals.var_vbci_dn9, locals.var_vbci_dn10, locals.var_vbci_dn11, locals.var_vbci_dn12,)
    }
};
        locals.var_vbci = assign28140_e21883;
        locals.var_vbci_dn3 = assign28140_e21883_d_n3;
        locals.var_vbci_dn4 = assign28140_e21883_d_n4;
        locals.var_vbci_dn5 = assign28140_e21883_d_n5;
        locals.var_vbci_dn6 = assign28140_e21883_d_n6;
        locals.var_vbci_dn7 = assign28140_e21883_d_n7;
        locals.var_vbci_dn8 = assign28140_e21883_d_n8;
        locals.var_vbci_dn9 = assign28140_e21883_d_n9;
        locals.var_vbci_dn10 = assign28140_e21883_d_n10;
        locals.var_vbci_dn11 = assign28140_e21883_d_n11;
        locals.var_vbci_dn12 = assign28140_e21883_d_n12;

        let assign28150_e21886: f64 = if locals.var_b4soimode > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1673 = assign28150_e21886;

        let (assign28160_e21897, assign28160_e21897_d_n3, assign28160_e21897_d_n4, assign28160_e21897_d_n5, assign28160_e21897_d_n6, assign28160_e21897_d_n7, assign28160_e21897_d_n8, assign28160_e21897_d_n9, assign28160_e21897_d_n10, assign28160_e21897_d_n11, assign28160_e21897_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1673 != 0.0)) {
        let assign28160_e21895: f64 = (locals.var_vbci - locals.var_vdbd);
        (assign28160_e21895, locals.var_vbci_dn3, locals.var_vbci_dn4, locals.var_vbci_dn5, locals.var_vbci_dn6, (locals.var_vbci_dn7 - locals.var_vdbd_dn7), locals.var_vbci_dn8, locals.var_vbci_dn9, locals.var_vbci_dn10, locals.var_vbci_dn11, (locals.var_vbci_dn12 - locals.var_vdbd_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign28160_e21897;
        locals.var_t1__blk1145_dn3 = assign28160_e21897_d_n3;
        locals.var_t1__blk1145_dn4 = assign28160_e21897_d_n4;
        locals.var_t1__blk1145_dn5 = assign28160_e21897_d_n5;
        locals.var_t1__blk1145_dn6 = assign28160_e21897_d_n6;
        locals.var_t1__blk1145_dn7 = assign28160_e21897_d_n7;
        locals.var_t1__blk1145_dn8 = assign28160_e21897_d_n8;
        locals.var_t1__blk1145_dn9 = assign28160_e21897_d_n9;
        locals.var_t1__blk1145_dn10 = assign28160_e21897_d_n10;
        locals.var_t1__blk1145_dn11 = assign28160_e21897_d_n11;
        locals.var_t1__blk1145_dn12 = assign28160_e21897_d_n12;

        let (assign28170_e21909, assign28170_e21909_d_n3, assign28170_e21909_d_n4, assign28170_e21909_d_n5, assign28170_e21909_d_n6, assign28170_e21909_d_n7, assign28170_e21909_d_n8, assign28170_e21909_d_n9, assign28170_e21909_d_n10, assign28170_e21909_d_n11, assign28170_e21909_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1673 == 0.0)) {
        let assign28170_e21907: f64 = (locals.var_vbci - locals.var_vsbs);
        (assign28170_e21907, locals.var_vbci_dn3, locals.var_vbci_dn4, locals.var_vbci_dn5, locals.var_vbci_dn6, locals.var_vbci_dn7, (locals.var_vbci_dn8 - locals.var_vsbs_dn8), locals.var_vbci_dn9, locals.var_vbci_dn10, (locals.var_vbci_dn11 - locals.var_vsbs_dn11), locals.var_vbci_dn12,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign28170_e21909;
        locals.var_t1__blk1145_dn3 = assign28170_e21909_d_n3;
        locals.var_t1__blk1145_dn4 = assign28170_e21909_d_n4;
        locals.var_t1__blk1145_dn5 = assign28170_e21909_d_n5;
        locals.var_t1__blk1145_dn6 = assign28170_e21909_d_n6;
        locals.var_t1__blk1145_dn7 = assign28170_e21909_d_n7;
        locals.var_t1__blk1145_dn8 = assign28170_e21909_d_n8;
        locals.var_t1__blk1145_dn9 = assign28170_e21909_d_n9;
        locals.var_t1__blk1145_dn10 = assign28170_e21909_d_n10;
        locals.var_t1__blk1145_dn11 = assign28170_e21909_d_n11;
        locals.var_t1__blk1145_dn12 = assign28170_e21909_d_n12;

        let (assign28180_e21918, assign28180_e21918_d_n3, assign28180_e21918_d_n4, assign28180_e21918_d_n5, assign28180_e21918_d_n6, assign28180_e21918_d_n7, assign28180_e21918_d_n8, assign28180_e21918_d_n9, assign28180_e21918_d_n10, assign28180_e21918_d_n11, assign28180_e21918_d_n12,) = {
    if ((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) {
        let assign28180_e21916: f64 = (locals.var_pparam_b4soimbjtii - 1.0);
        (assign28180_e21916, locals.var_pparam_b4soimbjtii_dn3, locals.var_pparam_b4soimbjtii_dn4, locals.var_pparam_b4soimbjtii_dn5, locals.var_pparam_b4soimbjtii_dn6, locals.var_pparam_b4soimbjtii_dn7, locals.var_pparam_b4soimbjtii_dn8, locals.var_pparam_b4soimbjtii_dn9, locals.var_pparam_b4soimbjtii_dn10, locals.var_pparam_b4soimbjtii_dn11, locals.var_pparam_b4soimbjtii_dn12,)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign28180_e21918;
        locals.var_t2__blk1146_dn3 = assign28180_e21918_d_n3;
        locals.var_t2__blk1146_dn4 = assign28180_e21918_d_n4;
        locals.var_t2__blk1146_dn5 = assign28180_e21918_d_n5;
        locals.var_t2__blk1146_dn6 = assign28180_e21918_d_n6;
        locals.var_t2__blk1146_dn7 = assign28180_e21918_d_n7;
        locals.var_t2__blk1146_dn8 = assign28180_e21918_d_n8;
        locals.var_t2__blk1146_dn9 = assign28180_e21918_d_n9;
        locals.var_t2__blk1146_dn10 = assign28180_e21918_d_n10;
        locals.var_t2__blk1146_dn11 = assign28180_e21918_d_n11;
        locals.var_t2__blk1146_dn12 = assign28180_e21918_d_n12;

        let assign28190_e21921: f64 = if locals.var_t1__blk1145 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1674 = assign28190_e21921;

        let (assign28200_e21930, assign28200_e21930_d_n3, assign28200_e21930_d_n4, assign28200_e21930_d_n5, assign28200_e21930_d_n6, assign28200_e21930_d_n7, assign28200_e21930_d_n8, assign28200_e21930_d_n9, assign28200_e21930_d_n10, assign28200_e21930_d_n11, assign28200_e21930_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1674 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign28200_e21930;
        locals.var_t3__blk1147_dn3 = assign28200_e21930_d_n3;
        locals.var_t3__blk1147_dn4 = assign28200_e21930_d_n4;
        locals.var_t3__blk1147_dn5 = assign28200_e21930_d_n5;
        locals.var_t3__blk1147_dn6 = assign28200_e21930_d_n6;
        locals.var_t3__blk1147_dn7 = assign28200_e21930_d_n7;
        locals.var_t3__blk1147_dn8 = assign28200_e21930_d_n8;
        locals.var_t3__blk1147_dn9 = assign28200_e21930_d_n9;
        locals.var_t3__blk1147_dn10 = assign28200_e21930_d_n10;
        locals.var_t3__blk1147_dn11 = assign28200_e21930_d_n11;
        locals.var_t3__blk1147_dn12 = assign28200_e21930_d_n12;

        let (assign28210_e21945, assign28210_e21945_d_n3, assign28210_e21945_d_n4, assign28210_e21945_d_n5, assign28210_e21945_d_n6, assign28210_e21945_d_n7, assign28210_e21945_d_n8, assign28210_e21945_d_n9, assign28210_e21945_d_n10, assign28210_e21945_d_n11, assign28210_e21945_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1674 == 0.0)) {
        let assign28210_e21939: f64 = (-locals.var_pparam_b4soiabjtii);
        let assign28210_e21942: f64 = (locals.var_t1__blk1145).powf(locals.var_t2__blk1146);
        let assign28210_e21943: f64 = (assign28210_e21939 * assign28210_e21942);
        (assign28210_e21943, (((-locals.var_pparam_b4soiabjtii_dn3) * assign28210_e21942) + (assign28210_e21939 * if locals.var_t2__blk1146_dn3 == 0.0 && ((locals.var_t2__blk1146) as f64).is_finite() && ((locals.var_t2__blk1146) as f64).fract() == 0.0 { if locals.var_t2__blk1146 == 0.0 { 0.0 } else { (locals.var_t2__blk1146 * ((locals.var_t1__blk1145).powf(locals.var_t2__blk1146 - 1.0) * locals.var_t1__blk1145_dn3)) } } else { (assign28210_e21942 * ((locals.var_t2__blk1146_dn3 * (locals.var_t1__blk1145).ln()) + (locals.var_t2__blk1146 * (locals.var_t1__blk1145_dn3 / locals.var_t1__blk1145)))) })), (((-locals.var_pparam_b4soiabjtii_dn4) * assign28210_e21942) + (assign28210_e21939 * if locals.var_t2__blk1146_dn4 == 0.0 && ((locals.var_t2__blk1146) as f64).is_finite() && ((locals.var_t2__blk1146) as f64).fract() == 0.0 { if locals.var_t2__blk1146 == 0.0 { 0.0 } else { (locals.var_t2__blk1146 * ((locals.var_t1__blk1145).powf(locals.var_t2__blk1146 - 1.0) * locals.var_t1__blk1145_dn4)) } } else { (assign28210_e21942 * ((locals.var_t2__blk1146_dn4 * (locals.var_t1__blk1145).ln()) + (locals.var_t2__blk1146 * (locals.var_t1__blk1145_dn4 / locals.var_t1__blk1145)))) })), (((-locals.var_pparam_b4soiabjtii_dn5) * assign28210_e21942) + (assign28210_e21939 * if locals.var_t2__blk1146_dn5 == 0.0 && ((locals.var_t2__blk1146) as f64).is_finite() && ((locals.var_t2__blk1146) as f64).fract() == 0.0 { if locals.var_t2__blk1146 == 0.0 { 0.0 } else { (locals.var_t2__blk1146 * ((locals.var_t1__blk1145).powf(locals.var_t2__blk1146 - 1.0) * locals.var_t1__blk1145_dn5)) } } else { (assign28210_e21942 * ((locals.var_t2__blk1146_dn5 * (locals.var_t1__blk1145).ln()) + (locals.var_t2__blk1146 * (locals.var_t1__blk1145_dn5 / locals.var_t1__blk1145)))) })), (((-locals.var_pparam_b4soiabjtii_dn6) * assign28210_e21942) + (assign28210_e21939 * if locals.var_t2__blk1146_dn6 == 0.0 && ((locals.var_t2__blk1146) as f64).is_finite() && ((locals.var_t2__blk1146) as f64).fract() == 0.0 { if locals.var_t2__blk1146 == 0.0 { 0.0 } else { (locals.var_t2__blk1146 * ((locals.var_t1__blk1145).powf(locals.var_t2__blk1146 - 1.0) * locals.var_t1__blk1145_dn6)) } } else { (assign28210_e21942 * ((locals.var_t2__blk1146_dn6 * (locals.var_t1__blk1145).ln()) + (locals.var_t2__blk1146 * (locals.var_t1__blk1145_dn6 / locals.var_t1__blk1145)))) })), (((-locals.var_pparam_b4soiabjtii_dn7) * assign28210_e21942) + (assign28210_e21939 * if locals.var_t2__blk1146_dn7 == 0.0 && ((locals.var_t2__blk1146) as f64).is_finite() && ((locals.var_t2__blk1146) as f64).fract() == 0.0 { if locals.var_t2__blk1146 == 0.0 { 0.0 } else { (locals.var_t2__blk1146 * ((locals.var_t1__blk1145).powf(locals.var_t2__blk1146 - 1.0) * locals.var_t1__blk1145_dn7)) } } else { (assign28210_e21942 * ((locals.var_t2__blk1146_dn7 * (locals.var_t1__blk1145).ln()) + (locals.var_t2__blk1146 * (locals.var_t1__blk1145_dn7 / locals.var_t1__blk1145)))) })), (((-locals.var_pparam_b4soiabjtii_dn8) * assign28210_e21942) + (assign28210_e21939 * if locals.var_t2__blk1146_dn8 == 0.0 && ((locals.var_t2__blk1146) as f64).is_finite() && ((locals.var_t2__blk1146) as f64).fract() == 0.0 { if locals.var_t2__blk1146 == 0.0 { 0.0 } else { (locals.var_t2__blk1146 * ((locals.var_t1__blk1145).powf(locals.var_t2__blk1146 - 1.0) * locals.var_t1__blk1145_dn8)) } } else { (assign28210_e21942 * ((locals.var_t2__blk1146_dn8 * (locals.var_t1__blk1145).ln()) + (locals.var_t2__blk1146 * (locals.var_t1__blk1145_dn8 / locals.var_t1__blk1145)))) })), (((-locals.var_pparam_b4soiabjtii_dn9) * assign28210_e21942) + (assign28210_e21939 * if locals.var_t2__blk1146_dn9 == 0.0 && ((locals.var_t2__blk1146) as f64).is_finite() && ((locals.var_t2__blk1146) as f64).fract() == 0.0 { if locals.var_t2__blk1146 == 0.0 { 0.0 } else { (locals.var_t2__blk1146 * ((locals.var_t1__blk1145).powf(locals.var_t2__blk1146 - 1.0) * locals.var_t1__blk1145_dn9)) } } else { (assign28210_e21942 * ((locals.var_t2__blk1146_dn9 * (locals.var_t1__blk1145).ln()) + (locals.var_t2__blk1146 * (locals.var_t1__blk1145_dn9 / locals.var_t1__blk1145)))) })), (((-locals.var_pparam_b4soiabjtii_dn10) * assign28210_e21942) + (assign28210_e21939 * if locals.var_t2__blk1146_dn10 == 0.0 && ((locals.var_t2__blk1146) as f64).is_finite() && ((locals.var_t2__blk1146) as f64).fract() == 0.0 { if locals.var_t2__blk1146 == 0.0 { 0.0 } else { (locals.var_t2__blk1146 * ((locals.var_t1__blk1145).powf(locals.var_t2__blk1146 - 1.0) * locals.var_t1__blk1145_dn10)) } } else { (assign28210_e21942 * ((locals.var_t2__blk1146_dn10 * (locals.var_t1__blk1145).ln()) + (locals.var_t2__blk1146 * (locals.var_t1__blk1145_dn10 / locals.var_t1__blk1145)))) })), (((-locals.var_pparam_b4soiabjtii_dn11) * assign28210_e21942) + (assign28210_e21939 * if locals.var_t2__blk1146_dn11 == 0.0 && ((locals.var_t2__blk1146) as f64).is_finite() && ((locals.var_t2__blk1146) as f64).fract() == 0.0 { if locals.var_t2__blk1146 == 0.0 { 0.0 } else { (locals.var_t2__blk1146 * ((locals.var_t1__blk1145).powf(locals.var_t2__blk1146 - 1.0) * locals.var_t1__blk1145_dn11)) } } else { (assign28210_e21942 * ((locals.var_t2__blk1146_dn11 * (locals.var_t1__blk1145).ln()) + (locals.var_t2__blk1146 * (locals.var_t1__blk1145_dn11 / locals.var_t1__blk1145)))) })), (((-locals.var_pparam_b4soiabjtii_dn12) * assign28210_e21942) + (assign28210_e21939 * if locals.var_t2__blk1146_dn12 == 0.0 && ((locals.var_t2__blk1146) as f64).is_finite() && ((locals.var_t2__blk1146) as f64).fract() == 0.0 { if locals.var_t2__blk1146 == 0.0 { 0.0 } else { (locals.var_t2__blk1146 * ((locals.var_t1__blk1145).powf(locals.var_t2__blk1146 - 1.0) * locals.var_t1__blk1145_dn12)) } } else { (assign28210_e21942 * ((locals.var_t2__blk1146_dn12 * (locals.var_t1__blk1145).ln()) + (locals.var_t2__blk1146 * (locals.var_t1__blk1145_dn12 / locals.var_t1__blk1145)))) })),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign28210_e21945;
        locals.var_t3__blk1147_dn3 = assign28210_e21945_d_n3;
        locals.var_t3__blk1147_dn4 = assign28210_e21945_d_n4;
        locals.var_t3__blk1147_dn5 = assign28210_e21945_d_n5;
        locals.var_t3__blk1147_dn6 = assign28210_e21945_d_n6;
        locals.var_t3__blk1147_dn7 = assign28210_e21945_d_n7;
        locals.var_t3__blk1147_dn8 = assign28210_e21945_d_n8;
        locals.var_t3__blk1147_dn9 = assign28210_e21945_d_n9;
        locals.var_t3__blk1147_dn10 = assign28210_e21945_d_n10;
        locals.var_t3__blk1147_dn11 = assign28210_e21945_d_n11;
        locals.var_t3__blk1147_dn12 = assign28210_e21945_d_n12;

        let assign28220_e21948: f64 = if locals.var_t3__blk1147 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1675 = assign28220_e21948;

        let (assign28230_e21957, assign28230_e21957_d_n3, assign28230_e21957_d_n4, assign28230_e21957_d_n5, assign28230_e21957_d_n6, assign28230_e21957_d_n7, assign28230_e21957_d_n8, assign28230_e21957_d_n9, assign28230_e21957_d_n10, assign28230_e21957_d_n11, assign28230_e21957_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1675 != 0.0)) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign28230_e21957;
        locals.var_t4__blk1148_dn3 = assign28230_e21957_d_n3;
        locals.var_t4__blk1148_dn4 = assign28230_e21957_d_n4;
        locals.var_t4__blk1148_dn5 = assign28230_e21957_d_n5;
        locals.var_t4__blk1148_dn6 = assign28230_e21957_d_n6;
        locals.var_t4__blk1148_dn7 = assign28230_e21957_d_n7;
        locals.var_t4__blk1148_dn8 = assign28230_e21957_d_n8;
        locals.var_t4__blk1148_dn9 = assign28230_e21957_d_n9;
        locals.var_t4__blk1148_dn10 = assign28230_e21957_d_n10;
        locals.var_t4__blk1148_dn11 = assign28230_e21957_d_n11;
        locals.var_t4__blk1148_dn12 = assign28230_e21957_d_n12;

        let assign28240_e21960: f64 = (-100.0);
        let assign28240_e21961: f64 = if locals.var_t3__blk1147 < assign28240_e21960 { 1.0 } else { 0.0 };
        locals.var_guard1676 = assign28240_e21961;

        let (assign28250_e21973, assign28250_e21973_d_n3, assign28250_e21973_d_n4, assign28250_e21973_d_n5, assign28250_e21973_d_n6, assign28250_e21973_d_n7, assign28250_e21973_d_n8, assign28250_e21973_d_n9, assign28250_e21973_d_n10, assign28250_e21973_d_n11, assign28250_e21973_d_n12,) = {
    if ((((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1675 == 0.0)) && (locals.var_guard1676 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign28250_e21973;
        locals.var_t4__blk1148_dn3 = assign28250_e21973_d_n3;
        locals.var_t4__blk1148_dn4 = assign28250_e21973_d_n4;
        locals.var_t4__blk1148_dn5 = assign28250_e21973_d_n5;
        locals.var_t4__blk1148_dn6 = assign28250_e21973_d_n6;
        locals.var_t4__blk1148_dn7 = assign28250_e21973_d_n7;
        locals.var_t4__blk1148_dn8 = assign28250_e21973_d_n8;
        locals.var_t4__blk1148_dn9 = assign28250_e21973_d_n9;
        locals.var_t4__blk1148_dn10 = assign28250_e21973_d_n10;
        locals.var_t4__blk1148_dn11 = assign28250_e21973_d_n11;
        locals.var_t4__blk1148_dn12 = assign28250_e21973_d_n12;

        let (assign28260_e21987, assign28260_e21987_d_n3, assign28260_e21987_d_n4, assign28260_e21987_d_n5, assign28260_e21987_d_n6, assign28260_e21987_d_n7, assign28260_e21987_d_n8, assign28260_e21987_d_n9, assign28260_e21987_d_n10, assign28260_e21987_d_n11, assign28260_e21987_d_n12,) = {
    if ((((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_guard1675 == 0.0)) && (locals.var_guard1676 == 0.0)) {
        let assign28260_e21985: f64 = (locals.var_t3__blk1147).exp();
        (assign28260_e21985, (assign28260_e21985 * locals.var_t3__blk1147_dn3), (assign28260_e21985 * locals.var_t3__blk1147_dn4), (assign28260_e21985 * locals.var_t3__blk1147_dn5), (assign28260_e21985 * locals.var_t3__blk1147_dn6), (assign28260_e21985 * locals.var_t3__blk1147_dn7), (assign28260_e21985 * locals.var_t3__blk1147_dn8), (assign28260_e21985 * locals.var_t3__blk1147_dn9), (assign28260_e21985 * locals.var_t3__blk1147_dn10), (assign28260_e21985 * locals.var_t3__blk1147_dn11), (assign28260_e21985 * locals.var_t3__blk1147_dn12),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign28260_e21987;
        locals.var_t4__blk1148_dn3 = assign28260_e21987_d_n3;
        locals.var_t4__blk1148_dn4 = assign28260_e21987_d_n4;
        locals.var_t4__blk1148_dn5 = assign28260_e21987_d_n5;
        locals.var_t4__blk1148_dn6 = assign28260_e21987_d_n6;
        locals.var_t4__blk1148_dn7 = assign28260_e21987_d_n7;
        locals.var_t4__blk1148_dn8 = assign28260_e21987_d_n8;
        locals.var_t4__blk1148_dn9 = assign28260_e21987_d_n9;
        locals.var_t4__blk1148_dn10 = assign28260_e21987_d_n10;
        locals.var_t4__blk1148_dn11 = assign28260_e21987_d_n11;
        locals.var_t4__blk1148_dn12 = assign28260_e21987_d_n12;

        let (assign28270_e22002, assign28270_e22002_d_n3, assign28270_e22002_d_n4, assign28270_e22002_d_n5, assign28270_e22002_d_n6, assign28270_e22002_d_n7, assign28270_e22002_d_n8, assign28270_e22002_d_n9, assign28270_e22002_d_n10, assign28270_e22002_d_n11, assign28270_e22002_d_n12,) = {
    if ((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) {
        let assign28270_e21994: f64 = (locals.var_t0__blk1144 * locals.var_b4soimode);
        let assign28270_e21996: f64 = (assign28270_e21994 * locals.var_ic_1);
        let assign28270_e21998: f64 = (assign28270_e21996 * locals.var_t1__blk1145);
        let assign28270_e22000: f64 = (assign28270_e21998 * locals.var_t4__blk1148);
        (assign28270_e22000, (((((((locals.var_t0__blk1144_dn3 * locals.var_b4soimode) * locals.var_ic_1) + (assign28270_e21994 * locals.var_ic_1_dn3)) * locals.var_t1__blk1145) + (assign28270_e21996 * locals.var_t1__blk1145_dn3)) * locals.var_t4__blk1148) + (assign28270_e21998 * locals.var_t4__blk1148_dn3)), (((((((locals.var_t0__blk1144_dn4 * locals.var_b4soimode) * locals.var_ic_1) + (assign28270_e21994 * locals.var_ic_1_dn4)) * locals.var_t1__blk1145) + (assign28270_e21996 * locals.var_t1__blk1145_dn4)) * locals.var_t4__blk1148) + (assign28270_e21998 * locals.var_t4__blk1148_dn4)), (((((((locals.var_t0__blk1144_dn5 * locals.var_b4soimode) * locals.var_ic_1) + (assign28270_e21994 * locals.var_ic_1_dn5)) * locals.var_t1__blk1145) + (assign28270_e21996 * locals.var_t1__blk1145_dn5)) * locals.var_t4__blk1148) + (assign28270_e21998 * locals.var_t4__blk1148_dn5)), (((((((locals.var_t0__blk1144_dn6 * locals.var_b4soimode) * locals.var_ic_1) + (assign28270_e21994 * locals.var_ic_1_dn6)) * locals.var_t1__blk1145) + (assign28270_e21996 * locals.var_t1__blk1145_dn6)) * locals.var_t4__blk1148) + (assign28270_e21998 * locals.var_t4__blk1148_dn6)), (((((((locals.var_t0__blk1144_dn7 * locals.var_b4soimode) * locals.var_ic_1) + (assign28270_e21994 * locals.var_ic_1_dn7)) * locals.var_t1__blk1145) + (assign28270_e21996 * locals.var_t1__blk1145_dn7)) * locals.var_t4__blk1148) + (assign28270_e21998 * locals.var_t4__blk1148_dn7)), (((((((locals.var_t0__blk1144_dn8 * locals.var_b4soimode) * locals.var_ic_1) + (assign28270_e21994 * locals.var_ic_1_dn8)) * locals.var_t1__blk1145) + (assign28270_e21996 * locals.var_t1__blk1145_dn8)) * locals.var_t4__blk1148) + (assign28270_e21998 * locals.var_t4__blk1148_dn8)), (((((((locals.var_t0__blk1144_dn9 * locals.var_b4soimode) * locals.var_ic_1) + (assign28270_e21994 * locals.var_ic_1_dn9)) * locals.var_t1__blk1145) + (assign28270_e21996 * locals.var_t1__blk1145_dn9)) * locals.var_t4__blk1148) + (assign28270_e21998 * locals.var_t4__blk1148_dn9)), (((((((locals.var_t0__blk1144_dn10 * locals.var_b4soimode) * locals.var_ic_1) + (assign28270_e21994 * locals.var_ic_1_dn10)) * locals.var_t1__blk1145) + (assign28270_e21996 * locals.var_t1__blk1145_dn10)) * locals.var_t4__blk1148) + (assign28270_e21998 * locals.var_t4__blk1148_dn10)), (((((((locals.var_t0__blk1144_dn11 * locals.var_b4soimode) * locals.var_ic_1) + (assign28270_e21994 * locals.var_ic_1_dn11)) * locals.var_t1__blk1145) + (assign28270_e21996 * locals.var_t1__blk1145_dn11)) * locals.var_t4__blk1148) + (assign28270_e21998 * locals.var_t4__blk1148_dn11)), (((((((locals.var_t0__blk1144_dn12 * locals.var_b4soimode) * locals.var_ic_1) + (assign28270_e21994 * locals.var_ic_1_dn12)) * locals.var_t1__blk1145) + (assign28270_e21996 * locals.var_t1__blk1145_dn12)) * locals.var_t4__blk1148) + (assign28270_e21998 * locals.var_t4__blk1148_dn12)),)
    } else {
        (locals.var_iiibjt, locals.var_iiibjt_dn3, locals.var_iiibjt_dn4, locals.var_iiibjt_dn5, locals.var_iiibjt_dn6, locals.var_iiibjt_dn7, locals.var_iiibjt_dn8, locals.var_iiibjt_dn9, locals.var_iiibjt_dn10, locals.var_iiibjt_dn11, locals.var_iiibjt_dn12,)
    }
};
        locals.var_iiibjt = assign28270_e22002;
        locals.var_iiibjt_dn3 = assign28270_e22002_d_n3;
        locals.var_iiibjt_dn4 = assign28270_e22002_d_n4;
        locals.var_iiibjt_dn5 = assign28270_e22002_d_n5;
        locals.var_iiibjt_dn6 = assign28270_e22002_d_n6;
        locals.var_iiibjt_dn7 = assign28270_e22002_d_n7;
        locals.var_iiibjt_dn8 = assign28270_e22002_d_n8;
        locals.var_iiibjt_dn9 = assign28270_e22002_d_n9;
        locals.var_iiibjt_dn10 = assign28270_e22002_d_n10;
        locals.var_iiibjt_dn11 = assign28270_e22002_d_n11;
        locals.var_iiibjt_dn12 = assign28270_e22002_d_n12;

        let (assign28280_e22011, assign28280_e22011_d_n3, assign28280_e22011_d_n4, assign28280_e22011_d_n5, assign28280_e22011_d_n6, assign28280_e22011_d_n7, assign28280_e22011_d_n8, assign28280_e22011_d_n9, assign28280_e22011_d_n10, assign28280_e22011_d_n11, assign28280_e22011_d_n12,) = {
    if ((locals.var_guard1661 != 0.0) && (locals.var_guard1662 == 0.0)) {
        let assign28280_e22009: f64 = (locals.var_idsmosfet + locals.var_iiibjt);
        (assign28280_e22009, (locals.var_idsmosfet_dn3 + locals.var_iiibjt_dn3), (locals.var_idsmosfet_dn4 + locals.var_iiibjt_dn4), (locals.var_idsmosfet_dn5 + locals.var_iiibjt_dn5), (locals.var_idsmosfet_dn6 + locals.var_iiibjt_dn6), (locals.var_idsmosfet_dn7 + locals.var_iiibjt_dn7), (locals.var_idsmosfet_dn8 + locals.var_iiibjt_dn8), (locals.var_idsmosfet_dn9 + locals.var_iiibjt_dn9), (locals.var_idsmosfet_dn10 + locals.var_iiibjt_dn10), (locals.var_idsmosfet_dn11 + locals.var_iiibjt_dn11), (locals.var_idsmosfet_dn12 + locals.var_iiibjt_dn12),)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11, locals.var_iii_dn12,)
    }
};
        locals.var_iii = assign28280_e22011;
        locals.var_iii_dn3 = assign28280_e22011_d_n3;
        locals.var_iii_dn4 = assign28280_e22011_d_n4;
        locals.var_iii_dn5 = assign28280_e22011_d_n5;
        locals.var_iii_dn6 = assign28280_e22011_d_n6;
        locals.var_iii_dn7 = assign28280_e22011_d_n7;
        locals.var_iii_dn8 = assign28280_e22011_d_n8;
        locals.var_iii_dn9 = assign28280_e22011_d_n9;
        locals.var_iii_dn10 = assign28280_e22011_d_n10;
        locals.var_iii_dn11 = assign28280_e22011_d_n11;
        locals.var_iii_dn12 = assign28280_e22011_d_n12;

        let assign28290_e22018: f64 = if ((locals.var_b4soibodymod == 0.0) || (locals.var_b4soibodymod == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard1677 = assign28290_e22018;

        let (assign28300_e22024, assign28300_e22024_d_n3, assign28300_e22024_d_n4, assign28300_e22024_d_n5, assign28300_e22024_d_n6, assign28300_e22024_d_n7, assign28300_e22024_d_n8, assign28300_e22024_d_n9, assign28300_e22024_d_n10, assign28300_e22024_d_n11, assign28300_e22024_d_n12,) = {
    if ((locals.var_guard1661 != 0.0) && (locals.var_guard1677 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibp, locals.var_ibp_dn3, locals.var_ibp_dn4, locals.var_ibp_dn5, locals.var_ibp_dn6, locals.var_ibp_dn7, locals.var_ibp_dn8, locals.var_ibp_dn9, locals.var_ibp_dn10, locals.var_ibp_dn11, locals.var_ibp_dn12,)
    }
};
        locals.var_ibp = assign28300_e22024;
        locals.var_ibp_dn3 = assign28300_e22024_d_n3;
        locals.var_ibp_dn4 = assign28300_e22024_d_n4;
        locals.var_ibp_dn5 = assign28300_e22024_d_n5;
        locals.var_ibp_dn6 = assign28300_e22024_d_n6;
        locals.var_ibp_dn7 = assign28300_e22024_d_n7;
        locals.var_ibp_dn8 = assign28300_e22024_d_n8;
        locals.var_ibp_dn9 = assign28300_e22024_d_n9;
        locals.var_ibp_dn10 = assign28300_e22024_d_n10;
        locals.var_ibp_dn11 = assign28300_e22024_d_n11;
        locals.var_ibp_dn12 = assign28300_e22024_d_n12;

        let assign28310_e22027: f64 = if locals.var_pparam_b4soirbody < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1678 = assign28310_e22027;

        let assign28320_e22030: f64 = if locals.var_b4soirbodyext <= 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1679 = assign28320_e22030;

        let (assign28330_e22043, assign28330_e22043_d_n3, assign28330_e22043_d_n4, assign28330_e22043_d_n5, assign28330_e22043_d_n6, assign28330_e22043_d_n7, assign28330_e22043_d_n8, assign28330_e22043_d_n9, assign28330_e22043_d_n10, assign28330_e22043_d_n11, assign28330_e22043_d_n12,) = {
    if ((((locals.var_guard1661 != 0.0) && (locals.var_guard1677 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1679 != 0.0)) {
        let assign28330_e22041: f64 = (1.0 / 0.001);
        (assign28330_e22041, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign28330_e22043;
        locals.var_t0__blk1144_dn3 = assign28330_e22043_d_n3;
        locals.var_t0__blk1144_dn4 = assign28330_e22043_d_n4;
        locals.var_t0__blk1144_dn5 = assign28330_e22043_d_n5;
        locals.var_t0__blk1144_dn6 = assign28330_e22043_d_n6;
        locals.var_t0__blk1144_dn7 = assign28330_e22043_d_n7;
        locals.var_t0__blk1144_dn8 = assign28330_e22043_d_n8;
        locals.var_t0__blk1144_dn9 = assign28330_e22043_d_n9;
        locals.var_t0__blk1144_dn10 = assign28330_e22043_d_n10;
        locals.var_t0__blk1144_dn11 = assign28330_e22043_d_n11;
        locals.var_t0__blk1144_dn12 = assign28330_e22043_d_n12;

        let (assign28340_e22057, assign28340_e22057_d_n3, assign28340_e22057_d_n4, assign28340_e22057_d_n5, assign28340_e22057_d_n6, assign28340_e22057_d_n7, assign28340_e22057_d_n8, assign28340_e22057_d_n9, assign28340_e22057_d_n10, assign28340_e22057_d_n11, assign28340_e22057_d_n12,) = {
    if ((((locals.var_guard1661 != 0.0) && (locals.var_guard1677 == 0.0)) && (locals.var_guard1678 != 0.0)) && (locals.var_guard1679 == 0.0)) {
        let assign28340_e22055: f64 = (1.0 / locals.var_b4soirbodyext);
        (assign28340_e22055, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign28340_e22057;
        locals.var_t0__blk1144_dn3 = assign28340_e22057_d_n3;
        locals.var_t0__blk1144_dn4 = assign28340_e22057_d_n4;
        locals.var_t0__blk1144_dn5 = assign28340_e22057_d_n5;
        locals.var_t0__blk1144_dn6 = assign28340_e22057_d_n6;
        locals.var_t0__blk1144_dn7 = assign28340_e22057_d_n7;
        locals.var_t0__blk1144_dn8 = assign28340_e22057_d_n8;
        locals.var_t0__blk1144_dn9 = assign28340_e22057_d_n9;
        locals.var_t0__blk1144_dn10 = assign28340_e22057_d_n10;
        locals.var_t0__blk1144_dn11 = assign28340_e22057_d_n11;
        locals.var_t0__blk1144_dn12 = assign28340_e22057_d_n12;

    }

    pub(super) fn stamp_transient_block_75(
        locals: &mut StampLocals,
    ) {
        let (assign28350_e22068, assign28350_e22068_d_n3, assign28350_e22068_d_n4, assign28350_e22068_d_n5, assign28350_e22068_d_n6, assign28350_e22068_d_n7, assign28350_e22068_d_n8, assign28350_e22068_d_n9, assign28350_e22068_d_n10, assign28350_e22068_d_n11, assign28350_e22068_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1677 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        let assign28350_e22066: f64 = (locals.var_vbp * locals.var_t0__blk1144);
        (assign28350_e22066, (locals.var_vbp * locals.var_t0__blk1144_dn3), ((locals.var_vbp_dn4 * locals.var_t0__blk1144) + (locals.var_vbp * locals.var_t0__blk1144_dn4)), ((locals.var_vbp_dn5 * locals.var_t0__blk1144) + (locals.var_vbp * locals.var_t0__blk1144_dn5)), (locals.var_vbp * locals.var_t0__blk1144_dn6), (locals.var_vbp * locals.var_t0__blk1144_dn7), (locals.var_vbp * locals.var_t0__blk1144_dn8), (locals.var_vbp * locals.var_t0__blk1144_dn9), (locals.var_vbp * locals.var_t0__blk1144_dn10), (locals.var_vbp * locals.var_t0__blk1144_dn11), (locals.var_vbp * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_ibp, locals.var_ibp_dn3, locals.var_ibp_dn4, locals.var_ibp_dn5, locals.var_ibp_dn6, locals.var_ibp_dn7, locals.var_ibp_dn8, locals.var_ibp_dn9, locals.var_ibp_dn10, locals.var_ibp_dn11, locals.var_ibp_dn12,)
    }
};
        locals.var_ibp = assign28350_e22068;
        locals.var_ibp_dn3 = assign28350_e22068_d_n3;
        locals.var_ibp_dn4 = assign28350_e22068_d_n4;
        locals.var_ibp_dn5 = assign28350_e22068_d_n5;
        locals.var_ibp_dn6 = assign28350_e22068_d_n6;
        locals.var_ibp_dn7 = assign28350_e22068_d_n7;
        locals.var_ibp_dn8 = assign28350_e22068_d_n8;
        locals.var_ibp_dn9 = assign28350_e22068_d_n9;
        locals.var_ibp_dn10 = assign28350_e22068_d_n10;
        locals.var_ibp_dn11 = assign28350_e22068_d_n11;
        locals.var_ibp_dn12 = assign28350_e22068_d_n12;

        let (assign28360_e22082, assign28360_e22082_d_n3, assign28360_e22082_d_n4, assign28360_e22082_d_n5, assign28360_e22082_d_n6, assign28360_e22082_d_n7, assign28360_e22082_d_n8, assign28360_e22082_d_n9, assign28360_e22082_d_n10, assign28360_e22082_d_n11, assign28360_e22082_d_n12,) = {
    if (((locals.var_guard1661 != 0.0) && (locals.var_guard1677 == 0.0)) && (locals.var_guard1678 == 0.0)) {
        let assign28360_e22079: f64 = (locals.var_pparam_b4soirbody + locals.var_b4soirbodyext);
        let assign28360_e22080: f64 = (locals.var_vbp / assign28360_e22079);
        (assign28360_e22080, (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn3) / (assign28360_e22079 * assign28360_e22079))), (((locals.var_vbp_dn4 * assign28360_e22079) - (locals.var_vbp * locals.var_pparam_b4soirbody_dn4)) / (assign28360_e22079 * assign28360_e22079)), (((locals.var_vbp_dn5 * assign28360_e22079) - (locals.var_vbp * locals.var_pparam_b4soirbody_dn5)) / (assign28360_e22079 * assign28360_e22079)), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn6) / (assign28360_e22079 * assign28360_e22079))), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn7) / (assign28360_e22079 * assign28360_e22079))), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn8) / (assign28360_e22079 * assign28360_e22079))), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn9) / (assign28360_e22079 * assign28360_e22079))), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn10) / (assign28360_e22079 * assign28360_e22079))), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn11) / (assign28360_e22079 * assign28360_e22079))), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn12) / (assign28360_e22079 * assign28360_e22079))),)
    } else {
        (locals.var_ibp, locals.var_ibp_dn3, locals.var_ibp_dn4, locals.var_ibp_dn5, locals.var_ibp_dn6, locals.var_ibp_dn7, locals.var_ibp_dn8, locals.var_ibp_dn9, locals.var_ibp_dn10, locals.var_ibp_dn11, locals.var_ibp_dn12,)
    }
};
        locals.var_ibp = assign28360_e22082;
        locals.var_ibp_dn3 = assign28360_e22082_d_n3;
        locals.var_ibp_dn4 = assign28360_e22082_d_n4;
        locals.var_ibp_dn5 = assign28360_e22082_d_n5;
        locals.var_ibp_dn6 = assign28360_e22082_d_n6;
        locals.var_ibp_dn7 = assign28360_e22082_d_n7;
        locals.var_ibp_dn8 = assign28360_e22082_d_n8;
        locals.var_ibp_dn9 = assign28360_e22082_d_n9;
        locals.var_ibp_dn10 = assign28360_e22082_d_n10;
        locals.var_ibp_dn11 = assign28360_e22082_d_n11;
        locals.var_ibp_dn12 = assign28360_e22082_d_n12;

        let (assign28370_e22087, assign28370_e22087_d_n3, assign28370_e22087_d_n4, assign28370_e22087_d_n5, assign28370_e22087_d_n6, assign28370_e22087_d_n7, assign28370_e22087_d_n8, assign28370_e22087_d_n9, assign28370_e22087_d_n10, assign28370_e22087_d_n11, assign28370_e22087_d_n12,) = {
    if (locals.var_guard1661 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11, locals.var_iii_dn12,)
    }
};
        locals.var_iii = assign28370_e22087;
        locals.var_iii_dn3 = assign28370_e22087_d_n3;
        locals.var_iii_dn4 = assign28370_e22087_d_n4;
        locals.var_iii_dn5 = assign28370_e22087_d_n5;
        locals.var_iii_dn6 = assign28370_e22087_d_n6;
        locals.var_iii_dn7 = assign28370_e22087_d_n7;
        locals.var_iii_dn8 = assign28370_e22087_d_n8;
        locals.var_iii_dn9 = assign28370_e22087_d_n9;
        locals.var_iii_dn10 = assign28370_e22087_d_n10;
        locals.var_iii_dn11 = assign28370_e22087_d_n11;
        locals.var_iii_dn12 = assign28370_e22087_d_n12;

        let (assign28380_e22092, assign28380_e22092_d_n3, assign28380_e22092_d_n4, assign28380_e22092_d_n5, assign28380_e22092_d_n6, assign28380_e22092_d_n7, assign28380_e22092_d_n8, assign28380_e22092_d_n9, assign28380_e22092_d_n10, assign28380_e22092_d_n11, assign28380_e22092_d_n12,) = {
    if (locals.var_guard1661 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibp, locals.var_ibp_dn3, locals.var_ibp_dn4, locals.var_ibp_dn5, locals.var_ibp_dn6, locals.var_ibp_dn7, locals.var_ibp_dn8, locals.var_ibp_dn9, locals.var_ibp_dn10, locals.var_ibp_dn11, locals.var_ibp_dn12,)
    }
};
        locals.var_ibp = assign28380_e22092;
        locals.var_ibp_dn3 = assign28380_e22092_d_n3;
        locals.var_ibp_dn4 = assign28380_e22092_d_n4;
        locals.var_ibp_dn5 = assign28380_e22092_d_n5;
        locals.var_ibp_dn6 = assign28380_e22092_d_n6;
        locals.var_ibp_dn7 = assign28380_e22092_d_n7;
        locals.var_ibp_dn8 = assign28380_e22092_d_n8;
        locals.var_ibp_dn9 = assign28380_e22092_d_n9;
        locals.var_ibp_dn10 = assign28380_e22092_d_n10;
        locals.var_ibp_dn11 = assign28380_e22092_d_n11;
        locals.var_ibp_dn12 = assign28380_e22092_d_n12;

        let assign28390_e22095: f64 = if locals.var_b4soirgatemod > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1680 = assign28390_e22095;

        let (assign28400_e22101, assign28400_e22101_d_n3, assign28400_e22101_d_n4, assign28400_e22101_d_n5, assign28400_e22101_d_n6, assign28400_e22101_d_n7, assign28400_e22101_d_n8, assign28400_e22101_d_n9, assign28400_e22101_d_n10, assign28400_e22101_d_n11, assign28400_e22101_d_n12,) = {
    if (locals.var_guard1680 != 0.0) {
        let assign28400_e22099: f64 = (locals.var_pparam_b4soixrcrg2 * locals.var_b4soivtm);
        (assign28400_e22099, (locals.var_pparam_b4soixrcrg2_dn3 * locals.var_b4soivtm), (locals.var_pparam_b4soixrcrg2_dn4 * locals.var_b4soivtm), (locals.var_pparam_b4soixrcrg2_dn5 * locals.var_b4soivtm), ((locals.var_pparam_b4soixrcrg2_dn6 * locals.var_b4soivtm) + (locals.var_pparam_b4soixrcrg2 * locals.var_b4soivtm_dn6)), (locals.var_pparam_b4soixrcrg2_dn7 * locals.var_b4soivtm), (locals.var_pparam_b4soixrcrg2_dn8 * locals.var_b4soivtm), (locals.var_pparam_b4soixrcrg2_dn9 * locals.var_b4soivtm), (locals.var_pparam_b4soixrcrg2_dn10 * locals.var_b4soivtm), (locals.var_pparam_b4soixrcrg2_dn11 * locals.var_b4soivtm), (locals.var_pparam_b4soixrcrg2_dn12 * locals.var_b4soivtm),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign28400_e22101;
        locals.var_t9_dn3 = assign28400_e22101_d_n3;
        locals.var_t9_dn4 = assign28400_e22101_d_n4;
        locals.var_t9_dn5 = assign28400_e22101_d_n5;
        locals.var_t9_dn6 = assign28400_e22101_d_n6;
        locals.var_t9_dn7 = assign28400_e22101_d_n7;
        locals.var_t9_dn8 = assign28400_e22101_d_n8;
        locals.var_t9_dn9 = assign28400_e22101_d_n9;
        locals.var_t9_dn10 = assign28400_e22101_d_n10;
        locals.var_t9_dn11 = assign28400_e22101_d_n11;
        locals.var_t9_dn12 = assign28400_e22101_d_n12;

        let (assign28410_e22107, assign28410_e22107_d_n3, assign28410_e22107_d_n4, assign28410_e22107_d_n5, assign28410_e22107_d_n6, assign28410_e22107_d_n7, assign28410_e22107_d_n8, assign28410_e22107_d_n9, assign28410_e22107_d_n10, assign28410_e22107_d_n11, assign28410_e22107_d_n12,) = {
    if (locals.var_guard1680 != 0.0) {
        let assign28410_e22105: f64 = (locals.var_t9 * locals.var_beta);
        (assign28410_e22105, ((locals.var_t9_dn3 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn3)), ((locals.var_t9_dn4 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn4)), ((locals.var_t9_dn5 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn5)), ((locals.var_t9_dn6 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn6)), ((locals.var_t9_dn7 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn7)), ((locals.var_t9_dn8 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn8)), ((locals.var_t9_dn9 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn9)), ((locals.var_t9_dn10 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn10)), ((locals.var_t9_dn11 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn11)), ((locals.var_t9_dn12 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign28410_e22107;
        locals.var_t0__blk1144_dn3 = assign28410_e22107_d_n3;
        locals.var_t0__blk1144_dn4 = assign28410_e22107_d_n4;
        locals.var_t0__blk1144_dn5 = assign28410_e22107_d_n5;
        locals.var_t0__blk1144_dn6 = assign28410_e22107_d_n6;
        locals.var_t0__blk1144_dn7 = assign28410_e22107_d_n7;
        locals.var_t0__blk1144_dn8 = assign28410_e22107_d_n8;
        locals.var_t0__blk1144_dn9 = assign28410_e22107_d_n9;
        locals.var_t0__blk1144_dn10 = assign28410_e22107_d_n10;
        locals.var_t0__blk1144_dn11 = assign28410_e22107_d_n11;
        locals.var_t0__blk1144_dn12 = assign28410_e22107_d_n12;

        let (assign28420_e22115, assign28420_e22115_d_n3, assign28420_e22115_d_n4, assign28420_e22115_d_n5, assign28420_e22115_d_n6, assign28420_e22115_d_n7, assign28420_e22115_d_n8, assign28420_e22115_d_n9, assign28420_e22115_d_n10, assign28420_e22115_d_n11, assign28420_e22115_d_n12,) = {
    if (locals.var_guard1680 != 0.0) {
        let assign28420_e22112: f64 = (locals.var_t0__blk1144 + locals.var_idovvds);
        let assign28420_e22113: f64 = (locals.var_pparam_b4soixrcrg1 * assign28420_e22112);
        (assign28420_e22113, ((locals.var_pparam_b4soixrcrg1_dn3 * assign28420_e22112) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk1144_dn3 + locals.var_idovvds_dn3))), ((locals.var_pparam_b4soixrcrg1_dn4 * assign28420_e22112) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk1144_dn4 + locals.var_idovvds_dn4))), ((locals.var_pparam_b4soixrcrg1_dn5 * assign28420_e22112) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk1144_dn5 + locals.var_idovvds_dn5))), ((locals.var_pparam_b4soixrcrg1_dn6 * assign28420_e22112) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk1144_dn6 + locals.var_idovvds_dn6))), ((locals.var_pparam_b4soixrcrg1_dn7 * assign28420_e22112) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk1144_dn7 + locals.var_idovvds_dn7))), ((locals.var_pparam_b4soixrcrg1_dn8 * assign28420_e22112) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk1144_dn8 + locals.var_idovvds_dn8))), ((locals.var_pparam_b4soixrcrg1_dn9 * assign28420_e22112) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk1144_dn9 + locals.var_idovvds_dn9))), ((locals.var_pparam_b4soixrcrg1_dn10 * assign28420_e22112) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk1144_dn10 + locals.var_idovvds_dn10))), ((locals.var_pparam_b4soixrcrg1_dn11 * assign28420_e22112) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk1144_dn11 + locals.var_idovvds_dn11))), ((locals.var_pparam_b4soixrcrg1_dn12 * assign28420_e22112) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk1144_dn12 + locals.var_idovvds_dn12))),)
    } else {
        (locals.var_b4soigcrg, locals.var_b4soigcrg_dn3, locals.var_b4soigcrg_dn4, locals.var_b4soigcrg_dn5, locals.var_b4soigcrg_dn6, locals.var_b4soigcrg_dn7, locals.var_b4soigcrg_dn8, locals.var_b4soigcrg_dn9, locals.var_b4soigcrg_dn10, locals.var_b4soigcrg_dn11, locals.var_b4soigcrg_dn12,)
    }
};
        locals.var_b4soigcrg = assign28420_e22115;
        locals.var_b4soigcrg_dn3 = assign28420_e22115_d_n3;
        locals.var_b4soigcrg_dn4 = assign28420_e22115_d_n4;
        locals.var_b4soigcrg_dn5 = assign28420_e22115_d_n5;
        locals.var_b4soigcrg_dn6 = assign28420_e22115_d_n6;
        locals.var_b4soigcrg_dn7 = assign28420_e22115_d_n7;
        locals.var_b4soigcrg_dn8 = assign28420_e22115_d_n8;
        locals.var_b4soigcrg_dn9 = assign28420_e22115_d_n9;
        locals.var_b4soigcrg_dn10 = assign28420_e22115_d_n10;
        locals.var_b4soigcrg_dn11 = assign28420_e22115_d_n11;
        locals.var_b4soigcrg_dn12 = assign28420_e22115_d_n12;

        let assign28430_e22118: f64 = if locals.var_b4soinf != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1681 = assign28430_e22118;

        let (assign28440_e22126, assign28440_e22126_d_n3, assign28440_e22126_d_n4, assign28440_e22126_d_n5, assign28440_e22126_d_n6, assign28440_e22126_d_n7, assign28440_e22126_d_n8, assign28440_e22126_d_n9, assign28440_e22126_d_n10, assign28440_e22126_d_n11, assign28440_e22126_d_n12,) = {
    if ((locals.var_guard1680 != 0.0) && (locals.var_guard1681 != 0.0)) {
        let assign28440_e22124: f64 = (locals.var_b4soigcrg * locals.var_b4soinf);
        (assign28440_e22124, (locals.var_b4soigcrg_dn3 * locals.var_b4soinf), (locals.var_b4soigcrg_dn4 * locals.var_b4soinf), (locals.var_b4soigcrg_dn5 * locals.var_b4soinf), (locals.var_b4soigcrg_dn6 * locals.var_b4soinf), (locals.var_b4soigcrg_dn7 * locals.var_b4soinf), (locals.var_b4soigcrg_dn8 * locals.var_b4soinf), (locals.var_b4soigcrg_dn9 * locals.var_b4soinf), (locals.var_b4soigcrg_dn10 * locals.var_b4soinf), (locals.var_b4soigcrg_dn11 * locals.var_b4soinf), (locals.var_b4soigcrg_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_b4soigcrg, locals.var_b4soigcrg_dn3, locals.var_b4soigcrg_dn4, locals.var_b4soigcrg_dn5, locals.var_b4soigcrg_dn6, locals.var_b4soigcrg_dn7, locals.var_b4soigcrg_dn8, locals.var_b4soigcrg_dn9, locals.var_b4soigcrg_dn10, locals.var_b4soigcrg_dn11, locals.var_b4soigcrg_dn12,)
    }
};
        locals.var_b4soigcrg = assign28440_e22126;
        locals.var_b4soigcrg_dn3 = assign28440_e22126_d_n3;
        locals.var_b4soigcrg_dn4 = assign28440_e22126_d_n4;
        locals.var_b4soigcrg_dn5 = assign28440_e22126_d_n5;
        locals.var_b4soigcrg_dn6 = assign28440_e22126_d_n6;
        locals.var_b4soigcrg_dn7 = assign28440_e22126_d_n7;
        locals.var_b4soigcrg_dn8 = assign28440_e22126_d_n8;
        locals.var_b4soigcrg_dn9 = assign28440_e22126_d_n9;
        locals.var_b4soigcrg_dn10 = assign28440_e22126_d_n10;
        locals.var_b4soigcrg_dn11 = assign28440_e22126_d_n11;
        locals.var_b4soigcrg_dn12 = assign28440_e22126_d_n12;

        let assign28450_e22129: f64 = if locals.var_b4soirgatemod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1682 = assign28450_e22129;

        let (assign28460_e22137, assign28460_e22137_d_n3, assign28460_e22137_d_n4, assign28460_e22137_d_n5, assign28460_e22137_d_n6, assign28460_e22137_d_n7, assign28460_e22137_d_n8, assign28460_e22137_d_n9, assign28460_e22137_d_n10, assign28460_e22137_d_n11, assign28460_e22137_d_n12,) = {
    if ((locals.var_guard1680 != 0.0) && (locals.var_guard1682 != 0.0)) {
        let assign28460_e22135: f64 = (locals.var_b4soigrgeltd + locals.var_b4soigcrg);
        (assign28460_e22135, (locals.var_b4soigrgeltd_dn3 + locals.var_b4soigcrg_dn3), (locals.var_b4soigrgeltd_dn4 + locals.var_b4soigcrg_dn4), (locals.var_b4soigrgeltd_dn5 + locals.var_b4soigcrg_dn5), (locals.var_b4soigrgeltd_dn6 + locals.var_b4soigcrg_dn6), (locals.var_b4soigrgeltd_dn7 + locals.var_b4soigcrg_dn7), (locals.var_b4soigrgeltd_dn8 + locals.var_b4soigcrg_dn8), (locals.var_b4soigrgeltd_dn9 + locals.var_b4soigcrg_dn9), (locals.var_b4soigrgeltd_dn10 + locals.var_b4soigcrg_dn10), (locals.var_b4soigrgeltd_dn11 + locals.var_b4soigcrg_dn11), (locals.var_b4soigrgeltd_dn12 + locals.var_b4soigcrg_dn12),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign28460_e22137;
        locals.var_t11_dn3 = assign28460_e22137_d_n3;
        locals.var_t11_dn4 = assign28460_e22137_d_n4;
        locals.var_t11_dn5 = assign28460_e22137_d_n5;
        locals.var_t11_dn6 = assign28460_e22137_d_n6;
        locals.var_t11_dn7 = assign28460_e22137_d_n7;
        locals.var_t11_dn8 = assign28460_e22137_d_n8;
        locals.var_t11_dn9 = assign28460_e22137_d_n9;
        locals.var_t11_dn10 = assign28460_e22137_d_n10;
        locals.var_t11_dn11 = assign28460_e22137_d_n11;
        locals.var_t11_dn12 = assign28460_e22137_d_n12;

        let (assign28470_e22147, assign28470_e22147_d_n3, assign28470_e22147_d_n4, assign28470_e22147_d_n5, assign28470_e22147_d_n6, assign28470_e22147_d_n7, assign28470_e22147_d_n8, assign28470_e22147_d_n9, assign28470_e22147_d_n10, assign28470_e22147_d_n11, assign28470_e22147_d_n12,) = {
    if ((locals.var_guard1680 != 0.0) && (locals.var_guard1682 != 0.0)) {
        let assign28470_e22143: f64 = (locals.var_b4soigrgeltd * locals.var_b4soigcrg);
        let assign28470_e22145: f64 = (assign28470_e22143 / locals.var_t11);
        (assign28470_e22145, (((((locals.var_b4soigrgeltd_dn3 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn3)) * locals.var_t11) - (assign28470_e22143 * locals.var_t11_dn3)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn4 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn4)) * locals.var_t11) - (assign28470_e22143 * locals.var_t11_dn4)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn5 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn5)) * locals.var_t11) - (assign28470_e22143 * locals.var_t11_dn5)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn6 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn6)) * locals.var_t11) - (assign28470_e22143 * locals.var_t11_dn6)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn7 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn7)) * locals.var_t11) - (assign28470_e22143 * locals.var_t11_dn7)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn8 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn8)) * locals.var_t11) - (assign28470_e22143 * locals.var_t11_dn8)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn9 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn9)) * locals.var_t11) - (assign28470_e22143 * locals.var_t11_dn9)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn10 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn10)) * locals.var_t11) - (assign28470_e22143 * locals.var_t11_dn10)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn11 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn11)) * locals.var_t11) - (assign28470_e22143 * locals.var_t11_dn11)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn12 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn12)) * locals.var_t11) - (assign28470_e22143 * locals.var_t11_dn12)) / (locals.var_t11 * locals.var_t11)),)
    } else {
        (locals.var_b4soigcrg, locals.var_b4soigcrg_dn3, locals.var_b4soigcrg_dn4, locals.var_b4soigcrg_dn5, locals.var_b4soigcrg_dn6, locals.var_b4soigcrg_dn7, locals.var_b4soigcrg_dn8, locals.var_b4soigcrg_dn9, locals.var_b4soigcrg_dn10, locals.var_b4soigcrg_dn11, locals.var_b4soigcrg_dn12,)
    }
};
        locals.var_b4soigcrg = assign28470_e22147;
        locals.var_b4soigcrg_dn3 = assign28470_e22147_d_n3;
        locals.var_b4soigcrg_dn4 = assign28470_e22147_d_n4;
        locals.var_b4soigcrg_dn5 = assign28470_e22147_d_n5;
        locals.var_b4soigcrg_dn6 = assign28470_e22147_d_n6;
        locals.var_b4soigcrg_dn7 = assign28470_e22147_d_n7;
        locals.var_b4soigcrg_dn8 = assign28470_e22147_d_n8;
        locals.var_b4soigcrg_dn9 = assign28470_e22147_d_n9;
        locals.var_b4soigcrg_dn10 = assign28470_e22147_d_n10;
        locals.var_b4soigcrg_dn11 = assign28470_e22147_d_n11;
        locals.var_b4soigcrg_dn12 = assign28470_e22147_d_n12;

        let (assign28480_e22152, assign28480_e22152_d_n3, assign28480_e22152_d_n4, assign28480_e22152_d_n5, assign28480_e22152_d_n6, assign28480_e22152_d_n7, assign28480_e22152_d_n8, assign28480_e22152_d_n9, assign28480_e22152_d_n10, assign28480_e22152_d_n11, assign28480_e22152_d_n12,) = {
    if (locals.var_guard1680 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soigcrg, locals.var_b4soigcrg_dn3, locals.var_b4soigcrg_dn4, locals.var_b4soigcrg_dn5, locals.var_b4soigcrg_dn6, locals.var_b4soigcrg_dn7, locals.var_b4soigcrg_dn8, locals.var_b4soigcrg_dn9, locals.var_b4soigcrg_dn10, locals.var_b4soigcrg_dn11, locals.var_b4soigcrg_dn12,)
    }
};
        locals.var_b4soigcrg = assign28480_e22152;
        locals.var_b4soigcrg_dn3 = assign28480_e22152_d_n3;
        locals.var_b4soigcrg_dn4 = assign28480_e22152_d_n4;
        locals.var_b4soigcrg_dn5 = assign28480_e22152_d_n5;
        locals.var_b4soigcrg_dn6 = assign28480_e22152_d_n6;
        locals.var_b4soigcrg_dn7 = assign28480_e22152_d_n7;
        locals.var_b4soigcrg_dn8 = assign28480_e22152_d_n8;
        locals.var_b4soigcrg_dn9 = assign28480_e22152_d_n9;
        locals.var_b4soigcrg_dn10 = assign28480_e22152_d_n10;
        locals.var_b4soigcrg_dn11 = assign28480_e22152_d_n11;
        locals.var_b4soigcrg_dn12 = assign28480_e22152_d_n12;

        let assign28490_e22155: f64 = if locals.var_b4soirdsmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1683 = assign28490_e22155;

        let (assign28500_e22159, assign28500_e22159_d_n3, assign28500_e22159_d_n4, assign28500_e22159_d_n5, assign28500_e22159_d_n6, assign28500_e22159_d_n7, assign28500_e22159_d_n8, assign28500_e22159_d_n9, assign28500_e22159_d_n10, assign28500_e22159_d_n11, assign28500_e22159_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rds, locals.var_rds_dn3, locals.var_rds_dn4, locals.var_rds_dn5, locals.var_rds_dn6, locals.var_rds_dn7, locals.var_rds_dn8, locals.var_rds_dn9, locals.var_rds_dn10, locals.var_rds_dn11, locals.var_rds_dn12,)
    }
};
        locals.var_rds = assign28500_e22159;
        locals.var_rds_dn3 = assign28500_e22159_d_n3;
        locals.var_rds_dn4 = assign28500_e22159_d_n4;
        locals.var_rds_dn5 = assign28500_e22159_d_n5;
        locals.var_rds_dn6 = assign28500_e22159_d_n6;
        locals.var_rds_dn7 = assign28500_e22159_d_n7;
        locals.var_rds_dn8 = assign28500_e22159_d_n8;
        locals.var_rds_dn9 = assign28500_e22159_d_n9;
        locals.var_rds_dn10 = assign28500_e22159_d_n10;
        locals.var_rds_dn11 = assign28500_e22159_d_n11;
        locals.var_rds_dn12 = assign28500_e22159_d_n12;

        let (assign28510_e22165, assign28510_e22165_d_n3, assign28510_e22165_d_n4, assign28510_e22165_d_n5, assign28510_e22165_d_n6, assign28510_e22165_d_n7, assign28510_e22165_d_n8, assign28510_e22165_d_n9, assign28510_e22165_d_n10, assign28510_e22165_d_n11, assign28510_e22165_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28510_e22163: f64 = (locals.var_vgs - locals.var_pparam_b4soivfbsd);
        (assign28510_e22163, (-locals.var_pparam_b4soivfbsd_dn3), (-locals.var_pparam_b4soivfbsd_dn4), (-locals.var_pparam_b4soivfbsd_dn5), (-locals.var_pparam_b4soivfbsd_dn6), (-locals.var_pparam_b4soivfbsd_dn7), (locals.var_vgs_dn8 - locals.var_pparam_b4soivfbsd_dn8), (locals.var_vgs_dn9 - locals.var_pparam_b4soivfbsd_dn9), (-locals.var_pparam_b4soivfbsd_dn10), (-locals.var_pparam_b4soivfbsd_dn11), (-locals.var_pparam_b4soivfbsd_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign28510_e22165;
        locals.var_t0__blk1144_dn3 = assign28510_e22165_d_n3;
        locals.var_t0__blk1144_dn4 = assign28510_e22165_d_n4;
        locals.var_t0__blk1144_dn5 = assign28510_e22165_d_n5;
        locals.var_t0__blk1144_dn6 = assign28510_e22165_d_n6;
        locals.var_t0__blk1144_dn7 = assign28510_e22165_d_n7;
        locals.var_t0__blk1144_dn8 = assign28510_e22165_d_n8;
        locals.var_t0__blk1144_dn9 = assign28510_e22165_d_n9;
        locals.var_t0__blk1144_dn10 = assign28510_e22165_d_n10;
        locals.var_t0__blk1144_dn11 = assign28510_e22165_d_n11;
        locals.var_t0__blk1144_dn12 = assign28510_e22165_d_n12;

        let (assign28520_e22174, assign28520_e22174_d_n3, assign28520_e22174_d_n4, assign28520_e22174_d_n5, assign28520_e22174_d_n6, assign28520_e22174_d_n7, assign28520_e22174_d_n8, assign28520_e22174_d_n9, assign28520_e22174_d_n10, assign28520_e22174_d_n11, assign28520_e22174_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28520_e22169: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign28520_e22171: f64 = (assign28520_e22169 + 0.0001);
        let assign28520_e22172: f64 = (assign28520_e22171).sqrt();
        (assign28520_e22172, (((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) / (2.0 * assign28520_e22172)), (((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) / (2.0 * assign28520_e22172)), (((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) / (2.0 * assign28520_e22172)), (((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) / (2.0 * assign28520_e22172)), (((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) / (2.0 * assign28520_e22172)), (((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) / (2.0 * assign28520_e22172)), (((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) / (2.0 * assign28520_e22172)), (((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) / (2.0 * assign28520_e22172)), (((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) / (2.0 * assign28520_e22172)), (((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) / (2.0 * assign28520_e22172)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign28520_e22174;
        locals.var_t1__blk1145_dn3 = assign28520_e22174_d_n3;
        locals.var_t1__blk1145_dn4 = assign28520_e22174_d_n4;
        locals.var_t1__blk1145_dn5 = assign28520_e22174_d_n5;
        locals.var_t1__blk1145_dn6 = assign28520_e22174_d_n6;
        locals.var_t1__blk1145_dn7 = assign28520_e22174_d_n7;
        locals.var_t1__blk1145_dn8 = assign28520_e22174_d_n8;
        locals.var_t1__blk1145_dn9 = assign28520_e22174_d_n9;
        locals.var_t1__blk1145_dn10 = assign28520_e22174_d_n10;
        locals.var_t1__blk1145_dn11 = assign28520_e22174_d_n11;
        locals.var_t1__blk1145_dn12 = assign28520_e22174_d_n12;

        let (assign28530_e22182, assign28530_e22182_d_n3, assign28530_e22182_d_n4, assign28530_e22182_d_n5, assign28530_e22182_d_n6, assign28530_e22182_d_n7, assign28530_e22182_d_n8, assign28530_e22182_d_n9, assign28530_e22182_d_n10, assign28530_e22182_d_n11, assign28530_e22182_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28530_e22179: f64 = (locals.var_t0__blk1144 + locals.var_t1__blk1145);
        let assign28530_e22180: f64 = (0.5 * assign28530_e22179);
        (assign28530_e22180, (0.5 * (locals.var_t0__blk1144_dn3 + locals.var_t1__blk1145_dn3)), (0.5 * (locals.var_t0__blk1144_dn4 + locals.var_t1__blk1145_dn4)), (0.5 * (locals.var_t0__blk1144_dn5 + locals.var_t1__blk1145_dn5)), (0.5 * (locals.var_t0__blk1144_dn6 + locals.var_t1__blk1145_dn6)), (0.5 * (locals.var_t0__blk1144_dn7 + locals.var_t1__blk1145_dn7)), (0.5 * (locals.var_t0__blk1144_dn8 + locals.var_t1__blk1145_dn8)), (0.5 * (locals.var_t0__blk1144_dn9 + locals.var_t1__blk1145_dn9)), (0.5 * (locals.var_t0__blk1144_dn10 + locals.var_t1__blk1145_dn10)), (0.5 * (locals.var_t0__blk1144_dn11 + locals.var_t1__blk1145_dn11)), (0.5 * (locals.var_t0__blk1144_dn12 + locals.var_t1__blk1145_dn12)),)
    } else {
        (locals.var_vgs_eff_1, locals.var_vgs_eff_1_dn3, locals.var_vgs_eff_1_dn4, locals.var_vgs_eff_1_dn5, locals.var_vgs_eff_1_dn6, locals.var_vgs_eff_1_dn7, locals.var_vgs_eff_1_dn8, locals.var_vgs_eff_1_dn9, locals.var_vgs_eff_1_dn10, locals.var_vgs_eff_1_dn11, locals.var_vgs_eff_1_dn12,)
    }
};
        locals.var_vgs_eff_1 = assign28530_e22182;
        locals.var_vgs_eff_1_dn3 = assign28530_e22182_d_n3;
        locals.var_vgs_eff_1_dn4 = assign28530_e22182_d_n4;
        locals.var_vgs_eff_1_dn5 = assign28530_e22182_d_n5;
        locals.var_vgs_eff_1_dn6 = assign28530_e22182_d_n6;
        locals.var_vgs_eff_1_dn7 = assign28530_e22182_d_n7;
        locals.var_vgs_eff_1_dn8 = assign28530_e22182_d_n8;
        locals.var_vgs_eff_1_dn9 = assign28530_e22182_d_n9;
        locals.var_vgs_eff_1_dn10 = assign28530_e22182_d_n10;
        locals.var_vgs_eff_1_dn11 = assign28530_e22182_d_n11;
        locals.var_vgs_eff_1_dn12 = assign28530_e22182_d_n12;

        let (assign28540_e22190, assign28540_e22190_d_n3, assign28540_e22190_d_n4, assign28540_e22190_d_n5, assign28540_e22190_d_n6, assign28540_e22190_d_n7, assign28540_e22190_d_n8, assign28540_e22190_d_n9, assign28540_e22190_d_n10, assign28540_e22190_d_n11, assign28540_e22190_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28540_e22187: f64 = (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1);
        let assign28540_e22188: f64 = (1.0 + assign28540_e22187);
        (assign28540_e22188, ((locals.var_pparam_b4soiprwg_dn3 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn3)), ((locals.var_pparam_b4soiprwg_dn4 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn4)), ((locals.var_pparam_b4soiprwg_dn5 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn5)), ((locals.var_pparam_b4soiprwg_dn6 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn6)), ((locals.var_pparam_b4soiprwg_dn7 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn7)), ((locals.var_pparam_b4soiprwg_dn8 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn8)), ((locals.var_pparam_b4soiprwg_dn9 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn9)), ((locals.var_pparam_b4soiprwg_dn10 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn10)), ((locals.var_pparam_b4soiprwg_dn11 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn11)), ((locals.var_pparam_b4soiprwg_dn12 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign28540_e22190;
        locals.var_t0__blk1144_dn3 = assign28540_e22190_d_n3;
        locals.var_t0__blk1144_dn4 = assign28540_e22190_d_n4;
        locals.var_t0__blk1144_dn5 = assign28540_e22190_d_n5;
        locals.var_t0__blk1144_dn6 = assign28540_e22190_d_n6;
        locals.var_t0__blk1144_dn7 = assign28540_e22190_d_n7;
        locals.var_t0__blk1144_dn8 = assign28540_e22190_d_n8;
        locals.var_t0__blk1144_dn9 = assign28540_e22190_d_n9;
        locals.var_t0__blk1144_dn10 = assign28540_e22190_d_n10;
        locals.var_t0__blk1144_dn11 = assign28540_e22190_d_n11;
        locals.var_t0__blk1144_dn12 = assign28540_e22190_d_n12;

        let (assign28550_e22197, assign28550_e22197_d_n3, assign28550_e22197_d_n4, assign28550_e22197_d_n5, assign28550_e22197_d_n6, assign28550_e22197_d_n7, assign28550_e22197_d_n8, assign28550_e22197_d_n9, assign28550_e22197_d_n10, assign28550_e22197_d_n11, assign28550_e22197_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28550_e22193: f64 = (-locals.var_pparam_b4soiprwb);
        let assign28550_e22195: f64 = (assign28550_e22193 * locals.var_vbs);
        (assign28550_e22195, ((-locals.var_pparam_b4soiprwb_dn3) * locals.var_vbs), ((-locals.var_pparam_b4soiprwb_dn4) * locals.var_vbs), (((-locals.var_pparam_b4soiprwb_dn5) * locals.var_vbs) + (assign28550_e22193 * locals.var_vbs_dn5)), ((-locals.var_pparam_b4soiprwb_dn6) * locals.var_vbs), ((-locals.var_pparam_b4soiprwb_dn7) * locals.var_vbs), (((-locals.var_pparam_b4soiprwb_dn8) * locals.var_vbs) + (assign28550_e22193 * locals.var_vbs_dn8)), ((-locals.var_pparam_b4soiprwb_dn9) * locals.var_vbs), ((-locals.var_pparam_b4soiprwb_dn10) * locals.var_vbs), ((-locals.var_pparam_b4soiprwb_dn11) * locals.var_vbs), ((-locals.var_pparam_b4soiprwb_dn12) * locals.var_vbs),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign28550_e22197;
        locals.var_t1__blk1145_dn3 = assign28550_e22197_d_n3;
        locals.var_t1__blk1145_dn4 = assign28550_e22197_d_n4;
        locals.var_t1__blk1145_dn5 = assign28550_e22197_d_n5;
        locals.var_t1__blk1145_dn6 = assign28550_e22197_d_n6;
        locals.var_t1__blk1145_dn7 = assign28550_e22197_d_n7;
        locals.var_t1__blk1145_dn8 = assign28550_e22197_d_n8;
        locals.var_t1__blk1145_dn9 = assign28550_e22197_d_n9;
        locals.var_t1__blk1145_dn10 = assign28550_e22197_d_n10;
        locals.var_t1__blk1145_dn11 = assign28550_e22197_d_n11;
        locals.var_t1__blk1145_dn12 = assign28550_e22197_d_n12;

        let (assign28560_e22205, assign28560_e22205_d_n3, assign28560_e22205_d_n4, assign28560_e22205_d_n5, assign28560_e22205_d_n6, assign28560_e22205_d_n7, assign28560_e22205_d_n8, assign28560_e22205_d_n9, assign28560_e22205_d_n10, assign28560_e22205_d_n11, assign28560_e22205_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28560_e22201: f64 = (1.0 / locals.var_t0__blk1144);
        let assign28560_e22203: f64 = (assign28560_e22201 + locals.var_t1__blk1145);
        (assign28560_e22203, ((-(locals.var_t0__blk1144_dn3 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn3), ((-(locals.var_t0__blk1144_dn4 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn4), ((-(locals.var_t0__blk1144_dn5 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn5), ((-(locals.var_t0__blk1144_dn6 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn6), ((-(locals.var_t0__blk1144_dn7 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn7), ((-(locals.var_t0__blk1144_dn8 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn8), ((-(locals.var_t0__blk1144_dn9 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn9), ((-(locals.var_t0__blk1144_dn10 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn10), ((-(locals.var_t0__blk1144_dn11 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn11), ((-(locals.var_t0__blk1144_dn12 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn12),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign28560_e22205;
        locals.var_t2__blk1146_dn3 = assign28560_e22205_d_n3;
        locals.var_t2__blk1146_dn4 = assign28560_e22205_d_n4;
        locals.var_t2__blk1146_dn5 = assign28560_e22205_d_n5;
        locals.var_t2__blk1146_dn6 = assign28560_e22205_d_n6;
        locals.var_t2__blk1146_dn7 = assign28560_e22205_d_n7;
        locals.var_t2__blk1146_dn8 = assign28560_e22205_d_n8;
        locals.var_t2__blk1146_dn9 = assign28560_e22205_d_n9;
        locals.var_t2__blk1146_dn10 = assign28560_e22205_d_n10;
        locals.var_t2__blk1146_dn11 = assign28560_e22205_d_n11;
        locals.var_t2__blk1146_dn12 = assign28560_e22205_d_n12;

        let (assign28570_e22216, assign28570_e22216_d_n3, assign28570_e22216_d_n4, assign28570_e22216_d_n5, assign28570_e22216_d_n6, assign28570_e22216_d_n7, assign28570_e22216_d_n8, assign28570_e22216_d_n9, assign28570_e22216_d_n10, assign28570_e22216_d_n11, assign28570_e22216_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28570_e22210: f64 = (locals.var_t2__blk1146 * locals.var_t2__blk1146);
        let assign28570_e22212: f64 = (assign28570_e22210 + 0.01);
        let assign28570_e22213: f64 = (assign28570_e22212).sqrt();
        let assign28570_e22214: f64 = (locals.var_t2__blk1146 + assign28570_e22213);
        (assign28570_e22214, (locals.var_t2__blk1146_dn3 + (((locals.var_t2__blk1146_dn3 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn3)) / (2.0 * assign28570_e22213))), (locals.var_t2__blk1146_dn4 + (((locals.var_t2__blk1146_dn4 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn4)) / (2.0 * assign28570_e22213))), (locals.var_t2__blk1146_dn5 + (((locals.var_t2__blk1146_dn5 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn5)) / (2.0 * assign28570_e22213))), (locals.var_t2__blk1146_dn6 + (((locals.var_t2__blk1146_dn6 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn6)) / (2.0 * assign28570_e22213))), (locals.var_t2__blk1146_dn7 + (((locals.var_t2__blk1146_dn7 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn7)) / (2.0 * assign28570_e22213))), (locals.var_t2__blk1146_dn8 + (((locals.var_t2__blk1146_dn8 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn8)) / (2.0 * assign28570_e22213))), (locals.var_t2__blk1146_dn9 + (((locals.var_t2__blk1146_dn9 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn9)) / (2.0 * assign28570_e22213))), (locals.var_t2__blk1146_dn10 + (((locals.var_t2__blk1146_dn10 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn10)) / (2.0 * assign28570_e22213))), (locals.var_t2__blk1146_dn11 + (((locals.var_t2__blk1146_dn11 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn11)) / (2.0 * assign28570_e22213))), (locals.var_t2__blk1146_dn12 + (((locals.var_t2__blk1146_dn12 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn12)) / (2.0 * assign28570_e22213))),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign28570_e22216;
        locals.var_t3__blk1147_dn3 = assign28570_e22216_d_n3;
        locals.var_t3__blk1147_dn4 = assign28570_e22216_d_n4;
        locals.var_t3__blk1147_dn5 = assign28570_e22216_d_n5;
        locals.var_t3__blk1147_dn6 = assign28570_e22216_d_n6;
        locals.var_t3__blk1147_dn7 = assign28570_e22216_d_n7;
        locals.var_t3__blk1147_dn8 = assign28570_e22216_d_n8;
        locals.var_t3__blk1147_dn9 = assign28570_e22216_d_n9;
        locals.var_t3__blk1147_dn10 = assign28570_e22216_d_n10;
        locals.var_t3__blk1147_dn11 = assign28570_e22216_d_n11;
        locals.var_t3__blk1147_dn12 = assign28570_e22216_d_n12;

        let (assign28580_e22222, assign28580_e22222_d_n3, assign28580_e22222_d_n4, assign28580_e22222_d_n5, assign28580_e22222_d_n6, assign28580_e22222_d_n7, assign28580_e22222_d_n8, assign28580_e22222_d_n9, assign28580_e22222_d_n10, assign28580_e22222_d_n11, assign28580_e22222_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28580_e22220: f64 = (locals.var_rs0 * 0.5);
        (assign28580_e22220, (locals.var_rs0_dn3 * 0.5), (locals.var_rs0_dn4 * 0.5), (locals.var_rs0_dn5 * 0.5), (locals.var_rs0_dn6 * 0.5), (locals.var_rs0_dn7 * 0.5), (locals.var_rs0_dn8 * 0.5), (locals.var_rs0_dn9 * 0.5), (locals.var_rs0_dn10 * 0.5), (locals.var_rs0_dn11 * 0.5), (locals.var_rs0_dn12 * 0.5),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign28580_e22222;
        locals.var_t4__blk1148_dn3 = assign28580_e22222_d_n3;
        locals.var_t4__blk1148_dn4 = assign28580_e22222_d_n4;
        locals.var_t4__blk1148_dn5 = assign28580_e22222_d_n5;
        locals.var_t4__blk1148_dn6 = assign28580_e22222_d_n6;
        locals.var_t4__blk1148_dn7 = assign28580_e22222_d_n7;
        locals.var_t4__blk1148_dn8 = assign28580_e22222_d_n8;
        locals.var_t4__blk1148_dn9 = assign28580_e22222_d_n9;
        locals.var_t4__blk1148_dn10 = assign28580_e22222_d_n10;
        locals.var_t4__blk1148_dn11 = assign28580_e22222_d_n11;
        locals.var_t4__blk1148_dn12 = assign28580_e22222_d_n12;

        let (assign28590_e22232, assign28590_e22232_d_n3, assign28590_e22232_d_n4, assign28590_e22232_d_n5, assign28590_e22232_d_n6, assign28590_e22232_d_n7, assign28590_e22232_d_n8, assign28590_e22232_d_n9, assign28590_e22232_d_n10, assign28590_e22232_d_n11, assign28590_e22232_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28590_e22227: f64 = (locals.var_t3__blk1147 * locals.var_t4__blk1148);
        let assign28590_e22228: f64 = (locals.var_rswmin + assign28590_e22227);
        let assign28590_e22230: f64 = (assign28590_e22228 + locals.var_b4soisourceresistance);
        (assign28590_e22230, (locals.var_rswmin_dn3 + ((locals.var_t3__blk1147_dn3 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn3))), (locals.var_rswmin_dn4 + ((locals.var_t3__blk1147_dn4 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn4))), (locals.var_rswmin_dn5 + ((locals.var_t3__blk1147_dn5 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn5))), (locals.var_rswmin_dn6 + ((locals.var_t3__blk1147_dn6 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn6))), (locals.var_rswmin_dn7 + ((locals.var_t3__blk1147_dn7 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn7))), (locals.var_rswmin_dn8 + ((locals.var_t3__blk1147_dn8 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn8))), (locals.var_rswmin_dn9 + ((locals.var_t3__blk1147_dn9 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn9))), (locals.var_rswmin_dn10 + ((locals.var_t3__blk1147_dn10 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn10))), (locals.var_rswmin_dn11 + ((locals.var_t3__blk1147_dn11 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn11))), (locals.var_rswmin_dn12 + ((locals.var_t3__blk1147_dn12 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn12))),)
    } else {
        (locals.var_rs, locals.var_rs_dn3, locals.var_rs_dn4, locals.var_rs_dn5, locals.var_rs_dn6, locals.var_rs_dn7, locals.var_rs_dn8, locals.var_rs_dn9, locals.var_rs_dn10, locals.var_rs_dn11, locals.var_rs_dn12,)
    }
};
        locals.var_rs = assign28590_e22232;
        locals.var_rs_dn3 = assign28590_e22232_d_n3;
        locals.var_rs_dn4 = assign28590_e22232_d_n4;
        locals.var_rs_dn5 = assign28590_e22232_d_n5;
        locals.var_rs_dn6 = assign28590_e22232_d_n6;
        locals.var_rs_dn7 = assign28590_e22232_d_n7;
        locals.var_rs_dn8 = assign28590_e22232_d_n8;
        locals.var_rs_dn9 = assign28590_e22232_d_n9;
        locals.var_rs_dn10 = assign28590_e22232_d_n10;
        locals.var_rs_dn11 = assign28590_e22232_d_n11;
        locals.var_rs_dn12 = assign28590_e22232_d_n12;

        let (assign28600_e22238, assign28600_e22238_d_n3, assign28600_e22238_d_n4, assign28600_e22238_d_n5, assign28600_e22238_d_n6, assign28600_e22238_d_n7, assign28600_e22238_d_n8, assign28600_e22238_d_n9, assign28600_e22238_d_n10, assign28600_e22238_d_n11, assign28600_e22238_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28600_e22236: f64 = (locals.var_vgd - locals.var_pparam_b4soivfbsd);
        (assign28600_e22236, (-locals.var_pparam_b4soivfbsd_dn3), (-locals.var_pparam_b4soivfbsd_dn4), (-locals.var_pparam_b4soivfbsd_dn5), (-locals.var_pparam_b4soivfbsd_dn6), (locals.var_vgd_dn7 - locals.var_pparam_b4soivfbsd_dn7), (locals.var_vgd_dn8 - locals.var_pparam_b4soivfbsd_dn8), (locals.var_vgd_dn9 - locals.var_pparam_b4soivfbsd_dn9), (-locals.var_pparam_b4soivfbsd_dn10), (-locals.var_pparam_b4soivfbsd_dn11), (-locals.var_pparam_b4soivfbsd_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign28600_e22238;
        locals.var_t0__blk1144_dn3 = assign28600_e22238_d_n3;
        locals.var_t0__blk1144_dn4 = assign28600_e22238_d_n4;
        locals.var_t0__blk1144_dn5 = assign28600_e22238_d_n5;
        locals.var_t0__blk1144_dn6 = assign28600_e22238_d_n6;
        locals.var_t0__blk1144_dn7 = assign28600_e22238_d_n7;
        locals.var_t0__blk1144_dn8 = assign28600_e22238_d_n8;
        locals.var_t0__blk1144_dn9 = assign28600_e22238_d_n9;
        locals.var_t0__blk1144_dn10 = assign28600_e22238_d_n10;
        locals.var_t0__blk1144_dn11 = assign28600_e22238_d_n11;
        locals.var_t0__blk1144_dn12 = assign28600_e22238_d_n12;

        let (assign28610_e22247, assign28610_e22247_d_n3, assign28610_e22247_d_n4, assign28610_e22247_d_n5, assign28610_e22247_d_n6, assign28610_e22247_d_n7, assign28610_e22247_d_n8, assign28610_e22247_d_n9, assign28610_e22247_d_n10, assign28610_e22247_d_n11, assign28610_e22247_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28610_e22242: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign28610_e22244: f64 = (assign28610_e22242 + 0.0001);
        let assign28610_e22245: f64 = (assign28610_e22244).sqrt();
        (assign28610_e22245, (((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) / (2.0 * assign28610_e22245)), (((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) / (2.0 * assign28610_e22245)), (((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) / (2.0 * assign28610_e22245)), (((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) / (2.0 * assign28610_e22245)), (((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) / (2.0 * assign28610_e22245)), (((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) / (2.0 * assign28610_e22245)), (((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) / (2.0 * assign28610_e22245)), (((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) / (2.0 * assign28610_e22245)), (((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) / (2.0 * assign28610_e22245)), (((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) / (2.0 * assign28610_e22245)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign28610_e22247;
        locals.var_t1__blk1145_dn3 = assign28610_e22247_d_n3;
        locals.var_t1__blk1145_dn4 = assign28610_e22247_d_n4;
        locals.var_t1__blk1145_dn5 = assign28610_e22247_d_n5;
        locals.var_t1__blk1145_dn6 = assign28610_e22247_d_n6;
        locals.var_t1__blk1145_dn7 = assign28610_e22247_d_n7;
        locals.var_t1__blk1145_dn8 = assign28610_e22247_d_n8;
        locals.var_t1__blk1145_dn9 = assign28610_e22247_d_n9;
        locals.var_t1__blk1145_dn10 = assign28610_e22247_d_n10;
        locals.var_t1__blk1145_dn11 = assign28610_e22247_d_n11;
        locals.var_t1__blk1145_dn12 = assign28610_e22247_d_n12;

        let (assign28620_e22255, assign28620_e22255_d_n3, assign28620_e22255_d_n4, assign28620_e22255_d_n5, assign28620_e22255_d_n6, assign28620_e22255_d_n7, assign28620_e22255_d_n8, assign28620_e22255_d_n9, assign28620_e22255_d_n10, assign28620_e22255_d_n11, assign28620_e22255_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28620_e22252: f64 = (locals.var_t0__blk1144 + locals.var_t1__blk1145);
        let assign28620_e22253: f64 = (0.5 * assign28620_e22252);
        (assign28620_e22253, (0.5 * (locals.var_t0__blk1144_dn3 + locals.var_t1__blk1145_dn3)), (0.5 * (locals.var_t0__blk1144_dn4 + locals.var_t1__blk1145_dn4)), (0.5 * (locals.var_t0__blk1144_dn5 + locals.var_t1__blk1145_dn5)), (0.5 * (locals.var_t0__blk1144_dn6 + locals.var_t1__blk1145_dn6)), (0.5 * (locals.var_t0__blk1144_dn7 + locals.var_t1__blk1145_dn7)), (0.5 * (locals.var_t0__blk1144_dn8 + locals.var_t1__blk1145_dn8)), (0.5 * (locals.var_t0__blk1144_dn9 + locals.var_t1__blk1145_dn9)), (0.5 * (locals.var_t0__blk1144_dn10 + locals.var_t1__blk1145_dn10)), (0.5 * (locals.var_t0__blk1144_dn11 + locals.var_t1__blk1145_dn11)), (0.5 * (locals.var_t0__blk1144_dn12 + locals.var_t1__blk1145_dn12)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11, locals.var_vgd_eff_dn12,)
    }
};
        locals.var_vgd_eff = assign28620_e22255;
        locals.var_vgd_eff_dn3 = assign28620_e22255_d_n3;
        locals.var_vgd_eff_dn4 = assign28620_e22255_d_n4;
        locals.var_vgd_eff_dn5 = assign28620_e22255_d_n5;
        locals.var_vgd_eff_dn6 = assign28620_e22255_d_n6;
        locals.var_vgd_eff_dn7 = assign28620_e22255_d_n7;
        locals.var_vgd_eff_dn8 = assign28620_e22255_d_n8;
        locals.var_vgd_eff_dn9 = assign28620_e22255_d_n9;
        locals.var_vgd_eff_dn10 = assign28620_e22255_d_n10;
        locals.var_vgd_eff_dn11 = assign28620_e22255_d_n11;
        locals.var_vgd_eff_dn12 = assign28620_e22255_d_n12;

    }

    pub(super) fn stamp_transient_block_76(
        locals: &mut StampLocals,
    ) {
        let (assign28630_e22263, assign28630_e22263_d_n3, assign28630_e22263_d_n4, assign28630_e22263_d_n5, assign28630_e22263_d_n6, assign28630_e22263_d_n7, assign28630_e22263_d_n8, assign28630_e22263_d_n9, assign28630_e22263_d_n10, assign28630_e22263_d_n11, assign28630_e22263_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28630_e22260: f64 = (locals.var_pparam_b4soiprwg * locals.var_vgd_eff);
        let assign28630_e22261: f64 = (1.0 + assign28630_e22260);
        (assign28630_e22261, ((locals.var_pparam_b4soiprwg_dn3 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn3)), ((locals.var_pparam_b4soiprwg_dn4 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn4)), ((locals.var_pparam_b4soiprwg_dn5 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn5)), ((locals.var_pparam_b4soiprwg_dn6 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn6)), ((locals.var_pparam_b4soiprwg_dn7 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn7)), ((locals.var_pparam_b4soiprwg_dn8 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn8)), ((locals.var_pparam_b4soiprwg_dn9 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn9)), ((locals.var_pparam_b4soiprwg_dn10 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn10)), ((locals.var_pparam_b4soiprwg_dn11 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn11)), ((locals.var_pparam_b4soiprwg_dn12 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign28630_e22263;
        locals.var_t0__blk1144_dn3 = assign28630_e22263_d_n3;
        locals.var_t0__blk1144_dn4 = assign28630_e22263_d_n4;
        locals.var_t0__blk1144_dn5 = assign28630_e22263_d_n5;
        locals.var_t0__blk1144_dn6 = assign28630_e22263_d_n6;
        locals.var_t0__blk1144_dn7 = assign28630_e22263_d_n7;
        locals.var_t0__blk1144_dn8 = assign28630_e22263_d_n8;
        locals.var_t0__blk1144_dn9 = assign28630_e22263_d_n9;
        locals.var_t0__blk1144_dn10 = assign28630_e22263_d_n10;
        locals.var_t0__blk1144_dn11 = assign28630_e22263_d_n11;
        locals.var_t0__blk1144_dn12 = assign28630_e22263_d_n12;

        let (assign28640_e22270, assign28640_e22270_d_n3, assign28640_e22270_d_n4, assign28640_e22270_d_n5, assign28640_e22270_d_n6, assign28640_e22270_d_n7, assign28640_e22270_d_n8, assign28640_e22270_d_n9, assign28640_e22270_d_n10, assign28640_e22270_d_n11, assign28640_e22270_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28640_e22266: f64 = (-locals.var_pparam_b4soiprwb);
        let assign28640_e22268: f64 = (assign28640_e22266 * locals.var_vbd);
        (assign28640_e22268, ((-locals.var_pparam_b4soiprwb_dn3) * locals.var_vbd), ((-locals.var_pparam_b4soiprwb_dn4) * locals.var_vbd), (((-locals.var_pparam_b4soiprwb_dn5) * locals.var_vbd) + (assign28640_e22266 * locals.var_vbd_dn5)), ((-locals.var_pparam_b4soiprwb_dn6) * locals.var_vbd), (((-locals.var_pparam_b4soiprwb_dn7) * locals.var_vbd) + (assign28640_e22266 * locals.var_vbd_dn7)), (((-locals.var_pparam_b4soiprwb_dn8) * locals.var_vbd) + (assign28640_e22266 * locals.var_vbd_dn8)), ((-locals.var_pparam_b4soiprwb_dn9) * locals.var_vbd), ((-locals.var_pparam_b4soiprwb_dn10) * locals.var_vbd), ((-locals.var_pparam_b4soiprwb_dn11) * locals.var_vbd), ((-locals.var_pparam_b4soiprwb_dn12) * locals.var_vbd),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign28640_e22270;
        locals.var_t1__blk1145_dn3 = assign28640_e22270_d_n3;
        locals.var_t1__blk1145_dn4 = assign28640_e22270_d_n4;
        locals.var_t1__blk1145_dn5 = assign28640_e22270_d_n5;
        locals.var_t1__blk1145_dn6 = assign28640_e22270_d_n6;
        locals.var_t1__blk1145_dn7 = assign28640_e22270_d_n7;
        locals.var_t1__blk1145_dn8 = assign28640_e22270_d_n8;
        locals.var_t1__blk1145_dn9 = assign28640_e22270_d_n9;
        locals.var_t1__blk1145_dn10 = assign28640_e22270_d_n10;
        locals.var_t1__blk1145_dn11 = assign28640_e22270_d_n11;
        locals.var_t1__blk1145_dn12 = assign28640_e22270_d_n12;

        let (assign28650_e22278, assign28650_e22278_d_n3, assign28650_e22278_d_n4, assign28650_e22278_d_n5, assign28650_e22278_d_n6, assign28650_e22278_d_n7, assign28650_e22278_d_n8, assign28650_e22278_d_n9, assign28650_e22278_d_n10, assign28650_e22278_d_n11, assign28650_e22278_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28650_e22274: f64 = (1.0 / locals.var_t0__blk1144);
        let assign28650_e22276: f64 = (assign28650_e22274 + locals.var_t1__blk1145);
        (assign28650_e22276, ((-(locals.var_t0__blk1144_dn3 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn3), ((-(locals.var_t0__blk1144_dn4 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn4), ((-(locals.var_t0__blk1144_dn5 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn5), ((-(locals.var_t0__blk1144_dn6 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn6), ((-(locals.var_t0__blk1144_dn7 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn7), ((-(locals.var_t0__blk1144_dn8 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn8), ((-(locals.var_t0__blk1144_dn9 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn9), ((-(locals.var_t0__blk1144_dn10 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn10), ((-(locals.var_t0__blk1144_dn11 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn11), ((-(locals.var_t0__blk1144_dn12 / (locals.var_t0__blk1144 * locals.var_t0__blk1144))) + locals.var_t1__blk1145_dn12),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign28650_e22278;
        locals.var_t2__blk1146_dn3 = assign28650_e22278_d_n3;
        locals.var_t2__blk1146_dn4 = assign28650_e22278_d_n4;
        locals.var_t2__blk1146_dn5 = assign28650_e22278_d_n5;
        locals.var_t2__blk1146_dn6 = assign28650_e22278_d_n6;
        locals.var_t2__blk1146_dn7 = assign28650_e22278_d_n7;
        locals.var_t2__blk1146_dn8 = assign28650_e22278_d_n8;
        locals.var_t2__blk1146_dn9 = assign28650_e22278_d_n9;
        locals.var_t2__blk1146_dn10 = assign28650_e22278_d_n10;
        locals.var_t2__blk1146_dn11 = assign28650_e22278_d_n11;
        locals.var_t2__blk1146_dn12 = assign28650_e22278_d_n12;

        let (assign28660_e22289, assign28660_e22289_d_n3, assign28660_e22289_d_n4, assign28660_e22289_d_n5, assign28660_e22289_d_n6, assign28660_e22289_d_n7, assign28660_e22289_d_n8, assign28660_e22289_d_n9, assign28660_e22289_d_n10, assign28660_e22289_d_n11, assign28660_e22289_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28660_e22283: f64 = (locals.var_t2__blk1146 * locals.var_t2__blk1146);
        let assign28660_e22285: f64 = (assign28660_e22283 + 0.01);
        let assign28660_e22286: f64 = (assign28660_e22285).sqrt();
        let assign28660_e22287: f64 = (locals.var_t2__blk1146 + assign28660_e22286);
        (assign28660_e22287, (locals.var_t2__blk1146_dn3 + (((locals.var_t2__blk1146_dn3 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn3)) / (2.0 * assign28660_e22286))), (locals.var_t2__blk1146_dn4 + (((locals.var_t2__blk1146_dn4 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn4)) / (2.0 * assign28660_e22286))), (locals.var_t2__blk1146_dn5 + (((locals.var_t2__blk1146_dn5 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn5)) / (2.0 * assign28660_e22286))), (locals.var_t2__blk1146_dn6 + (((locals.var_t2__blk1146_dn6 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn6)) / (2.0 * assign28660_e22286))), (locals.var_t2__blk1146_dn7 + (((locals.var_t2__blk1146_dn7 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn7)) / (2.0 * assign28660_e22286))), (locals.var_t2__blk1146_dn8 + (((locals.var_t2__blk1146_dn8 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn8)) / (2.0 * assign28660_e22286))), (locals.var_t2__blk1146_dn9 + (((locals.var_t2__blk1146_dn9 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn9)) / (2.0 * assign28660_e22286))), (locals.var_t2__blk1146_dn10 + (((locals.var_t2__blk1146_dn10 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn10)) / (2.0 * assign28660_e22286))), (locals.var_t2__blk1146_dn11 + (((locals.var_t2__blk1146_dn11 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn11)) / (2.0 * assign28660_e22286))), (locals.var_t2__blk1146_dn12 + (((locals.var_t2__blk1146_dn12 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn12)) / (2.0 * assign28660_e22286))),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign28660_e22289;
        locals.var_t3__blk1147_dn3 = assign28660_e22289_d_n3;
        locals.var_t3__blk1147_dn4 = assign28660_e22289_d_n4;
        locals.var_t3__blk1147_dn5 = assign28660_e22289_d_n5;
        locals.var_t3__blk1147_dn6 = assign28660_e22289_d_n6;
        locals.var_t3__blk1147_dn7 = assign28660_e22289_d_n7;
        locals.var_t3__blk1147_dn8 = assign28660_e22289_d_n8;
        locals.var_t3__blk1147_dn9 = assign28660_e22289_d_n9;
        locals.var_t3__blk1147_dn10 = assign28660_e22289_d_n10;
        locals.var_t3__blk1147_dn11 = assign28660_e22289_d_n11;
        locals.var_t3__blk1147_dn12 = assign28660_e22289_d_n12;

        let (assign28670_e22295, assign28670_e22295_d_n3, assign28670_e22295_d_n4, assign28670_e22295_d_n5, assign28670_e22295_d_n6, assign28670_e22295_d_n7, assign28670_e22295_d_n8, assign28670_e22295_d_n9, assign28670_e22295_d_n10, assign28670_e22295_d_n11, assign28670_e22295_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28670_e22293: f64 = (locals.var_rd0 * 0.5);
        (assign28670_e22293, (locals.var_rd0_dn3 * 0.5), (locals.var_rd0_dn4 * 0.5), (locals.var_rd0_dn5 * 0.5), (locals.var_rd0_dn6 * 0.5), (locals.var_rd0_dn7 * 0.5), (locals.var_rd0_dn8 * 0.5), (locals.var_rd0_dn9 * 0.5), (locals.var_rd0_dn10 * 0.5), (locals.var_rd0_dn11 * 0.5), (locals.var_rd0_dn12 * 0.5),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign28670_e22295;
        locals.var_t4__blk1148_dn3 = assign28670_e22295_d_n3;
        locals.var_t4__blk1148_dn4 = assign28670_e22295_d_n4;
        locals.var_t4__blk1148_dn5 = assign28670_e22295_d_n5;
        locals.var_t4__blk1148_dn6 = assign28670_e22295_d_n6;
        locals.var_t4__blk1148_dn7 = assign28670_e22295_d_n7;
        locals.var_t4__blk1148_dn8 = assign28670_e22295_d_n8;
        locals.var_t4__blk1148_dn9 = assign28670_e22295_d_n9;
        locals.var_t4__blk1148_dn10 = assign28670_e22295_d_n10;
        locals.var_t4__blk1148_dn11 = assign28670_e22295_d_n11;
        locals.var_t4__blk1148_dn12 = assign28670_e22295_d_n12;

        let (assign28680_e22305, assign28680_e22305_d_n3, assign28680_e22305_d_n4, assign28680_e22305_d_n5, assign28680_e22305_d_n6, assign28680_e22305_d_n7, assign28680_e22305_d_n8, assign28680_e22305_d_n9, assign28680_e22305_d_n10, assign28680_e22305_d_n11, assign28680_e22305_d_n12,) = {
    if (locals.var_guard1683 != 0.0) {
        let assign28680_e22300: f64 = (locals.var_t3__blk1147 * locals.var_t4__blk1148);
        let assign28680_e22301: f64 = (locals.var_rdwmin + assign28680_e22300);
        let assign28680_e22303: f64 = (assign28680_e22301 + locals.var_b4soidrainresistance);
        (assign28680_e22303, (locals.var_rdwmin_dn3 + ((locals.var_t3__blk1147_dn3 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn3))), (locals.var_rdwmin_dn4 + ((locals.var_t3__blk1147_dn4 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn4))), (locals.var_rdwmin_dn5 + ((locals.var_t3__blk1147_dn5 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn5))), (locals.var_rdwmin_dn6 + ((locals.var_t3__blk1147_dn6 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn6))), (locals.var_rdwmin_dn7 + ((locals.var_t3__blk1147_dn7 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn7))), (locals.var_rdwmin_dn8 + ((locals.var_t3__blk1147_dn8 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn8))), (locals.var_rdwmin_dn9 + ((locals.var_t3__blk1147_dn9 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn9))), (locals.var_rdwmin_dn10 + ((locals.var_t3__blk1147_dn10 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn10))), (locals.var_rdwmin_dn11 + ((locals.var_t3__blk1147_dn11 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn11))), (locals.var_rdwmin_dn12 + ((locals.var_t3__blk1147_dn12 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn12))),)
    } else {
        (locals.var_rd, locals.var_rd_dn3, locals.var_rd_dn4, locals.var_rd_dn5, locals.var_rd_dn6, locals.var_rd_dn7, locals.var_rd_dn8, locals.var_rd_dn9, locals.var_rd_dn10, locals.var_rd_dn11, locals.var_rd_dn12,)
    }
};
        locals.var_rd = assign28680_e22305;
        locals.var_rd_dn3 = assign28680_e22305_d_n3;
        locals.var_rd_dn4 = assign28680_e22305_d_n4;
        locals.var_rd_dn5 = assign28680_e22305_d_n5;
        locals.var_rd_dn6 = assign28680_e22305_d_n6;
        locals.var_rd_dn7 = assign28680_e22305_d_n7;
        locals.var_rd_dn8 = assign28680_e22305_d_n8;
        locals.var_rd_dn9 = assign28680_e22305_d_n9;
        locals.var_rd_dn10 = assign28680_e22305_d_n10;
        locals.var_rd_dn11 = assign28680_e22305_d_n11;
        locals.var_rd_dn12 = assign28680_e22305_d_n12;

        let (assign28690_e22310, assign28690_e22310_d_n3, assign28690_e22310_d_n4, assign28690_e22310_d_n5, assign28690_e22310_d_n6, assign28690_e22310_d_n7, assign28690_e22310_d_n8, assign28690_e22310_d_n9, assign28690_e22310_d_n10, assign28690_e22310_d_n11, assign28690_e22310_d_n12,) = {
    if (locals.var_guard1683 == 0.0) {
        (locals.var_b4soisourceresistance, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rs, locals.var_rs_dn3, locals.var_rs_dn4, locals.var_rs_dn5, locals.var_rs_dn6, locals.var_rs_dn7, locals.var_rs_dn8, locals.var_rs_dn9, locals.var_rs_dn10, locals.var_rs_dn11, locals.var_rs_dn12,)
    }
};
        locals.var_rs = assign28690_e22310;
        locals.var_rs_dn3 = assign28690_e22310_d_n3;
        locals.var_rs_dn4 = assign28690_e22310_d_n4;
        locals.var_rs_dn5 = assign28690_e22310_d_n5;
        locals.var_rs_dn6 = assign28690_e22310_d_n6;
        locals.var_rs_dn7 = assign28690_e22310_d_n7;
        locals.var_rs_dn8 = assign28690_e22310_d_n8;
        locals.var_rs_dn9 = assign28690_e22310_d_n9;
        locals.var_rs_dn10 = assign28690_e22310_d_n10;
        locals.var_rs_dn11 = assign28690_e22310_d_n11;
        locals.var_rs_dn12 = assign28690_e22310_d_n12;

        let (assign28700_e22315, assign28700_e22315_d_n3, assign28700_e22315_d_n4, assign28700_e22315_d_n5, assign28700_e22315_d_n6, assign28700_e22315_d_n7, assign28700_e22315_d_n8, assign28700_e22315_d_n9, assign28700_e22315_d_n10, assign28700_e22315_d_n11, assign28700_e22315_d_n12,) = {
    if (locals.var_guard1683 == 0.0) {
        (locals.var_b4soidrainresistance, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd, locals.var_rd_dn3, locals.var_rd_dn4, locals.var_rd_dn5, locals.var_rd_dn6, locals.var_rd_dn7, locals.var_rd_dn8, locals.var_rd_dn9, locals.var_rd_dn10, locals.var_rd_dn11, locals.var_rd_dn12,)
    }
};
        locals.var_rd = assign28700_e22315;
        locals.var_rd_dn3 = assign28700_e22315_d_n3;
        locals.var_rd_dn4 = assign28700_e22315_d_n4;
        locals.var_rd_dn5 = assign28700_e22315_d_n5;
        locals.var_rd_dn6 = assign28700_e22315_d_n6;
        locals.var_rd_dn7 = assign28700_e22315_d_n7;
        locals.var_rd_dn8 = assign28700_e22315_d_n8;
        locals.var_rd_dn9 = assign28700_e22315_d_n9;
        locals.var_rd_dn10 = assign28700_e22315_d_n10;
        locals.var_rd_dn11 = assign28700_e22315_d_n11;
        locals.var_rd_dn12 = assign28700_e22315_d_n12;

        let assign28710_e22318: f64 = if locals.var_b4soirdsmod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1684 = assign28710_e22318;

        let (assign28720_e22322, assign28720_e22322_d_n3, assign28720_e22322_d_n4, assign28720_e22322_d_n5, assign28720_e22322_d_n6, assign28720_e22322_d_n7, assign28720_e22322_d_n8, assign28720_e22322_d_n9, assign28720_e22322_d_n10, assign28720_e22322_d_n11, assign28720_e22322_d_n12,) = {
    if (locals.var_guard1684 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rs, locals.var_rs_dn3, locals.var_rs_dn4, locals.var_rs_dn5, locals.var_rs_dn6, locals.var_rs_dn7, locals.var_rs_dn8, locals.var_rs_dn9, locals.var_rs_dn10, locals.var_rs_dn11, locals.var_rs_dn12,)
    }
};
        locals.var_rs = assign28720_e22322;
        locals.var_rs_dn3 = assign28720_e22322_d_n3;
        locals.var_rs_dn4 = assign28720_e22322_d_n4;
        locals.var_rs_dn5 = assign28720_e22322_d_n5;
        locals.var_rs_dn6 = assign28720_e22322_d_n6;
        locals.var_rs_dn7 = assign28720_e22322_d_n7;
        locals.var_rs_dn8 = assign28720_e22322_d_n8;
        locals.var_rs_dn9 = assign28720_e22322_d_n9;
        locals.var_rs_dn10 = assign28720_e22322_d_n10;
        locals.var_rs_dn11 = assign28720_e22322_d_n11;
        locals.var_rs_dn12 = assign28720_e22322_d_n12;

        let (assign28730_e22326, assign28730_e22326_d_n3, assign28730_e22326_d_n4, assign28730_e22326_d_n5, assign28730_e22326_d_n6, assign28730_e22326_d_n7, assign28730_e22326_d_n8, assign28730_e22326_d_n9, assign28730_e22326_d_n10, assign28730_e22326_d_n11, assign28730_e22326_d_n12,) = {
    if (locals.var_guard1684 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd, locals.var_rd_dn3, locals.var_rd_dn4, locals.var_rd_dn5, locals.var_rd_dn6, locals.var_rd_dn7, locals.var_rd_dn8, locals.var_rd_dn9, locals.var_rd_dn10, locals.var_rd_dn11, locals.var_rd_dn12,)
    }
};
        locals.var_rd = assign28730_e22326;
        locals.var_rd_dn3 = assign28730_e22326_d_n3;
        locals.var_rd_dn4 = assign28730_e22326_d_n4;
        locals.var_rd_dn5 = assign28730_e22326_d_n5;
        locals.var_rd_dn6 = assign28730_e22326_d_n6;
        locals.var_rd_dn7 = assign28730_e22326_d_n7;
        locals.var_rd_dn8 = assign28730_e22326_d_n8;
        locals.var_rd_dn9 = assign28730_e22326_d_n9;
        locals.var_rd_dn10 = assign28730_e22326_d_n10;
        locals.var_rd_dn11 = assign28730_e22326_d_n11;
        locals.var_rd_dn12 = assign28730_e22326_d_n12;

        let assign28740_e22331: f64 = (0.5 * locals.var_abulk);
        let assign28740_e22333: f64 = (assign28740_e22331 * locals.var_vdseff);
        let assign28740_e22335: f64 = (assign28740_e22333 / locals.var_vgst2vtm);
        let assign28740_e22336: f64 = (1.0 - assign28740_e22335);
        let assign28740_e22337: f64 = (locals.var_vgsteff__blk1175 * assign28740_e22336);
        locals.var_t1__blk1145 = assign28740_e22337;
        locals.var_t1__blk1145_dn3 = ((locals.var_vgsteff__blk1175_dn3 * assign28740_e22336) + (locals.var_vgsteff__blk1175 * (-((((((0.5 * locals.var_abulk_dn3) * locals.var_vdseff) + (assign28740_e22331 * locals.var_vdseff_dn3)) * locals.var_vgst2vtm) - (assign28740_e22333 * locals.var_vgst2vtm_dn3)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk1145_dn4 = ((locals.var_vgsteff__blk1175_dn4 * assign28740_e22336) + (locals.var_vgsteff__blk1175 * (-((((((0.5 * locals.var_abulk_dn4) * locals.var_vdseff) + (assign28740_e22331 * locals.var_vdseff_dn4)) * locals.var_vgst2vtm) - (assign28740_e22333 * locals.var_vgst2vtm_dn4)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk1145_dn5 = ((locals.var_vgsteff__blk1175_dn5 * assign28740_e22336) + (locals.var_vgsteff__blk1175 * (-((((((0.5 * locals.var_abulk_dn5) * locals.var_vdseff) + (assign28740_e22331 * locals.var_vdseff_dn5)) * locals.var_vgst2vtm) - (assign28740_e22333 * locals.var_vgst2vtm_dn5)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk1145_dn6 = ((locals.var_vgsteff__blk1175_dn6 * assign28740_e22336) + (locals.var_vgsteff__blk1175 * (-((((((0.5 * locals.var_abulk_dn6) * locals.var_vdseff) + (assign28740_e22331 * locals.var_vdseff_dn6)) * locals.var_vgst2vtm) - (assign28740_e22333 * locals.var_vgst2vtm_dn6)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk1145_dn7 = ((locals.var_vgsteff__blk1175_dn7 * assign28740_e22336) + (locals.var_vgsteff__blk1175 * (-((((((0.5 * locals.var_abulk_dn7) * locals.var_vdseff) + (assign28740_e22331 * locals.var_vdseff_dn7)) * locals.var_vgst2vtm) - (assign28740_e22333 * locals.var_vgst2vtm_dn7)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk1145_dn8 = ((locals.var_vgsteff__blk1175_dn8 * assign28740_e22336) + (locals.var_vgsteff__blk1175 * (-((((((0.5 * locals.var_abulk_dn8) * locals.var_vdseff) + (assign28740_e22331 * locals.var_vdseff_dn8)) * locals.var_vgst2vtm) - (assign28740_e22333 * locals.var_vgst2vtm_dn8)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk1145_dn9 = ((locals.var_vgsteff__blk1175_dn9 * assign28740_e22336) + (locals.var_vgsteff__blk1175 * (-((((((0.5 * locals.var_abulk_dn9) * locals.var_vdseff) + (assign28740_e22331 * locals.var_vdseff_dn9)) * locals.var_vgst2vtm) - (assign28740_e22333 * locals.var_vgst2vtm_dn9)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk1145_dn10 = ((locals.var_vgsteff__blk1175_dn10 * assign28740_e22336) + (locals.var_vgsteff__blk1175 * (-((((((0.5 * locals.var_abulk_dn10) * locals.var_vdseff) + (assign28740_e22331 * locals.var_vdseff_dn10)) * locals.var_vgst2vtm) - (assign28740_e22333 * locals.var_vgst2vtm_dn10)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk1145_dn11 = ((locals.var_vgsteff__blk1175_dn11 * assign28740_e22336) + (locals.var_vgsteff__blk1175 * (-((((((0.5 * locals.var_abulk_dn11) * locals.var_vdseff) + (assign28740_e22331 * locals.var_vdseff_dn11)) * locals.var_vgst2vtm) - (assign28740_e22333 * locals.var_vgst2vtm_dn11)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk1145_dn12 = ((locals.var_vgsteff__blk1175_dn12 * assign28740_e22336) + (locals.var_vgsteff__blk1175 * (-((((((0.5 * locals.var_abulk_dn12) * locals.var_vdseff) + (assign28740_e22331 * locals.var_vdseff_dn12)) * locals.var_vgst2vtm) - (assign28740_e22333 * locals.var_vgst2vtm_dn12)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));

        let assign28760_e22350: f64 = if locals.var_b4soinf != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1685 = assign28760_e22350;

        let (assign28770_e22356, assign28770_e22356_d_n3, assign28770_e22356_d_n4, assign28770_e22356_d_n5, assign28770_e22356_d_n6, assign28770_e22356_d_n7, assign28770_e22356_d_n8, assign28770_e22356_d_n9, assign28770_e22356_d_n10, assign28770_e22356_d_n11, assign28770_e22356_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28770_e22354: f64 = (locals.var_ids_1 * locals.var_b4soinf);
        (assign28770_e22354, (locals.var_ids_1_dn3 * locals.var_b4soinf), (locals.var_ids_1_dn4 * locals.var_b4soinf), (locals.var_ids_1_dn5 * locals.var_b4soinf), (locals.var_ids_1_dn6 * locals.var_b4soinf), (locals.var_ids_1_dn7 * locals.var_b4soinf), (locals.var_ids_1_dn8 * locals.var_b4soinf), (locals.var_ids_1_dn9 * locals.var_b4soinf), (locals.var_ids_1_dn10 * locals.var_b4soinf), (locals.var_ids_1_dn11 * locals.var_b4soinf), (locals.var_ids_1_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_ids_1, locals.var_ids_1_dn3, locals.var_ids_1_dn4, locals.var_ids_1_dn5, locals.var_ids_1_dn6, locals.var_ids_1_dn7, locals.var_ids_1_dn8, locals.var_ids_1_dn9, locals.var_ids_1_dn10, locals.var_ids_1_dn11, locals.var_ids_1_dn12,)
    }
};
        locals.var_ids_1 = assign28770_e22356;
        locals.var_ids_1_dn3 = assign28770_e22356_d_n3;
        locals.var_ids_1_dn4 = assign28770_e22356_d_n4;
        locals.var_ids_1_dn5 = assign28770_e22356_d_n5;
        locals.var_ids_1_dn6 = assign28770_e22356_d_n6;
        locals.var_ids_1_dn7 = assign28770_e22356_d_n7;
        locals.var_ids_1_dn8 = assign28770_e22356_d_n8;
        locals.var_ids_1_dn9 = assign28770_e22356_d_n9;
        locals.var_ids_1_dn10 = assign28770_e22356_d_n10;
        locals.var_ids_1_dn11 = assign28770_e22356_d_n11;
        locals.var_ids_1_dn12 = assign28770_e22356_d_n12;

        let (assign28780_e22362, assign28780_e22362_d_n3, assign28780_e22362_d_n4, assign28780_e22362_d_n5, assign28780_e22362_d_n6, assign28780_e22362_d_n7, assign28780_e22362_d_n8, assign28780_e22362_d_n9, assign28780_e22362_d_n10, assign28780_e22362_d_n11, assign28780_e22362_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28780_e22360: f64 = (locals.var_ic_1 * locals.var_b4soinf);
        (assign28780_e22360, (locals.var_ic_1_dn3 * locals.var_b4soinf), (locals.var_ic_1_dn4 * locals.var_b4soinf), (locals.var_ic_1_dn5 * locals.var_b4soinf), (locals.var_ic_1_dn6 * locals.var_b4soinf), (locals.var_ic_1_dn7 * locals.var_b4soinf), (locals.var_ic_1_dn8 * locals.var_b4soinf), (locals.var_ic_1_dn9 * locals.var_b4soinf), (locals.var_ic_1_dn10 * locals.var_b4soinf), (locals.var_ic_1_dn11 * locals.var_b4soinf), (locals.var_ic_1_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_ic_1, locals.var_ic_1_dn3, locals.var_ic_1_dn4, locals.var_ic_1_dn5, locals.var_ic_1_dn6, locals.var_ic_1_dn7, locals.var_ic_1_dn8, locals.var_ic_1_dn9, locals.var_ic_1_dn10, locals.var_ic_1_dn11, locals.var_ic_1_dn12,)
    }
};
        locals.var_ic_1 = assign28780_e22362;
        locals.var_ic_1_dn3 = assign28780_e22362_d_n3;
        locals.var_ic_1_dn4 = assign28780_e22362_d_n4;
        locals.var_ic_1_dn5 = assign28780_e22362_d_n5;
        locals.var_ic_1_dn6 = assign28780_e22362_d_n6;
        locals.var_ic_1_dn7 = assign28780_e22362_d_n7;
        locals.var_ic_1_dn8 = assign28780_e22362_d_n8;
        locals.var_ic_1_dn9 = assign28780_e22362_d_n9;
        locals.var_ic_1_dn10 = assign28780_e22362_d_n10;
        locals.var_ic_1_dn11 = assign28780_e22362_d_n11;
        locals.var_ic_1_dn12 = assign28780_e22362_d_n12;

        let (assign28790_e22368, assign28790_e22368_d_n3, assign28790_e22368_d_n4, assign28790_e22368_d_n5, assign28790_e22368_d_n6, assign28790_e22368_d_n7, assign28790_e22368_d_n8, assign28790_e22368_d_n9, assign28790_e22368_d_n10, assign28790_e22368_d_n11, assign28790_e22368_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28790_e22366: f64 = (locals.var_b4soiidovvds * locals.var_b4soinf);
        (assign28790_e22366, (locals.var_b4soiidovvds_dn3 * locals.var_b4soinf), (locals.var_b4soiidovvds_dn4 * locals.var_b4soinf), (locals.var_b4soiidovvds_dn5 * locals.var_b4soinf), (locals.var_b4soiidovvds_dn6 * locals.var_b4soinf), (locals.var_b4soiidovvds_dn7 * locals.var_b4soinf), (locals.var_b4soiidovvds_dn8 * locals.var_b4soinf), (locals.var_b4soiidovvds_dn9 * locals.var_b4soinf), (locals.var_b4soiidovvds_dn10 * locals.var_b4soinf), (locals.var_b4soiidovvds_dn11 * locals.var_b4soinf), (locals.var_b4soiidovvds_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_b4soiidovvds, locals.var_b4soiidovvds_dn3, locals.var_b4soiidovvds_dn4, locals.var_b4soiidovvds_dn5, locals.var_b4soiidovvds_dn6, locals.var_b4soiidovvds_dn7, locals.var_b4soiidovvds_dn8, locals.var_b4soiidovvds_dn9, locals.var_b4soiidovvds_dn10, locals.var_b4soiidovvds_dn11, locals.var_b4soiidovvds_dn12,)
    }
};
        locals.var_b4soiidovvds = assign28790_e22368;
        locals.var_b4soiidovvds_dn3 = assign28790_e22368_d_n3;
        locals.var_b4soiidovvds_dn4 = assign28790_e22368_d_n4;
        locals.var_b4soiidovvds_dn5 = assign28790_e22368_d_n5;
        locals.var_b4soiidovvds_dn6 = assign28790_e22368_d_n6;
        locals.var_b4soiidovvds_dn7 = assign28790_e22368_d_n7;
        locals.var_b4soiidovvds_dn8 = assign28790_e22368_d_n8;
        locals.var_b4soiidovvds_dn9 = assign28790_e22368_d_n9;
        locals.var_b4soiidovvds_dn10 = assign28790_e22368_d_n10;
        locals.var_b4soiidovvds_dn11 = assign28790_e22368_d_n11;
        locals.var_b4soiidovvds_dn12 = assign28790_e22368_d_n12;

        let (assign28800_e22374, assign28800_e22374_d_n3, assign28800_e22374_d_n4, assign28800_e22374_d_n5, assign28800_e22374_d_n6, assign28800_e22374_d_n7, assign28800_e22374_d_n8, assign28800_e22374_d_n9, assign28800_e22374_d_n10, assign28800_e22374_d_n11, assign28800_e22374_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28800_e22372: f64 = (locals.var_ibs_1 * locals.var_b4soinf);
        (assign28800_e22372, (locals.var_ibs_1_dn3 * locals.var_b4soinf), (locals.var_ibs_1_dn4 * locals.var_b4soinf), (locals.var_ibs_1_dn5 * locals.var_b4soinf), (locals.var_ibs_1_dn6 * locals.var_b4soinf), (locals.var_ibs_1_dn7 * locals.var_b4soinf), (locals.var_ibs_1_dn8 * locals.var_b4soinf), (locals.var_ibs_1_dn9 * locals.var_b4soinf), (locals.var_ibs_1_dn10 * locals.var_b4soinf), (locals.var_ibs_1_dn11 * locals.var_b4soinf), (locals.var_ibs_1_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_ibs_1, locals.var_ibs_1_dn3, locals.var_ibs_1_dn4, locals.var_ibs_1_dn5, locals.var_ibs_1_dn6, locals.var_ibs_1_dn7, locals.var_ibs_1_dn8, locals.var_ibs_1_dn9, locals.var_ibs_1_dn10, locals.var_ibs_1_dn11, locals.var_ibs_1_dn12,)
    }
};
        locals.var_ibs_1 = assign28800_e22374;
        locals.var_ibs_1_dn3 = assign28800_e22374_d_n3;
        locals.var_ibs_1_dn4 = assign28800_e22374_d_n4;
        locals.var_ibs_1_dn5 = assign28800_e22374_d_n5;
        locals.var_ibs_1_dn6 = assign28800_e22374_d_n6;
        locals.var_ibs_1_dn7 = assign28800_e22374_d_n7;
        locals.var_ibs_1_dn8 = assign28800_e22374_d_n8;
        locals.var_ibs_1_dn9 = assign28800_e22374_d_n9;
        locals.var_ibs_1_dn10 = assign28800_e22374_d_n10;
        locals.var_ibs_1_dn11 = assign28800_e22374_d_n11;
        locals.var_ibs_1_dn12 = assign28800_e22374_d_n12;

        let (assign28810_e22380, assign28810_e22380_d_n3, assign28810_e22380_d_n4, assign28810_e22380_d_n5, assign28810_e22380_d_n6, assign28810_e22380_d_n7, assign28810_e22380_d_n8, assign28810_e22380_d_n9, assign28810_e22380_d_n10, assign28810_e22380_d_n11, assign28810_e22380_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28810_e22378: f64 = (locals.var_ibd_1 * locals.var_b4soinf);
        (assign28810_e22378, (locals.var_ibd_1_dn3 * locals.var_b4soinf), (locals.var_ibd_1_dn4 * locals.var_b4soinf), (locals.var_ibd_1_dn5 * locals.var_b4soinf), (locals.var_ibd_1_dn6 * locals.var_b4soinf), (locals.var_ibd_1_dn7 * locals.var_b4soinf), (locals.var_ibd_1_dn8 * locals.var_b4soinf), (locals.var_ibd_1_dn9 * locals.var_b4soinf), (locals.var_ibd_1_dn10 * locals.var_b4soinf), (locals.var_ibd_1_dn11 * locals.var_b4soinf), (locals.var_ibd_1_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_ibd_1, locals.var_ibd_1_dn3, locals.var_ibd_1_dn4, locals.var_ibd_1_dn5, locals.var_ibd_1_dn6, locals.var_ibd_1_dn7, locals.var_ibd_1_dn8, locals.var_ibd_1_dn9, locals.var_ibd_1_dn10, locals.var_ibd_1_dn11, locals.var_ibd_1_dn12,)
    }
};
        locals.var_ibd_1 = assign28810_e22380;
        locals.var_ibd_1_dn3 = assign28810_e22380_d_n3;
        locals.var_ibd_1_dn4 = assign28810_e22380_d_n4;
        locals.var_ibd_1_dn5 = assign28810_e22380_d_n5;
        locals.var_ibd_1_dn6 = assign28810_e22380_d_n6;
        locals.var_ibd_1_dn7 = assign28810_e22380_d_n7;
        locals.var_ibd_1_dn8 = assign28810_e22380_d_n8;
        locals.var_ibd_1_dn9 = assign28810_e22380_d_n9;
        locals.var_ibd_1_dn10 = assign28810_e22380_d_n10;
        locals.var_ibd_1_dn11 = assign28810_e22380_d_n11;
        locals.var_ibd_1_dn12 = assign28810_e22380_d_n12;

        let (assign28820_e22386, assign28820_e22386_d_n3, assign28820_e22386_d_n4, assign28820_e22386_d_n5, assign28820_e22386_d_n6, assign28820_e22386_d_n7, assign28820_e22386_d_n8, assign28820_e22386_d_n9, assign28820_e22386_d_n10, assign28820_e22386_d_n11, assign28820_e22386_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28820_e22384: f64 = (locals.var_igcs_1 * locals.var_b4soinf);
        (assign28820_e22384, (locals.var_igcs_1_dn3 * locals.var_b4soinf), (locals.var_igcs_1_dn4 * locals.var_b4soinf), (locals.var_igcs_1_dn5 * locals.var_b4soinf), (locals.var_igcs_1_dn6 * locals.var_b4soinf), (locals.var_igcs_1_dn7 * locals.var_b4soinf), (locals.var_igcs_1_dn8 * locals.var_b4soinf), (locals.var_igcs_1_dn9 * locals.var_b4soinf), (locals.var_igcs_1_dn10 * locals.var_b4soinf), (locals.var_igcs_1_dn11 * locals.var_b4soinf), (locals.var_igcs_1_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_igcs_1, locals.var_igcs_1_dn3, locals.var_igcs_1_dn4, locals.var_igcs_1_dn5, locals.var_igcs_1_dn6, locals.var_igcs_1_dn7, locals.var_igcs_1_dn8, locals.var_igcs_1_dn9, locals.var_igcs_1_dn10, locals.var_igcs_1_dn11, locals.var_igcs_1_dn12,)
    }
};
        locals.var_igcs_1 = assign28820_e22386;
        locals.var_igcs_1_dn3 = assign28820_e22386_d_n3;
        locals.var_igcs_1_dn4 = assign28820_e22386_d_n4;
        locals.var_igcs_1_dn5 = assign28820_e22386_d_n5;
        locals.var_igcs_1_dn6 = assign28820_e22386_d_n6;
        locals.var_igcs_1_dn7 = assign28820_e22386_d_n7;
        locals.var_igcs_1_dn8 = assign28820_e22386_d_n8;
        locals.var_igcs_1_dn9 = assign28820_e22386_d_n9;
        locals.var_igcs_1_dn10 = assign28820_e22386_d_n10;
        locals.var_igcs_1_dn11 = assign28820_e22386_d_n11;
        locals.var_igcs_1_dn12 = assign28820_e22386_d_n12;

        let (assign28830_e22392, assign28830_e22392_d_n3, assign28830_e22392_d_n4, assign28830_e22392_d_n5, assign28830_e22392_d_n6, assign28830_e22392_d_n7, assign28830_e22392_d_n8, assign28830_e22392_d_n9, assign28830_e22392_d_n10, assign28830_e22392_d_n11, assign28830_e22392_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28830_e22390: f64 = (locals.var_igcd_1 * locals.var_b4soinf);
        (assign28830_e22390, (locals.var_igcd_1_dn3 * locals.var_b4soinf), (locals.var_igcd_1_dn4 * locals.var_b4soinf), (locals.var_igcd_1_dn5 * locals.var_b4soinf), (locals.var_igcd_1_dn6 * locals.var_b4soinf), (locals.var_igcd_1_dn7 * locals.var_b4soinf), (locals.var_igcd_1_dn8 * locals.var_b4soinf), (locals.var_igcd_1_dn9 * locals.var_b4soinf), (locals.var_igcd_1_dn10 * locals.var_b4soinf), (locals.var_igcd_1_dn11 * locals.var_b4soinf), (locals.var_igcd_1_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_igcd_1, locals.var_igcd_1_dn3, locals.var_igcd_1_dn4, locals.var_igcd_1_dn5, locals.var_igcd_1_dn6, locals.var_igcd_1_dn7, locals.var_igcd_1_dn8, locals.var_igcd_1_dn9, locals.var_igcd_1_dn10, locals.var_igcd_1_dn11, locals.var_igcd_1_dn12,)
    }
};
        locals.var_igcd_1 = assign28830_e22392;
        locals.var_igcd_1_dn3 = assign28830_e22392_d_n3;
        locals.var_igcd_1_dn4 = assign28830_e22392_d_n4;
        locals.var_igcd_1_dn5 = assign28830_e22392_d_n5;
        locals.var_igcd_1_dn6 = assign28830_e22392_d_n6;
        locals.var_igcd_1_dn7 = assign28830_e22392_d_n7;
        locals.var_igcd_1_dn8 = assign28830_e22392_d_n8;
        locals.var_igcd_1_dn9 = assign28830_e22392_d_n9;
        locals.var_igcd_1_dn10 = assign28830_e22392_d_n10;
        locals.var_igcd_1_dn11 = assign28830_e22392_d_n11;
        locals.var_igcd_1_dn12 = assign28830_e22392_d_n12;

        let (assign28840_e22398, assign28840_e22398_d_n3, assign28840_e22398_d_n4, assign28840_e22398_d_n5, assign28840_e22398_d_n6, assign28840_e22398_d_n7, assign28840_e22398_d_n8, assign28840_e22398_d_n9, assign28840_e22398_d_n10, assign28840_e22398_d_n11, assign28840_e22398_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28840_e22396: f64 = (locals.var_igs_1 * locals.var_b4soinf);
        (assign28840_e22396, (locals.var_igs_1_dn3 * locals.var_b4soinf), (locals.var_igs_1_dn4 * locals.var_b4soinf), (locals.var_igs_1_dn5 * locals.var_b4soinf), (locals.var_igs_1_dn6 * locals.var_b4soinf), (locals.var_igs_1_dn7 * locals.var_b4soinf), (locals.var_igs_1_dn8 * locals.var_b4soinf), (locals.var_igs_1_dn9 * locals.var_b4soinf), (locals.var_igs_1_dn10 * locals.var_b4soinf), (locals.var_igs_1_dn11 * locals.var_b4soinf), (locals.var_igs_1_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_igs_1, locals.var_igs_1_dn3, locals.var_igs_1_dn4, locals.var_igs_1_dn5, locals.var_igs_1_dn6, locals.var_igs_1_dn7, locals.var_igs_1_dn8, locals.var_igs_1_dn9, locals.var_igs_1_dn10, locals.var_igs_1_dn11, locals.var_igs_1_dn12,)
    }
};
        locals.var_igs_1 = assign28840_e22398;
        locals.var_igs_1_dn3 = assign28840_e22398_d_n3;
        locals.var_igs_1_dn4 = assign28840_e22398_d_n4;
        locals.var_igs_1_dn5 = assign28840_e22398_d_n5;
        locals.var_igs_1_dn6 = assign28840_e22398_d_n6;
        locals.var_igs_1_dn7 = assign28840_e22398_d_n7;
        locals.var_igs_1_dn8 = assign28840_e22398_d_n8;
        locals.var_igs_1_dn9 = assign28840_e22398_d_n9;
        locals.var_igs_1_dn10 = assign28840_e22398_d_n10;
        locals.var_igs_1_dn11 = assign28840_e22398_d_n11;
        locals.var_igs_1_dn12 = assign28840_e22398_d_n12;

        let (assign28850_e22404, assign28850_e22404_d_n3, assign28850_e22404_d_n4, assign28850_e22404_d_n5, assign28850_e22404_d_n6, assign28850_e22404_d_n7, assign28850_e22404_d_n8, assign28850_e22404_d_n9, assign28850_e22404_d_n10, assign28850_e22404_d_n11, assign28850_e22404_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28850_e22402: f64 = (locals.var_igd_1 * locals.var_b4soinf);
        (assign28850_e22402, (locals.var_igd_1_dn3 * locals.var_b4soinf), (locals.var_igd_1_dn4 * locals.var_b4soinf), (locals.var_igd_1_dn5 * locals.var_b4soinf), (locals.var_igd_1_dn6 * locals.var_b4soinf), (locals.var_igd_1_dn7 * locals.var_b4soinf), (locals.var_igd_1_dn8 * locals.var_b4soinf), (locals.var_igd_1_dn9 * locals.var_b4soinf), (locals.var_igd_1_dn10 * locals.var_b4soinf), (locals.var_igd_1_dn11 * locals.var_b4soinf), (locals.var_igd_1_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_igd_1, locals.var_igd_1_dn3, locals.var_igd_1_dn4, locals.var_igd_1_dn5, locals.var_igd_1_dn6, locals.var_igd_1_dn7, locals.var_igd_1_dn8, locals.var_igd_1_dn9, locals.var_igd_1_dn10, locals.var_igd_1_dn11, locals.var_igd_1_dn12,)
    }
};
        locals.var_igd_1 = assign28850_e22404;
        locals.var_igd_1_dn3 = assign28850_e22404_d_n3;
        locals.var_igd_1_dn4 = assign28850_e22404_d_n4;
        locals.var_igd_1_dn5 = assign28850_e22404_d_n5;
        locals.var_igd_1_dn6 = assign28850_e22404_d_n6;
        locals.var_igd_1_dn7 = assign28850_e22404_d_n7;
        locals.var_igd_1_dn8 = assign28850_e22404_d_n8;
        locals.var_igd_1_dn9 = assign28850_e22404_d_n9;
        locals.var_igd_1_dn10 = assign28850_e22404_d_n10;
        locals.var_igd_1_dn11 = assign28850_e22404_d_n11;
        locals.var_igd_1_dn12 = assign28850_e22404_d_n12;

        let (assign28860_e22410, assign28860_e22410_d_n3, assign28860_e22410_d_n4, assign28860_e22410_d_n5, assign28860_e22410_d_n6, assign28860_e22410_d_n7, assign28860_e22410_d_n8, assign28860_e22410_d_n9, assign28860_e22410_d_n10, assign28860_e22410_d_n11, assign28860_e22410_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28860_e22408: f64 = (locals.var_iii * locals.var_b4soinf);
        (assign28860_e22408, (locals.var_iii_dn3 * locals.var_b4soinf), (locals.var_iii_dn4 * locals.var_b4soinf), (locals.var_iii_dn5 * locals.var_b4soinf), (locals.var_iii_dn6 * locals.var_b4soinf), (locals.var_iii_dn7 * locals.var_b4soinf), (locals.var_iii_dn8 * locals.var_b4soinf), (locals.var_iii_dn9 * locals.var_b4soinf), (locals.var_iii_dn10 * locals.var_b4soinf), (locals.var_iii_dn11 * locals.var_b4soinf), (locals.var_iii_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11, locals.var_iii_dn12,)
    }
};
        locals.var_iii = assign28860_e22410;
        locals.var_iii_dn3 = assign28860_e22410_d_n3;
        locals.var_iii_dn4 = assign28860_e22410_d_n4;
        locals.var_iii_dn5 = assign28860_e22410_d_n5;
        locals.var_iii_dn6 = assign28860_e22410_d_n6;
        locals.var_iii_dn7 = assign28860_e22410_d_n7;
        locals.var_iii_dn8 = assign28860_e22410_d_n8;
        locals.var_iii_dn9 = assign28860_e22410_d_n9;
        locals.var_iii_dn10 = assign28860_e22410_d_n10;
        locals.var_iii_dn11 = assign28860_e22410_d_n11;
        locals.var_iii_dn12 = assign28860_e22410_d_n12;

        let (assign28870_e22416, assign28870_e22416_d_n3, assign28870_e22416_d_n4, assign28870_e22416_d_n5, assign28870_e22416_d_n6, assign28870_e22416_d_n7, assign28870_e22416_d_n8, assign28870_e22416_d_n9, assign28870_e22416_d_n10, assign28870_e22416_d_n11, assign28870_e22416_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28870_e22414: f64 = (locals.var_b4soiig * locals.var_b4soinf);
        (assign28870_e22414, (locals.var_b4soiig_dn3 * locals.var_b4soinf), (locals.var_b4soiig_dn4 * locals.var_b4soinf), (locals.var_b4soiig_dn5 * locals.var_b4soinf), (locals.var_b4soiig_dn6 * locals.var_b4soinf), (locals.var_b4soiig_dn7 * locals.var_b4soinf), (locals.var_b4soiig_dn8 * locals.var_b4soinf), (locals.var_b4soiig_dn9 * locals.var_b4soinf), (locals.var_b4soiig_dn10 * locals.var_b4soinf), (locals.var_b4soiig_dn11 * locals.var_b4soinf), (locals.var_b4soiig_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_b4soiig, locals.var_b4soiig_dn3, locals.var_b4soiig_dn4, locals.var_b4soiig_dn5, locals.var_b4soiig_dn6, locals.var_b4soiig_dn7, locals.var_b4soiig_dn8, locals.var_b4soiig_dn9, locals.var_b4soiig_dn10, locals.var_b4soiig_dn11, locals.var_b4soiig_dn12,)
    }
};
        locals.var_b4soiig = assign28870_e22416;
        locals.var_b4soiig_dn3 = assign28870_e22416_d_n3;
        locals.var_b4soiig_dn4 = assign28870_e22416_d_n4;
        locals.var_b4soiig_dn5 = assign28870_e22416_d_n5;
        locals.var_b4soiig_dn6 = assign28870_e22416_d_n6;
        locals.var_b4soiig_dn7 = assign28870_e22416_d_n7;
        locals.var_b4soiig_dn8 = assign28870_e22416_d_n8;
        locals.var_b4soiig_dn9 = assign28870_e22416_d_n9;
        locals.var_b4soiig_dn10 = assign28870_e22416_d_n10;
        locals.var_b4soiig_dn11 = assign28870_e22416_d_n11;
        locals.var_b4soiig_dn12 = assign28870_e22416_d_n12;

        let (assign28880_e22422, assign28880_e22422_d_n3, assign28880_e22422_d_n4, assign28880_e22422_d_n5, assign28880_e22422_d_n6, assign28880_e22422_d_n7, assign28880_e22422_d_n8, assign28880_e22422_d_n9, assign28880_e22422_d_n10, assign28880_e22422_d_n11, assign28880_e22422_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28880_e22420: f64 = (locals.var_igidl_1 * locals.var_b4soinf);
        (assign28880_e22420, (locals.var_igidl_1_dn3 * locals.var_b4soinf), (locals.var_igidl_1_dn4 * locals.var_b4soinf), (locals.var_igidl_1_dn5 * locals.var_b4soinf), (locals.var_igidl_1_dn6 * locals.var_b4soinf), (locals.var_igidl_1_dn7 * locals.var_b4soinf), (locals.var_igidl_1_dn8 * locals.var_b4soinf), (locals.var_igidl_1_dn9 * locals.var_b4soinf), (locals.var_igidl_1_dn10 * locals.var_b4soinf), (locals.var_igidl_1_dn11 * locals.var_b4soinf), (locals.var_igidl_1_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_igidl_1, locals.var_igidl_1_dn3, locals.var_igidl_1_dn4, locals.var_igidl_1_dn5, locals.var_igidl_1_dn6, locals.var_igidl_1_dn7, locals.var_igidl_1_dn8, locals.var_igidl_1_dn9, locals.var_igidl_1_dn10, locals.var_igidl_1_dn11, locals.var_igidl_1_dn12,)
    }
};
        locals.var_igidl_1 = assign28880_e22422;
        locals.var_igidl_1_dn3 = assign28880_e22422_d_n3;
        locals.var_igidl_1_dn4 = assign28880_e22422_d_n4;
        locals.var_igidl_1_dn5 = assign28880_e22422_d_n5;
        locals.var_igidl_1_dn6 = assign28880_e22422_d_n6;
        locals.var_igidl_1_dn7 = assign28880_e22422_d_n7;
        locals.var_igidl_1_dn8 = assign28880_e22422_d_n8;
        locals.var_igidl_1_dn9 = assign28880_e22422_d_n9;
        locals.var_igidl_1_dn10 = assign28880_e22422_d_n10;
        locals.var_igidl_1_dn11 = assign28880_e22422_d_n11;
        locals.var_igidl_1_dn12 = assign28880_e22422_d_n12;

        let (assign28890_e22428, assign28890_e22428_d_n3, assign28890_e22428_d_n4, assign28890_e22428_d_n5, assign28890_e22428_d_n6, assign28890_e22428_d_n7, assign28890_e22428_d_n8, assign28890_e22428_d_n9, assign28890_e22428_d_n10, assign28890_e22428_d_n11, assign28890_e22428_d_n12,) = {
    if (locals.var_guard1685 != 0.0) {
        let assign28890_e22426: f64 = (locals.var_igisl_1 * locals.var_b4soinf);
        (assign28890_e22426, (locals.var_igisl_1_dn3 * locals.var_b4soinf), (locals.var_igisl_1_dn4 * locals.var_b4soinf), (locals.var_igisl_1_dn5 * locals.var_b4soinf), (locals.var_igisl_1_dn6 * locals.var_b4soinf), (locals.var_igisl_1_dn7 * locals.var_b4soinf), (locals.var_igisl_1_dn8 * locals.var_b4soinf), (locals.var_igisl_1_dn9 * locals.var_b4soinf), (locals.var_igisl_1_dn10 * locals.var_b4soinf), (locals.var_igisl_1_dn11 * locals.var_b4soinf), (locals.var_igisl_1_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_igisl_1, locals.var_igisl_1_dn3, locals.var_igisl_1_dn4, locals.var_igisl_1_dn5, locals.var_igisl_1_dn6, locals.var_igisl_1_dn7, locals.var_igisl_1_dn8, locals.var_igisl_1_dn9, locals.var_igisl_1_dn10, locals.var_igisl_1_dn11, locals.var_igisl_1_dn12,)
    }
};
        locals.var_igisl_1 = assign28890_e22428;
        locals.var_igisl_1_dn3 = assign28890_e22428_d_n3;
        locals.var_igisl_1_dn4 = assign28890_e22428_d_n4;
        locals.var_igisl_1_dn5 = assign28890_e22428_d_n5;
        locals.var_igisl_1_dn6 = assign28890_e22428_d_n6;
        locals.var_igisl_1_dn7 = assign28890_e22428_d_n7;
        locals.var_igisl_1_dn8 = assign28890_e22428_d_n8;
        locals.var_igisl_1_dn9 = assign28890_e22428_d_n9;
        locals.var_igisl_1_dn10 = assign28890_e22428_d_n10;
        locals.var_igisl_1_dn11 = assign28890_e22428_d_n11;
        locals.var_igisl_1_dn12 = assign28890_e22428_d_n12;

        let assign28900_e22432: f64 = locals.var_ids_1_dn9;
        let assign28900_e22433: f64 = (locals.var_b4soitype * assign28900_e22432);
        locals.var_b4soigm = assign28900_e22433;
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

        let assign28910_e22436: f64 = if locals.var_b4soimode > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1686 = assign28910_e22436;

    }

    pub(super) fn stamp_transient_block_77(
        locals: &mut StampLocals,
    ) {
        let (assign28920_e22444, assign28920_e22444_d_n3, assign28920_e22444_d_n4, assign28920_e22444_d_n5, assign28920_e22444_d_n6, assign28920_e22444_d_n7, assign28920_e22444_d_n8, assign28920_e22444_d_n9, assign28920_e22444_d_n10, assign28920_e22444_d_n11, assign28920_e22444_d_n12,) = {
    if (locals.var_guard1686 != 0.0) {
        let assign28920_e22441: f64 = locals.var_ids_1_dn7;
        let assign28920_e22442: f64 = (locals.var_b4soitype * assign28920_e22441);
        (assign28920_e22442, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soigds, locals.var_b4soigds_dn3, locals.var_b4soigds_dn4, locals.var_b4soigds_dn5, locals.var_b4soigds_dn6, locals.var_b4soigds_dn7, locals.var_b4soigds_dn8, locals.var_b4soigds_dn9, locals.var_b4soigds_dn10, locals.var_b4soigds_dn11, locals.var_b4soigds_dn12,)
    }
};
        locals.var_b4soigds = assign28920_e22444;
        locals.var_b4soigds_dn3 = assign28920_e22444_d_n3;
        locals.var_b4soigds_dn4 = assign28920_e22444_d_n4;
        locals.var_b4soigds_dn5 = assign28920_e22444_d_n5;
        locals.var_b4soigds_dn6 = assign28920_e22444_d_n6;
        locals.var_b4soigds_dn7 = assign28920_e22444_d_n7;
        locals.var_b4soigds_dn8 = assign28920_e22444_d_n8;
        locals.var_b4soigds_dn9 = assign28920_e22444_d_n9;
        locals.var_b4soigds_dn10 = assign28920_e22444_d_n10;
        locals.var_b4soigds_dn11 = assign28920_e22444_d_n11;
        locals.var_b4soigds_dn12 = assign28920_e22444_d_n12;

        let (assign28930_e22453, assign28930_e22453_d_n3, assign28930_e22453_d_n4, assign28930_e22453_d_n5, assign28930_e22453_d_n6, assign28930_e22453_d_n7, assign28930_e22453_d_n8, assign28930_e22453_d_n9, assign28930_e22453_d_n10, assign28930_e22453_d_n11, assign28930_e22453_d_n12,) = {
    if (locals.var_guard1686 == 0.0) {
        let assign28930_e22450: f64 = locals.var_ids_1_dn8;
        let assign28930_e22451: f64 = (locals.var_b4soitype * assign28930_e22450);
        (assign28930_e22451, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soigds, locals.var_b4soigds_dn3, locals.var_b4soigds_dn4, locals.var_b4soigds_dn5, locals.var_b4soigds_dn6, locals.var_b4soigds_dn7, locals.var_b4soigds_dn8, locals.var_b4soigds_dn9, locals.var_b4soigds_dn10, locals.var_b4soigds_dn11, locals.var_b4soigds_dn12,)
    }
};
        locals.var_b4soigds = assign28930_e22453;
        locals.var_b4soigds_dn3 = assign28930_e22453_d_n3;
        locals.var_b4soigds_dn4 = assign28930_e22453_d_n4;
        locals.var_b4soigds_dn5 = assign28930_e22453_d_n5;
        locals.var_b4soigds_dn6 = assign28930_e22453_d_n6;
        locals.var_b4soigds_dn7 = assign28930_e22453_d_n7;
        locals.var_b4soigds_dn8 = assign28930_e22453_d_n8;
        locals.var_b4soigds_dn9 = assign28930_e22453_d_n9;
        locals.var_b4soigds_dn10 = assign28930_e22453_d_n10;
        locals.var_b4soigds_dn11 = assign28930_e22453_d_n11;
        locals.var_b4soigds_dn12 = assign28930_e22453_d_n12;

        let assign28940_e22457: f64 = locals.var_ids_1_dn5;
        let assign28940_e22458: f64 = (locals.var_b4soitype * assign28940_e22457);
        locals.var_b4soigmbs = assign28940_e22458;
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

        let assign28950_e22462: f64 = (locals.var_pparam_b4soiweffcv / locals.var_b4soinseg);
        let assign28950_e22464: f64 = (assign28950_e22462 * locals.var_b4soinf);
        let assign28950_e22466: f64 = (assign28950_e22464 * locals.var_pparam_b4soileffcv);
        let assign28950_e22468: f64 = (assign28950_e22466 + locals.var_b4soiagbcp);
        let assign28950_e22469: f64 = (locals.var_b4soicox * assign28950_e22468);
        locals.var_coxwl = assign28950_e22469;
        locals.var_coxwl_dn3 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn3 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcv) + (assign28950_e22464 * locals.var_pparam_b4soileffcv_dn3)));
        locals.var_coxwl_dn4 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn4 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcv) + (assign28950_e22464 * locals.var_pparam_b4soileffcv_dn4)));
        locals.var_coxwl_dn5 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn5 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcv) + (assign28950_e22464 * locals.var_pparam_b4soileffcv_dn5)));
        locals.var_coxwl_dn6 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn6 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcv) + (assign28950_e22464 * locals.var_pparam_b4soileffcv_dn6)));
        locals.var_coxwl_dn7 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn7 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcv) + (assign28950_e22464 * locals.var_pparam_b4soileffcv_dn7)));
        locals.var_coxwl_dn8 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn8 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcv) + (assign28950_e22464 * locals.var_pparam_b4soileffcv_dn8)));
        locals.var_coxwl_dn9 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn9 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcv) + (assign28950_e22464 * locals.var_pparam_b4soileffcv_dn9)));
        locals.var_coxwl_dn10 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn10 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcv) + (assign28950_e22464 * locals.var_pparam_b4soileffcv_dn10)));
        locals.var_coxwl_dn11 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn11 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcv) + (assign28950_e22464 * locals.var_pparam_b4soileffcv_dn11)));
        locals.var_coxwl_dn12 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn12 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcv) + (assign28950_e22464 * locals.var_pparam_b4soileffcv_dn12)));

        let assign28960_e22472: f64 = (locals.var_b4soifbody * locals.var_b4soicox);
        let assign28960_e22475: f64 = (locals.var_pparam_b4soiweffcv / locals.var_b4soinseg);
        let assign28960_e22477: f64 = (assign28960_e22475 * locals.var_b4soinf);
        let assign28960_e22479: f64 = (assign28960_e22477 * locals.var_pparam_b4soileffcvb);
        let assign28960_e22481: f64 = (assign28960_e22479 + locals.var_b4soiagbcp);
        let assign28960_e22482: f64 = (assign28960_e22472 * assign28960_e22481);
        locals.var_coxwlb = assign28960_e22482;
        locals.var_coxwlb_dn3 = (assign28960_e22472 * ((((locals.var_pparam_b4soiweffcv_dn3 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvb) + (assign28960_e22477 * locals.var_pparam_b4soileffcvb_dn3)));
        locals.var_coxwlb_dn4 = (assign28960_e22472 * ((((locals.var_pparam_b4soiweffcv_dn4 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvb) + (assign28960_e22477 * locals.var_pparam_b4soileffcvb_dn4)));
        locals.var_coxwlb_dn5 = (assign28960_e22472 * ((((locals.var_pparam_b4soiweffcv_dn5 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvb) + (assign28960_e22477 * locals.var_pparam_b4soileffcvb_dn5)));
        locals.var_coxwlb_dn6 = (assign28960_e22472 * ((((locals.var_pparam_b4soiweffcv_dn6 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvb) + (assign28960_e22477 * locals.var_pparam_b4soileffcvb_dn6)));
        locals.var_coxwlb_dn7 = (assign28960_e22472 * ((((locals.var_pparam_b4soiweffcv_dn7 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvb) + (assign28960_e22477 * locals.var_pparam_b4soileffcvb_dn7)));
        locals.var_coxwlb_dn8 = (assign28960_e22472 * ((((locals.var_pparam_b4soiweffcv_dn8 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvb) + (assign28960_e22477 * locals.var_pparam_b4soileffcvb_dn8)));
        locals.var_coxwlb_dn9 = (assign28960_e22472 * ((((locals.var_pparam_b4soiweffcv_dn9 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvb) + (assign28960_e22477 * locals.var_pparam_b4soileffcvb_dn9)));
        locals.var_coxwlb_dn10 = (assign28960_e22472 * ((((locals.var_pparam_b4soiweffcv_dn10 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvb) + (assign28960_e22477 * locals.var_pparam_b4soileffcvb_dn10)));
        locals.var_coxwlb_dn11 = (assign28960_e22472 * ((((locals.var_pparam_b4soiweffcv_dn11 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvb) + (assign28960_e22477 * locals.var_pparam_b4soileffcvb_dn11)));
        locals.var_coxwlb_dn12 = (assign28960_e22472 * ((((locals.var_pparam_b4soiweffcv_dn12 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvb) + (assign28960_e22477 * locals.var_pparam_b4soileffcvb_dn12)));

        let assign28970_e22485: f64 = (locals.var_b4soicox * locals.var_b4soiagbcp2);
        locals.var_coxwl2 = assign28970_e22485;
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

        let assign28980_e22488: f64 = (locals.var_b4soifbody * locals.var_b4soicox);
        let assign28980_e22490: f64 = (assign28980_e22488 * locals.var_b4soiagbcp2);
        locals.var_coxwlb2 = assign28980_e22490;
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

        let assign28990_e22493: f64 = (locals.var_vgs_eff__blk1126 - locals.var_vth_cv);
        locals.var_vgst__blk1131 = assign28990_e22493;
        locals.var_vgst__blk1131_dn3 = (locals.var_vgs_eff__blk1126_dn3 - locals.var_vth_cv_dn3);
        locals.var_vgst__blk1131_dn4 = (locals.var_vgs_eff__blk1126_dn4 - locals.var_vth_cv_dn4);
        locals.var_vgst__blk1131_dn5 = (locals.var_vgs_eff__blk1126_dn5 - locals.var_vth_cv_dn5);
        locals.var_vgst__blk1131_dn6 = (locals.var_vgs_eff__blk1126_dn6 - locals.var_vth_cv_dn6);
        locals.var_vgst__blk1131_dn7 = (locals.var_vgs_eff__blk1126_dn7 - locals.var_vth_cv_dn7);
        locals.var_vgst__blk1131_dn8 = (locals.var_vgs_eff__blk1126_dn8 - locals.var_vth_cv_dn8);
        locals.var_vgst__blk1131_dn9 = (locals.var_vgs_eff__blk1126_dn9 - locals.var_vth_cv_dn9);
        locals.var_vgst__blk1131_dn10 = (locals.var_vgs_eff__blk1126_dn10 - locals.var_vth_cv_dn10);
        locals.var_vgst__blk1131_dn11 = (locals.var_vgs_eff__blk1126_dn11 - locals.var_vth_cv_dn11);
        locals.var_vgst__blk1131_dn12 = (locals.var_vgs_eff__blk1126_dn12 - locals.var_vth_cv_dn12);

        let assign29000_e22496: f64 = (locals.var_n_cv * locals.var_vtm);
        locals.var_t10__blk1154 = assign29000_e22496;
        locals.var_t10__blk1154_dn3 = (locals.var_n_cv_dn3 * locals.var_vtm);
        locals.var_t10__blk1154_dn4 = (locals.var_n_cv_dn4 * locals.var_vtm);
        locals.var_t10__blk1154_dn5 = (locals.var_n_cv_dn5 * locals.var_vtm);
        locals.var_t10__blk1154_dn6 = ((locals.var_n_cv_dn6 * locals.var_vtm) + (locals.var_n_cv * locals.var_vtm_dn6));
        locals.var_t10__blk1154_dn7 = (locals.var_n_cv_dn7 * locals.var_vtm);
        locals.var_t10__blk1154_dn8 = (locals.var_n_cv_dn8 * locals.var_vtm);
        locals.var_t10__blk1154_dn9 = (locals.var_n_cv_dn9 * locals.var_vtm);
        locals.var_t10__blk1154_dn10 = (locals.var_n_cv_dn10 * locals.var_vtm);
        locals.var_t10__blk1154_dn11 = (locals.var_n_cv_dn11 * locals.var_vtm);
        locals.var_t10__blk1154_dn12 = (locals.var_n_cv_dn12 * locals.var_vtm);

        let assign29010_e22499: f64 = (locals.var_pparam_b4soimstar * locals.var_vgst__blk1131);
        let assign29010_e22501: f64 = (assign29010_e22499 / locals.var_t10__blk1154);
        locals.var_vgstnvt__blk1110 = assign29010_e22501;
        locals.var_vgstnvt__blk1110_dn3 = (((((locals.var_pparam_b4soimstar_dn3 * locals.var_vgst__blk1131) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk1131_dn3)) * locals.var_t10__blk1154) - (assign29010_e22499 * locals.var_t10__blk1154_dn3)) / (locals.var_t10__blk1154 * locals.var_t10__blk1154));
        locals.var_vgstnvt__blk1110_dn4 = (((((locals.var_pparam_b4soimstar_dn4 * locals.var_vgst__blk1131) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk1131_dn4)) * locals.var_t10__blk1154) - (assign29010_e22499 * locals.var_t10__blk1154_dn4)) / (locals.var_t10__blk1154 * locals.var_t10__blk1154));
        locals.var_vgstnvt__blk1110_dn5 = (((((locals.var_pparam_b4soimstar_dn5 * locals.var_vgst__blk1131) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk1131_dn5)) * locals.var_t10__blk1154) - (assign29010_e22499 * locals.var_t10__blk1154_dn5)) / (locals.var_t10__blk1154 * locals.var_t10__blk1154));
        locals.var_vgstnvt__blk1110_dn6 = (((((locals.var_pparam_b4soimstar_dn6 * locals.var_vgst__blk1131) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk1131_dn6)) * locals.var_t10__blk1154) - (assign29010_e22499 * locals.var_t10__blk1154_dn6)) / (locals.var_t10__blk1154 * locals.var_t10__blk1154));
        locals.var_vgstnvt__blk1110_dn7 = (((((locals.var_pparam_b4soimstar_dn7 * locals.var_vgst__blk1131) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk1131_dn7)) * locals.var_t10__blk1154) - (assign29010_e22499 * locals.var_t10__blk1154_dn7)) / (locals.var_t10__blk1154 * locals.var_t10__blk1154));
        locals.var_vgstnvt__blk1110_dn8 = (((((locals.var_pparam_b4soimstar_dn8 * locals.var_vgst__blk1131) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk1131_dn8)) * locals.var_t10__blk1154) - (assign29010_e22499 * locals.var_t10__blk1154_dn8)) / (locals.var_t10__blk1154 * locals.var_t10__blk1154));
        locals.var_vgstnvt__blk1110_dn9 = (((((locals.var_pparam_b4soimstar_dn9 * locals.var_vgst__blk1131) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk1131_dn9)) * locals.var_t10__blk1154) - (assign29010_e22499 * locals.var_t10__blk1154_dn9)) / (locals.var_t10__blk1154 * locals.var_t10__blk1154));
        locals.var_vgstnvt__blk1110_dn10 = (((((locals.var_pparam_b4soimstar_dn10 * locals.var_vgst__blk1131) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk1131_dn10)) * locals.var_t10__blk1154) - (assign29010_e22499 * locals.var_t10__blk1154_dn10)) / (locals.var_t10__blk1154 * locals.var_t10__blk1154));
        locals.var_vgstnvt__blk1110_dn11 = (((((locals.var_pparam_b4soimstar_dn11 * locals.var_vgst__blk1131) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk1131_dn11)) * locals.var_t10__blk1154) - (assign29010_e22499 * locals.var_t10__blk1154_dn11)) / (locals.var_t10__blk1154 * locals.var_t10__blk1154));
        locals.var_vgstnvt__blk1110_dn12 = (((((locals.var_pparam_b4soimstar_dn12 * locals.var_vgst__blk1131) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk1131_dn12)) * locals.var_t10__blk1154) - (assign29010_e22499 * locals.var_t10__blk1154_dn12)) / (locals.var_t10__blk1154 * locals.var_t10__blk1154));

        let assign29020_e22504: f64 = (locals.var_n_cv * locals.var_pparam_b4soinoff);
        let assign29020_e22506: f64 = (assign29020_e22504 * locals.var_vtm);
        locals.var_noff = assign29020_e22506;
        locals.var_noff_dn3 = (((locals.var_n_cv_dn3 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn3)) * locals.var_vtm);
        locals.var_noff_dn4 = (((locals.var_n_cv_dn4 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn4)) * locals.var_vtm);
        locals.var_noff_dn5 = (((locals.var_n_cv_dn5 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn5)) * locals.var_vtm);
        locals.var_noff_dn6 = ((((locals.var_n_cv_dn6 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn6)) * locals.var_vtm) + (assign29020_e22504 * locals.var_vtm_dn6));
        locals.var_noff_dn7 = (((locals.var_n_cv_dn7 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn7)) * locals.var_vtm);
        locals.var_noff_dn8 = (((locals.var_n_cv_dn8 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn8)) * locals.var_vtm);
        locals.var_noff_dn9 = (((locals.var_n_cv_dn9 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn9)) * locals.var_vtm);
        locals.var_noff_dn10 = (((locals.var_n_cv_dn10 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn10)) * locals.var_vtm);
        locals.var_noff_dn11 = (((locals.var_n_cv_dn11 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn11)) * locals.var_vtm);
        locals.var_noff_dn12 = (((locals.var_n_cv_dn12 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn12)) * locals.var_vtm);

        let assign29030_e22509: f64 = (locals.var_n_cv * locals.var_pparam_b4soinoff2);
        let assign29030_e22511: f64 = (assign29030_e22509 * locals.var_vtm);
        locals.var_noff2 = assign29030_e22511;
        locals.var_noff2_dn3 = (((locals.var_n_cv_dn3 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn3)) * locals.var_vtm);
        locals.var_noff2_dn4 = (((locals.var_n_cv_dn4 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn4)) * locals.var_vtm);
        locals.var_noff2_dn5 = (((locals.var_n_cv_dn5 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn5)) * locals.var_vtm);
        locals.var_noff2_dn6 = ((((locals.var_n_cv_dn6 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn6)) * locals.var_vtm) + (assign29030_e22509 * locals.var_vtm_dn6));
        locals.var_noff2_dn7 = (((locals.var_n_cv_dn7 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn7)) * locals.var_vtm);
        locals.var_noff2_dn8 = (((locals.var_n_cv_dn8 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn8)) * locals.var_vtm);
        locals.var_noff2_dn9 = (((locals.var_n_cv_dn9 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn9)) * locals.var_vtm);
        locals.var_noff2_dn10 = (((locals.var_n_cv_dn10 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn10)) * locals.var_vtm);
        locals.var_noff2_dn11 = (((locals.var_n_cv_dn11 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn11)) * locals.var_vtm);
        locals.var_noff2_dn12 = (((locals.var_n_cv_dn12 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn12)) * locals.var_vtm);

        let assign29040_e22514: f64 = if locals.var_b4soivgstcvmod == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1687 = assign29040_e22514;

        let assign29050_e22517: f64 = (-100.0);
        let assign29050_e22522: f64 = if ((locals.var_vgstnvt__blk1110 > assign29050_e22517) && (locals.var_vgstnvt__blk1110 < 100.0)) { 1.0 } else { 0.0 };
        locals.var_guard1688 = assign29050_e22522;

        let (assign29060_e22532, assign29060_e22532_d_n3, assign29060_e22532_d_n4, assign29060_e22532_d_n5, assign29060_e22532_d_n6, assign29060_e22532_d_n7, assign29060_e22532_d_n8, assign29060_e22532_d_n9, assign29060_e22532_d_n10, assign29060_e22532_d_n11, assign29060_e22532_d_n12,) = {
    if ((locals.var_guard1687 != 0.0) && (locals.var_guard1688 != 0.0)) {
        let assign29060_e22527: f64 = (locals.var_vgstnvt__blk1110).exp();
        let assign29060_e22529: f64 = (locals.var_vgstnvt__blk1110).exp();
        let assign29060_e22530: f64 = (assign29060_e22527 * assign29060_e22529);
        (assign29060_e22530, (((assign29060_e22527 * locals.var_vgstnvt__blk1110_dn3) * assign29060_e22529) + (assign29060_e22527 * (assign29060_e22529 * locals.var_vgstnvt__blk1110_dn3))), (((assign29060_e22527 * locals.var_vgstnvt__blk1110_dn4) * assign29060_e22529) + (assign29060_e22527 * (assign29060_e22529 * locals.var_vgstnvt__blk1110_dn4))), (((assign29060_e22527 * locals.var_vgstnvt__blk1110_dn5) * assign29060_e22529) + (assign29060_e22527 * (assign29060_e22529 * locals.var_vgstnvt__blk1110_dn5))), (((assign29060_e22527 * locals.var_vgstnvt__blk1110_dn6) * assign29060_e22529) + (assign29060_e22527 * (assign29060_e22529 * locals.var_vgstnvt__blk1110_dn6))), (((assign29060_e22527 * locals.var_vgstnvt__blk1110_dn7) * assign29060_e22529) + (assign29060_e22527 * (assign29060_e22529 * locals.var_vgstnvt__blk1110_dn7))), (((assign29060_e22527 * locals.var_vgstnvt__blk1110_dn8) * assign29060_e22529) + (assign29060_e22527 * (assign29060_e22529 * locals.var_vgstnvt__blk1110_dn8))), (((assign29060_e22527 * locals.var_vgstnvt__blk1110_dn9) * assign29060_e22529) + (assign29060_e22527 * (assign29060_e22529 * locals.var_vgstnvt__blk1110_dn9))), (((assign29060_e22527 * locals.var_vgstnvt__blk1110_dn10) * assign29060_e22529) + (assign29060_e22527 * (assign29060_e22529 * locals.var_vgstnvt__blk1110_dn10))), (((assign29060_e22527 * locals.var_vgstnvt__blk1110_dn11) * assign29060_e22529) + (assign29060_e22527 * (assign29060_e22529 * locals.var_vgstnvt__blk1110_dn11))), (((assign29060_e22527 * locals.var_vgstnvt__blk1110_dn12) * assign29060_e22529) + (assign29060_e22527 * (assign29060_e22529 * locals.var_vgstnvt__blk1110_dn12))),)
    } else {
        (locals.var_expvgst__blk1111, locals.var_expvgst__blk1111_dn3, locals.var_expvgst__blk1111_dn4, locals.var_expvgst__blk1111_dn5, locals.var_expvgst__blk1111_dn6, locals.var_expvgst__blk1111_dn7, locals.var_expvgst__blk1111_dn8, locals.var_expvgst__blk1111_dn9, locals.var_expvgst__blk1111_dn10, locals.var_expvgst__blk1111_dn11, locals.var_expvgst__blk1111_dn12,)
    }
};
        locals.var_expvgst__blk1111 = assign29060_e22532;
        locals.var_expvgst__blk1111_dn3 = assign29060_e22532_d_n3;
        locals.var_expvgst__blk1111_dn4 = assign29060_e22532_d_n4;
        locals.var_expvgst__blk1111_dn5 = assign29060_e22532_d_n5;
        locals.var_expvgst__blk1111_dn6 = assign29060_e22532_d_n6;
        locals.var_expvgst__blk1111_dn7 = assign29060_e22532_d_n7;
        locals.var_expvgst__blk1111_dn8 = assign29060_e22532_d_n8;
        locals.var_expvgst__blk1111_dn9 = assign29060_e22532_d_n9;
        locals.var_expvgst__blk1111_dn10 = assign29060_e22532_d_n10;
        locals.var_expvgst__blk1111_dn11 = assign29060_e22532_d_n11;
        locals.var_expvgst__blk1111_dn12 = assign29060_e22532_d_n12;

        let (assign29070_e22544, assign29070_e22544_d_n3, assign29070_e22544_d_n4, assign29070_e22544_d_n5, assign29070_e22544_d_n6, assign29070_e22544_d_n7, assign29070_e22544_d_n8, assign29070_e22544_d_n9, assign29070_e22544_d_n10, assign29070_e22544_d_n11, assign29070_e22544_d_n12,) = {
    if ((locals.var_guard1687 != 0.0) && (locals.var_guard1688 != 0.0)) {
        let assign29070_e22539: f64 = (locals.var_pparam_b4soidelvt / locals.var_noff);
        let assign29070_e22540: f64 = (-assign29070_e22539);
        let assign29070_e22541: f64 = (assign29070_e22540).exp();
        let assign29070_e22542: f64 = (locals.var_expvgst__blk1111 * assign29070_e22541);
        (assign29070_e22542, ((locals.var_expvgst__blk1111_dn3 * assign29070_e22541) + (locals.var_expvgst__blk1111 * (assign29070_e22541 * (-(((locals.var_pparam_b4soidelvt_dn3 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn3)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn4 * assign29070_e22541) + (locals.var_expvgst__blk1111 * (assign29070_e22541 * (-(((locals.var_pparam_b4soidelvt_dn4 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn4)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn5 * assign29070_e22541) + (locals.var_expvgst__blk1111 * (assign29070_e22541 * (-(((locals.var_pparam_b4soidelvt_dn5 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn5)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn6 * assign29070_e22541) + (locals.var_expvgst__blk1111 * (assign29070_e22541 * (-(((locals.var_pparam_b4soidelvt_dn6 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn6)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn7 * assign29070_e22541) + (locals.var_expvgst__blk1111 * (assign29070_e22541 * (-(((locals.var_pparam_b4soidelvt_dn7 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn7)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn8 * assign29070_e22541) + (locals.var_expvgst__blk1111 * (assign29070_e22541 * (-(((locals.var_pparam_b4soidelvt_dn8 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn8)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn9 * assign29070_e22541) + (locals.var_expvgst__blk1111 * (assign29070_e22541 * (-(((locals.var_pparam_b4soidelvt_dn9 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn9)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn10 * assign29070_e22541) + (locals.var_expvgst__blk1111 * (assign29070_e22541 * (-(((locals.var_pparam_b4soidelvt_dn10 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn10)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn11 * assign29070_e22541) + (locals.var_expvgst__blk1111 * (assign29070_e22541 * (-(((locals.var_pparam_b4soidelvt_dn11 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn11)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn12 * assign29070_e22541) + (locals.var_expvgst__blk1111 * (assign29070_e22541 * (-(((locals.var_pparam_b4soidelvt_dn12 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn12)) / (locals.var_noff * locals.var_noff)))))),)
    } else {
        (locals.var_expvgst__blk1111, locals.var_expvgst__blk1111_dn3, locals.var_expvgst__blk1111_dn4, locals.var_expvgst__blk1111_dn5, locals.var_expvgst__blk1111_dn6, locals.var_expvgst__blk1111_dn7, locals.var_expvgst__blk1111_dn8, locals.var_expvgst__blk1111_dn9, locals.var_expvgst__blk1111_dn10, locals.var_expvgst__blk1111_dn11, locals.var_expvgst__blk1111_dn12,)
    }
};
        locals.var_expvgst__blk1111 = assign29070_e22544;
        locals.var_expvgst__blk1111_dn3 = assign29070_e22544_d_n3;
        locals.var_expvgst__blk1111_dn4 = assign29070_e22544_d_n4;
        locals.var_expvgst__blk1111_dn5 = assign29070_e22544_d_n5;
        locals.var_expvgst__blk1111_dn6 = assign29070_e22544_d_n6;
        locals.var_expvgst__blk1111_dn7 = assign29070_e22544_d_n7;
        locals.var_expvgst__blk1111_dn8 = assign29070_e22544_d_n8;
        locals.var_expvgst__blk1111_dn9 = assign29070_e22544_d_n9;
        locals.var_expvgst__blk1111_dn10 = assign29070_e22544_d_n10;
        locals.var_expvgst__blk1111_dn11 = assign29070_e22544_d_n11;
        locals.var_expvgst__blk1111_dn12 = assign29070_e22544_d_n12;

        let (assign29080_e22563, assign29080_e22563_d_n3, assign29080_e22563_d_n4, assign29080_e22563_d_n5, assign29080_e22563_d_n6, assign29080_e22563_d_n7, assign29080_e22563_d_n8, assign29080_e22563_d_n9, assign29080_e22563_d_n10, assign29080_e22563_d_n11, assign29080_e22563_d_n12,) = {
    if ((locals.var_guard1687 != 0.0) && (locals.var_guard1688 != 0.0)) {
        let assign29080_e22551: f64 = (1.0 + locals.var_expvgst__blk1111);
        let (assign29080_e22560, assign29080_e22560_d_n3, assign29080_e22560_d_n4, assign29080_e22560_d_n5, assign29080_e22560_d_n6, assign29080_e22560_d_n7, assign29080_e22560_d_n8, assign29080_e22560_d_n9, assign29080_e22560_d_n10, assign29080_e22560_d_n11, assign29080_e22560_d_n12,) = {
            if (assign29080_e22551 > 1e-38) {
                let assign29080_e22556: f64 = (1.0 + locals.var_expvgst__blk1111);
                let assign29080_e22557: f64 = (assign29080_e22556).ln();
                (assign29080_e22557, (locals.var_expvgst__blk1111_dn3 / assign29080_e22556), (locals.var_expvgst__blk1111_dn4 / assign29080_e22556), (locals.var_expvgst__blk1111_dn5 / assign29080_e22556), (locals.var_expvgst__blk1111_dn6 / assign29080_e22556), (locals.var_expvgst__blk1111_dn7 / assign29080_e22556), (locals.var_expvgst__blk1111_dn8 / assign29080_e22556), (locals.var_expvgst__blk1111_dn9 / assign29080_e22556), (locals.var_expvgst__blk1111_dn10 / assign29080_e22556), (locals.var_expvgst__blk1111_dn11 / assign29080_e22556), (locals.var_expvgst__blk1111_dn12 / assign29080_e22556),)
            } else {
                let assign29080_e22559: f64 = (-87.49823353377374);
                (assign29080_e22559, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign29080_e22561: f64 = (locals.var_noff * assign29080_e22560);
        (assign29080_e22561, ((locals.var_noff_dn3 * assign29080_e22560) + (locals.var_noff * assign29080_e22560_d_n3)), ((locals.var_noff_dn4 * assign29080_e22560) + (locals.var_noff * assign29080_e22560_d_n4)), ((locals.var_noff_dn5 * assign29080_e22560) + (locals.var_noff * assign29080_e22560_d_n5)), ((locals.var_noff_dn6 * assign29080_e22560) + (locals.var_noff * assign29080_e22560_d_n6)), ((locals.var_noff_dn7 * assign29080_e22560) + (locals.var_noff * assign29080_e22560_d_n7)), ((locals.var_noff_dn8 * assign29080_e22560) + (locals.var_noff * assign29080_e22560_d_n8)), ((locals.var_noff_dn9 * assign29080_e22560) + (locals.var_noff * assign29080_e22560_d_n9)), ((locals.var_noff_dn10 * assign29080_e22560) + (locals.var_noff * assign29080_e22560_d_n10)), ((locals.var_noff_dn11 * assign29080_e22560) + (locals.var_noff * assign29080_e22560_d_n11)), ((locals.var_noff_dn12 * assign29080_e22560) + (locals.var_noff * assign29080_e22560_d_n12)),)
    } else {
        (locals.var_vgsteff__blk1175, locals.var_vgsteff__blk1175_dn3, locals.var_vgsteff__blk1175_dn4, locals.var_vgsteff__blk1175_dn5, locals.var_vgsteff__blk1175_dn6, locals.var_vgsteff__blk1175_dn7, locals.var_vgsteff__blk1175_dn8, locals.var_vgsteff__blk1175_dn9, locals.var_vgsteff__blk1175_dn10, locals.var_vgsteff__blk1175_dn11, locals.var_vgsteff__blk1175_dn12,)
    }
};
        locals.var_vgsteff__blk1175 = assign29080_e22563;
        locals.var_vgsteff__blk1175_dn3 = assign29080_e22563_d_n3;
        locals.var_vgsteff__blk1175_dn4 = assign29080_e22563_d_n4;
        locals.var_vgsteff__blk1175_dn5 = assign29080_e22563_d_n5;
        locals.var_vgsteff__blk1175_dn6 = assign29080_e22563_d_n6;
        locals.var_vgsteff__blk1175_dn7 = assign29080_e22563_d_n7;
        locals.var_vgsteff__blk1175_dn8 = assign29080_e22563_d_n8;
        locals.var_vgsteff__blk1175_dn9 = assign29080_e22563_d_n9;
        locals.var_vgsteff__blk1175_dn10 = assign29080_e22563_d_n10;
        locals.var_vgsteff__blk1175_dn11 = assign29080_e22563_d_n11;
        locals.var_vgsteff__blk1175_dn12 = assign29080_e22563_d_n12;

        let assign29090_e22566: f64 = if locals.var_b4soiagbcp2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1689 = assign29090_e22566;

        let (assign29100_e22584, assign29100_e22584_d_n3, assign29100_e22584_d_n4, assign29100_e22584_d_n5, assign29100_e22584_d_n6, assign29100_e22584_d_n7, assign29100_e22584_d_n8, assign29100_e22584_d_n9, assign29100_e22584_d_n10, assign29100_e22584_d_n11, assign29100_e22584_d_n12,) = {
    if (((locals.var_guard1687 != 0.0) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) {
        let assign29100_e22574: f64 = (-locals.var_eggbcp2);
        let assign29100_e22576: f64 = (assign29100_e22574 / locals.var_noff2);
        let assign29100_e22579: f64 = (locals.var_vtm * locals.var_vtm);
        let assign29100_e22580: f64 = (assign29100_e22576 / assign29100_e22579);
        let assign29100_e22581: f64 = (assign29100_e22580).exp();
        let assign29100_e22582: f64 = (locals.var_expvgst__blk1111 * assign29100_e22581);
        (assign29100_e22582, ((locals.var_expvgst__blk1111_dn3 * assign29100_e22581) + (locals.var_expvgst__blk1111 * (assign29100_e22581 * ((-((assign29100_e22574 * locals.var_noff2_dn3) / (locals.var_noff2 * locals.var_noff2))) / assign29100_e22579)))), ((locals.var_expvgst__blk1111_dn4 * assign29100_e22581) + (locals.var_expvgst__blk1111 * (assign29100_e22581 * ((-((assign29100_e22574 * locals.var_noff2_dn4) / (locals.var_noff2 * locals.var_noff2))) / assign29100_e22579)))), ((locals.var_expvgst__blk1111_dn5 * assign29100_e22581) + (locals.var_expvgst__blk1111 * (assign29100_e22581 * ((-((assign29100_e22574 * locals.var_noff2_dn5) / (locals.var_noff2 * locals.var_noff2))) / assign29100_e22579)))), ((locals.var_expvgst__blk1111_dn6 * assign29100_e22581) + (locals.var_expvgst__blk1111 * (assign29100_e22581 * ((((-((assign29100_e22574 * locals.var_noff2_dn6) / (locals.var_noff2 * locals.var_noff2))) * assign29100_e22579) - (assign29100_e22576 * ((locals.var_vtm_dn6 * locals.var_vtm) + (locals.var_vtm * locals.var_vtm_dn6)))) / (assign29100_e22579 * assign29100_e22579))))), ((locals.var_expvgst__blk1111_dn7 * assign29100_e22581) + (locals.var_expvgst__blk1111 * (assign29100_e22581 * ((-((assign29100_e22574 * locals.var_noff2_dn7) / (locals.var_noff2 * locals.var_noff2))) / assign29100_e22579)))), ((locals.var_expvgst__blk1111_dn8 * assign29100_e22581) + (locals.var_expvgst__blk1111 * (assign29100_e22581 * ((-((assign29100_e22574 * locals.var_noff2_dn8) / (locals.var_noff2 * locals.var_noff2))) / assign29100_e22579)))), ((locals.var_expvgst__blk1111_dn9 * assign29100_e22581) + (locals.var_expvgst__blk1111 * (assign29100_e22581 * ((-((assign29100_e22574 * locals.var_noff2_dn9) / (locals.var_noff2 * locals.var_noff2))) / assign29100_e22579)))), ((locals.var_expvgst__blk1111_dn10 * assign29100_e22581) + (locals.var_expvgst__blk1111 * (assign29100_e22581 * ((-((assign29100_e22574 * locals.var_noff2_dn10) / (locals.var_noff2 * locals.var_noff2))) / assign29100_e22579)))), ((locals.var_expvgst__blk1111_dn11 * assign29100_e22581) + (locals.var_expvgst__blk1111 * (assign29100_e22581 * ((-((assign29100_e22574 * locals.var_noff2_dn11) / (locals.var_noff2 * locals.var_noff2))) / assign29100_e22579)))), ((locals.var_expvgst__blk1111_dn12 * assign29100_e22581) + (locals.var_expvgst__blk1111 * (assign29100_e22581 * ((-((assign29100_e22574 * locals.var_noff2_dn12) / (locals.var_noff2 * locals.var_noff2))) / assign29100_e22579)))),)
    } else {
        (locals.var_expvgst2, locals.var_expvgst2_dn3, locals.var_expvgst2_dn4, locals.var_expvgst2_dn5, locals.var_expvgst2_dn6, locals.var_expvgst2_dn7, locals.var_expvgst2_dn8, locals.var_expvgst2_dn9, locals.var_expvgst2_dn10, locals.var_expvgst2_dn11, locals.var_expvgst2_dn12,)
    }
};
        locals.var_expvgst2 = assign29100_e22584;
        locals.var_expvgst2_dn3 = assign29100_e22584_d_n3;
        locals.var_expvgst2_dn4 = assign29100_e22584_d_n4;
        locals.var_expvgst2_dn5 = assign29100_e22584_d_n5;
        locals.var_expvgst2_dn6 = assign29100_e22584_d_n6;
        locals.var_expvgst2_dn7 = assign29100_e22584_d_n7;
        locals.var_expvgst2_dn8 = assign29100_e22584_d_n8;
        locals.var_expvgst2_dn9 = assign29100_e22584_d_n9;
        locals.var_expvgst2_dn10 = assign29100_e22584_d_n10;
        locals.var_expvgst2_dn11 = assign29100_e22584_d_n11;
        locals.var_expvgst2_dn12 = assign29100_e22584_d_n12;

        let (assign29110_e22605, assign29110_e22605_d_n3, assign29110_e22605_d_n4, assign29110_e22605_d_n5, assign29110_e22605_d_n6, assign29110_e22605_d_n7, assign29110_e22605_d_n8, assign29110_e22605_d_n9, assign29110_e22605_d_n10, assign29110_e22605_d_n11, assign29110_e22605_d_n12,) = {
    if (((locals.var_guard1687 != 0.0) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) {
        let assign29110_e22593: f64 = (1.0 + locals.var_expvgst2);
        let (assign29110_e22602, assign29110_e22602_d_n3, assign29110_e22602_d_n4, assign29110_e22602_d_n5, assign29110_e22602_d_n6, assign29110_e22602_d_n7, assign29110_e22602_d_n8, assign29110_e22602_d_n9, assign29110_e22602_d_n10, assign29110_e22602_d_n11, assign29110_e22602_d_n12,) = {
            if (assign29110_e22593 > 1e-38) {
                let assign29110_e22598: f64 = (1.0 + locals.var_expvgst2);
                let assign29110_e22599: f64 = (assign29110_e22598).ln();
                (assign29110_e22599, (locals.var_expvgst2_dn3 / assign29110_e22598), (locals.var_expvgst2_dn4 / assign29110_e22598), (locals.var_expvgst2_dn5 / assign29110_e22598), (locals.var_expvgst2_dn6 / assign29110_e22598), (locals.var_expvgst2_dn7 / assign29110_e22598), (locals.var_expvgst2_dn8 / assign29110_e22598), (locals.var_expvgst2_dn9 / assign29110_e22598), (locals.var_expvgst2_dn10 / assign29110_e22598), (locals.var_expvgst2_dn11 / assign29110_e22598), (locals.var_expvgst2_dn12 / assign29110_e22598),)
            } else {
                let assign29110_e22601: f64 = (-87.49823353377374);
                (assign29110_e22601, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign29110_e22603: f64 = (locals.var_noff2 * assign29110_e22602);
        (assign29110_e22603, ((locals.var_noff2_dn3 * assign29110_e22602) + (locals.var_noff2 * assign29110_e22602_d_n3)), ((locals.var_noff2_dn4 * assign29110_e22602) + (locals.var_noff2 * assign29110_e22602_d_n4)), ((locals.var_noff2_dn5 * assign29110_e22602) + (locals.var_noff2 * assign29110_e22602_d_n5)), ((locals.var_noff2_dn6 * assign29110_e22602) + (locals.var_noff2 * assign29110_e22602_d_n6)), ((locals.var_noff2_dn7 * assign29110_e22602) + (locals.var_noff2 * assign29110_e22602_d_n7)), ((locals.var_noff2_dn8 * assign29110_e22602) + (locals.var_noff2 * assign29110_e22602_d_n8)), ((locals.var_noff2_dn9 * assign29110_e22602) + (locals.var_noff2 * assign29110_e22602_d_n9)), ((locals.var_noff2_dn10 * assign29110_e22602) + (locals.var_noff2 * assign29110_e22602_d_n10)), ((locals.var_noff2_dn11 * assign29110_e22602) + (locals.var_noff2 * assign29110_e22602_d_n11)), ((locals.var_noff2_dn12 * assign29110_e22602) + (locals.var_noff2 * assign29110_e22602_d_n12)),)
    } else {
        (locals.var_vgsteff2, locals.var_vgsteff2_dn3, locals.var_vgsteff2_dn4, locals.var_vgsteff2_dn5, locals.var_vgsteff2_dn6, locals.var_vgsteff2_dn7, locals.var_vgsteff2_dn8, locals.var_vgsteff2_dn9, locals.var_vgsteff2_dn10, locals.var_vgsteff2_dn11, locals.var_vgsteff2_dn12,)
    }
};
        locals.var_vgsteff2 = assign29110_e22605;
        locals.var_vgsteff2_dn3 = assign29110_e22605_d_n3;
        locals.var_vgsteff2_dn4 = assign29110_e22605_d_n4;
        locals.var_vgsteff2_dn5 = assign29110_e22605_d_n5;
        locals.var_vgsteff2_dn6 = assign29110_e22605_d_n6;
        locals.var_vgsteff2_dn7 = assign29110_e22605_d_n7;
        locals.var_vgsteff2_dn8 = assign29110_e22605_d_n8;
        locals.var_vgsteff2_dn9 = assign29110_e22605_d_n9;
        locals.var_vgsteff2_dn10 = assign29110_e22605_d_n10;
        locals.var_vgsteff2_dn11 = assign29110_e22605_d_n11;
        locals.var_vgsteff2_dn12 = assign29110_e22605_d_n12;

        let assign29120_e22608: f64 = if locals.var_b4soivgstcvmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1690 = assign29120_e22608;

        let assign29130_e22611: f64 = (-100.0);
        let assign29130_e22616: f64 = if ((locals.var_vgstnvt__blk1110 > assign29130_e22611) && (locals.var_vgstnvt__blk1110 < 100.0)) { 1.0 } else { 0.0 };
        locals.var_guard1691 = assign29130_e22616;

        let (assign29140_e22630, assign29140_e22630_d_n3, assign29140_e22630_d_n4, assign29140_e22630_d_n5, assign29140_e22630_d_n6, assign29140_e22630_d_n7, assign29140_e22630_d_n8, assign29140_e22630_d_n9, assign29140_e22630_d_n10, assign29140_e22630_d_n11, assign29140_e22630_d_n12,) = {
    if (((locals.var_guard1687 == 0.0) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) {
        let assign29140_e22626: f64 = (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff);
        let assign29140_e22627: f64 = (locals.var_vgstnvt__blk1110 / assign29140_e22626);
        let assign29140_e22628: f64 = (assign29140_e22627).exp();
        (assign29140_e22628, (assign29140_e22628 * (((locals.var_vgstnvt__blk1110_dn3 * assign29140_e22626) - (locals.var_vgstnvt__blk1110 * ((locals.var_pparam_b4soimstar_dn3 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn3)))) / (assign29140_e22626 * assign29140_e22626))), (assign29140_e22628 * (((locals.var_vgstnvt__blk1110_dn4 * assign29140_e22626) - (locals.var_vgstnvt__blk1110 * ((locals.var_pparam_b4soimstar_dn4 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn4)))) / (assign29140_e22626 * assign29140_e22626))), (assign29140_e22628 * (((locals.var_vgstnvt__blk1110_dn5 * assign29140_e22626) - (locals.var_vgstnvt__blk1110 * ((locals.var_pparam_b4soimstar_dn5 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn5)))) / (assign29140_e22626 * assign29140_e22626))), (assign29140_e22628 * (((locals.var_vgstnvt__blk1110_dn6 * assign29140_e22626) - (locals.var_vgstnvt__blk1110 * ((locals.var_pparam_b4soimstar_dn6 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn6)))) / (assign29140_e22626 * assign29140_e22626))), (assign29140_e22628 * (((locals.var_vgstnvt__blk1110_dn7 * assign29140_e22626) - (locals.var_vgstnvt__blk1110 * ((locals.var_pparam_b4soimstar_dn7 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn7)))) / (assign29140_e22626 * assign29140_e22626))), (assign29140_e22628 * (((locals.var_vgstnvt__blk1110_dn8 * assign29140_e22626) - (locals.var_vgstnvt__blk1110 * ((locals.var_pparam_b4soimstar_dn8 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn8)))) / (assign29140_e22626 * assign29140_e22626))), (assign29140_e22628 * (((locals.var_vgstnvt__blk1110_dn9 * assign29140_e22626) - (locals.var_vgstnvt__blk1110 * ((locals.var_pparam_b4soimstar_dn9 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn9)))) / (assign29140_e22626 * assign29140_e22626))), (assign29140_e22628 * (((locals.var_vgstnvt__blk1110_dn10 * assign29140_e22626) - (locals.var_vgstnvt__blk1110 * ((locals.var_pparam_b4soimstar_dn10 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn10)))) / (assign29140_e22626 * assign29140_e22626))), (assign29140_e22628 * (((locals.var_vgstnvt__blk1110_dn11 * assign29140_e22626) - (locals.var_vgstnvt__blk1110 * ((locals.var_pparam_b4soimstar_dn11 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn11)))) / (assign29140_e22626 * assign29140_e22626))), (assign29140_e22628 * (((locals.var_vgstnvt__blk1110_dn12 * assign29140_e22626) - (locals.var_vgstnvt__blk1110 * ((locals.var_pparam_b4soimstar_dn12 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn12)))) / (assign29140_e22626 * assign29140_e22626))),)
    } else {
        (locals.var_expvgst__blk1111, locals.var_expvgst__blk1111_dn3, locals.var_expvgst__blk1111_dn4, locals.var_expvgst__blk1111_dn5, locals.var_expvgst__blk1111_dn6, locals.var_expvgst__blk1111_dn7, locals.var_expvgst__blk1111_dn8, locals.var_expvgst__blk1111_dn9, locals.var_expvgst__blk1111_dn10, locals.var_expvgst__blk1111_dn11, locals.var_expvgst__blk1111_dn12,)
    }
};
        locals.var_expvgst__blk1111 = assign29140_e22630;
        locals.var_expvgst__blk1111_dn3 = assign29140_e22630_d_n3;
        locals.var_expvgst__blk1111_dn4 = assign29140_e22630_d_n4;
        locals.var_expvgst__blk1111_dn5 = assign29140_e22630_d_n5;
        locals.var_expvgst__blk1111_dn6 = assign29140_e22630_d_n6;
        locals.var_expvgst__blk1111_dn7 = assign29140_e22630_d_n7;
        locals.var_expvgst__blk1111_dn8 = assign29140_e22630_d_n8;
        locals.var_expvgst__blk1111_dn9 = assign29140_e22630_d_n9;
        locals.var_expvgst__blk1111_dn10 = assign29140_e22630_d_n10;
        locals.var_expvgst__blk1111_dn11 = assign29140_e22630_d_n11;
        locals.var_expvgst__blk1111_dn12 = assign29140_e22630_d_n12;

        let (assign29150_e22645, assign29150_e22645_d_n3, assign29150_e22645_d_n4, assign29150_e22645_d_n5, assign29150_e22645_d_n6, assign29150_e22645_d_n7, assign29150_e22645_d_n8, assign29150_e22645_d_n9, assign29150_e22645_d_n10, assign29150_e22645_d_n11, assign29150_e22645_d_n12,) = {
    if (((locals.var_guard1687 == 0.0) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) {
        let assign29150_e22640: f64 = (locals.var_pparam_b4soidelvt / locals.var_noff);
        let assign29150_e22641: f64 = (-assign29150_e22640);
        let assign29150_e22642: f64 = (assign29150_e22641).exp();
        let assign29150_e22643: f64 = (locals.var_expvgst__blk1111 * assign29150_e22642);
        (assign29150_e22643, ((locals.var_expvgst__blk1111_dn3 * assign29150_e22642) + (locals.var_expvgst__blk1111 * (assign29150_e22642 * (-(((locals.var_pparam_b4soidelvt_dn3 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn3)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn4 * assign29150_e22642) + (locals.var_expvgst__blk1111 * (assign29150_e22642 * (-(((locals.var_pparam_b4soidelvt_dn4 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn4)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn5 * assign29150_e22642) + (locals.var_expvgst__blk1111 * (assign29150_e22642 * (-(((locals.var_pparam_b4soidelvt_dn5 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn5)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn6 * assign29150_e22642) + (locals.var_expvgst__blk1111 * (assign29150_e22642 * (-(((locals.var_pparam_b4soidelvt_dn6 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn6)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn7 * assign29150_e22642) + (locals.var_expvgst__blk1111 * (assign29150_e22642 * (-(((locals.var_pparam_b4soidelvt_dn7 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn7)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn8 * assign29150_e22642) + (locals.var_expvgst__blk1111 * (assign29150_e22642 * (-(((locals.var_pparam_b4soidelvt_dn8 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn8)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn9 * assign29150_e22642) + (locals.var_expvgst__blk1111 * (assign29150_e22642 * (-(((locals.var_pparam_b4soidelvt_dn9 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn9)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn10 * assign29150_e22642) + (locals.var_expvgst__blk1111 * (assign29150_e22642 * (-(((locals.var_pparam_b4soidelvt_dn10 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn10)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn11 * assign29150_e22642) + (locals.var_expvgst__blk1111 * (assign29150_e22642 * (-(((locals.var_pparam_b4soidelvt_dn11 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn11)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk1111_dn12 * assign29150_e22642) + (locals.var_expvgst__blk1111 * (assign29150_e22642 * (-(((locals.var_pparam_b4soidelvt_dn12 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn12)) / (locals.var_noff * locals.var_noff)))))),)
    } else {
        (locals.var_expvgst__blk1111, locals.var_expvgst__blk1111_dn3, locals.var_expvgst__blk1111_dn4, locals.var_expvgst__blk1111_dn5, locals.var_expvgst__blk1111_dn6, locals.var_expvgst__blk1111_dn7, locals.var_expvgst__blk1111_dn8, locals.var_expvgst__blk1111_dn9, locals.var_expvgst__blk1111_dn10, locals.var_expvgst__blk1111_dn11, locals.var_expvgst__blk1111_dn12,)
    }
};
        locals.var_expvgst__blk1111 = assign29150_e22645;
        locals.var_expvgst__blk1111_dn3 = assign29150_e22645_d_n3;
        locals.var_expvgst__blk1111_dn4 = assign29150_e22645_d_n4;
        locals.var_expvgst__blk1111_dn5 = assign29150_e22645_d_n5;
        locals.var_expvgst__blk1111_dn6 = assign29150_e22645_d_n6;
        locals.var_expvgst__blk1111_dn7 = assign29150_e22645_d_n7;
        locals.var_expvgst__blk1111_dn8 = assign29150_e22645_d_n8;
        locals.var_expvgst__blk1111_dn9 = assign29150_e22645_d_n9;
        locals.var_expvgst__blk1111_dn10 = assign29150_e22645_d_n10;
        locals.var_expvgst__blk1111_dn11 = assign29150_e22645_d_n11;
        locals.var_expvgst__blk1111_dn12 = assign29150_e22645_d_n12;

        let (assign29160_e22667, assign29160_e22667_d_n3, assign29160_e22667_d_n4, assign29160_e22667_d_n5, assign29160_e22667_d_n6, assign29160_e22667_d_n7, assign29160_e22667_d_n8, assign29160_e22667_d_n9, assign29160_e22667_d_n10, assign29160_e22667_d_n11, assign29160_e22667_d_n12,) = {
    if (((locals.var_guard1687 == 0.0) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) {
        let assign29160_e22655: f64 = (1.0 + locals.var_expvgst__blk1111);
        let (assign29160_e22664, assign29160_e22664_d_n3, assign29160_e22664_d_n4, assign29160_e22664_d_n5, assign29160_e22664_d_n6, assign29160_e22664_d_n7, assign29160_e22664_d_n8, assign29160_e22664_d_n9, assign29160_e22664_d_n10, assign29160_e22664_d_n11, assign29160_e22664_d_n12,) = {
            if (assign29160_e22655 > 1e-38) {
                let assign29160_e22660: f64 = (1.0 + locals.var_expvgst__blk1111);
                let assign29160_e22661: f64 = (assign29160_e22660).ln();
                (assign29160_e22661, (locals.var_expvgst__blk1111_dn3 / assign29160_e22660), (locals.var_expvgst__blk1111_dn4 / assign29160_e22660), (locals.var_expvgst__blk1111_dn5 / assign29160_e22660), (locals.var_expvgst__blk1111_dn6 / assign29160_e22660), (locals.var_expvgst__blk1111_dn7 / assign29160_e22660), (locals.var_expvgst__blk1111_dn8 / assign29160_e22660), (locals.var_expvgst__blk1111_dn9 / assign29160_e22660), (locals.var_expvgst__blk1111_dn10 / assign29160_e22660), (locals.var_expvgst__blk1111_dn11 / assign29160_e22660), (locals.var_expvgst__blk1111_dn12 / assign29160_e22660),)
            } else {
                let assign29160_e22663: f64 = (-87.49823353377374);
                (assign29160_e22663, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign29160_e22665: f64 = (locals.var_noff * assign29160_e22664);
        (assign29160_e22665, ((locals.var_noff_dn3 * assign29160_e22664) + (locals.var_noff * assign29160_e22664_d_n3)), ((locals.var_noff_dn4 * assign29160_e22664) + (locals.var_noff * assign29160_e22664_d_n4)), ((locals.var_noff_dn5 * assign29160_e22664) + (locals.var_noff * assign29160_e22664_d_n5)), ((locals.var_noff_dn6 * assign29160_e22664) + (locals.var_noff * assign29160_e22664_d_n6)), ((locals.var_noff_dn7 * assign29160_e22664) + (locals.var_noff * assign29160_e22664_d_n7)), ((locals.var_noff_dn8 * assign29160_e22664) + (locals.var_noff * assign29160_e22664_d_n8)), ((locals.var_noff_dn9 * assign29160_e22664) + (locals.var_noff * assign29160_e22664_d_n9)), ((locals.var_noff_dn10 * assign29160_e22664) + (locals.var_noff * assign29160_e22664_d_n10)), ((locals.var_noff_dn11 * assign29160_e22664) + (locals.var_noff * assign29160_e22664_d_n11)), ((locals.var_noff_dn12 * assign29160_e22664) + (locals.var_noff * assign29160_e22664_d_n12)),)
    } else {
        (locals.var_vgsteff__blk1175, locals.var_vgsteff__blk1175_dn3, locals.var_vgsteff__blk1175_dn4, locals.var_vgsteff__blk1175_dn5, locals.var_vgsteff__blk1175_dn6, locals.var_vgsteff__blk1175_dn7, locals.var_vgsteff__blk1175_dn8, locals.var_vgsteff__blk1175_dn9, locals.var_vgsteff__blk1175_dn10, locals.var_vgsteff__blk1175_dn11, locals.var_vgsteff__blk1175_dn12,)
    }
};
        locals.var_vgsteff__blk1175 = assign29160_e22667;
        locals.var_vgsteff__blk1175_dn3 = assign29160_e22667_d_n3;
        locals.var_vgsteff__blk1175_dn4 = assign29160_e22667_d_n4;
        locals.var_vgsteff__blk1175_dn5 = assign29160_e22667_d_n5;
        locals.var_vgsteff__blk1175_dn6 = assign29160_e22667_d_n6;
        locals.var_vgsteff__blk1175_dn7 = assign29160_e22667_d_n7;
        locals.var_vgsteff__blk1175_dn8 = assign29160_e22667_d_n8;
        locals.var_vgsteff__blk1175_dn9 = assign29160_e22667_d_n9;
        locals.var_vgsteff__blk1175_dn10 = assign29160_e22667_d_n10;
        locals.var_vgsteff__blk1175_dn11 = assign29160_e22667_d_n11;
        locals.var_vgsteff__blk1175_dn12 = assign29160_e22667_d_n12;

        let assign29170_e22670: f64 = if locals.var_b4soiagbcp2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1692 = assign29170_e22670;

        let (assign29180_e22691, assign29180_e22691_d_n3, assign29180_e22691_d_n4, assign29180_e22691_d_n5, assign29180_e22691_d_n6, assign29180_e22691_d_n7, assign29180_e22691_d_n8, assign29180_e22691_d_n9, assign29180_e22691_d_n10, assign29180_e22691_d_n11, assign29180_e22691_d_n12,) = {
    if ((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) && (locals.var_guard1692 != 0.0)) {
        let assign29180_e22681: f64 = (-locals.var_eggbcp2);
        let assign29180_e22683: f64 = (assign29180_e22681 / locals.var_noff2);
        let assign29180_e22686: f64 = (locals.var_vtm * locals.var_vtm);
        let assign29180_e22687: f64 = (assign29180_e22683 / assign29180_e22686);
        let assign29180_e22688: f64 = (assign29180_e22687).exp();
        let assign29180_e22689: f64 = (locals.var_expvgst__blk1111 * assign29180_e22688);
        (assign29180_e22689, ((locals.var_expvgst__blk1111_dn3 * assign29180_e22688) + (locals.var_expvgst__blk1111 * (assign29180_e22688 * ((-((assign29180_e22681 * locals.var_noff2_dn3) / (locals.var_noff2 * locals.var_noff2))) / assign29180_e22686)))), ((locals.var_expvgst__blk1111_dn4 * assign29180_e22688) + (locals.var_expvgst__blk1111 * (assign29180_e22688 * ((-((assign29180_e22681 * locals.var_noff2_dn4) / (locals.var_noff2 * locals.var_noff2))) / assign29180_e22686)))), ((locals.var_expvgst__blk1111_dn5 * assign29180_e22688) + (locals.var_expvgst__blk1111 * (assign29180_e22688 * ((-((assign29180_e22681 * locals.var_noff2_dn5) / (locals.var_noff2 * locals.var_noff2))) / assign29180_e22686)))), ((locals.var_expvgst__blk1111_dn6 * assign29180_e22688) + (locals.var_expvgst__blk1111 * (assign29180_e22688 * ((((-((assign29180_e22681 * locals.var_noff2_dn6) / (locals.var_noff2 * locals.var_noff2))) * assign29180_e22686) - (assign29180_e22683 * ((locals.var_vtm_dn6 * locals.var_vtm) + (locals.var_vtm * locals.var_vtm_dn6)))) / (assign29180_e22686 * assign29180_e22686))))), ((locals.var_expvgst__blk1111_dn7 * assign29180_e22688) + (locals.var_expvgst__blk1111 * (assign29180_e22688 * ((-((assign29180_e22681 * locals.var_noff2_dn7) / (locals.var_noff2 * locals.var_noff2))) / assign29180_e22686)))), ((locals.var_expvgst__blk1111_dn8 * assign29180_e22688) + (locals.var_expvgst__blk1111 * (assign29180_e22688 * ((-((assign29180_e22681 * locals.var_noff2_dn8) / (locals.var_noff2 * locals.var_noff2))) / assign29180_e22686)))), ((locals.var_expvgst__blk1111_dn9 * assign29180_e22688) + (locals.var_expvgst__blk1111 * (assign29180_e22688 * ((-((assign29180_e22681 * locals.var_noff2_dn9) / (locals.var_noff2 * locals.var_noff2))) / assign29180_e22686)))), ((locals.var_expvgst__blk1111_dn10 * assign29180_e22688) + (locals.var_expvgst__blk1111 * (assign29180_e22688 * ((-((assign29180_e22681 * locals.var_noff2_dn10) / (locals.var_noff2 * locals.var_noff2))) / assign29180_e22686)))), ((locals.var_expvgst__blk1111_dn11 * assign29180_e22688) + (locals.var_expvgst__blk1111 * (assign29180_e22688 * ((-((assign29180_e22681 * locals.var_noff2_dn11) / (locals.var_noff2 * locals.var_noff2))) / assign29180_e22686)))), ((locals.var_expvgst__blk1111_dn12 * assign29180_e22688) + (locals.var_expvgst__blk1111 * (assign29180_e22688 * ((-((assign29180_e22681 * locals.var_noff2_dn12) / (locals.var_noff2 * locals.var_noff2))) / assign29180_e22686)))),)
    } else {
        (locals.var_expvgst2, locals.var_expvgst2_dn3, locals.var_expvgst2_dn4, locals.var_expvgst2_dn5, locals.var_expvgst2_dn6, locals.var_expvgst2_dn7, locals.var_expvgst2_dn8, locals.var_expvgst2_dn9, locals.var_expvgst2_dn10, locals.var_expvgst2_dn11, locals.var_expvgst2_dn12,)
    }
};
        locals.var_expvgst2 = assign29180_e22691;
        locals.var_expvgst2_dn3 = assign29180_e22691_d_n3;
        locals.var_expvgst2_dn4 = assign29180_e22691_d_n4;
        locals.var_expvgst2_dn5 = assign29180_e22691_d_n5;
        locals.var_expvgst2_dn6 = assign29180_e22691_d_n6;
        locals.var_expvgst2_dn7 = assign29180_e22691_d_n7;
        locals.var_expvgst2_dn8 = assign29180_e22691_d_n8;
        locals.var_expvgst2_dn9 = assign29180_e22691_d_n9;
        locals.var_expvgst2_dn10 = assign29180_e22691_d_n10;
        locals.var_expvgst2_dn11 = assign29180_e22691_d_n11;
        locals.var_expvgst2_dn12 = assign29180_e22691_d_n12;

        let (assign29190_e22715, assign29190_e22715_d_n3, assign29190_e22715_d_n4, assign29190_e22715_d_n5, assign29190_e22715_d_n6, assign29190_e22715_d_n7, assign29190_e22715_d_n8, assign29190_e22715_d_n9, assign29190_e22715_d_n10, assign29190_e22715_d_n11, assign29190_e22715_d_n12,) = {
    if ((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) && (locals.var_guard1692 != 0.0)) {
        let assign29190_e22703: f64 = (1.0 + locals.var_expvgst2);
        let (assign29190_e22712, assign29190_e22712_d_n3, assign29190_e22712_d_n4, assign29190_e22712_d_n5, assign29190_e22712_d_n6, assign29190_e22712_d_n7, assign29190_e22712_d_n8, assign29190_e22712_d_n9, assign29190_e22712_d_n10, assign29190_e22712_d_n11, assign29190_e22712_d_n12,) = {
            if (assign29190_e22703 > 1e-38) {
                let assign29190_e22708: f64 = (1.0 + locals.var_expvgst2);
                let assign29190_e22709: f64 = (assign29190_e22708).ln();
                (assign29190_e22709, (locals.var_expvgst2_dn3 / assign29190_e22708), (locals.var_expvgst2_dn4 / assign29190_e22708), (locals.var_expvgst2_dn5 / assign29190_e22708), (locals.var_expvgst2_dn6 / assign29190_e22708), (locals.var_expvgst2_dn7 / assign29190_e22708), (locals.var_expvgst2_dn8 / assign29190_e22708), (locals.var_expvgst2_dn9 / assign29190_e22708), (locals.var_expvgst2_dn10 / assign29190_e22708), (locals.var_expvgst2_dn11 / assign29190_e22708), (locals.var_expvgst2_dn12 / assign29190_e22708),)
            } else {
                let assign29190_e22711: f64 = (-87.49823353377374);
                (assign29190_e22711, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign29190_e22713: f64 = (locals.var_noff2 * assign29190_e22712);
        (assign29190_e22713, ((locals.var_noff2_dn3 * assign29190_e22712) + (locals.var_noff2 * assign29190_e22712_d_n3)), ((locals.var_noff2_dn4 * assign29190_e22712) + (locals.var_noff2 * assign29190_e22712_d_n4)), ((locals.var_noff2_dn5 * assign29190_e22712) + (locals.var_noff2 * assign29190_e22712_d_n5)), ((locals.var_noff2_dn6 * assign29190_e22712) + (locals.var_noff2 * assign29190_e22712_d_n6)), ((locals.var_noff2_dn7 * assign29190_e22712) + (locals.var_noff2 * assign29190_e22712_d_n7)), ((locals.var_noff2_dn8 * assign29190_e22712) + (locals.var_noff2 * assign29190_e22712_d_n8)), ((locals.var_noff2_dn9 * assign29190_e22712) + (locals.var_noff2 * assign29190_e22712_d_n9)), ((locals.var_noff2_dn10 * assign29190_e22712) + (locals.var_noff2 * assign29190_e22712_d_n10)), ((locals.var_noff2_dn11 * assign29190_e22712) + (locals.var_noff2 * assign29190_e22712_d_n11)), ((locals.var_noff2_dn12 * assign29190_e22712) + (locals.var_noff2 * assign29190_e22712_d_n12)),)
    } else {
        (locals.var_vgsteff2, locals.var_vgsteff2_dn3, locals.var_vgsteff2_dn4, locals.var_vgsteff2_dn5, locals.var_vgsteff2_dn6, locals.var_vgsteff2_dn7, locals.var_vgsteff2_dn8, locals.var_vgsteff2_dn9, locals.var_vgsteff2_dn10, locals.var_vgsteff2_dn11, locals.var_vgsteff2_dn12,)
    }
};
        locals.var_vgsteff2 = assign29190_e22715;
        locals.var_vgsteff2_dn3 = assign29190_e22715_d_n3;
        locals.var_vgsteff2_dn4 = assign29190_e22715_d_n4;
        locals.var_vgsteff2_dn5 = assign29190_e22715_d_n5;
        locals.var_vgsteff2_dn6 = assign29190_e22715_d_n6;
        locals.var_vgsteff2_dn7 = assign29190_e22715_d_n7;
        locals.var_vgsteff2_dn8 = assign29190_e22715_d_n8;
        locals.var_vgsteff2_dn9 = assign29190_e22715_d_n9;
        locals.var_vgsteff2_dn10 = assign29190_e22715_d_n10;
        locals.var_vgsteff2_dn11 = assign29190_e22715_d_n11;
        locals.var_vgsteff2_dn12 = assign29190_e22715_d_n12;

        let (assign29200_e22729, assign29200_e22729_d_n3, assign29200_e22729_d_n4, assign29200_e22729_d_n5, assign29200_e22729_d_n6, assign29200_e22729_d_n7, assign29200_e22729_d_n8, assign29200_e22729_d_n9, assign29200_e22729_d_n10, assign29200_e22729_d_n11, assign29200_e22729_d_n12,) = {
    if ((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) {
        let assign29200_e22724: f64 = (locals.var_vgst__blk1131 - locals.var_pparam_b4soidelvt);
        let assign29200_e22725: f64 = (locals.var_pparam_b4soimstarcv * assign29200_e22724);
        let assign29200_e22727: f64 = (assign29200_e22725 / locals.var_noff);
        (assign29200_e22727, (((((locals.var_pparam_b4soimstarcv_dn3 * assign29200_e22724) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn3 - locals.var_pparam_b4soidelvt_dn3))) * locals.var_noff) - (assign29200_e22725 * locals.var_noff_dn3)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn4 * assign29200_e22724) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn4 - locals.var_pparam_b4soidelvt_dn4))) * locals.var_noff) - (assign29200_e22725 * locals.var_noff_dn4)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn5 * assign29200_e22724) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn5 - locals.var_pparam_b4soidelvt_dn5))) * locals.var_noff) - (assign29200_e22725 * locals.var_noff_dn5)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn6 * assign29200_e22724) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn6 - locals.var_pparam_b4soidelvt_dn6))) * locals.var_noff) - (assign29200_e22725 * locals.var_noff_dn6)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn7 * assign29200_e22724) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn7 - locals.var_pparam_b4soidelvt_dn7))) * locals.var_noff) - (assign29200_e22725 * locals.var_noff_dn7)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn8 * assign29200_e22724) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn8 - locals.var_pparam_b4soidelvt_dn8))) * locals.var_noff) - (assign29200_e22725 * locals.var_noff_dn8)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn9 * assign29200_e22724) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn9 - locals.var_pparam_b4soidelvt_dn9))) * locals.var_noff) - (assign29200_e22725 * locals.var_noff_dn9)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn10 * assign29200_e22724) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn10 - locals.var_pparam_b4soidelvt_dn10))) * locals.var_noff) - (assign29200_e22725 * locals.var_noff_dn10)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn11 * assign29200_e22724) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn11 - locals.var_pparam_b4soidelvt_dn11))) * locals.var_noff) - (assign29200_e22725 * locals.var_noff_dn11)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn12 * assign29200_e22724) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn12 - locals.var_pparam_b4soidelvt_dn12))) * locals.var_noff) - (assign29200_e22725 * locals.var_noff_dn12)) / (locals.var_noff * locals.var_noff)),)
    } else {
        (locals.var_vgstnvt__blk1110, locals.var_vgstnvt__blk1110_dn3, locals.var_vgstnvt__blk1110_dn4, locals.var_vgstnvt__blk1110_dn5, locals.var_vgstnvt__blk1110_dn6, locals.var_vgstnvt__blk1110_dn7, locals.var_vgstnvt__blk1110_dn8, locals.var_vgstnvt__blk1110_dn9, locals.var_vgstnvt__blk1110_dn10, locals.var_vgstnvt__blk1110_dn11, locals.var_vgstnvt__blk1110_dn12,)
    }
};
        locals.var_vgstnvt__blk1110 = assign29200_e22729;
        locals.var_vgstnvt__blk1110_dn3 = assign29200_e22729_d_n3;
        locals.var_vgstnvt__blk1110_dn4 = assign29200_e22729_d_n4;
        locals.var_vgstnvt__blk1110_dn5 = assign29200_e22729_d_n5;
        locals.var_vgstnvt__blk1110_dn6 = assign29200_e22729_d_n6;
        locals.var_vgstnvt__blk1110_dn7 = assign29200_e22729_d_n7;
        locals.var_vgstnvt__blk1110_dn8 = assign29200_e22729_d_n8;
        locals.var_vgstnvt__blk1110_dn9 = assign29200_e22729_d_n9;
        locals.var_vgstnvt__blk1110_dn10 = assign29200_e22729_d_n10;
        locals.var_vgstnvt__blk1110_dn11 = assign29200_e22729_d_n11;
        locals.var_vgstnvt__blk1110_dn12 = assign29200_e22729_d_n12;

    }

    pub(super) fn stamp_transient_block_78(
        locals: &mut StampLocals,
    ) {
        let (assign29210_e22747, assign29210_e22747_d_n3, assign29210_e22747_d_n4, assign29210_e22747_d_n5, assign29210_e22747_d_n6, assign29210_e22747_d_n7, assign29210_e22747_d_n8, assign29210_e22747_d_n9, assign29210_e22747_d_n10, assign29210_e22747_d_n11, assign29210_e22747_d_n12,) = {
    if ((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) {
        let assign29210_e22738: f64 = (1.0 - locals.var_pparam_b4soimstarcv);
        let assign29210_e22741: f64 = (locals.var_vgst__blk1131 - locals.var_pparam_b4soidelvt);
        let assign29210_e22742: f64 = (assign29210_e22738 * assign29210_e22741);
        let assign29210_e22743: f64 = (locals.var_pparam_b4soivoffcv - assign29210_e22742);
        let assign29210_e22745: f64 = (assign29210_e22743 / locals.var_noff);
        (assign29210_e22745, ((((locals.var_pparam_b4soivoffcv_dn3 - (((-locals.var_pparam_b4soimstarcv_dn3) * assign29210_e22741) + (assign29210_e22738 * (locals.var_vgst__blk1131_dn3 - locals.var_pparam_b4soidelvt_dn3)))) * locals.var_noff) - (assign29210_e22743 * locals.var_noff_dn3)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn4 - (((-locals.var_pparam_b4soimstarcv_dn4) * assign29210_e22741) + (assign29210_e22738 * (locals.var_vgst__blk1131_dn4 - locals.var_pparam_b4soidelvt_dn4)))) * locals.var_noff) - (assign29210_e22743 * locals.var_noff_dn4)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn5 - (((-locals.var_pparam_b4soimstarcv_dn5) * assign29210_e22741) + (assign29210_e22738 * (locals.var_vgst__blk1131_dn5 - locals.var_pparam_b4soidelvt_dn5)))) * locals.var_noff) - (assign29210_e22743 * locals.var_noff_dn5)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn6 - (((-locals.var_pparam_b4soimstarcv_dn6) * assign29210_e22741) + (assign29210_e22738 * (locals.var_vgst__blk1131_dn6 - locals.var_pparam_b4soidelvt_dn6)))) * locals.var_noff) - (assign29210_e22743 * locals.var_noff_dn6)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn7 - (((-locals.var_pparam_b4soimstarcv_dn7) * assign29210_e22741) + (assign29210_e22738 * (locals.var_vgst__blk1131_dn7 - locals.var_pparam_b4soidelvt_dn7)))) * locals.var_noff) - (assign29210_e22743 * locals.var_noff_dn7)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn8 - (((-locals.var_pparam_b4soimstarcv_dn8) * assign29210_e22741) + (assign29210_e22738 * (locals.var_vgst__blk1131_dn8 - locals.var_pparam_b4soidelvt_dn8)))) * locals.var_noff) - (assign29210_e22743 * locals.var_noff_dn8)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn9 - (((-locals.var_pparam_b4soimstarcv_dn9) * assign29210_e22741) + (assign29210_e22738 * (locals.var_vgst__blk1131_dn9 - locals.var_pparam_b4soidelvt_dn9)))) * locals.var_noff) - (assign29210_e22743 * locals.var_noff_dn9)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn10 - (((-locals.var_pparam_b4soimstarcv_dn10) * assign29210_e22741) + (assign29210_e22738 * (locals.var_vgst__blk1131_dn10 - locals.var_pparam_b4soidelvt_dn10)))) * locals.var_noff) - (assign29210_e22743 * locals.var_noff_dn10)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn11 - (((-locals.var_pparam_b4soimstarcv_dn11) * assign29210_e22741) + (assign29210_e22738 * (locals.var_vgst__blk1131_dn11 - locals.var_pparam_b4soidelvt_dn11)))) * locals.var_noff) - (assign29210_e22743 * locals.var_noff_dn11)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn12 - (((-locals.var_pparam_b4soimstarcv_dn12) * assign29210_e22741) + (assign29210_e22738 * (locals.var_vgst__blk1131_dn12 - locals.var_pparam_b4soidelvt_dn12)))) * locals.var_noff) - (assign29210_e22743 * locals.var_noff_dn12)) / (locals.var_noff * locals.var_noff)),)
    } else {
        (locals.var_exparg__blk1134, locals.var_exparg__blk1134_dn3, locals.var_exparg__blk1134_dn4, locals.var_exparg__blk1134_dn5, locals.var_exparg__blk1134_dn6, locals.var_exparg__blk1134_dn7, locals.var_exparg__blk1134_dn8, locals.var_exparg__blk1134_dn9, locals.var_exparg__blk1134_dn10, locals.var_exparg__blk1134_dn11, locals.var_exparg__blk1134_dn12,)
    }
};
        locals.var_exparg__blk1134 = assign29210_e22747;
        locals.var_exparg__blk1134_dn3 = assign29210_e22747_d_n3;
        locals.var_exparg__blk1134_dn4 = assign29210_e22747_d_n4;
        locals.var_exparg__blk1134_dn5 = assign29210_e22747_d_n5;
        locals.var_exparg__blk1134_dn6 = assign29210_e22747_d_n6;
        locals.var_exparg__blk1134_dn7 = assign29210_e22747_d_n7;
        locals.var_exparg__blk1134_dn8 = assign29210_e22747_d_n8;
        locals.var_exparg__blk1134_dn9 = assign29210_e22747_d_n9;
        locals.var_exparg__blk1134_dn10 = assign29210_e22747_d_n10;
        locals.var_exparg__blk1134_dn11 = assign29210_e22747_d_n11;
        locals.var_exparg__blk1134_dn12 = assign29210_e22747_d_n12;

        let assign29220_e22750: f64 = if locals.var_vgstnvt__blk1110 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1693 = assign29220_e22750;

        let (assign29230_e22762, assign29230_e22762_d_n3, assign29230_e22762_d_n4, assign29230_e22762_d_n5, assign29230_e22762_d_n6, assign29230_e22762_d_n7, assign29230_e22762_d_n8, assign29230_e22762_d_n9, assign29230_e22762_d_n10, assign29230_e22762_d_n11, assign29230_e22762_d_n12,) = {
    if (((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1693 != 0.0)) {
        let assign29230_e22760: f64 = (locals.var_vgst__blk1131 - locals.var_pparam_b4soidelvt);
        (assign29230_e22760, (locals.var_vgst__blk1131_dn3 - locals.var_pparam_b4soidelvt_dn3), (locals.var_vgst__blk1131_dn4 - locals.var_pparam_b4soidelvt_dn4), (locals.var_vgst__blk1131_dn5 - locals.var_pparam_b4soidelvt_dn5), (locals.var_vgst__blk1131_dn6 - locals.var_pparam_b4soidelvt_dn6), (locals.var_vgst__blk1131_dn7 - locals.var_pparam_b4soidelvt_dn7), (locals.var_vgst__blk1131_dn8 - locals.var_pparam_b4soidelvt_dn8), (locals.var_vgst__blk1131_dn9 - locals.var_pparam_b4soidelvt_dn9), (locals.var_vgst__blk1131_dn10 - locals.var_pparam_b4soidelvt_dn10), (locals.var_vgst__blk1131_dn11 - locals.var_pparam_b4soidelvt_dn11), (locals.var_vgst__blk1131_dn12 - locals.var_pparam_b4soidelvt_dn12),)
    } else {
        (locals.var_vgsteff__blk1175, locals.var_vgsteff__blk1175_dn3, locals.var_vgsteff__blk1175_dn4, locals.var_vgsteff__blk1175_dn5, locals.var_vgsteff__blk1175_dn6, locals.var_vgsteff__blk1175_dn7, locals.var_vgsteff__blk1175_dn8, locals.var_vgsteff__blk1175_dn9, locals.var_vgsteff__blk1175_dn10, locals.var_vgsteff__blk1175_dn11, locals.var_vgsteff__blk1175_dn12,)
    }
};
        locals.var_vgsteff__blk1175 = assign29230_e22762;
        locals.var_vgsteff__blk1175_dn3 = assign29230_e22762_d_n3;
        locals.var_vgsteff__blk1175_dn4 = assign29230_e22762_d_n4;
        locals.var_vgsteff__blk1175_dn5 = assign29230_e22762_d_n5;
        locals.var_vgsteff__blk1175_dn6 = assign29230_e22762_d_n6;
        locals.var_vgsteff__blk1175_dn7 = assign29230_e22762_d_n7;
        locals.var_vgsteff__blk1175_dn8 = assign29230_e22762_d_n8;
        locals.var_vgsteff__blk1175_dn9 = assign29230_e22762_d_n9;
        locals.var_vgsteff__blk1175_dn10 = assign29230_e22762_d_n10;
        locals.var_vgsteff__blk1175_dn11 = assign29230_e22762_d_n11;
        locals.var_vgsteff__blk1175_dn12 = assign29230_e22762_d_n12;

        let assign29240_e22765: f64 = if locals.var_exparg__blk1134 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1694 = assign29240_e22765;

        let (assign29250_e22784, assign29250_e22784_d_n3, assign29250_e22784_d_n4, assign29250_e22784_d_n5, assign29250_e22784_d_n6, assign29250_e22784_d_n7, assign29250_e22784_d_n8, assign29250_e22784_d_n9, assign29250_e22784_d_n10, assign29250_e22784_d_n11, assign29250_e22784_d_n12,) = {
    if ((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1693 == 0.0)) && (locals.var_guard1694 != 0.0)) {
        let assign29250_e22778: f64 = (locals.var_vgst__blk1131 - locals.var_pparam_b4soidelvt);
        let assign29250_e22780: f64 = (assign29250_e22778 - locals.var_pparam_b4soivoffcv);
        let assign29250_e22782: f64 = (assign29250_e22780 / locals.var_noff);
        (assign29250_e22782, (((((locals.var_vgst__blk1131_dn3 - locals.var_pparam_b4soidelvt_dn3) - locals.var_pparam_b4soivoffcv_dn3) * locals.var_noff) - (assign29250_e22780 * locals.var_noff_dn3)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk1131_dn4 - locals.var_pparam_b4soidelvt_dn4) - locals.var_pparam_b4soivoffcv_dn4) * locals.var_noff) - (assign29250_e22780 * locals.var_noff_dn4)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk1131_dn5 - locals.var_pparam_b4soidelvt_dn5) - locals.var_pparam_b4soivoffcv_dn5) * locals.var_noff) - (assign29250_e22780 * locals.var_noff_dn5)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk1131_dn6 - locals.var_pparam_b4soidelvt_dn6) - locals.var_pparam_b4soivoffcv_dn6) * locals.var_noff) - (assign29250_e22780 * locals.var_noff_dn6)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk1131_dn7 - locals.var_pparam_b4soidelvt_dn7) - locals.var_pparam_b4soivoffcv_dn7) * locals.var_noff) - (assign29250_e22780 * locals.var_noff_dn7)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk1131_dn8 - locals.var_pparam_b4soidelvt_dn8) - locals.var_pparam_b4soivoffcv_dn8) * locals.var_noff) - (assign29250_e22780 * locals.var_noff_dn8)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk1131_dn9 - locals.var_pparam_b4soidelvt_dn9) - locals.var_pparam_b4soivoffcv_dn9) * locals.var_noff) - (assign29250_e22780 * locals.var_noff_dn9)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk1131_dn10 - locals.var_pparam_b4soidelvt_dn10) - locals.var_pparam_b4soivoffcv_dn10) * locals.var_noff) - (assign29250_e22780 * locals.var_noff_dn10)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk1131_dn11 - locals.var_pparam_b4soidelvt_dn11) - locals.var_pparam_b4soivoffcv_dn11) * locals.var_noff) - (assign29250_e22780 * locals.var_noff_dn11)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk1131_dn12 - locals.var_pparam_b4soidelvt_dn12) - locals.var_pparam_b4soivoffcv_dn12) * locals.var_noff) - (assign29250_e22780 * locals.var_noff_dn12)) / (locals.var_noff * locals.var_noff)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign29250_e22784;
        locals.var_t0__blk1144_dn3 = assign29250_e22784_d_n3;
        locals.var_t0__blk1144_dn4 = assign29250_e22784_d_n4;
        locals.var_t0__blk1144_dn5 = assign29250_e22784_d_n5;
        locals.var_t0__blk1144_dn6 = assign29250_e22784_d_n6;
        locals.var_t0__blk1144_dn7 = assign29250_e22784_d_n7;
        locals.var_t0__blk1144_dn8 = assign29250_e22784_d_n8;
        locals.var_t0__blk1144_dn9 = assign29250_e22784_d_n9;
        locals.var_t0__blk1144_dn10 = assign29250_e22784_d_n10;
        locals.var_t0__blk1144_dn11 = assign29250_e22784_d_n11;
        locals.var_t0__blk1144_dn12 = assign29250_e22784_d_n12;

        let (assign29260_e22798, assign29260_e22798_d_n3, assign29260_e22798_d_n4, assign29260_e22798_d_n5, assign29260_e22798_d_n6, assign29260_e22798_d_n7, assign29260_e22798_d_n8, assign29260_e22798_d_n9, assign29260_e22798_d_n10, assign29260_e22798_d_n11, assign29260_e22798_d_n12,) = {
    if ((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1693 == 0.0)) && (locals.var_guard1694 != 0.0)) {
        let assign29260_e22796: f64 = (locals.var_t0__blk1144).exp();
        (assign29260_e22796, (assign29260_e22796 * locals.var_t0__blk1144_dn3), (assign29260_e22796 * locals.var_t0__blk1144_dn4), (assign29260_e22796 * locals.var_t0__blk1144_dn5), (assign29260_e22796 * locals.var_t0__blk1144_dn6), (assign29260_e22796 * locals.var_t0__blk1144_dn7), (assign29260_e22796 * locals.var_t0__blk1144_dn8), (assign29260_e22796 * locals.var_t0__blk1144_dn9), (assign29260_e22796 * locals.var_t0__blk1144_dn10), (assign29260_e22796 * locals.var_t0__blk1144_dn11), (assign29260_e22796 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_expvgst__blk1111, locals.var_expvgst__blk1111_dn3, locals.var_expvgst__blk1111_dn4, locals.var_expvgst__blk1111_dn5, locals.var_expvgst__blk1111_dn6, locals.var_expvgst__blk1111_dn7, locals.var_expvgst__blk1111_dn8, locals.var_expvgst__blk1111_dn9, locals.var_expvgst__blk1111_dn10, locals.var_expvgst__blk1111_dn11, locals.var_expvgst__blk1111_dn12,)
    }
};
        locals.var_expvgst__blk1111 = assign29260_e22798;
        locals.var_expvgst__blk1111_dn3 = assign29260_e22798_d_n3;
        locals.var_expvgst__blk1111_dn4 = assign29260_e22798_d_n4;
        locals.var_expvgst__blk1111_dn5 = assign29260_e22798_d_n5;
        locals.var_expvgst__blk1111_dn6 = assign29260_e22798_d_n6;
        locals.var_expvgst__blk1111_dn7 = assign29260_e22798_d_n7;
        locals.var_expvgst__blk1111_dn8 = assign29260_e22798_d_n8;
        locals.var_expvgst__blk1111_dn9 = assign29260_e22798_d_n9;
        locals.var_expvgst__blk1111_dn10 = assign29260_e22798_d_n10;
        locals.var_expvgst__blk1111_dn11 = assign29260_e22798_d_n11;
        locals.var_expvgst__blk1111_dn12 = assign29260_e22798_d_n12;

        let (assign29270_e22817, assign29270_e22817_d_n3, assign29270_e22817_d_n4, assign29270_e22817_d_n5, assign29270_e22817_d_n6, assign29270_e22817_d_n7, assign29270_e22817_d_n8, assign29270_e22817_d_n9, assign29270_e22817_d_n10, assign29270_e22817_d_n11, assign29270_e22817_d_n12,) = {
    if ((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1693 == 0.0)) && (locals.var_guard1694 != 0.0)) {
        let assign29270_e22811: f64 = (locals.var_vtm * locals.var_cdep0);
        let assign29270_e22813: f64 = (assign29270_e22811 / locals.var_b4soicox);
        let assign29270_e22815: f64 = (assign29270_e22813 * locals.var_expvgst__blk1111);
        (assign29270_e22815, ((((locals.var_vtm * locals.var_cdep0_dn3) / locals.var_b4soicox) * locals.var_expvgst__blk1111) + (assign29270_e22813 * locals.var_expvgst__blk1111_dn3)), ((((locals.var_vtm * locals.var_cdep0_dn4) / locals.var_b4soicox) * locals.var_expvgst__blk1111) + (assign29270_e22813 * locals.var_expvgst__blk1111_dn4)), ((((locals.var_vtm * locals.var_cdep0_dn5) / locals.var_b4soicox) * locals.var_expvgst__blk1111) + (assign29270_e22813 * locals.var_expvgst__blk1111_dn5)), (((((locals.var_vtm_dn6 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn6)) / locals.var_b4soicox) * locals.var_expvgst__blk1111) + (assign29270_e22813 * locals.var_expvgst__blk1111_dn6)), ((((locals.var_vtm * locals.var_cdep0_dn7) / locals.var_b4soicox) * locals.var_expvgst__blk1111) + (assign29270_e22813 * locals.var_expvgst__blk1111_dn7)), ((((locals.var_vtm * locals.var_cdep0_dn8) / locals.var_b4soicox) * locals.var_expvgst__blk1111) + (assign29270_e22813 * locals.var_expvgst__blk1111_dn8)), ((((locals.var_vtm * locals.var_cdep0_dn9) / locals.var_b4soicox) * locals.var_expvgst__blk1111) + (assign29270_e22813 * locals.var_expvgst__blk1111_dn9)), ((((locals.var_vtm * locals.var_cdep0_dn10) / locals.var_b4soicox) * locals.var_expvgst__blk1111) + (assign29270_e22813 * locals.var_expvgst__blk1111_dn10)), ((((locals.var_vtm * locals.var_cdep0_dn11) / locals.var_b4soicox) * locals.var_expvgst__blk1111) + (assign29270_e22813 * locals.var_expvgst__blk1111_dn11)), ((((locals.var_vtm * locals.var_cdep0_dn12) / locals.var_b4soicox) * locals.var_expvgst__blk1111) + (assign29270_e22813 * locals.var_expvgst__blk1111_dn12)),)
    } else {
        (locals.var_vgsteff__blk1175, locals.var_vgsteff__blk1175_dn3, locals.var_vgsteff__blk1175_dn4, locals.var_vgsteff__blk1175_dn5, locals.var_vgsteff__blk1175_dn6, locals.var_vgsteff__blk1175_dn7, locals.var_vgsteff__blk1175_dn8, locals.var_vgsteff__blk1175_dn9, locals.var_vgsteff__blk1175_dn10, locals.var_vgsteff__blk1175_dn11, locals.var_vgsteff__blk1175_dn12,)
    }
};
        locals.var_vgsteff__blk1175 = assign29270_e22817;
        locals.var_vgsteff__blk1175_dn3 = assign29270_e22817_d_n3;
        locals.var_vgsteff__blk1175_dn4 = assign29270_e22817_d_n4;
        locals.var_vgsteff__blk1175_dn5 = assign29270_e22817_d_n5;
        locals.var_vgsteff__blk1175_dn6 = assign29270_e22817_d_n6;
        locals.var_vgsteff__blk1175_dn7 = assign29270_e22817_d_n7;
        locals.var_vgsteff__blk1175_dn8 = assign29270_e22817_d_n8;
        locals.var_vgsteff__blk1175_dn9 = assign29270_e22817_d_n9;
        locals.var_vgsteff__blk1175_dn10 = assign29270_e22817_d_n10;
        locals.var_vgsteff__blk1175_dn11 = assign29270_e22817_d_n11;
        locals.var_vgsteff__blk1175_dn12 = assign29270_e22817_d_n12;

        let (assign29280_e22832, assign29280_e22832_d_n3, assign29280_e22832_d_n4, assign29280_e22832_d_n5, assign29280_e22832_d_n6, assign29280_e22832_d_n7, assign29280_e22832_d_n8, assign29280_e22832_d_n9, assign29280_e22832_d_n10, assign29280_e22832_d_n11, assign29280_e22832_d_n12,) = {
    if ((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1693 == 0.0)) && (locals.var_guard1694 == 0.0)) {
        let assign29280_e22830: f64 = (locals.var_vgstnvt__blk1110).exp();
        (assign29280_e22830, (assign29280_e22830 * locals.var_vgstnvt__blk1110_dn3), (assign29280_e22830 * locals.var_vgstnvt__blk1110_dn4), (assign29280_e22830 * locals.var_vgstnvt__blk1110_dn5), (assign29280_e22830 * locals.var_vgstnvt__blk1110_dn6), (assign29280_e22830 * locals.var_vgstnvt__blk1110_dn7), (assign29280_e22830 * locals.var_vgstnvt__blk1110_dn8), (assign29280_e22830 * locals.var_vgstnvt__blk1110_dn9), (assign29280_e22830 * locals.var_vgstnvt__blk1110_dn10), (assign29280_e22830 * locals.var_vgstnvt__blk1110_dn11), (assign29280_e22830 * locals.var_vgstnvt__blk1110_dn12),)
    } else {
        (locals.var_expvgst__blk1111, locals.var_expvgst__blk1111_dn3, locals.var_expvgst__blk1111_dn4, locals.var_expvgst__blk1111_dn5, locals.var_expvgst__blk1111_dn6, locals.var_expvgst__blk1111_dn7, locals.var_expvgst__blk1111_dn8, locals.var_expvgst__blk1111_dn9, locals.var_expvgst__blk1111_dn10, locals.var_expvgst__blk1111_dn11, locals.var_expvgst__blk1111_dn12,)
    }
};
        locals.var_expvgst__blk1111 = assign29280_e22832;
        locals.var_expvgst__blk1111_dn3 = assign29280_e22832_d_n3;
        locals.var_expvgst__blk1111_dn4 = assign29280_e22832_d_n4;
        locals.var_expvgst__blk1111_dn5 = assign29280_e22832_d_n5;
        locals.var_expvgst__blk1111_dn6 = assign29280_e22832_d_n6;
        locals.var_expvgst__blk1111_dn7 = assign29280_e22832_d_n7;
        locals.var_expvgst__blk1111_dn8 = assign29280_e22832_d_n8;
        locals.var_expvgst__blk1111_dn9 = assign29280_e22832_d_n9;
        locals.var_expvgst__blk1111_dn10 = assign29280_e22832_d_n10;
        locals.var_expvgst__blk1111_dn11 = assign29280_e22832_d_n11;
        locals.var_expvgst__blk1111_dn12 = assign29280_e22832_d_n12;

        let (assign29290_e22859, assign29290_e22859_d_n3, assign29290_e22859_d_n4, assign29290_e22859_d_n5, assign29290_e22859_d_n6, assign29290_e22859_d_n7, assign29290_e22859_d_n8, assign29290_e22859_d_n9, assign29290_e22859_d_n10, assign29290_e22859_d_n11, assign29290_e22859_d_n12,) = {
    if ((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1693 == 0.0)) && (locals.var_guard1694 == 0.0)) {
        let assign29290_e22847: f64 = (1.0 + locals.var_expvgst__blk1111);
        let (assign29290_e22856, assign29290_e22856_d_n3, assign29290_e22856_d_n4, assign29290_e22856_d_n5, assign29290_e22856_d_n6, assign29290_e22856_d_n7, assign29290_e22856_d_n8, assign29290_e22856_d_n9, assign29290_e22856_d_n10, assign29290_e22856_d_n11, assign29290_e22856_d_n12,) = {
            if (assign29290_e22847 > 1e-38) {
                let assign29290_e22852: f64 = (1.0 + locals.var_expvgst__blk1111);
                let assign29290_e22853: f64 = (assign29290_e22852).ln();
                (assign29290_e22853, (locals.var_expvgst__blk1111_dn3 / assign29290_e22852), (locals.var_expvgst__blk1111_dn4 / assign29290_e22852), (locals.var_expvgst__blk1111_dn5 / assign29290_e22852), (locals.var_expvgst__blk1111_dn6 / assign29290_e22852), (locals.var_expvgst__blk1111_dn7 / assign29290_e22852), (locals.var_expvgst__blk1111_dn8 / assign29290_e22852), (locals.var_expvgst__blk1111_dn9 / assign29290_e22852), (locals.var_expvgst__blk1111_dn10 / assign29290_e22852), (locals.var_expvgst__blk1111_dn11 / assign29290_e22852), (locals.var_expvgst__blk1111_dn12 / assign29290_e22852),)
            } else {
                let assign29290_e22855: f64 = (-87.49823353377374);
                (assign29290_e22855, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign29290_e22857: f64 = (locals.var_noff * assign29290_e22856);
        (assign29290_e22857, ((locals.var_noff_dn3 * assign29290_e22856) + (locals.var_noff * assign29290_e22856_d_n3)), ((locals.var_noff_dn4 * assign29290_e22856) + (locals.var_noff * assign29290_e22856_d_n4)), ((locals.var_noff_dn5 * assign29290_e22856) + (locals.var_noff * assign29290_e22856_d_n5)), ((locals.var_noff_dn6 * assign29290_e22856) + (locals.var_noff * assign29290_e22856_d_n6)), ((locals.var_noff_dn7 * assign29290_e22856) + (locals.var_noff * assign29290_e22856_d_n7)), ((locals.var_noff_dn8 * assign29290_e22856) + (locals.var_noff * assign29290_e22856_d_n8)), ((locals.var_noff_dn9 * assign29290_e22856) + (locals.var_noff * assign29290_e22856_d_n9)), ((locals.var_noff_dn10 * assign29290_e22856) + (locals.var_noff * assign29290_e22856_d_n10)), ((locals.var_noff_dn11 * assign29290_e22856) + (locals.var_noff * assign29290_e22856_d_n11)), ((locals.var_noff_dn12 * assign29290_e22856) + (locals.var_noff * assign29290_e22856_d_n12)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign29290_e22859;
        locals.var_t1__blk1145_dn3 = assign29290_e22859_d_n3;
        locals.var_t1__blk1145_dn4 = assign29290_e22859_d_n4;
        locals.var_t1__blk1145_dn5 = assign29290_e22859_d_n5;
        locals.var_t1__blk1145_dn6 = assign29290_e22859_d_n6;
        locals.var_t1__blk1145_dn7 = assign29290_e22859_d_n7;
        locals.var_t1__blk1145_dn8 = assign29290_e22859_d_n8;
        locals.var_t1__blk1145_dn9 = assign29290_e22859_d_n9;
        locals.var_t1__blk1145_dn10 = assign29290_e22859_d_n10;
        locals.var_t1__blk1145_dn11 = assign29290_e22859_d_n11;
        locals.var_t1__blk1145_dn12 = assign29290_e22859_d_n12;

        let (assign29300_e22885, assign29300_e22885_d_n3, assign29300_e22885_d_n4, assign29300_e22885_d_n5, assign29300_e22885_d_n6, assign29300_e22885_d_n7, assign29300_e22885_d_n8, assign29300_e22885_d_n9, assign29300_e22885_d_n10, assign29300_e22885_d_n11, assign29300_e22885_d_n12,) = {
    if ((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1693 == 0.0)) && (locals.var_guard1694 == 0.0)) {
        let assign29300_e22872: f64 = (-locals.var_b4soicox);
        let assign29300_e22875: f64 = (locals.var_vtm * locals.var_cdep0);
        let assign29300_e22876: f64 = (assign29300_e22872 / assign29300_e22875);
        let assign29300_e22878: f64 = (locals.var_exparg__blk1134).exp();
        let assign29300_e22879: f64 = (assign29300_e22876 * assign29300_e22878);
        let assign29300_e22882: f64 = (1.0 - locals.var_pparam_b4soimstarcv);
        let assign29300_e22883: f64 = (assign29300_e22879 * assign29300_e22882);
        (assign29300_e22883, (((((-((assign29300_e22872 * (locals.var_vtm * locals.var_cdep0_dn3)) / (assign29300_e22875 * assign29300_e22875))) * assign29300_e22878) + (assign29300_e22876 * (assign29300_e22878 * locals.var_exparg__blk1134_dn3))) * assign29300_e22882) + (assign29300_e22879 * (-locals.var_pparam_b4soimstarcv_dn3))), (((((-((assign29300_e22872 * (locals.var_vtm * locals.var_cdep0_dn4)) / (assign29300_e22875 * assign29300_e22875))) * assign29300_e22878) + (assign29300_e22876 * (assign29300_e22878 * locals.var_exparg__blk1134_dn4))) * assign29300_e22882) + (assign29300_e22879 * (-locals.var_pparam_b4soimstarcv_dn4))), (((((-((assign29300_e22872 * (locals.var_vtm * locals.var_cdep0_dn5)) / (assign29300_e22875 * assign29300_e22875))) * assign29300_e22878) + (assign29300_e22876 * (assign29300_e22878 * locals.var_exparg__blk1134_dn5))) * assign29300_e22882) + (assign29300_e22879 * (-locals.var_pparam_b4soimstarcv_dn5))), (((((-((assign29300_e22872 * ((locals.var_vtm_dn6 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn6))) / (assign29300_e22875 * assign29300_e22875))) * assign29300_e22878) + (assign29300_e22876 * (assign29300_e22878 * locals.var_exparg__blk1134_dn6))) * assign29300_e22882) + (assign29300_e22879 * (-locals.var_pparam_b4soimstarcv_dn6))), (((((-((assign29300_e22872 * (locals.var_vtm * locals.var_cdep0_dn7)) / (assign29300_e22875 * assign29300_e22875))) * assign29300_e22878) + (assign29300_e22876 * (assign29300_e22878 * locals.var_exparg__blk1134_dn7))) * assign29300_e22882) + (assign29300_e22879 * (-locals.var_pparam_b4soimstarcv_dn7))), (((((-((assign29300_e22872 * (locals.var_vtm * locals.var_cdep0_dn8)) / (assign29300_e22875 * assign29300_e22875))) * assign29300_e22878) + (assign29300_e22876 * (assign29300_e22878 * locals.var_exparg__blk1134_dn8))) * assign29300_e22882) + (assign29300_e22879 * (-locals.var_pparam_b4soimstarcv_dn8))), (((((-((assign29300_e22872 * (locals.var_vtm * locals.var_cdep0_dn9)) / (assign29300_e22875 * assign29300_e22875))) * assign29300_e22878) + (assign29300_e22876 * (assign29300_e22878 * locals.var_exparg__blk1134_dn9))) * assign29300_e22882) + (assign29300_e22879 * (-locals.var_pparam_b4soimstarcv_dn9))), (((((-((assign29300_e22872 * (locals.var_vtm * locals.var_cdep0_dn10)) / (assign29300_e22875 * assign29300_e22875))) * assign29300_e22878) + (assign29300_e22876 * (assign29300_e22878 * locals.var_exparg__blk1134_dn10))) * assign29300_e22882) + (assign29300_e22879 * (-locals.var_pparam_b4soimstarcv_dn10))), (((((-((assign29300_e22872 * (locals.var_vtm * locals.var_cdep0_dn11)) / (assign29300_e22875 * assign29300_e22875))) * assign29300_e22878) + (assign29300_e22876 * (assign29300_e22878 * locals.var_exparg__blk1134_dn11))) * assign29300_e22882) + (assign29300_e22879 * (-locals.var_pparam_b4soimstarcv_dn11))), (((((-((assign29300_e22872 * (locals.var_vtm * locals.var_cdep0_dn12)) / (assign29300_e22875 * assign29300_e22875))) * assign29300_e22878) + (assign29300_e22876 * (assign29300_e22878 * locals.var_exparg__blk1134_dn12))) * assign29300_e22882) + (assign29300_e22879 * (-locals.var_pparam_b4soimstarcv_dn12))),)
    } else {
        (locals.var_dt2_dvg, locals.var_dt2_dvg_dn3, locals.var_dt2_dvg_dn4, locals.var_dt2_dvg_dn5, locals.var_dt2_dvg_dn6, locals.var_dt2_dvg_dn7, locals.var_dt2_dvg_dn8, locals.var_dt2_dvg_dn9, locals.var_dt2_dvg_dn10, locals.var_dt2_dvg_dn11, locals.var_dt2_dvg_dn12,)
    }
};
        locals.var_dt2_dvg = assign29300_e22885;
        locals.var_dt2_dvg_dn3 = assign29300_e22885_d_n3;
        locals.var_dt2_dvg_dn4 = assign29300_e22885_d_n4;
        locals.var_dt2_dvg_dn5 = assign29300_e22885_d_n5;
        locals.var_dt2_dvg_dn6 = assign29300_e22885_d_n6;
        locals.var_dt2_dvg_dn7 = assign29300_e22885_d_n7;
        locals.var_dt2_dvg_dn8 = assign29300_e22885_d_n8;
        locals.var_dt2_dvg_dn9 = assign29300_e22885_d_n9;
        locals.var_dt2_dvg_dn10 = assign29300_e22885_d_n10;
        locals.var_dt2_dvg_dn11 = assign29300_e22885_d_n11;
        locals.var_dt2_dvg_dn12 = assign29300_e22885_d_n12;

        let (assign29310_e22907, assign29310_e22907_d_n3, assign29310_e22907_d_n4, assign29310_e22907_d_n5, assign29310_e22907_d_n6, assign29310_e22907_d_n7, assign29310_e22907_d_n8, assign29310_e22907_d_n9, assign29310_e22907_d_n10, assign29310_e22907_d_n11, assign29310_e22907_d_n12,) = {
    if ((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1693 == 0.0)) && (locals.var_guard1694 == 0.0)) {
        let assign29310_e22900: f64 = (locals.var_noff * locals.var_dt2_dvg);
        let assign29310_e22903: f64 = (1.0 - locals.var_pparam_b4soimstarcv);
        let assign29310_e22904: f64 = (assign29310_e22900 / assign29310_e22903);
        let assign29310_e22905: f64 = (locals.var_pparam_b4soimstarcv - assign29310_e22904);
        (assign29310_e22905, (locals.var_pparam_b4soimstarcv_dn3 - (((((locals.var_noff_dn3 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn3)) * assign29310_e22903) - (assign29310_e22900 * (-locals.var_pparam_b4soimstarcv_dn3))) / (assign29310_e22903 * assign29310_e22903))), (locals.var_pparam_b4soimstarcv_dn4 - (((((locals.var_noff_dn4 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn4)) * assign29310_e22903) - (assign29310_e22900 * (-locals.var_pparam_b4soimstarcv_dn4))) / (assign29310_e22903 * assign29310_e22903))), (locals.var_pparam_b4soimstarcv_dn5 - (((((locals.var_noff_dn5 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn5)) * assign29310_e22903) - (assign29310_e22900 * (-locals.var_pparam_b4soimstarcv_dn5))) / (assign29310_e22903 * assign29310_e22903))), (locals.var_pparam_b4soimstarcv_dn6 - (((((locals.var_noff_dn6 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn6)) * assign29310_e22903) - (assign29310_e22900 * (-locals.var_pparam_b4soimstarcv_dn6))) / (assign29310_e22903 * assign29310_e22903))), (locals.var_pparam_b4soimstarcv_dn7 - (((((locals.var_noff_dn7 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn7)) * assign29310_e22903) - (assign29310_e22900 * (-locals.var_pparam_b4soimstarcv_dn7))) / (assign29310_e22903 * assign29310_e22903))), (locals.var_pparam_b4soimstarcv_dn8 - (((((locals.var_noff_dn8 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn8)) * assign29310_e22903) - (assign29310_e22900 * (-locals.var_pparam_b4soimstarcv_dn8))) / (assign29310_e22903 * assign29310_e22903))), (locals.var_pparam_b4soimstarcv_dn9 - (((((locals.var_noff_dn9 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn9)) * assign29310_e22903) - (assign29310_e22900 * (-locals.var_pparam_b4soimstarcv_dn9))) / (assign29310_e22903 * assign29310_e22903))), (locals.var_pparam_b4soimstarcv_dn10 - (((((locals.var_noff_dn10 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn10)) * assign29310_e22903) - (assign29310_e22900 * (-locals.var_pparam_b4soimstarcv_dn10))) / (assign29310_e22903 * assign29310_e22903))), (locals.var_pparam_b4soimstarcv_dn11 - (((((locals.var_noff_dn11 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn11)) * assign29310_e22903) - (assign29310_e22900 * (-locals.var_pparam_b4soimstarcv_dn11))) / (assign29310_e22903 * assign29310_e22903))), (locals.var_pparam_b4soimstarcv_dn12 - (((((locals.var_noff_dn12 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn12)) * assign29310_e22903) - (assign29310_e22900 * (-locals.var_pparam_b4soimstarcv_dn12))) / (assign29310_e22903 * assign29310_e22903))),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign29310_e22907;
        locals.var_t2__blk1146_dn3 = assign29310_e22907_d_n3;
        locals.var_t2__blk1146_dn4 = assign29310_e22907_d_n4;
        locals.var_t2__blk1146_dn5 = assign29310_e22907_d_n5;
        locals.var_t2__blk1146_dn6 = assign29310_e22907_d_n6;
        locals.var_t2__blk1146_dn7 = assign29310_e22907_d_n7;
        locals.var_t2__blk1146_dn8 = assign29310_e22907_d_n8;
        locals.var_t2__blk1146_dn9 = assign29310_e22907_d_n9;
        locals.var_t2__blk1146_dn10 = assign29310_e22907_d_n10;
        locals.var_t2__blk1146_dn11 = assign29310_e22907_d_n11;
        locals.var_t2__blk1146_dn12 = assign29310_e22907_d_n12;

        let (assign29320_e22923, assign29320_e22923_d_n3, assign29320_e22923_d_n4, assign29320_e22923_d_n5, assign29320_e22923_d_n6, assign29320_e22923_d_n7, assign29320_e22923_d_n8, assign29320_e22923_d_n9, assign29320_e22923_d_n10, assign29320_e22923_d_n11, assign29320_e22923_d_n12,) = {
    if ((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1693 == 0.0)) && (locals.var_guard1694 == 0.0)) {
        let assign29320_e22921: f64 = (locals.var_t1__blk1145 / locals.var_t2__blk1146);
        (assign29320_e22921, (((locals.var_t1__blk1145_dn3 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn3)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn4 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn4)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn5 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn5)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn6 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn6)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn7 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn7)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn8 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn8)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn9 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn9)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn10 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn10)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn11 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn11)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn12 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn12)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)),)
    } else {
        (locals.var_vgsteff__blk1175, locals.var_vgsteff__blk1175_dn3, locals.var_vgsteff__blk1175_dn4, locals.var_vgsteff__blk1175_dn5, locals.var_vgsteff__blk1175_dn6, locals.var_vgsteff__blk1175_dn7, locals.var_vgsteff__blk1175_dn8, locals.var_vgsteff__blk1175_dn9, locals.var_vgsteff__blk1175_dn10, locals.var_vgsteff__blk1175_dn11, locals.var_vgsteff__blk1175_dn12,)
    }
};
        locals.var_vgsteff__blk1175 = assign29320_e22923;
        locals.var_vgsteff__blk1175_dn3 = assign29320_e22923_d_n3;
        locals.var_vgsteff__blk1175_dn4 = assign29320_e22923_d_n4;
        locals.var_vgsteff__blk1175_dn5 = assign29320_e22923_d_n5;
        locals.var_vgsteff__blk1175_dn6 = assign29320_e22923_d_n6;
        locals.var_vgsteff__blk1175_dn7 = assign29320_e22923_d_n7;
        locals.var_vgsteff__blk1175_dn8 = assign29320_e22923_d_n8;
        locals.var_vgsteff__blk1175_dn9 = assign29320_e22923_d_n9;
        locals.var_vgsteff__blk1175_dn10 = assign29320_e22923_d_n10;
        locals.var_vgsteff__blk1175_dn11 = assign29320_e22923_d_n11;
        locals.var_vgsteff__blk1175_dn12 = assign29320_e22923_d_n12;

        let assign29330_e22926: f64 = if locals.var_b4soiagbcp2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1695 = assign29330_e22926;

        let (assign29340_e22944, assign29340_e22944_d_n3, assign29340_e22944_d_n4, assign29340_e22944_d_n5, assign29340_e22944_d_n6, assign29340_e22944_d_n7, assign29340_e22944_d_n8, assign29340_e22944_d_n9, assign29340_e22944_d_n10, assign29340_e22944_d_n11, assign29340_e22944_d_n12,) = {
    if (((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1695 != 0.0)) {
        let assign29340_e22937: f64 = (locals.var_vgst__blk1131 - locals.var_pparam_b4soidelvt);
        let assign29340_e22939: f64 = (assign29340_e22937 - locals.var_eggbcp2);
        let assign29340_e22940: f64 = (locals.var_pparam_b4soimstarcv * assign29340_e22939);
        let assign29340_e22942: f64 = (assign29340_e22940 / locals.var_noff2);
        (assign29340_e22942, (((((locals.var_pparam_b4soimstarcv_dn3 * assign29340_e22939) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn3 - locals.var_pparam_b4soidelvt_dn3))) * locals.var_noff2) - (assign29340_e22940 * locals.var_noff2_dn3)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn4 * assign29340_e22939) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn4 - locals.var_pparam_b4soidelvt_dn4))) * locals.var_noff2) - (assign29340_e22940 * locals.var_noff2_dn4)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn5 * assign29340_e22939) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn5 - locals.var_pparam_b4soidelvt_dn5))) * locals.var_noff2) - (assign29340_e22940 * locals.var_noff2_dn5)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn6 * assign29340_e22939) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn6 - locals.var_pparam_b4soidelvt_dn6))) * locals.var_noff2) - (assign29340_e22940 * locals.var_noff2_dn6)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn7 * assign29340_e22939) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn7 - locals.var_pparam_b4soidelvt_dn7))) * locals.var_noff2) - (assign29340_e22940 * locals.var_noff2_dn7)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn8 * assign29340_e22939) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn8 - locals.var_pparam_b4soidelvt_dn8))) * locals.var_noff2) - (assign29340_e22940 * locals.var_noff2_dn8)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn9 * assign29340_e22939) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn9 - locals.var_pparam_b4soidelvt_dn9))) * locals.var_noff2) - (assign29340_e22940 * locals.var_noff2_dn9)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn10 * assign29340_e22939) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn10 - locals.var_pparam_b4soidelvt_dn10))) * locals.var_noff2) - (assign29340_e22940 * locals.var_noff2_dn10)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn11 * assign29340_e22939) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn11 - locals.var_pparam_b4soidelvt_dn11))) * locals.var_noff2) - (assign29340_e22940 * locals.var_noff2_dn11)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn12 * assign29340_e22939) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk1131_dn12 - locals.var_pparam_b4soidelvt_dn12))) * locals.var_noff2) - (assign29340_e22940 * locals.var_noff2_dn12)) / (locals.var_noff2 * locals.var_noff2)),)
    } else {
        (locals.var_vgstnvt2, locals.var_vgstnvt2_dn3, locals.var_vgstnvt2_dn4, locals.var_vgstnvt2_dn5, locals.var_vgstnvt2_dn6, locals.var_vgstnvt2_dn7, locals.var_vgstnvt2_dn8, locals.var_vgstnvt2_dn9, locals.var_vgstnvt2_dn10, locals.var_vgstnvt2_dn11, locals.var_vgstnvt2_dn12,)
    }
};
        locals.var_vgstnvt2 = assign29340_e22944;
        locals.var_vgstnvt2_dn3 = assign29340_e22944_d_n3;
        locals.var_vgstnvt2_dn4 = assign29340_e22944_d_n4;
        locals.var_vgstnvt2_dn5 = assign29340_e22944_d_n5;
        locals.var_vgstnvt2_dn6 = assign29340_e22944_d_n6;
        locals.var_vgstnvt2_dn7 = assign29340_e22944_d_n7;
        locals.var_vgstnvt2_dn8 = assign29340_e22944_d_n8;
        locals.var_vgstnvt2_dn9 = assign29340_e22944_d_n9;
        locals.var_vgstnvt2_dn10 = assign29340_e22944_d_n10;
        locals.var_vgstnvt2_dn11 = assign29340_e22944_d_n11;
        locals.var_vgstnvt2_dn12 = assign29340_e22944_d_n12;

        let (assign29350_e22966, assign29350_e22966_d_n3, assign29350_e22966_d_n4, assign29350_e22966_d_n5, assign29350_e22966_d_n6, assign29350_e22966_d_n7, assign29350_e22966_d_n8, assign29350_e22966_d_n9, assign29350_e22966_d_n10, assign29350_e22966_d_n11, assign29350_e22966_d_n12,) = {
    if (((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1695 != 0.0)) {
        let assign29350_e22955: f64 = (1.0 - locals.var_pparam_b4soimstarcv);
        let assign29350_e22958: f64 = (locals.var_vgst__blk1131 - locals.var_pparam_b4soidelvt);
        let assign29350_e22960: f64 = (assign29350_e22958 - locals.var_eggbcp2);
        let assign29350_e22961: f64 = (assign29350_e22955 * assign29350_e22960);
        let assign29350_e22962: f64 = (locals.var_pparam_b4soivoffcv - assign29350_e22961);
        let assign29350_e22964: f64 = (assign29350_e22962 / locals.var_noff2);
        (assign29350_e22964, ((((locals.var_pparam_b4soivoffcv_dn3 - (((-locals.var_pparam_b4soimstarcv_dn3) * assign29350_e22960) + (assign29350_e22955 * (locals.var_vgst__blk1131_dn3 - locals.var_pparam_b4soidelvt_dn3)))) * locals.var_noff2) - (assign29350_e22962 * locals.var_noff2_dn3)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn4 - (((-locals.var_pparam_b4soimstarcv_dn4) * assign29350_e22960) + (assign29350_e22955 * (locals.var_vgst__blk1131_dn4 - locals.var_pparam_b4soidelvt_dn4)))) * locals.var_noff2) - (assign29350_e22962 * locals.var_noff2_dn4)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn5 - (((-locals.var_pparam_b4soimstarcv_dn5) * assign29350_e22960) + (assign29350_e22955 * (locals.var_vgst__blk1131_dn5 - locals.var_pparam_b4soidelvt_dn5)))) * locals.var_noff2) - (assign29350_e22962 * locals.var_noff2_dn5)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn6 - (((-locals.var_pparam_b4soimstarcv_dn6) * assign29350_e22960) + (assign29350_e22955 * (locals.var_vgst__blk1131_dn6 - locals.var_pparam_b4soidelvt_dn6)))) * locals.var_noff2) - (assign29350_e22962 * locals.var_noff2_dn6)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn7 - (((-locals.var_pparam_b4soimstarcv_dn7) * assign29350_e22960) + (assign29350_e22955 * (locals.var_vgst__blk1131_dn7 - locals.var_pparam_b4soidelvt_dn7)))) * locals.var_noff2) - (assign29350_e22962 * locals.var_noff2_dn7)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn8 - (((-locals.var_pparam_b4soimstarcv_dn8) * assign29350_e22960) + (assign29350_e22955 * (locals.var_vgst__blk1131_dn8 - locals.var_pparam_b4soidelvt_dn8)))) * locals.var_noff2) - (assign29350_e22962 * locals.var_noff2_dn8)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn9 - (((-locals.var_pparam_b4soimstarcv_dn9) * assign29350_e22960) + (assign29350_e22955 * (locals.var_vgst__blk1131_dn9 - locals.var_pparam_b4soidelvt_dn9)))) * locals.var_noff2) - (assign29350_e22962 * locals.var_noff2_dn9)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn10 - (((-locals.var_pparam_b4soimstarcv_dn10) * assign29350_e22960) + (assign29350_e22955 * (locals.var_vgst__blk1131_dn10 - locals.var_pparam_b4soidelvt_dn10)))) * locals.var_noff2) - (assign29350_e22962 * locals.var_noff2_dn10)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn11 - (((-locals.var_pparam_b4soimstarcv_dn11) * assign29350_e22960) + (assign29350_e22955 * (locals.var_vgst__blk1131_dn11 - locals.var_pparam_b4soidelvt_dn11)))) * locals.var_noff2) - (assign29350_e22962 * locals.var_noff2_dn11)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn12 - (((-locals.var_pparam_b4soimstarcv_dn12) * assign29350_e22960) + (assign29350_e22955 * (locals.var_vgst__blk1131_dn12 - locals.var_pparam_b4soidelvt_dn12)))) * locals.var_noff2) - (assign29350_e22962 * locals.var_noff2_dn12)) / (locals.var_noff2 * locals.var_noff2)),)
    } else {
        (locals.var_exparg2, locals.var_exparg2_dn3, locals.var_exparg2_dn4, locals.var_exparg2_dn5, locals.var_exparg2_dn6, locals.var_exparg2_dn7, locals.var_exparg2_dn8, locals.var_exparg2_dn9, locals.var_exparg2_dn10, locals.var_exparg2_dn11, locals.var_exparg2_dn12,)
    }
};
        locals.var_exparg2 = assign29350_e22966;
        locals.var_exparg2_dn3 = assign29350_e22966_d_n3;
        locals.var_exparg2_dn4 = assign29350_e22966_d_n4;
        locals.var_exparg2_dn5 = assign29350_e22966_d_n5;
        locals.var_exparg2_dn6 = assign29350_e22966_d_n6;
        locals.var_exparg2_dn7 = assign29350_e22966_d_n7;
        locals.var_exparg2_dn8 = assign29350_e22966_d_n8;
        locals.var_exparg2_dn9 = assign29350_e22966_d_n9;
        locals.var_exparg2_dn10 = assign29350_e22966_d_n10;
        locals.var_exparg2_dn11 = assign29350_e22966_d_n11;
        locals.var_exparg2_dn12 = assign29350_e22966_d_n12;

        let assign29360_e22969: f64 = if locals.var_vgstnvt2 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1696 = assign29360_e22969;

        let (assign29370_e22985, assign29370_e22985_d_n3, assign29370_e22985_d_n4, assign29370_e22985_d_n5, assign29370_e22985_d_n6, assign29370_e22985_d_n7, assign29370_e22985_d_n8, assign29370_e22985_d_n9, assign29370_e22985_d_n10, assign29370_e22985_d_n11, assign29370_e22985_d_n12,) = {
    if ((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1695 != 0.0)) && (locals.var_guard1696 != 0.0)) {
        let assign29370_e22981: f64 = (locals.var_vgst__blk1131 - locals.var_pparam_b4soidelvt);
        let assign29370_e22983: f64 = (assign29370_e22981 - locals.var_eggbcp2);
        (assign29370_e22983, (locals.var_vgst__blk1131_dn3 - locals.var_pparam_b4soidelvt_dn3), (locals.var_vgst__blk1131_dn4 - locals.var_pparam_b4soidelvt_dn4), (locals.var_vgst__blk1131_dn5 - locals.var_pparam_b4soidelvt_dn5), (locals.var_vgst__blk1131_dn6 - locals.var_pparam_b4soidelvt_dn6), (locals.var_vgst__blk1131_dn7 - locals.var_pparam_b4soidelvt_dn7), (locals.var_vgst__blk1131_dn8 - locals.var_pparam_b4soidelvt_dn8), (locals.var_vgst__blk1131_dn9 - locals.var_pparam_b4soidelvt_dn9), (locals.var_vgst__blk1131_dn10 - locals.var_pparam_b4soidelvt_dn10), (locals.var_vgst__blk1131_dn11 - locals.var_pparam_b4soidelvt_dn11), (locals.var_vgst__blk1131_dn12 - locals.var_pparam_b4soidelvt_dn12),)
    } else {
        (locals.var_vgsteff2, locals.var_vgsteff2_dn3, locals.var_vgsteff2_dn4, locals.var_vgsteff2_dn5, locals.var_vgsteff2_dn6, locals.var_vgsteff2_dn7, locals.var_vgsteff2_dn8, locals.var_vgsteff2_dn9, locals.var_vgsteff2_dn10, locals.var_vgsteff2_dn11, locals.var_vgsteff2_dn12,)
    }
};
        locals.var_vgsteff2 = assign29370_e22985;
        locals.var_vgsteff2_dn3 = assign29370_e22985_d_n3;
        locals.var_vgsteff2_dn4 = assign29370_e22985_d_n4;
        locals.var_vgsteff2_dn5 = assign29370_e22985_d_n5;
        locals.var_vgsteff2_dn6 = assign29370_e22985_d_n6;
        locals.var_vgsteff2_dn7 = assign29370_e22985_d_n7;
        locals.var_vgsteff2_dn8 = assign29370_e22985_d_n8;
        locals.var_vgsteff2_dn9 = assign29370_e22985_d_n9;
        locals.var_vgsteff2_dn10 = assign29370_e22985_d_n10;
        locals.var_vgsteff2_dn11 = assign29370_e22985_d_n11;
        locals.var_vgsteff2_dn12 = assign29370_e22985_d_n12;

        let assign29380_e22988: f64 = if locals.var_exparg2 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1697 = assign29380_e22988;

        let (assign29390_e23011, assign29390_e23011_d_n3, assign29390_e23011_d_n4, assign29390_e23011_d_n5, assign29390_e23011_d_n6, assign29390_e23011_d_n7, assign29390_e23011_d_n8, assign29390_e23011_d_n9, assign29390_e23011_d_n10, assign29390_e23011_d_n11, assign29390_e23011_d_n12,) = {
    if (((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1695 != 0.0)) && (locals.var_guard1696 == 0.0)) && (locals.var_guard1697 != 0.0)) {
        let assign29390_e23003: f64 = (locals.var_vgst__blk1131 - locals.var_pparam_b4soidelvt);
        let assign29390_e23005: f64 = (assign29390_e23003 - locals.var_pparam_b4soivoffcv);
        let assign29390_e23007: f64 = (assign29390_e23005 - locals.var_eggbcp2);
        let assign29390_e23009: f64 = (assign29390_e23007 / locals.var_noff2);
        (assign29390_e23009, (((((locals.var_vgst__blk1131_dn3 - locals.var_pparam_b4soidelvt_dn3) - locals.var_pparam_b4soivoffcv_dn3) * locals.var_noff2) - (assign29390_e23007 * locals.var_noff2_dn3)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk1131_dn4 - locals.var_pparam_b4soidelvt_dn4) - locals.var_pparam_b4soivoffcv_dn4) * locals.var_noff2) - (assign29390_e23007 * locals.var_noff2_dn4)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk1131_dn5 - locals.var_pparam_b4soidelvt_dn5) - locals.var_pparam_b4soivoffcv_dn5) * locals.var_noff2) - (assign29390_e23007 * locals.var_noff2_dn5)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk1131_dn6 - locals.var_pparam_b4soidelvt_dn6) - locals.var_pparam_b4soivoffcv_dn6) * locals.var_noff2) - (assign29390_e23007 * locals.var_noff2_dn6)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk1131_dn7 - locals.var_pparam_b4soidelvt_dn7) - locals.var_pparam_b4soivoffcv_dn7) * locals.var_noff2) - (assign29390_e23007 * locals.var_noff2_dn7)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk1131_dn8 - locals.var_pparam_b4soidelvt_dn8) - locals.var_pparam_b4soivoffcv_dn8) * locals.var_noff2) - (assign29390_e23007 * locals.var_noff2_dn8)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk1131_dn9 - locals.var_pparam_b4soidelvt_dn9) - locals.var_pparam_b4soivoffcv_dn9) * locals.var_noff2) - (assign29390_e23007 * locals.var_noff2_dn9)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk1131_dn10 - locals.var_pparam_b4soidelvt_dn10) - locals.var_pparam_b4soivoffcv_dn10) * locals.var_noff2) - (assign29390_e23007 * locals.var_noff2_dn10)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk1131_dn11 - locals.var_pparam_b4soidelvt_dn11) - locals.var_pparam_b4soivoffcv_dn11) * locals.var_noff2) - (assign29390_e23007 * locals.var_noff2_dn11)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk1131_dn12 - locals.var_pparam_b4soidelvt_dn12) - locals.var_pparam_b4soivoffcv_dn12) * locals.var_noff2) - (assign29390_e23007 * locals.var_noff2_dn12)) / (locals.var_noff2 * locals.var_noff2)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign29390_e23011;
        locals.var_t0__blk1144_dn3 = assign29390_e23011_d_n3;
        locals.var_t0__blk1144_dn4 = assign29390_e23011_d_n4;
        locals.var_t0__blk1144_dn5 = assign29390_e23011_d_n5;
        locals.var_t0__blk1144_dn6 = assign29390_e23011_d_n6;
        locals.var_t0__blk1144_dn7 = assign29390_e23011_d_n7;
        locals.var_t0__blk1144_dn8 = assign29390_e23011_d_n8;
        locals.var_t0__blk1144_dn9 = assign29390_e23011_d_n9;
        locals.var_t0__blk1144_dn10 = assign29390_e23011_d_n10;
        locals.var_t0__blk1144_dn11 = assign29390_e23011_d_n11;
        locals.var_t0__blk1144_dn12 = assign29390_e23011_d_n12;

        let (assign29400_e23027, assign29400_e23027_d_n3, assign29400_e23027_d_n4, assign29400_e23027_d_n5, assign29400_e23027_d_n6, assign29400_e23027_d_n7, assign29400_e23027_d_n8, assign29400_e23027_d_n9, assign29400_e23027_d_n10, assign29400_e23027_d_n11, assign29400_e23027_d_n12,) = {
    if (((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1695 != 0.0)) && (locals.var_guard1696 == 0.0)) && (locals.var_guard1697 != 0.0)) {
        let assign29400_e23025: f64 = (locals.var_t0__blk1144).exp();
        (assign29400_e23025, (assign29400_e23025 * locals.var_t0__blk1144_dn3), (assign29400_e23025 * locals.var_t0__blk1144_dn4), (assign29400_e23025 * locals.var_t0__blk1144_dn5), (assign29400_e23025 * locals.var_t0__blk1144_dn6), (assign29400_e23025 * locals.var_t0__blk1144_dn7), (assign29400_e23025 * locals.var_t0__blk1144_dn8), (assign29400_e23025 * locals.var_t0__blk1144_dn9), (assign29400_e23025 * locals.var_t0__blk1144_dn10), (assign29400_e23025 * locals.var_t0__blk1144_dn11), (assign29400_e23025 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_expvgst2, locals.var_expvgst2_dn3, locals.var_expvgst2_dn4, locals.var_expvgst2_dn5, locals.var_expvgst2_dn6, locals.var_expvgst2_dn7, locals.var_expvgst2_dn8, locals.var_expvgst2_dn9, locals.var_expvgst2_dn10, locals.var_expvgst2_dn11, locals.var_expvgst2_dn12,)
    }
};
        locals.var_expvgst2 = assign29400_e23027;
        locals.var_expvgst2_dn3 = assign29400_e23027_d_n3;
        locals.var_expvgst2_dn4 = assign29400_e23027_d_n4;
        locals.var_expvgst2_dn5 = assign29400_e23027_d_n5;
        locals.var_expvgst2_dn6 = assign29400_e23027_d_n6;
        locals.var_expvgst2_dn7 = assign29400_e23027_d_n7;
        locals.var_expvgst2_dn8 = assign29400_e23027_d_n8;
        locals.var_expvgst2_dn9 = assign29400_e23027_d_n9;
        locals.var_expvgst2_dn10 = assign29400_e23027_d_n10;
        locals.var_expvgst2_dn11 = assign29400_e23027_d_n11;
        locals.var_expvgst2_dn12 = assign29400_e23027_d_n12;

        let (assign29410_e23048, assign29410_e23048_d_n3, assign29410_e23048_d_n4, assign29410_e23048_d_n5, assign29410_e23048_d_n6, assign29410_e23048_d_n7, assign29410_e23048_d_n8, assign29410_e23048_d_n9, assign29410_e23048_d_n10, assign29410_e23048_d_n11, assign29410_e23048_d_n12,) = {
    if (((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1695 != 0.0)) && (locals.var_guard1696 == 0.0)) && (locals.var_guard1697 != 0.0)) {
        let assign29410_e23042: f64 = (locals.var_vtm * locals.var_cdep0);
        let assign29410_e23044: f64 = (assign29410_e23042 / locals.var_b4soicox);
        let assign29410_e23046: f64 = (assign29410_e23044 * locals.var_expvgst2);
        (assign29410_e23046, ((((locals.var_vtm * locals.var_cdep0_dn3) / locals.var_b4soicox) * locals.var_expvgst2) + (assign29410_e23044 * locals.var_expvgst2_dn3)), ((((locals.var_vtm * locals.var_cdep0_dn4) / locals.var_b4soicox) * locals.var_expvgst2) + (assign29410_e23044 * locals.var_expvgst2_dn4)), ((((locals.var_vtm * locals.var_cdep0_dn5) / locals.var_b4soicox) * locals.var_expvgst2) + (assign29410_e23044 * locals.var_expvgst2_dn5)), (((((locals.var_vtm_dn6 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn6)) / locals.var_b4soicox) * locals.var_expvgst2) + (assign29410_e23044 * locals.var_expvgst2_dn6)), ((((locals.var_vtm * locals.var_cdep0_dn7) / locals.var_b4soicox) * locals.var_expvgst2) + (assign29410_e23044 * locals.var_expvgst2_dn7)), ((((locals.var_vtm * locals.var_cdep0_dn8) / locals.var_b4soicox) * locals.var_expvgst2) + (assign29410_e23044 * locals.var_expvgst2_dn8)), ((((locals.var_vtm * locals.var_cdep0_dn9) / locals.var_b4soicox) * locals.var_expvgst2) + (assign29410_e23044 * locals.var_expvgst2_dn9)), ((((locals.var_vtm * locals.var_cdep0_dn10) / locals.var_b4soicox) * locals.var_expvgst2) + (assign29410_e23044 * locals.var_expvgst2_dn10)), ((((locals.var_vtm * locals.var_cdep0_dn11) / locals.var_b4soicox) * locals.var_expvgst2) + (assign29410_e23044 * locals.var_expvgst2_dn11)), ((((locals.var_vtm * locals.var_cdep0_dn12) / locals.var_b4soicox) * locals.var_expvgst2) + (assign29410_e23044 * locals.var_expvgst2_dn12)),)
    } else {
        (locals.var_vgsteff2, locals.var_vgsteff2_dn3, locals.var_vgsteff2_dn4, locals.var_vgsteff2_dn5, locals.var_vgsteff2_dn6, locals.var_vgsteff2_dn7, locals.var_vgsteff2_dn8, locals.var_vgsteff2_dn9, locals.var_vgsteff2_dn10, locals.var_vgsteff2_dn11, locals.var_vgsteff2_dn12,)
    }
};
        locals.var_vgsteff2 = assign29410_e23048;
        locals.var_vgsteff2_dn3 = assign29410_e23048_d_n3;
        locals.var_vgsteff2_dn4 = assign29410_e23048_d_n4;
        locals.var_vgsteff2_dn5 = assign29410_e23048_d_n5;
        locals.var_vgsteff2_dn6 = assign29410_e23048_d_n6;
        locals.var_vgsteff2_dn7 = assign29410_e23048_d_n7;
        locals.var_vgsteff2_dn8 = assign29410_e23048_d_n8;
        locals.var_vgsteff2_dn9 = assign29410_e23048_d_n9;
        locals.var_vgsteff2_dn10 = assign29410_e23048_d_n10;
        locals.var_vgsteff2_dn11 = assign29410_e23048_d_n11;
        locals.var_vgsteff2_dn12 = assign29410_e23048_d_n12;

        let (assign29420_e23065, assign29420_e23065_d_n3, assign29420_e23065_d_n4, assign29420_e23065_d_n5, assign29420_e23065_d_n6, assign29420_e23065_d_n7, assign29420_e23065_d_n8, assign29420_e23065_d_n9, assign29420_e23065_d_n10, assign29420_e23065_d_n11, assign29420_e23065_d_n12,) = {
    if (((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1695 != 0.0)) && (locals.var_guard1696 == 0.0)) && (locals.var_guard1697 == 0.0)) {
        let assign29420_e23063: f64 = (locals.var_vgstnvt2).exp();
        (assign29420_e23063, (assign29420_e23063 * locals.var_vgstnvt2_dn3), (assign29420_e23063 * locals.var_vgstnvt2_dn4), (assign29420_e23063 * locals.var_vgstnvt2_dn5), (assign29420_e23063 * locals.var_vgstnvt2_dn6), (assign29420_e23063 * locals.var_vgstnvt2_dn7), (assign29420_e23063 * locals.var_vgstnvt2_dn8), (assign29420_e23063 * locals.var_vgstnvt2_dn9), (assign29420_e23063 * locals.var_vgstnvt2_dn10), (assign29420_e23063 * locals.var_vgstnvt2_dn11), (assign29420_e23063 * locals.var_vgstnvt2_dn12),)
    } else {
        (locals.var_expvgst2, locals.var_expvgst2_dn3, locals.var_expvgst2_dn4, locals.var_expvgst2_dn5, locals.var_expvgst2_dn6, locals.var_expvgst2_dn7, locals.var_expvgst2_dn8, locals.var_expvgst2_dn9, locals.var_expvgst2_dn10, locals.var_expvgst2_dn11, locals.var_expvgst2_dn12,)
    }
};
        locals.var_expvgst2 = assign29420_e23065;
        locals.var_expvgst2_dn3 = assign29420_e23065_d_n3;
        locals.var_expvgst2_dn4 = assign29420_e23065_d_n4;
        locals.var_expvgst2_dn5 = assign29420_e23065_d_n5;
        locals.var_expvgst2_dn6 = assign29420_e23065_d_n6;
        locals.var_expvgst2_dn7 = assign29420_e23065_d_n7;
        locals.var_expvgst2_dn8 = assign29420_e23065_d_n8;
        locals.var_expvgst2_dn9 = assign29420_e23065_d_n9;
        locals.var_expvgst2_dn10 = assign29420_e23065_d_n10;
        locals.var_expvgst2_dn11 = assign29420_e23065_d_n11;
        locals.var_expvgst2_dn12 = assign29420_e23065_d_n12;

        let (assign29430_e23094, assign29430_e23094_d_n3, assign29430_e23094_d_n4, assign29430_e23094_d_n5, assign29430_e23094_d_n6, assign29430_e23094_d_n7, assign29430_e23094_d_n8, assign29430_e23094_d_n9, assign29430_e23094_d_n10, assign29430_e23094_d_n11, assign29430_e23094_d_n12,) = {
    if (((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1695 != 0.0)) && (locals.var_guard1696 == 0.0)) && (locals.var_guard1697 == 0.0)) {
        let assign29430_e23082: f64 = (1.0 + locals.var_expvgst2);
        let (assign29430_e23091, assign29430_e23091_d_n3, assign29430_e23091_d_n4, assign29430_e23091_d_n5, assign29430_e23091_d_n6, assign29430_e23091_d_n7, assign29430_e23091_d_n8, assign29430_e23091_d_n9, assign29430_e23091_d_n10, assign29430_e23091_d_n11, assign29430_e23091_d_n12,) = {
            if (assign29430_e23082 > 1e-38) {
                let assign29430_e23087: f64 = (1.0 + locals.var_expvgst2);
                let assign29430_e23088: f64 = (assign29430_e23087).ln();
                (assign29430_e23088, (locals.var_expvgst2_dn3 / assign29430_e23087), (locals.var_expvgst2_dn4 / assign29430_e23087), (locals.var_expvgst2_dn5 / assign29430_e23087), (locals.var_expvgst2_dn6 / assign29430_e23087), (locals.var_expvgst2_dn7 / assign29430_e23087), (locals.var_expvgst2_dn8 / assign29430_e23087), (locals.var_expvgst2_dn9 / assign29430_e23087), (locals.var_expvgst2_dn10 / assign29430_e23087), (locals.var_expvgst2_dn11 / assign29430_e23087), (locals.var_expvgst2_dn12 / assign29430_e23087),)
            } else {
                let assign29430_e23090: f64 = (-87.49823353377374);
                (assign29430_e23090, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign29430_e23092: f64 = (locals.var_noff2 * assign29430_e23091);
        (assign29430_e23092, ((locals.var_noff2_dn3 * assign29430_e23091) + (locals.var_noff2 * assign29430_e23091_d_n3)), ((locals.var_noff2_dn4 * assign29430_e23091) + (locals.var_noff2 * assign29430_e23091_d_n4)), ((locals.var_noff2_dn5 * assign29430_e23091) + (locals.var_noff2 * assign29430_e23091_d_n5)), ((locals.var_noff2_dn6 * assign29430_e23091) + (locals.var_noff2 * assign29430_e23091_d_n6)), ((locals.var_noff2_dn7 * assign29430_e23091) + (locals.var_noff2 * assign29430_e23091_d_n7)), ((locals.var_noff2_dn8 * assign29430_e23091) + (locals.var_noff2 * assign29430_e23091_d_n8)), ((locals.var_noff2_dn9 * assign29430_e23091) + (locals.var_noff2 * assign29430_e23091_d_n9)), ((locals.var_noff2_dn10 * assign29430_e23091) + (locals.var_noff2 * assign29430_e23091_d_n10)), ((locals.var_noff2_dn11 * assign29430_e23091) + (locals.var_noff2 * assign29430_e23091_d_n11)), ((locals.var_noff2_dn12 * assign29430_e23091) + (locals.var_noff2 * assign29430_e23091_d_n12)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign29430_e23094;
        locals.var_t1__blk1145_dn3 = assign29430_e23094_d_n3;
        locals.var_t1__blk1145_dn4 = assign29430_e23094_d_n4;
        locals.var_t1__blk1145_dn5 = assign29430_e23094_d_n5;
        locals.var_t1__blk1145_dn6 = assign29430_e23094_d_n6;
        locals.var_t1__blk1145_dn7 = assign29430_e23094_d_n7;
        locals.var_t1__blk1145_dn8 = assign29430_e23094_d_n8;
        locals.var_t1__blk1145_dn9 = assign29430_e23094_d_n9;
        locals.var_t1__blk1145_dn10 = assign29430_e23094_d_n10;
        locals.var_t1__blk1145_dn11 = assign29430_e23094_d_n11;
        locals.var_t1__blk1145_dn12 = assign29430_e23094_d_n12;

        let (assign29440_e23122, assign29440_e23122_d_n3, assign29440_e23122_d_n4, assign29440_e23122_d_n5, assign29440_e23122_d_n6, assign29440_e23122_d_n7, assign29440_e23122_d_n8, assign29440_e23122_d_n9, assign29440_e23122_d_n10, assign29440_e23122_d_n11, assign29440_e23122_d_n12,) = {
    if (((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1695 != 0.0)) && (locals.var_guard1696 == 0.0)) && (locals.var_guard1697 == 0.0)) {
        let assign29440_e23109: f64 = (-locals.var_b4soicox);
        let assign29440_e23112: f64 = (locals.var_vtm * locals.var_cdep0);
        let assign29440_e23113: f64 = (assign29440_e23109 / assign29440_e23112);
        let assign29440_e23115: f64 = (locals.var_exparg2).exp();
        let assign29440_e23116: f64 = (assign29440_e23113 * assign29440_e23115);
        let assign29440_e23119: f64 = (1.0 - locals.var_pparam_b4soimstarcv);
        let assign29440_e23120: f64 = (assign29440_e23116 * assign29440_e23119);
        (assign29440_e23120, (((((-((assign29440_e23109 * (locals.var_vtm * locals.var_cdep0_dn3)) / (assign29440_e23112 * assign29440_e23112))) * assign29440_e23115) + (assign29440_e23113 * (assign29440_e23115 * locals.var_exparg2_dn3))) * assign29440_e23119) + (assign29440_e23116 * (-locals.var_pparam_b4soimstarcv_dn3))), (((((-((assign29440_e23109 * (locals.var_vtm * locals.var_cdep0_dn4)) / (assign29440_e23112 * assign29440_e23112))) * assign29440_e23115) + (assign29440_e23113 * (assign29440_e23115 * locals.var_exparg2_dn4))) * assign29440_e23119) + (assign29440_e23116 * (-locals.var_pparam_b4soimstarcv_dn4))), (((((-((assign29440_e23109 * (locals.var_vtm * locals.var_cdep0_dn5)) / (assign29440_e23112 * assign29440_e23112))) * assign29440_e23115) + (assign29440_e23113 * (assign29440_e23115 * locals.var_exparg2_dn5))) * assign29440_e23119) + (assign29440_e23116 * (-locals.var_pparam_b4soimstarcv_dn5))), (((((-((assign29440_e23109 * ((locals.var_vtm_dn6 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn6))) / (assign29440_e23112 * assign29440_e23112))) * assign29440_e23115) + (assign29440_e23113 * (assign29440_e23115 * locals.var_exparg2_dn6))) * assign29440_e23119) + (assign29440_e23116 * (-locals.var_pparam_b4soimstarcv_dn6))), (((((-((assign29440_e23109 * (locals.var_vtm * locals.var_cdep0_dn7)) / (assign29440_e23112 * assign29440_e23112))) * assign29440_e23115) + (assign29440_e23113 * (assign29440_e23115 * locals.var_exparg2_dn7))) * assign29440_e23119) + (assign29440_e23116 * (-locals.var_pparam_b4soimstarcv_dn7))), (((((-((assign29440_e23109 * (locals.var_vtm * locals.var_cdep0_dn8)) / (assign29440_e23112 * assign29440_e23112))) * assign29440_e23115) + (assign29440_e23113 * (assign29440_e23115 * locals.var_exparg2_dn8))) * assign29440_e23119) + (assign29440_e23116 * (-locals.var_pparam_b4soimstarcv_dn8))), (((((-((assign29440_e23109 * (locals.var_vtm * locals.var_cdep0_dn9)) / (assign29440_e23112 * assign29440_e23112))) * assign29440_e23115) + (assign29440_e23113 * (assign29440_e23115 * locals.var_exparg2_dn9))) * assign29440_e23119) + (assign29440_e23116 * (-locals.var_pparam_b4soimstarcv_dn9))), (((((-((assign29440_e23109 * (locals.var_vtm * locals.var_cdep0_dn10)) / (assign29440_e23112 * assign29440_e23112))) * assign29440_e23115) + (assign29440_e23113 * (assign29440_e23115 * locals.var_exparg2_dn10))) * assign29440_e23119) + (assign29440_e23116 * (-locals.var_pparam_b4soimstarcv_dn10))), (((((-((assign29440_e23109 * (locals.var_vtm * locals.var_cdep0_dn11)) / (assign29440_e23112 * assign29440_e23112))) * assign29440_e23115) + (assign29440_e23113 * (assign29440_e23115 * locals.var_exparg2_dn11))) * assign29440_e23119) + (assign29440_e23116 * (-locals.var_pparam_b4soimstarcv_dn11))), (((((-((assign29440_e23109 * (locals.var_vtm * locals.var_cdep0_dn12)) / (assign29440_e23112 * assign29440_e23112))) * assign29440_e23115) + (assign29440_e23113 * (assign29440_e23115 * locals.var_exparg2_dn12))) * assign29440_e23119) + (assign29440_e23116 * (-locals.var_pparam_b4soimstarcv_dn12))),)
    } else {
        (locals.var_dt2_dvg, locals.var_dt2_dvg_dn3, locals.var_dt2_dvg_dn4, locals.var_dt2_dvg_dn5, locals.var_dt2_dvg_dn6, locals.var_dt2_dvg_dn7, locals.var_dt2_dvg_dn8, locals.var_dt2_dvg_dn9, locals.var_dt2_dvg_dn10, locals.var_dt2_dvg_dn11, locals.var_dt2_dvg_dn12,)
    }
};
        locals.var_dt2_dvg = assign29440_e23122;
        locals.var_dt2_dvg_dn3 = assign29440_e23122_d_n3;
        locals.var_dt2_dvg_dn4 = assign29440_e23122_d_n4;
        locals.var_dt2_dvg_dn5 = assign29440_e23122_d_n5;
        locals.var_dt2_dvg_dn6 = assign29440_e23122_d_n6;
        locals.var_dt2_dvg_dn7 = assign29440_e23122_d_n7;
        locals.var_dt2_dvg_dn8 = assign29440_e23122_d_n8;
        locals.var_dt2_dvg_dn9 = assign29440_e23122_d_n9;
        locals.var_dt2_dvg_dn10 = assign29440_e23122_d_n10;
        locals.var_dt2_dvg_dn11 = assign29440_e23122_d_n11;
        locals.var_dt2_dvg_dn12 = assign29440_e23122_d_n12;

        let (assign29450_e23146, assign29450_e23146_d_n3, assign29450_e23146_d_n4, assign29450_e23146_d_n5, assign29450_e23146_d_n6, assign29450_e23146_d_n7, assign29450_e23146_d_n8, assign29450_e23146_d_n9, assign29450_e23146_d_n10, assign29450_e23146_d_n11, assign29450_e23146_d_n12,) = {
    if (((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1695 != 0.0)) && (locals.var_guard1696 == 0.0)) && (locals.var_guard1697 == 0.0)) {
        let assign29450_e23139: f64 = (locals.var_noff2 * locals.var_dt2_dvg);
        let assign29450_e23142: f64 = (1.0 - locals.var_pparam_b4soimstarcv);
        let assign29450_e23143: f64 = (assign29450_e23139 / assign29450_e23142);
        let assign29450_e23144: f64 = (locals.var_pparam_b4soimstarcv - assign29450_e23143);
        (assign29450_e23144, (locals.var_pparam_b4soimstarcv_dn3 - (((((locals.var_noff2_dn3 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn3)) * assign29450_e23142) - (assign29450_e23139 * (-locals.var_pparam_b4soimstarcv_dn3))) / (assign29450_e23142 * assign29450_e23142))), (locals.var_pparam_b4soimstarcv_dn4 - (((((locals.var_noff2_dn4 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn4)) * assign29450_e23142) - (assign29450_e23139 * (-locals.var_pparam_b4soimstarcv_dn4))) / (assign29450_e23142 * assign29450_e23142))), (locals.var_pparam_b4soimstarcv_dn5 - (((((locals.var_noff2_dn5 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn5)) * assign29450_e23142) - (assign29450_e23139 * (-locals.var_pparam_b4soimstarcv_dn5))) / (assign29450_e23142 * assign29450_e23142))), (locals.var_pparam_b4soimstarcv_dn6 - (((((locals.var_noff2_dn6 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn6)) * assign29450_e23142) - (assign29450_e23139 * (-locals.var_pparam_b4soimstarcv_dn6))) / (assign29450_e23142 * assign29450_e23142))), (locals.var_pparam_b4soimstarcv_dn7 - (((((locals.var_noff2_dn7 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn7)) * assign29450_e23142) - (assign29450_e23139 * (-locals.var_pparam_b4soimstarcv_dn7))) / (assign29450_e23142 * assign29450_e23142))), (locals.var_pparam_b4soimstarcv_dn8 - (((((locals.var_noff2_dn8 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn8)) * assign29450_e23142) - (assign29450_e23139 * (-locals.var_pparam_b4soimstarcv_dn8))) / (assign29450_e23142 * assign29450_e23142))), (locals.var_pparam_b4soimstarcv_dn9 - (((((locals.var_noff2_dn9 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn9)) * assign29450_e23142) - (assign29450_e23139 * (-locals.var_pparam_b4soimstarcv_dn9))) / (assign29450_e23142 * assign29450_e23142))), (locals.var_pparam_b4soimstarcv_dn10 - (((((locals.var_noff2_dn10 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn10)) * assign29450_e23142) - (assign29450_e23139 * (-locals.var_pparam_b4soimstarcv_dn10))) / (assign29450_e23142 * assign29450_e23142))), (locals.var_pparam_b4soimstarcv_dn11 - (((((locals.var_noff2_dn11 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn11)) * assign29450_e23142) - (assign29450_e23139 * (-locals.var_pparam_b4soimstarcv_dn11))) / (assign29450_e23142 * assign29450_e23142))), (locals.var_pparam_b4soimstarcv_dn12 - (((((locals.var_noff2_dn12 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn12)) * assign29450_e23142) - (assign29450_e23139 * (-locals.var_pparam_b4soimstarcv_dn12))) / (assign29450_e23142 * assign29450_e23142))),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign29450_e23146;
        locals.var_t2__blk1146_dn3 = assign29450_e23146_d_n3;
        locals.var_t2__blk1146_dn4 = assign29450_e23146_d_n4;
        locals.var_t2__blk1146_dn5 = assign29450_e23146_d_n5;
        locals.var_t2__blk1146_dn6 = assign29450_e23146_d_n6;
        locals.var_t2__blk1146_dn7 = assign29450_e23146_d_n7;
        locals.var_t2__blk1146_dn8 = assign29450_e23146_d_n8;
        locals.var_t2__blk1146_dn9 = assign29450_e23146_d_n9;
        locals.var_t2__blk1146_dn10 = assign29450_e23146_d_n10;
        locals.var_t2__blk1146_dn11 = assign29450_e23146_d_n11;
        locals.var_t2__blk1146_dn12 = assign29450_e23146_d_n12;

        let (assign29460_e23164, assign29460_e23164_d_n3, assign29460_e23164_d_n4, assign29460_e23164_d_n5, assign29460_e23164_d_n6, assign29460_e23164_d_n7, assign29460_e23164_d_n8, assign29460_e23164_d_n9, assign29460_e23164_d_n10, assign29460_e23164_d_n11, assign29460_e23164_d_n12,) = {
    if (((((locals.var_guard1687 == 0.0) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1695 != 0.0)) && (locals.var_guard1696 == 0.0)) && (locals.var_guard1697 == 0.0)) {
        let assign29460_e23162: f64 = (locals.var_t1__blk1145 / locals.var_t2__blk1146);
        (assign29460_e23162, (((locals.var_t1__blk1145_dn3 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn3)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn4 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn4)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn5 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn5)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn6 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn6)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn7 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn7)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn8 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn8)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn9 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn9)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn10 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn10)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn11 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn11)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t1__blk1145_dn12 * locals.var_t2__blk1146) - (locals.var_t1__blk1145 * locals.var_t2__blk1146_dn12)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)),)
    } else {
        (locals.var_vgsteff2, locals.var_vgsteff2_dn3, locals.var_vgsteff2_dn4, locals.var_vgsteff2_dn5, locals.var_vgsteff2_dn6, locals.var_vgsteff2_dn7, locals.var_vgsteff2_dn8, locals.var_vgsteff2_dn9, locals.var_vgsteff2_dn10, locals.var_vgsteff2_dn11, locals.var_vgsteff2_dn12,)
    }
};
        locals.var_vgsteff2 = assign29460_e23164;
        locals.var_vgsteff2_dn3 = assign29460_e23164_d_n3;
        locals.var_vgsteff2_dn4 = assign29460_e23164_d_n4;
        locals.var_vgsteff2_dn5 = assign29460_e23164_d_n5;
        locals.var_vgsteff2_dn6 = assign29460_e23164_d_n6;
        locals.var_vgsteff2_dn7 = assign29460_e23164_d_n7;
        locals.var_vgsteff2_dn8 = assign29460_e23164_d_n8;
        locals.var_vgsteff2_dn9 = assign29460_e23164_d_n9;
        locals.var_vgsteff2_dn10 = assign29460_e23164_d_n10;
        locals.var_vgsteff2_dn11 = assign29460_e23164_d_n11;
        locals.var_vgsteff2_dn12 = assign29460_e23164_d_n12;

        locals.var_vth__blk1130 = locals.var_vth_cv;
        locals.var_vth__blk1130_dn3 = locals.var_vth_cv_dn3;
        locals.var_vth__blk1130_dn4 = locals.var_vth_cv_dn4;
        locals.var_vth__blk1130_dn5 = locals.var_vth_cv_dn5;
        locals.var_vth__blk1130_dn6 = locals.var_vth_cv_dn6;
        locals.var_vth__blk1130_dn7 = locals.var_vth_cv_dn7;
        locals.var_vth__blk1130_dn8 = locals.var_vth_cv_dn8;
        locals.var_vth__blk1130_dn9 = locals.var_vth_cv_dn9;
        locals.var_vth__blk1130_dn10 = locals.var_vth_cv_dn10;
        locals.var_vth__blk1130_dn11 = locals.var_vth_cv_dn11;
        locals.var_vth__blk1130_dn12 = locals.var_vth_cv_dn12;

    }

    pub(super) fn stamp_transient_block_79(
        locals: &mut StampLocals,
    ) {
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

        let assign29500_e23170: f64 = if locals.var_b4soicapmod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1698 = assign29500_e23170;

        let assign29510_e23173: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1699 = assign29510_e23173;

        let (assign29520_e23179, assign29520_e23179_d_n3, assign29520_e23179_d_n4, assign29520_e23179_d_n5, assign29520_e23179_d_n6, assign29520_e23179_d_n7, assign29520_e23179_d_n8, assign29520_e23179_d_n9, assign29520_e23179_d_n10, assign29520_e23179_d_n11, assign29520_e23179_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1699 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign29520_e23179;
        locals.var_qac0_dn3 = assign29520_e23179_d_n3;
        locals.var_qac0_dn4 = assign29520_e23179_d_n4;
        locals.var_qac0_dn5 = assign29520_e23179_d_n5;
        locals.var_qac0_dn6 = assign29520_e23179_d_n6;
        locals.var_qac0_dn7 = assign29520_e23179_d_n7;
        locals.var_qac0_dn8 = assign29520_e23179_d_n8;
        locals.var_qac0_dn9 = assign29520_e23179_d_n9;
        locals.var_qac0_dn10 = assign29520_e23179_d_n10;
        locals.var_qac0_dn11 = assign29520_e23179_d_n11;
        locals.var_qac0_dn12 = assign29520_e23179_d_n12;

        let (assign29530_e23185, assign29530_e23185_d_n3, assign29530_e23185_d_n4, assign29530_e23185_d_n5, assign29530_e23185_d_n6, assign29530_e23185_d_n7, assign29530_e23185_d_n8, assign29530_e23185_d_n9, assign29530_e23185_d_n10, assign29530_e23185_d_n11, assign29530_e23185_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1699 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign29530_e23185;
        locals.var_qsub0_dn3 = assign29530_e23185_d_n3;
        locals.var_qsub0_dn4 = assign29530_e23185_d_n4;
        locals.var_qsub0_dn5 = assign29530_e23185_d_n5;
        locals.var_qsub0_dn6 = assign29530_e23185_d_n6;
        locals.var_qsub0_dn7 = assign29530_e23185_d_n7;
        locals.var_qsub0_dn8 = assign29530_e23185_d_n8;
        locals.var_qsub0_dn9 = assign29530_e23185_d_n9;
        locals.var_qsub0_dn10 = assign29530_e23185_d_n10;
        locals.var_qsub0_dn11 = assign29530_e23185_d_n11;
        locals.var_qsub0_dn12 = assign29530_e23185_d_n12;

        let (assign29540_e23200, assign29540_e23200_d_n3, assign29540_e23200_d_n4, assign29540_e23200_d_n5, assign29540_e23200_d_n6, assign29540_e23200_d_n7, assign29540_e23200_d_n8, assign29540_e23200_d_n9, assign29540_e23200_d_n10, assign29540_e23200_d_n11, assign29540_e23200_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) {
        let assign29540_e23192: f64 = (locals.var_vth__blk1130 - locals.var_phi);
        let assign29540_e23195: f64 = (locals.var_pparam_b4soik1eff * locals.var_sqrtphis);
        let assign29540_e23196: f64 = (assign29540_e23192 - assign29540_e23195);
        let assign29540_e23198: f64 = (assign29540_e23196 + locals.var_pparam_b4soidelvt);
        (assign29540_e23198, (((locals.var_vth__blk1130_dn3 - locals.var_phi_dn3) - ((locals.var_pparam_b4soik1eff_dn3 * locals.var_sqrtphis) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphis_dn3))) + locals.var_pparam_b4soidelvt_dn3), (((locals.var_vth__blk1130_dn4 - locals.var_phi_dn4) - ((locals.var_pparam_b4soik1eff_dn4 * locals.var_sqrtphis) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphis_dn4))) + locals.var_pparam_b4soidelvt_dn4), (((locals.var_vth__blk1130_dn5 - locals.var_phi_dn5) - ((locals.var_pparam_b4soik1eff_dn5 * locals.var_sqrtphis) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphis_dn5))) + locals.var_pparam_b4soidelvt_dn5), (((locals.var_vth__blk1130_dn6 - locals.var_phi_dn6) - ((locals.var_pparam_b4soik1eff_dn6 * locals.var_sqrtphis) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphis_dn6))) + locals.var_pparam_b4soidelvt_dn6), (((locals.var_vth__blk1130_dn7 - locals.var_phi_dn7) - ((locals.var_pparam_b4soik1eff_dn7 * locals.var_sqrtphis) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphis_dn7))) + locals.var_pparam_b4soidelvt_dn7), (((locals.var_vth__blk1130_dn8 - locals.var_phi_dn8) - ((locals.var_pparam_b4soik1eff_dn8 * locals.var_sqrtphis) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphis_dn8))) + locals.var_pparam_b4soidelvt_dn8), (((locals.var_vth__blk1130_dn9 - locals.var_phi_dn9) - ((locals.var_pparam_b4soik1eff_dn9 * locals.var_sqrtphis) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphis_dn9))) + locals.var_pparam_b4soidelvt_dn9), (((locals.var_vth__blk1130_dn10 - locals.var_phi_dn10) - ((locals.var_pparam_b4soik1eff_dn10 * locals.var_sqrtphis) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphis_dn10))) + locals.var_pparam_b4soidelvt_dn10), (((locals.var_vth__blk1130_dn11 - locals.var_phi_dn11) - ((locals.var_pparam_b4soik1eff_dn11 * locals.var_sqrtphis) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphis_dn11))) + locals.var_pparam_b4soidelvt_dn11), (((locals.var_vth__blk1130_dn12 - locals.var_phi_dn12) - ((locals.var_pparam_b4soik1eff_dn12 * locals.var_sqrtphis) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphis_dn12))) + locals.var_pparam_b4soidelvt_dn12),)
    } else {
        (locals.var_vfb, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11, locals.var_vfb_dn12,)
    }
};
        locals.var_vfb = assign29540_e23200;
        locals.var_vfb_dn3 = assign29540_e23200_d_n3;
        locals.var_vfb_dn4 = assign29540_e23200_d_n4;
        locals.var_vfb_dn5 = assign29540_e23200_d_n5;
        locals.var_vfb_dn6 = assign29540_e23200_d_n6;
        locals.var_vfb_dn7 = assign29540_e23200_d_n7;
        locals.var_vfb_dn8 = assign29540_e23200_d_n8;
        locals.var_vfb_dn9 = assign29540_e23200_d_n9;
        locals.var_vfb_dn10 = assign29540_e23200_d_n10;
        locals.var_vfb_dn11 = assign29540_e23200_d_n11;
        locals.var_vfb_dn12 = assign29540_e23200_d_n12;

        let (assign29550_e23213, assign29550_e23213_d_n3, assign29550_e23213_d_n4, assign29550_e23213_d_n5, assign29550_e23213_d_n6, assign29550_e23213_d_n7, assign29550_e23213_d_n8, assign29550_e23213_d_n9, assign29550_e23213_d_n10, assign29550_e23213_d_n11, assign29550_e23213_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) {
        let assign29550_e23207: f64 = (locals.var_vfb - locals.var_vgs_eff__blk1126);
        let assign29550_e23209: f64 = (assign29550_e23207 + locals.var_vbseff);
        let assign29550_e23211: f64 = (assign29550_e23209 - 0.08);
        (assign29550_e23211, ((locals.var_vfb_dn3 - locals.var_vgs_eff__blk1126_dn3) + locals.var_vbseff_dn3), ((locals.var_vfb_dn4 - locals.var_vgs_eff__blk1126_dn4) + locals.var_vbseff_dn4), ((locals.var_vfb_dn5 - locals.var_vgs_eff__blk1126_dn5) + locals.var_vbseff_dn5), ((locals.var_vfb_dn6 - locals.var_vgs_eff__blk1126_dn6) + locals.var_vbseff_dn6), ((locals.var_vfb_dn7 - locals.var_vgs_eff__blk1126_dn7) + locals.var_vbseff_dn7), ((locals.var_vfb_dn8 - locals.var_vgs_eff__blk1126_dn8) + locals.var_vbseff_dn8), ((locals.var_vfb_dn9 - locals.var_vgs_eff__blk1126_dn9) + locals.var_vbseff_dn9), ((locals.var_vfb_dn10 - locals.var_vgs_eff__blk1126_dn10) + locals.var_vbseff_dn10), ((locals.var_vfb_dn11 - locals.var_vgs_eff__blk1126_dn11) + locals.var_vbseff_dn11), ((locals.var_vfb_dn12 - locals.var_vgs_eff__blk1126_dn12) + locals.var_vbseff_dn12),)
    } else {
        (locals.var_v3, locals.var_v3_dn3, locals.var_v3_dn4, locals.var_v3_dn5, locals.var_v3_dn6, locals.var_v3_dn7, locals.var_v3_dn8, locals.var_v3_dn9, locals.var_v3_dn10, locals.var_v3_dn11, locals.var_v3_dn12,)
    }
};
        locals.var_v3 = assign29550_e23213;
        locals.var_v3_dn3 = assign29550_e23213_d_n3;
        locals.var_v3_dn4 = assign29550_e23213_d_n4;
        locals.var_v3_dn5 = assign29550_e23213_d_n5;
        locals.var_v3_dn6 = assign29550_e23213_d_n6;
        locals.var_v3_dn7 = assign29550_e23213_d_n7;
        locals.var_v3_dn8 = assign29550_e23213_d_n8;
        locals.var_v3_dn9 = assign29550_e23213_d_n9;
        locals.var_v3_dn10 = assign29550_e23213_d_n10;
        locals.var_v3_dn11 = assign29550_e23213_d_n11;
        locals.var_v3_dn12 = assign29550_e23213_d_n12;

        let assign29560_e23216: f64 = if locals.var_vfb <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1700 = assign29560_e23216;

        let (assign29570_e23234, assign29570_e23234_d_n3, assign29570_e23234_d_n4, assign29570_e23234_d_n5, assign29570_e23234_d_n6, assign29570_e23234_d_n7, assign29570_e23234_d_n8, assign29570_e23234_d_n9, assign29570_e23234_d_n10, assign29570_e23234_d_n11, assign29570_e23234_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign29570_e23225: f64 = (locals.var_v3 * locals.var_v3);
        let assign29570_e23228: f64 = (4.0 * 0.08);
        let assign29570_e23230: f64 = (assign29570_e23228 * locals.var_vfb);
        let assign29570_e23231: f64 = (assign29570_e23225 - assign29570_e23230);
        let assign29570_e23232: f64 = (assign29570_e23231).sqrt();
        (assign29570_e23232, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) - (assign29570_e23228 * locals.var_vfb_dn3)) / (2.0 * assign29570_e23232)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) - (assign29570_e23228 * locals.var_vfb_dn4)) / (2.0 * assign29570_e23232)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) - (assign29570_e23228 * locals.var_vfb_dn5)) / (2.0 * assign29570_e23232)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) - (assign29570_e23228 * locals.var_vfb_dn6)) / (2.0 * assign29570_e23232)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) - (assign29570_e23228 * locals.var_vfb_dn7)) / (2.0 * assign29570_e23232)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) - (assign29570_e23228 * locals.var_vfb_dn8)) / (2.0 * assign29570_e23232)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) - (assign29570_e23228 * locals.var_vfb_dn9)) / (2.0 * assign29570_e23232)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) - (assign29570_e23228 * locals.var_vfb_dn10)) / (2.0 * assign29570_e23232)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) - (assign29570_e23228 * locals.var_vfb_dn11)) / (2.0 * assign29570_e23232)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) - (assign29570_e23228 * locals.var_vfb_dn12)) / (2.0 * assign29570_e23232)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign29570_e23234;
        locals.var_t0__blk1144_dn3 = assign29570_e23234_d_n3;
        locals.var_t0__blk1144_dn4 = assign29570_e23234_d_n4;
        locals.var_t0__blk1144_dn5 = assign29570_e23234_d_n5;
        locals.var_t0__blk1144_dn6 = assign29570_e23234_d_n6;
        locals.var_t0__blk1144_dn7 = assign29570_e23234_d_n7;
        locals.var_t0__blk1144_dn8 = assign29570_e23234_d_n8;
        locals.var_t0__blk1144_dn9 = assign29570_e23234_d_n9;
        locals.var_t0__blk1144_dn10 = assign29570_e23234_d_n10;
        locals.var_t0__blk1144_dn11 = assign29570_e23234_d_n11;
        locals.var_t0__blk1144_dn12 = assign29570_e23234_d_n12;

        let (assign29580_e23253, assign29580_e23253_d_n3, assign29580_e23253_d_n4, assign29580_e23253_d_n5, assign29580_e23253_d_n6, assign29580_e23253_d_n7, assign29580_e23253_d_n8, assign29580_e23253_d_n9, assign29580_e23253_d_n10, assign29580_e23253_d_n11, assign29580_e23253_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1700 == 0.0)) {
        let assign29580_e23244: f64 = (locals.var_v3 * locals.var_v3);
        let assign29580_e23247: f64 = (4.0 * 0.08);
        let assign29580_e23249: f64 = (assign29580_e23247 * locals.var_vfb);
        let assign29580_e23250: f64 = (assign29580_e23244 + assign29580_e23249);
        let assign29580_e23251: f64 = (assign29580_e23250).sqrt();
        (assign29580_e23251, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (assign29580_e23247 * locals.var_vfb_dn3)) / (2.0 * assign29580_e23251)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (assign29580_e23247 * locals.var_vfb_dn4)) / (2.0 * assign29580_e23251)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (assign29580_e23247 * locals.var_vfb_dn5)) / (2.0 * assign29580_e23251)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (assign29580_e23247 * locals.var_vfb_dn6)) / (2.0 * assign29580_e23251)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (assign29580_e23247 * locals.var_vfb_dn7)) / (2.0 * assign29580_e23251)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (assign29580_e23247 * locals.var_vfb_dn8)) / (2.0 * assign29580_e23251)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (assign29580_e23247 * locals.var_vfb_dn9)) / (2.0 * assign29580_e23251)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (assign29580_e23247 * locals.var_vfb_dn10)) / (2.0 * assign29580_e23251)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (assign29580_e23247 * locals.var_vfb_dn11)) / (2.0 * assign29580_e23251)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (assign29580_e23247 * locals.var_vfb_dn12)) / (2.0 * assign29580_e23251)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign29580_e23253;
        locals.var_t0__blk1144_dn3 = assign29580_e23253_d_n3;
        locals.var_t0__blk1144_dn4 = assign29580_e23253_d_n4;
        locals.var_t0__blk1144_dn5 = assign29580_e23253_d_n5;
        locals.var_t0__blk1144_dn6 = assign29580_e23253_d_n6;
        locals.var_t0__blk1144_dn7 = assign29580_e23253_d_n7;
        locals.var_t0__blk1144_dn8 = assign29580_e23253_d_n8;
        locals.var_t0__blk1144_dn9 = assign29580_e23253_d_n9;
        locals.var_t0__blk1144_dn10 = assign29580_e23253_d_n10;
        locals.var_t0__blk1144_dn11 = assign29580_e23253_d_n11;
        locals.var_t0__blk1144_dn12 = assign29580_e23253_d_n12;

        let (assign29590_e23266, assign29590_e23266_d_n3, assign29590_e23266_d_n4, assign29590_e23266_d_n5, assign29590_e23266_d_n6, assign29590_e23266_d_n7, assign29590_e23266_d_n8, assign29590_e23266_d_n9, assign29590_e23266_d_n10, assign29590_e23266_d_n11, assign29590_e23266_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) {
        let assign29590_e23262: f64 = (locals.var_v3 + locals.var_t0__blk1144);
        let assign29590_e23263: f64 = (0.5 * assign29590_e23262);
        let assign29590_e23264: f64 = (locals.var_vfb - assign29590_e23263);
        (assign29590_e23264, (locals.var_vfb_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_t0__blk1144_dn3))), (locals.var_vfb_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_t0__blk1144_dn4))), (locals.var_vfb_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_t0__blk1144_dn5))), (locals.var_vfb_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_t0__blk1144_dn6))), (locals.var_vfb_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_t0__blk1144_dn7))), (locals.var_vfb_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_t0__blk1144_dn8))), (locals.var_vfb_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_t0__blk1144_dn9))), (locals.var_vfb_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_t0__blk1144_dn10))), (locals.var_vfb_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_t0__blk1144_dn11))), (locals.var_vfb_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_vfbeff, locals.var_vfbeff_dn3, locals.var_vfbeff_dn4, locals.var_vfbeff_dn5, locals.var_vfbeff_dn6, locals.var_vfbeff_dn7, locals.var_vfbeff_dn8, locals.var_vfbeff_dn9, locals.var_vfbeff_dn10, locals.var_vfbeff_dn11, locals.var_vfbeff_dn12,)
    }
};
        locals.var_vfbeff = assign29590_e23266;
        locals.var_vfbeff_dn3 = assign29590_e23266_d_n3;
        locals.var_vfbeff_dn4 = assign29590_e23266_d_n4;
        locals.var_vfbeff_dn5 = assign29590_e23266_d_n5;
        locals.var_vfbeff_dn6 = assign29590_e23266_d_n6;
        locals.var_vfbeff_dn7 = assign29590_e23266_d_n7;
        locals.var_vfbeff_dn8 = assign29590_e23266_d_n8;
        locals.var_vfbeff_dn9 = assign29590_e23266_d_n9;
        locals.var_vfbeff_dn10 = assign29590_e23266_d_n10;
        locals.var_vfbeff_dn11 = assign29590_e23266_d_n11;
        locals.var_vfbeff_dn12 = assign29590_e23266_d_n12;

        let (assign29600_e23277, assign29600_e23277_d_n3, assign29600_e23277_d_n4, assign29600_e23277_d_n5, assign29600_e23277_d_n6, assign29600_e23277_d_n7, assign29600_e23277_d_n8, assign29600_e23277_d_n9, assign29600_e23277_d_n10, assign29600_e23277_d_n11, assign29600_e23277_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) {
        let assign29600_e23274: f64 = (locals.var_vfbeff - locals.var_vfb);
        let assign29600_e23275: f64 = (locals.var_coxwlb * assign29600_e23274);
        (assign29600_e23275, ((locals.var_coxwlb_dn3 * assign29600_e23274) + (locals.var_coxwlb * (locals.var_vfbeff_dn3 - locals.var_vfb_dn3))), ((locals.var_coxwlb_dn4 * assign29600_e23274) + (locals.var_coxwlb * (locals.var_vfbeff_dn4 - locals.var_vfb_dn4))), ((locals.var_coxwlb_dn5 * assign29600_e23274) + (locals.var_coxwlb * (locals.var_vfbeff_dn5 - locals.var_vfb_dn5))), ((locals.var_coxwlb_dn6 * assign29600_e23274) + (locals.var_coxwlb * (locals.var_vfbeff_dn6 - locals.var_vfb_dn6))), ((locals.var_coxwlb_dn7 * assign29600_e23274) + (locals.var_coxwlb * (locals.var_vfbeff_dn7 - locals.var_vfb_dn7))), ((locals.var_coxwlb_dn8 * assign29600_e23274) + (locals.var_coxwlb * (locals.var_vfbeff_dn8 - locals.var_vfb_dn8))), ((locals.var_coxwlb_dn9 * assign29600_e23274) + (locals.var_coxwlb * (locals.var_vfbeff_dn9 - locals.var_vfb_dn9))), ((locals.var_coxwlb_dn10 * assign29600_e23274) + (locals.var_coxwlb * (locals.var_vfbeff_dn10 - locals.var_vfb_dn10))), ((locals.var_coxwlb_dn11 * assign29600_e23274) + (locals.var_coxwlb * (locals.var_vfbeff_dn11 - locals.var_vfb_dn11))), ((locals.var_coxwlb_dn12 * assign29600_e23274) + (locals.var_coxwlb * (locals.var_vfbeff_dn12 - locals.var_vfb_dn12))),)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign29600_e23277;
        locals.var_qac0_dn3 = assign29600_e23277_d_n3;
        locals.var_qac0_dn4 = assign29600_e23277_d_n4;
        locals.var_qac0_dn5 = assign29600_e23277_d_n5;
        locals.var_qac0_dn6 = assign29600_e23277_d_n6;
        locals.var_qac0_dn7 = assign29600_e23277_d_n7;
        locals.var_qac0_dn8 = assign29600_e23277_d_n8;
        locals.var_qac0_dn9 = assign29600_e23277_d_n9;
        locals.var_qac0_dn10 = assign29600_e23277_d_n10;
        locals.var_qac0_dn11 = assign29600_e23277_d_n11;
        locals.var_qac0_dn12 = assign29600_e23277_d_n12;

        let assign29610_e23288: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1701 = assign29610_e23288;

        let (assign29620_e23299, assign29620_e23299_d_n3, assign29620_e23299_d_n4, assign29620_e23299_d_n5, assign29620_e23299_d_n6, assign29620_e23299_d_n7, assign29620_e23299_d_n8, assign29620_e23299_d_n9, assign29620_e23299_d_n10, assign29620_e23299_d_n11, assign29620_e23299_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1701 != 0.0)) {
        let assign29620_e23297: f64 = (locals.var_vfb + locals.var_eggbcp2);
        (assign29620_e23297, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11, locals.var_vfb_dn12,)
    } else {
        (locals.var_vfb2, locals.var_vfb2_dn3, locals.var_vfb2_dn4, locals.var_vfb2_dn5, locals.var_vfb2_dn6, locals.var_vfb2_dn7, locals.var_vfb2_dn8, locals.var_vfb2_dn9, locals.var_vfb2_dn10, locals.var_vfb2_dn11, locals.var_vfb2_dn12,)
    }
};
        locals.var_vfb2 = assign29620_e23299;
        locals.var_vfb2_dn3 = assign29620_e23299_d_n3;
        locals.var_vfb2_dn4 = assign29620_e23299_d_n4;
        locals.var_vfb2_dn5 = assign29620_e23299_d_n5;
        locals.var_vfb2_dn6 = assign29620_e23299_d_n6;
        locals.var_vfb2_dn7 = assign29620_e23299_d_n7;
        locals.var_vfb2_dn8 = assign29620_e23299_d_n8;
        locals.var_vfb2_dn9 = assign29620_e23299_d_n9;
        locals.var_vfb2_dn10 = assign29620_e23299_d_n10;
        locals.var_vfb2_dn11 = assign29620_e23299_d_n11;
        locals.var_vfb2_dn12 = assign29620_e23299_d_n12;

        let (assign29630_e23308,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1701 != 0.0)) {
        (0.08,)
    } else {
        (locals.var_delta_3_soi2,)
    }
};
        locals.var_delta_3_soi2 = assign29630_e23308;

        let (assign29640_e23323, assign29640_e23323_d_n3, assign29640_e23323_d_n4, assign29640_e23323_d_n5, assign29640_e23323_d_n6, assign29640_e23323_d_n7, assign29640_e23323_d_n8, assign29640_e23323_d_n9, assign29640_e23323_d_n10, assign29640_e23323_d_n11, assign29640_e23323_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1701 != 0.0)) {
        let assign29640_e23317: f64 = (locals.var_vfb2 - locals.var_vgs_eff2);
        let assign29640_e23319: f64 = (assign29640_e23317 + locals.var_vbseff);
        let assign29640_e23321: f64 = (assign29640_e23319 - locals.var_delta_3_soi2);
        (assign29640_e23321, (locals.var_vfb2_dn3 + locals.var_vbseff_dn3), (locals.var_vfb2_dn4 + locals.var_vbseff_dn4), (locals.var_vfb2_dn5 + locals.var_vbseff_dn5), (locals.var_vfb2_dn6 + locals.var_vbseff_dn6), ((locals.var_vfb2_dn7 - locals.var_vgs_eff2_dn7) + locals.var_vbseff_dn7), ((locals.var_vfb2_dn8 - locals.var_vgs_eff2_dn8) + locals.var_vbseff_dn8), ((locals.var_vfb2_dn9 - locals.var_vgs_eff2_dn9) + locals.var_vbseff_dn9), (locals.var_vfb2_dn10 + locals.var_vbseff_dn10), (locals.var_vfb2_dn11 + locals.var_vbseff_dn11), (locals.var_vfb2_dn12 + locals.var_vbseff_dn12),)
    } else {
        (locals.var_v3, locals.var_v3_dn3, locals.var_v3_dn4, locals.var_v3_dn5, locals.var_v3_dn6, locals.var_v3_dn7, locals.var_v3_dn8, locals.var_v3_dn9, locals.var_v3_dn10, locals.var_v3_dn11, locals.var_v3_dn12,)
    }
};
        locals.var_v3 = assign29640_e23323;
        locals.var_v3_dn3 = assign29640_e23323_d_n3;
        locals.var_v3_dn4 = assign29640_e23323_d_n4;
        locals.var_v3_dn5 = assign29640_e23323_d_n5;
        locals.var_v3_dn6 = assign29640_e23323_d_n6;
        locals.var_v3_dn7 = assign29640_e23323_d_n7;
        locals.var_v3_dn8 = assign29640_e23323_d_n8;
        locals.var_v3_dn9 = assign29640_e23323_d_n9;
        locals.var_v3_dn10 = assign29640_e23323_d_n10;
        locals.var_v3_dn11 = assign29640_e23323_d_n11;
        locals.var_v3_dn12 = assign29640_e23323_d_n12;

        let assign29650_e23326: f64 = if locals.var_vfb2 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1702 = assign29650_e23326;

        let (assign29660_e23346, assign29660_e23346_d_n3, assign29660_e23346_d_n4, assign29660_e23346_d_n5, assign29660_e23346_d_n6, assign29660_e23346_d_n7, assign29660_e23346_d_n8, assign29660_e23346_d_n9, assign29660_e23346_d_n10, assign29660_e23346_d_n11, assign29660_e23346_d_n12,) = {
    if ((((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1701 != 0.0)) && (locals.var_guard1702 != 0.0)) {
        let assign29660_e23337: f64 = (locals.var_v3 * locals.var_v3);
        let assign29660_e23340: f64 = (100.0 * locals.var_delta_3_soi2);
        let assign29660_e23342: f64 = (assign29660_e23340 * locals.var_vfb2);
        let assign29660_e23343: f64 = (assign29660_e23337 - assign29660_e23342);
        let assign29660_e23344: f64 = (assign29660_e23343).sqrt();
        (assign29660_e23344, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) - (assign29660_e23340 * locals.var_vfb2_dn3)) / (2.0 * assign29660_e23344)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) - (assign29660_e23340 * locals.var_vfb2_dn4)) / (2.0 * assign29660_e23344)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) - (assign29660_e23340 * locals.var_vfb2_dn5)) / (2.0 * assign29660_e23344)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) - (assign29660_e23340 * locals.var_vfb2_dn6)) / (2.0 * assign29660_e23344)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) - (assign29660_e23340 * locals.var_vfb2_dn7)) / (2.0 * assign29660_e23344)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) - (assign29660_e23340 * locals.var_vfb2_dn8)) / (2.0 * assign29660_e23344)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) - (assign29660_e23340 * locals.var_vfb2_dn9)) / (2.0 * assign29660_e23344)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) - (assign29660_e23340 * locals.var_vfb2_dn10)) / (2.0 * assign29660_e23344)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) - (assign29660_e23340 * locals.var_vfb2_dn11)) / (2.0 * assign29660_e23344)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) - (assign29660_e23340 * locals.var_vfb2_dn12)) / (2.0 * assign29660_e23344)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign29660_e23346;
        locals.var_t0__blk1144_dn3 = assign29660_e23346_d_n3;
        locals.var_t0__blk1144_dn4 = assign29660_e23346_d_n4;
        locals.var_t0__blk1144_dn5 = assign29660_e23346_d_n5;
        locals.var_t0__blk1144_dn6 = assign29660_e23346_d_n6;
        locals.var_t0__blk1144_dn7 = assign29660_e23346_d_n7;
        locals.var_t0__blk1144_dn8 = assign29660_e23346_d_n8;
        locals.var_t0__blk1144_dn9 = assign29660_e23346_d_n9;
        locals.var_t0__blk1144_dn10 = assign29660_e23346_d_n10;
        locals.var_t0__blk1144_dn11 = assign29660_e23346_d_n11;
        locals.var_t0__blk1144_dn12 = assign29660_e23346_d_n12;

        let (assign29670_e23367, assign29670_e23367_d_n3, assign29670_e23367_d_n4, assign29670_e23367_d_n5, assign29670_e23367_d_n6, assign29670_e23367_d_n7, assign29670_e23367_d_n8, assign29670_e23367_d_n9, assign29670_e23367_d_n10, assign29670_e23367_d_n11, assign29670_e23367_d_n12,) = {
    if ((((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1701 != 0.0)) && (locals.var_guard1702 == 0.0)) {
        let assign29670_e23358: f64 = (locals.var_v3 * locals.var_v3);
        let assign29670_e23361: f64 = (100.0 * locals.var_delta_3_soi2);
        let assign29670_e23363: f64 = (assign29670_e23361 * locals.var_vfb2);
        let assign29670_e23364: f64 = (assign29670_e23358 + assign29670_e23363);
        let assign29670_e23365: f64 = (assign29670_e23364).sqrt();
        (assign29670_e23365, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (assign29670_e23361 * locals.var_vfb2_dn3)) / (2.0 * assign29670_e23365)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (assign29670_e23361 * locals.var_vfb2_dn4)) / (2.0 * assign29670_e23365)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (assign29670_e23361 * locals.var_vfb2_dn5)) / (2.0 * assign29670_e23365)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (assign29670_e23361 * locals.var_vfb2_dn6)) / (2.0 * assign29670_e23365)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (assign29670_e23361 * locals.var_vfb2_dn7)) / (2.0 * assign29670_e23365)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (assign29670_e23361 * locals.var_vfb2_dn8)) / (2.0 * assign29670_e23365)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (assign29670_e23361 * locals.var_vfb2_dn9)) / (2.0 * assign29670_e23365)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (assign29670_e23361 * locals.var_vfb2_dn10)) / (2.0 * assign29670_e23365)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (assign29670_e23361 * locals.var_vfb2_dn11)) / (2.0 * assign29670_e23365)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (assign29670_e23361 * locals.var_vfb2_dn12)) / (2.0 * assign29670_e23365)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign29670_e23367;
        locals.var_t0__blk1144_dn3 = assign29670_e23367_d_n3;
        locals.var_t0__blk1144_dn4 = assign29670_e23367_d_n4;
        locals.var_t0__blk1144_dn5 = assign29670_e23367_d_n5;
        locals.var_t0__blk1144_dn6 = assign29670_e23367_d_n6;
        locals.var_t0__blk1144_dn7 = assign29670_e23367_d_n7;
        locals.var_t0__blk1144_dn8 = assign29670_e23367_d_n8;
        locals.var_t0__blk1144_dn9 = assign29670_e23367_d_n9;
        locals.var_t0__blk1144_dn10 = assign29670_e23367_d_n10;
        locals.var_t0__blk1144_dn11 = assign29670_e23367_d_n11;
        locals.var_t0__blk1144_dn12 = assign29670_e23367_d_n12;

        let (assign29680_e23382, assign29680_e23382_d_n3, assign29680_e23382_d_n4, assign29680_e23382_d_n5, assign29680_e23382_d_n6, assign29680_e23382_d_n7, assign29680_e23382_d_n8, assign29680_e23382_d_n9, assign29680_e23382_d_n10, assign29680_e23382_d_n11, assign29680_e23382_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1701 != 0.0)) {
        let assign29680_e23378: f64 = (locals.var_v3 + locals.var_t0__blk1144);
        let assign29680_e23379: f64 = (0.5 * assign29680_e23378);
        let assign29680_e23380: f64 = (locals.var_vfb2 - assign29680_e23379);
        (assign29680_e23380, (locals.var_vfb2_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_t0__blk1144_dn3))), (locals.var_vfb2_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_t0__blk1144_dn4))), (locals.var_vfb2_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_t0__blk1144_dn5))), (locals.var_vfb2_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_t0__blk1144_dn6))), (locals.var_vfb2_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_t0__blk1144_dn7))), (locals.var_vfb2_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_t0__blk1144_dn8))), (locals.var_vfb2_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_t0__blk1144_dn9))), (locals.var_vfb2_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_t0__blk1144_dn10))), (locals.var_vfb2_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_t0__blk1144_dn11))), (locals.var_vfb2_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_vfbeff2, locals.var_vfbeff2_dn3, locals.var_vfbeff2_dn4, locals.var_vfbeff2_dn5, locals.var_vfbeff2_dn6, locals.var_vfbeff2_dn7, locals.var_vfbeff2_dn8, locals.var_vfbeff2_dn9, locals.var_vfbeff2_dn10, locals.var_vfbeff2_dn11, locals.var_vfbeff2_dn12,)
    }
};
        locals.var_vfbeff2 = assign29680_e23382;
        locals.var_vfbeff2_dn3 = assign29680_e23382_d_n3;
        locals.var_vfbeff2_dn4 = assign29680_e23382_d_n4;
        locals.var_vfbeff2_dn5 = assign29680_e23382_d_n5;
        locals.var_vfbeff2_dn6 = assign29680_e23382_d_n6;
        locals.var_vfbeff2_dn7 = assign29680_e23382_d_n7;
        locals.var_vfbeff2_dn8 = assign29680_e23382_d_n8;
        locals.var_vfbeff2_dn9 = assign29680_e23382_d_n9;
        locals.var_vfbeff2_dn10 = assign29680_e23382_d_n10;
        locals.var_vfbeff2_dn11 = assign29680_e23382_d_n11;
        locals.var_vfbeff2_dn12 = assign29680_e23382_d_n12;

        let (assign29690_e23397, assign29690_e23397_d_n3, assign29690_e23397_d_n4, assign29690_e23397_d_n5, assign29690_e23397_d_n6, assign29690_e23397_d_n7, assign29690_e23397_d_n8, assign29690_e23397_d_n9, assign29690_e23397_d_n10, assign29690_e23397_d_n11, assign29690_e23397_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1701 != 0.0)) {
        let assign29690_e23393: f64 = (locals.var_vfbeff2 - locals.var_vfb2);
        let assign29690_e23394: f64 = (locals.var_coxwlb2 * assign29690_e23393);
        let assign29690_e23395: f64 = (locals.var_qac0 + assign29690_e23394);
        (assign29690_e23395, (locals.var_qac0_dn3 + ((locals.var_coxwlb2_dn3 * assign29690_e23393) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn3 - locals.var_vfb2_dn3)))), (locals.var_qac0_dn4 + ((locals.var_coxwlb2_dn4 * assign29690_e23393) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn4 - locals.var_vfb2_dn4)))), (locals.var_qac0_dn5 + ((locals.var_coxwlb2_dn5 * assign29690_e23393) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn5 - locals.var_vfb2_dn5)))), (locals.var_qac0_dn6 + ((locals.var_coxwlb2_dn6 * assign29690_e23393) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn6 - locals.var_vfb2_dn6)))), (locals.var_qac0_dn7 + ((locals.var_coxwlb2_dn7 * assign29690_e23393) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn7 - locals.var_vfb2_dn7)))), (locals.var_qac0_dn8 + ((locals.var_coxwlb2_dn8 * assign29690_e23393) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn8 - locals.var_vfb2_dn8)))), (locals.var_qac0_dn9 + ((locals.var_coxwlb2_dn9 * assign29690_e23393) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn9 - locals.var_vfb2_dn9)))), (locals.var_qac0_dn10 + ((locals.var_coxwlb2_dn10 * assign29690_e23393) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn10 - locals.var_vfb2_dn10)))), (locals.var_qac0_dn11 + ((locals.var_coxwlb2_dn11 * assign29690_e23393) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn11 - locals.var_vfb2_dn11)))), (locals.var_qac0_dn12 + ((locals.var_coxwlb2_dn12 * assign29690_e23393) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn12 - locals.var_vfb2_dn12)))),)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign29690_e23397;
        locals.var_qac0_dn3 = assign29690_e23397_d_n3;
        locals.var_qac0_dn4 = assign29690_e23397_d_n4;
        locals.var_qac0_dn5 = assign29690_e23397_d_n5;
        locals.var_qac0_dn6 = assign29690_e23397_d_n6;
        locals.var_qac0_dn7 = assign29690_e23397_d_n7;
        locals.var_qac0_dn8 = assign29690_e23397_d_n8;
        locals.var_qac0_dn9 = assign29690_e23397_d_n9;
        locals.var_qac0_dn10 = assign29690_e23397_d_n10;
        locals.var_qac0_dn11 = assign29690_e23397_d_n11;
        locals.var_qac0_dn12 = assign29690_e23397_d_n12;

        let (assign29700_e23406, assign29700_e23406_d_n3, assign29700_e23406_d_n4, assign29700_e23406_d_n5, assign29700_e23406_d_n6, assign29700_e23406_d_n7, assign29700_e23406_d_n8, assign29700_e23406_d_n9, assign29700_e23406_d_n10, assign29700_e23406_d_n11, assign29700_e23406_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) {
        let assign29700_e23404: f64 = (0.5 * locals.var_pparam_b4soik1ox);
        (assign29700_e23404, (0.5 * locals.var_pparam_b4soik1ox_dn3), (0.5 * locals.var_pparam_b4soik1ox_dn4), (0.5 * locals.var_pparam_b4soik1ox_dn5), (0.5 * locals.var_pparam_b4soik1ox_dn6), (0.5 * locals.var_pparam_b4soik1ox_dn7), (0.5 * locals.var_pparam_b4soik1ox_dn8), (0.5 * locals.var_pparam_b4soik1ox_dn9), (0.5 * locals.var_pparam_b4soik1ox_dn10), (0.5 * locals.var_pparam_b4soik1ox_dn11), (0.5 * locals.var_pparam_b4soik1ox_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign29700_e23406;
        locals.var_t0__blk1144_dn3 = assign29700_e23406_d_n3;
        locals.var_t0__blk1144_dn4 = assign29700_e23406_d_n4;
        locals.var_t0__blk1144_dn5 = assign29700_e23406_d_n5;
        locals.var_t0__blk1144_dn6 = assign29700_e23406_d_n6;
        locals.var_t0__blk1144_dn7 = assign29700_e23406_d_n7;
        locals.var_t0__blk1144_dn8 = assign29700_e23406_d_n8;
        locals.var_t0__blk1144_dn9 = assign29700_e23406_d_n9;
        locals.var_t0__blk1144_dn10 = assign29700_e23406_d_n10;
        locals.var_t0__blk1144_dn11 = assign29700_e23406_d_n11;
        locals.var_t0__blk1144_dn12 = assign29700_e23406_d_n12;

        let (assign29710_e23419, assign29710_e23419_d_n3, assign29710_e23419_d_n4, assign29710_e23419_d_n5, assign29710_e23419_d_n6, assign29710_e23419_d_n7, assign29710_e23419_d_n8, assign29710_e23419_d_n9, assign29710_e23419_d_n10, assign29710_e23419_d_n11, assign29710_e23419_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) {
        let assign29710_e23413: f64 = (locals.var_vgs_eff__blk1126 - locals.var_vfbeff);
        let assign29710_e23415: f64 = (assign29710_e23413 - locals.var_vbseff);
        let assign29710_e23417: f64 = (assign29710_e23415 - locals.var_vgsteff__blk1175);
        (assign29710_e23417, (((locals.var_vgs_eff__blk1126_dn3 - locals.var_vfbeff_dn3) - locals.var_vbseff_dn3) - locals.var_vgsteff__blk1175_dn3), (((locals.var_vgs_eff__blk1126_dn4 - locals.var_vfbeff_dn4) - locals.var_vbseff_dn4) - locals.var_vgsteff__blk1175_dn4), (((locals.var_vgs_eff__blk1126_dn5 - locals.var_vfbeff_dn5) - locals.var_vbseff_dn5) - locals.var_vgsteff__blk1175_dn5), (((locals.var_vgs_eff__blk1126_dn6 - locals.var_vfbeff_dn6) - locals.var_vbseff_dn6) - locals.var_vgsteff__blk1175_dn6), (((locals.var_vgs_eff__blk1126_dn7 - locals.var_vfbeff_dn7) - locals.var_vbseff_dn7) - locals.var_vgsteff__blk1175_dn7), (((locals.var_vgs_eff__blk1126_dn8 - locals.var_vfbeff_dn8) - locals.var_vbseff_dn8) - locals.var_vgsteff__blk1175_dn8), (((locals.var_vgs_eff__blk1126_dn9 - locals.var_vfbeff_dn9) - locals.var_vbseff_dn9) - locals.var_vgsteff__blk1175_dn9), (((locals.var_vgs_eff__blk1126_dn10 - locals.var_vfbeff_dn10) - locals.var_vbseff_dn10) - locals.var_vgsteff__blk1175_dn10), (((locals.var_vgs_eff__blk1126_dn11 - locals.var_vfbeff_dn11) - locals.var_vbseff_dn11) - locals.var_vgsteff__blk1175_dn11), (((locals.var_vgs_eff__blk1126_dn12 - locals.var_vfbeff_dn12) - locals.var_vbseff_dn12) - locals.var_vgsteff__blk1175_dn12),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign29710_e23419;
        locals.var_t3__blk1147_dn3 = assign29710_e23419_d_n3;
        locals.var_t3__blk1147_dn4 = assign29710_e23419_d_n4;
        locals.var_t3__blk1147_dn5 = assign29710_e23419_d_n5;
        locals.var_t3__blk1147_dn6 = assign29710_e23419_d_n6;
        locals.var_t3__blk1147_dn7 = assign29710_e23419_d_n7;
        locals.var_t3__blk1147_dn8 = assign29710_e23419_d_n8;
        locals.var_t3__blk1147_dn9 = assign29710_e23419_d_n9;
        locals.var_t3__blk1147_dn10 = assign29710_e23419_d_n10;
        locals.var_t3__blk1147_dn11 = assign29710_e23419_d_n11;
        locals.var_t3__blk1147_dn12 = assign29710_e23419_d_n12;

        let assign29720_e23422: f64 = if locals.var_pparam_b4soik1ox == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1703 = assign29720_e23422;

        let (assign29730_e23431, assign29730_e23431_d_n3, assign29730_e23431_d_n4, assign29730_e23431_d_n5, assign29730_e23431_d_n6, assign29730_e23431_d_n7, assign29730_e23431_d_n8, assign29730_e23431_d_n9, assign29730_e23431_d_n10, assign29730_e23431_d_n11, assign29730_e23431_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1703 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign29730_e23431;
        locals.var_t1__blk1145_dn3 = assign29730_e23431_d_n3;
        locals.var_t1__blk1145_dn4 = assign29730_e23431_d_n4;
        locals.var_t1__blk1145_dn5 = assign29730_e23431_d_n5;
        locals.var_t1__blk1145_dn6 = assign29730_e23431_d_n6;
        locals.var_t1__blk1145_dn7 = assign29730_e23431_d_n7;
        locals.var_t1__blk1145_dn8 = assign29730_e23431_d_n8;
        locals.var_t1__blk1145_dn9 = assign29730_e23431_d_n9;
        locals.var_t1__blk1145_dn10 = assign29730_e23431_d_n10;
        locals.var_t1__blk1145_dn11 = assign29730_e23431_d_n11;
        locals.var_t1__blk1145_dn12 = assign29730_e23431_d_n12;

        let assign29740_e23434: f64 = if locals.var_t3__blk1147 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1704 = assign29740_e23434;

        let (assign29750_e23450, assign29750_e23450_d_n3, assign29750_e23450_d_n4, assign29750_e23450_d_n5, assign29750_e23450_d_n6, assign29750_e23450_d_n7, assign29750_e23450_d_n8, assign29750_e23450_d_n9, assign29750_e23450_d_n10, assign29750_e23450_d_n11, assign29750_e23450_d_n12,) = {
    if ((((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1703 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign29750_e23447: f64 = (locals.var_t3__blk1147 / locals.var_pparam_b4soik1ox);
        let assign29750_e23448: f64 = (locals.var_t0__blk1144 + assign29750_e23447);
        (assign29750_e23448, (locals.var_t0__blk1144_dn3 + (((locals.var_t3__blk1147_dn3 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn3)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn4 + (((locals.var_t3__blk1147_dn4 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn4)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn5 + (((locals.var_t3__blk1147_dn5 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn5)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn6 + (((locals.var_t3__blk1147_dn6 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn6)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn7 + (((locals.var_t3__blk1147_dn7 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn7)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn8 + (((locals.var_t3__blk1147_dn8 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn8)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn9 + (((locals.var_t3__blk1147_dn9 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn9)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn10 + (((locals.var_t3__blk1147_dn10 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn10)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn11 + (((locals.var_t3__blk1147_dn11 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn11)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn12 + (((locals.var_t3__blk1147_dn12 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn12)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign29750_e23450;
        locals.var_t1__blk1145_dn3 = assign29750_e23450_d_n3;
        locals.var_t1__blk1145_dn4 = assign29750_e23450_d_n4;
        locals.var_t1__blk1145_dn5 = assign29750_e23450_d_n5;
        locals.var_t1__blk1145_dn6 = assign29750_e23450_d_n6;
        locals.var_t1__blk1145_dn7 = assign29750_e23450_d_n7;
        locals.var_t1__blk1145_dn8 = assign29750_e23450_d_n8;
        locals.var_t1__blk1145_dn9 = assign29750_e23450_d_n9;
        locals.var_t1__blk1145_dn10 = assign29750_e23450_d_n10;
        locals.var_t1__blk1145_dn11 = assign29750_e23450_d_n11;
        locals.var_t1__blk1145_dn12 = assign29750_e23450_d_n12;

        let (assign29760_e23468, assign29760_e23468_d_n3, assign29760_e23468_d_n4, assign29760_e23468_d_n5, assign29760_e23468_d_n6, assign29760_e23468_d_n7, assign29760_e23468_d_n8, assign29760_e23468_d_n9, assign29760_e23468_d_n10, assign29760_e23468_d_n11, assign29760_e23468_d_n12,) = {
    if ((((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1703 == 0.0)) && (locals.var_guard1704 == 0.0)) {
        let assign29760_e23463: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign29760_e23465: f64 = (assign29760_e23463 + locals.var_t3__blk1147);
        let assign29760_e23466: f64 = (assign29760_e23465).sqrt();
        (assign29760_e23466, ((((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) + locals.var_t3__blk1147_dn3) / (2.0 * assign29760_e23466)), ((((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) + locals.var_t3__blk1147_dn4) / (2.0 * assign29760_e23466)), ((((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) + locals.var_t3__blk1147_dn5) / (2.0 * assign29760_e23466)), ((((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) + locals.var_t3__blk1147_dn6) / (2.0 * assign29760_e23466)), ((((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) + locals.var_t3__blk1147_dn7) / (2.0 * assign29760_e23466)), ((((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) + locals.var_t3__blk1147_dn8) / (2.0 * assign29760_e23466)), ((((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) + locals.var_t3__blk1147_dn9) / (2.0 * assign29760_e23466)), ((((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) + locals.var_t3__blk1147_dn10) / (2.0 * assign29760_e23466)), ((((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) + locals.var_t3__blk1147_dn11) / (2.0 * assign29760_e23466)), ((((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) + locals.var_t3__blk1147_dn12) / (2.0 * assign29760_e23466)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign29760_e23468;
        locals.var_t1__blk1145_dn3 = assign29760_e23468_d_n3;
        locals.var_t1__blk1145_dn4 = assign29760_e23468_d_n4;
        locals.var_t1__blk1145_dn5 = assign29760_e23468_d_n5;
        locals.var_t1__blk1145_dn6 = assign29760_e23468_d_n6;
        locals.var_t1__blk1145_dn7 = assign29760_e23468_d_n7;
        locals.var_t1__blk1145_dn8 = assign29760_e23468_d_n8;
        locals.var_t1__blk1145_dn9 = assign29760_e23468_d_n9;
        locals.var_t1__blk1145_dn10 = assign29760_e23468_d_n10;
        locals.var_t1__blk1145_dn11 = assign29760_e23468_d_n11;
        locals.var_t1__blk1145_dn12 = assign29760_e23468_d_n12;

        let (assign29770_e23481, assign29770_e23481_d_n3, assign29770_e23481_d_n4, assign29770_e23481_d_n5, assign29770_e23481_d_n6, assign29770_e23481_d_n7, assign29770_e23481_d_n8, assign29770_e23481_d_n9, assign29770_e23481_d_n10, assign29770_e23481_d_n11, assign29770_e23481_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) {
        let assign29770_e23475: f64 = (locals.var_coxwlb * locals.var_pparam_b4soik1ox);
        let assign29770_e23478: f64 = (locals.var_t1__blk1145 - locals.var_t0__blk1144);
        let assign29770_e23479: f64 = (assign29770_e23475 * assign29770_e23478);
        (assign29770_e23479, ((((locals.var_coxwlb_dn3 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb * locals.var_pparam_b4soik1ox_dn3)) * assign29770_e23478) + (assign29770_e23475 * (locals.var_t1__blk1145_dn3 - locals.var_t0__blk1144_dn3))), ((((locals.var_coxwlb_dn4 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb * locals.var_pparam_b4soik1ox_dn4)) * assign29770_e23478) + (assign29770_e23475 * (locals.var_t1__blk1145_dn4 - locals.var_t0__blk1144_dn4))), ((((locals.var_coxwlb_dn5 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb * locals.var_pparam_b4soik1ox_dn5)) * assign29770_e23478) + (assign29770_e23475 * (locals.var_t1__blk1145_dn5 - locals.var_t0__blk1144_dn5))), ((((locals.var_coxwlb_dn6 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb * locals.var_pparam_b4soik1ox_dn6)) * assign29770_e23478) + (assign29770_e23475 * (locals.var_t1__blk1145_dn6 - locals.var_t0__blk1144_dn6))), ((((locals.var_coxwlb_dn7 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb * locals.var_pparam_b4soik1ox_dn7)) * assign29770_e23478) + (assign29770_e23475 * (locals.var_t1__blk1145_dn7 - locals.var_t0__blk1144_dn7))), ((((locals.var_coxwlb_dn8 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb * locals.var_pparam_b4soik1ox_dn8)) * assign29770_e23478) + (assign29770_e23475 * (locals.var_t1__blk1145_dn8 - locals.var_t0__blk1144_dn8))), ((((locals.var_coxwlb_dn9 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb * locals.var_pparam_b4soik1ox_dn9)) * assign29770_e23478) + (assign29770_e23475 * (locals.var_t1__blk1145_dn9 - locals.var_t0__blk1144_dn9))), ((((locals.var_coxwlb_dn10 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb * locals.var_pparam_b4soik1ox_dn10)) * assign29770_e23478) + (assign29770_e23475 * (locals.var_t1__blk1145_dn10 - locals.var_t0__blk1144_dn10))), ((((locals.var_coxwlb_dn11 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb * locals.var_pparam_b4soik1ox_dn11)) * assign29770_e23478) + (assign29770_e23475 * (locals.var_t1__blk1145_dn11 - locals.var_t0__blk1144_dn11))), ((((locals.var_coxwlb_dn12 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb * locals.var_pparam_b4soik1ox_dn12)) * assign29770_e23478) + (assign29770_e23475 * (locals.var_t1__blk1145_dn12 - locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign29770_e23481;
        locals.var_qsub0_dn3 = assign29770_e23481_d_n3;
        locals.var_qsub0_dn4 = assign29770_e23481_d_n4;
        locals.var_qsub0_dn5 = assign29770_e23481_d_n5;
        locals.var_qsub0_dn6 = assign29770_e23481_d_n6;
        locals.var_qsub0_dn7 = assign29770_e23481_d_n7;
        locals.var_qsub0_dn8 = assign29770_e23481_d_n8;
        locals.var_qsub0_dn9 = assign29770_e23481_d_n9;
        locals.var_qsub0_dn10 = assign29770_e23481_d_n10;
        locals.var_qsub0_dn11 = assign29770_e23481_d_n11;
        locals.var_qsub0_dn12 = assign29770_e23481_d_n12;

        let assign29780_e23492: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1705 = assign29780_e23492;

    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50840_e65595, assign50840_e65595_d_n5, assign50840_e65595_d_n6, assign50840_e65595_d_n7, assign50840_e65595_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xgs_dc, locals.var_xgs_dc_dn5, locals.var_xgs_dc_dn6, locals.var_xgs_dc_dn7, locals.var_xgs_dc_dn8,)
    } else {
        (locals.var_xgs__blk1358, locals.var_xgs__blk1358_dn5, locals.var_xgs__blk1358_dn6, locals.var_xgs__blk1358_dn7, locals.var_xgs__blk1358_dn8,)
    }
};
        locals.var_xgs__blk1358 = assign50840_e65595;
        locals.var_xgs__blk1358_dn5 = assign50840_e65595_d_n5;
        locals.var_xgs__blk1358_dn6 = assign50840_e65595_d_n6;
        locals.var_xgs__blk1358_dn7 = assign50840_e65595_d_n7;
        locals.var_xgs__blk1358_dn8 = assign50840_e65595_d_n8;
        locals.var_xgs__blk1358_rv = 0.0;

        let (assign50850_e65602, assign50850_e65602_d_n5, assign50850_e65602_d_n6, assign50850_e65602_d_n7, assign50850_e65602_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_qis_dc, locals.var_qis_dc_dn5, locals.var_qis_dc_dn6, locals.var_qis_dc_dn7, locals.var_qis_dc_dn8,)
    } else {
        (locals.var_qis__blk1359, locals.var_qis__blk1359_dn5, locals.var_qis__blk1359_dn6, locals.var_qis__blk1359_dn7, locals.var_qis__blk1359_dn8,)
    }
};
        locals.var_qis__blk1359 = assign50850_e65602;
        locals.var_qis__blk1359_dn5 = assign50850_e65602_d_n5;
        locals.var_qis__blk1359_dn6 = assign50850_e65602_d_n6;
        locals.var_qis__blk1359_dn7 = assign50850_e65602_d_n7;
        locals.var_qis__blk1359_dn8 = assign50850_e65602_d_n8;
        locals.var_qis__blk1359_rv = 0.0;

        let (assign50860_e65609, assign50860_e65609_d_n5, assign50860_e65609_d_n6, assign50860_e65609_d_n7, assign50860_e65609_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_qbs_dc, locals.var_qbs_dc_dn5, locals.var_qbs_dc_dn6, locals.var_qbs_dc_dn7, locals.var_qbs_dc_dn8,)
    } else {
        (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8,)
    }
};
        locals.var_qbs__blk1360 = assign50860_e65609;
        locals.var_qbs__blk1360_dn5 = assign50860_e65609_d_n5;
        locals.var_qbs__blk1360_dn6 = assign50860_e65609_d_n6;
        locals.var_qbs__blk1360_dn7 = assign50860_e65609_d_n7;
        locals.var_qbs__blk1360_dn8 = assign50860_e65609_d_n8;
        locals.var_qbs__blk1360_rv = 0.0;

        let (assign50870_e65616, assign50870_e65616_d_n5, assign50870_e65616_d_n6, assign50870_e65616_d_n7, assign50870_e65616_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_rhob_dc, locals.var_rhob_dc_dn5, locals.var_rhob_dc_dn6, locals.var_rhob_dc_dn7, locals.var_rhob_dc_dn8,)
    } else {
        (locals.var_rhob__blk1361, locals.var_rhob__blk1361_dn5, locals.var_rhob__blk1361_dn6, locals.var_rhob__blk1361_dn7, locals.var_rhob__blk1361_dn8,)
    }
};
        locals.var_rhob__blk1361 = assign50870_e65616;
        locals.var_rhob__blk1361_dn5 = assign50870_e65616_d_n5;
        locals.var_rhob__blk1361_dn6 = assign50870_e65616_d_n6;
        locals.var_rhob__blk1361_dn7 = assign50870_e65616_d_n7;
        locals.var_rhob__blk1361_dn8 = assign50870_e65616_d_n8;
        locals.var_rhob__blk1361_rv = 0.0;

        let (assign50880_e65623, assign50880_e65623_d_n5, assign50880_e65623_d_n6, assign50880_e65623_d_n7, assign50880_e65623_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_rhog_dc, locals.var_rhog_dc_dn5, locals.var_rhog_dc_dn6, locals.var_rhog_dc_dn7, locals.var_rhog_dc_dn8,)
    } else {
        (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8,)
    }
};
        locals.var_rhog__blk1362 = assign50880_e65623;
        locals.var_rhog__blk1362_dn5 = assign50880_e65623_d_n5;
        locals.var_rhog__blk1362_dn6 = assign50880_e65623_d_n6;
        locals.var_rhog__blk1362_dn7 = assign50880_e65623_d_n7;
        locals.var_rhog__blk1362_dn8 = assign50880_e65623_d_n8;
        locals.var_rhog__blk1362_rv = 0.0;

        let (assign50890_e65630, assign50890_e65630_d_n5, assign50890_e65630_d_n6, assign50890_e65630_d_n7, assign50890_e65630_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_gmobs_dc, locals.var_gmobs_dc_dn5, locals.var_gmobs_dc_dn6, locals.var_gmobs_dc_dn7, locals.var_gmobs_dc_dn8,)
    } else {
        (locals.var_gmobs__blk1366, locals.var_gmobs__blk1366_dn5, locals.var_gmobs__blk1366_dn6, locals.var_gmobs__blk1366_dn7, locals.var_gmobs__blk1366_dn8,)
    }
};
        locals.var_gmobs__blk1366 = assign50890_e65630;
        locals.var_gmobs__blk1366_dn5 = assign50890_e65630_d_n5;
        locals.var_gmobs__blk1366_dn6 = assign50890_e65630_d_n6;
        locals.var_gmobs__blk1366_dn7 = assign50890_e65630_d_n7;
        locals.var_gmobs__blk1366_dn8 = assign50890_e65630_d_n8;
        locals.var_gmobs__blk1366_rv = 0.0;

        let (assign50900_e65637, assign50900_e65637_d_n5, assign50900_e65637_d_n6, assign50900_e65637_d_n7, assign50900_e65637_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xitsb_dc, locals.var_xitsb_dc_dn5, locals.var_xitsb_dc_dn6, locals.var_xitsb_dc_dn7, locals.var_xitsb_dc_dn8,)
    } else {
        (locals.var_xitsb__blk1367, locals.var_xitsb__blk1367_dn5, locals.var_xitsb__blk1367_dn6, locals.var_xitsb__blk1367_dn7, locals.var_xitsb__blk1367_dn8,)
    }
};
        locals.var_xitsb__blk1367 = assign50900_e65637;
        locals.var_xitsb__blk1367_dn5 = assign50900_e65637_d_n5;
        locals.var_xitsb__blk1367_dn6 = assign50900_e65637_d_n6;
        locals.var_xitsb__blk1367_dn7 = assign50900_e65637_d_n7;
        locals.var_xitsb__blk1367_dn8 = assign50900_e65637_d_n8;
        locals.var_xitsb__blk1367_rv = 0.0;

        let (assign50910_e65644, assign50910_e65644_d_n5, assign50910_e65644_d_n6, assign50910_e65644_d_n7, assign50910_e65644_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_factheta_dc, locals.var_factheta_dc_dn5, locals.var_factheta_dc_dn6, locals.var_factheta_dc_dn7, locals.var_factheta_dc_dn8,)
    } else {
        (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8,)
    }
};
        locals.var_factheta__blk1369 = assign50910_e65644;
        locals.var_factheta__blk1369_dn5 = assign50910_e65644_d_n5;
        locals.var_factheta__blk1369_dn6 = assign50910_e65644_d_n6;
        locals.var_factheta__blk1369_dn7 = assign50910_e65644_d_n7;
        locals.var_factheta__blk1369_dn8 = assign50910_e65644_d_n8;
        locals.var_factheta__blk1369_rv = 0.0;

        let (assign50930_e65655,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_thesat_t,)
    } else {
        (locals.var_thesatloc__blk1302,)
    }
};
        locals.var_thesatloc__blk1302 = assign50930_e65655;
        locals.var_thesatloc__blk1302_rv = 0.0;

        let (assign50940_e65659,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_ar,)
    } else {
        (locals.var_arloc__blk1303,)
    }
};
        locals.var_arloc__blk1303 = assign50940_e65659;
        locals.var_arloc__blk1303_rv = 0.0;

        let assign50950_e65662: f64 = if p.p48 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1483 = assign50950_e65662;
        locals.var_guard1483_rv = 0.0;

        let (assign50960_e65668,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1483 != 0.0)) {
        (locals.var_thesatac_t,)
    } else {
        (locals.var_thesatloc__blk1302,)
    }
};
        locals.var_thesatloc__blk1302 = assign50960_e65668;
        locals.var_thesatloc__blk1302_rv = 0.0;

        let (assign50970_e65674,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1483 != 0.0)) {
        (locals.var_arac,)
    } else {
        (locals.var_arloc__blk1303,)
    }
};
        locals.var_arloc__blk1303 = assign50970_e65674;
        locals.var_arloc__blk1303_rv = 0.0;

        let (assign50980_e65678, assign50980_e65678_d_n5, assign50980_e65678_d_n6, assign50980_e65678_d_n7, assign50980_e65678_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesat1__blk1371, locals.var_thesat1__blk1371_dn5, locals.var_thesat1__blk1371_dn6, locals.var_thesat1__blk1371_dn7, locals.var_thesat1__blk1371_dn8,)
    }
};
        locals.var_thesat1__blk1371 = assign50980_e65678;
        locals.var_thesat1__blk1371_dn5 = assign50980_e65678_d_n5;
        locals.var_thesat1__blk1371_dn6 = assign50980_e65678_d_n6;
        locals.var_thesat1__blk1371_dn7 = assign50980_e65678_d_n7;
        locals.var_thesat1__blk1371_dn8 = assign50980_e65678_d_n8;
        locals.var_thesat1__blk1371_rv = 0.0;

        let (assign50990_e65684, assign50990_e65684_d_n5, assign50990_e65684_d_n6, assign50990_e65684_d_n7, assign50990_e65684_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        let assign50990_e65682: f64 = (locals.var_phit1__blk1322 * 4.60517018598809);
        (assign50990_e65682, (locals.var_phit1__blk1322_dn5 * 4.60517018598809), (locals.var_phit1__blk1322_dn6 * 4.60517018598809), (locals.var_phit1__blk1322_dn7 * 4.60517018598809), (locals.var_phit1__blk1322_dn8 * 4.60517018598809),)
    } else {
        (locals.var_vdsat_lim__blk1370, locals.var_vdsat_lim__blk1370_dn5, locals.var_vdsat_lim__blk1370_dn6, locals.var_vdsat_lim__blk1370_dn7, locals.var_vdsat_lim__blk1370_dn8,)
    }
};
        locals.var_vdsat_lim__blk1370 = assign50990_e65684;
        locals.var_vdsat_lim__blk1370_dn5 = assign50990_e65684_d_n5;
        locals.var_vdsat_lim__blk1370_dn6 = assign50990_e65684_d_n6;
        locals.var_vdsat_lim__blk1370_dn7 = assign50990_e65684_d_n7;
        locals.var_vdsat_lim__blk1370_dn8 = assign50990_e65684_d_n8;
        locals.var_vdsat_lim__blk1370_rv = 0.0;

        let (assign51000_e65688, assign51000_e65688_d_n5, assign51000_e65688_d_n6, assign51000_e65688_d_n7, assign51000_e65688_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_vdsat_lim__blk1370, locals.var_vdsat_lim__blk1370_dn5, locals.var_vdsat_lim__blk1370_dn6, locals.var_vdsat_lim__blk1370_dn7, locals.var_vdsat_lim__blk1370_dn8,)
    } else {
        (locals.var_v_dsat__blk1387, locals.var_v_dsat__blk1387_dn5, locals.var_v_dsat__blk1387_dn6, locals.var_v_dsat__blk1387_dn7, locals.var_v_dsat__blk1387_dn8,)
    }
};
        locals.var_v_dsat__blk1387 = assign51000_e65688;
        locals.var_v_dsat__blk1387_dn5 = assign51000_e65688_d_n5;
        locals.var_v_dsat__blk1387_dn6 = assign51000_e65688_d_n6;
        locals.var_v_dsat__blk1387_dn7 = assign51000_e65688_d_n7;
        locals.var_v_dsat__blk1387_dn8 = assign51000_e65688_d_n8;
        locals.var_v_dsat__blk1387_rv = 0.0;

        let (assign51010_e65692, assign51010_e65692_d_n5, assign51010_e65692_d_n6, assign51010_e65692_d_n7, assign51010_e65692_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_v_ds, 0.0, locals.var_v_ds_dn6, locals.var_v_ds_dn7, 0.0,)
    } else {
        (locals.var_vdse__blk1388, locals.var_vdse__blk1388_dn5, locals.var_vdse__blk1388_dn6, locals.var_vdse__blk1388_dn7, locals.var_vdse__blk1388_dn8,)
    }
};
        locals.var_vdse__blk1388 = assign51010_e65692;
        locals.var_vdse__blk1388_dn5 = assign51010_e65692_d_n5;
        locals.var_vdse__blk1388_dn6 = assign51010_e65692_d_n6;
        locals.var_vdse__blk1388_dn7 = assign51010_e65692_d_n7;
        locals.var_vdse__blk1388_dn8 = assign51010_e65692_d_n8;
        locals.var_vdse__blk1388_rv = 0.0;

        let (assign51020_e65698, assign51020_e65698_d_n5, assign51020_e65698_d_n6, assign51020_e65698_d_n7, assign51020_e65698_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        let assign51020_e65696: f64 = (locals.var_v_ds * locals.var_inv_phit1__blk1323);
        (assign51020_e65696, (locals.var_v_ds * locals.var_inv_phit1__blk1323_dn5), ((locals.var_v_ds_dn6 * locals.var_inv_phit1__blk1323) + (locals.var_v_ds * locals.var_inv_phit1__blk1323_dn6)), ((locals.var_v_ds_dn7 * locals.var_inv_phit1__blk1323) + (locals.var_v_ds * locals.var_inv_phit1__blk1323_dn7)), (locals.var_v_ds * locals.var_inv_phit1__blk1323_dn8),)
    } else {
        (locals.var_udse__blk1389, locals.var_udse__blk1389_dn5, locals.var_udse__blk1389_dn6, locals.var_udse__blk1389_dn7, locals.var_udse__blk1389_dn8,)
    }
};
        locals.var_udse__blk1389 = assign51020_e65698;
        locals.var_udse__blk1389_dn5 = assign51020_e65698_d_n5;
        locals.var_udse__blk1389_dn6 = assign51020_e65698_d_n6;
        locals.var_udse__blk1389_dn7 = assign51020_e65698_d_n7;
        locals.var_udse__blk1389_dn8 = assign51020_e65698_d_n8;
        locals.var_udse__blk1389_rv = 0.0;

        let (assign51030_e65702, assign51030_e65702_d_n5, assign51030_e65702_d_n6, assign51030_e65702_d_n7, assign51030_e65702_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    } else {
        (locals.var_x_d__blk1393, locals.var_x_d__blk1393_dn5, locals.var_x_d__blk1393_dn6, locals.var_x_d__blk1393_dn7, locals.var_x_d__blk1393_dn8,)
    }
};
        locals.var_x_d__blk1393 = assign51030_e65702;
        locals.var_x_d__blk1393_dn5 = assign51030_e65702_d_n5;
        locals.var_x_d__blk1393_dn6 = assign51030_e65702_d_n6;
        locals.var_x_d__blk1393_dn7 = assign51030_e65702_d_n7;
        locals.var_x_d__blk1393_dn8 = assign51030_e65702_d_n8;
        locals.var_x_d__blk1393_rv = 0.0;

        let (assign51040_e65706, assign51040_e65706_d_n5, assign51040_e65706_d_n6, assign51040_e65706_d_n7, assign51040_e65706_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_x_ds__blk1394, locals.var_x_ds__blk1394_dn5, locals.var_x_ds__blk1394_dn6, locals.var_x_ds__blk1394_dn7, locals.var_x_ds__blk1394_dn8,)
    }
};
        locals.var_x_ds__blk1394 = assign51040_e65706;
        locals.var_x_ds__blk1394_dn5 = assign51040_e65706_d_n5;
        locals.var_x_ds__blk1394_dn6 = assign51040_e65706_d_n6;
        locals.var_x_ds__blk1394_dn7 = assign51040_e65706_d_n7;
        locals.var_x_ds__blk1394_dn8 = assign51040_e65706_d_n8;
        locals.var_x_ds__blk1394_rv = 0.0;

        let (assign51050_e65710, assign51050_e65710_d_n5, assign51050_e65710_d_n6, assign51050_e65710_d_n7, assign51050_e65710_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1397, locals.var_dps__blk1397_dn5, locals.var_dps__blk1397_dn6, locals.var_dps__blk1397_dn7, locals.var_dps__blk1397_dn8,)
    }
};
        locals.var_dps__blk1397 = assign51050_e65710;
        locals.var_dps__blk1397_dn5 = assign51050_e65710_d_n5;
        locals.var_dps__blk1397_dn6 = assign51050_e65710_d_n6;
        locals.var_dps__blk1397_dn7 = assign51050_e65710_d_n7;
        locals.var_dps__blk1397_dn8 = assign51050_e65710_d_n8;
        locals.var_dps__blk1397_rv = 0.0;

        let (assign51060_e65714, assign51060_e65714_d_n5, assign51060_e65714_d_n6, assign51060_e65714_d_n7, assign51060_e65714_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    } else {
        (locals.var_ed__blk1399, locals.var_ed__blk1399_dn5, locals.var_ed__blk1399_dn6, locals.var_ed__blk1399_dn7, locals.var_ed__blk1399_dn8,)
    }
};
        locals.var_ed__blk1399 = assign51060_e65714;
        locals.var_ed__blk1399_dn5 = assign51060_e65714_d_n5;
        locals.var_ed__blk1399_dn6 = assign51060_e65714_d_n6;
        locals.var_ed__blk1399_dn7 = assign51060_e65714_d_n7;
        locals.var_ed__blk1399_dn8 = assign51060_e65714_d_n8;
        locals.var_ed__blk1399_rv = 0.0;

        let (assign51070_e65718, assign51070_e65718_d_n5, assign51070_e65718_d_n6, assign51070_e65718_d_n7, assign51070_e65718_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8,)
    } else {
        (locals.var_pd__blk1400, locals.var_pd__blk1400_dn5, locals.var_pd__blk1400_dn6, locals.var_pd__blk1400_dn7, locals.var_pd__blk1400_dn8,)
    }
};
        locals.var_pd__blk1400 = assign51070_e65718;
        locals.var_pd__blk1400_dn5 = assign51070_e65718_d_n5;
        locals.var_pd__blk1400_dn6 = assign51070_e65718_d_n6;
        locals.var_pd__blk1400_dn7 = assign51070_e65718_d_n7;
        locals.var_pd__blk1400_dn8 = assign51070_e65718_d_n8;
        locals.var_pd__blk1400_rv = 0.0;

        let (assign51080_e65722, assign51080_e65722_d_n5, assign51080_e65722_d_n6, assign51080_e65722_d_n7, assign51080_e65722_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8,)
    } else {
        (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8,)
    }
};
        locals.var_dd__blk1402 = assign51080_e65722;
        locals.var_dd__blk1402_dn5 = assign51080_e65722_d_n5;
        locals.var_dd__blk1402_dn6 = assign51080_e65722_d_n6;
        locals.var_dd__blk1402_dn7 = assign51080_e65722_d_n7;
        locals.var_dd__blk1402_dn8 = assign51080_e65722_d_n8;
        locals.var_dd__blk1402_rv = 0.0;

        let (assign51090_e65726, assign51090_e65726_d_n5, assign51090_e65726_d_n6, assign51090_e65726_d_n7, assign51090_e65726_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8,)
    } else {
        (locals.var_qbd__blk1403, locals.var_qbd__blk1403_dn5, locals.var_qbd__blk1403_dn6, locals.var_qbd__blk1403_dn7, locals.var_qbd__blk1403_dn8,)
    }
};
        locals.var_qbd__blk1403 = assign51090_e65726;
        locals.var_qbd__blk1403_dn5 = assign51090_e65726_d_n5;
        locals.var_qbd__blk1403_dn6 = assign51090_e65726_d_n6;
        locals.var_qbd__blk1403_dn7 = assign51090_e65726_d_n7;
        locals.var_qbd__blk1403_dn8 = assign51090_e65726_d_n8;
        locals.var_qbd__blk1403_rv = 0.0;

        let (assign51100_e65730, assign51100_e65730_d_n5, assign51100_e65730_d_n6, assign51100_e65730_d_n7, assign51100_e65730_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    } else {
        (locals.var_x_m__blk1404, locals.var_x_m__blk1404_dn5, locals.var_x_m__blk1404_dn6, locals.var_x_m__blk1404_dn7, locals.var_x_m__blk1404_dn8,)
    }
};
        locals.var_x_m__blk1404 = assign51100_e65730;
        locals.var_x_m__blk1404_dn5 = assign51100_e65730_d_n5;
        locals.var_x_m__blk1404_dn6 = assign51100_e65730_d_n6;
        locals.var_x_m__blk1404_dn7 = assign51100_e65730_d_n7;
        locals.var_x_m__blk1404_dn8 = assign51100_e65730_d_n8;
        locals.var_x_m__blk1404_rv = 0.0;

        let (assign51110_e65734, assign51110_e65734_d_n5, assign51110_e65734_d_n6, assign51110_e65734_d_n7, assign51110_e65734_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    } else {
        (locals.var_em__blk1405, locals.var_em__blk1405_dn5, locals.var_em__blk1405_dn6, locals.var_em__blk1405_dn7, locals.var_em__blk1405_dn8,)
    }
};
        locals.var_em__blk1405 = assign51110_e65734;
        locals.var_em__blk1405_dn5 = assign51110_e65734_d_n5;
        locals.var_em__blk1405_dn6 = assign51110_e65734_d_n6;
        locals.var_em__blk1405_dn7 = assign51110_e65734_d_n7;
        locals.var_em__blk1405_dn8 = assign51110_e65734_d_n8;
        locals.var_em__blk1405_rv = 0.0;

        let (assign51120_e65738, assign51120_e65738_d_n5, assign51120_e65738_d_n6, assign51120_e65738_d_n7, assign51120_e65738_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8,)
    } else {
        (locals.var_dm__blk1407, locals.var_dm__blk1407_dn5, locals.var_dm__blk1407_dn6, locals.var_dm__blk1407_dn7, locals.var_dm__blk1407_dn8,)
    }
};
        locals.var_dm__blk1407 = assign51120_e65738;
        locals.var_dm__blk1407_dn5 = assign51120_e65738_d_n5;
        locals.var_dm__blk1407_dn6 = assign51120_e65738_d_n6;
        locals.var_dm__blk1407_dn7 = assign51120_e65738_d_n7;
        locals.var_dm__blk1407_dn8 = assign51120_e65738_d_n8;
        locals.var_dm__blk1407_rv = 0.0;

        let (assign51130_e65742, assign51130_e65742_d_n5, assign51130_e65742_d_n6, assign51130_e65742_d_n7, assign51130_e65742_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8,)
    } else {
        (locals.var_pm__blk1408, locals.var_pm__blk1408_dn5, locals.var_pm__blk1408_dn6, locals.var_pm__blk1408_dn7, locals.var_pm__blk1408_dn8,)
    }
};
        locals.var_pm__blk1408 = assign51130_e65742;
        locals.var_pm__blk1408_dn5 = assign51130_e65742_d_n5;
        locals.var_pm__blk1408_dn6 = assign51130_e65742_d_n6;
        locals.var_pm__blk1408_dn7 = assign51130_e65742_d_n7;
        locals.var_pm__blk1408_dn8 = assign51130_e65742_d_n8;
        locals.var_pm__blk1408_rv = 0.0;

        let (assign51140_e65748, assign51140_e65748_d_n5, assign51140_e65748_d_n6, assign51140_e65748_d_n7, assign51140_e65748_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        let assign51140_e65746: f64 = (locals.var_xg__blk1326 - locals.var_x_s__blk1346);
        (assign51140_e65746, (locals.var_xg__blk1326_dn5 - locals.var_x_s__blk1346_dn5), (locals.var_xg__blk1326_dn6 - locals.var_x_s__blk1346_dn6), (locals.var_xg__blk1326_dn7 - locals.var_x_s__blk1346_dn7), (locals.var_xg__blk1326_dn8 - locals.var_x_s__blk1346_dn8),)
    } else {
        (locals.var_xgm__blk1409, locals.var_xgm__blk1409_dn5, locals.var_xgm__blk1409_dn6, locals.var_xgm__blk1409_dn7, locals.var_xgm__blk1409_dn8,)
    }
};
        locals.var_xgm__blk1409 = assign51140_e65748;
        locals.var_xgm__blk1409_dn5 = assign51140_e65748_d_n5;
        locals.var_xgm__blk1409_dn6 = assign51140_e65748_d_n6;
        locals.var_xgm__blk1409_dn7 = assign51140_e65748_d_n7;
        locals.var_xgm__blk1409_dn8 = assign51140_e65748_d_n8;
        locals.var_xgm__blk1409_rv = 0.0;

        let (assign51150_e65752, assign51150_e65752_d_n5, assign51150_e65752_d_n6, assign51150_e65752_d_n7, assign51150_e65752_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eta_p__blk1410, locals.var_eta_p__blk1410_dn5, locals.var_eta_p__blk1410_dn6, locals.var_eta_p__blk1410_dn7, locals.var_eta_p__blk1410_dn8,)
    }
};
        locals.var_eta_p__blk1410 = assign51150_e65752;
        locals.var_eta_p__blk1410_dn5 = assign51150_e65752_d_n5;
        locals.var_eta_p__blk1410_dn6 = assign51150_e65752_d_n6;
        locals.var_eta_p__blk1410_dn7 = assign51150_e65752_d_n7;
        locals.var_eta_p__blk1410_dn8 = assign51150_e65752_d_n8;
        locals.var_eta_p__blk1410_rv = 0.0;

        let (assign51160_e65756, assign51160_e65756_d_n5, assign51160_e65756_d_n6, assign51160_e65756_d_n7, assign51160_e65756_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_alpha__blk1412, locals.var_alpha__blk1412_dn5, locals.var_alpha__blk1412_dn6, locals.var_alpha__blk1412_dn7, locals.var_alpha__blk1412_dn8,)
    }
};
        locals.var_alpha__blk1412 = assign51160_e65756;
        locals.var_alpha__blk1412_dn5 = assign51160_e65756_d_n5;
        locals.var_alpha__blk1412_dn6 = assign51160_e65756_d_n6;
        locals.var_alpha__blk1412_dn7 = assign51160_e65756_d_n7;
        locals.var_alpha__blk1412_dn8 = assign51160_e65756_d_n8;
        locals.var_alpha__blk1412_rv = 0.0;

        let (assign51170_e65760, assign51170_e65760_d_n5, assign51170_e65760_d_n6, assign51170_e65760_d_n7, assign51170_e65760_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sqm__blk1411, locals.var_sqm__blk1411_dn5, locals.var_sqm__blk1411_dn6, locals.var_sqm__blk1411_dn7, locals.var_sqm__blk1411_dn8,)
    }
};
        locals.var_sqm__blk1411 = assign51170_e65760;
        locals.var_sqm__blk1411_dn5 = assign51170_e65760_d_n5;
        locals.var_sqm__blk1411_dn6 = assign51170_e65760_d_n6;
        locals.var_sqm__blk1411_dn7 = assign51170_e65760_d_n7;
        locals.var_sqm__blk1411_dn8 = assign51170_e65760_d_n8;
        locals.var_sqm__blk1411_rv = 0.0;

        let (assign51180_e65764, assign51180_e65764_d_n5, assign51180_e65764_d_n6, assign51180_e65764_d_n7, assign51180_e65764_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qis__blk1359, locals.var_qis__blk1359_dn5, locals.var_qis__blk1359_dn6, locals.var_qis__blk1359_dn7, locals.var_qis__blk1359_dn8,)
    } else {
        (locals.var_qim__blk1421, locals.var_qim__blk1421_dn5, locals.var_qim__blk1421_dn6, locals.var_qim__blk1421_dn7, locals.var_qim__blk1421_dn8,)
    }
};
        locals.var_qim__blk1421 = assign51180_e65764;
        locals.var_qim__blk1421_dn5 = assign51180_e65764_d_n5;
        locals.var_qim__blk1421_dn6 = assign51180_e65764_d_n6;
        locals.var_qim__blk1421_dn7 = assign51180_e65764_d_n7;
        locals.var_qim__blk1421_dn8 = assign51180_e65764_d_n8;
        locals.var_qim__blk1421_rv = 0.0;

        let (assign51190_e65770, assign51190_e65770_d_n5, assign51190_e65770_d_n6, assign51190_e65770_d_n7, assign51190_e65770_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        let assign51190_e65768: f64 = (locals.var_xgm__blk1409 * locals.var_phit1__blk1322);
        (assign51190_e65768, ((locals.var_xgm__blk1409_dn5 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn5)), ((locals.var_xgm__blk1409_dn6 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn6)), ((locals.var_xgm__blk1409_dn7 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn7)), ((locals.var_xgm__blk1409_dn8 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_qeff1__blk1425, locals.var_qeff1__blk1425_dn5, locals.var_qeff1__blk1425_dn6, locals.var_qeff1__blk1425_dn7, locals.var_qeff1__blk1425_dn8,)
    }
};
        locals.var_qeff1__blk1425 = assign51190_e65770;
        locals.var_qeff1__blk1425_dn5 = assign51190_e65770_d_n5;
        locals.var_qeff1__blk1425_dn6 = assign51190_e65770_d_n6;
        locals.var_qeff1__blk1425_dn7 = assign51190_e65770_d_n7;
        locals.var_qeff1__blk1425_dn8 = assign51190_e65770_d_n8;
        locals.var_qeff1__blk1425_rv = 0.0;

        let (assign51200_e65774, assign51200_e65774_d_n5, assign51200_e65774_d_n6, assign51200_e65774_d_n7, assign51200_e65774_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qim1__blk1422, locals.var_qim1__blk1422_dn5, locals.var_qim1__blk1422_dn6, locals.var_qim1__blk1422_dn7, locals.var_qim1__blk1422_dn8,)
    }
};
        locals.var_qim1__blk1422 = assign51200_e65774;
        locals.var_qim1__blk1422_dn5 = assign51200_e65774_d_n5;
        locals.var_qim1__blk1422_dn6 = assign51200_e65774_d_n6;
        locals.var_qim1__blk1422_dn7 = assign51200_e65774_d_n7;
        locals.var_qim1__blk1422_dn8 = assign51200_e65774_d_n8;
        locals.var_qim1__blk1422_rv = 0.0;

        let (assign51210_e65778, assign51210_e65778_d_n5, assign51210_e65778_d_n6, assign51210_e65778_d_n7, assign51210_e65778_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8,)
    } else {
        (locals.var_qbm__blk1423, locals.var_qbm__blk1423_dn5, locals.var_qbm__blk1423_dn6, locals.var_qbm__blk1423_dn7, locals.var_qbm__blk1423_dn8,)
    }
};
        locals.var_qbm__blk1423 = assign51210_e65778;
        locals.var_qbm__blk1423_dn5 = assign51210_e65778_d_n5;
        locals.var_qbm__blk1423_dn6 = assign51210_e65778_d_n6;
        locals.var_qbm__blk1423_dn7 = assign51210_e65778_d_n7;
        locals.var_qbm__blk1423_dn8 = assign51210_e65778_d_n8;
        locals.var_qbm__blk1423_rv = 0.0;

        let (assign51220_e65782, assign51220_e65782_d_n5, assign51220_e65782_d_n6, assign51220_e65782_d_n7, assign51220_e65782_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_s1__blk1428, locals.var_s1__blk1428_dn5, locals.var_s1__blk1428_dn6, locals.var_s1__blk1428_dn7, locals.var_s1__blk1428_dn8,)
    }
};
        locals.var_s1__blk1428 = assign51220_e65782;
        locals.var_s1__blk1428_dn5 = assign51220_e65782_d_n5;
        locals.var_s1__blk1428_dn6 = assign51220_e65782_d_n6;
        locals.var_s1__blk1428_dn7 = assign51220_e65782_d_n7;
        locals.var_s1__blk1428_dn8 = assign51220_e65782_d_n8;
        locals.var_s1__blk1428_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_44(
        locals: &mut StampLocals,
    ) {
        let (assign51230_e65786, assign51230_e65786_d_n5, assign51230_e65786_d_n6, assign51230_e65786_d_n7, assign51230_e65786_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gmob__blk1427, locals.var_gmob__blk1427_dn5, locals.var_gmob__blk1427_dn6, locals.var_gmob__blk1427_dn7, locals.var_gmob__blk1427_dn8,)
    }
};
        locals.var_gmob__blk1427 = assign51230_e65786;
        locals.var_gmob__blk1427_dn5 = assign51230_e65786_d_n5;
        locals.var_gmob__blk1427_dn6 = assign51230_e65786_d_n6;
        locals.var_gmob__blk1427_dn7 = assign51230_e65786_d_n7;
        locals.var_gmob__blk1427_dn8 = assign51230_e65786_d_n8;
        locals.var_gmob__blk1427_rv = 0.0;

        let (assign51240_e65790, assign51240_e65790_d_n5, assign51240_e65790_d_n6, assign51240_e65790_d_n7, assign51240_e65790_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_thesatloc__blk1302, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesateff__blk1430, locals.var_thesateff__blk1430_dn5, locals.var_thesateff__blk1430_dn6, locals.var_thesateff__blk1430_dn7, locals.var_thesateff__blk1430_dn8,)
    }
};
        locals.var_thesateff__blk1430 = assign51240_e65790;
        locals.var_thesateff__blk1430_dn5 = assign51240_e65790_d_n5;
        locals.var_thesateff__blk1430_dn6 = assign51240_e65790_d_n6;
        locals.var_thesateff__blk1430_dn7 = assign51240_e65790_d_n7;
        locals.var_thesateff__blk1430_dn8 = assign51240_e65790_d_n8;
        locals.var_thesateff__blk1430_rv = 0.0;

        let (assign51250_e65794, assign51250_e65794_d_n5, assign51250_e65794_d_n6, assign51250_e65794_d_n7, assign51250_e65794_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qeff1__blk1425, locals.var_qeff1__blk1425_dn5, locals.var_qeff1__blk1425_dn6, locals.var_qeff1__blk1425_dn7, locals.var_qeff1__blk1425_dn8,)
    } else {
        (locals.var_voxm__blk1429, locals.var_voxm__blk1429_dn5, locals.var_voxm__blk1429_dn6, locals.var_voxm__blk1429_dn7, locals.var_voxm__blk1429_dn8,)
    }
};
        locals.var_voxm__blk1429 = assign51250_e65794;
        locals.var_voxm__blk1429_dn5 = assign51250_e65794_d_n5;
        locals.var_voxm__blk1429_dn6 = assign51250_e65794_d_n6;
        locals.var_voxm__blk1429_dn7 = assign51250_e65794_d_n7;
        locals.var_voxm__blk1429_dn8 = assign51250_e65794_d_n8;
        locals.var_voxm__blk1429_rv = 0.0;

        let assign51260_e65797: f64 = if locals.var_xg__blk1326 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1484 = assign51260_e65797;
        locals.var_guard1484_rv = 0.0;

        let assign51270_e65800: f64 = if locals.var_ds__blk1353 > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1485 = assign51270_e65800;
        locals.var_guard1485_rv = 0.0;

        let (assign51280_e65810, assign51280_e65810_d_n5, assign51280_e65810_d_n6, assign51280_e65810_d_n7, assign51280_e65810_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51280_e65808: f64 = (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369);
        (assign51280_e65808, (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn5), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn6), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn7), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn8),)
    } else {
        (locals.var_thesateff__blk1430, locals.var_thesateff__blk1430_dn5, locals.var_thesateff__blk1430_dn6, locals.var_thesateff__blk1430_dn7, locals.var_thesateff__blk1430_dn8,)
    }
};
        locals.var_thesateff__blk1430 = assign51280_e65810;
        locals.var_thesateff__blk1430_dn5 = assign51280_e65810_d_n5;
        locals.var_thesateff__blk1430_dn6 = assign51280_e65810_d_n6;
        locals.var_thesateff__blk1430_dn7 = assign51280_e65810_d_n7;
        locals.var_thesateff__blk1430_dn8 = assign51280_e65810_d_n8;
        locals.var_thesateff__blk1430_rv = 0.0;

        let (assign51290_e65820, assign51290_e65820_d_n5, assign51290_e65820_d_n6, assign51290_e65820_d_n7, assign51290_e65820_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51290_e65818: f64 = (locals.var_thesateff__blk1430 / locals.var_gmobs__blk1366);
        (assign51290_e65818, (((locals.var_thesateff__blk1430_dn5 * locals.var_gmobs__blk1366) - (locals.var_thesateff__blk1430 * locals.var_gmobs__blk1366_dn5)) / (locals.var_gmobs__blk1366 * locals.var_gmobs__blk1366)), (((locals.var_thesateff__blk1430_dn6 * locals.var_gmobs__blk1366) - (locals.var_thesateff__blk1430 * locals.var_gmobs__blk1366_dn6)) / (locals.var_gmobs__blk1366 * locals.var_gmobs__blk1366)), (((locals.var_thesateff__blk1430_dn7 * locals.var_gmobs__blk1366) - (locals.var_thesateff__blk1430 * locals.var_gmobs__blk1366_dn7)) / (locals.var_gmobs__blk1366 * locals.var_gmobs__blk1366)), (((locals.var_thesateff__blk1430_dn8 * locals.var_gmobs__blk1366) - (locals.var_thesateff__blk1430 * locals.var_gmobs__blk1366_dn8)) / (locals.var_gmobs__blk1366 * locals.var_gmobs__blk1366)),)
    } else {
        (locals.var_thesat1__blk1371, locals.var_thesat1__blk1371_dn5, locals.var_thesat1__blk1371_dn6, locals.var_thesat1__blk1371_dn7, locals.var_thesat1__blk1371_dn8,)
    }
};
        locals.var_thesat1__blk1371 = assign51290_e65820;
        locals.var_thesat1__blk1371_dn5 = assign51290_e65820_d_n5;
        locals.var_thesat1__blk1371_dn6 = assign51290_e65820_d_n6;
        locals.var_thesat1__blk1371_dn7 = assign51290_e65820_d_n7;
        locals.var_thesat1__blk1371_dn8 = assign51290_e65820_d_n8;
        locals.var_thesat1__blk1371_rv = 0.0;

        let (assign51300_e65832, assign51300_e65832_d_n5, assign51300_e65832_d_n6, assign51300_e65832_d_n7, assign51300_e65832_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51300_e65829: f64 = (0.5 * locals.var_gf2__blk1308);
        let assign51300_e65830: f64 = (locals.var_xgs__blk1358 + assign51300_e65829);
        (assign51300_e65830, (locals.var_xgs__blk1358_dn5 + (0.5 * locals.var_gf2__blk1308_dn5)), (locals.var_xgs__blk1358_dn6 + (0.5 * locals.var_gf2__blk1308_dn6)), (locals.var_xgs__blk1358_dn7 + (0.5 * locals.var_gf2__blk1308_dn7)), (locals.var_xgs__blk1358_dn8 + (0.5 * locals.var_gf2__blk1308_dn8)),)
    } else {
        (locals.var_asat__blk1372, locals.var_asat__blk1372_dn5, locals.var_asat__blk1372_dn6, locals.var_asat__blk1372_dn7, locals.var_asat__blk1372_dn8,)
    }
};
        locals.var_asat__blk1372 = assign51300_e65832;
        locals.var_asat__blk1372_dn5 = assign51300_e65832_d_n5;
        locals.var_asat__blk1372_dn6 = assign51300_e65832_d_n6;
        locals.var_asat__blk1372_dn7 = assign51300_e65832_d_n7;
        locals.var_asat__blk1372_dn8 = assign51300_e65832_d_n8;
        locals.var_asat__blk1372_rv = 0.0;

        let (assign51310_e65846, assign51310_e65846_d_n5, assign51310_e65846_d_n6, assign51310_e65846_d_n7, assign51310_e65846_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51310_e65840: f64 = (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_asat__blk1372;
        let assign51310_e65842: f64 = (assign51310_e65840 * __rspice_inv_cse_0);
        let assign51310_e65844: f64 = (assign51310_e65842 * __rspice_inv_cse_0);
        (assign51310_e65844, ((((((((locals.var_gf2__blk1308_dn5 * locals.var_delta_1s__blk1351) + (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351_dn5)) * locals.var_asat__blk1372) - (assign51310_e65840 * locals.var_asat__blk1372_dn5)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)) * locals.var_asat__blk1372) - (assign51310_e65842 * locals.var_asat__blk1372_dn5)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)), ((((((((locals.var_gf2__blk1308_dn6 * locals.var_delta_1s__blk1351) + (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351_dn6)) * locals.var_asat__blk1372) - (assign51310_e65840 * locals.var_asat__blk1372_dn6)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)) * locals.var_asat__blk1372) - (assign51310_e65842 * locals.var_asat__blk1372_dn6)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)), ((((((((locals.var_gf2__blk1308_dn7 * locals.var_delta_1s__blk1351) + (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351_dn7)) * locals.var_asat__blk1372) - (assign51310_e65840 * locals.var_asat__blk1372_dn7)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)) * locals.var_asat__blk1372) - (assign51310_e65842 * locals.var_asat__blk1372_dn7)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)), ((((((((locals.var_gf2__blk1308_dn8 * locals.var_delta_1s__blk1351) + (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351_dn8)) * locals.var_asat__blk1372) - (assign51310_e65840 * locals.var_asat__blk1372_dn8)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)) * locals.var_asat__blk1372) - (assign51310_e65842 * locals.var_asat__blk1372_dn8)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51310_e65846;
        locals.var_temp__blk936_dn5 = assign51310_e65846_d_n5;
        locals.var_temp__blk936_dn6 = assign51310_e65846_d_n6;
        locals.var_temp__blk936_dn7 = assign51310_e65846_d_n7;
        locals.var_temp__blk936_dn8 = assign51310_e65846_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign51320_e65849: f64 = if locals.var_temp__blk936 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1486 = assign51320_e65849;
        locals.var_guard1486_rv = 0.0;

        let (assign51330_e65861, assign51330_e65861_d_n5, assign51330_e65861_d_n6, assign51330_e65861_d_n7, assign51330_e65861_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign51330_e65859: f64 = (1.0 - locals.var_temp__blk936);
        (assign51330_e65859, (-locals.var_temp__blk936_dn5), (-locals.var_temp__blk936_dn6), (-locals.var_temp__blk936_dn7), (-locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign51330_e65861;
        locals.var_temp1_dn5 = assign51330_e65861_d_n5;
        locals.var_temp1_dn6 = assign51330_e65861_d_n6;
        locals.var_temp1_dn7 = assign51330_e65861_d_n7;
        locals.var_temp1_dn8 = assign51330_e65861_d_n8;
        locals.var_temp1_rv = 0.0;

        let assign51340_e65864: f64 = if locals.var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1487 = assign51340_e65864;
        locals.var_guard1487_rv = 0.0;

        let (assign51350_e65876, assign51350_e65876_d_n5, assign51350_e65876_d_n6, assign51350_e65876_d_n7, assign51350_e65876_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 != 0.0)) && (locals.var_guard1487 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign51350_e65876;
        locals.var_temp2_dn5 = assign51350_e65876_d_n5;
        locals.var_temp2_dn6 = assign51350_e65876_d_n6;
        locals.var_temp2_dn7 = assign51350_e65876_d_n7;
        locals.var_temp2_dn8 = assign51350_e65876_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign51360_e65892, assign51360_e65892_d_n5, assign51360_e65892_d_n6, assign51360_e65892_d_n7, assign51360_e65892_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 != 0.0)) && (locals.var_guard1487 == 0.0)) {
        let assign51360_e65889: f64 = (locals.var_temp1).sqrt();
        let assign51360_e65890: f64 = (1.0 - assign51360_e65889);
        (assign51360_e65890, (-(locals.var_temp1_dn5 / (2.0 * assign51360_e65889))), (-(locals.var_temp1_dn6 / (2.0 * assign51360_e65889))), (-(locals.var_temp1_dn7 / (2.0 * assign51360_e65889))), (-(locals.var_temp1_dn8 / (2.0 * assign51360_e65889))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign51360_e65892;
        locals.var_temp2_dn5 = assign51360_e65892_d_n5;
        locals.var_temp2_dn6 = assign51360_e65892_d_n6;
        locals.var_temp2_dn7 = assign51360_e65892_d_n7;
        locals.var_temp2_dn8 = assign51360_e65892_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign51370_e65905, assign51370_e65905_d_n5, assign51370_e65905_d_n6, assign51370_e65905_d_n7, assign51370_e65905_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign51370_e65903: f64 = (0.5 * locals.var_temp__blk936);
        (assign51370_e65903, (0.5 * locals.var_temp__blk936_dn5), (0.5 * locals.var_temp__blk936_dn6), (0.5 * locals.var_temp__blk936_dn7), (0.5 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign51370_e65905;
        locals.var_temp2_dn5 = assign51370_e65905_d_n5;
        locals.var_temp2_dn6 = assign51370_e65905_d_n6;
        locals.var_temp2_dn7 = assign51370_e65905_d_n7;
        locals.var_temp2_dn8 = assign51370_e65905_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign51380_e65915, assign51380_e65915_d_n5, assign51380_e65915_d_n6, assign51380_e65915_d_n7, assign51380_e65915_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51380_e65913: f64 = (locals.var_temp2 * locals.var_asat__blk1372);
        (assign51380_e65913, ((locals.var_temp2_dn5 * locals.var_asat__blk1372) + (locals.var_temp2 * locals.var_asat__blk1372_dn5)), ((locals.var_temp2_dn6 * locals.var_asat__blk1372) + (locals.var_temp2 * locals.var_asat__blk1372_dn6)), ((locals.var_temp2_dn7 * locals.var_asat__blk1372) + (locals.var_temp2 * locals.var_asat__blk1372_dn7)), ((locals.var_temp2_dn8 * locals.var_asat__blk1372) + (locals.var_temp2 * locals.var_asat__blk1372_dn8)),)
    } else {
        (locals.var_x_inf0__blk1373, locals.var_x_inf0__blk1373_dn5, locals.var_x_inf0__blk1373_dn6, locals.var_x_inf0__blk1373_dn7, locals.var_x_inf0__blk1373_dn8,)
    }
};
        locals.var_x_inf0__blk1373 = assign51380_e65915;
        locals.var_x_inf0__blk1373_dn5 = assign51380_e65915_d_n5;
        locals.var_x_inf0__blk1373_dn6 = assign51380_e65915_d_n6;
        locals.var_x_inf0__blk1373_dn7 = assign51380_e65915_d_n7;
        locals.var_x_inf0__blk1373_dn8 = assign51380_e65915_d_n8;
        locals.var_x_inf0__blk1373_rv = 0.0;

        let assign51390_e65922: f64 = if ((locals.var_cs_t > 0.0) && (locals.var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1488 = assign51390_e65922;
        locals.var_guard1488_rv = 0.0;

        let (assign51400_e65936, assign51400_e65936_d_n5, assign51400_e65936_d_n6, assign51400_e65936_d_n7, assign51400_e65936_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51400_e65932: f64 = (0.475 * locals.var_phit1__blk1322);
        let assign51400_e65934: f64 = (assign51400_e65932 * locals.var_x_inf0__blk1373);
        (assign51400_e65934, (((0.475 * locals.var_phit1__blk1322_dn5) * locals.var_x_inf0__blk1373) + (assign51400_e65932 * locals.var_x_inf0__blk1373_dn5)), (((0.475 * locals.var_phit1__blk1322_dn6) * locals.var_x_inf0__blk1373) + (assign51400_e65932 * locals.var_x_inf0__blk1373_dn6)), (((0.475 * locals.var_phit1__blk1322_dn7) * locals.var_x_inf0__blk1373) + (assign51400_e65932 * locals.var_x_inf0__blk1373_dn7)), (((0.475 * locals.var_phit1__blk1322_dn8) * locals.var_x_inf0__blk1373) + (assign51400_e65932 * locals.var_x_inf0__blk1373_dn8)),)
    } else {
        (locals.var_midphi0__blk1374, locals.var_midphi0__blk1374_dn5, locals.var_midphi0__blk1374_dn6, locals.var_midphi0__blk1374_dn7, locals.var_midphi0__blk1374_dn8,)
    }
};
        locals.var_midphi0__blk1374 = assign51400_e65936;
        locals.var_midphi0__blk1374_dn5 = assign51400_e65936_d_n5;
        locals.var_midphi0__blk1374_dn6 = assign51400_e65936_d_n6;
        locals.var_midphi0__blk1374_dn7 = assign51400_e65936_d_n7;
        locals.var_midphi0__blk1374_dn8 = assign51400_e65936_d_n8;
        locals.var_midphi0__blk1374_rv = 0.0;

        let (assign51410_e65950, assign51410_e65950_d_n5, assign51410_e65950_d_n6, assign51410_e65950_d_n7, assign51410_e65950_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51410_e65947: f64 = (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374);
        let assign51410_e65948: f64 = (locals.var_qis__blk1359 - assign51410_e65947);
        (assign51410_e65948, (locals.var_qis__blk1359_dn5 - ((locals.var_alphas__blk1356_dn5 * locals.var_midphi0__blk1374) + (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374_dn5))), (locals.var_qis__blk1359_dn6 - ((locals.var_alphas__blk1356_dn6 * locals.var_midphi0__blk1374) + (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374_dn6))), (locals.var_qis__blk1359_dn7 - ((locals.var_alphas__blk1356_dn7 * locals.var_midphi0__blk1374) + (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374_dn7))), (locals.var_qis__blk1359_dn8 - ((locals.var_alphas__blk1356_dn8 * locals.var_midphi0__blk1374) + (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374_dn8))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51410_e65950;
        locals.var_temp__blk936_dn5 = assign51410_e65950_d_n5;
        locals.var_temp__blk936_dn6 = assign51410_e65950_d_n6;
        locals.var_temp__blk936_dn7 = assign51410_e65950_d_n7;
        locals.var_temp__blk936_dn8 = assign51410_e65950_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign51420_e65969, assign51420_e65969_d_n5, assign51420_e65969_d_n6, assign51420_e65969_d_n7, assign51420_e65969_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51420_e65962: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
        let assign51420_e65964: f64 = (assign51420_e65962 + 1e-12);
        let assign51420_e65965: f64 = (assign51420_e65964).sqrt();
        let assign51420_e65966: f64 = (locals.var_temp__blk936 + assign51420_e65965);
        let assign51420_e65967: f64 = (0.5 * assign51420_e65966);
        (assign51420_e65967, (0.5 * (locals.var_temp__blk936_dn5 + (((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) / (2.0 * assign51420_e65965)))), (0.5 * (locals.var_temp__blk936_dn6 + (((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) / (2.0 * assign51420_e65965)))), (0.5 * (locals.var_temp__blk936_dn7 + (((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) / (2.0 * assign51420_e65965)))), (0.5 * (locals.var_temp__blk936_dn8 + (((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) / (2.0 * assign51420_e65965)))),)
    } else {
        (locals.var_qisat__blk1375, locals.var_qisat__blk1375_dn5, locals.var_qisat__blk1375_dn6, locals.var_qisat__blk1375_dn7, locals.var_qisat__blk1375_dn8,)
    }
};
        locals.var_qisat__blk1375 = assign51420_e65969;
        locals.var_qisat__blk1375_dn5 = assign51420_e65969_d_n5;
        locals.var_qisat__blk1375_dn6 = assign51420_e65969_d_n6;
        locals.var_qisat__blk1375_dn7 = assign51420_e65969_d_n7;
        locals.var_qisat__blk1375_dn8 = assign51420_e65969_d_n8;
        locals.var_qisat__blk1375_rv = 0.0;

        let (assign51430_e65989, assign51430_e65989_d_n5, assign51430_e65989_d_n6, assign51430_e65989_d_n7, assign51430_e65989_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51430_e65979: f64 = (locals.var_phit1__blk1322 * locals.var_xgs__blk1358);
        let assign51430_e65981: f64 = (assign51430_e65979 - locals.var_qis__blk1359);
        let assign51430_e65984: f64 = (locals.var_alphas__blk1356 - 1.0);
        let assign51430_e65986: f64 = (assign51430_e65984 * locals.var_midphi0__blk1374);
        let assign51430_e65987: f64 = (assign51430_e65981 + assign51430_e65986);
        (assign51430_e65987, ((((locals.var_phit1__blk1322_dn5 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn5)) - locals.var_qis__blk1359_dn5) + ((locals.var_alphas__blk1356_dn5 * locals.var_midphi0__blk1374) + (assign51430_e65984 * locals.var_midphi0__blk1374_dn5))), ((((locals.var_phit1__blk1322_dn6 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn6)) - locals.var_qis__blk1359_dn6) + ((locals.var_alphas__blk1356_dn6 * locals.var_midphi0__blk1374) + (assign51430_e65984 * locals.var_midphi0__blk1374_dn6))), ((((locals.var_phit1__blk1322_dn7 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn7)) - locals.var_qis__blk1359_dn7) + ((locals.var_alphas__blk1356_dn7 * locals.var_midphi0__blk1374) + (assign51430_e65984 * locals.var_midphi0__blk1374_dn7))), ((((locals.var_phit1__blk1322_dn8 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn8)) - locals.var_qis__blk1359_dn8) + ((locals.var_alphas__blk1356_dn8 * locals.var_midphi0__blk1374) + (assign51430_e65984 * locals.var_midphi0__blk1374_dn8))),)
    } else {
        (locals.var_qbsat__blk1376, locals.var_qbsat__blk1376_dn5, locals.var_qbsat__blk1376_dn6, locals.var_qbsat__blk1376_dn7, locals.var_qbsat__blk1376_dn8,)
    }
};
        locals.var_qbsat__blk1376 = assign51430_e65989;
        locals.var_qbsat__blk1376_dn5 = assign51430_e65989_d_n5;
        locals.var_qbsat__blk1376_dn6 = assign51430_e65989_d_n6;
        locals.var_qbsat__blk1376_dn7 = assign51430_e65989_d_n7;
        locals.var_qbsat__blk1376_dn8 = assign51430_e65989_d_n8;
        locals.var_qbsat__blk1376_rv = 0.0;

        let (assign51440_e66007, assign51440_e66007_d_n5, assign51440_e66007_d_n6, assign51440_e66007_d_n7, assign51440_e66007_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51440_e66000: f64 = (0.5 * locals.var_gf2__blk1308);
        let assign51440_e66002: f64 = (assign51440_e66000 * locals.var_phit1__blk1322);
        let assign51440_e66004: f64 = (assign51440_e66002 / locals.var_qbsat__blk1376);
        let assign51440_e66005: f64 = (1.0 + assign51440_e66004);
        (assign51440_e66005, ((((((0.5 * locals.var_gf2__blk1308_dn5) * locals.var_phit1__blk1322) + (assign51440_e66000 * locals.var_phit1__blk1322_dn5)) * locals.var_qbsat__blk1376) - (assign51440_e66002 * locals.var_qbsat__blk1376_dn5)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), ((((((0.5 * locals.var_gf2__blk1308_dn6) * locals.var_phit1__blk1322) + (assign51440_e66000 * locals.var_phit1__blk1322_dn6)) * locals.var_qbsat__blk1376) - (assign51440_e66002 * locals.var_qbsat__blk1376_dn6)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), ((((((0.5 * locals.var_gf2__blk1308_dn7) * locals.var_phit1__blk1322) + (assign51440_e66000 * locals.var_phit1__blk1322_dn7)) * locals.var_qbsat__blk1376) - (assign51440_e66002 * locals.var_qbsat__blk1376_dn7)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), ((((((0.5 * locals.var_gf2__blk1308_dn8) * locals.var_phit1__blk1322) + (assign51440_e66000 * locals.var_phit1__blk1322_dn8)) * locals.var_qbsat__blk1376) - (assign51440_e66002 * locals.var_qbsat__blk1376_dn8)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)),)
    } else {
        (locals.var_alphasat__blk1377, locals.var_alphasat__blk1377_dn5, locals.var_alphasat__blk1377_dn6, locals.var_alphasat__blk1377_dn7, locals.var_alphasat__blk1377_dn8,)
    }
};
        locals.var_alphasat__blk1377 = assign51440_e66007;
        locals.var_alphasat__blk1377_dn5 = assign51440_e66007_d_n5;
        locals.var_alphasat__blk1377_dn6 = assign51440_e66007_d_n6;
        locals.var_alphasat__blk1377_dn7 = assign51440_e66007_d_n7;
        locals.var_alphasat__blk1377_dn8 = assign51440_e66007_d_n8;
        locals.var_alphasat__blk1377_rv = 0.0;

        let (assign51450_e66021, assign51450_e66021_d_n5, assign51450_e66021_d_n6, assign51450_e66021_d_n7, assign51450_e66021_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51450_e66018: f64 = (locals.var_eta_mu * locals.var_qisat__blk1375);
        let assign51450_e66019: f64 = (locals.var_qbsat__blk1376 + assign51450_e66018);
        (assign51450_e66019, (locals.var_qbsat__blk1376_dn5 + (locals.var_eta_mu * locals.var_qisat__blk1375_dn5)), (locals.var_qbsat__blk1376_dn6 + (locals.var_eta_mu * locals.var_qisat__blk1375_dn6)), (locals.var_qbsat__blk1376_dn7 + (locals.var_eta_mu * locals.var_qisat__blk1375_dn7)), (locals.var_qbsat__blk1376_dn8 + (locals.var_eta_mu * locals.var_qisat__blk1375_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51450_e66021;
        locals.var_temp__blk936_dn5 = assign51450_e66021_d_n5;
        locals.var_temp__blk936_dn6 = assign51450_e66021_d_n6;
        locals.var_temp__blk936_dn7 = assign51450_e66021_d_n7;
        locals.var_temp__blk936_dn8 = assign51450_e66021_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign51460_e66037, assign51460_e66037_d_n5, assign51460_e66037_d_n6, assign51460_e66037_d_n7, assign51460_e66037_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51460_e66031: f64 = (locals.var_e_eff0 * locals.var_temp__blk936);
        let assign51460_e66033: f64 = (assign51460_e66031 * locals.var_mue_t);
        let assign51460_e66035: f64 = (assign51460_e66033).powf(locals.var_themu_t);
        (assign51460_e66035, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51460_e66033).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn5) * locals.var_mue_t))) } } else { (assign51460_e66035 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn5) * locals.var_mue_t) / assign51460_e66033))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51460_e66033).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn6) * locals.var_mue_t))) } } else { (assign51460_e66035 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn6) * locals.var_mue_t) / assign51460_e66033))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51460_e66033).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn7) * locals.var_mue_t))) } } else { (assign51460_e66035 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn7) * locals.var_mue_t) / assign51460_e66033))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51460_e66033).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn8) * locals.var_mue_t))) } } else { (assign51460_e66035 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn8) * locals.var_mue_t) / assign51460_e66033))) },)
    } else {
        (locals.var_gmobmusat__blk1378, locals.var_gmobmusat__blk1378_dn5, locals.var_gmobmusat__blk1378_dn6, locals.var_gmobmusat__blk1378_dn7, locals.var_gmobmusat__blk1378_dn8,)
    }
};
        locals.var_gmobmusat__blk1378 = assign51460_e66037;
        locals.var_gmobmusat__blk1378_dn5 = assign51460_e66037_d_n5;
        locals.var_gmobmusat__blk1378_dn6 = assign51460_e66037_d_n6;
        locals.var_gmobmusat__blk1378_dn7 = assign51460_e66037_d_n7;
        locals.var_gmobmusat__blk1378_dn8 = assign51460_e66037_d_n8;
        locals.var_gmobmusat__blk1378_rv = 0.0;

        let (assign51470_e66059, assign51470_e66059_d_n5, assign51470_e66059_d_n6, assign51470_e66059_d_n7, assign51470_e66059_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51470_e66049: f64 = (1.0 - locals.var_eta_mu);
        let assign51470_e66050: f64 = (locals.var_alphasat__blk1377 * assign51470_e66049);
        let assign51470_e66052: f64 = (assign51470_e66050 - 1.0);
        let assign51470_e66053: f64 = (locals.var_themu_t * assign51470_e66052);
        let assign51470_e66055: f64 = (assign51470_e66053 / locals.var_temp__blk936);
        let assign51470_e66057: f64 = (assign51470_e66055 * locals.var_gmobmusat__blk1378);
        (assign51470_e66057, ((((((locals.var_themu_t * (locals.var_alphasat__blk1377_dn5 * assign51470_e66049)) * locals.var_temp__blk936) - (assign51470_e66053 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat__blk1378) + (assign51470_e66055 * locals.var_gmobmusat__blk1378_dn5)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1377_dn6 * assign51470_e66049)) * locals.var_temp__blk936) - (assign51470_e66053 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat__blk1378) + (assign51470_e66055 * locals.var_gmobmusat__blk1378_dn6)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1377_dn7 * assign51470_e66049)) * locals.var_temp__blk936) - (assign51470_e66053 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat__blk1378) + (assign51470_e66055 * locals.var_gmobmusat__blk1378_dn7)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1377_dn8 * assign51470_e66049)) * locals.var_temp__blk936) - (assign51470_e66053 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat__blk1378) + (assign51470_e66055 * locals.var_gmobmusat__blk1378_dn8)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign51470_e66059;
        locals.var_temp1_dn5 = assign51470_e66059_d_n5;
        locals.var_temp1_dn6 = assign51470_e66059_d_n6;
        locals.var_temp1_dn7 = assign51470_e66059_d_n7;
        locals.var_temp1_dn8 = assign51470_e66059_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign51480_e66071, assign51480_e66071_d_n5, assign51480_e66071_d_n6, assign51480_e66071_d_n7, assign51480_e66071_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51480_e66069: f64 = (locals.var_qisat__blk1375 / locals.var_qbsat__blk1376);
        (assign51480_e66069, (((locals.var_qisat__blk1375_dn5 * locals.var_qbsat__blk1376) - (locals.var_qisat__blk1375 * locals.var_qbsat__blk1376_dn5)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), (((locals.var_qisat__blk1375_dn6 * locals.var_qbsat__blk1376) - (locals.var_qisat__blk1375 * locals.var_qbsat__blk1376_dn6)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), (((locals.var_qisat__blk1375_dn7 * locals.var_qbsat__blk1376) - (locals.var_qisat__blk1375 * locals.var_qbsat__blk1376_dn7)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), (((locals.var_qisat__blk1375_dn8 * locals.var_qbsat__blk1376) - (locals.var_qisat__blk1375 * locals.var_qbsat__blk1376_dn8)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51480_e66071;
        locals.var_temp__blk936_dn5 = assign51480_e66071_d_n5;
        locals.var_temp__blk936_dn6 = assign51480_e66071_d_n6;
        locals.var_temp__blk936_dn7 = assign51480_e66071_d_n7;
        locals.var_temp__blk936_dn8 = assign51480_e66071_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign51490_e66088, assign51490_e66088_d_n5, assign51490_e66088_d_n6, assign51490_e66088_d_n7, assign51490_e66088_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51490_e66082: f64 = (1.0 + locals.var_temp__blk936);
        let assign51490_e66084: f64 = (-locals.var_thecs_t);
        let assign51490_e66085: f64 = (assign51490_e66082).powf(assign51490_e66084);
        let assign51490_e66086: f64 = (locals.var_cs_t * assign51490_e66085);
        (assign51490_e66086, (locals.var_cs_t * if 0.0 == 0.0 && ((assign51490_e66084) as f64).is_finite() && ((assign51490_e66084) as f64).fract() == 0.0 { if assign51490_e66084 == 0.0 { 0.0 } else { (assign51490_e66084 * ((assign51490_e66082).powf(assign51490_e66084 - 1.0) * locals.var_temp__blk936_dn5)) } } else { (assign51490_e66085 * (assign51490_e66084 * (locals.var_temp__blk936_dn5 / assign51490_e66082))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51490_e66084) as f64).is_finite() && ((assign51490_e66084) as f64).fract() == 0.0 { if assign51490_e66084 == 0.0 { 0.0 } else { (assign51490_e66084 * ((assign51490_e66082).powf(assign51490_e66084 - 1.0) * locals.var_temp__blk936_dn6)) } } else { (assign51490_e66085 * (assign51490_e66084 * (locals.var_temp__blk936_dn6 / assign51490_e66082))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51490_e66084) as f64).is_finite() && ((assign51490_e66084) as f64).fract() == 0.0 { if assign51490_e66084 == 0.0 { 0.0 } else { (assign51490_e66084 * ((assign51490_e66082).powf(assign51490_e66084 - 1.0) * locals.var_temp__blk936_dn7)) } } else { (assign51490_e66085 * (assign51490_e66084 * (locals.var_temp__blk936_dn7 / assign51490_e66082))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51490_e66084) as f64).is_finite() && ((assign51490_e66084) as f64).fract() == 0.0 { if assign51490_e66084 == 0.0 { 0.0 } else { (assign51490_e66084 * ((assign51490_e66082).powf(assign51490_e66084 - 1.0) * locals.var_temp__blk936_dn8)) } } else { (assign51490_e66085 * (assign51490_e66084 * (locals.var_temp__blk936_dn8 / assign51490_e66082))) }),)
    } else {
        (locals.var_gmobcssat__blk1379, locals.var_gmobcssat__blk1379_dn5, locals.var_gmobcssat__blk1379_dn6, locals.var_gmobcssat__blk1379_dn7, locals.var_gmobcssat__blk1379_dn8,)
    }
};
        locals.var_gmobcssat__blk1379 = assign51490_e66088;
        locals.var_gmobcssat__blk1379_dn5 = assign51490_e66088_d_n5;
        locals.var_gmobcssat__blk1379_dn6 = assign51490_e66088_d_n6;
        locals.var_gmobcssat__blk1379_dn7 = assign51490_e66088_d_n7;
        locals.var_gmobcssat__blk1379_dn8 = assign51490_e66088_d_n8;
        locals.var_gmobcssat__blk1379_rv = 0.0;

        let (assign51500_e66112, assign51500_e66112_d_n5, assign51500_e66112_d_n6, assign51500_e66112_d_n7, assign51500_e66112_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51500_e66099: f64 = (locals.var_alphasat__blk1377 - 1.0);
        let assign51500_e66103: f64 = (locals.var_temp__blk936 + 1.0);
        let assign51500_e66104: f64 = (1.0 / assign51500_e66103);
        let assign51500_e66105: f64 = (assign51500_e66099 + assign51500_e66104);
        let assign51500_e66106: f64 = (locals.var_thecs_t * assign51500_e66105);
        let assign51500_e66108: f64 = (assign51500_e66106 / locals.var_qbsat__blk1376);
        let assign51500_e66110: f64 = (assign51500_e66108 * locals.var_gmobcssat__blk1379);
        (assign51500_e66110, ((((((locals.var_thecs_t * (locals.var_alphasat__blk1377_dn5 + (-(locals.var_temp__blk936_dn5 / (assign51500_e66103 * assign51500_e66103))))) * locals.var_qbsat__blk1376) - (assign51500_e66106 * locals.var_qbsat__blk1376_dn5)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)) * locals.var_gmobcssat__blk1379) + (assign51500_e66108 * locals.var_gmobcssat__blk1379_dn5)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1377_dn6 + (-(locals.var_temp__blk936_dn6 / (assign51500_e66103 * assign51500_e66103))))) * locals.var_qbsat__blk1376) - (assign51500_e66106 * locals.var_qbsat__blk1376_dn6)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)) * locals.var_gmobcssat__blk1379) + (assign51500_e66108 * locals.var_gmobcssat__blk1379_dn6)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1377_dn7 + (-(locals.var_temp__blk936_dn7 / (assign51500_e66103 * assign51500_e66103))))) * locals.var_qbsat__blk1376) - (assign51500_e66106 * locals.var_qbsat__blk1376_dn7)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)) * locals.var_gmobcssat__blk1379) + (assign51500_e66108 * locals.var_gmobcssat__blk1379_dn7)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1377_dn8 + (-(locals.var_temp__blk936_dn8 / (assign51500_e66103 * assign51500_e66103))))) * locals.var_qbsat__blk1376) - (assign51500_e66106 * locals.var_qbsat__blk1376_dn8)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)) * locals.var_gmobcssat__blk1379) + (assign51500_e66108 * locals.var_gmobcssat__blk1379_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign51500_e66112;
        locals.var_temp2_dn5 = assign51500_e66112_d_n5;
        locals.var_temp2_dn6 = assign51500_e66112_d_n6;
        locals.var_temp2_dn7 = assign51500_e66112_d_n7;
        locals.var_temp2_dn8 = assign51500_e66112_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign51510_e66128, assign51510_e66128_d_n5, assign51510_e66128_d_n6, assign51510_e66128_d_n7, assign51510_e66128_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51510_e66122: f64 = (locals.var_ther_i * locals.var_rhob__blk1361);
        let assign51510_e66124: f64 = (assign51510_e66122 * locals.var_rhog__blk1362);
        let assign51510_e66126: f64 = (assign51510_e66124 * locals.var_qisat__blk1375);
        (assign51510_e66126, (((((locals.var_ther_i * locals.var_rhob__blk1361_dn5) * locals.var_rhog__blk1362) + (assign51510_e66122 * locals.var_rhog__blk1362_dn5)) * locals.var_qisat__blk1375) + (assign51510_e66124 * locals.var_qisat__blk1375_dn5)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn6) * locals.var_rhog__blk1362) + (assign51510_e66122 * locals.var_rhog__blk1362_dn6)) * locals.var_qisat__blk1375) + (assign51510_e66124 * locals.var_qisat__blk1375_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn7) * locals.var_rhog__blk1362) + (assign51510_e66122 * locals.var_rhog__blk1362_dn7)) * locals.var_qisat__blk1375) + (assign51510_e66124 * locals.var_qisat__blk1375_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn8) * locals.var_rhog__blk1362) + (assign51510_e66122 * locals.var_rhog__blk1362_dn8)) * locals.var_qisat__blk1375) + (assign51510_e66124 * locals.var_qisat__blk1375_dn8)),)
    } else {
        (locals.var_grsat__blk1380, locals.var_grsat__blk1380_dn5, locals.var_grsat__blk1380_dn6, locals.var_grsat__blk1380_dn7, locals.var_grsat__blk1380_dn8,)
    }
};
        locals.var_grsat__blk1380 = assign51510_e66128;
        locals.var_grsat__blk1380_dn5 = assign51510_e66128_d_n5;
        locals.var_grsat__blk1380_dn6 = assign51510_e66128_d_n6;
        locals.var_grsat__blk1380_dn7 = assign51510_e66128_d_n7;
        locals.var_grsat__blk1380_dn8 = assign51510_e66128_d_n8;
        locals.var_grsat__blk1380_rv = 0.0;

        let (assign51520_e66150, assign51520_e66150_d_n5, assign51520_e66150_d_n6, assign51520_e66150_d_n7, assign51520_e66150_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51520_e66140: f64 = (locals.var_ther_i * locals.var_rhob__blk1361);
        let assign51520_e66142: f64 = (assign51520_e66140 * locals.var_rhog__blk1362);
        let assign51520_e66144: f64 = (assign51520_e66142 * locals.var_alphasat__blk1377);
        let assign51520_e66145: f64 = (locals.var_temp1 - assign51520_e66144);
        let assign51520_e66147: f64 = (assign51520_e66145 / locals.var_temp2);
        let assign51520_e66148: f64 = (1.0 + assign51520_e66147);
        (assign51520_e66148, ((((locals.var_temp1_dn5 - (((((locals.var_ther_i * locals.var_rhob__blk1361_dn5) * locals.var_rhog__blk1362) + (assign51520_e66140 * locals.var_rhog__blk1362_dn5)) * locals.var_alphasat__blk1377) + (assign51520_e66142 * locals.var_alphasat__blk1377_dn5))) * locals.var_temp2) - (assign51520_e66145 * locals.var_temp2_dn5)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn6 - (((((locals.var_ther_i * locals.var_rhob__blk1361_dn6) * locals.var_rhog__blk1362) + (assign51520_e66140 * locals.var_rhog__blk1362_dn6)) * locals.var_alphasat__blk1377) + (assign51520_e66142 * locals.var_alphasat__blk1377_dn6))) * locals.var_temp2) - (assign51520_e66145 * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn7 - (((((locals.var_ther_i * locals.var_rhob__blk1361_dn7) * locals.var_rhog__blk1362) + (assign51520_e66140 * locals.var_rhog__blk1362_dn7)) * locals.var_alphasat__blk1377) + (assign51520_e66142 * locals.var_alphasat__blk1377_dn7))) * locals.var_temp2) - (assign51520_e66145 * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn8 - (((((locals.var_ther_i * locals.var_rhob__blk1361_dn8) * locals.var_rhog__blk1362) + (assign51520_e66140 * locals.var_rhog__blk1362_dn8)) * locals.var_alphasat__blk1377) + (assign51520_e66142 * locals.var_alphasat__blk1377_dn8))) * locals.var_temp2) - (assign51520_e66145 * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51520_e66150;
        locals.var_temp__blk936_dn5 = assign51520_e66150_d_n5;
        locals.var_temp__blk936_dn6 = assign51520_e66150_d_n6;
        locals.var_temp__blk936_dn7 = assign51520_e66150_d_n7;
        locals.var_temp__blk936_dn8 = assign51520_e66150_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign51530_e66153: f64 = if locals.var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1489 = assign51530_e66153;
        locals.var_guard1489_rv = 0.0;

        let (assign51540_e66173, assign51540_e66173_d_n5, assign51540_e66173_d_n6, assign51540_e66173_d_n7, assign51540_e66173_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) && (locals.var_guard1489 != 0.0)) {
        let assign51540_e66167: f64 = (2.0 * locals.var_temp__blk936);
        let assign51540_e66168: f64 = (assign51540_e66167).exp();
        let assign51540_e66169: f64 = (1.0 + assign51540_e66168);
        let assign51540_e66170: f64 = (assign51540_e66169).ln();
        let assign51540_e66171: f64 = (0.5 * assign51540_e66170);
        (assign51540_e66171, (0.5 * ((assign51540_e66168 * (2.0 * locals.var_temp__blk936_dn5)) / assign51540_e66169)), (0.5 * ((assign51540_e66168 * (2.0 * locals.var_temp__blk936_dn6)) / assign51540_e66169)), (0.5 * ((assign51540_e66168 * (2.0 * locals.var_temp__blk936_dn7)) / assign51540_e66169)), (0.5 * ((assign51540_e66168 * (2.0 * locals.var_temp__blk936_dn8)) / assign51540_e66169)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign51540_e66173;
        locals.var_temp1_dn5 = assign51540_e66173_d_n5;
        locals.var_temp1_dn6 = assign51540_e66173_d_n6;
        locals.var_temp1_dn7 = assign51540_e66173_d_n7;
        locals.var_temp1_dn8 = assign51540_e66173_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign51550_e66186, assign51550_e66186_d_n5, assign51550_e66186_d_n6, assign51550_e66186_d_n7, assign51550_e66186_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) && (locals.var_guard1489 == 0.0)) {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign51550_e66186;
        locals.var_temp1_dn5 = assign51550_e66186_d_n5;
        locals.var_temp1_dn6 = assign51550_e66186_d_n6;
        locals.var_temp1_dn7 = assign51550_e66186_d_n7;
        locals.var_temp1_dn8 = assign51550_e66186_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign51560_e66209, assign51560_e66209_d_n5, assign51560_e66209_d_n6, assign51560_e66209_d_n7, assign51560_e66209_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51560_e66195: f64 = (-locals.var_midphi0__blk1374);
        let assign51560_e66197: f64 = (assign51560_e66195 * locals.var_temp2);
        let assign51560_e66199: f64 = (assign51560_e66197 * locals.var_temp1);
        let assign51560_e66202: f64 = (1.0 + locals.var_gmobmusat__blk1378);
        let assign51560_e66204: f64 = (assign51560_e66202 + locals.var_gmobcssat__blk1379);
        let assign51560_e66206: f64 = (assign51560_e66204 + locals.var_grsat__blk1380);
        let assign51560_e66207: f64 = (assign51560_e66199 / assign51560_e66206);
        (assign51560_e66207, ((((((((-locals.var_midphi0__blk1374_dn5) * locals.var_temp2) + (assign51560_e66195 * locals.var_temp2_dn5)) * locals.var_temp1) + (assign51560_e66197 * locals.var_temp1_dn5)) * assign51560_e66206) - (assign51560_e66199 * ((locals.var_gmobmusat__blk1378_dn5 + locals.var_gmobcssat__blk1379_dn5) + locals.var_grsat__blk1380_dn5))) / (assign51560_e66206 * assign51560_e66206)), ((((((((-locals.var_midphi0__blk1374_dn6) * locals.var_temp2) + (assign51560_e66195 * locals.var_temp2_dn6)) * locals.var_temp1) + (assign51560_e66197 * locals.var_temp1_dn6)) * assign51560_e66206) - (assign51560_e66199 * ((locals.var_gmobmusat__blk1378_dn6 + locals.var_gmobcssat__blk1379_dn6) + locals.var_grsat__blk1380_dn6))) / (assign51560_e66206 * assign51560_e66206)), ((((((((-locals.var_midphi0__blk1374_dn7) * locals.var_temp2) + (assign51560_e66195 * locals.var_temp2_dn7)) * locals.var_temp1) + (assign51560_e66197 * locals.var_temp1_dn7)) * assign51560_e66206) - (assign51560_e66199 * ((locals.var_gmobmusat__blk1378_dn7 + locals.var_gmobcssat__blk1379_dn7) + locals.var_grsat__blk1380_dn7))) / (assign51560_e66206 * assign51560_e66206)), ((((((((-locals.var_midphi0__blk1374_dn8) * locals.var_temp2) + (assign51560_e66195 * locals.var_temp2_dn8)) * locals.var_temp1) + (assign51560_e66197 * locals.var_temp1_dn8)) * assign51560_e66206) - (assign51560_e66199 * ((locals.var_gmobmusat__blk1378_dn8 + locals.var_gmobcssat__blk1379_dn8) + locals.var_grsat__blk1380_dn8))) / (assign51560_e66206 * assign51560_e66206)),)
    } else {
        (locals.var_delta_gmob__blk1381, locals.var_delta_gmob__blk1381_dn5, locals.var_delta_gmob__blk1381_dn6, locals.var_delta_gmob__blk1381_dn7, locals.var_delta_gmob__blk1381_dn8,)
    }
};
        locals.var_delta_gmob__blk1381 = assign51560_e66209;
        locals.var_delta_gmob__blk1381_dn5 = assign51560_e66209_d_n5;
        locals.var_delta_gmob__blk1381_dn6 = assign51560_e66209_d_n6;
        locals.var_delta_gmob__blk1381_dn7 = assign51560_e66209_d_n7;
        locals.var_delta_gmob__blk1381_dn8 = assign51560_e66209_d_n8;
        locals.var_delta_gmob__blk1381_rv = 0.0;

        let (assign51570_e66232, assign51570_e66232_d_n5, assign51570_e66232_d_n6, assign51570_e66232_d_n7, assign51570_e66232_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51570_e66224: f64 = (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381);
        let assign51570_e66225: f64 = (1.0 + assign51570_e66224);
        let assign51570_e66226: f64 = (assign51570_e66225).sqrt();
        let assign51570_e66227: f64 = (1.0 + assign51570_e66226);
        let assign51570_e66228: f64 = (locals.var_delta_gmob__blk1381 / assign51570_e66227);
        let assign51570_e66229: f64 = (1.0 + assign51570_e66228);
        let assign51570_e66230: f64 = (locals.var_x_inf0__blk1373 * assign51570_e66229);
        (assign51570_e66230, ((locals.var_x_inf0__blk1373_dn5 * assign51570_e66229) + (locals.var_x_inf0__blk1373 * (((locals.var_delta_gmob__blk1381_dn5 * assign51570_e66227) - (locals.var_delta_gmob__blk1381 * (((locals.var_delta_gmob__blk1381_dn5 * locals.var_delta_gmob__blk1381) + (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381_dn5)) / (2.0 * assign51570_e66226)))) / (assign51570_e66227 * assign51570_e66227)))), ((locals.var_x_inf0__blk1373_dn6 * assign51570_e66229) + (locals.var_x_inf0__blk1373 * (((locals.var_delta_gmob__blk1381_dn6 * assign51570_e66227) - (locals.var_delta_gmob__blk1381 * (((locals.var_delta_gmob__blk1381_dn6 * locals.var_delta_gmob__blk1381) + (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381_dn6)) / (2.0 * assign51570_e66226)))) / (assign51570_e66227 * assign51570_e66227)))), ((locals.var_x_inf0__blk1373_dn7 * assign51570_e66229) + (locals.var_x_inf0__blk1373 * (((locals.var_delta_gmob__blk1381_dn7 * assign51570_e66227) - (locals.var_delta_gmob__blk1381 * (((locals.var_delta_gmob__blk1381_dn7 * locals.var_delta_gmob__blk1381) + (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381_dn7)) / (2.0 * assign51570_e66226)))) / (assign51570_e66227 * assign51570_e66227)))), ((locals.var_x_inf0__blk1373_dn8 * assign51570_e66229) + (locals.var_x_inf0__blk1373 * (((locals.var_delta_gmob__blk1381_dn8 * assign51570_e66227) - (locals.var_delta_gmob__blk1381 * (((locals.var_delta_gmob__blk1381_dn8 * locals.var_delta_gmob__blk1381) + (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381_dn8)) / (2.0 * assign51570_e66226)))) / (assign51570_e66227 * assign51570_e66227)))),)
    } else {
        (locals.var_x_inf__blk1382, locals.var_x_inf__blk1382_dn5, locals.var_x_inf__blk1382_dn6, locals.var_x_inf__blk1382_dn7, locals.var_x_inf__blk1382_dn8,)
    }
};
        locals.var_x_inf__blk1382 = assign51570_e66232;
        locals.var_x_inf__blk1382_dn5 = assign51570_e66232_d_n5;
        locals.var_x_inf__blk1382_dn6 = assign51570_e66232_d_n6;
        locals.var_x_inf__blk1382_dn7 = assign51570_e66232_d_n7;
        locals.var_x_inf__blk1382_dn8 = assign51570_e66232_d_n8;
        locals.var_x_inf__blk1382_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_45(
        locals: &mut StampLocals,
    ) {
        let (assign51580_e66243, assign51580_e66243_d_n5, assign51580_e66243_d_n6, assign51580_e66243_d_n7, assign51580_e66243_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 == 0.0)) {
        (locals.var_x_inf0__blk1373, locals.var_x_inf0__blk1373_dn5, locals.var_x_inf0__blk1373_dn6, locals.var_x_inf0__blk1373_dn7, locals.var_x_inf0__blk1373_dn8,)
    } else {
        (locals.var_x_inf__blk1382, locals.var_x_inf__blk1382_dn5, locals.var_x_inf__blk1382_dn6, locals.var_x_inf__blk1382_dn7, locals.var_x_inf__blk1382_dn8,)
    }
};
        locals.var_x_inf__blk1382 = assign51580_e66243;
        locals.var_x_inf__blk1382_dn5 = assign51580_e66243_d_n5;
        locals.var_x_inf__blk1382_dn6 = assign51580_e66243_d_n6;
        locals.var_x_inf__blk1382_dn7 = assign51580_e66243_d_n7;
        locals.var_x_inf__blk1382_dn8 = assign51580_e66243_d_n8;
        locals.var_x_inf__blk1382_rv = 0.0;

        let (assign51590_e66257, assign51590_e66257_d_n5, assign51590_e66257_d_n6, assign51590_e66257_d_n7, assign51590_e66257_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51590_e66251: f64 = (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371);
        let assign51590_e66253: f64 = (assign51590_e66251 * locals.var_x_inf__blk1382);
        let assign51590_e66255: f64 = (assign51590_e66253 * 0.7071067811865475);
        (assign51590_e66255, (((((locals.var_phit1__blk1322_dn5 * locals.var_thesat1__blk1371) + (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371_dn5)) * locals.var_x_inf__blk1382) + (assign51590_e66251 * locals.var_x_inf__blk1382_dn5)) * 0.7071067811865475), (((((locals.var_phit1__blk1322_dn6 * locals.var_thesat1__blk1371) + (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371_dn6)) * locals.var_x_inf__blk1382) + (assign51590_e66251 * locals.var_x_inf__blk1382_dn6)) * 0.7071067811865475), (((((locals.var_phit1__blk1322_dn7 * locals.var_thesat1__blk1371) + (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371_dn7)) * locals.var_x_inf__blk1382) + (assign51590_e66251 * locals.var_x_inf__blk1382_dn7)) * 0.7071067811865475), (((((locals.var_phit1__blk1322_dn8 * locals.var_thesat1__blk1371) + (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371_dn8)) * locals.var_x_inf__blk1382) + (assign51590_e66251 * locals.var_x_inf__blk1382_dn8)) * 0.7071067811865475),)
    } else {
        (locals.var_ysat__blk1383, locals.var_ysat__blk1383_dn5, locals.var_ysat__blk1383_dn6, locals.var_ysat__blk1383_dn7, locals.var_ysat__blk1383_dn8,)
    }
};
        locals.var_ysat__blk1383 = assign51590_e66257;
        locals.var_ysat__blk1383_dn5 = assign51590_e66257_d_n5;
        locals.var_ysat__blk1383_dn6 = assign51590_e66257_d_n6;
        locals.var_ysat__blk1383_dn7 = assign51590_e66257_d_n7;
        locals.var_ysat__blk1383_dn8 = assign51590_e66257_d_n8;
        locals.var_ysat__blk1383_rv = 0.0;

        let assign51600_e66260: f64 = (-1.0);
        let assign51600_e66261: f64 = if locals.var_chnl_type == assign51600_e66260 { 1.0 } else { 0.0 };
        locals.var_guard1490 = assign51600_e66261;
        locals.var_guard1490_rv = 0.0;

        let (assign51610_e66276, assign51610_e66276_d_n5, assign51610_e66276_d_n6, assign51610_e66276_d_n7, assign51610_e66276_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1490 != 0.0)) {
        let assign51610_e66272: f64 = (1.0 + locals.var_ysat__blk1383);
        let assign51610_e66273: f64 = (assign51610_e66272).sqrt();
        let assign51610_e66274: f64 = (locals.var_ysat__blk1383 / assign51610_e66273);
        (assign51610_e66274, (((locals.var_ysat__blk1383_dn5 * assign51610_e66273) - (locals.var_ysat__blk1383 * (locals.var_ysat__blk1383_dn5 / (2.0 * assign51610_e66273)))) / (assign51610_e66273 * assign51610_e66273)), (((locals.var_ysat__blk1383_dn6 * assign51610_e66273) - (locals.var_ysat__blk1383 * (locals.var_ysat__blk1383_dn6 / (2.0 * assign51610_e66273)))) / (assign51610_e66273 * assign51610_e66273)), (((locals.var_ysat__blk1383_dn7 * assign51610_e66273) - (locals.var_ysat__blk1383 * (locals.var_ysat__blk1383_dn7 / (2.0 * assign51610_e66273)))) / (assign51610_e66273 * assign51610_e66273)), (((locals.var_ysat__blk1383_dn8 * assign51610_e66273) - (locals.var_ysat__blk1383 * (locals.var_ysat__blk1383_dn8 / (2.0 * assign51610_e66273)))) / (assign51610_e66273 * assign51610_e66273)),)
    } else {
        (locals.var_ysat__blk1383, locals.var_ysat__blk1383_dn5, locals.var_ysat__blk1383_dn6, locals.var_ysat__blk1383_dn7, locals.var_ysat__blk1383_dn8,)
    }
};
        locals.var_ysat__blk1383 = assign51610_e66276;
        locals.var_ysat__blk1383_dn5 = assign51610_e66276_d_n5;
        locals.var_ysat__blk1383_dn6 = assign51610_e66276_d_n6;
        locals.var_ysat__blk1383_dn7 = assign51610_e66276_d_n7;
        locals.var_ysat__blk1383_dn8 = assign51610_e66276_d_n8;
        locals.var_ysat__blk1383_rv = 0.0;

        let (assign51620_e66293, assign51620_e66293_d_n5, assign51620_e66293_d_n6, assign51620_e66293_d_n7, assign51620_e66293_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51620_e66287: f64 = (4.0 * locals.var_ysat__blk1383);
        let assign51620_e66288: f64 = (1.0 + assign51620_e66287);
        let assign51620_e66289: f64 = (assign51620_e66288).sqrt();
        let assign51620_e66290: f64 = (1.0 + assign51620_e66289);
        let assign51620_e66291: f64 = (2.0 / assign51620_e66290);
        (assign51620_e66291, (-((2.0 * ((4.0 * locals.var_ysat__blk1383_dn5) / (2.0 * assign51620_e66289))) / (assign51620_e66290 * assign51620_e66290))), (-((2.0 * ((4.0 * locals.var_ysat__blk1383_dn6) / (2.0 * assign51620_e66289))) / (assign51620_e66290 * assign51620_e66290))), (-((2.0 * ((4.0 * locals.var_ysat__blk1383_dn7) / (2.0 * assign51620_e66289))) / (assign51620_e66290 * assign51620_e66290))), (-((2.0 * ((4.0 * locals.var_ysat__blk1383_dn8) / (2.0 * assign51620_e66289))) / (assign51620_e66290 * assign51620_e66290))),)
    } else {
        (locals.var_za__blk1384, locals.var_za__blk1384_dn5, locals.var_za__blk1384_dn6, locals.var_za__blk1384_dn7, locals.var_za__blk1384_dn8,)
    }
};
        locals.var_za__blk1384 = assign51620_e66293;
        locals.var_za__blk1384_dn5 = assign51620_e66293_d_n5;
        locals.var_za__blk1384_dn6 = assign51620_e66293_d_n6;
        locals.var_za__blk1384_dn7 = assign51620_e66293_d_n7;
        locals.var_za__blk1384_dn8 = assign51620_e66293_d_n8;
        locals.var_za__blk1384_rv = 0.0;

        let (assign51630_e66303, assign51630_e66303_d_n5, assign51630_e66303_d_n6, assign51630_e66303_d_n7, assign51630_e66303_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51630_e66301: f64 = (locals.var_za__blk1384 * locals.var_ysat__blk1383);
        (assign51630_e66301, ((locals.var_za__blk1384_dn5 * locals.var_ysat__blk1383) + (locals.var_za__blk1384 * locals.var_ysat__blk1383_dn5)), ((locals.var_za__blk1384_dn6 * locals.var_ysat__blk1383) + (locals.var_za__blk1384 * locals.var_ysat__blk1383_dn6)), ((locals.var_za__blk1384_dn7 * locals.var_ysat__blk1383) + (locals.var_za__blk1384 * locals.var_ysat__blk1383_dn7)), ((locals.var_za__blk1384_dn8 * locals.var_ysat__blk1383) + (locals.var_za__blk1384 * locals.var_ysat__blk1383_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51630_e66303;
        locals.var_temp__blk936_dn5 = assign51630_e66303_d_n5;
        locals.var_temp__blk936_dn6 = assign51630_e66303_d_n6;
        locals.var_temp__blk936_dn7 = assign51630_e66303_d_n7;
        locals.var_temp__blk936_dn8 = assign51630_e66303_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign51640_e66335, assign51640_e66335_d_n5, assign51640_e66335_d_n6, assign51640_e66335_d_n7, assign51640_e66335_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51640_e66311: f64 = (locals.var_x_inf__blk1382 * locals.var_za__blk1384);
        let assign51640_e66315: f64 = (0.86 * locals.var_temp__blk936);
        let assign51640_e66319: f64 = (locals.var_temp__blk936 * locals.var_za__blk1384);
        let assign51640_e66320: f64 = (1.0 - assign51640_e66319);
        let assign51640_e66321: f64 = (assign51640_e66315 * assign51640_e66320);
        let assign51640_e66325: f64 = (4.0 * locals.var_temp__blk936);
        let assign51640_e66327: f64 = (assign51640_e66325 * locals.var_temp__blk936);
        let assign51640_e66329: f64 = (assign51640_e66327 * locals.var_za__blk1384);
        let assign51640_e66330: f64 = (1.0 + assign51640_e66329);
        let assign51640_e66331: f64 = (assign51640_e66321 / assign51640_e66330);
        let assign51640_e66332: f64 = (1.0 + assign51640_e66331);
        let assign51640_e66333: f64 = (assign51640_e66311 * assign51640_e66332);
        (assign51640_e66333, ((((locals.var_x_inf__blk1382_dn5 * locals.var_za__blk1384) + (locals.var_x_inf__blk1382 * locals.var_za__blk1384_dn5)) * assign51640_e66332) + (assign51640_e66311 * ((((((0.86 * locals.var_temp__blk936_dn5) * assign51640_e66320) + (assign51640_e66315 * (-((locals.var_temp__blk936_dn5 * locals.var_za__blk1384) + (locals.var_temp__blk936 * locals.var_za__blk1384_dn5))))) * assign51640_e66330) - (assign51640_e66321 * (((((4.0 * locals.var_temp__blk936_dn5) * locals.var_temp__blk936) + (assign51640_e66325 * locals.var_temp__blk936_dn5)) * locals.var_za__blk1384) + (assign51640_e66327 * locals.var_za__blk1384_dn5)))) / (assign51640_e66330 * assign51640_e66330)))), ((((locals.var_x_inf__blk1382_dn6 * locals.var_za__blk1384) + (locals.var_x_inf__blk1382 * locals.var_za__blk1384_dn6)) * assign51640_e66332) + (assign51640_e66311 * ((((((0.86 * locals.var_temp__blk936_dn6) * assign51640_e66320) + (assign51640_e66315 * (-((locals.var_temp__blk936_dn6 * locals.var_za__blk1384) + (locals.var_temp__blk936 * locals.var_za__blk1384_dn6))))) * assign51640_e66330) - (assign51640_e66321 * (((((4.0 * locals.var_temp__blk936_dn6) * locals.var_temp__blk936) + (assign51640_e66325 * locals.var_temp__blk936_dn6)) * locals.var_za__blk1384) + (assign51640_e66327 * locals.var_za__blk1384_dn6)))) / (assign51640_e66330 * assign51640_e66330)))), ((((locals.var_x_inf__blk1382_dn7 * locals.var_za__blk1384) + (locals.var_x_inf__blk1382 * locals.var_za__blk1384_dn7)) * assign51640_e66332) + (assign51640_e66311 * ((((((0.86 * locals.var_temp__blk936_dn7) * assign51640_e66320) + (assign51640_e66315 * (-((locals.var_temp__blk936_dn7 * locals.var_za__blk1384) + (locals.var_temp__blk936 * locals.var_za__blk1384_dn7))))) * assign51640_e66330) - (assign51640_e66321 * (((((4.0 * locals.var_temp__blk936_dn7) * locals.var_temp__blk936) + (assign51640_e66325 * locals.var_temp__blk936_dn7)) * locals.var_za__blk1384) + (assign51640_e66327 * locals.var_za__blk1384_dn7)))) / (assign51640_e66330 * assign51640_e66330)))), ((((locals.var_x_inf__blk1382_dn8 * locals.var_za__blk1384) + (locals.var_x_inf__blk1382 * locals.var_za__blk1384_dn8)) * assign51640_e66332) + (assign51640_e66311 * ((((((0.86 * locals.var_temp__blk936_dn8) * assign51640_e66320) + (assign51640_e66315 * (-((locals.var_temp__blk936_dn8 * locals.var_za__blk1384) + (locals.var_temp__blk936 * locals.var_za__blk1384_dn8))))) * assign51640_e66330) - (assign51640_e66321 * (((((4.0 * locals.var_temp__blk936_dn8) * locals.var_temp__blk936) + (assign51640_e66325 * locals.var_temp__blk936_dn8)) * locals.var_za__blk1384) + (assign51640_e66327 * locals.var_za__blk1384_dn8)))) / (assign51640_e66330 * assign51640_e66330)))),)
    } else {
        (locals.var_x_0__blk1385, locals.var_x_0__blk1385_dn5, locals.var_x_0__blk1385_dn6, locals.var_x_0__blk1385_dn7, locals.var_x_0__blk1385_dn8,)
    }
};
        locals.var_x_0__blk1385 = assign51640_e66335;
        locals.var_x_0__blk1385_dn5 = assign51640_e66335_d_n5;
        locals.var_x_0__blk1385_dn6 = assign51640_e66335_d_n6;
        locals.var_x_0__blk1385_dn7 = assign51640_e66335_d_n7;
        locals.var_x_0__blk1385_dn8 = assign51640_e66335_d_n8;
        locals.var_x_0__blk1385_rv = 0.0;

        let (assign51650_e66345, assign51650_e66345_d_n5, assign51650_e66345_d_n6, assign51650_e66345_d_n7, assign51650_e66345_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51650_e66343: f64 = (0.99 * locals.var_x_0__blk1385);
        (assign51650_e66343, (0.99 * locals.var_x_0__blk1385_dn5), (0.99 * locals.var_x_0__blk1385_dn6), (0.99 * locals.var_x_0__blk1385_dn7), (0.99 * locals.var_x_0__blk1385_dn8),)
    } else {
        (locals.var_x_sat__blk1386, locals.var_x_sat__blk1386_dn5, locals.var_x_sat__blk1386_dn6, locals.var_x_sat__blk1386_dn7, locals.var_x_sat__blk1386_dn8,)
    }
};
        locals.var_x_sat__blk1386 = assign51650_e66345;
        locals.var_x_sat__blk1386_dn5 = assign51650_e66345_d_n5;
        locals.var_x_sat__blk1386_dn6 = assign51650_e66345_d_n6;
        locals.var_x_sat__blk1386_dn7 = assign51650_e66345_d_n7;
        locals.var_x_sat__blk1386_dn8 = assign51650_e66345_d_n8;
        locals.var_x_sat__blk1386_rv = 0.0;

        let (assign51660_e66363, assign51660_e66363_d_n5, assign51660_e66363_d_n6, assign51660_e66363_d_n7, assign51660_e66363_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51660_e66355: f64 = (2.0 * locals.var_asat__blk1372);
        let assign51660_e66356: f64 = (locals.var_x_sat__blk1386 - assign51660_e66355);
        let assign51660_e66357: f64 = (locals.var_x_sat__blk1386 * assign51660_e66356);
        let assign51660_e66359: f64 = (assign51660_e66357 * locals.var_inv_gf2__blk1324);
        let assign51660_e66361: f64 = (assign51660_e66359 / locals.var_ds__blk1353);
        (assign51660_e66361, (((((((locals.var_x_sat__blk1386_dn5 * assign51660_e66356) + (locals.var_x_sat__blk1386 * (locals.var_x_sat__blk1386_dn5 - (2.0 * locals.var_asat__blk1372_dn5)))) * locals.var_inv_gf2__blk1324) + (assign51660_e66357 * locals.var_inv_gf2__blk1324_dn5)) * locals.var_ds__blk1353) - (assign51660_e66359 * locals.var_ds__blk1353_dn5)) / (locals.var_ds__blk1353 * locals.var_ds__blk1353)), (((((((locals.var_x_sat__blk1386_dn6 * assign51660_e66356) + (locals.var_x_sat__blk1386 * (locals.var_x_sat__blk1386_dn6 - (2.0 * locals.var_asat__blk1372_dn6)))) * locals.var_inv_gf2__blk1324) + (assign51660_e66357 * locals.var_inv_gf2__blk1324_dn6)) * locals.var_ds__blk1353) - (assign51660_e66359 * locals.var_ds__blk1353_dn6)) / (locals.var_ds__blk1353 * locals.var_ds__blk1353)), (((((((locals.var_x_sat__blk1386_dn7 * assign51660_e66356) + (locals.var_x_sat__blk1386 * (locals.var_x_sat__blk1386_dn7 - (2.0 * locals.var_asat__blk1372_dn7)))) * locals.var_inv_gf2__blk1324) + (assign51660_e66357 * locals.var_inv_gf2__blk1324_dn7)) * locals.var_ds__blk1353) - (assign51660_e66359 * locals.var_ds__blk1353_dn7)) / (locals.var_ds__blk1353 * locals.var_ds__blk1353)), (((((((locals.var_x_sat__blk1386_dn8 * assign51660_e66356) + (locals.var_x_sat__blk1386 * (locals.var_x_sat__blk1386_dn8 - (2.0 * locals.var_asat__blk1372_dn8)))) * locals.var_inv_gf2__blk1324) + (assign51660_e66357 * locals.var_inv_gf2__blk1324_dn8)) * locals.var_ds__blk1353) - (assign51660_e66359 * locals.var_ds__blk1353_dn8)) / (locals.var_ds__blk1353 * locals.var_ds__blk1353)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51660_e66363;
        locals.var_temp__blk936_dn5 = assign51660_e66363_d_n5;
        locals.var_temp__blk936_dn6 = assign51660_e66363_d_n6;
        locals.var_temp__blk936_dn7 = assign51660_e66363_d_n7;
        locals.var_temp__blk936_dn8 = assign51660_e66363_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign51670_e66385, assign51670_e66385_d_n5, assign51670_e66385_d_n6, assign51670_e66385_d_n7, assign51670_e66385_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51670_e66374: f64 = (-0.99);
        let (assign51670_e66379, assign51670_e66379_d_n5, assign51670_e66379_d_n6, assign51670_e66379_d_n7, assign51670_e66379_d_n8,) = {
            if (locals.var_temp__blk936 > assign51670_e66374) {
                (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
            } else {
                let assign51670_e66378: f64 = (-0.99);
                (assign51670_e66378, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign51670_e66380: f64 = (1.0 + assign51670_e66379);
        let assign51670_e66381: f64 = (assign51670_e66380).ln();
        let assign51670_e66382: f64 = (locals.var_x_sat__blk1386 - assign51670_e66381);
        let assign51670_e66383: f64 = (locals.var_phit1__blk1322 * assign51670_e66382);
        (assign51670_e66383, ((locals.var_phit1__blk1322_dn5 * assign51670_e66382) + (locals.var_phit1__blk1322 * (locals.var_x_sat__blk1386_dn5 - (assign51670_e66379_d_n5 / assign51670_e66380)))), ((locals.var_phit1__blk1322_dn6 * assign51670_e66382) + (locals.var_phit1__blk1322 * (locals.var_x_sat__blk1386_dn6 - (assign51670_e66379_d_n6 / assign51670_e66380)))), ((locals.var_phit1__blk1322_dn7 * assign51670_e66382) + (locals.var_phit1__blk1322 * (locals.var_x_sat__blk1386_dn7 - (assign51670_e66379_d_n7 / assign51670_e66380)))), ((locals.var_phit1__blk1322_dn8 * assign51670_e66382) + (locals.var_phit1__blk1322 * (locals.var_x_sat__blk1386_dn8 - (assign51670_e66379_d_n8 / assign51670_e66380)))),)
    } else {
        (locals.var_v_dsat__blk1387, locals.var_v_dsat__blk1387_dn5, locals.var_v_dsat__blk1387_dn6, locals.var_v_dsat__blk1387_dn7, locals.var_v_dsat__blk1387_dn8,)
    }
};
        locals.var_v_dsat__blk1387 = assign51670_e66385;
        locals.var_v_dsat__blk1387_dn5 = assign51670_e66385_d_n5;
        locals.var_v_dsat__blk1387_dn6 = assign51670_e66385_d_n6;
        locals.var_v_dsat__blk1387_dn7 = assign51670_e66385_d_n7;
        locals.var_v_dsat__blk1387_dn8 = assign51670_e66385_d_n8;
        locals.var_v_dsat__blk1387_rv = 0.0;

        let (assign51680_e66394, assign51680_e66394_d_n5, assign51680_e66394_d_n6, assign51680_e66394_d_n7, assign51680_e66394_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 == 0.0)) {
        (locals.var_vdsat_lim__blk1370, locals.var_vdsat_lim__blk1370_dn5, locals.var_vdsat_lim__blk1370_dn6, locals.var_vdsat_lim__blk1370_dn7, locals.var_vdsat_lim__blk1370_dn8,)
    } else {
        (locals.var_v_dsat__blk1387, locals.var_v_dsat__blk1387_dn5, locals.var_v_dsat__blk1387_dn6, locals.var_v_dsat__blk1387_dn7, locals.var_v_dsat__blk1387_dn8,)
    }
};
        locals.var_v_dsat__blk1387 = assign51680_e66394;
        locals.var_v_dsat__blk1387_dn5 = assign51680_e66394_d_n5;
        locals.var_v_dsat__blk1387_dn6 = assign51680_e66394_d_n6;
        locals.var_v_dsat__blk1387_dn7 = assign51680_e66394_d_n7;
        locals.var_v_dsat__blk1387_dn8 = assign51680_e66394_d_n8;
        locals.var_v_dsat__blk1387_rv = 0.0;

        let (assign51690_e66402, assign51690_e66402_d_n5, assign51690_e66402_d_n6, assign51690_e66402_d_n7, assign51690_e66402_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51690_e66400: f64 = (1.0 + locals.var_arloc__blk1303);
        (assign51690_e66400, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51690_e66402;
        locals.var_temp__blk936_dn5 = assign51690_e66402_d_n5;
        locals.var_temp__blk936_dn6 = assign51690_e66402_d_n6;
        locals.var_temp__blk936_dn7 = assign51690_e66402_d_n7;
        locals.var_temp__blk936_dn8 = assign51690_e66402_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign51700_e66413, assign51700_e66413_d_n5, assign51700_e66413_d_n6, assign51700_e66413_d_n7, assign51700_e66413_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51700_e66407: f64 = (locals.var_temp__blk936).sqrt();
        let assign51700_e66409: f64 = (assign51700_e66407 * locals.var_v_ds);
        let assign51700_e66411: f64 = (assign51700_e66409 / locals.var_v_dsat__blk1387);
        (assign51700_e66411, (((((locals.var_temp__blk936_dn5 / (2.0 * assign51700_e66407)) * locals.var_v_ds) * locals.var_v_dsat__blk1387) - (assign51700_e66409 * locals.var_v_dsat__blk1387_dn5)) / (locals.var_v_dsat__blk1387 * locals.var_v_dsat__blk1387)), ((((((locals.var_temp__blk936_dn6 / (2.0 * assign51700_e66407)) * locals.var_v_ds) + (assign51700_e66407 * locals.var_v_ds_dn6)) * locals.var_v_dsat__blk1387) - (assign51700_e66409 * locals.var_v_dsat__blk1387_dn6)) / (locals.var_v_dsat__blk1387 * locals.var_v_dsat__blk1387)), ((((((locals.var_temp__blk936_dn7 / (2.0 * assign51700_e66407)) * locals.var_v_ds) + (assign51700_e66407 * locals.var_v_ds_dn7)) * locals.var_v_dsat__blk1387) - (assign51700_e66409 * locals.var_v_dsat__blk1387_dn7)) / (locals.var_v_dsat__blk1387 * locals.var_v_dsat__blk1387)), (((((locals.var_temp__blk936_dn8 / (2.0 * assign51700_e66407)) * locals.var_v_ds) * locals.var_v_dsat__blk1387) - (assign51700_e66409 * locals.var_v_dsat__blk1387_dn8)) / (locals.var_v_dsat__blk1387 * locals.var_v_dsat__blk1387)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign51700_e66413;
        locals.var_temp1_dn5 = assign51700_e66413_d_n5;
        locals.var_temp1_dn6 = assign51700_e66413_d_n6;
        locals.var_temp1_dn7 = assign51700_e66413_d_n7;
        locals.var_temp1_dn8 = assign51700_e66413_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign51710_e66423, assign51710_e66423_d_n5, assign51710_e66423_d_n6, assign51710_e66423_d_n7, assign51710_e66423_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51710_e66419: f64 = (locals.var_temp1 * locals.var_temp1);
        let assign51710_e66421: f64 = (assign51710_e66419 + locals.var_temp__blk936);
        (assign51710_e66421, (((locals.var_temp1_dn5 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn5)) + locals.var_temp__blk936_dn5), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk936_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk936_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign51710_e66423;
        locals.var_temp2_dn5 = assign51710_e66423_d_n5;
        locals.var_temp2_dn6 = assign51710_e66423_d_n6;
        locals.var_temp2_dn7 = assign51710_e66423_d_n7;
        locals.var_temp2_dn8 = assign51710_e66423_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign51720_e66431, assign51720_e66431_d_n5, assign51720_e66431_d_n6, assign51720_e66431_d_n7, assign51720_e66431_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51720_e66429: f64 = (2.0 * locals.var_temp1);
        (assign51720_e66429, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51720_e66431;
        locals.var_temp__blk936_dn5 = assign51720_e66431_d_n5;
        locals.var_temp__blk936_dn6 = assign51720_e66431_d_n6;
        locals.var_temp__blk936_dn7 = assign51720_e66431_d_n7;
        locals.var_temp__blk936_dn8 = assign51720_e66431_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign51730_e66449, assign51730_e66449_d_n5, assign51730_e66449_d_n6, assign51730_e66449_d_n7, assign51730_e66449_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51730_e66437: f64 = (locals.var_v_dsat__blk1387 * locals.var_temp__blk936);
        let assign51730_e66440: f64 = (locals.var_temp2 - locals.var_temp__blk936);
        let assign51730_e66441: f64 = (assign51730_e66440).sqrt();
        let assign51730_e66444: f64 = (locals.var_temp2 + locals.var_temp__blk936);
        let assign51730_e66445: f64 = (assign51730_e66444).sqrt();
        let assign51730_e66446: f64 = (assign51730_e66441 + assign51730_e66445);
        let assign51730_e66447: f64 = (assign51730_e66437 / assign51730_e66446);
        (assign51730_e66447, (((((locals.var_v_dsat__blk1387_dn5 * locals.var_temp__blk936) + (locals.var_v_dsat__blk1387 * locals.var_temp__blk936_dn5)) * assign51730_e66446) - (assign51730_e66437 * (((locals.var_temp2_dn5 - locals.var_temp__blk936_dn5) / (2.0 * assign51730_e66441)) + ((locals.var_temp2_dn5 + locals.var_temp__blk936_dn5) / (2.0 * assign51730_e66445))))) / (assign51730_e66446 * assign51730_e66446)), (((((locals.var_v_dsat__blk1387_dn6 * locals.var_temp__blk936) + (locals.var_v_dsat__blk1387 * locals.var_temp__blk936_dn6)) * assign51730_e66446) - (assign51730_e66437 * (((locals.var_temp2_dn6 - locals.var_temp__blk936_dn6) / (2.0 * assign51730_e66441)) + ((locals.var_temp2_dn6 + locals.var_temp__blk936_dn6) / (2.0 * assign51730_e66445))))) / (assign51730_e66446 * assign51730_e66446)), (((((locals.var_v_dsat__blk1387_dn7 * locals.var_temp__blk936) + (locals.var_v_dsat__blk1387 * locals.var_temp__blk936_dn7)) * assign51730_e66446) - (assign51730_e66437 * (((locals.var_temp2_dn7 - locals.var_temp__blk936_dn7) / (2.0 * assign51730_e66441)) + ((locals.var_temp2_dn7 + locals.var_temp__blk936_dn7) / (2.0 * assign51730_e66445))))) / (assign51730_e66446 * assign51730_e66446)), (((((locals.var_v_dsat__blk1387_dn8 * locals.var_temp__blk936) + (locals.var_v_dsat__blk1387 * locals.var_temp__blk936_dn8)) * assign51730_e66446) - (assign51730_e66437 * (((locals.var_temp2_dn8 - locals.var_temp__blk936_dn8) / (2.0 * assign51730_e66441)) + ((locals.var_temp2_dn8 + locals.var_temp__blk936_dn8) / (2.0 * assign51730_e66445))))) / (assign51730_e66446 * assign51730_e66446)),)
    } else {
        (locals.var_vdse__blk1388, locals.var_vdse__blk1388_dn5, locals.var_vdse__blk1388_dn6, locals.var_vdse__blk1388_dn7, locals.var_vdse__blk1388_dn8,)
    }
};
        locals.var_vdse__blk1388 = assign51730_e66449;
        locals.var_vdse__blk1388_dn5 = assign51730_e66449_d_n5;
        locals.var_vdse__blk1388_dn6 = assign51730_e66449_d_n6;
        locals.var_vdse__blk1388_dn7 = assign51730_e66449_d_n7;
        locals.var_vdse__blk1388_dn8 = assign51730_e66449_d_n8;
        locals.var_vdse__blk1388_rv = 0.0;

        let (assign51740_e66457, assign51740_e66457_d_n5, assign51740_e66457_d_n6, assign51740_e66457_d_n7, assign51740_e66457_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51740_e66455: f64 = (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323);
        (assign51740_e66455, ((locals.var_vdse__blk1388_dn5 * locals.var_inv_phit1__blk1323) + (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323_dn5)), ((locals.var_vdse__blk1388_dn6 * locals.var_inv_phit1__blk1323) + (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323_dn6)), ((locals.var_vdse__blk1388_dn7 * locals.var_inv_phit1__blk1323) + (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323_dn7)), ((locals.var_vdse__blk1388_dn8 * locals.var_inv_phit1__blk1323) + (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323_dn8)),)
    } else {
        (locals.var_udse__blk1389, locals.var_udse__blk1389_dn5, locals.var_udse__blk1389_dn6, locals.var_udse__blk1389_dn7, locals.var_udse__blk1389_dn8,)
    }
};
        locals.var_udse__blk1389 = assign51740_e66457;
        locals.var_udse__blk1389_dn5 = assign51740_e66457_d_n5;
        locals.var_udse__blk1389_dn6 = assign51740_e66457_d_n6;
        locals.var_udse__blk1389_dn7 = assign51740_e66457_d_n7;
        locals.var_udse__blk1389_dn8 = assign51740_e66457_d_n8;
        locals.var_udse__blk1389_rv = 0.0;

        let (assign51750_e66465, assign51750_e66465_d_n5, assign51750_e66465_d_n6, assign51750_e66465_d_n7, assign51750_e66465_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51750_e66463: f64 = (locals.var_xn_s__blk1332 + locals.var_udse__blk1389);
        (assign51750_e66463, (locals.var_xn_s__blk1332_dn5 + locals.var_udse__blk1389_dn5), (locals.var_xn_s__blk1332_dn6 + locals.var_udse__blk1389_dn6), (locals.var_xn_s__blk1332_dn7 + locals.var_udse__blk1389_dn7), (locals.var_xn_s__blk1332_dn8 + locals.var_udse__blk1389_dn8),)
    } else {
        (locals.var_xn_d__blk1390, locals.var_xn_d__blk1390_dn5, locals.var_xn_d__blk1390_dn6, locals.var_xn_d__blk1390_dn7, locals.var_xn_d__blk1390_dn8,)
    }
};
        locals.var_xn_d__blk1390 = assign51750_e66465;
        locals.var_xn_d__blk1390_dn5 = assign51750_e66465_d_n5;
        locals.var_xn_d__blk1390_dn6 = assign51750_e66465_d_n6;
        locals.var_xn_d__blk1390_dn7 = assign51750_e66465_d_n7;
        locals.var_xn_d__blk1390_dn8 = assign51750_e66465_d_n8;
        locals.var_xn_d__blk1390_rv = 0.0;

        let assign51760_e66468: f64 = if locals.var_udse__blk1389 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1491 = assign51760_e66468;
        locals.var_guard1491_rv = 0.0;

        let (assign51770_e66478, assign51770_e66478_d_n5, assign51770_e66478_d_n6, assign51770_e66478_d_n7, assign51770_e66478_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign51770_e66475: f64 = (-locals.var_udse__blk1389);
        let assign51770_e66476: f64 = (assign51770_e66475).exp();
        (assign51770_e66476, (assign51770_e66476 * (-locals.var_udse__blk1389_dn5)), (assign51770_e66476 * (-locals.var_udse__blk1389_dn6)), (assign51770_e66476 * (-locals.var_udse__blk1389_dn7)), (assign51770_e66476 * (-locals.var_udse__blk1389_dn8)),)
    } else {
        (locals.var_k_ds__blk1391, locals.var_k_ds__blk1391_dn5, locals.var_k_ds__blk1391_dn6, locals.var_k_ds__blk1391_dn7, locals.var_k_ds__blk1391_dn8,)
    }
};
        locals.var_k_ds__blk1391 = assign51770_e66478;
        locals.var_k_ds__blk1391_dn5 = assign51770_e66478_d_n5;
        locals.var_k_ds__blk1391_dn6 = assign51770_e66478_d_n6;
        locals.var_k_ds__blk1391_dn7 = assign51770_e66478_d_n7;
        locals.var_k_ds__blk1391_dn8 = assign51770_e66478_d_n8;
        locals.var_k_ds__blk1391_rv = 0.0;

        let (assign51780_e66509, assign51780_e66509_d_n5, assign51780_e66509_d_n6, assign51780_e66509_d_n7, assign51780_e66509_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1491 == 0.0)) {
        let assign51780_e66489: f64 = (locals.var_udse__blk1389 - 460.51701859880916);
        let assign51780_e66494: f64 = (locals.var_udse__blk1389 - 460.51701859880916);
        let assign51780_e66498: f64 = (locals.var_udse__blk1389 - 460.51701859880916);
        let assign51780_e66500: f64 = (assign51780_e66498 * 0.3333333333333333);
        let assign51780_e66501: f64 = (1.0 + assign51780_e66500);
        let assign51780_e66502: f64 = (assign51780_e66494 * assign51780_e66501);
        let assign51780_e66503: f64 = (0.5 * assign51780_e66502);
        let assign51780_e66504: f64 = (1.0 + assign51780_e66503);
        let assign51780_e66505: f64 = (assign51780_e66489 * assign51780_e66504);
        let assign51780_e66506: f64 = (1.0 + assign51780_e66505);
        let assign51780_e66507: f64 = (1e-200 / assign51780_e66506);
        (assign51780_e66507, (-((1e-200 * ((locals.var_udse__blk1389_dn5 * assign51780_e66504) + (assign51780_e66489 * (0.5 * ((locals.var_udse__blk1389_dn5 * assign51780_e66501) + (assign51780_e66494 * (locals.var_udse__blk1389_dn5 * 0.3333333333333333))))))) / (assign51780_e66506 * assign51780_e66506))), (-((1e-200 * ((locals.var_udse__blk1389_dn6 * assign51780_e66504) + (assign51780_e66489 * (0.5 * ((locals.var_udse__blk1389_dn6 * assign51780_e66501) + (assign51780_e66494 * (locals.var_udse__blk1389_dn6 * 0.3333333333333333))))))) / (assign51780_e66506 * assign51780_e66506))), (-((1e-200 * ((locals.var_udse__blk1389_dn7 * assign51780_e66504) + (assign51780_e66489 * (0.5 * ((locals.var_udse__blk1389_dn7 * assign51780_e66501) + (assign51780_e66494 * (locals.var_udse__blk1389_dn7 * 0.3333333333333333))))))) / (assign51780_e66506 * assign51780_e66506))), (-((1e-200 * ((locals.var_udse__blk1389_dn8 * assign51780_e66504) + (assign51780_e66489 * (0.5 * ((locals.var_udse__blk1389_dn8 * assign51780_e66501) + (assign51780_e66494 * (locals.var_udse__blk1389_dn8 * 0.3333333333333333))))))) / (assign51780_e66506 * assign51780_e66506))),)
    } else {
        (locals.var_k_ds__blk1391, locals.var_k_ds__blk1391_dn5, locals.var_k_ds__blk1391_dn6, locals.var_k_ds__blk1391_dn7, locals.var_k_ds__blk1391_dn8,)
    }
};
        locals.var_k_ds__blk1391 = assign51780_e66509;
        locals.var_k_ds__blk1391_dn5 = assign51780_e66509_d_n5;
        locals.var_k_ds__blk1391_dn6 = assign51780_e66509_d_n6;
        locals.var_k_ds__blk1391_dn7 = assign51780_e66509_d_n7;
        locals.var_k_ds__blk1391_dn8 = assign51780_e66509_d_n8;
        locals.var_k_ds__blk1391_rv = 0.0;

        let (assign51790_e66517, assign51790_e66517_d_n5, assign51790_e66517_d_n6, assign51790_e66517_d_n7, assign51790_e66517_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51790_e66515: f64 = (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391);
        (assign51790_e66515, ((locals.var_delta_ns__blk1347_dn5 * locals.var_k_ds__blk1391) + (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391_dn5)), ((locals.var_delta_ns__blk1347_dn6 * locals.var_k_ds__blk1391) + (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391_dn6)), ((locals.var_delta_ns__blk1347_dn7 * locals.var_k_ds__blk1391) + (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391_dn7)), ((locals.var_delta_ns__blk1347_dn8 * locals.var_k_ds__blk1391) + (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391_dn8)),)
    } else {
        (locals.var_delta_nd__blk1392, locals.var_delta_nd__blk1392_dn5, locals.var_delta_nd__blk1392_dn6, locals.var_delta_nd__blk1392_dn7, locals.var_delta_nd__blk1392_dn8,)
    }
};
        locals.var_delta_nd__blk1392 = assign51790_e66517;
        locals.var_delta_nd__blk1392_dn5 = assign51790_e66517_d_n5;
        locals.var_delta_nd__blk1392_dn6 = assign51790_e66517_d_n6;
        locals.var_delta_nd__blk1392_dn7 = assign51790_e66517_d_n7;
        locals.var_delta_nd__blk1392_dn8 = assign51790_e66517_d_n8;
        locals.var_delta_nd__blk1392_rv = 0.0;

        let assign51800_e66519: f64 = (locals.var_xg__blk1326).abs();
        let assign51800_e66521: f64 = if assign51800_e66519 <= locals.var_margin__blk1344 { 1.0 } else { 0.0 };
        locals.var_guard1492 = assign51800_e66521;
        locals.var_guard1492_rv = 0.0;

        let (assign51810_e66535, assign51810_e66535_d_n5, assign51810_e66535_d_n6, assign51810_e66535_d_n7, assign51810_e66535_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 != 0.0)) {
        let assign51810_e66529: f64 = (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345);
        let assign51810_e66531: f64 = (assign51810_e66529 * 0.16666666666666666);
        let assign51810_e66533: f64 = (assign51810_e66531 * 0.7071067811865475);
        (assign51810_e66533, ((((locals.var_inv_xi__blk1345_dn5 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn6 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn7 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn8 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn8)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8,)
    }
};
        locals.var_sp_s_temp1__blk1432 = assign51810_e66535;
        locals.var_sp_s_temp1__blk1432_dn5 = assign51810_e66535_d_n5;
        locals.var_sp_s_temp1__blk1432_dn6 = assign51810_e66535_d_n6;
        locals.var_sp_s_temp1__blk1432_dn7 = assign51810_e66535_d_n7;
        locals.var_sp_s_temp1__blk1432_dn8 = assign51810_e66535_d_n8;
        locals.var_sp_s_temp1__blk1432_rv = 0.0;

        let (assign51820_e66557, assign51820_e66557_d_n5, assign51820_e66557_d_n6, assign51820_e66557_d_n7, assign51820_e66557_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 != 0.0)) {
        let assign51820_e66543: f64 = (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345);
        let assign51820_e66548: f64 = (1.0 - locals.var_delta_nd__blk1392);
        let assign51820_e66549: f64 = (locals.var_xg__blk1326 * assign51820_e66548);
        let assign51820_e66551: f64 = (assign51820_e66549 * locals.var_gf__blk1307);
        let assign51820_e66553: f64 = (assign51820_e66551 * locals.var_sp_s_temp1__blk1432);
        let assign51820_e66554: f64 = (1.0 + assign51820_e66553);
        let assign51820_e66555: f64 = (assign51820_e66543 * assign51820_e66554);
        (assign51820_e66555, ((((locals.var_xg__blk1326_dn5 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn5)) * assign51820_e66554) + (assign51820_e66543 * ((((((locals.var_xg__blk1326_dn5 * assign51820_e66548) + (locals.var_xg__blk1326 * (-locals.var_delta_nd__blk1392_dn5))) * locals.var_gf__blk1307) + (assign51820_e66549 * locals.var_gf__blk1307_dn5)) * locals.var_sp_s_temp1__blk1432) + (assign51820_e66551 * locals.var_sp_s_temp1__blk1432_dn5)))), ((((locals.var_xg__blk1326_dn6 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn6)) * assign51820_e66554) + (assign51820_e66543 * ((((((locals.var_xg__blk1326_dn6 * assign51820_e66548) + (locals.var_xg__blk1326 * (-locals.var_delta_nd__blk1392_dn6))) * locals.var_gf__blk1307) + (assign51820_e66549 * locals.var_gf__blk1307_dn6)) * locals.var_sp_s_temp1__blk1432) + (assign51820_e66551 * locals.var_sp_s_temp1__blk1432_dn6)))), ((((locals.var_xg__blk1326_dn7 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn7)) * assign51820_e66554) + (assign51820_e66543 * ((((((locals.var_xg__blk1326_dn7 * assign51820_e66548) + (locals.var_xg__blk1326 * (-locals.var_delta_nd__blk1392_dn7))) * locals.var_gf__blk1307) + (assign51820_e66549 * locals.var_gf__blk1307_dn7)) * locals.var_sp_s_temp1__blk1432) + (assign51820_e66551 * locals.var_sp_s_temp1__blk1432_dn7)))), ((((locals.var_xg__blk1326_dn8 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn8)) * assign51820_e66554) + (assign51820_e66543 * ((((((locals.var_xg__blk1326_dn8 * assign51820_e66548) + (locals.var_xg__blk1326 * (-locals.var_delta_nd__blk1392_dn8))) * locals.var_gf__blk1307) + (assign51820_e66549 * locals.var_gf__blk1307_dn8)) * locals.var_sp_s_temp1__blk1432) + (assign51820_e66551 * locals.var_sp_s_temp1__blk1432_dn8)))),)
    } else {
        (locals.var_x_d__blk1393, locals.var_x_d__blk1393_dn5, locals.var_x_d__blk1393_dn6, locals.var_x_d__blk1393_dn7, locals.var_x_d__blk1393_dn8,)
    }
};
        locals.var_x_d__blk1393 = assign51820_e66557;
        locals.var_x_d__blk1393_dn5 = assign51820_e66557_d_n5;
        locals.var_x_d__blk1393_dn6 = assign51820_e66557_d_n6;
        locals.var_x_d__blk1393_dn7 = assign51820_e66557_d_n7;
        locals.var_x_d__blk1393_dn8 = assign51820_e66557_d_n8;
        locals.var_x_d__blk1393_rv = 0.0;

        let (assign51830_e66568, assign51830_e66568_d_n5, assign51830_e66568_d_n6, assign51830_e66568_d_n7, assign51830_e66568_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51830_e66566: f64 = (locals.var_xn_d__blk1390 + 3.0);
        (assign51830_e66566, locals.var_xn_d__blk1390_dn5, locals.var_xn_d__blk1390_dn6, locals.var_xn_d__blk1390_dn7, locals.var_xn_d__blk1390_dn8,)
    } else {
        (locals.var_sp_s_bx__blk1453, locals.var_sp_s_bx__blk1453_dn5, locals.var_sp_s_bx__blk1453_dn6, locals.var_sp_s_bx__blk1453_dn7, locals.var_sp_s_bx__blk1453_dn8,)
    }
};
        locals.var_sp_s_bx__blk1453 = assign51830_e66568;
        locals.var_sp_s_bx__blk1453_dn5 = assign51830_e66568_d_n5;
        locals.var_sp_s_bx__blk1453_dn6 = assign51830_e66568_d_n6;
        locals.var_sp_s_bx__blk1453_dn7 = assign51830_e66568_d_n7;
        locals.var_sp_s_bx__blk1453_dn8 = assign51830_e66568_d_n8;
        locals.var_sp_s_bx__blk1453_rv = 0.0;

        let (assign51840_e66603, assign51840_e66603_d_n5, assign51840_e66603_d_n6, assign51840_e66603_d_n7, assign51840_e66603_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51840_e66578: f64 = (locals.var_sp_s_x1__blk1452 + locals.var_sp_s_bx__blk1453);
        let assign51840_e66581: f64 = (locals.var_sp_s_x1__blk1452 - locals.var_sp_s_bx__blk1453);
        let assign51840_e66584: f64 = (locals.var_sp_s_x1__blk1452 - locals.var_sp_s_bx__blk1453);
        let assign51840_e66585: f64 = (assign51840_e66581 * assign51840_e66584);
        let assign51840_e66587: f64 = (assign51840_e66585 + 5.0);
        let assign51840_e66588: f64 = (assign51840_e66587).sqrt();
        let assign51840_e66589: f64 = (assign51840_e66578 - assign51840_e66588);
        let assign51840_e66590: f64 = (0.5 * assign51840_e66589);
        let assign51840_e66595: f64 = (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453);
        let assign51840_e66597: f64 = (assign51840_e66595 + 5.0);
        let assign51840_e66598: f64 = (assign51840_e66597).sqrt();
        let assign51840_e66599: f64 = (locals.var_sp_s_bx__blk1453 - assign51840_e66598);
        let assign51840_e66600: f64 = (0.5 * assign51840_e66599);
        let assign51840_e66601: f64 = (assign51840_e66590 - assign51840_e66600);
        (assign51840_e66601, ((0.5 * ((locals.var_sp_s_x1__blk1452_dn5 + locals.var_sp_s_bx__blk1453_dn5) - ((((locals.var_sp_s_x1__blk1452_dn5 - locals.var_sp_s_bx__blk1453_dn5) * assign51840_e66584) + (assign51840_e66581 * (locals.var_sp_s_x1__blk1452_dn5 - locals.var_sp_s_bx__blk1453_dn5))) / (2.0 * assign51840_e66588)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn5 - (((locals.var_sp_s_bx__blk1453_dn5 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn5)) / (2.0 * assign51840_e66598))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn6 + locals.var_sp_s_bx__blk1453_dn6) - ((((locals.var_sp_s_x1__blk1452_dn6 - locals.var_sp_s_bx__blk1453_dn6) * assign51840_e66584) + (assign51840_e66581 * (locals.var_sp_s_x1__blk1452_dn6 - locals.var_sp_s_bx__blk1453_dn6))) / (2.0 * assign51840_e66588)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn6 - (((locals.var_sp_s_bx__blk1453_dn6 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn6)) / (2.0 * assign51840_e66598))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn7 + locals.var_sp_s_bx__blk1453_dn7) - ((((locals.var_sp_s_x1__blk1452_dn7 - locals.var_sp_s_bx__blk1453_dn7) * assign51840_e66584) + (assign51840_e66581 * (locals.var_sp_s_x1__blk1452_dn7 - locals.var_sp_s_bx__blk1453_dn7))) / (2.0 * assign51840_e66588)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn7 - (((locals.var_sp_s_bx__blk1453_dn7 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn7)) / (2.0 * assign51840_e66598))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn8 + locals.var_sp_s_bx__blk1453_dn8) - ((((locals.var_sp_s_x1__blk1452_dn8 - locals.var_sp_s_bx__blk1453_dn8) * assign51840_e66584) + (assign51840_e66581 * (locals.var_sp_s_x1__blk1452_dn8 - locals.var_sp_s_bx__blk1453_dn8))) / (2.0 * assign51840_e66588)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn8 - (((locals.var_sp_s_bx__blk1453_dn8 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn8)) / (2.0 * assign51840_e66598))))),)
    } else {
        (locals.var_sp_s_eta__blk1436, locals.var_sp_s_eta__blk1436_dn5, locals.var_sp_s_eta__blk1436_dn6, locals.var_sp_s_eta__blk1436_dn7, locals.var_sp_s_eta__blk1436_dn8,)
    }
};
        locals.var_sp_s_eta__blk1436 = assign51840_e66603;
        locals.var_sp_s_eta__blk1436_dn5 = assign51840_e66603_d_n5;
        locals.var_sp_s_eta__blk1436_dn6 = assign51840_e66603_d_n6;
        locals.var_sp_s_eta__blk1436_dn7 = assign51840_e66603_d_n7;
        locals.var_sp_s_eta__blk1436_dn8 = assign51840_e66603_d_n8;
        locals.var_sp_s_eta__blk1436_rv = 0.0;

        let (assign51850_e66614, assign51850_e66614_d_n5, assign51850_e66614_d_n6, assign51850_e66614_d_n7, assign51850_e66614_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51850_e66612: f64 = (locals.var_xg__blk1326 - locals.var_sp_s_eta__blk1436);
        (assign51850_e66612, (locals.var_xg__blk1326_dn5 - locals.var_sp_s_eta__blk1436_dn5), (locals.var_xg__blk1326_dn6 - locals.var_sp_s_eta__blk1436_dn6), (locals.var_xg__blk1326_dn7 - locals.var_sp_s_eta__blk1436_dn7), (locals.var_xg__blk1326_dn8 - locals.var_sp_s_eta__blk1436_dn8),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign51850_e66614;
        locals.var_sp_s_temp__blk1431_dn5 = assign51850_e66614_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign51850_e66614_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign51850_e66614_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign51850_e66614_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign51860_e66625, assign51860_e66625_d_n5, assign51860_e66625_d_n6, assign51860_e66625_d_n7, assign51860_e66625_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51860_e66622: f64 = (-locals.var_sp_s_eta__blk1436);
        let assign51860_e66623: f64 = (assign51860_e66622).exp();
        (assign51860_e66623, (assign51860_e66623 * (-locals.var_sp_s_eta__blk1436_dn5)), (assign51860_e66623 * (-locals.var_sp_s_eta__blk1436_dn6)), (assign51860_e66623 * (-locals.var_sp_s_eta__blk1436_dn7)), (assign51860_e66623 * (-locals.var_sp_s_eta__blk1436_dn8)),)
    } else {
        (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8,)
    }
};
        locals.var_sp_s_temp1__blk1432 = assign51860_e66625;
        locals.var_sp_s_temp1__blk1432_dn5 = assign51860_e66625_d_n5;
        locals.var_sp_s_temp1__blk1432_dn6 = assign51860_e66625_d_n6;
        locals.var_sp_s_temp1__blk1432_dn7 = assign51860_e66625_d_n7;
        locals.var_sp_s_temp1__blk1432_dn8 = assign51860_e66625_d_n8;
        locals.var_sp_s_temp1__blk1432_rv = 0.0;

        let (assign51870_e66640, assign51870_e66640_d_n5, assign51870_e66640_d_n6, assign51870_e66640_d_n7, assign51870_e66640_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51870_e66636: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436);
        let assign51870_e66637: f64 = (2.0 + assign51870_e66636);
        let assign51870_e66638: f64 = (1.0 / assign51870_e66637);
        (assign51870_e66638, (-(((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn5)) / (assign51870_e66637 * assign51870_e66637))), (-(((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn6)) / (assign51870_e66637 * assign51870_e66637))), (-(((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn7)) / (assign51870_e66637 * assign51870_e66637))), (-(((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn8)) / (assign51870_e66637 * assign51870_e66637))),)
    } else {
        (locals.var_sp_s_temp2__blk1433, locals.var_sp_s_temp2__blk1433_dn5, locals.var_sp_s_temp2__blk1433_dn6, locals.var_sp_s_temp2__blk1433_dn7, locals.var_sp_s_temp2__blk1433_dn8,)
    }
};
        locals.var_sp_s_temp2__blk1433 = assign51870_e66640;
        locals.var_sp_s_temp2__blk1433_dn5 = assign51870_e66640_d_n5;
        locals.var_sp_s_temp2__blk1433_dn6 = assign51870_e66640_d_n6;
        locals.var_sp_s_temp2__blk1433_dn7 = assign51870_e66640_d_n7;
        locals.var_sp_s_temp2__blk1433_dn8 = assign51870_e66640_d_n8;
        locals.var_sp_s_temp2__blk1433_rv = 0.0;

        let (assign51880_e66653, assign51880_e66653_d_n5, assign51880_e66653_d_n6, assign51880_e66653_d_n7, assign51880_e66653_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51880_e66649: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436);
        let assign51880_e66651: f64 = (assign51880_e66649 * locals.var_sp_s_temp2__blk1433);
        (assign51880_e66651, ((((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign51880_e66649 * locals.var_sp_s_temp2__blk1433_dn5)), ((((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign51880_e66649 * locals.var_sp_s_temp2__blk1433_dn6)), ((((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign51880_e66649 * locals.var_sp_s_temp2__blk1433_dn7)), ((((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign51880_e66649 * locals.var_sp_s_temp2__blk1433_dn8)),)
    } else {
        (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8,)
    }
};
        locals.var_sp_s_xi0__blk1443 = assign51880_e66653;
        locals.var_sp_s_xi0__blk1443_dn5 = assign51880_e66653_d_n5;
        locals.var_sp_s_xi0__blk1443_dn6 = assign51880_e66653_d_n6;
        locals.var_sp_s_xi0__blk1443_dn7 = assign51880_e66653_d_n7;
        locals.var_sp_s_xi0__blk1443_dn8 = assign51880_e66653_d_n8;
        locals.var_sp_s_xi0__blk1443_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_46(
        locals: &mut StampLocals,
    ) {
        let (assign51890_e66668, assign51890_e66668_d_n5, assign51890_e66668_d_n6, assign51890_e66668_d_n7, assign51890_e66668_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51890_e66663: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433);
        let assign51890_e66665: f64 = (assign51890_e66663 * locals.var_sp_s_temp2__blk1433);
        let assign51890_e66666: f64 = (4.0 * assign51890_e66665);
        (assign51890_e66666, (4.0 * ((((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign51890_e66663 * locals.var_sp_s_temp2__blk1433_dn5))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign51890_e66663 * locals.var_sp_s_temp2__blk1433_dn6))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign51890_e66663 * locals.var_sp_s_temp2__blk1433_dn7))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign51890_e66663 * locals.var_sp_s_temp2__blk1433_dn8))),)
    } else {
        (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8,)
    }
};
        locals.var_sp_s_xi1__blk1444 = assign51890_e66668;
        locals.var_sp_s_xi1__blk1444_dn5 = assign51890_e66668_d_n5;
        locals.var_sp_s_xi1__blk1444_dn6 = assign51890_e66668_d_n6;
        locals.var_sp_s_xi1__blk1444_dn7 = assign51890_e66668_d_n7;
        locals.var_sp_s_xi1__blk1444_dn8 = assign51890_e66668_d_n8;
        locals.var_sp_s_xi1__blk1444_rv = 0.0;

        let (assign51900_e66687, assign51900_e66687_d_n5, assign51900_e66687_d_n6, assign51900_e66687_d_n7, assign51900_e66687_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51900_e66677: f64 = (8.0 * locals.var_sp_s_temp2__blk1433);
        let assign51900_e66680: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
        let assign51900_e66681: f64 = (assign51900_e66677 - assign51900_e66680);
        let assign51900_e66683: f64 = (assign51900_e66681 * locals.var_sp_s_temp2__blk1433);
        let assign51900_e66685: f64 = (assign51900_e66683 * locals.var_sp_s_temp2__blk1433);
        (assign51900_e66685, ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66681 * locals.var_sp_s_temp2__blk1433_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66683 * locals.var_sp_s_temp2__blk1433_dn5)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66681 * locals.var_sp_s_temp2__blk1433_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66683 * locals.var_sp_s_temp2__blk1433_dn6)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66681 * locals.var_sp_s_temp2__blk1433_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66683 * locals.var_sp_s_temp2__blk1433_dn7)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66681 * locals.var_sp_s_temp2__blk1433_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66683 * locals.var_sp_s_temp2__blk1433_dn8)),)
    } else {
        (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8,)
    }
};
        locals.var_sp_s_xi2__blk1445 = assign51900_e66687;
        locals.var_sp_s_xi2__blk1445_dn5 = assign51900_e66687_d_n5;
        locals.var_sp_s_xi2__blk1445_dn6 = assign51900_e66687_d_n6;
        locals.var_sp_s_xi2__blk1445_dn7 = assign51900_e66687_d_n7;
        locals.var_sp_s_xi2__blk1445_dn8 = assign51900_e66687_d_n8;
        locals.var_sp_s_xi2__blk1445_rv = 0.0;

        let (assign51910_e66737, assign51910_e66737_d_n5, assign51910_e66737_d_n6, assign51910_e66737_d_n7, assign51910_e66737_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51910_e66697: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign51910_e66701: f64 = (locals.var_sp_s_temp1__blk1432 + locals.var_sp_s_eta__blk1436);
        let assign51910_e66703: f64 = (assign51910_e66701 - 1.0);
        let assign51910_e66707: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
        let assign51910_e66709: f64 = (assign51910_e66707 + locals.var_sp_s_xi0__blk1443);
        let assign51910_e66710: f64 = (locals.var_delta_nd__blk1392 * assign51910_e66709);
        let assign51910_e66711: f64 = (assign51910_e66703 - assign51910_e66710);
        let assign51910_e66712: f64 = (locals.var_gf2__blk1308 * assign51910_e66711);
        let assign51910_e66713: f64 = (assign51910_e66697 - assign51910_e66712);
        let (assign51910_e66735, assign51910_e66735_d_n5, assign51910_e66735_d_n6, assign51910_e66735_d_n7, assign51910_e66735_d_n8,) = {
            if (1e-40 > assign51910_e66713) {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign51910_e66718: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
                let assign51910_e66722: f64 = (locals.var_sp_s_temp1__blk1432 + locals.var_sp_s_eta__blk1436);
                let assign51910_e66724: f64 = (assign51910_e66722 - 1.0);
                let assign51910_e66728: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
                let assign51910_e66730: f64 = (assign51910_e66728 + locals.var_sp_s_xi0__blk1443);
                let assign51910_e66731: f64 = (locals.var_delta_nd__blk1392 * assign51910_e66730);
                let assign51910_e66732: f64 = (assign51910_e66724 - assign51910_e66731);
                let assign51910_e66733: f64 = (locals.var_gf2__blk1308 * assign51910_e66732);
                let assign51910_e66734: f64 = (assign51910_e66718 - assign51910_e66733);
                (assign51910_e66734, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign51910_e66732) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn5 + locals.var_sp_s_eta__blk1436_dn5) - ((locals.var_delta_nd__blk1392_dn5 * assign51910_e66730) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_eta__blk1436_dn5 + locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign51910_e66732) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn6 + locals.var_sp_s_eta__blk1436_dn6) - ((locals.var_delta_nd__blk1392_dn6 * assign51910_e66730) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_eta__blk1436_dn6 + locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign51910_e66732) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn7 + locals.var_sp_s_eta__blk1436_dn7) - ((locals.var_delta_nd__blk1392_dn7 * assign51910_e66730) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_eta__blk1436_dn7 + locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign51910_e66732) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn8 + locals.var_sp_s_eta__blk1436_dn8) - ((locals.var_delta_nd__blk1392_dn8 * assign51910_e66730) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_eta__blk1436_dn8 + locals.var_sp_s_xi0__blk1443_dn8))))))),)
            }
        };
        (assign51910_e66735, assign51910_e66735_d_n5, assign51910_e66735_d_n6, assign51910_e66735_d_n7, assign51910_e66735_d_n8,)
    } else {
        (locals.var_sp_s_a__blk1437, locals.var_sp_s_a__blk1437_dn5, locals.var_sp_s_a__blk1437_dn6, locals.var_sp_s_a__blk1437_dn7, locals.var_sp_s_a__blk1437_dn8,)
    }
};
        locals.var_sp_s_a__blk1437 = assign51910_e66737;
        locals.var_sp_s_a__blk1437_dn5 = assign51910_e66737_d_n5;
        locals.var_sp_s_a__blk1437_dn6 = assign51910_e66737_d_n6;
        locals.var_sp_s_a__blk1437_dn7 = assign51910_e66737_d_n7;
        locals.var_sp_s_a__blk1437_dn8 = assign51910_e66737_d_n8;
        locals.var_sp_s_a__blk1437_rv = 0.0;

        let (assign51920_e66756, assign51920_e66756_d_n5, assign51920_e66756_d_n6, assign51920_e66756_d_n7, assign51920_e66756_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51920_e66750: f64 = (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445);
        let assign51920_e66751: f64 = (locals.var_sp_s_temp1__blk1432 - assign51920_e66750);
        let assign51920_e66752: f64 = (locals.var_gf2__blk1308 * assign51920_e66751);
        let assign51920_e66753: f64 = (0.5 * assign51920_e66752);
        let assign51920_e66754: f64 = (1.0 - assign51920_e66753);
        (assign51920_e66754, (-(0.5 * ((locals.var_gf2__blk1308_dn5 * assign51920_e66751) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn5 - ((locals.var_delta_nd__blk1392_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn5))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn6 * assign51920_e66751) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn6 - ((locals.var_delta_nd__blk1392_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn6))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn7 * assign51920_e66751) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn7 - ((locals.var_delta_nd__blk1392_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn7))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn8 * assign51920_e66751) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn8 - ((locals.var_delta_nd__blk1392_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn8))))))),)
    } else {
        (locals.var_sp_s_b__blk1454, locals.var_sp_s_b__blk1454_dn5, locals.var_sp_s_b__blk1454_dn6, locals.var_sp_s_b__blk1454_dn7, locals.var_sp_s_b__blk1454_dn8,)
    }
};
        locals.var_sp_s_b__blk1454 = assign51920_e66756;
        locals.var_sp_s_b__blk1454_dn5 = assign51920_e66756_d_n5;
        locals.var_sp_s_b__blk1454_dn6 = assign51920_e66756_d_n6;
        locals.var_sp_s_b__blk1454_dn7 = assign51920_e66756_d_n7;
        locals.var_sp_s_b__blk1454_dn8 = assign51920_e66756_d_n8;
        locals.var_sp_s_b__blk1454_rv = 0.0;

        let (assign51930_e66779, assign51930_e66779_d_n5, assign51930_e66779_d_n6, assign51930_e66779_d_n7, assign51930_e66779_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51930_e66765: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
        let assign51930_e66769: f64 = (1.0 - locals.var_sp_s_temp1__blk1432);
        let assign51930_e66773: f64 = (1.0 + locals.var_sp_s_xi1__blk1444);
        let assign51930_e66774: f64 = (locals.var_delta_nd__blk1392 * assign51930_e66773);
        let assign51930_e66775: f64 = (assign51930_e66769 - assign51930_e66774);
        let assign51930_e66776: f64 = (locals.var_gf2__blk1308 * assign51930_e66775);
        let assign51930_e66777: f64 = (assign51930_e66765 + assign51930_e66776);
        (assign51930_e66777, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign51930_e66775) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn5) - ((locals.var_delta_nd__blk1392_dn5 * assign51930_e66773) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn5)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign51930_e66775) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn6) - ((locals.var_delta_nd__blk1392_dn6 * assign51930_e66773) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign51930_e66775) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn7) - ((locals.var_delta_nd__blk1392_dn7 * assign51930_e66773) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign51930_e66775) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn8) - ((locals.var_delta_nd__blk1392_dn8 * assign51930_e66773) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn8)))))),)
    } else {
        (locals.var_sp_s_c__blk1438, locals.var_sp_s_c__blk1438_dn5, locals.var_sp_s_c__blk1438_dn6, locals.var_sp_s_c__blk1438_dn7, locals.var_sp_s_c__blk1438_dn8,)
    }
};
        locals.var_sp_s_c__blk1438 = assign51930_e66779;
        locals.var_sp_s_c__blk1438_dn5 = assign51930_e66779_d_n5;
        locals.var_sp_s_c__blk1438_dn6 = assign51930_e66779_d_n6;
        locals.var_sp_s_c__blk1438_dn7 = assign51930_e66779_d_n7;
        locals.var_sp_s_c__blk1438_dn8 = assign51930_e66779_d_n8;
        locals.var_sp_s_c__blk1438_rv = 0.0;

        let (assign51940_e66795, assign51940_e66795_d_n5, assign51940_e66795_d_n6, assign51940_e66795_d_n7, assign51940_e66795_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51940_e66788: f64 = (locals.var_xn_d__blk1390 - locals.var_sp_s_eta__blk1436);
        let assign51940_e66791: f64 = (locals.var_sp_s_a__blk1437 / locals.var_gf2__blk1308);
        let assign51940_e66792: f64 = (assign51940_e66791).ln();
        let assign51940_e66793: f64 = (assign51940_e66788 + assign51940_e66792);
        (assign51940_e66793, ((locals.var_xn_d__blk1390_dn5 - locals.var_sp_s_eta__blk1436_dn5) + ((((locals.var_sp_s_a__blk1437_dn5 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn5)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign51940_e66791)), ((locals.var_xn_d__blk1390_dn6 - locals.var_sp_s_eta__blk1436_dn6) + ((((locals.var_sp_s_a__blk1437_dn6 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn6)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign51940_e66791)), ((locals.var_xn_d__blk1390_dn7 - locals.var_sp_s_eta__blk1436_dn7) + ((((locals.var_sp_s_a__blk1437_dn7 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn7)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign51940_e66791)), ((locals.var_xn_d__blk1390_dn8 - locals.var_sp_s_eta__blk1436_dn8) + ((((locals.var_sp_s_a__blk1437_dn8 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn8)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign51940_e66791)),)
    } else {
        (locals.var_sp_s_tau__blk1439, locals.var_sp_s_tau__blk1439_dn5, locals.var_sp_s_tau__blk1439_dn6, locals.var_sp_s_tau__blk1439_dn7, locals.var_sp_s_tau__blk1439_dn8,)
    }
};
        locals.var_sp_s_tau__blk1439 = assign51940_e66795;
        locals.var_sp_s_tau__blk1439_dn5 = assign51940_e66795_d_n5;
        locals.var_sp_s_tau__blk1439_dn6 = assign51940_e66795_d_n6;
        locals.var_sp_s_tau__blk1439_dn7 = assign51940_e66795_d_n7;
        locals.var_sp_s_tau__blk1439_dn8 = assign51940_e66795_d_n8;
        locals.var_sp_s_tau__blk1439_rv = 0.0;

        let (assign51950_e66806, assign51950_e66806_d_n5, assign51950_e66806_d_n6, assign51950_e66806_d_n7, assign51950_e66806_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51950_e66804: f64 = (locals.var_sp_s_a__blk1437 + locals.var_sp_s_c__blk1438);
        (assign51950_e66804, (locals.var_sp_s_a__blk1437_dn5 + locals.var_sp_s_c__blk1438_dn5), (locals.var_sp_s_a__blk1437_dn6 + locals.var_sp_s_c__blk1438_dn6), (locals.var_sp_s_a__blk1437_dn7 + locals.var_sp_s_c__blk1438_dn7), (locals.var_sp_s_a__blk1437_dn8 + locals.var_sp_s_c__blk1438_dn8),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8,)
    }
};
        locals.var_nu = assign51950_e66806;
        locals.var_nu_dn5 = assign51950_e66806_d_n5;
        locals.var_nu_dn6 = assign51950_e66806_d_n6;
        locals.var_nu_dn7 = assign51950_e66806_d_n7;
        locals.var_nu_dn8 = assign51950_e66806_d_n8;
        locals.var_nu_rv = 0.0;

        let (assign51960_e66829, assign51960_e66829_d_n5, assign51960_e66829_d_n6, assign51960_e66829_d_n7, assign51960_e66829_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51960_e66815: f64 = (locals.var_nu * locals.var_nu);
        let assign51960_e66820: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
        let assign51960_e66821: f64 = (0.5 * assign51960_e66820);
        let assign51960_e66824: f64 = (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454);
        let assign51960_e66825: f64 = (assign51960_e66821 - assign51960_e66824);
        let assign51960_e66826: f64 = (locals.var_sp_s_tau__blk1439 * assign51960_e66825);
        let assign51960_e66827: f64 = (assign51960_e66815 + assign51960_e66826);
        (assign51960_e66827, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau__blk1439_dn5 * assign51960_e66825) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5))) - ((locals.var_sp_s_a__blk1437_dn5 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn5)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1439_dn6 * assign51960_e66825) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6))) - ((locals.var_sp_s_a__blk1437_dn6 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1439_dn7 * assign51960_e66825) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7))) - ((locals.var_sp_s_a__blk1437_dn7 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1439_dn8 * assign51960_e66825) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8))) - ((locals.var_sp_s_a__blk1437_dn8 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn8)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8,)
    }
};
        locals.var_mutau = assign51960_e66829;
        locals.var_mutau_dn5 = assign51960_e66829_d_n5;
        locals.var_mutau_dn6 = assign51960_e66829_d_n6;
        locals.var_mutau_dn7 = assign51960_e66829_d_n7;
        locals.var_mutau_dn8 = assign51960_e66829_d_n8;
        locals.var_mutau_rv = 0.0;

        let (assign51970_e66866, assign51970_e66866_d_n5, assign51970_e66866_d_n6, assign51970_e66866_d_n7, assign51970_e66866_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51970_e66839: f64 = (locals.var_sp_s_a__blk1437 * locals.var_nu);
        let assign51970_e66841: f64 = (assign51970_e66839 * locals.var_sp_s_tau__blk1439);
        let assign51970_e66845: f64 = (locals.var_nu / locals.var_mutau);
        let assign51970_e66847: f64 = (assign51970_e66845 * locals.var_sp_s_tau__blk1439);
        let assign51970_e66849: f64 = (assign51970_e66847 * locals.var_sp_s_tau__blk1439);
        let assign51970_e66851: f64 = (assign51970_e66849 * locals.var_sp_s_c__blk1438);
        let assign51970_e66854: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
        let assign51970_e66856: f64 = (assign51970_e66854 * 0.3333333333333333);
        let assign51970_e66859: f64 = (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454);
        let assign51970_e66860: f64 = (assign51970_e66856 - assign51970_e66859);
        let assign51970_e66861: f64 = (assign51970_e66851 * assign51970_e66860);
        let assign51970_e66862: f64 = (locals.var_mutau + assign51970_e66861);
        let assign51970_e66863: f64 = (assign51970_e66841 / assign51970_e66862);
        let assign51970_e66864: f64 = (locals.var_sp_s_eta__blk1436 + assign51970_e66863);
        (assign51970_e66864, (locals.var_sp_s_eta__blk1436_dn5 + (((((((locals.var_sp_s_a__blk1437_dn5 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn5)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66839 * locals.var_sp_s_tau__blk1439_dn5)) * assign51970_e66862) - (assign51970_e66841 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66845 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66847 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_c__blk1438) + (assign51970_e66849 * locals.var_sp_s_c__blk1438_dn5)) * assign51970_e66860) + (assign51970_e66851 * ((((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn5 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn5)))))))) / (assign51970_e66862 * assign51970_e66862))), (locals.var_sp_s_eta__blk1436_dn6 + (((((((locals.var_sp_s_a__blk1437_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66839 * locals.var_sp_s_tau__blk1439_dn6)) * assign51970_e66862) - (assign51970_e66841 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66845 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66847 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_c__blk1438) + (assign51970_e66849 * locals.var_sp_s_c__blk1438_dn6)) * assign51970_e66860) + (assign51970_e66851 * ((((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn6 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn6)))))))) / (assign51970_e66862 * assign51970_e66862))), (locals.var_sp_s_eta__blk1436_dn7 + (((((((locals.var_sp_s_a__blk1437_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66839 * locals.var_sp_s_tau__blk1439_dn7)) * assign51970_e66862) - (assign51970_e66841 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66845 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66847 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_c__blk1438) + (assign51970_e66849 * locals.var_sp_s_c__blk1438_dn7)) * assign51970_e66860) + (assign51970_e66851 * ((((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn7 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn7)))))))) / (assign51970_e66862 * assign51970_e66862))), (locals.var_sp_s_eta__blk1436_dn8 + (((((((locals.var_sp_s_a__blk1437_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66839 * locals.var_sp_s_tau__blk1439_dn8)) * assign51970_e66862) - (assign51970_e66841 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66845 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66847 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_c__blk1438) + (assign51970_e66849 * locals.var_sp_s_c__blk1438_dn8)) * assign51970_e66860) + (assign51970_e66851 * ((((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn8 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn8)))))))) / (assign51970_e66862 * assign51970_e66862))),)
    } else {
        (locals.var_sp_s_x0__blk1455, locals.var_sp_s_x0__blk1455_dn5, locals.var_sp_s_x0__blk1455_dn6, locals.var_sp_s_x0__blk1455_dn7, locals.var_sp_s_x0__blk1455_dn8,)
    }
};
        locals.var_sp_s_x0__blk1455 = assign51970_e66866;
        locals.var_sp_s_x0__blk1455_dn5 = assign51970_e66866_d_n5;
        locals.var_sp_s_x0__blk1455_dn6 = assign51970_e66866_d_n6;
        locals.var_sp_s_x0__blk1455_dn7 = assign51970_e66866_d_n7;
        locals.var_sp_s_x0__blk1455_dn8 = assign51970_e66866_d_n8;
        locals.var_sp_s_x0__blk1455_rv = 0.0;

        let assign51980_e66869: f64 = if locals.var_sp_s_x0__blk1455 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1493 = assign51980_e66869;
        locals.var_guard1493_rv = 0.0;

        let (assign51990_e66881, assign51990_e66881_d_n5, assign51990_e66881_d_n6, assign51990_e66881_d_n7, assign51990_e66881_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
        let assign51990_e66879: f64 = (locals.var_sp_s_x0__blk1455).exp();
        (assign51990_e66879, (assign51990_e66879 * locals.var_sp_s_x0__blk1455_dn5), (assign51990_e66879 * locals.var_sp_s_x0__blk1455_dn6), (assign51990_e66879 * locals.var_sp_s_x0__blk1455_dn7), (assign51990_e66879 * locals.var_sp_s_x0__blk1455_dn8),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign51990_e66881;
        locals.var_sp_s_delta0__blk1441_dn5 = assign51990_e66881_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign51990_e66881_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign51990_e66881_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign51990_e66881_d_n8;
        locals.var_sp_s_delta0__blk1441_rv = 0.0;

        let (assign52000_e66894, assign52000_e66894_d_n5, assign52000_e66894_d_n6, assign52000_e66894_d_n7, assign52000_e66894_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
        let assign52000_e66892: f64 = (1.0 / locals.var_sp_s_delta0__blk1441);
        (assign52000_e66892, (-(locals.var_sp_s_delta0__blk1441_dn5 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn6 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn7 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn8 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign52000_e66894;
        locals.var_sp_s_delta1__blk1442_dn5 = assign52000_e66894_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign52000_e66894_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign52000_e66894_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign52000_e66894_d_n8;
        locals.var_sp_s_delta1__blk1442_rv = 0.0;

        let (assign52010_e66907, assign52010_e66907_d_n5, assign52010_e66907_d_n6, assign52010_e66907_d_n7, assign52010_e66907_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
        let assign52010_e66905: f64 = (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441);
        (assign52010_e66905, ((locals.var_delta_nd__blk1392_dn5 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn5)), ((locals.var_delta_nd__blk1392_dn6 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn6)), ((locals.var_delta_nd__blk1392_dn7 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn7)), ((locals.var_delta_nd__blk1392_dn8 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn8)),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign52010_e66907;
        locals.var_sp_s_delta0__blk1441_dn5 = assign52010_e66907_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign52010_e66907_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign52010_e66907_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign52010_e66907_d_n8;
        locals.var_sp_s_delta0__blk1441_rv = 0.0;

        let assign52020_e66911: f64 = (locals.var_xn_d__blk1390 - 230.25850929940458);
        let assign52020_e66912: f64 = if locals.var_sp_s_x0__blk1455 > assign52020_e66911 { 1.0 } else { 0.0 };
        locals.var_guard1494 = assign52020_e66912;
        locals.var_guard1494_rv = 0.0;

        let (assign52030_e66929, assign52030_e66929_d_n5, assign52030_e66929_d_n6, assign52030_e66929_d_n7, assign52030_e66929_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign52030_e66926: f64 = (locals.var_sp_s_x0__blk1455 - locals.var_xn_d__blk1390);
        let assign52030_e66927: f64 = (assign52030_e66926).exp();
        (assign52030_e66927, (assign52030_e66927 * (locals.var_sp_s_x0__blk1455_dn5 - locals.var_xn_d__blk1390_dn5)), (assign52030_e66927 * (locals.var_sp_s_x0__blk1455_dn6 - locals.var_xn_d__blk1390_dn6)), (assign52030_e66927 * (locals.var_sp_s_x0__blk1455_dn7 - locals.var_xn_d__blk1390_dn7)), (assign52030_e66927 * (locals.var_sp_s_x0__blk1455_dn8 - locals.var_xn_d__blk1390_dn8)),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign52030_e66929;
        locals.var_sp_s_delta0__blk1441_dn5 = assign52030_e66929_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign52030_e66929_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign52030_e66929_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign52030_e66929_d_n8;
        locals.var_sp_s_delta0__blk1441_rv = 0.0;

        let (assign52040_e66945, assign52040_e66945_d_n5, assign52040_e66945_d_n6, assign52040_e66945_d_n7, assign52040_e66945_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign52040_e66943: f64 = (locals.var_delta_nd__blk1392 / locals.var_sp_s_delta0__blk1441);
        (assign52040_e66943, (((locals.var_delta_nd__blk1392_dn5 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn5)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_nd__blk1392_dn6 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn6)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_nd__blk1392_dn7 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn7)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_nd__blk1392_dn8 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn8)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign52040_e66945;
        locals.var_sp_s_delta1__blk1442_dn5 = assign52040_e66945_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign52040_e66945_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign52040_e66945_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign52040_e66945_d_n8;
        locals.var_sp_s_delta1__blk1442_rv = 0.0;

        let (assign52050_e66988, assign52050_e66988_d_n5, assign52050_e66988_d_n6, assign52050_e66988_d_n7, assign52050_e66988_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 == 0.0)) {
        let assign52050_e66962: f64 = (locals.var_xn_d__blk1390 - locals.var_sp_s_x0__blk1455);
        let assign52050_e66964: f64 = (assign52050_e66962 - 230.25850929940458);
        let assign52050_e66969: f64 = (locals.var_xn_d__blk1390 - locals.var_sp_s_x0__blk1455);
        let assign52050_e66971: f64 = (assign52050_e66969 - 230.25850929940458);
        let assign52050_e66975: f64 = (locals.var_xn_d__blk1390 - locals.var_sp_s_x0__blk1455);
        let assign52050_e66977: f64 = (assign52050_e66975 - 230.25850929940458);
        let assign52050_e66979: f64 = (assign52050_e66977 * 0.3333333333333333);
        let assign52050_e66980: f64 = (1.0 + assign52050_e66979);
        let assign52050_e66981: f64 = (assign52050_e66971 * assign52050_e66980);
        let assign52050_e66982: f64 = (0.5 * assign52050_e66981);
        let assign52050_e66983: f64 = (1.0 + assign52050_e66982);
        let assign52050_e66984: f64 = (assign52050_e66964 * assign52050_e66983);
        let assign52050_e66985: f64 = (1.0 + assign52050_e66984);
        let assign52050_e66986: f64 = (1e-100 / assign52050_e66985);
        (assign52050_e66986, (-((1e-100 * (((locals.var_xn_d__blk1390_dn5 - locals.var_sp_s_x0__blk1455_dn5) * assign52050_e66983) + (assign52050_e66964 * (0.5 * (((locals.var_xn_d__blk1390_dn5 - locals.var_sp_s_x0__blk1455_dn5) * assign52050_e66980) + (assign52050_e66971 * ((locals.var_xn_d__blk1390_dn5 - locals.var_sp_s_x0__blk1455_dn5) * 0.3333333333333333))))))) / (assign52050_e66985 * assign52050_e66985))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn6 - locals.var_sp_s_x0__blk1455_dn6) * assign52050_e66983) + (assign52050_e66964 * (0.5 * (((locals.var_xn_d__blk1390_dn6 - locals.var_sp_s_x0__blk1455_dn6) * assign52050_e66980) + (assign52050_e66971 * ((locals.var_xn_d__blk1390_dn6 - locals.var_sp_s_x0__blk1455_dn6) * 0.3333333333333333))))))) / (assign52050_e66985 * assign52050_e66985))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn7 - locals.var_sp_s_x0__blk1455_dn7) * assign52050_e66983) + (assign52050_e66964 * (0.5 * (((locals.var_xn_d__blk1390_dn7 - locals.var_sp_s_x0__blk1455_dn7) * assign52050_e66980) + (assign52050_e66971 * ((locals.var_xn_d__blk1390_dn7 - locals.var_sp_s_x0__blk1455_dn7) * 0.3333333333333333))))))) / (assign52050_e66985 * assign52050_e66985))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn8 - locals.var_sp_s_x0__blk1455_dn8) * assign52050_e66983) + (assign52050_e66964 * (0.5 * (((locals.var_xn_d__blk1390_dn8 - locals.var_sp_s_x0__blk1455_dn8) * assign52050_e66980) + (assign52050_e66971 * ((locals.var_xn_d__blk1390_dn8 - locals.var_sp_s_x0__blk1455_dn8) * 0.3333333333333333))))))) / (assign52050_e66985 * assign52050_e66985))),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign52050_e66988;
        locals.var_sp_s_delta0__blk1441_dn5 = assign52050_e66988_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign52050_e66988_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign52050_e66988_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign52050_e66988_d_n8;
        locals.var_sp_s_delta0__blk1441_rv = 0.0;

        let (assign52060_e67025, assign52060_e67025_d_n5, assign52060_e67025_d_n6, assign52060_e67025_d_n7, assign52060_e67025_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 == 0.0)) {
        let assign52060_e67005: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
        let assign52060_e67010: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
        let assign52060_e67014: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
        let assign52060_e67016: f64 = (assign52060_e67014 * 0.3333333333333333);
        let assign52060_e67017: f64 = (1.0 + assign52060_e67016);
        let assign52060_e67018: f64 = (assign52060_e67010 * assign52060_e67017);
        let assign52060_e67019: f64 = (0.5 * assign52060_e67018);
        let assign52060_e67020: f64 = (1.0 + assign52060_e67019);
        let assign52060_e67021: f64 = (assign52060_e67005 * assign52060_e67020);
        let assign52060_e67022: f64 = (1.0 + assign52060_e67021);
        let assign52060_e67023: f64 = (1e-100 / assign52060_e67022);
        (assign52060_e67023, (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn5 * assign52060_e67020) + (assign52060_e67005 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn5 * assign52060_e67017) + (assign52060_e67010 * (locals.var_sp_s_x0__blk1455_dn5 * 0.3333333333333333))))))) / (assign52060_e67022 * assign52060_e67022))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn6 * assign52060_e67020) + (assign52060_e67005 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn6 * assign52060_e67017) + (assign52060_e67010 * (locals.var_sp_s_x0__blk1455_dn6 * 0.3333333333333333))))))) / (assign52060_e67022 * assign52060_e67022))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn7 * assign52060_e67020) + (assign52060_e67005 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn7 * assign52060_e67017) + (assign52060_e67010 * (locals.var_sp_s_x0__blk1455_dn7 * 0.3333333333333333))))))) / (assign52060_e67022 * assign52060_e67022))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn8 * assign52060_e67020) + (assign52060_e67005 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn8 * assign52060_e67017) + (assign52060_e67010 * (locals.var_sp_s_x0__blk1455_dn8 * 0.3333333333333333))))))) / (assign52060_e67022 * assign52060_e67022))),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign52060_e67025;
        locals.var_sp_s_delta1__blk1442_dn5 = assign52060_e67025_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign52060_e67025_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign52060_e67025_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign52060_e67025_d_n8;
        locals.var_sp_s_delta1__blk1442_rv = 0.0;

        let (assign52070_e67040, assign52070_e67040_d_n5, assign52070_e67040_d_n6, assign52070_e67040_d_n7, assign52070_e67040_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52070_e67036: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455);
        let assign52070_e67037: f64 = (2.0 + assign52070_e67036);
        let assign52070_e67038: f64 = (1.0 / assign52070_e67037);
        (assign52070_e67038, (-(((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn5)) / (assign52070_e67037 * assign52070_e67037))), (-(((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn6)) / (assign52070_e67037 * assign52070_e67037))), (-(((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn7)) / (assign52070_e67037 * assign52070_e67037))), (-(((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn8)) / (assign52070_e67037 * assign52070_e67037))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign52070_e67040;
        locals.var_sp_s_temp__blk1431_dn5 = assign52070_e67040_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign52070_e67040_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign52070_e67040_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign52070_e67040_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign52080_e67053, assign52080_e67053_d_n5, assign52080_e67053_d_n6, assign52080_e67053_d_n7, assign52080_e67053_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52080_e67049: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455);
        let assign52080_e67051: f64 = (assign52080_e67049 * locals.var_sp_s_temp__blk1431);
        (assign52080_e67051, ((((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn5)) * locals.var_sp_s_temp__blk1431) + (assign52080_e67049 * locals.var_sp_s_temp__blk1431_dn5)), ((((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn6)) * locals.var_sp_s_temp__blk1431) + (assign52080_e67049 * locals.var_sp_s_temp__blk1431_dn6)), ((((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn7)) * locals.var_sp_s_temp__blk1431) + (assign52080_e67049 * locals.var_sp_s_temp__blk1431_dn7)), ((((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn8)) * locals.var_sp_s_temp__blk1431) + (assign52080_e67049 * locals.var_sp_s_temp__blk1431_dn8)),)
    } else {
        (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8,)
    }
};
        locals.var_sp_s_xi0__blk1443 = assign52080_e67053;
        locals.var_sp_s_xi0__blk1443_dn5 = assign52080_e67053_d_n5;
        locals.var_sp_s_xi0__blk1443_dn6 = assign52080_e67053_d_n6;
        locals.var_sp_s_xi0__blk1443_dn7 = assign52080_e67053_d_n7;
        locals.var_sp_s_xi0__blk1443_dn8 = assign52080_e67053_d_n8;
        locals.var_sp_s_xi0__blk1443_rv = 0.0;

        let (assign52090_e67068, assign52090_e67068_d_n5, assign52090_e67068_d_n6, assign52090_e67068_d_n7, assign52090_e67068_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52090_e67063: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431);
        let assign52090_e67065: f64 = (assign52090_e67063 * locals.var_sp_s_temp__blk1431);
        let assign52090_e67066: f64 = (4.0 * assign52090_e67065);
        (assign52090_e67066, (4.0 * ((((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign52090_e67063 * locals.var_sp_s_temp__blk1431_dn5))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign52090_e67063 * locals.var_sp_s_temp__blk1431_dn6))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign52090_e67063 * locals.var_sp_s_temp__blk1431_dn7))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign52090_e67063 * locals.var_sp_s_temp__blk1431_dn8))),)
    } else {
        (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8,)
    }
};
        locals.var_sp_s_xi1__blk1444 = assign52090_e67068;
        locals.var_sp_s_xi1__blk1444_dn5 = assign52090_e67068_d_n5;
        locals.var_sp_s_xi1__blk1444_dn6 = assign52090_e67068_d_n6;
        locals.var_sp_s_xi1__blk1444_dn7 = assign52090_e67068_d_n7;
        locals.var_sp_s_xi1__blk1444_dn8 = assign52090_e67068_d_n8;
        locals.var_sp_s_xi1__blk1444_rv = 0.0;

        let (assign52100_e67087, assign52100_e67087_d_n5, assign52100_e67087_d_n6, assign52100_e67087_d_n7, assign52100_e67087_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52100_e67077: f64 = (8.0 * locals.var_sp_s_temp__blk1431);
        let assign52100_e67080: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
        let assign52100_e67081: f64 = (assign52100_e67077 - assign52100_e67080);
        let assign52100_e67083: f64 = (assign52100_e67081 * locals.var_sp_s_temp__blk1431);
        let assign52100_e67085: f64 = (assign52100_e67083 * locals.var_sp_s_temp__blk1431);
        (assign52100_e67085, ((((((8.0 * locals.var_sp_s_temp__blk1431_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67081 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67083 * locals.var_sp_s_temp__blk1431_dn5)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67081 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67083 * locals.var_sp_s_temp__blk1431_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67081 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67083 * locals.var_sp_s_temp__blk1431_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67081 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67083 * locals.var_sp_s_temp__blk1431_dn8)),)
    } else {
        (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8,)
    }
};
        locals.var_sp_s_xi2__blk1445 = assign52100_e67087;
        locals.var_sp_s_xi2__blk1445_dn5 = assign52100_e67087_d_n5;
        locals.var_sp_s_xi2__blk1445_dn6 = assign52100_e67087_d_n6;
        locals.var_sp_s_xi2__blk1445_dn7 = assign52100_e67087_d_n7;
        locals.var_sp_s_xi2__blk1445_dn8 = assign52100_e67087_d_n8;
        locals.var_sp_s_xi2__blk1445_rv = 0.0;

        let (assign52110_e67098, assign52110_e67098_d_n5, assign52110_e67098_d_n6, assign52110_e67098_d_n7, assign52110_e67098_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52110_e67096: f64 = (locals.var_xg__blk1326 - locals.var_sp_s_x0__blk1455);
        (assign52110_e67096, (locals.var_xg__blk1326_dn5 - locals.var_sp_s_x0__blk1455_dn5), (locals.var_xg__blk1326_dn6 - locals.var_sp_s_x0__blk1455_dn6), (locals.var_xg__blk1326_dn7 - locals.var_sp_s_x0__blk1455_dn7), (locals.var_xg__blk1326_dn8 - locals.var_sp_s_x0__blk1455_dn8),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign52110_e67098;
        locals.var_sp_s_temp__blk1431_dn5 = assign52110_e67098_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign52110_e67098_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign52110_e67098_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign52110_e67098_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign52120_e67123, assign52120_e67123_d_n5, assign52120_e67123_d_n6, assign52120_e67123_d_n7, assign52120_e67123_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52120_e67107: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
        let assign52120_e67111: f64 = (1.0 - locals.var_sp_s_delta1__blk1442);
        let assign52120_e67113: f64 = (assign52120_e67111 + locals.var_sp_s_delta0__blk1441);
        let assign52120_e67117: f64 = (1.0 + locals.var_sp_s_xi1__blk1444);
        let assign52120_e67118: f64 = (locals.var_delta_nd__blk1392 * assign52120_e67117);
        let assign52120_e67119: f64 = (assign52120_e67113 - assign52120_e67118);
        let assign52120_e67120: f64 = (locals.var_gf2__blk1308 * assign52120_e67119);
        let assign52120_e67121: f64 = (assign52120_e67107 + assign52120_e67120);
        (assign52120_e67121, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign52120_e67119) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn5) + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_nd__blk1392_dn5 * assign52120_e67117) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn5)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign52120_e67119) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn6) + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_nd__blk1392_dn6 * assign52120_e67117) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign52120_e67119) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn7) + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_nd__blk1392_dn7 * assign52120_e67117) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign52120_e67119) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn8) + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_nd__blk1392_dn8 * assign52120_e67117) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn8)))))),)
    } else {
        (locals.var_sp_s_pc__blk1446, locals.var_sp_s_pc__blk1446_dn5, locals.var_sp_s_pc__blk1446_dn6, locals.var_sp_s_pc__blk1446_dn7, locals.var_sp_s_pc__blk1446_dn8,)
    }
};
        locals.var_sp_s_pc__blk1446 = assign52120_e67123;
        locals.var_sp_s_pc__blk1446_dn5 = assign52120_e67123_d_n5;
        locals.var_sp_s_pc__blk1446_dn6 = assign52120_e67123_d_n6;
        locals.var_sp_s_pc__blk1446_dn7 = assign52120_e67123_d_n7;
        locals.var_sp_s_pc__blk1446_dn8 = assign52120_e67123_d_n8;
        locals.var_sp_s_pc__blk1446_rv = 0.0;

        let (assign52130_e67152, assign52130_e67152_d_n5, assign52130_e67152_d_n6, assign52130_e67152_d_n7, assign52130_e67152_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52130_e67132: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign52130_e67136: f64 = (locals.var_sp_s_delta1__blk1442 + locals.var_sp_s_x0__blk1455);
        let assign52130_e67138: f64 = (assign52130_e67136 - 1.0);
        let assign52130_e67140: f64 = (assign52130_e67138 + locals.var_sp_s_delta0__blk1441);
        let assign52130_e67144: f64 = (locals.var_sp_s_x0__blk1455 + 1.0);
        let assign52130_e67146: f64 = (assign52130_e67144 + locals.var_sp_s_xi0__blk1443);
        let assign52130_e67147: f64 = (locals.var_delta_nd__blk1392 * assign52130_e67146);
        let assign52130_e67148: f64 = (assign52130_e67140 - assign52130_e67147);
        let assign52130_e67149: f64 = (locals.var_gf2__blk1308 * assign52130_e67148);
        let assign52130_e67150: f64 = (assign52130_e67132 - assign52130_e67149);
        (assign52130_e67150, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign52130_e67148) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn5 + locals.var_sp_s_x0__blk1455_dn5) + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_nd__blk1392_dn5 * assign52130_e67146) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_x0__blk1455_dn5 + locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign52130_e67148) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn6 + locals.var_sp_s_x0__blk1455_dn6) + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_nd__blk1392_dn6 * assign52130_e67146) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_x0__blk1455_dn6 + locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign52130_e67148) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn7 + locals.var_sp_s_x0__blk1455_dn7) + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_nd__blk1392_dn7 * assign52130_e67146) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_x0__blk1455_dn7 + locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign52130_e67148) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn8 + locals.var_sp_s_x0__blk1455_dn8) + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_nd__blk1392_dn8 * assign52130_e67146) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_x0__blk1455_dn8 + locals.var_sp_s_xi0__blk1443_dn8))))))),)
    } else {
        (locals.var_sp_s_qc__blk1447, locals.var_sp_s_qc__blk1447_dn5, locals.var_sp_s_qc__blk1447_dn6, locals.var_sp_s_qc__blk1447_dn7, locals.var_sp_s_qc__blk1447_dn8,)
    }
};
        locals.var_sp_s_qc__blk1447 = assign52130_e67152;
        locals.var_sp_s_qc__blk1447_dn5 = assign52130_e67152_d_n5;
        locals.var_sp_s_qc__blk1447_dn6 = assign52130_e67152_d_n6;
        locals.var_sp_s_qc__blk1447_dn7 = assign52130_e67152_d_n7;
        locals.var_sp_s_qc__blk1447_dn8 = assign52130_e67152_d_n8;
        locals.var_sp_s_qc__blk1447_rv = 0.0;

        let (assign52140_e67171, assign52140_e67171_d_n5, assign52140_e67171_d_n6, assign52140_e67171_d_n7, assign52140_e67171_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52140_e67163: f64 = (locals.var_sp_s_delta1__blk1442 + locals.var_sp_s_delta0__blk1441);
        let assign52140_e67166: f64 = (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445);
        let assign52140_e67167: f64 = (assign52140_e67163 - assign52140_e67166);
        let assign52140_e67168: f64 = (locals.var_gf2__blk1308 * assign52140_e67167);
        let assign52140_e67169: f64 = (2.0 - assign52140_e67168);
        (assign52140_e67169, (-((locals.var_gf2__blk1308_dn5 * assign52140_e67167) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn5 + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_nd__blk1392_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn5)))))), (-((locals.var_gf2__blk1308_dn6 * assign52140_e67167) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn6 + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_nd__blk1392_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn6)))))), (-((locals.var_gf2__blk1308_dn7 * assign52140_e67167) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn7 + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_nd__blk1392_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn7)))))), (-((locals.var_gf2__blk1308_dn8 * assign52140_e67167) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn8 + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_nd__blk1392_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn8)))))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign52140_e67171;
        locals.var_sp_s_temp__blk1431_dn5 = assign52140_e67171_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign52140_e67171_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign52140_e67171_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign52140_e67171_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

        let (assign52150_e67188, assign52150_e67188_d_n5, assign52150_e67188_d_n6, assign52150_e67188_d_n7, assign52150_e67188_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52150_e67180: f64 = (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446);
        let assign52150_e67184: f64 = (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431);
        let assign52150_e67185: f64 = (2.0 * assign52150_e67184);
        let assign52150_e67186: f64 = (assign52150_e67180 - assign52150_e67185);
        (assign52150_e67186, (((locals.var_sp_s_pc__blk1446_dn5 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn5)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn5)))), (((locals.var_sp_s_pc__blk1446_dn6 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn6)))), (((locals.var_sp_s_pc__blk1446_dn7 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn7)))), (((locals.var_sp_s_pc__blk1446_dn8 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn8)))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign52150_e67188;
        locals.var_sp_s_temp__blk1431_dn5 = assign52150_e67188_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign52150_e67188_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign52150_e67188_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign52150_e67188_d_n8;
        locals.var_sp_s_temp__blk1431_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_47(
        locals: &mut StampLocals,
    ) {
        let (assign52160_e67206, assign52160_e67206_d_n5, assign52160_e67206_d_n6, assign52160_e67206_d_n7, assign52160_e67206_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52160_e67200: f64 = (locals.var_sp_s_temp__blk1431).sqrt();
        let assign52160_e67201: f64 = (locals.var_sp_s_pc__blk1446 + assign52160_e67200);
        let assign52160_e67202: f64 = (locals.var_sp_s_qc__blk1447 / assign52160_e67201);
        let assign52160_e67203: f64 = (2.0 * assign52160_e67202);
        let assign52160_e67204: f64 = (locals.var_sp_s_x0__blk1455 + assign52160_e67203);
        (assign52160_e67204, (locals.var_sp_s_x0__blk1455_dn5 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn5 * assign52160_e67201) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn5 + (locals.var_sp_s_temp__blk1431_dn5 / (2.0 * assign52160_e67200))))) / (assign52160_e67201 * assign52160_e67201)))), (locals.var_sp_s_x0__blk1455_dn6 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn6 * assign52160_e67201) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn6 + (locals.var_sp_s_temp__blk1431_dn6 / (2.0 * assign52160_e67200))))) / (assign52160_e67201 * assign52160_e67201)))), (locals.var_sp_s_x0__blk1455_dn7 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn7 * assign52160_e67201) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn7 + (locals.var_sp_s_temp__blk1431_dn7 / (2.0 * assign52160_e67200))))) / (assign52160_e67201 * assign52160_e67201)))), (locals.var_sp_s_x0__blk1455_dn8 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn8 * assign52160_e67201) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn8 + (locals.var_sp_s_temp__blk1431_dn8 / (2.0 * assign52160_e67200))))) / (assign52160_e67201 * assign52160_e67201)))),)
    } else {
        (locals.var_x_d__blk1393, locals.var_x_d__blk1393_dn5, locals.var_x_d__blk1393_dn6, locals.var_x_d__blk1393_dn7, locals.var_x_d__blk1393_dn8,)
    }
};
        locals.var_x_d__blk1393 = assign52160_e67206;
        locals.var_x_d__blk1393_dn5 = assign52160_e67206_d_n5;
        locals.var_x_d__blk1393_dn6 = assign52160_e67206_d_n6;
        locals.var_x_d__blk1393_dn7 = assign52160_e67206_d_n7;
        locals.var_x_d__blk1393_dn8 = assign52160_e67206_d_n8;
        locals.var_x_d__blk1393_rv = 0.0;

        let (assign52170_e67214, assign52170_e67214_d_n5, assign52170_e67214_d_n6, assign52170_e67214_d_n7, assign52170_e67214_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52170_e67212: f64 = (locals.var_x_d__blk1393 - locals.var_x_s__blk1346);
        (assign52170_e67212, (locals.var_x_d__blk1393_dn5 - locals.var_x_s__blk1346_dn5), (locals.var_x_d__blk1393_dn6 - locals.var_x_s__blk1346_dn6), (locals.var_x_d__blk1393_dn7 - locals.var_x_s__blk1346_dn7), (locals.var_x_d__blk1393_dn8 - locals.var_x_s__blk1346_dn8),)
    } else {
        (locals.var_x_ds__blk1394, locals.var_x_ds__blk1394_dn5, locals.var_x_ds__blk1394_dn6, locals.var_x_ds__blk1394_dn7, locals.var_x_ds__blk1394_dn8,)
    }
};
        locals.var_x_ds__blk1394 = assign52170_e67214;
        locals.var_x_ds__blk1394_dn5 = assign52170_e67214_d_n5;
        locals.var_x_ds__blk1394_dn6 = assign52170_e67214_d_n6;
        locals.var_x_ds__blk1394_dn7 = assign52170_e67214_d_n7;
        locals.var_x_ds__blk1394_dn8 = assign52170_e67214_d_n8;
        locals.var_x_ds__blk1394_rv = 0.0;

        let assign52180_e67217: f64 = if locals.var_x_ds__blk1394 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1495 = assign52180_e67217;
        locals.var_guard1495_rv = 0.0;

        let (assign52190_e67245, assign52190_e67245_d_n5, assign52190_e67245_d_n6, assign52190_e67245_d_n7, assign52190_e67245_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign52190_e67226: f64 = (locals.var_xg__blk1326 - locals.var_x_s__blk1346);
        let assign52190_e67227: f64 = (2.0 * assign52190_e67226);
        let assign52190_e67231: f64 = (1.0 - locals.var_es__blk1352);
        let assign52190_e67234: f64 = (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391);
        let assign52190_e67235: f64 = (assign52190_e67231 + assign52190_e67234);
        let assign52190_e67239: f64 = (1.0 + locals.var_xi1s__blk1349);
        let assign52190_e67240: f64 = (locals.var_delta_nd__blk1392 * assign52190_e67239);
        let assign52190_e67241: f64 = (assign52190_e67235 - assign52190_e67240);
        let assign52190_e67242: f64 = (locals.var_gf2__blk1308 * assign52190_e67241);
        let assign52190_e67243: f64 = (assign52190_e67227 + assign52190_e67242);
        (assign52190_e67243, ((2.0 * (locals.var_xg__blk1326_dn5 - locals.var_x_s__blk1346_dn5)) + ((locals.var_gf2__blk1308_dn5 * assign52190_e67241) + (locals.var_gf2__blk1308 * (((-locals.var_es__blk1352_dn5) + ((locals.var_delta_1s__blk1351_dn5 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn5))) - ((locals.var_delta_nd__blk1392_dn5 * assign52190_e67239) + (locals.var_delta_nd__blk1392 * locals.var_xi1s__blk1349_dn5)))))), ((2.0 * (locals.var_xg__blk1326_dn6 - locals.var_x_s__blk1346_dn6)) + ((locals.var_gf2__blk1308_dn6 * assign52190_e67241) + (locals.var_gf2__blk1308 * (((-locals.var_es__blk1352_dn6) + ((locals.var_delta_1s__blk1351_dn6 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn6))) - ((locals.var_delta_nd__blk1392_dn6 * assign52190_e67239) + (locals.var_delta_nd__blk1392 * locals.var_xi1s__blk1349_dn6)))))), ((2.0 * (locals.var_xg__blk1326_dn7 - locals.var_x_s__blk1346_dn7)) + ((locals.var_gf2__blk1308_dn7 * assign52190_e67241) + (locals.var_gf2__blk1308 * (((-locals.var_es__blk1352_dn7) + ((locals.var_delta_1s__blk1351_dn7 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn7))) - ((locals.var_delta_nd__blk1392_dn7 * assign52190_e67239) + (locals.var_delta_nd__blk1392 * locals.var_xi1s__blk1349_dn7)))))), ((2.0 * (locals.var_xg__blk1326_dn8 - locals.var_x_s__blk1346_dn8)) + ((locals.var_gf2__blk1308_dn8 * assign52190_e67241) + (locals.var_gf2__blk1308 * (((-locals.var_es__blk1352_dn8) + ((locals.var_delta_1s__blk1351_dn8 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn8))) - ((locals.var_delta_nd__blk1392_dn8 * assign52190_e67239) + (locals.var_delta_nd__blk1392 * locals.var_xi1s__blk1349_dn8)))))),)
    } else {
        (locals.var_pc__blk1395, locals.var_pc__blk1395_dn5, locals.var_pc__blk1395_dn6, locals.var_pc__blk1395_dn7, locals.var_pc__blk1395_dn8,)
    }
};
        locals.var_pc__blk1395 = assign52190_e67245;
        locals.var_pc__blk1395_dn5 = assign52190_e67245_d_n5;
        locals.var_pc__blk1395_dn6 = assign52190_e67245_d_n6;
        locals.var_pc__blk1395_dn7 = assign52190_e67245_d_n7;
        locals.var_pc__blk1395_dn8 = assign52190_e67245_d_n8;
        locals.var_pc__blk1395_rv = 0.0;

        let (assign52200_e67259, assign52200_e67259_d_n5, assign52200_e67259_d_n6, assign52200_e67259_d_n7, assign52200_e67259_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign52200_e67254: f64 = (1.0 - locals.var_k_ds__blk1391);
        let assign52200_e67255: f64 = (locals.var_gf2__blk1308 * assign52200_e67254);
        let assign52200_e67257: f64 = (assign52200_e67255 * locals.var_ds__blk1353);
        (assign52200_e67257, ((((locals.var_gf2__blk1308_dn5 * assign52200_e67254) + (locals.var_gf2__blk1308 * (-locals.var_k_ds__blk1391_dn5))) * locals.var_ds__blk1353) + (assign52200_e67255 * locals.var_ds__blk1353_dn5)), ((((locals.var_gf2__blk1308_dn6 * assign52200_e67254) + (locals.var_gf2__blk1308 * (-locals.var_k_ds__blk1391_dn6))) * locals.var_ds__blk1353) + (assign52200_e67255 * locals.var_ds__blk1353_dn6)), ((((locals.var_gf2__blk1308_dn7 * assign52200_e67254) + (locals.var_gf2__blk1308 * (-locals.var_k_ds__blk1391_dn7))) * locals.var_ds__blk1353) + (assign52200_e67255 * locals.var_ds__blk1353_dn7)), ((((locals.var_gf2__blk1308_dn8 * assign52200_e67254) + (locals.var_gf2__blk1308 * (-locals.var_k_ds__blk1391_dn8))) * locals.var_ds__blk1353) + (assign52200_e67255 * locals.var_ds__blk1353_dn8)),)
    } else {
        (locals.var_qc__blk1396, locals.var_qc__blk1396_dn5, locals.var_qc__blk1396_dn6, locals.var_qc__blk1396_dn7, locals.var_qc__blk1396_dn8,)
    }
};
        locals.var_qc__blk1396 = assign52200_e67259;
        locals.var_qc__blk1396_dn5 = assign52200_e67259_d_n5;
        locals.var_qc__blk1396_dn6 = assign52200_e67259_d_n6;
        locals.var_qc__blk1396_dn7 = assign52200_e67259_d_n7;
        locals.var_qc__blk1396_dn8 = assign52200_e67259_d_n8;
        locals.var_qc__blk1396_rv = 0.0;

        let (assign52210_e67279, assign52210_e67279_d_n5, assign52210_e67279_d_n6, assign52210_e67279_d_n7, assign52210_e67279_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign52210_e67270: f64 = (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391);
        let assign52210_e67271: f64 = (locals.var_es__blk1352 + assign52210_e67270);
        let assign52210_e67274: f64 = (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350);
        let assign52210_e67275: f64 = (assign52210_e67271 - assign52210_e67274);
        let assign52210_e67276: f64 = (locals.var_gf2__blk1308 * assign52210_e67275);
        let assign52210_e67277: f64 = (2.0 - assign52210_e67276);
        (assign52210_e67277, (-((locals.var_gf2__blk1308_dn5 * assign52210_e67275) + (locals.var_gf2__blk1308 * ((locals.var_es__blk1352_dn5 + ((locals.var_delta_1s__blk1351_dn5 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn5))) - ((locals.var_delta_nd__blk1392_dn5 * locals.var_xi2s__blk1350) + (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350_dn5)))))), (-((locals.var_gf2__blk1308_dn6 * assign52210_e67275) + (locals.var_gf2__blk1308 * ((locals.var_es__blk1352_dn6 + ((locals.var_delta_1s__blk1351_dn6 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn6))) - ((locals.var_delta_nd__blk1392_dn6 * locals.var_xi2s__blk1350) + (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350_dn6)))))), (-((locals.var_gf2__blk1308_dn7 * assign52210_e67275) + (locals.var_gf2__blk1308 * ((locals.var_es__blk1352_dn7 + ((locals.var_delta_1s__blk1351_dn7 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn7))) - ((locals.var_delta_nd__blk1392_dn7 * locals.var_xi2s__blk1350) + (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350_dn7)))))), (-((locals.var_gf2__blk1308_dn8 * assign52210_e67275) + (locals.var_gf2__blk1308 * ((locals.var_es__blk1352_dn8 + ((locals.var_delta_1s__blk1351_dn8 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn8))) - ((locals.var_delta_nd__blk1392_dn8 * locals.var_xi2s__blk1350) + (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350_dn8)))))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52210_e67279;
        locals.var_temp__blk936_dn5 = assign52210_e67279_d_n5;
        locals.var_temp__blk936_dn6 = assign52210_e67279_d_n6;
        locals.var_temp__blk936_dn7 = assign52210_e67279_d_n7;
        locals.var_temp__blk936_dn8 = assign52210_e67279_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign52220_e67295, assign52220_e67295_d_n5, assign52220_e67295_d_n6, assign52220_e67295_d_n7, assign52220_e67295_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign52220_e67287: f64 = (locals.var_pc__blk1395 * locals.var_pc__blk1395);
        let assign52220_e67291: f64 = (locals.var_temp__blk936 * locals.var_qc__blk1396);
        let assign52220_e67292: f64 = (2.0 * assign52220_e67291);
        let assign52220_e67293: f64 = (assign52220_e67287 - assign52220_e67292);
        (assign52220_e67293, (((locals.var_pc__blk1395_dn5 * locals.var_pc__blk1395) + (locals.var_pc__blk1395 * locals.var_pc__blk1395_dn5)) - (2.0 * ((locals.var_temp__blk936_dn5 * locals.var_qc__blk1396) + (locals.var_temp__blk936 * locals.var_qc__blk1396_dn5)))), (((locals.var_pc__blk1395_dn6 * locals.var_pc__blk1395) + (locals.var_pc__blk1395 * locals.var_pc__blk1395_dn6)) - (2.0 * ((locals.var_temp__blk936_dn6 * locals.var_qc__blk1396) + (locals.var_temp__blk936 * locals.var_qc__blk1396_dn6)))), (((locals.var_pc__blk1395_dn7 * locals.var_pc__blk1395) + (locals.var_pc__blk1395 * locals.var_pc__blk1395_dn7)) - (2.0 * ((locals.var_temp__blk936_dn7 * locals.var_qc__blk1396) + (locals.var_temp__blk936 * locals.var_qc__blk1396_dn7)))), (((locals.var_pc__blk1395_dn8 * locals.var_pc__blk1395) + (locals.var_pc__blk1395 * locals.var_pc__blk1395_dn8)) - (2.0 * ((locals.var_temp__blk936_dn8 * locals.var_qc__blk1396) + (locals.var_temp__blk936 * locals.var_qc__blk1396_dn8)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52220_e67295;
        locals.var_temp__blk936_dn5 = assign52220_e67295_d_n5;
        locals.var_temp__blk936_dn6 = assign52220_e67295_d_n6;
        locals.var_temp__blk936_dn7 = assign52220_e67295_d_n7;
        locals.var_temp__blk936_dn8 = assign52220_e67295_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign52230_e67310, assign52230_e67310_d_n5, assign52230_e67310_d_n6, assign52230_e67310_d_n7, assign52230_e67310_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign52230_e67305: f64 = (locals.var_temp__blk936).sqrt();
        let assign52230_e67306: f64 = (locals.var_pc__blk1395 + assign52230_e67305);
        let assign52230_e67307: f64 = (locals.var_qc__blk1396 / assign52230_e67306);
        let assign52230_e67308: f64 = (2.0 * assign52230_e67307);
        (assign52230_e67308, (2.0 * (((locals.var_qc__blk1396_dn5 * assign52230_e67306) - (locals.var_qc__blk1396 * (locals.var_pc__blk1395_dn5 + (locals.var_temp__blk936_dn5 / (2.0 * assign52230_e67305))))) / (assign52230_e67306 * assign52230_e67306))), (2.0 * (((locals.var_qc__blk1396_dn6 * assign52230_e67306) - (locals.var_qc__blk1396 * (locals.var_pc__blk1395_dn6 + (locals.var_temp__blk936_dn6 / (2.0 * assign52230_e67305))))) / (assign52230_e67306 * assign52230_e67306))), (2.0 * (((locals.var_qc__blk1396_dn7 * assign52230_e67306) - (locals.var_qc__blk1396 * (locals.var_pc__blk1395_dn7 + (locals.var_temp__blk936_dn7 / (2.0 * assign52230_e67305))))) / (assign52230_e67306 * assign52230_e67306))), (2.0 * (((locals.var_qc__blk1396_dn8 * assign52230_e67306) - (locals.var_qc__blk1396 * (locals.var_pc__blk1395_dn8 + (locals.var_temp__blk936_dn8 / (2.0 * assign52230_e67305))))) / (assign52230_e67306 * assign52230_e67306))),)
    } else {
        (locals.var_x_ds__blk1394, locals.var_x_ds__blk1394_dn5, locals.var_x_ds__blk1394_dn6, locals.var_x_ds__blk1394_dn7, locals.var_x_ds__blk1394_dn8,)
    }
};
        locals.var_x_ds__blk1394 = assign52230_e67310;
        locals.var_x_ds__blk1394_dn5 = assign52230_e67310_d_n5;
        locals.var_x_ds__blk1394_dn6 = assign52230_e67310_d_n6;
        locals.var_x_ds__blk1394_dn7 = assign52230_e67310_d_n7;
        locals.var_x_ds__blk1394_dn8 = assign52230_e67310_d_n8;
        locals.var_x_ds__blk1394_rv = 0.0;

        let (assign52240_e67320, assign52240_e67320_d_n5, assign52240_e67320_d_n6, assign52240_e67320_d_n7, assign52240_e67320_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign52240_e67318: f64 = (locals.var_x_s__blk1346 + locals.var_x_ds__blk1394);
        (assign52240_e67318, (locals.var_x_s__blk1346_dn5 + locals.var_x_ds__blk1394_dn5), (locals.var_x_s__blk1346_dn6 + locals.var_x_ds__blk1394_dn6), (locals.var_x_s__blk1346_dn7 + locals.var_x_ds__blk1394_dn7), (locals.var_x_s__blk1346_dn8 + locals.var_x_ds__blk1394_dn8),)
    } else {
        (locals.var_x_d__blk1393, locals.var_x_d__blk1393_dn5, locals.var_x_d__blk1393_dn6, locals.var_x_d__blk1393_dn7, locals.var_x_d__blk1393_dn8,)
    }
};
        locals.var_x_d__blk1393 = assign52240_e67320;
        locals.var_x_d__blk1393_dn5 = assign52240_e67320_d_n5;
        locals.var_x_d__blk1393_dn6 = assign52240_e67320_d_n6;
        locals.var_x_d__blk1393_dn7 = assign52240_e67320_d_n7;
        locals.var_x_d__blk1393_dn8 = assign52240_e67320_d_n8;
        locals.var_x_d__blk1393_rv = 0.0;

        let (assign52250_e67328, assign52250_e67328_d_n5, assign52250_e67328_d_n6, assign52250_e67328_d_n7, assign52250_e67328_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52250_e67326: f64 = (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322);
        (assign52250_e67326, ((locals.var_x_ds__blk1394_dn5 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn5)), ((locals.var_x_ds__blk1394_dn6 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn6)), ((locals.var_x_ds__blk1394_dn7 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn7)), ((locals.var_x_ds__blk1394_dn8 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_dps__blk1397, locals.var_dps__blk1397_dn5, locals.var_dps__blk1397_dn6, locals.var_dps__blk1397_dn7, locals.var_dps__blk1397_dn8,)
    }
};
        locals.var_dps__blk1397 = assign52250_e67328;
        locals.var_dps__blk1397_dn5 = assign52250_e67328_d_n5;
        locals.var_dps__blk1397_dn6 = assign52250_e67328_d_n6;
        locals.var_dps__blk1397_dn7 = assign52250_e67328_d_n7;
        locals.var_dps__blk1397_dn8 = assign52250_e67328_d_n8;
        locals.var_dps__blk1397_rv = 0.0;

        let (assign52260_e67342, assign52260_e67342_d_n5, assign52260_e67342_d_n6, assign52260_e67342_d_n7, assign52260_e67342_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52260_e67334: f64 = (locals.var_x_d__blk1393 * locals.var_x_d__blk1393);
        let assign52260_e67338: f64 = (locals.var_x_d__blk1393 * locals.var_x_d__blk1393);
        let assign52260_e67339: f64 = (2.0 + assign52260_e67338);
        let assign52260_e67340: f64 = (assign52260_e67334 / assign52260_e67339);
        (assign52260_e67340, (((((locals.var_x_d__blk1393_dn5 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn5)) * assign52260_e67339) - (assign52260_e67334 * ((locals.var_x_d__blk1393_dn5 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn5)))) / (assign52260_e67339 * assign52260_e67339)), (((((locals.var_x_d__blk1393_dn6 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn6)) * assign52260_e67339) - (assign52260_e67334 * ((locals.var_x_d__blk1393_dn6 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn6)))) / (assign52260_e67339 * assign52260_e67339)), (((((locals.var_x_d__blk1393_dn7 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn7)) * assign52260_e67339) - (assign52260_e67334 * ((locals.var_x_d__blk1393_dn7 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn7)))) / (assign52260_e67339 * assign52260_e67339)), (((((locals.var_x_d__blk1393_dn8 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn8)) * assign52260_e67339) - (assign52260_e67334 * ((locals.var_x_d__blk1393_dn8 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn8)))) / (assign52260_e67339 * assign52260_e67339)),)
    } else {
        (locals.var_xi0d__blk1398, locals.var_xi0d__blk1398_dn5, locals.var_xi0d__blk1398_dn6, locals.var_xi0d__blk1398_dn7, locals.var_xi0d__blk1398_dn8,)
    }
};
        locals.var_xi0d__blk1398 = assign52260_e67342;
        locals.var_xi0d__blk1398_dn5 = assign52260_e67342_d_n5;
        locals.var_xi0d__blk1398_dn6 = assign52260_e67342_d_n6;
        locals.var_xi0d__blk1398_dn7 = assign52260_e67342_d_n7;
        locals.var_xi0d__blk1398_dn8 = assign52260_e67342_d_n8;
        locals.var_xi0d__blk1398_rv = 0.0;

        let assign52270_e67345: f64 = if locals.var_x_d__blk1393 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1496 = assign52270_e67345;
        locals.var_guard1496_rv = 0.0;

        let (assign52280_e67355, assign52280_e67355_d_n5, assign52280_e67355_d_n6, assign52280_e67355_d_n7, assign52280_e67355_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) {
        let assign52280_e67352: f64 = (-locals.var_x_d__blk1393);
        let assign52280_e67353: f64 = (assign52280_e67352).exp();
        (assign52280_e67353, (assign52280_e67353 * (-locals.var_x_d__blk1393_dn5)), (assign52280_e67353 * (-locals.var_x_d__blk1393_dn6)), (assign52280_e67353 * (-locals.var_x_d__blk1393_dn7)), (assign52280_e67353 * (-locals.var_x_d__blk1393_dn8)),)
    } else {
        (locals.var_ed__blk1399, locals.var_ed__blk1399_dn5, locals.var_ed__blk1399_dn6, locals.var_ed__blk1399_dn7, locals.var_ed__blk1399_dn8,)
    }
};
        locals.var_ed__blk1399 = assign52280_e67355;
        locals.var_ed__blk1399_dn5 = assign52280_e67355_d_n5;
        locals.var_ed__blk1399_dn6 = assign52280_e67355_d_n6;
        locals.var_ed__blk1399_dn7 = assign52280_e67355_d_n7;
        locals.var_ed__blk1399_dn8 = assign52280_e67355_d_n8;
        locals.var_ed__blk1399_rv = 0.0;

        let assign52290_e67358: f64 = if locals.var_x_d__blk1393 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1497 = assign52290_e67358;
        locals.var_guard1497_rv = 0.0;

        let (assign52300_e67384, assign52300_e67384_d_n5, assign52300_e67384_d_n6, assign52300_e67384_d_n7, assign52300_e67384_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign52300_e67369: f64 = (locals.var_x_d__blk1393 * locals.var_x_d__blk1393);
        let assign52300_e67376: f64 = (0.25 * locals.var_x_d__blk1393);
        let assign52300_e67377: f64 = (1.0 - assign52300_e67376);
        let assign52300_e67378: f64 = (locals.var_x_d__blk1393 * assign52300_e67377);
        let assign52300_e67379: f64 = (0.3333333333333333 * assign52300_e67378);
        let assign52300_e67380: f64 = (1.0 - assign52300_e67379);
        let assign52300_e67381: f64 = (assign52300_e67369 * assign52300_e67380);
        let assign52300_e67382: f64 = (0.5 * assign52300_e67381);
        (assign52300_e67382, (0.5 * ((((locals.var_x_d__blk1393_dn5 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn5)) * assign52300_e67380) + (assign52300_e67369 * (-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn5 * assign52300_e67377) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn5))))))))), (0.5 * ((((locals.var_x_d__blk1393_dn6 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn6)) * assign52300_e67380) + (assign52300_e67369 * (-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn6 * assign52300_e67377) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn6))))))))), (0.5 * ((((locals.var_x_d__blk1393_dn7 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn7)) * assign52300_e67380) + (assign52300_e67369 * (-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn7 * assign52300_e67377) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn7))))))))), (0.5 * ((((locals.var_x_d__blk1393_dn8 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn8)) * assign52300_e67380) + (assign52300_e67369 * (-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn8 * assign52300_e67377) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn8))))))))),)
    } else {
        (locals.var_pd__blk1400, locals.var_pd__blk1400_dn5, locals.var_pd__blk1400_dn6, locals.var_pd__blk1400_dn7, locals.var_pd__blk1400_dn8,)
    }
};
        locals.var_pd__blk1400 = assign52300_e67384;
        locals.var_pd__blk1400_dn5 = assign52300_e67384_d_n5;
        locals.var_pd__blk1400_dn6 = assign52300_e67384_d_n6;
        locals.var_pd__blk1400_dn7 = assign52300_e67384_d_n7;
        locals.var_pd__blk1400_dn8 = assign52300_e67384_d_n8;
        locals.var_pd__blk1400_rv = 0.0;

        let (assign52310_e67405, assign52310_e67405_d_n5, assign52310_e67405_d_n6, assign52310_e67405_d_n7, assign52310_e67405_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign52310_e67398: f64 = (0.25 * locals.var_x_d__blk1393);
        let assign52310_e67399: f64 = (1.0 - assign52310_e67398);
        let assign52310_e67400: f64 = (locals.var_x_d__blk1393 * assign52310_e67399);
        let assign52310_e67401: f64 = (0.3333333333333333 * assign52310_e67400);
        let assign52310_e67402: f64 = (1.0 - assign52310_e67401);
        let assign52310_e67403: f64 = (assign52310_e67402).sqrt();
        (assign52310_e67403, ((-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn5 * assign52310_e67399) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn5)))))) / (2.0 * assign52310_e67403)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn6 * assign52310_e67399) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn6)))))) / (2.0 * assign52310_e67403)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn7 * assign52310_e67399) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn7)))))) / (2.0 * assign52310_e67403)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn8 * assign52310_e67399) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn8)))))) / (2.0 * assign52310_e67403)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52310_e67405;
        locals.var_temp__blk936_dn5 = assign52310_e67405_d_n5;
        locals.var_temp__blk936_dn6 = assign52310_e67405_d_n6;
        locals.var_temp__blk936_dn7 = assign52310_e67405_d_n7;
        locals.var_temp__blk936_dn8 = assign52310_e67405_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign52320_e67419, assign52320_e67419_d_n5, assign52320_e67419_d_n6, assign52320_e67419_d_n7, assign52320_e67419_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign52320_e67416: f64 = (locals.var_x_d__blk1393 * locals.var_temp__blk936);
        let assign52320_e67417: f64 = (0.7071067811865475 * assign52320_e67416);
        (assign52320_e67417, (0.7071067811865475 * ((locals.var_x_d__blk1393_dn5 * locals.var_temp__blk936) + (locals.var_x_d__blk1393 * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_d__blk1393_dn6 * locals.var_temp__blk936) + (locals.var_x_d__blk1393 * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_d__blk1393_dn7 * locals.var_temp__blk936) + (locals.var_x_d__blk1393 * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_d__blk1393_dn8 * locals.var_temp__blk936) + (locals.var_x_d__blk1393 * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_sqd__blk1401, locals.var_sqd__blk1401_dn5, locals.var_sqd__blk1401_dn6, locals.var_sqd__blk1401_dn7, locals.var_sqd__blk1401_dn8,)
    }
};
        locals.var_sqd__blk1401 = assign52320_e67419;
        locals.var_sqd__blk1401_dn5 = assign52320_e67419_d_n5;
        locals.var_sqd__blk1401_dn6 = assign52320_e67419_d_n6;
        locals.var_sqd__blk1401_dn7 = assign52320_e67419_d_n7;
        locals.var_sqd__blk1401_dn8 = assign52320_e67419_d_n8;
        locals.var_sqd__blk1401_rv = 0.0;

        let (assign52330_e67443, assign52330_e67443_d_n5, assign52330_e67443_d_n6, assign52330_e67443_d_n7, assign52330_e67443_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign52330_e67429: f64 = (0.16666666666666666 * locals.var_delta_nd__blk1392);
        let assign52330_e67431: f64 = (assign52330_e67429 * locals.var_x_d__blk1393);
        let assign52330_e67433: f64 = (assign52330_e67431 * locals.var_x_d__blk1393);
        let assign52330_e67435: f64 = (assign52330_e67433 * locals.var_x_d__blk1393);
        let assign52330_e67439: f64 = (1.75 * locals.var_x_d__blk1393);
        let assign52330_e67440: f64 = (1.0 + assign52330_e67439);
        let assign52330_e67441: f64 = (assign52330_e67435 * assign52330_e67440);
        (assign52330_e67441, (((((((((0.16666666666666666 * locals.var_delta_nd__blk1392_dn5) * locals.var_x_d__blk1393) + (assign52330_e67429 * locals.var_x_d__blk1393_dn5)) * locals.var_x_d__blk1393) + (assign52330_e67431 * locals.var_x_d__blk1393_dn5)) * locals.var_x_d__blk1393) + (assign52330_e67433 * locals.var_x_d__blk1393_dn5)) * assign52330_e67440) + (assign52330_e67435 * (1.75 * locals.var_x_d__blk1393_dn5))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1392_dn6) * locals.var_x_d__blk1393) + (assign52330_e67429 * locals.var_x_d__blk1393_dn6)) * locals.var_x_d__blk1393) + (assign52330_e67431 * locals.var_x_d__blk1393_dn6)) * locals.var_x_d__blk1393) + (assign52330_e67433 * locals.var_x_d__blk1393_dn6)) * assign52330_e67440) + (assign52330_e67435 * (1.75 * locals.var_x_d__blk1393_dn6))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1392_dn7) * locals.var_x_d__blk1393) + (assign52330_e67429 * locals.var_x_d__blk1393_dn7)) * locals.var_x_d__blk1393) + (assign52330_e67431 * locals.var_x_d__blk1393_dn7)) * locals.var_x_d__blk1393) + (assign52330_e67433 * locals.var_x_d__blk1393_dn7)) * assign52330_e67440) + (assign52330_e67435 * (1.75 * locals.var_x_d__blk1393_dn7))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1392_dn8) * locals.var_x_d__blk1393) + (assign52330_e67429 * locals.var_x_d__blk1393_dn8)) * locals.var_x_d__blk1393) + (assign52330_e67431 * locals.var_x_d__blk1393_dn8)) * locals.var_x_d__blk1393) + (assign52330_e67433 * locals.var_x_d__blk1393_dn8)) * assign52330_e67440) + (assign52330_e67435 * (1.75 * locals.var_x_d__blk1393_dn8))),)
    } else {
        (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8,)
    }
};
        locals.var_dd__blk1402 = assign52330_e67443;
        locals.var_dd__blk1402_dn5 = assign52330_e67443_d_n5;
        locals.var_dd__blk1402_dn6 = assign52330_e67443_d_n6;
        locals.var_dd__blk1402_dn7 = assign52330_e67443_d_n7;
        locals.var_dd__blk1402_dn8 = assign52330_e67443_d_n8;
        locals.var_dd__blk1402_rv = 0.0;

        let (assign52340_e67458, assign52340_e67458_d_n5, assign52340_e67458_d_n6, assign52340_e67458_d_n7, assign52340_e67458_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 == 0.0)) {
        let assign52340_e67454: f64 = (locals.var_x_d__blk1393 - 1.0);
        let assign52340_e67456: f64 = (assign52340_e67454 + locals.var_ed__blk1399);
        (assign52340_e67456, (locals.var_x_d__blk1393_dn5 + locals.var_ed__blk1399_dn5), (locals.var_x_d__blk1393_dn6 + locals.var_ed__blk1399_dn6), (locals.var_x_d__blk1393_dn7 + locals.var_ed__blk1399_dn7), (locals.var_x_d__blk1393_dn8 + locals.var_ed__blk1399_dn8),)
    } else {
        (locals.var_pd__blk1400, locals.var_pd__blk1400_dn5, locals.var_pd__blk1400_dn6, locals.var_pd__blk1400_dn7, locals.var_pd__blk1400_dn8,)
    }
};
        locals.var_pd__blk1400 = assign52340_e67458;
        locals.var_pd__blk1400_dn5 = assign52340_e67458_d_n5;
        locals.var_pd__blk1400_dn6 = assign52340_e67458_d_n6;
        locals.var_pd__blk1400_dn7 = assign52340_e67458_d_n7;
        locals.var_pd__blk1400_dn8 = assign52340_e67458_d_n8;
        locals.var_pd__blk1400_rv = 0.0;

        let (assign52350_e67470, assign52350_e67470_d_n5, assign52350_e67470_d_n6, assign52350_e67470_d_n7, assign52350_e67470_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 == 0.0)) {
        let assign52350_e67468: f64 = (locals.var_pd__blk1400).sqrt();
        (assign52350_e67468, (locals.var_pd__blk1400_dn5 / (2.0 * assign52350_e67468)), (locals.var_pd__blk1400_dn6 / (2.0 * assign52350_e67468)), (locals.var_pd__blk1400_dn7 / (2.0 * assign52350_e67468)), (locals.var_pd__blk1400_dn8 / (2.0 * assign52350_e67468)),)
    } else {
        (locals.var_sqd__blk1401, locals.var_sqd__blk1401_dn5, locals.var_sqd__blk1401_dn6, locals.var_sqd__blk1401_dn7, locals.var_sqd__blk1401_dn8,)
    }
};
        locals.var_sqd__blk1401 = assign52350_e67470;
        locals.var_sqd__blk1401_dn5 = assign52350_e67470_d_n5;
        locals.var_sqd__blk1401_dn6 = assign52350_e67470_d_n6;
        locals.var_sqd__blk1401_dn7 = assign52350_e67470_d_n7;
        locals.var_sqd__blk1401_dn8 = assign52350_e67470_d_n8;
        locals.var_sqd__blk1401_rv = 0.0;

        let (assign52360_e67491, assign52360_e67491_d_n5, assign52360_e67491_d_n6, assign52360_e67491_d_n7, assign52360_e67491_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 == 0.0)) {
        let assign52360_e67482: f64 = (1.0 / locals.var_ed__blk1399);
        let assign52360_e67484: f64 = (assign52360_e67482 - locals.var_x_d__blk1393);
        let assign52360_e67486: f64 = (assign52360_e67484 - 1.0);
        let assign52360_e67488: f64 = (assign52360_e67486 - locals.var_xi0d__blk1398);
        let assign52360_e67489: f64 = (locals.var_delta_nd__blk1392 * assign52360_e67488);
        (assign52360_e67489, ((locals.var_delta_nd__blk1392_dn5 * assign52360_e67488) + (locals.var_delta_nd__blk1392 * (((-(locals.var_ed__blk1399_dn5 / (locals.var_ed__blk1399 * locals.var_ed__blk1399))) - locals.var_x_d__blk1393_dn5) - locals.var_xi0d__blk1398_dn5))), ((locals.var_delta_nd__blk1392_dn6 * assign52360_e67488) + (locals.var_delta_nd__blk1392 * (((-(locals.var_ed__blk1399_dn6 / (locals.var_ed__blk1399 * locals.var_ed__blk1399))) - locals.var_x_d__blk1393_dn6) - locals.var_xi0d__blk1398_dn6))), ((locals.var_delta_nd__blk1392_dn7 * assign52360_e67488) + (locals.var_delta_nd__blk1392 * (((-(locals.var_ed__blk1399_dn7 / (locals.var_ed__blk1399 * locals.var_ed__blk1399))) - locals.var_x_d__blk1393_dn7) - locals.var_xi0d__blk1398_dn7))), ((locals.var_delta_nd__blk1392_dn8 * assign52360_e67488) + (locals.var_delta_nd__blk1392 * (((-(locals.var_ed__blk1399_dn8 / (locals.var_ed__blk1399 * locals.var_ed__blk1399))) - locals.var_x_d__blk1393_dn8) - locals.var_xi0d__blk1398_dn8))),)
    } else {
        (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8,)
    }
};
        locals.var_dd__blk1402 = assign52360_e67491;
        locals.var_dd__blk1402_dn5 = assign52360_e67491_d_n5;
        locals.var_dd__blk1402_dn6 = assign52360_e67491_d_n6;
        locals.var_dd__blk1402_dn7 = assign52360_e67491_d_n7;
        locals.var_dd__blk1402_dn8 = assign52360_e67491_d_n8;
        locals.var_dd__blk1402_rv = 0.0;

        let assign52370_e67495: f64 = (locals.var_xn_d__blk1390 - 230.25850929940458);
        let assign52370_e67496: f64 = if locals.var_x_d__blk1393 > assign52370_e67495 { 1.0 } else { 0.0 };
        locals.var_guard1498 = assign52370_e67496;
        locals.var_guard1498_rv = 0.0;

        let (assign52380_e67510, assign52380_e67510_d_n5, assign52380_e67510_d_n6, assign52380_e67510_d_n7, assign52380_e67510_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign52380_e67507: f64 = (locals.var_x_d__blk1393 - locals.var_xn_d__blk1390);
        let assign52380_e67508: f64 = (assign52380_e67507).exp();
        (assign52380_e67508, (assign52380_e67508 * (locals.var_x_d__blk1393_dn5 - locals.var_xn_d__blk1390_dn5)), (assign52380_e67508 * (locals.var_x_d__blk1393_dn6 - locals.var_xn_d__blk1390_dn6)), (assign52380_e67508 * (locals.var_x_d__blk1393_dn7 - locals.var_xn_d__blk1390_dn7)), (assign52380_e67508 * (locals.var_x_d__blk1393_dn8 - locals.var_xn_d__blk1390_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52380_e67510;
        locals.var_temp__blk936_dn5 = assign52380_e67510_d_n5;
        locals.var_temp__blk936_dn6 = assign52380_e67510_d_n6;
        locals.var_temp__blk936_dn7 = assign52380_e67510_d_n7;
        locals.var_temp__blk936_dn8 = assign52380_e67510_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign52390_e67523, assign52390_e67523_d_n5, assign52390_e67523_d_n6, assign52390_e67523_d_n7, assign52390_e67523_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign52390_e67521: f64 = (locals.var_delta_nd__blk1392 / locals.var_temp__blk936);
        (assign52390_e67521, (((locals.var_delta_nd__blk1392_dn5 * locals.var_temp__blk936) - (locals.var_delta_nd__blk1392 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd__blk1392_dn6 * locals.var_temp__blk936) - (locals.var_delta_nd__blk1392 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd__blk1392_dn7 * locals.var_temp__blk936) - (locals.var_delta_nd__blk1392 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd__blk1392_dn8 * locals.var_temp__blk936) - (locals.var_delta_nd__blk1392 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)),)
    } else {
        (locals.var_ed__blk1399, locals.var_ed__blk1399_dn5, locals.var_ed__blk1399_dn6, locals.var_ed__blk1399_dn7, locals.var_ed__blk1399_dn8,)
    }
};
        locals.var_ed__blk1399 = assign52390_e67523;
        locals.var_ed__blk1399_dn5 = assign52390_e67523_d_n5;
        locals.var_ed__blk1399_dn6 = assign52390_e67523_d_n6;
        locals.var_ed__blk1399_dn7 = assign52390_e67523_d_n7;
        locals.var_ed__blk1399_dn8 = assign52390_e67523_d_n8;
        locals.var_ed__blk1399_rv = 0.0;

        let (assign52400_e67542, assign52400_e67542_d_n5, assign52400_e67542_d_n6, assign52400_e67542_d_n7, assign52400_e67542_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign52400_e67536: f64 = (locals.var_x_d__blk1393 + 1.0);
        let assign52400_e67538: f64 = (assign52400_e67536 + locals.var_xi0d__blk1398);
        let assign52400_e67539: f64 = (locals.var_delta_nd__blk1392 * assign52400_e67538);
        let assign52400_e67540: f64 = (locals.var_temp__blk936 - assign52400_e67539);
        (assign52400_e67540, (locals.var_temp__blk936_dn5 - ((locals.var_delta_nd__blk1392_dn5 * assign52400_e67538) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn5 + locals.var_xi0d__blk1398_dn5)))), (locals.var_temp__blk936_dn6 - ((locals.var_delta_nd__blk1392_dn6 * assign52400_e67538) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn6 + locals.var_xi0d__blk1398_dn6)))), (locals.var_temp__blk936_dn7 - ((locals.var_delta_nd__blk1392_dn7 * assign52400_e67538) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn7 + locals.var_xi0d__blk1398_dn7)))), (locals.var_temp__blk936_dn8 - ((locals.var_delta_nd__blk1392_dn8 * assign52400_e67538) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn8 + locals.var_xi0d__blk1398_dn8)))),)
    } else {
        (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8,)
    }
};
        locals.var_dd__blk1402 = assign52400_e67542;
        locals.var_dd__blk1402_dn5 = assign52400_e67542_d_n5;
        locals.var_dd__blk1402_dn6 = assign52400_e67542_d_n6;
        locals.var_dd__blk1402_dn7 = assign52400_e67542_d_n7;
        locals.var_dd__blk1402_dn8 = assign52400_e67542_d_n8;
        locals.var_dd__blk1402_rv = 0.0;

        let (assign52410_e67576, assign52410_e67576_d_n5, assign52410_e67576_d_n6, assign52410_e67576_d_n7, assign52410_e67576_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 == 0.0)) {
        let assign52410_e67556: f64 = (locals.var_x_d__blk1393 - 230.25850929940458);
        let assign52410_e67561: f64 = (locals.var_x_d__blk1393 - 230.25850929940458);
        let assign52410_e67565: f64 = (locals.var_x_d__blk1393 - 230.25850929940458);
        let assign52410_e67567: f64 = (assign52410_e67565 * 0.3333333333333333);
        let assign52410_e67568: f64 = (1.0 + assign52410_e67567);
        let assign52410_e67569: f64 = (assign52410_e67561 * assign52410_e67568);
        let assign52410_e67570: f64 = (0.5 * assign52410_e67569);
        let assign52410_e67571: f64 = (1.0 + assign52410_e67570);
        let assign52410_e67572: f64 = (assign52410_e67556 * assign52410_e67571);
        let assign52410_e67573: f64 = (1.0 + assign52410_e67572);
        let assign52410_e67574: f64 = (1e-100 / assign52410_e67573);
        (assign52410_e67574, (-((1e-100 * ((locals.var_x_d__blk1393_dn5 * assign52410_e67571) + (assign52410_e67556 * (0.5 * ((locals.var_x_d__blk1393_dn5 * assign52410_e67568) + (assign52410_e67561 * (locals.var_x_d__blk1393_dn5 * 0.3333333333333333))))))) / (assign52410_e67573 * assign52410_e67573))), (-((1e-100 * ((locals.var_x_d__blk1393_dn6 * assign52410_e67571) + (assign52410_e67556 * (0.5 * ((locals.var_x_d__blk1393_dn6 * assign52410_e67568) + (assign52410_e67561 * (locals.var_x_d__blk1393_dn6 * 0.3333333333333333))))))) / (assign52410_e67573 * assign52410_e67573))), (-((1e-100 * ((locals.var_x_d__blk1393_dn7 * assign52410_e67571) + (assign52410_e67556 * (0.5 * ((locals.var_x_d__blk1393_dn7 * assign52410_e67568) + (assign52410_e67561 * (locals.var_x_d__blk1393_dn7 * 0.3333333333333333))))))) / (assign52410_e67573 * assign52410_e67573))), (-((1e-100 * ((locals.var_x_d__blk1393_dn8 * assign52410_e67571) + (assign52410_e67556 * (0.5 * ((locals.var_x_d__blk1393_dn8 * assign52410_e67568) + (assign52410_e67561 * (locals.var_x_d__blk1393_dn8 * 0.3333333333333333))))))) / (assign52410_e67573 * assign52410_e67573))),)
    } else {
        (locals.var_ed__blk1399, locals.var_ed__blk1399_dn5, locals.var_ed__blk1399_dn6, locals.var_ed__blk1399_dn7, locals.var_ed__blk1399_dn8,)
    }
};
        locals.var_ed__blk1399 = assign52410_e67576;
        locals.var_ed__blk1399_dn5 = assign52410_e67576_d_n5;
        locals.var_ed__blk1399_dn6 = assign52410_e67576_d_n6;
        locals.var_ed__blk1399_dn7 = assign52410_e67576_d_n7;
        locals.var_ed__blk1399_dn8 = assign52410_e67576_d_n8;
        locals.var_ed__blk1399_rv = 0.0;

        let (assign52420_e67616, assign52420_e67616_d_n5, assign52420_e67616_d_n6, assign52420_e67616_d_n7, assign52420_e67616_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 == 0.0)) {
        let assign52420_e67590: f64 = (locals.var_xn_d__blk1390 - locals.var_x_d__blk1393);
        let assign52420_e67592: f64 = (assign52420_e67590 - 230.25850929940458);
        let assign52420_e67597: f64 = (locals.var_xn_d__blk1390 - locals.var_x_d__blk1393);
        let assign52420_e67599: f64 = (assign52420_e67597 - 230.25850929940458);
        let assign52420_e67603: f64 = (locals.var_xn_d__blk1390 - locals.var_x_d__blk1393);
        let assign52420_e67605: f64 = (assign52420_e67603 - 230.25850929940458);
        let assign52420_e67607: f64 = (assign52420_e67605 * 0.3333333333333333);
        let assign52420_e67608: f64 = (1.0 + assign52420_e67607);
        let assign52420_e67609: f64 = (assign52420_e67599 * assign52420_e67608);
        let assign52420_e67610: f64 = (0.5 * assign52420_e67609);
        let assign52420_e67611: f64 = (1.0 + assign52420_e67610);
        let assign52420_e67612: f64 = (assign52420_e67592 * assign52420_e67611);
        let assign52420_e67613: f64 = (1.0 + assign52420_e67612);
        let assign52420_e67614: f64 = (1e-100 / assign52420_e67613);
        (assign52420_e67614, (-((1e-100 * (((locals.var_xn_d__blk1390_dn5 - locals.var_x_d__blk1393_dn5) * assign52420_e67611) + (assign52420_e67592 * (0.5 * (((locals.var_xn_d__blk1390_dn5 - locals.var_x_d__blk1393_dn5) * assign52420_e67608) + (assign52420_e67599 * ((locals.var_xn_d__blk1390_dn5 - locals.var_x_d__blk1393_dn5) * 0.3333333333333333))))))) / (assign52420_e67613 * assign52420_e67613))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn6 - locals.var_x_d__blk1393_dn6) * assign52420_e67611) + (assign52420_e67592 * (0.5 * (((locals.var_xn_d__blk1390_dn6 - locals.var_x_d__blk1393_dn6) * assign52420_e67608) + (assign52420_e67599 * ((locals.var_xn_d__blk1390_dn6 - locals.var_x_d__blk1393_dn6) * 0.3333333333333333))))))) / (assign52420_e67613 * assign52420_e67613))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn7 - locals.var_x_d__blk1393_dn7) * assign52420_e67611) + (assign52420_e67592 * (0.5 * (((locals.var_xn_d__blk1390_dn7 - locals.var_x_d__blk1393_dn7) * assign52420_e67608) + (assign52420_e67599 * ((locals.var_xn_d__blk1390_dn7 - locals.var_x_d__blk1393_dn7) * 0.3333333333333333))))))) / (assign52420_e67613 * assign52420_e67613))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn8 - locals.var_x_d__blk1393_dn8) * assign52420_e67611) + (assign52420_e67592 * (0.5 * (((locals.var_xn_d__blk1390_dn8 - locals.var_x_d__blk1393_dn8) * assign52420_e67608) + (assign52420_e67599 * ((locals.var_xn_d__blk1390_dn8 - locals.var_x_d__blk1393_dn8) * 0.3333333333333333))))))) / (assign52420_e67613 * assign52420_e67613))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52420_e67616;
        locals.var_temp__blk936_dn5 = assign52420_e67616_d_n5;
        locals.var_temp__blk936_dn6 = assign52420_e67616_d_n6;
        locals.var_temp__blk936_dn7 = assign52420_e67616_d_n7;
        locals.var_temp__blk936_dn8 = assign52420_e67616_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign52430_e67636, assign52430_e67636_d_n5, assign52430_e67636_d_n6, assign52430_e67636_d_n7, assign52430_e67636_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 == 0.0)) {
        let assign52430_e67630: f64 = (locals.var_x_d__blk1393 + 1.0);
        let assign52430_e67632: f64 = (assign52430_e67630 + locals.var_xi0d__blk1398);
        let assign52430_e67633: f64 = (locals.var_delta_nd__blk1392 * assign52430_e67632);
        let assign52430_e67634: f64 = (locals.var_temp__blk936 - assign52430_e67633);
        (assign52430_e67634, (locals.var_temp__blk936_dn5 - ((locals.var_delta_nd__blk1392_dn5 * assign52430_e67632) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn5 + locals.var_xi0d__blk1398_dn5)))), (locals.var_temp__blk936_dn6 - ((locals.var_delta_nd__blk1392_dn6 * assign52430_e67632) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn6 + locals.var_xi0d__blk1398_dn6)))), (locals.var_temp__blk936_dn7 - ((locals.var_delta_nd__blk1392_dn7 * assign52430_e67632) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn7 + locals.var_xi0d__blk1398_dn7)))), (locals.var_temp__blk936_dn8 - ((locals.var_delta_nd__blk1392_dn8 * assign52430_e67632) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn8 + locals.var_xi0d__blk1398_dn8)))),)
    } else {
        (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8,)
    }
};
        locals.var_dd__blk1402 = assign52430_e67636;
        locals.var_dd__blk1402_dn5 = assign52430_e67636_d_n5;
        locals.var_dd__blk1402_dn6 = assign52430_e67636_d_n6;
        locals.var_dd__blk1402_dn7 = assign52430_e67636_d_n7;
        locals.var_dd__blk1402_dn8 = assign52430_e67636_d_n8;
        locals.var_dd__blk1402_rv = 0.0;

        let (assign52440_e67649, assign52440_e67649_d_n5, assign52440_e67649_d_n6, assign52440_e67649_d_n7, assign52440_e67649_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) {
        let assign52440_e67645: f64 = (locals.var_x_d__blk1393 - 1.0);
        let assign52440_e67647: f64 = (assign52440_e67645 + locals.var_ed__blk1399);
        (assign52440_e67647, (locals.var_x_d__blk1393_dn5 + locals.var_ed__blk1399_dn5), (locals.var_x_d__blk1393_dn6 + locals.var_ed__blk1399_dn6), (locals.var_x_d__blk1393_dn7 + locals.var_ed__blk1399_dn7), (locals.var_x_d__blk1393_dn8 + locals.var_ed__blk1399_dn8),)
    } else {
        (locals.var_pd__blk1400, locals.var_pd__blk1400_dn5, locals.var_pd__blk1400_dn6, locals.var_pd__blk1400_dn7, locals.var_pd__blk1400_dn8,)
    }
};
        locals.var_pd__blk1400 = assign52440_e67649;
        locals.var_pd__blk1400_dn5 = assign52440_e67649_d_n5;
        locals.var_pd__blk1400_dn6 = assign52440_e67649_d_n6;
        locals.var_pd__blk1400_dn7 = assign52440_e67649_d_n7;
        locals.var_pd__blk1400_dn8 = assign52440_e67649_d_n8;
        locals.var_pd__blk1400_rv = 0.0;

        let (assign52450_e67659, assign52450_e67659_d_n5, assign52450_e67659_d_n6, assign52450_e67659_d_n7, assign52450_e67659_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) {
        let assign52450_e67657: f64 = (locals.var_pd__blk1400).sqrt();
        (assign52450_e67657, (locals.var_pd__blk1400_dn5 / (2.0 * assign52450_e67657)), (locals.var_pd__blk1400_dn6 / (2.0 * assign52450_e67657)), (locals.var_pd__blk1400_dn7 / (2.0 * assign52450_e67657)), (locals.var_pd__blk1400_dn8 / (2.0 * assign52450_e67657)),)
    } else {
        (locals.var_sqd__blk1401, locals.var_sqd__blk1401_dn5, locals.var_sqd__blk1401_dn6, locals.var_sqd__blk1401_dn7, locals.var_sqd__blk1401_dn8,)
    }
};
        locals.var_sqd__blk1401 = assign52450_e67659;
        locals.var_sqd__blk1401_dn5 = assign52450_e67659_d_n5;
        locals.var_sqd__blk1401_dn6 = assign52450_e67659_d_n6;
        locals.var_sqd__blk1401_dn7 = assign52450_e67659_d_n7;
        locals.var_sqd__blk1401_dn8 = assign52450_e67659_d_n8;
        locals.var_sqd__blk1401_rv = 0.0;

        let (assign52460_e67669, assign52460_e67669_d_n5, assign52460_e67669_d_n6, assign52460_e67669_d_n7, assign52460_e67669_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52460_e67665: f64 = (locals.var_sqd__blk1401 * locals.var_gf__blk1307);
        let assign52460_e67667: f64 = (assign52460_e67665 * locals.var_phit1__blk1322);
        (assign52460_e67667, ((((locals.var_sqd__blk1401_dn5 * locals.var_gf__blk1307) + (locals.var_sqd__blk1401 * locals.var_gf__blk1307_dn5)) * locals.var_phit1__blk1322) + (assign52460_e67665 * locals.var_phit1__blk1322_dn5)), ((((locals.var_sqd__blk1401_dn6 * locals.var_gf__blk1307) + (locals.var_sqd__blk1401 * locals.var_gf__blk1307_dn6)) * locals.var_phit1__blk1322) + (assign52460_e67665 * locals.var_phit1__blk1322_dn6)), ((((locals.var_sqd__blk1401_dn7 * locals.var_gf__blk1307) + (locals.var_sqd__blk1401 * locals.var_gf__blk1307_dn7)) * locals.var_phit1__blk1322) + (assign52460_e67665 * locals.var_phit1__blk1322_dn7)), ((((locals.var_sqd__blk1401_dn8 * locals.var_gf__blk1307) + (locals.var_sqd__blk1401 * locals.var_gf__blk1307_dn8)) * locals.var_phit1__blk1322) + (assign52460_e67665 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_qbd__blk1403, locals.var_qbd__blk1403_dn5, locals.var_qbd__blk1403_dn6, locals.var_qbd__blk1403_dn7, locals.var_qbd__blk1403_dn8,)
    }
};
        locals.var_qbd__blk1403 = assign52460_e67669;
        locals.var_qbd__blk1403_dn5 = assign52460_e67669_d_n5;
        locals.var_qbd__blk1403_dn6 = assign52460_e67669_d_n6;
        locals.var_qbd__blk1403_dn7 = assign52460_e67669_d_n7;
        locals.var_qbd__blk1403_dn8 = assign52460_e67669_d_n8;
        locals.var_qbd__blk1403_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_48(
        locals: &mut StampLocals,
    ) {
        let (assign52470_e67679, assign52470_e67679_d_n5, assign52470_e67679_d_n6, assign52470_e67679_d_n7, assign52470_e67679_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52470_e67676: f64 = (locals.var_x_s__blk1346 + locals.var_x_d__blk1393);
        let assign52470_e67677: f64 = (0.5 * assign52470_e67676);
        (assign52470_e67677, (0.5 * (locals.var_x_s__blk1346_dn5 + locals.var_x_d__blk1393_dn5)), (0.5 * (locals.var_x_s__blk1346_dn6 + locals.var_x_d__blk1393_dn6)), (0.5 * (locals.var_x_s__blk1346_dn7 + locals.var_x_d__blk1393_dn7)), (0.5 * (locals.var_x_s__blk1346_dn8 + locals.var_x_d__blk1393_dn8)),)
    } else {
        (locals.var_x_m__blk1404, locals.var_x_m__blk1404_dn5, locals.var_x_m__blk1404_dn6, locals.var_x_m__blk1404_dn7, locals.var_x_m__blk1404_dn8,)
    }
};
        locals.var_x_m__blk1404 = assign52470_e67679;
        locals.var_x_m__blk1404_dn5 = assign52470_e67679_d_n5;
        locals.var_x_m__blk1404_dn6 = assign52470_e67679_d_n6;
        locals.var_x_m__blk1404_dn7 = assign52470_e67679_d_n7;
        locals.var_x_m__blk1404_dn8 = assign52470_e67679_d_n8;
        locals.var_x_m__blk1404_rv = 0.0;

        let (assign52480_e67685, assign52480_e67685_d_n5, assign52480_e67685_d_n6, assign52480_e67685_d_n7, assign52480_e67685_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_em__blk1405, locals.var_em__blk1405_dn5, locals.var_em__blk1405_dn6, locals.var_em__blk1405_dn7, locals.var_em__blk1405_dn8,)
    }
};
        locals.var_em__blk1405 = assign52480_e67685;
        locals.var_em__blk1405_dn5 = assign52480_e67685_d_n5;
        locals.var_em__blk1405_dn6 = assign52480_e67685_d_n6;
        locals.var_em__blk1405_dn7 = assign52480_e67685_d_n7;
        locals.var_em__blk1405_dn8 = assign52480_e67685_d_n8;
        locals.var_em__blk1405_rv = 0.0;

        let (assign52490_e67693, assign52490_e67693_d_n5, assign52490_e67693_d_n6, assign52490_e67693_d_n7, assign52490_e67693_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52490_e67691: f64 = (locals.var_ed__blk1399 * locals.var_es__blk1352);
        (assign52490_e67691, ((locals.var_ed__blk1399_dn5 * locals.var_es__blk1352) + (locals.var_ed__blk1399 * locals.var_es__blk1352_dn5)), ((locals.var_ed__blk1399_dn6 * locals.var_es__blk1352) + (locals.var_ed__blk1399 * locals.var_es__blk1352_dn6)), ((locals.var_ed__blk1399_dn7 * locals.var_es__blk1352) + (locals.var_ed__blk1399 * locals.var_es__blk1352_dn7)), ((locals.var_ed__blk1399_dn8 * locals.var_es__blk1352) + (locals.var_ed__blk1399 * locals.var_es__blk1352_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52490_e67693;
        locals.var_temp__blk936_dn5 = assign52490_e67693_d_n5;
        locals.var_temp__blk936_dn6 = assign52490_e67693_d_n6;
        locals.var_temp__blk936_dn7 = assign52490_e67693_d_n7;
        locals.var_temp__blk936_dn8 = assign52490_e67693_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign52500_e67696: f64 = if locals.var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1499 = assign52500_e67696;
        locals.var_guard1499_rv = 0.0;

        let (assign52510_e67705, assign52510_e67705_d_n5, assign52510_e67705_d_n6, assign52510_e67705_d_n7, assign52510_e67705_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign52510_e67703: f64 = (locals.var_temp__blk936).sqrt();
        (assign52510_e67703, (locals.var_temp__blk936_dn5 / (2.0 * assign52510_e67703)), (locals.var_temp__blk936_dn6 / (2.0 * assign52510_e67703)), (locals.var_temp__blk936_dn7 / (2.0 * assign52510_e67703)), (locals.var_temp__blk936_dn8 / (2.0 * assign52510_e67703)),)
    } else {
        (locals.var_em__blk1405, locals.var_em__blk1405_dn5, locals.var_em__blk1405_dn6, locals.var_em__blk1405_dn7, locals.var_em__blk1405_dn8,)
    }
};
        locals.var_em__blk1405 = assign52510_e67705;
        locals.var_em__blk1405_dn5 = assign52510_e67705_d_n5;
        locals.var_em__blk1405_dn6 = assign52510_e67705_d_n6;
        locals.var_em__blk1405_dn7 = assign52510_e67705_d_n7;
        locals.var_em__blk1405_dn8 = assign52510_e67705_d_n8;
        locals.var_em__blk1405_rv = 0.0;

        let (assign52520_e67715, assign52520_e67715_d_n5, assign52520_e67715_d_n6, assign52520_e67715_d_n7, assign52520_e67715_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52520_e67712: f64 = (locals.var_ds__blk1353 + locals.var_dd__blk1402);
        let assign52520_e67713: f64 = (0.5 * assign52520_e67712);
        (assign52520_e67713, (0.5 * (locals.var_ds__blk1353_dn5 + locals.var_dd__blk1402_dn5)), (0.5 * (locals.var_ds__blk1353_dn6 + locals.var_dd__blk1402_dn6)), (0.5 * (locals.var_ds__blk1353_dn7 + locals.var_dd__blk1402_dn7)), (0.5 * (locals.var_ds__blk1353_dn8 + locals.var_dd__blk1402_dn8)),)
    } else {
        (locals.var_d_bar__blk1406, locals.var_d_bar__blk1406_dn5, locals.var_d_bar__blk1406_dn6, locals.var_d_bar__blk1406_dn7, locals.var_d_bar__blk1406_dn8,)
    }
};
        locals.var_d_bar__blk1406 = assign52520_e67715;
        locals.var_d_bar__blk1406_dn5 = assign52520_e67715_d_n5;
        locals.var_d_bar__blk1406_dn6 = assign52520_e67715_d_n6;
        locals.var_d_bar__blk1406_dn7 = assign52520_e67715_d_n7;
        locals.var_d_bar__blk1406_dn8 = assign52520_e67715_d_n8;
        locals.var_d_bar__blk1406_rv = 0.0;

        let (assign52530_e67733, assign52530_e67733_d_n5, assign52530_e67733_d_n6, assign52530_e67733_d_n7, assign52530_e67733_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52530_e67723: f64 = (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394);
        let assign52530_e67727: f64 = (2.0 * locals.var_inv_gf2__blk1324);
        let assign52530_e67728: f64 = (locals.var_em__blk1405 - assign52530_e67727);
        let assign52530_e67729: f64 = (assign52530_e67723 * assign52530_e67728);
        let assign52530_e67730: f64 = (0.125 * assign52530_e67729);
        let assign52530_e67731: f64 = (locals.var_d_bar__blk1406 + assign52530_e67730);
        (assign52530_e67731, (locals.var_d_bar__blk1406_dn5 + (0.125 * ((((locals.var_x_ds__blk1394_dn5 * locals.var_x_ds__blk1394) + (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394_dn5)) * assign52530_e67728) + (assign52530_e67723 * (locals.var_em__blk1405_dn5 - (2.0 * locals.var_inv_gf2__blk1324_dn5)))))), (locals.var_d_bar__blk1406_dn6 + (0.125 * ((((locals.var_x_ds__blk1394_dn6 * locals.var_x_ds__blk1394) + (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394_dn6)) * assign52530_e67728) + (assign52530_e67723 * (locals.var_em__blk1405_dn6 - (2.0 * locals.var_inv_gf2__blk1324_dn6)))))), (locals.var_d_bar__blk1406_dn7 + (0.125 * ((((locals.var_x_ds__blk1394_dn7 * locals.var_x_ds__blk1394) + (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394_dn7)) * assign52530_e67728) + (assign52530_e67723 * (locals.var_em__blk1405_dn7 - (2.0 * locals.var_inv_gf2__blk1324_dn7)))))), (locals.var_d_bar__blk1406_dn8 + (0.125 * ((((locals.var_x_ds__blk1394_dn8 * locals.var_x_ds__blk1394) + (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394_dn8)) * assign52530_e67728) + (assign52530_e67723 * (locals.var_em__blk1405_dn8 - (2.0 * locals.var_inv_gf2__blk1324_dn8)))))),)
    } else {
        (locals.var_dm__blk1407, locals.var_dm__blk1407_dn5, locals.var_dm__blk1407_dn6, locals.var_dm__blk1407_dn7, locals.var_dm__blk1407_dn8,)
    }
};
        locals.var_dm__blk1407 = assign52530_e67733;
        locals.var_dm__blk1407_dn5 = assign52530_e67733_d_n5;
        locals.var_dm__blk1407_dn6 = assign52530_e67733_d_n6;
        locals.var_dm__blk1407_dn7 = assign52530_e67733_d_n7;
        locals.var_dm__blk1407_dn8 = assign52530_e67733_d_n8;
        locals.var_dm__blk1407_rv = 0.0;

        let assign52540_e67736: f64 = if locals.var_x_m__blk1404 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1500 = assign52540_e67736;
        locals.var_guard1500_rv = 0.0;

        let (assign52550_e67760, assign52550_e67760_d_n5, assign52550_e67760_d_n6, assign52550_e67760_d_n7, assign52550_e67760_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign52550_e67745: f64 = (locals.var_x_m__blk1404 * locals.var_x_m__blk1404);
        let assign52550_e67752: f64 = (0.25 * locals.var_x_m__blk1404);
        let assign52550_e67753: f64 = (1.0 - assign52550_e67752);
        let assign52550_e67754: f64 = (locals.var_x_m__blk1404 * assign52550_e67753);
        let assign52550_e67755: f64 = (0.3333333333333333 * assign52550_e67754);
        let assign52550_e67756: f64 = (1.0 - assign52550_e67755);
        let assign52550_e67757: f64 = (assign52550_e67745 * assign52550_e67756);
        let assign52550_e67758: f64 = (0.5 * assign52550_e67757);
        (assign52550_e67758, (0.5 * ((((locals.var_x_m__blk1404_dn5 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn5)) * assign52550_e67756) + (assign52550_e67745 * (-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn5 * assign52550_e67753) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn5))))))))), (0.5 * ((((locals.var_x_m__blk1404_dn6 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn6)) * assign52550_e67756) + (assign52550_e67745 * (-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn6 * assign52550_e67753) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn6))))))))), (0.5 * ((((locals.var_x_m__blk1404_dn7 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn7)) * assign52550_e67756) + (assign52550_e67745 * (-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn7 * assign52550_e67753) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn7))))))))), (0.5 * ((((locals.var_x_m__blk1404_dn8 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn8)) * assign52550_e67756) + (assign52550_e67745 * (-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn8 * assign52550_e67753) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn8))))))))),)
    } else {
        (locals.var_pm__blk1408, locals.var_pm__blk1408_dn5, locals.var_pm__blk1408_dn6, locals.var_pm__blk1408_dn7, locals.var_pm__blk1408_dn8,)
    }
};
        locals.var_pm__blk1408 = assign52550_e67760;
        locals.var_pm__blk1408_dn5 = assign52550_e67760_d_n5;
        locals.var_pm__blk1408_dn6 = assign52550_e67760_d_n6;
        locals.var_pm__blk1408_dn7 = assign52550_e67760_d_n7;
        locals.var_pm__blk1408_dn8 = assign52550_e67760_d_n8;
        locals.var_pm__blk1408_rv = 0.0;

        let (assign52560_e67773, assign52560_e67773_d_n5, assign52560_e67773_d_n6, assign52560_e67773_d_n7, assign52560_e67773_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign52560_e67769: f64 = (locals.var_dm__blk1407 + locals.var_pm__blk1408);
        let assign52560_e67770: f64 = (assign52560_e67769).sqrt();
        let assign52560_e67771: f64 = (locals.var_gf__blk1307 * assign52560_e67770);
        (assign52560_e67771, ((locals.var_gf__blk1307_dn5 * assign52560_e67770) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn5 + locals.var_pm__blk1408_dn5) / (2.0 * assign52560_e67770)))), ((locals.var_gf__blk1307_dn6 * assign52560_e67770) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn6 + locals.var_pm__blk1408_dn6) / (2.0 * assign52560_e67770)))), ((locals.var_gf__blk1307_dn7 * assign52560_e67770) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn7 + locals.var_pm__blk1408_dn7) / (2.0 * assign52560_e67770)))), ((locals.var_gf__blk1307_dn8 * assign52560_e67770) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn8 + locals.var_pm__blk1408_dn8) / (2.0 * assign52560_e67770)))),)
    } else {
        (locals.var_xgm__blk1409, locals.var_xgm__blk1409_dn5, locals.var_xgm__blk1409_dn6, locals.var_xgm__blk1409_dn7, locals.var_xgm__blk1409_dn8,)
    }
};
        locals.var_xgm__blk1409 = assign52560_e67773;
        locals.var_xgm__blk1409_dn5 = assign52560_e67773_d_n5;
        locals.var_xgm__blk1409_dn6 = assign52560_e67773_d_n6;
        locals.var_xgm__blk1409_dn7 = assign52560_e67773_d_n7;
        locals.var_xgm__blk1409_dn8 = assign52560_e67773_d_n8;
        locals.var_xgm__blk1409_rv = 0.0;

        let assign52570_e67776: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1501 = assign52570_e67776;
        locals.var_guard1501_rv = 0.0;

        let (assign52580_e67793, assign52580_e67793_d_n5, assign52580_e67793_d_n6, assign52580_e67793_d_n7, assign52580_e67793_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign52580_e67788: f64 = (locals.var_kp * locals.var_xgm__blk1409);
        let assign52580_e67789: f64 = (1.0 + assign52580_e67788);
        let assign52580_e67790: f64 = (assign52580_e67789).sqrt();
        let assign52580_e67791: f64 = (1.0 / assign52580_e67790);
        (assign52580_e67791, (-(((locals.var_kp * locals.var_xgm__blk1409_dn5) / (2.0 * assign52580_e67790)) / (assign52580_e67790 * assign52580_e67790))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn6) / (2.0 * assign52580_e67790)) / (assign52580_e67790 * assign52580_e67790))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn7) / (2.0 * assign52580_e67790)) / (assign52580_e67790 * assign52580_e67790))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn8) / (2.0 * assign52580_e67790)) / (assign52580_e67790 * assign52580_e67790))),)
    } else {
        (locals.var_eta_p__blk1410, locals.var_eta_p__blk1410_dn5, locals.var_eta_p__blk1410_dn6, locals.var_eta_p__blk1410_dn7, locals.var_eta_p__blk1410_dn8,)
    }
};
        locals.var_eta_p__blk1410 = assign52580_e67793;
        locals.var_eta_p__blk1410_dn5 = assign52580_e67793_d_n5;
        locals.var_eta_p__blk1410_dn6 = assign52580_e67793_d_n6;
        locals.var_eta_p__blk1410_dn7 = assign52580_e67793_d_n7;
        locals.var_eta_p__blk1410_dn8 = assign52580_e67793_d_n8;
        locals.var_eta_p__blk1410_rv = 0.0;

        let (assign52590_e67812, assign52590_e67812_d_n5, assign52590_e67812_d_n6, assign52590_e67812_d_n7, assign52590_e67812_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign52590_e67805: f64 = (0.25 * locals.var_x_m__blk1404);
        let assign52590_e67806: f64 = (1.0 - assign52590_e67805);
        let assign52590_e67807: f64 = (locals.var_x_m__blk1404 * assign52590_e67806);
        let assign52590_e67808: f64 = (0.3333333333333333 * assign52590_e67807);
        let assign52590_e67809: f64 = (1.0 - assign52590_e67808);
        let assign52590_e67810: f64 = (assign52590_e67809).sqrt();
        (assign52590_e67810, ((-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn5 * assign52590_e67806) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn5)))))) / (2.0 * assign52590_e67810)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn6 * assign52590_e67806) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn6)))))) / (2.0 * assign52590_e67810)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn7 * assign52590_e67806) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn7)))))) / (2.0 * assign52590_e67810)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn8 * assign52590_e67806) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn8)))))) / (2.0 * assign52590_e67810)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52590_e67812;
        locals.var_temp__blk936_dn5 = assign52590_e67812_d_n5;
        locals.var_temp__blk936_dn6 = assign52590_e67812_d_n6;
        locals.var_temp__blk936_dn7 = assign52590_e67812_d_n7;
        locals.var_temp__blk936_dn8 = assign52590_e67812_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign52600_e67824, assign52600_e67824_d_n5, assign52600_e67824_d_n6, assign52600_e67824_d_n7, assign52600_e67824_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign52600_e67821: f64 = (locals.var_x_m__blk1404 * locals.var_temp__blk936);
        let assign52600_e67822: f64 = (0.7071067811865475 * assign52600_e67821);
        (assign52600_e67822, (0.7071067811865475 * ((locals.var_x_m__blk1404_dn5 * locals.var_temp__blk936) + (locals.var_x_m__blk1404 * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_m__blk1404_dn6 * locals.var_temp__blk936) + (locals.var_x_m__blk1404 * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_m__blk1404_dn7 * locals.var_temp__blk936) + (locals.var_x_m__blk1404 * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_m__blk1404_dn8 * locals.var_temp__blk936) + (locals.var_x_m__blk1404 * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_sqm__blk1411, locals.var_sqm__blk1411_dn5, locals.var_sqm__blk1411_dn6, locals.var_sqm__blk1411_dn7, locals.var_sqm__blk1411_dn8,)
    }
};
        locals.var_sqm__blk1411 = assign52600_e67824;
        locals.var_sqm__blk1411_dn5 = assign52600_e67824_d_n5;
        locals.var_sqm__blk1411_dn6 = assign52600_e67824_d_n6;
        locals.var_sqm__blk1411_dn7 = assign52600_e67824_d_n7;
        locals.var_sqm__blk1411_dn8 = assign52600_e67824_d_n8;
        locals.var_sqm__blk1411_rv = 0.0;

        let (assign52610_e67850, assign52610_e67850_d_n5, assign52610_e67850_d_n6, assign52610_e67850_d_n7, assign52610_e67850_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign52610_e67836: f64 = (0.5 * locals.var_x_m__blk1404);
        let assign52610_e67837: f64 = (1.0 - assign52610_e67836);
        let assign52610_e67841: f64 = (locals.var_x_m__blk1404 * locals.var_x_m__blk1404);
        let assign52610_e67842: f64 = (0.16666666666666666 * assign52610_e67841);
        let assign52610_e67843: f64 = (assign52610_e67837 + assign52610_e67842);
        let assign52610_e67844: f64 = (locals.var_gf__blk1307 * assign52610_e67843);
        let assign52610_e67846: f64 = (assign52610_e67844 / locals.var_temp__blk936);
        let assign52610_e67847: f64 = (0.7071067811865475 * assign52610_e67846);
        let assign52610_e67848: f64 = (locals.var_eta_p__blk1410 + assign52610_e67847);
        (assign52610_e67848, (locals.var_eta_p__blk1410_dn5 + (0.7071067811865475 * (((((locals.var_gf__blk1307_dn5 * assign52610_e67843) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_m__blk1404_dn5)) + (0.16666666666666666 * ((locals.var_x_m__blk1404_dn5 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn5)))))) * locals.var_temp__blk936) - (assign52610_e67844 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p__blk1410_dn6 + (0.7071067811865475 * (((((locals.var_gf__blk1307_dn6 * assign52610_e67843) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_m__blk1404_dn6)) + (0.16666666666666666 * ((locals.var_x_m__blk1404_dn6 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn6)))))) * locals.var_temp__blk936) - (assign52610_e67844 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p__blk1410_dn7 + (0.7071067811865475 * (((((locals.var_gf__blk1307_dn7 * assign52610_e67843) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_m__blk1404_dn7)) + (0.16666666666666666 * ((locals.var_x_m__blk1404_dn7 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn7)))))) * locals.var_temp__blk936) - (assign52610_e67844 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p__blk1410_dn8 + (0.7071067811865475 * (((((locals.var_gf__blk1307_dn8 * assign52610_e67843) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_m__blk1404_dn8)) + (0.16666666666666666 * ((locals.var_x_m__blk1404_dn8 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn8)))))) * locals.var_temp__blk936) - (assign52610_e67844 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))),)
    } else {
        (locals.var_alpha__blk1412, locals.var_alpha__blk1412_dn5, locals.var_alpha__blk1412_dn6, locals.var_alpha__blk1412_dn7, locals.var_alpha__blk1412_dn8,)
    }
};
        locals.var_alpha__blk1412 = assign52610_e67850;
        locals.var_alpha__blk1412_dn5 = assign52610_e67850_d_n5;
        locals.var_alpha__blk1412_dn6 = assign52610_e67850_d_n6;
        locals.var_alpha__blk1412_dn7 = assign52610_e67850_d_n7;
        locals.var_alpha__blk1412_dn8 = assign52610_e67850_d_n8;
        locals.var_alpha__blk1412_rv = 0.0;

        let (assign52620_e67863, assign52620_e67863_d_n5, assign52620_e67863_d_n6, assign52620_e67863_d_n7, assign52620_e67863_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) {
        let assign52620_e67859: f64 = (locals.var_x_m__blk1404 - 1.0);
        let assign52620_e67861: f64 = (assign52620_e67859 + locals.var_em__blk1405);
        (assign52620_e67861, (locals.var_x_m__blk1404_dn5 + locals.var_em__blk1405_dn5), (locals.var_x_m__blk1404_dn6 + locals.var_em__blk1405_dn6), (locals.var_x_m__blk1404_dn7 + locals.var_em__blk1405_dn7), (locals.var_x_m__blk1404_dn8 + locals.var_em__blk1405_dn8),)
    } else {
        (locals.var_pm__blk1408, locals.var_pm__blk1408_dn5, locals.var_pm__blk1408_dn6, locals.var_pm__blk1408_dn7, locals.var_pm__blk1408_dn8,)
    }
};
        locals.var_pm__blk1408 = assign52620_e67863;
        locals.var_pm__blk1408_dn5 = assign52620_e67863_d_n5;
        locals.var_pm__blk1408_dn6 = assign52620_e67863_d_n6;
        locals.var_pm__blk1408_dn7 = assign52620_e67863_d_n7;
        locals.var_pm__blk1408_dn8 = assign52620_e67863_d_n8;
        locals.var_pm__blk1408_rv = 0.0;

        let (assign52630_e67877, assign52630_e67877_d_n5, assign52630_e67877_d_n6, assign52630_e67877_d_n7, assign52630_e67877_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) {
        let assign52630_e67873: f64 = (locals.var_dm__blk1407 + locals.var_pm__blk1408);
        let assign52630_e67874: f64 = (assign52630_e67873).sqrt();
        let assign52630_e67875: f64 = (locals.var_gf__blk1307 * assign52630_e67874);
        (assign52630_e67875, ((locals.var_gf__blk1307_dn5 * assign52630_e67874) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn5 + locals.var_pm__blk1408_dn5) / (2.0 * assign52630_e67874)))), ((locals.var_gf__blk1307_dn6 * assign52630_e67874) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn6 + locals.var_pm__blk1408_dn6) / (2.0 * assign52630_e67874)))), ((locals.var_gf__blk1307_dn7 * assign52630_e67874) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn7 + locals.var_pm__blk1408_dn7) / (2.0 * assign52630_e67874)))), ((locals.var_gf__blk1307_dn8 * assign52630_e67874) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn8 + locals.var_pm__blk1408_dn8) / (2.0 * assign52630_e67874)))),)
    } else {
        (locals.var_xgm__blk1409, locals.var_xgm__blk1409_dn5, locals.var_xgm__blk1409_dn6, locals.var_xgm__blk1409_dn7, locals.var_xgm__blk1409_dn8,)
    }
};
        locals.var_xgm__blk1409 = assign52630_e67877;
        locals.var_xgm__blk1409_dn5 = assign52630_e67877_d_n5;
        locals.var_xgm__blk1409_dn6 = assign52630_e67877_d_n6;
        locals.var_xgm__blk1409_dn7 = assign52630_e67877_d_n7;
        locals.var_xgm__blk1409_dn8 = assign52630_e67877_d_n8;
        locals.var_xgm__blk1409_rv = 0.0;

        let assign52640_e67880: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1502 = assign52640_e67880;
        locals.var_guard1502_rv = 0.0;

        let (assign52650_e67899, assign52650_e67899_d_n5, assign52650_e67899_d_n6, assign52650_e67899_d_n7, assign52650_e67899_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52650_e67891: f64 = (1.0 - locals.var_em__blk1405);
        let assign52650_e67895: f64 = (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324);
        let assign52650_e67896: f64 = (2.0 * assign52650_e67895);
        let assign52650_e67897: f64 = (assign52650_e67891 + assign52650_e67896);
        (assign52650_e67897, ((-locals.var_em__blk1405_dn5) + (2.0 * ((locals.var_xgm__blk1409_dn5 * locals.var_inv_gf2__blk1324) + (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324_dn5)))), ((-locals.var_em__blk1405_dn6) + (2.0 * ((locals.var_xgm__blk1409_dn6 * locals.var_inv_gf2__blk1324) + (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324_dn6)))), ((-locals.var_em__blk1405_dn7) + (2.0 * ((locals.var_xgm__blk1409_dn7 * locals.var_inv_gf2__blk1324) + (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324_dn7)))), ((-locals.var_em__blk1405_dn8) + (2.0 * ((locals.var_xgm__blk1409_dn8 * locals.var_inv_gf2__blk1324) + (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324_dn8)))),)
    } else {
        (locals.var_d0__blk1413, locals.var_d0__blk1413_dn5, locals.var_d0__blk1413_dn6, locals.var_d0__blk1413_dn7, locals.var_d0__blk1413_dn8,)
    }
};
        locals.var_d0__blk1413 = assign52650_e67899;
        locals.var_d0__blk1413_dn5 = assign52650_e67899_d_n5;
        locals.var_d0__blk1413_dn6 = assign52650_e67899_d_n6;
        locals.var_d0__blk1413_dn7 = assign52650_e67899_d_n7;
        locals.var_d0__blk1413_dn8 = assign52650_e67899_d_n8;
        locals.var_d0__blk1413_rv = 0.0;

        let (assign52660_e67917, assign52660_e67917_d_n5, assign52660_e67917_d_n6, assign52660_e67917_d_n7, assign52660_e67917_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52660_e67912: f64 = (locals.var_kp * locals.var_xgm__blk1409);
        let assign52660_e67913: f64 = (1.0 + assign52660_e67912);
        let assign52660_e67914: f64 = (assign52660_e67913).sqrt();
        let assign52660_e67915: f64 = (1.0 / assign52660_e67914);
        (assign52660_e67915, (-(((locals.var_kp * locals.var_xgm__blk1409_dn5) / (2.0 * assign52660_e67914)) / (assign52660_e67914 * assign52660_e67914))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn6) / (2.0 * assign52660_e67914)) / (assign52660_e67914 * assign52660_e67914))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn7) / (2.0 * assign52660_e67914)) / (assign52660_e67914 * assign52660_e67914))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn8) / (2.0 * assign52660_e67914)) / (assign52660_e67914 * assign52660_e67914))),)
    } else {
        (locals.var_eta_p__blk1410, locals.var_eta_p__blk1410_dn5, locals.var_eta_p__blk1410_dn6, locals.var_eta_p__blk1410_dn7, locals.var_eta_p__blk1410_dn8,)
    }
};
        locals.var_eta_p__blk1410 = assign52660_e67917;
        locals.var_eta_p__blk1410_dn5 = assign52660_e67917_d_n5;
        locals.var_eta_p__blk1410_dn6 = assign52660_e67917_d_n6;
        locals.var_eta_p__blk1410_dn7 = assign52660_e67917_d_n7;
        locals.var_eta_p__blk1410_dn8 = assign52660_e67917_d_n8;
        locals.var_eta_p__blk1410_rv = 0.0;

        let (assign52670_e67932, assign52670_e67932_d_n5, assign52670_e67932_d_n6, assign52670_e67932_d_n7, assign52670_e67932_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52670_e67929: f64 = (locals.var_eta_p__blk1410 + 1.0);
        let assign52670_e67930: f64 = (locals.var_eta_p__blk1410 / assign52670_e67929);
        (assign52670_e67930, (((locals.var_eta_p__blk1410_dn5 * assign52670_e67929) - (locals.var_eta_p__blk1410 * locals.var_eta_p__blk1410_dn5)) / (assign52670_e67929 * assign52670_e67929)), (((locals.var_eta_p__blk1410_dn6 * assign52670_e67929) - (locals.var_eta_p__blk1410 * locals.var_eta_p__blk1410_dn6)) / (assign52670_e67929 * assign52670_e67929)), (((locals.var_eta_p__blk1410_dn7 * assign52670_e67929) - (locals.var_eta_p__blk1410 * locals.var_eta_p__blk1410_dn7)) / (assign52670_e67929 * assign52670_e67929)), (((locals.var_eta_p__blk1410_dn8 * assign52670_e67929) - (locals.var_eta_p__blk1410 * locals.var_eta_p__blk1410_dn8)) / (assign52670_e67929 * assign52670_e67929)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52670_e67932;
        locals.var_temp__blk936_dn5 = assign52670_e67932_d_n5;
        locals.var_temp__blk936_dn6 = assign52670_e67932_d_n6;
        locals.var_temp__blk936_dn7 = assign52670_e67932_d_n7;
        locals.var_temp__blk936_dn8 = assign52670_e67932_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign52680_e67951, assign52680_e67951_d_n5, assign52680_e67951_d_n6, assign52680_e67951_d_n7, assign52680_e67951_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52680_e67944: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
        let assign52680_e67946: f64 = (assign52680_e67944 * locals.var_gf2__blk1308);
        let assign52680_e67948: f64 = (assign52680_e67946 * locals.var_dm__blk1407);
        let assign52680_e67949: f64 = (locals.var_kp * assign52680_e67948);
        (assign52680_e67949, (locals.var_kp * ((((((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) * locals.var_gf2__blk1308) + (assign52680_e67944 * locals.var_gf2__blk1308_dn5)) * locals.var_dm__blk1407) + (assign52680_e67946 * locals.var_dm__blk1407_dn5))), (locals.var_kp * ((((((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) * locals.var_gf2__blk1308) + (assign52680_e67944 * locals.var_gf2__blk1308_dn6)) * locals.var_dm__blk1407) + (assign52680_e67946 * locals.var_dm__blk1407_dn6))), (locals.var_kp * ((((((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) * locals.var_gf2__blk1308) + (assign52680_e67944 * locals.var_gf2__blk1308_dn7)) * locals.var_dm__blk1407) + (assign52680_e67946 * locals.var_dm__blk1407_dn7))), (locals.var_kp * ((((((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) * locals.var_gf2__blk1308) + (assign52680_e67944 * locals.var_gf2__blk1308_dn8)) * locals.var_dm__blk1407) + (assign52680_e67946 * locals.var_dm__blk1407_dn8))),)
    } else {
        (locals.var_x_pm__blk1414, locals.var_x_pm__blk1414_dn5, locals.var_x_pm__blk1414_dn6, locals.var_x_pm__blk1414_dn7, locals.var_x_pm__blk1414_dn8,)
    }
};
        locals.var_x_pm__blk1414 = assign52680_e67951;
        locals.var_x_pm__blk1414_dn5 = assign52680_e67951_d_n5;
        locals.var_x_pm__blk1414_dn6 = assign52680_e67951_d_n6;
        locals.var_x_pm__blk1414_dn7 = assign52680_e67951_d_n7;
        locals.var_x_pm__blk1414_dn8 = assign52680_e67951_d_n8;
        locals.var_x_pm__blk1414_rv = 0.0;

        let (assign52690_e67974, assign52690_e67974_d_n5, assign52690_e67974_d_n6, assign52690_e67974_d_n7, assign52690_e67974_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52690_e67963: f64 = (locals.var_xgm__blk1409 - locals.var_x_pm__blk1414);
        let assign52690_e67964: f64 = (2.0 * assign52690_e67963);
        let assign52690_e67968: f64 = (1.0 - locals.var_em__blk1405);
        let assign52690_e67970: f64 = (assign52690_e67968 + locals.var_dm__blk1407);
        let assign52690_e67971: f64 = (locals.var_gf2__blk1308 * assign52690_e67970);
        let assign52690_e67972: f64 = (assign52690_e67964 + assign52690_e67971);
        (assign52690_e67972, ((2.0 * (locals.var_xgm__blk1409_dn5 - locals.var_x_pm__blk1414_dn5)) + ((locals.var_gf2__blk1308_dn5 * assign52690_e67970) + (locals.var_gf2__blk1308 * ((-locals.var_em__blk1405_dn5) + locals.var_dm__blk1407_dn5)))), ((2.0 * (locals.var_xgm__blk1409_dn6 - locals.var_x_pm__blk1414_dn6)) + ((locals.var_gf2__blk1308_dn6 * assign52690_e67970) + (locals.var_gf2__blk1308 * ((-locals.var_em__blk1405_dn6) + locals.var_dm__blk1407_dn6)))), ((2.0 * (locals.var_xgm__blk1409_dn7 - locals.var_x_pm__blk1414_dn7)) + ((locals.var_gf2__blk1308_dn7 * assign52690_e67970) + (locals.var_gf2__blk1308 * ((-locals.var_em__blk1405_dn7) + locals.var_dm__blk1407_dn7)))), ((2.0 * (locals.var_xgm__blk1409_dn8 - locals.var_x_pm__blk1414_dn8)) + ((locals.var_gf2__blk1308_dn8 * assign52690_e67970) + (locals.var_gf2__blk1308 * ((-locals.var_em__blk1405_dn8) + locals.var_dm__blk1407_dn8)))),)
    } else {
        (locals.var_p_pd__blk1415, locals.var_p_pd__blk1415_dn5, locals.var_p_pd__blk1415_dn6, locals.var_p_pd__blk1415_dn7, locals.var_p_pd__blk1415_dn8,)
    }
};
        locals.var_p_pd__blk1415 = assign52690_e67974;
        locals.var_p_pd__blk1415_dn5 = assign52690_e67974_d_n5;
        locals.var_p_pd__blk1415_dn6 = assign52690_e67974_d_n6;
        locals.var_p_pd__blk1415_dn7 = assign52690_e67974_d_n7;
        locals.var_p_pd__blk1415_dn8 = assign52690_e67974_d_n8;
        locals.var_p_pd__blk1415_rv = 0.0;

        let (assign52700_e67991, assign52700_e67991_d_n5, assign52700_e67991_d_n6, assign52700_e67991_d_n7, assign52700_e67991_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52700_e67987: f64 = (2.0 * locals.var_xgm__blk1409);
        let assign52700_e67988: f64 = (locals.var_x_pm__blk1414 - assign52700_e67987);
        let assign52700_e67989: f64 = (locals.var_x_pm__blk1414 * assign52700_e67988);
        (assign52700_e67989, ((locals.var_x_pm__blk1414_dn5 * assign52700_e67988) + (locals.var_x_pm__blk1414 * (locals.var_x_pm__blk1414_dn5 - (2.0 * locals.var_xgm__blk1409_dn5)))), ((locals.var_x_pm__blk1414_dn6 * assign52700_e67988) + (locals.var_x_pm__blk1414 * (locals.var_x_pm__blk1414_dn6 - (2.0 * locals.var_xgm__blk1409_dn6)))), ((locals.var_x_pm__blk1414_dn7 * assign52700_e67988) + (locals.var_x_pm__blk1414 * (locals.var_x_pm__blk1414_dn7 - (2.0 * locals.var_xgm__blk1409_dn7)))), ((locals.var_x_pm__blk1414_dn8 * assign52700_e67988) + (locals.var_x_pm__blk1414 * (locals.var_x_pm__blk1414_dn8 - (2.0 * locals.var_xgm__blk1409_dn8)))),)
    } else {
        (locals.var_q_pd__blk1416, locals.var_q_pd__blk1416_dn5, locals.var_q_pd__blk1416_dn6, locals.var_q_pd__blk1416_dn7, locals.var_q_pd__blk1416_dn8,)
    }
};
        locals.var_q_pd__blk1416 = assign52700_e67991;
        locals.var_q_pd__blk1416_dn5 = assign52700_e67991_d_n5;
        locals.var_q_pd__blk1416_dn6 = assign52700_e67991_d_n6;
        locals.var_q_pd__blk1416_dn7 = assign52700_e67991_d_n7;
        locals.var_q_pd__blk1416_dn8 = assign52700_e67991_d_n8;
        locals.var_q_pd__blk1416_rv = 0.0;

        let (assign52710_e68010, assign52710_e68010_d_n5, assign52710_e68010_d_n6, assign52710_e68010_d_n7, assign52710_e68010_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52710_e68005: f64 = (locals.var_em__blk1405 + locals.var_dm__blk1407);
        let assign52710_e68006: f64 = (locals.var_gf2__blk1308 * assign52710_e68005);
        let assign52710_e68007: f64 = (0.5 * assign52710_e68006);
        let assign52710_e68008: f64 = (1.0 - assign52710_e68007);
        (assign52710_e68008, (-(0.5 * ((locals.var_gf2__blk1308_dn5 * assign52710_e68005) + (locals.var_gf2__blk1308 * (locals.var_em__blk1405_dn5 + locals.var_dm__blk1407_dn5))))), (-(0.5 * ((locals.var_gf2__blk1308_dn6 * assign52710_e68005) + (locals.var_gf2__blk1308 * (locals.var_em__blk1405_dn6 + locals.var_dm__blk1407_dn6))))), (-(0.5 * ((locals.var_gf2__blk1308_dn7 * assign52710_e68005) + (locals.var_gf2__blk1308 * (locals.var_em__blk1405_dn7 + locals.var_dm__blk1407_dn7))))), (-(0.5 * ((locals.var_gf2__blk1308_dn8 * assign52710_e68005) + (locals.var_gf2__blk1308 * (locals.var_em__blk1405_dn8 + locals.var_dm__blk1407_dn8))))),)
    } else {
        (locals.var_xi_pd__blk1417, locals.var_xi_pd__blk1417_dn5, locals.var_xi_pd__blk1417_dn6, locals.var_xi_pd__blk1417_dn7, locals.var_xi_pd__blk1417_dn8,)
    }
};
        locals.var_xi_pd__blk1417 = assign52710_e68010;
        locals.var_xi_pd__blk1417_dn5 = assign52710_e68010_d_n5;
        locals.var_xi_pd__blk1417_dn6 = assign52710_e68010_d_n6;
        locals.var_xi_pd__blk1417_dn7 = assign52710_e68010_d_n7;
        locals.var_xi_pd__blk1417_dn8 = assign52710_e68010_d_n8;
        locals.var_xi_pd__blk1417_rv = 0.0;

        let (assign52720_e68031, assign52720_e68031_d_n5, assign52720_e68031_d_n6, assign52720_e68031_d_n7, assign52720_e68031_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52720_e68021: f64 = (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415);
        let assign52720_e68024: f64 = (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415);
        let assign52720_e68027: f64 = (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416);
        let assign52720_e68028: f64 = (assign52720_e68024 - assign52720_e68027);
        let assign52720_e68029: f64 = (assign52720_e68021 / assign52720_e68028);
        (assign52720_e68029, (((((locals.var_q_pd__blk1416_dn5 * locals.var_p_pd__blk1415) + (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415_dn5)) * assign52720_e68028) - (assign52720_e68021 * (((locals.var_p_pd__blk1415_dn5 * locals.var_p_pd__blk1415) + (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415_dn5)) - ((locals.var_xi_pd__blk1417_dn5 * locals.var_q_pd__blk1416) + (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416_dn5))))) / (assign52720_e68028 * assign52720_e68028)), (((((locals.var_q_pd__blk1416_dn6 * locals.var_p_pd__blk1415) + (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415_dn6)) * assign52720_e68028) - (assign52720_e68021 * (((locals.var_p_pd__blk1415_dn6 * locals.var_p_pd__blk1415) + (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415_dn6)) - ((locals.var_xi_pd__blk1417_dn6 * locals.var_q_pd__blk1416) + (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416_dn6))))) / (assign52720_e68028 * assign52720_e68028)), (((((locals.var_q_pd__blk1416_dn7 * locals.var_p_pd__blk1415) + (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415_dn7)) * assign52720_e68028) - (assign52720_e68021 * (((locals.var_p_pd__blk1415_dn7 * locals.var_p_pd__blk1415) + (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415_dn7)) - ((locals.var_xi_pd__blk1417_dn7 * locals.var_q_pd__blk1416) + (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416_dn7))))) / (assign52720_e68028 * assign52720_e68028)), (((((locals.var_q_pd__blk1416_dn8 * locals.var_p_pd__blk1415) + (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415_dn8)) * assign52720_e68028) - (assign52720_e68021 * (((locals.var_p_pd__blk1415_dn8 * locals.var_p_pd__blk1415) + (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415_dn8)) - ((locals.var_xi_pd__blk1417_dn8 * locals.var_q_pd__blk1416) + (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416_dn8))))) / (assign52720_e68028 * assign52720_e68028)),)
    } else {
        (locals.var_u_pd__blk1418, locals.var_u_pd__blk1418_dn5, locals.var_u_pd__blk1418_dn6, locals.var_u_pd__blk1418_dn7, locals.var_u_pd__blk1418_dn8,)
    }
};
        locals.var_u_pd__blk1418 = assign52720_e68031;
        locals.var_u_pd__blk1418_dn5 = assign52720_e68031_d_n5;
        locals.var_u_pd__blk1418_dn6 = assign52720_e68031_d_n6;
        locals.var_u_pd__blk1418_dn7 = assign52720_e68031_d_n7;
        locals.var_u_pd__blk1418_dn8 = assign52720_e68031_d_n8;
        locals.var_u_pd__blk1418_rv = 0.0;

        let (assign52730_e68044, assign52730_e68044_d_n5, assign52730_e68044_d_n6, assign52730_e68044_d_n7, assign52730_e68044_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52730_e68042: f64 = (locals.var_x_m__blk1404 + locals.var_u_pd__blk1418);
        (assign52730_e68042, (locals.var_x_m__blk1404_dn5 + locals.var_u_pd__blk1418_dn5), (locals.var_x_m__blk1404_dn6 + locals.var_u_pd__blk1418_dn6), (locals.var_x_m__blk1404_dn7 + locals.var_u_pd__blk1418_dn7), (locals.var_x_m__blk1404_dn8 + locals.var_u_pd__blk1418_dn8),)
    } else {
        (locals.var_x_m__blk1404, locals.var_x_m__blk1404_dn5, locals.var_x_m__blk1404_dn6, locals.var_x_m__blk1404_dn7, locals.var_x_m__blk1404_dn8,)
    }
};
        locals.var_x_m__blk1404 = assign52730_e68044;
        locals.var_x_m__blk1404_dn5 = assign52730_e68044_d_n5;
        locals.var_x_m__blk1404_dn6 = assign52730_e68044_d_n6;
        locals.var_x_m__blk1404_dn7 = assign52730_e68044_d_n7;
        locals.var_x_m__blk1404_dn8 = assign52730_e68044_d_n8;
        locals.var_x_m__blk1404_rv = 0.0;

        let (assign52740_e68056, assign52740_e68056_d_n5, assign52740_e68056_d_n6, assign52740_e68056_d_n7, assign52740_e68056_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52740_e68054: f64 = (locals.var_u_pd__blk1418).exp();
        (assign52740_e68054, (assign52740_e68054 * locals.var_u_pd__blk1418_dn5), (assign52740_e68054 * locals.var_u_pd__blk1418_dn6), (assign52740_e68054 * locals.var_u_pd__blk1418_dn7), (assign52740_e68054 * locals.var_u_pd__blk1418_dn8),)
    } else {
        (locals.var_km__blk1419, locals.var_km__blk1419_dn5, locals.var_km__blk1419_dn6, locals.var_km__blk1419_dn7, locals.var_km__blk1419_dn8,)
    }
};
        locals.var_km__blk1419 = assign52740_e68056;
        locals.var_km__blk1419_dn5 = assign52740_e68056_d_n5;
        locals.var_km__blk1419_dn6 = assign52740_e68056_d_n6;
        locals.var_km__blk1419_dn7 = assign52740_e68056_d_n7;
        locals.var_km__blk1419_dn8 = assign52740_e68056_d_n8;
        locals.var_km__blk1419_rv = 0.0;

        let (assign52750_e68069, assign52750_e68069_d_n5, assign52750_e68069_d_n6, assign52750_e68069_d_n7, assign52750_e68069_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52750_e68067: f64 = (locals.var_em__blk1405 / locals.var_km__blk1419);
        (assign52750_e68067, (((locals.var_em__blk1405_dn5 * locals.var_km__blk1419) - (locals.var_em__blk1405 * locals.var_km__blk1419_dn5)) / (locals.var_km__blk1419 * locals.var_km__blk1419)), (((locals.var_em__blk1405_dn6 * locals.var_km__blk1419) - (locals.var_em__blk1405 * locals.var_km__blk1419_dn6)) / (locals.var_km__blk1419 * locals.var_km__blk1419)), (((locals.var_em__blk1405_dn7 * locals.var_km__blk1419) - (locals.var_em__blk1405 * locals.var_km__blk1419_dn7)) / (locals.var_km__blk1419 * locals.var_km__blk1419)), (((locals.var_em__blk1405_dn8 * locals.var_km__blk1419) - (locals.var_em__blk1405 * locals.var_km__blk1419_dn8)) / (locals.var_km__blk1419 * locals.var_km__blk1419)),)
    } else {
        (locals.var_em__blk1405, locals.var_em__blk1405_dn5, locals.var_em__blk1405_dn6, locals.var_em__blk1405_dn7, locals.var_em__blk1405_dn8,)
    }
};
        locals.var_em__blk1405 = assign52750_e68069;
        locals.var_em__blk1405_dn5 = assign52750_e68069_d_n5;
        locals.var_em__blk1405_dn6 = assign52750_e68069_d_n6;
        locals.var_em__blk1405_dn7 = assign52750_e68069_d_n7;
        locals.var_em__blk1405_dn8 = assign52750_e68069_d_n8;
        locals.var_em__blk1405_rv = 0.0;

        let (assign52760_e68082, assign52760_e68082_d_n5, assign52760_e68082_d_n6, assign52760_e68082_d_n7, assign52760_e68082_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52760_e68080: f64 = (locals.var_dm__blk1407 * locals.var_km__blk1419);
        (assign52760_e68080, ((locals.var_dm__blk1407_dn5 * locals.var_km__blk1419) + (locals.var_dm__blk1407 * locals.var_km__blk1419_dn5)), ((locals.var_dm__blk1407_dn6 * locals.var_km__blk1419) + (locals.var_dm__blk1407 * locals.var_km__blk1419_dn6)), ((locals.var_dm__blk1407_dn7 * locals.var_km__blk1419) + (locals.var_dm__blk1407 * locals.var_km__blk1419_dn7)), ((locals.var_dm__blk1407_dn8 * locals.var_km__blk1419) + (locals.var_dm__blk1407 * locals.var_km__blk1419_dn8)),)
    } else {
        (locals.var_dm__blk1407, locals.var_dm__blk1407_dn5, locals.var_dm__blk1407_dn6, locals.var_dm__blk1407_dn7, locals.var_dm__blk1407_dn8,)
    }
};
        locals.var_dm__blk1407 = assign52760_e68082;
        locals.var_dm__blk1407_dn5 = assign52760_e68082_d_n5;
        locals.var_dm__blk1407_dn6 = assign52760_e68082_d_n6;
        locals.var_dm__blk1407_dn7 = assign52760_e68082_d_n7;
        locals.var_dm__blk1407_dn8 = assign52760_e68082_d_n8;
        locals.var_dm__blk1407_rv = 0.0;

        let (assign52770_e68097, assign52770_e68097_d_n5, assign52770_e68097_d_n6, assign52770_e68097_d_n7, assign52770_e68097_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52770_e68093: f64 = (locals.var_x_m__blk1404 - 1.0);
        let assign52770_e68095: f64 = (assign52770_e68093 + locals.var_em__blk1405);
        (assign52770_e68095, (locals.var_x_m__blk1404_dn5 + locals.var_em__blk1405_dn5), (locals.var_x_m__blk1404_dn6 + locals.var_em__blk1405_dn6), (locals.var_x_m__blk1404_dn7 + locals.var_em__blk1405_dn7), (locals.var_x_m__blk1404_dn8 + locals.var_em__blk1405_dn8),)
    } else {
        (locals.var_pm__blk1408, locals.var_pm__blk1408_dn5, locals.var_pm__blk1408_dn6, locals.var_pm__blk1408_dn7, locals.var_pm__blk1408_dn8,)
    }
};
        locals.var_pm__blk1408 = assign52770_e68097;
        locals.var_pm__blk1408_dn5 = assign52770_e68097_d_n5;
        locals.var_pm__blk1408_dn6 = assign52770_e68097_d_n6;
        locals.var_pm__blk1408_dn7 = assign52770_e68097_d_n7;
        locals.var_pm__blk1408_dn8 = assign52770_e68097_d_n8;
        locals.var_pm__blk1408_rv = 0.0;

        let (assign52780_e68113, assign52780_e68113_d_n5, assign52780_e68113_d_n6, assign52780_e68113_d_n7, assign52780_e68113_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52780_e68109: f64 = (locals.var_dm__blk1407 + locals.var_pm__blk1408);
        let assign52780_e68110: f64 = (assign52780_e68109).sqrt();
        let assign52780_e68111: f64 = (locals.var_gf__blk1307 * assign52780_e68110);
        (assign52780_e68111, ((locals.var_gf__blk1307_dn5 * assign52780_e68110) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn5 + locals.var_pm__blk1408_dn5) / (2.0 * assign52780_e68110)))), ((locals.var_gf__blk1307_dn6 * assign52780_e68110) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn6 + locals.var_pm__blk1408_dn6) / (2.0 * assign52780_e68110)))), ((locals.var_gf__blk1307_dn7 * assign52780_e68110) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn7 + locals.var_pm__blk1408_dn7) / (2.0 * assign52780_e68110)))), ((locals.var_gf__blk1307_dn8 * assign52780_e68110) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn8 + locals.var_pm__blk1408_dn8) / (2.0 * assign52780_e68110)))),)
    } else {
        (locals.var_xgm__blk1409, locals.var_xgm__blk1409_dn5, locals.var_xgm__blk1409_dn6, locals.var_xgm__blk1409_dn7, locals.var_xgm__blk1409_dn8,)
    }
};
        locals.var_xgm__blk1409 = assign52780_e68113;
        locals.var_xgm__blk1409_dn5 = assign52780_e68113_d_n5;
        locals.var_xgm__blk1409_dn6 = assign52780_e68113_d_n6;
        locals.var_xgm__blk1409_dn7 = assign52780_e68113_d_n7;
        locals.var_xgm__blk1409_dn8 = assign52780_e68113_d_n8;
        locals.var_xgm__blk1409_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_49(
        locals: &mut StampLocals,
    ) {
        let (assign52790_e68134, assign52790_e68134_d_n5, assign52790_e68134_d_n6, assign52790_e68134_d_n7, assign52790_e68134_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52790_e68124: f64 = (1.0 - locals.var_em__blk1405);
        let assign52790_e68128: f64 = (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410);
        let assign52790_e68130: f64 = (assign52790_e68128 * locals.var_inv_gf2__blk1324);
        let assign52790_e68131: f64 = (2.0 * assign52790_e68130);
        let assign52790_e68132: f64 = (assign52790_e68124 + assign52790_e68131);
        (assign52790_e68132, ((-locals.var_em__blk1405_dn5) + (2.0 * ((((locals.var_xgm__blk1409_dn5 * locals.var_eta_p__blk1410) + (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410_dn5)) * locals.var_inv_gf2__blk1324) + (assign52790_e68128 * locals.var_inv_gf2__blk1324_dn5)))), ((-locals.var_em__blk1405_dn6) + (2.0 * ((((locals.var_xgm__blk1409_dn6 * locals.var_eta_p__blk1410) + (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410_dn6)) * locals.var_inv_gf2__blk1324) + (assign52790_e68128 * locals.var_inv_gf2__blk1324_dn6)))), ((-locals.var_em__blk1405_dn7) + (2.0 * ((((locals.var_xgm__blk1409_dn7 * locals.var_eta_p__blk1410) + (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410_dn7)) * locals.var_inv_gf2__blk1324) + (assign52790_e68128 * locals.var_inv_gf2__blk1324_dn7)))), ((-locals.var_em__blk1405_dn8) + (2.0 * ((((locals.var_xgm__blk1409_dn8 * locals.var_eta_p__blk1410) + (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410_dn8)) * locals.var_inv_gf2__blk1324) + (assign52790_e68128 * locals.var_inv_gf2__blk1324_dn8)))),)
    } else {
        (locals.var_km0__blk1420, locals.var_km0__blk1420_dn5, locals.var_km0__blk1420_dn6, locals.var_km0__blk1420_dn7, locals.var_km0__blk1420_dn8,)
    }
};
        locals.var_km0__blk1420 = assign52790_e68134;
        locals.var_km0__blk1420_dn5 = assign52790_e68134_d_n5;
        locals.var_km0__blk1420_dn6 = assign52790_e68134_d_n6;
        locals.var_km0__blk1420_dn7 = assign52790_e68134_d_n7;
        locals.var_km0__blk1420_dn8 = assign52790_e68134_d_n8;
        locals.var_km0__blk1420_rv = 0.0;

        let (assign52800_e68157, assign52800_e68157_d_n5, assign52800_e68157_d_n6, assign52800_e68157_d_n7, assign52800_e68157_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52800_e68145: f64 = (locals.var_x_ds__blk1394 * locals.var_km__blk1419);
        let assign52800_e68148: f64 = (locals.var_d0__blk1413 + locals.var_d_bar__blk1406);
        let assign52800_e68149: f64 = (assign52800_e68145 * assign52800_e68148);
        let assign52800_e68153: f64 = (locals.var_km__blk1419 * locals.var_d_bar__blk1406);
        let assign52800_e68154: f64 = (locals.var_km0__blk1420 + assign52800_e68153);
        let assign52800_e68155: f64 = (assign52800_e68149 / assign52800_e68154);
        (assign52800_e68155, (((((((locals.var_x_ds__blk1394_dn5 * locals.var_km__blk1419) + (locals.var_x_ds__blk1394 * locals.var_km__blk1419_dn5)) * assign52800_e68148) + (assign52800_e68145 * (locals.var_d0__blk1413_dn5 + locals.var_d_bar__blk1406_dn5))) * assign52800_e68154) - (assign52800_e68149 * (locals.var_km0__blk1420_dn5 + ((locals.var_km__blk1419_dn5 * locals.var_d_bar__blk1406) + (locals.var_km__blk1419 * locals.var_d_bar__blk1406_dn5))))) / (assign52800_e68154 * assign52800_e68154)), (((((((locals.var_x_ds__blk1394_dn6 * locals.var_km__blk1419) + (locals.var_x_ds__blk1394 * locals.var_km__blk1419_dn6)) * assign52800_e68148) + (assign52800_e68145 * (locals.var_d0__blk1413_dn6 + locals.var_d_bar__blk1406_dn6))) * assign52800_e68154) - (assign52800_e68149 * (locals.var_km0__blk1420_dn6 + ((locals.var_km__blk1419_dn6 * locals.var_d_bar__blk1406) + (locals.var_km__blk1419 * locals.var_d_bar__blk1406_dn6))))) / (assign52800_e68154 * assign52800_e68154)), (((((((locals.var_x_ds__blk1394_dn7 * locals.var_km__blk1419) + (locals.var_x_ds__blk1394 * locals.var_km__blk1419_dn7)) * assign52800_e68148) + (assign52800_e68145 * (locals.var_d0__blk1413_dn7 + locals.var_d_bar__blk1406_dn7))) * assign52800_e68154) - (assign52800_e68149 * (locals.var_km0__blk1420_dn7 + ((locals.var_km__blk1419_dn7 * locals.var_d_bar__blk1406) + (locals.var_km__blk1419 * locals.var_d_bar__blk1406_dn7))))) / (assign52800_e68154 * assign52800_e68154)), (((((((locals.var_x_ds__blk1394_dn8 * locals.var_km__blk1419) + (locals.var_x_ds__blk1394 * locals.var_km__blk1419_dn8)) * assign52800_e68148) + (assign52800_e68145 * (locals.var_d0__blk1413_dn8 + locals.var_d_bar__blk1406_dn8))) * assign52800_e68154) - (assign52800_e68149 * (locals.var_km0__blk1420_dn8 + ((locals.var_km__blk1419_dn8 * locals.var_d_bar__blk1406) + (locals.var_km__blk1419 * locals.var_d_bar__blk1406_dn8))))) / (assign52800_e68154 * assign52800_e68154)),)
    } else {
        (locals.var_x_ds__blk1394, locals.var_x_ds__blk1394_dn5, locals.var_x_ds__blk1394_dn6, locals.var_x_ds__blk1394_dn7, locals.var_x_ds__blk1394_dn8,)
    }
};
        locals.var_x_ds__blk1394 = assign52800_e68157;
        locals.var_x_ds__blk1394_dn5 = assign52800_e68157_d_n5;
        locals.var_x_ds__blk1394_dn6 = assign52800_e68157_d_n6;
        locals.var_x_ds__blk1394_dn7 = assign52800_e68157_d_n7;
        locals.var_x_ds__blk1394_dn8 = assign52800_e68157_d_n8;
        locals.var_x_ds__blk1394_rv = 0.0;

        let (assign52810_e68170, assign52810_e68170_d_n5, assign52810_e68170_d_n6, assign52810_e68170_d_n7, assign52810_e68170_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52810_e68168: f64 = (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322);
        (assign52810_e68168, ((locals.var_x_ds__blk1394_dn5 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn5)), ((locals.var_x_ds__blk1394_dn6 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn6)), ((locals.var_x_ds__blk1394_dn7 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn7)), ((locals.var_x_ds__blk1394_dn8 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_dps__blk1397, locals.var_dps__blk1397_dn5, locals.var_dps__blk1397_dn6, locals.var_dps__blk1397_dn7, locals.var_dps__blk1397_dn8,)
    }
};
        locals.var_dps__blk1397 = assign52810_e68170;
        locals.var_dps__blk1397_dn5 = assign52810_e68170_d_n5;
        locals.var_dps__blk1397_dn6 = assign52810_e68170_d_n6;
        locals.var_dps__blk1397_dn7 = assign52810_e68170_d_n7;
        locals.var_dps__blk1397_dn8 = assign52810_e68170_d_n8;
        locals.var_dps__blk1397_rv = 0.0;

        let (assign52820_e68180, assign52820_e68180_d_n5, assign52820_e68180_d_n6, assign52820_e68180_d_n7, assign52820_e68180_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) {
        let assign52820_e68178: f64 = (locals.var_pm__blk1408).sqrt();
        (assign52820_e68178, (locals.var_pm__blk1408_dn5 / (2.0 * assign52820_e68178)), (locals.var_pm__blk1408_dn6 / (2.0 * assign52820_e68178)), (locals.var_pm__blk1408_dn7 / (2.0 * assign52820_e68178)), (locals.var_pm__blk1408_dn8 / (2.0 * assign52820_e68178)),)
    } else {
        (locals.var_sqm__blk1411, locals.var_sqm__blk1411_dn5, locals.var_sqm__blk1411_dn6, locals.var_sqm__blk1411_dn7, locals.var_sqm__blk1411_dn8,)
    }
};
        locals.var_sqm__blk1411 = assign52820_e68180;
        locals.var_sqm__blk1411_dn5 = assign52820_e68180_d_n5;
        locals.var_sqm__blk1411_dn6 = assign52820_e68180_d_n6;
        locals.var_sqm__blk1411_dn7 = assign52820_e68180_d_n7;
        locals.var_sqm__blk1411_dn8 = assign52820_e68180_d_n8;
        locals.var_sqm__blk1411_rv = 0.0;

        let (assign52830_e68199, assign52830_e68199_d_n5, assign52830_e68199_d_n6, assign52830_e68199_d_n7, assign52830_e68199_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) {
        let assign52830_e68192: f64 = (1.0 - locals.var_em__blk1405);
        let assign52830_e68193: f64 = (locals.var_gf__blk1307 * assign52830_e68192);
        let assign52830_e68195: f64 = (assign52830_e68193 / locals.var_sqm__blk1411);
        let assign52830_e68196: f64 = (0.5 * assign52830_e68195);
        let assign52830_e68197: f64 = (locals.var_eta_p__blk1410 + assign52830_e68196);
        (assign52830_e68197, (locals.var_eta_p__blk1410_dn5 + (0.5 * (((((locals.var_gf__blk1307_dn5 * assign52830_e68192) + (locals.var_gf__blk1307 * (-locals.var_em__blk1405_dn5))) * locals.var_sqm__blk1411) - (assign52830_e68193 * locals.var_sqm__blk1411_dn5)) / (locals.var_sqm__blk1411 * locals.var_sqm__blk1411)))), (locals.var_eta_p__blk1410_dn6 + (0.5 * (((((locals.var_gf__blk1307_dn6 * assign52830_e68192) + (locals.var_gf__blk1307 * (-locals.var_em__blk1405_dn6))) * locals.var_sqm__blk1411) - (assign52830_e68193 * locals.var_sqm__blk1411_dn6)) / (locals.var_sqm__blk1411 * locals.var_sqm__blk1411)))), (locals.var_eta_p__blk1410_dn7 + (0.5 * (((((locals.var_gf__blk1307_dn7 * assign52830_e68192) + (locals.var_gf__blk1307 * (-locals.var_em__blk1405_dn7))) * locals.var_sqm__blk1411) - (assign52830_e68193 * locals.var_sqm__blk1411_dn7)) / (locals.var_sqm__blk1411 * locals.var_sqm__blk1411)))), (locals.var_eta_p__blk1410_dn8 + (0.5 * (((((locals.var_gf__blk1307_dn8 * assign52830_e68192) + (locals.var_gf__blk1307 * (-locals.var_em__blk1405_dn8))) * locals.var_sqm__blk1411) - (assign52830_e68193 * locals.var_sqm__blk1411_dn8)) / (locals.var_sqm__blk1411 * locals.var_sqm__blk1411)))),)
    } else {
        (locals.var_alpha__blk1412, locals.var_alpha__blk1412_dn5, locals.var_alpha__blk1412_dn6, locals.var_alpha__blk1412_dn7, locals.var_alpha__blk1412_dn8,)
    }
};
        locals.var_alpha__blk1412 = assign52830_e68199;
        locals.var_alpha__blk1412_dn5 = assign52830_e68199_d_n5;
        locals.var_alpha__blk1412_dn6 = assign52830_e68199_d_n6;
        locals.var_alpha__blk1412_dn7 = assign52830_e68199_d_n7;
        locals.var_alpha__blk1412_dn8 = assign52830_e68199_d_n8;
        locals.var_alpha__blk1412_rv = 0.0;

        let (assign52840_e68215, assign52840_e68215_d_n5, assign52840_e68215_d_n6, assign52840_e68215_d_n7, assign52840_e68215_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52840_e68206: f64 = (locals.var_gf2__blk1308 * locals.var_dm__blk1407);
        let assign52840_e68210: f64 = (locals.var_gf__blk1307 * locals.var_sqm__blk1411);
        let assign52840_e68211: f64 = (locals.var_xgm__blk1409 + assign52840_e68210);
        let assign52840_e68212: f64 = (assign52840_e68206 / assign52840_e68211);
        let assign52840_e68213: f64 = (locals.var_phit1__blk1322 * assign52840_e68212);
        (assign52840_e68213, ((locals.var_phit1__blk1322_dn5 * assign52840_e68212) + (locals.var_phit1__blk1322 * (((((locals.var_gf2__blk1308_dn5 * locals.var_dm__blk1407) + (locals.var_gf2__blk1308 * locals.var_dm__blk1407_dn5)) * assign52840_e68211) - (assign52840_e68206 * (locals.var_xgm__blk1409_dn5 + ((locals.var_gf__blk1307_dn5 * locals.var_sqm__blk1411) + (locals.var_gf__blk1307 * locals.var_sqm__blk1411_dn5))))) / (assign52840_e68211 * assign52840_e68211)))), ((locals.var_phit1__blk1322_dn6 * assign52840_e68212) + (locals.var_phit1__blk1322 * (((((locals.var_gf2__blk1308_dn6 * locals.var_dm__blk1407) + (locals.var_gf2__blk1308 * locals.var_dm__blk1407_dn6)) * assign52840_e68211) - (assign52840_e68206 * (locals.var_xgm__blk1409_dn6 + ((locals.var_gf__blk1307_dn6 * locals.var_sqm__blk1411) + (locals.var_gf__blk1307 * locals.var_sqm__blk1411_dn6))))) / (assign52840_e68211 * assign52840_e68211)))), ((locals.var_phit1__blk1322_dn7 * assign52840_e68212) + (locals.var_phit1__blk1322 * (((((locals.var_gf2__blk1308_dn7 * locals.var_dm__blk1407) + (locals.var_gf2__blk1308 * locals.var_dm__blk1407_dn7)) * assign52840_e68211) - (assign52840_e68206 * (locals.var_xgm__blk1409_dn7 + ((locals.var_gf__blk1307_dn7 * locals.var_sqm__blk1411) + (locals.var_gf__blk1307 * locals.var_sqm__blk1411_dn7))))) / (assign52840_e68211 * assign52840_e68211)))), ((locals.var_phit1__blk1322_dn8 * assign52840_e68212) + (locals.var_phit1__blk1322 * (((((locals.var_gf2__blk1308_dn8 * locals.var_dm__blk1407) + (locals.var_gf2__blk1308 * locals.var_dm__blk1407_dn8)) * assign52840_e68211) - (assign52840_e68206 * (locals.var_xgm__blk1409_dn8 + ((locals.var_gf__blk1307_dn8 * locals.var_sqm__blk1411) + (locals.var_gf__blk1307 * locals.var_sqm__blk1411_dn8))))) / (assign52840_e68211 * assign52840_e68211)))),)
    } else {
        (locals.var_qim__blk1421, locals.var_qim__blk1421_dn5, locals.var_qim__blk1421_dn6, locals.var_qim__blk1421_dn7, locals.var_qim__blk1421_dn8,)
    }
};
        locals.var_qim__blk1421 = assign52840_e68215;
        locals.var_qim__blk1421_dn5 = assign52840_e68215_d_n5;
        locals.var_qim__blk1421_dn6 = assign52840_e68215_d_n6;
        locals.var_qim__blk1421_dn7 = assign52840_e68215_d_n7;
        locals.var_qim__blk1421_dn8 = assign52840_e68215_d_n8;
        locals.var_qim__blk1421_rv = 0.0;

        let (assign52850_e68225, assign52850_e68225_d_n5, assign52850_e68225_d_n6, assign52850_e68225_d_n7, assign52850_e68225_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52850_e68222: f64 = (locals.var_phit1__blk1322 * locals.var_alpha__blk1412);
        let assign52850_e68223: f64 = (locals.var_qim__blk1421 + assign52850_e68222);
        (assign52850_e68223, (locals.var_qim__blk1421_dn5 + ((locals.var_phit1__blk1322_dn5 * locals.var_alpha__blk1412) + (locals.var_phit1__blk1322 * locals.var_alpha__blk1412_dn5))), (locals.var_qim__blk1421_dn6 + ((locals.var_phit1__blk1322_dn6 * locals.var_alpha__blk1412) + (locals.var_phit1__blk1322 * locals.var_alpha__blk1412_dn6))), (locals.var_qim__blk1421_dn7 + ((locals.var_phit1__blk1322_dn7 * locals.var_alpha__blk1412) + (locals.var_phit1__blk1322 * locals.var_alpha__blk1412_dn7))), (locals.var_qim__blk1421_dn8 + ((locals.var_phit1__blk1322_dn8 * locals.var_alpha__blk1412) + (locals.var_phit1__blk1322 * locals.var_alpha__blk1412_dn8))),)
    } else {
        (locals.var_qim1__blk1422, locals.var_qim1__blk1422_dn5, locals.var_qim1__blk1422_dn6, locals.var_qim1__blk1422_dn7, locals.var_qim1__blk1422_dn8,)
    }
};
        locals.var_qim1__blk1422 = assign52850_e68225;
        locals.var_qim1__blk1422_dn5 = assign52850_e68225_d_n5;
        locals.var_qim1__blk1422_dn6 = assign52850_e68225_d_n6;
        locals.var_qim1__blk1422_dn7 = assign52850_e68225_d_n7;
        locals.var_qim1__blk1422_dn8 = assign52850_e68225_d_n8;
        locals.var_qim1__blk1422_rv = 0.0;

        let (assign52860_e68235, assign52860_e68235_d_n5, assign52860_e68235_d_n6, assign52860_e68235_d_n7, assign52860_e68235_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52860_e68231: f64 = (locals.var_sqm__blk1411 * locals.var_gf__blk1307);
        let assign52860_e68233: f64 = (assign52860_e68231 * locals.var_phit1__blk1322);
        (assign52860_e68233, ((((locals.var_sqm__blk1411_dn5 * locals.var_gf__blk1307) + (locals.var_sqm__blk1411 * locals.var_gf__blk1307_dn5)) * locals.var_phit1__blk1322) + (assign52860_e68231 * locals.var_phit1__blk1322_dn5)), ((((locals.var_sqm__blk1411_dn6 * locals.var_gf__blk1307) + (locals.var_sqm__blk1411 * locals.var_gf__blk1307_dn6)) * locals.var_phit1__blk1322) + (assign52860_e68231 * locals.var_phit1__blk1322_dn6)), ((((locals.var_sqm__blk1411_dn7 * locals.var_gf__blk1307) + (locals.var_sqm__blk1411 * locals.var_gf__blk1307_dn7)) * locals.var_phit1__blk1322) + (assign52860_e68231 * locals.var_phit1__blk1322_dn7)), ((((locals.var_sqm__blk1411_dn8 * locals.var_gf__blk1307) + (locals.var_sqm__blk1411 * locals.var_gf__blk1307_dn8)) * locals.var_phit1__blk1322) + (assign52860_e68231 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_qbm__blk1423, locals.var_qbm__blk1423_dn5, locals.var_qbm__blk1423_dn6, locals.var_qbm__blk1423_dn7, locals.var_qbm__blk1423_dn8,)
    }
};
        locals.var_qbm__blk1423 = assign52860_e68235;
        locals.var_qbm__blk1423_dn5 = assign52860_e68235_d_n5;
        locals.var_qbm__blk1423_dn6 = assign52860_e68235_d_n6;
        locals.var_qbm__blk1423_dn7 = assign52860_e68235_d_n7;
        locals.var_qbm__blk1423_dn8 = assign52860_e68235_d_n8;
        locals.var_qbm__blk1423_rv = 0.0;

        let assign52870_e68238: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1503 = assign52870_e68238;
        locals.var_guard1503_rv = 0.0;

        let (assign52880_e68250, assign52880_e68250_d_n5, assign52880_e68250_d_n6, assign52880_e68250_d_n7, assign52880_e68250_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign52880_e68247: f64 = (locals.var_rsg_i * locals.var_qim__blk1421);
        let assign52880_e68248: f64 = (1.0 - assign52880_e68247);
        (assign52880_e68248, (-(locals.var_rsg_i * locals.var_qim__blk1421_dn5)), (-(locals.var_rsg_i * locals.var_qim__blk1421_dn6)), (-(locals.var_rsg_i * locals.var_qim__blk1421_dn7)), (-(locals.var_rsg_i * locals.var_qim__blk1421_dn8)),)
    } else {
        (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8,)
    }
};
        locals.var_rhog__blk1362 = assign52880_e68250;
        locals.var_rhog__blk1362_dn5 = assign52880_e68250_d_n5;
        locals.var_rhog__blk1362_dn6 = assign52880_e68250_d_n6;
        locals.var_rhog__blk1362_dn7 = assign52880_e68250_d_n7;
        locals.var_rhog__blk1362_dn8 = assign52880_e68250_d_n8;
        locals.var_rhog__blk1362_rv = 0.0;

        let (assign52890_e68265, assign52890_e68265_d_n5, assign52890_e68265_d_n6, assign52890_e68265_d_n7, assign52890_e68265_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1503 == 0.0)) {
        let assign52890_e68261: f64 = (locals.var_rsg_i * locals.var_qim__blk1421);
        let assign52890_e68262: f64 = (1.0 + assign52890_e68261);
        let assign52890_e68263: f64 = (1.0 / assign52890_e68262);
        (assign52890_e68263, (-((locals.var_rsg_i * locals.var_qim__blk1421_dn5) / (assign52890_e68262 * assign52890_e68262))), (-((locals.var_rsg_i * locals.var_qim__blk1421_dn6) / (assign52890_e68262 * assign52890_e68262))), (-((locals.var_rsg_i * locals.var_qim__blk1421_dn7) / (assign52890_e68262 * assign52890_e68262))), (-((locals.var_rsg_i * locals.var_qim__blk1421_dn8) / (assign52890_e68262 * assign52890_e68262))),)
    } else {
        (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8,)
    }
};
        locals.var_rhog__blk1362 = assign52890_e68265;
        locals.var_rhog__blk1362_dn5 = assign52890_e68265_d_n5;
        locals.var_rhog__blk1362_dn6 = assign52890_e68265_d_n6;
        locals.var_rhog__blk1362_dn7 = assign52890_e68265_d_n7;
        locals.var_rhog__blk1362_dn8 = assign52890_e68265_d_n8;
        locals.var_rhog__blk1362_rv = 0.0;

        let (assign52900_e68277, assign52900_e68277_d_n5, assign52900_e68277_d_n6, assign52900_e68277_d_n7, assign52900_e68277_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52900_e68271: f64 = (locals.var_ther_i * locals.var_rhob__blk1361);
        let assign52900_e68273: f64 = (assign52900_e68271 * locals.var_rhog__blk1362);
        let assign52900_e68275: f64 = (assign52900_e68273 * locals.var_qim__blk1421);
        (assign52900_e68275, (((((locals.var_ther_i * locals.var_rhob__blk1361_dn5) * locals.var_rhog__blk1362) + (assign52900_e68271 * locals.var_rhog__blk1362_dn5)) * locals.var_qim__blk1421) + (assign52900_e68273 * locals.var_qim__blk1421_dn5)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn6) * locals.var_rhog__blk1362) + (assign52900_e68271 * locals.var_rhog__blk1362_dn6)) * locals.var_qim__blk1421) + (assign52900_e68273 * locals.var_qim__blk1421_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn7) * locals.var_rhog__blk1362) + (assign52900_e68271 * locals.var_rhog__blk1362_dn7)) * locals.var_qim__blk1421) + (assign52900_e68273 * locals.var_qim__blk1421_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn8) * locals.var_rhog__blk1362) + (assign52900_e68271 * locals.var_rhog__blk1362_dn8)) * locals.var_qim__blk1421) + (assign52900_e68273 * locals.var_qim__blk1421_dn8)),)
    } else {
        (locals.var_gr__blk1363, locals.var_gr__blk1363_dn5, locals.var_gr__blk1363_dn6, locals.var_gr__blk1363_dn7, locals.var_gr__blk1363_dn8,)
    }
};
        locals.var_gr__blk1363 = assign52900_e68277;
        locals.var_gr__blk1363_dn5 = assign52900_e68277_d_n5;
        locals.var_gr__blk1363_dn6 = assign52900_e68277_d_n6;
        locals.var_gr__blk1363_dn7 = assign52900_e68277_d_n7;
        locals.var_gr__blk1363_dn8 = assign52900_e68277_d_n8;
        locals.var_gr__blk1363_rv = 0.0;

        let (assign52910_e68287, assign52910_e68287_d_n5, assign52910_e68287_d_n6, assign52910_e68287_d_n7, assign52910_e68287_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52910_e68284: f64 = (locals.var_eta_mu * locals.var_qim__blk1421);
        let assign52910_e68285: f64 = (locals.var_qbm__blk1423 + assign52910_e68284);
        (assign52910_e68285, (locals.var_qbm__blk1423_dn5 + (locals.var_eta_mu * locals.var_qim__blk1421_dn5)), (locals.var_qbm__blk1423_dn6 + (locals.var_eta_mu * locals.var_qim__blk1421_dn6)), (locals.var_qbm__blk1423_dn7 + (locals.var_eta_mu * locals.var_qim__blk1421_dn7)), (locals.var_qbm__blk1423_dn8 + (locals.var_eta_mu * locals.var_qim__blk1421_dn8)),)
    } else {
        (locals.var_qeff__blk1424, locals.var_qeff__blk1424_dn5, locals.var_qeff__blk1424_dn6, locals.var_qeff__blk1424_dn7, locals.var_qeff__blk1424_dn8,)
    }
};
        locals.var_qeff__blk1424 = assign52910_e68287;
        locals.var_qeff__blk1424_dn5 = assign52910_e68287_d_n5;
        locals.var_qeff__blk1424_dn6 = assign52910_e68287_d_n6;
        locals.var_qeff__blk1424_dn7 = assign52910_e68287_d_n7;
        locals.var_qeff__blk1424_dn8 = assign52910_e68287_d_n8;
        locals.var_qeff__blk1424_rv = 0.0;

        let (assign52920_e68297, assign52920_e68297_d_n5, assign52920_e68297_d_n6, assign52920_e68297_d_n7, assign52920_e68297_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52920_e68294: f64 = (locals.var_eta_mu1 * locals.var_qim__blk1421);
        let assign52920_e68295: f64 = (locals.var_qbm__blk1423 + assign52920_e68294);
        (assign52920_e68295, (locals.var_qbm__blk1423_dn5 + (locals.var_eta_mu1 * locals.var_qim__blk1421_dn5)), (locals.var_qbm__blk1423_dn6 + (locals.var_eta_mu1 * locals.var_qim__blk1421_dn6)), (locals.var_qbm__blk1423_dn7 + (locals.var_eta_mu1 * locals.var_qim__blk1421_dn7)), (locals.var_qbm__blk1423_dn8 + (locals.var_eta_mu1 * locals.var_qim__blk1421_dn8)),)
    } else {
        (locals.var_qeff1__blk1425, locals.var_qeff1__blk1425_dn5, locals.var_qeff1__blk1425_dn6, locals.var_qeff1__blk1425_dn7, locals.var_qeff1__blk1425_dn8,)
    }
};
        locals.var_qeff1__blk1425 = assign52920_e68297;
        locals.var_qeff1__blk1425_dn5 = assign52920_e68297_d_n5;
        locals.var_qeff1__blk1425_dn6 = assign52920_e68297_d_n6;
        locals.var_qeff1__blk1425_dn7 = assign52920_e68297_d_n7;
        locals.var_qeff1__blk1425_dn8 = assign52920_e68297_d_n8;
        locals.var_qeff1__blk1425_rv = 0.0;

        let (assign52930_e68305, assign52930_e68305_d_n5, assign52930_e68305_d_n6, assign52930_e68305_d_n7, assign52930_e68305_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52930_e68303: f64 = (locals.var_e_eff0 * locals.var_qeff__blk1424);
        (assign52930_e68303, (locals.var_e_eff0 * locals.var_qeff__blk1424_dn5), (locals.var_e_eff0 * locals.var_qeff__blk1424_dn6), (locals.var_e_eff0 * locals.var_qeff__blk1424_dn7), (locals.var_e_eff0 * locals.var_qeff__blk1424_dn8),)
    } else {
        (locals.var_eeffm__blk1426, locals.var_eeffm__blk1426_dn5, locals.var_eeffm__blk1426_dn6, locals.var_eeffm__blk1426_dn7, locals.var_eeffm__blk1426_dn8,)
    }
};
        locals.var_eeffm__blk1426 = assign52930_e68305;
        locals.var_eeffm__blk1426_dn5 = assign52930_e68305_d_n5;
        locals.var_eeffm__blk1426_dn6 = assign52930_e68305_d_n6;
        locals.var_eeffm__blk1426_dn7 = assign52930_e68305_d_n7;
        locals.var_eeffm__blk1426_dn8 = assign52930_e68305_d_n8;
        locals.var_eeffm__blk1426_rv = 0.0;

        let (assign52940_e68318, assign52940_e68318_d_n5, assign52940_e68318_d_n6, assign52940_e68318_d_n7, assign52940_e68318_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52940_e68312: f64 = (locals.var_pm__blk1408 + locals.var_dm__blk1407);
        let assign52940_e68314: f64 = (assign52940_e68312 + 1e-14);
        let assign52940_e68315: f64 = (locals.var_pm__blk1408 / assign52940_e68314);
        let assign52940_e68316: f64 = (assign52940_e68315).ln();
        (assign52940_e68316, ((((locals.var_pm__blk1408_dn5 * assign52940_e68314) - (locals.var_pm__blk1408 * (locals.var_pm__blk1408_dn5 + locals.var_dm__blk1407_dn5))) / (assign52940_e68314 * assign52940_e68314)) / assign52940_e68315), ((((locals.var_pm__blk1408_dn6 * assign52940_e68314) - (locals.var_pm__blk1408 * (locals.var_pm__blk1408_dn6 + locals.var_dm__blk1407_dn6))) / (assign52940_e68314 * assign52940_e68314)) / assign52940_e68315), ((((locals.var_pm__blk1408_dn7 * assign52940_e68314) - (locals.var_pm__blk1408 * (locals.var_pm__blk1408_dn7 + locals.var_dm__blk1407_dn7))) / (assign52940_e68314 * assign52940_e68314)) / assign52940_e68315), ((((locals.var_pm__blk1408_dn8 * assign52940_e68314) - (locals.var_pm__blk1408 * (locals.var_pm__blk1408_dn8 + locals.var_dm__blk1407_dn8))) / (assign52940_e68314 * assign52940_e68314)) / assign52940_e68315),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign52940_e68318;
        locals.var_temp1_dn5 = assign52940_e68318_d_n5;
        locals.var_temp1_dn6 = assign52940_e68318_d_n6;
        locals.var_temp1_dn7 = assign52940_e68318_d_n7;
        locals.var_temp1_dn8 = assign52940_e68318_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign52950_e68337, assign52950_e68337_d_n5, assign52950_e68337_d_n6, assign52950_e68337_d_n7, assign52950_e68337_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52950_e68324: f64 = (locals.var_eeffm__blk1426 * locals.var_mue_t);
        let assign52950_e68326: f64 = (assign52950_e68324).powf(locals.var_themu_t);
        let assign52950_e68330: f64 = (0.5 * locals.var_thecs_t);
        let assign52950_e68332: f64 = (assign52950_e68330 * locals.var_temp1);
        let assign52950_e68333: f64 = (assign52950_e68332).exp();
        let assign52950_e68334: f64 = (locals.var_cs_t * assign52950_e68333);
        let assign52950_e68335: f64 = (assign52950_e68326 + assign52950_e68334);
        (assign52950_e68335, (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign52950_e68324).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1426_dn5 * locals.var_mue_t))) } } else { (assign52950_e68326 * (locals.var_themu_t * ((locals.var_eeffm__blk1426_dn5 * locals.var_mue_t) / assign52950_e68324))) } + (locals.var_cs_t * (assign52950_e68333 * (assign52950_e68330 * locals.var_temp1_dn5)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign52950_e68324).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1426_dn6 * locals.var_mue_t))) } } else { (assign52950_e68326 * (locals.var_themu_t * ((locals.var_eeffm__blk1426_dn6 * locals.var_mue_t) / assign52950_e68324))) } + (locals.var_cs_t * (assign52950_e68333 * (assign52950_e68330 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign52950_e68324).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1426_dn7 * locals.var_mue_t))) } } else { (assign52950_e68326 * (locals.var_themu_t * ((locals.var_eeffm__blk1426_dn7 * locals.var_mue_t) / assign52950_e68324))) } + (locals.var_cs_t * (assign52950_e68333 * (assign52950_e68330 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign52950_e68324).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1426_dn8 * locals.var_mue_t))) } } else { (assign52950_e68326 * (locals.var_themu_t * ((locals.var_eeffm__blk1426_dn8 * locals.var_mue_t) / assign52950_e68324))) } + (locals.var_cs_t * (assign52950_e68333 * (assign52950_e68330 * locals.var_temp1_dn8)))),)
    } else {
        (locals.var_mutmp__blk1365, locals.var_mutmp__blk1365_dn5, locals.var_mutmp__blk1365_dn6, locals.var_mutmp__blk1365_dn7, locals.var_mutmp__blk1365_dn8,)
    }
};
        locals.var_mutmp__blk1365 = assign52950_e68337;
        locals.var_mutmp__blk1365_dn5 = assign52950_e68337_d_n5;
        locals.var_mutmp__blk1365_dn6 = assign52950_e68337_d_n6;
        locals.var_mutmp__blk1365_dn7 = assign52950_e68337_d_n7;
        locals.var_mutmp__blk1365_dn8 = assign52950_e68337_d_n8;
        locals.var_mutmp__blk1365_rv = 0.0;

        let (assign52960_e68349, assign52960_e68349_d_n5, assign52960_e68349_d_n6, assign52960_e68349_d_n7, assign52960_e68349_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52960_e68343: f64 = (1.0 + locals.var_mutmp__blk1365);
        let assign52960_e68345: f64 = (assign52960_e68343 + locals.var_gr__blk1363);
        let assign52960_e68347: f64 = (assign52960_e68345 * locals.var_rxcor__blk1357);
        (assign52960_e68347, (((locals.var_mutmp__blk1365_dn5 + locals.var_gr__blk1363_dn5) * locals.var_rxcor__blk1357) + (assign52960_e68345 * locals.var_rxcor__blk1357_dn5)), (((locals.var_mutmp__blk1365_dn6 + locals.var_gr__blk1363_dn6) * locals.var_rxcor__blk1357) + (assign52960_e68345 * locals.var_rxcor__blk1357_dn6)), (((locals.var_mutmp__blk1365_dn7 + locals.var_gr__blk1363_dn7) * locals.var_rxcor__blk1357) + (assign52960_e68345 * locals.var_rxcor__blk1357_dn7)), (((locals.var_mutmp__blk1365_dn8 + locals.var_gr__blk1363_dn8) * locals.var_rxcor__blk1357) + (assign52960_e68345 * locals.var_rxcor__blk1357_dn8)),)
    } else {
        (locals.var_gmob__blk1427, locals.var_gmob__blk1427_dn5, locals.var_gmob__blk1427_dn6, locals.var_gmob__blk1427_dn7, locals.var_gmob__blk1427_dn8,)
    }
};
        locals.var_gmob__blk1427 = assign52960_e68349;
        locals.var_gmob__blk1427_dn5 = assign52960_e68349_d_n5;
        locals.var_gmob__blk1427_dn6 = assign52960_e68349_d_n6;
        locals.var_gmob__blk1427_dn7 = assign52960_e68349_d_n7;
        locals.var_gmob__blk1427_dn8 = assign52960_e68349_d_n8;
        locals.var_gmob__blk1427_rv = 0.0;

        let (assign52970_e68370, assign52970_e68370_d_n5, assign52970_e68370_d_n6, assign52970_e68370_d_n7, assign52970_e68370_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52970_e68356: f64 = (locals.var_v_ds - locals.var_dps__blk1397);
        let assign52970_e68358: f64 = (assign52970_e68356 * locals.var_inv_vp);
        let assign52970_e68359: f64 = (1.0 + assign52970_e68358);
        let assign52970_e68363: f64 = (locals.var_vdse__blk1388 - locals.var_dps__blk1397);
        let assign52970_e68365: f64 = (assign52970_e68363 * locals.var_inv_vp);
        let assign52970_e68366: f64 = (1.0 + assign52970_e68365);
        let assign52970_e68367: f64 = (assign52970_e68359 / assign52970_e68366);
        let assign52970_e68368: f64 = (assign52970_e68367).ln();
        (assign52970_e68368, ((((((-locals.var_dps__blk1397_dn5) * locals.var_inv_vp) * assign52970_e68366) - (assign52970_e68359 * ((locals.var_vdse__blk1388_dn5 - locals.var_dps__blk1397_dn5) * locals.var_inv_vp))) / (assign52970_e68366 * assign52970_e68366)) / assign52970_e68367), ((((((locals.var_v_ds_dn6 - locals.var_dps__blk1397_dn6) * locals.var_inv_vp) * assign52970_e68366) - (assign52970_e68359 * ((locals.var_vdse__blk1388_dn6 - locals.var_dps__blk1397_dn6) * locals.var_inv_vp))) / (assign52970_e68366 * assign52970_e68366)) / assign52970_e68367), ((((((locals.var_v_ds_dn7 - locals.var_dps__blk1397_dn7) * locals.var_inv_vp) * assign52970_e68366) - (assign52970_e68359 * ((locals.var_vdse__blk1388_dn7 - locals.var_dps__blk1397_dn7) * locals.var_inv_vp))) / (assign52970_e68366 * assign52970_e68366)) / assign52970_e68367), ((((((-locals.var_dps__blk1397_dn8) * locals.var_inv_vp) * assign52970_e68366) - (assign52970_e68359 * ((locals.var_vdse__blk1388_dn8 - locals.var_dps__blk1397_dn8) * locals.var_inv_vp))) / (assign52970_e68366 * assign52970_e68366)) / assign52970_e68367),)
    } else {
        (locals.var_s1__blk1428, locals.var_s1__blk1428_dn5, locals.var_s1__blk1428_dn6, locals.var_s1__blk1428_dn7, locals.var_s1__blk1428_dn8,)
    }
};
        locals.var_s1__blk1428 = assign52970_e68370;
        locals.var_s1__blk1428_dn5 = assign52970_e68370_d_n5;
        locals.var_s1__blk1428_dn6 = assign52970_e68370_d_n6;
        locals.var_s1__blk1428_dn7 = assign52970_e68370_d_n7;
        locals.var_s1__blk1428_dn8 = assign52970_e68370_d_n8;
        locals.var_s1__blk1428_rv = 0.0;

        let (assign52980_e68378, assign52980_e68378_d_n5, assign52980_e68378_d_n6, assign52980_e68378_d_n7, assign52980_e68378_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52980_e68376: f64 = (locals.var_qim__blk1421 * locals.var_xitsb__blk1367);
        (assign52980_e68376, ((locals.var_qim__blk1421_dn5 * locals.var_xitsb__blk1367) + (locals.var_qim__blk1421 * locals.var_xitsb__blk1367_dn5)), ((locals.var_qim__blk1421_dn6 * locals.var_xitsb__blk1367) + (locals.var_qim__blk1421 * locals.var_xitsb__blk1367_dn6)), ((locals.var_qim__blk1421_dn7 * locals.var_xitsb__blk1367) + (locals.var_qim__blk1421 * locals.var_xitsb__blk1367_dn7)), ((locals.var_qim__blk1421_dn8 * locals.var_xitsb__blk1367) + (locals.var_qim__blk1421 * locals.var_xitsb__blk1367_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign52980_e68378;
        locals.var_temp2_dn5 = assign52980_e68378_d_n5;
        locals.var_temp2_dn6 = assign52980_e68378_d_n6;
        locals.var_temp2_dn7 = assign52980_e68378_d_n7;
        locals.var_temp2_dn8 = assign52980_e68378_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign52990_e68388, assign52990_e68388_d_n5, assign52990_e68388_d_n6, assign52990_e68388_d_n7, assign52990_e68388_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52990_e68385: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign52990_e68386: f64 = (locals.var_temp2 / assign52990_e68385);
        (assign52990_e68386, (((locals.var_temp2_dn5 * assign52990_e68385) - (locals.var_temp2 * locals.var_temp2_dn5)) / (assign52990_e68385 * assign52990_e68385)), (((locals.var_temp2_dn6 * assign52990_e68385) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign52990_e68385 * assign52990_e68385)), (((locals.var_temp2_dn7 * assign52990_e68385) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign52990_e68385 * assign52990_e68385)), (((locals.var_temp2_dn8 * assign52990_e68385) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign52990_e68385 * assign52990_e68385)),)
    } else {
        (locals.var_wsat__blk1368, locals.var_wsat__blk1368_dn5, locals.var_wsat__blk1368_dn6, locals.var_wsat__blk1368_dn7, locals.var_wsat__blk1368_dn8,)
    }
};
        locals.var_wsat__blk1368 = assign52990_e68388;
        locals.var_wsat__blk1368_dn5 = assign52990_e68388_d_n5;
        locals.var_wsat__blk1368_dn6 = assign52990_e68388_d_n6;
        locals.var_wsat__blk1368_dn7 = assign52990_e68388_d_n7;
        locals.var_wsat__blk1368_dn8 = assign52990_e68388_d_n8;
        locals.var_wsat__blk1368_rv = 0.0;

        let assign53000_e68391: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1504 = assign53000_e68391;
        locals.var_guard1504_rv = 0.0;

        let (assign53010_e68405, assign53010_e68405_d_n5, assign53010_e68405_d_n6, assign53010_e68405_d_n7, assign53010_e68405_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign53010_e68401: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1368);
        let assign53010_e68402: f64 = (1.0 - assign53010_e68401);
        let assign53010_e68403: f64 = (1.0 / assign53010_e68402);
        (assign53010_e68403, (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn5)) / (assign53010_e68402 * assign53010_e68402))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn6)) / (assign53010_e68402 * assign53010_e68402))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn7)) / (assign53010_e68402 * assign53010_e68402))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn8)) / (assign53010_e68402 * assign53010_e68402))),)
    } else {
        (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8,)
    }
};
        locals.var_factheta__blk1369 = assign53010_e68405;
        locals.var_factheta__blk1369_dn5 = assign53010_e68405_d_n5;
        locals.var_factheta__blk1369_dn6 = assign53010_e68405_d_n6;
        locals.var_factheta__blk1369_dn7 = assign53010_e68405_d_n7;
        locals.var_factheta__blk1369_dn8 = assign53010_e68405_d_n8;
        locals.var_factheta__blk1369_rv = 0.0;

        let (assign53020_e68418, assign53020_e68418_d_n5, assign53020_e68418_d_n6, assign53020_e68418_d_n7, assign53020_e68418_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1504 == 0.0)) {
        let assign53020_e68415: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1368);
        let assign53020_e68416: f64 = (1.0 + assign53020_e68415);
        (assign53020_e68416, (locals.var_thesatg_i * locals.var_wsat__blk1368_dn5), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn6), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn7), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn8),)
    } else {
        (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8,)
    }
};
        locals.var_factheta__blk1369 = assign53020_e68418;
        locals.var_factheta__blk1369_dn5 = assign53020_e68418_d_n5;
        locals.var_factheta__blk1369_dn6 = assign53020_e68418_d_n6;
        locals.var_factheta__blk1369_dn7 = assign53020_e68418_d_n7;
        locals.var_factheta__blk1369_dn8 = assign53020_e68418_d_n8;
        locals.var_factheta__blk1369_rv = 0.0;

        let (assign53030_e68426, assign53030_e68426_d_n5, assign53030_e68426_d_n6, assign53030_e68426_d_n7, assign53030_e68426_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign53030_e68424: f64 = (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369);
        (assign53030_e68424, (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn5), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn6), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn7), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn8),)
    } else {
        (locals.var_thesateff__blk1430, locals.var_thesateff__blk1430_dn5, locals.var_thesateff__blk1430_dn6, locals.var_thesateff__blk1430_dn7, locals.var_thesateff__blk1430_dn8,)
    }
};
        locals.var_thesateff__blk1430 = assign53030_e68426;
        locals.var_thesateff__blk1430_dn5 = assign53030_e68426_d_n5;
        locals.var_thesateff__blk1430_dn6 = assign53030_e68426_d_n6;
        locals.var_thesateff__blk1430_dn7 = assign53030_e68426_d_n7;
        locals.var_thesateff__blk1430_dn8 = assign53030_e68426_d_n8;
        locals.var_thesateff__blk1430_rv = 0.0;

        let (assign53040_e68434, assign53040_e68434_d_n5, assign53040_e68434_d_n6, assign53040_e68434_d_n7, assign53040_e68434_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign53040_e68432: f64 = (locals.var_xgm__blk1409 * locals.var_phit1__blk1322);
        (assign53040_e68432, ((locals.var_xgm__blk1409_dn5 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn5)), ((locals.var_xgm__blk1409_dn6 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn6)), ((locals.var_xgm__blk1409_dn7 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn7)), ((locals.var_xgm__blk1409_dn8 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_voxm__blk1429, locals.var_voxm__blk1429_dn5, locals.var_voxm__blk1429_dn6, locals.var_voxm__blk1429_dn7, locals.var_voxm__blk1429_dn8,)
    }
};
        locals.var_voxm__blk1429 = assign53040_e68434;
        locals.var_voxm__blk1429_dn5 = assign53040_e68434_d_n5;
        locals.var_voxm__blk1429_dn6 = assign53040_e68434_d_n6;
        locals.var_voxm__blk1429_dn7 = assign53040_e68434_d_n7;
        locals.var_voxm__blk1429_dn8 = assign53040_e68434_d_n8;
        locals.var_voxm__blk1429_rv = 0.0;

        let (assign53050_e68438, assign53050_e68438_d_n5, assign53050_e68438_d_n6, assign53050_e68438_d_n7, assign53050_e68438_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_vgb1__blk1304, locals.var_vgb1__blk1304_dn5, locals.var_vgb1__blk1304_dn6, locals.var_vgb1__blk1304_dn7, locals.var_vgb1__blk1304_dn8,)
    } else {
        (locals.var_vgb1_ac, locals.var_vgb1_ac_dn5, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8,)
    }
};
        locals.var_vgb1_ac = assign53050_e68438;
        locals.var_vgb1_ac_dn5 = assign53050_e68438_d_n5;
        locals.var_vgb1_ac_dn6 = assign53050_e68438_d_n6;
        locals.var_vgb1_ac_dn7 = assign53050_e68438_d_n7;
        locals.var_vgb1_ac_dn8 = assign53050_e68438_d_n8;
        locals.var_vgb1_ac_rv = 0.0;

        let (assign53060_e68442, assign53060_e68442_d_n5, assign53060_e68442_d_n6, assign53060_e68442_d_n7, assign53060_e68442_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_phit1__blk1322, locals.var_phit1__blk1322_dn5, locals.var_phit1__blk1322_dn6, locals.var_phit1__blk1322_dn7, locals.var_phit1__blk1322_dn8,)
    } else {
        (locals.var_phit1_ac, locals.var_phit1_ac_dn5, locals.var_phit1_ac_dn6, locals.var_phit1_ac_dn7, locals.var_phit1_ac_dn8,)
    }
};
        locals.var_phit1_ac = assign53060_e68442;
        locals.var_phit1_ac_dn5 = assign53060_e68442_d_n5;
        locals.var_phit1_ac_dn6 = assign53060_e68442_d_n6;
        locals.var_phit1_ac_dn7 = assign53060_e68442_d_n7;
        locals.var_phit1_ac_dn8 = assign53060_e68442_d_n8;
        locals.var_phit1_ac_rv = 0.0;

        let (assign53070_e68446, assign53070_e68446_d_n5, assign53070_e68446_d_n6, assign53070_e68446_d_n7, assign53070_e68446_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_gf__blk1307, locals.var_gf__blk1307_dn5, locals.var_gf__blk1307_dn6, locals.var_gf__blk1307_dn7, locals.var_gf__blk1307_dn8,)
    } else {
        (locals.var_gf_ac, locals.var_gf_ac_dn5, locals.var_gf_ac_dn6, locals.var_gf_ac_dn7, locals.var_gf_ac_dn8,)
    }
};
        locals.var_gf_ac = assign53070_e68446;
        locals.var_gf_ac_dn5 = assign53070_e68446_d_n5;
        locals.var_gf_ac_dn6 = assign53070_e68446_d_n6;
        locals.var_gf_ac_dn7 = assign53070_e68446_d_n7;
        locals.var_gf_ac_dn8 = assign53070_e68446_d_n8;
        locals.var_gf_ac_rv = 0.0;

        let (assign53080_e68450, assign53080_e68450_d_n5, assign53080_e68450_d_n6, assign53080_e68450_d_n7, assign53080_e68450_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_xg__blk1326, locals.var_xg__blk1326_dn5, locals.var_xg__blk1326_dn6, locals.var_xg__blk1326_dn7, locals.var_xg__blk1326_dn8,)
    } else {
        (locals.var_xg_ac, locals.var_xg_ac_dn5, locals.var_xg_ac_dn6, locals.var_xg_ac_dn7, locals.var_xg_ac_dn8,)
    }
};
        locals.var_xg_ac = assign53080_e68450;
        locals.var_xg_ac_dn5 = assign53080_e68450_d_n5;
        locals.var_xg_ac_dn6 = assign53080_e68450_d_n6;
        locals.var_xg_ac_dn7 = assign53080_e68450_d_n7;
        locals.var_xg_ac_dn8 = assign53080_e68450_d_n8;
        locals.var_xg_ac_rv = 0.0;

        let (assign53090_e68454, assign53090_e68454_d_n5, assign53090_e68454_d_n6, assign53090_e68454_d_n7, assign53090_e68454_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_xno_s__blk1331, locals.var_xno_s__blk1331_dn5, locals.var_xno_s__blk1331_dn6, locals.var_xno_s__blk1331_dn7, locals.var_xno_s__blk1331_dn8,)
    } else {
        (locals.var_xno_s_ac, locals.var_xno_s_ac_dn5, locals.var_xno_s_ac_dn6, locals.var_xno_s_ac_dn7, locals.var_xno_s_ac_dn8,)
    }
};
        locals.var_xno_s_ac = assign53090_e68454;
        locals.var_xno_s_ac_dn5 = assign53090_e68454_d_n5;
        locals.var_xno_s_ac_dn6 = assign53090_e68454_d_n6;
        locals.var_xno_s_ac_dn7 = assign53090_e68454_d_n7;
        locals.var_xno_s_ac_dn8 = assign53090_e68454_d_n8;
        locals.var_xno_s_ac_rv = 0.0;

        let (assign53100_e68458, assign53100_e68458_d_n5, assign53100_e68458_d_n6, assign53100_e68458_d_n7, assign53100_e68458_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8,)
    } else {
        (locals.var_qbs_ac, locals.var_qbs_ac_dn5, locals.var_qbs_ac_dn6, locals.var_qbs_ac_dn7, locals.var_qbs_ac_dn8,)
    }
};
        locals.var_qbs_ac = assign53100_e68458;
        locals.var_qbs_ac_dn5 = assign53100_e68458_d_n5;
        locals.var_qbs_ac_dn6 = assign53100_e68458_d_n6;
        locals.var_qbs_ac_dn7 = assign53100_e68458_d_n7;
        locals.var_qbs_ac_dn8 = assign53100_e68458_d_n8;
        locals.var_qbs_ac_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_50(
        locals: &mut StampLocals,
    ) {
        let (assign53110_e68462, assign53110_e68462_d_n5, assign53110_e68462_d_n6, assign53110_e68462_d_n7, assign53110_e68462_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_dps__blk1397, locals.var_dps__blk1397_dn5, locals.var_dps__blk1397_dn6, locals.var_dps__blk1397_dn7, locals.var_dps__blk1397_dn8,)
    } else {
        (locals.var_dps_ac, locals.var_dps_ac_dn5, locals.var_dps_ac_dn6, locals.var_dps_ac_dn7, locals.var_dps_ac_dn8,)
    }
};
        locals.var_dps_ac = assign53110_e68462;
        locals.var_dps_ac_dn5 = assign53110_e68462_d_n5;
        locals.var_dps_ac_dn6 = assign53110_e68462_d_n6;
        locals.var_dps_ac_dn7 = assign53110_e68462_d_n7;
        locals.var_dps_ac_dn8 = assign53110_e68462_d_n8;
        locals.var_dps_ac_rv = 0.0;

        let (assign53120_e68466, assign53120_e68466_d_n5, assign53120_e68466_d_n6, assign53120_e68466_d_n7, assign53120_e68466_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qbd__blk1403, locals.var_qbd__blk1403_dn5, locals.var_qbd__blk1403_dn6, locals.var_qbd__blk1403_dn7, locals.var_qbd__blk1403_dn8,)
    } else {
        (locals.var_qbd_ac, locals.var_qbd_ac_dn5, locals.var_qbd_ac_dn6, locals.var_qbd_ac_dn7, locals.var_qbd_ac_dn8,)
    }
};
        locals.var_qbd_ac = assign53120_e68466;
        locals.var_qbd_ac_dn5 = assign53120_e68466_d_n5;
        locals.var_qbd_ac_dn6 = assign53120_e68466_d_n6;
        locals.var_qbd_ac_dn7 = assign53120_e68466_d_n7;
        locals.var_qbd_ac_dn8 = assign53120_e68466_d_n8;
        locals.var_qbd_ac_rv = 0.0;

        let (assign53130_e68470, assign53130_e68470_d_n5, assign53130_e68470_d_n6, assign53130_e68470_d_n7, assign53130_e68470_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_eta_p__blk1410, locals.var_eta_p__blk1410_dn5, locals.var_eta_p__blk1410_dn6, locals.var_eta_p__blk1410_dn7, locals.var_eta_p__blk1410_dn8,)
    } else {
        (locals.var_eta_p_ac, locals.var_eta_p_ac_dn5, locals.var_eta_p_ac_dn6, locals.var_eta_p_ac_dn7, locals.var_eta_p_ac_dn8,)
    }
};
        locals.var_eta_p_ac = assign53130_e68470;
        locals.var_eta_p_ac_dn5 = assign53130_e68470_d_n5;
        locals.var_eta_p_ac_dn6 = assign53130_e68470_d_n6;
        locals.var_eta_p_ac_dn7 = assign53130_e68470_d_n7;
        locals.var_eta_p_ac_dn8 = assign53130_e68470_d_n8;
        locals.var_eta_p_ac_rv = 0.0;

        let (assign53140_e68474, assign53140_e68474_d_n5, assign53140_e68474_d_n6, assign53140_e68474_d_n7, assign53140_e68474_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_alpha__blk1412, locals.var_alpha__blk1412_dn5, locals.var_alpha__blk1412_dn6, locals.var_alpha__blk1412_dn7, locals.var_alpha__blk1412_dn8,)
    } else {
        (locals.var_alpha_ac, locals.var_alpha_ac_dn5, locals.var_alpha_ac_dn6, locals.var_alpha_ac_dn7, locals.var_alpha_ac_dn8,)
    }
};
        locals.var_alpha_ac = assign53140_e68474;
        locals.var_alpha_ac_dn5 = assign53140_e68474_d_n5;
        locals.var_alpha_ac_dn6 = assign53140_e68474_d_n6;
        locals.var_alpha_ac_dn7 = assign53140_e68474_d_n7;
        locals.var_alpha_ac_dn8 = assign53140_e68474_d_n8;
        locals.var_alpha_ac_rv = 0.0;

        let (assign53150_e68478, assign53150_e68478_d_n5, assign53150_e68478_d_n6, assign53150_e68478_d_n7, assign53150_e68478_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qim__blk1421, locals.var_qim__blk1421_dn5, locals.var_qim__blk1421_dn6, locals.var_qim__blk1421_dn7, locals.var_qim__blk1421_dn8,)
    } else {
        (locals.var_qim_ac, locals.var_qim_ac_dn5, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8,)
    }
};
        locals.var_qim_ac = assign53150_e68478;
        locals.var_qim_ac_dn5 = assign53150_e68478_d_n5;
        locals.var_qim_ac_dn6 = assign53150_e68478_d_n6;
        locals.var_qim_ac_dn7 = assign53150_e68478_d_n7;
        locals.var_qim_ac_dn8 = assign53150_e68478_d_n8;
        locals.var_qim_ac_rv = 0.0;

        let (assign53160_e68482, assign53160_e68482_d_n5, assign53160_e68482_d_n6, assign53160_e68482_d_n7, assign53160_e68482_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qim1__blk1422, locals.var_qim1__blk1422_dn5, locals.var_qim1__blk1422_dn6, locals.var_qim1__blk1422_dn7, locals.var_qim1__blk1422_dn8,)
    } else {
        (locals.var_qim1_ac, locals.var_qim1_ac_dn5, locals.var_qim1_ac_dn6, locals.var_qim1_ac_dn7, locals.var_qim1_ac_dn8,)
    }
};
        locals.var_qim1_ac = assign53160_e68482;
        locals.var_qim1_ac_dn5 = assign53160_e68482_d_n5;
        locals.var_qim1_ac_dn6 = assign53160_e68482_d_n6;
        locals.var_qim1_ac_dn7 = assign53160_e68482_d_n7;
        locals.var_qim1_ac_dn8 = assign53160_e68482_d_n8;
        locals.var_qim1_ac_rv = 0.0;

        let (assign53170_e68486, assign53170_e68486_d_n5, assign53170_e68486_d_n6, assign53170_e68486_d_n7, assign53170_e68486_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qeff1__blk1425, locals.var_qeff1__blk1425_dn5, locals.var_qeff1__blk1425_dn6, locals.var_qeff1__blk1425_dn7, locals.var_qeff1__blk1425_dn8,)
    } else {
        (locals.var_qeff1_ac, locals.var_qeff1_ac_dn5, locals.var_qeff1_ac_dn6, locals.var_qeff1_ac_dn7, locals.var_qeff1_ac_dn8,)
    }
};
        locals.var_qeff1_ac = assign53170_e68486;
        locals.var_qeff1_ac_dn5 = assign53170_e68486_d_n5;
        locals.var_qeff1_ac_dn6 = assign53170_e68486_d_n6;
        locals.var_qeff1_ac_dn7 = assign53170_e68486_d_n7;
        locals.var_qeff1_ac_dn8 = assign53170_e68486_d_n8;
        locals.var_qeff1_ac_rv = 0.0;

        let (assign53180_e68490, assign53180_e68490_d_n5, assign53180_e68490_d_n6, assign53180_e68490_d_n7, assign53180_e68490_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_gmob__blk1427, locals.var_gmob__blk1427_dn5, locals.var_gmob__blk1427_dn6, locals.var_gmob__blk1427_dn7, locals.var_gmob__blk1427_dn8,)
    } else {
        (locals.var_gmob_ac, locals.var_gmob_ac_dn5, locals.var_gmob_ac_dn6, locals.var_gmob_ac_dn7, locals.var_gmob_ac_dn8,)
    }
};
        locals.var_gmob_ac = assign53180_e68490;
        locals.var_gmob_ac_dn5 = assign53180_e68490_d_n5;
        locals.var_gmob_ac_dn6 = assign53180_e68490_d_n6;
        locals.var_gmob_ac_dn7 = assign53180_e68490_d_n7;
        locals.var_gmob_ac_dn8 = assign53180_e68490_d_n8;
        locals.var_gmob_ac_rv = 0.0;

        let (assign53190_e68494, assign53190_e68494_d_n5, assign53190_e68494_d_n6, assign53190_e68494_d_n7, assign53190_e68494_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_s1__blk1428, locals.var_s1__blk1428_dn5, locals.var_s1__blk1428_dn6, locals.var_s1__blk1428_dn7, locals.var_s1__blk1428_dn8,)
    } else {
        (locals.var_s1_ac, locals.var_s1_ac_dn5, locals.var_s1_ac_dn6, locals.var_s1_ac_dn7, locals.var_s1_ac_dn8,)
    }
};
        locals.var_s1_ac = assign53190_e68494;
        locals.var_s1_ac_dn5 = assign53190_e68494_d_n5;
        locals.var_s1_ac_dn6 = assign53190_e68494_d_n6;
        locals.var_s1_ac_dn7 = assign53190_e68494_d_n7;
        locals.var_s1_ac_dn8 = assign53190_e68494_d_n8;
        locals.var_s1_ac_rv = 0.0;

        let (assign53200_e68498, assign53200_e68498_d_n5, assign53200_e68498_d_n6, assign53200_e68498_d_n7, assign53200_e68498_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_thesateff__blk1430, locals.var_thesateff__blk1430_dn5, locals.var_thesateff__blk1430_dn6, locals.var_thesateff__blk1430_dn7, locals.var_thesateff__blk1430_dn8,)
    } else {
        (locals.var_thesateff_ac, locals.var_thesateff_ac_dn5, locals.var_thesateff_ac_dn6, locals.var_thesateff_ac_dn7, locals.var_thesateff_ac_dn8,)
    }
};
        locals.var_thesateff_ac = assign53200_e68498;
        locals.var_thesateff_ac_dn5 = assign53200_e68498_d_n5;
        locals.var_thesateff_ac_dn6 = assign53200_e68498_d_n6;
        locals.var_thesateff_ac_dn7 = assign53200_e68498_d_n7;
        locals.var_thesateff_ac_dn8 = assign53200_e68498_d_n8;
        locals.var_thesateff_ac_rv = 0.0;

        let (assign53210_e68502, assign53210_e68502_d_n5, assign53210_e68502_d_n6, assign53210_e68502_d_n7, assign53210_e68502_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_voxm__blk1429, locals.var_voxm__blk1429_dn5, locals.var_voxm__blk1429_dn6, locals.var_voxm__blk1429_dn7, locals.var_voxm__blk1429_dn8,)
    } else {
        (locals.var_voxm_ac, locals.var_voxm_ac_dn5, locals.var_voxm_ac_dn6, locals.var_voxm_ac_dn7, locals.var_voxm_ac_dn8,)
    }
};
        locals.var_voxm_ac = assign53210_e68502;
        locals.var_voxm_ac_dn5 = assign53210_e68502_d_n5;
        locals.var_voxm_ac_dn6 = assign53210_e68502_d_n6;
        locals.var_voxm_ac_dn7 = assign53210_e68502_d_n7;
        locals.var_voxm_ac_dn8 = assign53210_e68502_d_n8;
        locals.var_voxm_ac_rv = 0.0;

        let (assign53220_e68507,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_phib_dc,)
    } else {
        (locals.var_phib_ac,)
    }
};
        locals.var_phib_ac = assign53220_e68507;
        locals.var_phib_ac_rv = 0.0;

        let (assign53230_e68512, assign53230_e68512_d_n5, assign53230_e68512_d_n6, assign53230_e68512_d_n7, assign53230_e68512_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_vgb1_dc, locals.var_vgb1_dc_dn5, locals.var_vgb1_dc_dn6, locals.var_vgb1_dc_dn7, locals.var_vgb1_dc_dn8,)
    } else {
        (locals.var_vgb1_ac, locals.var_vgb1_ac_dn5, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8,)
    }
};
        locals.var_vgb1_ac = assign53230_e68512;
        locals.var_vgb1_ac_dn5 = assign53230_e68512_d_n5;
        locals.var_vgb1_ac_dn6 = assign53230_e68512_d_n6;
        locals.var_vgb1_ac_dn7 = assign53230_e68512_d_n7;
        locals.var_vgb1_ac_dn8 = assign53230_e68512_d_n8;
        locals.var_vgb1_ac_rv = 0.0;

        let (assign53240_e68517, assign53240_e68517_d_n5, assign53240_e68517_d_n6, assign53240_e68517_d_n7, assign53240_e68517_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_phit1_dc, locals.var_phit1_dc_dn5, locals.var_phit1_dc_dn6, locals.var_phit1_dc_dn7, locals.var_phit1_dc_dn8,)
    } else {
        (locals.var_phit1_ac, locals.var_phit1_ac_dn5, locals.var_phit1_ac_dn6, locals.var_phit1_ac_dn7, locals.var_phit1_ac_dn8,)
    }
};
        locals.var_phit1_ac = assign53240_e68517;
        locals.var_phit1_ac_dn5 = assign53240_e68517_d_n5;
        locals.var_phit1_ac_dn6 = assign53240_e68517_d_n6;
        locals.var_phit1_ac_dn7 = assign53240_e68517_d_n7;
        locals.var_phit1_ac_dn8 = assign53240_e68517_d_n8;
        locals.var_phit1_ac_rv = 0.0;

        let (assign53250_e68522, assign53250_e68522_d_n5, assign53250_e68522_d_n6, assign53250_e68522_d_n7, assign53250_e68522_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_gf_dc, locals.var_gf_dc_dn5, locals.var_gf_dc_dn6, locals.var_gf_dc_dn7, locals.var_gf_dc_dn8,)
    } else {
        (locals.var_gf_ac, locals.var_gf_ac_dn5, locals.var_gf_ac_dn6, locals.var_gf_ac_dn7, locals.var_gf_ac_dn8,)
    }
};
        locals.var_gf_ac = assign53250_e68522;
        locals.var_gf_ac_dn5 = assign53250_e68522_d_n5;
        locals.var_gf_ac_dn6 = assign53250_e68522_d_n6;
        locals.var_gf_ac_dn7 = assign53250_e68522_d_n7;
        locals.var_gf_ac_dn8 = assign53250_e68522_d_n8;
        locals.var_gf_ac_rv = 0.0;

        let (assign53260_e68527, assign53260_e68527_d_n5, assign53260_e68527_d_n6, assign53260_e68527_d_n7, assign53260_e68527_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_xg_dc, locals.var_xg_dc_dn5, locals.var_xg_dc_dn6, locals.var_xg_dc_dn7, locals.var_xg_dc_dn8,)
    } else {
        (locals.var_xg_ac, locals.var_xg_ac_dn5, locals.var_xg_ac_dn6, locals.var_xg_ac_dn7, locals.var_xg_ac_dn8,)
    }
};
        locals.var_xg_ac = assign53260_e68527;
        locals.var_xg_ac_dn5 = assign53260_e68527_d_n5;
        locals.var_xg_ac_dn6 = assign53260_e68527_d_n6;
        locals.var_xg_ac_dn7 = assign53260_e68527_d_n7;
        locals.var_xg_ac_dn8 = assign53260_e68527_d_n8;
        locals.var_xg_ac_rv = 0.0;

        let (assign53270_e68532, assign53270_e68532_d_n5, assign53270_e68532_d_n6, assign53270_e68532_d_n7, assign53270_e68532_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_xno_s_dc, locals.var_xno_s_dc_dn5, locals.var_xno_s_dc_dn6, locals.var_xno_s_dc_dn7, locals.var_xno_s_dc_dn8,)
    } else {
        (locals.var_xno_s_ac, locals.var_xno_s_ac_dn5, locals.var_xno_s_ac_dn6, locals.var_xno_s_ac_dn7, locals.var_xno_s_ac_dn8,)
    }
};
        locals.var_xno_s_ac = assign53270_e68532;
        locals.var_xno_s_ac_dn5 = assign53270_e68532_d_n5;
        locals.var_xno_s_ac_dn6 = assign53270_e68532_d_n6;
        locals.var_xno_s_ac_dn7 = assign53270_e68532_d_n7;
        locals.var_xno_s_ac_dn8 = assign53270_e68532_d_n8;
        locals.var_xno_s_ac_rv = 0.0;

        let (assign53280_e68537, assign53280_e68537_d_n5, assign53280_e68537_d_n6, assign53280_e68537_d_n7, assign53280_e68537_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_qbs_dc, locals.var_qbs_dc_dn5, locals.var_qbs_dc_dn6, locals.var_qbs_dc_dn7, locals.var_qbs_dc_dn8,)
    } else {
        (locals.var_qbs_ac, locals.var_qbs_ac_dn5, locals.var_qbs_ac_dn6, locals.var_qbs_ac_dn7, locals.var_qbs_ac_dn8,)
    }
};
        locals.var_qbs_ac = assign53280_e68537;
        locals.var_qbs_ac_dn5 = assign53280_e68537_d_n5;
        locals.var_qbs_ac_dn6 = assign53280_e68537_d_n6;
        locals.var_qbs_ac_dn7 = assign53280_e68537_d_n7;
        locals.var_qbs_ac_dn8 = assign53280_e68537_d_n8;
        locals.var_qbs_ac_rv = 0.0;

        let (assign53290_e68542, assign53290_e68542_d_n5, assign53290_e68542_d_n6, assign53290_e68542_d_n7, assign53290_e68542_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_dps_dc, locals.var_dps_dc_dn5, locals.var_dps_dc_dn6, locals.var_dps_dc_dn7, locals.var_dps_dc_dn8,)
    } else {
        (locals.var_dps_ac, locals.var_dps_ac_dn5, locals.var_dps_ac_dn6, locals.var_dps_ac_dn7, locals.var_dps_ac_dn8,)
    }
};
        locals.var_dps_ac = assign53290_e68542;
        locals.var_dps_ac_dn5 = assign53290_e68542_d_n5;
        locals.var_dps_ac_dn6 = assign53290_e68542_d_n6;
        locals.var_dps_ac_dn7 = assign53290_e68542_d_n7;
        locals.var_dps_ac_dn8 = assign53290_e68542_d_n8;
        locals.var_dps_ac_rv = 0.0;

        let (assign53300_e68547, assign53300_e68547_d_n5, assign53300_e68547_d_n6, assign53300_e68547_d_n7, assign53300_e68547_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_qbd_dc, locals.var_qbd_dc_dn5, locals.var_qbd_dc_dn6, locals.var_qbd_dc_dn7, locals.var_qbd_dc_dn8,)
    } else {
        (locals.var_qbd_ac, locals.var_qbd_ac_dn5, locals.var_qbd_ac_dn6, locals.var_qbd_ac_dn7, locals.var_qbd_ac_dn8,)
    }
};
        locals.var_qbd_ac = assign53300_e68547;
        locals.var_qbd_ac_dn5 = assign53300_e68547_d_n5;
        locals.var_qbd_ac_dn6 = assign53300_e68547_d_n6;
        locals.var_qbd_ac_dn7 = assign53300_e68547_d_n7;
        locals.var_qbd_ac_dn8 = assign53300_e68547_d_n8;
        locals.var_qbd_ac_rv = 0.0;

        let (assign53310_e68552, assign53310_e68552_d_n5, assign53310_e68552_d_n6, assign53310_e68552_d_n7, assign53310_e68552_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_eta_p_dc, locals.var_eta_p_dc_dn5, locals.var_eta_p_dc_dn6, locals.var_eta_p_dc_dn7, locals.var_eta_p_dc_dn8,)
    } else {
        (locals.var_eta_p_ac, locals.var_eta_p_ac_dn5, locals.var_eta_p_ac_dn6, locals.var_eta_p_ac_dn7, locals.var_eta_p_ac_dn8,)
    }
};
        locals.var_eta_p_ac = assign53310_e68552;
        locals.var_eta_p_ac_dn5 = assign53310_e68552_d_n5;
        locals.var_eta_p_ac_dn6 = assign53310_e68552_d_n6;
        locals.var_eta_p_ac_dn7 = assign53310_e68552_d_n7;
        locals.var_eta_p_ac_dn8 = assign53310_e68552_d_n8;
        locals.var_eta_p_ac_rv = 0.0;

        let (assign53320_e68557, assign53320_e68557_d_n5, assign53320_e68557_d_n6, assign53320_e68557_d_n7, assign53320_e68557_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_alpha_dc, locals.var_alpha_dc_dn5, locals.var_alpha_dc_dn6, locals.var_alpha_dc_dn7, locals.var_alpha_dc_dn8,)
    } else {
        (locals.var_alpha_ac, locals.var_alpha_ac_dn5, locals.var_alpha_ac_dn6, locals.var_alpha_ac_dn7, locals.var_alpha_ac_dn8,)
    }
};
        locals.var_alpha_ac = assign53320_e68557;
        locals.var_alpha_ac_dn5 = assign53320_e68557_d_n5;
        locals.var_alpha_ac_dn6 = assign53320_e68557_d_n6;
        locals.var_alpha_ac_dn7 = assign53320_e68557_d_n7;
        locals.var_alpha_ac_dn8 = assign53320_e68557_d_n8;
        locals.var_alpha_ac_rv = 0.0;

        let (assign53330_e68562, assign53330_e68562_d_n5, assign53330_e68562_d_n6, assign53330_e68562_d_n7, assign53330_e68562_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_qim_dc, locals.var_qim_dc_dn5, locals.var_qim_dc_dn6, locals.var_qim_dc_dn7, locals.var_qim_dc_dn8,)
    } else {
        (locals.var_qim_ac, locals.var_qim_ac_dn5, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8,)
    }
};
        locals.var_qim_ac = assign53330_e68562;
        locals.var_qim_ac_dn5 = assign53330_e68562_d_n5;
        locals.var_qim_ac_dn6 = assign53330_e68562_d_n6;
        locals.var_qim_ac_dn7 = assign53330_e68562_d_n7;
        locals.var_qim_ac_dn8 = assign53330_e68562_d_n8;
        locals.var_qim_ac_rv = 0.0;

        let (assign53340_e68567, assign53340_e68567_d_n5, assign53340_e68567_d_n6, assign53340_e68567_d_n7, assign53340_e68567_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_qim1_dc, locals.var_qim1_dc_dn5, locals.var_qim1_dc_dn6, locals.var_qim1_dc_dn7, locals.var_qim1_dc_dn8,)
    } else {
        (locals.var_qim1_ac, locals.var_qim1_ac_dn5, locals.var_qim1_ac_dn6, locals.var_qim1_ac_dn7, locals.var_qim1_ac_dn8,)
    }
};
        locals.var_qim1_ac = assign53340_e68567;
        locals.var_qim1_ac_dn5 = assign53340_e68567_d_n5;
        locals.var_qim1_ac_dn6 = assign53340_e68567_d_n6;
        locals.var_qim1_ac_dn7 = assign53340_e68567_d_n7;
        locals.var_qim1_ac_dn8 = assign53340_e68567_d_n8;
        locals.var_qim1_ac_rv = 0.0;

        let (assign53350_e68572, assign53350_e68572_d_n5, assign53350_e68572_d_n6, assign53350_e68572_d_n7, assign53350_e68572_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_qeff1_dc, locals.var_qeff1_dc_dn5, locals.var_qeff1_dc_dn6, locals.var_qeff1_dc_dn7, locals.var_qeff1_dc_dn8,)
    } else {
        (locals.var_qeff1_ac, locals.var_qeff1_ac_dn5, locals.var_qeff1_ac_dn6, locals.var_qeff1_ac_dn7, locals.var_qeff1_ac_dn8,)
    }
};
        locals.var_qeff1_ac = assign53350_e68572;
        locals.var_qeff1_ac_dn5 = assign53350_e68572_d_n5;
        locals.var_qeff1_ac_dn6 = assign53350_e68572_d_n6;
        locals.var_qeff1_ac_dn7 = assign53350_e68572_d_n7;
        locals.var_qeff1_ac_dn8 = assign53350_e68572_d_n8;
        locals.var_qeff1_ac_rv = 0.0;

        let (assign53360_e68577, assign53360_e68577_d_n5, assign53360_e68577_d_n6, assign53360_e68577_d_n7, assign53360_e68577_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_gmob_dc, locals.var_gmob_dc_dn5, locals.var_gmob_dc_dn6, locals.var_gmob_dc_dn7, locals.var_gmob_dc_dn8,)
    } else {
        (locals.var_gmob_ac, locals.var_gmob_ac_dn5, locals.var_gmob_ac_dn6, locals.var_gmob_ac_dn7, locals.var_gmob_ac_dn8,)
    }
};
        locals.var_gmob_ac = assign53360_e68577;
        locals.var_gmob_ac_dn5 = assign53360_e68577_d_n5;
        locals.var_gmob_ac_dn6 = assign53360_e68577_d_n6;
        locals.var_gmob_ac_dn7 = assign53360_e68577_d_n7;
        locals.var_gmob_ac_dn8 = assign53360_e68577_d_n8;
        locals.var_gmob_ac_rv = 0.0;

        let (assign53370_e68582, assign53370_e68582_d_n5, assign53370_e68582_d_n6, assign53370_e68582_d_n7, assign53370_e68582_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_s1_dc, locals.var_s1_dc_dn5, locals.var_s1_dc_dn6, locals.var_s1_dc_dn7, locals.var_s1_dc_dn8,)
    } else {
        (locals.var_s1_ac, locals.var_s1_ac_dn5, locals.var_s1_ac_dn6, locals.var_s1_ac_dn7, locals.var_s1_ac_dn8,)
    }
};
        locals.var_s1_ac = assign53370_e68582;
        locals.var_s1_ac_dn5 = assign53370_e68582_d_n5;
        locals.var_s1_ac_dn6 = assign53370_e68582_d_n6;
        locals.var_s1_ac_dn7 = assign53370_e68582_d_n7;
        locals.var_s1_ac_dn8 = assign53370_e68582_d_n8;
        locals.var_s1_ac_rv = 0.0;

        let (assign53380_e68587, assign53380_e68587_d_n5, assign53380_e68587_d_n6, assign53380_e68587_d_n7, assign53380_e68587_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_thesateff_dc, locals.var_thesateff_dc_dn5, locals.var_thesateff_dc_dn6, locals.var_thesateff_dc_dn7, locals.var_thesateff_dc_dn8,)
    } else {
        (locals.var_thesateff_ac, locals.var_thesateff_ac_dn5, locals.var_thesateff_ac_dn6, locals.var_thesateff_ac_dn7, locals.var_thesateff_ac_dn8,)
    }
};
        locals.var_thesateff_ac = assign53380_e68587;
        locals.var_thesateff_ac_dn5 = assign53380_e68587_d_n5;
        locals.var_thesateff_ac_dn6 = assign53380_e68587_d_n6;
        locals.var_thesateff_ac_dn7 = assign53380_e68587_d_n7;
        locals.var_thesateff_ac_dn8 = assign53380_e68587_d_n8;
        locals.var_thesateff_ac_rv = 0.0;

        let (assign53390_e68592, assign53390_e68592_d_n5, assign53390_e68592_d_n6, assign53390_e68592_d_n7, assign53390_e68592_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_voxm_dc, locals.var_voxm_dc_dn5, locals.var_voxm_dc_dn6, locals.var_voxm_dc_dn7, locals.var_voxm_dc_dn8,)
    } else {
        (locals.var_voxm_ac, locals.var_voxm_ac_dn5, locals.var_voxm_ac_dn6, locals.var_voxm_ac_dn7, locals.var_voxm_ac_dn8,)
    }
};
        locals.var_voxm_ac = assign53390_e68592;
        locals.var_voxm_ac_dn5 = assign53390_e68592_d_n5;
        locals.var_voxm_ac_dn6 = assign53390_e68592_d_n6;
        locals.var_voxm_ac_dn7 = assign53390_e68592_d_n7;
        locals.var_voxm_ac_dn8 = assign53390_e68592_d_n8;
        locals.var_voxm_ac_rv = 0.0;

        locals.var_cox_qm = locals.var_cox_i;
        locals.var_cox_qm_dn5 = 0.0;
        locals.var_cox_qm_dn6 = 0.0;
        locals.var_cox_qm_dn7 = 0.0;
        locals.var_cox_qm_dn8 = 0.0;
        locals.var_cox_qm_rv = 0.0;

        let assign53420_e68601: f64 = if locals.var_qq > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1505 = assign53420_e68601;
        locals.var_guard1505_rv = 0.0;

        let (assign53430_e68620, assign53430_e68620_d_n5, assign53430_e68620_d_n6, assign53430_e68620_d_n7, assign53430_e68620_d_n8,) = {
    if (locals.var_guard1505 != 0.0) {
        let assign53430_e68608: f64 = (locals.var_qeff1_ac * locals.var_qeff1_ac);
        let assign53430_e68610: f64 = (assign53430_e68608 + locals.var_qlim2);
        let assign53430_e68612: f64 = (-1.0);
        let assign53430_e68614: f64 = (assign53430_e68612 * 0.16666666666666666);
        let assign53430_e68615: f64 = (assign53430_e68610).powf(assign53430_e68614);
        let assign53430_e68616: f64 = (locals.var_qq * assign53430_e68615);
        let assign53430_e68617: f64 = (1.0 + assign53430_e68616);
        let assign53430_e68618: f64 = (locals.var_cox_i / assign53430_e68617);
        (assign53430_e68618, (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53430_e68614) as f64).is_finite() && ((assign53430_e68614) as f64).fract() == 0.0 { if assign53430_e68614 == 0.0 { 0.0 } else { (assign53430_e68614 * ((assign53430_e68610).powf(assign53430_e68614 - 1.0) * ((locals.var_qeff1_ac_dn5 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn5)))) } } else { (assign53430_e68615 * (assign53430_e68614 * (((locals.var_qeff1_ac_dn5 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn5)) / assign53430_e68610))) })) / (assign53430_e68617 * assign53430_e68617))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53430_e68614) as f64).is_finite() && ((assign53430_e68614) as f64).fract() == 0.0 { if assign53430_e68614 == 0.0 { 0.0 } else { (assign53430_e68614 * ((assign53430_e68610).powf(assign53430_e68614 - 1.0) * ((locals.var_qeff1_ac_dn6 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn6)))) } } else { (assign53430_e68615 * (assign53430_e68614 * (((locals.var_qeff1_ac_dn6 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn6)) / assign53430_e68610))) })) / (assign53430_e68617 * assign53430_e68617))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53430_e68614) as f64).is_finite() && ((assign53430_e68614) as f64).fract() == 0.0 { if assign53430_e68614 == 0.0 { 0.0 } else { (assign53430_e68614 * ((assign53430_e68610).powf(assign53430_e68614 - 1.0) * ((locals.var_qeff1_ac_dn7 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn7)))) } } else { (assign53430_e68615 * (assign53430_e68614 * (((locals.var_qeff1_ac_dn7 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn7)) / assign53430_e68610))) })) / (assign53430_e68617 * assign53430_e68617))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53430_e68614) as f64).is_finite() && ((assign53430_e68614) as f64).fract() == 0.0 { if assign53430_e68614 == 0.0 { 0.0 } else { (assign53430_e68614 * ((assign53430_e68610).powf(assign53430_e68614 - 1.0) * ((locals.var_qeff1_ac_dn8 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn8)))) } } else { (assign53430_e68615 * (assign53430_e68614 * (((locals.var_qeff1_ac_dn8 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn8)) / assign53430_e68610))) })) / (assign53430_e68617 * assign53430_e68617))),)
    } else {
        (locals.var_cox_qm, locals.var_cox_qm_dn5, locals.var_cox_qm_dn6, locals.var_cox_qm_dn7, locals.var_cox_qm_dn8,)
    }
};
        locals.var_cox_qm = assign53430_e68620;
        locals.var_cox_qm_dn5 = assign53430_e68620_d_n5;
        locals.var_cox_qm_dn6 = assign53430_e68620_d_n6;
        locals.var_cox_qm_dn7 = assign53430_e68620_d_n7;
        locals.var_cox_qm_dn8 = assign53430_e68620_d_n8;
        locals.var_cox_qm_rv = 0.0;

        locals.var_gdl_ac = 1.0;
        locals.var_gdl_ac_dn5 = 0.0;
        locals.var_gdl_ac_dn6 = 0.0;
        locals.var_gdl_ac_dn7 = 0.0;
        locals.var_gdl_ac_dn8 = 0.0;
        locals.var_gdl_ac_rv = 0.0;

        locals.var_gmob_dl_ac = 1.0;
        locals.var_gmob_dl_ac_dn5 = 0.0;
        locals.var_gmob_dl_ac_dn6 = 0.0;
        locals.var_gmob_dl_ac_dn7 = 0.0;
        locals.var_gmob_dl_ac_dn8 = 0.0;
        locals.var_gmob_dl_ac_rv = 0.0;

        locals.var_thesat1_ac = 0.0;
        locals.var_thesat1_ac_dn5 = 0.0;
        locals.var_thesat1_ac_dn6 = 0.0;
        locals.var_thesat1_ac_dn7 = 0.0;
        locals.var_thesat1_ac_dn8 = 0.0;
        locals.var_thesat1_ac_rv = 0.0;

        locals.var_gvsat_ac = 1.0;
        locals.var_gvsat_ac_dn5 = 0.0;
        locals.var_gvsat_ac_dn6 = 0.0;
        locals.var_gvsat_ac_dn7 = 0.0;
        locals.var_gvsat_ac_dn8 = 0.0;
        locals.var_gvsat_ac_rv = 0.0;

        locals.var_h_ac = 1.0;
        locals.var_h_ac_dn5 = 0.0;
        locals.var_h_ac_dn6 = 0.0;
        locals.var_h_ac_dn7 = 0.0;
        locals.var_h_ac_dn8 = 0.0;
        locals.var_h_ac_rv = 0.0;

        locals.var_qg_1 = locals.var_voxm_ac;
        locals.var_qg_1_dn5 = locals.var_voxm_ac_dn5;
        locals.var_qg_1_dn6 = locals.var_voxm_ac_dn6;
        locals.var_qg_1_dn7 = locals.var_voxm_ac_dn7;
        locals.var_qg_1_dn8 = locals.var_voxm_ac_dn8;
        locals.var_qg_1_rv = 0.0;

        locals.var_qi = 0.0;
        locals.var_qi_dn5 = 0.0;
        locals.var_qi_dn6 = 0.0;
        locals.var_qi_dn7 = 0.0;
        locals.var_qi_dn8 = 0.0;
        locals.var_qi_rv = 0.0;

        locals.var_qd_1 = 0.0;
        locals.var_qd_1_dn5 = 0.0;
        locals.var_qd_1_dn6 = 0.0;
        locals.var_qd_1_dn7 = 0.0;
        locals.var_qd_1_dn8 = 0.0;
        locals.var_qd_1_rv = 0.0;

        locals.var_qb_1 = locals.var_qg_1;
        locals.var_qb_1_dn5 = locals.var_qg_1_dn5;
        locals.var_qb_1_dn6 = locals.var_qg_1_dn6;
        locals.var_qb_1_dn7 = locals.var_qg_1_dn7;
        locals.var_qb_1_dn8 = locals.var_qg_1_dn8;
        locals.var_qb_1_rv = 0.0;

        let assign53530_e68632: f64 = if locals.var_xg_ac > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1506 = assign53530_e68632;
        locals.var_guard1506_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign53540_e68646, assign53540_e68646_d_n5, assign53540_e68646_d_n6, assign53540_e68646_d_n7, assign53540_e68646_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53540_e68637: f64 = (locals.var_alp1ac_i / locals.var_qim1_ac);
        let assign53540_e68638: f64 = (locals.var_alpac_i + assign53540_e68637);
        let assign53540_e68640: f64 = (assign53540_e68638 * locals.var_qim_ac);
        let assign53540_e68642: f64 = (assign53540_e68640 / locals.var_qim1_ac);
        let assign53540_e68644: f64 = (assign53540_e68642 * locals.var_s1_ac);
        (assign53540_e68644, ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn5) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53540_e68638 * locals.var_qim_ac_dn5)) * locals.var_qim1_ac) - (assign53540_e68640 * locals.var_qim1_ac_dn5)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53540_e68642 * locals.var_s1_ac_dn5)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn6) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53540_e68638 * locals.var_qim_ac_dn6)) * locals.var_qim1_ac) - (assign53540_e68640 * locals.var_qim1_ac_dn6)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53540_e68642 * locals.var_s1_ac_dn6)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn7) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53540_e68638 * locals.var_qim_ac_dn7)) * locals.var_qim1_ac) - (assign53540_e68640 * locals.var_qim1_ac_dn7)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53540_e68642 * locals.var_s1_ac_dn7)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn8) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53540_e68638 * locals.var_qim_ac_dn8)) * locals.var_qim1_ac) - (assign53540_e68640 * locals.var_qim1_ac_dn8)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53540_e68642 * locals.var_s1_ac_dn8)),)
    } else {
        (locals.var_dl__blk1263, locals.var_dl__blk1263_dn5, locals.var_dl__blk1263_dn6, locals.var_dl__blk1263_dn7, locals.var_dl__blk1263_dn8,)
    }
};
        locals.var_dl__blk1263 = assign53540_e68646;
        locals.var_dl__blk1263_dn5 = assign53540_e68646_d_n5;
        locals.var_dl__blk1263_dn6 = assign53540_e68646_d_n6;
        locals.var_dl__blk1263_dn7 = assign53540_e68646_d_n7;
        locals.var_dl__blk1263_dn8 = assign53540_e68646_d_n8;
        locals.var_dl__blk1263_rv = 0.0;

        let assign53550_e68649: f64 = if locals.var_dl__blk1263 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1507 = assign53550_e68649;
        locals.var_guard1507_rv = 0.0;

        let (assign53560_e68663, assign53560_e68663_d_n5, assign53560_e68663_d_n6, assign53560_e68663_d_n7, assign53560_e68663_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1507 != 0.0)) {
        let assign53560_e68656: f64 = (1.0 + locals.var_dl__blk1263);
        let assign53560_e68659: f64 = (locals.var_dl__blk1263 * locals.var_dl__blk1263);
        let assign53560_e68660: f64 = (assign53560_e68656 + assign53560_e68659);
        let assign53560_e68661: f64 = (1.0 / assign53560_e68660);
        (assign53560_e68661, (-((locals.var_dl__blk1263_dn5 + ((locals.var_dl__blk1263_dn5 * locals.var_dl__blk1263) + (locals.var_dl__blk1263 * locals.var_dl__blk1263_dn5))) / (assign53560_e68660 * assign53560_e68660))), (-((locals.var_dl__blk1263_dn6 + ((locals.var_dl__blk1263_dn6 * locals.var_dl__blk1263) + (locals.var_dl__blk1263 * locals.var_dl__blk1263_dn6))) / (assign53560_e68660 * assign53560_e68660))), (-((locals.var_dl__blk1263_dn7 + ((locals.var_dl__blk1263_dn7 * locals.var_dl__blk1263) + (locals.var_dl__blk1263 * locals.var_dl__blk1263_dn7))) / (assign53560_e68660 * assign53560_e68660))), (-((locals.var_dl__blk1263_dn8 + ((locals.var_dl__blk1263_dn8 * locals.var_dl__blk1263) + (locals.var_dl__blk1263 * locals.var_dl__blk1263_dn8))) / (assign53560_e68660 * assign53560_e68660))),)
    } else {
        (locals.var_gdl_ac, locals.var_gdl_ac_dn5, locals.var_gdl_ac_dn6, locals.var_gdl_ac_dn7, locals.var_gdl_ac_dn8,)
    }
};
        locals.var_gdl_ac = assign53560_e68663;
        locals.var_gdl_ac_dn5 = assign53560_e68663_d_n5;
        locals.var_gdl_ac_dn6 = assign53560_e68663_d_n6;
        locals.var_gdl_ac_dn7 = assign53560_e68663_d_n7;
        locals.var_gdl_ac_dn8 = assign53560_e68663_d_n8;
        locals.var_gdl_ac_rv = 0.0;

        let (assign53570_e68672, assign53570_e68672_d_n5, assign53570_e68672_d_n6, assign53570_e68672_d_n7, assign53570_e68672_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1507 == 0.0)) {
        let assign53570_e68670: f64 = (1.0 - locals.var_dl__blk1263);
        (assign53570_e68670, (-locals.var_dl__blk1263_dn5), (-locals.var_dl__blk1263_dn6), (-locals.var_dl__blk1263_dn7), (-locals.var_dl__blk1263_dn8),)
    } else {
        (locals.var_gdl_ac, locals.var_gdl_ac_dn5, locals.var_gdl_ac_dn6, locals.var_gdl_ac_dn7, locals.var_gdl_ac_dn8,)
    }
};
        locals.var_gdl_ac = assign53570_e68672;
        locals.var_gdl_ac_dn5 = assign53570_e68672_d_n5;
        locals.var_gdl_ac_dn6 = assign53570_e68672_d_n6;
        locals.var_gdl_ac_dn7 = assign53570_e68672_d_n7;
        locals.var_gdl_ac_dn8 = assign53570_e68672_d_n8;
        locals.var_gdl_ac_rv = 0.0;

        let (assign53580_e68678, assign53580_e68678_d_n5, assign53580_e68678_d_n6, assign53580_e68678_d_n7, assign53580_e68678_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53580_e68676: f64 = (locals.var_gmob_ac * locals.var_gdl_ac);
        (assign53580_e68676, ((locals.var_gmob_ac_dn5 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn5)), ((locals.var_gmob_ac_dn6 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn6)), ((locals.var_gmob_ac_dn7 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn7)), ((locals.var_gmob_ac_dn8 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn8)),)
    } else {
        (locals.var_gmob_dl_ac, locals.var_gmob_dl_ac_dn5, locals.var_gmob_dl_ac_dn6, locals.var_gmob_dl_ac_dn7, locals.var_gmob_dl_ac_dn8,)
    }
};
        locals.var_gmob_dl_ac = assign53580_e68678;
        locals.var_gmob_dl_ac_dn5 = assign53580_e68678_d_n5;
        locals.var_gmob_dl_ac_dn6 = assign53580_e68678_d_n6;
        locals.var_gmob_dl_ac_dn7 = assign53580_e68678_d_n7;
        locals.var_gmob_dl_ac_dn8 = assign53580_e68678_d_n8;
        locals.var_gmob_dl_ac_rv = 0.0;

        let (assign53590_e68684, assign53590_e68684_d_n5, assign53590_e68684_d_n6, assign53590_e68684_d_n7, assign53590_e68684_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53590_e68682: f64 = (locals.var_thesateff_ac / locals.var_gmob_dl_ac);
        (assign53590_e68682, (((locals.var_thesateff_ac_dn5 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn5)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn6 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn6)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn7 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn7)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn8 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn8)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)),)
    } else {
        (locals.var_thesat1_ac, locals.var_thesat1_ac_dn5, locals.var_thesat1_ac_dn6, locals.var_thesat1_ac_dn7, locals.var_thesat1_ac_dn8,)
    }
};
        locals.var_thesat1_ac = assign53590_e68684;
        locals.var_thesat1_ac_dn5 = assign53590_e68684_d_n5;
        locals.var_thesat1_ac_dn6 = assign53590_e68684_d_n6;
        locals.var_thesat1_ac_dn7 = assign53590_e68684_d_n7;
        locals.var_thesat1_ac_dn8 = assign53590_e68684_d_n8;
        locals.var_thesat1_ac_rv = 0.0;

        let (assign53600_e68694, assign53600_e68694_d_n5, assign53600_e68694_d_n6, assign53600_e68694_d_n7, assign53600_e68694_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53600_e68688: f64 = (locals.var_thesat1_ac * locals.var_thesat1_ac);
        let assign53600_e68690: f64 = (assign53600_e68688 * locals.var_dps_ac);
        let assign53600_e68692: f64 = (assign53600_e68690 * locals.var_dps_ac);
        (assign53600_e68692, ((((((locals.var_thesat1_ac_dn5 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn5)) * locals.var_dps_ac) + (assign53600_e68688 * locals.var_dps_ac_dn5)) * locals.var_dps_ac) + (assign53600_e68690 * locals.var_dps_ac_dn5)), ((((((locals.var_thesat1_ac_dn6 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn6)) * locals.var_dps_ac) + (assign53600_e68688 * locals.var_dps_ac_dn6)) * locals.var_dps_ac) + (assign53600_e68690 * locals.var_dps_ac_dn6)), ((((((locals.var_thesat1_ac_dn7 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn7)) * locals.var_dps_ac) + (assign53600_e68688 * locals.var_dps_ac_dn7)) * locals.var_dps_ac) + (assign53600_e68690 * locals.var_dps_ac_dn7)), ((((((locals.var_thesat1_ac_dn8 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn8)) * locals.var_dps_ac) + (assign53600_e68688 * locals.var_dps_ac_dn8)) * locals.var_dps_ac) + (assign53600_e68690 * locals.var_dps_ac_dn8)),)
    } else {
        (locals.var_zsat__blk1264, locals.var_zsat__blk1264_dn5, locals.var_zsat__blk1264_dn6, locals.var_zsat__blk1264_dn7, locals.var_zsat__blk1264_dn8,)
    }
};
        locals.var_zsat__blk1264 = assign53600_e68694;
        locals.var_zsat__blk1264_dn5 = assign53600_e68694_d_n5;
        locals.var_zsat__blk1264_dn6 = assign53600_e68694_d_n6;
        locals.var_zsat__blk1264_dn7 = assign53600_e68694_d_n7;
        locals.var_zsat__blk1264_dn8 = assign53600_e68694_d_n8;
        locals.var_zsat__blk1264_rv = 0.0;

        let assign53610_e68697: f64 = (-1.0);
        let assign53610_e68698: f64 = if locals.var_chnl_type == assign53610_e68697 { 1.0 } else { 0.0 };
        locals.var_guard1508 = assign53610_e68698;
        locals.var_guard1508_rv = 0.0;

        let (assign53620_e68710, assign53620_e68710_d_n5, assign53620_e68710_d_n6, assign53620_e68710_d_n7, assign53620_e68710_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1508 != 0.0)) {
        let assign53620_e68706: f64 = (locals.var_thesat1_ac * locals.var_dps_ac);
        let assign53620_e68707: f64 = (1.0 + assign53620_e68706);
        let assign53620_e68708: f64 = (locals.var_zsat__blk1264 / assign53620_e68707);
        (assign53620_e68708, (((locals.var_zsat__blk1264_dn5 * assign53620_e68707) - (locals.var_zsat__blk1264 * ((locals.var_thesat1_ac_dn5 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn5)))) / (assign53620_e68707 * assign53620_e68707)), (((locals.var_zsat__blk1264_dn6 * assign53620_e68707) - (locals.var_zsat__blk1264 * ((locals.var_thesat1_ac_dn6 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn6)))) / (assign53620_e68707 * assign53620_e68707)), (((locals.var_zsat__blk1264_dn7 * assign53620_e68707) - (locals.var_zsat__blk1264 * ((locals.var_thesat1_ac_dn7 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn7)))) / (assign53620_e68707 * assign53620_e68707)), (((locals.var_zsat__blk1264_dn8 * assign53620_e68707) - (locals.var_zsat__blk1264 * ((locals.var_thesat1_ac_dn8 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn8)))) / (assign53620_e68707 * assign53620_e68707)),)
    } else {
        (locals.var_zsat__blk1264, locals.var_zsat__blk1264_dn5, locals.var_zsat__blk1264_dn6, locals.var_zsat__blk1264_dn7, locals.var_zsat__blk1264_dn8,)
    }
};
        locals.var_zsat__blk1264 = assign53620_e68710;
        locals.var_zsat__blk1264_dn5 = assign53620_e68710_d_n5;
        locals.var_zsat__blk1264_dn6 = assign53620_e68710_d_n6;
        locals.var_zsat__blk1264_dn7 = assign53620_e68710_d_n7;
        locals.var_zsat__blk1264_dn8 = assign53620_e68710_d_n8;
        locals.var_zsat__blk1264_rv = 0.0;

        let (assign53630_e68725, assign53630_e68725_d_n5, assign53630_e68725_d_n6, assign53630_e68725_d_n7, assign53630_e68725_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53630_e68718: f64 = (2.0 * locals.var_zsat__blk1264);
        let assign53630_e68719: f64 = (1.0 + assign53630_e68718);
        let assign53630_e68720: f64 = (assign53630_e68719).sqrt();
        let assign53630_e68721: f64 = (1.0 + assign53630_e68720);
        let assign53630_e68722: f64 = (locals.var_gmob_dl_ac * assign53630_e68721);
        let assign53630_e68723: f64 = (0.5 * assign53630_e68722);
        (assign53630_e68723, (0.5 * ((locals.var_gmob_dl_ac_dn5 * assign53630_e68721) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1264_dn5) / (2.0 * assign53630_e68720))))), (0.5 * ((locals.var_gmob_dl_ac_dn6 * assign53630_e68721) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1264_dn6) / (2.0 * assign53630_e68720))))), (0.5 * ((locals.var_gmob_dl_ac_dn7 * assign53630_e68721) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1264_dn7) / (2.0 * assign53630_e68720))))), (0.5 * ((locals.var_gmob_dl_ac_dn8 * assign53630_e68721) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1264_dn8) / (2.0 * assign53630_e68720))))),)
    } else {
        (locals.var_gvsat_ac, locals.var_gvsat_ac_dn5, locals.var_gvsat_ac_dn6, locals.var_gvsat_ac_dn7, locals.var_gvsat_ac_dn8,)
    }
};
        locals.var_gvsat_ac = assign53630_e68725;
        locals.var_gvsat_ac_dn5 = assign53630_e68725_d_n5;
        locals.var_gvsat_ac_dn6 = assign53630_e68725_d_n6;
        locals.var_gvsat_ac_dn7 = assign53630_e68725_d_n7;
        locals.var_gvsat_ac_dn8 = assign53630_e68725_d_n8;
        locals.var_gvsat_ac_rv = 0.0;

        let (assign53640_e68731, assign53640_e68731_d_n5, assign53640_e68731_d_n6, assign53640_e68731_d_n7, assign53640_e68731_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53640_e68729: f64 = (locals.var_gmob_dl_ac / locals.var_gvsat_ac);
        (assign53640_e68729, (((locals.var_gmob_dl_ac_dn5 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn5)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn6 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn6)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn7 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn7)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn8 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn8)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign53640_e68731;
        locals.var_temp__blk936_dn5 = assign53640_e68731_d_n5;
        locals.var_temp__blk936_dn6 = assign53640_e68731_d_n6;
        locals.var_temp__blk936_dn7 = assign53640_e68731_d_n7;
        locals.var_temp__blk936_dn8 = assign53640_e68731_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign53650_e68745, assign53650_e68745_d_n5, assign53650_e68745_d_n6, assign53650_e68745_d_n7, assign53650_e68745_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53650_e68738: f64 = (locals.var_zsat__blk1264 * locals.var_temp__blk936);
        let assign53650_e68740: f64 = (assign53650_e68738 * locals.var_temp__blk936);
        let assign53650_e68741: f64 = (0.5 * assign53650_e68740);
        let assign53650_e68742: f64 = (1.0 + assign53650_e68741);
        let assign53650_e68743: f64 = (locals.var_alpha_ac * assign53650_e68742);
        (assign53650_e68743, ((locals.var_alpha_ac_dn5 * assign53650_e68742) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1264_dn5 * locals.var_temp__blk936) + (locals.var_zsat__blk1264 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign53650_e68738 * locals.var_temp__blk936_dn5))))), ((locals.var_alpha_ac_dn6 * assign53650_e68742) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1264_dn6 * locals.var_temp__blk936) + (locals.var_zsat__blk1264 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign53650_e68738 * locals.var_temp__blk936_dn6))))), ((locals.var_alpha_ac_dn7 * assign53650_e68742) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1264_dn7 * locals.var_temp__blk936) + (locals.var_zsat__blk1264 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign53650_e68738 * locals.var_temp__blk936_dn7))))), ((locals.var_alpha_ac_dn8 * assign53650_e68742) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1264_dn8 * locals.var_temp__blk936) + (locals.var_zsat__blk1264 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign53650_e68738 * locals.var_temp__blk936_dn8))))),)
    } else {
        (locals.var_alpha1__blk1265, locals.var_alpha1__blk1265_dn5, locals.var_alpha1__blk1265_dn6, locals.var_alpha1__blk1265_dn7, locals.var_alpha1__blk1265_dn8,)
    }
};
        locals.var_alpha1__blk1265 = assign53650_e68745;
        locals.var_alpha1__blk1265_dn5 = assign53650_e68745_d_n5;
        locals.var_alpha1__blk1265_dn6 = assign53650_e68745_d_n6;
        locals.var_alpha1__blk1265_dn7 = assign53650_e68745_d_n7;
        locals.var_alpha1__blk1265_dn8 = assign53650_e68745_d_n8;
        locals.var_alpha1__blk1265_rv = 0.0;

        let (assign53660_e68753, assign53660_e68753_d_n5, assign53660_e68753_d_n6, assign53660_e68753_d_n7, assign53660_e68753_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53660_e68749: f64 = (locals.var_temp__blk936 * locals.var_qim1_ac);
        let assign53660_e68751: f64 = (assign53660_e68749 / locals.var_alpha1__blk1265);
        (assign53660_e68751, (((((locals.var_temp__blk936_dn5 * locals.var_qim1_ac) + (locals.var_temp__blk936 * locals.var_qim1_ac_dn5)) * locals.var_alpha1__blk1265) - (assign53660_e68749 * locals.var_alpha1__blk1265_dn5)) / (locals.var_alpha1__blk1265 * locals.var_alpha1__blk1265)), (((((locals.var_temp__blk936_dn6 * locals.var_qim1_ac) + (locals.var_temp__blk936 * locals.var_qim1_ac_dn6)) * locals.var_alpha1__blk1265) - (assign53660_e68749 * locals.var_alpha1__blk1265_dn6)) / (locals.var_alpha1__blk1265 * locals.var_alpha1__blk1265)), (((((locals.var_temp__blk936_dn7 * locals.var_qim1_ac) + (locals.var_temp__blk936 * locals.var_qim1_ac_dn7)) * locals.var_alpha1__blk1265) - (assign53660_e68749 * locals.var_alpha1__blk1265_dn7)) / (locals.var_alpha1__blk1265 * locals.var_alpha1__blk1265)), (((((locals.var_temp__blk936_dn8 * locals.var_qim1_ac) + (locals.var_temp__blk936 * locals.var_qim1_ac_dn8)) * locals.var_alpha1__blk1265) - (assign53660_e68749 * locals.var_alpha1__blk1265_dn8)) / (locals.var_alpha1__blk1265 * locals.var_alpha1__blk1265)),)
    } else {
        (locals.var_h_ac, locals.var_h_ac_dn5, locals.var_h_ac_dn6, locals.var_h_ac_dn7, locals.var_h_ac_dn8,)
    }
};
        locals.var_h_ac = assign53660_e68753;
        locals.var_h_ac_dn5 = assign53660_e68753_d_n5;
        locals.var_h_ac_dn6 = assign53660_e68753_d_n6;
        locals.var_h_ac_dn7 = assign53660_e68753_d_n7;
        locals.var_h_ac_dn8 = assign53660_e68753_d_n8;
        locals.var_h_ac_rv = 0.0;

        let (assign53670_e68761, assign53670_e68761_d_n5, assign53670_e68761_d_n6, assign53670_e68761_d_n7, assign53670_e68761_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53670_e68758: f64 = (locals.var_dps_ac / locals.var_h_ac);
        let assign53670_e68759: f64 = (0.5 * assign53670_e68758);
        (assign53670_e68759, (0.5 * (((locals.var_dps_ac_dn5 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn5)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn6 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn6)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn7 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn7)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn8 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn8)) / (locals.var_h_ac * locals.var_h_ac))),)
    } else {
        (locals.var_fj, locals.var_fj_dn5, locals.var_fj_dn6, locals.var_fj_dn7, locals.var_fj_dn8,)
    }
};
        locals.var_fj = assign53670_e68761;
        locals.var_fj_dn5 = assign53670_e68761_d_n5;
        locals.var_fj_dn6 = assign53670_e68761_d_n6;
        locals.var_fj_dn7 = assign53670_e68761_d_n7;
        locals.var_fj_dn8 = assign53670_e68761_d_n8;
        locals.var_fj_rv = 0.0;

        let (assign53680_e68767, assign53680_e68767_d_n5, assign53680_e68767_d_n6, assign53680_e68767_d_n7, assign53680_e68767_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53680_e68765: f64 = (locals.var_fj * locals.var_fj);
        (assign53680_e68765, ((locals.var_fj_dn5 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn5)), ((locals.var_fj_dn6 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn6)), ((locals.var_fj_dn7 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn7)), ((locals.var_fj_dn8 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn8)),)
    } else {
        (locals.var_fj2, locals.var_fj2_dn5, locals.var_fj2_dn6, locals.var_fj2_dn7, locals.var_fj2_dn8,)
    }
};
        locals.var_fj2 = assign53680_e68767;
        locals.var_fj2_dn5 = assign53680_e68767_d_n5;
        locals.var_fj2_dn6 = assign53680_e68767_d_n6;
        locals.var_fj2_dn7 = assign53680_e68767_d_n7;
        locals.var_fj2_dn8 = assign53680_e68767_d_n8;
        locals.var_fj2_rv = 0.0;

        let (assign53690_e68787, assign53690_e68787_d_n5, assign53690_e68787_d_n6, assign53690_e68787_d_n7, assign53690_e68787_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53690_e68773: f64 = (locals.var_eta_p_ac * locals.var_dps_ac);
        let assign53690_e68776: f64 = (locals.var_fj * locals.var_gdl_ac);
        let assign53690_e68778: f64 = (assign53690_e68776 * 0.3333333333333333);
        let assign53690_e68780: f64 = (assign53690_e68778 - 1.0);
        let assign53690_e68782: f64 = (assign53690_e68780 + locals.var_gdl_ac);
        let assign53690_e68783: f64 = (assign53690_e68773 * assign53690_e68782);
        let assign53690_e68784: f64 = (0.5 * assign53690_e68783);
        let assign53690_e68785: f64 = (locals.var_voxm_ac + assign53690_e68784);
        (assign53690_e68785, (locals.var_voxm_ac_dn5 + (0.5 * ((((locals.var_eta_p_ac_dn5 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn5)) * assign53690_e68782) + (assign53690_e68773 * ((((locals.var_fj_dn5 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn5)) * 0.3333333333333333) + locals.var_gdl_ac_dn5))))), (locals.var_voxm_ac_dn6 + (0.5 * ((((locals.var_eta_p_ac_dn6 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn6)) * assign53690_e68782) + (assign53690_e68773 * ((((locals.var_fj_dn6 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn6)) * 0.3333333333333333) + locals.var_gdl_ac_dn6))))), (locals.var_voxm_ac_dn7 + (0.5 * ((((locals.var_eta_p_ac_dn7 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn7)) * assign53690_e68782) + (assign53690_e68773 * ((((locals.var_fj_dn7 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn7)) * 0.3333333333333333) + locals.var_gdl_ac_dn7))))), (locals.var_voxm_ac_dn8 + (0.5 * ((((locals.var_eta_p_ac_dn8 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn8)) * assign53690_e68782) + (assign53690_e68773 * ((((locals.var_fj_dn8 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn8)) * 0.3333333333333333) + locals.var_gdl_ac_dn8))))),)
    } else {
        (locals.var_qg_1, locals.var_qg_1_dn5, locals.var_qg_1_dn6, locals.var_qg_1_dn7, locals.var_qg_1_dn8,)
    }
};
        locals.var_qg_1 = assign53690_e68787;
        locals.var_qg_1_dn5 = assign53690_e68787_d_n5;
        locals.var_qg_1_dn6 = assign53690_e68787_d_n6;
        locals.var_qg_1_dn7 = assign53690_e68787_d_n7;
        locals.var_qg_1_dn8 = assign53690_e68787_d_n8;
        locals.var_qg_1_rv = 0.0;

        let (assign53700_e68795, assign53700_e68795_d_n5, assign53700_e68795_d_n6, assign53700_e68795_d_n7, assign53700_e68795_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53700_e68791: f64 = (locals.var_alpha_ac * locals.var_dps_ac);
        let assign53700_e68793: f64 = (assign53700_e68791 * 0.16666666666666666);
        (assign53700_e68793, (((locals.var_alpha_ac_dn5 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn5)) * 0.16666666666666666), (((locals.var_alpha_ac_dn6 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn6)) * 0.16666666666666666), (((locals.var_alpha_ac_dn7 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn7)) * 0.16666666666666666), (((locals.var_alpha_ac_dn8 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn8)) * 0.16666666666666666),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign53700_e68795;
        locals.var_temp__blk936_dn5 = assign53700_e68795_d_n5;
        locals.var_temp__blk936_dn6 = assign53700_e68795_d_n6;
        locals.var_temp__blk936_dn7 = assign53700_e68795_d_n7;
        locals.var_temp__blk936_dn8 = assign53700_e68795_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign53710_e68798: f64 = if p.p49 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1509 = assign53710_e68798;
        locals.var_guard1509_rv = 0.0;

        let (assign53720_e68804, assign53720_e68804_d_n5, assign53720_e68804_d_n6, assign53720_e68804_d_n7, assign53720_e68804_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1509 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qclm, locals.var_qclm_dn5, locals.var_qclm_dn6, locals.var_qclm_dn7, locals.var_qclm_dn8,)
    }
};
        locals.var_qclm = assign53720_e68804;
        locals.var_qclm_dn5 = assign53720_e68804_d_n5;
        locals.var_qclm_dn6 = assign53720_e68804_d_n6;
        locals.var_qclm_dn7 = assign53720_e68804_d_n7;
        locals.var_qclm_dn8 = assign53720_e68804_d_n8;
        locals.var_qclm_rv = 0.0;

        let (assign53730_e68824, assign53730_e68824_d_n5, assign53730_e68824_d_n6, assign53730_e68824_d_n7, assign53730_e68824_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1509 != 0.0)) {
        let assign53730_e68810: f64 = (0.5 * locals.var_gdl_ac);
        let assign53730_e68812: f64 = (assign53730_e68810 * locals.var_gdl_ac);
        let assign53730_e68816: f64 = (3.0 * locals.var_temp__blk936);
        let assign53730_e68819: f64 = (2.0 - locals.var_fj);
        let assign53730_e68820: f64 = (assign53730_e68816 * assign53730_e68819);
        let assign53730_e68821: f64 = (locals.var_qim_ac - assign53730_e68820);
        let assign53730_e68822: f64 = (assign53730_e68812 * assign53730_e68821);
        (assign53730_e68822, (((((0.5 * locals.var_gdl_ac_dn5) * locals.var_gdl_ac) + (assign53730_e68810 * locals.var_gdl_ac_dn5)) * assign53730_e68821) + (assign53730_e68812 * (locals.var_qim_ac_dn5 - (((3.0 * locals.var_temp__blk936_dn5) * assign53730_e68819) + (assign53730_e68816 * (-locals.var_fj_dn5)))))), (((((0.5 * locals.var_gdl_ac_dn6) * locals.var_gdl_ac) + (assign53730_e68810 * locals.var_gdl_ac_dn6)) * assign53730_e68821) + (assign53730_e68812 * (locals.var_qim_ac_dn6 - (((3.0 * locals.var_temp__blk936_dn6) * assign53730_e68819) + (assign53730_e68816 * (-locals.var_fj_dn6)))))), (((((0.5 * locals.var_gdl_ac_dn7) * locals.var_gdl_ac) + (assign53730_e68810 * locals.var_gdl_ac_dn7)) * assign53730_e68821) + (assign53730_e68812 * (locals.var_qim_ac_dn7 - (((3.0 * locals.var_temp__blk936_dn7) * assign53730_e68819) + (assign53730_e68816 * (-locals.var_fj_dn7)))))), (((((0.5 * locals.var_gdl_ac_dn8) * locals.var_gdl_ac) + (assign53730_e68810 * locals.var_gdl_ac_dn8)) * assign53730_e68821) + (assign53730_e68812 * (locals.var_qim_ac_dn8 - (((3.0 * locals.var_temp__blk936_dn8) * assign53730_e68819) + (assign53730_e68816 * (-locals.var_fj_dn8)))))),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8,)
    }
};
        locals.var_qd_1 = assign53730_e68824;
        locals.var_qd_1_dn5 = assign53730_e68824_d_n5;
        locals.var_qd_1_dn6 = assign53730_e68824_d_n6;
        locals.var_qd_1_dn7 = assign53730_e68824_d_n7;
        locals.var_qd_1_dn8 = assign53730_e68824_d_n8;
        locals.var_qd_1_rv = 0.0;

        let (assign53740_e68841, assign53740_e68841_d_n5, assign53740_e68841_d_n6, assign53740_e68841_d_n7, assign53740_e68841_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1509 == 0.0)) {
        let assign53740_e68831: f64 = (1.0 - locals.var_gdl_ac);
        let assign53740_e68836: f64 = (locals.var_alpha_ac * locals.var_dps_ac);
        let assign53740_e68837: f64 = (0.5 * assign53740_e68836);
        let assign53740_e68838: f64 = (locals.var_qim_ac - assign53740_e68837);
        let assign53740_e68839: f64 = (assign53740_e68831 * assign53740_e68838);
        (assign53740_e68839, (((-locals.var_gdl_ac_dn5) * assign53740_e68838) + (assign53740_e68831 * (locals.var_qim_ac_dn5 - (0.5 * ((locals.var_alpha_ac_dn5 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn5)))))), (((-locals.var_gdl_ac_dn6) * assign53740_e68838) + (assign53740_e68831 * (locals.var_qim_ac_dn6 - (0.5 * ((locals.var_alpha_ac_dn6 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn6)))))), (((-locals.var_gdl_ac_dn7) * assign53740_e68838) + (assign53740_e68831 * (locals.var_qim_ac_dn7 - (0.5 * ((locals.var_alpha_ac_dn7 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn7)))))), (((-locals.var_gdl_ac_dn8) * assign53740_e68838) + (assign53740_e68831 * (locals.var_qim_ac_dn8 - (0.5 * ((locals.var_alpha_ac_dn8 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn8)))))),)
    } else {
        (locals.var_qclm, locals.var_qclm_dn5, locals.var_qclm_dn6, locals.var_qclm_dn7, locals.var_qclm_dn8,)
    }
};
        locals.var_qclm = assign53740_e68841;
        locals.var_qclm_dn5 = assign53740_e68841_d_n5;
        locals.var_qclm_dn6 = assign53740_e68841_d_n6;
        locals.var_qclm_dn7 = assign53740_e68841_d_n7;
        locals.var_qclm_dn8 = assign53740_e68841_d_n8;
        locals.var_qclm_rv = 0.0;

        let (assign53750_e68870, assign53750_e68870_d_n5, assign53750_e68870_d_n6, assign53750_e68870_d_n7, assign53750_e68870_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1509 == 0.0)) {
        let assign53750_e68849: f64 = (locals.var_gdl_ac * locals.var_gdl_ac);
        let assign53750_e68854: f64 = (1.0 - locals.var_fj);
        let assign53750_e68857: f64 = (0.2 * locals.var_fj2);
        let assign53750_e68858: f64 = (assign53750_e68854 - assign53750_e68857);
        let assign53750_e68859: f64 = (locals.var_temp__blk936 * assign53750_e68858);
        let assign53750_e68860: f64 = (locals.var_qim_ac - assign53750_e68859);
        let assign53750_e68861: f64 = (assign53750_e68849 * assign53750_e68860);
        let assign53750_e68865: f64 = (1.0 + locals.var_gdl_ac);
        let assign53750_e68866: f64 = (locals.var_qclm * assign53750_e68865);
        let assign53750_e68867: f64 = (assign53750_e68861 + assign53750_e68866);
        let assign53750_e68868: f64 = (0.5 * assign53750_e68867);
        (assign53750_e68868, (0.5 * (((((locals.var_gdl_ac_dn5 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn5)) * assign53750_e68860) + (assign53750_e68849 * (locals.var_qim_ac_dn5 - ((locals.var_temp__blk936_dn5 * assign53750_e68858) + (locals.var_temp__blk936 * ((-locals.var_fj_dn5) - (0.2 * locals.var_fj2_dn5))))))) + ((locals.var_qclm_dn5 * assign53750_e68865) + (locals.var_qclm * locals.var_gdl_ac_dn5)))), (0.5 * (((((locals.var_gdl_ac_dn6 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn6)) * assign53750_e68860) + (assign53750_e68849 * (locals.var_qim_ac_dn6 - ((locals.var_temp__blk936_dn6 * assign53750_e68858) + (locals.var_temp__blk936 * ((-locals.var_fj_dn6) - (0.2 * locals.var_fj2_dn6))))))) + ((locals.var_qclm_dn6 * assign53750_e68865) + (locals.var_qclm * locals.var_gdl_ac_dn6)))), (0.5 * (((((locals.var_gdl_ac_dn7 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn7)) * assign53750_e68860) + (assign53750_e68849 * (locals.var_qim_ac_dn7 - ((locals.var_temp__blk936_dn7 * assign53750_e68858) + (locals.var_temp__blk936 * ((-locals.var_fj_dn7) - (0.2 * locals.var_fj2_dn7))))))) + ((locals.var_qclm_dn7 * assign53750_e68865) + (locals.var_qclm * locals.var_gdl_ac_dn7)))), (0.5 * (((((locals.var_gdl_ac_dn8 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn8)) * assign53750_e68860) + (assign53750_e68849 * (locals.var_qim_ac_dn8 - ((locals.var_temp__blk936_dn8 * assign53750_e68858) + (locals.var_temp__blk936 * ((-locals.var_fj_dn8) - (0.2 * locals.var_fj2_dn8))))))) + ((locals.var_qclm_dn8 * assign53750_e68865) + (locals.var_qclm * locals.var_gdl_ac_dn8)))),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8,)
    }
};
        locals.var_qd_1 = assign53750_e68870;
        locals.var_qd_1_dn5 = assign53750_e68870_d_n5;
        locals.var_qd_1_dn6 = assign53750_e68870_d_n6;
        locals.var_qd_1_dn7 = assign53750_e68870_d_n7;
        locals.var_qd_1_dn8 = assign53750_e68870_d_n8;
        locals.var_qd_1_rv = 0.0;

        let (assign53760_e68882, assign53760_e68882_d_n5, assign53760_e68882_d_n6, assign53760_e68882_d_n7, assign53760_e68882_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53760_e68876: f64 = (locals.var_temp__blk936 * locals.var_fj);
        let assign53760_e68877: f64 = (locals.var_qim_ac + assign53760_e68876);
        let assign53760_e68878: f64 = (locals.var_gdl_ac * assign53760_e68877);
        let assign53760_e68880: f64 = (assign53760_e68878 + locals.var_qclm);
        (assign53760_e68880, (((locals.var_gdl_ac_dn5 * assign53760_e68877) + (locals.var_gdl_ac * (locals.var_qim_ac_dn5 + ((locals.var_temp__blk936_dn5 * locals.var_fj) + (locals.var_temp__blk936 * locals.var_fj_dn5))))) + locals.var_qclm_dn5), (((locals.var_gdl_ac_dn6 * assign53760_e68877) + (locals.var_gdl_ac * (locals.var_qim_ac_dn6 + ((locals.var_temp__blk936_dn6 * locals.var_fj) + (locals.var_temp__blk936 * locals.var_fj_dn6))))) + locals.var_qclm_dn6), (((locals.var_gdl_ac_dn7 * assign53760_e68877) + (locals.var_gdl_ac * (locals.var_qim_ac_dn7 + ((locals.var_temp__blk936_dn7 * locals.var_fj) + (locals.var_temp__blk936 * locals.var_fj_dn7))))) + locals.var_qclm_dn7), (((locals.var_gdl_ac_dn8 * assign53760_e68877) + (locals.var_gdl_ac * (locals.var_qim_ac_dn8 + ((locals.var_temp__blk936_dn8 * locals.var_fj) + (locals.var_temp__blk936 * locals.var_fj_dn8))))) + locals.var_qclm_dn8),)
    } else {
        (locals.var_qi, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8,)
    }
};
        locals.var_qi = assign53760_e68882;
        locals.var_qi_dn5 = assign53760_e68882_d_n5;
        locals.var_qi_dn6 = assign53760_e68882_d_n6;
        locals.var_qi_dn7 = assign53760_e68882_d_n7;
        locals.var_qi_dn8 = assign53760_e68882_d_n8;
        locals.var_qi_rv = 0.0;

        let (assign53770_e68888, assign53770_e68888_d_n5, assign53770_e68888_d_n6, assign53770_e68888_d_n7, assign53770_e68888_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53770_e68886: f64 = (locals.var_qg_1 - locals.var_qi);
        (assign53770_e68886, (locals.var_qg_1_dn5 - locals.var_qi_dn5), (locals.var_qg_1_dn6 - locals.var_qi_dn6), (locals.var_qg_1_dn7 - locals.var_qi_dn7), (locals.var_qg_1_dn8 - locals.var_qi_dn8),)
    } else {
        (locals.var_qb_1, locals.var_qb_1_dn5, locals.var_qb_1_dn6, locals.var_qb_1_dn7, locals.var_qb_1_dn8,)
    }
};
        locals.var_qb_1 = assign53770_e68888;
        locals.var_qb_1_dn5 = assign53770_e68888_d_n5;
        locals.var_qb_1_dn6 = assign53770_e68888_d_n6;
        locals.var_qb_1_dn7 = assign53770_e68888_d_n7;
        locals.var_qb_1_dn8 = assign53770_e68888_d_n8;
        locals.var_qb_1_rv = 0.0;

        let assign53780_e68891: f64 = (locals.var_qg_1 * locals.var_cox_qm);
        locals.var_qg = assign53780_e68891;
        locals.var_qg_dn5 = ((locals.var_qg_1_dn5 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn5));
        locals.var_qg_dn6 = ((locals.var_qg_1_dn6 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn6));
        locals.var_qg_dn7 = ((locals.var_qg_1_dn7 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn7));
        locals.var_qg_dn8 = ((locals.var_qg_1_dn8 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn8));
        locals.var_qg_rv = 0.0;

        let assign53790_e68893: f64 = (-locals.var_qd_1);
        let assign53790_e68895: f64 = (assign53790_e68893 * locals.var_cox_qm);
        locals.var_qd = assign53790_e68895;
        locals.var_qd_dn5 = (((-locals.var_qd_1_dn5) * locals.var_cox_qm) + (assign53790_e68893 * locals.var_cox_qm_dn5));
        locals.var_qd_dn6 = (((-locals.var_qd_1_dn6) * locals.var_cox_qm) + (assign53790_e68893 * locals.var_cox_qm_dn6));
        locals.var_qd_dn7 = (((-locals.var_qd_1_dn7) * locals.var_cox_qm) + (assign53790_e68893 * locals.var_cox_qm_dn7));
        locals.var_qd_dn8 = (((-locals.var_qd_1_dn8) * locals.var_cox_qm) + (assign53790_e68893 * locals.var_cox_qm_dn8));
        locals.var_qd_rv = 0.0;

        let assign53800_e68897: f64 = (-locals.var_qb_1);
        let assign53800_e68899: f64 = (assign53800_e68897 * locals.var_cox_qm);
        locals.var_qb = assign53800_e68899;
        locals.var_qb_dn5 = (((-locals.var_qb_1_dn5) * locals.var_cox_qm) + (assign53800_e68897 * locals.var_cox_qm_dn5));
        locals.var_qb_dn6 = (((-locals.var_qb_1_dn6) * locals.var_cox_qm) + (assign53800_e68897 * locals.var_cox_qm_dn6));
        locals.var_qb_dn7 = (((-locals.var_qb_1_dn7) * locals.var_cox_qm) + (assign53800_e68897 * locals.var_cox_qm_dn7));
        locals.var_qb_dn8 = (((-locals.var_qb_1_dn8) * locals.var_cox_qm) + (assign53800_e68897 * locals.var_cox_qm_dn8));
        locals.var_qb_rv = 0.0;

        locals.var_qsinr = 0.0;
        locals.var_qsinr_dn5 = 0.0;
        locals.var_qsinr_dn6 = 0.0;
        locals.var_qsinr_dn7 = 0.0;
        locals.var_qsinr_dn8 = 0.0;
        locals.var_qsinr_rv = 0.0;

        locals.var_qdinr = 0.0;
        locals.var_qdinr_dn5 = 0.0;
        locals.var_qdinr_dn6 = 0.0;
        locals.var_qdinr_dn7 = 0.0;
        locals.var_qdinr_dn8 = 0.0;
        locals.var_qdinr_rv = 0.0;

        locals.var_qginr = 0.0;
        locals.var_qginr_dn5 = 0.0;
        locals.var_qginr_dn6 = 0.0;
        locals.var_qginr_dn7 = 0.0;
        locals.var_qginr_dn8 = 0.0;
        locals.var_qginr_rv = 0.0;

        let assign53840_e68909: f64 = if ((locals.var_cinr_i > 0.0) || (locals.var_cinrd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1510 = assign53840_e68909;
        locals.var_guard1510_rv = 0.0;

        let (assign53850_e68913, assign53850_e68913_d_n5, assign53850_e68913_d_n6, assign53850_e68913_d_n7, assign53850_e68913_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_finracc, locals.var_finracc_dn5, locals.var_finracc_dn6, locals.var_finracc_dn7, locals.var_finracc_dn8,)
    }
};
        locals.var_finracc = assign53850_e68913;
        locals.var_finracc_dn5 = assign53850_e68913_d_n5;
        locals.var_finracc_dn6 = assign53850_e68913_d_n6;
        locals.var_finracc_dn7 = assign53850_e68913_d_n7;
        locals.var_finracc_dn8 = assign53850_e68913_d_n8;
        locals.var_finracc_rv = 0.0;

        let (assign53860_e68917, assign53860_e68917_d_n5, assign53860_e68917_d_n6, assign53860_e68917_d_n7, assign53860_e68917_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        (locals.var_vgb1_ac, locals.var_vgb1_ac_dn5, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8,)
    } else {
        (locals.var_dvinracc, locals.var_dvinracc_dn5, locals.var_dvinracc_dn6, locals.var_dvinracc_dn7, locals.var_dvinracc_dn8,)
    }
};
        locals.var_dvinracc = assign53860_e68917;
        locals.var_dvinracc_dn5 = assign53860_e68917_d_n5;
        locals.var_dvinracc_dn6 = assign53860_e68917_d_n6;
        locals.var_dvinracc_dn7 = assign53860_e68917_d_n7;
        locals.var_dvinracc_dn8 = assign53860_e68917_d_n8;
        locals.var_dvinracc_rv = 0.0;

        let assign53870_e68920: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1511 = assign53870_e68920;
        locals.var_guard1511_rv = 0.0;

        let (assign53880_e68930, assign53880_e68930_d_n5, assign53880_e68930_d_n6, assign53880_e68930_d_n7, assign53880_e68930_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53880_e68926: f64 = (locals.var_vgb1_ac - locals.var_dvfbinr_i);
        let assign53880_e68928: f64 = (assign53880_e68926 + locals.var_vinr_max);
        (assign53880_e68928, locals.var_vgb1_ac_dn5, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8,)
    } else {
        (locals.var_vginr, locals.var_vginr_dn5, locals.var_vginr_dn6, locals.var_vginr_dn7, locals.var_vginr_dn8,)
    }
};
        locals.var_vginr = assign53880_e68930;
        locals.var_vginr_dn5 = assign53880_e68930_d_n5;
        locals.var_vginr_dn6 = assign53880_e68930_d_n6;
        locals.var_vginr_dn7 = assign53880_e68930_d_n7;
        locals.var_vginr_dn8 = assign53880_e68930_d_n8;
        locals.var_vginr_rv = 0.0;

        let (assign53890_e68951, assign53890_e68951_d_n5, assign53890_e68951_d_n6, assign53890_e68951_d_n7, assign53890_e68951_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53890_e68937: f64 = (locals.var_vginr + locals.var_vinr_max);
        let assign53890_e68940: f64 = (locals.var_vginr - locals.var_vinr_max);
        let assign53890_e68943: f64 = (locals.var_vginr - locals.var_vinr_max);
        let assign53890_e68944: f64 = (assign53890_e68940 * assign53890_e68943);
        let assign53890_e68946: f64 = (assign53890_e68944 + locals.var_ainr);
        let assign53890_e68947: f64 = (assign53890_e68946).sqrt();
        let assign53890_e68948: f64 = (assign53890_e68937 + assign53890_e68947);
        let assign53890_e68949: f64 = (0.5 * assign53890_e68948);
        (assign53890_e68949, (0.5 * (locals.var_vginr_dn5 + (((locals.var_vginr_dn5 * assign53890_e68943) + (assign53890_e68940 * locals.var_vginr_dn5)) / (2.0 * assign53890_e68947)))), (0.5 * (locals.var_vginr_dn6 + (((locals.var_vginr_dn6 * assign53890_e68943) + (assign53890_e68940 * locals.var_vginr_dn6)) / (2.0 * assign53890_e68947)))), (0.5 * (locals.var_vginr_dn7 + (((locals.var_vginr_dn7 * assign53890_e68943) + (assign53890_e68940 * locals.var_vginr_dn7)) / (2.0 * assign53890_e68947)))), (0.5 * (locals.var_vginr_dn8 + (((locals.var_vginr_dn8 * assign53890_e68943) + (assign53890_e68940 * locals.var_vginr_dn8)) / (2.0 * assign53890_e68947)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign53890_e68951;
        locals.var_temp__blk936_dn5 = assign53890_e68951_d_n5;
        locals.var_temp__blk936_dn6 = assign53890_e68951_d_n6;
        locals.var_temp__blk936_dn7 = assign53890_e68951_d_n7;
        locals.var_temp__blk936_dn8 = assign53890_e68951_d_n8;
        locals.var_temp__blk936_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_52(
        locals: &mut StampLocals,
    ) {
        let (assign53900_e68965, assign53900_e68965_d_n5, assign53900_e68965_d_n6, assign53900_e68965_d_n7, assign53900_e68965_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53900_e68958: f64 = (2.0 * locals.var_temp__blk936);
        let assign53900_e68960: f64 = (assign53900_e68958 - locals.var_vinr_max);
        let assign53900_e68962: f64 = (assign53900_e68960 - locals.var_vginr);
        let assign53900_e68963: f64 = (locals.var_temp__blk936 * assign53900_e68962);
        (assign53900_e68963, ((locals.var_temp__blk936_dn5 * assign53900_e68962) + (locals.var_temp__blk936 * ((2.0 * locals.var_temp__blk936_dn5) - locals.var_vginr_dn5))), ((locals.var_temp__blk936_dn6 * assign53900_e68962) + (locals.var_temp__blk936 * ((2.0 * locals.var_temp__blk936_dn6) - locals.var_vginr_dn6))), ((locals.var_temp__blk936_dn7 * assign53900_e68962) + (locals.var_temp__blk936 * ((2.0 * locals.var_temp__blk936_dn7) - locals.var_vginr_dn7))), ((locals.var_temp__blk936_dn8 * assign53900_e68962) + (locals.var_temp__blk936 * ((2.0 * locals.var_temp__blk936_dn8) - locals.var_vginr_dn8))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign53900_e68965;
        locals.var_temp1_dn5 = assign53900_e68965_d_n5;
        locals.var_temp1_dn6 = assign53900_e68965_d_n6;
        locals.var_temp1_dn7 = assign53900_e68965_d_n7;
        locals.var_temp1_dn8 = assign53900_e68965_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign53910_e68973, assign53910_e68973_d_n5, assign53910_e68973_d_n6, assign53910_e68973_d_n7, assign53910_e68973_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53910_e68971: f64 = (locals.var_vinr_max / locals.var_temp__blk936);
        (assign53910_e68971, (-((locals.var_vinr_max * locals.var_temp__blk936_dn5) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (-((locals.var_vinr_max * locals.var_temp__blk936_dn6) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (-((locals.var_vinr_max * locals.var_temp__blk936_dn7) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (-((locals.var_vinr_max * locals.var_temp__blk936_dn8) / (locals.var_temp__blk936 * locals.var_temp__blk936))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign53910_e68973;
        locals.var_temp2_dn5 = assign53910_e68973_d_n5;
        locals.var_temp2_dn6 = assign53910_e68973_d_n6;
        locals.var_temp2_dn7 = assign53910_e68973_d_n7;
        locals.var_temp2_dn8 = assign53910_e68973_d_n8;
        locals.var_temp2_rv = 0.0;

        let (assign53920_e68981, assign53920_e68981_d_n5, assign53920_e68981_d_n6, assign53920_e68981_d_n7, assign53920_e68981_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53920_e68979: f64 = (locals.var_vginr * locals.var_temp2);
        (assign53920_e68979, ((locals.var_vginr_dn5 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn5)), ((locals.var_vginr_dn6 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn6)), ((locals.var_vginr_dn7 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn7)), ((locals.var_vginr_dn8 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn8)),)
    } else {
        (locals.var_vginreff, locals.var_vginreff_dn5, locals.var_vginreff_dn6, locals.var_vginreff_dn7, locals.var_vginreff_dn8,)
    }
};
        locals.var_vginreff = assign53920_e68981;
        locals.var_vginreff_dn5 = assign53920_e68981_d_n5;
        locals.var_vginreff_dn6 = assign53920_e68981_d_n6;
        locals.var_vginreff_dn7 = assign53920_e68981_d_n7;
        locals.var_vginreff_dn8 = assign53920_e68981_d_n8;
        locals.var_vginreff_rv = 0.0;

        let (assign53930_e68992, assign53930_e68992_d_n5, assign53930_e68992_d_n6, assign53930_e68992_d_n7, assign53930_e68992_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53930_e68988: f64 = (locals.var_vginreff * locals.var_fcinracc_i);
        let assign53930_e68989: f64 = (1.0 - assign53930_e68988);
        let assign53930_e68990: f64 = (assign53930_e68989).sqrt();
        (assign53930_e68990, ((-(locals.var_vginreff_dn5 * locals.var_fcinracc_i)) / (2.0 * assign53930_e68990)), ((-(locals.var_vginreff_dn6 * locals.var_fcinracc_i)) / (2.0 * assign53930_e68990)), ((-(locals.var_vginreff_dn7 * locals.var_fcinracc_i)) / (2.0 * assign53930_e68990)), ((-(locals.var_vginreff_dn8 * locals.var_fcinracc_i)) / (2.0 * assign53930_e68990)),)
    } else {
        (locals.var_fqinr, locals.var_fqinr_dn5, locals.var_fqinr_dn6, locals.var_fqinr_dn7, locals.var_fqinr_dn8,)
    }
};
        locals.var_fqinr = assign53930_e68992;
        locals.var_fqinr_dn5 = assign53930_e68992_d_n5;
        locals.var_fqinr_dn6 = assign53930_e68992_d_n6;
        locals.var_fqinr_dn7 = assign53930_e68992_d_n7;
        locals.var_fqinr_dn8 = assign53930_e68992_d_n8;
        locals.var_fqinr_rv = 0.0;

        let (assign53940_e69006, assign53940_e69006_d_n5, assign53940_e69006_d_n6, assign53940_e69006_d_n7, assign53940_e69006_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53940_e68998: f64 = (1.0 - locals.var_fqinr);
        let assign53940_e69000: f64 = (assign53940_e68998 / locals.var_fcinracc_i);
        let assign53940_e69002: f64 = (assign53940_e69000 + locals.var_vginr);
        let assign53940_e69004: f64 = (assign53940_e69002 - locals.var_vginreff);
        (assign53940_e69004, ((((-locals.var_fqinr_dn5) / locals.var_fcinracc_i) + locals.var_vginr_dn5) - locals.var_vginreff_dn5), ((((-locals.var_fqinr_dn6) / locals.var_fcinracc_i) + locals.var_vginr_dn6) - locals.var_vginreff_dn6), ((((-locals.var_fqinr_dn7) / locals.var_fcinracc_i) + locals.var_vginr_dn7) - locals.var_vginreff_dn7), ((((-locals.var_fqinr_dn8) / locals.var_fcinracc_i) + locals.var_vginr_dn8) - locals.var_vginreff_dn8),)
    } else {
        (locals.var_dvinracc, locals.var_dvinracc_dn5, locals.var_dvinracc_dn6, locals.var_dvinracc_dn7, locals.var_dvinracc_dn8,)
    }
};
        locals.var_dvinracc = assign53940_e69006;
        locals.var_dvinracc_dn5 = assign53940_e69006_d_n5;
        locals.var_dvinracc_dn6 = assign53940_e69006_d_n6;
        locals.var_dvinracc_dn7 = assign53940_e69006_d_n7;
        locals.var_dvinracc_dn8 = assign53940_e69006_d_n8;
        locals.var_dvinracc_rv = 0.0;

        let (assign53950_e69030, assign53950_e69030_d_n5, assign53950_e69030_d_n6, assign53950_e69030_d_n7, assign53950_e69030_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53950_e69012: f64 = (0.5 / locals.var_fqinr);
        let assign53950_e69014: f64 = (assign53950_e69012 - 1.0);
        let assign53950_e69019: f64 = (locals.var_vinr_max - locals.var_temp__blk936);
        let assign53950_e69020: f64 = (locals.var_vginr * assign53950_e69019);
        let assign53950_e69021: f64 = (locals.var_temp1 + assign53950_e69020);
        let assign53950_e69022: f64 = (assign53950_e69014 * assign53950_e69021);
        let assign53950_e69024: f64 = (assign53950_e69022 * locals.var_temp2);
        let assign53950_e69026: f64 = (assign53950_e69024 / locals.var_temp1);
        let assign53950_e69028: f64 = (assign53950_e69026 + 1.0);
        (assign53950_e69028, ((((((((-((0.5 * locals.var_fqinr_dn5) / (locals.var_fqinr * locals.var_fqinr))) * assign53950_e69021) + (assign53950_e69014 * (locals.var_temp1_dn5 + ((locals.var_vginr_dn5 * assign53950_e69019) + (locals.var_vginr * (-locals.var_temp__blk936_dn5)))))) * locals.var_temp2) + (assign53950_e69022 * locals.var_temp2_dn5)) * locals.var_temp1) - (assign53950_e69024 * locals.var_temp1_dn5)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn6) / (locals.var_fqinr * locals.var_fqinr))) * assign53950_e69021) + (assign53950_e69014 * (locals.var_temp1_dn6 + ((locals.var_vginr_dn6 * assign53950_e69019) + (locals.var_vginr * (-locals.var_temp__blk936_dn6)))))) * locals.var_temp2) + (assign53950_e69022 * locals.var_temp2_dn6)) * locals.var_temp1) - (assign53950_e69024 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn7) / (locals.var_fqinr * locals.var_fqinr))) * assign53950_e69021) + (assign53950_e69014 * (locals.var_temp1_dn7 + ((locals.var_vginr_dn7 * assign53950_e69019) + (locals.var_vginr * (-locals.var_temp__blk936_dn7)))))) * locals.var_temp2) + (assign53950_e69022 * locals.var_temp2_dn7)) * locals.var_temp1) - (assign53950_e69024 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn8) / (locals.var_fqinr * locals.var_fqinr))) * assign53950_e69021) + (assign53950_e69014 * (locals.var_temp1_dn8 + ((locals.var_vginr_dn8 * assign53950_e69019) + (locals.var_vginr * (-locals.var_temp__blk936_dn8)))))) * locals.var_temp2) + (assign53950_e69022 * locals.var_temp2_dn8)) * locals.var_temp1) - (assign53950_e69024 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)),)
    } else {
        (locals.var_finracc, locals.var_finracc_dn5, locals.var_finracc_dn6, locals.var_finracc_dn7, locals.var_finracc_dn8,)
    }
};
        locals.var_finracc = assign53950_e69030;
        locals.var_finracc_dn5 = assign53950_e69030_d_n5;
        locals.var_finracc_dn6 = assign53950_e69030_d_n6;
        locals.var_finracc_dn7 = assign53950_e69030_d_n7;
        locals.var_finracc_dn8 = assign53950_e69030_d_n8;
        locals.var_finracc_rv = 0.0;

        let (assign53960_e69034, assign53960_e69034_d_n5, assign53960_e69034_d_n6, assign53960_e69034_d_n7, assign53960_e69034_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_finrdep, locals.var_finrdep_dn5, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8,)
    }
};
        locals.var_finrdep = assign53960_e69034;
        locals.var_finrdep_dn5 = assign53960_e69034_d_n5;
        locals.var_finrdep_dn6 = assign53960_e69034_d_n6;
        locals.var_finrdep_dn7 = assign53960_e69034_d_n7;
        locals.var_finrdep_dn8 = assign53960_e69034_d_n8;
        locals.var_finrdep_rv = 0.0;

        let (assign53970_e69038, assign53970_e69038_d_n5, assign53970_e69038_d_n6, assign53970_e69038_d_n7, assign53970_e69038_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvinrdep, locals.var_dvinrdep_dn5, locals.var_dvinrdep_dn6, locals.var_dvinrdep_dn7, locals.var_dvinrdep_dn8,)
    }
};
        locals.var_dvinrdep = assign53970_e69038;
        locals.var_dvinrdep_dn5 = assign53970_e69038_d_n5;
        locals.var_dvinrdep_dn6 = assign53970_e69038_d_n6;
        locals.var_dvinrdep_dn7 = assign53970_e69038_d_n7;
        locals.var_dvinrdep_dn8 = assign53970_e69038_d_n8;
        locals.var_dvinrdep_rv = 0.0;

        let assign53980_e69041: f64 = if locals.var_fcinrdep_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1512 = assign53980_e69041;
        locals.var_guard1512_rv = 0.0;

        let (assign53990_e69057, assign53990_e69057_d_n5, assign53990_e69057_d_n6, assign53990_e69057_d_n7, assign53990_e69057_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) {
        let assign53990_e69047: f64 = (0.5 * locals.var_phib_ac);
        let assign53990_e69052: f64 = (locals.var_gf_ac * 0.7071067811865475);
        let assign53990_e69053: f64 = (1.0 + assign53990_e69052);
        let assign53990_e69054: f64 = (locals.var_phit1_ac * assign53990_e69053);
        let assign53990_e69055: f64 = (assign53990_e69047 + assign53990_e69054);
        (assign53990_e69055, ((locals.var_phit1_ac_dn5 * assign53990_e69053) + (locals.var_phit1_ac * (locals.var_gf_ac_dn5 * 0.7071067811865475))), ((locals.var_phit1_ac_dn6 * assign53990_e69053) + (locals.var_phit1_ac * (locals.var_gf_ac_dn6 * 0.7071067811865475))), ((locals.var_phit1_ac_dn7 * assign53990_e69053) + (locals.var_phit1_ac * (locals.var_gf_ac_dn7 * 0.7071067811865475))), ((locals.var_phit1_ac_dn8 * assign53990_e69053) + (locals.var_phit1_ac * (locals.var_gf_ac_dn8 * 0.7071067811865475))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign53990_e69057;
        locals.var_temp__blk936_dn5 = assign53990_e69057_d_n5;
        locals.var_temp__blk936_dn6 = assign53990_e69057_d_n6;
        locals.var_temp__blk936_dn7 = assign53990_e69057_d_n7;
        locals.var_temp__blk936_dn8 = assign53990_e69057_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let (assign54000_e69065, assign54000_e69065_d_n5, assign54000_e69065_d_n6, assign54000_e69065_d_n7, assign54000_e69065_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) {
        let assign54000_e69063: f64 = (locals.var_vgb1_ac / locals.var_temp__blk936);
        (assign54000_e69063, (((locals.var_vgb1_ac_dn5 * locals.var_temp__blk936) - (locals.var_vgb1_ac * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_vgb1_ac_dn6 * locals.var_temp__blk936) - (locals.var_vgb1_ac * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_vgb1_ac_dn7 * locals.var_temp__blk936) - (locals.var_vgb1_ac * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_vgb1_ac_dn8 * locals.var_temp__blk936) - (locals.var_vgb1_ac * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)),)
    } else {
        (locals.var_xginrdep, locals.var_xginrdep_dn5, locals.var_xginrdep_dn6, locals.var_xginrdep_dn7, locals.var_xginrdep_dn8,)
    }
};
        locals.var_xginrdep = assign54000_e69065;
        locals.var_xginrdep_dn5 = assign54000_e69065_d_n5;
        locals.var_xginrdep_dn6 = assign54000_e69065_d_n6;
        locals.var_xginrdep_dn7 = assign54000_e69065_d_n7;
        locals.var_xginrdep_dn8 = assign54000_e69065_d_n8;
        locals.var_xginrdep_rv = 0.0;

        let assign54010_e69067: f64 = (locals.var_xginrdep).abs();
        let assign54010_e69069: f64 = if assign54010_e69067 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1513 = assign54010_e69069;
        locals.var_guard1513_rv = 0.0;

        let (assign54020_e69083, assign54020_e69083_d_n5, assign54020_e69083_d_n6, assign54020_e69083_d_n7, assign54020_e69083_d_n8,) = {
    if (((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign54020_e69078: f64 = (-locals.var_xginrdep);
        let assign54020_e69079: f64 = (assign54020_e69078).exp();
        let assign54020_e69080: f64 = (1.0 + assign54020_e69079);
        let assign54020_e69081: f64 = (1.0 / assign54020_e69080);
        (assign54020_e69081, (-((assign54020_e69079 * (-locals.var_xginrdep_dn5)) / (assign54020_e69080 * assign54020_e69080))), (-((assign54020_e69079 * (-locals.var_xginrdep_dn6)) / (assign54020_e69080 * assign54020_e69080))), (-((assign54020_e69079 * (-locals.var_xginrdep_dn7)) / (assign54020_e69080 * assign54020_e69080))), (-((assign54020_e69079 * (-locals.var_xginrdep_dn8)) / (assign54020_e69080 * assign54020_e69080))),)
    } else {
        (locals.var_finrdep, locals.var_finrdep_dn5, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8,)
    }
};
        locals.var_finrdep = assign54020_e69083;
        locals.var_finrdep_dn5 = assign54020_e69083_d_n5;
        locals.var_finrdep_dn6 = assign54020_e69083_d_n6;
        locals.var_finrdep_dn7 = assign54020_e69083_d_n7;
        locals.var_finrdep_dn8 = assign54020_e69083_d_n8;
        locals.var_finrdep_rv = 0.0;

        let assign54030_e69086: f64 = if locals.var_xginrdep < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1514 = assign54030_e69086;
        locals.var_guard1514_rv = 0.0;

        let (assign54040_e69122, assign54040_e69122_d_n5, assign54040_e69122_d_n6, assign54040_e69122_d_n7, assign54040_e69122_d_n8,) = {
    if ((((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1514 != 0.0)) {
        let assign54040_e69098: f64 = (-230.25850929940458);
        let assign54040_e69100: f64 = (assign54040_e69098 + locals.var_xginrdep);
        let assign54040_e69104: f64 = (-230.25850929940458);
        let assign54040_e69106: f64 = (assign54040_e69104 + locals.var_xginrdep);
        let assign54040_e69109: f64 = (-230.25850929940458);
        let assign54040_e69111: f64 = (assign54040_e69109 + locals.var_xginrdep);
        let assign54040_e69113: f64 = (assign54040_e69111 * 0.3333333333333333);
        let assign54040_e69114: f64 = (1.0 + assign54040_e69113);
        let assign54040_e69115: f64 = (assign54040_e69106 * assign54040_e69114);
        let assign54040_e69116: f64 = (0.5 * assign54040_e69115);
        let assign54040_e69117: f64 = (1.0 + assign54040_e69116);
        let assign54040_e69118: f64 = (assign54040_e69100 * assign54040_e69117);
        let assign54040_e69119: f64 = (1.0 + assign54040_e69118);
        let assign54040_e69120: f64 = (1e-100 / assign54040_e69119);
        (assign54040_e69120, (-((1e-100 * ((locals.var_xginrdep_dn5 * assign54040_e69117) + (assign54040_e69100 * (0.5 * ((locals.var_xginrdep_dn5 * assign54040_e69114) + (assign54040_e69106 * (locals.var_xginrdep_dn5 * 0.3333333333333333))))))) / (assign54040_e69119 * assign54040_e69119))), (-((1e-100 * ((locals.var_xginrdep_dn6 * assign54040_e69117) + (assign54040_e69100 * (0.5 * ((locals.var_xginrdep_dn6 * assign54040_e69114) + (assign54040_e69106 * (locals.var_xginrdep_dn6 * 0.3333333333333333))))))) / (assign54040_e69119 * assign54040_e69119))), (-((1e-100 * ((locals.var_xginrdep_dn7 * assign54040_e69117) + (assign54040_e69100 * (0.5 * ((locals.var_xginrdep_dn7 * assign54040_e69114) + (assign54040_e69106 * (locals.var_xginrdep_dn7 * 0.3333333333333333))))))) / (assign54040_e69119 * assign54040_e69119))), (-((1e-100 * ((locals.var_xginrdep_dn8 * assign54040_e69117) + (assign54040_e69100 * (0.5 * ((locals.var_xginrdep_dn8 * assign54040_e69114) + (assign54040_e69106 * (locals.var_xginrdep_dn8 * 0.3333333333333333))))))) / (assign54040_e69119 * assign54040_e69119))),)
    } else {
        (locals.var_finrdep, locals.var_finrdep_dn5, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8,)
    }
};
        locals.var_finrdep = assign54040_e69122;
        locals.var_finrdep_dn5 = assign54040_e69122_d_n5;
        locals.var_finrdep_dn6 = assign54040_e69122_d_n6;
        locals.var_finrdep_dn7 = assign54040_e69122_d_n7;
        locals.var_finrdep_dn8 = assign54040_e69122_d_n8;
        locals.var_finrdep_rv = 0.0;

        let assign54050_e69125: f64 = if locals.var_xginrdep < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1515 = assign54050_e69125;
        locals.var_guard1515_rv = 0.0;

        let (assign54060_e69137, assign54060_e69137_d_n5, assign54060_e69137_d_n6, assign54060_e69137_d_n7, assign54060_e69137_d_n8,) = {
    if (((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign54060_e69133: f64 = (locals.var_xginrdep).exp();
        let assign54060_e69134: f64 = (1.0 + assign54060_e69133);
        let assign54060_e69135: f64 = (assign54060_e69134).ln();
        (assign54060_e69135, ((assign54060_e69133 * locals.var_xginrdep_dn5) / assign54060_e69134), ((assign54060_e69133 * locals.var_xginrdep_dn6) / assign54060_e69134), ((assign54060_e69133 * locals.var_xginrdep_dn7) / assign54060_e69134), ((assign54060_e69133 * locals.var_xginrdep_dn8) / assign54060_e69134),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54060_e69137;
        locals.var_temp1_dn5 = assign54060_e69137_d_n5;
        locals.var_temp1_dn6 = assign54060_e69137_d_n6;
        locals.var_temp1_dn7 = assign54060_e69137_d_n7;
        locals.var_temp1_dn8 = assign54060_e69137_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign54070_e69146, assign54070_e69146_d_n5, assign54070_e69146_d_n6, assign54070_e69146_d_n7, assign54070_e69146_d_n8,) = {
    if (((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1515 == 0.0)) {
        (locals.var_xginrdep, locals.var_xginrdep_dn5, locals.var_xginrdep_dn6, locals.var_xginrdep_dn7, locals.var_xginrdep_dn8,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54070_e69146;
        locals.var_temp1_dn5 = assign54070_e69146_d_n5;
        locals.var_temp1_dn6 = assign54070_e69146_d_n6;
        locals.var_temp1_dn7 = assign54070_e69146_d_n7;
        locals.var_temp1_dn8 = assign54070_e69146_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign54080_e69154, assign54080_e69154_d_n5, assign54080_e69154_d_n6, assign54080_e69154_d_n7, assign54080_e69154_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) {
        let assign54080_e69152: f64 = (locals.var_temp__blk936 * locals.var_temp1);
        (assign54080_e69152, ((locals.var_temp__blk936_dn5 * locals.var_temp1) + (locals.var_temp__blk936 * locals.var_temp1_dn5)), ((locals.var_temp__blk936_dn6 * locals.var_temp1) + (locals.var_temp__blk936 * locals.var_temp1_dn6)), ((locals.var_temp__blk936_dn7 * locals.var_temp1) + (locals.var_temp__blk936 * locals.var_temp1_dn7)), ((locals.var_temp__blk936_dn8 * locals.var_temp1) + (locals.var_temp__blk936 * locals.var_temp1_dn8)),)
    } else {
        (locals.var_dvinrdep, locals.var_dvinrdep_dn5, locals.var_dvinrdep_dn6, locals.var_dvinrdep_dn7, locals.var_dvinrdep_dn8,)
    }
};
        locals.var_dvinrdep = assign54080_e69154;
        locals.var_dvinrdep_dn5 = assign54080_e69154_d_n5;
        locals.var_dvinrdep_dn6 = assign54080_e69154_d_n6;
        locals.var_dvinrdep_dn7 = assign54080_e69154_d_n7;
        locals.var_dvinrdep_dn8 = assign54080_e69154_d_n8;
        locals.var_dvinrdep_rv = 0.0;

        let (assign54090_e69164, assign54090_e69164_d_n5, assign54090_e69164_d_n6, assign54090_e69164_d_n7, assign54090_e69164_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54090_e69159: f64 = (locals.var_finrdep - locals.var_finracc);
        let assign54090_e69160: f64 = (locals.var_fcinrdep_i * assign54090_e69159);
        let assign54090_e69162: f64 = (assign54090_e69160 + locals.var_finracc);
        (assign54090_e69162, ((locals.var_fcinrdep_i * (locals.var_finrdep_dn5 - locals.var_finracc_dn5)) + locals.var_finracc_dn5), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn6 - locals.var_finracc_dn6)) + locals.var_finracc_dn6), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn7 - locals.var_finracc_dn7)) + locals.var_finracc_dn7), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn8 - locals.var_finracc_dn8)) + locals.var_finracc_dn8),)
    } else {
        (locals.var_finr, locals.var_finr_dn5, locals.var_finr_dn6, locals.var_finr_dn7, locals.var_finr_dn8,)
    }
};
        locals.var_finr = assign54090_e69164;
        locals.var_finr_dn5 = assign54090_e69164_d_n5;
        locals.var_finr_dn6 = assign54090_e69164_d_n6;
        locals.var_finr_dn7 = assign54090_e69164_d_n7;
        locals.var_finr_dn8 = assign54090_e69164_d_n8;
        locals.var_finr_rv = 0.0;

        let (assign54100_e69174, assign54100_e69174_d_n5, assign54100_e69174_d_n6, assign54100_e69174_d_n7, assign54100_e69174_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54100_e69169: f64 = (locals.var_dvinrdep - locals.var_dvinracc);
        let assign54100_e69170: f64 = (locals.var_fcinrdep_i * assign54100_e69169);
        let assign54100_e69172: f64 = (assign54100_e69170 + locals.var_dvinracc);
        (assign54100_e69172, ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn5 - locals.var_dvinracc_dn5)) + locals.var_dvinracc_dn5), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn6 - locals.var_dvinracc_dn6)) + locals.var_dvinracc_dn6), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn7 - locals.var_dvinracc_dn7)) + locals.var_dvinracc_dn7), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn8 - locals.var_dvinracc_dn8)) + locals.var_dvinracc_dn8),)
    } else {
        (locals.var_dvinr, locals.var_dvinr_dn5, locals.var_dvinr_dn6, locals.var_dvinr_dn7, locals.var_dvinr_dn8,)
    }
};
        locals.var_dvinr = assign54100_e69174;
        locals.var_dvinr_dn5 = assign54100_e69174_d_n5;
        locals.var_dvinr_dn6 = assign54100_e69174_d_n6;
        locals.var_dvinr_dn7 = assign54100_e69174_d_n7;
        locals.var_dvinr_dn8 = assign54100_e69174_d_n8;
        locals.var_dvinr_rv = 0.0;

        let (assign54110_e69188, assign54110_e69188_d_n5, assign54110_e69188_d_n6, assign54110_e69188_d_n7, assign54110_e69188_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54110_e69179: f64 = (locals.var_phit1_ac * locals.var_xno_s_ac);
        let assign54110_e69180: f64 = (locals.var_vgb1_ac - assign54110_e69179);
        let assign54110_e69182: f64 = (assign54110_e69180 - locals.var_voxm_ac);
        let assign54110_e69185: f64 = (0.5 * locals.var_dps_ac);
        let assign54110_e69186: f64 = (assign54110_e69182 - assign54110_e69185);
        (assign54110_e69186, (((locals.var_vgb1_ac_dn5 - ((locals.var_phit1_ac_dn5 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn5))) - locals.var_voxm_ac_dn5) - (0.5 * locals.var_dps_ac_dn5)), (((locals.var_vgb1_ac_dn6 - ((locals.var_phit1_ac_dn6 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn6))) - locals.var_voxm_ac_dn6) - (0.5 * locals.var_dps_ac_dn6)), (((locals.var_vgb1_ac_dn7 - ((locals.var_phit1_ac_dn7 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn7))) - locals.var_voxm_ac_dn7) - (0.5 * locals.var_dps_ac_dn7)), (((locals.var_vgb1_ac_dn8 - ((locals.var_phit1_ac_dn8 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn8))) - locals.var_voxm_ac_dn8) - (0.5 * locals.var_dps_ac_dn8)),)
    } else {
        (locals.var_vgsinr, locals.var_vgsinr_dn5, locals.var_vgsinr_dn6, locals.var_vgsinr_dn7, locals.var_vgsinr_dn8,)
    }
};
        locals.var_vgsinr = assign54110_e69188;
        locals.var_vgsinr_dn5 = assign54110_e69188_d_n5;
        locals.var_vgsinr_dn6 = assign54110_e69188_d_n6;
        locals.var_vgsinr_dn7 = assign54110_e69188_d_n7;
        locals.var_vgsinr_dn8 = assign54110_e69188_d_n8;
        locals.var_vgsinr_rv = 0.0;

        let (assign54120_e69196, assign54120_e69196_d_n5, assign54120_e69196_d_n6, assign54120_e69196_d_n7, assign54120_e69196_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54120_e69192: f64 = (locals.var_vgb1_ac - locals.var_vgsinr);
        let assign54120_e69194: f64 = (assign54120_e69192 - locals.var_qbs_ac);
        (assign54120_e69194, ((locals.var_vgb1_ac_dn5 - locals.var_vgsinr_dn5) - locals.var_qbs_ac_dn5), ((locals.var_vgb1_ac_dn6 - locals.var_vgsinr_dn6) - locals.var_qbs_ac_dn6), ((locals.var_vgb1_ac_dn7 - locals.var_vgsinr_dn7) - locals.var_qbs_ac_dn7), ((locals.var_vgb1_ac_dn8 - locals.var_vgsinr_dn8) - locals.var_qbs_ac_dn8),)
    } else {
        (locals.var_vsginr, locals.var_vsginr_dn5, locals.var_vsginr_dn6, locals.var_vsginr_dn7, locals.var_vsginr_dn8,)
    }
};
        locals.var_vsginr = assign54120_e69196;
        locals.var_vsginr_dn5 = assign54120_e69196_d_n5;
        locals.var_vsginr_dn6 = assign54120_e69196_d_n6;
        locals.var_vsginr_dn7 = assign54120_e69196_d_n7;
        locals.var_vsginr_dn8 = assign54120_e69196_d_n8;
        locals.var_vsginr_rv = 0.0;

        let (assign54130_e69204, assign54130_e69204_d_n5, assign54130_e69204_d_n6, assign54130_e69204_d_n7, assign54130_e69204_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54130_e69200: f64 = (locals.var_dps_ac + locals.var_vgsinr);
        let assign54130_e69202: f64 = (assign54130_e69200 - locals.var_v_ds);
        (assign54130_e69202, (locals.var_dps_ac_dn5 + locals.var_vgsinr_dn5), ((locals.var_dps_ac_dn6 + locals.var_vgsinr_dn6) - locals.var_v_ds_dn6), ((locals.var_dps_ac_dn7 + locals.var_vgsinr_dn7) - locals.var_v_ds_dn7), (locals.var_dps_ac_dn8 + locals.var_vgsinr_dn8),)
    } else {
        (locals.var_vgdinr, locals.var_vgdinr_dn5, locals.var_vgdinr_dn6, locals.var_vgdinr_dn7, locals.var_vgdinr_dn8,)
    }
};
        locals.var_vgdinr = assign54130_e69204;
        locals.var_vgdinr_dn5 = assign54130_e69204_d_n5;
        locals.var_vgdinr_dn6 = assign54130_e69204_d_n6;
        locals.var_vgdinr_dn7 = assign54130_e69204_d_n7;
        locals.var_vgdinr_dn8 = assign54130_e69204_d_n8;
        locals.var_vgdinr_rv = 0.0;

        let (assign54140_e69212, assign54140_e69212_d_n5, assign54140_e69212_d_n6, assign54140_e69212_d_n7, assign54140_e69212_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54140_e69208: f64 = (locals.var_vgb1_ac - locals.var_vgdinr);
        let assign54140_e69210: f64 = (assign54140_e69208 - locals.var_qbd_ac);
        (assign54140_e69210, ((locals.var_vgb1_ac_dn5 - locals.var_vgdinr_dn5) - locals.var_qbd_ac_dn5), ((locals.var_vgb1_ac_dn6 - locals.var_vgdinr_dn6) - locals.var_qbd_ac_dn6), ((locals.var_vgb1_ac_dn7 - locals.var_vgdinr_dn7) - locals.var_qbd_ac_dn7), ((locals.var_vgb1_ac_dn8 - locals.var_vgdinr_dn8) - locals.var_qbd_ac_dn8),)
    } else {
        (locals.var_vdginr, locals.var_vdginr_dn5, locals.var_vdginr_dn6, locals.var_vdginr_dn7, locals.var_vdginr_dn8,)
    }
};
        locals.var_vdginr = assign54140_e69212;
        locals.var_vdginr_dn5 = assign54140_e69212_d_n5;
        locals.var_vdginr_dn6 = assign54140_e69212_d_n6;
        locals.var_vdginr_dn7 = assign54140_e69212_d_n7;
        locals.var_vdginr_dn8 = assign54140_e69212_d_n8;
        locals.var_vdginr_rv = 0.0;

        let assign54150_e69215: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1516 = assign54150_e69215;
        locals.var_guard1516_rv = 0.0;

        let (assign54160_e69229, assign54160_e69229_d_n5, assign54160_e69229_d_n6, assign54160_e69229_d_n7, assign54160_e69229_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 != 0.0)) {
        let assign54160_e69222: f64 = (locals.var_cinrd_i * locals.var_vgdinr);
        let assign54160_e69225: f64 = (locals.var_cinr_i * locals.var_vgsinr);
        let assign54160_e69226: f64 = (assign54160_e69222 + assign54160_e69225);
        let assign54160_e69227: f64 = (locals.var_finr * assign54160_e69226);
        (assign54160_e69227, ((locals.var_finr_dn5 * assign54160_e69226) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn5) + (locals.var_cinr_i * locals.var_vgsinr_dn5)))), ((locals.var_finr_dn6 * assign54160_e69226) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn6) + (locals.var_cinr_i * locals.var_vgsinr_dn6)))), ((locals.var_finr_dn7 * assign54160_e69226) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn7) + (locals.var_cinr_i * locals.var_vgsinr_dn7)))), ((locals.var_finr_dn8 * assign54160_e69226) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn8) + (locals.var_cinr_i * locals.var_vgsinr_dn8)))),)
    } else {
        (locals.var_qginr, locals.var_qginr_dn5, locals.var_qginr_dn6, locals.var_qginr_dn7, locals.var_qginr_dn8,)
    }
};
        locals.var_qginr = assign54160_e69229;
        locals.var_qginr_dn5 = assign54160_e69229_d_n5;
        locals.var_qginr_dn6 = assign54160_e69229_d_n6;
        locals.var_qginr_dn7 = assign54160_e69229_d_n7;
        locals.var_qginr_dn8 = assign54160_e69229_d_n8;
        locals.var_qginr_rv = 0.0;

        let (assign54170_e69239, assign54170_e69239_d_n5, assign54170_e69239_d_n6, assign54170_e69239_d_n7, assign54170_e69239_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 != 0.0)) {
        let assign54170_e69236: f64 = (locals.var_vsginr - locals.var_dvinr);
        let assign54170_e69237: f64 = (locals.var_cinr_i * assign54170_e69236);
        (assign54170_e69237, (locals.var_cinr_i * (locals.var_vsginr_dn5 - locals.var_dvinr_dn5)), (locals.var_cinr_i * (locals.var_vsginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinr_i * (locals.var_vsginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinr_i * (locals.var_vsginr_dn8 - locals.var_dvinr_dn8)),)
    } else {
        (locals.var_qsinr, locals.var_qsinr_dn5, locals.var_qsinr_dn6, locals.var_qsinr_dn7, locals.var_qsinr_dn8,)
    }
};
        locals.var_qsinr = assign54170_e69239;
        locals.var_qsinr_dn5 = assign54170_e69239_d_n5;
        locals.var_qsinr_dn6 = assign54170_e69239_d_n6;
        locals.var_qsinr_dn7 = assign54170_e69239_d_n7;
        locals.var_qsinr_dn8 = assign54170_e69239_d_n8;
        locals.var_qsinr_rv = 0.0;

        let (assign54180_e69249, assign54180_e69249_d_n5, assign54180_e69249_d_n6, assign54180_e69249_d_n7, assign54180_e69249_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 != 0.0)) {
        let assign54180_e69246: f64 = (locals.var_vdginr - locals.var_dvinr);
        let assign54180_e69247: f64 = (locals.var_cinrd_i * assign54180_e69246);
        (assign54180_e69247, (locals.var_cinrd_i * (locals.var_vdginr_dn5 - locals.var_dvinr_dn5)), (locals.var_cinrd_i * (locals.var_vdginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinrd_i * (locals.var_vdginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinrd_i * (locals.var_vdginr_dn8 - locals.var_dvinr_dn8)),)
    } else {
        (locals.var_qdinr, locals.var_qdinr_dn5, locals.var_qdinr_dn6, locals.var_qdinr_dn7, locals.var_qdinr_dn8,)
    }
};
        locals.var_qdinr = assign54180_e69249;
        locals.var_qdinr_dn5 = assign54180_e69249_d_n5;
        locals.var_qdinr_dn6 = assign54180_e69249_d_n6;
        locals.var_qdinr_dn7 = assign54180_e69249_d_n7;
        locals.var_qdinr_dn8 = assign54180_e69249_d_n8;
        locals.var_qdinr_rv = 0.0;

        let (assign54190_e69264, assign54190_e69264_d_n5, assign54190_e69264_d_n6, assign54190_e69264_d_n7, assign54190_e69264_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 == 0.0)) {
        let assign54190_e69257: f64 = (locals.var_cinr_i * locals.var_vgdinr);
        let assign54190_e69260: f64 = (locals.var_cinrd_i * locals.var_vgsinr);
        let assign54190_e69261: f64 = (assign54190_e69257 + assign54190_e69260);
        let assign54190_e69262: f64 = (locals.var_finr * assign54190_e69261);
        (assign54190_e69262, ((locals.var_finr_dn5 * assign54190_e69261) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn5) + (locals.var_cinrd_i * locals.var_vgsinr_dn5)))), ((locals.var_finr_dn6 * assign54190_e69261) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn6) + (locals.var_cinrd_i * locals.var_vgsinr_dn6)))), ((locals.var_finr_dn7 * assign54190_e69261) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn7) + (locals.var_cinrd_i * locals.var_vgsinr_dn7)))), ((locals.var_finr_dn8 * assign54190_e69261) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn8) + (locals.var_cinrd_i * locals.var_vgsinr_dn8)))),)
    } else {
        (locals.var_qginr, locals.var_qginr_dn5, locals.var_qginr_dn6, locals.var_qginr_dn7, locals.var_qginr_dn8,)
    }
};
        locals.var_qginr = assign54190_e69264;
        locals.var_qginr_dn5 = assign54190_e69264_d_n5;
        locals.var_qginr_dn6 = assign54190_e69264_d_n6;
        locals.var_qginr_dn7 = assign54190_e69264_d_n7;
        locals.var_qginr_dn8 = assign54190_e69264_d_n8;
        locals.var_qginr_rv = 0.0;

        let (assign54200_e69275, assign54200_e69275_d_n5, assign54200_e69275_d_n6, assign54200_e69275_d_n7, assign54200_e69275_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 == 0.0)) {
        let assign54200_e69272: f64 = (locals.var_vsginr - locals.var_dvinr);
        let assign54200_e69273: f64 = (locals.var_cinrd_i * assign54200_e69272);
        (assign54200_e69273, (locals.var_cinrd_i * (locals.var_vsginr_dn5 - locals.var_dvinr_dn5)), (locals.var_cinrd_i * (locals.var_vsginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinrd_i * (locals.var_vsginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinrd_i * (locals.var_vsginr_dn8 - locals.var_dvinr_dn8)),)
    } else {
        (locals.var_qsinr, locals.var_qsinr_dn5, locals.var_qsinr_dn6, locals.var_qsinr_dn7, locals.var_qsinr_dn8,)
    }
};
        locals.var_qsinr = assign54200_e69275;
        locals.var_qsinr_dn5 = assign54200_e69275_d_n5;
        locals.var_qsinr_dn6 = assign54200_e69275_d_n6;
        locals.var_qsinr_dn7 = assign54200_e69275_d_n7;
        locals.var_qsinr_dn8 = assign54200_e69275_d_n8;
        locals.var_qsinr_rv = 0.0;

        let (assign54210_e69286, assign54210_e69286_d_n5, assign54210_e69286_d_n6, assign54210_e69286_d_n7, assign54210_e69286_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 == 0.0)) {
        let assign54210_e69283: f64 = (locals.var_vdginr - locals.var_dvinr);
        let assign54210_e69284: f64 = (locals.var_cinr_i * assign54210_e69283);
        (assign54210_e69284, (locals.var_cinr_i * (locals.var_vdginr_dn5 - locals.var_dvinr_dn5)), (locals.var_cinr_i * (locals.var_vdginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinr_i * (locals.var_vdginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinr_i * (locals.var_vdginr_dn8 - locals.var_dvinr_dn8)),)
    } else {
        (locals.var_qdinr, locals.var_qdinr_dn5, locals.var_qdinr_dn6, locals.var_qdinr_dn7, locals.var_qdinr_dn8,)
    }
};
        locals.var_qdinr = assign54210_e69286;
        locals.var_qdinr_dn5 = assign54210_e69286_d_n5;
        locals.var_qdinr_dn6 = assign54210_e69286_d_n6;
        locals.var_qdinr_dn7 = assign54210_e69286_d_n7;
        locals.var_qdinr_dn8 = assign54210_e69286_d_n8;
        locals.var_qdinr_rv = 0.0;

        let (assign54220_e69292, assign54220_e69292_d_n5, assign54220_e69292_d_n6, assign54220_e69292_d_n7, assign54220_e69292_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54220_e69290: f64 = (locals.var_qg + locals.var_qginr);
        (assign54220_e69290, (locals.var_qg_dn5 + locals.var_qginr_dn5), (locals.var_qg_dn6 + locals.var_qginr_dn6), (locals.var_qg_dn7 + locals.var_qginr_dn7), (locals.var_qg_dn8 + locals.var_qginr_dn8),)
    } else {
        (locals.var_qg, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8,)
    }
};
        locals.var_qg = assign54220_e69292;
        locals.var_qg_dn5 = assign54220_e69292_d_n5;
        locals.var_qg_dn6 = assign54220_e69292_d_n6;
        locals.var_qg_dn7 = assign54220_e69292_d_n7;
        locals.var_qg_dn8 = assign54220_e69292_d_n8;
        locals.var_qg_rv = 0.0;

        let (assign54230_e69298, assign54230_e69298_d_n5, assign54230_e69298_d_n6, assign54230_e69298_d_n7, assign54230_e69298_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54230_e69296: f64 = (locals.var_qd + locals.var_qdinr);
        (assign54230_e69296, (locals.var_qd_dn5 + locals.var_qdinr_dn5), (locals.var_qd_dn6 + locals.var_qdinr_dn6), (locals.var_qd_dn7 + locals.var_qdinr_dn7), (locals.var_qd_dn8 + locals.var_qdinr_dn8),)
    } else {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8,)
    }
};
        locals.var_qd = assign54230_e69298;
        locals.var_qd_dn5 = assign54230_e69298_d_n5;
        locals.var_qd_dn6 = assign54230_e69298_d_n6;
        locals.var_qd_dn7 = assign54230_e69298_d_n7;
        locals.var_qd_dn8 = assign54230_e69298_d_n8;
        locals.var_qd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_53(
        locals: &mut StampLocals,
    ) {
        let (assign54240_e69308, assign54240_e69308_d_n5, assign54240_e69308_d_n6, assign54240_e69308_d_n7, assign54240_e69308_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54240_e69302: f64 = (locals.var_qb - locals.var_qginr);
        let assign54240_e69304: f64 = (assign54240_e69302 - locals.var_qdinr);
        let assign54240_e69306: f64 = (assign54240_e69304 - locals.var_qsinr);
        (assign54240_e69306, (((locals.var_qb_dn5 - locals.var_qginr_dn5) - locals.var_qdinr_dn5) - locals.var_qsinr_dn5), (((locals.var_qb_dn6 - locals.var_qginr_dn6) - locals.var_qdinr_dn6) - locals.var_qsinr_dn6), (((locals.var_qb_dn7 - locals.var_qginr_dn7) - locals.var_qdinr_dn7) - locals.var_qsinr_dn7), (((locals.var_qb_dn8 - locals.var_qginr_dn8) - locals.var_qdinr_dn8) - locals.var_qsinr_dn8),)
    } else {
        (locals.var_qb, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8,)
    }
};
        locals.var_qb = assign54240_e69308;
        locals.var_qb_dn5 = assign54240_e69308_d_n5;
        locals.var_qb_dn6 = assign54240_e69308_d_n6;
        locals.var_qb_dn7 = assign54240_e69308_d_n7;
        locals.var_qb_dn8 = assign54240_e69308_d_n8;
        locals.var_qb_rv = 0.0;

        locals.var_qg_ov_s = 0.0;
        locals.var_qg_ov_s_dn5 = 0.0;
        locals.var_qg_ov_s_dn6 = 0.0;
        locals.var_qg_ov_s_dn7 = 0.0;
        locals.var_qg_ov_s_dn8 = 0.0;
        locals.var_qg_ov_s_rv = 0.0;

        locals.var_yb_ov_s = 0.0;
        locals.var_yb_ov_s_dn5 = 0.0;
        locals.var_yb_ov_s_dn6 = 0.0;
        locals.var_yb_ov_s_dn7 = 0.0;
        locals.var_yb_ov_s_dn8 = 0.0;
        locals.var_yb_ov_s_rv = 0.0;

        let assign54290_e69323: f64 = if ((locals.var_cgov_i > 0.0) && (locals.var_fcgovacc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1517 = assign54290_e69323;
        locals.var_guard1517_rv = 0.0;

        let (assign54300_e69333, assign54300_e69333_d_n5, assign54300_e69333_d_n6, assign54300_e69333_d_n7, assign54300_e69333_d_n8,) = {
    if (locals.var_guard1517 != 0.0) {
        let assign54300_e69328: f64 = (0.5 * locals.var_xgb_ov);
        let assign54300_e69330: f64 = (assign54300_e69328 + locals.var_dxgb_ov_s);
        let assign54300_e69331: f64 = (locals.var_cgovaccg_i * assign54300_e69330);
        (assign54300_e69331, (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn5)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn6)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn7)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign54300_e69333;
        locals.var_temp__blk936_dn5 = assign54300_e69333_d_n5;
        locals.var_temp__blk936_dn6 = assign54300_e69333_d_n6;
        locals.var_temp__blk936_dn7 = assign54300_e69333_d_n7;
        locals.var_temp__blk936_dn8 = assign54300_e69333_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign54310_e69336: f64 = if locals.var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1518 = assign54310_e69336;
        locals.var_guard1518_rv = 0.0;

        let assign54320_e69339: f64 = (-230.25850929940458);
        let assign54320_e69340: f64 = if locals.var_temp__blk936 > assign54320_e69339 { 1.0 } else { 0.0 };
        locals.var_guard1519 = assign54320_e69340;
        locals.var_guard1519_rv = 0.0;

        let (assign54330_e69349, assign54330_e69349_d_n5, assign54330_e69349_d_n6, assign54330_e69349_d_n7, assign54330_e69349_d_n8,) = {
    if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign54330_e69347: f64 = (locals.var_temp__blk936).exp();
        (assign54330_e69347, (assign54330_e69347 * locals.var_temp__blk936_dn5), (assign54330_e69347 * locals.var_temp__blk936_dn6), (assign54330_e69347 * locals.var_temp__blk936_dn7), (assign54330_e69347 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_yb_ov_s, locals.var_yb_ov_s_dn5, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8,)
    }
};
        locals.var_yb_ov_s = assign54330_e69349;
        locals.var_yb_ov_s_dn5 = assign54330_e69349_d_n5;
        locals.var_yb_ov_s_dn6 = assign54330_e69349_d_n6;
        locals.var_yb_ov_s_dn7 = assign54330_e69349_d_n7;
        locals.var_yb_ov_s_dn8 = assign54330_e69349_d_n8;
        locals.var_yb_ov_s_rv = 0.0;

        let (assign54340_e69383, assign54340_e69383_d_n5, assign54340_e69383_d_n6, assign54340_e69383_d_n7, assign54340_e69383_d_n8,) = {
    if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1519 == 0.0)) {
        let assign54340_e69359: f64 = (-230.25850929940458);
        let assign54340_e69361: f64 = (assign54340_e69359 - locals.var_temp__blk936);
        let assign54340_e69365: f64 = (-230.25850929940458);
        let assign54340_e69367: f64 = (assign54340_e69365 - locals.var_temp__blk936);
        let assign54340_e69370: f64 = (-230.25850929940458);
        let assign54340_e69372: f64 = (assign54340_e69370 - locals.var_temp__blk936);
        let assign54340_e69374: f64 = (assign54340_e69372 * 0.3333333333333333);
        let assign54340_e69375: f64 = (1.0 + assign54340_e69374);
        let assign54340_e69376: f64 = (assign54340_e69367 * assign54340_e69375);
        let assign54340_e69377: f64 = (0.5 * assign54340_e69376);
        let assign54340_e69378: f64 = (1.0 + assign54340_e69377);
        let assign54340_e69379: f64 = (assign54340_e69361 * assign54340_e69378);
        let assign54340_e69380: f64 = (1.0 + assign54340_e69379);
        let assign54340_e69381: f64 = (1e-100 / assign54340_e69380);
        (assign54340_e69381, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign54340_e69378) + (assign54340_e69361 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign54340_e69375) + (assign54340_e69367 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign54340_e69380 * assign54340_e69380))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign54340_e69378) + (assign54340_e69361 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign54340_e69375) + (assign54340_e69367 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign54340_e69380 * assign54340_e69380))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign54340_e69378) + (assign54340_e69361 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign54340_e69375) + (assign54340_e69367 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign54340_e69380 * assign54340_e69380))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign54340_e69378) + (assign54340_e69361 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign54340_e69375) + (assign54340_e69367 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign54340_e69380 * assign54340_e69380))),)
    } else {
        (locals.var_yb_ov_s, locals.var_yb_ov_s_dn5, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8,)
    }
};
        locals.var_yb_ov_s = assign54340_e69383;
        locals.var_yb_ov_s_dn5 = assign54340_e69383_d_n5;
        locals.var_yb_ov_s_dn6 = assign54340_e69383_d_n6;
        locals.var_yb_ov_s_dn7 = assign54340_e69383_d_n7;
        locals.var_yb_ov_s_dn8 = assign54340_e69383_d_n8;
        locals.var_yb_ov_s_rv = 0.0;

        let assign54350_e69386: f64 = if locals.var_yb_ov_s > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1520 = assign54350_e69386;
        locals.var_guard1520_rv = 0.0;

        let (assign54360_e69397, assign54360_e69397_d_n5, assign54360_e69397_d_n6, assign54360_e69397_d_n7, assign54360_e69397_d_n8,) = {
    if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1520 != 0.0)) {
        let assign54360_e69394: f64 = (1.0 + locals.var_yb_ov_s);
        let assign54360_e69395: f64 = (assign54360_e69394).ln();
        (assign54360_e69395, (locals.var_yb_ov_s_dn5 / assign54360_e69394), (locals.var_yb_ov_s_dn6 / assign54360_e69394), (locals.var_yb_ov_s_dn7 / assign54360_e69394), (locals.var_yb_ov_s_dn8 / assign54360_e69394),)
    } else {
        (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn5, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8,)
    }
};
        locals.var_xgbeff_ov_s = assign54360_e69397;
        locals.var_xgbeff_ov_s_dn5 = assign54360_e69397_d_n5;
        locals.var_xgbeff_ov_s_dn6 = assign54360_e69397_d_n6;
        locals.var_xgbeff_ov_s_dn7 = assign54360_e69397_d_n7;
        locals.var_xgbeff_ov_s_dn8 = assign54360_e69397_d_n8;
        locals.var_xgbeff_ov_s_rv = 0.0;

        let (assign54370_e69416, assign54370_e69416_d_n5, assign54370_e69416_d_n6, assign54370_e69416_d_n7, assign54370_e69416_d_n8,) = {
    if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1520 != 0.0)) {
        let assign54370_e69407: f64 = (1.0 + locals.var_xgbeff_ov_s);
        let assign54370_e69408: f64 = (assign54370_e69407).ln();
        let assign54370_e69411: f64 = (2.0 + locals.var_xgbeff_ov_s);
        let assign54370_e69412: f64 = (assign54370_e69408 / assign54370_e69411);
        let assign54370_e69413: f64 = (1.0 - assign54370_e69412);
        let assign54370_e69414: f64 = (locals.var_xgbeff_ov_s * assign54370_e69413);
        (assign54370_e69414, ((locals.var_xgbeff_ov_s_dn5 * assign54370_e69413) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn5 / assign54370_e69407) * assign54370_e69411) - (assign54370_e69408 * locals.var_xgbeff_ov_s_dn5)) / (assign54370_e69411 * assign54370_e69411))))), ((locals.var_xgbeff_ov_s_dn6 * assign54370_e69413) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn6 / assign54370_e69407) * assign54370_e69411) - (assign54370_e69408 * locals.var_xgbeff_ov_s_dn6)) / (assign54370_e69411 * assign54370_e69411))))), ((locals.var_xgbeff_ov_s_dn7 * assign54370_e69413) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn7 / assign54370_e69407) * assign54370_e69411) - (assign54370_e69408 * locals.var_xgbeff_ov_s_dn7)) / (assign54370_e69411 * assign54370_e69411))))), ((locals.var_xgbeff_ov_s_dn8 * assign54370_e69413) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn8 / assign54370_e69407) * assign54370_e69411) - (assign54370_e69408 * locals.var_xgbeff_ov_s_dn8)) / (assign54370_e69411 * assign54370_e69411))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54370_e69416;
        locals.var_temp1_dn5 = assign54370_e69416_d_n5;
        locals.var_temp1_dn6 = assign54370_e69416_d_n6;
        locals.var_temp1_dn7 = assign54370_e69416_d_n7;
        locals.var_temp1_dn8 = assign54370_e69416_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign54380_e69425, assign54380_e69425_d_n5, assign54380_e69425_d_n6, assign54380_e69425_d_n7, assign54380_e69425_d_n8,) = {
    if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1520 == 0.0)) {
        (locals.var_yb_ov_s, locals.var_yb_ov_s_dn5, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8,)
    } else {
        (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn5, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8,)
    }
};
        locals.var_xgbeff_ov_s = assign54380_e69425;
        locals.var_xgbeff_ov_s_dn5 = assign54380_e69425_d_n5;
        locals.var_xgbeff_ov_s_dn6 = assign54380_e69425_d_n6;
        locals.var_xgbeff_ov_s_dn7 = assign54380_e69425_d_n7;
        locals.var_xgbeff_ov_s_dn8 = assign54380_e69425_d_n8;
        locals.var_xgbeff_ov_s_rv = 0.0;

        let (assign54390_e69440, assign54390_e69440_d_n5, assign54390_e69440_d_n6, assign54390_e69440_d_n7, assign54390_e69440_d_n8,) = {
    if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1520 == 0.0)) {
        let assign54390_e69434: f64 = (2.0 * locals.var_xgbeff_ov_s);
        let assign54390_e69437: f64 = (2.0 + locals.var_xgbeff_ov_s);
        let assign54390_e69438: f64 = (assign54390_e69434 / assign54390_e69437);
        (assign54390_e69438, ((((2.0 * locals.var_xgbeff_ov_s_dn5) * assign54390_e69437) - (assign54390_e69434 * locals.var_xgbeff_ov_s_dn5)) / (assign54390_e69437 * assign54390_e69437)), ((((2.0 * locals.var_xgbeff_ov_s_dn6) * assign54390_e69437) - (assign54390_e69434 * locals.var_xgbeff_ov_s_dn6)) / (assign54390_e69437 * assign54390_e69437)), ((((2.0 * locals.var_xgbeff_ov_s_dn7) * assign54390_e69437) - (assign54390_e69434 * locals.var_xgbeff_ov_s_dn7)) / (assign54390_e69437 * assign54390_e69437)), ((((2.0 * locals.var_xgbeff_ov_s_dn8) * assign54390_e69437) - (assign54390_e69434 * locals.var_xgbeff_ov_s_dn8)) / (assign54390_e69437 * assign54390_e69437)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54390_e69440;
        locals.var_temp1_dn5 = assign54390_e69440_d_n5;
        locals.var_temp1_dn6 = assign54390_e69440_d_n6;
        locals.var_temp1_dn7 = assign54390_e69440_d_n7;
        locals.var_temp1_dn8 = assign54390_e69440_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign54400_e69447, assign54400_e69447_d_n5, assign54400_e69447_d_n6, assign54400_e69447_d_n7, assign54400_e69447_d_n8,) = {
    if ((locals.var_guard1517 != 0.0) && (locals.var_guard1518 == 0.0)) {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn5, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8,)
    }
};
        locals.var_xgbeff_ov_s = assign54400_e69447;
        locals.var_xgbeff_ov_s_dn5 = assign54400_e69447_d_n5;
        locals.var_xgbeff_ov_s_dn6 = assign54400_e69447_d_n6;
        locals.var_xgbeff_ov_s_dn7 = assign54400_e69447_d_n7;
        locals.var_xgbeff_ov_s_dn8 = assign54400_e69447_d_n8;
        locals.var_xgbeff_ov_s_rv = 0.0;

        let (assign54410_e69465, assign54410_e69465_d_n5, assign54410_e69465_d_n6, assign54410_e69465_d_n7, assign54410_e69465_d_n8,) = {
    if ((locals.var_guard1517 != 0.0) && (locals.var_guard1518 == 0.0)) {
        let assign54410_e69456: f64 = (1.0 + locals.var_xgbeff_ov_s);
        let assign54410_e69457: f64 = (assign54410_e69456).ln();
        let assign54410_e69460: f64 = (2.0 + locals.var_xgbeff_ov_s);
        let assign54410_e69461: f64 = (assign54410_e69457 / assign54410_e69460);
        let assign54410_e69462: f64 = (1.0 - assign54410_e69461);
        let assign54410_e69463: f64 = (locals.var_xgbeff_ov_s * assign54410_e69462);
        (assign54410_e69463, ((locals.var_xgbeff_ov_s_dn5 * assign54410_e69462) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn5 / assign54410_e69456) * assign54410_e69460) - (assign54410_e69457 * locals.var_xgbeff_ov_s_dn5)) / (assign54410_e69460 * assign54410_e69460))))), ((locals.var_xgbeff_ov_s_dn6 * assign54410_e69462) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn6 / assign54410_e69456) * assign54410_e69460) - (assign54410_e69457 * locals.var_xgbeff_ov_s_dn6)) / (assign54410_e69460 * assign54410_e69460))))), ((locals.var_xgbeff_ov_s_dn7 * assign54410_e69462) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn7 / assign54410_e69456) * assign54410_e69460) - (assign54410_e69457 * locals.var_xgbeff_ov_s_dn7)) / (assign54410_e69460 * assign54410_e69460))))), ((locals.var_xgbeff_ov_s_dn8 * assign54410_e69462) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn8 / assign54410_e69456) * assign54410_e69460) - (assign54410_e69457 * locals.var_xgbeff_ov_s_dn8)) / (assign54410_e69460 * assign54410_e69460))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54410_e69465;
        locals.var_temp1_dn5 = assign54410_e69465_d_n5;
        locals.var_temp1_dn6 = assign54410_e69465_d_n6;
        locals.var_temp1_dn7 = assign54410_e69465_d_n7;
        locals.var_temp1_dn8 = assign54410_e69465_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign54420_e69480, assign54420_e69480_d_n5, assign54420_e69480_d_n6, assign54420_e69480_d_n7, assign54420_e69480_d_n8,) = {
    if (locals.var_guard1517 != 0.0) {
        let assign54420_e69468: f64 = (-2.0);
        let assign54420_e69470: f64 = (assign54420_e69468 * locals.var_fcgovacc_i);
        let assign54420_e69472: f64 = (assign54420_e69470 / locals.var_cgovaccg_i);
        let assign54420_e69474: f64 = (assign54420_e69472 * locals.var_cgov_i);
        let assign54420_e69476: f64 = (assign54420_e69474 * locals.var_phita);
        let assign54420_e69478: f64 = (assign54420_e69476 * locals.var_temp1);
        (assign54420_e69478, (assign54420_e69476 * locals.var_temp1_dn5), (assign54420_e69476 * locals.var_temp1_dn6), (assign54420_e69476 * locals.var_temp1_dn7), (assign54420_e69476 * locals.var_temp1_dn8),)
    } else {
        (locals.var_qg_ov_s, locals.var_qg_ov_s_dn5, locals.var_qg_ov_s_dn6, locals.var_qg_ov_s_dn7, locals.var_qg_ov_s_dn8,)
    }
};
        locals.var_qg_ov_s = assign54420_e69480;
        locals.var_qg_ov_s_dn5 = assign54420_e69480_d_n5;
        locals.var_qg_ov_s_dn6 = assign54420_e69480_d_n6;
        locals.var_qg_ov_s_dn7 = assign54420_e69480_d_n7;
        locals.var_qg_ov_s_dn8 = assign54420_e69480_d_n8;
        locals.var_qg_ov_s_rv = 0.0;

        locals.var_qg_ov_d = 0.0;
        locals.var_qg_ov_d_dn5 = 0.0;
        locals.var_qg_ov_d_dn6 = 0.0;
        locals.var_qg_ov_d_dn7 = 0.0;
        locals.var_qg_ov_d_dn8 = 0.0;
        locals.var_qg_ov_d_rv = 0.0;

        locals.var_yb_ov_d = 0.0;
        locals.var_yb_ov_d_dn5 = 0.0;
        locals.var_yb_ov_d_dn6 = 0.0;
        locals.var_yb_ov_d_dn7 = 0.0;
        locals.var_yb_ov_d_dn8 = 0.0;
        locals.var_yb_ov_d_rv = 0.0;

        let assign54450_e69489: f64 = if ((locals.var_cgovd_i > 0.0) && (locals.var_fcgovaccd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1521 = assign54450_e69489;
        locals.var_guard1521_rv = 0.0;

        let (assign54460_e69499, assign54460_e69499_d_n5, assign54460_e69499_d_n6, assign54460_e69499_d_n7, assign54460_e69499_d_n8,) = {
    if (locals.var_guard1521 != 0.0) {
        let assign54460_e69494: f64 = (0.5 * locals.var_xgb_ov);
        let assign54460_e69496: f64 = (assign54460_e69494 + locals.var_dxgb_ov_d);
        let assign54460_e69497: f64 = (locals.var_cgovaccg_i * assign54460_e69496);
        (assign54460_e69497, (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn5)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn6)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn7)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign54460_e69499;
        locals.var_temp__blk936_dn5 = assign54460_e69499_d_n5;
        locals.var_temp__blk936_dn6 = assign54460_e69499_d_n6;
        locals.var_temp__blk936_dn7 = assign54460_e69499_d_n7;
        locals.var_temp__blk936_dn8 = assign54460_e69499_d_n8;
        locals.var_temp__blk936_rv = 0.0;

        let assign54470_e69502: f64 = if locals.var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1522 = assign54470_e69502;
        locals.var_guard1522_rv = 0.0;

        let assign54480_e69505: f64 = (-230.25850929940458);
        let assign54480_e69506: f64 = if locals.var_temp__blk936 > assign54480_e69505 { 1.0 } else { 0.0 };
        locals.var_guard1523 = assign54480_e69506;
        locals.var_guard1523_rv = 0.0;

        let (assign54490_e69515, assign54490_e69515_d_n5, assign54490_e69515_d_n6, assign54490_e69515_d_n7, assign54490_e69515_d_n8,) = {
    if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1523 != 0.0)) {
        let assign54490_e69513: f64 = (locals.var_temp__blk936).exp();
        (assign54490_e69513, (assign54490_e69513 * locals.var_temp__blk936_dn5), (assign54490_e69513 * locals.var_temp__blk936_dn6), (assign54490_e69513 * locals.var_temp__blk936_dn7), (assign54490_e69513 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn5, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8,)
    }
};
        locals.var_yb_ov_d = assign54490_e69515;
        locals.var_yb_ov_d_dn5 = assign54490_e69515_d_n5;
        locals.var_yb_ov_d_dn6 = assign54490_e69515_d_n6;
        locals.var_yb_ov_d_dn7 = assign54490_e69515_d_n7;
        locals.var_yb_ov_d_dn8 = assign54490_e69515_d_n8;
        locals.var_yb_ov_d_rv = 0.0;

        let (assign54500_e69549, assign54500_e69549_d_n5, assign54500_e69549_d_n6, assign54500_e69549_d_n7, assign54500_e69549_d_n8,) = {
    if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1523 == 0.0)) {
        let assign54500_e69525: f64 = (-230.25850929940458);
        let assign54500_e69527: f64 = (assign54500_e69525 - locals.var_temp__blk936);
        let assign54500_e69531: f64 = (-230.25850929940458);
        let assign54500_e69533: f64 = (assign54500_e69531 - locals.var_temp__blk936);
        let assign54500_e69536: f64 = (-230.25850929940458);
        let assign54500_e69538: f64 = (assign54500_e69536 - locals.var_temp__blk936);
        let assign54500_e69540: f64 = (assign54500_e69538 * 0.3333333333333333);
        let assign54500_e69541: f64 = (1.0 + assign54500_e69540);
        let assign54500_e69542: f64 = (assign54500_e69533 * assign54500_e69541);
        let assign54500_e69543: f64 = (0.5 * assign54500_e69542);
        let assign54500_e69544: f64 = (1.0 + assign54500_e69543);
        let assign54500_e69545: f64 = (assign54500_e69527 * assign54500_e69544);
        let assign54500_e69546: f64 = (1.0 + assign54500_e69545);
        let assign54500_e69547: f64 = (1e-100 / assign54500_e69546);
        (assign54500_e69547, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign54500_e69544) + (assign54500_e69527 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign54500_e69541) + (assign54500_e69533 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign54500_e69546 * assign54500_e69546))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign54500_e69544) + (assign54500_e69527 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign54500_e69541) + (assign54500_e69533 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign54500_e69546 * assign54500_e69546))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign54500_e69544) + (assign54500_e69527 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign54500_e69541) + (assign54500_e69533 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign54500_e69546 * assign54500_e69546))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign54500_e69544) + (assign54500_e69527 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign54500_e69541) + (assign54500_e69533 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign54500_e69546 * assign54500_e69546))),)
    } else {
        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn5, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8,)
    }
};
        locals.var_yb_ov_d = assign54500_e69549;
        locals.var_yb_ov_d_dn5 = assign54500_e69549_d_n5;
        locals.var_yb_ov_d_dn6 = assign54500_e69549_d_n6;
        locals.var_yb_ov_d_dn7 = assign54500_e69549_d_n7;
        locals.var_yb_ov_d_dn8 = assign54500_e69549_d_n8;
        locals.var_yb_ov_d_rv = 0.0;

        let assign54510_e69552: f64 = if locals.var_yb_ov_d > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1524 = assign54510_e69552;
        locals.var_guard1524_rv = 0.0;

        let (assign54520_e69563, assign54520_e69563_d_n5, assign54520_e69563_d_n6, assign54520_e69563_d_n7, assign54520_e69563_d_n8,) = {
    if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1524 != 0.0)) {
        let assign54520_e69560: f64 = (1.0 + locals.var_yb_ov_d);
        let assign54520_e69561: f64 = (assign54520_e69560).ln();
        (assign54520_e69561, (locals.var_yb_ov_d_dn5 / assign54520_e69560), (locals.var_yb_ov_d_dn6 / assign54520_e69560), (locals.var_yb_ov_d_dn7 / assign54520_e69560), (locals.var_yb_ov_d_dn8 / assign54520_e69560),)
    } else {
        (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn5, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8,)
    }
};
        locals.var_xgbeff_ov_d = assign54520_e69563;
        locals.var_xgbeff_ov_d_dn5 = assign54520_e69563_d_n5;
        locals.var_xgbeff_ov_d_dn6 = assign54520_e69563_d_n6;
        locals.var_xgbeff_ov_d_dn7 = assign54520_e69563_d_n7;
        locals.var_xgbeff_ov_d_dn8 = assign54520_e69563_d_n8;
        locals.var_xgbeff_ov_d_rv = 0.0;

        let (assign54530_e69582, assign54530_e69582_d_n5, assign54530_e69582_d_n6, assign54530_e69582_d_n7, assign54530_e69582_d_n8,) = {
    if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1524 != 0.0)) {
        let assign54530_e69573: f64 = (1.0 + locals.var_xgbeff_ov_d);
        let assign54530_e69574: f64 = (assign54530_e69573).ln();
        let assign54530_e69577: f64 = (2.0 + locals.var_xgbeff_ov_d);
        let assign54530_e69578: f64 = (assign54530_e69574 / assign54530_e69577);
        let assign54530_e69579: f64 = (1.0 - assign54530_e69578);
        let assign54530_e69580: f64 = (locals.var_xgbeff_ov_d * assign54530_e69579);
        (assign54530_e69580, ((locals.var_xgbeff_ov_d_dn5 * assign54530_e69579) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn5 / assign54530_e69573) * assign54530_e69577) - (assign54530_e69574 * locals.var_xgbeff_ov_d_dn5)) / (assign54530_e69577 * assign54530_e69577))))), ((locals.var_xgbeff_ov_d_dn6 * assign54530_e69579) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn6 / assign54530_e69573) * assign54530_e69577) - (assign54530_e69574 * locals.var_xgbeff_ov_d_dn6)) / (assign54530_e69577 * assign54530_e69577))))), ((locals.var_xgbeff_ov_d_dn7 * assign54530_e69579) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn7 / assign54530_e69573) * assign54530_e69577) - (assign54530_e69574 * locals.var_xgbeff_ov_d_dn7)) / (assign54530_e69577 * assign54530_e69577))))), ((locals.var_xgbeff_ov_d_dn8 * assign54530_e69579) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn8 / assign54530_e69573) * assign54530_e69577) - (assign54530_e69574 * locals.var_xgbeff_ov_d_dn8)) / (assign54530_e69577 * assign54530_e69577))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54530_e69582;
        locals.var_temp1_dn5 = assign54530_e69582_d_n5;
        locals.var_temp1_dn6 = assign54530_e69582_d_n6;
        locals.var_temp1_dn7 = assign54530_e69582_d_n7;
        locals.var_temp1_dn8 = assign54530_e69582_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign54540_e69591, assign54540_e69591_d_n5, assign54540_e69591_d_n6, assign54540_e69591_d_n7, assign54540_e69591_d_n8,) = {
    if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1524 == 0.0)) {
        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn5, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8,)
    } else {
        (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn5, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8,)
    }
};
        locals.var_xgbeff_ov_d = assign54540_e69591;
        locals.var_xgbeff_ov_d_dn5 = assign54540_e69591_d_n5;
        locals.var_xgbeff_ov_d_dn6 = assign54540_e69591_d_n6;
        locals.var_xgbeff_ov_d_dn7 = assign54540_e69591_d_n7;
        locals.var_xgbeff_ov_d_dn8 = assign54540_e69591_d_n8;
        locals.var_xgbeff_ov_d_rv = 0.0;

        let (assign54550_e69606, assign54550_e69606_d_n5, assign54550_e69606_d_n6, assign54550_e69606_d_n7, assign54550_e69606_d_n8,) = {
    if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1524 == 0.0)) {
        let assign54550_e69600: f64 = (2.0 * locals.var_xgbeff_ov_d);
        let assign54550_e69603: f64 = (2.0 + locals.var_xgbeff_ov_d);
        let assign54550_e69604: f64 = (assign54550_e69600 / assign54550_e69603);
        (assign54550_e69604, ((((2.0 * locals.var_xgbeff_ov_d_dn5) * assign54550_e69603) - (assign54550_e69600 * locals.var_xgbeff_ov_d_dn5)) / (assign54550_e69603 * assign54550_e69603)), ((((2.0 * locals.var_xgbeff_ov_d_dn6) * assign54550_e69603) - (assign54550_e69600 * locals.var_xgbeff_ov_d_dn6)) / (assign54550_e69603 * assign54550_e69603)), ((((2.0 * locals.var_xgbeff_ov_d_dn7) * assign54550_e69603) - (assign54550_e69600 * locals.var_xgbeff_ov_d_dn7)) / (assign54550_e69603 * assign54550_e69603)), ((((2.0 * locals.var_xgbeff_ov_d_dn8) * assign54550_e69603) - (assign54550_e69600 * locals.var_xgbeff_ov_d_dn8)) / (assign54550_e69603 * assign54550_e69603)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54550_e69606;
        locals.var_temp1_dn5 = assign54550_e69606_d_n5;
        locals.var_temp1_dn6 = assign54550_e69606_d_n6;
        locals.var_temp1_dn7 = assign54550_e69606_d_n7;
        locals.var_temp1_dn8 = assign54550_e69606_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign54560_e69613, assign54560_e69613_d_n5, assign54560_e69613_d_n6, assign54560_e69613_d_n7, assign54560_e69613_d_n8,) = {
    if ((locals.var_guard1521 != 0.0) && (locals.var_guard1522 == 0.0)) {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn5, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8,)
    }
};
        locals.var_xgbeff_ov_d = assign54560_e69613;
        locals.var_xgbeff_ov_d_dn5 = assign54560_e69613_d_n5;
        locals.var_xgbeff_ov_d_dn6 = assign54560_e69613_d_n6;
        locals.var_xgbeff_ov_d_dn7 = assign54560_e69613_d_n7;
        locals.var_xgbeff_ov_d_dn8 = assign54560_e69613_d_n8;
        locals.var_xgbeff_ov_d_rv = 0.0;

        let (assign54570_e69631, assign54570_e69631_d_n5, assign54570_e69631_d_n6, assign54570_e69631_d_n7, assign54570_e69631_d_n8,) = {
    if ((locals.var_guard1521 != 0.0) && (locals.var_guard1522 == 0.0)) {
        let assign54570_e69622: f64 = (1.0 + locals.var_xgbeff_ov_d);
        let assign54570_e69623: f64 = (assign54570_e69622).ln();
        let assign54570_e69626: f64 = (2.0 + locals.var_xgbeff_ov_d);
        let assign54570_e69627: f64 = (assign54570_e69623 / assign54570_e69626);
        let assign54570_e69628: f64 = (1.0 - assign54570_e69627);
        let assign54570_e69629: f64 = (locals.var_xgbeff_ov_d * assign54570_e69628);
        (assign54570_e69629, ((locals.var_xgbeff_ov_d_dn5 * assign54570_e69628) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn5 / assign54570_e69622) * assign54570_e69626) - (assign54570_e69623 * locals.var_xgbeff_ov_d_dn5)) / (assign54570_e69626 * assign54570_e69626))))), ((locals.var_xgbeff_ov_d_dn6 * assign54570_e69628) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn6 / assign54570_e69622) * assign54570_e69626) - (assign54570_e69623 * locals.var_xgbeff_ov_d_dn6)) / (assign54570_e69626 * assign54570_e69626))))), ((locals.var_xgbeff_ov_d_dn7 * assign54570_e69628) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn7 / assign54570_e69622) * assign54570_e69626) - (assign54570_e69623 * locals.var_xgbeff_ov_d_dn7)) / (assign54570_e69626 * assign54570_e69626))))), ((locals.var_xgbeff_ov_d_dn8 * assign54570_e69628) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn8 / assign54570_e69622) * assign54570_e69626) - (assign54570_e69623 * locals.var_xgbeff_ov_d_dn8)) / (assign54570_e69626 * assign54570_e69626))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54570_e69631;
        locals.var_temp1_dn5 = assign54570_e69631_d_n5;
        locals.var_temp1_dn6 = assign54570_e69631_d_n6;
        locals.var_temp1_dn7 = assign54570_e69631_d_n7;
        locals.var_temp1_dn8 = assign54570_e69631_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign54580_e69646, assign54580_e69646_d_n5, assign54580_e69646_d_n6, assign54580_e69646_d_n7, assign54580_e69646_d_n8,) = {
    if (locals.var_guard1521 != 0.0) {
        let assign54580_e69634: f64 = (-2.0);
        let assign54580_e69636: f64 = (assign54580_e69634 * locals.var_fcgovaccd_i);
        let assign54580_e69638: f64 = (assign54580_e69636 / locals.var_cgovaccg_i);
        let assign54580_e69640: f64 = (assign54580_e69638 * locals.var_cgovd_i);
        let assign54580_e69642: f64 = (assign54580_e69640 * locals.var_phita);
        let assign54580_e69644: f64 = (assign54580_e69642 * locals.var_temp1);
        (assign54580_e69644, (assign54580_e69642 * locals.var_temp1_dn5), (assign54580_e69642 * locals.var_temp1_dn6), (assign54580_e69642 * locals.var_temp1_dn7), (assign54580_e69642 * locals.var_temp1_dn8),)
    } else {
        (locals.var_qg_ov_d, locals.var_qg_ov_d_dn5, locals.var_qg_ov_d_dn6, locals.var_qg_ov_d_dn7, locals.var_qg_ov_d_dn8,)
    }
};
        locals.var_qg_ov_d = assign54580_e69646;
        locals.var_qg_ov_d_dn5 = assign54580_e69646_d_n5;
        locals.var_qg_ov_d_dn6 = assign54580_e69646_d_n6;
        locals.var_qg_ov_d_dn7 = assign54580_e69646_d_n7;
        locals.var_qg_ov_d_dn8 = assign54580_e69646_d_n8;
        locals.var_qg_ov_d_rv = 0.0;

        let assign54590_e69649: f64 = (locals.var_qg_ov_s + locals.var_qg_ov_d);
        locals.var_qg_ov = assign54590_e69649;
        locals.var_qg_ov_dn5 = (locals.var_qg_ov_s_dn5 + locals.var_qg_ov_d_dn5);
        locals.var_qg_ov_dn6 = (locals.var_qg_ov_s_dn6 + locals.var_qg_ov_d_dn6);
        locals.var_qg_ov_dn7 = (locals.var_qg_ov_s_dn7 + locals.var_qg_ov_d_dn7);
        locals.var_qg_ov_dn8 = (locals.var_qg_ov_s_dn8 + locals.var_qg_ov_d_dn8);
        locals.var_qg_ov_rv = 0.0;

        let assign54600_e69652: f64 = (locals.var_cgbov_i * locals.var_vgb);
        let assign54600_e69654: f64 = (assign54600_e69652 + locals.var_qg_ov);
        locals.var_qgb_ov = assign54600_e69654;
        locals.var_qgb_ov_dn5 = ((locals.var_cgbov_i * locals.var_vgb_dn5) + locals.var_qg_ov_dn5);
        locals.var_qgb_ov_dn6 = ((locals.var_cgbov_i * locals.var_vgb_dn6) + locals.var_qg_ov_dn6);
        locals.var_qgb_ov_dn7 = ((locals.var_cgbov_i * locals.var_vgb_dn7) + locals.var_qg_ov_dn7);
        locals.var_qgb_ov_dn8 = ((locals.var_cgbov_i * locals.var_vgb_dn8) + locals.var_qg_ov_dn8);
        locals.var_qgb_ov_rv = 0.0;

        let assign61970_e80533: f64 = (locals.var_qg + locals.var_qb);
        let assign61970_e80535: f64 = (assign61970_e80533 + locals.var_qd);
        let assign61970_e80536: f64 = (-assign61970_e80535);
        locals.var_qs = assign61970_e80536;
        locals.var_qs_dn5 = (-((locals.var_qg_dn5 + locals.var_qb_dn5) + locals.var_qd_dn5));
        locals.var_qs_dn6 = (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6));
        locals.var_qs_dn7 = (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7));
        locals.var_qs_dn8 = (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8));
        locals.var_qs_rv = 0.0;

        let assign62020_e80567: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1727 = assign62020_e80567;
        locals.var_guard1727_rv = 0.0;

        let (assign62030_e80571, assign62030_e80571_d_n5, assign62030_e80571_d_n6, assign62030_e80571_d_n7, assign62030_e80571_d_n8,) = {
    if (locals.var_guard1727 != 0.0) {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8,)
    } else {
        (locals.var_temp__blk1726, locals.var_temp__blk1726_dn5, locals.var_temp__blk1726_dn6, locals.var_temp__blk1726_dn7, locals.var_temp__blk1726_dn8,)
    }
};
        locals.var_temp__blk1726 = assign62030_e80571;
        locals.var_temp__blk1726_dn5 = assign62030_e80571_d_n5;
        locals.var_temp__blk1726_dn6 = assign62030_e80571_d_n6;
        locals.var_temp__blk1726_dn7 = assign62030_e80571_d_n7;
        locals.var_temp__blk1726_dn8 = assign62030_e80571_d_n8;
        locals.var_temp__blk1726_rv = 0.0;

        let (assign62040_e80575, assign62040_e80575_d_n5, assign62040_e80575_d_n6, assign62040_e80575_d_n7, assign62040_e80575_d_n8,) = {
    if (locals.var_guard1727 != 0.0) {
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8,)
    } else {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8,)
    }
};
        locals.var_qd = assign62040_e80575;
        locals.var_qd_dn5 = assign62040_e80575_d_n5;
        locals.var_qd_dn6 = assign62040_e80575_d_n6;
        locals.var_qd_dn7 = assign62040_e80575_d_n7;
        locals.var_qd_dn8 = assign62040_e80575_d_n8;
        locals.var_qd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_54(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62050_e80579, assign62050_e80579_d_n5, assign62050_e80579_d_n6, assign62050_e80579_d_n7, assign62050_e80579_d_n8,) = {
    if (locals.var_guard1727 != 0.0) {
        (locals.var_temp__blk1726, locals.var_temp__blk1726_dn5, locals.var_temp__blk1726_dn6, locals.var_temp__blk1726_dn7, locals.var_temp__blk1726_dn8,)
    } else {
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8,)
    }
};
        locals.var_qs = assign62050_e80579;
        locals.var_qs_dn5 = assign62050_e80579_d_n5;
        locals.var_qs_dn6 = assign62050_e80579_d_n6;
        locals.var_qs_dn7 = assign62050_e80579_d_n7;
        locals.var_qs_dn8 = assign62050_e80579_d_n8;
        locals.var_qs_rv = 0.0;

        let assign62120_e80588: f64 = (locals.var_cox_qm * locals.var_eta_p_ac);
        locals.var_cgeff = assign62120_e80588;
        locals.var_cgeff_dn5 = ((locals.var_cox_qm_dn5 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn5));
        locals.var_cgeff_dn6 = ((locals.var_cox_qm_dn6 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn6));
        locals.var_cgeff_dn7 = ((locals.var_cox_qm_dn7 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn7));
        locals.var_cgeff_dn8 = ((locals.var_cox_qm_dn8 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn8));
        locals.var_cgeff_rv = 0.0;

        let assign62180_e80600: f64 = if ((locals.var_xg_dc > 0.0) && (locals.var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1760 = assign62180_e80600;
        locals.var_guard1760_rv = 0.0;

        let assign62490_e80966: f64 = if ((((p.p50 == 1.0) && (locals.var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1765 = assign62490_e80966;
        locals.var_guard1765_rv = 0.0;

        let (assign62540_e81063, assign62540_e81063_d_n5, assign62540_e81063_d_n6, assign62540_e81063_d_n7, assign62540_e81063_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
        let assign62540_e81053: f64 = (locals.var_gvsat_ac * locals.var_gvsat_ac);
        let assign62540_e81055: f64 = (assign62540_e81053 * locals.var_cox_qm);
        let assign62540_e81057: f64 = (assign62540_e81055 * locals.var_eta_p_ac);
        let assign62540_e81060: f64 = (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac);
        let assign62540_e81061: f64 = (assign62540_e81057 / assign62540_e81060);
        (assign62540_e81061, (((((((((locals.var_gvsat_ac_dn5 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn5)) * locals.var_cox_qm) + (assign62540_e81053 * locals.var_cox_qm_dn5)) * locals.var_eta_p_ac) + (assign62540_e81055 * locals.var_eta_p_ac_dn5)) * assign62540_e81060) - (assign62540_e81057 * ((locals.var_gmob_dl_ac_dn5 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn5)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((locals.var_gvsat_ac_dn6 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn6)) * locals.var_cox_qm) + (assign62540_e81053 * locals.var_cox_qm_dn6)) * locals.var_eta_p_ac) + (assign62540_e81055 * locals.var_eta_p_ac_dn6)) * assign62540_e81060) - (assign62540_e81057 * ((locals.var_gmob_dl_ac_dn6 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn6)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((locals.var_gvsat_ac_dn7 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn7)) * locals.var_cox_qm) + (assign62540_e81053 * locals.var_cox_qm_dn7)) * locals.var_eta_p_ac) + (assign62540_e81055 * locals.var_eta_p_ac_dn7)) * assign62540_e81060) - (assign62540_e81057 * ((locals.var_gmob_dl_ac_dn7 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn7)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((locals.var_gvsat_ac_dn8 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn8)) * locals.var_cox_qm) + (assign62540_e81053 * locals.var_cox_qm_dn8)) * locals.var_eta_p_ac) + (assign62540_e81055 * locals.var_eta_p_ac_dn8)) * assign62540_e81060) - (assign62540_e81057 * ((locals.var_gmob_dl_ac_dn8 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn8)))) / (assign62540_e81060 * assign62540_e81060)),)
    } else {
        (locals.var_cgeff, locals.var_cgeff_dn5, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8,)
    }
};
        locals.var_cgeff = assign62540_e81063;
        locals.var_cgeff_dn5 = assign62540_e81063_d_n5;
        locals.var_cgeff_dn6 = assign62540_e81063_d_n6;
        locals.var_cgeff_dn7 = assign62540_e81063_d_n7;
        locals.var_cgeff_dn8 = assign62540_e81063_d_n8;
        locals.var_cgeff_rv = 0.0;

        let assign62800_e81277: f64 = if (((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) && (locals.var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1769 = assign62800_e81277;
        locals.var_guard1769_rv = 0.0;

        let (assign62810_e81285, assign62810_e81285_d_n5, assign62810_e81285_d_n6, assign62810_e81285_d_n7, assign62810_e81285_d_n8,) = {
    if (locals.var_guard1769 != 0.0) {
        let assign62810_e81281: f64 = (4.0 * locals.var_dsqredge);
        let assign62810_e81283: f64 = (assign62810_e81281 / locals.var_gfedge2);
        (assign62810_e81283, ((4.0 * locals.var_dsqredge_dn5) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn6) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn7) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn8) / locals.var_gfedge2),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign62810_e81285;
        locals.var_temp1_dn5 = assign62810_e81285_d_n5;
        locals.var_temp1_dn6 = assign62810_e81285_d_n6;
        locals.var_temp1_dn7 = assign62810_e81285_d_n7;
        locals.var_temp1_dn8 = assign62810_e81285_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign62830_e81305, assign62830_e81305_d_n5, assign62830_e81305_d_n6, assign62830_e81305_d_n7, assign62830_e81305_d_n8,) = {
    if (locals.var_guard1769 != 0.0) {
        let assign62830_e81303: f64 = (locals.var_cox_over_q * locals.var_phit);
        (assign62830_e81303, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign62830_e81305;
        locals.var_temp1_dn5 = assign62830_e81305_d_n5;
        locals.var_temp1_dn6 = assign62830_e81305_d_n6;
        locals.var_temp1_dn7 = assign62830_e81305_d_n7;
        locals.var_temp1_dn8 = assign62830_e81305_d_n8;
        locals.var_temp1_rv = 0.0;

        let (assign62960_e81445, assign62960_e81445_d_n5, assign62960_e81445_d_n6, assign62960_e81445_d_n7, assign62960_e81445_d_n8,) = {
    if (locals.var_guard1769 != 0.0) {
        let assign62960_e81443: f64 = (locals.var_alpha_dc * locals.var_h_dc);
        (assign62960_e81443, ((locals.var_alpha_dc_dn5 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn5)), ((locals.var_alpha_dc_dn6 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn6)), ((locals.var_alpha_dc_dn7 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn7)), ((locals.var_alpha_dc_dn8 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn8)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign62960_e81445;
        locals.var_temp1_dn5 = assign62960_e81445_d_n5;
        locals.var_temp1_dn6 = assign62960_e81445_d_n6;
        locals.var_temp1_dn7 = assign62960_e81445_d_n7;
        locals.var_temp1_dn8 = assign62960_e81445_d_n8;
        locals.var_temp1_rv = 0.0;

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
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq0_e948, eq0_e948_d_n5, eq0_e948_d_n6, eq0_e948_d_n7, eq0_e948_d_n8,) = {
    if (locals.var_guard1718 != 0.0) {
        let eq0_e942: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq0_e944: f64 = (eq0_e942 * p.p32);
        let eq0_e946: f64 = (eq0_e944 * locals.var_iimpact);
        let eq0_e946_d_n5: f64 = (eq0_e944 * locals.var_iimpact_dn5);
        let eq0_e946_d_n6: f64 = (eq0_e944 * locals.var_iimpact_dn6);
        let eq0_e946_d_n7: f64 = (eq0_e944 * locals.var_iimpact_dn7);
        let eq0_e946_d_n8: f64 = (eq0_e944 * locals.var_iimpact_dn8);
        (eq0_e946, eq0_e946_d_n5, eq0_e946_d_n6, eq0_e946_d_n7, eq0_e946_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e948;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq0_value),
            [5, 6, 7, 8],
            [multiplicity * (eq0_e948_d_n5), multiplicity * (eq0_e948_d_n6), multiplicity * (eq0_e948_d_n7), multiplicity * (eq0_e948_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq1_e960, eq1_e960_d_n5, eq1_e960_d_n6, eq1_e960_d_n7, eq1_e960_d_n8,) = {
    if (locals.var_guard1718 != 0.0) {
        let eq1_e952: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq1_e954: f64 = (eq1_e952 * p.p32);
        let eq1_e957: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let eq1_e957_d_n5: f64 = (locals.var_i_ds_dn5 + locals.var_i_dsedge_dn5);
        let eq1_e957_d_n6: f64 = (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6);
        let eq1_e957_d_n7: f64 = (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7);
        let eq1_e957_d_n8: f64 = (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8);
        let eq1_e958: f64 = (eq1_e954 * eq1_e957);
        let eq1_e958_d_n5: f64 = (eq1_e954 * eq1_e957_d_n5);
        let eq1_e958_d_n6: f64 = (eq1_e954 * eq1_e957_d_n6);
        let eq1_e958_d_n7: f64 = (eq1_e954 * eq1_e957_d_n7);
        let eq1_e958_d_n8: f64 = (eq1_e954 * eq1_e957_d_n8);
        (eq1_e958, eq1_e958_d_n5, eq1_e958_d_n6, eq1_e958_d_n7, eq1_e958_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e960;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            [5, 6, 7, 8],
            [multiplicity * (eq1_e960_d_n5), multiplicity * (eq1_e960_d_n6), multiplicity * (eq1_e960_d_n7), multiplicity * (eq1_e960_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq2_e970, eq2_e970_d_n5, eq2_e970_d_n6, eq2_e970_d_n7, eq2_e970_d_n8,) = {
    if (locals.var_guard1718 != 0.0) {
        let eq2_e964: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq2_e966: f64 = (eq2_e964 * p.p32);
        let eq2_e968: f64 = (eq2_e966 * locals.var_i_gcs);
        let eq2_e968_d_n5: f64 = (eq2_e966 * locals.var_i_gcs_dn5);
        let eq2_e968_d_n6: f64 = (eq2_e966 * locals.var_i_gcs_dn6);
        let eq2_e968_d_n7: f64 = (eq2_e966 * locals.var_i_gcs_dn7);
        let eq2_e968_d_n8: f64 = (eq2_e966 * locals.var_i_gcs_dn8);
        (eq2_e968, eq2_e968_d_n5, eq2_e968_d_n6, eq2_e968_d_n7, eq2_e968_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e970;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq2_value),
            [5, 6, 7, 8],
            [multiplicity * (eq2_e970_d_n5), multiplicity * (eq2_e970_d_n6), multiplicity * (eq2_e970_d_n7), multiplicity * (eq2_e970_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq3_e980, eq3_e980_d_n5, eq3_e980_d_n6, eq3_e980_d_n7, eq3_e980_d_n8,) = {
    if (locals.var_guard1718 != 0.0) {
        let eq3_e974: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq3_e976: f64 = (eq3_e974 * p.p32);
        let eq3_e978: f64 = (eq3_e976 * locals.var_i_gcd);
        let eq3_e978_d_n5: f64 = (eq3_e976 * locals.var_i_gcd_dn5);
        let eq3_e978_d_n6: f64 = (eq3_e976 * locals.var_i_gcd_dn6);
        let eq3_e978_d_n7: f64 = (eq3_e976 * locals.var_i_gcd_dn7);
        let eq3_e978_d_n8: f64 = (eq3_e976 * locals.var_i_gcd_dn8);
        (eq3_e978, eq3_e978_d_n5, eq3_e978_d_n6, eq3_e978_d_n7, eq3_e978_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e980;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * (eq3_value),
            [5, 6, 7, 8],
            [multiplicity * (eq3_e980_d_n5), multiplicity * (eq3_e980_d_n6), multiplicity * (eq3_e980_d_n7), multiplicity * (eq3_e980_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq4_e991, eq4_e991_d_n5, eq4_e991_d_n6, eq4_e991_d_n7, eq4_e991_d_n8,) = {
    if (locals.var_guard1718 == 0.0) {
        let eq4_e985: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq4_e987: f64 = (eq4_e985 * p.p32);
        let eq4_e989: f64 = (eq4_e987 * locals.var_iimpact);
        let eq4_e989_d_n5: f64 = (eq4_e987 * locals.var_iimpact_dn5);
        let eq4_e989_d_n6: f64 = (eq4_e987 * locals.var_iimpact_dn6);
        let eq4_e989_d_n7: f64 = (eq4_e987 * locals.var_iimpact_dn7);
        let eq4_e989_d_n8: f64 = (eq4_e987 * locals.var_iimpact_dn8);
        (eq4_e989, eq4_e989_d_n5, eq4_e989_d_n6, eq4_e989_d_n7, eq4_e989_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e991;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq4_value),
            [5, 6, 7, 8],
            [multiplicity * (eq4_e991_d_n5), multiplicity * (eq4_e991_d_n6), multiplicity * (eq4_e991_d_n7), multiplicity * (eq4_e991_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq5_e1004, eq5_e1004_d_n5, eq5_e1004_d_n6, eq5_e1004_d_n7, eq5_e1004_d_n8,) = {
    if (locals.var_guard1718 == 0.0) {
        let eq5_e996: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq5_e998: f64 = (eq5_e996 * p.p32);
        let eq5_e1001: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let eq5_e1001_d_n5: f64 = (locals.var_i_ds_dn5 + locals.var_i_dsedge_dn5);
        let eq5_e1001_d_n6: f64 = (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6);
        let eq5_e1001_d_n7: f64 = (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7);
        let eq5_e1001_d_n8: f64 = (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8);
        let eq5_e1002: f64 = (eq5_e998 * eq5_e1001);
        let eq5_e1002_d_n5: f64 = (eq5_e998 * eq5_e1001_d_n5);
        let eq5_e1002_d_n6: f64 = (eq5_e998 * eq5_e1001_d_n6);
        let eq5_e1002_d_n7: f64 = (eq5_e998 * eq5_e1001_d_n7);
        let eq5_e1002_d_n8: f64 = (eq5_e998 * eq5_e1001_d_n8);
        (eq5_e1002, eq5_e1002_d_n5, eq5_e1002_d_n6, eq5_e1002_d_n7, eq5_e1002_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1004;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq5_value),
            [5, 6, 7, 8],
            [multiplicity * (eq5_e1004_d_n5), multiplicity * (eq5_e1004_d_n6), multiplicity * (eq5_e1004_d_n7), multiplicity * (eq5_e1004_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq6_e1015, eq6_e1015_d_n5, eq6_e1015_d_n6, eq6_e1015_d_n7, eq6_e1015_d_n8,) = {
    if (locals.var_guard1718 == 0.0) {
        let eq6_e1009: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq6_e1011: f64 = (eq6_e1009 * p.p32);
        let eq6_e1013: f64 = (eq6_e1011 * locals.var_i_gcs);
        let eq6_e1013_d_n5: f64 = (eq6_e1011 * locals.var_i_gcs_dn5);
        let eq6_e1013_d_n6: f64 = (eq6_e1011 * locals.var_i_gcs_dn6);
        let eq6_e1013_d_n7: f64 = (eq6_e1011 * locals.var_i_gcs_dn7);
        let eq6_e1013_d_n8: f64 = (eq6_e1011 * locals.var_i_gcs_dn8);
        (eq6_e1013, eq6_e1013_d_n5, eq6_e1013_d_n6, eq6_e1013_d_n7, eq6_e1013_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1015;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * (eq6_value),
            [5, 6, 7, 8],
            [multiplicity * (eq6_e1015_d_n5), multiplicity * (eq6_e1015_d_n6), multiplicity * (eq6_e1015_d_n7), multiplicity * (eq6_e1015_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq7_e1026, eq7_e1026_d_n5, eq7_e1026_d_n6, eq7_e1026_d_n7, eq7_e1026_d_n8,) = {
    if (locals.var_guard1718 == 0.0) {
        let eq7_e1020: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq7_e1022: f64 = (eq7_e1020 * p.p32);
        let eq7_e1024: f64 = (eq7_e1022 * locals.var_i_gcd);
        let eq7_e1024_d_n5: f64 = (eq7_e1022 * locals.var_i_gcd_dn5);
        let eq7_e1024_d_n6: f64 = (eq7_e1022 * locals.var_i_gcd_dn6);
        let eq7_e1024_d_n7: f64 = (eq7_e1022 * locals.var_i_gcd_dn7);
        let eq7_e1024_d_n8: f64 = (eq7_e1022 * locals.var_i_gcd_dn8);
        (eq7_e1024, eq7_e1024_d_n5, eq7_e1024_d_n6, eq7_e1024_d_n7, eq7_e1024_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1026;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq7_value),
            [5, 6, 7, 8],
            [multiplicity * (eq7_e1026_d_n5), multiplicity * (eq7_e1026_d_n6), multiplicity * (eq7_e1026_d_n7), multiplicity * (eq7_e1026_d_n8)],
            [],
            [],
            1.0,
        );
        let eq8_e1029: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq8_e1031: f64 = (eq8_e1029 * p.p32);
        let eq8_e1033: f64 = (eq8_e1031 * locals.var_i_gb);
        let eq8_e1033_d_n5: f64 = (eq8_e1031 * locals.var_i_gb_dn5);
        let eq8_e1033_d_n6: f64 = (eq8_e1031 * locals.var_i_gb_dn6);
        let eq8_e1033_d_n7: f64 = (eq8_e1031 * locals.var_i_gb_dn7);
        let eq8_e1033_d_n8: f64 = (eq8_e1031 * locals.var_i_gb_dn8);
        let eq8_value: f64 = eq8_e1033;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            [5, 6, 7, 8],
            [multiplicity * (eq8_e1033_d_n5), multiplicity * (eq8_e1033_d_n6), multiplicity * (eq8_e1033_d_n7), multiplicity * (eq8_e1033_d_n8)],
            [],
            [],
            1.0,
        );
        let eq9_e1036: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq9_e1038: f64 = (eq9_e1036 * p.p32);
        let eq9_e1040: f64 = (eq9_e1038 * locals.var_igsov);
        let eq9_e1040_d_n5: f64 = (eq9_e1038 * locals.var_igsov_dn5);
        let eq9_e1040_d_n6: f64 = (eq9_e1038 * locals.var_igsov_dn6);
        let eq9_e1040_d_n7: f64 = (eq9_e1038 * locals.var_igsov_dn7);
        let eq9_e1040_d_n8: f64 = (eq9_e1038 * locals.var_igsov_dn8);
        let eq9_value: f64 = eq9_e1040;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq9_value),
            [5, 6, 7, 8],
            [multiplicity * (eq9_e1040_d_n5), multiplicity * (eq9_e1040_d_n6), multiplicity * (eq9_e1040_d_n7), multiplicity * (eq9_e1040_d_n8)],
            [],
            [],
            1.0,
        );
        let eq10_e1043: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq10_e1045: f64 = (eq10_e1043 * p.p32);
        let eq10_e1047: f64 = (eq10_e1045 * locals.var_igdov);
        let eq10_e1047_d_n5: f64 = (eq10_e1045 * locals.var_igdov_dn5);
        let eq10_e1047_d_n6: f64 = (eq10_e1045 * locals.var_igdov_dn6);
        let eq10_e1047_d_n7: f64 = (eq10_e1045 * locals.var_igdov_dn7);
        let eq10_e1047_d_n8: f64 = (eq10_e1045 * locals.var_igdov_dn8);
        let eq10_value: f64 = eq10_e1047;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            [5, 6, 7, 8],
            [multiplicity * (eq10_e1047_d_n5), multiplicity * (eq10_e1047_d_n6), multiplicity * (eq10_e1047_d_n7), multiplicity * (eq10_e1047_d_n8)],
            [],
            [],
            1.0,
        );
        let eq11_e1050: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq11_e1052: f64 = (eq11_e1050 * p.p32);
        let eq11_e1054: f64 = (eq11_e1052 * locals.var_i_gisl);
        let eq11_e1054_d_n5: f64 = (eq11_e1052 * locals.var_i_gisl_dn5);
        let eq11_e1054_d_n6: f64 = (eq11_e1052 * locals.var_i_gisl_dn6);
        let eq11_e1054_d_n7: f64 = (eq11_e1052 * locals.var_i_gisl_dn7);
        let eq11_e1054_d_n8: f64 = (eq11_e1052 * locals.var_i_gisl_dn8);
        let eq11_value: f64 = eq11_e1054;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq11_value),
            [5, 6, 7, 8],
            [multiplicity * (eq11_e1054_d_n5), multiplicity * (eq11_e1054_d_n6), multiplicity * (eq11_e1054_d_n7), multiplicity * (eq11_e1054_d_n8)],
            [],
            [],
            1.0,
        );
        let eq12_e1057: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq12_e1059: f64 = (eq12_e1057 * p.p32);
        let eq12_e1061: f64 = (eq12_e1059 * locals.var_i_gidl);
        let eq12_e1061_d_n5: f64 = (eq12_e1059 * locals.var_i_gidl_dn5);
        let eq12_e1061_d_n6: f64 = (eq12_e1059 * locals.var_i_gidl_dn6);
        let eq12_e1061_d_n7: f64 = (eq12_e1059 * locals.var_i_gidl_dn7);
        let eq12_e1061_d_n8: f64 = (eq12_e1059 * locals.var_i_gidl_dn8);
        let eq12_value: f64 = eq12_e1061;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq12_value),
            [5, 6, 7, 8],
            [multiplicity * (eq12_e1061_d_n5), multiplicity * (eq12_e1061_d_n6), multiplicity * (eq12_e1061_d_n7), multiplicity * (eq12_e1061_d_n8)],
            [],
            [],
            1.0,
        );
        let eq38_e1263: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq38_e1265: f64 = (eq38_e1263 * p.p33);
        let eq38_e1267: f64 = (eq38_e1265 * locals.var_qg);
        let eq38_e1267_d_n5: f64 = (eq38_e1265 * locals.var_qg_dn5);
        let eq38_e1267_d_n6: f64 = (eq38_e1265 * locals.var_qg_dn6);
        let eq38_e1267_d_n7: f64 = (eq38_e1265 * locals.var_qg_dn7);
        let eq38_e1267_d_n8: f64 = (eq38_e1265 * locals.var_qg_dn8);
        let eq38_e1268: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq38_e1267);
        let eq38_value: f64 = eq38_e1268;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq38_value),
            [5, 6, 7, 8],
            [multiplicity * ((eq38_e1267_d_n5 * ddt_scale)), multiplicity * ((eq38_e1267_d_n6 * ddt_scale)), multiplicity * ((eq38_e1267_d_n7 * ddt_scale)), multiplicity * ((eq38_e1267_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq39_e1271: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq39_e1273: f64 = (eq39_e1271 * p.p33);
        let eq39_e1275: f64 = (eq39_e1273 * locals.var_qb);
        let eq39_e1275_d_n5: f64 = (eq39_e1273 * locals.var_qb_dn5);
        let eq39_e1275_d_n6: f64 = (eq39_e1273 * locals.var_qb_dn6);
        let eq39_e1275_d_n7: f64 = (eq39_e1273 * locals.var_qb_dn7);
        let eq39_e1275_d_n8: f64 = (eq39_e1273 * locals.var_qb_dn8);
        let eq39_e1276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq39_e1275);
        let eq39_value: f64 = eq39_e1276;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq39_value),
            [5, 6, 7, 8],
            [multiplicity * ((eq39_e1275_d_n5 * ddt_scale)), multiplicity * ((eq39_e1275_d_n6 * ddt_scale)), multiplicity * ((eq39_e1275_d_n7 * ddt_scale)), multiplicity * ((eq39_e1275_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq40_e1279: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq40_e1281: f64 = (eq40_e1279 * p.p33);
        let eq40_e1283: f64 = (eq40_e1281 * locals.var_qd);
        let eq40_e1283_d_n5: f64 = (eq40_e1281 * locals.var_qd_dn5);
        let eq40_e1283_d_n6: f64 = (eq40_e1281 * locals.var_qd_dn6);
        let eq40_e1283_d_n7: f64 = (eq40_e1281 * locals.var_qd_dn7);
        let eq40_e1283_d_n8: f64 = (eq40_e1281 * locals.var_qd_dn8);
        let eq40_e1284: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq40_e1283);
        let eq40_value: f64 = eq40_e1284;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq40_value),
            [5, 6, 7, 8],
            [multiplicity * ((eq40_e1283_d_n5 * ddt_scale)), multiplicity * ((eq40_e1283_d_n6 * ddt_scale)), multiplicity * ((eq40_e1283_d_n7 * ddt_scale)), multiplicity * ((eq40_e1283_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq43_e1303: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq43_e1305: f64 = (eq43_e1303 * p.p33);
        let eq43_e1307: f64 = (eq43_e1305 * locals.var_qgb_ov);
        let eq43_e1307_d_n5: f64 = (eq43_e1305 * locals.var_qgb_ov_dn5);
        let eq43_e1307_d_n6: f64 = (eq43_e1305 * locals.var_qgb_ov_dn6);
        let eq43_e1307_d_n7: f64 = (eq43_e1305 * locals.var_qgb_ov_dn7);
        let eq43_e1307_d_n8: f64 = (eq43_e1305 * locals.var_qgb_ov_dn8);
        let eq43_e1308: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq43_e1307);
        let eq43_value: f64 = eq43_e1308;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(8),
            multiplicity * (eq43_value),
            [5, 6, 7, 8],
            [multiplicity * ((eq43_e1307_d_n5 * ddt_scale)), multiplicity * ((eq43_e1307_d_n6 * ddt_scale)), multiplicity * ((eq43_e1307_d_n7 * ddt_scale)), multiplicity * ((eq43_e1307_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_mig;
        let eq47_e1332: f64 = ((nv4 - 0.0) * __rspice_inv_cse_0);
        let eq47_e1332_d_n4: f64 = (1.0 * __rspice_inv_cse_0);
        let eq47_e1332_d_n5: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn5) / (locals.var_mig * locals.var_mig)));
        let eq47_e1332_d_n6: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn6) / (locals.var_mig * locals.var_mig)));
        let eq47_e1332_d_n7: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn7) / (locals.var_mig * locals.var_mig)));
        let eq47_e1332_d_n8: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn8) / (locals.var_mig * locals.var_mig)));
        let eq47_value: f64 = eq47_e1332;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq47_value),
            [4, 5, 6, 7, 8],
            [multiplicity * (eq47_e1332_d_n4), multiplicity * (eq47_e1332_d_n5), multiplicity * (eq47_e1332_d_n6), multiplicity * (eq47_e1332_d_n7), multiplicity * (eq47_e1332_d_n8)],
            [],
            [],
            1.0,
        );
        let eq48_e1335: f64 = (locals.var_cgeff * (nv4 - 0.0));
        let eq48_e1335_d_n5: f64 = (locals.var_cgeff_dn5 * (nv4 - 0.0));
        let eq48_e1335_d_n6: f64 = (locals.var_cgeff_dn6 * (nv4 - 0.0));
        let eq48_e1335_d_n7: f64 = (locals.var_cgeff_dn7 * (nv4 - 0.0));
        let eq48_e1335_d_n8: f64 = (locals.var_cgeff_dn8 * (nv4 - 0.0));
        let eq48_e1336: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq48_e1335);
        let eq48_value: f64 = eq48_e1336;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq48_value),
            [4, 5, 6, 7, 8],
            [multiplicity * ((locals.var_cgeff * ddt_scale)), multiplicity * ((eq48_e1335_d_n5 * ddt_scale)), multiplicity * ((eq48_e1335_d_n6 * ddt_scale)), multiplicity * ((eq48_e1335_d_n7 * ddt_scale)), multiplicity * ((eq48_e1335_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq49_e1339: f64 = (locals.var_mult_inst * p.p32);
        let eq49_e1340: f64 = (eq49_e1339).sqrt();
        let eq49_e1342: f64 = (eq49_e1340 * 0.5);
        let eq49_e1344: f64 = (eq49_e1342 * locals.var_cgeff);
        let eq49_e1344_d_n5: f64 = (eq49_e1342 * locals.var_cgeff_dn5);
        let eq49_e1344_d_n6: f64 = (eq49_e1342 * locals.var_cgeff_dn6);
        let eq49_e1344_d_n7: f64 = (eq49_e1342 * locals.var_cgeff_dn7);
        let eq49_e1344_d_n8: f64 = (eq49_e1342 * locals.var_cgeff_dn8);
        let eq49_e1346: f64 = (eq49_e1344 * (nv4 - 0.0));
        let eq49_e1346_d_n5: f64 = (eq49_e1344_d_n5 * (nv4 - 0.0));
        let eq49_e1346_d_n6: f64 = (eq49_e1344_d_n6 * (nv4 - 0.0));
        let eq49_e1346_d_n7: f64 = (eq49_e1344_d_n7 * (nv4 - 0.0));
        let eq49_e1346_d_n8: f64 = (eq49_e1344_d_n8 * (nv4 - 0.0));
        let eq49_e1347: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq49_e1346);
        let eq49_e1348: f64 = (-eq49_e1347);
        let eq49_e1348_d_n4: f64 = (-(eq49_e1344 * ddt_scale));
        let eq49_e1348_d_n5: f64 = (-(eq49_e1346_d_n5 * ddt_scale));
        let eq49_e1348_d_n6: f64 = (-(eq49_e1346_d_n6 * ddt_scale));
        let eq49_e1348_d_n7: f64 = (-(eq49_e1346_d_n7 * ddt_scale));
        let eq49_e1348_d_n8: f64 = (-(eq49_e1346_d_n8 * ddt_scale));
        let eq49_value: f64 = eq49_e1348;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq49_value),
            [4, 5, 6, 7, 8],
            [multiplicity * (eq49_e1348_d_n4), multiplicity * (eq49_e1348_d_n5), multiplicity * (eq49_e1348_d_n6), multiplicity * (eq49_e1348_d_n7), multiplicity * (eq49_e1348_d_n8)],
            [],
            [],
            1.0,
        );
        let eq50_e1351: f64 = (locals.var_mult_inst * p.p32);
        let eq50_e1352: f64 = (eq50_e1351).sqrt();
        let eq50_e1354: f64 = (eq50_e1352 * 0.5);
        let eq50_e1356: f64 = (eq50_e1354 * locals.var_cgeff);
        let eq50_e1356_d_n5: f64 = (eq50_e1354 * locals.var_cgeff_dn5);
        let eq50_e1356_d_n6: f64 = (eq50_e1354 * locals.var_cgeff_dn6);
        let eq50_e1356_d_n7: f64 = (eq50_e1354 * locals.var_cgeff_dn7);
        let eq50_e1356_d_n8: f64 = (eq50_e1354 * locals.var_cgeff_dn8);
        let eq50_e1358: f64 = (eq50_e1356 * (nv4 - 0.0));
        let eq50_e1358_d_n5: f64 = (eq50_e1356_d_n5 * (nv4 - 0.0));
        let eq50_e1358_d_n6: f64 = (eq50_e1356_d_n6 * (nv4 - 0.0));
        let eq50_e1358_d_n7: f64 = (eq50_e1356_d_n7 * (nv4 - 0.0));
        let eq50_e1358_d_n8: f64 = (eq50_e1356_d_n8 * (nv4 - 0.0));
        let eq50_e1359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq50_e1358);
        let eq50_e1360: f64 = (-eq50_e1359);
        let eq50_e1360_d_n4: f64 = (-(eq50_e1356 * ddt_scale));
        let eq50_e1360_d_n5: f64 = (-(eq50_e1358_d_n5 * ddt_scale));
        let eq50_e1360_d_n6: f64 = (-(eq50_e1358_d_n6 * ddt_scale));
        let eq50_e1360_d_n7: f64 = (-(eq50_e1358_d_n7 * ddt_scale));
        let eq50_e1360_d_n8: f64 = (-(eq50_e1358_d_n8 * ddt_scale));
        let eq50_value: f64 = eq50_e1360;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(7),
            multiplicity * (eq50_value),
            [4, 5, 6, 7, 8],
            [multiplicity * (eq50_e1360_d_n4), multiplicity * (eq50_e1360_d_n5), multiplicity * (eq50_e1360_d_n6), multiplicity * (eq50_e1360_d_n7), multiplicity * (eq50_e1360_d_n8)],
            [],
            [],
            1.0,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq38_e1263: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq38_e1265: f64 = (eq38_e1263 * p.p33);
        let eq38_e1267: f64 = (eq38_e1265 * locals.var_qg);
        let eq38_e1267_d_n5: f64 = (eq38_e1265 * locals.var_qg_dn5);
        let eq38_e1267_d_n6: f64 = (eq38_e1265 * locals.var_qg_dn6);
        let eq38_e1267_d_n7: f64 = (eq38_e1265 * locals.var_qg_dn7);
        let eq38_e1267_d_n8: f64 = (eq38_e1265 * locals.var_qg_dn8);
        let eq38_e1268_q: f64 = eq38_e1267;
        stamper.stamp_current_reactive(
            Some(nodes[5]),
            Some(nodes[6]),
            &[
                GeneratedDerivative::node(nodes[5], multiplicity * (eq38_e1267_d_n5)),
                GeneratedDerivative::node(nodes[6], multiplicity * (eq38_e1267_d_n6)),
                GeneratedDerivative::node(nodes[7], multiplicity * (eq38_e1267_d_n7)),
                GeneratedDerivative::node(nodes[8], multiplicity * (eq38_e1267_d_n8)),
            ],
        );
        let eq39_e1271: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq39_e1273: f64 = (eq39_e1271 * p.p33);
        let eq39_e1275: f64 = (eq39_e1273 * locals.var_qb);
        let eq39_e1275_d_n5: f64 = (eq39_e1273 * locals.var_qb_dn5);
        let eq39_e1275_d_n6: f64 = (eq39_e1273 * locals.var_qb_dn6);
        let eq39_e1275_d_n7: f64 = (eq39_e1273 * locals.var_qb_dn7);
        let eq39_e1275_d_n8: f64 = (eq39_e1273 * locals.var_qb_dn8);
        let eq39_e1276_q: f64 = eq39_e1275;
        stamper.stamp_current_reactive(
            Some(nodes[8]),
            Some(nodes[6]),
            &[
                GeneratedDerivative::node(nodes[5], multiplicity * (eq39_e1275_d_n5)),
                GeneratedDerivative::node(nodes[6], multiplicity * (eq39_e1275_d_n6)),
                GeneratedDerivative::node(nodes[7], multiplicity * (eq39_e1275_d_n7)),
                GeneratedDerivative::node(nodes[8], multiplicity * (eq39_e1275_d_n8)),
            ],
        );
        let eq40_e1279: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq40_e1281: f64 = (eq40_e1279 * p.p33);
        let eq40_e1283: f64 = (eq40_e1281 * locals.var_qd);
        let eq40_e1283_d_n5: f64 = (eq40_e1281 * locals.var_qd_dn5);
        let eq40_e1283_d_n6: f64 = (eq40_e1281 * locals.var_qd_dn6);
        let eq40_e1283_d_n7: f64 = (eq40_e1281 * locals.var_qd_dn7);
        let eq40_e1283_d_n8: f64 = (eq40_e1281 * locals.var_qd_dn8);
        let eq40_e1284_q: f64 = eq40_e1283;
        stamper.stamp_current_reactive(
            Some(nodes[7]),
            Some(nodes[6]),
            &[
                GeneratedDerivative::node(nodes[5], multiplicity * (eq40_e1283_d_n5)),
                GeneratedDerivative::node(nodes[6], multiplicity * (eq40_e1283_d_n6)),
                GeneratedDerivative::node(nodes[7], multiplicity * (eq40_e1283_d_n7)),
                GeneratedDerivative::node(nodes[8], multiplicity * (eq40_e1283_d_n8)),
            ],
        );
        let eq43_e1303: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq43_e1305: f64 = (eq43_e1303 * p.p33);
        let eq43_e1307: f64 = (eq43_e1305 * locals.var_qgb_ov);
        let eq43_e1307_d_n5: f64 = (eq43_e1305 * locals.var_qgb_ov_dn5);
        let eq43_e1307_d_n6: f64 = (eq43_e1305 * locals.var_qgb_ov_dn6);
        let eq43_e1307_d_n7: f64 = (eq43_e1305 * locals.var_qgb_ov_dn7);
        let eq43_e1307_d_n8: f64 = (eq43_e1305 * locals.var_qgb_ov_dn8);
        let eq43_e1308_q: f64 = eq43_e1307;
        stamper.stamp_current_reactive(
            Some(nodes[5]),
            Some(nodes[8]),
            &[
                GeneratedDerivative::node(nodes[5], multiplicity * (eq43_e1307_d_n5)),
                GeneratedDerivative::node(nodes[6], multiplicity * (eq43_e1307_d_n6)),
                GeneratedDerivative::node(nodes[7], multiplicity * (eq43_e1307_d_n7)),
                GeneratedDerivative::node(nodes[8], multiplicity * (eq43_e1307_d_n8)),
            ],
        );
        let eq48_e1335: f64 = (locals.var_cgeff * (nv4 - 0.0));
        let eq48_e1335_d_n5: f64 = (locals.var_cgeff_dn5 * (nv4 - 0.0));
        let eq48_e1335_d_n6: f64 = (locals.var_cgeff_dn6 * (nv4 - 0.0));
        let eq48_e1335_d_n7: f64 = (locals.var_cgeff_dn7 * (nv4 - 0.0));
        let eq48_e1335_d_n8: f64 = (locals.var_cgeff_dn8 * (nv4 - 0.0));
        let eq48_e1336_q: f64 = eq48_e1335;
        let eq48_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, locals.var_cgeff, eq48_e1335_d_n5, eq48_e1335_d_n6, eq48_e1335_d_n7, eq48_e1335_d_n8, 0.0, 0.0, 0.0];
        let eq48_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let eq49_e1339: f64 = (locals.var_mult_inst * p.p32);
        let eq49_e1340: f64 = (eq49_e1339).sqrt();
        let eq49_e1342: f64 = (eq49_e1340 * 0.5);
        let eq49_e1344: f64 = (eq49_e1342 * locals.var_cgeff);
        let eq49_e1344_d_n5: f64 = (eq49_e1342 * locals.var_cgeff_dn5);
        let eq49_e1344_d_n6: f64 = (eq49_e1342 * locals.var_cgeff_dn6);
        let eq49_e1344_d_n7: f64 = (eq49_e1342 * locals.var_cgeff_dn7);
        let eq49_e1344_d_n8: f64 = (eq49_e1342 * locals.var_cgeff_dn8);
        let eq49_e1346: f64 = (eq49_e1344 * (nv4 - 0.0));
        let eq49_e1346_d_n5: f64 = (eq49_e1344_d_n5 * (nv4 - 0.0));
        let eq49_e1346_d_n6: f64 = (eq49_e1344_d_n6 * (nv4 - 0.0));
        let eq49_e1346_d_n7: f64 = (eq49_e1344_d_n7 * (nv4 - 0.0));
        let eq49_e1346_d_n8: f64 = (eq49_e1344_d_n8 * (nv4 - 0.0));
        let eq49_e1347_q: f64 = eq49_e1346;
        let eq49_e1348: f64 = (-eq49_e1346);
        let eq49_e1348_q: f64 = (-eq49_e1347_q);
        let eq49_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, (-eq49_e1344), (-eq49_e1346_d_n5), (-eq49_e1346_d_n6), (-eq49_e1346_d_n7), (-eq49_e1346_d_n8), 0.0, 0.0, 0.0];
        let eq49_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq49_reactive_node_derivatives,
            branches,
            &eq49_reactive_branch_derivatives,
            multiplicity,
        );
        let eq50_e1351: f64 = (locals.var_mult_inst * p.p32);
        let eq50_e1352: f64 = (eq50_e1351).sqrt();
        let eq50_e1354: f64 = (eq50_e1352 * 0.5);
        let eq50_e1356: f64 = (eq50_e1354 * locals.var_cgeff);
        let eq50_e1356_d_n5: f64 = (eq50_e1354 * locals.var_cgeff_dn5);
        let eq50_e1356_d_n6: f64 = (eq50_e1354 * locals.var_cgeff_dn6);
        let eq50_e1356_d_n7: f64 = (eq50_e1354 * locals.var_cgeff_dn7);
        let eq50_e1356_d_n8: f64 = (eq50_e1354 * locals.var_cgeff_dn8);
        let eq50_e1358: f64 = (eq50_e1356 * (nv4 - 0.0));
        let eq50_e1358_d_n5: f64 = (eq50_e1356_d_n5 * (nv4 - 0.0));
        let eq50_e1358_d_n6: f64 = (eq50_e1356_d_n6 * (nv4 - 0.0));
        let eq50_e1358_d_n7: f64 = (eq50_e1356_d_n7 * (nv4 - 0.0));
        let eq50_e1358_d_n8: f64 = (eq50_e1356_d_n8 * (nv4 - 0.0));
        let eq50_e1359_q: f64 = eq50_e1358;
        let eq50_e1360: f64 = (-eq50_e1358);
        let eq50_e1360_q: f64 = (-eq50_e1359_q);
        let eq50_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, (-eq50_e1356), (-eq50_e1358_d_n5), (-eq50_e1358_d_n6), (-eq50_e1358_d_n7), (-eq50_e1358_d_n8), 0.0, 0.0, 0.0];
        let eq50_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
    }
}

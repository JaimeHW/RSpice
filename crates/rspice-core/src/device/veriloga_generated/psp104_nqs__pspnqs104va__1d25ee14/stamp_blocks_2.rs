#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    pub(super) fn stamp_transient_block_32(
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27940_e33313: f64 = (locals.var_xhighr_s).exp();
            (locals.var_expxhr_s, locals.var_expxhr_s_dn5, locals.var_expxhr_s_dn6, locals.var_expxhr_s_dn7, locals.var_expxhr_s_dn8, ) = (assign27940_e33313, (assign27940_e33313 * locals.var_xhighr_s_dn5), (assign27940_e33313 * locals.var_xhighr_s_dn6), (assign27940_e33313 * locals.var_xhighr_s_dn7), (assign27940_e33313 * locals.var_xhighr_s_dn8), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            locals.var_fracna = 0.4;
            locals.var_fracnb = 0.65;
            locals.var_fraci = 0.8;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27980_e33338: f64 = (-locals.var_fracna);
            let assign27980_e33340: f64 = (assign27980_e33338 * locals.var_vjunrefd_i);
            locals.var_v1 = assign27980_e33340;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27990_e33347: f64 = (-locals.var_fracnb);
            let assign27990_e33349: f64 = (assign27990_e33347 * locals.var_vjunrefd_i);
            locals.var_v2 = assign27990_e33349;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign28000_e33356: f64 = (-locals.var_fraci);
            let assign28000_e33358: f64 = (assign28000_e33356 * locals.var_vjunrefd_i);
            locals.var_v3 = assign28000_e33358;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            locals.var_v4 = 0.1;
            locals.var_v5 = 0.2;
            locals.var_vbbt = 0.0;
            locals.var_two_psistar = 0.0;
        }
        let assign28050_e33396: f64 = if (!(((locals.var_abdrain_i == 0.0) && (locals.var_lsdrain_i == 0.0)) && (locals.var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard538 = assign28050_e33396;
        let assign28130_e33482: f64 = if locals.var_v1 < locals.var_vmax_d { 1.0 } else { 0.0 };
        locals.var_guard539 = assign28130_e33482;
        let assign28140_e33484: f64 = (-0.5);
        let assign28140_e33487: f64 = (locals.var_v1 * locals.var_phitdinv);
        let assign28140_e33488: f64 = (assign28140_e33484 * assign28140_e33487);
        let assign28140_e33489: f64 = (assign28140_e33488).abs();
        let assign28140_e33491: f64 = if assign28140_e33489 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard540 = assign28140_e33491;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 != 0.0)) {
            let assign28150_e33502: f64 = (-0.5);
            let assign28150_e33505: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign28150_e33506: f64 = (assign28150_e33502 * assign28150_e33505);
            let assign28150_e33507: f64 = (assign28150_e33506).exp();
            locals.var_z = assign28150_e33507;
        }
        let assign28160_e33511: f64 = (-0.5);
        let assign28160_e33514: f64 = (locals.var_v1 * locals.var_phitdinv);
        let assign28160_e33515: f64 = (assign28160_e33511 * assign28160_e33514);
        let assign28160_e33517: f64 = if assign28160_e33515 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard541 = assign28160_e33517;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 == 0.0)) && (locals.var_guard541 != 0.0)) {
            let assign28170_e33533: f64 = (-230.25850929940458);
            let assign28170_e33535: f64 = (-0.5);
            let assign28170_e33538: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign28170_e33539: f64 = (assign28170_e33535 * assign28170_e33538);
            let assign28170_e33540: f64 = (assign28170_e33533 - assign28170_e33539);
            let assign28170_e33544: f64 = (-230.25850929940458);
            let assign28170_e33546: f64 = (-0.5);
            let assign28170_e33549: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign28170_e33550: f64 = (assign28170_e33546 * assign28170_e33549);
            let assign28170_e33551: f64 = (assign28170_e33544 - assign28170_e33550);
            let assign28170_e33554: f64 = (-230.25850929940458);
            let assign28170_e33556: f64 = (-0.5);
            let assign28170_e33559: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign28170_e33560: f64 = (assign28170_e33556 * assign28170_e33559);
            let assign28170_e33561: f64 = (assign28170_e33554 - assign28170_e33560);
            let assign28170_e33563: f64 = (assign28170_e33561 * 0.3333333333333333);
            let assign28170_e33564: f64 = (1.0 + assign28170_e33563);
            let assign28170_e33565: f64 = (assign28170_e33551 * assign28170_e33564);
            let assign28170_e33566: f64 = (0.5 * assign28170_e33565);
            let assign28170_e33567: f64 = (1.0 + assign28170_e33566);
            let assign28170_e33568: f64 = (assign28170_e33540 * assign28170_e33567);
            let assign28170_e33569: f64 = (1.0 + assign28170_e33568);
            let assign28170_e33570: f64 = (1e-100 / assign28170_e33569);
            locals.var_z = assign28170_e33570;
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 == 0.0)) && (locals.var_guard541 == 0.0)) {
            let assign28180_e33589: f64 = (-0.5);
            let assign28180_e33592: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign28180_e33593: f64 = (assign28180_e33589 * assign28180_e33592);
            let assign28180_e33595: f64 = (assign28180_e33593 - 230.25850929940458);
            let assign28180_e33599: f64 = (-0.5);
            let assign28180_e33602: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign28180_e33603: f64 = (assign28180_e33599 * assign28180_e33602);
            let assign28180_e33605: f64 = (assign28180_e33603 - 230.25850929940458);
            let assign28180_e33608: f64 = (-0.5);
            let assign28180_e33611: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign28180_e33612: f64 = (assign28180_e33608 * assign28180_e33611);
            let assign28180_e33614: f64 = (assign28180_e33612 - 230.25850929940458);
            let assign28180_e33616: f64 = (assign28180_e33614 * 0.3333333333333333);
            let assign28180_e33617: f64 = (1.0 + assign28180_e33616);
            let assign28180_e33618: f64 = (assign28180_e33605 * assign28180_e33617);
            let assign28180_e33619: f64 = (0.5 * assign28180_e33618);
            let assign28180_e33620: f64 = (1.0 + assign28180_e33619);
            let assign28180_e33621: f64 = (assign28180_e33595 * assign28180_e33620);
            let assign28180_e33622: f64 = (1.0 + assign28180_e33621);
            let assign28180_e33623: f64 = (1e100 * assign28180_e33622);
            locals.var_z = assign28180_e33623;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 != 0.0)) {
            let assign28190_e33635: f64 = (1.0 / locals.var_z);
            locals.var_zinv = assign28190_e33635;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 != 0.0)) {
            let assign28200_e33647: f64 = (locals.var_zinv * locals.var_zinv);
            locals.var_idmult = assign28200_e33647;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 == 0.0)) {
            let assign28210_e33661: f64 = (locals.var_v1 - locals.var_vmax_d);
            let assign28210_e33663: f64 = (assign28210_e33661 * locals.var_phitdinv);
            let assign28210_e33664: f64 = (1.0 + assign28210_e33663);
            let assign28210_e33666: f64 = (assign28210_e33664 * locals.var_exp_vmax_over_phitd_d);
            locals.var_idmult = assign28210_e33666;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 == 0.0)) {
            let assign28220_e33678: f64 = (locals.var_idmult).sqrt();
            locals.var_zinv = assign28220_e33678;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 == 0.0)) {
            let assign28230_e33691: f64 = (1.0 / locals.var_zinv);
            locals.var_z = assign28230_e33691;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign28240_e33701: f64 = (locals.var_idmult - 1.0);
            locals.var_idmult = assign28240_e33701;
        }
        let assign28250_e33706: f64 = if locals.var_v1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard542 = assign28250_e33706;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard542 != 0.0)) {
            let assign28260_e33718: f64 = (2.0 + locals.var_z);
            let assign28260_e33721: f64 = (locals.var_z + 1.0);
            let assign28260_e33724: f64 = (locals.var_z + 3.0);
            let assign28260_e33725: f64 = (assign28260_e33721 * assign28260_e33724);
            let assign28260_e33726: f64 = (assign28260_e33725).sqrt();
            let assign28260_e33727: f64 = (assign28260_e33718 + assign28260_e33726);
            let assign28260_e33728: f64 = (assign28260_e33727).ln();
            let assign28260_e33729: f64 = (locals.var_phitd * assign28260_e33728);
            let assign28260_e33730: f64 = (2.0 * assign28260_e33729);
            locals.var_two_psistar = assign28260_e33730;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard542 == 0.0)) {
            let assign28270_e33742: f64 = (-locals.var_v1);
            let assign28270_e33747: f64 = (2.0 * locals.var_zinv);
            let assign28270_e33749: f64 = (assign28270_e33747 + 1.0);
            let assign28270_e33752: f64 = (1.0 + locals.var_zinv);
            let assign28270_e33756: f64 = (3.0 * locals.var_zinv);
            let assign28270_e33757: f64 = (1.0 + assign28270_e33756);
            let assign28270_e33758: f64 = (assign28270_e33752 * assign28270_e33757);
            let assign28270_e33759: f64 = (assign28270_e33758).sqrt();
            let assign28270_e33760: f64 = (assign28270_e33749 + assign28270_e33759);
            let assign28270_e33761: f64 = (assign28270_e33760).ln();
            let assign28270_e33762: f64 = (locals.var_phitd * assign28270_e33761);
            let assign28270_e33763: f64 = (2.0 * assign28270_e33762);
            let assign28270_e33764: f64 = (assign28270_e33742 + assign28270_e33763);
            locals.var_two_psistar = assign28270_e33764;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign28280_e33774: f64 = (locals.var_vbimin_d - locals.var_two_psistar);
            locals.var_vjlim = assign28280_e33774;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign28290_e33785: f64 = (locals.var_v1 + locals.var_vjlim);
            let assign28290_e33788: f64 = (locals.var_v1 - locals.var_vjlim);
            let assign28290_e33791: f64 = (locals.var_v1 - locals.var_vjlim);
            let assign28290_e33792: f64 = (assign28290_e33788 * assign28290_e33791);
            let assign28290_e33795: f64 = (4.0 * locals.var_phitd);
            let assign28290_e33797: f64 = (assign28290_e33795 * locals.var_phitd);
            let assign28290_e33798: f64 = (assign28290_e33792 + assign28290_e33797);
            let assign28290_e33799: f64 = (assign28290_e33798).sqrt();
            let assign28290_e33800: f64 = (assign28290_e33785 - assign28290_e33799);
            let assign28290_e33801: f64 = (0.5 * assign28290_e33800);
            locals.var_vjsrh = assign28290_e33801;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign28300_e33812: f64 = (locals.var_v1 + locals.var_vbbtlim_d);
            let assign28300_e33815: f64 = (locals.var_v1 - locals.var_vbbtlim_d);
            let assign28300_e33818: f64 = (locals.var_v1 - locals.var_vbbtlim_d);
            let assign28300_e33819: f64 = (assign28300_e33815 * assign28300_e33818);
            let assign28300_e33822: f64 = (4.0 * locals.var_phitr);
            let assign28300_e33824: f64 = (assign28300_e33822 * locals.var_phitr);
            let assign28300_e33825: f64 = (assign28300_e33819 + assign28300_e33824);
            let assign28300_e33826: f64 = (assign28300_e33825).sqrt();
            let assign28300_e33827: f64 = (assign28300_e33812 - assign28300_e33826);
            let assign28300_e33828: f64 = (0.5 * assign28300_e33827);
            locals.var_vbbt = assign28300_e33828;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign28310_e33839: f64 = locals.var_v1;
            let assign28310_e33842: f64 = locals.var_v1;
            let assign28310_e33845: f64 = locals.var_v1;
            let assign28310_e33846: f64 = (assign28310_e33842 * assign28310_e33845);
            let assign28310_e33849: f64 = (4.0 * 1e-6);
            let assign28310_e33851: f64 = (assign28310_e33849 * 1e-6);
            let assign28310_e33852: f64 = (assign28310_e33846 + assign28310_e33851);
            let assign28310_e33853: f64 = (assign28310_e33852).sqrt();
            let assign28310_e33854: f64 = (assign28310_e33839 - assign28310_e33853);
            let assign28310_e33855: f64 = (0.5 * assign28310_e33854);
            locals.var_vav = assign28310_e33855;
        }
        let assign28320_e33860: f64 = if locals.var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard543 = assign28320_e33860;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 != 0.0)) {
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) {
            let assign28340_e33877: f64 = (locals.var_idsatbot_d * locals.var_idmult);
            locals.var_id__blk219 = assign28340_e33877;
        }
        let assign28350_e33886: f64 = if ((locals.var_csrhbotd_i == 0.0) && (locals.var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard544 = assign28350_e33886;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard544 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard544 == 0.0)) {
            let assign28370_e33909: f64 = (locals.var_vbibot_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign28370_e33909;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard544 == 0.0)) {
            let assign28380_e33925: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign28380_e33926: f64 = (1.0 - assign28380_e33925);
            let assign28380_e33927: f64 = (assign28380_e33926).sqrt();
            let assign28380_e33928: f64 = (1.0 - assign28380_e33927);
            locals.var_wsrhstep = assign28380_e33928;
        }
        let assign28390_e33933: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign28390_e33933;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 == 0.0)) {
            let assign28410_e33962: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign28410_e33964: f64 = (locals.var_wsrhstep).ln();
            let assign28410_e33965: f64 = (assign28410_e33962 * assign28410_e33964);
            let assign28410_e33968: f64 = (1.0 - locals.var_wsrhstep);
            let assign28410_e33969: f64 = (assign28410_e33965 / assign28410_e33968);
            let assign28410_e33971: f64 = (assign28410_e33969 + locals.var_wsrhstep);
            let assign28410_e33975: f64 = (2.0 * locals.var_pbotd_i);
            let assign28410_e33976: f64 = (1.0 - assign28410_e33975);
            let assign28410_e33977: f64 = (assign28410_e33971 * assign28410_e33976);
            locals.var_dwsrh = assign28410_e33977;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard544 == 0.0)) {
            let assign28420_e33991: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign28420_e33991;
        }
        let assign28430_e33996: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard546 = assign28430_e33996;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard544 == 0.0)) && (locals.var_guard546 != 0.0)) {
            let assign28440_e34010: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv_d);
            let assign28440_e34011: f64 = (assign28440_e34010).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28440_e34011, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard544 == 0.0)) && (locals.var_guard546 == 0.0)) {
            let assign28450_e34028: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv_d);
            let assign28450_e34030: f64 = (assign28450_e34028).powf(locals.var_pbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28450_e34030, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard544 == 0.0)) {
            let assign28460_e34044: f64 = (locals.var_wdepnulrbot_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign28460_e34044, (locals.var_wdepnulrbot_d * locals.var_tmp_dn5), (locals.var_wdepnulrbot_d * locals.var_tmp_dn6), (locals.var_wdepnulrbot_d * locals.var_tmp_dn7), (locals.var_wdepnulrbot_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard544 == 0.0)) {
            let assign28470_e34059: f64 = (locals.var_zinv - 1.0);
            let assign28470_e34061: f64 = (assign28470_e34059 * locals.var_wdep);
            let assign28470_e34062: f64 = (locals.var_ftdbot_d * assign28470_e34061);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign28470_e34062, (locals.var_ftdbot_d * (assign28470_e34059 * locals.var_wdep_dn5)), (locals.var_ftdbot_d * (assign28470_e34059 * locals.var_wdep_dn6)), (locals.var_ftdbot_d * (assign28470_e34059 * locals.var_wdep_dn7)), (locals.var_ftdbot_d * (assign28470_e34059 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard544 == 0.0)) {
            let assign28480_e34077: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign28480_e34078: f64 = (locals.var_csrhbotd_i * assign28480_e34077);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign28480_e34078, (locals.var_csrhbotd_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign28490_e34083: f64 = if locals.var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard547 = assign28490_e34083;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28510_e34107: f64 = (locals.var_wdep * locals.var_one_minus_pbot_d);
            let assign28510_e34109: f64 = (assign28510_e34107 / locals.var_vbi_minus_vjsrh);
            let assign28510_e34110: f64 = (locals.var_btatpartbot_d * assign28510_e34109);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign28510_e34110, (locals.var_btatpartbot_d * ((locals.var_wdep_dn5 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn6 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn7 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn8 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28520_e34124: f64 = (0.666666666666667 * locals.var_atatbot_d);
            let assign28520_e34126: f64 = (assign28520_e34124 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign28520_e34126, (-((assign28520_e34124 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign28520_e34124 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign28520_e34124 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign28520_e34124 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28530_e34140: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign28530_e34140, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28540_e34154: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign28540_e34157: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign28540_e34159: f64 = (assign28540_e34157 + 1.0);
            let assign28540_e34160: f64 = (assign28540_e34154 / assign28540_e34159);
            let assign28540_e34161: f64 = (assign28540_e34160).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign28540_e34161, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign28540_e34159) - (assign28540_e34154 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign28540_e34159 * assign28540_e34159)) / (2.0 * assign28540_e34161)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign28540_e34159) - (assign28540_e34154 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign28540_e34159 * assign28540_e34159)) / (2.0 * assign28540_e34161)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign28540_e34159) - (assign28540_e34154 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign28540_e34159 * assign28540_e34159)) / (2.0 * assign28540_e34161)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign28540_e34159) - (assign28540_e34154 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign28540_e34159 * assign28540_e34159)) / (2.0 * assign28540_e34161)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28550_e34174: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign28550_e34174, (locals.var_umax_dn5 / (2.0 * assign28550_e34174)), (locals.var_umax_dn6 / (2.0 * assign28550_e34174)), (locals.var_umax_dn7 / (2.0 * assign28550_e34174)), (locals.var_umax_dn8 / (2.0 * assign28550_e34174)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28560_e34188: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign28560_e34188, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign28570_e34192: f64 = (-locals.var_pbotd_i);
        let assign28570_e34194: f64 = (assign28570_e34192 * locals.var_one_over_one_minus_pbot_d);
        let assign28570_e34196: f64 = (-1.0);
        let assign28570_e34197: f64 = if assign28570_e34194 == assign28570_e34196 { 1.0 } else { 0.0 };
        locals.var_guard548 = assign28570_e34197;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) && (locals.var_guard548 != 0.0)) {
            let assign28580_e34213: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign28580_e34214: f64 = (1.0 + assign28580_e34213);
            let assign28580_e34215: f64 = (1.0 / assign28580_e34214);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign28580_e34215, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign28580_e34214 * assign28580_e34214))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign28580_e34214 * assign28580_e34214))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign28580_e34214 * assign28580_e34214))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign28580_e34214 * assign28580_e34214))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) && (locals.var_guard548 == 0.0)) {
            let assign28590_e34233: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign28590_e34234: f64 = (1.0 + assign28590_e34233);
            let assign28590_e34236: f64 = (-locals.var_pbotd_i);
            let assign28590_e34238: f64 = (assign28590_e34236 * locals.var_one_over_one_minus_pbot_d);
            let assign28590_e34239: f64 = (assign28590_e34234).powf(assign28590_e34238);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign28590_e34239, if 0.0 == 0.0 && ((assign28590_e34238) as f64).is_finite() && ((assign28590_e34238) as f64).fract() == 0.0 { if assign28590_e34238 == 0.0 { 0.0 } else { (assign28590_e34238 * ((assign28590_e34234).powf(assign28590_e34238 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign28590_e34239 * (assign28590_e34238 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign28590_e34234))) }, if 0.0 == 0.0 && ((assign28590_e34238) as f64).is_finite() && ((assign28590_e34238) as f64).fract() == 0.0 { if assign28590_e34238 == 0.0 { 0.0 } else { (assign28590_e34238 * ((assign28590_e34234).powf(assign28590_e34238 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign28590_e34239 * (assign28590_e34238 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign28590_e34234))) }, if 0.0 == 0.0 && ((assign28590_e34238) as f64).is_finite() && ((assign28590_e34238) as f64).fract() == 0.0 { if assign28590_e34238 == 0.0 { 0.0 } else { (assign28590_e34238 * ((assign28590_e34234).powf(assign28590_e34238 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign28590_e34239 * (assign28590_e34238 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign28590_e34234))) }, if 0.0 == 0.0 && ((assign28590_e34238) as f64).is_finite() && ((assign28590_e34238) as f64).fract() == 0.0 { if assign28590_e34238 == 0.0 { 0.0 } else { (assign28590_e34238 * ((assign28590_e34234).powf(assign28590_e34238 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign28590_e34239 * (assign28590_e34238 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign28590_e34234))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28600_e34253: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign28600_e34256: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign28600_e34257: f64 = (assign28600_e34253 / assign28600_e34256);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign28600_e34257, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign28600_e34256) - (assign28600_e34253 * locals.var_wgamma_dn5)) / (assign28600_e34256 * assign28600_e34256)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign28600_e34256) - (assign28600_e34253 * locals.var_wgamma_dn6)) / (assign28600_e34256 * assign28600_e34256)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign28600_e34256) - (assign28600_e34253 * locals.var_wgamma_dn7)) / (assign28600_e34256 * assign28600_e34256)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign28600_e34256) - (assign28600_e34253 * locals.var_wgamma_dn8)) / (assign28600_e34256 * assign28600_e34256)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28610_e34272: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign28610_e34273: f64 = (0.375 * assign28610_e34272);
            let assign28610_e34274: f64 = (assign28610_e34273).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign28610_e34274, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign28610_e34274)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign28610_e34274)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign28610_e34274)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign28610_e34274)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28620_e34289: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign28620_e34290: f64 = (2.0 * assign28620_e34289);
            let assign28620_e34292: f64 = (assign28620_e34290 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign28620_e34292, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28630_e34306: f64 = (locals.var_atatbot_d * locals.var_twoatatoverthreebtat);
            let assign28630_e34308: f64 = (assign28630_e34306 * locals.var_sqrtumax);
            let assign28630_e34311: f64 = (locals.var_atatbot_d * locals.var_umax);
            let assign28630_e34312: f64 = (assign28630_e34308 - assign28630_e34311);
            let assign28630_e34316: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign28630_e34317: f64 = (0.5 * assign28630_e34316);
            let assign28630_e34318: f64 = (assign28630_e34312 + assign28630_e34317);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign28630_e34318, (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign28630_e34306 * locals.var_sqrtumax_dn5)) - (locals.var_atatbot_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign28630_e34306 * locals.var_sqrtumax_dn6)) - (locals.var_atatbot_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign28630_e34306 * locals.var_sqrtumax_dn7)) - (locals.var_atatbot_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign28630_e34306 * locals.var_sqrtumax_dn8)) - (locals.var_atatbot_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28640_e34332: f64 = (locals.var_ltat - 1.0);
            let assign28640_e34334: f64 = (assign28640_e34332 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign28640_e34334, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign28640_e34332 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign28640_e34332 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign28640_e34332 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign28640_e34332 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28650_e34348: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign28650_e34348, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign28660_e34353: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard549 = assign28660_e34353;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) && (locals.var_guard549 != 0.0)) {
            let assign28670_e34369: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign28670_e34370: f64 = (1.0 + assign28670_e34369);
            let assign28670_e34371: f64 = (1.0 / assign28670_e34370);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign28670_e34371, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign28670_e34370 * assign28670_e34370))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign28670_e34370 * assign28670_e34370))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign28670_e34370 * assign28670_e34370))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign28670_e34370 * assign28670_e34370))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) && (locals.var_guard549 == 0.0)) {
            let assign28680_e34390: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign28680_e34391: f64 = (1.0 - assign28680_e34390);
            let assign28680_e34392: f64 = (1.0 / assign28680_e34391);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign28680_e34392, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign28680_e34391 * assign28680_e34391))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign28680_e34391 * assign28680_e34391))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign28680_e34391 * assign28680_e34391))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign28680_e34391 * assign28680_e34391))), );
        }
        let assign28690_e34396: f64 = (-locals.var_ysq);
        let assign28690_e34398: f64 = (assign28690_e34396 + locals.var_mtat);
        let assign28690_e34400: f64 = (-230.25850929940458);
        let assign28690_e34401: f64 = if assign28690_e34398 > assign28690_e34400 { 1.0 } else { 0.0 };
        locals.var_guard550 = assign28690_e34401;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) && (locals.var_guard550 != 0.0)) {
            let assign28700_e34414: f64 = (-locals.var_ysq);
            let assign28700_e34416: f64 = (assign28700_e34414 + locals.var_mtat);
            let assign28700_e34417: f64 = (assign28700_e34416).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28700_e34417, (assign28700_e34417 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign28700_e34417 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign28700_e34417 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign28700_e34417 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) && (locals.var_guard550 == 0.0)) {
            let assign28710_e34435: f64 = (-230.25850929940458);
            let assign28710_e34437: f64 = (-locals.var_ysq);
            let assign28710_e34439: f64 = (assign28710_e34437 + locals.var_mtat);
            let assign28710_e34440: f64 = (assign28710_e34435 - assign28710_e34439);
            let assign28710_e34444: f64 = (-230.25850929940458);
            let assign28710_e34446: f64 = (-locals.var_ysq);
            let assign28710_e34448: f64 = (assign28710_e34446 + locals.var_mtat);
            let assign28710_e34449: f64 = (assign28710_e34444 - assign28710_e34448);
            let assign28710_e34452: f64 = (-230.25850929940458);
            let assign28710_e34454: f64 = (-locals.var_ysq);
            let assign28710_e34456: f64 = (assign28710_e34454 + locals.var_mtat);
            let assign28710_e34457: f64 = (assign28710_e34452 - assign28710_e34456);
            let assign28710_e34459: f64 = (assign28710_e34457 * 0.3333333333333333);
            let assign28710_e34460: f64 = (1.0 + assign28710_e34459);
            let assign28710_e34461: f64 = (assign28710_e34449 * assign28710_e34460);
            let assign28710_e34462: f64 = (0.5 * assign28710_e34461);
            let assign28710_e34463: f64 = (1.0 + assign28710_e34462);
            let assign28710_e34464: f64 = (assign28710_e34440 * assign28710_e34463);
            let assign28710_e34465: f64 = (1.0 + assign28710_e34464);
            let assign28710_e34466: f64 = (1e-100 / assign28710_e34465);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28710_e34466, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign28710_e34463) + (assign28710_e34440 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign28710_e34460) + (assign28710_e34449 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign28710_e34465 * assign28710_e34465))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign28710_e34463) + (assign28710_e34440 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign28710_e34460) + (assign28710_e34449 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign28710_e34465 * assign28710_e34465))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign28710_e34463) + (assign28710_e34440 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign28710_e34460) + (assign28710_e34449 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign28710_e34465 * assign28710_e34465))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign28710_e34463) + (assign28710_e34440 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign28710_e34460) + (assign28710_e34449 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign28710_e34465 * assign28710_e34465))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28720_e34480: f64 = (0.29214664 * locals.var_terfc);
            let assign28720_e34484: f64 = (locals.var_terfc * locals.var_terfc);
            let assign28720_e34485: f64 = (locals.var_berfc * assign28720_e34484);
            let assign28720_e34486: f64 = (assign28720_e34480 + assign28720_e34485);
            let assign28720_e34490: f64 = (locals.var_terfc * locals.var_terfc);
            let assign28720_e34492: f64 = (assign28720_e34490 * locals.var_terfc);
            let assign28720_e34493: f64 = (locals.var_cerfc * assign28720_e34492);
            let assign28720_e34494: f64 = (assign28720_e34486 + assign28720_e34493);
            let assign28720_e34496: f64 = (assign28720_e34494 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign28720_e34496, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign28720_e34490 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign28720_e34494 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign28720_e34490 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign28720_e34494 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign28720_e34490 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign28720_e34494 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign28720_e34490 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign28720_e34494 * locals.var_tmp_dn8)), );
        }
        let assign28730_e34501: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard551 = assign28730_e34501;
    }
    pub(super) fn stamp_transient_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) && (locals.var_guard551 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign28750_e34518: f64 = (-230.25850929940458);
        let assign28750_e34519: f64 = if locals.var_mtat > assign28750_e34518 { 1.0 } else { 0.0 };
        locals.var_guard552 = assign28750_e34519;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) && (locals.var_guard551 == 0.0)) && (locals.var_guard552 != 0.0)) {
            let assign28760_e34535: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28760_e34535, (assign28760_e34535 * locals.var_mtat_dn5), (assign28760_e34535 * locals.var_mtat_dn6), (assign28760_e34535 * locals.var_mtat_dn7), (assign28760_e34535 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) && (locals.var_guard551 == 0.0)) && (locals.var_guard552 == 0.0)) {
            let assign28770_e34556: f64 = (-230.25850929940458);
            let assign28770_e34558: f64 = (assign28770_e34556 - locals.var_mtat);
            let assign28770_e34562: f64 = (-230.25850929940458);
            let assign28770_e34564: f64 = (assign28770_e34562 - locals.var_mtat);
            let assign28770_e34567: f64 = (-230.25850929940458);
            let assign28770_e34569: f64 = (assign28770_e34567 - locals.var_mtat);
            let assign28770_e34571: f64 = (assign28770_e34569 * 0.3333333333333333);
            let assign28770_e34572: f64 = (1.0 + assign28770_e34571);
            let assign28770_e34573: f64 = (assign28770_e34564 * assign28770_e34572);
            let assign28770_e34574: f64 = (0.5 * assign28770_e34573);
            let assign28770_e34575: f64 = (1.0 + assign28770_e34574);
            let assign28770_e34576: f64 = (assign28770_e34558 * assign28770_e34575);
            let assign28770_e34577: f64 = (1.0 + assign28770_e34576);
            let assign28770_e34578: f64 = (1e-100 / assign28770_e34577);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28770_e34578, (-((1e-100 * (((-locals.var_mtat_dn5) * assign28770_e34575) + (assign28770_e34558 * (0.5 * (((-locals.var_mtat_dn5) * assign28770_e34572) + (assign28770_e34564 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign28770_e34577 * assign28770_e34577))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign28770_e34575) + (assign28770_e34558 * (0.5 * (((-locals.var_mtat_dn6) * assign28770_e34572) + (assign28770_e34564 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign28770_e34577 * assign28770_e34577))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign28770_e34575) + (assign28770_e34558 * (0.5 * (((-locals.var_mtat_dn7) * assign28770_e34572) + (assign28770_e34564 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign28770_e34577 * assign28770_e34577))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign28770_e34575) + (assign28770_e34558 * (0.5 * (((-locals.var_mtat_dn8) * assign28770_e34572) + (assign28770_e34564 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign28770_e34577 * assign28770_e34577))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) && (locals.var_guard551 == 0.0)) {
            let assign28780_e34595: f64 = (2.0 * locals.var_tmp);
            let assign28780_e34597: f64 = (assign28780_e34595 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign28780_e34597, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28790_e34611: f64 = (1.772453850905516 * 0.5);
            let assign28790_e34614: f64 = (locals.var_atatbot_d * locals.var_erfctimesexpmtat);
            let assign28790_e34616: f64 = (assign28790_e34614 / locals.var_ktat);
            let assign28790_e34617: f64 = (assign28790_e34611 * assign28790_e34616);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign28790_e34617, (assign28790_e34611 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign28790_e34614 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign28790_e34611 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign28790_e34614 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign28790_e34611 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign28790_e34614 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign28790_e34611 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign28790_e34614 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard547 == 0.0)) {
            let assign28800_e34632: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign28800_e34634: f64 = (assign28800_e34632 * locals.var_wtat);
            let assign28800_e34635: f64 = (locals.var_ctatbotd_i * assign28800_e34634);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign28800_e34635, (locals.var_ctatbotd_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign28800_e34632 * locals.var_wtat_dn5))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign28800_e34632 * locals.var_wtat_dn6))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign28800_e34632 * locals.var_wtat_dn7))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign28800_e34632 * locals.var_wtat_dn8))), );
        }
        let assign28810_e34640: f64 = if locals.var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard553 = assign28810_e34640;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard553 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign28830_e34654: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard554 = assign28830_e34654;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard553 == 0.0)) && (locals.var_guard554 != 0.0)) {
            let assign28840_e34668: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign28840_e34670: f64 = (assign28840_e34668 * locals.var_vbirbotinv_d);
            let assign28840_e34671: f64 = (assign28840_e34670).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28840_e34671, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard553 == 0.0)) && (locals.var_guard554 == 0.0)) {
            let assign28850_e34688: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign28850_e34690: f64 = (assign28850_e34688 * locals.var_vbirbotinv_d);
            let assign28850_e34692: f64 = (assign28850_e34690).powf(locals.var_pbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28850_e34692, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard553 == 0.0)) {
            let assign28860_e34707: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign28860_e34709: f64 = (assign28860_e34707 * locals.var_wdepnulrinvbot_d);
            let assign28860_e34711: f64 = (assign28860_e34709 / locals.var_tmp);
            let assign28860_e34712: f64 = (locals.var_one_over_one_minus_pbot_d * assign28860_e34711);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign28860_e34712, (locals.var_one_over_one_minus_pbot_d * (-((assign28860_e34709 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign28860_e34709 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign28860_e34709 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign28860_e34709 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign28870_e34716: f64 = (-locals.var_fbbtbot_d);
        let assign28870_e34718: f64 = (assign28870_e34716 / locals.var_fmaxr);
        let assign28870_e34719: f64 = (assign28870_e34718).abs();
        let assign28870_e34721: f64 = if assign28870_e34719 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard555 = assign28870_e34721;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard553 == 0.0)) && (locals.var_guard555 != 0.0)) {
            let assign28880_e34734: f64 = (-locals.var_fbbtbot_d);
            let assign28880_e34736: f64 = (assign28880_e34734 / locals.var_fmaxr);
            let assign28880_e34737: f64 = (assign28880_e34736).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28880_e34737, (assign28880_e34737 * (-((assign28880_e34734 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign28880_e34737 * (-((assign28880_e34734 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign28880_e34737 * (-((assign28880_e34734 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign28880_e34737 * (-((assign28880_e34734 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign28890_e34741: f64 = (-locals.var_fbbtbot_d);
        let assign28890_e34743: f64 = (assign28890_e34741 / locals.var_fmaxr);
        let assign28890_e34745: f64 = if assign28890_e34743 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard556 = assign28890_e34745;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard553 == 0.0)) && (locals.var_guard555 == 0.0)) && (locals.var_guard556 != 0.0)) {
            let assign28900_e34763: f64 = (-230.25850929940458);
            let assign28900_e34765: f64 = (-locals.var_fbbtbot_d);
            let assign28900_e34767: f64 = (assign28900_e34765 / locals.var_fmaxr);
            let assign28900_e34768: f64 = (assign28900_e34763 - assign28900_e34767);
            let assign28900_e34772: f64 = (-230.25850929940458);
            let assign28900_e34774: f64 = (-locals.var_fbbtbot_d);
            let assign28900_e34776: f64 = (assign28900_e34774 / locals.var_fmaxr);
            let assign28900_e34777: f64 = (assign28900_e34772 - assign28900_e34776);
            let assign28900_e34780: f64 = (-230.25850929940458);
            let assign28900_e34782: f64 = (-locals.var_fbbtbot_d);
            let assign28900_e34784: f64 = (assign28900_e34782 / locals.var_fmaxr);
            let assign28900_e34785: f64 = (assign28900_e34780 - assign28900_e34784);
            let assign28900_e34787: f64 = (assign28900_e34785 * 0.3333333333333333);
            let assign28900_e34788: f64 = (1.0 + assign28900_e34787);
            let assign28900_e34789: f64 = (assign28900_e34777 * assign28900_e34788);
            let assign28900_e34790: f64 = (0.5 * assign28900_e34789);
            let assign28900_e34791: f64 = (1.0 + assign28900_e34790);
            let assign28900_e34792: f64 = (assign28900_e34768 * assign28900_e34791);
            let assign28900_e34793: f64 = (1.0 + assign28900_e34792);
            let assign28900_e34794: f64 = (1e-100 / assign28900_e34793);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28900_e34794, (-((1e-100 * (((-(-((assign28900_e34765 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign28900_e34791) + (assign28900_e34768 * (0.5 * (((-(-((assign28900_e34774 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign28900_e34788) + (assign28900_e34777 * ((-(-((assign28900_e34782 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign28900_e34793 * assign28900_e34793))), (-((1e-100 * (((-(-((assign28900_e34765 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign28900_e34791) + (assign28900_e34768 * (0.5 * (((-(-((assign28900_e34774 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign28900_e34788) + (assign28900_e34777 * ((-(-((assign28900_e34782 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign28900_e34793 * assign28900_e34793))), (-((1e-100 * (((-(-((assign28900_e34765 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign28900_e34791) + (assign28900_e34768 * (0.5 * (((-(-((assign28900_e34774 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign28900_e34788) + (assign28900_e34777 * ((-(-((assign28900_e34782 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign28900_e34793 * assign28900_e34793))), (-((1e-100 * (((-(-((assign28900_e34765 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign28900_e34791) + (assign28900_e34768 * (0.5 * (((-(-((assign28900_e34774 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign28900_e34788) + (assign28900_e34777 * ((-(-((assign28900_e34782 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign28900_e34793 * assign28900_e34793))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard553 == 0.0)) && (locals.var_guard555 == 0.0)) && (locals.var_guard556 == 0.0)) {
            let assign28910_e34815: f64 = (-locals.var_fbbtbot_d);
            let assign28910_e34817: f64 = (assign28910_e34815 / locals.var_fmaxr);
            let assign28910_e34819: f64 = (assign28910_e34817 - 230.25850929940458);
            let assign28910_e34823: f64 = (-locals.var_fbbtbot_d);
            let assign28910_e34825: f64 = (assign28910_e34823 / locals.var_fmaxr);
            let assign28910_e34827: f64 = (assign28910_e34825 - 230.25850929940458);
            let assign28910_e34830: f64 = (-locals.var_fbbtbot_d);
            let assign28910_e34832: f64 = (assign28910_e34830 / locals.var_fmaxr);
            let assign28910_e34834: f64 = (assign28910_e34832 - 230.25850929940458);
            let assign28910_e34836: f64 = (assign28910_e34834 * 0.3333333333333333);
            let assign28910_e34837: f64 = (1.0 + assign28910_e34836);
            let assign28910_e34838: f64 = (assign28910_e34827 * assign28910_e34837);
            let assign28910_e34839: f64 = (0.5 * assign28910_e34838);
            let assign28910_e34840: f64 = (1.0 + assign28910_e34839);
            let assign28910_e34841: f64 = (assign28910_e34819 * assign28910_e34840);
            let assign28910_e34842: f64 = (1.0 + assign28910_e34841);
            let assign28910_e34843: f64 = (1e100 * assign28910_e34842);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28910_e34843, (1e100 * (((-((assign28910_e34815 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign28910_e34840) + (assign28910_e34819 * (0.5 * (((-((assign28910_e34823 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign28910_e34837) + (assign28910_e34827 * ((-((assign28910_e34830 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign28910_e34815 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign28910_e34840) + (assign28910_e34819 * (0.5 * (((-((assign28910_e34823 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign28910_e34837) + (assign28910_e34827 * ((-((assign28910_e34830 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign28910_e34815 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign28910_e34840) + (assign28910_e34819 * (0.5 * (((-((assign28910_e34823 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign28910_e34837) + (assign28910_e34827 * ((-((assign28910_e34830 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign28910_e34815 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign28910_e34840) + (assign28910_e34819 * (0.5 * (((-((assign28910_e34823 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign28910_e34837) + (assign28910_e34827 * ((-((assign28910_e34830 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard553 == 0.0)) {
            let assign28920_e34858: f64 = (locals.var_v1 * locals.var_fmaxr);
            let assign28920_e34860: f64 = (assign28920_e34858 * locals.var_fmaxr);
            let assign28920_e34862: f64 = (assign28920_e34860 * locals.var_tmp);
            let assign28920_e34863: f64 = (locals.var_cbbtbotd_i * assign28920_e34862);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign28920_e34863, (locals.var_cbbtbotd_i * (((((locals.var_v1 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign28920_e34858 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign28920_e34860 * locals.var_tmp_dn5))), (locals.var_cbbtbotd_i * (((((locals.var_v1 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign28920_e34858 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign28920_e34860 * locals.var_tmp_dn6))), (locals.var_cbbtbotd_i * (((((locals.var_v1 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign28920_e34858 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign28920_e34860 * locals.var_tmp_dn7))), (locals.var_cbbtbotd_i * (((((locals.var_v1 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign28920_e34858 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign28920_e34860 * locals.var_tmp_dn8))), );
        }
        let assign28930_e34868: f64 = if locals.var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard557 = assign28930_e34868;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard557 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign28950_e34882: f64 = (-locals.var_alphaav);
        let assign28950_e34884: f64 = (assign28950_e34882 * locals.var_vbrbotd_i);
        let assign28950_e34885: f64 = if locals.var_vav > assign28950_e34884 { 1.0 } else { 0.0 };
        locals.var_guard558 = assign28950_e34885;
        let assign28960_e34888: f64 = if locals.var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard559 = assign28960_e34888;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard557 == 0.0)) && (locals.var_guard558 != 0.0)) && (locals.var_guard559 != 0.0)) {
            let assign28970_e34904: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign28970_e34907: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign28970_e34908: f64 = (assign28970_e34904 * assign28970_e34907);
            let assign28970_e34911: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign28970_e34912: f64 = (assign28970_e34908 * assign28970_e34911);
            let assign28970_e34915: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign28970_e34916: f64 = (assign28970_e34912 * assign28970_e34915);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28970_e34916, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard557 == 0.0)) && (locals.var_guard558 != 0.0)) && (locals.var_guard559 == 0.0)) {
            let assign28980_e34935: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign28980_e34936: f64 = (assign28980_e34935).abs();
            let assign28980_e34938: f64 = (assign28980_e34936).powf(locals.var_pbrbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign28980_e34938, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard557 == 0.0)) && (locals.var_guard558 != 0.0)) {
            let assign28990_e34955: f64 = (1.0 - locals.var_tmp);
            let assign28990_e34956: f64 = (1.0 / assign28990_e34955);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign28990_e34956, (-((-locals.var_tmp_dn5) / (assign28990_e34955 * assign28990_e34955))), (-((-locals.var_tmp_dn6) / (assign28990_e34955 * assign28990_e34955))), (-((-locals.var_tmp_dn7) / (assign28990_e34955 * assign28990_e34955))), (-((-locals.var_tmp_dn8) / (assign28990_e34955 * assign28990_e34955))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard557 == 0.0)) && (locals.var_guard558 == 0.0)) {
            let assign29000_e34975: f64 = (locals.var_alphaav * locals.var_vbrbotd_i);
            let assign29000_e34976: f64 = (locals.var_vav + assign29000_e34975);
            let assign29000_e34978: f64 = (assign29000_e34976 * locals.var_slopebot_d);
            let assign29000_e34979: f64 = (locals.var_fstopbot_d + assign29000_e34978);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign29000_e34979, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard543 == 0.0)) {
            let assign29010_e34991: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign29010_e34993: f64 = (assign29010_e34991 + locals.var_itat);
            let assign29010_e34995: f64 = (assign29010_e34993 + locals.var_ibbt);
            let assign29010_e34996: f64 = (p.p29 * assign29010_e34995);
            let assign29010_e34998: f64 = (assign29010_e34996 * locals.var_fbreakdown);
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (assign29010_e34998, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign29010_e34996 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign29010_e34996 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign29010_e34996 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign29010_e34996 * locals.var_fbreakdown_dn8)), );
        }
        let assign29020_e35003: f64 = if locals.var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard560 = assign29020_e35003;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 != 0.0)) {
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) {
            let assign29040_e35020: f64 = (locals.var_idsatsti_d * locals.var_idmult);
            locals.var_id__blk219 = assign29040_e35020;
        }
        let assign29050_e35029: f64 = if ((locals.var_csrhstid_i == 0.0) && (locals.var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard561 = assign29050_e35029;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 == 0.0)) {
            let assign29070_e35052: f64 = (locals.var_vbisti_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign29070_e35052;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 == 0.0)) {
            let assign29080_e35068: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign29080_e35069: f64 = (1.0 - assign29080_e35068);
            let assign29080_e35070: f64 = (assign29080_e35069).sqrt();
            let assign29080_e35071: f64 = (1.0 - assign29080_e35070);
            locals.var_wsrhstep = assign29080_e35071;
        }
        let assign29090_e35076: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard562 = assign29090_e35076;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 == 0.0)) && (locals.var_guard562 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 == 0.0)) && (locals.var_guard562 == 0.0)) {
            let assign29110_e35105: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign29110_e35107: f64 = (locals.var_wsrhstep).ln();
            let assign29110_e35108: f64 = (assign29110_e35105 * assign29110_e35107);
            let assign29110_e35111: f64 = (1.0 - locals.var_wsrhstep);
            let assign29110_e35112: f64 = (assign29110_e35108 / assign29110_e35111);
            let assign29110_e35114: f64 = (assign29110_e35112 + locals.var_wsrhstep);
            let assign29110_e35118: f64 = (2.0 * locals.var_pstid_i);
            let assign29110_e35119: f64 = (1.0 - assign29110_e35118);
            let assign29110_e35120: f64 = (assign29110_e35114 * assign29110_e35119);
            locals.var_dwsrh = assign29110_e35120;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 == 0.0)) {
            let assign29120_e35134: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign29120_e35134;
        }
        let assign29130_e35139: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard563 = assign29130_e35139;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 == 0.0)) && (locals.var_guard563 != 0.0)) {
            let assign29140_e35153: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv_d);
            let assign29140_e35154: f64 = (assign29140_e35153).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29140_e35154, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 == 0.0)) && (locals.var_guard563 == 0.0)) {
            let assign29150_e35171: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv_d);
            let assign29150_e35173: f64 = (assign29150_e35171).powf(locals.var_pstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29150_e35173, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 == 0.0)) {
            let assign29160_e35187: f64 = (locals.var_wdepnulrsti_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign29160_e35187, (locals.var_wdepnulrsti_d * locals.var_tmp_dn5), (locals.var_wdepnulrsti_d * locals.var_tmp_dn6), (locals.var_wdepnulrsti_d * locals.var_tmp_dn7), (locals.var_wdepnulrsti_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 == 0.0)) {
            let assign29170_e35202: f64 = (locals.var_zinv - 1.0);
            let assign29170_e35204: f64 = (assign29170_e35202 * locals.var_wdep);
            let assign29170_e35205: f64 = (locals.var_ftdsti_d * assign29170_e35204);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign29170_e35205, (locals.var_ftdsti_d * (assign29170_e35202 * locals.var_wdep_dn5)), (locals.var_ftdsti_d * (assign29170_e35202 * locals.var_wdep_dn6)), (locals.var_ftdsti_d * (assign29170_e35202 * locals.var_wdep_dn7)), (locals.var_ftdsti_d * (assign29170_e35202 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 == 0.0)) {
            let assign29180_e35220: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign29180_e35221: f64 = (locals.var_csrhstid_i * assign29180_e35220);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign29180_e35221, (locals.var_csrhstid_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign29190_e35226: f64 = if locals.var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard564 = assign29190_e35226;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29210_e35250: f64 = (locals.var_wdep * locals.var_one_minus_psti_d);
            let assign29210_e35252: f64 = (assign29210_e35250 / locals.var_vbi_minus_vjsrh);
            let assign29210_e35253: f64 = (locals.var_btatpartsti_d * assign29210_e35252);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign29210_e35253, (locals.var_btatpartsti_d * ((locals.var_wdep_dn5 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn6 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn7 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn8 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29220_e35267: f64 = (0.666666666666667 * locals.var_atatsti_d);
            let assign29220_e35269: f64 = (assign29220_e35267 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign29220_e35269, (-((assign29220_e35267 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign29220_e35267 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign29220_e35267 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign29220_e35267 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29230_e35283: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign29230_e35283, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29240_e35297: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign29240_e35300: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign29240_e35302: f64 = (assign29240_e35300 + 1.0);
            let assign29240_e35303: f64 = (assign29240_e35297 / assign29240_e35302);
            let assign29240_e35304: f64 = (assign29240_e35303).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign29240_e35304, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign29240_e35302) - (assign29240_e35297 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign29240_e35302 * assign29240_e35302)) / (2.0 * assign29240_e35304)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign29240_e35302) - (assign29240_e35297 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign29240_e35302 * assign29240_e35302)) / (2.0 * assign29240_e35304)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign29240_e35302) - (assign29240_e35297 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign29240_e35302 * assign29240_e35302)) / (2.0 * assign29240_e35304)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign29240_e35302) - (assign29240_e35297 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign29240_e35302 * assign29240_e35302)) / (2.0 * assign29240_e35304)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29250_e35317: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign29250_e35317, (locals.var_umax_dn5 / (2.0 * assign29250_e35317)), (locals.var_umax_dn6 / (2.0 * assign29250_e35317)), (locals.var_umax_dn7 / (2.0 * assign29250_e35317)), (locals.var_umax_dn8 / (2.0 * assign29250_e35317)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29260_e35331: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign29260_e35331, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign29270_e35335: f64 = (-locals.var_pstid_i);
        let assign29270_e35337: f64 = (assign29270_e35335 * locals.var_one_over_one_minus_psti_d);
        let assign29270_e35339: f64 = (-1.0);
        let assign29270_e35340: f64 = if assign29270_e35337 == assign29270_e35339 { 1.0 } else { 0.0 };
        locals.var_guard565 = assign29270_e35340;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard565 != 0.0)) {
            let assign29280_e35356: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign29280_e35357: f64 = (1.0 + assign29280_e35356);
            let assign29280_e35358: f64 = (1.0 / assign29280_e35357);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign29280_e35358, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign29280_e35357 * assign29280_e35357))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign29280_e35357 * assign29280_e35357))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign29280_e35357 * assign29280_e35357))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign29280_e35357 * assign29280_e35357))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard565 == 0.0)) {
            let assign29290_e35376: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign29290_e35377: f64 = (1.0 + assign29290_e35376);
            let assign29290_e35379: f64 = (-locals.var_pstid_i);
            let assign29290_e35381: f64 = (assign29290_e35379 * locals.var_one_over_one_minus_psti_d);
            let assign29290_e35382: f64 = (assign29290_e35377).powf(assign29290_e35381);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign29290_e35382, if 0.0 == 0.0 && ((assign29290_e35381) as f64).is_finite() && ((assign29290_e35381) as f64).fract() == 0.0 { if assign29290_e35381 == 0.0 { 0.0 } else { (assign29290_e35381 * ((assign29290_e35377).powf(assign29290_e35381 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign29290_e35382 * (assign29290_e35381 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign29290_e35377))) }, if 0.0 == 0.0 && ((assign29290_e35381) as f64).is_finite() && ((assign29290_e35381) as f64).fract() == 0.0 { if assign29290_e35381 == 0.0 { 0.0 } else { (assign29290_e35381 * ((assign29290_e35377).powf(assign29290_e35381 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign29290_e35382 * (assign29290_e35381 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign29290_e35377))) }, if 0.0 == 0.0 && ((assign29290_e35381) as f64).is_finite() && ((assign29290_e35381) as f64).fract() == 0.0 { if assign29290_e35381 == 0.0 { 0.0 } else { (assign29290_e35381 * ((assign29290_e35377).powf(assign29290_e35381 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign29290_e35382 * (assign29290_e35381 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign29290_e35377))) }, if 0.0 == 0.0 && ((assign29290_e35381) as f64).is_finite() && ((assign29290_e35381) as f64).fract() == 0.0 { if assign29290_e35381 == 0.0 { 0.0 } else { (assign29290_e35381 * ((assign29290_e35377).powf(assign29290_e35381 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign29290_e35382 * (assign29290_e35381 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign29290_e35377))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29300_e35396: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign29300_e35399: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign29300_e35400: f64 = (assign29300_e35396 / assign29300_e35399);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign29300_e35400, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign29300_e35399) - (assign29300_e35396 * locals.var_wgamma_dn5)) / (assign29300_e35399 * assign29300_e35399)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign29300_e35399) - (assign29300_e35396 * locals.var_wgamma_dn6)) / (assign29300_e35399 * assign29300_e35399)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign29300_e35399) - (assign29300_e35396 * locals.var_wgamma_dn7)) / (assign29300_e35399 * assign29300_e35399)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign29300_e35399) - (assign29300_e35396 * locals.var_wgamma_dn8)) / (assign29300_e35399 * assign29300_e35399)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29310_e35415: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign29310_e35416: f64 = (0.375 * assign29310_e35415);
            let assign29310_e35417: f64 = (assign29310_e35416).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign29310_e35417, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign29310_e35417)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign29310_e35417)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign29310_e35417)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign29310_e35417)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29320_e35432: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign29320_e35433: f64 = (2.0 * assign29320_e35432);
            let assign29320_e35435: f64 = (assign29320_e35433 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign29320_e35435, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29330_e35449: f64 = (locals.var_atatsti_d * locals.var_twoatatoverthreebtat);
            let assign29330_e35451: f64 = (assign29330_e35449 * locals.var_sqrtumax);
            let assign29330_e35454: f64 = (locals.var_atatsti_d * locals.var_umax);
            let assign29330_e35455: f64 = (assign29330_e35451 - assign29330_e35454);
            let assign29330_e35459: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign29330_e35460: f64 = (0.5 * assign29330_e35459);
            let assign29330_e35461: f64 = (assign29330_e35455 + assign29330_e35460);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign29330_e35461, (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign29330_e35449 * locals.var_sqrtumax_dn5)) - (locals.var_atatsti_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign29330_e35449 * locals.var_sqrtumax_dn6)) - (locals.var_atatsti_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign29330_e35449 * locals.var_sqrtumax_dn7)) - (locals.var_atatsti_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign29330_e35449 * locals.var_sqrtumax_dn8)) - (locals.var_atatsti_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29340_e35475: f64 = (locals.var_ltat - 1.0);
            let assign29340_e35477: f64 = (assign29340_e35475 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign29340_e35477, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign29340_e35475 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign29340_e35475 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign29340_e35475 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign29340_e35475 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29350_e35491: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign29350_e35491, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign29360_e35496: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard566 = assign29360_e35496;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard566 != 0.0)) {
            let assign29370_e35512: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign29370_e35513: f64 = (1.0 + assign29370_e35512);
            let assign29370_e35514: f64 = (1.0 / assign29370_e35513);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign29370_e35514, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign29370_e35513 * assign29370_e35513))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign29370_e35513 * assign29370_e35513))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign29370_e35513 * assign29370_e35513))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign29370_e35513 * assign29370_e35513))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard566 == 0.0)) {
            let assign29380_e35533: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign29380_e35534: f64 = (1.0 - assign29380_e35533);
            let assign29380_e35535: f64 = (1.0 / assign29380_e35534);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign29380_e35535, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign29380_e35534 * assign29380_e35534))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign29380_e35534 * assign29380_e35534))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign29380_e35534 * assign29380_e35534))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign29380_e35534 * assign29380_e35534))), );
        }
        let assign29390_e35539: f64 = (-locals.var_ysq);
        let assign29390_e35541: f64 = (assign29390_e35539 + locals.var_mtat);
        let assign29390_e35543: f64 = (-230.25850929940458);
        let assign29390_e35544: f64 = if assign29390_e35541 > assign29390_e35543 { 1.0 } else { 0.0 };
        locals.var_guard567 = assign29390_e35544;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard567 != 0.0)) {
            let assign29400_e35557: f64 = (-locals.var_ysq);
            let assign29400_e35559: f64 = (assign29400_e35557 + locals.var_mtat);
            let assign29400_e35560: f64 = (assign29400_e35559).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29400_e35560, (assign29400_e35560 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign29400_e35560 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign29400_e35560 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign29400_e35560 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard567 == 0.0)) {
            let assign29410_e35578: f64 = (-230.25850929940458);
            let assign29410_e35580: f64 = (-locals.var_ysq);
            let assign29410_e35582: f64 = (assign29410_e35580 + locals.var_mtat);
            let assign29410_e35583: f64 = (assign29410_e35578 - assign29410_e35582);
            let assign29410_e35587: f64 = (-230.25850929940458);
            let assign29410_e35589: f64 = (-locals.var_ysq);
            let assign29410_e35591: f64 = (assign29410_e35589 + locals.var_mtat);
            let assign29410_e35592: f64 = (assign29410_e35587 - assign29410_e35591);
            let assign29410_e35595: f64 = (-230.25850929940458);
            let assign29410_e35597: f64 = (-locals.var_ysq);
            let assign29410_e35599: f64 = (assign29410_e35597 + locals.var_mtat);
            let assign29410_e35600: f64 = (assign29410_e35595 - assign29410_e35599);
            let assign29410_e35602: f64 = (assign29410_e35600 * 0.3333333333333333);
            let assign29410_e35603: f64 = (1.0 + assign29410_e35602);
            let assign29410_e35604: f64 = (assign29410_e35592 * assign29410_e35603);
            let assign29410_e35605: f64 = (0.5 * assign29410_e35604);
            let assign29410_e35606: f64 = (1.0 + assign29410_e35605);
            let assign29410_e35607: f64 = (assign29410_e35583 * assign29410_e35606);
            let assign29410_e35608: f64 = (1.0 + assign29410_e35607);
            let assign29410_e35609: f64 = (1e-100 / assign29410_e35608);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29410_e35609, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign29410_e35606) + (assign29410_e35583 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign29410_e35603) + (assign29410_e35592 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign29410_e35608 * assign29410_e35608))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign29410_e35606) + (assign29410_e35583 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign29410_e35603) + (assign29410_e35592 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign29410_e35608 * assign29410_e35608))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign29410_e35606) + (assign29410_e35583 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign29410_e35603) + (assign29410_e35592 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign29410_e35608 * assign29410_e35608))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign29410_e35606) + (assign29410_e35583 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign29410_e35603) + (assign29410_e35592 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign29410_e35608 * assign29410_e35608))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29420_e35623: f64 = (0.29214664 * locals.var_terfc);
            let assign29420_e35627: f64 = (locals.var_terfc * locals.var_terfc);
            let assign29420_e35628: f64 = (locals.var_berfc * assign29420_e35627);
            let assign29420_e35629: f64 = (assign29420_e35623 + assign29420_e35628);
            let assign29420_e35633: f64 = (locals.var_terfc * locals.var_terfc);
            let assign29420_e35635: f64 = (assign29420_e35633 * locals.var_terfc);
            let assign29420_e35636: f64 = (locals.var_cerfc * assign29420_e35635);
            let assign29420_e35637: f64 = (assign29420_e35629 + assign29420_e35636);
            let assign29420_e35639: f64 = (assign29420_e35637 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign29420_e35639, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign29420_e35633 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign29420_e35637 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign29420_e35633 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign29420_e35637 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign29420_e35633 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign29420_e35637 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign29420_e35633 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign29420_e35637 * locals.var_tmp_dn8)), );
        }
        let assign29430_e35644: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard568 = assign29430_e35644;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard568 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign29450_e35661: f64 = (-230.25850929940458);
        let assign29450_e35662: f64 = if locals.var_mtat > assign29450_e35661 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign29450_e35662;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard568 == 0.0)) && (locals.var_guard569 != 0.0)) {
            let assign29460_e35678: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29460_e35678, (assign29460_e35678 * locals.var_mtat_dn5), (assign29460_e35678 * locals.var_mtat_dn6), (assign29460_e35678 * locals.var_mtat_dn7), (assign29460_e35678 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard568 == 0.0)) && (locals.var_guard569 == 0.0)) {
            let assign29470_e35699: f64 = (-230.25850929940458);
            let assign29470_e35701: f64 = (assign29470_e35699 - locals.var_mtat);
            let assign29470_e35705: f64 = (-230.25850929940458);
            let assign29470_e35707: f64 = (assign29470_e35705 - locals.var_mtat);
            let assign29470_e35710: f64 = (-230.25850929940458);
            let assign29470_e35712: f64 = (assign29470_e35710 - locals.var_mtat);
            let assign29470_e35714: f64 = (assign29470_e35712 * 0.3333333333333333);
            let assign29470_e35715: f64 = (1.0 + assign29470_e35714);
            let assign29470_e35716: f64 = (assign29470_e35707 * assign29470_e35715);
            let assign29470_e35717: f64 = (0.5 * assign29470_e35716);
            let assign29470_e35718: f64 = (1.0 + assign29470_e35717);
            let assign29470_e35719: f64 = (assign29470_e35701 * assign29470_e35718);
            let assign29470_e35720: f64 = (1.0 + assign29470_e35719);
            let assign29470_e35721: f64 = (1e-100 / assign29470_e35720);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29470_e35721, (-((1e-100 * (((-locals.var_mtat_dn5) * assign29470_e35718) + (assign29470_e35701 * (0.5 * (((-locals.var_mtat_dn5) * assign29470_e35715) + (assign29470_e35707 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign29470_e35720 * assign29470_e35720))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign29470_e35718) + (assign29470_e35701 * (0.5 * (((-locals.var_mtat_dn6) * assign29470_e35715) + (assign29470_e35707 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign29470_e35720 * assign29470_e35720))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign29470_e35718) + (assign29470_e35701 * (0.5 * (((-locals.var_mtat_dn7) * assign29470_e35715) + (assign29470_e35707 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign29470_e35720 * assign29470_e35720))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign29470_e35718) + (assign29470_e35701 * (0.5 * (((-locals.var_mtat_dn8) * assign29470_e35715) + (assign29470_e35707 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign29470_e35720 * assign29470_e35720))), );
        }
    }
    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard568 == 0.0)) {
            let assign29480_e35738: f64 = (2.0 * locals.var_tmp);
            let assign29480_e35740: f64 = (assign29480_e35738 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign29480_e35740, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29490_e35754: f64 = (1.772453850905516 * 0.5);
            let assign29490_e35757: f64 = (locals.var_atatsti_d * locals.var_erfctimesexpmtat);
            let assign29490_e35759: f64 = (assign29490_e35757 / locals.var_ktat);
            let assign29490_e35760: f64 = (assign29490_e35754 * assign29490_e35759);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign29490_e35760, (assign29490_e35754 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign29490_e35757 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign29490_e35754 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign29490_e35757 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign29490_e35754 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign29490_e35757 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign29490_e35754 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign29490_e35757 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard564 == 0.0)) {
            let assign29500_e35775: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign29500_e35777: f64 = (assign29500_e35775 * locals.var_wtat);
            let assign29500_e35778: f64 = (locals.var_ctatstid_i * assign29500_e35777);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign29500_e35778, (locals.var_ctatstid_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign29500_e35775 * locals.var_wtat_dn5))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign29500_e35775 * locals.var_wtat_dn6))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign29500_e35775 * locals.var_wtat_dn7))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign29500_e35775 * locals.var_wtat_dn8))), );
        }
        let assign29510_e35783: f64 = if locals.var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard570 = assign29510_e35783;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard570 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign29530_e35797: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard571 = assign29530_e35797;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard570 == 0.0)) && (locals.var_guard571 != 0.0)) {
            let assign29540_e35811: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign29540_e35813: f64 = (assign29540_e35811 * locals.var_vbirstiinv_d);
            let assign29540_e35814: f64 = (assign29540_e35813).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29540_e35814, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard570 == 0.0)) && (locals.var_guard571 == 0.0)) {
            let assign29550_e35831: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign29550_e35833: f64 = (assign29550_e35831 * locals.var_vbirstiinv_d);
            let assign29550_e35835: f64 = (assign29550_e35833).powf(locals.var_pstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29550_e35835, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard570 == 0.0)) {
            let assign29560_e35850: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign29560_e35852: f64 = (assign29560_e35850 * locals.var_wdepnulrinvsti_d);
            let assign29560_e35854: f64 = (assign29560_e35852 / locals.var_tmp);
            let assign29560_e35855: f64 = (locals.var_one_over_one_minus_psti_d * assign29560_e35854);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign29560_e35855, (locals.var_one_over_one_minus_psti_d * (-((assign29560_e35852 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign29560_e35852 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign29560_e35852 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign29560_e35852 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign29570_e35859: f64 = (-locals.var_fbbtsti_d);
        let assign29570_e35861: f64 = (assign29570_e35859 / locals.var_fmaxr);
        let assign29570_e35862: f64 = (assign29570_e35861).abs();
        let assign29570_e35864: f64 = if assign29570_e35862 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign29570_e35864;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard570 == 0.0)) && (locals.var_guard572 != 0.0)) {
            let assign29580_e35877: f64 = (-locals.var_fbbtsti_d);
            let assign29580_e35879: f64 = (assign29580_e35877 / locals.var_fmaxr);
            let assign29580_e35880: f64 = (assign29580_e35879).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29580_e35880, (assign29580_e35880 * (-((assign29580_e35877 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign29580_e35880 * (-((assign29580_e35877 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign29580_e35880 * (-((assign29580_e35877 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign29580_e35880 * (-((assign29580_e35877 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign29590_e35884: f64 = (-locals.var_fbbtsti_d);
        let assign29590_e35886: f64 = (assign29590_e35884 / locals.var_fmaxr);
        let assign29590_e35888: f64 = if assign29590_e35886 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign29590_e35888;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard570 == 0.0)) && (locals.var_guard572 == 0.0)) && (locals.var_guard573 != 0.0)) {
            let assign29600_e35906: f64 = (-230.25850929940458);
            let assign29600_e35908: f64 = (-locals.var_fbbtsti_d);
            let assign29600_e35910: f64 = (assign29600_e35908 / locals.var_fmaxr);
            let assign29600_e35911: f64 = (assign29600_e35906 - assign29600_e35910);
            let assign29600_e35915: f64 = (-230.25850929940458);
            let assign29600_e35917: f64 = (-locals.var_fbbtsti_d);
            let assign29600_e35919: f64 = (assign29600_e35917 / locals.var_fmaxr);
            let assign29600_e35920: f64 = (assign29600_e35915 - assign29600_e35919);
            let assign29600_e35923: f64 = (-230.25850929940458);
            let assign29600_e35925: f64 = (-locals.var_fbbtsti_d);
            let assign29600_e35927: f64 = (assign29600_e35925 / locals.var_fmaxr);
            let assign29600_e35928: f64 = (assign29600_e35923 - assign29600_e35927);
            let assign29600_e35930: f64 = (assign29600_e35928 * 0.3333333333333333);
            let assign29600_e35931: f64 = (1.0 + assign29600_e35930);
            let assign29600_e35932: f64 = (assign29600_e35920 * assign29600_e35931);
            let assign29600_e35933: f64 = (0.5 * assign29600_e35932);
            let assign29600_e35934: f64 = (1.0 + assign29600_e35933);
            let assign29600_e35935: f64 = (assign29600_e35911 * assign29600_e35934);
            let assign29600_e35936: f64 = (1.0 + assign29600_e35935);
            let assign29600_e35937: f64 = (1e-100 / assign29600_e35936);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29600_e35937, (-((1e-100 * (((-(-((assign29600_e35908 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign29600_e35934) + (assign29600_e35911 * (0.5 * (((-(-((assign29600_e35917 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign29600_e35931) + (assign29600_e35920 * ((-(-((assign29600_e35925 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign29600_e35936 * assign29600_e35936))), (-((1e-100 * (((-(-((assign29600_e35908 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign29600_e35934) + (assign29600_e35911 * (0.5 * (((-(-((assign29600_e35917 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign29600_e35931) + (assign29600_e35920 * ((-(-((assign29600_e35925 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign29600_e35936 * assign29600_e35936))), (-((1e-100 * (((-(-((assign29600_e35908 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign29600_e35934) + (assign29600_e35911 * (0.5 * (((-(-((assign29600_e35917 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign29600_e35931) + (assign29600_e35920 * ((-(-((assign29600_e35925 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign29600_e35936 * assign29600_e35936))), (-((1e-100 * (((-(-((assign29600_e35908 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign29600_e35934) + (assign29600_e35911 * (0.5 * (((-(-((assign29600_e35917 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign29600_e35931) + (assign29600_e35920 * ((-(-((assign29600_e35925 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign29600_e35936 * assign29600_e35936))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard570 == 0.0)) && (locals.var_guard572 == 0.0)) && (locals.var_guard573 == 0.0)) {
            let assign29610_e35958: f64 = (-locals.var_fbbtsti_d);
            let assign29610_e35960: f64 = (assign29610_e35958 / locals.var_fmaxr);
            let assign29610_e35962: f64 = (assign29610_e35960 - 230.25850929940458);
            let assign29610_e35966: f64 = (-locals.var_fbbtsti_d);
            let assign29610_e35968: f64 = (assign29610_e35966 / locals.var_fmaxr);
            let assign29610_e35970: f64 = (assign29610_e35968 - 230.25850929940458);
            let assign29610_e35973: f64 = (-locals.var_fbbtsti_d);
            let assign29610_e35975: f64 = (assign29610_e35973 / locals.var_fmaxr);
            let assign29610_e35977: f64 = (assign29610_e35975 - 230.25850929940458);
            let assign29610_e35979: f64 = (assign29610_e35977 * 0.3333333333333333);
            let assign29610_e35980: f64 = (1.0 + assign29610_e35979);
            let assign29610_e35981: f64 = (assign29610_e35970 * assign29610_e35980);
            let assign29610_e35982: f64 = (0.5 * assign29610_e35981);
            let assign29610_e35983: f64 = (1.0 + assign29610_e35982);
            let assign29610_e35984: f64 = (assign29610_e35962 * assign29610_e35983);
            let assign29610_e35985: f64 = (1.0 + assign29610_e35984);
            let assign29610_e35986: f64 = (1e100 * assign29610_e35985);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29610_e35986, (1e100 * (((-((assign29610_e35958 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign29610_e35983) + (assign29610_e35962 * (0.5 * (((-((assign29610_e35966 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign29610_e35980) + (assign29610_e35970 * ((-((assign29610_e35973 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign29610_e35958 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign29610_e35983) + (assign29610_e35962 * (0.5 * (((-((assign29610_e35966 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign29610_e35980) + (assign29610_e35970 * ((-((assign29610_e35973 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign29610_e35958 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign29610_e35983) + (assign29610_e35962 * (0.5 * (((-((assign29610_e35966 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign29610_e35980) + (assign29610_e35970 * ((-((assign29610_e35973 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign29610_e35958 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign29610_e35983) + (assign29610_e35962 * (0.5 * (((-((assign29610_e35966 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign29610_e35980) + (assign29610_e35970 * ((-((assign29610_e35973 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard570 == 0.0)) {
            let assign29620_e36001: f64 = (locals.var_v1 * locals.var_fmaxr);
            let assign29620_e36003: f64 = (assign29620_e36001 * locals.var_fmaxr);
            let assign29620_e36005: f64 = (assign29620_e36003 * locals.var_tmp);
            let assign29620_e36006: f64 = (locals.var_cbbtstid_i * assign29620_e36005);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign29620_e36006, (locals.var_cbbtstid_i * (((((locals.var_v1 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign29620_e36001 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign29620_e36003 * locals.var_tmp_dn5))), (locals.var_cbbtstid_i * (((((locals.var_v1 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign29620_e36001 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign29620_e36003 * locals.var_tmp_dn6))), (locals.var_cbbtstid_i * (((((locals.var_v1 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign29620_e36001 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign29620_e36003 * locals.var_tmp_dn7))), (locals.var_cbbtstid_i * (((((locals.var_v1 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign29620_e36001 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign29620_e36003 * locals.var_tmp_dn8))), );
        }
        let assign29630_e36011: f64 = if locals.var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard574 = assign29630_e36011;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard574 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign29650_e36025: f64 = (-locals.var_alphaav);
        let assign29650_e36027: f64 = (assign29650_e36025 * locals.var_vbrstid_i);
        let assign29650_e36028: f64 = if locals.var_vav > assign29650_e36027 { 1.0 } else { 0.0 };
        locals.var_guard575 = assign29650_e36028;
        let assign29660_e36031: f64 = if locals.var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard576 = assign29660_e36031;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard574 == 0.0)) && (locals.var_guard575 != 0.0)) && (locals.var_guard576 != 0.0)) {
            let assign29670_e36047: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign29670_e36050: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign29670_e36051: f64 = (assign29670_e36047 * assign29670_e36050);
            let assign29670_e36054: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign29670_e36055: f64 = (assign29670_e36051 * assign29670_e36054);
            let assign29670_e36058: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign29670_e36059: f64 = (assign29670_e36055 * assign29670_e36058);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29670_e36059, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard574 == 0.0)) && (locals.var_guard575 != 0.0)) && (locals.var_guard576 == 0.0)) {
            let assign29680_e36078: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign29680_e36079: f64 = (assign29680_e36078).abs();
            let assign29680_e36081: f64 = (assign29680_e36079).powf(locals.var_pbrstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29680_e36081, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard574 == 0.0)) && (locals.var_guard575 != 0.0)) {
            let assign29690_e36098: f64 = (1.0 - locals.var_tmp);
            let assign29690_e36099: f64 = (1.0 / assign29690_e36098);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign29690_e36099, (-((-locals.var_tmp_dn5) / (assign29690_e36098 * assign29690_e36098))), (-((-locals.var_tmp_dn6) / (assign29690_e36098 * assign29690_e36098))), (-((-locals.var_tmp_dn7) / (assign29690_e36098 * assign29690_e36098))), (-((-locals.var_tmp_dn8) / (assign29690_e36098 * assign29690_e36098))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard574 == 0.0)) && (locals.var_guard575 == 0.0)) {
            let assign29700_e36118: f64 = (locals.var_alphaav * locals.var_vbrstid_i);
            let assign29700_e36119: f64 = (locals.var_vav + assign29700_e36118);
            let assign29700_e36121: f64 = (assign29700_e36119 * locals.var_slopesti_d);
            let assign29700_e36122: f64 = (locals.var_fstopsti_d + assign29700_e36121);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign29700_e36122, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard560 == 0.0)) {
            let assign29710_e36134: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign29710_e36136: f64 = (assign29710_e36134 + locals.var_itat);
            let assign29710_e36138: f64 = (assign29710_e36136 + locals.var_ibbt);
            let assign29710_e36139: f64 = (p.p29 * assign29710_e36138);
            let assign29710_e36141: f64 = (assign29710_e36139 * locals.var_fbreakdown);
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (assign29710_e36141, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign29710_e36139 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign29710_e36139 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign29710_e36139 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign29710_e36139 * locals.var_fbreakdown_dn8)), );
        }
        let assign29720_e36146: f64 = if locals.var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard577 = assign29720_e36146;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 != 0.0)) {
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) {
            let assign29740_e36163: f64 = (locals.var_idsatgat_d * locals.var_idmult);
            locals.var_id__blk219 = assign29740_e36163;
        }
        let assign29750_e36172: f64 = if ((locals.var_csrhgatd_i == 0.0) && (locals.var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard578 = assign29750_e36172;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard578 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard578 == 0.0)) {
            let assign29770_e36195: f64 = (locals.var_vbigat_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign29770_e36195;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard578 == 0.0)) {
            let assign29780_e36211: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign29780_e36212: f64 = (1.0 - assign29780_e36211);
            let assign29780_e36213: f64 = (assign29780_e36212).sqrt();
            let assign29780_e36214: f64 = (1.0 - assign29780_e36213);
            locals.var_wsrhstep = assign29780_e36214;
        }
        let assign29790_e36219: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard579 = assign29790_e36219;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard579 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard579 == 0.0)) {
            let assign29810_e36248: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign29810_e36250: f64 = (locals.var_wsrhstep).ln();
            let assign29810_e36251: f64 = (assign29810_e36248 * assign29810_e36250);
            let assign29810_e36254: f64 = (1.0 - locals.var_wsrhstep);
            let assign29810_e36255: f64 = (assign29810_e36251 / assign29810_e36254);
            let assign29810_e36257: f64 = (assign29810_e36255 + locals.var_wsrhstep);
            let assign29810_e36261: f64 = (2.0 * locals.var_pgatd_i);
            let assign29810_e36262: f64 = (1.0 - assign29810_e36261);
            let assign29810_e36263: f64 = (assign29810_e36257 * assign29810_e36262);
            locals.var_dwsrh = assign29810_e36263;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard578 == 0.0)) {
            let assign29820_e36277: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign29820_e36277;
        }
        let assign29830_e36282: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard580 = assign29830_e36282;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 != 0.0)) {
            let assign29840_e36296: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv_d);
            let assign29840_e36297: f64 = (assign29840_e36296).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29840_e36297, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) {
            let assign29850_e36314: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv_d);
            let assign29850_e36316: f64 = (assign29850_e36314).powf(locals.var_pgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign29850_e36316, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard578 == 0.0)) {
            let assign29860_e36330: f64 = (locals.var_wdepnulrgat_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign29860_e36330, (locals.var_wdepnulrgat_d * locals.var_tmp_dn5), (locals.var_wdepnulrgat_d * locals.var_tmp_dn6), (locals.var_wdepnulrgat_d * locals.var_tmp_dn7), (locals.var_wdepnulrgat_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard578 == 0.0)) {
            let assign29870_e36345: f64 = (locals.var_zinv - 1.0);
            let assign29870_e36347: f64 = (assign29870_e36345 * locals.var_wdep);
            let assign29870_e36348: f64 = (locals.var_ftdgat_d * assign29870_e36347);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign29870_e36348, (locals.var_ftdgat_d * (assign29870_e36345 * locals.var_wdep_dn5)), (locals.var_ftdgat_d * (assign29870_e36345 * locals.var_wdep_dn6)), (locals.var_ftdgat_d * (assign29870_e36345 * locals.var_wdep_dn7)), (locals.var_ftdgat_d * (assign29870_e36345 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard578 == 0.0)) {
            let assign29880_e36363: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign29880_e36364: f64 = (locals.var_csrhgatd_i * assign29880_e36363);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign29880_e36364, (locals.var_csrhgatd_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign29890_e36369: f64 = if locals.var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard581 = assign29890_e36369;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign29910_e36393: f64 = (locals.var_wdep * locals.var_one_minus_pgat_d);
            let assign29910_e36395: f64 = (assign29910_e36393 / locals.var_vbi_minus_vjsrh);
            let assign29910_e36396: f64 = (locals.var_btatpartgat_d * assign29910_e36395);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign29910_e36396, (locals.var_btatpartgat_d * ((locals.var_wdep_dn5 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn6 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn7 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn8 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign29920_e36410: f64 = (0.666666666666667 * locals.var_atatgat_d);
            let assign29920_e36412: f64 = (assign29920_e36410 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign29920_e36412, (-((assign29920_e36410 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign29920_e36410 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign29920_e36410 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign29920_e36410 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign29930_e36426: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign29930_e36426, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign29940_e36440: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign29940_e36443: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign29940_e36445: f64 = (assign29940_e36443 + 1.0);
            let assign29940_e36446: f64 = (assign29940_e36440 / assign29940_e36445);
            let assign29940_e36447: f64 = (assign29940_e36446).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign29940_e36447, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign29940_e36445) - (assign29940_e36440 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign29940_e36445 * assign29940_e36445)) / (2.0 * assign29940_e36447)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign29940_e36445) - (assign29940_e36440 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign29940_e36445 * assign29940_e36445)) / (2.0 * assign29940_e36447)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign29940_e36445) - (assign29940_e36440 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign29940_e36445 * assign29940_e36445)) / (2.0 * assign29940_e36447)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign29940_e36445) - (assign29940_e36440 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign29940_e36445 * assign29940_e36445)) / (2.0 * assign29940_e36447)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign29950_e36460: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign29950_e36460, (locals.var_umax_dn5 / (2.0 * assign29950_e36460)), (locals.var_umax_dn6 / (2.0 * assign29950_e36460)), (locals.var_umax_dn7 / (2.0 * assign29950_e36460)), (locals.var_umax_dn8 / (2.0 * assign29950_e36460)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign29960_e36474: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign29960_e36474, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign29970_e36478: f64 = (-locals.var_pgatd_i);
        let assign29970_e36480: f64 = (assign29970_e36478 * locals.var_one_over_one_minus_pgat_d);
        let assign29970_e36482: f64 = (-1.0);
        let assign29970_e36483: f64 = if assign29970_e36480 == assign29970_e36482 { 1.0 } else { 0.0 };
        locals.var_guard582 = assign29970_e36483;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard582 != 0.0)) {
            let assign29980_e36499: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign29980_e36500: f64 = (1.0 + assign29980_e36499);
            let assign29980_e36501: f64 = (1.0 / assign29980_e36500);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign29980_e36501, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign29980_e36500 * assign29980_e36500))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign29980_e36500 * assign29980_e36500))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign29980_e36500 * assign29980_e36500))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign29980_e36500 * assign29980_e36500))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard582 == 0.0)) {
            let assign29990_e36519: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign29990_e36520: f64 = (1.0 + assign29990_e36519);
            let assign29990_e36522: f64 = (-locals.var_pgatd_i);
            let assign29990_e36524: f64 = (assign29990_e36522 * locals.var_one_over_one_minus_pgat_d);
            let assign29990_e36525: f64 = (assign29990_e36520).powf(assign29990_e36524);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign29990_e36525, if 0.0 == 0.0 && ((assign29990_e36524) as f64).is_finite() && ((assign29990_e36524) as f64).fract() == 0.0 { if assign29990_e36524 == 0.0 { 0.0 } else { (assign29990_e36524 * ((assign29990_e36520).powf(assign29990_e36524 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign29990_e36525 * (assign29990_e36524 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign29990_e36520))) }, if 0.0 == 0.0 && ((assign29990_e36524) as f64).is_finite() && ((assign29990_e36524) as f64).fract() == 0.0 { if assign29990_e36524 == 0.0 { 0.0 } else { (assign29990_e36524 * ((assign29990_e36520).powf(assign29990_e36524 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign29990_e36525 * (assign29990_e36524 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign29990_e36520))) }, if 0.0 == 0.0 && ((assign29990_e36524) as f64).is_finite() && ((assign29990_e36524) as f64).fract() == 0.0 { if assign29990_e36524 == 0.0 { 0.0 } else { (assign29990_e36524 * ((assign29990_e36520).powf(assign29990_e36524 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign29990_e36525 * (assign29990_e36524 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign29990_e36520))) }, if 0.0 == 0.0 && ((assign29990_e36524) as f64).is_finite() && ((assign29990_e36524) as f64).fract() == 0.0 { if assign29990_e36524 == 0.0 { 0.0 } else { (assign29990_e36524 * ((assign29990_e36520).powf(assign29990_e36524 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign29990_e36525 * (assign29990_e36524 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign29990_e36520))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign30000_e36539: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign30000_e36542: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign30000_e36543: f64 = (assign30000_e36539 / assign30000_e36542);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign30000_e36543, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign30000_e36542) - (assign30000_e36539 * locals.var_wgamma_dn5)) / (assign30000_e36542 * assign30000_e36542)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign30000_e36542) - (assign30000_e36539 * locals.var_wgamma_dn6)) / (assign30000_e36542 * assign30000_e36542)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign30000_e36542) - (assign30000_e36539 * locals.var_wgamma_dn7)) / (assign30000_e36542 * assign30000_e36542)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign30000_e36542) - (assign30000_e36539 * locals.var_wgamma_dn8)) / (assign30000_e36542 * assign30000_e36542)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign30010_e36558: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign30010_e36559: f64 = (0.375 * assign30010_e36558);
            let assign30010_e36560: f64 = (assign30010_e36559).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign30010_e36560, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign30010_e36560)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign30010_e36560)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign30010_e36560)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign30010_e36560)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign30020_e36575: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign30020_e36576: f64 = (2.0 * assign30020_e36575);
            let assign30020_e36578: f64 = (assign30020_e36576 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign30020_e36578, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign30030_e36592: f64 = (locals.var_atatgat_d * locals.var_twoatatoverthreebtat);
            let assign30030_e36594: f64 = (assign30030_e36592 * locals.var_sqrtumax);
            let assign30030_e36597: f64 = (locals.var_atatgat_d * locals.var_umax);
            let assign30030_e36598: f64 = (assign30030_e36594 - assign30030_e36597);
            let assign30030_e36602: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign30030_e36603: f64 = (0.5 * assign30030_e36602);
            let assign30030_e36604: f64 = (assign30030_e36598 + assign30030_e36603);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign30030_e36604, (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign30030_e36592 * locals.var_sqrtumax_dn5)) - (locals.var_atatgat_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign30030_e36592 * locals.var_sqrtumax_dn6)) - (locals.var_atatgat_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign30030_e36592 * locals.var_sqrtumax_dn7)) - (locals.var_atatgat_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign30030_e36592 * locals.var_sqrtumax_dn8)) - (locals.var_atatgat_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign30040_e36618: f64 = (locals.var_ltat - 1.0);
            let assign30040_e36620: f64 = (assign30040_e36618 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign30040_e36620, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign30040_e36618 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign30040_e36618 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign30040_e36618 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign30040_e36618 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign30050_e36634: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign30050_e36634, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign30060_e36639: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard583 = assign30060_e36639;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard583 != 0.0)) {
            let assign30070_e36655: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign30070_e36656: f64 = (1.0 + assign30070_e36655);
            let assign30070_e36657: f64 = (1.0 / assign30070_e36656);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign30070_e36657, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign30070_e36656 * assign30070_e36656))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign30070_e36656 * assign30070_e36656))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign30070_e36656 * assign30070_e36656))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign30070_e36656 * assign30070_e36656))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard583 == 0.0)) {
            let assign30080_e36676: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign30080_e36677: f64 = (1.0 - assign30080_e36676);
            let assign30080_e36678: f64 = (1.0 / assign30080_e36677);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign30080_e36678, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign30080_e36677 * assign30080_e36677))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign30080_e36677 * assign30080_e36677))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign30080_e36677 * assign30080_e36677))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign30080_e36677 * assign30080_e36677))), );
        }
        let assign30090_e36682: f64 = (-locals.var_ysq);
        let assign30090_e36684: f64 = (assign30090_e36682 + locals.var_mtat);
        let assign30090_e36686: f64 = (-230.25850929940458);
        let assign30090_e36687: f64 = if assign30090_e36684 > assign30090_e36686 { 1.0 } else { 0.0 };
        locals.var_guard584 = assign30090_e36687;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard584 != 0.0)) {
            let assign30100_e36700: f64 = (-locals.var_ysq);
            let assign30100_e36702: f64 = (assign30100_e36700 + locals.var_mtat);
            let assign30100_e36703: f64 = (assign30100_e36702).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30100_e36703, (assign30100_e36703 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign30100_e36703 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign30100_e36703 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign30100_e36703 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard584 == 0.0)) {
            let assign30110_e36721: f64 = (-230.25850929940458);
            let assign30110_e36723: f64 = (-locals.var_ysq);
            let assign30110_e36725: f64 = (assign30110_e36723 + locals.var_mtat);
            let assign30110_e36726: f64 = (assign30110_e36721 - assign30110_e36725);
            let assign30110_e36730: f64 = (-230.25850929940458);
            let assign30110_e36732: f64 = (-locals.var_ysq);
            let assign30110_e36734: f64 = (assign30110_e36732 + locals.var_mtat);
            let assign30110_e36735: f64 = (assign30110_e36730 - assign30110_e36734);
            let assign30110_e36738: f64 = (-230.25850929940458);
            let assign30110_e36740: f64 = (-locals.var_ysq);
            let assign30110_e36742: f64 = (assign30110_e36740 + locals.var_mtat);
            let assign30110_e36743: f64 = (assign30110_e36738 - assign30110_e36742);
            let assign30110_e36745: f64 = (assign30110_e36743 * 0.3333333333333333);
            let assign30110_e36746: f64 = (1.0 + assign30110_e36745);
            let assign30110_e36747: f64 = (assign30110_e36735 * assign30110_e36746);
            let assign30110_e36748: f64 = (0.5 * assign30110_e36747);
            let assign30110_e36749: f64 = (1.0 + assign30110_e36748);
            let assign30110_e36750: f64 = (assign30110_e36726 * assign30110_e36749);
            let assign30110_e36751: f64 = (1.0 + assign30110_e36750);
            let assign30110_e36752: f64 = (1e-100 / assign30110_e36751);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30110_e36752, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign30110_e36749) + (assign30110_e36726 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign30110_e36746) + (assign30110_e36735 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign30110_e36751 * assign30110_e36751))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign30110_e36749) + (assign30110_e36726 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign30110_e36746) + (assign30110_e36735 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign30110_e36751 * assign30110_e36751))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign30110_e36749) + (assign30110_e36726 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign30110_e36746) + (assign30110_e36735 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign30110_e36751 * assign30110_e36751))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign30110_e36749) + (assign30110_e36726 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign30110_e36746) + (assign30110_e36735 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign30110_e36751 * assign30110_e36751))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign30120_e36766: f64 = (0.29214664 * locals.var_terfc);
            let assign30120_e36770: f64 = (locals.var_terfc * locals.var_terfc);
            let assign30120_e36771: f64 = (locals.var_berfc * assign30120_e36770);
            let assign30120_e36772: f64 = (assign30120_e36766 + assign30120_e36771);
            let assign30120_e36776: f64 = (locals.var_terfc * locals.var_terfc);
            let assign30120_e36778: f64 = (assign30120_e36776 * locals.var_terfc);
            let assign30120_e36779: f64 = (locals.var_cerfc * assign30120_e36778);
            let assign30120_e36780: f64 = (assign30120_e36772 + assign30120_e36779);
            let assign30120_e36782: f64 = (assign30120_e36780 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign30120_e36782, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign30120_e36776 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign30120_e36780 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign30120_e36776 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign30120_e36780 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign30120_e36776 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign30120_e36780 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign30120_e36776 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign30120_e36780 * locals.var_tmp_dn8)), );
        }
        let assign30130_e36787: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard585 = assign30130_e36787;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard585 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign30150_e36804: f64 = (-230.25850929940458);
        let assign30150_e36805: f64 = if locals.var_mtat > assign30150_e36804 { 1.0 } else { 0.0 };
        locals.var_guard586 = assign30150_e36805;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard586 != 0.0)) {
            let assign30160_e36821: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30160_e36821, (assign30160_e36821 * locals.var_mtat_dn5), (assign30160_e36821 * locals.var_mtat_dn6), (assign30160_e36821 * locals.var_mtat_dn7), (assign30160_e36821 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard586 == 0.0)) {
            let assign30170_e36842: f64 = (-230.25850929940458);
            let assign30170_e36844: f64 = (assign30170_e36842 - locals.var_mtat);
            let assign30170_e36848: f64 = (-230.25850929940458);
            let assign30170_e36850: f64 = (assign30170_e36848 - locals.var_mtat);
            let assign30170_e36853: f64 = (-230.25850929940458);
            let assign30170_e36855: f64 = (assign30170_e36853 - locals.var_mtat);
            let assign30170_e36857: f64 = (assign30170_e36855 * 0.3333333333333333);
            let assign30170_e36858: f64 = (1.0 + assign30170_e36857);
            let assign30170_e36859: f64 = (assign30170_e36850 * assign30170_e36858);
            let assign30170_e36860: f64 = (0.5 * assign30170_e36859);
            let assign30170_e36861: f64 = (1.0 + assign30170_e36860);
            let assign30170_e36862: f64 = (assign30170_e36844 * assign30170_e36861);
            let assign30170_e36863: f64 = (1.0 + assign30170_e36862);
            let assign30170_e36864: f64 = (1e-100 / assign30170_e36863);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30170_e36864, (-((1e-100 * (((-locals.var_mtat_dn5) * assign30170_e36861) + (assign30170_e36844 * (0.5 * (((-locals.var_mtat_dn5) * assign30170_e36858) + (assign30170_e36850 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign30170_e36863 * assign30170_e36863))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign30170_e36861) + (assign30170_e36844 * (0.5 * (((-locals.var_mtat_dn6) * assign30170_e36858) + (assign30170_e36850 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign30170_e36863 * assign30170_e36863))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign30170_e36861) + (assign30170_e36844 * (0.5 * (((-locals.var_mtat_dn7) * assign30170_e36858) + (assign30170_e36850 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign30170_e36863 * assign30170_e36863))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign30170_e36861) + (assign30170_e36844 * (0.5 * (((-locals.var_mtat_dn8) * assign30170_e36858) + (assign30170_e36850 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign30170_e36863 * assign30170_e36863))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard585 == 0.0)) {
            let assign30180_e36881: f64 = (2.0 * locals.var_tmp);
            let assign30180_e36883: f64 = (assign30180_e36881 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign30180_e36883, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign30190_e36897: f64 = (1.772453850905516 * 0.5);
            let assign30190_e36900: f64 = (locals.var_atatgat_d * locals.var_erfctimesexpmtat);
            let assign30190_e36902: f64 = (assign30190_e36900 / locals.var_ktat);
            let assign30190_e36903: f64 = (assign30190_e36897 * assign30190_e36902);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign30190_e36903, (assign30190_e36897 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign30190_e36900 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign30190_e36897 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign30190_e36900 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign30190_e36897 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign30190_e36900 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign30190_e36897 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign30190_e36900 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard581 == 0.0)) {
            let assign30200_e36918: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign30200_e36920: f64 = (assign30200_e36918 * locals.var_wtat);
            let assign30200_e36921: f64 = (locals.var_ctatgatd_i * assign30200_e36920);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign30200_e36921, (locals.var_ctatgatd_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign30200_e36918 * locals.var_wtat_dn5))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign30200_e36918 * locals.var_wtat_dn6))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign30200_e36918 * locals.var_wtat_dn7))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign30200_e36918 * locals.var_wtat_dn8))), );
        }
        let assign30210_e36926: f64 = if locals.var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard587 = assign30210_e36926;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard587 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign30230_e36940: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard588 = assign30230_e36940;
    }
    pub(super) fn stamp_transient_block_35(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard588 != 0.0)) {
            let assign30240_e36954: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign30240_e36956: f64 = (assign30240_e36954 * locals.var_vbirgatinv_d);
            let assign30240_e36957: f64 = (assign30240_e36956).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30240_e36957, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard588 == 0.0)) {
            let assign30250_e36974: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign30250_e36976: f64 = (assign30250_e36974 * locals.var_vbirgatinv_d);
            let assign30250_e36978: f64 = (assign30250_e36976).powf(locals.var_pgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30250_e36978, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard587 == 0.0)) {
            let assign30260_e36993: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign30260_e36995: f64 = (assign30260_e36993 * locals.var_wdepnulrinvgat_d);
            let assign30260_e36997: f64 = (assign30260_e36995 / locals.var_tmp);
            let assign30260_e36998: f64 = (locals.var_one_over_one_minus_pgat_d * assign30260_e36997);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign30260_e36998, (locals.var_one_over_one_minus_pgat_d * (-((assign30260_e36995 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat_d * (-((assign30260_e36995 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat_d * (-((assign30260_e36995 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat_d * (-((assign30260_e36995 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign30270_e37002: f64 = (-locals.var_fbbtgat_d);
        let assign30270_e37004: f64 = (assign30270_e37002 / locals.var_fmaxr);
        let assign30270_e37005: f64 = (assign30270_e37004).abs();
        let assign30270_e37007: f64 = if assign30270_e37005 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard589 = assign30270_e37007;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard589 != 0.0)) {
            let assign30280_e37020: f64 = (-locals.var_fbbtgat_d);
            let assign30280_e37022: f64 = (assign30280_e37020 / locals.var_fmaxr);
            let assign30280_e37023: f64 = (assign30280_e37022).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30280_e37023, (assign30280_e37023 * ((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign30280_e37020 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign30280_e37023 * ((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign30280_e37020 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign30280_e37023 * ((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign30280_e37020 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign30280_e37023 * ((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign30280_e37020 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))), );
        }
        let assign30290_e37027: f64 = (-locals.var_fbbtgat_d);
        let assign30290_e37029: f64 = (assign30290_e37027 / locals.var_fmaxr);
        let assign30290_e37031: f64 = if assign30290_e37029 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard590 = assign30290_e37031;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard589 == 0.0)) && (locals.var_guard590 != 0.0)) {
            let assign30300_e37049: f64 = (-230.25850929940458);
            let assign30300_e37051: f64 = (-locals.var_fbbtgat_d);
            let assign30300_e37053: f64 = (assign30300_e37051 / locals.var_fmaxr);
            let assign30300_e37054: f64 = (assign30300_e37049 - assign30300_e37053);
            let assign30300_e37058: f64 = (-230.25850929940458);
            let assign30300_e37060: f64 = (-locals.var_fbbtgat_d);
            let assign30300_e37062: f64 = (assign30300_e37060 / locals.var_fmaxr);
            let assign30300_e37063: f64 = (assign30300_e37058 - assign30300_e37062);
            let assign30300_e37066: f64 = (-230.25850929940458);
            let assign30300_e37068: f64 = (-locals.var_fbbtgat_d);
            let assign30300_e37070: f64 = (assign30300_e37068 / locals.var_fmaxr);
            let assign30300_e37071: f64 = (assign30300_e37066 - assign30300_e37070);
            let assign30300_e37073: f64 = (assign30300_e37071 * 0.3333333333333333);
            let assign30300_e37074: f64 = (1.0 + assign30300_e37073);
            let assign30300_e37075: f64 = (assign30300_e37063 * assign30300_e37074);
            let assign30300_e37076: f64 = (0.5 * assign30300_e37075);
            let assign30300_e37077: f64 = (1.0 + assign30300_e37076);
            let assign30300_e37078: f64 = (assign30300_e37054 * assign30300_e37077);
            let assign30300_e37079: f64 = (1.0 + assign30300_e37078);
            let assign30300_e37080: f64 = (1e-100 / assign30300_e37079);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30300_e37080, (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign30300_e37051 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign30300_e37077) + (assign30300_e37054 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign30300_e37060 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign30300_e37074) + (assign30300_e37063 * ((-((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign30300_e37068 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign30300_e37079 * assign30300_e37079))), (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign30300_e37051 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign30300_e37077) + (assign30300_e37054 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign30300_e37060 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign30300_e37074) + (assign30300_e37063 * ((-((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign30300_e37068 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign30300_e37079 * assign30300_e37079))), (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign30300_e37051 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign30300_e37077) + (assign30300_e37054 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign30300_e37060 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign30300_e37074) + (assign30300_e37063 * ((-((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign30300_e37068 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign30300_e37079 * assign30300_e37079))), (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign30300_e37051 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign30300_e37077) + (assign30300_e37054 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign30300_e37060 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign30300_e37074) + (assign30300_e37063 * ((-((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign30300_e37068 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign30300_e37079 * assign30300_e37079))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard589 == 0.0)) && (locals.var_guard590 == 0.0)) {
            let assign30310_e37101: f64 = (-locals.var_fbbtgat_d);
            let assign30310_e37103: f64 = (assign30310_e37101 / locals.var_fmaxr);
            let assign30310_e37105: f64 = (assign30310_e37103 - 230.25850929940458);
            let assign30310_e37109: f64 = (-locals.var_fbbtgat_d);
            let assign30310_e37111: f64 = (assign30310_e37109 / locals.var_fmaxr);
            let assign30310_e37113: f64 = (assign30310_e37111 - 230.25850929940458);
            let assign30310_e37116: f64 = (-locals.var_fbbtgat_d);
            let assign30310_e37118: f64 = (assign30310_e37116 / locals.var_fmaxr);
            let assign30310_e37120: f64 = (assign30310_e37118 - 230.25850929940458);
            let assign30310_e37122: f64 = (assign30310_e37120 * 0.3333333333333333);
            let assign30310_e37123: f64 = (1.0 + assign30310_e37122);
            let assign30310_e37124: f64 = (assign30310_e37113 * assign30310_e37123);
            let assign30310_e37125: f64 = (0.5 * assign30310_e37124);
            let assign30310_e37126: f64 = (1.0 + assign30310_e37125);
            let assign30310_e37127: f64 = (assign30310_e37105 * assign30310_e37126);
            let assign30310_e37128: f64 = (1.0 + assign30310_e37127);
            let assign30310_e37129: f64 = (1e100 * assign30310_e37128);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30310_e37129, (1e100 * ((((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign30310_e37101 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign30310_e37126) + (assign30310_e37105 * (0.5 * ((((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign30310_e37109 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign30310_e37123) + (assign30310_e37113 * (((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign30310_e37116 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign30310_e37101 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign30310_e37126) + (assign30310_e37105 * (0.5 * ((((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign30310_e37109 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign30310_e37123) + (assign30310_e37113 * (((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign30310_e37116 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign30310_e37101 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign30310_e37126) + (assign30310_e37105 * (0.5 * ((((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign30310_e37109 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign30310_e37123) + (assign30310_e37113 * (((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign30310_e37116 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign30310_e37101 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign30310_e37126) + (assign30310_e37105 * (0.5 * ((((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign30310_e37109 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign30310_e37123) + (assign30310_e37113 * (((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign30310_e37116 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard587 == 0.0)) {
            let assign30320_e37144: f64 = (locals.var_v1 * locals.var_fmaxr);
            let assign30320_e37146: f64 = (assign30320_e37144 * locals.var_fmaxr);
            let assign30320_e37148: f64 = (assign30320_e37146 * locals.var_tmp);
            let assign30320_e37149: f64 = (locals.var_cbbtgatd_i * assign30320_e37148);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign30320_e37149, (locals.var_cbbtgatd_i * (((((locals.var_v1 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign30320_e37144 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign30320_e37146 * locals.var_tmp_dn5))), (locals.var_cbbtgatd_i * (((((locals.var_v1 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign30320_e37144 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign30320_e37146 * locals.var_tmp_dn6))), (locals.var_cbbtgatd_i * (((((locals.var_v1 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign30320_e37144 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign30320_e37146 * locals.var_tmp_dn7))), (locals.var_cbbtgatd_i * (((((locals.var_v1 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign30320_e37144 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign30320_e37146 * locals.var_tmp_dn8))), );
        }
        let assign30330_e37154: f64 = if locals.var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard591 = assign30330_e37154;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard591 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign30350_e37168: f64 = (-locals.var_alphaav);
        let assign30350_e37170: f64 = (assign30350_e37168 * locals.var_vbrgatd_i);
        let assign30350_e37171: f64 = if locals.var_vav > assign30350_e37170 { 1.0 } else { 0.0 };
        locals.var_guard592 = assign30350_e37171;
        let assign30360_e37174: f64 = if locals.var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard593 = assign30360_e37174;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard591 == 0.0)) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) {
            let assign30370_e37190: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign30370_e37193: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign30370_e37194: f64 = (assign30370_e37190 * assign30370_e37193);
            let assign30370_e37197: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign30370_e37198: f64 = (assign30370_e37194 * assign30370_e37197);
            let assign30370_e37201: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign30370_e37202: f64 = (assign30370_e37198 * assign30370_e37201);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30370_e37202, (((((((locals.var_vav * locals.var_vbrinvgat_d_dn5) * assign30370_e37193) + (assign30370_e37190 * (locals.var_vav * locals.var_vbrinvgat_d_dn5))) * assign30370_e37197) + (assign30370_e37194 * (locals.var_vav * locals.var_vbrinvgat_d_dn5))) * assign30370_e37201) + (assign30370_e37198 * (locals.var_vav * locals.var_vbrinvgat_d_dn5))), (((((((locals.var_vav * locals.var_vbrinvgat_d_dn6) * assign30370_e37193) + (assign30370_e37190 * (locals.var_vav * locals.var_vbrinvgat_d_dn6))) * assign30370_e37197) + (assign30370_e37194 * (locals.var_vav * locals.var_vbrinvgat_d_dn6))) * assign30370_e37201) + (assign30370_e37198 * (locals.var_vav * locals.var_vbrinvgat_d_dn6))), (((((((locals.var_vav * locals.var_vbrinvgat_d_dn7) * assign30370_e37193) + (assign30370_e37190 * (locals.var_vav * locals.var_vbrinvgat_d_dn7))) * assign30370_e37197) + (assign30370_e37194 * (locals.var_vav * locals.var_vbrinvgat_d_dn7))) * assign30370_e37201) + (assign30370_e37198 * (locals.var_vav * locals.var_vbrinvgat_d_dn7))), (((((((locals.var_vav * locals.var_vbrinvgat_d_dn8) * assign30370_e37193) + (assign30370_e37190 * (locals.var_vav * locals.var_vbrinvgat_d_dn8))) * assign30370_e37197) + (assign30370_e37194 * (locals.var_vav * locals.var_vbrinvgat_d_dn8))) * assign30370_e37201) + (assign30370_e37198 * (locals.var_vav * locals.var_vbrinvgat_d_dn8))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard591 == 0.0)) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) {
            let assign30380_e37221: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign30380_e37222: f64 = (assign30380_e37221).abs();
            let assign30380_e37224: f64 = (assign30380_e37222).powf(locals.var_pbrgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30380_e37224, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign30380_e37222).powf(locals.var_pbrgatd_i - 1.0) * if assign30380_e37221 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn5)) })) } } else { (assign30380_e37224 * (locals.var_pbrgatd_i * (if assign30380_e37221 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn5)) } / assign30380_e37222))) }, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign30380_e37222).powf(locals.var_pbrgatd_i - 1.0) * if assign30380_e37221 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn6)) })) } } else { (assign30380_e37224 * (locals.var_pbrgatd_i * (if assign30380_e37221 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn6)) } / assign30380_e37222))) }, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign30380_e37222).powf(locals.var_pbrgatd_i - 1.0) * if assign30380_e37221 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn7)) })) } } else { (assign30380_e37224 * (locals.var_pbrgatd_i * (if assign30380_e37221 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn7)) } / assign30380_e37222))) }, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign30380_e37222).powf(locals.var_pbrgatd_i - 1.0) * if assign30380_e37221 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn8)) })) } } else { (assign30380_e37224 * (locals.var_pbrgatd_i * (if assign30380_e37221 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn8)) } / assign30380_e37222))) }, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard591 == 0.0)) && (locals.var_guard592 != 0.0)) {
            let assign30390_e37241: f64 = (1.0 - locals.var_tmp);
            let assign30390_e37242: f64 = (1.0 / assign30390_e37241);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign30390_e37242, (-((-locals.var_tmp_dn5) / (assign30390_e37241 * assign30390_e37241))), (-((-locals.var_tmp_dn6) / (assign30390_e37241 * assign30390_e37241))), (-((-locals.var_tmp_dn7) / (assign30390_e37241 * assign30390_e37241))), (-((-locals.var_tmp_dn8) / (assign30390_e37241 * assign30390_e37241))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) && (locals.var_guard591 == 0.0)) && (locals.var_guard592 == 0.0)) {
            let assign30400_e37261: f64 = (locals.var_alphaav * locals.var_vbrgatd_i);
            let assign30400_e37262: f64 = (locals.var_vav + assign30400_e37261);
            let assign30400_e37264: f64 = (assign30400_e37262 * locals.var_slopegat_d);
            let assign30400_e37265: f64 = (locals.var_fstopgat_d + assign30400_e37264);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign30400_e37265, (assign30400_e37262 * locals.var_slopegat_d_dn5), (assign30400_e37262 * locals.var_slopegat_d_dn6), (assign30400_e37262 * locals.var_slopegat_d_dn7), (assign30400_e37262 * locals.var_slopegat_d_dn8), );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard577 == 0.0)) {
            let assign30410_e37277: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign30410_e37279: f64 = (assign30410_e37277 + locals.var_itat);
            let assign30410_e37281: f64 = (assign30410_e37279 + locals.var_ibbt);
            let assign30410_e37282: f64 = (p.p29 * assign30410_e37281);
            let assign30410_e37284: f64 = (assign30410_e37282 * locals.var_fbreakdown);
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (assign30410_e37284, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign30410_e37282 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign30410_e37282 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign30410_e37282 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign30410_e37282 * locals.var_fbreakdown_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign30420_e37292: f64 = (locals.var_abdrain_i * locals.var_ijunbot);
            let assign30420_e37295: f64 = (locals.var_lsdrain_i * locals.var_ijunsti);
            let assign30420_e37296: f64 = (assign30420_e37292 + assign30420_e37295);
            let assign30420_e37299: f64 = (locals.var_lgdrain_i * locals.var_ijungat);
            let assign30420_e37300: f64 = (assign30420_e37296 + assign30420_e37299);
            (locals.var_i1, locals.var_i1_dn5, locals.var_i1_dn6, locals.var_i1_dn7, locals.var_i1_dn8, ) = (assign30420_e37300, (((locals.var_abdrain_i * locals.var_ijunbot_dn5) + (locals.var_lsdrain_i * locals.var_ijunsti_dn5)) + (locals.var_lgdrain_i * locals.var_ijungat_dn5)), (((locals.var_abdrain_i * locals.var_ijunbot_dn6) + (locals.var_lsdrain_i * locals.var_ijunsti_dn6)) + (locals.var_lgdrain_i * locals.var_ijungat_dn6)), (((locals.var_abdrain_i * locals.var_ijunbot_dn7) + (locals.var_lsdrain_i * locals.var_ijunsti_dn7)) + (locals.var_lgdrain_i * locals.var_ijungat_dn7)), (((locals.var_abdrain_i * locals.var_ijunbot_dn8) + (locals.var_lsdrain_i * locals.var_ijunsti_dn8)) + (locals.var_lgdrain_i * locals.var_ijungat_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            locals.var_vbbt = 0.0;
            locals.var_two_psistar = 0.0;
        }
        let assign30450_e37326: f64 = if (!(((locals.var_abdrain_i == 0.0) && (locals.var_lsdrain_i == 0.0)) && (locals.var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard594 = assign30450_e37326;
        let assign30530_e37412: f64 = if locals.var_v2 < locals.var_vmax_d { 1.0 } else { 0.0 };
        locals.var_guard595 = assign30530_e37412;
        let assign30540_e37414: f64 = (-0.5);
        let assign30540_e37417: f64 = (locals.var_v2 * locals.var_phitdinv);
        let assign30540_e37418: f64 = (assign30540_e37414 * assign30540_e37417);
        let assign30540_e37419: f64 = (assign30540_e37418).abs();
        let assign30540_e37421: f64 = if assign30540_e37419 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard596 = assign30540_e37421;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 != 0.0)) && (locals.var_guard596 != 0.0)) {
            let assign30550_e37432: f64 = (-0.5);
            let assign30550_e37435: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign30550_e37436: f64 = (assign30550_e37432 * assign30550_e37435);
            let assign30550_e37437: f64 = (assign30550_e37436).exp();
            locals.var_z = assign30550_e37437;
        }
        let assign30560_e37441: f64 = (-0.5);
        let assign30560_e37444: f64 = (locals.var_v2 * locals.var_phitdinv);
        let assign30560_e37445: f64 = (assign30560_e37441 * assign30560_e37444);
        let assign30560_e37447: f64 = if assign30560_e37445 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard597 = assign30560_e37447;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 != 0.0)) && (locals.var_guard596 == 0.0)) && (locals.var_guard597 != 0.0)) {
            let assign30570_e37463: f64 = (-230.25850929940458);
            let assign30570_e37465: f64 = (-0.5);
            let assign30570_e37468: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign30570_e37469: f64 = (assign30570_e37465 * assign30570_e37468);
            let assign30570_e37470: f64 = (assign30570_e37463 - assign30570_e37469);
            let assign30570_e37474: f64 = (-230.25850929940458);
            let assign30570_e37476: f64 = (-0.5);
            let assign30570_e37479: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign30570_e37480: f64 = (assign30570_e37476 * assign30570_e37479);
            let assign30570_e37481: f64 = (assign30570_e37474 - assign30570_e37480);
            let assign30570_e37484: f64 = (-230.25850929940458);
            let assign30570_e37486: f64 = (-0.5);
            let assign30570_e37489: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign30570_e37490: f64 = (assign30570_e37486 * assign30570_e37489);
            let assign30570_e37491: f64 = (assign30570_e37484 - assign30570_e37490);
            let assign30570_e37493: f64 = (assign30570_e37491 * 0.3333333333333333);
            let assign30570_e37494: f64 = (1.0 + assign30570_e37493);
            let assign30570_e37495: f64 = (assign30570_e37481 * assign30570_e37494);
            let assign30570_e37496: f64 = (0.5 * assign30570_e37495);
            let assign30570_e37497: f64 = (1.0 + assign30570_e37496);
            let assign30570_e37498: f64 = (assign30570_e37470 * assign30570_e37497);
            let assign30570_e37499: f64 = (1.0 + assign30570_e37498);
            let assign30570_e37500: f64 = (1e-100 / assign30570_e37499);
            locals.var_z = assign30570_e37500;
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 != 0.0)) && (locals.var_guard596 == 0.0)) && (locals.var_guard597 == 0.0)) {
            let assign30580_e37519: f64 = (-0.5);
            let assign30580_e37522: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign30580_e37523: f64 = (assign30580_e37519 * assign30580_e37522);
            let assign30580_e37525: f64 = (assign30580_e37523 - 230.25850929940458);
            let assign30580_e37529: f64 = (-0.5);
            let assign30580_e37532: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign30580_e37533: f64 = (assign30580_e37529 * assign30580_e37532);
            let assign30580_e37535: f64 = (assign30580_e37533 - 230.25850929940458);
            let assign30580_e37538: f64 = (-0.5);
            let assign30580_e37541: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign30580_e37542: f64 = (assign30580_e37538 * assign30580_e37541);
            let assign30580_e37544: f64 = (assign30580_e37542 - 230.25850929940458);
            let assign30580_e37546: f64 = (assign30580_e37544 * 0.3333333333333333);
            let assign30580_e37547: f64 = (1.0 + assign30580_e37546);
            let assign30580_e37548: f64 = (assign30580_e37535 * assign30580_e37547);
            let assign30580_e37549: f64 = (0.5 * assign30580_e37548);
            let assign30580_e37550: f64 = (1.0 + assign30580_e37549);
            let assign30580_e37551: f64 = (assign30580_e37525 * assign30580_e37550);
            let assign30580_e37552: f64 = (1.0 + assign30580_e37551);
            let assign30580_e37553: f64 = (1e100 * assign30580_e37552);
            locals.var_z = assign30580_e37553;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 != 0.0)) {
            let assign30590_e37565: f64 = (1.0 / locals.var_z);
            locals.var_zinv = assign30590_e37565;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 != 0.0)) {
            let assign30600_e37577: f64 = (locals.var_zinv * locals.var_zinv);
            locals.var_idmult = assign30600_e37577;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 == 0.0)) {
            let assign30610_e37591: f64 = (locals.var_v2 - locals.var_vmax_d);
            let assign30610_e37593: f64 = (assign30610_e37591 * locals.var_phitdinv);
            let assign30610_e37594: f64 = (1.0 + assign30610_e37593);
            let assign30610_e37596: f64 = (assign30610_e37594 * locals.var_exp_vmax_over_phitd_d);
            locals.var_idmult = assign30610_e37596;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 == 0.0)) {
            let assign30620_e37608: f64 = (locals.var_idmult).sqrt();
            locals.var_zinv = assign30620_e37608;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 == 0.0)) {
            let assign30630_e37621: f64 = (1.0 / locals.var_zinv);
            locals.var_z = assign30630_e37621;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) {
            let assign30640_e37631: f64 = (locals.var_idmult - 1.0);
            locals.var_idmult = assign30640_e37631;
        }
        let assign30650_e37636: f64 = if locals.var_v2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard598 = assign30650_e37636;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard598 != 0.0)) {
            let assign30660_e37648: f64 = (2.0 + locals.var_z);
            let assign30660_e37651: f64 = (locals.var_z + 1.0);
            let assign30660_e37654: f64 = (locals.var_z + 3.0);
            let assign30660_e37655: f64 = (assign30660_e37651 * assign30660_e37654);
            let assign30660_e37656: f64 = (assign30660_e37655).sqrt();
            let assign30660_e37657: f64 = (assign30660_e37648 + assign30660_e37656);
            let assign30660_e37658: f64 = (assign30660_e37657).ln();
            let assign30660_e37659: f64 = (locals.var_phitd * assign30660_e37658);
            let assign30660_e37660: f64 = (2.0 * assign30660_e37659);
            locals.var_two_psistar = assign30660_e37660;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard598 == 0.0)) {
            let assign30670_e37672: f64 = (-locals.var_v2);
            let assign30670_e37677: f64 = (2.0 * locals.var_zinv);
            let assign30670_e37679: f64 = (assign30670_e37677 + 1.0);
            let assign30670_e37682: f64 = (1.0 + locals.var_zinv);
            let assign30670_e37686: f64 = (3.0 * locals.var_zinv);
            let assign30670_e37687: f64 = (1.0 + assign30670_e37686);
            let assign30670_e37688: f64 = (assign30670_e37682 * assign30670_e37687);
            let assign30670_e37689: f64 = (assign30670_e37688).sqrt();
            let assign30670_e37690: f64 = (assign30670_e37679 + assign30670_e37689);
            let assign30670_e37691: f64 = (assign30670_e37690).ln();
            let assign30670_e37692: f64 = (locals.var_phitd * assign30670_e37691);
            let assign30670_e37693: f64 = (2.0 * assign30670_e37692);
            let assign30670_e37694: f64 = (assign30670_e37672 + assign30670_e37693);
            locals.var_two_psistar = assign30670_e37694;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) {
            let assign30680_e37704: f64 = (locals.var_vbimin_d - locals.var_two_psistar);
            locals.var_vjlim = assign30680_e37704;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) {
            let assign30690_e37715: f64 = (locals.var_v2 + locals.var_vjlim);
            let assign30690_e37718: f64 = (locals.var_v2 - locals.var_vjlim);
            let assign30690_e37721: f64 = (locals.var_v2 - locals.var_vjlim);
            let assign30690_e37722: f64 = (assign30690_e37718 * assign30690_e37721);
            let assign30690_e37725: f64 = (4.0 * locals.var_phitd);
            let assign30690_e37727: f64 = (assign30690_e37725 * locals.var_phitd);
            let assign30690_e37728: f64 = (assign30690_e37722 + assign30690_e37727);
            let assign30690_e37729: f64 = (assign30690_e37728).sqrt();
            let assign30690_e37730: f64 = (assign30690_e37715 - assign30690_e37729);
            let assign30690_e37731: f64 = (0.5 * assign30690_e37730);
            locals.var_vjsrh = assign30690_e37731;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) {
            let assign30700_e37742: f64 = (locals.var_v2 + locals.var_vbbtlim_d);
            let assign30700_e37745: f64 = (locals.var_v2 - locals.var_vbbtlim_d);
            let assign30700_e37748: f64 = (locals.var_v2 - locals.var_vbbtlim_d);
            let assign30700_e37749: f64 = (assign30700_e37745 * assign30700_e37748);
            let assign30700_e37752: f64 = (4.0 * locals.var_phitr);
            let assign30700_e37754: f64 = (assign30700_e37752 * locals.var_phitr);
            let assign30700_e37755: f64 = (assign30700_e37749 + assign30700_e37754);
            let assign30700_e37756: f64 = (assign30700_e37755).sqrt();
            let assign30700_e37757: f64 = (assign30700_e37742 - assign30700_e37756);
            let assign30700_e37758: f64 = (0.5 * assign30700_e37757);
            locals.var_vbbt = assign30700_e37758;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard594 != 0.0)) {
            let assign30710_e37769: f64 = locals.var_v2;
            let assign30710_e37772: f64 = locals.var_v2;
            let assign30710_e37775: f64 = locals.var_v2;
            let assign30710_e37776: f64 = (assign30710_e37772 * assign30710_e37775);
            let assign30710_e37779: f64 = (4.0 * 1e-6);
            let assign30710_e37781: f64 = (assign30710_e37779 * 1e-6);
            let assign30710_e37782: f64 = (assign30710_e37776 + assign30710_e37781);
            let assign30710_e37783: f64 = (assign30710_e37782).sqrt();
            let assign30710_e37784: f64 = (assign30710_e37769 - assign30710_e37783);
            let assign30710_e37785: f64 = (0.5 * assign30710_e37784);
            locals.var_vav = assign30710_e37785;
        }
        let assign30720_e37790: f64 = if locals.var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard599 = assign30720_e37790;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 != 0.0)) {
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) {
            let assign30740_e37807: f64 = (locals.var_idsatbot_d * locals.var_idmult);
            locals.var_id__blk219 = assign30740_e37807;
        }
        let assign30750_e37816: f64 = if ((locals.var_csrhbotd_i == 0.0) && (locals.var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard600 = assign30750_e37816;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard600 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard600 == 0.0)) {
            let assign30770_e37839: f64 = (locals.var_vbibot_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign30770_e37839;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard600 == 0.0)) {
            let assign30780_e37855: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign30780_e37856: f64 = (1.0 - assign30780_e37855);
            let assign30780_e37857: f64 = (assign30780_e37856).sqrt();
            let assign30780_e37858: f64 = (1.0 - assign30780_e37857);
            locals.var_wsrhstep = assign30780_e37858;
        }
        let assign30790_e37863: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard601 = assign30790_e37863;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard600 == 0.0)) && (locals.var_guard601 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard600 == 0.0)) && (locals.var_guard601 == 0.0)) {
            let assign30810_e37892: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign30810_e37894: f64 = (locals.var_wsrhstep).ln();
            let assign30810_e37895: f64 = (assign30810_e37892 * assign30810_e37894);
            let assign30810_e37898: f64 = (1.0 - locals.var_wsrhstep);
            let assign30810_e37899: f64 = (assign30810_e37895 / assign30810_e37898);
            let assign30810_e37901: f64 = (assign30810_e37899 + locals.var_wsrhstep);
            let assign30810_e37905: f64 = (2.0 * locals.var_pbotd_i);
            let assign30810_e37906: f64 = (1.0 - assign30810_e37905);
            let assign30810_e37907: f64 = (assign30810_e37901 * assign30810_e37906);
            locals.var_dwsrh = assign30810_e37907;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard600 == 0.0)) {
            let assign30820_e37921: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign30820_e37921;
        }
        let assign30830_e37926: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard602 = assign30830_e37926;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard600 == 0.0)) && (locals.var_guard602 != 0.0)) {
            let assign30840_e37940: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv_d);
            let assign30840_e37941: f64 = (assign30840_e37940).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30840_e37941, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard600 == 0.0)) && (locals.var_guard602 == 0.0)) {
            let assign30850_e37958: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv_d);
            let assign30850_e37960: f64 = (assign30850_e37958).powf(locals.var_pbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign30850_e37960, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard600 == 0.0)) {
            let assign30860_e37974: f64 = (locals.var_wdepnulrbot_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign30860_e37974, (locals.var_wdepnulrbot_d * locals.var_tmp_dn5), (locals.var_wdepnulrbot_d * locals.var_tmp_dn6), (locals.var_wdepnulrbot_d * locals.var_tmp_dn7), (locals.var_wdepnulrbot_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard600 == 0.0)) {
            let assign30870_e37989: f64 = (locals.var_zinv - 1.0);
            let assign30870_e37991: f64 = (assign30870_e37989 * locals.var_wdep);
            let assign30870_e37992: f64 = (locals.var_ftdbot_d * assign30870_e37991);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign30870_e37992, (locals.var_ftdbot_d * (assign30870_e37989 * locals.var_wdep_dn5)), (locals.var_ftdbot_d * (assign30870_e37989 * locals.var_wdep_dn6)), (locals.var_ftdbot_d * (assign30870_e37989 * locals.var_wdep_dn7)), (locals.var_ftdbot_d * (assign30870_e37989 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard600 == 0.0)) {
            let assign30880_e38007: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign30880_e38008: f64 = (locals.var_csrhbotd_i * assign30880_e38007);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign30880_e38008, (locals.var_csrhbotd_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign30890_e38013: f64 = if locals.var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard603 = assign30890_e38013;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign30910_e38037: f64 = (locals.var_wdep * locals.var_one_minus_pbot_d);
            let assign30910_e38039: f64 = (assign30910_e38037 / locals.var_vbi_minus_vjsrh);
            let assign30910_e38040: f64 = (locals.var_btatpartbot_d * assign30910_e38039);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign30910_e38040, (locals.var_btatpartbot_d * ((locals.var_wdep_dn5 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn6 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn7 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn8 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign30920_e38054: f64 = (0.666666666666667 * locals.var_atatbot_d);
            let assign30920_e38056: f64 = (assign30920_e38054 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign30920_e38056, (-((assign30920_e38054 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign30920_e38054 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign30920_e38054 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign30920_e38054 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign30930_e38070: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign30930_e38070, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign30940_e38084: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign30940_e38087: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign30940_e38089: f64 = (assign30940_e38087 + 1.0);
            let assign30940_e38090: f64 = (assign30940_e38084 / assign30940_e38089);
            let assign30940_e38091: f64 = (assign30940_e38090).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign30940_e38091, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign30940_e38089) - (assign30940_e38084 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign30940_e38089 * assign30940_e38089)) / (2.0 * assign30940_e38091)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign30940_e38089) - (assign30940_e38084 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign30940_e38089 * assign30940_e38089)) / (2.0 * assign30940_e38091)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign30940_e38089) - (assign30940_e38084 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign30940_e38089 * assign30940_e38089)) / (2.0 * assign30940_e38091)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign30940_e38089) - (assign30940_e38084 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign30940_e38089 * assign30940_e38089)) / (2.0 * assign30940_e38091)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign30950_e38104: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign30950_e38104, (locals.var_umax_dn5 / (2.0 * assign30950_e38104)), (locals.var_umax_dn6 / (2.0 * assign30950_e38104)), (locals.var_umax_dn7 / (2.0 * assign30950_e38104)), (locals.var_umax_dn8 / (2.0 * assign30950_e38104)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign30960_e38118: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign30960_e38118, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign30970_e38122: f64 = (-locals.var_pbotd_i);
        let assign30970_e38124: f64 = (assign30970_e38122 * locals.var_one_over_one_minus_pbot_d);
        let assign30970_e38126: f64 = (-1.0);
        let assign30970_e38127: f64 = if assign30970_e38124 == assign30970_e38126 { 1.0 } else { 0.0 };
        locals.var_guard604 = assign30970_e38127;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) && (locals.var_guard604 != 0.0)) {
            let assign30980_e38143: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign30980_e38144: f64 = (1.0 + assign30980_e38143);
            let assign30980_e38145: f64 = (1.0 / assign30980_e38144);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign30980_e38145, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign30980_e38144 * assign30980_e38144))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign30980_e38144 * assign30980_e38144))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign30980_e38144 * assign30980_e38144))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign30980_e38144 * assign30980_e38144))), );
        }
    }
    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) && (locals.var_guard604 == 0.0)) {
            let assign30990_e38163: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign30990_e38164: f64 = (1.0 + assign30990_e38163);
            let assign30990_e38166: f64 = (-locals.var_pbotd_i);
            let assign30990_e38168: f64 = (assign30990_e38166 * locals.var_one_over_one_minus_pbot_d);
            let assign30990_e38169: f64 = (assign30990_e38164).powf(assign30990_e38168);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign30990_e38169, if 0.0 == 0.0 && ((assign30990_e38168) as f64).is_finite() && ((assign30990_e38168) as f64).fract() == 0.0 { if assign30990_e38168 == 0.0 { 0.0 } else { (assign30990_e38168 * ((assign30990_e38164).powf(assign30990_e38168 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign30990_e38169 * (assign30990_e38168 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign30990_e38164))) }, if 0.0 == 0.0 && ((assign30990_e38168) as f64).is_finite() && ((assign30990_e38168) as f64).fract() == 0.0 { if assign30990_e38168 == 0.0 { 0.0 } else { (assign30990_e38168 * ((assign30990_e38164).powf(assign30990_e38168 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign30990_e38169 * (assign30990_e38168 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign30990_e38164))) }, if 0.0 == 0.0 && ((assign30990_e38168) as f64).is_finite() && ((assign30990_e38168) as f64).fract() == 0.0 { if assign30990_e38168 == 0.0 { 0.0 } else { (assign30990_e38168 * ((assign30990_e38164).powf(assign30990_e38168 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign30990_e38169 * (assign30990_e38168 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign30990_e38164))) }, if 0.0 == 0.0 && ((assign30990_e38168) as f64).is_finite() && ((assign30990_e38168) as f64).fract() == 0.0 { if assign30990_e38168 == 0.0 { 0.0 } else { (assign30990_e38168 * ((assign30990_e38164).powf(assign30990_e38168 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign30990_e38169 * (assign30990_e38168 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign30990_e38164))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign31000_e38183: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign31000_e38186: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign31000_e38187: f64 = (assign31000_e38183 / assign31000_e38186);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign31000_e38187, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign31000_e38186) - (assign31000_e38183 * locals.var_wgamma_dn5)) / (assign31000_e38186 * assign31000_e38186)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign31000_e38186) - (assign31000_e38183 * locals.var_wgamma_dn6)) / (assign31000_e38186 * assign31000_e38186)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign31000_e38186) - (assign31000_e38183 * locals.var_wgamma_dn7)) / (assign31000_e38186 * assign31000_e38186)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign31000_e38186) - (assign31000_e38183 * locals.var_wgamma_dn8)) / (assign31000_e38186 * assign31000_e38186)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign31010_e38202: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign31010_e38203: f64 = (0.375 * assign31010_e38202);
            let assign31010_e38204: f64 = (assign31010_e38203).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign31010_e38204, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign31010_e38204)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign31010_e38204)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign31010_e38204)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign31010_e38204)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign31020_e38219: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign31020_e38220: f64 = (2.0 * assign31020_e38219);
            let assign31020_e38222: f64 = (assign31020_e38220 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign31020_e38222, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign31030_e38236: f64 = (locals.var_atatbot_d * locals.var_twoatatoverthreebtat);
            let assign31030_e38238: f64 = (assign31030_e38236 * locals.var_sqrtumax);
            let assign31030_e38241: f64 = (locals.var_atatbot_d * locals.var_umax);
            let assign31030_e38242: f64 = (assign31030_e38238 - assign31030_e38241);
            let assign31030_e38246: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign31030_e38247: f64 = (0.5 * assign31030_e38246);
            let assign31030_e38248: f64 = (assign31030_e38242 + assign31030_e38247);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign31030_e38248, (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign31030_e38236 * locals.var_sqrtumax_dn5)) - (locals.var_atatbot_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign31030_e38236 * locals.var_sqrtumax_dn6)) - (locals.var_atatbot_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign31030_e38236 * locals.var_sqrtumax_dn7)) - (locals.var_atatbot_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign31030_e38236 * locals.var_sqrtumax_dn8)) - (locals.var_atatbot_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign31040_e38262: f64 = (locals.var_ltat - 1.0);
            let assign31040_e38264: f64 = (assign31040_e38262 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign31040_e38264, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign31040_e38262 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign31040_e38262 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign31040_e38262 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign31040_e38262 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign31050_e38278: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign31050_e38278, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign31060_e38283: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard605 = assign31060_e38283;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) && (locals.var_guard605 != 0.0)) {
            let assign31070_e38299: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign31070_e38300: f64 = (1.0 + assign31070_e38299);
            let assign31070_e38301: f64 = (1.0 / assign31070_e38300);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign31070_e38301, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign31070_e38300 * assign31070_e38300))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign31070_e38300 * assign31070_e38300))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign31070_e38300 * assign31070_e38300))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign31070_e38300 * assign31070_e38300))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) && (locals.var_guard605 == 0.0)) {
            let assign31080_e38320: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign31080_e38321: f64 = (1.0 - assign31080_e38320);
            let assign31080_e38322: f64 = (1.0 / assign31080_e38321);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign31080_e38322, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign31080_e38321 * assign31080_e38321))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign31080_e38321 * assign31080_e38321))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign31080_e38321 * assign31080_e38321))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign31080_e38321 * assign31080_e38321))), );
        }
        let assign31090_e38326: f64 = (-locals.var_ysq);
        let assign31090_e38328: f64 = (assign31090_e38326 + locals.var_mtat);
        let assign31090_e38330: f64 = (-230.25850929940458);
        let assign31090_e38331: f64 = if assign31090_e38328 > assign31090_e38330 { 1.0 } else { 0.0 };
        locals.var_guard606 = assign31090_e38331;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) && (locals.var_guard606 != 0.0)) {
            let assign31100_e38344: f64 = (-locals.var_ysq);
            let assign31100_e38346: f64 = (assign31100_e38344 + locals.var_mtat);
            let assign31100_e38347: f64 = (assign31100_e38346).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31100_e38347, (assign31100_e38347 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign31100_e38347 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign31100_e38347 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign31100_e38347 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) && (locals.var_guard606 == 0.0)) {
            let assign31110_e38365: f64 = (-230.25850929940458);
            let assign31110_e38367: f64 = (-locals.var_ysq);
            let assign31110_e38369: f64 = (assign31110_e38367 + locals.var_mtat);
            let assign31110_e38370: f64 = (assign31110_e38365 - assign31110_e38369);
            let assign31110_e38374: f64 = (-230.25850929940458);
            let assign31110_e38376: f64 = (-locals.var_ysq);
            let assign31110_e38378: f64 = (assign31110_e38376 + locals.var_mtat);
            let assign31110_e38379: f64 = (assign31110_e38374 - assign31110_e38378);
            let assign31110_e38382: f64 = (-230.25850929940458);
            let assign31110_e38384: f64 = (-locals.var_ysq);
            let assign31110_e38386: f64 = (assign31110_e38384 + locals.var_mtat);
            let assign31110_e38387: f64 = (assign31110_e38382 - assign31110_e38386);
            let assign31110_e38389: f64 = (assign31110_e38387 * 0.3333333333333333);
            let assign31110_e38390: f64 = (1.0 + assign31110_e38389);
            let assign31110_e38391: f64 = (assign31110_e38379 * assign31110_e38390);
            let assign31110_e38392: f64 = (0.5 * assign31110_e38391);
            let assign31110_e38393: f64 = (1.0 + assign31110_e38392);
            let assign31110_e38394: f64 = (assign31110_e38370 * assign31110_e38393);
            let assign31110_e38395: f64 = (1.0 + assign31110_e38394);
            let assign31110_e38396: f64 = (1e-100 / assign31110_e38395);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31110_e38396, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign31110_e38393) + (assign31110_e38370 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign31110_e38390) + (assign31110_e38379 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign31110_e38395 * assign31110_e38395))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign31110_e38393) + (assign31110_e38370 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign31110_e38390) + (assign31110_e38379 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign31110_e38395 * assign31110_e38395))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign31110_e38393) + (assign31110_e38370 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign31110_e38390) + (assign31110_e38379 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign31110_e38395 * assign31110_e38395))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign31110_e38393) + (assign31110_e38370 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign31110_e38390) + (assign31110_e38379 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign31110_e38395 * assign31110_e38395))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign31120_e38410: f64 = (0.29214664 * locals.var_terfc);
            let assign31120_e38414: f64 = (locals.var_terfc * locals.var_terfc);
            let assign31120_e38415: f64 = (locals.var_berfc * assign31120_e38414);
            let assign31120_e38416: f64 = (assign31120_e38410 + assign31120_e38415);
            let assign31120_e38420: f64 = (locals.var_terfc * locals.var_terfc);
            let assign31120_e38422: f64 = (assign31120_e38420 * locals.var_terfc);
            let assign31120_e38423: f64 = (locals.var_cerfc * assign31120_e38422);
            let assign31120_e38424: f64 = (assign31120_e38416 + assign31120_e38423);
            let assign31120_e38426: f64 = (assign31120_e38424 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign31120_e38426, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign31120_e38420 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign31120_e38424 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign31120_e38420 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign31120_e38424 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign31120_e38420 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign31120_e38424 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign31120_e38420 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign31120_e38424 * locals.var_tmp_dn8)), );
        }
        let assign31130_e38431: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard607 = assign31130_e38431;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) && (locals.var_guard607 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign31150_e38448: f64 = (-230.25850929940458);
        let assign31150_e38449: f64 = if locals.var_mtat > assign31150_e38448 { 1.0 } else { 0.0 };
        locals.var_guard608 = assign31150_e38449;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard608 != 0.0)) {
            let assign31160_e38465: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31160_e38465, (assign31160_e38465 * locals.var_mtat_dn5), (assign31160_e38465 * locals.var_mtat_dn6), (assign31160_e38465 * locals.var_mtat_dn7), (assign31160_e38465 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard608 == 0.0)) {
            let assign31170_e38486: f64 = (-230.25850929940458);
            let assign31170_e38488: f64 = (assign31170_e38486 - locals.var_mtat);
            let assign31170_e38492: f64 = (-230.25850929940458);
            let assign31170_e38494: f64 = (assign31170_e38492 - locals.var_mtat);
            let assign31170_e38497: f64 = (-230.25850929940458);
            let assign31170_e38499: f64 = (assign31170_e38497 - locals.var_mtat);
            let assign31170_e38501: f64 = (assign31170_e38499 * 0.3333333333333333);
            let assign31170_e38502: f64 = (1.0 + assign31170_e38501);
            let assign31170_e38503: f64 = (assign31170_e38494 * assign31170_e38502);
            let assign31170_e38504: f64 = (0.5 * assign31170_e38503);
            let assign31170_e38505: f64 = (1.0 + assign31170_e38504);
            let assign31170_e38506: f64 = (assign31170_e38488 * assign31170_e38505);
            let assign31170_e38507: f64 = (1.0 + assign31170_e38506);
            let assign31170_e38508: f64 = (1e-100 / assign31170_e38507);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31170_e38508, (-((1e-100 * (((-locals.var_mtat_dn5) * assign31170_e38505) + (assign31170_e38488 * (0.5 * (((-locals.var_mtat_dn5) * assign31170_e38502) + (assign31170_e38494 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign31170_e38507 * assign31170_e38507))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign31170_e38505) + (assign31170_e38488 * (0.5 * (((-locals.var_mtat_dn6) * assign31170_e38502) + (assign31170_e38494 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign31170_e38507 * assign31170_e38507))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign31170_e38505) + (assign31170_e38488 * (0.5 * (((-locals.var_mtat_dn7) * assign31170_e38502) + (assign31170_e38494 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign31170_e38507 * assign31170_e38507))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign31170_e38505) + (assign31170_e38488 * (0.5 * (((-locals.var_mtat_dn8) * assign31170_e38502) + (assign31170_e38494 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign31170_e38507 * assign31170_e38507))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) && (locals.var_guard607 == 0.0)) {
            let assign31180_e38525: f64 = (2.0 * locals.var_tmp);
            let assign31180_e38527: f64 = (assign31180_e38525 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign31180_e38527, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign31190_e38541: f64 = (1.772453850905516 * 0.5);
            let assign31190_e38544: f64 = (locals.var_atatbot_d * locals.var_erfctimesexpmtat);
            let assign31190_e38546: f64 = (assign31190_e38544 / locals.var_ktat);
            let assign31190_e38547: f64 = (assign31190_e38541 * assign31190_e38546);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign31190_e38547, (assign31190_e38541 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign31190_e38544 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign31190_e38541 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign31190_e38544 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign31190_e38541 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign31190_e38544 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign31190_e38541 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign31190_e38544 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard603 == 0.0)) {
            let assign31200_e38562: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign31200_e38564: f64 = (assign31200_e38562 * locals.var_wtat);
            let assign31200_e38565: f64 = (locals.var_ctatbotd_i * assign31200_e38564);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign31200_e38565, (locals.var_ctatbotd_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign31200_e38562 * locals.var_wtat_dn5))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign31200_e38562 * locals.var_wtat_dn6))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign31200_e38562 * locals.var_wtat_dn7))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign31200_e38562 * locals.var_wtat_dn8))), );
        }
        let assign31210_e38570: f64 = if locals.var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard609 = assign31210_e38570;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard609 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign31230_e38584: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard610 = assign31230_e38584;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard610 != 0.0)) {
            let assign31240_e38598: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign31240_e38600: f64 = (assign31240_e38598 * locals.var_vbirbotinv_d);
            let assign31240_e38601: f64 = (assign31240_e38600).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31240_e38601, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard610 == 0.0)) {
            let assign31250_e38618: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign31250_e38620: f64 = (assign31250_e38618 * locals.var_vbirbotinv_d);
            let assign31250_e38622: f64 = (assign31250_e38620).powf(locals.var_pbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31250_e38622, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard609 == 0.0)) {
            let assign31260_e38637: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign31260_e38639: f64 = (assign31260_e38637 * locals.var_wdepnulrinvbot_d);
            let assign31260_e38641: f64 = (assign31260_e38639 / locals.var_tmp);
            let assign31260_e38642: f64 = (locals.var_one_over_one_minus_pbot_d * assign31260_e38641);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign31260_e38642, (locals.var_one_over_one_minus_pbot_d * (-((assign31260_e38639 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign31260_e38639 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign31260_e38639 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign31260_e38639 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign31270_e38646: f64 = (-locals.var_fbbtbot_d);
        let assign31270_e38648: f64 = (assign31270_e38646 / locals.var_fmaxr);
        let assign31270_e38649: f64 = (assign31270_e38648).abs();
        let assign31270_e38651: f64 = if assign31270_e38649 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard611 = assign31270_e38651;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard611 != 0.0)) {
            let assign31280_e38664: f64 = (-locals.var_fbbtbot_d);
            let assign31280_e38666: f64 = (assign31280_e38664 / locals.var_fmaxr);
            let assign31280_e38667: f64 = (assign31280_e38666).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31280_e38667, (assign31280_e38667 * (-((assign31280_e38664 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign31280_e38667 * (-((assign31280_e38664 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign31280_e38667 * (-((assign31280_e38664 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign31280_e38667 * (-((assign31280_e38664 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign31290_e38671: f64 = (-locals.var_fbbtbot_d);
        let assign31290_e38673: f64 = (assign31290_e38671 / locals.var_fmaxr);
        let assign31290_e38675: f64 = if assign31290_e38673 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard612 = assign31290_e38675;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard611 == 0.0)) && (locals.var_guard612 != 0.0)) {
            let assign31300_e38693: f64 = (-230.25850929940458);
            let assign31300_e38695: f64 = (-locals.var_fbbtbot_d);
            let assign31300_e38697: f64 = (assign31300_e38695 / locals.var_fmaxr);
            let assign31300_e38698: f64 = (assign31300_e38693 - assign31300_e38697);
            let assign31300_e38702: f64 = (-230.25850929940458);
            let assign31300_e38704: f64 = (-locals.var_fbbtbot_d);
            let assign31300_e38706: f64 = (assign31300_e38704 / locals.var_fmaxr);
            let assign31300_e38707: f64 = (assign31300_e38702 - assign31300_e38706);
            let assign31300_e38710: f64 = (-230.25850929940458);
            let assign31300_e38712: f64 = (-locals.var_fbbtbot_d);
            let assign31300_e38714: f64 = (assign31300_e38712 / locals.var_fmaxr);
            let assign31300_e38715: f64 = (assign31300_e38710 - assign31300_e38714);
            let assign31300_e38717: f64 = (assign31300_e38715 * 0.3333333333333333);
            let assign31300_e38718: f64 = (1.0 + assign31300_e38717);
            let assign31300_e38719: f64 = (assign31300_e38707 * assign31300_e38718);
            let assign31300_e38720: f64 = (0.5 * assign31300_e38719);
            let assign31300_e38721: f64 = (1.0 + assign31300_e38720);
            let assign31300_e38722: f64 = (assign31300_e38698 * assign31300_e38721);
            let assign31300_e38723: f64 = (1.0 + assign31300_e38722);
            let assign31300_e38724: f64 = (1e-100 / assign31300_e38723);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31300_e38724, (-((1e-100 * (((-(-((assign31300_e38695 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign31300_e38721) + (assign31300_e38698 * (0.5 * (((-(-((assign31300_e38704 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign31300_e38718) + (assign31300_e38707 * ((-(-((assign31300_e38712 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign31300_e38723 * assign31300_e38723))), (-((1e-100 * (((-(-((assign31300_e38695 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign31300_e38721) + (assign31300_e38698 * (0.5 * (((-(-((assign31300_e38704 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign31300_e38718) + (assign31300_e38707 * ((-(-((assign31300_e38712 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign31300_e38723 * assign31300_e38723))), (-((1e-100 * (((-(-((assign31300_e38695 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign31300_e38721) + (assign31300_e38698 * (0.5 * (((-(-((assign31300_e38704 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign31300_e38718) + (assign31300_e38707 * ((-(-((assign31300_e38712 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign31300_e38723 * assign31300_e38723))), (-((1e-100 * (((-(-((assign31300_e38695 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign31300_e38721) + (assign31300_e38698 * (0.5 * (((-(-((assign31300_e38704 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign31300_e38718) + (assign31300_e38707 * ((-(-((assign31300_e38712 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign31300_e38723 * assign31300_e38723))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard611 == 0.0)) && (locals.var_guard612 == 0.0)) {
            let assign31310_e38745: f64 = (-locals.var_fbbtbot_d);
            let assign31310_e38747: f64 = (assign31310_e38745 / locals.var_fmaxr);
            let assign31310_e38749: f64 = (assign31310_e38747 - 230.25850929940458);
            let assign31310_e38753: f64 = (-locals.var_fbbtbot_d);
            let assign31310_e38755: f64 = (assign31310_e38753 / locals.var_fmaxr);
            let assign31310_e38757: f64 = (assign31310_e38755 - 230.25850929940458);
            let assign31310_e38760: f64 = (-locals.var_fbbtbot_d);
            let assign31310_e38762: f64 = (assign31310_e38760 / locals.var_fmaxr);
            let assign31310_e38764: f64 = (assign31310_e38762 - 230.25850929940458);
            let assign31310_e38766: f64 = (assign31310_e38764 * 0.3333333333333333);
            let assign31310_e38767: f64 = (1.0 + assign31310_e38766);
            let assign31310_e38768: f64 = (assign31310_e38757 * assign31310_e38767);
            let assign31310_e38769: f64 = (0.5 * assign31310_e38768);
            let assign31310_e38770: f64 = (1.0 + assign31310_e38769);
            let assign31310_e38771: f64 = (assign31310_e38749 * assign31310_e38770);
            let assign31310_e38772: f64 = (1.0 + assign31310_e38771);
            let assign31310_e38773: f64 = (1e100 * assign31310_e38772);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31310_e38773, (1e100 * (((-((assign31310_e38745 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign31310_e38770) + (assign31310_e38749 * (0.5 * (((-((assign31310_e38753 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign31310_e38767) + (assign31310_e38757 * ((-((assign31310_e38760 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign31310_e38745 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign31310_e38770) + (assign31310_e38749 * (0.5 * (((-((assign31310_e38753 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign31310_e38767) + (assign31310_e38757 * ((-((assign31310_e38760 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign31310_e38745 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign31310_e38770) + (assign31310_e38749 * (0.5 * (((-((assign31310_e38753 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign31310_e38767) + (assign31310_e38757 * ((-((assign31310_e38760 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign31310_e38745 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign31310_e38770) + (assign31310_e38749 * (0.5 * (((-((assign31310_e38753 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign31310_e38767) + (assign31310_e38757 * ((-((assign31310_e38760 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard609 == 0.0)) {
            let assign31320_e38788: f64 = (locals.var_v2 * locals.var_fmaxr);
            let assign31320_e38790: f64 = (assign31320_e38788 * locals.var_fmaxr);
            let assign31320_e38792: f64 = (assign31320_e38790 * locals.var_tmp);
            let assign31320_e38793: f64 = (locals.var_cbbtbotd_i * assign31320_e38792);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign31320_e38793, (locals.var_cbbtbotd_i * (((((locals.var_v2 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign31320_e38788 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign31320_e38790 * locals.var_tmp_dn5))), (locals.var_cbbtbotd_i * (((((locals.var_v2 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign31320_e38788 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign31320_e38790 * locals.var_tmp_dn6))), (locals.var_cbbtbotd_i * (((((locals.var_v2 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign31320_e38788 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign31320_e38790 * locals.var_tmp_dn7))), (locals.var_cbbtbotd_i * (((((locals.var_v2 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign31320_e38788 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign31320_e38790 * locals.var_tmp_dn8))), );
        }
        let assign31330_e38798: f64 = if locals.var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard613 = assign31330_e38798;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard613 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign31350_e38812: f64 = (-locals.var_alphaav);
        let assign31350_e38814: f64 = (assign31350_e38812 * locals.var_vbrbotd_i);
        let assign31350_e38815: f64 = if locals.var_vav > assign31350_e38814 { 1.0 } else { 0.0 };
        locals.var_guard614 = assign31350_e38815;
        let assign31360_e38818: f64 = if locals.var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard615 = assign31360_e38818;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard613 == 0.0)) && (locals.var_guard614 != 0.0)) && (locals.var_guard615 != 0.0)) {
            let assign31370_e38834: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign31370_e38837: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign31370_e38838: f64 = (assign31370_e38834 * assign31370_e38837);
            let assign31370_e38841: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign31370_e38842: f64 = (assign31370_e38838 * assign31370_e38841);
            let assign31370_e38845: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign31370_e38846: f64 = (assign31370_e38842 * assign31370_e38845);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31370_e38846, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard613 == 0.0)) && (locals.var_guard614 != 0.0)) && (locals.var_guard615 == 0.0)) {
            let assign31380_e38865: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign31380_e38866: f64 = (assign31380_e38865).abs();
            let assign31380_e38868: f64 = (assign31380_e38866).powf(locals.var_pbrbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31380_e38868, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard613 == 0.0)) && (locals.var_guard614 != 0.0)) {
            let assign31390_e38885: f64 = (1.0 - locals.var_tmp);
            let assign31390_e38886: f64 = (1.0 / assign31390_e38885);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign31390_e38886, (-((-locals.var_tmp_dn5) / (assign31390_e38885 * assign31390_e38885))), (-((-locals.var_tmp_dn6) / (assign31390_e38885 * assign31390_e38885))), (-((-locals.var_tmp_dn7) / (assign31390_e38885 * assign31390_e38885))), (-((-locals.var_tmp_dn8) / (assign31390_e38885 * assign31390_e38885))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) && (locals.var_guard613 == 0.0)) && (locals.var_guard614 == 0.0)) {
            let assign31400_e38905: f64 = (locals.var_alphaav * locals.var_vbrbotd_i);
            let assign31400_e38906: f64 = (locals.var_vav + assign31400_e38905);
            let assign31400_e38908: f64 = (assign31400_e38906 * locals.var_slopebot_d);
            let assign31400_e38909: f64 = (locals.var_fstopbot_d + assign31400_e38908);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign31400_e38909, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard599 == 0.0)) {
            let assign31410_e38921: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign31410_e38923: f64 = (assign31410_e38921 + locals.var_itat);
            let assign31410_e38925: f64 = (assign31410_e38923 + locals.var_ibbt);
            let assign31410_e38926: f64 = (p.p29 * assign31410_e38925);
            let assign31410_e38928: f64 = (assign31410_e38926 * locals.var_fbreakdown);
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (assign31410_e38928, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign31410_e38926 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign31410_e38926 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign31410_e38926 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign31410_e38926 * locals.var_fbreakdown_dn8)), );
        }
        let assign31420_e38933: f64 = if locals.var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard616 = assign31420_e38933;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 != 0.0)) {
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) {
            let assign31440_e38950: f64 = (locals.var_idsatsti_d * locals.var_idmult);
            locals.var_id__blk219 = assign31440_e38950;
        }
        let assign31450_e38959: f64 = if ((locals.var_csrhstid_i == 0.0) && (locals.var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard617 = assign31450_e38959;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard617 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard617 == 0.0)) {
            let assign31470_e38982: f64 = (locals.var_vbisti_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign31470_e38982;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard617 == 0.0)) {
            let assign31480_e38998: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign31480_e38999: f64 = (1.0 - assign31480_e38998);
            let assign31480_e39000: f64 = (assign31480_e38999).sqrt();
            let assign31480_e39001: f64 = (1.0 - assign31480_e39000);
            locals.var_wsrhstep = assign31480_e39001;
        }
        let assign31490_e39006: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard618 = assign31490_e39006;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard618 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard618 == 0.0)) {
            let assign31510_e39035: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign31510_e39037: f64 = (locals.var_wsrhstep).ln();
            let assign31510_e39038: f64 = (assign31510_e39035 * assign31510_e39037);
            let assign31510_e39041: f64 = (1.0 - locals.var_wsrhstep);
            let assign31510_e39042: f64 = (assign31510_e39038 / assign31510_e39041);
            let assign31510_e39044: f64 = (assign31510_e39042 + locals.var_wsrhstep);
            let assign31510_e39048: f64 = (2.0 * locals.var_pstid_i);
            let assign31510_e39049: f64 = (1.0 - assign31510_e39048);
            let assign31510_e39050: f64 = (assign31510_e39044 * assign31510_e39049);
            locals.var_dwsrh = assign31510_e39050;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard617 == 0.0)) {
            let assign31520_e39064: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign31520_e39064;
        }
        let assign31530_e39069: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard619 = assign31530_e39069;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard619 != 0.0)) {
            let assign31540_e39083: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv_d);
            let assign31540_e39084: f64 = (assign31540_e39083).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31540_e39084, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard619 == 0.0)) {
            let assign31550_e39101: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv_d);
            let assign31550_e39103: f64 = (assign31550_e39101).powf(locals.var_pstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31550_e39103, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard617 == 0.0)) {
            let assign31560_e39117: f64 = (locals.var_wdepnulrsti_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign31560_e39117, (locals.var_wdepnulrsti_d * locals.var_tmp_dn5), (locals.var_wdepnulrsti_d * locals.var_tmp_dn6), (locals.var_wdepnulrsti_d * locals.var_tmp_dn7), (locals.var_wdepnulrsti_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard617 == 0.0)) {
            let assign31570_e39132: f64 = (locals.var_zinv - 1.0);
            let assign31570_e39134: f64 = (assign31570_e39132 * locals.var_wdep);
            let assign31570_e39135: f64 = (locals.var_ftdsti_d * assign31570_e39134);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign31570_e39135, (locals.var_ftdsti_d * (assign31570_e39132 * locals.var_wdep_dn5)), (locals.var_ftdsti_d * (assign31570_e39132 * locals.var_wdep_dn6)), (locals.var_ftdsti_d * (assign31570_e39132 * locals.var_wdep_dn7)), (locals.var_ftdsti_d * (assign31570_e39132 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard617 == 0.0)) {
            let assign31580_e39150: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign31580_e39151: f64 = (locals.var_csrhstid_i * assign31580_e39150);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign31580_e39151, (locals.var_csrhstid_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign31590_e39156: f64 = if locals.var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard620 = assign31590_e39156;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31610_e39180: f64 = (locals.var_wdep * locals.var_one_minus_psti_d);
            let assign31610_e39182: f64 = (assign31610_e39180 / locals.var_vbi_minus_vjsrh);
            let assign31610_e39183: f64 = (locals.var_btatpartsti_d * assign31610_e39182);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign31610_e39183, (locals.var_btatpartsti_d * ((locals.var_wdep_dn5 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn6 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn7 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn8 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31620_e39197: f64 = (0.666666666666667 * locals.var_atatsti_d);
            let assign31620_e39199: f64 = (assign31620_e39197 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign31620_e39199, (-((assign31620_e39197 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign31620_e39197 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign31620_e39197 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign31620_e39197 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31630_e39213: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign31630_e39213, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31640_e39227: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign31640_e39230: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign31640_e39232: f64 = (assign31640_e39230 + 1.0);
            let assign31640_e39233: f64 = (assign31640_e39227 / assign31640_e39232);
            let assign31640_e39234: f64 = (assign31640_e39233).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign31640_e39234, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign31640_e39232) - (assign31640_e39227 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign31640_e39232 * assign31640_e39232)) / (2.0 * assign31640_e39234)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign31640_e39232) - (assign31640_e39227 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign31640_e39232 * assign31640_e39232)) / (2.0 * assign31640_e39234)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign31640_e39232) - (assign31640_e39227 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign31640_e39232 * assign31640_e39232)) / (2.0 * assign31640_e39234)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign31640_e39232) - (assign31640_e39227 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign31640_e39232 * assign31640_e39232)) / (2.0 * assign31640_e39234)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31650_e39247: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign31650_e39247, (locals.var_umax_dn5 / (2.0 * assign31650_e39247)), (locals.var_umax_dn6 / (2.0 * assign31650_e39247)), (locals.var_umax_dn7 / (2.0 * assign31650_e39247)), (locals.var_umax_dn8 / (2.0 * assign31650_e39247)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31660_e39261: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign31660_e39261, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign31670_e39265: f64 = (-locals.var_pstid_i);
        let assign31670_e39267: f64 = (assign31670_e39265 * locals.var_one_over_one_minus_psti_d);
        let assign31670_e39269: f64 = (-1.0);
        let assign31670_e39270: f64 = if assign31670_e39267 == assign31670_e39269 { 1.0 } else { 0.0 };
        locals.var_guard621 = assign31670_e39270;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) && (locals.var_guard621 != 0.0)) {
            let assign31680_e39286: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign31680_e39287: f64 = (1.0 + assign31680_e39286);
            let assign31680_e39288: f64 = (1.0 / assign31680_e39287);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign31680_e39288, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign31680_e39287 * assign31680_e39287))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign31680_e39287 * assign31680_e39287))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign31680_e39287 * assign31680_e39287))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign31680_e39287 * assign31680_e39287))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) && (locals.var_guard621 == 0.0)) {
            let assign31690_e39306: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign31690_e39307: f64 = (1.0 + assign31690_e39306);
            let assign31690_e39309: f64 = (-locals.var_pstid_i);
            let assign31690_e39311: f64 = (assign31690_e39309 * locals.var_one_over_one_minus_psti_d);
            let assign31690_e39312: f64 = (assign31690_e39307).powf(assign31690_e39311);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign31690_e39312, if 0.0 == 0.0 && ((assign31690_e39311) as f64).is_finite() && ((assign31690_e39311) as f64).fract() == 0.0 { if assign31690_e39311 == 0.0 { 0.0 } else { (assign31690_e39311 * ((assign31690_e39307).powf(assign31690_e39311 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign31690_e39312 * (assign31690_e39311 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign31690_e39307))) }, if 0.0 == 0.0 && ((assign31690_e39311) as f64).is_finite() && ((assign31690_e39311) as f64).fract() == 0.0 { if assign31690_e39311 == 0.0 { 0.0 } else { (assign31690_e39311 * ((assign31690_e39307).powf(assign31690_e39311 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign31690_e39312 * (assign31690_e39311 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign31690_e39307))) }, if 0.0 == 0.0 && ((assign31690_e39311) as f64).is_finite() && ((assign31690_e39311) as f64).fract() == 0.0 { if assign31690_e39311 == 0.0 { 0.0 } else { (assign31690_e39311 * ((assign31690_e39307).powf(assign31690_e39311 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign31690_e39312 * (assign31690_e39311 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign31690_e39307))) }, if 0.0 == 0.0 && ((assign31690_e39311) as f64).is_finite() && ((assign31690_e39311) as f64).fract() == 0.0 { if assign31690_e39311 == 0.0 { 0.0 } else { (assign31690_e39311 * ((assign31690_e39307).powf(assign31690_e39311 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign31690_e39312 * (assign31690_e39311 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign31690_e39307))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31700_e39326: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign31700_e39329: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign31700_e39330: f64 = (assign31700_e39326 / assign31700_e39329);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign31700_e39330, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign31700_e39329) - (assign31700_e39326 * locals.var_wgamma_dn5)) / (assign31700_e39329 * assign31700_e39329)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign31700_e39329) - (assign31700_e39326 * locals.var_wgamma_dn6)) / (assign31700_e39329 * assign31700_e39329)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign31700_e39329) - (assign31700_e39326 * locals.var_wgamma_dn7)) / (assign31700_e39329 * assign31700_e39329)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign31700_e39329) - (assign31700_e39326 * locals.var_wgamma_dn8)) / (assign31700_e39329 * assign31700_e39329)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31710_e39345: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign31710_e39346: f64 = (0.375 * assign31710_e39345);
            let assign31710_e39347: f64 = (assign31710_e39346).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign31710_e39347, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign31710_e39347)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign31710_e39347)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign31710_e39347)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign31710_e39347)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31720_e39362: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign31720_e39363: f64 = (2.0 * assign31720_e39362);
            let assign31720_e39365: f64 = (assign31720_e39363 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign31720_e39365, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
    }
    pub(super) fn stamp_transient_block_37(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31730_e39379: f64 = (locals.var_atatsti_d * locals.var_twoatatoverthreebtat);
            let assign31730_e39381: f64 = (assign31730_e39379 * locals.var_sqrtumax);
            let assign31730_e39384: f64 = (locals.var_atatsti_d * locals.var_umax);
            let assign31730_e39385: f64 = (assign31730_e39381 - assign31730_e39384);
            let assign31730_e39389: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign31730_e39390: f64 = (0.5 * assign31730_e39389);
            let assign31730_e39391: f64 = (assign31730_e39385 + assign31730_e39390);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign31730_e39391, (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign31730_e39379 * locals.var_sqrtumax_dn5)) - (locals.var_atatsti_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign31730_e39379 * locals.var_sqrtumax_dn6)) - (locals.var_atatsti_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign31730_e39379 * locals.var_sqrtumax_dn7)) - (locals.var_atatsti_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign31730_e39379 * locals.var_sqrtumax_dn8)) - (locals.var_atatsti_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31740_e39405: f64 = (locals.var_ltat - 1.0);
            let assign31740_e39407: f64 = (assign31740_e39405 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign31740_e39407, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign31740_e39405 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign31740_e39405 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign31740_e39405 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign31740_e39405 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31750_e39421: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign31750_e39421, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign31760_e39426: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard622 = assign31760_e39426;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) && (locals.var_guard622 != 0.0)) {
            let assign31770_e39442: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign31770_e39443: f64 = (1.0 + assign31770_e39442);
            let assign31770_e39444: f64 = (1.0 / assign31770_e39443);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign31770_e39444, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign31770_e39443 * assign31770_e39443))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign31770_e39443 * assign31770_e39443))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign31770_e39443 * assign31770_e39443))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign31770_e39443 * assign31770_e39443))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) && (locals.var_guard622 == 0.0)) {
            let assign31780_e39463: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign31780_e39464: f64 = (1.0 - assign31780_e39463);
            let assign31780_e39465: f64 = (1.0 / assign31780_e39464);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign31780_e39465, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign31780_e39464 * assign31780_e39464))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign31780_e39464 * assign31780_e39464))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign31780_e39464 * assign31780_e39464))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign31780_e39464 * assign31780_e39464))), );
        }
        let assign31790_e39469: f64 = (-locals.var_ysq);
        let assign31790_e39471: f64 = (assign31790_e39469 + locals.var_mtat);
        let assign31790_e39473: f64 = (-230.25850929940458);
        let assign31790_e39474: f64 = if assign31790_e39471 > assign31790_e39473 { 1.0 } else { 0.0 };
        locals.var_guard623 = assign31790_e39474;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) && (locals.var_guard623 != 0.0)) {
            let assign31800_e39487: f64 = (-locals.var_ysq);
            let assign31800_e39489: f64 = (assign31800_e39487 + locals.var_mtat);
            let assign31800_e39490: f64 = (assign31800_e39489).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31800_e39490, (assign31800_e39490 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign31800_e39490 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign31800_e39490 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign31800_e39490 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) && (locals.var_guard623 == 0.0)) {
            let assign31810_e39508: f64 = (-230.25850929940458);
            let assign31810_e39510: f64 = (-locals.var_ysq);
            let assign31810_e39512: f64 = (assign31810_e39510 + locals.var_mtat);
            let assign31810_e39513: f64 = (assign31810_e39508 - assign31810_e39512);
            let assign31810_e39517: f64 = (-230.25850929940458);
            let assign31810_e39519: f64 = (-locals.var_ysq);
            let assign31810_e39521: f64 = (assign31810_e39519 + locals.var_mtat);
            let assign31810_e39522: f64 = (assign31810_e39517 - assign31810_e39521);
            let assign31810_e39525: f64 = (-230.25850929940458);
            let assign31810_e39527: f64 = (-locals.var_ysq);
            let assign31810_e39529: f64 = (assign31810_e39527 + locals.var_mtat);
            let assign31810_e39530: f64 = (assign31810_e39525 - assign31810_e39529);
            let assign31810_e39532: f64 = (assign31810_e39530 * 0.3333333333333333);
            let assign31810_e39533: f64 = (1.0 + assign31810_e39532);
            let assign31810_e39534: f64 = (assign31810_e39522 * assign31810_e39533);
            let assign31810_e39535: f64 = (0.5 * assign31810_e39534);
            let assign31810_e39536: f64 = (1.0 + assign31810_e39535);
            let assign31810_e39537: f64 = (assign31810_e39513 * assign31810_e39536);
            let assign31810_e39538: f64 = (1.0 + assign31810_e39537);
            let assign31810_e39539: f64 = (1e-100 / assign31810_e39538);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31810_e39539, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign31810_e39536) + (assign31810_e39513 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign31810_e39533) + (assign31810_e39522 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign31810_e39538 * assign31810_e39538))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign31810_e39536) + (assign31810_e39513 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign31810_e39533) + (assign31810_e39522 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign31810_e39538 * assign31810_e39538))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign31810_e39536) + (assign31810_e39513 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign31810_e39533) + (assign31810_e39522 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign31810_e39538 * assign31810_e39538))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign31810_e39536) + (assign31810_e39513 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign31810_e39533) + (assign31810_e39522 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign31810_e39538 * assign31810_e39538))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31820_e39553: f64 = (0.29214664 * locals.var_terfc);
            let assign31820_e39557: f64 = (locals.var_terfc * locals.var_terfc);
            let assign31820_e39558: f64 = (locals.var_berfc * assign31820_e39557);
            let assign31820_e39559: f64 = (assign31820_e39553 + assign31820_e39558);
            let assign31820_e39563: f64 = (locals.var_terfc * locals.var_terfc);
            let assign31820_e39565: f64 = (assign31820_e39563 * locals.var_terfc);
            let assign31820_e39566: f64 = (locals.var_cerfc * assign31820_e39565);
            let assign31820_e39567: f64 = (assign31820_e39559 + assign31820_e39566);
            let assign31820_e39569: f64 = (assign31820_e39567 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign31820_e39569, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign31820_e39563 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign31820_e39567 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign31820_e39563 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign31820_e39567 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign31820_e39563 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign31820_e39567 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign31820_e39563 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign31820_e39567 * locals.var_tmp_dn8)), );
        }
        let assign31830_e39574: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard624 = assign31830_e39574;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) && (locals.var_guard624 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign31850_e39591: f64 = (-230.25850929940458);
        let assign31850_e39592: f64 = if locals.var_mtat > assign31850_e39591 { 1.0 } else { 0.0 };
        locals.var_guard625 = assign31850_e39592;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) && (locals.var_guard624 == 0.0)) && (locals.var_guard625 != 0.0)) {
            let assign31860_e39608: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31860_e39608, (assign31860_e39608 * locals.var_mtat_dn5), (assign31860_e39608 * locals.var_mtat_dn6), (assign31860_e39608 * locals.var_mtat_dn7), (assign31860_e39608 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) && (locals.var_guard624 == 0.0)) && (locals.var_guard625 == 0.0)) {
            let assign31870_e39629: f64 = (-230.25850929940458);
            let assign31870_e39631: f64 = (assign31870_e39629 - locals.var_mtat);
            let assign31870_e39635: f64 = (-230.25850929940458);
            let assign31870_e39637: f64 = (assign31870_e39635 - locals.var_mtat);
            let assign31870_e39640: f64 = (-230.25850929940458);
            let assign31870_e39642: f64 = (assign31870_e39640 - locals.var_mtat);
            let assign31870_e39644: f64 = (assign31870_e39642 * 0.3333333333333333);
            let assign31870_e39645: f64 = (1.0 + assign31870_e39644);
            let assign31870_e39646: f64 = (assign31870_e39637 * assign31870_e39645);
            let assign31870_e39647: f64 = (0.5 * assign31870_e39646);
            let assign31870_e39648: f64 = (1.0 + assign31870_e39647);
            let assign31870_e39649: f64 = (assign31870_e39631 * assign31870_e39648);
            let assign31870_e39650: f64 = (1.0 + assign31870_e39649);
            let assign31870_e39651: f64 = (1e-100 / assign31870_e39650);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31870_e39651, (-((1e-100 * (((-locals.var_mtat_dn5) * assign31870_e39648) + (assign31870_e39631 * (0.5 * (((-locals.var_mtat_dn5) * assign31870_e39645) + (assign31870_e39637 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign31870_e39650 * assign31870_e39650))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign31870_e39648) + (assign31870_e39631 * (0.5 * (((-locals.var_mtat_dn6) * assign31870_e39645) + (assign31870_e39637 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign31870_e39650 * assign31870_e39650))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign31870_e39648) + (assign31870_e39631 * (0.5 * (((-locals.var_mtat_dn7) * assign31870_e39645) + (assign31870_e39637 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign31870_e39650 * assign31870_e39650))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign31870_e39648) + (assign31870_e39631 * (0.5 * (((-locals.var_mtat_dn8) * assign31870_e39645) + (assign31870_e39637 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign31870_e39650 * assign31870_e39650))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) && (locals.var_guard624 == 0.0)) {
            let assign31880_e39668: f64 = (2.0 * locals.var_tmp);
            let assign31880_e39670: f64 = (assign31880_e39668 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign31880_e39670, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31890_e39684: f64 = (1.772453850905516 * 0.5);
            let assign31890_e39687: f64 = (locals.var_atatsti_d * locals.var_erfctimesexpmtat);
            let assign31890_e39689: f64 = (assign31890_e39687 / locals.var_ktat);
            let assign31890_e39690: f64 = (assign31890_e39684 * assign31890_e39689);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign31890_e39690, (assign31890_e39684 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign31890_e39687 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign31890_e39684 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign31890_e39687 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign31890_e39684 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign31890_e39687 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign31890_e39684 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign31890_e39687 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard620 == 0.0)) {
            let assign31900_e39705: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign31900_e39707: f64 = (assign31900_e39705 * locals.var_wtat);
            let assign31900_e39708: f64 = (locals.var_ctatstid_i * assign31900_e39707);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign31900_e39708, (locals.var_ctatstid_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign31900_e39705 * locals.var_wtat_dn5))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign31900_e39705 * locals.var_wtat_dn6))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign31900_e39705 * locals.var_wtat_dn7))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign31900_e39705 * locals.var_wtat_dn8))), );
        }
        let assign31910_e39713: f64 = if locals.var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard626 = assign31910_e39713;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard626 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign31930_e39727: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard627 = assign31930_e39727;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard626 == 0.0)) && (locals.var_guard627 != 0.0)) {
            let assign31940_e39741: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign31940_e39743: f64 = (assign31940_e39741 * locals.var_vbirstiinv_d);
            let assign31940_e39744: f64 = (assign31940_e39743).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31940_e39744, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard626 == 0.0)) && (locals.var_guard627 == 0.0)) {
            let assign31950_e39761: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign31950_e39763: f64 = (assign31950_e39761 * locals.var_vbirstiinv_d);
            let assign31950_e39765: f64 = (assign31950_e39763).powf(locals.var_pstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31950_e39765, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard626 == 0.0)) {
            let assign31960_e39780: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign31960_e39782: f64 = (assign31960_e39780 * locals.var_wdepnulrinvsti_d);
            let assign31960_e39784: f64 = (assign31960_e39782 / locals.var_tmp);
            let assign31960_e39785: f64 = (locals.var_one_over_one_minus_psti_d * assign31960_e39784);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign31960_e39785, (locals.var_one_over_one_minus_psti_d * (-((assign31960_e39782 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign31960_e39782 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign31960_e39782 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign31960_e39782 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign31970_e39789: f64 = (-locals.var_fbbtsti_d);
        let assign31970_e39791: f64 = (assign31970_e39789 / locals.var_fmaxr);
        let assign31970_e39792: f64 = (assign31970_e39791).abs();
        let assign31970_e39794: f64 = if assign31970_e39792 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard628 = assign31970_e39794;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard626 == 0.0)) && (locals.var_guard628 != 0.0)) {
            let assign31980_e39807: f64 = (-locals.var_fbbtsti_d);
            let assign31980_e39809: f64 = (assign31980_e39807 / locals.var_fmaxr);
            let assign31980_e39810: f64 = (assign31980_e39809).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign31980_e39810, (assign31980_e39810 * (-((assign31980_e39807 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign31980_e39810 * (-((assign31980_e39807 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign31980_e39810 * (-((assign31980_e39807 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign31980_e39810 * (-((assign31980_e39807 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign31990_e39814: f64 = (-locals.var_fbbtsti_d);
        let assign31990_e39816: f64 = (assign31990_e39814 / locals.var_fmaxr);
        let assign31990_e39818: f64 = if assign31990_e39816 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard629 = assign31990_e39818;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard626 == 0.0)) && (locals.var_guard628 == 0.0)) && (locals.var_guard629 != 0.0)) {
            let assign32000_e39836: f64 = (-230.25850929940458);
            let assign32000_e39838: f64 = (-locals.var_fbbtsti_d);
            let assign32000_e39840: f64 = (assign32000_e39838 / locals.var_fmaxr);
            let assign32000_e39841: f64 = (assign32000_e39836 - assign32000_e39840);
            let assign32000_e39845: f64 = (-230.25850929940458);
            let assign32000_e39847: f64 = (-locals.var_fbbtsti_d);
            let assign32000_e39849: f64 = (assign32000_e39847 / locals.var_fmaxr);
            let assign32000_e39850: f64 = (assign32000_e39845 - assign32000_e39849);
            let assign32000_e39853: f64 = (-230.25850929940458);
            let assign32000_e39855: f64 = (-locals.var_fbbtsti_d);
            let assign32000_e39857: f64 = (assign32000_e39855 / locals.var_fmaxr);
            let assign32000_e39858: f64 = (assign32000_e39853 - assign32000_e39857);
            let assign32000_e39860: f64 = (assign32000_e39858 * 0.3333333333333333);
            let assign32000_e39861: f64 = (1.0 + assign32000_e39860);
            let assign32000_e39862: f64 = (assign32000_e39850 * assign32000_e39861);
            let assign32000_e39863: f64 = (0.5 * assign32000_e39862);
            let assign32000_e39864: f64 = (1.0 + assign32000_e39863);
            let assign32000_e39865: f64 = (assign32000_e39841 * assign32000_e39864);
            let assign32000_e39866: f64 = (1.0 + assign32000_e39865);
            let assign32000_e39867: f64 = (1e-100 / assign32000_e39866);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32000_e39867, (-((1e-100 * (((-(-((assign32000_e39838 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign32000_e39864) + (assign32000_e39841 * (0.5 * (((-(-((assign32000_e39847 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign32000_e39861) + (assign32000_e39850 * ((-(-((assign32000_e39855 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign32000_e39866 * assign32000_e39866))), (-((1e-100 * (((-(-((assign32000_e39838 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign32000_e39864) + (assign32000_e39841 * (0.5 * (((-(-((assign32000_e39847 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign32000_e39861) + (assign32000_e39850 * ((-(-((assign32000_e39855 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign32000_e39866 * assign32000_e39866))), (-((1e-100 * (((-(-((assign32000_e39838 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign32000_e39864) + (assign32000_e39841 * (0.5 * (((-(-((assign32000_e39847 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign32000_e39861) + (assign32000_e39850 * ((-(-((assign32000_e39855 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign32000_e39866 * assign32000_e39866))), (-((1e-100 * (((-(-((assign32000_e39838 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign32000_e39864) + (assign32000_e39841 * (0.5 * (((-(-((assign32000_e39847 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign32000_e39861) + (assign32000_e39850 * ((-(-((assign32000_e39855 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign32000_e39866 * assign32000_e39866))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard626 == 0.0)) && (locals.var_guard628 == 0.0)) && (locals.var_guard629 == 0.0)) {
            let assign32010_e39888: f64 = (-locals.var_fbbtsti_d);
            let assign32010_e39890: f64 = (assign32010_e39888 / locals.var_fmaxr);
            let assign32010_e39892: f64 = (assign32010_e39890 - 230.25850929940458);
            let assign32010_e39896: f64 = (-locals.var_fbbtsti_d);
            let assign32010_e39898: f64 = (assign32010_e39896 / locals.var_fmaxr);
            let assign32010_e39900: f64 = (assign32010_e39898 - 230.25850929940458);
            let assign32010_e39903: f64 = (-locals.var_fbbtsti_d);
            let assign32010_e39905: f64 = (assign32010_e39903 / locals.var_fmaxr);
            let assign32010_e39907: f64 = (assign32010_e39905 - 230.25850929940458);
            let assign32010_e39909: f64 = (assign32010_e39907 * 0.3333333333333333);
            let assign32010_e39910: f64 = (1.0 + assign32010_e39909);
            let assign32010_e39911: f64 = (assign32010_e39900 * assign32010_e39910);
            let assign32010_e39912: f64 = (0.5 * assign32010_e39911);
            let assign32010_e39913: f64 = (1.0 + assign32010_e39912);
            let assign32010_e39914: f64 = (assign32010_e39892 * assign32010_e39913);
            let assign32010_e39915: f64 = (1.0 + assign32010_e39914);
            let assign32010_e39916: f64 = (1e100 * assign32010_e39915);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32010_e39916, (1e100 * (((-((assign32010_e39888 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32010_e39913) + (assign32010_e39892 * (0.5 * (((-((assign32010_e39896 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32010_e39910) + (assign32010_e39900 * ((-((assign32010_e39903 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign32010_e39888 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32010_e39913) + (assign32010_e39892 * (0.5 * (((-((assign32010_e39896 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32010_e39910) + (assign32010_e39900 * ((-((assign32010_e39903 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign32010_e39888 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32010_e39913) + (assign32010_e39892 * (0.5 * (((-((assign32010_e39896 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32010_e39910) + (assign32010_e39900 * ((-((assign32010_e39903 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign32010_e39888 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32010_e39913) + (assign32010_e39892 * (0.5 * (((-((assign32010_e39896 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32010_e39910) + (assign32010_e39900 * ((-((assign32010_e39903 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard626 == 0.0)) {
            let assign32020_e39931: f64 = (locals.var_v2 * locals.var_fmaxr);
            let assign32020_e39933: f64 = (assign32020_e39931 * locals.var_fmaxr);
            let assign32020_e39935: f64 = (assign32020_e39933 * locals.var_tmp);
            let assign32020_e39936: f64 = (locals.var_cbbtstid_i * assign32020_e39935);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign32020_e39936, (locals.var_cbbtstid_i * (((((locals.var_v2 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign32020_e39931 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign32020_e39933 * locals.var_tmp_dn5))), (locals.var_cbbtstid_i * (((((locals.var_v2 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign32020_e39931 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign32020_e39933 * locals.var_tmp_dn6))), (locals.var_cbbtstid_i * (((((locals.var_v2 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign32020_e39931 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign32020_e39933 * locals.var_tmp_dn7))), (locals.var_cbbtstid_i * (((((locals.var_v2 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign32020_e39931 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign32020_e39933 * locals.var_tmp_dn8))), );
        }
        let assign32030_e39941: f64 = if locals.var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard630 = assign32030_e39941;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard630 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign32050_e39955: f64 = (-locals.var_alphaav);
        let assign32050_e39957: f64 = (assign32050_e39955 * locals.var_vbrstid_i);
        let assign32050_e39958: f64 = if locals.var_vav > assign32050_e39957 { 1.0 } else { 0.0 };
        locals.var_guard631 = assign32050_e39958;
        let assign32060_e39961: f64 = if locals.var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard632 = assign32060_e39961;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard630 == 0.0)) && (locals.var_guard631 != 0.0)) && (locals.var_guard632 != 0.0)) {
            let assign32070_e39977: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign32070_e39980: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign32070_e39981: f64 = (assign32070_e39977 * assign32070_e39980);
            let assign32070_e39984: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign32070_e39985: f64 = (assign32070_e39981 * assign32070_e39984);
            let assign32070_e39988: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign32070_e39989: f64 = (assign32070_e39985 * assign32070_e39988);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32070_e39989, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard630 == 0.0)) && (locals.var_guard631 != 0.0)) && (locals.var_guard632 == 0.0)) {
            let assign32080_e40008: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign32080_e40009: f64 = (assign32080_e40008).abs();
            let assign32080_e40011: f64 = (assign32080_e40009).powf(locals.var_pbrstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32080_e40011, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard630 == 0.0)) && (locals.var_guard631 != 0.0)) {
            let assign32090_e40028: f64 = (1.0 - locals.var_tmp);
            let assign32090_e40029: f64 = (1.0 / assign32090_e40028);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign32090_e40029, (-((-locals.var_tmp_dn5) / (assign32090_e40028 * assign32090_e40028))), (-((-locals.var_tmp_dn6) / (assign32090_e40028 * assign32090_e40028))), (-((-locals.var_tmp_dn7) / (assign32090_e40028 * assign32090_e40028))), (-((-locals.var_tmp_dn8) / (assign32090_e40028 * assign32090_e40028))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) && (locals.var_guard630 == 0.0)) && (locals.var_guard631 == 0.0)) {
            let assign32100_e40048: f64 = (locals.var_alphaav * locals.var_vbrstid_i);
            let assign32100_e40049: f64 = (locals.var_vav + assign32100_e40048);
            let assign32100_e40051: f64 = (assign32100_e40049 * locals.var_slopesti_d);
            let assign32100_e40052: f64 = (locals.var_fstopsti_d + assign32100_e40051);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign32100_e40052, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard616 == 0.0)) {
            let assign32110_e40064: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign32110_e40066: f64 = (assign32110_e40064 + locals.var_itat);
            let assign32110_e40068: f64 = (assign32110_e40066 + locals.var_ibbt);
            let assign32110_e40069: f64 = (p.p29 * assign32110_e40068);
            let assign32110_e40071: f64 = (assign32110_e40069 * locals.var_fbreakdown);
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (assign32110_e40071, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign32110_e40069 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign32110_e40069 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign32110_e40069 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign32110_e40069 * locals.var_fbreakdown_dn8)), );
        }
        let assign32120_e40076: f64 = if locals.var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard633 = assign32120_e40076;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 != 0.0)) {
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) {
            let assign32140_e40093: f64 = (locals.var_idsatgat_d * locals.var_idmult);
            locals.var_id__blk219 = assign32140_e40093;
        }
        let assign32150_e40102: f64 = if ((locals.var_csrhgatd_i == 0.0) && (locals.var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard634 = assign32150_e40102;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 == 0.0)) {
            let assign32170_e40125: f64 = (locals.var_vbigat_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign32170_e40125;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 == 0.0)) {
            let assign32180_e40141: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign32180_e40142: f64 = (1.0 - assign32180_e40141);
            let assign32180_e40143: f64 = (assign32180_e40142).sqrt();
            let assign32180_e40144: f64 = (1.0 - assign32180_e40143);
            locals.var_wsrhstep = assign32180_e40144;
        }
        let assign32190_e40149: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard635 = assign32190_e40149;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 == 0.0)) && (locals.var_guard635 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 == 0.0)) && (locals.var_guard635 == 0.0)) {
            let assign32210_e40178: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign32210_e40180: f64 = (locals.var_wsrhstep).ln();
            let assign32210_e40181: f64 = (assign32210_e40178 * assign32210_e40180);
            let assign32210_e40184: f64 = (1.0 - locals.var_wsrhstep);
            let assign32210_e40185: f64 = (assign32210_e40181 / assign32210_e40184);
            let assign32210_e40187: f64 = (assign32210_e40185 + locals.var_wsrhstep);
            let assign32210_e40191: f64 = (2.0 * locals.var_pgatd_i);
            let assign32210_e40192: f64 = (1.0 - assign32210_e40191);
            let assign32210_e40193: f64 = (assign32210_e40187 * assign32210_e40192);
            locals.var_dwsrh = assign32210_e40193;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 == 0.0)) {
            let assign32220_e40207: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign32220_e40207;
        }
        let assign32230_e40212: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard636 = assign32230_e40212;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 == 0.0)) && (locals.var_guard636 != 0.0)) {
            let assign32240_e40226: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv_d);
            let assign32240_e40227: f64 = (assign32240_e40226).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32240_e40227, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 == 0.0)) && (locals.var_guard636 == 0.0)) {
            let assign32250_e40244: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv_d);
            let assign32250_e40246: f64 = (assign32250_e40244).powf(locals.var_pgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32250_e40246, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 == 0.0)) {
            let assign32260_e40260: f64 = (locals.var_wdepnulrgat_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign32260_e40260, (locals.var_wdepnulrgat_d * locals.var_tmp_dn5), (locals.var_wdepnulrgat_d * locals.var_tmp_dn6), (locals.var_wdepnulrgat_d * locals.var_tmp_dn7), (locals.var_wdepnulrgat_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 == 0.0)) {
            let assign32270_e40275: f64 = (locals.var_zinv - 1.0);
            let assign32270_e40277: f64 = (assign32270_e40275 * locals.var_wdep);
            let assign32270_e40278: f64 = (locals.var_ftdgat_d * assign32270_e40277);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign32270_e40278, (locals.var_ftdgat_d * (assign32270_e40275 * locals.var_wdep_dn5)), (locals.var_ftdgat_d * (assign32270_e40275 * locals.var_wdep_dn6)), (locals.var_ftdgat_d * (assign32270_e40275 * locals.var_wdep_dn7)), (locals.var_ftdgat_d * (assign32270_e40275 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 == 0.0)) {
            let assign32280_e40293: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign32280_e40294: f64 = (locals.var_csrhgatd_i * assign32280_e40293);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign32280_e40294, (locals.var_csrhgatd_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign32290_e40299: f64 = if locals.var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard637 = assign32290_e40299;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32310_e40323: f64 = (locals.var_wdep * locals.var_one_minus_pgat_d);
            let assign32310_e40325: f64 = (assign32310_e40323 / locals.var_vbi_minus_vjsrh);
            let assign32310_e40326: f64 = (locals.var_btatpartgat_d * assign32310_e40325);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign32310_e40326, (locals.var_btatpartgat_d * ((locals.var_wdep_dn5 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn6 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn7 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn8 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32320_e40340: f64 = (0.666666666666667 * locals.var_atatgat_d);
            let assign32320_e40342: f64 = (assign32320_e40340 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign32320_e40342, (-((assign32320_e40340 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign32320_e40340 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign32320_e40340 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign32320_e40340 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32330_e40356: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign32330_e40356, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32340_e40370: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign32340_e40373: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign32340_e40375: f64 = (assign32340_e40373 + 1.0);
            let assign32340_e40376: f64 = (assign32340_e40370 / assign32340_e40375);
            let assign32340_e40377: f64 = (assign32340_e40376).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign32340_e40377, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign32340_e40375) - (assign32340_e40370 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign32340_e40375 * assign32340_e40375)) / (2.0 * assign32340_e40377)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign32340_e40375) - (assign32340_e40370 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign32340_e40375 * assign32340_e40375)) / (2.0 * assign32340_e40377)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign32340_e40375) - (assign32340_e40370 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign32340_e40375 * assign32340_e40375)) / (2.0 * assign32340_e40377)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign32340_e40375) - (assign32340_e40370 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign32340_e40375 * assign32340_e40375)) / (2.0 * assign32340_e40377)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32350_e40390: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign32350_e40390, (locals.var_umax_dn5 / (2.0 * assign32350_e40390)), (locals.var_umax_dn6 / (2.0 * assign32350_e40390)), (locals.var_umax_dn7 / (2.0 * assign32350_e40390)), (locals.var_umax_dn8 / (2.0 * assign32350_e40390)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32360_e40404: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign32360_e40404, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign32370_e40408: f64 = (-locals.var_pgatd_i);
        let assign32370_e40410: f64 = (assign32370_e40408 * locals.var_one_over_one_minus_pgat_d);
        let assign32370_e40412: f64 = (-1.0);
        let assign32370_e40413: f64 = if assign32370_e40410 == assign32370_e40412 { 1.0 } else { 0.0 };
        locals.var_guard638 = assign32370_e40413;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard638 != 0.0)) {
            let assign32380_e40429: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign32380_e40430: f64 = (1.0 + assign32380_e40429);
            let assign32380_e40431: f64 = (1.0 / assign32380_e40430);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign32380_e40431, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign32380_e40430 * assign32380_e40430))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign32380_e40430 * assign32380_e40430))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign32380_e40430 * assign32380_e40430))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign32380_e40430 * assign32380_e40430))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard638 == 0.0)) {
            let assign32390_e40449: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign32390_e40450: f64 = (1.0 + assign32390_e40449);
            let assign32390_e40452: f64 = (-locals.var_pgatd_i);
            let assign32390_e40454: f64 = (assign32390_e40452 * locals.var_one_over_one_minus_pgat_d);
            let assign32390_e40455: f64 = (assign32390_e40450).powf(assign32390_e40454);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign32390_e40455, if 0.0 == 0.0 && ((assign32390_e40454) as f64).is_finite() && ((assign32390_e40454) as f64).fract() == 0.0 { if assign32390_e40454 == 0.0 { 0.0 } else { (assign32390_e40454 * ((assign32390_e40450).powf(assign32390_e40454 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign32390_e40455 * (assign32390_e40454 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign32390_e40450))) }, if 0.0 == 0.0 && ((assign32390_e40454) as f64).is_finite() && ((assign32390_e40454) as f64).fract() == 0.0 { if assign32390_e40454 == 0.0 { 0.0 } else { (assign32390_e40454 * ((assign32390_e40450).powf(assign32390_e40454 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign32390_e40455 * (assign32390_e40454 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign32390_e40450))) }, if 0.0 == 0.0 && ((assign32390_e40454) as f64).is_finite() && ((assign32390_e40454) as f64).fract() == 0.0 { if assign32390_e40454 == 0.0 { 0.0 } else { (assign32390_e40454 * ((assign32390_e40450).powf(assign32390_e40454 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign32390_e40455 * (assign32390_e40454 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign32390_e40450))) }, if 0.0 == 0.0 && ((assign32390_e40454) as f64).is_finite() && ((assign32390_e40454) as f64).fract() == 0.0 { if assign32390_e40454 == 0.0 { 0.0 } else { (assign32390_e40454 * ((assign32390_e40450).powf(assign32390_e40454 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign32390_e40455 * (assign32390_e40454 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign32390_e40450))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32400_e40469: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign32400_e40472: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign32400_e40473: f64 = (assign32400_e40469 / assign32400_e40472);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign32400_e40473, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign32400_e40472) - (assign32400_e40469 * locals.var_wgamma_dn5)) / (assign32400_e40472 * assign32400_e40472)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign32400_e40472) - (assign32400_e40469 * locals.var_wgamma_dn6)) / (assign32400_e40472 * assign32400_e40472)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign32400_e40472) - (assign32400_e40469 * locals.var_wgamma_dn7)) / (assign32400_e40472 * assign32400_e40472)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign32400_e40472) - (assign32400_e40469 * locals.var_wgamma_dn8)) / (assign32400_e40472 * assign32400_e40472)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32410_e40488: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign32410_e40489: f64 = (0.375 * assign32410_e40488);
            let assign32410_e40490: f64 = (assign32410_e40489).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign32410_e40490, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign32410_e40490)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign32410_e40490)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign32410_e40490)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign32410_e40490)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32420_e40505: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign32420_e40506: f64 = (2.0 * assign32420_e40505);
            let assign32420_e40508: f64 = (assign32420_e40506 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign32420_e40508, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32430_e40522: f64 = (locals.var_atatgat_d * locals.var_twoatatoverthreebtat);
            let assign32430_e40524: f64 = (assign32430_e40522 * locals.var_sqrtumax);
            let assign32430_e40527: f64 = (locals.var_atatgat_d * locals.var_umax);
            let assign32430_e40528: f64 = (assign32430_e40524 - assign32430_e40527);
            let assign32430_e40532: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign32430_e40533: f64 = (0.5 * assign32430_e40532);
            let assign32430_e40534: f64 = (assign32430_e40528 + assign32430_e40533);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign32430_e40534, (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign32430_e40522 * locals.var_sqrtumax_dn5)) - (locals.var_atatgat_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign32430_e40522 * locals.var_sqrtumax_dn6)) - (locals.var_atatgat_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign32430_e40522 * locals.var_sqrtumax_dn7)) - (locals.var_atatgat_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign32430_e40522 * locals.var_sqrtumax_dn8)) - (locals.var_atatgat_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32440_e40548: f64 = (locals.var_ltat - 1.0);
            let assign32440_e40550: f64 = (assign32440_e40548 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign32440_e40550, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign32440_e40548 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign32440_e40548 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign32440_e40548 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign32440_e40548 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32450_e40564: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign32450_e40564, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign32460_e40569: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard639 = assign32460_e40569;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard639 != 0.0)) {
            let assign32470_e40585: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign32470_e40586: f64 = (1.0 + assign32470_e40585);
            let assign32470_e40587: f64 = (1.0 / assign32470_e40586);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign32470_e40587, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign32470_e40586 * assign32470_e40586))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign32470_e40586 * assign32470_e40586))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign32470_e40586 * assign32470_e40586))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign32470_e40586 * assign32470_e40586))), );
        }
    }
    pub(super) fn stamp_transient_block_38(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard639 == 0.0)) {
            let assign32480_e40606: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign32480_e40607: f64 = (1.0 - assign32480_e40606);
            let assign32480_e40608: f64 = (1.0 / assign32480_e40607);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign32480_e40608, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign32480_e40607 * assign32480_e40607))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign32480_e40607 * assign32480_e40607))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign32480_e40607 * assign32480_e40607))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign32480_e40607 * assign32480_e40607))), );
        }
        let assign32490_e40612: f64 = (-locals.var_ysq);
        let assign32490_e40614: f64 = (assign32490_e40612 + locals.var_mtat);
        let assign32490_e40616: f64 = (-230.25850929940458);
        let assign32490_e40617: f64 = if assign32490_e40614 > assign32490_e40616 { 1.0 } else { 0.0 };
        locals.var_guard640 = assign32490_e40617;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard640 != 0.0)) {
            let assign32500_e40630: f64 = (-locals.var_ysq);
            let assign32500_e40632: f64 = (assign32500_e40630 + locals.var_mtat);
            let assign32500_e40633: f64 = (assign32500_e40632).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32500_e40633, (assign32500_e40633 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign32500_e40633 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign32500_e40633 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign32500_e40633 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard640 == 0.0)) {
            let assign32510_e40651: f64 = (-230.25850929940458);
            let assign32510_e40653: f64 = (-locals.var_ysq);
            let assign32510_e40655: f64 = (assign32510_e40653 + locals.var_mtat);
            let assign32510_e40656: f64 = (assign32510_e40651 - assign32510_e40655);
            let assign32510_e40660: f64 = (-230.25850929940458);
            let assign32510_e40662: f64 = (-locals.var_ysq);
            let assign32510_e40664: f64 = (assign32510_e40662 + locals.var_mtat);
            let assign32510_e40665: f64 = (assign32510_e40660 - assign32510_e40664);
            let assign32510_e40668: f64 = (-230.25850929940458);
            let assign32510_e40670: f64 = (-locals.var_ysq);
            let assign32510_e40672: f64 = (assign32510_e40670 + locals.var_mtat);
            let assign32510_e40673: f64 = (assign32510_e40668 - assign32510_e40672);
            let assign32510_e40675: f64 = (assign32510_e40673 * 0.3333333333333333);
            let assign32510_e40676: f64 = (1.0 + assign32510_e40675);
            let assign32510_e40677: f64 = (assign32510_e40665 * assign32510_e40676);
            let assign32510_e40678: f64 = (0.5 * assign32510_e40677);
            let assign32510_e40679: f64 = (1.0 + assign32510_e40678);
            let assign32510_e40680: f64 = (assign32510_e40656 * assign32510_e40679);
            let assign32510_e40681: f64 = (1.0 + assign32510_e40680);
            let assign32510_e40682: f64 = (1e-100 / assign32510_e40681);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32510_e40682, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign32510_e40679) + (assign32510_e40656 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign32510_e40676) + (assign32510_e40665 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign32510_e40681 * assign32510_e40681))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign32510_e40679) + (assign32510_e40656 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign32510_e40676) + (assign32510_e40665 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign32510_e40681 * assign32510_e40681))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign32510_e40679) + (assign32510_e40656 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign32510_e40676) + (assign32510_e40665 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign32510_e40681 * assign32510_e40681))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign32510_e40679) + (assign32510_e40656 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign32510_e40676) + (assign32510_e40665 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign32510_e40681 * assign32510_e40681))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32520_e40696: f64 = (0.29214664 * locals.var_terfc);
            let assign32520_e40700: f64 = (locals.var_terfc * locals.var_terfc);
            let assign32520_e40701: f64 = (locals.var_berfc * assign32520_e40700);
            let assign32520_e40702: f64 = (assign32520_e40696 + assign32520_e40701);
            let assign32520_e40706: f64 = (locals.var_terfc * locals.var_terfc);
            let assign32520_e40708: f64 = (assign32520_e40706 * locals.var_terfc);
            let assign32520_e40709: f64 = (locals.var_cerfc * assign32520_e40708);
            let assign32520_e40710: f64 = (assign32520_e40702 + assign32520_e40709);
            let assign32520_e40712: f64 = (assign32520_e40710 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign32520_e40712, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign32520_e40706 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign32520_e40710 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign32520_e40706 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign32520_e40710 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign32520_e40706 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign32520_e40710 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign32520_e40706 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign32520_e40710 * locals.var_tmp_dn8)), );
        }
        let assign32530_e40717: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard641 = assign32530_e40717;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard641 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign32550_e40734: f64 = (-230.25850929940458);
        let assign32550_e40735: f64 = if locals.var_mtat > assign32550_e40734 { 1.0 } else { 0.0 };
        locals.var_guard642 = assign32550_e40735;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard641 == 0.0)) && (locals.var_guard642 != 0.0)) {
            let assign32560_e40751: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32560_e40751, (assign32560_e40751 * locals.var_mtat_dn5), (assign32560_e40751 * locals.var_mtat_dn6), (assign32560_e40751 * locals.var_mtat_dn7), (assign32560_e40751 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard641 == 0.0)) && (locals.var_guard642 == 0.0)) {
            let assign32570_e40772: f64 = (-230.25850929940458);
            let assign32570_e40774: f64 = (assign32570_e40772 - locals.var_mtat);
            let assign32570_e40778: f64 = (-230.25850929940458);
            let assign32570_e40780: f64 = (assign32570_e40778 - locals.var_mtat);
            let assign32570_e40783: f64 = (-230.25850929940458);
            let assign32570_e40785: f64 = (assign32570_e40783 - locals.var_mtat);
            let assign32570_e40787: f64 = (assign32570_e40785 * 0.3333333333333333);
            let assign32570_e40788: f64 = (1.0 + assign32570_e40787);
            let assign32570_e40789: f64 = (assign32570_e40780 * assign32570_e40788);
            let assign32570_e40790: f64 = (0.5 * assign32570_e40789);
            let assign32570_e40791: f64 = (1.0 + assign32570_e40790);
            let assign32570_e40792: f64 = (assign32570_e40774 * assign32570_e40791);
            let assign32570_e40793: f64 = (1.0 + assign32570_e40792);
            let assign32570_e40794: f64 = (1e-100 / assign32570_e40793);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32570_e40794, (-((1e-100 * (((-locals.var_mtat_dn5) * assign32570_e40791) + (assign32570_e40774 * (0.5 * (((-locals.var_mtat_dn5) * assign32570_e40788) + (assign32570_e40780 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign32570_e40793 * assign32570_e40793))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign32570_e40791) + (assign32570_e40774 * (0.5 * (((-locals.var_mtat_dn6) * assign32570_e40788) + (assign32570_e40780 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign32570_e40793 * assign32570_e40793))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign32570_e40791) + (assign32570_e40774 * (0.5 * (((-locals.var_mtat_dn7) * assign32570_e40788) + (assign32570_e40780 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign32570_e40793 * assign32570_e40793))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign32570_e40791) + (assign32570_e40774 * (0.5 * (((-locals.var_mtat_dn8) * assign32570_e40788) + (assign32570_e40780 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign32570_e40793 * assign32570_e40793))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard641 == 0.0)) {
            let assign32580_e40811: f64 = (2.0 * locals.var_tmp);
            let assign32580_e40813: f64 = (assign32580_e40811 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign32580_e40813, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32590_e40827: f64 = (1.772453850905516 * 0.5);
            let assign32590_e40830: f64 = (locals.var_atatgat_d * locals.var_erfctimesexpmtat);
            let assign32590_e40832: f64 = (assign32590_e40830 / locals.var_ktat);
            let assign32590_e40833: f64 = (assign32590_e40827 * assign32590_e40832);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign32590_e40833, (assign32590_e40827 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign32590_e40830 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign32590_e40827 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign32590_e40830 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign32590_e40827 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign32590_e40830 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign32590_e40827 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign32590_e40830 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard637 == 0.0)) {
            let assign32600_e40848: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign32600_e40850: f64 = (assign32600_e40848 * locals.var_wtat);
            let assign32600_e40851: f64 = (locals.var_ctatgatd_i * assign32600_e40850);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign32600_e40851, (locals.var_ctatgatd_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign32600_e40848 * locals.var_wtat_dn5))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign32600_e40848 * locals.var_wtat_dn6))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign32600_e40848 * locals.var_wtat_dn7))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign32600_e40848 * locals.var_wtat_dn8))), );
        }
        let assign32610_e40856: f64 = if locals.var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard643 = assign32610_e40856;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard643 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign32630_e40870: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard644 = assign32630_e40870;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard643 == 0.0)) && (locals.var_guard644 != 0.0)) {
            let assign32640_e40884: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign32640_e40886: f64 = (assign32640_e40884 * locals.var_vbirgatinv_d);
            let assign32640_e40887: f64 = (assign32640_e40886).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32640_e40887, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard643 == 0.0)) && (locals.var_guard644 == 0.0)) {
            let assign32650_e40904: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign32650_e40906: f64 = (assign32650_e40904 * locals.var_vbirgatinv_d);
            let assign32650_e40908: f64 = (assign32650_e40906).powf(locals.var_pgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32650_e40908, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard643 == 0.0)) {
            let assign32660_e40923: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign32660_e40925: f64 = (assign32660_e40923 * locals.var_wdepnulrinvgat_d);
            let assign32660_e40927: f64 = (assign32660_e40925 / locals.var_tmp);
            let assign32660_e40928: f64 = (locals.var_one_over_one_minus_pgat_d * assign32660_e40927);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign32660_e40928, (locals.var_one_over_one_minus_pgat_d * (-((assign32660_e40925 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat_d * (-((assign32660_e40925 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat_d * (-((assign32660_e40925 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat_d * (-((assign32660_e40925 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign32670_e40932: f64 = (-locals.var_fbbtgat_d);
        let assign32670_e40934: f64 = (assign32670_e40932 / locals.var_fmaxr);
        let assign32670_e40935: f64 = (assign32670_e40934).abs();
        let assign32670_e40937: f64 = if assign32670_e40935 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard645 = assign32670_e40937;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard643 == 0.0)) && (locals.var_guard645 != 0.0)) {
            let assign32680_e40950: f64 = (-locals.var_fbbtgat_d);
            let assign32680_e40952: f64 = (assign32680_e40950 / locals.var_fmaxr);
            let assign32680_e40953: f64 = (assign32680_e40952).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32680_e40953, (assign32680_e40953 * ((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign32680_e40950 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign32680_e40953 * ((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign32680_e40950 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign32680_e40953 * ((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign32680_e40950 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign32680_e40953 * ((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign32680_e40950 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))), );
        }
        let assign32690_e40957: f64 = (-locals.var_fbbtgat_d);
        let assign32690_e40959: f64 = (assign32690_e40957 / locals.var_fmaxr);
        let assign32690_e40961: f64 = if assign32690_e40959 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard646 = assign32690_e40961;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard643 == 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard646 != 0.0)) {
            let assign32700_e40979: f64 = (-230.25850929940458);
            let assign32700_e40981: f64 = (-locals.var_fbbtgat_d);
            let assign32700_e40983: f64 = (assign32700_e40981 / locals.var_fmaxr);
            let assign32700_e40984: f64 = (assign32700_e40979 - assign32700_e40983);
            let assign32700_e40988: f64 = (-230.25850929940458);
            let assign32700_e40990: f64 = (-locals.var_fbbtgat_d);
            let assign32700_e40992: f64 = (assign32700_e40990 / locals.var_fmaxr);
            let assign32700_e40993: f64 = (assign32700_e40988 - assign32700_e40992);
            let assign32700_e40996: f64 = (-230.25850929940458);
            let assign32700_e40998: f64 = (-locals.var_fbbtgat_d);
            let assign32700_e41000: f64 = (assign32700_e40998 / locals.var_fmaxr);
            let assign32700_e41001: f64 = (assign32700_e40996 - assign32700_e41000);
            let assign32700_e41003: f64 = (assign32700_e41001 * 0.3333333333333333);
            let assign32700_e41004: f64 = (1.0 + assign32700_e41003);
            let assign32700_e41005: f64 = (assign32700_e40993 * assign32700_e41004);
            let assign32700_e41006: f64 = (0.5 * assign32700_e41005);
            let assign32700_e41007: f64 = (1.0 + assign32700_e41006);
            let assign32700_e41008: f64 = (assign32700_e40984 * assign32700_e41007);
            let assign32700_e41009: f64 = (1.0 + assign32700_e41008);
            let assign32700_e41010: f64 = (1e-100 / assign32700_e41009);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32700_e41010, (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign32700_e40981 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32700_e41007) + (assign32700_e40984 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign32700_e40990 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32700_e41004) + (assign32700_e40993 * ((-((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign32700_e40998 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign32700_e41009 * assign32700_e41009))), (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign32700_e40981 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32700_e41007) + (assign32700_e40984 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign32700_e40990 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32700_e41004) + (assign32700_e40993 * ((-((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign32700_e40998 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign32700_e41009 * assign32700_e41009))), (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign32700_e40981 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32700_e41007) + (assign32700_e40984 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign32700_e40990 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32700_e41004) + (assign32700_e40993 * ((-((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign32700_e40998 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign32700_e41009 * assign32700_e41009))), (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign32700_e40981 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32700_e41007) + (assign32700_e40984 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign32700_e40990 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign32700_e41004) + (assign32700_e40993 * ((-((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign32700_e40998 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign32700_e41009 * assign32700_e41009))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard643 == 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard646 == 0.0)) {
            let assign32710_e41031: f64 = (-locals.var_fbbtgat_d);
            let assign32710_e41033: f64 = (assign32710_e41031 / locals.var_fmaxr);
            let assign32710_e41035: f64 = (assign32710_e41033 - 230.25850929940458);
            let assign32710_e41039: f64 = (-locals.var_fbbtgat_d);
            let assign32710_e41041: f64 = (assign32710_e41039 / locals.var_fmaxr);
            let assign32710_e41043: f64 = (assign32710_e41041 - 230.25850929940458);
            let assign32710_e41046: f64 = (-locals.var_fbbtgat_d);
            let assign32710_e41048: f64 = (assign32710_e41046 / locals.var_fmaxr);
            let assign32710_e41050: f64 = (assign32710_e41048 - 230.25850929940458);
            let assign32710_e41052: f64 = (assign32710_e41050 * 0.3333333333333333);
            let assign32710_e41053: f64 = (1.0 + assign32710_e41052);
            let assign32710_e41054: f64 = (assign32710_e41043 * assign32710_e41053);
            let assign32710_e41055: f64 = (0.5 * assign32710_e41054);
            let assign32710_e41056: f64 = (1.0 + assign32710_e41055);
            let assign32710_e41057: f64 = (assign32710_e41035 * assign32710_e41056);
            let assign32710_e41058: f64 = (1.0 + assign32710_e41057);
            let assign32710_e41059: f64 = (1e100 * assign32710_e41058);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32710_e41059, (1e100 * ((((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign32710_e41031 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign32710_e41056) + (assign32710_e41035 * (0.5 * ((((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign32710_e41039 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign32710_e41053) + (assign32710_e41043 * (((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign32710_e41046 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign32710_e41031 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign32710_e41056) + (assign32710_e41035 * (0.5 * ((((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign32710_e41039 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign32710_e41053) + (assign32710_e41043 * (((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign32710_e41046 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign32710_e41031 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign32710_e41056) + (assign32710_e41035 * (0.5 * ((((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign32710_e41039 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign32710_e41053) + (assign32710_e41043 * (((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign32710_e41046 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign32710_e41031 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign32710_e41056) + (assign32710_e41035 * (0.5 * ((((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign32710_e41039 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign32710_e41053) + (assign32710_e41043 * (((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign32710_e41046 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard643 == 0.0)) {
            let assign32720_e41074: f64 = (locals.var_v2 * locals.var_fmaxr);
            let assign32720_e41076: f64 = (assign32720_e41074 * locals.var_fmaxr);
            let assign32720_e41078: f64 = (assign32720_e41076 * locals.var_tmp);
            let assign32720_e41079: f64 = (locals.var_cbbtgatd_i * assign32720_e41078);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign32720_e41079, (locals.var_cbbtgatd_i * (((((locals.var_v2 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign32720_e41074 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign32720_e41076 * locals.var_tmp_dn5))), (locals.var_cbbtgatd_i * (((((locals.var_v2 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign32720_e41074 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign32720_e41076 * locals.var_tmp_dn6))), (locals.var_cbbtgatd_i * (((((locals.var_v2 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign32720_e41074 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign32720_e41076 * locals.var_tmp_dn7))), (locals.var_cbbtgatd_i * (((((locals.var_v2 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign32720_e41074 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign32720_e41076 * locals.var_tmp_dn8))), );
        }
        let assign32730_e41084: f64 = if locals.var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard647 = assign32730_e41084;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard647 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign32750_e41098: f64 = (-locals.var_alphaav);
        let assign32750_e41100: f64 = (assign32750_e41098 * locals.var_vbrgatd_i);
        let assign32750_e41101: f64 = if locals.var_vav > assign32750_e41100 { 1.0 } else { 0.0 };
        locals.var_guard648 = assign32750_e41101;
        let assign32760_e41104: f64 = if locals.var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard649 = assign32760_e41104;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard648 != 0.0)) && (locals.var_guard649 != 0.0)) {
            let assign32770_e41120: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign32770_e41123: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign32770_e41124: f64 = (assign32770_e41120 * assign32770_e41123);
            let assign32770_e41127: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign32770_e41128: f64 = (assign32770_e41124 * assign32770_e41127);
            let assign32770_e41131: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign32770_e41132: f64 = (assign32770_e41128 * assign32770_e41131);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32770_e41132, (((((((locals.var_vav * locals.var_vbrinvgat_d_dn5) * assign32770_e41123) + (assign32770_e41120 * (locals.var_vav * locals.var_vbrinvgat_d_dn5))) * assign32770_e41127) + (assign32770_e41124 * (locals.var_vav * locals.var_vbrinvgat_d_dn5))) * assign32770_e41131) + (assign32770_e41128 * (locals.var_vav * locals.var_vbrinvgat_d_dn5))), (((((((locals.var_vav * locals.var_vbrinvgat_d_dn6) * assign32770_e41123) + (assign32770_e41120 * (locals.var_vav * locals.var_vbrinvgat_d_dn6))) * assign32770_e41127) + (assign32770_e41124 * (locals.var_vav * locals.var_vbrinvgat_d_dn6))) * assign32770_e41131) + (assign32770_e41128 * (locals.var_vav * locals.var_vbrinvgat_d_dn6))), (((((((locals.var_vav * locals.var_vbrinvgat_d_dn7) * assign32770_e41123) + (assign32770_e41120 * (locals.var_vav * locals.var_vbrinvgat_d_dn7))) * assign32770_e41127) + (assign32770_e41124 * (locals.var_vav * locals.var_vbrinvgat_d_dn7))) * assign32770_e41131) + (assign32770_e41128 * (locals.var_vav * locals.var_vbrinvgat_d_dn7))), (((((((locals.var_vav * locals.var_vbrinvgat_d_dn8) * assign32770_e41123) + (assign32770_e41120 * (locals.var_vav * locals.var_vbrinvgat_d_dn8))) * assign32770_e41127) + (assign32770_e41124 * (locals.var_vav * locals.var_vbrinvgat_d_dn8))) * assign32770_e41131) + (assign32770_e41128 * (locals.var_vav * locals.var_vbrinvgat_d_dn8))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard648 != 0.0)) && (locals.var_guard649 == 0.0)) {
            let assign32780_e41151: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign32780_e41152: f64 = (assign32780_e41151).abs();
            let assign32780_e41154: f64 = (assign32780_e41152).powf(locals.var_pbrgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign32780_e41154, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign32780_e41152).powf(locals.var_pbrgatd_i - 1.0) * if assign32780_e41151 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn5)) })) } } else { (assign32780_e41154 * (locals.var_pbrgatd_i * (if assign32780_e41151 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn5)) } / assign32780_e41152))) }, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign32780_e41152).powf(locals.var_pbrgatd_i - 1.0) * if assign32780_e41151 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn6)) })) } } else { (assign32780_e41154 * (locals.var_pbrgatd_i * (if assign32780_e41151 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn6)) } / assign32780_e41152))) }, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign32780_e41152).powf(locals.var_pbrgatd_i - 1.0) * if assign32780_e41151 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn7)) })) } } else { (assign32780_e41154 * (locals.var_pbrgatd_i * (if assign32780_e41151 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn7)) } / assign32780_e41152))) }, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign32780_e41152).powf(locals.var_pbrgatd_i - 1.0) * if assign32780_e41151 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn8)) })) } } else { (assign32780_e41154 * (locals.var_pbrgatd_i * (if assign32780_e41151 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn8)) } / assign32780_e41152))) }, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard648 != 0.0)) {
            let assign32790_e41171: f64 = (1.0 - locals.var_tmp);
            let assign32790_e41172: f64 = (1.0 / assign32790_e41171);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign32790_e41172, (-((-locals.var_tmp_dn5) / (assign32790_e41171 * assign32790_e41171))), (-((-locals.var_tmp_dn6) / (assign32790_e41171 * assign32790_e41171))), (-((-locals.var_tmp_dn7) / (assign32790_e41171 * assign32790_e41171))), (-((-locals.var_tmp_dn8) / (assign32790_e41171 * assign32790_e41171))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard648 == 0.0)) {
            let assign32800_e41191: f64 = (locals.var_alphaav * locals.var_vbrgatd_i);
            let assign32800_e41192: f64 = (locals.var_vav + assign32800_e41191);
            let assign32800_e41194: f64 = (assign32800_e41192 * locals.var_slopegat_d);
            let assign32800_e41195: f64 = (locals.var_fstopgat_d + assign32800_e41194);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign32800_e41195, (assign32800_e41192 * locals.var_slopegat_d_dn5), (assign32800_e41192 * locals.var_slopegat_d_dn6), (assign32800_e41192 * locals.var_slopegat_d_dn7), (assign32800_e41192 * locals.var_slopegat_d_dn8), );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard633 == 0.0)) {
            let assign32810_e41207: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign32810_e41209: f64 = (assign32810_e41207 + locals.var_itat);
            let assign32810_e41211: f64 = (assign32810_e41209 + locals.var_ibbt);
            let assign32810_e41212: f64 = (p.p29 * assign32810_e41211);
            let assign32810_e41214: f64 = (assign32810_e41212 * locals.var_fbreakdown);
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (assign32810_e41214, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign32810_e41212 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign32810_e41212 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign32810_e41212 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign32810_e41212 * locals.var_fbreakdown_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign32820_e41222: f64 = (locals.var_abdrain_i * locals.var_ijunbot);
            let assign32820_e41225: f64 = (locals.var_lsdrain_i * locals.var_ijunsti);
            let assign32820_e41226: f64 = (assign32820_e41222 + assign32820_e41225);
            let assign32820_e41229: f64 = (locals.var_lgdrain_i * locals.var_ijungat);
            let assign32820_e41230: f64 = (assign32820_e41226 + assign32820_e41229);
            (locals.var_i2, locals.var_i2_dn5, locals.var_i2_dn6, locals.var_i2_dn7, locals.var_i2_dn8, ) = (assign32820_e41230, (((locals.var_abdrain_i * locals.var_ijunbot_dn5) + (locals.var_lsdrain_i * locals.var_ijunsti_dn5)) + (locals.var_lgdrain_i * locals.var_ijungat_dn5)), (((locals.var_abdrain_i * locals.var_ijunbot_dn6) + (locals.var_lsdrain_i * locals.var_ijunsti_dn6)) + (locals.var_lgdrain_i * locals.var_ijungat_dn6)), (((locals.var_abdrain_i * locals.var_ijunbot_dn7) + (locals.var_lsdrain_i * locals.var_ijunsti_dn7)) + (locals.var_lgdrain_i * locals.var_ijungat_dn7)), (((locals.var_abdrain_i * locals.var_ijunbot_dn8) + (locals.var_lsdrain_i * locals.var_ijunsti_dn8)) + (locals.var_lgdrain_i * locals.var_ijungat_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            locals.var_vbbt = 0.0;
            locals.var_two_psistar = 0.0;
        }
        let assign32850_e41256: f64 = if (!(((locals.var_abdrain_i == 0.0) && (locals.var_lsdrain_i == 0.0)) && (locals.var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard650 = assign32850_e41256;
        let assign32930_e41342: f64 = if locals.var_v3 < locals.var_vmax_d { 1.0 } else { 0.0 };
        locals.var_guard651 = assign32930_e41342;
        let assign32940_e41344: f64 = (-0.5);
        let assign32940_e41347: f64 = (locals.var_v3 * locals.var_phitdinv);
        let assign32940_e41348: f64 = (assign32940_e41344 * assign32940_e41347);
        let assign32940_e41349: f64 = (assign32940_e41348).abs();
        let assign32940_e41351: f64 = if assign32940_e41349 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard652 = assign32940_e41351;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 != 0.0)) {
            let assign32950_e41362: f64 = (-0.5);
            let assign32950_e41365: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign32950_e41366: f64 = (assign32950_e41362 * assign32950_e41365);
            let assign32950_e41367: f64 = (assign32950_e41366).exp();
            locals.var_z = assign32950_e41367;
        }
        let assign32960_e41371: f64 = (-0.5);
        let assign32960_e41374: f64 = (locals.var_v3 * locals.var_phitdinv);
        let assign32960_e41375: f64 = (assign32960_e41371 * assign32960_e41374);
        let assign32960_e41377: f64 = if assign32960_e41375 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard653 = assign32960_e41377;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard653 != 0.0)) {
            let assign32970_e41393: f64 = (-230.25850929940458);
            let assign32970_e41395: f64 = (-0.5);
            let assign32970_e41398: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign32970_e41399: f64 = (assign32970_e41395 * assign32970_e41398);
            let assign32970_e41400: f64 = (assign32970_e41393 - assign32970_e41399);
            let assign32970_e41404: f64 = (-230.25850929940458);
            let assign32970_e41406: f64 = (-0.5);
            let assign32970_e41409: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign32970_e41410: f64 = (assign32970_e41406 * assign32970_e41409);
            let assign32970_e41411: f64 = (assign32970_e41404 - assign32970_e41410);
            let assign32970_e41414: f64 = (-230.25850929940458);
            let assign32970_e41416: f64 = (-0.5);
            let assign32970_e41419: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign32970_e41420: f64 = (assign32970_e41416 * assign32970_e41419);
            let assign32970_e41421: f64 = (assign32970_e41414 - assign32970_e41420);
            let assign32970_e41423: f64 = (assign32970_e41421 * 0.3333333333333333);
            let assign32970_e41424: f64 = (1.0 + assign32970_e41423);
            let assign32970_e41425: f64 = (assign32970_e41411 * assign32970_e41424);
            let assign32970_e41426: f64 = (0.5 * assign32970_e41425);
            let assign32970_e41427: f64 = (1.0 + assign32970_e41426);
            let assign32970_e41428: f64 = (assign32970_e41400 * assign32970_e41427);
            let assign32970_e41429: f64 = (1.0 + assign32970_e41428);
            let assign32970_e41430: f64 = (1e-100 / assign32970_e41429);
            locals.var_z = assign32970_e41430;
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard653 == 0.0)) {
            let assign32980_e41449: f64 = (-0.5);
            let assign32980_e41452: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign32980_e41453: f64 = (assign32980_e41449 * assign32980_e41452);
            let assign32980_e41455: f64 = (assign32980_e41453 - 230.25850929940458);
            let assign32980_e41459: f64 = (-0.5);
            let assign32980_e41462: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign32980_e41463: f64 = (assign32980_e41459 * assign32980_e41462);
            let assign32980_e41465: f64 = (assign32980_e41463 - 230.25850929940458);
            let assign32980_e41468: f64 = (-0.5);
            let assign32980_e41471: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign32980_e41472: f64 = (assign32980_e41468 * assign32980_e41471);
            let assign32980_e41474: f64 = (assign32980_e41472 - 230.25850929940458);
            let assign32980_e41476: f64 = (assign32980_e41474 * 0.3333333333333333);
            let assign32980_e41477: f64 = (1.0 + assign32980_e41476);
            let assign32980_e41478: f64 = (assign32980_e41465 * assign32980_e41477);
            let assign32980_e41479: f64 = (0.5 * assign32980_e41478);
            let assign32980_e41480: f64 = (1.0 + assign32980_e41479);
            let assign32980_e41481: f64 = (assign32980_e41455 * assign32980_e41480);
            let assign32980_e41482: f64 = (1.0 + assign32980_e41481);
            let assign32980_e41483: f64 = (1e100 * assign32980_e41482);
            locals.var_z = assign32980_e41483;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 != 0.0)) {
            let assign32990_e41495: f64 = (1.0 / locals.var_z);
            locals.var_zinv = assign32990_e41495;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 != 0.0)) {
            let assign33000_e41507: f64 = (locals.var_zinv * locals.var_zinv);
            locals.var_idmult = assign33000_e41507;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 == 0.0)) {
            let assign33010_e41521: f64 = (locals.var_v3 - locals.var_vmax_d);
            let assign33010_e41523: f64 = (assign33010_e41521 * locals.var_phitdinv);
            let assign33010_e41524: f64 = (1.0 + assign33010_e41523);
            let assign33010_e41526: f64 = (assign33010_e41524 * locals.var_exp_vmax_over_phitd_d);
            locals.var_idmult = assign33010_e41526;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 == 0.0)) {
            let assign33020_e41538: f64 = (locals.var_idmult).sqrt();
            locals.var_zinv = assign33020_e41538;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 == 0.0)) {
            let assign33030_e41551: f64 = (1.0 / locals.var_zinv);
            locals.var_z = assign33030_e41551;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) {
            let assign33040_e41561: f64 = (locals.var_idmult - 1.0);
            locals.var_idmult = assign33040_e41561;
        }
        let assign33050_e41566: f64 = if locals.var_v3 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard654 = assign33050_e41566;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard654 != 0.0)) {
            let assign33060_e41578: f64 = (2.0 + locals.var_z);
            let assign33060_e41581: f64 = (locals.var_z + 1.0);
            let assign33060_e41584: f64 = (locals.var_z + 3.0);
            let assign33060_e41585: f64 = (assign33060_e41581 * assign33060_e41584);
            let assign33060_e41586: f64 = (assign33060_e41585).sqrt();
            let assign33060_e41587: f64 = (assign33060_e41578 + assign33060_e41586);
            let assign33060_e41588: f64 = (assign33060_e41587).ln();
            let assign33060_e41589: f64 = (locals.var_phitd * assign33060_e41588);
            let assign33060_e41590: f64 = (2.0 * assign33060_e41589);
            locals.var_two_psistar = assign33060_e41590;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard654 == 0.0)) {
            let assign33070_e41602: f64 = (-locals.var_v3);
            let assign33070_e41607: f64 = (2.0 * locals.var_zinv);
            let assign33070_e41609: f64 = (assign33070_e41607 + 1.0);
            let assign33070_e41612: f64 = (1.0 + locals.var_zinv);
            let assign33070_e41616: f64 = (3.0 * locals.var_zinv);
            let assign33070_e41617: f64 = (1.0 + assign33070_e41616);
            let assign33070_e41618: f64 = (assign33070_e41612 * assign33070_e41617);
            let assign33070_e41619: f64 = (assign33070_e41618).sqrt();
            let assign33070_e41620: f64 = (assign33070_e41609 + assign33070_e41619);
            let assign33070_e41621: f64 = (assign33070_e41620).ln();
            let assign33070_e41622: f64 = (locals.var_phitd * assign33070_e41621);
            let assign33070_e41623: f64 = (2.0 * assign33070_e41622);
            let assign33070_e41624: f64 = (assign33070_e41602 + assign33070_e41623);
            locals.var_two_psistar = assign33070_e41624;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) {
            let assign33080_e41634: f64 = (locals.var_vbimin_d - locals.var_two_psistar);
            locals.var_vjlim = assign33080_e41634;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) {
            let assign33090_e41645: f64 = (locals.var_v3 + locals.var_vjlim);
            let assign33090_e41648: f64 = (locals.var_v3 - locals.var_vjlim);
            let assign33090_e41651: f64 = (locals.var_v3 - locals.var_vjlim);
            let assign33090_e41652: f64 = (assign33090_e41648 * assign33090_e41651);
            let assign33090_e41655: f64 = (4.0 * locals.var_phitd);
            let assign33090_e41657: f64 = (assign33090_e41655 * locals.var_phitd);
            let assign33090_e41658: f64 = (assign33090_e41652 + assign33090_e41657);
            let assign33090_e41659: f64 = (assign33090_e41658).sqrt();
            let assign33090_e41660: f64 = (assign33090_e41645 - assign33090_e41659);
            let assign33090_e41661: f64 = (0.5 * assign33090_e41660);
            locals.var_vjsrh = assign33090_e41661;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) {
            let assign33100_e41672: f64 = (locals.var_v3 + locals.var_vbbtlim_d);
            let assign33100_e41675: f64 = (locals.var_v3 - locals.var_vbbtlim_d);
            let assign33100_e41678: f64 = (locals.var_v3 - locals.var_vbbtlim_d);
            let assign33100_e41679: f64 = (assign33100_e41675 * assign33100_e41678);
            let assign33100_e41682: f64 = (4.0 * locals.var_phitr);
            let assign33100_e41684: f64 = (assign33100_e41682 * locals.var_phitr);
            let assign33100_e41685: f64 = (assign33100_e41679 + assign33100_e41684);
            let assign33100_e41686: f64 = (assign33100_e41685).sqrt();
            let assign33100_e41687: f64 = (assign33100_e41672 - assign33100_e41686);
            let assign33100_e41688: f64 = (0.5 * assign33100_e41687);
            locals.var_vbbt = assign33100_e41688;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard650 != 0.0)) {
            let assign33110_e41699: f64 = locals.var_v3;
            let assign33110_e41702: f64 = locals.var_v3;
            let assign33110_e41705: f64 = locals.var_v3;
            let assign33110_e41706: f64 = (assign33110_e41702 * assign33110_e41705);
            let assign33110_e41709: f64 = (4.0 * 1e-6);
            let assign33110_e41711: f64 = (assign33110_e41709 * 1e-6);
            let assign33110_e41712: f64 = (assign33110_e41706 + assign33110_e41711);
            let assign33110_e41713: f64 = (assign33110_e41712).sqrt();
            let assign33110_e41714: f64 = (assign33110_e41699 - assign33110_e41713);
            let assign33110_e41715: f64 = (0.5 * assign33110_e41714);
            locals.var_vav = assign33110_e41715;
        }
        let assign33120_e41720: f64 = if locals.var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard655 = assign33120_e41720;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 != 0.0)) {
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) {
            let assign33140_e41737: f64 = (locals.var_idsatbot_d * locals.var_idmult);
            locals.var_id__blk219 = assign33140_e41737;
        }
        let assign33150_e41746: f64 = if ((locals.var_csrhbotd_i == 0.0) && (locals.var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard656 = assign33150_e41746;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 == 0.0)) {
            let assign33170_e41769: f64 = (locals.var_vbibot_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign33170_e41769;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 == 0.0)) {
            let assign33180_e41785: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign33180_e41786: f64 = (1.0 - assign33180_e41785);
            let assign33180_e41787: f64 = (assign33180_e41786).sqrt();
            let assign33180_e41788: f64 = (1.0 - assign33180_e41787);
            locals.var_wsrhstep = assign33180_e41788;
        }
    }
    pub(super) fn stamp_transient_block_39(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign33190_e41793: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard657 = assign33190_e41793;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 == 0.0)) {
            let assign33210_e41822: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign33210_e41824: f64 = (locals.var_wsrhstep).ln();
            let assign33210_e41825: f64 = (assign33210_e41822 * assign33210_e41824);
            let assign33210_e41828: f64 = (1.0 - locals.var_wsrhstep);
            let assign33210_e41829: f64 = (assign33210_e41825 / assign33210_e41828);
            let assign33210_e41831: f64 = (assign33210_e41829 + locals.var_wsrhstep);
            let assign33210_e41835: f64 = (2.0 * locals.var_pbotd_i);
            let assign33210_e41836: f64 = (1.0 - assign33210_e41835);
            let assign33210_e41837: f64 = (assign33210_e41831 * assign33210_e41836);
            locals.var_dwsrh = assign33210_e41837;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 == 0.0)) {
            let assign33220_e41851: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign33220_e41851;
        }
        let assign33230_e41856: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard658 = assign33230_e41856;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard658 != 0.0)) {
            let assign33240_e41870: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv_d);
            let assign33240_e41871: f64 = (assign33240_e41870).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33240_e41871, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard658 == 0.0)) {
            let assign33250_e41888: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv_d);
            let assign33250_e41890: f64 = (assign33250_e41888).powf(locals.var_pbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33250_e41890, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 == 0.0)) {
            let assign33260_e41904: f64 = (locals.var_wdepnulrbot_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign33260_e41904, (locals.var_wdepnulrbot_d * locals.var_tmp_dn5), (locals.var_wdepnulrbot_d * locals.var_tmp_dn6), (locals.var_wdepnulrbot_d * locals.var_tmp_dn7), (locals.var_wdepnulrbot_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 == 0.0)) {
            let assign33270_e41919: f64 = (locals.var_zinv - 1.0);
            let assign33270_e41921: f64 = (assign33270_e41919 * locals.var_wdep);
            let assign33270_e41922: f64 = (locals.var_ftdbot_d * assign33270_e41921);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign33270_e41922, (locals.var_ftdbot_d * (assign33270_e41919 * locals.var_wdep_dn5)), (locals.var_ftdbot_d * (assign33270_e41919 * locals.var_wdep_dn6)), (locals.var_ftdbot_d * (assign33270_e41919 * locals.var_wdep_dn7)), (locals.var_ftdbot_d * (assign33270_e41919 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 == 0.0)) {
            let assign33280_e41937: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign33280_e41938: f64 = (locals.var_csrhbotd_i * assign33280_e41937);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign33280_e41938, (locals.var_csrhbotd_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign33290_e41943: f64 = if locals.var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard659 = assign33290_e41943;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33310_e41967: f64 = (locals.var_wdep * locals.var_one_minus_pbot_d);
            let assign33310_e41969: f64 = (assign33310_e41967 / locals.var_vbi_minus_vjsrh);
            let assign33310_e41970: f64 = (locals.var_btatpartbot_d * assign33310_e41969);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign33310_e41970, (locals.var_btatpartbot_d * ((locals.var_wdep_dn5 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn6 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn7 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn8 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33320_e41984: f64 = (0.666666666666667 * locals.var_atatbot_d);
            let assign33320_e41986: f64 = (assign33320_e41984 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign33320_e41986, (-((assign33320_e41984 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign33320_e41984 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign33320_e41984 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign33320_e41984 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33330_e42000: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign33330_e42000, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33340_e42014: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign33340_e42017: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign33340_e42019: f64 = (assign33340_e42017 + 1.0);
            let assign33340_e42020: f64 = (assign33340_e42014 / assign33340_e42019);
            let assign33340_e42021: f64 = (assign33340_e42020).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign33340_e42021, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign33340_e42019) - (assign33340_e42014 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign33340_e42019 * assign33340_e42019)) / (2.0 * assign33340_e42021)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign33340_e42019) - (assign33340_e42014 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign33340_e42019 * assign33340_e42019)) / (2.0 * assign33340_e42021)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign33340_e42019) - (assign33340_e42014 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign33340_e42019 * assign33340_e42019)) / (2.0 * assign33340_e42021)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign33340_e42019) - (assign33340_e42014 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign33340_e42019 * assign33340_e42019)) / (2.0 * assign33340_e42021)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33350_e42034: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign33350_e42034, (locals.var_umax_dn5 / (2.0 * assign33350_e42034)), (locals.var_umax_dn6 / (2.0 * assign33350_e42034)), (locals.var_umax_dn7 / (2.0 * assign33350_e42034)), (locals.var_umax_dn8 / (2.0 * assign33350_e42034)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33360_e42048: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign33360_e42048, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign33370_e42052: f64 = (-locals.var_pbotd_i);
        let assign33370_e42054: f64 = (assign33370_e42052 * locals.var_one_over_one_minus_pbot_d);
        let assign33370_e42056: f64 = (-1.0);
        let assign33370_e42057: f64 = if assign33370_e42054 == assign33370_e42056 { 1.0 } else { 0.0 };
        locals.var_guard660 = assign33370_e42057;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard660 != 0.0)) {
            let assign33380_e42073: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign33380_e42074: f64 = (1.0 + assign33380_e42073);
            let assign33380_e42075: f64 = (1.0 / assign33380_e42074);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign33380_e42075, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign33380_e42074 * assign33380_e42074))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign33380_e42074 * assign33380_e42074))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign33380_e42074 * assign33380_e42074))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign33380_e42074 * assign33380_e42074))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard660 == 0.0)) {
            let assign33390_e42093: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign33390_e42094: f64 = (1.0 + assign33390_e42093);
            let assign33390_e42096: f64 = (-locals.var_pbotd_i);
            let assign33390_e42098: f64 = (assign33390_e42096 * locals.var_one_over_one_minus_pbot_d);
            let assign33390_e42099: f64 = (assign33390_e42094).powf(assign33390_e42098);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign33390_e42099, if 0.0 == 0.0 && ((assign33390_e42098) as f64).is_finite() && ((assign33390_e42098) as f64).fract() == 0.0 { if assign33390_e42098 == 0.0 { 0.0 } else { (assign33390_e42098 * ((assign33390_e42094).powf(assign33390_e42098 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign33390_e42099 * (assign33390_e42098 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign33390_e42094))) }, if 0.0 == 0.0 && ((assign33390_e42098) as f64).is_finite() && ((assign33390_e42098) as f64).fract() == 0.0 { if assign33390_e42098 == 0.0 { 0.0 } else { (assign33390_e42098 * ((assign33390_e42094).powf(assign33390_e42098 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign33390_e42099 * (assign33390_e42098 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign33390_e42094))) }, if 0.0 == 0.0 && ((assign33390_e42098) as f64).is_finite() && ((assign33390_e42098) as f64).fract() == 0.0 { if assign33390_e42098 == 0.0 { 0.0 } else { (assign33390_e42098 * ((assign33390_e42094).powf(assign33390_e42098 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign33390_e42099 * (assign33390_e42098 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign33390_e42094))) }, if 0.0 == 0.0 && ((assign33390_e42098) as f64).is_finite() && ((assign33390_e42098) as f64).fract() == 0.0 { if assign33390_e42098 == 0.0 { 0.0 } else { (assign33390_e42098 * ((assign33390_e42094).powf(assign33390_e42098 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign33390_e42099 * (assign33390_e42098 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign33390_e42094))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33400_e42113: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign33400_e42116: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign33400_e42117: f64 = (assign33400_e42113 / assign33400_e42116);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign33400_e42117, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign33400_e42116) - (assign33400_e42113 * locals.var_wgamma_dn5)) / (assign33400_e42116 * assign33400_e42116)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign33400_e42116) - (assign33400_e42113 * locals.var_wgamma_dn6)) / (assign33400_e42116 * assign33400_e42116)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign33400_e42116) - (assign33400_e42113 * locals.var_wgamma_dn7)) / (assign33400_e42116 * assign33400_e42116)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign33400_e42116) - (assign33400_e42113 * locals.var_wgamma_dn8)) / (assign33400_e42116 * assign33400_e42116)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33410_e42132: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign33410_e42133: f64 = (0.375 * assign33410_e42132);
            let assign33410_e42134: f64 = (assign33410_e42133).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign33410_e42134, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign33410_e42134)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign33410_e42134)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign33410_e42134)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign33410_e42134)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33420_e42149: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign33420_e42150: f64 = (2.0 * assign33420_e42149);
            let assign33420_e42152: f64 = (assign33420_e42150 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign33420_e42152, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33430_e42166: f64 = (locals.var_atatbot_d * locals.var_twoatatoverthreebtat);
            let assign33430_e42168: f64 = (assign33430_e42166 * locals.var_sqrtumax);
            let assign33430_e42171: f64 = (locals.var_atatbot_d * locals.var_umax);
            let assign33430_e42172: f64 = (assign33430_e42168 - assign33430_e42171);
            let assign33430_e42176: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign33430_e42177: f64 = (0.5 * assign33430_e42176);
            let assign33430_e42178: f64 = (assign33430_e42172 + assign33430_e42177);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign33430_e42178, (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign33430_e42166 * locals.var_sqrtumax_dn5)) - (locals.var_atatbot_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign33430_e42166 * locals.var_sqrtumax_dn6)) - (locals.var_atatbot_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign33430_e42166 * locals.var_sqrtumax_dn7)) - (locals.var_atatbot_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign33430_e42166 * locals.var_sqrtumax_dn8)) - (locals.var_atatbot_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33440_e42192: f64 = (locals.var_ltat - 1.0);
            let assign33440_e42194: f64 = (assign33440_e42192 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign33440_e42194, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign33440_e42192 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign33440_e42192 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign33440_e42192 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign33440_e42192 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33450_e42208: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign33450_e42208, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign33460_e42213: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard661 = assign33460_e42213;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard661 != 0.0)) {
            let assign33470_e42229: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign33470_e42230: f64 = (1.0 + assign33470_e42229);
            let assign33470_e42231: f64 = (1.0 / assign33470_e42230);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign33470_e42231, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign33470_e42230 * assign33470_e42230))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign33470_e42230 * assign33470_e42230))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign33470_e42230 * assign33470_e42230))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign33470_e42230 * assign33470_e42230))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard661 == 0.0)) {
            let assign33480_e42250: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign33480_e42251: f64 = (1.0 - assign33480_e42250);
            let assign33480_e42252: f64 = (1.0 / assign33480_e42251);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign33480_e42252, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign33480_e42251 * assign33480_e42251))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign33480_e42251 * assign33480_e42251))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign33480_e42251 * assign33480_e42251))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign33480_e42251 * assign33480_e42251))), );
        }
        let assign33490_e42256: f64 = (-locals.var_ysq);
        let assign33490_e42258: f64 = (assign33490_e42256 + locals.var_mtat);
        let assign33490_e42260: f64 = (-230.25850929940458);
        let assign33490_e42261: f64 = if assign33490_e42258 > assign33490_e42260 { 1.0 } else { 0.0 };
        locals.var_guard662 = assign33490_e42261;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard662 != 0.0)) {
            let assign33500_e42274: f64 = (-locals.var_ysq);
            let assign33500_e42276: f64 = (assign33500_e42274 + locals.var_mtat);
            let assign33500_e42277: f64 = (assign33500_e42276).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33500_e42277, (assign33500_e42277 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign33500_e42277 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign33500_e42277 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign33500_e42277 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard662 == 0.0)) {
            let assign33510_e42295: f64 = (-230.25850929940458);
            let assign33510_e42297: f64 = (-locals.var_ysq);
            let assign33510_e42299: f64 = (assign33510_e42297 + locals.var_mtat);
            let assign33510_e42300: f64 = (assign33510_e42295 - assign33510_e42299);
            let assign33510_e42304: f64 = (-230.25850929940458);
            let assign33510_e42306: f64 = (-locals.var_ysq);
            let assign33510_e42308: f64 = (assign33510_e42306 + locals.var_mtat);
            let assign33510_e42309: f64 = (assign33510_e42304 - assign33510_e42308);
            let assign33510_e42312: f64 = (-230.25850929940458);
            let assign33510_e42314: f64 = (-locals.var_ysq);
            let assign33510_e42316: f64 = (assign33510_e42314 + locals.var_mtat);
            let assign33510_e42317: f64 = (assign33510_e42312 - assign33510_e42316);
            let assign33510_e42319: f64 = (assign33510_e42317 * 0.3333333333333333);
            let assign33510_e42320: f64 = (1.0 + assign33510_e42319);
            let assign33510_e42321: f64 = (assign33510_e42309 * assign33510_e42320);
            let assign33510_e42322: f64 = (0.5 * assign33510_e42321);
            let assign33510_e42323: f64 = (1.0 + assign33510_e42322);
            let assign33510_e42324: f64 = (assign33510_e42300 * assign33510_e42323);
            let assign33510_e42325: f64 = (1.0 + assign33510_e42324);
            let assign33510_e42326: f64 = (1e-100 / assign33510_e42325);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33510_e42326, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign33510_e42323) + (assign33510_e42300 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign33510_e42320) + (assign33510_e42309 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign33510_e42325 * assign33510_e42325))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign33510_e42323) + (assign33510_e42300 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign33510_e42320) + (assign33510_e42309 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign33510_e42325 * assign33510_e42325))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign33510_e42323) + (assign33510_e42300 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign33510_e42320) + (assign33510_e42309 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign33510_e42325 * assign33510_e42325))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign33510_e42323) + (assign33510_e42300 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign33510_e42320) + (assign33510_e42309 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign33510_e42325 * assign33510_e42325))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33520_e42340: f64 = (0.29214664 * locals.var_terfc);
            let assign33520_e42344: f64 = (locals.var_terfc * locals.var_terfc);
            let assign33520_e42345: f64 = (locals.var_berfc * assign33520_e42344);
            let assign33520_e42346: f64 = (assign33520_e42340 + assign33520_e42345);
            let assign33520_e42350: f64 = (locals.var_terfc * locals.var_terfc);
            let assign33520_e42352: f64 = (assign33520_e42350 * locals.var_terfc);
            let assign33520_e42353: f64 = (locals.var_cerfc * assign33520_e42352);
            let assign33520_e42354: f64 = (assign33520_e42346 + assign33520_e42353);
            let assign33520_e42356: f64 = (assign33520_e42354 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign33520_e42356, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign33520_e42350 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign33520_e42354 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign33520_e42350 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign33520_e42354 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign33520_e42350 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign33520_e42354 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign33520_e42350 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign33520_e42354 * locals.var_tmp_dn8)), );
        }
        let assign33530_e42361: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard663 = assign33530_e42361;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign33550_e42378: f64 = (-230.25850929940458);
        let assign33550_e42379: f64 = if locals.var_mtat > assign33550_e42378 { 1.0 } else { 0.0 };
        locals.var_guard664 = assign33550_e42379;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard664 != 0.0)) {
            let assign33560_e42395: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33560_e42395, (assign33560_e42395 * locals.var_mtat_dn5), (assign33560_e42395 * locals.var_mtat_dn6), (assign33560_e42395 * locals.var_mtat_dn7), (assign33560_e42395 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard664 == 0.0)) {
            let assign33570_e42416: f64 = (-230.25850929940458);
            let assign33570_e42418: f64 = (assign33570_e42416 - locals.var_mtat);
            let assign33570_e42422: f64 = (-230.25850929940458);
            let assign33570_e42424: f64 = (assign33570_e42422 - locals.var_mtat);
            let assign33570_e42427: f64 = (-230.25850929940458);
            let assign33570_e42429: f64 = (assign33570_e42427 - locals.var_mtat);
            let assign33570_e42431: f64 = (assign33570_e42429 * 0.3333333333333333);
            let assign33570_e42432: f64 = (1.0 + assign33570_e42431);
            let assign33570_e42433: f64 = (assign33570_e42424 * assign33570_e42432);
            let assign33570_e42434: f64 = (0.5 * assign33570_e42433);
            let assign33570_e42435: f64 = (1.0 + assign33570_e42434);
            let assign33570_e42436: f64 = (assign33570_e42418 * assign33570_e42435);
            let assign33570_e42437: f64 = (1.0 + assign33570_e42436);
            let assign33570_e42438: f64 = (1e-100 / assign33570_e42437);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33570_e42438, (-((1e-100 * (((-locals.var_mtat_dn5) * assign33570_e42435) + (assign33570_e42418 * (0.5 * (((-locals.var_mtat_dn5) * assign33570_e42432) + (assign33570_e42424 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign33570_e42437 * assign33570_e42437))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign33570_e42435) + (assign33570_e42418 * (0.5 * (((-locals.var_mtat_dn6) * assign33570_e42432) + (assign33570_e42424 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign33570_e42437 * assign33570_e42437))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign33570_e42435) + (assign33570_e42418 * (0.5 * (((-locals.var_mtat_dn7) * assign33570_e42432) + (assign33570_e42424 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign33570_e42437 * assign33570_e42437))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign33570_e42435) + (assign33570_e42418 * (0.5 * (((-locals.var_mtat_dn8) * assign33570_e42432) + (assign33570_e42424 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign33570_e42437 * assign33570_e42437))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 == 0.0)) {
            let assign33580_e42455: f64 = (2.0 * locals.var_tmp);
            let assign33580_e42457: f64 = (assign33580_e42455 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign33580_e42457, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33590_e42471: f64 = (1.772453850905516 * 0.5);
            let assign33590_e42474: f64 = (locals.var_atatbot_d * locals.var_erfctimesexpmtat);
            let assign33590_e42476: f64 = (assign33590_e42474 / locals.var_ktat);
            let assign33590_e42477: f64 = (assign33590_e42471 * assign33590_e42476);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign33590_e42477, (assign33590_e42471 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign33590_e42474 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign33590_e42471 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign33590_e42474 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign33590_e42471 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign33590_e42474 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign33590_e42471 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign33590_e42474 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard659 == 0.0)) {
            let assign33600_e42492: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign33600_e42494: f64 = (assign33600_e42492 * locals.var_wtat);
            let assign33600_e42495: f64 = (locals.var_ctatbotd_i * assign33600_e42494);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign33600_e42495, (locals.var_ctatbotd_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign33600_e42492 * locals.var_wtat_dn5))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign33600_e42492 * locals.var_wtat_dn6))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign33600_e42492 * locals.var_wtat_dn7))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign33600_e42492 * locals.var_wtat_dn8))), );
        }
        let assign33610_e42500: f64 = if locals.var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard665 = assign33610_e42500;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard665 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign33630_e42514: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard666 = assign33630_e42514;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 != 0.0)) {
            let assign33640_e42528: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign33640_e42530: f64 = (assign33640_e42528 * locals.var_vbirbotinv_d);
            let assign33640_e42531: f64 = (assign33640_e42530).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33640_e42531, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 == 0.0)) {
            let assign33650_e42548: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign33650_e42550: f64 = (assign33650_e42548 * locals.var_vbirbotinv_d);
            let assign33650_e42552: f64 = (assign33650_e42550).powf(locals.var_pbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33650_e42552, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard665 == 0.0)) {
            let assign33660_e42567: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign33660_e42569: f64 = (assign33660_e42567 * locals.var_wdepnulrinvbot_d);
            let assign33660_e42571: f64 = (assign33660_e42569 / locals.var_tmp);
            let assign33660_e42572: f64 = (locals.var_one_over_one_minus_pbot_d * assign33660_e42571);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign33660_e42572, (locals.var_one_over_one_minus_pbot_d * (-((assign33660_e42569 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign33660_e42569 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign33660_e42569 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign33660_e42569 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign33670_e42576: f64 = (-locals.var_fbbtbot_d);
        let assign33670_e42578: f64 = (assign33670_e42576 / locals.var_fmaxr);
        let assign33670_e42579: f64 = (assign33670_e42578).abs();
        let assign33670_e42581: f64 = if assign33670_e42579 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard667 = assign33670_e42581;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard667 != 0.0)) {
            let assign33680_e42594: f64 = (-locals.var_fbbtbot_d);
            let assign33680_e42596: f64 = (assign33680_e42594 / locals.var_fmaxr);
            let assign33680_e42597: f64 = (assign33680_e42596).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33680_e42597, (assign33680_e42597 * (-((assign33680_e42594 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign33680_e42597 * (-((assign33680_e42594 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign33680_e42597 * (-((assign33680_e42594 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign33680_e42597 * (-((assign33680_e42594 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign33690_e42601: f64 = (-locals.var_fbbtbot_d);
        let assign33690_e42603: f64 = (assign33690_e42601 / locals.var_fmaxr);
        let assign33690_e42605: f64 = if assign33690_e42603 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard668 = assign33690_e42605;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 != 0.0)) {
            let assign33700_e42623: f64 = (-230.25850929940458);
            let assign33700_e42625: f64 = (-locals.var_fbbtbot_d);
            let assign33700_e42627: f64 = (assign33700_e42625 / locals.var_fmaxr);
            let assign33700_e42628: f64 = (assign33700_e42623 - assign33700_e42627);
            let assign33700_e42632: f64 = (-230.25850929940458);
            let assign33700_e42634: f64 = (-locals.var_fbbtbot_d);
            let assign33700_e42636: f64 = (assign33700_e42634 / locals.var_fmaxr);
            let assign33700_e42637: f64 = (assign33700_e42632 - assign33700_e42636);
            let assign33700_e42640: f64 = (-230.25850929940458);
            let assign33700_e42642: f64 = (-locals.var_fbbtbot_d);
            let assign33700_e42644: f64 = (assign33700_e42642 / locals.var_fmaxr);
            let assign33700_e42645: f64 = (assign33700_e42640 - assign33700_e42644);
            let assign33700_e42647: f64 = (assign33700_e42645 * 0.3333333333333333);
            let assign33700_e42648: f64 = (1.0 + assign33700_e42647);
            let assign33700_e42649: f64 = (assign33700_e42637 * assign33700_e42648);
            let assign33700_e42650: f64 = (0.5 * assign33700_e42649);
            let assign33700_e42651: f64 = (1.0 + assign33700_e42650);
            let assign33700_e42652: f64 = (assign33700_e42628 * assign33700_e42651);
            let assign33700_e42653: f64 = (1.0 + assign33700_e42652);
            let assign33700_e42654: f64 = (1e-100 / assign33700_e42653);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33700_e42654, (-((1e-100 * (((-(-((assign33700_e42625 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign33700_e42651) + (assign33700_e42628 * (0.5 * (((-(-((assign33700_e42634 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign33700_e42648) + (assign33700_e42637 * ((-(-((assign33700_e42642 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign33700_e42653 * assign33700_e42653))), (-((1e-100 * (((-(-((assign33700_e42625 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign33700_e42651) + (assign33700_e42628 * (0.5 * (((-(-((assign33700_e42634 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign33700_e42648) + (assign33700_e42637 * ((-(-((assign33700_e42642 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign33700_e42653 * assign33700_e42653))), (-((1e-100 * (((-(-((assign33700_e42625 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign33700_e42651) + (assign33700_e42628 * (0.5 * (((-(-((assign33700_e42634 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign33700_e42648) + (assign33700_e42637 * ((-(-((assign33700_e42642 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign33700_e42653 * assign33700_e42653))), (-((1e-100 * (((-(-((assign33700_e42625 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign33700_e42651) + (assign33700_e42628 * (0.5 * (((-(-((assign33700_e42634 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign33700_e42648) + (assign33700_e42637 * ((-(-((assign33700_e42642 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign33700_e42653 * assign33700_e42653))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 == 0.0)) {
            let assign33710_e42675: f64 = (-locals.var_fbbtbot_d);
            let assign33710_e42677: f64 = (assign33710_e42675 / locals.var_fmaxr);
            let assign33710_e42679: f64 = (assign33710_e42677 - 230.25850929940458);
            let assign33710_e42683: f64 = (-locals.var_fbbtbot_d);
            let assign33710_e42685: f64 = (assign33710_e42683 / locals.var_fmaxr);
            let assign33710_e42687: f64 = (assign33710_e42685 - 230.25850929940458);
            let assign33710_e42690: f64 = (-locals.var_fbbtbot_d);
            let assign33710_e42692: f64 = (assign33710_e42690 / locals.var_fmaxr);
            let assign33710_e42694: f64 = (assign33710_e42692 - 230.25850929940458);
            let assign33710_e42696: f64 = (assign33710_e42694 * 0.3333333333333333);
            let assign33710_e42697: f64 = (1.0 + assign33710_e42696);
            let assign33710_e42698: f64 = (assign33710_e42687 * assign33710_e42697);
            let assign33710_e42699: f64 = (0.5 * assign33710_e42698);
            let assign33710_e42700: f64 = (1.0 + assign33710_e42699);
            let assign33710_e42701: f64 = (assign33710_e42679 * assign33710_e42700);
            let assign33710_e42702: f64 = (1.0 + assign33710_e42701);
            let assign33710_e42703: f64 = (1e100 * assign33710_e42702);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33710_e42703, (1e100 * (((-((assign33710_e42675 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign33710_e42700) + (assign33710_e42679 * (0.5 * (((-((assign33710_e42683 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign33710_e42697) + (assign33710_e42687 * ((-((assign33710_e42690 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign33710_e42675 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign33710_e42700) + (assign33710_e42679 * (0.5 * (((-((assign33710_e42683 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign33710_e42697) + (assign33710_e42687 * ((-((assign33710_e42690 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign33710_e42675 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign33710_e42700) + (assign33710_e42679 * (0.5 * (((-((assign33710_e42683 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign33710_e42697) + (assign33710_e42687 * ((-((assign33710_e42690 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign33710_e42675 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign33710_e42700) + (assign33710_e42679 * (0.5 * (((-((assign33710_e42683 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign33710_e42697) + (assign33710_e42687 * ((-((assign33710_e42690 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard665 == 0.0)) {
            let assign33720_e42718: f64 = (locals.var_v3 * locals.var_fmaxr);
            let assign33720_e42720: f64 = (assign33720_e42718 * locals.var_fmaxr);
            let assign33720_e42722: f64 = (assign33720_e42720 * locals.var_tmp);
            let assign33720_e42723: f64 = (locals.var_cbbtbotd_i * assign33720_e42722);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign33720_e42723, (locals.var_cbbtbotd_i * (((((locals.var_v3 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign33720_e42718 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign33720_e42720 * locals.var_tmp_dn5))), (locals.var_cbbtbotd_i * (((((locals.var_v3 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign33720_e42718 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign33720_e42720 * locals.var_tmp_dn6))), (locals.var_cbbtbotd_i * (((((locals.var_v3 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign33720_e42718 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign33720_e42720 * locals.var_tmp_dn7))), (locals.var_cbbtbotd_i * (((((locals.var_v3 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign33720_e42718 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign33720_e42720 * locals.var_tmp_dn8))), );
        }
        let assign33730_e42728: f64 = if locals.var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard669 = assign33730_e42728;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard669 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign33750_e42742: f64 = (-locals.var_alphaav);
        let assign33750_e42744: f64 = (assign33750_e42742 * locals.var_vbrbotd_i);
        let assign33750_e42745: f64 = if locals.var_vav > assign33750_e42744 { 1.0 } else { 0.0 };
        locals.var_guard670 = assign33750_e42745;
        let assign33760_e42748: f64 = if locals.var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard671 = assign33760_e42748;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) {
            let assign33770_e42764: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign33770_e42767: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign33770_e42768: f64 = (assign33770_e42764 * assign33770_e42767);
            let assign33770_e42771: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign33770_e42772: f64 = (assign33770_e42768 * assign33770_e42771);
            let assign33770_e42775: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign33770_e42776: f64 = (assign33770_e42772 * assign33770_e42775);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33770_e42776, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 == 0.0)) {
            let assign33780_e42795: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign33780_e42796: f64 = (assign33780_e42795).abs();
            let assign33780_e42798: f64 = (assign33780_e42796).powf(locals.var_pbrbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33780_e42798, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard670 != 0.0)) {
            let assign33790_e42815: f64 = (1.0 - locals.var_tmp);
            let assign33790_e42816: f64 = (1.0 / assign33790_e42815);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign33790_e42816, (-((-locals.var_tmp_dn5) / (assign33790_e42815 * assign33790_e42815))), (-((-locals.var_tmp_dn6) / (assign33790_e42815 * assign33790_e42815))), (-((-locals.var_tmp_dn7) / (assign33790_e42815 * assign33790_e42815))), (-((-locals.var_tmp_dn8) / (assign33790_e42815 * assign33790_e42815))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard670 == 0.0)) {
            let assign33800_e42835: f64 = (locals.var_alphaav * locals.var_vbrbotd_i);
            let assign33800_e42836: f64 = (locals.var_vav + assign33800_e42835);
            let assign33800_e42838: f64 = (assign33800_e42836 * locals.var_slopebot_d);
            let assign33800_e42839: f64 = (locals.var_fstopbot_d + assign33800_e42838);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign33800_e42839, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard655 == 0.0)) {
            let assign33810_e42851: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign33810_e42853: f64 = (assign33810_e42851 + locals.var_itat);
            let assign33810_e42855: f64 = (assign33810_e42853 + locals.var_ibbt);
            let assign33810_e42856: f64 = (p.p29 * assign33810_e42855);
            let assign33810_e42858: f64 = (assign33810_e42856 * locals.var_fbreakdown);
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (assign33810_e42858, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign33810_e42856 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign33810_e42856 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign33810_e42856 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign33810_e42856 * locals.var_fbreakdown_dn8)), );
        }
        let assign33820_e42863: f64 = if locals.var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard672 = assign33820_e42863;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 != 0.0)) {
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) {
            let assign33840_e42880: f64 = (locals.var_idsatsti_d * locals.var_idmult);
            locals.var_id__blk219 = assign33840_e42880;
        }
        let assign33850_e42889: f64 = if ((locals.var_csrhstid_i == 0.0) && (locals.var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard673 = assign33850_e42889;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) {
            let assign33870_e42912: f64 = (locals.var_vbisti_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign33870_e42912;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) {
            let assign33880_e42928: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign33880_e42929: f64 = (1.0 - assign33880_e42928);
            let assign33880_e42930: f64 = (assign33880_e42929).sqrt();
            let assign33880_e42931: f64 = (1.0 - assign33880_e42930);
            locals.var_wsrhstep = assign33880_e42931;
        }
        let assign33890_e42936: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard674 = assign33890_e42936;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) && (locals.var_guard674 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) && (locals.var_guard674 == 0.0)) {
            let assign33910_e42965: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign33910_e42967: f64 = (locals.var_wsrhstep).ln();
            let assign33910_e42968: f64 = (assign33910_e42965 * assign33910_e42967);
            let assign33910_e42971: f64 = (1.0 - locals.var_wsrhstep);
            let assign33910_e42972: f64 = (assign33910_e42968 / assign33910_e42971);
            let assign33910_e42974: f64 = (assign33910_e42972 + locals.var_wsrhstep);
            let assign33910_e42978: f64 = (2.0 * locals.var_pstid_i);
            let assign33910_e42979: f64 = (1.0 - assign33910_e42978);
            let assign33910_e42980: f64 = (assign33910_e42974 * assign33910_e42979);
            locals.var_dwsrh = assign33910_e42980;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) {
            let assign33920_e42994: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign33920_e42994;
        }
        let assign33930_e42999: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard675 = assign33930_e42999;
    }
    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) && (locals.var_guard675 != 0.0)) {
            let assign33940_e43013: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv_d);
            let assign33940_e43014: f64 = (assign33940_e43013).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33940_e43014, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) && (locals.var_guard675 == 0.0)) {
            let assign33950_e43031: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv_d);
            let assign33950_e43033: f64 = (assign33950_e43031).powf(locals.var_pstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign33950_e43033, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) {
            let assign33960_e43047: f64 = (locals.var_wdepnulrsti_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign33960_e43047, (locals.var_wdepnulrsti_d * locals.var_tmp_dn5), (locals.var_wdepnulrsti_d * locals.var_tmp_dn6), (locals.var_wdepnulrsti_d * locals.var_tmp_dn7), (locals.var_wdepnulrsti_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) {
            let assign33970_e43062: f64 = (locals.var_zinv - 1.0);
            let assign33970_e43064: f64 = (assign33970_e43062 * locals.var_wdep);
            let assign33970_e43065: f64 = (locals.var_ftdsti_d * assign33970_e43064);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign33970_e43065, (locals.var_ftdsti_d * (assign33970_e43062 * locals.var_wdep_dn5)), (locals.var_ftdsti_d * (assign33970_e43062 * locals.var_wdep_dn6)), (locals.var_ftdsti_d * (assign33970_e43062 * locals.var_wdep_dn7)), (locals.var_ftdsti_d * (assign33970_e43062 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) {
            let assign33980_e43080: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign33980_e43081: f64 = (locals.var_csrhstid_i * assign33980_e43080);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign33980_e43081, (locals.var_csrhstid_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign33990_e43086: f64 = if locals.var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard676 = assign33990_e43086;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34010_e43110: f64 = (locals.var_wdep * locals.var_one_minus_psti_d);
            let assign34010_e43112: f64 = (assign34010_e43110 / locals.var_vbi_minus_vjsrh);
            let assign34010_e43113: f64 = (locals.var_btatpartsti_d * assign34010_e43112);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign34010_e43113, (locals.var_btatpartsti_d * ((locals.var_wdep_dn5 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn6 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn7 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn8 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34020_e43127: f64 = (0.666666666666667 * locals.var_atatsti_d);
            let assign34020_e43129: f64 = (assign34020_e43127 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign34020_e43129, (-((assign34020_e43127 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign34020_e43127 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign34020_e43127 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign34020_e43127 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34030_e43143: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign34030_e43143, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34040_e43157: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign34040_e43160: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign34040_e43162: f64 = (assign34040_e43160 + 1.0);
            let assign34040_e43163: f64 = (assign34040_e43157 / assign34040_e43162);
            let assign34040_e43164: f64 = (assign34040_e43163).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign34040_e43164, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign34040_e43162) - (assign34040_e43157 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign34040_e43162 * assign34040_e43162)) / (2.0 * assign34040_e43164)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign34040_e43162) - (assign34040_e43157 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign34040_e43162 * assign34040_e43162)) / (2.0 * assign34040_e43164)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign34040_e43162) - (assign34040_e43157 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign34040_e43162 * assign34040_e43162)) / (2.0 * assign34040_e43164)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign34040_e43162) - (assign34040_e43157 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign34040_e43162 * assign34040_e43162)) / (2.0 * assign34040_e43164)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34050_e43177: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign34050_e43177, (locals.var_umax_dn5 / (2.0 * assign34050_e43177)), (locals.var_umax_dn6 / (2.0 * assign34050_e43177)), (locals.var_umax_dn7 / (2.0 * assign34050_e43177)), (locals.var_umax_dn8 / (2.0 * assign34050_e43177)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34060_e43191: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign34060_e43191, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign34070_e43195: f64 = (-locals.var_pstid_i);
        let assign34070_e43197: f64 = (assign34070_e43195 * locals.var_one_over_one_minus_psti_d);
        let assign34070_e43199: f64 = (-1.0);
        let assign34070_e43200: f64 = if assign34070_e43197 == assign34070_e43199 { 1.0 } else { 0.0 };
        locals.var_guard677 = assign34070_e43200;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard677 != 0.0)) {
            let assign34080_e43216: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign34080_e43217: f64 = (1.0 + assign34080_e43216);
            let assign34080_e43218: f64 = (1.0 / assign34080_e43217);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign34080_e43218, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign34080_e43217 * assign34080_e43217))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign34080_e43217 * assign34080_e43217))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign34080_e43217 * assign34080_e43217))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign34080_e43217 * assign34080_e43217))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard677 == 0.0)) {
            let assign34090_e43236: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign34090_e43237: f64 = (1.0 + assign34090_e43236);
            let assign34090_e43239: f64 = (-locals.var_pstid_i);
            let assign34090_e43241: f64 = (assign34090_e43239 * locals.var_one_over_one_minus_psti_d);
            let assign34090_e43242: f64 = (assign34090_e43237).powf(assign34090_e43241);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign34090_e43242, if 0.0 == 0.0 && ((assign34090_e43241) as f64).is_finite() && ((assign34090_e43241) as f64).fract() == 0.0 { if assign34090_e43241 == 0.0 { 0.0 } else { (assign34090_e43241 * ((assign34090_e43237).powf(assign34090_e43241 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign34090_e43242 * (assign34090_e43241 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign34090_e43237))) }, if 0.0 == 0.0 && ((assign34090_e43241) as f64).is_finite() && ((assign34090_e43241) as f64).fract() == 0.0 { if assign34090_e43241 == 0.0 { 0.0 } else { (assign34090_e43241 * ((assign34090_e43237).powf(assign34090_e43241 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign34090_e43242 * (assign34090_e43241 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign34090_e43237))) }, if 0.0 == 0.0 && ((assign34090_e43241) as f64).is_finite() && ((assign34090_e43241) as f64).fract() == 0.0 { if assign34090_e43241 == 0.0 { 0.0 } else { (assign34090_e43241 * ((assign34090_e43237).powf(assign34090_e43241 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign34090_e43242 * (assign34090_e43241 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign34090_e43237))) }, if 0.0 == 0.0 && ((assign34090_e43241) as f64).is_finite() && ((assign34090_e43241) as f64).fract() == 0.0 { if assign34090_e43241 == 0.0 { 0.0 } else { (assign34090_e43241 * ((assign34090_e43237).powf(assign34090_e43241 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign34090_e43242 * (assign34090_e43241 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign34090_e43237))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34100_e43256: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign34100_e43259: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign34100_e43260: f64 = (assign34100_e43256 / assign34100_e43259);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign34100_e43260, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign34100_e43259) - (assign34100_e43256 * locals.var_wgamma_dn5)) / (assign34100_e43259 * assign34100_e43259)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign34100_e43259) - (assign34100_e43256 * locals.var_wgamma_dn6)) / (assign34100_e43259 * assign34100_e43259)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign34100_e43259) - (assign34100_e43256 * locals.var_wgamma_dn7)) / (assign34100_e43259 * assign34100_e43259)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign34100_e43259) - (assign34100_e43256 * locals.var_wgamma_dn8)) / (assign34100_e43259 * assign34100_e43259)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34110_e43275: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign34110_e43276: f64 = (0.375 * assign34110_e43275);
            let assign34110_e43277: f64 = (assign34110_e43276).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign34110_e43277, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign34110_e43277)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign34110_e43277)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign34110_e43277)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign34110_e43277)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34120_e43292: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign34120_e43293: f64 = (2.0 * assign34120_e43292);
            let assign34120_e43295: f64 = (assign34120_e43293 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign34120_e43295, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34130_e43309: f64 = (locals.var_atatsti_d * locals.var_twoatatoverthreebtat);
            let assign34130_e43311: f64 = (assign34130_e43309 * locals.var_sqrtumax);
            let assign34130_e43314: f64 = (locals.var_atatsti_d * locals.var_umax);
            let assign34130_e43315: f64 = (assign34130_e43311 - assign34130_e43314);
            let assign34130_e43319: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign34130_e43320: f64 = (0.5 * assign34130_e43319);
            let assign34130_e43321: f64 = (assign34130_e43315 + assign34130_e43320);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign34130_e43321, (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign34130_e43309 * locals.var_sqrtumax_dn5)) - (locals.var_atatsti_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign34130_e43309 * locals.var_sqrtumax_dn6)) - (locals.var_atatsti_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign34130_e43309 * locals.var_sqrtumax_dn7)) - (locals.var_atatsti_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign34130_e43309 * locals.var_sqrtumax_dn8)) - (locals.var_atatsti_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34140_e43335: f64 = (locals.var_ltat - 1.0);
            let assign34140_e43337: f64 = (assign34140_e43335 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign34140_e43337, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign34140_e43335 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign34140_e43335 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign34140_e43335 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign34140_e43335 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34150_e43351: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign34150_e43351, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign34160_e43356: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard678 = assign34160_e43356;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard678 != 0.0)) {
            let assign34170_e43372: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign34170_e43373: f64 = (1.0 + assign34170_e43372);
            let assign34170_e43374: f64 = (1.0 / assign34170_e43373);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign34170_e43374, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign34170_e43373 * assign34170_e43373))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign34170_e43373 * assign34170_e43373))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign34170_e43373 * assign34170_e43373))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign34170_e43373 * assign34170_e43373))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard678 == 0.0)) {
            let assign34180_e43393: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign34180_e43394: f64 = (1.0 - assign34180_e43393);
            let assign34180_e43395: f64 = (1.0 / assign34180_e43394);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign34180_e43395, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign34180_e43394 * assign34180_e43394))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign34180_e43394 * assign34180_e43394))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign34180_e43394 * assign34180_e43394))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign34180_e43394 * assign34180_e43394))), );
        }
        let assign34190_e43399: f64 = (-locals.var_ysq);
        let assign34190_e43401: f64 = (assign34190_e43399 + locals.var_mtat);
        let assign34190_e43403: f64 = (-230.25850929940458);
        let assign34190_e43404: f64 = if assign34190_e43401 > assign34190_e43403 { 1.0 } else { 0.0 };
        locals.var_guard679 = assign34190_e43404;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard679 != 0.0)) {
            let assign34200_e43417: f64 = (-locals.var_ysq);
            let assign34200_e43419: f64 = (assign34200_e43417 + locals.var_mtat);
            let assign34200_e43420: f64 = (assign34200_e43419).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34200_e43420, (assign34200_e43420 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign34200_e43420 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign34200_e43420 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign34200_e43420 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard679 == 0.0)) {
            let assign34210_e43438: f64 = (-230.25850929940458);
            let assign34210_e43440: f64 = (-locals.var_ysq);
            let assign34210_e43442: f64 = (assign34210_e43440 + locals.var_mtat);
            let assign34210_e43443: f64 = (assign34210_e43438 - assign34210_e43442);
            let assign34210_e43447: f64 = (-230.25850929940458);
            let assign34210_e43449: f64 = (-locals.var_ysq);
            let assign34210_e43451: f64 = (assign34210_e43449 + locals.var_mtat);
            let assign34210_e43452: f64 = (assign34210_e43447 - assign34210_e43451);
            let assign34210_e43455: f64 = (-230.25850929940458);
            let assign34210_e43457: f64 = (-locals.var_ysq);
            let assign34210_e43459: f64 = (assign34210_e43457 + locals.var_mtat);
            let assign34210_e43460: f64 = (assign34210_e43455 - assign34210_e43459);
            let assign34210_e43462: f64 = (assign34210_e43460 * 0.3333333333333333);
            let assign34210_e43463: f64 = (1.0 + assign34210_e43462);
            let assign34210_e43464: f64 = (assign34210_e43452 * assign34210_e43463);
            let assign34210_e43465: f64 = (0.5 * assign34210_e43464);
            let assign34210_e43466: f64 = (1.0 + assign34210_e43465);
            let assign34210_e43467: f64 = (assign34210_e43443 * assign34210_e43466);
            let assign34210_e43468: f64 = (1.0 + assign34210_e43467);
            let assign34210_e43469: f64 = (1e-100 / assign34210_e43468);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34210_e43469, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign34210_e43466) + (assign34210_e43443 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign34210_e43463) + (assign34210_e43452 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign34210_e43468 * assign34210_e43468))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign34210_e43466) + (assign34210_e43443 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign34210_e43463) + (assign34210_e43452 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign34210_e43468 * assign34210_e43468))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign34210_e43466) + (assign34210_e43443 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign34210_e43463) + (assign34210_e43452 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign34210_e43468 * assign34210_e43468))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign34210_e43466) + (assign34210_e43443 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign34210_e43463) + (assign34210_e43452 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign34210_e43468 * assign34210_e43468))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34220_e43483: f64 = (0.29214664 * locals.var_terfc);
            let assign34220_e43487: f64 = (locals.var_terfc * locals.var_terfc);
            let assign34220_e43488: f64 = (locals.var_berfc * assign34220_e43487);
            let assign34220_e43489: f64 = (assign34220_e43483 + assign34220_e43488);
            let assign34220_e43493: f64 = (locals.var_terfc * locals.var_terfc);
            let assign34220_e43495: f64 = (assign34220_e43493 * locals.var_terfc);
            let assign34220_e43496: f64 = (locals.var_cerfc * assign34220_e43495);
            let assign34220_e43497: f64 = (assign34220_e43489 + assign34220_e43496);
            let assign34220_e43499: f64 = (assign34220_e43497 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign34220_e43499, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign34220_e43493 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign34220_e43497 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign34220_e43493 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign34220_e43497 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign34220_e43493 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign34220_e43497 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign34220_e43493 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign34220_e43497 * locals.var_tmp_dn8)), );
        }
        let assign34230_e43504: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard680 = assign34230_e43504;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard680 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign34250_e43521: f64 = (-230.25850929940458);
        let assign34250_e43522: f64 = if locals.var_mtat > assign34250_e43521 { 1.0 } else { 0.0 };
        locals.var_guard681 = assign34250_e43522;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 != 0.0)) {
            let assign34260_e43538: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34260_e43538, (assign34260_e43538 * locals.var_mtat_dn5), (assign34260_e43538 * locals.var_mtat_dn6), (assign34260_e43538 * locals.var_mtat_dn7), (assign34260_e43538 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) {
            let assign34270_e43559: f64 = (-230.25850929940458);
            let assign34270_e43561: f64 = (assign34270_e43559 - locals.var_mtat);
            let assign34270_e43565: f64 = (-230.25850929940458);
            let assign34270_e43567: f64 = (assign34270_e43565 - locals.var_mtat);
            let assign34270_e43570: f64 = (-230.25850929940458);
            let assign34270_e43572: f64 = (assign34270_e43570 - locals.var_mtat);
            let assign34270_e43574: f64 = (assign34270_e43572 * 0.3333333333333333);
            let assign34270_e43575: f64 = (1.0 + assign34270_e43574);
            let assign34270_e43576: f64 = (assign34270_e43567 * assign34270_e43575);
            let assign34270_e43577: f64 = (0.5 * assign34270_e43576);
            let assign34270_e43578: f64 = (1.0 + assign34270_e43577);
            let assign34270_e43579: f64 = (assign34270_e43561 * assign34270_e43578);
            let assign34270_e43580: f64 = (1.0 + assign34270_e43579);
            let assign34270_e43581: f64 = (1e-100 / assign34270_e43580);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34270_e43581, (-((1e-100 * (((-locals.var_mtat_dn5) * assign34270_e43578) + (assign34270_e43561 * (0.5 * (((-locals.var_mtat_dn5) * assign34270_e43575) + (assign34270_e43567 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign34270_e43580 * assign34270_e43580))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign34270_e43578) + (assign34270_e43561 * (0.5 * (((-locals.var_mtat_dn6) * assign34270_e43575) + (assign34270_e43567 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign34270_e43580 * assign34270_e43580))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign34270_e43578) + (assign34270_e43561 * (0.5 * (((-locals.var_mtat_dn7) * assign34270_e43575) + (assign34270_e43567 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign34270_e43580 * assign34270_e43580))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign34270_e43578) + (assign34270_e43561 * (0.5 * (((-locals.var_mtat_dn8) * assign34270_e43575) + (assign34270_e43567 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign34270_e43580 * assign34270_e43580))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard680 == 0.0)) {
            let assign34280_e43598: f64 = (2.0 * locals.var_tmp);
            let assign34280_e43600: f64 = (assign34280_e43598 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign34280_e43600, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34290_e43614: f64 = (1.772453850905516 * 0.5);
            let assign34290_e43617: f64 = (locals.var_atatsti_d * locals.var_erfctimesexpmtat);
            let assign34290_e43619: f64 = (assign34290_e43617 / locals.var_ktat);
            let assign34290_e43620: f64 = (assign34290_e43614 * assign34290_e43619);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign34290_e43620, (assign34290_e43614 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign34290_e43617 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign34290_e43614 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign34290_e43617 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign34290_e43614 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign34290_e43617 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign34290_e43614 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign34290_e43617 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard676 == 0.0)) {
            let assign34300_e43635: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign34300_e43637: f64 = (assign34300_e43635 * locals.var_wtat);
            let assign34300_e43638: f64 = (locals.var_ctatstid_i * assign34300_e43637);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign34300_e43638, (locals.var_ctatstid_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign34300_e43635 * locals.var_wtat_dn5))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign34300_e43635 * locals.var_wtat_dn6))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign34300_e43635 * locals.var_wtat_dn7))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign34300_e43635 * locals.var_wtat_dn8))), );
        }
        let assign34310_e43643: f64 = if locals.var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard682 = assign34310_e43643;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard682 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign34330_e43657: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard683 = assign34330_e43657;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 != 0.0)) {
            let assign34340_e43671: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign34340_e43673: f64 = (assign34340_e43671 * locals.var_vbirstiinv_d);
            let assign34340_e43674: f64 = (assign34340_e43673).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34340_e43674, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) {
            let assign34350_e43691: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign34350_e43693: f64 = (assign34350_e43691 * locals.var_vbirstiinv_d);
            let assign34350_e43695: f64 = (assign34350_e43693).powf(locals.var_pstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34350_e43695, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard682 == 0.0)) {
            let assign34360_e43710: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign34360_e43712: f64 = (assign34360_e43710 * locals.var_wdepnulrinvsti_d);
            let assign34360_e43714: f64 = (assign34360_e43712 / locals.var_tmp);
            let assign34360_e43715: f64 = (locals.var_one_over_one_minus_psti_d * assign34360_e43714);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign34360_e43715, (locals.var_one_over_one_minus_psti_d * (-((assign34360_e43712 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign34360_e43712 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign34360_e43712 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign34360_e43712 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign34370_e43719: f64 = (-locals.var_fbbtsti_d);
        let assign34370_e43721: f64 = (assign34370_e43719 / locals.var_fmaxr);
        let assign34370_e43722: f64 = (assign34370_e43721).abs();
        let assign34370_e43724: f64 = if assign34370_e43722 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard684 = assign34370_e43724;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard682 == 0.0)) && (locals.var_guard684 != 0.0)) {
            let assign34380_e43737: f64 = (-locals.var_fbbtsti_d);
            let assign34380_e43739: f64 = (assign34380_e43737 / locals.var_fmaxr);
            let assign34380_e43740: f64 = (assign34380_e43739).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34380_e43740, (assign34380_e43740 * (-((assign34380_e43737 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign34380_e43740 * (-((assign34380_e43737 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign34380_e43740 * (-((assign34380_e43737 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign34380_e43740 * (-((assign34380_e43737 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign34390_e43744: f64 = (-locals.var_fbbtsti_d);
        let assign34390_e43746: f64 = (assign34390_e43744 / locals.var_fmaxr);
        let assign34390_e43748: f64 = if assign34390_e43746 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard685 = assign34390_e43748;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard682 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 != 0.0)) {
            let assign34400_e43766: f64 = (-230.25850929940458);
            let assign34400_e43768: f64 = (-locals.var_fbbtsti_d);
            let assign34400_e43770: f64 = (assign34400_e43768 / locals.var_fmaxr);
            let assign34400_e43771: f64 = (assign34400_e43766 - assign34400_e43770);
            let assign34400_e43775: f64 = (-230.25850929940458);
            let assign34400_e43777: f64 = (-locals.var_fbbtsti_d);
            let assign34400_e43779: f64 = (assign34400_e43777 / locals.var_fmaxr);
            let assign34400_e43780: f64 = (assign34400_e43775 - assign34400_e43779);
            let assign34400_e43783: f64 = (-230.25850929940458);
            let assign34400_e43785: f64 = (-locals.var_fbbtsti_d);
            let assign34400_e43787: f64 = (assign34400_e43785 / locals.var_fmaxr);
            let assign34400_e43788: f64 = (assign34400_e43783 - assign34400_e43787);
            let assign34400_e43790: f64 = (assign34400_e43788 * 0.3333333333333333);
            let assign34400_e43791: f64 = (1.0 + assign34400_e43790);
            let assign34400_e43792: f64 = (assign34400_e43780 * assign34400_e43791);
            let assign34400_e43793: f64 = (0.5 * assign34400_e43792);
            let assign34400_e43794: f64 = (1.0 + assign34400_e43793);
            let assign34400_e43795: f64 = (assign34400_e43771 * assign34400_e43794);
            let assign34400_e43796: f64 = (1.0 + assign34400_e43795);
            let assign34400_e43797: f64 = (1e-100 / assign34400_e43796);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34400_e43797, (-((1e-100 * (((-(-((assign34400_e43768 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign34400_e43794) + (assign34400_e43771 * (0.5 * (((-(-((assign34400_e43777 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign34400_e43791) + (assign34400_e43780 * ((-(-((assign34400_e43785 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign34400_e43796 * assign34400_e43796))), (-((1e-100 * (((-(-((assign34400_e43768 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign34400_e43794) + (assign34400_e43771 * (0.5 * (((-(-((assign34400_e43777 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign34400_e43791) + (assign34400_e43780 * ((-(-((assign34400_e43785 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign34400_e43796 * assign34400_e43796))), (-((1e-100 * (((-(-((assign34400_e43768 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign34400_e43794) + (assign34400_e43771 * (0.5 * (((-(-((assign34400_e43777 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign34400_e43791) + (assign34400_e43780 * ((-(-((assign34400_e43785 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign34400_e43796 * assign34400_e43796))), (-((1e-100 * (((-(-((assign34400_e43768 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign34400_e43794) + (assign34400_e43771 * (0.5 * (((-(-((assign34400_e43777 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign34400_e43791) + (assign34400_e43780 * ((-(-((assign34400_e43785 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign34400_e43796 * assign34400_e43796))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard682 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) {
            let assign34410_e43818: f64 = (-locals.var_fbbtsti_d);
            let assign34410_e43820: f64 = (assign34410_e43818 / locals.var_fmaxr);
            let assign34410_e43822: f64 = (assign34410_e43820 - 230.25850929940458);
            let assign34410_e43826: f64 = (-locals.var_fbbtsti_d);
            let assign34410_e43828: f64 = (assign34410_e43826 / locals.var_fmaxr);
            let assign34410_e43830: f64 = (assign34410_e43828 - 230.25850929940458);
            let assign34410_e43833: f64 = (-locals.var_fbbtsti_d);
            let assign34410_e43835: f64 = (assign34410_e43833 / locals.var_fmaxr);
            let assign34410_e43837: f64 = (assign34410_e43835 - 230.25850929940458);
            let assign34410_e43839: f64 = (assign34410_e43837 * 0.3333333333333333);
            let assign34410_e43840: f64 = (1.0 + assign34410_e43839);
            let assign34410_e43841: f64 = (assign34410_e43830 * assign34410_e43840);
            let assign34410_e43842: f64 = (0.5 * assign34410_e43841);
            let assign34410_e43843: f64 = (1.0 + assign34410_e43842);
            let assign34410_e43844: f64 = (assign34410_e43822 * assign34410_e43843);
            let assign34410_e43845: f64 = (1.0 + assign34410_e43844);
            let assign34410_e43846: f64 = (1e100 * assign34410_e43845);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34410_e43846, (1e100 * (((-((assign34410_e43818 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign34410_e43843) + (assign34410_e43822 * (0.5 * (((-((assign34410_e43826 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign34410_e43840) + (assign34410_e43830 * ((-((assign34410_e43833 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign34410_e43818 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign34410_e43843) + (assign34410_e43822 * (0.5 * (((-((assign34410_e43826 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign34410_e43840) + (assign34410_e43830 * ((-((assign34410_e43833 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign34410_e43818 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign34410_e43843) + (assign34410_e43822 * (0.5 * (((-((assign34410_e43826 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign34410_e43840) + (assign34410_e43830 * ((-((assign34410_e43833 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign34410_e43818 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign34410_e43843) + (assign34410_e43822 * (0.5 * (((-((assign34410_e43826 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign34410_e43840) + (assign34410_e43830 * ((-((assign34410_e43833 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard682 == 0.0)) {
            let assign34420_e43861: f64 = (locals.var_v3 * locals.var_fmaxr);
            let assign34420_e43863: f64 = (assign34420_e43861 * locals.var_fmaxr);
            let assign34420_e43865: f64 = (assign34420_e43863 * locals.var_tmp);
            let assign34420_e43866: f64 = (locals.var_cbbtstid_i * assign34420_e43865);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign34420_e43866, (locals.var_cbbtstid_i * (((((locals.var_v3 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign34420_e43861 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign34420_e43863 * locals.var_tmp_dn5))), (locals.var_cbbtstid_i * (((((locals.var_v3 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign34420_e43861 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign34420_e43863 * locals.var_tmp_dn6))), (locals.var_cbbtstid_i * (((((locals.var_v3 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign34420_e43861 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign34420_e43863 * locals.var_tmp_dn7))), (locals.var_cbbtstid_i * (((((locals.var_v3 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign34420_e43861 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign34420_e43863 * locals.var_tmp_dn8))), );
        }
        let assign34430_e43871: f64 = if locals.var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard686 = assign34430_e43871;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard686 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign34450_e43885: f64 = (-locals.var_alphaav);
        let assign34450_e43887: f64 = (assign34450_e43885 * locals.var_vbrstid_i);
        let assign34450_e43888: f64 = if locals.var_vav > assign34450_e43887 { 1.0 } else { 0.0 };
        locals.var_guard687 = assign34450_e43888;
        let assign34460_e43891: f64 = if locals.var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard688 = assign34460_e43891;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) {
            let assign34470_e43907: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign34470_e43910: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign34470_e43911: f64 = (assign34470_e43907 * assign34470_e43910);
            let assign34470_e43914: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign34470_e43915: f64 = (assign34470_e43911 * assign34470_e43914);
            let assign34470_e43918: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign34470_e43919: f64 = (assign34470_e43915 * assign34470_e43918);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34470_e43919, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 == 0.0)) {
            let assign34480_e43938: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign34480_e43939: f64 = (assign34480_e43938).abs();
            let assign34480_e43941: f64 = (assign34480_e43939).powf(locals.var_pbrstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34480_e43941, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
            let assign34490_e43958: f64 = (1.0 - locals.var_tmp);
            let assign34490_e43959: f64 = (1.0 / assign34490_e43958);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign34490_e43959, (-((-locals.var_tmp_dn5) / (assign34490_e43958 * assign34490_e43958))), (-((-locals.var_tmp_dn6) / (assign34490_e43958 * assign34490_e43958))), (-((-locals.var_tmp_dn7) / (assign34490_e43958 * assign34490_e43958))), (-((-locals.var_tmp_dn8) / (assign34490_e43958 * assign34490_e43958))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
            let assign34500_e43978: f64 = (locals.var_alphaav * locals.var_vbrstid_i);
            let assign34500_e43979: f64 = (locals.var_vav + assign34500_e43978);
            let assign34500_e43981: f64 = (assign34500_e43979 * locals.var_slopesti_d);
            let assign34500_e43982: f64 = (locals.var_fstopsti_d + assign34500_e43981);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign34500_e43982, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard672 == 0.0)) {
            let assign34510_e43994: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign34510_e43996: f64 = (assign34510_e43994 + locals.var_itat);
            let assign34510_e43998: f64 = (assign34510_e43996 + locals.var_ibbt);
            let assign34510_e43999: f64 = (p.p29 * assign34510_e43998);
            let assign34510_e44001: f64 = (assign34510_e43999 * locals.var_fbreakdown);
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (assign34510_e44001, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign34510_e43999 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign34510_e43999 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign34510_e43999 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign34510_e43999 * locals.var_fbreakdown_dn8)), );
        }
        let assign34520_e44006: f64 = if locals.var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard689 = assign34520_e44006;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 != 0.0)) {
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) {
            let assign34540_e44023: f64 = (locals.var_idsatgat_d * locals.var_idmult);
            locals.var_id__blk219 = assign34540_e44023;
        }
        let assign34550_e44032: f64 = if ((locals.var_csrhgatd_i == 0.0) && (locals.var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard690 = assign34550_e44032;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) {
            let assign34570_e44055: f64 = (locals.var_vbigat_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign34570_e44055;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) {
            let assign34580_e44071: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign34580_e44072: f64 = (1.0 - assign34580_e44071);
            let assign34580_e44073: f64 = (assign34580_e44072).sqrt();
            let assign34580_e44074: f64 = (1.0 - assign34580_e44073);
            locals.var_wsrhstep = assign34580_e44074;
        }
        let assign34590_e44079: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard691 = assign34590_e44079;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) && (locals.var_guard691 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) && (locals.var_guard691 == 0.0)) {
            let assign34610_e44108: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign34610_e44110: f64 = (locals.var_wsrhstep).ln();
            let assign34610_e44111: f64 = (assign34610_e44108 * assign34610_e44110);
            let assign34610_e44114: f64 = (1.0 - locals.var_wsrhstep);
            let assign34610_e44115: f64 = (assign34610_e44111 / assign34610_e44114);
            let assign34610_e44117: f64 = (assign34610_e44115 + locals.var_wsrhstep);
            let assign34610_e44121: f64 = (2.0 * locals.var_pgatd_i);
            let assign34610_e44122: f64 = (1.0 - assign34610_e44121);
            let assign34610_e44123: f64 = (assign34610_e44117 * assign34610_e44122);
            locals.var_dwsrh = assign34610_e44123;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) {
            let assign34620_e44137: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign34620_e44137;
        }
        let assign34630_e44142: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard692 = assign34630_e44142;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) && (locals.var_guard692 != 0.0)) {
            let assign34640_e44156: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv_d);
            let assign34640_e44157: f64 = (assign34640_e44156).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34640_e44157, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) && (locals.var_guard692 == 0.0)) {
            let assign34650_e44174: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv_d);
            let assign34650_e44176: f64 = (assign34650_e44174).powf(locals.var_pgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34650_e44176, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) {
            let assign34660_e44190: f64 = (locals.var_wdepnulrgat_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign34660_e44190, (locals.var_wdepnulrgat_d * locals.var_tmp_dn5), (locals.var_wdepnulrgat_d * locals.var_tmp_dn6), (locals.var_wdepnulrgat_d * locals.var_tmp_dn7), (locals.var_wdepnulrgat_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) {
            let assign34670_e44205: f64 = (locals.var_zinv - 1.0);
            let assign34670_e44207: f64 = (assign34670_e44205 * locals.var_wdep);
            let assign34670_e44208: f64 = (locals.var_ftdgat_d * assign34670_e44207);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign34670_e44208, (locals.var_ftdgat_d * (assign34670_e44205 * locals.var_wdep_dn5)), (locals.var_ftdgat_d * (assign34670_e44205 * locals.var_wdep_dn6)), (locals.var_ftdgat_d * (assign34670_e44205 * locals.var_wdep_dn7)), (locals.var_ftdgat_d * (assign34670_e44205 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) {
            let assign34680_e44223: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign34680_e44224: f64 = (locals.var_csrhgatd_i * assign34680_e44223);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign34680_e44224, (locals.var_csrhgatd_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
    }
    pub(super) fn stamp_transient_block_41(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign34690_e44229: f64 = if locals.var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard693 = assign34690_e44229;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34710_e44253: f64 = (locals.var_wdep * locals.var_one_minus_pgat_d);
            let assign34710_e44255: f64 = (assign34710_e44253 / locals.var_vbi_minus_vjsrh);
            let assign34710_e44256: f64 = (locals.var_btatpartgat_d * assign34710_e44255);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign34710_e44256, (locals.var_btatpartgat_d * ((locals.var_wdep_dn5 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn6 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn7 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn8 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34720_e44270: f64 = (0.666666666666667 * locals.var_atatgat_d);
            let assign34720_e44272: f64 = (assign34720_e44270 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign34720_e44272, (-((assign34720_e44270 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign34720_e44270 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign34720_e44270 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign34720_e44270 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34730_e44286: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign34730_e44286, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34740_e44300: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign34740_e44303: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign34740_e44305: f64 = (assign34740_e44303 + 1.0);
            let assign34740_e44306: f64 = (assign34740_e44300 / assign34740_e44305);
            let assign34740_e44307: f64 = (assign34740_e44306).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign34740_e44307, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign34740_e44305) - (assign34740_e44300 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign34740_e44305 * assign34740_e44305)) / (2.0 * assign34740_e44307)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign34740_e44305) - (assign34740_e44300 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign34740_e44305 * assign34740_e44305)) / (2.0 * assign34740_e44307)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign34740_e44305) - (assign34740_e44300 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign34740_e44305 * assign34740_e44305)) / (2.0 * assign34740_e44307)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign34740_e44305) - (assign34740_e44300 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign34740_e44305 * assign34740_e44305)) / (2.0 * assign34740_e44307)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34750_e44320: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign34750_e44320, (locals.var_umax_dn5 / (2.0 * assign34750_e44320)), (locals.var_umax_dn6 / (2.0 * assign34750_e44320)), (locals.var_umax_dn7 / (2.0 * assign34750_e44320)), (locals.var_umax_dn8 / (2.0 * assign34750_e44320)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34760_e44334: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign34760_e44334, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign34770_e44338: f64 = (-locals.var_pgatd_i);
        let assign34770_e44340: f64 = (assign34770_e44338 * locals.var_one_over_one_minus_pgat_d);
        let assign34770_e44342: f64 = (-1.0);
        let assign34770_e44343: f64 = if assign34770_e44340 == assign34770_e44342 { 1.0 } else { 0.0 };
        locals.var_guard694 = assign34770_e44343;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard694 != 0.0)) {
            let assign34780_e44359: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign34780_e44360: f64 = (1.0 + assign34780_e44359);
            let assign34780_e44361: f64 = (1.0 / assign34780_e44360);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign34780_e44361, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign34780_e44360 * assign34780_e44360))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign34780_e44360 * assign34780_e44360))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign34780_e44360 * assign34780_e44360))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign34780_e44360 * assign34780_e44360))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard694 == 0.0)) {
            let assign34790_e44379: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign34790_e44380: f64 = (1.0 + assign34790_e44379);
            let assign34790_e44382: f64 = (-locals.var_pgatd_i);
            let assign34790_e44384: f64 = (assign34790_e44382 * locals.var_one_over_one_minus_pgat_d);
            let assign34790_e44385: f64 = (assign34790_e44380).powf(assign34790_e44384);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign34790_e44385, if 0.0 == 0.0 && ((assign34790_e44384) as f64).is_finite() && ((assign34790_e44384) as f64).fract() == 0.0 { if assign34790_e44384 == 0.0 { 0.0 } else { (assign34790_e44384 * ((assign34790_e44380).powf(assign34790_e44384 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign34790_e44385 * (assign34790_e44384 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign34790_e44380))) }, if 0.0 == 0.0 && ((assign34790_e44384) as f64).is_finite() && ((assign34790_e44384) as f64).fract() == 0.0 { if assign34790_e44384 == 0.0 { 0.0 } else { (assign34790_e44384 * ((assign34790_e44380).powf(assign34790_e44384 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign34790_e44385 * (assign34790_e44384 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign34790_e44380))) }, if 0.0 == 0.0 && ((assign34790_e44384) as f64).is_finite() && ((assign34790_e44384) as f64).fract() == 0.0 { if assign34790_e44384 == 0.0 { 0.0 } else { (assign34790_e44384 * ((assign34790_e44380).powf(assign34790_e44384 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign34790_e44385 * (assign34790_e44384 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign34790_e44380))) }, if 0.0 == 0.0 && ((assign34790_e44384) as f64).is_finite() && ((assign34790_e44384) as f64).fract() == 0.0 { if assign34790_e44384 == 0.0 { 0.0 } else { (assign34790_e44384 * ((assign34790_e44380).powf(assign34790_e44384 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign34790_e44385 * (assign34790_e44384 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign34790_e44380))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34800_e44399: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign34800_e44402: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign34800_e44403: f64 = (assign34800_e44399 / assign34800_e44402);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign34800_e44403, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign34800_e44402) - (assign34800_e44399 * locals.var_wgamma_dn5)) / (assign34800_e44402 * assign34800_e44402)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign34800_e44402) - (assign34800_e44399 * locals.var_wgamma_dn6)) / (assign34800_e44402 * assign34800_e44402)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign34800_e44402) - (assign34800_e44399 * locals.var_wgamma_dn7)) / (assign34800_e44402 * assign34800_e44402)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign34800_e44402) - (assign34800_e44399 * locals.var_wgamma_dn8)) / (assign34800_e44402 * assign34800_e44402)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34810_e44418: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign34810_e44419: f64 = (0.375 * assign34810_e44418);
            let assign34810_e44420: f64 = (assign34810_e44419).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign34810_e44420, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign34810_e44420)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign34810_e44420)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign34810_e44420)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign34810_e44420)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34820_e44435: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign34820_e44436: f64 = (2.0 * assign34820_e44435);
            let assign34820_e44438: f64 = (assign34820_e44436 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign34820_e44438, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34830_e44452: f64 = (locals.var_atatgat_d * locals.var_twoatatoverthreebtat);
            let assign34830_e44454: f64 = (assign34830_e44452 * locals.var_sqrtumax);
            let assign34830_e44457: f64 = (locals.var_atatgat_d * locals.var_umax);
            let assign34830_e44458: f64 = (assign34830_e44454 - assign34830_e44457);
            let assign34830_e44462: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign34830_e44463: f64 = (0.5 * assign34830_e44462);
            let assign34830_e44464: f64 = (assign34830_e44458 + assign34830_e44463);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign34830_e44464, (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign34830_e44452 * locals.var_sqrtumax_dn5)) - (locals.var_atatgat_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign34830_e44452 * locals.var_sqrtumax_dn6)) - (locals.var_atatgat_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign34830_e44452 * locals.var_sqrtumax_dn7)) - (locals.var_atatgat_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign34830_e44452 * locals.var_sqrtumax_dn8)) - (locals.var_atatgat_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34840_e44478: f64 = (locals.var_ltat - 1.0);
            let assign34840_e44480: f64 = (assign34840_e44478 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign34840_e44480, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign34840_e44478 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign34840_e44478 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign34840_e44478 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign34840_e44478 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34850_e44494: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign34850_e44494, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign34860_e44499: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard695 = assign34860_e44499;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard695 != 0.0)) {
            let assign34870_e44515: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign34870_e44516: f64 = (1.0 + assign34870_e44515);
            let assign34870_e44517: f64 = (1.0 / assign34870_e44516);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign34870_e44517, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign34870_e44516 * assign34870_e44516))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign34870_e44516 * assign34870_e44516))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign34870_e44516 * assign34870_e44516))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign34870_e44516 * assign34870_e44516))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard695 == 0.0)) {
            let assign34880_e44536: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign34880_e44537: f64 = (1.0 - assign34880_e44536);
            let assign34880_e44538: f64 = (1.0 / assign34880_e44537);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign34880_e44538, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign34880_e44537 * assign34880_e44537))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign34880_e44537 * assign34880_e44537))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign34880_e44537 * assign34880_e44537))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign34880_e44537 * assign34880_e44537))), );
        }
        let assign34890_e44542: f64 = (-locals.var_ysq);
        let assign34890_e44544: f64 = (assign34890_e44542 + locals.var_mtat);
        let assign34890_e44546: f64 = (-230.25850929940458);
        let assign34890_e44547: f64 = if assign34890_e44544 > assign34890_e44546 { 1.0 } else { 0.0 };
        locals.var_guard696 = assign34890_e44547;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard696 != 0.0)) {
            let assign34900_e44560: f64 = (-locals.var_ysq);
            let assign34900_e44562: f64 = (assign34900_e44560 + locals.var_mtat);
            let assign34900_e44563: f64 = (assign34900_e44562).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34900_e44563, (assign34900_e44563 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign34900_e44563 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign34900_e44563 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign34900_e44563 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard696 == 0.0)) {
            let assign34910_e44581: f64 = (-230.25850929940458);
            let assign34910_e44583: f64 = (-locals.var_ysq);
            let assign34910_e44585: f64 = (assign34910_e44583 + locals.var_mtat);
            let assign34910_e44586: f64 = (assign34910_e44581 - assign34910_e44585);
            let assign34910_e44590: f64 = (-230.25850929940458);
            let assign34910_e44592: f64 = (-locals.var_ysq);
            let assign34910_e44594: f64 = (assign34910_e44592 + locals.var_mtat);
            let assign34910_e44595: f64 = (assign34910_e44590 - assign34910_e44594);
            let assign34910_e44598: f64 = (-230.25850929940458);
            let assign34910_e44600: f64 = (-locals.var_ysq);
            let assign34910_e44602: f64 = (assign34910_e44600 + locals.var_mtat);
            let assign34910_e44603: f64 = (assign34910_e44598 - assign34910_e44602);
            let assign34910_e44605: f64 = (assign34910_e44603 * 0.3333333333333333);
            let assign34910_e44606: f64 = (1.0 + assign34910_e44605);
            let assign34910_e44607: f64 = (assign34910_e44595 * assign34910_e44606);
            let assign34910_e44608: f64 = (0.5 * assign34910_e44607);
            let assign34910_e44609: f64 = (1.0 + assign34910_e44608);
            let assign34910_e44610: f64 = (assign34910_e44586 * assign34910_e44609);
            let assign34910_e44611: f64 = (1.0 + assign34910_e44610);
            let assign34910_e44612: f64 = (1e-100 / assign34910_e44611);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34910_e44612, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign34910_e44609) + (assign34910_e44586 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign34910_e44606) + (assign34910_e44595 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign34910_e44611 * assign34910_e44611))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign34910_e44609) + (assign34910_e44586 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign34910_e44606) + (assign34910_e44595 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign34910_e44611 * assign34910_e44611))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign34910_e44609) + (assign34910_e44586 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign34910_e44606) + (assign34910_e44595 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign34910_e44611 * assign34910_e44611))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign34910_e44609) + (assign34910_e44586 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign34910_e44606) + (assign34910_e44595 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign34910_e44611 * assign34910_e44611))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34920_e44626: f64 = (0.29214664 * locals.var_terfc);
            let assign34920_e44630: f64 = (locals.var_terfc * locals.var_terfc);
            let assign34920_e44631: f64 = (locals.var_berfc * assign34920_e44630);
            let assign34920_e44632: f64 = (assign34920_e44626 + assign34920_e44631);
            let assign34920_e44636: f64 = (locals.var_terfc * locals.var_terfc);
            let assign34920_e44638: f64 = (assign34920_e44636 * locals.var_terfc);
            let assign34920_e44639: f64 = (locals.var_cerfc * assign34920_e44638);
            let assign34920_e44640: f64 = (assign34920_e44632 + assign34920_e44639);
            let assign34920_e44642: f64 = (assign34920_e44640 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign34920_e44642, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign34920_e44636 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign34920_e44640 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign34920_e44636 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign34920_e44640 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign34920_e44636 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign34920_e44640 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign34920_e44636 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign34920_e44640 * locals.var_tmp_dn8)), );
        }
        let assign34930_e44647: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard697 = assign34930_e44647;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard697 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign34950_e44664: f64 = (-230.25850929940458);
        let assign34950_e44665: f64 = if locals.var_mtat > assign34950_e44664 { 1.0 } else { 0.0 };
        locals.var_guard698 = assign34950_e44665;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard697 == 0.0)) && (locals.var_guard698 != 0.0)) {
            let assign34960_e44681: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34960_e44681, (assign34960_e44681 * locals.var_mtat_dn5), (assign34960_e44681 * locals.var_mtat_dn6), (assign34960_e44681 * locals.var_mtat_dn7), (assign34960_e44681 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard697 == 0.0)) && (locals.var_guard698 == 0.0)) {
            let assign34970_e44702: f64 = (-230.25850929940458);
            let assign34970_e44704: f64 = (assign34970_e44702 - locals.var_mtat);
            let assign34970_e44708: f64 = (-230.25850929940458);
            let assign34970_e44710: f64 = (assign34970_e44708 - locals.var_mtat);
            let assign34970_e44713: f64 = (-230.25850929940458);
            let assign34970_e44715: f64 = (assign34970_e44713 - locals.var_mtat);
            let assign34970_e44717: f64 = (assign34970_e44715 * 0.3333333333333333);
            let assign34970_e44718: f64 = (1.0 + assign34970_e44717);
            let assign34970_e44719: f64 = (assign34970_e44710 * assign34970_e44718);
            let assign34970_e44720: f64 = (0.5 * assign34970_e44719);
            let assign34970_e44721: f64 = (1.0 + assign34970_e44720);
            let assign34970_e44722: f64 = (assign34970_e44704 * assign34970_e44721);
            let assign34970_e44723: f64 = (1.0 + assign34970_e44722);
            let assign34970_e44724: f64 = (1e-100 / assign34970_e44723);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign34970_e44724, (-((1e-100 * (((-locals.var_mtat_dn5) * assign34970_e44721) + (assign34970_e44704 * (0.5 * (((-locals.var_mtat_dn5) * assign34970_e44718) + (assign34970_e44710 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign34970_e44723 * assign34970_e44723))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign34970_e44721) + (assign34970_e44704 * (0.5 * (((-locals.var_mtat_dn6) * assign34970_e44718) + (assign34970_e44710 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign34970_e44723 * assign34970_e44723))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign34970_e44721) + (assign34970_e44704 * (0.5 * (((-locals.var_mtat_dn7) * assign34970_e44718) + (assign34970_e44710 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign34970_e44723 * assign34970_e44723))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign34970_e44721) + (assign34970_e44704 * (0.5 * (((-locals.var_mtat_dn8) * assign34970_e44718) + (assign34970_e44710 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign34970_e44723 * assign34970_e44723))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard697 == 0.0)) {
            let assign34980_e44741: f64 = (2.0 * locals.var_tmp);
            let assign34980_e44743: f64 = (assign34980_e44741 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign34980_e44743, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign34990_e44757: f64 = (1.772453850905516 * 0.5);
            let assign34990_e44760: f64 = (locals.var_atatgat_d * locals.var_erfctimesexpmtat);
            let assign34990_e44762: f64 = (assign34990_e44760 / locals.var_ktat);
            let assign34990_e44763: f64 = (assign34990_e44757 * assign34990_e44762);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign34990_e44763, (assign34990_e44757 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign34990_e44760 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign34990_e44757 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign34990_e44760 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign34990_e44757 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign34990_e44760 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign34990_e44757 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign34990_e44760 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard693 == 0.0)) {
            let assign35000_e44778: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign35000_e44780: f64 = (assign35000_e44778 * locals.var_wtat);
            let assign35000_e44781: f64 = (locals.var_ctatgatd_i * assign35000_e44780);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign35000_e44781, (locals.var_ctatgatd_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign35000_e44778 * locals.var_wtat_dn5))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign35000_e44778 * locals.var_wtat_dn6))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign35000_e44778 * locals.var_wtat_dn7))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign35000_e44778 * locals.var_wtat_dn8))), );
        }
        let assign35010_e44786: f64 = if locals.var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard699 = assign35010_e44786;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard699 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign35030_e44800: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard700 = assign35030_e44800;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard700 != 0.0)) {
            let assign35040_e44814: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign35040_e44816: f64 = (assign35040_e44814 * locals.var_vbirgatinv_d);
            let assign35040_e44817: f64 = (assign35040_e44816).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35040_e44817, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard700 == 0.0)) {
            let assign35050_e44834: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign35050_e44836: f64 = (assign35050_e44834 * locals.var_vbirgatinv_d);
            let assign35050_e44838: f64 = (assign35050_e44836).powf(locals.var_pgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35050_e44838, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard699 == 0.0)) {
            let assign35060_e44853: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign35060_e44855: f64 = (assign35060_e44853 * locals.var_wdepnulrinvgat_d);
            let assign35060_e44857: f64 = (assign35060_e44855 / locals.var_tmp);
            let assign35060_e44858: f64 = (locals.var_one_over_one_minus_pgat_d * assign35060_e44857);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign35060_e44858, (locals.var_one_over_one_minus_pgat_d * (-((assign35060_e44855 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat_d * (-((assign35060_e44855 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat_d * (-((assign35060_e44855 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat_d * (-((assign35060_e44855 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign35070_e44862: f64 = (-locals.var_fbbtgat_d);
        let assign35070_e44864: f64 = (assign35070_e44862 / locals.var_fmaxr);
        let assign35070_e44865: f64 = (assign35070_e44864).abs();
        let assign35070_e44867: f64 = if assign35070_e44865 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard701 = assign35070_e44867;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard701 != 0.0)) {
            let assign35080_e44880: f64 = (-locals.var_fbbtgat_d);
            let assign35080_e44882: f64 = (assign35080_e44880 / locals.var_fmaxr);
            let assign35080_e44883: f64 = (assign35080_e44882).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35080_e44883, (assign35080_e44883 * ((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign35080_e44880 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign35080_e44883 * ((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign35080_e44880 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign35080_e44883 * ((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign35080_e44880 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign35080_e44883 * ((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign35080_e44880 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))), );
        }
        let assign35090_e44887: f64 = (-locals.var_fbbtgat_d);
        let assign35090_e44889: f64 = (assign35090_e44887 / locals.var_fmaxr);
        let assign35090_e44891: f64 = if assign35090_e44889 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard702 = assign35090_e44891;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard701 == 0.0)) && (locals.var_guard702 != 0.0)) {
            let assign35100_e44909: f64 = (-230.25850929940458);
            let assign35100_e44911: f64 = (-locals.var_fbbtgat_d);
            let assign35100_e44913: f64 = (assign35100_e44911 / locals.var_fmaxr);
            let assign35100_e44914: f64 = (assign35100_e44909 - assign35100_e44913);
            let assign35100_e44918: f64 = (-230.25850929940458);
            let assign35100_e44920: f64 = (-locals.var_fbbtgat_d);
            let assign35100_e44922: f64 = (assign35100_e44920 / locals.var_fmaxr);
            let assign35100_e44923: f64 = (assign35100_e44918 - assign35100_e44922);
            let assign35100_e44926: f64 = (-230.25850929940458);
            let assign35100_e44928: f64 = (-locals.var_fbbtgat_d);
            let assign35100_e44930: f64 = (assign35100_e44928 / locals.var_fmaxr);
            let assign35100_e44931: f64 = (assign35100_e44926 - assign35100_e44930);
            let assign35100_e44933: f64 = (assign35100_e44931 * 0.3333333333333333);
            let assign35100_e44934: f64 = (1.0 + assign35100_e44933);
            let assign35100_e44935: f64 = (assign35100_e44923 * assign35100_e44934);
            let assign35100_e44936: f64 = (0.5 * assign35100_e44935);
            let assign35100_e44937: f64 = (1.0 + assign35100_e44936);
            let assign35100_e44938: f64 = (assign35100_e44914 * assign35100_e44937);
            let assign35100_e44939: f64 = (1.0 + assign35100_e44938);
            let assign35100_e44940: f64 = (1e-100 / assign35100_e44939);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35100_e44940, (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign35100_e44911 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign35100_e44937) + (assign35100_e44914 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign35100_e44920 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign35100_e44934) + (assign35100_e44923 * ((-((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign35100_e44928 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign35100_e44939 * assign35100_e44939))), (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign35100_e44911 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign35100_e44937) + (assign35100_e44914 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign35100_e44920 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign35100_e44934) + (assign35100_e44923 * ((-((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign35100_e44928 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign35100_e44939 * assign35100_e44939))), (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign35100_e44911 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign35100_e44937) + (assign35100_e44914 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign35100_e44920 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign35100_e44934) + (assign35100_e44923 * ((-((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign35100_e44928 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign35100_e44939 * assign35100_e44939))), (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign35100_e44911 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign35100_e44937) + (assign35100_e44914 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign35100_e44920 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign35100_e44934) + (assign35100_e44923 * ((-((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign35100_e44928 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign35100_e44939 * assign35100_e44939))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard701 == 0.0)) && (locals.var_guard702 == 0.0)) {
            let assign35110_e44961: f64 = (-locals.var_fbbtgat_d);
            let assign35110_e44963: f64 = (assign35110_e44961 / locals.var_fmaxr);
            let assign35110_e44965: f64 = (assign35110_e44963 - 230.25850929940458);
            let assign35110_e44969: f64 = (-locals.var_fbbtgat_d);
            let assign35110_e44971: f64 = (assign35110_e44969 / locals.var_fmaxr);
            let assign35110_e44973: f64 = (assign35110_e44971 - 230.25850929940458);
            let assign35110_e44976: f64 = (-locals.var_fbbtgat_d);
            let assign35110_e44978: f64 = (assign35110_e44976 / locals.var_fmaxr);
            let assign35110_e44980: f64 = (assign35110_e44978 - 230.25850929940458);
            let assign35110_e44982: f64 = (assign35110_e44980 * 0.3333333333333333);
            let assign35110_e44983: f64 = (1.0 + assign35110_e44982);
            let assign35110_e44984: f64 = (assign35110_e44973 * assign35110_e44983);
            let assign35110_e44985: f64 = (0.5 * assign35110_e44984);
            let assign35110_e44986: f64 = (1.0 + assign35110_e44985);
            let assign35110_e44987: f64 = (assign35110_e44965 * assign35110_e44986);
            let assign35110_e44988: f64 = (1.0 + assign35110_e44987);
            let assign35110_e44989: f64 = (1e100 * assign35110_e44988);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35110_e44989, (1e100 * ((((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign35110_e44961 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign35110_e44986) + (assign35110_e44965 * (0.5 * ((((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign35110_e44969 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign35110_e44983) + (assign35110_e44973 * (((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign35110_e44976 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign35110_e44961 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign35110_e44986) + (assign35110_e44965 * (0.5 * ((((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign35110_e44969 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign35110_e44983) + (assign35110_e44973 * (((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign35110_e44976 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign35110_e44961 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign35110_e44986) + (assign35110_e44965 * (0.5 * ((((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign35110_e44969 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign35110_e44983) + (assign35110_e44973 * (((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign35110_e44976 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign35110_e44961 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign35110_e44986) + (assign35110_e44965 * (0.5 * ((((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign35110_e44969 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign35110_e44983) + (assign35110_e44973 * (((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign35110_e44976 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard699 == 0.0)) {
            let assign35120_e45004: f64 = (locals.var_v3 * locals.var_fmaxr);
            let assign35120_e45006: f64 = (assign35120_e45004 * locals.var_fmaxr);
            let assign35120_e45008: f64 = (assign35120_e45006 * locals.var_tmp);
            let assign35120_e45009: f64 = (locals.var_cbbtgatd_i * assign35120_e45008);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign35120_e45009, (locals.var_cbbtgatd_i * (((((locals.var_v3 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign35120_e45004 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign35120_e45006 * locals.var_tmp_dn5))), (locals.var_cbbtgatd_i * (((((locals.var_v3 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign35120_e45004 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign35120_e45006 * locals.var_tmp_dn6))), (locals.var_cbbtgatd_i * (((((locals.var_v3 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign35120_e45004 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign35120_e45006 * locals.var_tmp_dn7))), (locals.var_cbbtgatd_i * (((((locals.var_v3 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign35120_e45004 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign35120_e45006 * locals.var_tmp_dn8))), );
        }
        let assign35130_e45014: f64 = if locals.var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard703 = assign35130_e45014;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard703 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign35150_e45028: f64 = (-locals.var_alphaav);
        let assign35150_e45030: f64 = (assign35150_e45028 * locals.var_vbrgatd_i);
        let assign35150_e45031: f64 = if locals.var_vav > assign35150_e45030 { 1.0 } else { 0.0 };
        locals.var_guard704 = assign35150_e45031;
        let assign35160_e45034: f64 = if locals.var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard705 = assign35160_e45034;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard703 == 0.0)) && (locals.var_guard704 != 0.0)) && (locals.var_guard705 != 0.0)) {
            let assign35170_e45050: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign35170_e45053: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign35170_e45054: f64 = (assign35170_e45050 * assign35170_e45053);
            let assign35170_e45057: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign35170_e45058: f64 = (assign35170_e45054 * assign35170_e45057);
            let assign35170_e45061: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign35170_e45062: f64 = (assign35170_e45058 * assign35170_e45061);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35170_e45062, (((((((locals.var_vav * locals.var_vbrinvgat_d_dn5) * assign35170_e45053) + (assign35170_e45050 * (locals.var_vav * locals.var_vbrinvgat_d_dn5))) * assign35170_e45057) + (assign35170_e45054 * (locals.var_vav * locals.var_vbrinvgat_d_dn5))) * assign35170_e45061) + (assign35170_e45058 * (locals.var_vav * locals.var_vbrinvgat_d_dn5))), (((((((locals.var_vav * locals.var_vbrinvgat_d_dn6) * assign35170_e45053) + (assign35170_e45050 * (locals.var_vav * locals.var_vbrinvgat_d_dn6))) * assign35170_e45057) + (assign35170_e45054 * (locals.var_vav * locals.var_vbrinvgat_d_dn6))) * assign35170_e45061) + (assign35170_e45058 * (locals.var_vav * locals.var_vbrinvgat_d_dn6))), (((((((locals.var_vav * locals.var_vbrinvgat_d_dn7) * assign35170_e45053) + (assign35170_e45050 * (locals.var_vav * locals.var_vbrinvgat_d_dn7))) * assign35170_e45057) + (assign35170_e45054 * (locals.var_vav * locals.var_vbrinvgat_d_dn7))) * assign35170_e45061) + (assign35170_e45058 * (locals.var_vav * locals.var_vbrinvgat_d_dn7))), (((((((locals.var_vav * locals.var_vbrinvgat_d_dn8) * assign35170_e45053) + (assign35170_e45050 * (locals.var_vav * locals.var_vbrinvgat_d_dn8))) * assign35170_e45057) + (assign35170_e45054 * (locals.var_vav * locals.var_vbrinvgat_d_dn8))) * assign35170_e45061) + (assign35170_e45058 * (locals.var_vav * locals.var_vbrinvgat_d_dn8))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard703 == 0.0)) && (locals.var_guard704 != 0.0)) && (locals.var_guard705 == 0.0)) {
            let assign35180_e45081: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign35180_e45082: f64 = (assign35180_e45081).abs();
            let assign35180_e45084: f64 = (assign35180_e45082).powf(locals.var_pbrgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35180_e45084, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign35180_e45082).powf(locals.var_pbrgatd_i - 1.0) * if assign35180_e45081 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn5)) })) } } else { (assign35180_e45084 * (locals.var_pbrgatd_i * (if assign35180_e45081 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn5)) } / assign35180_e45082))) }, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign35180_e45082).powf(locals.var_pbrgatd_i - 1.0) * if assign35180_e45081 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn6)) })) } } else { (assign35180_e45084 * (locals.var_pbrgatd_i * (if assign35180_e45081 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn6)) } / assign35180_e45082))) }, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign35180_e45082).powf(locals.var_pbrgatd_i - 1.0) * if assign35180_e45081 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn7)) })) } } else { (assign35180_e45084 * (locals.var_pbrgatd_i * (if assign35180_e45081 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn7)) } / assign35180_e45082))) }, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign35180_e45082).powf(locals.var_pbrgatd_i - 1.0) * if assign35180_e45081 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn8)) })) } } else { (assign35180_e45084 * (locals.var_pbrgatd_i * (if assign35180_e45081 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn8)) } / assign35180_e45082))) }, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard703 == 0.0)) && (locals.var_guard704 != 0.0)) {
            let assign35190_e45101: f64 = (1.0 - locals.var_tmp);
            let assign35190_e45102: f64 = (1.0 / assign35190_e45101);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign35190_e45102, (-((-locals.var_tmp_dn5) / (assign35190_e45101 * assign35190_e45101))), (-((-locals.var_tmp_dn6) / (assign35190_e45101 * assign35190_e45101))), (-((-locals.var_tmp_dn7) / (assign35190_e45101 * assign35190_e45101))), (-((-locals.var_tmp_dn8) / (assign35190_e45101 * assign35190_e45101))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard703 == 0.0)) && (locals.var_guard704 == 0.0)) {
            let assign35200_e45121: f64 = (locals.var_alphaav * locals.var_vbrgatd_i);
            let assign35200_e45122: f64 = (locals.var_vav + assign35200_e45121);
            let assign35200_e45124: f64 = (assign35200_e45122 * locals.var_slopegat_d);
            let assign35200_e45125: f64 = (locals.var_fstopgat_d + assign35200_e45124);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign35200_e45125, (assign35200_e45122 * locals.var_slopegat_d_dn5), (assign35200_e45122 * locals.var_slopegat_d_dn6), (assign35200_e45122 * locals.var_slopegat_d_dn7), (assign35200_e45122 * locals.var_slopegat_d_dn8), );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard689 == 0.0)) {
            let assign35210_e45137: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign35210_e45139: f64 = (assign35210_e45137 + locals.var_itat);
            let assign35210_e45141: f64 = (assign35210_e45139 + locals.var_ibbt);
            let assign35210_e45142: f64 = (p.p29 * assign35210_e45141);
            let assign35210_e45144: f64 = (assign35210_e45142 * locals.var_fbreakdown);
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (assign35210_e45144, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign35210_e45142 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign35210_e45142 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign35210_e45142 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign35210_e45142 * locals.var_fbreakdown_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign35220_e45152: f64 = (locals.var_abdrain_i * locals.var_ijunbot);
            let assign35220_e45155: f64 = (locals.var_lsdrain_i * locals.var_ijunsti);
            let assign35220_e45156: f64 = (assign35220_e45152 + assign35220_e45155);
            let assign35220_e45159: f64 = (locals.var_lgdrain_i * locals.var_ijungat);
            let assign35220_e45160: f64 = (assign35220_e45156 + assign35220_e45159);
            (locals.var_i3, locals.var_i3_dn5, locals.var_i3_dn6, locals.var_i3_dn7, locals.var_i3_dn8, ) = (assign35220_e45160, (((locals.var_abdrain_i * locals.var_ijunbot_dn5) + (locals.var_lsdrain_i * locals.var_ijunsti_dn5)) + (locals.var_lgdrain_i * locals.var_ijungat_dn5)), (((locals.var_abdrain_i * locals.var_ijunbot_dn6) + (locals.var_lsdrain_i * locals.var_ijunsti_dn6)) + (locals.var_lgdrain_i * locals.var_ijungat_dn6)), (((locals.var_abdrain_i * locals.var_ijunbot_dn7) + (locals.var_lsdrain_i * locals.var_ijunsti_dn7)) + (locals.var_lgdrain_i * locals.var_ijungat_dn7)), (((locals.var_abdrain_i * locals.var_ijunbot_dn8) + (locals.var_lsdrain_i * locals.var_ijunsti_dn8)) + (locals.var_lgdrain_i * locals.var_ijungat_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            locals.var_vbbt = 0.0;
            locals.var_two_psistar = 0.0;
        }
        let assign35250_e45186: f64 = if (!(((locals.var_abdrain_i == 0.0) && (locals.var_lsdrain_i == 0.0)) && (locals.var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard706 = assign35250_e45186;
        let assign35330_e45272: f64 = if locals.var_v4 < locals.var_vmax_d { 1.0 } else { 0.0 };
        locals.var_guard707 = assign35330_e45272;
        let assign35340_e45274: f64 = (-0.5);
        let assign35340_e45277: f64 = (locals.var_v4 * locals.var_phitdinv);
        let assign35340_e45278: f64 = (assign35340_e45274 * assign35340_e45277);
        let assign35340_e45279: f64 = (assign35340_e45278).abs();
        let assign35340_e45281: f64 = if assign35340_e45279 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard708 = assign35340_e45281;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) && (locals.var_guard707 != 0.0)) && (locals.var_guard708 != 0.0)) {
            let assign35350_e45292: f64 = (-0.5);
            let assign35350_e45295: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign35350_e45296: f64 = (assign35350_e45292 * assign35350_e45295);
            let assign35350_e45297: f64 = (assign35350_e45296).exp();
            locals.var_z = assign35350_e45297;
        }
        let assign35360_e45301: f64 = (-0.5);
        let assign35360_e45304: f64 = (locals.var_v4 * locals.var_phitdinv);
        let assign35360_e45305: f64 = (assign35360_e45301 * assign35360_e45304);
        let assign35360_e45307: f64 = if assign35360_e45305 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard709 = assign35360_e45307;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) && (locals.var_guard707 != 0.0)) && (locals.var_guard708 == 0.0)) && (locals.var_guard709 != 0.0)) {
            let assign35370_e45323: f64 = (-230.25850929940458);
            let assign35370_e45325: f64 = (-0.5);
            let assign35370_e45328: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign35370_e45329: f64 = (assign35370_e45325 * assign35370_e45328);
            let assign35370_e45330: f64 = (assign35370_e45323 - assign35370_e45329);
            let assign35370_e45334: f64 = (-230.25850929940458);
            let assign35370_e45336: f64 = (-0.5);
            let assign35370_e45339: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign35370_e45340: f64 = (assign35370_e45336 * assign35370_e45339);
            let assign35370_e45341: f64 = (assign35370_e45334 - assign35370_e45340);
            let assign35370_e45344: f64 = (-230.25850929940458);
            let assign35370_e45346: f64 = (-0.5);
            let assign35370_e45349: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign35370_e45350: f64 = (assign35370_e45346 * assign35370_e45349);
            let assign35370_e45351: f64 = (assign35370_e45344 - assign35370_e45350);
            let assign35370_e45353: f64 = (assign35370_e45351 * 0.3333333333333333);
            let assign35370_e45354: f64 = (1.0 + assign35370_e45353);
            let assign35370_e45355: f64 = (assign35370_e45341 * assign35370_e45354);
            let assign35370_e45356: f64 = (0.5 * assign35370_e45355);
            let assign35370_e45357: f64 = (1.0 + assign35370_e45356);
            let assign35370_e45358: f64 = (assign35370_e45330 * assign35370_e45357);
            let assign35370_e45359: f64 = (1.0 + assign35370_e45358);
            let assign35370_e45360: f64 = (1e-100 / assign35370_e45359);
            locals.var_z = assign35370_e45360;
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) && (locals.var_guard707 != 0.0)) && (locals.var_guard708 == 0.0)) && (locals.var_guard709 == 0.0)) {
            let assign35380_e45379: f64 = (-0.5);
            let assign35380_e45382: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign35380_e45383: f64 = (assign35380_e45379 * assign35380_e45382);
            let assign35380_e45385: f64 = (assign35380_e45383 - 230.25850929940458);
            let assign35380_e45389: f64 = (-0.5);
            let assign35380_e45392: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign35380_e45393: f64 = (assign35380_e45389 * assign35380_e45392);
            let assign35380_e45395: f64 = (assign35380_e45393 - 230.25850929940458);
            let assign35380_e45398: f64 = (-0.5);
            let assign35380_e45401: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign35380_e45402: f64 = (assign35380_e45398 * assign35380_e45401);
            let assign35380_e45404: f64 = (assign35380_e45402 - 230.25850929940458);
            let assign35380_e45406: f64 = (assign35380_e45404 * 0.3333333333333333);
            let assign35380_e45407: f64 = (1.0 + assign35380_e45406);
            let assign35380_e45408: f64 = (assign35380_e45395 * assign35380_e45407);
            let assign35380_e45409: f64 = (0.5 * assign35380_e45408);
            let assign35380_e45410: f64 = (1.0 + assign35380_e45409);
            let assign35380_e45411: f64 = (assign35380_e45385 * assign35380_e45410);
            let assign35380_e45412: f64 = (1.0 + assign35380_e45411);
            let assign35380_e45413: f64 = (1e100 * assign35380_e45412);
            locals.var_z = assign35380_e45413;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) && (locals.var_guard707 != 0.0)) {
            let assign35390_e45425: f64 = (1.0 / locals.var_z);
            locals.var_zinv = assign35390_e45425;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) && (locals.var_guard707 != 0.0)) {
            let assign35400_e45437: f64 = (locals.var_zinv * locals.var_zinv);
            locals.var_idmult = assign35400_e45437;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) && (locals.var_guard707 == 0.0)) {
            let assign35410_e45451: f64 = (locals.var_v4 - locals.var_vmax_d);
            let assign35410_e45453: f64 = (assign35410_e45451 * locals.var_phitdinv);
            let assign35410_e45454: f64 = (1.0 + assign35410_e45453);
            let assign35410_e45456: f64 = (assign35410_e45454 * locals.var_exp_vmax_over_phitd_d);
            locals.var_idmult = assign35410_e45456;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) && (locals.var_guard707 == 0.0)) {
            let assign35420_e45468: f64 = (locals.var_idmult).sqrt();
            locals.var_zinv = assign35420_e45468;
        }
    }
    pub(super) fn stamp_transient_block_42(
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) && (locals.var_guard707 == 0.0)) {
            let assign35430_e45481: f64 = (1.0 / locals.var_zinv);
            locals.var_z = assign35430_e45481;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) {
            let assign35440_e45491: f64 = (locals.var_idmult - 1.0);
            locals.var_idmult = assign35440_e45491;
        }
        let assign35450_e45496: f64 = if locals.var_v4 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard710 = assign35450_e45496;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) && (locals.var_guard710 != 0.0)) {
            let assign35460_e45508: f64 = (2.0 + locals.var_z);
            let assign35460_e45511: f64 = (locals.var_z + 1.0);
            let assign35460_e45514: f64 = (locals.var_z + 3.0);
            let assign35460_e45515: f64 = (assign35460_e45511 * assign35460_e45514);
            let assign35460_e45516: f64 = (assign35460_e45515).sqrt();
            let assign35460_e45517: f64 = (assign35460_e45508 + assign35460_e45516);
            let assign35460_e45518: f64 = (assign35460_e45517).ln();
            let assign35460_e45519: f64 = (locals.var_phitd * assign35460_e45518);
            let assign35460_e45520: f64 = (2.0 * assign35460_e45519);
            locals.var_two_psistar = assign35460_e45520;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) && (locals.var_guard710 == 0.0)) {
            let assign35470_e45532: f64 = (-locals.var_v4);
            let assign35470_e45537: f64 = (2.0 * locals.var_zinv);
            let assign35470_e45539: f64 = (assign35470_e45537 + 1.0);
            let assign35470_e45542: f64 = (1.0 + locals.var_zinv);
            let assign35470_e45546: f64 = (3.0 * locals.var_zinv);
            let assign35470_e45547: f64 = (1.0 + assign35470_e45546);
            let assign35470_e45548: f64 = (assign35470_e45542 * assign35470_e45547);
            let assign35470_e45549: f64 = (assign35470_e45548).sqrt();
            let assign35470_e45550: f64 = (assign35470_e45539 + assign35470_e45549);
            let assign35470_e45551: f64 = (assign35470_e45550).ln();
            let assign35470_e45552: f64 = (locals.var_phitd * assign35470_e45551);
            let assign35470_e45553: f64 = (2.0 * assign35470_e45552);
            let assign35470_e45554: f64 = (assign35470_e45532 + assign35470_e45553);
            locals.var_two_psistar = assign35470_e45554;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) {
            let assign35480_e45564: f64 = (locals.var_vbimin_d - locals.var_two_psistar);
            locals.var_vjlim = assign35480_e45564;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) {
            let assign35490_e45575: f64 = (locals.var_v4 + locals.var_vjlim);
            let assign35490_e45578: f64 = (locals.var_v4 - locals.var_vjlim);
            let assign35490_e45581: f64 = (locals.var_v4 - locals.var_vjlim);
            let assign35490_e45582: f64 = (assign35490_e45578 * assign35490_e45581);
            let assign35490_e45585: f64 = (4.0 * locals.var_phitd);
            let assign35490_e45587: f64 = (assign35490_e45585 * locals.var_phitd);
            let assign35490_e45588: f64 = (assign35490_e45582 + assign35490_e45587);
            let assign35490_e45589: f64 = (assign35490_e45588).sqrt();
            let assign35490_e45590: f64 = (assign35490_e45575 - assign35490_e45589);
            let assign35490_e45591: f64 = (0.5 * assign35490_e45590);
            locals.var_vjsrh = assign35490_e45591;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) {
            let assign35500_e45602: f64 = (locals.var_v4 + locals.var_vbbtlim_d);
            let assign35500_e45605: f64 = (locals.var_v4 - locals.var_vbbtlim_d);
            let assign35500_e45608: f64 = (locals.var_v4 - locals.var_vbbtlim_d);
            let assign35500_e45609: f64 = (assign35500_e45605 * assign35500_e45608);
            let assign35500_e45612: f64 = (4.0 * locals.var_phitr);
            let assign35500_e45614: f64 = (assign35500_e45612 * locals.var_phitr);
            let assign35500_e45615: f64 = (assign35500_e45609 + assign35500_e45614);
            let assign35500_e45616: f64 = (assign35500_e45615).sqrt();
            let assign35500_e45617: f64 = (assign35500_e45602 - assign35500_e45616);
            let assign35500_e45618: f64 = (0.5 * assign35500_e45617);
            locals.var_vbbt = assign35500_e45618;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard706 != 0.0)) {
            let assign35510_e45629: f64 = locals.var_v4;
            let assign35510_e45632: f64 = locals.var_v4;
            let assign35510_e45635: f64 = locals.var_v4;
            let assign35510_e45636: f64 = (assign35510_e45632 * assign35510_e45635);
            let assign35510_e45639: f64 = (4.0 * 1e-6);
            let assign35510_e45641: f64 = (assign35510_e45639 * 1e-6);
            let assign35510_e45642: f64 = (assign35510_e45636 + assign35510_e45641);
            let assign35510_e45643: f64 = (assign35510_e45642).sqrt();
            let assign35510_e45644: f64 = (assign35510_e45629 - assign35510_e45643);
            let assign35510_e45645: f64 = (0.5 * assign35510_e45644);
            locals.var_vav = assign35510_e45645;
        }
        let assign35520_e45650: f64 = if locals.var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard711 = assign35520_e45650;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 != 0.0)) {
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) {
            let assign35540_e45667: f64 = (locals.var_idsatbot_d * locals.var_idmult);
            locals.var_id__blk219 = assign35540_e45667;
        }
        let assign35550_e45676: f64 = if ((locals.var_csrhbotd_i == 0.0) && (locals.var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard712 = assign35550_e45676;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 == 0.0)) {
            let assign35570_e45699: f64 = (locals.var_vbibot_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign35570_e45699;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 == 0.0)) {
            let assign35580_e45715: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign35580_e45716: f64 = (1.0 - assign35580_e45715);
            let assign35580_e45717: f64 = (assign35580_e45716).sqrt();
            let assign35580_e45718: f64 = (1.0 - assign35580_e45717);
            locals.var_wsrhstep = assign35580_e45718;
        }
        let assign35590_e45723: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard713 = assign35590_e45723;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 == 0.0)) && (locals.var_guard713 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 == 0.0)) && (locals.var_guard713 == 0.0)) {
            let assign35610_e45752: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign35610_e45754: f64 = (locals.var_wsrhstep).ln();
            let assign35610_e45755: f64 = (assign35610_e45752 * assign35610_e45754);
            let assign35610_e45758: f64 = (1.0 - locals.var_wsrhstep);
            let assign35610_e45759: f64 = (assign35610_e45755 / assign35610_e45758);
            let assign35610_e45761: f64 = (assign35610_e45759 + locals.var_wsrhstep);
            let assign35610_e45765: f64 = (2.0 * locals.var_pbotd_i);
            let assign35610_e45766: f64 = (1.0 - assign35610_e45765);
            let assign35610_e45767: f64 = (assign35610_e45761 * assign35610_e45766);
            locals.var_dwsrh = assign35610_e45767;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 == 0.0)) {
            let assign35620_e45781: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign35620_e45781;
        }
        let assign35630_e45786: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard714 = assign35630_e45786;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 == 0.0)) && (locals.var_guard714 != 0.0)) {
            let assign35640_e45800: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv_d);
            let assign35640_e45801: f64 = (assign35640_e45800).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35640_e45801, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 == 0.0)) && (locals.var_guard714 == 0.0)) {
            let assign35650_e45818: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv_d);
            let assign35650_e45820: f64 = (assign35650_e45818).powf(locals.var_pbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35650_e45820, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 == 0.0)) {
            let assign35660_e45834: f64 = (locals.var_wdepnulrbot_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign35660_e45834, (locals.var_wdepnulrbot_d * locals.var_tmp_dn5), (locals.var_wdepnulrbot_d * locals.var_tmp_dn6), (locals.var_wdepnulrbot_d * locals.var_tmp_dn7), (locals.var_wdepnulrbot_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 == 0.0)) {
            let assign35670_e45849: f64 = (locals.var_zinv - 1.0);
            let assign35670_e45851: f64 = (assign35670_e45849 * locals.var_wdep);
            let assign35670_e45852: f64 = (locals.var_ftdbot_d * assign35670_e45851);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign35670_e45852, (locals.var_ftdbot_d * (assign35670_e45849 * locals.var_wdep_dn5)), (locals.var_ftdbot_d * (assign35670_e45849 * locals.var_wdep_dn6)), (locals.var_ftdbot_d * (assign35670_e45849 * locals.var_wdep_dn7)), (locals.var_ftdbot_d * (assign35670_e45849 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 == 0.0)) {
            let assign35680_e45867: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign35680_e45868: f64 = (locals.var_csrhbotd_i * assign35680_e45867);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign35680_e45868, (locals.var_csrhbotd_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign35690_e45873: f64 = if locals.var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard715 = assign35690_e45873;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35710_e45897: f64 = (locals.var_wdep * locals.var_one_minus_pbot_d);
            let assign35710_e45899: f64 = (assign35710_e45897 / locals.var_vbi_minus_vjsrh);
            let assign35710_e45900: f64 = (locals.var_btatpartbot_d * assign35710_e45899);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign35710_e45900, (locals.var_btatpartbot_d * ((locals.var_wdep_dn5 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn6 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn7 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn8 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35720_e45914: f64 = (0.666666666666667 * locals.var_atatbot_d);
            let assign35720_e45916: f64 = (assign35720_e45914 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign35720_e45916, (-((assign35720_e45914 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign35720_e45914 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign35720_e45914 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign35720_e45914 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35730_e45930: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign35730_e45930, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35740_e45944: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign35740_e45947: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign35740_e45949: f64 = (assign35740_e45947 + 1.0);
            let assign35740_e45950: f64 = (assign35740_e45944 / assign35740_e45949);
            let assign35740_e45951: f64 = (assign35740_e45950).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign35740_e45951, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign35740_e45949) - (assign35740_e45944 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign35740_e45949 * assign35740_e45949)) / (2.0 * assign35740_e45951)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign35740_e45949) - (assign35740_e45944 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign35740_e45949 * assign35740_e45949)) / (2.0 * assign35740_e45951)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign35740_e45949) - (assign35740_e45944 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign35740_e45949 * assign35740_e45949)) / (2.0 * assign35740_e45951)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign35740_e45949) - (assign35740_e45944 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign35740_e45949 * assign35740_e45949)) / (2.0 * assign35740_e45951)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35750_e45964: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign35750_e45964, (locals.var_umax_dn5 / (2.0 * assign35750_e45964)), (locals.var_umax_dn6 / (2.0 * assign35750_e45964)), (locals.var_umax_dn7 / (2.0 * assign35750_e45964)), (locals.var_umax_dn8 / (2.0 * assign35750_e45964)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35760_e45978: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign35760_e45978, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign35770_e45982: f64 = (-locals.var_pbotd_i);
        let assign35770_e45984: f64 = (assign35770_e45982 * locals.var_one_over_one_minus_pbot_d);
        let assign35770_e45986: f64 = (-1.0);
        let assign35770_e45987: f64 = if assign35770_e45984 == assign35770_e45986 { 1.0 } else { 0.0 };
        locals.var_guard716 = assign35770_e45987;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
            let assign35780_e46003: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign35780_e46004: f64 = (1.0 + assign35780_e46003);
            let assign35780_e46005: f64 = (1.0 / assign35780_e46004);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign35780_e46005, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign35780_e46004 * assign35780_e46004))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign35780_e46004 * assign35780_e46004))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign35780_e46004 * assign35780_e46004))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign35780_e46004 * assign35780_e46004))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) {
            let assign35790_e46023: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign35790_e46024: f64 = (1.0 + assign35790_e46023);
            let assign35790_e46026: f64 = (-locals.var_pbotd_i);
            let assign35790_e46028: f64 = (assign35790_e46026 * locals.var_one_over_one_minus_pbot_d);
            let assign35790_e46029: f64 = (assign35790_e46024).powf(assign35790_e46028);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign35790_e46029, if 0.0 == 0.0 && ((assign35790_e46028) as f64).is_finite() && ((assign35790_e46028) as f64).fract() == 0.0 { if assign35790_e46028 == 0.0 { 0.0 } else { (assign35790_e46028 * ((assign35790_e46024).powf(assign35790_e46028 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign35790_e46029 * (assign35790_e46028 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign35790_e46024))) }, if 0.0 == 0.0 && ((assign35790_e46028) as f64).is_finite() && ((assign35790_e46028) as f64).fract() == 0.0 { if assign35790_e46028 == 0.0 { 0.0 } else { (assign35790_e46028 * ((assign35790_e46024).powf(assign35790_e46028 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign35790_e46029 * (assign35790_e46028 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign35790_e46024))) }, if 0.0 == 0.0 && ((assign35790_e46028) as f64).is_finite() && ((assign35790_e46028) as f64).fract() == 0.0 { if assign35790_e46028 == 0.0 { 0.0 } else { (assign35790_e46028 * ((assign35790_e46024).powf(assign35790_e46028 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign35790_e46029 * (assign35790_e46028 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign35790_e46024))) }, if 0.0 == 0.0 && ((assign35790_e46028) as f64).is_finite() && ((assign35790_e46028) as f64).fract() == 0.0 { if assign35790_e46028 == 0.0 { 0.0 } else { (assign35790_e46028 * ((assign35790_e46024).powf(assign35790_e46028 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign35790_e46029 * (assign35790_e46028 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign35790_e46024))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35800_e46043: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign35800_e46046: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign35800_e46047: f64 = (assign35800_e46043 / assign35800_e46046);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign35800_e46047, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign35800_e46046) - (assign35800_e46043 * locals.var_wgamma_dn5)) / (assign35800_e46046 * assign35800_e46046)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign35800_e46046) - (assign35800_e46043 * locals.var_wgamma_dn6)) / (assign35800_e46046 * assign35800_e46046)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign35800_e46046) - (assign35800_e46043 * locals.var_wgamma_dn7)) / (assign35800_e46046 * assign35800_e46046)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign35800_e46046) - (assign35800_e46043 * locals.var_wgamma_dn8)) / (assign35800_e46046 * assign35800_e46046)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35810_e46062: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign35810_e46063: f64 = (0.375 * assign35810_e46062);
            let assign35810_e46064: f64 = (assign35810_e46063).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign35810_e46064, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign35810_e46064)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign35810_e46064)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign35810_e46064)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign35810_e46064)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35820_e46079: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign35820_e46080: f64 = (2.0 * assign35820_e46079);
            let assign35820_e46082: f64 = (assign35820_e46080 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign35820_e46082, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35830_e46096: f64 = (locals.var_atatbot_d * locals.var_twoatatoverthreebtat);
            let assign35830_e46098: f64 = (assign35830_e46096 * locals.var_sqrtumax);
            let assign35830_e46101: f64 = (locals.var_atatbot_d * locals.var_umax);
            let assign35830_e46102: f64 = (assign35830_e46098 - assign35830_e46101);
            let assign35830_e46106: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign35830_e46107: f64 = (0.5 * assign35830_e46106);
            let assign35830_e46108: f64 = (assign35830_e46102 + assign35830_e46107);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign35830_e46108, (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign35830_e46096 * locals.var_sqrtumax_dn5)) - (locals.var_atatbot_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign35830_e46096 * locals.var_sqrtumax_dn6)) - (locals.var_atatbot_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign35830_e46096 * locals.var_sqrtumax_dn7)) - (locals.var_atatbot_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign35830_e46096 * locals.var_sqrtumax_dn8)) - (locals.var_atatbot_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35840_e46122: f64 = (locals.var_ltat - 1.0);
            let assign35840_e46124: f64 = (assign35840_e46122 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign35840_e46124, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign35840_e46122 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign35840_e46122 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign35840_e46122 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign35840_e46122 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35850_e46138: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign35850_e46138, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign35860_e46143: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard717 = assign35860_e46143;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard717 != 0.0)) {
            let assign35870_e46159: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign35870_e46160: f64 = (1.0 + assign35870_e46159);
            let assign35870_e46161: f64 = (1.0 / assign35870_e46160);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign35870_e46161, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign35870_e46160 * assign35870_e46160))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign35870_e46160 * assign35870_e46160))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign35870_e46160 * assign35870_e46160))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign35870_e46160 * assign35870_e46160))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard717 == 0.0)) {
            let assign35880_e46180: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign35880_e46181: f64 = (1.0 - assign35880_e46180);
            let assign35880_e46182: f64 = (1.0 / assign35880_e46181);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign35880_e46182, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign35880_e46181 * assign35880_e46181))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign35880_e46181 * assign35880_e46181))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign35880_e46181 * assign35880_e46181))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign35880_e46181 * assign35880_e46181))), );
        }
        let assign35890_e46186: f64 = (-locals.var_ysq);
        let assign35890_e46188: f64 = (assign35890_e46186 + locals.var_mtat);
        let assign35890_e46190: f64 = (-230.25850929940458);
        let assign35890_e46191: f64 = if assign35890_e46188 > assign35890_e46190 { 1.0 } else { 0.0 };
        locals.var_guard718 = assign35890_e46191;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard718 != 0.0)) {
            let assign35900_e46204: f64 = (-locals.var_ysq);
            let assign35900_e46206: f64 = (assign35900_e46204 + locals.var_mtat);
            let assign35900_e46207: f64 = (assign35900_e46206).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35900_e46207, (assign35900_e46207 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign35900_e46207 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign35900_e46207 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign35900_e46207 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard718 == 0.0)) {
            let assign35910_e46225: f64 = (-230.25850929940458);
            let assign35910_e46227: f64 = (-locals.var_ysq);
            let assign35910_e46229: f64 = (assign35910_e46227 + locals.var_mtat);
            let assign35910_e46230: f64 = (assign35910_e46225 - assign35910_e46229);
            let assign35910_e46234: f64 = (-230.25850929940458);
            let assign35910_e46236: f64 = (-locals.var_ysq);
            let assign35910_e46238: f64 = (assign35910_e46236 + locals.var_mtat);
            let assign35910_e46239: f64 = (assign35910_e46234 - assign35910_e46238);
            let assign35910_e46242: f64 = (-230.25850929940458);
            let assign35910_e46244: f64 = (-locals.var_ysq);
            let assign35910_e46246: f64 = (assign35910_e46244 + locals.var_mtat);
            let assign35910_e46247: f64 = (assign35910_e46242 - assign35910_e46246);
            let assign35910_e46249: f64 = (assign35910_e46247 * 0.3333333333333333);
            let assign35910_e46250: f64 = (1.0 + assign35910_e46249);
            let assign35910_e46251: f64 = (assign35910_e46239 * assign35910_e46250);
            let assign35910_e46252: f64 = (0.5 * assign35910_e46251);
            let assign35910_e46253: f64 = (1.0 + assign35910_e46252);
            let assign35910_e46254: f64 = (assign35910_e46230 * assign35910_e46253);
            let assign35910_e46255: f64 = (1.0 + assign35910_e46254);
            let assign35910_e46256: f64 = (1e-100 / assign35910_e46255);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35910_e46256, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign35910_e46253) + (assign35910_e46230 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign35910_e46250) + (assign35910_e46239 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign35910_e46255 * assign35910_e46255))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign35910_e46253) + (assign35910_e46230 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign35910_e46250) + (assign35910_e46239 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign35910_e46255 * assign35910_e46255))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign35910_e46253) + (assign35910_e46230 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign35910_e46250) + (assign35910_e46239 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign35910_e46255 * assign35910_e46255))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign35910_e46253) + (assign35910_e46230 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign35910_e46250) + (assign35910_e46239 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign35910_e46255 * assign35910_e46255))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35920_e46270: f64 = (0.29214664 * locals.var_terfc);
            let assign35920_e46274: f64 = (locals.var_terfc * locals.var_terfc);
            let assign35920_e46275: f64 = (locals.var_berfc * assign35920_e46274);
            let assign35920_e46276: f64 = (assign35920_e46270 + assign35920_e46275);
            let assign35920_e46280: f64 = (locals.var_terfc * locals.var_terfc);
            let assign35920_e46282: f64 = (assign35920_e46280 * locals.var_terfc);
            let assign35920_e46283: f64 = (locals.var_cerfc * assign35920_e46282);
            let assign35920_e46284: f64 = (assign35920_e46276 + assign35920_e46283);
            let assign35920_e46286: f64 = (assign35920_e46284 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign35920_e46286, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign35920_e46280 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign35920_e46284 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign35920_e46280 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign35920_e46284 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign35920_e46280 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign35920_e46284 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign35920_e46280 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign35920_e46284 * locals.var_tmp_dn8)), );
        }
        let assign35930_e46291: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard719 = assign35930_e46291;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard719 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign35950_e46308: f64 = (-230.25850929940458);
        let assign35950_e46309: f64 = if locals.var_mtat > assign35950_e46308 { 1.0 } else { 0.0 };
        locals.var_guard720 = assign35950_e46309;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard719 == 0.0)) && (locals.var_guard720 != 0.0)) {
            let assign35960_e46325: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35960_e46325, (assign35960_e46325 * locals.var_mtat_dn5), (assign35960_e46325 * locals.var_mtat_dn6), (assign35960_e46325 * locals.var_mtat_dn7), (assign35960_e46325 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard719 == 0.0)) && (locals.var_guard720 == 0.0)) {
            let assign35970_e46346: f64 = (-230.25850929940458);
            let assign35970_e46348: f64 = (assign35970_e46346 - locals.var_mtat);
            let assign35970_e46352: f64 = (-230.25850929940458);
            let assign35970_e46354: f64 = (assign35970_e46352 - locals.var_mtat);
            let assign35970_e46357: f64 = (-230.25850929940458);
            let assign35970_e46359: f64 = (assign35970_e46357 - locals.var_mtat);
            let assign35970_e46361: f64 = (assign35970_e46359 * 0.3333333333333333);
            let assign35970_e46362: f64 = (1.0 + assign35970_e46361);
            let assign35970_e46363: f64 = (assign35970_e46354 * assign35970_e46362);
            let assign35970_e46364: f64 = (0.5 * assign35970_e46363);
            let assign35970_e46365: f64 = (1.0 + assign35970_e46364);
            let assign35970_e46366: f64 = (assign35970_e46348 * assign35970_e46365);
            let assign35970_e46367: f64 = (1.0 + assign35970_e46366);
            let assign35970_e46368: f64 = (1e-100 / assign35970_e46367);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign35970_e46368, (-((1e-100 * (((-locals.var_mtat_dn5) * assign35970_e46365) + (assign35970_e46348 * (0.5 * (((-locals.var_mtat_dn5) * assign35970_e46362) + (assign35970_e46354 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign35970_e46367 * assign35970_e46367))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign35970_e46365) + (assign35970_e46348 * (0.5 * (((-locals.var_mtat_dn6) * assign35970_e46362) + (assign35970_e46354 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign35970_e46367 * assign35970_e46367))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign35970_e46365) + (assign35970_e46348 * (0.5 * (((-locals.var_mtat_dn7) * assign35970_e46362) + (assign35970_e46354 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign35970_e46367 * assign35970_e46367))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign35970_e46365) + (assign35970_e46348 * (0.5 * (((-locals.var_mtat_dn8) * assign35970_e46362) + (assign35970_e46354 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign35970_e46367 * assign35970_e46367))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard719 == 0.0)) {
            let assign35980_e46385: f64 = (2.0 * locals.var_tmp);
            let assign35980_e46387: f64 = (assign35980_e46385 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign35980_e46387, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign35990_e46401: f64 = (1.772453850905516 * 0.5);
            let assign35990_e46404: f64 = (locals.var_atatbot_d * locals.var_erfctimesexpmtat);
            let assign35990_e46406: f64 = (assign35990_e46404 / locals.var_ktat);
            let assign35990_e46407: f64 = (assign35990_e46401 * assign35990_e46406);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign35990_e46407, (assign35990_e46401 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign35990_e46404 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign35990_e46401 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign35990_e46404 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign35990_e46401 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign35990_e46404 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign35990_e46401 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign35990_e46404 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard715 == 0.0)) {
            let assign36000_e46422: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign36000_e46424: f64 = (assign36000_e46422 * locals.var_wtat);
            let assign36000_e46425: f64 = (locals.var_ctatbotd_i * assign36000_e46424);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign36000_e46425, (locals.var_ctatbotd_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign36000_e46422 * locals.var_wtat_dn5))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign36000_e46422 * locals.var_wtat_dn6))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign36000_e46422 * locals.var_wtat_dn7))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign36000_e46422 * locals.var_wtat_dn8))), );
        }
        let assign36010_e46430: f64 = if locals.var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard721 = assign36010_e46430;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard721 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign36030_e46444: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard722 = assign36030_e46444;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard721 == 0.0)) && (locals.var_guard722 != 0.0)) {
            let assign36040_e46458: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign36040_e46460: f64 = (assign36040_e46458 * locals.var_vbirbotinv_d);
            let assign36040_e46461: f64 = (assign36040_e46460).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36040_e46461, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard721 == 0.0)) && (locals.var_guard722 == 0.0)) {
            let assign36050_e46478: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign36050_e46480: f64 = (assign36050_e46478 * locals.var_vbirbotinv_d);
            let assign36050_e46482: f64 = (assign36050_e46480).powf(locals.var_pbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36050_e46482, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard721 == 0.0)) {
            let assign36060_e46497: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign36060_e46499: f64 = (assign36060_e46497 * locals.var_wdepnulrinvbot_d);
            let assign36060_e46501: f64 = (assign36060_e46499 / locals.var_tmp);
            let assign36060_e46502: f64 = (locals.var_one_over_one_minus_pbot_d * assign36060_e46501);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign36060_e46502, (locals.var_one_over_one_minus_pbot_d * (-((assign36060_e46499 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign36060_e46499 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign36060_e46499 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign36060_e46499 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign36070_e46506: f64 = (-locals.var_fbbtbot_d);
        let assign36070_e46508: f64 = (assign36070_e46506 / locals.var_fmaxr);
        let assign36070_e46509: f64 = (assign36070_e46508).abs();
        let assign36070_e46511: f64 = if assign36070_e46509 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard723 = assign36070_e46511;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard721 == 0.0)) && (locals.var_guard723 != 0.0)) {
            let assign36080_e46524: f64 = (-locals.var_fbbtbot_d);
            let assign36080_e46526: f64 = (assign36080_e46524 / locals.var_fmaxr);
            let assign36080_e46527: f64 = (assign36080_e46526).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36080_e46527, (assign36080_e46527 * (-((assign36080_e46524 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign36080_e46527 * (-((assign36080_e46524 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign36080_e46527 * (-((assign36080_e46524 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign36080_e46527 * (-((assign36080_e46524 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign36090_e46531: f64 = (-locals.var_fbbtbot_d);
        let assign36090_e46533: f64 = (assign36090_e46531 / locals.var_fmaxr);
        let assign36090_e46535: f64 = if assign36090_e46533 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard724 = assign36090_e46535;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard721 == 0.0)) && (locals.var_guard723 == 0.0)) && (locals.var_guard724 != 0.0)) {
            let assign36100_e46553: f64 = (-230.25850929940458);
            let assign36100_e46555: f64 = (-locals.var_fbbtbot_d);
            let assign36100_e46557: f64 = (assign36100_e46555 / locals.var_fmaxr);
            let assign36100_e46558: f64 = (assign36100_e46553 - assign36100_e46557);
            let assign36100_e46562: f64 = (-230.25850929940458);
            let assign36100_e46564: f64 = (-locals.var_fbbtbot_d);
            let assign36100_e46566: f64 = (assign36100_e46564 / locals.var_fmaxr);
            let assign36100_e46567: f64 = (assign36100_e46562 - assign36100_e46566);
            let assign36100_e46570: f64 = (-230.25850929940458);
            let assign36100_e46572: f64 = (-locals.var_fbbtbot_d);
            let assign36100_e46574: f64 = (assign36100_e46572 / locals.var_fmaxr);
            let assign36100_e46575: f64 = (assign36100_e46570 - assign36100_e46574);
            let assign36100_e46577: f64 = (assign36100_e46575 * 0.3333333333333333);
            let assign36100_e46578: f64 = (1.0 + assign36100_e46577);
            let assign36100_e46579: f64 = (assign36100_e46567 * assign36100_e46578);
            let assign36100_e46580: f64 = (0.5 * assign36100_e46579);
            let assign36100_e46581: f64 = (1.0 + assign36100_e46580);
            let assign36100_e46582: f64 = (assign36100_e46558 * assign36100_e46581);
            let assign36100_e46583: f64 = (1.0 + assign36100_e46582);
            let assign36100_e46584: f64 = (1e-100 / assign36100_e46583);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36100_e46584, (-((1e-100 * (((-(-((assign36100_e46555 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36100_e46581) + (assign36100_e46558 * (0.5 * (((-(-((assign36100_e46564 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36100_e46578) + (assign36100_e46567 * ((-(-((assign36100_e46572 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign36100_e46583 * assign36100_e46583))), (-((1e-100 * (((-(-((assign36100_e46555 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36100_e46581) + (assign36100_e46558 * (0.5 * (((-(-((assign36100_e46564 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36100_e46578) + (assign36100_e46567 * ((-(-((assign36100_e46572 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign36100_e46583 * assign36100_e46583))), (-((1e-100 * (((-(-((assign36100_e46555 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36100_e46581) + (assign36100_e46558 * (0.5 * (((-(-((assign36100_e46564 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36100_e46578) + (assign36100_e46567 * ((-(-((assign36100_e46572 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign36100_e46583 * assign36100_e46583))), (-((1e-100 * (((-(-((assign36100_e46555 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36100_e46581) + (assign36100_e46558 * (0.5 * (((-(-((assign36100_e46564 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36100_e46578) + (assign36100_e46567 * ((-(-((assign36100_e46572 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign36100_e46583 * assign36100_e46583))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard721 == 0.0)) && (locals.var_guard723 == 0.0)) && (locals.var_guard724 == 0.0)) {
            let assign36110_e46605: f64 = (-locals.var_fbbtbot_d);
            let assign36110_e46607: f64 = (assign36110_e46605 / locals.var_fmaxr);
            let assign36110_e46609: f64 = (assign36110_e46607 - 230.25850929940458);
            let assign36110_e46613: f64 = (-locals.var_fbbtbot_d);
            let assign36110_e46615: f64 = (assign36110_e46613 / locals.var_fmaxr);
            let assign36110_e46617: f64 = (assign36110_e46615 - 230.25850929940458);
            let assign36110_e46620: f64 = (-locals.var_fbbtbot_d);
            let assign36110_e46622: f64 = (assign36110_e46620 / locals.var_fmaxr);
            let assign36110_e46624: f64 = (assign36110_e46622 - 230.25850929940458);
            let assign36110_e46626: f64 = (assign36110_e46624 * 0.3333333333333333);
            let assign36110_e46627: f64 = (1.0 + assign36110_e46626);
            let assign36110_e46628: f64 = (assign36110_e46617 * assign36110_e46627);
            let assign36110_e46629: f64 = (0.5 * assign36110_e46628);
            let assign36110_e46630: f64 = (1.0 + assign36110_e46629);
            let assign36110_e46631: f64 = (assign36110_e46609 * assign36110_e46630);
            let assign36110_e46632: f64 = (1.0 + assign36110_e46631);
            let assign36110_e46633: f64 = (1e100 * assign36110_e46632);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36110_e46633, (1e100 * (((-((assign36110_e46605 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36110_e46630) + (assign36110_e46609 * (0.5 * (((-((assign36110_e46613 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36110_e46627) + (assign36110_e46617 * ((-((assign36110_e46620 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36110_e46605 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36110_e46630) + (assign36110_e46609 * (0.5 * (((-((assign36110_e46613 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36110_e46627) + (assign36110_e46617 * ((-((assign36110_e46620 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36110_e46605 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36110_e46630) + (assign36110_e46609 * (0.5 * (((-((assign36110_e46613 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36110_e46627) + (assign36110_e46617 * ((-((assign36110_e46620 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36110_e46605 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36110_e46630) + (assign36110_e46609 * (0.5 * (((-((assign36110_e46613 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36110_e46627) + (assign36110_e46617 * ((-((assign36110_e46620 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
    }
    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard721 == 0.0)) {
            let assign36120_e46648: f64 = (locals.var_v4 * locals.var_fmaxr);
            let assign36120_e46650: f64 = (assign36120_e46648 * locals.var_fmaxr);
            let assign36120_e46652: f64 = (assign36120_e46650 * locals.var_tmp);
            let assign36120_e46653: f64 = (locals.var_cbbtbotd_i * assign36120_e46652);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign36120_e46653, (locals.var_cbbtbotd_i * (((((locals.var_v4 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign36120_e46648 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign36120_e46650 * locals.var_tmp_dn5))), (locals.var_cbbtbotd_i * (((((locals.var_v4 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign36120_e46648 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign36120_e46650 * locals.var_tmp_dn6))), (locals.var_cbbtbotd_i * (((((locals.var_v4 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign36120_e46648 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign36120_e46650 * locals.var_tmp_dn7))), (locals.var_cbbtbotd_i * (((((locals.var_v4 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign36120_e46648 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign36120_e46650 * locals.var_tmp_dn8))), );
        }
        let assign36130_e46658: f64 = if locals.var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard725 = assign36130_e46658;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard725 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign36150_e46672: f64 = (-locals.var_alphaav);
        let assign36150_e46674: f64 = (assign36150_e46672 * locals.var_vbrbotd_i);
        let assign36150_e46675: f64 = if locals.var_vav > assign36150_e46674 { 1.0 } else { 0.0 };
        locals.var_guard726 = assign36150_e46675;
        let assign36160_e46678: f64 = if locals.var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard727 = assign36160_e46678;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard725 == 0.0)) && (locals.var_guard726 != 0.0)) && (locals.var_guard727 != 0.0)) {
            let assign36170_e46694: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign36170_e46697: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign36170_e46698: f64 = (assign36170_e46694 * assign36170_e46697);
            let assign36170_e46701: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign36170_e46702: f64 = (assign36170_e46698 * assign36170_e46701);
            let assign36170_e46705: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign36170_e46706: f64 = (assign36170_e46702 * assign36170_e46705);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36170_e46706, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard725 == 0.0)) && (locals.var_guard726 != 0.0)) && (locals.var_guard727 == 0.0)) {
            let assign36180_e46725: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign36180_e46726: f64 = (assign36180_e46725).abs();
            let assign36180_e46728: f64 = (assign36180_e46726).powf(locals.var_pbrbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36180_e46728, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard725 == 0.0)) && (locals.var_guard726 != 0.0)) {
            let assign36190_e46745: f64 = (1.0 - locals.var_tmp);
            let assign36190_e46746: f64 = (1.0 / assign36190_e46745);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign36190_e46746, (-((-locals.var_tmp_dn5) / (assign36190_e46745 * assign36190_e46745))), (-((-locals.var_tmp_dn6) / (assign36190_e46745 * assign36190_e46745))), (-((-locals.var_tmp_dn7) / (assign36190_e46745 * assign36190_e46745))), (-((-locals.var_tmp_dn8) / (assign36190_e46745 * assign36190_e46745))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard725 == 0.0)) && (locals.var_guard726 == 0.0)) {
            let assign36200_e46765: f64 = (locals.var_alphaav * locals.var_vbrbotd_i);
            let assign36200_e46766: f64 = (locals.var_vav + assign36200_e46765);
            let assign36200_e46768: f64 = (assign36200_e46766 * locals.var_slopebot_d);
            let assign36200_e46769: f64 = (locals.var_fstopbot_d + assign36200_e46768);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign36200_e46769, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard711 == 0.0)) {
            let assign36210_e46781: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign36210_e46783: f64 = (assign36210_e46781 + locals.var_itat);
            let assign36210_e46785: f64 = (assign36210_e46783 + locals.var_ibbt);
            let assign36210_e46786: f64 = (p.p29 * assign36210_e46785);
            let assign36210_e46788: f64 = (assign36210_e46786 * locals.var_fbreakdown);
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (assign36210_e46788, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign36210_e46786 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign36210_e46786 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign36210_e46786 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign36210_e46786 * locals.var_fbreakdown_dn8)), );
        }
        let assign36220_e46793: f64 = if locals.var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard728 = assign36220_e46793;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 != 0.0)) {
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) {
            let assign36240_e46810: f64 = (locals.var_idsatsti_d * locals.var_idmult);
            locals.var_id__blk219 = assign36240_e46810;
        }
        let assign36250_e46819: f64 = if ((locals.var_csrhstid_i == 0.0) && (locals.var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard729 = assign36250_e46819;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 == 0.0)) {
            let assign36270_e46842: f64 = (locals.var_vbisti_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign36270_e46842;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 == 0.0)) {
            let assign36280_e46858: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign36280_e46859: f64 = (1.0 - assign36280_e46858);
            let assign36280_e46860: f64 = (assign36280_e46859).sqrt();
            let assign36280_e46861: f64 = (1.0 - assign36280_e46860);
            locals.var_wsrhstep = assign36280_e46861;
        }
        let assign36290_e46866: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard730 = assign36290_e46866;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 == 0.0)) && (locals.var_guard730 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 == 0.0)) && (locals.var_guard730 == 0.0)) {
            let assign36310_e46895: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign36310_e46897: f64 = (locals.var_wsrhstep).ln();
            let assign36310_e46898: f64 = (assign36310_e46895 * assign36310_e46897);
            let assign36310_e46901: f64 = (1.0 - locals.var_wsrhstep);
            let assign36310_e46902: f64 = (assign36310_e46898 / assign36310_e46901);
            let assign36310_e46904: f64 = (assign36310_e46902 + locals.var_wsrhstep);
            let assign36310_e46908: f64 = (2.0 * locals.var_pstid_i);
            let assign36310_e46909: f64 = (1.0 - assign36310_e46908);
            let assign36310_e46910: f64 = (assign36310_e46904 * assign36310_e46909);
            locals.var_dwsrh = assign36310_e46910;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 == 0.0)) {
            let assign36320_e46924: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign36320_e46924;
        }
        let assign36330_e46929: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard731 = assign36330_e46929;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 == 0.0)) && (locals.var_guard731 != 0.0)) {
            let assign36340_e46943: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv_d);
            let assign36340_e46944: f64 = (assign36340_e46943).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36340_e46944, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 == 0.0)) && (locals.var_guard731 == 0.0)) {
            let assign36350_e46961: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv_d);
            let assign36350_e46963: f64 = (assign36350_e46961).powf(locals.var_pstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36350_e46963, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 == 0.0)) {
            let assign36360_e46977: f64 = (locals.var_wdepnulrsti_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign36360_e46977, (locals.var_wdepnulrsti_d * locals.var_tmp_dn5), (locals.var_wdepnulrsti_d * locals.var_tmp_dn6), (locals.var_wdepnulrsti_d * locals.var_tmp_dn7), (locals.var_wdepnulrsti_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 == 0.0)) {
            let assign36370_e46992: f64 = (locals.var_zinv - 1.0);
            let assign36370_e46994: f64 = (assign36370_e46992 * locals.var_wdep);
            let assign36370_e46995: f64 = (locals.var_ftdsti_d * assign36370_e46994);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign36370_e46995, (locals.var_ftdsti_d * (assign36370_e46992 * locals.var_wdep_dn5)), (locals.var_ftdsti_d * (assign36370_e46992 * locals.var_wdep_dn6)), (locals.var_ftdsti_d * (assign36370_e46992 * locals.var_wdep_dn7)), (locals.var_ftdsti_d * (assign36370_e46992 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 == 0.0)) {
            let assign36380_e47010: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign36380_e47011: f64 = (locals.var_csrhstid_i * assign36380_e47010);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign36380_e47011, (locals.var_csrhstid_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign36390_e47016: f64 = if locals.var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard732 = assign36390_e47016;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36410_e47040: f64 = (locals.var_wdep * locals.var_one_minus_psti_d);
            let assign36410_e47042: f64 = (assign36410_e47040 / locals.var_vbi_minus_vjsrh);
            let assign36410_e47043: f64 = (locals.var_btatpartsti_d * assign36410_e47042);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign36410_e47043, (locals.var_btatpartsti_d * ((locals.var_wdep_dn5 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn6 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn7 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn8 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36420_e47057: f64 = (0.666666666666667 * locals.var_atatsti_d);
            let assign36420_e47059: f64 = (assign36420_e47057 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign36420_e47059, (-((assign36420_e47057 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign36420_e47057 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign36420_e47057 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign36420_e47057 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36430_e47073: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign36430_e47073, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36440_e47087: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign36440_e47090: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign36440_e47092: f64 = (assign36440_e47090 + 1.0);
            let assign36440_e47093: f64 = (assign36440_e47087 / assign36440_e47092);
            let assign36440_e47094: f64 = (assign36440_e47093).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign36440_e47094, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign36440_e47092) - (assign36440_e47087 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign36440_e47092 * assign36440_e47092)) / (2.0 * assign36440_e47094)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign36440_e47092) - (assign36440_e47087 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign36440_e47092 * assign36440_e47092)) / (2.0 * assign36440_e47094)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign36440_e47092) - (assign36440_e47087 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign36440_e47092 * assign36440_e47092)) / (2.0 * assign36440_e47094)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign36440_e47092) - (assign36440_e47087 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign36440_e47092 * assign36440_e47092)) / (2.0 * assign36440_e47094)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36450_e47107: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign36450_e47107, (locals.var_umax_dn5 / (2.0 * assign36450_e47107)), (locals.var_umax_dn6 / (2.0 * assign36450_e47107)), (locals.var_umax_dn7 / (2.0 * assign36450_e47107)), (locals.var_umax_dn8 / (2.0 * assign36450_e47107)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36460_e47121: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign36460_e47121, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign36470_e47125: f64 = (-locals.var_pstid_i);
        let assign36470_e47127: f64 = (assign36470_e47125 * locals.var_one_over_one_minus_psti_d);
        let assign36470_e47129: f64 = (-1.0);
        let assign36470_e47130: f64 = if assign36470_e47127 == assign36470_e47129 { 1.0 } else { 0.0 };
        locals.var_guard733 = assign36470_e47130;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) && (locals.var_guard733 != 0.0)) {
            let assign36480_e47146: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign36480_e47147: f64 = (1.0 + assign36480_e47146);
            let assign36480_e47148: f64 = (1.0 / assign36480_e47147);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign36480_e47148, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign36480_e47147 * assign36480_e47147))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign36480_e47147 * assign36480_e47147))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign36480_e47147 * assign36480_e47147))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign36480_e47147 * assign36480_e47147))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) && (locals.var_guard733 == 0.0)) {
            let assign36490_e47166: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign36490_e47167: f64 = (1.0 + assign36490_e47166);
            let assign36490_e47169: f64 = (-locals.var_pstid_i);
            let assign36490_e47171: f64 = (assign36490_e47169 * locals.var_one_over_one_minus_psti_d);
            let assign36490_e47172: f64 = (assign36490_e47167).powf(assign36490_e47171);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign36490_e47172, if 0.0 == 0.0 && ((assign36490_e47171) as f64).is_finite() && ((assign36490_e47171) as f64).fract() == 0.0 { if assign36490_e47171 == 0.0 { 0.0 } else { (assign36490_e47171 * ((assign36490_e47167).powf(assign36490_e47171 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign36490_e47172 * (assign36490_e47171 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign36490_e47167))) }, if 0.0 == 0.0 && ((assign36490_e47171) as f64).is_finite() && ((assign36490_e47171) as f64).fract() == 0.0 { if assign36490_e47171 == 0.0 { 0.0 } else { (assign36490_e47171 * ((assign36490_e47167).powf(assign36490_e47171 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign36490_e47172 * (assign36490_e47171 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign36490_e47167))) }, if 0.0 == 0.0 && ((assign36490_e47171) as f64).is_finite() && ((assign36490_e47171) as f64).fract() == 0.0 { if assign36490_e47171 == 0.0 { 0.0 } else { (assign36490_e47171 * ((assign36490_e47167).powf(assign36490_e47171 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign36490_e47172 * (assign36490_e47171 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign36490_e47167))) }, if 0.0 == 0.0 && ((assign36490_e47171) as f64).is_finite() && ((assign36490_e47171) as f64).fract() == 0.0 { if assign36490_e47171 == 0.0 { 0.0 } else { (assign36490_e47171 * ((assign36490_e47167).powf(assign36490_e47171 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign36490_e47172 * (assign36490_e47171 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign36490_e47167))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36500_e47186: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign36500_e47189: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign36500_e47190: f64 = (assign36500_e47186 / assign36500_e47189);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign36500_e47190, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign36500_e47189) - (assign36500_e47186 * locals.var_wgamma_dn5)) / (assign36500_e47189 * assign36500_e47189)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign36500_e47189) - (assign36500_e47186 * locals.var_wgamma_dn6)) / (assign36500_e47189 * assign36500_e47189)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign36500_e47189) - (assign36500_e47186 * locals.var_wgamma_dn7)) / (assign36500_e47189 * assign36500_e47189)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign36500_e47189) - (assign36500_e47186 * locals.var_wgamma_dn8)) / (assign36500_e47189 * assign36500_e47189)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36510_e47205: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign36510_e47206: f64 = (0.375 * assign36510_e47205);
            let assign36510_e47207: f64 = (assign36510_e47206).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign36510_e47207, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign36510_e47207)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign36510_e47207)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign36510_e47207)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign36510_e47207)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36520_e47222: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign36520_e47223: f64 = (2.0 * assign36520_e47222);
            let assign36520_e47225: f64 = (assign36520_e47223 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign36520_e47225, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36530_e47239: f64 = (locals.var_atatsti_d * locals.var_twoatatoverthreebtat);
            let assign36530_e47241: f64 = (assign36530_e47239 * locals.var_sqrtumax);
            let assign36530_e47244: f64 = (locals.var_atatsti_d * locals.var_umax);
            let assign36530_e47245: f64 = (assign36530_e47241 - assign36530_e47244);
            let assign36530_e47249: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign36530_e47250: f64 = (0.5 * assign36530_e47249);
            let assign36530_e47251: f64 = (assign36530_e47245 + assign36530_e47250);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign36530_e47251, (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign36530_e47239 * locals.var_sqrtumax_dn5)) - (locals.var_atatsti_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign36530_e47239 * locals.var_sqrtumax_dn6)) - (locals.var_atatsti_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign36530_e47239 * locals.var_sqrtumax_dn7)) - (locals.var_atatsti_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign36530_e47239 * locals.var_sqrtumax_dn8)) - (locals.var_atatsti_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36540_e47265: f64 = (locals.var_ltat - 1.0);
            let assign36540_e47267: f64 = (assign36540_e47265 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign36540_e47267, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign36540_e47265 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign36540_e47265 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign36540_e47265 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign36540_e47265 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36550_e47281: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign36550_e47281, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign36560_e47286: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard734 = assign36560_e47286;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) && (locals.var_guard734 != 0.0)) {
            let assign36570_e47302: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign36570_e47303: f64 = (1.0 + assign36570_e47302);
            let assign36570_e47304: f64 = (1.0 / assign36570_e47303);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign36570_e47304, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign36570_e47303 * assign36570_e47303))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign36570_e47303 * assign36570_e47303))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign36570_e47303 * assign36570_e47303))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign36570_e47303 * assign36570_e47303))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) && (locals.var_guard734 == 0.0)) {
            let assign36580_e47323: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign36580_e47324: f64 = (1.0 - assign36580_e47323);
            let assign36580_e47325: f64 = (1.0 / assign36580_e47324);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign36580_e47325, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign36580_e47324 * assign36580_e47324))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign36580_e47324 * assign36580_e47324))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign36580_e47324 * assign36580_e47324))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign36580_e47324 * assign36580_e47324))), );
        }
        let assign36590_e47329: f64 = (-locals.var_ysq);
        let assign36590_e47331: f64 = (assign36590_e47329 + locals.var_mtat);
        let assign36590_e47333: f64 = (-230.25850929940458);
        let assign36590_e47334: f64 = if assign36590_e47331 > assign36590_e47333 { 1.0 } else { 0.0 };
        locals.var_guard735 = assign36590_e47334;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) && (locals.var_guard735 != 0.0)) {
            let assign36600_e47347: f64 = (-locals.var_ysq);
            let assign36600_e47349: f64 = (assign36600_e47347 + locals.var_mtat);
            let assign36600_e47350: f64 = (assign36600_e47349).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36600_e47350, (assign36600_e47350 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign36600_e47350 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign36600_e47350 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign36600_e47350 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) && (locals.var_guard735 == 0.0)) {
            let assign36610_e47368: f64 = (-230.25850929940458);
            let assign36610_e47370: f64 = (-locals.var_ysq);
            let assign36610_e47372: f64 = (assign36610_e47370 + locals.var_mtat);
            let assign36610_e47373: f64 = (assign36610_e47368 - assign36610_e47372);
            let assign36610_e47377: f64 = (-230.25850929940458);
            let assign36610_e47379: f64 = (-locals.var_ysq);
            let assign36610_e47381: f64 = (assign36610_e47379 + locals.var_mtat);
            let assign36610_e47382: f64 = (assign36610_e47377 - assign36610_e47381);
            let assign36610_e47385: f64 = (-230.25850929940458);
            let assign36610_e47387: f64 = (-locals.var_ysq);
            let assign36610_e47389: f64 = (assign36610_e47387 + locals.var_mtat);
            let assign36610_e47390: f64 = (assign36610_e47385 - assign36610_e47389);
            let assign36610_e47392: f64 = (assign36610_e47390 * 0.3333333333333333);
            let assign36610_e47393: f64 = (1.0 + assign36610_e47392);
            let assign36610_e47394: f64 = (assign36610_e47382 * assign36610_e47393);
            let assign36610_e47395: f64 = (0.5 * assign36610_e47394);
            let assign36610_e47396: f64 = (1.0 + assign36610_e47395);
            let assign36610_e47397: f64 = (assign36610_e47373 * assign36610_e47396);
            let assign36610_e47398: f64 = (1.0 + assign36610_e47397);
            let assign36610_e47399: f64 = (1e-100 / assign36610_e47398);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36610_e47399, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign36610_e47396) + (assign36610_e47373 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign36610_e47393) + (assign36610_e47382 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign36610_e47398 * assign36610_e47398))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign36610_e47396) + (assign36610_e47373 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign36610_e47393) + (assign36610_e47382 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign36610_e47398 * assign36610_e47398))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign36610_e47396) + (assign36610_e47373 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign36610_e47393) + (assign36610_e47382 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign36610_e47398 * assign36610_e47398))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign36610_e47396) + (assign36610_e47373 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign36610_e47393) + (assign36610_e47382 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign36610_e47398 * assign36610_e47398))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36620_e47413: f64 = (0.29214664 * locals.var_terfc);
            let assign36620_e47417: f64 = (locals.var_terfc * locals.var_terfc);
            let assign36620_e47418: f64 = (locals.var_berfc * assign36620_e47417);
            let assign36620_e47419: f64 = (assign36620_e47413 + assign36620_e47418);
            let assign36620_e47423: f64 = (locals.var_terfc * locals.var_terfc);
            let assign36620_e47425: f64 = (assign36620_e47423 * locals.var_terfc);
            let assign36620_e47426: f64 = (locals.var_cerfc * assign36620_e47425);
            let assign36620_e47427: f64 = (assign36620_e47419 + assign36620_e47426);
            let assign36620_e47429: f64 = (assign36620_e47427 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign36620_e47429, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign36620_e47423 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign36620_e47427 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign36620_e47423 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign36620_e47427 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign36620_e47423 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign36620_e47427 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign36620_e47423 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign36620_e47427 * locals.var_tmp_dn8)), );
        }
        let assign36630_e47434: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard736 = assign36630_e47434;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) && (locals.var_guard736 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign36650_e47451: f64 = (-230.25850929940458);
        let assign36650_e47452: f64 = if locals.var_mtat > assign36650_e47451 { 1.0 } else { 0.0 };
        locals.var_guard737 = assign36650_e47452;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) && (locals.var_guard736 == 0.0)) && (locals.var_guard737 != 0.0)) {
            let assign36660_e47468: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36660_e47468, (assign36660_e47468 * locals.var_mtat_dn5), (assign36660_e47468 * locals.var_mtat_dn6), (assign36660_e47468 * locals.var_mtat_dn7), (assign36660_e47468 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) && (locals.var_guard736 == 0.0)) && (locals.var_guard737 == 0.0)) {
            let assign36670_e47489: f64 = (-230.25850929940458);
            let assign36670_e47491: f64 = (assign36670_e47489 - locals.var_mtat);
            let assign36670_e47495: f64 = (-230.25850929940458);
            let assign36670_e47497: f64 = (assign36670_e47495 - locals.var_mtat);
            let assign36670_e47500: f64 = (-230.25850929940458);
            let assign36670_e47502: f64 = (assign36670_e47500 - locals.var_mtat);
            let assign36670_e47504: f64 = (assign36670_e47502 * 0.3333333333333333);
            let assign36670_e47505: f64 = (1.0 + assign36670_e47504);
            let assign36670_e47506: f64 = (assign36670_e47497 * assign36670_e47505);
            let assign36670_e47507: f64 = (0.5 * assign36670_e47506);
            let assign36670_e47508: f64 = (1.0 + assign36670_e47507);
            let assign36670_e47509: f64 = (assign36670_e47491 * assign36670_e47508);
            let assign36670_e47510: f64 = (1.0 + assign36670_e47509);
            let assign36670_e47511: f64 = (1e-100 / assign36670_e47510);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36670_e47511, (-((1e-100 * (((-locals.var_mtat_dn5) * assign36670_e47508) + (assign36670_e47491 * (0.5 * (((-locals.var_mtat_dn5) * assign36670_e47505) + (assign36670_e47497 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign36670_e47510 * assign36670_e47510))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign36670_e47508) + (assign36670_e47491 * (0.5 * (((-locals.var_mtat_dn6) * assign36670_e47505) + (assign36670_e47497 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign36670_e47510 * assign36670_e47510))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign36670_e47508) + (assign36670_e47491 * (0.5 * (((-locals.var_mtat_dn7) * assign36670_e47505) + (assign36670_e47497 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign36670_e47510 * assign36670_e47510))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign36670_e47508) + (assign36670_e47491 * (0.5 * (((-locals.var_mtat_dn8) * assign36670_e47505) + (assign36670_e47497 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign36670_e47510 * assign36670_e47510))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) && (locals.var_guard736 == 0.0)) {
            let assign36680_e47528: f64 = (2.0 * locals.var_tmp);
            let assign36680_e47530: f64 = (assign36680_e47528 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign36680_e47530, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36690_e47544: f64 = (1.772453850905516 * 0.5);
            let assign36690_e47547: f64 = (locals.var_atatsti_d * locals.var_erfctimesexpmtat);
            let assign36690_e47549: f64 = (assign36690_e47547 / locals.var_ktat);
            let assign36690_e47550: f64 = (assign36690_e47544 * assign36690_e47549);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign36690_e47550, (assign36690_e47544 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign36690_e47547 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign36690_e47544 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign36690_e47547 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign36690_e47544 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign36690_e47547 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign36690_e47544 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign36690_e47547 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard732 == 0.0)) {
            let assign36700_e47565: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign36700_e47567: f64 = (assign36700_e47565 * locals.var_wtat);
            let assign36700_e47568: f64 = (locals.var_ctatstid_i * assign36700_e47567);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign36700_e47568, (locals.var_ctatstid_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign36700_e47565 * locals.var_wtat_dn5))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign36700_e47565 * locals.var_wtat_dn6))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign36700_e47565 * locals.var_wtat_dn7))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign36700_e47565 * locals.var_wtat_dn8))), );
        }
        let assign36710_e47573: f64 = if locals.var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard738 = assign36710_e47573;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard738 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign36730_e47587: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard739 = assign36730_e47587;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard738 == 0.0)) && (locals.var_guard739 != 0.0)) {
            let assign36740_e47601: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign36740_e47603: f64 = (assign36740_e47601 * locals.var_vbirstiinv_d);
            let assign36740_e47604: f64 = (assign36740_e47603).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36740_e47604, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard738 == 0.0)) && (locals.var_guard739 == 0.0)) {
            let assign36750_e47621: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign36750_e47623: f64 = (assign36750_e47621 * locals.var_vbirstiinv_d);
            let assign36750_e47625: f64 = (assign36750_e47623).powf(locals.var_pstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36750_e47625, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard738 == 0.0)) {
            let assign36760_e47640: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign36760_e47642: f64 = (assign36760_e47640 * locals.var_wdepnulrinvsti_d);
            let assign36760_e47644: f64 = (assign36760_e47642 / locals.var_tmp);
            let assign36760_e47645: f64 = (locals.var_one_over_one_minus_psti_d * assign36760_e47644);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign36760_e47645, (locals.var_one_over_one_minus_psti_d * (-((assign36760_e47642 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign36760_e47642 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign36760_e47642 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign36760_e47642 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign36770_e47649: f64 = (-locals.var_fbbtsti_d);
        let assign36770_e47651: f64 = (assign36770_e47649 / locals.var_fmaxr);
        let assign36770_e47652: f64 = (assign36770_e47651).abs();
        let assign36770_e47654: f64 = if assign36770_e47652 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard740 = assign36770_e47654;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard738 == 0.0)) && (locals.var_guard740 != 0.0)) {
            let assign36780_e47667: f64 = (-locals.var_fbbtsti_d);
            let assign36780_e47669: f64 = (assign36780_e47667 / locals.var_fmaxr);
            let assign36780_e47670: f64 = (assign36780_e47669).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36780_e47670, (assign36780_e47670 * (-((assign36780_e47667 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign36780_e47670 * (-((assign36780_e47667 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign36780_e47670 * (-((assign36780_e47667 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign36780_e47670 * (-((assign36780_e47667 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign36790_e47674: f64 = (-locals.var_fbbtsti_d);
        let assign36790_e47676: f64 = (assign36790_e47674 / locals.var_fmaxr);
        let assign36790_e47678: f64 = if assign36790_e47676 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard741 = assign36790_e47678;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard738 == 0.0)) && (locals.var_guard740 == 0.0)) && (locals.var_guard741 != 0.0)) {
            let assign36800_e47696: f64 = (-230.25850929940458);
            let assign36800_e47698: f64 = (-locals.var_fbbtsti_d);
            let assign36800_e47700: f64 = (assign36800_e47698 / locals.var_fmaxr);
            let assign36800_e47701: f64 = (assign36800_e47696 - assign36800_e47700);
            let assign36800_e47705: f64 = (-230.25850929940458);
            let assign36800_e47707: f64 = (-locals.var_fbbtsti_d);
            let assign36800_e47709: f64 = (assign36800_e47707 / locals.var_fmaxr);
            let assign36800_e47710: f64 = (assign36800_e47705 - assign36800_e47709);
            let assign36800_e47713: f64 = (-230.25850929940458);
            let assign36800_e47715: f64 = (-locals.var_fbbtsti_d);
            let assign36800_e47717: f64 = (assign36800_e47715 / locals.var_fmaxr);
            let assign36800_e47718: f64 = (assign36800_e47713 - assign36800_e47717);
            let assign36800_e47720: f64 = (assign36800_e47718 * 0.3333333333333333);
            let assign36800_e47721: f64 = (1.0 + assign36800_e47720);
            let assign36800_e47722: f64 = (assign36800_e47710 * assign36800_e47721);
            let assign36800_e47723: f64 = (0.5 * assign36800_e47722);
            let assign36800_e47724: f64 = (1.0 + assign36800_e47723);
            let assign36800_e47725: f64 = (assign36800_e47701 * assign36800_e47724);
            let assign36800_e47726: f64 = (1.0 + assign36800_e47725);
            let assign36800_e47727: f64 = (1e-100 / assign36800_e47726);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36800_e47727, (-((1e-100 * (((-(-((assign36800_e47698 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36800_e47724) + (assign36800_e47701 * (0.5 * (((-(-((assign36800_e47707 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36800_e47721) + (assign36800_e47710 * ((-(-((assign36800_e47715 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign36800_e47726 * assign36800_e47726))), (-((1e-100 * (((-(-((assign36800_e47698 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36800_e47724) + (assign36800_e47701 * (0.5 * (((-(-((assign36800_e47707 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36800_e47721) + (assign36800_e47710 * ((-(-((assign36800_e47715 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign36800_e47726 * assign36800_e47726))), (-((1e-100 * (((-(-((assign36800_e47698 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36800_e47724) + (assign36800_e47701 * (0.5 * (((-(-((assign36800_e47707 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36800_e47721) + (assign36800_e47710 * ((-(-((assign36800_e47715 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign36800_e47726 * assign36800_e47726))), (-((1e-100 * (((-(-((assign36800_e47698 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36800_e47724) + (assign36800_e47701 * (0.5 * (((-(-((assign36800_e47707 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign36800_e47721) + (assign36800_e47710 * ((-(-((assign36800_e47715 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign36800_e47726 * assign36800_e47726))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard738 == 0.0)) && (locals.var_guard740 == 0.0)) && (locals.var_guard741 == 0.0)) {
            let assign36810_e47748: f64 = (-locals.var_fbbtsti_d);
            let assign36810_e47750: f64 = (assign36810_e47748 / locals.var_fmaxr);
            let assign36810_e47752: f64 = (assign36810_e47750 - 230.25850929940458);
            let assign36810_e47756: f64 = (-locals.var_fbbtsti_d);
            let assign36810_e47758: f64 = (assign36810_e47756 / locals.var_fmaxr);
            let assign36810_e47760: f64 = (assign36810_e47758 - 230.25850929940458);
            let assign36810_e47763: f64 = (-locals.var_fbbtsti_d);
            let assign36810_e47765: f64 = (assign36810_e47763 / locals.var_fmaxr);
            let assign36810_e47767: f64 = (assign36810_e47765 - 230.25850929940458);
            let assign36810_e47769: f64 = (assign36810_e47767 * 0.3333333333333333);
            let assign36810_e47770: f64 = (1.0 + assign36810_e47769);
            let assign36810_e47771: f64 = (assign36810_e47760 * assign36810_e47770);
            let assign36810_e47772: f64 = (0.5 * assign36810_e47771);
            let assign36810_e47773: f64 = (1.0 + assign36810_e47772);
            let assign36810_e47774: f64 = (assign36810_e47752 * assign36810_e47773);
            let assign36810_e47775: f64 = (1.0 + assign36810_e47774);
            let assign36810_e47776: f64 = (1e100 * assign36810_e47775);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36810_e47776, (1e100 * (((-((assign36810_e47748 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36810_e47773) + (assign36810_e47752 * (0.5 * (((-((assign36810_e47756 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36810_e47770) + (assign36810_e47760 * ((-((assign36810_e47763 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36810_e47748 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36810_e47773) + (assign36810_e47752 * (0.5 * (((-((assign36810_e47756 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36810_e47770) + (assign36810_e47760 * ((-((assign36810_e47763 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36810_e47748 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36810_e47773) + (assign36810_e47752 * (0.5 * (((-((assign36810_e47756 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36810_e47770) + (assign36810_e47760 * ((-((assign36810_e47763 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36810_e47748 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36810_e47773) + (assign36810_e47752 * (0.5 * (((-((assign36810_e47756 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign36810_e47770) + (assign36810_e47760 * ((-((assign36810_e47763 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard738 == 0.0)) {
            let assign36820_e47791: f64 = (locals.var_v4 * locals.var_fmaxr);
            let assign36820_e47793: f64 = (assign36820_e47791 * locals.var_fmaxr);
            let assign36820_e47795: f64 = (assign36820_e47793 * locals.var_tmp);
            let assign36820_e47796: f64 = (locals.var_cbbtstid_i * assign36820_e47795);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign36820_e47796, (locals.var_cbbtstid_i * (((((locals.var_v4 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign36820_e47791 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign36820_e47793 * locals.var_tmp_dn5))), (locals.var_cbbtstid_i * (((((locals.var_v4 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign36820_e47791 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign36820_e47793 * locals.var_tmp_dn6))), (locals.var_cbbtstid_i * (((((locals.var_v4 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign36820_e47791 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign36820_e47793 * locals.var_tmp_dn7))), (locals.var_cbbtstid_i * (((((locals.var_v4 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign36820_e47791 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign36820_e47793 * locals.var_tmp_dn8))), );
        }
        let assign36830_e47801: f64 = if locals.var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign36830_e47801;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard742 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign36850_e47815: f64 = (-locals.var_alphaav);
        let assign36850_e47817: f64 = (assign36850_e47815 * locals.var_vbrstid_i);
        let assign36850_e47818: f64 = if locals.var_vav > assign36850_e47817 { 1.0 } else { 0.0 };
        locals.var_guard743 = assign36850_e47818;
        let assign36860_e47821: f64 = if locals.var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard744 = assign36860_e47821;
    }
    pub(super) fn stamp_transient_block_44(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard742 == 0.0)) && (locals.var_guard743 != 0.0)) && (locals.var_guard744 != 0.0)) {
            let assign36870_e47837: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign36870_e47840: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign36870_e47841: f64 = (assign36870_e47837 * assign36870_e47840);
            let assign36870_e47844: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign36870_e47845: f64 = (assign36870_e47841 * assign36870_e47844);
            let assign36870_e47848: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign36870_e47849: f64 = (assign36870_e47845 * assign36870_e47848);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36870_e47849, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard742 == 0.0)) && (locals.var_guard743 != 0.0)) && (locals.var_guard744 == 0.0)) {
            let assign36880_e47868: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign36880_e47869: f64 = (assign36880_e47868).abs();
            let assign36880_e47871: f64 = (assign36880_e47869).powf(locals.var_pbrstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign36880_e47871, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard742 == 0.0)) && (locals.var_guard743 != 0.0)) {
            let assign36890_e47888: f64 = (1.0 - locals.var_tmp);
            let assign36890_e47889: f64 = (1.0 / assign36890_e47888);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign36890_e47889, (-((-locals.var_tmp_dn5) / (assign36890_e47888 * assign36890_e47888))), (-((-locals.var_tmp_dn6) / (assign36890_e47888 * assign36890_e47888))), (-((-locals.var_tmp_dn7) / (assign36890_e47888 * assign36890_e47888))), (-((-locals.var_tmp_dn8) / (assign36890_e47888 * assign36890_e47888))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard742 == 0.0)) && (locals.var_guard743 == 0.0)) {
            let assign36900_e47908: f64 = (locals.var_alphaav * locals.var_vbrstid_i);
            let assign36900_e47909: f64 = (locals.var_vav + assign36900_e47908);
            let assign36900_e47911: f64 = (assign36900_e47909 * locals.var_slopesti_d);
            let assign36900_e47912: f64 = (locals.var_fstopsti_d + assign36900_e47911);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign36900_e47912, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard728 == 0.0)) {
            let assign36910_e47924: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign36910_e47926: f64 = (assign36910_e47924 + locals.var_itat);
            let assign36910_e47928: f64 = (assign36910_e47926 + locals.var_ibbt);
            let assign36910_e47929: f64 = (p.p29 * assign36910_e47928);
            let assign36910_e47931: f64 = (assign36910_e47929 * locals.var_fbreakdown);
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (assign36910_e47931, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign36910_e47929 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign36910_e47929 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign36910_e47929 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign36910_e47929 * locals.var_fbreakdown_dn8)), );
        }
        let assign36920_e47936: f64 = if locals.var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign36920_e47936;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 != 0.0)) {
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) {
            let assign36940_e47953: f64 = (locals.var_idsatgat_d * locals.var_idmult);
            locals.var_id__blk219 = assign36940_e47953;
        }
        let assign36950_e47962: f64 = if ((locals.var_csrhgatd_i == 0.0) && (locals.var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard746 = assign36950_e47962;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 == 0.0)) {
            let assign36970_e47985: f64 = (locals.var_vbigat_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign36970_e47985;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 == 0.0)) {
            let assign36980_e48001: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign36980_e48002: f64 = (1.0 - assign36980_e48001);
            let assign36980_e48003: f64 = (assign36980_e48002).sqrt();
            let assign36980_e48004: f64 = (1.0 - assign36980_e48003);
            locals.var_wsrhstep = assign36980_e48004;
        }
        let assign36990_e48009: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard747 = assign36990_e48009;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
            let assign37010_e48038: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign37010_e48040: f64 = (locals.var_wsrhstep).ln();
            let assign37010_e48041: f64 = (assign37010_e48038 * assign37010_e48040);
            let assign37010_e48044: f64 = (1.0 - locals.var_wsrhstep);
            let assign37010_e48045: f64 = (assign37010_e48041 / assign37010_e48044);
            let assign37010_e48047: f64 = (assign37010_e48045 + locals.var_wsrhstep);
            let assign37010_e48051: f64 = (2.0 * locals.var_pgatd_i);
            let assign37010_e48052: f64 = (1.0 - assign37010_e48051);
            let assign37010_e48053: f64 = (assign37010_e48047 * assign37010_e48052);
            locals.var_dwsrh = assign37010_e48053;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 == 0.0)) {
            let assign37020_e48067: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign37020_e48067;
        }
        let assign37030_e48072: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard748 = assign37030_e48072;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 == 0.0)) && (locals.var_guard748 != 0.0)) {
            let assign37040_e48086: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv_d);
            let assign37040_e48087: f64 = (assign37040_e48086).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37040_e48087, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 == 0.0)) && (locals.var_guard748 == 0.0)) {
            let assign37050_e48104: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv_d);
            let assign37050_e48106: f64 = (assign37050_e48104).powf(locals.var_pgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37050_e48106, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 == 0.0)) {
            let assign37060_e48120: f64 = (locals.var_wdepnulrgat_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign37060_e48120, (locals.var_wdepnulrgat_d * locals.var_tmp_dn5), (locals.var_wdepnulrgat_d * locals.var_tmp_dn6), (locals.var_wdepnulrgat_d * locals.var_tmp_dn7), (locals.var_wdepnulrgat_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 == 0.0)) {
            let assign37070_e48135: f64 = (locals.var_zinv - 1.0);
            let assign37070_e48137: f64 = (assign37070_e48135 * locals.var_wdep);
            let assign37070_e48138: f64 = (locals.var_ftdgat_d * assign37070_e48137);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign37070_e48138, (locals.var_ftdgat_d * (assign37070_e48135 * locals.var_wdep_dn5)), (locals.var_ftdgat_d * (assign37070_e48135 * locals.var_wdep_dn6)), (locals.var_ftdgat_d * (assign37070_e48135 * locals.var_wdep_dn7)), (locals.var_ftdgat_d * (assign37070_e48135 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 == 0.0)) {
            let assign37080_e48153: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign37080_e48154: f64 = (locals.var_csrhgatd_i * assign37080_e48153);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign37080_e48154, (locals.var_csrhgatd_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign37090_e48159: f64 = if locals.var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard749 = assign37090_e48159;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37110_e48183: f64 = (locals.var_wdep * locals.var_one_minus_pgat_d);
            let assign37110_e48185: f64 = (assign37110_e48183 / locals.var_vbi_minus_vjsrh);
            let assign37110_e48186: f64 = (locals.var_btatpartgat_d * assign37110_e48185);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign37110_e48186, (locals.var_btatpartgat_d * ((locals.var_wdep_dn5 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn6 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn7 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn8 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37120_e48200: f64 = (0.666666666666667 * locals.var_atatgat_d);
            let assign37120_e48202: f64 = (assign37120_e48200 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign37120_e48202, (-((assign37120_e48200 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign37120_e48200 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign37120_e48200 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign37120_e48200 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37130_e48216: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign37130_e48216, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37140_e48230: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign37140_e48233: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign37140_e48235: f64 = (assign37140_e48233 + 1.0);
            let assign37140_e48236: f64 = (assign37140_e48230 / assign37140_e48235);
            let assign37140_e48237: f64 = (assign37140_e48236).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign37140_e48237, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign37140_e48235) - (assign37140_e48230 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign37140_e48235 * assign37140_e48235)) / (2.0 * assign37140_e48237)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign37140_e48235) - (assign37140_e48230 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign37140_e48235 * assign37140_e48235)) / (2.0 * assign37140_e48237)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign37140_e48235) - (assign37140_e48230 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign37140_e48235 * assign37140_e48235)) / (2.0 * assign37140_e48237)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign37140_e48235) - (assign37140_e48230 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign37140_e48235 * assign37140_e48235)) / (2.0 * assign37140_e48237)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37150_e48250: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign37150_e48250, (locals.var_umax_dn5 / (2.0 * assign37150_e48250)), (locals.var_umax_dn6 / (2.0 * assign37150_e48250)), (locals.var_umax_dn7 / (2.0 * assign37150_e48250)), (locals.var_umax_dn8 / (2.0 * assign37150_e48250)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37160_e48264: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign37160_e48264, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign37170_e48268: f64 = (-locals.var_pgatd_i);
        let assign37170_e48270: f64 = (assign37170_e48268 * locals.var_one_over_one_minus_pgat_d);
        let assign37170_e48272: f64 = (-1.0);
        let assign37170_e48273: f64 = if assign37170_e48270 == assign37170_e48272 { 1.0 } else { 0.0 };
        locals.var_guard750 = assign37170_e48273;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) && (locals.var_guard750 != 0.0)) {
            let assign37180_e48289: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign37180_e48290: f64 = (1.0 + assign37180_e48289);
            let assign37180_e48291: f64 = (1.0 / assign37180_e48290);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign37180_e48291, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign37180_e48290 * assign37180_e48290))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign37180_e48290 * assign37180_e48290))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign37180_e48290 * assign37180_e48290))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign37180_e48290 * assign37180_e48290))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) && (locals.var_guard750 == 0.0)) {
            let assign37190_e48309: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign37190_e48310: f64 = (1.0 + assign37190_e48309);
            let assign37190_e48312: f64 = (-locals.var_pgatd_i);
            let assign37190_e48314: f64 = (assign37190_e48312 * locals.var_one_over_one_minus_pgat_d);
            let assign37190_e48315: f64 = (assign37190_e48310).powf(assign37190_e48314);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign37190_e48315, if 0.0 == 0.0 && ((assign37190_e48314) as f64).is_finite() && ((assign37190_e48314) as f64).fract() == 0.0 { if assign37190_e48314 == 0.0 { 0.0 } else { (assign37190_e48314 * ((assign37190_e48310).powf(assign37190_e48314 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign37190_e48315 * (assign37190_e48314 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign37190_e48310))) }, if 0.0 == 0.0 && ((assign37190_e48314) as f64).is_finite() && ((assign37190_e48314) as f64).fract() == 0.0 { if assign37190_e48314 == 0.0 { 0.0 } else { (assign37190_e48314 * ((assign37190_e48310).powf(assign37190_e48314 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign37190_e48315 * (assign37190_e48314 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign37190_e48310))) }, if 0.0 == 0.0 && ((assign37190_e48314) as f64).is_finite() && ((assign37190_e48314) as f64).fract() == 0.0 { if assign37190_e48314 == 0.0 { 0.0 } else { (assign37190_e48314 * ((assign37190_e48310).powf(assign37190_e48314 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign37190_e48315 * (assign37190_e48314 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign37190_e48310))) }, if 0.0 == 0.0 && ((assign37190_e48314) as f64).is_finite() && ((assign37190_e48314) as f64).fract() == 0.0 { if assign37190_e48314 == 0.0 { 0.0 } else { (assign37190_e48314 * ((assign37190_e48310).powf(assign37190_e48314 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign37190_e48315 * (assign37190_e48314 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign37190_e48310))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37200_e48329: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign37200_e48332: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign37200_e48333: f64 = (assign37200_e48329 / assign37200_e48332);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign37200_e48333, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign37200_e48332) - (assign37200_e48329 * locals.var_wgamma_dn5)) / (assign37200_e48332 * assign37200_e48332)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign37200_e48332) - (assign37200_e48329 * locals.var_wgamma_dn6)) / (assign37200_e48332 * assign37200_e48332)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign37200_e48332) - (assign37200_e48329 * locals.var_wgamma_dn7)) / (assign37200_e48332 * assign37200_e48332)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign37200_e48332) - (assign37200_e48329 * locals.var_wgamma_dn8)) / (assign37200_e48332 * assign37200_e48332)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37210_e48348: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign37210_e48349: f64 = (0.375 * assign37210_e48348);
            let assign37210_e48350: f64 = (assign37210_e48349).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign37210_e48350, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign37210_e48350)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign37210_e48350)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign37210_e48350)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign37210_e48350)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37220_e48365: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign37220_e48366: f64 = (2.0 * assign37220_e48365);
            let assign37220_e48368: f64 = (assign37220_e48366 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign37220_e48368, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37230_e48382: f64 = (locals.var_atatgat_d * locals.var_twoatatoverthreebtat);
            let assign37230_e48384: f64 = (assign37230_e48382 * locals.var_sqrtumax);
            let assign37230_e48387: f64 = (locals.var_atatgat_d * locals.var_umax);
            let assign37230_e48388: f64 = (assign37230_e48384 - assign37230_e48387);
            let assign37230_e48392: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign37230_e48393: f64 = (0.5 * assign37230_e48392);
            let assign37230_e48394: f64 = (assign37230_e48388 + assign37230_e48393);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign37230_e48394, (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign37230_e48382 * locals.var_sqrtumax_dn5)) - (locals.var_atatgat_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign37230_e48382 * locals.var_sqrtumax_dn6)) - (locals.var_atatgat_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign37230_e48382 * locals.var_sqrtumax_dn7)) - (locals.var_atatgat_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign37230_e48382 * locals.var_sqrtumax_dn8)) - (locals.var_atatgat_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37240_e48408: f64 = (locals.var_ltat - 1.0);
            let assign37240_e48410: f64 = (assign37240_e48408 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign37240_e48410, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign37240_e48408 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign37240_e48408 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign37240_e48408 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign37240_e48408 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37250_e48424: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign37250_e48424, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign37260_e48429: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard751 = assign37260_e48429;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) && (locals.var_guard751 != 0.0)) {
            let assign37270_e48445: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign37270_e48446: f64 = (1.0 + assign37270_e48445);
            let assign37270_e48447: f64 = (1.0 / assign37270_e48446);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign37270_e48447, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign37270_e48446 * assign37270_e48446))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign37270_e48446 * assign37270_e48446))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign37270_e48446 * assign37270_e48446))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign37270_e48446 * assign37270_e48446))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) && (locals.var_guard751 == 0.0)) {
            let assign37280_e48466: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign37280_e48467: f64 = (1.0 - assign37280_e48466);
            let assign37280_e48468: f64 = (1.0 / assign37280_e48467);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign37280_e48468, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign37280_e48467 * assign37280_e48467))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign37280_e48467 * assign37280_e48467))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign37280_e48467 * assign37280_e48467))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign37280_e48467 * assign37280_e48467))), );
        }
        let assign37290_e48472: f64 = (-locals.var_ysq);
        let assign37290_e48474: f64 = (assign37290_e48472 + locals.var_mtat);
        let assign37290_e48476: f64 = (-230.25850929940458);
        let assign37290_e48477: f64 = if assign37290_e48474 > assign37290_e48476 { 1.0 } else { 0.0 };
        locals.var_guard752 = assign37290_e48477;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) && (locals.var_guard752 != 0.0)) {
            let assign37300_e48490: f64 = (-locals.var_ysq);
            let assign37300_e48492: f64 = (assign37300_e48490 + locals.var_mtat);
            let assign37300_e48493: f64 = (assign37300_e48492).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37300_e48493, (assign37300_e48493 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign37300_e48493 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign37300_e48493 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign37300_e48493 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) && (locals.var_guard752 == 0.0)) {
            let assign37310_e48511: f64 = (-230.25850929940458);
            let assign37310_e48513: f64 = (-locals.var_ysq);
            let assign37310_e48515: f64 = (assign37310_e48513 + locals.var_mtat);
            let assign37310_e48516: f64 = (assign37310_e48511 - assign37310_e48515);
            let assign37310_e48520: f64 = (-230.25850929940458);
            let assign37310_e48522: f64 = (-locals.var_ysq);
            let assign37310_e48524: f64 = (assign37310_e48522 + locals.var_mtat);
            let assign37310_e48525: f64 = (assign37310_e48520 - assign37310_e48524);
            let assign37310_e48528: f64 = (-230.25850929940458);
            let assign37310_e48530: f64 = (-locals.var_ysq);
            let assign37310_e48532: f64 = (assign37310_e48530 + locals.var_mtat);
            let assign37310_e48533: f64 = (assign37310_e48528 - assign37310_e48532);
            let assign37310_e48535: f64 = (assign37310_e48533 * 0.3333333333333333);
            let assign37310_e48536: f64 = (1.0 + assign37310_e48535);
            let assign37310_e48537: f64 = (assign37310_e48525 * assign37310_e48536);
            let assign37310_e48538: f64 = (0.5 * assign37310_e48537);
            let assign37310_e48539: f64 = (1.0 + assign37310_e48538);
            let assign37310_e48540: f64 = (assign37310_e48516 * assign37310_e48539);
            let assign37310_e48541: f64 = (1.0 + assign37310_e48540);
            let assign37310_e48542: f64 = (1e-100 / assign37310_e48541);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37310_e48542, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign37310_e48539) + (assign37310_e48516 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign37310_e48536) + (assign37310_e48525 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign37310_e48541 * assign37310_e48541))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign37310_e48539) + (assign37310_e48516 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign37310_e48536) + (assign37310_e48525 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign37310_e48541 * assign37310_e48541))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign37310_e48539) + (assign37310_e48516 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign37310_e48536) + (assign37310_e48525 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign37310_e48541 * assign37310_e48541))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign37310_e48539) + (assign37310_e48516 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign37310_e48536) + (assign37310_e48525 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign37310_e48541 * assign37310_e48541))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37320_e48556: f64 = (0.29214664 * locals.var_terfc);
            let assign37320_e48560: f64 = (locals.var_terfc * locals.var_terfc);
            let assign37320_e48561: f64 = (locals.var_berfc * assign37320_e48560);
            let assign37320_e48562: f64 = (assign37320_e48556 + assign37320_e48561);
            let assign37320_e48566: f64 = (locals.var_terfc * locals.var_terfc);
            let assign37320_e48568: f64 = (assign37320_e48566 * locals.var_terfc);
            let assign37320_e48569: f64 = (locals.var_cerfc * assign37320_e48568);
            let assign37320_e48570: f64 = (assign37320_e48562 + assign37320_e48569);
            let assign37320_e48572: f64 = (assign37320_e48570 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign37320_e48572, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign37320_e48566 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign37320_e48570 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign37320_e48566 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign37320_e48570 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign37320_e48566 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign37320_e48570 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign37320_e48566 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign37320_e48570 * locals.var_tmp_dn8)), );
        }
        let assign37330_e48577: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard753 = assign37330_e48577;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) && (locals.var_guard753 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign37350_e48594: f64 = (-230.25850929940458);
        let assign37350_e48595: f64 = if locals.var_mtat > assign37350_e48594 { 1.0 } else { 0.0 };
        locals.var_guard754 = assign37350_e48595;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) && (locals.var_guard753 == 0.0)) && (locals.var_guard754 != 0.0)) {
            let assign37360_e48611: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37360_e48611, (assign37360_e48611 * locals.var_mtat_dn5), (assign37360_e48611 * locals.var_mtat_dn6), (assign37360_e48611 * locals.var_mtat_dn7), (assign37360_e48611 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) && (locals.var_guard753 == 0.0)) && (locals.var_guard754 == 0.0)) {
            let assign37370_e48632: f64 = (-230.25850929940458);
            let assign37370_e48634: f64 = (assign37370_e48632 - locals.var_mtat);
            let assign37370_e48638: f64 = (-230.25850929940458);
            let assign37370_e48640: f64 = (assign37370_e48638 - locals.var_mtat);
            let assign37370_e48643: f64 = (-230.25850929940458);
            let assign37370_e48645: f64 = (assign37370_e48643 - locals.var_mtat);
            let assign37370_e48647: f64 = (assign37370_e48645 * 0.3333333333333333);
            let assign37370_e48648: f64 = (1.0 + assign37370_e48647);
            let assign37370_e48649: f64 = (assign37370_e48640 * assign37370_e48648);
            let assign37370_e48650: f64 = (0.5 * assign37370_e48649);
            let assign37370_e48651: f64 = (1.0 + assign37370_e48650);
            let assign37370_e48652: f64 = (assign37370_e48634 * assign37370_e48651);
            let assign37370_e48653: f64 = (1.0 + assign37370_e48652);
            let assign37370_e48654: f64 = (1e-100 / assign37370_e48653);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37370_e48654, (-((1e-100 * (((-locals.var_mtat_dn5) * assign37370_e48651) + (assign37370_e48634 * (0.5 * (((-locals.var_mtat_dn5) * assign37370_e48648) + (assign37370_e48640 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign37370_e48653 * assign37370_e48653))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign37370_e48651) + (assign37370_e48634 * (0.5 * (((-locals.var_mtat_dn6) * assign37370_e48648) + (assign37370_e48640 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign37370_e48653 * assign37370_e48653))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign37370_e48651) + (assign37370_e48634 * (0.5 * (((-locals.var_mtat_dn7) * assign37370_e48648) + (assign37370_e48640 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign37370_e48653 * assign37370_e48653))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign37370_e48651) + (assign37370_e48634 * (0.5 * (((-locals.var_mtat_dn8) * assign37370_e48648) + (assign37370_e48640 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign37370_e48653 * assign37370_e48653))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) && (locals.var_guard753 == 0.0)) {
            let assign37380_e48671: f64 = (2.0 * locals.var_tmp);
            let assign37380_e48673: f64 = (assign37380_e48671 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign37380_e48673, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37390_e48687: f64 = (1.772453850905516 * 0.5);
            let assign37390_e48690: f64 = (locals.var_atatgat_d * locals.var_erfctimesexpmtat);
            let assign37390_e48692: f64 = (assign37390_e48690 / locals.var_ktat);
            let assign37390_e48693: f64 = (assign37390_e48687 * assign37390_e48692);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign37390_e48693, (assign37390_e48687 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign37390_e48690 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign37390_e48687 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign37390_e48690 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign37390_e48687 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign37390_e48690 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign37390_e48687 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign37390_e48690 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard749 == 0.0)) {
            let assign37400_e48708: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign37400_e48710: f64 = (assign37400_e48708 * locals.var_wtat);
            let assign37400_e48711: f64 = (locals.var_ctatgatd_i * assign37400_e48710);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign37400_e48711, (locals.var_ctatgatd_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign37400_e48708 * locals.var_wtat_dn5))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign37400_e48708 * locals.var_wtat_dn6))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign37400_e48708 * locals.var_wtat_dn7))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign37400_e48708 * locals.var_wtat_dn8))), );
        }
        let assign37410_e48716: f64 = if locals.var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard755 = assign37410_e48716;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard755 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign37430_e48730: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard756 = assign37430_e48730;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard755 == 0.0)) && (locals.var_guard756 != 0.0)) {
            let assign37440_e48744: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign37440_e48746: f64 = (assign37440_e48744 * locals.var_vbirgatinv_d);
            let assign37440_e48747: f64 = (assign37440_e48746).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37440_e48747, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard755 == 0.0)) && (locals.var_guard756 == 0.0)) {
            let assign37450_e48764: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign37450_e48766: f64 = (assign37450_e48764 * locals.var_vbirgatinv_d);
            let assign37450_e48768: f64 = (assign37450_e48766).powf(locals.var_pgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37450_e48768, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard755 == 0.0)) {
            let assign37460_e48783: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign37460_e48785: f64 = (assign37460_e48783 * locals.var_wdepnulrinvgat_d);
            let assign37460_e48787: f64 = (assign37460_e48785 / locals.var_tmp);
            let assign37460_e48788: f64 = (locals.var_one_over_one_minus_pgat_d * assign37460_e48787);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign37460_e48788, (locals.var_one_over_one_minus_pgat_d * (-((assign37460_e48785 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat_d * (-((assign37460_e48785 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat_d * (-((assign37460_e48785 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat_d * (-((assign37460_e48785 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign37470_e48792: f64 = (-locals.var_fbbtgat_d);
        let assign37470_e48794: f64 = (assign37470_e48792 / locals.var_fmaxr);
        let assign37470_e48795: f64 = (assign37470_e48794).abs();
        let assign37470_e48797: f64 = if assign37470_e48795 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard757 = assign37470_e48797;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard755 == 0.0)) && (locals.var_guard757 != 0.0)) {
            let assign37480_e48810: f64 = (-locals.var_fbbtgat_d);
            let assign37480_e48812: f64 = (assign37480_e48810 / locals.var_fmaxr);
            let assign37480_e48813: f64 = (assign37480_e48812).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37480_e48813, (assign37480_e48813 * ((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign37480_e48810 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign37480_e48813 * ((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign37480_e48810 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign37480_e48813 * ((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign37480_e48810 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign37480_e48813 * ((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign37480_e48810 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))), );
        }
        let assign37490_e48817: f64 = (-locals.var_fbbtgat_d);
        let assign37490_e48819: f64 = (assign37490_e48817 / locals.var_fmaxr);
        let assign37490_e48821: f64 = if assign37490_e48819 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard758 = assign37490_e48821;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard755 == 0.0)) && (locals.var_guard757 == 0.0)) && (locals.var_guard758 != 0.0)) {
            let assign37500_e48839: f64 = (-230.25850929940458);
            let assign37500_e48841: f64 = (-locals.var_fbbtgat_d);
            let assign37500_e48843: f64 = (assign37500_e48841 / locals.var_fmaxr);
            let assign37500_e48844: f64 = (assign37500_e48839 - assign37500_e48843);
            let assign37500_e48848: f64 = (-230.25850929940458);
            let assign37500_e48850: f64 = (-locals.var_fbbtgat_d);
            let assign37500_e48852: f64 = (assign37500_e48850 / locals.var_fmaxr);
            let assign37500_e48853: f64 = (assign37500_e48848 - assign37500_e48852);
            let assign37500_e48856: f64 = (-230.25850929940458);
            let assign37500_e48858: f64 = (-locals.var_fbbtgat_d);
            let assign37500_e48860: f64 = (assign37500_e48858 / locals.var_fmaxr);
            let assign37500_e48861: f64 = (assign37500_e48856 - assign37500_e48860);
            let assign37500_e48863: f64 = (assign37500_e48861 * 0.3333333333333333);
            let assign37500_e48864: f64 = (1.0 + assign37500_e48863);
            let assign37500_e48865: f64 = (assign37500_e48853 * assign37500_e48864);
            let assign37500_e48866: f64 = (0.5 * assign37500_e48865);
            let assign37500_e48867: f64 = (1.0 + assign37500_e48866);
            let assign37500_e48868: f64 = (assign37500_e48844 * assign37500_e48867);
            let assign37500_e48869: f64 = (1.0 + assign37500_e48868);
            let assign37500_e48870: f64 = (1e-100 / assign37500_e48869);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37500_e48870, (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign37500_e48841 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign37500_e48867) + (assign37500_e48844 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign37500_e48850 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign37500_e48864) + (assign37500_e48853 * ((-((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign37500_e48858 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign37500_e48869 * assign37500_e48869))), (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign37500_e48841 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign37500_e48867) + (assign37500_e48844 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign37500_e48850 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign37500_e48864) + (assign37500_e48853 * ((-((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign37500_e48858 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign37500_e48869 * assign37500_e48869))), (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign37500_e48841 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign37500_e48867) + (assign37500_e48844 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign37500_e48850 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign37500_e48864) + (assign37500_e48853 * ((-((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign37500_e48858 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign37500_e48869 * assign37500_e48869))), (-((1e-100 * (((-((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign37500_e48841 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign37500_e48867) + (assign37500_e48844 * (0.5 * (((-((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign37500_e48850 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign37500_e48864) + (assign37500_e48853 * ((-((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign37500_e48858 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign37500_e48869 * assign37500_e48869))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard755 == 0.0)) && (locals.var_guard757 == 0.0)) && (locals.var_guard758 == 0.0)) {
            let assign37510_e48891: f64 = (-locals.var_fbbtgat_d);
            let assign37510_e48893: f64 = (assign37510_e48891 / locals.var_fmaxr);
            let assign37510_e48895: f64 = (assign37510_e48893 - 230.25850929940458);
            let assign37510_e48899: f64 = (-locals.var_fbbtgat_d);
            let assign37510_e48901: f64 = (assign37510_e48899 / locals.var_fmaxr);
            let assign37510_e48903: f64 = (assign37510_e48901 - 230.25850929940458);
            let assign37510_e48906: f64 = (-locals.var_fbbtgat_d);
            let assign37510_e48908: f64 = (assign37510_e48906 / locals.var_fmaxr);
            let assign37510_e48910: f64 = (assign37510_e48908 - 230.25850929940458);
            let assign37510_e48912: f64 = (assign37510_e48910 * 0.3333333333333333);
            let assign37510_e48913: f64 = (1.0 + assign37510_e48912);
            let assign37510_e48914: f64 = (assign37510_e48903 * assign37510_e48913);
            let assign37510_e48915: f64 = (0.5 * assign37510_e48914);
            let assign37510_e48916: f64 = (1.0 + assign37510_e48915);
            let assign37510_e48917: f64 = (assign37510_e48895 * assign37510_e48916);
            let assign37510_e48918: f64 = (1.0 + assign37510_e48917);
            let assign37510_e48919: f64 = (1e100 * assign37510_e48918);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37510_e48919, (1e100 * ((((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign37510_e48891 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign37510_e48916) + (assign37510_e48895 * (0.5 * ((((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign37510_e48899 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign37510_e48913) + (assign37510_e48903 * (((((-locals.var_fbbtgat_d_dn5) * locals.var_fmaxr) - (assign37510_e48906 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign37510_e48891 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign37510_e48916) + (assign37510_e48895 * (0.5 * ((((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign37510_e48899 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign37510_e48913) + (assign37510_e48903 * (((((-locals.var_fbbtgat_d_dn6) * locals.var_fmaxr) - (assign37510_e48906 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign37510_e48891 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign37510_e48916) + (assign37510_e48895 * (0.5 * ((((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign37510_e48899 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign37510_e48913) + (assign37510_e48903 * (((((-locals.var_fbbtgat_d_dn7) * locals.var_fmaxr) - (assign37510_e48906 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign37510_e48891 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign37510_e48916) + (assign37510_e48895 * (0.5 * ((((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign37510_e48899 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign37510_e48913) + (assign37510_e48903 * (((((-locals.var_fbbtgat_d_dn8) * locals.var_fmaxr) - (assign37510_e48906 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard755 == 0.0)) {
            let assign37520_e48934: f64 = (locals.var_v4 * locals.var_fmaxr);
            let assign37520_e48936: f64 = (assign37520_e48934 * locals.var_fmaxr);
            let assign37520_e48938: f64 = (assign37520_e48936 * locals.var_tmp);
            let assign37520_e48939: f64 = (locals.var_cbbtgatd_i * assign37520_e48938);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign37520_e48939, (locals.var_cbbtgatd_i * (((((locals.var_v4 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign37520_e48934 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign37520_e48936 * locals.var_tmp_dn5))), (locals.var_cbbtgatd_i * (((((locals.var_v4 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign37520_e48934 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign37520_e48936 * locals.var_tmp_dn6))), (locals.var_cbbtgatd_i * (((((locals.var_v4 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign37520_e48934 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign37520_e48936 * locals.var_tmp_dn7))), (locals.var_cbbtgatd_i * (((((locals.var_v4 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign37520_e48934 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign37520_e48936 * locals.var_tmp_dn8))), );
        }
        let assign37530_e48944: f64 = if locals.var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard759 = assign37530_e48944;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard759 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign37550_e48958: f64 = (-locals.var_alphaav);
        let assign37550_e48960: f64 = (assign37550_e48958 * locals.var_vbrgatd_i);
        let assign37550_e48961: f64 = if locals.var_vav > assign37550_e48960 { 1.0 } else { 0.0 };
        locals.var_guard760 = assign37550_e48961;
        let assign37560_e48964: f64 = if locals.var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard761 = assign37560_e48964;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard759 == 0.0)) && (locals.var_guard760 != 0.0)) && (locals.var_guard761 != 0.0)) {
            let assign37570_e48980: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign37570_e48983: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign37570_e48984: f64 = (assign37570_e48980 * assign37570_e48983);
            let assign37570_e48987: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign37570_e48988: f64 = (assign37570_e48984 * assign37570_e48987);
            let assign37570_e48991: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign37570_e48992: f64 = (assign37570_e48988 * assign37570_e48991);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37570_e48992, (((((((locals.var_vav * locals.var_vbrinvgat_d_dn5) * assign37570_e48983) + (assign37570_e48980 * (locals.var_vav * locals.var_vbrinvgat_d_dn5))) * assign37570_e48987) + (assign37570_e48984 * (locals.var_vav * locals.var_vbrinvgat_d_dn5))) * assign37570_e48991) + (assign37570_e48988 * (locals.var_vav * locals.var_vbrinvgat_d_dn5))), (((((((locals.var_vav * locals.var_vbrinvgat_d_dn6) * assign37570_e48983) + (assign37570_e48980 * (locals.var_vav * locals.var_vbrinvgat_d_dn6))) * assign37570_e48987) + (assign37570_e48984 * (locals.var_vav * locals.var_vbrinvgat_d_dn6))) * assign37570_e48991) + (assign37570_e48988 * (locals.var_vav * locals.var_vbrinvgat_d_dn6))), (((((((locals.var_vav * locals.var_vbrinvgat_d_dn7) * assign37570_e48983) + (assign37570_e48980 * (locals.var_vav * locals.var_vbrinvgat_d_dn7))) * assign37570_e48987) + (assign37570_e48984 * (locals.var_vav * locals.var_vbrinvgat_d_dn7))) * assign37570_e48991) + (assign37570_e48988 * (locals.var_vav * locals.var_vbrinvgat_d_dn7))), (((((((locals.var_vav * locals.var_vbrinvgat_d_dn8) * assign37570_e48983) + (assign37570_e48980 * (locals.var_vav * locals.var_vbrinvgat_d_dn8))) * assign37570_e48987) + (assign37570_e48984 * (locals.var_vav * locals.var_vbrinvgat_d_dn8))) * assign37570_e48991) + (assign37570_e48988 * (locals.var_vav * locals.var_vbrinvgat_d_dn8))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard759 == 0.0)) && (locals.var_guard760 != 0.0)) && (locals.var_guard761 == 0.0)) {
            let assign37580_e49011: f64 = (locals.var_vav * locals.var_vbrinvgat_d);
            let assign37580_e49012: f64 = (assign37580_e49011).abs();
            let assign37580_e49014: f64 = (assign37580_e49012).powf(locals.var_pbrgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign37580_e49014, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign37580_e49012).powf(locals.var_pbrgatd_i - 1.0) * if assign37580_e49011 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn5)) })) } } else { (assign37580_e49014 * (locals.var_pbrgatd_i * (if assign37580_e49011 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn5)) } / assign37580_e49012))) }, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign37580_e49012).powf(locals.var_pbrgatd_i - 1.0) * if assign37580_e49011 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn6)) })) } } else { (assign37580_e49014 * (locals.var_pbrgatd_i * (if assign37580_e49011 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn6)) } / assign37580_e49012))) }, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign37580_e49012).powf(locals.var_pbrgatd_i - 1.0) * if assign37580_e49011 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn7)) })) } } else { (assign37580_e49014 * (locals.var_pbrgatd_i * (if assign37580_e49011 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn7)) } / assign37580_e49012))) }, if 0.0 == 0.0 && ((locals.var_pbrgatd_i) as f64).is_finite() && ((locals.var_pbrgatd_i) as f64).fract() == 0.0 { if locals.var_pbrgatd_i == 0.0 { 0.0 } else { (locals.var_pbrgatd_i * ((assign37580_e49012).powf(locals.var_pbrgatd_i - 1.0) * if assign37580_e49011 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn8)) })) } } else { (assign37580_e49014 * (locals.var_pbrgatd_i * (if assign37580_e49011 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_d_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_d_dn8)) } / assign37580_e49012))) }, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard759 == 0.0)) && (locals.var_guard760 != 0.0)) {
            let assign37590_e49031: f64 = (1.0 - locals.var_tmp);
            let assign37590_e49032: f64 = (1.0 / assign37590_e49031);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign37590_e49032, (-((-locals.var_tmp_dn5) / (assign37590_e49031 * assign37590_e49031))), (-((-locals.var_tmp_dn6) / (assign37590_e49031 * assign37590_e49031))), (-((-locals.var_tmp_dn7) / (assign37590_e49031 * assign37590_e49031))), (-((-locals.var_tmp_dn8) / (assign37590_e49031 * assign37590_e49031))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard759 == 0.0)) && (locals.var_guard760 == 0.0)) {
            let assign37600_e49051: f64 = (locals.var_alphaav * locals.var_vbrgatd_i);
            let assign37600_e49052: f64 = (locals.var_vav + assign37600_e49051);
            let assign37600_e49054: f64 = (assign37600_e49052 * locals.var_slopegat_d);
            let assign37600_e49055: f64 = (locals.var_fstopgat_d + assign37600_e49054);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign37600_e49055, (assign37600_e49052 * locals.var_slopegat_d_dn5), (assign37600_e49052 * locals.var_slopegat_d_dn6), (assign37600_e49052 * locals.var_slopegat_d_dn7), (assign37600_e49052 * locals.var_slopegat_d_dn8), );
        }
    }
    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard745 == 0.0)) {
            let assign37610_e49067: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign37610_e49069: f64 = (assign37610_e49067 + locals.var_itat);
            let assign37610_e49071: f64 = (assign37610_e49069 + locals.var_ibbt);
            let assign37610_e49072: f64 = (p.p29 * assign37610_e49071);
            let assign37610_e49074: f64 = (assign37610_e49072 * locals.var_fbreakdown);
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (assign37610_e49074, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign37610_e49072 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign37610_e49072 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign37610_e49072 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign37610_e49072 * locals.var_fbreakdown_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign37620_e49082: f64 = (locals.var_abdrain_i * locals.var_ijunbot);
            let assign37620_e49085: f64 = (locals.var_lsdrain_i * locals.var_ijunsti);
            let assign37620_e49086: f64 = (assign37620_e49082 + assign37620_e49085);
            let assign37620_e49089: f64 = (locals.var_lgdrain_i * locals.var_ijungat);
            let assign37620_e49090: f64 = (assign37620_e49086 + assign37620_e49089);
            (locals.var_i4, locals.var_i4_dn5, locals.var_i4_dn6, locals.var_i4_dn7, locals.var_i4_dn8, ) = (assign37620_e49090, (((locals.var_abdrain_i * locals.var_ijunbot_dn5) + (locals.var_lsdrain_i * locals.var_ijunsti_dn5)) + (locals.var_lgdrain_i * locals.var_ijungat_dn5)), (((locals.var_abdrain_i * locals.var_ijunbot_dn6) + (locals.var_lsdrain_i * locals.var_ijunsti_dn6)) + (locals.var_lgdrain_i * locals.var_ijungat_dn6)), (((locals.var_abdrain_i * locals.var_ijunbot_dn7) + (locals.var_lsdrain_i * locals.var_ijunsti_dn7)) + (locals.var_lgdrain_i * locals.var_ijungat_dn7)), (((locals.var_abdrain_i * locals.var_ijunbot_dn8) + (locals.var_lsdrain_i * locals.var_ijunsti_dn8)) + (locals.var_lgdrain_i * locals.var_ijungat_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            locals.var_vbbt = 0.0;
            locals.var_two_psistar = 0.0;
        }
        let assign37650_e49116: f64 = if (!(((locals.var_abdrain_i == 0.0) && (locals.var_lsdrain_i == 0.0)) && (locals.var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard762 = assign37650_e49116;
        let assign37730_e49202: f64 = if locals.var_v5 < locals.var_vmax_d { 1.0 } else { 0.0 };
        locals.var_guard763 = assign37730_e49202;
        let assign37740_e49204: f64 = (-0.5);
        let assign37740_e49207: f64 = (locals.var_v5 * locals.var_phitdinv);
        let assign37740_e49208: f64 = (assign37740_e49204 * assign37740_e49207);
        let assign37740_e49209: f64 = (assign37740_e49208).abs();
        let assign37740_e49211: f64 = if assign37740_e49209 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard764 = assign37740_e49211;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) && (locals.var_guard763 != 0.0)) && (locals.var_guard764 != 0.0)) {
            let assign37750_e49222: f64 = (-0.5);
            let assign37750_e49225: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign37750_e49226: f64 = (assign37750_e49222 * assign37750_e49225);
            let assign37750_e49227: f64 = (assign37750_e49226).exp();
            locals.var_z = assign37750_e49227;
        }
        let assign37760_e49231: f64 = (-0.5);
        let assign37760_e49234: f64 = (locals.var_v5 * locals.var_phitdinv);
        let assign37760_e49235: f64 = (assign37760_e49231 * assign37760_e49234);
        let assign37760_e49237: f64 = if assign37760_e49235 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard765 = assign37760_e49237;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) && (locals.var_guard763 != 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 != 0.0)) {
            let assign37770_e49253: f64 = (-230.25850929940458);
            let assign37770_e49255: f64 = (-0.5);
            let assign37770_e49258: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign37770_e49259: f64 = (assign37770_e49255 * assign37770_e49258);
            let assign37770_e49260: f64 = (assign37770_e49253 - assign37770_e49259);
            let assign37770_e49264: f64 = (-230.25850929940458);
            let assign37770_e49266: f64 = (-0.5);
            let assign37770_e49269: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign37770_e49270: f64 = (assign37770_e49266 * assign37770_e49269);
            let assign37770_e49271: f64 = (assign37770_e49264 - assign37770_e49270);
            let assign37770_e49274: f64 = (-230.25850929940458);
            let assign37770_e49276: f64 = (-0.5);
            let assign37770_e49279: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign37770_e49280: f64 = (assign37770_e49276 * assign37770_e49279);
            let assign37770_e49281: f64 = (assign37770_e49274 - assign37770_e49280);
            let assign37770_e49283: f64 = (assign37770_e49281 * 0.3333333333333333);
            let assign37770_e49284: f64 = (1.0 + assign37770_e49283);
            let assign37770_e49285: f64 = (assign37770_e49271 * assign37770_e49284);
            let assign37770_e49286: f64 = (0.5 * assign37770_e49285);
            let assign37770_e49287: f64 = (1.0 + assign37770_e49286);
            let assign37770_e49288: f64 = (assign37770_e49260 * assign37770_e49287);
            let assign37770_e49289: f64 = (1.0 + assign37770_e49288);
            let assign37770_e49290: f64 = (1e-100 / assign37770_e49289);
            locals.var_z = assign37770_e49290;
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) && (locals.var_guard763 != 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 == 0.0)) {
            let assign37780_e49309: f64 = (-0.5);
            let assign37780_e49312: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign37780_e49313: f64 = (assign37780_e49309 * assign37780_e49312);
            let assign37780_e49315: f64 = (assign37780_e49313 - 230.25850929940458);
            let assign37780_e49319: f64 = (-0.5);
            let assign37780_e49322: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign37780_e49323: f64 = (assign37780_e49319 * assign37780_e49322);
            let assign37780_e49325: f64 = (assign37780_e49323 - 230.25850929940458);
            let assign37780_e49328: f64 = (-0.5);
            let assign37780_e49331: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign37780_e49332: f64 = (assign37780_e49328 * assign37780_e49331);
            let assign37780_e49334: f64 = (assign37780_e49332 - 230.25850929940458);
            let assign37780_e49336: f64 = (assign37780_e49334 * 0.3333333333333333);
            let assign37780_e49337: f64 = (1.0 + assign37780_e49336);
            let assign37780_e49338: f64 = (assign37780_e49325 * assign37780_e49337);
            let assign37780_e49339: f64 = (0.5 * assign37780_e49338);
            let assign37780_e49340: f64 = (1.0 + assign37780_e49339);
            let assign37780_e49341: f64 = (assign37780_e49315 * assign37780_e49340);
            let assign37780_e49342: f64 = (1.0 + assign37780_e49341);
            let assign37780_e49343: f64 = (1e100 * assign37780_e49342);
            locals.var_z = assign37780_e49343;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) && (locals.var_guard763 != 0.0)) {
            let assign37790_e49355: f64 = (1.0 / locals.var_z);
            locals.var_zinv = assign37790_e49355;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) && (locals.var_guard763 != 0.0)) {
            let assign37800_e49367: f64 = (locals.var_zinv * locals.var_zinv);
            locals.var_idmult = assign37800_e49367;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) && (locals.var_guard763 == 0.0)) {
            let assign37810_e49381: f64 = (locals.var_v5 - locals.var_vmax_d);
            let assign37810_e49383: f64 = (assign37810_e49381 * locals.var_phitdinv);
            let assign37810_e49384: f64 = (1.0 + assign37810_e49383);
            let assign37810_e49386: f64 = (assign37810_e49384 * locals.var_exp_vmax_over_phitd_d);
            locals.var_idmult = assign37810_e49386;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) && (locals.var_guard763 == 0.0)) {
            let assign37820_e49398: f64 = (locals.var_idmult).sqrt();
            locals.var_zinv = assign37820_e49398;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) && (locals.var_guard763 == 0.0)) {
            let assign37830_e49411: f64 = (1.0 / locals.var_zinv);
            locals.var_z = assign37830_e49411;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) {
            let assign37840_e49421: f64 = (locals.var_idmult - 1.0);
            locals.var_idmult = assign37840_e49421;
        }
        let assign37850_e49426: f64 = if locals.var_v5 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard766 = assign37850_e49426;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) && (locals.var_guard766 != 0.0)) {
            let assign37860_e49438: f64 = (2.0 + locals.var_z);
            let assign37860_e49441: f64 = (locals.var_z + 1.0);
            let assign37860_e49444: f64 = (locals.var_z + 3.0);
            let assign37860_e49445: f64 = (assign37860_e49441 * assign37860_e49444);
            let assign37860_e49446: f64 = (assign37860_e49445).sqrt();
            let assign37860_e49447: f64 = (assign37860_e49438 + assign37860_e49446);
            let assign37860_e49448: f64 = (assign37860_e49447).ln();
            let assign37860_e49449: f64 = (locals.var_phitd * assign37860_e49448);
            let assign37860_e49450: f64 = (2.0 * assign37860_e49449);
            locals.var_two_psistar = assign37860_e49450;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) && (locals.var_guard766 == 0.0)) {
            let assign37870_e49462: f64 = (-locals.var_v5);
            let assign37870_e49467: f64 = (2.0 * locals.var_zinv);
            let assign37870_e49469: f64 = (assign37870_e49467 + 1.0);
            let assign37870_e49472: f64 = (1.0 + locals.var_zinv);
            let assign37870_e49476: f64 = (3.0 * locals.var_zinv);
            let assign37870_e49477: f64 = (1.0 + assign37870_e49476);
            let assign37870_e49478: f64 = (assign37870_e49472 * assign37870_e49477);
            let assign37870_e49479: f64 = (assign37870_e49478).sqrt();
            let assign37870_e49480: f64 = (assign37870_e49469 + assign37870_e49479);
            let assign37870_e49481: f64 = (assign37870_e49480).ln();
            let assign37870_e49482: f64 = (locals.var_phitd * assign37870_e49481);
            let assign37870_e49483: f64 = (2.0 * assign37870_e49482);
            let assign37870_e49484: f64 = (assign37870_e49462 + assign37870_e49483);
            locals.var_two_psistar = assign37870_e49484;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) {
            let assign37880_e49494: f64 = (locals.var_vbimin_d - locals.var_two_psistar);
            locals.var_vjlim = assign37880_e49494;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) {
            let assign37890_e49505: f64 = (locals.var_v5 + locals.var_vjlim);
            let assign37890_e49508: f64 = (locals.var_v5 - locals.var_vjlim);
            let assign37890_e49511: f64 = (locals.var_v5 - locals.var_vjlim);
            let assign37890_e49512: f64 = (assign37890_e49508 * assign37890_e49511);
            let assign37890_e49515: f64 = (4.0 * locals.var_phitd);
            let assign37890_e49517: f64 = (assign37890_e49515 * locals.var_phitd);
            let assign37890_e49518: f64 = (assign37890_e49512 + assign37890_e49517);
            let assign37890_e49519: f64 = (assign37890_e49518).sqrt();
            let assign37890_e49520: f64 = (assign37890_e49505 - assign37890_e49519);
            let assign37890_e49521: f64 = (0.5 * assign37890_e49520);
            locals.var_vjsrh = assign37890_e49521;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) {
            let assign37900_e49532: f64 = (locals.var_v5 + locals.var_vbbtlim_d);
            let assign37900_e49535: f64 = (locals.var_v5 - locals.var_vbbtlim_d);
            let assign37900_e49538: f64 = (locals.var_v5 - locals.var_vbbtlim_d);
            let assign37900_e49539: f64 = (assign37900_e49535 * assign37900_e49538);
            let assign37900_e49542: f64 = (4.0 * locals.var_phitr);
            let assign37900_e49544: f64 = (assign37900_e49542 * locals.var_phitr);
            let assign37900_e49545: f64 = (assign37900_e49539 + assign37900_e49544);
            let assign37900_e49546: f64 = (assign37900_e49545).sqrt();
            let assign37900_e49547: f64 = (assign37900_e49532 - assign37900_e49546);
            let assign37900_e49548: f64 = (0.5 * assign37900_e49547);
            locals.var_vbbt = assign37900_e49548;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard762 != 0.0)) {
            let assign37910_e49559: f64 = locals.var_v5;
            let assign37910_e49562: f64 = locals.var_v5;
            let assign37910_e49565: f64 = locals.var_v5;
            let assign37910_e49566: f64 = (assign37910_e49562 * assign37910_e49565);
            let assign37910_e49569: f64 = (4.0 * 1e-6);
            let assign37910_e49571: f64 = (assign37910_e49569 * 1e-6);
            let assign37910_e49572: f64 = (assign37910_e49566 + assign37910_e49571);
            let assign37910_e49573: f64 = (assign37910_e49572).sqrt();
            let assign37910_e49574: f64 = (assign37910_e49559 - assign37910_e49573);
            let assign37910_e49575: f64 = (0.5 * assign37910_e49574);
            locals.var_vav = assign37910_e49575;
        }
        let assign37920_e49580: f64 = if locals.var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard767 = assign37920_e49580;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 != 0.0)) {
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) {
            let assign37940_e49597: f64 = (locals.var_idsatbot_d * locals.var_idmult);
            locals.var_id__blk219 = assign37940_e49597;
        }
        let assign37950_e49606: f64 = if ((locals.var_csrhbotd_i == 0.0) && (locals.var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard768 = assign37950_e49606;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
            let assign37970_e49629: f64 = (locals.var_vbibot_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign37970_e49629;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
            let assign37980_e49645: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign37980_e49646: f64 = (1.0 - assign37980_e49645);
            let assign37980_e49647: f64 = (assign37980_e49646).sqrt();
            let assign37980_e49648: f64 = (1.0 - assign37980_e49647);
            locals.var_wsrhstep = assign37980_e49648;
        }
        let assign37990_e49653: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard769 = assign37990_e49653;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) && (locals.var_guard769 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) && (locals.var_guard769 == 0.0)) {
            let assign38010_e49682: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign38010_e49684: f64 = (locals.var_wsrhstep).ln();
            let assign38010_e49685: f64 = (assign38010_e49682 * assign38010_e49684);
            let assign38010_e49688: f64 = (1.0 - locals.var_wsrhstep);
            let assign38010_e49689: f64 = (assign38010_e49685 / assign38010_e49688);
            let assign38010_e49691: f64 = (assign38010_e49689 + locals.var_wsrhstep);
            let assign38010_e49695: f64 = (2.0 * locals.var_pbotd_i);
            let assign38010_e49696: f64 = (1.0 - assign38010_e49695);
            let assign38010_e49697: f64 = (assign38010_e49691 * assign38010_e49696);
            locals.var_dwsrh = assign38010_e49697;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
            let assign38020_e49711: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign38020_e49711;
        }
        let assign38030_e49716: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign38030_e49716;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) && (locals.var_guard770 != 0.0)) {
            let assign38040_e49730: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv_d);
            let assign38040_e49731: f64 = (assign38040_e49730).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38040_e49731, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) && (locals.var_guard770 == 0.0)) {
            let assign38050_e49748: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv_d);
            let assign38050_e49750: f64 = (assign38050_e49748).powf(locals.var_pbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38050_e49750, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
            let assign38060_e49764: f64 = (locals.var_wdepnulrbot_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign38060_e49764, (locals.var_wdepnulrbot_d * locals.var_tmp_dn5), (locals.var_wdepnulrbot_d * locals.var_tmp_dn6), (locals.var_wdepnulrbot_d * locals.var_tmp_dn7), (locals.var_wdepnulrbot_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
            let assign38070_e49779: f64 = (locals.var_zinv - 1.0);
            let assign38070_e49781: f64 = (assign38070_e49779 * locals.var_wdep);
            let assign38070_e49782: f64 = (locals.var_ftdbot_d * assign38070_e49781);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign38070_e49782, (locals.var_ftdbot_d * (assign38070_e49779 * locals.var_wdep_dn5)), (locals.var_ftdbot_d * (assign38070_e49779 * locals.var_wdep_dn6)), (locals.var_ftdbot_d * (assign38070_e49779 * locals.var_wdep_dn7)), (locals.var_ftdbot_d * (assign38070_e49779 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
            let assign38080_e49797: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign38080_e49798: f64 = (locals.var_csrhbotd_i * assign38080_e49797);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign38080_e49798, (locals.var_csrhbotd_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhbotd_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign38090_e49803: f64 = if locals.var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard771 = assign38090_e49803;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38110_e49827: f64 = (locals.var_wdep * locals.var_one_minus_pbot_d);
            let assign38110_e49829: f64 = (assign38110_e49827 / locals.var_vbi_minus_vjsrh);
            let assign38110_e49830: f64 = (locals.var_btatpartbot_d * assign38110_e49829);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign38110_e49830, (locals.var_btatpartbot_d * ((locals.var_wdep_dn5 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn6 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn7 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot_d * ((locals.var_wdep_dn8 * locals.var_one_minus_pbot_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38120_e49844: f64 = (0.666666666666667 * locals.var_atatbot_d);
            let assign38120_e49846: f64 = (assign38120_e49844 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign38120_e49846, (-((assign38120_e49844 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign38120_e49844 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign38120_e49844 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign38120_e49844 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38130_e49860: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign38130_e49860, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38140_e49874: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign38140_e49877: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign38140_e49879: f64 = (assign38140_e49877 + 1.0);
            let assign38140_e49880: f64 = (assign38140_e49874 / assign38140_e49879);
            let assign38140_e49881: f64 = (assign38140_e49880).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign38140_e49881, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign38140_e49879) - (assign38140_e49874 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign38140_e49879 * assign38140_e49879)) / (2.0 * assign38140_e49881)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign38140_e49879) - (assign38140_e49874 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign38140_e49879 * assign38140_e49879)) / (2.0 * assign38140_e49881)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign38140_e49879) - (assign38140_e49874 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign38140_e49879 * assign38140_e49879)) / (2.0 * assign38140_e49881)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign38140_e49879) - (assign38140_e49874 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign38140_e49879 * assign38140_e49879)) / (2.0 * assign38140_e49881)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38150_e49894: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign38150_e49894, (locals.var_umax_dn5 / (2.0 * assign38150_e49894)), (locals.var_umax_dn6 / (2.0 * assign38150_e49894)), (locals.var_umax_dn7 / (2.0 * assign38150_e49894)), (locals.var_umax_dn8 / (2.0 * assign38150_e49894)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38160_e49908: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign38160_e49908, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign38170_e49912: f64 = (-locals.var_pbotd_i);
        let assign38170_e49914: f64 = (assign38170_e49912 * locals.var_one_over_one_minus_pbot_d);
        let assign38170_e49916: f64 = (-1.0);
        let assign38170_e49917: f64 = if assign38170_e49914 == assign38170_e49916 { 1.0 } else { 0.0 };
        locals.var_guard772 = assign38170_e49917;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard772 != 0.0)) {
            let assign38180_e49933: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign38180_e49934: f64 = (1.0 + assign38180_e49933);
            let assign38180_e49935: f64 = (1.0 / assign38180_e49934);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign38180_e49935, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign38180_e49934 * assign38180_e49934))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign38180_e49934 * assign38180_e49934))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign38180_e49934 * assign38180_e49934))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign38180_e49934 * assign38180_e49934))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard772 == 0.0)) {
            let assign38190_e49953: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign38190_e49954: f64 = (1.0 + assign38190_e49953);
            let assign38190_e49956: f64 = (-locals.var_pbotd_i);
            let assign38190_e49958: f64 = (assign38190_e49956 * locals.var_one_over_one_minus_pbot_d);
            let assign38190_e49959: f64 = (assign38190_e49954).powf(assign38190_e49958);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign38190_e49959, if 0.0 == 0.0 && ((assign38190_e49958) as f64).is_finite() && ((assign38190_e49958) as f64).fract() == 0.0 { if assign38190_e49958 == 0.0 { 0.0 } else { (assign38190_e49958 * ((assign38190_e49954).powf(assign38190_e49958 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign38190_e49959 * (assign38190_e49958 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign38190_e49954))) }, if 0.0 == 0.0 && ((assign38190_e49958) as f64).is_finite() && ((assign38190_e49958) as f64).fract() == 0.0 { if assign38190_e49958 == 0.0 { 0.0 } else { (assign38190_e49958 * ((assign38190_e49954).powf(assign38190_e49958 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign38190_e49959 * (assign38190_e49958 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign38190_e49954))) }, if 0.0 == 0.0 && ((assign38190_e49958) as f64).is_finite() && ((assign38190_e49958) as f64).fract() == 0.0 { if assign38190_e49958 == 0.0 { 0.0 } else { (assign38190_e49958 * ((assign38190_e49954).powf(assign38190_e49958 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign38190_e49959 * (assign38190_e49958 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign38190_e49954))) }, if 0.0 == 0.0 && ((assign38190_e49958) as f64).is_finite() && ((assign38190_e49958) as f64).fract() == 0.0 { if assign38190_e49958 == 0.0 { 0.0 } else { (assign38190_e49958 * ((assign38190_e49954).powf(assign38190_e49958 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign38190_e49959 * (assign38190_e49958 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign38190_e49954))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38200_e49973: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign38200_e49976: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign38200_e49977: f64 = (assign38200_e49973 / assign38200_e49976);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign38200_e49977, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign38200_e49976) - (assign38200_e49973 * locals.var_wgamma_dn5)) / (assign38200_e49976 * assign38200_e49976)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign38200_e49976) - (assign38200_e49973 * locals.var_wgamma_dn6)) / (assign38200_e49976 * assign38200_e49976)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign38200_e49976) - (assign38200_e49973 * locals.var_wgamma_dn7)) / (assign38200_e49976 * assign38200_e49976)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign38200_e49976) - (assign38200_e49973 * locals.var_wgamma_dn8)) / (assign38200_e49976 * assign38200_e49976)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38210_e49992: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign38210_e49993: f64 = (0.375 * assign38210_e49992);
            let assign38210_e49994: f64 = (assign38210_e49993).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign38210_e49994, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign38210_e49994)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign38210_e49994)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign38210_e49994)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign38210_e49994)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38220_e50009: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign38220_e50010: f64 = (2.0 * assign38220_e50009);
            let assign38220_e50012: f64 = (assign38220_e50010 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign38220_e50012, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38230_e50026: f64 = (locals.var_atatbot_d * locals.var_twoatatoverthreebtat);
            let assign38230_e50028: f64 = (assign38230_e50026 * locals.var_sqrtumax);
            let assign38230_e50031: f64 = (locals.var_atatbot_d * locals.var_umax);
            let assign38230_e50032: f64 = (assign38230_e50028 - assign38230_e50031);
            let assign38230_e50036: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign38230_e50037: f64 = (0.5 * assign38230_e50036);
            let assign38230_e50038: f64 = (assign38230_e50032 + assign38230_e50037);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign38230_e50038, (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign38230_e50026 * locals.var_sqrtumax_dn5)) - (locals.var_atatbot_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign38230_e50026 * locals.var_sqrtumax_dn6)) - (locals.var_atatbot_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign38230_e50026 * locals.var_sqrtumax_dn7)) - (locals.var_atatbot_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatbot_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign38230_e50026 * locals.var_sqrtumax_dn8)) - (locals.var_atatbot_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38240_e50052: f64 = (locals.var_ltat - 1.0);
            let assign38240_e50054: f64 = (assign38240_e50052 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign38240_e50054, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign38240_e50052 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign38240_e50052 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign38240_e50052 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign38240_e50052 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38250_e50068: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign38250_e50068, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign38260_e50073: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard773 = assign38260_e50073;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 != 0.0)) {
            let assign38270_e50089: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign38270_e50090: f64 = (1.0 + assign38270_e50089);
            let assign38270_e50091: f64 = (1.0 / assign38270_e50090);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign38270_e50091, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign38270_e50090 * assign38270_e50090))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign38270_e50090 * assign38270_e50090))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign38270_e50090 * assign38270_e50090))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign38270_e50090 * assign38270_e50090))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
            let assign38280_e50110: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign38280_e50111: f64 = (1.0 - assign38280_e50110);
            let assign38280_e50112: f64 = (1.0 / assign38280_e50111);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign38280_e50112, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign38280_e50111 * assign38280_e50111))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign38280_e50111 * assign38280_e50111))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign38280_e50111 * assign38280_e50111))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign38280_e50111 * assign38280_e50111))), );
        }
        let assign38290_e50116: f64 = (-locals.var_ysq);
        let assign38290_e50118: f64 = (assign38290_e50116 + locals.var_mtat);
        let assign38290_e50120: f64 = (-230.25850929940458);
        let assign38290_e50121: f64 = if assign38290_e50118 > assign38290_e50120 { 1.0 } else { 0.0 };
        locals.var_guard774 = assign38290_e50121;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard774 != 0.0)) {
            let assign38300_e50134: f64 = (-locals.var_ysq);
            let assign38300_e50136: f64 = (assign38300_e50134 + locals.var_mtat);
            let assign38300_e50137: f64 = (assign38300_e50136).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38300_e50137, (assign38300_e50137 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign38300_e50137 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign38300_e50137 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign38300_e50137 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard774 == 0.0)) {
            let assign38310_e50155: f64 = (-230.25850929940458);
            let assign38310_e50157: f64 = (-locals.var_ysq);
            let assign38310_e50159: f64 = (assign38310_e50157 + locals.var_mtat);
            let assign38310_e50160: f64 = (assign38310_e50155 - assign38310_e50159);
            let assign38310_e50164: f64 = (-230.25850929940458);
            let assign38310_e50166: f64 = (-locals.var_ysq);
            let assign38310_e50168: f64 = (assign38310_e50166 + locals.var_mtat);
            let assign38310_e50169: f64 = (assign38310_e50164 - assign38310_e50168);
            let assign38310_e50172: f64 = (-230.25850929940458);
            let assign38310_e50174: f64 = (-locals.var_ysq);
            let assign38310_e50176: f64 = (assign38310_e50174 + locals.var_mtat);
            let assign38310_e50177: f64 = (assign38310_e50172 - assign38310_e50176);
            let assign38310_e50179: f64 = (assign38310_e50177 * 0.3333333333333333);
            let assign38310_e50180: f64 = (1.0 + assign38310_e50179);
            let assign38310_e50181: f64 = (assign38310_e50169 * assign38310_e50180);
            let assign38310_e50182: f64 = (0.5 * assign38310_e50181);
            let assign38310_e50183: f64 = (1.0 + assign38310_e50182);
            let assign38310_e50184: f64 = (assign38310_e50160 * assign38310_e50183);
            let assign38310_e50185: f64 = (1.0 + assign38310_e50184);
            let assign38310_e50186: f64 = (1e-100 / assign38310_e50185);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38310_e50186, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign38310_e50183) + (assign38310_e50160 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign38310_e50180) + (assign38310_e50169 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign38310_e50185 * assign38310_e50185))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign38310_e50183) + (assign38310_e50160 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign38310_e50180) + (assign38310_e50169 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign38310_e50185 * assign38310_e50185))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign38310_e50183) + (assign38310_e50160 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign38310_e50180) + (assign38310_e50169 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign38310_e50185 * assign38310_e50185))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign38310_e50183) + (assign38310_e50160 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign38310_e50180) + (assign38310_e50169 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign38310_e50185 * assign38310_e50185))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38320_e50200: f64 = (0.29214664 * locals.var_terfc);
            let assign38320_e50204: f64 = (locals.var_terfc * locals.var_terfc);
            let assign38320_e50205: f64 = (locals.var_berfc * assign38320_e50204);
            let assign38320_e50206: f64 = (assign38320_e50200 + assign38320_e50205);
            let assign38320_e50210: f64 = (locals.var_terfc * locals.var_terfc);
            let assign38320_e50212: f64 = (assign38320_e50210 * locals.var_terfc);
            let assign38320_e50213: f64 = (locals.var_cerfc * assign38320_e50212);
            let assign38320_e50214: f64 = (assign38320_e50206 + assign38320_e50213);
            let assign38320_e50216: f64 = (assign38320_e50214 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign38320_e50216, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign38320_e50210 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign38320_e50214 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign38320_e50210 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign38320_e50214 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign38320_e50210 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign38320_e50214 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign38320_e50210 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign38320_e50214 * locals.var_tmp_dn8)), );
        }
        let assign38330_e50221: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard775 = assign38330_e50221;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard775 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign38350_e50238: f64 = (-230.25850929940458);
        let assign38350_e50239: f64 = if locals.var_mtat > assign38350_e50238 { 1.0 } else { 0.0 };
        locals.var_guard776 = assign38350_e50239;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard775 == 0.0)) && (locals.var_guard776 != 0.0)) {
            let assign38360_e50255: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38360_e50255, (assign38360_e50255 * locals.var_mtat_dn5), (assign38360_e50255 * locals.var_mtat_dn6), (assign38360_e50255 * locals.var_mtat_dn7), (assign38360_e50255 * locals.var_mtat_dn8), );
        }
    }
    pub(super) fn stamp_transient_block_46(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard775 == 0.0)) && (locals.var_guard776 == 0.0)) {
            let assign38370_e50276: f64 = (-230.25850929940458);
            let assign38370_e50278: f64 = (assign38370_e50276 - locals.var_mtat);
            let assign38370_e50282: f64 = (-230.25850929940458);
            let assign38370_e50284: f64 = (assign38370_e50282 - locals.var_mtat);
            let assign38370_e50287: f64 = (-230.25850929940458);
            let assign38370_e50289: f64 = (assign38370_e50287 - locals.var_mtat);
            let assign38370_e50291: f64 = (assign38370_e50289 * 0.3333333333333333);
            let assign38370_e50292: f64 = (1.0 + assign38370_e50291);
            let assign38370_e50293: f64 = (assign38370_e50284 * assign38370_e50292);
            let assign38370_e50294: f64 = (0.5 * assign38370_e50293);
            let assign38370_e50295: f64 = (1.0 + assign38370_e50294);
            let assign38370_e50296: f64 = (assign38370_e50278 * assign38370_e50295);
            let assign38370_e50297: f64 = (1.0 + assign38370_e50296);
            let assign38370_e50298: f64 = (1e-100 / assign38370_e50297);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38370_e50298, (-((1e-100 * (((-locals.var_mtat_dn5) * assign38370_e50295) + (assign38370_e50278 * (0.5 * (((-locals.var_mtat_dn5) * assign38370_e50292) + (assign38370_e50284 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign38370_e50297 * assign38370_e50297))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign38370_e50295) + (assign38370_e50278 * (0.5 * (((-locals.var_mtat_dn6) * assign38370_e50292) + (assign38370_e50284 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign38370_e50297 * assign38370_e50297))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign38370_e50295) + (assign38370_e50278 * (0.5 * (((-locals.var_mtat_dn7) * assign38370_e50292) + (assign38370_e50284 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign38370_e50297 * assign38370_e50297))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign38370_e50295) + (assign38370_e50278 * (0.5 * (((-locals.var_mtat_dn8) * assign38370_e50292) + (assign38370_e50284 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign38370_e50297 * assign38370_e50297))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard775 == 0.0)) {
            let assign38380_e50315: f64 = (2.0 * locals.var_tmp);
            let assign38380_e50317: f64 = (assign38380_e50315 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign38380_e50317, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38390_e50331: f64 = (1.772453850905516 * 0.5);
            let assign38390_e50334: f64 = (locals.var_atatbot_d * locals.var_erfctimesexpmtat);
            let assign38390_e50336: f64 = (assign38390_e50334 / locals.var_ktat);
            let assign38390_e50337: f64 = (assign38390_e50331 * assign38390_e50336);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign38390_e50337, (assign38390_e50331 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign38390_e50334 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign38390_e50331 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign38390_e50334 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign38390_e50331 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign38390_e50334 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign38390_e50331 * ((((locals.var_atatbot_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign38390_e50334 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard771 == 0.0)) {
            let assign38400_e50352: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign38400_e50354: f64 = (assign38400_e50352 * locals.var_wtat);
            let assign38400_e50355: f64 = (locals.var_ctatbotd_i * assign38400_e50354);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign38400_e50355, (locals.var_ctatbotd_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign38400_e50352 * locals.var_wtat_dn5))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign38400_e50352 * locals.var_wtat_dn6))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign38400_e50352 * locals.var_wtat_dn7))), (locals.var_ctatbotd_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign38400_e50352 * locals.var_wtat_dn8))), );
        }
        let assign38410_e50360: f64 = if locals.var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard777 = assign38410_e50360;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard777 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign38430_e50374: f64 = if locals.var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard778 = assign38430_e50374;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard777 == 0.0)) && (locals.var_guard778 != 0.0)) {
            let assign38440_e50388: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign38440_e50390: f64 = (assign38440_e50388 * locals.var_vbirbotinv_d);
            let assign38440_e50391: f64 = (assign38440_e50390).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38440_e50391, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard777 == 0.0)) && (locals.var_guard778 == 0.0)) {
            let assign38450_e50408: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign38450_e50410: f64 = (assign38450_e50408 * locals.var_vbirbotinv_d);
            let assign38450_e50412: f64 = (assign38450_e50410).powf(locals.var_pbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38450_e50412, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard777 == 0.0)) {
            let assign38460_e50427: f64 = (locals.var_vbirbotd_i - locals.var_vbbt);
            let assign38460_e50429: f64 = (assign38460_e50427 * locals.var_wdepnulrinvbot_d);
            let assign38460_e50431: f64 = (assign38460_e50429 / locals.var_tmp);
            let assign38460_e50432: f64 = (locals.var_one_over_one_minus_pbot_d * assign38460_e50431);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign38460_e50432, (locals.var_one_over_one_minus_pbot_d * (-((assign38460_e50429 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign38460_e50429 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign38460_e50429 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot_d * (-((assign38460_e50429 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign38470_e50436: f64 = (-locals.var_fbbtbot_d);
        let assign38470_e50438: f64 = (assign38470_e50436 / locals.var_fmaxr);
        let assign38470_e50439: f64 = (assign38470_e50438).abs();
        let assign38470_e50441: f64 = if assign38470_e50439 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard779 = assign38470_e50441;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard777 == 0.0)) && (locals.var_guard779 != 0.0)) {
            let assign38480_e50454: f64 = (-locals.var_fbbtbot_d);
            let assign38480_e50456: f64 = (assign38480_e50454 / locals.var_fmaxr);
            let assign38480_e50457: f64 = (assign38480_e50456).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38480_e50457, (assign38480_e50457 * (-((assign38480_e50454 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign38480_e50457 * (-((assign38480_e50454 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign38480_e50457 * (-((assign38480_e50454 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign38480_e50457 * (-((assign38480_e50454 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign38490_e50461: f64 = (-locals.var_fbbtbot_d);
        let assign38490_e50463: f64 = (assign38490_e50461 / locals.var_fmaxr);
        let assign38490_e50465: f64 = if assign38490_e50463 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard780 = assign38490_e50465;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard777 == 0.0)) && (locals.var_guard779 == 0.0)) && (locals.var_guard780 != 0.0)) {
            let assign38500_e50483: f64 = (-230.25850929940458);
            let assign38500_e50485: f64 = (-locals.var_fbbtbot_d);
            let assign38500_e50487: f64 = (assign38500_e50485 / locals.var_fmaxr);
            let assign38500_e50488: f64 = (assign38500_e50483 - assign38500_e50487);
            let assign38500_e50492: f64 = (-230.25850929940458);
            let assign38500_e50494: f64 = (-locals.var_fbbtbot_d);
            let assign38500_e50496: f64 = (assign38500_e50494 / locals.var_fmaxr);
            let assign38500_e50497: f64 = (assign38500_e50492 - assign38500_e50496);
            let assign38500_e50500: f64 = (-230.25850929940458);
            let assign38500_e50502: f64 = (-locals.var_fbbtbot_d);
            let assign38500_e50504: f64 = (assign38500_e50502 / locals.var_fmaxr);
            let assign38500_e50505: f64 = (assign38500_e50500 - assign38500_e50504);
            let assign38500_e50507: f64 = (assign38500_e50505 * 0.3333333333333333);
            let assign38500_e50508: f64 = (1.0 + assign38500_e50507);
            let assign38500_e50509: f64 = (assign38500_e50497 * assign38500_e50508);
            let assign38500_e50510: f64 = (0.5 * assign38500_e50509);
            let assign38500_e50511: f64 = (1.0 + assign38500_e50510);
            let assign38500_e50512: f64 = (assign38500_e50488 * assign38500_e50511);
            let assign38500_e50513: f64 = (1.0 + assign38500_e50512);
            let assign38500_e50514: f64 = (1e-100 / assign38500_e50513);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38500_e50514, (-((1e-100 * (((-(-((assign38500_e50485 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign38500_e50511) + (assign38500_e50488 * (0.5 * (((-(-((assign38500_e50494 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign38500_e50508) + (assign38500_e50497 * ((-(-((assign38500_e50502 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign38500_e50513 * assign38500_e50513))), (-((1e-100 * (((-(-((assign38500_e50485 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign38500_e50511) + (assign38500_e50488 * (0.5 * (((-(-((assign38500_e50494 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign38500_e50508) + (assign38500_e50497 * ((-(-((assign38500_e50502 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign38500_e50513 * assign38500_e50513))), (-((1e-100 * (((-(-((assign38500_e50485 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign38500_e50511) + (assign38500_e50488 * (0.5 * (((-(-((assign38500_e50494 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign38500_e50508) + (assign38500_e50497 * ((-(-((assign38500_e50502 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign38500_e50513 * assign38500_e50513))), (-((1e-100 * (((-(-((assign38500_e50485 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign38500_e50511) + (assign38500_e50488 * (0.5 * (((-(-((assign38500_e50494 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign38500_e50508) + (assign38500_e50497 * ((-(-((assign38500_e50502 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign38500_e50513 * assign38500_e50513))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard777 == 0.0)) && (locals.var_guard779 == 0.0)) && (locals.var_guard780 == 0.0)) {
            let assign38510_e50535: f64 = (-locals.var_fbbtbot_d);
            let assign38510_e50537: f64 = (assign38510_e50535 / locals.var_fmaxr);
            let assign38510_e50539: f64 = (assign38510_e50537 - 230.25850929940458);
            let assign38510_e50543: f64 = (-locals.var_fbbtbot_d);
            let assign38510_e50545: f64 = (assign38510_e50543 / locals.var_fmaxr);
            let assign38510_e50547: f64 = (assign38510_e50545 - 230.25850929940458);
            let assign38510_e50550: f64 = (-locals.var_fbbtbot_d);
            let assign38510_e50552: f64 = (assign38510_e50550 / locals.var_fmaxr);
            let assign38510_e50554: f64 = (assign38510_e50552 - 230.25850929940458);
            let assign38510_e50556: f64 = (assign38510_e50554 * 0.3333333333333333);
            let assign38510_e50557: f64 = (1.0 + assign38510_e50556);
            let assign38510_e50558: f64 = (assign38510_e50547 * assign38510_e50557);
            let assign38510_e50559: f64 = (0.5 * assign38510_e50558);
            let assign38510_e50560: f64 = (1.0 + assign38510_e50559);
            let assign38510_e50561: f64 = (assign38510_e50539 * assign38510_e50560);
            let assign38510_e50562: f64 = (1.0 + assign38510_e50561);
            let assign38510_e50563: f64 = (1e100 * assign38510_e50562);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38510_e50563, (1e100 * (((-((assign38510_e50535 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign38510_e50560) + (assign38510_e50539 * (0.5 * (((-((assign38510_e50543 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign38510_e50557) + (assign38510_e50547 * ((-((assign38510_e50550 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign38510_e50535 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign38510_e50560) + (assign38510_e50539 * (0.5 * (((-((assign38510_e50543 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign38510_e50557) + (assign38510_e50547 * ((-((assign38510_e50550 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign38510_e50535 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign38510_e50560) + (assign38510_e50539 * (0.5 * (((-((assign38510_e50543 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign38510_e50557) + (assign38510_e50547 * ((-((assign38510_e50550 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign38510_e50535 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign38510_e50560) + (assign38510_e50539 * (0.5 * (((-((assign38510_e50543 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign38510_e50557) + (assign38510_e50547 * ((-((assign38510_e50550 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard777 == 0.0)) {
            let assign38520_e50578: f64 = (locals.var_v5 * locals.var_fmaxr);
            let assign38520_e50580: f64 = (assign38520_e50578 * locals.var_fmaxr);
            let assign38520_e50582: f64 = (assign38520_e50580 * locals.var_tmp);
            let assign38520_e50583: f64 = (locals.var_cbbtbotd_i * assign38520_e50582);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign38520_e50583, (locals.var_cbbtbotd_i * (((((locals.var_v5 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign38520_e50578 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign38520_e50580 * locals.var_tmp_dn5))), (locals.var_cbbtbotd_i * (((((locals.var_v5 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign38520_e50578 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign38520_e50580 * locals.var_tmp_dn6))), (locals.var_cbbtbotd_i * (((((locals.var_v5 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign38520_e50578 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign38520_e50580 * locals.var_tmp_dn7))), (locals.var_cbbtbotd_i * (((((locals.var_v5 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign38520_e50578 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign38520_e50580 * locals.var_tmp_dn8))), );
        }
        let assign38530_e50588: f64 = if locals.var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard781 = assign38530_e50588;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard781 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign38550_e50602: f64 = (-locals.var_alphaav);
        let assign38550_e50604: f64 = (assign38550_e50602 * locals.var_vbrbotd_i);
        let assign38550_e50605: f64 = if locals.var_vav > assign38550_e50604 { 1.0 } else { 0.0 };
        locals.var_guard782 = assign38550_e50605;
        let assign38560_e50608: f64 = if locals.var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard783 = assign38560_e50608;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard781 == 0.0)) && (locals.var_guard782 != 0.0)) && (locals.var_guard783 != 0.0)) {
            let assign38570_e50624: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign38570_e50627: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign38570_e50628: f64 = (assign38570_e50624 * assign38570_e50627);
            let assign38570_e50631: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign38570_e50632: f64 = (assign38570_e50628 * assign38570_e50631);
            let assign38570_e50635: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign38570_e50636: f64 = (assign38570_e50632 * assign38570_e50635);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38570_e50636, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard781 == 0.0)) && (locals.var_guard782 != 0.0)) && (locals.var_guard783 == 0.0)) {
            let assign38580_e50655: f64 = (locals.var_vav * locals.var_vbrinvbot_d);
            let assign38580_e50656: f64 = (assign38580_e50655).abs();
            let assign38580_e50658: f64 = (assign38580_e50656).powf(locals.var_pbrbotd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38580_e50658, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard781 == 0.0)) && (locals.var_guard782 != 0.0)) {
            let assign38590_e50675: f64 = (1.0 - locals.var_tmp);
            let assign38590_e50676: f64 = (1.0 / assign38590_e50675);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign38590_e50676, (-((-locals.var_tmp_dn5) / (assign38590_e50675 * assign38590_e50675))), (-((-locals.var_tmp_dn6) / (assign38590_e50675 * assign38590_e50675))), (-((-locals.var_tmp_dn7) / (assign38590_e50675 * assign38590_e50675))), (-((-locals.var_tmp_dn8) / (assign38590_e50675 * assign38590_e50675))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard781 == 0.0)) && (locals.var_guard782 == 0.0)) {
            let assign38600_e50695: f64 = (locals.var_alphaav * locals.var_vbrbotd_i);
            let assign38600_e50696: f64 = (locals.var_vav + assign38600_e50695);
            let assign38600_e50698: f64 = (assign38600_e50696 * locals.var_slopebot_d);
            let assign38600_e50699: f64 = (locals.var_fstopbot_d + assign38600_e50698);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign38600_e50699, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard767 == 0.0)) {
            let assign38610_e50711: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign38610_e50713: f64 = (assign38610_e50711 + locals.var_itat);
            let assign38610_e50715: f64 = (assign38610_e50713 + locals.var_ibbt);
            let assign38610_e50716: f64 = (p.p29 * assign38610_e50715);
            let assign38610_e50718: f64 = (assign38610_e50716 * locals.var_fbreakdown);
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (assign38610_e50718, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign38610_e50716 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign38610_e50716 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign38610_e50716 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign38610_e50716 * locals.var_fbreakdown_dn8)), );
        }
        let assign38620_e50723: f64 = if locals.var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard784 = assign38620_e50723;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 != 0.0)) {
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) {
            let assign38640_e50740: f64 = (locals.var_idsatsti_d * locals.var_idmult);
            locals.var_id__blk219 = assign38640_e50740;
        }
        let assign38650_e50749: f64 = if ((locals.var_csrhstid_i == 0.0) && (locals.var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard785 = assign38650_e50749;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) {
            let assign38670_e50772: f64 = (locals.var_vbisti_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign38670_e50772;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) {
            let assign38680_e50788: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign38680_e50789: f64 = (1.0 - assign38680_e50788);
            let assign38680_e50790: f64 = (assign38680_e50789).sqrt();
            let assign38680_e50791: f64 = (1.0 - assign38680_e50790);
            locals.var_wsrhstep = assign38680_e50791;
        }
        let assign38690_e50796: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard786 = assign38690_e50796;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) && (locals.var_guard786 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) && (locals.var_guard786 == 0.0)) {
            let assign38710_e50825: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign38710_e50827: f64 = (locals.var_wsrhstep).ln();
            let assign38710_e50828: f64 = (assign38710_e50825 * assign38710_e50827);
            let assign38710_e50831: f64 = (1.0 - locals.var_wsrhstep);
            let assign38710_e50832: f64 = (assign38710_e50828 / assign38710_e50831);
            let assign38710_e50834: f64 = (assign38710_e50832 + locals.var_wsrhstep);
            let assign38710_e50838: f64 = (2.0 * locals.var_pstid_i);
            let assign38710_e50839: f64 = (1.0 - assign38710_e50838);
            let assign38710_e50840: f64 = (assign38710_e50834 * assign38710_e50839);
            locals.var_dwsrh = assign38710_e50840;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) {
            let assign38720_e50854: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign38720_e50854;
        }
        let assign38730_e50859: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard787 = assign38730_e50859;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) && (locals.var_guard787 != 0.0)) {
            let assign38740_e50873: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv_d);
            let assign38740_e50874: f64 = (assign38740_e50873).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38740_e50874, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) && (locals.var_guard787 == 0.0)) {
            let assign38750_e50891: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv_d);
            let assign38750_e50893: f64 = (assign38750_e50891).powf(locals.var_pstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign38750_e50893, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) {
            let assign38760_e50907: f64 = (locals.var_wdepnulrsti_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign38760_e50907, (locals.var_wdepnulrsti_d * locals.var_tmp_dn5), (locals.var_wdepnulrsti_d * locals.var_tmp_dn6), (locals.var_wdepnulrsti_d * locals.var_tmp_dn7), (locals.var_wdepnulrsti_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) {
            let assign38770_e50922: f64 = (locals.var_zinv - 1.0);
            let assign38770_e50924: f64 = (assign38770_e50922 * locals.var_wdep);
            let assign38770_e50925: f64 = (locals.var_ftdsti_d * assign38770_e50924);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign38770_e50925, (locals.var_ftdsti_d * (assign38770_e50922 * locals.var_wdep_dn5)), (locals.var_ftdsti_d * (assign38770_e50922 * locals.var_wdep_dn6)), (locals.var_ftdsti_d * (assign38770_e50922 * locals.var_wdep_dn7)), (locals.var_ftdsti_d * (assign38770_e50922 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) {
            let assign38780_e50940: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign38780_e50941: f64 = (locals.var_csrhstid_i * assign38780_e50940);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign38780_e50941, (locals.var_csrhstid_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhstid_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign38790_e50946: f64 = if locals.var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard788 = assign38790_e50946;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign38810_e50970: f64 = (locals.var_wdep * locals.var_one_minus_psti_d);
            let assign38810_e50972: f64 = (assign38810_e50970 / locals.var_vbi_minus_vjsrh);
            let assign38810_e50973: f64 = (locals.var_btatpartsti_d * assign38810_e50972);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign38810_e50973, (locals.var_btatpartsti_d * ((locals.var_wdep_dn5 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn6 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn7 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti_d * ((locals.var_wdep_dn8 * locals.var_one_minus_psti_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign38820_e50987: f64 = (0.666666666666667 * locals.var_atatsti_d);
            let assign38820_e50989: f64 = (assign38820_e50987 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign38820_e50989, (-((assign38820_e50987 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign38820_e50987 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign38820_e50987 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign38820_e50987 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign38830_e51003: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign38830_e51003, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign38840_e51017: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign38840_e51020: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign38840_e51022: f64 = (assign38840_e51020 + 1.0);
            let assign38840_e51023: f64 = (assign38840_e51017 / assign38840_e51022);
            let assign38840_e51024: f64 = (assign38840_e51023).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign38840_e51024, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign38840_e51022) - (assign38840_e51017 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign38840_e51022 * assign38840_e51022)) / (2.0 * assign38840_e51024)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign38840_e51022) - (assign38840_e51017 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign38840_e51022 * assign38840_e51022)) / (2.0 * assign38840_e51024)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign38840_e51022) - (assign38840_e51017 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign38840_e51022 * assign38840_e51022)) / (2.0 * assign38840_e51024)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign38840_e51022) - (assign38840_e51017 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign38840_e51022 * assign38840_e51022)) / (2.0 * assign38840_e51024)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign38850_e51037: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign38850_e51037, (locals.var_umax_dn5 / (2.0 * assign38850_e51037)), (locals.var_umax_dn6 / (2.0 * assign38850_e51037)), (locals.var_umax_dn7 / (2.0 * assign38850_e51037)), (locals.var_umax_dn8 / (2.0 * assign38850_e51037)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign38860_e51051: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign38860_e51051, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign38870_e51055: f64 = (-locals.var_pstid_i);
        let assign38870_e51057: f64 = (assign38870_e51055 * locals.var_one_over_one_minus_psti_d);
        let assign38870_e51059: f64 = (-1.0);
        let assign38870_e51060: f64 = if assign38870_e51057 == assign38870_e51059 { 1.0 } else { 0.0 };
        locals.var_guard789 = assign38870_e51060;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard789 != 0.0)) {
            let assign38880_e51076: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign38880_e51077: f64 = (1.0 + assign38880_e51076);
            let assign38880_e51078: f64 = (1.0 / assign38880_e51077);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign38880_e51078, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign38880_e51077 * assign38880_e51077))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign38880_e51077 * assign38880_e51077))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign38880_e51077 * assign38880_e51077))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign38880_e51077 * assign38880_e51077))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard789 == 0.0)) {
            let assign38890_e51096: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign38890_e51097: f64 = (1.0 + assign38890_e51096);
            let assign38890_e51099: f64 = (-locals.var_pstid_i);
            let assign38890_e51101: f64 = (assign38890_e51099 * locals.var_one_over_one_minus_psti_d);
            let assign38890_e51102: f64 = (assign38890_e51097).powf(assign38890_e51101);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign38890_e51102, if 0.0 == 0.0 && ((assign38890_e51101) as f64).is_finite() && ((assign38890_e51101) as f64).fract() == 0.0 { if assign38890_e51101 == 0.0 { 0.0 } else { (assign38890_e51101 * ((assign38890_e51097).powf(assign38890_e51101 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign38890_e51102 * (assign38890_e51101 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign38890_e51097))) }, if 0.0 == 0.0 && ((assign38890_e51101) as f64).is_finite() && ((assign38890_e51101) as f64).fract() == 0.0 { if assign38890_e51101 == 0.0 { 0.0 } else { (assign38890_e51101 * ((assign38890_e51097).powf(assign38890_e51101 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign38890_e51102 * (assign38890_e51101 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign38890_e51097))) }, if 0.0 == 0.0 && ((assign38890_e51101) as f64).is_finite() && ((assign38890_e51101) as f64).fract() == 0.0 { if assign38890_e51101 == 0.0 { 0.0 } else { (assign38890_e51101 * ((assign38890_e51097).powf(assign38890_e51101 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign38890_e51102 * (assign38890_e51101 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign38890_e51097))) }, if 0.0 == 0.0 && ((assign38890_e51101) as f64).is_finite() && ((assign38890_e51101) as f64).fract() == 0.0 { if assign38890_e51101 == 0.0 { 0.0 } else { (assign38890_e51101 * ((assign38890_e51097).powf(assign38890_e51101 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign38890_e51102 * (assign38890_e51101 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign38890_e51097))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign38900_e51116: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign38900_e51119: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign38900_e51120: f64 = (assign38900_e51116 / assign38900_e51119);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign38900_e51120, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign38900_e51119) - (assign38900_e51116 * locals.var_wgamma_dn5)) / (assign38900_e51119 * assign38900_e51119)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign38900_e51119) - (assign38900_e51116 * locals.var_wgamma_dn6)) / (assign38900_e51119 * assign38900_e51119)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign38900_e51119) - (assign38900_e51116 * locals.var_wgamma_dn7)) / (assign38900_e51119 * assign38900_e51119)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign38900_e51119) - (assign38900_e51116 * locals.var_wgamma_dn8)) / (assign38900_e51119 * assign38900_e51119)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign38910_e51135: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign38910_e51136: f64 = (0.375 * assign38910_e51135);
            let assign38910_e51137: f64 = (assign38910_e51136).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign38910_e51137, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign38910_e51137)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign38910_e51137)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign38910_e51137)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign38910_e51137)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign38920_e51152: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign38920_e51153: f64 = (2.0 * assign38920_e51152);
            let assign38920_e51155: f64 = (assign38920_e51153 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign38920_e51155, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign38930_e51169: f64 = (locals.var_atatsti_d * locals.var_twoatatoverthreebtat);
            let assign38930_e51171: f64 = (assign38930_e51169 * locals.var_sqrtumax);
            let assign38930_e51174: f64 = (locals.var_atatsti_d * locals.var_umax);
            let assign38930_e51175: f64 = (assign38930_e51171 - assign38930_e51174);
            let assign38930_e51179: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign38930_e51180: f64 = (0.5 * assign38930_e51179);
            let assign38930_e51181: f64 = (assign38930_e51175 + assign38930_e51180);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign38930_e51181, (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign38930_e51169 * locals.var_sqrtumax_dn5)) - (locals.var_atatsti_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign38930_e51169 * locals.var_sqrtumax_dn6)) - (locals.var_atatsti_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign38930_e51169 * locals.var_sqrtumax_dn7)) - (locals.var_atatsti_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatsti_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign38930_e51169 * locals.var_sqrtumax_dn8)) - (locals.var_atatsti_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign38940_e51195: f64 = (locals.var_ltat - 1.0);
            let assign38940_e51197: f64 = (assign38940_e51195 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign38940_e51197, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign38940_e51195 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign38940_e51195 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign38940_e51195 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign38940_e51195 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign38950_e51211: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign38950_e51211, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign38960_e51216: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard790 = assign38960_e51216;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard790 != 0.0)) {
            let assign38970_e51232: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign38970_e51233: f64 = (1.0 + assign38970_e51232);
            let assign38970_e51234: f64 = (1.0 / assign38970_e51233);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign38970_e51234, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign38970_e51233 * assign38970_e51233))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign38970_e51233 * assign38970_e51233))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign38970_e51233 * assign38970_e51233))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign38970_e51233 * assign38970_e51233))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard790 == 0.0)) {
            let assign38980_e51253: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign38980_e51254: f64 = (1.0 - assign38980_e51253);
            let assign38980_e51255: f64 = (1.0 / assign38980_e51254);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign38980_e51255, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign38980_e51254 * assign38980_e51254))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign38980_e51254 * assign38980_e51254))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign38980_e51254 * assign38980_e51254))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign38980_e51254 * assign38980_e51254))), );
        }
        let assign38990_e51259: f64 = (-locals.var_ysq);
        let assign38990_e51261: f64 = (assign38990_e51259 + locals.var_mtat);
        let assign38990_e51263: f64 = (-230.25850929940458);
        let assign38990_e51264: f64 = if assign38990_e51261 > assign38990_e51263 { 1.0 } else { 0.0 };
        locals.var_guard791 = assign38990_e51264;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard791 != 0.0)) {
            let assign39000_e51277: f64 = (-locals.var_ysq);
            let assign39000_e51279: f64 = (assign39000_e51277 + locals.var_mtat);
            let assign39000_e51280: f64 = (assign39000_e51279).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39000_e51280, (assign39000_e51280 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign39000_e51280 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign39000_e51280 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign39000_e51280 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard791 == 0.0)) {
            let assign39010_e51298: f64 = (-230.25850929940458);
            let assign39010_e51300: f64 = (-locals.var_ysq);
            let assign39010_e51302: f64 = (assign39010_e51300 + locals.var_mtat);
            let assign39010_e51303: f64 = (assign39010_e51298 - assign39010_e51302);
            let assign39010_e51307: f64 = (-230.25850929940458);
            let assign39010_e51309: f64 = (-locals.var_ysq);
            let assign39010_e51311: f64 = (assign39010_e51309 + locals.var_mtat);
            let assign39010_e51312: f64 = (assign39010_e51307 - assign39010_e51311);
            let assign39010_e51315: f64 = (-230.25850929940458);
            let assign39010_e51317: f64 = (-locals.var_ysq);
            let assign39010_e51319: f64 = (assign39010_e51317 + locals.var_mtat);
            let assign39010_e51320: f64 = (assign39010_e51315 - assign39010_e51319);
            let assign39010_e51322: f64 = (assign39010_e51320 * 0.3333333333333333);
            let assign39010_e51323: f64 = (1.0 + assign39010_e51322);
            let assign39010_e51324: f64 = (assign39010_e51312 * assign39010_e51323);
            let assign39010_e51325: f64 = (0.5 * assign39010_e51324);
            let assign39010_e51326: f64 = (1.0 + assign39010_e51325);
            let assign39010_e51327: f64 = (assign39010_e51303 * assign39010_e51326);
            let assign39010_e51328: f64 = (1.0 + assign39010_e51327);
            let assign39010_e51329: f64 = (1e-100 / assign39010_e51328);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39010_e51329, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign39010_e51326) + (assign39010_e51303 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign39010_e51323) + (assign39010_e51312 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign39010_e51328 * assign39010_e51328))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign39010_e51326) + (assign39010_e51303 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign39010_e51323) + (assign39010_e51312 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign39010_e51328 * assign39010_e51328))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign39010_e51326) + (assign39010_e51303 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign39010_e51323) + (assign39010_e51312 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign39010_e51328 * assign39010_e51328))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign39010_e51326) + (assign39010_e51303 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign39010_e51323) + (assign39010_e51312 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign39010_e51328 * assign39010_e51328))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign39020_e51343: f64 = (0.29214664 * locals.var_terfc);
            let assign39020_e51347: f64 = (locals.var_terfc * locals.var_terfc);
            let assign39020_e51348: f64 = (locals.var_berfc * assign39020_e51347);
            let assign39020_e51349: f64 = (assign39020_e51343 + assign39020_e51348);
            let assign39020_e51353: f64 = (locals.var_terfc * locals.var_terfc);
            let assign39020_e51355: f64 = (assign39020_e51353 * locals.var_terfc);
            let assign39020_e51356: f64 = (locals.var_cerfc * assign39020_e51355);
            let assign39020_e51357: f64 = (assign39020_e51349 + assign39020_e51356);
            let assign39020_e51359: f64 = (assign39020_e51357 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign39020_e51359, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign39020_e51353 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign39020_e51357 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign39020_e51353 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign39020_e51357 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign39020_e51353 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign39020_e51357 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign39020_e51353 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign39020_e51357 * locals.var_tmp_dn8)), );
        }
        let assign39030_e51364: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard792 = assign39030_e51364;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard792 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign39050_e51381: f64 = (-230.25850929940458);
        let assign39050_e51382: f64 = if locals.var_mtat > assign39050_e51381 { 1.0 } else { 0.0 };
        locals.var_guard793 = assign39050_e51382;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard792 == 0.0)) && (locals.var_guard793 != 0.0)) {
            let assign39060_e51398: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39060_e51398, (assign39060_e51398 * locals.var_mtat_dn5), (assign39060_e51398 * locals.var_mtat_dn6), (assign39060_e51398 * locals.var_mtat_dn7), (assign39060_e51398 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard792 == 0.0)) && (locals.var_guard793 == 0.0)) {
            let assign39070_e51419: f64 = (-230.25850929940458);
            let assign39070_e51421: f64 = (assign39070_e51419 - locals.var_mtat);
            let assign39070_e51425: f64 = (-230.25850929940458);
            let assign39070_e51427: f64 = (assign39070_e51425 - locals.var_mtat);
            let assign39070_e51430: f64 = (-230.25850929940458);
            let assign39070_e51432: f64 = (assign39070_e51430 - locals.var_mtat);
            let assign39070_e51434: f64 = (assign39070_e51432 * 0.3333333333333333);
            let assign39070_e51435: f64 = (1.0 + assign39070_e51434);
            let assign39070_e51436: f64 = (assign39070_e51427 * assign39070_e51435);
            let assign39070_e51437: f64 = (0.5 * assign39070_e51436);
            let assign39070_e51438: f64 = (1.0 + assign39070_e51437);
            let assign39070_e51439: f64 = (assign39070_e51421 * assign39070_e51438);
            let assign39070_e51440: f64 = (1.0 + assign39070_e51439);
            let assign39070_e51441: f64 = (1e-100 / assign39070_e51440);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39070_e51441, (-((1e-100 * (((-locals.var_mtat_dn5) * assign39070_e51438) + (assign39070_e51421 * (0.5 * (((-locals.var_mtat_dn5) * assign39070_e51435) + (assign39070_e51427 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign39070_e51440 * assign39070_e51440))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign39070_e51438) + (assign39070_e51421 * (0.5 * (((-locals.var_mtat_dn6) * assign39070_e51435) + (assign39070_e51427 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign39070_e51440 * assign39070_e51440))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign39070_e51438) + (assign39070_e51421 * (0.5 * (((-locals.var_mtat_dn7) * assign39070_e51435) + (assign39070_e51427 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign39070_e51440 * assign39070_e51440))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign39070_e51438) + (assign39070_e51421 * (0.5 * (((-locals.var_mtat_dn8) * assign39070_e51435) + (assign39070_e51427 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign39070_e51440 * assign39070_e51440))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard792 == 0.0)) {
            let assign39080_e51458: f64 = (2.0 * locals.var_tmp);
            let assign39080_e51460: f64 = (assign39080_e51458 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign39080_e51460, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign39090_e51474: f64 = (1.772453850905516 * 0.5);
            let assign39090_e51477: f64 = (locals.var_atatsti_d * locals.var_erfctimesexpmtat);
            let assign39090_e51479: f64 = (assign39090_e51477 / locals.var_ktat);
            let assign39090_e51480: f64 = (assign39090_e51474 * assign39090_e51479);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign39090_e51480, (assign39090_e51474 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign39090_e51477 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign39090_e51474 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign39090_e51477 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign39090_e51474 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign39090_e51477 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign39090_e51474 * ((((locals.var_atatsti_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign39090_e51477 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
    }
    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard788 == 0.0)) {
            let assign39100_e51495: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign39100_e51497: f64 = (assign39100_e51495 * locals.var_wtat);
            let assign39100_e51498: f64 = (locals.var_ctatstid_i * assign39100_e51497);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign39100_e51498, (locals.var_ctatstid_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign39100_e51495 * locals.var_wtat_dn5))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign39100_e51495 * locals.var_wtat_dn6))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign39100_e51495 * locals.var_wtat_dn7))), (locals.var_ctatstid_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign39100_e51495 * locals.var_wtat_dn8))), );
        }
        let assign39110_e51503: f64 = if locals.var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard794 = assign39110_e51503;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard794 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign39130_e51517: f64 = if locals.var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard795 = assign39130_e51517;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard794 == 0.0)) && (locals.var_guard795 != 0.0)) {
            let assign39140_e51531: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign39140_e51533: f64 = (assign39140_e51531 * locals.var_vbirstiinv_d);
            let assign39140_e51534: f64 = (assign39140_e51533).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39140_e51534, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard794 == 0.0)) && (locals.var_guard795 == 0.0)) {
            let assign39150_e51551: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign39150_e51553: f64 = (assign39150_e51551 * locals.var_vbirstiinv_d);
            let assign39150_e51555: f64 = (assign39150_e51553).powf(locals.var_pstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39150_e51555, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard794 == 0.0)) {
            let assign39160_e51570: f64 = (locals.var_vbirstid_i - locals.var_vbbt);
            let assign39160_e51572: f64 = (assign39160_e51570 * locals.var_wdepnulrinvsti_d);
            let assign39160_e51574: f64 = (assign39160_e51572 / locals.var_tmp);
            let assign39160_e51575: f64 = (locals.var_one_over_one_minus_psti_d * assign39160_e51574);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign39160_e51575, (locals.var_one_over_one_minus_psti_d * (-((assign39160_e51572 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign39160_e51572 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign39160_e51572 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti_d * (-((assign39160_e51572 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign39170_e51579: f64 = (-locals.var_fbbtsti_d);
        let assign39170_e51581: f64 = (assign39170_e51579 / locals.var_fmaxr);
        let assign39170_e51582: f64 = (assign39170_e51581).abs();
        let assign39170_e51584: f64 = if assign39170_e51582 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard796 = assign39170_e51584;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard794 == 0.0)) && (locals.var_guard796 != 0.0)) {
            let assign39180_e51597: f64 = (-locals.var_fbbtsti_d);
            let assign39180_e51599: f64 = (assign39180_e51597 / locals.var_fmaxr);
            let assign39180_e51600: f64 = (assign39180_e51599).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39180_e51600, (assign39180_e51600 * (-((assign39180_e51597 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign39180_e51600 * (-((assign39180_e51597 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign39180_e51600 * (-((assign39180_e51597 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign39180_e51600 * (-((assign39180_e51597 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign39190_e51604: f64 = (-locals.var_fbbtsti_d);
        let assign39190_e51606: f64 = (assign39190_e51604 / locals.var_fmaxr);
        let assign39190_e51608: f64 = if assign39190_e51606 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard797 = assign39190_e51608;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard794 == 0.0)) && (locals.var_guard796 == 0.0)) && (locals.var_guard797 != 0.0)) {
            let assign39200_e51626: f64 = (-230.25850929940458);
            let assign39200_e51628: f64 = (-locals.var_fbbtsti_d);
            let assign39200_e51630: f64 = (assign39200_e51628 / locals.var_fmaxr);
            let assign39200_e51631: f64 = (assign39200_e51626 - assign39200_e51630);
            let assign39200_e51635: f64 = (-230.25850929940458);
            let assign39200_e51637: f64 = (-locals.var_fbbtsti_d);
            let assign39200_e51639: f64 = (assign39200_e51637 / locals.var_fmaxr);
            let assign39200_e51640: f64 = (assign39200_e51635 - assign39200_e51639);
            let assign39200_e51643: f64 = (-230.25850929940458);
            let assign39200_e51645: f64 = (-locals.var_fbbtsti_d);
            let assign39200_e51647: f64 = (assign39200_e51645 / locals.var_fmaxr);
            let assign39200_e51648: f64 = (assign39200_e51643 - assign39200_e51647);
            let assign39200_e51650: f64 = (assign39200_e51648 * 0.3333333333333333);
            let assign39200_e51651: f64 = (1.0 + assign39200_e51650);
            let assign39200_e51652: f64 = (assign39200_e51640 * assign39200_e51651);
            let assign39200_e51653: f64 = (0.5 * assign39200_e51652);
            let assign39200_e51654: f64 = (1.0 + assign39200_e51653);
            let assign39200_e51655: f64 = (assign39200_e51631 * assign39200_e51654);
            let assign39200_e51656: f64 = (1.0 + assign39200_e51655);
            let assign39200_e51657: f64 = (1e-100 / assign39200_e51656);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39200_e51657, (-((1e-100 * (((-(-((assign39200_e51628 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign39200_e51654) + (assign39200_e51631 * (0.5 * (((-(-((assign39200_e51637 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign39200_e51651) + (assign39200_e51640 * ((-(-((assign39200_e51645 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign39200_e51656 * assign39200_e51656))), (-((1e-100 * (((-(-((assign39200_e51628 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign39200_e51654) + (assign39200_e51631 * (0.5 * (((-(-((assign39200_e51637 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign39200_e51651) + (assign39200_e51640 * ((-(-((assign39200_e51645 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign39200_e51656 * assign39200_e51656))), (-((1e-100 * (((-(-((assign39200_e51628 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign39200_e51654) + (assign39200_e51631 * (0.5 * (((-(-((assign39200_e51637 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign39200_e51651) + (assign39200_e51640 * ((-(-((assign39200_e51645 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign39200_e51656 * assign39200_e51656))), (-((1e-100 * (((-(-((assign39200_e51628 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign39200_e51654) + (assign39200_e51631 * (0.5 * (((-(-((assign39200_e51637 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign39200_e51651) + (assign39200_e51640 * ((-(-((assign39200_e51645 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign39200_e51656 * assign39200_e51656))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard794 == 0.0)) && (locals.var_guard796 == 0.0)) && (locals.var_guard797 == 0.0)) {
            let assign39210_e51678: f64 = (-locals.var_fbbtsti_d);
            let assign39210_e51680: f64 = (assign39210_e51678 / locals.var_fmaxr);
            let assign39210_e51682: f64 = (assign39210_e51680 - 230.25850929940458);
            let assign39210_e51686: f64 = (-locals.var_fbbtsti_d);
            let assign39210_e51688: f64 = (assign39210_e51686 / locals.var_fmaxr);
            let assign39210_e51690: f64 = (assign39210_e51688 - 230.25850929940458);
            let assign39210_e51693: f64 = (-locals.var_fbbtsti_d);
            let assign39210_e51695: f64 = (assign39210_e51693 / locals.var_fmaxr);
            let assign39210_e51697: f64 = (assign39210_e51695 - 230.25850929940458);
            let assign39210_e51699: f64 = (assign39210_e51697 * 0.3333333333333333);
            let assign39210_e51700: f64 = (1.0 + assign39210_e51699);
            let assign39210_e51701: f64 = (assign39210_e51690 * assign39210_e51700);
            let assign39210_e51702: f64 = (0.5 * assign39210_e51701);
            let assign39210_e51703: f64 = (1.0 + assign39210_e51702);
            let assign39210_e51704: f64 = (assign39210_e51682 * assign39210_e51703);
            let assign39210_e51705: f64 = (1.0 + assign39210_e51704);
            let assign39210_e51706: f64 = (1e100 * assign39210_e51705);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39210_e51706, (1e100 * (((-((assign39210_e51678 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign39210_e51703) + (assign39210_e51682 * (0.5 * (((-((assign39210_e51686 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign39210_e51700) + (assign39210_e51690 * ((-((assign39210_e51693 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign39210_e51678 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign39210_e51703) + (assign39210_e51682 * (0.5 * (((-((assign39210_e51686 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign39210_e51700) + (assign39210_e51690 * ((-((assign39210_e51693 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign39210_e51678 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign39210_e51703) + (assign39210_e51682 * (0.5 * (((-((assign39210_e51686 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign39210_e51700) + (assign39210_e51690 * ((-((assign39210_e51693 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign39210_e51678 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign39210_e51703) + (assign39210_e51682 * (0.5 * (((-((assign39210_e51686 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign39210_e51700) + (assign39210_e51690 * ((-((assign39210_e51693 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard794 == 0.0)) {
            let assign39220_e51721: f64 = (locals.var_v5 * locals.var_fmaxr);
            let assign39220_e51723: f64 = (assign39220_e51721 * locals.var_fmaxr);
            let assign39220_e51725: f64 = (assign39220_e51723 * locals.var_tmp);
            let assign39220_e51726: f64 = (locals.var_cbbtstid_i * assign39220_e51725);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign39220_e51726, (locals.var_cbbtstid_i * (((((locals.var_v5 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign39220_e51721 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign39220_e51723 * locals.var_tmp_dn5))), (locals.var_cbbtstid_i * (((((locals.var_v5 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign39220_e51721 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign39220_e51723 * locals.var_tmp_dn6))), (locals.var_cbbtstid_i * (((((locals.var_v5 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign39220_e51721 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign39220_e51723 * locals.var_tmp_dn7))), (locals.var_cbbtstid_i * (((((locals.var_v5 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign39220_e51721 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign39220_e51723 * locals.var_tmp_dn8))), );
        }
        let assign39230_e51731: f64 = if locals.var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard798 = assign39230_e51731;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard798 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign39250_e51745: f64 = (-locals.var_alphaav);
        let assign39250_e51747: f64 = (assign39250_e51745 * locals.var_vbrstid_i);
        let assign39250_e51748: f64 = if locals.var_vav > assign39250_e51747 { 1.0 } else { 0.0 };
        locals.var_guard799 = assign39250_e51748;
        let assign39260_e51751: f64 = if locals.var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard800 = assign39260_e51751;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard798 == 0.0)) && (locals.var_guard799 != 0.0)) && (locals.var_guard800 != 0.0)) {
            let assign39270_e51767: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign39270_e51770: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign39270_e51771: f64 = (assign39270_e51767 * assign39270_e51770);
            let assign39270_e51774: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign39270_e51775: f64 = (assign39270_e51771 * assign39270_e51774);
            let assign39270_e51778: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign39270_e51779: f64 = (assign39270_e51775 * assign39270_e51778);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39270_e51779, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard798 == 0.0)) && (locals.var_guard799 != 0.0)) && (locals.var_guard800 == 0.0)) {
            let assign39280_e51798: f64 = (locals.var_vav * locals.var_vbrinvsti_d);
            let assign39280_e51799: f64 = (assign39280_e51798).abs();
            let assign39280_e51801: f64 = (assign39280_e51799).powf(locals.var_pbrstid_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39280_e51801, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard798 == 0.0)) && (locals.var_guard799 != 0.0)) {
            let assign39290_e51818: f64 = (1.0 - locals.var_tmp);
            let assign39290_e51819: f64 = (1.0 / assign39290_e51818);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign39290_e51819, (-((-locals.var_tmp_dn5) / (assign39290_e51818 * assign39290_e51818))), (-((-locals.var_tmp_dn6) / (assign39290_e51818 * assign39290_e51818))), (-((-locals.var_tmp_dn7) / (assign39290_e51818 * assign39290_e51818))), (-((-locals.var_tmp_dn8) / (assign39290_e51818 * assign39290_e51818))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard798 == 0.0)) && (locals.var_guard799 == 0.0)) {
            let assign39300_e51838: f64 = (locals.var_alphaav * locals.var_vbrstid_i);
            let assign39300_e51839: f64 = (locals.var_vav + assign39300_e51838);
            let assign39300_e51841: f64 = (assign39300_e51839 * locals.var_slopesti_d);
            let assign39300_e51842: f64 = (locals.var_fstopsti_d + assign39300_e51841);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign39300_e51842, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard784 == 0.0)) {
            let assign39310_e51854: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign39310_e51856: f64 = (assign39310_e51854 + locals.var_itat);
            let assign39310_e51858: f64 = (assign39310_e51856 + locals.var_ibbt);
            let assign39310_e51859: f64 = (p.p29 * assign39310_e51858);
            let assign39310_e51861: f64 = (assign39310_e51859 * locals.var_fbreakdown);
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (assign39310_e51861, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign39310_e51859 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign39310_e51859 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign39310_e51859 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign39310_e51859 * locals.var_fbreakdown_dn8)), );
        }
        let assign39320_e51866: f64 = if locals.var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard801 = assign39320_e51866;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 != 0.0)) {
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) {
            let assign39340_e51883: f64 = (locals.var_idsatgat_d * locals.var_idmult);
            locals.var_id__blk219 = assign39340_e51883;
        }
        let assign39350_e51892: f64 = if ((locals.var_csrhgatd_i == 0.0) && (locals.var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard802 = assign39350_e51892;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 == 0.0)) {
            let assign39370_e51915: f64 = (locals.var_vbigat_d - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign39370_e51915;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 == 0.0)) {
            let assign39380_e51931: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign39380_e51932: f64 = (1.0 - assign39380_e51931);
            let assign39380_e51933: f64 = (assign39380_e51932).sqrt();
            let assign39380_e51934: f64 = (1.0 - assign39380_e51933);
            locals.var_wsrhstep = assign39380_e51934;
        }
        let assign39390_e51939: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard803 = assign39390_e51939;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 == 0.0)) && (locals.var_guard803 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 == 0.0)) && (locals.var_guard803 == 0.0)) {
            let assign39410_e51968: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign39410_e51970: f64 = (locals.var_wsrhstep).ln();
            let assign39410_e51971: f64 = (assign39410_e51968 * assign39410_e51970);
            let assign39410_e51974: f64 = (1.0 - locals.var_wsrhstep);
            let assign39410_e51975: f64 = (assign39410_e51971 / assign39410_e51974);
            let assign39410_e51977: f64 = (assign39410_e51975 + locals.var_wsrhstep);
            let assign39410_e51981: f64 = (2.0 * locals.var_pgatd_i);
            let assign39410_e51982: f64 = (1.0 - assign39410_e51981);
            let assign39410_e51983: f64 = (assign39410_e51977 * assign39410_e51982);
            locals.var_dwsrh = assign39410_e51983;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 == 0.0)) {
            let assign39420_e51997: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign39420_e51997;
        }
        let assign39430_e52002: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard804 = assign39430_e52002;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 == 0.0)) && (locals.var_guard804 != 0.0)) {
            let assign39440_e52016: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv_d);
            let assign39440_e52017: f64 = (assign39440_e52016).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39440_e52017, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 == 0.0)) && (locals.var_guard804 == 0.0)) {
            let assign39450_e52034: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv_d);
            let assign39450_e52036: f64 = (assign39450_e52034).powf(locals.var_pgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39450_e52036, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 == 0.0)) {
            let assign39460_e52050: f64 = (locals.var_wdepnulrgat_d * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign39460_e52050, (locals.var_wdepnulrgat_d * locals.var_tmp_dn5), (locals.var_wdepnulrgat_d * locals.var_tmp_dn6), (locals.var_wdepnulrgat_d * locals.var_tmp_dn7), (locals.var_wdepnulrgat_d * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 == 0.0)) {
            let assign39470_e52065: f64 = (locals.var_zinv - 1.0);
            let assign39470_e52067: f64 = (assign39470_e52065 * locals.var_wdep);
            let assign39470_e52068: f64 = (locals.var_ftdgat_d * assign39470_e52067);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign39470_e52068, (locals.var_ftdgat_d * (assign39470_e52065 * locals.var_wdep_dn5)), (locals.var_ftdgat_d * (assign39470_e52065 * locals.var_wdep_dn6)), (locals.var_ftdgat_d * (assign39470_e52065 * locals.var_wdep_dn7)), (locals.var_ftdgat_d * (assign39470_e52065 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 == 0.0)) {
            let assign39480_e52083: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign39480_e52084: f64 = (locals.var_csrhgatd_i * assign39480_e52083);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign39480_e52084, (locals.var_csrhgatd_i * (locals.var_asrh_dn5 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn6 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn7 * locals.var_wsrh)), (locals.var_csrhgatd_i * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign39490_e52089: f64 = if locals.var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard805 = assign39490_e52089;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39510_e52113: f64 = (locals.var_wdep * locals.var_one_minus_pgat_d);
            let assign39510_e52115: f64 = (assign39510_e52113 / locals.var_vbi_minus_vjsrh);
            let assign39510_e52116: f64 = (locals.var_btatpartgat_d * assign39510_e52115);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign39510_e52116, (locals.var_btatpartgat_d * ((locals.var_wdep_dn5 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn6 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn7 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat_d * ((locals.var_wdep_dn8 * locals.var_one_minus_pgat_d) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39520_e52130: f64 = (0.666666666666667 * locals.var_atatgat_d);
            let assign39520_e52132: f64 = (assign39520_e52130 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign39520_e52132, (-((assign39520_e52130 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign39520_e52130 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign39520_e52130 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign39520_e52130 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39530_e52146: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign39530_e52146, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39540_e52160: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign39540_e52163: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign39540_e52165: f64 = (assign39540_e52163 + 1.0);
            let assign39540_e52166: f64 = (assign39540_e52160 / assign39540_e52165);
            let assign39540_e52167: f64 = (assign39540_e52166).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign39540_e52167, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign39540_e52165) - (assign39540_e52160 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign39540_e52165 * assign39540_e52165)) / (2.0 * assign39540_e52167)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign39540_e52165) - (assign39540_e52160 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign39540_e52165 * assign39540_e52165)) / (2.0 * assign39540_e52167)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign39540_e52165) - (assign39540_e52160 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign39540_e52165 * assign39540_e52165)) / (2.0 * assign39540_e52167)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign39540_e52165) - (assign39540_e52160 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign39540_e52165 * assign39540_e52165)) / (2.0 * assign39540_e52167)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39550_e52180: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign39550_e52180, (locals.var_umax_dn5 / (2.0 * assign39550_e52180)), (locals.var_umax_dn6 / (2.0 * assign39550_e52180)), (locals.var_umax_dn7 / (2.0 * assign39550_e52180)), (locals.var_umax_dn8 / (2.0 * assign39550_e52180)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39560_e52194: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign39560_e52194, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign39570_e52198: f64 = (-locals.var_pgatd_i);
        let assign39570_e52200: f64 = (assign39570_e52198 * locals.var_one_over_one_minus_pgat_d);
        let assign39570_e52202: f64 = (-1.0);
        let assign39570_e52203: f64 = if assign39570_e52200 == assign39570_e52202 { 1.0 } else { 0.0 };
        locals.var_guard806 = assign39570_e52203;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) && (locals.var_guard806 != 0.0)) {
            let assign39580_e52219: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign39580_e52220: f64 = (1.0 + assign39580_e52219);
            let assign39580_e52221: f64 = (1.0 / assign39580_e52220);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign39580_e52221, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign39580_e52220 * assign39580_e52220))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign39580_e52220 * assign39580_e52220))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign39580_e52220 * assign39580_e52220))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign39580_e52220 * assign39580_e52220))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) && (locals.var_guard806 == 0.0)) {
            let assign39590_e52239: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign39590_e52240: f64 = (1.0 + assign39590_e52239);
            let assign39590_e52242: f64 = (-locals.var_pgatd_i);
            let assign39590_e52244: f64 = (assign39590_e52242 * locals.var_one_over_one_minus_pgat_d);
            let assign39590_e52245: f64 = (assign39590_e52240).powf(assign39590_e52244);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign39590_e52245, if 0.0 == 0.0 && ((assign39590_e52244) as f64).is_finite() && ((assign39590_e52244) as f64).fract() == 0.0 { if assign39590_e52244 == 0.0 { 0.0 } else { (assign39590_e52244 * ((assign39590_e52240).powf(assign39590_e52244 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign39590_e52245 * (assign39590_e52244 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign39590_e52240))) }, if 0.0 == 0.0 && ((assign39590_e52244) as f64).is_finite() && ((assign39590_e52244) as f64).fract() == 0.0 { if assign39590_e52244 == 0.0 { 0.0 } else { (assign39590_e52244 * ((assign39590_e52240).powf(assign39590_e52244 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign39590_e52245 * (assign39590_e52244 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign39590_e52240))) }, if 0.0 == 0.0 && ((assign39590_e52244) as f64).is_finite() && ((assign39590_e52244) as f64).fract() == 0.0 { if assign39590_e52244 == 0.0 { 0.0 } else { (assign39590_e52244 * ((assign39590_e52240).powf(assign39590_e52244 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign39590_e52245 * (assign39590_e52244 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign39590_e52240))) }, if 0.0 == 0.0 && ((assign39590_e52244) as f64).is_finite() && ((assign39590_e52244) as f64).fract() == 0.0 { if assign39590_e52244 == 0.0 { 0.0 } else { (assign39590_e52244 * ((assign39590_e52240).powf(assign39590_e52244 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign39590_e52245 * (assign39590_e52244 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign39590_e52240))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39600_e52259: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign39600_e52262: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign39600_e52263: f64 = (assign39600_e52259 / assign39600_e52262);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign39600_e52263, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign39600_e52262) - (assign39600_e52259 * locals.var_wgamma_dn5)) / (assign39600_e52262 * assign39600_e52262)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign39600_e52262) - (assign39600_e52259 * locals.var_wgamma_dn6)) / (assign39600_e52262 * assign39600_e52262)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign39600_e52262) - (assign39600_e52259 * locals.var_wgamma_dn7)) / (assign39600_e52262 * assign39600_e52262)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign39600_e52262) - (assign39600_e52259 * locals.var_wgamma_dn8)) / (assign39600_e52262 * assign39600_e52262)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39610_e52278: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign39610_e52279: f64 = (0.375 * assign39610_e52278);
            let assign39610_e52280: f64 = (assign39610_e52279).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign39610_e52280, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign39610_e52280)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign39610_e52280)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign39610_e52280)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign39610_e52280)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39620_e52295: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign39620_e52296: f64 = (2.0 * assign39620_e52295);
            let assign39620_e52298: f64 = (assign39620_e52296 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign39620_e52298, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39630_e52312: f64 = (locals.var_atatgat_d * locals.var_twoatatoverthreebtat);
            let assign39630_e52314: f64 = (assign39630_e52312 * locals.var_sqrtumax);
            let assign39630_e52317: f64 = (locals.var_atatgat_d * locals.var_umax);
            let assign39630_e52318: f64 = (assign39630_e52314 - assign39630_e52317);
            let assign39630_e52322: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign39630_e52323: f64 = (0.5 * assign39630_e52322);
            let assign39630_e52324: f64 = (assign39630_e52318 + assign39630_e52323);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign39630_e52324, (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign39630_e52312 * locals.var_sqrtumax_dn5)) - (locals.var_atatgat_d * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign39630_e52312 * locals.var_sqrtumax_dn6)) - (locals.var_atatgat_d * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign39630_e52312 * locals.var_sqrtumax_dn7)) - (locals.var_atatgat_d * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatgat_d * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign39630_e52312 * locals.var_sqrtumax_dn8)) - (locals.var_atatgat_d * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39640_e52338: f64 = (locals.var_ltat - 1.0);
            let assign39640_e52340: f64 = (assign39640_e52338 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign39640_e52340, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign39640_e52338 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign39640_e52338 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign39640_e52338 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign39640_e52338 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39650_e52354: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign39650_e52354, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign39660_e52359: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard807 = assign39660_e52359;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) && (locals.var_guard807 != 0.0)) {
            let assign39670_e52375: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign39670_e52376: f64 = (1.0 + assign39670_e52375);
            let assign39670_e52377: f64 = (1.0 / assign39670_e52376);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign39670_e52377, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign39670_e52376 * assign39670_e52376))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign39670_e52376 * assign39670_e52376))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign39670_e52376 * assign39670_e52376))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign39670_e52376 * assign39670_e52376))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) && (locals.var_guard807 == 0.0)) {
            let assign39680_e52396: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign39680_e52397: f64 = (1.0 - assign39680_e52396);
            let assign39680_e52398: f64 = (1.0 / assign39680_e52397);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign39680_e52398, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign39680_e52397 * assign39680_e52397))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign39680_e52397 * assign39680_e52397))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign39680_e52397 * assign39680_e52397))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign39680_e52397 * assign39680_e52397))), );
        }
        let assign39690_e52402: f64 = (-locals.var_ysq);
        let assign39690_e52404: f64 = (assign39690_e52402 + locals.var_mtat);
        let assign39690_e52406: f64 = (-230.25850929940458);
        let assign39690_e52407: f64 = if assign39690_e52404 > assign39690_e52406 { 1.0 } else { 0.0 };
        locals.var_guard808 = assign39690_e52407;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) && (locals.var_guard808 != 0.0)) {
            let assign39700_e52420: f64 = (-locals.var_ysq);
            let assign39700_e52422: f64 = (assign39700_e52420 + locals.var_mtat);
            let assign39700_e52423: f64 = (assign39700_e52422).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39700_e52423, (assign39700_e52423 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign39700_e52423 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign39700_e52423 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign39700_e52423 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) && (locals.var_guard808 == 0.0)) {
            let assign39710_e52441: f64 = (-230.25850929940458);
            let assign39710_e52443: f64 = (-locals.var_ysq);
            let assign39710_e52445: f64 = (assign39710_e52443 + locals.var_mtat);
            let assign39710_e52446: f64 = (assign39710_e52441 - assign39710_e52445);
            let assign39710_e52450: f64 = (-230.25850929940458);
            let assign39710_e52452: f64 = (-locals.var_ysq);
            let assign39710_e52454: f64 = (assign39710_e52452 + locals.var_mtat);
            let assign39710_e52455: f64 = (assign39710_e52450 - assign39710_e52454);
            let assign39710_e52458: f64 = (-230.25850929940458);
            let assign39710_e52460: f64 = (-locals.var_ysq);
            let assign39710_e52462: f64 = (assign39710_e52460 + locals.var_mtat);
            let assign39710_e52463: f64 = (assign39710_e52458 - assign39710_e52462);
            let assign39710_e52465: f64 = (assign39710_e52463 * 0.3333333333333333);
            let assign39710_e52466: f64 = (1.0 + assign39710_e52465);
            let assign39710_e52467: f64 = (assign39710_e52455 * assign39710_e52466);
            let assign39710_e52468: f64 = (0.5 * assign39710_e52467);
            let assign39710_e52469: f64 = (1.0 + assign39710_e52468);
            let assign39710_e52470: f64 = (assign39710_e52446 * assign39710_e52469);
            let assign39710_e52471: f64 = (1.0 + assign39710_e52470);
            let assign39710_e52472: f64 = (1e-100 / assign39710_e52471);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39710_e52472, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign39710_e52469) + (assign39710_e52446 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign39710_e52466) + (assign39710_e52455 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign39710_e52471 * assign39710_e52471))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign39710_e52469) + (assign39710_e52446 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign39710_e52466) + (assign39710_e52455 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign39710_e52471 * assign39710_e52471))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign39710_e52469) + (assign39710_e52446 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign39710_e52466) + (assign39710_e52455 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign39710_e52471 * assign39710_e52471))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign39710_e52469) + (assign39710_e52446 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign39710_e52466) + (assign39710_e52455 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign39710_e52471 * assign39710_e52471))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39720_e52486: f64 = (0.29214664 * locals.var_terfc);
            let assign39720_e52490: f64 = (locals.var_terfc * locals.var_terfc);
            let assign39720_e52491: f64 = (locals.var_berfc * assign39720_e52490);
            let assign39720_e52492: f64 = (assign39720_e52486 + assign39720_e52491);
            let assign39720_e52496: f64 = (locals.var_terfc * locals.var_terfc);
            let assign39720_e52498: f64 = (assign39720_e52496 * locals.var_terfc);
            let assign39720_e52499: f64 = (locals.var_cerfc * assign39720_e52498);
            let assign39720_e52500: f64 = (assign39720_e52492 + assign39720_e52499);
            let assign39720_e52502: f64 = (assign39720_e52500 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign39720_e52502, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign39720_e52496 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign39720_e52500 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign39720_e52496 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign39720_e52500 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign39720_e52496 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign39720_e52500 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign39720_e52496 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign39720_e52500 * locals.var_tmp_dn8)), );
        }
        let assign39730_e52507: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard809 = assign39730_e52507;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) && (locals.var_guard809 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign39750_e52524: f64 = (-230.25850929940458);
        let assign39750_e52525: f64 = if locals.var_mtat > assign39750_e52524 { 1.0 } else { 0.0 };
        locals.var_guard810 = assign39750_e52525;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) && (locals.var_guard809 == 0.0)) && (locals.var_guard810 != 0.0)) {
            let assign39760_e52541: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39760_e52541, (assign39760_e52541 * locals.var_mtat_dn5), (assign39760_e52541 * locals.var_mtat_dn6), (assign39760_e52541 * locals.var_mtat_dn7), (assign39760_e52541 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) && (locals.var_guard809 == 0.0)) && (locals.var_guard810 == 0.0)) {
            let assign39770_e52562: f64 = (-230.25850929940458);
            let assign39770_e52564: f64 = (assign39770_e52562 - locals.var_mtat);
            let assign39770_e52568: f64 = (-230.25850929940458);
            let assign39770_e52570: f64 = (assign39770_e52568 - locals.var_mtat);
            let assign39770_e52573: f64 = (-230.25850929940458);
            let assign39770_e52575: f64 = (assign39770_e52573 - locals.var_mtat);
            let assign39770_e52577: f64 = (assign39770_e52575 * 0.3333333333333333);
            let assign39770_e52578: f64 = (1.0 + assign39770_e52577);
            let assign39770_e52579: f64 = (assign39770_e52570 * assign39770_e52578);
            let assign39770_e52580: f64 = (0.5 * assign39770_e52579);
            let assign39770_e52581: f64 = (1.0 + assign39770_e52580);
            let assign39770_e52582: f64 = (assign39770_e52564 * assign39770_e52581);
            let assign39770_e52583: f64 = (1.0 + assign39770_e52582);
            let assign39770_e52584: f64 = (1e-100 / assign39770_e52583);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39770_e52584, (-((1e-100 * (((-locals.var_mtat_dn5) * assign39770_e52581) + (assign39770_e52564 * (0.5 * (((-locals.var_mtat_dn5) * assign39770_e52578) + (assign39770_e52570 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign39770_e52583 * assign39770_e52583))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign39770_e52581) + (assign39770_e52564 * (0.5 * (((-locals.var_mtat_dn6) * assign39770_e52578) + (assign39770_e52570 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign39770_e52583 * assign39770_e52583))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign39770_e52581) + (assign39770_e52564 * (0.5 * (((-locals.var_mtat_dn7) * assign39770_e52578) + (assign39770_e52570 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign39770_e52583 * assign39770_e52583))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign39770_e52581) + (assign39770_e52564 * (0.5 * (((-locals.var_mtat_dn8) * assign39770_e52578) + (assign39770_e52570 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign39770_e52583 * assign39770_e52583))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) && (locals.var_guard809 == 0.0)) {
            let assign39780_e52601: f64 = (2.0 * locals.var_tmp);
            let assign39780_e52603: f64 = (assign39780_e52601 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign39780_e52603, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39790_e52617: f64 = (1.772453850905516 * 0.5);
            let assign39790_e52620: f64 = (locals.var_atatgat_d * locals.var_erfctimesexpmtat);
            let assign39790_e52622: f64 = (assign39790_e52620 / locals.var_ktat);
            let assign39790_e52623: f64 = (assign39790_e52617 * assign39790_e52622);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign39790_e52623, (assign39790_e52617 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign39790_e52620 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign39790_e52617 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign39790_e52620 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign39790_e52617 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign39790_e52620 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign39790_e52617 * ((((locals.var_atatgat_d * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign39790_e52620 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard805 == 0.0)) {
            let assign39800_e52638: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign39800_e52640: f64 = (assign39800_e52638 * locals.var_wtat);
            let assign39800_e52641: f64 = (locals.var_ctatgatd_i * assign39800_e52640);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign39800_e52641, (locals.var_ctatgatd_i * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign39800_e52638 * locals.var_wtat_dn5))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign39800_e52638 * locals.var_wtat_dn6))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign39800_e52638 * locals.var_wtat_dn7))), (locals.var_ctatgatd_i * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign39800_e52638 * locals.var_wtat_dn8))), );
        }
        let assign39810_e52646: f64 = if locals.var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard811 = assign39810_e52646;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard811 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign39830_e52660: f64 = if locals.var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard812 = assign39830_e52660;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard811 == 0.0)) && (locals.var_guard812 != 0.0)) {
            let assign39840_e52674: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign39840_e52676: f64 = (assign39840_e52674 * locals.var_vbirgatinv_d);
            let assign39840_e52677: f64 = (assign39840_e52676).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39840_e52677, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard811 == 0.0)) && (locals.var_guard812 == 0.0)) {
            let assign39850_e52694: f64 = (locals.var_vbirgatd_i - locals.var_vbbt);
            let assign39850_e52696: f64 = (assign39850_e52694 * locals.var_vbirgatinv_d);
            let assign39850_e52698: f64 = (assign39850_e52696).powf(locals.var_pgatd_i);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign39850_e52698, 0.0, 0.0, 0.0, 0.0, );
        }
    }
}

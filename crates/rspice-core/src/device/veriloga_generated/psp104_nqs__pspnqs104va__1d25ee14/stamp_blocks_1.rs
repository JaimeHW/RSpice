#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) && (locals.var_guard260 == 0.0)) {
            let assign16180_e14042: f64 = (-230.25850929940458);
            let assign16180_e14044: f64 = (-locals.var_ysq);
            let assign16180_e14046: f64 = (assign16180_e14044 + locals.var_mtat);
            let assign16180_e14047: f64 = (assign16180_e14042 - assign16180_e14046);
            let assign16180_e14051: f64 = (-230.25850929940458);
            let assign16180_e14053: f64 = (-locals.var_ysq);
            let assign16180_e14055: f64 = (assign16180_e14053 + locals.var_mtat);
            let assign16180_e14056: f64 = (assign16180_e14051 - assign16180_e14055);
            let assign16180_e14059: f64 = (-230.25850929940458);
            let assign16180_e14061: f64 = (-locals.var_ysq);
            let assign16180_e14063: f64 = (assign16180_e14061 + locals.var_mtat);
            let assign16180_e14064: f64 = (assign16180_e14059 - assign16180_e14063);
            let assign16180_e14066: f64 = (assign16180_e14064 * 0.3333333333333333);
            let assign16180_e14067: f64 = (1.0 + assign16180_e14066);
            let assign16180_e14068: f64 = (assign16180_e14056 * assign16180_e14067);
            let assign16180_e14069: f64 = (0.5 * assign16180_e14068);
            let assign16180_e14070: f64 = (1.0 + assign16180_e14069);
            let assign16180_e14071: f64 = (assign16180_e14047 * assign16180_e14070);
            let assign16180_e14072: f64 = (1.0 + assign16180_e14071);
            let assign16180_e14073: f64 = (1e-100 / assign16180_e14072);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16180_e14073, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign16180_e14070) + (assign16180_e14047 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign16180_e14067) + (assign16180_e14056 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign16180_e14072 * assign16180_e14072))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign16180_e14070) + (assign16180_e14047 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign16180_e14067) + (assign16180_e14056 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign16180_e14072 * assign16180_e14072))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign16180_e14070) + (assign16180_e14047 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign16180_e14067) + (assign16180_e14056 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign16180_e14072 * assign16180_e14072))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign16180_e14070) + (assign16180_e14047 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign16180_e14067) + (assign16180_e14056 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign16180_e14072 * assign16180_e14072))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16190_e14087: f64 = (0.29214664 * locals.var_terfc);
            let assign16190_e14091: f64 = (locals.var_terfc * locals.var_terfc);
            let assign16190_e14092: f64 = (locals.var_berfc * assign16190_e14091);
            let assign16190_e14093: f64 = (assign16190_e14087 + assign16190_e14092);
            let assign16190_e14097: f64 = (locals.var_terfc * locals.var_terfc);
            let assign16190_e14099: f64 = (assign16190_e14097 * locals.var_terfc);
            let assign16190_e14100: f64 = (locals.var_cerfc * assign16190_e14099);
            let assign16190_e14101: f64 = (assign16190_e14093 + assign16190_e14100);
            let assign16190_e14103: f64 = (assign16190_e14101 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign16190_e14103, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign16190_e14097 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign16190_e14101 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign16190_e14097 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign16190_e14101 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign16190_e14097 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign16190_e14101 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign16190_e14097 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign16190_e14101 * locals.var_tmp_dn8)), );
        }
        let assign16200_e14108: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign16200_e14108;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) && (locals.var_guard261 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign16220_e14125: f64 = (-230.25850929940458);
        let assign16220_e14126: f64 = if locals.var_mtat > assign16220_e14125 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign16220_e14126;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) && (locals.var_guard261 == 0.0)) && (locals.var_guard262 != 0.0)) {
            let assign16230_e14142: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16230_e14142, (assign16230_e14142 * locals.var_mtat_dn5), (assign16230_e14142 * locals.var_mtat_dn6), (assign16230_e14142 * locals.var_mtat_dn7), (assign16230_e14142 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) && (locals.var_guard261 == 0.0)) && (locals.var_guard262 == 0.0)) {
            let assign16240_e14163: f64 = (-230.25850929940458);
            let assign16240_e14165: f64 = (assign16240_e14163 - locals.var_mtat);
            let assign16240_e14169: f64 = (-230.25850929940458);
            let assign16240_e14171: f64 = (assign16240_e14169 - locals.var_mtat);
            let assign16240_e14174: f64 = (-230.25850929940458);
            let assign16240_e14176: f64 = (assign16240_e14174 - locals.var_mtat);
            let assign16240_e14178: f64 = (assign16240_e14176 * 0.3333333333333333);
            let assign16240_e14179: f64 = (1.0 + assign16240_e14178);
            let assign16240_e14180: f64 = (assign16240_e14171 * assign16240_e14179);
            let assign16240_e14181: f64 = (0.5 * assign16240_e14180);
            let assign16240_e14182: f64 = (1.0 + assign16240_e14181);
            let assign16240_e14183: f64 = (assign16240_e14165 * assign16240_e14182);
            let assign16240_e14184: f64 = (1.0 + assign16240_e14183);
            let assign16240_e14185: f64 = (1e-100 / assign16240_e14184);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16240_e14185, (-((1e-100 * (((-locals.var_mtat_dn5) * assign16240_e14182) + (assign16240_e14165 * (0.5 * (((-locals.var_mtat_dn5) * assign16240_e14179) + (assign16240_e14171 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign16240_e14184 * assign16240_e14184))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign16240_e14182) + (assign16240_e14165 * (0.5 * (((-locals.var_mtat_dn6) * assign16240_e14179) + (assign16240_e14171 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign16240_e14184 * assign16240_e14184))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign16240_e14182) + (assign16240_e14165 * (0.5 * (((-locals.var_mtat_dn7) * assign16240_e14179) + (assign16240_e14171 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign16240_e14184 * assign16240_e14184))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign16240_e14182) + (assign16240_e14165 * (0.5 * (((-locals.var_mtat_dn8) * assign16240_e14179) + (assign16240_e14171 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign16240_e14184 * assign16240_e14184))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) && (locals.var_guard261 == 0.0)) {
            let assign16250_e14202: f64 = (2.0 * locals.var_tmp);
            let assign16250_e14204: f64 = (assign16250_e14202 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign16250_e14204, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16260_e14218: f64 = (1.772453850905516 * 0.5);
            let assign16260_e14221: f64 = (locals.var_atatbot * locals.var_erfctimesexpmtat);
            let assign16260_e14223: f64 = (assign16260_e14221 / locals.var_ktat);
            let assign16260_e14224: f64 = (assign16260_e14218 * assign16260_e14223);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign16260_e14224, (assign16260_e14218 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign16260_e14221 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign16260_e14218 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign16260_e14221 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign16260_e14218 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign16260_e14221 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign16260_e14218 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign16260_e14221 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard257 == 0.0)) {
            let assign16270_e14239: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign16270_e14241: f64 = (assign16270_e14239 * locals.var_wtat);
            let assign16270_e14242: f64 = (p.p845 * assign16270_e14241);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign16270_e14242, (p.p845 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign16270_e14239 * locals.var_wtat_dn5))), (p.p845 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign16270_e14239 * locals.var_wtat_dn6))), (p.p845 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign16270_e14239 * locals.var_wtat_dn7))), (p.p845 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign16270_e14239 * locals.var_wtat_dn8))), );
        }
        let assign16280_e14247: f64 = if p.p851 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign16280_e14247;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard263 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign16300_e14261: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign16300_e14261;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard263 == 0.0)) && (locals.var_guard264 != 0.0)) {
            let assign16310_e14275: f64 = (p.p828 - locals.var_vbbt);
            let assign16310_e14277: f64 = (assign16310_e14275 * locals.var_vbirbotinv);
            let assign16310_e14278: f64 = (assign16310_e14277).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16310_e14278, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard263 == 0.0)) && (locals.var_guard264 == 0.0)) {
            let assign16320_e14295: f64 = (p.p828 - locals.var_vbbt);
            let assign16320_e14297: f64 = (assign16320_e14295 * locals.var_vbirbotinv);
            let assign16320_e14299: f64 = (assign16320_e14297).powf(p.p831);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16320_e14299, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard263 == 0.0)) {
            let assign16330_e14314: f64 = (p.p828 - locals.var_vbbt);
            let assign16330_e14316: f64 = (assign16330_e14314 * locals.var_wdepnulrinvbot);
            let assign16330_e14318: f64 = (assign16330_e14316 / locals.var_tmp);
            let assign16330_e14319: f64 = (locals.var_one_over_one_minus_pbot * assign16330_e14318);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign16330_e14319, (locals.var_one_over_one_minus_pbot * (-((assign16330_e14316 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign16330_e14316 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign16330_e14316 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign16330_e14316 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign16340_e14323: f64 = (-locals.var_fbbtbot);
        let assign16340_e14325: f64 = (assign16340_e14323 / locals.var_fmaxr);
        let assign16340_e14326: f64 = (assign16340_e14325).abs();
        let assign16340_e14328: f64 = if assign16340_e14326 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign16340_e14328;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard263 == 0.0)) && (locals.var_guard265 != 0.0)) {
            let assign16350_e14341: f64 = (-locals.var_fbbtbot);
            let assign16350_e14343: f64 = (assign16350_e14341 / locals.var_fmaxr);
            let assign16350_e14344: f64 = (assign16350_e14343).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16350_e14344, (assign16350_e14344 * (-((assign16350_e14341 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign16350_e14344 * (-((assign16350_e14341 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign16350_e14344 * (-((assign16350_e14341 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign16350_e14344 * (-((assign16350_e14341 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign16360_e14348: f64 = (-locals.var_fbbtbot);
        let assign16360_e14350: f64 = (assign16360_e14348 / locals.var_fmaxr);
        let assign16360_e14352: f64 = if assign16360_e14350 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard266 = assign16360_e14352;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard263 == 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) {
            let assign16370_e14370: f64 = (-230.25850929940458);
            let assign16370_e14372: f64 = (-locals.var_fbbtbot);
            let assign16370_e14374: f64 = (assign16370_e14372 / locals.var_fmaxr);
            let assign16370_e14375: f64 = (assign16370_e14370 - assign16370_e14374);
            let assign16370_e14379: f64 = (-230.25850929940458);
            let assign16370_e14381: f64 = (-locals.var_fbbtbot);
            let assign16370_e14383: f64 = (assign16370_e14381 / locals.var_fmaxr);
            let assign16370_e14384: f64 = (assign16370_e14379 - assign16370_e14383);
            let assign16370_e14387: f64 = (-230.25850929940458);
            let assign16370_e14389: f64 = (-locals.var_fbbtbot);
            let assign16370_e14391: f64 = (assign16370_e14389 / locals.var_fmaxr);
            let assign16370_e14392: f64 = (assign16370_e14387 - assign16370_e14391);
            let assign16370_e14394: f64 = (assign16370_e14392 * 0.3333333333333333);
            let assign16370_e14395: f64 = (1.0 + assign16370_e14394);
            let assign16370_e14396: f64 = (assign16370_e14384 * assign16370_e14395);
            let assign16370_e14397: f64 = (0.5 * assign16370_e14396);
            let assign16370_e14398: f64 = (1.0 + assign16370_e14397);
            let assign16370_e14399: f64 = (assign16370_e14375 * assign16370_e14398);
            let assign16370_e14400: f64 = (1.0 + assign16370_e14399);
            let assign16370_e14401: f64 = (1e-100 / assign16370_e14400);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16370_e14401, (-((1e-100 * (((-(-((assign16370_e14372 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign16370_e14398) + (assign16370_e14375 * (0.5 * (((-(-((assign16370_e14381 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign16370_e14395) + (assign16370_e14384 * ((-(-((assign16370_e14389 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign16370_e14400 * assign16370_e14400))), (-((1e-100 * (((-(-((assign16370_e14372 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign16370_e14398) + (assign16370_e14375 * (0.5 * (((-(-((assign16370_e14381 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign16370_e14395) + (assign16370_e14384 * ((-(-((assign16370_e14389 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign16370_e14400 * assign16370_e14400))), (-((1e-100 * (((-(-((assign16370_e14372 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign16370_e14398) + (assign16370_e14375 * (0.5 * (((-(-((assign16370_e14381 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign16370_e14395) + (assign16370_e14384 * ((-(-((assign16370_e14389 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign16370_e14400 * assign16370_e14400))), (-((1e-100 * (((-(-((assign16370_e14372 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign16370_e14398) + (assign16370_e14375 * (0.5 * (((-(-((assign16370_e14381 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign16370_e14395) + (assign16370_e14384 * ((-(-((assign16370_e14389 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign16370_e14400 * assign16370_e14400))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard263 == 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) {
            let assign16380_e14422: f64 = (-locals.var_fbbtbot);
            let assign16380_e14424: f64 = (assign16380_e14422 / locals.var_fmaxr);
            let assign16380_e14426: f64 = (assign16380_e14424 - 230.25850929940458);
            let assign16380_e14430: f64 = (-locals.var_fbbtbot);
            let assign16380_e14432: f64 = (assign16380_e14430 / locals.var_fmaxr);
            let assign16380_e14434: f64 = (assign16380_e14432 - 230.25850929940458);
            let assign16380_e14437: f64 = (-locals.var_fbbtbot);
            let assign16380_e14439: f64 = (assign16380_e14437 / locals.var_fmaxr);
            let assign16380_e14441: f64 = (assign16380_e14439 - 230.25850929940458);
            let assign16380_e14443: f64 = (assign16380_e14441 * 0.3333333333333333);
            let assign16380_e14444: f64 = (1.0 + assign16380_e14443);
            let assign16380_e14445: f64 = (assign16380_e14434 * assign16380_e14444);
            let assign16380_e14446: f64 = (0.5 * assign16380_e14445);
            let assign16380_e14447: f64 = (1.0 + assign16380_e14446);
            let assign16380_e14448: f64 = (assign16380_e14426 * assign16380_e14447);
            let assign16380_e14449: f64 = (1.0 + assign16380_e14448);
            let assign16380_e14450: f64 = (1e100 * assign16380_e14449);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16380_e14450, (1e100 * (((-((assign16380_e14422 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign16380_e14447) + (assign16380_e14426 * (0.5 * (((-((assign16380_e14430 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign16380_e14444) + (assign16380_e14434 * ((-((assign16380_e14437 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign16380_e14422 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign16380_e14447) + (assign16380_e14426 * (0.5 * (((-((assign16380_e14430 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign16380_e14444) + (assign16380_e14434 * ((-((assign16380_e14437 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign16380_e14422 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign16380_e14447) + (assign16380_e14426 * (0.5 * (((-((assign16380_e14430 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign16380_e14444) + (assign16380_e14434 * ((-((assign16380_e14437 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign16380_e14422 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign16380_e14447) + (assign16380_e14426 * (0.5 * (((-((assign16380_e14430 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign16380_e14444) + (assign16380_e14434 * ((-((assign16380_e14437 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard263 == 0.0)) {
            let assign16390_e14465: f64 = (locals.var_v1 * locals.var_fmaxr);
            let assign16390_e14467: f64 = (assign16390_e14465 * locals.var_fmaxr);
            let assign16390_e14469: f64 = (assign16390_e14467 * locals.var_tmp);
            let assign16390_e14470: f64 = (p.p851 * assign16390_e14469);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign16390_e14470, (p.p851 * (((((locals.var_v1 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign16390_e14465 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign16390_e14467 * locals.var_tmp_dn5))), (p.p851 * (((((locals.var_v1 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign16390_e14465 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign16390_e14467 * locals.var_tmp_dn6))), (p.p851 * (((((locals.var_v1 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign16390_e14465 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign16390_e14467 * locals.var_tmp_dn7))), (p.p851 * (((((locals.var_v1 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign16390_e14465 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign16390_e14467 * locals.var_tmp_dn8))), );
        }
        let assign16400_e14475: f64 = if p.p860 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard267 = assign16400_e14475;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard267 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign16420_e14489: f64 = (-locals.var_alphaav);
        let assign16420_e14491: f64 = (assign16420_e14489 * p.p860);
        let assign16420_e14492: f64 = if locals.var_vav > assign16420_e14491 { 1.0 } else { 0.0 };
        locals.var_guard268 = assign16420_e14492;
        let assign16430_e14495: f64 = if p.p863 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard269 = assign16430_e14495;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard267 == 0.0)) && (locals.var_guard268 != 0.0)) && (locals.var_guard269 != 0.0)) {
            let assign16440_e14511: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign16440_e14514: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign16440_e14515: f64 = (assign16440_e14511 * assign16440_e14514);
            let assign16440_e14518: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign16440_e14519: f64 = (assign16440_e14515 * assign16440_e14518);
            let assign16440_e14522: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign16440_e14523: f64 = (assign16440_e14519 * assign16440_e14522);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16440_e14523, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard267 == 0.0)) && (locals.var_guard268 != 0.0)) && (locals.var_guard269 == 0.0)) {
            let assign16450_e14542: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign16450_e14543: f64 = (assign16450_e14542).abs();
            let assign16450_e14545: f64 = (assign16450_e14543).powf(p.p863);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16450_e14545, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard267 == 0.0)) && (locals.var_guard268 != 0.0)) {
            let assign16460_e14562: f64 = (1.0 - locals.var_tmp);
            let assign16460_e14563: f64 = (1.0 / assign16460_e14562);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign16460_e14563, (-((-locals.var_tmp_dn5) / (assign16460_e14562 * assign16460_e14562))), (-((-locals.var_tmp_dn6) / (assign16460_e14562 * assign16460_e14562))), (-((-locals.var_tmp_dn7) / (assign16460_e14562 * assign16460_e14562))), (-((-locals.var_tmp_dn8) / (assign16460_e14562 * assign16460_e14562))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) && (locals.var_guard267 == 0.0)) && (locals.var_guard268 == 0.0)) {
            let assign16470_e14582: f64 = (locals.var_alphaav * p.p860);
            let assign16470_e14583: f64 = (locals.var_vav + assign16470_e14582);
            let assign16470_e14585: f64 = (assign16470_e14583 * locals.var_slopebot);
            let assign16470_e14586: f64 = (locals.var_fstopbot + assign16470_e14585);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign16470_e14586, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard253 == 0.0)) {
            let assign16480_e14598: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign16480_e14600: f64 = (assign16480_e14598 + locals.var_itat);
            let assign16480_e14602: f64 = (assign16480_e14600 + locals.var_ibbt);
            let assign16480_e14603: f64 = (p.p29 * assign16480_e14602);
            let assign16480_e14605: f64 = (assign16480_e14603 * locals.var_fbreakdown);
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (assign16480_e14605, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign16480_e14603 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign16480_e14603 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign16480_e14603 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign16480_e14603 * locals.var_fbreakdown_dn8)), );
        }
        let assign16490_e14610: f64 = if locals.var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard270 = assign16490_e14610;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 != 0.0)) {
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) {
            let assign16510_e14627: f64 = (locals.var_idsatsti * locals.var_idmult);
            locals.var_id__blk219 = assign16510_e14627;
        }
        let assign16520_e14636: f64 = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard271 = assign16520_e14636;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) {
            let assign16540_e14659: f64 = (locals.var_vbisti - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign16540_e14659;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) {
            let assign16550_e14675: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign16550_e14676: f64 = (1.0 - assign16550_e14675);
            let assign16550_e14677: f64 = (assign16550_e14676).sqrt();
            let assign16550_e14678: f64 = (1.0 - assign16550_e14677);
            locals.var_wsrhstep = assign16550_e14678;
        }
        let assign16560_e14683: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard272 = assign16560_e14683;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) && (locals.var_guard272 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) && (locals.var_guard272 == 0.0)) {
            let assign16580_e14712: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign16580_e14714: f64 = (locals.var_wsrhstep).ln();
            let assign16580_e14715: f64 = (assign16580_e14712 * assign16580_e14714);
            let assign16580_e14718: f64 = (1.0 - locals.var_wsrhstep);
            let assign16580_e14719: f64 = (assign16580_e14715 / assign16580_e14718);
            let assign16580_e14721: f64 = (assign16580_e14719 + locals.var_wsrhstep);
            let assign16580_e14725: f64 = (2.0 * p.p832);
            let assign16580_e14726: f64 = (1.0 - assign16580_e14725);
            let assign16580_e14727: f64 = (assign16580_e14721 * assign16580_e14726);
            locals.var_dwsrh = assign16580_e14727;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) {
            let assign16590_e14741: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign16590_e14741;
        }
        let assign16600_e14746: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign16600_e14746;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) && (locals.var_guard273 != 0.0)) {
            let assign16610_e14760: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv);
            let assign16610_e14761: f64 = (assign16610_e14760).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16610_e14761, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) && (locals.var_guard273 == 0.0)) {
            let assign16620_e14778: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv);
            let assign16620_e14780: f64 = (assign16620_e14778).powf(p.p832);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16620_e14780, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) {
            let assign16630_e14794: f64 = (locals.var_wdepnulrsti * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign16630_e14794, (locals.var_wdepnulrsti * locals.var_tmp_dn5), (locals.var_wdepnulrsti * locals.var_tmp_dn6), (locals.var_wdepnulrsti * locals.var_tmp_dn7), (locals.var_wdepnulrsti * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) {
            let assign16640_e14809: f64 = (locals.var_zinv - 1.0);
            let assign16640_e14811: f64 = (assign16640_e14809 * locals.var_wdep);
            let assign16640_e14812: f64 = (locals.var_ftdsti * assign16640_e14811);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign16640_e14812, (locals.var_ftdsti * (assign16640_e14809 * locals.var_wdep_dn5)), (locals.var_ftdsti * (assign16640_e14809 * locals.var_wdep_dn6)), (locals.var_ftdsti * (assign16640_e14809 * locals.var_wdep_dn7)), (locals.var_ftdsti * (assign16640_e14809 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) {
            let assign16650_e14827: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign16650_e14828: f64 = (p.p841 * assign16650_e14827);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign16650_e14828, (p.p841 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign16660_e14833: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard274 = assign16660_e14833;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16680_e14857: f64 = (locals.var_wdep * locals.var_one_minus_psti);
            let assign16680_e14859: f64 = (assign16680_e14857 / locals.var_vbi_minus_vjsrh);
            let assign16680_e14860: f64 = (locals.var_btatpartsti * assign16680_e14859);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign16680_e14860, (locals.var_btatpartsti * ((locals.var_wdep_dn5 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn6 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn7 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn8 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16690_e14874: f64 = (0.666666666666667 * locals.var_atatsti);
            let assign16690_e14876: f64 = (assign16690_e14874 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign16690_e14876, (-((assign16690_e14874 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign16690_e14874 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign16690_e14874 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign16690_e14874 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16700_e14890: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign16700_e14890, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16710_e14904: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign16710_e14907: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign16710_e14909: f64 = (assign16710_e14907 + 1.0);
            let assign16710_e14910: f64 = (assign16710_e14904 / assign16710_e14909);
            let assign16710_e14911: f64 = (assign16710_e14910).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign16710_e14911, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign16710_e14909) - (assign16710_e14904 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign16710_e14909 * assign16710_e14909)) / (2.0 * assign16710_e14911)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign16710_e14909) - (assign16710_e14904 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign16710_e14909 * assign16710_e14909)) / (2.0 * assign16710_e14911)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign16710_e14909) - (assign16710_e14904 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign16710_e14909 * assign16710_e14909)) / (2.0 * assign16710_e14911)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign16710_e14909) - (assign16710_e14904 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign16710_e14909 * assign16710_e14909)) / (2.0 * assign16710_e14911)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16720_e14924: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign16720_e14924, (locals.var_umax_dn5 / (2.0 * assign16720_e14924)), (locals.var_umax_dn6 / (2.0 * assign16720_e14924)), (locals.var_umax_dn7 / (2.0 * assign16720_e14924)), (locals.var_umax_dn8 / (2.0 * assign16720_e14924)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16730_e14938: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign16730_e14938, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign16740_e14942: f64 = (-p.p832);
        let assign16740_e14944: f64 = (assign16740_e14942 * locals.var_one_over_one_minus_psti);
        let assign16740_e14946: f64 = (-1.0);
        let assign16740_e14947: f64 = if assign16740_e14944 == assign16740_e14946 { 1.0 } else { 0.0 };
        locals.var_guard275 = assign16740_e14947;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) && (locals.var_guard275 != 0.0)) {
            let assign16750_e14963: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign16750_e14964: f64 = (1.0 + assign16750_e14963);
            let assign16750_e14965: f64 = (1.0 / assign16750_e14964);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign16750_e14965, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign16750_e14964 * assign16750_e14964))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign16750_e14964 * assign16750_e14964))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign16750_e14964 * assign16750_e14964))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign16750_e14964 * assign16750_e14964))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) && (locals.var_guard275 == 0.0)) {
            let assign16760_e14983: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign16760_e14984: f64 = (1.0 + assign16760_e14983);
            let assign16760_e14986: f64 = (-p.p832);
            let assign16760_e14988: f64 = (assign16760_e14986 * locals.var_one_over_one_minus_psti);
            let assign16760_e14989: f64 = (assign16760_e14984).powf(assign16760_e14988);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign16760_e14989, if 0.0 == 0.0 && ((assign16760_e14988) as f64).is_finite() && ((assign16760_e14988) as f64).fract() == 0.0 { if assign16760_e14988 == 0.0 { 0.0 } else { (assign16760_e14988 * ((assign16760_e14984).powf(assign16760_e14988 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign16760_e14989 * (assign16760_e14988 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign16760_e14984))) }, if 0.0 == 0.0 && ((assign16760_e14988) as f64).is_finite() && ((assign16760_e14988) as f64).fract() == 0.0 { if assign16760_e14988 == 0.0 { 0.0 } else { (assign16760_e14988 * ((assign16760_e14984).powf(assign16760_e14988 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign16760_e14989 * (assign16760_e14988 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign16760_e14984))) }, if 0.0 == 0.0 && ((assign16760_e14988) as f64).is_finite() && ((assign16760_e14988) as f64).fract() == 0.0 { if assign16760_e14988 == 0.0 { 0.0 } else { (assign16760_e14988 * ((assign16760_e14984).powf(assign16760_e14988 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign16760_e14989 * (assign16760_e14988 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign16760_e14984))) }, if 0.0 == 0.0 && ((assign16760_e14988) as f64).is_finite() && ((assign16760_e14988) as f64).fract() == 0.0 { if assign16760_e14988 == 0.0 { 0.0 } else { (assign16760_e14988 * ((assign16760_e14984).powf(assign16760_e14988 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign16760_e14989 * (assign16760_e14988 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign16760_e14984))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16770_e15003: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign16770_e15006: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign16770_e15007: f64 = (assign16770_e15003 / assign16770_e15006);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign16770_e15007, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign16770_e15006) - (assign16770_e15003 * locals.var_wgamma_dn5)) / (assign16770_e15006 * assign16770_e15006)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign16770_e15006) - (assign16770_e15003 * locals.var_wgamma_dn6)) / (assign16770_e15006 * assign16770_e15006)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign16770_e15006) - (assign16770_e15003 * locals.var_wgamma_dn7)) / (assign16770_e15006 * assign16770_e15006)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign16770_e15006) - (assign16770_e15003 * locals.var_wgamma_dn8)) / (assign16770_e15006 * assign16770_e15006)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16780_e15022: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign16780_e15023: f64 = (0.375 * assign16780_e15022);
            let assign16780_e15024: f64 = (assign16780_e15023).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign16780_e15024, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign16780_e15024)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign16780_e15024)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign16780_e15024)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign16780_e15024)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16790_e15039: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign16790_e15040: f64 = (2.0 * assign16790_e15039);
            let assign16790_e15042: f64 = (assign16790_e15040 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign16790_e15042, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16800_e15056: f64 = (locals.var_atatsti * locals.var_twoatatoverthreebtat);
            let assign16800_e15058: f64 = (assign16800_e15056 * locals.var_sqrtumax);
            let assign16800_e15061: f64 = (locals.var_atatsti * locals.var_umax);
            let assign16800_e15062: f64 = (assign16800_e15058 - assign16800_e15061);
            let assign16800_e15066: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign16800_e15067: f64 = (0.5 * assign16800_e15066);
            let assign16800_e15068: f64 = (assign16800_e15062 + assign16800_e15067);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign16800_e15068, (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign16800_e15056 * locals.var_sqrtumax_dn5)) - (locals.var_atatsti * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign16800_e15056 * locals.var_sqrtumax_dn6)) - (locals.var_atatsti * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign16800_e15056 * locals.var_sqrtumax_dn7)) - (locals.var_atatsti * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign16800_e15056 * locals.var_sqrtumax_dn8)) - (locals.var_atatsti * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16810_e15082: f64 = (locals.var_ltat - 1.0);
            let assign16810_e15084: f64 = (assign16810_e15082 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign16810_e15084, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign16810_e15082 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign16810_e15082 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign16810_e15082 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign16810_e15082 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16820_e15098: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign16820_e15098, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign16830_e15103: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign16830_e15103;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) && (locals.var_guard276 != 0.0)) {
            let assign16840_e15119: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign16840_e15120: f64 = (1.0 + assign16840_e15119);
            let assign16840_e15121: f64 = (1.0 / assign16840_e15120);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign16840_e15121, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign16840_e15120 * assign16840_e15120))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign16840_e15120 * assign16840_e15120))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign16840_e15120 * assign16840_e15120))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign16840_e15120 * assign16840_e15120))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) && (locals.var_guard276 == 0.0)) {
            let assign16850_e15140: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign16850_e15141: f64 = (1.0 - assign16850_e15140);
            let assign16850_e15142: f64 = (1.0 / assign16850_e15141);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign16850_e15142, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign16850_e15141 * assign16850_e15141))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign16850_e15141 * assign16850_e15141))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign16850_e15141 * assign16850_e15141))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign16850_e15141 * assign16850_e15141))), );
        }
        let assign16860_e15146: f64 = (-locals.var_ysq);
        let assign16860_e15148: f64 = (assign16860_e15146 + locals.var_mtat);
        let assign16860_e15150: f64 = (-230.25850929940458);
        let assign16860_e15151: f64 = if assign16860_e15148 > assign16860_e15150 { 1.0 } else { 0.0 };
        locals.var_guard277 = assign16860_e15151;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) && (locals.var_guard277 != 0.0)) {
            let assign16870_e15164: f64 = (-locals.var_ysq);
            let assign16870_e15166: f64 = (assign16870_e15164 + locals.var_mtat);
            let assign16870_e15167: f64 = (assign16870_e15166).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16870_e15167, (assign16870_e15167 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign16870_e15167 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign16870_e15167 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign16870_e15167 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) && (locals.var_guard277 == 0.0)) {
            let assign16880_e15185: f64 = (-230.25850929940458);
            let assign16880_e15187: f64 = (-locals.var_ysq);
            let assign16880_e15189: f64 = (assign16880_e15187 + locals.var_mtat);
            let assign16880_e15190: f64 = (assign16880_e15185 - assign16880_e15189);
            let assign16880_e15194: f64 = (-230.25850929940458);
            let assign16880_e15196: f64 = (-locals.var_ysq);
            let assign16880_e15198: f64 = (assign16880_e15196 + locals.var_mtat);
            let assign16880_e15199: f64 = (assign16880_e15194 - assign16880_e15198);
            let assign16880_e15202: f64 = (-230.25850929940458);
            let assign16880_e15204: f64 = (-locals.var_ysq);
            let assign16880_e15206: f64 = (assign16880_e15204 + locals.var_mtat);
            let assign16880_e15207: f64 = (assign16880_e15202 - assign16880_e15206);
            let assign16880_e15209: f64 = (assign16880_e15207 * 0.3333333333333333);
            let assign16880_e15210: f64 = (1.0 + assign16880_e15209);
            let assign16880_e15211: f64 = (assign16880_e15199 * assign16880_e15210);
            let assign16880_e15212: f64 = (0.5 * assign16880_e15211);
            let assign16880_e15213: f64 = (1.0 + assign16880_e15212);
            let assign16880_e15214: f64 = (assign16880_e15190 * assign16880_e15213);
            let assign16880_e15215: f64 = (1.0 + assign16880_e15214);
            let assign16880_e15216: f64 = (1e-100 / assign16880_e15215);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16880_e15216, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign16880_e15213) + (assign16880_e15190 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign16880_e15210) + (assign16880_e15199 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign16880_e15215 * assign16880_e15215))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign16880_e15213) + (assign16880_e15190 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign16880_e15210) + (assign16880_e15199 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign16880_e15215 * assign16880_e15215))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign16880_e15213) + (assign16880_e15190 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign16880_e15210) + (assign16880_e15199 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign16880_e15215 * assign16880_e15215))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign16880_e15213) + (assign16880_e15190 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign16880_e15210) + (assign16880_e15199 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign16880_e15215 * assign16880_e15215))), );
        }
    }
    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16890_e15230: f64 = (0.29214664 * locals.var_terfc);
            let assign16890_e15234: f64 = (locals.var_terfc * locals.var_terfc);
            let assign16890_e15235: f64 = (locals.var_berfc * assign16890_e15234);
            let assign16890_e15236: f64 = (assign16890_e15230 + assign16890_e15235);
            let assign16890_e15240: f64 = (locals.var_terfc * locals.var_terfc);
            let assign16890_e15242: f64 = (assign16890_e15240 * locals.var_terfc);
            let assign16890_e15243: f64 = (locals.var_cerfc * assign16890_e15242);
            let assign16890_e15244: f64 = (assign16890_e15236 + assign16890_e15243);
            let assign16890_e15246: f64 = (assign16890_e15244 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign16890_e15246, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign16890_e15240 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign16890_e15244 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign16890_e15240 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign16890_e15244 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign16890_e15240 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign16890_e15244 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign16890_e15240 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign16890_e15244 * locals.var_tmp_dn8)), );
        }
        let assign16900_e15251: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard278 = assign16900_e15251;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) && (locals.var_guard278 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign16920_e15268: f64 = (-230.25850929940458);
        let assign16920_e15269: f64 = if locals.var_mtat > assign16920_e15268 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign16920_e15269;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) && (locals.var_guard278 == 0.0)) && (locals.var_guard279 != 0.0)) {
            let assign16930_e15285: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16930_e15285, (assign16930_e15285 * locals.var_mtat_dn5), (assign16930_e15285 * locals.var_mtat_dn6), (assign16930_e15285 * locals.var_mtat_dn7), (assign16930_e15285 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) && (locals.var_guard278 == 0.0)) && (locals.var_guard279 == 0.0)) {
            let assign16940_e15306: f64 = (-230.25850929940458);
            let assign16940_e15308: f64 = (assign16940_e15306 - locals.var_mtat);
            let assign16940_e15312: f64 = (-230.25850929940458);
            let assign16940_e15314: f64 = (assign16940_e15312 - locals.var_mtat);
            let assign16940_e15317: f64 = (-230.25850929940458);
            let assign16940_e15319: f64 = (assign16940_e15317 - locals.var_mtat);
            let assign16940_e15321: f64 = (assign16940_e15319 * 0.3333333333333333);
            let assign16940_e15322: f64 = (1.0 + assign16940_e15321);
            let assign16940_e15323: f64 = (assign16940_e15314 * assign16940_e15322);
            let assign16940_e15324: f64 = (0.5 * assign16940_e15323);
            let assign16940_e15325: f64 = (1.0 + assign16940_e15324);
            let assign16940_e15326: f64 = (assign16940_e15308 * assign16940_e15325);
            let assign16940_e15327: f64 = (1.0 + assign16940_e15326);
            let assign16940_e15328: f64 = (1e-100 / assign16940_e15327);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign16940_e15328, (-((1e-100 * (((-locals.var_mtat_dn5) * assign16940_e15325) + (assign16940_e15308 * (0.5 * (((-locals.var_mtat_dn5) * assign16940_e15322) + (assign16940_e15314 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign16940_e15327 * assign16940_e15327))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign16940_e15325) + (assign16940_e15308 * (0.5 * (((-locals.var_mtat_dn6) * assign16940_e15322) + (assign16940_e15314 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign16940_e15327 * assign16940_e15327))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign16940_e15325) + (assign16940_e15308 * (0.5 * (((-locals.var_mtat_dn7) * assign16940_e15322) + (assign16940_e15314 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign16940_e15327 * assign16940_e15327))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign16940_e15325) + (assign16940_e15308 * (0.5 * (((-locals.var_mtat_dn8) * assign16940_e15322) + (assign16940_e15314 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign16940_e15327 * assign16940_e15327))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) && (locals.var_guard278 == 0.0)) {
            let assign16950_e15345: f64 = (2.0 * locals.var_tmp);
            let assign16950_e15347: f64 = (assign16950_e15345 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign16950_e15347, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16960_e15361: f64 = (1.772453850905516 * 0.5);
            let assign16960_e15364: f64 = (locals.var_atatsti * locals.var_erfctimesexpmtat);
            let assign16960_e15366: f64 = (assign16960_e15364 / locals.var_ktat);
            let assign16960_e15367: f64 = (assign16960_e15361 * assign16960_e15366);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign16960_e15367, (assign16960_e15361 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign16960_e15364 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign16960_e15361 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign16960_e15364 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign16960_e15361 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign16960_e15364 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign16960_e15361 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign16960_e15364 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard274 == 0.0)) {
            let assign16970_e15382: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign16970_e15384: f64 = (assign16970_e15382 * locals.var_wtat);
            let assign16970_e15385: f64 = (p.p846 * assign16970_e15384);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign16970_e15385, (p.p846 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign16970_e15382 * locals.var_wtat_dn5))), (p.p846 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign16970_e15382 * locals.var_wtat_dn6))), (p.p846 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign16970_e15382 * locals.var_wtat_dn7))), (p.p846 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign16970_e15382 * locals.var_wtat_dn8))), );
        }
        let assign16980_e15390: f64 = if p.p852 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign16980_e15390;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard280 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign17000_e15404: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard281 = assign17000_e15404;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard280 == 0.0)) && (locals.var_guard281 != 0.0)) {
            let assign17010_e15418: f64 = (p.p829 - locals.var_vbbt);
            let assign17010_e15420: f64 = (assign17010_e15418 * locals.var_vbirstiinv);
            let assign17010_e15421: f64 = (assign17010_e15420).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17010_e15421, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard280 == 0.0)) && (locals.var_guard281 == 0.0)) {
            let assign17020_e15438: f64 = (p.p829 - locals.var_vbbt);
            let assign17020_e15440: f64 = (assign17020_e15438 * locals.var_vbirstiinv);
            let assign17020_e15442: f64 = (assign17020_e15440).powf(p.p832);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17020_e15442, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard280 == 0.0)) {
            let assign17030_e15457: f64 = (p.p829 - locals.var_vbbt);
            let assign17030_e15459: f64 = (assign17030_e15457 * locals.var_wdepnulrinvsti);
            let assign17030_e15461: f64 = (assign17030_e15459 / locals.var_tmp);
            let assign17030_e15462: f64 = (locals.var_one_over_one_minus_psti * assign17030_e15461);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign17030_e15462, (locals.var_one_over_one_minus_psti * (-((assign17030_e15459 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign17030_e15459 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign17030_e15459 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign17030_e15459 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign17040_e15466: f64 = (-locals.var_fbbtsti);
        let assign17040_e15468: f64 = (assign17040_e15466 / locals.var_fmaxr);
        let assign17040_e15469: f64 = (assign17040_e15468).abs();
        let assign17040_e15471: f64 = if assign17040_e15469 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard282 = assign17040_e15471;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard280 == 0.0)) && (locals.var_guard282 != 0.0)) {
            let assign17050_e15484: f64 = (-locals.var_fbbtsti);
            let assign17050_e15486: f64 = (assign17050_e15484 / locals.var_fmaxr);
            let assign17050_e15487: f64 = (assign17050_e15486).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17050_e15487, (assign17050_e15487 * (-((assign17050_e15484 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign17050_e15487 * (-((assign17050_e15484 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign17050_e15487 * (-((assign17050_e15484 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign17050_e15487 * (-((assign17050_e15484 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign17060_e15491: f64 = (-locals.var_fbbtsti);
        let assign17060_e15493: f64 = (assign17060_e15491 / locals.var_fmaxr);
        let assign17060_e15495: f64 = if assign17060_e15493 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard283 = assign17060_e15495;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard280 == 0.0)) && (locals.var_guard282 == 0.0)) && (locals.var_guard283 != 0.0)) {
            let assign17070_e15513: f64 = (-230.25850929940458);
            let assign17070_e15515: f64 = (-locals.var_fbbtsti);
            let assign17070_e15517: f64 = (assign17070_e15515 / locals.var_fmaxr);
            let assign17070_e15518: f64 = (assign17070_e15513 - assign17070_e15517);
            let assign17070_e15522: f64 = (-230.25850929940458);
            let assign17070_e15524: f64 = (-locals.var_fbbtsti);
            let assign17070_e15526: f64 = (assign17070_e15524 / locals.var_fmaxr);
            let assign17070_e15527: f64 = (assign17070_e15522 - assign17070_e15526);
            let assign17070_e15530: f64 = (-230.25850929940458);
            let assign17070_e15532: f64 = (-locals.var_fbbtsti);
            let assign17070_e15534: f64 = (assign17070_e15532 / locals.var_fmaxr);
            let assign17070_e15535: f64 = (assign17070_e15530 - assign17070_e15534);
            let assign17070_e15537: f64 = (assign17070_e15535 * 0.3333333333333333);
            let assign17070_e15538: f64 = (1.0 + assign17070_e15537);
            let assign17070_e15539: f64 = (assign17070_e15527 * assign17070_e15538);
            let assign17070_e15540: f64 = (0.5 * assign17070_e15539);
            let assign17070_e15541: f64 = (1.0 + assign17070_e15540);
            let assign17070_e15542: f64 = (assign17070_e15518 * assign17070_e15541);
            let assign17070_e15543: f64 = (1.0 + assign17070_e15542);
            let assign17070_e15544: f64 = (1e-100 / assign17070_e15543);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17070_e15544, (-((1e-100 * (((-(-((assign17070_e15515 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign17070_e15541) + (assign17070_e15518 * (0.5 * (((-(-((assign17070_e15524 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign17070_e15538) + (assign17070_e15527 * ((-(-((assign17070_e15532 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign17070_e15543 * assign17070_e15543))), (-((1e-100 * (((-(-((assign17070_e15515 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign17070_e15541) + (assign17070_e15518 * (0.5 * (((-(-((assign17070_e15524 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign17070_e15538) + (assign17070_e15527 * ((-(-((assign17070_e15532 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign17070_e15543 * assign17070_e15543))), (-((1e-100 * (((-(-((assign17070_e15515 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign17070_e15541) + (assign17070_e15518 * (0.5 * (((-(-((assign17070_e15524 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign17070_e15538) + (assign17070_e15527 * ((-(-((assign17070_e15532 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign17070_e15543 * assign17070_e15543))), (-((1e-100 * (((-(-((assign17070_e15515 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign17070_e15541) + (assign17070_e15518 * (0.5 * (((-(-((assign17070_e15524 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign17070_e15538) + (assign17070_e15527 * ((-(-((assign17070_e15532 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign17070_e15543 * assign17070_e15543))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard280 == 0.0)) && (locals.var_guard282 == 0.0)) && (locals.var_guard283 == 0.0)) {
            let assign17080_e15565: f64 = (-locals.var_fbbtsti);
            let assign17080_e15567: f64 = (assign17080_e15565 / locals.var_fmaxr);
            let assign17080_e15569: f64 = (assign17080_e15567 - 230.25850929940458);
            let assign17080_e15573: f64 = (-locals.var_fbbtsti);
            let assign17080_e15575: f64 = (assign17080_e15573 / locals.var_fmaxr);
            let assign17080_e15577: f64 = (assign17080_e15575 - 230.25850929940458);
            let assign17080_e15580: f64 = (-locals.var_fbbtsti);
            let assign17080_e15582: f64 = (assign17080_e15580 / locals.var_fmaxr);
            let assign17080_e15584: f64 = (assign17080_e15582 - 230.25850929940458);
            let assign17080_e15586: f64 = (assign17080_e15584 * 0.3333333333333333);
            let assign17080_e15587: f64 = (1.0 + assign17080_e15586);
            let assign17080_e15588: f64 = (assign17080_e15577 * assign17080_e15587);
            let assign17080_e15589: f64 = (0.5 * assign17080_e15588);
            let assign17080_e15590: f64 = (1.0 + assign17080_e15589);
            let assign17080_e15591: f64 = (assign17080_e15569 * assign17080_e15590);
            let assign17080_e15592: f64 = (1.0 + assign17080_e15591);
            let assign17080_e15593: f64 = (1e100 * assign17080_e15592);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17080_e15593, (1e100 * (((-((assign17080_e15565 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17080_e15590) + (assign17080_e15569 * (0.5 * (((-((assign17080_e15573 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17080_e15587) + (assign17080_e15577 * ((-((assign17080_e15580 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign17080_e15565 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17080_e15590) + (assign17080_e15569 * (0.5 * (((-((assign17080_e15573 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17080_e15587) + (assign17080_e15577 * ((-((assign17080_e15580 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign17080_e15565 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17080_e15590) + (assign17080_e15569 * (0.5 * (((-((assign17080_e15573 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17080_e15587) + (assign17080_e15577 * ((-((assign17080_e15580 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign17080_e15565 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17080_e15590) + (assign17080_e15569 * (0.5 * (((-((assign17080_e15573 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17080_e15587) + (assign17080_e15577 * ((-((assign17080_e15580 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard280 == 0.0)) {
            let assign17090_e15608: f64 = (locals.var_v1 * locals.var_fmaxr);
            let assign17090_e15610: f64 = (assign17090_e15608 * locals.var_fmaxr);
            let assign17090_e15612: f64 = (assign17090_e15610 * locals.var_tmp);
            let assign17090_e15613: f64 = (p.p852 * assign17090_e15612);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign17090_e15613, (p.p852 * (((((locals.var_v1 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign17090_e15608 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign17090_e15610 * locals.var_tmp_dn5))), (p.p852 * (((((locals.var_v1 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign17090_e15608 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign17090_e15610 * locals.var_tmp_dn6))), (p.p852 * (((((locals.var_v1 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign17090_e15608 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign17090_e15610 * locals.var_tmp_dn7))), (p.p852 * (((((locals.var_v1 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign17090_e15608 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign17090_e15610 * locals.var_tmp_dn8))), );
        }
        let assign17100_e15618: f64 = if p.p861 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign17100_e15618;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard284 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign17120_e15632: f64 = (-locals.var_alphaav);
        let assign17120_e15634: f64 = (assign17120_e15632 * p.p861);
        let assign17120_e15635: f64 = if locals.var_vav > assign17120_e15634 { 1.0 } else { 0.0 };
        locals.var_guard285 = assign17120_e15635;
        let assign17130_e15638: f64 = if p.p864 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard286 = assign17130_e15638;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard284 == 0.0)) && (locals.var_guard285 != 0.0)) && (locals.var_guard286 != 0.0)) {
            let assign17140_e15654: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign17140_e15657: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign17140_e15658: f64 = (assign17140_e15654 * assign17140_e15657);
            let assign17140_e15661: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign17140_e15662: f64 = (assign17140_e15658 * assign17140_e15661);
            let assign17140_e15665: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign17140_e15666: f64 = (assign17140_e15662 * assign17140_e15665);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17140_e15666, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard284 == 0.0)) && (locals.var_guard285 != 0.0)) && (locals.var_guard286 == 0.0)) {
            let assign17150_e15685: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign17150_e15686: f64 = (assign17150_e15685).abs();
            let assign17150_e15688: f64 = (assign17150_e15686).powf(p.p864);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17150_e15688, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard284 == 0.0)) && (locals.var_guard285 != 0.0)) {
            let assign17160_e15705: f64 = (1.0 - locals.var_tmp);
            let assign17160_e15706: f64 = (1.0 / assign17160_e15705);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign17160_e15706, (-((-locals.var_tmp_dn5) / (assign17160_e15705 * assign17160_e15705))), (-((-locals.var_tmp_dn6) / (assign17160_e15705 * assign17160_e15705))), (-((-locals.var_tmp_dn7) / (assign17160_e15705 * assign17160_e15705))), (-((-locals.var_tmp_dn8) / (assign17160_e15705 * assign17160_e15705))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard284 == 0.0)) && (locals.var_guard285 == 0.0)) {
            let assign17170_e15725: f64 = (locals.var_alphaav * p.p861);
            let assign17170_e15726: f64 = (locals.var_vav + assign17170_e15725);
            let assign17170_e15728: f64 = (assign17170_e15726 * locals.var_slopesti);
            let assign17170_e15729: f64 = (locals.var_fstopsti + assign17170_e15728);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign17170_e15729, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard270 == 0.0)) {
            let assign17180_e15741: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign17180_e15743: f64 = (assign17180_e15741 + locals.var_itat);
            let assign17180_e15745: f64 = (assign17180_e15743 + locals.var_ibbt);
            let assign17180_e15746: f64 = (p.p29 * assign17180_e15745);
            let assign17180_e15748: f64 = (assign17180_e15746 * locals.var_fbreakdown);
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (assign17180_e15748, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign17180_e15746 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign17180_e15746 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign17180_e15746 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign17180_e15746 * locals.var_fbreakdown_dn8)), );
        }
        let assign17190_e15753: f64 = if locals.var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard287 = assign17190_e15753;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 != 0.0)) {
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) {
            let assign17210_e15770: f64 = (locals.var_idsatgat * locals.var_idmult);
            locals.var_id__blk219 = assign17210_e15770;
        }
        let assign17220_e15779: f64 = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard288 = assign17220_e15779;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) {
            let assign17240_e15802: f64 = (locals.var_vbigat - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign17240_e15802;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) {
            let assign17250_e15818: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign17250_e15819: f64 = (1.0 - assign17250_e15818);
            let assign17250_e15820: f64 = (assign17250_e15819).sqrt();
            let assign17250_e15821: f64 = (1.0 - assign17250_e15820);
            locals.var_wsrhstep = assign17250_e15821;
        }
        let assign17260_e15826: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard289 = assign17260_e15826;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard289 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard289 == 0.0)) {
            let assign17280_e15855: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign17280_e15857: f64 = (locals.var_wsrhstep).ln();
            let assign17280_e15858: f64 = (assign17280_e15855 * assign17280_e15857);
            let assign17280_e15861: f64 = (1.0 - locals.var_wsrhstep);
            let assign17280_e15862: f64 = (assign17280_e15858 / assign17280_e15861);
            let assign17280_e15864: f64 = (assign17280_e15862 + locals.var_wsrhstep);
            let assign17280_e15868: f64 = (2.0 * p.p833);
            let assign17280_e15869: f64 = (1.0 - assign17280_e15868);
            let assign17280_e15870: f64 = (assign17280_e15864 * assign17280_e15869);
            locals.var_dwsrh = assign17280_e15870;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) {
            let assign17290_e15884: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign17290_e15884;
        }
        let assign17300_e15889: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard290 = assign17300_e15889;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard290 != 0.0)) {
            let assign17310_e15903: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv);
            let assign17310_e15904: f64 = (assign17310_e15903).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17310_e15904, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard290 == 0.0)) {
            let assign17320_e15921: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv);
            let assign17320_e15923: f64 = (assign17320_e15921).powf(p.p833);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17320_e15923, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) {
            let assign17330_e15937: f64 = (locals.var_wdepnulrgat * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign17330_e15937, (locals.var_wdepnulrgat * locals.var_tmp_dn5), (locals.var_wdepnulrgat * locals.var_tmp_dn6), (locals.var_wdepnulrgat * locals.var_tmp_dn7), (locals.var_wdepnulrgat * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) {
            let assign17340_e15952: f64 = (locals.var_zinv - 1.0);
            let assign17340_e15954: f64 = (assign17340_e15952 * locals.var_wdep);
            let assign17340_e15955: f64 = (locals.var_ftdgat * assign17340_e15954);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign17340_e15955, (locals.var_ftdgat * (assign17340_e15952 * locals.var_wdep_dn5)), (locals.var_ftdgat * (assign17340_e15952 * locals.var_wdep_dn6)), (locals.var_ftdgat * (assign17340_e15952 * locals.var_wdep_dn7)), (locals.var_ftdgat * (assign17340_e15952 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) {
            let assign17350_e15970: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign17350_e15971: f64 = (p.p842 * assign17350_e15970);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign17350_e15971, (p.p842 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign17360_e15976: f64 = if p.p847 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard291 = assign17360_e15976;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17380_e16000: f64 = (locals.var_wdep * locals.var_one_minus_pgat);
            let assign17380_e16002: f64 = (assign17380_e16000 / locals.var_vbi_minus_vjsrh);
            let assign17380_e16003: f64 = (locals.var_btatpartgat * assign17380_e16002);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign17380_e16003, (locals.var_btatpartgat * ((locals.var_wdep_dn5 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn6 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn7 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn8 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17390_e16017: f64 = (0.666666666666667 * locals.var_atatgat);
            let assign17390_e16019: f64 = (assign17390_e16017 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign17390_e16019, (-((assign17390_e16017 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign17390_e16017 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign17390_e16017 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign17390_e16017 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17400_e16033: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign17400_e16033, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17410_e16047: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign17410_e16050: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign17410_e16052: f64 = (assign17410_e16050 + 1.0);
            let assign17410_e16053: f64 = (assign17410_e16047 / assign17410_e16052);
            let assign17410_e16054: f64 = (assign17410_e16053).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign17410_e16054, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign17410_e16052) - (assign17410_e16047 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign17410_e16052 * assign17410_e16052)) / (2.0 * assign17410_e16054)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign17410_e16052) - (assign17410_e16047 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign17410_e16052 * assign17410_e16052)) / (2.0 * assign17410_e16054)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign17410_e16052) - (assign17410_e16047 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign17410_e16052 * assign17410_e16052)) / (2.0 * assign17410_e16054)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign17410_e16052) - (assign17410_e16047 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign17410_e16052 * assign17410_e16052)) / (2.0 * assign17410_e16054)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17420_e16067: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign17420_e16067, (locals.var_umax_dn5 / (2.0 * assign17420_e16067)), (locals.var_umax_dn6 / (2.0 * assign17420_e16067)), (locals.var_umax_dn7 / (2.0 * assign17420_e16067)), (locals.var_umax_dn8 / (2.0 * assign17420_e16067)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17430_e16081: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign17430_e16081, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign17440_e16085: f64 = (-p.p833);
        let assign17440_e16087: f64 = (assign17440_e16085 * locals.var_one_over_one_minus_pgat);
        let assign17440_e16089: f64 = (-1.0);
        let assign17440_e16090: f64 = if assign17440_e16087 == assign17440_e16089 { 1.0 } else { 0.0 };
        locals.var_guard292 = assign17440_e16090;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) && (locals.var_guard292 != 0.0)) {
            let assign17450_e16106: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign17450_e16107: f64 = (1.0 + assign17450_e16106);
            let assign17450_e16108: f64 = (1.0 / assign17450_e16107);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign17450_e16108, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign17450_e16107 * assign17450_e16107))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign17450_e16107 * assign17450_e16107))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign17450_e16107 * assign17450_e16107))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign17450_e16107 * assign17450_e16107))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) && (locals.var_guard292 == 0.0)) {
            let assign17460_e16126: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign17460_e16127: f64 = (1.0 + assign17460_e16126);
            let assign17460_e16129: f64 = (-p.p833);
            let assign17460_e16131: f64 = (assign17460_e16129 * locals.var_one_over_one_minus_pgat);
            let assign17460_e16132: f64 = (assign17460_e16127).powf(assign17460_e16131);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign17460_e16132, if 0.0 == 0.0 && ((assign17460_e16131) as f64).is_finite() && ((assign17460_e16131) as f64).fract() == 0.0 { if assign17460_e16131 == 0.0 { 0.0 } else { (assign17460_e16131 * ((assign17460_e16127).powf(assign17460_e16131 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign17460_e16132 * (assign17460_e16131 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign17460_e16127))) }, if 0.0 == 0.0 && ((assign17460_e16131) as f64).is_finite() && ((assign17460_e16131) as f64).fract() == 0.0 { if assign17460_e16131 == 0.0 { 0.0 } else { (assign17460_e16131 * ((assign17460_e16127).powf(assign17460_e16131 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign17460_e16132 * (assign17460_e16131 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign17460_e16127))) }, if 0.0 == 0.0 && ((assign17460_e16131) as f64).is_finite() && ((assign17460_e16131) as f64).fract() == 0.0 { if assign17460_e16131 == 0.0 { 0.0 } else { (assign17460_e16131 * ((assign17460_e16127).powf(assign17460_e16131 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign17460_e16132 * (assign17460_e16131 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign17460_e16127))) }, if 0.0 == 0.0 && ((assign17460_e16131) as f64).is_finite() && ((assign17460_e16131) as f64).fract() == 0.0 { if assign17460_e16131 == 0.0 { 0.0 } else { (assign17460_e16131 * ((assign17460_e16127).powf(assign17460_e16131 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign17460_e16132 * (assign17460_e16131 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign17460_e16127))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17470_e16146: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign17470_e16149: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign17470_e16150: f64 = (assign17470_e16146 / assign17470_e16149);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign17470_e16150, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign17470_e16149) - (assign17470_e16146 * locals.var_wgamma_dn5)) / (assign17470_e16149 * assign17470_e16149)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign17470_e16149) - (assign17470_e16146 * locals.var_wgamma_dn6)) / (assign17470_e16149 * assign17470_e16149)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign17470_e16149) - (assign17470_e16146 * locals.var_wgamma_dn7)) / (assign17470_e16149 * assign17470_e16149)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign17470_e16149) - (assign17470_e16146 * locals.var_wgamma_dn8)) / (assign17470_e16149 * assign17470_e16149)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17480_e16165: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign17480_e16166: f64 = (0.375 * assign17480_e16165);
            let assign17480_e16167: f64 = (assign17480_e16166).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign17480_e16167, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign17480_e16167)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign17480_e16167)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign17480_e16167)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign17480_e16167)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17490_e16182: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign17490_e16183: f64 = (2.0 * assign17490_e16182);
            let assign17490_e16185: f64 = (assign17490_e16183 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign17490_e16185, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17500_e16199: f64 = (locals.var_atatgat * locals.var_twoatatoverthreebtat);
            let assign17500_e16201: f64 = (assign17500_e16199 * locals.var_sqrtumax);
            let assign17500_e16204: f64 = (locals.var_atatgat * locals.var_umax);
            let assign17500_e16205: f64 = (assign17500_e16201 - assign17500_e16204);
            let assign17500_e16209: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign17500_e16210: f64 = (0.5 * assign17500_e16209);
            let assign17500_e16211: f64 = (assign17500_e16205 + assign17500_e16210);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign17500_e16211, (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign17500_e16199 * locals.var_sqrtumax_dn5)) - (locals.var_atatgat * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign17500_e16199 * locals.var_sqrtumax_dn6)) - (locals.var_atatgat * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign17500_e16199 * locals.var_sqrtumax_dn7)) - (locals.var_atatgat * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign17500_e16199 * locals.var_sqrtumax_dn8)) - (locals.var_atatgat * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17510_e16225: f64 = (locals.var_ltat - 1.0);
            let assign17510_e16227: f64 = (assign17510_e16225 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign17510_e16227, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign17510_e16225 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign17510_e16225 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign17510_e16225 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign17510_e16225 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17520_e16241: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign17520_e16241, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign17530_e16246: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard293 = assign17530_e16246;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) && (locals.var_guard293 != 0.0)) {
            let assign17540_e16262: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign17540_e16263: f64 = (1.0 + assign17540_e16262);
            let assign17540_e16264: f64 = (1.0 / assign17540_e16263);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign17540_e16264, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign17540_e16263 * assign17540_e16263))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign17540_e16263 * assign17540_e16263))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign17540_e16263 * assign17540_e16263))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign17540_e16263 * assign17540_e16263))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) && (locals.var_guard293 == 0.0)) {
            let assign17550_e16283: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign17550_e16284: f64 = (1.0 - assign17550_e16283);
            let assign17550_e16285: f64 = (1.0 / assign17550_e16284);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign17550_e16285, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign17550_e16284 * assign17550_e16284))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign17550_e16284 * assign17550_e16284))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign17550_e16284 * assign17550_e16284))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign17550_e16284 * assign17550_e16284))), );
        }
        let assign17560_e16289: f64 = (-locals.var_ysq);
        let assign17560_e16291: f64 = (assign17560_e16289 + locals.var_mtat);
        let assign17560_e16293: f64 = (-230.25850929940458);
        let assign17560_e16294: f64 = if assign17560_e16291 > assign17560_e16293 { 1.0 } else { 0.0 };
        locals.var_guard294 = assign17560_e16294;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) && (locals.var_guard294 != 0.0)) {
            let assign17570_e16307: f64 = (-locals.var_ysq);
            let assign17570_e16309: f64 = (assign17570_e16307 + locals.var_mtat);
            let assign17570_e16310: f64 = (assign17570_e16309).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17570_e16310, (assign17570_e16310 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign17570_e16310 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign17570_e16310 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign17570_e16310 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) && (locals.var_guard294 == 0.0)) {
            let assign17580_e16328: f64 = (-230.25850929940458);
            let assign17580_e16330: f64 = (-locals.var_ysq);
            let assign17580_e16332: f64 = (assign17580_e16330 + locals.var_mtat);
            let assign17580_e16333: f64 = (assign17580_e16328 - assign17580_e16332);
            let assign17580_e16337: f64 = (-230.25850929940458);
            let assign17580_e16339: f64 = (-locals.var_ysq);
            let assign17580_e16341: f64 = (assign17580_e16339 + locals.var_mtat);
            let assign17580_e16342: f64 = (assign17580_e16337 - assign17580_e16341);
            let assign17580_e16345: f64 = (-230.25850929940458);
            let assign17580_e16347: f64 = (-locals.var_ysq);
            let assign17580_e16349: f64 = (assign17580_e16347 + locals.var_mtat);
            let assign17580_e16350: f64 = (assign17580_e16345 - assign17580_e16349);
            let assign17580_e16352: f64 = (assign17580_e16350 * 0.3333333333333333);
            let assign17580_e16353: f64 = (1.0 + assign17580_e16352);
            let assign17580_e16354: f64 = (assign17580_e16342 * assign17580_e16353);
            let assign17580_e16355: f64 = (0.5 * assign17580_e16354);
            let assign17580_e16356: f64 = (1.0 + assign17580_e16355);
            let assign17580_e16357: f64 = (assign17580_e16333 * assign17580_e16356);
            let assign17580_e16358: f64 = (1.0 + assign17580_e16357);
            let assign17580_e16359: f64 = (1e-100 / assign17580_e16358);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17580_e16359, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign17580_e16356) + (assign17580_e16333 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign17580_e16353) + (assign17580_e16342 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign17580_e16358 * assign17580_e16358))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign17580_e16356) + (assign17580_e16333 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign17580_e16353) + (assign17580_e16342 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign17580_e16358 * assign17580_e16358))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign17580_e16356) + (assign17580_e16333 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign17580_e16353) + (assign17580_e16342 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign17580_e16358 * assign17580_e16358))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign17580_e16356) + (assign17580_e16333 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign17580_e16353) + (assign17580_e16342 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign17580_e16358 * assign17580_e16358))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17590_e16373: f64 = (0.29214664 * locals.var_terfc);
            let assign17590_e16377: f64 = (locals.var_terfc * locals.var_terfc);
            let assign17590_e16378: f64 = (locals.var_berfc * assign17590_e16377);
            let assign17590_e16379: f64 = (assign17590_e16373 + assign17590_e16378);
            let assign17590_e16383: f64 = (locals.var_terfc * locals.var_terfc);
            let assign17590_e16385: f64 = (assign17590_e16383 * locals.var_terfc);
            let assign17590_e16386: f64 = (locals.var_cerfc * assign17590_e16385);
            let assign17590_e16387: f64 = (assign17590_e16379 + assign17590_e16386);
            let assign17590_e16389: f64 = (assign17590_e16387 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign17590_e16389, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign17590_e16383 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign17590_e16387 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign17590_e16383 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign17590_e16387 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign17590_e16383 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign17590_e16387 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign17590_e16383 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign17590_e16387 * locals.var_tmp_dn8)), );
        }
        let assign17600_e16394: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard295 = assign17600_e16394;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) && (locals.var_guard295 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign17620_e16411: f64 = (-230.25850929940458);
        let assign17620_e16412: f64 = if locals.var_mtat > assign17620_e16411 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign17620_e16412;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) && (locals.var_guard295 == 0.0)) && (locals.var_guard296 != 0.0)) {
            let assign17630_e16428: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17630_e16428, (assign17630_e16428 * locals.var_mtat_dn5), (assign17630_e16428 * locals.var_mtat_dn6), (assign17630_e16428 * locals.var_mtat_dn7), (assign17630_e16428 * locals.var_mtat_dn8), );
        }
    }
    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) && (locals.var_guard295 == 0.0)) && (locals.var_guard296 == 0.0)) {
            let assign17640_e16449: f64 = (-230.25850929940458);
            let assign17640_e16451: f64 = (assign17640_e16449 - locals.var_mtat);
            let assign17640_e16455: f64 = (-230.25850929940458);
            let assign17640_e16457: f64 = (assign17640_e16455 - locals.var_mtat);
            let assign17640_e16460: f64 = (-230.25850929940458);
            let assign17640_e16462: f64 = (assign17640_e16460 - locals.var_mtat);
            let assign17640_e16464: f64 = (assign17640_e16462 * 0.3333333333333333);
            let assign17640_e16465: f64 = (1.0 + assign17640_e16464);
            let assign17640_e16466: f64 = (assign17640_e16457 * assign17640_e16465);
            let assign17640_e16467: f64 = (0.5 * assign17640_e16466);
            let assign17640_e16468: f64 = (1.0 + assign17640_e16467);
            let assign17640_e16469: f64 = (assign17640_e16451 * assign17640_e16468);
            let assign17640_e16470: f64 = (1.0 + assign17640_e16469);
            let assign17640_e16471: f64 = (1e-100 / assign17640_e16470);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17640_e16471, (-((1e-100 * (((-locals.var_mtat_dn5) * assign17640_e16468) + (assign17640_e16451 * (0.5 * (((-locals.var_mtat_dn5) * assign17640_e16465) + (assign17640_e16457 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign17640_e16470 * assign17640_e16470))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign17640_e16468) + (assign17640_e16451 * (0.5 * (((-locals.var_mtat_dn6) * assign17640_e16465) + (assign17640_e16457 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign17640_e16470 * assign17640_e16470))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign17640_e16468) + (assign17640_e16451 * (0.5 * (((-locals.var_mtat_dn7) * assign17640_e16465) + (assign17640_e16457 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign17640_e16470 * assign17640_e16470))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign17640_e16468) + (assign17640_e16451 * (0.5 * (((-locals.var_mtat_dn8) * assign17640_e16465) + (assign17640_e16457 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign17640_e16470 * assign17640_e16470))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) && (locals.var_guard295 == 0.0)) {
            let assign17650_e16488: f64 = (2.0 * locals.var_tmp);
            let assign17650_e16490: f64 = (assign17650_e16488 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign17650_e16490, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17660_e16504: f64 = (1.772453850905516 * 0.5);
            let assign17660_e16507: f64 = (locals.var_atatgat * locals.var_erfctimesexpmtat);
            let assign17660_e16509: f64 = (assign17660_e16507 / locals.var_ktat);
            let assign17660_e16510: f64 = (assign17660_e16504 * assign17660_e16509);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign17660_e16510, (assign17660_e16504 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign17660_e16507 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign17660_e16504 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign17660_e16507 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign17660_e16504 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign17660_e16507 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign17660_e16504 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign17660_e16507 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign17670_e16525: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign17670_e16527: f64 = (assign17670_e16525 * locals.var_wtat);
            let assign17670_e16528: f64 = (p.p847 * assign17670_e16527);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign17670_e16528, (p.p847 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign17670_e16525 * locals.var_wtat_dn5))), (p.p847 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign17670_e16525 * locals.var_wtat_dn6))), (p.p847 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign17670_e16525 * locals.var_wtat_dn7))), (p.p847 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign17670_e16525 * locals.var_wtat_dn8))), );
        }
        let assign17680_e16533: f64 = if p.p853 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard297 = assign17680_e16533;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard297 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign17700_e16547: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard298 = assign17700_e16547;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard297 == 0.0)) && (locals.var_guard298 != 0.0)) {
            let assign17710_e16561: f64 = (p.p830 - locals.var_vbbt);
            let assign17710_e16563: f64 = (assign17710_e16561 * locals.var_vbirgatinv);
            let assign17710_e16564: f64 = (assign17710_e16563).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17710_e16564, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard297 == 0.0)) && (locals.var_guard298 == 0.0)) {
            let assign17720_e16581: f64 = (p.p830 - locals.var_vbbt);
            let assign17720_e16583: f64 = (assign17720_e16581 * locals.var_vbirgatinv);
            let assign17720_e16585: f64 = (assign17720_e16583).powf(p.p833);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17720_e16585, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard297 == 0.0)) {
            let assign17730_e16600: f64 = (p.p830 - locals.var_vbbt);
            let assign17730_e16602: f64 = (assign17730_e16600 * locals.var_wdepnulrinvgat);
            let assign17730_e16604: f64 = (assign17730_e16602 / locals.var_tmp);
            let assign17730_e16605: f64 = (locals.var_one_over_one_minus_pgat * assign17730_e16604);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign17730_e16605, (locals.var_one_over_one_minus_pgat * (-((assign17730_e16602 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign17730_e16602 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign17730_e16602 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign17730_e16602 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign17740_e16609: f64 = (-locals.var_fbbtgat);
        let assign17740_e16611: f64 = (assign17740_e16609 / locals.var_fmaxr);
        let assign17740_e16612: f64 = (assign17740_e16611).abs();
        let assign17740_e16614: f64 = if assign17740_e16612 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard299 = assign17740_e16614;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard297 == 0.0)) && (locals.var_guard299 != 0.0)) {
            let assign17750_e16627: f64 = (-locals.var_fbbtgat);
            let assign17750_e16629: f64 = (assign17750_e16627 / locals.var_fmaxr);
            let assign17750_e16630: f64 = (assign17750_e16629).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17750_e16630, (assign17750_e16630 * ((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign17750_e16627 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign17750_e16630 * ((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign17750_e16627 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign17750_e16630 * ((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign17750_e16627 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign17750_e16630 * ((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign17750_e16627 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))), );
        }
        let assign17760_e16634: f64 = (-locals.var_fbbtgat);
        let assign17760_e16636: f64 = (assign17760_e16634 / locals.var_fmaxr);
        let assign17760_e16638: f64 = if assign17760_e16636 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard300 = assign17760_e16638;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard297 == 0.0)) && (locals.var_guard299 == 0.0)) && (locals.var_guard300 != 0.0)) {
            let assign17770_e16656: f64 = (-230.25850929940458);
            let assign17770_e16658: f64 = (-locals.var_fbbtgat);
            let assign17770_e16660: f64 = (assign17770_e16658 / locals.var_fmaxr);
            let assign17770_e16661: f64 = (assign17770_e16656 - assign17770_e16660);
            let assign17770_e16665: f64 = (-230.25850929940458);
            let assign17770_e16667: f64 = (-locals.var_fbbtgat);
            let assign17770_e16669: f64 = (assign17770_e16667 / locals.var_fmaxr);
            let assign17770_e16670: f64 = (assign17770_e16665 - assign17770_e16669);
            let assign17770_e16673: f64 = (-230.25850929940458);
            let assign17770_e16675: f64 = (-locals.var_fbbtgat);
            let assign17770_e16677: f64 = (assign17770_e16675 / locals.var_fmaxr);
            let assign17770_e16678: f64 = (assign17770_e16673 - assign17770_e16677);
            let assign17770_e16680: f64 = (assign17770_e16678 * 0.3333333333333333);
            let assign17770_e16681: f64 = (1.0 + assign17770_e16680);
            let assign17770_e16682: f64 = (assign17770_e16670 * assign17770_e16681);
            let assign17770_e16683: f64 = (0.5 * assign17770_e16682);
            let assign17770_e16684: f64 = (1.0 + assign17770_e16683);
            let assign17770_e16685: f64 = (assign17770_e16661 * assign17770_e16684);
            let assign17770_e16686: f64 = (1.0 + assign17770_e16685);
            let assign17770_e16687: f64 = (1e-100 / assign17770_e16686);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17770_e16687, (-((1e-100 * (((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign17770_e16658 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17770_e16684) + (assign17770_e16661 * (0.5 * (((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign17770_e16667 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17770_e16681) + (assign17770_e16670 * ((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign17770_e16675 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign17770_e16686 * assign17770_e16686))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign17770_e16658 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17770_e16684) + (assign17770_e16661 * (0.5 * (((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign17770_e16667 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17770_e16681) + (assign17770_e16670 * ((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign17770_e16675 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign17770_e16686 * assign17770_e16686))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign17770_e16658 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17770_e16684) + (assign17770_e16661 * (0.5 * (((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign17770_e16667 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17770_e16681) + (assign17770_e16670 * ((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign17770_e16675 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign17770_e16686 * assign17770_e16686))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign17770_e16658 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17770_e16684) + (assign17770_e16661 * (0.5 * (((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign17770_e16667 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign17770_e16681) + (assign17770_e16670 * ((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign17770_e16675 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign17770_e16686 * assign17770_e16686))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard297 == 0.0)) && (locals.var_guard299 == 0.0)) && (locals.var_guard300 == 0.0)) {
            let assign17780_e16708: f64 = (-locals.var_fbbtgat);
            let assign17780_e16710: f64 = (assign17780_e16708 / locals.var_fmaxr);
            let assign17780_e16712: f64 = (assign17780_e16710 - 230.25850929940458);
            let assign17780_e16716: f64 = (-locals.var_fbbtgat);
            let assign17780_e16718: f64 = (assign17780_e16716 / locals.var_fmaxr);
            let assign17780_e16720: f64 = (assign17780_e16718 - 230.25850929940458);
            let assign17780_e16723: f64 = (-locals.var_fbbtgat);
            let assign17780_e16725: f64 = (assign17780_e16723 / locals.var_fmaxr);
            let assign17780_e16727: f64 = (assign17780_e16725 - 230.25850929940458);
            let assign17780_e16729: f64 = (assign17780_e16727 * 0.3333333333333333);
            let assign17780_e16730: f64 = (1.0 + assign17780_e16729);
            let assign17780_e16731: f64 = (assign17780_e16720 * assign17780_e16730);
            let assign17780_e16732: f64 = (0.5 * assign17780_e16731);
            let assign17780_e16733: f64 = (1.0 + assign17780_e16732);
            let assign17780_e16734: f64 = (assign17780_e16712 * assign17780_e16733);
            let assign17780_e16735: f64 = (1.0 + assign17780_e16734);
            let assign17780_e16736: f64 = (1e100 * assign17780_e16735);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17780_e16736, (1e100 * ((((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign17780_e16708 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign17780_e16733) + (assign17780_e16712 * (0.5 * ((((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign17780_e16716 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign17780_e16730) + (assign17780_e16720 * (((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign17780_e16723 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign17780_e16708 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign17780_e16733) + (assign17780_e16712 * (0.5 * ((((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign17780_e16716 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign17780_e16730) + (assign17780_e16720 * (((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign17780_e16723 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign17780_e16708 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign17780_e16733) + (assign17780_e16712 * (0.5 * ((((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign17780_e16716 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign17780_e16730) + (assign17780_e16720 * (((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign17780_e16723 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign17780_e16708 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign17780_e16733) + (assign17780_e16712 * (0.5 * ((((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign17780_e16716 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign17780_e16730) + (assign17780_e16720 * (((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign17780_e16723 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard297 == 0.0)) {
            let assign17790_e16751: f64 = (locals.var_v1 * locals.var_fmaxr);
            let assign17790_e16753: f64 = (assign17790_e16751 * locals.var_fmaxr);
            let assign17790_e16755: f64 = (assign17790_e16753 * locals.var_tmp);
            let assign17790_e16756: f64 = (p.p853 * assign17790_e16755);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign17790_e16756, (p.p853 * (((((locals.var_v1 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign17790_e16751 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign17790_e16753 * locals.var_tmp_dn5))), (p.p853 * (((((locals.var_v1 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign17790_e16751 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign17790_e16753 * locals.var_tmp_dn6))), (p.p853 * (((((locals.var_v1 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign17790_e16751 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign17790_e16753 * locals.var_tmp_dn7))), (p.p853 * (((((locals.var_v1 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign17790_e16751 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign17790_e16753 * locals.var_tmp_dn8))), );
        }
        let assign17800_e16761: f64 = if p.p862 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard301 = assign17800_e16761;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard301 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign17820_e16775: f64 = (-locals.var_alphaav);
        let assign17820_e16777: f64 = (assign17820_e16775 * p.p862);
        let assign17820_e16778: f64 = if locals.var_vav > assign17820_e16777 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign17820_e16778;
        let assign17830_e16781: f64 = if p.p865 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard303 = assign17830_e16781;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard301 == 0.0)) && (locals.var_guard302 != 0.0)) && (locals.var_guard303 != 0.0)) {
            let assign17840_e16797: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign17840_e16800: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign17840_e16801: f64 = (assign17840_e16797 * assign17840_e16800);
            let assign17840_e16804: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign17840_e16805: f64 = (assign17840_e16801 * assign17840_e16804);
            let assign17840_e16808: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign17840_e16809: f64 = (assign17840_e16805 * assign17840_e16808);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17840_e16809, (((((((locals.var_vav * locals.var_vbrinvgat_dn5) * assign17840_e16800) + (assign17840_e16797 * (locals.var_vav * locals.var_vbrinvgat_dn5))) * assign17840_e16804) + (assign17840_e16801 * (locals.var_vav * locals.var_vbrinvgat_dn5))) * assign17840_e16808) + (assign17840_e16805 * (locals.var_vav * locals.var_vbrinvgat_dn5))), (((((((locals.var_vav * locals.var_vbrinvgat_dn6) * assign17840_e16800) + (assign17840_e16797 * (locals.var_vav * locals.var_vbrinvgat_dn6))) * assign17840_e16804) + (assign17840_e16801 * (locals.var_vav * locals.var_vbrinvgat_dn6))) * assign17840_e16808) + (assign17840_e16805 * (locals.var_vav * locals.var_vbrinvgat_dn6))), (((((((locals.var_vav * locals.var_vbrinvgat_dn7) * assign17840_e16800) + (assign17840_e16797 * (locals.var_vav * locals.var_vbrinvgat_dn7))) * assign17840_e16804) + (assign17840_e16801 * (locals.var_vav * locals.var_vbrinvgat_dn7))) * assign17840_e16808) + (assign17840_e16805 * (locals.var_vav * locals.var_vbrinvgat_dn7))), (((((((locals.var_vav * locals.var_vbrinvgat_dn8) * assign17840_e16800) + (assign17840_e16797 * (locals.var_vav * locals.var_vbrinvgat_dn8))) * assign17840_e16804) + (assign17840_e16801 * (locals.var_vav * locals.var_vbrinvgat_dn8))) * assign17840_e16808) + (assign17840_e16805 * (locals.var_vav * locals.var_vbrinvgat_dn8))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard301 == 0.0)) && (locals.var_guard302 != 0.0)) && (locals.var_guard303 == 0.0)) {
            let assign17850_e16828: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign17850_e16829: f64 = (assign17850_e16828).abs();
            let assign17850_e16831: f64 = (assign17850_e16829).powf(p.p865);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign17850_e16831, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign17850_e16829).powf(p.p865 - 1.0) * if assign17850_e16828 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn5)) })) } } else { (assign17850_e16831 * (p.p865 * (if assign17850_e16828 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn5)) } / assign17850_e16829))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign17850_e16829).powf(p.p865 - 1.0) * if assign17850_e16828 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn6)) })) } } else { (assign17850_e16831 * (p.p865 * (if assign17850_e16828 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn6)) } / assign17850_e16829))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign17850_e16829).powf(p.p865 - 1.0) * if assign17850_e16828 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn7)) })) } } else { (assign17850_e16831 * (p.p865 * (if assign17850_e16828 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn7)) } / assign17850_e16829))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign17850_e16829).powf(p.p865 - 1.0) * if assign17850_e16828 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn8)) })) } } else { (assign17850_e16831 * (p.p865 * (if assign17850_e16828 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn8)) } / assign17850_e16829))) }, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard301 == 0.0)) && (locals.var_guard302 != 0.0)) {
            let assign17860_e16848: f64 = (1.0 - locals.var_tmp);
            let assign17860_e16849: f64 = (1.0 / assign17860_e16848);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign17860_e16849, (-((-locals.var_tmp_dn5) / (assign17860_e16848 * assign17860_e16848))), (-((-locals.var_tmp_dn6) / (assign17860_e16848 * assign17860_e16848))), (-((-locals.var_tmp_dn7) / (assign17860_e16848 * assign17860_e16848))), (-((-locals.var_tmp_dn8) / (assign17860_e16848 * assign17860_e16848))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) && (locals.var_guard301 == 0.0)) && (locals.var_guard302 == 0.0)) {
            let assign17870_e16868: f64 = (locals.var_alphaav * p.p862);
            let assign17870_e16869: f64 = (locals.var_vav + assign17870_e16868);
            let assign17870_e16871: f64 = (assign17870_e16869 * locals.var_slopegat);
            let assign17870_e16872: f64 = (locals.var_fstopgat + assign17870_e16871);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign17870_e16872, (assign17870_e16869 * locals.var_slopegat_dn5), (assign17870_e16869 * locals.var_slopegat_dn6), (assign17870_e16869 * locals.var_slopegat_dn7), (assign17870_e16869 * locals.var_slopegat_dn8), );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard287 == 0.0)) {
            let assign17880_e16884: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign17880_e16886: f64 = (assign17880_e16884 + locals.var_itat);
            let assign17880_e16888: f64 = (assign17880_e16886 + locals.var_ibbt);
            let assign17880_e16889: f64 = (p.p29 * assign17880_e16888);
            let assign17880_e16891: f64 = (assign17880_e16889 * locals.var_fbreakdown);
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (assign17880_e16891, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign17880_e16889 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign17880_e16889 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign17880_e16889 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign17880_e16889 * locals.var_fbreakdown_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign17890_e16899: f64 = (locals.var_absource_i * locals.var_ijunbot);
            let assign17890_e16902: f64 = (locals.var_lssource_i * locals.var_ijunsti);
            let assign17890_e16903: f64 = (assign17890_e16899 + assign17890_e16902);
            let assign17890_e16906: f64 = (locals.var_lgsource_i * locals.var_ijungat);
            let assign17890_e16907: f64 = (assign17890_e16903 + assign17890_e16906);
            (locals.var_i1, locals.var_i1_dn5, locals.var_i1_dn6, locals.var_i1_dn7, locals.var_i1_dn8, ) = (assign17890_e16907, (((locals.var_absource_i * locals.var_ijunbot_dn5) + (locals.var_lssource_i * locals.var_ijunsti_dn5)) + (locals.var_lgsource_i * locals.var_ijungat_dn5)), (((locals.var_absource_i * locals.var_ijunbot_dn6) + (locals.var_lssource_i * locals.var_ijunsti_dn6)) + (locals.var_lgsource_i * locals.var_ijungat_dn6)), (((locals.var_absource_i * locals.var_ijunbot_dn7) + (locals.var_lssource_i * locals.var_ijunsti_dn7)) + (locals.var_lgsource_i * locals.var_ijungat_dn7)), (((locals.var_absource_i * locals.var_ijunbot_dn8) + (locals.var_lssource_i * locals.var_ijunsti_dn8)) + (locals.var_lgsource_i * locals.var_ijungat_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            locals.var_vbbt = 0.0;
            locals.var_two_psistar = 0.0;
        }
        let assign17920_e16933: f64 = if (!(((locals.var_absource_i == 0.0) && (locals.var_lssource_i == 0.0)) && (locals.var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard304 = assign17920_e16933;
        let assign18000_e17019: f64 = if locals.var_v2 < locals.var_vmax_s { 1.0 } else { 0.0 };
        locals.var_guard305 = assign18000_e17019;
        let assign18010_e17021: f64 = (-0.5);
        let assign18010_e17024: f64 = (locals.var_v2 * locals.var_phitdinv);
        let assign18010_e17025: f64 = (assign18010_e17021 * assign18010_e17024);
        let assign18010_e17026: f64 = (assign18010_e17025).abs();
        let assign18010_e17028: f64 = if assign18010_e17026 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard306 = assign18010_e17028;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 != 0.0)) && (locals.var_guard306 != 0.0)) {
            let assign18020_e17039: f64 = (-0.5);
            let assign18020_e17042: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign18020_e17043: f64 = (assign18020_e17039 * assign18020_e17042);
            let assign18020_e17044: f64 = (assign18020_e17043).exp();
            locals.var_z = assign18020_e17044;
        }
        let assign18030_e17048: f64 = (-0.5);
        let assign18030_e17051: f64 = (locals.var_v2 * locals.var_phitdinv);
        let assign18030_e17052: f64 = (assign18030_e17048 * assign18030_e17051);
        let assign18030_e17054: f64 = if assign18030_e17052 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard307 = assign18030_e17054;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 != 0.0)) && (locals.var_guard306 == 0.0)) && (locals.var_guard307 != 0.0)) {
            let assign18040_e17070: f64 = (-230.25850929940458);
            let assign18040_e17072: f64 = (-0.5);
            let assign18040_e17075: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign18040_e17076: f64 = (assign18040_e17072 * assign18040_e17075);
            let assign18040_e17077: f64 = (assign18040_e17070 - assign18040_e17076);
            let assign18040_e17081: f64 = (-230.25850929940458);
            let assign18040_e17083: f64 = (-0.5);
            let assign18040_e17086: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign18040_e17087: f64 = (assign18040_e17083 * assign18040_e17086);
            let assign18040_e17088: f64 = (assign18040_e17081 - assign18040_e17087);
            let assign18040_e17091: f64 = (-230.25850929940458);
            let assign18040_e17093: f64 = (-0.5);
            let assign18040_e17096: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign18040_e17097: f64 = (assign18040_e17093 * assign18040_e17096);
            let assign18040_e17098: f64 = (assign18040_e17091 - assign18040_e17097);
            let assign18040_e17100: f64 = (assign18040_e17098 * 0.3333333333333333);
            let assign18040_e17101: f64 = (1.0 + assign18040_e17100);
            let assign18040_e17102: f64 = (assign18040_e17088 * assign18040_e17101);
            let assign18040_e17103: f64 = (0.5 * assign18040_e17102);
            let assign18040_e17104: f64 = (1.0 + assign18040_e17103);
            let assign18040_e17105: f64 = (assign18040_e17077 * assign18040_e17104);
            let assign18040_e17106: f64 = (1.0 + assign18040_e17105);
            let assign18040_e17107: f64 = (1e-100 / assign18040_e17106);
            locals.var_z = assign18040_e17107;
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 != 0.0)) && (locals.var_guard306 == 0.0)) && (locals.var_guard307 == 0.0)) {
            let assign18050_e17126: f64 = (-0.5);
            let assign18050_e17129: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign18050_e17130: f64 = (assign18050_e17126 * assign18050_e17129);
            let assign18050_e17132: f64 = (assign18050_e17130 - 230.25850929940458);
            let assign18050_e17136: f64 = (-0.5);
            let assign18050_e17139: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign18050_e17140: f64 = (assign18050_e17136 * assign18050_e17139);
            let assign18050_e17142: f64 = (assign18050_e17140 - 230.25850929940458);
            let assign18050_e17145: f64 = (-0.5);
            let assign18050_e17148: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign18050_e17149: f64 = (assign18050_e17145 * assign18050_e17148);
            let assign18050_e17151: f64 = (assign18050_e17149 - 230.25850929940458);
            let assign18050_e17153: f64 = (assign18050_e17151 * 0.3333333333333333);
            let assign18050_e17154: f64 = (1.0 + assign18050_e17153);
            let assign18050_e17155: f64 = (assign18050_e17142 * assign18050_e17154);
            let assign18050_e17156: f64 = (0.5 * assign18050_e17155);
            let assign18050_e17157: f64 = (1.0 + assign18050_e17156);
            let assign18050_e17158: f64 = (assign18050_e17132 * assign18050_e17157);
            let assign18050_e17159: f64 = (1.0 + assign18050_e17158);
            let assign18050_e17160: f64 = (1e100 * assign18050_e17159);
            locals.var_z = assign18050_e17160;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 != 0.0)) {
            let assign18060_e17172: f64 = (1.0 / locals.var_z);
            locals.var_zinv = assign18060_e17172;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 != 0.0)) {
            let assign18070_e17184: f64 = (locals.var_zinv * locals.var_zinv);
            locals.var_idmult = assign18070_e17184;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 == 0.0)) {
            let assign18080_e17198: f64 = (locals.var_v2 - locals.var_vmax_s);
            let assign18080_e17200: f64 = (assign18080_e17198 * locals.var_phitdinv);
            let assign18080_e17201: f64 = (1.0 + assign18080_e17200);
            let assign18080_e17203: f64 = (assign18080_e17201 * locals.var_exp_vmax_over_phitd_s);
            locals.var_idmult = assign18080_e17203;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 == 0.0)) {
            let assign18090_e17215: f64 = (locals.var_idmult).sqrt();
            locals.var_zinv = assign18090_e17215;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 == 0.0)) {
            let assign18100_e17228: f64 = (1.0 / locals.var_zinv);
            locals.var_z = assign18100_e17228;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) {
            let assign18110_e17238: f64 = (locals.var_idmult - 1.0);
            locals.var_idmult = assign18110_e17238;
        }
        let assign18120_e17243: f64 = if locals.var_v2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard308 = assign18120_e17243;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard308 != 0.0)) {
            let assign18130_e17255: f64 = (2.0 + locals.var_z);
            let assign18130_e17258: f64 = (locals.var_z + 1.0);
            let assign18130_e17261: f64 = (locals.var_z + 3.0);
            let assign18130_e17262: f64 = (assign18130_e17258 * assign18130_e17261);
            let assign18130_e17263: f64 = (assign18130_e17262).sqrt();
            let assign18130_e17264: f64 = (assign18130_e17255 + assign18130_e17263);
            let assign18130_e17265: f64 = (assign18130_e17264).ln();
            let assign18130_e17266: f64 = (locals.var_phitd * assign18130_e17265);
            let assign18130_e17267: f64 = (2.0 * assign18130_e17266);
            locals.var_two_psistar = assign18130_e17267;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard308 == 0.0)) {
            let assign18140_e17279: f64 = (-locals.var_v2);
            let assign18140_e17284: f64 = (2.0 * locals.var_zinv);
            let assign18140_e17286: f64 = (assign18140_e17284 + 1.0);
            let assign18140_e17289: f64 = (1.0 + locals.var_zinv);
            let assign18140_e17293: f64 = (3.0 * locals.var_zinv);
            let assign18140_e17294: f64 = (1.0 + assign18140_e17293);
            let assign18140_e17295: f64 = (assign18140_e17289 * assign18140_e17294);
            let assign18140_e17296: f64 = (assign18140_e17295).sqrt();
            let assign18140_e17297: f64 = (assign18140_e17286 + assign18140_e17296);
            let assign18140_e17298: f64 = (assign18140_e17297).ln();
            let assign18140_e17299: f64 = (locals.var_phitd * assign18140_e17298);
            let assign18140_e17300: f64 = (2.0 * assign18140_e17299);
            let assign18140_e17301: f64 = (assign18140_e17279 + assign18140_e17300);
            locals.var_two_psistar = assign18140_e17301;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) {
            let assign18150_e17311: f64 = (locals.var_vbimin_s - locals.var_two_psistar);
            locals.var_vjlim = assign18150_e17311;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) {
            let assign18160_e17322: f64 = (locals.var_v2 + locals.var_vjlim);
            let assign18160_e17325: f64 = (locals.var_v2 - locals.var_vjlim);
            let assign18160_e17328: f64 = (locals.var_v2 - locals.var_vjlim);
            let assign18160_e17329: f64 = (assign18160_e17325 * assign18160_e17328);
            let assign18160_e17332: f64 = (4.0 * locals.var_phitd);
            let assign18160_e17334: f64 = (assign18160_e17332 * locals.var_phitd);
            let assign18160_e17335: f64 = (assign18160_e17329 + assign18160_e17334);
            let assign18160_e17336: f64 = (assign18160_e17335).sqrt();
            let assign18160_e17337: f64 = (assign18160_e17322 - assign18160_e17336);
            let assign18160_e17338: f64 = (0.5 * assign18160_e17337);
            locals.var_vjsrh = assign18160_e17338;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) {
            let assign18170_e17349: f64 = (locals.var_v2 + locals.var_vbbtlim_s);
            let assign18170_e17352: f64 = (locals.var_v2 - locals.var_vbbtlim_s);
            let assign18170_e17355: f64 = (locals.var_v2 - locals.var_vbbtlim_s);
            let assign18170_e17356: f64 = (assign18170_e17352 * assign18170_e17355);
            let assign18170_e17359: f64 = (4.0 * locals.var_phitr);
            let assign18170_e17361: f64 = (assign18170_e17359 * locals.var_phitr);
            let assign18170_e17362: f64 = (assign18170_e17356 + assign18170_e17361);
            let assign18170_e17363: f64 = (assign18170_e17362).sqrt();
            let assign18170_e17364: f64 = (assign18170_e17349 - assign18170_e17363);
            let assign18170_e17365: f64 = (0.5 * assign18170_e17364);
            locals.var_vbbt = assign18170_e17365;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard304 != 0.0)) {
            let assign18180_e17376: f64 = locals.var_v2;
            let assign18180_e17379: f64 = locals.var_v2;
            let assign18180_e17382: f64 = locals.var_v2;
            let assign18180_e17383: f64 = (assign18180_e17379 * assign18180_e17382);
            let assign18180_e17386: f64 = (4.0 * 1e-6);
            let assign18180_e17388: f64 = (assign18180_e17386 * 1e-6);
            let assign18180_e17389: f64 = (assign18180_e17383 + assign18180_e17388);
            let assign18180_e17390: f64 = (assign18180_e17389).sqrt();
            let assign18180_e17391: f64 = (assign18180_e17376 - assign18180_e17390);
            let assign18180_e17392: f64 = (0.5 * assign18180_e17391);
            locals.var_vav = assign18180_e17392;
        }
        let assign18190_e17397: f64 = if locals.var_absource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard309 = assign18190_e17397;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 != 0.0)) {
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) {
            let assign18210_e17414: f64 = (locals.var_idsatbot * locals.var_idmult);
            locals.var_id__blk219 = assign18210_e17414;
        }
        let assign18220_e17423: f64 = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard310 = assign18220_e17423;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 == 0.0)) {
            let assign18240_e17446: f64 = (locals.var_vbibot - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign18240_e17446;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 == 0.0)) {
            let assign18250_e17462: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign18250_e17463: f64 = (1.0 - assign18250_e17462);
            let assign18250_e17464: f64 = (assign18250_e17463).sqrt();
            let assign18250_e17465: f64 = (1.0 - assign18250_e17464);
            locals.var_wsrhstep = assign18250_e17465;
        }
        let assign18260_e17470: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign18260_e17470;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 == 0.0)) && (locals.var_guard311 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 == 0.0)) && (locals.var_guard311 == 0.0)) {
            let assign18280_e17499: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign18280_e17501: f64 = (locals.var_wsrhstep).ln();
            let assign18280_e17502: f64 = (assign18280_e17499 * assign18280_e17501);
            let assign18280_e17505: f64 = (1.0 - locals.var_wsrhstep);
            let assign18280_e17506: f64 = (assign18280_e17502 / assign18280_e17505);
            let assign18280_e17508: f64 = (assign18280_e17506 + locals.var_wsrhstep);
            let assign18280_e17512: f64 = (2.0 * p.p831);
            let assign18280_e17513: f64 = (1.0 - assign18280_e17512);
            let assign18280_e17514: f64 = (assign18280_e17508 * assign18280_e17513);
            locals.var_dwsrh = assign18280_e17514;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 == 0.0)) {
            let assign18290_e17528: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign18290_e17528;
        }
        let assign18300_e17533: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign18300_e17533;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 == 0.0)) && (locals.var_guard312 != 0.0)) {
            let assign18310_e17547: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv);
            let assign18310_e17548: f64 = (assign18310_e17547).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18310_e17548, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 == 0.0)) && (locals.var_guard312 == 0.0)) {
            let assign18320_e17565: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv);
            let assign18320_e17567: f64 = (assign18320_e17565).powf(p.p831);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18320_e17567, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 == 0.0)) {
            let assign18330_e17581: f64 = (locals.var_wdepnulrbot * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign18330_e17581, (locals.var_wdepnulrbot * locals.var_tmp_dn5), (locals.var_wdepnulrbot * locals.var_tmp_dn6), (locals.var_wdepnulrbot * locals.var_tmp_dn7), (locals.var_wdepnulrbot * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 == 0.0)) {
            let assign18340_e17596: f64 = (locals.var_zinv - 1.0);
            let assign18340_e17598: f64 = (assign18340_e17596 * locals.var_wdep);
            let assign18340_e17599: f64 = (locals.var_ftdbot * assign18340_e17598);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign18340_e17599, (locals.var_ftdbot * (assign18340_e17596 * locals.var_wdep_dn5)), (locals.var_ftdbot * (assign18340_e17596 * locals.var_wdep_dn6)), (locals.var_ftdbot * (assign18340_e17596 * locals.var_wdep_dn7)), (locals.var_ftdbot * (assign18340_e17596 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 == 0.0)) {
            let assign18350_e17614: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign18350_e17615: f64 = (p.p840 * assign18350_e17614);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign18350_e17615, (p.p840 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign18360_e17620: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard313 = assign18360_e17620;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18380_e17644: f64 = (locals.var_wdep * locals.var_one_minus_pbot);
            let assign18380_e17646: f64 = (assign18380_e17644 / locals.var_vbi_minus_vjsrh);
            let assign18380_e17647: f64 = (locals.var_btatpartbot * assign18380_e17646);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign18380_e17647, (locals.var_btatpartbot * ((locals.var_wdep_dn5 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn6 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn7 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn8 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), );
        }
    }
    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18390_e17661: f64 = (0.666666666666667 * locals.var_atatbot);
            let assign18390_e17663: f64 = (assign18390_e17661 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign18390_e17663, (-((assign18390_e17661 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign18390_e17661 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign18390_e17661 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign18390_e17661 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18400_e17677: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign18400_e17677, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18410_e17691: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign18410_e17694: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign18410_e17696: f64 = (assign18410_e17694 + 1.0);
            let assign18410_e17697: f64 = (assign18410_e17691 / assign18410_e17696);
            let assign18410_e17698: f64 = (assign18410_e17697).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign18410_e17698, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign18410_e17696) - (assign18410_e17691 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign18410_e17696 * assign18410_e17696)) / (2.0 * assign18410_e17698)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign18410_e17696) - (assign18410_e17691 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign18410_e17696 * assign18410_e17696)) / (2.0 * assign18410_e17698)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign18410_e17696) - (assign18410_e17691 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign18410_e17696 * assign18410_e17696)) / (2.0 * assign18410_e17698)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign18410_e17696) - (assign18410_e17691 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign18410_e17696 * assign18410_e17696)) / (2.0 * assign18410_e17698)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18420_e17711: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign18420_e17711, (locals.var_umax_dn5 / (2.0 * assign18420_e17711)), (locals.var_umax_dn6 / (2.0 * assign18420_e17711)), (locals.var_umax_dn7 / (2.0 * assign18420_e17711)), (locals.var_umax_dn8 / (2.0 * assign18420_e17711)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18430_e17725: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign18430_e17725, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign18440_e17729: f64 = (-p.p831);
        let assign18440_e17731: f64 = (assign18440_e17729 * locals.var_one_over_one_minus_pbot);
        let assign18440_e17733: f64 = (-1.0);
        let assign18440_e17734: f64 = if assign18440_e17731 == assign18440_e17733 { 1.0 } else { 0.0 };
        locals.var_guard314 = assign18440_e17734;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard314 != 0.0)) {
            let assign18450_e17750: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign18450_e17751: f64 = (1.0 + assign18450_e17750);
            let assign18450_e17752: f64 = (1.0 / assign18450_e17751);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign18450_e17752, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign18450_e17751 * assign18450_e17751))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign18450_e17751 * assign18450_e17751))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign18450_e17751 * assign18450_e17751))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign18450_e17751 * assign18450_e17751))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard314 == 0.0)) {
            let assign18460_e17770: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign18460_e17771: f64 = (1.0 + assign18460_e17770);
            let assign18460_e17773: f64 = (-p.p831);
            let assign18460_e17775: f64 = (assign18460_e17773 * locals.var_one_over_one_minus_pbot);
            let assign18460_e17776: f64 = (assign18460_e17771).powf(assign18460_e17775);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign18460_e17776, if 0.0 == 0.0 && ((assign18460_e17775) as f64).is_finite() && ((assign18460_e17775) as f64).fract() == 0.0 { if assign18460_e17775 == 0.0 { 0.0 } else { (assign18460_e17775 * ((assign18460_e17771).powf(assign18460_e17775 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign18460_e17776 * (assign18460_e17775 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign18460_e17771))) }, if 0.0 == 0.0 && ((assign18460_e17775) as f64).is_finite() && ((assign18460_e17775) as f64).fract() == 0.0 { if assign18460_e17775 == 0.0 { 0.0 } else { (assign18460_e17775 * ((assign18460_e17771).powf(assign18460_e17775 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign18460_e17776 * (assign18460_e17775 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign18460_e17771))) }, if 0.0 == 0.0 && ((assign18460_e17775) as f64).is_finite() && ((assign18460_e17775) as f64).fract() == 0.0 { if assign18460_e17775 == 0.0 { 0.0 } else { (assign18460_e17775 * ((assign18460_e17771).powf(assign18460_e17775 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign18460_e17776 * (assign18460_e17775 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign18460_e17771))) }, if 0.0 == 0.0 && ((assign18460_e17775) as f64).is_finite() && ((assign18460_e17775) as f64).fract() == 0.0 { if assign18460_e17775 == 0.0 { 0.0 } else { (assign18460_e17775 * ((assign18460_e17771).powf(assign18460_e17775 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign18460_e17776 * (assign18460_e17775 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign18460_e17771))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18470_e17790: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign18470_e17793: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign18470_e17794: f64 = (assign18470_e17790 / assign18470_e17793);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign18470_e17794, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign18470_e17793) - (assign18470_e17790 * locals.var_wgamma_dn5)) / (assign18470_e17793 * assign18470_e17793)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign18470_e17793) - (assign18470_e17790 * locals.var_wgamma_dn6)) / (assign18470_e17793 * assign18470_e17793)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign18470_e17793) - (assign18470_e17790 * locals.var_wgamma_dn7)) / (assign18470_e17793 * assign18470_e17793)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign18470_e17793) - (assign18470_e17790 * locals.var_wgamma_dn8)) / (assign18470_e17793 * assign18470_e17793)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18480_e17809: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign18480_e17810: f64 = (0.375 * assign18480_e17809);
            let assign18480_e17811: f64 = (assign18480_e17810).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign18480_e17811, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign18480_e17811)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign18480_e17811)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign18480_e17811)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign18480_e17811)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18490_e17826: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign18490_e17827: f64 = (2.0 * assign18490_e17826);
            let assign18490_e17829: f64 = (assign18490_e17827 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign18490_e17829, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18500_e17843: f64 = (locals.var_atatbot * locals.var_twoatatoverthreebtat);
            let assign18500_e17845: f64 = (assign18500_e17843 * locals.var_sqrtumax);
            let assign18500_e17848: f64 = (locals.var_atatbot * locals.var_umax);
            let assign18500_e17849: f64 = (assign18500_e17845 - assign18500_e17848);
            let assign18500_e17853: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign18500_e17854: f64 = (0.5 * assign18500_e17853);
            let assign18500_e17855: f64 = (assign18500_e17849 + assign18500_e17854);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign18500_e17855, (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign18500_e17843 * locals.var_sqrtumax_dn5)) - (locals.var_atatbot * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign18500_e17843 * locals.var_sqrtumax_dn6)) - (locals.var_atatbot * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign18500_e17843 * locals.var_sqrtumax_dn7)) - (locals.var_atatbot * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign18500_e17843 * locals.var_sqrtumax_dn8)) - (locals.var_atatbot * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18510_e17869: f64 = (locals.var_ltat - 1.0);
            let assign18510_e17871: f64 = (assign18510_e17869 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign18510_e17871, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign18510_e17869 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign18510_e17869 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign18510_e17869 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign18510_e17869 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18520_e17885: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign18520_e17885, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign18530_e17890: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign18530_e17890;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 != 0.0)) {
            let assign18540_e17906: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign18540_e17907: f64 = (1.0 + assign18540_e17906);
            let assign18540_e17908: f64 = (1.0 / assign18540_e17907);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign18540_e17908, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign18540_e17907 * assign18540_e17907))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign18540_e17907 * assign18540_e17907))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign18540_e17907 * assign18540_e17907))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign18540_e17907 * assign18540_e17907))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard315 == 0.0)) {
            let assign18550_e17927: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign18550_e17928: f64 = (1.0 - assign18550_e17927);
            let assign18550_e17929: f64 = (1.0 / assign18550_e17928);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign18550_e17929, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign18550_e17928 * assign18550_e17928))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign18550_e17928 * assign18550_e17928))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign18550_e17928 * assign18550_e17928))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign18550_e17928 * assign18550_e17928))), );
        }
        let assign18560_e17933: f64 = (-locals.var_ysq);
        let assign18560_e17935: f64 = (assign18560_e17933 + locals.var_mtat);
        let assign18560_e17937: f64 = (-230.25850929940458);
        let assign18560_e17938: f64 = if assign18560_e17935 > assign18560_e17937 { 1.0 } else { 0.0 };
        locals.var_guard316 = assign18560_e17938;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard316 != 0.0)) {
            let assign18570_e17951: f64 = (-locals.var_ysq);
            let assign18570_e17953: f64 = (assign18570_e17951 + locals.var_mtat);
            let assign18570_e17954: f64 = (assign18570_e17953).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18570_e17954, (assign18570_e17954 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign18570_e17954 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign18570_e17954 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign18570_e17954 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard316 == 0.0)) {
            let assign18580_e17972: f64 = (-230.25850929940458);
            let assign18580_e17974: f64 = (-locals.var_ysq);
            let assign18580_e17976: f64 = (assign18580_e17974 + locals.var_mtat);
            let assign18580_e17977: f64 = (assign18580_e17972 - assign18580_e17976);
            let assign18580_e17981: f64 = (-230.25850929940458);
            let assign18580_e17983: f64 = (-locals.var_ysq);
            let assign18580_e17985: f64 = (assign18580_e17983 + locals.var_mtat);
            let assign18580_e17986: f64 = (assign18580_e17981 - assign18580_e17985);
            let assign18580_e17989: f64 = (-230.25850929940458);
            let assign18580_e17991: f64 = (-locals.var_ysq);
            let assign18580_e17993: f64 = (assign18580_e17991 + locals.var_mtat);
            let assign18580_e17994: f64 = (assign18580_e17989 - assign18580_e17993);
            let assign18580_e17996: f64 = (assign18580_e17994 * 0.3333333333333333);
            let assign18580_e17997: f64 = (1.0 + assign18580_e17996);
            let assign18580_e17998: f64 = (assign18580_e17986 * assign18580_e17997);
            let assign18580_e17999: f64 = (0.5 * assign18580_e17998);
            let assign18580_e18000: f64 = (1.0 + assign18580_e17999);
            let assign18580_e18001: f64 = (assign18580_e17977 * assign18580_e18000);
            let assign18580_e18002: f64 = (1.0 + assign18580_e18001);
            let assign18580_e18003: f64 = (1e-100 / assign18580_e18002);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18580_e18003, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign18580_e18000) + (assign18580_e17977 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign18580_e17997) + (assign18580_e17986 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign18580_e18002 * assign18580_e18002))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign18580_e18000) + (assign18580_e17977 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign18580_e17997) + (assign18580_e17986 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign18580_e18002 * assign18580_e18002))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign18580_e18000) + (assign18580_e17977 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign18580_e17997) + (assign18580_e17986 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign18580_e18002 * assign18580_e18002))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign18580_e18000) + (assign18580_e17977 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign18580_e17997) + (assign18580_e17986 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign18580_e18002 * assign18580_e18002))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18590_e18017: f64 = (0.29214664 * locals.var_terfc);
            let assign18590_e18021: f64 = (locals.var_terfc * locals.var_terfc);
            let assign18590_e18022: f64 = (locals.var_berfc * assign18590_e18021);
            let assign18590_e18023: f64 = (assign18590_e18017 + assign18590_e18022);
            let assign18590_e18027: f64 = (locals.var_terfc * locals.var_terfc);
            let assign18590_e18029: f64 = (assign18590_e18027 * locals.var_terfc);
            let assign18590_e18030: f64 = (locals.var_cerfc * assign18590_e18029);
            let assign18590_e18031: f64 = (assign18590_e18023 + assign18590_e18030);
            let assign18590_e18033: f64 = (assign18590_e18031 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign18590_e18033, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign18590_e18027 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign18590_e18031 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign18590_e18027 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign18590_e18031 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign18590_e18027 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign18590_e18031 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign18590_e18027 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign18590_e18031 * locals.var_tmp_dn8)), );
        }
        let assign18600_e18038: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard317 = assign18600_e18038;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard317 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign18620_e18055: f64 = (-230.25850929940458);
        let assign18620_e18056: f64 = if locals.var_mtat > assign18620_e18055 { 1.0 } else { 0.0 };
        locals.var_guard318 = assign18620_e18056;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard318 != 0.0)) {
            let assign18630_e18072: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18630_e18072, (assign18630_e18072 * locals.var_mtat_dn5), (assign18630_e18072 * locals.var_mtat_dn6), (assign18630_e18072 * locals.var_mtat_dn7), (assign18630_e18072 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard318 == 0.0)) {
            let assign18640_e18093: f64 = (-230.25850929940458);
            let assign18640_e18095: f64 = (assign18640_e18093 - locals.var_mtat);
            let assign18640_e18099: f64 = (-230.25850929940458);
            let assign18640_e18101: f64 = (assign18640_e18099 - locals.var_mtat);
            let assign18640_e18104: f64 = (-230.25850929940458);
            let assign18640_e18106: f64 = (assign18640_e18104 - locals.var_mtat);
            let assign18640_e18108: f64 = (assign18640_e18106 * 0.3333333333333333);
            let assign18640_e18109: f64 = (1.0 + assign18640_e18108);
            let assign18640_e18110: f64 = (assign18640_e18101 * assign18640_e18109);
            let assign18640_e18111: f64 = (0.5 * assign18640_e18110);
            let assign18640_e18112: f64 = (1.0 + assign18640_e18111);
            let assign18640_e18113: f64 = (assign18640_e18095 * assign18640_e18112);
            let assign18640_e18114: f64 = (1.0 + assign18640_e18113);
            let assign18640_e18115: f64 = (1e-100 / assign18640_e18114);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18640_e18115, (-((1e-100 * (((-locals.var_mtat_dn5) * assign18640_e18112) + (assign18640_e18095 * (0.5 * (((-locals.var_mtat_dn5) * assign18640_e18109) + (assign18640_e18101 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign18640_e18114 * assign18640_e18114))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign18640_e18112) + (assign18640_e18095 * (0.5 * (((-locals.var_mtat_dn6) * assign18640_e18109) + (assign18640_e18101 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign18640_e18114 * assign18640_e18114))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign18640_e18112) + (assign18640_e18095 * (0.5 * (((-locals.var_mtat_dn7) * assign18640_e18109) + (assign18640_e18101 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign18640_e18114 * assign18640_e18114))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign18640_e18112) + (assign18640_e18095 * (0.5 * (((-locals.var_mtat_dn8) * assign18640_e18109) + (assign18640_e18101 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign18640_e18114 * assign18640_e18114))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard317 == 0.0)) {
            let assign18650_e18132: f64 = (2.0 * locals.var_tmp);
            let assign18650_e18134: f64 = (assign18650_e18132 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign18650_e18134, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18660_e18148: f64 = (1.772453850905516 * 0.5);
            let assign18660_e18151: f64 = (locals.var_atatbot * locals.var_erfctimesexpmtat);
            let assign18660_e18153: f64 = (assign18660_e18151 / locals.var_ktat);
            let assign18660_e18154: f64 = (assign18660_e18148 * assign18660_e18153);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign18660_e18154, (assign18660_e18148 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign18660_e18151 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign18660_e18148 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign18660_e18151 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign18660_e18148 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign18660_e18151 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign18660_e18148 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign18660_e18151 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard313 == 0.0)) {
            let assign18670_e18169: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign18670_e18171: f64 = (assign18670_e18169 * locals.var_wtat);
            let assign18670_e18172: f64 = (p.p845 * assign18670_e18171);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign18670_e18172, (p.p845 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign18670_e18169 * locals.var_wtat_dn5))), (p.p845 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign18670_e18169 * locals.var_wtat_dn6))), (p.p845 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign18670_e18169 * locals.var_wtat_dn7))), (p.p845 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign18670_e18169 * locals.var_wtat_dn8))), );
        }
        let assign18680_e18177: f64 = if p.p851 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard319 = assign18680_e18177;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard319 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign18700_e18191: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard320 = assign18700_e18191;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard320 != 0.0)) {
            let assign18710_e18205: f64 = (p.p828 - locals.var_vbbt);
            let assign18710_e18207: f64 = (assign18710_e18205 * locals.var_vbirbotinv);
            let assign18710_e18208: f64 = (assign18710_e18207).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18710_e18208, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard320 == 0.0)) {
            let assign18720_e18225: f64 = (p.p828 - locals.var_vbbt);
            let assign18720_e18227: f64 = (assign18720_e18225 * locals.var_vbirbotinv);
            let assign18720_e18229: f64 = (assign18720_e18227).powf(p.p831);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18720_e18229, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard319 == 0.0)) {
            let assign18730_e18244: f64 = (p.p828 - locals.var_vbbt);
            let assign18730_e18246: f64 = (assign18730_e18244 * locals.var_wdepnulrinvbot);
            let assign18730_e18248: f64 = (assign18730_e18246 / locals.var_tmp);
            let assign18730_e18249: f64 = (locals.var_one_over_one_minus_pbot * assign18730_e18248);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign18730_e18249, (locals.var_one_over_one_minus_pbot * (-((assign18730_e18246 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign18730_e18246 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign18730_e18246 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign18730_e18246 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign18740_e18253: f64 = (-locals.var_fbbtbot);
        let assign18740_e18255: f64 = (assign18740_e18253 / locals.var_fmaxr);
        let assign18740_e18256: f64 = (assign18740_e18255).abs();
        let assign18740_e18258: f64 = if assign18740_e18256 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard321 = assign18740_e18258;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard321 != 0.0)) {
            let assign18750_e18271: f64 = (-locals.var_fbbtbot);
            let assign18750_e18273: f64 = (assign18750_e18271 / locals.var_fmaxr);
            let assign18750_e18274: f64 = (assign18750_e18273).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18750_e18274, (assign18750_e18274 * (-((assign18750_e18271 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign18750_e18274 * (-((assign18750_e18271 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign18750_e18274 * (-((assign18750_e18271 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign18750_e18274 * (-((assign18750_e18271 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign18760_e18278: f64 = (-locals.var_fbbtbot);
        let assign18760_e18280: f64 = (assign18760_e18278 / locals.var_fmaxr);
        let assign18760_e18282: f64 = if assign18760_e18280 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard322 = assign18760_e18282;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard322 != 0.0)) {
            let assign18770_e18300: f64 = (-230.25850929940458);
            let assign18770_e18302: f64 = (-locals.var_fbbtbot);
            let assign18770_e18304: f64 = (assign18770_e18302 / locals.var_fmaxr);
            let assign18770_e18305: f64 = (assign18770_e18300 - assign18770_e18304);
            let assign18770_e18309: f64 = (-230.25850929940458);
            let assign18770_e18311: f64 = (-locals.var_fbbtbot);
            let assign18770_e18313: f64 = (assign18770_e18311 / locals.var_fmaxr);
            let assign18770_e18314: f64 = (assign18770_e18309 - assign18770_e18313);
            let assign18770_e18317: f64 = (-230.25850929940458);
            let assign18770_e18319: f64 = (-locals.var_fbbtbot);
            let assign18770_e18321: f64 = (assign18770_e18319 / locals.var_fmaxr);
            let assign18770_e18322: f64 = (assign18770_e18317 - assign18770_e18321);
            let assign18770_e18324: f64 = (assign18770_e18322 * 0.3333333333333333);
            let assign18770_e18325: f64 = (1.0 + assign18770_e18324);
            let assign18770_e18326: f64 = (assign18770_e18314 * assign18770_e18325);
            let assign18770_e18327: f64 = (0.5 * assign18770_e18326);
            let assign18770_e18328: f64 = (1.0 + assign18770_e18327);
            let assign18770_e18329: f64 = (assign18770_e18305 * assign18770_e18328);
            let assign18770_e18330: f64 = (1.0 + assign18770_e18329);
            let assign18770_e18331: f64 = (1e-100 / assign18770_e18330);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18770_e18331, (-((1e-100 * (((-(-((assign18770_e18302 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign18770_e18328) + (assign18770_e18305 * (0.5 * (((-(-((assign18770_e18311 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign18770_e18325) + (assign18770_e18314 * ((-(-((assign18770_e18319 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign18770_e18330 * assign18770_e18330))), (-((1e-100 * (((-(-((assign18770_e18302 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign18770_e18328) + (assign18770_e18305 * (0.5 * (((-(-((assign18770_e18311 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign18770_e18325) + (assign18770_e18314 * ((-(-((assign18770_e18319 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign18770_e18330 * assign18770_e18330))), (-((1e-100 * (((-(-((assign18770_e18302 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign18770_e18328) + (assign18770_e18305 * (0.5 * (((-(-((assign18770_e18311 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign18770_e18325) + (assign18770_e18314 * ((-(-((assign18770_e18319 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign18770_e18330 * assign18770_e18330))), (-((1e-100 * (((-(-((assign18770_e18302 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign18770_e18328) + (assign18770_e18305 * (0.5 * (((-(-((assign18770_e18311 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign18770_e18325) + (assign18770_e18314 * ((-(-((assign18770_e18319 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign18770_e18330 * assign18770_e18330))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard322 == 0.0)) {
            let assign18780_e18352: f64 = (-locals.var_fbbtbot);
            let assign18780_e18354: f64 = (assign18780_e18352 / locals.var_fmaxr);
            let assign18780_e18356: f64 = (assign18780_e18354 - 230.25850929940458);
            let assign18780_e18360: f64 = (-locals.var_fbbtbot);
            let assign18780_e18362: f64 = (assign18780_e18360 / locals.var_fmaxr);
            let assign18780_e18364: f64 = (assign18780_e18362 - 230.25850929940458);
            let assign18780_e18367: f64 = (-locals.var_fbbtbot);
            let assign18780_e18369: f64 = (assign18780_e18367 / locals.var_fmaxr);
            let assign18780_e18371: f64 = (assign18780_e18369 - 230.25850929940458);
            let assign18780_e18373: f64 = (assign18780_e18371 * 0.3333333333333333);
            let assign18780_e18374: f64 = (1.0 + assign18780_e18373);
            let assign18780_e18375: f64 = (assign18780_e18364 * assign18780_e18374);
            let assign18780_e18376: f64 = (0.5 * assign18780_e18375);
            let assign18780_e18377: f64 = (1.0 + assign18780_e18376);
            let assign18780_e18378: f64 = (assign18780_e18356 * assign18780_e18377);
            let assign18780_e18379: f64 = (1.0 + assign18780_e18378);
            let assign18780_e18380: f64 = (1e100 * assign18780_e18379);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18780_e18380, (1e100 * (((-((assign18780_e18352 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign18780_e18377) + (assign18780_e18356 * (0.5 * (((-((assign18780_e18360 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign18780_e18374) + (assign18780_e18364 * ((-((assign18780_e18367 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign18780_e18352 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign18780_e18377) + (assign18780_e18356 * (0.5 * (((-((assign18780_e18360 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign18780_e18374) + (assign18780_e18364 * ((-((assign18780_e18367 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign18780_e18352 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign18780_e18377) + (assign18780_e18356 * (0.5 * (((-((assign18780_e18360 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign18780_e18374) + (assign18780_e18364 * ((-((assign18780_e18367 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign18780_e18352 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign18780_e18377) + (assign18780_e18356 * (0.5 * (((-((assign18780_e18360 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign18780_e18374) + (assign18780_e18364 * ((-((assign18780_e18367 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard319 == 0.0)) {
            let assign18790_e18395: f64 = (locals.var_v2 * locals.var_fmaxr);
            let assign18790_e18397: f64 = (assign18790_e18395 * locals.var_fmaxr);
            let assign18790_e18399: f64 = (assign18790_e18397 * locals.var_tmp);
            let assign18790_e18400: f64 = (p.p851 * assign18790_e18399);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign18790_e18400, (p.p851 * (((((locals.var_v2 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign18790_e18395 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign18790_e18397 * locals.var_tmp_dn5))), (p.p851 * (((((locals.var_v2 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign18790_e18395 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign18790_e18397 * locals.var_tmp_dn6))), (p.p851 * (((((locals.var_v2 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign18790_e18395 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign18790_e18397 * locals.var_tmp_dn7))), (p.p851 * (((((locals.var_v2 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign18790_e18395 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign18790_e18397 * locals.var_tmp_dn8))), );
        }
        let assign18800_e18405: f64 = if p.p860 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard323 = assign18800_e18405;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard323 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign18820_e18419: f64 = (-locals.var_alphaav);
        let assign18820_e18421: f64 = (assign18820_e18419 * p.p860);
        let assign18820_e18422: f64 = if locals.var_vav > assign18820_e18421 { 1.0 } else { 0.0 };
        locals.var_guard324 = assign18820_e18422;
        let assign18830_e18425: f64 = if p.p863 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard325 = assign18830_e18425;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard323 == 0.0)) && (locals.var_guard324 != 0.0)) && (locals.var_guard325 != 0.0)) {
            let assign18840_e18441: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign18840_e18444: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign18840_e18445: f64 = (assign18840_e18441 * assign18840_e18444);
            let assign18840_e18448: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign18840_e18449: f64 = (assign18840_e18445 * assign18840_e18448);
            let assign18840_e18452: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign18840_e18453: f64 = (assign18840_e18449 * assign18840_e18452);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18840_e18453, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard323 == 0.0)) && (locals.var_guard324 != 0.0)) && (locals.var_guard325 == 0.0)) {
            let assign18850_e18472: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign18850_e18473: f64 = (assign18850_e18472).abs();
            let assign18850_e18475: f64 = (assign18850_e18473).powf(p.p863);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign18850_e18475, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard323 == 0.0)) && (locals.var_guard324 != 0.0)) {
            let assign18860_e18492: f64 = (1.0 - locals.var_tmp);
            let assign18860_e18493: f64 = (1.0 / assign18860_e18492);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign18860_e18493, (-((-locals.var_tmp_dn5) / (assign18860_e18492 * assign18860_e18492))), (-((-locals.var_tmp_dn6) / (assign18860_e18492 * assign18860_e18492))), (-((-locals.var_tmp_dn7) / (assign18860_e18492 * assign18860_e18492))), (-((-locals.var_tmp_dn8) / (assign18860_e18492 * assign18860_e18492))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard323 == 0.0)) && (locals.var_guard324 == 0.0)) {
            let assign18870_e18512: f64 = (locals.var_alphaav * p.p860);
            let assign18870_e18513: f64 = (locals.var_vav + assign18870_e18512);
            let assign18870_e18515: f64 = (assign18870_e18513 * locals.var_slopebot);
            let assign18870_e18516: f64 = (locals.var_fstopbot + assign18870_e18515);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign18870_e18516, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard309 == 0.0)) {
            let assign18880_e18528: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign18880_e18530: f64 = (assign18880_e18528 + locals.var_itat);
            let assign18880_e18532: f64 = (assign18880_e18530 + locals.var_ibbt);
            let assign18880_e18533: f64 = (p.p29 * assign18880_e18532);
            let assign18880_e18535: f64 = (assign18880_e18533 * locals.var_fbreakdown);
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (assign18880_e18535, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign18880_e18533 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign18880_e18533 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign18880_e18533 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign18880_e18533 * locals.var_fbreakdown_dn8)), );
        }
        let assign18890_e18540: f64 = if locals.var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard326 = assign18890_e18540;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 != 0.0)) {
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) {
            let assign18910_e18557: f64 = (locals.var_idsatsti * locals.var_idmult);
            locals.var_id__blk219 = assign18910_e18557;
        }
        let assign18920_e18566: f64 = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard327 = assign18920_e18566;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard327 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard327 == 0.0)) {
            let assign18940_e18589: f64 = (locals.var_vbisti - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign18940_e18589;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard327 == 0.0)) {
            let assign18950_e18605: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign18950_e18606: f64 = (1.0 - assign18950_e18605);
            let assign18950_e18607: f64 = (assign18950_e18606).sqrt();
            let assign18950_e18608: f64 = (1.0 - assign18950_e18607);
            locals.var_wsrhstep = assign18950_e18608;
        }
        let assign18960_e18613: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard328 = assign18960_e18613;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard327 == 0.0)) && (locals.var_guard328 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard327 == 0.0)) && (locals.var_guard328 == 0.0)) {
            let assign18980_e18642: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign18980_e18644: f64 = (locals.var_wsrhstep).ln();
            let assign18980_e18645: f64 = (assign18980_e18642 * assign18980_e18644);
            let assign18980_e18648: f64 = (1.0 - locals.var_wsrhstep);
            let assign18980_e18649: f64 = (assign18980_e18645 / assign18980_e18648);
            let assign18980_e18651: f64 = (assign18980_e18649 + locals.var_wsrhstep);
            let assign18980_e18655: f64 = (2.0 * p.p832);
            let assign18980_e18656: f64 = (1.0 - assign18980_e18655);
            let assign18980_e18657: f64 = (assign18980_e18651 * assign18980_e18656);
            locals.var_dwsrh = assign18980_e18657;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard327 == 0.0)) {
            let assign18990_e18671: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign18990_e18671;
        }
        let assign19000_e18676: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard329 = assign19000_e18676;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard327 == 0.0)) && (locals.var_guard329 != 0.0)) {
            let assign19010_e18690: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv);
            let assign19010_e18691: f64 = (assign19010_e18690).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19010_e18691, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard327 == 0.0)) && (locals.var_guard329 == 0.0)) {
            let assign19020_e18708: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv);
            let assign19020_e18710: f64 = (assign19020_e18708).powf(p.p832);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19020_e18710, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard327 == 0.0)) {
            let assign19030_e18724: f64 = (locals.var_wdepnulrsti * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign19030_e18724, (locals.var_wdepnulrsti * locals.var_tmp_dn5), (locals.var_wdepnulrsti * locals.var_tmp_dn6), (locals.var_wdepnulrsti * locals.var_tmp_dn7), (locals.var_wdepnulrsti * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard327 == 0.0)) {
            let assign19040_e18739: f64 = (locals.var_zinv - 1.0);
            let assign19040_e18741: f64 = (assign19040_e18739 * locals.var_wdep);
            let assign19040_e18742: f64 = (locals.var_ftdsti * assign19040_e18741);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign19040_e18742, (locals.var_ftdsti * (assign19040_e18739 * locals.var_wdep_dn5)), (locals.var_ftdsti * (assign19040_e18739 * locals.var_wdep_dn6)), (locals.var_ftdsti * (assign19040_e18739 * locals.var_wdep_dn7)), (locals.var_ftdsti * (assign19040_e18739 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard327 == 0.0)) {
            let assign19050_e18757: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign19050_e18758: f64 = (p.p841 * assign19050_e18757);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign19050_e18758, (p.p841 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign19060_e18763: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard330 = assign19060_e18763;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19080_e18787: f64 = (locals.var_wdep * locals.var_one_minus_psti);
            let assign19080_e18789: f64 = (assign19080_e18787 / locals.var_vbi_minus_vjsrh);
            let assign19080_e18790: f64 = (locals.var_btatpartsti * assign19080_e18789);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign19080_e18790, (locals.var_btatpartsti * ((locals.var_wdep_dn5 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn6 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn7 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn8 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19090_e18804: f64 = (0.666666666666667 * locals.var_atatsti);
            let assign19090_e18806: f64 = (assign19090_e18804 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign19090_e18806, (-((assign19090_e18804 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign19090_e18804 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign19090_e18804 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign19090_e18804 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19100_e18820: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign19100_e18820, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19110_e18834: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign19110_e18837: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign19110_e18839: f64 = (assign19110_e18837 + 1.0);
            let assign19110_e18840: f64 = (assign19110_e18834 / assign19110_e18839);
            let assign19110_e18841: f64 = (assign19110_e18840).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign19110_e18841, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign19110_e18839) - (assign19110_e18834 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign19110_e18839 * assign19110_e18839)) / (2.0 * assign19110_e18841)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign19110_e18839) - (assign19110_e18834 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign19110_e18839 * assign19110_e18839)) / (2.0 * assign19110_e18841)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign19110_e18839) - (assign19110_e18834 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign19110_e18839 * assign19110_e18839)) / (2.0 * assign19110_e18841)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign19110_e18839) - (assign19110_e18834 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign19110_e18839 * assign19110_e18839)) / (2.0 * assign19110_e18841)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19120_e18854: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign19120_e18854, (locals.var_umax_dn5 / (2.0 * assign19120_e18854)), (locals.var_umax_dn6 / (2.0 * assign19120_e18854)), (locals.var_umax_dn7 / (2.0 * assign19120_e18854)), (locals.var_umax_dn8 / (2.0 * assign19120_e18854)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19130_e18868: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign19130_e18868, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
    }
    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign19140_e18872: f64 = (-p.p832);
        let assign19140_e18874: f64 = (assign19140_e18872 * locals.var_one_over_one_minus_psti);
        let assign19140_e18876: f64 = (-1.0);
        let assign19140_e18877: f64 = if assign19140_e18874 == assign19140_e18876 { 1.0 } else { 0.0 };
        locals.var_guard331 = assign19140_e18877;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard331 != 0.0)) {
            let assign19150_e18893: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign19150_e18894: f64 = (1.0 + assign19150_e18893);
            let assign19150_e18895: f64 = (1.0 / assign19150_e18894);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign19150_e18895, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign19150_e18894 * assign19150_e18894))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign19150_e18894 * assign19150_e18894))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign19150_e18894 * assign19150_e18894))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign19150_e18894 * assign19150_e18894))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard331 == 0.0)) {
            let assign19160_e18913: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign19160_e18914: f64 = (1.0 + assign19160_e18913);
            let assign19160_e18916: f64 = (-p.p832);
            let assign19160_e18918: f64 = (assign19160_e18916 * locals.var_one_over_one_minus_psti);
            let assign19160_e18919: f64 = (assign19160_e18914).powf(assign19160_e18918);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign19160_e18919, if 0.0 == 0.0 && ((assign19160_e18918) as f64).is_finite() && ((assign19160_e18918) as f64).fract() == 0.0 { if assign19160_e18918 == 0.0 { 0.0 } else { (assign19160_e18918 * ((assign19160_e18914).powf(assign19160_e18918 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign19160_e18919 * (assign19160_e18918 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign19160_e18914))) }, if 0.0 == 0.0 && ((assign19160_e18918) as f64).is_finite() && ((assign19160_e18918) as f64).fract() == 0.0 { if assign19160_e18918 == 0.0 { 0.0 } else { (assign19160_e18918 * ((assign19160_e18914).powf(assign19160_e18918 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign19160_e18919 * (assign19160_e18918 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign19160_e18914))) }, if 0.0 == 0.0 && ((assign19160_e18918) as f64).is_finite() && ((assign19160_e18918) as f64).fract() == 0.0 { if assign19160_e18918 == 0.0 { 0.0 } else { (assign19160_e18918 * ((assign19160_e18914).powf(assign19160_e18918 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign19160_e18919 * (assign19160_e18918 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign19160_e18914))) }, if 0.0 == 0.0 && ((assign19160_e18918) as f64).is_finite() && ((assign19160_e18918) as f64).fract() == 0.0 { if assign19160_e18918 == 0.0 { 0.0 } else { (assign19160_e18918 * ((assign19160_e18914).powf(assign19160_e18918 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign19160_e18919 * (assign19160_e18918 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign19160_e18914))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19170_e18933: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign19170_e18936: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign19170_e18937: f64 = (assign19170_e18933 / assign19170_e18936);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign19170_e18937, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign19170_e18936) - (assign19170_e18933 * locals.var_wgamma_dn5)) / (assign19170_e18936 * assign19170_e18936)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign19170_e18936) - (assign19170_e18933 * locals.var_wgamma_dn6)) / (assign19170_e18936 * assign19170_e18936)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign19170_e18936) - (assign19170_e18933 * locals.var_wgamma_dn7)) / (assign19170_e18936 * assign19170_e18936)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign19170_e18936) - (assign19170_e18933 * locals.var_wgamma_dn8)) / (assign19170_e18936 * assign19170_e18936)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19180_e18952: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign19180_e18953: f64 = (0.375 * assign19180_e18952);
            let assign19180_e18954: f64 = (assign19180_e18953).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign19180_e18954, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign19180_e18954)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign19180_e18954)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign19180_e18954)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign19180_e18954)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19190_e18969: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign19190_e18970: f64 = (2.0 * assign19190_e18969);
            let assign19190_e18972: f64 = (assign19190_e18970 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign19190_e18972, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19200_e18986: f64 = (locals.var_atatsti * locals.var_twoatatoverthreebtat);
            let assign19200_e18988: f64 = (assign19200_e18986 * locals.var_sqrtumax);
            let assign19200_e18991: f64 = (locals.var_atatsti * locals.var_umax);
            let assign19200_e18992: f64 = (assign19200_e18988 - assign19200_e18991);
            let assign19200_e18996: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign19200_e18997: f64 = (0.5 * assign19200_e18996);
            let assign19200_e18998: f64 = (assign19200_e18992 + assign19200_e18997);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign19200_e18998, (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign19200_e18986 * locals.var_sqrtumax_dn5)) - (locals.var_atatsti * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign19200_e18986 * locals.var_sqrtumax_dn6)) - (locals.var_atatsti * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign19200_e18986 * locals.var_sqrtumax_dn7)) - (locals.var_atatsti * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign19200_e18986 * locals.var_sqrtumax_dn8)) - (locals.var_atatsti * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19210_e19012: f64 = (locals.var_ltat - 1.0);
            let assign19210_e19014: f64 = (assign19210_e19012 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign19210_e19014, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign19210_e19012 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign19210_e19012 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign19210_e19012 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign19210_e19012 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19220_e19028: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign19220_e19028, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign19230_e19033: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard332 = assign19230_e19033;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard332 != 0.0)) {
            let assign19240_e19049: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign19240_e19050: f64 = (1.0 + assign19240_e19049);
            let assign19240_e19051: f64 = (1.0 / assign19240_e19050);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign19240_e19051, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign19240_e19050 * assign19240_e19050))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign19240_e19050 * assign19240_e19050))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign19240_e19050 * assign19240_e19050))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign19240_e19050 * assign19240_e19050))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard332 == 0.0)) {
            let assign19250_e19070: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign19250_e19071: f64 = (1.0 - assign19250_e19070);
            let assign19250_e19072: f64 = (1.0 / assign19250_e19071);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign19250_e19072, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign19250_e19071 * assign19250_e19071))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign19250_e19071 * assign19250_e19071))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign19250_e19071 * assign19250_e19071))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign19250_e19071 * assign19250_e19071))), );
        }
        let assign19260_e19076: f64 = (-locals.var_ysq);
        let assign19260_e19078: f64 = (assign19260_e19076 + locals.var_mtat);
        let assign19260_e19080: f64 = (-230.25850929940458);
        let assign19260_e19081: f64 = if assign19260_e19078 > assign19260_e19080 { 1.0 } else { 0.0 };
        locals.var_guard333 = assign19260_e19081;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard333 != 0.0)) {
            let assign19270_e19094: f64 = (-locals.var_ysq);
            let assign19270_e19096: f64 = (assign19270_e19094 + locals.var_mtat);
            let assign19270_e19097: f64 = (assign19270_e19096).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19270_e19097, (assign19270_e19097 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign19270_e19097 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign19270_e19097 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign19270_e19097 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard333 == 0.0)) {
            let assign19280_e19115: f64 = (-230.25850929940458);
            let assign19280_e19117: f64 = (-locals.var_ysq);
            let assign19280_e19119: f64 = (assign19280_e19117 + locals.var_mtat);
            let assign19280_e19120: f64 = (assign19280_e19115 - assign19280_e19119);
            let assign19280_e19124: f64 = (-230.25850929940458);
            let assign19280_e19126: f64 = (-locals.var_ysq);
            let assign19280_e19128: f64 = (assign19280_e19126 + locals.var_mtat);
            let assign19280_e19129: f64 = (assign19280_e19124 - assign19280_e19128);
            let assign19280_e19132: f64 = (-230.25850929940458);
            let assign19280_e19134: f64 = (-locals.var_ysq);
            let assign19280_e19136: f64 = (assign19280_e19134 + locals.var_mtat);
            let assign19280_e19137: f64 = (assign19280_e19132 - assign19280_e19136);
            let assign19280_e19139: f64 = (assign19280_e19137 * 0.3333333333333333);
            let assign19280_e19140: f64 = (1.0 + assign19280_e19139);
            let assign19280_e19141: f64 = (assign19280_e19129 * assign19280_e19140);
            let assign19280_e19142: f64 = (0.5 * assign19280_e19141);
            let assign19280_e19143: f64 = (1.0 + assign19280_e19142);
            let assign19280_e19144: f64 = (assign19280_e19120 * assign19280_e19143);
            let assign19280_e19145: f64 = (1.0 + assign19280_e19144);
            let assign19280_e19146: f64 = (1e-100 / assign19280_e19145);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19280_e19146, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign19280_e19143) + (assign19280_e19120 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign19280_e19140) + (assign19280_e19129 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign19280_e19145 * assign19280_e19145))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign19280_e19143) + (assign19280_e19120 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign19280_e19140) + (assign19280_e19129 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign19280_e19145 * assign19280_e19145))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign19280_e19143) + (assign19280_e19120 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign19280_e19140) + (assign19280_e19129 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign19280_e19145 * assign19280_e19145))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign19280_e19143) + (assign19280_e19120 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign19280_e19140) + (assign19280_e19129 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign19280_e19145 * assign19280_e19145))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19290_e19160: f64 = (0.29214664 * locals.var_terfc);
            let assign19290_e19164: f64 = (locals.var_terfc * locals.var_terfc);
            let assign19290_e19165: f64 = (locals.var_berfc * assign19290_e19164);
            let assign19290_e19166: f64 = (assign19290_e19160 + assign19290_e19165);
            let assign19290_e19170: f64 = (locals.var_terfc * locals.var_terfc);
            let assign19290_e19172: f64 = (assign19290_e19170 * locals.var_terfc);
            let assign19290_e19173: f64 = (locals.var_cerfc * assign19290_e19172);
            let assign19290_e19174: f64 = (assign19290_e19166 + assign19290_e19173);
            let assign19290_e19176: f64 = (assign19290_e19174 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign19290_e19176, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign19290_e19170 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign19290_e19174 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign19290_e19170 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign19290_e19174 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign19290_e19170 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign19290_e19174 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign19290_e19170 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign19290_e19174 * locals.var_tmp_dn8)), );
        }
        let assign19300_e19181: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard334 = assign19300_e19181;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard334 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign19320_e19198: f64 = (-230.25850929940458);
        let assign19320_e19199: f64 = if locals.var_mtat > assign19320_e19198 { 1.0 } else { 0.0 };
        locals.var_guard335 = assign19320_e19199;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard334 == 0.0)) && (locals.var_guard335 != 0.0)) {
            let assign19330_e19215: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19330_e19215, (assign19330_e19215 * locals.var_mtat_dn5), (assign19330_e19215 * locals.var_mtat_dn6), (assign19330_e19215 * locals.var_mtat_dn7), (assign19330_e19215 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard334 == 0.0)) && (locals.var_guard335 == 0.0)) {
            let assign19340_e19236: f64 = (-230.25850929940458);
            let assign19340_e19238: f64 = (assign19340_e19236 - locals.var_mtat);
            let assign19340_e19242: f64 = (-230.25850929940458);
            let assign19340_e19244: f64 = (assign19340_e19242 - locals.var_mtat);
            let assign19340_e19247: f64 = (-230.25850929940458);
            let assign19340_e19249: f64 = (assign19340_e19247 - locals.var_mtat);
            let assign19340_e19251: f64 = (assign19340_e19249 * 0.3333333333333333);
            let assign19340_e19252: f64 = (1.0 + assign19340_e19251);
            let assign19340_e19253: f64 = (assign19340_e19244 * assign19340_e19252);
            let assign19340_e19254: f64 = (0.5 * assign19340_e19253);
            let assign19340_e19255: f64 = (1.0 + assign19340_e19254);
            let assign19340_e19256: f64 = (assign19340_e19238 * assign19340_e19255);
            let assign19340_e19257: f64 = (1.0 + assign19340_e19256);
            let assign19340_e19258: f64 = (1e-100 / assign19340_e19257);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19340_e19258, (-((1e-100 * (((-locals.var_mtat_dn5) * assign19340_e19255) + (assign19340_e19238 * (0.5 * (((-locals.var_mtat_dn5) * assign19340_e19252) + (assign19340_e19244 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign19340_e19257 * assign19340_e19257))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign19340_e19255) + (assign19340_e19238 * (0.5 * (((-locals.var_mtat_dn6) * assign19340_e19252) + (assign19340_e19244 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign19340_e19257 * assign19340_e19257))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign19340_e19255) + (assign19340_e19238 * (0.5 * (((-locals.var_mtat_dn7) * assign19340_e19252) + (assign19340_e19244 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign19340_e19257 * assign19340_e19257))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign19340_e19255) + (assign19340_e19238 * (0.5 * (((-locals.var_mtat_dn8) * assign19340_e19252) + (assign19340_e19244 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign19340_e19257 * assign19340_e19257))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard334 == 0.0)) {
            let assign19350_e19275: f64 = (2.0 * locals.var_tmp);
            let assign19350_e19277: f64 = (assign19350_e19275 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign19350_e19277, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19360_e19291: f64 = (1.772453850905516 * 0.5);
            let assign19360_e19294: f64 = (locals.var_atatsti * locals.var_erfctimesexpmtat);
            let assign19360_e19296: f64 = (assign19360_e19294 / locals.var_ktat);
            let assign19360_e19297: f64 = (assign19360_e19291 * assign19360_e19296);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign19360_e19297, (assign19360_e19291 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign19360_e19294 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign19360_e19291 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign19360_e19294 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign19360_e19291 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign19360_e19294 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign19360_e19291 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign19360_e19294 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign19370_e19312: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign19370_e19314: f64 = (assign19370_e19312 * locals.var_wtat);
            let assign19370_e19315: f64 = (p.p846 * assign19370_e19314);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign19370_e19315, (p.p846 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign19370_e19312 * locals.var_wtat_dn5))), (p.p846 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign19370_e19312 * locals.var_wtat_dn6))), (p.p846 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign19370_e19312 * locals.var_wtat_dn7))), (p.p846 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign19370_e19312 * locals.var_wtat_dn8))), );
        }
        let assign19380_e19320: f64 = if p.p852 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard336 = assign19380_e19320;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard336 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign19400_e19334: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard337 = assign19400_e19334;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard336 == 0.0)) && (locals.var_guard337 != 0.0)) {
            let assign19410_e19348: f64 = (p.p829 - locals.var_vbbt);
            let assign19410_e19350: f64 = (assign19410_e19348 * locals.var_vbirstiinv);
            let assign19410_e19351: f64 = (assign19410_e19350).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19410_e19351, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard336 == 0.0)) && (locals.var_guard337 == 0.0)) {
            let assign19420_e19368: f64 = (p.p829 - locals.var_vbbt);
            let assign19420_e19370: f64 = (assign19420_e19368 * locals.var_vbirstiinv);
            let assign19420_e19372: f64 = (assign19420_e19370).powf(p.p832);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19420_e19372, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard336 == 0.0)) {
            let assign19430_e19387: f64 = (p.p829 - locals.var_vbbt);
            let assign19430_e19389: f64 = (assign19430_e19387 * locals.var_wdepnulrinvsti);
            let assign19430_e19391: f64 = (assign19430_e19389 / locals.var_tmp);
            let assign19430_e19392: f64 = (locals.var_one_over_one_minus_psti * assign19430_e19391);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign19430_e19392, (locals.var_one_over_one_minus_psti * (-((assign19430_e19389 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign19430_e19389 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign19430_e19389 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign19430_e19389 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign19440_e19396: f64 = (-locals.var_fbbtsti);
        let assign19440_e19398: f64 = (assign19440_e19396 / locals.var_fmaxr);
        let assign19440_e19399: f64 = (assign19440_e19398).abs();
        let assign19440_e19401: f64 = if assign19440_e19399 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard338 = assign19440_e19401;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard336 == 0.0)) && (locals.var_guard338 != 0.0)) {
            let assign19450_e19414: f64 = (-locals.var_fbbtsti);
            let assign19450_e19416: f64 = (assign19450_e19414 / locals.var_fmaxr);
            let assign19450_e19417: f64 = (assign19450_e19416).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19450_e19417, (assign19450_e19417 * (-((assign19450_e19414 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign19450_e19417 * (-((assign19450_e19414 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign19450_e19417 * (-((assign19450_e19414 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign19450_e19417 * (-((assign19450_e19414 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign19460_e19421: f64 = (-locals.var_fbbtsti);
        let assign19460_e19423: f64 = (assign19460_e19421 / locals.var_fmaxr);
        let assign19460_e19425: f64 = if assign19460_e19423 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard339 = assign19460_e19425;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard336 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_guard339 != 0.0)) {
            let assign19470_e19443: f64 = (-230.25850929940458);
            let assign19470_e19445: f64 = (-locals.var_fbbtsti);
            let assign19470_e19447: f64 = (assign19470_e19445 / locals.var_fmaxr);
            let assign19470_e19448: f64 = (assign19470_e19443 - assign19470_e19447);
            let assign19470_e19452: f64 = (-230.25850929940458);
            let assign19470_e19454: f64 = (-locals.var_fbbtsti);
            let assign19470_e19456: f64 = (assign19470_e19454 / locals.var_fmaxr);
            let assign19470_e19457: f64 = (assign19470_e19452 - assign19470_e19456);
            let assign19470_e19460: f64 = (-230.25850929940458);
            let assign19470_e19462: f64 = (-locals.var_fbbtsti);
            let assign19470_e19464: f64 = (assign19470_e19462 / locals.var_fmaxr);
            let assign19470_e19465: f64 = (assign19470_e19460 - assign19470_e19464);
            let assign19470_e19467: f64 = (assign19470_e19465 * 0.3333333333333333);
            let assign19470_e19468: f64 = (1.0 + assign19470_e19467);
            let assign19470_e19469: f64 = (assign19470_e19457 * assign19470_e19468);
            let assign19470_e19470: f64 = (0.5 * assign19470_e19469);
            let assign19470_e19471: f64 = (1.0 + assign19470_e19470);
            let assign19470_e19472: f64 = (assign19470_e19448 * assign19470_e19471);
            let assign19470_e19473: f64 = (1.0 + assign19470_e19472);
            let assign19470_e19474: f64 = (1e-100 / assign19470_e19473);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19470_e19474, (-((1e-100 * (((-(-((assign19470_e19445 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign19470_e19471) + (assign19470_e19448 * (0.5 * (((-(-((assign19470_e19454 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign19470_e19468) + (assign19470_e19457 * ((-(-((assign19470_e19462 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign19470_e19473 * assign19470_e19473))), (-((1e-100 * (((-(-((assign19470_e19445 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign19470_e19471) + (assign19470_e19448 * (0.5 * (((-(-((assign19470_e19454 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign19470_e19468) + (assign19470_e19457 * ((-(-((assign19470_e19462 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign19470_e19473 * assign19470_e19473))), (-((1e-100 * (((-(-((assign19470_e19445 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign19470_e19471) + (assign19470_e19448 * (0.5 * (((-(-((assign19470_e19454 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign19470_e19468) + (assign19470_e19457 * ((-(-((assign19470_e19462 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign19470_e19473 * assign19470_e19473))), (-((1e-100 * (((-(-((assign19470_e19445 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign19470_e19471) + (assign19470_e19448 * (0.5 * (((-(-((assign19470_e19454 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign19470_e19468) + (assign19470_e19457 * ((-(-((assign19470_e19462 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign19470_e19473 * assign19470_e19473))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard336 == 0.0)) && (locals.var_guard338 == 0.0)) && (locals.var_guard339 == 0.0)) {
            let assign19480_e19495: f64 = (-locals.var_fbbtsti);
            let assign19480_e19497: f64 = (assign19480_e19495 / locals.var_fmaxr);
            let assign19480_e19499: f64 = (assign19480_e19497 - 230.25850929940458);
            let assign19480_e19503: f64 = (-locals.var_fbbtsti);
            let assign19480_e19505: f64 = (assign19480_e19503 / locals.var_fmaxr);
            let assign19480_e19507: f64 = (assign19480_e19505 - 230.25850929940458);
            let assign19480_e19510: f64 = (-locals.var_fbbtsti);
            let assign19480_e19512: f64 = (assign19480_e19510 / locals.var_fmaxr);
            let assign19480_e19514: f64 = (assign19480_e19512 - 230.25850929940458);
            let assign19480_e19516: f64 = (assign19480_e19514 * 0.3333333333333333);
            let assign19480_e19517: f64 = (1.0 + assign19480_e19516);
            let assign19480_e19518: f64 = (assign19480_e19507 * assign19480_e19517);
            let assign19480_e19519: f64 = (0.5 * assign19480_e19518);
            let assign19480_e19520: f64 = (1.0 + assign19480_e19519);
            let assign19480_e19521: f64 = (assign19480_e19499 * assign19480_e19520);
            let assign19480_e19522: f64 = (1.0 + assign19480_e19521);
            let assign19480_e19523: f64 = (1e100 * assign19480_e19522);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19480_e19523, (1e100 * (((-((assign19480_e19495 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign19480_e19520) + (assign19480_e19499 * (0.5 * (((-((assign19480_e19503 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign19480_e19517) + (assign19480_e19507 * ((-((assign19480_e19510 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign19480_e19495 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign19480_e19520) + (assign19480_e19499 * (0.5 * (((-((assign19480_e19503 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign19480_e19517) + (assign19480_e19507 * ((-((assign19480_e19510 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign19480_e19495 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign19480_e19520) + (assign19480_e19499 * (0.5 * (((-((assign19480_e19503 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign19480_e19517) + (assign19480_e19507 * ((-((assign19480_e19510 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign19480_e19495 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign19480_e19520) + (assign19480_e19499 * (0.5 * (((-((assign19480_e19503 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign19480_e19517) + (assign19480_e19507 * ((-((assign19480_e19510 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard336 == 0.0)) {
            let assign19490_e19538: f64 = (locals.var_v2 * locals.var_fmaxr);
            let assign19490_e19540: f64 = (assign19490_e19538 * locals.var_fmaxr);
            let assign19490_e19542: f64 = (assign19490_e19540 * locals.var_tmp);
            let assign19490_e19543: f64 = (p.p852 * assign19490_e19542);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign19490_e19543, (p.p852 * (((((locals.var_v2 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign19490_e19538 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign19490_e19540 * locals.var_tmp_dn5))), (p.p852 * (((((locals.var_v2 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign19490_e19538 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign19490_e19540 * locals.var_tmp_dn6))), (p.p852 * (((((locals.var_v2 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign19490_e19538 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign19490_e19540 * locals.var_tmp_dn7))), (p.p852 * (((((locals.var_v2 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign19490_e19538 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign19490_e19540 * locals.var_tmp_dn8))), );
        }
        let assign19500_e19548: f64 = if p.p861 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard340 = assign19500_e19548;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard340 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign19520_e19562: f64 = (-locals.var_alphaav);
        let assign19520_e19564: f64 = (assign19520_e19562 * p.p861);
        let assign19520_e19565: f64 = if locals.var_vav > assign19520_e19564 { 1.0 } else { 0.0 };
        locals.var_guard341 = assign19520_e19565;
        let assign19530_e19568: f64 = if p.p864 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard342 = assign19530_e19568;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard341 != 0.0)) && (locals.var_guard342 != 0.0)) {
            let assign19540_e19584: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign19540_e19587: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign19540_e19588: f64 = (assign19540_e19584 * assign19540_e19587);
            let assign19540_e19591: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign19540_e19592: f64 = (assign19540_e19588 * assign19540_e19591);
            let assign19540_e19595: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign19540_e19596: f64 = (assign19540_e19592 * assign19540_e19595);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19540_e19596, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard341 != 0.0)) && (locals.var_guard342 == 0.0)) {
            let assign19550_e19615: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign19550_e19616: f64 = (assign19550_e19615).abs();
            let assign19550_e19618: f64 = (assign19550_e19616).powf(p.p864);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19550_e19618, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard341 != 0.0)) {
            let assign19560_e19635: f64 = (1.0 - locals.var_tmp);
            let assign19560_e19636: f64 = (1.0 / assign19560_e19635);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign19560_e19636, (-((-locals.var_tmp_dn5) / (assign19560_e19635 * assign19560_e19635))), (-((-locals.var_tmp_dn6) / (assign19560_e19635 * assign19560_e19635))), (-((-locals.var_tmp_dn7) / (assign19560_e19635 * assign19560_e19635))), (-((-locals.var_tmp_dn8) / (assign19560_e19635 * assign19560_e19635))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard341 == 0.0)) {
            let assign19570_e19655: f64 = (locals.var_alphaav * p.p861);
            let assign19570_e19656: f64 = (locals.var_vav + assign19570_e19655);
            let assign19570_e19658: f64 = (assign19570_e19656 * locals.var_slopesti);
            let assign19570_e19659: f64 = (locals.var_fstopsti + assign19570_e19658);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign19570_e19659, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard326 == 0.0)) {
            let assign19580_e19671: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign19580_e19673: f64 = (assign19580_e19671 + locals.var_itat);
            let assign19580_e19675: f64 = (assign19580_e19673 + locals.var_ibbt);
            let assign19580_e19676: f64 = (p.p29 * assign19580_e19675);
            let assign19580_e19678: f64 = (assign19580_e19676 * locals.var_fbreakdown);
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (assign19580_e19678, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign19580_e19676 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign19580_e19676 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign19580_e19676 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign19580_e19676 * locals.var_fbreakdown_dn8)), );
        }
        let assign19590_e19683: f64 = if locals.var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard343 = assign19590_e19683;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 != 0.0)) {
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) {
            let assign19610_e19700: f64 = (locals.var_idsatgat * locals.var_idmult);
            locals.var_id__blk219 = assign19610_e19700;
        }
        let assign19620_e19709: f64 = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard344 = assign19620_e19709;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) {
            let assign19640_e19732: f64 = (locals.var_vbigat - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign19640_e19732;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) {
            let assign19650_e19748: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign19650_e19749: f64 = (1.0 - assign19650_e19748);
            let assign19650_e19750: f64 = (assign19650_e19749).sqrt();
            let assign19650_e19751: f64 = (1.0 - assign19650_e19750);
            locals.var_wsrhstep = assign19650_e19751;
        }
        let assign19660_e19756: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard345 = assign19660_e19756;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) && (locals.var_guard345 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) && (locals.var_guard345 == 0.0)) {
            let assign19680_e19785: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign19680_e19787: f64 = (locals.var_wsrhstep).ln();
            let assign19680_e19788: f64 = (assign19680_e19785 * assign19680_e19787);
            let assign19680_e19791: f64 = (1.0 - locals.var_wsrhstep);
            let assign19680_e19792: f64 = (assign19680_e19788 / assign19680_e19791);
            let assign19680_e19794: f64 = (assign19680_e19792 + locals.var_wsrhstep);
            let assign19680_e19798: f64 = (2.0 * p.p833);
            let assign19680_e19799: f64 = (1.0 - assign19680_e19798);
            let assign19680_e19800: f64 = (assign19680_e19794 * assign19680_e19799);
            locals.var_dwsrh = assign19680_e19800;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) {
            let assign19690_e19814: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign19690_e19814;
        }
        let assign19700_e19819: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard346 = assign19700_e19819;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) && (locals.var_guard346 != 0.0)) {
            let assign19710_e19833: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv);
            let assign19710_e19834: f64 = (assign19710_e19833).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19710_e19834, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) && (locals.var_guard346 == 0.0)) {
            let assign19720_e19851: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv);
            let assign19720_e19853: f64 = (assign19720_e19851).powf(p.p833);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19720_e19853, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) {
            let assign19730_e19867: f64 = (locals.var_wdepnulrgat * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign19730_e19867, (locals.var_wdepnulrgat * locals.var_tmp_dn5), (locals.var_wdepnulrgat * locals.var_tmp_dn6), (locals.var_wdepnulrgat * locals.var_tmp_dn7), (locals.var_wdepnulrgat * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) {
            let assign19740_e19882: f64 = (locals.var_zinv - 1.0);
            let assign19740_e19884: f64 = (assign19740_e19882 * locals.var_wdep);
            let assign19740_e19885: f64 = (locals.var_ftdgat * assign19740_e19884);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign19740_e19885, (locals.var_ftdgat * (assign19740_e19882 * locals.var_wdep_dn5)), (locals.var_ftdgat * (assign19740_e19882 * locals.var_wdep_dn6)), (locals.var_ftdgat * (assign19740_e19882 * locals.var_wdep_dn7)), (locals.var_ftdgat * (assign19740_e19882 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) {
            let assign19750_e19900: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign19750_e19901: f64 = (p.p842 * assign19750_e19900);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign19750_e19901, (p.p842 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign19760_e19906: f64 = if p.p847 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard347 = assign19760_e19906;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19780_e19930: f64 = (locals.var_wdep * locals.var_one_minus_pgat);
            let assign19780_e19932: f64 = (assign19780_e19930 / locals.var_vbi_minus_vjsrh);
            let assign19780_e19933: f64 = (locals.var_btatpartgat * assign19780_e19932);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign19780_e19933, (locals.var_btatpartgat * ((locals.var_wdep_dn5 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn6 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn7 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn8 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19790_e19947: f64 = (0.666666666666667 * locals.var_atatgat);
            let assign19790_e19949: f64 = (assign19790_e19947 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign19790_e19949, (-((assign19790_e19947 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign19790_e19947 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign19790_e19947 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign19790_e19947 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19800_e19963: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign19800_e19963, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19810_e19977: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign19810_e19980: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign19810_e19982: f64 = (assign19810_e19980 + 1.0);
            let assign19810_e19983: f64 = (assign19810_e19977 / assign19810_e19982);
            let assign19810_e19984: f64 = (assign19810_e19983).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign19810_e19984, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign19810_e19982) - (assign19810_e19977 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign19810_e19982 * assign19810_e19982)) / (2.0 * assign19810_e19984)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign19810_e19982) - (assign19810_e19977 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign19810_e19982 * assign19810_e19982)) / (2.0 * assign19810_e19984)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign19810_e19982) - (assign19810_e19977 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign19810_e19982 * assign19810_e19982)) / (2.0 * assign19810_e19984)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign19810_e19982) - (assign19810_e19977 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign19810_e19982 * assign19810_e19982)) / (2.0 * assign19810_e19984)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19820_e19997: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign19820_e19997, (locals.var_umax_dn5 / (2.0 * assign19820_e19997)), (locals.var_umax_dn6 / (2.0 * assign19820_e19997)), (locals.var_umax_dn7 / (2.0 * assign19820_e19997)), (locals.var_umax_dn8 / (2.0 * assign19820_e19997)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19830_e20011: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign19830_e20011, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign19840_e20015: f64 = (-p.p833);
        let assign19840_e20017: f64 = (assign19840_e20015 * locals.var_one_over_one_minus_pgat);
        let assign19840_e20019: f64 = (-1.0);
        let assign19840_e20020: f64 = if assign19840_e20017 == assign19840_e20019 { 1.0 } else { 0.0 };
        locals.var_guard348 = assign19840_e20020;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard348 != 0.0)) {
            let assign19850_e20036: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign19850_e20037: f64 = (1.0 + assign19850_e20036);
            let assign19850_e20038: f64 = (1.0 / assign19850_e20037);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign19850_e20038, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign19850_e20037 * assign19850_e20037))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign19850_e20037 * assign19850_e20037))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign19850_e20037 * assign19850_e20037))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign19850_e20037 * assign19850_e20037))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard348 == 0.0)) {
            let assign19860_e20056: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign19860_e20057: f64 = (1.0 + assign19860_e20056);
            let assign19860_e20059: f64 = (-p.p833);
            let assign19860_e20061: f64 = (assign19860_e20059 * locals.var_one_over_one_minus_pgat);
            let assign19860_e20062: f64 = (assign19860_e20057).powf(assign19860_e20061);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign19860_e20062, if 0.0 == 0.0 && ((assign19860_e20061) as f64).is_finite() && ((assign19860_e20061) as f64).fract() == 0.0 { if assign19860_e20061 == 0.0 { 0.0 } else { (assign19860_e20061 * ((assign19860_e20057).powf(assign19860_e20061 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign19860_e20062 * (assign19860_e20061 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign19860_e20057))) }, if 0.0 == 0.0 && ((assign19860_e20061) as f64).is_finite() && ((assign19860_e20061) as f64).fract() == 0.0 { if assign19860_e20061 == 0.0 { 0.0 } else { (assign19860_e20061 * ((assign19860_e20057).powf(assign19860_e20061 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign19860_e20062 * (assign19860_e20061 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign19860_e20057))) }, if 0.0 == 0.0 && ((assign19860_e20061) as f64).is_finite() && ((assign19860_e20061) as f64).fract() == 0.0 { if assign19860_e20061 == 0.0 { 0.0 } else { (assign19860_e20061 * ((assign19860_e20057).powf(assign19860_e20061 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign19860_e20062 * (assign19860_e20061 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign19860_e20057))) }, if 0.0 == 0.0 && ((assign19860_e20061) as f64).is_finite() && ((assign19860_e20061) as f64).fract() == 0.0 { if assign19860_e20061 == 0.0 { 0.0 } else { (assign19860_e20061 * ((assign19860_e20057).powf(assign19860_e20061 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign19860_e20062 * (assign19860_e20061 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign19860_e20057))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19870_e20076: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign19870_e20079: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign19870_e20080: f64 = (assign19870_e20076 / assign19870_e20079);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign19870_e20080, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign19870_e20079) - (assign19870_e20076 * locals.var_wgamma_dn5)) / (assign19870_e20079 * assign19870_e20079)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign19870_e20079) - (assign19870_e20076 * locals.var_wgamma_dn6)) / (assign19870_e20079 * assign19870_e20079)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign19870_e20079) - (assign19870_e20076 * locals.var_wgamma_dn7)) / (assign19870_e20079 * assign19870_e20079)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign19870_e20079) - (assign19870_e20076 * locals.var_wgamma_dn8)) / (assign19870_e20079 * assign19870_e20079)), );
        }
    }
    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19880_e20095: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign19880_e20096: f64 = (0.375 * assign19880_e20095);
            let assign19880_e20097: f64 = (assign19880_e20096).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign19880_e20097, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign19880_e20097)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign19880_e20097)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign19880_e20097)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign19880_e20097)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19890_e20112: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign19890_e20113: f64 = (2.0 * assign19890_e20112);
            let assign19890_e20115: f64 = (assign19890_e20113 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign19890_e20115, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19900_e20129: f64 = (locals.var_atatgat * locals.var_twoatatoverthreebtat);
            let assign19900_e20131: f64 = (assign19900_e20129 * locals.var_sqrtumax);
            let assign19900_e20134: f64 = (locals.var_atatgat * locals.var_umax);
            let assign19900_e20135: f64 = (assign19900_e20131 - assign19900_e20134);
            let assign19900_e20139: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign19900_e20140: f64 = (0.5 * assign19900_e20139);
            let assign19900_e20141: f64 = (assign19900_e20135 + assign19900_e20140);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign19900_e20141, (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign19900_e20129 * locals.var_sqrtumax_dn5)) - (locals.var_atatgat * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign19900_e20129 * locals.var_sqrtumax_dn6)) - (locals.var_atatgat * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign19900_e20129 * locals.var_sqrtumax_dn7)) - (locals.var_atatgat * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign19900_e20129 * locals.var_sqrtumax_dn8)) - (locals.var_atatgat * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19910_e20155: f64 = (locals.var_ltat - 1.0);
            let assign19910_e20157: f64 = (assign19910_e20155 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign19910_e20157, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign19910_e20155 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign19910_e20155 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign19910_e20155 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign19910_e20155 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19920_e20171: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign19920_e20171, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign19930_e20176: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard349 = assign19930_e20176;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard349 != 0.0)) {
            let assign19940_e20192: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign19940_e20193: f64 = (1.0 + assign19940_e20192);
            let assign19940_e20194: f64 = (1.0 / assign19940_e20193);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign19940_e20194, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign19940_e20193 * assign19940_e20193))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign19940_e20193 * assign19940_e20193))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign19940_e20193 * assign19940_e20193))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign19940_e20193 * assign19940_e20193))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard349 == 0.0)) {
            let assign19950_e20213: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign19950_e20214: f64 = (1.0 - assign19950_e20213);
            let assign19950_e20215: f64 = (1.0 / assign19950_e20214);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign19950_e20215, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign19950_e20214 * assign19950_e20214))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign19950_e20214 * assign19950_e20214))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign19950_e20214 * assign19950_e20214))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign19950_e20214 * assign19950_e20214))), );
        }
        let assign19960_e20219: f64 = (-locals.var_ysq);
        let assign19960_e20221: f64 = (assign19960_e20219 + locals.var_mtat);
        let assign19960_e20223: f64 = (-230.25850929940458);
        let assign19960_e20224: f64 = if assign19960_e20221 > assign19960_e20223 { 1.0 } else { 0.0 };
        locals.var_guard350 = assign19960_e20224;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign19970_e20237: f64 = (-locals.var_ysq);
            let assign19970_e20239: f64 = (assign19970_e20237 + locals.var_mtat);
            let assign19970_e20240: f64 = (assign19970_e20239).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19970_e20240, (assign19970_e20240 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign19970_e20240 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign19970_e20240 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign19970_e20240 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard350 == 0.0)) {
            let assign19980_e20258: f64 = (-230.25850929940458);
            let assign19980_e20260: f64 = (-locals.var_ysq);
            let assign19980_e20262: f64 = (assign19980_e20260 + locals.var_mtat);
            let assign19980_e20263: f64 = (assign19980_e20258 - assign19980_e20262);
            let assign19980_e20267: f64 = (-230.25850929940458);
            let assign19980_e20269: f64 = (-locals.var_ysq);
            let assign19980_e20271: f64 = (assign19980_e20269 + locals.var_mtat);
            let assign19980_e20272: f64 = (assign19980_e20267 - assign19980_e20271);
            let assign19980_e20275: f64 = (-230.25850929940458);
            let assign19980_e20277: f64 = (-locals.var_ysq);
            let assign19980_e20279: f64 = (assign19980_e20277 + locals.var_mtat);
            let assign19980_e20280: f64 = (assign19980_e20275 - assign19980_e20279);
            let assign19980_e20282: f64 = (assign19980_e20280 * 0.3333333333333333);
            let assign19980_e20283: f64 = (1.0 + assign19980_e20282);
            let assign19980_e20284: f64 = (assign19980_e20272 * assign19980_e20283);
            let assign19980_e20285: f64 = (0.5 * assign19980_e20284);
            let assign19980_e20286: f64 = (1.0 + assign19980_e20285);
            let assign19980_e20287: f64 = (assign19980_e20263 * assign19980_e20286);
            let assign19980_e20288: f64 = (1.0 + assign19980_e20287);
            let assign19980_e20289: f64 = (1e-100 / assign19980_e20288);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign19980_e20289, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign19980_e20286) + (assign19980_e20263 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign19980_e20283) + (assign19980_e20272 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign19980_e20288 * assign19980_e20288))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign19980_e20286) + (assign19980_e20263 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign19980_e20283) + (assign19980_e20272 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign19980_e20288 * assign19980_e20288))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign19980_e20286) + (assign19980_e20263 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign19980_e20283) + (assign19980_e20272 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign19980_e20288 * assign19980_e20288))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign19980_e20286) + (assign19980_e20263 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign19980_e20283) + (assign19980_e20272 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign19980_e20288 * assign19980_e20288))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19990_e20303: f64 = (0.29214664 * locals.var_terfc);
            let assign19990_e20307: f64 = (locals.var_terfc * locals.var_terfc);
            let assign19990_e20308: f64 = (locals.var_berfc * assign19990_e20307);
            let assign19990_e20309: f64 = (assign19990_e20303 + assign19990_e20308);
            let assign19990_e20313: f64 = (locals.var_terfc * locals.var_terfc);
            let assign19990_e20315: f64 = (assign19990_e20313 * locals.var_terfc);
            let assign19990_e20316: f64 = (locals.var_cerfc * assign19990_e20315);
            let assign19990_e20317: f64 = (assign19990_e20309 + assign19990_e20316);
            let assign19990_e20319: f64 = (assign19990_e20317 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign19990_e20319, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign19990_e20313 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign19990_e20317 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign19990_e20313 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign19990_e20317 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign19990_e20313 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign19990_e20317 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign19990_e20313 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign19990_e20317 * locals.var_tmp_dn8)), );
        }
        let assign20000_e20324: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard351 = assign20000_e20324;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard351 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign20020_e20341: f64 = (-230.25850929940458);
        let assign20020_e20342: f64 = if locals.var_mtat > assign20020_e20341 { 1.0 } else { 0.0 };
        locals.var_guard352 = assign20020_e20342;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard351 == 0.0)) && (locals.var_guard352 != 0.0)) {
            let assign20030_e20358: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20030_e20358, (assign20030_e20358 * locals.var_mtat_dn5), (assign20030_e20358 * locals.var_mtat_dn6), (assign20030_e20358 * locals.var_mtat_dn7), (assign20030_e20358 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard351 == 0.0)) && (locals.var_guard352 == 0.0)) {
            let assign20040_e20379: f64 = (-230.25850929940458);
            let assign20040_e20381: f64 = (assign20040_e20379 - locals.var_mtat);
            let assign20040_e20385: f64 = (-230.25850929940458);
            let assign20040_e20387: f64 = (assign20040_e20385 - locals.var_mtat);
            let assign20040_e20390: f64 = (-230.25850929940458);
            let assign20040_e20392: f64 = (assign20040_e20390 - locals.var_mtat);
            let assign20040_e20394: f64 = (assign20040_e20392 * 0.3333333333333333);
            let assign20040_e20395: f64 = (1.0 + assign20040_e20394);
            let assign20040_e20396: f64 = (assign20040_e20387 * assign20040_e20395);
            let assign20040_e20397: f64 = (0.5 * assign20040_e20396);
            let assign20040_e20398: f64 = (1.0 + assign20040_e20397);
            let assign20040_e20399: f64 = (assign20040_e20381 * assign20040_e20398);
            let assign20040_e20400: f64 = (1.0 + assign20040_e20399);
            let assign20040_e20401: f64 = (1e-100 / assign20040_e20400);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20040_e20401, (-((1e-100 * (((-locals.var_mtat_dn5) * assign20040_e20398) + (assign20040_e20381 * (0.5 * (((-locals.var_mtat_dn5) * assign20040_e20395) + (assign20040_e20387 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign20040_e20400 * assign20040_e20400))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign20040_e20398) + (assign20040_e20381 * (0.5 * (((-locals.var_mtat_dn6) * assign20040_e20395) + (assign20040_e20387 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign20040_e20400 * assign20040_e20400))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign20040_e20398) + (assign20040_e20381 * (0.5 * (((-locals.var_mtat_dn7) * assign20040_e20395) + (assign20040_e20387 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign20040_e20400 * assign20040_e20400))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign20040_e20398) + (assign20040_e20381 * (0.5 * (((-locals.var_mtat_dn8) * assign20040_e20395) + (assign20040_e20387 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign20040_e20400 * assign20040_e20400))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard351 == 0.0)) {
            let assign20050_e20418: f64 = (2.0 * locals.var_tmp);
            let assign20050_e20420: f64 = (assign20050_e20418 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign20050_e20420, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign20060_e20434: f64 = (1.772453850905516 * 0.5);
            let assign20060_e20437: f64 = (locals.var_atatgat * locals.var_erfctimesexpmtat);
            let assign20060_e20439: f64 = (assign20060_e20437 / locals.var_ktat);
            let assign20060_e20440: f64 = (assign20060_e20434 * assign20060_e20439);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign20060_e20440, (assign20060_e20434 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign20060_e20437 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign20060_e20434 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign20060_e20437 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign20060_e20434 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign20060_e20437 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign20060_e20434 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign20060_e20437 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign20070_e20455: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign20070_e20457: f64 = (assign20070_e20455 * locals.var_wtat);
            let assign20070_e20458: f64 = (p.p847 * assign20070_e20457);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign20070_e20458, (p.p847 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign20070_e20455 * locals.var_wtat_dn5))), (p.p847 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign20070_e20455 * locals.var_wtat_dn6))), (p.p847 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign20070_e20455 * locals.var_wtat_dn7))), (p.p847 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign20070_e20455 * locals.var_wtat_dn8))), );
        }
        let assign20080_e20463: f64 = if p.p853 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard353 = assign20080_e20463;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard353 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign20100_e20477: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard354 = assign20100_e20477;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard353 == 0.0)) && (locals.var_guard354 != 0.0)) {
            let assign20110_e20491: f64 = (p.p830 - locals.var_vbbt);
            let assign20110_e20493: f64 = (assign20110_e20491 * locals.var_vbirgatinv);
            let assign20110_e20494: f64 = (assign20110_e20493).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20110_e20494, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard353 == 0.0)) && (locals.var_guard354 == 0.0)) {
            let assign20120_e20511: f64 = (p.p830 - locals.var_vbbt);
            let assign20120_e20513: f64 = (assign20120_e20511 * locals.var_vbirgatinv);
            let assign20120_e20515: f64 = (assign20120_e20513).powf(p.p833);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20120_e20515, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard353 == 0.0)) {
            let assign20130_e20530: f64 = (p.p830 - locals.var_vbbt);
            let assign20130_e20532: f64 = (assign20130_e20530 * locals.var_wdepnulrinvgat);
            let assign20130_e20534: f64 = (assign20130_e20532 / locals.var_tmp);
            let assign20130_e20535: f64 = (locals.var_one_over_one_minus_pgat * assign20130_e20534);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign20130_e20535, (locals.var_one_over_one_minus_pgat * (-((assign20130_e20532 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign20130_e20532 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign20130_e20532 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign20130_e20532 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign20140_e20539: f64 = (-locals.var_fbbtgat);
        let assign20140_e20541: f64 = (assign20140_e20539 / locals.var_fmaxr);
        let assign20140_e20542: f64 = (assign20140_e20541).abs();
        let assign20140_e20544: f64 = if assign20140_e20542 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard355 = assign20140_e20544;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard353 == 0.0)) && (locals.var_guard355 != 0.0)) {
            let assign20150_e20557: f64 = (-locals.var_fbbtgat);
            let assign20150_e20559: f64 = (assign20150_e20557 / locals.var_fmaxr);
            let assign20150_e20560: f64 = (assign20150_e20559).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20150_e20560, (assign20150_e20560 * ((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign20150_e20557 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign20150_e20560 * ((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign20150_e20557 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign20150_e20560 * ((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign20150_e20557 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign20150_e20560 * ((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign20150_e20557 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))), );
        }
        let assign20160_e20564: f64 = (-locals.var_fbbtgat);
        let assign20160_e20566: f64 = (assign20160_e20564 / locals.var_fmaxr);
        let assign20160_e20568: f64 = if assign20160_e20566 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard356 = assign20160_e20568;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard353 == 0.0)) && (locals.var_guard355 == 0.0)) && (locals.var_guard356 != 0.0)) {
            let assign20170_e20586: f64 = (-230.25850929940458);
            let assign20170_e20588: f64 = (-locals.var_fbbtgat);
            let assign20170_e20590: f64 = (assign20170_e20588 / locals.var_fmaxr);
            let assign20170_e20591: f64 = (assign20170_e20586 - assign20170_e20590);
            let assign20170_e20595: f64 = (-230.25850929940458);
            let assign20170_e20597: f64 = (-locals.var_fbbtgat);
            let assign20170_e20599: f64 = (assign20170_e20597 / locals.var_fmaxr);
            let assign20170_e20600: f64 = (assign20170_e20595 - assign20170_e20599);
            let assign20170_e20603: f64 = (-230.25850929940458);
            let assign20170_e20605: f64 = (-locals.var_fbbtgat);
            let assign20170_e20607: f64 = (assign20170_e20605 / locals.var_fmaxr);
            let assign20170_e20608: f64 = (assign20170_e20603 - assign20170_e20607);
            let assign20170_e20610: f64 = (assign20170_e20608 * 0.3333333333333333);
            let assign20170_e20611: f64 = (1.0 + assign20170_e20610);
            let assign20170_e20612: f64 = (assign20170_e20600 * assign20170_e20611);
            let assign20170_e20613: f64 = (0.5 * assign20170_e20612);
            let assign20170_e20614: f64 = (1.0 + assign20170_e20613);
            let assign20170_e20615: f64 = (assign20170_e20591 * assign20170_e20614);
            let assign20170_e20616: f64 = (1.0 + assign20170_e20615);
            let assign20170_e20617: f64 = (1e-100 / assign20170_e20616);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20170_e20617, (-((1e-100 * (((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign20170_e20588 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign20170_e20614) + (assign20170_e20591 * (0.5 * (((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign20170_e20597 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign20170_e20611) + (assign20170_e20600 * ((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign20170_e20605 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign20170_e20616 * assign20170_e20616))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign20170_e20588 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign20170_e20614) + (assign20170_e20591 * (0.5 * (((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign20170_e20597 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign20170_e20611) + (assign20170_e20600 * ((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign20170_e20605 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign20170_e20616 * assign20170_e20616))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign20170_e20588 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign20170_e20614) + (assign20170_e20591 * (0.5 * (((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign20170_e20597 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign20170_e20611) + (assign20170_e20600 * ((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign20170_e20605 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign20170_e20616 * assign20170_e20616))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign20170_e20588 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign20170_e20614) + (assign20170_e20591 * (0.5 * (((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign20170_e20597 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign20170_e20611) + (assign20170_e20600 * ((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign20170_e20605 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign20170_e20616 * assign20170_e20616))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard353 == 0.0)) && (locals.var_guard355 == 0.0)) && (locals.var_guard356 == 0.0)) {
            let assign20180_e20638: f64 = (-locals.var_fbbtgat);
            let assign20180_e20640: f64 = (assign20180_e20638 / locals.var_fmaxr);
            let assign20180_e20642: f64 = (assign20180_e20640 - 230.25850929940458);
            let assign20180_e20646: f64 = (-locals.var_fbbtgat);
            let assign20180_e20648: f64 = (assign20180_e20646 / locals.var_fmaxr);
            let assign20180_e20650: f64 = (assign20180_e20648 - 230.25850929940458);
            let assign20180_e20653: f64 = (-locals.var_fbbtgat);
            let assign20180_e20655: f64 = (assign20180_e20653 / locals.var_fmaxr);
            let assign20180_e20657: f64 = (assign20180_e20655 - 230.25850929940458);
            let assign20180_e20659: f64 = (assign20180_e20657 * 0.3333333333333333);
            let assign20180_e20660: f64 = (1.0 + assign20180_e20659);
            let assign20180_e20661: f64 = (assign20180_e20650 * assign20180_e20660);
            let assign20180_e20662: f64 = (0.5 * assign20180_e20661);
            let assign20180_e20663: f64 = (1.0 + assign20180_e20662);
            let assign20180_e20664: f64 = (assign20180_e20642 * assign20180_e20663);
            let assign20180_e20665: f64 = (1.0 + assign20180_e20664);
            let assign20180_e20666: f64 = (1e100 * assign20180_e20665);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20180_e20666, (1e100 * ((((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign20180_e20638 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign20180_e20663) + (assign20180_e20642 * (0.5 * ((((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign20180_e20646 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign20180_e20660) + (assign20180_e20650 * (((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign20180_e20653 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign20180_e20638 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign20180_e20663) + (assign20180_e20642 * (0.5 * ((((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign20180_e20646 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign20180_e20660) + (assign20180_e20650 * (((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign20180_e20653 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign20180_e20638 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign20180_e20663) + (assign20180_e20642 * (0.5 * ((((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign20180_e20646 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign20180_e20660) + (assign20180_e20650 * (((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign20180_e20653 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign20180_e20638 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign20180_e20663) + (assign20180_e20642 * (0.5 * ((((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign20180_e20646 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign20180_e20660) + (assign20180_e20650 * (((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign20180_e20653 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard353 == 0.0)) {
            let assign20190_e20681: f64 = (locals.var_v2 * locals.var_fmaxr);
            let assign20190_e20683: f64 = (assign20190_e20681 * locals.var_fmaxr);
            let assign20190_e20685: f64 = (assign20190_e20683 * locals.var_tmp);
            let assign20190_e20686: f64 = (p.p853 * assign20190_e20685);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign20190_e20686, (p.p853 * (((((locals.var_v2 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign20190_e20681 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign20190_e20683 * locals.var_tmp_dn5))), (p.p853 * (((((locals.var_v2 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign20190_e20681 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign20190_e20683 * locals.var_tmp_dn6))), (p.p853 * (((((locals.var_v2 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign20190_e20681 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign20190_e20683 * locals.var_tmp_dn7))), (p.p853 * (((((locals.var_v2 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign20190_e20681 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign20190_e20683 * locals.var_tmp_dn8))), );
        }
        let assign20200_e20691: f64 = if p.p862 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard357 = assign20200_e20691;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard357 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign20220_e20705: f64 = (-locals.var_alphaav);
        let assign20220_e20707: f64 = (assign20220_e20705 * p.p862);
        let assign20220_e20708: f64 = if locals.var_vav > assign20220_e20707 { 1.0 } else { 0.0 };
        locals.var_guard358 = assign20220_e20708;
        let assign20230_e20711: f64 = if p.p865 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard359 = assign20230_e20711;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard357 == 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard359 != 0.0)) {
            let assign20240_e20727: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign20240_e20730: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign20240_e20731: f64 = (assign20240_e20727 * assign20240_e20730);
            let assign20240_e20734: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign20240_e20735: f64 = (assign20240_e20731 * assign20240_e20734);
            let assign20240_e20738: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign20240_e20739: f64 = (assign20240_e20735 * assign20240_e20738);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20240_e20739, (((((((locals.var_vav * locals.var_vbrinvgat_dn5) * assign20240_e20730) + (assign20240_e20727 * (locals.var_vav * locals.var_vbrinvgat_dn5))) * assign20240_e20734) + (assign20240_e20731 * (locals.var_vav * locals.var_vbrinvgat_dn5))) * assign20240_e20738) + (assign20240_e20735 * (locals.var_vav * locals.var_vbrinvgat_dn5))), (((((((locals.var_vav * locals.var_vbrinvgat_dn6) * assign20240_e20730) + (assign20240_e20727 * (locals.var_vav * locals.var_vbrinvgat_dn6))) * assign20240_e20734) + (assign20240_e20731 * (locals.var_vav * locals.var_vbrinvgat_dn6))) * assign20240_e20738) + (assign20240_e20735 * (locals.var_vav * locals.var_vbrinvgat_dn6))), (((((((locals.var_vav * locals.var_vbrinvgat_dn7) * assign20240_e20730) + (assign20240_e20727 * (locals.var_vav * locals.var_vbrinvgat_dn7))) * assign20240_e20734) + (assign20240_e20731 * (locals.var_vav * locals.var_vbrinvgat_dn7))) * assign20240_e20738) + (assign20240_e20735 * (locals.var_vav * locals.var_vbrinvgat_dn7))), (((((((locals.var_vav * locals.var_vbrinvgat_dn8) * assign20240_e20730) + (assign20240_e20727 * (locals.var_vav * locals.var_vbrinvgat_dn8))) * assign20240_e20734) + (assign20240_e20731 * (locals.var_vav * locals.var_vbrinvgat_dn8))) * assign20240_e20738) + (assign20240_e20735 * (locals.var_vav * locals.var_vbrinvgat_dn8))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard357 == 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard359 == 0.0)) {
            let assign20250_e20758: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign20250_e20759: f64 = (assign20250_e20758).abs();
            let assign20250_e20761: f64 = (assign20250_e20759).powf(p.p865);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20250_e20761, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign20250_e20759).powf(p.p865 - 1.0) * if assign20250_e20758 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn5)) })) } } else { (assign20250_e20761 * (p.p865 * (if assign20250_e20758 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn5)) } / assign20250_e20759))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign20250_e20759).powf(p.p865 - 1.0) * if assign20250_e20758 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn6)) })) } } else { (assign20250_e20761 * (p.p865 * (if assign20250_e20758 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn6)) } / assign20250_e20759))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign20250_e20759).powf(p.p865 - 1.0) * if assign20250_e20758 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn7)) })) } } else { (assign20250_e20761 * (p.p865 * (if assign20250_e20758 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn7)) } / assign20250_e20759))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign20250_e20759).powf(p.p865 - 1.0) * if assign20250_e20758 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn8)) })) } } else { (assign20250_e20761 * (p.p865 * (if assign20250_e20758 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn8)) } / assign20250_e20759))) }, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard357 == 0.0)) && (locals.var_guard358 != 0.0)) {
            let assign20260_e20778: f64 = (1.0 - locals.var_tmp);
            let assign20260_e20779: f64 = (1.0 / assign20260_e20778);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign20260_e20779, (-((-locals.var_tmp_dn5) / (assign20260_e20778 * assign20260_e20778))), (-((-locals.var_tmp_dn6) / (assign20260_e20778 * assign20260_e20778))), (-((-locals.var_tmp_dn7) / (assign20260_e20778 * assign20260_e20778))), (-((-locals.var_tmp_dn8) / (assign20260_e20778 * assign20260_e20778))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard357 == 0.0)) && (locals.var_guard358 == 0.0)) {
            let assign20270_e20798: f64 = (locals.var_alphaav * p.p862);
            let assign20270_e20799: f64 = (locals.var_vav + assign20270_e20798);
            let assign20270_e20801: f64 = (assign20270_e20799 * locals.var_slopegat);
            let assign20270_e20802: f64 = (locals.var_fstopgat + assign20270_e20801);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign20270_e20802, (assign20270_e20799 * locals.var_slopegat_dn5), (assign20270_e20799 * locals.var_slopegat_dn6), (assign20270_e20799 * locals.var_slopegat_dn7), (assign20270_e20799 * locals.var_slopegat_dn8), );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard343 == 0.0)) {
            let assign20280_e20814: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign20280_e20816: f64 = (assign20280_e20814 + locals.var_itat);
            let assign20280_e20818: f64 = (assign20280_e20816 + locals.var_ibbt);
            let assign20280_e20819: f64 = (p.p29 * assign20280_e20818);
            let assign20280_e20821: f64 = (assign20280_e20819 * locals.var_fbreakdown);
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (assign20280_e20821, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign20280_e20819 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign20280_e20819 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign20280_e20819 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign20280_e20819 * locals.var_fbreakdown_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign20290_e20829: f64 = (locals.var_absource_i * locals.var_ijunbot);
            let assign20290_e20832: f64 = (locals.var_lssource_i * locals.var_ijunsti);
            let assign20290_e20833: f64 = (assign20290_e20829 + assign20290_e20832);
            let assign20290_e20836: f64 = (locals.var_lgsource_i * locals.var_ijungat);
            let assign20290_e20837: f64 = (assign20290_e20833 + assign20290_e20836);
            (locals.var_i2, locals.var_i2_dn5, locals.var_i2_dn6, locals.var_i2_dn7, locals.var_i2_dn8, ) = (assign20290_e20837, (((locals.var_absource_i * locals.var_ijunbot_dn5) + (locals.var_lssource_i * locals.var_ijunsti_dn5)) + (locals.var_lgsource_i * locals.var_ijungat_dn5)), (((locals.var_absource_i * locals.var_ijunbot_dn6) + (locals.var_lssource_i * locals.var_ijunsti_dn6)) + (locals.var_lgsource_i * locals.var_ijungat_dn6)), (((locals.var_absource_i * locals.var_ijunbot_dn7) + (locals.var_lssource_i * locals.var_ijunsti_dn7)) + (locals.var_lgsource_i * locals.var_ijungat_dn7)), (((locals.var_absource_i * locals.var_ijunbot_dn8) + (locals.var_lssource_i * locals.var_ijunsti_dn8)) + (locals.var_lgsource_i * locals.var_ijungat_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            locals.var_vbbt = 0.0;
            locals.var_two_psistar = 0.0;
        }
        let assign20320_e20863: f64 = if (!(((locals.var_absource_i == 0.0) && (locals.var_lssource_i == 0.0)) && (locals.var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard360 = assign20320_e20863;
        let assign20400_e20949: f64 = if locals.var_v3 < locals.var_vmax_s { 1.0 } else { 0.0 };
        locals.var_guard361 = assign20400_e20949;
        let assign20410_e20951: f64 = (-0.5);
        let assign20410_e20954: f64 = (locals.var_v3 * locals.var_phitdinv);
        let assign20410_e20955: f64 = (assign20410_e20951 * assign20410_e20954);
        let assign20410_e20956: f64 = (assign20410_e20955).abs();
        let assign20410_e20958: f64 = if assign20410_e20956 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard362 = assign20410_e20958;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) && (locals.var_guard361 != 0.0)) && (locals.var_guard362 != 0.0)) {
            let assign20420_e20969: f64 = (-0.5);
            let assign20420_e20972: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign20420_e20973: f64 = (assign20420_e20969 * assign20420_e20972);
            let assign20420_e20974: f64 = (assign20420_e20973).exp();
            locals.var_z = assign20420_e20974;
        }
        let assign20430_e20978: f64 = (-0.5);
        let assign20430_e20981: f64 = (locals.var_v3 * locals.var_phitdinv);
        let assign20430_e20982: f64 = (assign20430_e20978 * assign20430_e20981);
        let assign20430_e20984: f64 = if assign20430_e20982 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard363 = assign20430_e20984;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) && (locals.var_guard361 != 0.0)) && (locals.var_guard362 == 0.0)) && (locals.var_guard363 != 0.0)) {
            let assign20440_e21000: f64 = (-230.25850929940458);
            let assign20440_e21002: f64 = (-0.5);
            let assign20440_e21005: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign20440_e21006: f64 = (assign20440_e21002 * assign20440_e21005);
            let assign20440_e21007: f64 = (assign20440_e21000 - assign20440_e21006);
            let assign20440_e21011: f64 = (-230.25850929940458);
            let assign20440_e21013: f64 = (-0.5);
            let assign20440_e21016: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign20440_e21017: f64 = (assign20440_e21013 * assign20440_e21016);
            let assign20440_e21018: f64 = (assign20440_e21011 - assign20440_e21017);
            let assign20440_e21021: f64 = (-230.25850929940458);
            let assign20440_e21023: f64 = (-0.5);
            let assign20440_e21026: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign20440_e21027: f64 = (assign20440_e21023 * assign20440_e21026);
            let assign20440_e21028: f64 = (assign20440_e21021 - assign20440_e21027);
            let assign20440_e21030: f64 = (assign20440_e21028 * 0.3333333333333333);
            let assign20440_e21031: f64 = (1.0 + assign20440_e21030);
            let assign20440_e21032: f64 = (assign20440_e21018 * assign20440_e21031);
            let assign20440_e21033: f64 = (0.5 * assign20440_e21032);
            let assign20440_e21034: f64 = (1.0 + assign20440_e21033);
            let assign20440_e21035: f64 = (assign20440_e21007 * assign20440_e21034);
            let assign20440_e21036: f64 = (1.0 + assign20440_e21035);
            let assign20440_e21037: f64 = (1e-100 / assign20440_e21036);
            locals.var_z = assign20440_e21037;
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) && (locals.var_guard361 != 0.0)) && (locals.var_guard362 == 0.0)) && (locals.var_guard363 == 0.0)) {
            let assign20450_e21056: f64 = (-0.5);
            let assign20450_e21059: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign20450_e21060: f64 = (assign20450_e21056 * assign20450_e21059);
            let assign20450_e21062: f64 = (assign20450_e21060 - 230.25850929940458);
            let assign20450_e21066: f64 = (-0.5);
            let assign20450_e21069: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign20450_e21070: f64 = (assign20450_e21066 * assign20450_e21069);
            let assign20450_e21072: f64 = (assign20450_e21070 - 230.25850929940458);
            let assign20450_e21075: f64 = (-0.5);
            let assign20450_e21078: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign20450_e21079: f64 = (assign20450_e21075 * assign20450_e21078);
            let assign20450_e21081: f64 = (assign20450_e21079 - 230.25850929940458);
            let assign20450_e21083: f64 = (assign20450_e21081 * 0.3333333333333333);
            let assign20450_e21084: f64 = (1.0 + assign20450_e21083);
            let assign20450_e21085: f64 = (assign20450_e21072 * assign20450_e21084);
            let assign20450_e21086: f64 = (0.5 * assign20450_e21085);
            let assign20450_e21087: f64 = (1.0 + assign20450_e21086);
            let assign20450_e21088: f64 = (assign20450_e21062 * assign20450_e21087);
            let assign20450_e21089: f64 = (1.0 + assign20450_e21088);
            let assign20450_e21090: f64 = (1e100 * assign20450_e21089);
            locals.var_z = assign20450_e21090;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) && (locals.var_guard361 != 0.0)) {
            let assign20460_e21102: f64 = (1.0 / locals.var_z);
            locals.var_zinv = assign20460_e21102;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) && (locals.var_guard361 != 0.0)) {
            let assign20470_e21114: f64 = (locals.var_zinv * locals.var_zinv);
            locals.var_idmult = assign20470_e21114;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) && (locals.var_guard361 == 0.0)) {
            let assign20480_e21128: f64 = (locals.var_v3 - locals.var_vmax_s);
            let assign20480_e21130: f64 = (assign20480_e21128 * locals.var_phitdinv);
            let assign20480_e21131: f64 = (1.0 + assign20480_e21130);
            let assign20480_e21133: f64 = (assign20480_e21131 * locals.var_exp_vmax_over_phitd_s);
            locals.var_idmult = assign20480_e21133;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) && (locals.var_guard361 == 0.0)) {
            let assign20490_e21145: f64 = (locals.var_idmult).sqrt();
            locals.var_zinv = assign20490_e21145;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) && (locals.var_guard361 == 0.0)) {
            let assign20500_e21158: f64 = (1.0 / locals.var_zinv);
            locals.var_z = assign20500_e21158;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) {
            let assign20510_e21168: f64 = (locals.var_idmult - 1.0);
            locals.var_idmult = assign20510_e21168;
        }
        let assign20520_e21173: f64 = if locals.var_v3 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard364 = assign20520_e21173;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) && (locals.var_guard364 != 0.0)) {
            let assign20530_e21185: f64 = (2.0 + locals.var_z);
            let assign20530_e21188: f64 = (locals.var_z + 1.0);
            let assign20530_e21191: f64 = (locals.var_z + 3.0);
            let assign20530_e21192: f64 = (assign20530_e21188 * assign20530_e21191);
            let assign20530_e21193: f64 = (assign20530_e21192).sqrt();
            let assign20530_e21194: f64 = (assign20530_e21185 + assign20530_e21193);
            let assign20530_e21195: f64 = (assign20530_e21194).ln();
            let assign20530_e21196: f64 = (locals.var_phitd * assign20530_e21195);
            let assign20530_e21197: f64 = (2.0 * assign20530_e21196);
            locals.var_two_psistar = assign20530_e21197;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) && (locals.var_guard364 == 0.0)) {
            let assign20540_e21209: f64 = (-locals.var_v3);
            let assign20540_e21214: f64 = (2.0 * locals.var_zinv);
            let assign20540_e21216: f64 = (assign20540_e21214 + 1.0);
            let assign20540_e21219: f64 = (1.0 + locals.var_zinv);
            let assign20540_e21223: f64 = (3.0 * locals.var_zinv);
            let assign20540_e21224: f64 = (1.0 + assign20540_e21223);
            let assign20540_e21225: f64 = (assign20540_e21219 * assign20540_e21224);
            let assign20540_e21226: f64 = (assign20540_e21225).sqrt();
            let assign20540_e21227: f64 = (assign20540_e21216 + assign20540_e21226);
            let assign20540_e21228: f64 = (assign20540_e21227).ln();
            let assign20540_e21229: f64 = (locals.var_phitd * assign20540_e21228);
            let assign20540_e21230: f64 = (2.0 * assign20540_e21229);
            let assign20540_e21231: f64 = (assign20540_e21209 + assign20540_e21230);
            locals.var_two_psistar = assign20540_e21231;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) {
            let assign20550_e21241: f64 = (locals.var_vbimin_s - locals.var_two_psistar);
            locals.var_vjlim = assign20550_e21241;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) {
            let assign20560_e21252: f64 = (locals.var_v3 + locals.var_vjlim);
            let assign20560_e21255: f64 = (locals.var_v3 - locals.var_vjlim);
            let assign20560_e21258: f64 = (locals.var_v3 - locals.var_vjlim);
            let assign20560_e21259: f64 = (assign20560_e21255 * assign20560_e21258);
            let assign20560_e21262: f64 = (4.0 * locals.var_phitd);
            let assign20560_e21264: f64 = (assign20560_e21262 * locals.var_phitd);
            let assign20560_e21265: f64 = (assign20560_e21259 + assign20560_e21264);
            let assign20560_e21266: f64 = (assign20560_e21265).sqrt();
            let assign20560_e21267: f64 = (assign20560_e21252 - assign20560_e21266);
            let assign20560_e21268: f64 = (0.5 * assign20560_e21267);
            locals.var_vjsrh = assign20560_e21268;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) {
            let assign20570_e21279: f64 = (locals.var_v3 + locals.var_vbbtlim_s);
            let assign20570_e21282: f64 = (locals.var_v3 - locals.var_vbbtlim_s);
            let assign20570_e21285: f64 = (locals.var_v3 - locals.var_vbbtlim_s);
            let assign20570_e21286: f64 = (assign20570_e21282 * assign20570_e21285);
            let assign20570_e21289: f64 = (4.0 * locals.var_phitr);
            let assign20570_e21291: f64 = (assign20570_e21289 * locals.var_phitr);
            let assign20570_e21292: f64 = (assign20570_e21286 + assign20570_e21291);
            let assign20570_e21293: f64 = (assign20570_e21292).sqrt();
            let assign20570_e21294: f64 = (assign20570_e21279 - assign20570_e21293);
            let assign20570_e21295: f64 = (0.5 * assign20570_e21294);
            locals.var_vbbt = assign20570_e21295;
        }
    }
    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard360 != 0.0)) {
            let assign20580_e21306: f64 = locals.var_v3;
            let assign20580_e21309: f64 = locals.var_v3;
            let assign20580_e21312: f64 = locals.var_v3;
            let assign20580_e21313: f64 = (assign20580_e21309 * assign20580_e21312);
            let assign20580_e21316: f64 = (4.0 * 1e-6);
            let assign20580_e21318: f64 = (assign20580_e21316 * 1e-6);
            let assign20580_e21319: f64 = (assign20580_e21313 + assign20580_e21318);
            let assign20580_e21320: f64 = (assign20580_e21319).sqrt();
            let assign20580_e21321: f64 = (assign20580_e21306 - assign20580_e21320);
            let assign20580_e21322: f64 = (0.5 * assign20580_e21321);
            locals.var_vav = assign20580_e21322;
        }
        let assign20590_e21327: f64 = if locals.var_absource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard365 = assign20590_e21327;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 != 0.0)) {
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) {
            let assign20610_e21344: f64 = (locals.var_idsatbot * locals.var_idmult);
            locals.var_id__blk219 = assign20610_e21344;
        }
        let assign20620_e21353: f64 = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard366 = assign20620_e21353;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard366 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard366 == 0.0)) {
            let assign20640_e21376: f64 = (locals.var_vbibot - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign20640_e21376;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard366 == 0.0)) {
            let assign20650_e21392: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign20650_e21393: f64 = (1.0 - assign20650_e21392);
            let assign20650_e21394: f64 = (assign20650_e21393).sqrt();
            let assign20650_e21395: f64 = (1.0 - assign20650_e21394);
            locals.var_wsrhstep = assign20650_e21395;
        }
        let assign20660_e21400: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard367 = assign20660_e21400;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard366 == 0.0)) && (locals.var_guard367 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard366 == 0.0)) && (locals.var_guard367 == 0.0)) {
            let assign20680_e21429: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign20680_e21431: f64 = (locals.var_wsrhstep).ln();
            let assign20680_e21432: f64 = (assign20680_e21429 * assign20680_e21431);
            let assign20680_e21435: f64 = (1.0 - locals.var_wsrhstep);
            let assign20680_e21436: f64 = (assign20680_e21432 / assign20680_e21435);
            let assign20680_e21438: f64 = (assign20680_e21436 + locals.var_wsrhstep);
            let assign20680_e21442: f64 = (2.0 * p.p831);
            let assign20680_e21443: f64 = (1.0 - assign20680_e21442);
            let assign20680_e21444: f64 = (assign20680_e21438 * assign20680_e21443);
            locals.var_dwsrh = assign20680_e21444;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard366 == 0.0)) {
            let assign20690_e21458: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign20690_e21458;
        }
        let assign20700_e21463: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard368 = assign20700_e21463;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard366 == 0.0)) && (locals.var_guard368 != 0.0)) {
            let assign20710_e21477: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv);
            let assign20710_e21478: f64 = (assign20710_e21477).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20710_e21478, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard366 == 0.0)) && (locals.var_guard368 == 0.0)) {
            let assign20720_e21495: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv);
            let assign20720_e21497: f64 = (assign20720_e21495).powf(p.p831);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20720_e21497, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard366 == 0.0)) {
            let assign20730_e21511: f64 = (locals.var_wdepnulrbot * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign20730_e21511, (locals.var_wdepnulrbot * locals.var_tmp_dn5), (locals.var_wdepnulrbot * locals.var_tmp_dn6), (locals.var_wdepnulrbot * locals.var_tmp_dn7), (locals.var_wdepnulrbot * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard366 == 0.0)) {
            let assign20740_e21526: f64 = (locals.var_zinv - 1.0);
            let assign20740_e21528: f64 = (assign20740_e21526 * locals.var_wdep);
            let assign20740_e21529: f64 = (locals.var_ftdbot * assign20740_e21528);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign20740_e21529, (locals.var_ftdbot * (assign20740_e21526 * locals.var_wdep_dn5)), (locals.var_ftdbot * (assign20740_e21526 * locals.var_wdep_dn6)), (locals.var_ftdbot * (assign20740_e21526 * locals.var_wdep_dn7)), (locals.var_ftdbot * (assign20740_e21526 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard366 == 0.0)) {
            let assign20750_e21544: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign20750_e21545: f64 = (p.p840 * assign20750_e21544);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign20750_e21545, (p.p840 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign20760_e21550: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign20760_e21550;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20780_e21574: f64 = (locals.var_wdep * locals.var_one_minus_pbot);
            let assign20780_e21576: f64 = (assign20780_e21574 / locals.var_vbi_minus_vjsrh);
            let assign20780_e21577: f64 = (locals.var_btatpartbot * assign20780_e21576);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign20780_e21577, (locals.var_btatpartbot * ((locals.var_wdep_dn5 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn6 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn7 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn8 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20790_e21591: f64 = (0.666666666666667 * locals.var_atatbot);
            let assign20790_e21593: f64 = (assign20790_e21591 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign20790_e21593, (-((assign20790_e21591 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign20790_e21591 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign20790_e21591 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign20790_e21591 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20800_e21607: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign20800_e21607, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20810_e21621: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign20810_e21624: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign20810_e21626: f64 = (assign20810_e21624 + 1.0);
            let assign20810_e21627: f64 = (assign20810_e21621 / assign20810_e21626);
            let assign20810_e21628: f64 = (assign20810_e21627).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign20810_e21628, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign20810_e21626) - (assign20810_e21621 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign20810_e21626 * assign20810_e21626)) / (2.0 * assign20810_e21628)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign20810_e21626) - (assign20810_e21621 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign20810_e21626 * assign20810_e21626)) / (2.0 * assign20810_e21628)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign20810_e21626) - (assign20810_e21621 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign20810_e21626 * assign20810_e21626)) / (2.0 * assign20810_e21628)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign20810_e21626) - (assign20810_e21621 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign20810_e21626 * assign20810_e21626)) / (2.0 * assign20810_e21628)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20820_e21641: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign20820_e21641, (locals.var_umax_dn5 / (2.0 * assign20820_e21641)), (locals.var_umax_dn6 / (2.0 * assign20820_e21641)), (locals.var_umax_dn7 / (2.0 * assign20820_e21641)), (locals.var_umax_dn8 / (2.0 * assign20820_e21641)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20830_e21655: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign20830_e21655, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign20840_e21659: f64 = (-p.p831);
        let assign20840_e21661: f64 = (assign20840_e21659 * locals.var_one_over_one_minus_pbot);
        let assign20840_e21663: f64 = (-1.0);
        let assign20840_e21664: f64 = if assign20840_e21661 == assign20840_e21663 { 1.0 } else { 0.0 };
        locals.var_guard370 = assign20840_e21664;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard370 != 0.0)) {
            let assign20850_e21680: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign20850_e21681: f64 = (1.0 + assign20850_e21680);
            let assign20850_e21682: f64 = (1.0 / assign20850_e21681);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign20850_e21682, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign20850_e21681 * assign20850_e21681))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign20850_e21681 * assign20850_e21681))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign20850_e21681 * assign20850_e21681))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign20850_e21681 * assign20850_e21681))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard370 == 0.0)) {
            let assign20860_e21700: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign20860_e21701: f64 = (1.0 + assign20860_e21700);
            let assign20860_e21703: f64 = (-p.p831);
            let assign20860_e21705: f64 = (assign20860_e21703 * locals.var_one_over_one_minus_pbot);
            let assign20860_e21706: f64 = (assign20860_e21701).powf(assign20860_e21705);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign20860_e21706, if 0.0 == 0.0 && ((assign20860_e21705) as f64).is_finite() && ((assign20860_e21705) as f64).fract() == 0.0 { if assign20860_e21705 == 0.0 { 0.0 } else { (assign20860_e21705 * ((assign20860_e21701).powf(assign20860_e21705 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign20860_e21706 * (assign20860_e21705 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign20860_e21701))) }, if 0.0 == 0.0 && ((assign20860_e21705) as f64).is_finite() && ((assign20860_e21705) as f64).fract() == 0.0 { if assign20860_e21705 == 0.0 { 0.0 } else { (assign20860_e21705 * ((assign20860_e21701).powf(assign20860_e21705 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign20860_e21706 * (assign20860_e21705 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign20860_e21701))) }, if 0.0 == 0.0 && ((assign20860_e21705) as f64).is_finite() && ((assign20860_e21705) as f64).fract() == 0.0 { if assign20860_e21705 == 0.0 { 0.0 } else { (assign20860_e21705 * ((assign20860_e21701).powf(assign20860_e21705 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign20860_e21706 * (assign20860_e21705 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign20860_e21701))) }, if 0.0 == 0.0 && ((assign20860_e21705) as f64).is_finite() && ((assign20860_e21705) as f64).fract() == 0.0 { if assign20860_e21705 == 0.0 { 0.0 } else { (assign20860_e21705 * ((assign20860_e21701).powf(assign20860_e21705 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign20860_e21706 * (assign20860_e21705 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign20860_e21701))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20870_e21720: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign20870_e21723: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign20870_e21724: f64 = (assign20870_e21720 / assign20870_e21723);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign20870_e21724, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign20870_e21723) - (assign20870_e21720 * locals.var_wgamma_dn5)) / (assign20870_e21723 * assign20870_e21723)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign20870_e21723) - (assign20870_e21720 * locals.var_wgamma_dn6)) / (assign20870_e21723 * assign20870_e21723)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign20870_e21723) - (assign20870_e21720 * locals.var_wgamma_dn7)) / (assign20870_e21723 * assign20870_e21723)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign20870_e21723) - (assign20870_e21720 * locals.var_wgamma_dn8)) / (assign20870_e21723 * assign20870_e21723)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20880_e21739: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign20880_e21740: f64 = (0.375 * assign20880_e21739);
            let assign20880_e21741: f64 = (assign20880_e21740).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign20880_e21741, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign20880_e21741)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign20880_e21741)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign20880_e21741)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign20880_e21741)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20890_e21756: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign20890_e21757: f64 = (2.0 * assign20890_e21756);
            let assign20890_e21759: f64 = (assign20890_e21757 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign20890_e21759, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20900_e21773: f64 = (locals.var_atatbot * locals.var_twoatatoverthreebtat);
            let assign20900_e21775: f64 = (assign20900_e21773 * locals.var_sqrtumax);
            let assign20900_e21778: f64 = (locals.var_atatbot * locals.var_umax);
            let assign20900_e21779: f64 = (assign20900_e21775 - assign20900_e21778);
            let assign20900_e21783: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign20900_e21784: f64 = (0.5 * assign20900_e21783);
            let assign20900_e21785: f64 = (assign20900_e21779 + assign20900_e21784);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign20900_e21785, (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign20900_e21773 * locals.var_sqrtumax_dn5)) - (locals.var_atatbot * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign20900_e21773 * locals.var_sqrtumax_dn6)) - (locals.var_atatbot * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign20900_e21773 * locals.var_sqrtumax_dn7)) - (locals.var_atatbot * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign20900_e21773 * locals.var_sqrtumax_dn8)) - (locals.var_atatbot * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20910_e21799: f64 = (locals.var_ltat - 1.0);
            let assign20910_e21801: f64 = (assign20910_e21799 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign20910_e21801, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign20910_e21799 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign20910_e21799 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign20910_e21799 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign20910_e21799 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20920_e21815: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign20920_e21815, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign20930_e21820: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard371 = assign20930_e21820;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard371 != 0.0)) {
            let assign20940_e21836: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign20940_e21837: f64 = (1.0 + assign20940_e21836);
            let assign20940_e21838: f64 = (1.0 / assign20940_e21837);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign20940_e21838, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign20940_e21837 * assign20940_e21837))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign20940_e21837 * assign20940_e21837))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign20940_e21837 * assign20940_e21837))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign20940_e21837 * assign20940_e21837))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard371 == 0.0)) {
            let assign20950_e21857: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign20950_e21858: f64 = (1.0 - assign20950_e21857);
            let assign20950_e21859: f64 = (1.0 / assign20950_e21858);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign20950_e21859, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign20950_e21858 * assign20950_e21858))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign20950_e21858 * assign20950_e21858))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign20950_e21858 * assign20950_e21858))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign20950_e21858 * assign20950_e21858))), );
        }
        let assign20960_e21863: f64 = (-locals.var_ysq);
        let assign20960_e21865: f64 = (assign20960_e21863 + locals.var_mtat);
        let assign20960_e21867: f64 = (-230.25850929940458);
        let assign20960_e21868: f64 = if assign20960_e21865 > assign20960_e21867 { 1.0 } else { 0.0 };
        locals.var_guard372 = assign20960_e21868;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard372 != 0.0)) {
            let assign20970_e21881: f64 = (-locals.var_ysq);
            let assign20970_e21883: f64 = (assign20970_e21881 + locals.var_mtat);
            let assign20970_e21884: f64 = (assign20970_e21883).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20970_e21884, (assign20970_e21884 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign20970_e21884 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign20970_e21884 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign20970_e21884 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard372 == 0.0)) {
            let assign20980_e21902: f64 = (-230.25850929940458);
            let assign20980_e21904: f64 = (-locals.var_ysq);
            let assign20980_e21906: f64 = (assign20980_e21904 + locals.var_mtat);
            let assign20980_e21907: f64 = (assign20980_e21902 - assign20980_e21906);
            let assign20980_e21911: f64 = (-230.25850929940458);
            let assign20980_e21913: f64 = (-locals.var_ysq);
            let assign20980_e21915: f64 = (assign20980_e21913 + locals.var_mtat);
            let assign20980_e21916: f64 = (assign20980_e21911 - assign20980_e21915);
            let assign20980_e21919: f64 = (-230.25850929940458);
            let assign20980_e21921: f64 = (-locals.var_ysq);
            let assign20980_e21923: f64 = (assign20980_e21921 + locals.var_mtat);
            let assign20980_e21924: f64 = (assign20980_e21919 - assign20980_e21923);
            let assign20980_e21926: f64 = (assign20980_e21924 * 0.3333333333333333);
            let assign20980_e21927: f64 = (1.0 + assign20980_e21926);
            let assign20980_e21928: f64 = (assign20980_e21916 * assign20980_e21927);
            let assign20980_e21929: f64 = (0.5 * assign20980_e21928);
            let assign20980_e21930: f64 = (1.0 + assign20980_e21929);
            let assign20980_e21931: f64 = (assign20980_e21907 * assign20980_e21930);
            let assign20980_e21932: f64 = (1.0 + assign20980_e21931);
            let assign20980_e21933: f64 = (1e-100 / assign20980_e21932);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign20980_e21933, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign20980_e21930) + (assign20980_e21907 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign20980_e21927) + (assign20980_e21916 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign20980_e21932 * assign20980_e21932))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign20980_e21930) + (assign20980_e21907 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign20980_e21927) + (assign20980_e21916 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign20980_e21932 * assign20980_e21932))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign20980_e21930) + (assign20980_e21907 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign20980_e21927) + (assign20980_e21916 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign20980_e21932 * assign20980_e21932))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign20980_e21930) + (assign20980_e21907 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign20980_e21927) + (assign20980_e21916 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign20980_e21932 * assign20980_e21932))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign20990_e21947: f64 = (0.29214664 * locals.var_terfc);
            let assign20990_e21951: f64 = (locals.var_terfc * locals.var_terfc);
            let assign20990_e21952: f64 = (locals.var_berfc * assign20990_e21951);
            let assign20990_e21953: f64 = (assign20990_e21947 + assign20990_e21952);
            let assign20990_e21957: f64 = (locals.var_terfc * locals.var_terfc);
            let assign20990_e21959: f64 = (assign20990_e21957 * locals.var_terfc);
            let assign20990_e21960: f64 = (locals.var_cerfc * assign20990_e21959);
            let assign20990_e21961: f64 = (assign20990_e21953 + assign20990_e21960);
            let assign20990_e21963: f64 = (assign20990_e21961 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign20990_e21963, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign20990_e21957 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign20990_e21961 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign20990_e21957 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign20990_e21961 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign20990_e21957 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign20990_e21961 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign20990_e21957 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign20990_e21961 * locals.var_tmp_dn8)), );
        }
        let assign21000_e21968: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard373 = assign21000_e21968;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard373 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign21020_e21985: f64 = (-230.25850929940458);
        let assign21020_e21986: f64 = if locals.var_mtat > assign21020_e21985 { 1.0 } else { 0.0 };
        locals.var_guard374 = assign21020_e21986;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard373 == 0.0)) && (locals.var_guard374 != 0.0)) {
            let assign21030_e22002: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21030_e22002, (assign21030_e22002 * locals.var_mtat_dn5), (assign21030_e22002 * locals.var_mtat_dn6), (assign21030_e22002 * locals.var_mtat_dn7), (assign21030_e22002 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard373 == 0.0)) && (locals.var_guard374 == 0.0)) {
            let assign21040_e22023: f64 = (-230.25850929940458);
            let assign21040_e22025: f64 = (assign21040_e22023 - locals.var_mtat);
            let assign21040_e22029: f64 = (-230.25850929940458);
            let assign21040_e22031: f64 = (assign21040_e22029 - locals.var_mtat);
            let assign21040_e22034: f64 = (-230.25850929940458);
            let assign21040_e22036: f64 = (assign21040_e22034 - locals.var_mtat);
            let assign21040_e22038: f64 = (assign21040_e22036 * 0.3333333333333333);
            let assign21040_e22039: f64 = (1.0 + assign21040_e22038);
            let assign21040_e22040: f64 = (assign21040_e22031 * assign21040_e22039);
            let assign21040_e22041: f64 = (0.5 * assign21040_e22040);
            let assign21040_e22042: f64 = (1.0 + assign21040_e22041);
            let assign21040_e22043: f64 = (assign21040_e22025 * assign21040_e22042);
            let assign21040_e22044: f64 = (1.0 + assign21040_e22043);
            let assign21040_e22045: f64 = (1e-100 / assign21040_e22044);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21040_e22045, (-((1e-100 * (((-locals.var_mtat_dn5) * assign21040_e22042) + (assign21040_e22025 * (0.5 * (((-locals.var_mtat_dn5) * assign21040_e22039) + (assign21040_e22031 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign21040_e22044 * assign21040_e22044))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign21040_e22042) + (assign21040_e22025 * (0.5 * (((-locals.var_mtat_dn6) * assign21040_e22039) + (assign21040_e22031 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign21040_e22044 * assign21040_e22044))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign21040_e22042) + (assign21040_e22025 * (0.5 * (((-locals.var_mtat_dn7) * assign21040_e22039) + (assign21040_e22031 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign21040_e22044 * assign21040_e22044))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign21040_e22042) + (assign21040_e22025 * (0.5 * (((-locals.var_mtat_dn8) * assign21040_e22039) + (assign21040_e22031 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign21040_e22044 * assign21040_e22044))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard373 == 0.0)) {
            let assign21050_e22062: f64 = (2.0 * locals.var_tmp);
            let assign21050_e22064: f64 = (assign21050_e22062 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign21050_e22064, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign21060_e22078: f64 = (1.772453850905516 * 0.5);
            let assign21060_e22081: f64 = (locals.var_atatbot * locals.var_erfctimesexpmtat);
            let assign21060_e22083: f64 = (assign21060_e22081 / locals.var_ktat);
            let assign21060_e22084: f64 = (assign21060_e22078 * assign21060_e22083);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign21060_e22084, (assign21060_e22078 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign21060_e22081 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign21060_e22078 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign21060_e22081 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign21060_e22078 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign21060_e22081 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign21060_e22078 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign21060_e22081 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard369 == 0.0)) {
            let assign21070_e22099: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign21070_e22101: f64 = (assign21070_e22099 * locals.var_wtat);
            let assign21070_e22102: f64 = (p.p845 * assign21070_e22101);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign21070_e22102, (p.p845 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign21070_e22099 * locals.var_wtat_dn5))), (p.p845 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign21070_e22099 * locals.var_wtat_dn6))), (p.p845 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign21070_e22099 * locals.var_wtat_dn7))), (p.p845 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign21070_e22099 * locals.var_wtat_dn8))), );
        }
        let assign21080_e22107: f64 = if p.p851 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard375 = assign21080_e22107;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard375 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign21100_e22121: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard376 = assign21100_e22121;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard375 == 0.0)) && (locals.var_guard376 != 0.0)) {
            let assign21110_e22135: f64 = (p.p828 - locals.var_vbbt);
            let assign21110_e22137: f64 = (assign21110_e22135 * locals.var_vbirbotinv);
            let assign21110_e22138: f64 = (assign21110_e22137).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21110_e22138, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard375 == 0.0)) && (locals.var_guard376 == 0.0)) {
            let assign21120_e22155: f64 = (p.p828 - locals.var_vbbt);
            let assign21120_e22157: f64 = (assign21120_e22155 * locals.var_vbirbotinv);
            let assign21120_e22159: f64 = (assign21120_e22157).powf(p.p831);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21120_e22159, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard375 == 0.0)) {
            let assign21130_e22174: f64 = (p.p828 - locals.var_vbbt);
            let assign21130_e22176: f64 = (assign21130_e22174 * locals.var_wdepnulrinvbot);
            let assign21130_e22178: f64 = (assign21130_e22176 / locals.var_tmp);
            let assign21130_e22179: f64 = (locals.var_one_over_one_minus_pbot * assign21130_e22178);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign21130_e22179, (locals.var_one_over_one_minus_pbot * (-((assign21130_e22176 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign21130_e22176 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign21130_e22176 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign21130_e22176 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign21140_e22183: f64 = (-locals.var_fbbtbot);
        let assign21140_e22185: f64 = (assign21140_e22183 / locals.var_fmaxr);
        let assign21140_e22186: f64 = (assign21140_e22185).abs();
        let assign21140_e22188: f64 = if assign21140_e22186 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard377 = assign21140_e22188;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard375 == 0.0)) && (locals.var_guard377 != 0.0)) {
            let assign21150_e22201: f64 = (-locals.var_fbbtbot);
            let assign21150_e22203: f64 = (assign21150_e22201 / locals.var_fmaxr);
            let assign21150_e22204: f64 = (assign21150_e22203).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21150_e22204, (assign21150_e22204 * (-((assign21150_e22201 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign21150_e22204 * (-((assign21150_e22201 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign21150_e22204 * (-((assign21150_e22201 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign21150_e22204 * (-((assign21150_e22201 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign21160_e22208: f64 = (-locals.var_fbbtbot);
        let assign21160_e22210: f64 = (assign21160_e22208 / locals.var_fmaxr);
        let assign21160_e22212: f64 = if assign21160_e22210 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard378 = assign21160_e22212;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard375 == 0.0)) && (locals.var_guard377 == 0.0)) && (locals.var_guard378 != 0.0)) {
            let assign21170_e22230: f64 = (-230.25850929940458);
            let assign21170_e22232: f64 = (-locals.var_fbbtbot);
            let assign21170_e22234: f64 = (assign21170_e22232 / locals.var_fmaxr);
            let assign21170_e22235: f64 = (assign21170_e22230 - assign21170_e22234);
            let assign21170_e22239: f64 = (-230.25850929940458);
            let assign21170_e22241: f64 = (-locals.var_fbbtbot);
            let assign21170_e22243: f64 = (assign21170_e22241 / locals.var_fmaxr);
            let assign21170_e22244: f64 = (assign21170_e22239 - assign21170_e22243);
            let assign21170_e22247: f64 = (-230.25850929940458);
            let assign21170_e22249: f64 = (-locals.var_fbbtbot);
            let assign21170_e22251: f64 = (assign21170_e22249 / locals.var_fmaxr);
            let assign21170_e22252: f64 = (assign21170_e22247 - assign21170_e22251);
            let assign21170_e22254: f64 = (assign21170_e22252 * 0.3333333333333333);
            let assign21170_e22255: f64 = (1.0 + assign21170_e22254);
            let assign21170_e22256: f64 = (assign21170_e22244 * assign21170_e22255);
            let assign21170_e22257: f64 = (0.5 * assign21170_e22256);
            let assign21170_e22258: f64 = (1.0 + assign21170_e22257);
            let assign21170_e22259: f64 = (assign21170_e22235 * assign21170_e22258);
            let assign21170_e22260: f64 = (1.0 + assign21170_e22259);
            let assign21170_e22261: f64 = (1e-100 / assign21170_e22260);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21170_e22261, (-((1e-100 * (((-(-((assign21170_e22232 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21170_e22258) + (assign21170_e22235 * (0.5 * (((-(-((assign21170_e22241 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21170_e22255) + (assign21170_e22244 * ((-(-((assign21170_e22249 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign21170_e22260 * assign21170_e22260))), (-((1e-100 * (((-(-((assign21170_e22232 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21170_e22258) + (assign21170_e22235 * (0.5 * (((-(-((assign21170_e22241 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21170_e22255) + (assign21170_e22244 * ((-(-((assign21170_e22249 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign21170_e22260 * assign21170_e22260))), (-((1e-100 * (((-(-((assign21170_e22232 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21170_e22258) + (assign21170_e22235 * (0.5 * (((-(-((assign21170_e22241 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21170_e22255) + (assign21170_e22244 * ((-(-((assign21170_e22249 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign21170_e22260 * assign21170_e22260))), (-((1e-100 * (((-(-((assign21170_e22232 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21170_e22258) + (assign21170_e22235 * (0.5 * (((-(-((assign21170_e22241 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21170_e22255) + (assign21170_e22244 * ((-(-((assign21170_e22249 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign21170_e22260 * assign21170_e22260))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard375 == 0.0)) && (locals.var_guard377 == 0.0)) && (locals.var_guard378 == 0.0)) {
            let assign21180_e22282: f64 = (-locals.var_fbbtbot);
            let assign21180_e22284: f64 = (assign21180_e22282 / locals.var_fmaxr);
            let assign21180_e22286: f64 = (assign21180_e22284 - 230.25850929940458);
            let assign21180_e22290: f64 = (-locals.var_fbbtbot);
            let assign21180_e22292: f64 = (assign21180_e22290 / locals.var_fmaxr);
            let assign21180_e22294: f64 = (assign21180_e22292 - 230.25850929940458);
            let assign21180_e22297: f64 = (-locals.var_fbbtbot);
            let assign21180_e22299: f64 = (assign21180_e22297 / locals.var_fmaxr);
            let assign21180_e22301: f64 = (assign21180_e22299 - 230.25850929940458);
            let assign21180_e22303: f64 = (assign21180_e22301 * 0.3333333333333333);
            let assign21180_e22304: f64 = (1.0 + assign21180_e22303);
            let assign21180_e22305: f64 = (assign21180_e22294 * assign21180_e22304);
            let assign21180_e22306: f64 = (0.5 * assign21180_e22305);
            let assign21180_e22307: f64 = (1.0 + assign21180_e22306);
            let assign21180_e22308: f64 = (assign21180_e22286 * assign21180_e22307);
            let assign21180_e22309: f64 = (1.0 + assign21180_e22308);
            let assign21180_e22310: f64 = (1e100 * assign21180_e22309);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21180_e22310, (1e100 * (((-((assign21180_e22282 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21180_e22307) + (assign21180_e22286 * (0.5 * (((-((assign21180_e22290 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21180_e22304) + (assign21180_e22294 * ((-((assign21180_e22297 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21180_e22282 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21180_e22307) + (assign21180_e22286 * (0.5 * (((-((assign21180_e22290 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21180_e22304) + (assign21180_e22294 * ((-((assign21180_e22297 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21180_e22282 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21180_e22307) + (assign21180_e22286 * (0.5 * (((-((assign21180_e22290 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21180_e22304) + (assign21180_e22294 * ((-((assign21180_e22297 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21180_e22282 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21180_e22307) + (assign21180_e22286 * (0.5 * (((-((assign21180_e22290 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21180_e22304) + (assign21180_e22294 * ((-((assign21180_e22297 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard375 == 0.0)) {
            let assign21190_e22325: f64 = (locals.var_v3 * locals.var_fmaxr);
            let assign21190_e22327: f64 = (assign21190_e22325 * locals.var_fmaxr);
            let assign21190_e22329: f64 = (assign21190_e22327 * locals.var_tmp);
            let assign21190_e22330: f64 = (p.p851 * assign21190_e22329);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign21190_e22330, (p.p851 * (((((locals.var_v3 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign21190_e22325 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign21190_e22327 * locals.var_tmp_dn5))), (p.p851 * (((((locals.var_v3 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign21190_e22325 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign21190_e22327 * locals.var_tmp_dn6))), (p.p851 * (((((locals.var_v3 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign21190_e22325 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign21190_e22327 * locals.var_tmp_dn7))), (p.p851 * (((((locals.var_v3 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign21190_e22325 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign21190_e22327 * locals.var_tmp_dn8))), );
        }
        let assign21200_e22335: f64 = if p.p860 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard379 = assign21200_e22335;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard379 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign21220_e22349: f64 = (-locals.var_alphaav);
        let assign21220_e22351: f64 = (assign21220_e22349 * p.p860);
        let assign21220_e22352: f64 = if locals.var_vav > assign21220_e22351 { 1.0 } else { 0.0 };
        locals.var_guard380 = assign21220_e22352;
        let assign21230_e22355: f64 = if p.p863 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign21230_e22355;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard379 == 0.0)) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) {
            let assign21240_e22371: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign21240_e22374: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign21240_e22375: f64 = (assign21240_e22371 * assign21240_e22374);
            let assign21240_e22378: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign21240_e22379: f64 = (assign21240_e22375 * assign21240_e22378);
            let assign21240_e22382: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign21240_e22383: f64 = (assign21240_e22379 * assign21240_e22382);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21240_e22383, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard379 == 0.0)) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 == 0.0)) {
            let assign21250_e22402: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign21250_e22403: f64 = (assign21250_e22402).abs();
            let assign21250_e22405: f64 = (assign21250_e22403).powf(p.p863);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21250_e22405, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard379 == 0.0)) && (locals.var_guard380 != 0.0)) {
            let assign21260_e22422: f64 = (1.0 - locals.var_tmp);
            let assign21260_e22423: f64 = (1.0 / assign21260_e22422);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign21260_e22423, (-((-locals.var_tmp_dn5) / (assign21260_e22422 * assign21260_e22422))), (-((-locals.var_tmp_dn6) / (assign21260_e22422 * assign21260_e22422))), (-((-locals.var_tmp_dn7) / (assign21260_e22422 * assign21260_e22422))), (-((-locals.var_tmp_dn8) / (assign21260_e22422 * assign21260_e22422))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) && (locals.var_guard379 == 0.0)) && (locals.var_guard380 == 0.0)) {
            let assign21270_e22442: f64 = (locals.var_alphaav * p.p860);
            let assign21270_e22443: f64 = (locals.var_vav + assign21270_e22442);
            let assign21270_e22445: f64 = (assign21270_e22443 * locals.var_slopebot);
            let assign21270_e22446: f64 = (locals.var_fstopbot + assign21270_e22445);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign21270_e22446, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard365 == 0.0)) {
            let assign21280_e22458: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign21280_e22460: f64 = (assign21280_e22458 + locals.var_itat);
            let assign21280_e22462: f64 = (assign21280_e22460 + locals.var_ibbt);
            let assign21280_e22463: f64 = (p.p29 * assign21280_e22462);
            let assign21280_e22465: f64 = (assign21280_e22463 * locals.var_fbreakdown);
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (assign21280_e22465, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign21280_e22463 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign21280_e22463 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign21280_e22463 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign21280_e22463 * locals.var_fbreakdown_dn8)), );
        }
        let assign21290_e22470: f64 = if locals.var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard382 = assign21290_e22470;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 != 0.0)) {
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) {
            let assign21310_e22487: f64 = (locals.var_idsatsti * locals.var_idmult);
            locals.var_id__blk219 = assign21310_e22487;
        }
        let assign21320_e22496: f64 = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard383 = assign21320_e22496;
    }
    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) {
            let assign21340_e22519: f64 = (locals.var_vbisti - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign21340_e22519;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) {
            let assign21350_e22535: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign21350_e22536: f64 = (1.0 - assign21350_e22535);
            let assign21350_e22537: f64 = (assign21350_e22536).sqrt();
            let assign21350_e22538: f64 = (1.0 - assign21350_e22537);
            locals.var_wsrhstep = assign21350_e22538;
        }
        let assign21360_e22543: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard384 = assign21360_e22543;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (locals.var_guard384 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (locals.var_guard384 == 0.0)) {
            let assign21380_e22572: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign21380_e22574: f64 = (locals.var_wsrhstep).ln();
            let assign21380_e22575: f64 = (assign21380_e22572 * assign21380_e22574);
            let assign21380_e22578: f64 = (1.0 - locals.var_wsrhstep);
            let assign21380_e22579: f64 = (assign21380_e22575 / assign21380_e22578);
            let assign21380_e22581: f64 = (assign21380_e22579 + locals.var_wsrhstep);
            let assign21380_e22585: f64 = (2.0 * p.p832);
            let assign21380_e22586: f64 = (1.0 - assign21380_e22585);
            let assign21380_e22587: f64 = (assign21380_e22581 * assign21380_e22586);
            locals.var_dwsrh = assign21380_e22587;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) {
            let assign21390_e22601: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign21390_e22601;
        }
        let assign21400_e22606: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard385 = assign21400_e22606;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (locals.var_guard385 != 0.0)) {
            let assign21410_e22620: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv);
            let assign21410_e22621: f64 = (assign21410_e22620).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21410_e22621, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (locals.var_guard385 == 0.0)) {
            let assign21420_e22638: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv);
            let assign21420_e22640: f64 = (assign21420_e22638).powf(p.p832);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21420_e22640, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) {
            let assign21430_e22654: f64 = (locals.var_wdepnulrsti * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign21430_e22654, (locals.var_wdepnulrsti * locals.var_tmp_dn5), (locals.var_wdepnulrsti * locals.var_tmp_dn6), (locals.var_wdepnulrsti * locals.var_tmp_dn7), (locals.var_wdepnulrsti * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) {
            let assign21440_e22669: f64 = (locals.var_zinv - 1.0);
            let assign21440_e22671: f64 = (assign21440_e22669 * locals.var_wdep);
            let assign21440_e22672: f64 = (locals.var_ftdsti * assign21440_e22671);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign21440_e22672, (locals.var_ftdsti * (assign21440_e22669 * locals.var_wdep_dn5)), (locals.var_ftdsti * (assign21440_e22669 * locals.var_wdep_dn6)), (locals.var_ftdsti * (assign21440_e22669 * locals.var_wdep_dn7)), (locals.var_ftdsti * (assign21440_e22669 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) {
            let assign21450_e22687: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign21450_e22688: f64 = (p.p841 * assign21450_e22687);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign21450_e22688, (p.p841 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign21460_e22693: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard386 = assign21460_e22693;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21480_e22717: f64 = (locals.var_wdep * locals.var_one_minus_psti);
            let assign21480_e22719: f64 = (assign21480_e22717 / locals.var_vbi_minus_vjsrh);
            let assign21480_e22720: f64 = (locals.var_btatpartsti * assign21480_e22719);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign21480_e22720, (locals.var_btatpartsti * ((locals.var_wdep_dn5 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn6 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn7 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn8 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21490_e22734: f64 = (0.666666666666667 * locals.var_atatsti);
            let assign21490_e22736: f64 = (assign21490_e22734 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign21490_e22736, (-((assign21490_e22734 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign21490_e22734 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign21490_e22734 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign21490_e22734 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21500_e22750: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign21500_e22750, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21510_e22764: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign21510_e22767: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign21510_e22769: f64 = (assign21510_e22767 + 1.0);
            let assign21510_e22770: f64 = (assign21510_e22764 / assign21510_e22769);
            let assign21510_e22771: f64 = (assign21510_e22770).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign21510_e22771, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign21510_e22769) - (assign21510_e22764 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign21510_e22769 * assign21510_e22769)) / (2.0 * assign21510_e22771)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign21510_e22769) - (assign21510_e22764 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign21510_e22769 * assign21510_e22769)) / (2.0 * assign21510_e22771)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign21510_e22769) - (assign21510_e22764 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign21510_e22769 * assign21510_e22769)) / (2.0 * assign21510_e22771)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign21510_e22769) - (assign21510_e22764 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign21510_e22769 * assign21510_e22769)) / (2.0 * assign21510_e22771)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21520_e22784: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign21520_e22784, (locals.var_umax_dn5 / (2.0 * assign21520_e22784)), (locals.var_umax_dn6 / (2.0 * assign21520_e22784)), (locals.var_umax_dn7 / (2.0 * assign21520_e22784)), (locals.var_umax_dn8 / (2.0 * assign21520_e22784)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21530_e22798: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign21530_e22798, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign21540_e22802: f64 = (-p.p832);
        let assign21540_e22804: f64 = (assign21540_e22802 * locals.var_one_over_one_minus_psti);
        let assign21540_e22806: f64 = (-1.0);
        let assign21540_e22807: f64 = if assign21540_e22804 == assign21540_e22806 { 1.0 } else { 0.0 };
        locals.var_guard387 = assign21540_e22807;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) && (locals.var_guard387 != 0.0)) {
            let assign21550_e22823: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign21550_e22824: f64 = (1.0 + assign21550_e22823);
            let assign21550_e22825: f64 = (1.0 / assign21550_e22824);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign21550_e22825, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign21550_e22824 * assign21550_e22824))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign21550_e22824 * assign21550_e22824))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign21550_e22824 * assign21550_e22824))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign21550_e22824 * assign21550_e22824))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) && (locals.var_guard387 == 0.0)) {
            let assign21560_e22843: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign21560_e22844: f64 = (1.0 + assign21560_e22843);
            let assign21560_e22846: f64 = (-p.p832);
            let assign21560_e22848: f64 = (assign21560_e22846 * locals.var_one_over_one_minus_psti);
            let assign21560_e22849: f64 = (assign21560_e22844).powf(assign21560_e22848);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign21560_e22849, if 0.0 == 0.0 && ((assign21560_e22848) as f64).is_finite() && ((assign21560_e22848) as f64).fract() == 0.0 { if assign21560_e22848 == 0.0 { 0.0 } else { (assign21560_e22848 * ((assign21560_e22844).powf(assign21560_e22848 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign21560_e22849 * (assign21560_e22848 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign21560_e22844))) }, if 0.0 == 0.0 && ((assign21560_e22848) as f64).is_finite() && ((assign21560_e22848) as f64).fract() == 0.0 { if assign21560_e22848 == 0.0 { 0.0 } else { (assign21560_e22848 * ((assign21560_e22844).powf(assign21560_e22848 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign21560_e22849 * (assign21560_e22848 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign21560_e22844))) }, if 0.0 == 0.0 && ((assign21560_e22848) as f64).is_finite() && ((assign21560_e22848) as f64).fract() == 0.0 { if assign21560_e22848 == 0.0 { 0.0 } else { (assign21560_e22848 * ((assign21560_e22844).powf(assign21560_e22848 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign21560_e22849 * (assign21560_e22848 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign21560_e22844))) }, if 0.0 == 0.0 && ((assign21560_e22848) as f64).is_finite() && ((assign21560_e22848) as f64).fract() == 0.0 { if assign21560_e22848 == 0.0 { 0.0 } else { (assign21560_e22848 * ((assign21560_e22844).powf(assign21560_e22848 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign21560_e22849 * (assign21560_e22848 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign21560_e22844))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21570_e22863: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign21570_e22866: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign21570_e22867: f64 = (assign21570_e22863 / assign21570_e22866);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign21570_e22867, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign21570_e22866) - (assign21570_e22863 * locals.var_wgamma_dn5)) / (assign21570_e22866 * assign21570_e22866)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign21570_e22866) - (assign21570_e22863 * locals.var_wgamma_dn6)) / (assign21570_e22866 * assign21570_e22866)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign21570_e22866) - (assign21570_e22863 * locals.var_wgamma_dn7)) / (assign21570_e22866 * assign21570_e22866)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign21570_e22866) - (assign21570_e22863 * locals.var_wgamma_dn8)) / (assign21570_e22866 * assign21570_e22866)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21580_e22882: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign21580_e22883: f64 = (0.375 * assign21580_e22882);
            let assign21580_e22884: f64 = (assign21580_e22883).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign21580_e22884, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign21580_e22884)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign21580_e22884)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign21580_e22884)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign21580_e22884)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21590_e22899: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign21590_e22900: f64 = (2.0 * assign21590_e22899);
            let assign21590_e22902: f64 = (assign21590_e22900 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign21590_e22902, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21600_e22916: f64 = (locals.var_atatsti * locals.var_twoatatoverthreebtat);
            let assign21600_e22918: f64 = (assign21600_e22916 * locals.var_sqrtumax);
            let assign21600_e22921: f64 = (locals.var_atatsti * locals.var_umax);
            let assign21600_e22922: f64 = (assign21600_e22918 - assign21600_e22921);
            let assign21600_e22926: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign21600_e22927: f64 = (0.5 * assign21600_e22926);
            let assign21600_e22928: f64 = (assign21600_e22922 + assign21600_e22927);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign21600_e22928, (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign21600_e22916 * locals.var_sqrtumax_dn5)) - (locals.var_atatsti * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign21600_e22916 * locals.var_sqrtumax_dn6)) - (locals.var_atatsti * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign21600_e22916 * locals.var_sqrtumax_dn7)) - (locals.var_atatsti * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign21600_e22916 * locals.var_sqrtumax_dn8)) - (locals.var_atatsti * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21610_e22942: f64 = (locals.var_ltat - 1.0);
            let assign21610_e22944: f64 = (assign21610_e22942 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign21610_e22944, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign21610_e22942 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign21610_e22942 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign21610_e22942 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign21610_e22942 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21620_e22958: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign21620_e22958, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign21630_e22963: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard388 = assign21630_e22963;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) && (locals.var_guard388 != 0.0)) {
            let assign21640_e22979: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign21640_e22980: f64 = (1.0 + assign21640_e22979);
            let assign21640_e22981: f64 = (1.0 / assign21640_e22980);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign21640_e22981, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign21640_e22980 * assign21640_e22980))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign21640_e22980 * assign21640_e22980))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign21640_e22980 * assign21640_e22980))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign21640_e22980 * assign21640_e22980))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) && (locals.var_guard388 == 0.0)) {
            let assign21650_e23000: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign21650_e23001: f64 = (1.0 - assign21650_e23000);
            let assign21650_e23002: f64 = (1.0 / assign21650_e23001);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign21650_e23002, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign21650_e23001 * assign21650_e23001))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign21650_e23001 * assign21650_e23001))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign21650_e23001 * assign21650_e23001))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign21650_e23001 * assign21650_e23001))), );
        }
        let assign21660_e23006: f64 = (-locals.var_ysq);
        let assign21660_e23008: f64 = (assign21660_e23006 + locals.var_mtat);
        let assign21660_e23010: f64 = (-230.25850929940458);
        let assign21660_e23011: f64 = if assign21660_e23008 > assign21660_e23010 { 1.0 } else { 0.0 };
        locals.var_guard389 = assign21660_e23011;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) && (locals.var_guard389 != 0.0)) {
            let assign21670_e23024: f64 = (-locals.var_ysq);
            let assign21670_e23026: f64 = (assign21670_e23024 + locals.var_mtat);
            let assign21670_e23027: f64 = (assign21670_e23026).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21670_e23027, (assign21670_e23027 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign21670_e23027 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign21670_e23027 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign21670_e23027 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) && (locals.var_guard389 == 0.0)) {
            let assign21680_e23045: f64 = (-230.25850929940458);
            let assign21680_e23047: f64 = (-locals.var_ysq);
            let assign21680_e23049: f64 = (assign21680_e23047 + locals.var_mtat);
            let assign21680_e23050: f64 = (assign21680_e23045 - assign21680_e23049);
            let assign21680_e23054: f64 = (-230.25850929940458);
            let assign21680_e23056: f64 = (-locals.var_ysq);
            let assign21680_e23058: f64 = (assign21680_e23056 + locals.var_mtat);
            let assign21680_e23059: f64 = (assign21680_e23054 - assign21680_e23058);
            let assign21680_e23062: f64 = (-230.25850929940458);
            let assign21680_e23064: f64 = (-locals.var_ysq);
            let assign21680_e23066: f64 = (assign21680_e23064 + locals.var_mtat);
            let assign21680_e23067: f64 = (assign21680_e23062 - assign21680_e23066);
            let assign21680_e23069: f64 = (assign21680_e23067 * 0.3333333333333333);
            let assign21680_e23070: f64 = (1.0 + assign21680_e23069);
            let assign21680_e23071: f64 = (assign21680_e23059 * assign21680_e23070);
            let assign21680_e23072: f64 = (0.5 * assign21680_e23071);
            let assign21680_e23073: f64 = (1.0 + assign21680_e23072);
            let assign21680_e23074: f64 = (assign21680_e23050 * assign21680_e23073);
            let assign21680_e23075: f64 = (1.0 + assign21680_e23074);
            let assign21680_e23076: f64 = (1e-100 / assign21680_e23075);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21680_e23076, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign21680_e23073) + (assign21680_e23050 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign21680_e23070) + (assign21680_e23059 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign21680_e23075 * assign21680_e23075))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign21680_e23073) + (assign21680_e23050 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign21680_e23070) + (assign21680_e23059 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign21680_e23075 * assign21680_e23075))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign21680_e23073) + (assign21680_e23050 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign21680_e23070) + (assign21680_e23059 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign21680_e23075 * assign21680_e23075))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign21680_e23073) + (assign21680_e23050 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign21680_e23070) + (assign21680_e23059 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign21680_e23075 * assign21680_e23075))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21690_e23090: f64 = (0.29214664 * locals.var_terfc);
            let assign21690_e23094: f64 = (locals.var_terfc * locals.var_terfc);
            let assign21690_e23095: f64 = (locals.var_berfc * assign21690_e23094);
            let assign21690_e23096: f64 = (assign21690_e23090 + assign21690_e23095);
            let assign21690_e23100: f64 = (locals.var_terfc * locals.var_terfc);
            let assign21690_e23102: f64 = (assign21690_e23100 * locals.var_terfc);
            let assign21690_e23103: f64 = (locals.var_cerfc * assign21690_e23102);
            let assign21690_e23104: f64 = (assign21690_e23096 + assign21690_e23103);
            let assign21690_e23106: f64 = (assign21690_e23104 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign21690_e23106, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign21690_e23100 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign21690_e23104 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign21690_e23100 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign21690_e23104 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign21690_e23100 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign21690_e23104 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign21690_e23100 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign21690_e23104 * locals.var_tmp_dn8)), );
        }
        let assign21700_e23111: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard390 = assign21700_e23111;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) && (locals.var_guard390 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign21720_e23128: f64 = (-230.25850929940458);
        let assign21720_e23129: f64 = if locals.var_mtat > assign21720_e23128 { 1.0 } else { 0.0 };
        locals.var_guard391 = assign21720_e23129;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) && (locals.var_guard390 == 0.0)) && (locals.var_guard391 != 0.0)) {
            let assign21730_e23145: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21730_e23145, (assign21730_e23145 * locals.var_mtat_dn5), (assign21730_e23145 * locals.var_mtat_dn6), (assign21730_e23145 * locals.var_mtat_dn7), (assign21730_e23145 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) && (locals.var_guard390 == 0.0)) && (locals.var_guard391 == 0.0)) {
            let assign21740_e23166: f64 = (-230.25850929940458);
            let assign21740_e23168: f64 = (assign21740_e23166 - locals.var_mtat);
            let assign21740_e23172: f64 = (-230.25850929940458);
            let assign21740_e23174: f64 = (assign21740_e23172 - locals.var_mtat);
            let assign21740_e23177: f64 = (-230.25850929940458);
            let assign21740_e23179: f64 = (assign21740_e23177 - locals.var_mtat);
            let assign21740_e23181: f64 = (assign21740_e23179 * 0.3333333333333333);
            let assign21740_e23182: f64 = (1.0 + assign21740_e23181);
            let assign21740_e23183: f64 = (assign21740_e23174 * assign21740_e23182);
            let assign21740_e23184: f64 = (0.5 * assign21740_e23183);
            let assign21740_e23185: f64 = (1.0 + assign21740_e23184);
            let assign21740_e23186: f64 = (assign21740_e23168 * assign21740_e23185);
            let assign21740_e23187: f64 = (1.0 + assign21740_e23186);
            let assign21740_e23188: f64 = (1e-100 / assign21740_e23187);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21740_e23188, (-((1e-100 * (((-locals.var_mtat_dn5) * assign21740_e23185) + (assign21740_e23168 * (0.5 * (((-locals.var_mtat_dn5) * assign21740_e23182) + (assign21740_e23174 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign21740_e23187 * assign21740_e23187))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign21740_e23185) + (assign21740_e23168 * (0.5 * (((-locals.var_mtat_dn6) * assign21740_e23182) + (assign21740_e23174 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign21740_e23187 * assign21740_e23187))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign21740_e23185) + (assign21740_e23168 * (0.5 * (((-locals.var_mtat_dn7) * assign21740_e23182) + (assign21740_e23174 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign21740_e23187 * assign21740_e23187))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign21740_e23185) + (assign21740_e23168 * (0.5 * (((-locals.var_mtat_dn8) * assign21740_e23182) + (assign21740_e23174 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign21740_e23187 * assign21740_e23187))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) && (locals.var_guard390 == 0.0)) {
            let assign21750_e23205: f64 = (2.0 * locals.var_tmp);
            let assign21750_e23207: f64 = (assign21750_e23205 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign21750_e23207, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21760_e23221: f64 = (1.772453850905516 * 0.5);
            let assign21760_e23224: f64 = (locals.var_atatsti * locals.var_erfctimesexpmtat);
            let assign21760_e23226: f64 = (assign21760_e23224 / locals.var_ktat);
            let assign21760_e23227: f64 = (assign21760_e23221 * assign21760_e23226);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign21760_e23227, (assign21760_e23221 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign21760_e23224 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign21760_e23221 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign21760_e23224 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign21760_e23221 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign21760_e23224 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign21760_e23221 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign21760_e23224 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard386 == 0.0)) {
            let assign21770_e23242: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign21770_e23244: f64 = (assign21770_e23242 * locals.var_wtat);
            let assign21770_e23245: f64 = (p.p846 * assign21770_e23244);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign21770_e23245, (p.p846 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign21770_e23242 * locals.var_wtat_dn5))), (p.p846 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign21770_e23242 * locals.var_wtat_dn6))), (p.p846 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign21770_e23242 * locals.var_wtat_dn7))), (p.p846 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign21770_e23242 * locals.var_wtat_dn8))), );
        }
        let assign21780_e23250: f64 = if p.p852 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard392 = assign21780_e23250;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard392 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign21800_e23264: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard393 = assign21800_e23264;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard393 != 0.0)) {
            let assign21810_e23278: f64 = (p.p829 - locals.var_vbbt);
            let assign21810_e23280: f64 = (assign21810_e23278 * locals.var_vbirstiinv);
            let assign21810_e23281: f64 = (assign21810_e23280).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21810_e23281, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard393 == 0.0)) {
            let assign21820_e23298: f64 = (p.p829 - locals.var_vbbt);
            let assign21820_e23300: f64 = (assign21820_e23298 * locals.var_vbirstiinv);
            let assign21820_e23302: f64 = (assign21820_e23300).powf(p.p832);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21820_e23302, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard392 == 0.0)) {
            let assign21830_e23317: f64 = (p.p829 - locals.var_vbbt);
            let assign21830_e23319: f64 = (assign21830_e23317 * locals.var_wdepnulrinvsti);
            let assign21830_e23321: f64 = (assign21830_e23319 / locals.var_tmp);
            let assign21830_e23322: f64 = (locals.var_one_over_one_minus_psti * assign21830_e23321);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign21830_e23322, (locals.var_one_over_one_minus_psti * (-((assign21830_e23319 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign21830_e23319 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign21830_e23319 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign21830_e23319 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign21840_e23326: f64 = (-locals.var_fbbtsti);
        let assign21840_e23328: f64 = (assign21840_e23326 / locals.var_fmaxr);
        let assign21840_e23329: f64 = (assign21840_e23328).abs();
        let assign21840_e23331: f64 = if assign21840_e23329 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign21840_e23331;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard394 != 0.0)) {
            let assign21850_e23344: f64 = (-locals.var_fbbtsti);
            let assign21850_e23346: f64 = (assign21850_e23344 / locals.var_fmaxr);
            let assign21850_e23347: f64 = (assign21850_e23346).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21850_e23347, (assign21850_e23347 * (-((assign21850_e23344 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign21850_e23347 * (-((assign21850_e23344 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign21850_e23347 * (-((assign21850_e23344 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign21850_e23347 * (-((assign21850_e23344 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign21860_e23351: f64 = (-locals.var_fbbtsti);
        let assign21860_e23353: f64 = (assign21860_e23351 / locals.var_fmaxr);
        let assign21860_e23355: f64 = if assign21860_e23353 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard395 = assign21860_e23355;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard394 == 0.0)) && (locals.var_guard395 != 0.0)) {
            let assign21870_e23373: f64 = (-230.25850929940458);
            let assign21870_e23375: f64 = (-locals.var_fbbtsti);
            let assign21870_e23377: f64 = (assign21870_e23375 / locals.var_fmaxr);
            let assign21870_e23378: f64 = (assign21870_e23373 - assign21870_e23377);
            let assign21870_e23382: f64 = (-230.25850929940458);
            let assign21870_e23384: f64 = (-locals.var_fbbtsti);
            let assign21870_e23386: f64 = (assign21870_e23384 / locals.var_fmaxr);
            let assign21870_e23387: f64 = (assign21870_e23382 - assign21870_e23386);
            let assign21870_e23390: f64 = (-230.25850929940458);
            let assign21870_e23392: f64 = (-locals.var_fbbtsti);
            let assign21870_e23394: f64 = (assign21870_e23392 / locals.var_fmaxr);
            let assign21870_e23395: f64 = (assign21870_e23390 - assign21870_e23394);
            let assign21870_e23397: f64 = (assign21870_e23395 * 0.3333333333333333);
            let assign21870_e23398: f64 = (1.0 + assign21870_e23397);
            let assign21870_e23399: f64 = (assign21870_e23387 * assign21870_e23398);
            let assign21870_e23400: f64 = (0.5 * assign21870_e23399);
            let assign21870_e23401: f64 = (1.0 + assign21870_e23400);
            let assign21870_e23402: f64 = (assign21870_e23378 * assign21870_e23401);
            let assign21870_e23403: f64 = (1.0 + assign21870_e23402);
            let assign21870_e23404: f64 = (1e-100 / assign21870_e23403);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21870_e23404, (-((1e-100 * (((-(-((assign21870_e23375 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21870_e23401) + (assign21870_e23378 * (0.5 * (((-(-((assign21870_e23384 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21870_e23398) + (assign21870_e23387 * ((-(-((assign21870_e23392 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign21870_e23403 * assign21870_e23403))), (-((1e-100 * (((-(-((assign21870_e23375 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21870_e23401) + (assign21870_e23378 * (0.5 * (((-(-((assign21870_e23384 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21870_e23398) + (assign21870_e23387 * ((-(-((assign21870_e23392 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign21870_e23403 * assign21870_e23403))), (-((1e-100 * (((-(-((assign21870_e23375 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21870_e23401) + (assign21870_e23378 * (0.5 * (((-(-((assign21870_e23384 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21870_e23398) + (assign21870_e23387 * ((-(-((assign21870_e23392 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign21870_e23403 * assign21870_e23403))), (-((1e-100 * (((-(-((assign21870_e23375 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21870_e23401) + (assign21870_e23378 * (0.5 * (((-(-((assign21870_e23384 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign21870_e23398) + (assign21870_e23387 * ((-(-((assign21870_e23392 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign21870_e23403 * assign21870_e23403))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard394 == 0.0)) && (locals.var_guard395 == 0.0)) {
            let assign21880_e23425: f64 = (-locals.var_fbbtsti);
            let assign21880_e23427: f64 = (assign21880_e23425 / locals.var_fmaxr);
            let assign21880_e23429: f64 = (assign21880_e23427 - 230.25850929940458);
            let assign21880_e23433: f64 = (-locals.var_fbbtsti);
            let assign21880_e23435: f64 = (assign21880_e23433 / locals.var_fmaxr);
            let assign21880_e23437: f64 = (assign21880_e23435 - 230.25850929940458);
            let assign21880_e23440: f64 = (-locals.var_fbbtsti);
            let assign21880_e23442: f64 = (assign21880_e23440 / locals.var_fmaxr);
            let assign21880_e23444: f64 = (assign21880_e23442 - 230.25850929940458);
            let assign21880_e23446: f64 = (assign21880_e23444 * 0.3333333333333333);
            let assign21880_e23447: f64 = (1.0 + assign21880_e23446);
            let assign21880_e23448: f64 = (assign21880_e23437 * assign21880_e23447);
            let assign21880_e23449: f64 = (0.5 * assign21880_e23448);
            let assign21880_e23450: f64 = (1.0 + assign21880_e23449);
            let assign21880_e23451: f64 = (assign21880_e23429 * assign21880_e23450);
            let assign21880_e23452: f64 = (1.0 + assign21880_e23451);
            let assign21880_e23453: f64 = (1e100 * assign21880_e23452);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21880_e23453, (1e100 * (((-((assign21880_e23425 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21880_e23450) + (assign21880_e23429 * (0.5 * (((-((assign21880_e23433 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21880_e23447) + (assign21880_e23437 * ((-((assign21880_e23440 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21880_e23425 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21880_e23450) + (assign21880_e23429 * (0.5 * (((-((assign21880_e23433 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21880_e23447) + (assign21880_e23437 * ((-((assign21880_e23440 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21880_e23425 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21880_e23450) + (assign21880_e23429 * (0.5 * (((-((assign21880_e23433 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21880_e23447) + (assign21880_e23437 * ((-((assign21880_e23440 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21880_e23425 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21880_e23450) + (assign21880_e23429 * (0.5 * (((-((assign21880_e23433 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign21880_e23447) + (assign21880_e23437 * ((-((assign21880_e23440 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard392 == 0.0)) {
            let assign21890_e23468: f64 = (locals.var_v3 * locals.var_fmaxr);
            let assign21890_e23470: f64 = (assign21890_e23468 * locals.var_fmaxr);
            let assign21890_e23472: f64 = (assign21890_e23470 * locals.var_tmp);
            let assign21890_e23473: f64 = (p.p852 * assign21890_e23472);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign21890_e23473, (p.p852 * (((((locals.var_v3 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign21890_e23468 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign21890_e23470 * locals.var_tmp_dn5))), (p.p852 * (((((locals.var_v3 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign21890_e23468 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign21890_e23470 * locals.var_tmp_dn6))), (p.p852 * (((((locals.var_v3 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign21890_e23468 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign21890_e23470 * locals.var_tmp_dn7))), (p.p852 * (((((locals.var_v3 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign21890_e23468 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign21890_e23470 * locals.var_tmp_dn8))), );
        }
        let assign21900_e23478: f64 = if p.p861 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard396 = assign21900_e23478;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard396 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign21920_e23492: f64 = (-locals.var_alphaav);
        let assign21920_e23494: f64 = (assign21920_e23492 * p.p861);
        let assign21920_e23495: f64 = if locals.var_vav > assign21920_e23494 { 1.0 } else { 0.0 };
        locals.var_guard397 = assign21920_e23495;
        let assign21930_e23498: f64 = if p.p864 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard398 = assign21930_e23498;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_guard398 != 0.0)) {
            let assign21940_e23514: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign21940_e23517: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign21940_e23518: f64 = (assign21940_e23514 * assign21940_e23517);
            let assign21940_e23521: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign21940_e23522: f64 = (assign21940_e23518 * assign21940_e23521);
            let assign21940_e23525: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign21940_e23526: f64 = (assign21940_e23522 * assign21940_e23525);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21940_e23526, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_guard398 == 0.0)) {
            let assign21950_e23545: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign21950_e23546: f64 = (assign21950_e23545).abs();
            let assign21950_e23548: f64 = (assign21950_e23546).powf(p.p864);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign21950_e23548, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard397 != 0.0)) {
            let assign21960_e23565: f64 = (1.0 - locals.var_tmp);
            let assign21960_e23566: f64 = (1.0 / assign21960_e23565);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign21960_e23566, (-((-locals.var_tmp_dn5) / (assign21960_e23565 * assign21960_e23565))), (-((-locals.var_tmp_dn6) / (assign21960_e23565 * assign21960_e23565))), (-((-locals.var_tmp_dn7) / (assign21960_e23565 * assign21960_e23565))), (-((-locals.var_tmp_dn8) / (assign21960_e23565 * assign21960_e23565))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard397 == 0.0)) {
            let assign21970_e23585: f64 = (locals.var_alphaav * p.p861);
            let assign21970_e23586: f64 = (locals.var_vav + assign21970_e23585);
            let assign21970_e23588: f64 = (assign21970_e23586 * locals.var_slopesti);
            let assign21970_e23589: f64 = (locals.var_fstopsti + assign21970_e23588);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign21970_e23589, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard382 == 0.0)) {
            let assign21980_e23601: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign21980_e23603: f64 = (assign21980_e23601 + locals.var_itat);
            let assign21980_e23605: f64 = (assign21980_e23603 + locals.var_ibbt);
            let assign21980_e23606: f64 = (p.p29 * assign21980_e23605);
            let assign21980_e23608: f64 = (assign21980_e23606 * locals.var_fbreakdown);
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (assign21980_e23608, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign21980_e23606 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign21980_e23606 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign21980_e23606 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign21980_e23606 * locals.var_fbreakdown_dn8)), );
        }
        let assign21990_e23613: f64 = if locals.var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard399 = assign21990_e23613;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 != 0.0)) {
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) {
            let assign22010_e23630: f64 = (locals.var_idsatgat * locals.var_idmult);
            locals.var_id__blk219 = assign22010_e23630;
        }
        let assign22020_e23639: f64 = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard400 = assign22020_e23639;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 == 0.0)) {
            let assign22040_e23662: f64 = (locals.var_vbigat - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign22040_e23662;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 == 0.0)) {
            let assign22050_e23678: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign22050_e23679: f64 = (1.0 - assign22050_e23678);
            let assign22050_e23680: f64 = (assign22050_e23679).sqrt();
            let assign22050_e23681: f64 = (1.0 - assign22050_e23680);
            locals.var_wsrhstep = assign22050_e23681;
        }
        let assign22060_e23686: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard401 = assign22060_e23686;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 == 0.0)) && (locals.var_guard401 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
    }
    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 == 0.0)) && (locals.var_guard401 == 0.0)) {
            let assign22080_e23715: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign22080_e23717: f64 = (locals.var_wsrhstep).ln();
            let assign22080_e23718: f64 = (assign22080_e23715 * assign22080_e23717);
            let assign22080_e23721: f64 = (1.0 - locals.var_wsrhstep);
            let assign22080_e23722: f64 = (assign22080_e23718 / assign22080_e23721);
            let assign22080_e23724: f64 = (assign22080_e23722 + locals.var_wsrhstep);
            let assign22080_e23728: f64 = (2.0 * p.p833);
            let assign22080_e23729: f64 = (1.0 - assign22080_e23728);
            let assign22080_e23730: f64 = (assign22080_e23724 * assign22080_e23729);
            locals.var_dwsrh = assign22080_e23730;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 == 0.0)) {
            let assign22090_e23744: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign22090_e23744;
        }
        let assign22100_e23749: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard402 = assign22100_e23749;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 == 0.0)) && (locals.var_guard402 != 0.0)) {
            let assign22110_e23763: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv);
            let assign22110_e23764: f64 = (assign22110_e23763).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22110_e23764, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 == 0.0)) && (locals.var_guard402 == 0.0)) {
            let assign22120_e23781: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv);
            let assign22120_e23783: f64 = (assign22120_e23781).powf(p.p833);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22120_e23783, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 == 0.0)) {
            let assign22130_e23797: f64 = (locals.var_wdepnulrgat * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign22130_e23797, (locals.var_wdepnulrgat * locals.var_tmp_dn5), (locals.var_wdepnulrgat * locals.var_tmp_dn6), (locals.var_wdepnulrgat * locals.var_tmp_dn7), (locals.var_wdepnulrgat * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 == 0.0)) {
            let assign22140_e23812: f64 = (locals.var_zinv - 1.0);
            let assign22140_e23814: f64 = (assign22140_e23812 * locals.var_wdep);
            let assign22140_e23815: f64 = (locals.var_ftdgat * assign22140_e23814);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign22140_e23815, (locals.var_ftdgat * (assign22140_e23812 * locals.var_wdep_dn5)), (locals.var_ftdgat * (assign22140_e23812 * locals.var_wdep_dn6)), (locals.var_ftdgat * (assign22140_e23812 * locals.var_wdep_dn7)), (locals.var_ftdgat * (assign22140_e23812 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard400 == 0.0)) {
            let assign22150_e23830: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign22150_e23831: f64 = (p.p842 * assign22150_e23830);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign22150_e23831, (p.p842 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign22160_e23836: f64 = if p.p847 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard403 = assign22160_e23836;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22180_e23860: f64 = (locals.var_wdep * locals.var_one_minus_pgat);
            let assign22180_e23862: f64 = (assign22180_e23860 / locals.var_vbi_minus_vjsrh);
            let assign22180_e23863: f64 = (locals.var_btatpartgat * assign22180_e23862);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign22180_e23863, (locals.var_btatpartgat * ((locals.var_wdep_dn5 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn6 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn7 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn8 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22190_e23877: f64 = (0.666666666666667 * locals.var_atatgat);
            let assign22190_e23879: f64 = (assign22190_e23877 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign22190_e23879, (-((assign22190_e23877 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign22190_e23877 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign22190_e23877 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign22190_e23877 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22200_e23893: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign22200_e23893, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22210_e23907: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign22210_e23910: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign22210_e23912: f64 = (assign22210_e23910 + 1.0);
            let assign22210_e23913: f64 = (assign22210_e23907 / assign22210_e23912);
            let assign22210_e23914: f64 = (assign22210_e23913).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign22210_e23914, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign22210_e23912) - (assign22210_e23907 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign22210_e23912 * assign22210_e23912)) / (2.0 * assign22210_e23914)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign22210_e23912) - (assign22210_e23907 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign22210_e23912 * assign22210_e23912)) / (2.0 * assign22210_e23914)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign22210_e23912) - (assign22210_e23907 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign22210_e23912 * assign22210_e23912)) / (2.0 * assign22210_e23914)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign22210_e23912) - (assign22210_e23907 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign22210_e23912 * assign22210_e23912)) / (2.0 * assign22210_e23914)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22220_e23927: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign22220_e23927, (locals.var_umax_dn5 / (2.0 * assign22220_e23927)), (locals.var_umax_dn6 / (2.0 * assign22220_e23927)), (locals.var_umax_dn7 / (2.0 * assign22220_e23927)), (locals.var_umax_dn8 / (2.0 * assign22220_e23927)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22230_e23941: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign22230_e23941, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign22240_e23945: f64 = (-p.p833);
        let assign22240_e23947: f64 = (assign22240_e23945 * locals.var_one_over_one_minus_pgat);
        let assign22240_e23949: f64 = (-1.0);
        let assign22240_e23950: f64 = if assign22240_e23947 == assign22240_e23949 { 1.0 } else { 0.0 };
        locals.var_guard404 = assign22240_e23950;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) && (locals.var_guard404 != 0.0)) {
            let assign22250_e23966: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign22250_e23967: f64 = (1.0 + assign22250_e23966);
            let assign22250_e23968: f64 = (1.0 / assign22250_e23967);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign22250_e23968, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign22250_e23967 * assign22250_e23967))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign22250_e23967 * assign22250_e23967))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign22250_e23967 * assign22250_e23967))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign22250_e23967 * assign22250_e23967))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) && (locals.var_guard404 == 0.0)) {
            let assign22260_e23986: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign22260_e23987: f64 = (1.0 + assign22260_e23986);
            let assign22260_e23989: f64 = (-p.p833);
            let assign22260_e23991: f64 = (assign22260_e23989 * locals.var_one_over_one_minus_pgat);
            let assign22260_e23992: f64 = (assign22260_e23987).powf(assign22260_e23991);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign22260_e23992, if 0.0 == 0.0 && ((assign22260_e23991) as f64).is_finite() && ((assign22260_e23991) as f64).fract() == 0.0 { if assign22260_e23991 == 0.0 { 0.0 } else { (assign22260_e23991 * ((assign22260_e23987).powf(assign22260_e23991 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign22260_e23992 * (assign22260_e23991 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign22260_e23987))) }, if 0.0 == 0.0 && ((assign22260_e23991) as f64).is_finite() && ((assign22260_e23991) as f64).fract() == 0.0 { if assign22260_e23991 == 0.0 { 0.0 } else { (assign22260_e23991 * ((assign22260_e23987).powf(assign22260_e23991 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign22260_e23992 * (assign22260_e23991 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign22260_e23987))) }, if 0.0 == 0.0 && ((assign22260_e23991) as f64).is_finite() && ((assign22260_e23991) as f64).fract() == 0.0 { if assign22260_e23991 == 0.0 { 0.0 } else { (assign22260_e23991 * ((assign22260_e23987).powf(assign22260_e23991 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign22260_e23992 * (assign22260_e23991 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign22260_e23987))) }, if 0.0 == 0.0 && ((assign22260_e23991) as f64).is_finite() && ((assign22260_e23991) as f64).fract() == 0.0 { if assign22260_e23991 == 0.0 { 0.0 } else { (assign22260_e23991 * ((assign22260_e23987).powf(assign22260_e23991 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign22260_e23992 * (assign22260_e23991 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign22260_e23987))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22270_e24006: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign22270_e24009: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign22270_e24010: f64 = (assign22270_e24006 / assign22270_e24009);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign22270_e24010, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign22270_e24009) - (assign22270_e24006 * locals.var_wgamma_dn5)) / (assign22270_e24009 * assign22270_e24009)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign22270_e24009) - (assign22270_e24006 * locals.var_wgamma_dn6)) / (assign22270_e24009 * assign22270_e24009)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign22270_e24009) - (assign22270_e24006 * locals.var_wgamma_dn7)) / (assign22270_e24009 * assign22270_e24009)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign22270_e24009) - (assign22270_e24006 * locals.var_wgamma_dn8)) / (assign22270_e24009 * assign22270_e24009)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22280_e24025: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign22280_e24026: f64 = (0.375 * assign22280_e24025);
            let assign22280_e24027: f64 = (assign22280_e24026).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign22280_e24027, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign22280_e24027)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign22280_e24027)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign22280_e24027)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign22280_e24027)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22290_e24042: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign22290_e24043: f64 = (2.0 * assign22290_e24042);
            let assign22290_e24045: f64 = (assign22290_e24043 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign22290_e24045, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22300_e24059: f64 = (locals.var_atatgat * locals.var_twoatatoverthreebtat);
            let assign22300_e24061: f64 = (assign22300_e24059 * locals.var_sqrtumax);
            let assign22300_e24064: f64 = (locals.var_atatgat * locals.var_umax);
            let assign22300_e24065: f64 = (assign22300_e24061 - assign22300_e24064);
            let assign22300_e24069: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign22300_e24070: f64 = (0.5 * assign22300_e24069);
            let assign22300_e24071: f64 = (assign22300_e24065 + assign22300_e24070);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign22300_e24071, (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign22300_e24059 * locals.var_sqrtumax_dn5)) - (locals.var_atatgat * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign22300_e24059 * locals.var_sqrtumax_dn6)) - (locals.var_atatgat * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign22300_e24059 * locals.var_sqrtumax_dn7)) - (locals.var_atatgat * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign22300_e24059 * locals.var_sqrtumax_dn8)) - (locals.var_atatgat * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22310_e24085: f64 = (locals.var_ltat - 1.0);
            let assign22310_e24087: f64 = (assign22310_e24085 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign22310_e24087, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign22310_e24085 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign22310_e24085 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign22310_e24085 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign22310_e24085 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22320_e24101: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign22320_e24101, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign22330_e24106: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard405 = assign22330_e24106;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) && (locals.var_guard405 != 0.0)) {
            let assign22340_e24122: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign22340_e24123: f64 = (1.0 + assign22340_e24122);
            let assign22340_e24124: f64 = (1.0 / assign22340_e24123);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign22340_e24124, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign22340_e24123 * assign22340_e24123))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign22340_e24123 * assign22340_e24123))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign22340_e24123 * assign22340_e24123))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign22340_e24123 * assign22340_e24123))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) && (locals.var_guard405 == 0.0)) {
            let assign22350_e24143: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign22350_e24144: f64 = (1.0 - assign22350_e24143);
            let assign22350_e24145: f64 = (1.0 / assign22350_e24144);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign22350_e24145, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign22350_e24144 * assign22350_e24144))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign22350_e24144 * assign22350_e24144))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign22350_e24144 * assign22350_e24144))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign22350_e24144 * assign22350_e24144))), );
        }
        let assign22360_e24149: f64 = (-locals.var_ysq);
        let assign22360_e24151: f64 = (assign22360_e24149 + locals.var_mtat);
        let assign22360_e24153: f64 = (-230.25850929940458);
        let assign22360_e24154: f64 = if assign22360_e24151 > assign22360_e24153 { 1.0 } else { 0.0 };
        locals.var_guard406 = assign22360_e24154;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) && (locals.var_guard406 != 0.0)) {
            let assign22370_e24167: f64 = (-locals.var_ysq);
            let assign22370_e24169: f64 = (assign22370_e24167 + locals.var_mtat);
            let assign22370_e24170: f64 = (assign22370_e24169).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22370_e24170, (assign22370_e24170 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign22370_e24170 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign22370_e24170 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign22370_e24170 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) && (locals.var_guard406 == 0.0)) {
            let assign22380_e24188: f64 = (-230.25850929940458);
            let assign22380_e24190: f64 = (-locals.var_ysq);
            let assign22380_e24192: f64 = (assign22380_e24190 + locals.var_mtat);
            let assign22380_e24193: f64 = (assign22380_e24188 - assign22380_e24192);
            let assign22380_e24197: f64 = (-230.25850929940458);
            let assign22380_e24199: f64 = (-locals.var_ysq);
            let assign22380_e24201: f64 = (assign22380_e24199 + locals.var_mtat);
            let assign22380_e24202: f64 = (assign22380_e24197 - assign22380_e24201);
            let assign22380_e24205: f64 = (-230.25850929940458);
            let assign22380_e24207: f64 = (-locals.var_ysq);
            let assign22380_e24209: f64 = (assign22380_e24207 + locals.var_mtat);
            let assign22380_e24210: f64 = (assign22380_e24205 - assign22380_e24209);
            let assign22380_e24212: f64 = (assign22380_e24210 * 0.3333333333333333);
            let assign22380_e24213: f64 = (1.0 + assign22380_e24212);
            let assign22380_e24214: f64 = (assign22380_e24202 * assign22380_e24213);
            let assign22380_e24215: f64 = (0.5 * assign22380_e24214);
            let assign22380_e24216: f64 = (1.0 + assign22380_e24215);
            let assign22380_e24217: f64 = (assign22380_e24193 * assign22380_e24216);
            let assign22380_e24218: f64 = (1.0 + assign22380_e24217);
            let assign22380_e24219: f64 = (1e-100 / assign22380_e24218);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22380_e24219, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign22380_e24216) + (assign22380_e24193 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign22380_e24213) + (assign22380_e24202 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign22380_e24218 * assign22380_e24218))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign22380_e24216) + (assign22380_e24193 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign22380_e24213) + (assign22380_e24202 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign22380_e24218 * assign22380_e24218))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign22380_e24216) + (assign22380_e24193 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign22380_e24213) + (assign22380_e24202 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign22380_e24218 * assign22380_e24218))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign22380_e24216) + (assign22380_e24193 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign22380_e24213) + (assign22380_e24202 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign22380_e24218 * assign22380_e24218))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22390_e24233: f64 = (0.29214664 * locals.var_terfc);
            let assign22390_e24237: f64 = (locals.var_terfc * locals.var_terfc);
            let assign22390_e24238: f64 = (locals.var_berfc * assign22390_e24237);
            let assign22390_e24239: f64 = (assign22390_e24233 + assign22390_e24238);
            let assign22390_e24243: f64 = (locals.var_terfc * locals.var_terfc);
            let assign22390_e24245: f64 = (assign22390_e24243 * locals.var_terfc);
            let assign22390_e24246: f64 = (locals.var_cerfc * assign22390_e24245);
            let assign22390_e24247: f64 = (assign22390_e24239 + assign22390_e24246);
            let assign22390_e24249: f64 = (assign22390_e24247 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign22390_e24249, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign22390_e24243 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign22390_e24247 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign22390_e24243 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign22390_e24247 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign22390_e24243 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign22390_e24247 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign22390_e24243 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign22390_e24247 * locals.var_tmp_dn8)), );
        }
        let assign22400_e24254: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign22400_e24254;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) && (locals.var_guard407 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign22420_e24271: f64 = (-230.25850929940458);
        let assign22420_e24272: f64 = if locals.var_mtat > assign22420_e24271 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign22420_e24272;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) && (locals.var_guard407 == 0.0)) && (locals.var_guard408 != 0.0)) {
            let assign22430_e24288: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22430_e24288, (assign22430_e24288 * locals.var_mtat_dn5), (assign22430_e24288 * locals.var_mtat_dn6), (assign22430_e24288 * locals.var_mtat_dn7), (assign22430_e24288 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) && (locals.var_guard407 == 0.0)) && (locals.var_guard408 == 0.0)) {
            let assign22440_e24309: f64 = (-230.25850929940458);
            let assign22440_e24311: f64 = (assign22440_e24309 - locals.var_mtat);
            let assign22440_e24315: f64 = (-230.25850929940458);
            let assign22440_e24317: f64 = (assign22440_e24315 - locals.var_mtat);
            let assign22440_e24320: f64 = (-230.25850929940458);
            let assign22440_e24322: f64 = (assign22440_e24320 - locals.var_mtat);
            let assign22440_e24324: f64 = (assign22440_e24322 * 0.3333333333333333);
            let assign22440_e24325: f64 = (1.0 + assign22440_e24324);
            let assign22440_e24326: f64 = (assign22440_e24317 * assign22440_e24325);
            let assign22440_e24327: f64 = (0.5 * assign22440_e24326);
            let assign22440_e24328: f64 = (1.0 + assign22440_e24327);
            let assign22440_e24329: f64 = (assign22440_e24311 * assign22440_e24328);
            let assign22440_e24330: f64 = (1.0 + assign22440_e24329);
            let assign22440_e24331: f64 = (1e-100 / assign22440_e24330);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22440_e24331, (-((1e-100 * (((-locals.var_mtat_dn5) * assign22440_e24328) + (assign22440_e24311 * (0.5 * (((-locals.var_mtat_dn5) * assign22440_e24325) + (assign22440_e24317 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign22440_e24330 * assign22440_e24330))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign22440_e24328) + (assign22440_e24311 * (0.5 * (((-locals.var_mtat_dn6) * assign22440_e24325) + (assign22440_e24317 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign22440_e24330 * assign22440_e24330))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign22440_e24328) + (assign22440_e24311 * (0.5 * (((-locals.var_mtat_dn7) * assign22440_e24325) + (assign22440_e24317 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign22440_e24330 * assign22440_e24330))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign22440_e24328) + (assign22440_e24311 * (0.5 * (((-locals.var_mtat_dn8) * assign22440_e24325) + (assign22440_e24317 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign22440_e24330 * assign22440_e24330))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) && (locals.var_guard407 == 0.0)) {
            let assign22450_e24348: f64 = (2.0 * locals.var_tmp);
            let assign22450_e24350: f64 = (assign22450_e24348 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign22450_e24350, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22460_e24364: f64 = (1.772453850905516 * 0.5);
            let assign22460_e24367: f64 = (locals.var_atatgat * locals.var_erfctimesexpmtat);
            let assign22460_e24369: f64 = (assign22460_e24367 / locals.var_ktat);
            let assign22460_e24370: f64 = (assign22460_e24364 * assign22460_e24369);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign22460_e24370, (assign22460_e24364 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign22460_e24367 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign22460_e24364 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign22460_e24367 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign22460_e24364 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign22460_e24367 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign22460_e24364 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign22460_e24367 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign22470_e24385: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign22470_e24387: f64 = (assign22470_e24385 * locals.var_wtat);
            let assign22470_e24388: f64 = (p.p847 * assign22470_e24387);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign22470_e24388, (p.p847 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign22470_e24385 * locals.var_wtat_dn5))), (p.p847 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign22470_e24385 * locals.var_wtat_dn6))), (p.p847 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign22470_e24385 * locals.var_wtat_dn7))), (p.p847 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign22470_e24385 * locals.var_wtat_dn8))), );
        }
        let assign22480_e24393: f64 = if p.p853 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard409 = assign22480_e24393;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard409 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign22500_e24407: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign22500_e24407;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 != 0.0)) {
            let assign22510_e24421: f64 = (p.p830 - locals.var_vbbt);
            let assign22510_e24423: f64 = (assign22510_e24421 * locals.var_vbirgatinv);
            let assign22510_e24424: f64 = (assign22510_e24423).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22510_e24424, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 == 0.0)) {
            let assign22520_e24441: f64 = (p.p830 - locals.var_vbbt);
            let assign22520_e24443: f64 = (assign22520_e24441 * locals.var_vbirgatinv);
            let assign22520_e24445: f64 = (assign22520_e24443).powf(p.p833);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22520_e24445, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard409 == 0.0)) {
            let assign22530_e24460: f64 = (p.p830 - locals.var_vbbt);
            let assign22530_e24462: f64 = (assign22530_e24460 * locals.var_wdepnulrinvgat);
            let assign22530_e24464: f64 = (assign22530_e24462 / locals.var_tmp);
            let assign22530_e24465: f64 = (locals.var_one_over_one_minus_pgat * assign22530_e24464);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign22530_e24465, (locals.var_one_over_one_minus_pgat * (-((assign22530_e24462 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign22530_e24462 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign22530_e24462 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign22530_e24462 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign22540_e24469: f64 = (-locals.var_fbbtgat);
        let assign22540_e24471: f64 = (assign22540_e24469 / locals.var_fmaxr);
        let assign22540_e24472: f64 = (assign22540_e24471).abs();
        let assign22540_e24474: f64 = if assign22540_e24472 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign22540_e24474;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard409 == 0.0)) && (locals.var_guard411 != 0.0)) {
            let assign22550_e24487: f64 = (-locals.var_fbbtgat);
            let assign22550_e24489: f64 = (assign22550_e24487 / locals.var_fmaxr);
            let assign22550_e24490: f64 = (assign22550_e24489).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22550_e24490, (assign22550_e24490 * ((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign22550_e24487 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign22550_e24490 * ((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign22550_e24487 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign22550_e24490 * ((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign22550_e24487 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign22550_e24490 * ((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign22550_e24487 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))), );
        }
        let assign22560_e24494: f64 = (-locals.var_fbbtgat);
        let assign22560_e24496: f64 = (assign22560_e24494 / locals.var_fmaxr);
        let assign22560_e24498: f64 = if assign22560_e24496 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard412 = assign22560_e24498;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard409 == 0.0)) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 != 0.0)) {
            let assign22570_e24516: f64 = (-230.25850929940458);
            let assign22570_e24518: f64 = (-locals.var_fbbtgat);
            let assign22570_e24520: f64 = (assign22570_e24518 / locals.var_fmaxr);
            let assign22570_e24521: f64 = (assign22570_e24516 - assign22570_e24520);
            let assign22570_e24525: f64 = (-230.25850929940458);
            let assign22570_e24527: f64 = (-locals.var_fbbtgat);
            let assign22570_e24529: f64 = (assign22570_e24527 / locals.var_fmaxr);
            let assign22570_e24530: f64 = (assign22570_e24525 - assign22570_e24529);
            let assign22570_e24533: f64 = (-230.25850929940458);
            let assign22570_e24535: f64 = (-locals.var_fbbtgat);
            let assign22570_e24537: f64 = (assign22570_e24535 / locals.var_fmaxr);
            let assign22570_e24538: f64 = (assign22570_e24533 - assign22570_e24537);
            let assign22570_e24540: f64 = (assign22570_e24538 * 0.3333333333333333);
            let assign22570_e24541: f64 = (1.0 + assign22570_e24540);
            let assign22570_e24542: f64 = (assign22570_e24530 * assign22570_e24541);
            let assign22570_e24543: f64 = (0.5 * assign22570_e24542);
            let assign22570_e24544: f64 = (1.0 + assign22570_e24543);
            let assign22570_e24545: f64 = (assign22570_e24521 * assign22570_e24544);
            let assign22570_e24546: f64 = (1.0 + assign22570_e24545);
            let assign22570_e24547: f64 = (1e-100 / assign22570_e24546);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22570_e24547, (-((1e-100 * (((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign22570_e24518 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign22570_e24544) + (assign22570_e24521 * (0.5 * (((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign22570_e24527 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign22570_e24541) + (assign22570_e24530 * ((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign22570_e24535 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign22570_e24546 * assign22570_e24546))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign22570_e24518 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign22570_e24544) + (assign22570_e24521 * (0.5 * (((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign22570_e24527 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign22570_e24541) + (assign22570_e24530 * ((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign22570_e24535 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign22570_e24546 * assign22570_e24546))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign22570_e24518 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign22570_e24544) + (assign22570_e24521 * (0.5 * (((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign22570_e24527 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign22570_e24541) + (assign22570_e24530 * ((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign22570_e24535 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign22570_e24546 * assign22570_e24546))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign22570_e24518 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign22570_e24544) + (assign22570_e24521 * (0.5 * (((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign22570_e24527 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign22570_e24541) + (assign22570_e24530 * ((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign22570_e24535 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign22570_e24546 * assign22570_e24546))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard409 == 0.0)) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 == 0.0)) {
            let assign22580_e24568: f64 = (-locals.var_fbbtgat);
            let assign22580_e24570: f64 = (assign22580_e24568 / locals.var_fmaxr);
            let assign22580_e24572: f64 = (assign22580_e24570 - 230.25850929940458);
            let assign22580_e24576: f64 = (-locals.var_fbbtgat);
            let assign22580_e24578: f64 = (assign22580_e24576 / locals.var_fmaxr);
            let assign22580_e24580: f64 = (assign22580_e24578 - 230.25850929940458);
            let assign22580_e24583: f64 = (-locals.var_fbbtgat);
            let assign22580_e24585: f64 = (assign22580_e24583 / locals.var_fmaxr);
            let assign22580_e24587: f64 = (assign22580_e24585 - 230.25850929940458);
            let assign22580_e24589: f64 = (assign22580_e24587 * 0.3333333333333333);
            let assign22580_e24590: f64 = (1.0 + assign22580_e24589);
            let assign22580_e24591: f64 = (assign22580_e24580 * assign22580_e24590);
            let assign22580_e24592: f64 = (0.5 * assign22580_e24591);
            let assign22580_e24593: f64 = (1.0 + assign22580_e24592);
            let assign22580_e24594: f64 = (assign22580_e24572 * assign22580_e24593);
            let assign22580_e24595: f64 = (1.0 + assign22580_e24594);
            let assign22580_e24596: f64 = (1e100 * assign22580_e24595);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22580_e24596, (1e100 * ((((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign22580_e24568 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign22580_e24593) + (assign22580_e24572 * (0.5 * ((((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign22580_e24576 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign22580_e24590) + (assign22580_e24580 * (((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign22580_e24583 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign22580_e24568 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign22580_e24593) + (assign22580_e24572 * (0.5 * ((((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign22580_e24576 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign22580_e24590) + (assign22580_e24580 * (((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign22580_e24583 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign22580_e24568 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign22580_e24593) + (assign22580_e24572 * (0.5 * ((((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign22580_e24576 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign22580_e24590) + (assign22580_e24580 * (((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign22580_e24583 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign22580_e24568 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign22580_e24593) + (assign22580_e24572 * (0.5 * ((((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign22580_e24576 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign22580_e24590) + (assign22580_e24580 * (((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign22580_e24583 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard409 == 0.0)) {
            let assign22590_e24611: f64 = (locals.var_v3 * locals.var_fmaxr);
            let assign22590_e24613: f64 = (assign22590_e24611 * locals.var_fmaxr);
            let assign22590_e24615: f64 = (assign22590_e24613 * locals.var_tmp);
            let assign22590_e24616: f64 = (p.p853 * assign22590_e24615);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign22590_e24616, (p.p853 * (((((locals.var_v3 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign22590_e24611 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign22590_e24613 * locals.var_tmp_dn5))), (p.p853 * (((((locals.var_v3 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign22590_e24611 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign22590_e24613 * locals.var_tmp_dn6))), (p.p853 * (((((locals.var_v3 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign22590_e24611 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign22590_e24613 * locals.var_tmp_dn7))), (p.p853 * (((((locals.var_v3 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign22590_e24611 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign22590_e24613 * locals.var_tmp_dn8))), );
        }
        let assign22600_e24621: f64 = if p.p862 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard413 = assign22600_e24621;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard413 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign22620_e24635: f64 = (-locals.var_alphaav);
        let assign22620_e24637: f64 = (assign22620_e24635 * p.p862);
        let assign22620_e24638: f64 = if locals.var_vav > assign22620_e24637 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign22620_e24638;
        let assign22630_e24641: f64 = if p.p865 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard415 = assign22630_e24641;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard413 == 0.0)) && (locals.var_guard414 != 0.0)) && (locals.var_guard415 != 0.0)) {
            let assign22640_e24657: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign22640_e24660: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign22640_e24661: f64 = (assign22640_e24657 * assign22640_e24660);
            let assign22640_e24664: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign22640_e24665: f64 = (assign22640_e24661 * assign22640_e24664);
            let assign22640_e24668: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign22640_e24669: f64 = (assign22640_e24665 * assign22640_e24668);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22640_e24669, (((((((locals.var_vav * locals.var_vbrinvgat_dn5) * assign22640_e24660) + (assign22640_e24657 * (locals.var_vav * locals.var_vbrinvgat_dn5))) * assign22640_e24664) + (assign22640_e24661 * (locals.var_vav * locals.var_vbrinvgat_dn5))) * assign22640_e24668) + (assign22640_e24665 * (locals.var_vav * locals.var_vbrinvgat_dn5))), (((((((locals.var_vav * locals.var_vbrinvgat_dn6) * assign22640_e24660) + (assign22640_e24657 * (locals.var_vav * locals.var_vbrinvgat_dn6))) * assign22640_e24664) + (assign22640_e24661 * (locals.var_vav * locals.var_vbrinvgat_dn6))) * assign22640_e24668) + (assign22640_e24665 * (locals.var_vav * locals.var_vbrinvgat_dn6))), (((((((locals.var_vav * locals.var_vbrinvgat_dn7) * assign22640_e24660) + (assign22640_e24657 * (locals.var_vav * locals.var_vbrinvgat_dn7))) * assign22640_e24664) + (assign22640_e24661 * (locals.var_vav * locals.var_vbrinvgat_dn7))) * assign22640_e24668) + (assign22640_e24665 * (locals.var_vav * locals.var_vbrinvgat_dn7))), (((((((locals.var_vav * locals.var_vbrinvgat_dn8) * assign22640_e24660) + (assign22640_e24657 * (locals.var_vav * locals.var_vbrinvgat_dn8))) * assign22640_e24664) + (assign22640_e24661 * (locals.var_vav * locals.var_vbrinvgat_dn8))) * assign22640_e24668) + (assign22640_e24665 * (locals.var_vav * locals.var_vbrinvgat_dn8))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard413 == 0.0)) && (locals.var_guard414 != 0.0)) && (locals.var_guard415 == 0.0)) {
            let assign22650_e24688: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign22650_e24689: f64 = (assign22650_e24688).abs();
            let assign22650_e24691: f64 = (assign22650_e24689).powf(p.p865);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign22650_e24691, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign22650_e24689).powf(p.p865 - 1.0) * if assign22650_e24688 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn5)) })) } } else { (assign22650_e24691 * (p.p865 * (if assign22650_e24688 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn5)) } / assign22650_e24689))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign22650_e24689).powf(p.p865 - 1.0) * if assign22650_e24688 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn6)) })) } } else { (assign22650_e24691 * (p.p865 * (if assign22650_e24688 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn6)) } / assign22650_e24689))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign22650_e24689).powf(p.p865 - 1.0) * if assign22650_e24688 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn7)) })) } } else { (assign22650_e24691 * (p.p865 * (if assign22650_e24688 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn7)) } / assign22650_e24689))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign22650_e24689).powf(p.p865 - 1.0) * if assign22650_e24688 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn8)) })) } } else { (assign22650_e24691 * (p.p865 * (if assign22650_e24688 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn8)) } / assign22650_e24689))) }, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard413 == 0.0)) && (locals.var_guard414 != 0.0)) {
            let assign22660_e24708: f64 = (1.0 - locals.var_tmp);
            let assign22660_e24709: f64 = (1.0 / assign22660_e24708);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign22660_e24709, (-((-locals.var_tmp_dn5) / (assign22660_e24708 * assign22660_e24708))), (-((-locals.var_tmp_dn6) / (assign22660_e24708 * assign22660_e24708))), (-((-locals.var_tmp_dn7) / (assign22660_e24708 * assign22660_e24708))), (-((-locals.var_tmp_dn8) / (assign22660_e24708 * assign22660_e24708))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) && (locals.var_guard413 == 0.0)) && (locals.var_guard414 == 0.0)) {
            let assign22670_e24728: f64 = (locals.var_alphaav * p.p862);
            let assign22670_e24729: f64 = (locals.var_vav + assign22670_e24728);
            let assign22670_e24731: f64 = (assign22670_e24729 * locals.var_slopegat);
            let assign22670_e24732: f64 = (locals.var_fstopgat + assign22670_e24731);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign22670_e24732, (assign22670_e24729 * locals.var_slopegat_dn5), (assign22670_e24729 * locals.var_slopegat_dn6), (assign22670_e24729 * locals.var_slopegat_dn7), (assign22670_e24729 * locals.var_slopegat_dn8), );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard399 == 0.0)) {
            let assign22680_e24744: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign22680_e24746: f64 = (assign22680_e24744 + locals.var_itat);
            let assign22680_e24748: f64 = (assign22680_e24746 + locals.var_ibbt);
            let assign22680_e24749: f64 = (p.p29 * assign22680_e24748);
            let assign22680_e24751: f64 = (assign22680_e24749 * locals.var_fbreakdown);
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (assign22680_e24751, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign22680_e24749 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign22680_e24749 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign22680_e24749 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign22680_e24749 * locals.var_fbreakdown_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign22690_e24759: f64 = (locals.var_absource_i * locals.var_ijunbot);
            let assign22690_e24762: f64 = (locals.var_lssource_i * locals.var_ijunsti);
            let assign22690_e24763: f64 = (assign22690_e24759 + assign22690_e24762);
            let assign22690_e24766: f64 = (locals.var_lgsource_i * locals.var_ijungat);
            let assign22690_e24767: f64 = (assign22690_e24763 + assign22690_e24766);
            (locals.var_i3, locals.var_i3_dn5, locals.var_i3_dn6, locals.var_i3_dn7, locals.var_i3_dn8, ) = (assign22690_e24767, (((locals.var_absource_i * locals.var_ijunbot_dn5) + (locals.var_lssource_i * locals.var_ijunsti_dn5)) + (locals.var_lgsource_i * locals.var_ijungat_dn5)), (((locals.var_absource_i * locals.var_ijunbot_dn6) + (locals.var_lssource_i * locals.var_ijunsti_dn6)) + (locals.var_lgsource_i * locals.var_ijungat_dn6)), (((locals.var_absource_i * locals.var_ijunbot_dn7) + (locals.var_lssource_i * locals.var_ijunsti_dn7)) + (locals.var_lgsource_i * locals.var_ijungat_dn7)), (((locals.var_absource_i * locals.var_ijunbot_dn8) + (locals.var_lssource_i * locals.var_ijunsti_dn8)) + (locals.var_lgsource_i * locals.var_ijungat_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            locals.var_vbbt = 0.0;
            locals.var_two_psistar = 0.0;
        }
        let assign22720_e24793: f64 = if (!(((locals.var_absource_i == 0.0) && (locals.var_lssource_i == 0.0)) && (locals.var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard416 = assign22720_e24793;
        let assign22800_e24879: f64 = if locals.var_v4 < locals.var_vmax_s { 1.0 } else { 0.0 };
        locals.var_guard417 = assign22800_e24879;
        let assign22810_e24881: f64 = (-0.5);
        let assign22810_e24884: f64 = (locals.var_v4 * locals.var_phitdinv);
        let assign22810_e24885: f64 = (assign22810_e24881 * assign22810_e24884);
        let assign22810_e24886: f64 = (assign22810_e24885).abs();
        let assign22810_e24888: f64 = if assign22810_e24886 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard418 = assign22810_e24888;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard418 != 0.0)) {
            let assign22820_e24899: f64 = (-0.5);
            let assign22820_e24902: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign22820_e24903: f64 = (assign22820_e24899 * assign22820_e24902);
            let assign22820_e24904: f64 = (assign22820_e24903).exp();
            locals.var_z = assign22820_e24904;
        }
        let assign22830_e24908: f64 = (-0.5);
        let assign22830_e24911: f64 = (locals.var_v4 * locals.var_phitdinv);
        let assign22830_e24912: f64 = (assign22830_e24908 * assign22830_e24911);
        let assign22830_e24914: f64 = if assign22830_e24912 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign22830_e24914;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard418 == 0.0)) && (locals.var_guard419 != 0.0)) {
            let assign22840_e24930: f64 = (-230.25850929940458);
            let assign22840_e24932: f64 = (-0.5);
            let assign22840_e24935: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign22840_e24936: f64 = (assign22840_e24932 * assign22840_e24935);
            let assign22840_e24937: f64 = (assign22840_e24930 - assign22840_e24936);
            let assign22840_e24941: f64 = (-230.25850929940458);
            let assign22840_e24943: f64 = (-0.5);
            let assign22840_e24946: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign22840_e24947: f64 = (assign22840_e24943 * assign22840_e24946);
            let assign22840_e24948: f64 = (assign22840_e24941 - assign22840_e24947);
            let assign22840_e24951: f64 = (-230.25850929940458);
            let assign22840_e24953: f64 = (-0.5);
            let assign22840_e24956: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign22840_e24957: f64 = (assign22840_e24953 * assign22840_e24956);
            let assign22840_e24958: f64 = (assign22840_e24951 - assign22840_e24957);
            let assign22840_e24960: f64 = (assign22840_e24958 * 0.3333333333333333);
            let assign22840_e24961: f64 = (1.0 + assign22840_e24960);
            let assign22840_e24962: f64 = (assign22840_e24948 * assign22840_e24961);
            let assign22840_e24963: f64 = (0.5 * assign22840_e24962);
            let assign22840_e24964: f64 = (1.0 + assign22840_e24963);
            let assign22840_e24965: f64 = (assign22840_e24937 * assign22840_e24964);
            let assign22840_e24966: f64 = (1.0 + assign22840_e24965);
            let assign22840_e24967: f64 = (1e-100 / assign22840_e24966);
            locals.var_z = assign22840_e24967;
        }
    }
    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard418 == 0.0)) && (locals.var_guard419 == 0.0)) {
            let assign22850_e24986: f64 = (-0.5);
            let assign22850_e24989: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign22850_e24990: f64 = (assign22850_e24986 * assign22850_e24989);
            let assign22850_e24992: f64 = (assign22850_e24990 - 230.25850929940458);
            let assign22850_e24996: f64 = (-0.5);
            let assign22850_e24999: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign22850_e25000: f64 = (assign22850_e24996 * assign22850_e24999);
            let assign22850_e25002: f64 = (assign22850_e25000 - 230.25850929940458);
            let assign22850_e25005: f64 = (-0.5);
            let assign22850_e25008: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign22850_e25009: f64 = (assign22850_e25005 * assign22850_e25008);
            let assign22850_e25011: f64 = (assign22850_e25009 - 230.25850929940458);
            let assign22850_e25013: f64 = (assign22850_e25011 * 0.3333333333333333);
            let assign22850_e25014: f64 = (1.0 + assign22850_e25013);
            let assign22850_e25015: f64 = (assign22850_e25002 * assign22850_e25014);
            let assign22850_e25016: f64 = (0.5 * assign22850_e25015);
            let assign22850_e25017: f64 = (1.0 + assign22850_e25016);
            let assign22850_e25018: f64 = (assign22850_e24992 * assign22850_e25017);
            let assign22850_e25019: f64 = (1.0 + assign22850_e25018);
            let assign22850_e25020: f64 = (1e100 * assign22850_e25019);
            locals.var_z = assign22850_e25020;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) {
            let assign22860_e25032: f64 = (1.0 / locals.var_z);
            locals.var_zinv = assign22860_e25032;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) {
            let assign22870_e25044: f64 = (locals.var_zinv * locals.var_zinv);
            locals.var_idmult = assign22870_e25044;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 == 0.0)) {
            let assign22880_e25058: f64 = (locals.var_v4 - locals.var_vmax_s);
            let assign22880_e25060: f64 = (assign22880_e25058 * locals.var_phitdinv);
            let assign22880_e25061: f64 = (1.0 + assign22880_e25060);
            let assign22880_e25063: f64 = (assign22880_e25061 * locals.var_exp_vmax_over_phitd_s);
            locals.var_idmult = assign22880_e25063;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 == 0.0)) {
            let assign22890_e25075: f64 = (locals.var_idmult).sqrt();
            locals.var_zinv = assign22890_e25075;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 == 0.0)) {
            let assign22900_e25088: f64 = (1.0 / locals.var_zinv);
            locals.var_z = assign22900_e25088;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) {
            let assign22910_e25098: f64 = (locals.var_idmult - 1.0);
            locals.var_idmult = assign22910_e25098;
        }
        let assign22920_e25103: f64 = if locals.var_v4 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard420 = assign22920_e25103;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard420 != 0.0)) {
            let assign22930_e25115: f64 = (2.0 + locals.var_z);
            let assign22930_e25118: f64 = (locals.var_z + 1.0);
            let assign22930_e25121: f64 = (locals.var_z + 3.0);
            let assign22930_e25122: f64 = (assign22930_e25118 * assign22930_e25121);
            let assign22930_e25123: f64 = (assign22930_e25122).sqrt();
            let assign22930_e25124: f64 = (assign22930_e25115 + assign22930_e25123);
            let assign22930_e25125: f64 = (assign22930_e25124).ln();
            let assign22930_e25126: f64 = (locals.var_phitd * assign22930_e25125);
            let assign22930_e25127: f64 = (2.0 * assign22930_e25126);
            locals.var_two_psistar = assign22930_e25127;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard420 == 0.0)) {
            let assign22940_e25139: f64 = (-locals.var_v4);
            let assign22940_e25144: f64 = (2.0 * locals.var_zinv);
            let assign22940_e25146: f64 = (assign22940_e25144 + 1.0);
            let assign22940_e25149: f64 = (1.0 + locals.var_zinv);
            let assign22940_e25153: f64 = (3.0 * locals.var_zinv);
            let assign22940_e25154: f64 = (1.0 + assign22940_e25153);
            let assign22940_e25155: f64 = (assign22940_e25149 * assign22940_e25154);
            let assign22940_e25156: f64 = (assign22940_e25155).sqrt();
            let assign22940_e25157: f64 = (assign22940_e25146 + assign22940_e25156);
            let assign22940_e25158: f64 = (assign22940_e25157).ln();
            let assign22940_e25159: f64 = (locals.var_phitd * assign22940_e25158);
            let assign22940_e25160: f64 = (2.0 * assign22940_e25159);
            let assign22940_e25161: f64 = (assign22940_e25139 + assign22940_e25160);
            locals.var_two_psistar = assign22940_e25161;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) {
            let assign22950_e25171: f64 = (locals.var_vbimin_s - locals.var_two_psistar);
            locals.var_vjlim = assign22950_e25171;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) {
            let assign22960_e25182: f64 = (locals.var_v4 + locals.var_vjlim);
            let assign22960_e25185: f64 = (locals.var_v4 - locals.var_vjlim);
            let assign22960_e25188: f64 = (locals.var_v4 - locals.var_vjlim);
            let assign22960_e25189: f64 = (assign22960_e25185 * assign22960_e25188);
            let assign22960_e25192: f64 = (4.0 * locals.var_phitd);
            let assign22960_e25194: f64 = (assign22960_e25192 * locals.var_phitd);
            let assign22960_e25195: f64 = (assign22960_e25189 + assign22960_e25194);
            let assign22960_e25196: f64 = (assign22960_e25195).sqrt();
            let assign22960_e25197: f64 = (assign22960_e25182 - assign22960_e25196);
            let assign22960_e25198: f64 = (0.5 * assign22960_e25197);
            locals.var_vjsrh = assign22960_e25198;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) {
            let assign22970_e25209: f64 = (locals.var_v4 + locals.var_vbbtlim_s);
            let assign22970_e25212: f64 = (locals.var_v4 - locals.var_vbbtlim_s);
            let assign22970_e25215: f64 = (locals.var_v4 - locals.var_vbbtlim_s);
            let assign22970_e25216: f64 = (assign22970_e25212 * assign22970_e25215);
            let assign22970_e25219: f64 = (4.0 * locals.var_phitr);
            let assign22970_e25221: f64 = (assign22970_e25219 * locals.var_phitr);
            let assign22970_e25222: f64 = (assign22970_e25216 + assign22970_e25221);
            let assign22970_e25223: f64 = (assign22970_e25222).sqrt();
            let assign22970_e25224: f64 = (assign22970_e25209 - assign22970_e25223);
            let assign22970_e25225: f64 = (0.5 * assign22970_e25224);
            locals.var_vbbt = assign22970_e25225;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard416 != 0.0)) {
            let assign22980_e25236: f64 = locals.var_v4;
            let assign22980_e25239: f64 = locals.var_v4;
            let assign22980_e25242: f64 = locals.var_v4;
            let assign22980_e25243: f64 = (assign22980_e25239 * assign22980_e25242);
            let assign22980_e25246: f64 = (4.0 * 1e-6);
            let assign22980_e25248: f64 = (assign22980_e25246 * 1e-6);
            let assign22980_e25249: f64 = (assign22980_e25243 + assign22980_e25248);
            let assign22980_e25250: f64 = (assign22980_e25249).sqrt();
            let assign22980_e25251: f64 = (assign22980_e25236 - assign22980_e25250);
            let assign22980_e25252: f64 = (0.5 * assign22980_e25251);
            locals.var_vav = assign22980_e25252;
        }
        let assign22990_e25257: f64 = if locals.var_absource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign22990_e25257;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 != 0.0)) {
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) {
            let assign23010_e25274: f64 = (locals.var_idsatbot * locals.var_idmult);
            locals.var_id__blk219 = assign23010_e25274;
        }
        let assign23020_e25283: f64 = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard422 = assign23020_e25283;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 == 0.0)) {
            let assign23040_e25306: f64 = (locals.var_vbibot - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign23040_e25306;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 == 0.0)) {
            let assign23050_e25322: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign23050_e25323: f64 = (1.0 - assign23050_e25322);
            let assign23050_e25324: f64 = (assign23050_e25323).sqrt();
            let assign23050_e25325: f64 = (1.0 - assign23050_e25324);
            locals.var_wsrhstep = assign23050_e25325;
        }
        let assign23060_e25330: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign23060_e25330;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 == 0.0)) && (locals.var_guard423 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 == 0.0)) && (locals.var_guard423 == 0.0)) {
            let assign23080_e25359: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign23080_e25361: f64 = (locals.var_wsrhstep).ln();
            let assign23080_e25362: f64 = (assign23080_e25359 * assign23080_e25361);
            let assign23080_e25365: f64 = (1.0 - locals.var_wsrhstep);
            let assign23080_e25366: f64 = (assign23080_e25362 / assign23080_e25365);
            let assign23080_e25368: f64 = (assign23080_e25366 + locals.var_wsrhstep);
            let assign23080_e25372: f64 = (2.0 * p.p831);
            let assign23080_e25373: f64 = (1.0 - assign23080_e25372);
            let assign23080_e25374: f64 = (assign23080_e25368 * assign23080_e25373);
            locals.var_dwsrh = assign23080_e25374;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 == 0.0)) {
            let assign23090_e25388: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign23090_e25388;
        }
        let assign23100_e25393: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard424 = assign23100_e25393;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 == 0.0)) && (locals.var_guard424 != 0.0)) {
            let assign23110_e25407: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv);
            let assign23110_e25408: f64 = (assign23110_e25407).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23110_e25408, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 == 0.0)) && (locals.var_guard424 == 0.0)) {
            let assign23120_e25425: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv);
            let assign23120_e25427: f64 = (assign23120_e25425).powf(p.p831);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23120_e25427, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 == 0.0)) {
            let assign23130_e25441: f64 = (locals.var_wdepnulrbot * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign23130_e25441, (locals.var_wdepnulrbot * locals.var_tmp_dn5), (locals.var_wdepnulrbot * locals.var_tmp_dn6), (locals.var_wdepnulrbot * locals.var_tmp_dn7), (locals.var_wdepnulrbot * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 == 0.0)) {
            let assign23140_e25456: f64 = (locals.var_zinv - 1.0);
            let assign23140_e25458: f64 = (assign23140_e25456 * locals.var_wdep);
            let assign23140_e25459: f64 = (locals.var_ftdbot * assign23140_e25458);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign23140_e25459, (locals.var_ftdbot * (assign23140_e25456 * locals.var_wdep_dn5)), (locals.var_ftdbot * (assign23140_e25456 * locals.var_wdep_dn6)), (locals.var_ftdbot * (assign23140_e25456 * locals.var_wdep_dn7)), (locals.var_ftdbot * (assign23140_e25456 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 == 0.0)) {
            let assign23150_e25474: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign23150_e25475: f64 = (p.p840 * assign23150_e25474);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign23150_e25475, (p.p840 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign23160_e25480: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard425 = assign23160_e25480;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23180_e25504: f64 = (locals.var_wdep * locals.var_one_minus_pbot);
            let assign23180_e25506: f64 = (assign23180_e25504 / locals.var_vbi_minus_vjsrh);
            let assign23180_e25507: f64 = (locals.var_btatpartbot * assign23180_e25506);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign23180_e25507, (locals.var_btatpartbot * ((locals.var_wdep_dn5 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn6 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn7 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn8 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23190_e25521: f64 = (0.666666666666667 * locals.var_atatbot);
            let assign23190_e25523: f64 = (assign23190_e25521 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign23190_e25523, (-((assign23190_e25521 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign23190_e25521 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign23190_e25521 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign23190_e25521 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23200_e25537: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign23200_e25537, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23210_e25551: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign23210_e25554: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign23210_e25556: f64 = (assign23210_e25554 + 1.0);
            let assign23210_e25557: f64 = (assign23210_e25551 / assign23210_e25556);
            let assign23210_e25558: f64 = (assign23210_e25557).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign23210_e25558, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign23210_e25556) - (assign23210_e25551 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign23210_e25556 * assign23210_e25556)) / (2.0 * assign23210_e25558)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign23210_e25556) - (assign23210_e25551 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign23210_e25556 * assign23210_e25556)) / (2.0 * assign23210_e25558)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign23210_e25556) - (assign23210_e25551 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign23210_e25556 * assign23210_e25556)) / (2.0 * assign23210_e25558)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign23210_e25556) - (assign23210_e25551 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign23210_e25556 * assign23210_e25556)) / (2.0 * assign23210_e25558)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23220_e25571: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign23220_e25571, (locals.var_umax_dn5 / (2.0 * assign23220_e25571)), (locals.var_umax_dn6 / (2.0 * assign23220_e25571)), (locals.var_umax_dn7 / (2.0 * assign23220_e25571)), (locals.var_umax_dn8 / (2.0 * assign23220_e25571)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23230_e25585: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign23230_e25585, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign23240_e25589: f64 = (-p.p831);
        let assign23240_e25591: f64 = (assign23240_e25589 * locals.var_one_over_one_minus_pbot);
        let assign23240_e25593: f64 = (-1.0);
        let assign23240_e25594: f64 = if assign23240_e25591 == assign23240_e25593 { 1.0 } else { 0.0 };
        locals.var_guard426 = assign23240_e25594;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard426 != 0.0)) {
            let assign23250_e25610: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign23250_e25611: f64 = (1.0 + assign23250_e25610);
            let assign23250_e25612: f64 = (1.0 / assign23250_e25611);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign23250_e25612, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign23250_e25611 * assign23250_e25611))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign23250_e25611 * assign23250_e25611))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign23250_e25611 * assign23250_e25611))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign23250_e25611 * assign23250_e25611))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard426 == 0.0)) {
            let assign23260_e25630: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign23260_e25631: f64 = (1.0 + assign23260_e25630);
            let assign23260_e25633: f64 = (-p.p831);
            let assign23260_e25635: f64 = (assign23260_e25633 * locals.var_one_over_one_minus_pbot);
            let assign23260_e25636: f64 = (assign23260_e25631).powf(assign23260_e25635);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign23260_e25636, if 0.0 == 0.0 && ((assign23260_e25635) as f64).is_finite() && ((assign23260_e25635) as f64).fract() == 0.0 { if assign23260_e25635 == 0.0 { 0.0 } else { (assign23260_e25635 * ((assign23260_e25631).powf(assign23260_e25635 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign23260_e25636 * (assign23260_e25635 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign23260_e25631))) }, if 0.0 == 0.0 && ((assign23260_e25635) as f64).is_finite() && ((assign23260_e25635) as f64).fract() == 0.0 { if assign23260_e25635 == 0.0 { 0.0 } else { (assign23260_e25635 * ((assign23260_e25631).powf(assign23260_e25635 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign23260_e25636 * (assign23260_e25635 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign23260_e25631))) }, if 0.0 == 0.0 && ((assign23260_e25635) as f64).is_finite() && ((assign23260_e25635) as f64).fract() == 0.0 { if assign23260_e25635 == 0.0 { 0.0 } else { (assign23260_e25635 * ((assign23260_e25631).powf(assign23260_e25635 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign23260_e25636 * (assign23260_e25635 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign23260_e25631))) }, if 0.0 == 0.0 && ((assign23260_e25635) as f64).is_finite() && ((assign23260_e25635) as f64).fract() == 0.0 { if assign23260_e25635 == 0.0 { 0.0 } else { (assign23260_e25635 * ((assign23260_e25631).powf(assign23260_e25635 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign23260_e25636 * (assign23260_e25635 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign23260_e25631))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23270_e25650: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign23270_e25653: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign23270_e25654: f64 = (assign23270_e25650 / assign23270_e25653);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign23270_e25654, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign23270_e25653) - (assign23270_e25650 * locals.var_wgamma_dn5)) / (assign23270_e25653 * assign23270_e25653)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign23270_e25653) - (assign23270_e25650 * locals.var_wgamma_dn6)) / (assign23270_e25653 * assign23270_e25653)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign23270_e25653) - (assign23270_e25650 * locals.var_wgamma_dn7)) / (assign23270_e25653 * assign23270_e25653)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign23270_e25653) - (assign23270_e25650 * locals.var_wgamma_dn8)) / (assign23270_e25653 * assign23270_e25653)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23280_e25669: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign23280_e25670: f64 = (0.375 * assign23280_e25669);
            let assign23280_e25671: f64 = (assign23280_e25670).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign23280_e25671, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign23280_e25671)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign23280_e25671)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign23280_e25671)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign23280_e25671)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23290_e25686: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign23290_e25687: f64 = (2.0 * assign23290_e25686);
            let assign23290_e25689: f64 = (assign23290_e25687 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign23290_e25689, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23300_e25703: f64 = (locals.var_atatbot * locals.var_twoatatoverthreebtat);
            let assign23300_e25705: f64 = (assign23300_e25703 * locals.var_sqrtumax);
            let assign23300_e25708: f64 = (locals.var_atatbot * locals.var_umax);
            let assign23300_e25709: f64 = (assign23300_e25705 - assign23300_e25708);
            let assign23300_e25713: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign23300_e25714: f64 = (0.5 * assign23300_e25713);
            let assign23300_e25715: f64 = (assign23300_e25709 + assign23300_e25714);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign23300_e25715, (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign23300_e25703 * locals.var_sqrtumax_dn5)) - (locals.var_atatbot * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign23300_e25703 * locals.var_sqrtumax_dn6)) - (locals.var_atatbot * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign23300_e25703 * locals.var_sqrtumax_dn7)) - (locals.var_atatbot * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign23300_e25703 * locals.var_sqrtumax_dn8)) - (locals.var_atatbot * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23310_e25729: f64 = (locals.var_ltat - 1.0);
            let assign23310_e25731: f64 = (assign23310_e25729 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign23310_e25731, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign23310_e25729 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign23310_e25729 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign23310_e25729 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign23310_e25729 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23320_e25745: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign23320_e25745, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign23330_e25750: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard427 = assign23330_e25750;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard427 != 0.0)) {
            let assign23340_e25766: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign23340_e25767: f64 = (1.0 + assign23340_e25766);
            let assign23340_e25768: f64 = (1.0 / assign23340_e25767);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign23340_e25768, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign23340_e25767 * assign23340_e25767))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign23340_e25767 * assign23340_e25767))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign23340_e25767 * assign23340_e25767))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign23340_e25767 * assign23340_e25767))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard427 == 0.0)) {
            let assign23350_e25787: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign23350_e25788: f64 = (1.0 - assign23350_e25787);
            let assign23350_e25789: f64 = (1.0 / assign23350_e25788);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign23350_e25789, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign23350_e25788 * assign23350_e25788))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign23350_e25788 * assign23350_e25788))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign23350_e25788 * assign23350_e25788))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign23350_e25788 * assign23350_e25788))), );
        }
        let assign23360_e25793: f64 = (-locals.var_ysq);
        let assign23360_e25795: f64 = (assign23360_e25793 + locals.var_mtat);
        let assign23360_e25797: f64 = (-230.25850929940458);
        let assign23360_e25798: f64 = if assign23360_e25795 > assign23360_e25797 { 1.0 } else { 0.0 };
        locals.var_guard428 = assign23360_e25798;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard428 != 0.0)) {
            let assign23370_e25811: f64 = (-locals.var_ysq);
            let assign23370_e25813: f64 = (assign23370_e25811 + locals.var_mtat);
            let assign23370_e25814: f64 = (assign23370_e25813).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23370_e25814, (assign23370_e25814 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign23370_e25814 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign23370_e25814 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign23370_e25814 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard428 == 0.0)) {
            let assign23380_e25832: f64 = (-230.25850929940458);
            let assign23380_e25834: f64 = (-locals.var_ysq);
            let assign23380_e25836: f64 = (assign23380_e25834 + locals.var_mtat);
            let assign23380_e25837: f64 = (assign23380_e25832 - assign23380_e25836);
            let assign23380_e25841: f64 = (-230.25850929940458);
            let assign23380_e25843: f64 = (-locals.var_ysq);
            let assign23380_e25845: f64 = (assign23380_e25843 + locals.var_mtat);
            let assign23380_e25846: f64 = (assign23380_e25841 - assign23380_e25845);
            let assign23380_e25849: f64 = (-230.25850929940458);
            let assign23380_e25851: f64 = (-locals.var_ysq);
            let assign23380_e25853: f64 = (assign23380_e25851 + locals.var_mtat);
            let assign23380_e25854: f64 = (assign23380_e25849 - assign23380_e25853);
            let assign23380_e25856: f64 = (assign23380_e25854 * 0.3333333333333333);
            let assign23380_e25857: f64 = (1.0 + assign23380_e25856);
            let assign23380_e25858: f64 = (assign23380_e25846 * assign23380_e25857);
            let assign23380_e25859: f64 = (0.5 * assign23380_e25858);
            let assign23380_e25860: f64 = (1.0 + assign23380_e25859);
            let assign23380_e25861: f64 = (assign23380_e25837 * assign23380_e25860);
            let assign23380_e25862: f64 = (1.0 + assign23380_e25861);
            let assign23380_e25863: f64 = (1e-100 / assign23380_e25862);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23380_e25863, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign23380_e25860) + (assign23380_e25837 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign23380_e25857) + (assign23380_e25846 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign23380_e25862 * assign23380_e25862))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign23380_e25860) + (assign23380_e25837 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign23380_e25857) + (assign23380_e25846 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign23380_e25862 * assign23380_e25862))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign23380_e25860) + (assign23380_e25837 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign23380_e25857) + (assign23380_e25846 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign23380_e25862 * assign23380_e25862))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign23380_e25860) + (assign23380_e25837 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign23380_e25857) + (assign23380_e25846 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign23380_e25862 * assign23380_e25862))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23390_e25877: f64 = (0.29214664 * locals.var_terfc);
            let assign23390_e25881: f64 = (locals.var_terfc * locals.var_terfc);
            let assign23390_e25882: f64 = (locals.var_berfc * assign23390_e25881);
            let assign23390_e25883: f64 = (assign23390_e25877 + assign23390_e25882);
            let assign23390_e25887: f64 = (locals.var_terfc * locals.var_terfc);
            let assign23390_e25889: f64 = (assign23390_e25887 * locals.var_terfc);
            let assign23390_e25890: f64 = (locals.var_cerfc * assign23390_e25889);
            let assign23390_e25891: f64 = (assign23390_e25883 + assign23390_e25890);
            let assign23390_e25893: f64 = (assign23390_e25891 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign23390_e25893, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign23390_e25887 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign23390_e25891 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign23390_e25887 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign23390_e25891 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign23390_e25887 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign23390_e25891 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign23390_e25887 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign23390_e25891 * locals.var_tmp_dn8)), );
        }
        let assign23400_e25898: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard429 = assign23400_e25898;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard429 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign23420_e25915: f64 = (-230.25850929940458);
        let assign23420_e25916: f64 = if locals.var_mtat > assign23420_e25915 { 1.0 } else { 0.0 };
        locals.var_guard430 = assign23420_e25916;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard429 == 0.0)) && (locals.var_guard430 != 0.0)) {
            let assign23430_e25932: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23430_e25932, (assign23430_e25932 * locals.var_mtat_dn5), (assign23430_e25932 * locals.var_mtat_dn6), (assign23430_e25932 * locals.var_mtat_dn7), (assign23430_e25932 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard429 == 0.0)) && (locals.var_guard430 == 0.0)) {
            let assign23440_e25953: f64 = (-230.25850929940458);
            let assign23440_e25955: f64 = (assign23440_e25953 - locals.var_mtat);
            let assign23440_e25959: f64 = (-230.25850929940458);
            let assign23440_e25961: f64 = (assign23440_e25959 - locals.var_mtat);
            let assign23440_e25964: f64 = (-230.25850929940458);
            let assign23440_e25966: f64 = (assign23440_e25964 - locals.var_mtat);
            let assign23440_e25968: f64 = (assign23440_e25966 * 0.3333333333333333);
            let assign23440_e25969: f64 = (1.0 + assign23440_e25968);
            let assign23440_e25970: f64 = (assign23440_e25961 * assign23440_e25969);
            let assign23440_e25971: f64 = (0.5 * assign23440_e25970);
            let assign23440_e25972: f64 = (1.0 + assign23440_e25971);
            let assign23440_e25973: f64 = (assign23440_e25955 * assign23440_e25972);
            let assign23440_e25974: f64 = (1.0 + assign23440_e25973);
            let assign23440_e25975: f64 = (1e-100 / assign23440_e25974);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23440_e25975, (-((1e-100 * (((-locals.var_mtat_dn5) * assign23440_e25972) + (assign23440_e25955 * (0.5 * (((-locals.var_mtat_dn5) * assign23440_e25969) + (assign23440_e25961 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign23440_e25974 * assign23440_e25974))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign23440_e25972) + (assign23440_e25955 * (0.5 * (((-locals.var_mtat_dn6) * assign23440_e25969) + (assign23440_e25961 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign23440_e25974 * assign23440_e25974))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign23440_e25972) + (assign23440_e25955 * (0.5 * (((-locals.var_mtat_dn7) * assign23440_e25969) + (assign23440_e25961 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign23440_e25974 * assign23440_e25974))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign23440_e25972) + (assign23440_e25955 * (0.5 * (((-locals.var_mtat_dn8) * assign23440_e25969) + (assign23440_e25961 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign23440_e25974 * assign23440_e25974))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard429 == 0.0)) {
            let assign23450_e25992: f64 = (2.0 * locals.var_tmp);
            let assign23450_e25994: f64 = (assign23450_e25992 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign23450_e25994, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23460_e26008: f64 = (1.772453850905516 * 0.5);
            let assign23460_e26011: f64 = (locals.var_atatbot * locals.var_erfctimesexpmtat);
            let assign23460_e26013: f64 = (assign23460_e26011 / locals.var_ktat);
            let assign23460_e26014: f64 = (assign23460_e26008 * assign23460_e26013);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign23460_e26014, (assign23460_e26008 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign23460_e26011 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign23460_e26008 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign23460_e26011 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign23460_e26008 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign23460_e26011 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign23460_e26008 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign23460_e26011 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard425 == 0.0)) {
            let assign23470_e26029: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign23470_e26031: f64 = (assign23470_e26029 * locals.var_wtat);
            let assign23470_e26032: f64 = (p.p845 * assign23470_e26031);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign23470_e26032, (p.p845 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign23470_e26029 * locals.var_wtat_dn5))), (p.p845 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign23470_e26029 * locals.var_wtat_dn6))), (p.p845 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign23470_e26029 * locals.var_wtat_dn7))), (p.p845 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign23470_e26029 * locals.var_wtat_dn8))), );
        }
        let assign23480_e26037: f64 = if p.p851 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard431 = assign23480_e26037;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard431 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign23500_e26051: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign23500_e26051;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard432 != 0.0)) {
            let assign23510_e26065: f64 = (p.p828 - locals.var_vbbt);
            let assign23510_e26067: f64 = (assign23510_e26065 * locals.var_vbirbotinv);
            let assign23510_e26068: f64 = (assign23510_e26067).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23510_e26068, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard432 == 0.0)) {
            let assign23520_e26085: f64 = (p.p828 - locals.var_vbbt);
            let assign23520_e26087: f64 = (assign23520_e26085 * locals.var_vbirbotinv);
            let assign23520_e26089: f64 = (assign23520_e26087).powf(p.p831);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23520_e26089, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard431 == 0.0)) {
            let assign23530_e26104: f64 = (p.p828 - locals.var_vbbt);
            let assign23530_e26106: f64 = (assign23530_e26104 * locals.var_wdepnulrinvbot);
            let assign23530_e26108: f64 = (assign23530_e26106 / locals.var_tmp);
            let assign23530_e26109: f64 = (locals.var_one_over_one_minus_pbot * assign23530_e26108);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign23530_e26109, (locals.var_one_over_one_minus_pbot * (-((assign23530_e26106 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign23530_e26106 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign23530_e26106 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign23530_e26106 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign23540_e26113: f64 = (-locals.var_fbbtbot);
        let assign23540_e26115: f64 = (assign23540_e26113 / locals.var_fmaxr);
        let assign23540_e26116: f64 = (assign23540_e26115).abs();
        let assign23540_e26118: f64 = if assign23540_e26116 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard433 = assign23540_e26118;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard433 != 0.0)) {
            let assign23550_e26131: f64 = (-locals.var_fbbtbot);
            let assign23550_e26133: f64 = (assign23550_e26131 / locals.var_fmaxr);
            let assign23550_e26134: f64 = (assign23550_e26133).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23550_e26134, (assign23550_e26134 * (-((assign23550_e26131 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign23550_e26134 * (-((assign23550_e26131 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign23550_e26134 * (-((assign23550_e26131 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign23550_e26134 * (-((assign23550_e26131 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign23560_e26138: f64 = (-locals.var_fbbtbot);
        let assign23560_e26140: f64 = (assign23560_e26138 / locals.var_fmaxr);
        let assign23560_e26142: f64 = if assign23560_e26140 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard434 = assign23560_e26142;
    }
    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard433 == 0.0)) && (locals.var_guard434 != 0.0)) {
            let assign23570_e26160: f64 = (-230.25850929940458);
            let assign23570_e26162: f64 = (-locals.var_fbbtbot);
            let assign23570_e26164: f64 = (assign23570_e26162 / locals.var_fmaxr);
            let assign23570_e26165: f64 = (assign23570_e26160 - assign23570_e26164);
            let assign23570_e26169: f64 = (-230.25850929940458);
            let assign23570_e26171: f64 = (-locals.var_fbbtbot);
            let assign23570_e26173: f64 = (assign23570_e26171 / locals.var_fmaxr);
            let assign23570_e26174: f64 = (assign23570_e26169 - assign23570_e26173);
            let assign23570_e26177: f64 = (-230.25850929940458);
            let assign23570_e26179: f64 = (-locals.var_fbbtbot);
            let assign23570_e26181: f64 = (assign23570_e26179 / locals.var_fmaxr);
            let assign23570_e26182: f64 = (assign23570_e26177 - assign23570_e26181);
            let assign23570_e26184: f64 = (assign23570_e26182 * 0.3333333333333333);
            let assign23570_e26185: f64 = (1.0 + assign23570_e26184);
            let assign23570_e26186: f64 = (assign23570_e26174 * assign23570_e26185);
            let assign23570_e26187: f64 = (0.5 * assign23570_e26186);
            let assign23570_e26188: f64 = (1.0 + assign23570_e26187);
            let assign23570_e26189: f64 = (assign23570_e26165 * assign23570_e26188);
            let assign23570_e26190: f64 = (1.0 + assign23570_e26189);
            let assign23570_e26191: f64 = (1e-100 / assign23570_e26190);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23570_e26191, (-((1e-100 * (((-(-((assign23570_e26162 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign23570_e26188) + (assign23570_e26165 * (0.5 * (((-(-((assign23570_e26171 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign23570_e26185) + (assign23570_e26174 * ((-(-((assign23570_e26179 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign23570_e26190 * assign23570_e26190))), (-((1e-100 * (((-(-((assign23570_e26162 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign23570_e26188) + (assign23570_e26165 * (0.5 * (((-(-((assign23570_e26171 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign23570_e26185) + (assign23570_e26174 * ((-(-((assign23570_e26179 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign23570_e26190 * assign23570_e26190))), (-((1e-100 * (((-(-((assign23570_e26162 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign23570_e26188) + (assign23570_e26165 * (0.5 * (((-(-((assign23570_e26171 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign23570_e26185) + (assign23570_e26174 * ((-(-((assign23570_e26179 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign23570_e26190 * assign23570_e26190))), (-((1e-100 * (((-(-((assign23570_e26162 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign23570_e26188) + (assign23570_e26165 * (0.5 * (((-(-((assign23570_e26171 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign23570_e26185) + (assign23570_e26174 * ((-(-((assign23570_e26179 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign23570_e26190 * assign23570_e26190))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard433 == 0.0)) && (locals.var_guard434 == 0.0)) {
            let assign23580_e26212: f64 = (-locals.var_fbbtbot);
            let assign23580_e26214: f64 = (assign23580_e26212 / locals.var_fmaxr);
            let assign23580_e26216: f64 = (assign23580_e26214 - 230.25850929940458);
            let assign23580_e26220: f64 = (-locals.var_fbbtbot);
            let assign23580_e26222: f64 = (assign23580_e26220 / locals.var_fmaxr);
            let assign23580_e26224: f64 = (assign23580_e26222 - 230.25850929940458);
            let assign23580_e26227: f64 = (-locals.var_fbbtbot);
            let assign23580_e26229: f64 = (assign23580_e26227 / locals.var_fmaxr);
            let assign23580_e26231: f64 = (assign23580_e26229 - 230.25850929940458);
            let assign23580_e26233: f64 = (assign23580_e26231 * 0.3333333333333333);
            let assign23580_e26234: f64 = (1.0 + assign23580_e26233);
            let assign23580_e26235: f64 = (assign23580_e26224 * assign23580_e26234);
            let assign23580_e26236: f64 = (0.5 * assign23580_e26235);
            let assign23580_e26237: f64 = (1.0 + assign23580_e26236);
            let assign23580_e26238: f64 = (assign23580_e26216 * assign23580_e26237);
            let assign23580_e26239: f64 = (1.0 + assign23580_e26238);
            let assign23580_e26240: f64 = (1e100 * assign23580_e26239);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23580_e26240, (1e100 * (((-((assign23580_e26212 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign23580_e26237) + (assign23580_e26216 * (0.5 * (((-((assign23580_e26220 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign23580_e26234) + (assign23580_e26224 * ((-((assign23580_e26227 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign23580_e26212 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign23580_e26237) + (assign23580_e26216 * (0.5 * (((-((assign23580_e26220 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign23580_e26234) + (assign23580_e26224 * ((-((assign23580_e26227 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign23580_e26212 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign23580_e26237) + (assign23580_e26216 * (0.5 * (((-((assign23580_e26220 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign23580_e26234) + (assign23580_e26224 * ((-((assign23580_e26227 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign23580_e26212 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign23580_e26237) + (assign23580_e26216 * (0.5 * (((-((assign23580_e26220 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign23580_e26234) + (assign23580_e26224 * ((-((assign23580_e26227 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard431 == 0.0)) {
            let assign23590_e26255: f64 = (locals.var_v4 * locals.var_fmaxr);
            let assign23590_e26257: f64 = (assign23590_e26255 * locals.var_fmaxr);
            let assign23590_e26259: f64 = (assign23590_e26257 * locals.var_tmp);
            let assign23590_e26260: f64 = (p.p851 * assign23590_e26259);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign23590_e26260, (p.p851 * (((((locals.var_v4 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign23590_e26255 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign23590_e26257 * locals.var_tmp_dn5))), (p.p851 * (((((locals.var_v4 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign23590_e26255 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign23590_e26257 * locals.var_tmp_dn6))), (p.p851 * (((((locals.var_v4 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign23590_e26255 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign23590_e26257 * locals.var_tmp_dn7))), (p.p851 * (((((locals.var_v4 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign23590_e26255 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign23590_e26257 * locals.var_tmp_dn8))), );
        }
        let assign23600_e26265: f64 = if p.p860 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard435 = assign23600_e26265;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard435 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign23620_e26279: f64 = (-locals.var_alphaav);
        let assign23620_e26281: f64 = (assign23620_e26279 * p.p860);
        let assign23620_e26282: f64 = if locals.var_vav > assign23620_e26281 { 1.0 } else { 0.0 };
        locals.var_guard436 = assign23620_e26282;
        let assign23630_e26285: f64 = if p.p863 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard437 = assign23630_e26285;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard435 == 0.0)) && (locals.var_guard436 != 0.0)) && (locals.var_guard437 != 0.0)) {
            let assign23640_e26301: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign23640_e26304: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign23640_e26305: f64 = (assign23640_e26301 * assign23640_e26304);
            let assign23640_e26308: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign23640_e26309: f64 = (assign23640_e26305 * assign23640_e26308);
            let assign23640_e26312: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign23640_e26313: f64 = (assign23640_e26309 * assign23640_e26312);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23640_e26313, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard435 == 0.0)) && (locals.var_guard436 != 0.0)) && (locals.var_guard437 == 0.0)) {
            let assign23650_e26332: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign23650_e26333: f64 = (assign23650_e26332).abs();
            let assign23650_e26335: f64 = (assign23650_e26333).powf(p.p863);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23650_e26335, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard435 == 0.0)) && (locals.var_guard436 != 0.0)) {
            let assign23660_e26352: f64 = (1.0 - locals.var_tmp);
            let assign23660_e26353: f64 = (1.0 / assign23660_e26352);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign23660_e26353, (-((-locals.var_tmp_dn5) / (assign23660_e26352 * assign23660_e26352))), (-((-locals.var_tmp_dn6) / (assign23660_e26352 * assign23660_e26352))), (-((-locals.var_tmp_dn7) / (assign23660_e26352 * assign23660_e26352))), (-((-locals.var_tmp_dn8) / (assign23660_e26352 * assign23660_e26352))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard435 == 0.0)) && (locals.var_guard436 == 0.0)) {
            let assign23670_e26372: f64 = (locals.var_alphaav * p.p860);
            let assign23670_e26373: f64 = (locals.var_vav + assign23670_e26372);
            let assign23670_e26375: f64 = (assign23670_e26373 * locals.var_slopebot);
            let assign23670_e26376: f64 = (locals.var_fstopbot + assign23670_e26375);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign23670_e26376, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard421 == 0.0)) {
            let assign23680_e26388: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign23680_e26390: f64 = (assign23680_e26388 + locals.var_itat);
            let assign23680_e26392: f64 = (assign23680_e26390 + locals.var_ibbt);
            let assign23680_e26393: f64 = (p.p29 * assign23680_e26392);
            let assign23680_e26395: f64 = (assign23680_e26393 * locals.var_fbreakdown);
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (assign23680_e26395, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign23680_e26393 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign23680_e26393 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign23680_e26393 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign23680_e26393 * locals.var_fbreakdown_dn8)), );
        }
        let assign23690_e26400: f64 = if locals.var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard438 = assign23690_e26400;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 != 0.0)) {
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) {
            let assign23710_e26417: f64 = (locals.var_idsatsti * locals.var_idmult);
            locals.var_id__blk219 = assign23710_e26417;
        }
        let assign23720_e26426: f64 = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard439 = assign23720_e26426;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard439 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard439 == 0.0)) {
            let assign23740_e26449: f64 = (locals.var_vbisti - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign23740_e26449;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard439 == 0.0)) {
            let assign23750_e26465: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign23750_e26466: f64 = (1.0 - assign23750_e26465);
            let assign23750_e26467: f64 = (assign23750_e26466).sqrt();
            let assign23750_e26468: f64 = (1.0 - assign23750_e26467);
            locals.var_wsrhstep = assign23750_e26468;
        }
        let assign23760_e26473: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard440 = assign23760_e26473;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard439 == 0.0)) && (locals.var_guard440 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard439 == 0.0)) && (locals.var_guard440 == 0.0)) {
            let assign23780_e26502: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign23780_e26504: f64 = (locals.var_wsrhstep).ln();
            let assign23780_e26505: f64 = (assign23780_e26502 * assign23780_e26504);
            let assign23780_e26508: f64 = (1.0 - locals.var_wsrhstep);
            let assign23780_e26509: f64 = (assign23780_e26505 / assign23780_e26508);
            let assign23780_e26511: f64 = (assign23780_e26509 + locals.var_wsrhstep);
            let assign23780_e26515: f64 = (2.0 * p.p832);
            let assign23780_e26516: f64 = (1.0 - assign23780_e26515);
            let assign23780_e26517: f64 = (assign23780_e26511 * assign23780_e26516);
            locals.var_dwsrh = assign23780_e26517;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard439 == 0.0)) {
            let assign23790_e26531: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign23790_e26531;
        }
        let assign23800_e26536: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard441 = assign23800_e26536;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard439 == 0.0)) && (locals.var_guard441 != 0.0)) {
            let assign23810_e26550: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv);
            let assign23810_e26551: f64 = (assign23810_e26550).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23810_e26551, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard439 == 0.0)) && (locals.var_guard441 == 0.0)) {
            let assign23820_e26568: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv);
            let assign23820_e26570: f64 = (assign23820_e26568).powf(p.p832);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign23820_e26570, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard439 == 0.0)) {
            let assign23830_e26584: f64 = (locals.var_wdepnulrsti * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign23830_e26584, (locals.var_wdepnulrsti * locals.var_tmp_dn5), (locals.var_wdepnulrsti * locals.var_tmp_dn6), (locals.var_wdepnulrsti * locals.var_tmp_dn7), (locals.var_wdepnulrsti * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard439 == 0.0)) {
            let assign23840_e26599: f64 = (locals.var_zinv - 1.0);
            let assign23840_e26601: f64 = (assign23840_e26599 * locals.var_wdep);
            let assign23840_e26602: f64 = (locals.var_ftdsti * assign23840_e26601);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign23840_e26602, (locals.var_ftdsti * (assign23840_e26599 * locals.var_wdep_dn5)), (locals.var_ftdsti * (assign23840_e26599 * locals.var_wdep_dn6)), (locals.var_ftdsti * (assign23840_e26599 * locals.var_wdep_dn7)), (locals.var_ftdsti * (assign23840_e26599 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard439 == 0.0)) {
            let assign23850_e26617: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign23850_e26618: f64 = (p.p841 * assign23850_e26617);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign23850_e26618, (p.p841 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign23860_e26623: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard442 = assign23860_e26623;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign23880_e26647: f64 = (locals.var_wdep * locals.var_one_minus_psti);
            let assign23880_e26649: f64 = (assign23880_e26647 / locals.var_vbi_minus_vjsrh);
            let assign23880_e26650: f64 = (locals.var_btatpartsti * assign23880_e26649);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign23880_e26650, (locals.var_btatpartsti * ((locals.var_wdep_dn5 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn6 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn7 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn8 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign23890_e26664: f64 = (0.666666666666667 * locals.var_atatsti);
            let assign23890_e26666: f64 = (assign23890_e26664 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign23890_e26666, (-((assign23890_e26664 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign23890_e26664 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign23890_e26664 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign23890_e26664 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign23900_e26680: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign23900_e26680, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign23910_e26694: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign23910_e26697: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign23910_e26699: f64 = (assign23910_e26697 + 1.0);
            let assign23910_e26700: f64 = (assign23910_e26694 / assign23910_e26699);
            let assign23910_e26701: f64 = (assign23910_e26700).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign23910_e26701, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign23910_e26699) - (assign23910_e26694 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign23910_e26699 * assign23910_e26699)) / (2.0 * assign23910_e26701)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign23910_e26699) - (assign23910_e26694 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign23910_e26699 * assign23910_e26699)) / (2.0 * assign23910_e26701)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign23910_e26699) - (assign23910_e26694 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign23910_e26699 * assign23910_e26699)) / (2.0 * assign23910_e26701)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign23910_e26699) - (assign23910_e26694 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign23910_e26699 * assign23910_e26699)) / (2.0 * assign23910_e26701)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign23920_e26714: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign23920_e26714, (locals.var_umax_dn5 / (2.0 * assign23920_e26714)), (locals.var_umax_dn6 / (2.0 * assign23920_e26714)), (locals.var_umax_dn7 / (2.0 * assign23920_e26714)), (locals.var_umax_dn8 / (2.0 * assign23920_e26714)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign23930_e26728: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign23930_e26728, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign23940_e26732: f64 = (-p.p832);
        let assign23940_e26734: f64 = (assign23940_e26732 * locals.var_one_over_one_minus_psti);
        let assign23940_e26736: f64 = (-1.0);
        let assign23940_e26737: f64 = if assign23940_e26734 == assign23940_e26736 { 1.0 } else { 0.0 };
        locals.var_guard443 = assign23940_e26737;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
            let assign23950_e26753: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign23950_e26754: f64 = (1.0 + assign23950_e26753);
            let assign23950_e26755: f64 = (1.0 / assign23950_e26754);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign23950_e26755, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign23950_e26754 * assign23950_e26754))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign23950_e26754 * assign23950_e26754))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign23950_e26754 * assign23950_e26754))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign23950_e26754 * assign23950_e26754))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) {
            let assign23960_e26773: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign23960_e26774: f64 = (1.0 + assign23960_e26773);
            let assign23960_e26776: f64 = (-p.p832);
            let assign23960_e26778: f64 = (assign23960_e26776 * locals.var_one_over_one_minus_psti);
            let assign23960_e26779: f64 = (assign23960_e26774).powf(assign23960_e26778);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign23960_e26779, if 0.0 == 0.0 && ((assign23960_e26778) as f64).is_finite() && ((assign23960_e26778) as f64).fract() == 0.0 { if assign23960_e26778 == 0.0 { 0.0 } else { (assign23960_e26778 * ((assign23960_e26774).powf(assign23960_e26778 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign23960_e26779 * (assign23960_e26778 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign23960_e26774))) }, if 0.0 == 0.0 && ((assign23960_e26778) as f64).is_finite() && ((assign23960_e26778) as f64).fract() == 0.0 { if assign23960_e26778 == 0.0 { 0.0 } else { (assign23960_e26778 * ((assign23960_e26774).powf(assign23960_e26778 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign23960_e26779 * (assign23960_e26778 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign23960_e26774))) }, if 0.0 == 0.0 && ((assign23960_e26778) as f64).is_finite() && ((assign23960_e26778) as f64).fract() == 0.0 { if assign23960_e26778 == 0.0 { 0.0 } else { (assign23960_e26778 * ((assign23960_e26774).powf(assign23960_e26778 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign23960_e26779 * (assign23960_e26778 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign23960_e26774))) }, if 0.0 == 0.0 && ((assign23960_e26778) as f64).is_finite() && ((assign23960_e26778) as f64).fract() == 0.0 { if assign23960_e26778 == 0.0 { 0.0 } else { (assign23960_e26778 * ((assign23960_e26774).powf(assign23960_e26778 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign23960_e26779 * (assign23960_e26778 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign23960_e26774))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign23970_e26793: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign23970_e26796: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign23970_e26797: f64 = (assign23970_e26793 / assign23970_e26796);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign23970_e26797, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign23970_e26796) - (assign23970_e26793 * locals.var_wgamma_dn5)) / (assign23970_e26796 * assign23970_e26796)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign23970_e26796) - (assign23970_e26793 * locals.var_wgamma_dn6)) / (assign23970_e26796 * assign23970_e26796)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign23970_e26796) - (assign23970_e26793 * locals.var_wgamma_dn7)) / (assign23970_e26796 * assign23970_e26796)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign23970_e26796) - (assign23970_e26793 * locals.var_wgamma_dn8)) / (assign23970_e26796 * assign23970_e26796)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign23980_e26812: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign23980_e26813: f64 = (0.375 * assign23980_e26812);
            let assign23980_e26814: f64 = (assign23980_e26813).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign23980_e26814, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign23980_e26814)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign23980_e26814)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign23980_e26814)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign23980_e26814)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign23990_e26829: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign23990_e26830: f64 = (2.0 * assign23990_e26829);
            let assign23990_e26832: f64 = (assign23990_e26830 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign23990_e26832, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign24000_e26846: f64 = (locals.var_atatsti * locals.var_twoatatoverthreebtat);
            let assign24000_e26848: f64 = (assign24000_e26846 * locals.var_sqrtumax);
            let assign24000_e26851: f64 = (locals.var_atatsti * locals.var_umax);
            let assign24000_e26852: f64 = (assign24000_e26848 - assign24000_e26851);
            let assign24000_e26856: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign24000_e26857: f64 = (0.5 * assign24000_e26856);
            let assign24000_e26858: f64 = (assign24000_e26852 + assign24000_e26857);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign24000_e26858, (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign24000_e26846 * locals.var_sqrtumax_dn5)) - (locals.var_atatsti * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign24000_e26846 * locals.var_sqrtumax_dn6)) - (locals.var_atatsti * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign24000_e26846 * locals.var_sqrtumax_dn7)) - (locals.var_atatsti * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign24000_e26846 * locals.var_sqrtumax_dn8)) - (locals.var_atatsti * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign24010_e26872: f64 = (locals.var_ltat - 1.0);
            let assign24010_e26874: f64 = (assign24010_e26872 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign24010_e26874, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign24010_e26872 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign24010_e26872 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign24010_e26872 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign24010_e26872 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign24020_e26888: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign24020_e26888, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign24030_e26893: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard444 = assign24030_e26893;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 != 0.0)) {
            let assign24040_e26909: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign24040_e26910: f64 = (1.0 + assign24040_e26909);
            let assign24040_e26911: f64 = (1.0 / assign24040_e26910);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign24040_e26911, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign24040_e26910 * assign24040_e26910))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign24040_e26910 * assign24040_e26910))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign24040_e26910 * assign24040_e26910))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign24040_e26910 * assign24040_e26910))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) {
            let assign24050_e26930: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign24050_e26931: f64 = (1.0 - assign24050_e26930);
            let assign24050_e26932: f64 = (1.0 / assign24050_e26931);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign24050_e26932, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign24050_e26931 * assign24050_e26931))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign24050_e26931 * assign24050_e26931))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign24050_e26931 * assign24050_e26931))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign24050_e26931 * assign24050_e26931))), );
        }
        let assign24060_e26936: f64 = (-locals.var_ysq);
        let assign24060_e26938: f64 = (assign24060_e26936 + locals.var_mtat);
        let assign24060_e26940: f64 = (-230.25850929940458);
        let assign24060_e26941: f64 = if assign24060_e26938 > assign24060_e26940 { 1.0 } else { 0.0 };
        locals.var_guard445 = assign24060_e26941;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard445 != 0.0)) {
            let assign24070_e26954: f64 = (-locals.var_ysq);
            let assign24070_e26956: f64 = (assign24070_e26954 + locals.var_mtat);
            let assign24070_e26957: f64 = (assign24070_e26956).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24070_e26957, (assign24070_e26957 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign24070_e26957 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign24070_e26957 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign24070_e26957 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard445 == 0.0)) {
            let assign24080_e26975: f64 = (-230.25850929940458);
            let assign24080_e26977: f64 = (-locals.var_ysq);
            let assign24080_e26979: f64 = (assign24080_e26977 + locals.var_mtat);
            let assign24080_e26980: f64 = (assign24080_e26975 - assign24080_e26979);
            let assign24080_e26984: f64 = (-230.25850929940458);
            let assign24080_e26986: f64 = (-locals.var_ysq);
            let assign24080_e26988: f64 = (assign24080_e26986 + locals.var_mtat);
            let assign24080_e26989: f64 = (assign24080_e26984 - assign24080_e26988);
            let assign24080_e26992: f64 = (-230.25850929940458);
            let assign24080_e26994: f64 = (-locals.var_ysq);
            let assign24080_e26996: f64 = (assign24080_e26994 + locals.var_mtat);
            let assign24080_e26997: f64 = (assign24080_e26992 - assign24080_e26996);
            let assign24080_e26999: f64 = (assign24080_e26997 * 0.3333333333333333);
            let assign24080_e27000: f64 = (1.0 + assign24080_e26999);
            let assign24080_e27001: f64 = (assign24080_e26989 * assign24080_e27000);
            let assign24080_e27002: f64 = (0.5 * assign24080_e27001);
            let assign24080_e27003: f64 = (1.0 + assign24080_e27002);
            let assign24080_e27004: f64 = (assign24080_e26980 * assign24080_e27003);
            let assign24080_e27005: f64 = (1.0 + assign24080_e27004);
            let assign24080_e27006: f64 = (1e-100 / assign24080_e27005);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24080_e27006, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign24080_e27003) + (assign24080_e26980 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign24080_e27000) + (assign24080_e26989 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign24080_e27005 * assign24080_e27005))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign24080_e27003) + (assign24080_e26980 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign24080_e27000) + (assign24080_e26989 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign24080_e27005 * assign24080_e27005))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign24080_e27003) + (assign24080_e26980 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign24080_e27000) + (assign24080_e26989 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign24080_e27005 * assign24080_e27005))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign24080_e27003) + (assign24080_e26980 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign24080_e27000) + (assign24080_e26989 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign24080_e27005 * assign24080_e27005))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign24090_e27020: f64 = (0.29214664 * locals.var_terfc);
            let assign24090_e27024: f64 = (locals.var_terfc * locals.var_terfc);
            let assign24090_e27025: f64 = (locals.var_berfc * assign24090_e27024);
            let assign24090_e27026: f64 = (assign24090_e27020 + assign24090_e27025);
            let assign24090_e27030: f64 = (locals.var_terfc * locals.var_terfc);
            let assign24090_e27032: f64 = (assign24090_e27030 * locals.var_terfc);
            let assign24090_e27033: f64 = (locals.var_cerfc * assign24090_e27032);
            let assign24090_e27034: f64 = (assign24090_e27026 + assign24090_e27033);
            let assign24090_e27036: f64 = (assign24090_e27034 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign24090_e27036, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign24090_e27030 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign24090_e27034 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign24090_e27030 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign24090_e27034 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign24090_e27030 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign24090_e27034 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign24090_e27030 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign24090_e27034 * locals.var_tmp_dn8)), );
        }
        let assign24100_e27041: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard446 = assign24100_e27041;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard446 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign24120_e27058: f64 = (-230.25850929940458);
        let assign24120_e27059: f64 = if locals.var_mtat > assign24120_e27058 { 1.0 } else { 0.0 };
        locals.var_guard447 = assign24120_e27059;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
            let assign24130_e27075: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24130_e27075, (assign24130_e27075 * locals.var_mtat_dn5), (assign24130_e27075 * locals.var_mtat_dn6), (assign24130_e27075 * locals.var_mtat_dn7), (assign24130_e27075 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) {
            let assign24140_e27096: f64 = (-230.25850929940458);
            let assign24140_e27098: f64 = (assign24140_e27096 - locals.var_mtat);
            let assign24140_e27102: f64 = (-230.25850929940458);
            let assign24140_e27104: f64 = (assign24140_e27102 - locals.var_mtat);
            let assign24140_e27107: f64 = (-230.25850929940458);
            let assign24140_e27109: f64 = (assign24140_e27107 - locals.var_mtat);
            let assign24140_e27111: f64 = (assign24140_e27109 * 0.3333333333333333);
            let assign24140_e27112: f64 = (1.0 + assign24140_e27111);
            let assign24140_e27113: f64 = (assign24140_e27104 * assign24140_e27112);
            let assign24140_e27114: f64 = (0.5 * assign24140_e27113);
            let assign24140_e27115: f64 = (1.0 + assign24140_e27114);
            let assign24140_e27116: f64 = (assign24140_e27098 * assign24140_e27115);
            let assign24140_e27117: f64 = (1.0 + assign24140_e27116);
            let assign24140_e27118: f64 = (1e-100 / assign24140_e27117);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24140_e27118, (-((1e-100 * (((-locals.var_mtat_dn5) * assign24140_e27115) + (assign24140_e27098 * (0.5 * (((-locals.var_mtat_dn5) * assign24140_e27112) + (assign24140_e27104 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign24140_e27117 * assign24140_e27117))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign24140_e27115) + (assign24140_e27098 * (0.5 * (((-locals.var_mtat_dn6) * assign24140_e27112) + (assign24140_e27104 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign24140_e27117 * assign24140_e27117))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign24140_e27115) + (assign24140_e27098 * (0.5 * (((-locals.var_mtat_dn7) * assign24140_e27112) + (assign24140_e27104 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign24140_e27117 * assign24140_e27117))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign24140_e27115) + (assign24140_e27098 * (0.5 * (((-locals.var_mtat_dn8) * assign24140_e27112) + (assign24140_e27104 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign24140_e27117 * assign24140_e27117))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard446 == 0.0)) {
            let assign24150_e27135: f64 = (2.0 * locals.var_tmp);
            let assign24150_e27137: f64 = (assign24150_e27135 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign24150_e27137, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign24160_e27151: f64 = (1.772453850905516 * 0.5);
            let assign24160_e27154: f64 = (locals.var_atatsti * locals.var_erfctimesexpmtat);
            let assign24160_e27156: f64 = (assign24160_e27154 / locals.var_ktat);
            let assign24160_e27157: f64 = (assign24160_e27151 * assign24160_e27156);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign24160_e27157, (assign24160_e27151 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign24160_e27154 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign24160_e27151 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign24160_e27154 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign24160_e27151 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign24160_e27154 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign24160_e27151 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign24160_e27154 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard442 == 0.0)) {
            let assign24170_e27172: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign24170_e27174: f64 = (assign24170_e27172 * locals.var_wtat);
            let assign24170_e27175: f64 = (p.p846 * assign24170_e27174);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign24170_e27175, (p.p846 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign24170_e27172 * locals.var_wtat_dn5))), (p.p846 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign24170_e27172 * locals.var_wtat_dn6))), (p.p846 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign24170_e27172 * locals.var_wtat_dn7))), (p.p846 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign24170_e27172 * locals.var_wtat_dn8))), );
        }
        let assign24180_e27180: f64 = if p.p852 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard448 = assign24180_e27180;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard448 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign24200_e27194: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard449 = assign24200_e27194;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard448 == 0.0)) && (locals.var_guard449 != 0.0)) {
            let assign24210_e27208: f64 = (p.p829 - locals.var_vbbt);
            let assign24210_e27210: f64 = (assign24210_e27208 * locals.var_vbirstiinv);
            let assign24210_e27211: f64 = (assign24210_e27210).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24210_e27211, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard448 == 0.0)) && (locals.var_guard449 == 0.0)) {
            let assign24220_e27228: f64 = (p.p829 - locals.var_vbbt);
            let assign24220_e27230: f64 = (assign24220_e27228 * locals.var_vbirstiinv);
            let assign24220_e27232: f64 = (assign24220_e27230).powf(p.p832);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24220_e27232, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard448 == 0.0)) {
            let assign24230_e27247: f64 = (p.p829 - locals.var_vbbt);
            let assign24230_e27249: f64 = (assign24230_e27247 * locals.var_wdepnulrinvsti);
            let assign24230_e27251: f64 = (assign24230_e27249 / locals.var_tmp);
            let assign24230_e27252: f64 = (locals.var_one_over_one_minus_psti * assign24230_e27251);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign24230_e27252, (locals.var_one_over_one_minus_psti * (-((assign24230_e27249 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign24230_e27249 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign24230_e27249 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign24230_e27249 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign24240_e27256: f64 = (-locals.var_fbbtsti);
        let assign24240_e27258: f64 = (assign24240_e27256 / locals.var_fmaxr);
        let assign24240_e27259: f64 = (assign24240_e27258).abs();
        let assign24240_e27261: f64 = if assign24240_e27259 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard450 = assign24240_e27261;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
            let assign24250_e27274: f64 = (-locals.var_fbbtsti);
            let assign24250_e27276: f64 = (assign24250_e27274 / locals.var_fmaxr);
            let assign24250_e27277: f64 = (assign24250_e27276).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24250_e27277, (assign24250_e27277 * (-((assign24250_e27274 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign24250_e27277 * (-((assign24250_e27274 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign24250_e27277 * (-((assign24250_e27274 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign24250_e27277 * (-((assign24250_e27274 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign24260_e27281: f64 = (-locals.var_fbbtsti);
        let assign24260_e27283: f64 = (assign24260_e27281 / locals.var_fmaxr);
        let assign24260_e27285: f64 = if assign24260_e27283 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard451 = assign24260_e27285;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 == 0.0)) && (locals.var_guard451 != 0.0)) {
            let assign24270_e27303: f64 = (-230.25850929940458);
            let assign24270_e27305: f64 = (-locals.var_fbbtsti);
            let assign24270_e27307: f64 = (assign24270_e27305 / locals.var_fmaxr);
            let assign24270_e27308: f64 = (assign24270_e27303 - assign24270_e27307);
            let assign24270_e27312: f64 = (-230.25850929940458);
            let assign24270_e27314: f64 = (-locals.var_fbbtsti);
            let assign24270_e27316: f64 = (assign24270_e27314 / locals.var_fmaxr);
            let assign24270_e27317: f64 = (assign24270_e27312 - assign24270_e27316);
            let assign24270_e27320: f64 = (-230.25850929940458);
            let assign24270_e27322: f64 = (-locals.var_fbbtsti);
            let assign24270_e27324: f64 = (assign24270_e27322 / locals.var_fmaxr);
            let assign24270_e27325: f64 = (assign24270_e27320 - assign24270_e27324);
            let assign24270_e27327: f64 = (assign24270_e27325 * 0.3333333333333333);
            let assign24270_e27328: f64 = (1.0 + assign24270_e27327);
            let assign24270_e27329: f64 = (assign24270_e27317 * assign24270_e27328);
            let assign24270_e27330: f64 = (0.5 * assign24270_e27329);
            let assign24270_e27331: f64 = (1.0 + assign24270_e27330);
            let assign24270_e27332: f64 = (assign24270_e27308 * assign24270_e27331);
            let assign24270_e27333: f64 = (1.0 + assign24270_e27332);
            let assign24270_e27334: f64 = (1e-100 / assign24270_e27333);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24270_e27334, (-((1e-100 * (((-(-((assign24270_e27305 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign24270_e27331) + (assign24270_e27308 * (0.5 * (((-(-((assign24270_e27314 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign24270_e27328) + (assign24270_e27317 * ((-(-((assign24270_e27322 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign24270_e27333 * assign24270_e27333))), (-((1e-100 * (((-(-((assign24270_e27305 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign24270_e27331) + (assign24270_e27308 * (0.5 * (((-(-((assign24270_e27314 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign24270_e27328) + (assign24270_e27317 * ((-(-((assign24270_e27322 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign24270_e27333 * assign24270_e27333))), (-((1e-100 * (((-(-((assign24270_e27305 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign24270_e27331) + (assign24270_e27308 * (0.5 * (((-(-((assign24270_e27314 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign24270_e27328) + (assign24270_e27317 * ((-(-((assign24270_e27322 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign24270_e27333 * assign24270_e27333))), (-((1e-100 * (((-(-((assign24270_e27305 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign24270_e27331) + (assign24270_e27308 * (0.5 * (((-(-((assign24270_e27314 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign24270_e27328) + (assign24270_e27317 * ((-(-((assign24270_e27322 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign24270_e27333 * assign24270_e27333))), );
        }
    }
    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 == 0.0)) && (locals.var_guard451 == 0.0)) {
            let assign24280_e27355: f64 = (-locals.var_fbbtsti);
            let assign24280_e27357: f64 = (assign24280_e27355 / locals.var_fmaxr);
            let assign24280_e27359: f64 = (assign24280_e27357 - 230.25850929940458);
            let assign24280_e27363: f64 = (-locals.var_fbbtsti);
            let assign24280_e27365: f64 = (assign24280_e27363 / locals.var_fmaxr);
            let assign24280_e27367: f64 = (assign24280_e27365 - 230.25850929940458);
            let assign24280_e27370: f64 = (-locals.var_fbbtsti);
            let assign24280_e27372: f64 = (assign24280_e27370 / locals.var_fmaxr);
            let assign24280_e27374: f64 = (assign24280_e27372 - 230.25850929940458);
            let assign24280_e27376: f64 = (assign24280_e27374 * 0.3333333333333333);
            let assign24280_e27377: f64 = (1.0 + assign24280_e27376);
            let assign24280_e27378: f64 = (assign24280_e27367 * assign24280_e27377);
            let assign24280_e27379: f64 = (0.5 * assign24280_e27378);
            let assign24280_e27380: f64 = (1.0 + assign24280_e27379);
            let assign24280_e27381: f64 = (assign24280_e27359 * assign24280_e27380);
            let assign24280_e27382: f64 = (1.0 + assign24280_e27381);
            let assign24280_e27383: f64 = (1e100 * assign24280_e27382);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24280_e27383, (1e100 * (((-((assign24280_e27355 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24280_e27380) + (assign24280_e27359 * (0.5 * (((-((assign24280_e27363 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24280_e27377) + (assign24280_e27367 * ((-((assign24280_e27370 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign24280_e27355 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24280_e27380) + (assign24280_e27359 * (0.5 * (((-((assign24280_e27363 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24280_e27377) + (assign24280_e27367 * ((-((assign24280_e27370 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign24280_e27355 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24280_e27380) + (assign24280_e27359 * (0.5 * (((-((assign24280_e27363 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24280_e27377) + (assign24280_e27367 * ((-((assign24280_e27370 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign24280_e27355 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24280_e27380) + (assign24280_e27359 * (0.5 * (((-((assign24280_e27363 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24280_e27377) + (assign24280_e27367 * ((-((assign24280_e27370 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard448 == 0.0)) {
            let assign24290_e27398: f64 = (locals.var_v4 * locals.var_fmaxr);
            let assign24290_e27400: f64 = (assign24290_e27398 * locals.var_fmaxr);
            let assign24290_e27402: f64 = (assign24290_e27400 * locals.var_tmp);
            let assign24290_e27403: f64 = (p.p852 * assign24290_e27402);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign24290_e27403, (p.p852 * (((((locals.var_v4 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign24290_e27398 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign24290_e27400 * locals.var_tmp_dn5))), (p.p852 * (((((locals.var_v4 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign24290_e27398 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign24290_e27400 * locals.var_tmp_dn6))), (p.p852 * (((((locals.var_v4 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign24290_e27398 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign24290_e27400 * locals.var_tmp_dn7))), (p.p852 * (((((locals.var_v4 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign24290_e27398 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign24290_e27400 * locals.var_tmp_dn8))), );
        }
        let assign24300_e27408: f64 = if p.p861 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard452 = assign24300_e27408;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard452 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign24320_e27422: f64 = (-locals.var_alphaav);
        let assign24320_e27424: f64 = (assign24320_e27422 * p.p861);
        let assign24320_e27425: f64 = if locals.var_vav > assign24320_e27424 { 1.0 } else { 0.0 };
        locals.var_guard453 = assign24320_e27425;
        let assign24330_e27428: f64 = if p.p864 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard454 = assign24330_e27428;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard452 == 0.0)) && (locals.var_guard453 != 0.0)) && (locals.var_guard454 != 0.0)) {
            let assign24340_e27444: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign24340_e27447: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign24340_e27448: f64 = (assign24340_e27444 * assign24340_e27447);
            let assign24340_e27451: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign24340_e27452: f64 = (assign24340_e27448 * assign24340_e27451);
            let assign24340_e27455: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign24340_e27456: f64 = (assign24340_e27452 * assign24340_e27455);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24340_e27456, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard452 == 0.0)) && (locals.var_guard453 != 0.0)) && (locals.var_guard454 == 0.0)) {
            let assign24350_e27475: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign24350_e27476: f64 = (assign24350_e27475).abs();
            let assign24350_e27478: f64 = (assign24350_e27476).powf(p.p864);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24350_e27478, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard452 == 0.0)) && (locals.var_guard453 != 0.0)) {
            let assign24360_e27495: f64 = (1.0 - locals.var_tmp);
            let assign24360_e27496: f64 = (1.0 / assign24360_e27495);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign24360_e27496, (-((-locals.var_tmp_dn5) / (assign24360_e27495 * assign24360_e27495))), (-((-locals.var_tmp_dn6) / (assign24360_e27495 * assign24360_e27495))), (-((-locals.var_tmp_dn7) / (assign24360_e27495 * assign24360_e27495))), (-((-locals.var_tmp_dn8) / (assign24360_e27495 * assign24360_e27495))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) && (locals.var_guard452 == 0.0)) && (locals.var_guard453 == 0.0)) {
            let assign24370_e27515: f64 = (locals.var_alphaav * p.p861);
            let assign24370_e27516: f64 = (locals.var_vav + assign24370_e27515);
            let assign24370_e27518: f64 = (assign24370_e27516 * locals.var_slopesti);
            let assign24370_e27519: f64 = (locals.var_fstopsti + assign24370_e27518);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign24370_e27519, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard438 == 0.0)) {
            let assign24380_e27531: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign24380_e27533: f64 = (assign24380_e27531 + locals.var_itat);
            let assign24380_e27535: f64 = (assign24380_e27533 + locals.var_ibbt);
            let assign24380_e27536: f64 = (p.p29 * assign24380_e27535);
            let assign24380_e27538: f64 = (assign24380_e27536 * locals.var_fbreakdown);
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (assign24380_e27538, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign24380_e27536 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign24380_e27536 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign24380_e27536 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign24380_e27536 * locals.var_fbreakdown_dn8)), );
        }
        let assign24390_e27543: f64 = if locals.var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard455 = assign24390_e27543;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 != 0.0)) {
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) {
            let assign24410_e27560: f64 = (locals.var_idsatgat * locals.var_idmult);
            locals.var_id__blk219 = assign24410_e27560;
        }
        let assign24420_e27569: f64 = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard456 = assign24420_e27569;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
            let assign24440_e27592: f64 = (locals.var_vbigat - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign24440_e27592;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
            let assign24450_e27608: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign24450_e27609: f64 = (1.0 - assign24450_e27608);
            let assign24450_e27610: f64 = (assign24450_e27609).sqrt();
            let assign24450_e27611: f64 = (1.0 - assign24450_e27610);
            locals.var_wsrhstep = assign24450_e27611;
        }
        let assign24460_e27616: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard457 = assign24460_e27616;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) && (locals.var_guard457 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) && (locals.var_guard457 == 0.0)) {
            let assign24480_e27645: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign24480_e27647: f64 = (locals.var_wsrhstep).ln();
            let assign24480_e27648: f64 = (assign24480_e27645 * assign24480_e27647);
            let assign24480_e27651: f64 = (1.0 - locals.var_wsrhstep);
            let assign24480_e27652: f64 = (assign24480_e27648 / assign24480_e27651);
            let assign24480_e27654: f64 = (assign24480_e27652 + locals.var_wsrhstep);
            let assign24480_e27658: f64 = (2.0 * p.p833);
            let assign24480_e27659: f64 = (1.0 - assign24480_e27658);
            let assign24480_e27660: f64 = (assign24480_e27654 * assign24480_e27659);
            locals.var_dwsrh = assign24480_e27660;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
            let assign24490_e27674: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign24490_e27674;
        }
        let assign24500_e27679: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard458 = assign24500_e27679;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) && (locals.var_guard458 != 0.0)) {
            let assign24510_e27693: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv);
            let assign24510_e27694: f64 = (assign24510_e27693).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24510_e27694, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) && (locals.var_guard458 == 0.0)) {
            let assign24520_e27711: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv);
            let assign24520_e27713: f64 = (assign24520_e27711).powf(p.p833);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24520_e27713, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
            let assign24530_e27727: f64 = (locals.var_wdepnulrgat * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign24530_e27727, (locals.var_wdepnulrgat * locals.var_tmp_dn5), (locals.var_wdepnulrgat * locals.var_tmp_dn6), (locals.var_wdepnulrgat * locals.var_tmp_dn7), (locals.var_wdepnulrgat * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
            let assign24540_e27742: f64 = (locals.var_zinv - 1.0);
            let assign24540_e27744: f64 = (assign24540_e27742 * locals.var_wdep);
            let assign24540_e27745: f64 = (locals.var_ftdgat * assign24540_e27744);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign24540_e27745, (locals.var_ftdgat * (assign24540_e27742 * locals.var_wdep_dn5)), (locals.var_ftdgat * (assign24540_e27742 * locals.var_wdep_dn6)), (locals.var_ftdgat * (assign24540_e27742 * locals.var_wdep_dn7)), (locals.var_ftdgat * (assign24540_e27742 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
            let assign24550_e27760: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign24550_e27761: f64 = (p.p842 * assign24550_e27760);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign24550_e27761, (p.p842 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign24560_e27766: f64 = if p.p847 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard459 = assign24560_e27766;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24580_e27790: f64 = (locals.var_wdep * locals.var_one_minus_pgat);
            let assign24580_e27792: f64 = (assign24580_e27790 / locals.var_vbi_minus_vjsrh);
            let assign24580_e27793: f64 = (locals.var_btatpartgat * assign24580_e27792);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign24580_e27793, (locals.var_btatpartgat * ((locals.var_wdep_dn5 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn6 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn7 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn8 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24590_e27807: f64 = (0.666666666666667 * locals.var_atatgat);
            let assign24590_e27809: f64 = (assign24590_e27807 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign24590_e27809, (-((assign24590_e27807 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign24590_e27807 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign24590_e27807 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign24590_e27807 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24600_e27823: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign24600_e27823, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24610_e27837: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign24610_e27840: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign24610_e27842: f64 = (assign24610_e27840 + 1.0);
            let assign24610_e27843: f64 = (assign24610_e27837 / assign24610_e27842);
            let assign24610_e27844: f64 = (assign24610_e27843).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign24610_e27844, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign24610_e27842) - (assign24610_e27837 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign24610_e27842 * assign24610_e27842)) / (2.0 * assign24610_e27844)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign24610_e27842) - (assign24610_e27837 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign24610_e27842 * assign24610_e27842)) / (2.0 * assign24610_e27844)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign24610_e27842) - (assign24610_e27837 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign24610_e27842 * assign24610_e27842)) / (2.0 * assign24610_e27844)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign24610_e27842) - (assign24610_e27837 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign24610_e27842 * assign24610_e27842)) / (2.0 * assign24610_e27844)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24620_e27857: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign24620_e27857, (locals.var_umax_dn5 / (2.0 * assign24620_e27857)), (locals.var_umax_dn6 / (2.0 * assign24620_e27857)), (locals.var_umax_dn7 / (2.0 * assign24620_e27857)), (locals.var_umax_dn8 / (2.0 * assign24620_e27857)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24630_e27871: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign24630_e27871, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign24640_e27875: f64 = (-p.p833);
        let assign24640_e27877: f64 = (assign24640_e27875 * locals.var_one_over_one_minus_pgat);
        let assign24640_e27879: f64 = (-1.0);
        let assign24640_e27880: f64 = if assign24640_e27877 == assign24640_e27879 { 1.0 } else { 0.0 };
        locals.var_guard460 = assign24640_e27880;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard460 != 0.0)) {
            let assign24650_e27896: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign24650_e27897: f64 = (1.0 + assign24650_e27896);
            let assign24650_e27898: f64 = (1.0 / assign24650_e27897);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign24650_e27898, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign24650_e27897 * assign24650_e27897))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign24650_e27897 * assign24650_e27897))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign24650_e27897 * assign24650_e27897))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign24650_e27897 * assign24650_e27897))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard460 == 0.0)) {
            let assign24660_e27916: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign24660_e27917: f64 = (1.0 + assign24660_e27916);
            let assign24660_e27919: f64 = (-p.p833);
            let assign24660_e27921: f64 = (assign24660_e27919 * locals.var_one_over_one_minus_pgat);
            let assign24660_e27922: f64 = (assign24660_e27917).powf(assign24660_e27921);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign24660_e27922, if 0.0 == 0.0 && ((assign24660_e27921) as f64).is_finite() && ((assign24660_e27921) as f64).fract() == 0.0 { if assign24660_e27921 == 0.0 { 0.0 } else { (assign24660_e27921 * ((assign24660_e27917).powf(assign24660_e27921 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign24660_e27922 * (assign24660_e27921 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign24660_e27917))) }, if 0.0 == 0.0 && ((assign24660_e27921) as f64).is_finite() && ((assign24660_e27921) as f64).fract() == 0.0 { if assign24660_e27921 == 0.0 { 0.0 } else { (assign24660_e27921 * ((assign24660_e27917).powf(assign24660_e27921 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign24660_e27922 * (assign24660_e27921 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign24660_e27917))) }, if 0.0 == 0.0 && ((assign24660_e27921) as f64).is_finite() && ((assign24660_e27921) as f64).fract() == 0.0 { if assign24660_e27921 == 0.0 { 0.0 } else { (assign24660_e27921 * ((assign24660_e27917).powf(assign24660_e27921 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign24660_e27922 * (assign24660_e27921 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign24660_e27917))) }, if 0.0 == 0.0 && ((assign24660_e27921) as f64).is_finite() && ((assign24660_e27921) as f64).fract() == 0.0 { if assign24660_e27921 == 0.0 { 0.0 } else { (assign24660_e27921 * ((assign24660_e27917).powf(assign24660_e27921 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign24660_e27922 * (assign24660_e27921 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign24660_e27917))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24670_e27936: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign24670_e27939: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign24670_e27940: f64 = (assign24670_e27936 / assign24670_e27939);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign24670_e27940, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign24670_e27939) - (assign24670_e27936 * locals.var_wgamma_dn5)) / (assign24670_e27939 * assign24670_e27939)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign24670_e27939) - (assign24670_e27936 * locals.var_wgamma_dn6)) / (assign24670_e27939 * assign24670_e27939)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign24670_e27939) - (assign24670_e27936 * locals.var_wgamma_dn7)) / (assign24670_e27939 * assign24670_e27939)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign24670_e27939) - (assign24670_e27936 * locals.var_wgamma_dn8)) / (assign24670_e27939 * assign24670_e27939)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24680_e27955: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign24680_e27956: f64 = (0.375 * assign24680_e27955);
            let assign24680_e27957: f64 = (assign24680_e27956).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign24680_e27957, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign24680_e27957)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign24680_e27957)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign24680_e27957)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign24680_e27957)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24690_e27972: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign24690_e27973: f64 = (2.0 * assign24690_e27972);
            let assign24690_e27975: f64 = (assign24690_e27973 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign24690_e27975, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24700_e27989: f64 = (locals.var_atatgat * locals.var_twoatatoverthreebtat);
            let assign24700_e27991: f64 = (assign24700_e27989 * locals.var_sqrtumax);
            let assign24700_e27994: f64 = (locals.var_atatgat * locals.var_umax);
            let assign24700_e27995: f64 = (assign24700_e27991 - assign24700_e27994);
            let assign24700_e27999: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign24700_e28000: f64 = (0.5 * assign24700_e27999);
            let assign24700_e28001: f64 = (assign24700_e27995 + assign24700_e28000);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign24700_e28001, (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign24700_e27989 * locals.var_sqrtumax_dn5)) - (locals.var_atatgat * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign24700_e27989 * locals.var_sqrtumax_dn6)) - (locals.var_atatgat * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign24700_e27989 * locals.var_sqrtumax_dn7)) - (locals.var_atatgat * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign24700_e27989 * locals.var_sqrtumax_dn8)) - (locals.var_atatgat * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24710_e28015: f64 = (locals.var_ltat - 1.0);
            let assign24710_e28017: f64 = (assign24710_e28015 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign24710_e28017, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign24710_e28015 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign24710_e28015 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign24710_e28015 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign24710_e28015 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24720_e28031: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign24720_e28031, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign24730_e28036: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard461 = assign24730_e28036;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard461 != 0.0)) {
            let assign24740_e28052: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign24740_e28053: f64 = (1.0 + assign24740_e28052);
            let assign24740_e28054: f64 = (1.0 / assign24740_e28053);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign24740_e28054, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign24740_e28053 * assign24740_e28053))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign24740_e28053 * assign24740_e28053))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign24740_e28053 * assign24740_e28053))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign24740_e28053 * assign24740_e28053))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard461 == 0.0)) {
            let assign24750_e28073: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign24750_e28074: f64 = (1.0 - assign24750_e28073);
            let assign24750_e28075: f64 = (1.0 / assign24750_e28074);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign24750_e28075, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign24750_e28074 * assign24750_e28074))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign24750_e28074 * assign24750_e28074))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign24750_e28074 * assign24750_e28074))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign24750_e28074 * assign24750_e28074))), );
        }
        let assign24760_e28079: f64 = (-locals.var_ysq);
        let assign24760_e28081: f64 = (assign24760_e28079 + locals.var_mtat);
        let assign24760_e28083: f64 = (-230.25850929940458);
        let assign24760_e28084: f64 = if assign24760_e28081 > assign24760_e28083 { 1.0 } else { 0.0 };
        locals.var_guard462 = assign24760_e28084;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard462 != 0.0)) {
            let assign24770_e28097: f64 = (-locals.var_ysq);
            let assign24770_e28099: f64 = (assign24770_e28097 + locals.var_mtat);
            let assign24770_e28100: f64 = (assign24770_e28099).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24770_e28100, (assign24770_e28100 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign24770_e28100 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign24770_e28100 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign24770_e28100 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard462 == 0.0)) {
            let assign24780_e28118: f64 = (-230.25850929940458);
            let assign24780_e28120: f64 = (-locals.var_ysq);
            let assign24780_e28122: f64 = (assign24780_e28120 + locals.var_mtat);
            let assign24780_e28123: f64 = (assign24780_e28118 - assign24780_e28122);
            let assign24780_e28127: f64 = (-230.25850929940458);
            let assign24780_e28129: f64 = (-locals.var_ysq);
            let assign24780_e28131: f64 = (assign24780_e28129 + locals.var_mtat);
            let assign24780_e28132: f64 = (assign24780_e28127 - assign24780_e28131);
            let assign24780_e28135: f64 = (-230.25850929940458);
            let assign24780_e28137: f64 = (-locals.var_ysq);
            let assign24780_e28139: f64 = (assign24780_e28137 + locals.var_mtat);
            let assign24780_e28140: f64 = (assign24780_e28135 - assign24780_e28139);
            let assign24780_e28142: f64 = (assign24780_e28140 * 0.3333333333333333);
            let assign24780_e28143: f64 = (1.0 + assign24780_e28142);
            let assign24780_e28144: f64 = (assign24780_e28132 * assign24780_e28143);
            let assign24780_e28145: f64 = (0.5 * assign24780_e28144);
            let assign24780_e28146: f64 = (1.0 + assign24780_e28145);
            let assign24780_e28147: f64 = (assign24780_e28123 * assign24780_e28146);
            let assign24780_e28148: f64 = (1.0 + assign24780_e28147);
            let assign24780_e28149: f64 = (1e-100 / assign24780_e28148);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24780_e28149, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign24780_e28146) + (assign24780_e28123 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign24780_e28143) + (assign24780_e28132 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign24780_e28148 * assign24780_e28148))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign24780_e28146) + (assign24780_e28123 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign24780_e28143) + (assign24780_e28132 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign24780_e28148 * assign24780_e28148))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign24780_e28146) + (assign24780_e28123 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign24780_e28143) + (assign24780_e28132 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign24780_e28148 * assign24780_e28148))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign24780_e28146) + (assign24780_e28123 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign24780_e28143) + (assign24780_e28132 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign24780_e28148 * assign24780_e28148))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24790_e28163: f64 = (0.29214664 * locals.var_terfc);
            let assign24790_e28167: f64 = (locals.var_terfc * locals.var_terfc);
            let assign24790_e28168: f64 = (locals.var_berfc * assign24790_e28167);
            let assign24790_e28169: f64 = (assign24790_e28163 + assign24790_e28168);
            let assign24790_e28173: f64 = (locals.var_terfc * locals.var_terfc);
            let assign24790_e28175: f64 = (assign24790_e28173 * locals.var_terfc);
            let assign24790_e28176: f64 = (locals.var_cerfc * assign24790_e28175);
            let assign24790_e28177: f64 = (assign24790_e28169 + assign24790_e28176);
            let assign24790_e28179: f64 = (assign24790_e28177 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign24790_e28179, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign24790_e28173 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign24790_e28177 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign24790_e28173 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign24790_e28177 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign24790_e28173 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign24790_e28177 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign24790_e28173 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign24790_e28177 * locals.var_tmp_dn8)), );
        }
        let assign24800_e28184: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard463 = assign24800_e28184;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard463 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign24820_e28201: f64 = (-230.25850929940458);
        let assign24820_e28202: f64 = if locals.var_mtat > assign24820_e28201 { 1.0 } else { 0.0 };
        locals.var_guard464 = assign24820_e28202;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
            let assign24830_e28218: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24830_e28218, (assign24830_e28218 * locals.var_mtat_dn5), (assign24830_e28218 * locals.var_mtat_dn6), (assign24830_e28218 * locals.var_mtat_dn7), (assign24830_e28218 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
            let assign24840_e28239: f64 = (-230.25850929940458);
            let assign24840_e28241: f64 = (assign24840_e28239 - locals.var_mtat);
            let assign24840_e28245: f64 = (-230.25850929940458);
            let assign24840_e28247: f64 = (assign24840_e28245 - locals.var_mtat);
            let assign24840_e28250: f64 = (-230.25850929940458);
            let assign24840_e28252: f64 = (assign24840_e28250 - locals.var_mtat);
            let assign24840_e28254: f64 = (assign24840_e28252 * 0.3333333333333333);
            let assign24840_e28255: f64 = (1.0 + assign24840_e28254);
            let assign24840_e28256: f64 = (assign24840_e28247 * assign24840_e28255);
            let assign24840_e28257: f64 = (0.5 * assign24840_e28256);
            let assign24840_e28258: f64 = (1.0 + assign24840_e28257);
            let assign24840_e28259: f64 = (assign24840_e28241 * assign24840_e28258);
            let assign24840_e28260: f64 = (1.0 + assign24840_e28259);
            let assign24840_e28261: f64 = (1e-100 / assign24840_e28260);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24840_e28261, (-((1e-100 * (((-locals.var_mtat_dn5) * assign24840_e28258) + (assign24840_e28241 * (0.5 * (((-locals.var_mtat_dn5) * assign24840_e28255) + (assign24840_e28247 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign24840_e28260 * assign24840_e28260))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign24840_e28258) + (assign24840_e28241 * (0.5 * (((-locals.var_mtat_dn6) * assign24840_e28255) + (assign24840_e28247 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign24840_e28260 * assign24840_e28260))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign24840_e28258) + (assign24840_e28241 * (0.5 * (((-locals.var_mtat_dn7) * assign24840_e28255) + (assign24840_e28247 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign24840_e28260 * assign24840_e28260))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign24840_e28258) + (assign24840_e28241 * (0.5 * (((-locals.var_mtat_dn8) * assign24840_e28255) + (assign24840_e28247 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign24840_e28260 * assign24840_e28260))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard463 == 0.0)) {
            let assign24850_e28278: f64 = (2.0 * locals.var_tmp);
            let assign24850_e28280: f64 = (assign24850_e28278 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign24850_e28280, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24860_e28294: f64 = (1.772453850905516 * 0.5);
            let assign24860_e28297: f64 = (locals.var_atatgat * locals.var_erfctimesexpmtat);
            let assign24860_e28299: f64 = (assign24860_e28297 / locals.var_ktat);
            let assign24860_e28300: f64 = (assign24860_e28294 * assign24860_e28299);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign24860_e28300, (assign24860_e28294 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign24860_e28297 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign24860_e28294 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign24860_e28297 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign24860_e28294 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign24860_e28297 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign24860_e28294 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign24860_e28297 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard459 == 0.0)) {
            let assign24870_e28315: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign24870_e28317: f64 = (assign24870_e28315 * locals.var_wtat);
            let assign24870_e28318: f64 = (p.p847 * assign24870_e28317);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign24870_e28318, (p.p847 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign24870_e28315 * locals.var_wtat_dn5))), (p.p847 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign24870_e28315 * locals.var_wtat_dn6))), (p.p847 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign24870_e28315 * locals.var_wtat_dn7))), (p.p847 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign24870_e28315 * locals.var_wtat_dn8))), );
        }
        let assign24880_e28323: f64 = if p.p853 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard465 = assign24880_e28323;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard465 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign24900_e28337: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard466 = assign24900_e28337;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard465 == 0.0)) && (locals.var_guard466 != 0.0)) {
            let assign24910_e28351: f64 = (p.p830 - locals.var_vbbt);
            let assign24910_e28353: f64 = (assign24910_e28351 * locals.var_vbirgatinv);
            let assign24910_e28354: f64 = (assign24910_e28353).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24910_e28354, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard465 == 0.0)) && (locals.var_guard466 == 0.0)) {
            let assign24920_e28371: f64 = (p.p830 - locals.var_vbbt);
            let assign24920_e28373: f64 = (assign24920_e28371 * locals.var_vbirgatinv);
            let assign24920_e28375: f64 = (assign24920_e28373).powf(p.p833);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24920_e28375, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard465 == 0.0)) {
            let assign24930_e28390: f64 = (p.p830 - locals.var_vbbt);
            let assign24930_e28392: f64 = (assign24930_e28390 * locals.var_wdepnulrinvgat);
            let assign24930_e28394: f64 = (assign24930_e28392 / locals.var_tmp);
            let assign24930_e28395: f64 = (locals.var_one_over_one_minus_pgat * assign24930_e28394);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign24930_e28395, (locals.var_one_over_one_minus_pgat * (-((assign24930_e28392 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign24930_e28392 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign24930_e28392 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign24930_e28392 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign24940_e28399: f64 = (-locals.var_fbbtgat);
        let assign24940_e28401: f64 = (assign24940_e28399 / locals.var_fmaxr);
        let assign24940_e28402: f64 = (assign24940_e28401).abs();
        let assign24940_e28404: f64 = if assign24940_e28402 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard467 = assign24940_e28404;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard465 == 0.0)) && (locals.var_guard467 != 0.0)) {
            let assign24950_e28417: f64 = (-locals.var_fbbtgat);
            let assign24950_e28419: f64 = (assign24950_e28417 / locals.var_fmaxr);
            let assign24950_e28420: f64 = (assign24950_e28419).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24950_e28420, (assign24950_e28420 * ((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign24950_e28417 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign24950_e28420 * ((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign24950_e28417 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign24950_e28420 * ((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign24950_e28417 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign24950_e28420 * ((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign24950_e28417 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))), );
        }
        let assign24960_e28424: f64 = (-locals.var_fbbtgat);
        let assign24960_e28426: f64 = (assign24960_e28424 / locals.var_fmaxr);
        let assign24960_e28428: f64 = if assign24960_e28426 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard468 = assign24960_e28428;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard465 == 0.0)) && (locals.var_guard467 == 0.0)) && (locals.var_guard468 != 0.0)) {
            let assign24970_e28446: f64 = (-230.25850929940458);
            let assign24970_e28448: f64 = (-locals.var_fbbtgat);
            let assign24970_e28450: f64 = (assign24970_e28448 / locals.var_fmaxr);
            let assign24970_e28451: f64 = (assign24970_e28446 - assign24970_e28450);
            let assign24970_e28455: f64 = (-230.25850929940458);
            let assign24970_e28457: f64 = (-locals.var_fbbtgat);
            let assign24970_e28459: f64 = (assign24970_e28457 / locals.var_fmaxr);
            let assign24970_e28460: f64 = (assign24970_e28455 - assign24970_e28459);
            let assign24970_e28463: f64 = (-230.25850929940458);
            let assign24970_e28465: f64 = (-locals.var_fbbtgat);
            let assign24970_e28467: f64 = (assign24970_e28465 / locals.var_fmaxr);
            let assign24970_e28468: f64 = (assign24970_e28463 - assign24970_e28467);
            let assign24970_e28470: f64 = (assign24970_e28468 * 0.3333333333333333);
            let assign24970_e28471: f64 = (1.0 + assign24970_e28470);
            let assign24970_e28472: f64 = (assign24970_e28460 * assign24970_e28471);
            let assign24970_e28473: f64 = (0.5 * assign24970_e28472);
            let assign24970_e28474: f64 = (1.0 + assign24970_e28473);
            let assign24970_e28475: f64 = (assign24970_e28451 * assign24970_e28474);
            let assign24970_e28476: f64 = (1.0 + assign24970_e28475);
            let assign24970_e28477: f64 = (1e-100 / assign24970_e28476);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24970_e28477, (-((1e-100 * (((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign24970_e28448 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24970_e28474) + (assign24970_e28451 * (0.5 * (((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign24970_e28457 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24970_e28471) + (assign24970_e28460 * ((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign24970_e28465 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign24970_e28476 * assign24970_e28476))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign24970_e28448 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24970_e28474) + (assign24970_e28451 * (0.5 * (((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign24970_e28457 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24970_e28471) + (assign24970_e28460 * ((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign24970_e28465 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign24970_e28476 * assign24970_e28476))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign24970_e28448 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24970_e28474) + (assign24970_e28451 * (0.5 * (((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign24970_e28457 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24970_e28471) + (assign24970_e28460 * ((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign24970_e28465 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign24970_e28476 * assign24970_e28476))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign24970_e28448 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24970_e28474) + (assign24970_e28451 * (0.5 * (((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign24970_e28457 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign24970_e28471) + (assign24970_e28460 * ((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign24970_e28465 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign24970_e28476 * assign24970_e28476))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard465 == 0.0)) && (locals.var_guard467 == 0.0)) && (locals.var_guard468 == 0.0)) {
            let assign24980_e28498: f64 = (-locals.var_fbbtgat);
            let assign24980_e28500: f64 = (assign24980_e28498 / locals.var_fmaxr);
            let assign24980_e28502: f64 = (assign24980_e28500 - 230.25850929940458);
            let assign24980_e28506: f64 = (-locals.var_fbbtgat);
            let assign24980_e28508: f64 = (assign24980_e28506 / locals.var_fmaxr);
            let assign24980_e28510: f64 = (assign24980_e28508 - 230.25850929940458);
            let assign24980_e28513: f64 = (-locals.var_fbbtgat);
            let assign24980_e28515: f64 = (assign24980_e28513 / locals.var_fmaxr);
            let assign24980_e28517: f64 = (assign24980_e28515 - 230.25850929940458);
            let assign24980_e28519: f64 = (assign24980_e28517 * 0.3333333333333333);
            let assign24980_e28520: f64 = (1.0 + assign24980_e28519);
            let assign24980_e28521: f64 = (assign24980_e28510 * assign24980_e28520);
            let assign24980_e28522: f64 = (0.5 * assign24980_e28521);
            let assign24980_e28523: f64 = (1.0 + assign24980_e28522);
            let assign24980_e28524: f64 = (assign24980_e28502 * assign24980_e28523);
            let assign24980_e28525: f64 = (1.0 + assign24980_e28524);
            let assign24980_e28526: f64 = (1e100 * assign24980_e28525);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign24980_e28526, (1e100 * ((((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign24980_e28498 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign24980_e28523) + (assign24980_e28502 * (0.5 * ((((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign24980_e28506 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign24980_e28520) + (assign24980_e28510 * (((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign24980_e28513 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign24980_e28498 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign24980_e28523) + (assign24980_e28502 * (0.5 * ((((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign24980_e28506 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign24980_e28520) + (assign24980_e28510 * (((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign24980_e28513 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign24980_e28498 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign24980_e28523) + (assign24980_e28502 * (0.5 * ((((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign24980_e28506 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign24980_e28520) + (assign24980_e28510 * (((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign24980_e28513 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign24980_e28498 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign24980_e28523) + (assign24980_e28502 * (0.5 * ((((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign24980_e28506 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign24980_e28520) + (assign24980_e28510 * (((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign24980_e28513 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard465 == 0.0)) {
            let assign24990_e28541: f64 = (locals.var_v4 * locals.var_fmaxr);
            let assign24990_e28543: f64 = (assign24990_e28541 * locals.var_fmaxr);
            let assign24990_e28545: f64 = (assign24990_e28543 * locals.var_tmp);
            let assign24990_e28546: f64 = (p.p853 * assign24990_e28545);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign24990_e28546, (p.p853 * (((((locals.var_v4 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign24990_e28541 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign24990_e28543 * locals.var_tmp_dn5))), (p.p853 * (((((locals.var_v4 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign24990_e28541 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign24990_e28543 * locals.var_tmp_dn6))), (p.p853 * (((((locals.var_v4 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign24990_e28541 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign24990_e28543 * locals.var_tmp_dn7))), (p.p853 * (((((locals.var_v4 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign24990_e28541 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign24990_e28543 * locals.var_tmp_dn8))), );
        }
        let assign25000_e28551: f64 = if p.p862 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard469 = assign25000_e28551;
    }
    pub(super) fn stamp_transient_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard469 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign25020_e28565: f64 = (-locals.var_alphaav);
        let assign25020_e28567: f64 = (assign25020_e28565 * p.p862);
        let assign25020_e28568: f64 = if locals.var_vav > assign25020_e28567 { 1.0 } else { 0.0 };
        locals.var_guard470 = assign25020_e28568;
        let assign25030_e28571: f64 = if p.p865 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard471 = assign25030_e28571;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard469 == 0.0)) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 != 0.0)) {
            let assign25040_e28587: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign25040_e28590: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign25040_e28591: f64 = (assign25040_e28587 * assign25040_e28590);
            let assign25040_e28594: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign25040_e28595: f64 = (assign25040_e28591 * assign25040_e28594);
            let assign25040_e28598: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign25040_e28599: f64 = (assign25040_e28595 * assign25040_e28598);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25040_e28599, (((((((locals.var_vav * locals.var_vbrinvgat_dn5) * assign25040_e28590) + (assign25040_e28587 * (locals.var_vav * locals.var_vbrinvgat_dn5))) * assign25040_e28594) + (assign25040_e28591 * (locals.var_vav * locals.var_vbrinvgat_dn5))) * assign25040_e28598) + (assign25040_e28595 * (locals.var_vav * locals.var_vbrinvgat_dn5))), (((((((locals.var_vav * locals.var_vbrinvgat_dn6) * assign25040_e28590) + (assign25040_e28587 * (locals.var_vav * locals.var_vbrinvgat_dn6))) * assign25040_e28594) + (assign25040_e28591 * (locals.var_vav * locals.var_vbrinvgat_dn6))) * assign25040_e28598) + (assign25040_e28595 * (locals.var_vav * locals.var_vbrinvgat_dn6))), (((((((locals.var_vav * locals.var_vbrinvgat_dn7) * assign25040_e28590) + (assign25040_e28587 * (locals.var_vav * locals.var_vbrinvgat_dn7))) * assign25040_e28594) + (assign25040_e28591 * (locals.var_vav * locals.var_vbrinvgat_dn7))) * assign25040_e28598) + (assign25040_e28595 * (locals.var_vav * locals.var_vbrinvgat_dn7))), (((((((locals.var_vav * locals.var_vbrinvgat_dn8) * assign25040_e28590) + (assign25040_e28587 * (locals.var_vav * locals.var_vbrinvgat_dn8))) * assign25040_e28594) + (assign25040_e28591 * (locals.var_vav * locals.var_vbrinvgat_dn8))) * assign25040_e28598) + (assign25040_e28595 * (locals.var_vav * locals.var_vbrinvgat_dn8))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard469 == 0.0)) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) {
            let assign25050_e28618: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign25050_e28619: f64 = (assign25050_e28618).abs();
            let assign25050_e28621: f64 = (assign25050_e28619).powf(p.p865);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25050_e28621, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign25050_e28619).powf(p.p865 - 1.0) * if assign25050_e28618 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn5)) })) } } else { (assign25050_e28621 * (p.p865 * (if assign25050_e28618 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn5)) } / assign25050_e28619))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign25050_e28619).powf(p.p865 - 1.0) * if assign25050_e28618 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn6)) })) } } else { (assign25050_e28621 * (p.p865 * (if assign25050_e28618 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn6)) } / assign25050_e28619))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign25050_e28619).powf(p.p865 - 1.0) * if assign25050_e28618 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn7)) })) } } else { (assign25050_e28621 * (p.p865 * (if assign25050_e28618 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn7)) } / assign25050_e28619))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign25050_e28619).powf(p.p865 - 1.0) * if assign25050_e28618 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn8)) })) } } else { (assign25050_e28621 * (p.p865 * (if assign25050_e28618 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn8)) } / assign25050_e28619))) }, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard469 == 0.0)) && (locals.var_guard470 != 0.0)) {
            let assign25060_e28638: f64 = (1.0 - locals.var_tmp);
            let assign25060_e28639: f64 = (1.0 / assign25060_e28638);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign25060_e28639, (-((-locals.var_tmp_dn5) / (assign25060_e28638 * assign25060_e28638))), (-((-locals.var_tmp_dn6) / (assign25060_e28638 * assign25060_e28638))), (-((-locals.var_tmp_dn7) / (assign25060_e28638 * assign25060_e28638))), (-((-locals.var_tmp_dn8) / (assign25060_e28638 * assign25060_e28638))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard469 == 0.0)) && (locals.var_guard470 == 0.0)) {
            let assign25070_e28658: f64 = (locals.var_alphaav * p.p862);
            let assign25070_e28659: f64 = (locals.var_vav + assign25070_e28658);
            let assign25070_e28661: f64 = (assign25070_e28659 * locals.var_slopegat);
            let assign25070_e28662: f64 = (locals.var_fstopgat + assign25070_e28661);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign25070_e28662, (assign25070_e28659 * locals.var_slopegat_dn5), (assign25070_e28659 * locals.var_slopegat_dn6), (assign25070_e28659 * locals.var_slopegat_dn7), (assign25070_e28659 * locals.var_slopegat_dn8), );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard455 == 0.0)) {
            let assign25080_e28674: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign25080_e28676: f64 = (assign25080_e28674 + locals.var_itat);
            let assign25080_e28678: f64 = (assign25080_e28676 + locals.var_ibbt);
            let assign25080_e28679: f64 = (p.p29 * assign25080_e28678);
            let assign25080_e28681: f64 = (assign25080_e28679 * locals.var_fbreakdown);
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (assign25080_e28681, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign25080_e28679 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign25080_e28679 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign25080_e28679 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign25080_e28679 * locals.var_fbreakdown_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign25090_e28689: f64 = (locals.var_absource_i * locals.var_ijunbot);
            let assign25090_e28692: f64 = (locals.var_lssource_i * locals.var_ijunsti);
            let assign25090_e28693: f64 = (assign25090_e28689 + assign25090_e28692);
            let assign25090_e28696: f64 = (locals.var_lgsource_i * locals.var_ijungat);
            let assign25090_e28697: f64 = (assign25090_e28693 + assign25090_e28696);
            (locals.var_i4, locals.var_i4_dn5, locals.var_i4_dn6, locals.var_i4_dn7, locals.var_i4_dn8, ) = (assign25090_e28697, (((locals.var_absource_i * locals.var_ijunbot_dn5) + (locals.var_lssource_i * locals.var_ijunsti_dn5)) + (locals.var_lgsource_i * locals.var_ijungat_dn5)), (((locals.var_absource_i * locals.var_ijunbot_dn6) + (locals.var_lssource_i * locals.var_ijunsti_dn6)) + (locals.var_lgsource_i * locals.var_ijungat_dn6)), (((locals.var_absource_i * locals.var_ijunbot_dn7) + (locals.var_lssource_i * locals.var_ijunsti_dn7)) + (locals.var_lgsource_i * locals.var_ijungat_dn7)), (((locals.var_absource_i * locals.var_ijunbot_dn8) + (locals.var_lssource_i * locals.var_ijunsti_dn8)) + (locals.var_lgsource_i * locals.var_ijungat_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            locals.var_vbbt = 0.0;
            locals.var_two_psistar = 0.0;
        }
        let assign25120_e28723: f64 = if (!(((locals.var_absource_i == 0.0) && (locals.var_lssource_i == 0.0)) && (locals.var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard472 = assign25120_e28723;
        let assign25200_e28809: f64 = if locals.var_v5 < locals.var_vmax_s { 1.0 } else { 0.0 };
        locals.var_guard473 = assign25200_e28809;
        let assign25210_e28811: f64 = (-0.5);
        let assign25210_e28814: f64 = (locals.var_v5 * locals.var_phitdinv);
        let assign25210_e28815: f64 = (assign25210_e28811 * assign25210_e28814);
        let assign25210_e28816: f64 = (assign25210_e28815).abs();
        let assign25210_e28818: f64 = if assign25210_e28816 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard474 = assign25210_e28818;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) {
            let assign25220_e28829: f64 = (-0.5);
            let assign25220_e28832: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign25220_e28833: f64 = (assign25220_e28829 * assign25220_e28832);
            let assign25220_e28834: f64 = (assign25220_e28833).exp();
            locals.var_z = assign25220_e28834;
        }
        let assign25230_e28838: f64 = (-0.5);
        let assign25230_e28841: f64 = (locals.var_v5 * locals.var_phitdinv);
        let assign25230_e28842: f64 = (assign25230_e28838 * assign25230_e28841);
        let assign25230_e28844: f64 = if assign25230_e28842 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign25230_e28844;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 == 0.0)) && (locals.var_guard475 != 0.0)) {
            let assign25240_e28860: f64 = (-230.25850929940458);
            let assign25240_e28862: f64 = (-0.5);
            let assign25240_e28865: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign25240_e28866: f64 = (assign25240_e28862 * assign25240_e28865);
            let assign25240_e28867: f64 = (assign25240_e28860 - assign25240_e28866);
            let assign25240_e28871: f64 = (-230.25850929940458);
            let assign25240_e28873: f64 = (-0.5);
            let assign25240_e28876: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign25240_e28877: f64 = (assign25240_e28873 * assign25240_e28876);
            let assign25240_e28878: f64 = (assign25240_e28871 - assign25240_e28877);
            let assign25240_e28881: f64 = (-230.25850929940458);
            let assign25240_e28883: f64 = (-0.5);
            let assign25240_e28886: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign25240_e28887: f64 = (assign25240_e28883 * assign25240_e28886);
            let assign25240_e28888: f64 = (assign25240_e28881 - assign25240_e28887);
            let assign25240_e28890: f64 = (assign25240_e28888 * 0.3333333333333333);
            let assign25240_e28891: f64 = (1.0 + assign25240_e28890);
            let assign25240_e28892: f64 = (assign25240_e28878 * assign25240_e28891);
            let assign25240_e28893: f64 = (0.5 * assign25240_e28892);
            let assign25240_e28894: f64 = (1.0 + assign25240_e28893);
            let assign25240_e28895: f64 = (assign25240_e28867 * assign25240_e28894);
            let assign25240_e28896: f64 = (1.0 + assign25240_e28895);
            let assign25240_e28897: f64 = (1e-100 / assign25240_e28896);
            locals.var_z = assign25240_e28897;
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 == 0.0)) && (locals.var_guard475 == 0.0)) {
            let assign25250_e28916: f64 = (-0.5);
            let assign25250_e28919: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign25250_e28920: f64 = (assign25250_e28916 * assign25250_e28919);
            let assign25250_e28922: f64 = (assign25250_e28920 - 230.25850929940458);
            let assign25250_e28926: f64 = (-0.5);
            let assign25250_e28929: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign25250_e28930: f64 = (assign25250_e28926 * assign25250_e28929);
            let assign25250_e28932: f64 = (assign25250_e28930 - 230.25850929940458);
            let assign25250_e28935: f64 = (-0.5);
            let assign25250_e28938: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign25250_e28939: f64 = (assign25250_e28935 * assign25250_e28938);
            let assign25250_e28941: f64 = (assign25250_e28939 - 230.25850929940458);
            let assign25250_e28943: f64 = (assign25250_e28941 * 0.3333333333333333);
            let assign25250_e28944: f64 = (1.0 + assign25250_e28943);
            let assign25250_e28945: f64 = (assign25250_e28932 * assign25250_e28944);
            let assign25250_e28946: f64 = (0.5 * assign25250_e28945);
            let assign25250_e28947: f64 = (1.0 + assign25250_e28946);
            let assign25250_e28948: f64 = (assign25250_e28922 * assign25250_e28947);
            let assign25250_e28949: f64 = (1.0 + assign25250_e28948);
            let assign25250_e28950: f64 = (1e100 * assign25250_e28949);
            locals.var_z = assign25250_e28950;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard473 != 0.0)) {
            let assign25260_e28962: f64 = (1.0 / locals.var_z);
            locals.var_zinv = assign25260_e28962;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard473 != 0.0)) {
            let assign25270_e28974: f64 = (locals.var_zinv * locals.var_zinv);
            locals.var_idmult = assign25270_e28974;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard473 == 0.0)) {
            let assign25280_e28988: f64 = (locals.var_v5 - locals.var_vmax_s);
            let assign25280_e28990: f64 = (assign25280_e28988 * locals.var_phitdinv);
            let assign25280_e28991: f64 = (1.0 + assign25280_e28990);
            let assign25280_e28993: f64 = (assign25280_e28991 * locals.var_exp_vmax_over_phitd_s);
            locals.var_idmult = assign25280_e28993;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard473 == 0.0)) {
            let assign25290_e29005: f64 = (locals.var_idmult).sqrt();
            locals.var_zinv = assign25290_e29005;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard473 == 0.0)) {
            let assign25300_e29018: f64 = (1.0 / locals.var_zinv);
            locals.var_z = assign25300_e29018;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) {
            let assign25310_e29028: f64 = (locals.var_idmult - 1.0);
            locals.var_idmult = assign25310_e29028;
        }
        let assign25320_e29033: f64 = if locals.var_v5 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard476 = assign25320_e29033;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard476 != 0.0)) {
            let assign25330_e29045: f64 = (2.0 + locals.var_z);
            let assign25330_e29048: f64 = (locals.var_z + 1.0);
            let assign25330_e29051: f64 = (locals.var_z + 3.0);
            let assign25330_e29052: f64 = (assign25330_e29048 * assign25330_e29051);
            let assign25330_e29053: f64 = (assign25330_e29052).sqrt();
            let assign25330_e29054: f64 = (assign25330_e29045 + assign25330_e29053);
            let assign25330_e29055: f64 = (assign25330_e29054).ln();
            let assign25330_e29056: f64 = (locals.var_phitd * assign25330_e29055);
            let assign25330_e29057: f64 = (2.0 * assign25330_e29056);
            locals.var_two_psistar = assign25330_e29057;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) && (locals.var_guard476 == 0.0)) {
            let assign25340_e29069: f64 = (-locals.var_v5);
            let assign25340_e29074: f64 = (2.0 * locals.var_zinv);
            let assign25340_e29076: f64 = (assign25340_e29074 + 1.0);
            let assign25340_e29079: f64 = (1.0 + locals.var_zinv);
            let assign25340_e29083: f64 = (3.0 * locals.var_zinv);
            let assign25340_e29084: f64 = (1.0 + assign25340_e29083);
            let assign25340_e29085: f64 = (assign25340_e29079 * assign25340_e29084);
            let assign25340_e29086: f64 = (assign25340_e29085).sqrt();
            let assign25340_e29087: f64 = (assign25340_e29076 + assign25340_e29086);
            let assign25340_e29088: f64 = (assign25340_e29087).ln();
            let assign25340_e29089: f64 = (locals.var_phitd * assign25340_e29088);
            let assign25340_e29090: f64 = (2.0 * assign25340_e29089);
            let assign25340_e29091: f64 = (assign25340_e29069 + assign25340_e29090);
            locals.var_two_psistar = assign25340_e29091;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) {
            let assign25350_e29101: f64 = (locals.var_vbimin_s - locals.var_two_psistar);
            locals.var_vjlim = assign25350_e29101;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) {
            let assign25360_e29112: f64 = (locals.var_v5 + locals.var_vjlim);
            let assign25360_e29115: f64 = (locals.var_v5 - locals.var_vjlim);
            let assign25360_e29118: f64 = (locals.var_v5 - locals.var_vjlim);
            let assign25360_e29119: f64 = (assign25360_e29115 * assign25360_e29118);
            let assign25360_e29122: f64 = (4.0 * locals.var_phitd);
            let assign25360_e29124: f64 = (assign25360_e29122 * locals.var_phitd);
            let assign25360_e29125: f64 = (assign25360_e29119 + assign25360_e29124);
            let assign25360_e29126: f64 = (assign25360_e29125).sqrt();
            let assign25360_e29127: f64 = (assign25360_e29112 - assign25360_e29126);
            let assign25360_e29128: f64 = (0.5 * assign25360_e29127);
            locals.var_vjsrh = assign25360_e29128;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) {
            let assign25370_e29139: f64 = (locals.var_v5 + locals.var_vbbtlim_s);
            let assign25370_e29142: f64 = (locals.var_v5 - locals.var_vbbtlim_s);
            let assign25370_e29145: f64 = (locals.var_v5 - locals.var_vbbtlim_s);
            let assign25370_e29146: f64 = (assign25370_e29142 * assign25370_e29145);
            let assign25370_e29149: f64 = (4.0 * locals.var_phitr);
            let assign25370_e29151: f64 = (assign25370_e29149 * locals.var_phitr);
            let assign25370_e29152: f64 = (assign25370_e29146 + assign25370_e29151);
            let assign25370_e29153: f64 = (assign25370_e29152).sqrt();
            let assign25370_e29154: f64 = (assign25370_e29139 - assign25370_e29153);
            let assign25370_e29155: f64 = (0.5 * assign25370_e29154);
            locals.var_vbbt = assign25370_e29155;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard472 != 0.0)) {
            let assign25380_e29166: f64 = locals.var_v5;
            let assign25380_e29169: f64 = locals.var_v5;
            let assign25380_e29172: f64 = locals.var_v5;
            let assign25380_e29173: f64 = (assign25380_e29169 * assign25380_e29172);
            let assign25380_e29176: f64 = (4.0 * 1e-6);
            let assign25380_e29178: f64 = (assign25380_e29176 * 1e-6);
            let assign25380_e29179: f64 = (assign25380_e29173 + assign25380_e29178);
            let assign25380_e29180: f64 = (assign25380_e29179).sqrt();
            let assign25380_e29181: f64 = (assign25380_e29166 - assign25380_e29180);
            let assign25380_e29182: f64 = (0.5 * assign25380_e29181);
            locals.var_vav = assign25380_e29182;
        }
        let assign25390_e29187: f64 = if locals.var_absource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard477 = assign25390_e29187;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 != 0.0)) {
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) {
            let assign25410_e29204: f64 = (locals.var_idsatbot * locals.var_idmult);
            locals.var_id__blk219 = assign25410_e29204;
        }
        let assign25420_e29213: f64 = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard478 = assign25420_e29213;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard478 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard478 == 0.0)) {
            let assign25440_e29236: f64 = (locals.var_vbibot - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign25440_e29236;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard478 == 0.0)) {
            let assign25450_e29252: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign25450_e29253: f64 = (1.0 - assign25450_e29252);
            let assign25450_e29254: f64 = (assign25450_e29253).sqrt();
            let assign25450_e29255: f64 = (1.0 - assign25450_e29254);
            locals.var_wsrhstep = assign25450_e29255;
        }
        let assign25460_e29260: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard479 = assign25460_e29260;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard479 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard479 == 0.0)) {
            let assign25480_e29289: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign25480_e29291: f64 = (locals.var_wsrhstep).ln();
            let assign25480_e29292: f64 = (assign25480_e29289 * assign25480_e29291);
            let assign25480_e29295: f64 = (1.0 - locals.var_wsrhstep);
            let assign25480_e29296: f64 = (assign25480_e29292 / assign25480_e29295);
            let assign25480_e29298: f64 = (assign25480_e29296 + locals.var_wsrhstep);
            let assign25480_e29302: f64 = (2.0 * p.p831);
            let assign25480_e29303: f64 = (1.0 - assign25480_e29302);
            let assign25480_e29304: f64 = (assign25480_e29298 * assign25480_e29303);
            locals.var_dwsrh = assign25480_e29304;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard478 == 0.0)) {
            let assign25490_e29318: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign25490_e29318;
        }
        let assign25500_e29323: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard480 = assign25500_e29323;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard480 != 0.0)) {
            let assign25510_e29337: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv);
            let assign25510_e29338: f64 = (assign25510_e29337).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25510_e29338, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard478 == 0.0)) && (locals.var_guard480 == 0.0)) {
            let assign25520_e29355: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirbotinv);
            let assign25520_e29357: f64 = (assign25520_e29355).powf(p.p831);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25520_e29357, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard478 == 0.0)) {
            let assign25530_e29371: f64 = (locals.var_wdepnulrbot * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign25530_e29371, (locals.var_wdepnulrbot * locals.var_tmp_dn5), (locals.var_wdepnulrbot * locals.var_tmp_dn6), (locals.var_wdepnulrbot * locals.var_tmp_dn7), (locals.var_wdepnulrbot * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard478 == 0.0)) {
            let assign25540_e29386: f64 = (locals.var_zinv - 1.0);
            let assign25540_e29388: f64 = (assign25540_e29386 * locals.var_wdep);
            let assign25540_e29389: f64 = (locals.var_ftdbot * assign25540_e29388);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign25540_e29389, (locals.var_ftdbot * (assign25540_e29386 * locals.var_wdep_dn5)), (locals.var_ftdbot * (assign25540_e29386 * locals.var_wdep_dn6)), (locals.var_ftdbot * (assign25540_e29386 * locals.var_wdep_dn7)), (locals.var_ftdbot * (assign25540_e29386 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard478 == 0.0)) {
            let assign25550_e29404: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign25550_e29405: f64 = (p.p840 * assign25550_e29404);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign25550_e29405, (p.p840 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p840 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign25560_e29410: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard481 = assign25560_e29410;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25580_e29434: f64 = (locals.var_wdep * locals.var_one_minus_pbot);
            let assign25580_e29436: f64 = (assign25580_e29434 / locals.var_vbi_minus_vjsrh);
            let assign25580_e29437: f64 = (locals.var_btatpartbot * assign25580_e29436);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign25580_e29437, (locals.var_btatpartbot * ((locals.var_wdep_dn5 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn6 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn7 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartbot * ((locals.var_wdep_dn8 * locals.var_one_minus_pbot) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25590_e29451: f64 = (0.666666666666667 * locals.var_atatbot);
            let assign25590_e29453: f64 = (assign25590_e29451 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign25590_e29453, (-((assign25590_e29451 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign25590_e29451 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign25590_e29451 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign25590_e29451 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25600_e29467: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign25600_e29467, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25610_e29481: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign25610_e29484: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign25610_e29486: f64 = (assign25610_e29484 + 1.0);
            let assign25610_e29487: f64 = (assign25610_e29481 / assign25610_e29486);
            let assign25610_e29488: f64 = (assign25610_e29487).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign25610_e29488, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign25610_e29486) - (assign25610_e29481 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign25610_e29486 * assign25610_e29486)) / (2.0 * assign25610_e29488)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign25610_e29486) - (assign25610_e29481 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign25610_e29486 * assign25610_e29486)) / (2.0 * assign25610_e29488)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign25610_e29486) - (assign25610_e29481 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign25610_e29486 * assign25610_e29486)) / (2.0 * assign25610_e29488)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign25610_e29486) - (assign25610_e29481 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign25610_e29486 * assign25610_e29486)) / (2.0 * assign25610_e29488)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25620_e29501: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign25620_e29501, (locals.var_umax_dn5 / (2.0 * assign25620_e29501)), (locals.var_umax_dn6 / (2.0 * assign25620_e29501)), (locals.var_umax_dn7 / (2.0 * assign25620_e29501)), (locals.var_umax_dn8 / (2.0 * assign25620_e29501)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25630_e29515: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign25630_e29515, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign25640_e29519: f64 = (-p.p831);
        let assign25640_e29521: f64 = (assign25640_e29519 * locals.var_one_over_one_minus_pbot);
        let assign25640_e29523: f64 = (-1.0);
        let assign25640_e29524: f64 = if assign25640_e29521 == assign25640_e29523 { 1.0 } else { 0.0 };
        locals.var_guard482 = assign25640_e29524;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) && (locals.var_guard482 != 0.0)) {
            let assign25650_e29540: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign25650_e29541: f64 = (1.0 + assign25650_e29540);
            let assign25650_e29542: f64 = (1.0 / assign25650_e29541);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign25650_e29542, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign25650_e29541 * assign25650_e29541))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign25650_e29541 * assign25650_e29541))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign25650_e29541 * assign25650_e29541))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign25650_e29541 * assign25650_e29541))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) && (locals.var_guard482 == 0.0)) {
            let assign25660_e29560: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign25660_e29561: f64 = (1.0 + assign25660_e29560);
            let assign25660_e29563: f64 = (-p.p831);
            let assign25660_e29565: f64 = (assign25660_e29563 * locals.var_one_over_one_minus_pbot);
            let assign25660_e29566: f64 = (assign25660_e29561).powf(assign25660_e29565);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign25660_e29566, if 0.0 == 0.0 && ((assign25660_e29565) as f64).is_finite() && ((assign25660_e29565) as f64).fract() == 0.0 { if assign25660_e29565 == 0.0 { 0.0 } else { (assign25660_e29565 * ((assign25660_e29561).powf(assign25660_e29565 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign25660_e29566 * (assign25660_e29565 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign25660_e29561))) }, if 0.0 == 0.0 && ((assign25660_e29565) as f64).is_finite() && ((assign25660_e29565) as f64).fract() == 0.0 { if assign25660_e29565 == 0.0 { 0.0 } else { (assign25660_e29565 * ((assign25660_e29561).powf(assign25660_e29565 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign25660_e29566 * (assign25660_e29565 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign25660_e29561))) }, if 0.0 == 0.0 && ((assign25660_e29565) as f64).is_finite() && ((assign25660_e29565) as f64).fract() == 0.0 { if assign25660_e29565 == 0.0 { 0.0 } else { (assign25660_e29565 * ((assign25660_e29561).powf(assign25660_e29565 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign25660_e29566 * (assign25660_e29565 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign25660_e29561))) }, if 0.0 == 0.0 && ((assign25660_e29565) as f64).is_finite() && ((assign25660_e29565) as f64).fract() == 0.0 { if assign25660_e29565 == 0.0 { 0.0 } else { (assign25660_e29565 * ((assign25660_e29561).powf(assign25660_e29565 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign25660_e29566 * (assign25660_e29565 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign25660_e29561))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25670_e29580: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign25670_e29583: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign25670_e29584: f64 = (assign25670_e29580 / assign25670_e29583);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign25670_e29584, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign25670_e29583) - (assign25670_e29580 * locals.var_wgamma_dn5)) / (assign25670_e29583 * assign25670_e29583)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign25670_e29583) - (assign25670_e29580 * locals.var_wgamma_dn6)) / (assign25670_e29583 * assign25670_e29583)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign25670_e29583) - (assign25670_e29580 * locals.var_wgamma_dn7)) / (assign25670_e29583 * assign25670_e29583)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign25670_e29583) - (assign25670_e29580 * locals.var_wgamma_dn8)) / (assign25670_e29583 * assign25670_e29583)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25680_e29599: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign25680_e29600: f64 = (0.375 * assign25680_e29599);
            let assign25680_e29601: f64 = (assign25680_e29600).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign25680_e29601, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign25680_e29601)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign25680_e29601)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign25680_e29601)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign25680_e29601)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25690_e29616: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign25690_e29617: f64 = (2.0 * assign25690_e29616);
            let assign25690_e29619: f64 = (assign25690_e29617 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign25690_e29619, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25700_e29633: f64 = (locals.var_atatbot * locals.var_twoatatoverthreebtat);
            let assign25700_e29635: f64 = (assign25700_e29633 * locals.var_sqrtumax);
            let assign25700_e29638: f64 = (locals.var_atatbot * locals.var_umax);
            let assign25700_e29639: f64 = (assign25700_e29635 - assign25700_e29638);
            let assign25700_e29643: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign25700_e29644: f64 = (0.5 * assign25700_e29643);
            let assign25700_e29645: f64 = (assign25700_e29639 + assign25700_e29644);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign25700_e29645, (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign25700_e29633 * locals.var_sqrtumax_dn5)) - (locals.var_atatbot * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign25700_e29633 * locals.var_sqrtumax_dn6)) - (locals.var_atatbot * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign25700_e29633 * locals.var_sqrtumax_dn7)) - (locals.var_atatbot * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatbot * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign25700_e29633 * locals.var_sqrtumax_dn8)) - (locals.var_atatbot * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25710_e29659: f64 = (locals.var_ltat - 1.0);
            let assign25710_e29661: f64 = (assign25710_e29659 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign25710_e29661, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign25710_e29659 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign25710_e29659 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign25710_e29659 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign25710_e29659 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25720_e29675: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign25720_e29675, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign25730_e29680: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard483 = assign25730_e29680;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) && (locals.var_guard483 != 0.0)) {
            let assign25740_e29696: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign25740_e29697: f64 = (1.0 + assign25740_e29696);
            let assign25740_e29698: f64 = (1.0 / assign25740_e29697);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign25740_e29698, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign25740_e29697 * assign25740_e29697))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign25740_e29697 * assign25740_e29697))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign25740_e29697 * assign25740_e29697))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign25740_e29697 * assign25740_e29697))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) && (locals.var_guard483 == 0.0)) {
            let assign25750_e29717: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign25750_e29718: f64 = (1.0 - assign25750_e29717);
            let assign25750_e29719: f64 = (1.0 / assign25750_e29718);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign25750_e29719, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign25750_e29718 * assign25750_e29718))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign25750_e29718 * assign25750_e29718))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign25750_e29718 * assign25750_e29718))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign25750_e29718 * assign25750_e29718))), );
        }
        let assign25760_e29723: f64 = (-locals.var_ysq);
        let assign25760_e29725: f64 = (assign25760_e29723 + locals.var_mtat);
        let assign25760_e29727: f64 = (-230.25850929940458);
        let assign25760_e29728: f64 = if assign25760_e29725 > assign25760_e29727 { 1.0 } else { 0.0 };
        locals.var_guard484 = assign25760_e29728;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) && (locals.var_guard484 != 0.0)) {
            let assign25770_e29741: f64 = (-locals.var_ysq);
            let assign25770_e29743: f64 = (assign25770_e29741 + locals.var_mtat);
            let assign25770_e29744: f64 = (assign25770_e29743).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25770_e29744, (assign25770_e29744 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign25770_e29744 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign25770_e29744 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign25770_e29744 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
    }
    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) && (locals.var_guard484 == 0.0)) {
            let assign25780_e29762: f64 = (-230.25850929940458);
            let assign25780_e29764: f64 = (-locals.var_ysq);
            let assign25780_e29766: f64 = (assign25780_e29764 + locals.var_mtat);
            let assign25780_e29767: f64 = (assign25780_e29762 - assign25780_e29766);
            let assign25780_e29771: f64 = (-230.25850929940458);
            let assign25780_e29773: f64 = (-locals.var_ysq);
            let assign25780_e29775: f64 = (assign25780_e29773 + locals.var_mtat);
            let assign25780_e29776: f64 = (assign25780_e29771 - assign25780_e29775);
            let assign25780_e29779: f64 = (-230.25850929940458);
            let assign25780_e29781: f64 = (-locals.var_ysq);
            let assign25780_e29783: f64 = (assign25780_e29781 + locals.var_mtat);
            let assign25780_e29784: f64 = (assign25780_e29779 - assign25780_e29783);
            let assign25780_e29786: f64 = (assign25780_e29784 * 0.3333333333333333);
            let assign25780_e29787: f64 = (1.0 + assign25780_e29786);
            let assign25780_e29788: f64 = (assign25780_e29776 * assign25780_e29787);
            let assign25780_e29789: f64 = (0.5 * assign25780_e29788);
            let assign25780_e29790: f64 = (1.0 + assign25780_e29789);
            let assign25780_e29791: f64 = (assign25780_e29767 * assign25780_e29790);
            let assign25780_e29792: f64 = (1.0 + assign25780_e29791);
            let assign25780_e29793: f64 = (1e-100 / assign25780_e29792);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25780_e29793, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign25780_e29790) + (assign25780_e29767 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign25780_e29787) + (assign25780_e29776 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign25780_e29792 * assign25780_e29792))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign25780_e29790) + (assign25780_e29767 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign25780_e29787) + (assign25780_e29776 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign25780_e29792 * assign25780_e29792))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign25780_e29790) + (assign25780_e29767 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign25780_e29787) + (assign25780_e29776 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign25780_e29792 * assign25780_e29792))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign25780_e29790) + (assign25780_e29767 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign25780_e29787) + (assign25780_e29776 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign25780_e29792 * assign25780_e29792))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25790_e29807: f64 = (0.29214664 * locals.var_terfc);
            let assign25790_e29811: f64 = (locals.var_terfc * locals.var_terfc);
            let assign25790_e29812: f64 = (locals.var_berfc * assign25790_e29811);
            let assign25790_e29813: f64 = (assign25790_e29807 + assign25790_e29812);
            let assign25790_e29817: f64 = (locals.var_terfc * locals.var_terfc);
            let assign25790_e29819: f64 = (assign25790_e29817 * locals.var_terfc);
            let assign25790_e29820: f64 = (locals.var_cerfc * assign25790_e29819);
            let assign25790_e29821: f64 = (assign25790_e29813 + assign25790_e29820);
            let assign25790_e29823: f64 = (assign25790_e29821 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign25790_e29823, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign25790_e29817 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign25790_e29821 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign25790_e29817 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign25790_e29821 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign25790_e29817 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign25790_e29821 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign25790_e29817 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign25790_e29821 * locals.var_tmp_dn8)), );
        }
        let assign25800_e29828: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard485 = assign25800_e29828;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) && (locals.var_guard485 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign25820_e29845: f64 = (-230.25850929940458);
        let assign25820_e29846: f64 = if locals.var_mtat > assign25820_e29845 { 1.0 } else { 0.0 };
        locals.var_guard486 = assign25820_e29846;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) && (locals.var_guard485 == 0.0)) && (locals.var_guard486 != 0.0)) {
            let assign25830_e29862: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25830_e29862, (assign25830_e29862 * locals.var_mtat_dn5), (assign25830_e29862 * locals.var_mtat_dn6), (assign25830_e29862 * locals.var_mtat_dn7), (assign25830_e29862 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) && (locals.var_guard485 == 0.0)) && (locals.var_guard486 == 0.0)) {
            let assign25840_e29883: f64 = (-230.25850929940458);
            let assign25840_e29885: f64 = (assign25840_e29883 - locals.var_mtat);
            let assign25840_e29889: f64 = (-230.25850929940458);
            let assign25840_e29891: f64 = (assign25840_e29889 - locals.var_mtat);
            let assign25840_e29894: f64 = (-230.25850929940458);
            let assign25840_e29896: f64 = (assign25840_e29894 - locals.var_mtat);
            let assign25840_e29898: f64 = (assign25840_e29896 * 0.3333333333333333);
            let assign25840_e29899: f64 = (1.0 + assign25840_e29898);
            let assign25840_e29900: f64 = (assign25840_e29891 * assign25840_e29899);
            let assign25840_e29901: f64 = (0.5 * assign25840_e29900);
            let assign25840_e29902: f64 = (1.0 + assign25840_e29901);
            let assign25840_e29903: f64 = (assign25840_e29885 * assign25840_e29902);
            let assign25840_e29904: f64 = (1.0 + assign25840_e29903);
            let assign25840_e29905: f64 = (1e-100 / assign25840_e29904);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25840_e29905, (-((1e-100 * (((-locals.var_mtat_dn5) * assign25840_e29902) + (assign25840_e29885 * (0.5 * (((-locals.var_mtat_dn5) * assign25840_e29899) + (assign25840_e29891 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign25840_e29904 * assign25840_e29904))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign25840_e29902) + (assign25840_e29885 * (0.5 * (((-locals.var_mtat_dn6) * assign25840_e29899) + (assign25840_e29891 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign25840_e29904 * assign25840_e29904))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign25840_e29902) + (assign25840_e29885 * (0.5 * (((-locals.var_mtat_dn7) * assign25840_e29899) + (assign25840_e29891 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign25840_e29904 * assign25840_e29904))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign25840_e29902) + (assign25840_e29885 * (0.5 * (((-locals.var_mtat_dn8) * assign25840_e29899) + (assign25840_e29891 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign25840_e29904 * assign25840_e29904))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) && (locals.var_guard485 == 0.0)) {
            let assign25850_e29922: f64 = (2.0 * locals.var_tmp);
            let assign25850_e29924: f64 = (assign25850_e29922 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign25850_e29924, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25860_e29938: f64 = (1.772453850905516 * 0.5);
            let assign25860_e29941: f64 = (locals.var_atatbot * locals.var_erfctimesexpmtat);
            let assign25860_e29943: f64 = (assign25860_e29941 / locals.var_ktat);
            let assign25860_e29944: f64 = (assign25860_e29938 * assign25860_e29943);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign25860_e29944, (assign25860_e29938 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign25860_e29941 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign25860_e29938 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign25860_e29941 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign25860_e29938 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign25860_e29941 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign25860_e29938 * ((((locals.var_atatbot * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign25860_e29941 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard481 == 0.0)) {
            let assign25870_e29959: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign25870_e29961: f64 = (assign25870_e29959 * locals.var_wtat);
            let assign25870_e29962: f64 = (p.p845 * assign25870_e29961);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign25870_e29962, (p.p845 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign25870_e29959 * locals.var_wtat_dn5))), (p.p845 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign25870_e29959 * locals.var_wtat_dn6))), (p.p845 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign25870_e29959 * locals.var_wtat_dn7))), (p.p845 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign25870_e29959 * locals.var_wtat_dn8))), );
        }
        let assign25880_e29967: f64 = if p.p851 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard487 = assign25880_e29967;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard487 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign25900_e29981: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard488 = assign25900_e29981;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard487 == 0.0)) && (locals.var_guard488 != 0.0)) {
            let assign25910_e29995: f64 = (p.p828 - locals.var_vbbt);
            let assign25910_e29997: f64 = (assign25910_e29995 * locals.var_vbirbotinv);
            let assign25910_e29998: f64 = (assign25910_e29997).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25910_e29998, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard487 == 0.0)) && (locals.var_guard488 == 0.0)) {
            let assign25920_e30015: f64 = (p.p828 - locals.var_vbbt);
            let assign25920_e30017: f64 = (assign25920_e30015 * locals.var_vbirbotinv);
            let assign25920_e30019: f64 = (assign25920_e30017).powf(p.p831);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25920_e30019, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard487 == 0.0)) {
            let assign25930_e30034: f64 = (p.p828 - locals.var_vbbt);
            let assign25930_e30036: f64 = (assign25930_e30034 * locals.var_wdepnulrinvbot);
            let assign25930_e30038: f64 = (assign25930_e30036 / locals.var_tmp);
            let assign25930_e30039: f64 = (locals.var_one_over_one_minus_pbot * assign25930_e30038);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign25930_e30039, (locals.var_one_over_one_minus_pbot * (-((assign25930_e30036 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign25930_e30036 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign25930_e30036 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pbot * (-((assign25930_e30036 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign25940_e30043: f64 = (-locals.var_fbbtbot);
        let assign25940_e30045: f64 = (assign25940_e30043 / locals.var_fmaxr);
        let assign25940_e30046: f64 = (assign25940_e30045).abs();
        let assign25940_e30048: f64 = if assign25940_e30046 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard489 = assign25940_e30048;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard487 == 0.0)) && (locals.var_guard489 != 0.0)) {
            let assign25950_e30061: f64 = (-locals.var_fbbtbot);
            let assign25950_e30063: f64 = (assign25950_e30061 / locals.var_fmaxr);
            let assign25950_e30064: f64 = (assign25950_e30063).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25950_e30064, (assign25950_e30064 * (-((assign25950_e30061 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign25950_e30064 * (-((assign25950_e30061 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign25950_e30064 * (-((assign25950_e30061 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign25950_e30064 * (-((assign25950_e30061 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign25960_e30068: f64 = (-locals.var_fbbtbot);
        let assign25960_e30070: f64 = (assign25960_e30068 / locals.var_fmaxr);
        let assign25960_e30072: f64 = if assign25960_e30070 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard490 = assign25960_e30072;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard487 == 0.0)) && (locals.var_guard489 == 0.0)) && (locals.var_guard490 != 0.0)) {
            let assign25970_e30090: f64 = (-230.25850929940458);
            let assign25970_e30092: f64 = (-locals.var_fbbtbot);
            let assign25970_e30094: f64 = (assign25970_e30092 / locals.var_fmaxr);
            let assign25970_e30095: f64 = (assign25970_e30090 - assign25970_e30094);
            let assign25970_e30099: f64 = (-230.25850929940458);
            let assign25970_e30101: f64 = (-locals.var_fbbtbot);
            let assign25970_e30103: f64 = (assign25970_e30101 / locals.var_fmaxr);
            let assign25970_e30104: f64 = (assign25970_e30099 - assign25970_e30103);
            let assign25970_e30107: f64 = (-230.25850929940458);
            let assign25970_e30109: f64 = (-locals.var_fbbtbot);
            let assign25970_e30111: f64 = (assign25970_e30109 / locals.var_fmaxr);
            let assign25970_e30112: f64 = (assign25970_e30107 - assign25970_e30111);
            let assign25970_e30114: f64 = (assign25970_e30112 * 0.3333333333333333);
            let assign25970_e30115: f64 = (1.0 + assign25970_e30114);
            let assign25970_e30116: f64 = (assign25970_e30104 * assign25970_e30115);
            let assign25970_e30117: f64 = (0.5 * assign25970_e30116);
            let assign25970_e30118: f64 = (1.0 + assign25970_e30117);
            let assign25970_e30119: f64 = (assign25970_e30095 * assign25970_e30118);
            let assign25970_e30120: f64 = (1.0 + assign25970_e30119);
            let assign25970_e30121: f64 = (1e-100 / assign25970_e30120);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25970_e30121, (-((1e-100 * (((-(-((assign25970_e30092 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign25970_e30118) + (assign25970_e30095 * (0.5 * (((-(-((assign25970_e30101 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign25970_e30115) + (assign25970_e30104 * ((-(-((assign25970_e30109 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign25970_e30120 * assign25970_e30120))), (-((1e-100 * (((-(-((assign25970_e30092 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign25970_e30118) + (assign25970_e30095 * (0.5 * (((-(-((assign25970_e30101 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign25970_e30115) + (assign25970_e30104 * ((-(-((assign25970_e30109 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign25970_e30120 * assign25970_e30120))), (-((1e-100 * (((-(-((assign25970_e30092 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign25970_e30118) + (assign25970_e30095 * (0.5 * (((-(-((assign25970_e30101 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign25970_e30115) + (assign25970_e30104 * ((-(-((assign25970_e30109 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign25970_e30120 * assign25970_e30120))), (-((1e-100 * (((-(-((assign25970_e30092 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign25970_e30118) + (assign25970_e30095 * (0.5 * (((-(-((assign25970_e30101 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign25970_e30115) + (assign25970_e30104 * ((-(-((assign25970_e30109 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign25970_e30120 * assign25970_e30120))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard487 == 0.0)) && (locals.var_guard489 == 0.0)) && (locals.var_guard490 == 0.0)) {
            let assign25980_e30142: f64 = (-locals.var_fbbtbot);
            let assign25980_e30144: f64 = (assign25980_e30142 / locals.var_fmaxr);
            let assign25980_e30146: f64 = (assign25980_e30144 - 230.25850929940458);
            let assign25980_e30150: f64 = (-locals.var_fbbtbot);
            let assign25980_e30152: f64 = (assign25980_e30150 / locals.var_fmaxr);
            let assign25980_e30154: f64 = (assign25980_e30152 - 230.25850929940458);
            let assign25980_e30157: f64 = (-locals.var_fbbtbot);
            let assign25980_e30159: f64 = (assign25980_e30157 / locals.var_fmaxr);
            let assign25980_e30161: f64 = (assign25980_e30159 - 230.25850929940458);
            let assign25980_e30163: f64 = (assign25980_e30161 * 0.3333333333333333);
            let assign25980_e30164: f64 = (1.0 + assign25980_e30163);
            let assign25980_e30165: f64 = (assign25980_e30154 * assign25980_e30164);
            let assign25980_e30166: f64 = (0.5 * assign25980_e30165);
            let assign25980_e30167: f64 = (1.0 + assign25980_e30166);
            let assign25980_e30168: f64 = (assign25980_e30146 * assign25980_e30167);
            let assign25980_e30169: f64 = (1.0 + assign25980_e30168);
            let assign25980_e30170: f64 = (1e100 * assign25980_e30169);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign25980_e30170, (1e100 * (((-((assign25980_e30142 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign25980_e30167) + (assign25980_e30146 * (0.5 * (((-((assign25980_e30150 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign25980_e30164) + (assign25980_e30154 * ((-((assign25980_e30157 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign25980_e30142 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign25980_e30167) + (assign25980_e30146 * (0.5 * (((-((assign25980_e30150 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign25980_e30164) + (assign25980_e30154 * ((-((assign25980_e30157 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign25980_e30142 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign25980_e30167) + (assign25980_e30146 * (0.5 * (((-((assign25980_e30150 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign25980_e30164) + (assign25980_e30154 * ((-((assign25980_e30157 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign25980_e30142 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign25980_e30167) + (assign25980_e30146 * (0.5 * (((-((assign25980_e30150 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign25980_e30164) + (assign25980_e30154 * ((-((assign25980_e30157 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard487 == 0.0)) {
            let assign25990_e30185: f64 = (locals.var_v5 * locals.var_fmaxr);
            let assign25990_e30187: f64 = (assign25990_e30185 * locals.var_fmaxr);
            let assign25990_e30189: f64 = (assign25990_e30187 * locals.var_tmp);
            let assign25990_e30190: f64 = (p.p851 * assign25990_e30189);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign25990_e30190, (p.p851 * (((((locals.var_v5 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign25990_e30185 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign25990_e30187 * locals.var_tmp_dn5))), (p.p851 * (((((locals.var_v5 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign25990_e30185 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign25990_e30187 * locals.var_tmp_dn6))), (p.p851 * (((((locals.var_v5 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign25990_e30185 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign25990_e30187 * locals.var_tmp_dn7))), (p.p851 * (((((locals.var_v5 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign25990_e30185 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign25990_e30187 * locals.var_tmp_dn8))), );
        }
        let assign26000_e30195: f64 = if p.p860 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard491 = assign26000_e30195;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard491 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign26020_e30209: f64 = (-locals.var_alphaav);
        let assign26020_e30211: f64 = (assign26020_e30209 * p.p860);
        let assign26020_e30212: f64 = if locals.var_vav > assign26020_e30211 { 1.0 } else { 0.0 };
        locals.var_guard492 = assign26020_e30212;
        let assign26030_e30215: f64 = if p.p863 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard493 = assign26030_e30215;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard491 == 0.0)) && (locals.var_guard492 != 0.0)) && (locals.var_guard493 != 0.0)) {
            let assign26040_e30231: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign26040_e30234: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign26040_e30235: f64 = (assign26040_e30231 * assign26040_e30234);
            let assign26040_e30238: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign26040_e30239: f64 = (assign26040_e30235 * assign26040_e30238);
            let assign26040_e30242: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign26040_e30243: f64 = (assign26040_e30239 * assign26040_e30242);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26040_e30243, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard491 == 0.0)) && (locals.var_guard492 != 0.0)) && (locals.var_guard493 == 0.0)) {
            let assign26050_e30262: f64 = (locals.var_vav * locals.var_vbrinvbot);
            let assign26050_e30263: f64 = (assign26050_e30262).abs();
            let assign26050_e30265: f64 = (assign26050_e30263).powf(p.p863);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26050_e30265, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard491 == 0.0)) && (locals.var_guard492 != 0.0)) {
            let assign26060_e30282: f64 = (1.0 - locals.var_tmp);
            let assign26060_e30283: f64 = (1.0 / assign26060_e30282);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign26060_e30283, (-((-locals.var_tmp_dn5) / (assign26060_e30282 * assign26060_e30282))), (-((-locals.var_tmp_dn6) / (assign26060_e30282 * assign26060_e30282))), (-((-locals.var_tmp_dn7) / (assign26060_e30282 * assign26060_e30282))), (-((-locals.var_tmp_dn8) / (assign26060_e30282 * assign26060_e30282))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard491 == 0.0)) && (locals.var_guard492 == 0.0)) {
            let assign26070_e30302: f64 = (locals.var_alphaav * p.p860);
            let assign26070_e30303: f64 = (locals.var_vav + assign26070_e30302);
            let assign26070_e30305: f64 = (assign26070_e30303 * locals.var_slopebot);
            let assign26070_e30306: f64 = (locals.var_fstopbot + assign26070_e30305);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign26070_e30306, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard477 == 0.0)) {
            let assign26080_e30318: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign26080_e30320: f64 = (assign26080_e30318 + locals.var_itat);
            let assign26080_e30322: f64 = (assign26080_e30320 + locals.var_ibbt);
            let assign26080_e30323: f64 = (p.p29 * assign26080_e30322);
            let assign26080_e30325: f64 = (assign26080_e30323 * locals.var_fbreakdown);
            (locals.var_ijunbot, locals.var_ijunbot_dn5, locals.var_ijunbot_dn6, locals.var_ijunbot_dn7, locals.var_ijunbot_dn8, ) = (assign26080_e30325, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign26080_e30323 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign26080_e30323 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign26080_e30323 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign26080_e30323 * locals.var_fbreakdown_dn8)), );
        }
        let assign26090_e30330: f64 = if locals.var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard494 = assign26090_e30330;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 != 0.0)) {
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) {
            let assign26110_e30347: f64 = (locals.var_idsatsti * locals.var_idmult);
            locals.var_id__blk219 = assign26110_e30347;
        }
        let assign26120_e30356: f64 = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard495 = assign26120_e30356;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard495 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard495 == 0.0)) {
            let assign26140_e30379: f64 = (locals.var_vbisti - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign26140_e30379;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard495 == 0.0)) {
            let assign26150_e30395: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign26150_e30396: f64 = (1.0 - assign26150_e30395);
            let assign26150_e30397: f64 = (assign26150_e30396).sqrt();
            let assign26150_e30398: f64 = (1.0 - assign26150_e30397);
            locals.var_wsrhstep = assign26150_e30398;
        }
        let assign26160_e30403: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard496 = assign26160_e30403;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard495 == 0.0)) && (locals.var_guard496 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard495 == 0.0)) && (locals.var_guard496 == 0.0)) {
            let assign26180_e30432: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign26180_e30434: f64 = (locals.var_wsrhstep).ln();
            let assign26180_e30435: f64 = (assign26180_e30432 * assign26180_e30434);
            let assign26180_e30438: f64 = (1.0 - locals.var_wsrhstep);
            let assign26180_e30439: f64 = (assign26180_e30435 / assign26180_e30438);
            let assign26180_e30441: f64 = (assign26180_e30439 + locals.var_wsrhstep);
            let assign26180_e30445: f64 = (2.0 * p.p832);
            let assign26180_e30446: f64 = (1.0 - assign26180_e30445);
            let assign26180_e30447: f64 = (assign26180_e30441 * assign26180_e30446);
            locals.var_dwsrh = assign26180_e30447;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard495 == 0.0)) {
            let assign26190_e30461: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign26190_e30461;
        }
        let assign26200_e30466: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard497 = assign26200_e30466;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard495 == 0.0)) && (locals.var_guard497 != 0.0)) {
            let assign26210_e30480: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv);
            let assign26210_e30481: f64 = (assign26210_e30480).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26210_e30481, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard495 == 0.0)) && (locals.var_guard497 == 0.0)) {
            let assign26220_e30498: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirstiinv);
            let assign26220_e30500: f64 = (assign26220_e30498).powf(p.p832);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26220_e30500, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard495 == 0.0)) {
            let assign26230_e30514: f64 = (locals.var_wdepnulrsti * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign26230_e30514, (locals.var_wdepnulrsti * locals.var_tmp_dn5), (locals.var_wdepnulrsti * locals.var_tmp_dn6), (locals.var_wdepnulrsti * locals.var_tmp_dn7), (locals.var_wdepnulrsti * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard495 == 0.0)) {
            let assign26240_e30529: f64 = (locals.var_zinv - 1.0);
            let assign26240_e30531: f64 = (assign26240_e30529 * locals.var_wdep);
            let assign26240_e30532: f64 = (locals.var_ftdsti * assign26240_e30531);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign26240_e30532, (locals.var_ftdsti * (assign26240_e30529 * locals.var_wdep_dn5)), (locals.var_ftdsti * (assign26240_e30529 * locals.var_wdep_dn6)), (locals.var_ftdsti * (assign26240_e30529 * locals.var_wdep_dn7)), (locals.var_ftdsti * (assign26240_e30529 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard495 == 0.0)) {
            let assign26250_e30547: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign26250_e30548: f64 = (p.p841 * assign26250_e30547);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign26250_e30548, (p.p841 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p841 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign26260_e30553: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard498 = assign26260_e30553;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26280_e30577: f64 = (locals.var_wdep * locals.var_one_minus_psti);
            let assign26280_e30579: f64 = (assign26280_e30577 / locals.var_vbi_minus_vjsrh);
            let assign26280_e30580: f64 = (locals.var_btatpartsti * assign26280_e30579);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign26280_e30580, (locals.var_btatpartsti * ((locals.var_wdep_dn5 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn6 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn7 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartsti * ((locals.var_wdep_dn8 * locals.var_one_minus_psti) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26290_e30594: f64 = (0.666666666666667 * locals.var_atatsti);
            let assign26290_e30596: f64 = (assign26290_e30594 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign26290_e30596, (-((assign26290_e30594 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign26290_e30594 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign26290_e30594 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign26290_e30594 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26300_e30610: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign26300_e30610, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26310_e30624: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign26310_e30627: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign26310_e30629: f64 = (assign26310_e30627 + 1.0);
            let assign26310_e30630: f64 = (assign26310_e30624 / assign26310_e30629);
            let assign26310_e30631: f64 = (assign26310_e30630).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign26310_e30631, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign26310_e30629) - (assign26310_e30624 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign26310_e30629 * assign26310_e30629)) / (2.0 * assign26310_e30631)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign26310_e30629) - (assign26310_e30624 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign26310_e30629 * assign26310_e30629)) / (2.0 * assign26310_e30631)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign26310_e30629) - (assign26310_e30624 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign26310_e30629 * assign26310_e30629)) / (2.0 * assign26310_e30631)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign26310_e30629) - (assign26310_e30624 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign26310_e30629 * assign26310_e30629)) / (2.0 * assign26310_e30631)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26320_e30644: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign26320_e30644, (locals.var_umax_dn5 / (2.0 * assign26320_e30644)), (locals.var_umax_dn6 / (2.0 * assign26320_e30644)), (locals.var_umax_dn7 / (2.0 * assign26320_e30644)), (locals.var_umax_dn8 / (2.0 * assign26320_e30644)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26330_e30658: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign26330_e30658, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign26340_e30662: f64 = (-p.p832);
        let assign26340_e30664: f64 = (assign26340_e30662 * locals.var_one_over_one_minus_psti);
        let assign26340_e30666: f64 = (-1.0);
        let assign26340_e30667: f64 = if assign26340_e30664 == assign26340_e30666 { 1.0 } else { 0.0 };
        locals.var_guard499 = assign26340_e30667;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard499 != 0.0)) {
            let assign26350_e30683: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign26350_e30684: f64 = (1.0 + assign26350_e30683);
            let assign26350_e30685: f64 = (1.0 / assign26350_e30684);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign26350_e30685, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign26350_e30684 * assign26350_e30684))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign26350_e30684 * assign26350_e30684))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign26350_e30684 * assign26350_e30684))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign26350_e30684 * assign26350_e30684))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard499 == 0.0)) {
            let assign26360_e30703: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign26360_e30704: f64 = (1.0 + assign26360_e30703);
            let assign26360_e30706: f64 = (-p.p832);
            let assign26360_e30708: f64 = (assign26360_e30706 * locals.var_one_over_one_minus_psti);
            let assign26360_e30709: f64 = (assign26360_e30704).powf(assign26360_e30708);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign26360_e30709, if 0.0 == 0.0 && ((assign26360_e30708) as f64).is_finite() && ((assign26360_e30708) as f64).fract() == 0.0 { if assign26360_e30708 == 0.0 { 0.0 } else { (assign26360_e30708 * ((assign26360_e30704).powf(assign26360_e30708 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign26360_e30709 * (assign26360_e30708 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign26360_e30704))) }, if 0.0 == 0.0 && ((assign26360_e30708) as f64).is_finite() && ((assign26360_e30708) as f64).fract() == 0.0 { if assign26360_e30708 == 0.0 { 0.0 } else { (assign26360_e30708 * ((assign26360_e30704).powf(assign26360_e30708 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign26360_e30709 * (assign26360_e30708 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign26360_e30704))) }, if 0.0 == 0.0 && ((assign26360_e30708) as f64).is_finite() && ((assign26360_e30708) as f64).fract() == 0.0 { if assign26360_e30708 == 0.0 { 0.0 } else { (assign26360_e30708 * ((assign26360_e30704).powf(assign26360_e30708 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign26360_e30709 * (assign26360_e30708 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign26360_e30704))) }, if 0.0 == 0.0 && ((assign26360_e30708) as f64).is_finite() && ((assign26360_e30708) as f64).fract() == 0.0 { if assign26360_e30708 == 0.0 { 0.0 } else { (assign26360_e30708 * ((assign26360_e30704).powf(assign26360_e30708 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign26360_e30709 * (assign26360_e30708 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign26360_e30704))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26370_e30723: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign26370_e30726: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign26370_e30727: f64 = (assign26370_e30723 / assign26370_e30726);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign26370_e30727, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign26370_e30726) - (assign26370_e30723 * locals.var_wgamma_dn5)) / (assign26370_e30726 * assign26370_e30726)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign26370_e30726) - (assign26370_e30723 * locals.var_wgamma_dn6)) / (assign26370_e30726 * assign26370_e30726)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign26370_e30726) - (assign26370_e30723 * locals.var_wgamma_dn7)) / (assign26370_e30726 * assign26370_e30726)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign26370_e30726) - (assign26370_e30723 * locals.var_wgamma_dn8)) / (assign26370_e30726 * assign26370_e30726)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26380_e30742: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign26380_e30743: f64 = (0.375 * assign26380_e30742);
            let assign26380_e30744: f64 = (assign26380_e30743).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign26380_e30744, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign26380_e30744)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign26380_e30744)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign26380_e30744)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign26380_e30744)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26390_e30759: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign26390_e30760: f64 = (2.0 * assign26390_e30759);
            let assign26390_e30762: f64 = (assign26390_e30760 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign26390_e30762, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26400_e30776: f64 = (locals.var_atatsti * locals.var_twoatatoverthreebtat);
            let assign26400_e30778: f64 = (assign26400_e30776 * locals.var_sqrtumax);
            let assign26400_e30781: f64 = (locals.var_atatsti * locals.var_umax);
            let assign26400_e30782: f64 = (assign26400_e30778 - assign26400_e30781);
            let assign26400_e30786: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign26400_e30787: f64 = (0.5 * assign26400_e30786);
            let assign26400_e30788: f64 = (assign26400_e30782 + assign26400_e30787);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign26400_e30788, (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign26400_e30776 * locals.var_sqrtumax_dn5)) - (locals.var_atatsti * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign26400_e30776 * locals.var_sqrtumax_dn6)) - (locals.var_atatsti * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign26400_e30776 * locals.var_sqrtumax_dn7)) - (locals.var_atatsti * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatsti * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign26400_e30776 * locals.var_sqrtumax_dn8)) - (locals.var_atatsti * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26410_e30802: f64 = (locals.var_ltat - 1.0);
            let assign26410_e30804: f64 = (assign26410_e30802 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign26410_e30804, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign26410_e30802 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign26410_e30802 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign26410_e30802 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign26410_e30802 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26420_e30818: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign26420_e30818, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign26430_e30823: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard500 = assign26430_e30823;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard500 != 0.0)) {
            let assign26440_e30839: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign26440_e30840: f64 = (1.0 + assign26440_e30839);
            let assign26440_e30841: f64 = (1.0 / assign26440_e30840);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign26440_e30841, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign26440_e30840 * assign26440_e30840))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign26440_e30840 * assign26440_e30840))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign26440_e30840 * assign26440_e30840))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign26440_e30840 * assign26440_e30840))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard500 == 0.0)) {
            let assign26450_e30860: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign26450_e30861: f64 = (1.0 - assign26450_e30860);
            let assign26450_e30862: f64 = (1.0 / assign26450_e30861);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign26450_e30862, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign26450_e30861 * assign26450_e30861))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign26450_e30861 * assign26450_e30861))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign26450_e30861 * assign26450_e30861))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign26450_e30861 * assign26450_e30861))), );
        }
        let assign26460_e30866: f64 = (-locals.var_ysq);
        let assign26460_e30868: f64 = (assign26460_e30866 + locals.var_mtat);
        let assign26460_e30870: f64 = (-230.25850929940458);
        let assign26460_e30871: f64 = if assign26460_e30868 > assign26460_e30870 { 1.0 } else { 0.0 };
        locals.var_guard501 = assign26460_e30871;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard501 != 0.0)) {
            let assign26470_e30884: f64 = (-locals.var_ysq);
            let assign26470_e30886: f64 = (assign26470_e30884 + locals.var_mtat);
            let assign26470_e30887: f64 = (assign26470_e30886).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26470_e30887, (assign26470_e30887 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign26470_e30887 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign26470_e30887 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign26470_e30887 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard501 == 0.0)) {
            let assign26480_e30905: f64 = (-230.25850929940458);
            let assign26480_e30907: f64 = (-locals.var_ysq);
            let assign26480_e30909: f64 = (assign26480_e30907 + locals.var_mtat);
            let assign26480_e30910: f64 = (assign26480_e30905 - assign26480_e30909);
            let assign26480_e30914: f64 = (-230.25850929940458);
            let assign26480_e30916: f64 = (-locals.var_ysq);
            let assign26480_e30918: f64 = (assign26480_e30916 + locals.var_mtat);
            let assign26480_e30919: f64 = (assign26480_e30914 - assign26480_e30918);
            let assign26480_e30922: f64 = (-230.25850929940458);
            let assign26480_e30924: f64 = (-locals.var_ysq);
            let assign26480_e30926: f64 = (assign26480_e30924 + locals.var_mtat);
            let assign26480_e30927: f64 = (assign26480_e30922 - assign26480_e30926);
            let assign26480_e30929: f64 = (assign26480_e30927 * 0.3333333333333333);
            let assign26480_e30930: f64 = (1.0 + assign26480_e30929);
            let assign26480_e30931: f64 = (assign26480_e30919 * assign26480_e30930);
            let assign26480_e30932: f64 = (0.5 * assign26480_e30931);
            let assign26480_e30933: f64 = (1.0 + assign26480_e30932);
            let assign26480_e30934: f64 = (assign26480_e30910 * assign26480_e30933);
            let assign26480_e30935: f64 = (1.0 + assign26480_e30934);
            let assign26480_e30936: f64 = (1e-100 / assign26480_e30935);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26480_e30936, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign26480_e30933) + (assign26480_e30910 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign26480_e30930) + (assign26480_e30919 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign26480_e30935 * assign26480_e30935))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign26480_e30933) + (assign26480_e30910 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign26480_e30930) + (assign26480_e30919 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign26480_e30935 * assign26480_e30935))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign26480_e30933) + (assign26480_e30910 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign26480_e30930) + (assign26480_e30919 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign26480_e30935 * assign26480_e30935))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign26480_e30933) + (assign26480_e30910 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign26480_e30930) + (assign26480_e30919 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign26480_e30935 * assign26480_e30935))), );
        }
    }
    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26490_e30950: f64 = (0.29214664 * locals.var_terfc);
            let assign26490_e30954: f64 = (locals.var_terfc * locals.var_terfc);
            let assign26490_e30955: f64 = (locals.var_berfc * assign26490_e30954);
            let assign26490_e30956: f64 = (assign26490_e30950 + assign26490_e30955);
            let assign26490_e30960: f64 = (locals.var_terfc * locals.var_terfc);
            let assign26490_e30962: f64 = (assign26490_e30960 * locals.var_terfc);
            let assign26490_e30963: f64 = (locals.var_cerfc * assign26490_e30962);
            let assign26490_e30964: f64 = (assign26490_e30956 + assign26490_e30963);
            let assign26490_e30966: f64 = (assign26490_e30964 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign26490_e30966, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign26490_e30960 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign26490_e30964 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign26490_e30960 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign26490_e30964 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign26490_e30960 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign26490_e30964 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign26490_e30960 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign26490_e30964 * locals.var_tmp_dn8)), );
        }
        let assign26500_e30971: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard502 = assign26500_e30971;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard502 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign26520_e30988: f64 = (-230.25850929940458);
        let assign26520_e30989: f64 = if locals.var_mtat > assign26520_e30988 { 1.0 } else { 0.0 };
        locals.var_guard503 = assign26520_e30989;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard502 == 0.0)) && (locals.var_guard503 != 0.0)) {
            let assign26530_e31005: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26530_e31005, (assign26530_e31005 * locals.var_mtat_dn5), (assign26530_e31005 * locals.var_mtat_dn6), (assign26530_e31005 * locals.var_mtat_dn7), (assign26530_e31005 * locals.var_mtat_dn8), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard502 == 0.0)) && (locals.var_guard503 == 0.0)) {
            let assign26540_e31026: f64 = (-230.25850929940458);
            let assign26540_e31028: f64 = (assign26540_e31026 - locals.var_mtat);
            let assign26540_e31032: f64 = (-230.25850929940458);
            let assign26540_e31034: f64 = (assign26540_e31032 - locals.var_mtat);
            let assign26540_e31037: f64 = (-230.25850929940458);
            let assign26540_e31039: f64 = (assign26540_e31037 - locals.var_mtat);
            let assign26540_e31041: f64 = (assign26540_e31039 * 0.3333333333333333);
            let assign26540_e31042: f64 = (1.0 + assign26540_e31041);
            let assign26540_e31043: f64 = (assign26540_e31034 * assign26540_e31042);
            let assign26540_e31044: f64 = (0.5 * assign26540_e31043);
            let assign26540_e31045: f64 = (1.0 + assign26540_e31044);
            let assign26540_e31046: f64 = (assign26540_e31028 * assign26540_e31045);
            let assign26540_e31047: f64 = (1.0 + assign26540_e31046);
            let assign26540_e31048: f64 = (1e-100 / assign26540_e31047);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26540_e31048, (-((1e-100 * (((-locals.var_mtat_dn5) * assign26540_e31045) + (assign26540_e31028 * (0.5 * (((-locals.var_mtat_dn5) * assign26540_e31042) + (assign26540_e31034 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign26540_e31047 * assign26540_e31047))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign26540_e31045) + (assign26540_e31028 * (0.5 * (((-locals.var_mtat_dn6) * assign26540_e31042) + (assign26540_e31034 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign26540_e31047 * assign26540_e31047))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign26540_e31045) + (assign26540_e31028 * (0.5 * (((-locals.var_mtat_dn7) * assign26540_e31042) + (assign26540_e31034 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign26540_e31047 * assign26540_e31047))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign26540_e31045) + (assign26540_e31028 * (0.5 * (((-locals.var_mtat_dn8) * assign26540_e31042) + (assign26540_e31034 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign26540_e31047 * assign26540_e31047))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard502 == 0.0)) {
            let assign26550_e31065: f64 = (2.0 * locals.var_tmp);
            let assign26550_e31067: f64 = (assign26550_e31065 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign26550_e31067, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26560_e31081: f64 = (1.772453850905516 * 0.5);
            let assign26560_e31084: f64 = (locals.var_atatsti * locals.var_erfctimesexpmtat);
            let assign26560_e31086: f64 = (assign26560_e31084 / locals.var_ktat);
            let assign26560_e31087: f64 = (assign26560_e31081 * assign26560_e31086);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign26560_e31087, (assign26560_e31081 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign26560_e31084 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign26560_e31081 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign26560_e31084 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign26560_e31081 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign26560_e31084 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign26560_e31081 * ((((locals.var_atatsti * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign26560_e31084 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard498 == 0.0)) {
            let assign26570_e31102: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign26570_e31104: f64 = (assign26570_e31102 * locals.var_wtat);
            let assign26570_e31105: f64 = (p.p846 * assign26570_e31104);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign26570_e31105, (p.p846 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign26570_e31102 * locals.var_wtat_dn5))), (p.p846 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign26570_e31102 * locals.var_wtat_dn6))), (p.p846 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign26570_e31102 * locals.var_wtat_dn7))), (p.p846 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign26570_e31102 * locals.var_wtat_dn8))), );
        }
        let assign26580_e31110: f64 = if p.p852 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard504 = assign26580_e31110;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard504 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign26600_e31124: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard505 = assign26600_e31124;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard504 == 0.0)) && (locals.var_guard505 != 0.0)) {
            let assign26610_e31138: f64 = (p.p829 - locals.var_vbbt);
            let assign26610_e31140: f64 = (assign26610_e31138 * locals.var_vbirstiinv);
            let assign26610_e31141: f64 = (assign26610_e31140).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26610_e31141, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard504 == 0.0)) && (locals.var_guard505 == 0.0)) {
            let assign26620_e31158: f64 = (p.p829 - locals.var_vbbt);
            let assign26620_e31160: f64 = (assign26620_e31158 * locals.var_vbirstiinv);
            let assign26620_e31162: f64 = (assign26620_e31160).powf(p.p832);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26620_e31162, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard504 == 0.0)) {
            let assign26630_e31177: f64 = (p.p829 - locals.var_vbbt);
            let assign26630_e31179: f64 = (assign26630_e31177 * locals.var_wdepnulrinvsti);
            let assign26630_e31181: f64 = (assign26630_e31179 / locals.var_tmp);
            let assign26630_e31182: f64 = (locals.var_one_over_one_minus_psti * assign26630_e31181);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign26630_e31182, (locals.var_one_over_one_minus_psti * (-((assign26630_e31179 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign26630_e31179 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign26630_e31179 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_psti * (-((assign26630_e31179 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign26640_e31186: f64 = (-locals.var_fbbtsti);
        let assign26640_e31188: f64 = (assign26640_e31186 / locals.var_fmaxr);
        let assign26640_e31189: f64 = (assign26640_e31188).abs();
        let assign26640_e31191: f64 = if assign26640_e31189 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard506 = assign26640_e31191;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard504 == 0.0)) && (locals.var_guard506 != 0.0)) {
            let assign26650_e31204: f64 = (-locals.var_fbbtsti);
            let assign26650_e31206: f64 = (assign26650_e31204 / locals.var_fmaxr);
            let assign26650_e31207: f64 = (assign26650_e31206).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26650_e31207, (assign26650_e31207 * (-((assign26650_e31204 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign26650_e31207 * (-((assign26650_e31204 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign26650_e31207 * (-((assign26650_e31204 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))), (assign26650_e31207 * (-((assign26650_e31204 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))), );
        }
        let assign26660_e31211: f64 = (-locals.var_fbbtsti);
        let assign26660_e31213: f64 = (assign26660_e31211 / locals.var_fmaxr);
        let assign26660_e31215: f64 = if assign26660_e31213 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard507 = assign26660_e31215;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard504 == 0.0)) && (locals.var_guard506 == 0.0)) && (locals.var_guard507 != 0.0)) {
            let assign26670_e31233: f64 = (-230.25850929940458);
            let assign26670_e31235: f64 = (-locals.var_fbbtsti);
            let assign26670_e31237: f64 = (assign26670_e31235 / locals.var_fmaxr);
            let assign26670_e31238: f64 = (assign26670_e31233 - assign26670_e31237);
            let assign26670_e31242: f64 = (-230.25850929940458);
            let assign26670_e31244: f64 = (-locals.var_fbbtsti);
            let assign26670_e31246: f64 = (assign26670_e31244 / locals.var_fmaxr);
            let assign26670_e31247: f64 = (assign26670_e31242 - assign26670_e31246);
            let assign26670_e31250: f64 = (-230.25850929940458);
            let assign26670_e31252: f64 = (-locals.var_fbbtsti);
            let assign26670_e31254: f64 = (assign26670_e31252 / locals.var_fmaxr);
            let assign26670_e31255: f64 = (assign26670_e31250 - assign26670_e31254);
            let assign26670_e31257: f64 = (assign26670_e31255 * 0.3333333333333333);
            let assign26670_e31258: f64 = (1.0 + assign26670_e31257);
            let assign26670_e31259: f64 = (assign26670_e31247 * assign26670_e31258);
            let assign26670_e31260: f64 = (0.5 * assign26670_e31259);
            let assign26670_e31261: f64 = (1.0 + assign26670_e31260);
            let assign26670_e31262: f64 = (assign26670_e31238 * assign26670_e31261);
            let assign26670_e31263: f64 = (1.0 + assign26670_e31262);
            let assign26670_e31264: f64 = (1e-100 / assign26670_e31263);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26670_e31264, (-((1e-100 * (((-(-((assign26670_e31235 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign26670_e31261) + (assign26670_e31238 * (0.5 * (((-(-((assign26670_e31244 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign26670_e31258) + (assign26670_e31247 * ((-(-((assign26670_e31252 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign26670_e31263 * assign26670_e31263))), (-((1e-100 * (((-(-((assign26670_e31235 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign26670_e31261) + (assign26670_e31238 * (0.5 * (((-(-((assign26670_e31244 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign26670_e31258) + (assign26670_e31247 * ((-(-((assign26670_e31252 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign26670_e31263 * assign26670_e31263))), (-((1e-100 * (((-(-((assign26670_e31235 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign26670_e31261) + (assign26670_e31238 * (0.5 * (((-(-((assign26670_e31244 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign26670_e31258) + (assign26670_e31247 * ((-(-((assign26670_e31252 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign26670_e31263 * assign26670_e31263))), (-((1e-100 * (((-(-((assign26670_e31235 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign26670_e31261) + (assign26670_e31238 * (0.5 * (((-(-((assign26670_e31244 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * assign26670_e31258) + (assign26670_e31247 * ((-(-((assign26670_e31252 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr)))) * 0.3333333333333333))))))) / (assign26670_e31263 * assign26670_e31263))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard504 == 0.0)) && (locals.var_guard506 == 0.0)) && (locals.var_guard507 == 0.0)) {
            let assign26680_e31285: f64 = (-locals.var_fbbtsti);
            let assign26680_e31287: f64 = (assign26680_e31285 / locals.var_fmaxr);
            let assign26680_e31289: f64 = (assign26680_e31287 - 230.25850929940458);
            let assign26680_e31293: f64 = (-locals.var_fbbtsti);
            let assign26680_e31295: f64 = (assign26680_e31293 / locals.var_fmaxr);
            let assign26680_e31297: f64 = (assign26680_e31295 - 230.25850929940458);
            let assign26680_e31300: f64 = (-locals.var_fbbtsti);
            let assign26680_e31302: f64 = (assign26680_e31300 / locals.var_fmaxr);
            let assign26680_e31304: f64 = (assign26680_e31302 - 230.25850929940458);
            let assign26680_e31306: f64 = (assign26680_e31304 * 0.3333333333333333);
            let assign26680_e31307: f64 = (1.0 + assign26680_e31306);
            let assign26680_e31308: f64 = (assign26680_e31297 * assign26680_e31307);
            let assign26680_e31309: f64 = (0.5 * assign26680_e31308);
            let assign26680_e31310: f64 = (1.0 + assign26680_e31309);
            let assign26680_e31311: f64 = (assign26680_e31289 * assign26680_e31310);
            let assign26680_e31312: f64 = (1.0 + assign26680_e31311);
            let assign26680_e31313: f64 = (1e100 * assign26680_e31312);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26680_e31313, (1e100 * (((-((assign26680_e31285 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign26680_e31310) + (assign26680_e31289 * (0.5 * (((-((assign26680_e31293 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * assign26680_e31307) + (assign26680_e31297 * ((-((assign26680_e31300 * locals.var_fmaxr_dn5) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign26680_e31285 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign26680_e31310) + (assign26680_e31289 * (0.5 * (((-((assign26680_e31293 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * assign26680_e31307) + (assign26680_e31297 * ((-((assign26680_e31300 * locals.var_fmaxr_dn6) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign26680_e31285 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign26680_e31310) + (assign26680_e31289 * (0.5 * (((-((assign26680_e31293 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * assign26680_e31307) + (assign26680_e31297 * ((-((assign26680_e31300 * locals.var_fmaxr_dn7) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign26680_e31285 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign26680_e31310) + (assign26680_e31289 * (0.5 * (((-((assign26680_e31293 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * assign26680_e31307) + (assign26680_e31297 * ((-((assign26680_e31300 * locals.var_fmaxr_dn8) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard504 == 0.0)) {
            let assign26690_e31328: f64 = (locals.var_v5 * locals.var_fmaxr);
            let assign26690_e31330: f64 = (assign26690_e31328 * locals.var_fmaxr);
            let assign26690_e31332: f64 = (assign26690_e31330 * locals.var_tmp);
            let assign26690_e31333: f64 = (p.p852 * assign26690_e31332);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign26690_e31333, (p.p852 * (((((locals.var_v5 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign26690_e31328 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign26690_e31330 * locals.var_tmp_dn5))), (p.p852 * (((((locals.var_v5 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign26690_e31328 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign26690_e31330 * locals.var_tmp_dn6))), (p.p852 * (((((locals.var_v5 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign26690_e31328 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign26690_e31330 * locals.var_tmp_dn7))), (p.p852 * (((((locals.var_v5 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign26690_e31328 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign26690_e31330 * locals.var_tmp_dn8))), );
        }
        let assign26700_e31338: f64 = if p.p861 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard508 = assign26700_e31338;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard508 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign26720_e31352: f64 = (-locals.var_alphaav);
        let assign26720_e31354: f64 = (assign26720_e31352 * p.p861);
        let assign26720_e31355: f64 = if locals.var_vav > assign26720_e31354 { 1.0 } else { 0.0 };
        locals.var_guard509 = assign26720_e31355;
        let assign26730_e31358: f64 = if p.p864 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard510 = assign26730_e31358;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard508 == 0.0)) && (locals.var_guard509 != 0.0)) && (locals.var_guard510 != 0.0)) {
            let assign26740_e31374: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign26740_e31377: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign26740_e31378: f64 = (assign26740_e31374 * assign26740_e31377);
            let assign26740_e31381: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign26740_e31382: f64 = (assign26740_e31378 * assign26740_e31381);
            let assign26740_e31385: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign26740_e31386: f64 = (assign26740_e31382 * assign26740_e31385);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26740_e31386, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard508 == 0.0)) && (locals.var_guard509 != 0.0)) && (locals.var_guard510 == 0.0)) {
            let assign26750_e31405: f64 = (locals.var_vav * locals.var_vbrinvsti);
            let assign26750_e31406: f64 = (assign26750_e31405).abs();
            let assign26750_e31408: f64 = (assign26750_e31406).powf(p.p864);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26750_e31408, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard508 == 0.0)) && (locals.var_guard509 != 0.0)) {
            let assign26760_e31425: f64 = (1.0 - locals.var_tmp);
            let assign26760_e31426: f64 = (1.0 / assign26760_e31425);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign26760_e31426, (-((-locals.var_tmp_dn5) / (assign26760_e31425 * assign26760_e31425))), (-((-locals.var_tmp_dn6) / (assign26760_e31425 * assign26760_e31425))), (-((-locals.var_tmp_dn7) / (assign26760_e31425 * assign26760_e31425))), (-((-locals.var_tmp_dn8) / (assign26760_e31425 * assign26760_e31425))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) && (locals.var_guard508 == 0.0)) && (locals.var_guard509 == 0.0)) {
            let assign26770_e31445: f64 = (locals.var_alphaav * p.p861);
            let assign26770_e31446: f64 = (locals.var_vav + assign26770_e31445);
            let assign26770_e31448: f64 = (assign26770_e31446 * locals.var_slopesti);
            let assign26770_e31449: f64 = (locals.var_fstopsti + assign26770_e31448);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign26770_e31449, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard494 == 0.0)) {
            let assign26780_e31461: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign26780_e31463: f64 = (assign26780_e31461 + locals.var_itat);
            let assign26780_e31465: f64 = (assign26780_e31463 + locals.var_ibbt);
            let assign26780_e31466: f64 = (p.p29 * assign26780_e31465);
            let assign26780_e31468: f64 = (assign26780_e31466 * locals.var_fbreakdown);
            (locals.var_ijunsti, locals.var_ijunsti_dn5, locals.var_ijunsti_dn6, locals.var_ijunsti_dn7, locals.var_ijunsti_dn8, ) = (assign26780_e31468, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign26780_e31466 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign26780_e31466 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign26780_e31466 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign26780_e31466 * locals.var_fbreakdown_dn8)), );
        }
        let assign26790_e31473: f64 = if locals.var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard511 = assign26790_e31473;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 != 0.0)) {
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) {
            let assign26810_e31490: f64 = (locals.var_idsatgat * locals.var_idmult);
            locals.var_id__blk219 = assign26810_e31490;
        }
        let assign26820_e31499: f64 = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard512 = assign26820_e31499;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard512 != 0.0)) {
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard512 == 0.0)) {
            let assign26840_e31522: f64 = (locals.var_vbigat - locals.var_vjsrh);
            locals.var_vbi_minus_vjsrh = assign26840_e31522;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard512 == 0.0)) {
            let assign26850_e31538: f64 = (locals.var_two_psistar / locals.var_vbi_minus_vjsrh);
            let assign26850_e31539: f64 = (1.0 - assign26850_e31538);
            let assign26850_e31540: f64 = (assign26850_e31539).sqrt();
            let assign26850_e31541: f64 = (1.0 - assign26850_e31540);
            locals.var_wsrhstep = assign26850_e31541;
        }
        let assign26860_e31546: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard513 = assign26860_e31546;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard512 == 0.0)) && (locals.var_guard513 != 0.0)) {
            locals.var_dwsrh = 0.0;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard512 == 0.0)) && (locals.var_guard513 == 0.0)) {
            let assign26880_e31575: f64 = (locals.var_wsrhstep * locals.var_wsrhstep);
            let assign26880_e31577: f64 = (locals.var_wsrhstep).ln();
            let assign26880_e31578: f64 = (assign26880_e31575 * assign26880_e31577);
            let assign26880_e31581: f64 = (1.0 - locals.var_wsrhstep);
            let assign26880_e31582: f64 = (assign26880_e31578 / assign26880_e31581);
            let assign26880_e31584: f64 = (assign26880_e31582 + locals.var_wsrhstep);
            let assign26880_e31588: f64 = (2.0 * p.p833);
            let assign26880_e31589: f64 = (1.0 - assign26880_e31588);
            let assign26880_e31590: f64 = (assign26880_e31584 * assign26880_e31589);
            locals.var_dwsrh = assign26880_e31590;
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard512 == 0.0)) {
            let assign26890_e31604: f64 = (locals.var_wsrhstep + locals.var_dwsrh);
            locals.var_wsrh = assign26890_e31604;
        }
        let assign26900_e31609: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard514 = assign26900_e31609;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard512 == 0.0)) && (locals.var_guard514 != 0.0)) {
            let assign26910_e31623: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv);
            let assign26910_e31624: f64 = (assign26910_e31623).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26910_e31624, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard512 == 0.0)) && (locals.var_guard514 == 0.0)) {
            let assign26920_e31641: f64 = (locals.var_vbi_minus_vjsrh * locals.var_vbirgatinv);
            let assign26920_e31643: f64 = (assign26920_e31641).powf(p.p833);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign26920_e31643, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard512 == 0.0)) {
            let assign26930_e31657: f64 = (locals.var_wdepnulrgat * locals.var_tmp);
            (locals.var_wdep, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, ) = (assign26930_e31657, (locals.var_wdepnulrgat * locals.var_tmp_dn5), (locals.var_wdepnulrgat * locals.var_tmp_dn6), (locals.var_wdepnulrgat * locals.var_tmp_dn7), (locals.var_wdepnulrgat * locals.var_tmp_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard512 == 0.0)) {
            let assign26940_e31672: f64 = (locals.var_zinv - 1.0);
            let assign26940_e31674: f64 = (assign26940_e31672 * locals.var_wdep);
            let assign26940_e31675: f64 = (locals.var_ftdgat * assign26940_e31674);
            (locals.var_asrh, locals.var_asrh_dn5, locals.var_asrh_dn6, locals.var_asrh_dn7, locals.var_asrh_dn8, ) = (assign26940_e31675, (locals.var_ftdgat * (assign26940_e31672 * locals.var_wdep_dn5)), (locals.var_ftdgat * (assign26940_e31672 * locals.var_wdep_dn6)), (locals.var_ftdgat * (assign26940_e31672 * locals.var_wdep_dn7)), (locals.var_ftdgat * (assign26940_e31672 * locals.var_wdep_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard512 == 0.0)) {
            let assign26950_e31690: f64 = (locals.var_asrh * locals.var_wsrh);
            let assign26950_e31691: f64 = (p.p842 * assign26950_e31690);
            (locals.var_isrh, locals.var_isrh_dn5, locals.var_isrh_dn6, locals.var_isrh_dn7, locals.var_isrh_dn8, ) = (assign26950_e31691, (p.p842 * (locals.var_asrh_dn5 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn6 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn7 * locals.var_wsrh)), (p.p842 * (locals.var_asrh_dn8 * locals.var_wsrh)), );
        }
        let assign26960_e31696: f64 = if p.p847 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard515 = assign26960_e31696;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 != 0.0)) {
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign26980_e31720: f64 = (locals.var_wdep * locals.var_one_minus_pgat);
            let assign26980_e31722: f64 = (assign26980_e31720 / locals.var_vbi_minus_vjsrh);
            let assign26980_e31723: f64 = (locals.var_btatpartgat * assign26980_e31722);
            (locals.var_btat, locals.var_btat_dn5, locals.var_btat_dn6, locals.var_btat_dn7, locals.var_btat_dn8, ) = (assign26980_e31723, (locals.var_btatpartgat * ((locals.var_wdep_dn5 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn6 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn7 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), (locals.var_btatpartgat * ((locals.var_wdep_dn8 * locals.var_one_minus_pgat) / locals.var_vbi_minus_vjsrh)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign26990_e31737: f64 = (0.666666666666667 * locals.var_atatgat);
            let assign26990_e31739: f64 = (assign26990_e31737 / locals.var_btat);
            (locals.var_twoatatoverthreebtat, locals.var_twoatatoverthreebtat_dn5, locals.var_twoatatoverthreebtat_dn6, locals.var_twoatatoverthreebtat_dn7, locals.var_twoatatoverthreebtat_dn8, ) = (assign26990_e31739, (-((assign26990_e31737 * locals.var_btat_dn5) / (locals.var_btat * locals.var_btat))), (-((assign26990_e31737 * locals.var_btat_dn6) / (locals.var_btat * locals.var_btat))), (-((assign26990_e31737 * locals.var_btat_dn7) / (locals.var_btat * locals.var_btat))), (-((assign26990_e31737 * locals.var_btat_dn8) / (locals.var_btat * locals.var_btat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27000_e31753: f64 = (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat);
            (locals.var_umaxbeforelimiting, locals.var_umaxbeforelimiting_dn5, locals.var_umaxbeforelimiting_dn6, locals.var_umaxbeforelimiting_dn7, locals.var_umaxbeforelimiting_dn8, ) = (assign27000_e31753, ((locals.var_twoatatoverthreebtat_dn5 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn5)), ((locals.var_twoatatoverthreebtat_dn6 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn6)), ((locals.var_twoatatoverthreebtat_dn7 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn7)), ((locals.var_twoatatoverthreebtat_dn8 * locals.var_twoatatoverthreebtat) + (locals.var_twoatatoverthreebtat * locals.var_twoatatoverthreebtat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27010_e31767: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign27010_e31770: f64 = (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting);
            let assign27010_e31772: f64 = (assign27010_e31770 + 1.0);
            let assign27010_e31773: f64 = (assign27010_e31767 / assign27010_e31772);
            let assign27010_e31774: f64 = (assign27010_e31773).sqrt();
            (locals.var_umax, locals.var_umax_dn5, locals.var_umax_dn6, locals.var_umax_dn7, locals.var_umax_dn8, ) = (assign27010_e31774, ((((((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)) * assign27010_e31772) - (assign27010_e31767 * ((locals.var_umaxbeforelimiting_dn5 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn5)))) / (assign27010_e31772 * assign27010_e31772)) / (2.0 * assign27010_e31774)), ((((((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)) * assign27010_e31772) - (assign27010_e31767 * ((locals.var_umaxbeforelimiting_dn6 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn6)))) / (assign27010_e31772 * assign27010_e31772)) / (2.0 * assign27010_e31774)), ((((((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)) * assign27010_e31772) - (assign27010_e31767 * ((locals.var_umaxbeforelimiting_dn7 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn7)))) / (assign27010_e31772 * assign27010_e31772)) / (2.0 * assign27010_e31774)), ((((((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)) * assign27010_e31772) - (assign27010_e31767 * ((locals.var_umaxbeforelimiting_dn8 * locals.var_umaxbeforelimiting) + (locals.var_umaxbeforelimiting * locals.var_umaxbeforelimiting_dn8)))) / (assign27010_e31772 * assign27010_e31772)) / (2.0 * assign27010_e31774)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27020_e31787: f64 = (locals.var_umax).sqrt();
            (locals.var_sqrtumax, locals.var_sqrtumax_dn5, locals.var_sqrtumax_dn6, locals.var_sqrtumax_dn7, locals.var_sqrtumax_dn8, ) = (assign27020_e31787, (locals.var_umax_dn5 / (2.0 * assign27020_e31787)), (locals.var_umax_dn6 / (2.0 * assign27020_e31787)), (locals.var_umax_dn7 / (2.0 * assign27020_e31787)), (locals.var_umax_dn8 / (2.0 * assign27020_e31787)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27030_e31801: f64 = (locals.var_umax * locals.var_sqrtumax);
            (locals.var_umaxpoweronepointfive, locals.var_umaxpoweronepointfive_dn5, locals.var_umaxpoweronepointfive_dn6, locals.var_umaxpoweronepointfive_dn7, locals.var_umaxpoweronepointfive_dn8, ) = (assign27030_e31801, ((locals.var_umax_dn5 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn5)), ((locals.var_umax_dn6 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn6)), ((locals.var_umax_dn7 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn7)), ((locals.var_umax_dn8 * locals.var_sqrtumax) + (locals.var_umax * locals.var_sqrtumax_dn8)), );
        }
        let assign27040_e31805: f64 = (-p.p833);
        let assign27040_e31807: f64 = (assign27040_e31805 * locals.var_one_over_one_minus_pgat);
        let assign27040_e31809: f64 = (-1.0);
        let assign27040_e31810: f64 = if assign27040_e31807 == assign27040_e31809 { 1.0 } else { 0.0 };
        locals.var_guard516 = assign27040_e31810;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) && (locals.var_guard516 != 0.0)) {
            let assign27050_e31826: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign27050_e31827: f64 = (1.0 + assign27050_e31826);
            let assign27050_e31828: f64 = (1.0 / assign27050_e31827);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign27050_e31828, (-(((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / (assign27050_e31827 * assign27050_e31827))), (-(((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / (assign27050_e31827 * assign27050_e31827))), (-(((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / (assign27050_e31827 * assign27050_e31827))), (-(((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / (assign27050_e31827 * assign27050_e31827))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) && (locals.var_guard516 == 0.0)) {
            let assign27060_e31846: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign27060_e31847: f64 = (1.0 + assign27060_e31846);
            let assign27060_e31849: f64 = (-p.p833);
            let assign27060_e31851: f64 = (assign27060_e31849 * locals.var_one_over_one_minus_pgat);
            let assign27060_e31852: f64 = (assign27060_e31847).powf(assign27060_e31851);
            (locals.var_wgamma, locals.var_wgamma_dn5, locals.var_wgamma_dn6, locals.var_wgamma_dn7, locals.var_wgamma_dn8, ) = (assign27060_e31852, if 0.0 == 0.0 && ((assign27060_e31851) as f64).is_finite() && ((assign27060_e31851) as f64).fract() == 0.0 { if assign27060_e31851 == 0.0 { 0.0 } else { (assign27060_e31851 * ((assign27060_e31847).powf(assign27060_e31851 - 1.0) * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))) } } else { (assign27060_e31852 * (assign27060_e31851 * (((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)) / assign27060_e31847))) }, if 0.0 == 0.0 && ((assign27060_e31851) as f64).is_finite() && ((assign27060_e31851) as f64).fract() == 0.0 { if assign27060_e31851 == 0.0 { 0.0 } else { (assign27060_e31851 * ((assign27060_e31847).powf(assign27060_e31851 - 1.0) * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))) } } else { (assign27060_e31852 * (assign27060_e31851 * (((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)) / assign27060_e31847))) }, if 0.0 == 0.0 && ((assign27060_e31851) as f64).is_finite() && ((assign27060_e31851) as f64).fract() == 0.0 { if assign27060_e31851 == 0.0 { 0.0 } else { (assign27060_e31851 * ((assign27060_e31847).powf(assign27060_e31851 - 1.0) * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))) } } else { (assign27060_e31852 * (assign27060_e31851 * (((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)) / assign27060_e31847))) }, if 0.0 == 0.0 && ((assign27060_e31851) as f64).is_finite() && ((assign27060_e31851) as f64).fract() == 0.0 { if assign27060_e31851 == 0.0 { 0.0 } else { (assign27060_e31851 * ((assign27060_e31847).powf(assign27060_e31851 - 1.0) * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))) } } else { (assign27060_e31852 * (assign27060_e31851 * (((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)) / assign27060_e31847))) }, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27070_e31866: f64 = (locals.var_wsrh * locals.var_wgamma);
            let assign27070_e31869: f64 = (locals.var_wsrh + locals.var_wgamma);
            let assign27070_e31870: f64 = (assign27070_e31866 / assign27070_e31869);
            (locals.var_wtat, locals.var_wtat_dn5, locals.var_wtat_dn6, locals.var_wtat_dn7, locals.var_wtat_dn8, ) = (assign27070_e31870, ((((locals.var_wsrh * locals.var_wgamma_dn5) * assign27070_e31869) - (assign27070_e31866 * locals.var_wgamma_dn5)) / (assign27070_e31869 * assign27070_e31869)), ((((locals.var_wsrh * locals.var_wgamma_dn6) * assign27070_e31869) - (assign27070_e31866 * locals.var_wgamma_dn6)) / (assign27070_e31869 * assign27070_e31869)), ((((locals.var_wsrh * locals.var_wgamma_dn7) * assign27070_e31869) - (assign27070_e31866 * locals.var_wgamma_dn7)) / (assign27070_e31869 * assign27070_e31869)), ((((locals.var_wsrh * locals.var_wgamma_dn8) * assign27070_e31869) - (assign27070_e31866 * locals.var_wgamma_dn8)) / (assign27070_e31869 * assign27070_e31869)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27080_e31885: f64 = (locals.var_btat / locals.var_sqrtumax);
            let assign27080_e31886: f64 = (0.375 * assign27080_e31885);
            let assign27080_e31887: f64 = (assign27080_e31886).sqrt();
            (locals.var_ktat, locals.var_ktat_dn5, locals.var_ktat_dn6, locals.var_ktat_dn7, locals.var_ktat_dn8, ) = (assign27080_e31887, ((0.375 * (((locals.var_btat_dn5 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn5)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign27080_e31887)), ((0.375 * (((locals.var_btat_dn6 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn6)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign27080_e31887)), ((0.375 * (((locals.var_btat_dn7 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn7)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign27080_e31887)), ((0.375 * (((locals.var_btat_dn8 * locals.var_sqrtumax) - (locals.var_btat * locals.var_sqrtumax_dn8)) / (locals.var_sqrtumax * locals.var_sqrtumax))) / (2.0 * assign27080_e31887)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27090_e31902: f64 = (locals.var_twoatatoverthreebtat * locals.var_sqrtumax);
            let assign27090_e31903: f64 = (2.0 * assign27090_e31902);
            let assign27090_e31905: f64 = (assign27090_e31903 - locals.var_umax);
            (locals.var_ltat, locals.var_ltat_dn5, locals.var_ltat_dn6, locals.var_ltat_dn7, locals.var_ltat_dn8, ) = (assign27090_e31905, ((2.0 * ((locals.var_twoatatoverthreebtat_dn5 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn5))) - locals.var_umax_dn5), ((2.0 * ((locals.var_twoatatoverthreebtat_dn6 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn6))) - locals.var_umax_dn6), ((2.0 * ((locals.var_twoatatoverthreebtat_dn7 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn7))) - locals.var_umax_dn7), ((2.0 * ((locals.var_twoatatoverthreebtat_dn8 * locals.var_sqrtumax) + (locals.var_twoatatoverthreebtat * locals.var_sqrtumax_dn8))) - locals.var_umax_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27100_e31919: f64 = (locals.var_atatgat * locals.var_twoatatoverthreebtat);
            let assign27100_e31921: f64 = (assign27100_e31919 * locals.var_sqrtumax);
            let assign27100_e31924: f64 = (locals.var_atatgat * locals.var_umax);
            let assign27100_e31925: f64 = (assign27100_e31921 - assign27100_e31924);
            let assign27100_e31929: f64 = (locals.var_btat * locals.var_umaxpoweronepointfive);
            let assign27100_e31930: f64 = (0.5 * assign27100_e31929);
            let assign27100_e31931: f64 = (assign27100_e31925 + assign27100_e31930);
            (locals.var_mtat, locals.var_mtat_dn5, locals.var_mtat_dn6, locals.var_mtat_dn7, locals.var_mtat_dn8, ) = (assign27100_e31931, (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn5) * locals.var_sqrtumax) + (assign27100_e31919 * locals.var_sqrtumax_dn5)) - (locals.var_atatgat * locals.var_umax_dn5)) + (0.5 * ((locals.var_btat_dn5 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn5)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn6) * locals.var_sqrtumax) + (assign27100_e31919 * locals.var_sqrtumax_dn6)) - (locals.var_atatgat * locals.var_umax_dn6)) + (0.5 * ((locals.var_btat_dn6 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn6)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn7) * locals.var_sqrtumax) + (assign27100_e31919 * locals.var_sqrtumax_dn7)) - (locals.var_atatgat * locals.var_umax_dn7)) + (0.5 * ((locals.var_btat_dn7 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn7)))), (((((locals.var_atatgat * locals.var_twoatatoverthreebtat_dn8) * locals.var_sqrtumax) + (assign27100_e31919 * locals.var_sqrtumax_dn8)) - (locals.var_atatgat * locals.var_umax_dn8)) + (0.5 * ((locals.var_btat_dn8 * locals.var_umaxpoweronepointfive) + (locals.var_btat * locals.var_umaxpoweronepointfive_dn8)))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27110_e31945: f64 = (locals.var_ltat - 1.0);
            let assign27110_e31947: f64 = (assign27110_e31945 * locals.var_ktat);
            (locals.var_xerfc, locals.var_xerfc_dn5, locals.var_xerfc_dn6, locals.var_xerfc_dn7, locals.var_xerfc_dn8, ) = (assign27110_e31947, ((locals.var_ltat_dn5 * locals.var_ktat) + (assign27110_e31945 * locals.var_ktat_dn5)), ((locals.var_ltat_dn6 * locals.var_ktat) + (assign27110_e31945 * locals.var_ktat_dn6)), ((locals.var_ltat_dn7 * locals.var_ktat) + (assign27110_e31945 * locals.var_ktat_dn7)), ((locals.var_ltat_dn8 * locals.var_ktat) + (assign27110_e31945 * locals.var_ktat_dn8)), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27120_e31961: f64 = (locals.var_xerfc * locals.var_xerfc);
            (locals.var_ysq, locals.var_ysq_dn5, locals.var_ysq_dn6, locals.var_ysq_dn7, locals.var_ysq_dn8, ) = (assign27120_e31961, ((locals.var_xerfc_dn5 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn5)), ((locals.var_xerfc_dn6 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn6)), ((locals.var_xerfc_dn7 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn7)), ((locals.var_xerfc_dn8 * locals.var_xerfc) + (locals.var_xerfc * locals.var_xerfc_dn8)), );
        }
        let assign27130_e31966: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard517 = assign27130_e31966;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) && (locals.var_guard517 != 0.0)) {
            let assign27140_e31982: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign27140_e31983: f64 = (1.0 + assign27140_e31982);
            let assign27140_e31984: f64 = (1.0 / assign27140_e31983);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign27140_e31984, (-((locals.var_perfc * locals.var_xerfc_dn5) / (assign27140_e31983 * assign27140_e31983))), (-((locals.var_perfc * locals.var_xerfc_dn6) / (assign27140_e31983 * assign27140_e31983))), (-((locals.var_perfc * locals.var_xerfc_dn7) / (assign27140_e31983 * assign27140_e31983))), (-((locals.var_perfc * locals.var_xerfc_dn8) / (assign27140_e31983 * assign27140_e31983))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) && (locals.var_guard517 == 0.0)) {
            let assign27150_e32003: f64 = (locals.var_perfc * locals.var_xerfc);
            let assign27150_e32004: f64 = (1.0 - assign27150_e32003);
            let assign27150_e32005: f64 = (1.0 / assign27150_e32004);
            (locals.var_terfc, locals.var_terfc_dn5, locals.var_terfc_dn6, locals.var_terfc_dn7, locals.var_terfc_dn8, ) = (assign27150_e32005, (-((-(locals.var_perfc * locals.var_xerfc_dn5)) / (assign27150_e32004 * assign27150_e32004))), (-((-(locals.var_perfc * locals.var_xerfc_dn6)) / (assign27150_e32004 * assign27150_e32004))), (-((-(locals.var_perfc * locals.var_xerfc_dn7)) / (assign27150_e32004 * assign27150_e32004))), (-((-(locals.var_perfc * locals.var_xerfc_dn8)) / (assign27150_e32004 * assign27150_e32004))), );
        }
        let assign27160_e32009: f64 = (-locals.var_ysq);
        let assign27160_e32011: f64 = (assign27160_e32009 + locals.var_mtat);
        let assign27160_e32013: f64 = (-230.25850929940458);
        let assign27160_e32014: f64 = if assign27160_e32011 > assign27160_e32013 { 1.0 } else { 0.0 };
        locals.var_guard518 = assign27160_e32014;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) && (locals.var_guard518 != 0.0)) {
            let assign27170_e32027: f64 = (-locals.var_ysq);
            let assign27170_e32029: f64 = (assign27170_e32027 + locals.var_mtat);
            let assign27170_e32030: f64 = (assign27170_e32029).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign27170_e32030, (assign27170_e32030 * ((-locals.var_ysq_dn5) + locals.var_mtat_dn5)), (assign27170_e32030 * ((-locals.var_ysq_dn6) + locals.var_mtat_dn6)), (assign27170_e32030 * ((-locals.var_ysq_dn7) + locals.var_mtat_dn7)), (assign27170_e32030 * ((-locals.var_ysq_dn8) + locals.var_mtat_dn8)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) && (locals.var_guard518 == 0.0)) {
            let assign27180_e32048: f64 = (-230.25850929940458);
            let assign27180_e32050: f64 = (-locals.var_ysq);
            let assign27180_e32052: f64 = (assign27180_e32050 + locals.var_mtat);
            let assign27180_e32053: f64 = (assign27180_e32048 - assign27180_e32052);
            let assign27180_e32057: f64 = (-230.25850929940458);
            let assign27180_e32059: f64 = (-locals.var_ysq);
            let assign27180_e32061: f64 = (assign27180_e32059 + locals.var_mtat);
            let assign27180_e32062: f64 = (assign27180_e32057 - assign27180_e32061);
            let assign27180_e32065: f64 = (-230.25850929940458);
            let assign27180_e32067: f64 = (-locals.var_ysq);
            let assign27180_e32069: f64 = (assign27180_e32067 + locals.var_mtat);
            let assign27180_e32070: f64 = (assign27180_e32065 - assign27180_e32069);
            let assign27180_e32072: f64 = (assign27180_e32070 * 0.3333333333333333);
            let assign27180_e32073: f64 = (1.0 + assign27180_e32072);
            let assign27180_e32074: f64 = (assign27180_e32062 * assign27180_e32073);
            let assign27180_e32075: f64 = (0.5 * assign27180_e32074);
            let assign27180_e32076: f64 = (1.0 + assign27180_e32075);
            let assign27180_e32077: f64 = (assign27180_e32053 * assign27180_e32076);
            let assign27180_e32078: f64 = (1.0 + assign27180_e32077);
            let assign27180_e32079: f64 = (1e-100 / assign27180_e32078);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign27180_e32079, (-((1e-100 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign27180_e32076) + (assign27180_e32053 * (0.5 * (((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * assign27180_e32073) + (assign27180_e32062 * ((-((-locals.var_ysq_dn5) + locals.var_mtat_dn5)) * 0.3333333333333333))))))) / (assign27180_e32078 * assign27180_e32078))), (-((1e-100 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign27180_e32076) + (assign27180_e32053 * (0.5 * (((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * assign27180_e32073) + (assign27180_e32062 * ((-((-locals.var_ysq_dn6) + locals.var_mtat_dn6)) * 0.3333333333333333))))))) / (assign27180_e32078 * assign27180_e32078))), (-((1e-100 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign27180_e32076) + (assign27180_e32053 * (0.5 * (((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * assign27180_e32073) + (assign27180_e32062 * ((-((-locals.var_ysq_dn7) + locals.var_mtat_dn7)) * 0.3333333333333333))))))) / (assign27180_e32078 * assign27180_e32078))), (-((1e-100 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign27180_e32076) + (assign27180_e32053 * (0.5 * (((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * assign27180_e32073) + (assign27180_e32062 * ((-((-locals.var_ysq_dn8) + locals.var_mtat_dn8)) * 0.3333333333333333))))))) / (assign27180_e32078 * assign27180_e32078))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27190_e32093: f64 = (0.29214664 * locals.var_terfc);
            let assign27190_e32097: f64 = (locals.var_terfc * locals.var_terfc);
            let assign27190_e32098: f64 = (locals.var_berfc * assign27190_e32097);
            let assign27190_e32099: f64 = (assign27190_e32093 + assign27190_e32098);
            let assign27190_e32103: f64 = (locals.var_terfc * locals.var_terfc);
            let assign27190_e32105: f64 = (assign27190_e32103 * locals.var_terfc);
            let assign27190_e32106: f64 = (locals.var_cerfc * assign27190_e32105);
            let assign27190_e32107: f64 = (assign27190_e32099 + assign27190_e32106);
            let assign27190_e32109: f64 = (assign27190_e32107 * locals.var_tmp);
            (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, ) = (assign27190_e32109, (((((0.29214664 * locals.var_terfc_dn5) + (locals.var_berfc * ((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)))) + (locals.var_cerfc * ((((locals.var_terfc_dn5 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn5)) * locals.var_terfc) + (assign27190_e32103 * locals.var_terfc_dn5)))) * locals.var_tmp) + (assign27190_e32107 * locals.var_tmp_dn5)), (((((0.29214664 * locals.var_terfc_dn6) + (locals.var_berfc * ((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)))) + (locals.var_cerfc * ((((locals.var_terfc_dn6 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn6)) * locals.var_terfc) + (assign27190_e32103 * locals.var_terfc_dn6)))) * locals.var_tmp) + (assign27190_e32107 * locals.var_tmp_dn6)), (((((0.29214664 * locals.var_terfc_dn7) + (locals.var_berfc * ((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)))) + (locals.var_cerfc * ((((locals.var_terfc_dn7 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn7)) * locals.var_terfc) + (assign27190_e32103 * locals.var_terfc_dn7)))) * locals.var_tmp) + (assign27190_e32107 * locals.var_tmp_dn7)), (((((0.29214664 * locals.var_terfc_dn8) + (locals.var_berfc * ((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)))) + (locals.var_cerfc * ((((locals.var_terfc_dn8 * locals.var_terfc) + (locals.var_terfc * locals.var_terfc_dn8)) * locals.var_terfc) + (assign27190_e32103 * locals.var_terfc_dn8)))) * locals.var_tmp) + (assign27190_e32107 * locals.var_tmp_dn8)), );
        }
        let assign27200_e32114: f64 = if locals.var_xerfc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard519 = assign27200_e32114;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) && (locals.var_guard519 != 0.0)) {
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (locals.var_erfcpos, locals.var_erfcpos_dn5, locals.var_erfcpos_dn6, locals.var_erfcpos_dn7, locals.var_erfcpos_dn8, );
        }
        let assign27220_e32131: f64 = (-230.25850929940458);
        let assign27220_e32132: f64 = if locals.var_mtat > assign27220_e32131 { 1.0 } else { 0.0 };
        locals.var_guard520 = assign27220_e32132;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard520 != 0.0)) {
            let assign27230_e32148: f64 = (locals.var_mtat).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign27230_e32148, (assign27230_e32148 * locals.var_mtat_dn5), (assign27230_e32148 * locals.var_mtat_dn6), (assign27230_e32148 * locals.var_mtat_dn7), (assign27230_e32148 * locals.var_mtat_dn8), );
        }
    }
    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard520 == 0.0)) {
            let assign27240_e32169: f64 = (-230.25850929940458);
            let assign27240_e32171: f64 = (assign27240_e32169 - locals.var_mtat);
            let assign27240_e32175: f64 = (-230.25850929940458);
            let assign27240_e32177: f64 = (assign27240_e32175 - locals.var_mtat);
            let assign27240_e32180: f64 = (-230.25850929940458);
            let assign27240_e32182: f64 = (assign27240_e32180 - locals.var_mtat);
            let assign27240_e32184: f64 = (assign27240_e32182 * 0.3333333333333333);
            let assign27240_e32185: f64 = (1.0 + assign27240_e32184);
            let assign27240_e32186: f64 = (assign27240_e32177 * assign27240_e32185);
            let assign27240_e32187: f64 = (0.5 * assign27240_e32186);
            let assign27240_e32188: f64 = (1.0 + assign27240_e32187);
            let assign27240_e32189: f64 = (assign27240_e32171 * assign27240_e32188);
            let assign27240_e32190: f64 = (1.0 + assign27240_e32189);
            let assign27240_e32191: f64 = (1e-100 / assign27240_e32190);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign27240_e32191, (-((1e-100 * (((-locals.var_mtat_dn5) * assign27240_e32188) + (assign27240_e32171 * (0.5 * (((-locals.var_mtat_dn5) * assign27240_e32185) + (assign27240_e32177 * ((-locals.var_mtat_dn5) * 0.3333333333333333))))))) / (assign27240_e32190 * assign27240_e32190))), (-((1e-100 * (((-locals.var_mtat_dn6) * assign27240_e32188) + (assign27240_e32171 * (0.5 * (((-locals.var_mtat_dn6) * assign27240_e32185) + (assign27240_e32177 * ((-locals.var_mtat_dn6) * 0.3333333333333333))))))) / (assign27240_e32190 * assign27240_e32190))), (-((1e-100 * (((-locals.var_mtat_dn7) * assign27240_e32188) + (assign27240_e32171 * (0.5 * (((-locals.var_mtat_dn7) * assign27240_e32185) + (assign27240_e32177 * ((-locals.var_mtat_dn7) * 0.3333333333333333))))))) / (assign27240_e32190 * assign27240_e32190))), (-((1e-100 * (((-locals.var_mtat_dn8) * assign27240_e32188) + (assign27240_e32171 * (0.5 * (((-locals.var_mtat_dn8) * assign27240_e32185) + (assign27240_e32177 * ((-locals.var_mtat_dn8) * 0.3333333333333333))))))) / (assign27240_e32190 * assign27240_e32190))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) && (locals.var_guard519 == 0.0)) {
            let assign27250_e32208: f64 = (2.0 * locals.var_tmp);
            let assign27250_e32210: f64 = (assign27250_e32208 - locals.var_erfcpos);
            (locals.var_erfctimesexpmtat, locals.var_erfctimesexpmtat_dn5, locals.var_erfctimesexpmtat_dn6, locals.var_erfctimesexpmtat_dn7, locals.var_erfctimesexpmtat_dn8, ) = (assign27250_e32210, ((2.0 * locals.var_tmp_dn5) - locals.var_erfcpos_dn5), ((2.0 * locals.var_tmp_dn6) - locals.var_erfcpos_dn6), ((2.0 * locals.var_tmp_dn7) - locals.var_erfcpos_dn7), ((2.0 * locals.var_tmp_dn8) - locals.var_erfcpos_dn8), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27260_e32224: f64 = (1.772453850905516 * 0.5);
            let assign27260_e32227: f64 = (locals.var_atatgat * locals.var_erfctimesexpmtat);
            let assign27260_e32229: f64 = (assign27260_e32227 / locals.var_ktat);
            let assign27260_e32230: f64 = (assign27260_e32224 * assign27260_e32229);
            (locals.var_gammamax, locals.var_gammamax_dn5, locals.var_gammamax_dn6, locals.var_gammamax_dn7, locals.var_gammamax_dn8, ) = (assign27260_e32230, (assign27260_e32224 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn5) * locals.var_ktat) - (assign27260_e32227 * locals.var_ktat_dn5)) / (locals.var_ktat * locals.var_ktat))), (assign27260_e32224 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn6) * locals.var_ktat) - (assign27260_e32227 * locals.var_ktat_dn6)) / (locals.var_ktat * locals.var_ktat))), (assign27260_e32224 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn7) * locals.var_ktat) - (assign27260_e32227 * locals.var_ktat_dn7)) / (locals.var_ktat * locals.var_ktat))), (assign27260_e32224 * ((((locals.var_atatgat * locals.var_erfctimesexpmtat_dn8) * locals.var_ktat) - (assign27260_e32227 * locals.var_ktat_dn8)) / (locals.var_ktat * locals.var_ktat))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard515 == 0.0)) {
            let assign27270_e32245: f64 = (locals.var_asrh * locals.var_gammamax);
            let assign27270_e32247: f64 = (assign27270_e32245 * locals.var_wtat);
            let assign27270_e32248: f64 = (p.p847 * assign27270_e32247);
            (locals.var_itat, locals.var_itat_dn5, locals.var_itat_dn6, locals.var_itat_dn7, locals.var_itat_dn8, ) = (assign27270_e32248, (p.p847 * ((((locals.var_asrh_dn5 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn5)) * locals.var_wtat) + (assign27270_e32245 * locals.var_wtat_dn5))), (p.p847 * ((((locals.var_asrh_dn6 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn6)) * locals.var_wtat) + (assign27270_e32245 * locals.var_wtat_dn6))), (p.p847 * ((((locals.var_asrh_dn7 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn7)) * locals.var_wtat) + (assign27270_e32245 * locals.var_wtat_dn7))), (p.p847 * ((((locals.var_asrh_dn8 * locals.var_gammamax) + (locals.var_asrh * locals.var_gammamax_dn8)) * locals.var_wtat) + (assign27270_e32245 * locals.var_wtat_dn8))), );
        }
        let assign27280_e32253: f64 = if p.p853 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard521 = assign27280_e32253;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard521 != 0.0)) {
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign27300_e32267: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard522 = assign27300_e32267;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard522 != 0.0)) {
            let assign27310_e32281: f64 = (p.p830 - locals.var_vbbt);
            let assign27310_e32283: f64 = (assign27310_e32281 * locals.var_vbirgatinv);
            let assign27310_e32284: f64 = (assign27310_e32283).sqrt();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign27310_e32284, 0.0, 0.0, 0.0, 0.0, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard522 == 0.0)) {
            let assign27320_e32301: f64 = (p.p830 - locals.var_vbbt);
            let assign27320_e32303: f64 = (assign27320_e32301 * locals.var_vbirgatinv);
            let assign27320_e32305: f64 = (assign27320_e32303).powf(p.p833);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign27320_e32305, 0.0, 0.0, 0.0, 0.0, );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard521 == 0.0)) {
            let assign27330_e32320: f64 = (p.p830 - locals.var_vbbt);
            let assign27330_e32322: f64 = (assign27330_e32320 * locals.var_wdepnulrinvgat);
            let assign27330_e32324: f64 = (assign27330_e32322 / locals.var_tmp);
            let assign27330_e32325: f64 = (locals.var_one_over_one_minus_pgat * assign27330_e32324);
            (locals.var_fmaxr, locals.var_fmaxr_dn5, locals.var_fmaxr_dn6, locals.var_fmaxr_dn7, locals.var_fmaxr_dn8, ) = (assign27330_e32325, (locals.var_one_over_one_minus_pgat * (-((assign27330_e32322 * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign27330_e32322 * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign27330_e32322 * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp)))), (locals.var_one_over_one_minus_pgat * (-((assign27330_e32322 * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp)))), );
        }
        let assign27340_e32329: f64 = (-locals.var_fbbtgat);
        let assign27340_e32331: f64 = (assign27340_e32329 / locals.var_fmaxr);
        let assign27340_e32332: f64 = (assign27340_e32331).abs();
        let assign27340_e32334: f64 = if assign27340_e32332 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard523 = assign27340_e32334;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard523 != 0.0)) {
            let assign27350_e32347: f64 = (-locals.var_fbbtgat);
            let assign27350_e32349: f64 = (assign27350_e32347 / locals.var_fmaxr);
            let assign27350_e32350: f64 = (assign27350_e32349).exp();
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign27350_e32350, (assign27350_e32350 * ((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign27350_e32347 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign27350_e32350 * ((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign27350_e32347 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign27350_e32350 * ((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign27350_e32347 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))), (assign27350_e32350 * ((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign27350_e32347 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))), );
        }
        let assign27360_e32354: f64 = (-locals.var_fbbtgat);
        let assign27360_e32356: f64 = (assign27360_e32354 / locals.var_fmaxr);
        let assign27360_e32358: f64 = if assign27360_e32356 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard524 = assign27360_e32358;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard523 == 0.0)) && (locals.var_guard524 != 0.0)) {
            let assign27370_e32376: f64 = (-230.25850929940458);
            let assign27370_e32378: f64 = (-locals.var_fbbtgat);
            let assign27370_e32380: f64 = (assign27370_e32378 / locals.var_fmaxr);
            let assign27370_e32381: f64 = (assign27370_e32376 - assign27370_e32380);
            let assign27370_e32385: f64 = (-230.25850929940458);
            let assign27370_e32387: f64 = (-locals.var_fbbtgat);
            let assign27370_e32389: f64 = (assign27370_e32387 / locals.var_fmaxr);
            let assign27370_e32390: f64 = (assign27370_e32385 - assign27370_e32389);
            let assign27370_e32393: f64 = (-230.25850929940458);
            let assign27370_e32395: f64 = (-locals.var_fbbtgat);
            let assign27370_e32397: f64 = (assign27370_e32395 / locals.var_fmaxr);
            let assign27370_e32398: f64 = (assign27370_e32393 - assign27370_e32397);
            let assign27370_e32400: f64 = (assign27370_e32398 * 0.3333333333333333);
            let assign27370_e32401: f64 = (1.0 + assign27370_e32400);
            let assign27370_e32402: f64 = (assign27370_e32390 * assign27370_e32401);
            let assign27370_e32403: f64 = (0.5 * assign27370_e32402);
            let assign27370_e32404: f64 = (1.0 + assign27370_e32403);
            let assign27370_e32405: f64 = (assign27370_e32381 * assign27370_e32404);
            let assign27370_e32406: f64 = (1.0 + assign27370_e32405);
            let assign27370_e32407: f64 = (1e-100 / assign27370_e32406);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign27370_e32407, (-((1e-100 * (((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign27370_e32378 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign27370_e32404) + (assign27370_e32381 * (0.5 * (((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign27370_e32387 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign27370_e32401) + (assign27370_e32390 * ((-((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign27370_e32395 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign27370_e32406 * assign27370_e32406))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign27370_e32378 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign27370_e32404) + (assign27370_e32381 * (0.5 * (((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign27370_e32387 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign27370_e32401) + (assign27370_e32390 * ((-((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign27370_e32395 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign27370_e32406 * assign27370_e32406))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign27370_e32378 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign27370_e32404) + (assign27370_e32381 * (0.5 * (((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign27370_e32387 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign27370_e32401) + (assign27370_e32390 * ((-((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign27370_e32395 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign27370_e32406 * assign27370_e32406))), (-((1e-100 * (((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign27370_e32378 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign27370_e32404) + (assign27370_e32381 * (0.5 * (((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign27370_e32387 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * assign27370_e32401) + (assign27370_e32390 * ((-((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign27370_e32395 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr))) * 0.3333333333333333))))))) / (assign27370_e32406 * assign27370_e32406))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard523 == 0.0)) && (locals.var_guard524 == 0.0)) {
            let assign27380_e32428: f64 = (-locals.var_fbbtgat);
            let assign27380_e32430: f64 = (assign27380_e32428 / locals.var_fmaxr);
            let assign27380_e32432: f64 = (assign27380_e32430 - 230.25850929940458);
            let assign27380_e32436: f64 = (-locals.var_fbbtgat);
            let assign27380_e32438: f64 = (assign27380_e32436 / locals.var_fmaxr);
            let assign27380_e32440: f64 = (assign27380_e32438 - 230.25850929940458);
            let assign27380_e32443: f64 = (-locals.var_fbbtgat);
            let assign27380_e32445: f64 = (assign27380_e32443 / locals.var_fmaxr);
            let assign27380_e32447: f64 = (assign27380_e32445 - 230.25850929940458);
            let assign27380_e32449: f64 = (assign27380_e32447 * 0.3333333333333333);
            let assign27380_e32450: f64 = (1.0 + assign27380_e32449);
            let assign27380_e32451: f64 = (assign27380_e32440 * assign27380_e32450);
            let assign27380_e32452: f64 = (0.5 * assign27380_e32451);
            let assign27380_e32453: f64 = (1.0 + assign27380_e32452);
            let assign27380_e32454: f64 = (assign27380_e32432 * assign27380_e32453);
            let assign27380_e32455: f64 = (1.0 + assign27380_e32454);
            let assign27380_e32456: f64 = (1e100 * assign27380_e32455);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign27380_e32456, (1e100 * ((((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign27380_e32428 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign27380_e32453) + (assign27380_e32432 * (0.5 * ((((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign27380_e32436 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign27380_e32450) + (assign27380_e32440 * (((((-locals.var_fbbtgat_dn5) * locals.var_fmaxr) - (assign27380_e32443 * locals.var_fmaxr_dn5)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign27380_e32428 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign27380_e32453) + (assign27380_e32432 * (0.5 * ((((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign27380_e32436 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign27380_e32450) + (assign27380_e32440 * (((((-locals.var_fbbtgat_dn6) * locals.var_fmaxr) - (assign27380_e32443 * locals.var_fmaxr_dn6)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign27380_e32428 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign27380_e32453) + (assign27380_e32432 * (0.5 * ((((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign27380_e32436 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign27380_e32450) + (assign27380_e32440 * (((((-locals.var_fbbtgat_dn7) * locals.var_fmaxr) - (assign27380_e32443 * locals.var_fmaxr_dn7)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign27380_e32428 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign27380_e32453) + (assign27380_e32432 * (0.5 * ((((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign27380_e32436 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * assign27380_e32450) + (assign27380_e32440 * (((((-locals.var_fbbtgat_dn8) * locals.var_fmaxr) - (assign27380_e32443 * locals.var_fmaxr_dn8)) / (locals.var_fmaxr * locals.var_fmaxr)) * 0.3333333333333333))))))), );
        }
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard521 == 0.0)) {
            let assign27390_e32471: f64 = (locals.var_v5 * locals.var_fmaxr);
            let assign27390_e32473: f64 = (assign27390_e32471 * locals.var_fmaxr);
            let assign27390_e32475: f64 = (assign27390_e32473 * locals.var_tmp);
            let assign27390_e32476: f64 = (p.p853 * assign27390_e32475);
            (locals.var_ibbt, locals.var_ibbt_dn5, locals.var_ibbt_dn6, locals.var_ibbt_dn7, locals.var_ibbt_dn8, ) = (assign27390_e32476, (p.p853 * (((((locals.var_v5 * locals.var_fmaxr_dn5) * locals.var_fmaxr) + (assign27390_e32471 * locals.var_fmaxr_dn5)) * locals.var_tmp) + (assign27390_e32473 * locals.var_tmp_dn5))), (p.p853 * (((((locals.var_v5 * locals.var_fmaxr_dn6) * locals.var_fmaxr) + (assign27390_e32471 * locals.var_fmaxr_dn6)) * locals.var_tmp) + (assign27390_e32473 * locals.var_tmp_dn6))), (p.p853 * (((((locals.var_v5 * locals.var_fmaxr_dn7) * locals.var_fmaxr) + (assign27390_e32471 * locals.var_fmaxr_dn7)) * locals.var_tmp) + (assign27390_e32473 * locals.var_tmp_dn7))), (p.p853 * (((((locals.var_v5 * locals.var_fmaxr_dn8) * locals.var_fmaxr) + (assign27390_e32471 * locals.var_fmaxr_dn8)) * locals.var_tmp) + (assign27390_e32473 * locals.var_tmp_dn8))), );
        }
        let assign27400_e32481: f64 = if p.p862 > 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard525 = assign27400_e32481;
        if ((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard525 != 0.0)) {
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }
        let assign27420_e32495: f64 = (-locals.var_alphaav);
        let assign27420_e32497: f64 = (assign27420_e32495 * p.p862);
        let assign27420_e32498: f64 = if locals.var_vav > assign27420_e32497 { 1.0 } else { 0.0 };
        locals.var_guard526 = assign27420_e32498;
        let assign27430_e32501: f64 = if p.p865 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard527 = assign27430_e32501;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard525 == 0.0)) && (locals.var_guard526 != 0.0)) && (locals.var_guard527 != 0.0)) {
            let assign27440_e32517: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign27440_e32520: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign27440_e32521: f64 = (assign27440_e32517 * assign27440_e32520);
            let assign27440_e32524: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign27440_e32525: f64 = (assign27440_e32521 * assign27440_e32524);
            let assign27440_e32528: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign27440_e32529: f64 = (assign27440_e32525 * assign27440_e32528);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign27440_e32529, (((((((locals.var_vav * locals.var_vbrinvgat_dn5) * assign27440_e32520) + (assign27440_e32517 * (locals.var_vav * locals.var_vbrinvgat_dn5))) * assign27440_e32524) + (assign27440_e32521 * (locals.var_vav * locals.var_vbrinvgat_dn5))) * assign27440_e32528) + (assign27440_e32525 * (locals.var_vav * locals.var_vbrinvgat_dn5))), (((((((locals.var_vav * locals.var_vbrinvgat_dn6) * assign27440_e32520) + (assign27440_e32517 * (locals.var_vav * locals.var_vbrinvgat_dn6))) * assign27440_e32524) + (assign27440_e32521 * (locals.var_vav * locals.var_vbrinvgat_dn6))) * assign27440_e32528) + (assign27440_e32525 * (locals.var_vav * locals.var_vbrinvgat_dn6))), (((((((locals.var_vav * locals.var_vbrinvgat_dn7) * assign27440_e32520) + (assign27440_e32517 * (locals.var_vav * locals.var_vbrinvgat_dn7))) * assign27440_e32524) + (assign27440_e32521 * (locals.var_vav * locals.var_vbrinvgat_dn7))) * assign27440_e32528) + (assign27440_e32525 * (locals.var_vav * locals.var_vbrinvgat_dn7))), (((((((locals.var_vav * locals.var_vbrinvgat_dn8) * assign27440_e32520) + (assign27440_e32517 * (locals.var_vav * locals.var_vbrinvgat_dn8))) * assign27440_e32524) + (assign27440_e32521 * (locals.var_vav * locals.var_vbrinvgat_dn8))) * assign27440_e32528) + (assign27440_e32525 * (locals.var_vav * locals.var_vbrinvgat_dn8))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard525 == 0.0)) && (locals.var_guard526 != 0.0)) && (locals.var_guard527 == 0.0)) {
            let assign27450_e32548: f64 = (locals.var_vav * locals.var_vbrinvgat);
            let assign27450_e32549: f64 = (assign27450_e32548).abs();
            let assign27450_e32551: f64 = (assign27450_e32549).powf(p.p865);
            (locals.var_tmp, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, ) = (assign27450_e32551, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign27450_e32549).powf(p.p865 - 1.0) * if assign27450_e32548 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn5)) })) } } else { (assign27450_e32551 * (p.p865 * (if assign27450_e32548 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn5) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn5)) } / assign27450_e32549))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign27450_e32549).powf(p.p865 - 1.0) * if assign27450_e32548 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn6)) })) } } else { (assign27450_e32551 * (p.p865 * (if assign27450_e32548 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn6) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn6)) } / assign27450_e32549))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign27450_e32549).powf(p.p865 - 1.0) * if assign27450_e32548 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn7)) })) } } else { (assign27450_e32551 * (p.p865 * (if assign27450_e32548 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn7) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn7)) } / assign27450_e32549))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign27450_e32549).powf(p.p865 - 1.0) * if assign27450_e32548 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn8)) })) } } else { (assign27450_e32551 * (p.p865 * (if assign27450_e32548 >= 0.0 { (locals.var_vav * locals.var_vbrinvgat_dn8) } else { (-(locals.var_vav * locals.var_vbrinvgat_dn8)) } / assign27450_e32549))) }, );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard525 == 0.0)) && (locals.var_guard526 != 0.0)) {
            let assign27460_e32568: f64 = (1.0 - locals.var_tmp);
            let assign27460_e32569: f64 = (1.0 / assign27460_e32568);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign27460_e32569, (-((-locals.var_tmp_dn5) / (assign27460_e32568 * assign27460_e32568))), (-((-locals.var_tmp_dn6) / (assign27460_e32568 * assign27460_e32568))), (-((-locals.var_tmp_dn7) / (assign27460_e32568 * assign27460_e32568))), (-((-locals.var_tmp_dn8) / (assign27460_e32568 * assign27460_e32568))), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) && (locals.var_guard525 == 0.0)) && (locals.var_guard526 == 0.0)) {
            let assign27470_e32588: f64 = (locals.var_alphaav * p.p862);
            let assign27470_e32589: f64 = (locals.var_vav + assign27470_e32588);
            let assign27470_e32591: f64 = (assign27470_e32589 * locals.var_slopegat);
            let assign27470_e32592: f64 = (locals.var_fstopgat + assign27470_e32591);
            (locals.var_fbreakdown, locals.var_fbreakdown_dn5, locals.var_fbreakdown_dn6, locals.var_fbreakdown_dn7, locals.var_fbreakdown_dn8, ) = (assign27470_e32592, (assign27470_e32589 * locals.var_slopegat_dn5), (assign27470_e32589 * locals.var_slopegat_dn6), (assign27470_e32589 * locals.var_slopegat_dn7), (assign27470_e32589 * locals.var_slopegat_dn8), );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard511 == 0.0)) {
            let assign27480_e32604: f64 = (locals.var_id__blk219 + locals.var_isrh);
            let assign27480_e32606: f64 = (assign27480_e32604 + locals.var_itat);
            let assign27480_e32608: f64 = (assign27480_e32606 + locals.var_ibbt);
            let assign27480_e32609: f64 = (p.p29 * assign27480_e32608);
            let assign27480_e32611: f64 = (assign27480_e32609 * locals.var_fbreakdown);
            (locals.var_ijungat, locals.var_ijungat_dn5, locals.var_ijungat_dn6, locals.var_ijungat_dn7, locals.var_ijungat_dn8, ) = (assign27480_e32611, (((p.p29 * ((locals.var_isrh_dn5 + locals.var_itat_dn5) + locals.var_ibbt_dn5)) * locals.var_fbreakdown) + (assign27480_e32609 * locals.var_fbreakdown_dn5)), (((p.p29 * ((locals.var_isrh_dn6 + locals.var_itat_dn6) + locals.var_ibbt_dn6)) * locals.var_fbreakdown) + (assign27480_e32609 * locals.var_fbreakdown_dn6)), (((p.p29 * ((locals.var_isrh_dn7 + locals.var_itat_dn7) + locals.var_ibbt_dn7)) * locals.var_fbreakdown) + (assign27480_e32609 * locals.var_fbreakdown_dn7)), (((p.p29 * ((locals.var_isrh_dn8 + locals.var_itat_dn8) + locals.var_ibbt_dn8)) * locals.var_fbreakdown) + (assign27480_e32609 * locals.var_fbreakdown_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27490_e32619: f64 = (locals.var_absource_i * locals.var_ijunbot);
            let assign27490_e32622: f64 = (locals.var_lssource_i * locals.var_ijunsti);
            let assign27490_e32623: f64 = (assign27490_e32619 + assign27490_e32622);
            let assign27490_e32626: f64 = (locals.var_lgsource_i * locals.var_ijungat);
            let assign27490_e32627: f64 = (assign27490_e32623 + assign27490_e32626);
            (locals.var_i5, locals.var_i5_dn5, locals.var_i5_dn6, locals.var_i5_dn7, locals.var_i5_dn8, ) = (assign27490_e32627, (((locals.var_absource_i * locals.var_ijunbot_dn5) + (locals.var_lssource_i * locals.var_ijunsti_dn5)) + (locals.var_lgsource_i * locals.var_ijungat_dn5)), (((locals.var_absource_i * locals.var_ijunbot_dn6) + (locals.var_lssource_i * locals.var_ijunsti_dn6)) + (locals.var_lgsource_i * locals.var_ijungat_dn6)), (((locals.var_absource_i * locals.var_ijunbot_dn7) + (locals.var_lssource_i * locals.var_ijunsti_dn7)) + (locals.var_lgsource_i * locals.var_ijungat_dn7)), (((locals.var_absource_i * locals.var_ijunbot_dn8) + (locals.var_lssource_i * locals.var_ijunsti_dn8)) + (locals.var_lgsource_i * locals.var_ijungat_dn8)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27500_e32635: f64 = (locals.var_absource_i * locals.var_idsatbot);
            let assign27500_e32638: f64 = (locals.var_lssource_i * locals.var_idsatsti);
            let assign27500_e32639: f64 = (assign27500_e32635 + assign27500_e32638);
            let assign27500_e32642: f64 = (locals.var_lgsource_i * locals.var_idsatgat);
            let assign27500_e32643: f64 = (assign27500_e32639 + assign27500_e32642);
            locals.var_isatfor1_s = assign27500_e32643;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27510_e32653: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign27510_e32655: f64 = (assign27510_e32653 * locals.var_mfor1_s);
            let assign27510_e32656: f64 = (assign27510_e32655).exp();
            let assign27510_e32658: f64 = (assign27510_e32656 - 1.0);
            let assign27510_e32659: f64 = (locals.var_isatfor1_s * assign27510_e32658);
            let assign27510_e32660: f64 = (locals.var_i4 - assign27510_e32659);
            (locals.var_i4_cor, locals.var_i4_cor_dn5, locals.var_i4_cor_dn6, locals.var_i4_cor_dn7, locals.var_i4_cor_dn8, ) = (assign27510_e32660, locals.var_i4_dn5, locals.var_i4_dn6, locals.var_i4_dn7, locals.var_i4_dn8, );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27520_e32670: f64 = (locals.var_v5 * locals.var_phitdinv);
            let assign27520_e32672: f64 = (assign27520_e32670 * locals.var_mfor1_s);
            let assign27520_e32673: f64 = (assign27520_e32672).exp();
            let assign27520_e32675: f64 = (assign27520_e32673 - 1.0);
            let assign27520_e32676: f64 = (locals.var_isatfor1_s * assign27520_e32675);
            let assign27520_e32677: f64 = (locals.var_i5 - assign27520_e32676);
            (locals.var_i5_cor, locals.var_i5_cor_dn5, locals.var_i5_cor_dn6, locals.var_i5_cor_dn7, locals.var_i5_cor_dn8, ) = (assign27520_e32677, locals.var_i5_dn5, locals.var_i5_dn6, locals.var_i5_dn7, locals.var_i5_dn8, );
        }
        let assign27530_e32691: f64 = if (!(((locals.var_absource_i == 0.0) && (locals.var_lssource_i == 0.0)) && (locals.var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard528 = assign27530_e32691;
        let assign27540_e32698: f64 = if ((locals.var_i4 > 0.0) && (locals.var_i5 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard529 = assign27540_e32698;
        let assign27550_e32701: f64 = (locals.var_i4_cor / locals.var_i4);
        let assign27550_e32706: f64 = (locals.var_i5_cor / locals.var_i5);
        let assign27550_e32721: f64 = if (((((assign27550_e32701 > 0.001) || (assign27550_e32706 > 0.001)) && (locals.var_i4_cor > 0.0)) && (locals.var_i5_cor > 0.0)) && (locals.var_i5_cor > locals.var_i4_cor)) { 1.0 } else { 0.0 };
        locals.var_guard530 = assign27550_e32721;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard529 != 0.0)) && (locals.var_guard530 != 0.0)) {
            let assign27560_e32733: f64 = (locals.var_i4_cor / locals.var_i5_cor);
            (locals.var_alphaje, locals.var_alphaje_dn5, locals.var_alphaje_dn6, locals.var_alphaje_dn7, locals.var_alphaje_dn8, ) = (assign27560_e32733, (((locals.var_i4_cor_dn5 * locals.var_i5_cor) - (locals.var_i4_cor * locals.var_i5_cor_dn5)) / (locals.var_i5_cor * locals.var_i5_cor)), (((locals.var_i4_cor_dn6 * locals.var_i5_cor) - (locals.var_i4_cor * locals.var_i5_cor_dn6)) / (locals.var_i5_cor * locals.var_i5_cor)), (((locals.var_i4_cor_dn7 * locals.var_i5_cor) - (locals.var_i4_cor * locals.var_i5_cor_dn7)) / (locals.var_i5_cor * locals.var_i5_cor)), (((locals.var_i4_cor_dn8 * locals.var_i5_cor) - (locals.var_i4_cor * locals.var_i5_cor_dn8)) / (locals.var_i5_cor * locals.var_i5_cor)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard529 != 0.0)) && (locals.var_guard530 != 0.0)) {
            let assign27570_e32747: f64 = (locals.var_alphaje).ln();
            let assign27570_e32748: f64 = (locals.var_phitd * assign27570_e32747);
            let assign27570_e32751: f64 = (locals.var_v4 - locals.var_v5);
            let assign27570_e32752: f64 = (assign27570_e32748 / assign27570_e32751);
            (locals.var_mfor2_s, locals.var_mfor2_s_dn5, locals.var_mfor2_s_dn6, locals.var_mfor2_s_dn7, locals.var_mfor2_s_dn8, ) = (assign27570_e32752, ((locals.var_phitd * (locals.var_alphaje_dn5 / locals.var_alphaje)) / assign27570_e32751), ((locals.var_phitd * (locals.var_alphaje_dn6 / locals.var_alphaje)) / assign27570_e32751), ((locals.var_phitd * (locals.var_alphaje_dn7 / locals.var_alphaje)) / assign27570_e32751), ((locals.var_phitd * (locals.var_alphaje_dn8 / locals.var_alphaje)) / assign27570_e32751), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard529 != 0.0)) && (locals.var_guard530 != 0.0)) {
            let assign27580_e32767: f64 = (locals.var_v4 * locals.var_phitdinv);
            let assign27580_e32769: f64 = (assign27580_e32767 * locals.var_mfor2_s);
            let assign27580_e32770: f64 = (assign27580_e32769).exp();
            let assign27580_e32772: f64 = (assign27580_e32770 - 1.0);
            let assign27580_e32773: f64 = (locals.var_i4_cor / assign27580_e32772);
            (locals.var_isatfor2_s, locals.var_isatfor2_s_dn5, locals.var_isatfor2_s_dn6, locals.var_isatfor2_s_dn7, locals.var_isatfor2_s_dn8, ) = (assign27580_e32773, (((locals.var_i4_cor_dn5 * assign27580_e32772) - (locals.var_i4_cor * (assign27580_e32770 * (assign27580_e32767 * locals.var_mfor2_s_dn5)))) / (assign27580_e32772 * assign27580_e32772)), (((locals.var_i4_cor_dn6 * assign27580_e32772) - (locals.var_i4_cor * (assign27580_e32770 * (assign27580_e32767 * locals.var_mfor2_s_dn6)))) / (assign27580_e32772 * assign27580_e32772)), (((locals.var_i4_cor_dn7 * assign27580_e32772) - (locals.var_i4_cor * (assign27580_e32770 * (assign27580_e32767 * locals.var_mfor2_s_dn7)))) / (assign27580_e32772 * assign27580_e32772)), (((locals.var_i4_cor_dn8 * assign27580_e32772) - (locals.var_i4_cor * (assign27580_e32770 * (assign27580_e32767 * locals.var_mfor2_s_dn8)))) / (assign27580_e32772 * assign27580_e32772)), );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) {
            let assign27590_e32785: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign27590_e32787: f64 = (assign27590_e32785 * locals.var_mfor1_s);
            let assign27590_e32788: f64 = (assign27590_e32787).exp();
            let assign27590_e32790: f64 = (assign27590_e32788 - 1.0);
            let assign27590_e32791: f64 = (locals.var_isatfor1_s * assign27590_e32790);
            let assign27590_e32792: f64 = (locals.var_i1 - assign27590_e32791);
            let assign27590_e32796: f64 = (locals.var_v1 * locals.var_phitdinv);
            let assign27590_e32798: f64 = (assign27590_e32796 * locals.var_mfor2_s);
            let assign27590_e32799: f64 = (assign27590_e32798).exp();
            let assign27590_e32801: f64 = (assign27590_e32799 - 1.0);
            let assign27590_e32802: f64 = (locals.var_isatfor2_s * assign27590_e32801);
            let assign27590_e32803: f64 = (assign27590_e32792 - assign27590_e32802);
            (locals.var_i1_cor, locals.var_i1_cor_dn5, locals.var_i1_cor_dn6, locals.var_i1_cor_dn7, locals.var_i1_cor_dn8, ) = (assign27590_e32803, (locals.var_i1_dn5 - ((locals.var_isatfor2_s_dn5 * assign27590_e32801) + (locals.var_isatfor2_s * (assign27590_e32799 * (assign27590_e32796 * locals.var_mfor2_s_dn5))))), (locals.var_i1_dn6 - ((locals.var_isatfor2_s_dn6 * assign27590_e32801) + (locals.var_isatfor2_s * (assign27590_e32799 * (assign27590_e32796 * locals.var_mfor2_s_dn6))))), (locals.var_i1_dn7 - ((locals.var_isatfor2_s_dn7 * assign27590_e32801) + (locals.var_isatfor2_s * (assign27590_e32799 * (assign27590_e32796 * locals.var_mfor2_s_dn7))))), (locals.var_i1_dn8 - ((locals.var_isatfor2_s_dn8 * assign27590_e32801) + (locals.var_isatfor2_s * (assign27590_e32799 * (assign27590_e32796 * locals.var_mfor2_s_dn8))))), );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) {
            let assign27600_e32815: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign27600_e32817: f64 = (assign27600_e32815 * locals.var_mfor1_s);
            let assign27600_e32818: f64 = (assign27600_e32817).exp();
            let assign27600_e32820: f64 = (assign27600_e32818 - 1.0);
            let assign27600_e32821: f64 = (locals.var_isatfor1_s * assign27600_e32820);
            let assign27600_e32822: f64 = (locals.var_i2 - assign27600_e32821);
            let assign27600_e32826: f64 = (locals.var_v2 * locals.var_phitdinv);
            let assign27600_e32828: f64 = (assign27600_e32826 * locals.var_mfor2_s);
            let assign27600_e32829: f64 = (assign27600_e32828).exp();
            let assign27600_e32831: f64 = (assign27600_e32829 - 1.0);
            let assign27600_e32832: f64 = (locals.var_isatfor2_s * assign27600_e32831);
            let assign27600_e32833: f64 = (assign27600_e32822 - assign27600_e32832);
            (locals.var_i2_cor, locals.var_i2_cor_dn5, locals.var_i2_cor_dn6, locals.var_i2_cor_dn7, locals.var_i2_cor_dn8, ) = (assign27600_e32833, (locals.var_i2_dn5 - ((locals.var_isatfor2_s_dn5 * assign27600_e32831) + (locals.var_isatfor2_s * (assign27600_e32829 * (assign27600_e32826 * locals.var_mfor2_s_dn5))))), (locals.var_i2_dn6 - ((locals.var_isatfor2_s_dn6 * assign27600_e32831) + (locals.var_isatfor2_s * (assign27600_e32829 * (assign27600_e32826 * locals.var_mfor2_s_dn6))))), (locals.var_i2_dn7 - ((locals.var_isatfor2_s_dn7 * assign27600_e32831) + (locals.var_isatfor2_s * (assign27600_e32829 * (assign27600_e32826 * locals.var_mfor2_s_dn7))))), (locals.var_i2_dn8 - ((locals.var_isatfor2_s_dn8 * assign27600_e32831) + (locals.var_isatfor2_s * (assign27600_e32829 * (assign27600_e32826 * locals.var_mfor2_s_dn8))))), );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) {
            let assign27610_e32845: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign27610_e32847: f64 = (assign27610_e32845 * locals.var_mfor1_s);
            let assign27610_e32848: f64 = (assign27610_e32847).exp();
            let assign27610_e32850: f64 = (assign27610_e32848 - 1.0);
            let assign27610_e32851: f64 = (locals.var_isatfor1_s * assign27610_e32850);
            let assign27610_e32852: f64 = (locals.var_i3 - assign27610_e32851);
            let assign27610_e32856: f64 = (locals.var_v3 * locals.var_phitdinv);
            let assign27610_e32858: f64 = (assign27610_e32856 * locals.var_mfor2_s);
            let assign27610_e32859: f64 = (assign27610_e32858).exp();
            let assign27610_e32861: f64 = (assign27610_e32859 - 1.0);
            let assign27610_e32862: f64 = (locals.var_isatfor2_s * assign27610_e32861);
            let assign27610_e32863: f64 = (assign27610_e32852 - assign27610_e32862);
            (locals.var_i3_cor, locals.var_i3_cor_dn5, locals.var_i3_cor_dn6, locals.var_i3_cor_dn7, locals.var_i3_cor_dn8, ) = (assign27610_e32863, (locals.var_i3_dn5 - ((locals.var_isatfor2_s_dn5 * assign27610_e32861) + (locals.var_isatfor2_s * (assign27610_e32859 * (assign27610_e32856 * locals.var_mfor2_s_dn5))))), (locals.var_i3_dn6 - ((locals.var_isatfor2_s_dn6 * assign27610_e32861) + (locals.var_isatfor2_s * (assign27610_e32859 * (assign27610_e32856 * locals.var_mfor2_s_dn6))))), (locals.var_i3_dn7 - ((locals.var_isatfor2_s_dn7 * assign27610_e32861) + (locals.var_isatfor2_s * (assign27610_e32859 * (assign27610_e32856 * locals.var_mfor2_s_dn7))))), (locals.var_i3_dn8 - ((locals.var_isatfor2_s_dn8 * assign27610_e32861) + (locals.var_isatfor2_s * (assign27610_e32859 * (assign27610_e32856 * locals.var_mfor2_s_dn8))))), );
        }
        let assign27620_e32876: f64 = if (((locals.var_i1 < 0.0) && (locals.var_i2 < 0.0)) && (locals.var_i3 < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard531 = assign27620_e32876;
        let assign27630_e32879: f64 = (locals.var_i1_cor / locals.var_i1);
        let assign27630_e32884: f64 = (locals.var_i2_cor / locals.var_i2);
        let assign27630_e32890: f64 = (locals.var_i3_cor / locals.var_i3);
        let assign27630_e32905: f64 = if ((((((assign27630_e32879 > 0.001) || (assign27630_e32884 > 0.001)) || (assign27630_e32890 > 0.001)) && (locals.var_i1_cor < 0.0)) && (locals.var_i2_cor < 0.0)) && (locals.var_i3_cor < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard532 = assign27630_e32905;
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) {
            let assign27640_e32917: f64 = (locals.var_i1_cor / locals.var_i2_cor);
            (locals.var_alphaje, locals.var_alphaje_dn5, locals.var_alphaje_dn6, locals.var_alphaje_dn7, locals.var_alphaje_dn8, ) = (assign27640_e32917, (((locals.var_i1_cor_dn5 * locals.var_i2_cor) - (locals.var_i1_cor * locals.var_i2_cor_dn5)) / (locals.var_i2_cor * locals.var_i2_cor)), (((locals.var_i1_cor_dn6 * locals.var_i2_cor) - (locals.var_i1_cor * locals.var_i2_cor_dn6)) / (locals.var_i2_cor * locals.var_i2_cor)), (((locals.var_i1_cor_dn7 * locals.var_i2_cor) - (locals.var_i1_cor * locals.var_i2_cor_dn7)) / (locals.var_i2_cor * locals.var_i2_cor)), (((locals.var_i1_cor_dn8 * locals.var_i2_cor) - (locals.var_i1_cor * locals.var_i2_cor_dn8)) / (locals.var_i2_cor * locals.var_i2_cor)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) {
            let assign27650_e32930: f64 = (-locals.var_phitd);
            let assign27650_e32932: f64 = (locals.var_alphaje).ln();
            let assign27650_e32933: f64 = (assign27650_e32930 * assign27650_e32932);
            let assign27650_e32936: f64 = (locals.var_v1 - locals.var_v2);
            let assign27650_e32937: f64 = (assign27650_e32933 / assign27650_e32936);
            (locals.var_m0_rev, locals.var_m0_rev_dn5, locals.var_m0_rev_dn6, locals.var_m0_rev_dn7, locals.var_m0_rev_dn8, ) = (assign27650_e32937, ((assign27650_e32930 * (locals.var_alphaje_dn5 / locals.var_alphaje)) / assign27650_e32936), ((assign27650_e32930 * (locals.var_alphaje_dn6 / locals.var_alphaje)) / assign27650_e32936), ((assign27650_e32930 * (locals.var_alphaje_dn7 / locals.var_alphaje)) / assign27650_e32936), ((assign27650_e32930 * (locals.var_alphaje_dn8 / locals.var_alphaje)) / assign27650_e32936), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) {
            let assign27660_e32952: f64 = (locals.var_v2 - locals.var_v1);
            let assign27660_e32953: f64 = (locals.var_v2 / assign27660_e32952);
            locals.var_tt0 = assign27660_e32953;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) {
            let assign27670_e32968: f64 = (locals.var_alphaje - 1.0);
            let assign27670_e32969: f64 = (locals.var_phitd * assign27670_e32968);
            let assign27670_e32972: f64 = (locals.var_alphaje).powf(locals.var_tt0);
            let assign27670_e32974: f64 = (assign27670_e32972 - 1.0);
            let assign27670_e32975: f64 = (assign27670_e32969 * assign27670_e32974);
            (locals.var_tt1, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, ) = (assign27670_e32975, (((locals.var_phitd * locals.var_alphaje_dn5) * assign27670_e32974) + (assign27670_e32969 * if 0.0 == 0.0 && ((locals.var_tt0) as f64).is_finite() && ((locals.var_tt0) as f64).fract() == 0.0 { if locals.var_tt0 == 0.0 { 0.0 } else { (locals.var_tt0 * ((locals.var_alphaje).powf(locals.var_tt0 - 1.0) * locals.var_alphaje_dn5)) } } else { (assign27670_e32972 * (locals.var_tt0 * (locals.var_alphaje_dn5 / locals.var_alphaje))) })), (((locals.var_phitd * locals.var_alphaje_dn6) * assign27670_e32974) + (assign27670_e32969 * if 0.0 == 0.0 && ((locals.var_tt0) as f64).is_finite() && ((locals.var_tt0) as f64).fract() == 0.0 { if locals.var_tt0 == 0.0 { 0.0 } else { (locals.var_tt0 * ((locals.var_alphaje).powf(locals.var_tt0 - 1.0) * locals.var_alphaje_dn6)) } } else { (assign27670_e32972 * (locals.var_tt0 * (locals.var_alphaje_dn6 / locals.var_alphaje))) })), (((locals.var_phitd * locals.var_alphaje_dn7) * assign27670_e32974) + (assign27670_e32969 * if 0.0 == 0.0 && ((locals.var_tt0) as f64).is_finite() && ((locals.var_tt0) as f64).fract() == 0.0 { if locals.var_tt0 == 0.0 { 0.0 } else { (locals.var_tt0 * ((locals.var_alphaje).powf(locals.var_tt0 - 1.0) * locals.var_alphaje_dn7)) } } else { (assign27670_e32972 * (locals.var_tt0 * (locals.var_alphaje_dn7 / locals.var_alphaje))) })), (((locals.var_phitd * locals.var_alphaje_dn8) * assign27670_e32974) + (assign27670_e32969 * if 0.0 == 0.0 && ((locals.var_tt0) as f64).is_finite() && ((locals.var_tt0) as f64).fract() == 0.0 { if locals.var_tt0 == 0.0 { 0.0 } else { (locals.var_tt0 * ((locals.var_alphaje).powf(locals.var_tt0 - 1.0) * locals.var_alphaje_dn8)) } } else { (assign27670_e32972 * (locals.var_tt0 * (locals.var_alphaje_dn8 / locals.var_alphaje))) })), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) {
            let assign27680_e32990: f64 = (locals.var_v1 - locals.var_v2);
            let assign27680_e32991: f64 = (locals.var_v1 / assign27680_e32990);
            locals.var_tt0 = assign27680_e32991;
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) {
            let assign27690_e33005: f64 = (locals.var_alphaje).powf(locals.var_tt0);
            let assign27690_e33008: f64 = (locals.var_v2 - locals.var_v1);
            let assign27690_e33009: f64 = (assign27690_e33005 * assign27690_e33008);
            let assign27690_e33012: f64 = (locals.var_alphaje * locals.var_v1);
            let assign27690_e33013: f64 = (assign27690_e33009 + assign27690_e33012);
            let assign27690_e33015: f64 = (assign27690_e33013 - locals.var_v2);
            (locals.var_tt2, locals.var_tt2_dn5, locals.var_tt2_dn6, locals.var_tt2_dn7, locals.var_tt2_dn8, ) = (assign27690_e33015, ((if 0.0 == 0.0 && ((locals.var_tt0) as f64).is_finite() && ((locals.var_tt0) as f64).fract() == 0.0 { if locals.var_tt0 == 0.0 { 0.0 } else { (locals.var_tt0 * ((locals.var_alphaje).powf(locals.var_tt0 - 1.0) * locals.var_alphaje_dn5)) } } else { (assign27690_e33005 * (locals.var_tt0 * (locals.var_alphaje_dn5 / locals.var_alphaje))) } * assign27690_e33008) + (locals.var_alphaje_dn5 * locals.var_v1)), ((if 0.0 == 0.0 && ((locals.var_tt0) as f64).is_finite() && ((locals.var_tt0) as f64).fract() == 0.0 { if locals.var_tt0 == 0.0 { 0.0 } else { (locals.var_tt0 * ((locals.var_alphaje).powf(locals.var_tt0 - 1.0) * locals.var_alphaje_dn6)) } } else { (assign27690_e33005 * (locals.var_tt0 * (locals.var_alphaje_dn6 / locals.var_alphaje))) } * assign27690_e33008) + (locals.var_alphaje_dn6 * locals.var_v1)), ((if 0.0 == 0.0 && ((locals.var_tt0) as f64).is_finite() && ((locals.var_tt0) as f64).fract() == 0.0 { if locals.var_tt0 == 0.0 { 0.0 } else { (locals.var_tt0 * ((locals.var_alphaje).powf(locals.var_tt0 - 1.0) * locals.var_alphaje_dn7)) } } else { (assign27690_e33005 * (locals.var_tt0 * (locals.var_alphaje_dn7 / locals.var_alphaje))) } * assign27690_e33008) + (locals.var_alphaje_dn7 * locals.var_v1)), ((if 0.0 == 0.0 && ((locals.var_tt0) as f64).is_finite() && ((locals.var_tt0) as f64).fract() == 0.0 { if locals.var_tt0 == 0.0 { 0.0 } else { (locals.var_tt0 * ((locals.var_alphaje).powf(locals.var_tt0 - 1.0) * locals.var_alphaje_dn8)) } } else { (assign27690_e33005 * (locals.var_tt0 * (locals.var_alphaje_dn8 / locals.var_alphaje))) } * assign27690_e33008) + (locals.var_alphaje_dn8 * locals.var_v1)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) {
            let assign27700_e33029: f64 = (locals.var_tt1 / locals.var_tt2);
            (locals.var_mcor_rev, locals.var_mcor_rev_dn5, locals.var_mcor_rev_dn6, locals.var_mcor_rev_dn7, locals.var_mcor_rev_dn8, ) = (assign27700_e33029, (((locals.var_tt1_dn5 * locals.var_tt2) - (locals.var_tt1 * locals.var_tt2_dn5)) / (locals.var_tt2 * locals.var_tt2)), (((locals.var_tt1_dn6 * locals.var_tt2) - (locals.var_tt1 * locals.var_tt2_dn6)) / (locals.var_tt2 * locals.var_tt2)), (((locals.var_tt1_dn7 * locals.var_tt2) - (locals.var_tt1 * locals.var_tt2_dn7)) / (locals.var_tt2 * locals.var_tt2)), (((locals.var_tt1_dn8 * locals.var_tt2) - (locals.var_tt1 * locals.var_tt2_dn8)) / (locals.var_tt2 * locals.var_tt2)), );
        }
        if (((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) {
            let assign27710_e33043: f64 = (locals.var_m0_rev + locals.var_mcor_rev);
            (locals.var_mrev_s, locals.var_mrev_s_dn5, locals.var_mrev_s_dn6, locals.var_mrev_s_dn7, locals.var_mrev_s_dn8, ) = (assign27710_e33043, (locals.var_m0_rev_dn5 + locals.var_mcor_rev_dn5), (locals.var_m0_rev_dn6 + locals.var_mcor_rev_dn6), (locals.var_m0_rev_dn7 + locals.var_mcor_rev_dn7), (locals.var_m0_rev_dn8 + locals.var_mcor_rev_dn8), );
        }
        let assign27720_e33048: f64 = (locals.var_v3 * locals.var_phitdinv);
        let assign27720_e33050: f64 = (assign27720_e33048 * locals.var_mrev_s);
        let assign27720_e33051: f64 = (assign27720_e33050).abs();
        let assign27720_e33053: f64 = if assign27720_e33051 < 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard533 = assign27720_e33053;
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) && (locals.var_guard533 != 0.0)) {
            locals.var_m0flag_s = 1.0;
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) && (locals.var_guard533 != 0.0)) {
            let assign27740_e33082: f64 = (1.0 / locals.var_v3);
            let assign27740_e33085: f64 = (0.5 * locals.var_phitdinv);
            let assign27740_e33087: f64 = (assign27740_e33085 * locals.var_mrev_s);
            let assign27740_e33088: f64 = (assign27740_e33082 + assign27740_e33087);
            let assign27740_e33089: f64 = (locals.var_i3_cor * assign27740_e33088);
            (locals.var_isatrev_s, locals.var_isatrev_s_dn5, locals.var_isatrev_s_dn6, locals.var_isatrev_s_dn7, locals.var_isatrev_s_dn8, ) = (assign27740_e33089, ((locals.var_i3_cor_dn5 * assign27740_e33088) + (locals.var_i3_cor * (assign27740_e33085 * locals.var_mrev_s_dn5))), ((locals.var_i3_cor_dn6 * assign27740_e33088) + (locals.var_i3_cor * (assign27740_e33085 * locals.var_mrev_s_dn6))), ((locals.var_i3_cor_dn7 * assign27740_e33088) + (locals.var_i3_cor * (assign27740_e33085 * locals.var_mrev_s_dn7))), ((locals.var_i3_cor_dn8 * assign27740_e33088) + (locals.var_i3_cor * (assign27740_e33085 * locals.var_mrev_s_dn8))), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) && (locals.var_guard533 != 0.0)) {
            let assign27750_e33104: f64 = (-0.5);
            let assign27750_e33106: f64 = (assign27750_e33104 * locals.var_i3_cor);
            let assign27750_e33108: f64 = (assign27750_e33106 * locals.var_mrev_s);
            let assign27750_e33110: f64 = (assign27750_e33108 * locals.var_phitdinv);
            let assign27750_e33112: f64 = (assign27750_e33110 / locals.var_v3);
            (locals.var_mrev_s, locals.var_mrev_s_dn5, locals.var_mrev_s_dn6, locals.var_mrev_s_dn7, locals.var_mrev_s_dn8, ) = (assign27750_e33112, (((((assign27750_e33104 * locals.var_i3_cor_dn5) * locals.var_mrev_s) + (assign27750_e33106 * locals.var_mrev_s_dn5)) * locals.var_phitdinv) / locals.var_v3), (((((assign27750_e33104 * locals.var_i3_cor_dn6) * locals.var_mrev_s) + (assign27750_e33106 * locals.var_mrev_s_dn6)) * locals.var_phitdinv) / locals.var_v3), (((((assign27750_e33104 * locals.var_i3_cor_dn7) * locals.var_mrev_s) + (assign27750_e33106 * locals.var_mrev_s_dn7)) * locals.var_phitdinv) / locals.var_v3), (((((assign27750_e33104 * locals.var_i3_cor_dn8) * locals.var_mrev_s) + (assign27750_e33106 * locals.var_mrev_s_dn8)) * locals.var_phitdinv) / locals.var_v3), );
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) && (locals.var_guard533 == 0.0)) {
            locals.var_m0flag_s = 0.0;
        }
        if ((((((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard531 != 0.0)) && (locals.var_guard532 != 0.0)) && (locals.var_guard533 == 0.0)) {
            let assign27770_e33143: f64 = (-locals.var_i3_cor);
            let assign27770_e33145: f64 = (-locals.var_v3);
            let assign27770_e33147: f64 = (assign27770_e33145 * locals.var_phitdinv);
            let assign27770_e33149: f64 = (assign27770_e33147 * locals.var_mrev_s);
            let assign27770_e33150: f64 = (assign27770_e33149).exp();
            let assign27770_e33152: f64 = (assign27770_e33150 - 1.0);
            let assign27770_e33153: f64 = (assign27770_e33143 / assign27770_e33152);
            (locals.var_isatrev_s, locals.var_isatrev_s_dn5, locals.var_isatrev_s_dn6, locals.var_isatrev_s_dn7, locals.var_isatrev_s_dn8, ) = (assign27770_e33153, ((((-locals.var_i3_cor_dn5) * assign27770_e33152) - (assign27770_e33143 * (assign27770_e33150 * (assign27770_e33147 * locals.var_mrev_s_dn5)))) / (assign27770_e33152 * assign27770_e33152)), ((((-locals.var_i3_cor_dn6) * assign27770_e33152) - (assign27770_e33143 * (assign27770_e33150 * (assign27770_e33147 * locals.var_mrev_s_dn6)))) / (assign27770_e33152 * assign27770_e33152)), ((((-locals.var_i3_cor_dn7) * assign27770_e33152) - (assign27770_e33143 * (assign27770_e33150 * (assign27770_e33147 * locals.var_mrev_s_dn7)))) / (assign27770_e33152 * assign27770_e33152)), ((((-locals.var_i3_cor_dn8) * assign27770_e33152) - (assign27770_e33143 * (assign27770_e33150 * (assign27770_e33147 * locals.var_mrev_s_dn8)))) / (assign27770_e33152 * assign27770_e33152)), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27780_e33162: f64 = (locals.var_absource_i * locals.var_cjobot);
            let assign27780_e33165: f64 = (locals.var_lssource_i * locals.var_cjosti);
            let assign27780_e33166: f64 = (assign27780_e33162 + assign27780_e33165);
            let assign27780_e33169: f64 = (locals.var_lgsource_i * locals.var_cjogat);
            let assign27780_e33170: f64 = (assign27780_e33166 + assign27780_e33169);
            let assign27780_e33171: f64 = (p.p929 * assign27780_e33170);
            locals.var_zfrac = assign27780_e33171;
        }
        let assign27790_e33176: f64 = (locals.var_absource_i * locals.var_cjobot);
        let assign27790_e33178: f64 = if assign27790_e33176 <= locals.var_zfrac { 1.0 } else { 0.0 };
        locals.var_guard534 = assign27790_e33178;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard534 != 0.0)) {
            locals.var_zflagbot_s = 0.0;
        }
        let assign27810_e33189: f64 = (locals.var_lssource_i * locals.var_cjosti);
        let assign27810_e33191: f64 = if assign27810_e33189 <= locals.var_zfrac { 1.0 } else { 0.0 };
        locals.var_guard535 = assign27810_e33191;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard535 != 0.0)) {
            locals.var_zflagsti_s = 0.0;
        }
        let assign27830_e33202: f64 = (locals.var_lgsource_i * locals.var_cjogat);
        let assign27830_e33204: f64 = if assign27830_e33202 <= locals.var_zfrac { 1.0 } else { 0.0 };
        locals.var_guard536 = assign27830_e33204;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard536 != 0.0)) {
            locals.var_zflaggat_s = 0.0;
        }
        let assign27850_e33224: f64 = if (!(((locals.var_absource_i == 0.0) && (locals.var_lssource_i == 0.0)) && (locals.var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard537 = assign27850_e33224;
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard537 != 0.0)) {
            let assign27860_e33232: f64 = (0.5 * p.p822);
            let assign27860_e33235: f64 = (locals.var_isatfor1_s + 1e-21);
            let assign27860_e33236: f64 = (assign27860_e33232 / assign27860_e33235);
            let assign27860_e33237: f64 = (assign27860_e33236).ln();
            locals.var_xhighf1_s = assign27860_e33237;
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard537 != 0.0)) {
            let assign27870_e33247: f64 = (0.5 * p.p822);
            let assign27870_e33250: f64 = (locals.var_isatfor2_s + 1e-21);
            let assign27870_e33251: f64 = (assign27870_e33247 / assign27870_e33250);
            let assign27870_e33252: f64 = (assign27870_e33251).ln();
            (locals.var_xhighf2_s, locals.var_xhighf2_s_dn5, locals.var_xhighf2_s_dn6, locals.var_xhighf2_s_dn7, locals.var_xhighf2_s_dn8, ) = (assign27870_e33252, ((-((assign27870_e33247 * locals.var_isatfor2_s_dn5) / (assign27870_e33250 * assign27870_e33250))) / assign27870_e33251), ((-((assign27870_e33247 * locals.var_isatfor2_s_dn6) / (assign27870_e33250 * assign27870_e33250))) / assign27870_e33251), ((-((assign27870_e33247 * locals.var_isatfor2_s_dn7) / (assign27870_e33250 * assign27870_e33250))) / assign27870_e33251), ((-((assign27870_e33247 * locals.var_isatfor2_s_dn8) / (assign27870_e33250 * assign27870_e33250))) / assign27870_e33251), );
        }
        if (((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard537 != 0.0)) {
            let assign27880_e33262: f64 = (0.5 * p.p822);
            let assign27880_e33264: f64 = (locals.var_isatrev_s).abs();
            let assign27880_e33266: f64 = (assign27880_e33264 + 1e-21);
            let assign27880_e33267: f64 = (assign27880_e33262 / assign27880_e33266);
            let assign27880_e33268: f64 = (assign27880_e33267).ln();
            (locals.var_xhighr_s, locals.var_xhighr_s_dn5, locals.var_xhighr_s_dn6, locals.var_xhighr_s_dn7, locals.var_xhighr_s_dn8, ) = (assign27880_e33268, ((-((assign27880_e33262 * if locals.var_isatrev_s >= 0.0 { locals.var_isatrev_s_dn5 } else { (-locals.var_isatrev_s_dn5) }) / (assign27880_e33266 * assign27880_e33266))) / assign27880_e33267), ((-((assign27880_e33262 * if locals.var_isatrev_s >= 0.0 { locals.var_isatrev_s_dn6 } else { (-locals.var_isatrev_s_dn6) }) / (assign27880_e33266 * assign27880_e33266))) / assign27880_e33267), ((-((assign27880_e33262 * if locals.var_isatrev_s >= 0.0 { locals.var_isatrev_s_dn7 } else { (-locals.var_isatrev_s_dn7) }) / (assign27880_e33266 * assign27880_e33266))) / assign27880_e33267), ((-((assign27880_e33262 * if locals.var_isatrev_s >= 0.0 { locals.var_isatrev_s_dn8 } else { (-locals.var_isatrev_s_dn8) }) / (assign27880_e33266 * assign27880_e33266))) / assign27880_e33267), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27890_e33276: f64 = (locals.var_xhighf1_s).min(230.25850929940458);
            locals.var_xhighf1_s = assign27890_e33276;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27900_e33283: f64 = (locals.var_xhighf1_s).exp();
            locals.var_expxhf1_s = assign27900_e33283;
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27910_e33291: f64 = (locals.var_xhighf2_s).min(230.25850929940458);
            (locals.var_xhighf2_s, locals.var_xhighf2_s_dn5, locals.var_xhighf2_s_dn6, locals.var_xhighf2_s_dn7, locals.var_xhighf2_s_dn8, ) = (assign27910_e33291, if locals.var_xhighf2_s <= 230.25850929940458 { locals.var_xhighf2_s_dn5 } else { 0.0 }, if locals.var_xhighf2_s <= 230.25850929940458 { locals.var_xhighf2_s_dn6 } else { 0.0 }, if locals.var_xhighf2_s <= 230.25850929940458 { locals.var_xhighf2_s_dn7 } else { 0.0 }, if locals.var_xhighf2_s <= 230.25850929940458 { locals.var_xhighf2_s_dn8 } else { 0.0 }, );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27920_e33298: f64 = (locals.var_xhighf2_s).exp();
            (locals.var_expxhf2_s, locals.var_expxhf2_s_dn5, locals.var_expxhf2_s_dn6, locals.var_expxhf2_s_dn7, locals.var_expxhf2_s_dn8, ) = (assign27920_e33298, (assign27920_e33298 * locals.var_xhighf2_s_dn5), (assign27920_e33298 * locals.var_xhighf2_s_dn6), (assign27920_e33298 * locals.var_xhighf2_s_dn7), (assign27920_e33298 * locals.var_xhighf2_s_dn8), );
        }
        if ((locals.var_guard182 != 0.0) && (locals.var_guard199 != 0.0)) {
            let assign27930_e33306: f64 = (locals.var_xhighr_s).min(230.25850929940458);
            (locals.var_xhighr_s, locals.var_xhighr_s_dn5, locals.var_xhighr_s_dn6, locals.var_xhighr_s_dn7, locals.var_xhighr_s_dn8, ) = (assign27930_e33306, if locals.var_xhighr_s <= 230.25850929940458 { locals.var_xhighr_s_dn5 } else { 0.0 }, if locals.var_xhighr_s <= 230.25850929940458 { locals.var_xhighr_s_dn6 } else { 0.0 }, if locals.var_xhighr_s <= 230.25850929940458 { locals.var_xhighr_s_dn7 } else { 0.0 }, if locals.var_xhighr_s <= 230.25850929940458 { locals.var_xhighr_s_dn8 } else { 0.0 }, );
        }
    }
}

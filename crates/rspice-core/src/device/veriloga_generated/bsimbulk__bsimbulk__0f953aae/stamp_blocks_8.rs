#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10660_e14808,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && ((locals.var_guard364 != 0.0) && (locals.var_guard363 == 0.0))) && (locals.var_guard367 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10660_e14808;
        locals.var_rend_rv = 0.0;

        let (assign10670_e14849,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && ((locals.var_guard364 != 0.0) && (locals.var_guard363 == 0.0))) && (locals.var_guard367 == 0.0)) {
        let assign10670_e14839: f64 = (p.p374 * locals.var_weff);
        let assign10670_e14842: f64 = (3.0 * locals.var_nuends);
        let assign10670_e14845: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10670_e14846: f64 = (assign10670_e14842 * assign10670_e14845);
        let assign10670_e14847: f64 = (assign10670_e14839 / assign10670_e14846);
        (assign10670_e14847,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10670_e14849;
        locals.var_rend_rv = 0.0;

        let (assign10680_e14877,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && (!((locals.var_guard363 != 0.0) || (locals.var_guard364 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10680_e14877;
        locals.var_rend_rv = 0.0;

        let (assign10690_e14902,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 == 0.0)) {
        let assign10690_e14898: f64 = (p.p374 * locals.var_dmdgeff);
        let assign10690_e14900: f64 = (assign10690_e14898 / locals.var_weff);
        (assign10690_e14900,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10690_e14902;
        locals.var_rend_rv = 0.0;

        let assign10700_e14905: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard368 = assign10700_e14905;
        locals.var_guard368_rv = 0.0;

        let assign10710_e14908: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign10710_e14908;
        locals.var_guard369_rv = 0.0;

        let assign10720_e14919: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard370 = assign10720_e14919;
        locals.var_guard370_rv = 0.0;

        let assign10730_e14930: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard371 = assign10730_e14930;
        locals.var_guard371_rv = 0.0;

        let assign10740_e14933: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard372 = assign10740_e14933;
        locals.var_guard372_rv = 0.0;

        let (assign10750_e14961,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard372 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10750_e14961;
        locals.var_rend_rv = 0.0;

        let (assign10760_e14996,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard372 == 0.0)) {
        let assign10760_e14990: f64 = (p.p374 * locals.var_dmcgeff);
        let assign10760_e14993: f64 = (locals.var_weff * locals.var_nuends);
        let assign10760_e14994: f64 = (assign10760_e14990 / assign10760_e14993);
        (assign10760_e14994,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10760_e14996;
        locals.var_rend_rv = 0.0;

        let assign10780_e15006: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard374 = assign10780_e15006;
        locals.var_guard374_rv = 0.0;

        let (assign10790_e15037,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && ((locals.var_guard371 != 0.0) && (locals.var_guard370 == 0.0))) && (locals.var_guard374 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10790_e15037;
        locals.var_rend_rv = 0.0;

        let (assign10800_e15077,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && ((locals.var_guard371 != 0.0) && (locals.var_guard370 == 0.0))) && (locals.var_guard374 == 0.0)) {
        let assign10800_e15069: f64 = (p.p374 * locals.var_weff);
        let assign10800_e15072: f64 = (6.0 * locals.var_nuends);
        let assign10800_e15074: f64 = (assign10800_e15072 * locals.var_dmcgeff);
        let assign10800_e15075: f64 = (assign10800_e15069 / assign10800_e15074);
        (assign10800_e15075,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10800_e15077;
        locals.var_rend_rv = 0.0;

        let (assign10810_e15106,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && (!((locals.var_guard370 != 0.0) || (locals.var_guard371 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10810_e15106;
        locals.var_rend_rv = 0.0;

        let assign10820_e15117: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard375 = assign10820_e15117;
        locals.var_guard375_rv = 0.0;

        let assign10830_e15128: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard376 = assign10830_e15128;
        locals.var_guard376_rv = 0.0;

        let assign10840_e15131: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard377 = assign10840_e15131;
        locals.var_guard377_rv = 0.0;

        let (assign10850_e15160,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard375 != 0.0)) && (locals.var_guard377 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10850_e15160;
        locals.var_rend_rv = 0.0;

        let (assign10860_e15196,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard375 != 0.0)) && (locals.var_guard377 == 0.0)) {
        let assign10860_e15190: f64 = (p.p374 * locals.var_dmcgeff);
        let assign10860_e15193: f64 = (locals.var_weff * locals.var_nuends);
        let assign10860_e15194: f64 = (assign10860_e15190 / assign10860_e15193);
        (assign10860_e15194,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10860_e15196;
        locals.var_rend_rv = 0.0;

        let assign10880_e15206: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard379 = assign10880_e15206;
        locals.var_guard379_rv = 0.0;

        let (assign10890_e15238,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && ((locals.var_guard376 != 0.0) && (locals.var_guard375 == 0.0))) && (locals.var_guard379 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10890_e15238;
        locals.var_rend_rv = 0.0;

        let (assign10900_e15279,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && ((locals.var_guard376 != 0.0) && (locals.var_guard375 == 0.0))) && (locals.var_guard379 == 0.0)) {
        let assign10900_e15271: f64 = (p.p374 * locals.var_weff);
        let assign10900_e15274: f64 = (6.0 * locals.var_nuends);
        let assign10900_e15276: f64 = (assign10900_e15274 * locals.var_dmcgeff);
        let assign10900_e15277: f64 = (assign10900_e15271 / assign10900_e15276);
        (assign10900_e15277,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10900_e15279;
        locals.var_rend_rv = 0.0;

        let (assign10910_e15309,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && (!((locals.var_guard375 != 0.0) || (locals.var_guard376 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10910_e15309;
        locals.var_rend_rv = 0.0;

        let assign10920_e15312: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard380 = assign10920_e15312;
        locals.var_guard380_rv = 0.0;

        let (assign10930_e15337,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 == 0.0)) && (locals.var_guard380 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10930_e15337;
        locals.var_rend_rv = 0.0;

        let (assign10940_e15369,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 == 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign10940_e15363: f64 = (p.p374 * locals.var_dmdgeff);
        let assign10940_e15366: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10940_e15367: f64 = (assign10940_e15363 / assign10940_e15366);
        (assign10940_e15367,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10940_e15369;
        locals.var_rend_rv = 0.0;

        let assign10950_e15372: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign10950_e15372;
        locals.var_guard381_rv = 0.0;

        let (assign10960_e15400,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 != 0.0)) {
        let assign10960_e15396: f64 = (p.p374 * locals.var_dmdgeff);
        let assign10960_e15398: f64 = (assign10960_e15396 / locals.var_weff);
        (assign10960_e15398,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10960_e15400;
        locals.var_rend_rv = 0.0;

        let assign10970_e15403: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard382 = assign10970_e15403;
        locals.var_guard382_rv = 0.0;

        let assign10980_e15414: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard383 = assign10980_e15414;
        locals.var_guard383_rv = 0.0;

        let assign10990_e15425: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard384 = assign10990_e15425;
        locals.var_guard384_rv = 0.0;

        let assign11000_e15428: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard385 = assign11000_e15428;
        locals.var_guard385_rv = 0.0;

        let (assign11010_e15459,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11010_e15459;
        locals.var_rend_rv = 0.0;

        let (assign11020_e15497,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign11020_e15491: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11020_e15494: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11020_e15495: f64 = (assign11020_e15491 / assign11020_e15494);
        (assign11020_e15495,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11020_e15497;
        locals.var_rend_rv = 0.0;

        let assign11040_e15508: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11040_e15511: f64 = if ((locals.var_nuendd == 0.0) || (assign11040_e15508 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard387 = assign11040_e15511;
        locals.var_guard387_rv = 0.0;

        let (assign11050_e15545,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && ((locals.var_guard384 != 0.0) && (locals.var_guard383 == 0.0))) && (locals.var_guard387 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11050_e15545;
        locals.var_rend_rv = 0.0;

        let (assign11060_e15590,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && ((locals.var_guard384 != 0.0) && (locals.var_guard383 == 0.0))) && (locals.var_guard387 == 0.0)) {
        let assign11060_e15580: f64 = (p.p374 * locals.var_weff);
        let assign11060_e15583: f64 = (3.0 * locals.var_nuendd);
        let assign11060_e15586: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11060_e15587: f64 = (assign11060_e15583 * assign11060_e15586);
        let assign11060_e15588: f64 = (assign11060_e15580 / assign11060_e15587);
        (assign11060_e15588,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11060_e15590;
        locals.var_rend_rv = 0.0;

        let (assign11070_e15622,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && (!((locals.var_guard383 != 0.0) || (locals.var_guard384 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11070_e15622;
        locals.var_rend_rv = 0.0;

        let assign11080_e15633: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard388 = assign11080_e15633;
        locals.var_guard388_rv = 0.0;

        let assign11090_e15644: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard389 = assign11090_e15644;
        locals.var_guard389_rv = 0.0;

        let assign11100_e15647: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard390 = assign11100_e15647;
        locals.var_guard390_rv = 0.0;

        let (assign11110_e15679,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard388 != 0.0)) && (locals.var_guard390 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11110_e15679;
        locals.var_rend_rv = 0.0;

        let (assign11120_e15718,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard388 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign11120_e15712: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11120_e15715: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11120_e15716: f64 = (assign11120_e15712 / assign11120_e15715);
        (assign11120_e15716,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11120_e15718;
        locals.var_rend_rv = 0.0;

        let assign11140_e15729: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11140_e15732: f64 = if ((locals.var_nuendd == 0.0) || (assign11140_e15729 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard392 = assign11140_e15732;
        locals.var_guard392_rv = 0.0;

        let (assign11150_e15767,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && ((locals.var_guard389 != 0.0) && (locals.var_guard388 == 0.0))) && (locals.var_guard392 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11150_e15767;
        locals.var_rend_rv = 0.0;

        let (assign11160_e15813,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && ((locals.var_guard389 != 0.0) && (locals.var_guard388 == 0.0))) && (locals.var_guard392 == 0.0)) {
        let assign11160_e15803: f64 = (p.p374 * locals.var_weff);
        let assign11160_e15806: f64 = (3.0 * locals.var_nuendd);
        let assign11160_e15809: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11160_e15810: f64 = (assign11160_e15806 * assign11160_e15809);
        let assign11160_e15811: f64 = (assign11160_e15803 / assign11160_e15810);
        (assign11160_e15811,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11160_e15813;
        locals.var_rend_rv = 0.0;

        let (assign11170_e15846,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && (!((locals.var_guard388 != 0.0) || (locals.var_guard389 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11170_e15846;
        locals.var_rend_rv = 0.0;

        let assign11180_e15849: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard393 = assign11180_e15849;
        locals.var_guard393_rv = 0.0;

        let assign11190_e15852: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign11190_e15852;
        locals.var_guard394_rv = 0.0;

        let (assign11200_e15880,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 != 0.0)) && (locals.var_guard394 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11200_e15880;
        locals.var_rend_rv = 0.0;

        let (assign11210_e15915,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 != 0.0)) && (locals.var_guard394 == 0.0)) {
        let assign11210_e15909: f64 = (p.p374 * locals.var_dmdgeff);
        let assign11210_e15912: f64 = (locals.var_weff * locals.var_nuends);
        let assign11210_e15913: f64 = (assign11210_e15909 / assign11210_e15912);
        (assign11210_e15913,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11210_e15915;
        locals.var_rend_rv = 0.0;

        let assign11220_e15918: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard395 = assign11220_e15918;
        locals.var_guard395_rv = 0.0;

        let assign11230_e15929: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard396 = assign11230_e15929;
        locals.var_guard396_rv = 0.0;

        let assign11240_e15940: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard397 = assign11240_e15940;
        locals.var_guard397_rv = 0.0;

        let assign11250_e15943: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard398 = assign11250_e15943;
        locals.var_guard398_rv = 0.0;

        let (assign11260_e15976,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard398 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11260_e15976;
        locals.var_rend_rv = 0.0;

        let (assign11270_e16016,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard398 == 0.0)) {
        let assign11270_e16010: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11270_e16013: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11270_e16014: f64 = (assign11270_e16010 / assign11270_e16013);
        (assign11270_e16014,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11270_e16016;
        locals.var_rend_rv = 0.0;

        let assign11290_e16026: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard400 = assign11290_e16026;
        locals.var_guard400_rv = 0.0;

        let (assign11300_e16062,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && ((locals.var_guard397 != 0.0) && (locals.var_guard396 == 0.0))) && (locals.var_guard400 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11300_e16062;
        locals.var_rend_rv = 0.0;

        let (assign11310_e16107,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && ((locals.var_guard397 != 0.0) && (locals.var_guard396 == 0.0))) && (locals.var_guard400 == 0.0)) {
        let assign11310_e16099: f64 = (p.p374 * locals.var_weff);
        let assign11310_e16102: f64 = (6.0 * locals.var_nuendd);
        let assign11310_e16104: f64 = (assign11310_e16102 * locals.var_dmcgeff);
        let assign11310_e16105: f64 = (assign11310_e16099 / assign11310_e16104);
        (assign11310_e16105,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11310_e16107;
        locals.var_rend_rv = 0.0;

        let (assign11320_e16141,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && (!((locals.var_guard396 != 0.0) || (locals.var_guard397 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11320_e16141;
        locals.var_rend_rv = 0.0;

        let assign11330_e16152: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard401 = assign11330_e16152;
        locals.var_guard401_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign11340_e16163: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard402 = assign11340_e16163;
        locals.var_guard402_rv = 0.0;

        let assign11350_e16166: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard403 = assign11350_e16166;
        locals.var_guard403_rv = 0.0;

        let (assign11360_e16200,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_guard403 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11360_e16200;
        locals.var_rend_rv = 0.0;

        let (assign11370_e16241,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_guard403 == 0.0)) {
        let assign11370_e16235: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11370_e16238: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11370_e16239: f64 = (assign11370_e16235 / assign11370_e16238);
        (assign11370_e16239,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11370_e16241;
        locals.var_rend_rv = 0.0;

        let assign11390_e16251: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard405 = assign11390_e16251;
        locals.var_guard405_rv = 0.0;

        let (assign11400_e16288,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && ((locals.var_guard402 != 0.0) && (locals.var_guard401 == 0.0))) && (locals.var_guard405 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11400_e16288;
        locals.var_rend_rv = 0.0;

        let (assign11410_e16334,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && ((locals.var_guard402 != 0.0) && (locals.var_guard401 == 0.0))) && (locals.var_guard405 == 0.0)) {
        let assign11410_e16326: f64 = (p.p374 * locals.var_weff);
        let assign11410_e16329: f64 = (6.0 * locals.var_nuendd);
        let assign11410_e16331: f64 = (assign11410_e16329 * locals.var_dmcgeff);
        let assign11410_e16332: f64 = (assign11410_e16326 / assign11410_e16331);
        (assign11410_e16332,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11410_e16334;
        locals.var_rend_rv = 0.0;

        let (assign11420_e16369,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && (!((locals.var_guard401 != 0.0) || (locals.var_guard402 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11420_e16369;
        locals.var_rend_rv = 0.0;

        let (assign11430_e16399,) = {
    if (((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard261 != 0.0) && (!((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) {
        let assign11430_e16395: f64 = (p.p374 * locals.var_dmdgeff);
        let assign11430_e16397: f64 = (assign11430_e16395 / locals.var_weff);
        (assign11430_e16397,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11430_e16399;
        locals.var_rend_rv = 0.0;

        let assign11440_e16402: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard406 = assign11440_e16402;
        locals.var_guard406_rv = 0.0;

        let (assign11450_e16438,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 != 0.0)) {
        let assign11450_e16432: f64 = (0.5 * p.p374);
        let assign11450_e16434: f64 = (assign11450_e16432 * locals.var_dmcgeff);
        let assign11450_e16436: f64 = (assign11450_e16434 / locals.var_weff);
        (assign11450_e16436,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11450_e16438;
        locals.var_rend_rv = 0.0;

        let assign11460_e16441: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign11460_e16441;
        locals.var_guard407_rv = 0.0;

        let (assign11470_e16473,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 != 0.0)) && (locals.var_guard407 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11470_e16473;
        locals.var_rint_rv = 0.0;

        let (assign11480_e16514,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 != 0.0)) && (locals.var_guard407 == 0.0)) {
        let assign11480_e16506: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11480_e16510: f64 = (p.p2 - 2.0);
        let assign11480_e16511: f64 = (locals.var_weff * assign11480_e16510);
        let assign11480_e16512: f64 = (assign11480_e16506 / assign11480_e16511);
        (assign11480_e16512,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11480_e16514;
        locals.var_rint_rv = 0.0;

        let (assign11490_e16545,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11490_e16545;
        locals.var_rend_rv = 0.0;

        let (assign11500_e16582,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 == 0.0)) {
        let assign11500_e16576: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11500_e16579: f64 = (locals.var_weff * p.p2);
        let assign11500_e16580: f64 = (assign11500_e16576 / assign11500_e16579);
        (assign11500_e16580,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11500_e16582;
        locals.var_rint_rv = 0.0;

        let assign11510_e16585: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign11510_e16585;
        locals.var_guard408_rv = 0.0;

        let (assign11520_e16617,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11520_e16617;
        locals.var_rend_rv = 0.0;

        let (assign11530_e16655,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 != 0.0)) {
        let assign11530_e16649: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11530_e16652: f64 = (locals.var_weff * p.p2);
        let assign11530_e16653: f64 = (assign11530_e16649 / assign11530_e16652);
        (assign11530_e16653,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11530_e16655;
        locals.var_rint_rv = 0.0;

        let (assign11540_e16694,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 == 0.0)) {
        let assign11540_e16688: f64 = (0.5 * p.p374);
        let assign11540_e16690: f64 = (assign11540_e16688 * locals.var_dmcgeff);
        let assign11540_e16692: f64 = (assign11540_e16690 / locals.var_weff);
        (assign11540_e16692,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11540_e16694;
        locals.var_rend_rv = 0.0;

        let assign11550_e16697: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard409 = assign11550_e16697;
        locals.var_guard409_rv = 0.0;

        let (assign11560_e16732,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 == 0.0)) && (locals.var_guard409 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11560_e16732;
        locals.var_rint_rv = 0.0;

        let (assign11570_e16776,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 == 0.0)) && (locals.var_guard409 == 0.0)) {
        let assign11570_e16768: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11570_e16772: f64 = (p.p2 - 2.0);
        let assign11570_e16773: f64 = (locals.var_weff * assign11570_e16772);
        let assign11570_e16774: f64 = (assign11570_e16768 / assign11570_e16773);
        (assign11570_e16774,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11570_e16776;
        locals.var_rint_rv = 0.0;

        let (assign11580_e16806,) = {
    if (((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (!(((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11580_e16806;
        locals.var_rint_rv = 0.0;

        let assign11590_e16809: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign11590_e16809;
        locals.var_guard410_rv = 0.0;

        let (assign11600_e16818,) = {
    if (((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard410 != 0.0)) {
        (locals.var_rend,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign11600_e16818;
        locals.var_rdraingeo_rv = 0.0;

        let assign11610_e16821: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign11610_e16821;
        locals.var_guard411_rv = 0.0;

        let (assign11620_e16833,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard410 == 0.0)) && (locals.var_guard411 != 0.0)) {
        (locals.var_rint,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign11620_e16833;
        locals.var_rdraingeo_rv = 0.0;

        let (assign11630_e16852,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard410 == 0.0)) && (locals.var_guard411 == 0.0)) {
        let assign11630_e16846: f64 = (locals.var_rint * locals.var_rend);
        let assign11630_e16849: f64 = (locals.var_rint + locals.var_rend);
        let assign11630_e16850: f64 = (assign11630_e16846 / assign11630_e16849);
        (assign11630_e16850,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign11630_e16852;
        locals.var_rdraingeo_rv = 0.0;

        let (assign11650_e16863,) = {
    if ((locals.var_guard245 == 0.0) && (locals.var_guard246 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign11650_e16863;
        locals.var_rdraingeo_rv = 0.0;

        let assign11660_e16866: f64 = if p.p42 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard413 = assign11660_e16866;
        locals.var_guard413_rv = 0.0;

        let assign11670_e16869: f64 = if locals.var_rsourcegeo < p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign11670_e16869;
        locals.var_guard414_rv = 0.0;

        let (assign11680_e16875,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign11680_e16875;
        locals.var_rsourcegeo_rv = 0.0;

        let assign11690_e16878: f64 = if locals.var_rdraingeo < p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard415 = assign11690_e16878;
        locals.var_guard415_rv = 0.0;

        let (assign11700_e16884,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign11700_e16884;
        locals.var_rdraingeo_rv = 0.0;

        let assign11710_e16887: f64 = if locals.var_rsourcegeo <= p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign11710_e16887;
        locals.var_guard416_rv = 0.0;

        let (assign11720_e16894,) = {
    if ((locals.var_guard413 == 0.0) && (locals.var_guard416 != 0.0)) {
        (p.p1093,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign11720_e16894;
        locals.var_rsourcegeo_rv = 0.0;

        let assign11730_e16897: f64 = if locals.var_rdraingeo <= p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard417 = assign11730_e16897;
        locals.var_guard417_rv = 0.0;

        let (assign11740_e16904,) = {
    if ((locals.var_guard413 == 0.0) && (locals.var_guard417 != 0.0)) {
        (p.p1093,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign11740_e16904;
        locals.var_rdraingeo_rv = 0.0;

        let assign11750_e16907: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard418 = assign11750_e16907;
        locals.var_guard418_rv = 0.0;

        let assign11760_e16910: f64 = if locals.var_rswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign11760_e16910;
        locals.var_guard419_rv = 0.0;

        let (assign11770_e16916,) = {
    if ((locals.var_guard418 != 0.0) && (locals.var_guard419 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rswmin_i,)
    }
};
        locals.var_rswmin_i = assign11770_e16916;
        locals.var_rswmin_i_rv = 0.0;

        let assign11780_e16919: f64 = if locals.var_rdwmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard420 = assign11780_e16919;
        locals.var_guard420_rv = 0.0;

        let (assign11790_e16925,) = {
    if ((locals.var_guard418 != 0.0) && (locals.var_guard420 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdwmin_i,)
    }
};
        locals.var_rdwmin_i = assign11790_e16925;
        locals.var_rdwmin_i_rv = 0.0;

        let assign11800_e16928: f64 = if locals.var_rsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign11800_e16928;
        locals.var_guard421_rv = 0.0;

        let (assign11810_e16934,) = {
    if ((locals.var_guard418 != 0.0) && (locals.var_guard421 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsw_i,)
    }
};
        locals.var_rsw_i = assign11810_e16934;
        locals.var_rsw_i_rv = 0.0;

        let assign11820_e16937: f64 = if locals.var_rdw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard422 = assign11820_e16937;
        locals.var_guard422_rv = 0.0;

        let (assign11830_e16943,) = {
    if ((locals.var_guard418 != 0.0) && (locals.var_guard422 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdw_i,)
    }
};
        locals.var_rdw_i = assign11830_e16943;
        locals.var_rdw_i_rv = 0.0;

        let assign11840_e16946: f64 = if locals.var_rdswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign11840_e16946;
        locals.var_guard423_rv = 0.0;

        let (assign11850_e16953,) = {
    if ((locals.var_guard418 == 0.0) && (locals.var_guard423 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdswmin_i,)
    }
};
        locals.var_rdswmin_i = assign11850_e16953;
        locals.var_rdswmin_i_rv = 0.0;

        let assign11860_e16956: f64 = if locals.var_rdsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard424 = assign11860_e16956;
        locals.var_guard424_rv = 0.0;

        let (assign11870_e16963,) = {
    if ((locals.var_guard418 == 0.0) && (locals.var_guard424 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdsw_i,)
    }
};
        locals.var_rdsw_i = assign11870_e16963;
        locals.var_rdsw_i_rv = 0.0;

        let assign12580_e17615: f64 = if p.p1097 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard443 = assign12580_e17615;
        locals.var_guard443_rv = 0.0;

        let (assign12620_e17641,) = {
    if (locals.var_guard443 != 0.0) {
        let assign12620_e17639: f64 = (1.0 - p.p1128);
        (assign12620_e17639,)
    } else {
        (locals.var_oneminusxpart,)
    }
};
        locals.var_oneminusxpart = assign12620_e17641;
        locals.var_oneminusxpart_rv = 0.0;

        let (assign12630_e17646,) = {
    if (locals.var_guard443 == 0.0) {
        (1.0,)
    } else {
        (locals.var_oneminusxpart,)
    }
};
        locals.var_oneminusxpart = assign12630_e17646;
        locals.var_oneminusxpart_rv = 0.0;

        let assign12640_e17651: f64 = (locals.var_weffcj / 3.0);
        let assign12640_e17653: f64 = (assign12640_e17651 / p.p32);
        let assign12640_e17654: f64 = (p.p31 + assign12640_e17653);
        let assign12640_e17655: f64 = (p.p700 * assign12640_e17654);
        let assign12640_e17658: f64 = (p.p32 * p.p2);
        let assign12640_e17661: f64 = (locals.var_lnew - p.p699);
        let assign12640_e17662: f64 = (assign12640_e17658 * assign12640_e17661);
        let assign12640_e17663: f64 = (assign12640_e17655 / assign12640_e17662);
        locals.var_grgeltd = assign12640_e17663;
        locals.var_grgeltd_rv = 0.0;

        let assign12650_e17666: f64 = if locals.var_grgeltd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard445 = assign12650_e17666;
        locals.var_guard445_rv = 0.0;

        let (assign12660_e17672,) = {
    if (locals.var_guard445 != 0.0) {
        let assign12660_e17670: f64 = (1.0 / locals.var_grgeltd);
        (assign12660_e17670,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign12660_e17672;
        locals.var_grgeltd_rv = 0.0;

        let (assign12670_e17677,) = {
    if (locals.var_guard445 == 0.0) {
        (1000.0,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign12670_e17677;
        locals.var_grgeltd_rv = 0.0;

        let assign12690_e17683: f64 = (p.p77 * p.p77);
        locals.var_t0 = assign12690_e17683;
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
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let assign12700_e17686: f64 = (p.p77 * locals.var_poxedge_i);
        locals.var_t1 = assign12700_e17686;
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
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign12710_e17689: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign12710_e17689;
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
        locals.var_t2_dn12 = ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12));
        locals.var_t2_dn13 = ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13));
        locals.var_t2_dn14 = ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14));
        locals.var_t2_rv = 0.0;

        let (assign12750_e17723,) = {
    if (p.p39 == 1.0) {
        (745669000000.0,)
    } else {
        (1166450000000.0,)
    }
};
        locals.var_bechvb = assign12750_e17723;
        locals.var_bechvb_rv = 0.0;

        let assign12770_e17730: f64 = (-locals.var_bechvb);
        let assign12770_e17732: f64 = (assign12770_e17730 * p.p77);
        let assign12770_e17734: f64 = (assign12770_e17732 * locals.var_poxedge_i);
        locals.var_bechvbedge = assign12770_e17734;
        locals.var_bechvbedge_rv = 0.0;

        let assign12790_e17743: f64 = (-locals.var_bechvb);
        let assign12790_e17745: f64 = (assign12790_e17743 * p.p77);
        locals.var_bechvb = assign12790_e17745;
        locals.var_bechvb_rv = 0.0;

        let assign12800_e17748: f64 = (p.p911 + locals.var_weff);
        locals.var_weff_sh = assign12800_e17748;
        locals.var_weff_sh_rv = 0.0;

        let assign12810_e17759: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard447 = assign12810_e17759;
        locals.var_guard447_rv = 0.0;

        let (assign12820_e17767,) = {
    if (locals.var_guard447 != 0.0) {
        let assign12820_e17763: f64 = (locals.var_weff_sh * p.p2);
        let assign12820_e17765: f64 = (assign12820_e17763 / p.p909);
        (assign12820_e17765,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign12820_e17767;
        locals.var_gth_rv = 0.0;

        let (assign12830_e17775,) = {
    if (locals.var_guard447 != 0.0) {
        let assign12830_e17771: f64 = (p.p910 * locals.var_weff_sh);
        let assign12830_e17773: f64 = (assign12830_e17771 * p.p2);
        (assign12830_e17773,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign12830_e17775;
        locals.var_cth_rv = 0.0;

        let (assign12840_e17780,) = {
    if (locals.var_guard447 == 0.0) {
        (1.0,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign12840_e17780;
        locals.var_gth_rv = 0.0;

        let (assign12850_e17785,) = {
    if (locals.var_guard447 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign12850_e17785;
        locals.var_cth_rv = 0.0;

        let assign12860_e17788: f64 = (-273.15);
        let assign12860_e17789: f64 = if p.p820 <= assign12860_e17788 { 1.0 } else { 0.0 };
        locals.var_guard448 = assign12860_e17789;
        locals.var_guard448_rv = 0.0;

        let (assign12870_e17795, assign12870_e17795_d_n0, assign12870_e17795_d_n2, assign12870_e17795_d_n3, assign12870_e17795_d_n4, assign12870_e17795_d_n5, assign12870_e17795_d_n6, assign12870_e17795_d_n7, assign12870_e17795_d_n8, assign12870_e17795_d_n9, assign12870_e17795_d_n10, assign12870_e17795_d_n11, assign12870_e17795_d_n12, assign12870_e17795_d_n13, assign12870_e17795_d_n14,) = {
    if (locals.var_guard448 != 0.0) {
        let assign12870_e17793: f64 = (300.15 - 273.15);
        (assign12870_e17793, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign12870_e17795;
        locals.var_t0_dn0 = assign12870_e17795_d_n0;
        locals.var_t0_dn2 = assign12870_e17795_d_n2;
        locals.var_t0_dn3 = assign12870_e17795_d_n3;
        locals.var_t0_dn4 = assign12870_e17795_d_n4;
        locals.var_t0_dn5 = assign12870_e17795_d_n5;
        locals.var_t0_dn6 = assign12870_e17795_d_n6;
        locals.var_t0_dn7 = assign12870_e17795_d_n7;
        locals.var_t0_dn8 = assign12870_e17795_d_n8;
        locals.var_t0_dn9 = assign12870_e17795_d_n9;
        locals.var_t0_dn10 = assign12870_e17795_d_n10;
        locals.var_t0_dn11 = assign12870_e17795_d_n11;
        locals.var_t0_dn12 = assign12870_e17795_d_n12;
        locals.var_t0_dn13 = assign12870_e17795_d_n13;
        locals.var_t0_dn14 = assign12870_e17795_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign12880_e17799,) = {
    if (locals.var_guard448 != 0.0) {
        (300.15,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign12880_e17799;
        locals.var_tnom_rv = 0.0;

        let (assign12890_e17806,) = {
    if (locals.var_guard448 == 0.0) {
        let assign12890_e17804: f64 = (p.p820 + 273.15);
        (assign12890_e17804,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign12890_e17806;
        locals.var_tnom_rv = 0.0;

        let assign12900_e17807: f64 = ctx_temp;
        let assign12900_e17809: f64 = (assign12900_e17807 + p.p33);
        locals.var_devtemp = assign12900_e17809;
        locals.var_devtemp_dn4 = 0.0;
        locals.var_devtemp_rv = 0.0;

        let assign12910_e17820: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard449 = assign12910_e17820;
        locals.var_guard449_rv = 0.0;

        let (assign12920_e17824, assign12920_e17824_d_n4,) = {
    if (locals.var_guard449 != 0.0) {
        ((nv4 - 0.0), 1.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4,)
    }
};
        locals.var_deltemp1 = assign12920_e17824;
        locals.var_deltemp1_dn4 = assign12920_e17824_d_n4;
        locals.var_deltemp1_rv = 0.0;

        let (assign12930_e17829, assign12930_e17829_d_n4,) = {
    if (locals.var_guard449 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4,)
    }
};
        locals.var_deltemp1 = assign12930_e17829;
        locals.var_deltemp1_dn4 = assign12930_e17829_d_n4;
        locals.var_deltemp1_rv = 0.0;

        let assign12940_e17832: f64 = (locals.var_deltemp1 + locals.var_devtemp);
        locals.var_devtemp = assign12940_e17832;
        locals.var_devtemp_dn4 = (locals.var_deltemp1_dn4 + locals.var_devtemp_dn4);
        locals.var_devtemp_rv = 0.0;

        let assign12980_e17840: f64 = (8.617087e-5 * locals.var_devtemp);
        locals.var_vt = assign12980_e17840;
        locals.var_vt_dn4 = (8.617087e-5 * locals.var_devtemp_dn4);
        locals.var_vt_rv = 0.0;

        let assign12990_e17843: f64 = (1.0 / locals.var_vt);
        locals.var_inv_vt = assign12990_e17843;
        locals.var_inv_vt_dn4 = (-(locals.var_vt_dn4 / (locals.var_vt * locals.var_vt)));
        locals.var_inv_vt_rv = 0.0;

        let assign13000_e17846: f64 = (locals.var_devtemp / locals.var_tnom);
        locals.var_tratio = assign13000_e17846;
        locals.var_tratio_dn4 = (locals.var_devtemp_dn4 / locals.var_tnom);
        locals.var_tratio_rv = 0.0;

        let assign13010_e17849: f64 = (locals.var_devtemp - locals.var_tnom);
        locals.var_deltemp = assign13010_e17849;
        locals.var_deltemp_dn4 = locals.var_devtemp_dn4;
        locals.var_deltemp_rv = 0.0;

        let assign13020_e17852: f64 = (8.617087e-5 * locals.var_devtemp);
        locals.var_vtm = assign13020_e17852;
        locals.var_vtm_dn4 = (8.617087e-5 * locals.var_devtemp_dn4);
        locals.var_vtm_rv = 0.0;

        let assign13030_e17855: f64 = (8.617087e-5 * locals.var_tnom);
        locals.var_vtm0 = assign13030_e17855;
        locals.var_vtm0_rv = 0.0;

        let assign13040_e17859: f64 = (p.p821 * locals.var_devtemp);
        let assign13040_e17861: f64 = (assign13040_e17859 * locals.var_devtemp);
        let assign13040_e17864: f64 = (locals.var_devtemp + p.p822);
        let assign13040_e17865: f64 = (assign13040_e17861 / assign13040_e17864);
        let assign13040_e17866: f64 = (p.p109 - assign13040_e17865);
        locals.var_eg = assign13040_e17866;
        locals.var_eg_dn4 = (-((((((p.p821 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign13040_e17859 * locals.var_devtemp_dn4)) * assign13040_e17864) - (assign13040_e17861 * locals.var_devtemp_dn4)) / (assign13040_e17864 * assign13040_e17864)));
        locals.var_eg_rv = 0.0;

        let assign13050_e17870: f64 = (p.p821 * locals.var_tnom);
        let assign13050_e17872: f64 = (assign13050_e17870 * locals.var_tnom);
        let assign13050_e17875: f64 = (locals.var_tnom + p.p822);
        let assign13050_e17876: f64 = (assign13050_e17872 / assign13050_e17875);
        let assign13050_e17877: f64 = (p.p109 - assign13050_e17876);
        locals.var_eg0 = assign13050_e17877;
        locals.var_eg0_rv = 0.0;

        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tnom;
        let assign13060_e17880: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign13060_e17883: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign13060_e17884: f64 = (assign13060_e17883).sqrt();
        let assign13060_e17885: f64 = (assign13060_e17880 * assign13060_e17884);
        locals.var_t1 = assign13060_e17885;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = (((locals.var_devtemp_dn4 / locals.var_tnom) * assign13060_e17884) + (assign13060_e17880 * ((locals.var_devtemp_dn4 / locals.var_tnom) / (2.0 * assign13060_e17884))));
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign13070_e17888: f64 = (p.p108 * locals.var_t1);
        let assign13070_e17892: f64 = (2.0 * locals.var_vtm0);
        let assign13070_e17893: f64 = (locals.var_eg / assign13070_e17892);
        let assign13070_e17897: f64 = (2.0 * locals.var_vtm);
        let assign13070_e17898: f64 = (locals.var_eg / assign13070_e17897);
        let assign13070_e17899: f64 = (assign13070_e17893 - assign13070_e17898);
        let assign13070_e17900: f64 = { let limited_exp_arg = assign13070_e17899; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13070_e17901: f64 = (assign13070_e17888 * assign13070_e17900);
        locals.var_ni = assign13070_e17901;
        locals.var_ni_dn0 = ((p.p108 * locals.var_t1_dn0) * assign13070_e17900);
        locals.var_ni_dn2 = ((p.p108 * locals.var_t1_dn2) * assign13070_e17900);
        locals.var_ni_dn3 = ((p.p108 * locals.var_t1_dn3) * assign13070_e17900);
        locals.var_ni_dn4 = (((p.p108 * locals.var_t1_dn4) * assign13070_e17900) + (assign13070_e17888 * ({ let limited_exp_arg = assign13070_e17899; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_eg_dn4 / assign13070_e17892) - (((locals.var_eg_dn4 * assign13070_e17897) - (locals.var_eg * (2.0 * locals.var_vtm_dn4))) / (assign13070_e17897 * assign13070_e17897))))));
        locals.var_ni_dn5 = ((p.p108 * locals.var_t1_dn5) * assign13070_e17900);
        locals.var_ni_dn6 = ((p.p108 * locals.var_t1_dn6) * assign13070_e17900);
        locals.var_ni_dn7 = ((p.p108 * locals.var_t1_dn7) * assign13070_e17900);
        locals.var_ni_dn8 = ((p.p108 * locals.var_t1_dn8) * assign13070_e17900);
        locals.var_ni_dn9 = ((p.p108 * locals.var_t1_dn9) * assign13070_e17900);
        locals.var_ni_dn10 = ((p.p108 * locals.var_t1_dn10) * assign13070_e17900);
        locals.var_ni_dn11 = ((p.p108 * locals.var_t1_dn11) * assign13070_e17900);
        locals.var_ni_dn12 = ((p.p108 * locals.var_t1_dn12) * assign13070_e17900);
        locals.var_ni_dn13 = ((p.p108 * locals.var_t1_dn13) * assign13070_e17900);
        locals.var_ni_dn14 = ((p.p108 * locals.var_t1_dn14) * assign13070_e17900);
        locals.var_ni_rv = 0.0;

        let assign13080_e17912: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard450 = assign13080_e17912;
        locals.var_guard450_rv = 0.0;

        let (assign13090_e17921, assign13090_e17921_d_n0, assign13090_e17921_d_n2, assign13090_e17921_d_n3, assign13090_e17921_d_n4, assign13090_e17921_d_n5, assign13090_e17921_d_n6, assign13090_e17921_d_n7, assign13090_e17921_d_n8, assign13090_e17921_d_n9, assign13090_e17921_d_n10, assign13090_e17921_d_n11, assign13090_e17921_d_n12, assign13090_e17921_d_n13, assign13090_e17921_d_n14,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13090_e17916: f64 = (locals.var_ndep_i / locals.var_ni);
        let assign13090_e17918: f64 = (assign13090_e17916).max(1e-38);
        let assign13090_e17919: f64 = (assign13090_e17918).ln();
        (assign13090_e17919, (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn0 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn0)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn2 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn2)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn3 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn4 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn5 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn6 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn7 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn8 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn9 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn10 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn11 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn12 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn12)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn13 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn13)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn14 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn14)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13090_e17921;
        locals.var_t0_dn0 = assign13090_e17921_d_n0;
        locals.var_t0_dn2 = assign13090_e17921_d_n2;
        locals.var_t0_dn3 = assign13090_e17921_d_n3;
        locals.var_t0_dn4 = assign13090_e17921_d_n4;
        locals.var_t0_dn5 = assign13090_e17921_d_n5;
        locals.var_t0_dn6 = assign13090_e17921_d_n6;
        locals.var_t0_dn7 = assign13090_e17921_d_n7;
        locals.var_t0_dn8 = assign13090_e17921_d_n8;
        locals.var_t0_dn9 = assign13090_e17921_d_n9;
        locals.var_t0_dn10 = assign13090_e17921_d_n10;
        locals.var_t0_dn11 = assign13090_e17921_d_n11;
        locals.var_t0_dn12 = assign13090_e17921_d_n12;
        locals.var_t0_dn13 = assign13090_e17921_d_n13;
        locals.var_t0_dn14 = assign13090_e17921_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign13100_e17930, assign13100_e17930_d_n0, assign13100_e17930_d_n2, assign13100_e17930_d_n3, assign13100_e17930_d_n4, assign13100_e17930_d_n5, assign13100_e17930_d_n6, assign13100_e17930_d_n7, assign13100_e17930_d_n8, assign13100_e17930_d_n9, assign13100_e17930_d_n10, assign13100_e17930_d_n11, assign13100_e17930_d_n12, assign13100_e17930_d_n13, assign13100_e17930_d_n14,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13100_e17925: f64 = (locals.var_t0 * locals.var_t0);
        let assign13100_e17927: f64 = (assign13100_e17925 + 1e-6);
        let assign13100_e17928: f64 = (assign13100_e17927).sqrt();
        (assign13100_e17928, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign13100_e17928)),)
    } else {
        (locals.var_phib, locals.var_phib_dn0, locals.var_phib_dn2, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11, locals.var_phib_dn12, locals.var_phib_dn13, locals.var_phib_dn14,)
    }
};
        locals.var_phib = assign13100_e17930;
        locals.var_phib_dn0 = assign13100_e17930_d_n0;
        locals.var_phib_dn2 = assign13100_e17930_d_n2;
        locals.var_phib_dn3 = assign13100_e17930_d_n3;
        locals.var_phib_dn4 = assign13100_e17930_d_n4;
        locals.var_phib_dn5 = assign13100_e17930_d_n5;
        locals.var_phib_dn6 = assign13100_e17930_d_n6;
        locals.var_phib_dn7 = assign13100_e17930_d_n7;
        locals.var_phib_dn8 = assign13100_e17930_d_n8;
        locals.var_phib_dn9 = assign13100_e17930_d_n9;
        locals.var_phib_dn10 = assign13100_e17930_d_n10;
        locals.var_phib_dn11 = assign13100_e17930_d_n11;
        locals.var_phib_dn12 = assign13100_e17930_d_n12;
        locals.var_phib_dn13 = assign13100_e17930_d_n13;
        locals.var_phib_dn14 = assign13100_e17930_d_n14;
        locals.var_phib_rv = 0.0;

        let (assign13110_e17940, assign13110_e17940_d_n0, assign13110_e17940_d_n2, assign13110_e17940_d_n3, assign13110_e17940_d_n4, assign13110_e17940_d_n5, assign13110_e17940_d_n6, assign13110_e17940_d_n7, assign13110_e17940_d_n8, assign13110_e17940_d_n9, assign13110_e17940_d_n10, assign13110_e17940_d_n11, assign13110_e17940_d_n12, assign13110_e17940_d_n13, assign13110_e17940_d_n14,) = {
    if (locals.var_guard450 == 0.0) {
        let assign13110_e17935: f64 = (locals.var_ndep_i / locals.var_ni);
        let assign13110_e17937: f64 = (assign13110_e17935).max(1e-38);
        let assign13110_e17938: f64 = (assign13110_e17937).ln();
        (assign13110_e17938, (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn0 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn0)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn2 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn2)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn3 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn4 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn5 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn6 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn7 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn8 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn9 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn10 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn11 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn12 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn12)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn13 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn13)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn14 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn14)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937),)
    } else {
        (locals.var_phib, locals.var_phib_dn0, locals.var_phib_dn2, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11, locals.var_phib_dn12, locals.var_phib_dn13, locals.var_phib_dn14,)
    }
};
        locals.var_phib = assign13110_e17940;
        locals.var_phib_dn0 = assign13110_e17940_d_n0;
        locals.var_phib_dn2 = assign13110_e17940_d_n2;
        locals.var_phib_dn3 = assign13110_e17940_d_n3;
        locals.var_phib_dn4 = assign13110_e17940_d_n4;
        locals.var_phib_dn5 = assign13110_e17940_d_n5;
        locals.var_phib_dn6 = assign13110_e17940_d_n6;
        locals.var_phib_dn7 = assign13110_e17940_d_n7;
        locals.var_phib_dn8 = assign13110_e17940_d_n8;
        locals.var_phib_dn9 = assign13110_e17940_d_n9;
        locals.var_phib_dn10 = assign13110_e17940_d_n10;
        locals.var_phib_dn11 = assign13110_e17940_d_n11;
        locals.var_phib_dn12 = assign13110_e17940_d_n12;
        locals.var_phib_dn13 = assign13110_e17940_d_n13;
        locals.var_phib_dn14 = assign13110_e17940_d_n14;
        locals.var_phib_rv = 0.0;

        let assign13120_e17951: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard451 = assign13120_e17951;
        locals.var_guard451_rv = 0.0;

        let (assign13130_e17964, assign13130_e17964_d_n0, assign13130_e17964_d_n2, assign13130_e17964_d_n3, assign13130_e17964_d_n4, assign13130_e17964_d_n5, assign13130_e17964_d_n6, assign13130_e17964_d_n7, assign13130_e17964_d_n8, assign13130_e17964_d_n9, assign13130_e17964_d_n10, assign13130_e17964_d_n11, assign13130_e17964_d_n12, assign13130_e17964_d_n13, assign13130_e17964_d_n14,) = {
    if (locals.var_guard451 != 0.0) {
        let assign13130_e17955: f64 = (locals.var_ndepedge_i * locals.var_nsd_i);
        let assign13130_e17958: f64 = (locals.var_ni * locals.var_ni);
        let assign13130_e17959: f64 = (assign13130_e17955 / assign13130_e17958);
        let assign13130_e17961: f64 = (assign13130_e17959).max(1e-38);
        let assign13130_e17962: f64 = (assign13130_e17961).ln();
        (assign13130_e17962, (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn0 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn0))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn2 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn2))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn12 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn12))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn13 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn13))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn14 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn14))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13130_e17964;
        locals.var_t0_dn0 = assign13130_e17964_d_n0;
        locals.var_t0_dn2 = assign13130_e17964_d_n2;
        locals.var_t0_dn3 = assign13130_e17964_d_n3;
        locals.var_t0_dn4 = assign13130_e17964_d_n4;
        locals.var_t0_dn5 = assign13130_e17964_d_n5;
        locals.var_t0_dn6 = assign13130_e17964_d_n6;
        locals.var_t0_dn7 = assign13130_e17964_d_n7;
        locals.var_t0_dn8 = assign13130_e17964_d_n8;
        locals.var_t0_dn9 = assign13130_e17964_d_n9;
        locals.var_t0_dn10 = assign13130_e17964_d_n10;
        locals.var_t0_dn11 = assign13130_e17964_d_n11;
        locals.var_t0_dn12 = assign13130_e17964_d_n12;
        locals.var_t0_dn13 = assign13130_e17964_d_n13;
        locals.var_t0_dn14 = assign13130_e17964_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign13140_e17973, assign13140_e17973_d_n0, assign13140_e17973_d_n2, assign13140_e17973_d_n3, assign13140_e17973_d_n4, assign13140_e17973_d_n5, assign13140_e17973_d_n6, assign13140_e17973_d_n7, assign13140_e17973_d_n8, assign13140_e17973_d_n9, assign13140_e17973_d_n10, assign13140_e17973_d_n11, assign13140_e17973_d_n12, assign13140_e17973_d_n13, assign13140_e17973_d_n14,) = {
    if (locals.var_guard451 != 0.0) {
        let assign13140_e17968: f64 = (locals.var_t0 * locals.var_t0);
        let assign13140_e17970: f64 = (assign13140_e17968 + 1e-6);
        let assign13140_e17971: f64 = (assign13140_e17970).sqrt();
        (assign13140_e17971, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign13140_e17971)),)
    } else {
        (locals.var_vbi_edge, locals.var_vbi_edge_dn0, locals.var_vbi_edge_dn2, locals.var_vbi_edge_dn3, locals.var_vbi_edge_dn4, locals.var_vbi_edge_dn5, locals.var_vbi_edge_dn6, locals.var_vbi_edge_dn7, locals.var_vbi_edge_dn8, locals.var_vbi_edge_dn9, locals.var_vbi_edge_dn10, locals.var_vbi_edge_dn11, locals.var_vbi_edge_dn12, locals.var_vbi_edge_dn13, locals.var_vbi_edge_dn14,)
    }
};
        locals.var_vbi_edge = assign13140_e17973;
        locals.var_vbi_edge_dn0 = assign13140_e17973_d_n0;
        locals.var_vbi_edge_dn2 = assign13140_e17973_d_n2;
        locals.var_vbi_edge_dn3 = assign13140_e17973_d_n3;
        locals.var_vbi_edge_dn4 = assign13140_e17973_d_n4;
        locals.var_vbi_edge_dn5 = assign13140_e17973_d_n5;
        locals.var_vbi_edge_dn6 = assign13140_e17973_d_n6;
        locals.var_vbi_edge_dn7 = assign13140_e17973_d_n7;
        locals.var_vbi_edge_dn8 = assign13140_e17973_d_n8;
        locals.var_vbi_edge_dn9 = assign13140_e17973_d_n9;
        locals.var_vbi_edge_dn10 = assign13140_e17973_d_n10;
        locals.var_vbi_edge_dn11 = assign13140_e17973_d_n11;
        locals.var_vbi_edge_dn12 = assign13140_e17973_d_n12;
        locals.var_vbi_edge_dn13 = assign13140_e17973_d_n13;
        locals.var_vbi_edge_dn14 = assign13140_e17973_d_n14;
        locals.var_vbi_edge_rv = 0.0;

        let (assign13150_e17987, assign13150_e17987_d_n0, assign13150_e17987_d_n2, assign13150_e17987_d_n3, assign13150_e17987_d_n4, assign13150_e17987_d_n5, assign13150_e17987_d_n6, assign13150_e17987_d_n7, assign13150_e17987_d_n8, assign13150_e17987_d_n9, assign13150_e17987_d_n10, assign13150_e17987_d_n11, assign13150_e17987_d_n12, assign13150_e17987_d_n13, assign13150_e17987_d_n14,) = {
    if (locals.var_guard451 == 0.0) {
        let assign13150_e17978: f64 = (locals.var_ndepedge_i * locals.var_nsd_i);
        let assign13150_e17981: f64 = (locals.var_ni * locals.var_ni);
        let assign13150_e17982: f64 = (assign13150_e17978 / assign13150_e17981);
        let assign13150_e17984: f64 = (assign13150_e17982).max(1e-38);
        let assign13150_e17985: f64 = (assign13150_e17984).ln();
        (assign13150_e17985, (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn0 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn0))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn2 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn2))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn12 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn12))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn13 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn13))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn14 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn14))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984),)
    } else {
        (locals.var_vbi_edge, locals.var_vbi_edge_dn0, locals.var_vbi_edge_dn2, locals.var_vbi_edge_dn3, locals.var_vbi_edge_dn4, locals.var_vbi_edge_dn5, locals.var_vbi_edge_dn6, locals.var_vbi_edge_dn7, locals.var_vbi_edge_dn8, locals.var_vbi_edge_dn9, locals.var_vbi_edge_dn10, locals.var_vbi_edge_dn11, locals.var_vbi_edge_dn12, locals.var_vbi_edge_dn13, locals.var_vbi_edge_dn14,)
    }
};
        locals.var_vbi_edge = assign13150_e17987;
        locals.var_vbi_edge_dn0 = assign13150_e17987_d_n0;
        locals.var_vbi_edge_dn2 = assign13150_e17987_d_n2;
        locals.var_vbi_edge_dn3 = assign13150_e17987_d_n3;
        locals.var_vbi_edge_dn4 = assign13150_e17987_d_n4;
        locals.var_vbi_edge_dn5 = assign13150_e17987_d_n5;
        locals.var_vbi_edge_dn6 = assign13150_e17987_d_n6;
        locals.var_vbi_edge_dn7 = assign13150_e17987_d_n7;
        locals.var_vbi_edge_dn8 = assign13150_e17987_d_n8;
        locals.var_vbi_edge_dn9 = assign13150_e17987_d_n9;
        locals.var_vbi_edge_dn10 = assign13150_e17987_d_n10;
        locals.var_vbi_edge_dn11 = assign13150_e17987_d_n11;
        locals.var_vbi_edge_dn12 = assign13150_e17987_d_n12;
        locals.var_vbi_edge_dn13 = assign13150_e17987_d_n13;
        locals.var_vbi_edge_dn14 = assign13150_e17987_d_n14;
        locals.var_vbi_edge_rv = 0.0;

        let assign13160_e17990: f64 = if locals.var_ngate_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard452 = assign13160_e17990;
        locals.var_guard452_rv = 0.0;

        let (assign13170_e18006, assign13170_e18006_d_n4,) = {
    if (locals.var_guard452 != 0.0) {
        let assign13170_e17993: f64 = (-locals.var_devsign);
        let assign13170_e17995: f64 = (assign13170_e17993 * locals.var_vt);
        let assign13170_e17998: f64 = (locals.var_ngate_i / locals.var_nsd_i);
        let assign13170_e18000: f64 = (assign13170_e17998).max(1e-38);
        let assign13170_e18001: f64 = (assign13170_e18000).ln();
        let assign13170_e18002: f64 = (assign13170_e17995 * assign13170_e18001);
        let assign13170_e18004: f64 = (assign13170_e18002 + p.p5);
        (assign13170_e18004, ((assign13170_e17993 * locals.var_vt_dn4) * assign13170_e18001),)
    } else {
        (locals.var_vfbsdr, locals.var_vfbsdr_dn4,)
    }
};
        locals.var_vfbsdr = assign13170_e18006;
        locals.var_vfbsdr_dn4 = assign13170_e18006_d_n4;
        locals.var_vfbsdr_rv = 0.0;

        let (assign13180_e18011, assign13180_e18011_d_n4,) = {
    if (locals.var_guard452 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vfbsdr, locals.var_vfbsdr_dn4,)
    }
};
        locals.var_vfbsdr = assign13180_e18011;
        locals.var_vfbsdr_dn4 = assign13180_e18011_d_n4;
        locals.var_vfbsdr_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13190_e18015: f64 = (locals.var_vt * locals.var_phib);
        let assign13190_e18016: f64 = (0.4 + assign13190_e18015);
        let assign13190_e18018: f64 = (assign13190_e18016 + locals.var_phin_i);
        let assign13190_e18020: f64 = (assign13190_e18018).max(0.4);
        locals.var_phist = assign13190_e18020;
        locals.var_phist_dn0 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn0) } else { 0.0 };
        locals.var_phist_dn2 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn2) } else { 0.0 };
        locals.var_phist_dn3 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn3) } else { 0.0 };
        locals.var_phist_dn4 = if assign13190_e18018 >= 0.4 { ((locals.var_vt_dn4 * locals.var_phib) + (locals.var_vt * locals.var_phib_dn4)) } else { 0.0 };
        locals.var_phist_dn5 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn5) } else { 0.0 };
        locals.var_phist_dn6 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn6) } else { 0.0 };
        locals.var_phist_dn7 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn7) } else { 0.0 };
        locals.var_phist_dn8 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn8) } else { 0.0 };
        locals.var_phist_dn9 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn9) } else { 0.0 };
        locals.var_phist_dn10 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn10) } else { 0.0 };
        locals.var_phist_dn11 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn11) } else { 0.0 };
        locals.var_phist_dn12 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn12) } else { 0.0 };
        locals.var_phist_dn13 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn13) } else { 0.0 };
        locals.var_phist_dn14 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn14) } else { 0.0 };
        locals.var_phist_rv = 0.0;

        let assign13200_e18022: f64 = (locals.var_phist).sqrt();
        locals.var_sqrtphist = assign13200_e18022;
        locals.var_sqrtphist_dn0 = (locals.var_phist_dn0 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn2 = (locals.var_phist_dn2 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn3 = (locals.var_phist_dn3 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn4 = (locals.var_phist_dn4 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn5 = (locals.var_phist_dn5 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn6 = (locals.var_phist_dn6 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn7 = (locals.var_phist_dn7 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn8 = (locals.var_phist_dn8 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn9 = (locals.var_phist_dn9 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn10 = (locals.var_phist_dn10 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn11 = (locals.var_phist_dn11 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn12 = (locals.var_phist_dn12 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn13 = (locals.var_phist_dn13 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn14 = (locals.var_phist_dn14 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_rv = 0.0;

        let assign13210_e18025: f64 = (2.0 * locals.var_epssi);
        let assign13210_e18028: f64 = (1.60219e-19 * locals.var_ndep_i);
        let assign13210_e18029: f64 = (assign13210_e18025 / assign13210_e18028);
        let assign13210_e18030: f64 = (assign13210_e18029).sqrt();
        locals.var_t1dep = assign13210_e18030;
        locals.var_t1dep_dn0 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn0)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn2 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn2)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn3 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn3)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn4 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn4)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn5 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn5)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn6 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn6)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn7 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn7)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn8 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn8)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn9 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn9)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn10 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn10)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn11 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn11)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn12 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn12)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn13 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn13)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn14 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn14)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_rv = 0.0;

        let assign13220_e18033: f64 = (locals.var_epssi / locals.var_epsox);
        let assign13220_e18035: f64 = (assign13220_e18033 * p.p77);
        let assign13220_e18037: f64 = (assign13220_e18035 * locals.var_xj_i);
        let assign13220_e18038: f64 = (assign13220_e18037).sqrt();
        locals.var_litl = assign13220_e18038;
        locals.var_litl_rv = 0.0;

        let assign13230_e18044: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18045: f64 = (p.p823 * assign13230_e18044);
        let assign13230_e18046: f64 = (1.0 + assign13230_e18045);
        let assign13230_e18048: f64 = (-10000.0);
        let assign13230_e18050: f64 = (assign13230_e18048 * 0.001);
        let (assign13230_e18111, assign13230_e18111_d_n4,) = {
    if (!(assign13230_e18046 < assign13230_e18050)) {
        let assign13230_e18058: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18059: f64 = (p.p823 * assign13230_e18058);
        let assign13230_e18060: f64 = (1.0 + assign13230_e18059);
        let assign13230_e18065: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18066: f64 = (p.p823 * assign13230_e18065);
        let assign13230_e18067: f64 = (1.0 + assign13230_e18066);
        let assign13230_e18072: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18073: f64 = (p.p823 * assign13230_e18072);
        let assign13230_e18074: f64 = (1.0 + assign13230_e18073);
        let assign13230_e18075: f64 = (assign13230_e18067 * assign13230_e18074);
        let assign13230_e18078: f64 = (4.0 * 0.001);
        let assign13230_e18080: f64 = (assign13230_e18078 * 0.001);
        let assign13230_e18081: f64 = (assign13230_e18075 + assign13230_e18080);
        let assign13230_e18082: f64 = (assign13230_e18081).sqrt();
        let assign13230_e18083: f64 = (assign13230_e18060 + assign13230_e18082);
        let assign13230_e18084: f64 = (0.5 * assign13230_e18083);
        (assign13230_e18084, (0.5 * ((p.p823 * locals.var_tratio_dn4) + ((((p.p823 * locals.var_tratio_dn4) * assign13230_e18074) + (assign13230_e18067 * (p.p823 * locals.var_tratio_dn4))) / (2.0 * assign13230_e18082)))),)
    } else {
        let assign13230_e18089: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18090: f64 = (p.p823 * assign13230_e18089);
        let assign13230_e18091: f64 = (1.0 + assign13230_e18090);
        let assign13230_e18093: f64 = (-10000.0);
        let assign13230_e18095: f64 = (assign13230_e18093 * 0.001);
        let (assign13230_e18110, assign13230_e18110_d_n4,) = {
            if (assign13230_e18091 < assign13230_e18095) {
                let assign13230_e18098: f64 = (-0.001);
                let assign13230_e18100: f64 = (assign13230_e18098 * 0.001);
                let assign13230_e18105: f64 = (locals.var_tratio - 1.0);
                let assign13230_e18106: f64 = (p.p823 * assign13230_e18105);
                let assign13230_e18107: f64 = (1.0 + assign13230_e18106);
                let assign13230_e18108: f64 = (assign13230_e18100 / assign13230_e18107);
                (assign13230_e18108, (-((assign13230_e18100 * (p.p823 * locals.var_tratio_dn4)) / (assign13230_e18107 * assign13230_e18107))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13230_e18110, assign13230_e18110_d_n4,)
    }
};
        let assign13230_e18112: f64 = (locals.var_nfactor_i * assign13230_e18111);
        locals.var_nfactor_t = assign13230_e18112;
        locals.var_nfactor_t_dn0 = (locals.var_nfactor_i_dn0 * assign13230_e18111);
        locals.var_nfactor_t_dn2 = (locals.var_nfactor_i_dn2 * assign13230_e18111);
        locals.var_nfactor_t_dn3 = (locals.var_nfactor_i_dn3 * assign13230_e18111);
        locals.var_nfactor_t_dn4 = ((locals.var_nfactor_i_dn4 * assign13230_e18111) + (locals.var_nfactor_i * assign13230_e18111_d_n4));
        locals.var_nfactor_t_dn5 = (locals.var_nfactor_i_dn5 * assign13230_e18111);
        locals.var_nfactor_t_dn6 = (locals.var_nfactor_i_dn6 * assign13230_e18111);
        locals.var_nfactor_t_dn7 = (locals.var_nfactor_i_dn7 * assign13230_e18111);
        locals.var_nfactor_t_dn8 = (locals.var_nfactor_i_dn8 * assign13230_e18111);
        locals.var_nfactor_t_dn9 = (locals.var_nfactor_i_dn9 * assign13230_e18111);
        locals.var_nfactor_t_dn10 = (locals.var_nfactor_i_dn10 * assign13230_e18111);
        locals.var_nfactor_t_dn11 = (locals.var_nfactor_i_dn11 * assign13230_e18111);
        locals.var_nfactor_t_dn12 = (locals.var_nfactor_i_dn12 * assign13230_e18111);
        locals.var_nfactor_t_dn13 = (locals.var_nfactor_i_dn13 * assign13230_e18111);
        locals.var_nfactor_t_dn14 = (locals.var_nfactor_i_dn14 * assign13230_e18111);
        locals.var_nfactor_t_rv = 0.0;

        let assign13240_e18118: f64 = (locals.var_tratio - 1.0);
        let assign13240_e18119: f64 = (p.p851 * assign13240_e18118);
        let assign13240_e18120: f64 = (1.0 + assign13240_e18119);
        let assign13240_e18121: f64 = (locals.var_eta0_i * assign13240_e18120);
        locals.var_eta0_t = assign13240_e18121;
        locals.var_eta0_t_dn0 = (locals.var_eta0_i_dn0 * assign13240_e18120);
        locals.var_eta0_t_dn2 = (locals.var_eta0_i_dn2 * assign13240_e18120);
        locals.var_eta0_t_dn3 = (locals.var_eta0_i_dn3 * assign13240_e18120);
        locals.var_eta0_t_dn4 = ((locals.var_eta0_i_dn4 * assign13240_e18120) + (locals.var_eta0_i * (p.p851 * locals.var_tratio_dn4)));
        locals.var_eta0_t_dn5 = (locals.var_eta0_i_dn5 * assign13240_e18120);
        locals.var_eta0_t_dn6 = (locals.var_eta0_i_dn6 * assign13240_e18120);
        locals.var_eta0_t_dn7 = (locals.var_eta0_i_dn7 * assign13240_e18120);
        locals.var_eta0_t_dn8 = (locals.var_eta0_i_dn8 * assign13240_e18120);
        locals.var_eta0_t_dn9 = (locals.var_eta0_i_dn9 * assign13240_e18120);
        locals.var_eta0_t_dn10 = (locals.var_eta0_i_dn10 * assign13240_e18120);
        locals.var_eta0_t_dn11 = (locals.var_eta0_i_dn11 * assign13240_e18120);
        locals.var_eta0_t_dn12 = (locals.var_eta0_i_dn12 * assign13240_e18120);
        locals.var_eta0_t_dn13 = (locals.var_eta0_i_dn13 * assign13240_e18120);
        locals.var_eta0_t_dn14 = (locals.var_eta0_i_dn14 * assign13240_e18120);
        locals.var_eta0_t_rv = 0.0;

        let assign13250_e18124: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard453 = assign13250_e18124;
        locals.var_guard453_rv = 0.0;

        let (assign13260_e18136, assign13260_e18136_d_n0, assign13260_e18136_d_n2, assign13260_e18136_d_n3, assign13260_e18136_d_n4, assign13260_e18136_d_n5, assign13260_e18136_d_n6, assign13260_e18136_d_n7, assign13260_e18136_d_n8, assign13260_e18136_d_n9, assign13260_e18136_d_n10, assign13260_e18136_d_n11, assign13260_e18136_d_n12, assign13260_e18136_d_n13, assign13260_e18136_d_n14,) = {
    if (locals.var_guard453 != 0.0) {
        let assign13260_e18131: f64 = (locals.var_tratio - 1.0);
        let assign13260_e18132: f64 = (p.p851 * assign13260_e18131);
        let assign13260_e18133: f64 = (1.0 + assign13260_e18132);
        let assign13260_e18134: f64 = (locals.var_eta0r_i * assign13260_e18133);
        (assign13260_e18134, (locals.var_eta0r_i_dn0 * assign13260_e18133), (locals.var_eta0r_i_dn2 * assign13260_e18133), (locals.var_eta0r_i_dn3 * assign13260_e18133), ((locals.var_eta0r_i_dn4 * assign13260_e18133) + (locals.var_eta0r_i * (p.p851 * locals.var_tratio_dn4))), (locals.var_eta0r_i_dn5 * assign13260_e18133), (locals.var_eta0r_i_dn6 * assign13260_e18133), (locals.var_eta0r_i_dn7 * assign13260_e18133), (locals.var_eta0r_i_dn8 * assign13260_e18133), (locals.var_eta0r_i_dn9 * assign13260_e18133), (locals.var_eta0r_i_dn10 * assign13260_e18133), (locals.var_eta0r_i_dn11 * assign13260_e18133), (locals.var_eta0r_i_dn12 * assign13260_e18133), (locals.var_eta0r_i_dn13 * assign13260_e18133), (locals.var_eta0r_i_dn14 * assign13260_e18133),)
    } else {
        (locals.var_eta0r_t, locals.var_eta0r_t_dn0, locals.var_eta0r_t_dn2, locals.var_eta0r_t_dn3, locals.var_eta0r_t_dn4, locals.var_eta0r_t_dn5, locals.var_eta0r_t_dn6, locals.var_eta0r_t_dn7, locals.var_eta0r_t_dn8, locals.var_eta0r_t_dn9, locals.var_eta0r_t_dn10, locals.var_eta0r_t_dn11, locals.var_eta0r_t_dn12, locals.var_eta0r_t_dn13, locals.var_eta0r_t_dn14,)
    }
};
        locals.var_eta0r_t = assign13260_e18136;
        locals.var_eta0r_t_dn0 = assign13260_e18136_d_n0;
        locals.var_eta0r_t_dn2 = assign13260_e18136_d_n2;
        locals.var_eta0r_t_dn3 = assign13260_e18136_d_n3;
        locals.var_eta0r_t_dn4 = assign13260_e18136_d_n4;
        locals.var_eta0r_t_dn5 = assign13260_e18136_d_n5;
        locals.var_eta0r_t_dn6 = assign13260_e18136_d_n6;
        locals.var_eta0r_t_dn7 = assign13260_e18136_d_n7;
        locals.var_eta0r_t_dn8 = assign13260_e18136_d_n8;
        locals.var_eta0r_t_dn9 = assign13260_e18136_d_n9;
        locals.var_eta0r_t_dn10 = assign13260_e18136_d_n10;
        locals.var_eta0r_t_dn11 = assign13260_e18136_d_n11;
        locals.var_eta0r_t_dn12 = assign13260_e18136_d_n12;
        locals.var_eta0r_t_dn13 = assign13260_e18136_d_n13;
        locals.var_eta0r_t_dn14 = assign13260_e18136_d_n14;
        locals.var_eta0r_t_rv = 0.0;

        let (assign13270_e18146,) = {
    if (p.p39 != 1.0) {
        let assign13270_e18142: f64 = (0.3333333333333333 * p.p283);
        (assign13270_e18142,)
    } else {
        let assign13270_e18145: f64 = (0.5 * p.p283);
        (assign13270_e18145,)
    }
};
        locals.var_eta_mu = assign13270_e18146;
        locals.var_eta_mu_rv = 0.0;

        let assign13280_e18150: f64 = (locals.var_tratio).powf(locals.var_ute_i);
        let assign13280_e18151: f64 = (locals.var_u0_i * assign13280_e18150);
        locals.var_u0_t = assign13280_e18151;
        locals.var_u0_t_dn0 = 0.0;
        locals.var_u0_t_dn2 = 0.0;
        locals.var_u0_t_dn3 = 0.0;
        locals.var_u0_t_dn4 = (locals.var_u0_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13280_e18150 * (locals.var_ute_i * (locals.var_tratio_dn4 / locals.var_tratio))) });
        locals.var_u0_t_dn5 = 0.0;
        locals.var_u0_t_dn6 = 0.0;
        locals.var_u0_t_dn7 = 0.0;
        locals.var_u0_t_dn8 = 0.0;
        locals.var_u0_t_dn9 = 0.0;
        locals.var_u0_t_dn10 = 0.0;
        locals.var_u0_t_dn11 = 0.0;
        locals.var_u0_t_dn12 = 0.0;
        locals.var_u0_t_dn13 = 0.0;
        locals.var_u0_t_dn14 = 0.0;
        locals.var_u0_t_rv = 0.0;

        let assign13290_e18156: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18157: f64 = (1.0 + assign13290_e18156);
        let assign13290_e18159: f64 = (assign13290_e18157 - 1e-6);
        let assign13290_e18161: f64 = (-10000.0);
        let assign13290_e18163: f64 = (assign13290_e18161 * 0.001);
        let (assign13290_e18224, assign13290_e18224_d_n4,) = {
    if (!(assign13290_e18159 < assign13290_e18163)) {
        let assign13290_e18170: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18171: f64 = (1.0 + assign13290_e18170);
        let assign13290_e18173: f64 = (assign13290_e18171 - 1e-6);
        let assign13290_e18177: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18178: f64 = (1.0 + assign13290_e18177);
        let assign13290_e18180: f64 = (assign13290_e18178 - 1e-6);
        let assign13290_e18184: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18185: f64 = (1.0 + assign13290_e18184);
        let assign13290_e18187: f64 = (assign13290_e18185 - 1e-6);
        let assign13290_e18188: f64 = (assign13290_e18180 * assign13290_e18187);
        let assign13290_e18191: f64 = (4.0 * 0.001);
        let assign13290_e18193: f64 = (assign13290_e18191 * 0.001);
        let assign13290_e18194: f64 = (assign13290_e18188 + assign13290_e18193);
        let assign13290_e18195: f64 = (assign13290_e18194).sqrt();
        let assign13290_e18196: f64 = (assign13290_e18173 + assign13290_e18195);
        let assign13290_e18197: f64 = (0.5 * assign13290_e18196);
        (assign13290_e18197, (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn4) + ((((locals.var_ua1_i * locals.var_deltemp_dn4) * assign13290_e18187) + (assign13290_e18180 * (locals.var_ua1_i * locals.var_deltemp_dn4))) / (2.0 * assign13290_e18195)))),)
    } else {
        let assign13290_e18201: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18202: f64 = (1.0 + assign13290_e18201);
        let assign13290_e18204: f64 = (assign13290_e18202 - 1e-6);
        let assign13290_e18206: f64 = (-10000.0);
        let assign13290_e18208: f64 = (assign13290_e18206 * 0.001);
        let (assign13290_e18223, assign13290_e18223_d_n4,) = {
            if (assign13290_e18204 < assign13290_e18208) {
                let assign13290_e18211: f64 = (-0.001);
                let assign13290_e18213: f64 = (assign13290_e18211 * 0.001);
                let assign13290_e18217: f64 = (locals.var_ua1_i * locals.var_deltemp);
                let assign13290_e18218: f64 = (1.0 + assign13290_e18217);
                let assign13290_e18220: f64 = (assign13290_e18218 - 1e-6);
                let assign13290_e18221: f64 = (assign13290_e18213 / assign13290_e18220);
                (assign13290_e18221, (-((assign13290_e18213 * (locals.var_ua1_i * locals.var_deltemp_dn4)) / (assign13290_e18220 * assign13290_e18220))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13290_e18223, assign13290_e18223_d_n4,)
    }
};
        let assign13290_e18225: f64 = (locals.var_ua_i * assign13290_e18224);
        locals.var_ua_t = assign13290_e18225;
        locals.var_ua_t_dn0 = (locals.var_ua_i_dn0 * assign13290_e18224);
        locals.var_ua_t_dn2 = (locals.var_ua_i_dn2 * assign13290_e18224);
        locals.var_ua_t_dn3 = (locals.var_ua_i_dn3 * assign13290_e18224);
        locals.var_ua_t_dn4 = ((locals.var_ua_i_dn4 * assign13290_e18224) + (locals.var_ua_i * assign13290_e18224_d_n4));
        locals.var_ua_t_dn5 = (locals.var_ua_i_dn5 * assign13290_e18224);
        locals.var_ua_t_dn6 = (locals.var_ua_i_dn6 * assign13290_e18224);
        locals.var_ua_t_dn7 = (locals.var_ua_i_dn7 * assign13290_e18224);
        locals.var_ua_t_dn8 = (locals.var_ua_i_dn8 * assign13290_e18224);
        locals.var_ua_t_dn9 = (locals.var_ua_i_dn9 * assign13290_e18224);
        locals.var_ua_t_dn10 = (locals.var_ua_i_dn10 * assign13290_e18224);
        locals.var_ua_t_dn11 = (locals.var_ua_i_dn11 * assign13290_e18224);
        locals.var_ua_t_dn12 = (locals.var_ua_i_dn12 * assign13290_e18224);
        locals.var_ua_t_dn13 = (locals.var_ua_i_dn13 * assign13290_e18224);
        locals.var_ua_t_dn14 = (locals.var_ua_i_dn14 * assign13290_e18224);
        locals.var_ua_t_rv = 0.0;

        let assign13300_e18230: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18231: f64 = (1.0 + assign13300_e18230);
        let assign13300_e18233: f64 = (assign13300_e18231 - 1e-6);
        let assign13300_e18235: f64 = (-10000.0);
        let assign13300_e18237: f64 = (assign13300_e18235 * 0.001);
        let (assign13300_e18298, assign13300_e18298_d_n4,) = {
    if (!(assign13300_e18233 < assign13300_e18237)) {
        let assign13300_e18244: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18245: f64 = (1.0 + assign13300_e18244);
        let assign13300_e18247: f64 = (assign13300_e18245 - 1e-6);
        let assign13300_e18251: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18252: f64 = (1.0 + assign13300_e18251);
        let assign13300_e18254: f64 = (assign13300_e18252 - 1e-6);
        let assign13300_e18258: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18259: f64 = (1.0 + assign13300_e18258);
        let assign13300_e18261: f64 = (assign13300_e18259 - 1e-6);
        let assign13300_e18262: f64 = (assign13300_e18254 * assign13300_e18261);
        let assign13300_e18265: f64 = (4.0 * 0.001);
        let assign13300_e18267: f64 = (assign13300_e18265 * 0.001);
        let assign13300_e18268: f64 = (assign13300_e18262 + assign13300_e18267);
        let assign13300_e18269: f64 = (assign13300_e18268).sqrt();
        let assign13300_e18270: f64 = (assign13300_e18247 + assign13300_e18269);
        let assign13300_e18271: f64 = (0.5 * assign13300_e18270);
        (assign13300_e18271, (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn4) + ((((locals.var_uc1_i * locals.var_deltemp_dn4) * assign13300_e18261) + (assign13300_e18254 * (locals.var_uc1_i * locals.var_deltemp_dn4))) / (2.0 * assign13300_e18269)))),)
    } else {
        let assign13300_e18275: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18276: f64 = (1.0 + assign13300_e18275);
        let assign13300_e18278: f64 = (assign13300_e18276 - 1e-6);
        let assign13300_e18280: f64 = (-10000.0);
        let assign13300_e18282: f64 = (assign13300_e18280 * 0.001);
        let (assign13300_e18297, assign13300_e18297_d_n4,) = {
            if (assign13300_e18278 < assign13300_e18282) {
                let assign13300_e18285: f64 = (-0.001);
                let assign13300_e18287: f64 = (assign13300_e18285 * 0.001);
                let assign13300_e18291: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign13300_e18292: f64 = (1.0 + assign13300_e18291);
                let assign13300_e18294: f64 = (assign13300_e18292 - 1e-6);
                let assign13300_e18295: f64 = (assign13300_e18287 / assign13300_e18294);
                (assign13300_e18295, (-((assign13300_e18287 * (locals.var_uc1_i * locals.var_deltemp_dn4)) / (assign13300_e18294 * assign13300_e18294))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13300_e18297, assign13300_e18297_d_n4,)
    }
};
        let assign13300_e18299: f64 = (locals.var_uc_i * assign13300_e18298);
        locals.var_uc_t = assign13300_e18299;
        locals.var_uc_t_dn0 = (locals.var_uc_i_dn0 * assign13300_e18298);
        locals.var_uc_t_dn2 = (locals.var_uc_i_dn2 * assign13300_e18298);
        locals.var_uc_t_dn3 = (locals.var_uc_i_dn3 * assign13300_e18298);
        locals.var_uc_t_dn4 = ((locals.var_uc_i_dn4 * assign13300_e18298) + (locals.var_uc_i * assign13300_e18298_d_n4));
        locals.var_uc_t_dn5 = (locals.var_uc_i_dn5 * assign13300_e18298);
        locals.var_uc_t_dn6 = (locals.var_uc_i_dn6 * assign13300_e18298);
        locals.var_uc_t_dn7 = (locals.var_uc_i_dn7 * assign13300_e18298);
        locals.var_uc_t_dn8 = (locals.var_uc_i_dn8 * assign13300_e18298);
        locals.var_uc_t_dn9 = (locals.var_uc_i_dn9 * assign13300_e18298);
        locals.var_uc_t_dn10 = (locals.var_uc_i_dn10 * assign13300_e18298);
        locals.var_uc_t_dn11 = (locals.var_uc_i_dn11 * assign13300_e18298);
        locals.var_uc_t_dn12 = (locals.var_uc_i_dn12 * assign13300_e18298);
        locals.var_uc_t_dn13 = (locals.var_uc_i_dn13 * assign13300_e18298);
        locals.var_uc_t_dn14 = (locals.var_uc_i_dn14 * assign13300_e18298);
        locals.var_uc_t_rv = 0.0;

        let assign13310_e18303: f64 = (locals.var_tratio).powf(locals.var_ud1_i);
        let assign13310_e18304: f64 = (locals.var_ud_i * assign13310_e18303);
        locals.var_ud_t = assign13310_e18304;
        locals.var_ud_t_dn0 = (locals.var_ud_i_dn0 * assign13310_e18303);
        locals.var_ud_t_dn2 = (locals.var_ud_i_dn2 * assign13310_e18303);
        locals.var_ud_t_dn3 = (locals.var_ud_i_dn3 * assign13310_e18303);
        locals.var_ud_t_dn4 = ((locals.var_ud_i_dn4 * assign13310_e18303) + (locals.var_ud_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13310_e18303 * (locals.var_ud1_i * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_ud_t_dn5 = (locals.var_ud_i_dn5 * assign13310_e18303);
        locals.var_ud_t_dn6 = (locals.var_ud_i_dn6 * assign13310_e18303);
        locals.var_ud_t_dn7 = (locals.var_ud_i_dn7 * assign13310_e18303);
        locals.var_ud_t_dn8 = (locals.var_ud_i_dn8 * assign13310_e18303);
        locals.var_ud_t_dn9 = (locals.var_ud_i_dn9 * assign13310_e18303);
        locals.var_ud_t_dn10 = (locals.var_ud_i_dn10 * assign13310_e18303);
        locals.var_ud_t_dn11 = (locals.var_ud_i_dn11 * assign13310_e18303);
        locals.var_ud_t_dn12 = (locals.var_ud_i_dn12 * assign13310_e18303);
        locals.var_ud_t_dn13 = (locals.var_ud_i_dn13 * assign13310_e18303);
        locals.var_ud_t_dn14 = (locals.var_ud_i_dn14 * assign13310_e18303);
        locals.var_ud_t_rv = 0.0;

        let assign13320_e18308: f64 = (locals.var_tratio).powf(locals.var_ucste_i);
        let assign13320_e18309: f64 = (locals.var_ucs_i * assign13320_e18308);
        locals.var_ucs_t = assign13320_e18309;
        locals.var_ucs_t_dn4 = (locals.var_ucs_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13320_e18308 * (locals.var_ucste_i * (locals.var_tratio_dn4 / locals.var_tratio))) });
        locals.var_ucs_t_rv = 0.0;

        let assign13330_e18315: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18316: f64 = (locals.var_eu1_i * assign13330_e18315);
        let assign13330_e18317: f64 = (1.0 + assign13330_e18316);
        let assign13330_e18319: f64 = (-10000.0);
        let assign13330_e18321: f64 = (assign13330_e18319 * 0.001);
        let (assign13330_e18382, assign13330_e18382_d_n4,) = {
    if (!(assign13330_e18317 < assign13330_e18321)) {
        let assign13330_e18329: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18330: f64 = (locals.var_eu1_i * assign13330_e18329);
        let assign13330_e18331: f64 = (1.0 + assign13330_e18330);
        let assign13330_e18336: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18337: f64 = (locals.var_eu1_i * assign13330_e18336);
        let assign13330_e18338: f64 = (1.0 + assign13330_e18337);
        let assign13330_e18343: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18344: f64 = (locals.var_eu1_i * assign13330_e18343);
        let assign13330_e18345: f64 = (1.0 + assign13330_e18344);
        let assign13330_e18346: f64 = (assign13330_e18338 * assign13330_e18345);
        let assign13330_e18349: f64 = (4.0 * 0.001);
        let assign13330_e18351: f64 = (assign13330_e18349 * 0.001);
        let assign13330_e18352: f64 = (assign13330_e18346 + assign13330_e18351);
        let assign13330_e18353: f64 = (assign13330_e18352).sqrt();
        let assign13330_e18354: f64 = (assign13330_e18331 + assign13330_e18353);
        let assign13330_e18355: f64 = (0.5 * assign13330_e18354);
        (assign13330_e18355, (0.5 * ((locals.var_eu1_i * locals.var_tratio_dn4) + ((((locals.var_eu1_i * locals.var_tratio_dn4) * assign13330_e18345) + (assign13330_e18338 * (locals.var_eu1_i * locals.var_tratio_dn4))) / (2.0 * assign13330_e18353)))),)
    } else {
        let assign13330_e18360: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18361: f64 = (locals.var_eu1_i * assign13330_e18360);
        let assign13330_e18362: f64 = (1.0 + assign13330_e18361);
        let assign13330_e18364: f64 = (-10000.0);
        let assign13330_e18366: f64 = (assign13330_e18364 * 0.001);
        let (assign13330_e18381, assign13330_e18381_d_n4,) = {
            if (assign13330_e18362 < assign13330_e18366) {
                let assign13330_e18369: f64 = (-0.001);
                let assign13330_e18371: f64 = (assign13330_e18369 * 0.001);
                let assign13330_e18376: f64 = (locals.var_tratio - 1.0);
                let assign13330_e18377: f64 = (locals.var_eu1_i * assign13330_e18376);
                let assign13330_e18378: f64 = (1.0 + assign13330_e18377);
                let assign13330_e18379: f64 = (assign13330_e18371 / assign13330_e18378);
                (assign13330_e18379, (-((assign13330_e18371 * (locals.var_eu1_i * locals.var_tratio_dn4)) / (assign13330_e18378 * assign13330_e18378))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13330_e18381, assign13330_e18381_d_n4,)
    }
};
        let assign13330_e18383: f64 = (locals.var_eu_i * assign13330_e18382);
        locals.var_eu_t = assign13330_e18383;
        locals.var_eu_t_dn0 = (locals.var_eu_i_dn0 * assign13330_e18382);
        locals.var_eu_t_dn2 = (locals.var_eu_i_dn2 * assign13330_e18382);
        locals.var_eu_t_dn3 = (locals.var_eu_i_dn3 * assign13330_e18382);
        locals.var_eu_t_dn4 = ((locals.var_eu_i_dn4 * assign13330_e18382) + (locals.var_eu_i * assign13330_e18382_d_n4));
        locals.var_eu_t_dn5 = (locals.var_eu_i_dn5 * assign13330_e18382);
        locals.var_eu_t_dn6 = (locals.var_eu_i_dn6 * assign13330_e18382);
        locals.var_eu_t_dn7 = (locals.var_eu_i_dn7 * assign13330_e18382);
        locals.var_eu_t_dn8 = (locals.var_eu_i_dn8 * assign13330_e18382);
        locals.var_eu_t_dn9 = (locals.var_eu_i_dn9 * assign13330_e18382);
        locals.var_eu_t_dn10 = (locals.var_eu_i_dn10 * assign13330_e18382);
        locals.var_eu_t_dn11 = (locals.var_eu_i_dn11 * assign13330_e18382);
        locals.var_eu_t_dn12 = (locals.var_eu_i_dn12 * assign13330_e18382);
        locals.var_eu_t_dn13 = (locals.var_eu_i_dn13 * assign13330_e18382);
        locals.var_eu_t_dn14 = (locals.var_eu_i_dn14 * assign13330_e18382);
        locals.var_eu_t_rv = 0.0;

        let assign13340_e18386: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard454 = assign13340_e18386;
        locals.var_guard454_rv = 0.0;

        let (assign13350_e18394, assign13350_e18394_d_n4,) = {
    if (locals.var_guard454 != 0.0) {
        let assign13350_e18391: f64 = (locals.var_tratio).powf(locals.var_ute_i);
        let assign13350_e18392: f64 = (locals.var_u0r_i * assign13350_e18391);
        (assign13350_e18392, (locals.var_u0r_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13350_e18391 * (locals.var_ute_i * (locals.var_tratio_dn4 / locals.var_tratio))) }),)
    } else {
        (locals.var_u0r_t, locals.var_u0r_t_dn4,)
    }
};
        locals.var_u0r_t = assign13350_e18394;
        locals.var_u0r_t_dn4 = assign13350_e18394_d_n4;
        locals.var_u0r_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13360_e18471, assign13360_e18471_d_n0, assign13360_e18471_d_n2, assign13360_e18471_d_n3, assign13360_e18471_d_n4, assign13360_e18471_d_n5, assign13360_e18471_d_n6, assign13360_e18471_d_n7, assign13360_e18471_d_n8, assign13360_e18471_d_n9, assign13360_e18471_d_n10, assign13360_e18471_d_n11, assign13360_e18471_d_n12, assign13360_e18471_d_n13, assign13360_e18471_d_n14,) = {
    if (locals.var_guard454 != 0.0) {
        let assign13360_e18400: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13360_e18401: f64 = (1.0 + assign13360_e18400);
        let assign13360_e18403: f64 = (assign13360_e18401 - 1e-6);
        let assign13360_e18405: f64 = (-10000.0);
        let assign13360_e18407: f64 = (assign13360_e18405 * 0.001);
        let (assign13360_e18468, assign13360_e18468_d_n4,) = {
            if (!(assign13360_e18403 < assign13360_e18407)) {
                let assign13360_e18414: f64 = (locals.var_ua1_i * locals.var_deltemp);
                let assign13360_e18415: f64 = (1.0 + assign13360_e18414);
                let assign13360_e18417: f64 = (assign13360_e18415 - 1e-6);
                let assign13360_e18421: f64 = (locals.var_ua1_i * locals.var_deltemp);
                let assign13360_e18422: f64 = (1.0 + assign13360_e18421);
                let assign13360_e18424: f64 = (assign13360_e18422 - 1e-6);
                let assign13360_e18428: f64 = (locals.var_ua1_i * locals.var_deltemp);
                let assign13360_e18429: f64 = (1.0 + assign13360_e18428);
                let assign13360_e18431: f64 = (assign13360_e18429 - 1e-6);
                let assign13360_e18432: f64 = (assign13360_e18424 * assign13360_e18431);
                let assign13360_e18435: f64 = (4.0 * 0.001);
                let assign13360_e18437: f64 = (assign13360_e18435 * 0.001);
                let assign13360_e18438: f64 = (assign13360_e18432 + assign13360_e18437);
                let assign13360_e18439: f64 = (assign13360_e18438).sqrt();
                let assign13360_e18440: f64 = (assign13360_e18417 + assign13360_e18439);
                let assign13360_e18441: f64 = (0.5 * assign13360_e18440);
                (assign13360_e18441, (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn4) + ((((locals.var_ua1_i * locals.var_deltemp_dn4) * assign13360_e18431) + (assign13360_e18424 * (locals.var_ua1_i * locals.var_deltemp_dn4))) / (2.0 * assign13360_e18439)))),)
            } else {
                let assign13360_e18445: f64 = (locals.var_ua1_i * locals.var_deltemp);
                let assign13360_e18446: f64 = (1.0 + assign13360_e18445);
                let assign13360_e18448: f64 = (assign13360_e18446 - 1e-6);
                let assign13360_e18450: f64 = (-10000.0);
                let assign13360_e18452: f64 = (assign13360_e18450 * 0.001);
                let (assign13360_e18467, assign13360_e18467_d_n4,) = {
                    if (assign13360_e18448 < assign13360_e18452) {
                        let assign13360_e18455: f64 = (-0.001);
                        let assign13360_e18457: f64 = (assign13360_e18455 * 0.001);
                        let assign13360_e18461: f64 = (locals.var_ua1_i * locals.var_deltemp);
                        let assign13360_e18462: f64 = (1.0 + assign13360_e18461);
                        let assign13360_e18464: f64 = (assign13360_e18462 - 1e-6);
                        let assign13360_e18465: f64 = (assign13360_e18457 / assign13360_e18464);
                        (assign13360_e18465, (-((assign13360_e18457 * (locals.var_ua1_i * locals.var_deltemp_dn4)) / (assign13360_e18464 * assign13360_e18464))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign13360_e18467, assign13360_e18467_d_n4,)
            }
        };
        let assign13360_e18469: f64 = (locals.var_uar_i * assign13360_e18468);
        (assign13360_e18469, (locals.var_uar_i_dn0 * assign13360_e18468), (locals.var_uar_i_dn2 * assign13360_e18468), (locals.var_uar_i_dn3 * assign13360_e18468), ((locals.var_uar_i_dn4 * assign13360_e18468) + (locals.var_uar_i * assign13360_e18468_d_n4)), (locals.var_uar_i_dn5 * assign13360_e18468), (locals.var_uar_i_dn6 * assign13360_e18468), (locals.var_uar_i_dn7 * assign13360_e18468), (locals.var_uar_i_dn8 * assign13360_e18468), (locals.var_uar_i_dn9 * assign13360_e18468), (locals.var_uar_i_dn10 * assign13360_e18468), (locals.var_uar_i_dn11 * assign13360_e18468), (locals.var_uar_i_dn12 * assign13360_e18468), (locals.var_uar_i_dn13 * assign13360_e18468), (locals.var_uar_i_dn14 * assign13360_e18468),)
    } else {
        (locals.var_uar_t, locals.var_uar_t_dn0, locals.var_uar_t_dn2, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11, locals.var_uar_t_dn12, locals.var_uar_t_dn13, locals.var_uar_t_dn14,)
    }
};
        locals.var_uar_t = assign13360_e18471;
        locals.var_uar_t_dn0 = assign13360_e18471_d_n0;
        locals.var_uar_t_dn2 = assign13360_e18471_d_n2;
        locals.var_uar_t_dn3 = assign13360_e18471_d_n3;
        locals.var_uar_t_dn4 = assign13360_e18471_d_n4;
        locals.var_uar_t_dn5 = assign13360_e18471_d_n5;
        locals.var_uar_t_dn6 = assign13360_e18471_d_n6;
        locals.var_uar_t_dn7 = assign13360_e18471_d_n7;
        locals.var_uar_t_dn8 = assign13360_e18471_d_n8;
        locals.var_uar_t_dn9 = assign13360_e18471_d_n9;
        locals.var_uar_t_dn10 = assign13360_e18471_d_n10;
        locals.var_uar_t_dn11 = assign13360_e18471_d_n11;
        locals.var_uar_t_dn12 = assign13360_e18471_d_n12;
        locals.var_uar_t_dn13 = assign13360_e18471_d_n13;
        locals.var_uar_t_dn14 = assign13360_e18471_d_n14;
        locals.var_uar_t_rv = 0.0;

        let (assign13370_e18548, assign13370_e18548_d_n0, assign13370_e18548_d_n2, assign13370_e18548_d_n3, assign13370_e18548_d_n4, assign13370_e18548_d_n5, assign13370_e18548_d_n6, assign13370_e18548_d_n7, assign13370_e18548_d_n8, assign13370_e18548_d_n9, assign13370_e18548_d_n10, assign13370_e18548_d_n11, assign13370_e18548_d_n12, assign13370_e18548_d_n13, assign13370_e18548_d_n14,) = {
    if (locals.var_guard454 != 0.0) {
        let assign13370_e18477: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13370_e18478: f64 = (1.0 + assign13370_e18477);
        let assign13370_e18480: f64 = (assign13370_e18478 - 1e-6);
        let assign13370_e18482: f64 = (-10000.0);
        let assign13370_e18484: f64 = (assign13370_e18482 * 0.001);
        let (assign13370_e18545, assign13370_e18545_d_n4,) = {
            if (!(assign13370_e18480 < assign13370_e18484)) {
                let assign13370_e18491: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign13370_e18492: f64 = (1.0 + assign13370_e18491);
                let assign13370_e18494: f64 = (assign13370_e18492 - 1e-6);
                let assign13370_e18498: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign13370_e18499: f64 = (1.0 + assign13370_e18498);
                let assign13370_e18501: f64 = (assign13370_e18499 - 1e-6);
                let assign13370_e18505: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign13370_e18506: f64 = (1.0 + assign13370_e18505);
                let assign13370_e18508: f64 = (assign13370_e18506 - 1e-6);
                let assign13370_e18509: f64 = (assign13370_e18501 * assign13370_e18508);
                let assign13370_e18512: f64 = (4.0 * 0.001);
                let assign13370_e18514: f64 = (assign13370_e18512 * 0.001);
                let assign13370_e18515: f64 = (assign13370_e18509 + assign13370_e18514);
                let assign13370_e18516: f64 = (assign13370_e18515).sqrt();
                let assign13370_e18517: f64 = (assign13370_e18494 + assign13370_e18516);
                let assign13370_e18518: f64 = (0.5 * assign13370_e18517);
                (assign13370_e18518, (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn4) + ((((locals.var_uc1_i * locals.var_deltemp_dn4) * assign13370_e18508) + (assign13370_e18501 * (locals.var_uc1_i * locals.var_deltemp_dn4))) / (2.0 * assign13370_e18516)))),)
            } else {
                let assign13370_e18522: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign13370_e18523: f64 = (1.0 + assign13370_e18522);
                let assign13370_e18525: f64 = (assign13370_e18523 - 1e-6);
                let assign13370_e18527: f64 = (-10000.0);
                let assign13370_e18529: f64 = (assign13370_e18527 * 0.001);
                let (assign13370_e18544, assign13370_e18544_d_n4,) = {
                    if (assign13370_e18525 < assign13370_e18529) {
                        let assign13370_e18532: f64 = (-0.001);
                        let assign13370_e18534: f64 = (assign13370_e18532 * 0.001);
                        let assign13370_e18538: f64 = (locals.var_uc1_i * locals.var_deltemp);
                        let assign13370_e18539: f64 = (1.0 + assign13370_e18538);
                        let assign13370_e18541: f64 = (assign13370_e18539 - 1e-6);
                        let assign13370_e18542: f64 = (assign13370_e18534 / assign13370_e18541);
                        (assign13370_e18542, (-((assign13370_e18534 * (locals.var_uc1_i * locals.var_deltemp_dn4)) / (assign13370_e18541 * assign13370_e18541))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign13370_e18544, assign13370_e18544_d_n4,)
            }
        };
        let assign13370_e18546: f64 = (locals.var_ucr_i * assign13370_e18545);
        (assign13370_e18546, (locals.var_ucr_i_dn0 * assign13370_e18545), (locals.var_ucr_i_dn2 * assign13370_e18545), (locals.var_ucr_i_dn3 * assign13370_e18545), ((locals.var_ucr_i_dn4 * assign13370_e18545) + (locals.var_ucr_i * assign13370_e18545_d_n4)), (locals.var_ucr_i_dn5 * assign13370_e18545), (locals.var_ucr_i_dn6 * assign13370_e18545), (locals.var_ucr_i_dn7 * assign13370_e18545), (locals.var_ucr_i_dn8 * assign13370_e18545), (locals.var_ucr_i_dn9 * assign13370_e18545), (locals.var_ucr_i_dn10 * assign13370_e18545), (locals.var_ucr_i_dn11 * assign13370_e18545), (locals.var_ucr_i_dn12 * assign13370_e18545), (locals.var_ucr_i_dn13 * assign13370_e18545), (locals.var_ucr_i_dn14 * assign13370_e18545),)
    } else {
        (locals.var_ucr_t, locals.var_ucr_t_dn0, locals.var_ucr_t_dn2, locals.var_ucr_t_dn3, locals.var_ucr_t_dn4, locals.var_ucr_t_dn5, locals.var_ucr_t_dn6, locals.var_ucr_t_dn7, locals.var_ucr_t_dn8, locals.var_ucr_t_dn9, locals.var_ucr_t_dn10, locals.var_ucr_t_dn11, locals.var_ucr_t_dn12, locals.var_ucr_t_dn13, locals.var_ucr_t_dn14,)
    }
};
        locals.var_ucr_t = assign13370_e18548;
        locals.var_ucr_t_dn0 = assign13370_e18548_d_n0;
        locals.var_ucr_t_dn2 = assign13370_e18548_d_n2;
        locals.var_ucr_t_dn3 = assign13370_e18548_d_n3;
        locals.var_ucr_t_dn4 = assign13370_e18548_d_n4;
        locals.var_ucr_t_dn5 = assign13370_e18548_d_n5;
        locals.var_ucr_t_dn6 = assign13370_e18548_d_n6;
        locals.var_ucr_t_dn7 = assign13370_e18548_d_n7;
        locals.var_ucr_t_dn8 = assign13370_e18548_d_n8;
        locals.var_ucr_t_dn9 = assign13370_e18548_d_n9;
        locals.var_ucr_t_dn10 = assign13370_e18548_d_n10;
        locals.var_ucr_t_dn11 = assign13370_e18548_d_n11;
        locals.var_ucr_t_dn12 = assign13370_e18548_d_n12;
        locals.var_ucr_t_dn13 = assign13370_e18548_d_n13;
        locals.var_ucr_t_dn14 = assign13370_e18548_d_n14;
        locals.var_ucr_t_rv = 0.0;

        let (assign13380_e18556, assign13380_e18556_d_n0, assign13380_e18556_d_n2, assign13380_e18556_d_n3, assign13380_e18556_d_n4, assign13380_e18556_d_n5, assign13380_e18556_d_n6, assign13380_e18556_d_n7, assign13380_e18556_d_n8, assign13380_e18556_d_n9, assign13380_e18556_d_n10, assign13380_e18556_d_n11, assign13380_e18556_d_n12, assign13380_e18556_d_n13, assign13380_e18556_d_n14,) = {
    if (locals.var_guard454 != 0.0) {
        let assign13380_e18553: f64 = (locals.var_tratio).powf(locals.var_ud1_i);
        let assign13380_e18554: f64 = (locals.var_udr_i * assign13380_e18553);
        (assign13380_e18554, (locals.var_udr_i_dn0 * assign13380_e18553), (locals.var_udr_i_dn2 * assign13380_e18553), (locals.var_udr_i_dn3 * assign13380_e18553), ((locals.var_udr_i_dn4 * assign13380_e18553) + (locals.var_udr_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13380_e18553 * (locals.var_ud1_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_udr_i_dn5 * assign13380_e18553), (locals.var_udr_i_dn6 * assign13380_e18553), (locals.var_udr_i_dn7 * assign13380_e18553), (locals.var_udr_i_dn8 * assign13380_e18553), (locals.var_udr_i_dn9 * assign13380_e18553), (locals.var_udr_i_dn10 * assign13380_e18553), (locals.var_udr_i_dn11 * assign13380_e18553), (locals.var_udr_i_dn12 * assign13380_e18553), (locals.var_udr_i_dn13 * assign13380_e18553), (locals.var_udr_i_dn14 * assign13380_e18553),)
    } else {
        (locals.var_udr_t, locals.var_udr_t_dn0, locals.var_udr_t_dn2, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11, locals.var_udr_t_dn12, locals.var_udr_t_dn13, locals.var_udr_t_dn14,)
    }
};
        locals.var_udr_t = assign13380_e18556;
        locals.var_udr_t_dn0 = assign13380_e18556_d_n0;
        locals.var_udr_t_dn2 = assign13380_e18556_d_n2;
        locals.var_udr_t_dn3 = assign13380_e18556_d_n3;
        locals.var_udr_t_dn4 = assign13380_e18556_d_n4;
        locals.var_udr_t_dn5 = assign13380_e18556_d_n5;
        locals.var_udr_t_dn6 = assign13380_e18556_d_n6;
        locals.var_udr_t_dn7 = assign13380_e18556_d_n7;
        locals.var_udr_t_dn8 = assign13380_e18556_d_n8;
        locals.var_udr_t_dn9 = assign13380_e18556_d_n9;
        locals.var_udr_t_dn10 = assign13380_e18556_d_n10;
        locals.var_udr_t_dn11 = assign13380_e18556_d_n11;
        locals.var_udr_t_dn12 = assign13380_e18556_d_n12;
        locals.var_udr_t_dn13 = assign13380_e18556_d_n13;
        locals.var_udr_t_dn14 = assign13380_e18556_d_n14;
        locals.var_udr_t_rv = 0.0;

        let (assign13390_e18564, assign13390_e18564_d_n4,) = {
    if (locals.var_guard454 != 0.0) {
        let assign13390_e18561: f64 = (locals.var_tratio).powf(locals.var_ucste_i);
        let assign13390_e18562: f64 = (locals.var_ucsr_i * assign13390_e18561);
        (assign13390_e18562, (locals.var_ucsr_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13390_e18561 * (locals.var_ucste_i * (locals.var_tratio_dn4 / locals.var_tratio))) }),)
    } else {
        (locals.var_ucsr_t, locals.var_ucsr_t_dn4,)
    }
};
        locals.var_ucsr_t = assign13390_e18564;
        locals.var_ucsr_t_dn4 = assign13390_e18564_d_n4;
        locals.var_ucsr_t_rv = 0.0;

        let assign13400_e18567: f64 = (locals.var_tratio).powf(locals.var_prt_i);
        locals.var_rdstemp = assign13400_e18567;
        locals.var_rdstemp_dn4 = if 0.0 == 0.0 && ((locals.var_prt_i) as f64).is_finite() && ((locals.var_prt_i) as f64).fract() == 0.0 { if locals.var_prt_i == 0.0 { 0.0 } else { (locals.var_prt_i * ((locals.var_tratio).powf(locals.var_prt_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13400_e18567 * (locals.var_prt_i * (locals.var_tratio_dn4 / locals.var_tratio))) };
        locals.var_rdstemp_rv = 0.0;

        let assign13410_e18571: f64 = (-locals.var_at_i);
        let assign13410_e18572: f64 = (locals.var_tratio).powf(assign13410_e18571);
        let assign13410_e18573: f64 = (locals.var_vsat_i * assign13410_e18572);
        locals.var_vsat_t = assign13410_e18573;
        locals.var_vsat_t_dn0 = (locals.var_vsat_i_dn0 * assign13410_e18572);
        locals.var_vsat_t_dn2 = (locals.var_vsat_i_dn2 * assign13410_e18572);
        locals.var_vsat_t_dn3 = (locals.var_vsat_i_dn3 * assign13410_e18572);
        locals.var_vsat_t_dn4 = ((locals.var_vsat_i_dn4 * assign13410_e18572) + (locals.var_vsat_i * if 0.0 == 0.0 && ((assign13410_e18571) as f64).is_finite() && ((assign13410_e18571) as f64).fract() == 0.0 { if assign13410_e18571 == 0.0 { 0.0 } else { (assign13410_e18571 * ((locals.var_tratio).powf(assign13410_e18571 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13410_e18572 * (assign13410_e18571 * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_vsat_t_dn5 = (locals.var_vsat_i_dn5 * assign13410_e18572);
        locals.var_vsat_t_dn6 = (locals.var_vsat_i_dn6 * assign13410_e18572);
        locals.var_vsat_t_dn7 = (locals.var_vsat_i_dn7 * assign13410_e18572);
        locals.var_vsat_t_dn8 = (locals.var_vsat_i_dn8 * assign13410_e18572);
        locals.var_vsat_t_dn9 = (locals.var_vsat_i_dn9 * assign13410_e18572);
        locals.var_vsat_t_dn10 = (locals.var_vsat_i_dn10 * assign13410_e18572);
        locals.var_vsat_t_dn11 = (locals.var_vsat_i_dn11 * assign13410_e18572);
        locals.var_vsat_t_dn12 = (locals.var_vsat_i_dn12 * assign13410_e18572);
        locals.var_vsat_t_dn13 = (locals.var_vsat_i_dn13 * assign13410_e18572);
        locals.var_vsat_t_dn14 = (locals.var_vsat_i_dn14 * assign13410_e18572);
        locals.var_vsat_t_rv = 0.0;

        let assign13420_e18576: f64 = if locals.var_vsat_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard455 = assign13420_e18576;
        locals.var_guard455_rv = 0.0;

        let (assign13430_e18580, assign13430_e18580_d_n0, assign13430_e18580_d_n2, assign13430_e18580_d_n3, assign13430_e18580_d_n4, assign13430_e18580_d_n5, assign13430_e18580_d_n6, assign13430_e18580_d_n7, assign13430_e18580_d_n8, assign13430_e18580_d_n9, assign13430_e18580_d_n10, assign13430_e18580_d_n11, assign13430_e18580_d_n12, assign13430_e18580_d_n13, assign13430_e18580_d_n14,) = {
    if (locals.var_guard455 != 0.0) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn12, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign13430_e18580;
        locals.var_vsat_t_dn0 = assign13430_e18580_d_n0;
        locals.var_vsat_t_dn2 = assign13430_e18580_d_n2;
        locals.var_vsat_t_dn3 = assign13430_e18580_d_n3;
        locals.var_vsat_t_dn4 = assign13430_e18580_d_n4;
        locals.var_vsat_t_dn5 = assign13430_e18580_d_n5;
        locals.var_vsat_t_dn6 = assign13430_e18580_d_n6;
        locals.var_vsat_t_dn7 = assign13430_e18580_d_n7;
        locals.var_vsat_t_dn8 = assign13430_e18580_d_n8;
        locals.var_vsat_t_dn9 = assign13430_e18580_d_n9;
        locals.var_vsat_t_dn10 = assign13430_e18580_d_n10;
        locals.var_vsat_t_dn11 = assign13430_e18580_d_n11;
        locals.var_vsat_t_dn12 = assign13430_e18580_d_n12;
        locals.var_vsat_t_dn13 = assign13430_e18580_d_n13;
        locals.var_vsat_t_dn14 = assign13430_e18580_d_n14;
        locals.var_vsat_t_rv = 0.0;

        let assign13440_e18583: f64 = if p.p1094 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard456 = assign13440_e18583;
        locals.var_guard456_rv = 0.0;

        let (assign13450_e18589, assign13450_e18589_d_n4,) = {
    if (locals.var_guard456 != 0.0) {
        let assign13450_e18587: f64 = (locals.var_tratio).powf(p.p1120);
        (assign13450_e18587, if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_tratio).powf(p.p1120 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13450_e18587 * (p.p1120 * (locals.var_tratio_dn4 / locals.var_tratio))) },)
    } else {
        (locals.var_rdstemphv, locals.var_rdstemphv_dn4,)
    }
};
        locals.var_rdstemphv = assign13450_e18589;
        locals.var_rdstemphv_dn4 = assign13450_e18589_d_n4;
        locals.var_rdstemphv_rv = 0.0;

        let (assign13460_e18598, assign13460_e18598_d_n4,) = {
    if (locals.var_guard456 != 0.0) {
        let assign13460_e18594: f64 = (-p.p1121);
        let assign13460_e18595: f64 = (locals.var_tratio).powf(assign13460_e18594);
        let assign13460_e18596: f64 = (p.p1100 * assign13460_e18595);
        (assign13460_e18596, (p.p1100 * if 0.0 == 0.0 && ((assign13460_e18594) as f64).is_finite() && ((assign13460_e18594) as f64).fract() == 0.0 { if assign13460_e18594 == 0.0 { 0.0 } else { (assign13460_e18594 * ((locals.var_tratio).powf(assign13460_e18594 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13460_e18595 * (assign13460_e18594 * (locals.var_tratio_dn4 / locals.var_tratio))) }),)
    } else {
        (locals.var_vdrift_t, locals.var_vdrift_t_dn4,)
    }
};
        locals.var_vdrift_t = assign13460_e18598;
        locals.var_vdrift_t_dn4 = assign13460_e18598_d_n4;
        locals.var_vdrift_t_rv = 0.0;

        let assign13470_e18601: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard457 = assign13470_e18601;
        locals.var_guard457_rv = 0.0;

        let (assign13480_e18610, assign13480_e18610_d_n0, assign13480_e18610_d_n2, assign13480_e18610_d_n3, assign13480_e18610_d_n4, assign13480_e18610_d_n5, assign13480_e18610_d_n6, assign13480_e18610_d_n7, assign13480_e18610_d_n8, assign13480_e18610_d_n9, assign13480_e18610_d_n10, assign13480_e18610_d_n11, assign13480_e18610_d_n12, assign13480_e18610_d_n13, assign13480_e18610_d_n14,) = {
    if (locals.var_guard457 != 0.0) {
        let assign13480_e18606: f64 = (-locals.var_at_i);
        let assign13480_e18607: f64 = (locals.var_tratio).powf(assign13480_e18606);
        let assign13480_e18608: f64 = (locals.var_vsatr_i * assign13480_e18607);
        (assign13480_e18608, (locals.var_vsatr_i_dn0 * assign13480_e18607), (locals.var_vsatr_i_dn2 * assign13480_e18607), (locals.var_vsatr_i_dn3 * assign13480_e18607), ((locals.var_vsatr_i_dn4 * assign13480_e18607) + (locals.var_vsatr_i * if 0.0 == 0.0 && ((assign13480_e18606) as f64).is_finite() && ((assign13480_e18606) as f64).fract() == 0.0 { if assign13480_e18606 == 0.0 { 0.0 } else { (assign13480_e18606 * ((locals.var_tratio).powf(assign13480_e18606 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13480_e18607 * (assign13480_e18606 * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_vsatr_i_dn5 * assign13480_e18607), (locals.var_vsatr_i_dn6 * assign13480_e18607), (locals.var_vsatr_i_dn7 * assign13480_e18607), (locals.var_vsatr_i_dn8 * assign13480_e18607), (locals.var_vsatr_i_dn9 * assign13480_e18607), (locals.var_vsatr_i_dn10 * assign13480_e18607), (locals.var_vsatr_i_dn11 * assign13480_e18607), (locals.var_vsatr_i_dn12 * assign13480_e18607), (locals.var_vsatr_i_dn13 * assign13480_e18607), (locals.var_vsatr_i_dn14 * assign13480_e18607),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn0, locals.var_vsatr_t_dn2, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11, locals.var_vsatr_t_dn12, locals.var_vsatr_t_dn13, locals.var_vsatr_t_dn14,)
    }
};
        locals.var_vsatr_t = assign13480_e18610;
        locals.var_vsatr_t_dn0 = assign13480_e18610_d_n0;
        locals.var_vsatr_t_dn2 = assign13480_e18610_d_n2;
        locals.var_vsatr_t_dn3 = assign13480_e18610_d_n3;
        locals.var_vsatr_t_dn4 = assign13480_e18610_d_n4;
        locals.var_vsatr_t_dn5 = assign13480_e18610_d_n5;
        locals.var_vsatr_t_dn6 = assign13480_e18610_d_n6;
        locals.var_vsatr_t_dn7 = assign13480_e18610_d_n7;
        locals.var_vsatr_t_dn8 = assign13480_e18610_d_n8;
        locals.var_vsatr_t_dn9 = assign13480_e18610_d_n9;
        locals.var_vsatr_t_dn10 = assign13480_e18610_d_n10;
        locals.var_vsatr_t_dn11 = assign13480_e18610_d_n11;
        locals.var_vsatr_t_dn12 = assign13480_e18610_d_n12;
        locals.var_vsatr_t_dn13 = assign13480_e18610_d_n13;
        locals.var_vsatr_t_dn14 = assign13480_e18610_d_n14;
        locals.var_vsatr_t_rv = 0.0;

        let assign13490_e18613: f64 = if locals.var_vsatr_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard458 = assign13490_e18613;
        locals.var_guard458_rv = 0.0;

        let (assign13500_e18619, assign13500_e18619_d_n0, assign13500_e18619_d_n2, assign13500_e18619_d_n3, assign13500_e18619_d_n4, assign13500_e18619_d_n5, assign13500_e18619_d_n6, assign13500_e18619_d_n7, assign13500_e18619_d_n8, assign13500_e18619_d_n9, assign13500_e18619_d_n10, assign13500_e18619_d_n11, assign13500_e18619_d_n12, assign13500_e18619_d_n13, assign13500_e18619_d_n14,) = {
    if ((locals.var_guard457 != 0.0) && (locals.var_guard458 != 0.0)) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn0, locals.var_vsatr_t_dn2, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11, locals.var_vsatr_t_dn12, locals.var_vsatr_t_dn13, locals.var_vsatr_t_dn14,)
    }
};
        locals.var_vsatr_t = assign13500_e18619;
        locals.var_vsatr_t_dn0 = assign13500_e18619_d_n0;
        locals.var_vsatr_t_dn2 = assign13500_e18619_d_n2;
        locals.var_vsatr_t_dn3 = assign13500_e18619_d_n3;
        locals.var_vsatr_t_dn4 = assign13500_e18619_d_n4;
        locals.var_vsatr_t_dn5 = assign13500_e18619_d_n5;
        locals.var_vsatr_t_dn6 = assign13500_e18619_d_n6;
        locals.var_vsatr_t_dn7 = assign13500_e18619_d_n7;
        locals.var_vsatr_t_dn8 = assign13500_e18619_d_n8;
        locals.var_vsatr_t_dn9 = assign13500_e18619_d_n9;
        locals.var_vsatr_t_dn10 = assign13500_e18619_d_n10;
        locals.var_vsatr_t_dn11 = assign13500_e18619_d_n11;
        locals.var_vsatr_t_dn12 = assign13500_e18619_d_n12;
        locals.var_vsatr_t_dn13 = assign13500_e18619_d_n13;
        locals.var_vsatr_t_dn14 = assign13500_e18619_d_n14;
        locals.var_vsatr_t_rv = 0.0;

        let assign13510_e18623: f64 = (-locals.var_at_i);
        let assign13510_e18624: f64 = (locals.var_tratio).powf(assign13510_e18623);
        let assign13510_e18625: f64 = (locals.var_vsatcv_i * assign13510_e18624);
        locals.var_vsatcv_t = assign13510_e18625;
        locals.var_vsatcv_t_dn0 = (locals.var_vsatcv_i_dn0 * assign13510_e18624);
        locals.var_vsatcv_t_dn2 = (locals.var_vsatcv_i_dn2 * assign13510_e18624);
        locals.var_vsatcv_t_dn3 = (locals.var_vsatcv_i_dn3 * assign13510_e18624);
        locals.var_vsatcv_t_dn4 = ((locals.var_vsatcv_i_dn4 * assign13510_e18624) + (locals.var_vsatcv_i * if 0.0 == 0.0 && ((assign13510_e18623) as f64).is_finite() && ((assign13510_e18623) as f64).fract() == 0.0 { if assign13510_e18623 == 0.0 { 0.0 } else { (assign13510_e18623 * ((locals.var_tratio).powf(assign13510_e18623 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13510_e18624 * (assign13510_e18623 * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_vsatcv_t_dn5 = (locals.var_vsatcv_i_dn5 * assign13510_e18624);
        locals.var_vsatcv_t_dn6 = (locals.var_vsatcv_i_dn6 * assign13510_e18624);
        locals.var_vsatcv_t_dn7 = (locals.var_vsatcv_i_dn7 * assign13510_e18624);
        locals.var_vsatcv_t_dn8 = (locals.var_vsatcv_i_dn8 * assign13510_e18624);
        locals.var_vsatcv_t_dn9 = (locals.var_vsatcv_i_dn9 * assign13510_e18624);
        locals.var_vsatcv_t_dn10 = (locals.var_vsatcv_i_dn10 * assign13510_e18624);
        locals.var_vsatcv_t_dn11 = (locals.var_vsatcv_i_dn11 * assign13510_e18624);
        locals.var_vsatcv_t_dn12 = (locals.var_vsatcv_i_dn12 * assign13510_e18624);
        locals.var_vsatcv_t_dn13 = (locals.var_vsatcv_i_dn13 * assign13510_e18624);
        locals.var_vsatcv_t_dn14 = (locals.var_vsatcv_i_dn14 * assign13510_e18624);
        locals.var_vsatcv_t_rv = 0.0;

        let assign13520_e18628: f64 = if locals.var_vsatcv_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard459 = assign13520_e18628;
        locals.var_guard459_rv = 0.0;

        let (assign13530_e18632, assign13530_e18632_d_n0, assign13530_e18632_d_n2, assign13530_e18632_d_n3, assign13530_e18632_d_n4, assign13530_e18632_d_n5, assign13530_e18632_d_n6, assign13530_e18632_d_n7, assign13530_e18632_d_n8, assign13530_e18632_d_n9, assign13530_e18632_d_n10, assign13530_e18632_d_n11, assign13530_e18632_d_n12, assign13530_e18632_d_n13, assign13530_e18632_d_n14,) = {
    if (locals.var_guard459 != 0.0) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn12, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign13530_e18632;
        locals.var_vsatcv_t_dn0 = assign13530_e18632_d_n0;
        locals.var_vsatcv_t_dn2 = assign13530_e18632_d_n2;
        locals.var_vsatcv_t_dn3 = assign13530_e18632_d_n3;
        locals.var_vsatcv_t_dn4 = assign13530_e18632_d_n4;
        locals.var_vsatcv_t_dn5 = assign13530_e18632_d_n5;
        locals.var_vsatcv_t_dn6 = assign13530_e18632_d_n6;
        locals.var_vsatcv_t_dn7 = assign13530_e18632_d_n7;
        locals.var_vsatcv_t_dn8 = assign13530_e18632_d_n8;
        locals.var_vsatcv_t_dn9 = assign13530_e18632_d_n9;
        locals.var_vsatcv_t_dn10 = assign13530_e18632_d_n10;
        locals.var_vsatcv_t_dn11 = assign13530_e18632_d_n11;
        locals.var_vsatcv_t_dn12 = assign13530_e18632_d_n12;
        locals.var_vsatcv_t_dn13 = assign13530_e18632_d_n13;
        locals.var_vsatcv_t_dn14 = assign13530_e18632_d_n14;
        locals.var_vsatcv_t_rv = 0.0;

        let assign13540_e18636: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18640: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18641: f64 = (1.0 + assign13540_e18640);
        let assign13540_e18642: f64 = (assign13540_e18636 * assign13540_e18641);
        let assign13540_e18644: f64 = (assign13540_e18642 - 2.0);
        let assign13540_e18646: f64 = (-10000.0);
        let assign13540_e18648: f64 = (assign13540_e18646 * 0.001);
        let (assign13540_e18729, assign13540_e18729_d_n0, assign13540_e18729_d_n2, assign13540_e18729_d_n3, assign13540_e18729_d_n4, assign13540_e18729_d_n5, assign13540_e18729_d_n6, assign13540_e18729_d_n7, assign13540_e18729_d_n8, assign13540_e18729_d_n9, assign13540_e18729_d_n10, assign13540_e18729_d_n11, assign13540_e18729_d_n12, assign13540_e18729_d_n13, assign13540_e18729_d_n14,) = {
    if (!(assign13540_e18644 < assign13540_e18648)) {
        let assign13540_e18654: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18658: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18659: f64 = (1.0 + assign13540_e18658);
        let assign13540_e18660: f64 = (assign13540_e18654 * assign13540_e18659);
        let assign13540_e18662: f64 = (assign13540_e18660 - 2.0);
        let assign13540_e18665: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18669: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18670: f64 = (1.0 + assign13540_e18669);
        let assign13540_e18671: f64 = (assign13540_e18665 * assign13540_e18670);
        let assign13540_e18673: f64 = (assign13540_e18671 - 2.0);
        let assign13540_e18676: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18680: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18681: f64 = (1.0 + assign13540_e18680);
        let assign13540_e18682: f64 = (assign13540_e18676 * assign13540_e18681);
        let assign13540_e18684: f64 = (assign13540_e18682 - 2.0);
        let assign13540_e18685: f64 = (assign13540_e18673 * assign13540_e18684);
        let assign13540_e18688: f64 = (4.0 * 0.001);
        let assign13540_e18690: f64 = (assign13540_e18688 * 0.001);
        let assign13540_e18691: f64 = (assign13540_e18685 + assign13540_e18690);
        let assign13540_e18692: f64 = (assign13540_e18691).sqrt();
        let assign13540_e18693: f64 = (assign13540_e18662 + assign13540_e18692);
        let assign13540_e18694: f64 = (0.5 * assign13540_e18693);
        (assign13540_e18694, (0.5 * (((-(locals.var_delta_i_dn0 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn0 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn0 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn2 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn2 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn2 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * ((((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (assign13540_e18654 * (p.p861 * locals.var_deltemp_dn4))) + ((((((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) + (assign13540_e18665 * (p.p861 * locals.var_deltemp_dn4))) * assign13540_e18684) + (assign13540_e18673 * (((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681) + (assign13540_e18676 * (p.p861 * locals.var_deltemp_dn4))))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn12 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn12 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn12 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn13 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn13 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn13 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn14 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn14 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn14 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))),)
    } else {
        let assign13540_e18697: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18701: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18702: f64 = (1.0 + assign13540_e18701);
        let assign13540_e18703: f64 = (assign13540_e18697 * assign13540_e18702);
        let assign13540_e18705: f64 = (assign13540_e18703 - 2.0);
        let assign13540_e18707: f64 = (-10000.0);
        let assign13540_e18709: f64 = (assign13540_e18707 * 0.001);
        let (assign13540_e18728, assign13540_e18728_d_n0, assign13540_e18728_d_n2, assign13540_e18728_d_n3, assign13540_e18728_d_n4, assign13540_e18728_d_n5, assign13540_e18728_d_n6, assign13540_e18728_d_n7, assign13540_e18728_d_n8, assign13540_e18728_d_n9, assign13540_e18728_d_n10, assign13540_e18728_d_n11, assign13540_e18728_d_n12, assign13540_e18728_d_n13, assign13540_e18728_d_n14,) = {
            if (assign13540_e18705 < assign13540_e18709) {
                let assign13540_e18712: f64 = (-0.001);
                let assign13540_e18714: f64 = (assign13540_e18712 * 0.001);
                let assign13540_e18717: f64 = (1.0 / locals.var_delta_i);
                let assign13540_e18721: f64 = (p.p861 * locals.var_deltemp);
                let assign13540_e18722: f64 = (1.0 + assign13540_e18721);
                let assign13540_e18723: f64 = (assign13540_e18717 * assign13540_e18722);
                let assign13540_e18725: f64 = (assign13540_e18723 - 2.0);
                let assign13540_e18726: f64 = (assign13540_e18714 / assign13540_e18725);
                (assign13540_e18726, (-((assign13540_e18714 * ((-(locals.var_delta_i_dn0 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn2 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * (((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722) + (assign13540_e18717 * (p.p861 * locals.var_deltemp_dn4)))) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn12 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn13 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn14 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign13540_e18728, assign13540_e18728_d_n0, assign13540_e18728_d_n2, assign13540_e18728_d_n3, assign13540_e18728_d_n4, assign13540_e18728_d_n5, assign13540_e18728_d_n6, assign13540_e18728_d_n7, assign13540_e18728_d_n8, assign13540_e18728_d_n9, assign13540_e18728_d_n10, assign13540_e18728_d_n11, assign13540_e18728_d_n12, assign13540_e18728_d_n13, assign13540_e18728_d_n14,)
    }
};
        let assign13540_e18731: f64 = (assign13540_e18729 + 2.0);
        let assign13540_e18732: f64 = (1.0 / assign13540_e18731);
        locals.var_delta_t = assign13540_e18732;
        locals.var_delta_t_dn0 = (-(assign13540_e18729_d_n0 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn2 = (-(assign13540_e18729_d_n2 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn3 = (-(assign13540_e18729_d_n3 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn4 = (-(assign13540_e18729_d_n4 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn5 = (-(assign13540_e18729_d_n5 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn6 = (-(assign13540_e18729_d_n6 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn7 = (-(assign13540_e18729_d_n7 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn8 = (-(assign13540_e18729_d_n8 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn9 = (-(assign13540_e18729_d_n9 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn10 = (-(assign13540_e18729_d_n10 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn11 = (-(assign13540_e18729_d_n11 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn12 = (-(assign13540_e18729_d_n12 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn13 = (-(assign13540_e18729_d_n13 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn14 = (-(assign13540_e18729_d_n14 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_rv = 0.0;

        let assign13550_e18737: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18738: f64 = (1.0 - assign13550_e18737);
        let assign13550_e18740: f64 = (assign13550_e18738 - 1e-6);
        let assign13550_e18742: f64 = (-10000.0);
        let assign13550_e18744: f64 = (assign13550_e18742 * 0.001);
        let (assign13550_e18805, assign13550_e18805_d_n4,) = {
    if (!(assign13550_e18740 < assign13550_e18744)) {
        let assign13550_e18751: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18752: f64 = (1.0 - assign13550_e18751);
        let assign13550_e18754: f64 = (assign13550_e18752 - 1e-6);
        let assign13550_e18758: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18759: f64 = (1.0 - assign13550_e18758);
        let assign13550_e18761: f64 = (assign13550_e18759 - 1e-6);
        let assign13550_e18765: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18766: f64 = (1.0 - assign13550_e18765);
        let assign13550_e18768: f64 = (assign13550_e18766 - 1e-6);
        let assign13550_e18769: f64 = (assign13550_e18761 * assign13550_e18768);
        let assign13550_e18772: f64 = (4.0 * 0.001);
        let assign13550_e18774: f64 = (assign13550_e18772 * 0.001);
        let assign13550_e18775: f64 = (assign13550_e18769 + assign13550_e18774);
        let assign13550_e18776: f64 = (assign13550_e18775).sqrt();
        let assign13550_e18777: f64 = (assign13550_e18754 + assign13550_e18776);
        let assign13550_e18778: f64 = (0.5 * assign13550_e18777);
        (assign13550_e18778, (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) * assign13550_e18768) + (assign13550_e18761 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4)))) / (2.0 * assign13550_e18776)))),)
    } else {
        let assign13550_e18782: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18783: f64 = (1.0 - assign13550_e18782);
        let assign13550_e18785: f64 = (assign13550_e18783 - 1e-6);
        let assign13550_e18787: f64 = (-10000.0);
        let assign13550_e18789: f64 = (assign13550_e18787 * 0.001);
        let (assign13550_e18804, assign13550_e18804_d_n4,) = {
            if (assign13550_e18785 < assign13550_e18789) {
                let assign13550_e18792: f64 = (-0.001);
                let assign13550_e18794: f64 = (assign13550_e18792 * 0.001);
                let assign13550_e18798: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
                let assign13550_e18799: f64 = (1.0 - assign13550_e18798);
                let assign13550_e18801: f64 = (assign13550_e18799 - 1e-6);
                let assign13550_e18802: f64 = (assign13550_e18794 / assign13550_e18801);
                (assign13550_e18802, (-((assign13550_e18794 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4))) / (assign13550_e18801 * assign13550_e18801))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13550_e18804, assign13550_e18804_d_n4,)
    }
};
        let assign13550_e18806: f64 = (locals.var_ptwg_i * assign13550_e18805);
        locals.var_ptwg_t = assign13550_e18806;
        locals.var_ptwg_t_dn0 = (locals.var_ptwg_i_dn0 * assign13550_e18805);
        locals.var_ptwg_t_dn2 = (locals.var_ptwg_i_dn2 * assign13550_e18805);
        locals.var_ptwg_t_dn3 = (locals.var_ptwg_i_dn3 * assign13550_e18805);
        locals.var_ptwg_t_dn4 = ((locals.var_ptwg_i_dn4 * assign13550_e18805) + (locals.var_ptwg_i * assign13550_e18805_d_n4));
        locals.var_ptwg_t_dn5 = (locals.var_ptwg_i_dn5 * assign13550_e18805);
        locals.var_ptwg_t_dn6 = (locals.var_ptwg_i_dn6 * assign13550_e18805);
        locals.var_ptwg_t_dn7 = (locals.var_ptwg_i_dn7 * assign13550_e18805);
        locals.var_ptwg_t_dn8 = (locals.var_ptwg_i_dn8 * assign13550_e18805);
        locals.var_ptwg_t_dn9 = (locals.var_ptwg_i_dn9 * assign13550_e18805);
        locals.var_ptwg_t_dn10 = (locals.var_ptwg_i_dn10 * assign13550_e18805);
        locals.var_ptwg_t_dn11 = (locals.var_ptwg_i_dn11 * assign13550_e18805);
        locals.var_ptwg_t_dn12 = (locals.var_ptwg_i_dn12 * assign13550_e18805);
        locals.var_ptwg_t_dn13 = (locals.var_ptwg_i_dn13 * assign13550_e18805);
        locals.var_ptwg_t_dn14 = (locals.var_ptwg_i_dn14 * assign13550_e18805);
        locals.var_ptwg_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13560_e18809: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard460 = assign13560_e18809;
        locals.var_guard460_rv = 0.0;

        let (assign13570_e18886, assign13570_e18886_d_n0, assign13570_e18886_d_n2, assign13570_e18886_d_n3, assign13570_e18886_d_n4, assign13570_e18886_d_n5, assign13570_e18886_d_n6, assign13570_e18886_d_n7, assign13570_e18886_d_n8, assign13570_e18886_d_n9, assign13570_e18886_d_n10, assign13570_e18886_d_n11, assign13570_e18886_d_n12, assign13570_e18886_d_n13, assign13570_e18886_d_n14,) = {
    if (locals.var_guard460 != 0.0) {
        let assign13570_e18815: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13570_e18816: f64 = (1.0 - assign13570_e18815);
        let assign13570_e18818: f64 = (assign13570_e18816 - 1e-6);
        let assign13570_e18820: f64 = (-10000.0);
        let assign13570_e18822: f64 = (assign13570_e18820 * 0.001);
        let (assign13570_e18883, assign13570_e18883_d_n4,) = {
            if (!(assign13570_e18818 < assign13570_e18822)) {
                let assign13570_e18829: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
                let assign13570_e18830: f64 = (1.0 - assign13570_e18829);
                let assign13570_e18832: f64 = (assign13570_e18830 - 1e-6);
                let assign13570_e18836: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
                let assign13570_e18837: f64 = (1.0 - assign13570_e18836);
                let assign13570_e18839: f64 = (assign13570_e18837 - 1e-6);
                let assign13570_e18843: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
                let assign13570_e18844: f64 = (1.0 - assign13570_e18843);
                let assign13570_e18846: f64 = (assign13570_e18844 - 1e-6);
                let assign13570_e18847: f64 = (assign13570_e18839 * assign13570_e18846);
                let assign13570_e18850: f64 = (4.0 * 0.001);
                let assign13570_e18852: f64 = (assign13570_e18850 * 0.001);
                let assign13570_e18853: f64 = (assign13570_e18847 + assign13570_e18852);
                let assign13570_e18854: f64 = (assign13570_e18853).sqrt();
                let assign13570_e18855: f64 = (assign13570_e18832 + assign13570_e18854);
                let assign13570_e18856: f64 = (0.5 * assign13570_e18855);
                (assign13570_e18856, (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) * assign13570_e18846) + (assign13570_e18839 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4)))) / (2.0 * assign13570_e18854)))),)
            } else {
                let assign13570_e18860: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
                let assign13570_e18861: f64 = (1.0 - assign13570_e18860);
                let assign13570_e18863: f64 = (assign13570_e18861 - 1e-6);
                let assign13570_e18865: f64 = (-10000.0);
                let assign13570_e18867: f64 = (assign13570_e18865 * 0.001);
                let (assign13570_e18882, assign13570_e18882_d_n4,) = {
                    if (assign13570_e18863 < assign13570_e18867) {
                        let assign13570_e18870: f64 = (-0.001);
                        let assign13570_e18872: f64 = (assign13570_e18870 * 0.001);
                        let assign13570_e18876: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
                        let assign13570_e18877: f64 = (1.0 - assign13570_e18876);
                        let assign13570_e18879: f64 = (assign13570_e18877 - 1e-6);
                        let assign13570_e18880: f64 = (assign13570_e18872 / assign13570_e18879);
                        (assign13570_e18880, (-((assign13570_e18872 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4))) / (assign13570_e18879 * assign13570_e18879))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign13570_e18882, assign13570_e18882_d_n4,)
            }
        };
        let assign13570_e18884: f64 = (locals.var_ptwgr_i * assign13570_e18883);
        (assign13570_e18884, (locals.var_ptwgr_i_dn0 * assign13570_e18883), (locals.var_ptwgr_i_dn2 * assign13570_e18883), (locals.var_ptwgr_i_dn3 * assign13570_e18883), ((locals.var_ptwgr_i_dn4 * assign13570_e18883) + (locals.var_ptwgr_i * assign13570_e18883_d_n4)), (locals.var_ptwgr_i_dn5 * assign13570_e18883), (locals.var_ptwgr_i_dn6 * assign13570_e18883), (locals.var_ptwgr_i_dn7 * assign13570_e18883), (locals.var_ptwgr_i_dn8 * assign13570_e18883), (locals.var_ptwgr_i_dn9 * assign13570_e18883), (locals.var_ptwgr_i_dn10 * assign13570_e18883), (locals.var_ptwgr_i_dn11 * assign13570_e18883), (locals.var_ptwgr_i_dn12 * assign13570_e18883), (locals.var_ptwgr_i_dn13 * assign13570_e18883), (locals.var_ptwgr_i_dn14 * assign13570_e18883),)
    } else {
        (locals.var_ptwgr_t, locals.var_ptwgr_t_dn0, locals.var_ptwgr_t_dn2, locals.var_ptwgr_t_dn3, locals.var_ptwgr_t_dn4, locals.var_ptwgr_t_dn5, locals.var_ptwgr_t_dn6, locals.var_ptwgr_t_dn7, locals.var_ptwgr_t_dn8, locals.var_ptwgr_t_dn9, locals.var_ptwgr_t_dn10, locals.var_ptwgr_t_dn11, locals.var_ptwgr_t_dn12, locals.var_ptwgr_t_dn13, locals.var_ptwgr_t_dn14,)
    }
};
        locals.var_ptwgr_t = assign13570_e18886;
        locals.var_ptwgr_t_dn0 = assign13570_e18886_d_n0;
        locals.var_ptwgr_t_dn2 = assign13570_e18886_d_n2;
        locals.var_ptwgr_t_dn3 = assign13570_e18886_d_n3;
        locals.var_ptwgr_t_dn4 = assign13570_e18886_d_n4;
        locals.var_ptwgr_t_dn5 = assign13570_e18886_d_n5;
        locals.var_ptwgr_t_dn6 = assign13570_e18886_d_n6;
        locals.var_ptwgr_t_dn7 = assign13570_e18886_d_n7;
        locals.var_ptwgr_t_dn8 = assign13570_e18886_d_n8;
        locals.var_ptwgr_t_dn9 = assign13570_e18886_d_n9;
        locals.var_ptwgr_t_dn10 = assign13570_e18886_d_n10;
        locals.var_ptwgr_t_dn11 = assign13570_e18886_d_n11;
        locals.var_ptwgr_t_dn12 = assign13570_e18886_d_n12;
        locals.var_ptwgr_t_dn13 = assign13570_e18886_d_n13;
        locals.var_ptwgr_t_dn14 = assign13570_e18886_d_n14;
        locals.var_ptwgr_t_rv = 0.0;

        let assign13580_e18891: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18892: f64 = (1.0 + assign13580_e18891);
        let assign13580_e18894: f64 = (assign13580_e18892 - 1e-6);
        let assign13580_e18896: f64 = (-10000.0);
        let assign13580_e18898: f64 = (assign13580_e18896 * 0.001);
        let (assign13580_e18959, assign13580_e18959_d_n4,) = {
    if (!(assign13580_e18894 < assign13580_e18898)) {
        let assign13580_e18905: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18906: f64 = (1.0 + assign13580_e18905);
        let assign13580_e18908: f64 = (assign13580_e18906 - 1e-6);
        let assign13580_e18912: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18913: f64 = (1.0 + assign13580_e18912);
        let assign13580_e18915: f64 = (assign13580_e18913 - 1e-6);
        let assign13580_e18919: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18920: f64 = (1.0 + assign13580_e18919);
        let assign13580_e18922: f64 = (assign13580_e18920 - 1e-6);
        let assign13580_e18923: f64 = (assign13580_e18915 * assign13580_e18922);
        let assign13580_e18926: f64 = (4.0 * 0.001);
        let assign13580_e18928: f64 = (assign13580_e18926 * 0.001);
        let assign13580_e18929: f64 = (assign13580_e18923 + assign13580_e18928);
        let assign13580_e18930: f64 = (assign13580_e18929).sqrt();
        let assign13580_e18931: f64 = (assign13580_e18908 + assign13580_e18930);
        let assign13580_e18932: f64 = (0.5 * assign13580_e18931);
        (assign13580_e18932, (0.5 * ((locals.var_a11_i * locals.var_deltemp_dn4) + ((((locals.var_a11_i * locals.var_deltemp_dn4) * assign13580_e18922) + (assign13580_e18915 * (locals.var_a11_i * locals.var_deltemp_dn4))) / (2.0 * assign13580_e18930)))),)
    } else {
        let assign13580_e18936: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18937: f64 = (1.0 + assign13580_e18936);
        let assign13580_e18939: f64 = (assign13580_e18937 - 1e-6);
        let assign13580_e18941: f64 = (-10000.0);
        let assign13580_e18943: f64 = (assign13580_e18941 * 0.001);
        let (assign13580_e18958, assign13580_e18958_d_n4,) = {
            if (assign13580_e18939 < assign13580_e18943) {
                let assign13580_e18946: f64 = (-0.001);
                let assign13580_e18948: f64 = (assign13580_e18946 * 0.001);
                let assign13580_e18952: f64 = (locals.var_a11_i * locals.var_deltemp);
                let assign13580_e18953: f64 = (1.0 + assign13580_e18952);
                let assign13580_e18955: f64 = (assign13580_e18953 - 1e-6);
                let assign13580_e18956: f64 = (assign13580_e18948 / assign13580_e18955);
                (assign13580_e18956, (-((assign13580_e18948 * (locals.var_a11_i * locals.var_deltemp_dn4)) / (assign13580_e18955 * assign13580_e18955))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13580_e18958, assign13580_e18958_d_n4,)
    }
};
        let assign13580_e18960: f64 = (locals.var_a1_i * assign13580_e18959);
        locals.var_a1_t = assign13580_e18960;
        locals.var_a1_t_dn4 = (locals.var_a1_i * assign13580_e18959_d_n4);
        locals.var_a1_t_rv = 0.0;

        let assign13590_e18965: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e18966: f64 = (1.0 + assign13590_e18965);
        let assign13590_e18968: f64 = (assign13590_e18966 - 1e-6);
        let assign13590_e18970: f64 = (-10000.0);
        let assign13590_e18972: f64 = (assign13590_e18970 * 0.001);
        let (assign13590_e19033, assign13590_e19033_d_n4,) = {
    if (!(assign13590_e18968 < assign13590_e18972)) {
        let assign13590_e18979: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e18980: f64 = (1.0 + assign13590_e18979);
        let assign13590_e18982: f64 = (assign13590_e18980 - 1e-6);
        let assign13590_e18986: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e18987: f64 = (1.0 + assign13590_e18986);
        let assign13590_e18989: f64 = (assign13590_e18987 - 1e-6);
        let assign13590_e18993: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e18994: f64 = (1.0 + assign13590_e18993);
        let assign13590_e18996: f64 = (assign13590_e18994 - 1e-6);
        let assign13590_e18997: f64 = (assign13590_e18989 * assign13590_e18996);
        let assign13590_e19000: f64 = (4.0 * 0.001);
        let assign13590_e19002: f64 = (assign13590_e19000 * 0.001);
        let assign13590_e19003: f64 = (assign13590_e18997 + assign13590_e19002);
        let assign13590_e19004: f64 = (assign13590_e19003).sqrt();
        let assign13590_e19005: f64 = (assign13590_e18982 + assign13590_e19004);
        let assign13590_e19006: f64 = (0.5 * assign13590_e19005);
        (assign13590_e19006, (0.5 * ((locals.var_a21_i * locals.var_deltemp_dn4) + ((((locals.var_a21_i * locals.var_deltemp_dn4) * assign13590_e18996) + (assign13590_e18989 * (locals.var_a21_i * locals.var_deltemp_dn4))) / (2.0 * assign13590_e19004)))),)
    } else {
        let assign13590_e19010: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e19011: f64 = (1.0 + assign13590_e19010);
        let assign13590_e19013: f64 = (assign13590_e19011 - 1e-6);
        let assign13590_e19015: f64 = (-10000.0);
        let assign13590_e19017: f64 = (assign13590_e19015 * 0.001);
        let (assign13590_e19032, assign13590_e19032_d_n4,) = {
            if (assign13590_e19013 < assign13590_e19017) {
                let assign13590_e19020: f64 = (-0.001);
                let assign13590_e19022: f64 = (assign13590_e19020 * 0.001);
                let assign13590_e19026: f64 = (locals.var_a21_i * locals.var_deltemp);
                let assign13590_e19027: f64 = (1.0 + assign13590_e19026);
                let assign13590_e19029: f64 = (assign13590_e19027 - 1e-6);
                let assign13590_e19030: f64 = (assign13590_e19022 / assign13590_e19029);
                (assign13590_e19030, (-((assign13590_e19022 * (locals.var_a21_i * locals.var_deltemp_dn4)) / (assign13590_e19029 * assign13590_e19029))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13590_e19032, assign13590_e19032_d_n4,)
    }
};
        let assign13590_e19034: f64 = (locals.var_a2_i * assign13590_e19033);
        locals.var_a2_t = assign13590_e19034;
        locals.var_a2_t_dn4 = (locals.var_a2_i * assign13590_e19033_d_n4);
        locals.var_a2_t_rv = 0.0;

        let assign13600_e19038: f64 = (locals.var_tratio).powf(locals.var_iit_i);
        let assign13600_e19039: f64 = (locals.var_beta0_i * assign13600_e19038);
        locals.var_beta0_t = assign13600_e19039;
        locals.var_beta0_t_dn0 = (locals.var_beta0_i_dn0 * assign13600_e19038);
        locals.var_beta0_t_dn2 = (locals.var_beta0_i_dn2 * assign13600_e19038);
        locals.var_beta0_t_dn3 = (locals.var_beta0_i_dn3 * assign13600_e19038);
        locals.var_beta0_t_dn4 = ((locals.var_beta0_i_dn4 * assign13600_e19038) + (locals.var_beta0_i * if 0.0 == 0.0 && ((locals.var_iit_i) as f64).is_finite() && ((locals.var_iit_i) as f64).fract() == 0.0 { if locals.var_iit_i == 0.0 { 0.0 } else { (locals.var_iit_i * ((locals.var_tratio).powf(locals.var_iit_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13600_e19038 * (locals.var_iit_i * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_beta0_t_dn5 = (locals.var_beta0_i_dn5 * assign13600_e19038);
        locals.var_beta0_t_dn6 = (locals.var_beta0_i_dn6 * assign13600_e19038);
        locals.var_beta0_t_dn7 = (locals.var_beta0_i_dn7 * assign13600_e19038);
        locals.var_beta0_t_dn8 = (locals.var_beta0_i_dn8 * assign13600_e19038);
        locals.var_beta0_t_dn9 = (locals.var_beta0_i_dn9 * assign13600_e19038);
        locals.var_beta0_t_dn10 = (locals.var_beta0_i_dn10 * assign13600_e19038);
        locals.var_beta0_t_dn11 = (locals.var_beta0_i_dn11 * assign13600_e19038);
        locals.var_beta0_t_dn12 = (locals.var_beta0_i_dn12 * assign13600_e19038);
        locals.var_beta0_t_dn13 = (locals.var_beta0_i_dn13 * assign13600_e19038);
        locals.var_beta0_t_dn14 = (locals.var_beta0_i_dn14 * assign13600_e19038);
        locals.var_beta0_t_rv = 0.0;

        let assign13610_e19042: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard461 = assign13610_e19042;
        locals.var_guard461_rv = 0.0;

        let (assign13620_e19050, assign13620_e19050_d_n4,) = {
    if (locals.var_guard461 != 0.0) {
        let assign13620_e19047: f64 = (locals.var_tratio).powf(locals.var_iit_i);
        let assign13620_e19048: f64 = (locals.var_beta0r_i * assign13620_e19047);
        (assign13620_e19048, (locals.var_beta0r_i * if 0.0 == 0.0 && ((locals.var_iit_i) as f64).is_finite() && ((locals.var_iit_i) as f64).fract() == 0.0 { if locals.var_iit_i == 0.0 { 0.0 } else { (locals.var_iit_i * ((locals.var_tratio).powf(locals.var_iit_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13620_e19047 * (locals.var_iit_i * (locals.var_tratio_dn4 / locals.var_tratio))) }),)
    } else {
        (locals.var_beta0r_t, locals.var_beta0r_t_dn4,)
    }
};
        locals.var_beta0r_t = assign13620_e19050;
        locals.var_beta0r_t_dn4 = assign13620_e19050_d_n4;
        locals.var_beta0r_t_rv = 0.0;

        let assign13630_e19055: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19056: f64 = (1.0 + assign13630_e19055);
        let assign13630_e19058: f64 = (assign13630_e19056 - 1e-6);
        let assign13630_e19060: f64 = (-10000.0);
        let assign13630_e19062: f64 = (assign13630_e19060 * 0.001);
        let (assign13630_e19123, assign13630_e19123_d_n4,) = {
    if (!(assign13630_e19058 < assign13630_e19062)) {
        let assign13630_e19069: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19070: f64 = (1.0 + assign13630_e19069);
        let assign13630_e19072: f64 = (assign13630_e19070 - 1e-6);
        let assign13630_e19076: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19077: f64 = (1.0 + assign13630_e19076);
        let assign13630_e19079: f64 = (assign13630_e19077 - 1e-6);
        let assign13630_e19083: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19084: f64 = (1.0 + assign13630_e19083);
        let assign13630_e19086: f64 = (assign13630_e19084 - 1e-6);
        let assign13630_e19087: f64 = (assign13630_e19079 * assign13630_e19086);
        let assign13630_e19090: f64 = (4.0 * 0.001);
        let assign13630_e19092: f64 = (assign13630_e19090 * 0.001);
        let assign13630_e19093: f64 = (assign13630_e19087 + assign13630_e19092);
        let assign13630_e19094: f64 = (assign13630_e19093).sqrt();
        let assign13630_e19095: f64 = (assign13630_e19072 + assign13630_e19094);
        let assign13630_e19096: f64 = (0.5 * assign13630_e19095);
        (assign13630_e19096, (0.5 * ((locals.var_tgidl_i * locals.var_deltemp_dn4) + ((((locals.var_tgidl_i * locals.var_deltemp_dn4) * assign13630_e19086) + (assign13630_e19079 * (locals.var_tgidl_i * locals.var_deltemp_dn4))) / (2.0 * assign13630_e19094)))),)
    } else {
        let assign13630_e19100: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19101: f64 = (1.0 + assign13630_e19100);
        let assign13630_e19103: f64 = (assign13630_e19101 - 1e-6);
        let assign13630_e19105: f64 = (-10000.0);
        let assign13630_e19107: f64 = (assign13630_e19105 * 0.001);
        let (assign13630_e19122, assign13630_e19122_d_n4,) = {
            if (assign13630_e19103 < assign13630_e19107) {
                let assign13630_e19110: f64 = (-0.001);
                let assign13630_e19112: f64 = (assign13630_e19110 * 0.001);
                let assign13630_e19116: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign13630_e19117: f64 = (1.0 + assign13630_e19116);
                let assign13630_e19119: f64 = (assign13630_e19117 - 1e-6);
                let assign13630_e19120: f64 = (assign13630_e19112 / assign13630_e19119);
                (assign13630_e19120, (-((assign13630_e19112 * (locals.var_tgidl_i * locals.var_deltemp_dn4)) / (assign13630_e19119 * assign13630_e19119))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13630_e19122, assign13630_e19122_d_n4,)
    }
};
        let assign13630_e19124: f64 = (locals.var_bgidl_i * assign13630_e19123);
        locals.var_bgidl_t = assign13630_e19124;
        locals.var_bgidl_t_dn4 = (locals.var_bgidl_i * assign13630_e19123_d_n4);
        locals.var_bgidl_t_rv = 0.0;

        let assign13640_e19129: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19130: f64 = (1.0 + assign13640_e19129);
        let assign13640_e19132: f64 = (assign13640_e19130 - 1e-6);
        let assign13640_e19134: f64 = (-10000.0);
        let assign13640_e19136: f64 = (assign13640_e19134 * 0.001);
        let (assign13640_e19197, assign13640_e19197_d_n4,) = {
    if (!(assign13640_e19132 < assign13640_e19136)) {
        let assign13640_e19143: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19144: f64 = (1.0 + assign13640_e19143);
        let assign13640_e19146: f64 = (assign13640_e19144 - 1e-6);
        let assign13640_e19150: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19151: f64 = (1.0 + assign13640_e19150);
        let assign13640_e19153: f64 = (assign13640_e19151 - 1e-6);
        let assign13640_e19157: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19158: f64 = (1.0 + assign13640_e19157);
        let assign13640_e19160: f64 = (assign13640_e19158 - 1e-6);
        let assign13640_e19161: f64 = (assign13640_e19153 * assign13640_e19160);
        let assign13640_e19164: f64 = (4.0 * 0.001);
        let assign13640_e19166: f64 = (assign13640_e19164 * 0.001);
        let assign13640_e19167: f64 = (assign13640_e19161 + assign13640_e19166);
        let assign13640_e19168: f64 = (assign13640_e19167).sqrt();
        let assign13640_e19169: f64 = (assign13640_e19146 + assign13640_e19168);
        let assign13640_e19170: f64 = (0.5 * assign13640_e19169);
        (assign13640_e19170, (0.5 * ((locals.var_tgidl_i * locals.var_deltemp_dn4) + ((((locals.var_tgidl_i * locals.var_deltemp_dn4) * assign13640_e19160) + (assign13640_e19153 * (locals.var_tgidl_i * locals.var_deltemp_dn4))) / (2.0 * assign13640_e19168)))),)
    } else {
        let assign13640_e19174: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19175: f64 = (1.0 + assign13640_e19174);
        let assign13640_e19177: f64 = (assign13640_e19175 - 1e-6);
        let assign13640_e19179: f64 = (-10000.0);
        let assign13640_e19181: f64 = (assign13640_e19179 * 0.001);
        let (assign13640_e19196, assign13640_e19196_d_n4,) = {
            if (assign13640_e19177 < assign13640_e19181) {
                let assign13640_e19184: f64 = (-0.001);
                let assign13640_e19186: f64 = (assign13640_e19184 * 0.001);
                let assign13640_e19190: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign13640_e19191: f64 = (1.0 + assign13640_e19190);
                let assign13640_e19193: f64 = (assign13640_e19191 - 1e-6);
                let assign13640_e19194: f64 = (assign13640_e19186 / assign13640_e19193);
                (assign13640_e19194, (-((assign13640_e19186 * (locals.var_tgidl_i * locals.var_deltemp_dn4)) / (assign13640_e19193 * assign13640_e19193))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13640_e19196, assign13640_e19196_d_n4,)
    }
};
        let assign13640_e19198: f64 = (locals.var_bgisl_i * assign13640_e19197);
        locals.var_bgisl_t = assign13640_e19198;
        locals.var_bgisl_t_dn4 = (locals.var_bgisl_i * assign13640_e19197_d_n4);
        locals.var_bgisl_t_rv = 0.0;

        let assign13660_e19210: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19211: f64 = (1.0 + assign13660_e19210);
        let assign13660_e19213: f64 = (assign13660_e19211 - 1e-6);
        let assign13660_e19215: f64 = (-10000.0);
        let assign13660_e19217: f64 = (assign13660_e19215 * 0.001);
        let (assign13660_e19278, assign13660_e19278_d_n4,) = {
    if (!(assign13660_e19213 < assign13660_e19217)) {
        let assign13660_e19224: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19225: f64 = (1.0 + assign13660_e19224);
        let assign13660_e19227: f64 = (assign13660_e19225 - 1e-6);
        let assign13660_e19231: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19232: f64 = (1.0 + assign13660_e19231);
        let assign13660_e19234: f64 = (assign13660_e19232 - 1e-6);
        let assign13660_e19238: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19239: f64 = (1.0 + assign13660_e19238);
        let assign13660_e19241: f64 = (assign13660_e19239 - 1e-6);
        let assign13660_e19242: f64 = (assign13660_e19234 * assign13660_e19241);
        let assign13660_e19245: f64 = (4.0 * 0.001);
        let assign13660_e19247: f64 = (assign13660_e19245 * 0.001);
        let assign13660_e19248: f64 = (assign13660_e19242 + assign13660_e19247);
        let assign13660_e19249: f64 = (assign13660_e19248).sqrt();
        let assign13660_e19250: f64 = (assign13660_e19227 + assign13660_e19249);
        let assign13660_e19251: f64 = (0.5 * assign13660_e19250);
        (assign13660_e19251, (0.5 * ((locals.var_k01_i * locals.var_deltemp_dn4) + ((((locals.var_k01_i * locals.var_deltemp_dn4) * assign13660_e19241) + (assign13660_e19234 * (locals.var_k01_i * locals.var_deltemp_dn4))) / (2.0 * assign13660_e19249)))),)
    } else {
        let assign13660_e19255: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19256: f64 = (1.0 + assign13660_e19255);
        let assign13660_e19258: f64 = (assign13660_e19256 - 1e-6);
        let assign13660_e19260: f64 = (-10000.0);
        let assign13660_e19262: f64 = (assign13660_e19260 * 0.001);
        let (assign13660_e19277, assign13660_e19277_d_n4,) = {
            if (assign13660_e19258 < assign13660_e19262) {
                let assign13660_e19265: f64 = (-0.001);
                let assign13660_e19267: f64 = (assign13660_e19265 * 0.001);
                let assign13660_e19271: f64 = (locals.var_k01_i * locals.var_deltemp);
                let assign13660_e19272: f64 = (1.0 + assign13660_e19271);
                let assign13660_e19274: f64 = (assign13660_e19272 - 1e-6);
                let assign13660_e19275: f64 = (assign13660_e19267 / assign13660_e19274);
                (assign13660_e19275, (-((assign13660_e19267 * (locals.var_k01_i * locals.var_deltemp_dn4)) / (assign13660_e19274 * assign13660_e19274))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13660_e19277, assign13660_e19277_d_n4,)
    }
};
        let assign13660_e19279: f64 = (locals.var_k0_i * assign13660_e19278);
        locals.var_k0_t = assign13660_e19279;
        locals.var_k0_t_dn4 = (locals.var_k0_i * assign13660_e19278_d_n4);
        locals.var_k0_t_rv = 0.0;

        let assign13670_e19284: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19285: f64 = (1.0 + assign13670_e19284);
        let assign13670_e19287: f64 = (assign13670_e19285 - 1e-6);
        let assign13670_e19289: f64 = (-10000.0);
        let assign13670_e19291: f64 = (assign13670_e19289 * 0.001);
        let (assign13670_e19352, assign13670_e19352_d_n4,) = {
    if (!(assign13670_e19287 < assign13670_e19291)) {
        let assign13670_e19298: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19299: f64 = (1.0 + assign13670_e19298);
        let assign13670_e19301: f64 = (assign13670_e19299 - 1e-6);
        let assign13670_e19305: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19306: f64 = (1.0 + assign13670_e19305);
        let assign13670_e19308: f64 = (assign13670_e19306 - 1e-6);
        let assign13670_e19312: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19313: f64 = (1.0 + assign13670_e19312);
        let assign13670_e19315: f64 = (assign13670_e19313 - 1e-6);
        let assign13670_e19316: f64 = (assign13670_e19308 * assign13670_e19315);
        let assign13670_e19319: f64 = (4.0 * 0.001);
        let assign13670_e19321: f64 = (assign13670_e19319 * 0.001);
        let assign13670_e19322: f64 = (assign13670_e19316 + assign13670_e19321);
        let assign13670_e19323: f64 = (assign13670_e19322).sqrt();
        let assign13670_e19324: f64 = (assign13670_e19301 + assign13670_e19323);
        let assign13670_e19325: f64 = (0.5 * assign13670_e19324);
        (assign13670_e19325, (0.5 * ((locals.var_m01_i * locals.var_deltemp_dn4) + ((((locals.var_m01_i * locals.var_deltemp_dn4) * assign13670_e19315) + (assign13670_e19308 * (locals.var_m01_i * locals.var_deltemp_dn4))) / (2.0 * assign13670_e19323)))),)
    } else {
        let assign13670_e19329: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19330: f64 = (1.0 + assign13670_e19329);
        let assign13670_e19332: f64 = (assign13670_e19330 - 1e-6);
        let assign13670_e19334: f64 = (-10000.0);
        let assign13670_e19336: f64 = (assign13670_e19334 * 0.001);
        let (assign13670_e19351, assign13670_e19351_d_n4,) = {
            if (assign13670_e19332 < assign13670_e19336) {
                let assign13670_e19339: f64 = (-0.001);
                let assign13670_e19341: f64 = (assign13670_e19339 * 0.001);
                let assign13670_e19345: f64 = (locals.var_m01_i * locals.var_deltemp);
                let assign13670_e19346: f64 = (1.0 + assign13670_e19345);
                let assign13670_e19348: f64 = (assign13670_e19346 - 1e-6);
                let assign13670_e19349: f64 = (assign13670_e19341 / assign13670_e19348);
                (assign13670_e19349, (-((assign13670_e19341 * (locals.var_m01_i * locals.var_deltemp_dn4)) / (assign13670_e19348 * assign13670_e19348))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13670_e19351, assign13670_e19351_d_n4,)
    }
};
        let assign13670_e19353: f64 = (locals.var_m0_i * assign13670_e19352);
        locals.var_m0_t = assign13670_e19353;
        locals.var_m0_t_dn4 = (locals.var_m0_i * assign13670_e19352_d_n4);
        locals.var_m0_t_rv = 0.0;

        let assign13680_e19358: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19359: f64 = (1.0 + assign13680_e19358);
        let assign13680_e19361: f64 = (assign13680_e19359 - 1e-6);
        let assign13680_e19363: f64 = (-10000.0);
        let assign13680_e19365: f64 = (assign13680_e19363 * 0.001);
        let (assign13680_e19426, assign13680_e19426_d_n4,) = {
    if (!(assign13680_e19361 < assign13680_e19365)) {
        let assign13680_e19372: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19373: f64 = (1.0 + assign13680_e19372);
        let assign13680_e19375: f64 = (assign13680_e19373 - 1e-6);
        let assign13680_e19379: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19380: f64 = (1.0 + assign13680_e19379);
        let assign13680_e19382: f64 = (assign13680_e19380 - 1e-6);
        let assign13680_e19386: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19387: f64 = (1.0 + assign13680_e19386);
        let assign13680_e19389: f64 = (assign13680_e19387 - 1e-6);
        let assign13680_e19390: f64 = (assign13680_e19382 * assign13680_e19389);
        let assign13680_e19393: f64 = (4.0 * 0.001);
        let assign13680_e19395: f64 = (assign13680_e19393 * 0.001);
        let assign13680_e19396: f64 = (assign13680_e19390 + assign13680_e19395);
        let assign13680_e19397: f64 = (assign13680_e19396).sqrt();
        let assign13680_e19398: f64 = (assign13680_e19375 + assign13680_e19397);
        let assign13680_e19399: f64 = (0.5 * assign13680_e19398);
        (assign13680_e19399, (0.5 * ((locals.var_c01_i * locals.var_deltemp_dn4) + ((((locals.var_c01_i * locals.var_deltemp_dn4) * assign13680_e19389) + (assign13680_e19382 * (locals.var_c01_i * locals.var_deltemp_dn4))) / (2.0 * assign13680_e19397)))),)
    } else {
        let assign13680_e19403: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19404: f64 = (1.0 + assign13680_e19403);
        let assign13680_e19406: f64 = (assign13680_e19404 - 1e-6);
        let assign13680_e19408: f64 = (-10000.0);
        let assign13680_e19410: f64 = (assign13680_e19408 * 0.001);
        let (assign13680_e19425, assign13680_e19425_d_n4,) = {
            if (assign13680_e19406 < assign13680_e19410) {
                let assign13680_e19413: f64 = (-0.001);
                let assign13680_e19415: f64 = (assign13680_e19413 * 0.001);
                let assign13680_e19419: f64 = (locals.var_c01_i * locals.var_deltemp);
                let assign13680_e19420: f64 = (1.0 + assign13680_e19419);
                let assign13680_e19422: f64 = (assign13680_e19420 - 1e-6);
                let assign13680_e19423: f64 = (assign13680_e19415 / assign13680_e19422);
                (assign13680_e19423, (-((assign13680_e19415 * (locals.var_c01_i * locals.var_deltemp_dn4)) / (assign13680_e19422 * assign13680_e19422))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13680_e19425, assign13680_e19425_d_n4,)
    }
};
        let assign13680_e19427: f64 = (locals.var_c0_i * assign13680_e19426);
        locals.var_c0_t = assign13680_e19427;
        locals.var_c0_t_dn4 = (locals.var_c0_i * assign13680_e19426_d_n4);
        locals.var_c0_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13690_e19432: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19433: f64 = (1.0 + assign13690_e19432);
        let assign13690_e19435: f64 = (assign13690_e19433 - 1e-6);
        let assign13690_e19437: f64 = (-10000.0);
        let assign13690_e19439: f64 = (assign13690_e19437 * 0.001);
        let (assign13690_e19500, assign13690_e19500_d_n4,) = {
    if (!(assign13690_e19435 < assign13690_e19439)) {
        let assign13690_e19446: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19447: f64 = (1.0 + assign13690_e19446);
        let assign13690_e19449: f64 = (assign13690_e19447 - 1e-6);
        let assign13690_e19453: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19454: f64 = (1.0 + assign13690_e19453);
        let assign13690_e19456: f64 = (assign13690_e19454 - 1e-6);
        let assign13690_e19460: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19461: f64 = (1.0 + assign13690_e19460);
        let assign13690_e19463: f64 = (assign13690_e19461 - 1e-6);
        let assign13690_e19464: f64 = (assign13690_e19456 * assign13690_e19463);
        let assign13690_e19467: f64 = (4.0 * 0.001);
        let assign13690_e19469: f64 = (assign13690_e19467 * 0.001);
        let assign13690_e19470: f64 = (assign13690_e19464 + assign13690_e19469);
        let assign13690_e19471: f64 = (assign13690_e19470).sqrt();
        let assign13690_e19472: f64 = (assign13690_e19449 + assign13690_e19471);
        let assign13690_e19473: f64 = (0.5 * assign13690_e19472);
        (assign13690_e19473, (0.5 * ((locals.var_c0si1_i * locals.var_deltemp_dn4) + ((((locals.var_c0si1_i * locals.var_deltemp_dn4) * assign13690_e19463) + (assign13690_e19456 * (locals.var_c0si1_i * locals.var_deltemp_dn4))) / (2.0 * assign13690_e19471)))),)
    } else {
        let assign13690_e19477: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19478: f64 = (1.0 + assign13690_e19477);
        let assign13690_e19480: f64 = (assign13690_e19478 - 1e-6);
        let assign13690_e19482: f64 = (-10000.0);
        let assign13690_e19484: f64 = (assign13690_e19482 * 0.001);
        let (assign13690_e19499, assign13690_e19499_d_n4,) = {
            if (assign13690_e19480 < assign13690_e19484) {
                let assign13690_e19487: f64 = (-0.001);
                let assign13690_e19489: f64 = (assign13690_e19487 * 0.001);
                let assign13690_e19493: f64 = (locals.var_c0si1_i * locals.var_deltemp);
                let assign13690_e19494: f64 = (1.0 + assign13690_e19493);
                let assign13690_e19496: f64 = (assign13690_e19494 - 1e-6);
                let assign13690_e19497: f64 = (assign13690_e19489 / assign13690_e19496);
                (assign13690_e19497, (-((assign13690_e19489 * (locals.var_c0si1_i * locals.var_deltemp_dn4)) / (assign13690_e19496 * assign13690_e19496))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13690_e19499, assign13690_e19499_d_n4,)
    }
};
        let assign13690_e19501: f64 = (locals.var_c0si_i * assign13690_e19500);
        locals.var_c0si_t = assign13690_e19501;
        locals.var_c0si_t_dn4 = (locals.var_c0si_i * assign13690_e19500_d_n4);
        locals.var_c0si_t_rv = 0.0;

        let assign13700_e19506: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19507: f64 = (1.0 + assign13700_e19506);
        let assign13700_e19509: f64 = (assign13700_e19507 - 1e-6);
        let assign13700_e19511: f64 = (-10000.0);
        let assign13700_e19513: f64 = (assign13700_e19511 * 0.001);
        let (assign13700_e19574, assign13700_e19574_d_n4,) = {
    if (!(assign13700_e19509 < assign13700_e19513)) {
        let assign13700_e19520: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19521: f64 = (1.0 + assign13700_e19520);
        let assign13700_e19523: f64 = (assign13700_e19521 - 1e-6);
        let assign13700_e19527: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19528: f64 = (1.0 + assign13700_e19527);
        let assign13700_e19530: f64 = (assign13700_e19528 - 1e-6);
        let assign13700_e19534: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19535: f64 = (1.0 + assign13700_e19534);
        let assign13700_e19537: f64 = (assign13700_e19535 - 1e-6);
        let assign13700_e19538: f64 = (assign13700_e19530 * assign13700_e19537);
        let assign13700_e19541: f64 = (4.0 * 0.001);
        let assign13700_e19543: f64 = (assign13700_e19541 * 0.001);
        let assign13700_e19544: f64 = (assign13700_e19538 + assign13700_e19543);
        let assign13700_e19545: f64 = (assign13700_e19544).sqrt();
        let assign13700_e19546: f64 = (assign13700_e19523 + assign13700_e19545);
        let assign13700_e19547: f64 = (0.5 * assign13700_e19546);
        (assign13700_e19547, (0.5 * ((locals.var_c0sisat1_i * locals.var_deltemp_dn4) + ((((locals.var_c0sisat1_i * locals.var_deltemp_dn4) * assign13700_e19537) + (assign13700_e19530 * (locals.var_c0sisat1_i * locals.var_deltemp_dn4))) / (2.0 * assign13700_e19545)))),)
    } else {
        let assign13700_e19551: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19552: f64 = (1.0 + assign13700_e19551);
        let assign13700_e19554: f64 = (assign13700_e19552 - 1e-6);
        let assign13700_e19556: f64 = (-10000.0);
        let assign13700_e19558: f64 = (assign13700_e19556 * 0.001);
        let (assign13700_e19573, assign13700_e19573_d_n4,) = {
            if (assign13700_e19554 < assign13700_e19558) {
                let assign13700_e19561: f64 = (-0.001);
                let assign13700_e19563: f64 = (assign13700_e19561 * 0.001);
                let assign13700_e19567: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
                let assign13700_e19568: f64 = (1.0 + assign13700_e19567);
                let assign13700_e19570: f64 = (assign13700_e19568 - 1e-6);
                let assign13700_e19571: f64 = (assign13700_e19563 / assign13700_e19570);
                (assign13700_e19571, (-((assign13700_e19563 * (locals.var_c0sisat1_i * locals.var_deltemp_dn4)) / (assign13700_e19570 * assign13700_e19570))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13700_e19573, assign13700_e19573_d_n4,)
    }
};
        let assign13700_e19575: f64 = (locals.var_c0sisat_i * assign13700_e19574);
        locals.var_c0sisat_t = assign13700_e19575;
        locals.var_c0sisat_t_dn4 = (locals.var_c0sisat_i * assign13700_e19574_d_n4);
        locals.var_c0sisat_t_rv = 0.0;

        let assign13710_e19580: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19581: f64 = (1.0 + assign13710_e19580);
        let assign13710_e19583: f64 = (assign13710_e19581 - 1e-6);
        let assign13710_e19585: f64 = (-10000.0);
        let assign13710_e19587: f64 = (assign13710_e19585 * 0.001);
        let (assign13710_e19648, assign13710_e19648_d_n4,) = {
    if (!(assign13710_e19583 < assign13710_e19587)) {
        let assign13710_e19594: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19595: f64 = (1.0 + assign13710_e19594);
        let assign13710_e19597: f64 = (assign13710_e19595 - 1e-6);
        let assign13710_e19601: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19602: f64 = (1.0 + assign13710_e19601);
        let assign13710_e19604: f64 = (assign13710_e19602 - 1e-6);
        let assign13710_e19608: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19609: f64 = (1.0 + assign13710_e19608);
        let assign13710_e19611: f64 = (assign13710_e19609 - 1e-6);
        let assign13710_e19612: f64 = (assign13710_e19604 * assign13710_e19611);
        let assign13710_e19615: f64 = (4.0 * 0.001);
        let assign13710_e19617: f64 = (assign13710_e19615 * 0.001);
        let assign13710_e19618: f64 = (assign13710_e19612 + assign13710_e19617);
        let assign13710_e19619: f64 = (assign13710_e19618).sqrt();
        let assign13710_e19620: f64 = (assign13710_e19597 + assign13710_e19619);
        let assign13710_e19621: f64 = (0.5 * assign13710_e19620);
        (assign13710_e19621, (0.5 * ((p.p889 * locals.var_deltemp_dn4) + ((((p.p889 * locals.var_deltemp_dn4) * assign13710_e19611) + (assign13710_e19604 * (p.p889 * locals.var_deltemp_dn4))) / (2.0 * assign13710_e19619)))),)
    } else {
        let assign13710_e19625: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19626: f64 = (1.0 + assign13710_e19625);
        let assign13710_e19628: f64 = (assign13710_e19626 - 1e-6);
        let assign13710_e19630: f64 = (-10000.0);
        let assign13710_e19632: f64 = (assign13710_e19630 * 0.001);
        let (assign13710_e19647, assign13710_e19647_d_n4,) = {
            if (assign13710_e19628 < assign13710_e19632) {
                let assign13710_e19635: f64 = (-0.001);
                let assign13710_e19637: f64 = (assign13710_e19635 * 0.001);
                let assign13710_e19641: f64 = (p.p889 * locals.var_deltemp);
                let assign13710_e19642: f64 = (1.0 + assign13710_e19641);
                let assign13710_e19644: f64 = (assign13710_e19642 - 1e-6);
                let assign13710_e19645: f64 = (assign13710_e19637 / assign13710_e19644);
                (assign13710_e19645, (-((assign13710_e19637 * (p.p889 * locals.var_deltemp_dn4)) / (assign13710_e19644 * assign13710_e19644))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13710_e19647, assign13710_e19647_d_n4,)
    }
};
        let assign13710_e19649: f64 = (p.p701 * assign13710_e19648);
        locals.var_cjs_t = assign13710_e19649;
        locals.var_cjs_t_dn4 = (p.p701 * assign13710_e19648_d_n4);
        locals.var_cjs_t_rv = 0.0;

        let assign13720_e19654: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19655: f64 = (1.0 + assign13720_e19654);
        let assign13720_e19657: f64 = (assign13720_e19655 - 1e-6);
        let assign13720_e19659: f64 = (-10000.0);
        let assign13720_e19661: f64 = (assign13720_e19659 * 0.001);
        let (assign13720_e19722, assign13720_e19722_d_n4,) = {
    if (!(assign13720_e19657 < assign13720_e19661)) {
        let assign13720_e19668: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19669: f64 = (1.0 + assign13720_e19668);
        let assign13720_e19671: f64 = (assign13720_e19669 - 1e-6);
        let assign13720_e19675: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19676: f64 = (1.0 + assign13720_e19675);
        let assign13720_e19678: f64 = (assign13720_e19676 - 1e-6);
        let assign13720_e19682: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19683: f64 = (1.0 + assign13720_e19682);
        let assign13720_e19685: f64 = (assign13720_e19683 - 1e-6);
        let assign13720_e19686: f64 = (assign13720_e19678 * assign13720_e19685);
        let assign13720_e19689: f64 = (4.0 * 0.001);
        let assign13720_e19691: f64 = (assign13720_e19689 * 0.001);
        let assign13720_e19692: f64 = (assign13720_e19686 + assign13720_e19691);
        let assign13720_e19693: f64 = (assign13720_e19692).sqrt();
        let assign13720_e19694: f64 = (assign13720_e19671 + assign13720_e19693);
        let assign13720_e19695: f64 = (0.5 * assign13720_e19694);
        (assign13720_e19695, (0.5 * ((p.p889 * locals.var_deltemp_dn4) + ((((p.p889 * locals.var_deltemp_dn4) * assign13720_e19685) + (assign13720_e19678 * (p.p889 * locals.var_deltemp_dn4))) / (2.0 * assign13720_e19693)))),)
    } else {
        let assign13720_e19699: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19700: f64 = (1.0 + assign13720_e19699);
        let assign13720_e19702: f64 = (assign13720_e19700 - 1e-6);
        let assign13720_e19704: f64 = (-10000.0);
        let assign13720_e19706: f64 = (assign13720_e19704 * 0.001);
        let (assign13720_e19721, assign13720_e19721_d_n4,) = {
            if (assign13720_e19702 < assign13720_e19706) {
                let assign13720_e19709: f64 = (-0.001);
                let assign13720_e19711: f64 = (assign13720_e19709 * 0.001);
                let assign13720_e19715: f64 = (p.p889 * locals.var_deltemp);
                let assign13720_e19716: f64 = (1.0 + assign13720_e19715);
                let assign13720_e19718: f64 = (assign13720_e19716 - 1e-6);
                let assign13720_e19719: f64 = (assign13720_e19711 / assign13720_e19718);
                (assign13720_e19719, (-((assign13720_e19711 * (p.p889 * locals.var_deltemp_dn4)) / (assign13720_e19718 * assign13720_e19718))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13720_e19721, assign13720_e19721_d_n4,)
    }
};
        let assign13720_e19723: f64 = (p.p702 * assign13720_e19722);
        locals.var_cjd_t = assign13720_e19723;
        locals.var_cjd_t_dn4 = (p.p702 * assign13720_e19722_d_n4);
        locals.var_cjd_t_rv = 0.0;

        let assign13730_e19728: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19729: f64 = (1.0 + assign13730_e19728);
        let assign13730_e19731: f64 = (assign13730_e19729 - 1e-6);
        let assign13730_e19733: f64 = (-10000.0);
        let assign13730_e19735: f64 = (assign13730_e19733 * 0.001);
        let (assign13730_e19796, assign13730_e19796_d_n4,) = {
    if (!(assign13730_e19731 < assign13730_e19735)) {
        let assign13730_e19742: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19743: f64 = (1.0 + assign13730_e19742);
        let assign13730_e19745: f64 = (assign13730_e19743 - 1e-6);
        let assign13730_e19749: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19750: f64 = (1.0 + assign13730_e19749);
        let assign13730_e19752: f64 = (assign13730_e19750 - 1e-6);
        let assign13730_e19756: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19757: f64 = (1.0 + assign13730_e19756);
        let assign13730_e19759: f64 = (assign13730_e19757 - 1e-6);
        let assign13730_e19760: f64 = (assign13730_e19752 * assign13730_e19759);
        let assign13730_e19763: f64 = (4.0 * 0.001);
        let assign13730_e19765: f64 = (assign13730_e19763 * 0.001);
        let assign13730_e19766: f64 = (assign13730_e19760 + assign13730_e19765);
        let assign13730_e19767: f64 = (assign13730_e19766).sqrt();
        let assign13730_e19768: f64 = (assign13730_e19745 + assign13730_e19767);
        let assign13730_e19769: f64 = (0.5 * assign13730_e19768);
        (assign13730_e19769, (0.5 * ((p.p890 * locals.var_deltemp_dn4) + ((((p.p890 * locals.var_deltemp_dn4) * assign13730_e19759) + (assign13730_e19752 * (p.p890 * locals.var_deltemp_dn4))) / (2.0 * assign13730_e19767)))),)
    } else {
        let assign13730_e19773: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19774: f64 = (1.0 + assign13730_e19773);
        let assign13730_e19776: f64 = (assign13730_e19774 - 1e-6);
        let assign13730_e19778: f64 = (-10000.0);
        let assign13730_e19780: f64 = (assign13730_e19778 * 0.001);
        let (assign13730_e19795, assign13730_e19795_d_n4,) = {
            if (assign13730_e19776 < assign13730_e19780) {
                let assign13730_e19783: f64 = (-0.001);
                let assign13730_e19785: f64 = (assign13730_e19783 * 0.001);
                let assign13730_e19789: f64 = (p.p890 * locals.var_deltemp);
                let assign13730_e19790: f64 = (1.0 + assign13730_e19789);
                let assign13730_e19792: f64 = (assign13730_e19790 - 1e-6);
                let assign13730_e19793: f64 = (assign13730_e19785 / assign13730_e19792);
                (assign13730_e19793, (-((assign13730_e19785 * (p.p890 * locals.var_deltemp_dn4)) / (assign13730_e19792 * assign13730_e19792))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13730_e19795, assign13730_e19795_d_n4,)
    }
};
        let assign13730_e19797: f64 = (p.p703 * assign13730_e19796);
        locals.var_cjsws_t = assign13730_e19797;
        locals.var_cjsws_t_dn4 = (p.p703 * assign13730_e19796_d_n4);
        locals.var_cjsws_t_rv = 0.0;

        let assign13740_e19802: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19803: f64 = (1.0 + assign13740_e19802);
        let assign13740_e19805: f64 = (assign13740_e19803 - 1e-6);
        let assign13740_e19807: f64 = (-10000.0);
        let assign13740_e19809: f64 = (assign13740_e19807 * 0.001);
        let (assign13740_e19870, assign13740_e19870_d_n4,) = {
    if (!(assign13740_e19805 < assign13740_e19809)) {
        let assign13740_e19816: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19817: f64 = (1.0 + assign13740_e19816);
        let assign13740_e19819: f64 = (assign13740_e19817 - 1e-6);
        let assign13740_e19823: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19824: f64 = (1.0 + assign13740_e19823);
        let assign13740_e19826: f64 = (assign13740_e19824 - 1e-6);
        let assign13740_e19830: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19831: f64 = (1.0 + assign13740_e19830);
        let assign13740_e19833: f64 = (assign13740_e19831 - 1e-6);
        let assign13740_e19834: f64 = (assign13740_e19826 * assign13740_e19833);
        let assign13740_e19837: f64 = (4.0 * 0.001);
        let assign13740_e19839: f64 = (assign13740_e19837 * 0.001);
        let assign13740_e19840: f64 = (assign13740_e19834 + assign13740_e19839);
        let assign13740_e19841: f64 = (assign13740_e19840).sqrt();
        let assign13740_e19842: f64 = (assign13740_e19819 + assign13740_e19841);
        let assign13740_e19843: f64 = (0.5 * assign13740_e19842);
        (assign13740_e19843, (0.5 * ((p.p890 * locals.var_deltemp_dn4) + ((((p.p890 * locals.var_deltemp_dn4) * assign13740_e19833) + (assign13740_e19826 * (p.p890 * locals.var_deltemp_dn4))) / (2.0 * assign13740_e19841)))),)
    } else {
        let assign13740_e19847: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19848: f64 = (1.0 + assign13740_e19847);
        let assign13740_e19850: f64 = (assign13740_e19848 - 1e-6);
        let assign13740_e19852: f64 = (-10000.0);
        let assign13740_e19854: f64 = (assign13740_e19852 * 0.001);
        let (assign13740_e19869, assign13740_e19869_d_n4,) = {
            if (assign13740_e19850 < assign13740_e19854) {
                let assign13740_e19857: f64 = (-0.001);
                let assign13740_e19859: f64 = (assign13740_e19857 * 0.001);
                let assign13740_e19863: f64 = (p.p890 * locals.var_deltemp);
                let assign13740_e19864: f64 = (1.0 + assign13740_e19863);
                let assign13740_e19866: f64 = (assign13740_e19864 - 1e-6);
                let assign13740_e19867: f64 = (assign13740_e19859 / assign13740_e19866);
                (assign13740_e19867, (-((assign13740_e19859 * (p.p890 * locals.var_deltemp_dn4)) / (assign13740_e19866 * assign13740_e19866))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13740_e19869, assign13740_e19869_d_n4,)
    }
};
        let assign13740_e19871: f64 = (p.p704 * assign13740_e19870);
        locals.var_cjswd_t = assign13740_e19871;
        locals.var_cjswd_t_dn4 = (p.p704 * assign13740_e19870_d_n4);
        locals.var_cjswd_t_rv = 0.0;

        let assign13750_e19876: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19877: f64 = (1.0 + assign13750_e19876);
        let assign13750_e19879: f64 = (assign13750_e19877 - 1e-6);
        let assign13750_e19881: f64 = (-10000.0);
        let assign13750_e19883: f64 = (assign13750_e19881 * 0.001);
        let (assign13750_e19944, assign13750_e19944_d_n4,) = {
    if (!(assign13750_e19879 < assign13750_e19883)) {
        let assign13750_e19890: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19891: f64 = (1.0 + assign13750_e19890);
        let assign13750_e19893: f64 = (assign13750_e19891 - 1e-6);
        let assign13750_e19897: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19898: f64 = (1.0 + assign13750_e19897);
        let assign13750_e19900: f64 = (assign13750_e19898 - 1e-6);
        let assign13750_e19904: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19905: f64 = (1.0 + assign13750_e19904);
        let assign13750_e19907: f64 = (assign13750_e19905 - 1e-6);
        let assign13750_e19908: f64 = (assign13750_e19900 * assign13750_e19907);
        let assign13750_e19911: f64 = (4.0 * 0.001);
        let assign13750_e19913: f64 = (assign13750_e19911 * 0.001);
        let assign13750_e19914: f64 = (assign13750_e19908 + assign13750_e19913);
        let assign13750_e19915: f64 = (assign13750_e19914).sqrt();
        let assign13750_e19916: f64 = (assign13750_e19893 + assign13750_e19915);
        let assign13750_e19917: f64 = (0.5 * assign13750_e19916);
        (assign13750_e19917, (0.5 * ((p.p891 * locals.var_deltemp_dn4) + ((((p.p891 * locals.var_deltemp_dn4) * assign13750_e19907) + (assign13750_e19900 * (p.p891 * locals.var_deltemp_dn4))) / (2.0 * assign13750_e19915)))),)
    } else {
        let assign13750_e19921: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19922: f64 = (1.0 + assign13750_e19921);
        let assign13750_e19924: f64 = (assign13750_e19922 - 1e-6);
        let assign13750_e19926: f64 = (-10000.0);
        let assign13750_e19928: f64 = (assign13750_e19926 * 0.001);
        let (assign13750_e19943, assign13750_e19943_d_n4,) = {
            if (assign13750_e19924 < assign13750_e19928) {
                let assign13750_e19931: f64 = (-0.001);
                let assign13750_e19933: f64 = (assign13750_e19931 * 0.001);
                let assign13750_e19937: f64 = (p.p891 * locals.var_deltemp);
                let assign13750_e19938: f64 = (1.0 + assign13750_e19937);
                let assign13750_e19940: f64 = (assign13750_e19938 - 1e-6);
                let assign13750_e19941: f64 = (assign13750_e19933 / assign13750_e19940);
                (assign13750_e19941, (-((assign13750_e19933 * (p.p891 * locals.var_deltemp_dn4)) / (assign13750_e19940 * assign13750_e19940))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13750_e19943, assign13750_e19943_d_n4,)
    }
};
        let assign13750_e19945: f64 = (p.p705 * assign13750_e19944);
        locals.var_cjswgs_t = assign13750_e19945;
        locals.var_cjswgs_t_dn4 = (p.p705 * assign13750_e19944_d_n4);
        locals.var_cjswgs_t_rv = 0.0;

        let assign13760_e19950: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19951: f64 = (1.0 + assign13760_e19950);
        let assign13760_e19953: f64 = (assign13760_e19951 - 1e-6);
        let assign13760_e19955: f64 = (-10000.0);
        let assign13760_e19957: f64 = (assign13760_e19955 * 0.001);
        let (assign13760_e20018, assign13760_e20018_d_n4,) = {
    if (!(assign13760_e19953 < assign13760_e19957)) {
        let assign13760_e19964: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19965: f64 = (1.0 + assign13760_e19964);
        let assign13760_e19967: f64 = (assign13760_e19965 - 1e-6);
        let assign13760_e19971: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19972: f64 = (1.0 + assign13760_e19971);
        let assign13760_e19974: f64 = (assign13760_e19972 - 1e-6);
        let assign13760_e19978: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19979: f64 = (1.0 + assign13760_e19978);
        let assign13760_e19981: f64 = (assign13760_e19979 - 1e-6);
        let assign13760_e19982: f64 = (assign13760_e19974 * assign13760_e19981);
        let assign13760_e19985: f64 = (4.0 * 0.001);
        let assign13760_e19987: f64 = (assign13760_e19985 * 0.001);
        let assign13760_e19988: f64 = (assign13760_e19982 + assign13760_e19987);
        let assign13760_e19989: f64 = (assign13760_e19988).sqrt();
        let assign13760_e19990: f64 = (assign13760_e19967 + assign13760_e19989);
        let assign13760_e19991: f64 = (0.5 * assign13760_e19990);
        (assign13760_e19991, (0.5 * ((p.p891 * locals.var_deltemp_dn4) + ((((p.p891 * locals.var_deltemp_dn4) * assign13760_e19981) + (assign13760_e19974 * (p.p891 * locals.var_deltemp_dn4))) / (2.0 * assign13760_e19989)))),)
    } else {
        let assign13760_e19995: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19996: f64 = (1.0 + assign13760_e19995);
        let assign13760_e19998: f64 = (assign13760_e19996 - 1e-6);
        let assign13760_e20000: f64 = (-10000.0);
        let assign13760_e20002: f64 = (assign13760_e20000 * 0.001);
        let (assign13760_e20017, assign13760_e20017_d_n4,) = {
            if (assign13760_e19998 < assign13760_e20002) {
                let assign13760_e20005: f64 = (-0.001);
                let assign13760_e20007: f64 = (assign13760_e20005 * 0.001);
                let assign13760_e20011: f64 = (p.p891 * locals.var_deltemp);
                let assign13760_e20012: f64 = (1.0 + assign13760_e20011);
                let assign13760_e20014: f64 = (assign13760_e20012 - 1e-6);
                let assign13760_e20015: f64 = (assign13760_e20007 / assign13760_e20014);
                (assign13760_e20015, (-((assign13760_e20007 * (p.p891 * locals.var_deltemp_dn4)) / (assign13760_e20014 * assign13760_e20014))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13760_e20017, assign13760_e20017_d_n4,)
    }
};
        let assign13760_e20019: f64 = (p.p706 * assign13760_e20018);
        locals.var_cjswgd_t = assign13760_e20019;
        locals.var_cjswgd_t_dn4 = (p.p706 * assign13760_e20018_d_n4);
        locals.var_cjswgd_t_rv = 0.0;

        let assign13770_e20023: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20024: f64 = (p.p707 - assign13770_e20023);
        let assign13770_e20026: f64 = (assign13770_e20024 - 0.01);
        let assign13770_e20028: f64 = (-10000.0);
        let assign13770_e20030: f64 = (assign13770_e20028 * 0.001);
        let (assign13770_e20091, assign13770_e20091_d_n4,) = {
    if (!(assign13770_e20026 < assign13770_e20030)) {
        let assign13770_e20037: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20038: f64 = (p.p707 - assign13770_e20037);
        let assign13770_e20040: f64 = (assign13770_e20038 - 0.01);
        let assign13770_e20044: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20045: f64 = (p.p707 - assign13770_e20044);
        let assign13770_e20047: f64 = (assign13770_e20045 - 0.01);
        let assign13770_e20051: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20052: f64 = (p.p707 - assign13770_e20051);
        let assign13770_e20054: f64 = (assign13770_e20052 - 0.01);
        let assign13770_e20055: f64 = (assign13770_e20047 * assign13770_e20054);
        let assign13770_e20058: f64 = (4.0 * 0.001);
        let assign13770_e20060: f64 = (assign13770_e20058 * 0.001);
        let assign13770_e20061: f64 = (assign13770_e20055 + assign13770_e20060);
        let assign13770_e20062: f64 = (assign13770_e20061).sqrt();
        let assign13770_e20063: f64 = (assign13770_e20040 + assign13770_e20062);
        let assign13770_e20064: f64 = (0.5 * assign13770_e20063);
        (assign13770_e20064, (0.5 * ((-(p.p892 * locals.var_deltemp_dn4)) + ((((-(p.p892 * locals.var_deltemp_dn4)) * assign13770_e20054) + (assign13770_e20047 * (-(p.p892 * locals.var_deltemp_dn4)))) / (2.0 * assign13770_e20062)))),)
    } else {
        let assign13770_e20068: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20069: f64 = (p.p707 - assign13770_e20068);
        let assign13770_e20071: f64 = (assign13770_e20069 - 0.01);
        let assign13770_e20073: f64 = (-10000.0);
        let assign13770_e20075: f64 = (assign13770_e20073 * 0.001);
        let (assign13770_e20090, assign13770_e20090_d_n4,) = {
            if (assign13770_e20071 < assign13770_e20075) {
                let assign13770_e20078: f64 = (-0.001);
                let assign13770_e20080: f64 = (assign13770_e20078 * 0.001);
                let assign13770_e20084: f64 = (p.p892 * locals.var_deltemp);
                let assign13770_e20085: f64 = (p.p707 - assign13770_e20084);
                let assign13770_e20087: f64 = (assign13770_e20085 - 0.01);
                let assign13770_e20088: f64 = (assign13770_e20080 / assign13770_e20087);
                (assign13770_e20088, (-((assign13770_e20080 * (-(p.p892 * locals.var_deltemp_dn4))) / (assign13770_e20087 * assign13770_e20087))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13770_e20090, assign13770_e20090_d_n4,)
    }
};
        let assign13770_e20093: f64 = (assign13770_e20091 + 0.01);
        locals.var_pbs_t = assign13770_e20093;
        locals.var_pbs_t_dn4 = assign13770_e20091_d_n4;
        locals.var_pbs_t_rv = 0.0;

        let assign13780_e20097: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20098: f64 = (p.p708 - assign13780_e20097);
        let assign13780_e20100: f64 = (assign13780_e20098 - 0.01);
        let assign13780_e20102: f64 = (-10000.0);
        let assign13780_e20104: f64 = (assign13780_e20102 * 0.001);
        let (assign13780_e20165, assign13780_e20165_d_n4,) = {
    if (!(assign13780_e20100 < assign13780_e20104)) {
        let assign13780_e20111: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20112: f64 = (p.p708 - assign13780_e20111);
        let assign13780_e20114: f64 = (assign13780_e20112 - 0.01);
        let assign13780_e20118: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20119: f64 = (p.p708 - assign13780_e20118);
        let assign13780_e20121: f64 = (assign13780_e20119 - 0.01);
        let assign13780_e20125: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20126: f64 = (p.p708 - assign13780_e20125);
        let assign13780_e20128: f64 = (assign13780_e20126 - 0.01);
        let assign13780_e20129: f64 = (assign13780_e20121 * assign13780_e20128);
        let assign13780_e20132: f64 = (4.0 * 0.001);
        let assign13780_e20134: f64 = (assign13780_e20132 * 0.001);
        let assign13780_e20135: f64 = (assign13780_e20129 + assign13780_e20134);
        let assign13780_e20136: f64 = (assign13780_e20135).sqrt();
        let assign13780_e20137: f64 = (assign13780_e20114 + assign13780_e20136);
        let assign13780_e20138: f64 = (0.5 * assign13780_e20137);
        (assign13780_e20138, (0.5 * ((-(p.p892 * locals.var_deltemp_dn4)) + ((((-(p.p892 * locals.var_deltemp_dn4)) * assign13780_e20128) + (assign13780_e20121 * (-(p.p892 * locals.var_deltemp_dn4)))) / (2.0 * assign13780_e20136)))),)
    } else {
        let assign13780_e20142: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20143: f64 = (p.p708 - assign13780_e20142);
        let assign13780_e20145: f64 = (assign13780_e20143 - 0.01);
        let assign13780_e20147: f64 = (-10000.0);
        let assign13780_e20149: f64 = (assign13780_e20147 * 0.001);
        let (assign13780_e20164, assign13780_e20164_d_n4,) = {
            if (assign13780_e20145 < assign13780_e20149) {
                let assign13780_e20152: f64 = (-0.001);
                let assign13780_e20154: f64 = (assign13780_e20152 * 0.001);
                let assign13780_e20158: f64 = (p.p892 * locals.var_deltemp);
                let assign13780_e20159: f64 = (p.p708 - assign13780_e20158);
                let assign13780_e20161: f64 = (assign13780_e20159 - 0.01);
                let assign13780_e20162: f64 = (assign13780_e20154 / assign13780_e20161);
                (assign13780_e20162, (-((assign13780_e20154 * (-(p.p892 * locals.var_deltemp_dn4))) / (assign13780_e20161 * assign13780_e20161))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13780_e20164, assign13780_e20164_d_n4,)
    }
};
        let assign13780_e20167: f64 = (assign13780_e20165 + 0.01);
        locals.var_pbd_t = assign13780_e20167;
        locals.var_pbd_t_dn4 = assign13780_e20165_d_n4;
        locals.var_pbd_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13790_e20171: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20172: f64 = (p.p709 - assign13790_e20171);
        let assign13790_e20174: f64 = (assign13790_e20172 - 0.01);
        let assign13790_e20176: f64 = (-10000.0);
        let assign13790_e20178: f64 = (assign13790_e20176 * 0.001);
        let (assign13790_e20239, assign13790_e20239_d_n4,) = {
    if (!(assign13790_e20174 < assign13790_e20178)) {
        let assign13790_e20185: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20186: f64 = (p.p709 - assign13790_e20185);
        let assign13790_e20188: f64 = (assign13790_e20186 - 0.01);
        let assign13790_e20192: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20193: f64 = (p.p709 - assign13790_e20192);
        let assign13790_e20195: f64 = (assign13790_e20193 - 0.01);
        let assign13790_e20199: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20200: f64 = (p.p709 - assign13790_e20199);
        let assign13790_e20202: f64 = (assign13790_e20200 - 0.01);
        let assign13790_e20203: f64 = (assign13790_e20195 * assign13790_e20202);
        let assign13790_e20206: f64 = (4.0 * 0.001);
        let assign13790_e20208: f64 = (assign13790_e20206 * 0.001);
        let assign13790_e20209: f64 = (assign13790_e20203 + assign13790_e20208);
        let assign13790_e20210: f64 = (assign13790_e20209).sqrt();
        let assign13790_e20211: f64 = (assign13790_e20188 + assign13790_e20210);
        let assign13790_e20212: f64 = (0.5 * assign13790_e20211);
        (assign13790_e20212, (0.5 * ((-(p.p893 * locals.var_deltemp_dn4)) + ((((-(p.p893 * locals.var_deltemp_dn4)) * assign13790_e20202) + (assign13790_e20195 * (-(p.p893 * locals.var_deltemp_dn4)))) / (2.0 * assign13790_e20210)))),)
    } else {
        let assign13790_e20216: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20217: f64 = (p.p709 - assign13790_e20216);
        let assign13790_e20219: f64 = (assign13790_e20217 - 0.01);
        let assign13790_e20221: f64 = (-10000.0);
        let assign13790_e20223: f64 = (assign13790_e20221 * 0.001);
        let (assign13790_e20238, assign13790_e20238_d_n4,) = {
            if (assign13790_e20219 < assign13790_e20223) {
                let assign13790_e20226: f64 = (-0.001);
                let assign13790_e20228: f64 = (assign13790_e20226 * 0.001);
                let assign13790_e20232: f64 = (p.p893 * locals.var_deltemp);
                let assign13790_e20233: f64 = (p.p709 - assign13790_e20232);
                let assign13790_e20235: f64 = (assign13790_e20233 - 0.01);
                let assign13790_e20236: f64 = (assign13790_e20228 / assign13790_e20235);
                (assign13790_e20236, (-((assign13790_e20228 * (-(p.p893 * locals.var_deltemp_dn4))) / (assign13790_e20235 * assign13790_e20235))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13790_e20238, assign13790_e20238_d_n4,)
    }
};
        let assign13790_e20241: f64 = (assign13790_e20239 + 0.01);
        locals.var_pbsws_t = assign13790_e20241;
        locals.var_pbsws_t_dn4 = assign13790_e20239_d_n4;
        locals.var_pbsws_t_rv = 0.0;

        let assign13800_e20245: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20246: f64 = (p.p710 - assign13800_e20245);
        let assign13800_e20248: f64 = (assign13800_e20246 - 0.01);
        let assign13800_e20250: f64 = (-10000.0);
        let assign13800_e20252: f64 = (assign13800_e20250 * 0.001);
        let (assign13800_e20313, assign13800_e20313_d_n4,) = {
    if (!(assign13800_e20248 < assign13800_e20252)) {
        let assign13800_e20259: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20260: f64 = (p.p710 - assign13800_e20259);
        let assign13800_e20262: f64 = (assign13800_e20260 - 0.01);
        let assign13800_e20266: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20267: f64 = (p.p710 - assign13800_e20266);
        let assign13800_e20269: f64 = (assign13800_e20267 - 0.01);
        let assign13800_e20273: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20274: f64 = (p.p710 - assign13800_e20273);
        let assign13800_e20276: f64 = (assign13800_e20274 - 0.01);
        let assign13800_e20277: f64 = (assign13800_e20269 * assign13800_e20276);
        let assign13800_e20280: f64 = (4.0 * 0.001);
        let assign13800_e20282: f64 = (assign13800_e20280 * 0.001);
        let assign13800_e20283: f64 = (assign13800_e20277 + assign13800_e20282);
        let assign13800_e20284: f64 = (assign13800_e20283).sqrt();
        let assign13800_e20285: f64 = (assign13800_e20262 + assign13800_e20284);
        let assign13800_e20286: f64 = (0.5 * assign13800_e20285);
        (assign13800_e20286, (0.5 * ((-(p.p893 * locals.var_deltemp_dn4)) + ((((-(p.p893 * locals.var_deltemp_dn4)) * assign13800_e20276) + (assign13800_e20269 * (-(p.p893 * locals.var_deltemp_dn4)))) / (2.0 * assign13800_e20284)))),)
    } else {
        let assign13800_e20290: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20291: f64 = (p.p710 - assign13800_e20290);
        let assign13800_e20293: f64 = (assign13800_e20291 - 0.01);
        let assign13800_e20295: f64 = (-10000.0);
        let assign13800_e20297: f64 = (assign13800_e20295 * 0.001);
        let (assign13800_e20312, assign13800_e20312_d_n4,) = {
            if (assign13800_e20293 < assign13800_e20297) {
                let assign13800_e20300: f64 = (-0.001);
                let assign13800_e20302: f64 = (assign13800_e20300 * 0.001);
                let assign13800_e20306: f64 = (p.p893 * locals.var_deltemp);
                let assign13800_e20307: f64 = (p.p710 - assign13800_e20306);
                let assign13800_e20309: f64 = (assign13800_e20307 - 0.01);
                let assign13800_e20310: f64 = (assign13800_e20302 / assign13800_e20309);
                (assign13800_e20310, (-((assign13800_e20302 * (-(p.p893 * locals.var_deltemp_dn4))) / (assign13800_e20309 * assign13800_e20309))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13800_e20312, assign13800_e20312_d_n4,)
    }
};
        let assign13800_e20315: f64 = (assign13800_e20313 + 0.01);
        locals.var_pbswd_t = assign13800_e20315;
        locals.var_pbswd_t_dn4 = assign13800_e20313_d_n4;
        locals.var_pbswd_t_rv = 0.0;

        let assign13810_e20319: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20320: f64 = (p.p711 - assign13810_e20319);
        let assign13810_e20322: f64 = (assign13810_e20320 - 0.01);
        let assign13810_e20324: f64 = (-10000.0);
        let assign13810_e20326: f64 = (assign13810_e20324 * 0.001);
        let (assign13810_e20387, assign13810_e20387_d_n4,) = {
    if (!(assign13810_e20322 < assign13810_e20326)) {
        let assign13810_e20333: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20334: f64 = (p.p711 - assign13810_e20333);
        let assign13810_e20336: f64 = (assign13810_e20334 - 0.01);
        let assign13810_e20340: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20341: f64 = (p.p711 - assign13810_e20340);
        let assign13810_e20343: f64 = (assign13810_e20341 - 0.01);
        let assign13810_e20347: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20348: f64 = (p.p711 - assign13810_e20347);
        let assign13810_e20350: f64 = (assign13810_e20348 - 0.01);
        let assign13810_e20351: f64 = (assign13810_e20343 * assign13810_e20350);
        let assign13810_e20354: f64 = (4.0 * 0.001);
        let assign13810_e20356: f64 = (assign13810_e20354 * 0.001);
        let assign13810_e20357: f64 = (assign13810_e20351 + assign13810_e20356);
        let assign13810_e20358: f64 = (assign13810_e20357).sqrt();
        let assign13810_e20359: f64 = (assign13810_e20336 + assign13810_e20358);
        let assign13810_e20360: f64 = (0.5 * assign13810_e20359);
        (assign13810_e20360, (0.5 * ((-(p.p894 * locals.var_deltemp_dn4)) + ((((-(p.p894 * locals.var_deltemp_dn4)) * assign13810_e20350) + (assign13810_e20343 * (-(p.p894 * locals.var_deltemp_dn4)))) / (2.0 * assign13810_e20358)))),)
    } else {
        let assign13810_e20364: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20365: f64 = (p.p711 - assign13810_e20364);
        let assign13810_e20367: f64 = (assign13810_e20365 - 0.01);
        let assign13810_e20369: f64 = (-10000.0);
        let assign13810_e20371: f64 = (assign13810_e20369 * 0.001);
        let (assign13810_e20386, assign13810_e20386_d_n4,) = {
            if (assign13810_e20367 < assign13810_e20371) {
                let assign13810_e20374: f64 = (-0.001);
                let assign13810_e20376: f64 = (assign13810_e20374 * 0.001);
                let assign13810_e20380: f64 = (p.p894 * locals.var_deltemp);
                let assign13810_e20381: f64 = (p.p711 - assign13810_e20380);
                let assign13810_e20383: f64 = (assign13810_e20381 - 0.01);
                let assign13810_e20384: f64 = (assign13810_e20376 / assign13810_e20383);
                (assign13810_e20384, (-((assign13810_e20376 * (-(p.p894 * locals.var_deltemp_dn4))) / (assign13810_e20383 * assign13810_e20383))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13810_e20386, assign13810_e20386_d_n4,)
    }
};
        let assign13810_e20389: f64 = (assign13810_e20387 + 0.01);
        locals.var_pbswgs_t = assign13810_e20389;
        locals.var_pbswgs_t_dn4 = assign13810_e20387_d_n4;
        locals.var_pbswgs_t_rv = 0.0;

        let assign13820_e20393: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20394: f64 = (p.p712 - assign13820_e20393);
        let assign13820_e20396: f64 = (assign13820_e20394 - 0.01);
        let assign13820_e20398: f64 = (-10000.0);
        let assign13820_e20400: f64 = (assign13820_e20398 * 0.001);
        let (assign13820_e20461, assign13820_e20461_d_n4,) = {
    if (!(assign13820_e20396 < assign13820_e20400)) {
        let assign13820_e20407: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20408: f64 = (p.p712 - assign13820_e20407);
        let assign13820_e20410: f64 = (assign13820_e20408 - 0.01);
        let assign13820_e20414: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20415: f64 = (p.p712 - assign13820_e20414);
        let assign13820_e20417: f64 = (assign13820_e20415 - 0.01);
        let assign13820_e20421: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20422: f64 = (p.p712 - assign13820_e20421);
        let assign13820_e20424: f64 = (assign13820_e20422 - 0.01);
        let assign13820_e20425: f64 = (assign13820_e20417 * assign13820_e20424);
        let assign13820_e20428: f64 = (4.0 * 0.001);
        let assign13820_e20430: f64 = (assign13820_e20428 * 0.001);
        let assign13820_e20431: f64 = (assign13820_e20425 + assign13820_e20430);
        let assign13820_e20432: f64 = (assign13820_e20431).sqrt();
        let assign13820_e20433: f64 = (assign13820_e20410 + assign13820_e20432);
        let assign13820_e20434: f64 = (0.5 * assign13820_e20433);
        (assign13820_e20434, (0.5 * ((-(p.p894 * locals.var_deltemp_dn4)) + ((((-(p.p894 * locals.var_deltemp_dn4)) * assign13820_e20424) + (assign13820_e20417 * (-(p.p894 * locals.var_deltemp_dn4)))) / (2.0 * assign13820_e20432)))),)
    } else {
        let assign13820_e20438: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20439: f64 = (p.p712 - assign13820_e20438);
        let assign13820_e20441: f64 = (assign13820_e20439 - 0.01);
        let assign13820_e20443: f64 = (-10000.0);
        let assign13820_e20445: f64 = (assign13820_e20443 * 0.001);
        let (assign13820_e20460, assign13820_e20460_d_n4,) = {
            if (assign13820_e20441 < assign13820_e20445) {
                let assign13820_e20448: f64 = (-0.001);
                let assign13820_e20450: f64 = (assign13820_e20448 * 0.001);
                let assign13820_e20454: f64 = (p.p894 * locals.var_deltemp);
                let assign13820_e20455: f64 = (p.p712 - assign13820_e20454);
                let assign13820_e20457: f64 = (assign13820_e20455 - 0.01);
                let assign13820_e20458: f64 = (assign13820_e20450 / assign13820_e20457);
                (assign13820_e20458, (-((assign13820_e20450 * (-(p.p894 * locals.var_deltemp_dn4))) / (assign13820_e20457 * assign13820_e20457))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13820_e20460, assign13820_e20460_d_n4,)
    }
};
        let assign13820_e20463: f64 = (assign13820_e20461 + 0.01);
        locals.var_pbswgd_t = assign13820_e20463;
        locals.var_pbswgd_t_dn4 = assign13820_e20461_d_n4;
        locals.var_pbswgd_t_rv = 0.0;

        let assign13830_e20466: f64 = (locals.var_eg0 / locals.var_vtm0);
        let assign13830_e20469: f64 = (locals.var_eg / locals.var_vtm);
        let assign13830_e20470: f64 = (assign13830_e20466 - assign13830_e20469);
        locals.var_t0 = assign13830_e20470;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = (-(((locals.var_eg_dn4 * locals.var_vtm) - (locals.var_eg * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)));
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign13840_e20473: f64 = (locals.var_tratio).max(1e-38);
        let assign13840_e20474: f64 = (assign13840_e20473).ln();
        locals.var_t1 = assign13840_e20474;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = (if locals.var_tratio >= 1e-38 { locals.var_tratio_dn4 } else { 0.0 } / assign13840_e20473);
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign13850_e20478: f64 = (p.p895 * locals.var_t1);
        let assign13850_e20479: f64 = (locals.var_t0 + assign13850_e20478);
        let assign13850_e20481: f64 = (assign13850_e20479 / p.p725);
        let assign13850_e20482: f64 = { let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        locals.var_t3 = assign13850_e20482;
        locals.var_t3_dn0 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 + (p.p895 * locals.var_t1_dn0)) / p.p725));
        locals.var_t3_dn2 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 + (p.p895 * locals.var_t1_dn2)) / p.p725));
        locals.var_t3_dn3 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 + (p.p895 * locals.var_t1_dn3)) / p.p725));
        locals.var_t3_dn4 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 + (p.p895 * locals.var_t1_dn4)) / p.p725));
        locals.var_t3_dn5 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 + (p.p895 * locals.var_t1_dn5)) / p.p725));
        locals.var_t3_dn6 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 + (p.p895 * locals.var_t1_dn6)) / p.p725));
        locals.var_t3_dn7 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 + (p.p895 * locals.var_t1_dn7)) / p.p725));
        locals.var_t3_dn8 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 + (p.p895 * locals.var_t1_dn8)) / p.p725));
        locals.var_t3_dn9 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 + (p.p895 * locals.var_t1_dn9)) / p.p725));
        locals.var_t3_dn10 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 + (p.p895 * locals.var_t1_dn10)) / p.p725));
        locals.var_t3_dn11 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 + (p.p895 * locals.var_t1_dn11)) / p.p725));
        locals.var_t3_dn12 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 + (p.p895 * locals.var_t1_dn12)) / p.p725));
        locals.var_t3_dn13 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 + (p.p895 * locals.var_t1_dn13)) / p.p725));
        locals.var_t3_dn14 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 + (p.p895 * locals.var_t1_dn14)) / p.p725));
        locals.var_t3_rv = 0.0;

        let assign13860_e20485: f64 = (p.p719 * locals.var_t3);
        locals.var_jss_t = assign13860_e20485;
        locals.var_jss_t_dn0 = (p.p719 * locals.var_t3_dn0);
        locals.var_jss_t_dn2 = (p.p719 * locals.var_t3_dn2);
        locals.var_jss_t_dn3 = (p.p719 * locals.var_t3_dn3);
        locals.var_jss_t_dn4 = (p.p719 * locals.var_t3_dn4);
        locals.var_jss_t_dn5 = (p.p719 * locals.var_t3_dn5);
        locals.var_jss_t_dn6 = (p.p719 * locals.var_t3_dn6);
        locals.var_jss_t_dn7 = (p.p719 * locals.var_t3_dn7);
        locals.var_jss_t_dn8 = (p.p719 * locals.var_t3_dn8);
        locals.var_jss_t_dn9 = (p.p719 * locals.var_t3_dn9);
        locals.var_jss_t_dn10 = (p.p719 * locals.var_t3_dn10);
        locals.var_jss_t_dn11 = (p.p719 * locals.var_t3_dn11);
        locals.var_jss_t_dn12 = (p.p719 * locals.var_t3_dn12);
        locals.var_jss_t_dn13 = (p.p719 * locals.var_t3_dn13);
        locals.var_jss_t_dn14 = (p.p719 * locals.var_t3_dn14);
        locals.var_jss_t_rv = 0.0;

        let assign13870_e20488: f64 = (p.p721 * locals.var_t3);
        locals.var_jsws_t = assign13870_e20488;
        locals.var_jsws_t_dn0 = (p.p721 * locals.var_t3_dn0);
        locals.var_jsws_t_dn2 = (p.p721 * locals.var_t3_dn2);
        locals.var_jsws_t_dn3 = (p.p721 * locals.var_t3_dn3);
        locals.var_jsws_t_dn4 = (p.p721 * locals.var_t3_dn4);
        locals.var_jsws_t_dn5 = (p.p721 * locals.var_t3_dn5);
        locals.var_jsws_t_dn6 = (p.p721 * locals.var_t3_dn6);
        locals.var_jsws_t_dn7 = (p.p721 * locals.var_t3_dn7);
        locals.var_jsws_t_dn8 = (p.p721 * locals.var_t3_dn8);
        locals.var_jsws_t_dn9 = (p.p721 * locals.var_t3_dn9);
        locals.var_jsws_t_dn10 = (p.p721 * locals.var_t3_dn10);
        locals.var_jsws_t_dn11 = (p.p721 * locals.var_t3_dn11);
        locals.var_jsws_t_dn12 = (p.p721 * locals.var_t3_dn12);
        locals.var_jsws_t_dn13 = (p.p721 * locals.var_t3_dn13);
        locals.var_jsws_t_dn14 = (p.p721 * locals.var_t3_dn14);
        locals.var_jsws_t_rv = 0.0;

        let assign13880_e20491: f64 = (p.p723 * locals.var_t3);
        locals.var_jswgs_t = assign13880_e20491;
        locals.var_jswgs_t_dn0 = (p.p723 * locals.var_t3_dn0);
        locals.var_jswgs_t_dn2 = (p.p723 * locals.var_t3_dn2);
        locals.var_jswgs_t_dn3 = (p.p723 * locals.var_t3_dn3);
        locals.var_jswgs_t_dn4 = (p.p723 * locals.var_t3_dn4);
        locals.var_jswgs_t_dn5 = (p.p723 * locals.var_t3_dn5);
        locals.var_jswgs_t_dn6 = (p.p723 * locals.var_t3_dn6);
        locals.var_jswgs_t_dn7 = (p.p723 * locals.var_t3_dn7);
        locals.var_jswgs_t_dn8 = (p.p723 * locals.var_t3_dn8);
        locals.var_jswgs_t_dn9 = (p.p723 * locals.var_t3_dn9);
        locals.var_jswgs_t_dn10 = (p.p723 * locals.var_t3_dn10);
        locals.var_jswgs_t_dn11 = (p.p723 * locals.var_t3_dn11);
        locals.var_jswgs_t_dn12 = (p.p723 * locals.var_t3_dn12);
        locals.var_jswgs_t_dn13 = (p.p723 * locals.var_t3_dn13);
        locals.var_jswgs_t_dn14 = (p.p723 * locals.var_t3_dn14);
        locals.var_jswgs_t_rv = 0.0;

        let assign13890_e20495: f64 = (p.p896 * locals.var_t1);
        let assign13890_e20496: f64 = (locals.var_t0 + assign13890_e20495);
        let assign13890_e20498: f64 = (assign13890_e20496 / p.p726);
        let assign13890_e20499: f64 = { let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        locals.var_t3 = assign13890_e20499;
        locals.var_t3_dn0 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 + (p.p896 * locals.var_t1_dn0)) / p.p726));
        locals.var_t3_dn2 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 + (p.p896 * locals.var_t1_dn2)) / p.p726));
        locals.var_t3_dn3 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 + (p.p896 * locals.var_t1_dn3)) / p.p726));
        locals.var_t3_dn4 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 + (p.p896 * locals.var_t1_dn4)) / p.p726));
        locals.var_t3_dn5 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 + (p.p896 * locals.var_t1_dn5)) / p.p726));
        locals.var_t3_dn6 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 + (p.p896 * locals.var_t1_dn6)) / p.p726));
        locals.var_t3_dn7 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 + (p.p896 * locals.var_t1_dn7)) / p.p726));
        locals.var_t3_dn8 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 + (p.p896 * locals.var_t1_dn8)) / p.p726));
        locals.var_t3_dn9 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 + (p.p896 * locals.var_t1_dn9)) / p.p726));
        locals.var_t3_dn10 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 + (p.p896 * locals.var_t1_dn10)) / p.p726));
        locals.var_t3_dn11 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 + (p.p896 * locals.var_t1_dn11)) / p.p726));
        locals.var_t3_dn12 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 + (p.p896 * locals.var_t1_dn12)) / p.p726));
        locals.var_t3_dn13 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 + (p.p896 * locals.var_t1_dn13)) / p.p726));
        locals.var_t3_dn14 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 + (p.p896 * locals.var_t1_dn14)) / p.p726));
        locals.var_t3_rv = 0.0;

        let assign13900_e20502: f64 = (p.p720 * locals.var_t3);
        locals.var_jsd_t = assign13900_e20502;
        locals.var_jsd_t_dn0 = (p.p720 * locals.var_t3_dn0);
        locals.var_jsd_t_dn2 = (p.p720 * locals.var_t3_dn2);
        locals.var_jsd_t_dn3 = (p.p720 * locals.var_t3_dn3);
        locals.var_jsd_t_dn4 = (p.p720 * locals.var_t3_dn4);
        locals.var_jsd_t_dn5 = (p.p720 * locals.var_t3_dn5);
        locals.var_jsd_t_dn6 = (p.p720 * locals.var_t3_dn6);
        locals.var_jsd_t_dn7 = (p.p720 * locals.var_t3_dn7);
        locals.var_jsd_t_dn8 = (p.p720 * locals.var_t3_dn8);
        locals.var_jsd_t_dn9 = (p.p720 * locals.var_t3_dn9);
        locals.var_jsd_t_dn10 = (p.p720 * locals.var_t3_dn10);
        locals.var_jsd_t_dn11 = (p.p720 * locals.var_t3_dn11);
        locals.var_jsd_t_dn12 = (p.p720 * locals.var_t3_dn12);
        locals.var_jsd_t_dn13 = (p.p720 * locals.var_t3_dn13);
        locals.var_jsd_t_dn14 = (p.p720 * locals.var_t3_dn14);
        locals.var_jsd_t_rv = 0.0;

        let assign13910_e20505: f64 = (p.p722 * locals.var_t3);
        locals.var_jswd_t = assign13910_e20505;
        locals.var_jswd_t_dn0 = (p.p722 * locals.var_t3_dn0);
        locals.var_jswd_t_dn2 = (p.p722 * locals.var_t3_dn2);
        locals.var_jswd_t_dn3 = (p.p722 * locals.var_t3_dn3);
        locals.var_jswd_t_dn4 = (p.p722 * locals.var_t3_dn4);
        locals.var_jswd_t_dn5 = (p.p722 * locals.var_t3_dn5);
        locals.var_jswd_t_dn6 = (p.p722 * locals.var_t3_dn6);
        locals.var_jswd_t_dn7 = (p.p722 * locals.var_t3_dn7);
        locals.var_jswd_t_dn8 = (p.p722 * locals.var_t3_dn8);
        locals.var_jswd_t_dn9 = (p.p722 * locals.var_t3_dn9);
        locals.var_jswd_t_dn10 = (p.p722 * locals.var_t3_dn10);
        locals.var_jswd_t_dn11 = (p.p722 * locals.var_t3_dn11);
        locals.var_jswd_t_dn12 = (p.p722 * locals.var_t3_dn12);
        locals.var_jswd_t_dn13 = (p.p722 * locals.var_t3_dn13);
        locals.var_jswd_t_dn14 = (p.p722 * locals.var_t3_dn14);
        locals.var_jswd_t_rv = 0.0;

        let assign13920_e20508: f64 = (p.p724 * locals.var_t3);
        locals.var_jswgd_t = assign13920_e20508;
        locals.var_jswgd_t_dn0 = (p.p724 * locals.var_t3_dn0);
        locals.var_jswgd_t_dn2 = (p.p724 * locals.var_t3_dn2);
        locals.var_jswgd_t_dn3 = (p.p724 * locals.var_t3_dn3);
        locals.var_jswgd_t_dn4 = (p.p724 * locals.var_t3_dn4);
        locals.var_jswgd_t_dn5 = (p.p724 * locals.var_t3_dn5);
        locals.var_jswgd_t_dn6 = (p.p724 * locals.var_t3_dn6);
        locals.var_jswgd_t_dn7 = (p.p724 * locals.var_t3_dn7);
        locals.var_jswgd_t_dn8 = (p.p724 * locals.var_t3_dn8);
        locals.var_jswgd_t_dn9 = (p.p724 * locals.var_t3_dn9);
        locals.var_jswgd_t_dn10 = (p.p724 * locals.var_t3_dn10);
        locals.var_jswgd_t_dn11 = (p.p724 * locals.var_t3_dn11);
        locals.var_jswgd_t_dn12 = (p.p724 * locals.var_t3_dn12);
        locals.var_jswgd_t_dn13 = (p.p724 * locals.var_t3_dn13);
        locals.var_jswgd_t_dn14 = (p.p724 * locals.var_t3_dn14);
        locals.var_jswgd_t_rv = 0.0;

        let assign13930_e20512: f64 = (locals.var_eg0 * p.p897);
        let assign13930_e20515: f64 = (locals.var_tratio - 1.0);
        let assign13930_e20516: f64 = (assign13930_e20512 * assign13930_e20515);
        let assign13930_e20518: f64 = (assign13930_e20516 / locals.var_vtm);
        let assign13930_e20519: f64 = { let limited_exp_arg = assign13930_e20518; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13930_e20520: f64 = (p.p735 * assign13930_e20519);
        locals.var_jtss_t = assign13930_e20520;
        locals.var_jtss_t_dn4 = (p.p735 * ({ let limited_exp_arg = assign13930_e20518; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13930_e20512 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13930_e20516 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))));
        locals.var_jtss_t_rv = 0.0;

        let assign13940_e20524: f64 = (locals.var_eg0 * p.p899);
        let assign13940_e20527: f64 = (locals.var_tratio - 1.0);
        let assign13940_e20528: f64 = (assign13940_e20524 * assign13940_e20527);
        let assign13940_e20530: f64 = (assign13940_e20528 / locals.var_vtm);
        let assign13940_e20531: f64 = { let limited_exp_arg = assign13940_e20530; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13940_e20532: f64 = (p.p737 * assign13940_e20531);
        locals.var_jtssws_t = assign13940_e20532;
        locals.var_jtssws_t_dn4 = (p.p737 * ({ let limited_exp_arg = assign13940_e20530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13940_e20524 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13940_e20528 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))));
        locals.var_jtssws_t_rv = 0.0;

        let assign13950_e20536: f64 = (p.p741 / locals.var_weffcj);
        let assign13950_e20537: f64 = (assign13950_e20536).sqrt();
        let assign13950_e20539: f64 = (assign13950_e20537 + 1.0);
        let assign13950_e20540: f64 = (p.p739 * assign13950_e20539);
        let assign13950_e20543: f64 = (locals.var_eg0 * p.p901);
        let assign13950_e20546: f64 = (locals.var_tratio - 1.0);
        let assign13950_e20547: f64 = (assign13950_e20543 * assign13950_e20546);
        let assign13950_e20549: f64 = (assign13950_e20547 / locals.var_vtm);
        let assign13950_e20550: f64 = { let limited_exp_arg = assign13950_e20549; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13950_e20551: f64 = (assign13950_e20540 * assign13950_e20550);
        locals.var_jtsswgs_t = assign13950_e20551;
        locals.var_jtsswgs_t_dn4 = (assign13950_e20540 * ({ let limited_exp_arg = assign13950_e20549; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13950_e20543 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13950_e20547 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))));
        locals.var_jtsswgs_t_rv = 0.0;

        let assign13960_e20555: f64 = (locals.var_eg0 * p.p898);
        let assign13960_e20558: f64 = (locals.var_tratio - 1.0);
        let assign13960_e20559: f64 = (assign13960_e20555 * assign13960_e20558);
        let assign13960_e20561: f64 = (assign13960_e20559 / locals.var_vtm);
        let assign13960_e20562: f64 = { let limited_exp_arg = assign13960_e20561; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13960_e20563: f64 = (p.p736 * assign13960_e20562);
        locals.var_jtsd_t = assign13960_e20563;
        locals.var_jtsd_t_dn4 = (p.p736 * ({ let limited_exp_arg = assign13960_e20561; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13960_e20555 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13960_e20559 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))));
        locals.var_jtsd_t_rv = 0.0;

        let assign13970_e20567: f64 = (locals.var_eg0 * p.p900);
        let assign13970_e20570: f64 = (locals.var_tratio - 1.0);
        let assign13970_e20571: f64 = (assign13970_e20567 * assign13970_e20570);
        let assign13970_e20573: f64 = (assign13970_e20571 / locals.var_vtm);
        let assign13970_e20574: f64 = { let limited_exp_arg = assign13970_e20573; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13970_e20575: f64 = (p.p738 * assign13970_e20574);
        locals.var_jtsswd_t = assign13970_e20575;
        locals.var_jtsswd_t_dn4 = (p.p738 * ({ let limited_exp_arg = assign13970_e20573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13970_e20567 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13970_e20571 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))));
        locals.var_jtsswd_t_rv = 0.0;

        let assign13980_e20579: f64 = (p.p741 / locals.var_weffcj);
        let assign13980_e20580: f64 = (assign13980_e20579).sqrt();
        let assign13980_e20582: f64 = (assign13980_e20580 + 1.0);
        let assign13980_e20583: f64 = (p.p740 * assign13980_e20582);
        let assign13980_e20586: f64 = (locals.var_eg0 * p.p902);
        let assign13980_e20589: f64 = (locals.var_tratio - 1.0);
        let assign13980_e20590: f64 = (assign13980_e20586 * assign13980_e20589);
        let assign13980_e20592: f64 = (assign13980_e20590 / locals.var_vtm);
        let assign13980_e20593: f64 = { let limited_exp_arg = assign13980_e20592; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13980_e20594: f64 = (assign13980_e20583 * assign13980_e20593);
        locals.var_jtsswgd_t = assign13980_e20594;
        locals.var_jtsswgd_t_dn4 = (assign13980_e20583 * ({ let limited_exp_arg = assign13980_e20592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13980_e20586 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13980_e20590 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))));
        locals.var_jtsswgd_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13990_e20600: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20601: f64 = (p.p903 * assign13990_e20600);
        let assign13990_e20602: f64 = (1.0 + assign13990_e20601);
        let assign13990_e20603: f64 = (p.p742 * assign13990_e20602);
        let assign13990_e20605: f64 = (assign13990_e20603 - 0.01);
        let assign13990_e20607: f64 = (-10000.0);
        let assign13990_e20609: f64 = (assign13990_e20607 * 0.001);
        let (assign13990_e20690, assign13990_e20690_d_n4,) = {
    if (!(assign13990_e20605 < assign13990_e20609)) {
        let assign13990_e20618: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20619: f64 = (p.p903 * assign13990_e20618);
        let assign13990_e20620: f64 = (1.0 + assign13990_e20619);
        let assign13990_e20621: f64 = (p.p742 * assign13990_e20620);
        let assign13990_e20623: f64 = (assign13990_e20621 - 0.01);
        let assign13990_e20629: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20630: f64 = (p.p903 * assign13990_e20629);
        let assign13990_e20631: f64 = (1.0 + assign13990_e20630);
        let assign13990_e20632: f64 = (p.p742 * assign13990_e20631);
        let assign13990_e20634: f64 = (assign13990_e20632 - 0.01);
        let assign13990_e20640: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20641: f64 = (p.p903 * assign13990_e20640);
        let assign13990_e20642: f64 = (1.0 + assign13990_e20641);
        let assign13990_e20643: f64 = (p.p742 * assign13990_e20642);
        let assign13990_e20645: f64 = (assign13990_e20643 - 0.01);
        let assign13990_e20646: f64 = (assign13990_e20634 * assign13990_e20645);
        let assign13990_e20649: f64 = (4.0 * 0.001);
        let assign13990_e20651: f64 = (assign13990_e20649 * 0.001);
        let assign13990_e20652: f64 = (assign13990_e20646 + assign13990_e20651);
        let assign13990_e20653: f64 = (assign13990_e20652).sqrt();
        let assign13990_e20654: f64 = (assign13990_e20623 + assign13990_e20653);
        let assign13990_e20655: f64 = (0.5 * assign13990_e20654);
        (assign13990_e20655, (0.5 * ((p.p742 * (p.p903 * locals.var_tratio_dn4)) + ((((p.p742 * (p.p903 * locals.var_tratio_dn4)) * assign13990_e20645) + (assign13990_e20634 * (p.p742 * (p.p903 * locals.var_tratio_dn4)))) / (2.0 * assign13990_e20653)))),)
    } else {
        let assign13990_e20661: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20662: f64 = (p.p903 * assign13990_e20661);
        let assign13990_e20663: f64 = (1.0 + assign13990_e20662);
        let assign13990_e20664: f64 = (p.p742 * assign13990_e20663);
        let assign13990_e20666: f64 = (assign13990_e20664 - 0.01);
        let assign13990_e20668: f64 = (-10000.0);
        let assign13990_e20670: f64 = (assign13990_e20668 * 0.001);
        let (assign13990_e20689, assign13990_e20689_d_n4,) = {
            if (assign13990_e20666 < assign13990_e20670) {
                let assign13990_e20673: f64 = (-0.001);
                let assign13990_e20675: f64 = (assign13990_e20673 * 0.001);
                let assign13990_e20681: f64 = (locals.var_tratio - 1.0);
                let assign13990_e20682: f64 = (p.p903 * assign13990_e20681);
                let assign13990_e20683: f64 = (1.0 + assign13990_e20682);
                let assign13990_e20684: f64 = (p.p742 * assign13990_e20683);
                let assign13990_e20686: f64 = (assign13990_e20684 - 0.01);
                let assign13990_e20687: f64 = (assign13990_e20675 / assign13990_e20686);
                (assign13990_e20687, (-((assign13990_e20675 * (p.p742 * (p.p903 * locals.var_tratio_dn4))) / (assign13990_e20686 * assign13990_e20686))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13990_e20689, assign13990_e20689_d_n4,)
    }
};
        let assign13990_e20692: f64 = (assign13990_e20690 + 0.01);
        locals.var_njts_t = assign13990_e20692;
        locals.var_njts_t_dn4 = assign13990_e20690_d_n4;
        locals.var_njts_t_rv = 0.0;

        let assign14000_e20698: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20699: f64 = (p.p905 * assign14000_e20698);
        let assign14000_e20700: f64 = (1.0 + assign14000_e20699);
        let assign14000_e20701: f64 = (p.p744 * assign14000_e20700);
        let assign14000_e20703: f64 = (assign14000_e20701 - 0.01);
        let assign14000_e20705: f64 = (-10000.0);
        let assign14000_e20707: f64 = (assign14000_e20705 * 0.001);
        let (assign14000_e20788, assign14000_e20788_d_n4,) = {
    if (!(assign14000_e20703 < assign14000_e20707)) {
        let assign14000_e20716: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20717: f64 = (p.p905 * assign14000_e20716);
        let assign14000_e20718: f64 = (1.0 + assign14000_e20717);
        let assign14000_e20719: f64 = (p.p744 * assign14000_e20718);
        let assign14000_e20721: f64 = (assign14000_e20719 - 0.01);
        let assign14000_e20727: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20728: f64 = (p.p905 * assign14000_e20727);
        let assign14000_e20729: f64 = (1.0 + assign14000_e20728);
        let assign14000_e20730: f64 = (p.p744 * assign14000_e20729);
        let assign14000_e20732: f64 = (assign14000_e20730 - 0.01);
        let assign14000_e20738: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20739: f64 = (p.p905 * assign14000_e20738);
        let assign14000_e20740: f64 = (1.0 + assign14000_e20739);
        let assign14000_e20741: f64 = (p.p744 * assign14000_e20740);
        let assign14000_e20743: f64 = (assign14000_e20741 - 0.01);
        let assign14000_e20744: f64 = (assign14000_e20732 * assign14000_e20743);
        let assign14000_e20747: f64 = (4.0 * 0.001);
        let assign14000_e20749: f64 = (assign14000_e20747 * 0.001);
        let assign14000_e20750: f64 = (assign14000_e20744 + assign14000_e20749);
        let assign14000_e20751: f64 = (assign14000_e20750).sqrt();
        let assign14000_e20752: f64 = (assign14000_e20721 + assign14000_e20751);
        let assign14000_e20753: f64 = (0.5 * assign14000_e20752);
        (assign14000_e20753, (0.5 * ((p.p744 * (p.p905 * locals.var_tratio_dn4)) + ((((p.p744 * (p.p905 * locals.var_tratio_dn4)) * assign14000_e20743) + (assign14000_e20732 * (p.p744 * (p.p905 * locals.var_tratio_dn4)))) / (2.0 * assign14000_e20751)))),)
    } else {
        let assign14000_e20759: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20760: f64 = (p.p905 * assign14000_e20759);
        let assign14000_e20761: f64 = (1.0 + assign14000_e20760);
        let assign14000_e20762: f64 = (p.p744 * assign14000_e20761);
        let assign14000_e20764: f64 = (assign14000_e20762 - 0.01);
        let assign14000_e20766: f64 = (-10000.0);
        let assign14000_e20768: f64 = (assign14000_e20766 * 0.001);
        let (assign14000_e20787, assign14000_e20787_d_n4,) = {
            if (assign14000_e20764 < assign14000_e20768) {
                let assign14000_e20771: f64 = (-0.001);
                let assign14000_e20773: f64 = (assign14000_e20771 * 0.001);
                let assign14000_e20779: f64 = (locals.var_tratio - 1.0);
                let assign14000_e20780: f64 = (p.p905 * assign14000_e20779);
                let assign14000_e20781: f64 = (1.0 + assign14000_e20780);
                let assign14000_e20782: f64 = (p.p744 * assign14000_e20781);
                let assign14000_e20784: f64 = (assign14000_e20782 - 0.01);
                let assign14000_e20785: f64 = (assign14000_e20773 / assign14000_e20784);
                (assign14000_e20785, (-((assign14000_e20773 * (p.p744 * (p.p905 * locals.var_tratio_dn4))) / (assign14000_e20784 * assign14000_e20784))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14000_e20787, assign14000_e20787_d_n4,)
    }
};
        let assign14000_e20790: f64 = (assign14000_e20788 + 0.01);
        locals.var_njtssw_t = assign14000_e20790;
        locals.var_njtssw_t_dn4 = assign14000_e20788_d_n4;
        locals.var_njtssw_t_rv = 0.0;

        let assign14010_e20796: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20797: f64 = (p.p907 * assign14010_e20796);
        let assign14010_e20798: f64 = (1.0 + assign14010_e20797);
        let assign14010_e20799: f64 = (p.p746 * assign14010_e20798);
        let assign14010_e20801: f64 = (assign14010_e20799 - 0.01);
        let assign14010_e20803: f64 = (-10000.0);
        let assign14010_e20805: f64 = (assign14010_e20803 * 0.001);
        let (assign14010_e20886, assign14010_e20886_d_n4,) = {
    if (!(assign14010_e20801 < assign14010_e20805)) {
        let assign14010_e20814: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20815: f64 = (p.p907 * assign14010_e20814);
        let assign14010_e20816: f64 = (1.0 + assign14010_e20815);
        let assign14010_e20817: f64 = (p.p746 * assign14010_e20816);
        let assign14010_e20819: f64 = (assign14010_e20817 - 0.01);
        let assign14010_e20825: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20826: f64 = (p.p907 * assign14010_e20825);
        let assign14010_e20827: f64 = (1.0 + assign14010_e20826);
        let assign14010_e20828: f64 = (p.p746 * assign14010_e20827);
        let assign14010_e20830: f64 = (assign14010_e20828 - 0.01);
        let assign14010_e20836: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20837: f64 = (p.p907 * assign14010_e20836);
        let assign14010_e20838: f64 = (1.0 + assign14010_e20837);
        let assign14010_e20839: f64 = (p.p746 * assign14010_e20838);
        let assign14010_e20841: f64 = (assign14010_e20839 - 0.01);
        let assign14010_e20842: f64 = (assign14010_e20830 * assign14010_e20841);
        let assign14010_e20845: f64 = (4.0 * 0.001);
        let assign14010_e20847: f64 = (assign14010_e20845 * 0.001);
        let assign14010_e20848: f64 = (assign14010_e20842 + assign14010_e20847);
        let assign14010_e20849: f64 = (assign14010_e20848).sqrt();
        let assign14010_e20850: f64 = (assign14010_e20819 + assign14010_e20849);
        let assign14010_e20851: f64 = (0.5 * assign14010_e20850);
        (assign14010_e20851, (0.5 * ((p.p746 * (p.p907 * locals.var_tratio_dn4)) + ((((p.p746 * (p.p907 * locals.var_tratio_dn4)) * assign14010_e20841) + (assign14010_e20830 * (p.p746 * (p.p907 * locals.var_tratio_dn4)))) / (2.0 * assign14010_e20849)))),)
    } else {
        let assign14010_e20857: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20858: f64 = (p.p907 * assign14010_e20857);
        let assign14010_e20859: f64 = (1.0 + assign14010_e20858);
        let assign14010_e20860: f64 = (p.p746 * assign14010_e20859);
        let assign14010_e20862: f64 = (assign14010_e20860 - 0.01);
        let assign14010_e20864: f64 = (-10000.0);
        let assign14010_e20866: f64 = (assign14010_e20864 * 0.001);
        let (assign14010_e20885, assign14010_e20885_d_n4,) = {
            if (assign14010_e20862 < assign14010_e20866) {
                let assign14010_e20869: f64 = (-0.001);
                let assign14010_e20871: f64 = (assign14010_e20869 * 0.001);
                let assign14010_e20877: f64 = (locals.var_tratio - 1.0);
                let assign14010_e20878: f64 = (p.p907 * assign14010_e20877);
                let assign14010_e20879: f64 = (1.0 + assign14010_e20878);
                let assign14010_e20880: f64 = (p.p746 * assign14010_e20879);
                let assign14010_e20882: f64 = (assign14010_e20880 - 0.01);
                let assign14010_e20883: f64 = (assign14010_e20871 / assign14010_e20882);
                (assign14010_e20883, (-((assign14010_e20871 * (p.p746 * (p.p907 * locals.var_tratio_dn4))) / (assign14010_e20882 * assign14010_e20882))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14010_e20885, assign14010_e20885_d_n4,)
    }
};
        let assign14010_e20888: f64 = (assign14010_e20886 + 0.01);
        locals.var_njtsswg_t = assign14010_e20888;
        locals.var_njtsswg_t_dn4 = assign14010_e20886_d_n4;
        locals.var_njtsswg_t_rv = 0.0;

        let assign14020_e20894: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20895: f64 = (p.p904 * assign14020_e20894);
        let assign14020_e20896: f64 = (1.0 + assign14020_e20895);
        let assign14020_e20897: f64 = (p.p743 * assign14020_e20896);
        let assign14020_e20899: f64 = (assign14020_e20897 - 0.01);
        let assign14020_e20901: f64 = (-10000.0);
        let assign14020_e20903: f64 = (assign14020_e20901 * 0.001);
        let (assign14020_e20984, assign14020_e20984_d_n4,) = {
    if (!(assign14020_e20899 < assign14020_e20903)) {
        let assign14020_e20912: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20913: f64 = (p.p904 * assign14020_e20912);
        let assign14020_e20914: f64 = (1.0 + assign14020_e20913);
        let assign14020_e20915: f64 = (p.p743 * assign14020_e20914);
        let assign14020_e20917: f64 = (assign14020_e20915 - 0.01);
        let assign14020_e20923: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20924: f64 = (p.p904 * assign14020_e20923);
        let assign14020_e20925: f64 = (1.0 + assign14020_e20924);
        let assign14020_e20926: f64 = (p.p743 * assign14020_e20925);
        let assign14020_e20928: f64 = (assign14020_e20926 - 0.01);
        let assign14020_e20934: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20935: f64 = (p.p904 * assign14020_e20934);
        let assign14020_e20936: f64 = (1.0 + assign14020_e20935);
        let assign14020_e20937: f64 = (p.p743 * assign14020_e20936);
        let assign14020_e20939: f64 = (assign14020_e20937 - 0.01);
        let assign14020_e20940: f64 = (assign14020_e20928 * assign14020_e20939);
        let assign14020_e20943: f64 = (4.0 * 0.001);
        let assign14020_e20945: f64 = (assign14020_e20943 * 0.001);
        let assign14020_e20946: f64 = (assign14020_e20940 + assign14020_e20945);
        let assign14020_e20947: f64 = (assign14020_e20946).sqrt();
        let assign14020_e20948: f64 = (assign14020_e20917 + assign14020_e20947);
        let assign14020_e20949: f64 = (0.5 * assign14020_e20948);
        (assign14020_e20949, (0.5 * ((p.p743 * (p.p904 * locals.var_tratio_dn4)) + ((((p.p743 * (p.p904 * locals.var_tratio_dn4)) * assign14020_e20939) + (assign14020_e20928 * (p.p743 * (p.p904 * locals.var_tratio_dn4)))) / (2.0 * assign14020_e20947)))),)
    } else {
        let assign14020_e20955: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20956: f64 = (p.p904 * assign14020_e20955);
        let assign14020_e20957: f64 = (1.0 + assign14020_e20956);
        let assign14020_e20958: f64 = (p.p743 * assign14020_e20957);
        let assign14020_e20960: f64 = (assign14020_e20958 - 0.01);
        let assign14020_e20962: f64 = (-10000.0);
        let assign14020_e20964: f64 = (assign14020_e20962 * 0.001);
        let (assign14020_e20983, assign14020_e20983_d_n4,) = {
            if (assign14020_e20960 < assign14020_e20964) {
                let assign14020_e20967: f64 = (-0.001);
                let assign14020_e20969: f64 = (assign14020_e20967 * 0.001);
                let assign14020_e20975: f64 = (locals.var_tratio - 1.0);
                let assign14020_e20976: f64 = (p.p904 * assign14020_e20975);
                let assign14020_e20977: f64 = (1.0 + assign14020_e20976);
                let assign14020_e20978: f64 = (p.p743 * assign14020_e20977);
                let assign14020_e20980: f64 = (assign14020_e20978 - 0.01);
                let assign14020_e20981: f64 = (assign14020_e20969 / assign14020_e20980);
                (assign14020_e20981, (-((assign14020_e20969 * (p.p743 * (p.p904 * locals.var_tratio_dn4))) / (assign14020_e20980 * assign14020_e20980))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14020_e20983, assign14020_e20983_d_n4,)
    }
};
        let assign14020_e20986: f64 = (assign14020_e20984 + 0.01);
        locals.var_njtsd_t = assign14020_e20986;
        locals.var_njtsd_t_dn4 = assign14020_e20984_d_n4;
        locals.var_njtsd_t_rv = 0.0;

        let assign14030_e20992: f64 = (locals.var_tratio - 1.0);
        let assign14030_e20993: f64 = (p.p906 * assign14030_e20992);
        let assign14030_e20994: f64 = (1.0 + assign14030_e20993);
        let assign14030_e20995: f64 = (p.p745 * assign14030_e20994);
        let assign14030_e20997: f64 = (assign14030_e20995 - 0.01);
        let assign14030_e20999: f64 = (-10000.0);
        let assign14030_e21001: f64 = (assign14030_e20999 * 0.001);
        let (assign14030_e21082, assign14030_e21082_d_n4,) = {
    if (!(assign14030_e20997 < assign14030_e21001)) {
        let assign14030_e21010: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21011: f64 = (p.p906 * assign14030_e21010);
        let assign14030_e21012: f64 = (1.0 + assign14030_e21011);
        let assign14030_e21013: f64 = (p.p745 * assign14030_e21012);
        let assign14030_e21015: f64 = (assign14030_e21013 - 0.01);
        let assign14030_e21021: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21022: f64 = (p.p906 * assign14030_e21021);
        let assign14030_e21023: f64 = (1.0 + assign14030_e21022);
        let assign14030_e21024: f64 = (p.p745 * assign14030_e21023);
        let assign14030_e21026: f64 = (assign14030_e21024 - 0.01);
        let assign14030_e21032: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21033: f64 = (p.p906 * assign14030_e21032);
        let assign14030_e21034: f64 = (1.0 + assign14030_e21033);
        let assign14030_e21035: f64 = (p.p745 * assign14030_e21034);
        let assign14030_e21037: f64 = (assign14030_e21035 - 0.01);
        let assign14030_e21038: f64 = (assign14030_e21026 * assign14030_e21037);
        let assign14030_e21041: f64 = (4.0 * 0.001);
        let assign14030_e21043: f64 = (assign14030_e21041 * 0.001);
        let assign14030_e21044: f64 = (assign14030_e21038 + assign14030_e21043);
        let assign14030_e21045: f64 = (assign14030_e21044).sqrt();
        let assign14030_e21046: f64 = (assign14030_e21015 + assign14030_e21045);
        let assign14030_e21047: f64 = (0.5 * assign14030_e21046);
        (assign14030_e21047, (0.5 * ((p.p745 * (p.p906 * locals.var_tratio_dn4)) + ((((p.p745 * (p.p906 * locals.var_tratio_dn4)) * assign14030_e21037) + (assign14030_e21026 * (p.p745 * (p.p906 * locals.var_tratio_dn4)))) / (2.0 * assign14030_e21045)))),)
    } else {
        let assign14030_e21053: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21054: f64 = (p.p906 * assign14030_e21053);
        let assign14030_e21055: f64 = (1.0 + assign14030_e21054);
        let assign14030_e21056: f64 = (p.p745 * assign14030_e21055);
        let assign14030_e21058: f64 = (assign14030_e21056 - 0.01);
        let assign14030_e21060: f64 = (-10000.0);
        let assign14030_e21062: f64 = (assign14030_e21060 * 0.001);
        let (assign14030_e21081, assign14030_e21081_d_n4,) = {
            if (assign14030_e21058 < assign14030_e21062) {
                let assign14030_e21065: f64 = (-0.001);
                let assign14030_e21067: f64 = (assign14030_e21065 * 0.001);
                let assign14030_e21073: f64 = (locals.var_tratio - 1.0);
                let assign14030_e21074: f64 = (p.p906 * assign14030_e21073);
                let assign14030_e21075: f64 = (1.0 + assign14030_e21074);
                let assign14030_e21076: f64 = (p.p745 * assign14030_e21075);
                let assign14030_e21078: f64 = (assign14030_e21076 - 0.01);
                let assign14030_e21079: f64 = (assign14030_e21067 / assign14030_e21078);
                (assign14030_e21079, (-((assign14030_e21067 * (p.p745 * (p.p906 * locals.var_tratio_dn4))) / (assign14030_e21078 * assign14030_e21078))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14030_e21081, assign14030_e21081_d_n4,)
    }
};
        let assign14030_e21084: f64 = (assign14030_e21082 + 0.01);
        locals.var_njtsswd_t = assign14030_e21084;
        locals.var_njtsswd_t_dn4 = assign14030_e21082_d_n4;
        locals.var_njtsswd_t_rv = 0.0;

        let assign14040_e21090: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21091: f64 = (p.p908 * assign14040_e21090);
        let assign14040_e21092: f64 = (1.0 + assign14040_e21091);
        let assign14040_e21093: f64 = (p.p747 * assign14040_e21092);
        let assign14040_e21095: f64 = (assign14040_e21093 - 0.01);
        let assign14040_e21097: f64 = (-10000.0);
        let assign14040_e21099: f64 = (assign14040_e21097 * 0.001);
        let (assign14040_e21180, assign14040_e21180_d_n4,) = {
    if (!(assign14040_e21095 < assign14040_e21099)) {
        let assign14040_e21108: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21109: f64 = (p.p908 * assign14040_e21108);
        let assign14040_e21110: f64 = (1.0 + assign14040_e21109);
        let assign14040_e21111: f64 = (p.p747 * assign14040_e21110);
        let assign14040_e21113: f64 = (assign14040_e21111 - 0.01);
        let assign14040_e21119: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21120: f64 = (p.p908 * assign14040_e21119);
        let assign14040_e21121: f64 = (1.0 + assign14040_e21120);
        let assign14040_e21122: f64 = (p.p747 * assign14040_e21121);
        let assign14040_e21124: f64 = (assign14040_e21122 - 0.01);
        let assign14040_e21130: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21131: f64 = (p.p908 * assign14040_e21130);
        let assign14040_e21132: f64 = (1.0 + assign14040_e21131);
        let assign14040_e21133: f64 = (p.p747 * assign14040_e21132);
        let assign14040_e21135: f64 = (assign14040_e21133 - 0.01);
        let assign14040_e21136: f64 = (assign14040_e21124 * assign14040_e21135);
        let assign14040_e21139: f64 = (4.0 * 0.001);
        let assign14040_e21141: f64 = (assign14040_e21139 * 0.001);
        let assign14040_e21142: f64 = (assign14040_e21136 + assign14040_e21141);
        let assign14040_e21143: f64 = (assign14040_e21142).sqrt();
        let assign14040_e21144: f64 = (assign14040_e21113 + assign14040_e21143);
        let assign14040_e21145: f64 = (0.5 * assign14040_e21144);
        (assign14040_e21145, (0.5 * ((p.p747 * (p.p908 * locals.var_tratio_dn4)) + ((((p.p747 * (p.p908 * locals.var_tratio_dn4)) * assign14040_e21135) + (assign14040_e21124 * (p.p747 * (p.p908 * locals.var_tratio_dn4)))) / (2.0 * assign14040_e21143)))),)
    } else {
        let assign14040_e21151: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21152: f64 = (p.p908 * assign14040_e21151);
        let assign14040_e21153: f64 = (1.0 + assign14040_e21152);
        let assign14040_e21154: f64 = (p.p747 * assign14040_e21153);
        let assign14040_e21156: f64 = (assign14040_e21154 - 0.01);
        let assign14040_e21158: f64 = (-10000.0);
        let assign14040_e21160: f64 = (assign14040_e21158 * 0.001);
        let (assign14040_e21179, assign14040_e21179_d_n4,) = {
            if (assign14040_e21156 < assign14040_e21160) {
                let assign14040_e21163: f64 = (-0.001);
                let assign14040_e21165: f64 = (assign14040_e21163 * 0.001);
                let assign14040_e21171: f64 = (locals.var_tratio - 1.0);
                let assign14040_e21172: f64 = (p.p908 * assign14040_e21171);
                let assign14040_e21173: f64 = (1.0 + assign14040_e21172);
                let assign14040_e21174: f64 = (p.p747 * assign14040_e21173);
                let assign14040_e21176: f64 = (assign14040_e21174 - 0.01);
                let assign14040_e21177: f64 = (assign14040_e21165 / assign14040_e21176);
                (assign14040_e21177, (-((assign14040_e21165 * (p.p747 * (p.p908 * locals.var_tratio_dn4))) / (assign14040_e21176 * assign14040_e21176))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14040_e21179, assign14040_e21179_d_n4,)
    }
};
        let assign14040_e21182: f64 = (assign14040_e21180 + 0.01);
        locals.var_njtsswgd_t = assign14040_e21182;
        locals.var_njtsswgd_t_dn4 = assign14040_e21180_d_n4;
        locals.var_njtsswgd_t_rv = 0.0;

        let assign14050_e21185: f64 = if p.p9 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard462 = assign14050_e21185;
        locals.var_guard462_rv = 0.0;

        let assign14060_e21188: f64 = (p.p2 % 2.0);
        let assign14060_e21190: f64 = if assign14060_e21188 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard463 = assign14060_e21190;
        locals.var_guard463_rv = 0.0;

        let (assign14070_e21196,) = {
    if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign14070_e21196;
        locals.var_nuendd_rv = 0.0;

        let (assign14080_e21202,) = {
    if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign14080_e21202;
        locals.var_nuends_rv = 0.0;

        let (assign14090_e21216,) = {
    if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
        let assign14090_e21209: f64 = (p.p2 - 1.0);
        let assign14090_e21211: f64 = (assign14090_e21209 / 2.0);
        let assign14090_e21213: f64 = (assign14090_e21211).max(0.0);
        let assign14090_e21214: f64 = (2.0 * assign14090_e21213);
        (assign14090_e21214,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign14090_e21216;
        locals.var_nuintd_rv = 0.0;

        let (assign14100_e21222,) = {
    if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
        (locals.var_nuintd,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign14100_e21222;
        locals.var_nuints_rv = 0.0;

        let assign14110_e21225: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard464 = assign14110_e21225;
        locals.var_guard464_rv = 0.0;

        let (assign14120_e21234,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign14120_e21234;
        locals.var_nuendd_rv = 0.0;

        let (assign14130_e21251,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
        let assign14130_e21244: f64 = (p.p2 / 2.0);
        let assign14130_e21246: f64 = (assign14130_e21244 - 1.0);
        let assign14130_e21248: f64 = (assign14130_e21246).max(0.0);
        let assign14130_e21249: f64 = (2.0 * assign14130_e21248);
        (assign14130_e21249,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign14130_e21251;
        locals.var_nuintd_rv = 0.0;

        let (assign14140_e21260,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign14140_e21260;
        locals.var_nuends_rv = 0.0;

        let (assign14150_e21269,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign14150_e21269;
        locals.var_nuints_rv = 0.0;

        let (assign14160_e21279,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign14160_e21279;
        locals.var_nuendd_rv = 0.0;

        let (assign14170_e21289,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign14170_e21289;
        locals.var_nuintd_rv = 0.0;

        let (assign14180_e21299,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign14180_e21299;
        locals.var_nuends_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14190_e21317,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
        let assign14190_e21310: f64 = (p.p2 / 2.0);
        let assign14190_e21312: f64 = (assign14190_e21310 - 1.0);
        let assign14190_e21314: f64 = (assign14190_e21312).max(0.0);
        let assign14190_e21315: f64 = (2.0 * assign14190_e21314);
        (assign14190_e21315,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign14190_e21317;
        locals.var_nuints_rv = 0.0;

        let assign14200_e21320: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        locals.var_t0 = assign14200_e21320;
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
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign14210_e21323: f64 = (locals.var_dmcgeff + locals.var_dmcgeff);
        locals.var_t1 = assign14210_e21323;
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
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign14220_e21326: f64 = (locals.var_dmdgeff + locals.var_dmdgeff);
        locals.var_t2 = assign14220_e21326;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn3 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn12 = 0.0;
        locals.var_t2_dn13 = 0.0;
        locals.var_t2_dn14 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign14230_e21329: f64 = (locals.var_t0 + locals.var_t0);
        let assign14230_e21331: f64 = (assign14230_e21329 + locals.var_weffcj);
        locals.var_psiso = assign14230_e21331;
        locals.var_psiso_dn0 = (locals.var_t0_dn0 + locals.var_t0_dn0);
        locals.var_psiso_dn2 = (locals.var_t0_dn2 + locals.var_t0_dn2);
        locals.var_psiso_dn3 = (locals.var_t0_dn3 + locals.var_t0_dn3);
        locals.var_psiso_dn4 = (locals.var_t0_dn4 + locals.var_t0_dn4);
        locals.var_psiso_dn5 = (locals.var_t0_dn5 + locals.var_t0_dn5);
        locals.var_psiso_dn6 = (locals.var_t0_dn6 + locals.var_t0_dn6);
        locals.var_psiso_dn7 = (locals.var_t0_dn7 + locals.var_t0_dn7);
        locals.var_psiso_dn8 = (locals.var_t0_dn8 + locals.var_t0_dn8);
        locals.var_psiso_dn9 = (locals.var_t0_dn9 + locals.var_t0_dn9);
        locals.var_psiso_dn10 = (locals.var_t0_dn10 + locals.var_t0_dn10);
        locals.var_psiso_dn11 = (locals.var_t0_dn11 + locals.var_t0_dn11);
        locals.var_psiso_dn12 = (locals.var_t0_dn12 + locals.var_t0_dn12);
        locals.var_psiso_dn13 = (locals.var_t0_dn13 + locals.var_t0_dn13);
        locals.var_psiso_dn14 = (locals.var_t0_dn14 + locals.var_t0_dn14);
        locals.var_psiso_rv = 0.0;

        let assign14240_e21334: f64 = (locals.var_t0 + locals.var_t0);
        let assign14240_e21336: f64 = (assign14240_e21334 + locals.var_weffcj);
        locals.var_pdiso = assign14240_e21336;
        locals.var_pdiso_dn0 = (locals.var_t0_dn0 + locals.var_t0_dn0);
        locals.var_pdiso_dn2 = (locals.var_t0_dn2 + locals.var_t0_dn2);
        locals.var_pdiso_dn3 = (locals.var_t0_dn3 + locals.var_t0_dn3);
        locals.var_pdiso_dn4 = (locals.var_t0_dn4 + locals.var_t0_dn4);
        locals.var_pdiso_dn5 = (locals.var_t0_dn5 + locals.var_t0_dn5);
        locals.var_pdiso_dn6 = (locals.var_t0_dn6 + locals.var_t0_dn6);
        locals.var_pdiso_dn7 = (locals.var_t0_dn7 + locals.var_t0_dn7);
        locals.var_pdiso_dn8 = (locals.var_t0_dn8 + locals.var_t0_dn8);
        locals.var_pdiso_dn9 = (locals.var_t0_dn9 + locals.var_t0_dn9);
        locals.var_pdiso_dn10 = (locals.var_t0_dn10 + locals.var_t0_dn10);
        locals.var_pdiso_dn11 = (locals.var_t0_dn11 + locals.var_t0_dn11);
        locals.var_pdiso_dn12 = (locals.var_t0_dn12 + locals.var_t0_dn12);
        locals.var_pdiso_dn13 = (locals.var_t0_dn13 + locals.var_t0_dn13);
        locals.var_pdiso_dn14 = (locals.var_t0_dn14 + locals.var_t0_dn14);
        locals.var_pdiso_rv = 0.0;

        locals.var_pssha = locals.var_t1;
        locals.var_pssha_dn0 = locals.var_t1_dn0;
        locals.var_pssha_dn2 = locals.var_t1_dn2;
        locals.var_pssha_dn3 = locals.var_t1_dn3;
        locals.var_pssha_dn4 = locals.var_t1_dn4;
        locals.var_pssha_dn5 = locals.var_t1_dn5;
        locals.var_pssha_dn6 = locals.var_t1_dn6;
        locals.var_pssha_dn7 = locals.var_t1_dn7;
        locals.var_pssha_dn8 = locals.var_t1_dn8;
        locals.var_pssha_dn9 = locals.var_t1_dn9;
        locals.var_pssha_dn10 = locals.var_t1_dn10;
        locals.var_pssha_dn11 = locals.var_t1_dn11;
        locals.var_pssha_dn12 = locals.var_t1_dn12;
        locals.var_pssha_dn13 = locals.var_t1_dn13;
        locals.var_pssha_dn14 = locals.var_t1_dn14;
        locals.var_pssha_rv = 0.0;

        locals.var_pdsha = locals.var_t1;
        locals.var_pdsha_dn0 = locals.var_t1_dn0;
        locals.var_pdsha_dn2 = locals.var_t1_dn2;
        locals.var_pdsha_dn3 = locals.var_t1_dn3;
        locals.var_pdsha_dn4 = locals.var_t1_dn4;
        locals.var_pdsha_dn5 = locals.var_t1_dn5;
        locals.var_pdsha_dn6 = locals.var_t1_dn6;
        locals.var_pdsha_dn7 = locals.var_t1_dn7;
        locals.var_pdsha_dn8 = locals.var_t1_dn8;
        locals.var_pdsha_dn9 = locals.var_t1_dn9;
        locals.var_pdsha_dn10 = locals.var_t1_dn10;
        locals.var_pdsha_dn11 = locals.var_t1_dn11;
        locals.var_pdsha_dn12 = locals.var_t1_dn12;
        locals.var_pdsha_dn13 = locals.var_t1_dn13;
        locals.var_pdsha_dn14 = locals.var_t1_dn14;
        locals.var_pdsha_rv = 0.0;

        locals.var_psmer = locals.var_t2;
        locals.var_psmer_dn0 = locals.var_t2_dn0;
        locals.var_psmer_dn2 = locals.var_t2_dn2;
        locals.var_psmer_dn3 = locals.var_t2_dn3;
        locals.var_psmer_dn4 = locals.var_t2_dn4;
        locals.var_psmer_dn5 = locals.var_t2_dn5;
        locals.var_psmer_dn6 = locals.var_t2_dn6;
        locals.var_psmer_dn7 = locals.var_t2_dn7;
        locals.var_psmer_dn8 = locals.var_t2_dn8;
        locals.var_psmer_dn9 = locals.var_t2_dn9;
        locals.var_psmer_dn10 = locals.var_t2_dn10;
        locals.var_psmer_dn11 = locals.var_t2_dn11;
        locals.var_psmer_dn12 = locals.var_t2_dn12;
        locals.var_psmer_dn13 = locals.var_t2_dn13;
        locals.var_psmer_dn14 = locals.var_t2_dn14;
        locals.var_psmer_rv = 0.0;

        locals.var_pdmer = locals.var_t2;
        locals.var_pdmer_dn0 = locals.var_t2_dn0;
        locals.var_pdmer_dn2 = locals.var_t2_dn2;
        locals.var_pdmer_dn3 = locals.var_t2_dn3;
        locals.var_pdmer_dn4 = locals.var_t2_dn4;
        locals.var_pdmer_dn5 = locals.var_t2_dn5;
        locals.var_pdmer_dn6 = locals.var_t2_dn6;
        locals.var_pdmer_dn7 = locals.var_t2_dn7;
        locals.var_pdmer_dn8 = locals.var_t2_dn8;
        locals.var_pdmer_dn9 = locals.var_t2_dn9;
        locals.var_pdmer_dn10 = locals.var_t2_dn10;
        locals.var_pdmer_dn11 = locals.var_t2_dn11;
        locals.var_pdmer_dn12 = locals.var_t2_dn12;
        locals.var_pdmer_dn13 = locals.var_t2_dn13;
        locals.var_pdmer_dn14 = locals.var_t2_dn14;
        locals.var_pdmer_rv = 0.0;

        let assign14290_e21343: f64 = (locals.var_t0 * locals.var_weffcj);
        locals.var_asiso = assign14290_e21343;
        locals.var_asiso_dn0 = (locals.var_t0_dn0 * locals.var_weffcj);
        locals.var_asiso_dn2 = (locals.var_t0_dn2 * locals.var_weffcj);
        locals.var_asiso_dn3 = (locals.var_t0_dn3 * locals.var_weffcj);
        locals.var_asiso_dn4 = (locals.var_t0_dn4 * locals.var_weffcj);
        locals.var_asiso_dn5 = (locals.var_t0_dn5 * locals.var_weffcj);
        locals.var_asiso_dn6 = (locals.var_t0_dn6 * locals.var_weffcj);
        locals.var_asiso_dn7 = (locals.var_t0_dn7 * locals.var_weffcj);
        locals.var_asiso_dn8 = (locals.var_t0_dn8 * locals.var_weffcj);
        locals.var_asiso_dn9 = (locals.var_t0_dn9 * locals.var_weffcj);
        locals.var_asiso_dn10 = (locals.var_t0_dn10 * locals.var_weffcj);
        locals.var_asiso_dn11 = (locals.var_t0_dn11 * locals.var_weffcj);
        locals.var_asiso_dn12 = (locals.var_t0_dn12 * locals.var_weffcj);
        locals.var_asiso_dn13 = (locals.var_t0_dn13 * locals.var_weffcj);
        locals.var_asiso_dn14 = (locals.var_t0_dn14 * locals.var_weffcj);
        locals.var_asiso_rv = 0.0;

        let assign14300_e21346: f64 = (locals.var_t0 * locals.var_weffcj);
        locals.var_adiso = assign14300_e21346;
        locals.var_adiso_dn0 = (locals.var_t0_dn0 * locals.var_weffcj);
        locals.var_adiso_dn2 = (locals.var_t0_dn2 * locals.var_weffcj);
        locals.var_adiso_dn3 = (locals.var_t0_dn3 * locals.var_weffcj);
        locals.var_adiso_dn4 = (locals.var_t0_dn4 * locals.var_weffcj);
        locals.var_adiso_dn5 = (locals.var_t0_dn5 * locals.var_weffcj);
        locals.var_adiso_dn6 = (locals.var_t0_dn6 * locals.var_weffcj);
        locals.var_adiso_dn7 = (locals.var_t0_dn7 * locals.var_weffcj);
        locals.var_adiso_dn8 = (locals.var_t0_dn8 * locals.var_weffcj);
        locals.var_adiso_dn9 = (locals.var_t0_dn9 * locals.var_weffcj);
        locals.var_adiso_dn10 = (locals.var_t0_dn10 * locals.var_weffcj);
        locals.var_adiso_dn11 = (locals.var_t0_dn11 * locals.var_weffcj);
        locals.var_adiso_dn12 = (locals.var_t0_dn12 * locals.var_weffcj);
        locals.var_adiso_dn13 = (locals.var_t0_dn13 * locals.var_weffcj);
        locals.var_adiso_dn14 = (locals.var_t0_dn14 * locals.var_weffcj);
        locals.var_adiso_rv = 0.0;

        let assign14310_e21349: f64 = (locals.var_dmcgeff * locals.var_weffcj);
        locals.var_assha = assign14310_e21349;
        locals.var_assha_rv = 0.0;

        let assign14320_e21352: f64 = (locals.var_dmcgeff * locals.var_weffcj);
        locals.var_adsha = assign14320_e21352;
        locals.var_adsha_rv = 0.0;

        let assign14330_e21355: f64 = (locals.var_dmdgeff * locals.var_weffcj);
        locals.var_asmer = assign14330_e21355;
        locals.var_asmer_rv = 0.0;

        let assign14340_e21358: f64 = (locals.var_dmdgeff * locals.var_weffcj);
        locals.var_admer = assign14340_e21358;
        locals.var_admer_rv = 0.0;

        let assign14350_e21361: f64 = if p.p9 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard465 = assign14350_e21361;
        locals.var_guard465_rv = 0.0;

        let assign14360_e21364: f64 = if p.p9 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard466 = assign14360_e21364;
        locals.var_guard466_rv = 0.0;

        let assign14370_e21367: f64 = if p.p9 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard467 = assign14370_e21367;
        locals.var_guard467_rv = 0.0;

        let assign14380_e21370: f64 = if p.p9 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard468 = assign14380_e21370;
        locals.var_guard468_rv = 0.0;

        let assign14390_e21373: f64 = if p.p9 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard469 = assign14390_e21373;
        locals.var_guard469_rv = 0.0;

        let assign14400_e21376: f64 = if p.p9 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard470 = assign14400_e21376;
        locals.var_guard470_rv = 0.0;

        let assign14410_e21379: f64 = if p.p9 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard471 = assign14410_e21379;
        locals.var_guard471_rv = 0.0;

        let assign14420_e21382: f64 = if p.p9 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard472 = assign14420_e21382;
        locals.var_guard472_rv = 0.0;

        let assign14430_e21385: f64 = if p.p9 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign14430_e21385;
        locals.var_guard473_rv = 0.0;

        let assign14440_e21388: f64 = if p.p9 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard474 = assign14440_e21388;
        locals.var_guard474_rv = 0.0;

        let assign14450_e21391: f64 = if p.p9 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign14450_e21391;
        locals.var_guard475_rv = 0.0;

        let (assign14460_e21401, assign14460_e21401_d_n0, assign14460_e21401_d_n2, assign14460_e21401_d_n3, assign14460_e21401_d_n4, assign14460_e21401_d_n5, assign14460_e21401_d_n6, assign14460_e21401_d_n7, assign14460_e21401_d_n8, assign14460_e21401_d_n9, assign14460_e21401_d_n10, assign14460_e21401_d_n11, assign14460_e21401_d_n12, assign14460_e21401_d_n13, assign14460_e21401_d_n14,) = {
    if (locals.var_guard465 != 0.0) {
        let assign14460_e21395: f64 = (locals.var_nuends * locals.var_psiso);
        let assign14460_e21398: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14460_e21399: f64 = (assign14460_e21395 + assign14460_e21398);
        (assign14460_e21399, ((locals.var_nuends * locals.var_psiso_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psiso_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psiso_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psiso_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psiso_dn14) + (locals.var_nuints * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14460_e21401;
        locals.var_temp_pseff_dn0 = assign14460_e21401_d_n0;
        locals.var_temp_pseff_dn2 = assign14460_e21401_d_n2;
        locals.var_temp_pseff_dn3 = assign14460_e21401_d_n3;
        locals.var_temp_pseff_dn4 = assign14460_e21401_d_n4;
        locals.var_temp_pseff_dn5 = assign14460_e21401_d_n5;
        locals.var_temp_pseff_dn6 = assign14460_e21401_d_n6;
        locals.var_temp_pseff_dn7 = assign14460_e21401_d_n7;
        locals.var_temp_pseff_dn8 = assign14460_e21401_d_n8;
        locals.var_temp_pseff_dn9 = assign14460_e21401_d_n9;
        locals.var_temp_pseff_dn10 = assign14460_e21401_d_n10;
        locals.var_temp_pseff_dn11 = assign14460_e21401_d_n11;
        locals.var_temp_pseff_dn12 = assign14460_e21401_d_n12;
        locals.var_temp_pseff_dn13 = assign14460_e21401_d_n13;
        locals.var_temp_pseff_dn14 = assign14460_e21401_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14470_e21411, assign14470_e21411_d_n0, assign14470_e21411_d_n2, assign14470_e21411_d_n3, assign14470_e21411_d_n4, assign14470_e21411_d_n5, assign14470_e21411_d_n6, assign14470_e21411_d_n7, assign14470_e21411_d_n8, assign14470_e21411_d_n9, assign14470_e21411_d_n10, assign14470_e21411_d_n11, assign14470_e21411_d_n12, assign14470_e21411_d_n13, assign14470_e21411_d_n14,) = {
    if (locals.var_guard465 != 0.0) {
        let assign14470_e21405: f64 = (locals.var_nuendd * locals.var_pdiso);
        let assign14470_e21408: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14470_e21409: f64 = (assign14470_e21405 + assign14470_e21408);
        (assign14470_e21409, ((locals.var_nuendd * locals.var_pdiso_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdiso_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdiso_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdiso_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdiso_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14470_e21411;
        locals.var_temp_pdeff_dn0 = assign14470_e21411_d_n0;
        locals.var_temp_pdeff_dn2 = assign14470_e21411_d_n2;
        locals.var_temp_pdeff_dn3 = assign14470_e21411_d_n3;
        locals.var_temp_pdeff_dn4 = assign14470_e21411_d_n4;
        locals.var_temp_pdeff_dn5 = assign14470_e21411_d_n5;
        locals.var_temp_pdeff_dn6 = assign14470_e21411_d_n6;
        locals.var_temp_pdeff_dn7 = assign14470_e21411_d_n7;
        locals.var_temp_pdeff_dn8 = assign14470_e21411_d_n8;
        locals.var_temp_pdeff_dn9 = assign14470_e21411_d_n9;
        locals.var_temp_pdeff_dn10 = assign14470_e21411_d_n10;
        locals.var_temp_pdeff_dn11 = assign14470_e21411_d_n11;
        locals.var_temp_pdeff_dn12 = assign14470_e21411_d_n12;
        locals.var_temp_pdeff_dn13 = assign14470_e21411_d_n13;
        locals.var_temp_pdeff_dn14 = assign14470_e21411_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14480_e21421, assign14480_e21421_d_n0, assign14480_e21421_d_n2, assign14480_e21421_d_n3, assign14480_e21421_d_n4, assign14480_e21421_d_n5, assign14480_e21421_d_n6, assign14480_e21421_d_n7, assign14480_e21421_d_n8, assign14480_e21421_d_n9, assign14480_e21421_d_n10, assign14480_e21421_d_n11, assign14480_e21421_d_n12, assign14480_e21421_d_n13, assign14480_e21421_d_n14,) = {
    if (locals.var_guard465 != 0.0) {
        let assign14480_e21415: f64 = (locals.var_nuends * locals.var_asiso);
        let assign14480_e21418: f64 = (locals.var_nuints * locals.var_assha);
        let assign14480_e21419: f64 = (assign14480_e21415 + assign14480_e21418);
        (assign14480_e21419, (locals.var_nuends * locals.var_asiso_dn0), (locals.var_nuends * locals.var_asiso_dn2), (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11), (locals.var_nuends * locals.var_asiso_dn12), (locals.var_nuends * locals.var_asiso_dn13), (locals.var_nuends * locals.var_asiso_dn14),)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14480_e21421;
        locals.var_temp_aseff_dn0 = assign14480_e21421_d_n0;
        locals.var_temp_aseff_dn2 = assign14480_e21421_d_n2;
        locals.var_temp_aseff_dn3 = assign14480_e21421_d_n3;
        locals.var_temp_aseff_dn4 = assign14480_e21421_d_n4;
        locals.var_temp_aseff_dn5 = assign14480_e21421_d_n5;
        locals.var_temp_aseff_dn6 = assign14480_e21421_d_n6;
        locals.var_temp_aseff_dn7 = assign14480_e21421_d_n7;
        locals.var_temp_aseff_dn8 = assign14480_e21421_d_n8;
        locals.var_temp_aseff_dn9 = assign14480_e21421_d_n9;
        locals.var_temp_aseff_dn10 = assign14480_e21421_d_n10;
        locals.var_temp_aseff_dn11 = assign14480_e21421_d_n11;
        locals.var_temp_aseff_dn12 = assign14480_e21421_d_n12;
        locals.var_temp_aseff_dn13 = assign14480_e21421_d_n13;
        locals.var_temp_aseff_dn14 = assign14480_e21421_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14490_e21431, assign14490_e21431_d_n0, assign14490_e21431_d_n2, assign14490_e21431_d_n3, assign14490_e21431_d_n4, assign14490_e21431_d_n5, assign14490_e21431_d_n6, assign14490_e21431_d_n7, assign14490_e21431_d_n8, assign14490_e21431_d_n9, assign14490_e21431_d_n10, assign14490_e21431_d_n11, assign14490_e21431_d_n12, assign14490_e21431_d_n13, assign14490_e21431_d_n14,) = {
    if (locals.var_guard465 != 0.0) {
        let assign14490_e21425: f64 = (locals.var_nuendd * locals.var_adiso);
        let assign14490_e21428: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14490_e21429: f64 = (assign14490_e21425 + assign14490_e21428);
        (assign14490_e21429, (locals.var_nuendd * locals.var_adiso_dn0), (locals.var_nuendd * locals.var_adiso_dn2), (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11), (locals.var_nuendd * locals.var_adiso_dn12), (locals.var_nuendd * locals.var_adiso_dn13), (locals.var_nuendd * locals.var_adiso_dn14),)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14490_e21431;
        locals.var_temp_adeff_dn0 = assign14490_e21431_d_n0;
        locals.var_temp_adeff_dn2 = assign14490_e21431_d_n2;
        locals.var_temp_adeff_dn3 = assign14490_e21431_d_n3;
        locals.var_temp_adeff_dn4 = assign14490_e21431_d_n4;
        locals.var_temp_adeff_dn5 = assign14490_e21431_d_n5;
        locals.var_temp_adeff_dn6 = assign14490_e21431_d_n6;
        locals.var_temp_adeff_dn7 = assign14490_e21431_d_n7;
        locals.var_temp_adeff_dn8 = assign14490_e21431_d_n8;
        locals.var_temp_adeff_dn9 = assign14490_e21431_d_n9;
        locals.var_temp_adeff_dn10 = assign14490_e21431_d_n10;
        locals.var_temp_adeff_dn11 = assign14490_e21431_d_n11;
        locals.var_temp_adeff_dn12 = assign14490_e21431_d_n12;
        locals.var_temp_adeff_dn13 = assign14490_e21431_d_n13;
        locals.var_temp_adeff_dn14 = assign14490_e21431_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14500_e21444, assign14500_e21444_d_n0, assign14500_e21444_d_n2, assign14500_e21444_d_n3, assign14500_e21444_d_n4, assign14500_e21444_d_n5, assign14500_e21444_d_n6, assign14500_e21444_d_n7, assign14500_e21444_d_n8, assign14500_e21444_d_n9, assign14500_e21444_d_n10, assign14500_e21444_d_n11, assign14500_e21444_d_n12, assign14500_e21444_d_n13, assign14500_e21444_d_n14,) = {
    if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
        let assign14500_e21438: f64 = (locals.var_nuends * locals.var_psiso);
        let assign14500_e21441: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14500_e21442: f64 = (assign14500_e21438 + assign14500_e21441);
        (assign14500_e21442, ((locals.var_nuends * locals.var_psiso_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psiso_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psiso_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psiso_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psiso_dn14) + (locals.var_nuints * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14500_e21444;
        locals.var_temp_pseff_dn0 = assign14500_e21444_d_n0;
        locals.var_temp_pseff_dn2 = assign14500_e21444_d_n2;
        locals.var_temp_pseff_dn3 = assign14500_e21444_d_n3;
        locals.var_temp_pseff_dn4 = assign14500_e21444_d_n4;
        locals.var_temp_pseff_dn5 = assign14500_e21444_d_n5;
        locals.var_temp_pseff_dn6 = assign14500_e21444_d_n6;
        locals.var_temp_pseff_dn7 = assign14500_e21444_d_n7;
        locals.var_temp_pseff_dn8 = assign14500_e21444_d_n8;
        locals.var_temp_pseff_dn9 = assign14500_e21444_d_n9;
        locals.var_temp_pseff_dn10 = assign14500_e21444_d_n10;
        locals.var_temp_pseff_dn11 = assign14500_e21444_d_n11;
        locals.var_temp_pseff_dn12 = assign14500_e21444_d_n12;
        locals.var_temp_pseff_dn13 = assign14500_e21444_d_n13;
        locals.var_temp_pseff_dn14 = assign14500_e21444_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14510_e21455, assign14510_e21455_d_n0, assign14510_e21455_d_n2, assign14510_e21455_d_n3, assign14510_e21455_d_n4, assign14510_e21455_d_n5, assign14510_e21455_d_n6, assign14510_e21455_d_n7, assign14510_e21455_d_n8, assign14510_e21455_d_n9, assign14510_e21455_d_n10, assign14510_e21455_d_n11, assign14510_e21455_d_n12, assign14510_e21455_d_n13, assign14510_e21455_d_n14,) = {
    if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
        let assign14510_e21451: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14510_e21453: f64 = (assign14510_e21451 * locals.var_pdsha);
        (assign14510_e21453, (assign14510_e21451 * locals.var_pdsha_dn0), (assign14510_e21451 * locals.var_pdsha_dn2), (assign14510_e21451 * locals.var_pdsha_dn3), (assign14510_e21451 * locals.var_pdsha_dn4), (assign14510_e21451 * locals.var_pdsha_dn5), (assign14510_e21451 * locals.var_pdsha_dn6), (assign14510_e21451 * locals.var_pdsha_dn7), (assign14510_e21451 * locals.var_pdsha_dn8), (assign14510_e21451 * locals.var_pdsha_dn9), (assign14510_e21451 * locals.var_pdsha_dn10), (assign14510_e21451 * locals.var_pdsha_dn11), (assign14510_e21451 * locals.var_pdsha_dn12), (assign14510_e21451 * locals.var_pdsha_dn13), (assign14510_e21451 * locals.var_pdsha_dn14),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14510_e21455;
        locals.var_temp_pdeff_dn0 = assign14510_e21455_d_n0;
        locals.var_temp_pdeff_dn2 = assign14510_e21455_d_n2;
        locals.var_temp_pdeff_dn3 = assign14510_e21455_d_n3;
        locals.var_temp_pdeff_dn4 = assign14510_e21455_d_n4;
        locals.var_temp_pdeff_dn5 = assign14510_e21455_d_n5;
        locals.var_temp_pdeff_dn6 = assign14510_e21455_d_n6;
        locals.var_temp_pdeff_dn7 = assign14510_e21455_d_n7;
        locals.var_temp_pdeff_dn8 = assign14510_e21455_d_n8;
        locals.var_temp_pdeff_dn9 = assign14510_e21455_d_n9;
        locals.var_temp_pdeff_dn10 = assign14510_e21455_d_n10;
        locals.var_temp_pdeff_dn11 = assign14510_e21455_d_n11;
        locals.var_temp_pdeff_dn12 = assign14510_e21455_d_n12;
        locals.var_temp_pdeff_dn13 = assign14510_e21455_d_n13;
        locals.var_temp_pdeff_dn14 = assign14510_e21455_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14520_e21468, assign14520_e21468_d_n0, assign14520_e21468_d_n2, assign14520_e21468_d_n3, assign14520_e21468_d_n4, assign14520_e21468_d_n5, assign14520_e21468_d_n6, assign14520_e21468_d_n7, assign14520_e21468_d_n8, assign14520_e21468_d_n9, assign14520_e21468_d_n10, assign14520_e21468_d_n11, assign14520_e21468_d_n12, assign14520_e21468_d_n13, assign14520_e21468_d_n14,) = {
    if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
        let assign14520_e21462: f64 = (locals.var_nuends * locals.var_asiso);
        let assign14520_e21465: f64 = (locals.var_nuints * locals.var_assha);
        let assign14520_e21466: f64 = (assign14520_e21462 + assign14520_e21465);
        (assign14520_e21466, (locals.var_nuends * locals.var_asiso_dn0), (locals.var_nuends * locals.var_asiso_dn2), (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11), (locals.var_nuends * locals.var_asiso_dn12), (locals.var_nuends * locals.var_asiso_dn13), (locals.var_nuends * locals.var_asiso_dn14),)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14520_e21468;
        locals.var_temp_aseff_dn0 = assign14520_e21468_d_n0;
        locals.var_temp_aseff_dn2 = assign14520_e21468_d_n2;
        locals.var_temp_aseff_dn3 = assign14520_e21468_d_n3;
        locals.var_temp_aseff_dn4 = assign14520_e21468_d_n4;
        locals.var_temp_aseff_dn5 = assign14520_e21468_d_n5;
        locals.var_temp_aseff_dn6 = assign14520_e21468_d_n6;
        locals.var_temp_aseff_dn7 = assign14520_e21468_d_n7;
        locals.var_temp_aseff_dn8 = assign14520_e21468_d_n8;
        locals.var_temp_aseff_dn9 = assign14520_e21468_d_n9;
        locals.var_temp_aseff_dn10 = assign14520_e21468_d_n10;
        locals.var_temp_aseff_dn11 = assign14520_e21468_d_n11;
        locals.var_temp_aseff_dn12 = assign14520_e21468_d_n12;
        locals.var_temp_aseff_dn13 = assign14520_e21468_d_n13;
        locals.var_temp_aseff_dn14 = assign14520_e21468_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14530_e21479, assign14530_e21479_d_n0, assign14530_e21479_d_n2, assign14530_e21479_d_n3, assign14530_e21479_d_n4, assign14530_e21479_d_n5, assign14530_e21479_d_n6, assign14530_e21479_d_n7, assign14530_e21479_d_n8, assign14530_e21479_d_n9, assign14530_e21479_d_n10, assign14530_e21479_d_n11, assign14530_e21479_d_n12, assign14530_e21479_d_n13, assign14530_e21479_d_n14,) = {
    if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
        let assign14530_e21475: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14530_e21477: f64 = (assign14530_e21475 * locals.var_adsha);
        (assign14530_e21477, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14530_e21479;
        locals.var_temp_adeff_dn0 = assign14530_e21479_d_n0;
        locals.var_temp_adeff_dn2 = assign14530_e21479_d_n2;
        locals.var_temp_adeff_dn3 = assign14530_e21479_d_n3;
        locals.var_temp_adeff_dn4 = assign14530_e21479_d_n4;
        locals.var_temp_adeff_dn5 = assign14530_e21479_d_n5;
        locals.var_temp_adeff_dn6 = assign14530_e21479_d_n6;
        locals.var_temp_adeff_dn7 = assign14530_e21479_d_n7;
        locals.var_temp_adeff_dn8 = assign14530_e21479_d_n8;
        locals.var_temp_adeff_dn9 = assign14530_e21479_d_n9;
        locals.var_temp_adeff_dn10 = assign14530_e21479_d_n10;
        locals.var_temp_adeff_dn11 = assign14530_e21479_d_n11;
        locals.var_temp_adeff_dn12 = assign14530_e21479_d_n12;
        locals.var_temp_adeff_dn13 = assign14530_e21479_d_n13;
        locals.var_temp_adeff_dn14 = assign14530_e21479_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14540_e21492, assign14540_e21492_d_n0, assign14540_e21492_d_n2, assign14540_e21492_d_n3, assign14540_e21492_d_n4, assign14540_e21492_d_n5, assign14540_e21492_d_n6, assign14540_e21492_d_n7, assign14540_e21492_d_n8, assign14540_e21492_d_n9, assign14540_e21492_d_n10, assign14540_e21492_d_n11, assign14540_e21492_d_n12, assign14540_e21492_d_n13, assign14540_e21492_d_n14,) = {
    if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
        let assign14540_e21488: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14540_e21490: f64 = (assign14540_e21488 * locals.var_pssha);
        (assign14540_e21490, (assign14540_e21488 * locals.var_pssha_dn0), (assign14540_e21488 * locals.var_pssha_dn2), (assign14540_e21488 * locals.var_pssha_dn3), (assign14540_e21488 * locals.var_pssha_dn4), (assign14540_e21488 * locals.var_pssha_dn5), (assign14540_e21488 * locals.var_pssha_dn6), (assign14540_e21488 * locals.var_pssha_dn7), (assign14540_e21488 * locals.var_pssha_dn8), (assign14540_e21488 * locals.var_pssha_dn9), (assign14540_e21488 * locals.var_pssha_dn10), (assign14540_e21488 * locals.var_pssha_dn11), (assign14540_e21488 * locals.var_pssha_dn12), (assign14540_e21488 * locals.var_pssha_dn13), (assign14540_e21488 * locals.var_pssha_dn14),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14540_e21492;
        locals.var_temp_pseff_dn0 = assign14540_e21492_d_n0;
        locals.var_temp_pseff_dn2 = assign14540_e21492_d_n2;
        locals.var_temp_pseff_dn3 = assign14540_e21492_d_n3;
        locals.var_temp_pseff_dn4 = assign14540_e21492_d_n4;
        locals.var_temp_pseff_dn5 = assign14540_e21492_d_n5;
        locals.var_temp_pseff_dn6 = assign14540_e21492_d_n6;
        locals.var_temp_pseff_dn7 = assign14540_e21492_d_n7;
        locals.var_temp_pseff_dn8 = assign14540_e21492_d_n8;
        locals.var_temp_pseff_dn9 = assign14540_e21492_d_n9;
        locals.var_temp_pseff_dn10 = assign14540_e21492_d_n10;
        locals.var_temp_pseff_dn11 = assign14540_e21492_d_n11;
        locals.var_temp_pseff_dn12 = assign14540_e21492_d_n12;
        locals.var_temp_pseff_dn13 = assign14540_e21492_d_n13;
        locals.var_temp_pseff_dn14 = assign14540_e21492_d_n14;
        locals.var_temp_pseff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_29(
        locals: &mut StampLocals,
    ) {
        let (assign14550_e21507, assign14550_e21507_d_n0, assign14550_e21507_d_n2, assign14550_e21507_d_n3, assign14550_e21507_d_n4, assign14550_e21507_d_n5, assign14550_e21507_d_n6, assign14550_e21507_d_n7, assign14550_e21507_d_n8, assign14550_e21507_d_n9, assign14550_e21507_d_n10, assign14550_e21507_d_n11, assign14550_e21507_d_n12, assign14550_e21507_d_n13, assign14550_e21507_d_n14,) = {
    if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
        let assign14550_e21501: f64 = (locals.var_nuendd * locals.var_pdiso);
        let assign14550_e21504: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14550_e21505: f64 = (assign14550_e21501 + assign14550_e21504);
        (assign14550_e21505, ((locals.var_nuendd * locals.var_pdiso_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdiso_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdiso_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdiso_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdiso_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14550_e21507;
        locals.var_temp_pdeff_dn0 = assign14550_e21507_d_n0;
        locals.var_temp_pdeff_dn2 = assign14550_e21507_d_n2;
        locals.var_temp_pdeff_dn3 = assign14550_e21507_d_n3;
        locals.var_temp_pdeff_dn4 = assign14550_e21507_d_n4;
        locals.var_temp_pdeff_dn5 = assign14550_e21507_d_n5;
        locals.var_temp_pdeff_dn6 = assign14550_e21507_d_n6;
        locals.var_temp_pdeff_dn7 = assign14550_e21507_d_n7;
        locals.var_temp_pdeff_dn8 = assign14550_e21507_d_n8;
        locals.var_temp_pdeff_dn9 = assign14550_e21507_d_n9;
        locals.var_temp_pdeff_dn10 = assign14550_e21507_d_n10;
        locals.var_temp_pdeff_dn11 = assign14550_e21507_d_n11;
        locals.var_temp_pdeff_dn12 = assign14550_e21507_d_n12;
        locals.var_temp_pdeff_dn13 = assign14550_e21507_d_n13;
        locals.var_temp_pdeff_dn14 = assign14550_e21507_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14560_e21520, assign14560_e21520_d_n0, assign14560_e21520_d_n2, assign14560_e21520_d_n3, assign14560_e21520_d_n4, assign14560_e21520_d_n5, assign14560_e21520_d_n6, assign14560_e21520_d_n7, assign14560_e21520_d_n8, assign14560_e21520_d_n9, assign14560_e21520_d_n10, assign14560_e21520_d_n11, assign14560_e21520_d_n12, assign14560_e21520_d_n13, assign14560_e21520_d_n14,) = {
    if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
        let assign14560_e21516: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14560_e21518: f64 = (assign14560_e21516 * locals.var_assha);
        (assign14560_e21518, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14560_e21520;
        locals.var_temp_aseff_dn0 = assign14560_e21520_d_n0;
        locals.var_temp_aseff_dn2 = assign14560_e21520_d_n2;
        locals.var_temp_aseff_dn3 = assign14560_e21520_d_n3;
        locals.var_temp_aseff_dn4 = assign14560_e21520_d_n4;
        locals.var_temp_aseff_dn5 = assign14560_e21520_d_n5;
        locals.var_temp_aseff_dn6 = assign14560_e21520_d_n6;
        locals.var_temp_aseff_dn7 = assign14560_e21520_d_n7;
        locals.var_temp_aseff_dn8 = assign14560_e21520_d_n8;
        locals.var_temp_aseff_dn9 = assign14560_e21520_d_n9;
        locals.var_temp_aseff_dn10 = assign14560_e21520_d_n10;
        locals.var_temp_aseff_dn11 = assign14560_e21520_d_n11;
        locals.var_temp_aseff_dn12 = assign14560_e21520_d_n12;
        locals.var_temp_aseff_dn13 = assign14560_e21520_d_n13;
        locals.var_temp_aseff_dn14 = assign14560_e21520_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14570_e21535, assign14570_e21535_d_n0, assign14570_e21535_d_n2, assign14570_e21535_d_n3, assign14570_e21535_d_n4, assign14570_e21535_d_n5, assign14570_e21535_d_n6, assign14570_e21535_d_n7, assign14570_e21535_d_n8, assign14570_e21535_d_n9, assign14570_e21535_d_n10, assign14570_e21535_d_n11, assign14570_e21535_d_n12, assign14570_e21535_d_n13, assign14570_e21535_d_n14,) = {
    if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
        let assign14570_e21529: f64 = (locals.var_nuendd * locals.var_adiso);
        let assign14570_e21532: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14570_e21533: f64 = (assign14570_e21529 + assign14570_e21532);
        (assign14570_e21533, (locals.var_nuendd * locals.var_adiso_dn0), (locals.var_nuendd * locals.var_adiso_dn2), (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11), (locals.var_nuendd * locals.var_adiso_dn12), (locals.var_nuendd * locals.var_adiso_dn13), (locals.var_nuendd * locals.var_adiso_dn14),)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14570_e21535;
        locals.var_temp_adeff_dn0 = assign14570_e21535_d_n0;
        locals.var_temp_adeff_dn2 = assign14570_e21535_d_n2;
        locals.var_temp_adeff_dn3 = assign14570_e21535_d_n3;
        locals.var_temp_adeff_dn4 = assign14570_e21535_d_n4;
        locals.var_temp_adeff_dn5 = assign14570_e21535_d_n5;
        locals.var_temp_adeff_dn6 = assign14570_e21535_d_n6;
        locals.var_temp_adeff_dn7 = assign14570_e21535_d_n7;
        locals.var_temp_adeff_dn8 = assign14570_e21535_d_n8;
        locals.var_temp_adeff_dn9 = assign14570_e21535_d_n9;
        locals.var_temp_adeff_dn10 = assign14570_e21535_d_n10;
        locals.var_temp_adeff_dn11 = assign14570_e21535_d_n11;
        locals.var_temp_adeff_dn12 = assign14570_e21535_d_n12;
        locals.var_temp_adeff_dn13 = assign14570_e21535_d_n13;
        locals.var_temp_adeff_dn14 = assign14570_e21535_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14580_e21550, assign14580_e21550_d_n0, assign14580_e21550_d_n2, assign14580_e21550_d_n3, assign14580_e21550_d_n4, assign14580_e21550_d_n5, assign14580_e21550_d_n6, assign14580_e21550_d_n7, assign14580_e21550_d_n8, assign14580_e21550_d_n9, assign14580_e21550_d_n10, assign14580_e21550_d_n11, assign14580_e21550_d_n12, assign14580_e21550_d_n13, assign14580_e21550_d_n14,) = {
    if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign14580_e21546: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14580_e21548: f64 = (assign14580_e21546 * locals.var_pssha);
        (assign14580_e21548, (assign14580_e21546 * locals.var_pssha_dn0), (assign14580_e21546 * locals.var_pssha_dn2), (assign14580_e21546 * locals.var_pssha_dn3), (assign14580_e21546 * locals.var_pssha_dn4), (assign14580_e21546 * locals.var_pssha_dn5), (assign14580_e21546 * locals.var_pssha_dn6), (assign14580_e21546 * locals.var_pssha_dn7), (assign14580_e21546 * locals.var_pssha_dn8), (assign14580_e21546 * locals.var_pssha_dn9), (assign14580_e21546 * locals.var_pssha_dn10), (assign14580_e21546 * locals.var_pssha_dn11), (assign14580_e21546 * locals.var_pssha_dn12), (assign14580_e21546 * locals.var_pssha_dn13), (assign14580_e21546 * locals.var_pssha_dn14),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14580_e21550;
        locals.var_temp_pseff_dn0 = assign14580_e21550_d_n0;
        locals.var_temp_pseff_dn2 = assign14580_e21550_d_n2;
        locals.var_temp_pseff_dn3 = assign14580_e21550_d_n3;
        locals.var_temp_pseff_dn4 = assign14580_e21550_d_n4;
        locals.var_temp_pseff_dn5 = assign14580_e21550_d_n5;
        locals.var_temp_pseff_dn6 = assign14580_e21550_d_n6;
        locals.var_temp_pseff_dn7 = assign14580_e21550_d_n7;
        locals.var_temp_pseff_dn8 = assign14580_e21550_d_n8;
        locals.var_temp_pseff_dn9 = assign14580_e21550_d_n9;
        locals.var_temp_pseff_dn10 = assign14580_e21550_d_n10;
        locals.var_temp_pseff_dn11 = assign14580_e21550_d_n11;
        locals.var_temp_pseff_dn12 = assign14580_e21550_d_n12;
        locals.var_temp_pseff_dn13 = assign14580_e21550_d_n13;
        locals.var_temp_pseff_dn14 = assign14580_e21550_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14590_e21565, assign14590_e21565_d_n0, assign14590_e21565_d_n2, assign14590_e21565_d_n3, assign14590_e21565_d_n4, assign14590_e21565_d_n5, assign14590_e21565_d_n6, assign14590_e21565_d_n7, assign14590_e21565_d_n8, assign14590_e21565_d_n9, assign14590_e21565_d_n10, assign14590_e21565_d_n11, assign14590_e21565_d_n12, assign14590_e21565_d_n13, assign14590_e21565_d_n14,) = {
    if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign14590_e21561: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14590_e21563: f64 = (assign14590_e21561 * locals.var_pdsha);
        (assign14590_e21563, (assign14590_e21561 * locals.var_pdsha_dn0), (assign14590_e21561 * locals.var_pdsha_dn2), (assign14590_e21561 * locals.var_pdsha_dn3), (assign14590_e21561 * locals.var_pdsha_dn4), (assign14590_e21561 * locals.var_pdsha_dn5), (assign14590_e21561 * locals.var_pdsha_dn6), (assign14590_e21561 * locals.var_pdsha_dn7), (assign14590_e21561 * locals.var_pdsha_dn8), (assign14590_e21561 * locals.var_pdsha_dn9), (assign14590_e21561 * locals.var_pdsha_dn10), (assign14590_e21561 * locals.var_pdsha_dn11), (assign14590_e21561 * locals.var_pdsha_dn12), (assign14590_e21561 * locals.var_pdsha_dn13), (assign14590_e21561 * locals.var_pdsha_dn14),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14590_e21565;
        locals.var_temp_pdeff_dn0 = assign14590_e21565_d_n0;
        locals.var_temp_pdeff_dn2 = assign14590_e21565_d_n2;
        locals.var_temp_pdeff_dn3 = assign14590_e21565_d_n3;
        locals.var_temp_pdeff_dn4 = assign14590_e21565_d_n4;
        locals.var_temp_pdeff_dn5 = assign14590_e21565_d_n5;
        locals.var_temp_pdeff_dn6 = assign14590_e21565_d_n6;
        locals.var_temp_pdeff_dn7 = assign14590_e21565_d_n7;
        locals.var_temp_pdeff_dn8 = assign14590_e21565_d_n8;
        locals.var_temp_pdeff_dn9 = assign14590_e21565_d_n9;
        locals.var_temp_pdeff_dn10 = assign14590_e21565_d_n10;
        locals.var_temp_pdeff_dn11 = assign14590_e21565_d_n11;
        locals.var_temp_pdeff_dn12 = assign14590_e21565_d_n12;
        locals.var_temp_pdeff_dn13 = assign14590_e21565_d_n13;
        locals.var_temp_pdeff_dn14 = assign14590_e21565_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14600_e21580, assign14600_e21580_d_n0, assign14600_e21580_d_n2, assign14600_e21580_d_n3, assign14600_e21580_d_n4, assign14600_e21580_d_n5, assign14600_e21580_d_n6, assign14600_e21580_d_n7, assign14600_e21580_d_n8, assign14600_e21580_d_n9, assign14600_e21580_d_n10, assign14600_e21580_d_n11, assign14600_e21580_d_n12, assign14600_e21580_d_n13, assign14600_e21580_d_n14,) = {
    if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign14600_e21576: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14600_e21578: f64 = (assign14600_e21576 * locals.var_assha);
        (assign14600_e21578, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14600_e21580;
        locals.var_temp_aseff_dn0 = assign14600_e21580_d_n0;
        locals.var_temp_aseff_dn2 = assign14600_e21580_d_n2;
        locals.var_temp_aseff_dn3 = assign14600_e21580_d_n3;
        locals.var_temp_aseff_dn4 = assign14600_e21580_d_n4;
        locals.var_temp_aseff_dn5 = assign14600_e21580_d_n5;
        locals.var_temp_aseff_dn6 = assign14600_e21580_d_n6;
        locals.var_temp_aseff_dn7 = assign14600_e21580_d_n7;
        locals.var_temp_aseff_dn8 = assign14600_e21580_d_n8;
        locals.var_temp_aseff_dn9 = assign14600_e21580_d_n9;
        locals.var_temp_aseff_dn10 = assign14600_e21580_d_n10;
        locals.var_temp_aseff_dn11 = assign14600_e21580_d_n11;
        locals.var_temp_aseff_dn12 = assign14600_e21580_d_n12;
        locals.var_temp_aseff_dn13 = assign14600_e21580_d_n13;
        locals.var_temp_aseff_dn14 = assign14600_e21580_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14610_e21595, assign14610_e21595_d_n0, assign14610_e21595_d_n2, assign14610_e21595_d_n3, assign14610_e21595_d_n4, assign14610_e21595_d_n5, assign14610_e21595_d_n6, assign14610_e21595_d_n7, assign14610_e21595_d_n8, assign14610_e21595_d_n9, assign14610_e21595_d_n10, assign14610_e21595_d_n11, assign14610_e21595_d_n12, assign14610_e21595_d_n13, assign14610_e21595_d_n14,) = {
    if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign14610_e21591: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14610_e21593: f64 = (assign14610_e21591 * locals.var_adsha);
        (assign14610_e21593, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14610_e21595;
        locals.var_temp_adeff_dn0 = assign14610_e21595_d_n0;
        locals.var_temp_adeff_dn2 = assign14610_e21595_d_n2;
        locals.var_temp_adeff_dn3 = assign14610_e21595_d_n3;
        locals.var_temp_adeff_dn4 = assign14610_e21595_d_n4;
        locals.var_temp_adeff_dn5 = assign14610_e21595_d_n5;
        locals.var_temp_adeff_dn6 = assign14610_e21595_d_n6;
        locals.var_temp_adeff_dn7 = assign14610_e21595_d_n7;
        locals.var_temp_adeff_dn8 = assign14610_e21595_d_n8;
        locals.var_temp_adeff_dn9 = assign14610_e21595_d_n9;
        locals.var_temp_adeff_dn10 = assign14610_e21595_d_n10;
        locals.var_temp_adeff_dn11 = assign14610_e21595_d_n11;
        locals.var_temp_adeff_dn12 = assign14610_e21595_d_n12;
        locals.var_temp_adeff_dn13 = assign14610_e21595_d_n13;
        locals.var_temp_adeff_dn14 = assign14610_e21595_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14620_e21614, assign14620_e21614_d_n0, assign14620_e21614_d_n2, assign14620_e21614_d_n3, assign14620_e21614_d_n4, assign14620_e21614_d_n5, assign14620_e21614_d_n6, assign14620_e21614_d_n7, assign14620_e21614_d_n8, assign14620_e21614_d_n9, assign14620_e21614_d_n10, assign14620_e21614_d_n11, assign14620_e21614_d_n12, assign14620_e21614_d_n13, assign14620_e21614_d_n14,) = {
    if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign14620_e21608: f64 = (locals.var_nuends * locals.var_psiso);
        let assign14620_e21611: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14620_e21612: f64 = (assign14620_e21608 + assign14620_e21611);
        (assign14620_e21612, ((locals.var_nuends * locals.var_psiso_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psiso_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psiso_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psiso_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psiso_dn14) + (locals.var_nuints * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14620_e21614;
        locals.var_temp_pseff_dn0 = assign14620_e21614_d_n0;
        locals.var_temp_pseff_dn2 = assign14620_e21614_d_n2;
        locals.var_temp_pseff_dn3 = assign14620_e21614_d_n3;
        locals.var_temp_pseff_dn4 = assign14620_e21614_d_n4;
        locals.var_temp_pseff_dn5 = assign14620_e21614_d_n5;
        locals.var_temp_pseff_dn6 = assign14620_e21614_d_n6;
        locals.var_temp_pseff_dn7 = assign14620_e21614_d_n7;
        locals.var_temp_pseff_dn8 = assign14620_e21614_d_n8;
        locals.var_temp_pseff_dn9 = assign14620_e21614_d_n9;
        locals.var_temp_pseff_dn10 = assign14620_e21614_d_n10;
        locals.var_temp_pseff_dn11 = assign14620_e21614_d_n11;
        locals.var_temp_pseff_dn12 = assign14620_e21614_d_n12;
        locals.var_temp_pseff_dn13 = assign14620_e21614_d_n13;
        locals.var_temp_pseff_dn14 = assign14620_e21614_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14630_e21633, assign14630_e21633_d_n0, assign14630_e21633_d_n2, assign14630_e21633_d_n3, assign14630_e21633_d_n4, assign14630_e21633_d_n5, assign14630_e21633_d_n6, assign14630_e21633_d_n7, assign14630_e21633_d_n8, assign14630_e21633_d_n9, assign14630_e21633_d_n10, assign14630_e21633_d_n11, assign14630_e21633_d_n12, assign14630_e21633_d_n13, assign14630_e21633_d_n14,) = {
    if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign14630_e21627: f64 = (locals.var_nuendd * locals.var_pdmer);
        let assign14630_e21630: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14630_e21631: f64 = (assign14630_e21627 + assign14630_e21630);
        (assign14630_e21631, ((locals.var_nuendd * locals.var_pdmer_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdmer_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdmer_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdmer_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdmer_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14630_e21633;
        locals.var_temp_pdeff_dn0 = assign14630_e21633_d_n0;
        locals.var_temp_pdeff_dn2 = assign14630_e21633_d_n2;
        locals.var_temp_pdeff_dn3 = assign14630_e21633_d_n3;
        locals.var_temp_pdeff_dn4 = assign14630_e21633_d_n4;
        locals.var_temp_pdeff_dn5 = assign14630_e21633_d_n5;
        locals.var_temp_pdeff_dn6 = assign14630_e21633_d_n6;
        locals.var_temp_pdeff_dn7 = assign14630_e21633_d_n7;
        locals.var_temp_pdeff_dn8 = assign14630_e21633_d_n8;
        locals.var_temp_pdeff_dn9 = assign14630_e21633_d_n9;
        locals.var_temp_pdeff_dn10 = assign14630_e21633_d_n10;
        locals.var_temp_pdeff_dn11 = assign14630_e21633_d_n11;
        locals.var_temp_pdeff_dn12 = assign14630_e21633_d_n12;
        locals.var_temp_pdeff_dn13 = assign14630_e21633_d_n13;
        locals.var_temp_pdeff_dn14 = assign14630_e21633_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14640_e21652, assign14640_e21652_d_n0, assign14640_e21652_d_n2, assign14640_e21652_d_n3, assign14640_e21652_d_n4, assign14640_e21652_d_n5, assign14640_e21652_d_n6, assign14640_e21652_d_n7, assign14640_e21652_d_n8, assign14640_e21652_d_n9, assign14640_e21652_d_n10, assign14640_e21652_d_n11, assign14640_e21652_d_n12, assign14640_e21652_d_n13, assign14640_e21652_d_n14,) = {
    if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign14640_e21646: f64 = (locals.var_nuends * locals.var_asiso);
        let assign14640_e21649: f64 = (locals.var_nuints * locals.var_assha);
        let assign14640_e21650: f64 = (assign14640_e21646 + assign14640_e21649);
        (assign14640_e21650, (locals.var_nuends * locals.var_asiso_dn0), (locals.var_nuends * locals.var_asiso_dn2), (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11), (locals.var_nuends * locals.var_asiso_dn12), (locals.var_nuends * locals.var_asiso_dn13), (locals.var_nuends * locals.var_asiso_dn14),)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14640_e21652;
        locals.var_temp_aseff_dn0 = assign14640_e21652_d_n0;
        locals.var_temp_aseff_dn2 = assign14640_e21652_d_n2;
        locals.var_temp_aseff_dn3 = assign14640_e21652_d_n3;
        locals.var_temp_aseff_dn4 = assign14640_e21652_d_n4;
        locals.var_temp_aseff_dn5 = assign14640_e21652_d_n5;
        locals.var_temp_aseff_dn6 = assign14640_e21652_d_n6;
        locals.var_temp_aseff_dn7 = assign14640_e21652_d_n7;
        locals.var_temp_aseff_dn8 = assign14640_e21652_d_n8;
        locals.var_temp_aseff_dn9 = assign14640_e21652_d_n9;
        locals.var_temp_aseff_dn10 = assign14640_e21652_d_n10;
        locals.var_temp_aseff_dn11 = assign14640_e21652_d_n11;
        locals.var_temp_aseff_dn12 = assign14640_e21652_d_n12;
        locals.var_temp_aseff_dn13 = assign14640_e21652_d_n13;
        locals.var_temp_aseff_dn14 = assign14640_e21652_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14650_e21671, assign14650_e21671_d_n0, assign14650_e21671_d_n2, assign14650_e21671_d_n3, assign14650_e21671_d_n4, assign14650_e21671_d_n5, assign14650_e21671_d_n6, assign14650_e21671_d_n7, assign14650_e21671_d_n8, assign14650_e21671_d_n9, assign14650_e21671_d_n10, assign14650_e21671_d_n11, assign14650_e21671_d_n12, assign14650_e21671_d_n13, assign14650_e21671_d_n14,) = {
    if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign14650_e21665: f64 = (locals.var_nuendd * locals.var_admer);
        let assign14650_e21668: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14650_e21669: f64 = (assign14650_e21665 + assign14650_e21668);
        (assign14650_e21669, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14650_e21671;
        locals.var_temp_adeff_dn0 = assign14650_e21671_d_n0;
        locals.var_temp_adeff_dn2 = assign14650_e21671_d_n2;
        locals.var_temp_adeff_dn3 = assign14650_e21671_d_n3;
        locals.var_temp_adeff_dn4 = assign14650_e21671_d_n4;
        locals.var_temp_adeff_dn5 = assign14650_e21671_d_n5;
        locals.var_temp_adeff_dn6 = assign14650_e21671_d_n6;
        locals.var_temp_adeff_dn7 = assign14650_e21671_d_n7;
        locals.var_temp_adeff_dn8 = assign14650_e21671_d_n8;
        locals.var_temp_adeff_dn9 = assign14650_e21671_d_n9;
        locals.var_temp_adeff_dn10 = assign14650_e21671_d_n10;
        locals.var_temp_adeff_dn11 = assign14650_e21671_d_n11;
        locals.var_temp_adeff_dn12 = assign14650_e21671_d_n12;
        locals.var_temp_adeff_dn13 = assign14650_e21671_d_n13;
        locals.var_temp_adeff_dn14 = assign14650_e21671_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14660_e21690, assign14660_e21690_d_n0, assign14660_e21690_d_n2, assign14660_e21690_d_n3, assign14660_e21690_d_n4, assign14660_e21690_d_n5, assign14660_e21690_d_n6, assign14660_e21690_d_n7, assign14660_e21690_d_n8, assign14660_e21690_d_n9, assign14660_e21690_d_n10, assign14660_e21690_d_n11, assign14660_e21690_d_n12, assign14660_e21690_d_n13, assign14660_e21690_d_n14,) = {
    if ((locals.var_guard470 != 0.0) && (!(((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)))) {
        let assign14660_e21686: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14660_e21688: f64 = (assign14660_e21686 * locals.var_pssha);
        (assign14660_e21688, (assign14660_e21686 * locals.var_pssha_dn0), (assign14660_e21686 * locals.var_pssha_dn2), (assign14660_e21686 * locals.var_pssha_dn3), (assign14660_e21686 * locals.var_pssha_dn4), (assign14660_e21686 * locals.var_pssha_dn5), (assign14660_e21686 * locals.var_pssha_dn6), (assign14660_e21686 * locals.var_pssha_dn7), (assign14660_e21686 * locals.var_pssha_dn8), (assign14660_e21686 * locals.var_pssha_dn9), (assign14660_e21686 * locals.var_pssha_dn10), (assign14660_e21686 * locals.var_pssha_dn11), (assign14660_e21686 * locals.var_pssha_dn12), (assign14660_e21686 * locals.var_pssha_dn13), (assign14660_e21686 * locals.var_pssha_dn14),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14660_e21690;
        locals.var_temp_pseff_dn0 = assign14660_e21690_d_n0;
        locals.var_temp_pseff_dn2 = assign14660_e21690_d_n2;
        locals.var_temp_pseff_dn3 = assign14660_e21690_d_n3;
        locals.var_temp_pseff_dn4 = assign14660_e21690_d_n4;
        locals.var_temp_pseff_dn5 = assign14660_e21690_d_n5;
        locals.var_temp_pseff_dn6 = assign14660_e21690_d_n6;
        locals.var_temp_pseff_dn7 = assign14660_e21690_d_n7;
        locals.var_temp_pseff_dn8 = assign14660_e21690_d_n8;
        locals.var_temp_pseff_dn9 = assign14660_e21690_d_n9;
        locals.var_temp_pseff_dn10 = assign14660_e21690_d_n10;
        locals.var_temp_pseff_dn11 = assign14660_e21690_d_n11;
        locals.var_temp_pseff_dn12 = assign14660_e21690_d_n12;
        locals.var_temp_pseff_dn13 = assign14660_e21690_d_n13;
        locals.var_temp_pseff_dn14 = assign14660_e21690_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14670_e21711, assign14670_e21711_d_n0, assign14670_e21711_d_n2, assign14670_e21711_d_n3, assign14670_e21711_d_n4, assign14670_e21711_d_n5, assign14670_e21711_d_n6, assign14670_e21711_d_n7, assign14670_e21711_d_n8, assign14670_e21711_d_n9, assign14670_e21711_d_n10, assign14670_e21711_d_n11, assign14670_e21711_d_n12, assign14670_e21711_d_n13, assign14670_e21711_d_n14,) = {
    if ((locals.var_guard470 != 0.0) && (!(((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)))) {
        let assign14670_e21705: f64 = (locals.var_nuendd * locals.var_pdmer);
        let assign14670_e21708: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14670_e21709: f64 = (assign14670_e21705 + assign14670_e21708);
        (assign14670_e21709, ((locals.var_nuendd * locals.var_pdmer_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdmer_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdmer_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdmer_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdmer_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14670_e21711;
        locals.var_temp_pdeff_dn0 = assign14670_e21711_d_n0;
        locals.var_temp_pdeff_dn2 = assign14670_e21711_d_n2;
        locals.var_temp_pdeff_dn3 = assign14670_e21711_d_n3;
        locals.var_temp_pdeff_dn4 = assign14670_e21711_d_n4;
        locals.var_temp_pdeff_dn5 = assign14670_e21711_d_n5;
        locals.var_temp_pdeff_dn6 = assign14670_e21711_d_n6;
        locals.var_temp_pdeff_dn7 = assign14670_e21711_d_n7;
        locals.var_temp_pdeff_dn8 = assign14670_e21711_d_n8;
        locals.var_temp_pdeff_dn9 = assign14670_e21711_d_n9;
        locals.var_temp_pdeff_dn10 = assign14670_e21711_d_n10;
        locals.var_temp_pdeff_dn11 = assign14670_e21711_d_n11;
        locals.var_temp_pdeff_dn12 = assign14670_e21711_d_n12;
        locals.var_temp_pdeff_dn13 = assign14670_e21711_d_n13;
        locals.var_temp_pdeff_dn14 = assign14670_e21711_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14680_e21730, assign14680_e21730_d_n0, assign14680_e21730_d_n2, assign14680_e21730_d_n3, assign14680_e21730_d_n4, assign14680_e21730_d_n5, assign14680_e21730_d_n6, assign14680_e21730_d_n7, assign14680_e21730_d_n8, assign14680_e21730_d_n9, assign14680_e21730_d_n10, assign14680_e21730_d_n11, assign14680_e21730_d_n12, assign14680_e21730_d_n13, assign14680_e21730_d_n14,) = {
    if ((locals.var_guard470 != 0.0) && (!(((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)))) {
        let assign14680_e21726: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14680_e21728: f64 = (assign14680_e21726 * locals.var_assha);
        (assign14680_e21728, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14680_e21730;
        locals.var_temp_aseff_dn0 = assign14680_e21730_d_n0;
        locals.var_temp_aseff_dn2 = assign14680_e21730_d_n2;
        locals.var_temp_aseff_dn3 = assign14680_e21730_d_n3;
        locals.var_temp_aseff_dn4 = assign14680_e21730_d_n4;
        locals.var_temp_aseff_dn5 = assign14680_e21730_d_n5;
        locals.var_temp_aseff_dn6 = assign14680_e21730_d_n6;
        locals.var_temp_aseff_dn7 = assign14680_e21730_d_n7;
        locals.var_temp_aseff_dn8 = assign14680_e21730_d_n8;
        locals.var_temp_aseff_dn9 = assign14680_e21730_d_n9;
        locals.var_temp_aseff_dn10 = assign14680_e21730_d_n10;
        locals.var_temp_aseff_dn11 = assign14680_e21730_d_n11;
        locals.var_temp_aseff_dn12 = assign14680_e21730_d_n12;
        locals.var_temp_aseff_dn13 = assign14680_e21730_d_n13;
        locals.var_temp_aseff_dn14 = assign14680_e21730_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14690_e21751, assign14690_e21751_d_n0, assign14690_e21751_d_n2, assign14690_e21751_d_n3, assign14690_e21751_d_n4, assign14690_e21751_d_n5, assign14690_e21751_d_n6, assign14690_e21751_d_n7, assign14690_e21751_d_n8, assign14690_e21751_d_n9, assign14690_e21751_d_n10, assign14690_e21751_d_n11, assign14690_e21751_d_n12, assign14690_e21751_d_n13, assign14690_e21751_d_n14,) = {
    if ((locals.var_guard470 != 0.0) && (!(((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)))) {
        let assign14690_e21745: f64 = (locals.var_nuendd * locals.var_admer);
        let assign14690_e21748: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14690_e21749: f64 = (assign14690_e21745 + assign14690_e21748);
        (assign14690_e21749, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14690_e21751;
        locals.var_temp_adeff_dn0 = assign14690_e21751_d_n0;
        locals.var_temp_adeff_dn2 = assign14690_e21751_d_n2;
        locals.var_temp_adeff_dn3 = assign14690_e21751_d_n3;
        locals.var_temp_adeff_dn4 = assign14690_e21751_d_n4;
        locals.var_temp_adeff_dn5 = assign14690_e21751_d_n5;
        locals.var_temp_adeff_dn6 = assign14690_e21751_d_n6;
        locals.var_temp_adeff_dn7 = assign14690_e21751_d_n7;
        locals.var_temp_adeff_dn8 = assign14690_e21751_d_n8;
        locals.var_temp_adeff_dn9 = assign14690_e21751_d_n9;
        locals.var_temp_adeff_dn10 = assign14690_e21751_d_n10;
        locals.var_temp_adeff_dn11 = assign14690_e21751_d_n11;
        locals.var_temp_adeff_dn12 = assign14690_e21751_d_n12;
        locals.var_temp_adeff_dn13 = assign14690_e21751_d_n13;
        locals.var_temp_adeff_dn14 = assign14690_e21751_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14700_e21774, assign14700_e21774_d_n0, assign14700_e21774_d_n2, assign14700_e21774_d_n3, assign14700_e21774_d_n4, assign14700_e21774_d_n5, assign14700_e21774_d_n6, assign14700_e21774_d_n7, assign14700_e21774_d_n8, assign14700_e21774_d_n9, assign14700_e21774_d_n10, assign14700_e21774_d_n11, assign14700_e21774_d_n12, assign14700_e21774_d_n13, assign14700_e21774_d_n14,) = {
    if ((locals.var_guard471 != 0.0) && (!((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)))) {
        let assign14700_e21768: f64 = (locals.var_nuends * locals.var_psmer);
        let assign14700_e21771: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14700_e21772: f64 = (assign14700_e21768 + assign14700_e21771);
        (assign14700_e21772, ((locals.var_nuends * locals.var_psmer_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psmer_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psmer_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psmer_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psmer_dn14) + (locals.var_nuints * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14700_e21774;
        locals.var_temp_pseff_dn0 = assign14700_e21774_d_n0;
        locals.var_temp_pseff_dn2 = assign14700_e21774_d_n2;
        locals.var_temp_pseff_dn3 = assign14700_e21774_d_n3;
        locals.var_temp_pseff_dn4 = assign14700_e21774_d_n4;
        locals.var_temp_pseff_dn5 = assign14700_e21774_d_n5;
        locals.var_temp_pseff_dn6 = assign14700_e21774_d_n6;
        locals.var_temp_pseff_dn7 = assign14700_e21774_d_n7;
        locals.var_temp_pseff_dn8 = assign14700_e21774_d_n8;
        locals.var_temp_pseff_dn9 = assign14700_e21774_d_n9;
        locals.var_temp_pseff_dn10 = assign14700_e21774_d_n10;
        locals.var_temp_pseff_dn11 = assign14700_e21774_d_n11;
        locals.var_temp_pseff_dn12 = assign14700_e21774_d_n12;
        locals.var_temp_pseff_dn13 = assign14700_e21774_d_n13;
        locals.var_temp_pseff_dn14 = assign14700_e21774_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14710_e21797, assign14710_e21797_d_n0, assign14710_e21797_d_n2, assign14710_e21797_d_n3, assign14710_e21797_d_n4, assign14710_e21797_d_n5, assign14710_e21797_d_n6, assign14710_e21797_d_n7, assign14710_e21797_d_n8, assign14710_e21797_d_n9, assign14710_e21797_d_n10, assign14710_e21797_d_n11, assign14710_e21797_d_n12, assign14710_e21797_d_n13, assign14710_e21797_d_n14,) = {
    if ((locals.var_guard471 != 0.0) && (!((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)))) {
        let assign14710_e21791: f64 = (locals.var_nuendd * locals.var_pdiso);
        let assign14710_e21794: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14710_e21795: f64 = (assign14710_e21791 + assign14710_e21794);
        (assign14710_e21795, ((locals.var_nuendd * locals.var_pdiso_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdiso_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdiso_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdiso_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdiso_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14710_e21797;
        locals.var_temp_pdeff_dn0 = assign14710_e21797_d_n0;
        locals.var_temp_pdeff_dn2 = assign14710_e21797_d_n2;
        locals.var_temp_pdeff_dn3 = assign14710_e21797_d_n3;
        locals.var_temp_pdeff_dn4 = assign14710_e21797_d_n4;
        locals.var_temp_pdeff_dn5 = assign14710_e21797_d_n5;
        locals.var_temp_pdeff_dn6 = assign14710_e21797_d_n6;
        locals.var_temp_pdeff_dn7 = assign14710_e21797_d_n7;
        locals.var_temp_pdeff_dn8 = assign14710_e21797_d_n8;
        locals.var_temp_pdeff_dn9 = assign14710_e21797_d_n9;
        locals.var_temp_pdeff_dn10 = assign14710_e21797_d_n10;
        locals.var_temp_pdeff_dn11 = assign14710_e21797_d_n11;
        locals.var_temp_pdeff_dn12 = assign14710_e21797_d_n12;
        locals.var_temp_pdeff_dn13 = assign14710_e21797_d_n13;
        locals.var_temp_pdeff_dn14 = assign14710_e21797_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14720_e21820, assign14720_e21820_d_n0, assign14720_e21820_d_n2, assign14720_e21820_d_n3, assign14720_e21820_d_n4, assign14720_e21820_d_n5, assign14720_e21820_d_n6, assign14720_e21820_d_n7, assign14720_e21820_d_n8, assign14720_e21820_d_n9, assign14720_e21820_d_n10, assign14720_e21820_d_n11, assign14720_e21820_d_n12, assign14720_e21820_d_n13, assign14720_e21820_d_n14,) = {
    if ((locals.var_guard471 != 0.0) && (!((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)))) {
        let assign14720_e21814: f64 = (locals.var_nuends * locals.var_asmer);
        let assign14720_e21817: f64 = (locals.var_nuints * locals.var_assha);
        let assign14720_e21818: f64 = (assign14720_e21814 + assign14720_e21817);
        (assign14720_e21818, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14720_e21820;
        locals.var_temp_aseff_dn0 = assign14720_e21820_d_n0;
        locals.var_temp_aseff_dn2 = assign14720_e21820_d_n2;
        locals.var_temp_aseff_dn3 = assign14720_e21820_d_n3;
        locals.var_temp_aseff_dn4 = assign14720_e21820_d_n4;
        locals.var_temp_aseff_dn5 = assign14720_e21820_d_n5;
        locals.var_temp_aseff_dn6 = assign14720_e21820_d_n6;
        locals.var_temp_aseff_dn7 = assign14720_e21820_d_n7;
        locals.var_temp_aseff_dn8 = assign14720_e21820_d_n8;
        locals.var_temp_aseff_dn9 = assign14720_e21820_d_n9;
        locals.var_temp_aseff_dn10 = assign14720_e21820_d_n10;
        locals.var_temp_aseff_dn11 = assign14720_e21820_d_n11;
        locals.var_temp_aseff_dn12 = assign14720_e21820_d_n12;
        locals.var_temp_aseff_dn13 = assign14720_e21820_d_n13;
        locals.var_temp_aseff_dn14 = assign14720_e21820_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14730_e21843, assign14730_e21843_d_n0, assign14730_e21843_d_n2, assign14730_e21843_d_n3, assign14730_e21843_d_n4, assign14730_e21843_d_n5, assign14730_e21843_d_n6, assign14730_e21843_d_n7, assign14730_e21843_d_n8, assign14730_e21843_d_n9, assign14730_e21843_d_n10, assign14730_e21843_d_n11, assign14730_e21843_d_n12, assign14730_e21843_d_n13, assign14730_e21843_d_n14,) = {
    if ((locals.var_guard471 != 0.0) && (!((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)))) {
        let assign14730_e21837: f64 = (locals.var_nuendd * locals.var_adiso);
        let assign14730_e21840: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14730_e21841: f64 = (assign14730_e21837 + assign14730_e21840);
        (assign14730_e21841, (locals.var_nuendd * locals.var_adiso_dn0), (locals.var_nuendd * locals.var_adiso_dn2), (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11), (locals.var_nuendd * locals.var_adiso_dn12), (locals.var_nuendd * locals.var_adiso_dn13), (locals.var_nuendd * locals.var_adiso_dn14),)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14730_e21843;
        locals.var_temp_adeff_dn0 = assign14730_e21843_d_n0;
        locals.var_temp_adeff_dn2 = assign14730_e21843_d_n2;
        locals.var_temp_adeff_dn3 = assign14730_e21843_d_n3;
        locals.var_temp_adeff_dn4 = assign14730_e21843_d_n4;
        locals.var_temp_adeff_dn5 = assign14730_e21843_d_n5;
        locals.var_temp_adeff_dn6 = assign14730_e21843_d_n6;
        locals.var_temp_adeff_dn7 = assign14730_e21843_d_n7;
        locals.var_temp_adeff_dn8 = assign14730_e21843_d_n8;
        locals.var_temp_adeff_dn9 = assign14730_e21843_d_n9;
        locals.var_temp_adeff_dn10 = assign14730_e21843_d_n10;
        locals.var_temp_adeff_dn11 = assign14730_e21843_d_n11;
        locals.var_temp_adeff_dn12 = assign14730_e21843_d_n12;
        locals.var_temp_adeff_dn13 = assign14730_e21843_d_n13;
        locals.var_temp_adeff_dn14 = assign14730_e21843_d_n14;
        locals.var_temp_adeff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14740_e21868, assign14740_e21868_d_n0, assign14740_e21868_d_n2, assign14740_e21868_d_n3, assign14740_e21868_d_n4, assign14740_e21868_d_n5, assign14740_e21868_d_n6, assign14740_e21868_d_n7, assign14740_e21868_d_n8, assign14740_e21868_d_n9, assign14740_e21868_d_n10, assign14740_e21868_d_n11, assign14740_e21868_d_n12, assign14740_e21868_d_n13, assign14740_e21868_d_n14,) = {
    if ((locals.var_guard472 != 0.0) && (!(((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)))) {
        let assign14740_e21862: f64 = (locals.var_nuends * locals.var_psmer);
        let assign14740_e21865: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14740_e21866: f64 = (assign14740_e21862 + assign14740_e21865);
        (assign14740_e21866, ((locals.var_nuends * locals.var_psmer_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psmer_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psmer_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psmer_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psmer_dn14) + (locals.var_nuints * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14740_e21868;
        locals.var_temp_pseff_dn0 = assign14740_e21868_d_n0;
        locals.var_temp_pseff_dn2 = assign14740_e21868_d_n2;
        locals.var_temp_pseff_dn3 = assign14740_e21868_d_n3;
        locals.var_temp_pseff_dn4 = assign14740_e21868_d_n4;
        locals.var_temp_pseff_dn5 = assign14740_e21868_d_n5;
        locals.var_temp_pseff_dn6 = assign14740_e21868_d_n6;
        locals.var_temp_pseff_dn7 = assign14740_e21868_d_n7;
        locals.var_temp_pseff_dn8 = assign14740_e21868_d_n8;
        locals.var_temp_pseff_dn9 = assign14740_e21868_d_n9;
        locals.var_temp_pseff_dn10 = assign14740_e21868_d_n10;
        locals.var_temp_pseff_dn11 = assign14740_e21868_d_n11;
        locals.var_temp_pseff_dn12 = assign14740_e21868_d_n12;
        locals.var_temp_pseff_dn13 = assign14740_e21868_d_n13;
        locals.var_temp_pseff_dn14 = assign14740_e21868_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14750_e21891, assign14750_e21891_d_n0, assign14750_e21891_d_n2, assign14750_e21891_d_n3, assign14750_e21891_d_n4, assign14750_e21891_d_n5, assign14750_e21891_d_n6, assign14750_e21891_d_n7, assign14750_e21891_d_n8, assign14750_e21891_d_n9, assign14750_e21891_d_n10, assign14750_e21891_d_n11, assign14750_e21891_d_n12, assign14750_e21891_d_n13, assign14750_e21891_d_n14,) = {
    if ((locals.var_guard472 != 0.0) && (!(((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)))) {
        let assign14750_e21887: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14750_e21889: f64 = (assign14750_e21887 * locals.var_pdsha);
        (assign14750_e21889, (assign14750_e21887 * locals.var_pdsha_dn0), (assign14750_e21887 * locals.var_pdsha_dn2), (assign14750_e21887 * locals.var_pdsha_dn3), (assign14750_e21887 * locals.var_pdsha_dn4), (assign14750_e21887 * locals.var_pdsha_dn5), (assign14750_e21887 * locals.var_pdsha_dn6), (assign14750_e21887 * locals.var_pdsha_dn7), (assign14750_e21887 * locals.var_pdsha_dn8), (assign14750_e21887 * locals.var_pdsha_dn9), (assign14750_e21887 * locals.var_pdsha_dn10), (assign14750_e21887 * locals.var_pdsha_dn11), (assign14750_e21887 * locals.var_pdsha_dn12), (assign14750_e21887 * locals.var_pdsha_dn13), (assign14750_e21887 * locals.var_pdsha_dn14),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14750_e21891;
        locals.var_temp_pdeff_dn0 = assign14750_e21891_d_n0;
        locals.var_temp_pdeff_dn2 = assign14750_e21891_d_n2;
        locals.var_temp_pdeff_dn3 = assign14750_e21891_d_n3;
        locals.var_temp_pdeff_dn4 = assign14750_e21891_d_n4;
        locals.var_temp_pdeff_dn5 = assign14750_e21891_d_n5;
        locals.var_temp_pdeff_dn6 = assign14750_e21891_d_n6;
        locals.var_temp_pdeff_dn7 = assign14750_e21891_d_n7;
        locals.var_temp_pdeff_dn8 = assign14750_e21891_d_n8;
        locals.var_temp_pdeff_dn9 = assign14750_e21891_d_n9;
        locals.var_temp_pdeff_dn10 = assign14750_e21891_d_n10;
        locals.var_temp_pdeff_dn11 = assign14750_e21891_d_n11;
        locals.var_temp_pdeff_dn12 = assign14750_e21891_d_n12;
        locals.var_temp_pdeff_dn13 = assign14750_e21891_d_n13;
        locals.var_temp_pdeff_dn14 = assign14750_e21891_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14760_e21916, assign14760_e21916_d_n0, assign14760_e21916_d_n2, assign14760_e21916_d_n3, assign14760_e21916_d_n4, assign14760_e21916_d_n5, assign14760_e21916_d_n6, assign14760_e21916_d_n7, assign14760_e21916_d_n8, assign14760_e21916_d_n9, assign14760_e21916_d_n10, assign14760_e21916_d_n11, assign14760_e21916_d_n12, assign14760_e21916_d_n13, assign14760_e21916_d_n14,) = {
    if ((locals.var_guard472 != 0.0) && (!(((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)))) {
        let assign14760_e21910: f64 = (locals.var_nuends * locals.var_asmer);
        let assign14760_e21913: f64 = (locals.var_nuints * locals.var_assha);
        let assign14760_e21914: f64 = (assign14760_e21910 + assign14760_e21913);
        (assign14760_e21914, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14760_e21916;
        locals.var_temp_aseff_dn0 = assign14760_e21916_d_n0;
        locals.var_temp_aseff_dn2 = assign14760_e21916_d_n2;
        locals.var_temp_aseff_dn3 = assign14760_e21916_d_n3;
        locals.var_temp_aseff_dn4 = assign14760_e21916_d_n4;
        locals.var_temp_aseff_dn5 = assign14760_e21916_d_n5;
        locals.var_temp_aseff_dn6 = assign14760_e21916_d_n6;
        locals.var_temp_aseff_dn7 = assign14760_e21916_d_n7;
        locals.var_temp_aseff_dn8 = assign14760_e21916_d_n8;
        locals.var_temp_aseff_dn9 = assign14760_e21916_d_n9;
        locals.var_temp_aseff_dn10 = assign14760_e21916_d_n10;
        locals.var_temp_aseff_dn11 = assign14760_e21916_d_n11;
        locals.var_temp_aseff_dn12 = assign14760_e21916_d_n12;
        locals.var_temp_aseff_dn13 = assign14760_e21916_d_n13;
        locals.var_temp_aseff_dn14 = assign14760_e21916_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14770_e21939, assign14770_e21939_d_n0, assign14770_e21939_d_n2, assign14770_e21939_d_n3, assign14770_e21939_d_n4, assign14770_e21939_d_n5, assign14770_e21939_d_n6, assign14770_e21939_d_n7, assign14770_e21939_d_n8, assign14770_e21939_d_n9, assign14770_e21939_d_n10, assign14770_e21939_d_n11, assign14770_e21939_d_n12, assign14770_e21939_d_n13, assign14770_e21939_d_n14,) = {
    if ((locals.var_guard472 != 0.0) && (!(((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)))) {
        let assign14770_e21935: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14770_e21937: f64 = (assign14770_e21935 * locals.var_adsha);
        (assign14770_e21937, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14770_e21939;
        locals.var_temp_adeff_dn0 = assign14770_e21939_d_n0;
        locals.var_temp_adeff_dn2 = assign14770_e21939_d_n2;
        locals.var_temp_adeff_dn3 = assign14770_e21939_d_n3;
        locals.var_temp_adeff_dn4 = assign14770_e21939_d_n4;
        locals.var_temp_adeff_dn5 = assign14770_e21939_d_n5;
        locals.var_temp_adeff_dn6 = assign14770_e21939_d_n6;
        locals.var_temp_adeff_dn7 = assign14770_e21939_d_n7;
        locals.var_temp_adeff_dn8 = assign14770_e21939_d_n8;
        locals.var_temp_adeff_dn9 = assign14770_e21939_d_n9;
        locals.var_temp_adeff_dn10 = assign14770_e21939_d_n10;
        locals.var_temp_adeff_dn11 = assign14770_e21939_d_n11;
        locals.var_temp_adeff_dn12 = assign14770_e21939_d_n12;
        locals.var_temp_adeff_dn13 = assign14770_e21939_d_n13;
        locals.var_temp_adeff_dn14 = assign14770_e21939_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14780_e21966, assign14780_e21966_d_n0, assign14780_e21966_d_n2, assign14780_e21966_d_n3, assign14780_e21966_d_n4, assign14780_e21966_d_n5, assign14780_e21966_d_n6, assign14780_e21966_d_n7, assign14780_e21966_d_n8, assign14780_e21966_d_n9, assign14780_e21966_d_n10, assign14780_e21966_d_n11, assign14780_e21966_d_n12, assign14780_e21966_d_n13, assign14780_e21966_d_n14,) = {
    if ((locals.var_guard473 != 0.0) && (!((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)))) {
        let assign14780_e21960: f64 = (locals.var_nuends * locals.var_psmer);
        let assign14780_e21963: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14780_e21964: f64 = (assign14780_e21960 + assign14780_e21963);
        (assign14780_e21964, ((locals.var_nuends * locals.var_psmer_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psmer_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psmer_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psmer_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psmer_dn14) + (locals.var_nuints * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14780_e21966;
        locals.var_temp_pseff_dn0 = assign14780_e21966_d_n0;
        locals.var_temp_pseff_dn2 = assign14780_e21966_d_n2;
        locals.var_temp_pseff_dn3 = assign14780_e21966_d_n3;
        locals.var_temp_pseff_dn4 = assign14780_e21966_d_n4;
        locals.var_temp_pseff_dn5 = assign14780_e21966_d_n5;
        locals.var_temp_pseff_dn6 = assign14780_e21966_d_n6;
        locals.var_temp_pseff_dn7 = assign14780_e21966_d_n7;
        locals.var_temp_pseff_dn8 = assign14780_e21966_d_n8;
        locals.var_temp_pseff_dn9 = assign14780_e21966_d_n9;
        locals.var_temp_pseff_dn10 = assign14780_e21966_d_n10;
        locals.var_temp_pseff_dn11 = assign14780_e21966_d_n11;
        locals.var_temp_pseff_dn12 = assign14780_e21966_d_n12;
        locals.var_temp_pseff_dn13 = assign14780_e21966_d_n13;
        locals.var_temp_pseff_dn14 = assign14780_e21966_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14790_e21993, assign14790_e21993_d_n0, assign14790_e21993_d_n2, assign14790_e21993_d_n3, assign14790_e21993_d_n4, assign14790_e21993_d_n5, assign14790_e21993_d_n6, assign14790_e21993_d_n7, assign14790_e21993_d_n8, assign14790_e21993_d_n9, assign14790_e21993_d_n10, assign14790_e21993_d_n11, assign14790_e21993_d_n12, assign14790_e21993_d_n13, assign14790_e21993_d_n14,) = {
    if ((locals.var_guard473 != 0.0) && (!((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)))) {
        let assign14790_e21987: f64 = (locals.var_nuendd * locals.var_pdmer);
        let assign14790_e21990: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14790_e21991: f64 = (assign14790_e21987 + assign14790_e21990);
        (assign14790_e21991, ((locals.var_nuendd * locals.var_pdmer_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdmer_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdmer_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdmer_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdmer_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14790_e21993;
        locals.var_temp_pdeff_dn0 = assign14790_e21993_d_n0;
        locals.var_temp_pdeff_dn2 = assign14790_e21993_d_n2;
        locals.var_temp_pdeff_dn3 = assign14790_e21993_d_n3;
        locals.var_temp_pdeff_dn4 = assign14790_e21993_d_n4;
        locals.var_temp_pdeff_dn5 = assign14790_e21993_d_n5;
        locals.var_temp_pdeff_dn6 = assign14790_e21993_d_n6;
        locals.var_temp_pdeff_dn7 = assign14790_e21993_d_n7;
        locals.var_temp_pdeff_dn8 = assign14790_e21993_d_n8;
        locals.var_temp_pdeff_dn9 = assign14790_e21993_d_n9;
        locals.var_temp_pdeff_dn10 = assign14790_e21993_d_n10;
        locals.var_temp_pdeff_dn11 = assign14790_e21993_d_n11;
        locals.var_temp_pdeff_dn12 = assign14790_e21993_d_n12;
        locals.var_temp_pdeff_dn13 = assign14790_e21993_d_n13;
        locals.var_temp_pdeff_dn14 = assign14790_e21993_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14800_e22020, assign14800_e22020_d_n0, assign14800_e22020_d_n2, assign14800_e22020_d_n3, assign14800_e22020_d_n4, assign14800_e22020_d_n5, assign14800_e22020_d_n6, assign14800_e22020_d_n7, assign14800_e22020_d_n8, assign14800_e22020_d_n9, assign14800_e22020_d_n10, assign14800_e22020_d_n11, assign14800_e22020_d_n12, assign14800_e22020_d_n13, assign14800_e22020_d_n14,) = {
    if ((locals.var_guard473 != 0.0) && (!((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)))) {
        let assign14800_e22014: f64 = (locals.var_nuends * locals.var_asmer);
        let assign14800_e22017: f64 = (locals.var_nuints * locals.var_assha);
        let assign14800_e22018: f64 = (assign14800_e22014 + assign14800_e22017);
        (assign14800_e22018, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14800_e22020;
        locals.var_temp_aseff_dn0 = assign14800_e22020_d_n0;
        locals.var_temp_aseff_dn2 = assign14800_e22020_d_n2;
        locals.var_temp_aseff_dn3 = assign14800_e22020_d_n3;
        locals.var_temp_aseff_dn4 = assign14800_e22020_d_n4;
        locals.var_temp_aseff_dn5 = assign14800_e22020_d_n5;
        locals.var_temp_aseff_dn6 = assign14800_e22020_d_n6;
        locals.var_temp_aseff_dn7 = assign14800_e22020_d_n7;
        locals.var_temp_aseff_dn8 = assign14800_e22020_d_n8;
        locals.var_temp_aseff_dn9 = assign14800_e22020_d_n9;
        locals.var_temp_aseff_dn10 = assign14800_e22020_d_n10;
        locals.var_temp_aseff_dn11 = assign14800_e22020_d_n11;
        locals.var_temp_aseff_dn12 = assign14800_e22020_d_n12;
        locals.var_temp_aseff_dn13 = assign14800_e22020_d_n13;
        locals.var_temp_aseff_dn14 = assign14800_e22020_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14810_e22047, assign14810_e22047_d_n0, assign14810_e22047_d_n2, assign14810_e22047_d_n3, assign14810_e22047_d_n4, assign14810_e22047_d_n5, assign14810_e22047_d_n6, assign14810_e22047_d_n7, assign14810_e22047_d_n8, assign14810_e22047_d_n9, assign14810_e22047_d_n10, assign14810_e22047_d_n11, assign14810_e22047_d_n12, assign14810_e22047_d_n13, assign14810_e22047_d_n14,) = {
    if ((locals.var_guard473 != 0.0) && (!((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)))) {
        let assign14810_e22041: f64 = (locals.var_nuendd * locals.var_admer);
        let assign14810_e22044: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14810_e22045: f64 = (assign14810_e22041 + assign14810_e22044);
        (assign14810_e22045, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14810_e22047;
        locals.var_temp_adeff_dn0 = assign14810_e22047_d_n0;
        locals.var_temp_adeff_dn2 = assign14810_e22047_d_n2;
        locals.var_temp_adeff_dn3 = assign14810_e22047_d_n3;
        locals.var_temp_adeff_dn4 = assign14810_e22047_d_n4;
        locals.var_temp_adeff_dn5 = assign14810_e22047_d_n5;
        locals.var_temp_adeff_dn6 = assign14810_e22047_d_n6;
        locals.var_temp_adeff_dn7 = assign14810_e22047_d_n7;
        locals.var_temp_adeff_dn8 = assign14810_e22047_d_n8;
        locals.var_temp_adeff_dn9 = assign14810_e22047_d_n9;
        locals.var_temp_adeff_dn10 = assign14810_e22047_d_n10;
        locals.var_temp_adeff_dn11 = assign14810_e22047_d_n11;
        locals.var_temp_adeff_dn12 = assign14810_e22047_d_n12;
        locals.var_temp_adeff_dn13 = assign14810_e22047_d_n13;
        locals.var_temp_adeff_dn14 = assign14810_e22047_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14820_e22076, assign14820_e22076_d_n0, assign14820_e22076_d_n2, assign14820_e22076_d_n3, assign14820_e22076_d_n4, assign14820_e22076_d_n5, assign14820_e22076_d_n6, assign14820_e22076_d_n7, assign14820_e22076_d_n8, assign14820_e22076_d_n9, assign14820_e22076_d_n10, assign14820_e22076_d_n11, assign14820_e22076_d_n12, assign14820_e22076_d_n13, assign14820_e22076_d_n14,) = {
    if ((locals.var_guard474 != 0.0) && (!(((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)))) {
        let assign14820_e22071: f64 = (p.p2 - 1.0);
        let assign14820_e22073: f64 = (assign14820_e22071 * locals.var_pssha);
        let assign14820_e22074: f64 = (locals.var_psiso + assign14820_e22073);
        (assign14820_e22074, (locals.var_psiso_dn0 + (assign14820_e22071 * locals.var_pssha_dn0)), (locals.var_psiso_dn2 + (assign14820_e22071 * locals.var_pssha_dn2)), (locals.var_psiso_dn3 + (assign14820_e22071 * locals.var_pssha_dn3)), (locals.var_psiso_dn4 + (assign14820_e22071 * locals.var_pssha_dn4)), (locals.var_psiso_dn5 + (assign14820_e22071 * locals.var_pssha_dn5)), (locals.var_psiso_dn6 + (assign14820_e22071 * locals.var_pssha_dn6)), (locals.var_psiso_dn7 + (assign14820_e22071 * locals.var_pssha_dn7)), (locals.var_psiso_dn8 + (assign14820_e22071 * locals.var_pssha_dn8)), (locals.var_psiso_dn9 + (assign14820_e22071 * locals.var_pssha_dn9)), (locals.var_psiso_dn10 + (assign14820_e22071 * locals.var_pssha_dn10)), (locals.var_psiso_dn11 + (assign14820_e22071 * locals.var_pssha_dn11)), (locals.var_psiso_dn12 + (assign14820_e22071 * locals.var_pssha_dn12)), (locals.var_psiso_dn13 + (assign14820_e22071 * locals.var_pssha_dn13)), (locals.var_psiso_dn14 + (assign14820_e22071 * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14820_e22076;
        locals.var_temp_pseff_dn0 = assign14820_e22076_d_n0;
        locals.var_temp_pseff_dn2 = assign14820_e22076_d_n2;
        locals.var_temp_pseff_dn3 = assign14820_e22076_d_n3;
        locals.var_temp_pseff_dn4 = assign14820_e22076_d_n4;
        locals.var_temp_pseff_dn5 = assign14820_e22076_d_n5;
        locals.var_temp_pseff_dn6 = assign14820_e22076_d_n6;
        locals.var_temp_pseff_dn7 = assign14820_e22076_d_n7;
        locals.var_temp_pseff_dn8 = assign14820_e22076_d_n8;
        locals.var_temp_pseff_dn9 = assign14820_e22076_d_n9;
        locals.var_temp_pseff_dn10 = assign14820_e22076_d_n10;
        locals.var_temp_pseff_dn11 = assign14820_e22076_d_n11;
        locals.var_temp_pseff_dn12 = assign14820_e22076_d_n12;
        locals.var_temp_pseff_dn13 = assign14820_e22076_d_n13;
        locals.var_temp_pseff_dn14 = assign14820_e22076_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14830_e22101, assign14830_e22101_d_n0, assign14830_e22101_d_n2, assign14830_e22101_d_n3, assign14830_e22101_d_n4, assign14830_e22101_d_n5, assign14830_e22101_d_n6, assign14830_e22101_d_n7, assign14830_e22101_d_n8, assign14830_e22101_d_n9, assign14830_e22101_d_n10, assign14830_e22101_d_n11, assign14830_e22101_d_n12, assign14830_e22101_d_n13, assign14830_e22101_d_n14,) = {
    if ((locals.var_guard474 != 0.0) && (!(((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)))) {
        let assign14830_e22099: f64 = (p.p2 * locals.var_pdsha);
        (assign14830_e22099, (p.p2 * locals.var_pdsha_dn0), (p.p2 * locals.var_pdsha_dn2), (p.p2 * locals.var_pdsha_dn3), (p.p2 * locals.var_pdsha_dn4), (p.p2 * locals.var_pdsha_dn5), (p.p2 * locals.var_pdsha_dn6), (p.p2 * locals.var_pdsha_dn7), (p.p2 * locals.var_pdsha_dn8), (p.p2 * locals.var_pdsha_dn9), (p.p2 * locals.var_pdsha_dn10), (p.p2 * locals.var_pdsha_dn11), (p.p2 * locals.var_pdsha_dn12), (p.p2 * locals.var_pdsha_dn13), (p.p2 * locals.var_pdsha_dn14),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14830_e22101;
        locals.var_temp_pdeff_dn0 = assign14830_e22101_d_n0;
        locals.var_temp_pdeff_dn2 = assign14830_e22101_d_n2;
        locals.var_temp_pdeff_dn3 = assign14830_e22101_d_n3;
        locals.var_temp_pdeff_dn4 = assign14830_e22101_d_n4;
        locals.var_temp_pdeff_dn5 = assign14830_e22101_d_n5;
        locals.var_temp_pdeff_dn6 = assign14830_e22101_d_n6;
        locals.var_temp_pdeff_dn7 = assign14830_e22101_d_n7;
        locals.var_temp_pdeff_dn8 = assign14830_e22101_d_n8;
        locals.var_temp_pdeff_dn9 = assign14830_e22101_d_n9;
        locals.var_temp_pdeff_dn10 = assign14830_e22101_d_n10;
        locals.var_temp_pdeff_dn11 = assign14830_e22101_d_n11;
        locals.var_temp_pdeff_dn12 = assign14830_e22101_d_n12;
        locals.var_temp_pdeff_dn13 = assign14830_e22101_d_n13;
        locals.var_temp_pdeff_dn14 = assign14830_e22101_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14840_e22130, assign14840_e22130_d_n0, assign14840_e22130_d_n2, assign14840_e22130_d_n3, assign14840_e22130_d_n4, assign14840_e22130_d_n5, assign14840_e22130_d_n6, assign14840_e22130_d_n7, assign14840_e22130_d_n8, assign14840_e22130_d_n9, assign14840_e22130_d_n10, assign14840_e22130_d_n11, assign14840_e22130_d_n12, assign14840_e22130_d_n13, assign14840_e22130_d_n14,) = {
    if ((locals.var_guard474 != 0.0) && (!(((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)))) {
        let assign14840_e22125: f64 = (p.p2 - 1.0);
        let assign14840_e22127: f64 = (assign14840_e22125 * locals.var_assha);
        let assign14840_e22128: f64 = (locals.var_asiso + assign14840_e22127);
        (assign14840_e22128, locals.var_asiso_dn0, locals.var_asiso_dn2, locals.var_asiso_dn3, locals.var_asiso_dn4, locals.var_asiso_dn5, locals.var_asiso_dn6, locals.var_asiso_dn7, locals.var_asiso_dn8, locals.var_asiso_dn9, locals.var_asiso_dn10, locals.var_asiso_dn11, locals.var_asiso_dn12, locals.var_asiso_dn13, locals.var_asiso_dn14,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14840_e22130;
        locals.var_temp_aseff_dn0 = assign14840_e22130_d_n0;
        locals.var_temp_aseff_dn2 = assign14840_e22130_d_n2;
        locals.var_temp_aseff_dn3 = assign14840_e22130_d_n3;
        locals.var_temp_aseff_dn4 = assign14840_e22130_d_n4;
        locals.var_temp_aseff_dn5 = assign14840_e22130_d_n5;
        locals.var_temp_aseff_dn6 = assign14840_e22130_d_n6;
        locals.var_temp_aseff_dn7 = assign14840_e22130_d_n7;
        locals.var_temp_aseff_dn8 = assign14840_e22130_d_n8;
        locals.var_temp_aseff_dn9 = assign14840_e22130_d_n9;
        locals.var_temp_aseff_dn10 = assign14840_e22130_d_n10;
        locals.var_temp_aseff_dn11 = assign14840_e22130_d_n11;
        locals.var_temp_aseff_dn12 = assign14840_e22130_d_n12;
        locals.var_temp_aseff_dn13 = assign14840_e22130_d_n13;
        locals.var_temp_aseff_dn14 = assign14840_e22130_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14850_e22155, assign14850_e22155_d_n0, assign14850_e22155_d_n2, assign14850_e22155_d_n3, assign14850_e22155_d_n4, assign14850_e22155_d_n5, assign14850_e22155_d_n6, assign14850_e22155_d_n7, assign14850_e22155_d_n8, assign14850_e22155_d_n9, assign14850_e22155_d_n10, assign14850_e22155_d_n11, assign14850_e22155_d_n12, assign14850_e22155_d_n13, assign14850_e22155_d_n14,) = {
    if ((locals.var_guard474 != 0.0) && (!(((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)))) {
        let assign14850_e22153: f64 = (p.p2 * locals.var_adsha);
        (assign14850_e22153, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14850_e22155;
        locals.var_temp_adeff_dn0 = assign14850_e22155_d_n0;
        locals.var_temp_adeff_dn2 = assign14850_e22155_d_n2;
        locals.var_temp_adeff_dn3 = assign14850_e22155_d_n3;
        locals.var_temp_adeff_dn4 = assign14850_e22155_d_n4;
        locals.var_temp_adeff_dn5 = assign14850_e22155_d_n5;
        locals.var_temp_adeff_dn6 = assign14850_e22155_d_n6;
        locals.var_temp_adeff_dn7 = assign14850_e22155_d_n7;
        locals.var_temp_adeff_dn8 = assign14850_e22155_d_n8;
        locals.var_temp_adeff_dn9 = assign14850_e22155_d_n9;
        locals.var_temp_adeff_dn10 = assign14850_e22155_d_n10;
        locals.var_temp_adeff_dn11 = assign14850_e22155_d_n11;
        locals.var_temp_adeff_dn12 = assign14850_e22155_d_n12;
        locals.var_temp_adeff_dn13 = assign14850_e22155_d_n13;
        locals.var_temp_adeff_dn14 = assign14850_e22155_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14860_e22182, assign14860_e22182_d_n0, assign14860_e22182_d_n2, assign14860_e22182_d_n3, assign14860_e22182_d_n4, assign14860_e22182_d_n5, assign14860_e22182_d_n6, assign14860_e22182_d_n7, assign14860_e22182_d_n8, assign14860_e22182_d_n9, assign14860_e22182_d_n10, assign14860_e22182_d_n11, assign14860_e22182_d_n12, assign14860_e22182_d_n13, assign14860_e22182_d_n14,) = {
    if ((locals.var_guard475 != 0.0) && (!((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)))) {
        let assign14860_e22180: f64 = (p.p2 * locals.var_pssha);
        (assign14860_e22180, (p.p2 * locals.var_pssha_dn0), (p.p2 * locals.var_pssha_dn2), (p.p2 * locals.var_pssha_dn3), (p.p2 * locals.var_pssha_dn4), (p.p2 * locals.var_pssha_dn5), (p.p2 * locals.var_pssha_dn6), (p.p2 * locals.var_pssha_dn7), (p.p2 * locals.var_pssha_dn8), (p.p2 * locals.var_pssha_dn9), (p.p2 * locals.var_pssha_dn10), (p.p2 * locals.var_pssha_dn11), (p.p2 * locals.var_pssha_dn12), (p.p2 * locals.var_pssha_dn13), (p.p2 * locals.var_pssha_dn14),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14860_e22182;
        locals.var_temp_pseff_dn0 = assign14860_e22182_d_n0;
        locals.var_temp_pseff_dn2 = assign14860_e22182_d_n2;
        locals.var_temp_pseff_dn3 = assign14860_e22182_d_n3;
        locals.var_temp_pseff_dn4 = assign14860_e22182_d_n4;
        locals.var_temp_pseff_dn5 = assign14860_e22182_d_n5;
        locals.var_temp_pseff_dn6 = assign14860_e22182_d_n6;
        locals.var_temp_pseff_dn7 = assign14860_e22182_d_n7;
        locals.var_temp_pseff_dn8 = assign14860_e22182_d_n8;
        locals.var_temp_pseff_dn9 = assign14860_e22182_d_n9;
        locals.var_temp_pseff_dn10 = assign14860_e22182_d_n10;
        locals.var_temp_pseff_dn11 = assign14860_e22182_d_n11;
        locals.var_temp_pseff_dn12 = assign14860_e22182_d_n12;
        locals.var_temp_pseff_dn13 = assign14860_e22182_d_n13;
        locals.var_temp_pseff_dn14 = assign14860_e22182_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14870_e22213, assign14870_e22213_d_n0, assign14870_e22213_d_n2, assign14870_e22213_d_n3, assign14870_e22213_d_n4, assign14870_e22213_d_n5, assign14870_e22213_d_n6, assign14870_e22213_d_n7, assign14870_e22213_d_n8, assign14870_e22213_d_n9, assign14870_e22213_d_n10, assign14870_e22213_d_n11, assign14870_e22213_d_n12, assign14870_e22213_d_n13, assign14870_e22213_d_n14,) = {
    if ((locals.var_guard475 != 0.0) && (!((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)))) {
        let assign14870_e22208: f64 = (p.p2 - 1.0);
        let assign14870_e22210: f64 = (assign14870_e22208 * locals.var_pdsha);
        let assign14870_e22211: f64 = (locals.var_pdiso + assign14870_e22210);
        (assign14870_e22211, (locals.var_pdiso_dn0 + (assign14870_e22208 * locals.var_pdsha_dn0)), (locals.var_pdiso_dn2 + (assign14870_e22208 * locals.var_pdsha_dn2)), (locals.var_pdiso_dn3 + (assign14870_e22208 * locals.var_pdsha_dn3)), (locals.var_pdiso_dn4 + (assign14870_e22208 * locals.var_pdsha_dn4)), (locals.var_pdiso_dn5 + (assign14870_e22208 * locals.var_pdsha_dn5)), (locals.var_pdiso_dn6 + (assign14870_e22208 * locals.var_pdsha_dn6)), (locals.var_pdiso_dn7 + (assign14870_e22208 * locals.var_pdsha_dn7)), (locals.var_pdiso_dn8 + (assign14870_e22208 * locals.var_pdsha_dn8)), (locals.var_pdiso_dn9 + (assign14870_e22208 * locals.var_pdsha_dn9)), (locals.var_pdiso_dn10 + (assign14870_e22208 * locals.var_pdsha_dn10)), (locals.var_pdiso_dn11 + (assign14870_e22208 * locals.var_pdsha_dn11)), (locals.var_pdiso_dn12 + (assign14870_e22208 * locals.var_pdsha_dn12)), (locals.var_pdiso_dn13 + (assign14870_e22208 * locals.var_pdsha_dn13)), (locals.var_pdiso_dn14 + (assign14870_e22208 * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14870_e22213;
        locals.var_temp_pdeff_dn0 = assign14870_e22213_d_n0;
        locals.var_temp_pdeff_dn2 = assign14870_e22213_d_n2;
        locals.var_temp_pdeff_dn3 = assign14870_e22213_d_n3;
        locals.var_temp_pdeff_dn4 = assign14870_e22213_d_n4;
        locals.var_temp_pdeff_dn5 = assign14870_e22213_d_n5;
        locals.var_temp_pdeff_dn6 = assign14870_e22213_d_n6;
        locals.var_temp_pdeff_dn7 = assign14870_e22213_d_n7;
        locals.var_temp_pdeff_dn8 = assign14870_e22213_d_n8;
        locals.var_temp_pdeff_dn9 = assign14870_e22213_d_n9;
        locals.var_temp_pdeff_dn10 = assign14870_e22213_d_n10;
        locals.var_temp_pdeff_dn11 = assign14870_e22213_d_n11;
        locals.var_temp_pdeff_dn12 = assign14870_e22213_d_n12;
        locals.var_temp_pdeff_dn13 = assign14870_e22213_d_n13;
        locals.var_temp_pdeff_dn14 = assign14870_e22213_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14880_e22240, assign14880_e22240_d_n0, assign14880_e22240_d_n2, assign14880_e22240_d_n3, assign14880_e22240_d_n4, assign14880_e22240_d_n5, assign14880_e22240_d_n6, assign14880_e22240_d_n7, assign14880_e22240_d_n8, assign14880_e22240_d_n9, assign14880_e22240_d_n10, assign14880_e22240_d_n11, assign14880_e22240_d_n12, assign14880_e22240_d_n13, assign14880_e22240_d_n14,) = {
    if ((locals.var_guard475 != 0.0) && (!((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)))) {
        let assign14880_e22238: f64 = (p.p2 * locals.var_assha);
        (assign14880_e22238, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14880_e22240;
        locals.var_temp_aseff_dn0 = assign14880_e22240_d_n0;
        locals.var_temp_aseff_dn2 = assign14880_e22240_d_n2;
        locals.var_temp_aseff_dn3 = assign14880_e22240_d_n3;
        locals.var_temp_aseff_dn4 = assign14880_e22240_d_n4;
        locals.var_temp_aseff_dn5 = assign14880_e22240_d_n5;
        locals.var_temp_aseff_dn6 = assign14880_e22240_d_n6;
        locals.var_temp_aseff_dn7 = assign14880_e22240_d_n7;
        locals.var_temp_aseff_dn8 = assign14880_e22240_d_n8;
        locals.var_temp_aseff_dn9 = assign14880_e22240_d_n9;
        locals.var_temp_aseff_dn10 = assign14880_e22240_d_n10;
        locals.var_temp_aseff_dn11 = assign14880_e22240_d_n11;
        locals.var_temp_aseff_dn12 = assign14880_e22240_d_n12;
        locals.var_temp_aseff_dn13 = assign14880_e22240_d_n13;
        locals.var_temp_aseff_dn14 = assign14880_e22240_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14890_e22271, assign14890_e22271_d_n0, assign14890_e22271_d_n2, assign14890_e22271_d_n3, assign14890_e22271_d_n4, assign14890_e22271_d_n5, assign14890_e22271_d_n6, assign14890_e22271_d_n7, assign14890_e22271_d_n8, assign14890_e22271_d_n9, assign14890_e22271_d_n10, assign14890_e22271_d_n11, assign14890_e22271_d_n12, assign14890_e22271_d_n13, assign14890_e22271_d_n14,) = {
    if ((locals.var_guard475 != 0.0) && (!((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)))) {
        let assign14890_e22266: f64 = (p.p2 - 1.0);
        let assign14890_e22268: f64 = (assign14890_e22266 * locals.var_adsha);
        let assign14890_e22269: f64 = (locals.var_adiso + assign14890_e22268);
        (assign14890_e22269, locals.var_adiso_dn0, locals.var_adiso_dn2, locals.var_adiso_dn3, locals.var_adiso_dn4, locals.var_adiso_dn5, locals.var_adiso_dn6, locals.var_adiso_dn7, locals.var_adiso_dn8, locals.var_adiso_dn9, locals.var_adiso_dn10, locals.var_adiso_dn11, locals.var_adiso_dn12, locals.var_adiso_dn13, locals.var_adiso_dn14,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14890_e22271;
        locals.var_temp_adeff_dn0 = assign14890_e22271_d_n0;
        locals.var_temp_adeff_dn2 = assign14890_e22271_d_n2;
        locals.var_temp_adeff_dn3 = assign14890_e22271_d_n3;
        locals.var_temp_adeff_dn4 = assign14890_e22271_d_n4;
        locals.var_temp_adeff_dn5 = assign14890_e22271_d_n5;
        locals.var_temp_adeff_dn6 = assign14890_e22271_d_n6;
        locals.var_temp_adeff_dn7 = assign14890_e22271_d_n7;
        locals.var_temp_adeff_dn8 = assign14890_e22271_d_n8;
        locals.var_temp_adeff_dn9 = assign14890_e22271_d_n9;
        locals.var_temp_adeff_dn10 = assign14890_e22271_d_n10;
        locals.var_temp_adeff_dn11 = assign14890_e22271_d_n11;
        locals.var_temp_adeff_dn12 = assign14890_e22271_d_n12;
        locals.var_temp_adeff_dn13 = assign14890_e22271_d_n13;
        locals.var_temp_adeff_dn14 = assign14890_e22271_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14900_e22296, assign14900_e22296_d_n0, assign14900_e22296_d_n2, assign14900_e22296_d_n3, assign14900_e22296_d_n4, assign14900_e22296_d_n5, assign14900_e22296_d_n6, assign14900_e22296_d_n7, assign14900_e22296_d_n8, assign14900_e22296_d_n9, assign14900_e22296_d_n10, assign14900_e22296_d_n11, assign14900_e22296_d_n12, assign14900_e22296_d_n13, assign14900_e22296_d_n14,) = {
    if (!(((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)) || (locals.var_guard475 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14900_e22296;
        locals.var_temp_pseff_dn0 = assign14900_e22296_d_n0;
        locals.var_temp_pseff_dn2 = assign14900_e22296_d_n2;
        locals.var_temp_pseff_dn3 = assign14900_e22296_d_n3;
        locals.var_temp_pseff_dn4 = assign14900_e22296_d_n4;
        locals.var_temp_pseff_dn5 = assign14900_e22296_d_n5;
        locals.var_temp_pseff_dn6 = assign14900_e22296_d_n6;
        locals.var_temp_pseff_dn7 = assign14900_e22296_d_n7;
        locals.var_temp_pseff_dn8 = assign14900_e22296_d_n8;
        locals.var_temp_pseff_dn9 = assign14900_e22296_d_n9;
        locals.var_temp_pseff_dn10 = assign14900_e22296_d_n10;
        locals.var_temp_pseff_dn11 = assign14900_e22296_d_n11;
        locals.var_temp_pseff_dn12 = assign14900_e22296_d_n12;
        locals.var_temp_pseff_dn13 = assign14900_e22296_d_n13;
        locals.var_temp_pseff_dn14 = assign14900_e22296_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14910_e22321, assign14910_e22321_d_n0, assign14910_e22321_d_n2, assign14910_e22321_d_n3, assign14910_e22321_d_n4, assign14910_e22321_d_n5, assign14910_e22321_d_n6, assign14910_e22321_d_n7, assign14910_e22321_d_n8, assign14910_e22321_d_n9, assign14910_e22321_d_n10, assign14910_e22321_d_n11, assign14910_e22321_d_n12, assign14910_e22321_d_n13, assign14910_e22321_d_n14,) = {
    if (!(((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)) || (locals.var_guard475 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14910_e22321;
        locals.var_temp_pdeff_dn0 = assign14910_e22321_d_n0;
        locals.var_temp_pdeff_dn2 = assign14910_e22321_d_n2;
        locals.var_temp_pdeff_dn3 = assign14910_e22321_d_n3;
        locals.var_temp_pdeff_dn4 = assign14910_e22321_d_n4;
        locals.var_temp_pdeff_dn5 = assign14910_e22321_d_n5;
        locals.var_temp_pdeff_dn6 = assign14910_e22321_d_n6;
        locals.var_temp_pdeff_dn7 = assign14910_e22321_d_n7;
        locals.var_temp_pdeff_dn8 = assign14910_e22321_d_n8;
        locals.var_temp_pdeff_dn9 = assign14910_e22321_d_n9;
        locals.var_temp_pdeff_dn10 = assign14910_e22321_d_n10;
        locals.var_temp_pdeff_dn11 = assign14910_e22321_d_n11;
        locals.var_temp_pdeff_dn12 = assign14910_e22321_d_n12;
        locals.var_temp_pdeff_dn13 = assign14910_e22321_d_n13;
        locals.var_temp_pdeff_dn14 = assign14910_e22321_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14920_e22346, assign14920_e22346_d_n0, assign14920_e22346_d_n2, assign14920_e22346_d_n3, assign14920_e22346_d_n4, assign14920_e22346_d_n5, assign14920_e22346_d_n6, assign14920_e22346_d_n7, assign14920_e22346_d_n8, assign14920_e22346_d_n9, assign14920_e22346_d_n10, assign14920_e22346_d_n11, assign14920_e22346_d_n12, assign14920_e22346_d_n13, assign14920_e22346_d_n14,) = {
    if (!(((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)) || (locals.var_guard475 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14920_e22346;
        locals.var_temp_aseff_dn0 = assign14920_e22346_d_n0;
        locals.var_temp_aseff_dn2 = assign14920_e22346_d_n2;
        locals.var_temp_aseff_dn3 = assign14920_e22346_d_n3;
        locals.var_temp_aseff_dn4 = assign14920_e22346_d_n4;
        locals.var_temp_aseff_dn5 = assign14920_e22346_d_n5;
        locals.var_temp_aseff_dn6 = assign14920_e22346_d_n6;
        locals.var_temp_aseff_dn7 = assign14920_e22346_d_n7;
        locals.var_temp_aseff_dn8 = assign14920_e22346_d_n8;
        locals.var_temp_aseff_dn9 = assign14920_e22346_d_n9;
        locals.var_temp_aseff_dn10 = assign14920_e22346_d_n10;
        locals.var_temp_aseff_dn11 = assign14920_e22346_d_n11;
        locals.var_temp_aseff_dn12 = assign14920_e22346_d_n12;
        locals.var_temp_aseff_dn13 = assign14920_e22346_d_n13;
        locals.var_temp_aseff_dn14 = assign14920_e22346_d_n14;
        locals.var_temp_aseff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_31(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign14930_e22371, assign14930_e22371_d_n0, assign14930_e22371_d_n2, assign14930_e22371_d_n3, assign14930_e22371_d_n4, assign14930_e22371_d_n5, assign14930_e22371_d_n6, assign14930_e22371_d_n7, assign14930_e22371_d_n8, assign14930_e22371_d_n9, assign14930_e22371_d_n10, assign14930_e22371_d_n11, assign14930_e22371_d_n12, assign14930_e22371_d_n13, assign14930_e22371_d_n14,) = {
    if (!(((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)) || (locals.var_guard475 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14930_e22371;
        locals.var_temp_adeff_dn0 = assign14930_e22371_d_n0;
        locals.var_temp_adeff_dn2 = assign14930_e22371_d_n2;
        locals.var_temp_adeff_dn3 = assign14930_e22371_d_n3;
        locals.var_temp_adeff_dn4 = assign14930_e22371_d_n4;
        locals.var_temp_adeff_dn5 = assign14930_e22371_d_n5;
        locals.var_temp_adeff_dn6 = assign14930_e22371_d_n6;
        locals.var_temp_adeff_dn7 = assign14930_e22371_d_n7;
        locals.var_temp_adeff_dn8 = assign14930_e22371_d_n8;
        locals.var_temp_adeff_dn9 = assign14930_e22371_d_n9;
        locals.var_temp_adeff_dn10 = assign14930_e22371_d_n10;
        locals.var_temp_adeff_dn11 = assign14930_e22371_d_n11;
        locals.var_temp_adeff_dn12 = assign14930_e22371_d_n12;
        locals.var_temp_adeff_dn13 = assign14930_e22371_d_n13;
        locals.var_temp_adeff_dn14 = assign14930_e22371_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let assign14940_e22373: f64 = if param_given[24] { 1.0 } else { 0.0 };
        locals.var_guard476 = assign14940_e22373;
        locals.var_guard476_rv = 0.0;

        let (assign14950_e22381, assign14950_e22381_d_n0, assign14950_e22381_d_n2, assign14950_e22381_d_n3, assign14950_e22381_d_n4, assign14950_e22381_d_n5, assign14950_e22381_d_n6, assign14950_e22381_d_n7, assign14950_e22381_d_n8, assign14950_e22381_d_n9, assign14950_e22381_d_n10, assign14950_e22381_d_n11, assign14950_e22381_d_n12, assign14950_e22381_d_n13, assign14950_e22381_d_n14,) = {
    if (locals.var_guard476 != 0.0) {
        let assign14950_e22377: f64 = (p.p24 * p.p53);
        let assign14950_e22379: f64 = (assign14950_e22377 * p.p52);
        (assign14950_e22379, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_aseff, locals.var_aseff_dn0, locals.var_aseff_dn2, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11, locals.var_aseff_dn12, locals.var_aseff_dn13, locals.var_aseff_dn14,)
    }
};
        locals.var_aseff = assign14950_e22381;
        locals.var_aseff_dn0 = assign14950_e22381_d_n0;
        locals.var_aseff_dn2 = assign14950_e22381_d_n2;
        locals.var_aseff_dn3 = assign14950_e22381_d_n3;
        locals.var_aseff_dn4 = assign14950_e22381_d_n4;
        locals.var_aseff_dn5 = assign14950_e22381_d_n5;
        locals.var_aseff_dn6 = assign14950_e22381_d_n6;
        locals.var_aseff_dn7 = assign14950_e22381_d_n7;
        locals.var_aseff_dn8 = assign14950_e22381_d_n8;
        locals.var_aseff_dn9 = assign14950_e22381_d_n9;
        locals.var_aseff_dn10 = assign14950_e22381_d_n10;
        locals.var_aseff_dn11 = assign14950_e22381_d_n11;
        locals.var_aseff_dn12 = assign14950_e22381_d_n12;
        locals.var_aseff_dn13 = assign14950_e22381_d_n13;
        locals.var_aseff_dn14 = assign14950_e22381_d_n14;
        locals.var_aseff_rv = 0.0;

        let (assign14960_e22386, assign14960_e22386_d_n0, assign14960_e22386_d_n2, assign14960_e22386_d_n3, assign14960_e22386_d_n4, assign14960_e22386_d_n5, assign14960_e22386_d_n6, assign14960_e22386_d_n7, assign14960_e22386_d_n8, assign14960_e22386_d_n9, assign14960_e22386_d_n10, assign14960_e22386_d_n11, assign14960_e22386_d_n12, assign14960_e22386_d_n13, assign14960_e22386_d_n14,) = {
    if (locals.var_guard476 == 0.0) {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    } else {
        (locals.var_aseff, locals.var_aseff_dn0, locals.var_aseff_dn2, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11, locals.var_aseff_dn12, locals.var_aseff_dn13, locals.var_aseff_dn14,)
    }
};
        locals.var_aseff = assign14960_e22386;
        locals.var_aseff_dn0 = assign14960_e22386_d_n0;
        locals.var_aseff_dn2 = assign14960_e22386_d_n2;
        locals.var_aseff_dn3 = assign14960_e22386_d_n3;
        locals.var_aseff_dn4 = assign14960_e22386_d_n4;
        locals.var_aseff_dn5 = assign14960_e22386_d_n5;
        locals.var_aseff_dn6 = assign14960_e22386_d_n6;
        locals.var_aseff_dn7 = assign14960_e22386_d_n7;
        locals.var_aseff_dn8 = assign14960_e22386_d_n8;
        locals.var_aseff_dn9 = assign14960_e22386_d_n9;
        locals.var_aseff_dn10 = assign14960_e22386_d_n10;
        locals.var_aseff_dn11 = assign14960_e22386_d_n11;
        locals.var_aseff_dn12 = assign14960_e22386_d_n12;
        locals.var_aseff_dn13 = assign14960_e22386_d_n13;
        locals.var_aseff_dn14 = assign14960_e22386_d_n14;
        locals.var_aseff_rv = 0.0;

        let assign14970_e22389: f64 = if locals.var_aseff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard477 = assign14970_e22389;
        locals.var_guard477_rv = 0.0;

        let (assign14980_e22393, assign14980_e22393_d_n0, assign14980_e22393_d_n2, assign14980_e22393_d_n3, assign14980_e22393_d_n4, assign14980_e22393_d_n5, assign14980_e22393_d_n6, assign14980_e22393_d_n7, assign14980_e22393_d_n8, assign14980_e22393_d_n9, assign14980_e22393_d_n10, assign14980_e22393_d_n11, assign14980_e22393_d_n12, assign14980_e22393_d_n13, assign14980_e22393_d_n14,) = {
    if (locals.var_guard477 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_aseff, locals.var_aseff_dn0, locals.var_aseff_dn2, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11, locals.var_aseff_dn12, locals.var_aseff_dn13, locals.var_aseff_dn14,)
    }
};
        locals.var_aseff = assign14980_e22393;
        locals.var_aseff_dn0 = assign14980_e22393_d_n0;
        locals.var_aseff_dn2 = assign14980_e22393_d_n2;
        locals.var_aseff_dn3 = assign14980_e22393_d_n3;
        locals.var_aseff_dn4 = assign14980_e22393_d_n4;
        locals.var_aseff_dn5 = assign14980_e22393_d_n5;
        locals.var_aseff_dn6 = assign14980_e22393_d_n6;
        locals.var_aseff_dn7 = assign14980_e22393_d_n7;
        locals.var_aseff_dn8 = assign14980_e22393_d_n8;
        locals.var_aseff_dn9 = assign14980_e22393_d_n9;
        locals.var_aseff_dn10 = assign14980_e22393_d_n10;
        locals.var_aseff_dn11 = assign14980_e22393_d_n11;
        locals.var_aseff_dn12 = assign14980_e22393_d_n12;
        locals.var_aseff_dn13 = assign14980_e22393_d_n13;
        locals.var_aseff_dn14 = assign14980_e22393_d_n14;
        locals.var_aseff_rv = 0.0;

        let assign14990_e22395: f64 = if param_given[25] { 1.0 } else { 0.0 };
        locals.var_guard478 = assign14990_e22395;
        locals.var_guard478_rv = 0.0;

        let (assign15000_e22403, assign15000_e22403_d_n0, assign15000_e22403_d_n2, assign15000_e22403_d_n3, assign15000_e22403_d_n4, assign15000_e22403_d_n5, assign15000_e22403_d_n6, assign15000_e22403_d_n7, assign15000_e22403_d_n8, assign15000_e22403_d_n9, assign15000_e22403_d_n10, assign15000_e22403_d_n11, assign15000_e22403_d_n12, assign15000_e22403_d_n13, assign15000_e22403_d_n14,) = {
    if (locals.var_guard478 != 0.0) {
        let assign15000_e22399: f64 = (p.p25 * p.p53);
        let assign15000_e22401: f64 = (assign15000_e22399 * p.p52);
        (assign15000_e22401, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_adeff, locals.var_adeff_dn0, locals.var_adeff_dn2, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11, locals.var_adeff_dn12, locals.var_adeff_dn13, locals.var_adeff_dn14,)
    }
};
        locals.var_adeff = assign15000_e22403;
        locals.var_adeff_dn0 = assign15000_e22403_d_n0;
        locals.var_adeff_dn2 = assign15000_e22403_d_n2;
        locals.var_adeff_dn3 = assign15000_e22403_d_n3;
        locals.var_adeff_dn4 = assign15000_e22403_d_n4;
        locals.var_adeff_dn5 = assign15000_e22403_d_n5;
        locals.var_adeff_dn6 = assign15000_e22403_d_n6;
        locals.var_adeff_dn7 = assign15000_e22403_d_n7;
        locals.var_adeff_dn8 = assign15000_e22403_d_n8;
        locals.var_adeff_dn9 = assign15000_e22403_d_n9;
        locals.var_adeff_dn10 = assign15000_e22403_d_n10;
        locals.var_adeff_dn11 = assign15000_e22403_d_n11;
        locals.var_adeff_dn12 = assign15000_e22403_d_n12;
        locals.var_adeff_dn13 = assign15000_e22403_d_n13;
        locals.var_adeff_dn14 = assign15000_e22403_d_n14;
        locals.var_adeff_rv = 0.0;

        let (assign15010_e22408, assign15010_e22408_d_n0, assign15010_e22408_d_n2, assign15010_e22408_d_n3, assign15010_e22408_d_n4, assign15010_e22408_d_n5, assign15010_e22408_d_n6, assign15010_e22408_d_n7, assign15010_e22408_d_n8, assign15010_e22408_d_n9, assign15010_e22408_d_n10, assign15010_e22408_d_n11, assign15010_e22408_d_n12, assign15010_e22408_d_n13, assign15010_e22408_d_n14,) = {
    if (locals.var_guard478 == 0.0) {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    } else {
        (locals.var_adeff, locals.var_adeff_dn0, locals.var_adeff_dn2, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11, locals.var_adeff_dn12, locals.var_adeff_dn13, locals.var_adeff_dn14,)
    }
};
        locals.var_adeff = assign15010_e22408;
        locals.var_adeff_dn0 = assign15010_e22408_d_n0;
        locals.var_adeff_dn2 = assign15010_e22408_d_n2;
        locals.var_adeff_dn3 = assign15010_e22408_d_n3;
        locals.var_adeff_dn4 = assign15010_e22408_d_n4;
        locals.var_adeff_dn5 = assign15010_e22408_d_n5;
        locals.var_adeff_dn6 = assign15010_e22408_d_n6;
        locals.var_adeff_dn7 = assign15010_e22408_d_n7;
        locals.var_adeff_dn8 = assign15010_e22408_d_n8;
        locals.var_adeff_dn9 = assign15010_e22408_d_n9;
        locals.var_adeff_dn10 = assign15010_e22408_d_n10;
        locals.var_adeff_dn11 = assign15010_e22408_d_n11;
        locals.var_adeff_dn12 = assign15010_e22408_d_n12;
        locals.var_adeff_dn13 = assign15010_e22408_d_n13;
        locals.var_adeff_dn14 = assign15010_e22408_d_n14;
        locals.var_adeff_rv = 0.0;

        let assign15020_e22411: f64 = if locals.var_adeff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard479 = assign15020_e22411;
        locals.var_guard479_rv = 0.0;

        let (assign15030_e22415, assign15030_e22415_d_n0, assign15030_e22415_d_n2, assign15030_e22415_d_n3, assign15030_e22415_d_n4, assign15030_e22415_d_n5, assign15030_e22415_d_n6, assign15030_e22415_d_n7, assign15030_e22415_d_n8, assign15030_e22415_d_n9, assign15030_e22415_d_n10, assign15030_e22415_d_n11, assign15030_e22415_d_n12, assign15030_e22415_d_n13, assign15030_e22415_d_n14,) = {
    if (locals.var_guard479 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_adeff, locals.var_adeff_dn0, locals.var_adeff_dn2, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11, locals.var_adeff_dn12, locals.var_adeff_dn13, locals.var_adeff_dn14,)
    }
};
        locals.var_adeff = assign15030_e22415;
        locals.var_adeff_dn0 = assign15030_e22415_d_n0;
        locals.var_adeff_dn2 = assign15030_e22415_d_n2;
        locals.var_adeff_dn3 = assign15030_e22415_d_n3;
        locals.var_adeff_dn4 = assign15030_e22415_d_n4;
        locals.var_adeff_dn5 = assign15030_e22415_d_n5;
        locals.var_adeff_dn6 = assign15030_e22415_d_n6;
        locals.var_adeff_dn7 = assign15030_e22415_d_n7;
        locals.var_adeff_dn8 = assign15030_e22415_d_n8;
        locals.var_adeff_dn9 = assign15030_e22415_d_n9;
        locals.var_adeff_dn10 = assign15030_e22415_d_n10;
        locals.var_adeff_dn11 = assign15030_e22415_d_n11;
        locals.var_adeff_dn12 = assign15030_e22415_d_n12;
        locals.var_adeff_dn13 = assign15030_e22415_d_n13;
        locals.var_adeff_dn14 = assign15030_e22415_d_n14;
        locals.var_adeff_rv = 0.0;

        let assign15040_e22417: f64 = if param_given[26] { 1.0 } else { 0.0 };
        locals.var_guard480 = assign15040_e22417;
        locals.var_guard480_rv = 0.0;

        let assign15050_e22420: f64 = if p.p137 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard481 = assign15050_e22420;
        locals.var_guard481_rv = 0.0;

        let (assign15060_e22428, assign15060_e22428_d_n0, assign15060_e22428_d_n2, assign15060_e22428_d_n3, assign15060_e22428_d_n4, assign15060_e22428_d_n5, assign15060_e22428_d_n6, assign15060_e22428_d_n7, assign15060_e22428_d_n8, assign15060_e22428_d_n9, assign15060_e22428_d_n10, assign15060_e22428_d_n11, assign15060_e22428_d_n12, assign15060_e22428_d_n13, assign15060_e22428_d_n14,) = {
    if ((locals.var_guard480 != 0.0) && (locals.var_guard481 != 0.0)) {
        let assign15060_e22426: f64 = (p.p26 * p.p53);
        (assign15060_e22426, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn0, locals.var_pseff_dn2, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11, locals.var_pseff_dn12, locals.var_pseff_dn13, locals.var_pseff_dn14,)
    }
};
        locals.var_pseff = assign15060_e22428;
        locals.var_pseff_dn0 = assign15060_e22428_d_n0;
        locals.var_pseff_dn2 = assign15060_e22428_d_n2;
        locals.var_pseff_dn3 = assign15060_e22428_d_n3;
        locals.var_pseff_dn4 = assign15060_e22428_d_n4;
        locals.var_pseff_dn5 = assign15060_e22428_d_n5;
        locals.var_pseff_dn6 = assign15060_e22428_d_n6;
        locals.var_pseff_dn7 = assign15060_e22428_d_n7;
        locals.var_pseff_dn8 = assign15060_e22428_d_n8;
        locals.var_pseff_dn9 = assign15060_e22428_d_n9;
        locals.var_pseff_dn10 = assign15060_e22428_d_n10;
        locals.var_pseff_dn11 = assign15060_e22428_d_n11;
        locals.var_pseff_dn12 = assign15060_e22428_d_n12;
        locals.var_pseff_dn13 = assign15060_e22428_d_n13;
        locals.var_pseff_dn14 = assign15060_e22428_d_n14;
        locals.var_pseff_rv = 0.0;

        let (assign15070_e22443, assign15070_e22443_d_n0, assign15070_e22443_d_n2, assign15070_e22443_d_n3, assign15070_e22443_d_n4, assign15070_e22443_d_n5, assign15070_e22443_d_n6, assign15070_e22443_d_n7, assign15070_e22443_d_n8, assign15070_e22443_d_n9, assign15070_e22443_d_n10, assign15070_e22443_d_n11, assign15070_e22443_d_n12, assign15070_e22443_d_n13, assign15070_e22443_d_n14,) = {
    if ((locals.var_guard480 != 0.0) && (locals.var_guard481 == 0.0)) {
        let assign15070_e22435: f64 = (p.p26 * p.p53);
        let assign15070_e22438: f64 = (locals.var_weffcj * p.p2);
        let assign15070_e22439: f64 = (assign15070_e22435 - assign15070_e22438);
        let assign15070_e22441: f64 = (assign15070_e22439).max(0.0);
        (assign15070_e22441, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn0, locals.var_pseff_dn2, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11, locals.var_pseff_dn12, locals.var_pseff_dn13, locals.var_pseff_dn14,)
    }
};
        locals.var_pseff = assign15070_e22443;
        locals.var_pseff_dn0 = assign15070_e22443_d_n0;
        locals.var_pseff_dn2 = assign15070_e22443_d_n2;
        locals.var_pseff_dn3 = assign15070_e22443_d_n3;
        locals.var_pseff_dn4 = assign15070_e22443_d_n4;
        locals.var_pseff_dn5 = assign15070_e22443_d_n5;
        locals.var_pseff_dn6 = assign15070_e22443_d_n6;
        locals.var_pseff_dn7 = assign15070_e22443_d_n7;
        locals.var_pseff_dn8 = assign15070_e22443_d_n8;
        locals.var_pseff_dn9 = assign15070_e22443_d_n9;
        locals.var_pseff_dn10 = assign15070_e22443_d_n10;
        locals.var_pseff_dn11 = assign15070_e22443_d_n11;
        locals.var_pseff_dn12 = assign15070_e22443_d_n12;
        locals.var_pseff_dn13 = assign15070_e22443_d_n13;
        locals.var_pseff_dn14 = assign15070_e22443_d_n14;
        locals.var_pseff_rv = 0.0;

        let (assign15080_e22448, assign15080_e22448_d_n0, assign15080_e22448_d_n2, assign15080_e22448_d_n3, assign15080_e22448_d_n4, assign15080_e22448_d_n5, assign15080_e22448_d_n6, assign15080_e22448_d_n7, assign15080_e22448_d_n8, assign15080_e22448_d_n9, assign15080_e22448_d_n10, assign15080_e22448_d_n11, assign15080_e22448_d_n12, assign15080_e22448_d_n13, assign15080_e22448_d_n14,) = {
    if (locals.var_guard480 == 0.0) {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn0, locals.var_pseff_dn2, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11, locals.var_pseff_dn12, locals.var_pseff_dn13, locals.var_pseff_dn14,)
    }
};
        locals.var_pseff = assign15080_e22448;
        locals.var_pseff_dn0 = assign15080_e22448_d_n0;
        locals.var_pseff_dn2 = assign15080_e22448_d_n2;
        locals.var_pseff_dn3 = assign15080_e22448_d_n3;
        locals.var_pseff_dn4 = assign15080_e22448_d_n4;
        locals.var_pseff_dn5 = assign15080_e22448_d_n5;
        locals.var_pseff_dn6 = assign15080_e22448_d_n6;
        locals.var_pseff_dn7 = assign15080_e22448_d_n7;
        locals.var_pseff_dn8 = assign15080_e22448_d_n8;
        locals.var_pseff_dn9 = assign15080_e22448_d_n9;
        locals.var_pseff_dn10 = assign15080_e22448_d_n10;
        locals.var_pseff_dn11 = assign15080_e22448_d_n11;
        locals.var_pseff_dn12 = assign15080_e22448_d_n12;
        locals.var_pseff_dn13 = assign15080_e22448_d_n13;
        locals.var_pseff_dn14 = assign15080_e22448_d_n14;
        locals.var_pseff_rv = 0.0;

        let assign15090_e22451: f64 = if locals.var_pseff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard482 = assign15090_e22451;
        locals.var_guard482_rv = 0.0;

        let (assign15100_e22458, assign15100_e22458_d_n0, assign15100_e22458_d_n2, assign15100_e22458_d_n3, assign15100_e22458_d_n4, assign15100_e22458_d_n5, assign15100_e22458_d_n6, assign15100_e22458_d_n7, assign15100_e22458_d_n8, assign15100_e22458_d_n9, assign15100_e22458_d_n10, assign15100_e22458_d_n11, assign15100_e22458_d_n12, assign15100_e22458_d_n13, assign15100_e22458_d_n14,) = {
    if ((locals.var_guard480 == 0.0) && (locals.var_guard482 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn0, locals.var_pseff_dn2, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11, locals.var_pseff_dn12, locals.var_pseff_dn13, locals.var_pseff_dn14,)
    }
};
        locals.var_pseff = assign15100_e22458;
        locals.var_pseff_dn0 = assign15100_e22458_d_n0;
        locals.var_pseff_dn2 = assign15100_e22458_d_n2;
        locals.var_pseff_dn3 = assign15100_e22458_d_n3;
        locals.var_pseff_dn4 = assign15100_e22458_d_n4;
        locals.var_pseff_dn5 = assign15100_e22458_d_n5;
        locals.var_pseff_dn6 = assign15100_e22458_d_n6;
        locals.var_pseff_dn7 = assign15100_e22458_d_n7;
        locals.var_pseff_dn8 = assign15100_e22458_d_n8;
        locals.var_pseff_dn9 = assign15100_e22458_d_n9;
        locals.var_pseff_dn10 = assign15100_e22458_d_n10;
        locals.var_pseff_dn11 = assign15100_e22458_d_n11;
        locals.var_pseff_dn12 = assign15100_e22458_d_n12;
        locals.var_pseff_dn13 = assign15100_e22458_d_n13;
        locals.var_pseff_dn14 = assign15100_e22458_d_n14;
        locals.var_pseff_rv = 0.0;

        let assign15110_e22460: f64 = if param_given[27] { 1.0 } else { 0.0 };
        locals.var_guard483 = assign15110_e22460;
        locals.var_guard483_rv = 0.0;

        let assign15120_e22463: f64 = if p.p137 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard484 = assign15120_e22463;
        locals.var_guard484_rv = 0.0;

        let (assign15130_e22471, assign15130_e22471_d_n0, assign15130_e22471_d_n2, assign15130_e22471_d_n3, assign15130_e22471_d_n4, assign15130_e22471_d_n5, assign15130_e22471_d_n6, assign15130_e22471_d_n7, assign15130_e22471_d_n8, assign15130_e22471_d_n9, assign15130_e22471_d_n10, assign15130_e22471_d_n11, assign15130_e22471_d_n12, assign15130_e22471_d_n13, assign15130_e22471_d_n14,) = {
    if ((locals.var_guard483 != 0.0) && (locals.var_guard484 != 0.0)) {
        let assign15130_e22469: f64 = (p.p27 * p.p53);
        (assign15130_e22469, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn0, locals.var_pdeff_dn2, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11, locals.var_pdeff_dn12, locals.var_pdeff_dn13, locals.var_pdeff_dn14,)
    }
};
        locals.var_pdeff = assign15130_e22471;
        locals.var_pdeff_dn0 = assign15130_e22471_d_n0;
        locals.var_pdeff_dn2 = assign15130_e22471_d_n2;
        locals.var_pdeff_dn3 = assign15130_e22471_d_n3;
        locals.var_pdeff_dn4 = assign15130_e22471_d_n4;
        locals.var_pdeff_dn5 = assign15130_e22471_d_n5;
        locals.var_pdeff_dn6 = assign15130_e22471_d_n6;
        locals.var_pdeff_dn7 = assign15130_e22471_d_n7;
        locals.var_pdeff_dn8 = assign15130_e22471_d_n8;
        locals.var_pdeff_dn9 = assign15130_e22471_d_n9;
        locals.var_pdeff_dn10 = assign15130_e22471_d_n10;
        locals.var_pdeff_dn11 = assign15130_e22471_d_n11;
        locals.var_pdeff_dn12 = assign15130_e22471_d_n12;
        locals.var_pdeff_dn13 = assign15130_e22471_d_n13;
        locals.var_pdeff_dn14 = assign15130_e22471_d_n14;
        locals.var_pdeff_rv = 0.0;

        let (assign15140_e22486, assign15140_e22486_d_n0, assign15140_e22486_d_n2, assign15140_e22486_d_n3, assign15140_e22486_d_n4, assign15140_e22486_d_n5, assign15140_e22486_d_n6, assign15140_e22486_d_n7, assign15140_e22486_d_n8, assign15140_e22486_d_n9, assign15140_e22486_d_n10, assign15140_e22486_d_n11, assign15140_e22486_d_n12, assign15140_e22486_d_n13, assign15140_e22486_d_n14,) = {
    if ((locals.var_guard483 != 0.0) && (locals.var_guard484 == 0.0)) {
        let assign15140_e22478: f64 = (p.p27 * p.p53);
        let assign15140_e22481: f64 = (locals.var_weffcj * p.p2);
        let assign15140_e22482: f64 = (assign15140_e22478 - assign15140_e22481);
        let assign15140_e22484: f64 = (assign15140_e22482).max(0.0);
        (assign15140_e22484, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn0, locals.var_pdeff_dn2, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11, locals.var_pdeff_dn12, locals.var_pdeff_dn13, locals.var_pdeff_dn14,)
    }
};
        locals.var_pdeff = assign15140_e22486;
        locals.var_pdeff_dn0 = assign15140_e22486_d_n0;
        locals.var_pdeff_dn2 = assign15140_e22486_d_n2;
        locals.var_pdeff_dn3 = assign15140_e22486_d_n3;
        locals.var_pdeff_dn4 = assign15140_e22486_d_n4;
        locals.var_pdeff_dn5 = assign15140_e22486_d_n5;
        locals.var_pdeff_dn6 = assign15140_e22486_d_n6;
        locals.var_pdeff_dn7 = assign15140_e22486_d_n7;
        locals.var_pdeff_dn8 = assign15140_e22486_d_n8;
        locals.var_pdeff_dn9 = assign15140_e22486_d_n9;
        locals.var_pdeff_dn10 = assign15140_e22486_d_n10;
        locals.var_pdeff_dn11 = assign15140_e22486_d_n11;
        locals.var_pdeff_dn12 = assign15140_e22486_d_n12;
        locals.var_pdeff_dn13 = assign15140_e22486_d_n13;
        locals.var_pdeff_dn14 = assign15140_e22486_d_n14;
        locals.var_pdeff_rv = 0.0;

        let (assign15150_e22491, assign15150_e22491_d_n0, assign15150_e22491_d_n2, assign15150_e22491_d_n3, assign15150_e22491_d_n4, assign15150_e22491_d_n5, assign15150_e22491_d_n6, assign15150_e22491_d_n7, assign15150_e22491_d_n8, assign15150_e22491_d_n9, assign15150_e22491_d_n10, assign15150_e22491_d_n11, assign15150_e22491_d_n12, assign15150_e22491_d_n13, assign15150_e22491_d_n14,) = {
    if (locals.var_guard483 == 0.0) {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn0, locals.var_pdeff_dn2, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11, locals.var_pdeff_dn12, locals.var_pdeff_dn13, locals.var_pdeff_dn14,)
    }
};
        locals.var_pdeff = assign15150_e22491;
        locals.var_pdeff_dn0 = assign15150_e22491_d_n0;
        locals.var_pdeff_dn2 = assign15150_e22491_d_n2;
        locals.var_pdeff_dn3 = assign15150_e22491_d_n3;
        locals.var_pdeff_dn4 = assign15150_e22491_d_n4;
        locals.var_pdeff_dn5 = assign15150_e22491_d_n5;
        locals.var_pdeff_dn6 = assign15150_e22491_d_n6;
        locals.var_pdeff_dn7 = assign15150_e22491_d_n7;
        locals.var_pdeff_dn8 = assign15150_e22491_d_n8;
        locals.var_pdeff_dn9 = assign15150_e22491_d_n9;
        locals.var_pdeff_dn10 = assign15150_e22491_d_n10;
        locals.var_pdeff_dn11 = assign15150_e22491_d_n11;
        locals.var_pdeff_dn12 = assign15150_e22491_d_n12;
        locals.var_pdeff_dn13 = assign15150_e22491_d_n13;
        locals.var_pdeff_dn14 = assign15150_e22491_d_n14;
        locals.var_pdeff_rv = 0.0;

        let assign15160_e22494: f64 = if locals.var_pdeff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard485 = assign15160_e22494;
        locals.var_guard485_rv = 0.0;

        let (assign15170_e22501, assign15170_e22501_d_n0, assign15170_e22501_d_n2, assign15170_e22501_d_n3, assign15170_e22501_d_n4, assign15170_e22501_d_n5, assign15170_e22501_d_n6, assign15170_e22501_d_n7, assign15170_e22501_d_n8, assign15170_e22501_d_n9, assign15170_e22501_d_n10, assign15170_e22501_d_n11, assign15170_e22501_d_n12, assign15170_e22501_d_n13, assign15170_e22501_d_n14,) = {
    if ((locals.var_guard483 == 0.0) && (locals.var_guard485 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn0, locals.var_pdeff_dn2, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11, locals.var_pdeff_dn12, locals.var_pdeff_dn13, locals.var_pdeff_dn14,)
    }
};
        locals.var_pdeff = assign15170_e22501;
        locals.var_pdeff_dn0 = assign15170_e22501_d_n0;
        locals.var_pdeff_dn2 = assign15170_e22501_d_n2;
        locals.var_pdeff_dn3 = assign15170_e22501_d_n3;
        locals.var_pdeff_dn4 = assign15170_e22501_d_n4;
        locals.var_pdeff_dn5 = assign15170_e22501_d_n5;
        locals.var_pdeff_dn6 = assign15170_e22501_d_n6;
        locals.var_pdeff_dn7 = assign15170_e22501_d_n7;
        locals.var_pdeff_dn8 = assign15170_e22501_d_n8;
        locals.var_pdeff_dn9 = assign15170_e22501_d_n9;
        locals.var_pdeff_dn10 = assign15170_e22501_d_n10;
        locals.var_pdeff_dn11 = assign15170_e22501_d_n11;
        locals.var_pdeff_dn12 = assign15170_e22501_d_n12;
        locals.var_pdeff_dn13 = assign15170_e22501_d_n13;
        locals.var_pdeff_dn14 = assign15170_e22501_d_n14;
        locals.var_pdeff_rv = 0.0;

        let assign15180_e22504: f64 = (locals.var_aseff * locals.var_jss_t);
        let assign15180_e22507: f64 = (locals.var_pseff * locals.var_jsws_t);
        let assign15180_e22508: f64 = (assign15180_e22504 + assign15180_e22507);
        let assign15180_e22511: f64 = (locals.var_weffcj * p.p2);
        let assign15180_e22513: f64 = (assign15180_e22511 * locals.var_jswgs_t);
        let assign15180_e22514: f64 = (assign15180_e22508 + assign15180_e22513);
        locals.var_isbs = assign15180_e22514;
        locals.var_isbs_dn0 = ((((locals.var_aseff_dn0 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn0)) + ((locals.var_pseff_dn0 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn0))) + (assign15180_e22511 * locals.var_jswgs_t_dn0));
        locals.var_isbs_dn2 = ((((locals.var_aseff_dn2 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn2)) + ((locals.var_pseff_dn2 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn2))) + (assign15180_e22511 * locals.var_jswgs_t_dn2));
        locals.var_isbs_dn3 = ((((locals.var_aseff_dn3 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn3)) + ((locals.var_pseff_dn3 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn3))) + (assign15180_e22511 * locals.var_jswgs_t_dn3));
        locals.var_isbs_dn4 = ((((locals.var_aseff_dn4 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn4)) + ((locals.var_pseff_dn4 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn4))) + (assign15180_e22511 * locals.var_jswgs_t_dn4));
        locals.var_isbs_dn5 = ((((locals.var_aseff_dn5 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn5)) + ((locals.var_pseff_dn5 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn5))) + (assign15180_e22511 * locals.var_jswgs_t_dn5));
        locals.var_isbs_dn6 = ((((locals.var_aseff_dn6 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn6)) + ((locals.var_pseff_dn6 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn6))) + (assign15180_e22511 * locals.var_jswgs_t_dn6));
        locals.var_isbs_dn7 = ((((locals.var_aseff_dn7 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn7)) + ((locals.var_pseff_dn7 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn7))) + (assign15180_e22511 * locals.var_jswgs_t_dn7));
        locals.var_isbs_dn8 = ((((locals.var_aseff_dn8 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn8)) + ((locals.var_pseff_dn8 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn8))) + (assign15180_e22511 * locals.var_jswgs_t_dn8));
        locals.var_isbs_dn9 = ((((locals.var_aseff_dn9 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn9)) + ((locals.var_pseff_dn9 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn9))) + (assign15180_e22511 * locals.var_jswgs_t_dn9));
        locals.var_isbs_dn10 = ((((locals.var_aseff_dn10 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn10)) + ((locals.var_pseff_dn10 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn10))) + (assign15180_e22511 * locals.var_jswgs_t_dn10));
        locals.var_isbs_dn11 = ((((locals.var_aseff_dn11 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn11)) + ((locals.var_pseff_dn11 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn11))) + (assign15180_e22511 * locals.var_jswgs_t_dn11));
        locals.var_isbs_dn12 = ((((locals.var_aseff_dn12 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn12)) + ((locals.var_pseff_dn12 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn12))) + (assign15180_e22511 * locals.var_jswgs_t_dn12));
        locals.var_isbs_dn13 = ((((locals.var_aseff_dn13 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn13)) + ((locals.var_pseff_dn13 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn13))) + (assign15180_e22511 * locals.var_jswgs_t_dn13));
        locals.var_isbs_dn14 = ((((locals.var_aseff_dn14 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn14)) + ((locals.var_pseff_dn14 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn14))) + (assign15180_e22511 * locals.var_jswgs_t_dn14));
        locals.var_isbs_rv = 0.0;

        let assign15190_e22517: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard486 = assign15190_e22517;
        locals.var_guard486_rv = 0.0;

        let (assign15200_e22523, assign15200_e22523_d_n4,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15200_e22521: f64 = (locals.var_vtm * p.p725);
        (assign15200_e22521, (locals.var_vtm_dn4 * p.p725),)
    } else {
        (locals.var_nvtms, locals.var_nvtms_dn4,)
    }
};
        locals.var_nvtms = assign15200_e22523;
        locals.var_nvtms_dn4 = assign15200_e22523_d_n4;
        locals.var_nvtms_rv = 0.0;

        let (assign15210_e22533, assign15210_e22533_d_n4,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15210_e22526: f64 = (-p.p731);
        let assign15210_e22528: f64 = (assign15210_e22526 / locals.var_nvtms);
        let assign15210_e22529: f64 = { let limited_exp_arg = assign15210_e22528; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15210_e22531: f64 = (assign15210_e22529 * p.p733);
        (assign15210_e22531, (({ let limited_exp_arg = assign15210_e22528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign15210_e22526 * locals.var_nvtms_dn4) / (locals.var_nvtms * locals.var_nvtms)))) * p.p733),)
    } else {
        (locals.var_xexpbvs, locals.var_xexpbvs_dn4,)
    }
};
        locals.var_xexpbvs = assign15210_e22533;
        locals.var_xexpbvs_dn4 = assign15210_e22533_d_n4;
        locals.var_xexpbvs_rv = 0.0;

        let (assign15220_e22541, assign15220_e22541_d_n0, assign15220_e22541_d_n2, assign15220_e22541_d_n3, assign15220_e22541_d_n4, assign15220_e22541_d_n5, assign15220_e22541_d_n6, assign15220_e22541_d_n7, assign15220_e22541_d_n8, assign15220_e22541_d_n9, assign15220_e22541_d_n10, assign15220_e22541_d_n11, assign15220_e22541_d_n12, assign15220_e22541_d_n13, assign15220_e22541_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15220_e22537: f64 = (p.p727 / locals.var_isbs);
        let assign15220_e22539: f64 = (assign15220_e22537).max(10.0);
        (assign15220_e22539, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15220_e22541;
        locals.var_t2_dn0 = assign15220_e22541_d_n0;
        locals.var_t2_dn2 = assign15220_e22541_d_n2;
        locals.var_t2_dn3 = assign15220_e22541_d_n3;
        locals.var_t2_dn4 = assign15220_e22541_d_n4;
        locals.var_t2_dn5 = assign15220_e22541_d_n5;
        locals.var_t2_dn6 = assign15220_e22541_d_n6;
        locals.var_t2_dn7 = assign15220_e22541_d_n7;
        locals.var_t2_dn8 = assign15220_e22541_d_n8;
        locals.var_t2_dn9 = assign15220_e22541_d_n9;
        locals.var_t2_dn10 = assign15220_e22541_d_n10;
        locals.var_t2_dn11 = assign15220_e22541_d_n11;
        locals.var_t2_dn12 = assign15220_e22541_d_n12;
        locals.var_t2_dn13 = assign15220_e22541_d_n13;
        locals.var_t2_dn14 = assign15220_e22541_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15230_e22549, assign15230_e22549_d_n0, assign15230_e22549_d_n2, assign15230_e22549_d_n3, assign15230_e22549_d_n4, assign15230_e22549_d_n5, assign15230_e22549_d_n6, assign15230_e22549_d_n7, assign15230_e22549_d_n8, assign15230_e22549_d_n9, assign15230_e22549_d_n10, assign15230_e22549_d_n11, assign15230_e22549_d_n12, assign15230_e22549_d_n13, assign15230_e22549_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15230_e22545: f64 = (1.0 + locals.var_t2);
        let assign15230_e22547: f64 = (assign15230_e22545 - locals.var_xexpbvs);
        (assign15230_e22547, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, (locals.var_t2_dn4 - locals.var_xexpbvs_dn4), locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    } else {
        (locals.var_tb, locals.var_tb_dn0, locals.var_tb_dn2, locals.var_tb_dn3, locals.var_tb_dn4, locals.var_tb_dn5, locals.var_tb_dn6, locals.var_tb_dn7, locals.var_tb_dn8, locals.var_tb_dn9, locals.var_tb_dn10, locals.var_tb_dn11, locals.var_tb_dn12, locals.var_tb_dn13, locals.var_tb_dn14,)
    }
};
        locals.var_tb = assign15230_e22549;
        locals.var_tb_dn0 = assign15230_e22549_d_n0;
        locals.var_tb_dn2 = assign15230_e22549_d_n2;
        locals.var_tb_dn3 = assign15230_e22549_d_n3;
        locals.var_tb_dn4 = assign15230_e22549_d_n4;
        locals.var_tb_dn5 = assign15230_e22549_d_n5;
        locals.var_tb_dn6 = assign15230_e22549_d_n6;
        locals.var_tb_dn7 = assign15230_e22549_d_n7;
        locals.var_tb_dn8 = assign15230_e22549_d_n8;
        locals.var_tb_dn9 = assign15230_e22549_d_n9;
        locals.var_tb_dn10 = assign15230_e22549_d_n10;
        locals.var_tb_dn11 = assign15230_e22549_d_n11;
        locals.var_tb_dn12 = assign15230_e22549_d_n12;
        locals.var_tb_dn13 = assign15230_e22549_d_n13;
        locals.var_tb_dn14 = assign15230_e22549_d_n14;
        locals.var_tb_rv = 0.0;

        let (assign15240_e22569, assign15240_e22569_d_n0, assign15240_e22569_d_n2, assign15240_e22569_d_n3, assign15240_e22569_d_n4, assign15240_e22569_d_n5, assign15240_e22569_d_n6, assign15240_e22569_d_n7, assign15240_e22569_d_n8, assign15240_e22569_d_n9, assign15240_e22569_d_n10, assign15240_e22569_d_n11, assign15240_e22569_d_n12, assign15240_e22569_d_n13, assign15240_e22569_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15240_e22556: f64 = (locals.var_tb * locals.var_tb);
        let assign15240_e22559: f64 = (4.0 * locals.var_xexpbvs);
        let assign15240_e22560: f64 = (assign15240_e22556 + assign15240_e22559);
        let assign15240_e22561: f64 = (assign15240_e22560).sqrt();
        let assign15240_e22562: f64 = (locals.var_tb + assign15240_e22561);
        let assign15240_e22563: f64 = (0.5 * assign15240_e22562);
        let assign15240_e22565: f64 = (assign15240_e22563).max(1e-38);
        let assign15240_e22566: f64 = (assign15240_e22565).ln();
        let assign15240_e22567: f64 = (locals.var_nvtms * assign15240_e22566);
        (assign15240_e22567, (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn0 + (((locals.var_tb_dn0 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn0)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn2 + (((locals.var_tb_dn2 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn2)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn3 + (((locals.var_tb_dn3 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn3)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), ((locals.var_nvtms_dn4 * assign15240_e22566) + (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn4 + ((((locals.var_tb_dn4 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn4)) + (4.0 * locals.var_xexpbvs_dn4)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565))), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn5 + (((locals.var_tb_dn5 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn5)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn6 + (((locals.var_tb_dn6 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn6)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn7 + (((locals.var_tb_dn7 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn7)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn8 + (((locals.var_tb_dn8 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn8)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn9 + (((locals.var_tb_dn9 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn9)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn10 + (((locals.var_tb_dn10 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn10)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn11 + (((locals.var_tb_dn11 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn11)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn12 + (((locals.var_tb_dn12 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn12)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn13 + (((locals.var_tb_dn13 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn13)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn14 + (((locals.var_tb_dn14 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn14)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)),)
    } else {
        (locals.var_vjsmfwd, locals.var_vjsmfwd_dn0, locals.var_vjsmfwd_dn2, locals.var_vjsmfwd_dn3, locals.var_vjsmfwd_dn4, locals.var_vjsmfwd_dn5, locals.var_vjsmfwd_dn6, locals.var_vjsmfwd_dn7, locals.var_vjsmfwd_dn8, locals.var_vjsmfwd_dn9, locals.var_vjsmfwd_dn10, locals.var_vjsmfwd_dn11, locals.var_vjsmfwd_dn12, locals.var_vjsmfwd_dn13, locals.var_vjsmfwd_dn14,)
    }
};
        locals.var_vjsmfwd = assign15240_e22569;
        locals.var_vjsmfwd_dn0 = assign15240_e22569_d_n0;
        locals.var_vjsmfwd_dn2 = assign15240_e22569_d_n2;
        locals.var_vjsmfwd_dn3 = assign15240_e22569_d_n3;
        locals.var_vjsmfwd_dn4 = assign15240_e22569_d_n4;
        locals.var_vjsmfwd_dn5 = assign15240_e22569_d_n5;
        locals.var_vjsmfwd_dn6 = assign15240_e22569_d_n6;
        locals.var_vjsmfwd_dn7 = assign15240_e22569_d_n7;
        locals.var_vjsmfwd_dn8 = assign15240_e22569_d_n8;
        locals.var_vjsmfwd_dn9 = assign15240_e22569_d_n9;
        locals.var_vjsmfwd_dn10 = assign15240_e22569_d_n10;
        locals.var_vjsmfwd_dn11 = assign15240_e22569_d_n11;
        locals.var_vjsmfwd_dn12 = assign15240_e22569_d_n12;
        locals.var_vjsmfwd_dn13 = assign15240_e22569_d_n13;
        locals.var_vjsmfwd_dn14 = assign15240_e22569_d_n14;
        locals.var_vjsmfwd_rv = 0.0;

        let (assign15250_e22576, assign15250_e22576_d_n0, assign15250_e22576_d_n2, assign15250_e22576_d_n3, assign15250_e22576_d_n4, assign15250_e22576_d_n5, assign15250_e22576_d_n6, assign15250_e22576_d_n7, assign15250_e22576_d_n8, assign15250_e22576_d_n9, assign15250_e22576_d_n10, assign15250_e22576_d_n11, assign15250_e22576_d_n12, assign15250_e22576_d_n13, assign15250_e22576_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15250_e22573: f64 = (locals.var_vjsmfwd / locals.var_nvtms);
        let assign15250_e22574: f64 = { let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign15250_e22574, ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn0 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn2 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn3 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vjsmfwd_dn4 * locals.var_nvtms) - (locals.var_vjsmfwd * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms))), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn5 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn6 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn7 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn8 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn9 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn10 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn11 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn12 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn13 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn14 / locals.var_nvtms)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15250_e22576;
        locals.var_t0_dn0 = assign15250_e22576_d_n0;
        locals.var_t0_dn2 = assign15250_e22576_d_n2;
        locals.var_t0_dn3 = assign15250_e22576_d_n3;
        locals.var_t0_dn4 = assign15250_e22576_d_n4;
        locals.var_t0_dn5 = assign15250_e22576_d_n5;
        locals.var_t0_dn6 = assign15250_e22576_d_n6;
        locals.var_t0_dn7 = assign15250_e22576_d_n7;
        locals.var_t0_dn8 = assign15250_e22576_d_n8;
        locals.var_t0_dn9 = assign15250_e22576_d_n9;
        locals.var_t0_dn10 = assign15250_e22576_d_n10;
        locals.var_t0_dn11 = assign15250_e22576_d_n11;
        locals.var_t0_dn12 = assign15250_e22576_d_n12;
        locals.var_t0_dn13 = assign15250_e22576_d_n13;
        locals.var_t0_dn14 = assign15250_e22576_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15260_e22590, assign15260_e22590_d_n0, assign15260_e22590_d_n2, assign15260_e22590_d_n3, assign15260_e22590_d_n4, assign15260_e22590_d_n5, assign15260_e22590_d_n6, assign15260_e22590_d_n7, assign15260_e22590_d_n8, assign15260_e22590_d_n9, assign15260_e22590_d_n10, assign15260_e22590_d_n11, assign15260_e22590_d_n12, assign15260_e22590_d_n13, assign15260_e22590_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15260_e22582: f64 = (locals.var_xexpbvs / locals.var_t0);
        let assign15260_e22583: f64 = (locals.var_t0 - assign15260_e22582);
        let assign15260_e22585: f64 = (assign15260_e22583 + locals.var_xexpbvs);
        let assign15260_e22587: f64 = (assign15260_e22585 - 1.0);
        let assign15260_e22588: f64 = (locals.var_isbs * assign15260_e22587);
        (assign15260_e22588, ((locals.var_isbs_dn0 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn0 - (-((locals.var_xexpbvs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn2 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn2 - (-((locals.var_xexpbvs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn3 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn3 - (-((locals.var_xexpbvs * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn4 * assign15260_e22587) + (locals.var_isbs * ((locals.var_t0_dn4 - (((locals.var_xexpbvs_dn4 * locals.var_t0) - (locals.var_xexpbvs * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))) + locals.var_xexpbvs_dn4))), ((locals.var_isbs_dn5 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn5 - (-((locals.var_xexpbvs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn6 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn6 - (-((locals.var_xexpbvs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn7 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn7 - (-((locals.var_xexpbvs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn8 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn8 - (-((locals.var_xexpbvs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn9 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn9 - (-((locals.var_xexpbvs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn10 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn10 - (-((locals.var_xexpbvs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn11 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn11 - (-((locals.var_xexpbvs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn12 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn12 - (-((locals.var_xexpbvs * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn13 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn13 - (-((locals.var_xexpbvs * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn14 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn14 - (-((locals.var_xexpbvs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))),)
    } else {
        (locals.var_ivjsmfwd, locals.var_ivjsmfwd_dn0, locals.var_ivjsmfwd_dn2, locals.var_ivjsmfwd_dn3, locals.var_ivjsmfwd_dn4, locals.var_ivjsmfwd_dn5, locals.var_ivjsmfwd_dn6, locals.var_ivjsmfwd_dn7, locals.var_ivjsmfwd_dn8, locals.var_ivjsmfwd_dn9, locals.var_ivjsmfwd_dn10, locals.var_ivjsmfwd_dn11, locals.var_ivjsmfwd_dn12, locals.var_ivjsmfwd_dn13, locals.var_ivjsmfwd_dn14,)
    }
};
        locals.var_ivjsmfwd = assign15260_e22590;
        locals.var_ivjsmfwd_dn0 = assign15260_e22590_d_n0;
        locals.var_ivjsmfwd_dn2 = assign15260_e22590_d_n2;
        locals.var_ivjsmfwd_dn3 = assign15260_e22590_d_n3;
        locals.var_ivjsmfwd_dn4 = assign15260_e22590_d_n4;
        locals.var_ivjsmfwd_dn5 = assign15260_e22590_d_n5;
        locals.var_ivjsmfwd_dn6 = assign15260_e22590_d_n6;
        locals.var_ivjsmfwd_dn7 = assign15260_e22590_d_n7;
        locals.var_ivjsmfwd_dn8 = assign15260_e22590_d_n8;
        locals.var_ivjsmfwd_dn9 = assign15260_e22590_d_n9;
        locals.var_ivjsmfwd_dn10 = assign15260_e22590_d_n10;
        locals.var_ivjsmfwd_dn11 = assign15260_e22590_d_n11;
        locals.var_ivjsmfwd_dn12 = assign15260_e22590_d_n12;
        locals.var_ivjsmfwd_dn13 = assign15260_e22590_d_n13;
        locals.var_ivjsmfwd_dn14 = assign15260_e22590_d_n14;
        locals.var_ivjsmfwd_rv = 0.0;

        let (assign15270_e22602, assign15270_e22602_d_n0, assign15270_e22602_d_n2, assign15270_e22602_d_n3, assign15270_e22602_d_n4, assign15270_e22602_d_n5, assign15270_e22602_d_n6, assign15270_e22602_d_n7, assign15270_e22602_d_n8, assign15270_e22602_d_n9, assign15270_e22602_d_n10, assign15270_e22602_d_n11, assign15270_e22602_d_n12, assign15270_e22602_d_n13, assign15270_e22602_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15270_e22596: f64 = (locals.var_xexpbvs / locals.var_t0);
        let assign15270_e22597: f64 = (locals.var_t0 + assign15270_e22596);
        let assign15270_e22598: f64 = (locals.var_isbs * assign15270_e22597);
        let assign15270_e22600: f64 = (assign15270_e22598 / locals.var_nvtms);
        (assign15270_e22600, (((locals.var_isbs_dn0 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn0 + (-((locals.var_xexpbvs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn2 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn2 + (-((locals.var_xexpbvs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn3 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn3 + (-((locals.var_xexpbvs * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((((locals.var_isbs_dn4 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn4 + (((locals.var_xexpbvs_dn4 * locals.var_t0) - (locals.var_xexpbvs * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))))) * locals.var_nvtms) - (assign15270_e22598 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)), (((locals.var_isbs_dn5 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn5 + (-((locals.var_xexpbvs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn6 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn6 + (-((locals.var_xexpbvs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn7 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn7 + (-((locals.var_xexpbvs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn8 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn8 + (-((locals.var_xexpbvs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn9 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn9 + (-((locals.var_xexpbvs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn10 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn10 + (-((locals.var_xexpbvs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn11 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn11 + (-((locals.var_xexpbvs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn12 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn12 + (-((locals.var_xexpbvs * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn13 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn13 + (-((locals.var_xexpbvs * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn14 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn14 + (-((locals.var_xexpbvs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms),)
    } else {
        (locals.var_sslpfwd, locals.var_sslpfwd_dn0, locals.var_sslpfwd_dn2, locals.var_sslpfwd_dn3, locals.var_sslpfwd_dn4, locals.var_sslpfwd_dn5, locals.var_sslpfwd_dn6, locals.var_sslpfwd_dn7, locals.var_sslpfwd_dn8, locals.var_sslpfwd_dn9, locals.var_sslpfwd_dn10, locals.var_sslpfwd_dn11, locals.var_sslpfwd_dn12, locals.var_sslpfwd_dn13, locals.var_sslpfwd_dn14,)
    }
};
        locals.var_sslpfwd = assign15270_e22602;
        locals.var_sslpfwd_dn0 = assign15270_e22602_d_n0;
        locals.var_sslpfwd_dn2 = assign15270_e22602_d_n2;
        locals.var_sslpfwd_dn3 = assign15270_e22602_d_n3;
        locals.var_sslpfwd_dn4 = assign15270_e22602_d_n4;
        locals.var_sslpfwd_dn5 = assign15270_e22602_d_n5;
        locals.var_sslpfwd_dn6 = assign15270_e22602_d_n6;
        locals.var_sslpfwd_dn7 = assign15270_e22602_d_n7;
        locals.var_sslpfwd_dn8 = assign15270_e22602_d_n8;
        locals.var_sslpfwd_dn9 = assign15270_e22602_d_n9;
        locals.var_sslpfwd_dn10 = assign15270_e22602_d_n10;
        locals.var_sslpfwd_dn11 = assign15270_e22602_d_n11;
        locals.var_sslpfwd_dn12 = assign15270_e22602_d_n12;
        locals.var_sslpfwd_dn13 = assign15270_e22602_d_n13;
        locals.var_sslpfwd_dn14 = assign15270_e22602_d_n14;
        locals.var_sslpfwd_rv = 0.0;

        let (assign15280_e22667, assign15280_e22667_d_n0, assign15280_e22667_d_n2, assign15280_e22667_d_n3, assign15280_e22667_d_n4, assign15280_e22667_d_n5, assign15280_e22667_d_n6, assign15280_e22667_d_n7, assign15280_e22667_d_n8, assign15280_e22667_d_n9, assign15280_e22667_d_n10, assign15280_e22667_d_n11, assign15280_e22667_d_n12, assign15280_e22667_d_n13, assign15280_e22667_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15280_e22606: f64 = (p.p729 / locals.var_isbs);
        let assign15280_e22608: f64 = (assign15280_e22606 - 10.0);
        let assign15280_e22610: f64 = (-10000.0);
        let assign15280_e22612: f64 = (assign15280_e22610 * 0.001);
        let (assign15280_e22663, assign15280_e22663_d_n0, assign15280_e22663_d_n2, assign15280_e22663_d_n3, assign15280_e22663_d_n4, assign15280_e22663_d_n5, assign15280_e22663_d_n6, assign15280_e22663_d_n7, assign15280_e22663_d_n8, assign15280_e22663_d_n9, assign15280_e22663_d_n10, assign15280_e22663_d_n11, assign15280_e22663_d_n12, assign15280_e22663_d_n13, assign15280_e22663_d_n14,) = {
            if (!(assign15280_e22608 < assign15280_e22612)) {
                let assign15280_e22618: f64 = (p.p729 / locals.var_isbs);
                let assign15280_e22620: f64 = (assign15280_e22618 - 10.0);
                let assign15280_e22623: f64 = (p.p729 / locals.var_isbs);
                let assign15280_e22625: f64 = (assign15280_e22623 - 10.0);
                let assign15280_e22628: f64 = (p.p729 / locals.var_isbs);
                let assign15280_e22630: f64 = (assign15280_e22628 - 10.0);
                let assign15280_e22631: f64 = (assign15280_e22625 * assign15280_e22630);
                let assign15280_e22634: f64 = (4.0 * 0.001);
                let assign15280_e22636: f64 = (assign15280_e22634 * 0.001);
                let assign15280_e22637: f64 = (assign15280_e22631 + assign15280_e22636);
                let assign15280_e22638: f64 = (assign15280_e22637).sqrt();
                let assign15280_e22639: f64 = (assign15280_e22620 + assign15280_e22638);
                let assign15280_e22640: f64 = (0.5 * assign15280_e22639);
                (assign15280_e22640, (0.5 * ((-((p.p729 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))),)
            } else {
                let assign15280_e22643: f64 = (p.p729 / locals.var_isbs);
                let assign15280_e22645: f64 = (assign15280_e22643 - 10.0);
                let assign15280_e22647: f64 = (-10000.0);
                let assign15280_e22649: f64 = (assign15280_e22647 * 0.001);
                let (assign15280_e22662, assign15280_e22662_d_n0, assign15280_e22662_d_n2, assign15280_e22662_d_n3, assign15280_e22662_d_n4, assign15280_e22662_d_n5, assign15280_e22662_d_n6, assign15280_e22662_d_n7, assign15280_e22662_d_n8, assign15280_e22662_d_n9, assign15280_e22662_d_n10, assign15280_e22662_d_n11, assign15280_e22662_d_n12, assign15280_e22662_d_n13, assign15280_e22662_d_n14,) = {
                    if (assign15280_e22645 < assign15280_e22649) {
                        let assign15280_e22652: f64 = (-0.001);
                        let assign15280_e22654: f64 = (assign15280_e22652 * 0.001);
                        let assign15280_e22657: f64 = (p.p729 / locals.var_isbs);
                        let assign15280_e22659: f64 = (assign15280_e22657 - 10.0);
                        let assign15280_e22660: f64 = (assign15280_e22654 / assign15280_e22659);
                        (assign15280_e22660, (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15280_e22662, assign15280_e22662_d_n0, assign15280_e22662_d_n2, assign15280_e22662_d_n3, assign15280_e22662_d_n4, assign15280_e22662_d_n5, assign15280_e22662_d_n6, assign15280_e22662_d_n7, assign15280_e22662_d_n8, assign15280_e22662_d_n9, assign15280_e22662_d_n10, assign15280_e22662_d_n11, assign15280_e22662_d_n12, assign15280_e22662_d_n13, assign15280_e22662_d_n14,)
            }
        };
        let assign15280_e22665: f64 = (assign15280_e22663 + 10.0);
        (assign15280_e22665, assign15280_e22663_d_n0, assign15280_e22663_d_n2, assign15280_e22663_d_n3, assign15280_e22663_d_n4, assign15280_e22663_d_n5, assign15280_e22663_d_n6, assign15280_e22663_d_n7, assign15280_e22663_d_n8, assign15280_e22663_d_n9, assign15280_e22663_d_n10, assign15280_e22663_d_n11, assign15280_e22663_d_n12, assign15280_e22663_d_n13, assign15280_e22663_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15280_e22667;
        locals.var_t2_dn0 = assign15280_e22667_d_n0;
        locals.var_t2_dn2 = assign15280_e22667_d_n2;
        locals.var_t2_dn3 = assign15280_e22667_d_n3;
        locals.var_t2_dn4 = assign15280_e22667_d_n4;
        locals.var_t2_dn5 = assign15280_e22667_d_n5;
        locals.var_t2_dn6 = assign15280_e22667_d_n6;
        locals.var_t2_dn7 = assign15280_e22667_d_n7;
        locals.var_t2_dn8 = assign15280_e22667_d_n8;
        locals.var_t2_dn9 = assign15280_e22667_d_n9;
        locals.var_t2_dn10 = assign15280_e22667_d_n10;
        locals.var_t2_dn11 = assign15280_e22667_d_n11;
        locals.var_t2_dn12 = assign15280_e22667_d_n12;
        locals.var_t2_dn13 = assign15280_e22667_d_n13;
        locals.var_t2_dn14 = assign15280_e22667_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign15290_e22683, assign15290_e22683_d_n0, assign15290_e22683_d_n2, assign15290_e22683_d_n3, assign15290_e22683_d_n4, assign15290_e22683_d_n5, assign15290_e22683_d_n6, assign15290_e22683_d_n7, assign15290_e22683_d_n8, assign15290_e22683_d_n9, assign15290_e22683_d_n10, assign15290_e22683_d_n11, assign15290_e22683_d_n12, assign15290_e22683_d_n13, assign15290_e22683_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15290_e22670: f64 = (-p.p731);
        let assign15290_e22674: f64 = (locals.var_t2 - 1.0);
        let assign15290_e22676: f64 = (assign15290_e22674 / p.p733);
        let assign15290_e22678: f64 = (assign15290_e22676).max(1e-38);
        let assign15290_e22679: f64 = (assign15290_e22678).ln();
        let assign15290_e22680: f64 = (locals.var_nvtms * assign15290_e22679);
        let assign15290_e22681: f64 = (assign15290_e22670 - assign15290_e22680);
        (assign15290_e22681, (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn0 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn2 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn3 / p.p733) } else { 0.0 } / assign15290_e22678))), (-((locals.var_nvtms_dn4 * assign15290_e22679) + (locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn4 / p.p733) } else { 0.0 } / assign15290_e22678)))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn5 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn6 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn7 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn8 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn9 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn10 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn11 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn12 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn13 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn14 / p.p733) } else { 0.0 } / assign15290_e22678))),)
    } else {
        (locals.var_vjsmrev, locals.var_vjsmrev_dn0, locals.var_vjsmrev_dn2, locals.var_vjsmrev_dn3, locals.var_vjsmrev_dn4, locals.var_vjsmrev_dn5, locals.var_vjsmrev_dn6, locals.var_vjsmrev_dn7, locals.var_vjsmrev_dn8, locals.var_vjsmrev_dn9, locals.var_vjsmrev_dn10, locals.var_vjsmrev_dn11, locals.var_vjsmrev_dn12, locals.var_vjsmrev_dn13, locals.var_vjsmrev_dn14,)
    }
};
        locals.var_vjsmrev = assign15290_e22683;
        locals.var_vjsmrev_dn0 = assign15290_e22683_d_n0;
        locals.var_vjsmrev_dn2 = assign15290_e22683_d_n2;
        locals.var_vjsmrev_dn3 = assign15290_e22683_d_n3;
        locals.var_vjsmrev_dn4 = assign15290_e22683_d_n4;
        locals.var_vjsmrev_dn5 = assign15290_e22683_d_n5;
        locals.var_vjsmrev_dn6 = assign15290_e22683_d_n6;
        locals.var_vjsmrev_dn7 = assign15290_e22683_d_n7;
        locals.var_vjsmrev_dn8 = assign15290_e22683_d_n8;
        locals.var_vjsmrev_dn9 = assign15290_e22683_d_n9;
        locals.var_vjsmrev_dn10 = assign15290_e22683_d_n10;
        locals.var_vjsmrev_dn11 = assign15290_e22683_d_n11;
        locals.var_vjsmrev_dn12 = assign15290_e22683_d_n12;
        locals.var_vjsmrev_dn13 = assign15290_e22683_d_n13;
        locals.var_vjsmrev_dn14 = assign15290_e22683_d_n14;
        locals.var_vjsmrev_rv = 0.0;

        let (assign15300_e22695, assign15300_e22695_d_n0, assign15300_e22695_d_n2, assign15300_e22695_d_n3, assign15300_e22695_d_n4, assign15300_e22695_d_n5, assign15300_e22695_d_n6, assign15300_e22695_d_n7, assign15300_e22695_d_n8, assign15300_e22695_d_n9, assign15300_e22695_d_n10, assign15300_e22695_d_n11, assign15300_e22695_d_n12, assign15300_e22695_d_n13, assign15300_e22695_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15300_e22688: f64 = (p.p731 + locals.var_vjsmrev);
        let assign15300_e22689: f64 = (-assign15300_e22688);
        let assign15300_e22691: f64 = (assign15300_e22689 / locals.var_nvtms);
        let assign15300_e22692: f64 = { let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15300_e22693: f64 = (p.p733 * assign15300_e22692);
        (assign15300_e22693, (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn0) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn2) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn3) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((-locals.var_vjsmrev_dn4) * locals.var_nvtms) - (assign15300_e22689 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn5) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn6) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn7) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn8) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn9) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn10) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn11) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn12) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn13) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn14) / locals.var_nvtms))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15300_e22695;
        locals.var_t1_dn0 = assign15300_e22695_d_n0;
        locals.var_t1_dn2 = assign15300_e22695_d_n2;
        locals.var_t1_dn3 = assign15300_e22695_d_n3;
        locals.var_t1_dn4 = assign15300_e22695_d_n4;
        locals.var_t1_dn5 = assign15300_e22695_d_n5;
        locals.var_t1_dn6 = assign15300_e22695_d_n6;
        locals.var_t1_dn7 = assign15300_e22695_d_n7;
        locals.var_t1_dn8 = assign15300_e22695_d_n8;
        locals.var_t1_dn9 = assign15300_e22695_d_n9;
        locals.var_t1_dn10 = assign15300_e22695_d_n10;
        locals.var_t1_dn11 = assign15300_e22695_d_n11;
        locals.var_t1_dn12 = assign15300_e22695_d_n12;
        locals.var_t1_dn13 = assign15300_e22695_d_n13;
        locals.var_t1_dn14 = assign15300_e22695_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15310_e22703, assign15310_e22703_d_n0, assign15310_e22703_d_n2, assign15310_e22703_d_n3, assign15310_e22703_d_n4, assign15310_e22703_d_n5, assign15310_e22703_d_n6, assign15310_e22703_d_n7, assign15310_e22703_d_n8, assign15310_e22703_d_n9, assign15310_e22703_d_n10, assign15310_e22703_d_n11, assign15310_e22703_d_n12, assign15310_e22703_d_n13, assign15310_e22703_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15310_e22700: f64 = (1.0 + locals.var_t1);
        let assign15310_e22701: f64 = (locals.var_isbs * assign15310_e22700);
        (assign15310_e22701, ((locals.var_isbs_dn0 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn0)), ((locals.var_isbs_dn2 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn2)), ((locals.var_isbs_dn3 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn3)), ((locals.var_isbs_dn4 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn4)), ((locals.var_isbs_dn5 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn5)), ((locals.var_isbs_dn6 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn6)), ((locals.var_isbs_dn7 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn7)), ((locals.var_isbs_dn8 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn8)), ((locals.var_isbs_dn9 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn9)), ((locals.var_isbs_dn10 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn10)), ((locals.var_isbs_dn11 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn11)), ((locals.var_isbs_dn12 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn12)), ((locals.var_isbs_dn13 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn13)), ((locals.var_isbs_dn14 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn14)),)
    } else {
        (locals.var_ivjsmrev, locals.var_ivjsmrev_dn0, locals.var_ivjsmrev_dn2, locals.var_ivjsmrev_dn3, locals.var_ivjsmrev_dn4, locals.var_ivjsmrev_dn5, locals.var_ivjsmrev_dn6, locals.var_ivjsmrev_dn7, locals.var_ivjsmrev_dn8, locals.var_ivjsmrev_dn9, locals.var_ivjsmrev_dn10, locals.var_ivjsmrev_dn11, locals.var_ivjsmrev_dn12, locals.var_ivjsmrev_dn13, locals.var_ivjsmrev_dn14,)
    }
};
        locals.var_ivjsmrev = assign15310_e22703;
        locals.var_ivjsmrev_dn0 = assign15310_e22703_d_n0;
        locals.var_ivjsmrev_dn2 = assign15310_e22703_d_n2;
        locals.var_ivjsmrev_dn3 = assign15310_e22703_d_n3;
        locals.var_ivjsmrev_dn4 = assign15310_e22703_d_n4;
        locals.var_ivjsmrev_dn5 = assign15310_e22703_d_n5;
        locals.var_ivjsmrev_dn6 = assign15310_e22703_d_n6;
        locals.var_ivjsmrev_dn7 = assign15310_e22703_d_n7;
        locals.var_ivjsmrev_dn8 = assign15310_e22703_d_n8;
        locals.var_ivjsmrev_dn9 = assign15310_e22703_d_n9;
        locals.var_ivjsmrev_dn10 = assign15310_e22703_d_n10;
        locals.var_ivjsmrev_dn11 = assign15310_e22703_d_n11;
        locals.var_ivjsmrev_dn12 = assign15310_e22703_d_n12;
        locals.var_ivjsmrev_dn13 = assign15310_e22703_d_n13;
        locals.var_ivjsmrev_dn14 = assign15310_e22703_d_n14;
        locals.var_ivjsmrev_rv = 0.0;

        let (assign15320_e22712, assign15320_e22712_d_n0, assign15320_e22712_d_n2, assign15320_e22712_d_n3, assign15320_e22712_d_n4, assign15320_e22712_d_n5, assign15320_e22712_d_n6, assign15320_e22712_d_n7, assign15320_e22712_d_n8, assign15320_e22712_d_n9, assign15320_e22712_d_n10, assign15320_e22712_d_n11, assign15320_e22712_d_n12, assign15320_e22712_d_n13, assign15320_e22712_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15320_e22706: f64 = (-locals.var_isbs);
        let assign15320_e22708: f64 = (assign15320_e22706 * locals.var_t1);
        let assign15320_e22710: f64 = (assign15320_e22708 / locals.var_nvtms);
        (assign15320_e22710, ((((-locals.var_isbs_dn0) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn0)) / locals.var_nvtms), ((((-locals.var_isbs_dn2) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn2)) / locals.var_nvtms), ((((-locals.var_isbs_dn3) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn3)) / locals.var_nvtms), ((((((-locals.var_isbs_dn4) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn4)) * locals.var_nvtms) - (assign15320_e22708 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)), ((((-locals.var_isbs_dn5) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn5)) / locals.var_nvtms), ((((-locals.var_isbs_dn6) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn6)) / locals.var_nvtms), ((((-locals.var_isbs_dn7) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn7)) / locals.var_nvtms), ((((-locals.var_isbs_dn8) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn8)) / locals.var_nvtms), ((((-locals.var_isbs_dn9) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn9)) / locals.var_nvtms), ((((-locals.var_isbs_dn10) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn10)) / locals.var_nvtms), ((((-locals.var_isbs_dn11) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn11)) / locals.var_nvtms), ((((-locals.var_isbs_dn12) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn12)) / locals.var_nvtms), ((((-locals.var_isbs_dn13) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn13)) / locals.var_nvtms), ((((-locals.var_isbs_dn14) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn14)) / locals.var_nvtms),)
    } else {
        (locals.var_sslprev, locals.var_sslprev_dn0, locals.var_sslprev_dn2, locals.var_sslprev_dn3, locals.var_sslprev_dn4, locals.var_sslprev_dn5, locals.var_sslprev_dn6, locals.var_sslprev_dn7, locals.var_sslprev_dn8, locals.var_sslprev_dn9, locals.var_sslprev_dn10, locals.var_sslprev_dn11, locals.var_sslprev_dn12, locals.var_sslprev_dn13, locals.var_sslprev_dn14,)
    }
};
        locals.var_sslprev = assign15320_e22712;
        locals.var_sslprev_dn0 = assign15320_e22712_d_n0;
        locals.var_sslprev_dn2 = assign15320_e22712_d_n2;
        locals.var_sslprev_dn3 = assign15320_e22712_d_n3;
        locals.var_sslprev_dn4 = assign15320_e22712_d_n4;
        locals.var_sslprev_dn5 = assign15320_e22712_d_n5;
        locals.var_sslprev_dn6 = assign15320_e22712_d_n6;
        locals.var_sslprev_dn7 = assign15320_e22712_d_n7;
        locals.var_sslprev_dn8 = assign15320_e22712_d_n8;
        locals.var_sslprev_dn9 = assign15320_e22712_d_n9;
        locals.var_sslprev_dn10 = assign15320_e22712_d_n10;
        locals.var_sslprev_dn11 = assign15320_e22712_d_n11;
        locals.var_sslprev_dn12 = assign15320_e22712_d_n12;
        locals.var_sslprev_dn13 = assign15320_e22712_d_n13;
        locals.var_sslprev_dn14 = assign15320_e22712_d_n14;
        locals.var_sslprev_rv = 0.0;

        let (assign15330_e22717, assign15330_e22717_d_n4,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_nvtms, locals.var_nvtms_dn4,)
    }
};
        locals.var_nvtms = assign15330_e22717;
        locals.var_nvtms_dn4 = assign15330_e22717_d_n4;
        locals.var_nvtms_rv = 0.0;

        let (assign15340_e22722, assign15340_e22722_d_n4,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_xexpbvs, locals.var_xexpbvs_dn4,)
    }
};
        locals.var_xexpbvs = assign15340_e22722;
        locals.var_xexpbvs_dn4 = assign15340_e22722_d_n4;
        locals.var_xexpbvs_rv = 0.0;

        let (assign15350_e22727, assign15350_e22727_d_n0, assign15350_e22727_d_n2, assign15350_e22727_d_n3, assign15350_e22727_d_n4, assign15350_e22727_d_n5, assign15350_e22727_d_n6, assign15350_e22727_d_n7, assign15350_e22727_d_n8, assign15350_e22727_d_n9, assign15350_e22727_d_n10, assign15350_e22727_d_n11, assign15350_e22727_d_n12, assign15350_e22727_d_n13, assign15350_e22727_d_n14,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjsmfwd, locals.var_vjsmfwd_dn0, locals.var_vjsmfwd_dn2, locals.var_vjsmfwd_dn3, locals.var_vjsmfwd_dn4, locals.var_vjsmfwd_dn5, locals.var_vjsmfwd_dn6, locals.var_vjsmfwd_dn7, locals.var_vjsmfwd_dn8, locals.var_vjsmfwd_dn9, locals.var_vjsmfwd_dn10, locals.var_vjsmfwd_dn11, locals.var_vjsmfwd_dn12, locals.var_vjsmfwd_dn13, locals.var_vjsmfwd_dn14,)
    }
};
        locals.var_vjsmfwd = assign15350_e22727;
        locals.var_vjsmfwd_dn0 = assign15350_e22727_d_n0;
        locals.var_vjsmfwd_dn2 = assign15350_e22727_d_n2;
        locals.var_vjsmfwd_dn3 = assign15350_e22727_d_n3;
        locals.var_vjsmfwd_dn4 = assign15350_e22727_d_n4;
        locals.var_vjsmfwd_dn5 = assign15350_e22727_d_n5;
        locals.var_vjsmfwd_dn6 = assign15350_e22727_d_n6;
        locals.var_vjsmfwd_dn7 = assign15350_e22727_d_n7;
        locals.var_vjsmfwd_dn8 = assign15350_e22727_d_n8;
        locals.var_vjsmfwd_dn9 = assign15350_e22727_d_n9;
        locals.var_vjsmfwd_dn10 = assign15350_e22727_d_n10;
        locals.var_vjsmfwd_dn11 = assign15350_e22727_d_n11;
        locals.var_vjsmfwd_dn12 = assign15350_e22727_d_n12;
        locals.var_vjsmfwd_dn13 = assign15350_e22727_d_n13;
        locals.var_vjsmfwd_dn14 = assign15350_e22727_d_n14;
        locals.var_vjsmfwd_rv = 0.0;

        let (assign15360_e22732, assign15360_e22732_d_n0, assign15360_e22732_d_n2, assign15360_e22732_d_n3, assign15360_e22732_d_n4, assign15360_e22732_d_n5, assign15360_e22732_d_n6, assign15360_e22732_d_n7, assign15360_e22732_d_n8, assign15360_e22732_d_n9, assign15360_e22732_d_n10, assign15360_e22732_d_n11, assign15360_e22732_d_n12, assign15360_e22732_d_n13, assign15360_e22732_d_n14,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ivjsmfwd, locals.var_ivjsmfwd_dn0, locals.var_ivjsmfwd_dn2, locals.var_ivjsmfwd_dn3, locals.var_ivjsmfwd_dn4, locals.var_ivjsmfwd_dn5, locals.var_ivjsmfwd_dn6, locals.var_ivjsmfwd_dn7, locals.var_ivjsmfwd_dn8, locals.var_ivjsmfwd_dn9, locals.var_ivjsmfwd_dn10, locals.var_ivjsmfwd_dn11, locals.var_ivjsmfwd_dn12, locals.var_ivjsmfwd_dn13, locals.var_ivjsmfwd_dn14,)
    }
};
        locals.var_ivjsmfwd = assign15360_e22732;
        locals.var_ivjsmfwd_dn0 = assign15360_e22732_d_n0;
        locals.var_ivjsmfwd_dn2 = assign15360_e22732_d_n2;
        locals.var_ivjsmfwd_dn3 = assign15360_e22732_d_n3;
        locals.var_ivjsmfwd_dn4 = assign15360_e22732_d_n4;
        locals.var_ivjsmfwd_dn5 = assign15360_e22732_d_n5;
        locals.var_ivjsmfwd_dn6 = assign15360_e22732_d_n6;
        locals.var_ivjsmfwd_dn7 = assign15360_e22732_d_n7;
        locals.var_ivjsmfwd_dn8 = assign15360_e22732_d_n8;
        locals.var_ivjsmfwd_dn9 = assign15360_e22732_d_n9;
        locals.var_ivjsmfwd_dn10 = assign15360_e22732_d_n10;
        locals.var_ivjsmfwd_dn11 = assign15360_e22732_d_n11;
        locals.var_ivjsmfwd_dn12 = assign15360_e22732_d_n12;
        locals.var_ivjsmfwd_dn13 = assign15360_e22732_d_n13;
        locals.var_ivjsmfwd_dn14 = assign15360_e22732_d_n14;
        locals.var_ivjsmfwd_rv = 0.0;

        let (assign15370_e22737, assign15370_e22737_d_n0, assign15370_e22737_d_n2, assign15370_e22737_d_n3, assign15370_e22737_d_n4, assign15370_e22737_d_n5, assign15370_e22737_d_n6, assign15370_e22737_d_n7, assign15370_e22737_d_n8, assign15370_e22737_d_n9, assign15370_e22737_d_n10, assign15370_e22737_d_n11, assign15370_e22737_d_n12, assign15370_e22737_d_n13, assign15370_e22737_d_n14,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sslpfwd, locals.var_sslpfwd_dn0, locals.var_sslpfwd_dn2, locals.var_sslpfwd_dn3, locals.var_sslpfwd_dn4, locals.var_sslpfwd_dn5, locals.var_sslpfwd_dn6, locals.var_sslpfwd_dn7, locals.var_sslpfwd_dn8, locals.var_sslpfwd_dn9, locals.var_sslpfwd_dn10, locals.var_sslpfwd_dn11, locals.var_sslpfwd_dn12, locals.var_sslpfwd_dn13, locals.var_sslpfwd_dn14,)
    }
};
        locals.var_sslpfwd = assign15370_e22737;
        locals.var_sslpfwd_dn0 = assign15370_e22737_d_n0;
        locals.var_sslpfwd_dn2 = assign15370_e22737_d_n2;
        locals.var_sslpfwd_dn3 = assign15370_e22737_d_n3;
        locals.var_sslpfwd_dn4 = assign15370_e22737_d_n4;
        locals.var_sslpfwd_dn5 = assign15370_e22737_d_n5;
        locals.var_sslpfwd_dn6 = assign15370_e22737_d_n6;
        locals.var_sslpfwd_dn7 = assign15370_e22737_d_n7;
        locals.var_sslpfwd_dn8 = assign15370_e22737_d_n8;
        locals.var_sslpfwd_dn9 = assign15370_e22737_d_n9;
        locals.var_sslpfwd_dn10 = assign15370_e22737_d_n10;
        locals.var_sslpfwd_dn11 = assign15370_e22737_d_n11;
        locals.var_sslpfwd_dn12 = assign15370_e22737_d_n12;
        locals.var_sslpfwd_dn13 = assign15370_e22737_d_n13;
        locals.var_sslpfwd_dn14 = assign15370_e22737_d_n14;
        locals.var_sslpfwd_rv = 0.0;

        let (assign15380_e22742, assign15380_e22742_d_n0, assign15380_e22742_d_n2, assign15380_e22742_d_n3, assign15380_e22742_d_n4, assign15380_e22742_d_n5, assign15380_e22742_d_n6, assign15380_e22742_d_n7, assign15380_e22742_d_n8, assign15380_e22742_d_n9, assign15380_e22742_d_n10, assign15380_e22742_d_n11, assign15380_e22742_d_n12, assign15380_e22742_d_n13, assign15380_e22742_d_n14,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjsmrev, locals.var_vjsmrev_dn0, locals.var_vjsmrev_dn2, locals.var_vjsmrev_dn3, locals.var_vjsmrev_dn4, locals.var_vjsmrev_dn5, locals.var_vjsmrev_dn6, locals.var_vjsmrev_dn7, locals.var_vjsmrev_dn8, locals.var_vjsmrev_dn9, locals.var_vjsmrev_dn10, locals.var_vjsmrev_dn11, locals.var_vjsmrev_dn12, locals.var_vjsmrev_dn13, locals.var_vjsmrev_dn14,)
    }
};
        locals.var_vjsmrev = assign15380_e22742;
        locals.var_vjsmrev_dn0 = assign15380_e22742_d_n0;
        locals.var_vjsmrev_dn2 = assign15380_e22742_d_n2;
        locals.var_vjsmrev_dn3 = assign15380_e22742_d_n3;
        locals.var_vjsmrev_dn4 = assign15380_e22742_d_n4;
        locals.var_vjsmrev_dn5 = assign15380_e22742_d_n5;
        locals.var_vjsmrev_dn6 = assign15380_e22742_d_n6;
        locals.var_vjsmrev_dn7 = assign15380_e22742_d_n7;
        locals.var_vjsmrev_dn8 = assign15380_e22742_d_n8;
        locals.var_vjsmrev_dn9 = assign15380_e22742_d_n9;
        locals.var_vjsmrev_dn10 = assign15380_e22742_d_n10;
        locals.var_vjsmrev_dn11 = assign15380_e22742_d_n11;
        locals.var_vjsmrev_dn12 = assign15380_e22742_d_n12;
        locals.var_vjsmrev_dn13 = assign15380_e22742_d_n13;
        locals.var_vjsmrev_dn14 = assign15380_e22742_d_n14;
        locals.var_vjsmrev_rv = 0.0;

        let (assign15390_e22747, assign15390_e22747_d_n0, assign15390_e22747_d_n2, assign15390_e22747_d_n3, assign15390_e22747_d_n4, assign15390_e22747_d_n5, assign15390_e22747_d_n6, assign15390_e22747_d_n7, assign15390_e22747_d_n8, assign15390_e22747_d_n9, assign15390_e22747_d_n10, assign15390_e22747_d_n11, assign15390_e22747_d_n12, assign15390_e22747_d_n13, assign15390_e22747_d_n14,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ivjsmrev, locals.var_ivjsmrev_dn0, locals.var_ivjsmrev_dn2, locals.var_ivjsmrev_dn3, locals.var_ivjsmrev_dn4, locals.var_ivjsmrev_dn5, locals.var_ivjsmrev_dn6, locals.var_ivjsmrev_dn7, locals.var_ivjsmrev_dn8, locals.var_ivjsmrev_dn9, locals.var_ivjsmrev_dn10, locals.var_ivjsmrev_dn11, locals.var_ivjsmrev_dn12, locals.var_ivjsmrev_dn13, locals.var_ivjsmrev_dn14,)
    }
};
        locals.var_ivjsmrev = assign15390_e22747;
        locals.var_ivjsmrev_dn0 = assign15390_e22747_d_n0;
        locals.var_ivjsmrev_dn2 = assign15390_e22747_d_n2;
        locals.var_ivjsmrev_dn3 = assign15390_e22747_d_n3;
        locals.var_ivjsmrev_dn4 = assign15390_e22747_d_n4;
        locals.var_ivjsmrev_dn5 = assign15390_e22747_d_n5;
        locals.var_ivjsmrev_dn6 = assign15390_e22747_d_n6;
        locals.var_ivjsmrev_dn7 = assign15390_e22747_d_n7;
        locals.var_ivjsmrev_dn8 = assign15390_e22747_d_n8;
        locals.var_ivjsmrev_dn9 = assign15390_e22747_d_n9;
        locals.var_ivjsmrev_dn10 = assign15390_e22747_d_n10;
        locals.var_ivjsmrev_dn11 = assign15390_e22747_d_n11;
        locals.var_ivjsmrev_dn12 = assign15390_e22747_d_n12;
        locals.var_ivjsmrev_dn13 = assign15390_e22747_d_n13;
        locals.var_ivjsmrev_dn14 = assign15390_e22747_d_n14;
        locals.var_ivjsmrev_rv = 0.0;

        let (assign15400_e22752, assign15400_e22752_d_n0, assign15400_e22752_d_n2, assign15400_e22752_d_n3, assign15400_e22752_d_n4, assign15400_e22752_d_n5, assign15400_e22752_d_n6, assign15400_e22752_d_n7, assign15400_e22752_d_n8, assign15400_e22752_d_n9, assign15400_e22752_d_n10, assign15400_e22752_d_n11, assign15400_e22752_d_n12, assign15400_e22752_d_n13, assign15400_e22752_d_n14,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sslprev, locals.var_sslprev_dn0, locals.var_sslprev_dn2, locals.var_sslprev_dn3, locals.var_sslprev_dn4, locals.var_sslprev_dn5, locals.var_sslprev_dn6, locals.var_sslprev_dn7, locals.var_sslprev_dn8, locals.var_sslprev_dn9, locals.var_sslprev_dn10, locals.var_sslprev_dn11, locals.var_sslprev_dn12, locals.var_sslprev_dn13, locals.var_sslprev_dn14,)
    }
};
        locals.var_sslprev = assign15400_e22752;
        locals.var_sslprev_dn0 = assign15400_e22752_d_n0;
        locals.var_sslprev_dn2 = assign15400_e22752_d_n2;
        locals.var_sslprev_dn3 = assign15400_e22752_d_n3;
        locals.var_sslprev_dn4 = assign15400_e22752_d_n4;
        locals.var_sslprev_dn5 = assign15400_e22752_d_n5;
        locals.var_sslprev_dn6 = assign15400_e22752_d_n6;
        locals.var_sslprev_dn7 = assign15400_e22752_d_n7;
        locals.var_sslprev_dn8 = assign15400_e22752_d_n8;
        locals.var_sslprev_dn9 = assign15400_e22752_d_n9;
        locals.var_sslprev_dn10 = assign15400_e22752_d_n10;
        locals.var_sslprev_dn11 = assign15400_e22752_d_n11;
        locals.var_sslprev_dn12 = assign15400_e22752_d_n12;
        locals.var_sslprev_dn13 = assign15400_e22752_d_n13;
        locals.var_sslprev_dn14 = assign15400_e22752_d_n14;
        locals.var_sslprev_rv = 0.0;

        let assign15410_e22755: f64 = (locals.var_adeff * locals.var_jsd_t);
        let assign15410_e22758: f64 = (locals.var_pdeff * locals.var_jswd_t);
        let assign15410_e22759: f64 = (assign15410_e22755 + assign15410_e22758);
        let assign15410_e22762: f64 = (locals.var_weffcj * p.p2);
        let assign15410_e22764: f64 = (assign15410_e22762 * locals.var_jswgd_t);
        let assign15410_e22765: f64 = (assign15410_e22759 + assign15410_e22764);
        locals.var_isbd = assign15410_e22765;
        locals.var_isbd_dn0 = ((((locals.var_adeff_dn0 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn0)) + ((locals.var_pdeff_dn0 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn0))) + (assign15410_e22762 * locals.var_jswgd_t_dn0));
        locals.var_isbd_dn2 = ((((locals.var_adeff_dn2 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn2)) + ((locals.var_pdeff_dn2 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn2))) + (assign15410_e22762 * locals.var_jswgd_t_dn2));
        locals.var_isbd_dn3 = ((((locals.var_adeff_dn3 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn3)) + ((locals.var_pdeff_dn3 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn3))) + (assign15410_e22762 * locals.var_jswgd_t_dn3));
        locals.var_isbd_dn4 = ((((locals.var_adeff_dn4 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn4)) + ((locals.var_pdeff_dn4 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn4))) + (assign15410_e22762 * locals.var_jswgd_t_dn4));
        locals.var_isbd_dn5 = ((((locals.var_adeff_dn5 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn5)) + ((locals.var_pdeff_dn5 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn5))) + (assign15410_e22762 * locals.var_jswgd_t_dn5));
        locals.var_isbd_dn6 = ((((locals.var_adeff_dn6 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn6)) + ((locals.var_pdeff_dn6 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn6))) + (assign15410_e22762 * locals.var_jswgd_t_dn6));
        locals.var_isbd_dn7 = ((((locals.var_adeff_dn7 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn7)) + ((locals.var_pdeff_dn7 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn7))) + (assign15410_e22762 * locals.var_jswgd_t_dn7));
        locals.var_isbd_dn8 = ((((locals.var_adeff_dn8 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn8)) + ((locals.var_pdeff_dn8 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn8))) + (assign15410_e22762 * locals.var_jswgd_t_dn8));
        locals.var_isbd_dn9 = ((((locals.var_adeff_dn9 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn9)) + ((locals.var_pdeff_dn9 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn9))) + (assign15410_e22762 * locals.var_jswgd_t_dn9));
        locals.var_isbd_dn10 = ((((locals.var_adeff_dn10 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn10)) + ((locals.var_pdeff_dn10 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn10))) + (assign15410_e22762 * locals.var_jswgd_t_dn10));
        locals.var_isbd_dn11 = ((((locals.var_adeff_dn11 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn11)) + ((locals.var_pdeff_dn11 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn11))) + (assign15410_e22762 * locals.var_jswgd_t_dn11));
        locals.var_isbd_dn12 = ((((locals.var_adeff_dn12 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn12)) + ((locals.var_pdeff_dn12 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn12))) + (assign15410_e22762 * locals.var_jswgd_t_dn12));
        locals.var_isbd_dn13 = ((((locals.var_adeff_dn13 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn13)) + ((locals.var_pdeff_dn13 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn13))) + (assign15410_e22762 * locals.var_jswgd_t_dn13));
        locals.var_isbd_dn14 = ((((locals.var_adeff_dn14 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn14)) + ((locals.var_pdeff_dn14 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn14))) + (assign15410_e22762 * locals.var_jswgd_t_dn14));
        locals.var_isbd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign15420_e22768: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard487 = assign15420_e22768;
        locals.var_guard487_rv = 0.0;

        let (assign15430_e22774, assign15430_e22774_d_n4,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15430_e22772: f64 = (locals.var_vtm * p.p726);
        (assign15430_e22772, (locals.var_vtm_dn4 * p.p726),)
    } else {
        (locals.var_nvtmd, locals.var_nvtmd_dn4,)
    }
};
        locals.var_nvtmd = assign15430_e22774;
        locals.var_nvtmd_dn4 = assign15430_e22774_d_n4;
        locals.var_nvtmd_rv = 0.0;

        let (assign15440_e22784, assign15440_e22784_d_n4,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15440_e22777: f64 = (-p.p732);
        let assign15440_e22779: f64 = (assign15440_e22777 / locals.var_nvtmd);
        let assign15440_e22780: f64 = { let limited_exp_arg = assign15440_e22779; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15440_e22782: f64 = (assign15440_e22780 * p.p734);
        (assign15440_e22782, (({ let limited_exp_arg = assign15440_e22779; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign15440_e22777 * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd)))) * p.p734),)
    } else {
        (locals.var_xexpbvd, locals.var_xexpbvd_dn4,)
    }
};
        locals.var_xexpbvd = assign15440_e22784;
        locals.var_xexpbvd_dn4 = assign15440_e22784_d_n4;
        locals.var_xexpbvd_rv = 0.0;

        let (assign15450_e22792, assign15450_e22792_d_n0, assign15450_e22792_d_n2, assign15450_e22792_d_n3, assign15450_e22792_d_n4, assign15450_e22792_d_n5, assign15450_e22792_d_n6, assign15450_e22792_d_n7, assign15450_e22792_d_n8, assign15450_e22792_d_n9, assign15450_e22792_d_n10, assign15450_e22792_d_n11, assign15450_e22792_d_n12, assign15450_e22792_d_n13, assign15450_e22792_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15450_e22788: f64 = (p.p728 / locals.var_isbd);
        let assign15450_e22790: f64 = (assign15450_e22788).max(10.0);
        (assign15450_e22790, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15450_e22792;
        locals.var_t2_dn0 = assign15450_e22792_d_n0;
        locals.var_t2_dn2 = assign15450_e22792_d_n2;
        locals.var_t2_dn3 = assign15450_e22792_d_n3;
        locals.var_t2_dn4 = assign15450_e22792_d_n4;
        locals.var_t2_dn5 = assign15450_e22792_d_n5;
        locals.var_t2_dn6 = assign15450_e22792_d_n6;
        locals.var_t2_dn7 = assign15450_e22792_d_n7;
        locals.var_t2_dn8 = assign15450_e22792_d_n8;
        locals.var_t2_dn9 = assign15450_e22792_d_n9;
        locals.var_t2_dn10 = assign15450_e22792_d_n10;
        locals.var_t2_dn11 = assign15450_e22792_d_n11;
        locals.var_t2_dn12 = assign15450_e22792_d_n12;
        locals.var_t2_dn13 = assign15450_e22792_d_n13;
        locals.var_t2_dn14 = assign15450_e22792_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign15460_e22800, assign15460_e22800_d_n0, assign15460_e22800_d_n2, assign15460_e22800_d_n3, assign15460_e22800_d_n4, assign15460_e22800_d_n5, assign15460_e22800_d_n6, assign15460_e22800_d_n7, assign15460_e22800_d_n8, assign15460_e22800_d_n9, assign15460_e22800_d_n10, assign15460_e22800_d_n11, assign15460_e22800_d_n12, assign15460_e22800_d_n13, assign15460_e22800_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15460_e22796: f64 = (1.0 + locals.var_t2);
        let assign15460_e22798: f64 = (assign15460_e22796 - locals.var_xexpbvd);
        (assign15460_e22798, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, (locals.var_t2_dn4 - locals.var_xexpbvd_dn4), locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    } else {
        (locals.var_tb, locals.var_tb_dn0, locals.var_tb_dn2, locals.var_tb_dn3, locals.var_tb_dn4, locals.var_tb_dn5, locals.var_tb_dn6, locals.var_tb_dn7, locals.var_tb_dn8, locals.var_tb_dn9, locals.var_tb_dn10, locals.var_tb_dn11, locals.var_tb_dn12, locals.var_tb_dn13, locals.var_tb_dn14,)
    }
};
        locals.var_tb = assign15460_e22800;
        locals.var_tb_dn0 = assign15460_e22800_d_n0;
        locals.var_tb_dn2 = assign15460_e22800_d_n2;
        locals.var_tb_dn3 = assign15460_e22800_d_n3;
        locals.var_tb_dn4 = assign15460_e22800_d_n4;
        locals.var_tb_dn5 = assign15460_e22800_d_n5;
        locals.var_tb_dn6 = assign15460_e22800_d_n6;
        locals.var_tb_dn7 = assign15460_e22800_d_n7;
        locals.var_tb_dn8 = assign15460_e22800_d_n8;
        locals.var_tb_dn9 = assign15460_e22800_d_n9;
        locals.var_tb_dn10 = assign15460_e22800_d_n10;
        locals.var_tb_dn11 = assign15460_e22800_d_n11;
        locals.var_tb_dn12 = assign15460_e22800_d_n12;
        locals.var_tb_dn13 = assign15460_e22800_d_n13;
        locals.var_tb_dn14 = assign15460_e22800_d_n14;
        locals.var_tb_rv = 0.0;

        let (assign15470_e22820, assign15470_e22820_d_n0, assign15470_e22820_d_n2, assign15470_e22820_d_n3, assign15470_e22820_d_n4, assign15470_e22820_d_n5, assign15470_e22820_d_n6, assign15470_e22820_d_n7, assign15470_e22820_d_n8, assign15470_e22820_d_n9, assign15470_e22820_d_n10, assign15470_e22820_d_n11, assign15470_e22820_d_n12, assign15470_e22820_d_n13, assign15470_e22820_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15470_e22807: f64 = (locals.var_tb * locals.var_tb);
        let assign15470_e22810: f64 = (4.0 * locals.var_xexpbvd);
        let assign15470_e22811: f64 = (assign15470_e22807 + assign15470_e22810);
        let assign15470_e22812: f64 = (assign15470_e22811).sqrt();
        let assign15470_e22813: f64 = (locals.var_tb + assign15470_e22812);
        let assign15470_e22814: f64 = (0.5 * assign15470_e22813);
        let assign15470_e22816: f64 = (assign15470_e22814).max(1e-38);
        let assign15470_e22817: f64 = (assign15470_e22816).ln();
        let assign15470_e22818: f64 = (locals.var_nvtmd * assign15470_e22817);
        (assign15470_e22818, (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn0 + (((locals.var_tb_dn0 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn0)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn2 + (((locals.var_tb_dn2 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn2)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn3 + (((locals.var_tb_dn3 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn3)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), ((locals.var_nvtmd_dn4 * assign15470_e22817) + (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn4 + ((((locals.var_tb_dn4 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn4)) + (4.0 * locals.var_xexpbvd_dn4)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816))), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn5 + (((locals.var_tb_dn5 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn5)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn6 + (((locals.var_tb_dn6 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn6)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn7 + (((locals.var_tb_dn7 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn7)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn8 + (((locals.var_tb_dn8 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn8)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn9 + (((locals.var_tb_dn9 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn9)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn10 + (((locals.var_tb_dn10 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn10)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn11 + (((locals.var_tb_dn11 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn11)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn12 + (((locals.var_tb_dn12 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn12)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn13 + (((locals.var_tb_dn13 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn13)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn14 + (((locals.var_tb_dn14 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn14)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)),)
    } else {
        (locals.var_vjdmfwd, locals.var_vjdmfwd_dn0, locals.var_vjdmfwd_dn2, locals.var_vjdmfwd_dn3, locals.var_vjdmfwd_dn4, locals.var_vjdmfwd_dn5, locals.var_vjdmfwd_dn6, locals.var_vjdmfwd_dn7, locals.var_vjdmfwd_dn8, locals.var_vjdmfwd_dn9, locals.var_vjdmfwd_dn10, locals.var_vjdmfwd_dn11, locals.var_vjdmfwd_dn12, locals.var_vjdmfwd_dn13, locals.var_vjdmfwd_dn14,)
    }
};
        locals.var_vjdmfwd = assign15470_e22820;
        locals.var_vjdmfwd_dn0 = assign15470_e22820_d_n0;
        locals.var_vjdmfwd_dn2 = assign15470_e22820_d_n2;
        locals.var_vjdmfwd_dn3 = assign15470_e22820_d_n3;
        locals.var_vjdmfwd_dn4 = assign15470_e22820_d_n4;
        locals.var_vjdmfwd_dn5 = assign15470_e22820_d_n5;
        locals.var_vjdmfwd_dn6 = assign15470_e22820_d_n6;
        locals.var_vjdmfwd_dn7 = assign15470_e22820_d_n7;
        locals.var_vjdmfwd_dn8 = assign15470_e22820_d_n8;
        locals.var_vjdmfwd_dn9 = assign15470_e22820_d_n9;
        locals.var_vjdmfwd_dn10 = assign15470_e22820_d_n10;
        locals.var_vjdmfwd_dn11 = assign15470_e22820_d_n11;
        locals.var_vjdmfwd_dn12 = assign15470_e22820_d_n12;
        locals.var_vjdmfwd_dn13 = assign15470_e22820_d_n13;
        locals.var_vjdmfwd_dn14 = assign15470_e22820_d_n14;
        locals.var_vjdmfwd_rv = 0.0;

        let (assign15480_e22827, assign15480_e22827_d_n0, assign15480_e22827_d_n2, assign15480_e22827_d_n3, assign15480_e22827_d_n4, assign15480_e22827_d_n5, assign15480_e22827_d_n6, assign15480_e22827_d_n7, assign15480_e22827_d_n8, assign15480_e22827_d_n9, assign15480_e22827_d_n10, assign15480_e22827_d_n11, assign15480_e22827_d_n12, assign15480_e22827_d_n13, assign15480_e22827_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15480_e22824: f64 = (locals.var_vjdmfwd / locals.var_nvtmd);
        let assign15480_e22825: f64 = { let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign15480_e22825, ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn0 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn2 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn3 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vjdmfwd_dn4 * locals.var_nvtmd) - (locals.var_vjdmfwd * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd))), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn5 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn6 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn7 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn8 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn9 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn10 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn11 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn12 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn13 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn14 / locals.var_nvtmd)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15480_e22827;
        locals.var_t0_dn0 = assign15480_e22827_d_n0;
        locals.var_t0_dn2 = assign15480_e22827_d_n2;
        locals.var_t0_dn3 = assign15480_e22827_d_n3;
        locals.var_t0_dn4 = assign15480_e22827_d_n4;
        locals.var_t0_dn5 = assign15480_e22827_d_n5;
        locals.var_t0_dn6 = assign15480_e22827_d_n6;
        locals.var_t0_dn7 = assign15480_e22827_d_n7;
        locals.var_t0_dn8 = assign15480_e22827_d_n8;
        locals.var_t0_dn9 = assign15480_e22827_d_n9;
        locals.var_t0_dn10 = assign15480_e22827_d_n10;
        locals.var_t0_dn11 = assign15480_e22827_d_n11;
        locals.var_t0_dn12 = assign15480_e22827_d_n12;
        locals.var_t0_dn13 = assign15480_e22827_d_n13;
        locals.var_t0_dn14 = assign15480_e22827_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15490_e22841, assign15490_e22841_d_n0, assign15490_e22841_d_n2, assign15490_e22841_d_n3, assign15490_e22841_d_n4, assign15490_e22841_d_n5, assign15490_e22841_d_n6, assign15490_e22841_d_n7, assign15490_e22841_d_n8, assign15490_e22841_d_n9, assign15490_e22841_d_n10, assign15490_e22841_d_n11, assign15490_e22841_d_n12, assign15490_e22841_d_n13, assign15490_e22841_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15490_e22833: f64 = (locals.var_xexpbvd / locals.var_t0);
        let assign15490_e22834: f64 = (locals.var_t0 - assign15490_e22833);
        let assign15490_e22836: f64 = (assign15490_e22834 + locals.var_xexpbvd);
        let assign15490_e22838: f64 = (assign15490_e22836 - 1.0);
        let assign15490_e22839: f64 = (locals.var_isbd * assign15490_e22838);
        (assign15490_e22839, ((locals.var_isbd_dn0 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn0 - (-((locals.var_xexpbvd * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn2 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn2 - (-((locals.var_xexpbvd * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn3 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn3 - (-((locals.var_xexpbvd * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn4 * assign15490_e22838) + (locals.var_isbd * ((locals.var_t0_dn4 - (((locals.var_xexpbvd_dn4 * locals.var_t0) - (locals.var_xexpbvd * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))) + locals.var_xexpbvd_dn4))), ((locals.var_isbd_dn5 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn5 - (-((locals.var_xexpbvd * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn6 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn6 - (-((locals.var_xexpbvd * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn7 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn7 - (-((locals.var_xexpbvd * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn8 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn8 - (-((locals.var_xexpbvd * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn9 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn9 - (-((locals.var_xexpbvd * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn10 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn10 - (-((locals.var_xexpbvd * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn11 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn11 - (-((locals.var_xexpbvd * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn12 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn12 - (-((locals.var_xexpbvd * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn13 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn13 - (-((locals.var_xexpbvd * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn14 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn14 - (-((locals.var_xexpbvd * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))),)
    } else {
        (locals.var_ivjdmfwd, locals.var_ivjdmfwd_dn0, locals.var_ivjdmfwd_dn2, locals.var_ivjdmfwd_dn3, locals.var_ivjdmfwd_dn4, locals.var_ivjdmfwd_dn5, locals.var_ivjdmfwd_dn6, locals.var_ivjdmfwd_dn7, locals.var_ivjdmfwd_dn8, locals.var_ivjdmfwd_dn9, locals.var_ivjdmfwd_dn10, locals.var_ivjdmfwd_dn11, locals.var_ivjdmfwd_dn12, locals.var_ivjdmfwd_dn13, locals.var_ivjdmfwd_dn14,)
    }
};
        locals.var_ivjdmfwd = assign15490_e22841;
        locals.var_ivjdmfwd_dn0 = assign15490_e22841_d_n0;
        locals.var_ivjdmfwd_dn2 = assign15490_e22841_d_n2;
        locals.var_ivjdmfwd_dn3 = assign15490_e22841_d_n3;
        locals.var_ivjdmfwd_dn4 = assign15490_e22841_d_n4;
        locals.var_ivjdmfwd_dn5 = assign15490_e22841_d_n5;
        locals.var_ivjdmfwd_dn6 = assign15490_e22841_d_n6;
        locals.var_ivjdmfwd_dn7 = assign15490_e22841_d_n7;
        locals.var_ivjdmfwd_dn8 = assign15490_e22841_d_n8;
        locals.var_ivjdmfwd_dn9 = assign15490_e22841_d_n9;
        locals.var_ivjdmfwd_dn10 = assign15490_e22841_d_n10;
        locals.var_ivjdmfwd_dn11 = assign15490_e22841_d_n11;
        locals.var_ivjdmfwd_dn12 = assign15490_e22841_d_n12;
        locals.var_ivjdmfwd_dn13 = assign15490_e22841_d_n13;
        locals.var_ivjdmfwd_dn14 = assign15490_e22841_d_n14;
        locals.var_ivjdmfwd_rv = 0.0;

        let (assign15500_e22853, assign15500_e22853_d_n0, assign15500_e22853_d_n2, assign15500_e22853_d_n3, assign15500_e22853_d_n4, assign15500_e22853_d_n5, assign15500_e22853_d_n6, assign15500_e22853_d_n7, assign15500_e22853_d_n8, assign15500_e22853_d_n9, assign15500_e22853_d_n10, assign15500_e22853_d_n11, assign15500_e22853_d_n12, assign15500_e22853_d_n13, assign15500_e22853_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15500_e22847: f64 = (locals.var_xexpbvd / locals.var_t0);
        let assign15500_e22848: f64 = (locals.var_t0 + assign15500_e22847);
        let assign15500_e22849: f64 = (locals.var_isbd * assign15500_e22848);
        let assign15500_e22851: f64 = (assign15500_e22849 / locals.var_nvtmd);
        (assign15500_e22851, (((locals.var_isbd_dn0 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn0 + (-((locals.var_xexpbvd * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn2 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn2 + (-((locals.var_xexpbvd * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn3 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn3 + (-((locals.var_xexpbvd * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((((locals.var_isbd_dn4 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn4 + (((locals.var_xexpbvd_dn4 * locals.var_t0) - (locals.var_xexpbvd * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))))) * locals.var_nvtmd) - (assign15500_e22849 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)), (((locals.var_isbd_dn5 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn5 + (-((locals.var_xexpbvd * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn6 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn6 + (-((locals.var_xexpbvd * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn7 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn7 + (-((locals.var_xexpbvd * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn8 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn8 + (-((locals.var_xexpbvd * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn9 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn9 + (-((locals.var_xexpbvd * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn10 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn10 + (-((locals.var_xexpbvd * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn11 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn11 + (-((locals.var_xexpbvd * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn12 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn12 + (-((locals.var_xexpbvd * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn13 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn13 + (-((locals.var_xexpbvd * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn14 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn14 + (-((locals.var_xexpbvd * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd),)
    } else {
        (locals.var_dslpfwd, locals.var_dslpfwd_dn0, locals.var_dslpfwd_dn2, locals.var_dslpfwd_dn3, locals.var_dslpfwd_dn4, locals.var_dslpfwd_dn5, locals.var_dslpfwd_dn6, locals.var_dslpfwd_dn7, locals.var_dslpfwd_dn8, locals.var_dslpfwd_dn9, locals.var_dslpfwd_dn10, locals.var_dslpfwd_dn11, locals.var_dslpfwd_dn12, locals.var_dslpfwd_dn13, locals.var_dslpfwd_dn14,)
    }
};
        locals.var_dslpfwd = assign15500_e22853;
        locals.var_dslpfwd_dn0 = assign15500_e22853_d_n0;
        locals.var_dslpfwd_dn2 = assign15500_e22853_d_n2;
        locals.var_dslpfwd_dn3 = assign15500_e22853_d_n3;
        locals.var_dslpfwd_dn4 = assign15500_e22853_d_n4;
        locals.var_dslpfwd_dn5 = assign15500_e22853_d_n5;
        locals.var_dslpfwd_dn6 = assign15500_e22853_d_n6;
        locals.var_dslpfwd_dn7 = assign15500_e22853_d_n7;
        locals.var_dslpfwd_dn8 = assign15500_e22853_d_n8;
        locals.var_dslpfwd_dn9 = assign15500_e22853_d_n9;
        locals.var_dslpfwd_dn10 = assign15500_e22853_d_n10;
        locals.var_dslpfwd_dn11 = assign15500_e22853_d_n11;
        locals.var_dslpfwd_dn12 = assign15500_e22853_d_n12;
        locals.var_dslpfwd_dn13 = assign15500_e22853_d_n13;
        locals.var_dslpfwd_dn14 = assign15500_e22853_d_n14;
        locals.var_dslpfwd_rv = 0.0;

        let (assign15510_e22918, assign15510_e22918_d_n0, assign15510_e22918_d_n2, assign15510_e22918_d_n3, assign15510_e22918_d_n4, assign15510_e22918_d_n5, assign15510_e22918_d_n6, assign15510_e22918_d_n7, assign15510_e22918_d_n8, assign15510_e22918_d_n9, assign15510_e22918_d_n10, assign15510_e22918_d_n11, assign15510_e22918_d_n12, assign15510_e22918_d_n13, assign15510_e22918_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15510_e22857: f64 = (p.p730 / locals.var_isbd);
        let assign15510_e22859: f64 = (assign15510_e22857 - 10.0);
        let assign15510_e22861: f64 = (-10000.0);
        let assign15510_e22863: f64 = (assign15510_e22861 * 0.001);
        let (assign15510_e22914, assign15510_e22914_d_n0, assign15510_e22914_d_n2, assign15510_e22914_d_n3, assign15510_e22914_d_n4, assign15510_e22914_d_n5, assign15510_e22914_d_n6, assign15510_e22914_d_n7, assign15510_e22914_d_n8, assign15510_e22914_d_n9, assign15510_e22914_d_n10, assign15510_e22914_d_n11, assign15510_e22914_d_n12, assign15510_e22914_d_n13, assign15510_e22914_d_n14,) = {
            if (!(assign15510_e22859 < assign15510_e22863)) {
                let assign15510_e22869: f64 = (p.p730 / locals.var_isbd);
                let assign15510_e22871: f64 = (assign15510_e22869 - 10.0);
                let assign15510_e22874: f64 = (p.p730 / locals.var_isbd);
                let assign15510_e22876: f64 = (assign15510_e22874 - 10.0);
                let assign15510_e22879: f64 = (p.p730 / locals.var_isbd);
                let assign15510_e22881: f64 = (assign15510_e22879 - 10.0);
                let assign15510_e22882: f64 = (assign15510_e22876 * assign15510_e22881);
                let assign15510_e22885: f64 = (4.0 * 0.001);
                let assign15510_e22887: f64 = (assign15510_e22885 * 0.001);
                let assign15510_e22888: f64 = (assign15510_e22882 + assign15510_e22887);
                let assign15510_e22889: f64 = (assign15510_e22888).sqrt();
                let assign15510_e22890: f64 = (assign15510_e22871 + assign15510_e22889);
                let assign15510_e22891: f64 = (0.5 * assign15510_e22890);
                (assign15510_e22891, (0.5 * ((-((p.p730 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))),)
            } else {
                let assign15510_e22894: f64 = (p.p730 / locals.var_isbd);
                let assign15510_e22896: f64 = (assign15510_e22894 - 10.0);
                let assign15510_e22898: f64 = (-10000.0);
                let assign15510_e22900: f64 = (assign15510_e22898 * 0.001);
                let (assign15510_e22913, assign15510_e22913_d_n0, assign15510_e22913_d_n2, assign15510_e22913_d_n3, assign15510_e22913_d_n4, assign15510_e22913_d_n5, assign15510_e22913_d_n6, assign15510_e22913_d_n7, assign15510_e22913_d_n8, assign15510_e22913_d_n9, assign15510_e22913_d_n10, assign15510_e22913_d_n11, assign15510_e22913_d_n12, assign15510_e22913_d_n13, assign15510_e22913_d_n14,) = {
                    if (assign15510_e22896 < assign15510_e22900) {
                        let assign15510_e22903: f64 = (-0.001);
                        let assign15510_e22905: f64 = (assign15510_e22903 * 0.001);
                        let assign15510_e22908: f64 = (p.p730 / locals.var_isbd);
                        let assign15510_e22910: f64 = (assign15510_e22908 - 10.0);
                        let assign15510_e22911: f64 = (assign15510_e22905 / assign15510_e22910);
                        (assign15510_e22911, (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15510_e22913, assign15510_e22913_d_n0, assign15510_e22913_d_n2, assign15510_e22913_d_n3, assign15510_e22913_d_n4, assign15510_e22913_d_n5, assign15510_e22913_d_n6, assign15510_e22913_d_n7, assign15510_e22913_d_n8, assign15510_e22913_d_n9, assign15510_e22913_d_n10, assign15510_e22913_d_n11, assign15510_e22913_d_n12, assign15510_e22913_d_n13, assign15510_e22913_d_n14,)
            }
        };
        let assign15510_e22916: f64 = (assign15510_e22914 + 10.0);
        (assign15510_e22916, assign15510_e22914_d_n0, assign15510_e22914_d_n2, assign15510_e22914_d_n3, assign15510_e22914_d_n4, assign15510_e22914_d_n5, assign15510_e22914_d_n6, assign15510_e22914_d_n7, assign15510_e22914_d_n8, assign15510_e22914_d_n9, assign15510_e22914_d_n10, assign15510_e22914_d_n11, assign15510_e22914_d_n12, assign15510_e22914_d_n13, assign15510_e22914_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15510_e22918;
        locals.var_t2_dn0 = assign15510_e22918_d_n0;
        locals.var_t2_dn2 = assign15510_e22918_d_n2;
        locals.var_t2_dn3 = assign15510_e22918_d_n3;
        locals.var_t2_dn4 = assign15510_e22918_d_n4;
        locals.var_t2_dn5 = assign15510_e22918_d_n5;
        locals.var_t2_dn6 = assign15510_e22918_d_n6;
        locals.var_t2_dn7 = assign15510_e22918_d_n7;
        locals.var_t2_dn8 = assign15510_e22918_d_n8;
        locals.var_t2_dn9 = assign15510_e22918_d_n9;
        locals.var_t2_dn10 = assign15510_e22918_d_n10;
        locals.var_t2_dn11 = assign15510_e22918_d_n11;
        locals.var_t2_dn12 = assign15510_e22918_d_n12;
        locals.var_t2_dn13 = assign15510_e22918_d_n13;
        locals.var_t2_dn14 = assign15510_e22918_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign15520_e22934, assign15520_e22934_d_n0, assign15520_e22934_d_n2, assign15520_e22934_d_n3, assign15520_e22934_d_n4, assign15520_e22934_d_n5, assign15520_e22934_d_n6, assign15520_e22934_d_n7, assign15520_e22934_d_n8, assign15520_e22934_d_n9, assign15520_e22934_d_n10, assign15520_e22934_d_n11, assign15520_e22934_d_n12, assign15520_e22934_d_n13, assign15520_e22934_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15520_e22921: f64 = (-p.p732);
        let assign15520_e22925: f64 = (locals.var_t2 - 1.0);
        let assign15520_e22927: f64 = (assign15520_e22925 / p.p734);
        let assign15520_e22929: f64 = (assign15520_e22927).max(1e-38);
        let assign15520_e22930: f64 = (assign15520_e22929).ln();
        let assign15520_e22931: f64 = (locals.var_nvtmd * assign15520_e22930);
        let assign15520_e22932: f64 = (assign15520_e22921 - assign15520_e22931);
        (assign15520_e22932, (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn0 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn2 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn3 / p.p734) } else { 0.0 } / assign15520_e22929))), (-((locals.var_nvtmd_dn4 * assign15520_e22930) + (locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn4 / p.p734) } else { 0.0 } / assign15520_e22929)))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn5 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn6 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn7 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn8 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn9 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn10 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn11 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn12 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn13 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn14 / p.p734) } else { 0.0 } / assign15520_e22929))),)
    } else {
        (locals.var_vjdmrev, locals.var_vjdmrev_dn0, locals.var_vjdmrev_dn2, locals.var_vjdmrev_dn3, locals.var_vjdmrev_dn4, locals.var_vjdmrev_dn5, locals.var_vjdmrev_dn6, locals.var_vjdmrev_dn7, locals.var_vjdmrev_dn8, locals.var_vjdmrev_dn9, locals.var_vjdmrev_dn10, locals.var_vjdmrev_dn11, locals.var_vjdmrev_dn12, locals.var_vjdmrev_dn13, locals.var_vjdmrev_dn14,)
    }
};
        locals.var_vjdmrev = assign15520_e22934;
        locals.var_vjdmrev_dn0 = assign15520_e22934_d_n0;
        locals.var_vjdmrev_dn2 = assign15520_e22934_d_n2;
        locals.var_vjdmrev_dn3 = assign15520_e22934_d_n3;
        locals.var_vjdmrev_dn4 = assign15520_e22934_d_n4;
        locals.var_vjdmrev_dn5 = assign15520_e22934_d_n5;
        locals.var_vjdmrev_dn6 = assign15520_e22934_d_n6;
        locals.var_vjdmrev_dn7 = assign15520_e22934_d_n7;
        locals.var_vjdmrev_dn8 = assign15520_e22934_d_n8;
        locals.var_vjdmrev_dn9 = assign15520_e22934_d_n9;
        locals.var_vjdmrev_dn10 = assign15520_e22934_d_n10;
        locals.var_vjdmrev_dn11 = assign15520_e22934_d_n11;
        locals.var_vjdmrev_dn12 = assign15520_e22934_d_n12;
        locals.var_vjdmrev_dn13 = assign15520_e22934_d_n13;
        locals.var_vjdmrev_dn14 = assign15520_e22934_d_n14;
        locals.var_vjdmrev_rv = 0.0;

        let (assign15530_e22946, assign15530_e22946_d_n0, assign15530_e22946_d_n2, assign15530_e22946_d_n3, assign15530_e22946_d_n4, assign15530_e22946_d_n5, assign15530_e22946_d_n6, assign15530_e22946_d_n7, assign15530_e22946_d_n8, assign15530_e22946_d_n9, assign15530_e22946_d_n10, assign15530_e22946_d_n11, assign15530_e22946_d_n12, assign15530_e22946_d_n13, assign15530_e22946_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15530_e22939: f64 = (p.p732 + locals.var_vjdmrev);
        let assign15530_e22940: f64 = (-assign15530_e22939);
        let assign15530_e22942: f64 = (assign15530_e22940 / locals.var_nvtmd);
        let assign15530_e22943: f64 = { let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15530_e22944: f64 = (p.p734 * assign15530_e22943);
        (assign15530_e22944, (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn0) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn2) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn3) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((-locals.var_vjdmrev_dn4) * locals.var_nvtmd) - (assign15530_e22940 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn5) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn6) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn7) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn8) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn9) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn10) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn11) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn12) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn13) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn14) / locals.var_nvtmd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15530_e22946;
        locals.var_t1_dn0 = assign15530_e22946_d_n0;
        locals.var_t1_dn2 = assign15530_e22946_d_n2;
        locals.var_t1_dn3 = assign15530_e22946_d_n3;
        locals.var_t1_dn4 = assign15530_e22946_d_n4;
        locals.var_t1_dn5 = assign15530_e22946_d_n5;
        locals.var_t1_dn6 = assign15530_e22946_d_n6;
        locals.var_t1_dn7 = assign15530_e22946_d_n7;
        locals.var_t1_dn8 = assign15530_e22946_d_n8;
        locals.var_t1_dn9 = assign15530_e22946_d_n9;
        locals.var_t1_dn10 = assign15530_e22946_d_n10;
        locals.var_t1_dn11 = assign15530_e22946_d_n11;
        locals.var_t1_dn12 = assign15530_e22946_d_n12;
        locals.var_t1_dn13 = assign15530_e22946_d_n13;
        locals.var_t1_dn14 = assign15530_e22946_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15540_e22954, assign15540_e22954_d_n0, assign15540_e22954_d_n2, assign15540_e22954_d_n3, assign15540_e22954_d_n4, assign15540_e22954_d_n5, assign15540_e22954_d_n6, assign15540_e22954_d_n7, assign15540_e22954_d_n8, assign15540_e22954_d_n9, assign15540_e22954_d_n10, assign15540_e22954_d_n11, assign15540_e22954_d_n12, assign15540_e22954_d_n13, assign15540_e22954_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15540_e22951: f64 = (1.0 + locals.var_t1);
        let assign15540_e22952: f64 = (locals.var_isbd * assign15540_e22951);
        (assign15540_e22952, ((locals.var_isbd_dn0 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn0)), ((locals.var_isbd_dn2 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn2)), ((locals.var_isbd_dn3 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn3)), ((locals.var_isbd_dn4 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn4)), ((locals.var_isbd_dn5 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn5)), ((locals.var_isbd_dn6 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn6)), ((locals.var_isbd_dn7 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn7)), ((locals.var_isbd_dn8 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn8)), ((locals.var_isbd_dn9 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn9)), ((locals.var_isbd_dn10 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn10)), ((locals.var_isbd_dn11 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn11)), ((locals.var_isbd_dn12 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn12)), ((locals.var_isbd_dn13 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn13)), ((locals.var_isbd_dn14 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn14)),)
    } else {
        (locals.var_ivjdmrev, locals.var_ivjdmrev_dn0, locals.var_ivjdmrev_dn2, locals.var_ivjdmrev_dn3, locals.var_ivjdmrev_dn4, locals.var_ivjdmrev_dn5, locals.var_ivjdmrev_dn6, locals.var_ivjdmrev_dn7, locals.var_ivjdmrev_dn8, locals.var_ivjdmrev_dn9, locals.var_ivjdmrev_dn10, locals.var_ivjdmrev_dn11, locals.var_ivjdmrev_dn12, locals.var_ivjdmrev_dn13, locals.var_ivjdmrev_dn14,)
    }
};
        locals.var_ivjdmrev = assign15540_e22954;
        locals.var_ivjdmrev_dn0 = assign15540_e22954_d_n0;
        locals.var_ivjdmrev_dn2 = assign15540_e22954_d_n2;
        locals.var_ivjdmrev_dn3 = assign15540_e22954_d_n3;
        locals.var_ivjdmrev_dn4 = assign15540_e22954_d_n4;
        locals.var_ivjdmrev_dn5 = assign15540_e22954_d_n5;
        locals.var_ivjdmrev_dn6 = assign15540_e22954_d_n6;
        locals.var_ivjdmrev_dn7 = assign15540_e22954_d_n7;
        locals.var_ivjdmrev_dn8 = assign15540_e22954_d_n8;
        locals.var_ivjdmrev_dn9 = assign15540_e22954_d_n9;
        locals.var_ivjdmrev_dn10 = assign15540_e22954_d_n10;
        locals.var_ivjdmrev_dn11 = assign15540_e22954_d_n11;
        locals.var_ivjdmrev_dn12 = assign15540_e22954_d_n12;
        locals.var_ivjdmrev_dn13 = assign15540_e22954_d_n13;
        locals.var_ivjdmrev_dn14 = assign15540_e22954_d_n14;
        locals.var_ivjdmrev_rv = 0.0;

        let (assign15550_e22963, assign15550_e22963_d_n0, assign15550_e22963_d_n2, assign15550_e22963_d_n3, assign15550_e22963_d_n4, assign15550_e22963_d_n5, assign15550_e22963_d_n6, assign15550_e22963_d_n7, assign15550_e22963_d_n8, assign15550_e22963_d_n9, assign15550_e22963_d_n10, assign15550_e22963_d_n11, assign15550_e22963_d_n12, assign15550_e22963_d_n13, assign15550_e22963_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15550_e22957: f64 = (-locals.var_isbd);
        let assign15550_e22959: f64 = (assign15550_e22957 * locals.var_t1);
        let assign15550_e22961: f64 = (assign15550_e22959 / locals.var_nvtmd);
        (assign15550_e22961, ((((-locals.var_isbd_dn0) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn0)) / locals.var_nvtmd), ((((-locals.var_isbd_dn2) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn2)) / locals.var_nvtmd), ((((-locals.var_isbd_dn3) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn3)) / locals.var_nvtmd), ((((((-locals.var_isbd_dn4) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn4)) * locals.var_nvtmd) - (assign15550_e22959 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)), ((((-locals.var_isbd_dn5) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn5)) / locals.var_nvtmd), ((((-locals.var_isbd_dn6) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn6)) / locals.var_nvtmd), ((((-locals.var_isbd_dn7) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn7)) / locals.var_nvtmd), ((((-locals.var_isbd_dn8) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn8)) / locals.var_nvtmd), ((((-locals.var_isbd_dn9) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn9)) / locals.var_nvtmd), ((((-locals.var_isbd_dn10) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn10)) / locals.var_nvtmd), ((((-locals.var_isbd_dn11) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn11)) / locals.var_nvtmd), ((((-locals.var_isbd_dn12) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn12)) / locals.var_nvtmd), ((((-locals.var_isbd_dn13) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn13)) / locals.var_nvtmd), ((((-locals.var_isbd_dn14) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn14)) / locals.var_nvtmd),)
    } else {
        (locals.var_dslprev, locals.var_dslprev_dn0, locals.var_dslprev_dn2, locals.var_dslprev_dn3, locals.var_dslprev_dn4, locals.var_dslprev_dn5, locals.var_dslprev_dn6, locals.var_dslprev_dn7, locals.var_dslprev_dn8, locals.var_dslprev_dn9, locals.var_dslprev_dn10, locals.var_dslprev_dn11, locals.var_dslprev_dn12, locals.var_dslprev_dn13, locals.var_dslprev_dn14,)
    }
};
        locals.var_dslprev = assign15550_e22963;
        locals.var_dslprev_dn0 = assign15550_e22963_d_n0;
        locals.var_dslprev_dn2 = assign15550_e22963_d_n2;
        locals.var_dslprev_dn3 = assign15550_e22963_d_n3;
        locals.var_dslprev_dn4 = assign15550_e22963_d_n4;
        locals.var_dslprev_dn5 = assign15550_e22963_d_n5;
        locals.var_dslprev_dn6 = assign15550_e22963_d_n6;
        locals.var_dslprev_dn7 = assign15550_e22963_d_n7;
        locals.var_dslprev_dn8 = assign15550_e22963_d_n8;
        locals.var_dslprev_dn9 = assign15550_e22963_d_n9;
        locals.var_dslprev_dn10 = assign15550_e22963_d_n10;
        locals.var_dslprev_dn11 = assign15550_e22963_d_n11;
        locals.var_dslprev_dn12 = assign15550_e22963_d_n12;
        locals.var_dslprev_dn13 = assign15550_e22963_d_n13;
        locals.var_dslprev_dn14 = assign15550_e22963_d_n14;
        locals.var_dslprev_rv = 0.0;

        let (assign15560_e22968, assign15560_e22968_d_n4,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_nvtmd, locals.var_nvtmd_dn4,)
    }
};
        locals.var_nvtmd = assign15560_e22968;
        locals.var_nvtmd_dn4 = assign15560_e22968_d_n4;
        locals.var_nvtmd_rv = 0.0;

        let (assign15570_e22973, assign15570_e22973_d_n4,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_xexpbvd, locals.var_xexpbvd_dn4,)
    }
};
        locals.var_xexpbvd = assign15570_e22973;
        locals.var_xexpbvd_dn4 = assign15570_e22973_d_n4;
        locals.var_xexpbvd_rv = 0.0;

        let (assign15580_e22978, assign15580_e22978_d_n0, assign15580_e22978_d_n2, assign15580_e22978_d_n3, assign15580_e22978_d_n4, assign15580_e22978_d_n5, assign15580_e22978_d_n6, assign15580_e22978_d_n7, assign15580_e22978_d_n8, assign15580_e22978_d_n9, assign15580_e22978_d_n10, assign15580_e22978_d_n11, assign15580_e22978_d_n12, assign15580_e22978_d_n13, assign15580_e22978_d_n14,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjdmfwd, locals.var_vjdmfwd_dn0, locals.var_vjdmfwd_dn2, locals.var_vjdmfwd_dn3, locals.var_vjdmfwd_dn4, locals.var_vjdmfwd_dn5, locals.var_vjdmfwd_dn6, locals.var_vjdmfwd_dn7, locals.var_vjdmfwd_dn8, locals.var_vjdmfwd_dn9, locals.var_vjdmfwd_dn10, locals.var_vjdmfwd_dn11, locals.var_vjdmfwd_dn12, locals.var_vjdmfwd_dn13, locals.var_vjdmfwd_dn14,)
    }
};
        locals.var_vjdmfwd = assign15580_e22978;
        locals.var_vjdmfwd_dn0 = assign15580_e22978_d_n0;
        locals.var_vjdmfwd_dn2 = assign15580_e22978_d_n2;
        locals.var_vjdmfwd_dn3 = assign15580_e22978_d_n3;
        locals.var_vjdmfwd_dn4 = assign15580_e22978_d_n4;
        locals.var_vjdmfwd_dn5 = assign15580_e22978_d_n5;
        locals.var_vjdmfwd_dn6 = assign15580_e22978_d_n6;
        locals.var_vjdmfwd_dn7 = assign15580_e22978_d_n7;
        locals.var_vjdmfwd_dn8 = assign15580_e22978_d_n8;
        locals.var_vjdmfwd_dn9 = assign15580_e22978_d_n9;
        locals.var_vjdmfwd_dn10 = assign15580_e22978_d_n10;
        locals.var_vjdmfwd_dn11 = assign15580_e22978_d_n11;
        locals.var_vjdmfwd_dn12 = assign15580_e22978_d_n12;
        locals.var_vjdmfwd_dn13 = assign15580_e22978_d_n13;
        locals.var_vjdmfwd_dn14 = assign15580_e22978_d_n14;
        locals.var_vjdmfwd_rv = 0.0;

        let (assign15590_e22983, assign15590_e22983_d_n0, assign15590_e22983_d_n2, assign15590_e22983_d_n3, assign15590_e22983_d_n4, assign15590_e22983_d_n5, assign15590_e22983_d_n6, assign15590_e22983_d_n7, assign15590_e22983_d_n8, assign15590_e22983_d_n9, assign15590_e22983_d_n10, assign15590_e22983_d_n11, assign15590_e22983_d_n12, assign15590_e22983_d_n13, assign15590_e22983_d_n14,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ivjdmfwd, locals.var_ivjdmfwd_dn0, locals.var_ivjdmfwd_dn2, locals.var_ivjdmfwd_dn3, locals.var_ivjdmfwd_dn4, locals.var_ivjdmfwd_dn5, locals.var_ivjdmfwd_dn6, locals.var_ivjdmfwd_dn7, locals.var_ivjdmfwd_dn8, locals.var_ivjdmfwd_dn9, locals.var_ivjdmfwd_dn10, locals.var_ivjdmfwd_dn11, locals.var_ivjdmfwd_dn12, locals.var_ivjdmfwd_dn13, locals.var_ivjdmfwd_dn14,)
    }
};
        locals.var_ivjdmfwd = assign15590_e22983;
        locals.var_ivjdmfwd_dn0 = assign15590_e22983_d_n0;
        locals.var_ivjdmfwd_dn2 = assign15590_e22983_d_n2;
        locals.var_ivjdmfwd_dn3 = assign15590_e22983_d_n3;
        locals.var_ivjdmfwd_dn4 = assign15590_e22983_d_n4;
        locals.var_ivjdmfwd_dn5 = assign15590_e22983_d_n5;
        locals.var_ivjdmfwd_dn6 = assign15590_e22983_d_n6;
        locals.var_ivjdmfwd_dn7 = assign15590_e22983_d_n7;
        locals.var_ivjdmfwd_dn8 = assign15590_e22983_d_n8;
        locals.var_ivjdmfwd_dn9 = assign15590_e22983_d_n9;
        locals.var_ivjdmfwd_dn10 = assign15590_e22983_d_n10;
        locals.var_ivjdmfwd_dn11 = assign15590_e22983_d_n11;
        locals.var_ivjdmfwd_dn12 = assign15590_e22983_d_n12;
        locals.var_ivjdmfwd_dn13 = assign15590_e22983_d_n13;
        locals.var_ivjdmfwd_dn14 = assign15590_e22983_d_n14;
        locals.var_ivjdmfwd_rv = 0.0;

        let (assign15600_e22988, assign15600_e22988_d_n0, assign15600_e22988_d_n2, assign15600_e22988_d_n3, assign15600_e22988_d_n4, assign15600_e22988_d_n5, assign15600_e22988_d_n6, assign15600_e22988_d_n7, assign15600_e22988_d_n8, assign15600_e22988_d_n9, assign15600_e22988_d_n10, assign15600_e22988_d_n11, assign15600_e22988_d_n12, assign15600_e22988_d_n13, assign15600_e22988_d_n14,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dslpfwd, locals.var_dslpfwd_dn0, locals.var_dslpfwd_dn2, locals.var_dslpfwd_dn3, locals.var_dslpfwd_dn4, locals.var_dslpfwd_dn5, locals.var_dslpfwd_dn6, locals.var_dslpfwd_dn7, locals.var_dslpfwd_dn8, locals.var_dslpfwd_dn9, locals.var_dslpfwd_dn10, locals.var_dslpfwd_dn11, locals.var_dslpfwd_dn12, locals.var_dslpfwd_dn13, locals.var_dslpfwd_dn14,)
    }
};
        locals.var_dslpfwd = assign15600_e22988;
        locals.var_dslpfwd_dn0 = assign15600_e22988_d_n0;
        locals.var_dslpfwd_dn2 = assign15600_e22988_d_n2;
        locals.var_dslpfwd_dn3 = assign15600_e22988_d_n3;
        locals.var_dslpfwd_dn4 = assign15600_e22988_d_n4;
        locals.var_dslpfwd_dn5 = assign15600_e22988_d_n5;
        locals.var_dslpfwd_dn6 = assign15600_e22988_d_n6;
        locals.var_dslpfwd_dn7 = assign15600_e22988_d_n7;
        locals.var_dslpfwd_dn8 = assign15600_e22988_d_n8;
        locals.var_dslpfwd_dn9 = assign15600_e22988_d_n9;
        locals.var_dslpfwd_dn10 = assign15600_e22988_d_n10;
        locals.var_dslpfwd_dn11 = assign15600_e22988_d_n11;
        locals.var_dslpfwd_dn12 = assign15600_e22988_d_n12;
        locals.var_dslpfwd_dn13 = assign15600_e22988_d_n13;
        locals.var_dslpfwd_dn14 = assign15600_e22988_d_n14;
        locals.var_dslpfwd_rv = 0.0;

        let (assign15610_e22993, assign15610_e22993_d_n0, assign15610_e22993_d_n2, assign15610_e22993_d_n3, assign15610_e22993_d_n4, assign15610_e22993_d_n5, assign15610_e22993_d_n6, assign15610_e22993_d_n7, assign15610_e22993_d_n8, assign15610_e22993_d_n9, assign15610_e22993_d_n10, assign15610_e22993_d_n11, assign15610_e22993_d_n12, assign15610_e22993_d_n13, assign15610_e22993_d_n14,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjdmrev, locals.var_vjdmrev_dn0, locals.var_vjdmrev_dn2, locals.var_vjdmrev_dn3, locals.var_vjdmrev_dn4, locals.var_vjdmrev_dn5, locals.var_vjdmrev_dn6, locals.var_vjdmrev_dn7, locals.var_vjdmrev_dn8, locals.var_vjdmrev_dn9, locals.var_vjdmrev_dn10, locals.var_vjdmrev_dn11, locals.var_vjdmrev_dn12, locals.var_vjdmrev_dn13, locals.var_vjdmrev_dn14,)
    }
};
        locals.var_vjdmrev = assign15610_e22993;
        locals.var_vjdmrev_dn0 = assign15610_e22993_d_n0;
        locals.var_vjdmrev_dn2 = assign15610_e22993_d_n2;
        locals.var_vjdmrev_dn3 = assign15610_e22993_d_n3;
        locals.var_vjdmrev_dn4 = assign15610_e22993_d_n4;
        locals.var_vjdmrev_dn5 = assign15610_e22993_d_n5;
        locals.var_vjdmrev_dn6 = assign15610_e22993_d_n6;
        locals.var_vjdmrev_dn7 = assign15610_e22993_d_n7;
        locals.var_vjdmrev_dn8 = assign15610_e22993_d_n8;
        locals.var_vjdmrev_dn9 = assign15610_e22993_d_n9;
        locals.var_vjdmrev_dn10 = assign15610_e22993_d_n10;
        locals.var_vjdmrev_dn11 = assign15610_e22993_d_n11;
        locals.var_vjdmrev_dn12 = assign15610_e22993_d_n12;
        locals.var_vjdmrev_dn13 = assign15610_e22993_d_n13;
        locals.var_vjdmrev_dn14 = assign15610_e22993_d_n14;
        locals.var_vjdmrev_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15620_e22998, assign15620_e22998_d_n0, assign15620_e22998_d_n2, assign15620_e22998_d_n3, assign15620_e22998_d_n4, assign15620_e22998_d_n5, assign15620_e22998_d_n6, assign15620_e22998_d_n7, assign15620_e22998_d_n8, assign15620_e22998_d_n9, assign15620_e22998_d_n10, assign15620_e22998_d_n11, assign15620_e22998_d_n12, assign15620_e22998_d_n13, assign15620_e22998_d_n14,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ivjdmrev, locals.var_ivjdmrev_dn0, locals.var_ivjdmrev_dn2, locals.var_ivjdmrev_dn3, locals.var_ivjdmrev_dn4, locals.var_ivjdmrev_dn5, locals.var_ivjdmrev_dn6, locals.var_ivjdmrev_dn7, locals.var_ivjdmrev_dn8, locals.var_ivjdmrev_dn9, locals.var_ivjdmrev_dn10, locals.var_ivjdmrev_dn11, locals.var_ivjdmrev_dn12, locals.var_ivjdmrev_dn13, locals.var_ivjdmrev_dn14,)
    }
};
        locals.var_ivjdmrev = assign15620_e22998;
        locals.var_ivjdmrev_dn0 = assign15620_e22998_d_n0;
        locals.var_ivjdmrev_dn2 = assign15620_e22998_d_n2;
        locals.var_ivjdmrev_dn3 = assign15620_e22998_d_n3;
        locals.var_ivjdmrev_dn4 = assign15620_e22998_d_n4;
        locals.var_ivjdmrev_dn5 = assign15620_e22998_d_n5;
        locals.var_ivjdmrev_dn6 = assign15620_e22998_d_n6;
        locals.var_ivjdmrev_dn7 = assign15620_e22998_d_n7;
        locals.var_ivjdmrev_dn8 = assign15620_e22998_d_n8;
        locals.var_ivjdmrev_dn9 = assign15620_e22998_d_n9;
        locals.var_ivjdmrev_dn10 = assign15620_e22998_d_n10;
        locals.var_ivjdmrev_dn11 = assign15620_e22998_d_n11;
        locals.var_ivjdmrev_dn12 = assign15620_e22998_d_n12;
        locals.var_ivjdmrev_dn13 = assign15620_e22998_d_n13;
        locals.var_ivjdmrev_dn14 = assign15620_e22998_d_n14;
        locals.var_ivjdmrev_rv = 0.0;

        let (assign15630_e23003, assign15630_e23003_d_n0, assign15630_e23003_d_n2, assign15630_e23003_d_n3, assign15630_e23003_d_n4, assign15630_e23003_d_n5, assign15630_e23003_d_n6, assign15630_e23003_d_n7, assign15630_e23003_d_n8, assign15630_e23003_d_n9, assign15630_e23003_d_n10, assign15630_e23003_d_n11, assign15630_e23003_d_n12, assign15630_e23003_d_n13, assign15630_e23003_d_n14,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dslprev, locals.var_dslprev_dn0, locals.var_dslprev_dn2, locals.var_dslprev_dn3, locals.var_dslprev_dn4, locals.var_dslprev_dn5, locals.var_dslprev_dn6, locals.var_dslprev_dn7, locals.var_dslprev_dn8, locals.var_dslprev_dn9, locals.var_dslprev_dn10, locals.var_dslprev_dn11, locals.var_dslprev_dn12, locals.var_dslprev_dn13, locals.var_dslprev_dn14,)
    }
};
        locals.var_dslprev = assign15630_e23003;
        locals.var_dslprev_dn0 = assign15630_e23003_d_n0;
        locals.var_dslprev_dn2 = assign15630_e23003_d_n2;
        locals.var_dslprev_dn3 = assign15630_e23003_d_n3;
        locals.var_dslprev_dn4 = assign15630_e23003_d_n4;
        locals.var_dslprev_dn5 = assign15630_e23003_d_n5;
        locals.var_dslprev_dn6 = assign15630_e23003_d_n6;
        locals.var_dslprev_dn7 = assign15630_e23003_d_n7;
        locals.var_dslprev_dn8 = assign15630_e23003_d_n8;
        locals.var_dslprev_dn9 = assign15630_e23003_d_n9;
        locals.var_dslprev_dn10 = assign15630_e23003_d_n10;
        locals.var_dslprev_dn11 = assign15630_e23003_d_n11;
        locals.var_dslprev_dn12 = assign15630_e23003_d_n12;
        locals.var_dslprev_dn13 = assign15630_e23003_d_n13;
        locals.var_dslprev_dn14 = assign15630_e23003_d_n14;
        locals.var_dslprev_rv = 0.0;

        let assign15640_e23022: f64 = if (((p.p17 > 0.0) && (p.p18 > 0.0)) && ((p.p2 == 1.0) || ((p.p2 > 1.0) && (p.p19 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard488 = assign15640_e23022;
        locals.var_guard488_rv = 0.0;

        let (assign15650_e23028, assign15650_e23028_d_n0, assign15650_e23028_d_n2, assign15650_e23028_d_n3, assign15650_e23028_d_n4, assign15650_e23028_d_n5, assign15650_e23028_d_n6, assign15650_e23028_d_n7, assign15650_e23028_d_n8, assign15650_e23028_d_n9, assign15650_e23028_d_n10, assign15650_e23028_d_n11, assign15650_e23028_d_n12, assign15650_e23028_d_n13, assign15650_e23028_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15650_e23026: f64 = (locals.var_lnew).powf(p.p921);
        (assign15650_e23026, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15650_e23028;
        locals.var_t0_dn0 = assign15650_e23028_d_n0;
        locals.var_t0_dn2 = assign15650_e23028_d_n2;
        locals.var_t0_dn3 = assign15650_e23028_d_n3;
        locals.var_t0_dn4 = assign15650_e23028_d_n4;
        locals.var_t0_dn5 = assign15650_e23028_d_n5;
        locals.var_t0_dn6 = assign15650_e23028_d_n6;
        locals.var_t0_dn7 = assign15650_e23028_d_n7;
        locals.var_t0_dn8 = assign15650_e23028_d_n8;
        locals.var_t0_dn9 = assign15650_e23028_d_n9;
        locals.var_t0_dn10 = assign15650_e23028_d_n10;
        locals.var_t0_dn11 = assign15650_e23028_d_n11;
        locals.var_t0_dn12 = assign15650_e23028_d_n12;
        locals.var_t0_dn13 = assign15650_e23028_d_n13;
        locals.var_t0_dn14 = assign15650_e23028_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15660_e23034,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15660_e23032: f64 = (locals.var_wnew + p.p914);
        (assign15660_e23032,)
    } else {
        (locals.var_w_tmp_stress,)
    }
};
        locals.var_w_tmp_stress = assign15660_e23034;
        locals.var_w_tmp_stress_rv = 0.0;

        let (assign15670_e23040, assign15670_e23040_d_n0, assign15670_e23040_d_n2, assign15670_e23040_d_n3, assign15670_e23040_d_n4, assign15670_e23040_d_n5, assign15670_e23040_d_n6, assign15670_e23040_d_n7, assign15670_e23040_d_n8, assign15670_e23040_d_n9, assign15670_e23040_d_n10, assign15670_e23040_d_n11, assign15670_e23040_d_n12, assign15670_e23040_d_n13, assign15670_e23040_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15670_e23038: f64 = (locals.var_w_tmp_stress).powf(p.p922);
        (assign15670_e23038, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15670_e23040;
        locals.var_t1_dn0 = assign15670_e23040_d_n0;
        locals.var_t1_dn2 = assign15670_e23040_d_n2;
        locals.var_t1_dn3 = assign15670_e23040_d_n3;
        locals.var_t1_dn4 = assign15670_e23040_d_n4;
        locals.var_t1_dn5 = assign15670_e23040_d_n5;
        locals.var_t1_dn6 = assign15670_e23040_d_n6;
        locals.var_t1_dn7 = assign15670_e23040_d_n7;
        locals.var_t1_dn8 = assign15670_e23040_d_n8;
        locals.var_t1_dn9 = assign15670_e23040_d_n9;
        locals.var_t1_dn10 = assign15670_e23040_d_n10;
        locals.var_t1_dn11 = assign15670_e23040_d_n11;
        locals.var_t1_dn12 = assign15670_e23040_d_n12;
        locals.var_t1_dn13 = assign15670_e23040_d_n13;
        locals.var_t1_dn14 = assign15670_e23040_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15680_e23056, assign15680_e23056_d_n0, assign15680_e23056_d_n2, assign15680_e23056_d_n3, assign15680_e23056_d_n4, assign15680_e23056_d_n5, assign15680_e23056_d_n6, assign15680_e23056_d_n7, assign15680_e23056_d_n8, assign15680_e23056_d_n9, assign15680_e23056_d_n10, assign15680_e23056_d_n11, assign15680_e23056_d_n12, assign15680_e23056_d_n13, assign15680_e23056_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15680_e23044: f64 = (p.p918 / locals.var_t0);
        let assign15680_e23047: f64 = (p.p919 / locals.var_t1);
        let assign15680_e23048: f64 = (assign15680_e23044 + assign15680_e23047);
        let assign15680_e23052: f64 = (locals.var_t0 * locals.var_t1);
        let assign15680_e23053: f64 = (p.p920 / assign15680_e23052);
        let assign15680_e23054: f64 = (assign15680_e23048 + assign15680_e23053);
        (assign15680_e23054, (((-((p.p918 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn0 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn0))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn2 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn2))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn13 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn13))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn14 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn14))) / (assign15680_e23052 * assign15680_e23052)))),)
    } else {
        (locals.var_tmp1_stress, locals.var_tmp1_stress_dn0, locals.var_tmp1_stress_dn2, locals.var_tmp1_stress_dn3, locals.var_tmp1_stress_dn4, locals.var_tmp1_stress_dn5, locals.var_tmp1_stress_dn6, locals.var_tmp1_stress_dn7, locals.var_tmp1_stress_dn8, locals.var_tmp1_stress_dn9, locals.var_tmp1_stress_dn10, locals.var_tmp1_stress_dn11, locals.var_tmp1_stress_dn12, locals.var_tmp1_stress_dn13, locals.var_tmp1_stress_dn14,)
    }
};
        locals.var_tmp1_stress = assign15680_e23056;
        locals.var_tmp1_stress_dn0 = assign15680_e23056_d_n0;
        locals.var_tmp1_stress_dn2 = assign15680_e23056_d_n2;
        locals.var_tmp1_stress_dn3 = assign15680_e23056_d_n3;
        locals.var_tmp1_stress_dn4 = assign15680_e23056_d_n4;
        locals.var_tmp1_stress_dn5 = assign15680_e23056_d_n5;
        locals.var_tmp1_stress_dn6 = assign15680_e23056_d_n6;
        locals.var_tmp1_stress_dn7 = assign15680_e23056_d_n7;
        locals.var_tmp1_stress_dn8 = assign15680_e23056_d_n8;
        locals.var_tmp1_stress_dn9 = assign15680_e23056_d_n9;
        locals.var_tmp1_stress_dn10 = assign15680_e23056_d_n10;
        locals.var_tmp1_stress_dn11 = assign15680_e23056_d_n11;
        locals.var_tmp1_stress_dn12 = assign15680_e23056_d_n12;
        locals.var_tmp1_stress_dn13 = assign15680_e23056_d_n13;
        locals.var_tmp1_stress_dn14 = assign15680_e23056_d_n14;
        locals.var_tmp1_stress_rv = 0.0;

        let (assign15690_e23062, assign15690_e23062_d_n0, assign15690_e23062_d_n2, assign15690_e23062_d_n3, assign15690_e23062_d_n4, assign15690_e23062_d_n5, assign15690_e23062_d_n6, assign15690_e23062_d_n7, assign15690_e23062_d_n8, assign15690_e23062_d_n9, assign15690_e23062_d_n10, assign15690_e23062_d_n11, assign15690_e23062_d_n12, assign15690_e23062_d_n13, assign15690_e23062_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15690_e23060: f64 = (1.0 + locals.var_tmp1_stress);
        (assign15690_e23060, locals.var_tmp1_stress_dn0, locals.var_tmp1_stress_dn2, locals.var_tmp1_stress_dn3, locals.var_tmp1_stress_dn4, locals.var_tmp1_stress_dn5, locals.var_tmp1_stress_dn6, locals.var_tmp1_stress_dn7, locals.var_tmp1_stress_dn8, locals.var_tmp1_stress_dn9, locals.var_tmp1_stress_dn10, locals.var_tmp1_stress_dn11, locals.var_tmp1_stress_dn12, locals.var_tmp1_stress_dn13, locals.var_tmp1_stress_dn14,)
    } else {
        (locals.var_kstress_u0, locals.var_kstress_u0_dn0, locals.var_kstress_u0_dn2, locals.var_kstress_u0_dn3, locals.var_kstress_u0_dn4, locals.var_kstress_u0_dn5, locals.var_kstress_u0_dn6, locals.var_kstress_u0_dn7, locals.var_kstress_u0_dn8, locals.var_kstress_u0_dn9, locals.var_kstress_u0_dn10, locals.var_kstress_u0_dn11, locals.var_kstress_u0_dn12, locals.var_kstress_u0_dn13, locals.var_kstress_u0_dn14,)
    }
};
        locals.var_kstress_u0 = assign15690_e23062;
        locals.var_kstress_u0_dn0 = assign15690_e23062_d_n0;
        locals.var_kstress_u0_dn2 = assign15690_e23062_d_n2;
        locals.var_kstress_u0_dn3 = assign15690_e23062_d_n3;
        locals.var_kstress_u0_dn4 = assign15690_e23062_d_n4;
        locals.var_kstress_u0_dn5 = assign15690_e23062_d_n5;
        locals.var_kstress_u0_dn6 = assign15690_e23062_d_n6;
        locals.var_kstress_u0_dn7 = assign15690_e23062_d_n7;
        locals.var_kstress_u0_dn8 = assign15690_e23062_d_n8;
        locals.var_kstress_u0_dn9 = assign15690_e23062_d_n9;
        locals.var_kstress_u0_dn10 = assign15690_e23062_d_n10;
        locals.var_kstress_u0_dn11 = assign15690_e23062_d_n11;
        locals.var_kstress_u0_dn12 = assign15690_e23062_d_n12;
        locals.var_kstress_u0_dn13 = assign15690_e23062_d_n13;
        locals.var_kstress_u0_dn14 = assign15690_e23062_d_n14;
        locals.var_kstress_u0_rv = 0.0;

        let (assign15700_e23068, assign15700_e23068_d_n0, assign15700_e23068_d_n2, assign15700_e23068_d_n3, assign15700_e23068_d_n4, assign15700_e23068_d_n5, assign15700_e23068_d_n6, assign15700_e23068_d_n7, assign15700_e23068_d_n8, assign15700_e23068_d_n9, assign15700_e23068_d_n10, assign15700_e23068_d_n11, assign15700_e23068_d_n12, assign15700_e23068_d_n13, assign15700_e23068_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15700_e23066: f64 = (locals.var_lnew).powf(p.p927);
        (assign15700_e23066, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15700_e23068;
        locals.var_t0_dn0 = assign15700_e23068_d_n0;
        locals.var_t0_dn2 = assign15700_e23068_d_n2;
        locals.var_t0_dn3 = assign15700_e23068_d_n3;
        locals.var_t0_dn4 = assign15700_e23068_d_n4;
        locals.var_t0_dn5 = assign15700_e23068_d_n5;
        locals.var_t0_dn6 = assign15700_e23068_d_n6;
        locals.var_t0_dn7 = assign15700_e23068_d_n7;
        locals.var_t0_dn8 = assign15700_e23068_d_n8;
        locals.var_t0_dn9 = assign15700_e23068_d_n9;
        locals.var_t0_dn10 = assign15700_e23068_d_n10;
        locals.var_t0_dn11 = assign15700_e23068_d_n11;
        locals.var_t0_dn12 = assign15700_e23068_d_n12;
        locals.var_t0_dn13 = assign15700_e23068_d_n13;
        locals.var_t0_dn14 = assign15700_e23068_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15710_e23074, assign15710_e23074_d_n0, assign15710_e23074_d_n2, assign15710_e23074_d_n3, assign15710_e23074_d_n4, assign15710_e23074_d_n5, assign15710_e23074_d_n6, assign15710_e23074_d_n7, assign15710_e23074_d_n8, assign15710_e23074_d_n9, assign15710_e23074_d_n10, assign15710_e23074_d_n11, assign15710_e23074_d_n12, assign15710_e23074_d_n13, assign15710_e23074_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15710_e23072: f64 = (locals.var_w_tmp_stress).powf(p.p928);
        (assign15710_e23072, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15710_e23074;
        locals.var_t1_dn0 = assign15710_e23074_d_n0;
        locals.var_t1_dn2 = assign15710_e23074_d_n2;
        locals.var_t1_dn3 = assign15710_e23074_d_n3;
        locals.var_t1_dn4 = assign15710_e23074_d_n4;
        locals.var_t1_dn5 = assign15710_e23074_d_n5;
        locals.var_t1_dn6 = assign15710_e23074_d_n6;
        locals.var_t1_dn7 = assign15710_e23074_d_n7;
        locals.var_t1_dn8 = assign15710_e23074_d_n8;
        locals.var_t1_dn9 = assign15710_e23074_d_n9;
        locals.var_t1_dn10 = assign15710_e23074_d_n10;
        locals.var_t1_dn11 = assign15710_e23074_d_n11;
        locals.var_t1_dn12 = assign15710_e23074_d_n12;
        locals.var_t1_dn13 = assign15710_e23074_d_n13;
        locals.var_t1_dn14 = assign15710_e23074_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15720_e23090, assign15720_e23090_d_n0, assign15720_e23090_d_n2, assign15720_e23090_d_n3, assign15720_e23090_d_n4, assign15720_e23090_d_n5, assign15720_e23090_d_n6, assign15720_e23090_d_n7, assign15720_e23090_d_n8, assign15720_e23090_d_n9, assign15720_e23090_d_n10, assign15720_e23090_d_n11, assign15720_e23090_d_n12, assign15720_e23090_d_n13, assign15720_e23090_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15720_e23078: f64 = (p.p924 / locals.var_t0);
        let assign15720_e23081: f64 = (p.p925 / locals.var_t1);
        let assign15720_e23082: f64 = (assign15720_e23078 + assign15720_e23081);
        let assign15720_e23086: f64 = (locals.var_t0 * locals.var_t1);
        let assign15720_e23087: f64 = (p.p926 / assign15720_e23086);
        let assign15720_e23088: f64 = (assign15720_e23082 + assign15720_e23087);
        (assign15720_e23088, (((-((p.p924 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn0 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn0))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn2 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn2))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn13 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn13))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn14 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn14))) / (assign15720_e23086 * assign15720_e23086)))),)
    } else {
        (locals.var_tmp1_stress_vth, locals.var_tmp1_stress_vth_dn0, locals.var_tmp1_stress_vth_dn2, locals.var_tmp1_stress_vth_dn3, locals.var_tmp1_stress_vth_dn4, locals.var_tmp1_stress_vth_dn5, locals.var_tmp1_stress_vth_dn6, locals.var_tmp1_stress_vth_dn7, locals.var_tmp1_stress_vth_dn8, locals.var_tmp1_stress_vth_dn9, locals.var_tmp1_stress_vth_dn10, locals.var_tmp1_stress_vth_dn11, locals.var_tmp1_stress_vth_dn12, locals.var_tmp1_stress_vth_dn13, locals.var_tmp1_stress_vth_dn14,)
    }
};
        locals.var_tmp1_stress_vth = assign15720_e23090;
        locals.var_tmp1_stress_vth_dn0 = assign15720_e23090_d_n0;
        locals.var_tmp1_stress_vth_dn2 = assign15720_e23090_d_n2;
        locals.var_tmp1_stress_vth_dn3 = assign15720_e23090_d_n3;
        locals.var_tmp1_stress_vth_dn4 = assign15720_e23090_d_n4;
        locals.var_tmp1_stress_vth_dn5 = assign15720_e23090_d_n5;
        locals.var_tmp1_stress_vth_dn6 = assign15720_e23090_d_n6;
        locals.var_tmp1_stress_vth_dn7 = assign15720_e23090_d_n7;
        locals.var_tmp1_stress_vth_dn8 = assign15720_e23090_d_n8;
        locals.var_tmp1_stress_vth_dn9 = assign15720_e23090_d_n9;
        locals.var_tmp1_stress_vth_dn10 = assign15720_e23090_d_n10;
        locals.var_tmp1_stress_vth_dn11 = assign15720_e23090_d_n11;
        locals.var_tmp1_stress_vth_dn12 = assign15720_e23090_d_n12;
        locals.var_tmp1_stress_vth_dn13 = assign15720_e23090_d_n13;
        locals.var_tmp1_stress_vth_dn14 = assign15720_e23090_d_n14;
        locals.var_tmp1_stress_vth_rv = 0.0;

        let (assign15730_e23096, assign15730_e23096_d_n0, assign15730_e23096_d_n2, assign15730_e23096_d_n3, assign15730_e23096_d_n4, assign15730_e23096_d_n5, assign15730_e23096_d_n6, assign15730_e23096_d_n7, assign15730_e23096_d_n8, assign15730_e23096_d_n9, assign15730_e23096_d_n10, assign15730_e23096_d_n11, assign15730_e23096_d_n12, assign15730_e23096_d_n13, assign15730_e23096_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15730_e23094: f64 = (1.0 + locals.var_tmp1_stress_vth);
        (assign15730_e23094, locals.var_tmp1_stress_vth_dn0, locals.var_tmp1_stress_vth_dn2, locals.var_tmp1_stress_vth_dn3, locals.var_tmp1_stress_vth_dn4, locals.var_tmp1_stress_vth_dn5, locals.var_tmp1_stress_vth_dn6, locals.var_tmp1_stress_vth_dn7, locals.var_tmp1_stress_vth_dn8, locals.var_tmp1_stress_vth_dn9, locals.var_tmp1_stress_vth_dn10, locals.var_tmp1_stress_vth_dn11, locals.var_tmp1_stress_vth_dn12, locals.var_tmp1_stress_vth_dn13, locals.var_tmp1_stress_vth_dn14,)
    } else {
        (locals.var_kstress_vth0, locals.var_kstress_vth0_dn0, locals.var_kstress_vth0_dn2, locals.var_kstress_vth0_dn3, locals.var_kstress_vth0_dn4, locals.var_kstress_vth0_dn5, locals.var_kstress_vth0_dn6, locals.var_kstress_vth0_dn7, locals.var_kstress_vth0_dn8, locals.var_kstress_vth0_dn9, locals.var_kstress_vth0_dn10, locals.var_kstress_vth0_dn11, locals.var_kstress_vth0_dn12, locals.var_kstress_vth0_dn13, locals.var_kstress_vth0_dn14,)
    }
};
        locals.var_kstress_vth0 = assign15730_e23096;
        locals.var_kstress_vth0_dn0 = assign15730_e23096_d_n0;
        locals.var_kstress_vth0_dn2 = assign15730_e23096_d_n2;
        locals.var_kstress_vth0_dn3 = assign15730_e23096_d_n3;
        locals.var_kstress_vth0_dn4 = assign15730_e23096_d_n4;
        locals.var_kstress_vth0_dn5 = assign15730_e23096_d_n5;
        locals.var_kstress_vth0_dn6 = assign15730_e23096_d_n6;
        locals.var_kstress_vth0_dn7 = assign15730_e23096_d_n7;
        locals.var_kstress_vth0_dn8 = assign15730_e23096_d_n8;
        locals.var_kstress_vth0_dn9 = assign15730_e23096_d_n9;
        locals.var_kstress_vth0_dn10 = assign15730_e23096_d_n10;
        locals.var_kstress_vth0_dn11 = assign15730_e23096_d_n11;
        locals.var_kstress_vth0_dn12 = assign15730_e23096_d_n12;
        locals.var_kstress_vth0_dn13 = assign15730_e23096_d_n13;
        locals.var_kstress_vth0_dn14 = assign15730_e23096_d_n14;
        locals.var_kstress_vth0_rv = 0.0;

        let (assign15740_e23102, assign15740_e23102_d_n0, assign15740_e23102_d_n2, assign15740_e23102_d_n3, assign15740_e23102_d_n4, assign15740_e23102_d_n5, assign15740_e23102_d_n6, assign15740_e23102_d_n7, assign15740_e23102_d_n8, assign15740_e23102_d_n9, assign15740_e23102_d_n10, assign15740_e23102_d_n11, assign15740_e23102_d_n12, assign15740_e23102_d_n13, assign15740_e23102_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15740_e23100: f64 = (locals.var_tratio - 1.0);
        (assign15740_e23100, 0.0, 0.0, 0.0, locals.var_tratio_dn4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15740_e23102;
        locals.var_t0_dn0 = assign15740_e23102_d_n0;
        locals.var_t0_dn2 = assign15740_e23102_d_n2;
        locals.var_t0_dn3 = assign15740_e23102_d_n3;
        locals.var_t0_dn4 = assign15740_e23102_d_n4;
        locals.var_t0_dn5 = assign15740_e23102_d_n5;
        locals.var_t0_dn6 = assign15740_e23102_d_n6;
        locals.var_t0_dn7 = assign15740_e23102_d_n7;
        locals.var_t0_dn8 = assign15740_e23102_d_n8;
        locals.var_t0_dn9 = assign15740_e23102_d_n9;
        locals.var_t0_dn10 = assign15740_e23102_d_n10;
        locals.var_t0_dn11 = assign15740_e23102_d_n11;
        locals.var_t0_dn12 = assign15740_e23102_d_n12;
        locals.var_t0_dn13 = assign15740_e23102_d_n13;
        locals.var_t0_dn14 = assign15740_e23102_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15750_e23114, assign15750_e23114_d_n0, assign15750_e23114_d_n2, assign15750_e23114_d_n3, assign15750_e23114_d_n4, assign15750_e23114_d_n5, assign15750_e23114_d_n6, assign15750_e23114_d_n7, assign15750_e23114_d_n8, assign15750_e23114_d_n9, assign15750_e23114_d_n10, assign15750_e23114_d_n11, assign15750_e23114_d_n12, assign15750_e23114_d_n13, assign15750_e23114_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15750_e23108: f64 = (p.p917 * locals.var_t0);
        let assign15750_e23109: f64 = (1.0 + assign15750_e23108);
        let assign15750_e23110: f64 = (locals.var_kstress_u0 * assign15750_e23109);
        let assign15750_e23112: f64 = (assign15750_e23110 + 1e-9);
        (assign15750_e23112, ((locals.var_kstress_u0_dn0 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn0))), ((locals.var_kstress_u0_dn2 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn2))), ((locals.var_kstress_u0_dn3 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn3))), ((locals.var_kstress_u0_dn4 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn4))), ((locals.var_kstress_u0_dn5 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn5))), ((locals.var_kstress_u0_dn6 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn6))), ((locals.var_kstress_u0_dn7 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn7))), ((locals.var_kstress_u0_dn8 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn8))), ((locals.var_kstress_u0_dn9 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn9))), ((locals.var_kstress_u0_dn10 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn10))), ((locals.var_kstress_u0_dn11 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn11))), ((locals.var_kstress_u0_dn12 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn12))), ((locals.var_kstress_u0_dn13 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn13))), ((locals.var_kstress_u0_dn14 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn14))),)
    } else {
        (locals.var_ku0_temp, locals.var_ku0_temp_dn0, locals.var_ku0_temp_dn2, locals.var_ku0_temp_dn3, locals.var_ku0_temp_dn4, locals.var_ku0_temp_dn5, locals.var_ku0_temp_dn6, locals.var_ku0_temp_dn7, locals.var_ku0_temp_dn8, locals.var_ku0_temp_dn9, locals.var_ku0_temp_dn10, locals.var_ku0_temp_dn11, locals.var_ku0_temp_dn12, locals.var_ku0_temp_dn13, locals.var_ku0_temp_dn14,)
    }
};
        locals.var_ku0_temp = assign15750_e23114;
        locals.var_ku0_temp_dn0 = assign15750_e23114_d_n0;
        locals.var_ku0_temp_dn2 = assign15750_e23114_d_n2;
        locals.var_ku0_temp_dn3 = assign15750_e23114_d_n3;
        locals.var_ku0_temp_dn4 = assign15750_e23114_d_n4;
        locals.var_ku0_temp_dn5 = assign15750_e23114_d_n5;
        locals.var_ku0_temp_dn6 = assign15750_e23114_d_n6;
        locals.var_ku0_temp_dn7 = assign15750_e23114_d_n7;
        locals.var_ku0_temp_dn8 = assign15750_e23114_d_n8;
        locals.var_ku0_temp_dn9 = assign15750_e23114_d_n9;
        locals.var_ku0_temp_dn10 = assign15750_e23114_d_n10;
        locals.var_ku0_temp_dn11 = assign15750_e23114_d_n11;
        locals.var_ku0_temp_dn12 = assign15750_e23114_d_n12;
        locals.var_ku0_temp_dn13 = assign15750_e23114_d_n13;
        locals.var_ku0_temp_dn14 = assign15750_e23114_d_n14;
        locals.var_ku0_temp_rv = 0.0;

        let (assign15760_e23118,) = {
    if (locals.var_guard488 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign15760_e23118;
        locals.var_i_rv = 0.0;

        let mut assign15770_loop_guard: usize = 0;
        while {
            let assign15770_cond_e23123: f64 = if ((locals.var_guard488 != 0.0) && (locals.var_i < p.p2)) { 1.0 } else { 0.0 };
            assign15770_cond_e23123 != 0.0
        } {
            assign15770_loop_guard += 1;
            assert!(assign15770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15770_body0_e23141, assign15770_body0_e23141_d_n0, assign15770_body0_e23141_d_n2, assign15770_body0_e23141_d_n3, assign15770_body0_e23141_d_n4, assign15770_body0_e23141_d_n5, assign15770_body0_e23141_d_n6, assign15770_body0_e23141_d_n7, assign15770_body0_e23141_d_n8, assign15770_body0_e23141_d_n9, assign15770_body0_e23141_d_n10, assign15770_body0_e23141_d_n11, assign15770_body0_e23141_d_n12, assign15770_body0_e23141_d_n13, assign15770_body0_e23141_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15770_body0_e23127: f64 = (1.0 / p.p2);
        let assign15770_body0_e23131: f64 = (0.5 * locals.var_l_mult);
        let assign15770_body0_e23132: f64 = (p.p17 + assign15770_body0_e23131);
        let assign15770_body0_e23136: f64 = (p.p19 + locals.var_l_mult);
        let assign15770_body0_e23137: f64 = (locals.var_i * assign15770_body0_e23136);
        let assign15770_body0_e23138: f64 = (assign15770_body0_e23132 + assign15770_body0_e23137);
        let assign15770_body0_e23139: f64 = (assign15770_body0_e23127 / assign15770_body0_e23138);
        (assign15770_body0_e23139, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign15770_body0_e23141;
            locals.var_t0_dn0 = assign15770_body0_e23141_d_n0;
            locals.var_t0_dn2 = assign15770_body0_e23141_d_n2;
            locals.var_t0_dn3 = assign15770_body0_e23141_d_n3;
            locals.var_t0_dn4 = assign15770_body0_e23141_d_n4;
            locals.var_t0_dn5 = assign15770_body0_e23141_d_n5;
            locals.var_t0_dn6 = assign15770_body0_e23141_d_n6;
            locals.var_t0_dn7 = assign15770_body0_e23141_d_n7;
            locals.var_t0_dn8 = assign15770_body0_e23141_d_n8;
            locals.var_t0_dn9 = assign15770_body0_e23141_d_n9;
            locals.var_t0_dn10 = assign15770_body0_e23141_d_n10;
            locals.var_t0_dn11 = assign15770_body0_e23141_d_n11;
            locals.var_t0_dn12 = assign15770_body0_e23141_d_n12;
            locals.var_t0_dn13 = assign15770_body0_e23141_d_n13;
            locals.var_t0_dn14 = assign15770_body0_e23141_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign15770_body1_e23159, assign15770_body1_e23159_d_n0, assign15770_body1_e23159_d_n2, assign15770_body1_e23159_d_n3, assign15770_body1_e23159_d_n4, assign15770_body1_e23159_d_n5, assign15770_body1_e23159_d_n6, assign15770_body1_e23159_d_n7, assign15770_body1_e23159_d_n8, assign15770_body1_e23159_d_n9, assign15770_body1_e23159_d_n10, assign15770_body1_e23159_d_n11, assign15770_body1_e23159_d_n12, assign15770_body1_e23159_d_n13, assign15770_body1_e23159_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15770_body1_e23145: f64 = (1.0 / p.p2);
        let assign15770_body1_e23149: f64 = (0.5 * locals.var_l_mult);
        let assign15770_body1_e23150: f64 = (p.p18 + assign15770_body1_e23149);
        let assign15770_body1_e23154: f64 = (p.p19 + locals.var_l_mult);
        let assign15770_body1_e23155: f64 = (locals.var_i * assign15770_body1_e23154);
        let assign15770_body1_e23156: f64 = (assign15770_body1_e23150 + assign15770_body1_e23155);
        let assign15770_body1_e23157: f64 = (assign15770_body1_e23145 / assign15770_body1_e23156);
        (assign15770_body1_e23157, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign15770_body1_e23159;
            locals.var_t1_dn0 = assign15770_body1_e23159_d_n0;
            locals.var_t1_dn2 = assign15770_body1_e23159_d_n2;
            locals.var_t1_dn3 = assign15770_body1_e23159_d_n3;
            locals.var_t1_dn4 = assign15770_body1_e23159_d_n4;
            locals.var_t1_dn5 = assign15770_body1_e23159_d_n5;
            locals.var_t1_dn6 = assign15770_body1_e23159_d_n6;
            locals.var_t1_dn7 = assign15770_body1_e23159_d_n7;
            locals.var_t1_dn8 = assign15770_body1_e23159_d_n8;
            locals.var_t1_dn9 = assign15770_body1_e23159_d_n9;
            locals.var_t1_dn10 = assign15770_body1_e23159_d_n10;
            locals.var_t1_dn11 = assign15770_body1_e23159_d_n11;
            locals.var_t1_dn12 = assign15770_body1_e23159_d_n12;
            locals.var_t1_dn13 = assign15770_body1_e23159_d_n13;
            locals.var_t1_dn14 = assign15770_body1_e23159_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign15770_body2_e23165, assign15770_body2_e23165_d_n0, assign15770_body2_e23165_d_n2, assign15770_body2_e23165_d_n3, assign15770_body2_e23165_d_n4, assign15770_body2_e23165_d_n5, assign15770_body2_e23165_d_n6, assign15770_body2_e23165_d_n7, assign15770_body2_e23165_d_n8, assign15770_body2_e23165_d_n9, assign15770_body2_e23165_d_n10, assign15770_body2_e23165_d_n11, assign15770_body2_e23165_d_n12, assign15770_body2_e23165_d_n13, assign15770_body2_e23165_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15770_body2_e23163: f64 = (locals.var_inv_sa + locals.var_t0);
        (assign15770_body2_e23163, (locals.var_inv_sa_dn0 + locals.var_t0_dn0), (locals.var_inv_sa_dn2 + locals.var_t0_dn2), (locals.var_inv_sa_dn3 + locals.var_t0_dn3), (locals.var_inv_sa_dn4 + locals.var_t0_dn4), (locals.var_inv_sa_dn5 + locals.var_t0_dn5), (locals.var_inv_sa_dn6 + locals.var_t0_dn6), (locals.var_inv_sa_dn7 + locals.var_t0_dn7), (locals.var_inv_sa_dn8 + locals.var_t0_dn8), (locals.var_inv_sa_dn9 + locals.var_t0_dn9), (locals.var_inv_sa_dn10 + locals.var_t0_dn10), (locals.var_inv_sa_dn11 + locals.var_t0_dn11), (locals.var_inv_sa_dn12 + locals.var_t0_dn12), (locals.var_inv_sa_dn13 + locals.var_t0_dn13), (locals.var_inv_sa_dn14 + locals.var_t0_dn14),)
    } else {
        (locals.var_inv_sa, locals.var_inv_sa_dn0, locals.var_inv_sa_dn2, locals.var_inv_sa_dn3, locals.var_inv_sa_dn4, locals.var_inv_sa_dn5, locals.var_inv_sa_dn6, locals.var_inv_sa_dn7, locals.var_inv_sa_dn8, locals.var_inv_sa_dn9, locals.var_inv_sa_dn10, locals.var_inv_sa_dn11, locals.var_inv_sa_dn12, locals.var_inv_sa_dn13, locals.var_inv_sa_dn14,)
    }
};
            locals.var_inv_sa = assign15770_body2_e23165;
            locals.var_inv_sa_dn0 = assign15770_body2_e23165_d_n0;
            locals.var_inv_sa_dn2 = assign15770_body2_e23165_d_n2;
            locals.var_inv_sa_dn3 = assign15770_body2_e23165_d_n3;
            locals.var_inv_sa_dn4 = assign15770_body2_e23165_d_n4;
            locals.var_inv_sa_dn5 = assign15770_body2_e23165_d_n5;
            locals.var_inv_sa_dn6 = assign15770_body2_e23165_d_n6;
            locals.var_inv_sa_dn7 = assign15770_body2_e23165_d_n7;
            locals.var_inv_sa_dn8 = assign15770_body2_e23165_d_n8;
            locals.var_inv_sa_dn9 = assign15770_body2_e23165_d_n9;
            locals.var_inv_sa_dn10 = assign15770_body2_e23165_d_n10;
            locals.var_inv_sa_dn11 = assign15770_body2_e23165_d_n11;
            locals.var_inv_sa_dn12 = assign15770_body2_e23165_d_n12;
            locals.var_inv_sa_dn13 = assign15770_body2_e23165_d_n13;
            locals.var_inv_sa_dn14 = assign15770_body2_e23165_d_n14;
            locals.var_inv_sa_rv = 0.0;
            let (assign15770_body3_e23171, assign15770_body3_e23171_d_n0, assign15770_body3_e23171_d_n2, assign15770_body3_e23171_d_n3, assign15770_body3_e23171_d_n4, assign15770_body3_e23171_d_n5, assign15770_body3_e23171_d_n6, assign15770_body3_e23171_d_n7, assign15770_body3_e23171_d_n8, assign15770_body3_e23171_d_n9, assign15770_body3_e23171_d_n10, assign15770_body3_e23171_d_n11, assign15770_body3_e23171_d_n12, assign15770_body3_e23171_d_n13, assign15770_body3_e23171_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15770_body3_e23169: f64 = (locals.var_inv_sb + locals.var_t1);
        (assign15770_body3_e23169, (locals.var_inv_sb_dn0 + locals.var_t1_dn0), (locals.var_inv_sb_dn2 + locals.var_t1_dn2), (locals.var_inv_sb_dn3 + locals.var_t1_dn3), (locals.var_inv_sb_dn4 + locals.var_t1_dn4), (locals.var_inv_sb_dn5 + locals.var_t1_dn5), (locals.var_inv_sb_dn6 + locals.var_t1_dn6), (locals.var_inv_sb_dn7 + locals.var_t1_dn7), (locals.var_inv_sb_dn8 + locals.var_t1_dn8), (locals.var_inv_sb_dn9 + locals.var_t1_dn9), (locals.var_inv_sb_dn10 + locals.var_t1_dn10), (locals.var_inv_sb_dn11 + locals.var_t1_dn11), (locals.var_inv_sb_dn12 + locals.var_t1_dn12), (locals.var_inv_sb_dn13 + locals.var_t1_dn13), (locals.var_inv_sb_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_inv_sb, locals.var_inv_sb_dn0, locals.var_inv_sb_dn2, locals.var_inv_sb_dn3, locals.var_inv_sb_dn4, locals.var_inv_sb_dn5, locals.var_inv_sb_dn6, locals.var_inv_sb_dn7, locals.var_inv_sb_dn8, locals.var_inv_sb_dn9, locals.var_inv_sb_dn10, locals.var_inv_sb_dn11, locals.var_inv_sb_dn12, locals.var_inv_sb_dn13, locals.var_inv_sb_dn14,)
    }
};
            locals.var_inv_sb = assign15770_body3_e23171;
            locals.var_inv_sb_dn0 = assign15770_body3_e23171_d_n0;
            locals.var_inv_sb_dn2 = assign15770_body3_e23171_d_n2;
            locals.var_inv_sb_dn3 = assign15770_body3_e23171_d_n3;
            locals.var_inv_sb_dn4 = assign15770_body3_e23171_d_n4;
            locals.var_inv_sb_dn5 = assign15770_body3_e23171_d_n5;
            locals.var_inv_sb_dn6 = assign15770_body3_e23171_d_n6;
            locals.var_inv_sb_dn7 = assign15770_body3_e23171_d_n7;
            locals.var_inv_sb_dn8 = assign15770_body3_e23171_d_n8;
            locals.var_inv_sb_dn9 = assign15770_body3_e23171_d_n9;
            locals.var_inv_sb_dn10 = assign15770_body3_e23171_d_n10;
            locals.var_inv_sb_dn11 = assign15770_body3_e23171_d_n11;
            locals.var_inv_sb_dn12 = assign15770_body3_e23171_d_n12;
            locals.var_inv_sb_dn13 = assign15770_body3_e23171_d_n13;
            locals.var_inv_sb_dn14 = assign15770_body3_e23171_d_n14;
            locals.var_inv_sb_rv = 0.0;
            let (assign15770_body4_e23177,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15770_body4_e23175: f64 = (locals.var_i + 1.0);
        (assign15770_body4_e23175,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign15770_body4_e23177;
            locals.var_i_rv = 0.0;
        }

        let (assign15780_e23187,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15780_e23183: f64 = (0.5 * locals.var_l_mult);
        let assign15780_e23184: f64 = (p.p912 + assign15780_e23183);
        let assign15780_e23185: f64 = (1.0 / assign15780_e23184);
        (assign15780_e23185,)
    } else {
        (locals.var_inv_saref,)
    }
};
        locals.var_inv_saref = assign15780_e23187;
        locals.var_inv_saref_rv = 0.0;

        let (assign15790_e23197,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15790_e23193: f64 = (0.5 * locals.var_l_mult);
        let assign15790_e23194: f64 = (p.p913 + assign15790_e23193);
        let assign15790_e23195: f64 = (1.0 / assign15790_e23194);
        (assign15790_e23195,)
    } else {
        (locals.var_inv_sbref,)
    }
};
        locals.var_inv_sbref = assign15790_e23197;
        locals.var_inv_sbref_rv = 0.0;

        let (assign15800_e23203,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15800_e23201: f64 = (locals.var_inv_saref + locals.var_inv_sbref);
        (assign15800_e23201,)
    } else {
        (locals.var_inv_odref,)
    }
};
        locals.var_inv_odref = assign15800_e23203;
        locals.var_inv_odref_rv = 0.0;

    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign10470_e14050: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard301 = assign10470_e14050;

        let assign10480_e14061: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard302 = assign10480_e14061;

        let assign10490_e14064: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard303 = assign10490_e14064;

        let (assign10500_e14085,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard303 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10500_e14085;

        let (assign10510_e14113,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard303 == 0.0)) {
        let assign10510_e14107: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10510_e14110: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10510_e14111: f64 = (assign10510_e14107 / assign10510_e14110);
        (assign10510_e14111,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10510_e14113;

        let assign10530_e14123: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard305 = assign10530_e14123;

        let (assign10540_e14147,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && ((locals.var_guard302 != 0.0) && (locals.var_guard301 == 0.0))) && (locals.var_guard305 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10540_e14147;

        let (assign10550_e14180,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && ((locals.var_guard302 != 0.0) && (locals.var_guard301 == 0.0))) && (locals.var_guard305 == 0.0)) {
        let assign10550_e14172: f64 = (p.p438 * locals.var_weff);
        let assign10550_e14175: f64 = (6.0 * locals.var_nuendd);
        let assign10550_e14177: f64 = (assign10550_e14175 * locals.var_dmcgeff);
        let assign10550_e14178: f64 = (assign10550_e14172 / assign10550_e14177);
        (assign10550_e14178,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10550_e14180;

        let (assign10560_e14202,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && (!((locals.var_guard301 != 0.0) || (locals.var_guard302 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10560_e14202;

        let assign10570_e14213: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard306 = assign10570_e14213;

        let assign10580_e14224: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard307 = assign10580_e14224;

        let assign10590_e14227: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard308 = assign10590_e14227;

        let (assign10600_e14249,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard306 != 0.0)) && (locals.var_guard308 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10600_e14249;

        let (assign10610_e14278,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard306 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign10610_e14272: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10610_e14275: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10610_e14276: f64 = (assign10610_e14272 / assign10610_e14275);
        (assign10610_e14276,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10610_e14278;

        let assign10630_e14288: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard310 = assign10630_e14288;

        let (assign10640_e14313,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && ((locals.var_guard307 != 0.0) && (locals.var_guard306 == 0.0))) && (locals.var_guard310 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10640_e14313;

        let (assign10650_e14347,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && ((locals.var_guard307 != 0.0) && (locals.var_guard306 == 0.0))) && (locals.var_guard310 == 0.0)) {
        let assign10650_e14339: f64 = (p.p438 * locals.var_weff);
        let assign10650_e14342: f64 = (6.0 * locals.var_nuendd);
        let assign10650_e14344: f64 = (assign10650_e14342 * locals.var_dmcgeff);
        let assign10650_e14345: f64 = (assign10650_e14339 / assign10650_e14344);
        (assign10650_e14345,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10650_e14347;

        let (assign10660_e14370,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && (!((locals.var_guard306 != 0.0) || (locals.var_guard307 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10660_e14370;

        let assign10670_e14373: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign10670_e14373;

        let assign10680_e14376: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign10680_e14376;

        let assign10690_e14387: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard313 = assign10690_e14387;

        let assign10700_e14398: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard314 = assign10700_e14398;

        let assign10710_e14401: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign10710_e14401;

        let (assign10720_e14423,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard315 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10720_e14423;

        let (assign10730_e14452,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign10730_e14446: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10730_e14449: f64 = (locals.var_weff * locals.var_nuends);
        let assign10730_e14450: f64 = (assign10730_e14446 / assign10730_e14449);
        (assign10730_e14450,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10730_e14452;

        let assign10750_e14462: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard317 = assign10750_e14462;

        let (assign10760_e14487,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && ((locals.var_guard314 != 0.0) && (locals.var_guard313 == 0.0))) && (locals.var_guard317 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10760_e14487;

        let (assign10770_e14521,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && ((locals.var_guard314 != 0.0) && (locals.var_guard313 == 0.0))) && (locals.var_guard317 == 0.0)) {
        let assign10770_e14513: f64 = (p.p438 * locals.var_weff);
        let assign10770_e14516: f64 = (6.0 * locals.var_nuends);
        let assign10770_e14518: f64 = (assign10770_e14516 * locals.var_dmcgeff);
        let assign10770_e14519: f64 = (assign10770_e14513 / assign10770_e14518);
        (assign10770_e14519,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10770_e14521;

        let (assign10780_e14544,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (!((locals.var_guard313 != 0.0) || (locals.var_guard314 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10780_e14544;

        let assign10790_e14555: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard318 = assign10790_e14555;

        let assign10800_e14566: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard319 = assign10800_e14566;

        let assign10810_e14569: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard320 = assign10810_e14569;

        let (assign10820_e14592,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10820_e14592;

        let (assign10830_e14622,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign10830_e14616: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10830_e14619: f64 = (locals.var_weff * locals.var_nuends);
        let assign10830_e14620: f64 = (assign10830_e14616 / assign10830_e14619);
        (assign10830_e14620,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10830_e14622;

        let assign10850_e14632: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard322 = assign10850_e14632;

        let (assign10860_e14658,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && ((locals.var_guard319 != 0.0) && (locals.var_guard318 == 0.0))) && (locals.var_guard322 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10860_e14658;

        let (assign10870_e14693,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && ((locals.var_guard319 != 0.0) && (locals.var_guard318 == 0.0))) && (locals.var_guard322 == 0.0)) {
        let assign10870_e14685: f64 = (p.p438 * locals.var_weff);
        let assign10870_e14688: f64 = (6.0 * locals.var_nuends);
        let assign10870_e14690: f64 = (assign10870_e14688 * locals.var_dmcgeff);
        let assign10870_e14691: f64 = (assign10870_e14685 / assign10870_e14690);
        (assign10870_e14691,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10870_e14693;

        let (assign10880_e14717,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && (!((locals.var_guard318 != 0.0) || (locals.var_guard319 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10880_e14717;

        let assign10890_e14720: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard323 = assign10890_e14720;

        let assign10900_e14731: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard324 = assign10900_e14731;

        let assign10910_e14742: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard325 = assign10910_e14742;

        let assign10920_e14745: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard326 = assign10920_e14745;

        let (assign10930_e14768,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && (locals.var_guard324 != 0.0)) && (locals.var_guard326 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10930_e14768;

        let (assign10940_e14798,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && (locals.var_guard324 != 0.0)) && (locals.var_guard326 == 0.0)) {
        let assign10940_e14792: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10940_e14795: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10940_e14796: f64 = (assign10940_e14792 / assign10940_e14795);
        (assign10940_e14796,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10940_e14798;

        let assign10960_e14809: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10960_e14812: f64 = if ((locals.var_nuendd == 0.0) || (assign10960_e14809 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard328 = assign10960_e14812;

        let (assign10970_e14838,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && ((locals.var_guard325 != 0.0) && (locals.var_guard324 == 0.0))) && (locals.var_guard328 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10970_e14838;

        let (assign10980_e14875,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && ((locals.var_guard325 != 0.0) && (locals.var_guard324 == 0.0))) && (locals.var_guard328 == 0.0)) {
        let assign10980_e14865: f64 = (p.p438 * locals.var_weff);
        let assign10980_e14868: f64 = (3.0 * locals.var_nuendd);
        let assign10980_e14871: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10980_e14872: f64 = (assign10980_e14868 * assign10980_e14871);
        let assign10980_e14873: f64 = (assign10980_e14865 / assign10980_e14872);
        (assign10980_e14873,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10980_e14875;

        let (assign10990_e14899,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && (!((locals.var_guard324 != 0.0) || (locals.var_guard325 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10990_e14899;

        let assign11000_e14910: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard329 = assign11000_e14910;

        let assign11010_e14921: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard330 = assign11010_e14921;

        let assign11020_e14924: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard331 = assign11020_e14924;

        let (assign11030_e14948,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && (locals.var_guard329 != 0.0)) && (locals.var_guard331 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11030_e14948;

        let (assign11040_e14979,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && (locals.var_guard329 != 0.0)) && (locals.var_guard331 == 0.0)) {
        let assign11040_e14973: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11040_e14976: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11040_e14977: f64 = (assign11040_e14973 / assign11040_e14976);
        (assign11040_e14977,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11040_e14979;

        let assign11060_e14990: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11060_e14993: f64 = if ((locals.var_nuendd == 0.0) || (assign11060_e14990 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard333 = assign11060_e14993;

        let (assign11070_e15020,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && ((locals.var_guard330 != 0.0) && (locals.var_guard329 == 0.0))) && (locals.var_guard333 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11070_e15020;

        let (assign11080_e15058,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && ((locals.var_guard330 != 0.0) && (locals.var_guard329 == 0.0))) && (locals.var_guard333 == 0.0)) {
        let assign11080_e15048: f64 = (p.p438 * locals.var_weff);
        let assign11080_e15051: f64 = (3.0 * locals.var_nuendd);
        let assign11080_e15054: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11080_e15055: f64 = (assign11080_e15051 * assign11080_e15054);
        let assign11080_e15056: f64 = (assign11080_e15048 / assign11080_e15055);
        (assign11080_e15056,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11080_e15058;

        let (assign11090_e15083,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && (!((locals.var_guard329 != 0.0) || (locals.var_guard330 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11090_e15083;

        let assign11100_e15086: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard334 = assign11100_e15086;

        let assign11110_e15089: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard335 = assign11110_e15089;

        let assign11120_e15100: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard336 = assign11120_e15100;

        let assign11130_e15111: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard337 = assign11130_e15111;

        let assign11140_e15114: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard338 = assign11140_e15114;

        let (assign11150_e15138,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard338 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11150_e15138;

        let (assign11160_e15169,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign11160_e15163: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11160_e15166: f64 = (locals.var_weff * locals.var_nuends);
        let assign11160_e15167: f64 = (assign11160_e15163 / assign11160_e15166);
        (assign11160_e15167,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11160_e15169;

        let assign11180_e15179: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard340 = assign11180_e15179;

        let (assign11190_e15206,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && ((locals.var_guard337 != 0.0) && (locals.var_guard336 == 0.0))) && (locals.var_guard340 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11190_e15206;

        let (assign11200_e15242,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && ((locals.var_guard337 != 0.0) && (locals.var_guard336 == 0.0))) && (locals.var_guard340 == 0.0)) {
        let assign11200_e15234: f64 = (p.p438 * locals.var_weff);
        let assign11200_e15237: f64 = (6.0 * locals.var_nuends);
        let assign11200_e15239: f64 = (assign11200_e15237 * locals.var_dmcgeff);
        let assign11200_e15240: f64 = (assign11200_e15234 / assign11200_e15239);
        (assign11200_e15240,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11200_e15242;

        let (assign11210_e15267,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (!((locals.var_guard336 != 0.0) || (locals.var_guard337 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11210_e15267;

        let assign11220_e15278: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard341 = assign11220_e15278;

        let assign11230_e15289: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard342 = assign11230_e15289;

        let assign11240_e15292: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard343 = assign11240_e15292;

        let (assign11250_e15317,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && (locals.var_guard341 != 0.0)) && (locals.var_guard343 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11250_e15317;

        let (assign11260_e15349,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && (locals.var_guard341 != 0.0)) && (locals.var_guard343 == 0.0)) {
        let assign11260_e15343: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11260_e15346: f64 = (locals.var_weff * locals.var_nuends);
        let assign11260_e15347: f64 = (assign11260_e15343 / assign11260_e15346);
        (assign11260_e15347,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11260_e15349;

        let assign11280_e15359: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard345 = assign11280_e15359;

        let (assign11290_e15387,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && ((locals.var_guard342 != 0.0) && (locals.var_guard341 == 0.0))) && (locals.var_guard345 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11290_e15387;

    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11300_e15424,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && ((locals.var_guard342 != 0.0) && (locals.var_guard341 == 0.0))) && (locals.var_guard345 == 0.0)) {
        let assign11300_e15416: f64 = (p.p438 * locals.var_weff);
        let assign11300_e15419: f64 = (6.0 * locals.var_nuends);
        let assign11300_e15421: f64 = (assign11300_e15419 * locals.var_dmcgeff);
        let assign11300_e15422: f64 = (assign11300_e15416 / assign11300_e15421);
        (assign11300_e15422,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11300_e15424;

        let (assign11310_e15450,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && (!((locals.var_guard341 != 0.0) || (locals.var_guard342 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11310_e15450;

        let assign11320_e15453: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard346 = assign11320_e15453;

        let assign11330_e15464: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard347 = assign11330_e15464;

        let assign11340_e15475: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard348 = assign11340_e15475;

        let assign11350_e15478: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard349 = assign11350_e15478;

        let (assign11360_e15503,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && (locals.var_guard347 != 0.0)) && (locals.var_guard349 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11360_e15503;

        let (assign11370_e15535,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && (locals.var_guard347 != 0.0)) && (locals.var_guard349 == 0.0)) {
        let assign11370_e15529: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11370_e15532: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11370_e15533: f64 = (assign11370_e15529 / assign11370_e15532);
        (assign11370_e15533,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11370_e15535;

        let assign11390_e15545: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard351 = assign11390_e15545;

        let (assign11400_e15573,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && ((locals.var_guard348 != 0.0) && (locals.var_guard347 == 0.0))) && (locals.var_guard351 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11400_e15573;

        let (assign11410_e15610,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && ((locals.var_guard348 != 0.0) && (locals.var_guard347 == 0.0))) && (locals.var_guard351 == 0.0)) {
        let assign11410_e15602: f64 = (p.p438 * locals.var_weff);
        let assign11410_e15605: f64 = (6.0 * locals.var_nuendd);
        let assign11410_e15607: f64 = (assign11410_e15605 * locals.var_dmcgeff);
        let assign11410_e15608: f64 = (assign11410_e15602 / assign11410_e15607);
        (assign11410_e15608,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11410_e15610;

        let (assign11420_e15636,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && (!((locals.var_guard347 != 0.0) || (locals.var_guard348 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11420_e15636;

        let assign11430_e15647: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard352 = assign11430_e15647;

        let assign11440_e15658: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard353 = assign11440_e15658;

        let assign11450_e15661: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard354 = assign11450_e15661;

        let (assign11460_e15687,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && (locals.var_guard352 != 0.0)) && (locals.var_guard354 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11460_e15687;

        let (assign11470_e15720,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && (locals.var_guard352 != 0.0)) && (locals.var_guard354 == 0.0)) {
        let assign11470_e15714: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11470_e15717: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11470_e15718: f64 = (assign11470_e15714 / assign11470_e15717);
        (assign11470_e15718,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11470_e15720;

        let assign11490_e15730: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard356 = assign11490_e15730;

        let (assign11500_e15759,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && ((locals.var_guard353 != 0.0) && (locals.var_guard352 == 0.0))) && (locals.var_guard356 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11500_e15759;

        let (assign11510_e15797,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && ((locals.var_guard353 != 0.0) && (locals.var_guard352 == 0.0))) && (locals.var_guard356 == 0.0)) {
        let assign11510_e15789: f64 = (p.p438 * locals.var_weff);
        let assign11510_e15792: f64 = (6.0 * locals.var_nuendd);
        let assign11510_e15794: f64 = (assign11510_e15792 * locals.var_dmcgeff);
        let assign11510_e15795: f64 = (assign11510_e15789 / assign11510_e15794);
        (assign11510_e15795,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11510_e15797;

        let (assign11520_e15824,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && (!((locals.var_guard352 != 0.0) || (locals.var_guard353 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11520_e15824;

        let assign11530_e15827: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard357 = assign11530_e15827;

        let assign11540_e15830: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard358 = assign11540_e15830;

        let assign11550_e15841: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard359 = assign11550_e15841;

        let assign11560_e15852: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard360 = assign11560_e15852;

        let assign11570_e15855: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard361 = assign11570_e15855;

        let (assign11580_e15881,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard359 != 0.0)) && (locals.var_guard361 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11580_e15881;

        let (assign11590_e15914,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard359 != 0.0)) && (locals.var_guard361 == 0.0)) {
        let assign11590_e15908: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11590_e15911: f64 = (locals.var_weff * locals.var_nuends);
        let assign11590_e15912: f64 = (assign11590_e15908 / assign11590_e15911);
        (assign11590_e15912,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11590_e15914;

        let assign11610_e15925: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11610_e15928: f64 = if ((locals.var_nuends == 0.0) || (assign11610_e15925 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard363 = assign11610_e15928;

        let (assign11620_e15957,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && ((locals.var_guard360 != 0.0) && (locals.var_guard359 == 0.0))) && (locals.var_guard363 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11620_e15957;

        let (assign11630_e15997,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && ((locals.var_guard360 != 0.0) && (locals.var_guard359 == 0.0))) && (locals.var_guard363 == 0.0)) {
        let assign11630_e15987: f64 = (p.p438 * locals.var_weff);
        let assign11630_e15990: f64 = (3.0 * locals.var_nuends);
        let assign11630_e15993: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11630_e15994: f64 = (assign11630_e15990 * assign11630_e15993);
        let assign11630_e15995: f64 = (assign11630_e15987 / assign11630_e15994);
        (assign11630_e15995,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11630_e15997;

        let (assign11640_e16024,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (!((locals.var_guard359 != 0.0) || (locals.var_guard360 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11640_e16024;

        let assign11650_e16035: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard364 = assign11650_e16035;

        let assign11660_e16046: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard365 = assign11660_e16046;

        let assign11670_e16049: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard366 = assign11670_e16049;

        let (assign11680_e16076,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && (locals.var_guard364 != 0.0)) && (locals.var_guard366 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11680_e16076;

        let (assign11690_e16110,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && (locals.var_guard364 != 0.0)) && (locals.var_guard366 == 0.0)) {
        let assign11690_e16104: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11690_e16107: f64 = (locals.var_weff * locals.var_nuends);
        let assign11690_e16108: f64 = (assign11690_e16104 / assign11690_e16107);
        (assign11690_e16108,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11690_e16110;

        let assign11710_e16121: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11710_e16124: f64 = if ((locals.var_nuends == 0.0) || (assign11710_e16121 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard368 = assign11710_e16124;

        let (assign11720_e16154,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && ((locals.var_guard365 != 0.0) && (locals.var_guard364 == 0.0))) && (locals.var_guard368 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11720_e16154;

        let (assign11730_e16195,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && ((locals.var_guard365 != 0.0) && (locals.var_guard364 == 0.0))) && (locals.var_guard368 == 0.0)) {
        let assign11730_e16185: f64 = (p.p438 * locals.var_weff);
        let assign11730_e16188: f64 = (3.0 * locals.var_nuends);
        let assign11730_e16191: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11730_e16192: f64 = (assign11730_e16188 * assign11730_e16191);
        let assign11730_e16193: f64 = (assign11730_e16185 / assign11730_e16192);
        (assign11730_e16193,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11730_e16195;

        let (assign11740_e16223,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && (!((locals.var_guard364 != 0.0) || (locals.var_guard365 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11740_e16223;

        let (assign11750_e16248,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 == 0.0)) {
        let assign11750_e16244: f64 = (p.p438 * locals.var_dmdgeff);
        let assign11750_e16246: f64 = (assign11750_e16244 / locals.var_weff);
        (assign11750_e16246,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11750_e16248;

        let assign11760_e16251: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign11760_e16251;

        let assign11770_e16254: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard370 = assign11770_e16254;

        let assign11780_e16265: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard371 = assign11780_e16265;

        let assign11790_e16276: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard372 = assign11790_e16276;

        let assign11800_e16279: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard373 = assign11800_e16279;

        let (assign11810_e16307,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard371 != 0.0)) && (locals.var_guard373 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11810_e16307;

        let (assign11820_e16342,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard371 != 0.0)) && (locals.var_guard373 == 0.0)) {
        let assign11820_e16336: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11820_e16339: f64 = (locals.var_weff * locals.var_nuends);
        let assign11820_e16340: f64 = (assign11820_e16336 / assign11820_e16339);
        (assign11820_e16340,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11820_e16342;

        let assign11840_e16352: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard375 = assign11840_e16352;

        let (assign11850_e16383,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && ((locals.var_guard372 != 0.0) && (locals.var_guard371 == 0.0))) && (locals.var_guard375 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11850_e16383;

        let (assign11860_e16423,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && ((locals.var_guard372 != 0.0) && (locals.var_guard371 == 0.0))) && (locals.var_guard375 == 0.0)) {
        let assign11860_e16415: f64 = (p.p438 * locals.var_weff);
        let assign11860_e16418: f64 = (6.0 * locals.var_nuends);
        let assign11860_e16420: f64 = (assign11860_e16418 * locals.var_dmcgeff);
        let assign11860_e16421: f64 = (assign11860_e16415 / assign11860_e16420);
        (assign11860_e16421,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11860_e16423;

        let (assign11870_e16452,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (!((locals.var_guard371 != 0.0) || (locals.var_guard372 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11870_e16452;

        let assign11880_e16463: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard376 = assign11880_e16463;

        let assign11890_e16474: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard377 = assign11890_e16474;

        let assign11900_e16477: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard378 = assign11900_e16477;

        let (assign11910_e16506,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && (locals.var_guard376 != 0.0)) && (locals.var_guard378 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11910_e16506;

        let (assign11920_e16542,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && (locals.var_guard376 != 0.0)) && (locals.var_guard378 == 0.0)) {
        let assign11920_e16536: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11920_e16539: f64 = (locals.var_weff * locals.var_nuends);
        let assign11920_e16540: f64 = (assign11920_e16536 / assign11920_e16539);
        (assign11920_e16540,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11920_e16542;

        let assign11940_e16552: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard380 = assign11940_e16552;

        let (assign11950_e16584,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && ((locals.var_guard377 != 0.0) && (locals.var_guard376 == 0.0))) && (locals.var_guard380 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11950_e16584;

        let (assign11960_e16625,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && ((locals.var_guard377 != 0.0) && (locals.var_guard376 == 0.0))) && (locals.var_guard380 == 0.0)) {
        let assign11960_e16617: f64 = (p.p438 * locals.var_weff);
        let assign11960_e16620: f64 = (6.0 * locals.var_nuends);
        let assign11960_e16622: f64 = (assign11960_e16620 * locals.var_dmcgeff);
        let assign11960_e16623: f64 = (assign11960_e16617 / assign11960_e16622);
        (assign11960_e16623,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11960_e16625;

        let (assign11970_e16655,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && (!((locals.var_guard376 != 0.0) || (locals.var_guard377 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11970_e16655;

        let assign11980_e16658: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign11980_e16658;

        let (assign11990_e16683,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 == 0.0)) && (locals.var_guard381 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11990_e16683;

        let (assign12000_e16715,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 == 0.0)) && (locals.var_guard381 == 0.0)) {
        let assign12000_e16709: f64 = (p.p438 * locals.var_dmdgeff);
        let assign12000_e16712: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12000_e16713: f64 = (assign12000_e16709 / assign12000_e16712);
        (assign12000_e16713,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12000_e16715;

        let assign12010_e16718: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard382 = assign12010_e16718;

        let (assign12020_e16746,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 != 0.0)) {
        let assign12020_e16742: f64 = (p.p438 * locals.var_dmdgeff);
        let assign12020_e16744: f64 = (assign12020_e16742 / locals.var_weff);
        (assign12020_e16744,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12020_e16746;

        let assign12030_e16749: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard383 = assign12030_e16749;

        let assign12040_e16760: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard384 = assign12040_e16760;

        let assign12050_e16771: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard385 = assign12050_e16771;

        let assign12060_e16774: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard386 = assign12060_e16774;

        let (assign12070_e16805,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) && (locals.var_guard386 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12070_e16805;

        let (assign12080_e16843,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign12080_e16837: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12080_e16840: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12080_e16841: f64 = (assign12080_e16837 / assign12080_e16840);
        (assign12080_e16841,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12080_e16843;

        let assign12100_e16854: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign12100_e16857: f64 = if ((locals.var_nuendd == 0.0) || (assign12100_e16854 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard388 = assign12100_e16857;

    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12110_e16891,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && ((locals.var_guard385 != 0.0) && (locals.var_guard384 == 0.0))) && (locals.var_guard388 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12110_e16891;

        let (assign12120_e16936,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && ((locals.var_guard385 != 0.0) && (locals.var_guard384 == 0.0))) && (locals.var_guard388 == 0.0)) {
        let assign12120_e16926: f64 = (p.p438 * locals.var_weff);
        let assign12120_e16929: f64 = (3.0 * locals.var_nuendd);
        let assign12120_e16932: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign12120_e16933: f64 = (assign12120_e16929 * assign12120_e16932);
        let assign12120_e16934: f64 = (assign12120_e16926 / assign12120_e16933);
        (assign12120_e16934,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12120_e16936;

        let (assign12130_e16968,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && (!((locals.var_guard384 != 0.0) || (locals.var_guard385 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12130_e16968;

        let assign12140_e16979: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard389 = assign12140_e16979;

        let assign12150_e16990: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard390 = assign12150_e16990;

        let assign12160_e16993: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard391 = assign12160_e16993;

        let (assign12170_e17025,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (locals.var_guard389 != 0.0)) && (locals.var_guard391 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12170_e17025;

        let (assign12180_e17064,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (locals.var_guard389 != 0.0)) && (locals.var_guard391 == 0.0)) {
        let assign12180_e17058: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12180_e17061: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12180_e17062: f64 = (assign12180_e17058 / assign12180_e17061);
        (assign12180_e17062,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12180_e17064;

        let assign12200_e17075: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign12200_e17078: f64 = if ((locals.var_nuendd == 0.0) || (assign12200_e17075 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard393 = assign12200_e17078;

        let (assign12210_e17113,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && ((locals.var_guard390 != 0.0) && (locals.var_guard389 == 0.0))) && (locals.var_guard393 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12210_e17113;

        let (assign12220_e17159,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && ((locals.var_guard390 != 0.0) && (locals.var_guard389 == 0.0))) && (locals.var_guard393 == 0.0)) {
        let assign12220_e17149: f64 = (p.p438 * locals.var_weff);
        let assign12220_e17152: f64 = (3.0 * locals.var_nuendd);
        let assign12220_e17155: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign12220_e17156: f64 = (assign12220_e17152 * assign12220_e17155);
        let assign12220_e17157: f64 = (assign12220_e17149 / assign12220_e17156);
        (assign12220_e17157,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12220_e17159;

        let (assign12230_e17192,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (!((locals.var_guard389 != 0.0) || (locals.var_guard390 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12230_e17192;

        let assign12240_e17195: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign12240_e17195;

        let assign12250_e17198: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard395 = assign12250_e17198;

        let (assign12260_e17226,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12260_e17226;

        let (assign12270_e17261,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 == 0.0)) {
        let assign12270_e17255: f64 = (p.p438 * locals.var_dmdgeff);
        let assign12270_e17258: f64 = (locals.var_weff * locals.var_nuends);
        let assign12270_e17259: f64 = (assign12270_e17255 / assign12270_e17258);
        (assign12270_e17259,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12270_e17261;

        let assign12280_e17264: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard396 = assign12280_e17264;

        let assign12290_e17275: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard397 = assign12290_e17275;

        let assign12300_e17286: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard398 = assign12300_e17286;

        let assign12310_e17289: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard399 = assign12310_e17289;

        let (assign12320_e17322,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_guard399 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12320_e17322;

        let (assign12330_e17362,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_guard399 == 0.0)) {
        let assign12330_e17356: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12330_e17359: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12330_e17360: f64 = (assign12330_e17356 / assign12330_e17359);
        (assign12330_e17360,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12330_e17362;

        let assign12350_e17372: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard401 = assign12350_e17372;

        let (assign12360_e17408,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && ((locals.var_guard398 != 0.0) && (locals.var_guard397 == 0.0))) && (locals.var_guard401 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12360_e17408;

        let (assign12370_e17453,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && ((locals.var_guard398 != 0.0) && (locals.var_guard397 == 0.0))) && (locals.var_guard401 == 0.0)) {
        let assign12370_e17445: f64 = (p.p438 * locals.var_weff);
        let assign12370_e17448: f64 = (6.0 * locals.var_nuendd);
        let assign12370_e17450: f64 = (assign12370_e17448 * locals.var_dmcgeff);
        let assign12370_e17451: f64 = (assign12370_e17445 / assign12370_e17450);
        (assign12370_e17451,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12370_e17453;

        let (assign12380_e17487,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && (!((locals.var_guard397 != 0.0) || (locals.var_guard398 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12380_e17487;

        let assign12390_e17498: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard402 = assign12390_e17498;

        let assign12400_e17509: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard403 = assign12400_e17509;

        let assign12410_e17512: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard404 = assign12410_e17512;

        let (assign12420_e17546,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard402 != 0.0)) && (locals.var_guard404 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12420_e17546;

        let (assign12430_e17587,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard402 != 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12430_e17581: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12430_e17584: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12430_e17585: f64 = (assign12430_e17581 / assign12430_e17584);
        (assign12430_e17585,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12430_e17587;

        let assign12450_e17597: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard406 = assign12450_e17597;

        let (assign12460_e17634,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && ((locals.var_guard403 != 0.0) && (locals.var_guard402 == 0.0))) && (locals.var_guard406 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12460_e17634;

        let (assign12470_e17680,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && ((locals.var_guard403 != 0.0) && (locals.var_guard402 == 0.0))) && (locals.var_guard406 == 0.0)) {
        let assign12470_e17672: f64 = (p.p438 * locals.var_weff);
        let assign12470_e17675: f64 = (6.0 * locals.var_nuendd);
        let assign12470_e17677: f64 = (assign12470_e17675 * locals.var_dmcgeff);
        let assign12470_e17678: f64 = (assign12470_e17672 / assign12470_e17677);
        (assign12470_e17678,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12470_e17680;

        let (assign12480_e17715,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && (!((locals.var_guard402 != 0.0) || (locals.var_guard403 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12480_e17715;

        let (assign12490_e17745,) = {
    if (((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard262 != 0.0) && (!((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) {
        let assign12490_e17741: f64 = (p.p438 * locals.var_dmdgeff);
        let assign12490_e17743: f64 = (assign12490_e17741 / locals.var_weff);
        (assign12490_e17743,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12490_e17745;

        let assign12500_e17748: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign12500_e17748;

        let (assign12510_e17784,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 != 0.0)) {
        let assign12510_e17778: f64 = (0.5 * p.p438);
        let assign12510_e17780: f64 = (assign12510_e17778 * locals.var_dmcgeff);
        let assign12510_e17782: f64 = (assign12510_e17780 / locals.var_weff);
        (assign12510_e17782,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12510_e17784;

        let assign12520_e17787: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign12520_e17787;

        let (assign12530_e17819,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 != 0.0)) && (locals.var_guard408 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12530_e17819;

        let (assign12540_e17860,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 != 0.0)) && (locals.var_guard408 == 0.0)) {
        let assign12540_e17852: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12540_e17856: f64 = (p.p2 - 2.0);
        let assign12540_e17857: f64 = (locals.var_weff * assign12540_e17856);
        let assign12540_e17858: f64 = (assign12540_e17852 / assign12540_e17857);
        (assign12540_e17858,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12540_e17860;

        let (assign12550_e17891,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12550_e17891;

        let (assign12560_e17928,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 == 0.0)) {
        let assign12560_e17922: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12560_e17925: f64 = (locals.var_weff * p.p2);
        let assign12560_e17926: f64 = (assign12560_e17922 / assign12560_e17925);
        (assign12560_e17926,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12560_e17928;

        let assign12570_e17931: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard409 = assign12570_e17931;

        let (assign12580_e17963,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12580_e17963;

        let (assign12590_e18001,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 != 0.0)) {
        let assign12590_e17995: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12590_e17998: f64 = (locals.var_weff * p.p2);
        let assign12590_e17999: f64 = (assign12590_e17995 / assign12590_e17998);
        (assign12590_e17999,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12590_e18001;

        let (assign12600_e18040,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 == 0.0)) {
        let assign12600_e18034: f64 = (0.5 * p.p438);
        let assign12600_e18036: f64 = (assign12600_e18034 * locals.var_dmcgeff);
        let assign12600_e18038: f64 = (assign12600_e18036 / locals.var_weff);
        (assign12600_e18038,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12600_e18040;

        let assign12610_e18043: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign12610_e18043;

        let (assign12620_e18078,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12620_e18078;

        let (assign12630_e18122,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 == 0.0)) {
        let assign12630_e18114: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12630_e18118: f64 = (p.p2 - 2.0);
        let assign12630_e18119: f64 = (locals.var_weff * assign12630_e18118);
        let assign12630_e18120: f64 = (assign12630_e18114 / assign12630_e18119);
        (assign12630_e18120,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12630_e18122;

        let (assign12640_e18152,) = {
    if (((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (!(((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0)) || (locals.var_guard264 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12640_e18152;

        let assign12650_e18155: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign12650_e18155;

        let (assign12660_e18164,) = {
    if (((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard411 != 0.0)) {
        (locals.var_rend,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12660_e18164;

        let assign12670_e18167: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard412 = assign12670_e18167;

        let (assign12680_e18179,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 != 0.0)) {
        (locals.var_rint,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12680_e18179;

        let (assign12690_e18198,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 == 0.0)) {
        let assign12690_e18192: f64 = (locals.var_rint * locals.var_rend);
        let assign12690_e18195: f64 = (locals.var_rint + locals.var_rend);
        let assign12690_e18196: f64 = (assign12690_e18192 / assign12690_e18195);
        (assign12690_e18196,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12690_e18198;

        let (assign12710_e18209,) = {
    if ((locals.var_guard246 == 0.0) && (locals.var_guard247 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12710_e18209;

        let assign12720_e18212: f64 = if p.p33 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign12720_e18212;

        let assign12730_e18215: f64 = if locals.var_rsourcegeo < p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard415 = assign12730_e18215;

        let (assign12740_e18221,) = {
    if ((locals.var_guard414 != 0.0) && (locals.var_guard415 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign12740_e18221;

        let assign12750_e18224: f64 = if locals.var_rdraingeo < p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign12750_e18224;

        let (assign12760_e18230,) = {
    if ((locals.var_guard414 != 0.0) && (locals.var_guard416 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12760_e18230;

        let assign12770_e18233: f64 = if locals.var_rsourcegeo <= p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard417 = assign12770_e18233;

        let (assign12780_e18240,) = {
    if ((locals.var_guard414 == 0.0) && (locals.var_guard417 != 0.0)) {
        (p.p1347,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign12780_e18240;

        let assign12790_e18243: f64 = if locals.var_rdraingeo <= p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard418 = assign12790_e18243;

        let (assign12800_e18250,) = {
    if ((locals.var_guard414 == 0.0) && (locals.var_guard418 != 0.0)) {
        (p.p1347,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12800_e18250;

        let assign12810_e18253: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign12810_e18253;

        let assign12820_e18256: f64 = if locals.var_rswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard420 = assign12820_e18256;

        let (assign12830_e18262,) = {
    if ((locals.var_guard419 != 0.0) && (locals.var_guard420 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rswmin_i,)
    }
};
        locals.var_rswmin_i = assign12830_e18262;

        let assign12840_e18265: f64 = if locals.var_rdwmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign12840_e18265;

    }

    pub(super) fn stamp_transient_block_19(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (assign12850_e18271,) = {
    if ((locals.var_guard419 != 0.0) && (locals.var_guard421 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdwmin_i,)
    }
};
        locals.var_rdwmin_i = assign12850_e18271;

        let assign12860_e18274: f64 = if locals.var_rsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard422 = assign12860_e18274;

        let (assign12870_e18280,) = {
    if ((locals.var_guard419 != 0.0) && (locals.var_guard422 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsw_i,)
    }
};
        locals.var_rsw_i = assign12870_e18280;

        let assign12880_e18283: f64 = if locals.var_rdw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign12880_e18283;

        let (assign12890_e18289,) = {
    if ((locals.var_guard419 != 0.0) && (locals.var_guard423 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdw_i,)
    }
};
        locals.var_rdw_i = assign12890_e18289;

        let assign12900_e18292: f64 = if locals.var_rdswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard424 = assign12900_e18292;

        let (assign12910_e18299,) = {
    if ((locals.var_guard419 == 0.0) && (locals.var_guard424 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdswmin_i,)
    }
};
        locals.var_rdswmin_i = assign12910_e18299;

        let assign12920_e18302: f64 = if locals.var_rdsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard425 = assign12920_e18302;

        let (assign12930_e18309,) = {
    if ((locals.var_guard419 == 0.0) && (locals.var_guard425 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdsw_i,)
    }
};
        locals.var_rdsw_i = assign12930_e18309;

        let assign12940_e18314: f64 = (locals.var_weffcj / 3.0);
        let assign12940_e18316: f64 = (assign12940_e18314 / p.p22);
        let assign12940_e18317: f64 = (p.p21 + assign12940_e18316);
        let assign12940_e18318: f64 = (p.p900 * assign12940_e18317);
        let assign12940_e18321: f64 = (p.p22 * p.p2);
        let assign12940_e18324: f64 = (locals.var_lnew - p.p899);
        let assign12940_e18325: f64 = (assign12940_e18321 * assign12940_e18324);
        let assign12940_e18326: f64 = (assign12940_e18318 / assign12940_e18325);
        locals.var_grgeltd = assign12940_e18326;

        let assign12950_e18329: f64 = if locals.var_grgeltd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard426 = assign12950_e18329;

        let (assign12960_e18335,) = {
    if (locals.var_guard426 != 0.0) {
        let assign12960_e18333: f64 = (1.0 / locals.var_grgeltd);
        (assign12960_e18333,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign12960_e18335;

        let (assign12970_e18340,) = {
    if (locals.var_guard426 == 0.0) {
        (1000.0,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign12970_e18340;

        let assign12990_e18346: f64 = (p.p76 * p.p76);
        locals.var_t0 = assign12990_e18346;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;

        let assign13000_e18349: f64 = (p.p76 * locals.var_poxedge_i);
        locals.var_t1 = assign13000_e18349;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;

        let assign13010_e18352: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign13010_e18352;
        locals.var_t2_dn3 = ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_t2_dn7 = ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn9 = ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9));
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_t2_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));

        let assign13020_e18356: f64 = (p.p722 / p.p76);
        let assign13020_e18358: f64 = (assign13020_e18356).max(1e-38);
        let assign13020_e18359: f64 = (assign13020_e18358).ln();
        let assign13020_e18360: f64 = (locals.var_ntox_i * assign13020_e18359);
        let assign13020_e18361: f64 = { let limited_exp_arg = assign13020_e18360; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13020_e18363: f64 = (assign13020_e18361 / locals.var_t0);
        locals.var_toxratio = assign13020_e18363;
        locals.var_toxratio_dn3 = (-((assign13020_e18361 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn4 = (-((assign13020_e18361 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn5 = (-((assign13020_e18361 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn6 = (-((assign13020_e18361 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn7 = (-((assign13020_e18361 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn8 = (-((assign13020_e18361 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn9 = (-((assign13020_e18361 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn10 = (-((assign13020_e18361 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn11 = (-((assign13020_e18361 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)));

        let assign13030_e18367: f64 = (p.p722 / locals.var_t1);
        let assign13030_e18369: f64 = (assign13030_e18367).max(1e-38);
        let assign13030_e18370: f64 = (assign13030_e18369).ln();
        let assign13030_e18371: f64 = (locals.var_ntox_i * assign13030_e18370);
        let assign13030_e18372: f64 = { let limited_exp_arg = assign13030_e18371; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13030_e18374: f64 = (assign13030_e18372 / locals.var_t2);
        locals.var_toxratioedge = assign13030_e18374;
        locals.var_toxratioedge_dn3 = (((({ let limited_exp_arg = assign13030_e18371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign13030_e18367 >= 1e-38 { (-((p.p722 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign13030_e18369))) * locals.var_t2) - (assign13030_e18372 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn4 = (((({ let limited_exp_arg = assign13030_e18371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign13030_e18367 >= 1e-38 { (-((p.p722 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign13030_e18369))) * locals.var_t2) - (assign13030_e18372 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn5 = (((({ let limited_exp_arg = assign13030_e18371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign13030_e18367 >= 1e-38 { (-((p.p722 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign13030_e18369))) * locals.var_t2) - (assign13030_e18372 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn6 = (((({ let limited_exp_arg = assign13030_e18371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign13030_e18367 >= 1e-38 { (-((p.p722 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign13030_e18369))) * locals.var_t2) - (assign13030_e18372 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn7 = (((({ let limited_exp_arg = assign13030_e18371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign13030_e18367 >= 1e-38 { (-((p.p722 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign13030_e18369))) * locals.var_t2) - (assign13030_e18372 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn8 = (((({ let limited_exp_arg = assign13030_e18371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign13030_e18367 >= 1e-38 { (-((p.p722 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign13030_e18369))) * locals.var_t2) - (assign13030_e18372 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn9 = (((({ let limited_exp_arg = assign13030_e18371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign13030_e18367 >= 1e-38 { (-((p.p722 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign13030_e18369))) * locals.var_t2) - (assign13030_e18372 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn10 = (((({ let limited_exp_arg = assign13030_e18371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign13030_e18367 >= 1e-38 { (-((p.p722 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign13030_e18369))) * locals.var_t2) - (assign13030_e18372 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn11 = (((({ let limited_exp_arg = assign13030_e18371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign13030_e18367 >= 1e-38 { (-((p.p722 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign13030_e18369))) * locals.var_t2) - (assign13030_e18372 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2));

        let (assign13040_e18380,) = {
    if (p.p30 == 1.0) {
        (p.p703,)
    } else {
        (p.p702,)
    }
};
        locals.var_aechvb = assign13040_e18380;
        locals.var_aechvb_dn3 = 0.0;
        locals.var_aechvb_dn4 = 0.0;
        locals.var_aechvb_dn5 = 0.0;
        locals.var_aechvb_dn6 = 0.0;
        locals.var_aechvb_dn7 = 0.0;
        locals.var_aechvb_dn8 = 0.0;
        locals.var_aechvb_dn9 = 0.0;
        locals.var_aechvb_dn10 = 0.0;
        locals.var_aechvb_dn11 = 0.0;

        let (assign13050_e18386,) = {
    if (p.p30 == 1.0) {
        (p.p705,)
    } else {
        (p.p704,)
    }
};
        locals.var_bechvb = assign13050_e18386;

        let assign13060_e18390: f64 = (locals.var_weff / p.p1373);
        let assign13060_e18392: f64 = (assign13060_e18390 + p.p1378);
        let assign13060_e18393: f64 = (locals.var_aechvb * assign13060_e18392);
        let assign13060_e18395: f64 = (assign13060_e18393 * locals.var_toxratioedge);
        locals.var_aechvbedges = assign13060_e18395;
        locals.var_aechvbedges_dn3 = (((locals.var_aechvb_dn3 * assign13060_e18392) * locals.var_toxratioedge) + (assign13060_e18393 * locals.var_toxratioedge_dn3));
        locals.var_aechvbedges_dn4 = (((locals.var_aechvb_dn4 * assign13060_e18392) * locals.var_toxratioedge) + (assign13060_e18393 * locals.var_toxratioedge_dn4));
        locals.var_aechvbedges_dn5 = (((locals.var_aechvb_dn5 * assign13060_e18392) * locals.var_toxratioedge) + (assign13060_e18393 * locals.var_toxratioedge_dn5));
        locals.var_aechvbedges_dn6 = (((locals.var_aechvb_dn6 * assign13060_e18392) * locals.var_toxratioedge) + (assign13060_e18393 * locals.var_toxratioedge_dn6));
        locals.var_aechvbedges_dn7 = (((locals.var_aechvb_dn7 * assign13060_e18392) * locals.var_toxratioedge) + (assign13060_e18393 * locals.var_toxratioedge_dn7));
        locals.var_aechvbedges_dn8 = (((locals.var_aechvb_dn8 * assign13060_e18392) * locals.var_toxratioedge) + (assign13060_e18393 * locals.var_toxratioedge_dn8));
        locals.var_aechvbedges_dn9 = (((locals.var_aechvb_dn9 * assign13060_e18392) * locals.var_toxratioedge) + (assign13060_e18393 * locals.var_toxratioedge_dn9));
        locals.var_aechvbedges_dn10 = (((locals.var_aechvb_dn10 * assign13060_e18392) * locals.var_toxratioedge) + (assign13060_e18393 * locals.var_toxratioedge_dn10));
        locals.var_aechvbedges_dn11 = (((locals.var_aechvb_dn11 * assign13060_e18392) * locals.var_toxratioedge) + (assign13060_e18393 * locals.var_toxratioedge_dn11));

        let assign13070_e18399: f64 = (locals.var_weff / p.p1373);
        let assign13070_e18401: f64 = (assign13070_e18399 + p.p1377);
        let assign13070_e18402: f64 = (locals.var_aechvb * assign13070_e18401);
        let assign13070_e18404: f64 = (assign13070_e18402 * locals.var_toxratioedge);
        locals.var_aechvbedged = assign13070_e18404;
        locals.var_aechvbedged_dn3 = (((locals.var_aechvb_dn3 * assign13070_e18401) * locals.var_toxratioedge) + (assign13070_e18402 * locals.var_toxratioedge_dn3));
        locals.var_aechvbedged_dn4 = (((locals.var_aechvb_dn4 * assign13070_e18401) * locals.var_toxratioedge) + (assign13070_e18402 * locals.var_toxratioedge_dn4));
        locals.var_aechvbedged_dn5 = (((locals.var_aechvb_dn5 * assign13070_e18401) * locals.var_toxratioedge) + (assign13070_e18402 * locals.var_toxratioedge_dn5));
        locals.var_aechvbedged_dn6 = (((locals.var_aechvb_dn6 * assign13070_e18401) * locals.var_toxratioedge) + (assign13070_e18402 * locals.var_toxratioedge_dn6));
        locals.var_aechvbedged_dn7 = (((locals.var_aechvb_dn7 * assign13070_e18401) * locals.var_toxratioedge) + (assign13070_e18402 * locals.var_toxratioedge_dn7));
        locals.var_aechvbedged_dn8 = (((locals.var_aechvb_dn8 * assign13070_e18401) * locals.var_toxratioedge) + (assign13070_e18402 * locals.var_toxratioedge_dn8));
        locals.var_aechvbedged_dn9 = (((locals.var_aechvb_dn9 * assign13070_e18401) * locals.var_toxratioedge) + (assign13070_e18402 * locals.var_toxratioedge_dn9));
        locals.var_aechvbedged_dn10 = (((locals.var_aechvb_dn10 * assign13070_e18401) * locals.var_toxratioedge) + (assign13070_e18402 * locals.var_toxratioedge_dn10));
        locals.var_aechvbedged_dn11 = (((locals.var_aechvb_dn11 * assign13070_e18401) * locals.var_toxratioedge) + (assign13070_e18402 * locals.var_toxratioedge_dn11));

        let assign13080_e18406: f64 = (-locals.var_bechvb);
        let assign13080_e18408: f64 = (assign13080_e18406 * p.p76);
        let assign13080_e18410: f64 = (assign13080_e18408 * locals.var_poxedge_i);
        locals.var_bechvbedge = assign13080_e18410;

        let assign13090_e18414: f64 = (locals.var_weff / p.p1373);
        let assign13090_e18416: f64 = (assign13090_e18414 * locals.var_leff);
        let assign13090_e18419: f64 = (p.p1381 / p.p2);
        let assign13090_e18420: f64 = (assign13090_e18416 + assign13090_e18419);
        let assign13090_e18421: f64 = (locals.var_aechvb * assign13090_e18420);
        let assign13090_e18423: f64 = (assign13090_e18421 * locals.var_toxratio);
        locals.var_aechvb = assign13090_e18423;
        locals.var_aechvb_dn3 = (((locals.var_aechvb_dn3 * assign13090_e18420) * locals.var_toxratio) + (assign13090_e18421 * locals.var_toxratio_dn3));
        locals.var_aechvb_dn4 = (((locals.var_aechvb_dn4 * assign13090_e18420) * locals.var_toxratio) + (assign13090_e18421 * locals.var_toxratio_dn4));
        locals.var_aechvb_dn5 = (((locals.var_aechvb_dn5 * assign13090_e18420) * locals.var_toxratio) + (assign13090_e18421 * locals.var_toxratio_dn5));
        locals.var_aechvb_dn6 = (((locals.var_aechvb_dn6 * assign13090_e18420) * locals.var_toxratio) + (assign13090_e18421 * locals.var_toxratio_dn6));
        locals.var_aechvb_dn7 = (((locals.var_aechvb_dn7 * assign13090_e18420) * locals.var_toxratio) + (assign13090_e18421 * locals.var_toxratio_dn7));
        locals.var_aechvb_dn8 = (((locals.var_aechvb_dn8 * assign13090_e18420) * locals.var_toxratio) + (assign13090_e18421 * locals.var_toxratio_dn8));
        locals.var_aechvb_dn9 = (((locals.var_aechvb_dn9 * assign13090_e18420) * locals.var_toxratio) + (assign13090_e18421 * locals.var_toxratio_dn9));
        locals.var_aechvb_dn10 = (((locals.var_aechvb_dn10 * assign13090_e18420) * locals.var_toxratio) + (assign13090_e18421 * locals.var_toxratio_dn10));
        locals.var_aechvb_dn11 = (((locals.var_aechvb_dn11 * assign13090_e18420) * locals.var_toxratio) + (assign13090_e18421 * locals.var_toxratio_dn11));

        let assign13100_e18425: f64 = (-locals.var_bechvb);
        let assign13100_e18427: f64 = (assign13100_e18425 * p.p76);
        locals.var_bechvb = assign13100_e18427;

        let assign13110_e18430: f64 = (p.p1101 + locals.var_weff);
        locals.var_weff_sh = assign13110_e18430;

        let assign13150_e18459: f64 = if (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard431 = assign13150_e18459;

        let (assign13160_e18467,) = {
    if (locals.var_guard431 != 0.0) {
        let assign13160_e18463: f64 = (locals.var_weff_sh * p.p2);
        let assign13160_e18465: f64 = (assign13160_e18463 / p.p1099);
        (assign13160_e18465,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign13160_e18467;

        let (assign13170_e18475,) = {
    if (locals.var_guard431 != 0.0) {
        let assign13170_e18471: f64 = (p.p1100 * locals.var_weff_sh);
        let assign13170_e18473: f64 = (assign13170_e18471 * p.p2);
        (assign13170_e18473,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign13170_e18475;

        let (assign13180_e18480,) = {
    if (locals.var_guard431 == 0.0) {
        (1.0,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign13180_e18480;

        let (assign13190_e18485,) = {
    if (locals.var_guard431 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign13190_e18485;

        let assign13200_e18488: f64 = (-273.15);
        let assign13200_e18489: f64 = if p.p1028 <= assign13200_e18488 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign13200_e18489;

        let (assign13210_e18495, assign13210_e18495_d_n3, assign13210_e18495_d_n4, assign13210_e18495_d_n5, assign13210_e18495_d_n6, assign13210_e18495_d_n7, assign13210_e18495_d_n8, assign13210_e18495_d_n9, assign13210_e18495_d_n10, assign13210_e18495_d_n11,) = {
    if (locals.var_guard432 != 0.0) {
        let assign13210_e18493: f64 = (300.15 - 273.15);
        (assign13210_e18493, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign13210_e18495;
        locals.var_t0_dn3 = assign13210_e18495_d_n3;
        locals.var_t0_dn4 = assign13210_e18495_d_n4;
        locals.var_t0_dn5 = assign13210_e18495_d_n5;
        locals.var_t0_dn6 = assign13210_e18495_d_n6;
        locals.var_t0_dn7 = assign13210_e18495_d_n7;
        locals.var_t0_dn8 = assign13210_e18495_d_n8;
        locals.var_t0_dn9 = assign13210_e18495_d_n9;
        locals.var_t0_dn10 = assign13210_e18495_d_n10;
        locals.var_t0_dn11 = assign13210_e18495_d_n11;

        let (assign13220_e18499,) = {
    if (locals.var_guard432 != 0.0) {
        (300.15,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign13220_e18499;

        let (assign13230_e18506,) = {
    if (locals.var_guard432 == 0.0) {
        let assign13230_e18504: f64 = (p.p1028 + 273.15);
        (assign13230_e18504,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign13230_e18506;

        let assign13240_e18507: f64 = ctx_temp;
        let assign13240_e18509: f64 = (assign13240_e18507 + p.p23);
        locals.var_devtemp = assign13240_e18509;
        locals.var_devtemp_dn4 = 0.0;
        locals.var_devtemp_dn5 = 0.0;

        let assign13250_e18516: f64 = if ((p.p41 != 0.0) && (p.p1099 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard433 = assign13250_e18516;

        let assign13260_e18523: f64 = if ((p.p40 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        locals.var_guard434 = assign13260_e18523;

        let assign13270_e18525: f64 = 1.0;
        locals.var_guard435 = assign13270_e18525;

        let (assign13280_e18533, assign13280_e18533_d_n4, assign13280_e18533_d_n5,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 != 0.0)) && (locals.var_guard435 != 0.0)) {
        ((nv4 - 0.0), 1.0, 0.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4, locals.var_deltemp1_dn5,)
    }
};
        locals.var_deltemp1 = assign13280_e18533;
        locals.var_deltemp1_dn4 = assign13280_e18533_d_n4;
        locals.var_deltemp1_dn5 = assign13280_e18533_d_n5;

        let (assign13290_e18542, assign13290_e18542_d_n4, assign13290_e18542_d_n5,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 != 0.0)) && (locals.var_guard435 == 0.0)) {
        ((nv5 - 0.0), 0.0, 1.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4, locals.var_deltemp1_dn5,)
    }
};
        locals.var_deltemp1 = assign13290_e18542;
        locals.var_deltemp1_dn4 = assign13290_e18542_d_n4;
        locals.var_deltemp1_dn5 = assign13290_e18542_d_n5;

        let (assign13300_e18549, assign13300_e18549_d_n4, assign13300_e18549_d_n5,) = {
    if ((locals.var_guard433 != 0.0) && (locals.var_guard434 == 0.0)) {
        ((nv5 - 0.0), 0.0, 1.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4, locals.var_deltemp1_dn5,)
    }
};
        locals.var_deltemp1 = assign13300_e18549;
        locals.var_deltemp1_dn4 = assign13300_e18549_d_n4;
        locals.var_deltemp1_dn5 = assign13300_e18549_d_n5;

        let (assign13310_e18554, assign13310_e18554_d_n4, assign13310_e18554_d_n5,) = {
    if (locals.var_guard433 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4, locals.var_deltemp1_dn5,)
    }
};
        locals.var_deltemp1 = assign13310_e18554;
        locals.var_deltemp1_dn4 = assign13310_e18554_d_n4;
        locals.var_deltemp1_dn5 = assign13310_e18554_d_n5;

        let assign13320_e18557: f64 = (locals.var_deltemp1 + locals.var_devtemp);
        locals.var_devtemp = assign13320_e18557;
        locals.var_devtemp_dn4 = (locals.var_deltemp1_dn4 + locals.var_devtemp_dn4);
        locals.var_devtemp_dn5 = (locals.var_deltemp1_dn5 + locals.var_devtemp_dn5);

        let assign13360_e18565: f64 = (locals.var_kboq * locals.var_devtemp);
        locals.var_vt = assign13360_e18565;
        locals.var_vt_dn4 = (locals.var_kboq * locals.var_devtemp_dn4);
        locals.var_vt_dn5 = (locals.var_kboq * locals.var_devtemp_dn5);

        let assign13370_e18568: f64 = (1.0 / locals.var_vt);
        locals.var_inv_vt = assign13370_e18568;
        locals.var_inv_vt_dn4 = (-(locals.var_vt_dn4 / (locals.var_vt * locals.var_vt)));
        locals.var_inv_vt_dn5 = (-(locals.var_vt_dn5 / (locals.var_vt * locals.var_vt)));

        let assign13380_e18571: f64 = (locals.var_devtemp / locals.var_tnom);
        locals.var_tratio = assign13380_e18571;
        locals.var_tratio_dn4 = (locals.var_devtemp_dn4 / locals.var_tnom);
        locals.var_tratio_dn5 = (locals.var_devtemp_dn5 / locals.var_tnom);

        let assign13390_e18574: f64 = (locals.var_devtemp - locals.var_tnom);
        locals.var_deltemp = assign13390_e18574;
        locals.var_deltemp_dn4 = locals.var_devtemp_dn4;
        locals.var_deltemp_dn5 = locals.var_devtemp_dn5;

        let assign13400_e18577: f64 = (locals.var_kboq * locals.var_devtemp);
        locals.var_vtm = assign13400_e18577;
        locals.var_vtm_dn4 = (locals.var_kboq * locals.var_devtemp_dn4);
        locals.var_vtm_dn5 = (locals.var_kboq * locals.var_devtemp_dn5);

        let assign13410_e18580: f64 = (locals.var_kboq * locals.var_tnom);
        locals.var_vtm0 = assign13410_e18580;

        let assign13420_e18584: f64 = (p.p1029 * locals.var_devtemp);
        let assign13420_e18586: f64 = (assign13420_e18584 * locals.var_devtemp);
        let assign13420_e18589: f64 = (locals.var_devtemp + p.p1030);
        let assign13420_e18590: f64 = (assign13420_e18586 / assign13420_e18589);
        let assign13420_e18591: f64 = (p.p108 - assign13420_e18590);
        locals.var_eg = assign13420_e18591;
        locals.var_eg_dn4 = (-((((((p.p1029 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign13420_e18584 * locals.var_devtemp_dn4)) * assign13420_e18589) - (assign13420_e18586 * locals.var_devtemp_dn4)) / (assign13420_e18589 * assign13420_e18589)));
        locals.var_eg_dn5 = (-((((((p.p1029 * locals.var_devtemp_dn5) * locals.var_devtemp) + (assign13420_e18584 * locals.var_devtemp_dn5)) * assign13420_e18589) - (assign13420_e18586 * locals.var_devtemp_dn5)) / (assign13420_e18589 * assign13420_e18589)));

        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tnom;
        let assign13430_e18594: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign13430_e18597: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign13430_e18598: f64 = (assign13430_e18597).sqrt();
        let assign13430_e18599: f64 = (assign13430_e18594 * assign13430_e18598);
        locals.var_t1 = assign13430_e18599;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = (((locals.var_devtemp_dn4 / locals.var_tnom) * assign13430_e18598) + (assign13430_e18594 * ((locals.var_devtemp_dn4 / locals.var_tnom) / (2.0 * assign13430_e18598))));
        locals.var_t1_dn5 = (((locals.var_devtemp_dn5 / locals.var_tnom) * assign13430_e18598) + (assign13430_e18594 * ((locals.var_devtemp_dn5 / locals.var_tnom) / (2.0 * assign13430_e18598))));
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;

        let assign13440_e18602: f64 = (p.p107 * locals.var_t1);
        let assign13440_e18606: f64 = (2.0 * locals.var_vtm0);
        let assign13440_e18607: f64 = (locals.var_eg / assign13440_e18606);
        let assign13440_e18611: f64 = (2.0 * locals.var_vtm);
        let assign13440_e18612: f64 = (locals.var_eg / assign13440_e18611);
        let assign13440_e18613: f64 = (assign13440_e18607 - assign13440_e18612);
        let assign13440_e18614: f64 = { let limited_exp_arg = assign13440_e18613; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13440_e18615: f64 = (assign13440_e18602 * assign13440_e18614);
        locals.var_ni = assign13440_e18615;
        locals.var_ni_dn3 = ((p.p107 * locals.var_t1_dn3) * assign13440_e18614);
        locals.var_ni_dn4 = (((p.p107 * locals.var_t1_dn4) * assign13440_e18614) + (assign13440_e18602 * ({ let limited_exp_arg = assign13440_e18613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_eg_dn4 / assign13440_e18606) - (((locals.var_eg_dn4 * assign13440_e18611) - (locals.var_eg * (2.0 * locals.var_vtm_dn4))) / (assign13440_e18611 * assign13440_e18611))))));
        locals.var_ni_dn5 = (((p.p107 * locals.var_t1_dn5) * assign13440_e18614) + (assign13440_e18602 * ({ let limited_exp_arg = assign13440_e18613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_eg_dn5 / assign13440_e18606) - (((locals.var_eg_dn5 * assign13440_e18611) - (locals.var_eg * (2.0 * locals.var_vtm_dn5))) / (assign13440_e18611 * assign13440_e18611))))));
        locals.var_ni_dn6 = ((p.p107 * locals.var_t1_dn6) * assign13440_e18614);
        locals.var_ni_dn7 = ((p.p107 * locals.var_t1_dn7) * assign13440_e18614);
        locals.var_ni_dn8 = ((p.p107 * locals.var_t1_dn8) * assign13440_e18614);
        locals.var_ni_dn9 = ((p.p107 * locals.var_t1_dn9) * assign13440_e18614);
        locals.var_ni_dn10 = ((p.p107 * locals.var_t1_dn10) * assign13440_e18614);
        locals.var_ni_dn11 = ((p.p107 * locals.var_t1_dn11) * assign13440_e18614);

        let assign13450_e18626: f64 = if (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard436 = assign13450_e18626;

        let (assign13460_e18635, assign13460_e18635_d_n3, assign13460_e18635_d_n4, assign13460_e18635_d_n5, assign13460_e18635_d_n6, assign13460_e18635_d_n7, assign13460_e18635_d_n8, assign13460_e18635_d_n9, assign13460_e18635_d_n10, assign13460_e18635_d_n11,) = {
    if (locals.var_guard436 != 0.0) {
        let assign13460_e18630: f64 = (locals.var_ndep_i / locals.var_ni);
        let assign13460_e18632: f64 = (assign13460_e18630).max(1e-38);
        let assign13460_e18633: f64 = (assign13460_e18632).ln();
        (assign13460_e18633, (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn3 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn4 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn5 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn6 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn7 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn8 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn9 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn10 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn11 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign13460_e18635;
        locals.var_t0_dn3 = assign13460_e18635_d_n3;
        locals.var_t0_dn4 = assign13460_e18635_d_n4;
        locals.var_t0_dn5 = assign13460_e18635_d_n5;
        locals.var_t0_dn6 = assign13460_e18635_d_n6;
        locals.var_t0_dn7 = assign13460_e18635_d_n7;
        locals.var_t0_dn8 = assign13460_e18635_d_n8;
        locals.var_t0_dn9 = assign13460_e18635_d_n9;
        locals.var_t0_dn10 = assign13460_e18635_d_n10;
        locals.var_t0_dn11 = assign13460_e18635_d_n11;

        let (assign13470_e18644, assign13470_e18644_d_n3, assign13470_e18644_d_n4, assign13470_e18644_d_n5, assign13470_e18644_d_n6, assign13470_e18644_d_n7, assign13470_e18644_d_n8, assign13470_e18644_d_n9, assign13470_e18644_d_n10, assign13470_e18644_d_n11,) = {
    if (locals.var_guard436 != 0.0) {
        let assign13470_e18639: f64 = (locals.var_t0 * locals.var_t0);
        let assign13470_e18641: f64 = (assign13470_e18639 + 1e-6);
        let assign13470_e18642: f64 = (assign13470_e18641).sqrt();
        (assign13470_e18642, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign13470_e18642)),)
    } else {
        (locals.var_phib, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11,)
    }
};
        locals.var_phib = assign13470_e18644;
        locals.var_phib_dn3 = assign13470_e18644_d_n3;
        locals.var_phib_dn4 = assign13470_e18644_d_n4;
        locals.var_phib_dn5 = assign13470_e18644_d_n5;
        locals.var_phib_dn6 = assign13470_e18644_d_n6;
        locals.var_phib_dn7 = assign13470_e18644_d_n7;
        locals.var_phib_dn8 = assign13470_e18644_d_n8;
        locals.var_phib_dn9 = assign13470_e18644_d_n9;
        locals.var_phib_dn10 = assign13470_e18644_d_n10;
        locals.var_phib_dn11 = assign13470_e18644_d_n11;

    }

    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13480_e18654, assign13480_e18654_d_n3, assign13480_e18654_d_n4, assign13480_e18654_d_n5, assign13480_e18654_d_n6, assign13480_e18654_d_n7, assign13480_e18654_d_n8, assign13480_e18654_d_n9, assign13480_e18654_d_n10, assign13480_e18654_d_n11,) = {
    if (locals.var_guard436 == 0.0) {
        let assign13480_e18649: f64 = (locals.var_ndep_i / locals.var_ni);
        let assign13480_e18651: f64 = (assign13480_e18649).max(1e-38);
        let assign13480_e18652: f64 = (assign13480_e18651).ln();
        (assign13480_e18652, (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn3 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn4 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn5 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn6 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn7 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn8 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn9 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn10 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn11 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651),)
    } else {
        (locals.var_phib, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11,)
    }
};
        locals.var_phib = assign13480_e18654;
        locals.var_phib_dn3 = assign13480_e18654_d_n3;
        locals.var_phib_dn4 = assign13480_e18654_d_n4;
        locals.var_phib_dn5 = assign13480_e18654_d_n5;
        locals.var_phib_dn6 = assign13480_e18654_d_n6;
        locals.var_phib_dn7 = assign13480_e18654_d_n7;
        locals.var_phib_dn8 = assign13480_e18654_d_n8;
        locals.var_phib_dn9 = assign13480_e18654_d_n9;
        locals.var_phib_dn10 = assign13480_e18654_d_n10;
        locals.var_phib_dn11 = assign13480_e18654_d_n11;

        let assign13490_e18665: f64 = if (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard437 = assign13490_e18665;

        let (assign13500_e18678, assign13500_e18678_d_n3, assign13500_e18678_d_n4, assign13500_e18678_d_n5, assign13500_e18678_d_n6, assign13500_e18678_d_n7, assign13500_e18678_d_n8, assign13500_e18678_d_n9, assign13500_e18678_d_n10, assign13500_e18678_d_n11,) = {
    if (locals.var_guard437 != 0.0) {
        let assign13500_e18669: f64 = (locals.var_ndepedge_i * locals.var_nsd_i);
        let assign13500_e18672: f64 = (locals.var_ni * locals.var_ni);
        let assign13500_e18673: f64 = (assign13500_e18669 / assign13500_e18672);
        let assign13500_e18675: f64 = (assign13500_e18673).max(1e-38);
        let assign13500_e18676: f64 = (assign13500_e18675).ln();
        (assign13500_e18676, (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign13500_e18678;
        locals.var_t0_dn3 = assign13500_e18678_d_n3;
        locals.var_t0_dn4 = assign13500_e18678_d_n4;
        locals.var_t0_dn5 = assign13500_e18678_d_n5;
        locals.var_t0_dn6 = assign13500_e18678_d_n6;
        locals.var_t0_dn7 = assign13500_e18678_d_n7;
        locals.var_t0_dn8 = assign13500_e18678_d_n8;
        locals.var_t0_dn9 = assign13500_e18678_d_n9;
        locals.var_t0_dn10 = assign13500_e18678_d_n10;
        locals.var_t0_dn11 = assign13500_e18678_d_n11;

        let (assign13510_e18687, assign13510_e18687_d_n3, assign13510_e18687_d_n4, assign13510_e18687_d_n5, assign13510_e18687_d_n6, assign13510_e18687_d_n7, assign13510_e18687_d_n8, assign13510_e18687_d_n9, assign13510_e18687_d_n10, assign13510_e18687_d_n11,) = {
    if (locals.var_guard437 != 0.0) {
        let assign13510_e18682: f64 = (locals.var_t0 * locals.var_t0);
        let assign13510_e18684: f64 = (assign13510_e18682 + 1e-6);
        let assign13510_e18685: f64 = (assign13510_e18684).sqrt();
        (assign13510_e18685, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign13510_e18685)),)
    } else {
        (locals.var_vbi_edge, locals.var_vbi_edge_dn3, locals.var_vbi_edge_dn4, locals.var_vbi_edge_dn5, locals.var_vbi_edge_dn6, locals.var_vbi_edge_dn7, locals.var_vbi_edge_dn8, locals.var_vbi_edge_dn9, locals.var_vbi_edge_dn10, locals.var_vbi_edge_dn11,)
    }
};
        locals.var_vbi_edge = assign13510_e18687;
        locals.var_vbi_edge_dn3 = assign13510_e18687_d_n3;
        locals.var_vbi_edge_dn4 = assign13510_e18687_d_n4;
        locals.var_vbi_edge_dn5 = assign13510_e18687_d_n5;
        locals.var_vbi_edge_dn6 = assign13510_e18687_d_n6;
        locals.var_vbi_edge_dn7 = assign13510_e18687_d_n7;
        locals.var_vbi_edge_dn8 = assign13510_e18687_d_n8;
        locals.var_vbi_edge_dn9 = assign13510_e18687_d_n9;
        locals.var_vbi_edge_dn10 = assign13510_e18687_d_n10;
        locals.var_vbi_edge_dn11 = assign13510_e18687_d_n11;

        let (assign13520_e18701, assign13520_e18701_d_n3, assign13520_e18701_d_n4, assign13520_e18701_d_n5, assign13520_e18701_d_n6, assign13520_e18701_d_n7, assign13520_e18701_d_n8, assign13520_e18701_d_n9, assign13520_e18701_d_n10, assign13520_e18701_d_n11,) = {
    if (locals.var_guard437 == 0.0) {
        let assign13520_e18692: f64 = (locals.var_ndepedge_i * locals.var_nsd_i);
        let assign13520_e18695: f64 = (locals.var_ni * locals.var_ni);
        let assign13520_e18696: f64 = (assign13520_e18692 / assign13520_e18695);
        let assign13520_e18698: f64 = (assign13520_e18696).max(1e-38);
        let assign13520_e18699: f64 = (assign13520_e18698).ln();
        (assign13520_e18699, (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698),)
    } else {
        (locals.var_vbi_edge, locals.var_vbi_edge_dn3, locals.var_vbi_edge_dn4, locals.var_vbi_edge_dn5, locals.var_vbi_edge_dn6, locals.var_vbi_edge_dn7, locals.var_vbi_edge_dn8, locals.var_vbi_edge_dn9, locals.var_vbi_edge_dn10, locals.var_vbi_edge_dn11,)
    }
};
        locals.var_vbi_edge = assign13520_e18701;
        locals.var_vbi_edge_dn3 = assign13520_e18701_d_n3;
        locals.var_vbi_edge_dn4 = assign13520_e18701_d_n4;
        locals.var_vbi_edge_dn5 = assign13520_e18701_d_n5;
        locals.var_vbi_edge_dn6 = assign13520_e18701_d_n6;
        locals.var_vbi_edge_dn7 = assign13520_e18701_d_n7;
        locals.var_vbi_edge_dn8 = assign13520_e18701_d_n8;
        locals.var_vbi_edge_dn9 = assign13520_e18701_d_n9;
        locals.var_vbi_edge_dn10 = assign13520_e18701_d_n10;
        locals.var_vbi_edge_dn11 = assign13520_e18701_d_n11;

        let assign13530_e18704: f64 = if locals.var_ngate_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard438 = assign13530_e18704;

        let (assign13540_e18720, assign13540_e18720_d_n4, assign13540_e18720_d_n5,) = {
    if (locals.var_guard438 != 0.0) {
        let assign13540_e18707: f64 = (-locals.var_devsign);
        let assign13540_e18709: f64 = (assign13540_e18707 * locals.var_vt);
        let assign13540_e18712: f64 = (locals.var_ngate_i / locals.var_nsd_i);
        let assign13540_e18714: f64 = (assign13540_e18712).max(1e-38);
        let assign13540_e18715: f64 = (assign13540_e18714).ln();
        let assign13540_e18716: f64 = (assign13540_e18709 * assign13540_e18715);
        let assign13540_e18718: f64 = (assign13540_e18716 + p.p5);
        (assign13540_e18718, ((assign13540_e18707 * locals.var_vt_dn4) * assign13540_e18715), ((assign13540_e18707 * locals.var_vt_dn5) * assign13540_e18715),)
    } else {
        (locals.var_vfbsdr, locals.var_vfbsdr_dn4, locals.var_vfbsdr_dn5,)
    }
};
        locals.var_vfbsdr = assign13540_e18720;
        locals.var_vfbsdr_dn4 = assign13540_e18720_d_n4;
        locals.var_vfbsdr_dn5 = assign13540_e18720_d_n5;

        let (assign13550_e18725, assign13550_e18725_d_n4, assign13550_e18725_d_n5,) = {
    if (locals.var_guard438 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbsdr, locals.var_vfbsdr_dn4, locals.var_vfbsdr_dn5,)
    }
};
        locals.var_vfbsdr = assign13550_e18725;
        locals.var_vfbsdr_dn4 = assign13550_e18725_d_n4;
        locals.var_vfbsdr_dn5 = assign13550_e18725_d_n5;

        let assign13660_e18777: f64 = (locals.var_vt * locals.var_phib);
        let assign13660_e18778: f64 = (0.4 + assign13660_e18777);
        let assign13660_e18780: f64 = (assign13660_e18778 + locals.var_phin_i);
        let assign13660_e18782: f64 = (assign13660_e18780).max(0.4);
        locals.var_phist = assign13660_e18782;
        locals.var_phist_dn3 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn3) } else { 0.0 };
        locals.var_phist_dn4 = if assign13660_e18780 >= 0.4 { ((locals.var_vt_dn4 * locals.var_phib) + (locals.var_vt * locals.var_phib_dn4)) } else { 0.0 };
        locals.var_phist_dn5 = if assign13660_e18780 >= 0.4 { ((locals.var_vt_dn5 * locals.var_phib) + (locals.var_vt * locals.var_phib_dn5)) } else { 0.0 };
        locals.var_phist_dn6 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn6) } else { 0.0 };
        locals.var_phist_dn7 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn7) } else { 0.0 };
        locals.var_phist_dn8 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn8) } else { 0.0 };
        locals.var_phist_dn9 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn9) } else { 0.0 };
        locals.var_phist_dn10 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn10) } else { 0.0 };
        locals.var_phist_dn11 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn11) } else { 0.0 };

        let assign13670_e18784: f64 = (locals.var_phist).sqrt();
        locals.var_sqrtphist = assign13670_e18784;
        locals.var_sqrtphist_dn3 = (locals.var_phist_dn3 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn4 = (locals.var_phist_dn4 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn5 = (locals.var_phist_dn5 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn6 = (locals.var_phist_dn6 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn7 = (locals.var_phist_dn7 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn8 = (locals.var_phist_dn8 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn9 = (locals.var_phist_dn9 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn10 = (locals.var_phist_dn10 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn11 = (locals.var_phist_dn11 / (2.0 * assign13670_e18784));

        let assign13680_e18787: f64 = (2.0 * locals.var_epssi);
        let assign13680_e18790: f64 = (1.602176462e-19 * locals.var_ndep_i);
        let assign13680_e18791: f64 = (assign13680_e18787 / assign13680_e18790);
        let assign13680_e18792: f64 = (assign13680_e18791).sqrt();
        locals.var_t1dep = assign13680_e18792;
        locals.var_t1dep_dn3 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn3)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn4 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn4)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn5 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn5)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn6 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn6)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn7 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn7)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn8 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn8)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn9 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn9)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn10 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn10)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn11 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn11)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));

        let assign13690_e18795: f64 = (locals.var_epssi / locals.var_epsox);
        let assign13690_e18797: f64 = (assign13690_e18795 * p.p76);
        let assign13690_e18799: f64 = (assign13690_e18797 * locals.var_xj_i);
        let assign13690_e18800: f64 = (assign13690_e18799).sqrt();
        locals.var_litl = assign13690_e18800;

        let assign13700_e18807: f64 = (locals.var_tratio - 1.0);
        let assign13700_e18808: f64 = (p.p1031 * assign13700_e18807);
        let assign13700_e18809: f64 = (1.0 + assign13700_e18808);
        let assign13700_e18814: f64 = (locals.var_tratio - 1.0);
        let assign13700_e18815: f64 = (p.p1031 * assign13700_e18814);
        let assign13700_e18816: f64 = (1.0 + assign13700_e18815);
        let assign13700_e18821: f64 = (locals.var_tratio - 1.0);
        let assign13700_e18822: f64 = (p.p1031 * assign13700_e18821);
        let assign13700_e18823: f64 = (1.0 + assign13700_e18822);
        let assign13700_e18824: f64 = (assign13700_e18816 * assign13700_e18823);
        let assign13700_e18827: f64 = (4.0 * 0.001);
        let assign13700_e18829: f64 = (assign13700_e18827 * 0.001);
        let assign13700_e18830: f64 = (assign13700_e18824 + assign13700_e18829);
        let assign13700_e18831: f64 = (assign13700_e18830).sqrt();
        let assign13700_e18832: f64 = (assign13700_e18809 + assign13700_e18831);
        let assign13700_e18833: f64 = (0.5 * assign13700_e18832);
        let assign13700_e18834: f64 = (locals.var_nfactor_i * assign13700_e18833);
        locals.var_nfactor_t = assign13700_e18834;
        locals.var_nfactor_t_dn3 = (locals.var_nfactor_i_dn3 * assign13700_e18833);
        locals.var_nfactor_t_dn4 = ((locals.var_nfactor_i_dn4 * assign13700_e18833) + (locals.var_nfactor_i * (0.5 * ((p.p1031 * locals.var_tratio_dn4) + ((((p.p1031 * locals.var_tratio_dn4) * assign13700_e18823) + (assign13700_e18816 * (p.p1031 * locals.var_tratio_dn4))) / (2.0 * assign13700_e18831))))));
        locals.var_nfactor_t_dn5 = ((locals.var_nfactor_i_dn5 * assign13700_e18833) + (locals.var_nfactor_i * (0.5 * ((p.p1031 * locals.var_tratio_dn5) + ((((p.p1031 * locals.var_tratio_dn5) * assign13700_e18823) + (assign13700_e18816 * (p.p1031 * locals.var_tratio_dn5))) / (2.0 * assign13700_e18831))))));
        locals.var_nfactor_t_dn6 = (locals.var_nfactor_i_dn6 * assign13700_e18833);
        locals.var_nfactor_t_dn7 = (locals.var_nfactor_i_dn7 * assign13700_e18833);
        locals.var_nfactor_t_dn8 = (locals.var_nfactor_i_dn8 * assign13700_e18833);
        locals.var_nfactor_t_dn9 = (locals.var_nfactor_i_dn9 * assign13700_e18833);
        locals.var_nfactor_t_dn10 = (locals.var_nfactor_i_dn10 * assign13700_e18833);
        locals.var_nfactor_t_dn11 = (locals.var_nfactor_i_dn11 * assign13700_e18833);

        let assign13710_e18840: f64 = (locals.var_tratio - 1.0);
        let assign13710_e18841: f64 = (p.p1059 * assign13710_e18840);
        let assign13710_e18842: f64 = (1.0 + assign13710_e18841);
        let assign13710_e18843: f64 = (locals.var_eta0_i * assign13710_e18842);
        locals.var_eta0_t = assign13710_e18843;
        locals.var_eta0_t_dn3 = (locals.var_eta0_i_dn3 * assign13710_e18842);
        locals.var_eta0_t_dn4 = ((locals.var_eta0_i_dn4 * assign13710_e18842) + (locals.var_eta0_i * (p.p1059 * locals.var_tratio_dn4)));
        locals.var_eta0_t_dn5 = ((locals.var_eta0_i_dn5 * assign13710_e18842) + (locals.var_eta0_i * (p.p1059 * locals.var_tratio_dn5)));
        locals.var_eta0_t_dn6 = (locals.var_eta0_i_dn6 * assign13710_e18842);
        locals.var_eta0_t_dn7 = (locals.var_eta0_i_dn7 * assign13710_e18842);
        locals.var_eta0_t_dn8 = (locals.var_eta0_i_dn8 * assign13710_e18842);
        locals.var_eta0_t_dn9 = (locals.var_eta0_i_dn9 * assign13710_e18842);
        locals.var_eta0_t_dn10 = (locals.var_eta0_i_dn10 * assign13710_e18842);
        locals.var_eta0_t_dn11 = (locals.var_eta0_i_dn11 * assign13710_e18842);

        let assign13720_e18846: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard449 = assign13720_e18846;

        let (assign13730_e18858, assign13730_e18858_d_n3, assign13730_e18858_d_n4, assign13730_e18858_d_n5, assign13730_e18858_d_n6, assign13730_e18858_d_n7, assign13730_e18858_d_n8, assign13730_e18858_d_n9, assign13730_e18858_d_n10, assign13730_e18858_d_n11,) = {
    if (locals.var_guard449 != 0.0) {
        let assign13730_e18853: f64 = (locals.var_tratio - 1.0);
        let assign13730_e18854: f64 = (p.p1059 * assign13730_e18853);
        let assign13730_e18855: f64 = (1.0 + assign13730_e18854);
        let assign13730_e18856: f64 = (locals.var_eta0r_i * assign13730_e18855);
        (assign13730_e18856, (locals.var_eta0r_i_dn3 * assign13730_e18855), ((locals.var_eta0r_i_dn4 * assign13730_e18855) + (locals.var_eta0r_i * (p.p1059 * locals.var_tratio_dn4))), ((locals.var_eta0r_i_dn5 * assign13730_e18855) + (locals.var_eta0r_i * (p.p1059 * locals.var_tratio_dn5))), (locals.var_eta0r_i_dn6 * assign13730_e18855), (locals.var_eta0r_i_dn7 * assign13730_e18855), (locals.var_eta0r_i_dn8 * assign13730_e18855), (locals.var_eta0r_i_dn9 * assign13730_e18855), (locals.var_eta0r_i_dn10 * assign13730_e18855), (locals.var_eta0r_i_dn11 * assign13730_e18855),)
    } else {
        (locals.var_eta0r_t, locals.var_eta0r_t_dn3, locals.var_eta0r_t_dn4, locals.var_eta0r_t_dn5, locals.var_eta0r_t_dn6, locals.var_eta0r_t_dn7, locals.var_eta0r_t_dn8, locals.var_eta0r_t_dn9, locals.var_eta0r_t_dn10, locals.var_eta0r_t_dn11,)
    }
};
        locals.var_eta0r_t = assign13730_e18858;
        locals.var_eta0r_t_dn3 = assign13730_e18858_d_n3;
        locals.var_eta0r_t_dn4 = assign13730_e18858_d_n4;
        locals.var_eta0r_t_dn5 = assign13730_e18858_d_n5;
        locals.var_eta0r_t_dn6 = assign13730_e18858_d_n6;
        locals.var_eta0r_t_dn7 = assign13730_e18858_d_n7;
        locals.var_eta0r_t_dn8 = assign13730_e18858_d_n8;
        locals.var_eta0r_t_dn9 = assign13730_e18858_d_n9;
        locals.var_eta0r_t_dn10 = assign13730_e18858_d_n10;
        locals.var_eta0r_t_dn11 = assign13730_e18858_d_n11;

        let (assign13740_e18868,) = {
    if (p.p30 != 1.0) {
        let assign13740_e18864: f64 = (0.3333333333333333 * p.p347);
        (assign13740_e18864,)
    } else {
        let assign13740_e18867: f64 = (0.5 * p.p347);
        (assign13740_e18867,)
    }
};
        locals.var_eta_mu = assign13740_e18868;

        let assign13750_e18872: f64 = (locals.var_tratio).powf(locals.var_ute_i);
        let assign13750_e18873: f64 = (locals.var_u0_i * assign13750_e18872);
        locals.var_u0_t = assign13750_e18873;
        locals.var_u0_t_dn3 = 0.0;
        locals.var_u0_t_dn4 = (locals.var_u0_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13750_e18872 * (locals.var_ute_i * (locals.var_tratio_dn4 / locals.var_tratio))) });
        locals.var_u0_t_dn5 = (locals.var_u0_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13750_e18872 * (locals.var_ute_i * (locals.var_tratio_dn5 / locals.var_tratio))) });
        locals.var_u0_t_dn6 = 0.0;
        locals.var_u0_t_dn7 = 0.0;
        locals.var_u0_t_dn8 = 0.0;
        locals.var_u0_t_dn9 = 0.0;
        locals.var_u0_t_dn10 = 0.0;
        locals.var_u0_t_dn11 = 0.0;

        let assign13760_e18879: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13760_e18880: f64 = (1.0 + assign13760_e18879);
        let assign13760_e18882: f64 = (assign13760_e18880 - 1e-6);
        let assign13760_e18886: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13760_e18887: f64 = (1.0 + assign13760_e18886);
        let assign13760_e18889: f64 = (assign13760_e18887 - 1e-6);
        let assign13760_e18893: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13760_e18894: f64 = (1.0 + assign13760_e18893);
        let assign13760_e18896: f64 = (assign13760_e18894 - 1e-6);
        let assign13760_e18897: f64 = (assign13760_e18889 * assign13760_e18896);
        let assign13760_e18900: f64 = (4.0 * 0.001);
        let assign13760_e18902: f64 = (assign13760_e18900 * 0.001);
        let assign13760_e18903: f64 = (assign13760_e18897 + assign13760_e18902);
        let assign13760_e18904: f64 = (assign13760_e18903).sqrt();
        let assign13760_e18905: f64 = (assign13760_e18882 + assign13760_e18904);
        let assign13760_e18906: f64 = (0.5 * assign13760_e18905);
        let assign13760_e18907: f64 = (locals.var_ua_i * assign13760_e18906);
        locals.var_ua_t = assign13760_e18907;
        locals.var_ua_t_dn3 = (locals.var_ua_i_dn3 * assign13760_e18906);
        locals.var_ua_t_dn4 = ((locals.var_ua_i_dn4 * assign13760_e18906) + (locals.var_ua_i * (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn4) + ((((locals.var_ua1_i * locals.var_deltemp_dn4) * assign13760_e18896) + (assign13760_e18889 * (locals.var_ua1_i * locals.var_deltemp_dn4))) / (2.0 * assign13760_e18904))))));
        locals.var_ua_t_dn5 = ((locals.var_ua_i_dn5 * assign13760_e18906) + (locals.var_ua_i * (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn5) + ((((locals.var_ua1_i * locals.var_deltemp_dn5) * assign13760_e18896) + (assign13760_e18889 * (locals.var_ua1_i * locals.var_deltemp_dn5))) / (2.0 * assign13760_e18904))))));
        locals.var_ua_t_dn6 = (locals.var_ua_i_dn6 * assign13760_e18906);
        locals.var_ua_t_dn7 = (locals.var_ua_i_dn7 * assign13760_e18906);
        locals.var_ua_t_dn8 = (locals.var_ua_i_dn8 * assign13760_e18906);
        locals.var_ua_t_dn9 = (locals.var_ua_i_dn9 * assign13760_e18906);
        locals.var_ua_t_dn10 = (locals.var_ua_i_dn10 * assign13760_e18906);
        locals.var_ua_t_dn11 = (locals.var_ua_i_dn11 * assign13760_e18906);

        let assign13770_e18913: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13770_e18914: f64 = (1.0 + assign13770_e18913);
        let assign13770_e18916: f64 = (assign13770_e18914 - 1e-6);
        let assign13770_e18920: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13770_e18921: f64 = (1.0 + assign13770_e18920);
        let assign13770_e18923: f64 = (assign13770_e18921 - 1e-6);
        let assign13770_e18927: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13770_e18928: f64 = (1.0 + assign13770_e18927);
        let assign13770_e18930: f64 = (assign13770_e18928 - 1e-6);
        let assign13770_e18931: f64 = (assign13770_e18923 * assign13770_e18930);
        let assign13770_e18934: f64 = (4.0 * 0.001);
        let assign13770_e18936: f64 = (assign13770_e18934 * 0.001);
        let assign13770_e18937: f64 = (assign13770_e18931 + assign13770_e18936);
        let assign13770_e18938: f64 = (assign13770_e18937).sqrt();
        let assign13770_e18939: f64 = (assign13770_e18916 + assign13770_e18938);
        let assign13770_e18940: f64 = (0.5 * assign13770_e18939);
        let assign13770_e18941: f64 = (locals.var_uc_i * assign13770_e18940);
        locals.var_uc_t = assign13770_e18941;
        locals.var_uc_t_dn3 = (locals.var_uc_i_dn3 * assign13770_e18940);
        locals.var_uc_t_dn4 = ((locals.var_uc_i_dn4 * assign13770_e18940) + (locals.var_uc_i * (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn4) + ((((locals.var_uc1_i * locals.var_deltemp_dn4) * assign13770_e18930) + (assign13770_e18923 * (locals.var_uc1_i * locals.var_deltemp_dn4))) / (2.0 * assign13770_e18938))))));
        locals.var_uc_t_dn5 = ((locals.var_uc_i_dn5 * assign13770_e18940) + (locals.var_uc_i * (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn5) + ((((locals.var_uc1_i * locals.var_deltemp_dn5) * assign13770_e18930) + (assign13770_e18923 * (locals.var_uc1_i * locals.var_deltemp_dn5))) / (2.0 * assign13770_e18938))))));
        locals.var_uc_t_dn6 = (locals.var_uc_i_dn6 * assign13770_e18940);
        locals.var_uc_t_dn7 = (locals.var_uc_i_dn7 * assign13770_e18940);
        locals.var_uc_t_dn8 = (locals.var_uc_i_dn8 * assign13770_e18940);
        locals.var_uc_t_dn9 = (locals.var_uc_i_dn9 * assign13770_e18940);
        locals.var_uc_t_dn10 = (locals.var_uc_i_dn10 * assign13770_e18940);
        locals.var_uc_t_dn11 = (locals.var_uc_i_dn11 * assign13770_e18940);

        let assign13780_e18945: f64 = (locals.var_tratio).powf(locals.var_ud1_i);
        let assign13780_e18946: f64 = (locals.var_ud_i * assign13780_e18945);
        locals.var_ud_t = assign13780_e18946;
        locals.var_ud_t_dn3 = (locals.var_ud_i_dn3 * assign13780_e18945);
        locals.var_ud_t_dn4 = ((locals.var_ud_i_dn4 * assign13780_e18945) + (locals.var_ud_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13780_e18945 * (locals.var_ud1_i * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_ud_t_dn5 = ((locals.var_ud_i_dn5 * assign13780_e18945) + (locals.var_ud_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13780_e18945 * (locals.var_ud1_i * (locals.var_tratio_dn5 / locals.var_tratio))) }));
        locals.var_ud_t_dn6 = (locals.var_ud_i_dn6 * assign13780_e18945);
        locals.var_ud_t_dn7 = (locals.var_ud_i_dn7 * assign13780_e18945);
        locals.var_ud_t_dn8 = (locals.var_ud_i_dn8 * assign13780_e18945);
        locals.var_ud_t_dn9 = (locals.var_ud_i_dn9 * assign13780_e18945);
        locals.var_ud_t_dn10 = (locals.var_ud_i_dn10 * assign13780_e18945);
        locals.var_ud_t_dn11 = (locals.var_ud_i_dn11 * assign13780_e18945);

        let assign13790_e18950: f64 = (locals.var_tratio).powf(locals.var_ucste_i);
        let assign13790_e18951: f64 = (locals.var_ucs_i * assign13790_e18950);
        locals.var_ucs_t = assign13790_e18951;
        locals.var_ucs_t_dn4 = (locals.var_ucs_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13790_e18950 * (locals.var_ucste_i * (locals.var_tratio_dn4 / locals.var_tratio))) });
        locals.var_ucs_t_dn5 = (locals.var_ucs_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13790_e18950 * (locals.var_ucste_i * (locals.var_tratio_dn5 / locals.var_tratio))) });

        let assign13800_e18958: f64 = (locals.var_tratio - 1.0);
        let assign13800_e18959: f64 = (locals.var_eu1_i * assign13800_e18958);
        let assign13800_e18960: f64 = (1.0 + assign13800_e18959);
        let assign13800_e18965: f64 = (locals.var_tratio - 1.0);
        let assign13800_e18966: f64 = (locals.var_eu1_i * assign13800_e18965);
        let assign13800_e18967: f64 = (1.0 + assign13800_e18966);
        let assign13800_e18972: f64 = (locals.var_tratio - 1.0);
        let assign13800_e18973: f64 = (locals.var_eu1_i * assign13800_e18972);
        let assign13800_e18974: f64 = (1.0 + assign13800_e18973);
        let assign13800_e18975: f64 = (assign13800_e18967 * assign13800_e18974);
        let assign13800_e18978: f64 = (4.0 * 0.001);
        let assign13800_e18980: f64 = (assign13800_e18978 * 0.001);
        let assign13800_e18981: f64 = (assign13800_e18975 + assign13800_e18980);
        let assign13800_e18982: f64 = (assign13800_e18981).sqrt();
        let assign13800_e18983: f64 = (assign13800_e18960 + assign13800_e18982);
        let assign13800_e18984: f64 = (0.5 * assign13800_e18983);
        let assign13800_e18985: f64 = (locals.var_eu_i * assign13800_e18984);
        locals.var_eu_t = assign13800_e18985;
        locals.var_eu_t_dn3 = (locals.var_eu_i_dn3 * assign13800_e18984);
        locals.var_eu_t_dn4 = ((locals.var_eu_i_dn4 * assign13800_e18984) + (locals.var_eu_i * (0.5 * ((locals.var_eu1_i * locals.var_tratio_dn4) + ((((locals.var_eu1_i * locals.var_tratio_dn4) * assign13800_e18974) + (assign13800_e18967 * (locals.var_eu1_i * locals.var_tratio_dn4))) / (2.0 * assign13800_e18982))))));
        locals.var_eu_t_dn5 = ((locals.var_eu_i_dn5 * assign13800_e18984) + (locals.var_eu_i * (0.5 * ((locals.var_eu1_i * locals.var_tratio_dn5) + ((((locals.var_eu1_i * locals.var_tratio_dn5) * assign13800_e18974) + (assign13800_e18967 * (locals.var_eu1_i * locals.var_tratio_dn5))) / (2.0 * assign13800_e18982))))));
        locals.var_eu_t_dn6 = (locals.var_eu_i_dn6 * assign13800_e18984);
        locals.var_eu_t_dn7 = (locals.var_eu_i_dn7 * assign13800_e18984);
        locals.var_eu_t_dn8 = (locals.var_eu_i_dn8 * assign13800_e18984);
        locals.var_eu_t_dn9 = (locals.var_eu_i_dn9 * assign13800_e18984);
        locals.var_eu_t_dn10 = (locals.var_eu_i_dn10 * assign13800_e18984);
        locals.var_eu_t_dn11 = (locals.var_eu_i_dn11 * assign13800_e18984);

        let assign13810_e18988: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard450 = assign13810_e18988;

        let (assign13820_e18996, assign13820_e18996_d_n4, assign13820_e18996_d_n5,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13820_e18993: f64 = (locals.var_tratio).powf(locals.var_ute_i);
        let assign13820_e18994: f64 = (locals.var_u0r_i * assign13820_e18993);
        (assign13820_e18994, (locals.var_u0r_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13820_e18993 * (locals.var_ute_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), (locals.var_u0r_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13820_e18993 * (locals.var_ute_i * (locals.var_tratio_dn5 / locals.var_tratio))) }),)
    } else {
        (locals.var_u0r_t, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5,)
    }
};
        locals.var_u0r_t = assign13820_e18996;
        locals.var_u0r_t_dn4 = assign13820_e18996_d_n4;
        locals.var_u0r_t_dn5 = assign13820_e18996_d_n5;

        let (assign13830_e19033, assign13830_e19033_d_n3, assign13830_e19033_d_n4, assign13830_e19033_d_n5, assign13830_e19033_d_n6, assign13830_e19033_d_n7, assign13830_e19033_d_n8, assign13830_e19033_d_n9, assign13830_e19033_d_n10, assign13830_e19033_d_n11,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13830_e19003: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13830_e19004: f64 = (1.0 + assign13830_e19003);
        let assign13830_e19006: f64 = (assign13830_e19004 - 1e-6);
        let assign13830_e19010: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13830_e19011: f64 = (1.0 + assign13830_e19010);
        let assign13830_e19013: f64 = (assign13830_e19011 - 1e-6);
        let assign13830_e19017: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13830_e19018: f64 = (1.0 + assign13830_e19017);
        let assign13830_e19020: f64 = (assign13830_e19018 - 1e-6);
        let assign13830_e19021: f64 = (assign13830_e19013 * assign13830_e19020);
        let assign13830_e19024: f64 = (4.0 * 0.001);
        let assign13830_e19026: f64 = (assign13830_e19024 * 0.001);
        let assign13830_e19027: f64 = (assign13830_e19021 + assign13830_e19026);
        let assign13830_e19028: f64 = (assign13830_e19027).sqrt();
        let assign13830_e19029: f64 = (assign13830_e19006 + assign13830_e19028);
        let assign13830_e19030: f64 = (0.5 * assign13830_e19029);
        let assign13830_e19031: f64 = (locals.var_uar_i * assign13830_e19030);
        (assign13830_e19031, (locals.var_uar_i_dn3 * assign13830_e19030), ((locals.var_uar_i_dn4 * assign13830_e19030) + (locals.var_uar_i * (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn4) + ((((locals.var_ua1_i * locals.var_deltemp_dn4) * assign13830_e19020) + (assign13830_e19013 * (locals.var_ua1_i * locals.var_deltemp_dn4))) / (2.0 * assign13830_e19028)))))), ((locals.var_uar_i_dn5 * assign13830_e19030) + (locals.var_uar_i * (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn5) + ((((locals.var_ua1_i * locals.var_deltemp_dn5) * assign13830_e19020) + (assign13830_e19013 * (locals.var_ua1_i * locals.var_deltemp_dn5))) / (2.0 * assign13830_e19028)))))), (locals.var_uar_i_dn6 * assign13830_e19030), (locals.var_uar_i_dn7 * assign13830_e19030), (locals.var_uar_i_dn8 * assign13830_e19030), (locals.var_uar_i_dn9 * assign13830_e19030), (locals.var_uar_i_dn10 * assign13830_e19030), (locals.var_uar_i_dn11 * assign13830_e19030),)
    } else {
        (locals.var_uar_t, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11,)
    }
};
        locals.var_uar_t = assign13830_e19033;
        locals.var_uar_t_dn3 = assign13830_e19033_d_n3;
        locals.var_uar_t_dn4 = assign13830_e19033_d_n4;
        locals.var_uar_t_dn5 = assign13830_e19033_d_n5;
        locals.var_uar_t_dn6 = assign13830_e19033_d_n6;
        locals.var_uar_t_dn7 = assign13830_e19033_d_n7;
        locals.var_uar_t_dn8 = assign13830_e19033_d_n8;
        locals.var_uar_t_dn9 = assign13830_e19033_d_n9;
        locals.var_uar_t_dn10 = assign13830_e19033_d_n10;
        locals.var_uar_t_dn11 = assign13830_e19033_d_n11;

        let (assign13840_e19070, assign13840_e19070_d_n3, assign13840_e19070_d_n4, assign13840_e19070_d_n5, assign13840_e19070_d_n6, assign13840_e19070_d_n7, assign13840_e19070_d_n8, assign13840_e19070_d_n9, assign13840_e19070_d_n10, assign13840_e19070_d_n11,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13840_e19040: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13840_e19041: f64 = (1.0 + assign13840_e19040);
        let assign13840_e19043: f64 = (assign13840_e19041 - 1e-6);
        let assign13840_e19047: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13840_e19048: f64 = (1.0 + assign13840_e19047);
        let assign13840_e19050: f64 = (assign13840_e19048 - 1e-6);
        let assign13840_e19054: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13840_e19055: f64 = (1.0 + assign13840_e19054);
        let assign13840_e19057: f64 = (assign13840_e19055 - 1e-6);
        let assign13840_e19058: f64 = (assign13840_e19050 * assign13840_e19057);
        let assign13840_e19061: f64 = (4.0 * 0.001);
        let assign13840_e19063: f64 = (assign13840_e19061 * 0.001);
        let assign13840_e19064: f64 = (assign13840_e19058 + assign13840_e19063);
        let assign13840_e19065: f64 = (assign13840_e19064).sqrt();
        let assign13840_e19066: f64 = (assign13840_e19043 + assign13840_e19065);
        let assign13840_e19067: f64 = (0.5 * assign13840_e19066);
        let assign13840_e19068: f64 = (locals.var_ucr_i * assign13840_e19067);
        (assign13840_e19068, (locals.var_ucr_i_dn3 * assign13840_e19067), ((locals.var_ucr_i_dn4 * assign13840_e19067) + (locals.var_ucr_i * (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn4) + ((((locals.var_uc1_i * locals.var_deltemp_dn4) * assign13840_e19057) + (assign13840_e19050 * (locals.var_uc1_i * locals.var_deltemp_dn4))) / (2.0 * assign13840_e19065)))))), ((locals.var_ucr_i_dn5 * assign13840_e19067) + (locals.var_ucr_i * (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn5) + ((((locals.var_uc1_i * locals.var_deltemp_dn5) * assign13840_e19057) + (assign13840_e19050 * (locals.var_uc1_i * locals.var_deltemp_dn5))) / (2.0 * assign13840_e19065)))))), (locals.var_ucr_i_dn6 * assign13840_e19067), (locals.var_ucr_i_dn7 * assign13840_e19067), (locals.var_ucr_i_dn8 * assign13840_e19067), (locals.var_ucr_i_dn9 * assign13840_e19067), (locals.var_ucr_i_dn10 * assign13840_e19067), (locals.var_ucr_i_dn11 * assign13840_e19067),)
    } else {
        (locals.var_ucr_t, locals.var_ucr_t_dn3, locals.var_ucr_t_dn4, locals.var_ucr_t_dn5, locals.var_ucr_t_dn6, locals.var_ucr_t_dn7, locals.var_ucr_t_dn8, locals.var_ucr_t_dn9, locals.var_ucr_t_dn10, locals.var_ucr_t_dn11,)
    }
};
        locals.var_ucr_t = assign13840_e19070;
        locals.var_ucr_t_dn3 = assign13840_e19070_d_n3;
        locals.var_ucr_t_dn4 = assign13840_e19070_d_n4;
        locals.var_ucr_t_dn5 = assign13840_e19070_d_n5;
        locals.var_ucr_t_dn6 = assign13840_e19070_d_n6;
        locals.var_ucr_t_dn7 = assign13840_e19070_d_n7;
        locals.var_ucr_t_dn8 = assign13840_e19070_d_n8;
        locals.var_ucr_t_dn9 = assign13840_e19070_d_n9;
        locals.var_ucr_t_dn10 = assign13840_e19070_d_n10;
        locals.var_ucr_t_dn11 = assign13840_e19070_d_n11;

        let (assign13850_e19078, assign13850_e19078_d_n3, assign13850_e19078_d_n4, assign13850_e19078_d_n5, assign13850_e19078_d_n6, assign13850_e19078_d_n7, assign13850_e19078_d_n8, assign13850_e19078_d_n9, assign13850_e19078_d_n10, assign13850_e19078_d_n11,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13850_e19075: f64 = (locals.var_tratio).powf(locals.var_ud1_i);
        let assign13850_e19076: f64 = (locals.var_udr_i * assign13850_e19075);
        (assign13850_e19076, (locals.var_udr_i_dn3 * assign13850_e19075), ((locals.var_udr_i_dn4 * assign13850_e19075) + (locals.var_udr_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13850_e19075 * (locals.var_ud1_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), ((locals.var_udr_i_dn5 * assign13850_e19075) + (locals.var_udr_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13850_e19075 * (locals.var_ud1_i * (locals.var_tratio_dn5 / locals.var_tratio))) })), (locals.var_udr_i_dn6 * assign13850_e19075), (locals.var_udr_i_dn7 * assign13850_e19075), (locals.var_udr_i_dn8 * assign13850_e19075), (locals.var_udr_i_dn9 * assign13850_e19075), (locals.var_udr_i_dn10 * assign13850_e19075), (locals.var_udr_i_dn11 * assign13850_e19075),)
    } else {
        (locals.var_udr_t, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11,)
    }
};
        locals.var_udr_t = assign13850_e19078;
        locals.var_udr_t_dn3 = assign13850_e19078_d_n3;
        locals.var_udr_t_dn4 = assign13850_e19078_d_n4;
        locals.var_udr_t_dn5 = assign13850_e19078_d_n5;
        locals.var_udr_t_dn6 = assign13850_e19078_d_n6;
        locals.var_udr_t_dn7 = assign13850_e19078_d_n7;
        locals.var_udr_t_dn8 = assign13850_e19078_d_n8;
        locals.var_udr_t_dn9 = assign13850_e19078_d_n9;
        locals.var_udr_t_dn10 = assign13850_e19078_d_n10;
        locals.var_udr_t_dn11 = assign13850_e19078_d_n11;

        let (assign13860_e19086, assign13860_e19086_d_n4, assign13860_e19086_d_n5,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13860_e19083: f64 = (locals.var_tratio).powf(locals.var_ucste_i);
        let assign13860_e19084: f64 = (locals.var_ucsr_i * assign13860_e19083);
        (assign13860_e19084, (locals.var_ucsr_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13860_e19083 * (locals.var_ucste_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), (locals.var_ucsr_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13860_e19083 * (locals.var_ucste_i * (locals.var_tratio_dn5 / locals.var_tratio))) }),)
    } else {
        (locals.var_ucsr_t, locals.var_ucsr_t_dn4, locals.var_ucsr_t_dn5,)
    }
};
        locals.var_ucsr_t = assign13860_e19086;
        locals.var_ucsr_t_dn4 = assign13860_e19086_d_n4;
        locals.var_ucsr_t_dn5 = assign13860_e19086_d_n5;

        let assign13870_e19089: f64 = (locals.var_tratio).powf(locals.var_prt_i);
        locals.var_rdstemp = assign13870_e19089;
        locals.var_rdstemp_dn4 = if 0.0 == 0.0 && ((locals.var_prt_i) as f64).is_finite() && ((locals.var_prt_i) as f64).fract() == 0.0 { if locals.var_prt_i == 0.0 { 0.0 } else { (locals.var_prt_i * ((locals.var_tratio).powf(locals.var_prt_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13870_e19089 * (locals.var_prt_i * (locals.var_tratio_dn4 / locals.var_tratio))) };
        locals.var_rdstemp_dn5 = if 0.0 == 0.0 && ((locals.var_prt_i) as f64).is_finite() && ((locals.var_prt_i) as f64).fract() == 0.0 { if locals.var_prt_i == 0.0 { 0.0 } else { (locals.var_prt_i * ((locals.var_tratio).powf(locals.var_prt_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13870_e19089 * (locals.var_prt_i * (locals.var_tratio_dn5 / locals.var_tratio))) };

        let assign13880_e19093: f64 = (-locals.var_at_i);
        let assign13880_e19094: f64 = (locals.var_tratio).powf(assign13880_e19093);
        let assign13880_e19095: f64 = (locals.var_vsat_i * assign13880_e19094);
        locals.var_vsat_t = assign13880_e19095;
        locals.var_vsat_t_dn3 = (locals.var_vsat_i_dn3 * assign13880_e19094);
        locals.var_vsat_t_dn4 = ((locals.var_vsat_i_dn4 * assign13880_e19094) + (locals.var_vsat_i * if 0.0 == 0.0 && ((assign13880_e19093) as f64).is_finite() && ((assign13880_e19093) as f64).fract() == 0.0 { if assign13880_e19093 == 0.0 { 0.0 } else { (assign13880_e19093 * ((locals.var_tratio).powf(assign13880_e19093 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13880_e19094 * (assign13880_e19093 * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_vsat_t_dn5 = ((locals.var_vsat_i_dn5 * assign13880_e19094) + (locals.var_vsat_i * if 0.0 == 0.0 && ((assign13880_e19093) as f64).is_finite() && ((assign13880_e19093) as f64).fract() == 0.0 { if assign13880_e19093 == 0.0 { 0.0 } else { (assign13880_e19093 * ((locals.var_tratio).powf(assign13880_e19093 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13880_e19094 * (assign13880_e19093 * (locals.var_tratio_dn5 / locals.var_tratio))) }));
        locals.var_vsat_t_dn6 = (locals.var_vsat_i_dn6 * assign13880_e19094);
        locals.var_vsat_t_dn7 = (locals.var_vsat_i_dn7 * assign13880_e19094);
        locals.var_vsat_t_dn8 = (locals.var_vsat_i_dn8 * assign13880_e19094);
        locals.var_vsat_t_dn9 = (locals.var_vsat_i_dn9 * assign13880_e19094);
        locals.var_vsat_t_dn10 = (locals.var_vsat_i_dn10 * assign13880_e19094);
        locals.var_vsat_t_dn11 = (locals.var_vsat_i_dn11 * assign13880_e19094);

        let assign13890_e19098: f64 = if locals.var_vsat_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard451 = assign13890_e19098;

    }

    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13900_e19102, assign13900_e19102_d_n3, assign13900_e19102_d_n4, assign13900_e19102_d_n5, assign13900_e19102_d_n6, assign13900_e19102_d_n7, assign13900_e19102_d_n8, assign13900_e19102_d_n9, assign13900_e19102_d_n10, assign13900_e19102_d_n11,) = {
    if (locals.var_guard451 != 0.0) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11,)
    }
};
        locals.var_vsat_t = assign13900_e19102;
        locals.var_vsat_t_dn3 = assign13900_e19102_d_n3;
        locals.var_vsat_t_dn4 = assign13900_e19102_d_n4;
        locals.var_vsat_t_dn5 = assign13900_e19102_d_n5;
        locals.var_vsat_t_dn6 = assign13900_e19102_d_n6;
        locals.var_vsat_t_dn7 = assign13900_e19102_d_n7;
        locals.var_vsat_t_dn8 = assign13900_e19102_d_n8;
        locals.var_vsat_t_dn9 = assign13900_e19102_d_n9;
        locals.var_vsat_t_dn10 = assign13900_e19102_d_n10;
        locals.var_vsat_t_dn11 = assign13900_e19102_d_n11;

        let assign13910_e19105: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard452 = assign13910_e19105;

        let (assign13920_e19114, assign13920_e19114_d_n3, assign13920_e19114_d_n4, assign13920_e19114_d_n5, assign13920_e19114_d_n6, assign13920_e19114_d_n7, assign13920_e19114_d_n8, assign13920_e19114_d_n9, assign13920_e19114_d_n10, assign13920_e19114_d_n11,) = {
    if (locals.var_guard452 != 0.0) {
        let assign13920_e19110: f64 = (-locals.var_at_i);
        let assign13920_e19111: f64 = (locals.var_tratio).powf(assign13920_e19110);
        let assign13920_e19112: f64 = (locals.var_vsatr_i * assign13920_e19111);
        (assign13920_e19112, (locals.var_vsatr_i_dn3 * assign13920_e19111), ((locals.var_vsatr_i_dn4 * assign13920_e19111) + (locals.var_vsatr_i * if 0.0 == 0.0 && ((assign13920_e19110) as f64).is_finite() && ((assign13920_e19110) as f64).fract() == 0.0 { if assign13920_e19110 == 0.0 { 0.0 } else { (assign13920_e19110 * ((locals.var_tratio).powf(assign13920_e19110 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13920_e19111 * (assign13920_e19110 * (locals.var_tratio_dn4 / locals.var_tratio))) })), ((locals.var_vsatr_i_dn5 * assign13920_e19111) + (locals.var_vsatr_i * if 0.0 == 0.0 && ((assign13920_e19110) as f64).is_finite() && ((assign13920_e19110) as f64).fract() == 0.0 { if assign13920_e19110 == 0.0 { 0.0 } else { (assign13920_e19110 * ((locals.var_tratio).powf(assign13920_e19110 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13920_e19111 * (assign13920_e19110 * (locals.var_tratio_dn5 / locals.var_tratio))) })), (locals.var_vsatr_i_dn6 * assign13920_e19111), (locals.var_vsatr_i_dn7 * assign13920_e19111), (locals.var_vsatr_i_dn8 * assign13920_e19111), (locals.var_vsatr_i_dn9 * assign13920_e19111), (locals.var_vsatr_i_dn10 * assign13920_e19111), (locals.var_vsatr_i_dn11 * assign13920_e19111),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11,)
    }
};
        locals.var_vsatr_t = assign13920_e19114;
        locals.var_vsatr_t_dn3 = assign13920_e19114_d_n3;
        locals.var_vsatr_t_dn4 = assign13920_e19114_d_n4;
        locals.var_vsatr_t_dn5 = assign13920_e19114_d_n5;
        locals.var_vsatr_t_dn6 = assign13920_e19114_d_n6;
        locals.var_vsatr_t_dn7 = assign13920_e19114_d_n7;
        locals.var_vsatr_t_dn8 = assign13920_e19114_d_n8;
        locals.var_vsatr_t_dn9 = assign13920_e19114_d_n9;
        locals.var_vsatr_t_dn10 = assign13920_e19114_d_n10;
        locals.var_vsatr_t_dn11 = assign13920_e19114_d_n11;

        let assign13930_e19117: f64 = if locals.var_vsatr_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard453 = assign13930_e19117;

        let (assign13940_e19123, assign13940_e19123_d_n3, assign13940_e19123_d_n4, assign13940_e19123_d_n5, assign13940_e19123_d_n6, assign13940_e19123_d_n7, assign13940_e19123_d_n8, assign13940_e19123_d_n9, assign13940_e19123_d_n10, assign13940_e19123_d_n11,) = {
    if ((locals.var_guard452 != 0.0) && (locals.var_guard453 != 0.0)) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11,)
    }
};
        locals.var_vsatr_t = assign13940_e19123;
        locals.var_vsatr_t_dn3 = assign13940_e19123_d_n3;
        locals.var_vsatr_t_dn4 = assign13940_e19123_d_n4;
        locals.var_vsatr_t_dn5 = assign13940_e19123_d_n5;
        locals.var_vsatr_t_dn6 = assign13940_e19123_d_n6;
        locals.var_vsatr_t_dn7 = assign13940_e19123_d_n7;
        locals.var_vsatr_t_dn8 = assign13940_e19123_d_n8;
        locals.var_vsatr_t_dn9 = assign13940_e19123_d_n9;
        locals.var_vsatr_t_dn10 = assign13940_e19123_d_n10;
        locals.var_vsatr_t_dn11 = assign13940_e19123_d_n11;

        let assign13950_e19127: f64 = (-locals.var_at_i);
        let assign13950_e19128: f64 = (locals.var_tratio).powf(assign13950_e19127);
        let assign13950_e19129: f64 = (locals.var_vsatcv_i * assign13950_e19128);
        locals.var_vsatcv_t = assign13950_e19129;
        locals.var_vsatcv_t_dn3 = (locals.var_vsatcv_i_dn3 * assign13950_e19128);
        locals.var_vsatcv_t_dn4 = ((locals.var_vsatcv_i_dn4 * assign13950_e19128) + (locals.var_vsatcv_i * if 0.0 == 0.0 && ((assign13950_e19127) as f64).is_finite() && ((assign13950_e19127) as f64).fract() == 0.0 { if assign13950_e19127 == 0.0 { 0.0 } else { (assign13950_e19127 * ((locals.var_tratio).powf(assign13950_e19127 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13950_e19128 * (assign13950_e19127 * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_vsatcv_t_dn5 = ((locals.var_vsatcv_i_dn5 * assign13950_e19128) + (locals.var_vsatcv_i * if 0.0 == 0.0 && ((assign13950_e19127) as f64).is_finite() && ((assign13950_e19127) as f64).fract() == 0.0 { if assign13950_e19127 == 0.0 { 0.0 } else { (assign13950_e19127 * ((locals.var_tratio).powf(assign13950_e19127 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13950_e19128 * (assign13950_e19127 * (locals.var_tratio_dn5 / locals.var_tratio))) }));
        locals.var_vsatcv_t_dn6 = (locals.var_vsatcv_i_dn6 * assign13950_e19128);
        locals.var_vsatcv_t_dn7 = (locals.var_vsatcv_i_dn7 * assign13950_e19128);
        locals.var_vsatcv_t_dn8 = (locals.var_vsatcv_i_dn8 * assign13950_e19128);
        locals.var_vsatcv_t_dn9 = (locals.var_vsatcv_i_dn9 * assign13950_e19128);
        locals.var_vsatcv_t_dn10 = (locals.var_vsatcv_i_dn10 * assign13950_e19128);
        locals.var_vsatcv_t_dn11 = (locals.var_vsatcv_i_dn11 * assign13950_e19128);

        let assign13960_e19132: f64 = if locals.var_vsatcv_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard454 = assign13960_e19132;

        let (assign13970_e19136, assign13970_e19136_d_n3, assign13970_e19136_d_n4, assign13970_e19136_d_n5, assign13970_e19136_d_n6, assign13970_e19136_d_n7, assign13970_e19136_d_n8, assign13970_e19136_d_n9, assign13970_e19136_d_n10, assign13970_e19136_d_n11,) = {
    if (locals.var_guard454 != 0.0) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11,)
    }
};
        locals.var_vsatcv_t = assign13970_e19136;
        locals.var_vsatcv_t_dn3 = assign13970_e19136_d_n3;
        locals.var_vsatcv_t_dn4 = assign13970_e19136_d_n4;
        locals.var_vsatcv_t_dn5 = assign13970_e19136_d_n5;
        locals.var_vsatcv_t_dn6 = assign13970_e19136_d_n6;
        locals.var_vsatcv_t_dn7 = assign13970_e19136_d_n7;
        locals.var_vsatcv_t_dn8 = assign13970_e19136_d_n8;
        locals.var_vsatcv_t_dn9 = assign13970_e19136_d_n9;
        locals.var_vsatcv_t_dn10 = assign13970_e19136_d_n10;
        locals.var_vsatcv_t_dn11 = assign13970_e19136_d_n11;

        let assign13980_e19141: f64 = (1.0 / locals.var_delta_i);
        let assign13980_e19145: f64 = (p.p1069 * locals.var_deltemp);
        let assign13980_e19146: f64 = (1.0 + assign13980_e19145);
        let assign13980_e19147: f64 = (assign13980_e19141 * assign13980_e19146);
        let assign13980_e19149: f64 = (assign13980_e19147 - 2.0);
        let assign13980_e19152: f64 = (1.0 / locals.var_delta_i);
        let assign13980_e19156: f64 = (p.p1069 * locals.var_deltemp);
        let assign13980_e19157: f64 = (1.0 + assign13980_e19156);
        let assign13980_e19158: f64 = (assign13980_e19152 * assign13980_e19157);
        let assign13980_e19160: f64 = (assign13980_e19158 - 2.0);
        let assign13980_e19163: f64 = (1.0 / locals.var_delta_i);
        let assign13980_e19167: f64 = (p.p1069 * locals.var_deltemp);
        let assign13980_e19168: f64 = (1.0 + assign13980_e19167);
        let assign13980_e19169: f64 = (assign13980_e19163 * assign13980_e19168);
        let assign13980_e19171: f64 = (assign13980_e19169 - 2.0);
        let assign13980_e19172: f64 = (assign13980_e19160 * assign13980_e19171);
        let assign13980_e19175: f64 = (4.0 * 0.001);
        let assign13980_e19177: f64 = (assign13980_e19175 * 0.001);
        let assign13980_e19178: f64 = (assign13980_e19172 + assign13980_e19177);
        let assign13980_e19179: f64 = (assign13980_e19178).sqrt();
        let assign13980_e19180: f64 = (assign13980_e19149 + assign13980_e19179);
        let assign13980_e19181: f64 = (0.5 * assign13980_e19180);
        let assign13980_e19183: f64 = (assign13980_e19181 + 2.0);
        let assign13980_e19184: f64 = (1.0 / assign13980_e19183);
        locals.var_delta_t = assign13980_e19184;
        locals.var_delta_t_dn3 = (-((0.5 * (((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn4 = (-((0.5 * ((((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (assign13980_e19141 * (p.p1069 * locals.var_deltemp_dn4))) + ((((((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) + (assign13980_e19152 * (p.p1069 * locals.var_deltemp_dn4))) * assign13980_e19171) + (assign13980_e19160 * (((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168) + (assign13980_e19163 * (p.p1069 * locals.var_deltemp_dn4))))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn5 = (-((0.5 * ((((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (assign13980_e19141 * (p.p1069 * locals.var_deltemp_dn5))) + ((((((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) + (assign13980_e19152 * (p.p1069 * locals.var_deltemp_dn5))) * assign13980_e19171) + (assign13980_e19160 * (((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168) + (assign13980_e19163 * (p.p1069 * locals.var_deltemp_dn5))))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn6 = (-((0.5 * (((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn7 = (-((0.5 * (((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn8 = (-((0.5 * (((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn9 = (-((0.5 * (((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn10 = (-((0.5 * (((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn11 = (-((0.5 * (((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));

        let assign13990_e19190: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13990_e19191: f64 = (1.0 - assign13990_e19190);
        let assign13990_e19193: f64 = (assign13990_e19191 - 1e-6);
        let assign13990_e19197: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13990_e19198: f64 = (1.0 - assign13990_e19197);
        let assign13990_e19200: f64 = (assign13990_e19198 - 1e-6);
        let assign13990_e19204: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13990_e19205: f64 = (1.0 - assign13990_e19204);
        let assign13990_e19207: f64 = (assign13990_e19205 - 1e-6);
        let assign13990_e19208: f64 = (assign13990_e19200 * assign13990_e19207);
        let assign13990_e19211: f64 = (4.0 * 0.001);
        let assign13990_e19213: f64 = (assign13990_e19211 * 0.001);
        let assign13990_e19214: f64 = (assign13990_e19208 + assign13990_e19213);
        let assign13990_e19215: f64 = (assign13990_e19214).sqrt();
        let assign13990_e19216: f64 = (assign13990_e19193 + assign13990_e19215);
        let assign13990_e19217: f64 = (0.5 * assign13990_e19216);
        let assign13990_e19218: f64 = (locals.var_ptwg_i * assign13990_e19217);
        locals.var_ptwg_t = assign13990_e19218;
        locals.var_ptwg_t_dn3 = (locals.var_ptwg_i_dn3 * assign13990_e19217);
        locals.var_ptwg_t_dn4 = ((locals.var_ptwg_i_dn4 * assign13990_e19217) + (locals.var_ptwg_i * (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) * assign13990_e19207) + (assign13990_e19200 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4)))) / (2.0 * assign13990_e19215))))));
        locals.var_ptwg_t_dn5 = ((locals.var_ptwg_i_dn5 * assign13990_e19217) + (locals.var_ptwg_i * (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn5)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn5)) * assign13990_e19207) + (assign13990_e19200 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn5)))) / (2.0 * assign13990_e19215))))));
        locals.var_ptwg_t_dn6 = (locals.var_ptwg_i_dn6 * assign13990_e19217);
        locals.var_ptwg_t_dn7 = (locals.var_ptwg_i_dn7 * assign13990_e19217);
        locals.var_ptwg_t_dn8 = (locals.var_ptwg_i_dn8 * assign13990_e19217);
        locals.var_ptwg_t_dn9 = (locals.var_ptwg_i_dn9 * assign13990_e19217);
        locals.var_ptwg_t_dn10 = (locals.var_ptwg_i_dn10 * assign13990_e19217);
        locals.var_ptwg_t_dn11 = (locals.var_ptwg_i_dn11 * assign13990_e19217);

        let assign14000_e19221: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard455 = assign14000_e19221;

        let (assign14010_e19258, assign14010_e19258_d_n3, assign14010_e19258_d_n4, assign14010_e19258_d_n5, assign14010_e19258_d_n6, assign14010_e19258_d_n7, assign14010_e19258_d_n8, assign14010_e19258_d_n9, assign14010_e19258_d_n10, assign14010_e19258_d_n11,) = {
    if (locals.var_guard455 != 0.0) {
        let assign14010_e19228: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign14010_e19229: f64 = (1.0 - assign14010_e19228);
        let assign14010_e19231: f64 = (assign14010_e19229 - 1e-6);
        let assign14010_e19235: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign14010_e19236: f64 = (1.0 - assign14010_e19235);
        let assign14010_e19238: f64 = (assign14010_e19236 - 1e-6);
        let assign14010_e19242: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign14010_e19243: f64 = (1.0 - assign14010_e19242);
        let assign14010_e19245: f64 = (assign14010_e19243 - 1e-6);
        let assign14010_e19246: f64 = (assign14010_e19238 * assign14010_e19245);
        let assign14010_e19249: f64 = (4.0 * 0.001);
        let assign14010_e19251: f64 = (assign14010_e19249 * 0.001);
        let assign14010_e19252: f64 = (assign14010_e19246 + assign14010_e19251);
        let assign14010_e19253: f64 = (assign14010_e19252).sqrt();
        let assign14010_e19254: f64 = (assign14010_e19231 + assign14010_e19253);
        let assign14010_e19255: f64 = (0.5 * assign14010_e19254);
        let assign14010_e19256: f64 = (locals.var_ptwgr_i * assign14010_e19255);
        (assign14010_e19256, (locals.var_ptwgr_i_dn3 * assign14010_e19255), ((locals.var_ptwgr_i_dn4 * assign14010_e19255) + (locals.var_ptwgr_i * (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) * assign14010_e19245) + (assign14010_e19238 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4)))) / (2.0 * assign14010_e19253)))))), ((locals.var_ptwgr_i_dn5 * assign14010_e19255) + (locals.var_ptwgr_i * (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn5)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn5)) * assign14010_e19245) + (assign14010_e19238 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn5)))) / (2.0 * assign14010_e19253)))))), (locals.var_ptwgr_i_dn6 * assign14010_e19255), (locals.var_ptwgr_i_dn7 * assign14010_e19255), (locals.var_ptwgr_i_dn8 * assign14010_e19255), (locals.var_ptwgr_i_dn9 * assign14010_e19255), (locals.var_ptwgr_i_dn10 * assign14010_e19255), (locals.var_ptwgr_i_dn11 * assign14010_e19255),)
    } else {
        (locals.var_ptwgr_t, locals.var_ptwgr_t_dn3, locals.var_ptwgr_t_dn4, locals.var_ptwgr_t_dn5, locals.var_ptwgr_t_dn6, locals.var_ptwgr_t_dn7, locals.var_ptwgr_t_dn8, locals.var_ptwgr_t_dn9, locals.var_ptwgr_t_dn10, locals.var_ptwgr_t_dn11,)
    }
};
        locals.var_ptwgr_t = assign14010_e19258;
        locals.var_ptwgr_t_dn3 = assign14010_e19258_d_n3;
        locals.var_ptwgr_t_dn4 = assign14010_e19258_d_n4;
        locals.var_ptwgr_t_dn5 = assign14010_e19258_d_n5;
        locals.var_ptwgr_t_dn6 = assign14010_e19258_d_n6;
        locals.var_ptwgr_t_dn7 = assign14010_e19258_d_n7;
        locals.var_ptwgr_t_dn8 = assign14010_e19258_d_n8;
        locals.var_ptwgr_t_dn9 = assign14010_e19258_d_n9;
        locals.var_ptwgr_t_dn10 = assign14010_e19258_d_n10;
        locals.var_ptwgr_t_dn11 = assign14010_e19258_d_n11;

        let assign14020_e19264: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign14020_e19265: f64 = (1.0 + assign14020_e19264);
        let assign14020_e19267: f64 = (assign14020_e19265 - 1e-6);
        let assign14020_e19271: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign14020_e19272: f64 = (1.0 + assign14020_e19271);
        let assign14020_e19274: f64 = (assign14020_e19272 - 1e-6);
        let assign14020_e19278: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign14020_e19279: f64 = (1.0 + assign14020_e19278);
        let assign14020_e19281: f64 = (assign14020_e19279 - 1e-6);
        let assign14020_e19282: f64 = (assign14020_e19274 * assign14020_e19281);
        let assign14020_e19285: f64 = (4.0 * 0.001);
        let assign14020_e19287: f64 = (assign14020_e19285 * 0.001);
        let assign14020_e19288: f64 = (assign14020_e19282 + assign14020_e19287);
        let assign14020_e19289: f64 = (assign14020_e19288).sqrt();
        let assign14020_e19290: f64 = (assign14020_e19267 + assign14020_e19289);
        let assign14020_e19291: f64 = (0.5 * assign14020_e19290);
        let assign14020_e19292: f64 = (locals.var_a1_i * assign14020_e19291);
        locals.var_a1_t = assign14020_e19292;
        locals.var_a1_t_dn4 = (locals.var_a1_i * (0.5 * ((locals.var_a11_i * locals.var_deltemp_dn4) + ((((locals.var_a11_i * locals.var_deltemp_dn4) * assign14020_e19281) + (assign14020_e19274 * (locals.var_a11_i * locals.var_deltemp_dn4))) / (2.0 * assign14020_e19289)))));
        locals.var_a1_t_dn5 = (locals.var_a1_i * (0.5 * ((locals.var_a11_i * locals.var_deltemp_dn5) + ((((locals.var_a11_i * locals.var_deltemp_dn5) * assign14020_e19281) + (assign14020_e19274 * (locals.var_a11_i * locals.var_deltemp_dn5))) / (2.0 * assign14020_e19289)))));

        let assign14030_e19298: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign14030_e19299: f64 = (1.0 + assign14030_e19298);
        let assign14030_e19301: f64 = (assign14030_e19299 - 1e-6);
        let assign14030_e19305: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign14030_e19306: f64 = (1.0 + assign14030_e19305);
        let assign14030_e19308: f64 = (assign14030_e19306 - 1e-6);
        let assign14030_e19312: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign14030_e19313: f64 = (1.0 + assign14030_e19312);
        let assign14030_e19315: f64 = (assign14030_e19313 - 1e-6);
        let assign14030_e19316: f64 = (assign14030_e19308 * assign14030_e19315);
        let assign14030_e19319: f64 = (4.0 * 0.001);
        let assign14030_e19321: f64 = (assign14030_e19319 * 0.001);
        let assign14030_e19322: f64 = (assign14030_e19316 + assign14030_e19321);
        let assign14030_e19323: f64 = (assign14030_e19322).sqrt();
        let assign14030_e19324: f64 = (assign14030_e19301 + assign14030_e19323);
        let assign14030_e19325: f64 = (0.5 * assign14030_e19324);
        let assign14030_e19326: f64 = (locals.var_a2_i * assign14030_e19325);
        locals.var_a2_t = assign14030_e19326;
        locals.var_a2_t_dn4 = (locals.var_a2_i * (0.5 * ((locals.var_a21_i * locals.var_deltemp_dn4) + ((((locals.var_a21_i * locals.var_deltemp_dn4) * assign14030_e19315) + (assign14030_e19308 * (locals.var_a21_i * locals.var_deltemp_dn4))) / (2.0 * assign14030_e19323)))));
        locals.var_a2_t_dn5 = (locals.var_a2_i * (0.5 * ((locals.var_a21_i * locals.var_deltemp_dn5) + ((((locals.var_a21_i * locals.var_deltemp_dn5) * assign14030_e19315) + (assign14030_e19308 * (locals.var_a21_i * locals.var_deltemp_dn5))) / (2.0 * assign14030_e19323)))));

        let assign14040_e19330: f64 = (locals.var_tratio).powf(locals.var_iit_i);
        let assign14040_e19331: f64 = (locals.var_beta0_i * assign14040_e19330);
        locals.var_beta0_t = assign14040_e19331;
        locals.var_beta0_t_dn4 = (locals.var_beta0_i * if 0.0 == 0.0 && ((locals.var_iit_i) as f64).is_finite() && ((locals.var_iit_i) as f64).fract() == 0.0 { if locals.var_iit_i == 0.0 { 0.0 } else { (locals.var_iit_i * ((locals.var_tratio).powf(locals.var_iit_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign14040_e19330 * (locals.var_iit_i * (locals.var_tratio_dn4 / locals.var_tratio))) });
        locals.var_beta0_t_dn5 = (locals.var_beta0_i * if 0.0 == 0.0 && ((locals.var_iit_i) as f64).is_finite() && ((locals.var_iit_i) as f64).fract() == 0.0 { if locals.var_iit_i == 0.0 { 0.0 } else { (locals.var_iit_i * ((locals.var_tratio).powf(locals.var_iit_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign14040_e19330 * (locals.var_iit_i * (locals.var_tratio_dn5 / locals.var_tratio))) });

        let assign14050_e19336: f64 = (locals.var_tratio - 1.0);
        let assign14050_e19337: f64 = (locals.var_bgidl1_i * assign14050_e19336);
        let assign14050_e19338: f64 = (locals.var_bgidl_i + assign14050_e19337);
        locals.var_bgidl_t = assign14050_e19338;
        locals.var_bgidl_t_dn4 = (locals.var_bgidl1_i * locals.var_tratio_dn4);
        locals.var_bgidl_t_dn5 = (locals.var_bgidl1_i * locals.var_tratio_dn5);

        let assign14060_e19343: f64 = (locals.var_tratio - 1.0);
        let assign14060_e19344: f64 = (locals.var_bgisl1_i * assign14060_e19343);
        let assign14060_e19345: f64 = (locals.var_bgisl_i + assign14060_e19344);
        locals.var_bgisl_t = assign14060_e19345;
        locals.var_bgisl_t_dn4 = (locals.var_bgisl1_i * locals.var_tratio_dn4);
        locals.var_bgisl_t_dn5 = (locals.var_bgisl1_i * locals.var_tratio_dn5);

        let assign14070_e19349: f64 = (locals.var_tratio).max(1e-38);
        let assign14070_e19350: f64 = (assign14070_e19349).ln();
        let assign14070_e19351: f64 = (locals.var_igt_i * assign14070_e19350);
        let assign14070_e19352: f64 = { let limited_exp_arg = assign14070_e19351; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        locals.var_igtemp = assign14070_e19352;
        locals.var_igtemp_dn4 = ({ let limited_exp_arg = assign14070_e19351; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_igt_i * (if locals.var_tratio >= 1e-38 { locals.var_tratio_dn4 } else { 0.0 } / assign14070_e19349)));
        locals.var_igtemp_dn5 = ({ let limited_exp_arg = assign14070_e19351; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_igt_i * (if locals.var_tratio >= 1e-38 { locals.var_tratio_dn5 } else { 0.0 } / assign14070_e19349)));

        let assign14080_e19358: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign14080_e19359: f64 = (1.0 + assign14080_e19358);
        let assign14080_e19361: f64 = (assign14080_e19359 - 1e-6);
        let assign14080_e19365: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign14080_e19366: f64 = (1.0 + assign14080_e19365);
        let assign14080_e19368: f64 = (assign14080_e19366 - 1e-6);
        let assign14080_e19372: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign14080_e19373: f64 = (1.0 + assign14080_e19372);
        let assign14080_e19375: f64 = (assign14080_e19373 - 1e-6);
        let assign14080_e19376: f64 = (assign14080_e19368 * assign14080_e19375);
        let assign14080_e19379: f64 = (4.0 * 0.001);
        let assign14080_e19381: f64 = (assign14080_e19379 * 0.001);
        let assign14080_e19382: f64 = (assign14080_e19376 + assign14080_e19381);
        let assign14080_e19383: f64 = (assign14080_e19382).sqrt();
        let assign14080_e19384: f64 = (assign14080_e19361 + assign14080_e19383);
        let assign14080_e19385: f64 = (0.5 * assign14080_e19384);
        let assign14080_e19386: f64 = (locals.var_k0_i * assign14080_e19385);
        locals.var_k0_t = assign14080_e19386;
        locals.var_k0_t_dn4 = (locals.var_k0_i * (0.5 * ((locals.var_k01_i * locals.var_deltemp_dn4) + ((((locals.var_k01_i * locals.var_deltemp_dn4) * assign14080_e19375) + (assign14080_e19368 * (locals.var_k01_i * locals.var_deltemp_dn4))) / (2.0 * assign14080_e19383)))));
        locals.var_k0_t_dn5 = (locals.var_k0_i * (0.5 * ((locals.var_k01_i * locals.var_deltemp_dn5) + ((((locals.var_k01_i * locals.var_deltemp_dn5) * assign14080_e19375) + (assign14080_e19368 * (locals.var_k01_i * locals.var_deltemp_dn5))) / (2.0 * assign14080_e19383)))));

        let assign14090_e19392: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign14090_e19393: f64 = (1.0 + assign14090_e19392);
        let assign14090_e19395: f64 = (assign14090_e19393 - 1e-6);
        let assign14090_e19399: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign14090_e19400: f64 = (1.0 + assign14090_e19399);
        let assign14090_e19402: f64 = (assign14090_e19400 - 1e-6);
        let assign14090_e19406: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign14090_e19407: f64 = (1.0 + assign14090_e19406);
        let assign14090_e19409: f64 = (assign14090_e19407 - 1e-6);
        let assign14090_e19410: f64 = (assign14090_e19402 * assign14090_e19409);
        let assign14090_e19413: f64 = (4.0 * 0.001);
        let assign14090_e19415: f64 = (assign14090_e19413 * 0.001);
        let assign14090_e19416: f64 = (assign14090_e19410 + assign14090_e19415);
        let assign14090_e19417: f64 = (assign14090_e19416).sqrt();
        let assign14090_e19418: f64 = (assign14090_e19395 + assign14090_e19417);
        let assign14090_e19419: f64 = (0.5 * assign14090_e19418);
        let assign14090_e19420: f64 = (locals.var_m0_i * assign14090_e19419);
        locals.var_m0_t = assign14090_e19420;
        locals.var_m0_t_dn4 = (locals.var_m0_i * (0.5 * ((locals.var_m01_i * locals.var_deltemp_dn4) + ((((locals.var_m01_i * locals.var_deltemp_dn4) * assign14090_e19409) + (assign14090_e19402 * (locals.var_m01_i * locals.var_deltemp_dn4))) / (2.0 * assign14090_e19417)))));
        locals.var_m0_t_dn5 = (locals.var_m0_i * (0.5 * ((locals.var_m01_i * locals.var_deltemp_dn5) + ((((locals.var_m01_i * locals.var_deltemp_dn5) * assign14090_e19409) + (assign14090_e19402 * (locals.var_m01_i * locals.var_deltemp_dn5))) / (2.0 * assign14090_e19417)))));

        let assign14100_e19426: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign14100_e19427: f64 = (1.0 + assign14100_e19426);
        let assign14100_e19429: f64 = (assign14100_e19427 - 1e-6);
        let assign14100_e19433: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign14100_e19434: f64 = (1.0 + assign14100_e19433);
        let assign14100_e19436: f64 = (assign14100_e19434 - 1e-6);
        let assign14100_e19440: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign14100_e19441: f64 = (1.0 + assign14100_e19440);
        let assign14100_e19443: f64 = (assign14100_e19441 - 1e-6);
        let assign14100_e19444: f64 = (assign14100_e19436 * assign14100_e19443);
        let assign14100_e19447: f64 = (4.0 * 0.001);
        let assign14100_e19449: f64 = (assign14100_e19447 * 0.001);
        let assign14100_e19450: f64 = (assign14100_e19444 + assign14100_e19449);
        let assign14100_e19451: f64 = (assign14100_e19450).sqrt();
        let assign14100_e19452: f64 = (assign14100_e19429 + assign14100_e19451);
        let assign14100_e19453: f64 = (0.5 * assign14100_e19452);
        let assign14100_e19454: f64 = (locals.var_c0_i * assign14100_e19453);
        locals.var_c0_t = assign14100_e19454;
        locals.var_c0_t_dn4 = (locals.var_c0_i * (0.5 * ((locals.var_c01_i * locals.var_deltemp_dn4) + ((((locals.var_c01_i * locals.var_deltemp_dn4) * assign14100_e19443) + (assign14100_e19436 * (locals.var_c01_i * locals.var_deltemp_dn4))) / (2.0 * assign14100_e19451)))));
        locals.var_c0_t_dn5 = (locals.var_c0_i * (0.5 * ((locals.var_c01_i * locals.var_deltemp_dn5) + ((((locals.var_c01_i * locals.var_deltemp_dn5) * assign14100_e19443) + (assign14100_e19436 * (locals.var_c01_i * locals.var_deltemp_dn5))) / (2.0 * assign14100_e19451)))));

        let assign14110_e19460: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign14110_e19461: f64 = (1.0 + assign14110_e19460);
        let assign14110_e19463: f64 = (assign14110_e19461 - 1e-6);
        let assign14110_e19467: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign14110_e19468: f64 = (1.0 + assign14110_e19467);
        let assign14110_e19470: f64 = (assign14110_e19468 - 1e-6);
        let assign14110_e19474: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign14110_e19475: f64 = (1.0 + assign14110_e19474);
        let assign14110_e19477: f64 = (assign14110_e19475 - 1e-6);
        let assign14110_e19478: f64 = (assign14110_e19470 * assign14110_e19477);
        let assign14110_e19481: f64 = (4.0 * 0.001);
        let assign14110_e19483: f64 = (assign14110_e19481 * 0.001);
        let assign14110_e19484: f64 = (assign14110_e19478 + assign14110_e19483);
        let assign14110_e19485: f64 = (assign14110_e19484).sqrt();
        let assign14110_e19486: f64 = (assign14110_e19463 + assign14110_e19485);
        let assign14110_e19487: f64 = (0.5 * assign14110_e19486);
        let assign14110_e19488: f64 = (locals.var_c0si_i * assign14110_e19487);
        locals.var_c0si_t = assign14110_e19488;
        locals.var_c0si_t_dn4 = (locals.var_c0si_i * (0.5 * ((locals.var_c0si1_i * locals.var_deltemp_dn4) + ((((locals.var_c0si1_i * locals.var_deltemp_dn4) * assign14110_e19477) + (assign14110_e19470 * (locals.var_c0si1_i * locals.var_deltemp_dn4))) / (2.0 * assign14110_e19485)))));
        locals.var_c0si_t_dn5 = (locals.var_c0si_i * (0.5 * ((locals.var_c0si1_i * locals.var_deltemp_dn5) + ((((locals.var_c0si1_i * locals.var_deltemp_dn5) * assign14110_e19477) + (assign14110_e19470 * (locals.var_c0si1_i * locals.var_deltemp_dn5))) / (2.0 * assign14110_e19485)))));

        let assign14120_e19494: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign14120_e19495: f64 = (1.0 + assign14120_e19494);
        let assign14120_e19497: f64 = (assign14120_e19495 - 1e-6);
        let assign14120_e19501: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign14120_e19502: f64 = (1.0 + assign14120_e19501);
        let assign14120_e19504: f64 = (assign14120_e19502 - 1e-6);
        let assign14120_e19508: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign14120_e19509: f64 = (1.0 + assign14120_e19508);
        let assign14120_e19511: f64 = (assign14120_e19509 - 1e-6);
        let assign14120_e19512: f64 = (assign14120_e19504 * assign14120_e19511);
        let assign14120_e19515: f64 = (4.0 * 0.001);
        let assign14120_e19517: f64 = (assign14120_e19515 * 0.001);
        let assign14120_e19518: f64 = (assign14120_e19512 + assign14120_e19517);
        let assign14120_e19519: f64 = (assign14120_e19518).sqrt();
        let assign14120_e19520: f64 = (assign14120_e19497 + assign14120_e19519);
        let assign14120_e19521: f64 = (0.5 * assign14120_e19520);
        let assign14120_e19522: f64 = (locals.var_c0sisat_i * assign14120_e19521);
        locals.var_c0sisat_t = assign14120_e19522;
        locals.var_c0sisat_t_dn4 = (locals.var_c0sisat_i * (0.5 * ((locals.var_c0sisat1_i * locals.var_deltemp_dn4) + ((((locals.var_c0sisat1_i * locals.var_deltemp_dn4) * assign14120_e19511) + (assign14120_e19504 * (locals.var_c0sisat1_i * locals.var_deltemp_dn4))) / (2.0 * assign14120_e19519)))));
        locals.var_c0sisat_t_dn5 = (locals.var_c0sisat_i * (0.5 * ((locals.var_c0sisat1_i * locals.var_deltemp_dn5) + ((((locals.var_c0sisat1_i * locals.var_deltemp_dn5) * assign14120_e19511) + (assign14120_e19504 * (locals.var_c0sisat1_i * locals.var_deltemp_dn5))) / (2.0 * assign14120_e19519)))));

        let assign14130_e19528: f64 = (p.p1093 * locals.var_deltemp);
        let assign14130_e19529: f64 = (1.0 + assign14130_e19528);
        let assign14130_e19531: f64 = (assign14130_e19529 - 1e-6);
        let assign14130_e19535: f64 = (p.p1093 * locals.var_deltemp);
        let assign14130_e19536: f64 = (1.0 + assign14130_e19535);
        let assign14130_e19538: f64 = (assign14130_e19536 - 1e-6);
        let assign14130_e19542: f64 = (p.p1093 * locals.var_deltemp);
        let assign14130_e19543: f64 = (1.0 + assign14130_e19542);
        let assign14130_e19545: f64 = (assign14130_e19543 - 1e-6);
        let assign14130_e19546: f64 = (assign14130_e19538 * assign14130_e19545);
        let assign14130_e19549: f64 = (4.0 * 0.001);
        let assign14130_e19551: f64 = (assign14130_e19549 * 0.001);
        let assign14130_e19552: f64 = (assign14130_e19546 + assign14130_e19551);
        let assign14130_e19553: f64 = (assign14130_e19552).sqrt();
        let assign14130_e19554: f64 = (assign14130_e19531 + assign14130_e19553);
        let assign14130_e19555: f64 = (0.5 * assign14130_e19554);
        let assign14130_e19556: f64 = (p.p901 * assign14130_e19555);
        locals.var_cjs_t = assign14130_e19556;
        locals.var_cjs_t_dn4 = (p.p901 * (0.5 * ((p.p1093 * locals.var_deltemp_dn4) + ((((p.p1093 * locals.var_deltemp_dn4) * assign14130_e19545) + (assign14130_e19538 * (p.p1093 * locals.var_deltemp_dn4))) / (2.0 * assign14130_e19553)))));
        locals.var_cjs_t_dn5 = (p.p901 * (0.5 * ((p.p1093 * locals.var_deltemp_dn5) + ((((p.p1093 * locals.var_deltemp_dn5) * assign14130_e19545) + (assign14130_e19538 * (p.p1093 * locals.var_deltemp_dn5))) / (2.0 * assign14130_e19553)))));

        let assign14140_e19562: f64 = (p.p1093 * locals.var_deltemp);
        let assign14140_e19563: f64 = (1.0 + assign14140_e19562);
        let assign14140_e19565: f64 = (assign14140_e19563 - 1e-6);
        let assign14140_e19569: f64 = (p.p1093 * locals.var_deltemp);
        let assign14140_e19570: f64 = (1.0 + assign14140_e19569);
        let assign14140_e19572: f64 = (assign14140_e19570 - 1e-6);
        let assign14140_e19576: f64 = (p.p1093 * locals.var_deltemp);
        let assign14140_e19577: f64 = (1.0 + assign14140_e19576);
        let assign14140_e19579: f64 = (assign14140_e19577 - 1e-6);
        let assign14140_e19580: f64 = (assign14140_e19572 * assign14140_e19579);
        let assign14140_e19583: f64 = (4.0 * 0.001);
        let assign14140_e19585: f64 = (assign14140_e19583 * 0.001);
        let assign14140_e19586: f64 = (assign14140_e19580 + assign14140_e19585);
        let assign14140_e19587: f64 = (assign14140_e19586).sqrt();
        let assign14140_e19588: f64 = (assign14140_e19565 + assign14140_e19587);
        let assign14140_e19589: f64 = (0.5 * assign14140_e19588);
        let assign14140_e19590: f64 = (p.p902 * assign14140_e19589);
        locals.var_cjd_t = assign14140_e19590;
        locals.var_cjd_t_dn4 = (p.p902 * (0.5 * ((p.p1093 * locals.var_deltemp_dn4) + ((((p.p1093 * locals.var_deltemp_dn4) * assign14140_e19579) + (assign14140_e19572 * (p.p1093 * locals.var_deltemp_dn4))) / (2.0 * assign14140_e19587)))));
        locals.var_cjd_t_dn5 = (p.p902 * (0.5 * ((p.p1093 * locals.var_deltemp_dn5) + ((((p.p1093 * locals.var_deltemp_dn5) * assign14140_e19579) + (assign14140_e19572 * (p.p1093 * locals.var_deltemp_dn5))) / (2.0 * assign14140_e19587)))));

        let assign14150_e19596: f64 = (p.p1094 * locals.var_deltemp);
        let assign14150_e19597: f64 = (1.0 + assign14150_e19596);
        let assign14150_e19599: f64 = (assign14150_e19597 - 1e-6);
        let assign14150_e19603: f64 = (p.p1094 * locals.var_deltemp);
        let assign14150_e19604: f64 = (1.0 + assign14150_e19603);
        let assign14150_e19606: f64 = (assign14150_e19604 - 1e-6);
        let assign14150_e19610: f64 = (p.p1094 * locals.var_deltemp);
        let assign14150_e19611: f64 = (1.0 + assign14150_e19610);
        let assign14150_e19613: f64 = (assign14150_e19611 - 1e-6);
        let assign14150_e19614: f64 = (assign14150_e19606 * assign14150_e19613);
        let assign14150_e19617: f64 = (4.0 * 0.001);
        let assign14150_e19619: f64 = (assign14150_e19617 * 0.001);
        let assign14150_e19620: f64 = (assign14150_e19614 + assign14150_e19619);
        let assign14150_e19621: f64 = (assign14150_e19620).sqrt();
        let assign14150_e19622: f64 = (assign14150_e19599 + assign14150_e19621);
        let assign14150_e19623: f64 = (0.5 * assign14150_e19622);
        let assign14150_e19624: f64 = (p.p903 * assign14150_e19623);
        locals.var_cjsws_t = assign14150_e19624;
        locals.var_cjsws_t_dn4 = (p.p903 * (0.5 * ((p.p1094 * locals.var_deltemp_dn4) + ((((p.p1094 * locals.var_deltemp_dn4) * assign14150_e19613) + (assign14150_e19606 * (p.p1094 * locals.var_deltemp_dn4))) / (2.0 * assign14150_e19621)))));
        locals.var_cjsws_t_dn5 = (p.p903 * (0.5 * ((p.p1094 * locals.var_deltemp_dn5) + ((((p.p1094 * locals.var_deltemp_dn5) * assign14150_e19613) + (assign14150_e19606 * (p.p1094 * locals.var_deltemp_dn5))) / (2.0 * assign14150_e19621)))));

        let assign14160_e19630: f64 = (p.p1094 * locals.var_deltemp);
        let assign14160_e19631: f64 = (1.0 + assign14160_e19630);
        let assign14160_e19633: f64 = (assign14160_e19631 - 1e-6);
        let assign14160_e19637: f64 = (p.p1094 * locals.var_deltemp);
        let assign14160_e19638: f64 = (1.0 + assign14160_e19637);
        let assign14160_e19640: f64 = (assign14160_e19638 - 1e-6);
        let assign14160_e19644: f64 = (p.p1094 * locals.var_deltemp);
        let assign14160_e19645: f64 = (1.0 + assign14160_e19644);
        let assign14160_e19647: f64 = (assign14160_e19645 - 1e-6);
        let assign14160_e19648: f64 = (assign14160_e19640 * assign14160_e19647);
        let assign14160_e19651: f64 = (4.0 * 0.001);
        let assign14160_e19653: f64 = (assign14160_e19651 * 0.001);
        let assign14160_e19654: f64 = (assign14160_e19648 + assign14160_e19653);
        let assign14160_e19655: f64 = (assign14160_e19654).sqrt();
        let assign14160_e19656: f64 = (assign14160_e19633 + assign14160_e19655);
        let assign14160_e19657: f64 = (0.5 * assign14160_e19656);
        let assign14160_e19658: f64 = (p.p904 * assign14160_e19657);
        locals.var_cjswd_t = assign14160_e19658;
        locals.var_cjswd_t_dn4 = (p.p904 * (0.5 * ((p.p1094 * locals.var_deltemp_dn4) + ((((p.p1094 * locals.var_deltemp_dn4) * assign14160_e19647) + (assign14160_e19640 * (p.p1094 * locals.var_deltemp_dn4))) / (2.0 * assign14160_e19655)))));
        locals.var_cjswd_t_dn5 = (p.p904 * (0.5 * ((p.p1094 * locals.var_deltemp_dn5) + ((((p.p1094 * locals.var_deltemp_dn5) * assign14160_e19647) + (assign14160_e19640 * (p.p1094 * locals.var_deltemp_dn5))) / (2.0 * assign14160_e19655)))));

        let assign14170_e19664: f64 = (p.p1095 * locals.var_deltemp);
        let assign14170_e19665: f64 = (1.0 + assign14170_e19664);
        let assign14170_e19667: f64 = (assign14170_e19665 - 1e-6);
        let assign14170_e19671: f64 = (p.p1095 * locals.var_deltemp);
        let assign14170_e19672: f64 = (1.0 + assign14170_e19671);
        let assign14170_e19674: f64 = (assign14170_e19672 - 1e-6);
        let assign14170_e19678: f64 = (p.p1095 * locals.var_deltemp);
        let assign14170_e19679: f64 = (1.0 + assign14170_e19678);
        let assign14170_e19681: f64 = (assign14170_e19679 - 1e-6);
        let assign14170_e19682: f64 = (assign14170_e19674 * assign14170_e19681);
        let assign14170_e19685: f64 = (4.0 * 0.001);
        let assign14170_e19687: f64 = (assign14170_e19685 * 0.001);
        let assign14170_e19688: f64 = (assign14170_e19682 + assign14170_e19687);
        let assign14170_e19689: f64 = (assign14170_e19688).sqrt();
        let assign14170_e19690: f64 = (assign14170_e19667 + assign14170_e19689);
        let assign14170_e19691: f64 = (0.5 * assign14170_e19690);
        let assign14170_e19692: f64 = (p.p905 * assign14170_e19691);
        locals.var_cjswgs_t = assign14170_e19692;
        locals.var_cjswgs_t_dn4 = (p.p905 * (0.5 * ((p.p1095 * locals.var_deltemp_dn4) + ((((p.p1095 * locals.var_deltemp_dn4) * assign14170_e19681) + (assign14170_e19674 * (p.p1095 * locals.var_deltemp_dn4))) / (2.0 * assign14170_e19689)))));
        locals.var_cjswgs_t_dn5 = (p.p905 * (0.5 * ((p.p1095 * locals.var_deltemp_dn5) + ((((p.p1095 * locals.var_deltemp_dn5) * assign14170_e19681) + (assign14170_e19674 * (p.p1095 * locals.var_deltemp_dn5))) / (2.0 * assign14170_e19689)))));

        let assign14180_e19698: f64 = (p.p1095 * locals.var_deltemp);
        let assign14180_e19699: f64 = (1.0 + assign14180_e19698);
        let assign14180_e19701: f64 = (assign14180_e19699 - 1e-6);
        let assign14180_e19705: f64 = (p.p1095 * locals.var_deltemp);
        let assign14180_e19706: f64 = (1.0 + assign14180_e19705);
        let assign14180_e19708: f64 = (assign14180_e19706 - 1e-6);
        let assign14180_e19712: f64 = (p.p1095 * locals.var_deltemp);
        let assign14180_e19713: f64 = (1.0 + assign14180_e19712);
        let assign14180_e19715: f64 = (assign14180_e19713 - 1e-6);
        let assign14180_e19716: f64 = (assign14180_e19708 * assign14180_e19715);
        let assign14180_e19719: f64 = (4.0 * 0.001);
        let assign14180_e19721: f64 = (assign14180_e19719 * 0.001);
        let assign14180_e19722: f64 = (assign14180_e19716 + assign14180_e19721);
        let assign14180_e19723: f64 = (assign14180_e19722).sqrt();
        let assign14180_e19724: f64 = (assign14180_e19701 + assign14180_e19723);
        let assign14180_e19725: f64 = (0.5 * assign14180_e19724);
        let assign14180_e19726: f64 = (p.p906 * assign14180_e19725);
        locals.var_cjswgd_t = assign14180_e19726;
        locals.var_cjswgd_t_dn4 = (p.p906 * (0.5 * ((p.p1095 * locals.var_deltemp_dn4) + ((((p.p1095 * locals.var_deltemp_dn4) * assign14180_e19715) + (assign14180_e19708 * (p.p1095 * locals.var_deltemp_dn4))) / (2.0 * assign14180_e19723)))));
        locals.var_cjswgd_t_dn5 = (p.p906 * (0.5 * ((p.p1095 * locals.var_deltemp_dn5) + ((((p.p1095 * locals.var_deltemp_dn5) * assign14180_e19715) + (assign14180_e19708 * (p.p1095 * locals.var_deltemp_dn5))) / (2.0 * assign14180_e19723)))));

    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign14190_e19731: f64 = (p.p1096 * locals.var_deltemp);
        let assign14190_e19732: f64 = (p.p907 - assign14190_e19731);
        let assign14190_e19734: f64 = (assign14190_e19732 - 0.01);
        let assign14190_e19738: f64 = (p.p1096 * locals.var_deltemp);
        let assign14190_e19739: f64 = (p.p907 - assign14190_e19738);
        let assign14190_e19741: f64 = (assign14190_e19739 - 0.01);
        let assign14190_e19745: f64 = (p.p1096 * locals.var_deltemp);
        let assign14190_e19746: f64 = (p.p907 - assign14190_e19745);
        let assign14190_e19748: f64 = (assign14190_e19746 - 0.01);
        let assign14190_e19749: f64 = (assign14190_e19741 * assign14190_e19748);
        let assign14190_e19752: f64 = (4.0 * 0.001);
        let assign14190_e19754: f64 = (assign14190_e19752 * 0.001);
        let assign14190_e19755: f64 = (assign14190_e19749 + assign14190_e19754);
        let assign14190_e19756: f64 = (assign14190_e19755).sqrt();
        let assign14190_e19757: f64 = (assign14190_e19734 + assign14190_e19756);
        let assign14190_e19758: f64 = (0.5 * assign14190_e19757);
        let assign14190_e19760: f64 = (assign14190_e19758 + 0.01);
        locals.var_pbs_t = assign14190_e19760;
        locals.var_pbs_t_dn4 = (0.5 * ((-(p.p1096 * locals.var_deltemp_dn4)) + ((((-(p.p1096 * locals.var_deltemp_dn4)) * assign14190_e19748) + (assign14190_e19741 * (-(p.p1096 * locals.var_deltemp_dn4)))) / (2.0 * assign14190_e19756))));
        locals.var_pbs_t_dn5 = (0.5 * ((-(p.p1096 * locals.var_deltemp_dn5)) + ((((-(p.p1096 * locals.var_deltemp_dn5)) * assign14190_e19748) + (assign14190_e19741 * (-(p.p1096 * locals.var_deltemp_dn5)))) / (2.0 * assign14190_e19756))));

        let assign14200_e19765: f64 = (p.p1096 * locals.var_deltemp);
        let assign14200_e19766: f64 = (p.p908 - assign14200_e19765);
        let assign14200_e19768: f64 = (assign14200_e19766 - 0.01);
        let assign14200_e19772: f64 = (p.p1096 * locals.var_deltemp);
        let assign14200_e19773: f64 = (p.p908 - assign14200_e19772);
        let assign14200_e19775: f64 = (assign14200_e19773 - 0.01);
        let assign14200_e19779: f64 = (p.p1096 * locals.var_deltemp);
        let assign14200_e19780: f64 = (p.p908 - assign14200_e19779);
        let assign14200_e19782: f64 = (assign14200_e19780 - 0.01);
        let assign14200_e19783: f64 = (assign14200_e19775 * assign14200_e19782);
        let assign14200_e19786: f64 = (4.0 * 0.001);
        let assign14200_e19788: f64 = (assign14200_e19786 * 0.001);
        let assign14200_e19789: f64 = (assign14200_e19783 + assign14200_e19788);
        let assign14200_e19790: f64 = (assign14200_e19789).sqrt();
        let assign14200_e19791: f64 = (assign14200_e19768 + assign14200_e19790);
        let assign14200_e19792: f64 = (0.5 * assign14200_e19791);
        let assign14200_e19794: f64 = (assign14200_e19792 + 0.01);
        locals.var_pbd_t = assign14200_e19794;
        locals.var_pbd_t_dn4 = (0.5 * ((-(p.p1096 * locals.var_deltemp_dn4)) + ((((-(p.p1096 * locals.var_deltemp_dn4)) * assign14200_e19782) + (assign14200_e19775 * (-(p.p1096 * locals.var_deltemp_dn4)))) / (2.0 * assign14200_e19790))));
        locals.var_pbd_t_dn5 = (0.5 * ((-(p.p1096 * locals.var_deltemp_dn5)) + ((((-(p.p1096 * locals.var_deltemp_dn5)) * assign14200_e19782) + (assign14200_e19775 * (-(p.p1096 * locals.var_deltemp_dn5)))) / (2.0 * assign14200_e19790))));

        let assign14210_e19799: f64 = (p.p1097 * locals.var_deltemp);
        let assign14210_e19800: f64 = (p.p909 - assign14210_e19799);
        let assign14210_e19802: f64 = (assign14210_e19800 - 0.01);
        let assign14210_e19806: f64 = (p.p1097 * locals.var_deltemp);
        let assign14210_e19807: f64 = (p.p909 - assign14210_e19806);
        let assign14210_e19809: f64 = (assign14210_e19807 - 0.01);
        let assign14210_e19813: f64 = (p.p1097 * locals.var_deltemp);
        let assign14210_e19814: f64 = (p.p909 - assign14210_e19813);
        let assign14210_e19816: f64 = (assign14210_e19814 - 0.01);
        let assign14210_e19817: f64 = (assign14210_e19809 * assign14210_e19816);
        let assign14210_e19820: f64 = (4.0 * 0.001);
        let assign14210_e19822: f64 = (assign14210_e19820 * 0.001);
        let assign14210_e19823: f64 = (assign14210_e19817 + assign14210_e19822);
        let assign14210_e19824: f64 = (assign14210_e19823).sqrt();
        let assign14210_e19825: f64 = (assign14210_e19802 + assign14210_e19824);
        let assign14210_e19826: f64 = (0.5 * assign14210_e19825);
        let assign14210_e19828: f64 = (assign14210_e19826 + 0.01);
        locals.var_pbsws_t = assign14210_e19828;
        locals.var_pbsws_t_dn4 = (0.5 * ((-(p.p1097 * locals.var_deltemp_dn4)) + ((((-(p.p1097 * locals.var_deltemp_dn4)) * assign14210_e19816) + (assign14210_e19809 * (-(p.p1097 * locals.var_deltemp_dn4)))) / (2.0 * assign14210_e19824))));
        locals.var_pbsws_t_dn5 = (0.5 * ((-(p.p1097 * locals.var_deltemp_dn5)) + ((((-(p.p1097 * locals.var_deltemp_dn5)) * assign14210_e19816) + (assign14210_e19809 * (-(p.p1097 * locals.var_deltemp_dn5)))) / (2.0 * assign14210_e19824))));

        let assign14220_e19833: f64 = (p.p1097 * locals.var_deltemp);
        let assign14220_e19834: f64 = (p.p910 - assign14220_e19833);
        let assign14220_e19836: f64 = (assign14220_e19834 - 0.01);
        let assign14220_e19840: f64 = (p.p1097 * locals.var_deltemp);
        let assign14220_e19841: f64 = (p.p910 - assign14220_e19840);
        let assign14220_e19843: f64 = (assign14220_e19841 - 0.01);
        let assign14220_e19847: f64 = (p.p1097 * locals.var_deltemp);
        let assign14220_e19848: f64 = (p.p910 - assign14220_e19847);
        let assign14220_e19850: f64 = (assign14220_e19848 - 0.01);
        let assign14220_e19851: f64 = (assign14220_e19843 * assign14220_e19850);
        let assign14220_e19854: f64 = (4.0 * 0.001);
        let assign14220_e19856: f64 = (assign14220_e19854 * 0.001);
        let assign14220_e19857: f64 = (assign14220_e19851 + assign14220_e19856);
        let assign14220_e19858: f64 = (assign14220_e19857).sqrt();
        let assign14220_e19859: f64 = (assign14220_e19836 + assign14220_e19858);
        let assign14220_e19860: f64 = (0.5 * assign14220_e19859);
        let assign14220_e19862: f64 = (assign14220_e19860 + 0.01);
        locals.var_pbswd_t = assign14220_e19862;
        locals.var_pbswd_t_dn4 = (0.5 * ((-(p.p1097 * locals.var_deltemp_dn4)) + ((((-(p.p1097 * locals.var_deltemp_dn4)) * assign14220_e19850) + (assign14220_e19843 * (-(p.p1097 * locals.var_deltemp_dn4)))) / (2.0 * assign14220_e19858))));
        locals.var_pbswd_t_dn5 = (0.5 * ((-(p.p1097 * locals.var_deltemp_dn5)) + ((((-(p.p1097 * locals.var_deltemp_dn5)) * assign14220_e19850) + (assign14220_e19843 * (-(p.p1097 * locals.var_deltemp_dn5)))) / (2.0 * assign14220_e19858))));

        let assign14230_e19867: f64 = (p.p1098 * locals.var_deltemp);
        let assign14230_e19868: f64 = (p.p911 - assign14230_e19867);
        let assign14230_e19870: f64 = (assign14230_e19868 - 0.01);
        let assign14230_e19874: f64 = (p.p1098 * locals.var_deltemp);
        let assign14230_e19875: f64 = (p.p911 - assign14230_e19874);
        let assign14230_e19877: f64 = (assign14230_e19875 - 0.01);
        let assign14230_e19881: f64 = (p.p1098 * locals.var_deltemp);
        let assign14230_e19882: f64 = (p.p911 - assign14230_e19881);
        let assign14230_e19884: f64 = (assign14230_e19882 - 0.01);
        let assign14230_e19885: f64 = (assign14230_e19877 * assign14230_e19884);
        let assign14230_e19888: f64 = (4.0 * 0.001);
        let assign14230_e19890: f64 = (assign14230_e19888 * 0.001);
        let assign14230_e19891: f64 = (assign14230_e19885 + assign14230_e19890);
        let assign14230_e19892: f64 = (assign14230_e19891).sqrt();
        let assign14230_e19893: f64 = (assign14230_e19870 + assign14230_e19892);
        let assign14230_e19894: f64 = (0.5 * assign14230_e19893);
        let assign14230_e19896: f64 = (assign14230_e19894 + 0.01);
        locals.var_pbswgs_t = assign14230_e19896;
        locals.var_pbswgs_t_dn4 = (0.5 * ((-(p.p1098 * locals.var_deltemp_dn4)) + ((((-(p.p1098 * locals.var_deltemp_dn4)) * assign14230_e19884) + (assign14230_e19877 * (-(p.p1098 * locals.var_deltemp_dn4)))) / (2.0 * assign14230_e19892))));
        locals.var_pbswgs_t_dn5 = (0.5 * ((-(p.p1098 * locals.var_deltemp_dn5)) + ((((-(p.p1098 * locals.var_deltemp_dn5)) * assign14230_e19884) + (assign14230_e19877 * (-(p.p1098 * locals.var_deltemp_dn5)))) / (2.0 * assign14230_e19892))));

        let assign14240_e19901: f64 = (p.p1098 * locals.var_deltemp);
        let assign14240_e19902: f64 = (p.p912 - assign14240_e19901);
        let assign14240_e19904: f64 = (assign14240_e19902 - 0.01);
        let assign14240_e19908: f64 = (p.p1098 * locals.var_deltemp);
        let assign14240_e19909: f64 = (p.p912 - assign14240_e19908);
        let assign14240_e19911: f64 = (assign14240_e19909 - 0.01);
        let assign14240_e19915: f64 = (p.p1098 * locals.var_deltemp);
        let assign14240_e19916: f64 = (p.p912 - assign14240_e19915);
        let assign14240_e19918: f64 = (assign14240_e19916 - 0.01);
        let assign14240_e19919: f64 = (assign14240_e19911 * assign14240_e19918);
        let assign14240_e19922: f64 = (4.0 * 0.001);
        let assign14240_e19924: f64 = (assign14240_e19922 * 0.001);
        let assign14240_e19925: f64 = (assign14240_e19919 + assign14240_e19924);
        let assign14240_e19926: f64 = (assign14240_e19925).sqrt();
        let assign14240_e19927: f64 = (assign14240_e19904 + assign14240_e19926);
        let assign14240_e19928: f64 = (0.5 * assign14240_e19927);
        let assign14240_e19930: f64 = (assign14240_e19928 + 0.01);
        locals.var_pbswgd_t = assign14240_e19930;
        locals.var_pbswgd_t_dn4 = (0.5 * ((-(p.p1098 * locals.var_deltemp_dn4)) + ((((-(p.p1098 * locals.var_deltemp_dn4)) * assign14240_e19918) + (assign14240_e19911 * (-(p.p1098 * locals.var_deltemp_dn4)))) / (2.0 * assign14240_e19926))));
        locals.var_pbswgd_t_dn5 = (0.5 * ((-(p.p1098 * locals.var_deltemp_dn5)) + ((((-(p.p1098 * locals.var_deltemp_dn5)) * assign14240_e19918) + (assign14240_e19911 * (-(p.p1098 * locals.var_deltemp_dn5)))) / (2.0 * assign14240_e19926))));

        let assign14250_e19933: f64 = if p.p8 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard456 = assign14250_e19933;

        let assign14260_e19936: f64 = (p.p2 % 2.0);
        let assign14260_e19938: f64 = if assign14260_e19936 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard457 = assign14260_e19938;

        let (assign14270_e19944,) = {
    if ((locals.var_guard456 != 0.0) && (locals.var_guard457 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign14270_e19944;

        let (assign14280_e19950,) = {
    if ((locals.var_guard456 != 0.0) && (locals.var_guard457 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign14280_e19950;

        let (assign14290_e19964,) = {
    if ((locals.var_guard456 != 0.0) && (locals.var_guard457 != 0.0)) {
        let assign14290_e19957: f64 = (p.p2 - 1.0);
        let assign14290_e19959: f64 = (assign14290_e19957 / 2.0);
        let assign14290_e19961: f64 = (assign14290_e19959).max(0.0);
        let assign14290_e19962: f64 = (2.0 * assign14290_e19961);
        (assign14290_e19962,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign14290_e19964;

        let (assign14300_e19970,) = {
    if ((locals.var_guard456 != 0.0) && (locals.var_guard457 != 0.0)) {
        (locals.var_nuintd,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign14300_e19970;

        let assign14310_e19973: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard458 = assign14310_e19973;

        let (assign14320_e19982,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign14320_e19982;

        let (assign14330_e19999,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        let assign14330_e19992: f64 = (p.p2 / 2.0);
        let assign14330_e19994: f64 = (assign14330_e19992 - 1.0);
        let assign14330_e19996: f64 = (assign14330_e19994).max(0.0);
        let assign14330_e19997: f64 = (2.0 * assign14330_e19996);
        (assign14330_e19997,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign14330_e19999;

        let (assign14340_e20008,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign14340_e20008;

        let (assign14350_e20017,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign14350_e20017;

        let (assign14360_e20027,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign14360_e20027;

        let (assign14370_e20037,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 == 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign14370_e20037;

        let (assign14380_e20047,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign14380_e20047;

        let (assign14390_e20065,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 == 0.0)) {
        let assign14390_e20058: f64 = (p.p2 / 2.0);
        let assign14390_e20060: f64 = (assign14390_e20058 - 1.0);
        let assign14390_e20062: f64 = (assign14390_e20060).max(0.0);
        let assign14390_e20063: f64 = (2.0 * assign14390_e20062);
        (assign14390_e20063,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign14390_e20065;

        let assign14400_e20068: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        locals.var_t0 = assign14400_e20068;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;

        let assign14410_e20071: f64 = (locals.var_dmcgeff + locals.var_dmcgeff);
        locals.var_t1 = assign14410_e20071;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;

        let assign14420_e20074: f64 = (locals.var_dmdgeff + locals.var_dmdgeff);
        locals.var_t2 = assign14420_e20074;
        locals.var_t2_dn3 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;

        let assign14430_e20077: f64 = (locals.var_t0 + locals.var_t0);
        let assign14430_e20079: f64 = (assign14430_e20077 + locals.var_weffcj);
        locals.var_psiso = assign14430_e20079;
        locals.var_psiso_dn3 = (locals.var_t0_dn3 + locals.var_t0_dn3);
        locals.var_psiso_dn4 = (locals.var_t0_dn4 + locals.var_t0_dn4);
        locals.var_psiso_dn5 = (locals.var_t0_dn5 + locals.var_t0_dn5);
        locals.var_psiso_dn6 = (locals.var_t0_dn6 + locals.var_t0_dn6);
        locals.var_psiso_dn7 = (locals.var_t0_dn7 + locals.var_t0_dn7);
        locals.var_psiso_dn8 = (locals.var_t0_dn8 + locals.var_t0_dn8);
        locals.var_psiso_dn9 = (locals.var_t0_dn9 + locals.var_t0_dn9);
        locals.var_psiso_dn10 = (locals.var_t0_dn10 + locals.var_t0_dn10);
        locals.var_psiso_dn11 = (locals.var_t0_dn11 + locals.var_t0_dn11);

        let assign14440_e20082: f64 = (locals.var_t0 + locals.var_t0);
        let assign14440_e20084: f64 = (assign14440_e20082 + locals.var_weffcj);
        locals.var_pdiso = assign14440_e20084;
        locals.var_pdiso_dn3 = (locals.var_t0_dn3 + locals.var_t0_dn3);
        locals.var_pdiso_dn4 = (locals.var_t0_dn4 + locals.var_t0_dn4);
        locals.var_pdiso_dn5 = (locals.var_t0_dn5 + locals.var_t0_dn5);
        locals.var_pdiso_dn6 = (locals.var_t0_dn6 + locals.var_t0_dn6);
        locals.var_pdiso_dn7 = (locals.var_t0_dn7 + locals.var_t0_dn7);
        locals.var_pdiso_dn8 = (locals.var_t0_dn8 + locals.var_t0_dn8);
        locals.var_pdiso_dn9 = (locals.var_t0_dn9 + locals.var_t0_dn9);
        locals.var_pdiso_dn10 = (locals.var_t0_dn10 + locals.var_t0_dn10);
        locals.var_pdiso_dn11 = (locals.var_t0_dn11 + locals.var_t0_dn11);

        locals.var_pssha = locals.var_t1;
        locals.var_pssha_dn3 = locals.var_t1_dn3;
        locals.var_pssha_dn4 = locals.var_t1_dn4;
        locals.var_pssha_dn5 = locals.var_t1_dn5;
        locals.var_pssha_dn6 = locals.var_t1_dn6;
        locals.var_pssha_dn7 = locals.var_t1_dn7;
        locals.var_pssha_dn8 = locals.var_t1_dn8;
        locals.var_pssha_dn9 = locals.var_t1_dn9;
        locals.var_pssha_dn10 = locals.var_t1_dn10;
        locals.var_pssha_dn11 = locals.var_t1_dn11;

        locals.var_pdsha = locals.var_t1;
        locals.var_pdsha_dn3 = locals.var_t1_dn3;
        locals.var_pdsha_dn4 = locals.var_t1_dn4;
        locals.var_pdsha_dn5 = locals.var_t1_dn5;
        locals.var_pdsha_dn6 = locals.var_t1_dn6;
        locals.var_pdsha_dn7 = locals.var_t1_dn7;
        locals.var_pdsha_dn8 = locals.var_t1_dn8;
        locals.var_pdsha_dn9 = locals.var_t1_dn9;
        locals.var_pdsha_dn10 = locals.var_t1_dn10;
        locals.var_pdsha_dn11 = locals.var_t1_dn11;

        locals.var_psmer = locals.var_t2;
        locals.var_psmer_dn3 = locals.var_t2_dn3;
        locals.var_psmer_dn4 = locals.var_t2_dn4;
        locals.var_psmer_dn5 = locals.var_t2_dn5;
        locals.var_psmer_dn6 = locals.var_t2_dn6;
        locals.var_psmer_dn7 = locals.var_t2_dn7;
        locals.var_psmer_dn8 = locals.var_t2_dn8;
        locals.var_psmer_dn9 = locals.var_t2_dn9;
        locals.var_psmer_dn10 = locals.var_t2_dn10;
        locals.var_psmer_dn11 = locals.var_t2_dn11;

        locals.var_pdmer = locals.var_t2;
        locals.var_pdmer_dn3 = locals.var_t2_dn3;
        locals.var_pdmer_dn4 = locals.var_t2_dn4;
        locals.var_pdmer_dn5 = locals.var_t2_dn5;
        locals.var_pdmer_dn6 = locals.var_t2_dn6;
        locals.var_pdmer_dn7 = locals.var_t2_dn7;
        locals.var_pdmer_dn8 = locals.var_t2_dn8;
        locals.var_pdmer_dn9 = locals.var_t2_dn9;
        locals.var_pdmer_dn10 = locals.var_t2_dn10;
        locals.var_pdmer_dn11 = locals.var_t2_dn11;

        let assign14490_e20091: f64 = (locals.var_t0 * locals.var_weffcj);
        locals.var_asiso = assign14490_e20091;
        locals.var_asiso_dn3 = (locals.var_t0_dn3 * locals.var_weffcj);
        locals.var_asiso_dn4 = (locals.var_t0_dn4 * locals.var_weffcj);
        locals.var_asiso_dn5 = (locals.var_t0_dn5 * locals.var_weffcj);
        locals.var_asiso_dn6 = (locals.var_t0_dn6 * locals.var_weffcj);
        locals.var_asiso_dn7 = (locals.var_t0_dn7 * locals.var_weffcj);
        locals.var_asiso_dn8 = (locals.var_t0_dn8 * locals.var_weffcj);
        locals.var_asiso_dn9 = (locals.var_t0_dn9 * locals.var_weffcj);
        locals.var_asiso_dn10 = (locals.var_t0_dn10 * locals.var_weffcj);
        locals.var_asiso_dn11 = (locals.var_t0_dn11 * locals.var_weffcj);

        let assign14500_e20094: f64 = (locals.var_t0 * locals.var_weffcj);
        locals.var_adiso = assign14500_e20094;
        locals.var_adiso_dn3 = (locals.var_t0_dn3 * locals.var_weffcj);
        locals.var_adiso_dn4 = (locals.var_t0_dn4 * locals.var_weffcj);
        locals.var_adiso_dn5 = (locals.var_t0_dn5 * locals.var_weffcj);
        locals.var_adiso_dn6 = (locals.var_t0_dn6 * locals.var_weffcj);
        locals.var_adiso_dn7 = (locals.var_t0_dn7 * locals.var_weffcj);
        locals.var_adiso_dn8 = (locals.var_t0_dn8 * locals.var_weffcj);
        locals.var_adiso_dn9 = (locals.var_t0_dn9 * locals.var_weffcj);
        locals.var_adiso_dn10 = (locals.var_t0_dn10 * locals.var_weffcj);
        locals.var_adiso_dn11 = (locals.var_t0_dn11 * locals.var_weffcj);

        let assign14510_e20097: f64 = (locals.var_dmcgeff * locals.var_weffcj);
        locals.var_assha = assign14510_e20097;

        let assign14520_e20100: f64 = (locals.var_dmcgeff * locals.var_weffcj);
        locals.var_adsha = assign14520_e20100;

        let assign14530_e20103: f64 = (locals.var_dmdgeff * locals.var_weffcj);
        locals.var_asmer = assign14530_e20103;

        let assign14540_e20106: f64 = (locals.var_dmdgeff * locals.var_weffcj);
        locals.var_admer = assign14540_e20106;

        let assign14550_e20109: f64 = if p.p8 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard459 = assign14550_e20109;

        let assign14560_e20112: f64 = if p.p8 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard460 = assign14560_e20112;

        let assign14570_e20115: f64 = if p.p8 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard461 = assign14570_e20115;

        let assign14580_e20118: f64 = if p.p8 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard462 = assign14580_e20118;

        let assign14590_e20121: f64 = if p.p8 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard463 = assign14590_e20121;

        let assign14600_e20124: f64 = if p.p8 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard464 = assign14600_e20124;

        let assign14610_e20127: f64 = if p.p8 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard465 = assign14610_e20127;

        let assign14620_e20130: f64 = if p.p8 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard466 = assign14620_e20130;

        let assign14630_e20133: f64 = if p.p8 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard467 = assign14630_e20133;

        let assign14640_e20136: f64 = if p.p8 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard468 = assign14640_e20136;

        let assign14650_e20139: f64 = if p.p8 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard469 = assign14650_e20139;

        let (assign14660_e20149, assign14660_e20149_d_n3, assign14660_e20149_d_n4, assign14660_e20149_d_n5, assign14660_e20149_d_n6, assign14660_e20149_d_n7, assign14660_e20149_d_n8, assign14660_e20149_d_n9, assign14660_e20149_d_n10, assign14660_e20149_d_n11,) = {
    if (locals.var_guard459 != 0.0) {
        let assign14660_e20143: f64 = (locals.var_nuends * locals.var_psiso);
        let assign14660_e20146: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14660_e20147: f64 = (assign14660_e20143 + assign14660_e20146);
        (assign14660_e20147, ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14660_e20149;
        locals.var_temp_pseff_dn3 = assign14660_e20149_d_n3;
        locals.var_temp_pseff_dn4 = assign14660_e20149_d_n4;
        locals.var_temp_pseff_dn5 = assign14660_e20149_d_n5;
        locals.var_temp_pseff_dn6 = assign14660_e20149_d_n6;
        locals.var_temp_pseff_dn7 = assign14660_e20149_d_n7;
        locals.var_temp_pseff_dn8 = assign14660_e20149_d_n8;
        locals.var_temp_pseff_dn9 = assign14660_e20149_d_n9;
        locals.var_temp_pseff_dn10 = assign14660_e20149_d_n10;
        locals.var_temp_pseff_dn11 = assign14660_e20149_d_n11;

        let (assign14670_e20159, assign14670_e20159_d_n3, assign14670_e20159_d_n4, assign14670_e20159_d_n5, assign14670_e20159_d_n6, assign14670_e20159_d_n7, assign14670_e20159_d_n8, assign14670_e20159_d_n9, assign14670_e20159_d_n10, assign14670_e20159_d_n11,) = {
    if (locals.var_guard459 != 0.0) {
        let assign14670_e20153: f64 = (locals.var_nuendd * locals.var_pdiso);
        let assign14670_e20156: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14670_e20157: f64 = (assign14670_e20153 + assign14670_e20156);
        (assign14670_e20157, ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14670_e20159;
        locals.var_temp_pdeff_dn3 = assign14670_e20159_d_n3;
        locals.var_temp_pdeff_dn4 = assign14670_e20159_d_n4;
        locals.var_temp_pdeff_dn5 = assign14670_e20159_d_n5;
        locals.var_temp_pdeff_dn6 = assign14670_e20159_d_n6;
        locals.var_temp_pdeff_dn7 = assign14670_e20159_d_n7;
        locals.var_temp_pdeff_dn8 = assign14670_e20159_d_n8;
        locals.var_temp_pdeff_dn9 = assign14670_e20159_d_n9;
        locals.var_temp_pdeff_dn10 = assign14670_e20159_d_n10;
        locals.var_temp_pdeff_dn11 = assign14670_e20159_d_n11;

        let (assign14680_e20169, assign14680_e20169_d_n3, assign14680_e20169_d_n4, assign14680_e20169_d_n5, assign14680_e20169_d_n6, assign14680_e20169_d_n7, assign14680_e20169_d_n8, assign14680_e20169_d_n9, assign14680_e20169_d_n10, assign14680_e20169_d_n11,) = {
    if (locals.var_guard459 != 0.0) {
        let assign14680_e20163: f64 = (locals.var_nuends * locals.var_asiso);
        let assign14680_e20166: f64 = (locals.var_nuints * locals.var_assha);
        let assign14680_e20167: f64 = (assign14680_e20163 + assign14680_e20166);
        (assign14680_e20167, (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11),)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14680_e20169;
        locals.var_temp_aseff_dn3 = assign14680_e20169_d_n3;
        locals.var_temp_aseff_dn4 = assign14680_e20169_d_n4;
        locals.var_temp_aseff_dn5 = assign14680_e20169_d_n5;
        locals.var_temp_aseff_dn6 = assign14680_e20169_d_n6;
        locals.var_temp_aseff_dn7 = assign14680_e20169_d_n7;
        locals.var_temp_aseff_dn8 = assign14680_e20169_d_n8;
        locals.var_temp_aseff_dn9 = assign14680_e20169_d_n9;
        locals.var_temp_aseff_dn10 = assign14680_e20169_d_n10;
        locals.var_temp_aseff_dn11 = assign14680_e20169_d_n11;

    }

    pub(super) fn stamp_transient_block_23(
        locals: &mut StampLocals,
    ) {
        let (assign14690_e20179, assign14690_e20179_d_n3, assign14690_e20179_d_n4, assign14690_e20179_d_n5, assign14690_e20179_d_n6, assign14690_e20179_d_n7, assign14690_e20179_d_n8, assign14690_e20179_d_n9, assign14690_e20179_d_n10, assign14690_e20179_d_n11,) = {
    if (locals.var_guard459 != 0.0) {
        let assign14690_e20173: f64 = (locals.var_nuendd * locals.var_adiso);
        let assign14690_e20176: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14690_e20177: f64 = (assign14690_e20173 + assign14690_e20176);
        (assign14690_e20177, (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11),)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14690_e20179;
        locals.var_temp_adeff_dn3 = assign14690_e20179_d_n3;
        locals.var_temp_adeff_dn4 = assign14690_e20179_d_n4;
        locals.var_temp_adeff_dn5 = assign14690_e20179_d_n5;
        locals.var_temp_adeff_dn6 = assign14690_e20179_d_n6;
        locals.var_temp_adeff_dn7 = assign14690_e20179_d_n7;
        locals.var_temp_adeff_dn8 = assign14690_e20179_d_n8;
        locals.var_temp_adeff_dn9 = assign14690_e20179_d_n9;
        locals.var_temp_adeff_dn10 = assign14690_e20179_d_n10;
        locals.var_temp_adeff_dn11 = assign14690_e20179_d_n11;

        let (assign14700_e20192, assign14700_e20192_d_n3, assign14700_e20192_d_n4, assign14700_e20192_d_n5, assign14700_e20192_d_n6, assign14700_e20192_d_n7, assign14700_e20192_d_n8, assign14700_e20192_d_n9, assign14700_e20192_d_n10, assign14700_e20192_d_n11,) = {
    if ((locals.var_guard460 != 0.0) && (locals.var_guard459 == 0.0)) {
        let assign14700_e20186: f64 = (locals.var_nuends * locals.var_psiso);
        let assign14700_e20189: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14700_e20190: f64 = (assign14700_e20186 + assign14700_e20189);
        (assign14700_e20190, ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14700_e20192;
        locals.var_temp_pseff_dn3 = assign14700_e20192_d_n3;
        locals.var_temp_pseff_dn4 = assign14700_e20192_d_n4;
        locals.var_temp_pseff_dn5 = assign14700_e20192_d_n5;
        locals.var_temp_pseff_dn6 = assign14700_e20192_d_n6;
        locals.var_temp_pseff_dn7 = assign14700_e20192_d_n7;
        locals.var_temp_pseff_dn8 = assign14700_e20192_d_n8;
        locals.var_temp_pseff_dn9 = assign14700_e20192_d_n9;
        locals.var_temp_pseff_dn10 = assign14700_e20192_d_n10;
        locals.var_temp_pseff_dn11 = assign14700_e20192_d_n11;

        let (assign14710_e20203, assign14710_e20203_d_n3, assign14710_e20203_d_n4, assign14710_e20203_d_n5, assign14710_e20203_d_n6, assign14710_e20203_d_n7, assign14710_e20203_d_n8, assign14710_e20203_d_n9, assign14710_e20203_d_n10, assign14710_e20203_d_n11,) = {
    if ((locals.var_guard460 != 0.0) && (locals.var_guard459 == 0.0)) {
        let assign14710_e20199: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14710_e20201: f64 = (assign14710_e20199 * locals.var_pdsha);
        (assign14710_e20201, (assign14710_e20199 * locals.var_pdsha_dn3), (assign14710_e20199 * locals.var_pdsha_dn4), (assign14710_e20199 * locals.var_pdsha_dn5), (assign14710_e20199 * locals.var_pdsha_dn6), (assign14710_e20199 * locals.var_pdsha_dn7), (assign14710_e20199 * locals.var_pdsha_dn8), (assign14710_e20199 * locals.var_pdsha_dn9), (assign14710_e20199 * locals.var_pdsha_dn10), (assign14710_e20199 * locals.var_pdsha_dn11),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14710_e20203;
        locals.var_temp_pdeff_dn3 = assign14710_e20203_d_n3;
        locals.var_temp_pdeff_dn4 = assign14710_e20203_d_n4;
        locals.var_temp_pdeff_dn5 = assign14710_e20203_d_n5;
        locals.var_temp_pdeff_dn6 = assign14710_e20203_d_n6;
        locals.var_temp_pdeff_dn7 = assign14710_e20203_d_n7;
        locals.var_temp_pdeff_dn8 = assign14710_e20203_d_n8;
        locals.var_temp_pdeff_dn9 = assign14710_e20203_d_n9;
        locals.var_temp_pdeff_dn10 = assign14710_e20203_d_n10;
        locals.var_temp_pdeff_dn11 = assign14710_e20203_d_n11;

        let (assign14720_e20216, assign14720_e20216_d_n3, assign14720_e20216_d_n4, assign14720_e20216_d_n5, assign14720_e20216_d_n6, assign14720_e20216_d_n7, assign14720_e20216_d_n8, assign14720_e20216_d_n9, assign14720_e20216_d_n10, assign14720_e20216_d_n11,) = {
    if ((locals.var_guard460 != 0.0) && (locals.var_guard459 == 0.0)) {
        let assign14720_e20210: f64 = (locals.var_nuends * locals.var_asiso);
        let assign14720_e20213: f64 = (locals.var_nuints * locals.var_assha);
        let assign14720_e20214: f64 = (assign14720_e20210 + assign14720_e20213);
        (assign14720_e20214, (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11),)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14720_e20216;
        locals.var_temp_aseff_dn3 = assign14720_e20216_d_n3;
        locals.var_temp_aseff_dn4 = assign14720_e20216_d_n4;
        locals.var_temp_aseff_dn5 = assign14720_e20216_d_n5;
        locals.var_temp_aseff_dn6 = assign14720_e20216_d_n6;
        locals.var_temp_aseff_dn7 = assign14720_e20216_d_n7;
        locals.var_temp_aseff_dn8 = assign14720_e20216_d_n8;
        locals.var_temp_aseff_dn9 = assign14720_e20216_d_n9;
        locals.var_temp_aseff_dn10 = assign14720_e20216_d_n10;
        locals.var_temp_aseff_dn11 = assign14720_e20216_d_n11;

        let (assign14730_e20227, assign14730_e20227_d_n3, assign14730_e20227_d_n4, assign14730_e20227_d_n5, assign14730_e20227_d_n6, assign14730_e20227_d_n7, assign14730_e20227_d_n8, assign14730_e20227_d_n9, assign14730_e20227_d_n10, assign14730_e20227_d_n11,) = {
    if ((locals.var_guard460 != 0.0) && (locals.var_guard459 == 0.0)) {
        let assign14730_e20223: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14730_e20225: f64 = (assign14730_e20223 * locals.var_adsha);
        (assign14730_e20225, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14730_e20227;
        locals.var_temp_adeff_dn3 = assign14730_e20227_d_n3;
        locals.var_temp_adeff_dn4 = assign14730_e20227_d_n4;
        locals.var_temp_adeff_dn5 = assign14730_e20227_d_n5;
        locals.var_temp_adeff_dn6 = assign14730_e20227_d_n6;
        locals.var_temp_adeff_dn7 = assign14730_e20227_d_n7;
        locals.var_temp_adeff_dn8 = assign14730_e20227_d_n8;
        locals.var_temp_adeff_dn9 = assign14730_e20227_d_n9;
        locals.var_temp_adeff_dn10 = assign14730_e20227_d_n10;
        locals.var_temp_adeff_dn11 = assign14730_e20227_d_n11;

        let (assign14740_e20240, assign14740_e20240_d_n3, assign14740_e20240_d_n4, assign14740_e20240_d_n5, assign14740_e20240_d_n6, assign14740_e20240_d_n7, assign14740_e20240_d_n8, assign14740_e20240_d_n9, assign14740_e20240_d_n10, assign14740_e20240_d_n11,) = {
    if ((locals.var_guard461 != 0.0) && (!((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)))) {
        let assign14740_e20236: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14740_e20238: f64 = (assign14740_e20236 * locals.var_pssha);
        (assign14740_e20238, (assign14740_e20236 * locals.var_pssha_dn3), (assign14740_e20236 * locals.var_pssha_dn4), (assign14740_e20236 * locals.var_pssha_dn5), (assign14740_e20236 * locals.var_pssha_dn6), (assign14740_e20236 * locals.var_pssha_dn7), (assign14740_e20236 * locals.var_pssha_dn8), (assign14740_e20236 * locals.var_pssha_dn9), (assign14740_e20236 * locals.var_pssha_dn10), (assign14740_e20236 * locals.var_pssha_dn11),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14740_e20240;
        locals.var_temp_pseff_dn3 = assign14740_e20240_d_n3;
        locals.var_temp_pseff_dn4 = assign14740_e20240_d_n4;
        locals.var_temp_pseff_dn5 = assign14740_e20240_d_n5;
        locals.var_temp_pseff_dn6 = assign14740_e20240_d_n6;
        locals.var_temp_pseff_dn7 = assign14740_e20240_d_n7;
        locals.var_temp_pseff_dn8 = assign14740_e20240_d_n8;
        locals.var_temp_pseff_dn9 = assign14740_e20240_d_n9;
        locals.var_temp_pseff_dn10 = assign14740_e20240_d_n10;
        locals.var_temp_pseff_dn11 = assign14740_e20240_d_n11;

        let (assign14750_e20255, assign14750_e20255_d_n3, assign14750_e20255_d_n4, assign14750_e20255_d_n5, assign14750_e20255_d_n6, assign14750_e20255_d_n7, assign14750_e20255_d_n8, assign14750_e20255_d_n9, assign14750_e20255_d_n10, assign14750_e20255_d_n11,) = {
    if ((locals.var_guard461 != 0.0) && (!((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)))) {
        let assign14750_e20249: f64 = (locals.var_nuendd * locals.var_pdiso);
        let assign14750_e20252: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14750_e20253: f64 = (assign14750_e20249 + assign14750_e20252);
        (assign14750_e20253, ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14750_e20255;
        locals.var_temp_pdeff_dn3 = assign14750_e20255_d_n3;
        locals.var_temp_pdeff_dn4 = assign14750_e20255_d_n4;
        locals.var_temp_pdeff_dn5 = assign14750_e20255_d_n5;
        locals.var_temp_pdeff_dn6 = assign14750_e20255_d_n6;
        locals.var_temp_pdeff_dn7 = assign14750_e20255_d_n7;
        locals.var_temp_pdeff_dn8 = assign14750_e20255_d_n8;
        locals.var_temp_pdeff_dn9 = assign14750_e20255_d_n9;
        locals.var_temp_pdeff_dn10 = assign14750_e20255_d_n10;
        locals.var_temp_pdeff_dn11 = assign14750_e20255_d_n11;

        let (assign14760_e20268, assign14760_e20268_d_n3, assign14760_e20268_d_n4, assign14760_e20268_d_n5, assign14760_e20268_d_n6, assign14760_e20268_d_n7, assign14760_e20268_d_n8, assign14760_e20268_d_n9, assign14760_e20268_d_n10, assign14760_e20268_d_n11,) = {
    if ((locals.var_guard461 != 0.0) && (!((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)))) {
        let assign14760_e20264: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14760_e20266: f64 = (assign14760_e20264 * locals.var_assha);
        (assign14760_e20266, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14760_e20268;
        locals.var_temp_aseff_dn3 = assign14760_e20268_d_n3;
        locals.var_temp_aseff_dn4 = assign14760_e20268_d_n4;
        locals.var_temp_aseff_dn5 = assign14760_e20268_d_n5;
        locals.var_temp_aseff_dn6 = assign14760_e20268_d_n6;
        locals.var_temp_aseff_dn7 = assign14760_e20268_d_n7;
        locals.var_temp_aseff_dn8 = assign14760_e20268_d_n8;
        locals.var_temp_aseff_dn9 = assign14760_e20268_d_n9;
        locals.var_temp_aseff_dn10 = assign14760_e20268_d_n10;
        locals.var_temp_aseff_dn11 = assign14760_e20268_d_n11;

        let (assign14770_e20283, assign14770_e20283_d_n3, assign14770_e20283_d_n4, assign14770_e20283_d_n5, assign14770_e20283_d_n6, assign14770_e20283_d_n7, assign14770_e20283_d_n8, assign14770_e20283_d_n9, assign14770_e20283_d_n10, assign14770_e20283_d_n11,) = {
    if ((locals.var_guard461 != 0.0) && (!((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)))) {
        let assign14770_e20277: f64 = (locals.var_nuendd * locals.var_adiso);
        let assign14770_e20280: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14770_e20281: f64 = (assign14770_e20277 + assign14770_e20280);
        (assign14770_e20281, (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11),)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14770_e20283;
        locals.var_temp_adeff_dn3 = assign14770_e20283_d_n3;
        locals.var_temp_adeff_dn4 = assign14770_e20283_d_n4;
        locals.var_temp_adeff_dn5 = assign14770_e20283_d_n5;
        locals.var_temp_adeff_dn6 = assign14770_e20283_d_n6;
        locals.var_temp_adeff_dn7 = assign14770_e20283_d_n7;
        locals.var_temp_adeff_dn8 = assign14770_e20283_d_n8;
        locals.var_temp_adeff_dn9 = assign14770_e20283_d_n9;
        locals.var_temp_adeff_dn10 = assign14770_e20283_d_n10;
        locals.var_temp_adeff_dn11 = assign14770_e20283_d_n11;

        let (assign14780_e20298, assign14780_e20298_d_n3, assign14780_e20298_d_n4, assign14780_e20298_d_n5, assign14780_e20298_d_n6, assign14780_e20298_d_n7, assign14780_e20298_d_n8, assign14780_e20298_d_n9, assign14780_e20298_d_n10, assign14780_e20298_d_n11,) = {
    if ((locals.var_guard462 != 0.0) && (!(((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)))) {
        let assign14780_e20294: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14780_e20296: f64 = (assign14780_e20294 * locals.var_pssha);
        (assign14780_e20296, (assign14780_e20294 * locals.var_pssha_dn3), (assign14780_e20294 * locals.var_pssha_dn4), (assign14780_e20294 * locals.var_pssha_dn5), (assign14780_e20294 * locals.var_pssha_dn6), (assign14780_e20294 * locals.var_pssha_dn7), (assign14780_e20294 * locals.var_pssha_dn8), (assign14780_e20294 * locals.var_pssha_dn9), (assign14780_e20294 * locals.var_pssha_dn10), (assign14780_e20294 * locals.var_pssha_dn11),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14780_e20298;
        locals.var_temp_pseff_dn3 = assign14780_e20298_d_n3;
        locals.var_temp_pseff_dn4 = assign14780_e20298_d_n4;
        locals.var_temp_pseff_dn5 = assign14780_e20298_d_n5;
        locals.var_temp_pseff_dn6 = assign14780_e20298_d_n6;
        locals.var_temp_pseff_dn7 = assign14780_e20298_d_n7;
        locals.var_temp_pseff_dn8 = assign14780_e20298_d_n8;
        locals.var_temp_pseff_dn9 = assign14780_e20298_d_n9;
        locals.var_temp_pseff_dn10 = assign14780_e20298_d_n10;
        locals.var_temp_pseff_dn11 = assign14780_e20298_d_n11;

        let (assign14790_e20313, assign14790_e20313_d_n3, assign14790_e20313_d_n4, assign14790_e20313_d_n5, assign14790_e20313_d_n6, assign14790_e20313_d_n7, assign14790_e20313_d_n8, assign14790_e20313_d_n9, assign14790_e20313_d_n10, assign14790_e20313_d_n11,) = {
    if ((locals.var_guard462 != 0.0) && (!(((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)))) {
        let assign14790_e20309: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14790_e20311: f64 = (assign14790_e20309 * locals.var_pdsha);
        (assign14790_e20311, (assign14790_e20309 * locals.var_pdsha_dn3), (assign14790_e20309 * locals.var_pdsha_dn4), (assign14790_e20309 * locals.var_pdsha_dn5), (assign14790_e20309 * locals.var_pdsha_dn6), (assign14790_e20309 * locals.var_pdsha_dn7), (assign14790_e20309 * locals.var_pdsha_dn8), (assign14790_e20309 * locals.var_pdsha_dn9), (assign14790_e20309 * locals.var_pdsha_dn10), (assign14790_e20309 * locals.var_pdsha_dn11),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14790_e20313;
        locals.var_temp_pdeff_dn3 = assign14790_e20313_d_n3;
        locals.var_temp_pdeff_dn4 = assign14790_e20313_d_n4;
        locals.var_temp_pdeff_dn5 = assign14790_e20313_d_n5;
        locals.var_temp_pdeff_dn6 = assign14790_e20313_d_n6;
        locals.var_temp_pdeff_dn7 = assign14790_e20313_d_n7;
        locals.var_temp_pdeff_dn8 = assign14790_e20313_d_n8;
        locals.var_temp_pdeff_dn9 = assign14790_e20313_d_n9;
        locals.var_temp_pdeff_dn10 = assign14790_e20313_d_n10;
        locals.var_temp_pdeff_dn11 = assign14790_e20313_d_n11;

        let (assign14800_e20328, assign14800_e20328_d_n3, assign14800_e20328_d_n4, assign14800_e20328_d_n5, assign14800_e20328_d_n6, assign14800_e20328_d_n7, assign14800_e20328_d_n8, assign14800_e20328_d_n9, assign14800_e20328_d_n10, assign14800_e20328_d_n11,) = {
    if ((locals.var_guard462 != 0.0) && (!(((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)))) {
        let assign14800_e20324: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14800_e20326: f64 = (assign14800_e20324 * locals.var_assha);
        (assign14800_e20326, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14800_e20328;
        locals.var_temp_aseff_dn3 = assign14800_e20328_d_n3;
        locals.var_temp_aseff_dn4 = assign14800_e20328_d_n4;
        locals.var_temp_aseff_dn5 = assign14800_e20328_d_n5;
        locals.var_temp_aseff_dn6 = assign14800_e20328_d_n6;
        locals.var_temp_aseff_dn7 = assign14800_e20328_d_n7;
        locals.var_temp_aseff_dn8 = assign14800_e20328_d_n8;
        locals.var_temp_aseff_dn9 = assign14800_e20328_d_n9;
        locals.var_temp_aseff_dn10 = assign14800_e20328_d_n10;
        locals.var_temp_aseff_dn11 = assign14800_e20328_d_n11;

        let (assign14810_e20343, assign14810_e20343_d_n3, assign14810_e20343_d_n4, assign14810_e20343_d_n5, assign14810_e20343_d_n6, assign14810_e20343_d_n7, assign14810_e20343_d_n8, assign14810_e20343_d_n9, assign14810_e20343_d_n10, assign14810_e20343_d_n11,) = {
    if ((locals.var_guard462 != 0.0) && (!(((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)))) {
        let assign14810_e20339: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14810_e20341: f64 = (assign14810_e20339 * locals.var_adsha);
        (assign14810_e20341, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14810_e20343;
        locals.var_temp_adeff_dn3 = assign14810_e20343_d_n3;
        locals.var_temp_adeff_dn4 = assign14810_e20343_d_n4;
        locals.var_temp_adeff_dn5 = assign14810_e20343_d_n5;
        locals.var_temp_adeff_dn6 = assign14810_e20343_d_n6;
        locals.var_temp_adeff_dn7 = assign14810_e20343_d_n7;
        locals.var_temp_adeff_dn8 = assign14810_e20343_d_n8;
        locals.var_temp_adeff_dn9 = assign14810_e20343_d_n9;
        locals.var_temp_adeff_dn10 = assign14810_e20343_d_n10;
        locals.var_temp_adeff_dn11 = assign14810_e20343_d_n11;

        let (assign14820_e20362, assign14820_e20362_d_n3, assign14820_e20362_d_n4, assign14820_e20362_d_n5, assign14820_e20362_d_n6, assign14820_e20362_d_n7, assign14820_e20362_d_n8, assign14820_e20362_d_n9, assign14820_e20362_d_n10, assign14820_e20362_d_n11,) = {
    if ((locals.var_guard463 != 0.0) && (!((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)))) {
        let assign14820_e20356: f64 = (locals.var_nuends * locals.var_psiso);
        let assign14820_e20359: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14820_e20360: f64 = (assign14820_e20356 + assign14820_e20359);
        (assign14820_e20360, ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14820_e20362;
        locals.var_temp_pseff_dn3 = assign14820_e20362_d_n3;
        locals.var_temp_pseff_dn4 = assign14820_e20362_d_n4;
        locals.var_temp_pseff_dn5 = assign14820_e20362_d_n5;
        locals.var_temp_pseff_dn6 = assign14820_e20362_d_n6;
        locals.var_temp_pseff_dn7 = assign14820_e20362_d_n7;
        locals.var_temp_pseff_dn8 = assign14820_e20362_d_n8;
        locals.var_temp_pseff_dn9 = assign14820_e20362_d_n9;
        locals.var_temp_pseff_dn10 = assign14820_e20362_d_n10;
        locals.var_temp_pseff_dn11 = assign14820_e20362_d_n11;

        let (assign14830_e20381, assign14830_e20381_d_n3, assign14830_e20381_d_n4, assign14830_e20381_d_n5, assign14830_e20381_d_n6, assign14830_e20381_d_n7, assign14830_e20381_d_n8, assign14830_e20381_d_n9, assign14830_e20381_d_n10, assign14830_e20381_d_n11,) = {
    if ((locals.var_guard463 != 0.0) && (!((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)))) {
        let assign14830_e20375: f64 = (locals.var_nuendd * locals.var_pdmer);
        let assign14830_e20378: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14830_e20379: f64 = (assign14830_e20375 + assign14830_e20378);
        (assign14830_e20379, ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14830_e20381;
        locals.var_temp_pdeff_dn3 = assign14830_e20381_d_n3;
        locals.var_temp_pdeff_dn4 = assign14830_e20381_d_n4;
        locals.var_temp_pdeff_dn5 = assign14830_e20381_d_n5;
        locals.var_temp_pdeff_dn6 = assign14830_e20381_d_n6;
        locals.var_temp_pdeff_dn7 = assign14830_e20381_d_n7;
        locals.var_temp_pdeff_dn8 = assign14830_e20381_d_n8;
        locals.var_temp_pdeff_dn9 = assign14830_e20381_d_n9;
        locals.var_temp_pdeff_dn10 = assign14830_e20381_d_n10;
        locals.var_temp_pdeff_dn11 = assign14830_e20381_d_n11;

        let (assign14840_e20400, assign14840_e20400_d_n3, assign14840_e20400_d_n4, assign14840_e20400_d_n5, assign14840_e20400_d_n6, assign14840_e20400_d_n7, assign14840_e20400_d_n8, assign14840_e20400_d_n9, assign14840_e20400_d_n10, assign14840_e20400_d_n11,) = {
    if ((locals.var_guard463 != 0.0) && (!((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)))) {
        let assign14840_e20394: f64 = (locals.var_nuends * locals.var_asiso);
        let assign14840_e20397: f64 = (locals.var_nuints * locals.var_assha);
        let assign14840_e20398: f64 = (assign14840_e20394 + assign14840_e20397);
        (assign14840_e20398, (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11),)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14840_e20400;
        locals.var_temp_aseff_dn3 = assign14840_e20400_d_n3;
        locals.var_temp_aseff_dn4 = assign14840_e20400_d_n4;
        locals.var_temp_aseff_dn5 = assign14840_e20400_d_n5;
        locals.var_temp_aseff_dn6 = assign14840_e20400_d_n6;
        locals.var_temp_aseff_dn7 = assign14840_e20400_d_n7;
        locals.var_temp_aseff_dn8 = assign14840_e20400_d_n8;
        locals.var_temp_aseff_dn9 = assign14840_e20400_d_n9;
        locals.var_temp_aseff_dn10 = assign14840_e20400_d_n10;
        locals.var_temp_aseff_dn11 = assign14840_e20400_d_n11;

        let (assign14850_e20419, assign14850_e20419_d_n3, assign14850_e20419_d_n4, assign14850_e20419_d_n5, assign14850_e20419_d_n6, assign14850_e20419_d_n7, assign14850_e20419_d_n8, assign14850_e20419_d_n9, assign14850_e20419_d_n10, assign14850_e20419_d_n11,) = {
    if ((locals.var_guard463 != 0.0) && (!((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)))) {
        let assign14850_e20413: f64 = (locals.var_nuendd * locals.var_admer);
        let assign14850_e20416: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14850_e20417: f64 = (assign14850_e20413 + assign14850_e20416);
        (assign14850_e20417, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14850_e20419;
        locals.var_temp_adeff_dn3 = assign14850_e20419_d_n3;
        locals.var_temp_adeff_dn4 = assign14850_e20419_d_n4;
        locals.var_temp_adeff_dn5 = assign14850_e20419_d_n5;
        locals.var_temp_adeff_dn6 = assign14850_e20419_d_n6;
        locals.var_temp_adeff_dn7 = assign14850_e20419_d_n7;
        locals.var_temp_adeff_dn8 = assign14850_e20419_d_n8;
        locals.var_temp_adeff_dn9 = assign14850_e20419_d_n9;
        locals.var_temp_adeff_dn10 = assign14850_e20419_d_n10;
        locals.var_temp_adeff_dn11 = assign14850_e20419_d_n11;

        let (assign14860_e20438, assign14860_e20438_d_n3, assign14860_e20438_d_n4, assign14860_e20438_d_n5, assign14860_e20438_d_n6, assign14860_e20438_d_n7, assign14860_e20438_d_n8, assign14860_e20438_d_n9, assign14860_e20438_d_n10, assign14860_e20438_d_n11,) = {
    if ((locals.var_guard464 != 0.0) && (!(((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)))) {
        let assign14860_e20434: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14860_e20436: f64 = (assign14860_e20434 * locals.var_pssha);
        (assign14860_e20436, (assign14860_e20434 * locals.var_pssha_dn3), (assign14860_e20434 * locals.var_pssha_dn4), (assign14860_e20434 * locals.var_pssha_dn5), (assign14860_e20434 * locals.var_pssha_dn6), (assign14860_e20434 * locals.var_pssha_dn7), (assign14860_e20434 * locals.var_pssha_dn8), (assign14860_e20434 * locals.var_pssha_dn9), (assign14860_e20434 * locals.var_pssha_dn10), (assign14860_e20434 * locals.var_pssha_dn11),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14860_e20438;
        locals.var_temp_pseff_dn3 = assign14860_e20438_d_n3;
        locals.var_temp_pseff_dn4 = assign14860_e20438_d_n4;
        locals.var_temp_pseff_dn5 = assign14860_e20438_d_n5;
        locals.var_temp_pseff_dn6 = assign14860_e20438_d_n6;
        locals.var_temp_pseff_dn7 = assign14860_e20438_d_n7;
        locals.var_temp_pseff_dn8 = assign14860_e20438_d_n8;
        locals.var_temp_pseff_dn9 = assign14860_e20438_d_n9;
        locals.var_temp_pseff_dn10 = assign14860_e20438_d_n10;
        locals.var_temp_pseff_dn11 = assign14860_e20438_d_n11;

        let (assign14870_e20459, assign14870_e20459_d_n3, assign14870_e20459_d_n4, assign14870_e20459_d_n5, assign14870_e20459_d_n6, assign14870_e20459_d_n7, assign14870_e20459_d_n8, assign14870_e20459_d_n9, assign14870_e20459_d_n10, assign14870_e20459_d_n11,) = {
    if ((locals.var_guard464 != 0.0) && (!(((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)))) {
        let assign14870_e20453: f64 = (locals.var_nuendd * locals.var_pdmer);
        let assign14870_e20456: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14870_e20457: f64 = (assign14870_e20453 + assign14870_e20456);
        (assign14870_e20457, ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14870_e20459;
        locals.var_temp_pdeff_dn3 = assign14870_e20459_d_n3;
        locals.var_temp_pdeff_dn4 = assign14870_e20459_d_n4;
        locals.var_temp_pdeff_dn5 = assign14870_e20459_d_n5;
        locals.var_temp_pdeff_dn6 = assign14870_e20459_d_n6;
        locals.var_temp_pdeff_dn7 = assign14870_e20459_d_n7;
        locals.var_temp_pdeff_dn8 = assign14870_e20459_d_n8;
        locals.var_temp_pdeff_dn9 = assign14870_e20459_d_n9;
        locals.var_temp_pdeff_dn10 = assign14870_e20459_d_n10;
        locals.var_temp_pdeff_dn11 = assign14870_e20459_d_n11;

        let (assign14880_e20478, assign14880_e20478_d_n3, assign14880_e20478_d_n4, assign14880_e20478_d_n5, assign14880_e20478_d_n6, assign14880_e20478_d_n7, assign14880_e20478_d_n8, assign14880_e20478_d_n9, assign14880_e20478_d_n10, assign14880_e20478_d_n11,) = {
    if ((locals.var_guard464 != 0.0) && (!(((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)))) {
        let assign14880_e20474: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14880_e20476: f64 = (assign14880_e20474 * locals.var_assha);
        (assign14880_e20476, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14880_e20478;
        locals.var_temp_aseff_dn3 = assign14880_e20478_d_n3;
        locals.var_temp_aseff_dn4 = assign14880_e20478_d_n4;
        locals.var_temp_aseff_dn5 = assign14880_e20478_d_n5;
        locals.var_temp_aseff_dn6 = assign14880_e20478_d_n6;
        locals.var_temp_aseff_dn7 = assign14880_e20478_d_n7;
        locals.var_temp_aseff_dn8 = assign14880_e20478_d_n8;
        locals.var_temp_aseff_dn9 = assign14880_e20478_d_n9;
        locals.var_temp_aseff_dn10 = assign14880_e20478_d_n10;
        locals.var_temp_aseff_dn11 = assign14880_e20478_d_n11;

        let (assign14890_e20499, assign14890_e20499_d_n3, assign14890_e20499_d_n4, assign14890_e20499_d_n5, assign14890_e20499_d_n6, assign14890_e20499_d_n7, assign14890_e20499_d_n8, assign14890_e20499_d_n9, assign14890_e20499_d_n10, assign14890_e20499_d_n11,) = {
    if ((locals.var_guard464 != 0.0) && (!(((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)))) {
        let assign14890_e20493: f64 = (locals.var_nuendd * locals.var_admer);
        let assign14890_e20496: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14890_e20497: f64 = (assign14890_e20493 + assign14890_e20496);
        (assign14890_e20497, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14890_e20499;
        locals.var_temp_adeff_dn3 = assign14890_e20499_d_n3;
        locals.var_temp_adeff_dn4 = assign14890_e20499_d_n4;
        locals.var_temp_adeff_dn5 = assign14890_e20499_d_n5;
        locals.var_temp_adeff_dn6 = assign14890_e20499_d_n6;
        locals.var_temp_adeff_dn7 = assign14890_e20499_d_n7;
        locals.var_temp_adeff_dn8 = assign14890_e20499_d_n8;
        locals.var_temp_adeff_dn9 = assign14890_e20499_d_n9;
        locals.var_temp_adeff_dn10 = assign14890_e20499_d_n10;
        locals.var_temp_adeff_dn11 = assign14890_e20499_d_n11;

        let (assign14900_e20522, assign14900_e20522_d_n3, assign14900_e20522_d_n4, assign14900_e20522_d_n5, assign14900_e20522_d_n6, assign14900_e20522_d_n7, assign14900_e20522_d_n8, assign14900_e20522_d_n9, assign14900_e20522_d_n10, assign14900_e20522_d_n11,) = {
    if ((locals.var_guard465 != 0.0) && (!((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)))) {
        let assign14900_e20516: f64 = (locals.var_nuends * locals.var_psmer);
        let assign14900_e20519: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14900_e20520: f64 = (assign14900_e20516 + assign14900_e20519);
        (assign14900_e20520, ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14900_e20522;
        locals.var_temp_pseff_dn3 = assign14900_e20522_d_n3;
        locals.var_temp_pseff_dn4 = assign14900_e20522_d_n4;
        locals.var_temp_pseff_dn5 = assign14900_e20522_d_n5;
        locals.var_temp_pseff_dn6 = assign14900_e20522_d_n6;
        locals.var_temp_pseff_dn7 = assign14900_e20522_d_n7;
        locals.var_temp_pseff_dn8 = assign14900_e20522_d_n8;
        locals.var_temp_pseff_dn9 = assign14900_e20522_d_n9;
        locals.var_temp_pseff_dn10 = assign14900_e20522_d_n10;
        locals.var_temp_pseff_dn11 = assign14900_e20522_d_n11;

        let (assign14910_e20545, assign14910_e20545_d_n3, assign14910_e20545_d_n4, assign14910_e20545_d_n5, assign14910_e20545_d_n6, assign14910_e20545_d_n7, assign14910_e20545_d_n8, assign14910_e20545_d_n9, assign14910_e20545_d_n10, assign14910_e20545_d_n11,) = {
    if ((locals.var_guard465 != 0.0) && (!((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)))) {
        let assign14910_e20539: f64 = (locals.var_nuendd * locals.var_pdiso);
        let assign14910_e20542: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14910_e20543: f64 = (assign14910_e20539 + assign14910_e20542);
        (assign14910_e20543, ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14910_e20545;
        locals.var_temp_pdeff_dn3 = assign14910_e20545_d_n3;
        locals.var_temp_pdeff_dn4 = assign14910_e20545_d_n4;
        locals.var_temp_pdeff_dn5 = assign14910_e20545_d_n5;
        locals.var_temp_pdeff_dn6 = assign14910_e20545_d_n6;
        locals.var_temp_pdeff_dn7 = assign14910_e20545_d_n7;
        locals.var_temp_pdeff_dn8 = assign14910_e20545_d_n8;
        locals.var_temp_pdeff_dn9 = assign14910_e20545_d_n9;
        locals.var_temp_pdeff_dn10 = assign14910_e20545_d_n10;
        locals.var_temp_pdeff_dn11 = assign14910_e20545_d_n11;

        let (assign14920_e20568, assign14920_e20568_d_n3, assign14920_e20568_d_n4, assign14920_e20568_d_n5, assign14920_e20568_d_n6, assign14920_e20568_d_n7, assign14920_e20568_d_n8, assign14920_e20568_d_n9, assign14920_e20568_d_n10, assign14920_e20568_d_n11,) = {
    if ((locals.var_guard465 != 0.0) && (!((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)))) {
        let assign14920_e20562: f64 = (locals.var_nuends * locals.var_asmer);
        let assign14920_e20565: f64 = (locals.var_nuints * locals.var_assha);
        let assign14920_e20566: f64 = (assign14920_e20562 + assign14920_e20565);
        (assign14920_e20566, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14920_e20568;
        locals.var_temp_aseff_dn3 = assign14920_e20568_d_n3;
        locals.var_temp_aseff_dn4 = assign14920_e20568_d_n4;
        locals.var_temp_aseff_dn5 = assign14920_e20568_d_n5;
        locals.var_temp_aseff_dn6 = assign14920_e20568_d_n6;
        locals.var_temp_aseff_dn7 = assign14920_e20568_d_n7;
        locals.var_temp_aseff_dn8 = assign14920_e20568_d_n8;
        locals.var_temp_aseff_dn9 = assign14920_e20568_d_n9;
        locals.var_temp_aseff_dn10 = assign14920_e20568_d_n10;
        locals.var_temp_aseff_dn11 = assign14920_e20568_d_n11;

    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign14930_e20591, assign14930_e20591_d_n3, assign14930_e20591_d_n4, assign14930_e20591_d_n5, assign14930_e20591_d_n6, assign14930_e20591_d_n7, assign14930_e20591_d_n8, assign14930_e20591_d_n9, assign14930_e20591_d_n10, assign14930_e20591_d_n11,) = {
    if ((locals.var_guard465 != 0.0) && (!((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)))) {
        let assign14930_e20585: f64 = (locals.var_nuendd * locals.var_adiso);
        let assign14930_e20588: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14930_e20589: f64 = (assign14930_e20585 + assign14930_e20588);
        (assign14930_e20589, (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11),)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14930_e20591;
        locals.var_temp_adeff_dn3 = assign14930_e20591_d_n3;
        locals.var_temp_adeff_dn4 = assign14930_e20591_d_n4;
        locals.var_temp_adeff_dn5 = assign14930_e20591_d_n5;
        locals.var_temp_adeff_dn6 = assign14930_e20591_d_n6;
        locals.var_temp_adeff_dn7 = assign14930_e20591_d_n7;
        locals.var_temp_adeff_dn8 = assign14930_e20591_d_n8;
        locals.var_temp_adeff_dn9 = assign14930_e20591_d_n9;
        locals.var_temp_adeff_dn10 = assign14930_e20591_d_n10;
        locals.var_temp_adeff_dn11 = assign14930_e20591_d_n11;

        let (assign14940_e20616, assign14940_e20616_d_n3, assign14940_e20616_d_n4, assign14940_e20616_d_n5, assign14940_e20616_d_n6, assign14940_e20616_d_n7, assign14940_e20616_d_n8, assign14940_e20616_d_n9, assign14940_e20616_d_n10, assign14940_e20616_d_n11,) = {
    if ((locals.var_guard466 != 0.0) && (!(((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)))) {
        let assign14940_e20610: f64 = (locals.var_nuends * locals.var_psmer);
        let assign14940_e20613: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14940_e20614: f64 = (assign14940_e20610 + assign14940_e20613);
        (assign14940_e20614, ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14940_e20616;
        locals.var_temp_pseff_dn3 = assign14940_e20616_d_n3;
        locals.var_temp_pseff_dn4 = assign14940_e20616_d_n4;
        locals.var_temp_pseff_dn5 = assign14940_e20616_d_n5;
        locals.var_temp_pseff_dn6 = assign14940_e20616_d_n6;
        locals.var_temp_pseff_dn7 = assign14940_e20616_d_n7;
        locals.var_temp_pseff_dn8 = assign14940_e20616_d_n8;
        locals.var_temp_pseff_dn9 = assign14940_e20616_d_n9;
        locals.var_temp_pseff_dn10 = assign14940_e20616_d_n10;
        locals.var_temp_pseff_dn11 = assign14940_e20616_d_n11;

        let (assign14950_e20639, assign14950_e20639_d_n3, assign14950_e20639_d_n4, assign14950_e20639_d_n5, assign14950_e20639_d_n6, assign14950_e20639_d_n7, assign14950_e20639_d_n8, assign14950_e20639_d_n9, assign14950_e20639_d_n10, assign14950_e20639_d_n11,) = {
    if ((locals.var_guard466 != 0.0) && (!(((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)))) {
        let assign14950_e20635: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14950_e20637: f64 = (assign14950_e20635 * locals.var_pdsha);
        (assign14950_e20637, (assign14950_e20635 * locals.var_pdsha_dn3), (assign14950_e20635 * locals.var_pdsha_dn4), (assign14950_e20635 * locals.var_pdsha_dn5), (assign14950_e20635 * locals.var_pdsha_dn6), (assign14950_e20635 * locals.var_pdsha_dn7), (assign14950_e20635 * locals.var_pdsha_dn8), (assign14950_e20635 * locals.var_pdsha_dn9), (assign14950_e20635 * locals.var_pdsha_dn10), (assign14950_e20635 * locals.var_pdsha_dn11),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14950_e20639;
        locals.var_temp_pdeff_dn3 = assign14950_e20639_d_n3;
        locals.var_temp_pdeff_dn4 = assign14950_e20639_d_n4;
        locals.var_temp_pdeff_dn5 = assign14950_e20639_d_n5;
        locals.var_temp_pdeff_dn6 = assign14950_e20639_d_n6;
        locals.var_temp_pdeff_dn7 = assign14950_e20639_d_n7;
        locals.var_temp_pdeff_dn8 = assign14950_e20639_d_n8;
        locals.var_temp_pdeff_dn9 = assign14950_e20639_d_n9;
        locals.var_temp_pdeff_dn10 = assign14950_e20639_d_n10;
        locals.var_temp_pdeff_dn11 = assign14950_e20639_d_n11;

        let (assign14960_e20664, assign14960_e20664_d_n3, assign14960_e20664_d_n4, assign14960_e20664_d_n5, assign14960_e20664_d_n6, assign14960_e20664_d_n7, assign14960_e20664_d_n8, assign14960_e20664_d_n9, assign14960_e20664_d_n10, assign14960_e20664_d_n11,) = {
    if ((locals.var_guard466 != 0.0) && (!(((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)))) {
        let assign14960_e20658: f64 = (locals.var_nuends * locals.var_asmer);
        let assign14960_e20661: f64 = (locals.var_nuints * locals.var_assha);
        let assign14960_e20662: f64 = (assign14960_e20658 + assign14960_e20661);
        (assign14960_e20662, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14960_e20664;
        locals.var_temp_aseff_dn3 = assign14960_e20664_d_n3;
        locals.var_temp_aseff_dn4 = assign14960_e20664_d_n4;
        locals.var_temp_aseff_dn5 = assign14960_e20664_d_n5;
        locals.var_temp_aseff_dn6 = assign14960_e20664_d_n6;
        locals.var_temp_aseff_dn7 = assign14960_e20664_d_n7;
        locals.var_temp_aseff_dn8 = assign14960_e20664_d_n8;
        locals.var_temp_aseff_dn9 = assign14960_e20664_d_n9;
        locals.var_temp_aseff_dn10 = assign14960_e20664_d_n10;
        locals.var_temp_aseff_dn11 = assign14960_e20664_d_n11;

        let (assign14970_e20687, assign14970_e20687_d_n3, assign14970_e20687_d_n4, assign14970_e20687_d_n5, assign14970_e20687_d_n6, assign14970_e20687_d_n7, assign14970_e20687_d_n8, assign14970_e20687_d_n9, assign14970_e20687_d_n10, assign14970_e20687_d_n11,) = {
    if ((locals.var_guard466 != 0.0) && (!(((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)))) {
        let assign14970_e20683: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14970_e20685: f64 = (assign14970_e20683 * locals.var_adsha);
        (assign14970_e20685, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14970_e20687;
        locals.var_temp_adeff_dn3 = assign14970_e20687_d_n3;
        locals.var_temp_adeff_dn4 = assign14970_e20687_d_n4;
        locals.var_temp_adeff_dn5 = assign14970_e20687_d_n5;
        locals.var_temp_adeff_dn6 = assign14970_e20687_d_n6;
        locals.var_temp_adeff_dn7 = assign14970_e20687_d_n7;
        locals.var_temp_adeff_dn8 = assign14970_e20687_d_n8;
        locals.var_temp_adeff_dn9 = assign14970_e20687_d_n9;
        locals.var_temp_adeff_dn10 = assign14970_e20687_d_n10;
        locals.var_temp_adeff_dn11 = assign14970_e20687_d_n11;

        let (assign14980_e20714, assign14980_e20714_d_n3, assign14980_e20714_d_n4, assign14980_e20714_d_n5, assign14980_e20714_d_n6, assign14980_e20714_d_n7, assign14980_e20714_d_n8, assign14980_e20714_d_n9, assign14980_e20714_d_n10, assign14980_e20714_d_n11,) = {
    if ((locals.var_guard467 != 0.0) && (!((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)))) {
        let assign14980_e20708: f64 = (locals.var_nuends * locals.var_psmer);
        let assign14980_e20711: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14980_e20712: f64 = (assign14980_e20708 + assign14980_e20711);
        (assign14980_e20712, ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14980_e20714;
        locals.var_temp_pseff_dn3 = assign14980_e20714_d_n3;
        locals.var_temp_pseff_dn4 = assign14980_e20714_d_n4;
        locals.var_temp_pseff_dn5 = assign14980_e20714_d_n5;
        locals.var_temp_pseff_dn6 = assign14980_e20714_d_n6;
        locals.var_temp_pseff_dn7 = assign14980_e20714_d_n7;
        locals.var_temp_pseff_dn8 = assign14980_e20714_d_n8;
        locals.var_temp_pseff_dn9 = assign14980_e20714_d_n9;
        locals.var_temp_pseff_dn10 = assign14980_e20714_d_n10;
        locals.var_temp_pseff_dn11 = assign14980_e20714_d_n11;

        let (assign14990_e20741, assign14990_e20741_d_n3, assign14990_e20741_d_n4, assign14990_e20741_d_n5, assign14990_e20741_d_n6, assign14990_e20741_d_n7, assign14990_e20741_d_n8, assign14990_e20741_d_n9, assign14990_e20741_d_n10, assign14990_e20741_d_n11,) = {
    if ((locals.var_guard467 != 0.0) && (!((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)))) {
        let assign14990_e20735: f64 = (locals.var_nuendd * locals.var_pdmer);
        let assign14990_e20738: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14990_e20739: f64 = (assign14990_e20735 + assign14990_e20738);
        (assign14990_e20739, ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14990_e20741;
        locals.var_temp_pdeff_dn3 = assign14990_e20741_d_n3;
        locals.var_temp_pdeff_dn4 = assign14990_e20741_d_n4;
        locals.var_temp_pdeff_dn5 = assign14990_e20741_d_n5;
        locals.var_temp_pdeff_dn6 = assign14990_e20741_d_n6;
        locals.var_temp_pdeff_dn7 = assign14990_e20741_d_n7;
        locals.var_temp_pdeff_dn8 = assign14990_e20741_d_n8;
        locals.var_temp_pdeff_dn9 = assign14990_e20741_d_n9;
        locals.var_temp_pdeff_dn10 = assign14990_e20741_d_n10;
        locals.var_temp_pdeff_dn11 = assign14990_e20741_d_n11;

        let (assign15000_e20768, assign15000_e20768_d_n3, assign15000_e20768_d_n4, assign15000_e20768_d_n5, assign15000_e20768_d_n6, assign15000_e20768_d_n7, assign15000_e20768_d_n8, assign15000_e20768_d_n9, assign15000_e20768_d_n10, assign15000_e20768_d_n11,) = {
    if ((locals.var_guard467 != 0.0) && (!((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)))) {
        let assign15000_e20762: f64 = (locals.var_nuends * locals.var_asmer);
        let assign15000_e20765: f64 = (locals.var_nuints * locals.var_assha);
        let assign15000_e20766: f64 = (assign15000_e20762 + assign15000_e20765);
        (assign15000_e20766, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign15000_e20768;
        locals.var_temp_aseff_dn3 = assign15000_e20768_d_n3;
        locals.var_temp_aseff_dn4 = assign15000_e20768_d_n4;
        locals.var_temp_aseff_dn5 = assign15000_e20768_d_n5;
        locals.var_temp_aseff_dn6 = assign15000_e20768_d_n6;
        locals.var_temp_aseff_dn7 = assign15000_e20768_d_n7;
        locals.var_temp_aseff_dn8 = assign15000_e20768_d_n8;
        locals.var_temp_aseff_dn9 = assign15000_e20768_d_n9;
        locals.var_temp_aseff_dn10 = assign15000_e20768_d_n10;
        locals.var_temp_aseff_dn11 = assign15000_e20768_d_n11;

        let (assign15010_e20795, assign15010_e20795_d_n3, assign15010_e20795_d_n4, assign15010_e20795_d_n5, assign15010_e20795_d_n6, assign15010_e20795_d_n7, assign15010_e20795_d_n8, assign15010_e20795_d_n9, assign15010_e20795_d_n10, assign15010_e20795_d_n11,) = {
    if ((locals.var_guard467 != 0.0) && (!((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)))) {
        let assign15010_e20789: f64 = (locals.var_nuendd * locals.var_admer);
        let assign15010_e20792: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign15010_e20793: f64 = (assign15010_e20789 + assign15010_e20792);
        (assign15010_e20793, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign15010_e20795;
        locals.var_temp_adeff_dn3 = assign15010_e20795_d_n3;
        locals.var_temp_adeff_dn4 = assign15010_e20795_d_n4;
        locals.var_temp_adeff_dn5 = assign15010_e20795_d_n5;
        locals.var_temp_adeff_dn6 = assign15010_e20795_d_n6;
        locals.var_temp_adeff_dn7 = assign15010_e20795_d_n7;
        locals.var_temp_adeff_dn8 = assign15010_e20795_d_n8;
        locals.var_temp_adeff_dn9 = assign15010_e20795_d_n9;
        locals.var_temp_adeff_dn10 = assign15010_e20795_d_n10;
        locals.var_temp_adeff_dn11 = assign15010_e20795_d_n11;

        let (assign15020_e20824, assign15020_e20824_d_n3, assign15020_e20824_d_n4, assign15020_e20824_d_n5, assign15020_e20824_d_n6, assign15020_e20824_d_n7, assign15020_e20824_d_n8, assign15020_e20824_d_n9, assign15020_e20824_d_n10, assign15020_e20824_d_n11,) = {
    if ((locals.var_guard468 != 0.0) && (!(((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign15020_e20819: f64 = (p.p2 - 1.0);
        let assign15020_e20821: f64 = (assign15020_e20819 * locals.var_pssha);
        let assign15020_e20822: f64 = (locals.var_psiso + assign15020_e20821);
        (assign15020_e20822, (locals.var_psiso_dn3 + (assign15020_e20819 * locals.var_pssha_dn3)), (locals.var_psiso_dn4 + (assign15020_e20819 * locals.var_pssha_dn4)), (locals.var_psiso_dn5 + (assign15020_e20819 * locals.var_pssha_dn5)), (locals.var_psiso_dn6 + (assign15020_e20819 * locals.var_pssha_dn6)), (locals.var_psiso_dn7 + (assign15020_e20819 * locals.var_pssha_dn7)), (locals.var_psiso_dn8 + (assign15020_e20819 * locals.var_pssha_dn8)), (locals.var_psiso_dn9 + (assign15020_e20819 * locals.var_pssha_dn9)), (locals.var_psiso_dn10 + (assign15020_e20819 * locals.var_pssha_dn10)), (locals.var_psiso_dn11 + (assign15020_e20819 * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign15020_e20824;
        locals.var_temp_pseff_dn3 = assign15020_e20824_d_n3;
        locals.var_temp_pseff_dn4 = assign15020_e20824_d_n4;
        locals.var_temp_pseff_dn5 = assign15020_e20824_d_n5;
        locals.var_temp_pseff_dn6 = assign15020_e20824_d_n6;
        locals.var_temp_pseff_dn7 = assign15020_e20824_d_n7;
        locals.var_temp_pseff_dn8 = assign15020_e20824_d_n8;
        locals.var_temp_pseff_dn9 = assign15020_e20824_d_n9;
        locals.var_temp_pseff_dn10 = assign15020_e20824_d_n10;
        locals.var_temp_pseff_dn11 = assign15020_e20824_d_n11;

        let (assign15030_e20849, assign15030_e20849_d_n3, assign15030_e20849_d_n4, assign15030_e20849_d_n5, assign15030_e20849_d_n6, assign15030_e20849_d_n7, assign15030_e20849_d_n8, assign15030_e20849_d_n9, assign15030_e20849_d_n10, assign15030_e20849_d_n11,) = {
    if ((locals.var_guard468 != 0.0) && (!(((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign15030_e20847: f64 = (p.p2 * locals.var_pdsha);
        (assign15030_e20847, (p.p2 * locals.var_pdsha_dn3), (p.p2 * locals.var_pdsha_dn4), (p.p2 * locals.var_pdsha_dn5), (p.p2 * locals.var_pdsha_dn6), (p.p2 * locals.var_pdsha_dn7), (p.p2 * locals.var_pdsha_dn8), (p.p2 * locals.var_pdsha_dn9), (p.p2 * locals.var_pdsha_dn10), (p.p2 * locals.var_pdsha_dn11),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign15030_e20849;
        locals.var_temp_pdeff_dn3 = assign15030_e20849_d_n3;
        locals.var_temp_pdeff_dn4 = assign15030_e20849_d_n4;
        locals.var_temp_pdeff_dn5 = assign15030_e20849_d_n5;
        locals.var_temp_pdeff_dn6 = assign15030_e20849_d_n6;
        locals.var_temp_pdeff_dn7 = assign15030_e20849_d_n7;
        locals.var_temp_pdeff_dn8 = assign15030_e20849_d_n8;
        locals.var_temp_pdeff_dn9 = assign15030_e20849_d_n9;
        locals.var_temp_pdeff_dn10 = assign15030_e20849_d_n10;
        locals.var_temp_pdeff_dn11 = assign15030_e20849_d_n11;

        let (assign15040_e20878, assign15040_e20878_d_n3, assign15040_e20878_d_n4, assign15040_e20878_d_n5, assign15040_e20878_d_n6, assign15040_e20878_d_n7, assign15040_e20878_d_n8, assign15040_e20878_d_n9, assign15040_e20878_d_n10, assign15040_e20878_d_n11,) = {
    if ((locals.var_guard468 != 0.0) && (!(((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign15040_e20873: f64 = (p.p2 - 1.0);
        let assign15040_e20875: f64 = (assign15040_e20873 * locals.var_assha);
        let assign15040_e20876: f64 = (locals.var_asiso + assign15040_e20875);
        (assign15040_e20876, locals.var_asiso_dn3, locals.var_asiso_dn4, locals.var_asiso_dn5, locals.var_asiso_dn6, locals.var_asiso_dn7, locals.var_asiso_dn8, locals.var_asiso_dn9, locals.var_asiso_dn10, locals.var_asiso_dn11,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign15040_e20878;
        locals.var_temp_aseff_dn3 = assign15040_e20878_d_n3;
        locals.var_temp_aseff_dn4 = assign15040_e20878_d_n4;
        locals.var_temp_aseff_dn5 = assign15040_e20878_d_n5;
        locals.var_temp_aseff_dn6 = assign15040_e20878_d_n6;
        locals.var_temp_aseff_dn7 = assign15040_e20878_d_n7;
        locals.var_temp_aseff_dn8 = assign15040_e20878_d_n8;
        locals.var_temp_aseff_dn9 = assign15040_e20878_d_n9;
        locals.var_temp_aseff_dn10 = assign15040_e20878_d_n10;
        locals.var_temp_aseff_dn11 = assign15040_e20878_d_n11;

        let (assign15050_e20903, assign15050_e20903_d_n3, assign15050_e20903_d_n4, assign15050_e20903_d_n5, assign15050_e20903_d_n6, assign15050_e20903_d_n7, assign15050_e20903_d_n8, assign15050_e20903_d_n9, assign15050_e20903_d_n10, assign15050_e20903_d_n11,) = {
    if ((locals.var_guard468 != 0.0) && (!(((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign15050_e20901: f64 = (p.p2 * locals.var_adsha);
        (assign15050_e20901, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign15050_e20903;
        locals.var_temp_adeff_dn3 = assign15050_e20903_d_n3;
        locals.var_temp_adeff_dn4 = assign15050_e20903_d_n4;
        locals.var_temp_adeff_dn5 = assign15050_e20903_d_n5;
        locals.var_temp_adeff_dn6 = assign15050_e20903_d_n6;
        locals.var_temp_adeff_dn7 = assign15050_e20903_d_n7;
        locals.var_temp_adeff_dn8 = assign15050_e20903_d_n8;
        locals.var_temp_adeff_dn9 = assign15050_e20903_d_n9;
        locals.var_temp_adeff_dn10 = assign15050_e20903_d_n10;
        locals.var_temp_adeff_dn11 = assign15050_e20903_d_n11;

        let (assign15060_e20930, assign15060_e20930_d_n3, assign15060_e20930_d_n4, assign15060_e20930_d_n5, assign15060_e20930_d_n6, assign15060_e20930_d_n7, assign15060_e20930_d_n8, assign15060_e20930_d_n9, assign15060_e20930_d_n10, assign15060_e20930_d_n11,) = {
    if ((locals.var_guard469 != 0.0) && (!((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign15060_e20928: f64 = (p.p2 * locals.var_pssha);
        (assign15060_e20928, (p.p2 * locals.var_pssha_dn3), (p.p2 * locals.var_pssha_dn4), (p.p2 * locals.var_pssha_dn5), (p.p2 * locals.var_pssha_dn6), (p.p2 * locals.var_pssha_dn7), (p.p2 * locals.var_pssha_dn8), (p.p2 * locals.var_pssha_dn9), (p.p2 * locals.var_pssha_dn10), (p.p2 * locals.var_pssha_dn11),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign15060_e20930;
        locals.var_temp_pseff_dn3 = assign15060_e20930_d_n3;
        locals.var_temp_pseff_dn4 = assign15060_e20930_d_n4;
        locals.var_temp_pseff_dn5 = assign15060_e20930_d_n5;
        locals.var_temp_pseff_dn6 = assign15060_e20930_d_n6;
        locals.var_temp_pseff_dn7 = assign15060_e20930_d_n7;
        locals.var_temp_pseff_dn8 = assign15060_e20930_d_n8;
        locals.var_temp_pseff_dn9 = assign15060_e20930_d_n9;
        locals.var_temp_pseff_dn10 = assign15060_e20930_d_n10;
        locals.var_temp_pseff_dn11 = assign15060_e20930_d_n11;

        let (assign15070_e20961, assign15070_e20961_d_n3, assign15070_e20961_d_n4, assign15070_e20961_d_n5, assign15070_e20961_d_n6, assign15070_e20961_d_n7, assign15070_e20961_d_n8, assign15070_e20961_d_n9, assign15070_e20961_d_n10, assign15070_e20961_d_n11,) = {
    if ((locals.var_guard469 != 0.0) && (!((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign15070_e20956: f64 = (p.p2 - 1.0);
        let assign15070_e20958: f64 = (assign15070_e20956 * locals.var_pdsha);
        let assign15070_e20959: f64 = (locals.var_pdiso + assign15070_e20958);
        (assign15070_e20959, (locals.var_pdiso_dn3 + (assign15070_e20956 * locals.var_pdsha_dn3)), (locals.var_pdiso_dn4 + (assign15070_e20956 * locals.var_pdsha_dn4)), (locals.var_pdiso_dn5 + (assign15070_e20956 * locals.var_pdsha_dn5)), (locals.var_pdiso_dn6 + (assign15070_e20956 * locals.var_pdsha_dn6)), (locals.var_pdiso_dn7 + (assign15070_e20956 * locals.var_pdsha_dn7)), (locals.var_pdiso_dn8 + (assign15070_e20956 * locals.var_pdsha_dn8)), (locals.var_pdiso_dn9 + (assign15070_e20956 * locals.var_pdsha_dn9)), (locals.var_pdiso_dn10 + (assign15070_e20956 * locals.var_pdsha_dn10)), (locals.var_pdiso_dn11 + (assign15070_e20956 * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign15070_e20961;
        locals.var_temp_pdeff_dn3 = assign15070_e20961_d_n3;
        locals.var_temp_pdeff_dn4 = assign15070_e20961_d_n4;
        locals.var_temp_pdeff_dn5 = assign15070_e20961_d_n5;
        locals.var_temp_pdeff_dn6 = assign15070_e20961_d_n6;
        locals.var_temp_pdeff_dn7 = assign15070_e20961_d_n7;
        locals.var_temp_pdeff_dn8 = assign15070_e20961_d_n8;
        locals.var_temp_pdeff_dn9 = assign15070_e20961_d_n9;
        locals.var_temp_pdeff_dn10 = assign15070_e20961_d_n10;
        locals.var_temp_pdeff_dn11 = assign15070_e20961_d_n11;

        let (assign15080_e20988, assign15080_e20988_d_n3, assign15080_e20988_d_n4, assign15080_e20988_d_n5, assign15080_e20988_d_n6, assign15080_e20988_d_n7, assign15080_e20988_d_n8, assign15080_e20988_d_n9, assign15080_e20988_d_n10, assign15080_e20988_d_n11,) = {
    if ((locals.var_guard469 != 0.0) && (!((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign15080_e20986: f64 = (p.p2 * locals.var_assha);
        (assign15080_e20986, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign15080_e20988;
        locals.var_temp_aseff_dn3 = assign15080_e20988_d_n3;
        locals.var_temp_aseff_dn4 = assign15080_e20988_d_n4;
        locals.var_temp_aseff_dn5 = assign15080_e20988_d_n5;
        locals.var_temp_aseff_dn6 = assign15080_e20988_d_n6;
        locals.var_temp_aseff_dn7 = assign15080_e20988_d_n7;
        locals.var_temp_aseff_dn8 = assign15080_e20988_d_n8;
        locals.var_temp_aseff_dn9 = assign15080_e20988_d_n9;
        locals.var_temp_aseff_dn10 = assign15080_e20988_d_n10;
        locals.var_temp_aseff_dn11 = assign15080_e20988_d_n11;

        let (assign15090_e21019, assign15090_e21019_d_n3, assign15090_e21019_d_n4, assign15090_e21019_d_n5, assign15090_e21019_d_n6, assign15090_e21019_d_n7, assign15090_e21019_d_n8, assign15090_e21019_d_n9, assign15090_e21019_d_n10, assign15090_e21019_d_n11,) = {
    if ((locals.var_guard469 != 0.0) && (!((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign15090_e21014: f64 = (p.p2 - 1.0);
        let assign15090_e21016: f64 = (assign15090_e21014 * locals.var_adsha);
        let assign15090_e21017: f64 = (locals.var_adiso + assign15090_e21016);
        (assign15090_e21017, locals.var_adiso_dn3, locals.var_adiso_dn4, locals.var_adiso_dn5, locals.var_adiso_dn6, locals.var_adiso_dn7, locals.var_adiso_dn8, locals.var_adiso_dn9, locals.var_adiso_dn10, locals.var_adiso_dn11,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign15090_e21019;
        locals.var_temp_adeff_dn3 = assign15090_e21019_d_n3;
        locals.var_temp_adeff_dn4 = assign15090_e21019_d_n4;
        locals.var_temp_adeff_dn5 = assign15090_e21019_d_n5;
        locals.var_temp_adeff_dn6 = assign15090_e21019_d_n6;
        locals.var_temp_adeff_dn7 = assign15090_e21019_d_n7;
        locals.var_temp_adeff_dn8 = assign15090_e21019_d_n8;
        locals.var_temp_adeff_dn9 = assign15090_e21019_d_n9;
        locals.var_temp_adeff_dn10 = assign15090_e21019_d_n10;
        locals.var_temp_adeff_dn11 = assign15090_e21019_d_n11;

        let (assign15100_e21044, assign15100_e21044_d_n3, assign15100_e21044_d_n4, assign15100_e21044_d_n5, assign15100_e21044_d_n6, assign15100_e21044_d_n7, assign15100_e21044_d_n8, assign15100_e21044_d_n9, assign15100_e21044_d_n10, assign15100_e21044_d_n11,) = {
    if (!(((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign15100_e21044;
        locals.var_temp_pseff_dn3 = assign15100_e21044_d_n3;
        locals.var_temp_pseff_dn4 = assign15100_e21044_d_n4;
        locals.var_temp_pseff_dn5 = assign15100_e21044_d_n5;
        locals.var_temp_pseff_dn6 = assign15100_e21044_d_n6;
        locals.var_temp_pseff_dn7 = assign15100_e21044_d_n7;
        locals.var_temp_pseff_dn8 = assign15100_e21044_d_n8;
        locals.var_temp_pseff_dn9 = assign15100_e21044_d_n9;
        locals.var_temp_pseff_dn10 = assign15100_e21044_d_n10;
        locals.var_temp_pseff_dn11 = assign15100_e21044_d_n11;

        let (assign15110_e21069, assign15110_e21069_d_n3, assign15110_e21069_d_n4, assign15110_e21069_d_n5, assign15110_e21069_d_n6, assign15110_e21069_d_n7, assign15110_e21069_d_n8, assign15110_e21069_d_n9, assign15110_e21069_d_n10, assign15110_e21069_d_n11,) = {
    if (!(((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign15110_e21069;
        locals.var_temp_pdeff_dn3 = assign15110_e21069_d_n3;
        locals.var_temp_pdeff_dn4 = assign15110_e21069_d_n4;
        locals.var_temp_pdeff_dn5 = assign15110_e21069_d_n5;
        locals.var_temp_pdeff_dn6 = assign15110_e21069_d_n6;
        locals.var_temp_pdeff_dn7 = assign15110_e21069_d_n7;
        locals.var_temp_pdeff_dn8 = assign15110_e21069_d_n8;
        locals.var_temp_pdeff_dn9 = assign15110_e21069_d_n9;
        locals.var_temp_pdeff_dn10 = assign15110_e21069_d_n10;
        locals.var_temp_pdeff_dn11 = assign15110_e21069_d_n11;

        let (assign15120_e21094, assign15120_e21094_d_n3, assign15120_e21094_d_n4, assign15120_e21094_d_n5, assign15120_e21094_d_n6, assign15120_e21094_d_n7, assign15120_e21094_d_n8, assign15120_e21094_d_n9, assign15120_e21094_d_n10, assign15120_e21094_d_n11,) = {
    if (!(((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign15120_e21094;
        locals.var_temp_aseff_dn3 = assign15120_e21094_d_n3;
        locals.var_temp_aseff_dn4 = assign15120_e21094_d_n4;
        locals.var_temp_aseff_dn5 = assign15120_e21094_d_n5;
        locals.var_temp_aseff_dn6 = assign15120_e21094_d_n6;
        locals.var_temp_aseff_dn7 = assign15120_e21094_d_n7;
        locals.var_temp_aseff_dn8 = assign15120_e21094_d_n8;
        locals.var_temp_aseff_dn9 = assign15120_e21094_d_n9;
        locals.var_temp_aseff_dn10 = assign15120_e21094_d_n10;
        locals.var_temp_aseff_dn11 = assign15120_e21094_d_n11;

        let (assign15130_e21119, assign15130_e21119_d_n3, assign15130_e21119_d_n4, assign15130_e21119_d_n5, assign15130_e21119_d_n6, assign15130_e21119_d_n7, assign15130_e21119_d_n8, assign15130_e21119_d_n9, assign15130_e21119_d_n10, assign15130_e21119_d_n11,) = {
    if (!(((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign15130_e21119;
        locals.var_temp_adeff_dn3 = assign15130_e21119_d_n3;
        locals.var_temp_adeff_dn4 = assign15130_e21119_d_n4;
        locals.var_temp_adeff_dn5 = assign15130_e21119_d_n5;
        locals.var_temp_adeff_dn6 = assign15130_e21119_d_n6;
        locals.var_temp_adeff_dn7 = assign15130_e21119_d_n7;
        locals.var_temp_adeff_dn8 = assign15130_e21119_d_n8;
        locals.var_temp_adeff_dn9 = assign15130_e21119_d_n9;
        locals.var_temp_adeff_dn10 = assign15130_e21119_d_n10;
        locals.var_temp_adeff_dn11 = assign15130_e21119_d_n11;

        let assign15140_e21121: f64 = if param_given[17] { 1.0 } else { 0.0 };
        locals.var_guard470 = assign15140_e21121;

        let (assign15150_e21129, assign15150_e21129_d_n3, assign15150_e21129_d_n4, assign15150_e21129_d_n5, assign15150_e21129_d_n6, assign15150_e21129_d_n7, assign15150_e21129_d_n8, assign15150_e21129_d_n9, assign15150_e21129_d_n10, assign15150_e21129_d_n11,) = {
    if (locals.var_guard470 != 0.0) {
        let assign15150_e21125: f64 = (p.p17 * p.p50);
        let assign15150_e21127: f64 = (assign15150_e21125 * p.p49);
        (assign15150_e21127, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_aseff, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11,)
    }
};
        locals.var_aseff = assign15150_e21129;
        locals.var_aseff_dn3 = assign15150_e21129_d_n3;
        locals.var_aseff_dn4 = assign15150_e21129_d_n4;
        locals.var_aseff_dn5 = assign15150_e21129_d_n5;
        locals.var_aseff_dn6 = assign15150_e21129_d_n6;
        locals.var_aseff_dn7 = assign15150_e21129_d_n7;
        locals.var_aseff_dn8 = assign15150_e21129_d_n8;
        locals.var_aseff_dn9 = assign15150_e21129_d_n9;
        locals.var_aseff_dn10 = assign15150_e21129_d_n10;
        locals.var_aseff_dn11 = assign15150_e21129_d_n11;

        let (assign15160_e21134, assign15160_e21134_d_n3, assign15160_e21134_d_n4, assign15160_e21134_d_n5, assign15160_e21134_d_n6, assign15160_e21134_d_n7, assign15160_e21134_d_n8, assign15160_e21134_d_n9, assign15160_e21134_d_n10, assign15160_e21134_d_n11,) = {
    if (locals.var_guard470 == 0.0) {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    } else {
        (locals.var_aseff, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11,)
    }
};
        locals.var_aseff = assign15160_e21134;
        locals.var_aseff_dn3 = assign15160_e21134_d_n3;
        locals.var_aseff_dn4 = assign15160_e21134_d_n4;
        locals.var_aseff_dn5 = assign15160_e21134_d_n5;
        locals.var_aseff_dn6 = assign15160_e21134_d_n6;
        locals.var_aseff_dn7 = assign15160_e21134_d_n7;
        locals.var_aseff_dn8 = assign15160_e21134_d_n8;
        locals.var_aseff_dn9 = assign15160_e21134_d_n9;
        locals.var_aseff_dn10 = assign15160_e21134_d_n10;
        locals.var_aseff_dn11 = assign15160_e21134_d_n11;

        let assign15170_e21137: f64 = if locals.var_aseff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard471 = assign15170_e21137;

        let (assign15180_e21141, assign15180_e21141_d_n3, assign15180_e21141_d_n4, assign15180_e21141_d_n5, assign15180_e21141_d_n6, assign15180_e21141_d_n7, assign15180_e21141_d_n8, assign15180_e21141_d_n9, assign15180_e21141_d_n10, assign15180_e21141_d_n11,) = {
    if (locals.var_guard471 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_aseff, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11,)
    }
};
        locals.var_aseff = assign15180_e21141;
        locals.var_aseff_dn3 = assign15180_e21141_d_n3;
        locals.var_aseff_dn4 = assign15180_e21141_d_n4;
        locals.var_aseff_dn5 = assign15180_e21141_d_n5;
        locals.var_aseff_dn6 = assign15180_e21141_d_n6;
        locals.var_aseff_dn7 = assign15180_e21141_d_n7;
        locals.var_aseff_dn8 = assign15180_e21141_d_n8;
        locals.var_aseff_dn9 = assign15180_e21141_d_n9;
        locals.var_aseff_dn10 = assign15180_e21141_d_n10;
        locals.var_aseff_dn11 = assign15180_e21141_d_n11;

        let assign15190_e21143: f64 = if param_given[18] { 1.0 } else { 0.0 };
        locals.var_guard472 = assign15190_e21143;

        let (assign15200_e21151, assign15200_e21151_d_n3, assign15200_e21151_d_n4, assign15200_e21151_d_n5, assign15200_e21151_d_n6, assign15200_e21151_d_n7, assign15200_e21151_d_n8, assign15200_e21151_d_n9, assign15200_e21151_d_n10, assign15200_e21151_d_n11,) = {
    if (locals.var_guard472 != 0.0) {
        let assign15200_e21147: f64 = (p.p18 * p.p50);
        let assign15200_e21149: f64 = (assign15200_e21147 * p.p49);
        (assign15200_e21149, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_adeff, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11,)
    }
};
        locals.var_adeff = assign15200_e21151;
        locals.var_adeff_dn3 = assign15200_e21151_d_n3;
        locals.var_adeff_dn4 = assign15200_e21151_d_n4;
        locals.var_adeff_dn5 = assign15200_e21151_d_n5;
        locals.var_adeff_dn6 = assign15200_e21151_d_n6;
        locals.var_adeff_dn7 = assign15200_e21151_d_n7;
        locals.var_adeff_dn8 = assign15200_e21151_d_n8;
        locals.var_adeff_dn9 = assign15200_e21151_d_n9;
        locals.var_adeff_dn10 = assign15200_e21151_d_n10;
        locals.var_adeff_dn11 = assign15200_e21151_d_n11;

    }

    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign15210_e21156, assign15210_e21156_d_n3, assign15210_e21156_d_n4, assign15210_e21156_d_n5, assign15210_e21156_d_n6, assign15210_e21156_d_n7, assign15210_e21156_d_n8, assign15210_e21156_d_n9, assign15210_e21156_d_n10, assign15210_e21156_d_n11,) = {
    if (locals.var_guard472 == 0.0) {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    } else {
        (locals.var_adeff, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11,)
    }
};
        locals.var_adeff = assign15210_e21156;
        locals.var_adeff_dn3 = assign15210_e21156_d_n3;
        locals.var_adeff_dn4 = assign15210_e21156_d_n4;
        locals.var_adeff_dn5 = assign15210_e21156_d_n5;
        locals.var_adeff_dn6 = assign15210_e21156_d_n6;
        locals.var_adeff_dn7 = assign15210_e21156_d_n7;
        locals.var_adeff_dn8 = assign15210_e21156_d_n8;
        locals.var_adeff_dn9 = assign15210_e21156_d_n9;
        locals.var_adeff_dn10 = assign15210_e21156_d_n10;
        locals.var_adeff_dn11 = assign15210_e21156_d_n11;

        let assign15220_e21159: f64 = if locals.var_adeff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign15220_e21159;

        let (assign15230_e21163, assign15230_e21163_d_n3, assign15230_e21163_d_n4, assign15230_e21163_d_n5, assign15230_e21163_d_n6, assign15230_e21163_d_n7, assign15230_e21163_d_n8, assign15230_e21163_d_n9, assign15230_e21163_d_n10, assign15230_e21163_d_n11,) = {
    if (locals.var_guard473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_adeff, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11,)
    }
};
        locals.var_adeff = assign15230_e21163;
        locals.var_adeff_dn3 = assign15230_e21163_d_n3;
        locals.var_adeff_dn4 = assign15230_e21163_d_n4;
        locals.var_adeff_dn5 = assign15230_e21163_d_n5;
        locals.var_adeff_dn6 = assign15230_e21163_d_n6;
        locals.var_adeff_dn7 = assign15230_e21163_d_n7;
        locals.var_adeff_dn8 = assign15230_e21163_d_n8;
        locals.var_adeff_dn9 = assign15230_e21163_d_n9;
        locals.var_adeff_dn10 = assign15230_e21163_d_n10;
        locals.var_adeff_dn11 = assign15230_e21163_d_n11;

        let assign15240_e21165: f64 = if param_given[19] { 1.0 } else { 0.0 };
        locals.var_guard474 = assign15240_e21165;

        let assign15250_e21168: f64 = if p.p926 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign15250_e21168;

        let (assign15260_e21176, assign15260_e21176_d_n3, assign15260_e21176_d_n4, assign15260_e21176_d_n5, assign15260_e21176_d_n6, assign15260_e21176_d_n7, assign15260_e21176_d_n8, assign15260_e21176_d_n9, assign15260_e21176_d_n10, assign15260_e21176_d_n11,) = {
    if ((locals.var_guard474 != 0.0) && (locals.var_guard475 != 0.0)) {
        let assign15260_e21174: f64 = (p.p19 * p.p50);
        (assign15260_e21174, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11,)
    }
};
        locals.var_pseff = assign15260_e21176;
        locals.var_pseff_dn3 = assign15260_e21176_d_n3;
        locals.var_pseff_dn4 = assign15260_e21176_d_n4;
        locals.var_pseff_dn5 = assign15260_e21176_d_n5;
        locals.var_pseff_dn6 = assign15260_e21176_d_n6;
        locals.var_pseff_dn7 = assign15260_e21176_d_n7;
        locals.var_pseff_dn8 = assign15260_e21176_d_n8;
        locals.var_pseff_dn9 = assign15260_e21176_d_n9;
        locals.var_pseff_dn10 = assign15260_e21176_d_n10;
        locals.var_pseff_dn11 = assign15260_e21176_d_n11;

        let (assign15270_e21191, assign15270_e21191_d_n3, assign15270_e21191_d_n4, assign15270_e21191_d_n5, assign15270_e21191_d_n6, assign15270_e21191_d_n7, assign15270_e21191_d_n8, assign15270_e21191_d_n9, assign15270_e21191_d_n10, assign15270_e21191_d_n11,) = {
    if ((locals.var_guard474 != 0.0) && (locals.var_guard475 == 0.0)) {
        let assign15270_e21183: f64 = (p.p19 * p.p50);
        let assign15270_e21186: f64 = (locals.var_weffcj * p.p2);
        let assign15270_e21187: f64 = (assign15270_e21183 - assign15270_e21186);
        let assign15270_e21189: f64 = (assign15270_e21187).max(0.0);
        (assign15270_e21189, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11,)
    }
};
        locals.var_pseff = assign15270_e21191;
        locals.var_pseff_dn3 = assign15270_e21191_d_n3;
        locals.var_pseff_dn4 = assign15270_e21191_d_n4;
        locals.var_pseff_dn5 = assign15270_e21191_d_n5;
        locals.var_pseff_dn6 = assign15270_e21191_d_n6;
        locals.var_pseff_dn7 = assign15270_e21191_d_n7;
        locals.var_pseff_dn8 = assign15270_e21191_d_n8;
        locals.var_pseff_dn9 = assign15270_e21191_d_n9;
        locals.var_pseff_dn10 = assign15270_e21191_d_n10;
        locals.var_pseff_dn11 = assign15270_e21191_d_n11;

        let (assign15280_e21196, assign15280_e21196_d_n3, assign15280_e21196_d_n4, assign15280_e21196_d_n5, assign15280_e21196_d_n6, assign15280_e21196_d_n7, assign15280_e21196_d_n8, assign15280_e21196_d_n9, assign15280_e21196_d_n10, assign15280_e21196_d_n11,) = {
    if (locals.var_guard474 == 0.0) {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11,)
    }
};
        locals.var_pseff = assign15280_e21196;
        locals.var_pseff_dn3 = assign15280_e21196_d_n3;
        locals.var_pseff_dn4 = assign15280_e21196_d_n4;
        locals.var_pseff_dn5 = assign15280_e21196_d_n5;
        locals.var_pseff_dn6 = assign15280_e21196_d_n6;
        locals.var_pseff_dn7 = assign15280_e21196_d_n7;
        locals.var_pseff_dn8 = assign15280_e21196_d_n8;
        locals.var_pseff_dn9 = assign15280_e21196_d_n9;
        locals.var_pseff_dn10 = assign15280_e21196_d_n10;
        locals.var_pseff_dn11 = assign15280_e21196_d_n11;

        let assign15290_e21199: f64 = if locals.var_pseff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard476 = assign15290_e21199;

        let (assign15300_e21206, assign15300_e21206_d_n3, assign15300_e21206_d_n4, assign15300_e21206_d_n5, assign15300_e21206_d_n6, assign15300_e21206_d_n7, assign15300_e21206_d_n8, assign15300_e21206_d_n9, assign15300_e21206_d_n10, assign15300_e21206_d_n11,) = {
    if ((locals.var_guard474 == 0.0) && (locals.var_guard476 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11,)
    }
};
        locals.var_pseff = assign15300_e21206;
        locals.var_pseff_dn3 = assign15300_e21206_d_n3;
        locals.var_pseff_dn4 = assign15300_e21206_d_n4;
        locals.var_pseff_dn5 = assign15300_e21206_d_n5;
        locals.var_pseff_dn6 = assign15300_e21206_d_n6;
        locals.var_pseff_dn7 = assign15300_e21206_d_n7;
        locals.var_pseff_dn8 = assign15300_e21206_d_n8;
        locals.var_pseff_dn9 = assign15300_e21206_d_n9;
        locals.var_pseff_dn10 = assign15300_e21206_d_n10;
        locals.var_pseff_dn11 = assign15300_e21206_d_n11;

        let assign15310_e21208: f64 = if param_given[20] { 1.0 } else { 0.0 };
        locals.var_guard477 = assign15310_e21208;

        let assign15320_e21211: f64 = if p.p926 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard478 = assign15320_e21211;

        let (assign15330_e21219, assign15330_e21219_d_n3, assign15330_e21219_d_n4, assign15330_e21219_d_n5, assign15330_e21219_d_n6, assign15330_e21219_d_n7, assign15330_e21219_d_n8, assign15330_e21219_d_n9, assign15330_e21219_d_n10, assign15330_e21219_d_n11,) = {
    if ((locals.var_guard477 != 0.0) && (locals.var_guard478 != 0.0)) {
        let assign15330_e21217: f64 = (p.p20 * p.p50);
        (assign15330_e21217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11,)
    }
};
        locals.var_pdeff = assign15330_e21219;
        locals.var_pdeff_dn3 = assign15330_e21219_d_n3;
        locals.var_pdeff_dn4 = assign15330_e21219_d_n4;
        locals.var_pdeff_dn5 = assign15330_e21219_d_n5;
        locals.var_pdeff_dn6 = assign15330_e21219_d_n6;
        locals.var_pdeff_dn7 = assign15330_e21219_d_n7;
        locals.var_pdeff_dn8 = assign15330_e21219_d_n8;
        locals.var_pdeff_dn9 = assign15330_e21219_d_n9;
        locals.var_pdeff_dn10 = assign15330_e21219_d_n10;
        locals.var_pdeff_dn11 = assign15330_e21219_d_n11;

        let (assign15340_e21234, assign15340_e21234_d_n3, assign15340_e21234_d_n4, assign15340_e21234_d_n5, assign15340_e21234_d_n6, assign15340_e21234_d_n7, assign15340_e21234_d_n8, assign15340_e21234_d_n9, assign15340_e21234_d_n10, assign15340_e21234_d_n11,) = {
    if ((locals.var_guard477 != 0.0) && (locals.var_guard478 == 0.0)) {
        let assign15340_e21226: f64 = (p.p20 * p.p50);
        let assign15340_e21229: f64 = (locals.var_weffcj * p.p2);
        let assign15340_e21230: f64 = (assign15340_e21226 - assign15340_e21229);
        let assign15340_e21232: f64 = (assign15340_e21230).max(0.0);
        (assign15340_e21232, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11,)
    }
};
        locals.var_pdeff = assign15340_e21234;
        locals.var_pdeff_dn3 = assign15340_e21234_d_n3;
        locals.var_pdeff_dn4 = assign15340_e21234_d_n4;
        locals.var_pdeff_dn5 = assign15340_e21234_d_n5;
        locals.var_pdeff_dn6 = assign15340_e21234_d_n6;
        locals.var_pdeff_dn7 = assign15340_e21234_d_n7;
        locals.var_pdeff_dn8 = assign15340_e21234_d_n8;
        locals.var_pdeff_dn9 = assign15340_e21234_d_n9;
        locals.var_pdeff_dn10 = assign15340_e21234_d_n10;
        locals.var_pdeff_dn11 = assign15340_e21234_d_n11;

        let (assign15350_e21239, assign15350_e21239_d_n3, assign15350_e21239_d_n4, assign15350_e21239_d_n5, assign15350_e21239_d_n6, assign15350_e21239_d_n7, assign15350_e21239_d_n8, assign15350_e21239_d_n9, assign15350_e21239_d_n10, assign15350_e21239_d_n11,) = {
    if (locals.var_guard477 == 0.0) {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11,)
    }
};
        locals.var_pdeff = assign15350_e21239;
        locals.var_pdeff_dn3 = assign15350_e21239_d_n3;
        locals.var_pdeff_dn4 = assign15350_e21239_d_n4;
        locals.var_pdeff_dn5 = assign15350_e21239_d_n5;
        locals.var_pdeff_dn6 = assign15350_e21239_d_n6;
        locals.var_pdeff_dn7 = assign15350_e21239_d_n7;
        locals.var_pdeff_dn8 = assign15350_e21239_d_n8;
        locals.var_pdeff_dn9 = assign15350_e21239_d_n9;
        locals.var_pdeff_dn10 = assign15350_e21239_d_n10;
        locals.var_pdeff_dn11 = assign15350_e21239_d_n11;

        let assign15360_e21242: f64 = if locals.var_pdeff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard479 = assign15360_e21242;

        let (assign15370_e21249, assign15370_e21249_d_n3, assign15370_e21249_d_n4, assign15370_e21249_d_n5, assign15370_e21249_d_n6, assign15370_e21249_d_n7, assign15370_e21249_d_n8, assign15370_e21249_d_n9, assign15370_e21249_d_n10, assign15370_e21249_d_n11,) = {
    if ((locals.var_guard477 == 0.0) && (locals.var_guard479 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11,)
    }
};
        locals.var_pdeff = assign15370_e21249;
        locals.var_pdeff_dn3 = assign15370_e21249_d_n3;
        locals.var_pdeff_dn4 = assign15370_e21249_d_n4;
        locals.var_pdeff_dn5 = assign15370_e21249_d_n5;
        locals.var_pdeff_dn6 = assign15370_e21249_d_n6;
        locals.var_pdeff_dn7 = assign15370_e21249_d_n7;
        locals.var_pdeff_dn8 = assign15370_e21249_d_n8;
        locals.var_pdeff_dn9 = assign15370_e21249_d_n9;
        locals.var_pdeff_dn10 = assign15370_e21249_d_n10;
        locals.var_pdeff_dn11 = assign15370_e21249_d_n11;

        let assign15380_e21268: f64 = if (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p2 == 1.0) || ((p.p2 > 1.0) && (p.p12 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard480 = assign15380_e21268;

        let (assign15390_e21274, assign15390_e21274_d_n3, assign15390_e21274_d_n4, assign15390_e21274_d_n5, assign15390_e21274_d_n6, assign15390_e21274_d_n7, assign15390_e21274_d_n8, assign15390_e21274_d_n9, assign15390_e21274_d_n10, assign15390_e21274_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15390_e21272: f64 = (locals.var_lnew).powf(p.p1111);
        (assign15390_e21272, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign15390_e21274;
        locals.var_t0_dn3 = assign15390_e21274_d_n3;
        locals.var_t0_dn4 = assign15390_e21274_d_n4;
        locals.var_t0_dn5 = assign15390_e21274_d_n5;
        locals.var_t0_dn6 = assign15390_e21274_d_n6;
        locals.var_t0_dn7 = assign15390_e21274_d_n7;
        locals.var_t0_dn8 = assign15390_e21274_d_n8;
        locals.var_t0_dn9 = assign15390_e21274_d_n9;
        locals.var_t0_dn10 = assign15390_e21274_d_n10;
        locals.var_t0_dn11 = assign15390_e21274_d_n11;

        let (assign15400_e21280,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15400_e21278: f64 = (locals.var_wnew + p.p1104);
        (assign15400_e21278,)
    } else {
        (locals.var_w_tmp_stress,)
    }
};
        locals.var_w_tmp_stress = assign15400_e21280;

        let (assign15410_e21286, assign15410_e21286_d_n3, assign15410_e21286_d_n4, assign15410_e21286_d_n5, assign15410_e21286_d_n6, assign15410_e21286_d_n7, assign15410_e21286_d_n8, assign15410_e21286_d_n9, assign15410_e21286_d_n10, assign15410_e21286_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15410_e21284: f64 = (locals.var_w_tmp_stress).powf(p.p1112);
        (assign15410_e21284, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign15410_e21286;
        locals.var_t1_dn3 = assign15410_e21286_d_n3;
        locals.var_t1_dn4 = assign15410_e21286_d_n4;
        locals.var_t1_dn5 = assign15410_e21286_d_n5;
        locals.var_t1_dn6 = assign15410_e21286_d_n6;
        locals.var_t1_dn7 = assign15410_e21286_d_n7;
        locals.var_t1_dn8 = assign15410_e21286_d_n8;
        locals.var_t1_dn9 = assign15410_e21286_d_n9;
        locals.var_t1_dn10 = assign15410_e21286_d_n10;
        locals.var_t1_dn11 = assign15410_e21286_d_n11;

        let (assign15420_e21302, assign15420_e21302_d_n3, assign15420_e21302_d_n4, assign15420_e21302_d_n5, assign15420_e21302_d_n6, assign15420_e21302_d_n7, assign15420_e21302_d_n8, assign15420_e21302_d_n9, assign15420_e21302_d_n10, assign15420_e21302_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15420_e21290: f64 = (p.p1108 / locals.var_t0);
        let assign15420_e21293: f64 = (p.p1109 / locals.var_t1);
        let assign15420_e21294: f64 = (assign15420_e21290 + assign15420_e21293);
        let assign15420_e21298: f64 = (locals.var_t0 * locals.var_t1);
        let assign15420_e21299: f64 = (p.p1110 / assign15420_e21298);
        let assign15420_e21300: f64 = (assign15420_e21294 + assign15420_e21299);
        (assign15420_e21300, (((-((p.p1108 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign15420_e21298 * assign15420_e21298)))),)
    } else {
        (locals.var_tmp1_stress, locals.var_tmp1_stress_dn3, locals.var_tmp1_stress_dn4, locals.var_tmp1_stress_dn5, locals.var_tmp1_stress_dn6, locals.var_tmp1_stress_dn7, locals.var_tmp1_stress_dn8, locals.var_tmp1_stress_dn9, locals.var_tmp1_stress_dn10, locals.var_tmp1_stress_dn11,)
    }
};
        locals.var_tmp1_stress = assign15420_e21302;
        locals.var_tmp1_stress_dn3 = assign15420_e21302_d_n3;
        locals.var_tmp1_stress_dn4 = assign15420_e21302_d_n4;
        locals.var_tmp1_stress_dn5 = assign15420_e21302_d_n5;
        locals.var_tmp1_stress_dn6 = assign15420_e21302_d_n6;
        locals.var_tmp1_stress_dn7 = assign15420_e21302_d_n7;
        locals.var_tmp1_stress_dn8 = assign15420_e21302_d_n8;
        locals.var_tmp1_stress_dn9 = assign15420_e21302_d_n9;
        locals.var_tmp1_stress_dn10 = assign15420_e21302_d_n10;
        locals.var_tmp1_stress_dn11 = assign15420_e21302_d_n11;

        let (assign15430_e21308, assign15430_e21308_d_n3, assign15430_e21308_d_n4, assign15430_e21308_d_n5, assign15430_e21308_d_n6, assign15430_e21308_d_n7, assign15430_e21308_d_n8, assign15430_e21308_d_n9, assign15430_e21308_d_n10, assign15430_e21308_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15430_e21306: f64 = (1.0 + locals.var_tmp1_stress);
        (assign15430_e21306, locals.var_tmp1_stress_dn3, locals.var_tmp1_stress_dn4, locals.var_tmp1_stress_dn5, locals.var_tmp1_stress_dn6, locals.var_tmp1_stress_dn7, locals.var_tmp1_stress_dn8, locals.var_tmp1_stress_dn9, locals.var_tmp1_stress_dn10, locals.var_tmp1_stress_dn11,)
    } else {
        (locals.var_kstress_u0, locals.var_kstress_u0_dn3, locals.var_kstress_u0_dn4, locals.var_kstress_u0_dn5, locals.var_kstress_u0_dn6, locals.var_kstress_u0_dn7, locals.var_kstress_u0_dn8, locals.var_kstress_u0_dn9, locals.var_kstress_u0_dn10, locals.var_kstress_u0_dn11,)
    }
};
        locals.var_kstress_u0 = assign15430_e21308;
        locals.var_kstress_u0_dn3 = assign15430_e21308_d_n3;
        locals.var_kstress_u0_dn4 = assign15430_e21308_d_n4;
        locals.var_kstress_u0_dn5 = assign15430_e21308_d_n5;
        locals.var_kstress_u0_dn6 = assign15430_e21308_d_n6;
        locals.var_kstress_u0_dn7 = assign15430_e21308_d_n7;
        locals.var_kstress_u0_dn8 = assign15430_e21308_d_n8;
        locals.var_kstress_u0_dn9 = assign15430_e21308_d_n9;
        locals.var_kstress_u0_dn10 = assign15430_e21308_d_n10;
        locals.var_kstress_u0_dn11 = assign15430_e21308_d_n11;

        let (assign15440_e21314, assign15440_e21314_d_n3, assign15440_e21314_d_n4, assign15440_e21314_d_n5, assign15440_e21314_d_n6, assign15440_e21314_d_n7, assign15440_e21314_d_n8, assign15440_e21314_d_n9, assign15440_e21314_d_n10, assign15440_e21314_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15440_e21312: f64 = (locals.var_lnew).powf(p.p1117);
        (assign15440_e21312, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign15440_e21314;
        locals.var_t0_dn3 = assign15440_e21314_d_n3;
        locals.var_t0_dn4 = assign15440_e21314_d_n4;
        locals.var_t0_dn5 = assign15440_e21314_d_n5;
        locals.var_t0_dn6 = assign15440_e21314_d_n6;
        locals.var_t0_dn7 = assign15440_e21314_d_n7;
        locals.var_t0_dn8 = assign15440_e21314_d_n8;
        locals.var_t0_dn9 = assign15440_e21314_d_n9;
        locals.var_t0_dn10 = assign15440_e21314_d_n10;
        locals.var_t0_dn11 = assign15440_e21314_d_n11;

        let (assign15450_e21320, assign15450_e21320_d_n3, assign15450_e21320_d_n4, assign15450_e21320_d_n5, assign15450_e21320_d_n6, assign15450_e21320_d_n7, assign15450_e21320_d_n8, assign15450_e21320_d_n9, assign15450_e21320_d_n10, assign15450_e21320_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15450_e21318: f64 = (locals.var_w_tmp_stress).powf(p.p1118);
        (assign15450_e21318, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign15450_e21320;
        locals.var_t1_dn3 = assign15450_e21320_d_n3;
        locals.var_t1_dn4 = assign15450_e21320_d_n4;
        locals.var_t1_dn5 = assign15450_e21320_d_n5;
        locals.var_t1_dn6 = assign15450_e21320_d_n6;
        locals.var_t1_dn7 = assign15450_e21320_d_n7;
        locals.var_t1_dn8 = assign15450_e21320_d_n8;
        locals.var_t1_dn9 = assign15450_e21320_d_n9;
        locals.var_t1_dn10 = assign15450_e21320_d_n10;
        locals.var_t1_dn11 = assign15450_e21320_d_n11;

        let (assign15460_e21336, assign15460_e21336_d_n3, assign15460_e21336_d_n4, assign15460_e21336_d_n5, assign15460_e21336_d_n6, assign15460_e21336_d_n7, assign15460_e21336_d_n8, assign15460_e21336_d_n9, assign15460_e21336_d_n10, assign15460_e21336_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15460_e21324: f64 = (p.p1114 / locals.var_t0);
        let assign15460_e21327: f64 = (p.p1115 / locals.var_t1);
        let assign15460_e21328: f64 = (assign15460_e21324 + assign15460_e21327);
        let assign15460_e21332: f64 = (locals.var_t0 * locals.var_t1);
        let assign15460_e21333: f64 = (p.p1116 / assign15460_e21332);
        let assign15460_e21334: f64 = (assign15460_e21328 + assign15460_e21333);
        (assign15460_e21334, (((-((p.p1114 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign15460_e21332 * assign15460_e21332)))),)
    } else {
        (locals.var_tmp1_stress_vth, locals.var_tmp1_stress_vth_dn3, locals.var_tmp1_stress_vth_dn4, locals.var_tmp1_stress_vth_dn5, locals.var_tmp1_stress_vth_dn6, locals.var_tmp1_stress_vth_dn7, locals.var_tmp1_stress_vth_dn8, locals.var_tmp1_stress_vth_dn9, locals.var_tmp1_stress_vth_dn10, locals.var_tmp1_stress_vth_dn11,)
    }
};
        locals.var_tmp1_stress_vth = assign15460_e21336;
        locals.var_tmp1_stress_vth_dn3 = assign15460_e21336_d_n3;
        locals.var_tmp1_stress_vth_dn4 = assign15460_e21336_d_n4;
        locals.var_tmp1_stress_vth_dn5 = assign15460_e21336_d_n5;
        locals.var_tmp1_stress_vth_dn6 = assign15460_e21336_d_n6;
        locals.var_tmp1_stress_vth_dn7 = assign15460_e21336_d_n7;
        locals.var_tmp1_stress_vth_dn8 = assign15460_e21336_d_n8;
        locals.var_tmp1_stress_vth_dn9 = assign15460_e21336_d_n9;
        locals.var_tmp1_stress_vth_dn10 = assign15460_e21336_d_n10;
        locals.var_tmp1_stress_vth_dn11 = assign15460_e21336_d_n11;

        let (assign15470_e21342, assign15470_e21342_d_n3, assign15470_e21342_d_n4, assign15470_e21342_d_n5, assign15470_e21342_d_n6, assign15470_e21342_d_n7, assign15470_e21342_d_n8, assign15470_e21342_d_n9, assign15470_e21342_d_n10, assign15470_e21342_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15470_e21340: f64 = (1.0 + locals.var_tmp1_stress_vth);
        (assign15470_e21340, locals.var_tmp1_stress_vth_dn3, locals.var_tmp1_stress_vth_dn4, locals.var_tmp1_stress_vth_dn5, locals.var_tmp1_stress_vth_dn6, locals.var_tmp1_stress_vth_dn7, locals.var_tmp1_stress_vth_dn8, locals.var_tmp1_stress_vth_dn9, locals.var_tmp1_stress_vth_dn10, locals.var_tmp1_stress_vth_dn11,)
    } else {
        (locals.var_kstress_vth0, locals.var_kstress_vth0_dn3, locals.var_kstress_vth0_dn4, locals.var_kstress_vth0_dn5, locals.var_kstress_vth0_dn6, locals.var_kstress_vth0_dn7, locals.var_kstress_vth0_dn8, locals.var_kstress_vth0_dn9, locals.var_kstress_vth0_dn10, locals.var_kstress_vth0_dn11,)
    }
};
        locals.var_kstress_vth0 = assign15470_e21342;
        locals.var_kstress_vth0_dn3 = assign15470_e21342_d_n3;
        locals.var_kstress_vth0_dn4 = assign15470_e21342_d_n4;
        locals.var_kstress_vth0_dn5 = assign15470_e21342_d_n5;
        locals.var_kstress_vth0_dn6 = assign15470_e21342_d_n6;
        locals.var_kstress_vth0_dn7 = assign15470_e21342_d_n7;
        locals.var_kstress_vth0_dn8 = assign15470_e21342_d_n8;
        locals.var_kstress_vth0_dn9 = assign15470_e21342_d_n9;
        locals.var_kstress_vth0_dn10 = assign15470_e21342_d_n10;
        locals.var_kstress_vth0_dn11 = assign15470_e21342_d_n11;

        let (assign15480_e21348, assign15480_e21348_d_n3, assign15480_e21348_d_n4, assign15480_e21348_d_n5, assign15480_e21348_d_n6, assign15480_e21348_d_n7, assign15480_e21348_d_n8, assign15480_e21348_d_n9, assign15480_e21348_d_n10, assign15480_e21348_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15480_e21346: f64 = (locals.var_tratio - 1.0);
        (assign15480_e21346, 0.0, locals.var_tratio_dn4, locals.var_tratio_dn5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign15480_e21348;
        locals.var_t0_dn3 = assign15480_e21348_d_n3;
        locals.var_t0_dn4 = assign15480_e21348_d_n4;
        locals.var_t0_dn5 = assign15480_e21348_d_n5;
        locals.var_t0_dn6 = assign15480_e21348_d_n6;
        locals.var_t0_dn7 = assign15480_e21348_d_n7;
        locals.var_t0_dn8 = assign15480_e21348_d_n8;
        locals.var_t0_dn9 = assign15480_e21348_d_n9;
        locals.var_t0_dn10 = assign15480_e21348_d_n10;
        locals.var_t0_dn11 = assign15480_e21348_d_n11;

        let (assign15490_e21360, assign15490_e21360_d_n3, assign15490_e21360_d_n4, assign15490_e21360_d_n5, assign15490_e21360_d_n6, assign15490_e21360_d_n7, assign15490_e21360_d_n8, assign15490_e21360_d_n9, assign15490_e21360_d_n10, assign15490_e21360_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15490_e21354: f64 = (p.p1107 * locals.var_t0);
        let assign15490_e21355: f64 = (1.0 + assign15490_e21354);
        let assign15490_e21356: f64 = (locals.var_kstress_u0 * assign15490_e21355);
        let assign15490_e21358: f64 = (assign15490_e21356 + 1e-9);
        (assign15490_e21358, ((locals.var_kstress_u0_dn3 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn3))), ((locals.var_kstress_u0_dn4 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn4))), ((locals.var_kstress_u0_dn5 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn5))), ((locals.var_kstress_u0_dn6 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn6))), ((locals.var_kstress_u0_dn7 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn7))), ((locals.var_kstress_u0_dn8 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn8))), ((locals.var_kstress_u0_dn9 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn9))), ((locals.var_kstress_u0_dn10 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn10))), ((locals.var_kstress_u0_dn11 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn11))),)
    } else {
        (locals.var_ku0_temp, locals.var_ku0_temp_dn3, locals.var_ku0_temp_dn4, locals.var_ku0_temp_dn5, locals.var_ku0_temp_dn6, locals.var_ku0_temp_dn7, locals.var_ku0_temp_dn8, locals.var_ku0_temp_dn9, locals.var_ku0_temp_dn10, locals.var_ku0_temp_dn11,)
    }
};
        locals.var_ku0_temp = assign15490_e21360;
        locals.var_ku0_temp_dn3 = assign15490_e21360_d_n3;
        locals.var_ku0_temp_dn4 = assign15490_e21360_d_n4;
        locals.var_ku0_temp_dn5 = assign15490_e21360_d_n5;
        locals.var_ku0_temp_dn6 = assign15490_e21360_d_n6;
        locals.var_ku0_temp_dn7 = assign15490_e21360_d_n7;
        locals.var_ku0_temp_dn8 = assign15490_e21360_d_n8;
        locals.var_ku0_temp_dn9 = assign15490_e21360_d_n9;
        locals.var_ku0_temp_dn10 = assign15490_e21360_d_n10;
        locals.var_ku0_temp_dn11 = assign15490_e21360_d_n11;

        let (assign15500_e21364,) = {
    if (locals.var_guard480 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign15500_e21364;

    }

    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign15510_loop_guard: usize = 0;
        while {
            let assign15510_cond_e21369: f64 = if ((locals.var_guard480 != 0.0) && (locals.var_i < p.p2)) { 1.0 } else { 0.0 };
            assign15510_cond_e21369 != 0.0
        } {
            assign15510_loop_guard += 1;
            assert!(assign15510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15510_body0_e21387, assign15510_body0_e21387_d_n3, assign15510_body0_e21387_d_n4, assign15510_body0_e21387_d_n5, assign15510_body0_e21387_d_n6, assign15510_body0_e21387_d_n7, assign15510_body0_e21387_d_n8, assign15510_body0_e21387_d_n9, assign15510_body0_e21387_d_n10, assign15510_body0_e21387_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15510_body0_e21373: f64 = (1.0 / p.p2);
        let assign15510_body0_e21377: f64 = (0.5 * locals.var_l_mult);
        let assign15510_body0_e21378: f64 = (p.p10 + assign15510_body0_e21377);
        let assign15510_body0_e21382: f64 = (p.p12 + locals.var_l_mult);
        let assign15510_body0_e21383: f64 = (locals.var_i * assign15510_body0_e21382);
        let assign15510_body0_e21384: f64 = (assign15510_body0_e21378 + assign15510_body0_e21383);
        let assign15510_body0_e21385: f64 = (assign15510_body0_e21373 / assign15510_body0_e21384);
        (assign15510_body0_e21385, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
            locals.var_t0 = assign15510_body0_e21387;
            locals.var_t0_dn3 = assign15510_body0_e21387_d_n3;
            locals.var_t0_dn4 = assign15510_body0_e21387_d_n4;
            locals.var_t0_dn5 = assign15510_body0_e21387_d_n5;
            locals.var_t0_dn6 = assign15510_body0_e21387_d_n6;
            locals.var_t0_dn7 = assign15510_body0_e21387_d_n7;
            locals.var_t0_dn8 = assign15510_body0_e21387_d_n8;
            locals.var_t0_dn9 = assign15510_body0_e21387_d_n9;
            locals.var_t0_dn10 = assign15510_body0_e21387_d_n10;
            locals.var_t0_dn11 = assign15510_body0_e21387_d_n11;
            let (assign15510_body1_e21405, assign15510_body1_e21405_d_n3, assign15510_body1_e21405_d_n4, assign15510_body1_e21405_d_n5, assign15510_body1_e21405_d_n6, assign15510_body1_e21405_d_n7, assign15510_body1_e21405_d_n8, assign15510_body1_e21405_d_n9, assign15510_body1_e21405_d_n10, assign15510_body1_e21405_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15510_body1_e21391: f64 = (1.0 / p.p2);
        let assign15510_body1_e21395: f64 = (0.5 * locals.var_l_mult);
        let assign15510_body1_e21396: f64 = (p.p11 + assign15510_body1_e21395);
        let assign15510_body1_e21400: f64 = (p.p12 + locals.var_l_mult);
        let assign15510_body1_e21401: f64 = (locals.var_i * assign15510_body1_e21400);
        let assign15510_body1_e21402: f64 = (assign15510_body1_e21396 + assign15510_body1_e21401);
        let assign15510_body1_e21403: f64 = (assign15510_body1_e21391 / assign15510_body1_e21402);
        (assign15510_body1_e21403, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
            locals.var_t1 = assign15510_body1_e21405;
            locals.var_t1_dn3 = assign15510_body1_e21405_d_n3;
            locals.var_t1_dn4 = assign15510_body1_e21405_d_n4;
            locals.var_t1_dn5 = assign15510_body1_e21405_d_n5;
            locals.var_t1_dn6 = assign15510_body1_e21405_d_n6;
            locals.var_t1_dn7 = assign15510_body1_e21405_d_n7;
            locals.var_t1_dn8 = assign15510_body1_e21405_d_n8;
            locals.var_t1_dn9 = assign15510_body1_e21405_d_n9;
            locals.var_t1_dn10 = assign15510_body1_e21405_d_n10;
            locals.var_t1_dn11 = assign15510_body1_e21405_d_n11;
            let (assign15510_body2_e21411, assign15510_body2_e21411_d_n3, assign15510_body2_e21411_d_n4, assign15510_body2_e21411_d_n5, assign15510_body2_e21411_d_n6, assign15510_body2_e21411_d_n7, assign15510_body2_e21411_d_n8, assign15510_body2_e21411_d_n9, assign15510_body2_e21411_d_n10, assign15510_body2_e21411_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15510_body2_e21409: f64 = (locals.var_inv_sa + locals.var_t0);
        (assign15510_body2_e21409, (locals.var_inv_sa_dn3 + locals.var_t0_dn3), (locals.var_inv_sa_dn4 + locals.var_t0_dn4), (locals.var_inv_sa_dn5 + locals.var_t0_dn5), (locals.var_inv_sa_dn6 + locals.var_t0_dn6), (locals.var_inv_sa_dn7 + locals.var_t0_dn7), (locals.var_inv_sa_dn8 + locals.var_t0_dn8), (locals.var_inv_sa_dn9 + locals.var_t0_dn9), (locals.var_inv_sa_dn10 + locals.var_t0_dn10), (locals.var_inv_sa_dn11 + locals.var_t0_dn11),)
    } else {
        (locals.var_inv_sa, locals.var_inv_sa_dn3, locals.var_inv_sa_dn4, locals.var_inv_sa_dn5, locals.var_inv_sa_dn6, locals.var_inv_sa_dn7, locals.var_inv_sa_dn8, locals.var_inv_sa_dn9, locals.var_inv_sa_dn10, locals.var_inv_sa_dn11,)
    }
};
            locals.var_inv_sa = assign15510_body2_e21411;
            locals.var_inv_sa_dn3 = assign15510_body2_e21411_d_n3;
            locals.var_inv_sa_dn4 = assign15510_body2_e21411_d_n4;
            locals.var_inv_sa_dn5 = assign15510_body2_e21411_d_n5;
            locals.var_inv_sa_dn6 = assign15510_body2_e21411_d_n6;
            locals.var_inv_sa_dn7 = assign15510_body2_e21411_d_n7;
            locals.var_inv_sa_dn8 = assign15510_body2_e21411_d_n8;
            locals.var_inv_sa_dn9 = assign15510_body2_e21411_d_n9;
            locals.var_inv_sa_dn10 = assign15510_body2_e21411_d_n10;
            locals.var_inv_sa_dn11 = assign15510_body2_e21411_d_n11;
            let (assign15510_body3_e21417, assign15510_body3_e21417_d_n3, assign15510_body3_e21417_d_n4, assign15510_body3_e21417_d_n5, assign15510_body3_e21417_d_n6, assign15510_body3_e21417_d_n7, assign15510_body3_e21417_d_n8, assign15510_body3_e21417_d_n9, assign15510_body3_e21417_d_n10, assign15510_body3_e21417_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15510_body3_e21415: f64 = (locals.var_inv_sb + locals.var_t1);
        (assign15510_body3_e21415, (locals.var_inv_sb_dn3 + locals.var_t1_dn3), (locals.var_inv_sb_dn4 + locals.var_t1_dn4), (locals.var_inv_sb_dn5 + locals.var_t1_dn5), (locals.var_inv_sb_dn6 + locals.var_t1_dn6), (locals.var_inv_sb_dn7 + locals.var_t1_dn7), (locals.var_inv_sb_dn8 + locals.var_t1_dn8), (locals.var_inv_sb_dn9 + locals.var_t1_dn9), (locals.var_inv_sb_dn10 + locals.var_t1_dn10), (locals.var_inv_sb_dn11 + locals.var_t1_dn11),)
    } else {
        (locals.var_inv_sb, locals.var_inv_sb_dn3, locals.var_inv_sb_dn4, locals.var_inv_sb_dn5, locals.var_inv_sb_dn6, locals.var_inv_sb_dn7, locals.var_inv_sb_dn8, locals.var_inv_sb_dn9, locals.var_inv_sb_dn10, locals.var_inv_sb_dn11,)
    }
};
            locals.var_inv_sb = assign15510_body3_e21417;
            locals.var_inv_sb_dn3 = assign15510_body3_e21417_d_n3;
            locals.var_inv_sb_dn4 = assign15510_body3_e21417_d_n4;
            locals.var_inv_sb_dn5 = assign15510_body3_e21417_d_n5;
            locals.var_inv_sb_dn6 = assign15510_body3_e21417_d_n6;
            locals.var_inv_sb_dn7 = assign15510_body3_e21417_d_n7;
            locals.var_inv_sb_dn8 = assign15510_body3_e21417_d_n8;
            locals.var_inv_sb_dn9 = assign15510_body3_e21417_d_n9;
            locals.var_inv_sb_dn10 = assign15510_body3_e21417_d_n10;
            locals.var_inv_sb_dn11 = assign15510_body3_e21417_d_n11;
            let (assign15510_body4_e21423,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15510_body4_e21421: f64 = (locals.var_i + 1.0);
        (assign15510_body4_e21421,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign15510_body4_e21423;
        }

        let (assign15520_e21433,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15520_e21429: f64 = (0.5 * locals.var_l_mult);
        let assign15520_e21430: f64 = (p.p1102 + assign15520_e21429);
        let assign15520_e21431: f64 = (1.0 / assign15520_e21430);
        (assign15520_e21431,)
    } else {
        (locals.var_inv_saref,)
    }
};
        locals.var_inv_saref = assign15520_e21433;

        let (assign15530_e21443,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15530_e21439: f64 = (0.5 * locals.var_l_mult);
        let assign15530_e21440: f64 = (p.p1103 + assign15530_e21439);
        let assign15530_e21441: f64 = (1.0 / assign15530_e21440);
        (assign15530_e21441,)
    } else {
        (locals.var_inv_sbref,)
    }
};
        locals.var_inv_sbref = assign15530_e21443;

        let (assign15540_e21449,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15540_e21447: f64 = (locals.var_inv_saref + locals.var_inv_sbref);
        (assign15540_e21447,)
    } else {
        (locals.var_inv_odref,)
    }
};
        locals.var_inv_odref = assign15540_e21449;

        let (assign15550_e21457, assign15550_e21457_d_n3, assign15550_e21457_d_n4, assign15550_e21457_d_n5, assign15550_e21457_d_n6, assign15550_e21457_d_n7, assign15550_e21457_d_n8, assign15550_e21457_d_n9, assign15550_e21457_d_n10, assign15550_e21457_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15550_e21453: f64 = (p.p1105 / locals.var_ku0_temp);
        let assign15550_e21455: f64 = (assign15550_e21453 * locals.var_inv_odref);
        (assign15550_e21455, ((-((p.p1105 * locals.var_ku0_temp_dn3) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn4) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn5) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn6) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn7) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn8) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn9) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn10) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn11) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref),)
    } else {
        (locals.var_rho_ref, locals.var_rho_ref_dn3, locals.var_rho_ref_dn4, locals.var_rho_ref_dn5, locals.var_rho_ref_dn6, locals.var_rho_ref_dn7, locals.var_rho_ref_dn8, locals.var_rho_ref_dn9, locals.var_rho_ref_dn10, locals.var_rho_ref_dn11,)
    }
};
        locals.var_rho_ref = assign15550_e21457;
        locals.var_rho_ref_dn3 = assign15550_e21457_d_n3;
        locals.var_rho_ref_dn4 = assign15550_e21457_d_n4;
        locals.var_rho_ref_dn5 = assign15550_e21457_d_n5;
        locals.var_rho_ref_dn6 = assign15550_e21457_d_n6;
        locals.var_rho_ref_dn7 = assign15550_e21457_d_n7;
        locals.var_rho_ref_dn8 = assign15550_e21457_d_n8;
        locals.var_rho_ref_dn9 = assign15550_e21457_d_n9;
        locals.var_rho_ref_dn10 = assign15550_e21457_d_n10;
        locals.var_rho_ref_dn11 = assign15550_e21457_d_n11;

        let (assign15560_e21463, assign15560_e21463_d_n3, assign15560_e21463_d_n4, assign15560_e21463_d_n5, assign15560_e21463_d_n6, assign15560_e21463_d_n7, assign15560_e21463_d_n8, assign15560_e21463_d_n9, assign15560_e21463_d_n10, assign15560_e21463_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15560_e21461: f64 = (locals.var_inv_sa + locals.var_inv_sb);
        (assign15560_e21461, (locals.var_inv_sa_dn3 + locals.var_inv_sb_dn3), (locals.var_inv_sa_dn4 + locals.var_inv_sb_dn4), (locals.var_inv_sa_dn5 + locals.var_inv_sb_dn5), (locals.var_inv_sa_dn6 + locals.var_inv_sb_dn6), (locals.var_inv_sa_dn7 + locals.var_inv_sb_dn7), (locals.var_inv_sa_dn8 + locals.var_inv_sb_dn8), (locals.var_inv_sa_dn9 + locals.var_inv_sb_dn9), (locals.var_inv_sa_dn10 + locals.var_inv_sb_dn10), (locals.var_inv_sa_dn11 + locals.var_inv_sb_dn11),)
    } else {
        (locals.var_inv_od, locals.var_inv_od_dn3, locals.var_inv_od_dn4, locals.var_inv_od_dn5, locals.var_inv_od_dn6, locals.var_inv_od_dn7, locals.var_inv_od_dn8, locals.var_inv_od_dn9, locals.var_inv_od_dn10, locals.var_inv_od_dn11,)
    }
};
        locals.var_inv_od = assign15560_e21463;
        locals.var_inv_od_dn3 = assign15560_e21463_d_n3;
        locals.var_inv_od_dn4 = assign15560_e21463_d_n4;
        locals.var_inv_od_dn5 = assign15560_e21463_d_n5;
        locals.var_inv_od_dn6 = assign15560_e21463_d_n6;
        locals.var_inv_od_dn7 = assign15560_e21463_d_n7;
        locals.var_inv_od_dn8 = assign15560_e21463_d_n8;
        locals.var_inv_od_dn9 = assign15560_e21463_d_n9;
        locals.var_inv_od_dn10 = assign15560_e21463_d_n10;
        locals.var_inv_od_dn11 = assign15560_e21463_d_n11;

        let (assign15570_e21471, assign15570_e21471_d_n3, assign15570_e21471_d_n4, assign15570_e21471_d_n5, assign15570_e21471_d_n6, assign15570_e21471_d_n7, assign15570_e21471_d_n8, assign15570_e21471_d_n9, assign15570_e21471_d_n10, assign15570_e21471_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15570_e21467: f64 = (p.p1105 / locals.var_ku0_temp);
        let assign15570_e21469: f64 = (assign15570_e21467 * locals.var_inv_od);
        (assign15570_e21469, (((-((p.p1105 * locals.var_ku0_temp_dn3) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn3)), (((-((p.p1105 * locals.var_ku0_temp_dn4) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn4)), (((-((p.p1105 * locals.var_ku0_temp_dn5) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn5)), (((-((p.p1105 * locals.var_ku0_temp_dn6) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn6)), (((-((p.p1105 * locals.var_ku0_temp_dn7) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn7)), (((-((p.p1105 * locals.var_ku0_temp_dn8) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn8)), (((-((p.p1105 * locals.var_ku0_temp_dn9) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn9)), (((-((p.p1105 * locals.var_ku0_temp_dn10) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn10)), (((-((p.p1105 * locals.var_ku0_temp_dn11) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn11)),)
    } else {
        (locals.var_rho, locals.var_rho_dn3, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn11,)
    }
};
        locals.var_rho = assign15570_e21471;
        locals.var_rho_dn3 = assign15570_e21471_d_n3;
        locals.var_rho_dn4 = assign15570_e21471_d_n4;
        locals.var_rho_dn5 = assign15570_e21471_d_n5;
        locals.var_rho_dn6 = assign15570_e21471_d_n6;
        locals.var_rho_dn7 = assign15570_e21471_d_n7;
        locals.var_rho_dn8 = assign15570_e21471_d_n8;
        locals.var_rho_dn9 = assign15570_e21471_d_n9;
        locals.var_rho_dn10 = assign15570_e21471_d_n10;
        locals.var_rho_dn11 = assign15570_e21471_d_n11;

        let (assign15580_e21481, assign15580_e21481_d_n3, assign15580_e21481_d_n4, assign15580_e21481_d_n5, assign15580_e21481_d_n6, assign15580_e21481_d_n7, assign15580_e21481_d_n8, assign15580_e21481_d_n9, assign15580_e21481_d_n10, assign15580_e21481_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15580_e21475: f64 = (1.0 + locals.var_rho);
        let assign15580_e21478: f64 = (1.0 + locals.var_rho_ref);
        let assign15580_e21479: f64 = (assign15580_e21475 / assign15580_e21478);
        (assign15580_e21479, (((locals.var_rho_dn3 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn3)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn4 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn4)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn5 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn5)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn6 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn6)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn7 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn7)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn8 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn8)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn9 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn9)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn10 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn10)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn11 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn11)) / (assign15580_e21478 * assign15580_e21478)),)
    } else {
        (locals.var_mu0_mult, locals.var_mu0_mult_dn3, locals.var_mu0_mult_dn4, locals.var_mu0_mult_dn5, locals.var_mu0_mult_dn6, locals.var_mu0_mult_dn7, locals.var_mu0_mult_dn8, locals.var_mu0_mult_dn9, locals.var_mu0_mult_dn10, locals.var_mu0_mult_dn11,)
    }
};
        locals.var_mu0_mult = assign15580_e21481;
        locals.var_mu0_mult_dn3 = assign15580_e21481_d_n3;
        locals.var_mu0_mult_dn4 = assign15580_e21481_d_n4;
        locals.var_mu0_mult_dn5 = assign15580_e21481_d_n5;
        locals.var_mu0_mult_dn6 = assign15580_e21481_d_n6;
        locals.var_mu0_mult_dn7 = assign15580_e21481_d_n7;
        locals.var_mu0_mult_dn8 = assign15580_e21481_d_n8;
        locals.var_mu0_mult_dn9 = assign15580_e21481_d_n9;
        locals.var_mu0_mult_dn10 = assign15580_e21481_d_n10;
        locals.var_mu0_mult_dn11 = assign15580_e21481_d_n11;

        let (assign15590_e21495, assign15590_e21495_d_n3, assign15590_e21495_d_n4, assign15590_e21495_d_n5, assign15590_e21495_d_n6, assign15590_e21495_d_n7, assign15590_e21495_d_n8, assign15590_e21495_d_n9, assign15590_e21495_d_n10, assign15590_e21495_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15590_e21486: f64 = (locals.var_rho * p.p1106);
        let assign15590_e21487: f64 = (1.0 + assign15590_e21486);
        let assign15590_e21491: f64 = (locals.var_rho_ref * p.p1106);
        let assign15590_e21492: f64 = (1.0 + assign15590_e21491);
        let assign15590_e21493: f64 = (assign15590_e21487 / assign15590_e21492);
        (assign15590_e21493, ((((locals.var_rho_dn3 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn3 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn4 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn4 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn5 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn5 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn6 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn6 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn7 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn7 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn8 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn8 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn9 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn9 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn10 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn10 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn11 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn11 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)),)
    } else {
        (locals.var_vsat_mult, locals.var_vsat_mult_dn3, locals.var_vsat_mult_dn4, locals.var_vsat_mult_dn5, locals.var_vsat_mult_dn6, locals.var_vsat_mult_dn7, locals.var_vsat_mult_dn8, locals.var_vsat_mult_dn9, locals.var_vsat_mult_dn10, locals.var_vsat_mult_dn11,)
    }
};
        locals.var_vsat_mult = assign15590_e21495;
        locals.var_vsat_mult_dn3 = assign15590_e21495_d_n3;
        locals.var_vsat_mult_dn4 = assign15590_e21495_d_n4;
        locals.var_vsat_mult_dn5 = assign15590_e21495_d_n5;
        locals.var_vsat_mult_dn6 = assign15590_e21495_d_n6;
        locals.var_vsat_mult_dn7 = assign15590_e21495_d_n7;
        locals.var_vsat_mult_dn8 = assign15590_e21495_d_n8;
        locals.var_vsat_mult_dn9 = assign15590_e21495_d_n9;
        locals.var_vsat_mult_dn10 = assign15590_e21495_d_n10;
        locals.var_vsat_mult_dn11 = assign15590_e21495_d_n11;

        let (assign15600_e21505, assign15600_e21505_d_n3, assign15600_e21505_d_n4, assign15600_e21505_d_n5, assign15600_e21505_d_n6, assign15600_e21505_d_n7, assign15600_e21505_d_n8, assign15600_e21505_d_n9, assign15600_e21505_d_n10, assign15600_e21505_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15600_e21499: f64 = (p.p1113 / locals.var_kstress_vth0);
        let assign15600_e21502: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15600_e21503: f64 = (assign15600_e21499 * assign15600_e21502);
        (assign15600_e21503, (((-((p.p1113 * locals.var_kstress_vth0_dn3) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn3)), (((-((p.p1113 * locals.var_kstress_vth0_dn4) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn4)), (((-((p.p1113 * locals.var_kstress_vth0_dn5) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn5)), (((-((p.p1113 * locals.var_kstress_vth0_dn6) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn6)), (((-((p.p1113 * locals.var_kstress_vth0_dn7) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn7)), (((-((p.p1113 * locals.var_kstress_vth0_dn8) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn8)), (((-((p.p1113 * locals.var_kstress_vth0_dn9) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn9)), (((-((p.p1113 * locals.var_kstress_vth0_dn10) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn10)), (((-((p.p1113 * locals.var_kstress_vth0_dn11) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn11)),)
    } else {
        (locals.var_vth0_stress, locals.var_vth0_stress_dn3, locals.var_vth0_stress_dn4, locals.var_vth0_stress_dn5, locals.var_vth0_stress_dn6, locals.var_vth0_stress_dn7, locals.var_vth0_stress_dn8, locals.var_vth0_stress_dn9, locals.var_vth0_stress_dn10, locals.var_vth0_stress_dn11,)
    }
};
        locals.var_vth0_stress = assign15600_e21505;
        locals.var_vth0_stress_dn3 = assign15600_e21505_d_n3;
        locals.var_vth0_stress_dn4 = assign15600_e21505_d_n4;
        locals.var_vth0_stress_dn5 = assign15600_e21505_d_n5;
        locals.var_vth0_stress_dn6 = assign15600_e21505_d_n6;
        locals.var_vth0_stress_dn7 = assign15600_e21505_d_n7;
        locals.var_vth0_stress_dn8 = assign15600_e21505_d_n8;
        locals.var_vth0_stress_dn9 = assign15600_e21505_d_n9;
        locals.var_vth0_stress_dn10 = assign15600_e21505_d_n10;
        locals.var_vth0_stress_dn11 = assign15600_e21505_d_n11;

        let (assign15610_e21517, assign15610_e21517_d_n3, assign15610_e21517_d_n4, assign15610_e21517_d_n5, assign15610_e21517_d_n6, assign15610_e21517_d_n7, assign15610_e21517_d_n8, assign15610_e21517_d_n9, assign15610_e21517_d_n10, assign15610_e21517_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15610_e21510: f64 = (locals.var_kstress_vth0).powf(p.p1120);
        let assign15610_e21511: f64 = (p.p1119 / assign15610_e21510);
        let assign15610_e21514: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15610_e21515: f64 = (assign15610_e21511 * assign15610_e21514);
        (assign15610_e21515, (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn3)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn4)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn5)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn6)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn7)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn8)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn9)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn10)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn11)),)
    } else {
        (locals.var_k2_stress, locals.var_k2_stress_dn3, locals.var_k2_stress_dn4, locals.var_k2_stress_dn5, locals.var_k2_stress_dn6, locals.var_k2_stress_dn7, locals.var_k2_stress_dn8, locals.var_k2_stress_dn9, locals.var_k2_stress_dn10, locals.var_k2_stress_dn11,)
    }
};
        locals.var_k2_stress = assign15610_e21517;
        locals.var_k2_stress_dn3 = assign15610_e21517_d_n3;
        locals.var_k2_stress_dn4 = assign15610_e21517_d_n4;
        locals.var_k2_stress_dn5 = assign15610_e21517_d_n5;
        locals.var_k2_stress_dn6 = assign15610_e21517_d_n6;
        locals.var_k2_stress_dn7 = assign15610_e21517_d_n7;
        locals.var_k2_stress_dn8 = assign15610_e21517_d_n8;
        locals.var_k2_stress_dn9 = assign15610_e21517_d_n9;
        locals.var_k2_stress_dn10 = assign15610_e21517_d_n10;
        locals.var_k2_stress_dn11 = assign15610_e21517_d_n11;

        let (assign15620_e21529, assign15620_e21529_d_n3, assign15620_e21529_d_n4, assign15620_e21529_d_n5, assign15620_e21529_d_n6, assign15620_e21529_d_n7, assign15620_e21529_d_n8, assign15620_e21529_d_n9, assign15620_e21529_d_n10, assign15620_e21529_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15620_e21522: f64 = (locals.var_kstress_vth0).powf(p.p1122);
        let assign15620_e21523: f64 = (p.p1121 / assign15620_e21522);
        let assign15620_e21526: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15620_e21527: f64 = (assign15620_e21523 * assign15620_e21526);
        (assign15620_e21527, (((-((p.p1121 * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15620_e21522 * (p.p1122 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15620_e21522 * assign15620_e21522))) * assign15620_e21526) + (assign15620_e21523 * locals.var_inv_od_dn3)), (((-((p.p1121 * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15620_e21522 * (p.p1122 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15620_e21522 * assign15620_e21522))) * assign15620_e21526) + (assign15620_e21523 * locals.var_inv_od_dn4)), (((-((p.p1121 * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15620_e21522 * (p.p1122 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15620_e21522 * assign15620_e21522))) * assign15620_e21526) + (assign15620_e21523 * locals.var_inv_od_dn5)), (((-((p.p1121 * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15620_e21522 * (p.p1122 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15620_e21522 * assign15620_e21522))) * assign15620_e21526) + (assign15620_e21523 * locals.var_inv_od_dn6)), (((-((p.p1121 * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15620_e21522 * (p.p1122 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15620_e21522 * assign15620_e21522))) * assign15620_e21526) + (assign15620_e21523 * locals.var_inv_od_dn7)), (((-((p.p1121 * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15620_e21522 * (p.p1122 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15620_e21522 * assign15620_e21522))) * assign15620_e21526) + (assign15620_e21523 * locals.var_inv_od_dn8)), (((-((p.p1121 * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15620_e21522 * (p.p1122 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15620_e21522 * assign15620_e21522))) * assign15620_e21526) + (assign15620_e21523 * locals.var_inv_od_dn9)), (((-((p.p1121 * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15620_e21522 * (p.p1122 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15620_e21522 * assign15620_e21522))) * assign15620_e21526) + (assign15620_e21523 * locals.var_inv_od_dn10)), (((-((p.p1121 * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15620_e21522 * (p.p1122 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15620_e21522 * assign15620_e21522))) * assign15620_e21526) + (assign15620_e21523 * locals.var_inv_od_dn11)),)
    } else {
        (locals.var_eta_stress, locals.var_eta_stress_dn3, locals.var_eta_stress_dn4, locals.var_eta_stress_dn5, locals.var_eta_stress_dn6, locals.var_eta_stress_dn7, locals.var_eta_stress_dn8, locals.var_eta_stress_dn9, locals.var_eta_stress_dn10, locals.var_eta_stress_dn11,)
    }
};
        locals.var_eta_stress = assign15620_e21529;
        locals.var_eta_stress_dn3 = assign15620_e21529_d_n3;
        locals.var_eta_stress_dn4 = assign15620_e21529_d_n4;
        locals.var_eta_stress_dn5 = assign15620_e21529_d_n5;
        locals.var_eta_stress_dn6 = assign15620_e21529_d_n6;
        locals.var_eta_stress_dn7 = assign15620_e21529_d_n7;
        locals.var_eta_stress_dn8 = assign15620_e21529_d_n8;
        locals.var_eta_stress_dn9 = assign15620_e21529_d_n9;
        locals.var_eta_stress_dn10 = assign15620_e21529_d_n10;
        locals.var_eta_stress_dn11 = assign15620_e21529_d_n11;

        let (assign15630_e21535, assign15630_e21535_d_n3, assign15630_e21535_d_n4, assign15630_e21535_d_n5, assign15630_e21535_d_n6, assign15630_e21535_d_n7, assign15630_e21535_d_n8, assign15630_e21535_d_n9, assign15630_e21535_d_n10, assign15630_e21535_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15630_e21533: f64 = (locals.var_u0_t * locals.var_mu0_mult);
        (assign15630_e21533, ((locals.var_u0_t_dn3 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn3)), ((locals.var_u0_t_dn4 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn4)), ((locals.var_u0_t_dn5 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn5)), ((locals.var_u0_t_dn6 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn6)), ((locals.var_u0_t_dn7 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn7)), ((locals.var_u0_t_dn8 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn8)), ((locals.var_u0_t_dn9 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn9)), ((locals.var_u0_t_dn10 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn10)), ((locals.var_u0_t_dn11 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn11)),)
    } else {
        (locals.var_u0_t, locals.var_u0_t_dn3, locals.var_u0_t_dn4, locals.var_u0_t_dn5, locals.var_u0_t_dn6, locals.var_u0_t_dn7, locals.var_u0_t_dn8, locals.var_u0_t_dn9, locals.var_u0_t_dn10, locals.var_u0_t_dn11,)
    }
};
        locals.var_u0_t = assign15630_e21535;
        locals.var_u0_t_dn3 = assign15630_e21535_d_n3;
        locals.var_u0_t_dn4 = assign15630_e21535_d_n4;
        locals.var_u0_t_dn5 = assign15630_e21535_d_n5;
        locals.var_u0_t_dn6 = assign15630_e21535_d_n6;
        locals.var_u0_t_dn7 = assign15630_e21535_d_n7;
        locals.var_u0_t_dn8 = assign15630_e21535_d_n8;
        locals.var_u0_t_dn9 = assign15630_e21535_d_n9;
        locals.var_u0_t_dn10 = assign15630_e21535_d_n10;
        locals.var_u0_t_dn11 = assign15630_e21535_d_n11;

        let (assign15640_e21541, assign15640_e21541_d_n3, assign15640_e21541_d_n4, assign15640_e21541_d_n5, assign15640_e21541_d_n6, assign15640_e21541_d_n7, assign15640_e21541_d_n8, assign15640_e21541_d_n9, assign15640_e21541_d_n10, assign15640_e21541_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15640_e21539: f64 = (locals.var_vsat_t * locals.var_vsat_mult);
        (assign15640_e21539, ((locals.var_vsat_t_dn3 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn3)), ((locals.var_vsat_t_dn4 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn4)), ((locals.var_vsat_t_dn5 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn5)), ((locals.var_vsat_t_dn6 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn6)), ((locals.var_vsat_t_dn7 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn7)), ((locals.var_vsat_t_dn8 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn8)), ((locals.var_vsat_t_dn9 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn9)), ((locals.var_vsat_t_dn10 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn10)), ((locals.var_vsat_t_dn11 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn11)),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11,)
    }
};
        locals.var_vsat_t = assign15640_e21541;
        locals.var_vsat_t_dn3 = assign15640_e21541_d_n3;
        locals.var_vsat_t_dn4 = assign15640_e21541_d_n4;
        locals.var_vsat_t_dn5 = assign15640_e21541_d_n5;
        locals.var_vsat_t_dn6 = assign15640_e21541_d_n6;
        locals.var_vsat_t_dn7 = assign15640_e21541_d_n7;
        locals.var_vsat_t_dn8 = assign15640_e21541_d_n8;
        locals.var_vsat_t_dn9 = assign15640_e21541_d_n9;
        locals.var_vsat_t_dn10 = assign15640_e21541_d_n10;
        locals.var_vsat_t_dn11 = assign15640_e21541_d_n11;

        let (assign15650_e21547, assign15650_e21547_d_n3, assign15650_e21547_d_n4, assign15650_e21547_d_n5, assign15650_e21547_d_n6, assign15650_e21547_d_n7, assign15650_e21547_d_n8, assign15650_e21547_d_n9, assign15650_e21547_d_n10, assign15650_e21547_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15650_e21545: f64 = (locals.var_k2_i + locals.var_k2_stress);
        (assign15650_e21545, (locals.var_k2_i_dn3 + locals.var_k2_stress_dn3), (locals.var_k2_i_dn4 + locals.var_k2_stress_dn4), (locals.var_k2_i_dn5 + locals.var_k2_stress_dn5), (locals.var_k2_i_dn6 + locals.var_k2_stress_dn6), (locals.var_k2_i_dn7 + locals.var_k2_stress_dn7), (locals.var_k2_i_dn8 + locals.var_k2_stress_dn8), (locals.var_k2_i_dn9 + locals.var_k2_stress_dn9), (locals.var_k2_i_dn10 + locals.var_k2_stress_dn10), (locals.var_k2_i_dn11 + locals.var_k2_stress_dn11),)
    } else {
        (locals.var_k2_i, locals.var_k2_i_dn3, locals.var_k2_i_dn4, locals.var_k2_i_dn5, locals.var_k2_i_dn6, locals.var_k2_i_dn7, locals.var_k2_i_dn8, locals.var_k2_i_dn9, locals.var_k2_i_dn10, locals.var_k2_i_dn11,)
    }
};
        locals.var_k2_i = assign15650_e21547;
        locals.var_k2_i_dn3 = assign15650_e21547_d_n3;
        locals.var_k2_i_dn4 = assign15650_e21547_d_n4;
        locals.var_k2_i_dn5 = assign15650_e21547_d_n5;
        locals.var_k2_i_dn6 = assign15650_e21547_d_n6;
        locals.var_k2_i_dn7 = assign15650_e21547_d_n7;
        locals.var_k2_i_dn8 = assign15650_e21547_d_n8;
        locals.var_k2_i_dn9 = assign15650_e21547_d_n9;
        locals.var_k2_i_dn10 = assign15650_e21547_d_n10;
        locals.var_k2_i_dn11 = assign15650_e21547_d_n11;

        let (assign15660_e21553, assign15660_e21553_d_n3, assign15660_e21553_d_n4, assign15660_e21553_d_n5, assign15660_e21553_d_n6, assign15660_e21553_d_n7, assign15660_e21553_d_n8, assign15660_e21553_d_n9, assign15660_e21553_d_n10, assign15660_e21553_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15660_e21551: f64 = (locals.var_eta0_t + locals.var_eta_stress);
        (assign15660_e21551, (locals.var_eta0_t_dn3 + locals.var_eta_stress_dn3), (locals.var_eta0_t_dn4 + locals.var_eta_stress_dn4), (locals.var_eta0_t_dn5 + locals.var_eta_stress_dn5), (locals.var_eta0_t_dn6 + locals.var_eta_stress_dn6), (locals.var_eta0_t_dn7 + locals.var_eta_stress_dn7), (locals.var_eta0_t_dn8 + locals.var_eta_stress_dn8), (locals.var_eta0_t_dn9 + locals.var_eta_stress_dn9), (locals.var_eta0_t_dn10 + locals.var_eta_stress_dn10), (locals.var_eta0_t_dn11 + locals.var_eta_stress_dn11),)
    } else {
        (locals.var_eta0_t, locals.var_eta0_t_dn3, locals.var_eta0_t_dn4, locals.var_eta0_t_dn5, locals.var_eta0_t_dn6, locals.var_eta0_t_dn7, locals.var_eta0_t_dn8, locals.var_eta0_t_dn9, locals.var_eta0_t_dn10, locals.var_eta0_t_dn11,)
    }
};
        locals.var_eta0_t = assign15660_e21553;
        locals.var_eta0_t_dn3 = assign15660_e21553_d_n3;
        locals.var_eta0_t_dn4 = assign15660_e21553_d_n4;
        locals.var_eta0_t_dn5 = assign15660_e21553_d_n5;
        locals.var_eta0_t_dn6 = assign15660_e21553_d_n6;
        locals.var_eta0_t_dn7 = assign15660_e21553_d_n7;
        locals.var_eta0_t_dn8 = assign15660_e21553_d_n8;
        locals.var_eta0_t_dn9 = assign15660_e21553_d_n9;
        locals.var_eta0_t_dn10 = assign15660_e21553_d_n10;
        locals.var_eta0_t_dn11 = assign15660_e21553_d_n11;

        let assign15670_e21556: f64 = if p.p27 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard481 = assign15670_e21556;

        let (assign15680_e21568, assign15680_e21568_d_n3, assign15680_e21568_d_n4, assign15680_e21568_d_n5, assign15680_e21568_d_n6, assign15680_e21568_d_n7, assign15680_e21568_d_n8, assign15680_e21568_d_n9, assign15680_e21568_d_n10, assign15680_e21568_d_n11,) = {
    if ((locals.var_guard480 != 0.0) && (locals.var_guard481 != 0.0)) {
        let assign15680_e21562: f64 = (locals.var_kvth0edge_i / locals.var_kstress_vth0);
        let assign15680_e21565: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15680_e21566: f64 = (assign15680_e21562 * assign15680_e21565);
        (assign15680_e21566, (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn3) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15680_e21565) + (assign15680_e21562 * locals.var_inv_od_dn3)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn4) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15680_e21565) + (assign15680_e21562 * locals.var_inv_od_dn4)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn5) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15680_e21565) + (assign15680_e21562 * locals.var_inv_od_dn5)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn6) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15680_e21565) + (assign15680_e21562 * locals.var_inv_od_dn6)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn7) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15680_e21565) + (assign15680_e21562 * locals.var_inv_od_dn7)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn8) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15680_e21565) + (assign15680_e21562 * locals.var_inv_od_dn8)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn9) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15680_e21565) + (assign15680_e21562 * locals.var_inv_od_dn9)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn10) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15680_e21565) + (assign15680_e21562 * locals.var_inv_od_dn10)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn11) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15680_e21565) + (assign15680_e21562 * locals.var_inv_od_dn11)),)
    } else {
        (locals.var_vth0_stress_edge, locals.var_vth0_stress_edge_dn3, locals.var_vth0_stress_edge_dn4, locals.var_vth0_stress_edge_dn5, locals.var_vth0_stress_edge_dn6, locals.var_vth0_stress_edge_dn7, locals.var_vth0_stress_edge_dn8, locals.var_vth0_stress_edge_dn9, locals.var_vth0_stress_edge_dn10, locals.var_vth0_stress_edge_dn11,)
    }
};
        locals.var_vth0_stress_edge = assign15680_e21568;
        locals.var_vth0_stress_edge_dn3 = assign15680_e21568_d_n3;
        locals.var_vth0_stress_edge_dn4 = assign15680_e21568_d_n4;
        locals.var_vth0_stress_edge_dn5 = assign15680_e21568_d_n5;
        locals.var_vth0_stress_edge_dn6 = assign15680_e21568_d_n6;
        locals.var_vth0_stress_edge_dn7 = assign15680_e21568_d_n7;
        locals.var_vth0_stress_edge_dn8 = assign15680_e21568_d_n8;
        locals.var_vth0_stress_edge_dn9 = assign15680_e21568_d_n9;
        locals.var_vth0_stress_edge_dn10 = assign15680_e21568_d_n10;
        locals.var_vth0_stress_edge_dn11 = assign15680_e21568_d_n11;

        let (assign15690_e21582, assign15690_e21582_d_n3, assign15690_e21582_d_n4, assign15690_e21582_d_n5, assign15690_e21582_d_n6, assign15690_e21582_d_n7, assign15690_e21582_d_n8, assign15690_e21582_d_n9, assign15690_e21582_d_n10, assign15690_e21582_d_n11,) = {
    if ((locals.var_guard480 != 0.0) && (locals.var_guard481 != 0.0)) {
        let assign15690_e21575: f64 = (locals.var_kstress_vth0).powf(p.p1120);
        let assign15690_e21576: f64 = (locals.var_stk2edge_i / assign15690_e21575);
        let assign15690_e21579: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15690_e21580: f64 = (assign15690_e21576 * assign15690_e21579);
        (assign15690_e21580, (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15690_e21575 * (p.p1120 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15690_e21575 * assign15690_e21575))) * assign15690_e21579) + (assign15690_e21576 * locals.var_inv_od_dn3)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15690_e21575 * (p.p1120 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15690_e21575 * assign15690_e21575))) * assign15690_e21579) + (assign15690_e21576 * locals.var_inv_od_dn4)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15690_e21575 * (p.p1120 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15690_e21575 * assign15690_e21575))) * assign15690_e21579) + (assign15690_e21576 * locals.var_inv_od_dn5)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15690_e21575 * (p.p1120 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15690_e21575 * assign15690_e21575))) * assign15690_e21579) + (assign15690_e21576 * locals.var_inv_od_dn6)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15690_e21575 * (p.p1120 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15690_e21575 * assign15690_e21575))) * assign15690_e21579) + (assign15690_e21576 * locals.var_inv_od_dn7)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15690_e21575 * (p.p1120 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15690_e21575 * assign15690_e21575))) * assign15690_e21579) + (assign15690_e21576 * locals.var_inv_od_dn8)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15690_e21575 * (p.p1120 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15690_e21575 * assign15690_e21575))) * assign15690_e21579) + (assign15690_e21576 * locals.var_inv_od_dn9)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15690_e21575 * (p.p1120 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15690_e21575 * assign15690_e21575))) * assign15690_e21579) + (assign15690_e21576 * locals.var_inv_od_dn10)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15690_e21575 * (p.p1120 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15690_e21575 * assign15690_e21575))) * assign15690_e21579) + (assign15690_e21576 * locals.var_inv_od_dn11)),)
    } else {
        (locals.var_k2_stress_edge, locals.var_k2_stress_edge_dn3, locals.var_k2_stress_edge_dn4, locals.var_k2_stress_edge_dn5, locals.var_k2_stress_edge_dn6, locals.var_k2_stress_edge_dn7, locals.var_k2_stress_edge_dn8, locals.var_k2_stress_edge_dn9, locals.var_k2_stress_edge_dn10, locals.var_k2_stress_edge_dn11,)
    }
};
        locals.var_k2_stress_edge = assign15690_e21582;
        locals.var_k2_stress_edge_dn3 = assign15690_e21582_d_n3;
        locals.var_k2_stress_edge_dn4 = assign15690_e21582_d_n4;
        locals.var_k2_stress_edge_dn5 = assign15690_e21582_d_n5;
        locals.var_k2_stress_edge_dn6 = assign15690_e21582_d_n6;
        locals.var_k2_stress_edge_dn7 = assign15690_e21582_d_n7;
        locals.var_k2_stress_edge_dn8 = assign15690_e21582_d_n8;
        locals.var_k2_stress_edge_dn9 = assign15690_e21582_d_n9;
        locals.var_k2_stress_edge_dn10 = assign15690_e21582_d_n10;
        locals.var_k2_stress_edge_dn11 = assign15690_e21582_d_n11;

        let (assign15700_e21596, assign15700_e21596_d_n3, assign15700_e21596_d_n4, assign15700_e21596_d_n5, assign15700_e21596_d_n6, assign15700_e21596_d_n7, assign15700_e21596_d_n8, assign15700_e21596_d_n9, assign15700_e21596_d_n10, assign15700_e21596_d_n11,) = {
    if ((locals.var_guard480 != 0.0) && (locals.var_guard481 != 0.0)) {
        let assign15700_e21589: f64 = (locals.var_kstress_vth0).powf(p.p1122);
        let assign15700_e21590: f64 = (locals.var_steta0edge_i / assign15700_e21589);
        let assign15700_e21593: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15700_e21594: f64 = (assign15700_e21590 * assign15700_e21593);
        (assign15700_e21594, (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15700_e21589 * (p.p1122 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15700_e21589 * assign15700_e21589))) * assign15700_e21593) + (assign15700_e21590 * locals.var_inv_od_dn3)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15700_e21589 * (p.p1122 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15700_e21589 * assign15700_e21589))) * assign15700_e21593) + (assign15700_e21590 * locals.var_inv_od_dn4)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15700_e21589 * (p.p1122 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15700_e21589 * assign15700_e21589))) * assign15700_e21593) + (assign15700_e21590 * locals.var_inv_od_dn5)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15700_e21589 * (p.p1122 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15700_e21589 * assign15700_e21589))) * assign15700_e21593) + (assign15700_e21590 * locals.var_inv_od_dn6)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15700_e21589 * (p.p1122 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15700_e21589 * assign15700_e21589))) * assign15700_e21593) + (assign15700_e21590 * locals.var_inv_od_dn7)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15700_e21589 * (p.p1122 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15700_e21589 * assign15700_e21589))) * assign15700_e21593) + (assign15700_e21590 * locals.var_inv_od_dn8)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15700_e21589 * (p.p1122 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15700_e21589 * assign15700_e21589))) * assign15700_e21593) + (assign15700_e21590 * locals.var_inv_od_dn9)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15700_e21589 * (p.p1122 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15700_e21589 * assign15700_e21589))) * assign15700_e21593) + (assign15700_e21590 * locals.var_inv_od_dn10)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p1122) as f64).is_finite() && ((p.p1122) as f64).fract() == 0.0 { if p.p1122 == 0.0 { 0.0 } else { (p.p1122 * ((locals.var_kstress_vth0).powf(p.p1122 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15700_e21589 * (p.p1122 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15700_e21589 * assign15700_e21589))) * assign15700_e21593) + (assign15700_e21590 * locals.var_inv_od_dn11)),)
    } else {
        (locals.var_eta_stress_edge, locals.var_eta_stress_edge_dn3, locals.var_eta_stress_edge_dn4, locals.var_eta_stress_edge_dn5, locals.var_eta_stress_edge_dn6, locals.var_eta_stress_edge_dn7, locals.var_eta_stress_edge_dn8, locals.var_eta_stress_edge_dn9, locals.var_eta_stress_edge_dn10, locals.var_eta_stress_edge_dn11,)
    }
};
        locals.var_eta_stress_edge = assign15700_e21596;
        locals.var_eta_stress_edge_dn3 = assign15700_e21596_d_n3;
        locals.var_eta_stress_edge_dn4 = assign15700_e21596_d_n4;
        locals.var_eta_stress_edge_dn5 = assign15700_e21596_d_n5;
        locals.var_eta_stress_edge_dn6 = assign15700_e21596_d_n6;
        locals.var_eta_stress_edge_dn7 = assign15700_e21596_d_n7;
        locals.var_eta_stress_edge_dn8 = assign15700_e21596_d_n8;
        locals.var_eta_stress_edge_dn9 = assign15700_e21596_d_n9;
        locals.var_eta_stress_edge_dn10 = assign15700_e21596_d_n10;
        locals.var_eta_stress_edge_dn11 = assign15700_e21596_d_n11;

        let (assign15710_e21602, assign15710_e21602_d_n3, assign15710_e21602_d_n4, assign15710_e21602_d_n5, assign15710_e21602_d_n6, assign15710_e21602_d_n7, assign15710_e21602_d_n8, assign15710_e21602_d_n9, assign15710_e21602_d_n10, assign15710_e21602_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15710_e21600: f64 = (locals.var_k2edge_i + locals.var_k2_stress_edge);
        (assign15710_e21600, (locals.var_k2edge_i_dn3 + locals.var_k2_stress_edge_dn3), (locals.var_k2edge_i_dn4 + locals.var_k2_stress_edge_dn4), (locals.var_k2edge_i_dn5 + locals.var_k2_stress_edge_dn5), (locals.var_k2edge_i_dn6 + locals.var_k2_stress_edge_dn6), (locals.var_k2edge_i_dn7 + locals.var_k2_stress_edge_dn7), (locals.var_k2edge_i_dn8 + locals.var_k2_stress_edge_dn8), (locals.var_k2edge_i_dn9 + locals.var_k2_stress_edge_dn9), (locals.var_k2edge_i_dn10 + locals.var_k2_stress_edge_dn10), (locals.var_k2edge_i_dn11 + locals.var_k2_stress_edge_dn11),)
    } else {
        (locals.var_k2edge_i, locals.var_k2edge_i_dn3, locals.var_k2edge_i_dn4, locals.var_k2edge_i_dn5, locals.var_k2edge_i_dn6, locals.var_k2edge_i_dn7, locals.var_k2edge_i_dn8, locals.var_k2edge_i_dn9, locals.var_k2edge_i_dn10, locals.var_k2edge_i_dn11,)
    }
};
        locals.var_k2edge_i = assign15710_e21602;
        locals.var_k2edge_i_dn3 = assign15710_e21602_d_n3;
        locals.var_k2edge_i_dn4 = assign15710_e21602_d_n4;
        locals.var_k2edge_i_dn5 = assign15710_e21602_d_n5;
        locals.var_k2edge_i_dn6 = assign15710_e21602_d_n6;
        locals.var_k2edge_i_dn7 = assign15710_e21602_d_n7;
        locals.var_k2edge_i_dn8 = assign15710_e21602_d_n8;
        locals.var_k2edge_i_dn9 = assign15710_e21602_d_n9;
        locals.var_k2edge_i_dn10 = assign15710_e21602_d_n10;
        locals.var_k2edge_i_dn11 = assign15710_e21602_d_n11;

        let (assign15720_e21608, assign15720_e21608_d_n3, assign15720_e21608_d_n4, assign15720_e21608_d_n5, assign15720_e21608_d_n6, assign15720_e21608_d_n7, assign15720_e21608_d_n8, assign15720_e21608_d_n9, assign15720_e21608_d_n10, assign15720_e21608_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15720_e21606: f64 = (locals.var_eta0edge_i + locals.var_eta_stress_edge);
        (assign15720_e21606, (locals.var_eta0edge_i_dn3 + locals.var_eta_stress_edge_dn3), (locals.var_eta0edge_i_dn4 + locals.var_eta_stress_edge_dn4), (locals.var_eta0edge_i_dn5 + locals.var_eta_stress_edge_dn5), (locals.var_eta0edge_i_dn6 + locals.var_eta_stress_edge_dn6), (locals.var_eta0edge_i_dn7 + locals.var_eta_stress_edge_dn7), (locals.var_eta0edge_i_dn8 + locals.var_eta_stress_edge_dn8), (locals.var_eta0edge_i_dn9 + locals.var_eta_stress_edge_dn9), (locals.var_eta0edge_i_dn10 + locals.var_eta_stress_edge_dn10), (locals.var_eta0edge_i_dn11 + locals.var_eta_stress_edge_dn11),)
    } else {
        (locals.var_eta0edge_i, locals.var_eta0edge_i_dn3, locals.var_eta0edge_i_dn4, locals.var_eta0edge_i_dn5, locals.var_eta0edge_i_dn6, locals.var_eta0edge_i_dn7, locals.var_eta0edge_i_dn8, locals.var_eta0edge_i_dn9, locals.var_eta0edge_i_dn10, locals.var_eta0edge_i_dn11,)
    }
};
        locals.var_eta0edge_i = assign15720_e21608;
        locals.var_eta0edge_i_dn3 = assign15720_e21608_d_n3;
        locals.var_eta0edge_i_dn4 = assign15720_e21608_d_n4;
        locals.var_eta0edge_i_dn5 = assign15720_e21608_d_n5;
        locals.var_eta0edge_i_dn6 = assign15720_e21608_d_n6;
        locals.var_eta0edge_i_dn7 = assign15720_e21608_d_n7;
        locals.var_eta0edge_i_dn8 = assign15720_e21608_d_n8;
        locals.var_eta0edge_i_dn9 = assign15720_e21608_d_n9;
        locals.var_eta0edge_i_dn10 = assign15720_e21608_d_n10;
        locals.var_eta0edge_i_dn11 = assign15720_e21608_d_n11;

        let (assign15730_e21613, assign15730_e21613_d_n3, assign15730_e21613_d_n4, assign15730_e21613_d_n5, assign15730_e21613_d_n6, assign15730_e21613_d_n7, assign15730_e21613_d_n8, assign15730_e21613_d_n9, assign15730_e21613_d_n10, assign15730_e21613_d_n11,) = {
    if (locals.var_guard480 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vth0_stress, locals.var_vth0_stress_dn3, locals.var_vth0_stress_dn4, locals.var_vth0_stress_dn5, locals.var_vth0_stress_dn6, locals.var_vth0_stress_dn7, locals.var_vth0_stress_dn8, locals.var_vth0_stress_dn9, locals.var_vth0_stress_dn10, locals.var_vth0_stress_dn11,)
    }
};
        locals.var_vth0_stress = assign15730_e21613;
        locals.var_vth0_stress_dn3 = assign15730_e21613_d_n3;
        locals.var_vth0_stress_dn4 = assign15730_e21613_d_n4;
        locals.var_vth0_stress_dn5 = assign15730_e21613_d_n5;
        locals.var_vth0_stress_dn6 = assign15730_e21613_d_n6;
        locals.var_vth0_stress_dn7 = assign15730_e21613_d_n7;
        locals.var_vth0_stress_dn8 = assign15730_e21613_d_n8;
        locals.var_vth0_stress_dn9 = assign15730_e21613_d_n9;
        locals.var_vth0_stress_dn10 = assign15730_e21613_d_n10;
        locals.var_vth0_stress_dn11 = assign15730_e21613_d_n11;

    }

    pub(super) fn stamp_transient_block_27(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (assign15740_e21618, assign15740_e21618_d_n3, assign15740_e21618_d_n4, assign15740_e21618_d_n5, assign15740_e21618_d_n6, assign15740_e21618_d_n7, assign15740_e21618_d_n8, assign15740_e21618_d_n9, assign15740_e21618_d_n10, assign15740_e21618_d_n11,) = {
    if (locals.var_guard480 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vth0_stress_edge, locals.var_vth0_stress_edge_dn3, locals.var_vth0_stress_edge_dn4, locals.var_vth0_stress_edge_dn5, locals.var_vth0_stress_edge_dn6, locals.var_vth0_stress_edge_dn7, locals.var_vth0_stress_edge_dn8, locals.var_vth0_stress_edge_dn9, locals.var_vth0_stress_edge_dn10, locals.var_vth0_stress_edge_dn11,)
    }
};
        locals.var_vth0_stress_edge = assign15740_e21618;
        locals.var_vth0_stress_edge_dn3 = assign15740_e21618_d_n3;
        locals.var_vth0_stress_edge_dn4 = assign15740_e21618_d_n4;
        locals.var_vth0_stress_edge_dn5 = assign15740_e21618_d_n5;
        locals.var_vth0_stress_edge_dn6 = assign15740_e21618_d_n6;
        locals.var_vth0_stress_edge_dn7 = assign15740_e21618_d_n7;
        locals.var_vth0_stress_edge_dn8 = assign15740_e21618_d_n8;
        locals.var_vth0_stress_edge_dn9 = assign15740_e21618_d_n9;
        locals.var_vth0_stress_edge_dn10 = assign15740_e21618_d_n10;
        locals.var_vth0_stress_edge_dn11 = assign15740_e21618_d_n11;

        let assign15750_e21621: f64 = if p.p34 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard482 = assign15750_e21621;

        let (assign15760_e21627,) = {
    if (locals.var_guard482 != 0.0) {
        let assign15760_e21625: f64 = (p.p1 / p.p2);
        (assign15760_e21625,)
    } else {
        (locals.var_wdrn,)
    }
};
        locals.var_wdrn = assign15760_e21627;

        let (assign15770_e21631, assign15770_e21631_d_n3, assign15770_e21631_d_n4, assign15770_e21631_d_n5, assign15770_e21631_d_n6, assign15770_e21631_d_n7, assign15770_e21631_d_n8, assign15770_e21631_d_n9, assign15770_e21631_d_n10, assign15770_e21631_d_n11,) = {
    if (locals.var_guard482 != 0.0) {
        (p.p13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_local_sca, locals.var_local_sca_dn3, locals.var_local_sca_dn4, locals.var_local_sca_dn5, locals.var_local_sca_dn6, locals.var_local_sca_dn7, locals.var_local_sca_dn8, locals.var_local_sca_dn9, locals.var_local_sca_dn10, locals.var_local_sca_dn11,)
    }
};
        locals.var_local_sca = assign15770_e21631;
        locals.var_local_sca_dn3 = assign15770_e21631_d_n3;
        locals.var_local_sca_dn4 = assign15770_e21631_d_n4;
        locals.var_local_sca_dn5 = assign15770_e21631_d_n5;
        locals.var_local_sca_dn6 = assign15770_e21631_d_n6;
        locals.var_local_sca_dn7 = assign15770_e21631_d_n7;
        locals.var_local_sca_dn8 = assign15770_e21631_d_n8;
        locals.var_local_sca_dn9 = assign15770_e21631_d_n9;
        locals.var_local_sca_dn10 = assign15770_e21631_d_n10;
        locals.var_local_sca_dn11 = assign15770_e21631_d_n11;

        let (assign15780_e21635, assign15780_e21635_d_n3, assign15780_e21635_d_n4, assign15780_e21635_d_n5, assign15780_e21635_d_n6, assign15780_e21635_d_n7, assign15780_e21635_d_n8, assign15780_e21635_d_n9, assign15780_e21635_d_n10, assign15780_e21635_d_n11,) = {
    if (locals.var_guard482 != 0.0) {
        (p.p14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_local_scb, locals.var_local_scb_dn3, locals.var_local_scb_dn4, locals.var_local_scb_dn5, locals.var_local_scb_dn6, locals.var_local_scb_dn7, locals.var_local_scb_dn8, locals.var_local_scb_dn9, locals.var_local_scb_dn10, locals.var_local_scb_dn11,)
    }
};
        locals.var_local_scb = assign15780_e21635;
        locals.var_local_scb_dn3 = assign15780_e21635_d_n3;
        locals.var_local_scb_dn4 = assign15780_e21635_d_n4;
        locals.var_local_scb_dn5 = assign15780_e21635_d_n5;
        locals.var_local_scb_dn6 = assign15780_e21635_d_n6;
        locals.var_local_scb_dn7 = assign15780_e21635_d_n7;
        locals.var_local_scb_dn8 = assign15780_e21635_d_n8;
        locals.var_local_scb_dn9 = assign15780_e21635_d_n9;
        locals.var_local_scb_dn10 = assign15780_e21635_d_n10;
        locals.var_local_scb_dn11 = assign15780_e21635_d_n11;

        let (assign15790_e21639, assign15790_e21639_d_n3, assign15790_e21639_d_n4, assign15790_e21639_d_n5, assign15790_e21639_d_n6, assign15790_e21639_d_n7, assign15790_e21639_d_n8, assign15790_e21639_d_n9, assign15790_e21639_d_n10, assign15790_e21639_d_n11,) = {
    if (locals.var_guard482 != 0.0) {
        (p.p15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_local_scc, locals.var_local_scc_dn3, locals.var_local_scc_dn4, locals.var_local_scc_dn5, locals.var_local_scc_dn6, locals.var_local_scc_dn7, locals.var_local_scc_dn8, locals.var_local_scc_dn9, locals.var_local_scc_dn10, locals.var_local_scc_dn11,)
    }
};
        locals.var_local_scc = assign15790_e21639;
        locals.var_local_scc_dn3 = assign15790_e21639_d_n3;
        locals.var_local_scc_dn4 = assign15790_e21639_d_n4;
        locals.var_local_scc_dn5 = assign15790_e21639_d_n5;
        locals.var_local_scc_dn6 = assign15790_e21639_d_n6;
        locals.var_local_scc_dn7 = assign15790_e21639_d_n7;
        locals.var_local_scc_dn8 = assign15790_e21639_d_n8;
        locals.var_local_scc_dn9 = assign15790_e21639_d_n9;
        locals.var_local_scc_dn10 = assign15790_e21639_d_n10;
        locals.var_local_scc_dn11 = assign15790_e21639_d_n11;

        let assign15800_e21650: f64 = if (((!param_given[13]) && (!param_given[14])) && (!param_given[15])) { 1.0 } else { 0.0 };
        locals.var_guard483 = assign15800_e21650;

        let assign15810_e21656: f64 = if (param_given[16] && (p.p16 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard484 = assign15810_e21656;

        let (assign15820_e21666, assign15820_e21666_d_n3, assign15820_e21666_d_n4, assign15820_e21666_d_n5, assign15820_e21666_d_n6, assign15820_e21666_d_n7, assign15820_e21666_d_n8, assign15820_e21666_d_n9, assign15820_e21666_d_n10, assign15820_e21666_d_n11,) = {
    if (((locals.var_guard482 != 0.0) && (locals.var_guard483 != 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign15820_e21664: f64 = (p.p16 + locals.var_wdrn);
        (assign15820_e21664, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign15820_e21666;
        locals.var_t1_dn3 = assign15820_e21666_d_n3;
        locals.var_t1_dn4 = assign15820_e21666_d_n4;
        locals.var_t1_dn5 = assign15820_e21666_d_n5;
        locals.var_t1_dn6 = assign15820_e21666_d_n6;
        locals.var_t1_dn7 = assign15820_e21666_d_n7;
        locals.var_t1_dn8 = assign15820_e21666_d_n8;
        locals.var_t1_dn9 = assign15820_e21666_d_n9;
        locals.var_t1_dn10 = assign15820_e21666_d_n10;
        locals.var_t1_dn11 = assign15820_e21666_d_n11;

        let (assign15830_e21676, assign15830_e21676_d_n3, assign15830_e21676_d_n4, assign15830_e21676_d_n5, assign15830_e21676_d_n6, assign15830_e21676_d_n7, assign15830_e21676_d_n8, assign15830_e21676_d_n9, assign15830_e21676_d_n10, assign15830_e21676_d_n11,) = {
    if (((locals.var_guard482 != 0.0) && (locals.var_guard483 != 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign15830_e21674: f64 = (1.0 / p.p1137);
        (assign15830_e21674, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign15830_e21676;
        locals.var_t2_dn3 = assign15830_e21676_d_n3;
        locals.var_t2_dn4 = assign15830_e21676_d_n4;
        locals.var_t2_dn5 = assign15830_e21676_d_n5;
        locals.var_t2_dn6 = assign15830_e21676_d_n6;
        locals.var_t2_dn7 = assign15830_e21676_d_n7;
        locals.var_t2_dn8 = assign15830_e21676_d_n8;
        locals.var_t2_dn9 = assign15830_e21676_d_n9;
        locals.var_t2_dn10 = assign15830_e21676_d_n10;
        locals.var_t2_dn11 = assign15830_e21676_d_n11;

        let (assign15840_e21690, assign15840_e21690_d_n3, assign15840_e21690_d_n4, assign15840_e21690_d_n5, assign15840_e21690_d_n6, assign15840_e21690_d_n7, assign15840_e21690_d_n8, assign15840_e21690_d_n9, assign15840_e21690_d_n10, assign15840_e21690_d_n11,) = {
    if (((locals.var_guard482 != 0.0) && (locals.var_guard483 != 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign15840_e21684: f64 = (p.p1137 * p.p1137);
        let assign15840_e21687: f64 = (p.p16 * locals.var_t1);
        let assign15840_e21688: f64 = (assign15840_e21684 / assign15840_e21687);
        (assign15840_e21688, (-((assign15840_e21684 * (p.p16 * locals.var_t1_dn3)) / (assign15840_e21687 * assign15840_e21687))), (-((assign15840_e21684 * (p.p16 * locals.var_t1_dn4)) / (assign15840_e21687 * assign15840_e21687))), (-((assign15840_e21684 * (p.p16 * locals.var_t1_dn5)) / (assign15840_e21687 * assign15840_e21687))), (-((assign15840_e21684 * (p.p16 * locals.var_t1_dn6)) / (assign15840_e21687 * assign15840_e21687))), (-((assign15840_e21684 * (p.p16 * locals.var_t1_dn7)) / (assign15840_e21687 * assign15840_e21687))), (-((assign15840_e21684 * (p.p16 * locals.var_t1_dn8)) / (assign15840_e21687 * assign15840_e21687))), (-((assign15840_e21684 * (p.p16 * locals.var_t1_dn9)) / (assign15840_e21687 * assign15840_e21687))), (-((assign15840_e21684 * (p.p16 * locals.var_t1_dn10)) / (assign15840_e21687 * assign15840_e21687))), (-((assign15840_e21684 * (p.p16 * locals.var_t1_dn11)) / (assign15840_e21687 * assign15840_e21687))),)
    } else {
        (locals.var_local_sca, locals.var_local_sca_dn3, locals.var_local_sca_dn4, locals.var_local_sca_dn5, locals.var_local_sca_dn6, locals.var_local_sca_dn7, locals.var_local_sca_dn8, locals.var_local_sca_dn9, locals.var_local_sca_dn10, locals.var_local_sca_dn11,)
    }
};
        locals.var_local_sca = assign15840_e21690;
        locals.var_local_sca_dn3 = assign15840_e21690_d_n3;
        locals.var_local_sca_dn4 = assign15840_e21690_d_n4;
        locals.var_local_sca_dn5 = assign15840_e21690_d_n5;
        locals.var_local_sca_dn6 = assign15840_e21690_d_n6;
        locals.var_local_sca_dn7 = assign15840_e21690_d_n7;
        locals.var_local_sca_dn8 = assign15840_e21690_d_n8;
        locals.var_local_sca_dn9 = assign15840_e21690_d_n9;
        locals.var_local_sca_dn10 = assign15840_e21690_d_n10;
        locals.var_local_sca_dn11 = assign15840_e21690_d_n11;

        let (assign15850_e21730, assign15850_e21730_d_n3, assign15850_e21730_d_n4, assign15850_e21730_d_n5, assign15850_e21730_d_n6, assign15850_e21730_d_n7, assign15850_e21730_d_n8, assign15850_e21730_d_n9, assign15850_e21730_d_n10, assign15850_e21730_d_n11,) = {
    if (((locals.var_guard482 != 0.0) && (locals.var_guard483 != 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign15850_e21698: f64 = (0.1 * p.p16);
        let assign15850_e21701: f64 = (0.01 * p.p1137);
        let assign15850_e21702: f64 = (assign15850_e21698 + assign15850_e21701);
        let assign15850_e21704: f64 = (-10.0);
        let assign15850_e21706: f64 = (assign15850_e21704 * p.p16);
        let assign15850_e21708: f64 = (assign15850_e21706 * locals.var_t2);
        let assign15850_e21709: f64 = { let limited_exp_arg = assign15850_e21708; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15850_e21710: f64 = (assign15850_e21702 * assign15850_e21709);
        let assign15850_e21713: f64 = (0.1 * locals.var_t1);
        let assign15850_e21716: f64 = (0.01 * p.p1137);
        let assign15850_e21717: f64 = (assign15850_e21713 + assign15850_e21716);
        let assign15850_e21719: f64 = (-10.0);
        let assign15850_e21721: f64 = (assign15850_e21719 * locals.var_t1);
        let assign15850_e21723: f64 = (assign15850_e21721 * locals.var_t2);
        let assign15850_e21724: f64 = { let limited_exp_arg = assign15850_e21723; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15850_e21725: f64 = (assign15850_e21717 * assign15850_e21724);
        let assign15850_e21726: f64 = (assign15850_e21710 - assign15850_e21725);
        let assign15850_e21728: f64 = (assign15850_e21726 / locals.var_wdrn);
        (assign15850_e21728, (((assign15850_e21702 * ({ let limited_exp_arg = assign15850_e21708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15850_e21706 * locals.var_t2_dn3))) - (((0.1 * locals.var_t1_dn3) * assign15850_e21724) + (assign15850_e21717 * ({ let limited_exp_arg = assign15850_e21723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15850_e21719 * locals.var_t1_dn3) * locals.var_t2) + (assign15850_e21721 * locals.var_t2_dn3)))))) / locals.var_wdrn), (((assign15850_e21702 * ({ let limited_exp_arg = assign15850_e21708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15850_e21706 * locals.var_t2_dn4))) - (((0.1 * locals.var_t1_dn4) * assign15850_e21724) + (assign15850_e21717 * ({ let limited_exp_arg = assign15850_e21723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15850_e21719 * locals.var_t1_dn4) * locals.var_t2) + (assign15850_e21721 * locals.var_t2_dn4)))))) / locals.var_wdrn), (((assign15850_e21702 * ({ let limited_exp_arg = assign15850_e21708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15850_e21706 * locals.var_t2_dn5))) - (((0.1 * locals.var_t1_dn5) * assign15850_e21724) + (assign15850_e21717 * ({ let limited_exp_arg = assign15850_e21723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15850_e21719 * locals.var_t1_dn5) * locals.var_t2) + (assign15850_e21721 * locals.var_t2_dn5)))))) / locals.var_wdrn), (((assign15850_e21702 * ({ let limited_exp_arg = assign15850_e21708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15850_e21706 * locals.var_t2_dn6))) - (((0.1 * locals.var_t1_dn6) * assign15850_e21724) + (assign15850_e21717 * ({ let limited_exp_arg = assign15850_e21723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15850_e21719 * locals.var_t1_dn6) * locals.var_t2) + (assign15850_e21721 * locals.var_t2_dn6)))))) / locals.var_wdrn), (((assign15850_e21702 * ({ let limited_exp_arg = assign15850_e21708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15850_e21706 * locals.var_t2_dn7))) - (((0.1 * locals.var_t1_dn7) * assign15850_e21724) + (assign15850_e21717 * ({ let limited_exp_arg = assign15850_e21723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15850_e21719 * locals.var_t1_dn7) * locals.var_t2) + (assign15850_e21721 * locals.var_t2_dn7)))))) / locals.var_wdrn), (((assign15850_e21702 * ({ let limited_exp_arg = assign15850_e21708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15850_e21706 * locals.var_t2_dn8))) - (((0.1 * locals.var_t1_dn8) * assign15850_e21724) + (assign15850_e21717 * ({ let limited_exp_arg = assign15850_e21723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15850_e21719 * locals.var_t1_dn8) * locals.var_t2) + (assign15850_e21721 * locals.var_t2_dn8)))))) / locals.var_wdrn), (((assign15850_e21702 * ({ let limited_exp_arg = assign15850_e21708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15850_e21706 * locals.var_t2_dn9))) - (((0.1 * locals.var_t1_dn9) * assign15850_e21724) + (assign15850_e21717 * ({ let limited_exp_arg = assign15850_e21723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15850_e21719 * locals.var_t1_dn9) * locals.var_t2) + (assign15850_e21721 * locals.var_t2_dn9)))))) / locals.var_wdrn), (((assign15850_e21702 * ({ let limited_exp_arg = assign15850_e21708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15850_e21706 * locals.var_t2_dn10))) - (((0.1 * locals.var_t1_dn10) * assign15850_e21724) + (assign15850_e21717 * ({ let limited_exp_arg = assign15850_e21723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15850_e21719 * locals.var_t1_dn10) * locals.var_t2) + (assign15850_e21721 * locals.var_t2_dn10)))))) / locals.var_wdrn), (((assign15850_e21702 * ({ let limited_exp_arg = assign15850_e21708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15850_e21706 * locals.var_t2_dn11))) - (((0.1 * locals.var_t1_dn11) * assign15850_e21724) + (assign15850_e21717 * ({ let limited_exp_arg = assign15850_e21723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15850_e21719 * locals.var_t1_dn11) * locals.var_t2) + (assign15850_e21721 * locals.var_t2_dn11)))))) / locals.var_wdrn),)
    } else {
        (locals.var_local_scb, locals.var_local_scb_dn3, locals.var_local_scb_dn4, locals.var_local_scb_dn5, locals.var_local_scb_dn6, locals.var_local_scb_dn7, locals.var_local_scb_dn8, locals.var_local_scb_dn9, locals.var_local_scb_dn10, locals.var_local_scb_dn11,)
    }
};
        locals.var_local_scb = assign15850_e21730;
        locals.var_local_scb_dn3 = assign15850_e21730_d_n3;
        locals.var_local_scb_dn4 = assign15850_e21730_d_n4;
        locals.var_local_scb_dn5 = assign15850_e21730_d_n5;
        locals.var_local_scb_dn6 = assign15850_e21730_d_n6;
        locals.var_local_scb_dn7 = assign15850_e21730_d_n7;
        locals.var_local_scb_dn8 = assign15850_e21730_d_n8;
        locals.var_local_scb_dn9 = assign15850_e21730_d_n9;
        locals.var_local_scb_dn10 = assign15850_e21730_d_n10;
        locals.var_local_scb_dn11 = assign15850_e21730_d_n11;

        let (assign15860_e21770, assign15860_e21770_d_n3, assign15860_e21770_d_n4, assign15860_e21770_d_n5, assign15860_e21770_d_n6, assign15860_e21770_d_n7, assign15860_e21770_d_n8, assign15860_e21770_d_n9, assign15860_e21770_d_n10, assign15860_e21770_d_n11,) = {
    if (((locals.var_guard482 != 0.0) && (locals.var_guard483 != 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign15860_e21738: f64 = (0.05 * p.p16);
        let assign15860_e21741: f64 = (0.0025 * p.p1137);
        let assign15860_e21742: f64 = (assign15860_e21738 + assign15860_e21741);
        let assign15860_e21744: f64 = (-20.0);
        let assign15860_e21746: f64 = (assign15860_e21744 * p.p16);
        let assign15860_e21748: f64 = (assign15860_e21746 * locals.var_t2);
        let assign15860_e21749: f64 = { let limited_exp_arg = assign15860_e21748; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15860_e21750: f64 = (assign15860_e21742 * assign15860_e21749);
        let assign15860_e21753: f64 = (0.05 * locals.var_t1);
        let assign15860_e21756: f64 = (0.0025 * p.p1137);
        let assign15860_e21757: f64 = (assign15860_e21753 + assign15860_e21756);
        let assign15860_e21759: f64 = (-20.0);
        let assign15860_e21761: f64 = (assign15860_e21759 * locals.var_t1);
        let assign15860_e21763: f64 = (assign15860_e21761 * locals.var_t2);
        let assign15860_e21764: f64 = { let limited_exp_arg = assign15860_e21763; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15860_e21765: f64 = (assign15860_e21757 * assign15860_e21764);
        let assign15860_e21766: f64 = (assign15860_e21750 - assign15860_e21765);
        let assign15860_e21768: f64 = (assign15860_e21766 / locals.var_wdrn);
        (assign15860_e21768, (((assign15860_e21742 * ({ let limited_exp_arg = assign15860_e21748; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15860_e21746 * locals.var_t2_dn3))) - (((0.05 * locals.var_t1_dn3) * assign15860_e21764) + (assign15860_e21757 * ({ let limited_exp_arg = assign15860_e21763; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15860_e21759 * locals.var_t1_dn3) * locals.var_t2) + (assign15860_e21761 * locals.var_t2_dn3)))))) / locals.var_wdrn), (((assign15860_e21742 * ({ let limited_exp_arg = assign15860_e21748; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15860_e21746 * locals.var_t2_dn4))) - (((0.05 * locals.var_t1_dn4) * assign15860_e21764) + (assign15860_e21757 * ({ let limited_exp_arg = assign15860_e21763; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15860_e21759 * locals.var_t1_dn4) * locals.var_t2) + (assign15860_e21761 * locals.var_t2_dn4)))))) / locals.var_wdrn), (((assign15860_e21742 * ({ let limited_exp_arg = assign15860_e21748; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15860_e21746 * locals.var_t2_dn5))) - (((0.05 * locals.var_t1_dn5) * assign15860_e21764) + (assign15860_e21757 * ({ let limited_exp_arg = assign15860_e21763; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15860_e21759 * locals.var_t1_dn5) * locals.var_t2) + (assign15860_e21761 * locals.var_t2_dn5)))))) / locals.var_wdrn), (((assign15860_e21742 * ({ let limited_exp_arg = assign15860_e21748; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15860_e21746 * locals.var_t2_dn6))) - (((0.05 * locals.var_t1_dn6) * assign15860_e21764) + (assign15860_e21757 * ({ let limited_exp_arg = assign15860_e21763; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15860_e21759 * locals.var_t1_dn6) * locals.var_t2) + (assign15860_e21761 * locals.var_t2_dn6)))))) / locals.var_wdrn), (((assign15860_e21742 * ({ let limited_exp_arg = assign15860_e21748; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15860_e21746 * locals.var_t2_dn7))) - (((0.05 * locals.var_t1_dn7) * assign15860_e21764) + (assign15860_e21757 * ({ let limited_exp_arg = assign15860_e21763; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15860_e21759 * locals.var_t1_dn7) * locals.var_t2) + (assign15860_e21761 * locals.var_t2_dn7)))))) / locals.var_wdrn), (((assign15860_e21742 * ({ let limited_exp_arg = assign15860_e21748; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15860_e21746 * locals.var_t2_dn8))) - (((0.05 * locals.var_t1_dn8) * assign15860_e21764) + (assign15860_e21757 * ({ let limited_exp_arg = assign15860_e21763; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15860_e21759 * locals.var_t1_dn8) * locals.var_t2) + (assign15860_e21761 * locals.var_t2_dn8)))))) / locals.var_wdrn), (((assign15860_e21742 * ({ let limited_exp_arg = assign15860_e21748; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15860_e21746 * locals.var_t2_dn9))) - (((0.05 * locals.var_t1_dn9) * assign15860_e21764) + (assign15860_e21757 * ({ let limited_exp_arg = assign15860_e21763; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15860_e21759 * locals.var_t1_dn9) * locals.var_t2) + (assign15860_e21761 * locals.var_t2_dn9)))))) / locals.var_wdrn), (((assign15860_e21742 * ({ let limited_exp_arg = assign15860_e21748; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15860_e21746 * locals.var_t2_dn10))) - (((0.05 * locals.var_t1_dn10) * assign15860_e21764) + (assign15860_e21757 * ({ let limited_exp_arg = assign15860_e21763; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15860_e21759 * locals.var_t1_dn10) * locals.var_t2) + (assign15860_e21761 * locals.var_t2_dn10)))))) / locals.var_wdrn), (((assign15860_e21742 * ({ let limited_exp_arg = assign15860_e21748; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign15860_e21746 * locals.var_t2_dn11))) - (((0.05 * locals.var_t1_dn11) * assign15860_e21764) + (assign15860_e21757 * ({ let limited_exp_arg = assign15860_e21763; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign15860_e21759 * locals.var_t1_dn11) * locals.var_t2) + (assign15860_e21761 * locals.var_t2_dn11)))))) / locals.var_wdrn),)
    } else {
        (locals.var_local_scc, locals.var_local_scc_dn3, locals.var_local_scc_dn4, locals.var_local_scc_dn5, locals.var_local_scc_dn6, locals.var_local_scc_dn7, locals.var_local_scc_dn8, locals.var_local_scc_dn9, locals.var_local_scc_dn10, locals.var_local_scc_dn11,)
    }
};
        locals.var_local_scc = assign15860_e21770;
        locals.var_local_scc_dn3 = assign15860_e21770_d_n3;
        locals.var_local_scc_dn4 = assign15860_e21770_d_n4;
        locals.var_local_scc_dn5 = assign15860_e21770_d_n5;
        locals.var_local_scc_dn6 = assign15860_e21770_d_n6;
        locals.var_local_scc_dn7 = assign15860_e21770_d_n7;
        locals.var_local_scc_dn8 = assign15860_e21770_d_n8;
        locals.var_local_scc_dn9 = assign15860_e21770_d_n9;
        locals.var_local_scc_dn10 = assign15860_e21770_d_n10;
        locals.var_local_scc_dn11 = assign15860_e21770_d_n11;

        let assign15870_e21775: f64 = (p.p1123 * locals.var_local_scb);
        let assign15870_e21776: f64 = (locals.var_local_sca + assign15870_e21775);
        let assign15870_e21779: f64 = (p.p1124 * locals.var_local_scc);
        let assign15870_e21780: f64 = (assign15870_e21776 + assign15870_e21779);
        let assign15870_e21781: f64 = (locals.var_kvth0we_i * assign15870_e21780);
        locals.var_vth0_well = assign15870_e21781;
        locals.var_vth0_well_dn3 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn3 + (p.p1123 * locals.var_local_scb_dn3)) + (p.p1124 * locals.var_local_scc_dn3)));
        locals.var_vth0_well_dn4 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn4 + (p.p1123 * locals.var_local_scb_dn4)) + (p.p1124 * locals.var_local_scc_dn4)));
        locals.var_vth0_well_dn5 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn5 + (p.p1123 * locals.var_local_scb_dn5)) + (p.p1124 * locals.var_local_scc_dn5)));
        locals.var_vth0_well_dn6 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn6 + (p.p1123 * locals.var_local_scb_dn6)) + (p.p1124 * locals.var_local_scc_dn6)));
        locals.var_vth0_well_dn7 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn7 + (p.p1123 * locals.var_local_scb_dn7)) + (p.p1124 * locals.var_local_scc_dn7)));
        locals.var_vth0_well_dn8 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn8 + (p.p1123 * locals.var_local_scb_dn8)) + (p.p1124 * locals.var_local_scc_dn8)));
        locals.var_vth0_well_dn9 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn9 + (p.p1123 * locals.var_local_scb_dn9)) + (p.p1124 * locals.var_local_scc_dn9)));
        locals.var_vth0_well_dn10 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn10 + (p.p1123 * locals.var_local_scb_dn10)) + (p.p1124 * locals.var_local_scc_dn10)));
        locals.var_vth0_well_dn11 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn11 + (p.p1123 * locals.var_local_scb_dn11)) + (p.p1124 * locals.var_local_scc_dn11)));

        let assign15880_e21786: f64 = (p.p1123 * locals.var_local_scb);
        let assign15880_e21787: f64 = (locals.var_local_sca + assign15880_e21786);
        let assign15880_e21790: f64 = (p.p1124 * locals.var_local_scc);
        let assign15880_e21791: f64 = (assign15880_e21787 + assign15880_e21790);
        let assign15880_e21792: f64 = (locals.var_k2we_i * assign15880_e21791);
        locals.var_k2_well = assign15880_e21792;
        locals.var_k2_well_dn3 = (locals.var_k2we_i * ((locals.var_local_sca_dn3 + (p.p1123 * locals.var_local_scb_dn3)) + (p.p1124 * locals.var_local_scc_dn3)));
        locals.var_k2_well_dn4 = (locals.var_k2we_i * ((locals.var_local_sca_dn4 + (p.p1123 * locals.var_local_scb_dn4)) + (p.p1124 * locals.var_local_scc_dn4)));
        locals.var_k2_well_dn5 = (locals.var_k2we_i * ((locals.var_local_sca_dn5 + (p.p1123 * locals.var_local_scb_dn5)) + (p.p1124 * locals.var_local_scc_dn5)));
        locals.var_k2_well_dn6 = (locals.var_k2we_i * ((locals.var_local_sca_dn6 + (p.p1123 * locals.var_local_scb_dn6)) + (p.p1124 * locals.var_local_scc_dn6)));
        locals.var_k2_well_dn7 = (locals.var_k2we_i * ((locals.var_local_sca_dn7 + (p.p1123 * locals.var_local_scb_dn7)) + (p.p1124 * locals.var_local_scc_dn7)));
        locals.var_k2_well_dn8 = (locals.var_k2we_i * ((locals.var_local_sca_dn8 + (p.p1123 * locals.var_local_scb_dn8)) + (p.p1124 * locals.var_local_scc_dn8)));
        locals.var_k2_well_dn9 = (locals.var_k2we_i * ((locals.var_local_sca_dn9 + (p.p1123 * locals.var_local_scb_dn9)) + (p.p1124 * locals.var_local_scc_dn9)));
        locals.var_k2_well_dn10 = (locals.var_k2we_i * ((locals.var_local_sca_dn10 + (p.p1123 * locals.var_local_scb_dn10)) + (p.p1124 * locals.var_local_scc_dn10)));
        locals.var_k2_well_dn11 = (locals.var_k2we_i * ((locals.var_local_sca_dn11 + (p.p1123 * locals.var_local_scb_dn11)) + (p.p1124 * locals.var_local_scc_dn11)));

        let assign15890_e21797: f64 = (p.p1123 * locals.var_local_scb);
        let assign15890_e21798: f64 = (locals.var_local_sca + assign15890_e21797);
        let assign15890_e21801: f64 = (p.p1124 * locals.var_local_scc);
        let assign15890_e21802: f64 = (assign15890_e21798 + assign15890_e21801);
        let assign15890_e21803: f64 = (locals.var_kvth0edgewe_i * assign15890_e21802);
        locals.var_vth0_well_edge = assign15890_e21803;
        locals.var_vth0_well_edge_dn3 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn3 + (p.p1123 * locals.var_local_scb_dn3)) + (p.p1124 * locals.var_local_scc_dn3)));
        locals.var_vth0_well_edge_dn4 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn4 + (p.p1123 * locals.var_local_scb_dn4)) + (p.p1124 * locals.var_local_scc_dn4)));
        locals.var_vth0_well_edge_dn5 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn5 + (p.p1123 * locals.var_local_scb_dn5)) + (p.p1124 * locals.var_local_scc_dn5)));
        locals.var_vth0_well_edge_dn6 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn6 + (p.p1123 * locals.var_local_scb_dn6)) + (p.p1124 * locals.var_local_scc_dn6)));
        locals.var_vth0_well_edge_dn7 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn7 + (p.p1123 * locals.var_local_scb_dn7)) + (p.p1124 * locals.var_local_scc_dn7)));
        locals.var_vth0_well_edge_dn8 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn8 + (p.p1123 * locals.var_local_scb_dn8)) + (p.p1124 * locals.var_local_scc_dn8)));
        locals.var_vth0_well_edge_dn9 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn9 + (p.p1123 * locals.var_local_scb_dn9)) + (p.p1124 * locals.var_local_scc_dn9)));
        locals.var_vth0_well_edge_dn10 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn10 + (p.p1123 * locals.var_local_scb_dn10)) + (p.p1124 * locals.var_local_scc_dn10)));
        locals.var_vth0_well_edge_dn11 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn11 + (p.p1123 * locals.var_local_scb_dn11)) + (p.p1124 * locals.var_local_scc_dn11)));

        let assign15900_e21808: f64 = (p.p1123 * locals.var_local_scb);
        let assign15900_e21809: f64 = (locals.var_local_sca + assign15900_e21808);
        let assign15900_e21812: f64 = (p.p1124 * locals.var_local_scc);
        let assign15900_e21813: f64 = (assign15900_e21809 + assign15900_e21812);
        let assign15900_e21814: f64 = (locals.var_k2edgewe_i * assign15900_e21813);
        locals.var_k2_well_edge = assign15900_e21814;
        locals.var_k2_well_edge_dn3 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn3 + (p.p1123 * locals.var_local_scb_dn3)) + (p.p1124 * locals.var_local_scc_dn3)));
        locals.var_k2_well_edge_dn4 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn4 + (p.p1123 * locals.var_local_scb_dn4)) + (p.p1124 * locals.var_local_scc_dn4)));
        locals.var_k2_well_edge_dn5 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn5 + (p.p1123 * locals.var_local_scb_dn5)) + (p.p1124 * locals.var_local_scc_dn5)));
        locals.var_k2_well_edge_dn6 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn6 + (p.p1123 * locals.var_local_scb_dn6)) + (p.p1124 * locals.var_local_scc_dn6)));
        locals.var_k2_well_edge_dn7 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn7 + (p.p1123 * locals.var_local_scb_dn7)) + (p.p1124 * locals.var_local_scc_dn7)));
        locals.var_k2_well_edge_dn8 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn8 + (p.p1123 * locals.var_local_scb_dn8)) + (p.p1124 * locals.var_local_scc_dn8)));
        locals.var_k2_well_edge_dn9 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn9 + (p.p1123 * locals.var_local_scb_dn9)) + (p.p1124 * locals.var_local_scc_dn9)));
        locals.var_k2_well_edge_dn10 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn10 + (p.p1123 * locals.var_local_scb_dn10)) + (p.p1124 * locals.var_local_scc_dn10)));
        locals.var_k2_well_edge_dn11 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn11 + (p.p1123 * locals.var_local_scb_dn11)) + (p.p1124 * locals.var_local_scc_dn11)));

        let assign15910_e21820: f64 = (p.p1123 * locals.var_local_scb);
        let assign15910_e21821: f64 = (locals.var_local_sca + assign15910_e21820);
        let assign15910_e21824: f64 = (p.p1124 * locals.var_local_scc);
        let assign15910_e21825: f64 = (assign15910_e21821 + assign15910_e21824);
        let assign15910_e21826: f64 = (locals.var_ku0we_i * assign15910_e21825);
        let assign15910_e21827: f64 = (1.0 + assign15910_e21826);
        locals.var_mu_well = assign15910_e21827;
        locals.var_mu_well_dn3 = (locals.var_ku0we_i * ((locals.var_local_sca_dn3 + (p.p1123 * locals.var_local_scb_dn3)) + (p.p1124 * locals.var_local_scc_dn3)));
        locals.var_mu_well_dn4 = (locals.var_ku0we_i * ((locals.var_local_sca_dn4 + (p.p1123 * locals.var_local_scb_dn4)) + (p.p1124 * locals.var_local_scc_dn4)));
        locals.var_mu_well_dn5 = (locals.var_ku0we_i * ((locals.var_local_sca_dn5 + (p.p1123 * locals.var_local_scb_dn5)) + (p.p1124 * locals.var_local_scc_dn5)));
        locals.var_mu_well_dn6 = (locals.var_ku0we_i * ((locals.var_local_sca_dn6 + (p.p1123 * locals.var_local_scb_dn6)) + (p.p1124 * locals.var_local_scc_dn6)));
        locals.var_mu_well_dn7 = (locals.var_ku0we_i * ((locals.var_local_sca_dn7 + (p.p1123 * locals.var_local_scb_dn7)) + (p.p1124 * locals.var_local_scc_dn7)));
        locals.var_mu_well_dn8 = (locals.var_ku0we_i * ((locals.var_local_sca_dn8 + (p.p1123 * locals.var_local_scb_dn8)) + (p.p1124 * locals.var_local_scc_dn8)));
        locals.var_mu_well_dn9 = (locals.var_ku0we_i * ((locals.var_local_sca_dn9 + (p.p1123 * locals.var_local_scb_dn9)) + (p.p1124 * locals.var_local_scc_dn9)));
        locals.var_mu_well_dn10 = (locals.var_ku0we_i * ((locals.var_local_sca_dn10 + (p.p1123 * locals.var_local_scb_dn10)) + (p.p1124 * locals.var_local_scc_dn10)));
        locals.var_mu_well_dn11 = (locals.var_ku0we_i * ((locals.var_local_sca_dn11 + (p.p1123 * locals.var_local_scb_dn11)) + (p.p1124 * locals.var_local_scc_dn11)));

        let assign15920_e21830: f64 = (locals.var_u0_t * locals.var_mu_well);
        locals.var_u0_t = assign15920_e21830;
        locals.var_u0_t_dn3 = ((locals.var_u0_t_dn3 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn3));
        locals.var_u0_t_dn4 = ((locals.var_u0_t_dn4 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn4));
        locals.var_u0_t_dn5 = ((locals.var_u0_t_dn5 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn5));
        locals.var_u0_t_dn6 = ((locals.var_u0_t_dn6 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn6));
        locals.var_u0_t_dn7 = ((locals.var_u0_t_dn7 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn7));
        locals.var_u0_t_dn8 = ((locals.var_u0_t_dn8 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn8));
        locals.var_u0_t_dn9 = ((locals.var_u0_t_dn9 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn9));
        locals.var_u0_t_dn10 = ((locals.var_u0_t_dn10 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn10));
        locals.var_u0_t_dn11 = ((locals.var_u0_t_dn11 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn11));

        let assign15930_e21833: f64 = (locals.var_k2_i + locals.var_k2_well);
        locals.var_k2_i = assign15930_e21833;
        locals.var_k2_i_dn3 = (locals.var_k2_i_dn3 + locals.var_k2_well_dn3);
        locals.var_k2_i_dn4 = (locals.var_k2_i_dn4 + locals.var_k2_well_dn4);
        locals.var_k2_i_dn5 = (locals.var_k2_i_dn5 + locals.var_k2_well_dn5);
        locals.var_k2_i_dn6 = (locals.var_k2_i_dn6 + locals.var_k2_well_dn6);
        locals.var_k2_i_dn7 = (locals.var_k2_i_dn7 + locals.var_k2_well_dn7);
        locals.var_k2_i_dn8 = (locals.var_k2_i_dn8 + locals.var_k2_well_dn8);
        locals.var_k2_i_dn9 = (locals.var_k2_i_dn9 + locals.var_k2_well_dn9);
        locals.var_k2_i_dn10 = (locals.var_k2_i_dn10 + locals.var_k2_well_dn10);
        locals.var_k2_i_dn11 = (locals.var_k2_i_dn11 + locals.var_k2_well_dn11);

        let assign15940_e21836: f64 = (locals.var_k2edge_i + locals.var_k2_well_edge);
        locals.var_k2edge_i = assign15940_e21836;
        locals.var_k2edge_i_dn3 = (locals.var_k2edge_i_dn3 + locals.var_k2_well_edge_dn3);
        locals.var_k2edge_i_dn4 = (locals.var_k2edge_i_dn4 + locals.var_k2_well_edge_dn4);
        locals.var_k2edge_i_dn5 = (locals.var_k2edge_i_dn5 + locals.var_k2_well_edge_dn5);
        locals.var_k2edge_i_dn6 = (locals.var_k2edge_i_dn6 + locals.var_k2_well_edge_dn6);
        locals.var_k2edge_i_dn7 = (locals.var_k2edge_i_dn7 + locals.var_k2_well_edge_dn7);
        locals.var_k2edge_i_dn8 = (locals.var_k2edge_i_dn8 + locals.var_k2_well_edge_dn8);
        locals.var_k2edge_i_dn9 = (locals.var_k2edge_i_dn9 + locals.var_k2_well_edge_dn9);
        locals.var_k2edge_i_dn10 = (locals.var_k2edge_i_dn10 + locals.var_k2_well_edge_dn10);
        locals.var_k2edge_i_dn11 = (locals.var_k2edge_i_dn11 + locals.var_k2_well_edge_dn11);

        let assign15950_e21839: f64 = (locals.var_devsign * (nv8 - nv10));
        locals.var_vg = assign15950_e21839;
        locals.var_vg_dn8 = locals.var_devsign;
        locals.var_vg_dn10 = (-locals.var_devsign);

        let assign15960_e21842: f64 = (locals.var_devsign * (nv8 - nv11));
        locals.var_vg1 = assign15960_e21842;
        locals.var_vg1_dn8 = locals.var_devsign;
        locals.var_vg1_dn11 = (-locals.var_devsign);

        let assign15970_e21845: f64 = (locals.var_devsign * (nv6 - nv10));
        locals.var_vd = assign15970_e21845;
        locals.var_vd_dn6 = locals.var_devsign;
        locals.var_vd_dn7 = 0.0;
        locals.var_vd_dn10 = (-locals.var_devsign);

        let assign15980_e21848: f64 = (locals.var_devsign * (nv7 - nv10));
        locals.var_vs = assign15980_e21848;
        locals.var_vs_dn6 = 0.0;
        locals.var_vs_dn7 = locals.var_devsign;
        locals.var_vs_dn10 = (-locals.var_devsign);

        let assign15990_e21851: f64 = (locals.var_devsign * (nv7 - nv11));
        locals.var_vs1 = assign15990_e21851;
        locals.var_vs1_dn6 = 0.0;
        locals.var_vs1_dn7 = locals.var_devsign;
        locals.var_vs1_dn11 = (-locals.var_devsign);

        let assign16000_e21854: f64 = (locals.var_vd - locals.var_vs);
        locals.var_vds = assign16000_e21854;
        locals.var_vds_dn6 = (locals.var_vd_dn6 - locals.var_vs_dn6);
        locals.var_vds_dn7 = (locals.var_vd_dn7 - locals.var_vs_dn7);
        locals.var_vds_dn10 = (locals.var_vd_dn10 - locals.var_vs_dn10);

        locals.var_vds_noswap = locals.var_vds;
        locals.var_vds_noswap_dn6 = locals.var_vds_dn6;
        locals.var_vds_noswap_dn7 = locals.var_vds_dn7;
        locals.var_vds_noswap_dn10 = locals.var_vds_dn10;

        locals.var_vsb_noswap = locals.var_vs;
        locals.var_vsb_noswap_dn6 = locals.var_vs_dn6;
        locals.var_vsb_noswap_dn7 = locals.var_vs_dn7;
        locals.var_vsb_noswap_dn10 = locals.var_vs_dn10;

        locals.var_vdb_noswap = locals.var_vd;
        locals.var_vdb_noswap_dn6 = locals.var_vd_dn6;
        locals.var_vdb_noswap_dn7 = locals.var_vd_dn7;
        locals.var_vdb_noswap_dn10 = locals.var_vd_dn10;

        let assign16040_e21860: f64 = (locals.var_devsign * (nv10 - nv7));
        locals.var_vbs_jct = assign16040_e21860;
        locals.var_vbs_jct_dn7 = (-locals.var_devsign);
        locals.var_vbs_jct_dn10 = locals.var_devsign;

        let assign16050_e21863: f64 = (locals.var_devsign * (nv10 - nv6));
        locals.var_vbd_jct = assign16050_e21863;
        locals.var_vbd_jct_dn6 = (-locals.var_devsign);
        locals.var_vbd_jct_dn10 = locals.var_devsign;

        let assign16060_e21866: f64 = (locals.var_vg - locals.var_vd);
        locals.var_vgd_noswap = assign16060_e21866;
        locals.var_vgd_noswap_dn6 = (-locals.var_vd_dn6);
        locals.var_vgd_noswap_dn7 = (-locals.var_vd_dn7);
        locals.var_vgd_noswap_dn8 = locals.var_vg_dn8;
        locals.var_vgd_noswap_dn10 = (locals.var_vg_dn10 - locals.var_vd_dn10);

        let assign16070_e21869: f64 = (locals.var_vg - locals.var_vs);
        locals.var_vgs_noswap = assign16070_e21869;
        locals.var_vgs_noswap_dn6 = (-locals.var_vs_dn6);
        locals.var_vgs_noswap_dn7 = (-locals.var_vs_dn7);
        locals.var_vgs_noswap_dn8 = locals.var_vg_dn8;
        locals.var_vgs_noswap_dn10 = (locals.var_vg_dn10 - locals.var_vs_dn10);

        let assign16080_e21872: f64 = (locals.var_devsign * (nv9 - nv6));
        locals.var_vgd_ov_noswap = assign16080_e21872;
        locals.var_vgd_ov_noswap_dn6 = (-locals.var_devsign);
        locals.var_vgd_ov_noswap_dn9 = locals.var_devsign;

        let assign16090_e21875: f64 = (locals.var_devsign * (nv9 - nv7));
        locals.var_vgs_ov_noswap = assign16090_e21875;
        locals.var_vgs_ov_noswap_dn7 = (-locals.var_devsign);
        locals.var_vgs_ov_noswap_dn9 = locals.var_devsign;

        let assign16100_e21878: f64 = (locals.var_devsign * (nv3 - nv10));
        locals.var_ve = assign16100_e21878;
        locals.var_ve_dn3 = locals.var_devsign;
        locals.var_ve_dn10 = (-locals.var_devsign);

        let assign16110_e21881: f64 = (locals.var_devsign * (nv3 - nv11));
        locals.var_ve1 = assign16110_e21881;
        locals.var_ve1_dn3 = locals.var_devsign;
        locals.var_ve1_dn11 = (-locals.var_devsign);

        let assign16120_e21884: f64 = (locals.var_ve - locals.var_vs);
        locals.var_ves = assign16120_e21884;
        locals.var_ves_dn3 = locals.var_ve_dn3;
        locals.var_ves_dn6 = (-locals.var_vs_dn6);
        locals.var_ves_dn7 = (-locals.var_vs_dn7);
        locals.var_ves_dn10 = (locals.var_ve_dn10 - locals.var_vs_dn10);

        let assign16130_e21887: f64 = (locals.var_ve - locals.var_vs);
        locals.var_ves_1 = assign16130_e21887;
        locals.var_ves_1_dn3 = locals.var_ve_dn3;
        locals.var_ves_1_dn6 = (-locals.var_vs_dn6);
        locals.var_ves_1_dn7 = (-locals.var_vs_dn7);
        locals.var_ves_1_dn10 = (locals.var_ve_dn10 - locals.var_vs_dn10);

        let assign16140_e21890: f64 = (locals.var_ve - locals.var_vd);
        locals.var_ved = assign16140_e21890;
        locals.var_ved_dn3 = locals.var_ve_dn3;
        locals.var_ved_dn6 = (-locals.var_vd_dn6);
        locals.var_ved_dn7 = (-locals.var_vd_dn7);
        locals.var_ved_dn10 = (locals.var_ve_dn10 - locals.var_vd_dn10);

        let assign16150_e21892: f64 = (-locals.var_devsign);
        let assign16150_e21894: f64 = (assign16150_e21892 * (nv7 - nv10));
        locals.var_vbs = assign16150_e21894;
        locals.var_vbs_dn6 = 0.0;
        locals.var_vbs_dn7 = assign16150_e21892;
        locals.var_vbs_dn10 = (-assign16150_e21892);

        locals.var_sigvds = 1.0;

        let assign16170_e21898: f64 = if locals.var_vds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard485 = assign16170_e21898;

        let (assign16180_e21903,) = {
    if (locals.var_guard485 != 0.0) {
        let assign16180_e21901: f64 = (-1.0);
        (assign16180_e21901,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign16180_e21903;

        let (assign16190_e21909, assign16190_e21909_d_n6, assign16190_e21909_d_n7, assign16190_e21909_d_n10,) = {
    if (locals.var_guard485 != 0.0) {
        let assign16190_e21907: f64 = (locals.var_devsign * (nv7 - nv10));
        (assign16190_e21907, 0.0, locals.var_devsign, (-locals.var_devsign),)
    } else {
        (locals.var_vd, locals.var_vd_dn6, locals.var_vd_dn7, locals.var_vd_dn10,)
    }
};
        locals.var_vd = assign16190_e21909;
        locals.var_vd_dn6 = assign16190_e21909_d_n6;
        locals.var_vd_dn7 = assign16190_e21909_d_n7;
        locals.var_vd_dn10 = assign16190_e21909_d_n10;

        let (assign16200_e21915, assign16200_e21915_d_n6, assign16200_e21915_d_n7, assign16200_e21915_d_n10,) = {
    if (locals.var_guard485 != 0.0) {
        let assign16200_e21913: f64 = (locals.var_devsign * (nv6 - nv10));
        (assign16200_e21913, locals.var_devsign, 0.0, (-locals.var_devsign),)
    } else {
        (locals.var_vs, locals.var_vs_dn6, locals.var_vs_dn7, locals.var_vs_dn10,)
    }
};
        locals.var_vs = assign16200_e21915;
        locals.var_vs_dn6 = assign16200_e21915_d_n6;
        locals.var_vs_dn7 = assign16200_e21915_d_n7;
        locals.var_vs_dn10 = assign16200_e21915_d_n10;

    }

    pub(super) fn stamp_transient_block_28(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (assign16210_e21921, assign16210_e21921_d_n6, assign16210_e21921_d_n7, assign16210_e21921_d_n11,) = {
    if (locals.var_guard485 != 0.0) {
        let assign16210_e21919: f64 = (locals.var_devsign * (nv6 - nv11));
        (assign16210_e21919, locals.var_devsign, 0.0, (-locals.var_devsign),)
    } else {
        (locals.var_vs1, locals.var_vs1_dn6, locals.var_vs1_dn7, locals.var_vs1_dn11,)
    }
};
        locals.var_vs1 = assign16210_e21921;
        locals.var_vs1_dn6 = assign16210_e21921_d_n6;
        locals.var_vs1_dn7 = assign16210_e21921_d_n7;
        locals.var_vs1_dn11 = assign16210_e21921_d_n11;

        let (assign16220_e21927, assign16220_e21927_d_n3, assign16220_e21927_d_n6, assign16220_e21927_d_n7, assign16220_e21927_d_n10,) = {
    if (locals.var_guard485 != 0.0) {
        let assign16220_e21925: f64 = (locals.var_ve - locals.var_vs);
        (assign16220_e21925, locals.var_ve_dn3, (-locals.var_vs_dn6), (-locals.var_vs_dn7), (locals.var_ve_dn10 - locals.var_vs_dn10),)
    } else {
        (locals.var_ves, locals.var_ves_dn3, locals.var_ves_dn6, locals.var_ves_dn7, locals.var_ves_dn10,)
    }
};
        locals.var_ves = assign16220_e21927;
        locals.var_ves_dn3 = assign16220_e21927_d_n3;
        locals.var_ves_dn6 = assign16220_e21927_d_n6;
        locals.var_ves_dn7 = assign16220_e21927_d_n7;
        locals.var_ves_dn10 = assign16220_e21927_d_n10;

        let (assign16230_e21934, assign16230_e21934_d_n6, assign16230_e21934_d_n7, assign16230_e21934_d_n10,) = {
    if (locals.var_guard485 != 0.0) {
        let assign16230_e21930: f64 = (-locals.var_devsign);
        let assign16230_e21932: f64 = (assign16230_e21930 * (nv6 - nv10));
        (assign16230_e21932, assign16230_e21930, 0.0, (-assign16230_e21930),)
    } else {
        (locals.var_vbs, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10,)
    }
};
        locals.var_vbs = assign16230_e21934;
        locals.var_vbs_dn6 = assign16230_e21934_d_n6;
        locals.var_vbs_dn7 = assign16230_e21934_d_n7;
        locals.var_vbs_dn10 = assign16230_e21934_d_n10;

        let assign16240_e21937: f64 = (locals.var_vd - locals.var_vs);
        locals.var_vds = assign16240_e21937;
        locals.var_vds_dn6 = (locals.var_vd_dn6 - locals.var_vs_dn6);
        locals.var_vds_dn7 = (locals.var_vd_dn7 - locals.var_vs_dn7);
        locals.var_vds_dn10 = (locals.var_vd_dn10 - locals.var_vs_dn10);

        let assign16250_e21940: f64 = (p.p1146 * locals.var_vds);
        locals.var_t0 = assign16250_e21940;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = (p.p1146 * locals.var_vds_dn6);
        locals.var_t0_dn7 = (p.p1146 * locals.var_vds_dn7);
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = (p.p1146 * locals.var_vds_dn10);
        locals.var_t0_dn11 = 0.0;

        let assign16260_e21943: f64 = if locals.var_t0 > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard486 = assign16260_e21943;

        let (assign16270_e21947, assign16270_e21947_d_n3, assign16270_e21947_d_n4, assign16270_e21947_d_n5, assign16270_e21947_d_n6, assign16270_e21947_d_n7, assign16270_e21947_d_n8, assign16270_e21947_d_n9, assign16270_e21947_d_n10, assign16270_e21947_d_n11,) = {
    if (locals.var_guard486 != 0.0) {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign16270_e21947;
        locals.var_t1_dn3 = assign16270_e21947_d_n3;
        locals.var_t1_dn4 = assign16270_e21947_d_n4;
        locals.var_t1_dn5 = assign16270_e21947_d_n5;
        locals.var_t1_dn6 = assign16270_e21947_d_n6;
        locals.var_t1_dn7 = assign16270_e21947_d_n7;
        locals.var_t1_dn8 = assign16270_e21947_d_n8;
        locals.var_t1_dn9 = assign16270_e21947_d_n9;
        locals.var_t1_dn10 = assign16270_e21947_d_n10;
        locals.var_t1_dn11 = assign16270_e21947_d_n11;

        let (assign16280_e21956, assign16280_e21956_d_n3, assign16280_e21956_d_n4, assign16280_e21956_d_n5, assign16280_e21956_d_n6, assign16280_e21956_d_n7, assign16280_e21956_d_n8, assign16280_e21956_d_n9, assign16280_e21956_d_n10, assign16280_e21956_d_n11,) = {
    if (locals.var_guard486 == 0.0) {
        let assign16280_e21952: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign16280_e21953: f64 = (1.0 + assign16280_e21952);
        let assign16280_e21954: f64 = (assign16280_e21953).ln();
        (assign16280_e21954, (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3) / assign16280_e21953), (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) / assign16280_e21953), (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) / assign16280_e21953), (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) / assign16280_e21953), (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) / assign16280_e21953), (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) / assign16280_e21953), (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) / assign16280_e21953), (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) / assign16280_e21953), (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) / assign16280_e21953),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign16280_e21956;
        locals.var_t1_dn3 = assign16280_e21956_d_n3;
        locals.var_t1_dn4 = assign16280_e21956_d_n4;
        locals.var_t1_dn5 = assign16280_e21956_d_n5;
        locals.var_t1_dn6 = assign16280_e21956_d_n6;
        locals.var_t1_dn7 = assign16280_e21956_d_n7;
        locals.var_t1_dn8 = assign16280_e21956_d_n8;
        locals.var_t1_dn9 = assign16280_e21956_d_n9;
        locals.var_t1_dn10 = assign16280_e21956_d_n10;
        locals.var_t1_dn11 = assign16280_e21956_d_n11;

        let assign16290_e21959: f64 = (2.0 / p.p1146);
        let assign16290_e21961: f64 = (assign16290_e21959 * locals.var_t1);
        let assign16290_e21963: f64 = (assign16290_e21961 - locals.var_vds);
        let assign16290_e21966: f64 = (2.0 / p.p1146);
        let assign16290_e21968: f64 = (2.0_f64).ln();
        let assign16290_e21969: f64 = (assign16290_e21966 * assign16290_e21968);
        let assign16290_e21970: f64 = (assign16290_e21963 - assign16290_e21969);
        locals.var_vdsx = assign16290_e21970;
        locals.var_vdsx_dn3 = (assign16290_e21959 * locals.var_t1_dn3);
        locals.var_vdsx_dn4 = (assign16290_e21959 * locals.var_t1_dn4);
        locals.var_vdsx_dn5 = (assign16290_e21959 * locals.var_t1_dn5);
        locals.var_vdsx_dn6 = ((assign16290_e21959 * locals.var_t1_dn6) - locals.var_vds_dn6);
        locals.var_vdsx_dn7 = ((assign16290_e21959 * locals.var_t1_dn7) - locals.var_vds_dn7);
        locals.var_vdsx_dn8 = (assign16290_e21959 * locals.var_t1_dn8);
        locals.var_vdsx_dn9 = (assign16290_e21959 * locals.var_t1_dn9);
        locals.var_vdsx_dn10 = ((assign16290_e21959 * locals.var_t1_dn10) - locals.var_vds_dn10);
        locals.var_vdsx_dn11 = (assign16290_e21959 * locals.var_t1_dn11);

        let assign16300_e21975: f64 = (locals.var_vds - locals.var_vdsx);
        let assign16300_e21976: f64 = (0.5 * assign16300_e21975);
        let assign16300_e21977: f64 = (locals.var_vs + assign16300_e21976);
        let assign16300_e21978: f64 = (-assign16300_e21977);
        locals.var_vbsx = assign16300_e21978;
        locals.var_vbsx_dn3 = (-(0.5 * (-locals.var_vdsx_dn3)));
        locals.var_vbsx_dn4 = (-(0.5 * (-locals.var_vdsx_dn4)));
        locals.var_vbsx_dn5 = (-(0.5 * (-locals.var_vdsx_dn5)));
        locals.var_vbsx_dn6 = (-(locals.var_vs_dn6 + (0.5 * (locals.var_vds_dn6 - locals.var_vdsx_dn6))));
        locals.var_vbsx_dn7 = (-(locals.var_vs_dn7 + (0.5 * (locals.var_vds_dn7 - locals.var_vdsx_dn7))));
        locals.var_vbsx_dn8 = (-(0.5 * (-locals.var_vdsx_dn8)));
        locals.var_vbsx_dn9 = (-(0.5 * (-locals.var_vdsx_dn9)));
        locals.var_vbsx_dn10 = (-(locals.var_vs_dn10 + (0.5 * (locals.var_vds_dn10 - locals.var_vdsx_dn10))));
        locals.var_vbsx_dn11 = (-(0.5 * (-locals.var_vdsx_dn11)));

        let assign16310_e21983: f64 = (locals.var_vds - locals.var_vdsx);
        let assign16310_e21984: f64 = (0.5 * assign16310_e21983);
        let assign16310_e21985: f64 = (locals.var_vs1 + assign16310_e21984);
        let assign16310_e21986: f64 = (-assign16310_e21985);
        locals.var_vbsx1 = assign16310_e21986;
        locals.var_vbsx1_dn3 = (-(0.5 * (-locals.var_vdsx_dn3)));
        locals.var_vbsx1_dn4 = (-(0.5 * (-locals.var_vdsx_dn4)));
        locals.var_vbsx1_dn5 = (-(0.5 * (-locals.var_vdsx_dn5)));
        locals.var_vbsx1_dn6 = (-(locals.var_vs1_dn6 + (0.5 * (locals.var_vds_dn6 - locals.var_vdsx_dn6))));
        locals.var_vbsx1_dn7 = (-(locals.var_vs1_dn7 + (0.5 * (locals.var_vds_dn7 - locals.var_vdsx_dn7))));
        locals.var_vbsx1_dn8 = (-(0.5 * (-locals.var_vdsx_dn8)));
        locals.var_vbsx1_dn9 = (-(0.5 * (-locals.var_vdsx_dn9)));
        locals.var_vbsx1_dn10 = (-(0.5 * (locals.var_vds_dn10 - locals.var_vdsx_dn10)));
        locals.var_vbsx1_dn11 = (-(locals.var_vs1_dn11 + (0.5 * (-locals.var_vdsx_dn11))));

        let assign16320_e21991: f64 = (locals.var_vdsx - locals.var_vds);
        let assign16320_e21992: f64 = (0.5 * assign16320_e21991);
        let assign16320_e21993: f64 = (locals.var_ve + assign16320_e21992);
        locals.var_vesx = assign16320_e21993;
        locals.var_vesx_dn3 = (locals.var_ve_dn3 + (0.5 * locals.var_vdsx_dn3));
        locals.var_vesx_dn4 = (0.5 * locals.var_vdsx_dn4);
        locals.var_vesx_dn5 = (0.5 * locals.var_vdsx_dn5);
        locals.var_vesx_dn6 = (0.5 * (locals.var_vdsx_dn6 - locals.var_vds_dn6));
        locals.var_vesx_dn7 = (0.5 * (locals.var_vdsx_dn7 - locals.var_vds_dn7));
        locals.var_vesx_dn8 = (0.5 * locals.var_vdsx_dn8);
        locals.var_vesx_dn9 = (0.5 * locals.var_vdsx_dn9);
        locals.var_vesx_dn10 = (locals.var_ve_dn10 + (0.5 * (locals.var_vdsx_dn10 - locals.var_vds_dn10)));
        locals.var_vesx_dn11 = (0.5 * locals.var_vdsx_dn11);

        let assign16330_e21996: f64 = (0.6 * locals.var_vds_noswap);
        let assign16330_e21998: f64 = (assign16330_e21996 / locals.var_vtm);
        let assign16330_e21999: f64 = (assign16330_e21998).tanh();
        locals.var_t0 = assign16330_e21999;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = ((-((assign16330_e21996 * locals.var_vtm_dn4) / (locals.var_vtm * locals.var_vtm))) / ((assign16330_e21998).cosh() * (assign16330_e21998).cosh()));
        locals.var_t0_dn5 = ((-((assign16330_e21996 * locals.var_vtm_dn5) / (locals.var_vtm * locals.var_vtm))) / ((assign16330_e21998).cosh() * (assign16330_e21998).cosh()));
        locals.var_t0_dn6 = (((0.6 * locals.var_vds_noswap_dn6) / locals.var_vtm) / ((assign16330_e21998).cosh() * (assign16330_e21998).cosh()));
        locals.var_t0_dn7 = (((0.6 * locals.var_vds_noswap_dn7) / locals.var_vtm) / ((assign16330_e21998).cosh() * (assign16330_e21998).cosh()));
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = (((0.6 * locals.var_vds_noswap_dn10) / locals.var_vtm) / ((assign16330_e21998).cosh() * (assign16330_e21998).cosh()));
        locals.var_t0_dn11 = 0.0;

        let assign16340_e22003: f64 = (0.5 * locals.var_t0);
        let assign16340_e22004: f64 = (0.5 + assign16340_e22003);
        locals.var_wf = assign16340_e22004;
        locals.var_wf_dn3 = (0.5 * locals.var_t0_dn3);
        locals.var_wf_dn4 = (0.5 * locals.var_t0_dn4);
        locals.var_wf_dn5 = (0.5 * locals.var_t0_dn5);
        locals.var_wf_dn6 = (0.5 * locals.var_t0_dn6);
        locals.var_wf_dn7 = (0.5 * locals.var_t0_dn7);
        locals.var_wf_dn8 = (0.5 * locals.var_t0_dn8);
        locals.var_wf_dn9 = (0.5 * locals.var_t0_dn9);
        locals.var_wf_dn10 = (0.5 * locals.var_t0_dn10);
        locals.var_wf_dn11 = (0.5 * locals.var_t0_dn11);

        let assign16350_e22007: f64 = (1.0 - locals.var_wf);
        locals.var_wr = assign16350_e22007;
        locals.var_wr_dn3 = (-locals.var_wf_dn3);
        locals.var_wr_dn4 = (-locals.var_wf_dn4);
        locals.var_wr_dn5 = (-locals.var_wf_dn5);
        locals.var_wr_dn6 = (-locals.var_wf_dn6);
        locals.var_wr_dn7 = (-locals.var_wf_dn7);
        locals.var_wr_dn8 = (-locals.var_wf_dn8);
        locals.var_wr_dn9 = (-locals.var_wf_dn9);
        locals.var_wr_dn10 = (-locals.var_wf_dn10);
        locals.var_wr_dn11 = (-locals.var_wf_dn11);

        let assign16360_e22010: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard487 = assign16360_e22010;

        let (assign16370_e22020, assign16370_e22020_d_n3, assign16370_e22020_d_n4, assign16370_e22020_d_n5, assign16370_e22020_d_n6, assign16370_e22020_d_n7, assign16370_e22020_d_n8, assign16370_e22020_d_n9, assign16370_e22020_d_n10, assign16370_e22020_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16370_e22014: f64 = (locals.var_cdscdedger_i * locals.var_wr);
        let assign16370_e22017: f64 = (locals.var_cdscdedge_i * locals.var_wf);
        let assign16370_e22018: f64 = (assign16370_e22014 + assign16370_e22017);
        (assign16370_e22018, (((locals.var_cdscdedger_i_dn3 * locals.var_wr) + (locals.var_cdscdedger_i * locals.var_wr_dn3)) + (locals.var_cdscdedge_i * locals.var_wf_dn3)), (((locals.var_cdscdedger_i_dn4 * locals.var_wr) + (locals.var_cdscdedger_i * locals.var_wr_dn4)) + (locals.var_cdscdedge_i * locals.var_wf_dn4)), (((locals.var_cdscdedger_i_dn5 * locals.var_wr) + (locals.var_cdscdedger_i * locals.var_wr_dn5)) + (locals.var_cdscdedge_i * locals.var_wf_dn5)), (((locals.var_cdscdedger_i_dn6 * locals.var_wr) + (locals.var_cdscdedger_i * locals.var_wr_dn6)) + (locals.var_cdscdedge_i * locals.var_wf_dn6)), (((locals.var_cdscdedger_i_dn7 * locals.var_wr) + (locals.var_cdscdedger_i * locals.var_wr_dn7)) + (locals.var_cdscdedge_i * locals.var_wf_dn7)), (((locals.var_cdscdedger_i_dn8 * locals.var_wr) + (locals.var_cdscdedger_i * locals.var_wr_dn8)) + (locals.var_cdscdedge_i * locals.var_wf_dn8)), (((locals.var_cdscdedger_i_dn9 * locals.var_wr) + (locals.var_cdscdedger_i * locals.var_wr_dn9)) + (locals.var_cdscdedge_i * locals.var_wf_dn9)), (((locals.var_cdscdedger_i_dn10 * locals.var_wr) + (locals.var_cdscdedger_i * locals.var_wr_dn10)) + (locals.var_cdscdedge_i * locals.var_wf_dn10)), (((locals.var_cdscdedger_i_dn11 * locals.var_wr) + (locals.var_cdscdedger_i * locals.var_wr_dn11)) + (locals.var_cdscdedge_i * locals.var_wf_dn11)),)
    } else {
        (locals.var_cdscdedge_a, locals.var_cdscdedge_a_dn3, locals.var_cdscdedge_a_dn4, locals.var_cdscdedge_a_dn5, locals.var_cdscdedge_a_dn6, locals.var_cdscdedge_a_dn7, locals.var_cdscdedge_a_dn8, locals.var_cdscdedge_a_dn9, locals.var_cdscdedge_a_dn10, locals.var_cdscdedge_a_dn11,)
    }
};
        locals.var_cdscdedge_a = assign16370_e22020;
        locals.var_cdscdedge_a_dn3 = assign16370_e22020_d_n3;
        locals.var_cdscdedge_a_dn4 = assign16370_e22020_d_n4;
        locals.var_cdscdedge_a_dn5 = assign16370_e22020_d_n5;
        locals.var_cdscdedge_a_dn6 = assign16370_e22020_d_n6;
        locals.var_cdscdedge_a_dn7 = assign16370_e22020_d_n7;
        locals.var_cdscdedge_a_dn8 = assign16370_e22020_d_n8;
        locals.var_cdscdedge_a_dn9 = assign16370_e22020_d_n9;
        locals.var_cdscdedge_a_dn10 = assign16370_e22020_d_n10;
        locals.var_cdscdedge_a_dn11 = assign16370_e22020_d_n11;

        let (assign16380_e22030, assign16380_e22030_d_n3, assign16380_e22030_d_n4, assign16380_e22030_d_n5, assign16380_e22030_d_n6, assign16380_e22030_d_n7, assign16380_e22030_d_n8, assign16380_e22030_d_n9, assign16380_e22030_d_n10, assign16380_e22030_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16380_e22024: f64 = (locals.var_cdscdr_i * locals.var_wr);
        let assign16380_e22027: f64 = (locals.var_cdscd_i * locals.var_wf);
        let assign16380_e22028: f64 = (assign16380_e22024 + assign16380_e22027);
        (assign16380_e22028, (((locals.var_cdscdr_i_dn3 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn3)) + ((locals.var_cdscd_i_dn3 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn3))), (((locals.var_cdscdr_i_dn4 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn4)) + ((locals.var_cdscd_i_dn4 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn4))), (((locals.var_cdscdr_i_dn5 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn5)) + ((locals.var_cdscd_i_dn5 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn5))), (((locals.var_cdscdr_i_dn6 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn6)) + ((locals.var_cdscd_i_dn6 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn6))), (((locals.var_cdscdr_i_dn7 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn7)) + ((locals.var_cdscd_i_dn7 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn7))), (((locals.var_cdscdr_i_dn8 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn8)) + ((locals.var_cdscd_i_dn8 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn8))), (((locals.var_cdscdr_i_dn9 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn9)) + ((locals.var_cdscd_i_dn9 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn9))), (((locals.var_cdscdr_i_dn10 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn10)) + ((locals.var_cdscd_i_dn10 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn10))), (((locals.var_cdscdr_i_dn11 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn11)) + ((locals.var_cdscd_i_dn11 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn11))),)
    } else {
        (locals.var_cdscd_a, locals.var_cdscd_a_dn3, locals.var_cdscd_a_dn4, locals.var_cdscd_a_dn5, locals.var_cdscd_a_dn6, locals.var_cdscd_a_dn7, locals.var_cdscd_a_dn8, locals.var_cdscd_a_dn9, locals.var_cdscd_a_dn10, locals.var_cdscd_a_dn11,)
    }
};
        locals.var_cdscd_a = assign16380_e22030;
        locals.var_cdscd_a_dn3 = assign16380_e22030_d_n3;
        locals.var_cdscd_a_dn4 = assign16380_e22030_d_n4;
        locals.var_cdscd_a_dn5 = assign16380_e22030_d_n5;
        locals.var_cdscd_a_dn6 = assign16380_e22030_d_n6;
        locals.var_cdscd_a_dn7 = assign16380_e22030_d_n7;
        locals.var_cdscd_a_dn8 = assign16380_e22030_d_n8;
        locals.var_cdscd_a_dn9 = assign16380_e22030_d_n9;
        locals.var_cdscd_a_dn10 = assign16380_e22030_d_n10;
        locals.var_cdscd_a_dn11 = assign16380_e22030_d_n11;

        let (assign16390_e22040, assign16390_e22040_d_n3, assign16390_e22040_d_n4, assign16390_e22040_d_n5, assign16390_e22040_d_n6, assign16390_e22040_d_n7, assign16390_e22040_d_n8, assign16390_e22040_d_n9, assign16390_e22040_d_n10, assign16390_e22040_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16390_e22034: f64 = (locals.var_eta0r_t * locals.var_wr);
        let assign16390_e22037: f64 = (locals.var_eta0_t * locals.var_wf);
        let assign16390_e22038: f64 = (assign16390_e22034 + assign16390_e22037);
        (assign16390_e22038, (((locals.var_eta0r_t_dn3 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn3)) + ((locals.var_eta0_t_dn3 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn3))), (((locals.var_eta0r_t_dn4 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn4)) + ((locals.var_eta0_t_dn4 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn4))), (((locals.var_eta0r_t_dn5 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn5)) + ((locals.var_eta0_t_dn5 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn5))), (((locals.var_eta0r_t_dn6 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn6)) + ((locals.var_eta0_t_dn6 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn6))), (((locals.var_eta0r_t_dn7 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn7)) + ((locals.var_eta0_t_dn7 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn7))), (((locals.var_eta0r_t_dn8 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn8)) + ((locals.var_eta0_t_dn8 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn8))), (((locals.var_eta0r_t_dn9 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn9)) + ((locals.var_eta0_t_dn9 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn9))), (((locals.var_eta0r_t_dn10 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn10)) + ((locals.var_eta0_t_dn10 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn10))), (((locals.var_eta0r_t_dn11 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn11)) + ((locals.var_eta0_t_dn11 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn11))),)
    } else {
        (locals.var_eta0_a, locals.var_eta0_a_dn3, locals.var_eta0_a_dn4, locals.var_eta0_a_dn5, locals.var_eta0_a_dn6, locals.var_eta0_a_dn7, locals.var_eta0_a_dn8, locals.var_eta0_a_dn9, locals.var_eta0_a_dn10, locals.var_eta0_a_dn11,)
    }
};
        locals.var_eta0_a = assign16390_e22040;
        locals.var_eta0_a_dn3 = assign16390_e22040_d_n3;
        locals.var_eta0_a_dn4 = assign16390_e22040_d_n4;
        locals.var_eta0_a_dn5 = assign16390_e22040_d_n5;
        locals.var_eta0_a_dn6 = assign16390_e22040_d_n6;
        locals.var_eta0_a_dn7 = assign16390_e22040_d_n7;
        locals.var_eta0_a_dn8 = assign16390_e22040_d_n8;
        locals.var_eta0_a_dn9 = assign16390_e22040_d_n9;
        locals.var_eta0_a_dn10 = assign16390_e22040_d_n10;
        locals.var_eta0_a_dn11 = assign16390_e22040_d_n11;

        let (assign16400_e22050, assign16400_e22050_d_n3, assign16400_e22050_d_n4, assign16400_e22050_d_n5, assign16400_e22050_d_n6, assign16400_e22050_d_n7, assign16400_e22050_d_n8, assign16400_e22050_d_n9, assign16400_e22050_d_n10, assign16400_e22050_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16400_e22044: f64 = (locals.var_pdiblcr_i * locals.var_wr);
        let assign16400_e22047: f64 = (locals.var_pdiblc_i * locals.var_wf);
        let assign16400_e22048: f64 = (assign16400_e22044 + assign16400_e22047);
        (assign16400_e22048, (((locals.var_pdiblcr_i_dn3 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn3)) + ((locals.var_pdiblc_i_dn3 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn3))), (((locals.var_pdiblcr_i_dn4 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn4)) + ((locals.var_pdiblc_i_dn4 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn4))), (((locals.var_pdiblcr_i_dn5 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn5)) + ((locals.var_pdiblc_i_dn5 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn5))), (((locals.var_pdiblcr_i_dn6 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn6)) + ((locals.var_pdiblc_i_dn6 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn6))), (((locals.var_pdiblcr_i_dn7 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn7)) + ((locals.var_pdiblc_i_dn7 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn7))), (((locals.var_pdiblcr_i_dn8 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn8)) + ((locals.var_pdiblc_i_dn8 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn8))), (((locals.var_pdiblcr_i_dn9 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn9)) + ((locals.var_pdiblc_i_dn9 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn9))), (((locals.var_pdiblcr_i_dn10 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn10)) + ((locals.var_pdiblc_i_dn10 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn10))), (((locals.var_pdiblcr_i_dn11 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn11)) + ((locals.var_pdiblc_i_dn11 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn11))),)
    } else {
        (locals.var_pdiblc_a, locals.var_pdiblc_a_dn3, locals.var_pdiblc_a_dn4, locals.var_pdiblc_a_dn5, locals.var_pdiblc_a_dn6, locals.var_pdiblc_a_dn7, locals.var_pdiblc_a_dn8, locals.var_pdiblc_a_dn9, locals.var_pdiblc_a_dn10, locals.var_pdiblc_a_dn11,)
    }
};
        locals.var_pdiblc_a = assign16400_e22050;
        locals.var_pdiblc_a_dn3 = assign16400_e22050_d_n3;
        locals.var_pdiblc_a_dn4 = assign16400_e22050_d_n4;
        locals.var_pdiblc_a_dn5 = assign16400_e22050_d_n5;
        locals.var_pdiblc_a_dn6 = assign16400_e22050_d_n6;
        locals.var_pdiblc_a_dn7 = assign16400_e22050_d_n7;
        locals.var_pdiblc_a_dn8 = assign16400_e22050_d_n8;
        locals.var_pdiblc_a_dn9 = assign16400_e22050_d_n9;
        locals.var_pdiblc_a_dn10 = assign16400_e22050_d_n10;
        locals.var_pdiblc_a_dn11 = assign16400_e22050_d_n11;

        let (assign16410_e22060, assign16410_e22060_d_n3, assign16410_e22060_d_n4, assign16410_e22060_d_n5, assign16410_e22060_d_n6, assign16410_e22060_d_n7, assign16410_e22060_d_n8, assign16410_e22060_d_n9, assign16410_e22060_d_n10, assign16410_e22060_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16410_e22054: f64 = (locals.var_pclmr_i * locals.var_wr);
        let assign16410_e22057: f64 = (locals.var_pclm_i * locals.var_wf);
        let assign16410_e22058: f64 = (assign16410_e22054 + assign16410_e22057);
        (assign16410_e22058, (((locals.var_pclmr_i_dn3 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn3)) + ((locals.var_pclm_i_dn3 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn3))), (((locals.var_pclmr_i_dn4 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn4)) + ((locals.var_pclm_i_dn4 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn4))), (((locals.var_pclmr_i_dn5 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn5)) + ((locals.var_pclm_i_dn5 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn5))), (((locals.var_pclmr_i_dn6 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn6)) + ((locals.var_pclm_i_dn6 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn6))), (((locals.var_pclmr_i_dn7 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn7)) + ((locals.var_pclm_i_dn7 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn7))), (((locals.var_pclmr_i_dn8 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn8)) + ((locals.var_pclm_i_dn8 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn8))), (((locals.var_pclmr_i_dn9 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn9)) + ((locals.var_pclm_i_dn9 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn9))), (((locals.var_pclmr_i_dn10 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn10)) + ((locals.var_pclm_i_dn10 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn10))), (((locals.var_pclmr_i_dn11 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn11)) + ((locals.var_pclm_i_dn11 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn11))),)
    } else {
        (locals.var_pclm_a, locals.var_pclm_a_dn3, locals.var_pclm_a_dn4, locals.var_pclm_a_dn5, locals.var_pclm_a_dn6, locals.var_pclm_a_dn7, locals.var_pclm_a_dn8, locals.var_pclm_a_dn9, locals.var_pclm_a_dn10, locals.var_pclm_a_dn11,)
    }
};
        locals.var_pclm_a = assign16410_e22060;
        locals.var_pclm_a_dn3 = assign16410_e22060_d_n3;
        locals.var_pclm_a_dn4 = assign16410_e22060_d_n4;
        locals.var_pclm_a_dn5 = assign16410_e22060_d_n5;
        locals.var_pclm_a_dn6 = assign16410_e22060_d_n6;
        locals.var_pclm_a_dn7 = assign16410_e22060_d_n7;
        locals.var_pclm_a_dn8 = assign16410_e22060_d_n8;
        locals.var_pclm_a_dn9 = assign16410_e22060_d_n9;
        locals.var_pclm_a_dn10 = assign16410_e22060_d_n10;
        locals.var_pclm_a_dn11 = assign16410_e22060_d_n11;

        let (assign16420_e22070, assign16420_e22070_d_n3, assign16420_e22070_d_n4, assign16420_e22070_d_n5, assign16420_e22070_d_n6, assign16420_e22070_d_n7, assign16420_e22070_d_n8, assign16420_e22070_d_n9, assign16420_e22070_d_n10, assign16420_e22070_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16420_e22064: f64 = (locals.var_psatr_i * locals.var_wr);
        let assign16420_e22067: f64 = (locals.var_psat_i * locals.var_wf);
        let assign16420_e22068: f64 = (assign16420_e22064 + assign16420_e22067);
        (assign16420_e22068, ((locals.var_psatr_i * locals.var_wr_dn3) + (locals.var_psat_i * locals.var_wf_dn3)), ((locals.var_psatr_i * locals.var_wr_dn4) + (locals.var_psat_i * locals.var_wf_dn4)), ((locals.var_psatr_i * locals.var_wr_dn5) + (locals.var_psat_i * locals.var_wf_dn5)), ((locals.var_psatr_i * locals.var_wr_dn6) + (locals.var_psat_i * locals.var_wf_dn6)), ((locals.var_psatr_i * locals.var_wr_dn7) + (locals.var_psat_i * locals.var_wf_dn7)), ((locals.var_psatr_i * locals.var_wr_dn8) + (locals.var_psat_i * locals.var_wf_dn8)), ((locals.var_psatr_i * locals.var_wr_dn9) + (locals.var_psat_i * locals.var_wf_dn9)), ((locals.var_psatr_i * locals.var_wr_dn10) + (locals.var_psat_i * locals.var_wf_dn10)), ((locals.var_psatr_i * locals.var_wr_dn11) + (locals.var_psat_i * locals.var_wf_dn11)),)
    } else {
        (locals.var_psat_a, locals.var_psat_a_dn3, locals.var_psat_a_dn4, locals.var_psat_a_dn5, locals.var_psat_a_dn6, locals.var_psat_a_dn7, locals.var_psat_a_dn8, locals.var_psat_a_dn9, locals.var_psat_a_dn10, locals.var_psat_a_dn11,)
    }
};
        locals.var_psat_a = assign16420_e22070;
        locals.var_psat_a_dn3 = assign16420_e22070_d_n3;
        locals.var_psat_a_dn4 = assign16420_e22070_d_n4;
        locals.var_psat_a_dn5 = assign16420_e22070_d_n5;
        locals.var_psat_a_dn6 = assign16420_e22070_d_n6;
        locals.var_psat_a_dn7 = assign16420_e22070_d_n7;
        locals.var_psat_a_dn8 = assign16420_e22070_d_n8;
        locals.var_psat_a_dn9 = assign16420_e22070_d_n9;
        locals.var_psat_a_dn10 = assign16420_e22070_d_n10;
        locals.var_psat_a_dn11 = assign16420_e22070_d_n11;

        let (assign16430_e22080, assign16430_e22080_d_n3, assign16430_e22080_d_n4, assign16430_e22080_d_n5, assign16430_e22080_d_n6, assign16430_e22080_d_n7, assign16430_e22080_d_n8, assign16430_e22080_d_n9, assign16430_e22080_d_n10, assign16430_e22080_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16430_e22074: f64 = (locals.var_vsatr_t * locals.var_wr);
        let assign16430_e22077: f64 = (locals.var_vsat_t * locals.var_wf);
        let assign16430_e22078: f64 = (assign16430_e22074 + assign16430_e22077);
        (assign16430_e22078, (((locals.var_vsatr_t_dn3 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn3)) + ((locals.var_vsat_t_dn3 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn3))), (((locals.var_vsatr_t_dn4 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn4)) + ((locals.var_vsat_t_dn4 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn4))), (((locals.var_vsatr_t_dn5 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn5)) + ((locals.var_vsat_t_dn5 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn5))), (((locals.var_vsatr_t_dn6 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn6)) + ((locals.var_vsat_t_dn6 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn6))), (((locals.var_vsatr_t_dn7 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn7)) + ((locals.var_vsat_t_dn7 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn7))), (((locals.var_vsatr_t_dn8 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn8)) + ((locals.var_vsat_t_dn8 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn8))), (((locals.var_vsatr_t_dn9 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn9)) + ((locals.var_vsat_t_dn9 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn9))), (((locals.var_vsatr_t_dn10 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn10)) + ((locals.var_vsat_t_dn10 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn10))), (((locals.var_vsatr_t_dn11 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn11)) + ((locals.var_vsat_t_dn11 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn11))),)
    } else {
        (locals.var_vsat_a, locals.var_vsat_a_dn3, locals.var_vsat_a_dn4, locals.var_vsat_a_dn5, locals.var_vsat_a_dn6, locals.var_vsat_a_dn7, locals.var_vsat_a_dn8, locals.var_vsat_a_dn9, locals.var_vsat_a_dn10, locals.var_vsat_a_dn11,)
    }
};
        locals.var_vsat_a = assign16430_e22080;
        locals.var_vsat_a_dn3 = assign16430_e22080_d_n3;
        locals.var_vsat_a_dn4 = assign16430_e22080_d_n4;
        locals.var_vsat_a_dn5 = assign16430_e22080_d_n5;
        locals.var_vsat_a_dn6 = assign16430_e22080_d_n6;
        locals.var_vsat_a_dn7 = assign16430_e22080_d_n7;
        locals.var_vsat_a_dn8 = assign16430_e22080_d_n8;
        locals.var_vsat_a_dn9 = assign16430_e22080_d_n9;
        locals.var_vsat_a_dn10 = assign16430_e22080_d_n10;
        locals.var_vsat_a_dn11 = assign16430_e22080_d_n11;

        let (assign16440_e22090, assign16440_e22090_d_n3, assign16440_e22090_d_n4, assign16440_e22090_d_n5, assign16440_e22090_d_n6, assign16440_e22090_d_n7, assign16440_e22090_d_n8, assign16440_e22090_d_n9, assign16440_e22090_d_n10, assign16440_e22090_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16440_e22084: f64 = (locals.var_ptwgr_t * locals.var_wr);
        let assign16440_e22087: f64 = (locals.var_ptwg_t * locals.var_wf);
        let assign16440_e22088: f64 = (assign16440_e22084 + assign16440_e22087);
        (assign16440_e22088, (((locals.var_ptwgr_t_dn3 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn3)) + ((locals.var_ptwg_t_dn3 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn3))), (((locals.var_ptwgr_t_dn4 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn4)) + ((locals.var_ptwg_t_dn4 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn4))), (((locals.var_ptwgr_t_dn5 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn5)) + ((locals.var_ptwg_t_dn5 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn5))), (((locals.var_ptwgr_t_dn6 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn6)) + ((locals.var_ptwg_t_dn6 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn6))), (((locals.var_ptwgr_t_dn7 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn7)) + ((locals.var_ptwg_t_dn7 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn7))), (((locals.var_ptwgr_t_dn8 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn8)) + ((locals.var_ptwg_t_dn8 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn8))), (((locals.var_ptwgr_t_dn9 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn9)) + ((locals.var_ptwg_t_dn9 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn9))), (((locals.var_ptwgr_t_dn10 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn10)) + ((locals.var_ptwg_t_dn10 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn10))), (((locals.var_ptwgr_t_dn11 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn11)) + ((locals.var_ptwg_t_dn11 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn11))),)
    } else {
        (locals.var_ptwg_a, locals.var_ptwg_a_dn3, locals.var_ptwg_a_dn4, locals.var_ptwg_a_dn5, locals.var_ptwg_a_dn6, locals.var_ptwg_a_dn7, locals.var_ptwg_a_dn8, locals.var_ptwg_a_dn9, locals.var_ptwg_a_dn10, locals.var_ptwg_a_dn11,)
    }
};
        locals.var_ptwg_a = assign16440_e22090;
        locals.var_ptwg_a_dn3 = assign16440_e22090_d_n3;
        locals.var_ptwg_a_dn4 = assign16440_e22090_d_n4;
        locals.var_ptwg_a_dn5 = assign16440_e22090_d_n5;
        locals.var_ptwg_a_dn6 = assign16440_e22090_d_n6;
        locals.var_ptwg_a_dn7 = assign16440_e22090_d_n7;
        locals.var_ptwg_a_dn8 = assign16440_e22090_d_n8;
        locals.var_ptwg_a_dn9 = assign16440_e22090_d_n9;
        locals.var_ptwg_a_dn10 = assign16440_e22090_d_n10;
        locals.var_ptwg_a_dn11 = assign16440_e22090_d_n11;

        let (assign16450_e22100, assign16450_e22100_d_n3, assign16450_e22100_d_n4, assign16450_e22100_d_n5, assign16450_e22100_d_n6, assign16450_e22100_d_n7, assign16450_e22100_d_n8, assign16450_e22100_d_n9, assign16450_e22100_d_n10, assign16450_e22100_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16450_e22094: f64 = (locals.var_u0r_t * locals.var_wr);
        let assign16450_e22097: f64 = (locals.var_u0_t * locals.var_wf);
        let assign16450_e22098: f64 = (assign16450_e22094 + assign16450_e22097);
        (assign16450_e22098, ((locals.var_u0r_t * locals.var_wr_dn3) + ((locals.var_u0_t_dn3 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn3))), (((locals.var_u0r_t_dn4 * locals.var_wr) + (locals.var_u0r_t * locals.var_wr_dn4)) + ((locals.var_u0_t_dn4 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn4))), (((locals.var_u0r_t_dn5 * locals.var_wr) + (locals.var_u0r_t * locals.var_wr_dn5)) + ((locals.var_u0_t_dn5 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn5))), ((locals.var_u0r_t * locals.var_wr_dn6) + ((locals.var_u0_t_dn6 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn6))), ((locals.var_u0r_t * locals.var_wr_dn7) + ((locals.var_u0_t_dn7 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn7))), ((locals.var_u0r_t * locals.var_wr_dn8) + ((locals.var_u0_t_dn8 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn8))), ((locals.var_u0r_t * locals.var_wr_dn9) + ((locals.var_u0_t_dn9 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn9))), ((locals.var_u0r_t * locals.var_wr_dn10) + ((locals.var_u0_t_dn10 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn10))), ((locals.var_u0r_t * locals.var_wr_dn11) + ((locals.var_u0_t_dn11 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn11))),)
    } else {
        (locals.var_u0_a, locals.var_u0_a_dn3, locals.var_u0_a_dn4, locals.var_u0_a_dn5, locals.var_u0_a_dn6, locals.var_u0_a_dn7, locals.var_u0_a_dn8, locals.var_u0_a_dn9, locals.var_u0_a_dn10, locals.var_u0_a_dn11,)
    }
};
        locals.var_u0_a = assign16450_e22100;
        locals.var_u0_a_dn3 = assign16450_e22100_d_n3;
        locals.var_u0_a_dn4 = assign16450_e22100_d_n4;
        locals.var_u0_a_dn5 = assign16450_e22100_d_n5;
        locals.var_u0_a_dn6 = assign16450_e22100_d_n6;
        locals.var_u0_a_dn7 = assign16450_e22100_d_n7;
        locals.var_u0_a_dn8 = assign16450_e22100_d_n8;
        locals.var_u0_a_dn9 = assign16450_e22100_d_n9;
        locals.var_u0_a_dn10 = assign16450_e22100_d_n10;
        locals.var_u0_a_dn11 = assign16450_e22100_d_n11;

        let (assign16460_e22110, assign16460_e22110_d_n3, assign16460_e22110_d_n4, assign16460_e22110_d_n5, assign16460_e22110_d_n6, assign16460_e22110_d_n7, assign16460_e22110_d_n8, assign16460_e22110_d_n9, assign16460_e22110_d_n10, assign16460_e22110_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16460_e22104: f64 = (locals.var_uar_t * locals.var_wr);
        let assign16460_e22107: f64 = (locals.var_ua_t * locals.var_wf);
        let assign16460_e22108: f64 = (assign16460_e22104 + assign16460_e22107);
        (assign16460_e22108, (((locals.var_uar_t_dn3 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn3)) + ((locals.var_ua_t_dn3 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn3))), (((locals.var_uar_t_dn4 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn4)) + ((locals.var_ua_t_dn4 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn4))), (((locals.var_uar_t_dn5 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn5)) + ((locals.var_ua_t_dn5 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn5))), (((locals.var_uar_t_dn6 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn6)) + ((locals.var_ua_t_dn6 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn6))), (((locals.var_uar_t_dn7 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn7)) + ((locals.var_ua_t_dn7 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn7))), (((locals.var_uar_t_dn8 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn8)) + ((locals.var_ua_t_dn8 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn8))), (((locals.var_uar_t_dn9 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn9)) + ((locals.var_ua_t_dn9 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn9))), (((locals.var_uar_t_dn10 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn10)) + ((locals.var_ua_t_dn10 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn10))), (((locals.var_uar_t_dn11 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn11)) + ((locals.var_ua_t_dn11 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn11))),)
    } else {
        (locals.var_ua_a, locals.var_ua_a_dn3, locals.var_ua_a_dn4, locals.var_ua_a_dn5, locals.var_ua_a_dn6, locals.var_ua_a_dn7, locals.var_ua_a_dn8, locals.var_ua_a_dn9, locals.var_ua_a_dn10, locals.var_ua_a_dn11,)
    }
};
        locals.var_ua_a = assign16460_e22110;
        locals.var_ua_a_dn3 = assign16460_e22110_d_n3;
        locals.var_ua_a_dn4 = assign16460_e22110_d_n4;
        locals.var_ua_a_dn5 = assign16460_e22110_d_n5;
        locals.var_ua_a_dn6 = assign16460_e22110_d_n6;
        locals.var_ua_a_dn7 = assign16460_e22110_d_n7;
        locals.var_ua_a_dn8 = assign16460_e22110_d_n8;
        locals.var_ua_a_dn9 = assign16460_e22110_d_n9;
        locals.var_ua_a_dn10 = assign16460_e22110_d_n10;
        locals.var_ua_a_dn11 = assign16460_e22110_d_n11;

        let (assign16470_e22120, assign16470_e22120_d_n3, assign16470_e22120_d_n4, assign16470_e22120_d_n5, assign16470_e22120_d_n6, assign16470_e22120_d_n7, assign16470_e22120_d_n8, assign16470_e22120_d_n9, assign16470_e22120_d_n10, assign16470_e22120_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16470_e22114: f64 = (locals.var_ucr_t * locals.var_wr);
        let assign16470_e22117: f64 = (locals.var_uc_t * locals.var_wf);
        let assign16470_e22118: f64 = (assign16470_e22114 + assign16470_e22117);
        (assign16470_e22118, (((locals.var_ucr_t_dn3 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn3)) + ((locals.var_uc_t_dn3 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn3))), (((locals.var_ucr_t_dn4 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn4)) + ((locals.var_uc_t_dn4 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn4))), (((locals.var_ucr_t_dn5 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn5)) + ((locals.var_uc_t_dn5 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn5))), (((locals.var_ucr_t_dn6 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn6)) + ((locals.var_uc_t_dn6 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn6))), (((locals.var_ucr_t_dn7 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn7)) + ((locals.var_uc_t_dn7 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn7))), (((locals.var_ucr_t_dn8 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn8)) + ((locals.var_uc_t_dn8 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn8))), (((locals.var_ucr_t_dn9 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn9)) + ((locals.var_uc_t_dn9 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn9))), (((locals.var_ucr_t_dn10 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn10)) + ((locals.var_uc_t_dn10 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn10))), (((locals.var_ucr_t_dn11 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn11)) + ((locals.var_uc_t_dn11 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn11))),)
    } else {
        (locals.var_uc_a, locals.var_uc_a_dn3, locals.var_uc_a_dn4, locals.var_uc_a_dn5, locals.var_uc_a_dn6, locals.var_uc_a_dn7, locals.var_uc_a_dn8, locals.var_uc_a_dn9, locals.var_uc_a_dn10, locals.var_uc_a_dn11,)
    }
};
        locals.var_uc_a = assign16470_e22120;
        locals.var_uc_a_dn3 = assign16470_e22120_d_n3;
        locals.var_uc_a_dn4 = assign16470_e22120_d_n4;
        locals.var_uc_a_dn5 = assign16470_e22120_d_n5;
        locals.var_uc_a_dn6 = assign16470_e22120_d_n6;
        locals.var_uc_a_dn7 = assign16470_e22120_d_n7;
        locals.var_uc_a_dn8 = assign16470_e22120_d_n8;
        locals.var_uc_a_dn9 = assign16470_e22120_d_n9;
        locals.var_uc_a_dn10 = assign16470_e22120_d_n10;
        locals.var_uc_a_dn11 = assign16470_e22120_d_n11;

        let (assign16480_e22130, assign16480_e22130_d_n3, assign16480_e22130_d_n4, assign16480_e22130_d_n5, assign16480_e22130_d_n6, assign16480_e22130_d_n7, assign16480_e22130_d_n8, assign16480_e22130_d_n9, assign16480_e22130_d_n10, assign16480_e22130_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16480_e22124: f64 = (locals.var_udr_t * locals.var_wr);
        let assign16480_e22127: f64 = (locals.var_ud_t * locals.var_wf);
        let assign16480_e22128: f64 = (assign16480_e22124 + assign16480_e22127);
        (assign16480_e22128, (((locals.var_udr_t_dn3 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn3)) + ((locals.var_ud_t_dn3 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn3))), (((locals.var_udr_t_dn4 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn4)) + ((locals.var_ud_t_dn4 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn4))), (((locals.var_udr_t_dn5 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn5)) + ((locals.var_ud_t_dn5 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn5))), (((locals.var_udr_t_dn6 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn6)) + ((locals.var_ud_t_dn6 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn6))), (((locals.var_udr_t_dn7 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn7)) + ((locals.var_ud_t_dn7 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn7))), (((locals.var_udr_t_dn8 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn8)) + ((locals.var_ud_t_dn8 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn8))), (((locals.var_udr_t_dn9 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn9)) + ((locals.var_ud_t_dn9 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn9))), (((locals.var_udr_t_dn10 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn10)) + ((locals.var_ud_t_dn10 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn10))), (((locals.var_udr_t_dn11 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn11)) + ((locals.var_ud_t_dn11 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn11))),)
    } else {
        (locals.var_ud_a, locals.var_ud_a_dn3, locals.var_ud_a_dn4, locals.var_ud_a_dn5, locals.var_ud_a_dn6, locals.var_ud_a_dn7, locals.var_ud_a_dn8, locals.var_ud_a_dn9, locals.var_ud_a_dn10, locals.var_ud_a_dn11,)
    }
};
        locals.var_ud_a = assign16480_e22130;
        locals.var_ud_a_dn3 = assign16480_e22130_d_n3;
        locals.var_ud_a_dn4 = assign16480_e22130_d_n4;
        locals.var_ud_a_dn5 = assign16480_e22130_d_n5;
        locals.var_ud_a_dn6 = assign16480_e22130_d_n6;
        locals.var_ud_a_dn7 = assign16480_e22130_d_n7;
        locals.var_ud_a_dn8 = assign16480_e22130_d_n8;
        locals.var_ud_a_dn9 = assign16480_e22130_d_n9;
        locals.var_ud_a_dn10 = assign16480_e22130_d_n10;
        locals.var_ud_a_dn11 = assign16480_e22130_d_n11;

        let (assign16490_e22140, assign16490_e22140_d_n3, assign16490_e22140_d_n4, assign16490_e22140_d_n5, assign16490_e22140_d_n6, assign16490_e22140_d_n7, assign16490_e22140_d_n8, assign16490_e22140_d_n9, assign16490_e22140_d_n10, assign16490_e22140_d_n11,) = {
    if (locals.var_guard487 != 0.0) {
        let assign16490_e22134: f64 = (locals.var_ucsr_t * locals.var_wr);
        let assign16490_e22137: f64 = (locals.var_ucs_t * locals.var_wf);
        let assign16490_e22138: f64 = (assign16490_e22134 + assign16490_e22137);
        (assign16490_e22138, ((locals.var_ucsr_t * locals.var_wr_dn3) + (locals.var_ucs_t * locals.var_wf_dn3)), (((locals.var_ucsr_t_dn4 * locals.var_wr) + (locals.var_ucsr_t * locals.var_wr_dn4)) + ((locals.var_ucs_t_dn4 * locals.var_wf) + (locals.var_ucs_t * locals.var_wf_dn4))), (((locals.var_ucsr_t_dn5 * locals.var_wr) + (locals.var_ucsr_t * locals.var_wr_dn5)) + ((locals.var_ucs_t_dn5 * locals.var_wf) + (locals.var_ucs_t * locals.var_wf_dn5))), ((locals.var_ucsr_t * locals.var_wr_dn6) + (locals.var_ucs_t * locals.var_wf_dn6)), ((locals.var_ucsr_t * locals.var_wr_dn7) + (locals.var_ucs_t * locals.var_wf_dn7)), ((locals.var_ucsr_t * locals.var_wr_dn8) + (locals.var_ucs_t * locals.var_wf_dn8)), ((locals.var_ucsr_t * locals.var_wr_dn9) + (locals.var_ucs_t * locals.var_wf_dn9)), ((locals.var_ucsr_t * locals.var_wr_dn10) + (locals.var_ucs_t * locals.var_wf_dn10)), ((locals.var_ucsr_t * locals.var_wr_dn11) + (locals.var_ucs_t * locals.var_wf_dn11)),)
    } else {
        (locals.var_ucs_a, locals.var_ucs_a_dn3, locals.var_ucs_a_dn4, locals.var_ucs_a_dn5, locals.var_ucs_a_dn6, locals.var_ucs_a_dn7, locals.var_ucs_a_dn8, locals.var_ucs_a_dn9, locals.var_ucs_a_dn10, locals.var_ucs_a_dn11,)
    }
};
        locals.var_ucs_a = assign16490_e22140;
        locals.var_ucs_a_dn3 = assign16490_e22140_d_n3;
        locals.var_ucs_a_dn4 = assign16490_e22140_d_n4;
        locals.var_ucs_a_dn5 = assign16490_e22140_d_n5;
        locals.var_ucs_a_dn6 = assign16490_e22140_d_n6;
        locals.var_ucs_a_dn7 = assign16490_e22140_d_n7;
        locals.var_ucs_a_dn8 = assign16490_e22140_d_n8;
        locals.var_ucs_a_dn9 = assign16490_e22140_d_n9;
        locals.var_ucs_a_dn10 = assign16490_e22140_d_n10;
        locals.var_ucs_a_dn11 = assign16490_e22140_d_n11;

        let (assign16500_e22145, assign16500_e22145_d_n3, assign16500_e22145_d_n4, assign16500_e22145_d_n5, assign16500_e22145_d_n6, assign16500_e22145_d_n7, assign16500_e22145_d_n8, assign16500_e22145_d_n9, assign16500_e22145_d_n10, assign16500_e22145_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_cdscdedge_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cdscdedge_a, locals.var_cdscdedge_a_dn3, locals.var_cdscdedge_a_dn4, locals.var_cdscdedge_a_dn5, locals.var_cdscdedge_a_dn6, locals.var_cdscdedge_a_dn7, locals.var_cdscdedge_a_dn8, locals.var_cdscdedge_a_dn9, locals.var_cdscdedge_a_dn10, locals.var_cdscdedge_a_dn11,)
    }
};
        locals.var_cdscdedge_a = assign16500_e22145;
        locals.var_cdscdedge_a_dn3 = assign16500_e22145_d_n3;
        locals.var_cdscdedge_a_dn4 = assign16500_e22145_d_n4;
        locals.var_cdscdedge_a_dn5 = assign16500_e22145_d_n5;
        locals.var_cdscdedge_a_dn6 = assign16500_e22145_d_n6;
        locals.var_cdscdedge_a_dn7 = assign16500_e22145_d_n7;
        locals.var_cdscdedge_a_dn8 = assign16500_e22145_d_n8;
        locals.var_cdscdedge_a_dn9 = assign16500_e22145_d_n9;
        locals.var_cdscdedge_a_dn10 = assign16500_e22145_d_n10;
        locals.var_cdscdedge_a_dn11 = assign16500_e22145_d_n11;

    }

    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16510_e22150, assign16510_e22150_d_n3, assign16510_e22150_d_n4, assign16510_e22150_d_n5, assign16510_e22150_d_n6, assign16510_e22150_d_n7, assign16510_e22150_d_n8, assign16510_e22150_d_n9, assign16510_e22150_d_n10, assign16510_e22150_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_cdscd_i, locals.var_cdscd_i_dn3, locals.var_cdscd_i_dn4, locals.var_cdscd_i_dn5, locals.var_cdscd_i_dn6, locals.var_cdscd_i_dn7, locals.var_cdscd_i_dn8, locals.var_cdscd_i_dn9, locals.var_cdscd_i_dn10, locals.var_cdscd_i_dn11,)
    } else {
        (locals.var_cdscd_a, locals.var_cdscd_a_dn3, locals.var_cdscd_a_dn4, locals.var_cdscd_a_dn5, locals.var_cdscd_a_dn6, locals.var_cdscd_a_dn7, locals.var_cdscd_a_dn8, locals.var_cdscd_a_dn9, locals.var_cdscd_a_dn10, locals.var_cdscd_a_dn11,)
    }
};
        locals.var_cdscd_a = assign16510_e22150;
        locals.var_cdscd_a_dn3 = assign16510_e22150_d_n3;
        locals.var_cdscd_a_dn4 = assign16510_e22150_d_n4;
        locals.var_cdscd_a_dn5 = assign16510_e22150_d_n5;
        locals.var_cdscd_a_dn6 = assign16510_e22150_d_n6;
        locals.var_cdscd_a_dn7 = assign16510_e22150_d_n7;
        locals.var_cdscd_a_dn8 = assign16510_e22150_d_n8;
        locals.var_cdscd_a_dn9 = assign16510_e22150_d_n9;
        locals.var_cdscd_a_dn10 = assign16510_e22150_d_n10;
        locals.var_cdscd_a_dn11 = assign16510_e22150_d_n11;

        let (assign16520_e22155, assign16520_e22155_d_n3, assign16520_e22155_d_n4, assign16520_e22155_d_n5, assign16520_e22155_d_n6, assign16520_e22155_d_n7, assign16520_e22155_d_n8, assign16520_e22155_d_n9, assign16520_e22155_d_n10, assign16520_e22155_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_eta0_t, locals.var_eta0_t_dn3, locals.var_eta0_t_dn4, locals.var_eta0_t_dn5, locals.var_eta0_t_dn6, locals.var_eta0_t_dn7, locals.var_eta0_t_dn8, locals.var_eta0_t_dn9, locals.var_eta0_t_dn10, locals.var_eta0_t_dn11,)
    } else {
        (locals.var_eta0_a, locals.var_eta0_a_dn3, locals.var_eta0_a_dn4, locals.var_eta0_a_dn5, locals.var_eta0_a_dn6, locals.var_eta0_a_dn7, locals.var_eta0_a_dn8, locals.var_eta0_a_dn9, locals.var_eta0_a_dn10, locals.var_eta0_a_dn11,)
    }
};
        locals.var_eta0_a = assign16520_e22155;
        locals.var_eta0_a_dn3 = assign16520_e22155_d_n3;
        locals.var_eta0_a_dn4 = assign16520_e22155_d_n4;
        locals.var_eta0_a_dn5 = assign16520_e22155_d_n5;
        locals.var_eta0_a_dn6 = assign16520_e22155_d_n6;
        locals.var_eta0_a_dn7 = assign16520_e22155_d_n7;
        locals.var_eta0_a_dn8 = assign16520_e22155_d_n8;
        locals.var_eta0_a_dn9 = assign16520_e22155_d_n9;
        locals.var_eta0_a_dn10 = assign16520_e22155_d_n10;
        locals.var_eta0_a_dn11 = assign16520_e22155_d_n11;

        let (assign16530_e22160, assign16530_e22160_d_n3, assign16530_e22160_d_n4, assign16530_e22160_d_n5, assign16530_e22160_d_n6, assign16530_e22160_d_n7, assign16530_e22160_d_n8, assign16530_e22160_d_n9, assign16530_e22160_d_n10, assign16530_e22160_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_pdiblc_i, locals.var_pdiblc_i_dn3, locals.var_pdiblc_i_dn4, locals.var_pdiblc_i_dn5, locals.var_pdiblc_i_dn6, locals.var_pdiblc_i_dn7, locals.var_pdiblc_i_dn8, locals.var_pdiblc_i_dn9, locals.var_pdiblc_i_dn10, locals.var_pdiblc_i_dn11,)
    } else {
        (locals.var_pdiblc_a, locals.var_pdiblc_a_dn3, locals.var_pdiblc_a_dn4, locals.var_pdiblc_a_dn5, locals.var_pdiblc_a_dn6, locals.var_pdiblc_a_dn7, locals.var_pdiblc_a_dn8, locals.var_pdiblc_a_dn9, locals.var_pdiblc_a_dn10, locals.var_pdiblc_a_dn11,)
    }
};
        locals.var_pdiblc_a = assign16530_e22160;
        locals.var_pdiblc_a_dn3 = assign16530_e22160_d_n3;
        locals.var_pdiblc_a_dn4 = assign16530_e22160_d_n4;
        locals.var_pdiblc_a_dn5 = assign16530_e22160_d_n5;
        locals.var_pdiblc_a_dn6 = assign16530_e22160_d_n6;
        locals.var_pdiblc_a_dn7 = assign16530_e22160_d_n7;
        locals.var_pdiblc_a_dn8 = assign16530_e22160_d_n8;
        locals.var_pdiblc_a_dn9 = assign16530_e22160_d_n9;
        locals.var_pdiblc_a_dn10 = assign16530_e22160_d_n10;
        locals.var_pdiblc_a_dn11 = assign16530_e22160_d_n11;

        let (assign16540_e22165, assign16540_e22165_d_n3, assign16540_e22165_d_n4, assign16540_e22165_d_n5, assign16540_e22165_d_n6, assign16540_e22165_d_n7, assign16540_e22165_d_n8, assign16540_e22165_d_n9, assign16540_e22165_d_n10, assign16540_e22165_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_pclm_i, locals.var_pclm_i_dn3, locals.var_pclm_i_dn4, locals.var_pclm_i_dn5, locals.var_pclm_i_dn6, locals.var_pclm_i_dn7, locals.var_pclm_i_dn8, locals.var_pclm_i_dn9, locals.var_pclm_i_dn10, locals.var_pclm_i_dn11,)
    } else {
        (locals.var_pclm_a, locals.var_pclm_a_dn3, locals.var_pclm_a_dn4, locals.var_pclm_a_dn5, locals.var_pclm_a_dn6, locals.var_pclm_a_dn7, locals.var_pclm_a_dn8, locals.var_pclm_a_dn9, locals.var_pclm_a_dn10, locals.var_pclm_a_dn11,)
    }
};
        locals.var_pclm_a = assign16540_e22165;
        locals.var_pclm_a_dn3 = assign16540_e22165_d_n3;
        locals.var_pclm_a_dn4 = assign16540_e22165_d_n4;
        locals.var_pclm_a_dn5 = assign16540_e22165_d_n5;
        locals.var_pclm_a_dn6 = assign16540_e22165_d_n6;
        locals.var_pclm_a_dn7 = assign16540_e22165_d_n7;
        locals.var_pclm_a_dn8 = assign16540_e22165_d_n8;
        locals.var_pclm_a_dn9 = assign16540_e22165_d_n9;
        locals.var_pclm_a_dn10 = assign16540_e22165_d_n10;
        locals.var_pclm_a_dn11 = assign16540_e22165_d_n11;

        let (assign16550_e22170, assign16550_e22170_d_n3, assign16550_e22170_d_n4, assign16550_e22170_d_n5, assign16550_e22170_d_n6, assign16550_e22170_d_n7, assign16550_e22170_d_n8, assign16550_e22170_d_n9, assign16550_e22170_d_n10, assign16550_e22170_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_psat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psat_a, locals.var_psat_a_dn3, locals.var_psat_a_dn4, locals.var_psat_a_dn5, locals.var_psat_a_dn6, locals.var_psat_a_dn7, locals.var_psat_a_dn8, locals.var_psat_a_dn9, locals.var_psat_a_dn10, locals.var_psat_a_dn11,)
    }
};
        locals.var_psat_a = assign16550_e22170;
        locals.var_psat_a_dn3 = assign16550_e22170_d_n3;
        locals.var_psat_a_dn4 = assign16550_e22170_d_n4;
        locals.var_psat_a_dn5 = assign16550_e22170_d_n5;
        locals.var_psat_a_dn6 = assign16550_e22170_d_n6;
        locals.var_psat_a_dn7 = assign16550_e22170_d_n7;
        locals.var_psat_a_dn8 = assign16550_e22170_d_n8;
        locals.var_psat_a_dn9 = assign16550_e22170_d_n9;
        locals.var_psat_a_dn10 = assign16550_e22170_d_n10;
        locals.var_psat_a_dn11 = assign16550_e22170_d_n11;

        let (assign16560_e22175, assign16560_e22175_d_n3, assign16560_e22175_d_n4, assign16560_e22175_d_n5, assign16560_e22175_d_n6, assign16560_e22175_d_n7, assign16560_e22175_d_n8, assign16560_e22175_d_n9, assign16560_e22175_d_n10, assign16560_e22175_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_vsat_t, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11,)
    } else {
        (locals.var_vsat_a, locals.var_vsat_a_dn3, locals.var_vsat_a_dn4, locals.var_vsat_a_dn5, locals.var_vsat_a_dn6, locals.var_vsat_a_dn7, locals.var_vsat_a_dn8, locals.var_vsat_a_dn9, locals.var_vsat_a_dn10, locals.var_vsat_a_dn11,)
    }
};
        locals.var_vsat_a = assign16560_e22175;
        locals.var_vsat_a_dn3 = assign16560_e22175_d_n3;
        locals.var_vsat_a_dn4 = assign16560_e22175_d_n4;
        locals.var_vsat_a_dn5 = assign16560_e22175_d_n5;
        locals.var_vsat_a_dn6 = assign16560_e22175_d_n6;
        locals.var_vsat_a_dn7 = assign16560_e22175_d_n7;
        locals.var_vsat_a_dn8 = assign16560_e22175_d_n8;
        locals.var_vsat_a_dn9 = assign16560_e22175_d_n9;
        locals.var_vsat_a_dn10 = assign16560_e22175_d_n10;
        locals.var_vsat_a_dn11 = assign16560_e22175_d_n11;

        let (assign16570_e22180, assign16570_e22180_d_n3, assign16570_e22180_d_n4, assign16570_e22180_d_n5, assign16570_e22180_d_n6, assign16570_e22180_d_n7, assign16570_e22180_d_n8, assign16570_e22180_d_n9, assign16570_e22180_d_n10, assign16570_e22180_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_ptwg_t, locals.var_ptwg_t_dn3, locals.var_ptwg_t_dn4, locals.var_ptwg_t_dn5, locals.var_ptwg_t_dn6, locals.var_ptwg_t_dn7, locals.var_ptwg_t_dn8, locals.var_ptwg_t_dn9, locals.var_ptwg_t_dn10, locals.var_ptwg_t_dn11,)
    } else {
        (locals.var_ptwg_a, locals.var_ptwg_a_dn3, locals.var_ptwg_a_dn4, locals.var_ptwg_a_dn5, locals.var_ptwg_a_dn6, locals.var_ptwg_a_dn7, locals.var_ptwg_a_dn8, locals.var_ptwg_a_dn9, locals.var_ptwg_a_dn10, locals.var_ptwg_a_dn11,)
    }
};
        locals.var_ptwg_a = assign16570_e22180;
        locals.var_ptwg_a_dn3 = assign16570_e22180_d_n3;
        locals.var_ptwg_a_dn4 = assign16570_e22180_d_n4;
        locals.var_ptwg_a_dn5 = assign16570_e22180_d_n5;
        locals.var_ptwg_a_dn6 = assign16570_e22180_d_n6;
        locals.var_ptwg_a_dn7 = assign16570_e22180_d_n7;
        locals.var_ptwg_a_dn8 = assign16570_e22180_d_n8;
        locals.var_ptwg_a_dn9 = assign16570_e22180_d_n9;
        locals.var_ptwg_a_dn10 = assign16570_e22180_d_n10;
        locals.var_ptwg_a_dn11 = assign16570_e22180_d_n11;

        let (assign16580_e22185, assign16580_e22185_d_n3, assign16580_e22185_d_n4, assign16580_e22185_d_n5, assign16580_e22185_d_n6, assign16580_e22185_d_n7, assign16580_e22185_d_n8, assign16580_e22185_d_n9, assign16580_e22185_d_n10, assign16580_e22185_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_u0_t, locals.var_u0_t_dn3, locals.var_u0_t_dn4, locals.var_u0_t_dn5, locals.var_u0_t_dn6, locals.var_u0_t_dn7, locals.var_u0_t_dn8, locals.var_u0_t_dn9, locals.var_u0_t_dn10, locals.var_u0_t_dn11,)
    } else {
        (locals.var_u0_a, locals.var_u0_a_dn3, locals.var_u0_a_dn4, locals.var_u0_a_dn5, locals.var_u0_a_dn6, locals.var_u0_a_dn7, locals.var_u0_a_dn8, locals.var_u0_a_dn9, locals.var_u0_a_dn10, locals.var_u0_a_dn11,)
    }
};
        locals.var_u0_a = assign16580_e22185;
        locals.var_u0_a_dn3 = assign16580_e22185_d_n3;
        locals.var_u0_a_dn4 = assign16580_e22185_d_n4;
        locals.var_u0_a_dn5 = assign16580_e22185_d_n5;
        locals.var_u0_a_dn6 = assign16580_e22185_d_n6;
        locals.var_u0_a_dn7 = assign16580_e22185_d_n7;
        locals.var_u0_a_dn8 = assign16580_e22185_d_n8;
        locals.var_u0_a_dn9 = assign16580_e22185_d_n9;
        locals.var_u0_a_dn10 = assign16580_e22185_d_n10;
        locals.var_u0_a_dn11 = assign16580_e22185_d_n11;

        let (assign16590_e22190, assign16590_e22190_d_n3, assign16590_e22190_d_n4, assign16590_e22190_d_n5, assign16590_e22190_d_n6, assign16590_e22190_d_n7, assign16590_e22190_d_n8, assign16590_e22190_d_n9, assign16590_e22190_d_n10, assign16590_e22190_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_ua_t, locals.var_ua_t_dn3, locals.var_ua_t_dn4, locals.var_ua_t_dn5, locals.var_ua_t_dn6, locals.var_ua_t_dn7, locals.var_ua_t_dn8, locals.var_ua_t_dn9, locals.var_ua_t_dn10, locals.var_ua_t_dn11,)
    } else {
        (locals.var_ua_a, locals.var_ua_a_dn3, locals.var_ua_a_dn4, locals.var_ua_a_dn5, locals.var_ua_a_dn6, locals.var_ua_a_dn7, locals.var_ua_a_dn8, locals.var_ua_a_dn9, locals.var_ua_a_dn10, locals.var_ua_a_dn11,)
    }
};
        locals.var_ua_a = assign16590_e22190;
        locals.var_ua_a_dn3 = assign16590_e22190_d_n3;
        locals.var_ua_a_dn4 = assign16590_e22190_d_n4;
        locals.var_ua_a_dn5 = assign16590_e22190_d_n5;
        locals.var_ua_a_dn6 = assign16590_e22190_d_n6;
        locals.var_ua_a_dn7 = assign16590_e22190_d_n7;
        locals.var_ua_a_dn8 = assign16590_e22190_d_n8;
        locals.var_ua_a_dn9 = assign16590_e22190_d_n9;
        locals.var_ua_a_dn10 = assign16590_e22190_d_n10;
        locals.var_ua_a_dn11 = assign16590_e22190_d_n11;

        let (assign16600_e22195, assign16600_e22195_d_n3, assign16600_e22195_d_n4, assign16600_e22195_d_n5, assign16600_e22195_d_n6, assign16600_e22195_d_n7, assign16600_e22195_d_n8, assign16600_e22195_d_n9, assign16600_e22195_d_n10, assign16600_e22195_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_uc_t, locals.var_uc_t_dn3, locals.var_uc_t_dn4, locals.var_uc_t_dn5, locals.var_uc_t_dn6, locals.var_uc_t_dn7, locals.var_uc_t_dn8, locals.var_uc_t_dn9, locals.var_uc_t_dn10, locals.var_uc_t_dn11,)
    } else {
        (locals.var_uc_a, locals.var_uc_a_dn3, locals.var_uc_a_dn4, locals.var_uc_a_dn5, locals.var_uc_a_dn6, locals.var_uc_a_dn7, locals.var_uc_a_dn8, locals.var_uc_a_dn9, locals.var_uc_a_dn10, locals.var_uc_a_dn11,)
    }
};
        locals.var_uc_a = assign16600_e22195;
        locals.var_uc_a_dn3 = assign16600_e22195_d_n3;
        locals.var_uc_a_dn4 = assign16600_e22195_d_n4;
        locals.var_uc_a_dn5 = assign16600_e22195_d_n5;
        locals.var_uc_a_dn6 = assign16600_e22195_d_n6;
        locals.var_uc_a_dn7 = assign16600_e22195_d_n7;
        locals.var_uc_a_dn8 = assign16600_e22195_d_n8;
        locals.var_uc_a_dn9 = assign16600_e22195_d_n9;
        locals.var_uc_a_dn10 = assign16600_e22195_d_n10;
        locals.var_uc_a_dn11 = assign16600_e22195_d_n11;

        let (assign16610_e22200, assign16610_e22200_d_n3, assign16610_e22200_d_n4, assign16610_e22200_d_n5, assign16610_e22200_d_n6, assign16610_e22200_d_n7, assign16610_e22200_d_n8, assign16610_e22200_d_n9, assign16610_e22200_d_n10, assign16610_e22200_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_ud_t, locals.var_ud_t_dn3, locals.var_ud_t_dn4, locals.var_ud_t_dn5, locals.var_ud_t_dn6, locals.var_ud_t_dn7, locals.var_ud_t_dn8, locals.var_ud_t_dn9, locals.var_ud_t_dn10, locals.var_ud_t_dn11,)
    } else {
        (locals.var_ud_a, locals.var_ud_a_dn3, locals.var_ud_a_dn4, locals.var_ud_a_dn5, locals.var_ud_a_dn6, locals.var_ud_a_dn7, locals.var_ud_a_dn8, locals.var_ud_a_dn9, locals.var_ud_a_dn10, locals.var_ud_a_dn11,)
    }
};
        locals.var_ud_a = assign16610_e22200;
        locals.var_ud_a_dn3 = assign16610_e22200_d_n3;
        locals.var_ud_a_dn4 = assign16610_e22200_d_n4;
        locals.var_ud_a_dn5 = assign16610_e22200_d_n5;
        locals.var_ud_a_dn6 = assign16610_e22200_d_n6;
        locals.var_ud_a_dn7 = assign16610_e22200_d_n7;
        locals.var_ud_a_dn8 = assign16610_e22200_d_n8;
        locals.var_ud_a_dn9 = assign16610_e22200_d_n9;
        locals.var_ud_a_dn10 = assign16610_e22200_d_n10;
        locals.var_ud_a_dn11 = assign16610_e22200_d_n11;

        let (assign16620_e22205, assign16620_e22205_d_n3, assign16620_e22205_d_n4, assign16620_e22205_d_n5, assign16620_e22205_d_n6, assign16620_e22205_d_n7, assign16620_e22205_d_n8, assign16620_e22205_d_n9, assign16620_e22205_d_n10, assign16620_e22205_d_n11,) = {
    if (locals.var_guard487 == 0.0) {
        (locals.var_ucs_t, 0.0, locals.var_ucs_t_dn4, locals.var_ucs_t_dn5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ucs_a, locals.var_ucs_a_dn3, locals.var_ucs_a_dn4, locals.var_ucs_a_dn5, locals.var_ucs_a_dn6, locals.var_ucs_a_dn7, locals.var_ucs_a_dn8, locals.var_ucs_a_dn9, locals.var_ucs_a_dn10, locals.var_ucs_a_dn11,)
    }
};
        locals.var_ucs_a = assign16620_e22205;
        locals.var_ucs_a_dn3 = assign16620_e22205_d_n3;
        locals.var_ucs_a_dn4 = assign16620_e22205_d_n4;
        locals.var_ucs_a_dn5 = assign16620_e22205_d_n5;
        locals.var_ucs_a_dn6 = assign16620_e22205_d_n6;
        locals.var_ucs_a_dn7 = assign16620_e22205_d_n7;
        locals.var_ucs_a_dn8 = assign16620_e22205_d_n8;
        locals.var_ucs_a_dn9 = assign16620_e22205_d_n9;
        locals.var_ucs_a_dn10 = assign16620_e22205_d_n10;
        locals.var_ucs_a_dn11 = assign16620_e22205_d_n11;

        let assign16630_e22208: f64 = (locals.var_epsratio * p.p74);
        let assign16630_e22210: f64 = (assign16630_e22208 * p.p76);
        let assign16630_e22211: f64 = (assign16630_e22210).sqrt();
        locals.var_sclf = assign16630_e22211;

        let assign16640_e22215: f64 = (locals.var_epsratio * p.p76);
        let assign16640_e22218: f64 = (0.375 * p.p74);
        let assign16640_e22219: f64 = (assign16640_e22215 + assign16640_e22218);
        let assign16640_e22220: f64 = (p.p74 * assign16640_e22219);
        let assign16640_e22221: f64 = (assign16640_e22220).sqrt();
        locals.var_sclm = assign16640_e22221;

        let assign16650_e22226: f64 = (p.p76 + p.p75);
        let assign16650_e22227: f64 = (locals.var_epsratio * assign16650_e22226);
        let assign16650_e22228: f64 = (p.p74 + assign16650_e22227);
        locals.var_teff = assign16650_e22228;

        let assign16660_e22231: f64 = (locals.var_vg - locals.var_vfb_i);
        let assign16660_e22234: f64 = (p.p75 * locals.var_epsratio);
        let assign16660_e22235: f64 = (assign16660_e22231 * assign16660_e22234);
        let assign16660_e22238: f64 = (locals.var_ve - locals.var_vfbb_i);
        let assign16660_e22241: f64 = (p.p76 * locals.var_epsratio);
        let assign16660_e22243: f64 = (assign16660_e22241 + p.p74);
        let assign16660_e22244: f64 = (assign16660_e22238 * assign16660_e22243);
        let assign16660_e22245: f64 = (assign16660_e22235 + assign16660_e22244);
        let assign16660_e22247: f64 = (assign16660_e22245 / locals.var_teff);
        locals.var_t0 = assign16660_e22247;
        locals.var_t0_dn3 = ((((-locals.var_vfb_i_dn3) * assign16660_e22234) + (locals.var_ve_dn3 * assign16660_e22243)) / locals.var_teff);
        locals.var_t0_dn4 = (((-locals.var_vfb_i_dn4) * assign16660_e22234) / locals.var_teff);
        locals.var_t0_dn5 = (((-locals.var_vfb_i_dn5) * assign16660_e22234) / locals.var_teff);
        locals.var_t0_dn6 = (((-locals.var_vfb_i_dn6) * assign16660_e22234) / locals.var_teff);
        locals.var_t0_dn7 = (((-locals.var_vfb_i_dn7) * assign16660_e22234) / locals.var_teff);
        locals.var_t0_dn8 = (((locals.var_vg_dn8 - locals.var_vfb_i_dn8) * assign16660_e22234) / locals.var_teff);
        locals.var_t0_dn9 = (((-locals.var_vfb_i_dn9) * assign16660_e22234) / locals.var_teff);
        locals.var_t0_dn10 = ((((locals.var_vg_dn10 - locals.var_vfb_i_dn10) * assign16660_e22234) + (locals.var_ve_dn10 * assign16660_e22243)) / locals.var_teff);
        locals.var_t0_dn11 = (((-locals.var_vfb_i_dn11) * assign16660_e22234) / locals.var_teff);

        let assign16670_e22251: f64 = (locals.var_bscl_i * locals.var_t0);
        let assign16670_e22252: f64 = (locals.var_ascl_i + assign16670_e22251);
        let assign16670_e22253: f64 = (assign16670_e22252).atan();
        let assign16670_e22255: f64 = (assign16670_e22253 / 3.141592653589793);
        let assign16670_e22257: f64 = (assign16670_e22255 + 0.5);
        locals.var_t1 = assign16670_e22257;
        locals.var_t1_dn3 = (((locals.var_bscl_i * locals.var_t0_dn3) / (1.0 + (assign16670_e22252 * assign16670_e22252))) / 3.141592653589793);
        locals.var_t1_dn4 = (((locals.var_bscl_i * locals.var_t0_dn4) / (1.0 + (assign16670_e22252 * assign16670_e22252))) / 3.141592653589793);
        locals.var_t1_dn5 = (((locals.var_bscl_i * locals.var_t0_dn5) / (1.0 + (assign16670_e22252 * assign16670_e22252))) / 3.141592653589793);
        locals.var_t1_dn6 = (((locals.var_bscl_i * locals.var_t0_dn6) / (1.0 + (assign16670_e22252 * assign16670_e22252))) / 3.141592653589793);
        locals.var_t1_dn7 = (((locals.var_bscl_i * locals.var_t0_dn7) / (1.0 + (assign16670_e22252 * assign16670_e22252))) / 3.141592653589793);
        locals.var_t1_dn8 = (((locals.var_bscl_i * locals.var_t0_dn8) / (1.0 + (assign16670_e22252 * assign16670_e22252))) / 3.141592653589793);
        locals.var_t1_dn9 = (((locals.var_bscl_i * locals.var_t0_dn9) / (1.0 + (assign16670_e22252 * assign16670_e22252))) / 3.141592653589793);
        locals.var_t1_dn10 = (((locals.var_bscl_i * locals.var_t0_dn10) / (1.0 + (assign16670_e22252 * assign16670_e22252))) / 3.141592653589793);
        locals.var_t1_dn11 = (((locals.var_bscl_i * locals.var_t0_dn11) / (1.0 + (assign16670_e22252 * assign16670_e22252))) / 3.141592653589793);

        let assign16680_e22262: f64 = (locals.var_sclf - locals.var_sclm);
        let assign16680_e22263: f64 = (locals.var_t1 * assign16680_e22262);
        let assign16680_e22264: f64 = (locals.var_sclm + assign16680_e22263);
        locals.var_scl = assign16680_e22264;
        locals.var_scl_dn3 = (locals.var_t1_dn3 * assign16680_e22262);
        locals.var_scl_dn4 = (locals.var_t1_dn4 * assign16680_e22262);
        locals.var_scl_dn5 = (locals.var_t1_dn5 * assign16680_e22262);
        locals.var_scl_dn6 = (locals.var_t1_dn6 * assign16680_e22262);
        locals.var_scl_dn7 = (locals.var_t1_dn7 * assign16680_e22262);
        locals.var_scl_dn8 = (locals.var_t1_dn8 * assign16680_e22262);
        locals.var_scl_dn9 = (locals.var_t1_dn9 * assign16680_e22262);
        locals.var_scl_dn10 = (locals.var_t1_dn10 * assign16680_e22262);
        locals.var_scl_dn11 = (locals.var_t1_dn11 * assign16680_e22262);

        let assign16690_e22267: f64 = (locals.var_dvt1_i * locals.var_leff);
        let assign16690_e22269: f64 = (assign16690_e22267 / locals.var_scl);
        let assign16690_e22271: f64 = (assign16690_e22269 + 1e-6);
        locals.var_tmp = assign16690_e22271;
        locals.var_tmp_dn3 = (-((assign16690_e22267 * locals.var_scl_dn3) / (locals.var_scl * locals.var_scl)));
        locals.var_tmp_dn4 = (-((assign16690_e22267 * locals.var_scl_dn4) / (locals.var_scl * locals.var_scl)));
        locals.var_tmp_dn5 = (-((assign16690_e22267 * locals.var_scl_dn5) / (locals.var_scl * locals.var_scl)));
        locals.var_tmp_dn6 = (-((assign16690_e22267 * locals.var_scl_dn6) / (locals.var_scl * locals.var_scl)));
        locals.var_tmp_dn7 = (-((assign16690_e22267 * locals.var_scl_dn7) / (locals.var_scl * locals.var_scl)));
        locals.var_tmp_dn8 = (-((assign16690_e22267 * locals.var_scl_dn8) / (locals.var_scl * locals.var_scl)));
        locals.var_tmp_dn9 = (-((assign16690_e22267 * locals.var_scl_dn9) / (locals.var_scl * locals.var_scl)));
        locals.var_tmp_dn10 = (-((assign16690_e22267 * locals.var_scl_dn10) / (locals.var_scl * locals.var_scl)));
        locals.var_tmp_dn11 = (-((assign16690_e22267 * locals.var_scl_dn11) / (locals.var_scl * locals.var_scl)));

        let assign16700_e22274: f64 = if locals.var_tmp < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard488 = assign16700_e22274;

        let (assign16710_e22283, assign16710_e22283_d_n3, assign16710_e22283_d_n4, assign16710_e22283_d_n5, assign16710_e22283_d_n6, assign16710_e22283_d_n7, assign16710_e22283_d_n8, assign16710_e22283_d_n9, assign16710_e22283_d_n10, assign16710_e22283_d_n11,) = {
    if (locals.var_guard488 != 0.0) {
        let assign16710_e22278: f64 = (locals.var_tmp).cosh();
        let assign16710_e22280: f64 = (assign16710_e22278 - 1.0);
        let assign16710_e22281: f64 = (0.5 / assign16710_e22280);
        (assign16710_e22281, (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn3)) / (assign16710_e22280 * assign16710_e22280))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn4)) / (assign16710_e22280 * assign16710_e22280))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn5)) / (assign16710_e22280 * assign16710_e22280))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn6)) / (assign16710_e22280 * assign16710_e22280))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn7)) / (assign16710_e22280 * assign16710_e22280))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn8)) / (assign16710_e22280 * assign16710_e22280))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn9)) / (assign16710_e22280 * assign16710_e22280))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn10)) / (assign16710_e22280 * assign16710_e22280))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn11)) / (assign16710_e22280 * assign16710_e22280))),)
    } else {
        (locals.var_theta_sce, locals.var_theta_sce_dn3, locals.var_theta_sce_dn4, locals.var_theta_sce_dn5, locals.var_theta_sce_dn6, locals.var_theta_sce_dn7, locals.var_theta_sce_dn8, locals.var_theta_sce_dn9, locals.var_theta_sce_dn10, locals.var_theta_sce_dn11,)
    }
};
        locals.var_theta_sce = assign16710_e22283;
        locals.var_theta_sce_dn3 = assign16710_e22283_d_n3;
        locals.var_theta_sce_dn4 = assign16710_e22283_d_n4;
        locals.var_theta_sce_dn5 = assign16710_e22283_d_n5;
        locals.var_theta_sce_dn6 = assign16710_e22283_d_n6;
        locals.var_theta_sce_dn7 = assign16710_e22283_d_n7;
        locals.var_theta_sce_dn8 = assign16710_e22283_d_n8;
        locals.var_theta_sce_dn9 = assign16710_e22283_d_n9;
        locals.var_theta_sce_dn10 = assign16710_e22283_d_n10;
        locals.var_theta_sce_dn11 = assign16710_e22283_d_n11;

        let (assign16720_e22290, assign16720_e22290_d_n3, assign16720_e22290_d_n4, assign16720_e22290_d_n5, assign16720_e22290_d_n6, assign16720_e22290_d_n7, assign16720_e22290_d_n8, assign16720_e22290_d_n9, assign16720_e22290_d_n10, assign16720_e22290_d_n11,) = {
    if (locals.var_guard488 == 0.0) {
        let assign16720_e22287: f64 = (-locals.var_tmp);
        let assign16720_e22288: f64 = { let limited_exp_arg = assign16720_e22287; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign16720_e22288, ({ let limited_exp_arg = assign16720_e22287; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn3)), ({ let limited_exp_arg = assign16720_e22287; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn4)), ({ let limited_exp_arg = assign16720_e22287; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn5)), ({ let limited_exp_arg = assign16720_e22287; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn6)), ({ let limited_exp_arg = assign16720_e22287; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn7)), ({ let limited_exp_arg = assign16720_e22287; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn8)), ({ let limited_exp_arg = assign16720_e22287; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn9)), ({ let limited_exp_arg = assign16720_e22287; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn10)), ({ let limited_exp_arg = assign16720_e22287; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn11)),)
    } else {
        (locals.var_theta_sce, locals.var_theta_sce_dn3, locals.var_theta_sce_dn4, locals.var_theta_sce_dn5, locals.var_theta_sce_dn6, locals.var_theta_sce_dn7, locals.var_theta_sce_dn8, locals.var_theta_sce_dn9, locals.var_theta_sce_dn10, locals.var_theta_sce_dn11,)
    }
};
        locals.var_theta_sce = assign16720_e22290;
        locals.var_theta_sce_dn3 = assign16720_e22290_d_n3;
        locals.var_theta_sce_dn4 = assign16720_e22290_d_n4;
        locals.var_theta_sce_dn5 = assign16720_e22290_d_n5;
        locals.var_theta_sce_dn6 = assign16720_e22290_d_n6;
        locals.var_theta_sce_dn7 = assign16720_e22290_d_n7;
        locals.var_theta_sce_dn8 = assign16720_e22290_d_n8;
        locals.var_theta_sce_dn9 = assign16720_e22290_d_n9;
        locals.var_theta_sce_dn10 = assign16720_e22290_d_n10;
        locals.var_theta_sce_dn11 = assign16720_e22290_d_n11;

        let assign16730_e22293: f64 = (locals.var_epssi / p.p74);
        locals.var_cb = assign16730_e22293;

        let assign16740_e22296: f64 = (locals.var_epsox / p.p75);
        locals.var_cbox = assign16740_e22296;

        let assign16750_e22300: f64 = (locals.var_phist - locals.var_vbsx);
        let assign16750_e22302: f64 = (assign16750_e22300 + 0.05);
        let assign16750_e22305: f64 = (locals.var_phist - locals.var_vbsx);
        let assign16750_e22307: f64 = (assign16750_e22305 - 0.05);
        let assign16750_e22310: f64 = (locals.var_phist - locals.var_vbsx);
        let assign16750_e22312: f64 = (assign16750_e22310 - 0.05);
        let assign16750_e22313: f64 = (assign16750_e22307 * assign16750_e22312);
        let assign16750_e22316: f64 = (0.25 * 0.1);
        let assign16750_e22318: f64 = (assign16750_e22316 * 0.1);
        let assign16750_e22319: f64 = (assign16750_e22313 + assign16750_e22318);
        let assign16750_e22320: f64 = (assign16750_e22319).sqrt();
        let assign16750_e22321: f64 = (assign16750_e22302 + assign16750_e22320);
        let assign16750_e22322: f64 = (0.5 * assign16750_e22321);
        locals.var_phistvbs = assign16750_e22322;
        locals.var_phistvbs_dn3 = (0.5 * ((locals.var_phist_dn3 - locals.var_vbsx_dn3) + ((((locals.var_phist_dn3 - locals.var_vbsx_dn3) * assign16750_e22312) + (assign16750_e22307 * (locals.var_phist_dn3 - locals.var_vbsx_dn3))) / (2.0 * assign16750_e22320))));
        locals.var_phistvbs_dn4 = (0.5 * ((locals.var_phist_dn4 - locals.var_vbsx_dn4) + ((((locals.var_phist_dn4 - locals.var_vbsx_dn4) * assign16750_e22312) + (assign16750_e22307 * (locals.var_phist_dn4 - locals.var_vbsx_dn4))) / (2.0 * assign16750_e22320))));
        locals.var_phistvbs_dn5 = (0.5 * ((locals.var_phist_dn5 - locals.var_vbsx_dn5) + ((((locals.var_phist_dn5 - locals.var_vbsx_dn5) * assign16750_e22312) + (assign16750_e22307 * (locals.var_phist_dn5 - locals.var_vbsx_dn5))) / (2.0 * assign16750_e22320))));
        locals.var_phistvbs_dn6 = (0.5 * ((locals.var_phist_dn6 - locals.var_vbsx_dn6) + ((((locals.var_phist_dn6 - locals.var_vbsx_dn6) * assign16750_e22312) + (assign16750_e22307 * (locals.var_phist_dn6 - locals.var_vbsx_dn6))) / (2.0 * assign16750_e22320))));
        locals.var_phistvbs_dn7 = (0.5 * ((locals.var_phist_dn7 - locals.var_vbsx_dn7) + ((((locals.var_phist_dn7 - locals.var_vbsx_dn7) * assign16750_e22312) + (assign16750_e22307 * (locals.var_phist_dn7 - locals.var_vbsx_dn7))) / (2.0 * assign16750_e22320))));
        locals.var_phistvbs_dn8 = (0.5 * ((locals.var_phist_dn8 - locals.var_vbsx_dn8) + ((((locals.var_phist_dn8 - locals.var_vbsx_dn8) * assign16750_e22312) + (assign16750_e22307 * (locals.var_phist_dn8 - locals.var_vbsx_dn8))) / (2.0 * assign16750_e22320))));
        locals.var_phistvbs_dn9 = (0.5 * ((locals.var_phist_dn9 - locals.var_vbsx_dn9) + ((((locals.var_phist_dn9 - locals.var_vbsx_dn9) * assign16750_e22312) + (assign16750_e22307 * (locals.var_phist_dn9 - locals.var_vbsx_dn9))) / (2.0 * assign16750_e22320))));
        locals.var_phistvbs_dn10 = (0.5 * ((locals.var_phist_dn10 - locals.var_vbsx_dn10) + ((((locals.var_phist_dn10 - locals.var_vbsx_dn10) * assign16750_e22312) + (assign16750_e22307 * (locals.var_phist_dn10 - locals.var_vbsx_dn10))) / (2.0 * assign16750_e22320))));
        locals.var_phistvbs_dn11 = (0.5 * ((locals.var_phist_dn11 - locals.var_vbsx_dn11) + ((((locals.var_phist_dn11 - locals.var_vbsx_dn11) * assign16750_e22312) + (assign16750_e22307 * (locals.var_phist_dn11 - locals.var_vbsx_dn11))) / (2.0 * assign16750_e22320))));

        let assign16760_e22324: f64 = (locals.var_phistvbs).sqrt();
        locals.var_sqrtphistvbs = assign16760_e22324;
        locals.var_sqrtphistvbs_dn3 = (locals.var_phistvbs_dn3 / (2.0 * assign16760_e22324));
        locals.var_sqrtphistvbs_dn4 = (locals.var_phistvbs_dn4 / (2.0 * assign16760_e22324));
        locals.var_sqrtphistvbs_dn5 = (locals.var_phistvbs_dn5 / (2.0 * assign16760_e22324));
        locals.var_sqrtphistvbs_dn6 = (locals.var_phistvbs_dn6 / (2.0 * assign16760_e22324));
        locals.var_sqrtphistvbs_dn7 = (locals.var_phistvbs_dn7 / (2.0 * assign16760_e22324));
        locals.var_sqrtphistvbs_dn8 = (locals.var_phistvbs_dn8 / (2.0 * assign16760_e22324));
        locals.var_sqrtphistvbs_dn9 = (locals.var_phistvbs_dn9 / (2.0 * assign16760_e22324));
        locals.var_sqrtphistvbs_dn10 = (locals.var_phistvbs_dn10 / (2.0 * assign16760_e22324));
        locals.var_sqrtphistvbs_dn11 = (locals.var_phistvbs_dn11 / (2.0 * assign16760_e22324));

        let assign16770_e22327: f64 = (locals.var_t1dep * locals.var_sqrtphistvbs);
        locals.var_xdep = assign16770_e22327;
        locals.var_xdep_dn3 = ((locals.var_t1dep_dn3 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn3));
        locals.var_xdep_dn4 = ((locals.var_t1dep_dn4 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn4));
        locals.var_xdep_dn5 = ((locals.var_t1dep_dn5 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn5));
        locals.var_xdep_dn6 = ((locals.var_t1dep_dn6 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn6));
        locals.var_xdep_dn7 = ((locals.var_t1dep_dn7 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn7));
        locals.var_xdep_dn8 = ((locals.var_t1dep_dn8 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn8));
        locals.var_xdep_dn9 = ((locals.var_t1dep_dn9 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn9));
        locals.var_xdep_dn10 = ((locals.var_t1dep_dn10 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn10));
        locals.var_xdep_dn11 = ((locals.var_t1dep_dn11 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn11));

        let assign16780_e22330: f64 = (locals.var_epssi / locals.var_xdep);
        locals.var_cdep = assign16780_e22330;
        locals.var_cdep_dn3 = (-((locals.var_epssi * locals.var_xdep_dn3) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn4 = (-((locals.var_epssi * locals.var_xdep_dn4) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn5 = (-((locals.var_epssi * locals.var_xdep_dn5) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn6 = (-((locals.var_epssi * locals.var_xdep_dn6) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn7 = (-((locals.var_epssi * locals.var_xdep_dn7) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn8 = (-((locals.var_epssi * locals.var_xdep_dn8) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn9 = (-((locals.var_epssi * locals.var_xdep_dn9) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn10 = (-((locals.var_epssi * locals.var_xdep_dn10) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn11 = (-((locals.var_epssi * locals.var_xdep_dn11) / (locals.var_xdep * locals.var_xdep)));

        let assign16790_e22333: f64 = (locals.var_cit_i + locals.var_nfactor_t);
        let assign16790_e22336: f64 = (locals.var_cdscd_a * locals.var_vdsx);
        let assign16790_e22337: f64 = (assign16790_e22333 + assign16790_e22336);
        let assign16790_e22340: f64 = (locals.var_cdscb_i * locals.var_vbsx);
        let assign16790_e22341: f64 = (assign16790_e22337 - assign16790_e22340);
        locals.var_cdsc = assign16790_e22341;
        locals.var_cdsc_dn3 = ((locals.var_nfactor_t_dn3 + ((locals.var_cdscd_a_dn3 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn3))) - (locals.var_cdscb_i * locals.var_vbsx_dn3));
        locals.var_cdsc_dn4 = ((locals.var_nfactor_t_dn4 + ((locals.var_cdscd_a_dn4 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn4))) - (locals.var_cdscb_i * locals.var_vbsx_dn4));
        locals.var_cdsc_dn5 = ((locals.var_nfactor_t_dn5 + ((locals.var_cdscd_a_dn5 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn5))) - (locals.var_cdscb_i * locals.var_vbsx_dn5));
        locals.var_cdsc_dn6 = ((locals.var_nfactor_t_dn6 + ((locals.var_cdscd_a_dn6 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn6))) - (locals.var_cdscb_i * locals.var_vbsx_dn6));
        locals.var_cdsc_dn7 = ((locals.var_nfactor_t_dn7 + ((locals.var_cdscd_a_dn7 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn7))) - (locals.var_cdscb_i * locals.var_vbsx_dn7));
        locals.var_cdsc_dn8 = ((locals.var_nfactor_t_dn8 + ((locals.var_cdscd_a_dn8 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn8))) - (locals.var_cdscb_i * locals.var_vbsx_dn8));
        locals.var_cdsc_dn9 = ((locals.var_nfactor_t_dn9 + ((locals.var_cdscd_a_dn9 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn9))) - (locals.var_cdscb_i * locals.var_vbsx_dn9));
        locals.var_cdsc_dn10 = ((locals.var_nfactor_t_dn10 + ((locals.var_cdscd_a_dn10 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn10))) - (locals.var_cdscb_i * locals.var_vbsx_dn10));
        locals.var_cdsc_dn11 = ((locals.var_nfactor_t_dn11 + ((locals.var_cdscd_a_dn11 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn11))) - (locals.var_cdscb_i * locals.var_vbsx_dn11));

        let assign16800_e22345: f64 = (locals.var_cdsc / locals.var_cox);
        let assign16800_e22346: f64 = (1.0 + assign16800_e22345);
        locals.var_t1 = assign16800_e22346;
        locals.var_t1_dn3 = (locals.var_cdsc_dn3 / locals.var_cox);
        locals.var_t1_dn4 = (locals.var_cdsc_dn4 / locals.var_cox);
        locals.var_t1_dn5 = (locals.var_cdsc_dn5 / locals.var_cox);
        locals.var_t1_dn6 = (locals.var_cdsc_dn6 / locals.var_cox);
        locals.var_t1_dn7 = (locals.var_cdsc_dn7 / locals.var_cox);
        locals.var_t1_dn8 = (locals.var_cdsc_dn8 / locals.var_cox);
        locals.var_t1_dn9 = (locals.var_cdsc_dn9 / locals.var_cox);
        locals.var_t1_dn10 = (locals.var_cdsc_dn10 / locals.var_cox);
        locals.var_t1_dn11 = (locals.var_cdsc_dn11 / locals.var_cox);

        let assign16810_e22349: f64 = if p.p29 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard489 = assign16810_e22349;

        let (assign16820_e22359, assign16820_e22359_d_n3, assign16820_e22359_d_n4, assign16820_e22359_d_n5, assign16820_e22359_d_n6, assign16820_e22359_d_n7, assign16820_e22359_d_n8, assign16820_e22359_d_n9, assign16820_e22359_d_n10, assign16820_e22359_d_n11,) = {
    if (locals.var_guard489 != 0.0) {
        let assign16820_e22353: f64 = (locals.var_cb * locals.var_cbox);
        let assign16820_e22356: f64 = (locals.var_cb + locals.var_cbox);
        let assign16820_e22357: f64 = (assign16820_e22353 / assign16820_e22356);
        (assign16820_e22357, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign16820_e22359;
        locals.var_t2_dn3 = assign16820_e22359_d_n3;
        locals.var_t2_dn4 = assign16820_e22359_d_n4;
        locals.var_t2_dn5 = assign16820_e22359_d_n5;
        locals.var_t2_dn6 = assign16820_e22359_d_n6;
        locals.var_t2_dn7 = assign16820_e22359_d_n7;
        locals.var_t2_dn8 = assign16820_e22359_d_n8;
        locals.var_t2_dn9 = assign16820_e22359_d_n9;
        locals.var_t2_dn10 = assign16820_e22359_d_n10;
        locals.var_t2_dn11 = assign16820_e22359_d_n11;

        let (assign16830_e22373, assign16830_e22373_d_n3, assign16830_e22373_d_n4, assign16830_e22373_d_n5, assign16830_e22373_d_n6, assign16830_e22373_d_n7, assign16830_e22373_d_n8, assign16830_e22373_d_n9, assign16830_e22373_d_n10, assign16830_e22373_d_n11,) = {
    if (locals.var_guard489 != 0.0) {
        let assign16830_e22364: f64 = (p.p266 * locals.var_vesx);
        let assign16830_e22365: f64 = (locals.var_cdscd_a + assign16830_e22364);
        let assign16830_e22368: f64 = (p.p267 * locals.var_vbsx);
        let assign16830_e22369: f64 = (assign16830_e22365 - assign16830_e22368);
        let assign16830_e22371: f64 = (assign16830_e22369 * locals.var_vdsx);
        (assign16830_e22371, ((((locals.var_cdscd_a_dn3 + (p.p266 * locals.var_vesx_dn3)) - (p.p267 * locals.var_vbsx_dn3)) * locals.var_vdsx) + (assign16830_e22369 * locals.var_vdsx_dn3)), ((((locals.var_cdscd_a_dn4 + (p.p266 * locals.var_vesx_dn4)) - (p.p267 * locals.var_vbsx_dn4)) * locals.var_vdsx) + (assign16830_e22369 * locals.var_vdsx_dn4)), ((((locals.var_cdscd_a_dn5 + (p.p266 * locals.var_vesx_dn5)) - (p.p267 * locals.var_vbsx_dn5)) * locals.var_vdsx) + (assign16830_e22369 * locals.var_vdsx_dn5)), ((((locals.var_cdscd_a_dn6 + (p.p266 * locals.var_vesx_dn6)) - (p.p267 * locals.var_vbsx_dn6)) * locals.var_vdsx) + (assign16830_e22369 * locals.var_vdsx_dn6)), ((((locals.var_cdscd_a_dn7 + (p.p266 * locals.var_vesx_dn7)) - (p.p267 * locals.var_vbsx_dn7)) * locals.var_vdsx) + (assign16830_e22369 * locals.var_vdsx_dn7)), ((((locals.var_cdscd_a_dn8 + (p.p266 * locals.var_vesx_dn8)) - (p.p267 * locals.var_vbsx_dn8)) * locals.var_vdsx) + (assign16830_e22369 * locals.var_vdsx_dn8)), ((((locals.var_cdscd_a_dn9 + (p.p266 * locals.var_vesx_dn9)) - (p.p267 * locals.var_vbsx_dn9)) * locals.var_vdsx) + (assign16830_e22369 * locals.var_vdsx_dn9)), ((((locals.var_cdscd_a_dn10 + (p.p266 * locals.var_vesx_dn10)) - (p.p267 * locals.var_vbsx_dn10)) * locals.var_vdsx) + (assign16830_e22369 * locals.var_vdsx_dn10)), ((((locals.var_cdscd_a_dn11 + (p.p266 * locals.var_vesx_dn11)) - (p.p267 * locals.var_vbsx_dn11)) * locals.var_vdsx) + (assign16830_e22369 * locals.var_vdsx_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign16830_e22373;
        locals.var_t3_dn3 = assign16830_e22373_d_n3;
        locals.var_t3_dn4 = assign16830_e22373_d_n4;
        locals.var_t3_dn5 = assign16830_e22373_d_n5;
        locals.var_t3_dn6 = assign16830_e22373_d_n6;
        locals.var_t3_dn7 = assign16830_e22373_d_n7;
        locals.var_t3_dn8 = assign16830_e22373_d_n8;
        locals.var_t3_dn9 = assign16830_e22373_d_n9;
        locals.var_t3_dn10 = assign16830_e22373_d_n10;
        locals.var_t3_dn11 = assign16830_e22373_d_n11;

    }

    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16840_e22421, assign16840_e22421_d_n3, assign16840_e22421_d_n4, assign16840_e22421_d_n5, assign16840_e22421_d_n6, assign16840_e22421_d_n7, assign16840_e22421_d_n8, assign16840_e22421_d_n9, assign16840_e22421_d_n10, assign16840_e22421_d_n11,) = {
    if (locals.var_guard489 != 0.0) {
        let assign16840_e22377: f64 = (p.p268 * locals.var_vesx);
        let assign16840_e22380: f64 = (p.p269 * locals.var_vesx);
        let assign16840_e22382: f64 = (assign16840_e22380 * locals.var_vesx);
        let assign16840_e22383: f64 = (assign16840_e22377 + assign16840_e22382);
        let assign16840_e22386: f64 = (p.p280 * locals.var_vbsx);
        let assign16840_e22387: f64 = (assign16840_e22383 - assign16840_e22386);
        let assign16840_e22390: f64 = (p.p281 * locals.var_vbsx);
        let assign16840_e22392: f64 = (assign16840_e22390 * locals.var_vbsx);
        let assign16840_e22393: f64 = (assign16840_e22387 - assign16840_e22392);
        let assign16840_e22398: f64 = (locals.var_csecse_i * locals.var_vesx);
        let assign16840_e22399: f64 = (locals.var_cdsc_i + assign16840_e22398);
        let assign16840_e22402: f64 = (p.p274 * locals.var_vesx);
        let assign16840_e22404: f64 = (assign16840_e22402 * locals.var_vesx);
        let assign16840_e22405: f64 = (assign16840_e22399 + assign16840_e22404);
        let assign16840_e22408: f64 = (locals.var_cbcb_i * locals.var_vbsx);
        let assign16840_e22409: f64 = (assign16840_e22405 + assign16840_e22408);
        let assign16840_e22412: f64 = (p.p279 * locals.var_vbsx);
        let assign16840_e22414: f64 = (assign16840_e22412 * locals.var_vbsx);
        let assign16840_e22415: f64 = (assign16840_e22409 + assign16840_e22414);
        let assign16840_e22417: f64 = (assign16840_e22415 + locals.var_t3);
        let assign16840_e22418: f64 = (locals.var_theta_sce * assign16840_e22417);
        let assign16840_e22419: f64 = (assign16840_e22393 + assign16840_e22418);
        (assign16840_e22419, (((((p.p268 * locals.var_vesx_dn3) + (((p.p269 * locals.var_vesx_dn3) * locals.var_vesx) + (assign16840_e22380 * locals.var_vesx_dn3))) - (p.p280 * locals.var_vbsx_dn3)) - (((p.p281 * locals.var_vbsx_dn3) * locals.var_vbsx) + (assign16840_e22390 * locals.var_vbsx_dn3))) + ((locals.var_theta_sce_dn3 * assign16840_e22417) + (locals.var_theta_sce * (((((locals.var_csecse_i * locals.var_vesx_dn3) + (((p.p274 * locals.var_vesx_dn3) * locals.var_vesx) + (assign16840_e22402 * locals.var_vesx_dn3))) + (locals.var_cbcb_i * locals.var_vbsx_dn3)) + (((p.p279 * locals.var_vbsx_dn3) * locals.var_vbsx) + (assign16840_e22412 * locals.var_vbsx_dn3))) + locals.var_t3_dn3)))), (((((p.p268 * locals.var_vesx_dn4) + (((p.p269 * locals.var_vesx_dn4) * locals.var_vesx) + (assign16840_e22380 * locals.var_vesx_dn4))) - (p.p280 * locals.var_vbsx_dn4)) - (((p.p281 * locals.var_vbsx_dn4) * locals.var_vbsx) + (assign16840_e22390 * locals.var_vbsx_dn4))) + ((locals.var_theta_sce_dn4 * assign16840_e22417) + (locals.var_theta_sce * (((((locals.var_csecse_i * locals.var_vesx_dn4) + (((p.p274 * locals.var_vesx_dn4) * locals.var_vesx) + (assign16840_e22402 * locals.var_vesx_dn4))) + (locals.var_cbcb_i * locals.var_vbsx_dn4)) + (((p.p279 * locals.var_vbsx_dn4) * locals.var_vbsx) + (assign16840_e22412 * locals.var_vbsx_dn4))) + locals.var_t3_dn4)))), (((((p.p268 * locals.var_vesx_dn5) + (((p.p269 * locals.var_vesx_dn5) * locals.var_vesx) + (assign16840_e22380 * locals.var_vesx_dn5))) - (p.p280 * locals.var_vbsx_dn5)) - (((p.p281 * locals.var_vbsx_dn5) * locals.var_vbsx) + (assign16840_e22390 * locals.var_vbsx_dn5))) + ((locals.var_theta_sce_dn5 * assign16840_e22417) + (locals.var_theta_sce * (((((locals.var_csecse_i * locals.var_vesx_dn5) + (((p.p274 * locals.var_vesx_dn5) * locals.var_vesx) + (assign16840_e22402 * locals.var_vesx_dn5))) + (locals.var_cbcb_i * locals.var_vbsx_dn5)) + (((p.p279 * locals.var_vbsx_dn5) * locals.var_vbsx) + (assign16840_e22412 * locals.var_vbsx_dn5))) + locals.var_t3_dn5)))), (((((p.p268 * locals.var_vesx_dn6) + (((p.p269 * locals.var_vesx_dn6) * locals.var_vesx) + (assign16840_e22380 * locals.var_vesx_dn6))) - (p.p280 * locals.var_vbsx_dn6)) - (((p.p281 * locals.var_vbsx_dn6) * locals.var_vbsx) + (assign16840_e22390 * locals.var_vbsx_dn6))) + ((locals.var_theta_sce_dn6 * assign16840_e22417) + (locals.var_theta_sce * (((((locals.var_csecse_i * locals.var_vesx_dn6) + (((p.p274 * locals.var_vesx_dn6) * locals.var_vesx) + (assign16840_e22402 * locals.var_vesx_dn6))) + (locals.var_cbcb_i * locals.var_vbsx_dn6)) + (((p.p279 * locals.var_vbsx_dn6) * locals.var_vbsx) + (assign16840_e22412 * locals.var_vbsx_dn6))) + locals.var_t3_dn6)))), (((((p.p268 * locals.var_vesx_dn7) + (((p.p269 * locals.var_vesx_dn7) * locals.var_vesx) + (assign16840_e22380 * locals.var_vesx_dn7))) - (p.p280 * locals.var_vbsx_dn7)) - (((p.p281 * locals.var_vbsx_dn7) * locals.var_vbsx) + (assign16840_e22390 * locals.var_vbsx_dn7))) + ((locals.var_theta_sce_dn7 * assign16840_e22417) + (locals.var_theta_sce * (((((locals.var_csecse_i * locals.var_vesx_dn7) + (((p.p274 * locals.var_vesx_dn7) * locals.var_vesx) + (assign16840_e22402 * locals.var_vesx_dn7))) + (locals.var_cbcb_i * locals.var_vbsx_dn7)) + (((p.p279 * locals.var_vbsx_dn7) * locals.var_vbsx) + (assign16840_e22412 * locals.var_vbsx_dn7))) + locals.var_t3_dn7)))), (((((p.p268 * locals.var_vesx_dn8) + (((p.p269 * locals.var_vesx_dn8) * locals.var_vesx) + (assign16840_e22380 * locals.var_vesx_dn8))) - (p.p280 * locals.var_vbsx_dn8)) - (((p.p281 * locals.var_vbsx_dn8) * locals.var_vbsx) + (assign16840_e22390 * locals.var_vbsx_dn8))) + ((locals.var_theta_sce_dn8 * assign16840_e22417) + (locals.var_theta_sce * (((((locals.var_csecse_i * locals.var_vesx_dn8) + (((p.p274 * locals.var_vesx_dn8) * locals.var_vesx) + (assign16840_e22402 * locals.var_vesx_dn8))) + (locals.var_cbcb_i * locals.var_vbsx_dn8)) + (((p.p279 * locals.var_vbsx_dn8) * locals.var_vbsx) + (assign16840_e22412 * locals.var_vbsx_dn8))) + locals.var_t3_dn8)))), (((((p.p268 * locals.var_vesx_dn9) + (((p.p269 * locals.var_vesx_dn9) * locals.var_vesx) + (assign16840_e22380 * locals.var_vesx_dn9))) - (p.p280 * locals.var_vbsx_dn9)) - (((p.p281 * locals.var_vbsx_dn9) * locals.var_vbsx) + (assign16840_e22390 * locals.var_vbsx_dn9))) + ((locals.var_theta_sce_dn9 * assign16840_e22417) + (locals.var_theta_sce * (((((locals.var_csecse_i * locals.var_vesx_dn9) + (((p.p274 * locals.var_vesx_dn9) * locals.var_vesx) + (assign16840_e22402 * locals.var_vesx_dn9))) + (locals.var_cbcb_i * locals.var_vbsx_dn9)) + (((p.p279 * locals.var_vbsx_dn9) * locals.var_vbsx) + (assign16840_e22412 * locals.var_vbsx_dn9))) + locals.var_t3_dn9)))), (((((p.p268 * locals.var_vesx_dn10) + (((p.p269 * locals.var_vesx_dn10) * locals.var_vesx) + (assign16840_e22380 * locals.var_vesx_dn10))) - (p.p280 * locals.var_vbsx_dn10)) - (((p.p281 * locals.var_vbsx_dn10) * locals.var_vbsx) + (assign16840_e22390 * locals.var_vbsx_dn10))) + ((locals.var_theta_sce_dn10 * assign16840_e22417) + (locals.var_theta_sce * (((((locals.var_csecse_i * locals.var_vesx_dn10) + (((p.p274 * locals.var_vesx_dn10) * locals.var_vesx) + (assign16840_e22402 * locals.var_vesx_dn10))) + (locals.var_cbcb_i * locals.var_vbsx_dn10)) + (((p.p279 * locals.var_vbsx_dn10) * locals.var_vbsx) + (assign16840_e22412 * locals.var_vbsx_dn10))) + locals.var_t3_dn10)))), (((((p.p268 * locals.var_vesx_dn11) + (((p.p269 * locals.var_vesx_dn11) * locals.var_vesx) + (assign16840_e22380 * locals.var_vesx_dn11))) - (p.p280 * locals.var_vbsx_dn11)) - (((p.p281 * locals.var_vbsx_dn11) * locals.var_vbsx) + (assign16840_e22390 * locals.var_vbsx_dn11))) + ((locals.var_theta_sce_dn11 * assign16840_e22417) + (locals.var_theta_sce * (((((locals.var_csecse_i * locals.var_vesx_dn11) + (((p.p274 * locals.var_vesx_dn11) * locals.var_vesx) + (assign16840_e22402 * locals.var_vesx_dn11))) + (locals.var_cbcb_i * locals.var_vbsx_dn11)) + (((p.p279 * locals.var_vbsx_dn11) * locals.var_vbsx) + (assign16840_e22412 * locals.var_vbsx_dn11))) + locals.var_t3_dn11)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign16840_e22421;
        locals.var_t4_dn3 = assign16840_e22421_d_n3;
        locals.var_t4_dn4 = assign16840_e22421_d_n4;
        locals.var_t4_dn5 = assign16840_e22421_d_n5;
        locals.var_t4_dn6 = assign16840_e22421_d_n6;
        locals.var_t4_dn7 = assign16840_e22421_d_n7;
        locals.var_t4_dn8 = assign16840_e22421_d_n8;
        locals.var_t4_dn9 = assign16840_e22421_d_n9;
        locals.var_t4_dn10 = assign16840_e22421_d_n10;
        locals.var_t4_dn11 = assign16840_e22421_d_n11;

        let (assign16850_e22437, assign16850_e22437_d_n3, assign16850_e22437_d_n4, assign16850_e22437_d_n5, assign16850_e22437_d_n6, assign16850_e22437_d_n7, assign16850_e22437_d_n8, assign16850_e22437_d_n9, assign16850_e22437_d_n10, assign16850_e22437_d_n11,) = {
    if (locals.var_guard489 != 0.0) {
        let assign16850_e22425: f64 = (locals.var_cox + locals.var_t2);
        let assign16850_e22427: f64 = (assign16850_e22425 + locals.var_cit_i);
        let assign16850_e22429: f64 = (assign16850_e22427 + locals.var_nfactor_t);
        let assign16850_e22431: f64 = (assign16850_e22429 + locals.var_t4);
        let assign16850_e22434: f64 = (locals.var_cox + locals.var_t2);
        let assign16850_e22435: f64 = (assign16850_e22431 / assign16850_e22434);
        (assign16850_e22435, (((((locals.var_t2_dn3 + locals.var_nfactor_t_dn3) + locals.var_t4_dn3) * assign16850_e22434) - (assign16850_e22431 * locals.var_t2_dn3)) / (assign16850_e22434 * assign16850_e22434)), (((((locals.var_t2_dn4 + locals.var_nfactor_t_dn4) + locals.var_t4_dn4) * assign16850_e22434) - (assign16850_e22431 * locals.var_t2_dn4)) / (assign16850_e22434 * assign16850_e22434)), (((((locals.var_t2_dn5 + locals.var_nfactor_t_dn5) + locals.var_t4_dn5) * assign16850_e22434) - (assign16850_e22431 * locals.var_t2_dn5)) / (assign16850_e22434 * assign16850_e22434)), (((((locals.var_t2_dn6 + locals.var_nfactor_t_dn6) + locals.var_t4_dn6) * assign16850_e22434) - (assign16850_e22431 * locals.var_t2_dn6)) / (assign16850_e22434 * assign16850_e22434)), (((((locals.var_t2_dn7 + locals.var_nfactor_t_dn7) + locals.var_t4_dn7) * assign16850_e22434) - (assign16850_e22431 * locals.var_t2_dn7)) / (assign16850_e22434 * assign16850_e22434)), (((((locals.var_t2_dn8 + locals.var_nfactor_t_dn8) + locals.var_t4_dn8) * assign16850_e22434) - (assign16850_e22431 * locals.var_t2_dn8)) / (assign16850_e22434 * assign16850_e22434)), (((((locals.var_t2_dn9 + locals.var_nfactor_t_dn9) + locals.var_t4_dn9) * assign16850_e22434) - (assign16850_e22431 * locals.var_t2_dn9)) / (assign16850_e22434 * assign16850_e22434)), (((((locals.var_t2_dn10 + locals.var_nfactor_t_dn10) + locals.var_t4_dn10) * assign16850_e22434) - (assign16850_e22431 * locals.var_t2_dn10)) / (assign16850_e22434 * assign16850_e22434)), (((((locals.var_t2_dn11 + locals.var_nfactor_t_dn11) + locals.var_t4_dn11) * assign16850_e22434) - (assign16850_e22431 * locals.var_t2_dn11)) / (assign16850_e22434 * assign16850_e22434)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign16850_e22437;
        locals.var_t1_dn3 = assign16850_e22437_d_n3;
        locals.var_t1_dn4 = assign16850_e22437_d_n4;
        locals.var_t1_dn5 = assign16850_e22437_d_n5;
        locals.var_t1_dn6 = assign16850_e22437_d_n6;
        locals.var_t1_dn7 = assign16850_e22437_d_n7;
        locals.var_t1_dn8 = assign16850_e22437_d_n8;
        locals.var_t1_dn9 = assign16850_e22437_d_n9;
        locals.var_t1_dn10 = assign16850_e22437_d_n10;
        locals.var_t1_dn11 = assign16850_e22437_d_n11;

        let assign16860_e22441: f64 = (locals.var_t1 + 1.0);
        let assign16860_e22444: f64 = (locals.var_t1 - 1.0);
        let assign16860_e22447: f64 = (locals.var_t1 - 1.0);
        let assign16860_e22448: f64 = (assign16860_e22444 * assign16860_e22447);
        let assign16860_e22451: f64 = (0.25 * 0.05);
        let assign16860_e22453: f64 = (assign16860_e22451 * 0.05);
        let assign16860_e22454: f64 = (assign16860_e22448 + assign16860_e22453);
        let assign16860_e22455: f64 = (assign16860_e22454).sqrt();
        let assign16860_e22456: f64 = (assign16860_e22441 + assign16860_e22455);
        let assign16860_e22457: f64 = (0.5 * assign16860_e22456);
        locals.var_n = assign16860_e22457;
        locals.var_n_dn3 = (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * assign16860_e22447) + (assign16860_e22444 * locals.var_t1_dn3)) / (2.0 * assign16860_e22455))));
        locals.var_n_dn4 = (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * assign16860_e22447) + (assign16860_e22444 * locals.var_t1_dn4)) / (2.0 * assign16860_e22455))));
        locals.var_n_dn5 = (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * assign16860_e22447) + (assign16860_e22444 * locals.var_t1_dn5)) / (2.0 * assign16860_e22455))));
        locals.var_n_dn6 = (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * assign16860_e22447) + (assign16860_e22444 * locals.var_t1_dn6)) / (2.0 * assign16860_e22455))));
        locals.var_n_dn7 = (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * assign16860_e22447) + (assign16860_e22444 * locals.var_t1_dn7)) / (2.0 * assign16860_e22455))));
        locals.var_n_dn8 = (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * assign16860_e22447) + (assign16860_e22444 * locals.var_t1_dn8)) / (2.0 * assign16860_e22455))));
        locals.var_n_dn9 = (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * assign16860_e22447) + (assign16860_e22444 * locals.var_t1_dn9)) / (2.0 * assign16860_e22455))));
        locals.var_n_dn10 = (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * assign16860_e22447) + (assign16860_e22444 * locals.var_t1_dn10)) / (2.0 * assign16860_e22455))));
        locals.var_n_dn11 = (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * assign16860_e22447) + (assign16860_e22444 * locals.var_t1_dn11)) / (2.0 * assign16860_e22455))));

        let assign16870_e22460: f64 = (locals.var_n * locals.var_vt);
        locals.var_nvt = assign16870_e22460;
        locals.var_nvt_dn3 = (locals.var_n_dn3 * locals.var_vt);
        locals.var_nvt_dn4 = ((locals.var_n_dn4 * locals.var_vt) + (locals.var_n * locals.var_vt_dn4));
        locals.var_nvt_dn5 = ((locals.var_n_dn5 * locals.var_vt) + (locals.var_n * locals.var_vt_dn5));
        locals.var_nvt_dn6 = (locals.var_n_dn6 * locals.var_vt);
        locals.var_nvt_dn7 = (locals.var_n_dn7 * locals.var_vt);
        locals.var_nvt_dn8 = (locals.var_n_dn8 * locals.var_vt);
        locals.var_nvt_dn9 = (locals.var_n_dn9 * locals.var_vt);
        locals.var_nvt_dn10 = (locals.var_n_dn10 * locals.var_vt);
        locals.var_nvt_dn11 = (locals.var_n_dn11 * locals.var_vt);

        let assign16880_e22463: f64 = (1.0 / locals.var_nvt);
        locals.var_inv_nvt = assign16880_e22463;
        locals.var_inv_nvt_dn3 = (-(locals.var_nvt_dn3 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn4 = (-(locals.var_nvt_dn4 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn5 = (-(locals.var_nvt_dn5 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn6 = (-(locals.var_nvt_dn6 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn7 = (-(locals.var_nvt_dn7 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn8 = (-(locals.var_nvt_dn8 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn9 = (-(locals.var_nvt_dn9 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn10 = (-(locals.var_nvt_dn10 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn11 = (-(locals.var_nvt_dn11 / (locals.var_nvt * locals.var_nvt)));

        let assign16890_e22467: f64 = (locals.var_etab_i * locals.var_vbsx);
        let assign16890_e22468: f64 = (locals.var_eta0_a + assign16890_e22467);
        let assign16890_e22469: f64 = (-assign16890_e22468);
        let assign16890_e22471: f64 = (assign16890_e22469 * locals.var_vdsx);
        locals.var_dvth_dibl = assign16890_e22471;
        locals.var_dvth_dibl_dn3 = (((-(locals.var_eta0_a_dn3 + (locals.var_etab_i * locals.var_vbsx_dn3))) * locals.var_vdsx) + (assign16890_e22469 * locals.var_vdsx_dn3));
        locals.var_dvth_dibl_dn4 = (((-(locals.var_eta0_a_dn4 + (locals.var_etab_i * locals.var_vbsx_dn4))) * locals.var_vdsx) + (assign16890_e22469 * locals.var_vdsx_dn4));
        locals.var_dvth_dibl_dn5 = (((-(locals.var_eta0_a_dn5 + (locals.var_etab_i * locals.var_vbsx_dn5))) * locals.var_vdsx) + (assign16890_e22469 * locals.var_vdsx_dn5));
        locals.var_dvth_dibl_dn6 = (((-(locals.var_eta0_a_dn6 + (locals.var_etab_i * locals.var_vbsx_dn6))) * locals.var_vdsx) + (assign16890_e22469 * locals.var_vdsx_dn6));
        locals.var_dvth_dibl_dn7 = (((-(locals.var_eta0_a_dn7 + (locals.var_etab_i * locals.var_vbsx_dn7))) * locals.var_vdsx) + (assign16890_e22469 * locals.var_vdsx_dn7));
        locals.var_dvth_dibl_dn8 = (((-(locals.var_eta0_a_dn8 + (locals.var_etab_i * locals.var_vbsx_dn8))) * locals.var_vdsx) + (assign16890_e22469 * locals.var_vdsx_dn8));
        locals.var_dvth_dibl_dn9 = (((-(locals.var_eta0_a_dn9 + (locals.var_etab_i * locals.var_vbsx_dn9))) * locals.var_vdsx) + (assign16890_e22469 * locals.var_vdsx_dn9));
        locals.var_dvth_dibl_dn10 = (((-(locals.var_eta0_a_dn10 + (locals.var_etab_i * locals.var_vbsx_dn10))) * locals.var_vdsx) + (assign16890_e22469 * locals.var_vdsx_dn10));
        locals.var_dvth_dibl_dn11 = (((-(locals.var_eta0_a_dn11 + (locals.var_etab_i * locals.var_vbsx_dn11))) * locals.var_vdsx) + (assign16890_e22469 * locals.var_vdsx_dn11));

        let assign16900_e22475: f64 = locals.var_dvth_dibl;
        let assign16900_e22478: f64 = locals.var_dvth_dibl;
        let assign16900_e22481: f64 = locals.var_dvth_dibl;
        let assign16900_e22482: f64 = (assign16900_e22478 * assign16900_e22481);
        let assign16900_e22485: f64 = (0.25 * 5e-5);
        let assign16900_e22487: f64 = (assign16900_e22485 * 5e-5);
        let assign16900_e22488: f64 = (assign16900_e22482 + assign16900_e22487);
        let assign16900_e22489: f64 = (assign16900_e22488).sqrt();
        let assign16900_e22490: f64 = (assign16900_e22475 - assign16900_e22489);
        let assign16900_e22491: f64 = (0.5 * assign16900_e22490);
        let assign16900_e22494: f64 = (0.25 * 5e-5);
        let assign16900_e22495: f64 = (assign16900_e22491 + assign16900_e22494);
        locals.var_dvth_dibl = assign16900_e22495;
        locals.var_dvth_dibl_dn3 = (0.5 * (locals.var_dvth_dibl_dn3 - (((locals.var_dvth_dibl_dn3 * assign16900_e22481) + (assign16900_e22478 * locals.var_dvth_dibl_dn3)) / (2.0 * assign16900_e22489))));
        locals.var_dvth_dibl_dn4 = (0.5 * (locals.var_dvth_dibl_dn4 - (((locals.var_dvth_dibl_dn4 * assign16900_e22481) + (assign16900_e22478 * locals.var_dvth_dibl_dn4)) / (2.0 * assign16900_e22489))));
        locals.var_dvth_dibl_dn5 = (0.5 * (locals.var_dvth_dibl_dn5 - (((locals.var_dvth_dibl_dn5 * assign16900_e22481) + (assign16900_e22478 * locals.var_dvth_dibl_dn5)) / (2.0 * assign16900_e22489))));
        locals.var_dvth_dibl_dn6 = (0.5 * (locals.var_dvth_dibl_dn6 - (((locals.var_dvth_dibl_dn6 * assign16900_e22481) + (assign16900_e22478 * locals.var_dvth_dibl_dn6)) / (2.0 * assign16900_e22489))));
        locals.var_dvth_dibl_dn7 = (0.5 * (locals.var_dvth_dibl_dn7 - (((locals.var_dvth_dibl_dn7 * assign16900_e22481) + (assign16900_e22478 * locals.var_dvth_dibl_dn7)) / (2.0 * assign16900_e22489))));
        locals.var_dvth_dibl_dn8 = (0.5 * (locals.var_dvth_dibl_dn8 - (((locals.var_dvth_dibl_dn8 * assign16900_e22481) + (assign16900_e22478 * locals.var_dvth_dibl_dn8)) / (2.0 * assign16900_e22489))));
        locals.var_dvth_dibl_dn9 = (0.5 * (locals.var_dvth_dibl_dn9 - (((locals.var_dvth_dibl_dn9 * assign16900_e22481) + (assign16900_e22478 * locals.var_dvth_dibl_dn9)) / (2.0 * assign16900_e22489))));
        locals.var_dvth_dibl_dn10 = (0.5 * (locals.var_dvth_dibl_dn10 - (((locals.var_dvth_dibl_dn10 * assign16900_e22481) + (assign16900_e22478 * locals.var_dvth_dibl_dn10)) / (2.0 * assign16900_e22489))));
        locals.var_dvth_dibl_dn11 = (0.5 * (locals.var_dvth_dibl_dn11 - (((locals.var_dvth_dibl_dn11 * assign16900_e22481) + (assign16900_e22478 * locals.var_dvth_dibl_dn11)) / (2.0 * assign16900_e22489))));

        let assign16910_e22499: f64 = (p.p1077 / locals.var_leff);
        let assign16910_e22500: f64 = (locals.var_kt1_i + assign16910_e22499);
        let assign16910_e22503: f64 = (locals.var_kt2_i * locals.var_vbsx);
        let assign16910_e22504: f64 = (assign16910_e22500 + assign16910_e22503);
        let assign16910_e22507: f64 = (locals.var_tratio).powf(p.p1076);
        let assign16910_e22509: f64 = (assign16910_e22507 - 1.0);
        let assign16910_e22510: f64 = (assign16910_e22504 * assign16910_e22509);
        locals.var_dvth_temp = assign16910_e22510;
        locals.var_dvth_temp_dn3 = ((locals.var_kt2_i * locals.var_vbsx_dn3) * assign16910_e22509);
        locals.var_dvth_temp_dn4 = (((locals.var_kt2_i * locals.var_vbsx_dn4) * assign16910_e22509) + (assign16910_e22504 * if 0.0 == 0.0 && ((p.p1076) as f64).is_finite() && ((p.p1076) as f64).fract() == 0.0 { if p.p1076 == 0.0 { 0.0 } else { (p.p1076 * ((locals.var_tratio).powf(p.p1076 - 1.0) * locals.var_tratio_dn4)) } } else { (assign16910_e22507 * (p.p1076 * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_dvth_temp_dn5 = (((locals.var_kt2_i * locals.var_vbsx_dn5) * assign16910_e22509) + (assign16910_e22504 * if 0.0 == 0.0 && ((p.p1076) as f64).is_finite() && ((p.p1076) as f64).fract() == 0.0 { if p.p1076 == 0.0 { 0.0 } else { (p.p1076 * ((locals.var_tratio).powf(p.p1076 - 1.0) * locals.var_tratio_dn5)) } } else { (assign16910_e22507 * (p.p1076 * (locals.var_tratio_dn5 / locals.var_tratio))) }));
        locals.var_dvth_temp_dn6 = ((locals.var_kt2_i * locals.var_vbsx_dn6) * assign16910_e22509);
        locals.var_dvth_temp_dn7 = ((locals.var_kt2_i * locals.var_vbsx_dn7) * assign16910_e22509);
        locals.var_dvth_temp_dn8 = ((locals.var_kt2_i * locals.var_vbsx_dn8) * assign16910_e22509);
        locals.var_dvth_temp_dn9 = ((locals.var_kt2_i * locals.var_vbsx_dn9) * assign16910_e22509);
        locals.var_dvth_temp_dn10 = ((locals.var_kt2_i * locals.var_vbsx_dn10) * assign16910_e22509);
        locals.var_dvth_temp_dn11 = ((locals.var_kt2_i * locals.var_vbsx_dn11) * assign16910_e22509);

        let assign16920_e22513: f64 = if locals.var_dvtp0_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard490 = assign16920_e22513;

        let (assign16930_e22520, assign16930_e22520_d_n3, assign16930_e22520_d_n4, assign16930_e22520_d_n5, assign16930_e22520_d_n6, assign16930_e22520_d_n7, assign16930_e22520_d_n8, assign16930_e22520_d_n9, assign16930_e22520_d_n10, assign16930_e22520_d_n11,) = {
    if (locals.var_guard490 != 0.0) {
        let assign16930_e22516: f64 = (-locals.var_dvtp1_i);
        let assign16930_e22518: f64 = (assign16930_e22516 * locals.var_vdsx);
        (assign16930_e22518, (assign16930_e22516 * locals.var_vdsx_dn3), (assign16930_e22516 * locals.var_vdsx_dn4), (assign16930_e22516 * locals.var_vdsx_dn5), (assign16930_e22516 * locals.var_vdsx_dn6), (assign16930_e22516 * locals.var_vdsx_dn7), (assign16930_e22516 * locals.var_vdsx_dn8), (assign16930_e22516 * locals.var_vdsx_dn9), (assign16930_e22516 * locals.var_vdsx_dn10), (assign16930_e22516 * locals.var_vdsx_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign16930_e22520;
        locals.var_t0_dn3 = assign16930_e22520_d_n3;
        locals.var_t0_dn4 = assign16930_e22520_d_n4;
        locals.var_t0_dn5 = assign16930_e22520_d_n5;
        locals.var_t0_dn6 = assign16930_e22520_d_n6;
        locals.var_t0_dn7 = assign16930_e22520_d_n7;
        locals.var_t0_dn8 = assign16930_e22520_d_n8;
        locals.var_t0_dn9 = assign16930_e22520_d_n9;
        locals.var_t0_dn10 = assign16930_e22520_d_n10;
        locals.var_t0_dn11 = assign16930_e22520_d_n11;

        let assign16940_e22523: f64 = (-80.0);
        let assign16940_e22524: f64 = if locals.var_t0 < assign16940_e22523 { 1.0 } else { 0.0 };
        locals.var_guard491 = assign16940_e22524;

        let (assign16950_e22530, assign16950_e22530_d_n3, assign16950_e22530_d_n4, assign16950_e22530_d_n5, assign16950_e22530_d_n6, assign16950_e22530_d_n7, assign16950_e22530_d_n8, assign16950_e22530_d_n9, assign16950_e22530_d_n10, assign16950_e22530_d_n11,) = {
    if ((locals.var_guard490 != 0.0) && (locals.var_guard491 != 0.0)) {
        (1.804851387e-35, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign16950_e22530;
        locals.var_t2_dn3 = assign16950_e22530_d_n3;
        locals.var_t2_dn4 = assign16950_e22530_d_n4;
        locals.var_t2_dn5 = assign16950_e22530_d_n5;
        locals.var_t2_dn6 = assign16950_e22530_d_n6;
        locals.var_t2_dn7 = assign16950_e22530_d_n7;
        locals.var_t2_dn8 = assign16950_e22530_d_n8;
        locals.var_t2_dn9 = assign16950_e22530_d_n9;
        locals.var_t2_dn10 = assign16950_e22530_d_n10;
        locals.var_t2_dn11 = assign16950_e22530_d_n11;

        let (assign16960_e22538, assign16960_e22538_d_n3, assign16960_e22538_d_n4, assign16960_e22538_d_n5, assign16960_e22538_d_n6, assign16960_e22538_d_n7, assign16960_e22538_d_n8, assign16960_e22538_d_n9, assign16960_e22538_d_n10, assign16960_e22538_d_n11,) = {
    if ((locals.var_guard490 != 0.0) && (locals.var_guard491 == 0.0)) {
        let assign16960_e22536: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign16960_e22536, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign16960_e22538;
        locals.var_t2_dn3 = assign16960_e22538_d_n3;
        locals.var_t2_dn4 = assign16960_e22538_d_n4;
        locals.var_t2_dn5 = assign16960_e22538_d_n5;
        locals.var_t2_dn6 = assign16960_e22538_d_n6;
        locals.var_t2_dn7 = assign16960_e22538_d_n7;
        locals.var_t2_dn8 = assign16960_e22538_d_n8;
        locals.var_t2_dn9 = assign16960_e22538_d_n9;
        locals.var_t2_dn10 = assign16960_e22538_d_n10;
        locals.var_t2_dn11 = assign16960_e22538_d_n11;

        let (assign16970_e22548, assign16970_e22548_d_n3, assign16970_e22548_d_n4, assign16970_e22548_d_n5, assign16970_e22548_d_n6, assign16970_e22548_d_n7, assign16970_e22548_d_n8, assign16970_e22548_d_n9, assign16970_e22548_d_n10, assign16970_e22548_d_n11,) = {
    if (locals.var_guard490 != 0.0) {
        let assign16970_e22544: f64 = (1.0 + locals.var_t2);
        let assign16970_e22545: f64 = (locals.var_dvtp0_i * assign16970_e22544);
        let assign16970_e22546: f64 = (locals.var_leff + assign16970_e22545);
        (assign16970_e22546, (locals.var_dvtp0_i * locals.var_t2_dn3), (locals.var_dvtp0_i * locals.var_t2_dn4), (locals.var_dvtp0_i * locals.var_t2_dn5), (locals.var_dvtp0_i * locals.var_t2_dn6), (locals.var_dvtp0_i * locals.var_t2_dn7), (locals.var_dvtp0_i * locals.var_t2_dn8), (locals.var_dvtp0_i * locals.var_t2_dn9), (locals.var_dvtp0_i * locals.var_t2_dn10), (locals.var_dvtp0_i * locals.var_t2_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign16970_e22548;
        locals.var_t3_dn3 = assign16970_e22548_d_n3;
        locals.var_t3_dn4 = assign16970_e22548_d_n4;
        locals.var_t3_dn5 = assign16970_e22548_d_n5;
        locals.var_t3_dn6 = assign16970_e22548_d_n6;
        locals.var_t3_dn7 = assign16970_e22548_d_n7;
        locals.var_t3_dn8 = assign16970_e22548_d_n8;
        locals.var_t3_dn9 = assign16970_e22548_d_n9;
        locals.var_t3_dn10 = assign16970_e22548_d_n10;
        locals.var_t3_dn11 = assign16970_e22548_d_n11;

        let (assign16980_e22560, assign16980_e22560_d_n3, assign16980_e22560_d_n4, assign16980_e22560_d_n5, assign16980_e22560_d_n6, assign16980_e22560_d_n7, assign16980_e22560_d_n8, assign16980_e22560_d_n9, assign16980_e22560_d_n10, assign16980_e22560_d_n11,) = {
    if (locals.var_guard490 != 0.0) {
        let assign16980_e22551: f64 = (-locals.var_nvt);
        let assign16980_e22554: f64 = (locals.var_leff / locals.var_t3);
        let assign16980_e22556: f64 = (assign16980_e22554).max(1e-38);
        let assign16980_e22557: f64 = (assign16980_e22556).ln();
        let assign16980_e22558: f64 = (assign16980_e22551 * assign16980_e22557);
        (assign16980_e22558, (((-locals.var_nvt_dn3) * assign16980_e22557) + (assign16980_e22551 * (if assign16980_e22554 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign16980_e22556))), (((-locals.var_nvt_dn4) * assign16980_e22557) + (assign16980_e22551 * (if assign16980_e22554 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign16980_e22556))), (((-locals.var_nvt_dn5) * assign16980_e22557) + (assign16980_e22551 * (if assign16980_e22554 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign16980_e22556))), (((-locals.var_nvt_dn6) * assign16980_e22557) + (assign16980_e22551 * (if assign16980_e22554 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign16980_e22556))), (((-locals.var_nvt_dn7) * assign16980_e22557) + (assign16980_e22551 * (if assign16980_e22554 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign16980_e22556))), (((-locals.var_nvt_dn8) * assign16980_e22557) + (assign16980_e22551 * (if assign16980_e22554 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign16980_e22556))), (((-locals.var_nvt_dn9) * assign16980_e22557) + (assign16980_e22551 * (if assign16980_e22554 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign16980_e22556))), (((-locals.var_nvt_dn10) * assign16980_e22557) + (assign16980_e22551 * (if assign16980_e22554 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign16980_e22556))), (((-locals.var_nvt_dn11) * assign16980_e22557) + (assign16980_e22551 * (if assign16980_e22554 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign16980_e22556))),)
    } else {
        (locals.var_dvth_ldop, locals.var_dvth_ldop_dn3, locals.var_dvth_ldop_dn4, locals.var_dvth_ldop_dn5, locals.var_dvth_ldop_dn6, locals.var_dvth_ldop_dn7, locals.var_dvth_ldop_dn8, locals.var_dvth_ldop_dn9, locals.var_dvth_ldop_dn10, locals.var_dvth_ldop_dn11,)
    }
};
        locals.var_dvth_ldop = assign16980_e22560;
        locals.var_dvth_ldop_dn3 = assign16980_e22560_d_n3;
        locals.var_dvth_ldop_dn4 = assign16980_e22560_d_n4;
        locals.var_dvth_ldop_dn5 = assign16980_e22560_d_n5;
        locals.var_dvth_ldop_dn6 = assign16980_e22560_d_n6;
        locals.var_dvth_ldop_dn7 = assign16980_e22560_d_n7;
        locals.var_dvth_ldop_dn8 = assign16980_e22560_d_n8;
        locals.var_dvth_ldop_dn9 = assign16980_e22560_d_n9;
        locals.var_dvth_ldop_dn10 = assign16980_e22560_d_n10;
        locals.var_dvth_ldop_dn11 = assign16980_e22560_d_n11;

        let (assign16990_e22565, assign16990_e22565_d_n3, assign16990_e22565_d_n4, assign16990_e22565_d_n5, assign16990_e22565_d_n6, assign16990_e22565_d_n7, assign16990_e22565_d_n8, assign16990_e22565_d_n9, assign16990_e22565_d_n10, assign16990_e22565_d_n11,) = {
    if (locals.var_guard490 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvth_ldop, locals.var_dvth_ldop_dn3, locals.var_dvth_ldop_dn4, locals.var_dvth_ldop_dn5, locals.var_dvth_ldop_dn6, locals.var_dvth_ldop_dn7, locals.var_dvth_ldop_dn8, locals.var_dvth_ldop_dn9, locals.var_dvth_ldop_dn10, locals.var_dvth_ldop_dn11,)
    }
};
        locals.var_dvth_ldop = assign16990_e22565;
        locals.var_dvth_ldop_dn3 = assign16990_e22565_d_n3;
        locals.var_dvth_ldop_dn4 = assign16990_e22565_d_n4;
        locals.var_dvth_ldop_dn5 = assign16990_e22565_d_n5;
        locals.var_dvth_ldop_dn6 = assign16990_e22565_d_n6;
        locals.var_dvth_ldop_dn7 = assign16990_e22565_d_n7;
        locals.var_dvth_ldop_dn8 = assign16990_e22565_d_n8;
        locals.var_dvth_ldop_dn9 = assign16990_e22565_d_n9;
        locals.var_dvth_ldop_dn10 = assign16990_e22565_d_n10;
        locals.var_dvth_ldop_dn11 = assign16990_e22565_d_n11;

        let assign17000_e22570: f64 = (locals.var_leff).powf(locals.var_dvtp3_i);
        let assign17000_e22571: f64 = (locals.var_dvtp2_i / assign17000_e22570);
        let assign17000_e22572: f64 = (locals.var_dvtp5_i + assign17000_e22571);
        locals.var_t4 = assign17000_e22572;
        locals.var_t4_dn3 = 0.0;
        locals.var_t4_dn4 = 0.0;
        locals.var_t4_dn5 = 0.0;
        locals.var_t4_dn6 = 0.0;
        locals.var_t4_dn7 = 0.0;
        locals.var_t4_dn8 = 0.0;
        locals.var_t4_dn9 = 0.0;
        locals.var_t4_dn10 = 0.0;
        locals.var_t4_dn11 = 0.0;

        let assign17010_e22577: f64 = (locals.var_dvtp4_i * locals.var_vdsx);
        let assign17010_e22578: f64 = (assign17010_e22577).tanh();
        let assign17010_e22579: f64 = (locals.var_t4 * assign17010_e22578);
        let assign17010_e22580: f64 = (locals.var_dvth_ldop - assign17010_e22579);
        locals.var_dvth_ldop = assign17010_e22580;
        locals.var_dvth_ldop_dn3 = (locals.var_dvth_ldop_dn3 - ((locals.var_t4_dn3 * assign17010_e22578) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn3) / ((assign17010_e22577).cosh() * (assign17010_e22577).cosh())))));
        locals.var_dvth_ldop_dn4 = (locals.var_dvth_ldop_dn4 - ((locals.var_t4_dn4 * assign17010_e22578) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn4) / ((assign17010_e22577).cosh() * (assign17010_e22577).cosh())))));
        locals.var_dvth_ldop_dn5 = (locals.var_dvth_ldop_dn5 - ((locals.var_t4_dn5 * assign17010_e22578) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn5) / ((assign17010_e22577).cosh() * (assign17010_e22577).cosh())))));
        locals.var_dvth_ldop_dn6 = (locals.var_dvth_ldop_dn6 - ((locals.var_t4_dn6 * assign17010_e22578) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn6) / ((assign17010_e22577).cosh() * (assign17010_e22577).cosh())))));
        locals.var_dvth_ldop_dn7 = (locals.var_dvth_ldop_dn7 - ((locals.var_t4_dn7 * assign17010_e22578) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn7) / ((assign17010_e22577).cosh() * (assign17010_e22577).cosh())))));
        locals.var_dvth_ldop_dn8 = (locals.var_dvth_ldop_dn8 - ((locals.var_t4_dn8 * assign17010_e22578) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn8) / ((assign17010_e22577).cosh() * (assign17010_e22577).cosh())))));
        locals.var_dvth_ldop_dn9 = (locals.var_dvth_ldop_dn9 - ((locals.var_t4_dn9 * assign17010_e22578) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn9) / ((assign17010_e22577).cosh() * (assign17010_e22577).cosh())))));
        locals.var_dvth_ldop_dn10 = (locals.var_dvth_ldop_dn10 - ((locals.var_t4_dn10 * assign17010_e22578) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn10) / ((assign17010_e22577).cosh() * (assign17010_e22577).cosh())))));
        locals.var_dvth_ldop_dn11 = (locals.var_dvth_ldop_dn11 - ((locals.var_t4_dn11 * assign17010_e22578) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn11) / ((assign17010_e22577).cosh() * (assign17010_e22577).cosh())))));

        let assign17020_e22583: f64 = (locals.var_vfb_i + p.p25);
        locals.var_vfb_i = assign17020_e22583;
        locals.var_vfb_i_dn3 = locals.var_vfb_i_dn3;
        locals.var_vfb_i_dn4 = locals.var_vfb_i_dn4;
        locals.var_vfb_i_dn5 = locals.var_vfb_i_dn5;
        locals.var_vfb_i_dn6 = locals.var_vfb_i_dn6;
        locals.var_vfb_i_dn7 = locals.var_vfb_i_dn7;
        locals.var_vfb_i_dn8 = locals.var_vfb_i_dn8;
        locals.var_vfb_i_dn9 = locals.var_vfb_i_dn9;
        locals.var_vfb_i_dn10 = locals.var_vfb_i_dn10;
        locals.var_vfb_i_dn11 = locals.var_vfb_i_dn11;

        let assign17030_e22586: f64 = (locals.var_vg * locals.var_inv_nvt);
        locals.var_vg_1 = assign17030_e22586;
        locals.var_vg_1_dn3 = (locals.var_vg * locals.var_inv_nvt_dn3);
        locals.var_vg_1_dn4 = (locals.var_vg * locals.var_inv_nvt_dn4);
        locals.var_vg_1_dn5 = (locals.var_vg * locals.var_inv_nvt_dn5);
        locals.var_vg_1_dn6 = (locals.var_vg * locals.var_inv_nvt_dn6);
        locals.var_vg_1_dn7 = (locals.var_vg * locals.var_inv_nvt_dn7);
        locals.var_vg_1_dn8 = ((locals.var_vg_dn8 * locals.var_inv_nvt) + (locals.var_vg * locals.var_inv_nvt_dn8));
        locals.var_vg_1_dn9 = (locals.var_vg * locals.var_inv_nvt_dn9);
        locals.var_vg_1_dn10 = ((locals.var_vg_dn10 * locals.var_inv_nvt) + (locals.var_vg * locals.var_inv_nvt_dn10));
        locals.var_vg_1_dn11 = (locals.var_vg * locals.var_inv_nvt_dn11);

        let assign17040_e22589: f64 = (locals.var_vs * locals.var_inv_nvt);
        locals.var_vs_1 = assign17040_e22589;
        locals.var_vs_1_dn3 = (locals.var_vs * locals.var_inv_nvt_dn3);
        locals.var_vs_1_dn4 = (locals.var_vs * locals.var_inv_nvt_dn4);
        locals.var_vs_1_dn5 = (locals.var_vs * locals.var_inv_nvt_dn5);
        locals.var_vs_1_dn6 = ((locals.var_vs_dn6 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn6));
        locals.var_vs_1_dn7 = ((locals.var_vs_dn7 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn7));
        locals.var_vs_1_dn8 = (locals.var_vs * locals.var_inv_nvt_dn8);
        locals.var_vs_1_dn9 = (locals.var_vs * locals.var_inv_nvt_dn9);
        locals.var_vs_1_dn10 = ((locals.var_vs_dn10 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn10));
        locals.var_vs_1_dn11 = (locals.var_vs * locals.var_inv_nvt_dn11);

        let assign17050_e22592: f64 = (locals.var_vfb_i * locals.var_inv_nvt);
        locals.var_vfb = assign17050_e22592;
        locals.var_vfb_dn3 = ((locals.var_vfb_i_dn3 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn3));
        locals.var_vfb_dn4 = ((locals.var_vfb_i_dn4 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn4));
        locals.var_vfb_dn5 = ((locals.var_vfb_i_dn5 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn5));
        locals.var_vfb_dn6 = ((locals.var_vfb_i_dn6 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn6));
        locals.var_vfb_dn7 = ((locals.var_vfb_i_dn7 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn7));
        locals.var_vfb_dn8 = ((locals.var_vfb_i_dn8 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn8));
        locals.var_vfb_dn9 = ((locals.var_vfb_i_dn9 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn9));
        locals.var_vfb_dn10 = ((locals.var_vfb_i_dn10 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn10));
        locals.var_vfb_dn11 = ((locals.var_vfb_i_dn11 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn11));

        let assign17060_e22595: f64 = (locals.var_vfbb_i * locals.var_inv_nvt);
        locals.var_vfbb = assign17060_e22595;
        locals.var_vfbb_dn3 = (locals.var_vfbb_i * locals.var_inv_nvt_dn3);
        locals.var_vfbb_dn4 = (locals.var_vfbb_i * locals.var_inv_nvt_dn4);
        locals.var_vfbb_dn5 = (locals.var_vfbb_i * locals.var_inv_nvt_dn5);
        locals.var_vfbb_dn6 = (locals.var_vfbb_i * locals.var_inv_nvt_dn6);
        locals.var_vfbb_dn7 = (locals.var_vfbb_i * locals.var_inv_nvt_dn7);
        locals.var_vfbb_dn8 = (locals.var_vfbb_i * locals.var_inv_nvt_dn8);
        locals.var_vfbb_dn9 = (locals.var_vfbb_i * locals.var_inv_nvt_dn9);
        locals.var_vfbb_dn10 = (locals.var_vfbb_i * locals.var_inv_nvt_dn10);
        locals.var_vfbb_dn11 = (locals.var_vfbb_i * locals.var_inv_nvt_dn11);

        let assign17070_e22598: f64 = (locals.var_ve * locals.var_inv_nvt);
        locals.var_ve_1 = assign17070_e22598;
        locals.var_ve_1_dn3 = ((locals.var_ve_dn3 * locals.var_inv_nvt) + (locals.var_ve * locals.var_inv_nvt_dn3));
        locals.var_ve_1_dn4 = (locals.var_ve * locals.var_inv_nvt_dn4);
        locals.var_ve_1_dn5 = (locals.var_ve * locals.var_inv_nvt_dn5);
        locals.var_ve_1_dn6 = (locals.var_ve * locals.var_inv_nvt_dn6);
        locals.var_ve_1_dn7 = (locals.var_ve * locals.var_inv_nvt_dn7);
        locals.var_ve_1_dn8 = (locals.var_ve * locals.var_inv_nvt_dn8);
        locals.var_ve_1_dn9 = (locals.var_ve * locals.var_inv_nvt_dn9);
        locals.var_ve_1_dn10 = ((locals.var_ve_dn10 * locals.var_inv_nvt) + (locals.var_ve * locals.var_inv_nvt_dn10));
        locals.var_ve_1_dn11 = (locals.var_ve * locals.var_inv_nvt_dn11);

        let assign17080_e22602: f64 = (locals.var_sqrtphistvbs - locals.var_sqrtphist);
        let assign17080_e22603: f64 = (locals.var_k1_i * assign17080_e22602);
        let assign17080_e22606: f64 = (locals.var_k2_i * locals.var_vbsx);
        let assign17080_e22607: f64 = (assign17080_e22603 - assign17080_e22606);
        locals.var_dvth_vnud = assign17080_e22607;
        locals.var_dvth_vnud_dn3 = (((locals.var_k1_i_dn3 * assign17080_e22602) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn3 - locals.var_sqrtphist_dn3))) - ((locals.var_k2_i_dn3 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn3)));
        locals.var_dvth_vnud_dn4 = (((locals.var_k1_i_dn4 * assign17080_e22602) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn4 - locals.var_sqrtphist_dn4))) - ((locals.var_k2_i_dn4 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn4)));
        locals.var_dvth_vnud_dn5 = (((locals.var_k1_i_dn5 * assign17080_e22602) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn5 - locals.var_sqrtphist_dn5))) - ((locals.var_k2_i_dn5 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn5)));
        locals.var_dvth_vnud_dn6 = (((locals.var_k1_i_dn6 * assign17080_e22602) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn6 - locals.var_sqrtphist_dn6))) - ((locals.var_k2_i_dn6 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn6)));
        locals.var_dvth_vnud_dn7 = (((locals.var_k1_i_dn7 * assign17080_e22602) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn7 - locals.var_sqrtphist_dn7))) - ((locals.var_k2_i_dn7 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn7)));
        locals.var_dvth_vnud_dn8 = (((locals.var_k1_i_dn8 * assign17080_e22602) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn8 - locals.var_sqrtphist_dn8))) - ((locals.var_k2_i_dn8 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn8)));
        locals.var_dvth_vnud_dn9 = (((locals.var_k1_i_dn9 * assign17080_e22602) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn9 - locals.var_sqrtphist_dn9))) - ((locals.var_k2_i_dn9 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn9)));
        locals.var_dvth_vnud_dn10 = (((locals.var_k1_i_dn10 * assign17080_e22602) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn10 - locals.var_sqrtphist_dn10))) - ((locals.var_k2_i_dn10 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn10)));
        locals.var_dvth_vnud_dn11 = (((locals.var_k1_i_dn11 * assign17080_e22602) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn11 - locals.var_sqrtphist_dn11))) - ((locals.var_k2_i_dn11 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn11)));

        let assign17090_e22610: f64 = (locals.var_dvth_dibl + locals.var_dvth_ldop);
        let assign17090_e22612: f64 = (assign17090_e22610 + locals.var_dvth_vnud);
        let assign17090_e22614: f64 = (assign17090_e22612 - locals.var_dvth_temp);
        let assign17090_e22616: f64 = (assign17090_e22614 + locals.var_vth0_stress);
        let assign17090_e22618: f64 = (assign17090_e22616 + locals.var_vth0_well);
        locals.var_vth_shift = assign17090_e22618;
        locals.var_vth_shift_dn3 = (((((locals.var_dvth_dibl_dn3 + locals.var_dvth_ldop_dn3) + locals.var_dvth_vnud_dn3) - locals.var_dvth_temp_dn3) + locals.var_vth0_stress_dn3) + locals.var_vth0_well_dn3);
        locals.var_vth_shift_dn4 = (((((locals.var_dvth_dibl_dn4 + locals.var_dvth_ldop_dn4) + locals.var_dvth_vnud_dn4) - locals.var_dvth_temp_dn4) + locals.var_vth0_stress_dn4) + locals.var_vth0_well_dn4);
        locals.var_vth_shift_dn5 = (((((locals.var_dvth_dibl_dn5 + locals.var_dvth_ldop_dn5) + locals.var_dvth_vnud_dn5) - locals.var_dvth_temp_dn5) + locals.var_vth0_stress_dn5) + locals.var_vth0_well_dn5);
        locals.var_vth_shift_dn6 = (((((locals.var_dvth_dibl_dn6 + locals.var_dvth_ldop_dn6) + locals.var_dvth_vnud_dn6) - locals.var_dvth_temp_dn6) + locals.var_vth0_stress_dn6) + locals.var_vth0_well_dn6);
        locals.var_vth_shift_dn7 = (((((locals.var_dvth_dibl_dn7 + locals.var_dvth_ldop_dn7) + locals.var_dvth_vnud_dn7) - locals.var_dvth_temp_dn7) + locals.var_vth0_stress_dn7) + locals.var_vth0_well_dn7);
        locals.var_vth_shift_dn8 = (((((locals.var_dvth_dibl_dn8 + locals.var_dvth_ldop_dn8) + locals.var_dvth_vnud_dn8) - locals.var_dvth_temp_dn8) + locals.var_vth0_stress_dn8) + locals.var_vth0_well_dn8);
        locals.var_vth_shift_dn9 = (((((locals.var_dvth_dibl_dn9 + locals.var_dvth_ldop_dn9) + locals.var_dvth_vnud_dn9) - locals.var_dvth_temp_dn9) + locals.var_vth0_stress_dn9) + locals.var_vth0_well_dn9);
        locals.var_vth_shift_dn10 = (((((locals.var_dvth_dibl_dn10 + locals.var_dvth_ldop_dn10) + locals.var_dvth_vnud_dn10) - locals.var_dvth_temp_dn10) + locals.var_vth0_stress_dn10) + locals.var_vth0_well_dn10);
        locals.var_vth_shift_dn11 = (((((locals.var_dvth_dibl_dn11 + locals.var_dvth_ldop_dn11) + locals.var_dvth_vnud_dn11) - locals.var_dvth_temp_dn11) + locals.var_vth0_stress_dn11) + locals.var_vth0_well_dn11);

        let assign17100_e22621: f64 = (locals.var_vg_1 - locals.var_vfb);
        let assign17100_e22624: f64 = (locals.var_vth_shift * locals.var_inv_nvt);
        let assign17100_e22625: f64 = (assign17100_e22621 - assign17100_e22624);
        locals.var_vgfb = assign17100_e22625;
        locals.var_vgfb_dn3 = ((locals.var_vg_1_dn3 - locals.var_vfb_dn3) - ((locals.var_vth_shift_dn3 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn3)));
        locals.var_vgfb_dn4 = ((locals.var_vg_1_dn4 - locals.var_vfb_dn4) - ((locals.var_vth_shift_dn4 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn4)));
        locals.var_vgfb_dn5 = ((locals.var_vg_1_dn5 - locals.var_vfb_dn5) - ((locals.var_vth_shift_dn5 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn5)));
        locals.var_vgfb_dn6 = ((locals.var_vg_1_dn6 - locals.var_vfb_dn6) - ((locals.var_vth_shift_dn6 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn6)));
        locals.var_vgfb_dn7 = ((locals.var_vg_1_dn7 - locals.var_vfb_dn7) - ((locals.var_vth_shift_dn7 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn7)));
        locals.var_vgfb_dn8 = ((locals.var_vg_1_dn8 - locals.var_vfb_dn8) - ((locals.var_vth_shift_dn8 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn8)));
        locals.var_vgfb_dn9 = ((locals.var_vg_1_dn9 - locals.var_vfb_dn9) - ((locals.var_vth_shift_dn9 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn9)));
        locals.var_vgfb_dn10 = ((locals.var_vg_1_dn10 - locals.var_vfb_dn10) - ((locals.var_vth_shift_dn10 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn10)));
        locals.var_vgfb_dn11 = ((locals.var_vg_1_dn11 - locals.var_vfb_dn11) - ((locals.var_vth_shift_dn11 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn11)));

        let assign17110_e22629: f64 = (locals.var_vbsx * locals.var_inv_nvt);
        let assign17110_e22630: f64 = (locals.var_vg_1 - assign17110_e22629);
        let assign17110_e22632: f64 = (assign17110_e22630 - locals.var_vfb);
        let assign17110_e22635: f64 = (locals.var_vth_shift * locals.var_inv_nvt);
        let assign17110_e22636: f64 = (assign17110_e22632 - assign17110_e22635);
        locals.var_vgsfb = assign17110_e22636;
        locals.var_vgsfb_dn3 = (((locals.var_vg_1_dn3 - ((locals.var_vbsx_dn3 * locals.var_inv_nvt) + (locals.var_vbsx * locals.var_inv_nvt_dn3))) - locals.var_vfb_dn3) - ((locals.var_vth_shift_dn3 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn3)));
        locals.var_vgsfb_dn4 = (((locals.var_vg_1_dn4 - ((locals.var_vbsx_dn4 * locals.var_inv_nvt) + (locals.var_vbsx * locals.var_inv_nvt_dn4))) - locals.var_vfb_dn4) - ((locals.var_vth_shift_dn4 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn4)));
        locals.var_vgsfb_dn5 = (((locals.var_vg_1_dn5 - ((locals.var_vbsx_dn5 * locals.var_inv_nvt) + (locals.var_vbsx * locals.var_inv_nvt_dn5))) - locals.var_vfb_dn5) - ((locals.var_vth_shift_dn5 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn5)));
        locals.var_vgsfb_dn6 = (((locals.var_vg_1_dn6 - ((locals.var_vbsx_dn6 * locals.var_inv_nvt) + (locals.var_vbsx * locals.var_inv_nvt_dn6))) - locals.var_vfb_dn6) - ((locals.var_vth_shift_dn6 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn6)));
        locals.var_vgsfb_dn7 = (((locals.var_vg_1_dn7 - ((locals.var_vbsx_dn7 * locals.var_inv_nvt) + (locals.var_vbsx * locals.var_inv_nvt_dn7))) - locals.var_vfb_dn7) - ((locals.var_vth_shift_dn7 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn7)));
        locals.var_vgsfb_dn8 = (((locals.var_vg_1_dn8 - ((locals.var_vbsx_dn8 * locals.var_inv_nvt) + (locals.var_vbsx * locals.var_inv_nvt_dn8))) - locals.var_vfb_dn8) - ((locals.var_vth_shift_dn8 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn8)));
        locals.var_vgsfb_dn9 = (((locals.var_vg_1_dn9 - ((locals.var_vbsx_dn9 * locals.var_inv_nvt) + (locals.var_vbsx * locals.var_inv_nvt_dn9))) - locals.var_vfb_dn9) - ((locals.var_vth_shift_dn9 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn9)));
        locals.var_vgsfb_dn10 = (((locals.var_vg_1_dn10 - ((locals.var_vbsx_dn10 * locals.var_inv_nvt) + (locals.var_vbsx * locals.var_inv_nvt_dn10))) - locals.var_vfb_dn10) - ((locals.var_vth_shift_dn10 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn10)));
        locals.var_vgsfb_dn11 = (((locals.var_vg_1_dn11 - ((locals.var_vbsx_dn11 * locals.var_inv_nvt) + (locals.var_vbsx * locals.var_inv_nvt_dn11))) - locals.var_vfb_dn11) - ((locals.var_vth_shift_dn11 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn11)));

        let assign17120_e22639: f64 = (locals.var_vg_1 - locals.var_vfb);
        let assign17120_e22642: f64 = (locals.var_vth_shift * locals.var_inv_nvt);
        let assign17120_e22643: f64 = (assign17120_e22639 - assign17120_e22642);
        locals.var_vgfb1 = assign17120_e22643;
        locals.var_vgfb1_dn3 = ((locals.var_vg_1_dn3 - locals.var_vfb_dn3) - ((locals.var_vth_shift_dn3 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn3)));
        locals.var_vgfb1_dn4 = ((locals.var_vg_1_dn4 - locals.var_vfb_dn4) - ((locals.var_vth_shift_dn4 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn4)));
        locals.var_vgfb1_dn5 = ((locals.var_vg_1_dn5 - locals.var_vfb_dn5) - ((locals.var_vth_shift_dn5 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn5)));
        locals.var_vgfb1_dn6 = ((locals.var_vg_1_dn6 - locals.var_vfb_dn6) - ((locals.var_vth_shift_dn6 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn6)));
        locals.var_vgfb1_dn7 = ((locals.var_vg_1_dn7 - locals.var_vfb_dn7) - ((locals.var_vth_shift_dn7 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn7)));
        locals.var_vgfb1_dn8 = ((locals.var_vg_1_dn8 - locals.var_vfb_dn8) - ((locals.var_vth_shift_dn8 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn8)));
        locals.var_vgfb1_dn9 = ((locals.var_vg_1_dn9 - locals.var_vfb_dn9) - ((locals.var_vth_shift_dn9 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn9)));
        locals.var_vgfb1_dn10 = ((locals.var_vg_1_dn10 - locals.var_vfb_dn10) - ((locals.var_vth_shift_dn10 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn10)));
        locals.var_vgfb1_dn11 = ((locals.var_vg_1_dn11 - locals.var_vfb_dn11) - ((locals.var_vth_shift_dn11 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn11)));

        let assign17130_e22646: f64 = (locals.var_ve_1 - locals.var_vfbb);
        locals.var_vgfbb = assign17130_e22646;
        locals.var_vgfbb_dn3 = (locals.var_ve_1_dn3 - locals.var_vfbb_dn3);
        locals.var_vgfbb_dn4 = (locals.var_ve_1_dn4 - locals.var_vfbb_dn4);
        locals.var_vgfbb_dn5 = (locals.var_ve_1_dn5 - locals.var_vfbb_dn5);
        locals.var_vgfbb_dn6 = (locals.var_ve_1_dn6 - locals.var_vfbb_dn6);
        locals.var_vgfbb_dn7 = (locals.var_ve_1_dn7 - locals.var_vfbb_dn7);
        locals.var_vgfbb_dn8 = (locals.var_ve_1_dn8 - locals.var_vfbb_dn8);
        locals.var_vgfbb_dn9 = (locals.var_ve_1_dn9 - locals.var_vfbb_dn9);
        locals.var_vgfbb_dn10 = (locals.var_ve_1_dn10 - locals.var_vfbb_dn10);
        locals.var_vgfbb_dn11 = (locals.var_ve_1_dn11 - locals.var_vfbb_dn11);

        let assign17140_e22649: f64 = (2.0 * 1.602176462e-19);
        let assign17140_e22651: f64 = (assign17140_e22649 * locals.var_epssi);
        let assign17140_e22653: f64 = (assign17140_e22651 * locals.var_ndep_i);
        let assign17140_e22655: f64 = (assign17140_e22653 * locals.var_inv_vt);
        let assign17140_e22656: f64 = (assign17140_e22655).sqrt();
        let assign17140_e22658: f64 = (assign17140_e22656 / locals.var_cox);
        locals.var_gam = assign17140_e22658;
        locals.var_gam_dn3 = ((((assign17140_e22651 * locals.var_ndep_i_dn3) * locals.var_inv_vt) / (2.0 * assign17140_e22656)) / locals.var_cox);
        locals.var_gam_dn4 = (((((assign17140_e22651 * locals.var_ndep_i_dn4) * locals.var_inv_vt) + (assign17140_e22653 * locals.var_inv_vt_dn4)) / (2.0 * assign17140_e22656)) / locals.var_cox);
        locals.var_gam_dn5 = (((((assign17140_e22651 * locals.var_ndep_i_dn5) * locals.var_inv_vt) + (assign17140_e22653 * locals.var_inv_vt_dn5)) / (2.0 * assign17140_e22656)) / locals.var_cox);
        locals.var_gam_dn6 = ((((assign17140_e22651 * locals.var_ndep_i_dn6) * locals.var_inv_vt) / (2.0 * assign17140_e22656)) / locals.var_cox);
        locals.var_gam_dn7 = ((((assign17140_e22651 * locals.var_ndep_i_dn7) * locals.var_inv_vt) / (2.0 * assign17140_e22656)) / locals.var_cox);
        locals.var_gam_dn8 = ((((assign17140_e22651 * locals.var_ndep_i_dn8) * locals.var_inv_vt) / (2.0 * assign17140_e22656)) / locals.var_cox);
        locals.var_gam_dn9 = ((((assign17140_e22651 * locals.var_ndep_i_dn9) * locals.var_inv_vt) / (2.0 * assign17140_e22656)) / locals.var_cox);
        locals.var_gam_dn10 = ((((assign17140_e22651 * locals.var_ndep_i_dn10) * locals.var_inv_vt) / (2.0 * assign17140_e22656)) / locals.var_cox);
        locals.var_gam_dn11 = ((((assign17140_e22651 * locals.var_ndep_i_dn11) * locals.var_inv_vt) / (2.0 * assign17140_e22656)) / locals.var_cox);

        locals.var_q_vth = 0.5;

    }

    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign17160_e22663: f64 = (2.0 * locals.var_phib);
        let assign17160_e22666: f64 = (locals.var_vs * locals.var_inv_vt);
        let assign17160_e22667: f64 = (assign17160_e22663 + assign17160_e22666);
        let assign17160_e22670: f64 = (2.0 * locals.var_phib);
        let assign17160_e22673: f64 = (locals.var_vs * locals.var_inv_vt);
        let assign17160_e22674: f64 = (assign17160_e22670 + assign17160_e22673);
        let assign17160_e22677: f64 = (2.0 * locals.var_phib);
        let assign17160_e22680: f64 = (locals.var_vs * locals.var_inv_vt);
        let assign17160_e22681: f64 = (assign17160_e22677 + assign17160_e22680);
        let assign17160_e22682: f64 = (assign17160_e22674 * assign17160_e22681);
        let assign17160_e22685: f64 = (4.0 * 0.001);
        let assign17160_e22687: f64 = (assign17160_e22685 * 0.001);
        let assign17160_e22688: f64 = (assign17160_e22682 + assign17160_e22687);
        let assign17160_e22689: f64 = (assign17160_e22688).sqrt();
        let assign17160_e22690: f64 = (assign17160_e22667 + assign17160_e22689);
        let assign17160_e22691: f64 = (0.5 * assign17160_e22690);
        locals.var_t0 = assign17160_e22691;
        locals.var_t0_dn3 = (0.5 * ((2.0 * locals.var_phib_dn3) + ((((2.0 * locals.var_phib_dn3) * assign17160_e22681) + (assign17160_e22674 * (2.0 * locals.var_phib_dn3))) / (2.0 * assign17160_e22689))));
        locals.var_t0_dn4 = (0.5 * (((2.0 * locals.var_phib_dn4) + (locals.var_vs * locals.var_inv_vt_dn4)) + (((((2.0 * locals.var_phib_dn4) + (locals.var_vs * locals.var_inv_vt_dn4)) * assign17160_e22681) + (assign17160_e22674 * ((2.0 * locals.var_phib_dn4) + (locals.var_vs * locals.var_inv_vt_dn4)))) / (2.0 * assign17160_e22689))));
        locals.var_t0_dn5 = (0.5 * (((2.0 * locals.var_phib_dn5) + (locals.var_vs * locals.var_inv_vt_dn5)) + (((((2.0 * locals.var_phib_dn5) + (locals.var_vs * locals.var_inv_vt_dn5)) * assign17160_e22681) + (assign17160_e22674 * ((2.0 * locals.var_phib_dn5) + (locals.var_vs * locals.var_inv_vt_dn5)))) / (2.0 * assign17160_e22689))));
        locals.var_t0_dn6 = (0.5 * (((2.0 * locals.var_phib_dn6) + (locals.var_vs_dn6 * locals.var_inv_vt)) + (((((2.0 * locals.var_phib_dn6) + (locals.var_vs_dn6 * locals.var_inv_vt)) * assign17160_e22681) + (assign17160_e22674 * ((2.0 * locals.var_phib_dn6) + (locals.var_vs_dn6 * locals.var_inv_vt)))) / (2.0 * assign17160_e22689))));
        locals.var_t0_dn7 = (0.5 * (((2.0 * locals.var_phib_dn7) + (locals.var_vs_dn7 * locals.var_inv_vt)) + (((((2.0 * locals.var_phib_dn7) + (locals.var_vs_dn7 * locals.var_inv_vt)) * assign17160_e22681) + (assign17160_e22674 * ((2.0 * locals.var_phib_dn7) + (locals.var_vs_dn7 * locals.var_inv_vt)))) / (2.0 * assign17160_e22689))));
        locals.var_t0_dn8 = (0.5 * ((2.0 * locals.var_phib_dn8) + ((((2.0 * locals.var_phib_dn8) * assign17160_e22681) + (assign17160_e22674 * (2.0 * locals.var_phib_dn8))) / (2.0 * assign17160_e22689))));
        locals.var_t0_dn9 = (0.5 * ((2.0 * locals.var_phib_dn9) + ((((2.0 * locals.var_phib_dn9) * assign17160_e22681) + (assign17160_e22674 * (2.0 * locals.var_phib_dn9))) / (2.0 * assign17160_e22689))));
        locals.var_t0_dn10 = (0.5 * (((2.0 * locals.var_phib_dn10) + (locals.var_vs_dn10 * locals.var_inv_vt)) + (((((2.0 * locals.var_phib_dn10) + (locals.var_vs_dn10 * locals.var_inv_vt)) * assign17160_e22681) + (assign17160_e22674 * ((2.0 * locals.var_phib_dn10) + (locals.var_vs_dn10 * locals.var_inv_vt)))) / (2.0 * assign17160_e22689))));
        locals.var_t0_dn11 = (0.5 * ((2.0 * locals.var_phib_dn11) + ((((2.0 * locals.var_phib_dn11) * assign17160_e22681) + (assign17160_e22674 * (2.0 * locals.var_phib_dn11))) / (2.0 * assign17160_e22689))));

        let assign17170_e22696: f64 = (locals.var_t0).sqrt();
        let assign17170_e22697: f64 = (2.0 * assign17170_e22696);
        let assign17170_e22698: f64 = (locals.var_gam / assign17170_e22697);
        let assign17170_e22699: f64 = (1.0 + assign17170_e22698);
        locals.var_nq = assign17170_e22699;
        locals.var_nq_dn3 = (((locals.var_gam_dn3 * assign17170_e22697) - (locals.var_gam * (2.0 * (locals.var_t0_dn3 / (2.0 * assign17170_e22696))))) / (assign17170_e22697 * assign17170_e22697));
        locals.var_nq_dn4 = (((locals.var_gam_dn4 * assign17170_e22697) - (locals.var_gam * (2.0 * (locals.var_t0_dn4 / (2.0 * assign17170_e22696))))) / (assign17170_e22697 * assign17170_e22697));
        locals.var_nq_dn5 = (((locals.var_gam_dn5 * assign17170_e22697) - (locals.var_gam * (2.0 * (locals.var_t0_dn5 / (2.0 * assign17170_e22696))))) / (assign17170_e22697 * assign17170_e22697));
        locals.var_nq_dn6 = (((locals.var_gam_dn6 * assign17170_e22697) - (locals.var_gam * (2.0 * (locals.var_t0_dn6 / (2.0 * assign17170_e22696))))) / (assign17170_e22697 * assign17170_e22697));
        locals.var_nq_dn7 = (((locals.var_gam_dn7 * assign17170_e22697) - (locals.var_gam * (2.0 * (locals.var_t0_dn7 / (2.0 * assign17170_e22696))))) / (assign17170_e22697 * assign17170_e22697));
        locals.var_nq_dn8 = (((locals.var_gam_dn8 * assign17170_e22697) - (locals.var_gam * (2.0 * (locals.var_t0_dn8 / (2.0 * assign17170_e22696))))) / (assign17170_e22697 * assign17170_e22697));
        locals.var_nq_dn9 = (((locals.var_gam_dn9 * assign17170_e22697) - (locals.var_gam * (2.0 * (locals.var_t0_dn9 / (2.0 * assign17170_e22696))))) / (assign17170_e22697 * assign17170_e22697));
        locals.var_nq_dn10 = (((locals.var_gam_dn10 * assign17170_e22697) - (locals.var_gam * (2.0 * (locals.var_t0_dn10 / (2.0 * assign17170_e22696))))) / (assign17170_e22697 * assign17170_e22697));
        locals.var_nq_dn11 = (((locals.var_gam_dn11 * assign17170_e22697) - (locals.var_gam * (2.0 * (locals.var_t0_dn11 / (2.0 * assign17170_e22696))))) / (assign17170_e22697 * assign17170_e22697));

        let assign17180_e22703: f64 = (locals.var_vs * locals.var_inv_vt);
        let assign17180_e22706: f64 = (2.0 * locals.var_phib);
        let assign17180_e22707: f64 = (assign17180_e22703 + assign17180_e22706);
        let assign17180_e22710: f64 = (locals.var_q_vth).max(1e-38);
        let assign17180_e22711: f64 = (assign17180_e22710).ln();
        let assign17180_e22712: f64 = (assign17180_e22707 + assign17180_e22711);
        let assign17180_e22715: f64 = (2.0 * locals.var_q_vth);
        let assign17180_e22716: f64 = (assign17180_e22712 + assign17180_e22715);
        let assign17180_e22719: f64 = (2.0 * locals.var_nq);
        let assign17180_e22721: f64 = (assign17180_e22719 / locals.var_gam);
        let assign17180_e22724: f64 = (2.0 * locals.var_q_vth);
        let assign17180_e22726: f64 = (assign17180_e22724 * locals.var_nq);
        let assign17180_e22728: f64 = (assign17180_e22726 / locals.var_gam);
        let assign17180_e22731: f64 = (locals.var_t0).sqrt();
        let assign17180_e22732: f64 = (2.0 * assign17180_e22731);
        let assign17180_e22733: f64 = (assign17180_e22728 + assign17180_e22732);
        let assign17180_e22734: f64 = (assign17180_e22721 * assign17180_e22733);
        let assign17180_e22736: f64 = (assign17180_e22734).max(1e-38);
        let assign17180_e22737: f64 = (assign17180_e22736).ln();
        let assign17180_e22738: f64 = (assign17180_e22716 + assign17180_e22737);
        let assign17180_e22741: f64 = (locals.var_vs * locals.var_inv_vt);
        let assign17180_e22744: f64 = (2.0 * locals.var_phib);
        let assign17180_e22745: f64 = (assign17180_e22741 + assign17180_e22744);
        let assign17180_e22748: f64 = (locals.var_q_vth).max(1e-38);
        let assign17180_e22749: f64 = (assign17180_e22748).ln();
        let assign17180_e22750: f64 = (assign17180_e22745 + assign17180_e22749);
        let assign17180_e22753: f64 = (2.0 * locals.var_q_vth);
        let assign17180_e22754: f64 = (assign17180_e22750 + assign17180_e22753);
        let assign17180_e22757: f64 = (2.0 * locals.var_nq);
        let assign17180_e22759: f64 = (assign17180_e22757 / locals.var_gam);
        let assign17180_e22762: f64 = (2.0 * locals.var_q_vth);
        let assign17180_e22764: f64 = (assign17180_e22762 * locals.var_nq);
        let assign17180_e22766: f64 = (assign17180_e22764 / locals.var_gam);
        let assign17180_e22769: f64 = (locals.var_t0).sqrt();
        let assign17180_e22770: f64 = (2.0 * assign17180_e22769);
        let assign17180_e22771: f64 = (assign17180_e22766 + assign17180_e22770);
        let assign17180_e22772: f64 = (assign17180_e22759 * assign17180_e22771);
        let assign17180_e22774: f64 = (assign17180_e22772).max(1e-38);
        let assign17180_e22775: f64 = (assign17180_e22774).ln();
        let assign17180_e22776: f64 = (assign17180_e22754 + assign17180_e22775);
        let assign17180_e22779: f64 = (locals.var_vs * locals.var_inv_vt);
        let assign17180_e22782: f64 = (2.0 * locals.var_phib);
        let assign17180_e22783: f64 = (assign17180_e22779 + assign17180_e22782);
        let assign17180_e22786: f64 = (locals.var_q_vth).max(1e-38);
        let assign17180_e22787: f64 = (assign17180_e22786).ln();
        let assign17180_e22788: f64 = (assign17180_e22783 + assign17180_e22787);
        let assign17180_e22791: f64 = (2.0 * locals.var_q_vth);
        let assign17180_e22792: f64 = (assign17180_e22788 + assign17180_e22791);
        let assign17180_e22795: f64 = (2.0 * locals.var_nq);
        let assign17180_e22797: f64 = (assign17180_e22795 / locals.var_gam);
        let assign17180_e22800: f64 = (2.0 * locals.var_q_vth);
        let assign17180_e22802: f64 = (assign17180_e22800 * locals.var_nq);
        let assign17180_e22804: f64 = (assign17180_e22802 / locals.var_gam);
        let assign17180_e22807: f64 = (locals.var_t0).sqrt();
        let assign17180_e22808: f64 = (2.0 * assign17180_e22807);
        let assign17180_e22809: f64 = (assign17180_e22804 + assign17180_e22808);
        let assign17180_e22810: f64 = (assign17180_e22797 * assign17180_e22809);
        let assign17180_e22812: f64 = (assign17180_e22810).max(1e-38);
        let assign17180_e22813: f64 = (assign17180_e22812).ln();
        let assign17180_e22814: f64 = (assign17180_e22792 + assign17180_e22813);
        let assign17180_e22815: f64 = (assign17180_e22776 * assign17180_e22814);
        let assign17180_e22818: f64 = (4.0 * 0.001);
        let assign17180_e22820: f64 = (assign17180_e22818 * 0.001);
        let assign17180_e22821: f64 = (assign17180_e22815 + assign17180_e22820);
        let assign17180_e22822: f64 = (assign17180_e22821).sqrt();
        let assign17180_e22823: f64 = (assign17180_e22738 + assign17180_e22822);
        let assign17180_e22824: f64 = (0.5 * assign17180_e22823);
        locals.var_psip_th = assign17180_e22824;
        locals.var_psip_th_dn3 = (0.5 * (((2.0 * locals.var_phib_dn3) + (if assign17180_e22734 >= 1e-38 { ((((((2.0 * locals.var_nq_dn3) * locals.var_gam) - (assign17180_e22719 * locals.var_gam_dn3)) / (locals.var_gam * locals.var_gam)) * assign17180_e22733) + (assign17180_e22721 * (((((assign17180_e22724 * locals.var_nq_dn3) * locals.var_gam) - (assign17180_e22726 * locals.var_gam_dn3)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn3 / (2.0 * assign17180_e22731)))))) } else { 0.0 } / assign17180_e22736)) + (((((2.0 * locals.var_phib_dn3) + (if assign17180_e22772 >= 1e-38 { ((((((2.0 * locals.var_nq_dn3) * locals.var_gam) - (assign17180_e22757 * locals.var_gam_dn3)) / (locals.var_gam * locals.var_gam)) * assign17180_e22771) + (assign17180_e22759 * (((((assign17180_e22762 * locals.var_nq_dn3) * locals.var_gam) - (assign17180_e22764 * locals.var_gam_dn3)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn3 / (2.0 * assign17180_e22769)))))) } else { 0.0 } / assign17180_e22774)) * assign17180_e22814) + (assign17180_e22776 * ((2.0 * locals.var_phib_dn3) + (if assign17180_e22810 >= 1e-38 { ((((((2.0 * locals.var_nq_dn3) * locals.var_gam) - (assign17180_e22795 * locals.var_gam_dn3)) / (locals.var_gam * locals.var_gam)) * assign17180_e22809) + (assign17180_e22797 * (((((assign17180_e22800 * locals.var_nq_dn3) * locals.var_gam) - (assign17180_e22802 * locals.var_gam_dn3)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn3 / (2.0 * assign17180_e22807)))))) } else { 0.0 } / assign17180_e22812)))) / (2.0 * assign17180_e22822))));
        locals.var_psip_th_dn4 = (0.5 * ((((locals.var_vs * locals.var_inv_vt_dn4) + (2.0 * locals.var_phib_dn4)) + (if assign17180_e22734 >= 1e-38 { ((((((2.0 * locals.var_nq_dn4) * locals.var_gam) - (assign17180_e22719 * locals.var_gam_dn4)) / (locals.var_gam * locals.var_gam)) * assign17180_e22733) + (assign17180_e22721 * (((((assign17180_e22724 * locals.var_nq_dn4) * locals.var_gam) - (assign17180_e22726 * locals.var_gam_dn4)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn4 / (2.0 * assign17180_e22731)))))) } else { 0.0 } / assign17180_e22736)) + ((((((locals.var_vs * locals.var_inv_vt_dn4) + (2.0 * locals.var_phib_dn4)) + (if assign17180_e22772 >= 1e-38 { ((((((2.0 * locals.var_nq_dn4) * locals.var_gam) - (assign17180_e22757 * locals.var_gam_dn4)) / (locals.var_gam * locals.var_gam)) * assign17180_e22771) + (assign17180_e22759 * (((((assign17180_e22762 * locals.var_nq_dn4) * locals.var_gam) - (assign17180_e22764 * locals.var_gam_dn4)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn4 / (2.0 * assign17180_e22769)))))) } else { 0.0 } / assign17180_e22774)) * assign17180_e22814) + (assign17180_e22776 * (((locals.var_vs * locals.var_inv_vt_dn4) + (2.0 * locals.var_phib_dn4)) + (if assign17180_e22810 >= 1e-38 { ((((((2.0 * locals.var_nq_dn4) * locals.var_gam) - (assign17180_e22795 * locals.var_gam_dn4)) / (locals.var_gam * locals.var_gam)) * assign17180_e22809) + (assign17180_e22797 * (((((assign17180_e22800 * locals.var_nq_dn4) * locals.var_gam) - (assign17180_e22802 * locals.var_gam_dn4)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn4 / (2.0 * assign17180_e22807)))))) } else { 0.0 } / assign17180_e22812)))) / (2.0 * assign17180_e22822))));
        locals.var_psip_th_dn5 = (0.5 * ((((locals.var_vs * locals.var_inv_vt_dn5) + (2.0 * locals.var_phib_dn5)) + (if assign17180_e22734 >= 1e-38 { ((((((2.0 * locals.var_nq_dn5) * locals.var_gam) - (assign17180_e22719 * locals.var_gam_dn5)) / (locals.var_gam * locals.var_gam)) * assign17180_e22733) + (assign17180_e22721 * (((((assign17180_e22724 * locals.var_nq_dn5) * locals.var_gam) - (assign17180_e22726 * locals.var_gam_dn5)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn5 / (2.0 * assign17180_e22731)))))) } else { 0.0 } / assign17180_e22736)) + ((((((locals.var_vs * locals.var_inv_vt_dn5) + (2.0 * locals.var_phib_dn5)) + (if assign17180_e22772 >= 1e-38 { ((((((2.0 * locals.var_nq_dn5) * locals.var_gam) - (assign17180_e22757 * locals.var_gam_dn5)) / (locals.var_gam * locals.var_gam)) * assign17180_e22771) + (assign17180_e22759 * (((((assign17180_e22762 * locals.var_nq_dn5) * locals.var_gam) - (assign17180_e22764 * locals.var_gam_dn5)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn5 / (2.0 * assign17180_e22769)))))) } else { 0.0 } / assign17180_e22774)) * assign17180_e22814) + (assign17180_e22776 * (((locals.var_vs * locals.var_inv_vt_dn5) + (2.0 * locals.var_phib_dn5)) + (if assign17180_e22810 >= 1e-38 { ((((((2.0 * locals.var_nq_dn5) * locals.var_gam) - (assign17180_e22795 * locals.var_gam_dn5)) / (locals.var_gam * locals.var_gam)) * assign17180_e22809) + (assign17180_e22797 * (((((assign17180_e22800 * locals.var_nq_dn5) * locals.var_gam) - (assign17180_e22802 * locals.var_gam_dn5)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn5 / (2.0 * assign17180_e22807)))))) } else { 0.0 } / assign17180_e22812)))) / (2.0 * assign17180_e22822))));
        locals.var_psip_th_dn6 = (0.5 * ((((locals.var_vs_dn6 * locals.var_inv_vt) + (2.0 * locals.var_phib_dn6)) + (if assign17180_e22734 >= 1e-38 { ((((((2.0 * locals.var_nq_dn6) * locals.var_gam) - (assign17180_e22719 * locals.var_gam_dn6)) / (locals.var_gam * locals.var_gam)) * assign17180_e22733) + (assign17180_e22721 * (((((assign17180_e22724 * locals.var_nq_dn6) * locals.var_gam) - (assign17180_e22726 * locals.var_gam_dn6)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn6 / (2.0 * assign17180_e22731)))))) } else { 0.0 } / assign17180_e22736)) + ((((((locals.var_vs_dn6 * locals.var_inv_vt) + (2.0 * locals.var_phib_dn6)) + (if assign17180_e22772 >= 1e-38 { ((((((2.0 * locals.var_nq_dn6) * locals.var_gam) - (assign17180_e22757 * locals.var_gam_dn6)) / (locals.var_gam * locals.var_gam)) * assign17180_e22771) + (assign17180_e22759 * (((((assign17180_e22762 * locals.var_nq_dn6) * locals.var_gam) - (assign17180_e22764 * locals.var_gam_dn6)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn6 / (2.0 * assign17180_e22769)))))) } else { 0.0 } / assign17180_e22774)) * assign17180_e22814) + (assign17180_e22776 * (((locals.var_vs_dn6 * locals.var_inv_vt) + (2.0 * locals.var_phib_dn6)) + (if assign17180_e22810 >= 1e-38 { ((((((2.0 * locals.var_nq_dn6) * locals.var_gam) - (assign17180_e22795 * locals.var_gam_dn6)) / (locals.var_gam * locals.var_gam)) * assign17180_e22809) + (assign17180_e22797 * (((((assign17180_e22800 * locals.var_nq_dn6) * locals.var_gam) - (assign17180_e22802 * locals.var_gam_dn6)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn6 / (2.0 * assign17180_e22807)))))) } else { 0.0 } / assign17180_e22812)))) / (2.0 * assign17180_e22822))));
        locals.var_psip_th_dn7 = (0.5 * ((((locals.var_vs_dn7 * locals.var_inv_vt) + (2.0 * locals.var_phib_dn7)) + (if assign17180_e22734 >= 1e-38 { ((((((2.0 * locals.var_nq_dn7) * locals.var_gam) - (assign17180_e22719 * locals.var_gam_dn7)) / (locals.var_gam * locals.var_gam)) * assign17180_e22733) + (assign17180_e22721 * (((((assign17180_e22724 * locals.var_nq_dn7) * locals.var_gam) - (assign17180_e22726 * locals.var_gam_dn7)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn7 / (2.0 * assign17180_e22731)))))) } else { 0.0 } / assign17180_e22736)) + ((((((locals.var_vs_dn7 * locals.var_inv_vt) + (2.0 * locals.var_phib_dn7)) + (if assign17180_e22772 >= 1e-38 { ((((((2.0 * locals.var_nq_dn7) * locals.var_gam) - (assign17180_e22757 * locals.var_gam_dn7)) / (locals.var_gam * locals.var_gam)) * assign17180_e22771) + (assign17180_e22759 * (((((assign17180_e22762 * locals.var_nq_dn7) * locals.var_gam) - (assign17180_e22764 * locals.var_gam_dn7)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn7 / (2.0 * assign17180_e22769)))))) } else { 0.0 } / assign17180_e22774)) * assign17180_e22814) + (assign17180_e22776 * (((locals.var_vs_dn7 * locals.var_inv_vt) + (2.0 * locals.var_phib_dn7)) + (if assign17180_e22810 >= 1e-38 { ((((((2.0 * locals.var_nq_dn7) * locals.var_gam) - (assign17180_e22795 * locals.var_gam_dn7)) / (locals.var_gam * locals.var_gam)) * assign17180_e22809) + (assign17180_e22797 * (((((assign17180_e22800 * locals.var_nq_dn7) * locals.var_gam) - (assign17180_e22802 * locals.var_gam_dn7)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn7 / (2.0 * assign17180_e22807)))))) } else { 0.0 } / assign17180_e22812)))) / (2.0 * assign17180_e22822))));
        locals.var_psip_th_dn8 = (0.5 * (((2.0 * locals.var_phib_dn8) + (if assign17180_e22734 >= 1e-38 { ((((((2.0 * locals.var_nq_dn8) * locals.var_gam) - (assign17180_e22719 * locals.var_gam_dn8)) / (locals.var_gam * locals.var_gam)) * assign17180_e22733) + (assign17180_e22721 * (((((assign17180_e22724 * locals.var_nq_dn8) * locals.var_gam) - (assign17180_e22726 * locals.var_gam_dn8)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn8 / (2.0 * assign17180_e22731)))))) } else { 0.0 } / assign17180_e22736)) + (((((2.0 * locals.var_phib_dn8) + (if assign17180_e22772 >= 1e-38 { ((((((2.0 * locals.var_nq_dn8) * locals.var_gam) - (assign17180_e22757 * locals.var_gam_dn8)) / (locals.var_gam * locals.var_gam)) * assign17180_e22771) + (assign17180_e22759 * (((((assign17180_e22762 * locals.var_nq_dn8) * locals.var_gam) - (assign17180_e22764 * locals.var_gam_dn8)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn8 / (2.0 * assign17180_e22769)))))) } else { 0.0 } / assign17180_e22774)) * assign17180_e22814) + (assign17180_e22776 * ((2.0 * locals.var_phib_dn8) + (if assign17180_e22810 >= 1e-38 { ((((((2.0 * locals.var_nq_dn8) * locals.var_gam) - (assign17180_e22795 * locals.var_gam_dn8)) / (locals.var_gam * locals.var_gam)) * assign17180_e22809) + (assign17180_e22797 * (((((assign17180_e22800 * locals.var_nq_dn8) * locals.var_gam) - (assign17180_e22802 * locals.var_gam_dn8)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn8 / (2.0 * assign17180_e22807)))))) } else { 0.0 } / assign17180_e22812)))) / (2.0 * assign17180_e22822))));
        locals.var_psip_th_dn9 = (0.5 * (((2.0 * locals.var_phib_dn9) + (if assign17180_e22734 >= 1e-38 { ((((((2.0 * locals.var_nq_dn9) * locals.var_gam) - (assign17180_e22719 * locals.var_gam_dn9)) / (locals.var_gam * locals.var_gam)) * assign17180_e22733) + (assign17180_e22721 * (((((assign17180_e22724 * locals.var_nq_dn9) * locals.var_gam) - (assign17180_e22726 * locals.var_gam_dn9)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn9 / (2.0 * assign17180_e22731)))))) } else { 0.0 } / assign17180_e22736)) + (((((2.0 * locals.var_phib_dn9) + (if assign17180_e22772 >= 1e-38 { ((((((2.0 * locals.var_nq_dn9) * locals.var_gam) - (assign17180_e22757 * locals.var_gam_dn9)) / (locals.var_gam * locals.var_gam)) * assign17180_e22771) + (assign17180_e22759 * (((((assign17180_e22762 * locals.var_nq_dn9) * locals.var_gam) - (assign17180_e22764 * locals.var_gam_dn9)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn9 / (2.0 * assign17180_e22769)))))) } else { 0.0 } / assign17180_e22774)) * assign17180_e22814) + (assign17180_e22776 * ((2.0 * locals.var_phib_dn9) + (if assign17180_e22810 >= 1e-38 { ((((((2.0 * locals.var_nq_dn9) * locals.var_gam) - (assign17180_e22795 * locals.var_gam_dn9)) / (locals.var_gam * locals.var_gam)) * assign17180_e22809) + (assign17180_e22797 * (((((assign17180_e22800 * locals.var_nq_dn9) * locals.var_gam) - (assign17180_e22802 * locals.var_gam_dn9)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn9 / (2.0 * assign17180_e22807)))))) } else { 0.0 } / assign17180_e22812)))) / (2.0 * assign17180_e22822))));
        locals.var_psip_th_dn10 = (0.5 * ((((locals.var_vs_dn10 * locals.var_inv_vt) + (2.0 * locals.var_phib_dn10)) + (if assign17180_e22734 >= 1e-38 { ((((((2.0 * locals.var_nq_dn10) * locals.var_gam) - (assign17180_e22719 * locals.var_gam_dn10)) / (locals.var_gam * locals.var_gam)) * assign17180_e22733) + (assign17180_e22721 * (((((assign17180_e22724 * locals.var_nq_dn10) * locals.var_gam) - (assign17180_e22726 * locals.var_gam_dn10)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn10 / (2.0 * assign17180_e22731)))))) } else { 0.0 } / assign17180_e22736)) + ((((((locals.var_vs_dn10 * locals.var_inv_vt) + (2.0 * locals.var_phib_dn10)) + (if assign17180_e22772 >= 1e-38 { ((((((2.0 * locals.var_nq_dn10) * locals.var_gam) - (assign17180_e22757 * locals.var_gam_dn10)) / (locals.var_gam * locals.var_gam)) * assign17180_e22771) + (assign17180_e22759 * (((((assign17180_e22762 * locals.var_nq_dn10) * locals.var_gam) - (assign17180_e22764 * locals.var_gam_dn10)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn10 / (2.0 * assign17180_e22769)))))) } else { 0.0 } / assign17180_e22774)) * assign17180_e22814) + (assign17180_e22776 * (((locals.var_vs_dn10 * locals.var_inv_vt) + (2.0 * locals.var_phib_dn10)) + (if assign17180_e22810 >= 1e-38 { ((((((2.0 * locals.var_nq_dn10) * locals.var_gam) - (assign17180_e22795 * locals.var_gam_dn10)) / (locals.var_gam * locals.var_gam)) * assign17180_e22809) + (assign17180_e22797 * (((((assign17180_e22800 * locals.var_nq_dn10) * locals.var_gam) - (assign17180_e22802 * locals.var_gam_dn10)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn10 / (2.0 * assign17180_e22807)))))) } else { 0.0 } / assign17180_e22812)))) / (2.0 * assign17180_e22822))));
        locals.var_psip_th_dn11 = (0.5 * (((2.0 * locals.var_phib_dn11) + (if assign17180_e22734 >= 1e-38 { ((((((2.0 * locals.var_nq_dn11) * locals.var_gam) - (assign17180_e22719 * locals.var_gam_dn11)) / (locals.var_gam * locals.var_gam)) * assign17180_e22733) + (assign17180_e22721 * (((((assign17180_e22724 * locals.var_nq_dn11) * locals.var_gam) - (assign17180_e22726 * locals.var_gam_dn11)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn11 / (2.0 * assign17180_e22731)))))) } else { 0.0 } / assign17180_e22736)) + (((((2.0 * locals.var_phib_dn11) + (if assign17180_e22772 >= 1e-38 { ((((((2.0 * locals.var_nq_dn11) * locals.var_gam) - (assign17180_e22757 * locals.var_gam_dn11)) / (locals.var_gam * locals.var_gam)) * assign17180_e22771) + (assign17180_e22759 * (((((assign17180_e22762 * locals.var_nq_dn11) * locals.var_gam) - (assign17180_e22764 * locals.var_gam_dn11)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn11 / (2.0 * assign17180_e22769)))))) } else { 0.0 } / assign17180_e22774)) * assign17180_e22814) + (assign17180_e22776 * ((2.0 * locals.var_phib_dn11) + (if assign17180_e22810 >= 1e-38 { ((((((2.0 * locals.var_nq_dn11) * locals.var_gam) - (assign17180_e22795 * locals.var_gam_dn11)) / (locals.var_gam * locals.var_gam)) * assign17180_e22809) + (assign17180_e22797 * (((((assign17180_e22800 * locals.var_nq_dn11) * locals.var_gam) - (assign17180_e22802 * locals.var_gam_dn11)) / (locals.var_gam * locals.var_gam)) + (2.0 * (locals.var_t0_dn11 / (2.0 * assign17180_e22807)))))) } else { 0.0 } / assign17180_e22812)))) / (2.0 * assign17180_e22822))));

        let assign17190_e22830: f64 = (locals.var_vs * locals.var_inv_vt);
        let assign17190_e22831: f64 = (locals.var_psip_th - assign17190_e22830);
        let assign17190_e22833: f64 = (assign17190_e22831 * locals.var_vt);
        let assign17190_e22834: f64 = (locals.var_vfb_i + assign17190_e22833);
        let assign17190_e22837: f64 = (locals.var_vt * locals.var_gam);
        let assign17190_e22839: f64 = (locals.var_psip_th).sqrt();
        let assign17190_e22840: f64 = (assign17190_e22837 * assign17190_e22839);
        let assign17190_e22841: f64 = (assign17190_e22834 + assign17190_e22840);
        let assign17190_e22843: f64 = (assign17190_e22841 + locals.var_vth_shift);
        let assign17190_e22844: f64 = (locals.var_devsign * assign17190_e22843);
        locals.var_vth = assign17190_e22844;
        locals.var_vth_dn3 = (locals.var_devsign * (((locals.var_vfb_i_dn3 + (locals.var_psip_th_dn3 * locals.var_vt)) + (((locals.var_vt * locals.var_gam_dn3) * assign17190_e22839) + (assign17190_e22837 * (locals.var_psip_th_dn3 / (2.0 * assign17190_e22839))))) + locals.var_vth_shift_dn3));
        locals.var_vth_dn4 = (locals.var_devsign * (((locals.var_vfb_i_dn4 + (((locals.var_psip_th_dn4 - (locals.var_vs * locals.var_inv_vt_dn4)) * locals.var_vt) + (assign17190_e22831 * locals.var_vt_dn4))) + ((((locals.var_vt_dn4 * locals.var_gam) + (locals.var_vt * locals.var_gam_dn4)) * assign17190_e22839) + (assign17190_e22837 * (locals.var_psip_th_dn4 / (2.0 * assign17190_e22839))))) + locals.var_vth_shift_dn4));
        locals.var_vth_dn5 = (locals.var_devsign * (((locals.var_vfb_i_dn5 + (((locals.var_psip_th_dn5 - (locals.var_vs * locals.var_inv_vt_dn5)) * locals.var_vt) + (assign17190_e22831 * locals.var_vt_dn5))) + ((((locals.var_vt_dn5 * locals.var_gam) + (locals.var_vt * locals.var_gam_dn5)) * assign17190_e22839) + (assign17190_e22837 * (locals.var_psip_th_dn5 / (2.0 * assign17190_e22839))))) + locals.var_vth_shift_dn5));
        locals.var_vth_dn6 = (locals.var_devsign * (((locals.var_vfb_i_dn6 + ((locals.var_psip_th_dn6 - (locals.var_vs_dn6 * locals.var_inv_vt)) * locals.var_vt)) + (((locals.var_vt * locals.var_gam_dn6) * assign17190_e22839) + (assign17190_e22837 * (locals.var_psip_th_dn6 / (2.0 * assign17190_e22839))))) + locals.var_vth_shift_dn6));
        locals.var_vth_dn7 = (locals.var_devsign * (((locals.var_vfb_i_dn7 + ((locals.var_psip_th_dn7 - (locals.var_vs_dn7 * locals.var_inv_vt)) * locals.var_vt)) + (((locals.var_vt * locals.var_gam_dn7) * assign17190_e22839) + (assign17190_e22837 * (locals.var_psip_th_dn7 / (2.0 * assign17190_e22839))))) + locals.var_vth_shift_dn7));
        locals.var_vth_dn8 = (locals.var_devsign * (((locals.var_vfb_i_dn8 + (locals.var_psip_th_dn8 * locals.var_vt)) + (((locals.var_vt * locals.var_gam_dn8) * assign17190_e22839) + (assign17190_e22837 * (locals.var_psip_th_dn8 / (2.0 * assign17190_e22839))))) + locals.var_vth_shift_dn8));
        locals.var_vth_dn9 = (locals.var_devsign * (((locals.var_vfb_i_dn9 + (locals.var_psip_th_dn9 * locals.var_vt)) + (((locals.var_vt * locals.var_gam_dn9) * assign17190_e22839) + (assign17190_e22837 * (locals.var_psip_th_dn9 / (2.0 * assign17190_e22839))))) + locals.var_vth_shift_dn9));
        locals.var_vth_dn10 = (locals.var_devsign * (((locals.var_vfb_i_dn10 + ((locals.var_psip_th_dn10 - (locals.var_vs_dn10 * locals.var_inv_vt)) * locals.var_vt)) + (((locals.var_vt * locals.var_gam_dn10) * assign17190_e22839) + (assign17190_e22837 * (locals.var_psip_th_dn10 / (2.0 * assign17190_e22839))))) + locals.var_vth_shift_dn10));
        locals.var_vth_dn11 = (locals.var_devsign * (((locals.var_vfb_i_dn11 + (locals.var_psip_th_dn11 * locals.var_vt)) + (((locals.var_vt * locals.var_gam_dn11) * assign17190_e22839) + (assign17190_e22837 * (locals.var_psip_th_dn11 / (2.0 * assign17190_e22839))))) + locals.var_vth_shift_dn11));

        let assign17200_e22847: f64 = (1.0 / locals.var_gam);
        locals.var_inv_gam = assign17200_e22847;
        locals.var_inv_gam_dn3 = (-(locals.var_gam_dn3 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn4 = (-(locals.var_gam_dn4 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn5 = (-(locals.var_gam_dn5 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn6 = (-(locals.var_gam_dn6 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn7 = (-(locals.var_gam_dn7 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn8 = (-(locals.var_gam_dn8 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn9 = (-(locals.var_gam_dn9 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn10 = (-(locals.var_gam_dn10 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn11 = (-(locals.var_gam_dn11 / (locals.var_gam * locals.var_gam)));

        let assign17210_e22850: f64 = if p.p29 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard492 = assign17210_e22850;

        let (assign17220_e22865, assign17220_e22865_d_n3, assign17220_e22865_d_n4, assign17220_e22865_d_n5, assign17220_e22865_d_n6, assign17220_e22865_d_n7, assign17220_e22865_d_n8, assign17220_e22865_d_n9, assign17220_e22865_d_n10, assign17220_e22865_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17220_e22854: f64 = (2.0 * 1.602176462e-19);
        let assign17220_e22856: f64 = (assign17220_e22854 * locals.var_epssi);
        let assign17220_e22858: f64 = (assign17220_e22856 * locals.var_ndep_i);
        let assign17220_e22860: f64 = (assign17220_e22858 * locals.var_inv_nvt);
        let assign17220_e22861: f64 = (assign17220_e22860).sqrt();
        let assign17220_e22863: f64 = (assign17220_e22861 / locals.var_cox);
        (assign17220_e22863, (((((assign17220_e22856 * locals.var_ndep_i_dn3) * locals.var_inv_nvt) + (assign17220_e22858 * locals.var_inv_nvt_dn3)) / (2.0 * assign17220_e22861)) / locals.var_cox), (((((assign17220_e22856 * locals.var_ndep_i_dn4) * locals.var_inv_nvt) + (assign17220_e22858 * locals.var_inv_nvt_dn4)) / (2.0 * assign17220_e22861)) / locals.var_cox), (((((assign17220_e22856 * locals.var_ndep_i_dn5) * locals.var_inv_nvt) + (assign17220_e22858 * locals.var_inv_nvt_dn5)) / (2.0 * assign17220_e22861)) / locals.var_cox), (((((assign17220_e22856 * locals.var_ndep_i_dn6) * locals.var_inv_nvt) + (assign17220_e22858 * locals.var_inv_nvt_dn6)) / (2.0 * assign17220_e22861)) / locals.var_cox), (((((assign17220_e22856 * locals.var_ndep_i_dn7) * locals.var_inv_nvt) + (assign17220_e22858 * locals.var_inv_nvt_dn7)) / (2.0 * assign17220_e22861)) / locals.var_cox), (((((assign17220_e22856 * locals.var_ndep_i_dn8) * locals.var_inv_nvt) + (assign17220_e22858 * locals.var_inv_nvt_dn8)) / (2.0 * assign17220_e22861)) / locals.var_cox), (((((assign17220_e22856 * locals.var_ndep_i_dn9) * locals.var_inv_nvt) + (assign17220_e22858 * locals.var_inv_nvt_dn9)) / (2.0 * assign17220_e22861)) / locals.var_cox), (((((assign17220_e22856 * locals.var_ndep_i_dn10) * locals.var_inv_nvt) + (assign17220_e22858 * locals.var_inv_nvt_dn10)) / (2.0 * assign17220_e22861)) / locals.var_cox), (((((assign17220_e22856 * locals.var_ndep_i_dn11) * locals.var_inv_nvt) + (assign17220_e22858 * locals.var_inv_nvt_dn11)) / (2.0 * assign17220_e22861)) / locals.var_cox),)
    } else {
        (locals.var_gam, locals.var_gam_dn3, locals.var_gam_dn4, locals.var_gam_dn5, locals.var_gam_dn6, locals.var_gam_dn7, locals.var_gam_dn8, locals.var_gam_dn9, locals.var_gam_dn10, locals.var_gam_dn11,)
    }
};
        locals.var_gam = assign17220_e22865;
        locals.var_gam_dn3 = assign17220_e22865_d_n3;
        locals.var_gam_dn4 = assign17220_e22865_d_n4;
        locals.var_gam_dn5 = assign17220_e22865_d_n5;
        locals.var_gam_dn6 = assign17220_e22865_d_n6;
        locals.var_gam_dn7 = assign17220_e22865_d_n7;
        locals.var_gam_dn8 = assign17220_e22865_d_n8;
        locals.var_gam_dn9 = assign17220_e22865_d_n9;
        locals.var_gam_dn10 = assign17220_e22865_d_n10;
        locals.var_gam_dn11 = assign17220_e22865_d_n11;

        let (assign17230_e22871, assign17230_e22871_d_n3, assign17230_e22871_d_n4, assign17230_e22871_d_n5, assign17230_e22871_d_n6, assign17230_e22871_d_n7, assign17230_e22871_d_n8, assign17230_e22871_d_n9, assign17230_e22871_d_n10, assign17230_e22871_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17230_e22869: f64 = (1.0 / locals.var_gam);
        (assign17230_e22869, (-(locals.var_gam_dn3 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn4 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn5 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn6 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn7 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn8 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn9 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn10 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn11 / (locals.var_gam * locals.var_gam))),)
    } else {
        (locals.var_inv_gam, locals.var_inv_gam_dn3, locals.var_inv_gam_dn4, locals.var_inv_gam_dn5, locals.var_inv_gam_dn6, locals.var_inv_gam_dn7, locals.var_inv_gam_dn8, locals.var_inv_gam_dn9, locals.var_inv_gam_dn10, locals.var_inv_gam_dn11,)
    }
};
        locals.var_inv_gam = assign17230_e22871;
        locals.var_inv_gam_dn3 = assign17230_e22871_d_n3;
        locals.var_inv_gam_dn4 = assign17230_e22871_d_n4;
        locals.var_inv_gam_dn5 = assign17230_e22871_d_n5;
        locals.var_inv_gam_dn6 = assign17230_e22871_d_n6;
        locals.var_inv_gam_dn7 = assign17230_e22871_d_n7;
        locals.var_inv_gam_dn8 = assign17230_e22871_d_n8;
        locals.var_inv_gam_dn9 = assign17230_e22871_d_n9;
        locals.var_inv_gam_dn10 = assign17230_e22871_d_n10;
        locals.var_inv_gam_dn11 = assign17230_e22871_d_n11;

        let (assign17240_e22877, assign17240_e22877_d_n3, assign17240_e22877_d_n4, assign17240_e22877_d_n5, assign17240_e22877_d_n6, assign17240_e22877_d_n7, assign17240_e22877_d_n8, assign17240_e22877_d_n9, assign17240_e22877_d_n10, assign17240_e22877_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17240_e22875: f64 = (locals.var_gam * locals.var_gam);
        (assign17240_e22875, ((locals.var_gam_dn3 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn3)), ((locals.var_gam_dn4 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn4)), ((locals.var_gam_dn5 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn5)), ((locals.var_gam_dn6 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn6)), ((locals.var_gam_dn7 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn7)), ((locals.var_gam_dn8 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn8)), ((locals.var_gam_dn9 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn9)), ((locals.var_gam_dn10 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn10)), ((locals.var_gam_dn11 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn11)),)
    } else {
        (locals.var_gam2, locals.var_gam2_dn3, locals.var_gam2_dn4, locals.var_gam2_dn5, locals.var_gam2_dn6, locals.var_gam2_dn7, locals.var_gam2_dn8, locals.var_gam2_dn9, locals.var_gam2_dn10, locals.var_gam2_dn11,)
    }
};
        locals.var_gam2 = assign17240_e22877;
        locals.var_gam2_dn3 = assign17240_e22877_d_n3;
        locals.var_gam2_dn4 = assign17240_e22877_d_n4;
        locals.var_gam2_dn5 = assign17240_e22877_d_n5;
        locals.var_gam2_dn6 = assign17240_e22877_d_n6;
        locals.var_gam2_dn7 = assign17240_e22877_d_n7;
        locals.var_gam2_dn8 = assign17240_e22877_d_n8;
        locals.var_gam2_dn9 = assign17240_e22877_d_n9;
        locals.var_gam2_dn10 = assign17240_e22877_d_n10;
        locals.var_gam2_dn11 = assign17240_e22877_d_n11;

        let (assign17250_e22883, assign17250_e22883_d_n3, assign17250_e22883_d_n4, assign17250_e22883_d_n5, assign17250_e22883_d_n6, assign17250_e22883_d_n7, assign17250_e22883_d_n8, assign17250_e22883_d_n9, assign17250_e22883_d_n10, assign17250_e22883_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17250_e22881: f64 = (1.0 / locals.var_gam2);
        (assign17250_e22881, (-(locals.var_gam2_dn3 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn4 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn5 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn6 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn7 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn8 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn9 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn10 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn11 / (locals.var_gam2 * locals.var_gam2))),)
    } else {
        (locals.var_inv_gam2, locals.var_inv_gam2_dn3, locals.var_inv_gam2_dn4, locals.var_inv_gam2_dn5, locals.var_inv_gam2_dn6, locals.var_inv_gam2_dn7, locals.var_inv_gam2_dn8, locals.var_inv_gam2_dn9, locals.var_inv_gam2_dn10, locals.var_inv_gam2_dn11,)
    }
};
        locals.var_inv_gam2 = assign17250_e22883;
        locals.var_inv_gam2_dn3 = assign17250_e22883_d_n3;
        locals.var_inv_gam2_dn4 = assign17250_e22883_d_n4;
        locals.var_inv_gam2_dn5 = assign17250_e22883_d_n5;
        locals.var_inv_gam2_dn6 = assign17250_e22883_d_n6;
        locals.var_inv_gam2_dn7 = assign17250_e22883_d_n7;
        locals.var_inv_gam2_dn8 = assign17250_e22883_d_n8;
        locals.var_inv_gam2_dn9 = assign17250_e22883_d_n9;
        locals.var_inv_gam2_dn10 = assign17250_e22883_d_n10;
        locals.var_inv_gam2_dn11 = assign17250_e22883_d_n11;

        let (assign17260_e22889,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17260_e22887: f64 = (locals.var_epssi / p.p74);
        (assign17260_e22887,)
    } else {
        (locals.var_cb,)
    }
};
        locals.var_cb = assign17260_e22889;

        let (assign17270_e22895,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17270_e22893: f64 = (locals.var_epsox / p.p75);
        (assign17270_e22893,)
    } else {
        (locals.var_cbox,)
    }
};
        locals.var_cbox = assign17270_e22895;

        let (assign17280_e22903,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17280_e22899: f64 = (locals.var_cbox + locals.var_cdsbs_i);
        let assign17280_e22901: f64 = (assign17280_e22899 / locals.var_cb);
        (assign17280_e22901,)
    } else {
        (locals.var_rc,)
    }
};
        locals.var_rc = assign17280_e22903;

        let (assign17290_e22909,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17290_e22907: f64 = (p.p76 / p.p75);
        (assign17290_e22907,)
    } else {
        (locals.var_rt,)
    }
};
        locals.var_rt = assign17290_e22909;

        let (assign17300_e22915, assign17300_e22915_d_n3, assign17300_e22915_d_n4, assign17300_e22915_d_n5, assign17300_e22915_d_n6, assign17300_e22915_d_n7, assign17300_e22915_d_n8, assign17300_e22915_d_n9, assign17300_e22915_d_n10, assign17300_e22915_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17300_e22913: f64 = (locals.var_gam / locals.var_rt);
        (assign17300_e22913, (locals.var_gam_dn3 / locals.var_rt), (locals.var_gam_dn4 / locals.var_rt), (locals.var_gam_dn5 / locals.var_rt), (locals.var_gam_dn6 / locals.var_rt), (locals.var_gam_dn7 / locals.var_rt), (locals.var_gam_dn8 / locals.var_rt), (locals.var_gam_dn9 / locals.var_rt), (locals.var_gam_dn10 / locals.var_rt), (locals.var_gam_dn11 / locals.var_rt),)
    } else {
        (locals.var_gam_sb, locals.var_gam_sb_dn3, locals.var_gam_sb_dn4, locals.var_gam_sb_dn5, locals.var_gam_sb_dn6, locals.var_gam_sb_dn7, locals.var_gam_sb_dn8, locals.var_gam_sb_dn9, locals.var_gam_sb_dn10, locals.var_gam_sb_dn11,)
    }
};
        locals.var_gam_sb = assign17300_e22915;
        locals.var_gam_sb_dn3 = assign17300_e22915_d_n3;
        locals.var_gam_sb_dn4 = assign17300_e22915_d_n4;
        locals.var_gam_sb_dn5 = assign17300_e22915_d_n5;
        locals.var_gam_sb_dn6 = assign17300_e22915_d_n6;
        locals.var_gam_sb_dn7 = assign17300_e22915_d_n7;
        locals.var_gam_sb_dn8 = assign17300_e22915_d_n8;
        locals.var_gam_sb_dn9 = assign17300_e22915_d_n9;
        locals.var_gam_sb_dn10 = assign17300_e22915_d_n10;
        locals.var_gam_sb_dn11 = assign17300_e22915_d_n11;

        let (assign17310_e22923, assign17310_e22923_d_n3, assign17310_e22923_d_n4, assign17310_e22923_d_n5, assign17310_e22923_d_n6, assign17310_e22923_d_n7, assign17310_e22923_d_n8, assign17310_e22923_d_n9, assign17310_e22923_d_n10, assign17310_e22923_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17310_e22920: f64 = (locals.var_gam_sb * 0.7071067811865475);
        let assign17310_e22921: f64 = (1.0 + assign17310_e22920);
        (assign17310_e22921, (locals.var_gam_sb_dn3 * 0.7071067811865475), (locals.var_gam_sb_dn4 * 0.7071067811865475), (locals.var_gam_sb_dn5 * 0.7071067811865475), (locals.var_gam_sb_dn6 * 0.7071067811865475), (locals.var_gam_sb_dn7 * 0.7071067811865475), (locals.var_gam_sb_dn8 * 0.7071067811865475), (locals.var_gam_sb_dn9 * 0.7071067811865475), (locals.var_gam_sb_dn10 * 0.7071067811865475), (locals.var_gam_sb_dn11 * 0.7071067811865475),)
    } else {
        (locals.var_x1_sb, locals.var_x1_sb_dn3, locals.var_x1_sb_dn4, locals.var_x1_sb_dn5, locals.var_x1_sb_dn6, locals.var_x1_sb_dn7, locals.var_x1_sb_dn8, locals.var_x1_sb_dn9, locals.var_x1_sb_dn10, locals.var_x1_sb_dn11,)
    }
};
        locals.var_x1_sb = assign17310_e22923;
        locals.var_x1_sb_dn3 = assign17310_e22923_d_n3;
        locals.var_x1_sb_dn4 = assign17310_e22923_d_n4;
        locals.var_x1_sb_dn5 = assign17310_e22923_d_n5;
        locals.var_x1_sb_dn6 = assign17310_e22923_d_n6;
        locals.var_x1_sb_dn7 = assign17310_e22923_d_n7;
        locals.var_x1_sb_dn8 = assign17310_e22923_d_n8;
        locals.var_x1_sb_dn9 = assign17310_e22923_d_n9;
        locals.var_x1_sb_dn10 = assign17310_e22923_d_n10;
        locals.var_x1_sb_dn11 = assign17310_e22923_d_n11;

        let (assign17320_e22929,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17320_e22927: f64 = (1e-7 * locals.var_x1_sb);
        (assign17320_e22927,)
    } else {
        (locals.var_limit_sb,)
    }
};
        locals.var_limit_sb = assign17320_e22929;

        let (assign17330_e22935,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17330_e22933: f64 = (5.0 / 4.0);
        (assign17330_e22933,)
    } else {
        (locals.var_x1_csb,)
    }
};
        locals.var_x1_csb = assign17330_e22935;

        let (assign17340_e22941, assign17340_e22941_d_n3, assign17340_e22941_d_n4, assign17340_e22941_d_n5, assign17340_e22941_d_n6, assign17340_e22941_d_n7, assign17340_e22941_d_n8, assign17340_e22941_d_n9, assign17340_e22941_d_n10, assign17340_e22941_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17340_e22939: f64 = (1.0 / locals.var_gam_sb);
        (assign17340_e22939, (-(locals.var_gam_sb_dn3 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn4 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn5 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn6 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn7 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn8 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn9 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn10 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn11 / (locals.var_gam_sb * locals.var_gam_sb))),)
    } else {
        (locals.var_inv_xi_sb, locals.var_inv_xi_sb_dn3, locals.var_inv_xi_sb_dn4, locals.var_inv_xi_sb_dn5, locals.var_inv_xi_sb_dn6, locals.var_inv_xi_sb_dn7, locals.var_inv_xi_sb_dn8, locals.var_inv_xi_sb_dn9, locals.var_inv_xi_sb_dn10, locals.var_inv_xi_sb_dn11,)
    }
};
        locals.var_inv_xi_sb = assign17340_e22941;
        locals.var_inv_xi_sb_dn3 = assign17340_e22941_d_n3;
        locals.var_inv_xi_sb_dn4 = assign17340_e22941_d_n4;
        locals.var_inv_xi_sb_dn5 = assign17340_e22941_d_n5;
        locals.var_inv_xi_sb_dn6 = assign17340_e22941_d_n6;
        locals.var_inv_xi_sb_dn7 = assign17340_e22941_d_n7;
        locals.var_inv_xi_sb_dn8 = assign17340_e22941_d_n8;
        locals.var_inv_xi_sb_dn9 = assign17340_e22941_d_n9;
        locals.var_inv_xi_sb_dn10 = assign17340_e22941_d_n10;
        locals.var_inv_xi_sb_dn11 = assign17340_e22941_d_n11;

        let (assign17350_e22947, assign17350_e22947_d_n3, assign17350_e22947_d_n4, assign17350_e22947_d_n5, assign17350_e22947_d_n6, assign17350_e22947_d_n7, assign17350_e22947_d_n8, assign17350_e22947_d_n9, assign17350_e22947_d_n10, assign17350_e22947_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17350_e22945: f64 = (locals.var_gam_sb * locals.var_gam_sb);
        (assign17350_e22945, ((locals.var_gam_sb_dn3 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn3)), ((locals.var_gam_sb_dn4 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn4)), ((locals.var_gam_sb_dn5 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn5)), ((locals.var_gam_sb_dn6 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn6)), ((locals.var_gam_sb_dn7 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn7)), ((locals.var_gam_sb_dn8 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn8)), ((locals.var_gam_sb_dn9 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn9)), ((locals.var_gam_sb_dn10 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn10)), ((locals.var_gam_sb_dn11 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn11)),)
    } else {
        (locals.var_gam_sb2, locals.var_gam_sb2_dn3, locals.var_gam_sb2_dn4, locals.var_gam_sb2_dn5, locals.var_gam_sb2_dn6, locals.var_gam_sb2_dn7, locals.var_gam_sb2_dn8, locals.var_gam_sb2_dn9, locals.var_gam_sb2_dn10, locals.var_gam_sb2_dn11,)
    }
};
        locals.var_gam_sb2 = assign17350_e22947;
        locals.var_gam_sb2_dn3 = assign17350_e22947_d_n3;
        locals.var_gam_sb2_dn4 = assign17350_e22947_d_n4;
        locals.var_gam_sb2_dn5 = assign17350_e22947_d_n5;
        locals.var_gam_sb2_dn6 = assign17350_e22947_d_n6;
        locals.var_gam_sb2_dn7 = assign17350_e22947_d_n7;
        locals.var_gam_sb2_dn8 = assign17350_e22947_d_n8;
        locals.var_gam_sb2_dn9 = assign17350_e22947_d_n9;
        locals.var_gam_sb2_dn10 = assign17350_e22947_d_n10;
        locals.var_gam_sb2_dn11 = assign17350_e22947_d_n11;

        let (assign17360_e22957, assign17360_e22957_d_n3, assign17360_e22957_d_n4, assign17360_e22957_d_n5, assign17360_e22957_d_n6, assign17360_e22957_d_n7, assign17360_e22957_d_n8, assign17360_e22957_d_n9, assign17360_e22957_d_n10, assign17360_e22957_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign17360_e22953: f64 = (locals.var_gam_sb * 0.7324648775608221);
        let assign17360_e22954: f64 = (locals.var_x1_csb + assign17360_e22953);
        let assign17360_e22955: f64 = (1.0 / assign17360_e22954);
        (assign17360_e22955, (-((locals.var_gam_sb_dn3 * 0.7324648775608221) / (assign17360_e22954 * assign17360_e22954))), (-((locals.var_gam_sb_dn4 * 0.7324648775608221) / (assign17360_e22954 * assign17360_e22954))), (-((locals.var_gam_sb_dn5 * 0.7324648775608221) / (assign17360_e22954 * assign17360_e22954))), (-((locals.var_gam_sb_dn6 * 0.7324648775608221) / (assign17360_e22954 * assign17360_e22954))), (-((locals.var_gam_sb_dn7 * 0.7324648775608221) / (assign17360_e22954 * assign17360_e22954))), (-((locals.var_gam_sb_dn8 * 0.7324648775608221) / (assign17360_e22954 * assign17360_e22954))), (-((locals.var_gam_sb_dn9 * 0.7324648775608221) / (assign17360_e22954 * assign17360_e22954))), (-((locals.var_gam_sb_dn10 * 0.7324648775608221) / (assign17360_e22954 * assign17360_e22954))), (-((locals.var_gam_sb_dn11 * 0.7324648775608221) / (assign17360_e22954 * assign17360_e22954))),)
    } else {
        (locals.var_inv_xg1_sb, locals.var_inv_xg1_sb_dn3, locals.var_inv_xg1_sb_dn4, locals.var_inv_xg1_sb_dn5, locals.var_inv_xg1_sb_dn6, locals.var_inv_xg1_sb_dn7, locals.var_inv_xg1_sb_dn8, locals.var_inv_xg1_sb_dn9, locals.var_inv_xg1_sb_dn10, locals.var_inv_xg1_sb_dn11,)
    }
};
        locals.var_inv_xg1_sb = assign17360_e22957;
        locals.var_inv_xg1_sb_dn3 = assign17360_e22957_d_n3;
        locals.var_inv_xg1_sb_dn4 = assign17360_e22957_d_n4;
        locals.var_inv_xg1_sb_dn5 = assign17360_e22957_d_n5;
        locals.var_inv_xg1_sb_dn6 = assign17360_e22957_d_n6;
        locals.var_inv_xg1_sb_dn7 = assign17360_e22957_d_n7;
        locals.var_inv_xg1_sb_dn8 = assign17360_e22957_d_n8;
        locals.var_inv_xg1_sb_dn9 = assign17360_e22957_d_n9;
        locals.var_inv_xg1_sb_dn10 = assign17360_e22957_d_n10;
        locals.var_inv_xg1_sb_dn11 = assign17360_e22957_d_n11;

        let assign17370_e22959: f64 = (locals.var_vgfbb).abs();
        let assign17370_e22961: f64 = if assign17370_e22959 <= locals.var_limit_sb { 1.0 } else { 0.0 };
        locals.var_guard493 = assign17370_e22961;

        let (assign17380_e22986, assign17380_e22986_d_n3, assign17380_e22986_d_n4, assign17380_e22986_d_n5, assign17380_e22986_d_n6, assign17380_e22986_d_n7, assign17380_e22986_d_n8, assign17380_e22986_d_n9, assign17380_e22986_d_n10, assign17380_e22986_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard493 != 0.0)) {
        let assign17380_e22966: f64 = (-locals.var_vgfbb);
        let assign17380_e22968: f64 = (assign17380_e22966 * locals.var_inv_xi_sb);
        let assign17380_e22972: f64 = (-locals.var_vgfbb);
        let assign17380_e22975: f64 = (2.0_f64).sqrt();
        let assign17380_e22976: f64 = (6.0 * assign17380_e22975);
        let assign17380_e22978: f64 = (assign17380_e22976 * locals.var_x1_sb);
        let assign17380_e22980: f64 = (assign17380_e22978 * locals.var_x1_sb);
        let assign17380_e22981: f64 = (assign17380_e22972 / assign17380_e22980);
        let assign17380_e22982: f64 = (locals.var_gam_sb * assign17380_e22981);
        let assign17380_e22983: f64 = (1.0 + assign17380_e22982);
        let assign17380_e22984: f64 = (assign17380_e22968 * assign17380_e22983);
        (assign17380_e22984, (((((-locals.var_vgfbb_dn3) * locals.var_inv_xi_sb) + (assign17380_e22966 * locals.var_inv_xi_sb_dn3)) * assign17380_e22983) + (assign17380_e22968 * ((locals.var_gam_sb_dn3 * assign17380_e22981) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn3) * assign17380_e22980) - (assign17380_e22972 * (((assign17380_e22976 * locals.var_x1_sb_dn3) * locals.var_x1_sb) + (assign17380_e22978 * locals.var_x1_sb_dn3)))) / (assign17380_e22980 * assign17380_e22980)))))), (((((-locals.var_vgfbb_dn4) * locals.var_inv_xi_sb) + (assign17380_e22966 * locals.var_inv_xi_sb_dn4)) * assign17380_e22983) + (assign17380_e22968 * ((locals.var_gam_sb_dn4 * assign17380_e22981) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn4) * assign17380_e22980) - (assign17380_e22972 * (((assign17380_e22976 * locals.var_x1_sb_dn4) * locals.var_x1_sb) + (assign17380_e22978 * locals.var_x1_sb_dn4)))) / (assign17380_e22980 * assign17380_e22980)))))), (((((-locals.var_vgfbb_dn5) * locals.var_inv_xi_sb) + (assign17380_e22966 * locals.var_inv_xi_sb_dn5)) * assign17380_e22983) + (assign17380_e22968 * ((locals.var_gam_sb_dn5 * assign17380_e22981) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn5) * assign17380_e22980) - (assign17380_e22972 * (((assign17380_e22976 * locals.var_x1_sb_dn5) * locals.var_x1_sb) + (assign17380_e22978 * locals.var_x1_sb_dn5)))) / (assign17380_e22980 * assign17380_e22980)))))), (((((-locals.var_vgfbb_dn6) * locals.var_inv_xi_sb) + (assign17380_e22966 * locals.var_inv_xi_sb_dn6)) * assign17380_e22983) + (assign17380_e22968 * ((locals.var_gam_sb_dn6 * assign17380_e22981) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn6) * assign17380_e22980) - (assign17380_e22972 * (((assign17380_e22976 * locals.var_x1_sb_dn6) * locals.var_x1_sb) + (assign17380_e22978 * locals.var_x1_sb_dn6)))) / (assign17380_e22980 * assign17380_e22980)))))), (((((-locals.var_vgfbb_dn7) * locals.var_inv_xi_sb) + (assign17380_e22966 * locals.var_inv_xi_sb_dn7)) * assign17380_e22983) + (assign17380_e22968 * ((locals.var_gam_sb_dn7 * assign17380_e22981) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn7) * assign17380_e22980) - (assign17380_e22972 * (((assign17380_e22976 * locals.var_x1_sb_dn7) * locals.var_x1_sb) + (assign17380_e22978 * locals.var_x1_sb_dn7)))) / (assign17380_e22980 * assign17380_e22980)))))), (((((-locals.var_vgfbb_dn8) * locals.var_inv_xi_sb) + (assign17380_e22966 * locals.var_inv_xi_sb_dn8)) * assign17380_e22983) + (assign17380_e22968 * ((locals.var_gam_sb_dn8 * assign17380_e22981) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn8) * assign17380_e22980) - (assign17380_e22972 * (((assign17380_e22976 * locals.var_x1_sb_dn8) * locals.var_x1_sb) + (assign17380_e22978 * locals.var_x1_sb_dn8)))) / (assign17380_e22980 * assign17380_e22980)))))), (((((-locals.var_vgfbb_dn9) * locals.var_inv_xi_sb) + (assign17380_e22966 * locals.var_inv_xi_sb_dn9)) * assign17380_e22983) + (assign17380_e22968 * ((locals.var_gam_sb_dn9 * assign17380_e22981) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn9) * assign17380_e22980) - (assign17380_e22972 * (((assign17380_e22976 * locals.var_x1_sb_dn9) * locals.var_x1_sb) + (assign17380_e22978 * locals.var_x1_sb_dn9)))) / (assign17380_e22980 * assign17380_e22980)))))), (((((-locals.var_vgfbb_dn10) * locals.var_inv_xi_sb) + (assign17380_e22966 * locals.var_inv_xi_sb_dn10)) * assign17380_e22983) + (assign17380_e22968 * ((locals.var_gam_sb_dn10 * assign17380_e22981) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn10) * assign17380_e22980) - (assign17380_e22972 * (((assign17380_e22976 * locals.var_x1_sb_dn10) * locals.var_x1_sb) + (assign17380_e22978 * locals.var_x1_sb_dn10)))) / (assign17380_e22980 * assign17380_e22980)))))), (((((-locals.var_vgfbb_dn11) * locals.var_inv_xi_sb) + (assign17380_e22966 * locals.var_inv_xi_sb_dn11)) * assign17380_e22983) + (assign17380_e22968 * ((locals.var_gam_sb_dn11 * assign17380_e22981) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn11) * assign17380_e22980) - (assign17380_e22972 * (((assign17380_e22976 * locals.var_x1_sb_dn11) * locals.var_x1_sb) + (assign17380_e22978 * locals.var_x1_sb_dn11)))) / (assign17380_e22980 * assign17380_e22980)))))),)
    } else {
        (locals.var_pd_sb, locals.var_pd_sb_dn3, locals.var_pd_sb_dn4, locals.var_pd_sb_dn5, locals.var_pd_sb_dn6, locals.var_pd_sb_dn7, locals.var_pd_sb_dn8, locals.var_pd_sb_dn9, locals.var_pd_sb_dn10, locals.var_pd_sb_dn11,)
    }
};
        locals.var_pd_sb = assign17380_e22986;
        locals.var_pd_sb_dn3 = assign17380_e22986_d_n3;
        locals.var_pd_sb_dn4 = assign17380_e22986_d_n4;
        locals.var_pd_sb_dn5 = assign17380_e22986_d_n5;
        locals.var_pd_sb_dn6 = assign17380_e22986_d_n6;
        locals.var_pd_sb_dn7 = assign17380_e22986_d_n7;
        locals.var_pd_sb_dn8 = assign17380_e22986_d_n8;
        locals.var_pd_sb_dn9 = assign17380_e22986_d_n9;
        locals.var_pd_sb_dn10 = assign17380_e22986_d_n10;
        locals.var_pd_sb_dn11 = assign17380_e22986_d_n11;

        let assign17390_e22989: f64 = (-locals.var_limit_sb);
        let assign17390_e22990: f64 = if locals.var_vgfbb < assign17390_e22989 { 1.0 } else { 0.0 };
        locals.var_guard494 = assign17390_e22990;

        let (assign17400_e23000, assign17400_e23000_d_n3, assign17400_e23000_d_n4, assign17400_e23000_d_n5, assign17400_e23000_d_n6, assign17400_e23000_d_n7, assign17400_e23000_d_n8, assign17400_e23000_d_n9, assign17400_e23000_d_n10, assign17400_e23000_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard493 == 0.0)) && (locals.var_guard494 != 0.0)) {
        let assign17400_e22998: f64 = (-locals.var_vgfbb);
        (assign17400_e22998, (-locals.var_vgfbb_dn3), (-locals.var_vgfbb_dn4), (-locals.var_vgfbb_dn5), (-locals.var_vgfbb_dn6), (-locals.var_vgfbb_dn7), (-locals.var_vgfbb_dn8), (-locals.var_vgfbb_dn9), (-locals.var_vgfbb_dn10), (-locals.var_vgfbb_dn11),)
    } else {
        (locals.var_pd_yg, locals.var_pd_yg_dn3, locals.var_pd_yg_dn4, locals.var_pd_yg_dn5, locals.var_pd_yg_dn6, locals.var_pd_yg_dn7, locals.var_pd_yg_dn8, locals.var_pd_yg_dn9, locals.var_pd_yg_dn10, locals.var_pd_yg_dn11,)
    }
};
        locals.var_pd_yg = assign17400_e23000;
        locals.var_pd_yg_dn3 = assign17400_e23000_d_n3;
        locals.var_pd_yg_dn4 = assign17400_e23000_d_n4;
        locals.var_pd_yg_dn5 = assign17400_e23000_d_n5;
        locals.var_pd_yg_dn6 = assign17400_e23000_d_n6;
        locals.var_pd_yg_dn7 = assign17400_e23000_d_n7;
        locals.var_pd_yg_dn8 = assign17400_e23000_d_n8;
        locals.var_pd_yg_dn9 = assign17400_e23000_d_n9;
        locals.var_pd_yg_dn10 = assign17400_e23000_d_n10;
        locals.var_pd_yg_dn11 = assign17400_e23000_d_n11;

        let (assign17410_e23013, assign17410_e23013_d_n3, assign17410_e23013_d_n4, assign17410_e23013_d_n5, assign17410_e23013_d_n6, assign17410_e23013_d_n7, assign17410_e23013_d_n8, assign17410_e23013_d_n9, assign17410_e23013_d_n10, assign17410_e23013_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard493 == 0.0)) && (locals.var_guard494 != 0.0)) {
        let assign17410_e23009: f64 = (locals.var_x1_csb * locals.var_pd_yg);
        let assign17410_e23011: f64 = (assign17410_e23009 * locals.var_inv_xi_sb);
        (assign17410_e23011, (((locals.var_x1_csb * locals.var_pd_yg_dn3) * locals.var_inv_xi_sb) + (assign17410_e23009 * locals.var_inv_xi_sb_dn3)), (((locals.var_x1_csb * locals.var_pd_yg_dn4) * locals.var_inv_xi_sb) + (assign17410_e23009 * locals.var_inv_xi_sb_dn4)), (((locals.var_x1_csb * locals.var_pd_yg_dn5) * locals.var_inv_xi_sb) + (assign17410_e23009 * locals.var_inv_xi_sb_dn5)), (((locals.var_x1_csb * locals.var_pd_yg_dn6) * locals.var_inv_xi_sb) + (assign17410_e23009 * locals.var_inv_xi_sb_dn6)), (((locals.var_x1_csb * locals.var_pd_yg_dn7) * locals.var_inv_xi_sb) + (assign17410_e23009 * locals.var_inv_xi_sb_dn7)), (((locals.var_x1_csb * locals.var_pd_yg_dn8) * locals.var_inv_xi_sb) + (assign17410_e23009 * locals.var_inv_xi_sb_dn8)), (((locals.var_x1_csb * locals.var_pd_yg_dn9) * locals.var_inv_xi_sb) + (assign17410_e23009 * locals.var_inv_xi_sb_dn9)), (((locals.var_x1_csb * locals.var_pd_yg_dn10) * locals.var_inv_xi_sb) + (assign17410_e23009 * locals.var_inv_xi_sb_dn10)), (((locals.var_x1_csb * locals.var_pd_yg_dn11) * locals.var_inv_xi_sb) + (assign17410_e23009 * locals.var_inv_xi_sb_dn11)),)
    } else {
        (locals.var_pd_z, locals.var_pd_z_dn3, locals.var_pd_z_dn4, locals.var_pd_z_dn5, locals.var_pd_z_dn6, locals.var_pd_z_dn7, locals.var_pd_z_dn8, locals.var_pd_z_dn9, locals.var_pd_z_dn10, locals.var_pd_z_dn11,)
    }
};
        locals.var_pd_z = assign17410_e23013;
        locals.var_pd_z_dn3 = assign17410_e23013_d_n3;
        locals.var_pd_z_dn4 = assign17410_e23013_d_n4;
        locals.var_pd_z_dn5 = assign17410_e23013_d_n5;
        locals.var_pd_z_dn6 = assign17410_e23013_d_n6;
        locals.var_pd_z_dn7 = assign17410_e23013_d_n7;
        locals.var_pd_z_dn8 = assign17410_e23013_d_n8;
        locals.var_pd_z_dn9 = assign17410_e23013_d_n9;
        locals.var_pd_z_dn10 = assign17410_e23013_d_n10;
        locals.var_pd_z_dn11 = assign17410_e23013_d_n11;

        let (assign17420_e23037, assign17420_e23037_d_n3, assign17420_e23037_d_n4, assign17420_e23037_d_n5, assign17420_e23037_d_n6, assign17420_e23037_d_n7, assign17420_e23037_d_n8, assign17420_e23037_d_n9, assign17420_e23037_d_n10, assign17420_e23037_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard493 == 0.0)) && (locals.var_guard494 != 0.0)) {
        let assign17420_e23023: f64 = (locals.var_pd_z + 10.0);
        let assign17420_e23026: f64 = (locals.var_pd_z - 6.0);
        let assign17420_e23029: f64 = (locals.var_pd_z - 6.0);
        let assign17420_e23030: f64 = (assign17420_e23026 * assign17420_e23029);
        let assign17420_e23032: f64 = (assign17420_e23030 + 64.0);
        let assign17420_e23033: f64 = (assign17420_e23032).sqrt();
        let assign17420_e23034: f64 = (assign17420_e23023 - assign17420_e23033);
        let assign17420_e23035: f64 = (0.5 * assign17420_e23034);
        (assign17420_e23035, (0.5 * (locals.var_pd_z_dn3 - (((locals.var_pd_z_dn3 * assign17420_e23029) + (assign17420_e23026 * locals.var_pd_z_dn3)) / (2.0 * assign17420_e23033)))), (0.5 * (locals.var_pd_z_dn4 - (((locals.var_pd_z_dn4 * assign17420_e23029) + (assign17420_e23026 * locals.var_pd_z_dn4)) / (2.0 * assign17420_e23033)))), (0.5 * (locals.var_pd_z_dn5 - (((locals.var_pd_z_dn5 * assign17420_e23029) + (assign17420_e23026 * locals.var_pd_z_dn5)) / (2.0 * assign17420_e23033)))), (0.5 * (locals.var_pd_z_dn6 - (((locals.var_pd_z_dn6 * assign17420_e23029) + (assign17420_e23026 * locals.var_pd_z_dn6)) / (2.0 * assign17420_e23033)))), (0.5 * (locals.var_pd_z_dn7 - (((locals.var_pd_z_dn7 * assign17420_e23029) + (assign17420_e23026 * locals.var_pd_z_dn7)) / (2.0 * assign17420_e23033)))), (0.5 * (locals.var_pd_z_dn8 - (((locals.var_pd_z_dn8 * assign17420_e23029) + (assign17420_e23026 * locals.var_pd_z_dn8)) / (2.0 * assign17420_e23033)))), (0.5 * (locals.var_pd_z_dn9 - (((locals.var_pd_z_dn9 * assign17420_e23029) + (assign17420_e23026 * locals.var_pd_z_dn9)) / (2.0 * assign17420_e23033)))), (0.5 * (locals.var_pd_z_dn10 - (((locals.var_pd_z_dn10 * assign17420_e23029) + (assign17420_e23026 * locals.var_pd_z_dn10)) / (2.0 * assign17420_e23033)))), (0.5 * (locals.var_pd_z_dn11 - (((locals.var_pd_z_dn11 * assign17420_e23029) + (assign17420_e23026 * locals.var_pd_z_dn11)) / (2.0 * assign17420_e23033)))),)
    } else {
        (locals.var_pd_eta, locals.var_pd_eta_dn3, locals.var_pd_eta_dn4, locals.var_pd_eta_dn5, locals.var_pd_eta_dn6, locals.var_pd_eta_dn7, locals.var_pd_eta_dn8, locals.var_pd_eta_dn9, locals.var_pd_eta_dn10, locals.var_pd_eta_dn11,)
    }
};
        locals.var_pd_eta = assign17420_e23037;
        locals.var_pd_eta_dn3 = assign17420_e23037_d_n3;
        locals.var_pd_eta_dn4 = assign17420_e23037_d_n4;
        locals.var_pd_eta_dn5 = assign17420_e23037_d_n5;
        locals.var_pd_eta_dn6 = assign17420_e23037_d_n6;
        locals.var_pd_eta_dn7 = assign17420_e23037_d_n7;
        locals.var_pd_eta_dn8 = assign17420_e23037_d_n8;
        locals.var_pd_eta_dn9 = assign17420_e23037_d_n9;
        locals.var_pd_eta_dn10 = assign17420_e23037_d_n10;
        locals.var_pd_eta_dn11 = assign17420_e23037_d_n11;

    }
}

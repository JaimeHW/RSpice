#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6620_e10122,) = {
    if (locals.var_guard53 != 0.0) {
        let assign6620_e10103: f64 = (locals.var_inv_l * p.p1882);
        let assign6620_e10104: f64 = (p.p1881 + assign6620_e10103);
        let assign6620_e10107: f64 = (locals.var_inv_nfin * p.p1883);
        let assign6620_e10108: f64 = (assign6620_e10104 + assign6620_e10107);
        let assign6620_e10111: f64 = (locals.var_inv_lnfin * p.p1884);
        let assign6620_e10112: f64 = (assign6620_e10108 + assign6620_e10111);
        let assign6620_e10115: f64 = (locals.var_inv_w * p.p1885);
        let assign6620_e10116: f64 = (assign6620_e10112 + assign6620_e10115);
        let assign6620_e10119: f64 = (locals.var_inv_wl * p.p1886);
        let assign6620_e10120: f64 = (assign6620_e10116 + assign6620_e10119);
        (assign6620_e10120,)
    } else {
        (locals.var_mfq3nom_i,)
    }
};
        locals.var_mfq3nom_i = assign6620_e10122;
        locals.var_mfq3nom_i_rv = 0.0;

        let assign6630_e10125: f64 = if p.p100 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard54 = assign6630_e10125;
        locals.var_guard54_rv = 0.0;

        let (assign6640_e10162, assign6640_e10162_d_n0, assign6640_e10162_d_n2, assign6640_e10162_d_n3, assign6640_e10162_d_n4, assign6640_e10162_d_n5, assign6640_e10162_d_n6, assign6640_e10162_d_n7, assign6640_e10162_d_n8, assign6640_e10162_d_n9, assign6640_e10162_d_n10, assign6640_e10162_d_n11, assign6640_e10162_d_n13, assign6640_e10162_d_n14,) = {
    if (locals.var_guard54 != 0.0) {
        let assign6640_e10131: f64 = (p.p100 / p.p5);
        let assign6640_e10135: f64 = (p.p5 / p.p101);
        let assign6640_e10136: f64 = (1.0 + assign6640_e10135);
        let (assign6640_e10157,) = {
            if (!(assign6640_e10136 > 1e-38)) {
                let assign6640_e10141: f64 = (-87.498233534);
                (assign6640_e10141,)
            } else {
                let assign6640_e10145: f64 = (p.p5 / p.p101);
                let assign6640_e10146: f64 = (1.0 + assign6640_e10145);
                let (assign6640_e10156,) = {
                    if (assign6640_e10146 > 1e-38) {
                        let assign6640_e10152: f64 = (p.p5 / p.p101);
                        let assign6640_e10153: f64 = (1.0 + assign6640_e10152);
                        let assign6640_e10154: f64 = (assign6640_e10153).ln();
                        (assign6640_e10154,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6640_e10156,)
            }
        };
        let assign6640_e10158: f64 = (assign6640_e10131 * assign6640_e10157);
        let assign6640_e10159: f64 = (1.0 + assign6640_e10158);
        let assign6640_e10160: f64 = (locals.var_phig_i * assign6640_e10159);
        (assign6640_e10160, (locals.var_phig_i_dn0 * assign6640_e10159), (locals.var_phig_i_dn2 * assign6640_e10159), (locals.var_phig_i_dn3 * assign6640_e10159), (locals.var_phig_i_dn4 * assign6640_e10159), (locals.var_phig_i_dn5 * assign6640_e10159), (locals.var_phig_i_dn6 * assign6640_e10159), (locals.var_phig_i_dn7 * assign6640_e10159), (locals.var_phig_i_dn8 * assign6640_e10159), (locals.var_phig_i_dn9 * assign6640_e10159), (locals.var_phig_i_dn10 * assign6640_e10159), (locals.var_phig_i_dn11 * assign6640_e10159), (locals.var_phig_i_dn13 * assign6640_e10159), (locals.var_phig_i_dn14 * assign6640_e10159),)
    } else {
        (locals.var_phig_i, locals.var_phig_i_dn0, locals.var_phig_i_dn2, locals.var_phig_i_dn3, locals.var_phig_i_dn4, locals.var_phig_i_dn5, locals.var_phig_i_dn6, locals.var_phig_i_dn7, locals.var_phig_i_dn8, locals.var_phig_i_dn9, locals.var_phig_i_dn10, locals.var_phig_i_dn11, locals.var_phig_i_dn13, locals.var_phig_i_dn14,)
    }
};
        locals.var_phig_i = assign6640_e10162;
        locals.var_phig_i_dn0 = assign6640_e10162_d_n0;
        locals.var_phig_i_dn2 = assign6640_e10162_d_n2;
        locals.var_phig_i_dn3 = assign6640_e10162_d_n3;
        locals.var_phig_i_dn4 = assign6640_e10162_d_n4;
        locals.var_phig_i_dn5 = assign6640_e10162_d_n5;
        locals.var_phig_i_dn6 = assign6640_e10162_d_n6;
        locals.var_phig_i_dn7 = assign6640_e10162_d_n7;
        locals.var_phig_i_dn8 = assign6640_e10162_d_n8;
        locals.var_phig_i_dn9 = assign6640_e10162_d_n9;
        locals.var_phig_i_dn10 = assign6640_e10162_d_n10;
        locals.var_phig_i_dn11 = assign6640_e10162_d_n11;
        locals.var_phig_i_dn13 = assign6640_e10162_d_n13;
        locals.var_phig_i_dn14 = assign6640_e10162_d_n14;
        locals.var_phig_i_rv = 0.0;

        let assign6650_e10165: f64 = if p.p158 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign6650_e10165;
        locals.var_guard55_rv = 0.0;

        let (assign6660_e10202, assign6660_e10202_d_n0, assign6660_e10202_d_n2, assign6660_e10202_d_n3, assign6660_e10202_d_n4, assign6660_e10202_d_n5, assign6660_e10202_d_n6, assign6660_e10202_d_n7, assign6660_e10202_d_n8, assign6660_e10202_d_n9, assign6660_e10202_d_n10, assign6660_e10202_d_n11, assign6660_e10202_d_n13, assign6660_e10202_d_n14,) = {
    if (locals.var_guard55 != 0.0) {
        let assign6660_e10171: f64 = (p.p158 / p.p5);
        let assign6660_e10175: f64 = (p.p5 / p.p159);
        let assign6660_e10176: f64 = (1.0 + assign6660_e10175);
        let (assign6660_e10197,) = {
            if (!(assign6660_e10176 > 1e-38)) {
                let assign6660_e10181: f64 = (-87.498233534);
                (assign6660_e10181,)
            } else {
                let assign6660_e10185: f64 = (p.p5 / p.p159);
                let assign6660_e10186: f64 = (1.0 + assign6660_e10185);
                let (assign6660_e10196,) = {
                    if (assign6660_e10186 > 1e-38) {
                        let assign6660_e10192: f64 = (p.p5 / p.p159);
                        let assign6660_e10193: f64 = (1.0 + assign6660_e10192);
                        let assign6660_e10194: f64 = (assign6660_e10193).ln();
                        (assign6660_e10194,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6660_e10196,)
            }
        };
        let assign6660_e10198: f64 = (assign6660_e10171 * assign6660_e10197);
        let assign6660_e10199: f64 = (1.0 + assign6660_e10198);
        let assign6660_e10200: f64 = (locals.var_eta0_i * assign6660_e10199);
        (assign6660_e10200, (locals.var_eta0_i_dn0 * assign6660_e10199), (locals.var_eta0_i_dn2 * assign6660_e10199), (locals.var_eta0_i_dn3 * assign6660_e10199), (locals.var_eta0_i_dn4 * assign6660_e10199), (locals.var_eta0_i_dn5 * assign6660_e10199), (locals.var_eta0_i_dn6 * assign6660_e10199), (locals.var_eta0_i_dn7 * assign6660_e10199), (locals.var_eta0_i_dn8 * assign6660_e10199), (locals.var_eta0_i_dn9 * assign6660_e10199), (locals.var_eta0_i_dn10 * assign6660_e10199), (locals.var_eta0_i_dn11 * assign6660_e10199), (locals.var_eta0_i_dn13 * assign6660_e10199), (locals.var_eta0_i_dn14 * assign6660_e10199),)
    } else {
        (locals.var_eta0_i, locals.var_eta0_i_dn0, locals.var_eta0_i_dn2, locals.var_eta0_i_dn3, locals.var_eta0_i_dn4, locals.var_eta0_i_dn5, locals.var_eta0_i_dn6, locals.var_eta0_i_dn7, locals.var_eta0_i_dn8, locals.var_eta0_i_dn9, locals.var_eta0_i_dn10, locals.var_eta0_i_dn11, locals.var_eta0_i_dn13, locals.var_eta0_i_dn14,)
    }
};
        locals.var_eta0_i = assign6660_e10202;
        locals.var_eta0_i_dn0 = assign6660_e10202_d_n0;
        locals.var_eta0_i_dn2 = assign6660_e10202_d_n2;
        locals.var_eta0_i_dn3 = assign6660_e10202_d_n3;
        locals.var_eta0_i_dn4 = assign6660_e10202_d_n4;
        locals.var_eta0_i_dn5 = assign6660_e10202_d_n5;
        locals.var_eta0_i_dn6 = assign6660_e10202_d_n6;
        locals.var_eta0_i_dn7 = assign6660_e10202_d_n7;
        locals.var_eta0_i_dn8 = assign6660_e10202_d_n8;
        locals.var_eta0_i_dn9 = assign6660_e10202_d_n9;
        locals.var_eta0_i_dn10 = assign6660_e10202_d_n10;
        locals.var_eta0_i_dn11 = assign6660_e10202_d_n11;
        locals.var_eta0_i_dn13 = assign6660_e10202_d_n13;
        locals.var_eta0_i_dn14 = assign6660_e10202_d_n14;
        locals.var_eta0_i_rv = 0.0;

        let assign6670_e10205: f64 = if p.p152 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard56 = assign6670_e10205;
        locals.var_guard56_rv = 0.0;

        let (assign6680_e10242,) = {
    if (locals.var_guard56 != 0.0) {
        let assign6680_e10211: f64 = (p.p152 / p.p5);
        let assign6680_e10215: f64 = (p.p5 / p.p153);
        let assign6680_e10216: f64 = (1.0 + assign6680_e10215);
        let (assign6680_e10237,) = {
            if (!(assign6680_e10216 > 1e-38)) {
                let assign6680_e10221: f64 = (-87.498233534);
                (assign6680_e10221,)
            } else {
                let assign6680_e10225: f64 = (p.p5 / p.p153);
                let assign6680_e10226: f64 = (1.0 + assign6680_e10225);
                let (assign6680_e10236,) = {
                    if (assign6680_e10226 > 1e-38) {
                        let assign6680_e10232: f64 = (p.p5 / p.p153);
                        let assign6680_e10233: f64 = (1.0 + assign6680_e10232);
                        let assign6680_e10234: f64 = (assign6680_e10233).ln();
                        (assign6680_e10234,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6680_e10236,)
            }
        };
        let assign6680_e10238: f64 = (assign6680_e10211 * assign6680_e10237);
        let assign6680_e10239: f64 = (1.0 + assign6680_e10238);
        let assign6680_e10240: f64 = (locals.var_cdsc_i * assign6680_e10239);
        (assign6680_e10240,)
    } else {
        (locals.var_cdsc_i,)
    }
};
        locals.var_cdsc_i = assign6680_e10242;
        locals.var_cdsc_i_rv = 0.0;

        let assign6690_e10245: f64 = if p.p154 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard57 = assign6690_e10245;
        locals.var_guard57_rv = 0.0;

        let (assign6700_e10282,) = {
    if (locals.var_guard57 != 0.0) {
        let assign6700_e10251: f64 = (p.p154 / p.p5);
        let assign6700_e10255: f64 = (p.p5 / p.p155);
        let assign6700_e10256: f64 = (1.0 + assign6700_e10255);
        let (assign6700_e10277,) = {
            if (!(assign6700_e10256 > 1e-38)) {
                let assign6700_e10261: f64 = (-87.498233534);
                (assign6700_e10261,)
            } else {
                let assign6700_e10265: f64 = (p.p5 / p.p155);
                let assign6700_e10266: f64 = (1.0 + assign6700_e10265);
                let (assign6700_e10276,) = {
                    if (assign6700_e10266 > 1e-38) {
                        let assign6700_e10272: f64 = (p.p5 / p.p155);
                        let assign6700_e10273: f64 = (1.0 + assign6700_e10272);
                        let assign6700_e10274: f64 = (assign6700_e10273).ln();
                        (assign6700_e10274,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6700_e10276,)
            }
        };
        let assign6700_e10278: f64 = (assign6700_e10251 * assign6700_e10277);
        let assign6700_e10279: f64 = (1.0 + assign6700_e10278);
        let assign6700_e10280: f64 = (locals.var_cdscd_i * assign6700_e10279);
        (assign6700_e10280,)
    } else {
        (locals.var_cdscd_i,)
    }
};
        locals.var_cdscd_i = assign6700_e10282;
        locals.var_cdscd_i_rv = 0.0;

        let assign6710_e10285: f64 = if p.p156 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard58 = assign6710_e10285;
        locals.var_guard58_rv = 0.0;

        let (assign6720_e10322,) = {
    if (locals.var_guard58 != 0.0) {
        let assign6720_e10291: f64 = (p.p156 / p.p5);
        let assign6720_e10295: f64 = (p.p5 / p.p157);
        let assign6720_e10296: f64 = (1.0 + assign6720_e10295);
        let (assign6720_e10317,) = {
            if (!(assign6720_e10296 > 1e-38)) {
                let assign6720_e10301: f64 = (-87.498233534);
                (assign6720_e10301,)
            } else {
                let assign6720_e10305: f64 = (p.p5 / p.p157);
                let assign6720_e10306: f64 = (1.0 + assign6720_e10305);
                let (assign6720_e10316,) = {
                    if (assign6720_e10306 > 1e-38) {
                        let assign6720_e10312: f64 = (p.p5 / p.p157);
                        let assign6720_e10313: f64 = (1.0 + assign6720_e10312);
                        let assign6720_e10314: f64 = (assign6720_e10313).ln();
                        (assign6720_e10314,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6720_e10316,)
            }
        };
        let assign6720_e10318: f64 = (assign6720_e10291 * assign6720_e10317);
        let assign6720_e10319: f64 = (1.0 + assign6720_e10318);
        let assign6720_e10320: f64 = (locals.var_cdscdr_i * assign6720_e10319);
        (assign6720_e10320,)
    } else {
        (locals.var_cdscdr_i,)
    }
};
        locals.var_cdscdr_i = assign6720_e10322;
        locals.var_cdscdr_i_rv = 0.0;

        let assign6730_e10325: f64 = if p.p428 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard59 = assign6730_e10325;
        locals.var_guard59_rv = 0.0;

        let (assign6740_e10362, assign6740_e10362_d_n0, assign6740_e10362_d_n2, assign6740_e10362_d_n3, assign6740_e10362_d_n4, assign6740_e10362_d_n5, assign6740_e10362_d_n6, assign6740_e10362_d_n7, assign6740_e10362_d_n8, assign6740_e10362_d_n9, assign6740_e10362_d_n10, assign6740_e10362_d_n11, assign6740_e10362_d_n13, assign6740_e10362_d_n14,) = {
    if (locals.var_guard59 != 0.0) {
        let assign6740_e10331: f64 = (p.p428 / p.p5);
        let assign6740_e10335: f64 = (p.p5 / p.p429);
        let assign6740_e10336: f64 = (1.0 + assign6740_e10335);
        let (assign6740_e10357,) = {
            if (!(assign6740_e10336 > 1e-38)) {
                let assign6740_e10341: f64 = (-87.498233534);
                (assign6740_e10341,)
            } else {
                let assign6740_e10345: f64 = (p.p5 / p.p429);
                let assign6740_e10346: f64 = (1.0 + assign6740_e10345);
                let (assign6740_e10356,) = {
                    if (assign6740_e10346 > 1e-38) {
                        let assign6740_e10352: f64 = (p.p5 / p.p429);
                        let assign6740_e10353: f64 = (1.0 + assign6740_e10352);
                        let assign6740_e10354: f64 = (assign6740_e10353).ln();
                        (assign6740_e10354,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6740_e10356,)
            }
        };
        let assign6740_e10358: f64 = (assign6740_e10331 * assign6740_e10357);
        let assign6740_e10359: f64 = (1.0 + assign6740_e10358);
        let assign6740_e10360: f64 = (locals.var_vsat_i * assign6740_e10359);
        (assign6740_e10360, (locals.var_vsat_i_dn0 * assign6740_e10359), (locals.var_vsat_i_dn2 * assign6740_e10359), (locals.var_vsat_i_dn3 * assign6740_e10359), (locals.var_vsat_i_dn4 * assign6740_e10359), (locals.var_vsat_i_dn5 * assign6740_e10359), (locals.var_vsat_i_dn6 * assign6740_e10359), (locals.var_vsat_i_dn7 * assign6740_e10359), (locals.var_vsat_i_dn8 * assign6740_e10359), (locals.var_vsat_i_dn9 * assign6740_e10359), (locals.var_vsat_i_dn10 * assign6740_e10359), (locals.var_vsat_i_dn11 * assign6740_e10359), (locals.var_vsat_i_dn13 * assign6740_e10359), (locals.var_vsat_i_dn14 * assign6740_e10359),)
    } else {
        (locals.var_vsat_i, locals.var_vsat_i_dn0, locals.var_vsat_i_dn2, locals.var_vsat_i_dn3, locals.var_vsat_i_dn4, locals.var_vsat_i_dn5, locals.var_vsat_i_dn6, locals.var_vsat_i_dn7, locals.var_vsat_i_dn8, locals.var_vsat_i_dn9, locals.var_vsat_i_dn10, locals.var_vsat_i_dn11, locals.var_vsat_i_dn13, locals.var_vsat_i_dn14,)
    }
};
        locals.var_vsat_i = assign6740_e10362;
        locals.var_vsat_i_dn0 = assign6740_e10362_d_n0;
        locals.var_vsat_i_dn2 = assign6740_e10362_d_n2;
        locals.var_vsat_i_dn3 = assign6740_e10362_d_n3;
        locals.var_vsat_i_dn4 = assign6740_e10362_d_n4;
        locals.var_vsat_i_dn5 = assign6740_e10362_d_n5;
        locals.var_vsat_i_dn6 = assign6740_e10362_d_n6;
        locals.var_vsat_i_dn7 = assign6740_e10362_d_n7;
        locals.var_vsat_i_dn8 = assign6740_e10362_d_n8;
        locals.var_vsat_i_dn9 = assign6740_e10362_d_n9;
        locals.var_vsat_i_dn10 = assign6740_e10362_d_n10;
        locals.var_vsat_i_dn11 = assign6740_e10362_d_n11;
        locals.var_vsat_i_dn13 = assign6740_e10362_d_n13;
        locals.var_vsat_i_dn14 = assign6740_e10362_d_n14;
        locals.var_vsat_i_rv = 0.0;

        let assign6750_e10365: f64 = if p.p432 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard60 = assign6750_e10365;
        locals.var_guard60_rv = 0.0;

        let (assign6760_e10402, assign6760_e10402_d_n0, assign6760_e10402_d_n2, assign6760_e10402_d_n3, assign6760_e10402_d_n4, assign6760_e10402_d_n5, assign6760_e10402_d_n6, assign6760_e10402_d_n7, assign6760_e10402_d_n8, assign6760_e10402_d_n9, assign6760_e10402_d_n10, assign6760_e10402_d_n11, assign6760_e10402_d_n13, assign6760_e10402_d_n14,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6760_e10371: f64 = (p.p432 / p.p5);
        let assign6760_e10375: f64 = (p.p5 / p.p433);
        let assign6760_e10376: f64 = (1.0 + assign6760_e10375);
        let (assign6760_e10397,) = {
            if (!(assign6760_e10376 > 1e-38)) {
                let assign6760_e10381: f64 = (-87.498233534);
                (assign6760_e10381,)
            } else {
                let assign6760_e10385: f64 = (p.p5 / p.p433);
                let assign6760_e10386: f64 = (1.0 + assign6760_e10385);
                let (assign6760_e10396,) = {
                    if (assign6760_e10386 > 1e-38) {
                        let assign6760_e10392: f64 = (p.p5 / p.p433);
                        let assign6760_e10393: f64 = (1.0 + assign6760_e10392);
                        let assign6760_e10394: f64 = (assign6760_e10393).ln();
                        (assign6760_e10394,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6760_e10396,)
            }
        };
        let assign6760_e10398: f64 = (assign6760_e10371 * assign6760_e10397);
        let assign6760_e10399: f64 = (1.0 + assign6760_e10398);
        let assign6760_e10400: f64 = (locals.var_vsat1_i * assign6760_e10399);
        (assign6760_e10400, (locals.var_vsat1_i_dn0 * assign6760_e10399), (locals.var_vsat1_i_dn2 * assign6760_e10399), (locals.var_vsat1_i_dn3 * assign6760_e10399), (locals.var_vsat1_i_dn4 * assign6760_e10399), (locals.var_vsat1_i_dn5 * assign6760_e10399), (locals.var_vsat1_i_dn6 * assign6760_e10399), (locals.var_vsat1_i_dn7 * assign6760_e10399), (locals.var_vsat1_i_dn8 * assign6760_e10399), (locals.var_vsat1_i_dn9 * assign6760_e10399), (locals.var_vsat1_i_dn10 * assign6760_e10399), (locals.var_vsat1_i_dn11 * assign6760_e10399), (locals.var_vsat1_i_dn13 * assign6760_e10399), (locals.var_vsat1_i_dn14 * assign6760_e10399),)
    } else {
        (locals.var_vsat1_i, locals.var_vsat1_i_dn0, locals.var_vsat1_i_dn2, locals.var_vsat1_i_dn3, locals.var_vsat1_i_dn4, locals.var_vsat1_i_dn5, locals.var_vsat1_i_dn6, locals.var_vsat1_i_dn7, locals.var_vsat1_i_dn8, locals.var_vsat1_i_dn9, locals.var_vsat1_i_dn10, locals.var_vsat1_i_dn11, locals.var_vsat1_i_dn13, locals.var_vsat1_i_dn14,)
    }
};
        locals.var_vsat1_i = assign6760_e10402;
        locals.var_vsat1_i_dn0 = assign6760_e10402_d_n0;
        locals.var_vsat1_i_dn2 = assign6760_e10402_d_n2;
        locals.var_vsat1_i_dn3 = assign6760_e10402_d_n3;
        locals.var_vsat1_i_dn4 = assign6760_e10402_d_n4;
        locals.var_vsat1_i_dn5 = assign6760_e10402_d_n5;
        locals.var_vsat1_i_dn6 = assign6760_e10402_d_n6;
        locals.var_vsat1_i_dn7 = assign6760_e10402_d_n7;
        locals.var_vsat1_i_dn8 = assign6760_e10402_d_n8;
        locals.var_vsat1_i_dn9 = assign6760_e10402_d_n9;
        locals.var_vsat1_i_dn10 = assign6760_e10402_d_n10;
        locals.var_vsat1_i_dn11 = assign6760_e10402_d_n11;
        locals.var_vsat1_i_dn13 = assign6760_e10402_d_n13;
        locals.var_vsat1_i_dn14 = assign6760_e10402_d_n14;
        locals.var_vsat1_i_rv = 0.0;

        let assign6770_e10405: f64 = if p.p434 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard61 = assign6770_e10405;
        locals.var_guard61_rv = 0.0;

        let (assign6780_e10442, assign6780_e10442_d_n0, assign6780_e10442_d_n2, assign6780_e10442_d_n3, assign6780_e10442_d_n4, assign6780_e10442_d_n5, assign6780_e10442_d_n6, assign6780_e10442_d_n7, assign6780_e10442_d_n8, assign6780_e10442_d_n9, assign6780_e10442_d_n10, assign6780_e10442_d_n11, assign6780_e10442_d_n13, assign6780_e10442_d_n14,) = {
    if (locals.var_guard61 != 0.0) {
        let assign6780_e10411: f64 = (p.p434 / p.p5);
        let assign6780_e10415: f64 = (p.p5 / p.p435);
        let assign6780_e10416: f64 = (1.0 + assign6780_e10415);
        let (assign6780_e10437,) = {
            if (!(assign6780_e10416 > 1e-38)) {
                let assign6780_e10421: f64 = (-87.498233534);
                (assign6780_e10421,)
            } else {
                let assign6780_e10425: f64 = (p.p5 / p.p435);
                let assign6780_e10426: f64 = (1.0 + assign6780_e10425);
                let (assign6780_e10436,) = {
                    if (assign6780_e10426 > 1e-38) {
                        let assign6780_e10432: f64 = (p.p5 / p.p435);
                        let assign6780_e10433: f64 = (1.0 + assign6780_e10432);
                        let assign6780_e10434: f64 = (assign6780_e10433).ln();
                        (assign6780_e10434,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6780_e10436,)
            }
        };
        let assign6780_e10438: f64 = (assign6780_e10411 * assign6780_e10437);
        let assign6780_e10439: f64 = (1.0 + assign6780_e10438);
        let assign6780_e10440: f64 = (locals.var_vsat1r_i * assign6780_e10439);
        (assign6780_e10440, (locals.var_vsat1r_i_dn0 * assign6780_e10439), (locals.var_vsat1r_i_dn2 * assign6780_e10439), (locals.var_vsat1r_i_dn3 * assign6780_e10439), (locals.var_vsat1r_i_dn4 * assign6780_e10439), (locals.var_vsat1r_i_dn5 * assign6780_e10439), (locals.var_vsat1r_i_dn6 * assign6780_e10439), (locals.var_vsat1r_i_dn7 * assign6780_e10439), (locals.var_vsat1r_i_dn8 * assign6780_e10439), (locals.var_vsat1r_i_dn9 * assign6780_e10439), (locals.var_vsat1r_i_dn10 * assign6780_e10439), (locals.var_vsat1r_i_dn11 * assign6780_e10439), (locals.var_vsat1r_i_dn13 * assign6780_e10439), (locals.var_vsat1r_i_dn14 * assign6780_e10439),)
    } else {
        (locals.var_vsat1r_i, locals.var_vsat1r_i_dn0, locals.var_vsat1r_i_dn2, locals.var_vsat1r_i_dn3, locals.var_vsat1r_i_dn4, locals.var_vsat1r_i_dn5, locals.var_vsat1r_i_dn6, locals.var_vsat1r_i_dn7, locals.var_vsat1r_i_dn8, locals.var_vsat1r_i_dn9, locals.var_vsat1r_i_dn10, locals.var_vsat1r_i_dn11, locals.var_vsat1r_i_dn13, locals.var_vsat1r_i_dn14,)
    }
};
        locals.var_vsat1r_i = assign6780_e10442;
        locals.var_vsat1r_i_dn0 = assign6780_e10442_d_n0;
        locals.var_vsat1r_i_dn2 = assign6780_e10442_d_n2;
        locals.var_vsat1r_i_dn3 = assign6780_e10442_d_n3;
        locals.var_vsat1r_i_dn4 = assign6780_e10442_d_n4;
        locals.var_vsat1r_i_dn5 = assign6780_e10442_d_n5;
        locals.var_vsat1r_i_dn6 = assign6780_e10442_d_n6;
        locals.var_vsat1r_i_dn7 = assign6780_e10442_d_n7;
        locals.var_vsat1r_i_dn8 = assign6780_e10442_d_n8;
        locals.var_vsat1r_i_dn9 = assign6780_e10442_d_n9;
        locals.var_vsat1r_i_dn10 = assign6780_e10442_d_n10;
        locals.var_vsat1r_i_dn11 = assign6780_e10442_d_n11;
        locals.var_vsat1r_i_dn13 = assign6780_e10442_d_n13;
        locals.var_vsat1r_i_dn14 = assign6780_e10442_d_n14;
        locals.var_vsat1r_i_rv = 0.0;

        let assign6790_e10445: f64 = if p.p581 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard62 = assign6790_e10445;
        locals.var_guard62_rv = 0.0;

        let (assign6800_e10482, assign6800_e10482_d_n0, assign6800_e10482_d_n2, assign6800_e10482_d_n3, assign6800_e10482_d_n4, assign6800_e10482_d_n5, assign6800_e10482_d_n6, assign6800_e10482_d_n7, assign6800_e10482_d_n8, assign6800_e10482_d_n9, assign6800_e10482_d_n10, assign6800_e10482_d_n11, assign6800_e10482_d_n13, assign6800_e10482_d_n14,) = {
    if (locals.var_guard62 != 0.0) {
        let assign6800_e10451: f64 = (p.p581 / p.p5);
        let assign6800_e10455: f64 = (p.p5 / p.p584);
        let assign6800_e10456: f64 = (1.0 + assign6800_e10455);
        let (assign6800_e10477,) = {
            if (!(assign6800_e10456 > 1e-38)) {
                let assign6800_e10461: f64 = (-87.498233534);
                (assign6800_e10461,)
            } else {
                let assign6800_e10465: f64 = (p.p5 / p.p584);
                let assign6800_e10466: f64 = (1.0 + assign6800_e10465);
                let (assign6800_e10476,) = {
                    if (assign6800_e10466 > 1e-38) {
                        let assign6800_e10472: f64 = (p.p5 / p.p584);
                        let assign6800_e10473: f64 = (1.0 + assign6800_e10472);
                        let assign6800_e10474: f64 = (assign6800_e10473).ln();
                        (assign6800_e10474,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6800_e10476,)
            }
        };
        let assign6800_e10478: f64 = (assign6800_e10451 * assign6800_e10477);
        let assign6800_e10479: f64 = (1.0 + assign6800_e10478);
        let assign6800_e10480: f64 = (locals.var_u0_i * assign6800_e10479);
        (assign6800_e10480, (locals.var_u0_i_dn0 * assign6800_e10479), (locals.var_u0_i_dn2 * assign6800_e10479), (locals.var_u0_i_dn3 * assign6800_e10479), (locals.var_u0_i_dn4 * assign6800_e10479), (locals.var_u0_i_dn5 * assign6800_e10479), (locals.var_u0_i_dn6 * assign6800_e10479), (locals.var_u0_i_dn7 * assign6800_e10479), (locals.var_u0_i_dn8 * assign6800_e10479), (locals.var_u0_i_dn9 * assign6800_e10479), (locals.var_u0_i_dn10 * assign6800_e10479), (locals.var_u0_i_dn11 * assign6800_e10479), (locals.var_u0_i_dn13 * assign6800_e10479), (locals.var_u0_i_dn14 * assign6800_e10479),)
    } else {
        (locals.var_u0_i, locals.var_u0_i_dn0, locals.var_u0_i_dn2, locals.var_u0_i_dn3, locals.var_u0_i_dn4, locals.var_u0_i_dn5, locals.var_u0_i_dn6, locals.var_u0_i_dn7, locals.var_u0_i_dn8, locals.var_u0_i_dn9, locals.var_u0_i_dn10, locals.var_u0_i_dn11, locals.var_u0_i_dn13, locals.var_u0_i_dn14,)
    }
};
        locals.var_u0_i = assign6800_e10482;
        locals.var_u0_i_dn0 = assign6800_e10482_d_n0;
        locals.var_u0_i_dn2 = assign6800_e10482_d_n2;
        locals.var_u0_i_dn3 = assign6800_e10482_d_n3;
        locals.var_u0_i_dn4 = assign6800_e10482_d_n4;
        locals.var_u0_i_dn5 = assign6800_e10482_d_n5;
        locals.var_u0_i_dn6 = assign6800_e10482_d_n6;
        locals.var_u0_i_dn7 = assign6800_e10482_d_n7;
        locals.var_u0_i_dn8 = assign6800_e10482_d_n8;
        locals.var_u0_i_dn9 = assign6800_e10482_d_n9;
        locals.var_u0_i_dn10 = assign6800_e10482_d_n10;
        locals.var_u0_i_dn11 = assign6800_e10482_d_n11;
        locals.var_u0_i_dn13 = assign6800_e10482_d_n13;
        locals.var_u0_i_dn14 = assign6800_e10482_d_n14;
        locals.var_u0_i_rv = 0.0;

        let assign6810_e10485: f64 = if p.p583 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard63 = assign6810_e10485;
        locals.var_guard63_rv = 0.0;

        let (assign6820_e10522, assign6820_e10522_d_n0, assign6820_e10522_d_n2, assign6820_e10522_d_n3, assign6820_e10522_d_n4, assign6820_e10522_d_n5, assign6820_e10522_d_n6, assign6820_e10522_d_n7, assign6820_e10522_d_n8, assign6820_e10522_d_n9, assign6820_e10522_d_n10, assign6820_e10522_d_n11, assign6820_e10522_d_n13, assign6820_e10522_d_n14,) = {
    if (locals.var_guard63 != 0.0) {
        let assign6820_e10491: f64 = (p.p583 / p.p5);
        let assign6820_e10495: f64 = (p.p5 / p.p586);
        let assign6820_e10496: f64 = (1.0 + assign6820_e10495);
        let (assign6820_e10517,) = {
            if (!(assign6820_e10496 > 1e-38)) {
                let assign6820_e10501: f64 = (-87.498233534);
                (assign6820_e10501,)
            } else {
                let assign6820_e10505: f64 = (p.p5 / p.p586);
                let assign6820_e10506: f64 = (1.0 + assign6820_e10505);
                let (assign6820_e10516,) = {
                    if (assign6820_e10506 > 1e-38) {
                        let assign6820_e10512: f64 = (p.p5 / p.p586);
                        let assign6820_e10513: f64 = (1.0 + assign6820_e10512);
                        let assign6820_e10514: f64 = (assign6820_e10513).ln();
                        (assign6820_e10514,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6820_e10516,)
            }
        };
        let assign6820_e10518: f64 = (assign6820_e10491 * assign6820_e10517);
        let assign6820_e10519: f64 = (1.0 + assign6820_e10518);
        let assign6820_e10520: f64 = (locals.var_u0r_i * assign6820_e10519);
        (assign6820_e10520, (locals.var_u0r_i_dn0 * assign6820_e10519), (locals.var_u0r_i_dn2 * assign6820_e10519), (locals.var_u0r_i_dn3 * assign6820_e10519), (locals.var_u0r_i_dn4 * assign6820_e10519), (locals.var_u0r_i_dn5 * assign6820_e10519), (locals.var_u0r_i_dn6 * assign6820_e10519), (locals.var_u0r_i_dn7 * assign6820_e10519), (locals.var_u0r_i_dn8 * assign6820_e10519), (locals.var_u0r_i_dn9 * assign6820_e10519), (locals.var_u0r_i_dn10 * assign6820_e10519), (locals.var_u0r_i_dn11 * assign6820_e10519), (locals.var_u0r_i_dn13 * assign6820_e10519), (locals.var_u0r_i_dn14 * assign6820_e10519),)
    } else {
        (locals.var_u0r_i, locals.var_u0r_i_dn0, locals.var_u0r_i_dn2, locals.var_u0r_i_dn3, locals.var_u0r_i_dn4, locals.var_u0r_i_dn5, locals.var_u0r_i_dn6, locals.var_u0r_i_dn7, locals.var_u0r_i_dn8, locals.var_u0r_i_dn9, locals.var_u0r_i_dn10, locals.var_u0r_i_dn11, locals.var_u0r_i_dn13, locals.var_u0r_i_dn14,)
    }
};
        locals.var_u0r_i = assign6820_e10522;
        locals.var_u0r_i_dn0 = assign6820_e10522_d_n0;
        locals.var_u0r_i_dn2 = assign6820_e10522_d_n2;
        locals.var_u0r_i_dn3 = assign6820_e10522_d_n3;
        locals.var_u0r_i_dn4 = assign6820_e10522_d_n4;
        locals.var_u0r_i_dn5 = assign6820_e10522_d_n5;
        locals.var_u0r_i_dn6 = assign6820_e10522_d_n6;
        locals.var_u0r_i_dn7 = assign6820_e10522_d_n7;
        locals.var_u0r_i_dn8 = assign6820_e10522_d_n8;
        locals.var_u0r_i_dn9 = assign6820_e10522_d_n9;
        locals.var_u0r_i_dn10 = assign6820_e10522_d_n10;
        locals.var_u0r_i_dn11 = assign6820_e10522_d_n11;
        locals.var_u0r_i_dn13 = assign6820_e10522_d_n13;
        locals.var_u0r_i_dn14 = assign6820_e10522_d_n14;
        locals.var_u0r_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign6830_e10525: f64 = if p.p21 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard64 = assign6830_e10525;
        locals.var_guard64_rv = 0.0;

        let (assign6840_e10539, assign6840_e10539_d_n0, assign6840_e10539_d_n2, assign6840_e10539_d_n3, assign6840_e10539_d_n4, assign6840_e10539_d_n5, assign6840_e10539_d_n6, assign6840_e10539_d_n7, assign6840_e10539_d_n8, assign6840_e10539_d_n9, assign6840_e10539_d_n10, assign6840_e10539_d_n11, assign6840_e10539_d_n13, assign6840_e10539_d_n14,) = {
    if (locals.var_guard64 != 0.0) {
        let assign6840_e10531: f64 = (p.p5 - p.p21);
        let assign6840_e10533: f64 = (assign6840_e10531 * p.p99);
        let assign6840_e10535: f64 = (assign6840_e10533 * locals.var_leff_1);
        let assign6840_e10536: f64 = (1.0 + assign6840_e10535);
        let assign6840_e10537: f64 = (locals.var_phig_i * assign6840_e10536);
        (assign6840_e10537, ((locals.var_phig_i_dn0 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn0))), ((locals.var_phig_i_dn2 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn2))), ((locals.var_phig_i_dn3 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn3))), ((locals.var_phig_i_dn4 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn4))), ((locals.var_phig_i_dn5 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn5))), ((locals.var_phig_i_dn6 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn6))), ((locals.var_phig_i_dn7 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn7))), ((locals.var_phig_i_dn8 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn8))), ((locals.var_phig_i_dn9 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn9))), ((locals.var_phig_i_dn10 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn10))), ((locals.var_phig_i_dn11 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn11))), ((locals.var_phig_i_dn13 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn13))), ((locals.var_phig_i_dn14 * assign6840_e10536) + (locals.var_phig_i * (assign6840_e10533 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_phig_i, locals.var_phig_i_dn0, locals.var_phig_i_dn2, locals.var_phig_i_dn3, locals.var_phig_i_dn4, locals.var_phig_i_dn5, locals.var_phig_i_dn6, locals.var_phig_i_dn7, locals.var_phig_i_dn8, locals.var_phig_i_dn9, locals.var_phig_i_dn10, locals.var_phig_i_dn11, locals.var_phig_i_dn13, locals.var_phig_i_dn14,)
    }
};
        locals.var_phig_i = assign6840_e10539;
        locals.var_phig_i_dn0 = assign6840_e10539_d_n0;
        locals.var_phig_i_dn2 = assign6840_e10539_d_n2;
        locals.var_phig_i_dn3 = assign6840_e10539_d_n3;
        locals.var_phig_i_dn4 = assign6840_e10539_d_n4;
        locals.var_phig_i_dn5 = assign6840_e10539_d_n5;
        locals.var_phig_i_dn6 = assign6840_e10539_d_n6;
        locals.var_phig_i_dn7 = assign6840_e10539_d_n7;
        locals.var_phig_i_dn8 = assign6840_e10539_d_n8;
        locals.var_phig_i_dn9 = assign6840_e10539_d_n9;
        locals.var_phig_i_dn10 = assign6840_e10539_d_n10;
        locals.var_phig_i_dn11 = assign6840_e10539_d_n11;
        locals.var_phig_i_dn13 = assign6840_e10539_d_n13;
        locals.var_phig_i_dn14 = assign6840_e10539_d_n14;
        locals.var_phig_i_rv = 0.0;

        let (assign6850_e10553, assign6850_e10553_d_n0, assign6850_e10553_d_n2, assign6850_e10553_d_n3, assign6850_e10553_d_n4, assign6850_e10553_d_n5, assign6850_e10553_d_n6, assign6850_e10553_d_n7, assign6850_e10553_d_n8, assign6850_e10553_d_n9, assign6850_e10553_d_n10, assign6850_e10553_d_n11, assign6850_e10553_d_n13, assign6850_e10553_d_n14,) = {
    if (locals.var_guard64 != 0.0) {
        let assign6850_e10545: f64 = (p.p5 - p.p21);
        let assign6850_e10547: f64 = (assign6850_e10545 * p.p160);
        let assign6850_e10549: f64 = (assign6850_e10547 * locals.var_leff_1);
        let assign6850_e10550: f64 = (1.0 + assign6850_e10549);
        let assign6850_e10551: f64 = (locals.var_eta0_i * assign6850_e10550);
        (assign6850_e10551, ((locals.var_eta0_i_dn0 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn0))), ((locals.var_eta0_i_dn2 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn2))), ((locals.var_eta0_i_dn3 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn3))), ((locals.var_eta0_i_dn4 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn4))), ((locals.var_eta0_i_dn5 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn5))), ((locals.var_eta0_i_dn6 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn6))), ((locals.var_eta0_i_dn7 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn7))), ((locals.var_eta0_i_dn8 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn8))), ((locals.var_eta0_i_dn9 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn9))), ((locals.var_eta0_i_dn10 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn10))), ((locals.var_eta0_i_dn11 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn11))), ((locals.var_eta0_i_dn13 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn13))), ((locals.var_eta0_i_dn14 * assign6850_e10550) + (locals.var_eta0_i * (assign6850_e10547 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_eta0_i, locals.var_eta0_i_dn0, locals.var_eta0_i_dn2, locals.var_eta0_i_dn3, locals.var_eta0_i_dn4, locals.var_eta0_i_dn5, locals.var_eta0_i_dn6, locals.var_eta0_i_dn7, locals.var_eta0_i_dn8, locals.var_eta0_i_dn9, locals.var_eta0_i_dn10, locals.var_eta0_i_dn11, locals.var_eta0_i_dn13, locals.var_eta0_i_dn14,)
    }
};
        locals.var_eta0_i = assign6850_e10553;
        locals.var_eta0_i_dn0 = assign6850_e10553_d_n0;
        locals.var_eta0_i_dn2 = assign6850_e10553_d_n2;
        locals.var_eta0_i_dn3 = assign6850_e10553_d_n3;
        locals.var_eta0_i_dn4 = assign6850_e10553_d_n4;
        locals.var_eta0_i_dn5 = assign6850_e10553_d_n5;
        locals.var_eta0_i_dn6 = assign6850_e10553_d_n6;
        locals.var_eta0_i_dn7 = assign6850_e10553_d_n7;
        locals.var_eta0_i_dn8 = assign6850_e10553_d_n8;
        locals.var_eta0_i_dn9 = assign6850_e10553_d_n9;
        locals.var_eta0_i_dn10 = assign6850_e10553_d_n10;
        locals.var_eta0_i_dn11 = assign6850_e10553_d_n11;
        locals.var_eta0_i_dn13 = assign6850_e10553_d_n13;
        locals.var_eta0_i_dn14 = assign6850_e10553_d_n14;
        locals.var_eta0_i_rv = 0.0;

        let (assign6860_e10567, assign6860_e10567_d_n0, assign6860_e10567_d_n2, assign6860_e10567_d_n3, assign6860_e10567_d_n4, assign6860_e10567_d_n5, assign6860_e10567_d_n6, assign6860_e10567_d_n7, assign6860_e10567_d_n8, assign6860_e10567_d_n9, assign6860_e10567_d_n10, assign6860_e10567_d_n11, assign6860_e10567_d_n13, assign6860_e10567_d_n14,) = {
    if (locals.var_guard64 != 0.0) {
        let assign6860_e10559: f64 = (p.p5 - p.p21);
        let assign6860_e10561: f64 = (assign6860_e10559 * p.p587);
        let assign6860_e10563: f64 = (assign6860_e10561 * locals.var_leff_1);
        let assign6860_e10564: f64 = (1.0 + assign6860_e10563);
        let assign6860_e10565: f64 = (locals.var_u0_i * assign6860_e10564);
        (assign6860_e10565, ((locals.var_u0_i_dn0 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn0))), ((locals.var_u0_i_dn2 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn2))), ((locals.var_u0_i_dn3 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn3))), ((locals.var_u0_i_dn4 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn4))), ((locals.var_u0_i_dn5 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn5))), ((locals.var_u0_i_dn6 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn6))), ((locals.var_u0_i_dn7 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn7))), ((locals.var_u0_i_dn8 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn8))), ((locals.var_u0_i_dn9 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn9))), ((locals.var_u0_i_dn10 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn10))), ((locals.var_u0_i_dn11 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn11))), ((locals.var_u0_i_dn13 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn13))), ((locals.var_u0_i_dn14 * assign6860_e10564) + (locals.var_u0_i * (assign6860_e10561 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_u0_i, locals.var_u0_i_dn0, locals.var_u0_i_dn2, locals.var_u0_i_dn3, locals.var_u0_i_dn4, locals.var_u0_i_dn5, locals.var_u0_i_dn6, locals.var_u0_i_dn7, locals.var_u0_i_dn8, locals.var_u0_i_dn9, locals.var_u0_i_dn10, locals.var_u0_i_dn11, locals.var_u0_i_dn13, locals.var_u0_i_dn14,)
    }
};
        locals.var_u0_i = assign6860_e10567;
        locals.var_u0_i_dn0 = assign6860_e10567_d_n0;
        locals.var_u0_i_dn2 = assign6860_e10567_d_n2;
        locals.var_u0_i_dn3 = assign6860_e10567_d_n3;
        locals.var_u0_i_dn4 = assign6860_e10567_d_n4;
        locals.var_u0_i_dn5 = assign6860_e10567_d_n5;
        locals.var_u0_i_dn6 = assign6860_e10567_d_n6;
        locals.var_u0_i_dn7 = assign6860_e10567_d_n7;
        locals.var_u0_i_dn8 = assign6860_e10567_d_n8;
        locals.var_u0_i_dn9 = assign6860_e10567_d_n9;
        locals.var_u0_i_dn10 = assign6860_e10567_d_n10;
        locals.var_u0_i_dn11 = assign6860_e10567_d_n11;
        locals.var_u0_i_dn13 = assign6860_e10567_d_n13;
        locals.var_u0_i_dn14 = assign6860_e10567_d_n14;
        locals.var_u0_i_rv = 0.0;

        let assign6870_e10569: f64 = (locals.var_leff_1).ln();
        locals.var_leff_ln = assign6870_e10569;
        locals.var_leff_ln_dn0 = (locals.var_leff_1_dn0 / locals.var_leff_1);
        locals.var_leff_ln_dn2 = (locals.var_leff_1_dn2 / locals.var_leff_1);
        locals.var_leff_ln_dn3 = (locals.var_leff_1_dn3 / locals.var_leff_1);
        locals.var_leff_ln_dn4 = (locals.var_leff_1_dn4 / locals.var_leff_1);
        locals.var_leff_ln_dn5 = (locals.var_leff_1_dn5 / locals.var_leff_1);
        locals.var_leff_ln_dn6 = (locals.var_leff_1_dn6 / locals.var_leff_1);
        locals.var_leff_ln_dn7 = (locals.var_leff_1_dn7 / locals.var_leff_1);
        locals.var_leff_ln_dn8 = (locals.var_leff_1_dn8 / locals.var_leff_1);
        locals.var_leff_ln_dn9 = (locals.var_leff_1_dn9 / locals.var_leff_1);
        locals.var_leff_ln_dn10 = (locals.var_leff_1_dn10 / locals.var_leff_1);
        locals.var_leff_ln_dn11 = (locals.var_leff_1_dn11 / locals.var_leff_1);
        locals.var_leff_ln_dn13 = (locals.var_leff_1_dn13 / locals.var_leff_1);
        locals.var_leff_ln_dn14 = (locals.var_leff_1_dn14 / locals.var_leff_1);
        locals.var_leff_ln_rv = 0.0;

        let assign6880_e10573: f64 = (p.p98 * locals.var_leff_1);
        let assign6880_e10574: f64 = (locals.var_phig_i + assign6880_e10573);
        locals.var_phig_i = assign6880_e10574;
        locals.var_phig_i_dn0 = (locals.var_phig_i_dn0 + (p.p98 * locals.var_leff_1_dn0));
        locals.var_phig_i_dn2 = (locals.var_phig_i_dn2 + (p.p98 * locals.var_leff_1_dn2));
        locals.var_phig_i_dn3 = (locals.var_phig_i_dn3 + (p.p98 * locals.var_leff_1_dn3));
        locals.var_phig_i_dn4 = (locals.var_phig_i_dn4 + (p.p98 * locals.var_leff_1_dn4));
        locals.var_phig_i_dn5 = (locals.var_phig_i_dn5 + (p.p98 * locals.var_leff_1_dn5));
        locals.var_phig_i_dn6 = (locals.var_phig_i_dn6 + (p.p98 * locals.var_leff_1_dn6));
        locals.var_phig_i_dn7 = (locals.var_phig_i_dn7 + (p.p98 * locals.var_leff_1_dn7));
        locals.var_phig_i_dn8 = (locals.var_phig_i_dn8 + (p.p98 * locals.var_leff_1_dn8));
        locals.var_phig_i_dn9 = (locals.var_phig_i_dn9 + (p.p98 * locals.var_leff_1_dn9));
        locals.var_phig_i_dn10 = (locals.var_phig_i_dn10 + (p.p98 * locals.var_leff_1_dn10));
        locals.var_phig_i_dn11 = (locals.var_phig_i_dn11 + (p.p98 * locals.var_leff_1_dn11));
        locals.var_phig_i_dn13 = (locals.var_phig_i_dn13 + (p.p98 * locals.var_leff_1_dn13));
        locals.var_phig_i_dn14 = (locals.var_phig_i_dn14 + (p.p98 * locals.var_leff_1_dn14));
        locals.var_phig_i_rv = 0.0;

        let assign6890_e10578: f64 = (p.p427 * locals.var_leff_1);
        let assign6890_e10579: f64 = (locals.var_pqm_i + assign6890_e10578);
        locals.var_pqm_i = assign6890_e10579;
        locals.var_pqm_i_dn0 = (locals.var_pqm_i_dn0 + (p.p427 * locals.var_leff_1_dn0));
        locals.var_pqm_i_dn2 = (locals.var_pqm_i_dn2 + (p.p427 * locals.var_leff_1_dn2));
        locals.var_pqm_i_dn3 = (locals.var_pqm_i_dn3 + (p.p427 * locals.var_leff_1_dn3));
        locals.var_pqm_i_dn4 = (locals.var_pqm_i_dn4 + (p.p427 * locals.var_leff_1_dn4));
        locals.var_pqm_i_dn5 = (locals.var_pqm_i_dn5 + (p.p427 * locals.var_leff_1_dn5));
        locals.var_pqm_i_dn6 = (locals.var_pqm_i_dn6 + (p.p427 * locals.var_leff_1_dn6));
        locals.var_pqm_i_dn7 = (locals.var_pqm_i_dn7 + (p.p427 * locals.var_leff_1_dn7));
        locals.var_pqm_i_dn8 = (locals.var_pqm_i_dn8 + (p.p427 * locals.var_leff_1_dn8));
        locals.var_pqm_i_dn9 = (locals.var_pqm_i_dn9 + (p.p427 * locals.var_leff_1_dn9));
        locals.var_pqm_i_dn10 = (locals.var_pqm_i_dn10 + (p.p427 * locals.var_leff_1_dn10));
        locals.var_pqm_i_dn11 = (locals.var_pqm_i_dn11 + (p.p427 * locals.var_leff_1_dn11));
        locals.var_pqm_i_dn13 = (locals.var_pqm_i_dn13 + (p.p427 * locals.var_leff_1_dn13));
        locals.var_pqm_i_dn14 = (locals.var_pqm_i_dn14 + (p.p427 * locals.var_leff_1_dn14));
        locals.var_pqm_i_rv = 0.0;

        let assign6900_e10582: f64 = if p.p589 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard65 = assign6900_e10582;
        locals.var_guard65_rv = 0.0;

        let (assign6910_e10596, assign6910_e10596_d_n0, assign6910_e10596_d_n2, assign6910_e10596_d_n3, assign6910_e10596_d_n4, assign6910_e10596_d_n5, assign6910_e10596_d_n6, assign6910_e10596_d_n7, assign6910_e10596_d_n8, assign6910_e10596_d_n9, assign6910_e10596_d_n10, assign6910_e10596_d_n11, assign6910_e10596_d_n13, assign6910_e10596_d_n14,) = {
    if (locals.var_guard65 != 0.0) {
        let assign6910_e10588: f64 = (-p.p589);
        let assign6910_e10590: f64 = (assign6910_e10588 * locals.var_leff_ln);
        let assign6910_e10591: f64 = (assign6910_e10590).exp();
        let assign6910_e10592: f64 = (locals.var_up_i * assign6910_e10591);
        let assign6910_e10593: f64 = (1.0 - assign6910_e10592);
        let assign6910_e10594: f64 = (locals.var_u0_i * assign6910_e10593);
        (assign6910_e10594, ((locals.var_u0_i_dn0 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn0)))))), ((locals.var_u0_i_dn2 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn2)))))), ((locals.var_u0_i_dn3 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn3)))))), ((locals.var_u0_i_dn4 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn4)))))), ((locals.var_u0_i_dn5 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn5)))))), ((locals.var_u0_i_dn6 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn6)))))), ((locals.var_u0_i_dn7 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn7)))))), ((locals.var_u0_i_dn8 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn8)))))), ((locals.var_u0_i_dn9 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn9)))))), ((locals.var_u0_i_dn10 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn10)))))), ((locals.var_u0_i_dn11 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn11)))))), ((locals.var_u0_i_dn13 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn13)))))), ((locals.var_u0_i_dn14 * assign6910_e10593) + (locals.var_u0_i * (-(locals.var_up_i * (assign6910_e10591 * (assign6910_e10588 * locals.var_leff_ln_dn14)))))),)
    } else {
        (locals.var_u0_i, locals.var_u0_i_dn0, locals.var_u0_i_dn2, locals.var_u0_i_dn3, locals.var_u0_i_dn4, locals.var_u0_i_dn5, locals.var_u0_i_dn6, locals.var_u0_i_dn7, locals.var_u0_i_dn8, locals.var_u0_i_dn9, locals.var_u0_i_dn10, locals.var_u0_i_dn11, locals.var_u0_i_dn13, locals.var_u0_i_dn14,)
    }
};
        locals.var_u0_i = assign6910_e10596;
        locals.var_u0_i_dn0 = assign6910_e10596_d_n0;
        locals.var_u0_i_dn2 = assign6910_e10596_d_n2;
        locals.var_u0_i_dn3 = assign6910_e10596_d_n3;
        locals.var_u0_i_dn4 = assign6910_e10596_d_n4;
        locals.var_u0_i_dn5 = assign6910_e10596_d_n5;
        locals.var_u0_i_dn6 = assign6910_e10596_d_n6;
        locals.var_u0_i_dn7 = assign6910_e10596_d_n7;
        locals.var_u0_i_dn8 = assign6910_e10596_d_n8;
        locals.var_u0_i_dn9 = assign6910_e10596_d_n9;
        locals.var_u0_i_dn10 = assign6910_e10596_d_n10;
        locals.var_u0_i_dn11 = assign6910_e10596_d_n11;
        locals.var_u0_i_dn13 = assign6910_e10596_d_n13;
        locals.var_u0_i_dn14 = assign6910_e10596_d_n14;
        locals.var_u0_i_rv = 0.0;

        let (assign6920_e10605, assign6920_e10605_d_n0, assign6920_e10605_d_n2, assign6920_e10605_d_n3, assign6920_e10605_d_n4, assign6920_e10605_d_n5, assign6920_e10605_d_n6, assign6920_e10605_d_n7, assign6920_e10605_d_n8, assign6920_e10605_d_n9, assign6920_e10605_d_n10, assign6920_e10605_d_n11, assign6920_e10605_d_n13, assign6920_e10605_d_n14,) = {
    if (locals.var_guard65 == 0.0) {
        let assign6920_e10602: f64 = (1.0 - locals.var_up_i);
        let assign6920_e10603: f64 = (locals.var_u0_i * assign6920_e10602);
        (assign6920_e10603, (locals.var_u0_i_dn0 * assign6920_e10602), (locals.var_u0_i_dn2 * assign6920_e10602), (locals.var_u0_i_dn3 * assign6920_e10602), (locals.var_u0_i_dn4 * assign6920_e10602), (locals.var_u0_i_dn5 * assign6920_e10602), (locals.var_u0_i_dn6 * assign6920_e10602), (locals.var_u0_i_dn7 * assign6920_e10602), (locals.var_u0_i_dn8 * assign6920_e10602), (locals.var_u0_i_dn9 * assign6920_e10602), (locals.var_u0_i_dn10 * assign6920_e10602), (locals.var_u0_i_dn11 * assign6920_e10602), (locals.var_u0_i_dn13 * assign6920_e10602), (locals.var_u0_i_dn14 * assign6920_e10602),)
    } else {
        (locals.var_u0_i, locals.var_u0_i_dn0, locals.var_u0_i_dn2, locals.var_u0_i_dn3, locals.var_u0_i_dn4, locals.var_u0_i_dn5, locals.var_u0_i_dn6, locals.var_u0_i_dn7, locals.var_u0_i_dn8, locals.var_u0_i_dn9, locals.var_u0_i_dn10, locals.var_u0_i_dn11, locals.var_u0_i_dn13, locals.var_u0_i_dn14,)
    }
};
        locals.var_u0_i = assign6920_e10605;
        locals.var_u0_i_dn0 = assign6920_e10605_d_n0;
        locals.var_u0_i_dn2 = assign6920_e10605_d_n2;
        locals.var_u0_i_dn3 = assign6920_e10605_d_n3;
        locals.var_u0_i_dn4 = assign6920_e10605_d_n4;
        locals.var_u0_i_dn5 = assign6920_e10605_d_n5;
        locals.var_u0_i_dn6 = assign6920_e10605_d_n6;
        locals.var_u0_i_dn7 = assign6920_e10605_d_n7;
        locals.var_u0_i_dn8 = assign6920_e10605_d_n8;
        locals.var_u0_i_dn9 = assign6920_e10605_d_n9;
        locals.var_u0_i_dn10 = assign6920_e10605_d_n10;
        locals.var_u0_i_dn11 = assign6920_e10605_d_n11;
        locals.var_u0_i_dn13 = assign6920_e10605_d_n13;
        locals.var_u0_i_dn14 = assign6920_e10605_d_n14;
        locals.var_u0_i_rv = 0.0;

        let assign6930_e10609: f64 = (-locals.var_leff_1);
        let assign6930_e10611: f64 = (assign6930_e10609 / p.p593);
        let assign6930_e10612: f64 = { let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6930_e10613: f64 = (p.p591 * assign6930_e10612);
        let assign6930_e10614: f64 = (locals.var_ua_i + assign6930_e10613);
        locals.var_ua_i = assign6930_e10614;
        locals.var_ua_i_dn0 = (locals.var_ua_i_dn0 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p593))));
        locals.var_ua_i_dn2 = (locals.var_ua_i_dn2 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p593))));
        locals.var_ua_i_dn3 = (locals.var_ua_i_dn3 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p593))));
        locals.var_ua_i_dn4 = (locals.var_ua_i_dn4 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p593))));
        locals.var_ua_i_dn5 = (locals.var_ua_i_dn5 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p593))));
        locals.var_ua_i_dn6 = (locals.var_ua_i_dn6 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p593))));
        locals.var_ua_i_dn7 = (locals.var_ua_i_dn7 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p593))));
        locals.var_ua_i_dn8 = (locals.var_ua_i_dn8 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p593))));
        locals.var_ua_i_dn9 = (locals.var_ua_i_dn9 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p593))));
        locals.var_ua_i_dn10 = (locals.var_ua_i_dn10 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p593))));
        locals.var_ua_i_dn11 = (locals.var_ua_i_dn11 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p593))));
        locals.var_ua_i_dn13 = (locals.var_ua_i_dn13 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p593))));
        locals.var_ua_i_dn14 = (locals.var_ua_i_dn14 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p593))));
        locals.var_ua_i_rv = 0.0;

        let assign6940_e10618: f64 = (-locals.var_leff_1);
        let assign6940_e10620: f64 = (assign6940_e10618 / p.p601);
        let assign6940_e10621: f64 = { let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6940_e10622: f64 = (p.p599 * assign6940_e10621);
        let assign6940_e10623: f64 = (locals.var_ud_i + assign6940_e10622);
        locals.var_ud_i = assign6940_e10623;
        locals.var_ud_i_dn0 = (locals.var_ud_i_dn0 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p601))));
        locals.var_ud_i_dn2 = (locals.var_ud_i_dn2 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p601))));
        locals.var_ud_i_dn3 = (locals.var_ud_i_dn3 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p601))));
        locals.var_ud_i_dn4 = (locals.var_ud_i_dn4 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p601))));
        locals.var_ud_i_dn5 = (locals.var_ud_i_dn5 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p601))));
        locals.var_ud_i_dn6 = (locals.var_ud_i_dn6 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p601))));
        locals.var_ud_i_dn7 = (locals.var_ud_i_dn7 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p601))));
        locals.var_ud_i_dn8 = (locals.var_ud_i_dn8 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p601))));
        locals.var_ud_i_dn9 = (locals.var_ud_i_dn9 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p601))));
        locals.var_ud_i_dn10 = (locals.var_ud_i_dn10 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p601))));
        locals.var_ud_i_dn11 = (locals.var_ud_i_dn11 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p601))));
        locals.var_ud_i_dn13 = (locals.var_ud_i_dn13 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p601))));
        locals.var_ud_i_dn14 = (locals.var_ud_i_dn14 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p601))));
        locals.var_ud_i_rv = 0.0;

        let assign6950_e10627: f64 = (-locals.var_leff_1);
        let assign6950_e10629: f64 = (assign6950_e10627 / p.p597);
        let assign6950_e10630: f64 = { let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6950_e10631: f64 = (p.p595 * assign6950_e10630);
        let assign6950_e10632: f64 = (locals.var_eu_i + assign6950_e10631);
        locals.var_eu_i = assign6950_e10632;
        locals.var_eu_i_dn0 = (locals.var_eu_i_dn0 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p597))));
        locals.var_eu_i_dn2 = (locals.var_eu_i_dn2 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p597))));
        locals.var_eu_i_dn3 = (locals.var_eu_i_dn3 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p597))));
        locals.var_eu_i_dn4 = (locals.var_eu_i_dn4 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p597))));
        locals.var_eu_i_dn5 = (locals.var_eu_i_dn5 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p597))));
        locals.var_eu_i_dn6 = (locals.var_eu_i_dn6 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p597))));
        locals.var_eu_i_dn7 = (locals.var_eu_i_dn7 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p597))));
        locals.var_eu_i_dn8 = (locals.var_eu_i_dn8 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p597))));
        locals.var_eu_i_dn9 = (locals.var_eu_i_dn9 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p597))));
        locals.var_eu_i_dn10 = (locals.var_eu_i_dn10 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p597))));
        locals.var_eu_i_dn11 = (locals.var_eu_i_dn11 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p597))));
        locals.var_eu_i_dn13 = (locals.var_eu_i_dn13 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p597))));
        locals.var_eu_i_dn14 = (locals.var_eu_i_dn14 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p597))));
        locals.var_eu_i_rv = 0.0;

        let assign6960_e10635: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard66 = assign6960_e10635;
        locals.var_guard66_rv = 0.0;

        let (assign6970_e10647, assign6970_e10647_d_n0, assign6970_e10647_d_n2, assign6970_e10647_d_n3, assign6970_e10647_d_n4, assign6970_e10647_d_n5, assign6970_e10647_d_n6, assign6970_e10647_d_n7, assign6970_e10647_d_n8, assign6970_e10647_d_n9, assign6970_e10647_d_n10, assign6970_e10647_d_n11, assign6970_e10647_d_n13, assign6970_e10647_d_n14,) = {
    if (locals.var_guard66 != 0.0) {
        let assign6970_e10640: f64 = (-locals.var_leff_1);
        let assign6970_e10642: f64 = (assign6970_e10640 / p.p594);
        let assign6970_e10643: f64 = { let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6970_e10644: f64 = (p.p592 * assign6970_e10643);
        let assign6970_e10645: f64 = (locals.var_uar_i + assign6970_e10644);
        (assign6970_e10645, (locals.var_uar_i_dn0 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p594)))), (locals.var_uar_i_dn2 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p594)))), (locals.var_uar_i_dn3 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p594)))), (locals.var_uar_i_dn4 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p594)))), (locals.var_uar_i_dn5 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p594)))), (locals.var_uar_i_dn6 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p594)))), (locals.var_uar_i_dn7 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p594)))), (locals.var_uar_i_dn8 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p594)))), (locals.var_uar_i_dn9 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p594)))), (locals.var_uar_i_dn10 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p594)))), (locals.var_uar_i_dn11 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p594)))), (locals.var_uar_i_dn13 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p594)))), (locals.var_uar_i_dn14 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p594)))),)
    } else {
        (locals.var_uar_i, locals.var_uar_i_dn0, locals.var_uar_i_dn2, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11, locals.var_uar_i_dn13, locals.var_uar_i_dn14,)
    }
};
        locals.var_uar_i = assign6970_e10647;
        locals.var_uar_i_dn0 = assign6970_e10647_d_n0;
        locals.var_uar_i_dn2 = assign6970_e10647_d_n2;
        locals.var_uar_i_dn3 = assign6970_e10647_d_n3;
        locals.var_uar_i_dn4 = assign6970_e10647_d_n4;
        locals.var_uar_i_dn5 = assign6970_e10647_d_n5;
        locals.var_uar_i_dn6 = assign6970_e10647_d_n6;
        locals.var_uar_i_dn7 = assign6970_e10647_d_n7;
        locals.var_uar_i_dn8 = assign6970_e10647_d_n8;
        locals.var_uar_i_dn9 = assign6970_e10647_d_n9;
        locals.var_uar_i_dn10 = assign6970_e10647_d_n10;
        locals.var_uar_i_dn11 = assign6970_e10647_d_n11;
        locals.var_uar_i_dn13 = assign6970_e10647_d_n13;
        locals.var_uar_i_dn14 = assign6970_e10647_d_n14;
        locals.var_uar_i_rv = 0.0;

        let (assign6980_e10659, assign6980_e10659_d_n0, assign6980_e10659_d_n2, assign6980_e10659_d_n3, assign6980_e10659_d_n4, assign6980_e10659_d_n5, assign6980_e10659_d_n6, assign6980_e10659_d_n7, assign6980_e10659_d_n8, assign6980_e10659_d_n9, assign6980_e10659_d_n10, assign6980_e10659_d_n11, assign6980_e10659_d_n13, assign6980_e10659_d_n14,) = {
    if (locals.var_guard66 != 0.0) {
        let assign6980_e10652: f64 = (-locals.var_leff_1);
        let assign6980_e10654: f64 = (assign6980_e10652 / p.p602);
        let assign6980_e10655: f64 = { let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6980_e10656: f64 = (p.p600 * assign6980_e10655);
        let assign6980_e10657: f64 = (locals.var_udr_i + assign6980_e10656);
        (assign6980_e10657, (locals.var_udr_i_dn0 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p602)))), (locals.var_udr_i_dn2 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p602)))), (locals.var_udr_i_dn3 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p602)))), (locals.var_udr_i_dn4 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p602)))), (locals.var_udr_i_dn5 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p602)))), (locals.var_udr_i_dn6 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p602)))), (locals.var_udr_i_dn7 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p602)))), (locals.var_udr_i_dn8 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p602)))), (locals.var_udr_i_dn9 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p602)))), (locals.var_udr_i_dn10 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p602)))), (locals.var_udr_i_dn11 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p602)))), (locals.var_udr_i_dn13 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p602)))), (locals.var_udr_i_dn14 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p602)))),)
    } else {
        (locals.var_udr_i, locals.var_udr_i_dn0, locals.var_udr_i_dn2, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11, locals.var_udr_i_dn13, locals.var_udr_i_dn14,)
    }
};
        locals.var_udr_i = assign6980_e10659;
        locals.var_udr_i_dn0 = assign6980_e10659_d_n0;
        locals.var_udr_i_dn2 = assign6980_e10659_d_n2;
        locals.var_udr_i_dn3 = assign6980_e10659_d_n3;
        locals.var_udr_i_dn4 = assign6980_e10659_d_n4;
        locals.var_udr_i_dn5 = assign6980_e10659_d_n5;
        locals.var_udr_i_dn6 = assign6980_e10659_d_n6;
        locals.var_udr_i_dn7 = assign6980_e10659_d_n7;
        locals.var_udr_i_dn8 = assign6980_e10659_d_n8;
        locals.var_udr_i_dn9 = assign6980_e10659_d_n9;
        locals.var_udr_i_dn10 = assign6980_e10659_d_n10;
        locals.var_udr_i_dn11 = assign6980_e10659_d_n11;
        locals.var_udr_i_dn13 = assign6980_e10659_d_n13;
        locals.var_udr_i_dn14 = assign6980_e10659_d_n14;
        locals.var_udr_i_rv = 0.0;

        let (assign6990_e10671, assign6990_e10671_d_n0, assign6990_e10671_d_n2, assign6990_e10671_d_n3, assign6990_e10671_d_n4, assign6990_e10671_d_n5, assign6990_e10671_d_n6, assign6990_e10671_d_n7, assign6990_e10671_d_n8, assign6990_e10671_d_n9, assign6990_e10671_d_n10, assign6990_e10671_d_n11, assign6990_e10671_d_n13, assign6990_e10671_d_n14,) = {
    if (locals.var_guard66 != 0.0) {
        let assign6990_e10664: f64 = (-locals.var_leff_1);
        let assign6990_e10666: f64 = (assign6990_e10664 / p.p598);
        let assign6990_e10667: f64 = { let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6990_e10668: f64 = (p.p596 * assign6990_e10667);
        let assign6990_e10669: f64 = (locals.var_eur_i + assign6990_e10668);
        (assign6990_e10669, (locals.var_eur_i_dn0 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p598)))), (locals.var_eur_i_dn2 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p598)))), (locals.var_eur_i_dn3 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p598)))), (locals.var_eur_i_dn4 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p598)))), (locals.var_eur_i_dn5 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p598)))), (locals.var_eur_i_dn6 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p598)))), (locals.var_eur_i_dn7 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p598)))), (locals.var_eur_i_dn8 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p598)))), (locals.var_eur_i_dn9 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p598)))), (locals.var_eur_i_dn10 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p598)))), (locals.var_eur_i_dn11 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p598)))), (locals.var_eur_i_dn13 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p598)))), (locals.var_eur_i_dn14 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p598)))),)
    } else {
        (locals.var_eur_i, locals.var_eur_i_dn0, locals.var_eur_i_dn2, locals.var_eur_i_dn3, locals.var_eur_i_dn4, locals.var_eur_i_dn5, locals.var_eur_i_dn6, locals.var_eur_i_dn7, locals.var_eur_i_dn8, locals.var_eur_i_dn9, locals.var_eur_i_dn10, locals.var_eur_i_dn11, locals.var_eur_i_dn13, locals.var_eur_i_dn14,)
    }
};
        locals.var_eur_i = assign6990_e10671;
        locals.var_eur_i_dn0 = assign6990_e10671_d_n0;
        locals.var_eur_i_dn2 = assign6990_e10671_d_n2;
        locals.var_eur_i_dn3 = assign6990_e10671_d_n3;
        locals.var_eur_i_dn4 = assign6990_e10671_d_n4;
        locals.var_eur_i_dn5 = assign6990_e10671_d_n5;
        locals.var_eur_i_dn6 = assign6990_e10671_d_n6;
        locals.var_eur_i_dn7 = assign6990_e10671_d_n7;
        locals.var_eur_i_dn8 = assign6990_e10671_d_n8;
        locals.var_eur_i_dn9 = assign6990_e10671_d_n9;
        locals.var_eur_i_dn10 = assign6990_e10671_d_n10;
        locals.var_eur_i_dn11 = assign6990_e10671_d_n11;
        locals.var_eur_i_dn13 = assign6990_e10671_d_n13;
        locals.var_eur_i_dn14 = assign6990_e10671_d_n14;
        locals.var_eur_i_rv = 0.0;

        let assign7000_e10674: f64 = if p.p590 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard67 = assign7000_e10674;
        locals.var_guard67_rv = 0.0;

        let (assign7010_e10690, assign7010_e10690_d_n0, assign7010_e10690_d_n2, assign7010_e10690_d_n3, assign7010_e10690_d_n4, assign7010_e10690_d_n5, assign7010_e10690_d_n6, assign7010_e10690_d_n7, assign7010_e10690_d_n8, assign7010_e10690_d_n9, assign7010_e10690_d_n10, assign7010_e10690_d_n11, assign7010_e10690_d_n13, assign7010_e10690_d_n14,) = {
    if ((locals.var_guard66 != 0.0) && (locals.var_guard67 != 0.0)) {
        let assign7010_e10682: f64 = (-p.p590);
        let assign7010_e10684: f64 = (assign7010_e10682 * locals.var_leff_ln);
        let assign7010_e10685: f64 = (assign7010_e10684).exp();
        let assign7010_e10686: f64 = (locals.var_upr_i * assign7010_e10685);
        let assign7010_e10687: f64 = (1.0 - assign7010_e10686);
        let assign7010_e10688: f64 = (locals.var_u0r_i * assign7010_e10687);
        (assign7010_e10688, ((locals.var_u0r_i_dn0 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn0)))))), ((locals.var_u0r_i_dn2 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn2)))))), ((locals.var_u0r_i_dn3 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn3)))))), ((locals.var_u0r_i_dn4 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn4)))))), ((locals.var_u0r_i_dn5 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn5)))))), ((locals.var_u0r_i_dn6 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn6)))))), ((locals.var_u0r_i_dn7 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn7)))))), ((locals.var_u0r_i_dn8 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn8)))))), ((locals.var_u0r_i_dn9 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn9)))))), ((locals.var_u0r_i_dn10 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn10)))))), ((locals.var_u0r_i_dn11 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn11)))))), ((locals.var_u0r_i_dn13 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn13)))))), ((locals.var_u0r_i_dn14 * assign7010_e10687) + (locals.var_u0r_i * (-(locals.var_upr_i * (assign7010_e10685 * (assign7010_e10682 * locals.var_leff_ln_dn14)))))),)
    } else {
        (locals.var_u0r_i, locals.var_u0r_i_dn0, locals.var_u0r_i_dn2, locals.var_u0r_i_dn3, locals.var_u0r_i_dn4, locals.var_u0r_i_dn5, locals.var_u0r_i_dn6, locals.var_u0r_i_dn7, locals.var_u0r_i_dn8, locals.var_u0r_i_dn9, locals.var_u0r_i_dn10, locals.var_u0r_i_dn11, locals.var_u0r_i_dn13, locals.var_u0r_i_dn14,)
    }
};
        locals.var_u0r_i = assign7010_e10690;
        locals.var_u0r_i_dn0 = assign7010_e10690_d_n0;
        locals.var_u0r_i_dn2 = assign7010_e10690_d_n2;
        locals.var_u0r_i_dn3 = assign7010_e10690_d_n3;
        locals.var_u0r_i_dn4 = assign7010_e10690_d_n4;
        locals.var_u0r_i_dn5 = assign7010_e10690_d_n5;
        locals.var_u0r_i_dn6 = assign7010_e10690_d_n6;
        locals.var_u0r_i_dn7 = assign7010_e10690_d_n7;
        locals.var_u0r_i_dn8 = assign7010_e10690_d_n8;
        locals.var_u0r_i_dn9 = assign7010_e10690_d_n9;
        locals.var_u0r_i_dn10 = assign7010_e10690_d_n10;
        locals.var_u0r_i_dn11 = assign7010_e10690_d_n11;
        locals.var_u0r_i_dn13 = assign7010_e10690_d_n13;
        locals.var_u0r_i_dn14 = assign7010_e10690_d_n14;
        locals.var_u0r_i_rv = 0.0;

        let (assign7020_e10701, assign7020_e10701_d_n0, assign7020_e10701_d_n2, assign7020_e10701_d_n3, assign7020_e10701_d_n4, assign7020_e10701_d_n5, assign7020_e10701_d_n6, assign7020_e10701_d_n7, assign7020_e10701_d_n8, assign7020_e10701_d_n9, assign7020_e10701_d_n10, assign7020_e10701_d_n11, assign7020_e10701_d_n13, assign7020_e10701_d_n14,) = {
    if ((locals.var_guard66 != 0.0) && (locals.var_guard67 == 0.0)) {
        let assign7020_e10698: f64 = (1.0 - locals.var_upr_i);
        let assign7020_e10699: f64 = (locals.var_u0r_i * assign7020_e10698);
        (assign7020_e10699, (locals.var_u0r_i_dn0 * assign7020_e10698), (locals.var_u0r_i_dn2 * assign7020_e10698), (locals.var_u0r_i_dn3 * assign7020_e10698), (locals.var_u0r_i_dn4 * assign7020_e10698), (locals.var_u0r_i_dn5 * assign7020_e10698), (locals.var_u0r_i_dn6 * assign7020_e10698), (locals.var_u0r_i_dn7 * assign7020_e10698), (locals.var_u0r_i_dn8 * assign7020_e10698), (locals.var_u0r_i_dn9 * assign7020_e10698), (locals.var_u0r_i_dn10 * assign7020_e10698), (locals.var_u0r_i_dn11 * assign7020_e10698), (locals.var_u0r_i_dn13 * assign7020_e10698), (locals.var_u0r_i_dn14 * assign7020_e10698),)
    } else {
        (locals.var_u0r_i, locals.var_u0r_i_dn0, locals.var_u0r_i_dn2, locals.var_u0r_i_dn3, locals.var_u0r_i_dn4, locals.var_u0r_i_dn5, locals.var_u0r_i_dn6, locals.var_u0r_i_dn7, locals.var_u0r_i_dn8, locals.var_u0r_i_dn9, locals.var_u0r_i_dn10, locals.var_u0r_i_dn11, locals.var_u0r_i_dn13, locals.var_u0r_i_dn14,)
    }
};
        locals.var_u0r_i = assign7020_e10701;
        locals.var_u0r_i_dn0 = assign7020_e10701_d_n0;
        locals.var_u0r_i_dn2 = assign7020_e10701_d_n2;
        locals.var_u0r_i_dn3 = assign7020_e10701_d_n3;
        locals.var_u0r_i_dn4 = assign7020_e10701_d_n4;
        locals.var_u0r_i_dn5 = assign7020_e10701_d_n5;
        locals.var_u0r_i_dn6 = assign7020_e10701_d_n6;
        locals.var_u0r_i_dn7 = assign7020_e10701_d_n7;
        locals.var_u0r_i_dn8 = assign7020_e10701_d_n8;
        locals.var_u0r_i_dn9 = assign7020_e10701_d_n9;
        locals.var_u0r_i_dn10 = assign7020_e10701_d_n10;
        locals.var_u0r_i_dn11 = assign7020_e10701_d_n11;
        locals.var_u0r_i_dn13 = assign7020_e10701_d_n13;
        locals.var_u0r_i_dn14 = assign7020_e10701_d_n14;
        locals.var_u0r_i_rv = 0.0;

        let assign7030_e10704: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign7030_e10704;
        locals.var_guard68_rv = 0.0;

        let (assign7040_e10716, assign7040_e10716_d_n0, assign7040_e10716_d_n2, assign7040_e10716_d_n3, assign7040_e10716_d_n4, assign7040_e10716_d_n5, assign7040_e10716_d_n6, assign7040_e10716_d_n7, assign7040_e10716_d_n8, assign7040_e10716_d_n9, assign7040_e10716_d_n10, assign7040_e10716_d_n11, assign7040_e10716_d_n13, assign7040_e10716_d_n14,) = {
    if (locals.var_guard68 != 0.0) {
        let assign7040_e10709: f64 = (-locals.var_leff_1);
        let assign7040_e10711: f64 = (assign7040_e10709 / p.p913);
        let assign7040_e10712: f64 = { let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7040_e10713: f64 = (p.p912 * assign7040_e10712);
        let assign7040_e10714: f64 = (locals.var_rsw_i + assign7040_e10713);
        (assign7040_e10714, (locals.var_rsw_i_dn0 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p913)))), (locals.var_rsw_i_dn2 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p913)))), (locals.var_rsw_i_dn3 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p913)))), (locals.var_rsw_i_dn4 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p913)))), (locals.var_rsw_i_dn5 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p913)))), (locals.var_rsw_i_dn6 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p913)))), (locals.var_rsw_i_dn7 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p913)))), (locals.var_rsw_i_dn8 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p913)))), (locals.var_rsw_i_dn9 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p913)))), (locals.var_rsw_i_dn10 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p913)))), (locals.var_rsw_i_dn11 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p913)))), (locals.var_rsw_i_dn13 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p913)))), (locals.var_rsw_i_dn14 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p913)))),)
    } else {
        (locals.var_rsw_i, locals.var_rsw_i_dn0, locals.var_rsw_i_dn2, locals.var_rsw_i_dn3, locals.var_rsw_i_dn4, locals.var_rsw_i_dn5, locals.var_rsw_i_dn6, locals.var_rsw_i_dn7, locals.var_rsw_i_dn8, locals.var_rsw_i_dn9, locals.var_rsw_i_dn10, locals.var_rsw_i_dn11, locals.var_rsw_i_dn13, locals.var_rsw_i_dn14,)
    }
};
        locals.var_rsw_i = assign7040_e10716;
        locals.var_rsw_i_dn0 = assign7040_e10716_d_n0;
        locals.var_rsw_i_dn2 = assign7040_e10716_d_n2;
        locals.var_rsw_i_dn3 = assign7040_e10716_d_n3;
        locals.var_rsw_i_dn4 = assign7040_e10716_d_n4;
        locals.var_rsw_i_dn5 = assign7040_e10716_d_n5;
        locals.var_rsw_i_dn6 = assign7040_e10716_d_n6;
        locals.var_rsw_i_dn7 = assign7040_e10716_d_n7;
        locals.var_rsw_i_dn8 = assign7040_e10716_d_n8;
        locals.var_rsw_i_dn9 = assign7040_e10716_d_n9;
        locals.var_rsw_i_dn10 = assign7040_e10716_d_n10;
        locals.var_rsw_i_dn11 = assign7040_e10716_d_n11;
        locals.var_rsw_i_dn13 = assign7040_e10716_d_n13;
        locals.var_rsw_i_dn14 = assign7040_e10716_d_n14;
        locals.var_rsw_i_rv = 0.0;

        let (assign7050_e10728, assign7050_e10728_d_n0, assign7050_e10728_d_n2, assign7050_e10728_d_n3, assign7050_e10728_d_n4, assign7050_e10728_d_n5, assign7050_e10728_d_n6, assign7050_e10728_d_n7, assign7050_e10728_d_n8, assign7050_e10728_d_n9, assign7050_e10728_d_n10, assign7050_e10728_d_n11, assign7050_e10728_d_n13, assign7050_e10728_d_n14,) = {
    if (locals.var_guard68 != 0.0) {
        let assign7050_e10721: f64 = (-locals.var_leff_1);
        let assign7050_e10723: f64 = (assign7050_e10721 / p.p916);
        let assign7050_e10724: f64 = { let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7050_e10725: f64 = (p.p915 * assign7050_e10724);
        let assign7050_e10726: f64 = (locals.var_rdw_i + assign7050_e10725);
        (assign7050_e10726, (locals.var_rdw_i_dn0 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p916)))), (locals.var_rdw_i_dn2 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p916)))), (locals.var_rdw_i_dn3 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p916)))), (locals.var_rdw_i_dn4 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p916)))), (locals.var_rdw_i_dn5 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p916)))), (locals.var_rdw_i_dn6 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p916)))), (locals.var_rdw_i_dn7 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p916)))), (locals.var_rdw_i_dn8 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p916)))), (locals.var_rdw_i_dn9 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p916)))), (locals.var_rdw_i_dn10 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p916)))), (locals.var_rdw_i_dn11 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p916)))), (locals.var_rdw_i_dn13 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p916)))), (locals.var_rdw_i_dn14 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p916)))),)
    } else {
        (locals.var_rdw_i, locals.var_rdw_i_dn0, locals.var_rdw_i_dn2, locals.var_rdw_i_dn3, locals.var_rdw_i_dn4, locals.var_rdw_i_dn5, locals.var_rdw_i_dn6, locals.var_rdw_i_dn7, locals.var_rdw_i_dn8, locals.var_rdw_i_dn9, locals.var_rdw_i_dn10, locals.var_rdw_i_dn11, locals.var_rdw_i_dn13, locals.var_rdw_i_dn14,)
    }
};
        locals.var_rdw_i = assign7050_e10728;
        locals.var_rdw_i_dn0 = assign7050_e10728_d_n0;
        locals.var_rdw_i_dn2 = assign7050_e10728_d_n2;
        locals.var_rdw_i_dn3 = assign7050_e10728_d_n3;
        locals.var_rdw_i_dn4 = assign7050_e10728_d_n4;
        locals.var_rdw_i_dn5 = assign7050_e10728_d_n5;
        locals.var_rdw_i_dn6 = assign7050_e10728_d_n6;
        locals.var_rdw_i_dn7 = assign7050_e10728_d_n7;
        locals.var_rdw_i_dn8 = assign7050_e10728_d_n8;
        locals.var_rdw_i_dn9 = assign7050_e10728_d_n9;
        locals.var_rdw_i_dn10 = assign7050_e10728_d_n10;
        locals.var_rdw_i_dn11 = assign7050_e10728_d_n11;
        locals.var_rdw_i_dn13 = assign7050_e10728_d_n13;
        locals.var_rdw_i_dn14 = assign7050_e10728_d_n14;
        locals.var_rdw_i_rv = 0.0;

        let (assign7060_e10741, assign7060_e10741_d_n0, assign7060_e10741_d_n2, assign7060_e10741_d_n3, assign7060_e10741_d_n4, assign7060_e10741_d_n5, assign7060_e10741_d_n6, assign7060_e10741_d_n7, assign7060_e10741_d_n8, assign7060_e10741_d_n9, assign7060_e10741_d_n10, assign7060_e10741_d_n11, assign7060_e10741_d_n13, assign7060_e10741_d_n14,) = {
    if (locals.var_guard68 == 0.0) {
        let assign7060_e10734: f64 = (-locals.var_leff_1);
        let assign7060_e10736: f64 = (assign7060_e10734 / p.p910);
        let assign7060_e10737: f64 = { let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7060_e10738: f64 = (p.p909 * assign7060_e10737);
        let assign7060_e10739: f64 = (locals.var_rdsw_i + assign7060_e10738);
        (assign7060_e10739, (locals.var_rdsw_i_dn0 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p910)))), (locals.var_rdsw_i_dn2 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p910)))), (locals.var_rdsw_i_dn3 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p910)))), (locals.var_rdsw_i_dn4 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p910)))), (locals.var_rdsw_i_dn5 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p910)))), (locals.var_rdsw_i_dn6 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p910)))), (locals.var_rdsw_i_dn7 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p910)))), (locals.var_rdsw_i_dn8 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p910)))), (locals.var_rdsw_i_dn9 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p910)))), (locals.var_rdsw_i_dn10 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p910)))), (locals.var_rdsw_i_dn11 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p910)))), (locals.var_rdsw_i_dn13 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p910)))), (locals.var_rdsw_i_dn14 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p910)))),)
    } else {
        (locals.var_rdsw_i, locals.var_rdsw_i_dn0, locals.var_rdsw_i_dn2, locals.var_rdsw_i_dn3, locals.var_rdsw_i_dn4, locals.var_rdsw_i_dn5, locals.var_rdsw_i_dn6, locals.var_rdsw_i_dn7, locals.var_rdsw_i_dn8, locals.var_rdsw_i_dn9, locals.var_rdsw_i_dn10, locals.var_rdsw_i_dn11, locals.var_rdsw_i_dn13, locals.var_rdsw_i_dn14,)
    }
};
        locals.var_rdsw_i = assign7060_e10741;
        locals.var_rdsw_i_dn0 = assign7060_e10741_d_n0;
        locals.var_rdsw_i_dn2 = assign7060_e10741_d_n2;
        locals.var_rdsw_i_dn3 = assign7060_e10741_d_n3;
        locals.var_rdsw_i_dn4 = assign7060_e10741_d_n4;
        locals.var_rdsw_i_dn5 = assign7060_e10741_d_n5;
        locals.var_rdsw_i_dn6 = assign7060_e10741_d_n6;
        locals.var_rdsw_i_dn7 = assign7060_e10741_d_n7;
        locals.var_rdsw_i_dn8 = assign7060_e10741_d_n8;
        locals.var_rdsw_i_dn9 = assign7060_e10741_d_n9;
        locals.var_rdsw_i_dn10 = assign7060_e10741_d_n10;
        locals.var_rdsw_i_dn11 = assign7060_e10741_d_n11;
        locals.var_rdsw_i_dn13 = assign7060_e10741_d_n13;
        locals.var_rdsw_i_dn14 = assign7060_e10741_d_n14;
        locals.var_rdsw_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign7070_e10745: f64 = (-locals.var_leff_1);
        let assign7070_e10747: f64 = (assign7070_e10745 / p.p1023);
        let assign7070_e10748: f64 = { let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7070_e10749: f64 = (p.p1021 * assign7070_e10748);
        let assign7070_e10750: f64 = (locals.var_pclm_i + assign7070_e10749);
        locals.var_pclm_i = assign7070_e10750;
        locals.var_pclm_i_dn0 = (locals.var_pclm_i_dn0 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p1023))));
        locals.var_pclm_i_dn2 = (locals.var_pclm_i_dn2 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p1023))));
        locals.var_pclm_i_dn3 = (locals.var_pclm_i_dn3 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p1023))));
        locals.var_pclm_i_dn4 = (locals.var_pclm_i_dn4 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p1023))));
        locals.var_pclm_i_dn5 = (locals.var_pclm_i_dn5 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p1023))));
        locals.var_pclm_i_dn6 = (locals.var_pclm_i_dn6 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p1023))));
        locals.var_pclm_i_dn7 = (locals.var_pclm_i_dn7 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p1023))));
        locals.var_pclm_i_dn8 = (locals.var_pclm_i_dn8 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p1023))));
        locals.var_pclm_i_dn9 = (locals.var_pclm_i_dn9 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p1023))));
        locals.var_pclm_i_dn10 = (locals.var_pclm_i_dn10 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p1023))));
        locals.var_pclm_i_dn11 = (locals.var_pclm_i_dn11 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p1023))));
        locals.var_pclm_i_dn13 = (locals.var_pclm_i_dn13 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p1023))));
        locals.var_pclm_i_dn14 = (locals.var_pclm_i_dn14 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p1023))));
        locals.var_pclm_i_rv = 0.0;

        let assign7080_e10753: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign7080_e10753;
        locals.var_guard69_rv = 0.0;

        let (assign7090_e10765, assign7090_e10765_d_n0, assign7090_e10765_d_n2, assign7090_e10765_d_n3, assign7090_e10765_d_n4, assign7090_e10765_d_n5, assign7090_e10765_d_n6, assign7090_e10765_d_n7, assign7090_e10765_d_n8, assign7090_e10765_d_n9, assign7090_e10765_d_n10, assign7090_e10765_d_n11, assign7090_e10765_d_n13, assign7090_e10765_d_n14,) = {
    if (locals.var_guard69 != 0.0) {
        let assign7090_e10758: f64 = (-p.p1024);
        let assign7090_e10760: f64 = (assign7090_e10758 * locals.var_leff_ln);
        let assign7090_e10761: f64 = (assign7090_e10760).exp();
        let assign7090_e10762: f64 = (p.p1022 * assign7090_e10761);
        let assign7090_e10763: f64 = (locals.var_pclmr_i + assign7090_e10762);
        (assign7090_e10763, (locals.var_pclmr_i_dn0 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn0)))), (locals.var_pclmr_i_dn2 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn2)))), (locals.var_pclmr_i_dn3 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn3)))), (locals.var_pclmr_i_dn4 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn4)))), (locals.var_pclmr_i_dn5 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn5)))), (locals.var_pclmr_i_dn6 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn6)))), (locals.var_pclmr_i_dn7 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn7)))), (locals.var_pclmr_i_dn8 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn8)))), (locals.var_pclmr_i_dn9 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn9)))), (locals.var_pclmr_i_dn10 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn10)))), (locals.var_pclmr_i_dn11 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn11)))), (locals.var_pclmr_i_dn13 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn13)))), (locals.var_pclmr_i_dn14 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * locals.var_leff_ln_dn14)))),)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn0, locals.var_pclmr_i_dn2, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11, locals.var_pclmr_i_dn13, locals.var_pclmr_i_dn14,)
    }
};
        locals.var_pclmr_i = assign7090_e10765;
        locals.var_pclmr_i_dn0 = assign7090_e10765_d_n0;
        locals.var_pclmr_i_dn2 = assign7090_e10765_d_n2;
        locals.var_pclmr_i_dn3 = assign7090_e10765_d_n3;
        locals.var_pclmr_i_dn4 = assign7090_e10765_d_n4;
        locals.var_pclmr_i_dn5 = assign7090_e10765_d_n5;
        locals.var_pclmr_i_dn6 = assign7090_e10765_d_n6;
        locals.var_pclmr_i_dn7 = assign7090_e10765_d_n7;
        locals.var_pclmr_i_dn8 = assign7090_e10765_d_n8;
        locals.var_pclmr_i_dn9 = assign7090_e10765_d_n9;
        locals.var_pclmr_i_dn10 = assign7090_e10765_d_n10;
        locals.var_pclmr_i_dn11 = assign7090_e10765_d_n11;
        locals.var_pclmr_i_dn13 = assign7090_e10765_d_n13;
        locals.var_pclmr_i_dn14 = assign7090_e10765_d_n14;
        locals.var_pclmr_i_rv = 0.0;

        let assign7100_e10769: f64 = (-p.p445);
        let assign7100_e10771: f64 = (assign7100_e10769 * locals.var_leff_ln);
        let assign7100_e10772: f64 = (assign7100_e10771).exp();
        let assign7100_e10773: f64 = (p.p444 * assign7100_e10772);
        let assign7100_e10774: f64 = (locals.var_mexp_i + assign7100_e10773);
        locals.var_mexp_i = assign7100_e10774;
        locals.var_mexp_i_dn0 = (locals.var_mexp_i_dn0 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn0))));
        locals.var_mexp_i_dn2 = (locals.var_mexp_i_dn2 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn2))));
        locals.var_mexp_i_dn3 = (locals.var_mexp_i_dn3 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn3))));
        locals.var_mexp_i_dn4 = (locals.var_mexp_i_dn4 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn4))));
        locals.var_mexp_i_dn5 = (locals.var_mexp_i_dn5 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn5))));
        locals.var_mexp_i_dn6 = (locals.var_mexp_i_dn6 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn6))));
        locals.var_mexp_i_dn7 = (locals.var_mexp_i_dn7 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn7))));
        locals.var_mexp_i_dn8 = (locals.var_mexp_i_dn8 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn8))));
        locals.var_mexp_i_dn9 = (locals.var_mexp_i_dn9 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn9))));
        locals.var_mexp_i_dn10 = (locals.var_mexp_i_dn10 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn10))));
        locals.var_mexp_i_dn11 = (locals.var_mexp_i_dn11 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn11))));
        locals.var_mexp_i_dn13 = (locals.var_mexp_i_dn13 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn13))));
        locals.var_mexp_i_dn14 = (locals.var_mexp_i_dn14 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * locals.var_leff_ln_dn14))));
        locals.var_mexp_i_rv = 0.0;

        let assign7110_e10777: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign7110_e10777;
        locals.var_guard70_rv = 0.0;

        let (assign7120_e10789, assign7120_e10789_d_n0, assign7120_e10789_d_n2, assign7120_e10789_d_n3, assign7120_e10789_d_n4, assign7120_e10789_d_n5, assign7120_e10789_d_n6, assign7120_e10789_d_n7, assign7120_e10789_d_n8, assign7120_e10789_d_n9, assign7120_e10789_d_n10, assign7120_e10789_d_n11, assign7120_e10789_d_n13, assign7120_e10789_d_n14,) = {
    if (locals.var_guard70 != 0.0) {
        let assign7120_e10782: f64 = (-p.p447);
        let assign7120_e10784: f64 = (assign7120_e10782 * locals.var_leff_ln);
        let assign7120_e10785: f64 = (assign7120_e10784).exp();
        let assign7120_e10786: f64 = (p.p446 * assign7120_e10785);
        let assign7120_e10787: f64 = (locals.var_mexpr_i + assign7120_e10786);
        (assign7120_e10787, (locals.var_mexpr_i_dn0 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn0)))), (locals.var_mexpr_i_dn2 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn2)))), (locals.var_mexpr_i_dn3 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn3)))), (locals.var_mexpr_i_dn4 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn4)))), (locals.var_mexpr_i_dn5 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn5)))), (locals.var_mexpr_i_dn6 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn6)))), (locals.var_mexpr_i_dn7 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn7)))), (locals.var_mexpr_i_dn8 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn8)))), (locals.var_mexpr_i_dn9 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn9)))), (locals.var_mexpr_i_dn10 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn10)))), (locals.var_mexpr_i_dn11 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn11)))), (locals.var_mexpr_i_dn13 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn13)))), (locals.var_mexpr_i_dn14 + (p.p446 * (assign7120_e10785 * (assign7120_e10782 * locals.var_leff_ln_dn14)))),)
    } else {
        (locals.var_mexpr_i, locals.var_mexpr_i_dn0, locals.var_mexpr_i_dn2, locals.var_mexpr_i_dn3, locals.var_mexpr_i_dn4, locals.var_mexpr_i_dn5, locals.var_mexpr_i_dn6, locals.var_mexpr_i_dn7, locals.var_mexpr_i_dn8, locals.var_mexpr_i_dn9, locals.var_mexpr_i_dn10, locals.var_mexpr_i_dn11, locals.var_mexpr_i_dn13, locals.var_mexpr_i_dn14,)
    }
};
        locals.var_mexpr_i = assign7120_e10789;
        locals.var_mexpr_i_dn0 = assign7120_e10789_d_n0;
        locals.var_mexpr_i_dn2 = assign7120_e10789_d_n2;
        locals.var_mexpr_i_dn3 = assign7120_e10789_d_n3;
        locals.var_mexpr_i_dn4 = assign7120_e10789_d_n4;
        locals.var_mexpr_i_dn5 = assign7120_e10789_d_n5;
        locals.var_mexpr_i_dn6 = assign7120_e10789_d_n6;
        locals.var_mexpr_i_dn7 = assign7120_e10789_d_n7;
        locals.var_mexpr_i_dn8 = assign7120_e10789_d_n8;
        locals.var_mexpr_i_dn9 = assign7120_e10789_d_n9;
        locals.var_mexpr_i_dn10 = assign7120_e10789_d_n10;
        locals.var_mexpr_i_dn11 = assign7120_e10789_d_n11;
        locals.var_mexpr_i_dn13 = assign7120_e10789_d_n13;
        locals.var_mexpr_i_dn14 = assign7120_e10789_d_n14;
        locals.var_mexpr_i_rv = 0.0;

        let assign7130_e10793: f64 = (-locals.var_leff_1);
        let assign7130_e10795: f64 = (assign7130_e10793 / p.p449);
        let assign7130_e10796: f64 = { let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7130_e10797: f64 = (p.p448 * assign7130_e10796);
        let assign7130_e10798: f64 = (locals.var_ptwg_i + assign7130_e10797);
        locals.var_ptwg_i = assign7130_e10798;
        locals.var_ptwg_i_dn0 = (locals.var_ptwg_i_dn0 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p449))));
        locals.var_ptwg_i_dn2 = (locals.var_ptwg_i_dn2 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p449))));
        locals.var_ptwg_i_dn3 = (locals.var_ptwg_i_dn3 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p449))));
        locals.var_ptwg_i_dn4 = (locals.var_ptwg_i_dn4 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p449))));
        locals.var_ptwg_i_dn5 = (locals.var_ptwg_i_dn5 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p449))));
        locals.var_ptwg_i_dn6 = (locals.var_ptwg_i_dn6 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p449))));
        locals.var_ptwg_i_dn7 = (locals.var_ptwg_i_dn7 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p449))));
        locals.var_ptwg_i_dn8 = (locals.var_ptwg_i_dn8 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p449))));
        locals.var_ptwg_i_dn9 = (locals.var_ptwg_i_dn9 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p449))));
        locals.var_ptwg_i_dn10 = (locals.var_ptwg_i_dn10 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p449))));
        locals.var_ptwg_i_dn11 = (locals.var_ptwg_i_dn11 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p449))));
        locals.var_ptwg_i_dn13 = (locals.var_ptwg_i_dn13 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p449))));
        locals.var_ptwg_i_dn14 = (locals.var_ptwg_i_dn14 + (p.p448 * ({ let limited_exp_arg = assign7130_e10795; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p449))));
        locals.var_ptwg_i_rv = 0.0;

        let assign7140_e10801: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign7140_e10801;
        locals.var_guard71_rv = 0.0;

        let (assign7150_e10813, assign7150_e10813_d_n0, assign7150_e10813_d_n2, assign7150_e10813_d_n3, assign7150_e10813_d_n4, assign7150_e10813_d_n5, assign7150_e10813_d_n6, assign7150_e10813_d_n7, assign7150_e10813_d_n8, assign7150_e10813_d_n9, assign7150_e10813_d_n10, assign7150_e10813_d_n11, assign7150_e10813_d_n13, assign7150_e10813_d_n14,) = {
    if (locals.var_guard71 != 0.0) {
        let assign7150_e10806: f64 = (-locals.var_leff_1);
        let assign7150_e10808: f64 = (assign7150_e10806 / p.p449);
        let assign7150_e10809: f64 = { let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7150_e10810: f64 = (p.p448 * assign7150_e10809);
        let assign7150_e10811: f64 = (locals.var_ptwgr_i + assign7150_e10810);
        (assign7150_e10811, (locals.var_ptwgr_i_dn0 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p449)))), (locals.var_ptwgr_i_dn2 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p449)))), (locals.var_ptwgr_i_dn3 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p449)))), (locals.var_ptwgr_i_dn4 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p449)))), (locals.var_ptwgr_i_dn5 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p449)))), (locals.var_ptwgr_i_dn6 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p449)))), (locals.var_ptwgr_i_dn7 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p449)))), (locals.var_ptwgr_i_dn8 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p449)))), (locals.var_ptwgr_i_dn9 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p449)))), (locals.var_ptwgr_i_dn10 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p449)))), (locals.var_ptwgr_i_dn11 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p449)))), (locals.var_ptwgr_i_dn13 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p449)))), (locals.var_ptwgr_i_dn14 + (p.p448 * ({ let limited_exp_arg = assign7150_e10808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p449)))),)
    } else {
        (locals.var_ptwgr_i, locals.var_ptwgr_i_dn0, locals.var_ptwgr_i_dn2, locals.var_ptwgr_i_dn3, locals.var_ptwgr_i_dn4, locals.var_ptwgr_i_dn5, locals.var_ptwgr_i_dn6, locals.var_ptwgr_i_dn7, locals.var_ptwgr_i_dn8, locals.var_ptwgr_i_dn9, locals.var_ptwgr_i_dn10, locals.var_ptwgr_i_dn11, locals.var_ptwgr_i_dn13, locals.var_ptwgr_i_dn14,)
    }
};
        locals.var_ptwgr_i = assign7150_e10813;
        locals.var_ptwgr_i_dn0 = assign7150_e10813_d_n0;
        locals.var_ptwgr_i_dn2 = assign7150_e10813_d_n2;
        locals.var_ptwgr_i_dn3 = assign7150_e10813_d_n3;
        locals.var_ptwgr_i_dn4 = assign7150_e10813_d_n4;
        locals.var_ptwgr_i_dn5 = assign7150_e10813_d_n5;
        locals.var_ptwgr_i_dn6 = assign7150_e10813_d_n6;
        locals.var_ptwgr_i_dn7 = assign7150_e10813_d_n7;
        locals.var_ptwgr_i_dn8 = assign7150_e10813_d_n8;
        locals.var_ptwgr_i_dn9 = assign7150_e10813_d_n9;
        locals.var_ptwgr_i_dn10 = assign7150_e10813_d_n10;
        locals.var_ptwgr_i_dn11 = assign7150_e10813_d_n11;
        locals.var_ptwgr_i_dn13 = assign7150_e10813_d_n13;
        locals.var_ptwgr_i_dn14 = assign7150_e10813_d_n14;
        locals.var_ptwgr_i_rv = 0.0;

        let assign7160_e10817: f64 = (-locals.var_leff_1);
        let assign7160_e10819: f64 = (assign7160_e10817 / p.p431);
        let assign7160_e10820: f64 = { let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7160_e10821: f64 = (p.p430 * assign7160_e10820);
        let assign7160_e10822: f64 = (locals.var_vsat_i + assign7160_e10821);
        locals.var_vsat_i = assign7160_e10822;
        locals.var_vsat_i_dn0 = (locals.var_vsat_i_dn0 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p431))));
        locals.var_vsat_i_dn2 = (locals.var_vsat_i_dn2 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p431))));
        locals.var_vsat_i_dn3 = (locals.var_vsat_i_dn3 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p431))));
        locals.var_vsat_i_dn4 = (locals.var_vsat_i_dn4 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p431))));
        locals.var_vsat_i_dn5 = (locals.var_vsat_i_dn5 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p431))));
        locals.var_vsat_i_dn6 = (locals.var_vsat_i_dn6 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p431))));
        locals.var_vsat_i_dn7 = (locals.var_vsat_i_dn7 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p431))));
        locals.var_vsat_i_dn8 = (locals.var_vsat_i_dn8 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p431))));
        locals.var_vsat_i_dn9 = (locals.var_vsat_i_dn9 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p431))));
        locals.var_vsat_i_dn10 = (locals.var_vsat_i_dn10 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p431))));
        locals.var_vsat_i_dn11 = (locals.var_vsat_i_dn11 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p431))));
        locals.var_vsat_i_dn13 = (locals.var_vsat_i_dn13 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p431))));
        locals.var_vsat_i_dn14 = (locals.var_vsat_i_dn14 + (p.p430 * ({ let limited_exp_arg = assign7160_e10819; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p431))));
        locals.var_vsat_i_rv = 0.0;

        let assign7170_e10826: f64 = (-locals.var_leff_1);
        let assign7170_e10828: f64 = (assign7170_e10826 / p.p437);
        let assign7170_e10829: f64 = { let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7170_e10830: f64 = (p.p436 * assign7170_e10829);
        let assign7170_e10831: f64 = (locals.var_vsat1_i + assign7170_e10830);
        locals.var_vsat1_i = assign7170_e10831;
        locals.var_vsat1_i_dn0 = (locals.var_vsat1_i_dn0 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p437))));
        locals.var_vsat1_i_dn2 = (locals.var_vsat1_i_dn2 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p437))));
        locals.var_vsat1_i_dn3 = (locals.var_vsat1_i_dn3 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p437))));
        locals.var_vsat1_i_dn4 = (locals.var_vsat1_i_dn4 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p437))));
        locals.var_vsat1_i_dn5 = (locals.var_vsat1_i_dn5 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p437))));
        locals.var_vsat1_i_dn6 = (locals.var_vsat1_i_dn6 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p437))));
        locals.var_vsat1_i_dn7 = (locals.var_vsat1_i_dn7 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p437))));
        locals.var_vsat1_i_dn8 = (locals.var_vsat1_i_dn8 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p437))));
        locals.var_vsat1_i_dn9 = (locals.var_vsat1_i_dn9 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p437))));
        locals.var_vsat1_i_dn10 = (locals.var_vsat1_i_dn10 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p437))));
        locals.var_vsat1_i_dn11 = (locals.var_vsat1_i_dn11 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p437))));
        locals.var_vsat1_i_dn13 = (locals.var_vsat1_i_dn13 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p437))));
        locals.var_vsat1_i_dn14 = (locals.var_vsat1_i_dn14 + (p.p436 * ({ let limited_exp_arg = assign7170_e10828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p437))));
        locals.var_vsat1_i_rv = 0.0;

        let assign7180_e10834: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign7180_e10834;
        locals.var_guard72_rv = 0.0;

        let (assign7190_e10846, assign7190_e10846_d_n0, assign7190_e10846_d_n2, assign7190_e10846_d_n3, assign7190_e10846_d_n4, assign7190_e10846_d_n5, assign7190_e10846_d_n6, assign7190_e10846_d_n7, assign7190_e10846_d_n8, assign7190_e10846_d_n9, assign7190_e10846_d_n10, assign7190_e10846_d_n11, assign7190_e10846_d_n13, assign7190_e10846_d_n14,) = {
    if (locals.var_guard72 != 0.0) {
        let assign7190_e10839: f64 = (-locals.var_leff_1);
        let assign7190_e10841: f64 = (assign7190_e10839 / p.p437);
        let assign7190_e10842: f64 = { let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7190_e10843: f64 = (p.p436 * assign7190_e10842);
        let assign7190_e10844: f64 = (locals.var_vsat1r_i + assign7190_e10843);
        (assign7190_e10844, (locals.var_vsat1r_i_dn0 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p437)))), (locals.var_vsat1r_i_dn2 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p437)))), (locals.var_vsat1r_i_dn3 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p437)))), (locals.var_vsat1r_i_dn4 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p437)))), (locals.var_vsat1r_i_dn5 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p437)))), (locals.var_vsat1r_i_dn6 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p437)))), (locals.var_vsat1r_i_dn7 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p437)))), (locals.var_vsat1r_i_dn8 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p437)))), (locals.var_vsat1r_i_dn9 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p437)))), (locals.var_vsat1r_i_dn10 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p437)))), (locals.var_vsat1r_i_dn11 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p437)))), (locals.var_vsat1r_i_dn13 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p437)))), (locals.var_vsat1r_i_dn14 + (p.p436 * ({ let limited_exp_arg = assign7190_e10841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p437)))),)
    } else {
        (locals.var_vsat1r_i, locals.var_vsat1r_i_dn0, locals.var_vsat1r_i_dn2, locals.var_vsat1r_i_dn3, locals.var_vsat1r_i_dn4, locals.var_vsat1r_i_dn5, locals.var_vsat1r_i_dn6, locals.var_vsat1r_i_dn7, locals.var_vsat1r_i_dn8, locals.var_vsat1r_i_dn9, locals.var_vsat1r_i_dn10, locals.var_vsat1r_i_dn11, locals.var_vsat1r_i_dn13, locals.var_vsat1r_i_dn14,)
    }
};
        locals.var_vsat1r_i = assign7190_e10846;
        locals.var_vsat1r_i_dn0 = assign7190_e10846_d_n0;
        locals.var_vsat1r_i_dn2 = assign7190_e10846_d_n2;
        locals.var_vsat1r_i_dn3 = assign7190_e10846_d_n3;
        locals.var_vsat1r_i_dn4 = assign7190_e10846_d_n4;
        locals.var_vsat1r_i_dn5 = assign7190_e10846_d_n5;
        locals.var_vsat1r_i_dn6 = assign7190_e10846_d_n6;
        locals.var_vsat1r_i_dn7 = assign7190_e10846_d_n7;
        locals.var_vsat1r_i_dn8 = assign7190_e10846_d_n8;
        locals.var_vsat1r_i_dn9 = assign7190_e10846_d_n9;
        locals.var_vsat1r_i_dn10 = assign7190_e10846_d_n10;
        locals.var_vsat1r_i_dn11 = assign7190_e10846_d_n11;
        locals.var_vsat1r_i_dn13 = assign7190_e10846_d_n13;
        locals.var_vsat1r_i_dn14 = assign7190_e10846_d_n14;
        locals.var_vsat1r_i_rv = 0.0;

        let assign7200_e10850: f64 = (-locals.var_leff_1);
        let assign7200_e10852: f64 = (assign7200_e10850 / p.p439);
        let assign7200_e10853: f64 = { let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7200_e10854: f64 = (p.p438 * assign7200_e10853);
        let assign7200_e10855: f64 = (locals.var_psat_i + assign7200_e10854);
        locals.var_psat_i = assign7200_e10855;
        locals.var_psat_i_dn0 = (locals.var_psat_i_dn0 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p439))));
        locals.var_psat_i_dn2 = (locals.var_psat_i_dn2 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p439))));
        locals.var_psat_i_dn3 = (locals.var_psat_i_dn3 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p439))));
        locals.var_psat_i_dn4 = (locals.var_psat_i_dn4 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p439))));
        locals.var_psat_i_dn5 = (locals.var_psat_i_dn5 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p439))));
        locals.var_psat_i_dn6 = (locals.var_psat_i_dn6 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p439))));
        locals.var_psat_i_dn7 = (locals.var_psat_i_dn7 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p439))));
        locals.var_psat_i_dn8 = (locals.var_psat_i_dn8 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p439))));
        locals.var_psat_i_dn9 = (locals.var_psat_i_dn9 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p439))));
        locals.var_psat_i_dn10 = (locals.var_psat_i_dn10 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p439))));
        locals.var_psat_i_dn11 = (locals.var_psat_i_dn11 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p439))));
        locals.var_psat_i_dn13 = (locals.var_psat_i_dn13 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p439))));
        locals.var_psat_i_dn14 = (locals.var_psat_i_dn14 + (p.p438 * ({ let limited_exp_arg = assign7200_e10852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p439))));
        locals.var_psat_i_rv = 0.0;

        let assign7210_e10859: f64 = (-locals.var_leffcv_1);
        let assign7210_e10861: f64 = (assign7210_e10859 / p.p443);
        let assign7210_e10862: f64 = { let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7210_e10863: f64 = (p.p442 * assign7210_e10862);
        let assign7210_e10864: f64 = (locals.var_psatcv_i + assign7210_e10863);
        locals.var_psatcv_i = assign7210_e10864;
        locals.var_psatcv_i_dn0 = (locals.var_psatcv_i_dn0 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn0) / p.p443))));
        locals.var_psatcv_i_dn2 = (locals.var_psatcv_i_dn2 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn2) / p.p443))));
        locals.var_psatcv_i_dn3 = (locals.var_psatcv_i_dn3 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn3) / p.p443))));
        locals.var_psatcv_i_dn4 = (locals.var_psatcv_i_dn4 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn4) / p.p443))));
        locals.var_psatcv_i_dn5 = (locals.var_psatcv_i_dn5 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn5) / p.p443))));
        locals.var_psatcv_i_dn6 = (locals.var_psatcv_i_dn6 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn6) / p.p443))));
        locals.var_psatcv_i_dn7 = (locals.var_psatcv_i_dn7 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn7) / p.p443))));
        locals.var_psatcv_i_dn8 = (locals.var_psatcv_i_dn8 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn8) / p.p443))));
        locals.var_psatcv_i_dn9 = (locals.var_psatcv_i_dn9 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn9) / p.p443))));
        locals.var_psatcv_i_dn10 = (locals.var_psatcv_i_dn10 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn10) / p.p443))));
        locals.var_psatcv_i_dn11 = (locals.var_psatcv_i_dn11 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn11) / p.p443))));
        locals.var_psatcv_i_dn13 = (locals.var_psatcv_i_dn13 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn13) / p.p443))));
        locals.var_psatcv_i_dn14 = (locals.var_psatcv_i_dn14 + (p.p442 * ({ let limited_exp_arg = assign7210_e10861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn14) / p.p443))));
        locals.var_psatcv_i_rv = 0.0;

        let assign7220_e10868: f64 = (-locals.var_leffcv_1);
        let assign7220_e10870: f64 = (assign7220_e10868 / p.p441);
        let assign7220_e10871: f64 = { let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7220_e10872: f64 = (p.p440 * assign7220_e10871);
        let assign7220_e10873: f64 = (locals.var_vsatcv_i + assign7220_e10872);
        locals.var_vsatcv_i = assign7220_e10873;
        locals.var_vsatcv_i_dn0 = (locals.var_vsatcv_i_dn0 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn0) / p.p441))));
        locals.var_vsatcv_i_dn2 = (locals.var_vsatcv_i_dn2 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn2) / p.p441))));
        locals.var_vsatcv_i_dn3 = (locals.var_vsatcv_i_dn3 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn3) / p.p441))));
        locals.var_vsatcv_i_dn4 = (locals.var_vsatcv_i_dn4 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn4) / p.p441))));
        locals.var_vsatcv_i_dn5 = (locals.var_vsatcv_i_dn5 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn5) / p.p441))));
        locals.var_vsatcv_i_dn6 = (locals.var_vsatcv_i_dn6 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn6) / p.p441))));
        locals.var_vsatcv_i_dn7 = (locals.var_vsatcv_i_dn7 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn7) / p.p441))));
        locals.var_vsatcv_i_dn8 = (locals.var_vsatcv_i_dn8 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn8) / p.p441))));
        locals.var_vsatcv_i_dn9 = (locals.var_vsatcv_i_dn9 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn9) / p.p441))));
        locals.var_vsatcv_i_dn10 = (locals.var_vsatcv_i_dn10 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn10) / p.p441))));
        locals.var_vsatcv_i_dn11 = (locals.var_vsatcv_i_dn11 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn11) / p.p441))));
        locals.var_vsatcv_i_dn13 = (locals.var_vsatcv_i_dn13 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn13) / p.p441))));
        locals.var_vsatcv_i_dn14 = (locals.var_vsatcv_i_dn14 + (p.p440 * ({ let limited_exp_arg = assign7220_e10870; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leffcv_1_dn14) / p.p441))));
        locals.var_vsatcv_i_rv = 0.0;

        let assign7230_e10877: f64 = (-locals.var_leff_1);
        let assign7230_e10879: f64 = (assign7230_e10877 / p.p168);
        let assign7230_e10880: f64 = { let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7230_e10881: f64 = (p.p167 * assign7230_e10880);
        let assign7230_e10882: f64 = (locals.var_dvtp0_i + assign7230_e10881);
        locals.var_dvtp0_i = assign7230_e10882;
        locals.var_dvtp0_i_dn0 = (locals.var_dvtp0_i_dn0 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p168))));
        locals.var_dvtp0_i_dn2 = (locals.var_dvtp0_i_dn2 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p168))));
        locals.var_dvtp0_i_dn3 = (locals.var_dvtp0_i_dn3 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p168))));
        locals.var_dvtp0_i_dn4 = (locals.var_dvtp0_i_dn4 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p168))));
        locals.var_dvtp0_i_dn5 = (locals.var_dvtp0_i_dn5 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p168))));
        locals.var_dvtp0_i_dn6 = (locals.var_dvtp0_i_dn6 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p168))));
        locals.var_dvtp0_i_dn7 = (locals.var_dvtp0_i_dn7 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p168))));
        locals.var_dvtp0_i_dn8 = (locals.var_dvtp0_i_dn8 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p168))));
        locals.var_dvtp0_i_dn9 = (locals.var_dvtp0_i_dn9 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p168))));
        locals.var_dvtp0_i_dn10 = (locals.var_dvtp0_i_dn10 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p168))));
        locals.var_dvtp0_i_dn11 = (locals.var_dvtp0_i_dn11 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p168))));
        locals.var_dvtp0_i_dn13 = (locals.var_dvtp0_i_dn13 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p168))));
        locals.var_dvtp0_i_dn14 = (locals.var_dvtp0_i_dn14 + (p.p167 * ({ let limited_exp_arg = assign7230_e10879; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p168))));
        locals.var_dvtp0_i_rv = 0.0;

        let assign7240_e10886: f64 = (-locals.var_leff_1);
        let assign7240_e10888: f64 = (assign7240_e10886 / p.p170);
        let assign7240_e10889: f64 = { let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7240_e10890: f64 = (p.p169 * assign7240_e10889);
        let assign7240_e10891: f64 = (locals.var_dvtp1_i + assign7240_e10890);
        locals.var_dvtp1_i = assign7240_e10891;
        locals.var_dvtp1_i_dn0 = (locals.var_dvtp1_i_dn0 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn0) / p.p170))));
        locals.var_dvtp1_i_dn2 = (locals.var_dvtp1_i_dn2 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn2) / p.p170))));
        locals.var_dvtp1_i_dn3 = (locals.var_dvtp1_i_dn3 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn3) / p.p170))));
        locals.var_dvtp1_i_dn4 = (locals.var_dvtp1_i_dn4 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn4) / p.p170))));
        locals.var_dvtp1_i_dn5 = (locals.var_dvtp1_i_dn5 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn5) / p.p170))));
        locals.var_dvtp1_i_dn6 = (locals.var_dvtp1_i_dn6 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn6) / p.p170))));
        locals.var_dvtp1_i_dn7 = (locals.var_dvtp1_i_dn7 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn7) / p.p170))));
        locals.var_dvtp1_i_dn8 = (locals.var_dvtp1_i_dn8 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn8) / p.p170))));
        locals.var_dvtp1_i_dn9 = (locals.var_dvtp1_i_dn9 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn9) / p.p170))));
        locals.var_dvtp1_i_dn10 = (locals.var_dvtp1_i_dn10 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn10) / p.p170))));
        locals.var_dvtp1_i_dn11 = (locals.var_dvtp1_i_dn11 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn11) / p.p170))));
        locals.var_dvtp1_i_dn13 = (locals.var_dvtp1_i_dn13 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn13) / p.p170))));
        locals.var_dvtp1_i_dn14 = (locals.var_dvtp1_i_dn14 + (p.p169 * ({ let limited_exp_arg = assign7240_e10888; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_leff_1_dn14) / p.p170))));
        locals.var_dvtp1_i_rv = 0.0;

        let assign7250_e10898: f64 = if ((locals.var_qmtcencv_i > 0.0) || (locals.var_qmtcencva_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard73 = assign7250_e10898;
        locals.var_guard73_rv = 0.0;

        let (assign7260_e10914,) = {
    if (locals.var_guard73 != 0.0) {
        let assign7260_e10904: f64 = (2.0 * locals.var_ach);
        let assign7260_e10906: f64 = (assign7260_e10904 / locals.var_weff_ufcm);
        let assign7260_e10907: f64 = (-assign7260_e10906);
        let assign7260_e10909: f64 = (assign7260_e10907 / p.p399);
        let assign7260_e10910: f64 = { let limited_exp_arg = assign7260_e10909; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7260_e10911: f64 = (p.p398 * assign7260_e10910);
        let assign7260_e10912: f64 = (1.0 + assign7260_e10911);
        (assign7260_e10912,)
    } else {
        (locals.var_mtcen,)
    }
};
        locals.var_mtcen = assign7260_e10914;
        locals.var_mtcen_rv = 0.0;

        let (assign7270_e10924,) = {
    if (locals.var_guard73 != 0.0) {
        let assign7270_e10918: f64 = (2.0 * locals.var_ach);
        let assign7270_e10920: f64 = (assign7270_e10918 / locals.var_weff_ufcm);
        let assign7270_e10922: f64 = (assign7270_e10920 * locals.var_mtcen);
        (assign7270_e10922,)
    } else {
        (locals.var_tcen0,)
    }
};
        locals.var_tcen0 = assign7270_e10924;
        locals.var_tcen0_rv = 0.0;

        let assign7300_e10933: f64 = if locals.var_qsref_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign7300_e10933;
        locals.var_guard76_rv = 0.0;

        let (assign7310_e10937,) = {
    if (locals.var_guard76 != 0.0) {
        (0.05,)
    } else {
        (locals.var_qsref_i,)
    }
};
        locals.var_qsref_i = assign7310_e10937;
        locals.var_qsref_i_rv = 0.0;

        let assign7380_e10970: f64 = if locals.var_phig_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign7380_e10970;
        locals.var_guard81_rv = 0.0;

        let (assign7390_e10974, assign7390_e10974_d_n0, assign7390_e10974_d_n2, assign7390_e10974_d_n3, assign7390_e10974_d_n4, assign7390_e10974_d_n5, assign7390_e10974_d_n6, assign7390_e10974_d_n7, assign7390_e10974_d_n8, assign7390_e10974_d_n9, assign7390_e10974_d_n10, assign7390_e10974_d_n11, assign7390_e10974_d_n13, assign7390_e10974_d_n14,) = {
    if (locals.var_guard81 != 0.0) {
        (4.61, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phig_i, locals.var_phig_i_dn0, locals.var_phig_i_dn2, locals.var_phig_i_dn3, locals.var_phig_i_dn4, locals.var_phig_i_dn5, locals.var_phig_i_dn6, locals.var_phig_i_dn7, locals.var_phig_i_dn8, locals.var_phig_i_dn9, locals.var_phig_i_dn10, locals.var_phig_i_dn11, locals.var_phig_i_dn13, locals.var_phig_i_dn14,)
    }
};
        locals.var_phig_i = assign7390_e10974;
        locals.var_phig_i_dn0 = assign7390_e10974_d_n0;
        locals.var_phig_i_dn2 = assign7390_e10974_d_n2;
        locals.var_phig_i_dn3 = assign7390_e10974_d_n3;
        locals.var_phig_i_dn4 = assign7390_e10974_d_n4;
        locals.var_phig_i_dn5 = assign7390_e10974_d_n5;
        locals.var_phig_i_dn6 = assign7390_e10974_d_n6;
        locals.var_phig_i_dn7 = assign7390_e10974_d_n7;
        locals.var_phig_i_dn8 = assign7390_e10974_d_n8;
        locals.var_phig_i_dn9 = assign7390_e10974_d_n9;
        locals.var_phig_i_dn10 = assign7390_e10974_d_n10;
        locals.var_phig_i_dn11 = assign7390_e10974_d_n11;
        locals.var_phig_i_dn13 = assign7390_e10974_d_n13;
        locals.var_phig_i_dn14 = assign7390_e10974_d_n14;
        locals.var_phig_i_rv = 0.0;

        let assign7400_e10977: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard82 = assign7400_e10977;
        locals.var_guard82_rv = 0.0;

        let assign7410_e10980: f64 = if locals.var_k1_i < 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign7410_e10980;
        locals.var_guard83_rv = 0.0;

        let (assign7420_e10986,) = {
    if ((locals.var_guard82 != 0.0) && (locals.var_guard83 != 0.0)) {
        (1e-6,)
    } else {
        (locals.var_k1_i,)
    }
};
        locals.var_k1_i = assign7420_e10986;
        locals.var_k1_i_rv = 0.0;

        let assign7430_e10989: f64 = if locals.var_sprt_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign7430_e10989;
        locals.var_guard84_rv = 0.0;

        let (assign7440_e10993,) = {
    if (locals.var_guard84 != 0.0) {
        (0.01,)
    } else {
        (locals.var_sprt_i,)
    }
};
        locals.var_sprt_i = assign7440_e10993;
        locals.var_sprt_i_rv = 0.0;

        let assign7450_e10996: f64 = if locals.var_qsref_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign7450_e10996;
        locals.var_guard85_rv = 0.0;

        let (assign7460_e11000,) = {
    if (locals.var_guard85 != 0.0) {
        (0.05,)
    } else {
        (locals.var_qsref_i,)
    }
};
        locals.var_qsref_i = assign7460_e11000;
        locals.var_qsref_i_rv = 0.0;

        let assign7470_e11003: f64 = if locals.var_noia2_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign7470_e11003;
        locals.var_guard86_rv = 0.0;

        let (assign7480_e11007,) = {
    if (locals.var_guard86 != 0.0) {
        (p.p1682,)
    } else {
        (locals.var_noia2_i,)
    }
};
        locals.var_noia2_i = assign7480_e11007;
        locals.var_noia2_i_rv = 0.0;

        let assign7490_e11010: f64 = if locals.var_mpower_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign7490_e11010;
        locals.var_guard87_rv = 0.0;

        let (assign7500_e11014,) = {
    if (locals.var_guard87 != 0.0) {
        (1.2,)
    } else {
        (locals.var_mpower_i,)
    }
};
        locals.var_mpower_i = assign7500_e11014;
        locals.var_mpower_i_rv = 0.0;

        let assign7510_e11017: f64 = if locals.var_covs_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign7510_e11017;
        locals.var_guard88_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7520_e11021, assign7520_e11021_d_n0, assign7520_e11021_d_n2, assign7520_e11021_d_n3, assign7520_e11021_d_n4, assign7520_e11021_d_n5, assign7520_e11021_d_n6, assign7520_e11021_d_n7, assign7520_e11021_d_n8, assign7520_e11021_d_n9, assign7520_e11021_d_n10, assign7520_e11021_d_n11, assign7520_e11021_d_n13, assign7520_e11021_d_n14,) = {
    if (locals.var_guard88 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_covs_i, locals.var_covs_i_dn0, locals.var_covs_i_dn2, locals.var_covs_i_dn3, locals.var_covs_i_dn4, locals.var_covs_i_dn5, locals.var_covs_i_dn6, locals.var_covs_i_dn7, locals.var_covs_i_dn8, locals.var_covs_i_dn9, locals.var_covs_i_dn10, locals.var_covs_i_dn11, locals.var_covs_i_dn13, locals.var_covs_i_dn14,)
    }
};
        locals.var_covs_i = assign7520_e11021;
        locals.var_covs_i_dn0 = assign7520_e11021_d_n0;
        locals.var_covs_i_dn2 = assign7520_e11021_d_n2;
        locals.var_covs_i_dn3 = assign7520_e11021_d_n3;
        locals.var_covs_i_dn4 = assign7520_e11021_d_n4;
        locals.var_covs_i_dn5 = assign7520_e11021_d_n5;
        locals.var_covs_i_dn6 = assign7520_e11021_d_n6;
        locals.var_covs_i_dn7 = assign7520_e11021_d_n7;
        locals.var_covs_i_dn8 = assign7520_e11021_d_n8;
        locals.var_covs_i_dn9 = assign7520_e11021_d_n9;
        locals.var_covs_i_dn10 = assign7520_e11021_d_n10;
        locals.var_covs_i_dn11 = assign7520_e11021_d_n11;
        locals.var_covs_i_dn13 = assign7520_e11021_d_n13;
        locals.var_covs_i_dn14 = assign7520_e11021_d_n14;
        locals.var_covs_i_rv = 0.0;

        let assign7530_e11024: f64 = if locals.var_covd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign7530_e11024;
        locals.var_guard89_rv = 0.0;

        let (assign7540_e11028, assign7540_e11028_d_n0, assign7540_e11028_d_n2, assign7540_e11028_d_n3, assign7540_e11028_d_n4, assign7540_e11028_d_n5, assign7540_e11028_d_n6, assign7540_e11028_d_n7, assign7540_e11028_d_n8, assign7540_e11028_d_n9, assign7540_e11028_d_n10, assign7540_e11028_d_n11, assign7540_e11028_d_n13, assign7540_e11028_d_n14,) = {
    if (locals.var_guard89 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_covd_i, locals.var_covd_i_dn0, locals.var_covd_i_dn2, locals.var_covd_i_dn3, locals.var_covd_i_dn4, locals.var_covd_i_dn5, locals.var_covd_i_dn6, locals.var_covd_i_dn7, locals.var_covd_i_dn8, locals.var_covd_i_dn9, locals.var_covd_i_dn10, locals.var_covd_i_dn11, locals.var_covd_i_dn13, locals.var_covd_i_dn14,)
    }
};
        locals.var_covd_i = assign7540_e11028;
        locals.var_covd_i_dn0 = assign7540_e11028_d_n0;
        locals.var_covd_i_dn2 = assign7540_e11028_d_n2;
        locals.var_covd_i_dn3 = assign7540_e11028_d_n3;
        locals.var_covd_i_dn4 = assign7540_e11028_d_n4;
        locals.var_covd_i_dn5 = assign7540_e11028_d_n5;
        locals.var_covd_i_dn6 = assign7540_e11028_d_n6;
        locals.var_covd_i_dn7 = assign7540_e11028_d_n7;
        locals.var_covd_i_dn8 = assign7540_e11028_d_n8;
        locals.var_covd_i_dn9 = assign7540_e11028_d_n9;
        locals.var_covd_i_dn10 = assign7540_e11028_d_n10;
        locals.var_covd_i_dn11 = assign7540_e11028_d_n11;
        locals.var_covd_i_dn13 = assign7540_e11028_d_n13;
        locals.var_covd_i_dn14 = assign7540_e11028_d_n14;
        locals.var_covd_i_rv = 0.0;

        let assign7550_e11031: f64 = if locals.var_vsat_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign7550_e11031;
        locals.var_guard90_rv = 0.0;

        let (assign7560_e11035, assign7560_e11035_d_n0, assign7560_e11035_d_n2, assign7560_e11035_d_n3, assign7560_e11035_d_n4, assign7560_e11035_d_n5, assign7560_e11035_d_n6, assign7560_e11035_d_n7, assign7560_e11035_d_n8, assign7560_e11035_d_n9, assign7560_e11035_d_n10, assign7560_e11035_d_n11, assign7560_e11035_d_n13, assign7560_e11035_d_n14,) = {
    if (locals.var_guard90 != 0.0) {
        (85000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat_i, locals.var_vsat_i_dn0, locals.var_vsat_i_dn2, locals.var_vsat_i_dn3, locals.var_vsat_i_dn4, locals.var_vsat_i_dn5, locals.var_vsat_i_dn6, locals.var_vsat_i_dn7, locals.var_vsat_i_dn8, locals.var_vsat_i_dn9, locals.var_vsat_i_dn10, locals.var_vsat_i_dn11, locals.var_vsat_i_dn13, locals.var_vsat_i_dn14,)
    }
};
        locals.var_vsat_i = assign7560_e11035;
        locals.var_vsat_i_dn0 = assign7560_e11035_d_n0;
        locals.var_vsat_i_dn2 = assign7560_e11035_d_n2;
        locals.var_vsat_i_dn3 = assign7560_e11035_d_n3;
        locals.var_vsat_i_dn4 = assign7560_e11035_d_n4;
        locals.var_vsat_i_dn5 = assign7560_e11035_d_n5;
        locals.var_vsat_i_dn6 = assign7560_e11035_d_n6;
        locals.var_vsat_i_dn7 = assign7560_e11035_d_n7;
        locals.var_vsat_i_dn8 = assign7560_e11035_d_n8;
        locals.var_vsat_i_dn9 = assign7560_e11035_d_n9;
        locals.var_vsat_i_dn10 = assign7560_e11035_d_n10;
        locals.var_vsat_i_dn11 = assign7560_e11035_d_n11;
        locals.var_vsat_i_dn13 = assign7560_e11035_d_n13;
        locals.var_vsat_i_dn14 = assign7560_e11035_d_n14;
        locals.var_vsat_i_rv = 0.0;

        let assign7570_e11038: f64 = if locals.var_vsat1_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign7570_e11038;
        locals.var_guard91_rv = 0.0;

        let (assign7580_e11042, assign7580_e11042_d_n0, assign7580_e11042_d_n2, assign7580_e11042_d_n3, assign7580_e11042_d_n4, assign7580_e11042_d_n5, assign7580_e11042_d_n6, assign7580_e11042_d_n7, assign7580_e11042_d_n8, assign7580_e11042_d_n9, assign7580_e11042_d_n10, assign7580_e11042_d_n11, assign7580_e11042_d_n13, assign7580_e11042_d_n14,) = {
    if (locals.var_guard91 != 0.0) {
        (85000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat1_i, locals.var_vsat1_i_dn0, locals.var_vsat1_i_dn2, locals.var_vsat1_i_dn3, locals.var_vsat1_i_dn4, locals.var_vsat1_i_dn5, locals.var_vsat1_i_dn6, locals.var_vsat1_i_dn7, locals.var_vsat1_i_dn8, locals.var_vsat1_i_dn9, locals.var_vsat1_i_dn10, locals.var_vsat1_i_dn11, locals.var_vsat1_i_dn13, locals.var_vsat1_i_dn14,)
    }
};
        locals.var_vsat1_i = assign7580_e11042;
        locals.var_vsat1_i_dn0 = assign7580_e11042_d_n0;
        locals.var_vsat1_i_dn2 = assign7580_e11042_d_n2;
        locals.var_vsat1_i_dn3 = assign7580_e11042_d_n3;
        locals.var_vsat1_i_dn4 = assign7580_e11042_d_n4;
        locals.var_vsat1_i_dn5 = assign7580_e11042_d_n5;
        locals.var_vsat1_i_dn6 = assign7580_e11042_d_n6;
        locals.var_vsat1_i_dn7 = assign7580_e11042_d_n7;
        locals.var_vsat1_i_dn8 = assign7580_e11042_d_n8;
        locals.var_vsat1_i_dn9 = assign7580_e11042_d_n9;
        locals.var_vsat1_i_dn10 = assign7580_e11042_d_n10;
        locals.var_vsat1_i_dn11 = assign7580_e11042_d_n11;
        locals.var_vsat1_i_dn13 = assign7580_e11042_d_n13;
        locals.var_vsat1_i_dn14 = assign7580_e11042_d_n14;
        locals.var_vsat1_i_rv = 0.0;

        let assign7590_e11049: f64 = if ((p.p66 != 0.0) && (locals.var_vsat1r_i <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard92 = assign7590_e11049;
        locals.var_guard92_rv = 0.0;

        let (assign7600_e11053, assign7600_e11053_d_n0, assign7600_e11053_d_n2, assign7600_e11053_d_n3, assign7600_e11053_d_n4, assign7600_e11053_d_n5, assign7600_e11053_d_n6, assign7600_e11053_d_n7, assign7600_e11053_d_n8, assign7600_e11053_d_n9, assign7600_e11053_d_n10, assign7600_e11053_d_n11, assign7600_e11053_d_n13, assign7600_e11053_d_n14,) = {
    if (locals.var_guard92 != 0.0) {
        (85000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat1r_i, locals.var_vsat1r_i_dn0, locals.var_vsat1r_i_dn2, locals.var_vsat1r_i_dn3, locals.var_vsat1r_i_dn4, locals.var_vsat1r_i_dn5, locals.var_vsat1r_i_dn6, locals.var_vsat1r_i_dn7, locals.var_vsat1r_i_dn8, locals.var_vsat1r_i_dn9, locals.var_vsat1r_i_dn10, locals.var_vsat1r_i_dn11, locals.var_vsat1r_i_dn13, locals.var_vsat1r_i_dn14,)
    }
};
        locals.var_vsat1r_i = assign7600_e11053;
        locals.var_vsat1r_i_dn0 = assign7600_e11053_d_n0;
        locals.var_vsat1r_i_dn2 = assign7600_e11053_d_n2;
        locals.var_vsat1r_i_dn3 = assign7600_e11053_d_n3;
        locals.var_vsat1r_i_dn4 = assign7600_e11053_d_n4;
        locals.var_vsat1r_i_dn5 = assign7600_e11053_d_n5;
        locals.var_vsat1r_i_dn6 = assign7600_e11053_d_n6;
        locals.var_vsat1r_i_dn7 = assign7600_e11053_d_n7;
        locals.var_vsat1r_i_dn8 = assign7600_e11053_d_n8;
        locals.var_vsat1r_i_dn9 = assign7600_e11053_d_n9;
        locals.var_vsat1r_i_dn10 = assign7600_e11053_d_n10;
        locals.var_vsat1r_i_dn11 = assign7600_e11053_d_n11;
        locals.var_vsat1r_i_dn13 = assign7600_e11053_d_n13;
        locals.var_vsat1r_i_dn14 = assign7600_e11053_d_n14;
        locals.var_vsat1r_i_rv = 0.0;

        let assign7610_e11056: f64 = if locals.var_dvt1_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign7610_e11056;
        locals.var_guard93_rv = 0.0;

        let (assign7620_e11060,) = {
    if (locals.var_guard93 != 0.0) {
        (0.6,)
    } else {
        (locals.var_dvt1_i,)
    }
};
        locals.var_dvt1_i = assign7620_e11060;
        locals.var_dvt1_i_rv = 0.0;

        let assign7630_e11063: f64 = if locals.var_dvt1ss_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign7630_e11063;
        locals.var_guard94_rv = 0.0;

        let (assign7640_e11067,) = {
    if (locals.var_guard94 != 0.0) {
        (0.6,)
    } else {
        (locals.var_dvt1ss_i,)
    }
};
        locals.var_dvt1ss_i = assign7640_e11067;
        locals.var_dvt1ss_i_rv = 0.0;

        let assign7680_e11083: f64 = if locals.var_dsub_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign7680_e11083;
        locals.var_guard98_rv = 0.0;

        let (assign7690_e11087,) = {
    if (locals.var_guard98 != 0.0) {
        (1.06,)
    } else {
        (locals.var_dsub_i,)
    }
};
        locals.var_dsub_i = assign7690_e11087;
        locals.var_dsub_i_rv = 0.0;

        let assign7700_e11090: f64 = if locals.var_eta0_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign7700_e11090;
        locals.var_guard99_rv = 0.0;

        let (assign7710_e11094, assign7710_e11094_d_n0, assign7710_e11094_d_n2, assign7710_e11094_d_n3, assign7710_e11094_d_n4, assign7710_e11094_d_n5, assign7710_e11094_d_n6, assign7710_e11094_d_n7, assign7710_e11094_d_n8, assign7710_e11094_d_n9, assign7710_e11094_d_n10, assign7710_e11094_d_n11, assign7710_e11094_d_n13, assign7710_e11094_d_n14,) = {
    if (locals.var_guard99 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eta0_i, locals.var_eta0_i_dn0, locals.var_eta0_i_dn2, locals.var_eta0_i_dn3, locals.var_eta0_i_dn4, locals.var_eta0_i_dn5, locals.var_eta0_i_dn6, locals.var_eta0_i_dn7, locals.var_eta0_i_dn8, locals.var_eta0_i_dn9, locals.var_eta0_i_dn10, locals.var_eta0_i_dn11, locals.var_eta0_i_dn13, locals.var_eta0_i_dn14,)
    }
};
        locals.var_eta0_i = assign7710_e11094;
        locals.var_eta0_i_dn0 = assign7710_e11094_d_n0;
        locals.var_eta0_i_dn2 = assign7710_e11094_d_n2;
        locals.var_eta0_i_dn3 = assign7710_e11094_d_n3;
        locals.var_eta0_i_dn4 = assign7710_e11094_d_n4;
        locals.var_eta0_i_dn5 = assign7710_e11094_d_n5;
        locals.var_eta0_i_dn6 = assign7710_e11094_d_n6;
        locals.var_eta0_i_dn7 = assign7710_e11094_d_n7;
        locals.var_eta0_i_dn8 = assign7710_e11094_d_n8;
        locals.var_eta0_i_dn9 = assign7710_e11094_d_n9;
        locals.var_eta0_i_dn10 = assign7710_e11094_d_n10;
        locals.var_eta0_i_dn11 = assign7710_e11094_d_n11;
        locals.var_eta0_i_dn13 = assign7710_e11094_d_n13;
        locals.var_eta0_i_dn14 = assign7710_e11094_d_n14;
        locals.var_eta0_i_rv = 0.0;

        let assign7720_e11097: f64 = if locals.var_eta0r_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign7720_e11097;
        locals.var_guard100_rv = 0.0;

        let (assign7730_e11101,) = {
    if (locals.var_guard100 != 0.0) {
        (0.0,)
    } else {
        (locals.var_eta0r_i,)
    }
};
        locals.var_eta0r_i = assign7730_e11101;
        locals.var_eta0r_i_rv = 0.0;

        let assign7740_e11104: f64 = (-locals.var_leff_1);
        let assign7740_e11105: f64 = if locals.var_lpe0_i < assign7740_e11104 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign7740_e11105;
        locals.var_guard101_rv = 0.0;

        let (assign7750_e11109,) = {
    if (locals.var_guard101 != 0.0) {
        (0.0,)
    } else {
        (locals.var_lpe0_i,)
    }
};
        locals.var_lpe0_i = assign7750_e11109;
        locals.var_lpe0_i_rv = 0.0;

        let assign7760_e11112: f64 = if locals.var_k0si_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard102 = assign7760_e11112;
        locals.var_guard102_rv = 0.0;

        let (assign7770_e11116,) = {
    if (locals.var_guard102 != 0.0) {
        (0.0,)
    } else {
        (locals.var_k0si_i,)
    }
};
        locals.var_k0si_i = assign7770_e11116;
        locals.var_k0si_i_rv = 0.0;

        let assign7780_e11119: f64 = if locals.var_k2si_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard103 = assign7780_e11119;
        locals.var_guard103_rv = 0.0;

        let (assign7790_e11123,) = {
    if (locals.var_guard103 != 0.0) {
        (0.0,)
    } else {
        (locals.var_k2si_i,)
    }
};
        locals.var_k2si_i = assign7790_e11123;
        locals.var_k2si_i_rv = 0.0;

        let assign7800_e11130: f64 = if ((p.p61 != 0.0) && (locals.var_phibe_i < 0.2)) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign7800_e11130;
        locals.var_guard104_rv = 0.0;

        let (assign7810_e11134,) = {
    if (locals.var_guard104 != 0.0) {
        (0.2,)
    } else {
        (locals.var_phibe_i,)
    }
};
        locals.var_phibe_i = assign7810_e11134;
        locals.var_phibe_i_rv = 0.0;

        let assign7820_e11141: f64 = if ((p.p61 != 0.0) && (locals.var_phibe_i > 1.2)) { 1.0 } else { 0.0 };
        locals.var_guard105 = assign7820_e11141;
        locals.var_guard105_rv = 0.0;

        let (assign7830_e11145,) = {
    if (locals.var_guard105 != 0.0) {
        (1.2,)
    } else {
        (locals.var_phibe_i,)
    }
};
        locals.var_phibe_i = assign7830_e11145;
        locals.var_phibe_i_rv = 0.0;

        let assign7840_e11148: f64 = if locals.var_psat_i < 2.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign7840_e11148;
        locals.var_guard106_rv = 0.0;

        let (assign7850_e11152, assign7850_e11152_d_n0, assign7850_e11152_d_n2, assign7850_e11152_d_n3, assign7850_e11152_d_n4, assign7850_e11152_d_n5, assign7850_e11152_d_n6, assign7850_e11152_d_n7, assign7850_e11152_d_n8, assign7850_e11152_d_n9, assign7850_e11152_d_n10, assign7850_e11152_d_n11, assign7850_e11152_d_n13, assign7850_e11152_d_n14,) = {
    if (locals.var_guard106 != 0.0) {
        (2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psat_i, locals.var_psat_i_dn0, locals.var_psat_i_dn2, locals.var_psat_i_dn3, locals.var_psat_i_dn4, locals.var_psat_i_dn5, locals.var_psat_i_dn6, locals.var_psat_i_dn7, locals.var_psat_i_dn8, locals.var_psat_i_dn9, locals.var_psat_i_dn10, locals.var_psat_i_dn11, locals.var_psat_i_dn13, locals.var_psat_i_dn14,)
    }
};
        locals.var_psat_i = assign7850_e11152;
        locals.var_psat_i_dn0 = assign7850_e11152_d_n0;
        locals.var_psat_i_dn2 = assign7850_e11152_d_n2;
        locals.var_psat_i_dn3 = assign7850_e11152_d_n3;
        locals.var_psat_i_dn4 = assign7850_e11152_d_n4;
        locals.var_psat_i_dn5 = assign7850_e11152_d_n5;
        locals.var_psat_i_dn6 = assign7850_e11152_d_n6;
        locals.var_psat_i_dn7 = assign7850_e11152_d_n7;
        locals.var_psat_i_dn8 = assign7850_e11152_d_n8;
        locals.var_psat_i_dn9 = assign7850_e11152_d_n9;
        locals.var_psat_i_dn10 = assign7850_e11152_d_n10;
        locals.var_psat_i_dn11 = assign7850_e11152_d_n11;
        locals.var_psat_i_dn13 = assign7850_e11152_d_n13;
        locals.var_psat_i_dn14 = assign7850_e11152_d_n14;
        locals.var_psat_i_rv = 0.0;

        let assign7860_e11155: f64 = if locals.var_psatcv_i < 2.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign7860_e11155;
        locals.var_guard107_rv = 0.0;

        let (assign7870_e11159, assign7870_e11159_d_n0, assign7870_e11159_d_n2, assign7870_e11159_d_n3, assign7870_e11159_d_n4, assign7870_e11159_d_n5, assign7870_e11159_d_n6, assign7870_e11159_d_n7, assign7870_e11159_d_n8, assign7870_e11159_d_n9, assign7870_e11159_d_n10, assign7870_e11159_d_n11, assign7870_e11159_d_n13, assign7870_e11159_d_n14,) = {
    if (locals.var_guard107 != 0.0) {
        (2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psatcv_i, locals.var_psatcv_i_dn0, locals.var_psatcv_i_dn2, locals.var_psatcv_i_dn3, locals.var_psatcv_i_dn4, locals.var_psatcv_i_dn5, locals.var_psatcv_i_dn6, locals.var_psatcv_i_dn7, locals.var_psatcv_i_dn8, locals.var_psatcv_i_dn9, locals.var_psatcv_i_dn10, locals.var_psatcv_i_dn11, locals.var_psatcv_i_dn13, locals.var_psatcv_i_dn14,)
    }
};
        locals.var_psatcv_i = assign7870_e11159;
        locals.var_psatcv_i_dn0 = assign7870_e11159_d_n0;
        locals.var_psatcv_i_dn2 = assign7870_e11159_d_n2;
        locals.var_psatcv_i_dn3 = assign7870_e11159_d_n3;
        locals.var_psatcv_i_dn4 = assign7870_e11159_d_n4;
        locals.var_psatcv_i_dn5 = assign7870_e11159_d_n5;
        locals.var_psatcv_i_dn6 = assign7870_e11159_d_n6;
        locals.var_psatcv_i_dn7 = assign7870_e11159_d_n7;
        locals.var_psatcv_i_dn8 = assign7870_e11159_d_n8;
        locals.var_psatcv_i_dn9 = assign7870_e11159_d_n9;
        locals.var_psatcv_i_dn10 = assign7870_e11159_d_n10;
        locals.var_psatcv_i_dn11 = assign7870_e11159_d_n11;
        locals.var_psatcv_i_dn13 = assign7870_e11159_d_n13;
        locals.var_psatcv_i_dn14 = assign7870_e11159_d_n14;
        locals.var_psatcv_i_rv = 0.0;

        let assign7880_e11162: f64 = if locals.var_u0_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign7880_e11162;
        locals.var_guard108_rv = 0.0;

        let (assign7890_e11166, assign7890_e11166_d_n0, assign7890_e11166_d_n2, assign7890_e11166_d_n3, assign7890_e11166_d_n4, assign7890_e11166_d_n5, assign7890_e11166_d_n6, assign7890_e11166_d_n7, assign7890_e11166_d_n8, assign7890_e11166_d_n9, assign7890_e11166_d_n10, assign7890_e11166_d_n11, assign7890_e11166_d_n13, assign7890_e11166_d_n14,) = {
    if (locals.var_guard108 != 0.0) {
        (0.03, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_u0_i, locals.var_u0_i_dn0, locals.var_u0_i_dn2, locals.var_u0_i_dn3, locals.var_u0_i_dn4, locals.var_u0_i_dn5, locals.var_u0_i_dn6, locals.var_u0_i_dn7, locals.var_u0_i_dn8, locals.var_u0_i_dn9, locals.var_u0_i_dn10, locals.var_u0_i_dn11, locals.var_u0_i_dn13, locals.var_u0_i_dn14,)
    }
};
        locals.var_u0_i = assign7890_e11166;
        locals.var_u0_i_dn0 = assign7890_e11166_d_n0;
        locals.var_u0_i_dn2 = assign7890_e11166_d_n2;
        locals.var_u0_i_dn3 = assign7890_e11166_d_n3;
        locals.var_u0_i_dn4 = assign7890_e11166_d_n4;
        locals.var_u0_i_dn5 = assign7890_e11166_d_n5;
        locals.var_u0_i_dn6 = assign7890_e11166_d_n6;
        locals.var_u0_i_dn7 = assign7890_e11166_d_n7;
        locals.var_u0_i_dn8 = assign7890_e11166_d_n8;
        locals.var_u0_i_dn9 = assign7890_e11166_d_n9;
        locals.var_u0_i_dn10 = assign7890_e11166_d_n10;
        locals.var_u0_i_dn11 = assign7890_e11166_d_n11;
        locals.var_u0_i_dn13 = assign7890_e11166_d_n13;
        locals.var_u0_i_dn14 = assign7890_e11166_d_n14;
        locals.var_u0_i_rv = 0.0;

        let assign7900_e11169: f64 = if locals.var_ua_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign7900_e11169;
        locals.var_guard109_rv = 0.0;

        let (assign7910_e11173, assign7910_e11173_d_n0, assign7910_e11173_d_n2, assign7910_e11173_d_n3, assign7910_e11173_d_n4, assign7910_e11173_d_n5, assign7910_e11173_d_n6, assign7910_e11173_d_n7, assign7910_e11173_d_n8, assign7910_e11173_d_n9, assign7910_e11173_d_n10, assign7910_e11173_d_n11, assign7910_e11173_d_n13, assign7910_e11173_d_n14,) = {
    if (locals.var_guard109 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ua_i, locals.var_ua_i_dn0, locals.var_ua_i_dn2, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11, locals.var_ua_i_dn13, locals.var_ua_i_dn14,)
    }
};
        locals.var_ua_i = assign7910_e11173;
        locals.var_ua_i_dn0 = assign7910_e11173_d_n0;
        locals.var_ua_i_dn2 = assign7910_e11173_d_n2;
        locals.var_ua_i_dn3 = assign7910_e11173_d_n3;
        locals.var_ua_i_dn4 = assign7910_e11173_d_n4;
        locals.var_ua_i_dn5 = assign7910_e11173_d_n5;
        locals.var_ua_i_dn6 = assign7910_e11173_d_n6;
        locals.var_ua_i_dn7 = assign7910_e11173_d_n7;
        locals.var_ua_i_dn8 = assign7910_e11173_d_n8;
        locals.var_ua_i_dn9 = assign7910_e11173_d_n9;
        locals.var_ua_i_dn10 = assign7910_e11173_d_n10;
        locals.var_ua_i_dn11 = assign7910_e11173_d_n11;
        locals.var_ua_i_dn13 = assign7910_e11173_d_n13;
        locals.var_ua_i_dn14 = assign7910_e11173_d_n14;
        locals.var_ua_i_rv = 0.0;

        let assign7920_e11176: f64 = if locals.var_eu_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign7920_e11176;
        locals.var_guard110_rv = 0.0;

        let (assign7930_e11180, assign7930_e11180_d_n0, assign7930_e11180_d_n2, assign7930_e11180_d_n3, assign7930_e11180_d_n4, assign7930_e11180_d_n5, assign7930_e11180_d_n6, assign7930_e11180_d_n7, assign7930_e11180_d_n8, assign7930_e11180_d_n9, assign7930_e11180_d_n10, assign7930_e11180_d_n11, assign7930_e11180_d_n13, assign7930_e11180_d_n14,) = {
    if (locals.var_guard110 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eu_i, locals.var_eu_i_dn0, locals.var_eu_i_dn2, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11, locals.var_eu_i_dn13, locals.var_eu_i_dn14,)
    }
};
        locals.var_eu_i = assign7930_e11180;
        locals.var_eu_i_dn0 = assign7930_e11180_d_n0;
        locals.var_eu_i_dn2 = assign7930_e11180_d_n2;
        locals.var_eu_i_dn3 = assign7930_e11180_d_n3;
        locals.var_eu_i_dn4 = assign7930_e11180_d_n4;
        locals.var_eu_i_dn5 = assign7930_e11180_d_n5;
        locals.var_eu_i_dn6 = assign7930_e11180_d_n6;
        locals.var_eu_i_dn7 = assign7930_e11180_d_n7;
        locals.var_eu_i_dn8 = assign7930_e11180_d_n8;
        locals.var_eu_i_dn9 = assign7930_e11180_d_n9;
        locals.var_eu_i_dn10 = assign7930_e11180_d_n10;
        locals.var_eu_i_dn11 = assign7930_e11180_d_n11;
        locals.var_eu_i_dn13 = assign7930_e11180_d_n13;
        locals.var_eu_i_dn14 = assign7930_e11180_d_n14;
        locals.var_eu_i_rv = 0.0;

        let assign7940_e11183: f64 = if locals.var_ud_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign7940_e11183;
        locals.var_guard111_rv = 0.0;

        let (assign7950_e11187, assign7950_e11187_d_n0, assign7950_e11187_d_n2, assign7950_e11187_d_n3, assign7950_e11187_d_n4, assign7950_e11187_d_n5, assign7950_e11187_d_n6, assign7950_e11187_d_n7, assign7950_e11187_d_n8, assign7950_e11187_d_n9, assign7950_e11187_d_n10, assign7950_e11187_d_n11, assign7950_e11187_d_n13, assign7950_e11187_d_n14,) = {
    if (locals.var_guard111 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ud_i, locals.var_ud_i_dn0, locals.var_ud_i_dn2, locals.var_ud_i_dn3, locals.var_ud_i_dn4, locals.var_ud_i_dn5, locals.var_ud_i_dn6, locals.var_ud_i_dn7, locals.var_ud_i_dn8, locals.var_ud_i_dn9, locals.var_ud_i_dn10, locals.var_ud_i_dn11, locals.var_ud_i_dn13, locals.var_ud_i_dn14,)
    }
};
        locals.var_ud_i = assign7950_e11187;
        locals.var_ud_i_dn0 = assign7950_e11187_d_n0;
        locals.var_ud_i_dn2 = assign7950_e11187_d_n2;
        locals.var_ud_i_dn3 = assign7950_e11187_d_n3;
        locals.var_ud_i_dn4 = assign7950_e11187_d_n4;
        locals.var_ud_i_dn5 = assign7950_e11187_d_n5;
        locals.var_ud_i_dn6 = assign7950_e11187_d_n6;
        locals.var_ud_i_dn7 = assign7950_e11187_d_n7;
        locals.var_ud_i_dn8 = assign7950_e11187_d_n8;
        locals.var_ud_i_dn9 = assign7950_e11187_d_n9;
        locals.var_ud_i_dn10 = assign7950_e11187_d_n10;
        locals.var_ud_i_dn11 = assign7950_e11187_d_n11;
        locals.var_ud_i_dn13 = assign7950_e11187_d_n13;
        locals.var_ud_i_dn14 = assign7950_e11187_d_n14;
        locals.var_ud_i_rv = 0.0;

        let assign7960_e11190: f64 = if locals.var_ucs_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign7960_e11190;
        locals.var_guard112_rv = 0.0;

        let (assign7970_e11194,) = {
    if (locals.var_guard112 != 0.0) {
        (0.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign7970_e11194;
        locals.var_ucs_i_rv = 0.0;

        let assign7980_e11197: f64 = if locals.var_etamob_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign7980_e11197;
        locals.var_guard113_rv = 0.0;

        let (assign7990_e11201,) = {
    if (locals.var_guard113 != 0.0) {
        (0.0,)
    } else {
        (locals.var_etamob_i,)
    }
};
        locals.var_etamob_i = assign7990_e11201;
        locals.var_etamob_i_rv = 0.0;

        let assign8000_e11204: f64 = if locals.var_rdsw_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard114 = assign8000_e11204;
        locals.var_guard114_rv = 0.0;

        let (assign8010_e11208, assign8010_e11208_d_n0, assign8010_e11208_d_n2, assign8010_e11208_d_n3, assign8010_e11208_d_n4, assign8010_e11208_d_n5, assign8010_e11208_d_n6, assign8010_e11208_d_n7, assign8010_e11208_d_n8, assign8010_e11208_d_n9, assign8010_e11208_d_n10, assign8010_e11208_d_n11, assign8010_e11208_d_n13, assign8010_e11208_d_n14,) = {
    if (locals.var_guard114 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdsw_i, locals.var_rdsw_i_dn0, locals.var_rdsw_i_dn2, locals.var_rdsw_i_dn3, locals.var_rdsw_i_dn4, locals.var_rdsw_i_dn5, locals.var_rdsw_i_dn6, locals.var_rdsw_i_dn7, locals.var_rdsw_i_dn8, locals.var_rdsw_i_dn9, locals.var_rdsw_i_dn10, locals.var_rdsw_i_dn11, locals.var_rdsw_i_dn13, locals.var_rdsw_i_dn14,)
    }
};
        locals.var_rdsw_i = assign8010_e11208;
        locals.var_rdsw_i_dn0 = assign8010_e11208_d_n0;
        locals.var_rdsw_i_dn2 = assign8010_e11208_d_n2;
        locals.var_rdsw_i_dn3 = assign8010_e11208_d_n3;
        locals.var_rdsw_i_dn4 = assign8010_e11208_d_n4;
        locals.var_rdsw_i_dn5 = assign8010_e11208_d_n5;
        locals.var_rdsw_i_dn6 = assign8010_e11208_d_n6;
        locals.var_rdsw_i_dn7 = assign8010_e11208_d_n7;
        locals.var_rdsw_i_dn8 = assign8010_e11208_d_n8;
        locals.var_rdsw_i_dn9 = assign8010_e11208_d_n9;
        locals.var_rdsw_i_dn10 = assign8010_e11208_d_n10;
        locals.var_rdsw_i_dn11 = assign8010_e11208_d_n11;
        locals.var_rdsw_i_dn13 = assign8010_e11208_d_n13;
        locals.var_rdsw_i_dn14 = assign8010_e11208_d_n14;
        locals.var_rdsw_i_rv = 0.0;

        let assign8020_e11211: f64 = if locals.var_rsw_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign8020_e11211;
        locals.var_guard115_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8030_e11215, assign8030_e11215_d_n0, assign8030_e11215_d_n2, assign8030_e11215_d_n3, assign8030_e11215_d_n4, assign8030_e11215_d_n5, assign8030_e11215_d_n6, assign8030_e11215_d_n7, assign8030_e11215_d_n8, assign8030_e11215_d_n9, assign8030_e11215_d_n10, assign8030_e11215_d_n11, assign8030_e11215_d_n13, assign8030_e11215_d_n14,) = {
    if (locals.var_guard115 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsw_i, locals.var_rsw_i_dn0, locals.var_rsw_i_dn2, locals.var_rsw_i_dn3, locals.var_rsw_i_dn4, locals.var_rsw_i_dn5, locals.var_rsw_i_dn6, locals.var_rsw_i_dn7, locals.var_rsw_i_dn8, locals.var_rsw_i_dn9, locals.var_rsw_i_dn10, locals.var_rsw_i_dn11, locals.var_rsw_i_dn13, locals.var_rsw_i_dn14,)
    }
};
        locals.var_rsw_i = assign8030_e11215;
        locals.var_rsw_i_dn0 = assign8030_e11215_d_n0;
        locals.var_rsw_i_dn2 = assign8030_e11215_d_n2;
        locals.var_rsw_i_dn3 = assign8030_e11215_d_n3;
        locals.var_rsw_i_dn4 = assign8030_e11215_d_n4;
        locals.var_rsw_i_dn5 = assign8030_e11215_d_n5;
        locals.var_rsw_i_dn6 = assign8030_e11215_d_n6;
        locals.var_rsw_i_dn7 = assign8030_e11215_d_n7;
        locals.var_rsw_i_dn8 = assign8030_e11215_d_n8;
        locals.var_rsw_i_dn9 = assign8030_e11215_d_n9;
        locals.var_rsw_i_dn10 = assign8030_e11215_d_n10;
        locals.var_rsw_i_dn11 = assign8030_e11215_d_n11;
        locals.var_rsw_i_dn13 = assign8030_e11215_d_n13;
        locals.var_rsw_i_dn14 = assign8030_e11215_d_n14;
        locals.var_rsw_i_rv = 0.0;

        let assign8040_e11218: f64 = if locals.var_rdw_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign8040_e11218;
        locals.var_guard116_rv = 0.0;

        let (assign8050_e11222, assign8050_e11222_d_n0, assign8050_e11222_d_n2, assign8050_e11222_d_n3, assign8050_e11222_d_n4, assign8050_e11222_d_n5, assign8050_e11222_d_n6, assign8050_e11222_d_n7, assign8050_e11222_d_n8, assign8050_e11222_d_n9, assign8050_e11222_d_n10, assign8050_e11222_d_n11, assign8050_e11222_d_n13, assign8050_e11222_d_n14,) = {
    if (locals.var_guard116 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdw_i, locals.var_rdw_i_dn0, locals.var_rdw_i_dn2, locals.var_rdw_i_dn3, locals.var_rdw_i_dn4, locals.var_rdw_i_dn5, locals.var_rdw_i_dn6, locals.var_rdw_i_dn7, locals.var_rdw_i_dn8, locals.var_rdw_i_dn9, locals.var_rdw_i_dn10, locals.var_rdw_i_dn11, locals.var_rdw_i_dn13, locals.var_rdw_i_dn14,)
    }
};
        locals.var_rdw_i = assign8050_e11222;
        locals.var_rdw_i_dn0 = assign8050_e11222_d_n0;
        locals.var_rdw_i_dn2 = assign8050_e11222_d_n2;
        locals.var_rdw_i_dn3 = assign8050_e11222_d_n3;
        locals.var_rdw_i_dn4 = assign8050_e11222_d_n4;
        locals.var_rdw_i_dn5 = assign8050_e11222_d_n5;
        locals.var_rdw_i_dn6 = assign8050_e11222_d_n6;
        locals.var_rdw_i_dn7 = assign8050_e11222_d_n7;
        locals.var_rdw_i_dn8 = assign8050_e11222_d_n8;
        locals.var_rdw_i_dn9 = assign8050_e11222_d_n9;
        locals.var_rdw_i_dn10 = assign8050_e11222_d_n10;
        locals.var_rdw_i_dn11 = assign8050_e11222_d_n11;
        locals.var_rdw_i_dn13 = assign8050_e11222_d_n13;
        locals.var_rdw_i_dn14 = assign8050_e11222_d_n14;
        locals.var_rdw_i_rv = 0.0;

        let assign8060_e11225: f64 = if locals.var_prwgd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign8060_e11225;
        locals.var_guard117_rv = 0.0;

        let (assign8070_e11229,) = {
    if (locals.var_guard117 != 0.0) {
        (0.0,)
    } else {
        (locals.var_prwgd_i,)
    }
};
        locals.var_prwgd_i = assign8070_e11229;
        locals.var_prwgd_i_rv = 0.0;

        let assign8080_e11232: f64 = if locals.var_prwgs_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign8080_e11232;
        locals.var_guard118_rv = 0.0;

        let (assign8090_e11236,) = {
    if (locals.var_guard118 != 0.0) {
        (0.0,)
    } else {
        (locals.var_prwgs_i,)
    }
};
        locals.var_prwgs_i = assign8090_e11236;
        locals.var_prwgs_i_rv = 0.0;

        let assign8120_e11245: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign8120_e11245;
        locals.var_guard121_rv = 0.0;

        let assign8150_e11254: f64 = if locals.var_u0r_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign8150_e11254;
        locals.var_guard124_rv = 0.0;

        let (assign8160_e11260, assign8160_e11260_d_n0, assign8160_e11260_d_n2, assign8160_e11260_d_n3, assign8160_e11260_d_n4, assign8160_e11260_d_n5, assign8160_e11260_d_n6, assign8160_e11260_d_n7, assign8160_e11260_d_n8, assign8160_e11260_d_n9, assign8160_e11260_d_n10, assign8160_e11260_d_n11, assign8160_e11260_d_n13, assign8160_e11260_d_n14,) = {
    if ((locals.var_guard121 != 0.0) && (locals.var_guard124 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_u0r_i, locals.var_u0r_i_dn0, locals.var_u0r_i_dn2, locals.var_u0r_i_dn3, locals.var_u0r_i_dn4, locals.var_u0r_i_dn5, locals.var_u0r_i_dn6, locals.var_u0r_i_dn7, locals.var_u0r_i_dn8, locals.var_u0r_i_dn9, locals.var_u0r_i_dn10, locals.var_u0r_i_dn11, locals.var_u0r_i_dn13, locals.var_u0r_i_dn14,)
    }
};
        locals.var_u0r_i = assign8160_e11260;
        locals.var_u0r_i_dn0 = assign8160_e11260_d_n0;
        locals.var_u0r_i_dn2 = assign8160_e11260_d_n2;
        locals.var_u0r_i_dn3 = assign8160_e11260_d_n3;
        locals.var_u0r_i_dn4 = assign8160_e11260_d_n4;
        locals.var_u0r_i_dn5 = assign8160_e11260_d_n5;
        locals.var_u0r_i_dn6 = assign8160_e11260_d_n6;
        locals.var_u0r_i_dn7 = assign8160_e11260_d_n7;
        locals.var_u0r_i_dn8 = assign8160_e11260_d_n8;
        locals.var_u0r_i_dn9 = assign8160_e11260_d_n9;
        locals.var_u0r_i_dn10 = assign8160_e11260_d_n10;
        locals.var_u0r_i_dn11 = assign8160_e11260_d_n11;
        locals.var_u0r_i_dn13 = assign8160_e11260_d_n13;
        locals.var_u0r_i_dn14 = assign8160_e11260_d_n14;
        locals.var_u0r_i_rv = 0.0;

        let assign8170_e11263: f64 = if locals.var_uar_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign8170_e11263;
        locals.var_guard125_rv = 0.0;

        let (assign8180_e11269, assign8180_e11269_d_n0, assign8180_e11269_d_n2, assign8180_e11269_d_n3, assign8180_e11269_d_n4, assign8180_e11269_d_n5, assign8180_e11269_d_n6, assign8180_e11269_d_n7, assign8180_e11269_d_n8, assign8180_e11269_d_n9, assign8180_e11269_d_n10, assign8180_e11269_d_n11, assign8180_e11269_d_n13, assign8180_e11269_d_n14,) = {
    if ((locals.var_guard121 != 0.0) && (locals.var_guard125 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uar_i, locals.var_uar_i_dn0, locals.var_uar_i_dn2, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11, locals.var_uar_i_dn13, locals.var_uar_i_dn14,)
    }
};
        locals.var_uar_i = assign8180_e11269;
        locals.var_uar_i_dn0 = assign8180_e11269_d_n0;
        locals.var_uar_i_dn2 = assign8180_e11269_d_n2;
        locals.var_uar_i_dn3 = assign8180_e11269_d_n3;
        locals.var_uar_i_dn4 = assign8180_e11269_d_n4;
        locals.var_uar_i_dn5 = assign8180_e11269_d_n5;
        locals.var_uar_i_dn6 = assign8180_e11269_d_n6;
        locals.var_uar_i_dn7 = assign8180_e11269_d_n7;
        locals.var_uar_i_dn8 = assign8180_e11269_d_n8;
        locals.var_uar_i_dn9 = assign8180_e11269_d_n9;
        locals.var_uar_i_dn10 = assign8180_e11269_d_n10;
        locals.var_uar_i_dn11 = assign8180_e11269_d_n11;
        locals.var_uar_i_dn13 = assign8180_e11269_d_n13;
        locals.var_uar_i_dn14 = assign8180_e11269_d_n14;
        locals.var_uar_i_rv = 0.0;

        let assign8190_e11272: f64 = if locals.var_eur_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign8190_e11272;
        locals.var_guard126_rv = 0.0;

        let (assign8200_e11278, assign8200_e11278_d_n0, assign8200_e11278_d_n2, assign8200_e11278_d_n3, assign8200_e11278_d_n4, assign8200_e11278_d_n5, assign8200_e11278_d_n6, assign8200_e11278_d_n7, assign8200_e11278_d_n8, assign8200_e11278_d_n9, assign8200_e11278_d_n10, assign8200_e11278_d_n11, assign8200_e11278_d_n13, assign8200_e11278_d_n14,) = {
    if ((locals.var_guard121 != 0.0) && (locals.var_guard126 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eur_i, locals.var_eur_i_dn0, locals.var_eur_i_dn2, locals.var_eur_i_dn3, locals.var_eur_i_dn4, locals.var_eur_i_dn5, locals.var_eur_i_dn6, locals.var_eur_i_dn7, locals.var_eur_i_dn8, locals.var_eur_i_dn9, locals.var_eur_i_dn10, locals.var_eur_i_dn11, locals.var_eur_i_dn13, locals.var_eur_i_dn14,)
    }
};
        locals.var_eur_i = assign8200_e11278;
        locals.var_eur_i_dn0 = assign8200_e11278_d_n0;
        locals.var_eur_i_dn2 = assign8200_e11278_d_n2;
        locals.var_eur_i_dn3 = assign8200_e11278_d_n3;
        locals.var_eur_i_dn4 = assign8200_e11278_d_n4;
        locals.var_eur_i_dn5 = assign8200_e11278_d_n5;
        locals.var_eur_i_dn6 = assign8200_e11278_d_n6;
        locals.var_eur_i_dn7 = assign8200_e11278_d_n7;
        locals.var_eur_i_dn8 = assign8200_e11278_d_n8;
        locals.var_eur_i_dn9 = assign8200_e11278_d_n9;
        locals.var_eur_i_dn10 = assign8200_e11278_d_n10;
        locals.var_eur_i_dn11 = assign8200_e11278_d_n11;
        locals.var_eur_i_dn13 = assign8200_e11278_d_n13;
        locals.var_eur_i_dn14 = assign8200_e11278_d_n14;
        locals.var_eur_i_rv = 0.0;

        let assign8210_e11281: f64 = if locals.var_udr_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard127 = assign8210_e11281;
        locals.var_guard127_rv = 0.0;

        let (assign8220_e11287, assign8220_e11287_d_n0, assign8220_e11287_d_n2, assign8220_e11287_d_n3, assign8220_e11287_d_n4, assign8220_e11287_d_n5, assign8220_e11287_d_n6, assign8220_e11287_d_n7, assign8220_e11287_d_n8, assign8220_e11287_d_n9, assign8220_e11287_d_n10, assign8220_e11287_d_n11, assign8220_e11287_d_n13, assign8220_e11287_d_n14,) = {
    if ((locals.var_guard121 != 0.0) && (locals.var_guard127 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_udr_i, locals.var_udr_i_dn0, locals.var_udr_i_dn2, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11, locals.var_udr_i_dn13, locals.var_udr_i_dn14,)
    }
};
        locals.var_udr_i = assign8220_e11287;
        locals.var_udr_i_dn0 = assign8220_e11287_d_n0;
        locals.var_udr_i_dn2 = assign8220_e11287_d_n2;
        locals.var_udr_i_dn3 = assign8220_e11287_d_n3;
        locals.var_udr_i_dn4 = assign8220_e11287_d_n4;
        locals.var_udr_i_dn5 = assign8220_e11287_d_n5;
        locals.var_udr_i_dn6 = assign8220_e11287_d_n6;
        locals.var_udr_i_dn7 = assign8220_e11287_d_n7;
        locals.var_udr_i_dn8 = assign8220_e11287_d_n8;
        locals.var_udr_i_dn9 = assign8220_e11287_d_n9;
        locals.var_udr_i_dn10 = assign8220_e11287_d_n10;
        locals.var_udr_i_dn11 = assign8220_e11287_d_n11;
        locals.var_udr_i_dn13 = assign8220_e11287_d_n13;
        locals.var_udr_i_dn14 = assign8220_e11287_d_n14;
        locals.var_udr_i_rv = 0.0;

        let assign8240_e11293: f64 = if locals.var_drout_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard129 = assign8240_e11293;
        locals.var_guard129_rv = 0.0;

        let (assign8250_e11297,) = {
    if (locals.var_guard129 != 0.0) {
        (1.06,)
    } else {
        (locals.var_drout_i,)
    }
};
        locals.var_drout_i = assign8250_e11297;
        locals.var_drout_i_rv = 0.0;

        let assign8260_e11300: f64 = if locals.var_mexp_i < 2.0 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign8260_e11300;
        locals.var_guard130_rv = 0.0;

        let (assign8270_e11304, assign8270_e11304_d_n0, assign8270_e11304_d_n2, assign8270_e11304_d_n3, assign8270_e11304_d_n4, assign8270_e11304_d_n5, assign8270_e11304_d_n6, assign8270_e11304_d_n7, assign8270_e11304_d_n8, assign8270_e11304_d_n9, assign8270_e11304_d_n10, assign8270_e11304_d_n11, assign8270_e11304_d_n13, assign8270_e11304_d_n14,) = {
    if (locals.var_guard130 != 0.0) {
        (2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mexp_i, locals.var_mexp_i_dn0, locals.var_mexp_i_dn2, locals.var_mexp_i_dn3, locals.var_mexp_i_dn4, locals.var_mexp_i_dn5, locals.var_mexp_i_dn6, locals.var_mexp_i_dn7, locals.var_mexp_i_dn8, locals.var_mexp_i_dn9, locals.var_mexp_i_dn10, locals.var_mexp_i_dn11, locals.var_mexp_i_dn13, locals.var_mexp_i_dn14,)
    }
};
        locals.var_mexp_i = assign8270_e11304;
        locals.var_mexp_i_dn0 = assign8270_e11304_d_n0;
        locals.var_mexp_i_dn2 = assign8270_e11304_d_n2;
        locals.var_mexp_i_dn3 = assign8270_e11304_d_n3;
        locals.var_mexp_i_dn4 = assign8270_e11304_d_n4;
        locals.var_mexp_i_dn5 = assign8270_e11304_d_n5;
        locals.var_mexp_i_dn6 = assign8270_e11304_d_n6;
        locals.var_mexp_i_dn7 = assign8270_e11304_d_n7;
        locals.var_mexp_i_dn8 = assign8270_e11304_d_n8;
        locals.var_mexp_i_dn9 = assign8270_e11304_d_n9;
        locals.var_mexp_i_dn10 = assign8270_e11304_d_n10;
        locals.var_mexp_i_dn11 = assign8270_e11304_d_n11;
        locals.var_mexp_i_dn13 = assign8270_e11304_d_n13;
        locals.var_mexp_i_dn14 = assign8270_e11304_d_n14;
        locals.var_mexp_i_rv = 0.0;

        let assign8280_e11307: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign8280_e11307;
        locals.var_guard131_rv = 0.0;

        let assign8290_e11310: f64 = if locals.var_mexpr_i < 2.0 { 1.0 } else { 0.0 };
        locals.var_guard132 = assign8290_e11310;
        locals.var_guard132_rv = 0.0;

        let (assign8300_e11316, assign8300_e11316_d_n0, assign8300_e11316_d_n2, assign8300_e11316_d_n3, assign8300_e11316_d_n4, assign8300_e11316_d_n5, assign8300_e11316_d_n6, assign8300_e11316_d_n7, assign8300_e11316_d_n8, assign8300_e11316_d_n9, assign8300_e11316_d_n10, assign8300_e11316_d_n11, assign8300_e11316_d_n13, assign8300_e11316_d_n14,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard132 != 0.0)) {
        (2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mexpr_i, locals.var_mexpr_i_dn0, locals.var_mexpr_i_dn2, locals.var_mexpr_i_dn3, locals.var_mexpr_i_dn4, locals.var_mexpr_i_dn5, locals.var_mexpr_i_dn6, locals.var_mexpr_i_dn7, locals.var_mexpr_i_dn8, locals.var_mexpr_i_dn9, locals.var_mexpr_i_dn10, locals.var_mexpr_i_dn11, locals.var_mexpr_i_dn13, locals.var_mexpr_i_dn14,)
    }
};
        locals.var_mexpr_i = assign8300_e11316;
        locals.var_mexpr_i_dn0 = assign8300_e11316_d_n0;
        locals.var_mexpr_i_dn2 = assign8300_e11316_d_n2;
        locals.var_mexpr_i_dn3 = assign8300_e11316_d_n3;
        locals.var_mexpr_i_dn4 = assign8300_e11316_d_n4;
        locals.var_mexpr_i_dn5 = assign8300_e11316_d_n5;
        locals.var_mexpr_i_dn6 = assign8300_e11316_d_n6;
        locals.var_mexpr_i_dn7 = assign8300_e11316_d_n7;
        locals.var_mexpr_i_dn8 = assign8300_e11316_d_n8;
        locals.var_mexpr_i_dn9 = assign8300_e11316_d_n9;
        locals.var_mexpr_i_dn10 = assign8300_e11316_d_n10;
        locals.var_mexpr_i_dn11 = assign8300_e11316_d_n11;
        locals.var_mexpr_i_dn13 = assign8300_e11316_d_n13;
        locals.var_mexpr_i_dn14 = assign8300_e11316_d_n14;
        locals.var_mexpr_i_rv = 0.0;

        let assign8310_e11319: f64 = if locals.var_ptwg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard133 = assign8310_e11319;
        locals.var_guard133_rv = 0.0;

        let (assign8320_e11323, assign8320_e11323_d_n0, assign8320_e11323_d_n2, assign8320_e11323_d_n3, assign8320_e11323_d_n4, assign8320_e11323_d_n5, assign8320_e11323_d_n6, assign8320_e11323_d_n7, assign8320_e11323_d_n8, assign8320_e11323_d_n9, assign8320_e11323_d_n10, assign8320_e11323_d_n11, assign8320_e11323_d_n13, assign8320_e11323_d_n14,) = {
    if (locals.var_guard133 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptwg_i, locals.var_ptwg_i_dn0, locals.var_ptwg_i_dn2, locals.var_ptwg_i_dn3, locals.var_ptwg_i_dn4, locals.var_ptwg_i_dn5, locals.var_ptwg_i_dn6, locals.var_ptwg_i_dn7, locals.var_ptwg_i_dn8, locals.var_ptwg_i_dn9, locals.var_ptwg_i_dn10, locals.var_ptwg_i_dn11, locals.var_ptwg_i_dn13, locals.var_ptwg_i_dn14,)
    }
};
        locals.var_ptwg_i = assign8320_e11323;
        locals.var_ptwg_i_dn0 = assign8320_e11323_d_n0;
        locals.var_ptwg_i_dn2 = assign8320_e11323_d_n2;
        locals.var_ptwg_i_dn3 = assign8320_e11323_d_n3;
        locals.var_ptwg_i_dn4 = assign8320_e11323_d_n4;
        locals.var_ptwg_i_dn5 = assign8320_e11323_d_n5;
        locals.var_ptwg_i_dn6 = assign8320_e11323_d_n6;
        locals.var_ptwg_i_dn7 = assign8320_e11323_d_n7;
        locals.var_ptwg_i_dn8 = assign8320_e11323_d_n8;
        locals.var_ptwg_i_dn9 = assign8320_e11323_d_n9;
        locals.var_ptwg_i_dn10 = assign8320_e11323_d_n10;
        locals.var_ptwg_i_dn11 = assign8320_e11323_d_n11;
        locals.var_ptwg_i_dn13 = assign8320_e11323_d_n13;
        locals.var_ptwg_i_dn14 = assign8320_e11323_d_n14;
        locals.var_ptwg_i_rv = 0.0;

        let assign8330_e11326: f64 = if locals.var_cgidl_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign8330_e11326;
        locals.var_guard134_rv = 0.0;

        let (assign8340_e11330,) = {
    if (locals.var_guard134 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cgidl_i,)
    }
};
        locals.var_cgidl_i = assign8340_e11330;
        locals.var_cgidl_i_rv = 0.0;

        let assign8350_e11333: f64 = if locals.var_cgisl_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard135 = assign8350_e11333;
        locals.var_guard135_rv = 0.0;

        let (assign8360_e11337,) = {
    if (locals.var_guard135 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cgisl_i,)
    }
};
        locals.var_cgisl_i = assign8360_e11337;
        locals.var_cgisl_i_rv = 0.0;

        let assign8370_e11340: f64 = if p.p69 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard136 = assign8370_e11340;
        locals.var_guard136_rv = 0.0;

        let assign8380_e11343: f64 = if locals.var_nigbinv_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard137 = assign8380_e11343;
        locals.var_guard137_rv = 0.0;

        let (assign8390_e11349,) = {
    if ((locals.var_guard136 != 0.0) && (locals.var_guard137 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_nigbinv_i,)
    }
};
        locals.var_nigbinv_i = assign8390_e11349;
        locals.var_nigbinv_i_rv = 0.0;

        let assign8400_e11352: f64 = if locals.var_nigbacc_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard138 = assign8400_e11352;
        locals.var_guard138_rv = 0.0;

        let (assign8410_e11358,) = {
    if ((locals.var_guard136 != 0.0) && (locals.var_guard138 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nigbacc_i,)
    }
};
        locals.var_nigbacc_i = assign8410_e11358;
        locals.var_nigbacc_i_rv = 0.0;

        let assign8420_e11361: f64 = if p.p68 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign8420_e11361;
        locals.var_guard139_rv = 0.0;

        let assign8430_e11364: f64 = if locals.var_poxedge_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard140 = assign8430_e11364;
        locals.var_guard140_rv = 0.0;

        let (assign8440_e11370,) = {
    if ((locals.var_guard139 != 0.0) && (locals.var_guard140 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_poxedge_i,)
    }
};
        locals.var_poxedge_i = assign8440_e11370;
        locals.var_poxedge_i_rv = 0.0;

        let assign8450_e11373: f64 = if locals.var_pigcd_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard141 = assign8450_e11373;
        locals.var_guard141_rv = 0.0;

        let (assign8460_e11379,) = {
    if ((locals.var_guard139 != 0.0) && (locals.var_guard141 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_pigcd_i,)
    }
};
        locals.var_pigcd_i = assign8460_e11379;
        locals.var_pigcd_i_rv = 0.0;

        let assign8700_e11473: f64 = if locals.var_cgsl_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign8700_e11473;
        locals.var_guard159_rv = 0.0;

        let (assign8710_e11477,) = {
    if (locals.var_guard159 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cgsl_i,)
    }
};
        locals.var_cgsl_i = assign8710_e11477;
        locals.var_cgsl_i_rv = 0.0;

        let assign8720_e11480: f64 = if locals.var_cgdl_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign8720_e11480;
        locals.var_guard160_rv = 0.0;

        let (assign8730_e11484,) = {
    if (locals.var_guard160 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cgdl_i,)
    }
};
        locals.var_cgdl_i = assign8730_e11484;
        locals.var_cgdl_i_rv = 0.0;

        let assign8740_e11487: f64 = if locals.var_cfs_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign8740_e11487;
        locals.var_guard161_rv = 0.0;

        let (assign8750_e11491,) = {
    if (locals.var_guard161 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cfs_i,)
    }
};
        locals.var_cfs_i = assign8750_e11491;
        locals.var_cfs_i_rv = 0.0;

        let assign8760_e11494: f64 = if locals.var_cfd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign8760_e11494;
        locals.var_guard162_rv = 0.0;

        let (assign8770_e11498,) = {
    if (locals.var_guard162 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cfd_i,)
    }
};
        locals.var_cfd_i = assign8770_e11498;
        locals.var_cfd_i_rv = 0.0;

        let assign8780_e11501: f64 = if locals.var_cgbl_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign8780_e11501;
        locals.var_guard163_rv = 0.0;

        let (assign8790_e11505,) = {
    if (locals.var_guard163 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cgbl_i,)
    }
};
        locals.var_cgbl_i = assign8790_e11505;
        locals.var_cgbl_i_rv = 0.0;

        let assign8800_e11508: f64 = if locals.var_ckappas_i <= 0.02 { 1.0 } else { 0.0 };
        locals.var_guard164 = assign8800_e11508;
        locals.var_guard164_rv = 0.0;

        let (assign8810_e11512,) = {
    if (locals.var_guard164 != 0.0) {
        (0.02,)
    } else {
        (locals.var_ckappas_i,)
    }
};
        locals.var_ckappas_i = assign8810_e11512;
        locals.var_ckappas_i_rv = 0.0;

        let assign8820_e11515: f64 = if locals.var_ckappad_i <= 0.02 { 1.0 } else { 0.0 };
        locals.var_guard165 = assign8820_e11515;
        locals.var_guard165_rv = 0.0;

        let (assign8830_e11519,) = {
    if (locals.var_guard165 != 0.0) {
        (0.02,)
    } else {
        (locals.var_ckappad_i,)
    }
};
        locals.var_ckappad_i = assign8830_e11519;
        locals.var_ckappad_i_rv = 0.0;

        let assign8840_e11522: f64 = if locals.var_ckappab_i <= 0.02 { 1.0 } else { 0.0 };
        locals.var_guard166 = assign8840_e11522;
        locals.var_guard166_rv = 0.0;

        let (assign8850_e11526,) = {
    if (locals.var_guard166 != 0.0) {
        (0.02,)
    } else {
        (locals.var_ckappab_i,)
    }
};
        locals.var_ckappab_i = assign8850_e11526;
        locals.var_ckappab_i_rv = 0.0;

        let assign8860_e11529: f64 = (-p.p4);
        let assign8860_e11530: f64 = if locals.var_deltaprsd_v < assign8860_e11529 { 1.0 } else { 0.0 };
        locals.var_guard167 = assign8860_e11530;
        locals.var_guard167_rv = 0.0;

        let (assign8870_e11534,) = {
    if (locals.var_guard167 != 0.0) {
        (0.0,)
    } else {
        (locals.var_deltaprsd_v,)
    }
};
        locals.var_deltaprsd_v = assign8870_e11534;
        locals.var_deltaprsd_v_rv = 0.0;

        let assign8880_e11537: f64 = if p.p57 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard168 = assign8880_e11537;
        locals.var_guard168_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign8890_e11544: f64 = if ((locals.var_dimension1_i < 1.0) || (locals.var_dimension1_i > 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard169 = assign8890_e11544;
        locals.var_guard169_rv = 0.0;

        let (assign8900_e11550,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard169 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_dimension1_i,)
    }
};
        locals.var_dimension1_i = assign8900_e11550;
        locals.var_dimension1_i_rv = 0.0;

        let assign8910_e11557: f64 = if ((locals.var_dimension2_i < 1.0) || (locals.var_dimension2_i > 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard170 = assign8910_e11557;
        locals.var_guard170_rv = 0.0;

        let (assign8920_e11563,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard170 != 0.0)) {
        (2.6,)
    } else {
        (locals.var_dimension2_i,)
    }
};
        locals.var_dimension2_i = assign8920_e11563;
        locals.var_dimension2_i_rv = 0.0;

        let assign8930_e11570: f64 = if ((locals.var_dimension3_i < 1.0) || (locals.var_dimension3_i > 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard171 = assign8930_e11570;
        locals.var_guard171_rv = 0.0;

        let (assign8940_e11576,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard171 != 0.0)) {
        (2.6,)
    } else {
        (locals.var_dimension3_i,)
    }
};
        locals.var_dimension3_i = assign8940_e11576;
        locals.var_dimension3_i_rv = 0.0;

        let assign8950_e11579: f64 = if locals.var_ssp1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard172 = assign8950_e11579;
        locals.var_guard172_rv = 0.0;

        let (assign8960_e11585,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard172 != 0.0)) {
        (14.0,)
    } else {
        (locals.var_ssp1_i,)
    }
};
        locals.var_ssp1_i = assign8960_e11585;
        locals.var_ssp1_i_rv = 0.0;

        let assign8970_e11588: f64 = if locals.var_ssp2_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard173 = assign8970_e11588;
        locals.var_guard173_rv = 0.0;

        let (assign8980_e11594,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard173 != 0.0)) {
        (24.0,)
    } else {
        (locals.var_ssp2_i,)
    }
};
        locals.var_ssp2_i = assign8980_e11594;
        locals.var_ssp2_i_rv = 0.0;

        let assign8990_e11597: f64 = if locals.var_ssp3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard174 = assign8990_e11597;
        locals.var_guard174_rv = 0.0;

        let (assign9000_e11603,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard174 != 0.0)) {
        (24.0,)
    } else {
        (locals.var_ssp3_i,)
    }
};
        locals.var_ssp3_i = assign9000_e11603;
        locals.var_ssp3_i_rv = 0.0;

        let assign9010_e11606: f64 = if locals.var_e2nom_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard175 = assign9010_e11606;
        locals.var_guard175_rv = 0.0;

        let (assign9020_e11612,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard175 != 0.0)) {
        (0.139,)
    } else {
        (locals.var_e2nom_i,)
    }
};
        locals.var_e2nom_i = assign9020_e11612;
        locals.var_e2nom_i_rv = 0.0;

        let assign9030_e11615: f64 = if locals.var_e3nom_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard176 = assign9030_e11615;
        locals.var_guard176_rv = 0.0;

        let (assign9040_e11621,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard176 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_e3nom_i,)
    }
};
        locals.var_e3nom_i = assign9040_e11621;
        locals.var_e3nom_i_rv = 0.0;

        let assign9050_e11624: f64 = if locals.var_mfq1nom_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard177 = assign9050_e11624;
        locals.var_guard177_rv = 0.0;

        let (assign9060_e11630,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard177 != 0.0)) {
        (11.2,)
    } else {
        (locals.var_mfq1nom_i,)
    }
};
        locals.var_mfq1nom_i = assign9060_e11630;
        locals.var_mfq1nom_i_rv = 0.0;

        let assign9070_e11633: f64 = if locals.var_mfq2nom_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard178 = assign9070_e11633;
        locals.var_guard178_rv = 0.0;

        let (assign9080_e11639,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard178 != 0.0)) {
        (8.02,)
    } else {
        (locals.var_mfq2nom_i,)
    }
};
        locals.var_mfq2nom_i = assign9080_e11639;
        locals.var_mfq2nom_i_rv = 0.0;

        let assign9090_e11642: f64 = if locals.var_mfq3nom_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard179 = assign9090_e11642;
        locals.var_guard179_rv = 0.0;

        let (assign9100_e11648,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard179 != 0.0)) {
        (6.18,)
    } else {
        (locals.var_mfq3nom_i,)
    }
};
        locals.var_mfq3nom_i = assign9100_e11648;
        locals.var_mfq3nom_i_rv = 0.0;

        let assign9110_e11655: f64 = if ((p.p74 != 0.0) && (p.p1791 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard180 = assign9110_e11655;
        locals.var_guard180_rv = 0.0;

        let assign9120_e11658: f64 = if p.p1795 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard181 = assign9120_e11658;
        locals.var_guard181_rv = 0.0;

        let (assign9130_e11668, assign9130_e11668_d_n0, assign9130_e11668_d_n2, assign9130_e11668_d_n3, assign9130_e11668_d_n4, assign9130_e11668_d_n5, assign9130_e11668_d_n6, assign9130_e11668_d_n7, assign9130_e11668_d_n8, assign9130_e11668_d_n9, assign9130_e11668_d_n10, assign9130_e11668_d_n11, assign9130_e11668_d_n13, assign9130_e11668_d_n14,) = {
    if ((locals.var_guard180 != 0.0) && (locals.var_guard181 != 0.0)) {
        let assign9130_e11665: f64 = (p.p59).powf(p.p1795);
        let assign9130_e11666: f64 = (p.p1793 * assign9130_e11665);
        (assign9130_e11666, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign9130_e11668;
        locals.var_t1_dn0 = assign9130_e11668_d_n0;
        locals.var_t1_dn2 = assign9130_e11668_d_n2;
        locals.var_t1_dn3 = assign9130_e11668_d_n3;
        locals.var_t1_dn4 = assign9130_e11668_d_n4;
        locals.var_t1_dn5 = assign9130_e11668_d_n5;
        locals.var_t1_dn6 = assign9130_e11668_d_n6;
        locals.var_t1_dn7 = assign9130_e11668_d_n7;
        locals.var_t1_dn8 = assign9130_e11668_d_n8;
        locals.var_t1_dn9 = assign9130_e11668_d_n9;
        locals.var_t1_dn10 = assign9130_e11668_d_n10;
        locals.var_t1_dn11 = assign9130_e11668_d_n11;
        locals.var_t1_dn13 = assign9130_e11668_d_n13;
        locals.var_t1_dn14 = assign9130_e11668_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign9140_e11675, assign9140_e11675_d_n0, assign9140_e11675_d_n2, assign9140_e11675_d_n3, assign9140_e11675_d_n4, assign9140_e11675_d_n5, assign9140_e11675_d_n6, assign9140_e11675_d_n7, assign9140_e11675_d_n8, assign9140_e11675_d_n9, assign9140_e11675_d_n10, assign9140_e11675_d_n11, assign9140_e11675_d_n13, assign9140_e11675_d_n14,) = {
    if ((locals.var_guard180 != 0.0) && (locals.var_guard181 == 0.0)) {
        (p.p1793, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign9140_e11675;
        locals.var_t1_dn0 = assign9140_e11675_d_n0;
        locals.var_t1_dn2 = assign9140_e11675_d_n2;
        locals.var_t1_dn3 = assign9140_e11675_d_n3;
        locals.var_t1_dn4 = assign9140_e11675_d_n4;
        locals.var_t1_dn5 = assign9140_e11675_d_n5;
        locals.var_t1_dn6 = assign9140_e11675_d_n6;
        locals.var_t1_dn7 = assign9140_e11675_d_n7;
        locals.var_t1_dn8 = assign9140_e11675_d_n8;
        locals.var_t1_dn9 = assign9140_e11675_d_n9;
        locals.var_t1_dn10 = assign9140_e11675_d_n10;
        locals.var_t1_dn11 = assign9140_e11675_d_n11;
        locals.var_t1_dn13 = assign9140_e11675_d_n13;
        locals.var_t1_dn14 = assign9140_e11675_d_n14;
        locals.var_t1_rv = 0.0;

        let assign9150_e11678: f64 = if p.p1794 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard182 = assign9150_e11678;
        locals.var_guard182_rv = 0.0;

        let (assign9160_e11690, assign9160_e11690_d_n0, assign9160_e11690_d_n2, assign9160_e11690_d_n3, assign9160_e11690_d_n4, assign9160_e11690_d_n5, assign9160_e11690_d_n6, assign9160_e11690_d_n7, assign9160_e11690_d_n8, assign9160_e11690_d_n9, assign9160_e11690_d_n10, assign9160_e11690_d_n11, assign9160_e11690_d_n13, assign9160_e11690_d_n14,) = {
    if ((locals.var_guard180 != 0.0) && (locals.var_guard182 != 0.0)) {
        let assign9160_e11684: f64 = (p.p1797 * p.p4);
        let assign9160_e11687: f64 = (locals.var_nfintotal).powf(p.p1794);
        let assign9160_e11688: f64 = (assign9160_e11684 * assign9160_e11687);
        (assign9160_e11688, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign9160_e11690;
        locals.var_t2_dn0 = assign9160_e11690_d_n0;
        locals.var_t2_dn2 = assign9160_e11690_d_n2;
        locals.var_t2_dn3 = assign9160_e11690_d_n3;
        locals.var_t2_dn4 = assign9160_e11690_d_n4;
        locals.var_t2_dn5 = assign9160_e11690_d_n5;
        locals.var_t2_dn6 = assign9160_e11690_d_n6;
        locals.var_t2_dn7 = assign9160_e11690_d_n7;
        locals.var_t2_dn8 = assign9160_e11690_d_n8;
        locals.var_t2_dn9 = assign9160_e11690_d_n9;
        locals.var_t2_dn10 = assign9160_e11690_d_n10;
        locals.var_t2_dn11 = assign9160_e11690_d_n11;
        locals.var_t2_dn13 = assign9160_e11690_d_n13;
        locals.var_t2_dn14 = assign9160_e11690_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign9170_e11699, assign9170_e11699_d_n0, assign9170_e11699_d_n2, assign9170_e11699_d_n3, assign9170_e11699_d_n4, assign9170_e11699_d_n5, assign9170_e11699_d_n6, assign9170_e11699_d_n7, assign9170_e11699_d_n8, assign9170_e11699_d_n9, assign9170_e11699_d_n10, assign9170_e11699_d_n11, assign9170_e11699_d_n13, assign9170_e11699_d_n14,) = {
    if ((locals.var_guard180 != 0.0) && (locals.var_guard182 == 0.0)) {
        let assign9170_e11697: f64 = (p.p1797 * p.p4);
        (assign9170_e11697, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign9170_e11699;
        locals.var_t2_dn0 = assign9170_e11699_d_n0;
        locals.var_t2_dn2 = assign9170_e11699_d_n2;
        locals.var_t2_dn3 = assign9170_e11699_d_n3;
        locals.var_t2_dn4 = assign9170_e11699_d_n4;
        locals.var_t2_dn5 = assign9170_e11699_d_n5;
        locals.var_t2_dn6 = assign9170_e11699_d_n6;
        locals.var_t2_dn7 = assign9170_e11699_d_n7;
        locals.var_t2_dn8 = assign9170_e11699_d_n8;
        locals.var_t2_dn9 = assign9170_e11699_d_n9;
        locals.var_t2_dn10 = assign9170_e11699_d_n10;
        locals.var_t2_dn11 = assign9170_e11699_d_n11;
        locals.var_t2_dn13 = assign9170_e11699_d_n13;
        locals.var_t2_dn14 = assign9170_e11699_d_n14;
        locals.var_t2_rv = 0.0;

        let assign9180_e11702: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard183 = assign9180_e11702;
        locals.var_guard183_rv = 0.0;

        let assign9190_e11705: f64 = if p.p1796 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard184 = assign9190_e11705;
        locals.var_guard184_rv = 0.0;

        let (assign9200_e11721, assign9200_e11721_d_n0, assign9200_e11721_d_n2, assign9200_e11721_d_n3, assign9200_e11721_d_n4, assign9200_e11721_d_n5, assign9200_e11721_d_n6, assign9200_e11721_d_n7, assign9200_e11721_d_n8, assign9200_e11721_d_n9, assign9200_e11721_d_n10, assign9200_e11721_d_n11, assign9200_e11721_d_n13, assign9200_e11721_d_n14,) = {
    if (((locals.var_guard180 != 0.0) && (locals.var_guard183 != 0.0)) && (locals.var_guard184 != 0.0)) {
        let assign9200_e11713: f64 = (p.p1798 * p.p59);
        let assign9200_e11715: f64 = (assign9200_e11713 * p.p43);
        let assign9200_e11718: f64 = (p.p56).powf(p.p1796);
        let assign9200_e11719: f64 = (assign9200_e11715 * assign9200_e11718);
        (assign9200_e11719, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9200_e11721;
        locals.var_t3_dn0 = assign9200_e11721_d_n0;
        locals.var_t3_dn2 = assign9200_e11721_d_n2;
        locals.var_t3_dn3 = assign9200_e11721_d_n3;
        locals.var_t3_dn4 = assign9200_e11721_d_n4;
        locals.var_t3_dn5 = assign9200_e11721_d_n5;
        locals.var_t3_dn6 = assign9200_e11721_d_n6;
        locals.var_t3_dn7 = assign9200_e11721_d_n7;
        locals.var_t3_dn8 = assign9200_e11721_d_n8;
        locals.var_t3_dn9 = assign9200_e11721_d_n9;
        locals.var_t3_dn10 = assign9200_e11721_d_n10;
        locals.var_t3_dn11 = assign9200_e11721_d_n11;
        locals.var_t3_dn13 = assign9200_e11721_d_n13;
        locals.var_t3_dn14 = assign9200_e11721_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9210_e11734, assign9210_e11734_d_n0, assign9210_e11734_d_n2, assign9210_e11734_d_n3, assign9210_e11734_d_n4, assign9210_e11734_d_n5, assign9210_e11734_d_n6, assign9210_e11734_d_n7, assign9210_e11734_d_n8, assign9210_e11734_d_n9, assign9210_e11734_d_n10, assign9210_e11734_d_n11, assign9210_e11734_d_n13, assign9210_e11734_d_n14,) = {
    if (((locals.var_guard180 != 0.0) && (locals.var_guard183 != 0.0)) && (locals.var_guard184 == 0.0)) {
        let assign9210_e11730: f64 = (p.p1798 * p.p59);
        let assign9210_e11732: f64 = (assign9210_e11730 * p.p43);
        (assign9210_e11732, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9210_e11734;
        locals.var_t3_dn0 = assign9210_e11734_d_n0;
        locals.var_t3_dn2 = assign9210_e11734_d_n2;
        locals.var_t3_dn3 = assign9210_e11734_d_n3;
        locals.var_t3_dn4 = assign9210_e11734_d_n4;
        locals.var_t3_dn5 = assign9210_e11734_d_n5;
        locals.var_t3_dn6 = assign9210_e11734_d_n6;
        locals.var_t3_dn7 = assign9210_e11734_d_n7;
        locals.var_t3_dn8 = assign9210_e11734_d_n8;
        locals.var_t3_dn9 = assign9210_e11734_d_n9;
        locals.var_t3_dn10 = assign9210_e11734_d_n10;
        locals.var_t3_dn11 = assign9210_e11734_d_n11;
        locals.var_t3_dn13 = assign9210_e11734_d_n13;
        locals.var_t3_dn14 = assign9210_e11734_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9220_e11741, assign9220_e11741_d_n0, assign9220_e11741_d_n2, assign9220_e11741_d_n3, assign9220_e11741_d_n4, assign9220_e11741_d_n5, assign9220_e11741_d_n6, assign9220_e11741_d_n7, assign9220_e11741_d_n8, assign9220_e11741_d_n9, assign9220_e11741_d_n10, assign9220_e11741_d_n11, assign9220_e11741_d_n13, assign9220_e11741_d_n14,) = {
    if ((locals.var_guard180 != 0.0) && (locals.var_guard183 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9220_e11741;
        locals.var_t3_dn0 = assign9220_e11741_d_n0;
        locals.var_t3_dn2 = assign9220_e11741_d_n2;
        locals.var_t3_dn3 = assign9220_e11741_d_n3;
        locals.var_t3_dn4 = assign9220_e11741_d_n4;
        locals.var_t3_dn5 = assign9220_e11741_d_n5;
        locals.var_t3_dn6 = assign9220_e11741_d_n6;
        locals.var_t3_dn7 = assign9220_e11741_d_n7;
        locals.var_t3_dn8 = assign9220_e11741_d_n8;
        locals.var_t3_dn9 = assign9220_e11741_d_n9;
        locals.var_t3_dn10 = assign9220_e11741_d_n10;
        locals.var_t3_dn11 = assign9220_e11741_d_n11;
        locals.var_t3_dn13 = assign9220_e11741_d_n13;
        locals.var_t3_dn14 = assign9220_e11741_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9240_e11761, assign9240_e11761_d_n0, assign9240_e11761_d_n2, assign9240_e11761_d_n3, assign9240_e11761_d_n4, assign9240_e11761_d_n5, assign9240_e11761_d_n6, assign9240_e11761_d_n7, assign9240_e11761_d_n8, assign9240_e11761_d_n9, assign9240_e11761_d_n10, assign9240_e11761_d_n11, assign9240_e11761_d_n13, assign9240_e11761_d_n14,) = {
    if (locals.var_guard180 != 0.0) {
        let assign9240_e11756: f64 = (locals.var_t1 + locals.var_t2);
        let assign9240_e11758: f64 = (assign9240_e11756 + locals.var_t3);
        let assign9240_e11759: f64 = (p.p1792 * assign9240_e11758);
        (assign9240_e11759, (p.p1792 * ((locals.var_t1_dn0 + locals.var_t2_dn0) + locals.var_t3_dn0)), (p.p1792 * ((locals.var_t1_dn2 + locals.var_t2_dn2) + locals.var_t3_dn2)), (p.p1792 * ((locals.var_t1_dn3 + locals.var_t2_dn3) + locals.var_t3_dn3)), (p.p1792 * ((locals.var_t1_dn4 + locals.var_t2_dn4) + locals.var_t3_dn4)), (p.p1792 * ((locals.var_t1_dn5 + locals.var_t2_dn5) + locals.var_t3_dn5)), (p.p1792 * ((locals.var_t1_dn6 + locals.var_t2_dn6) + locals.var_t3_dn6)), (p.p1792 * ((locals.var_t1_dn7 + locals.var_t2_dn7) + locals.var_t3_dn7)), (p.p1792 * ((locals.var_t1_dn8 + locals.var_t2_dn8) + locals.var_t3_dn8)), (p.p1792 * ((locals.var_t1_dn9 + locals.var_t2_dn9) + locals.var_t3_dn9)), (p.p1792 * ((locals.var_t1_dn10 + locals.var_t2_dn10) + locals.var_t3_dn10)), (p.p1792 * ((locals.var_t1_dn11 + locals.var_t2_dn11) + locals.var_t3_dn11)), (p.p1792 * ((locals.var_t1_dn13 + locals.var_t2_dn13) + locals.var_t3_dn13)), (p.p1792 * ((locals.var_t1_dn14 + locals.var_t2_dn14) + locals.var_t3_dn14)),)
    } else {
        (locals.var_cth, locals.var_cth_dn0, locals.var_cth_dn2, locals.var_cth_dn3, locals.var_cth_dn4, locals.var_cth_dn5, locals.var_cth_dn6, locals.var_cth_dn7, locals.var_cth_dn8, locals.var_cth_dn9, locals.var_cth_dn10, locals.var_cth_dn11, locals.var_cth_dn13, locals.var_cth_dn14,)
    }
};
        locals.var_cth = assign9240_e11761;
        locals.var_cth_dn0 = assign9240_e11761_d_n0;
        locals.var_cth_dn2 = assign9240_e11761_d_n2;
        locals.var_cth_dn3 = assign9240_e11761_d_n3;
        locals.var_cth_dn4 = assign9240_e11761_d_n4;
        locals.var_cth_dn5 = assign9240_e11761_d_n5;
        locals.var_cth_dn6 = assign9240_e11761_d_n6;
        locals.var_cth_dn7 = assign9240_e11761_d_n7;
        locals.var_cth_dn8 = assign9240_e11761_d_n8;
        locals.var_cth_dn9 = assign9240_e11761_d_n9;
        locals.var_cth_dn10 = assign9240_e11761_d_n10;
        locals.var_cth_dn11 = assign9240_e11761_d_n11;
        locals.var_cth_dn13 = assign9240_e11761_d_n13;
        locals.var_cth_dn14 = assign9240_e11761_d_n14;
        locals.var_cth_rv = 0.0;

        let assign9310_e11817: f64 = if p.p77 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard187 = assign9310_e11817;
        locals.var_guard187_rv = 0.0;

        let (assign9320_e11823, assign9320_e11823_d_n0, assign9320_e11823_d_n2, assign9320_e11823_d_n3, assign9320_e11823_d_n4, assign9320_e11823_d_n5, assign9320_e11823_d_n6, assign9320_e11823_d_n7, assign9320_e11823_d_n8, assign9320_e11823_d_n9, assign9320_e11823_d_n10, assign9320_e11823_d_n11, assign9320_e11823_d_n13, assign9320_e11823_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9320_e11821: f64 = (p.p1078 * p.p18);
        (assign9320_e11821, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsourcegeo, locals.var_rsourcegeo_dn0, locals.var_rsourcegeo_dn2, locals.var_rsourcegeo_dn3, locals.var_rsourcegeo_dn4, locals.var_rsourcegeo_dn5, locals.var_rsourcegeo_dn6, locals.var_rsourcegeo_dn7, locals.var_rsourcegeo_dn8, locals.var_rsourcegeo_dn9, locals.var_rsourcegeo_dn10, locals.var_rsourcegeo_dn11, locals.var_rsourcegeo_dn13, locals.var_rsourcegeo_dn14,)
    }
};
        locals.var_rsourcegeo = assign9320_e11823;
        locals.var_rsourcegeo_dn0 = assign9320_e11823_d_n0;
        locals.var_rsourcegeo_dn2 = assign9320_e11823_d_n2;
        locals.var_rsourcegeo_dn3 = assign9320_e11823_d_n3;
        locals.var_rsourcegeo_dn4 = assign9320_e11823_d_n4;
        locals.var_rsourcegeo_dn5 = assign9320_e11823_d_n5;
        locals.var_rsourcegeo_dn6 = assign9320_e11823_d_n6;
        locals.var_rsourcegeo_dn7 = assign9320_e11823_d_n7;
        locals.var_rsourcegeo_dn8 = assign9320_e11823_d_n8;
        locals.var_rsourcegeo_dn9 = assign9320_e11823_d_n9;
        locals.var_rsourcegeo_dn10 = assign9320_e11823_d_n10;
        locals.var_rsourcegeo_dn11 = assign9320_e11823_d_n11;
        locals.var_rsourcegeo_dn13 = assign9320_e11823_d_n13;
        locals.var_rsourcegeo_dn14 = assign9320_e11823_d_n14;
        locals.var_rsourcegeo_rv = 0.0;

        let (assign9330_e11829, assign9330_e11829_d_n0, assign9330_e11829_d_n2, assign9330_e11829_d_n3, assign9330_e11829_d_n4, assign9330_e11829_d_n5, assign9330_e11829_d_n6, assign9330_e11829_d_n7, assign9330_e11829_d_n8, assign9330_e11829_d_n9, assign9330_e11829_d_n10, assign9330_e11829_d_n11, assign9330_e11829_d_n13, assign9330_e11829_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9330_e11827: f64 = (p.p1079 * p.p19);
        (assign9330_e11827, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdraingeo, locals.var_rdraingeo_dn0, locals.var_rdraingeo_dn2, locals.var_rdraingeo_dn3, locals.var_rdraingeo_dn4, locals.var_rdraingeo_dn5, locals.var_rdraingeo_dn6, locals.var_rdraingeo_dn7, locals.var_rdraingeo_dn8, locals.var_rdraingeo_dn9, locals.var_rdraingeo_dn10, locals.var_rdraingeo_dn11, locals.var_rdraingeo_dn13, locals.var_rdraingeo_dn14,)
    }
};
        locals.var_rdraingeo = assign9330_e11829;
        locals.var_rdraingeo_dn0 = assign9330_e11829_d_n0;
        locals.var_rdraingeo_dn2 = assign9330_e11829_d_n2;
        locals.var_rdraingeo_dn3 = assign9330_e11829_d_n3;
        locals.var_rdraingeo_dn4 = assign9330_e11829_d_n4;
        locals.var_rdraingeo_dn5 = assign9330_e11829_d_n5;
        locals.var_rdraingeo_dn6 = assign9330_e11829_d_n6;
        locals.var_rdraingeo_dn7 = assign9330_e11829_d_n7;
        locals.var_rdraingeo_dn8 = assign9330_e11829_d_n8;
        locals.var_rdraingeo_dn9 = assign9330_e11829_d_n9;
        locals.var_rdraingeo_dn10 = assign9330_e11829_d_n10;
        locals.var_rdraingeo_dn11 = assign9330_e11829_d_n11;
        locals.var_rdraingeo_dn13 = assign9330_e11829_d_n13;
        locals.var_rdraingeo_dn14 = assign9330_e11829_d_n14;
        locals.var_rdraingeo_rv = 0.0;

        let assign9340_e11832: f64 = if p.p1080 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard188 = assign9340_e11832;
        locals.var_guard188_rv = 0.0;

        let (assign9350_e11851,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard188 != 0.0)) {
        let assign9350_e11839: f64 = (p.p4 * p.p92);
        let assign9350_e11843: f64 = (p.p4 - p.p3);
        let assign9350_e11845: f64 = (assign9350_e11843 * p.p1084);
        let assign9350_e11846: f64 = (p.p3 + assign9350_e11845);
        let assign9350_e11848: f64 = (assign9350_e11846 * p.p1080);
        let assign9350_e11849: f64 = (assign9350_e11839 + assign9350_e11848);
        (assign9350_e11849,)
    } else {
        (locals.var_arsd,)
    }
};
        locals.var_arsd = assign9350_e11851;
        locals.var_arsd_rv = 0.0;

        let (assign9360_e11865,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard188 == 0.0)) {
        let assign9360_e11861: f64 = (p.p92 + p.p1080);
        let assign9360_e11862: f64 = (1e-9_f64).max(assign9360_e11861);
        let assign9360_e11863: f64 = (p.p4 * assign9360_e11862);
        (assign9360_e11863,)
    } else {
        (locals.var_arsd,)
    }
};
        locals.var_arsd = assign9360_e11865;
        locals.var_arsd_rv = 0.0;

        let (assign9370_e11872,) = {
    if (locals.var_guard187 == 0.0) {
        let assign9370_e11870: f64 = (p.p4 + locals.var_deltaprsd_v);
        (assign9370_e11870,)
    } else {
        (locals.var_prsd,)
    }
};
        locals.var_prsd = assign9370_e11872;
        locals.var_prsd_rv = 0.0;

        let assign9380_e11874: f64 = if param_given[1083] { 1.0 } else { 0.0 };
        locals.var_guard189 = assign9380_e11874;
        locals.var_guard189_rv = 0.0;

        let (assign9390_e11881, assign9390_e11881_d_n0, assign9390_e11881_d_n2, assign9390_e11881_d_n3, assign9390_e11881_d_n4, assign9390_e11881_d_n5, assign9390_e11881_d_n6, assign9390_e11881_d_n7, assign9390_e11881_d_n8, assign9390_e11881_d_n9, assign9390_e11881_d_n10, assign9390_e11881_d_n11, assign9390_e11881_d_n13, assign9390_e11881_d_n14,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard189 != 0.0)) {
        (p.p1083, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rhorsd_v, locals.var_rhorsd_v_dn0, locals.var_rhorsd_v_dn2, locals.var_rhorsd_v_dn3, locals.var_rhorsd_v_dn4, locals.var_rhorsd_v_dn5, locals.var_rhorsd_v_dn6, locals.var_rhorsd_v_dn7, locals.var_rhorsd_v_dn8, locals.var_rhorsd_v_dn9, locals.var_rhorsd_v_dn10, locals.var_rhorsd_v_dn11, locals.var_rhorsd_v_dn13, locals.var_rhorsd_v_dn14,)
    }
};
        locals.var_rhorsd_v = assign9390_e11881;
        locals.var_rhorsd_v_dn0 = assign9390_e11881_d_n0;
        locals.var_rhorsd_v_dn2 = assign9390_e11881_d_n2;
        locals.var_rhorsd_v_dn3 = assign9390_e11881_d_n3;
        locals.var_rhorsd_v_dn4 = assign9390_e11881_d_n4;
        locals.var_rhorsd_v_dn5 = assign9390_e11881_d_n5;
        locals.var_rhorsd_v_dn6 = assign9390_e11881_d_n6;
        locals.var_rhorsd_v_dn7 = assign9390_e11881_d_n7;
        locals.var_rhorsd_v_dn8 = assign9390_e11881_d_n8;
        locals.var_rhorsd_v_dn9 = assign9390_e11881_d_n9;
        locals.var_rhorsd_v_dn10 = assign9390_e11881_d_n10;
        locals.var_rhorsd_v_dn11 = assign9390_e11881_d_n11;
        locals.var_rhorsd_v_dn13 = assign9390_e11881_d_n13;
        locals.var_rhorsd_v_dn14 = assign9390_e11881_d_n14;
        locals.var_rhorsd_v_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9400_e11894,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard189 == 0.0)) {
        let (assign9400_e11892,) = {
            if (p.p60 == 1.0) {
                (1417.0,)
            } else {
                (470.5,)
            }
        };
        (assign9400_e11892,)
    } else {
        (locals.var_mu_max,)
    }
};
        locals.var_mu_max = assign9400_e11894;
        locals.var_mu_max_rv = 0.0;

        let assign9410_e11897: f64 = if p.p60 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard190 = assign9410_e11897;
        locals.var_guard190_rv = 0.0;

        let (assign9420_e11911, assign9420_e11911_d_n0, assign9420_e11911_d_n2, assign9420_e11911_d_n3, assign9420_e11911_d_n4, assign9420_e11911_d_n5, assign9420_e11911_d_n6, assign9420_e11911_d_n7, assign9420_e11911_d_n8, assign9420_e11911_d_n9, assign9420_e11911_d_n10, assign9420_e11911_d_n11, assign9420_e11911_d_n13, assign9420_e11911_d_n14,) = {
    if (((locals.var_guard187 == 0.0) && (locals.var_guard189 == 0.0)) && (locals.var_guard190 != 0.0)) {
        let assign9420_e11907: f64 = (p.p97 / 9.68e22);
        let assign9420_e11909: f64 = (assign9420_e11907).powf(0.68);
        (assign9420_e11909, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign9420_e11911;
        locals.var_t0_dn0 = assign9420_e11911_d_n0;
        locals.var_t0_dn2 = assign9420_e11911_d_n2;
        locals.var_t0_dn3 = assign9420_e11911_d_n3;
        locals.var_t0_dn4 = assign9420_e11911_d_n4;
        locals.var_t0_dn5 = assign9420_e11911_d_n5;
        locals.var_t0_dn6 = assign9420_e11911_d_n6;
        locals.var_t0_dn7 = assign9420_e11911_d_n7;
        locals.var_t0_dn8 = assign9420_e11911_d_n8;
        locals.var_t0_dn9 = assign9420_e11911_d_n9;
        locals.var_t0_dn10 = assign9420_e11911_d_n10;
        locals.var_t0_dn11 = assign9420_e11911_d_n11;
        locals.var_t0_dn13 = assign9420_e11911_d_n13;
        locals.var_t0_dn14 = assign9420_e11911_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign9430_e11923, assign9430_e11923_d_n0, assign9430_e11923_d_n2, assign9430_e11923_d_n3, assign9430_e11923_d_n4, assign9430_e11923_d_n5, assign9430_e11923_d_n6, assign9430_e11923_d_n7, assign9430_e11923_d_n8, assign9430_e11923_d_n9, assign9430_e11923_d_n10, assign9430_e11923_d_n11, assign9430_e11923_d_n13, assign9430_e11923_d_n14,) = {
    if (((locals.var_guard187 == 0.0) && (locals.var_guard189 == 0.0)) && (locals.var_guard190 != 0.0)) {
        let assign9430_e11921: f64 = (3.43e26 / p.p97);
        (assign9430_e11921, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign9430_e11923;
        locals.var_t1_dn0 = assign9430_e11923_d_n0;
        locals.var_t1_dn2 = assign9430_e11923_d_n2;
        locals.var_t1_dn3 = assign9430_e11923_d_n3;
        locals.var_t1_dn4 = assign9430_e11923_d_n4;
        locals.var_t1_dn5 = assign9430_e11923_d_n5;
        locals.var_t1_dn6 = assign9430_e11923_d_n6;
        locals.var_t1_dn7 = assign9430_e11923_d_n7;
        locals.var_t1_dn8 = assign9430_e11923_d_n8;
        locals.var_t1_dn9 = assign9430_e11923_d_n9;
        locals.var_t1_dn10 = assign9430_e11923_d_n10;
        locals.var_t1_dn11 = assign9430_e11923_d_n11;
        locals.var_t1_dn13 = assign9430_e11923_d_n13;
        locals.var_t1_dn14 = assign9430_e11923_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign9440_e11951, assign9440_e11951_d_n0, assign9440_e11951_d_n2, assign9440_e11951_d_n3, assign9440_e11951_d_n4, assign9440_e11951_d_n5, assign9440_e11951_d_n6, assign9440_e11951_d_n7, assign9440_e11951_d_n8, assign9440_e11951_d_n9, assign9440_e11951_d_n10, assign9440_e11951_d_n11, assign9440_e11951_d_n13, assign9440_e11951_d_n14,) = {
    if (((locals.var_guard187 == 0.0) && (locals.var_guard189 == 0.0)) && (locals.var_guard190 != 0.0)) {
        let assign9440_e11934: f64 = (locals.var_mu_max - 52.2);
        let assign9440_e11937: f64 = (1.0 + locals.var_t0);
        let assign9440_e11938: f64 = (assign9440_e11934 / assign9440_e11937);
        let assign9440_e11939: f64 = (52.2 + assign9440_e11938);
        let assign9440_e11944: f64 = (locals.var_t1 * locals.var_t1);
        let assign9440_e11945: f64 = (1.0 + assign9440_e11944);
        let assign9440_e11946: f64 = (43.4 / assign9440_e11945);
        let assign9440_e11947: f64 = (assign9440_e11939 - assign9440_e11946);
        let assign9440_e11949: f64 = (assign9440_e11947 * 0.0001);
        (assign9440_e11949, (((-((assign9440_e11934 * locals.var_t0_dn0) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * locals.var_t0_dn2) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * locals.var_t0_dn3) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * locals.var_t0_dn4) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * locals.var_t0_dn5) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * locals.var_t0_dn6) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * locals.var_t0_dn7) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * locals.var_t0_dn8) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * locals.var_t0_dn9) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * locals.var_t0_dn10) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * locals.var_t0_dn11) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * locals.var_t0_dn13) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001), (((-((assign9440_e11934 * locals.var_t0_dn14) / (assign9440_e11937 * assign9440_e11937))) - (-((43.4 * ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14))) / (assign9440_e11945 * assign9440_e11945)))) * 0.0001),)
    } else {
        (locals.var_mu_rsd, locals.var_mu_rsd_dn0, locals.var_mu_rsd_dn2, locals.var_mu_rsd_dn3, locals.var_mu_rsd_dn4, locals.var_mu_rsd_dn5, locals.var_mu_rsd_dn6, locals.var_mu_rsd_dn7, locals.var_mu_rsd_dn8, locals.var_mu_rsd_dn9, locals.var_mu_rsd_dn10, locals.var_mu_rsd_dn11, locals.var_mu_rsd_dn13, locals.var_mu_rsd_dn14,)
    }
};
        locals.var_mu_rsd = assign9440_e11951;
        locals.var_mu_rsd_dn0 = assign9440_e11951_d_n0;
        locals.var_mu_rsd_dn2 = assign9440_e11951_d_n2;
        locals.var_mu_rsd_dn3 = assign9440_e11951_d_n3;
        locals.var_mu_rsd_dn4 = assign9440_e11951_d_n4;
        locals.var_mu_rsd_dn5 = assign9440_e11951_d_n5;
        locals.var_mu_rsd_dn6 = assign9440_e11951_d_n6;
        locals.var_mu_rsd_dn7 = assign9440_e11951_d_n7;
        locals.var_mu_rsd_dn8 = assign9440_e11951_d_n8;
        locals.var_mu_rsd_dn9 = assign9440_e11951_d_n9;
        locals.var_mu_rsd_dn10 = assign9440_e11951_d_n10;
        locals.var_mu_rsd_dn11 = assign9440_e11951_d_n11;
        locals.var_mu_rsd_dn13 = assign9440_e11951_d_n13;
        locals.var_mu_rsd_dn14 = assign9440_e11951_d_n14;
        locals.var_mu_rsd_rv = 0.0;

        let (assign9450_e11966, assign9450_e11966_d_n0, assign9450_e11966_d_n2, assign9450_e11966_d_n3, assign9450_e11966_d_n4, assign9450_e11966_d_n5, assign9450_e11966_d_n6, assign9450_e11966_d_n7, assign9450_e11966_d_n8, assign9450_e11966_d_n9, assign9450_e11966_d_n10, assign9450_e11966_d_n11, assign9450_e11966_d_n13, assign9450_e11966_d_n14,) = {
    if (((locals.var_guard187 == 0.0) && (locals.var_guard189 == 0.0)) && (locals.var_guard190 == 0.0)) {
        let assign9450_e11962: f64 = (p.p97 / 2.23e22);
        let assign9450_e11964: f64 = (assign9450_e11962).powf(0.719);
        (assign9450_e11964, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign9450_e11966;
        locals.var_t0_dn0 = assign9450_e11966_d_n0;
        locals.var_t0_dn2 = assign9450_e11966_d_n2;
        locals.var_t0_dn3 = assign9450_e11966_d_n3;
        locals.var_t0_dn4 = assign9450_e11966_d_n4;
        locals.var_t0_dn5 = assign9450_e11966_d_n5;
        locals.var_t0_dn6 = assign9450_e11966_d_n6;
        locals.var_t0_dn7 = assign9450_e11966_d_n7;
        locals.var_t0_dn8 = assign9450_e11966_d_n8;
        locals.var_t0_dn9 = assign9450_e11966_d_n9;
        locals.var_t0_dn10 = assign9450_e11966_d_n10;
        locals.var_t0_dn11 = assign9450_e11966_d_n11;
        locals.var_t0_dn13 = assign9450_e11966_d_n13;
        locals.var_t0_dn14 = assign9450_e11966_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign9460_e11979, assign9460_e11979_d_n0, assign9460_e11979_d_n2, assign9460_e11979_d_n3, assign9460_e11979_d_n4, assign9460_e11979_d_n5, assign9460_e11979_d_n6, assign9460_e11979_d_n7, assign9460_e11979_d_n8, assign9460_e11979_d_n9, assign9460_e11979_d_n10, assign9460_e11979_d_n11, assign9460_e11979_d_n13, assign9460_e11979_d_n14,) = {
    if (((locals.var_guard187 == 0.0) && (locals.var_guard189 == 0.0)) && (locals.var_guard190 == 0.0)) {
        let assign9460_e11977: f64 = (6.1e26 / p.p97);
        (assign9460_e11977, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign9460_e11979;
        locals.var_t1_dn0 = assign9460_e11979_d_n0;
        locals.var_t1_dn2 = assign9460_e11979_d_n2;
        locals.var_t1_dn3 = assign9460_e11979_d_n3;
        locals.var_t1_dn4 = assign9460_e11979_d_n4;
        locals.var_t1_dn5 = assign9460_e11979_d_n5;
        locals.var_t1_dn6 = assign9460_e11979_d_n6;
        locals.var_t1_dn7 = assign9460_e11979_d_n7;
        locals.var_t1_dn8 = assign9460_e11979_d_n8;
        locals.var_t1_dn9 = assign9460_e11979_d_n9;
        locals.var_t1_dn10 = assign9460_e11979_d_n10;
        locals.var_t1_dn11 = assign9460_e11979_d_n11;
        locals.var_t1_dn13 = assign9460_e11979_d_n13;
        locals.var_t1_dn14 = assign9460_e11979_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign9470_e12008, assign9470_e12008_d_n0, assign9470_e12008_d_n2, assign9470_e12008_d_n3, assign9470_e12008_d_n4, assign9470_e12008_d_n5, assign9470_e12008_d_n6, assign9470_e12008_d_n7, assign9470_e12008_d_n8, assign9470_e12008_d_n9, assign9470_e12008_d_n10, assign9470_e12008_d_n11, assign9470_e12008_d_n13, assign9470_e12008_d_n14,) = {
    if (((locals.var_guard187 == 0.0) && (locals.var_guard189 == 0.0)) && (locals.var_guard190 == 0.0)) {
        let assign9470_e11991: f64 = (locals.var_mu_max - 44.9);
        let assign9470_e11994: f64 = (1.0 + locals.var_t0);
        let assign9470_e11995: f64 = (assign9470_e11991 / assign9470_e11994);
        let assign9470_e11996: f64 = (44.9 + assign9470_e11995);
        let assign9470_e12001: f64 = (locals.var_t1 * locals.var_t1);
        let assign9470_e12002: f64 = (1.0 + assign9470_e12001);
        let assign9470_e12003: f64 = (29.0 / assign9470_e12002);
        let assign9470_e12004: f64 = (assign9470_e11996 - assign9470_e12003);
        let assign9470_e12006: f64 = (assign9470_e12004 * 0.0001);
        (assign9470_e12006, (((-((assign9470_e11991 * locals.var_t0_dn0) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * locals.var_t0_dn2) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * locals.var_t0_dn3) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * locals.var_t0_dn4) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * locals.var_t0_dn5) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * locals.var_t0_dn6) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * locals.var_t0_dn7) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * locals.var_t0_dn8) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * locals.var_t0_dn9) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * locals.var_t0_dn10) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * locals.var_t0_dn11) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * locals.var_t0_dn13) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001), (((-((assign9470_e11991 * locals.var_t0_dn14) / (assign9470_e11994 * assign9470_e11994))) - (-((29.0 * ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14))) / (assign9470_e12002 * assign9470_e12002)))) * 0.0001),)
    } else {
        (locals.var_mu_rsd, locals.var_mu_rsd_dn0, locals.var_mu_rsd_dn2, locals.var_mu_rsd_dn3, locals.var_mu_rsd_dn4, locals.var_mu_rsd_dn5, locals.var_mu_rsd_dn6, locals.var_mu_rsd_dn7, locals.var_mu_rsd_dn8, locals.var_mu_rsd_dn9, locals.var_mu_rsd_dn10, locals.var_mu_rsd_dn11, locals.var_mu_rsd_dn13, locals.var_mu_rsd_dn14,)
    }
};
        locals.var_mu_rsd = assign9470_e12008;
        locals.var_mu_rsd_dn0 = assign9470_e12008_d_n0;
        locals.var_mu_rsd_dn2 = assign9470_e12008_d_n2;
        locals.var_mu_rsd_dn3 = assign9470_e12008_d_n3;
        locals.var_mu_rsd_dn4 = assign9470_e12008_d_n4;
        locals.var_mu_rsd_dn5 = assign9470_e12008_d_n5;
        locals.var_mu_rsd_dn6 = assign9470_e12008_d_n6;
        locals.var_mu_rsd_dn7 = assign9470_e12008_d_n7;
        locals.var_mu_rsd_dn8 = assign9470_e12008_d_n8;
        locals.var_mu_rsd_dn9 = assign9470_e12008_d_n9;
        locals.var_mu_rsd_dn10 = assign9470_e12008_d_n10;
        locals.var_mu_rsd_dn11 = assign9470_e12008_d_n11;
        locals.var_mu_rsd_dn13 = assign9470_e12008_d_n13;
        locals.var_mu_rsd_dn14 = assign9470_e12008_d_n14;
        locals.var_mu_rsd_rv = 0.0;

        let (assign9480_e12022, assign9480_e12022_d_n0, assign9480_e12022_d_n2, assign9480_e12022_d_n3, assign9480_e12022_d_n4, assign9480_e12022_d_n5, assign9480_e12022_d_n6, assign9480_e12022_d_n7, assign9480_e12022_d_n8, assign9480_e12022_d_n9, assign9480_e12022_d_n10, assign9480_e12022_d_n11, assign9480_e12022_d_n13, assign9480_e12022_d_n14,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard189 == 0.0)) {
        let assign9480_e12017: f64 = (1.60219e-19 * p.p97);
        let assign9480_e12019: f64 = (assign9480_e12017 * locals.var_mu_rsd);
        let assign9480_e12020: f64 = (1.0 / assign9480_e12019);
        (assign9480_e12020, (-((assign9480_e12017 * locals.var_mu_rsd_dn0) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * locals.var_mu_rsd_dn2) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * locals.var_mu_rsd_dn3) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * locals.var_mu_rsd_dn4) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * locals.var_mu_rsd_dn5) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * locals.var_mu_rsd_dn6) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * locals.var_mu_rsd_dn7) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * locals.var_mu_rsd_dn8) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * locals.var_mu_rsd_dn9) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * locals.var_mu_rsd_dn10) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * locals.var_mu_rsd_dn11) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * locals.var_mu_rsd_dn13) / (assign9480_e12019 * assign9480_e12019))), (-((assign9480_e12017 * locals.var_mu_rsd_dn14) / (assign9480_e12019 * assign9480_e12019))),)
    } else {
        (locals.var_rhorsd_v, locals.var_rhorsd_v_dn0, locals.var_rhorsd_v_dn2, locals.var_rhorsd_v_dn3, locals.var_rhorsd_v_dn4, locals.var_rhorsd_v_dn5, locals.var_rhorsd_v_dn6, locals.var_rhorsd_v_dn7, locals.var_rhorsd_v_dn8, locals.var_rhorsd_v_dn9, locals.var_rhorsd_v_dn10, locals.var_rhorsd_v_dn11, locals.var_rhorsd_v_dn13, locals.var_rhorsd_v_dn14,)
    }
};
        locals.var_rhorsd_v = assign9480_e12022;
        locals.var_rhorsd_v_dn0 = assign9480_e12022_d_n0;
        locals.var_rhorsd_v_dn2 = assign9480_e12022_d_n2;
        locals.var_rhorsd_v_dn3 = assign9480_e12022_d_n3;
        locals.var_rhorsd_v_dn4 = assign9480_e12022_d_n4;
        locals.var_rhorsd_v_dn5 = assign9480_e12022_d_n5;
        locals.var_rhorsd_v_dn6 = assign9480_e12022_d_n6;
        locals.var_rhorsd_v_dn7 = assign9480_e12022_d_n7;
        locals.var_rhorsd_v_dn8 = assign9480_e12022_d_n8;
        locals.var_rhorsd_v_dn9 = assign9480_e12022_d_n9;
        locals.var_rhorsd_v_dn10 = assign9480_e12022_d_n10;
        locals.var_rhorsd_v_dn11 = assign9480_e12022_d_n11;
        locals.var_rhorsd_v_dn13 = assign9480_e12022_d_n13;
        locals.var_rhorsd_v_dn14 = assign9480_e12022_d_n14;
        locals.var_rhorsd_v_rv = 0.0;

        let (assign9490_e12031,) = {
    if (locals.var_guard187 == 0.0) {
        let assign9490_e12027: f64 = (55.0 * 3.141592653589793);
        let assign9490_e12029: f64 = (assign9490_e12027 / 180.0);
        (assign9490_e12029,)
    } else {
        (locals.var_thetarsp,)
    }
};
        locals.var_thetarsp = assign9490_e12031;
        locals.var_thetarsp_rv = 0.0;

        let (assign9500_e12046,) = {
    if (locals.var_guard187 == 0.0) {
        let assign9500_e12040: f64 = (0.0_f64).min(p.p1080);
        let assign9500_e12041: f64 = (p.p92 + assign9500_e12040);
        let assign9500_e12042: f64 = (p.p3 * assign9500_e12041);
        let assign9500_e12043: f64 = (1e-18_f64).max(assign9500_e12042);
        let assign9500_e12044: f64 = (locals.var_arsd).min(assign9500_e12043);
        (assign9500_e12044,)
    } else {
        (locals.var_afin,)
    }
};
        locals.var_afin = assign9500_e12046;
        locals.var_afin_rv = 0.0;

        let (assign9510_e12076, assign9510_e12076_d_n0, assign9510_e12076_d_n2, assign9510_e12076_d_n3, assign9510_e12076_d_n4, assign9510_e12076_d_n5, assign9510_e12076_d_n6, assign9510_e12076_d_n7, assign9510_e12076_d_n8, assign9510_e12076_d_n9, assign9510_e12076_d_n10, assign9510_e12076_d_n11, assign9510_e12076_d_n13, assign9510_e12076_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        let assign9510_e12051: f64 = (locals.var_thetarsp).tan();
        let assign9510_e12052: f64 = (locals.var_rhorsd_v / assign9510_e12051);
        let assign9510_e12054: f64 = (3.141592653589793_f64).sqrt();
        let assign9510_e12056: f64 = (assign9510_e12054 * p.p5);
        let assign9510_e12057: f64 = (assign9510_e12052 / assign9510_e12056);
        let assign9510_e12060: f64 = (locals.var_afin).sqrt();
        let assign9510_e12061: f64 = (1.0 / assign9510_e12060);
        let assign9510_e12064: f64 = (locals.var_arsd).sqrt();
        let assign9510_e12065: f64 = (2.0 / assign9510_e12064);
        let assign9510_e12066: f64 = (assign9510_e12061 - assign9510_e12065);
        let assign9510_e12070: f64 = (locals.var_arsd * locals.var_arsd);
        let assign9510_e12071: f64 = (locals.var_afin / assign9510_e12070);
        let assign9510_e12072: f64 = (assign9510_e12071).sqrt();
        let assign9510_e12073: f64 = (assign9510_e12066 + assign9510_e12072);
        let assign9510_e12074: f64 = (assign9510_e12057 * assign9510_e12073);
        (assign9510_e12074, (((locals.var_rhorsd_v_dn0 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((locals.var_rhorsd_v_dn2 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((locals.var_rhorsd_v_dn3 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((locals.var_rhorsd_v_dn4 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((locals.var_rhorsd_v_dn5 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((locals.var_rhorsd_v_dn6 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((locals.var_rhorsd_v_dn7 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((locals.var_rhorsd_v_dn8 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((locals.var_rhorsd_v_dn9 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((locals.var_rhorsd_v_dn10 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((locals.var_rhorsd_v_dn11 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((locals.var_rhorsd_v_dn13 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073), (((locals.var_rhorsd_v_dn14 / assign9510_e12051) / assign9510_e12056) * assign9510_e12073),)
    } else {
        (locals.var_rsp, locals.var_rsp_dn0, locals.var_rsp_dn2, locals.var_rsp_dn3, locals.var_rsp_dn4, locals.var_rsp_dn5, locals.var_rsp_dn6, locals.var_rsp_dn7, locals.var_rsp_dn8, locals.var_rsp_dn9, locals.var_rsp_dn10, locals.var_rsp_dn11, locals.var_rsp_dn13, locals.var_rsp_dn14,)
    }
};
        locals.var_rsp = assign9510_e12076;
        locals.var_rsp_dn0 = assign9510_e12076_d_n0;
        locals.var_rsp_dn2 = assign9510_e12076_d_n2;
        locals.var_rsp_dn3 = assign9510_e12076_d_n3;
        locals.var_rsp_dn4 = assign9510_e12076_d_n4;
        locals.var_rsp_dn5 = assign9510_e12076_d_n5;
        locals.var_rsp_dn6 = assign9510_e12076_d_n6;
        locals.var_rsp_dn7 = assign9510_e12076_d_n7;
        locals.var_rsp_dn8 = assign9510_e12076_d_n8;
        locals.var_rsp_dn9 = assign9510_e12076_d_n9;
        locals.var_rsp_dn10 = assign9510_e12076_d_n10;
        locals.var_rsp_dn11 = assign9510_e12076_d_n11;
        locals.var_rsp_dn13 = assign9510_e12076_d_n13;
        locals.var_rsp_dn14 = assign9510_e12076_d_n14;
        locals.var_rsp_rv = 0.0;

        let (assign9520_e12085,) = {
    if (locals.var_guard187 == 0.0) {
        let assign9520_e12081: f64 = (locals.var_arsd * p.p5);
        let assign9520_e12083: f64 = (assign9520_e12081 + p.p1092);
        (assign9520_e12083,)
    } else {
        (locals.var_arsd_total,)
    }
};
        locals.var_arsd_total = assign9520_e12085;
        locals.var_arsd_total_rv = 0.0;

        let (assign9530_e12094,) = {
    if (locals.var_guard187 == 0.0) {
        let assign9530_e12090: f64 = (locals.var_prsd * p.p5);
        let assign9530_e12092: f64 = (assign9530_e12090 + p.p1093);
        (assign9530_e12092,)
    } else {
        (locals.var_prsd_total,)
    }
};
        locals.var_prsd_total = assign9530_e12094;
        locals.var_prsd_total_rv = 0.0;

        let (assign9540_e12106, assign9540_e12106_d_n0, assign9540_e12106_d_n2, assign9540_e12106_d_n3, assign9540_e12106_d_n4, assign9540_e12106_d_n5, assign9540_e12106_d_n6, assign9540_e12106_d_n7, assign9540_e12106_d_n8, assign9540_e12106_d_n9, assign9540_e12106_d_n10, assign9540_e12106_d_n11, assign9540_e12106_d_n13, assign9540_e12106_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        let assign9540_e12099: f64 = (p.p1082 * locals.var_arsd_total);
        let assign9540_e12102: f64 = (locals.var_rhorsd_v * locals.var_prsd_total);
        let assign9540_e12103: f64 = (assign9540_e12099 / assign9540_e12102);
        let assign9540_e12104: f64 = (assign9540_e12103).sqrt();
        (assign9540_e12104, ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn0 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn2 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn3 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn4 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn5 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn6 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn7 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn8 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn9 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn10 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn11 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn13 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)), ((-((assign9540_e12099 * (locals.var_rhorsd_v_dn14 * locals.var_prsd_total)) / (assign9540_e12102 * assign9540_e12102))) / (2.0 * assign9540_e12104)),)
    } else {
        (locals.var_lt, locals.var_lt_dn0, locals.var_lt_dn2, locals.var_lt_dn3, locals.var_lt_dn4, locals.var_lt_dn5, locals.var_lt_dn6, locals.var_lt_dn7, locals.var_lt_dn8, locals.var_lt_dn9, locals.var_lt_dn10, locals.var_lt_dn11, locals.var_lt_dn13, locals.var_lt_dn14,)
    }
};
        locals.var_lt = assign9540_e12106;
        locals.var_lt_dn0 = assign9540_e12106_d_n0;
        locals.var_lt_dn2 = assign9540_e12106_d_n2;
        locals.var_lt_dn3 = assign9540_e12106_d_n3;
        locals.var_lt_dn4 = assign9540_e12106_d_n4;
        locals.var_lt_dn5 = assign9540_e12106_d_n5;
        locals.var_lt_dn6 = assign9540_e12106_d_n6;
        locals.var_lt_dn7 = assign9540_e12106_d_n7;
        locals.var_lt_dn8 = assign9540_e12106_d_n8;
        locals.var_lt_dn9 = assign9540_e12106_d_n9;
        locals.var_lt_dn10 = assign9540_e12106_d_n10;
        locals.var_lt_dn11 = assign9540_e12106_d_n11;
        locals.var_lt_dn13 = assign9540_e12106_d_n13;
        locals.var_lt_dn14 = assign9540_e12106_d_n14;
        locals.var_lt_rv = 0.0;

        let (assign9550_e12113, assign9550_e12113_d_n0, assign9550_e12113_d_n2, assign9550_e12113_d_n3, assign9550_e12113_d_n4, assign9550_e12113_d_n5, assign9550_e12113_d_n6, assign9550_e12113_d_n7, assign9550_e12113_d_n8, assign9550_e12113_d_n9, assign9550_e12113_d_n10, assign9550_e12113_d_n11, assign9550_e12113_d_n13, assign9550_e12113_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        let assign9550_e12111: f64 = (p.p20 / locals.var_lt);
        (assign9550_e12111, (-((p.p20 * locals.var_lt_dn0) / (locals.var_lt * locals.var_lt))), (-((p.p20 * locals.var_lt_dn2) / (locals.var_lt * locals.var_lt))), (-((p.p20 * locals.var_lt_dn3) / (locals.var_lt * locals.var_lt))), (-((p.p20 * locals.var_lt_dn4) / (locals.var_lt * locals.var_lt))), (-((p.p20 * locals.var_lt_dn5) / (locals.var_lt * locals.var_lt))), (-((p.p20 * locals.var_lt_dn6) / (locals.var_lt * locals.var_lt))), (-((p.p20 * locals.var_lt_dn7) / (locals.var_lt * locals.var_lt))), (-((p.p20 * locals.var_lt_dn8) / (locals.var_lt * locals.var_lt))), (-((p.p20 * locals.var_lt_dn9) / (locals.var_lt * locals.var_lt))), (-((p.p20 * locals.var_lt_dn10) / (locals.var_lt * locals.var_lt))), (-((p.p20 * locals.var_lt_dn11) / (locals.var_lt * locals.var_lt))), (-((p.p20 * locals.var_lt_dn13) / (locals.var_lt * locals.var_lt))), (-((p.p20 * locals.var_lt_dn14) / (locals.var_lt * locals.var_lt))),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn3, locals.var_alpha_dn4, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn13, locals.var_alpha_dn14,)
    }
};
        locals.var_alpha = assign9550_e12113;
        locals.var_alpha_dn0 = assign9550_e12113_d_n0;
        locals.var_alpha_dn2 = assign9550_e12113_d_n2;
        locals.var_alpha_dn3 = assign9550_e12113_d_n3;
        locals.var_alpha_dn4 = assign9550_e12113_d_n4;
        locals.var_alpha_dn5 = assign9550_e12113_d_n5;
        locals.var_alpha_dn6 = assign9550_e12113_d_n6;
        locals.var_alpha_dn7 = assign9550_e12113_d_n7;
        locals.var_alpha_dn8 = assign9550_e12113_d_n8;
        locals.var_alpha_dn9 = assign9550_e12113_d_n9;
        locals.var_alpha_dn10 = assign9550_e12113_d_n10;
        locals.var_alpha_dn11 = assign9550_e12113_d_n11;
        locals.var_alpha_dn13 = assign9550_e12113_d_n13;
        locals.var_alpha_dn14 = assign9550_e12113_d_n14;
        locals.var_alpha_rv = 0.0;

        let (assign9560_e12121, assign9560_e12121_d_n0, assign9560_e12121_d_n2, assign9560_e12121_d_n3, assign9560_e12121_d_n4, assign9560_e12121_d_n5, assign9560_e12121_d_n6, assign9560_e12121_d_n7, assign9560_e12121_d_n8, assign9560_e12121_d_n9, assign9560_e12121_d_n10, assign9560_e12121_d_n11, assign9560_e12121_d_n13, assign9560_e12121_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        let assign9560_e12118: f64 = (2.0 * locals.var_alpha);
        let assign9560_e12119: f64 = { let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign9560_e12119, ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn0)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn2)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn3)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn4)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn5)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn6)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn7)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn8)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn9)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn10)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn11)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn13)), ({ let limited_exp_arg = assign9560_e12118; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (2.0 * locals.var_alpha_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign9560_e12121;
        locals.var_t0_dn0 = assign9560_e12121_d_n0;
        locals.var_t0_dn2 = assign9560_e12121_d_n2;
        locals.var_t0_dn3 = assign9560_e12121_d_n3;
        locals.var_t0_dn4 = assign9560_e12121_d_n4;
        locals.var_t0_dn5 = assign9560_e12121_d_n5;
        locals.var_t0_dn6 = assign9560_e12121_d_n6;
        locals.var_t0_dn7 = assign9560_e12121_d_n7;
        locals.var_t0_dn8 = assign9560_e12121_d_n8;
        locals.var_t0_dn9 = assign9560_e12121_d_n9;
        locals.var_t0_dn10 = assign9560_e12121_d_n10;
        locals.var_t0_dn11 = assign9560_e12121_d_n11;
        locals.var_t0_dn13 = assign9560_e12121_d_n13;
        locals.var_t0_dn14 = assign9560_e12121_d_n14;
        locals.var_t0_rv = 0.0;

        let assign9570_e12124: f64 = if p.p1086 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard191 = assign9570_e12124;
        locals.var_guard191_rv = 0.0;

        let (assign9580_e12135, assign9580_e12135_d_n0, assign9580_e12135_d_n2, assign9580_e12135_d_n3, assign9580_e12135_d_n4, assign9580_e12135_d_n5, assign9580_e12135_d_n6, assign9580_e12135_d_n7, assign9580_e12135_d_n8, assign9580_e12135_d_n9, assign9580_e12135_d_n10, assign9580_e12135_d_n11, assign9580_e12135_d_n13, assign9580_e12135_d_n14,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard191 != 0.0)) {
        let assign9580_e12131: f64 = (locals.var_rhorsd_v * locals.var_lt);
        let assign9580_e12133: f64 = (assign9580_e12131 / p.p1082);
        (assign9580_e12133, (((locals.var_rhorsd_v_dn0 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn0)) / p.p1082), (((locals.var_rhorsd_v_dn2 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn2)) / p.p1082), (((locals.var_rhorsd_v_dn3 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn3)) / p.p1082), (((locals.var_rhorsd_v_dn4 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn4)) / p.p1082), (((locals.var_rhorsd_v_dn5 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn5)) / p.p1082), (((locals.var_rhorsd_v_dn6 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn6)) / p.p1082), (((locals.var_rhorsd_v_dn7 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn7)) / p.p1082), (((locals.var_rhorsd_v_dn8 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn8)) / p.p1082), (((locals.var_rhorsd_v_dn9 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn9)) / p.p1082), (((locals.var_rhorsd_v_dn10 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn10)) / p.p1082), (((locals.var_rhorsd_v_dn11 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn11)) / p.p1082), (((locals.var_rhorsd_v_dn13 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn13)) / p.p1082), (((locals.var_rhorsd_v_dn14 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn14)) / p.p1082),)
    } else {
        (locals.var_eta, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn3, locals.var_eta_dn4, locals.var_eta_dn5, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn8, locals.var_eta_dn9, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn13, locals.var_eta_dn14,)
    }
};
        locals.var_eta = assign9580_e12135;
        locals.var_eta_dn0 = assign9580_e12135_d_n0;
        locals.var_eta_dn2 = assign9580_e12135_d_n2;
        locals.var_eta_dn3 = assign9580_e12135_d_n3;
        locals.var_eta_dn4 = assign9580_e12135_d_n4;
        locals.var_eta_dn5 = assign9580_e12135_d_n5;
        locals.var_eta_dn6 = assign9580_e12135_d_n6;
        locals.var_eta_dn7 = assign9580_e12135_d_n7;
        locals.var_eta_dn8 = assign9580_e12135_d_n8;
        locals.var_eta_dn9 = assign9580_e12135_d_n9;
        locals.var_eta_dn10 = assign9580_e12135_d_n10;
        locals.var_eta_dn11 = assign9580_e12135_d_n11;
        locals.var_eta_dn13 = assign9580_e12135_d_n13;
        locals.var_eta_dn14 = assign9580_e12135_d_n14;
        locals.var_eta_rv = 0.0;

        let (assign9590_e12146, assign9590_e12146_d_n0, assign9590_e12146_d_n2, assign9590_e12146_d_n3, assign9590_e12146_d_n4, assign9590_e12146_d_n5, assign9590_e12146_d_n6, assign9590_e12146_d_n7, assign9590_e12146_d_n8, assign9590_e12146_d_n9, assign9590_e12146_d_n10, assign9590_e12146_d_n11, assign9590_e12146_d_n13, assign9590_e12146_d_n14,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard191 != 0.0)) {
        let assign9590_e12143: f64 = (1.0 + locals.var_eta);
        let assign9590_e12144: f64 = (locals.var_t0 * assign9590_e12143);
        (assign9590_e12144, ((locals.var_t0_dn0 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn0)), ((locals.var_t0_dn2 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn2)), ((locals.var_t0_dn3 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn3)), ((locals.var_t0_dn4 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn4)), ((locals.var_t0_dn5 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn5)), ((locals.var_t0_dn6 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn6)), ((locals.var_t0_dn7 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn7)), ((locals.var_t0_dn8 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn8)), ((locals.var_t0_dn9 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn9)), ((locals.var_t0_dn10 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn10)), ((locals.var_t0_dn11 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn11)), ((locals.var_t0_dn13 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn13)), ((locals.var_t0_dn14 * assign9590_e12143) + (locals.var_t0 * locals.var_eta_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign9590_e12146;
        locals.var_t1_dn0 = assign9590_e12146_d_n0;
        locals.var_t1_dn2 = assign9590_e12146_d_n2;
        locals.var_t1_dn3 = assign9590_e12146_d_n3;
        locals.var_t1_dn4 = assign9590_e12146_d_n4;
        locals.var_t1_dn5 = assign9590_e12146_d_n5;
        locals.var_t1_dn6 = assign9590_e12146_d_n6;
        locals.var_t1_dn7 = assign9590_e12146_d_n7;
        locals.var_t1_dn8 = assign9590_e12146_d_n8;
        locals.var_t1_dn9 = assign9590_e12146_d_n9;
        locals.var_t1_dn10 = assign9590_e12146_d_n10;
        locals.var_t1_dn11 = assign9590_e12146_d_n11;
        locals.var_t1_dn13 = assign9590_e12146_d_n13;
        locals.var_t1_dn14 = assign9590_e12146_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign9600_e12157, assign9600_e12157_d_n0, assign9600_e12157_d_n2, assign9600_e12157_d_n3, assign9600_e12157_d_n4, assign9600_e12157_d_n5, assign9600_e12157_d_n6, assign9600_e12157_d_n7, assign9600_e12157_d_n8, assign9600_e12157_d_n9, assign9600_e12157_d_n10, assign9600_e12157_d_n11, assign9600_e12157_d_n13, assign9600_e12157_d_n14,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard191 != 0.0)) {
        let assign9600_e12153: f64 = (locals.var_t1 + 1.0);
        let assign9600_e12155: f64 = (assign9600_e12153 - locals.var_eta);
        (assign9600_e12155, (locals.var_t1_dn0 - locals.var_eta_dn0), (locals.var_t1_dn2 - locals.var_eta_dn2), (locals.var_t1_dn3 - locals.var_eta_dn3), (locals.var_t1_dn4 - locals.var_eta_dn4), (locals.var_t1_dn5 - locals.var_eta_dn5), (locals.var_t1_dn6 - locals.var_eta_dn6), (locals.var_t1_dn7 - locals.var_eta_dn7), (locals.var_t1_dn8 - locals.var_eta_dn8), (locals.var_t1_dn9 - locals.var_eta_dn9), (locals.var_t1_dn10 - locals.var_eta_dn10), (locals.var_t1_dn11 - locals.var_eta_dn11), (locals.var_t1_dn13 - locals.var_eta_dn13), (locals.var_t1_dn14 - locals.var_eta_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign9600_e12157;
        locals.var_t2_dn0 = assign9600_e12157_d_n0;
        locals.var_t2_dn2 = assign9600_e12157_d_n2;
        locals.var_t2_dn3 = assign9600_e12157_d_n3;
        locals.var_t2_dn4 = assign9600_e12157_d_n4;
        locals.var_t2_dn5 = assign9600_e12157_d_n5;
        locals.var_t2_dn6 = assign9600_e12157_d_n6;
        locals.var_t2_dn7 = assign9600_e12157_d_n7;
        locals.var_t2_dn8 = assign9600_e12157_d_n8;
        locals.var_t2_dn9 = assign9600_e12157_d_n9;
        locals.var_t2_dn10 = assign9600_e12157_d_n10;
        locals.var_t2_dn11 = assign9600_e12157_d_n11;
        locals.var_t2_dn13 = assign9600_e12157_d_n13;
        locals.var_t2_dn14 = assign9600_e12157_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign9610_e12168, assign9610_e12168_d_n0, assign9610_e12168_d_n2, assign9610_e12168_d_n3, assign9610_e12168_d_n4, assign9610_e12168_d_n5, assign9610_e12168_d_n6, assign9610_e12168_d_n7, assign9610_e12168_d_n8, assign9610_e12168_d_n9, assign9610_e12168_d_n10, assign9610_e12168_d_n11, assign9610_e12168_d_n13, assign9610_e12168_d_n14,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard191 != 0.0)) {
        let assign9610_e12164: f64 = (locals.var_t1 - 1.0);
        let assign9610_e12166: f64 = (assign9610_e12164 + locals.var_eta);
        (assign9610_e12166, (locals.var_t1_dn0 + locals.var_eta_dn0), (locals.var_t1_dn2 + locals.var_eta_dn2), (locals.var_t1_dn3 + locals.var_eta_dn3), (locals.var_t1_dn4 + locals.var_eta_dn4), (locals.var_t1_dn5 + locals.var_eta_dn5), (locals.var_t1_dn6 + locals.var_eta_dn6), (locals.var_t1_dn7 + locals.var_eta_dn7), (locals.var_t1_dn8 + locals.var_eta_dn8), (locals.var_t1_dn9 + locals.var_eta_dn9), (locals.var_t1_dn10 + locals.var_eta_dn10), (locals.var_t1_dn11 + locals.var_eta_dn11), (locals.var_t1_dn13 + locals.var_eta_dn13), (locals.var_t1_dn14 + locals.var_eta_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9610_e12168;
        locals.var_t3_dn0 = assign9610_e12168_d_n0;
        locals.var_t3_dn2 = assign9610_e12168_d_n2;
        locals.var_t3_dn3 = assign9610_e12168_d_n3;
        locals.var_t3_dn4 = assign9610_e12168_d_n4;
        locals.var_t3_dn5 = assign9610_e12168_d_n5;
        locals.var_t3_dn6 = assign9610_e12168_d_n6;
        locals.var_t3_dn7 = assign9610_e12168_d_n7;
        locals.var_t3_dn8 = assign9610_e12168_d_n8;
        locals.var_t3_dn9 = assign9610_e12168_d_n9;
        locals.var_t3_dn10 = assign9610_e12168_d_n10;
        locals.var_t3_dn11 = assign9610_e12168_d_n11;
        locals.var_t3_dn13 = assign9610_e12168_d_n13;
        locals.var_t3_dn14 = assign9610_e12168_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9620_e12178, assign9620_e12178_d_n0, assign9620_e12178_d_n2, assign9620_e12178_d_n3, assign9620_e12178_d_n4, assign9620_e12178_d_n5, assign9620_e12178_d_n6, assign9620_e12178_d_n7, assign9620_e12178_d_n8, assign9620_e12178_d_n9, assign9620_e12178_d_n10, assign9620_e12178_d_n11, assign9620_e12178_d_n13, assign9620_e12178_d_n14,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard191 == 0.0)) {
        let assign9620_e12176: f64 = (locals.var_t0 + 1.0);
        (assign9620_e12176, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign9620_e12178;
        locals.var_t2_dn0 = assign9620_e12178_d_n0;
        locals.var_t2_dn2 = assign9620_e12178_d_n2;
        locals.var_t2_dn3 = assign9620_e12178_d_n3;
        locals.var_t2_dn4 = assign9620_e12178_d_n4;
        locals.var_t2_dn5 = assign9620_e12178_d_n5;
        locals.var_t2_dn6 = assign9620_e12178_d_n6;
        locals.var_t2_dn7 = assign9620_e12178_d_n7;
        locals.var_t2_dn8 = assign9620_e12178_d_n8;
        locals.var_t2_dn9 = assign9620_e12178_d_n9;
        locals.var_t2_dn10 = assign9620_e12178_d_n10;
        locals.var_t2_dn11 = assign9620_e12178_d_n11;
        locals.var_t2_dn13 = assign9620_e12178_d_n13;
        locals.var_t2_dn14 = assign9620_e12178_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign9630_e12188, assign9630_e12188_d_n0, assign9630_e12188_d_n2, assign9630_e12188_d_n3, assign9630_e12188_d_n4, assign9630_e12188_d_n5, assign9630_e12188_d_n6, assign9630_e12188_d_n7, assign9630_e12188_d_n8, assign9630_e12188_d_n9, assign9630_e12188_d_n10, assign9630_e12188_d_n11, assign9630_e12188_d_n13, assign9630_e12188_d_n14,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard191 == 0.0)) {
        let assign9630_e12186: f64 = (locals.var_t0 - 1.0);
        (assign9630_e12186, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9630_e12188;
        locals.var_t3_dn0 = assign9630_e12188_d_n0;
        locals.var_t3_dn2 = assign9630_e12188_d_n2;
        locals.var_t3_dn3 = assign9630_e12188_d_n3;
        locals.var_t3_dn4 = assign9630_e12188_d_n4;
        locals.var_t3_dn5 = assign9630_e12188_d_n5;
        locals.var_t3_dn6 = assign9630_e12188_d_n6;
        locals.var_t3_dn7 = assign9630_e12188_d_n7;
        locals.var_t3_dn8 = assign9630_e12188_d_n8;
        locals.var_t3_dn9 = assign9630_e12188_d_n9;
        locals.var_t3_dn10 = assign9630_e12188_d_n10;
        locals.var_t3_dn11 = assign9630_e12188_d_n11;
        locals.var_t3_dn13 = assign9630_e12188_d_n13;
        locals.var_t3_dn14 = assign9630_e12188_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9640_e12201, assign9640_e12201_d_n0, assign9640_e12201_d_n2, assign9640_e12201_d_n3, assign9640_e12201_d_n4, assign9640_e12201_d_n5, assign9640_e12201_d_n6, assign9640_e12201_d_n7, assign9640_e12201_d_n8, assign9640_e12201_d_n9, assign9640_e12201_d_n10, assign9640_e12201_d_n11, assign9640_e12201_d_n13, assign9640_e12201_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        let assign9640_e12193: f64 = (locals.var_rhorsd_v * locals.var_lt);
        let assign9640_e12195: f64 = (assign9640_e12193 * locals.var_t2);
        let assign9640_e12198: f64 = (locals.var_arsd_total * locals.var_t3);
        let assign9640_e12199: f64 = (assign9640_e12195 / assign9640_e12198);
        (assign9640_e12199, (((((((locals.var_rhorsd_v_dn0 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn0)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn0)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn0))) / (assign9640_e12198 * assign9640_e12198)), (((((((locals.var_rhorsd_v_dn2 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn2)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn2)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn2))) / (assign9640_e12198 * assign9640_e12198)), (((((((locals.var_rhorsd_v_dn3 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn3)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn3)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn3))) / (assign9640_e12198 * assign9640_e12198)), (((((((locals.var_rhorsd_v_dn4 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn4)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn4)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn4))) / (assign9640_e12198 * assign9640_e12198)), (((((((locals.var_rhorsd_v_dn5 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn5)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn5)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn5))) / (assign9640_e12198 * assign9640_e12198)), (((((((locals.var_rhorsd_v_dn6 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn6)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn6)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn6))) / (assign9640_e12198 * assign9640_e12198)), (((((((locals.var_rhorsd_v_dn7 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn7)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn7)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn7))) / (assign9640_e12198 * assign9640_e12198)), (((((((locals.var_rhorsd_v_dn8 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn8)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn8)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn8))) / (assign9640_e12198 * assign9640_e12198)), (((((((locals.var_rhorsd_v_dn9 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn9)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn9)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn9))) / (assign9640_e12198 * assign9640_e12198)), (((((((locals.var_rhorsd_v_dn10 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn10)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn10)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn10))) / (assign9640_e12198 * assign9640_e12198)), (((((((locals.var_rhorsd_v_dn11 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn11)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn11)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn11))) / (assign9640_e12198 * assign9640_e12198)), (((((((locals.var_rhorsd_v_dn13 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn13)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn13)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn13))) / (assign9640_e12198 * assign9640_e12198)), (((((((locals.var_rhorsd_v_dn14 * locals.var_lt) + (locals.var_rhorsd_v * locals.var_lt_dn14)) * locals.var_t2) + (assign9640_e12193 * locals.var_t2_dn14)) * assign9640_e12198) - (assign9640_e12195 * (locals.var_arsd_total * locals.var_t3_dn14))) / (assign9640_e12198 * assign9640_e12198)),)
    } else {
        (locals.var_rrsdtml, locals.var_rrsdtml_dn0, locals.var_rrsdtml_dn2, locals.var_rrsdtml_dn3, locals.var_rrsdtml_dn4, locals.var_rrsdtml_dn5, locals.var_rrsdtml_dn6, locals.var_rrsdtml_dn7, locals.var_rrsdtml_dn8, locals.var_rrsdtml_dn9, locals.var_rrsdtml_dn10, locals.var_rrsdtml_dn11, locals.var_rrsdtml_dn13, locals.var_rrsdtml_dn14,)
    }
};
        locals.var_rrsdtml = assign9640_e12201;
        locals.var_rrsdtml_dn0 = assign9640_e12201_d_n0;
        locals.var_rrsdtml_dn2 = assign9640_e12201_d_n2;
        locals.var_rrsdtml_dn3 = assign9640_e12201_d_n3;
        locals.var_rrsdtml_dn4 = assign9640_e12201_d_n4;
        locals.var_rrsdtml_dn5 = assign9640_e12201_d_n5;
        locals.var_rrsdtml_dn6 = assign9640_e12201_d_n6;
        locals.var_rrsdtml_dn7 = assign9640_e12201_d_n7;
        locals.var_rrsdtml_dn8 = assign9640_e12201_d_n8;
        locals.var_rrsdtml_dn9 = assign9640_e12201_d_n9;
        locals.var_rrsdtml_dn10 = assign9640_e12201_d_n10;
        locals.var_rrsdtml_dn11 = assign9640_e12201_d_n11;
        locals.var_rrsdtml_dn13 = assign9640_e12201_d_n13;
        locals.var_rrsdtml_dn14 = assign9640_e12201_d_n14;
        locals.var_rrsdtml_rv = 0.0;

        let assign9650_e12204: f64 = (-1e-10);
        let assign9650_e12205: f64 = if p.p1080 < assign9650_e12204 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign9650_e12205;
        locals.var_guard192_rv = 0.0;

        let (assign9660_e12219,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard192 != 0.0)) {
        let assign9660_e12212: f64 = (-p.p1080);
        let assign9660_e12214: f64 = (assign9660_e12212 * p.p3);
        let assign9660_e12216: f64 = (assign9660_e12214 * p.p5);
        let assign9660_e12217: f64 = (p.p1082 / assign9660_e12216);
        (assign9660_e12217,)
    } else {
        (locals.var_rrsdside,)
    }
};
        locals.var_rrsdside = assign9660_e12219;
        locals.var_rrsdside_rv = 0.0;

        let (assign9670_e12236, assign9670_e12236_d_n0, assign9670_e12236_d_n2, assign9670_e12236_d_n3, assign9670_e12236_d_n4, assign9670_e12236_d_n5, assign9670_e12236_d_n6, assign9670_e12236_d_n7, assign9670_e12236_d_n8, assign9670_e12236_d_n9, assign9670_e12236_d_n10, assign9670_e12236_d_n11, assign9670_e12236_d_n13, assign9670_e12236_d_n14,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard192 != 0.0)) {
        let assign9670_e12226: f64 = (locals.var_rrsdtml + locals.var_rsp);
        let assign9670_e12228: f64 = (assign9670_e12226 * locals.var_rrsdside);
        let assign9670_e12231: f64 = (locals.var_rrsdtml + locals.var_rsp);
        let assign9670_e12233: f64 = (assign9670_e12231 + locals.var_rrsdside);
        let assign9670_e12234: f64 = (assign9670_e12228 / assign9670_e12233);
        (assign9670_e12234, (((((locals.var_rrsdtml_dn0 + locals.var_rsp_dn0) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn0 + locals.var_rsp_dn0))) / (assign9670_e12233 * assign9670_e12233)), (((((locals.var_rrsdtml_dn2 + locals.var_rsp_dn2) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn2 + locals.var_rsp_dn2))) / (assign9670_e12233 * assign9670_e12233)), (((((locals.var_rrsdtml_dn3 + locals.var_rsp_dn3) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn3 + locals.var_rsp_dn3))) / (assign9670_e12233 * assign9670_e12233)), (((((locals.var_rrsdtml_dn4 + locals.var_rsp_dn4) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn4 + locals.var_rsp_dn4))) / (assign9670_e12233 * assign9670_e12233)), (((((locals.var_rrsdtml_dn5 + locals.var_rsp_dn5) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn5 + locals.var_rsp_dn5))) / (assign9670_e12233 * assign9670_e12233)), (((((locals.var_rrsdtml_dn6 + locals.var_rsp_dn6) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn6 + locals.var_rsp_dn6))) / (assign9670_e12233 * assign9670_e12233)), (((((locals.var_rrsdtml_dn7 + locals.var_rsp_dn7) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn7 + locals.var_rsp_dn7))) / (assign9670_e12233 * assign9670_e12233)), (((((locals.var_rrsdtml_dn8 + locals.var_rsp_dn8) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn8 + locals.var_rsp_dn8))) / (assign9670_e12233 * assign9670_e12233)), (((((locals.var_rrsdtml_dn9 + locals.var_rsp_dn9) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn9 + locals.var_rsp_dn9))) / (assign9670_e12233 * assign9670_e12233)), (((((locals.var_rrsdtml_dn10 + locals.var_rsp_dn10) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn10 + locals.var_rsp_dn10))) / (assign9670_e12233 * assign9670_e12233)), (((((locals.var_rrsdtml_dn11 + locals.var_rsp_dn11) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn11 + locals.var_rsp_dn11))) / (assign9670_e12233 * assign9670_e12233)), (((((locals.var_rrsdtml_dn13 + locals.var_rsp_dn13) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn13 + locals.var_rsp_dn13))) / (assign9670_e12233 * assign9670_e12233)), (((((locals.var_rrsdtml_dn14 + locals.var_rsp_dn14) * locals.var_rrsdside) * assign9670_e12233) - (assign9670_e12228 * (locals.var_rrsdtml_dn14 + locals.var_rsp_dn14))) / (assign9670_e12233 * assign9670_e12233)),)
    } else {
        (locals.var_rrsd, locals.var_rrsd_dn0, locals.var_rrsd_dn2, locals.var_rrsd_dn3, locals.var_rrsd_dn4, locals.var_rrsd_dn5, locals.var_rrsd_dn6, locals.var_rrsd_dn7, locals.var_rrsd_dn8, locals.var_rrsd_dn9, locals.var_rrsd_dn10, locals.var_rrsd_dn11, locals.var_rrsd_dn13, locals.var_rrsd_dn14,)
    }
};
        locals.var_rrsd = assign9670_e12236;
        locals.var_rrsd_dn0 = assign9670_e12236_d_n0;
        locals.var_rrsd_dn2 = assign9670_e12236_d_n2;
        locals.var_rrsd_dn3 = assign9670_e12236_d_n3;
        locals.var_rrsd_dn4 = assign9670_e12236_d_n4;
        locals.var_rrsd_dn5 = assign9670_e12236_d_n5;
        locals.var_rrsd_dn6 = assign9670_e12236_d_n6;
        locals.var_rrsd_dn7 = assign9670_e12236_d_n7;
        locals.var_rrsd_dn8 = assign9670_e12236_d_n8;
        locals.var_rrsd_dn9 = assign9670_e12236_d_n9;
        locals.var_rrsd_dn10 = assign9670_e12236_d_n10;
        locals.var_rrsd_dn11 = assign9670_e12236_d_n11;
        locals.var_rrsd_dn13 = assign9670_e12236_d_n13;
        locals.var_rrsd_dn14 = assign9670_e12236_d_n14;
        locals.var_rrsd_rv = 0.0;

        let (assign9680_e12246, assign9680_e12246_d_n0, assign9680_e12246_d_n2, assign9680_e12246_d_n3, assign9680_e12246_d_n4, assign9680_e12246_d_n5, assign9680_e12246_d_n6, assign9680_e12246_d_n7, assign9680_e12246_d_n8, assign9680_e12246_d_n9, assign9680_e12246_d_n10, assign9680_e12246_d_n11, assign9680_e12246_d_n13, assign9680_e12246_d_n14,) = {
    if ((locals.var_guard187 == 0.0) && (locals.var_guard192 == 0.0)) {
        let assign9680_e12244: f64 = (locals.var_rrsdtml + locals.var_rsp);
        (assign9680_e12244, (locals.var_rrsdtml_dn0 + locals.var_rsp_dn0), (locals.var_rrsdtml_dn2 + locals.var_rsp_dn2), (locals.var_rrsdtml_dn3 + locals.var_rsp_dn3), (locals.var_rrsdtml_dn4 + locals.var_rsp_dn4), (locals.var_rrsdtml_dn5 + locals.var_rsp_dn5), (locals.var_rrsdtml_dn6 + locals.var_rsp_dn6), (locals.var_rrsdtml_dn7 + locals.var_rsp_dn7), (locals.var_rrsdtml_dn8 + locals.var_rsp_dn8), (locals.var_rrsdtml_dn9 + locals.var_rsp_dn9), (locals.var_rrsdtml_dn10 + locals.var_rsp_dn10), (locals.var_rrsdtml_dn11 + locals.var_rsp_dn11), (locals.var_rrsdtml_dn13 + locals.var_rsp_dn13), (locals.var_rrsdtml_dn14 + locals.var_rsp_dn14),)
    } else {
        (locals.var_rrsd, locals.var_rrsd_dn0, locals.var_rrsd_dn2, locals.var_rrsd_dn3, locals.var_rrsd_dn4, locals.var_rrsd_dn5, locals.var_rrsd_dn6, locals.var_rrsd_dn7, locals.var_rrsd_dn8, locals.var_rrsd_dn9, locals.var_rrsd_dn10, locals.var_rrsd_dn11, locals.var_rrsd_dn13, locals.var_rrsd_dn14,)
    }
};
        locals.var_rrsd = assign9680_e12246;
        locals.var_rrsd_dn0 = assign9680_e12246_d_n0;
        locals.var_rrsd_dn2 = assign9680_e12246_d_n2;
        locals.var_rrsd_dn3 = assign9680_e12246_d_n3;
        locals.var_rrsd_dn4 = assign9680_e12246_d_n4;
        locals.var_rrsd_dn5 = assign9680_e12246_d_n5;
        locals.var_rrsd_dn6 = assign9680_e12246_d_n6;
        locals.var_rrsd_dn7 = assign9680_e12246_d_n7;
        locals.var_rrsd_dn8 = assign9680_e12246_d_n8;
        locals.var_rrsd_dn9 = assign9680_e12246_d_n9;
        locals.var_rrsd_dn10 = assign9680_e12246_d_n10;
        locals.var_rrsd_dn11 = assign9680_e12246_d_n11;
        locals.var_rrsd_dn13 = assign9680_e12246_d_n13;
        locals.var_rrsd_dn14 = assign9680_e12246_d_n14;
        locals.var_rrsd_rv = 0.0;

        let (assign9690_e12273, assign9690_e12273_d_n0, assign9690_e12273_d_n2, assign9690_e12273_d_n3, assign9690_e12273_d_n4, assign9690_e12273_d_n5, assign9690_e12273_d_n6, assign9690_e12273_d_n7, assign9690_e12273_d_n8, assign9690_e12273_d_n9, assign9690_e12273_d_n10, assign9690_e12273_d_n11, assign9690_e12273_d_n13, assign9690_e12273_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        let assign9690_e12251: f64 = (locals.var_rrsd / p.p59);
        let assign9690_e12256: f64 = (p.p1095 * p.p3);
        let assign9690_e12257: f64 = (p.p1094 + assign9690_e12256);
        let assign9690_e12260: f64 = (p.p1096 * p.p4);
        let assign9690_e12261: f64 = (assign9690_e12257 + assign9690_e12260);
        let assign9690_e12264: f64 = (p.p1097 * p.p20);
        let assign9690_e12265: f64 = (assign9690_e12261 + assign9690_e12264);
        let assign9690_e12268: f64 = (p.p1098 * p.p1080);
        let assign9690_e12269: f64 = (assign9690_e12265 + assign9690_e12268);
        let assign9690_e12270: f64 = (0.0_f64).max(assign9690_e12269);
        let assign9690_e12271: f64 = (assign9690_e12251 * assign9690_e12270);
        (assign9690_e12271, ((locals.var_rrsd_dn0 / p.p59) * assign9690_e12270), ((locals.var_rrsd_dn2 / p.p59) * assign9690_e12270), ((locals.var_rrsd_dn3 / p.p59) * assign9690_e12270), ((locals.var_rrsd_dn4 / p.p59) * assign9690_e12270), ((locals.var_rrsd_dn5 / p.p59) * assign9690_e12270), ((locals.var_rrsd_dn6 / p.p59) * assign9690_e12270), ((locals.var_rrsd_dn7 / p.p59) * assign9690_e12270), ((locals.var_rrsd_dn8 / p.p59) * assign9690_e12270), ((locals.var_rrsd_dn9 / p.p59) * assign9690_e12270), ((locals.var_rrsd_dn10 / p.p59) * assign9690_e12270), ((locals.var_rrsd_dn11 / p.p59) * assign9690_e12270), ((locals.var_rrsd_dn13 / p.p59) * assign9690_e12270), ((locals.var_rrsd_dn14 / p.p59) * assign9690_e12270),)
    } else {
        (locals.var_rdsgeo, locals.var_rdsgeo_dn0, locals.var_rdsgeo_dn2, locals.var_rdsgeo_dn3, locals.var_rdsgeo_dn4, locals.var_rdsgeo_dn5, locals.var_rdsgeo_dn6, locals.var_rdsgeo_dn7, locals.var_rdsgeo_dn8, locals.var_rdsgeo_dn9, locals.var_rdsgeo_dn10, locals.var_rdsgeo_dn11, locals.var_rdsgeo_dn13, locals.var_rdsgeo_dn14,)
    }
};
        locals.var_rdsgeo = assign9690_e12273;
        locals.var_rdsgeo_dn0 = assign9690_e12273_d_n0;
        locals.var_rdsgeo_dn2 = assign9690_e12273_d_n2;
        locals.var_rdsgeo_dn3 = assign9690_e12273_d_n3;
        locals.var_rdsgeo_dn4 = assign9690_e12273_d_n4;
        locals.var_rdsgeo_dn5 = assign9690_e12273_d_n5;
        locals.var_rdsgeo_dn6 = assign9690_e12273_d_n6;
        locals.var_rdsgeo_dn7 = assign9690_e12273_d_n7;
        locals.var_rdsgeo_dn8 = assign9690_e12273_d_n8;
        locals.var_rdsgeo_dn9 = assign9690_e12273_d_n9;
        locals.var_rdsgeo_dn10 = assign9690_e12273_d_n10;
        locals.var_rdsgeo_dn11 = assign9690_e12273_d_n11;
        locals.var_rdsgeo_dn13 = assign9690_e12273_d_n13;
        locals.var_rdsgeo_dn14 = assign9690_e12273_d_n14;
        locals.var_rdsgeo_rv = 0.0;

        let (assign9700_e12278, assign9700_e12278_d_n0, assign9700_e12278_d_n2, assign9700_e12278_d_n3, assign9700_e12278_d_n4, assign9700_e12278_d_n5, assign9700_e12278_d_n6, assign9700_e12278_d_n7, assign9700_e12278_d_n8, assign9700_e12278_d_n9, assign9700_e12278_d_n10, assign9700_e12278_d_n11, assign9700_e12278_d_n13, assign9700_e12278_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (locals.var_rdsgeo, locals.var_rdsgeo_dn0, locals.var_rdsgeo_dn2, locals.var_rdsgeo_dn3, locals.var_rdsgeo_dn4, locals.var_rdsgeo_dn5, locals.var_rdsgeo_dn6, locals.var_rdsgeo_dn7, locals.var_rdsgeo_dn8, locals.var_rdsgeo_dn9, locals.var_rdsgeo_dn10, locals.var_rdsgeo_dn11, locals.var_rdsgeo_dn13, locals.var_rdsgeo_dn14,)
    } else {
        (locals.var_rsourcegeo, locals.var_rsourcegeo_dn0, locals.var_rsourcegeo_dn2, locals.var_rsourcegeo_dn3, locals.var_rsourcegeo_dn4, locals.var_rsourcegeo_dn5, locals.var_rsourcegeo_dn6, locals.var_rsourcegeo_dn7, locals.var_rsourcegeo_dn8, locals.var_rsourcegeo_dn9, locals.var_rsourcegeo_dn10, locals.var_rsourcegeo_dn11, locals.var_rsourcegeo_dn13, locals.var_rsourcegeo_dn14,)
    }
};
        locals.var_rsourcegeo = assign9700_e12278;
        locals.var_rsourcegeo_dn0 = assign9700_e12278_d_n0;
        locals.var_rsourcegeo_dn2 = assign9700_e12278_d_n2;
        locals.var_rsourcegeo_dn3 = assign9700_e12278_d_n3;
        locals.var_rsourcegeo_dn4 = assign9700_e12278_d_n4;
        locals.var_rsourcegeo_dn5 = assign9700_e12278_d_n5;
        locals.var_rsourcegeo_dn6 = assign9700_e12278_d_n6;
        locals.var_rsourcegeo_dn7 = assign9700_e12278_d_n7;
        locals.var_rsourcegeo_dn8 = assign9700_e12278_d_n8;
        locals.var_rsourcegeo_dn9 = assign9700_e12278_d_n9;
        locals.var_rsourcegeo_dn10 = assign9700_e12278_d_n10;
        locals.var_rsourcegeo_dn11 = assign9700_e12278_d_n11;
        locals.var_rsourcegeo_dn13 = assign9700_e12278_d_n13;
        locals.var_rsourcegeo_dn14 = assign9700_e12278_d_n14;
        locals.var_rsourcegeo_rv = 0.0;

        let (assign9710_e12283, assign9710_e12283_d_n0, assign9710_e12283_d_n2, assign9710_e12283_d_n3, assign9710_e12283_d_n4, assign9710_e12283_d_n5, assign9710_e12283_d_n6, assign9710_e12283_d_n7, assign9710_e12283_d_n8, assign9710_e12283_d_n9, assign9710_e12283_d_n10, assign9710_e12283_d_n11, assign9710_e12283_d_n13, assign9710_e12283_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (locals.var_rdsgeo, locals.var_rdsgeo_dn0, locals.var_rdsgeo_dn2, locals.var_rdsgeo_dn3, locals.var_rdsgeo_dn4, locals.var_rdsgeo_dn5, locals.var_rdsgeo_dn6, locals.var_rdsgeo_dn7, locals.var_rdsgeo_dn8, locals.var_rdsgeo_dn9, locals.var_rdsgeo_dn10, locals.var_rdsgeo_dn11, locals.var_rdsgeo_dn13, locals.var_rdsgeo_dn14,)
    } else {
        (locals.var_rdraingeo, locals.var_rdraingeo_dn0, locals.var_rdraingeo_dn2, locals.var_rdraingeo_dn3, locals.var_rdraingeo_dn4, locals.var_rdraingeo_dn5, locals.var_rdraingeo_dn6, locals.var_rdraingeo_dn7, locals.var_rdraingeo_dn8, locals.var_rdraingeo_dn9, locals.var_rdraingeo_dn10, locals.var_rdraingeo_dn11, locals.var_rdraingeo_dn13, locals.var_rdraingeo_dn14,)
    }
};
        locals.var_rdraingeo = assign9710_e12283;
        locals.var_rdraingeo_dn0 = assign9710_e12283_d_n0;
        locals.var_rdraingeo_dn2 = assign9710_e12283_d_n2;
        locals.var_rdraingeo_dn3 = assign9710_e12283_d_n3;
        locals.var_rdraingeo_dn4 = assign9710_e12283_d_n4;
        locals.var_rdraingeo_dn5 = assign9710_e12283_d_n5;
        locals.var_rdraingeo_dn6 = assign9710_e12283_d_n6;
        locals.var_rdraingeo_dn7 = assign9710_e12283_d_n7;
        locals.var_rdraingeo_dn8 = assign9710_e12283_d_n8;
        locals.var_rdraingeo_dn9 = assign9710_e12283_d_n9;
        locals.var_rdraingeo_dn10 = assign9710_e12283_d_n10;
        locals.var_rdraingeo_dn11 = assign9710_e12283_d_n11;
        locals.var_rdraingeo_dn13 = assign9710_e12283_d_n13;
        locals.var_rdraingeo_dn14 = assign9710_e12283_d_n14;
        locals.var_rdraingeo_rv = 0.0;

        let assign9720_e12286: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard193 = assign9720_e12286;
        locals.var_guard193_rv = 0.0;

        let assign9730_e12289: f64 = if locals.var_rsourcegeo < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard194 = assign9730_e12289;
        locals.var_guard194_rv = 0.0;

        let (assign9740_e12295, assign9740_e12295_d_n0, assign9740_e12295_d_n2, assign9740_e12295_d_n3, assign9740_e12295_d_n4, assign9740_e12295_d_n5, assign9740_e12295_d_n6, assign9740_e12295_d_n7, assign9740_e12295_d_n8, assign9740_e12295_d_n9, assign9740_e12295_d_n10, assign9740_e12295_d_n11, assign9740_e12295_d_n13, assign9740_e12295_d_n14,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard194 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsourcegeo, locals.var_rsourcegeo_dn0, locals.var_rsourcegeo_dn2, locals.var_rsourcegeo_dn3, locals.var_rsourcegeo_dn4, locals.var_rsourcegeo_dn5, locals.var_rsourcegeo_dn6, locals.var_rsourcegeo_dn7, locals.var_rsourcegeo_dn8, locals.var_rsourcegeo_dn9, locals.var_rsourcegeo_dn10, locals.var_rsourcegeo_dn11, locals.var_rsourcegeo_dn13, locals.var_rsourcegeo_dn14,)
    }
};
        locals.var_rsourcegeo = assign9740_e12295;
        locals.var_rsourcegeo_dn0 = assign9740_e12295_d_n0;
        locals.var_rsourcegeo_dn2 = assign9740_e12295_d_n2;
        locals.var_rsourcegeo_dn3 = assign9740_e12295_d_n3;
        locals.var_rsourcegeo_dn4 = assign9740_e12295_d_n4;
        locals.var_rsourcegeo_dn5 = assign9740_e12295_d_n5;
        locals.var_rsourcegeo_dn6 = assign9740_e12295_d_n6;
        locals.var_rsourcegeo_dn7 = assign9740_e12295_d_n7;
        locals.var_rsourcegeo_dn8 = assign9740_e12295_d_n8;
        locals.var_rsourcegeo_dn9 = assign9740_e12295_d_n9;
        locals.var_rsourcegeo_dn10 = assign9740_e12295_d_n10;
        locals.var_rsourcegeo_dn11 = assign9740_e12295_d_n11;
        locals.var_rsourcegeo_dn13 = assign9740_e12295_d_n13;
        locals.var_rsourcegeo_dn14 = assign9740_e12295_d_n14;
        locals.var_rsourcegeo_rv = 0.0;

        let assign9750_e12298: f64 = if locals.var_rdraingeo < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard195 = assign9750_e12298;
        locals.var_guard195_rv = 0.0;

        let (assign9760_e12304, assign9760_e12304_d_n0, assign9760_e12304_d_n2, assign9760_e12304_d_n3, assign9760_e12304_d_n4, assign9760_e12304_d_n5, assign9760_e12304_d_n6, assign9760_e12304_d_n7, assign9760_e12304_d_n8, assign9760_e12304_d_n9, assign9760_e12304_d_n10, assign9760_e12304_d_n11, assign9760_e12304_d_n13, assign9760_e12304_d_n14,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard195 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdraingeo, locals.var_rdraingeo_dn0, locals.var_rdraingeo_dn2, locals.var_rdraingeo_dn3, locals.var_rdraingeo_dn4, locals.var_rdraingeo_dn5, locals.var_rdraingeo_dn6, locals.var_rdraingeo_dn7, locals.var_rdraingeo_dn8, locals.var_rdraingeo_dn9, locals.var_rdraingeo_dn10, locals.var_rdraingeo_dn11, locals.var_rdraingeo_dn13, locals.var_rdraingeo_dn14,)
    }
};
        locals.var_rdraingeo = assign9760_e12304;
        locals.var_rdraingeo_dn0 = assign9760_e12304_d_n0;
        locals.var_rdraingeo_dn2 = assign9760_e12304_d_n2;
        locals.var_rdraingeo_dn3 = assign9760_e12304_d_n3;
        locals.var_rdraingeo_dn4 = assign9760_e12304_d_n4;
        locals.var_rdraingeo_dn5 = assign9760_e12304_d_n5;
        locals.var_rdraingeo_dn6 = assign9760_e12304_d_n6;
        locals.var_rdraingeo_dn7 = assign9760_e12304_d_n7;
        locals.var_rdraingeo_dn8 = assign9760_e12304_d_n8;
        locals.var_rdraingeo_dn9 = assign9760_e12304_d_n9;
        locals.var_rdraingeo_dn10 = assign9760_e12304_d_n10;
        locals.var_rdraingeo_dn11 = assign9760_e12304_d_n11;
        locals.var_rdraingeo_dn13 = assign9760_e12304_d_n13;
        locals.var_rdraingeo_dn14 = assign9760_e12304_d_n14;
        locals.var_rdraingeo_rv = 0.0;

        let assign9770_e12307: f64 = if locals.var_rsourcegeo <= p.p151 { 1.0 } else { 0.0 };
        locals.var_guard196 = assign9770_e12307;
        locals.var_guard196_rv = 0.0;

        let (assign9780_e12314, assign9780_e12314_d_n0, assign9780_e12314_d_n2, assign9780_e12314_d_n3, assign9780_e12314_d_n4, assign9780_e12314_d_n5, assign9780_e12314_d_n6, assign9780_e12314_d_n7, assign9780_e12314_d_n8, assign9780_e12314_d_n9, assign9780_e12314_d_n10, assign9780_e12314_d_n11, assign9780_e12314_d_n13, assign9780_e12314_d_n14,) = {
    if ((locals.var_guard193 == 0.0) && (locals.var_guard196 != 0.0)) {
        (p.p151, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsourcegeo, locals.var_rsourcegeo_dn0, locals.var_rsourcegeo_dn2, locals.var_rsourcegeo_dn3, locals.var_rsourcegeo_dn4, locals.var_rsourcegeo_dn5, locals.var_rsourcegeo_dn6, locals.var_rsourcegeo_dn7, locals.var_rsourcegeo_dn8, locals.var_rsourcegeo_dn9, locals.var_rsourcegeo_dn10, locals.var_rsourcegeo_dn11, locals.var_rsourcegeo_dn13, locals.var_rsourcegeo_dn14,)
    }
};
        locals.var_rsourcegeo = assign9780_e12314;
        locals.var_rsourcegeo_dn0 = assign9780_e12314_d_n0;
        locals.var_rsourcegeo_dn2 = assign9780_e12314_d_n2;
        locals.var_rsourcegeo_dn3 = assign9780_e12314_d_n3;
        locals.var_rsourcegeo_dn4 = assign9780_e12314_d_n4;
        locals.var_rsourcegeo_dn5 = assign9780_e12314_d_n5;
        locals.var_rsourcegeo_dn6 = assign9780_e12314_d_n6;
        locals.var_rsourcegeo_dn7 = assign9780_e12314_d_n7;
        locals.var_rsourcegeo_dn8 = assign9780_e12314_d_n8;
        locals.var_rsourcegeo_dn9 = assign9780_e12314_d_n9;
        locals.var_rsourcegeo_dn10 = assign9780_e12314_d_n10;
        locals.var_rsourcegeo_dn11 = assign9780_e12314_d_n11;
        locals.var_rsourcegeo_dn13 = assign9780_e12314_d_n13;
        locals.var_rsourcegeo_dn14 = assign9780_e12314_d_n14;
        locals.var_rsourcegeo_rv = 0.0;

        let assign9790_e12317: f64 = if locals.var_rdraingeo <= p.p151 { 1.0 } else { 0.0 };
        locals.var_guard197 = assign9790_e12317;
        locals.var_guard197_rv = 0.0;

        let (assign9800_e12324, assign9800_e12324_d_n0, assign9800_e12324_d_n2, assign9800_e12324_d_n3, assign9800_e12324_d_n4, assign9800_e12324_d_n5, assign9800_e12324_d_n6, assign9800_e12324_d_n7, assign9800_e12324_d_n8, assign9800_e12324_d_n9, assign9800_e12324_d_n10, assign9800_e12324_d_n11, assign9800_e12324_d_n13, assign9800_e12324_d_n14,) = {
    if ((locals.var_guard193 == 0.0) && (locals.var_guard197 != 0.0)) {
        (p.p151, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdraingeo, locals.var_rdraingeo_dn0, locals.var_rdraingeo_dn2, locals.var_rdraingeo_dn3, locals.var_rdraingeo_dn4, locals.var_rdraingeo_dn5, locals.var_rdraingeo_dn6, locals.var_rdraingeo_dn7, locals.var_rdraingeo_dn8, locals.var_rdraingeo_dn9, locals.var_rdraingeo_dn10, locals.var_rdraingeo_dn11, locals.var_rdraingeo_dn13, locals.var_rdraingeo_dn14,)
    }
};
        locals.var_rdraingeo = assign9800_e12324;
        locals.var_rdraingeo_dn0 = assign9800_e12324_d_n0;
        locals.var_rdraingeo_dn2 = assign9800_e12324_d_n2;
        locals.var_rdraingeo_dn3 = assign9800_e12324_d_n3;
        locals.var_rdraingeo_dn4 = assign9800_e12324_d_n4;
        locals.var_rdraingeo_dn5 = assign9800_e12324_d_n5;
        locals.var_rdraingeo_dn6 = assign9800_e12324_d_n6;
        locals.var_rdraingeo_dn7 = assign9800_e12324_d_n7;
        locals.var_rdraingeo_dn8 = assign9800_e12324_d_n8;
        locals.var_rdraingeo_dn9 = assign9800_e12324_d_n9;
        locals.var_rdraingeo_dn10 = assign9800_e12324_d_n10;
        locals.var_rdraingeo_dn11 = assign9800_e12324_d_n11;
        locals.var_rdraingeo_dn13 = assign9800_e12324_d_n13;
        locals.var_rdraingeo_dn14 = assign9800_e12324_d_n14;
        locals.var_rdraingeo_rv = 0.0;

        let assign9810_e12327: f64 = if p.p78 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard198 = assign9810_e12327;
        locals.var_guard198_rv = 0.0;

        let assign9820_e12329: f64 = if param_given[1542] { 1.0 } else { 0.0 };
        locals.var_guard199 = assign9820_e12329;
        locals.var_guard199_rv = 0.0;

        let (assign9830_e12335,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        (p.p1542,)
    } else {
        (locals.var_cgso_i,)
    }
};
        locals.var_cgso_i = assign9830_e12335;
        locals.var_cgso_i_rv = 0.0;

        let assign9840_e12341: f64 = if (param_given[85] && (p.p85 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard200 = assign9840_e12341;
        locals.var_guard200_rv = 0.0;

        let (assign9850_e12356,) = {
    if (((locals.var_guard198 != 0.0) && (locals.var_guard199 == 0.0)) && (locals.var_guard200 != 0.0)) {
        let assign9850_e12351: f64 = (p.p85 * locals.var_cox);
        let assign9850_e12353: f64 = (assign9850_e12351 - locals.var_cgsl_i);
        let assign9850_e12354: f64 = (0.0_f64).max(assign9850_e12353);
        (assign9850_e12354,)
    } else {
        (locals.var_cgso_i,)
    }
};
        locals.var_cgso_i = assign9850_e12356;
        locals.var_cgso_i_rv = 0.0;

        let assign9860_e12359: f64 = if p.p78 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard201 = assign9860_e12359;
        locals.var_guard201_rv = 0.0;

        let (assign9870_e12375,) = {
    if ((((locals.var_guard198 != 0.0) && (locals.var_guard199 == 0.0)) && (locals.var_guard200 == 0.0)) && (locals.var_guard201 != 0.0)) {
        let assign9870_e12371: f64 = (0.3 * p.p43);
        let assign9870_e12373: f64 = (assign9870_e12371 * locals.var_cox);
        (assign9870_e12373,)
    } else {
        (locals.var_cgso_i,)
    }
};
        locals.var_cgso_i = assign9870_e12375;
        locals.var_cgso_i_rv = 0.0;

        let (assign9880_e12392,) = {
    if ((((locals.var_guard198 != 0.0) && (locals.var_guard199 == 0.0)) && (locals.var_guard200 == 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign9880_e12388: f64 = (0.3 * p.p3);
        let assign9880_e12390: f64 = (assign9880_e12388 * locals.var_cox);
        (assign9880_e12390,)
    } else {
        (locals.var_cgso_i,)
    }
};
        locals.var_cgso_i = assign9880_e12392;
        locals.var_cgso_i_rv = 0.0;

        let assign9890_e12394: f64 = if param_given[1543] { 1.0 } else { 0.0 };
        locals.var_guard202 = assign9890_e12394;
        locals.var_guard202_rv = 0.0;

        let (assign9900_e12400,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard202 != 0.0)) {
        (p.p1543,)
    } else {
        (locals.var_cgdo_i,)
    }
};
        locals.var_cgdo_i = assign9900_e12400;
        locals.var_cgdo_i_rv = 0.0;

        let assign9910_e12406: f64 = if (param_given[85] && (p.p85 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard203 = assign9910_e12406;
        locals.var_guard203_rv = 0.0;

        let (assign9920_e12421,) = {
    if (((locals.var_guard198 != 0.0) && (locals.var_guard202 == 0.0)) && (locals.var_guard203 != 0.0)) {
        let assign9920_e12416: f64 = (p.p85 * locals.var_cox);
        let assign9920_e12418: f64 = (assign9920_e12416 - locals.var_cgdl_i);
        let assign9920_e12419: f64 = (0.0_f64).max(assign9920_e12418);
        (assign9920_e12419,)
    } else {
        (locals.var_cgdo_i,)
    }
};
        locals.var_cgdo_i = assign9920_e12421;
        locals.var_cgdo_i_rv = 0.0;

        let assign9930_e12424: f64 = if p.p78 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard204 = assign9930_e12424;
        locals.var_guard204_rv = 0.0;

        let (assign9940_e12440,) = {
    if ((((locals.var_guard198 != 0.0) && (locals.var_guard202 == 0.0)) && (locals.var_guard203 == 0.0)) && (locals.var_guard204 != 0.0)) {
        let assign9940_e12436: f64 = (0.3 * p.p43);
        let assign9940_e12438: f64 = (assign9940_e12436 * locals.var_cox);
        (assign9940_e12438,)
    } else {
        (locals.var_cgdo_i,)
    }
};
        locals.var_cgdo_i = assign9940_e12440;
        locals.var_cgdo_i_rv = 0.0;

        let (assign9950_e12457,) = {
    if ((((locals.var_guard198 != 0.0) && (locals.var_guard202 == 0.0)) && (locals.var_guard203 == 0.0)) && (locals.var_guard204 == 0.0)) {
        let assign9950_e12453: f64 = (0.3 * p.p3);
        let assign9950_e12455: f64 = (assign9950_e12453 * locals.var_cox);
        (assign9950_e12455,)
    } else {
        (locals.var_cgdo_i,)
    }
};
        locals.var_cgdo_i = assign9950_e12457;
        locals.var_cgdo_i_rv = 0.0;

        let assign9960_e12460: f64 = if p.p78 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard205 = assign9960_e12460;
        locals.var_guard205_rv = 0.0;

        let (assign9970_e12466,) = {
    if (locals.var_guard205 != 0.0) {
        let assign9970_e12464: f64 = (p.p1089 + p.p1090);
        (assign9970_e12464,)
    } else {
        (locals.var_hg,)
    }
};
        locals.var_hg = assign9970_e12466;
        locals.var_hg_rv = 0.0;

        let (assign9980_e12474,) = {
    if (locals.var_guard205 != 0.0) {
        let assign9980_e12471: f64 = (p.p4 - p.p3);
        let assign9980_e12472: f64 = (0.5 * assign9980_e12471);
        (assign9980_e12472,)
    } else {
        (locals.var_trsd,)
    }
};
        locals.var_trsd = assign9980_e12474;
        locals.var_trsd_rv = 0.0;

        let (assign9990_e12482,) = {
    if (locals.var_guard205 != 0.0) {
        let assign9990_e12479: f64 = (locals.var_trsd - p.p90);
        let assign9990_e12480: f64 = (0.0_f64).max(assign9990_e12479);
        (assign9990_e12480,)
    } else {
        (locals.var_wg,)
    }
};
        locals.var_wg = assign9990_e12482;
        locals.var_wg_rv = 0.0;

        let (assign10000_e12490,) = {
    if (locals.var_guard205 != 0.0) {
        let assign10000_e12487: f64 = (p.p1080 + p.p1081);
        let assign10000_e12488: f64 = (0.0_f64).max(assign10000_e12487);
        (assign10000_e12488,)
    } else {
        (locals.var_hrsd,)
    }
};
        locals.var_hrsd = assign10000_e12490;
        locals.var_hrsd_rv = 0.0;

        let assign10010_e12493: f64 = if p.p1090 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard206 = assign10010_e12493;
        locals.var_guard206_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10020_e12532, assign10020_e12532_d_n0, assign10020_e12532_d_n2, assign10020_e12532_d_n3, assign10020_e12532_d_n4, assign10020_e12532_d_n5, assign10020_e12532_d_n6, assign10020_e12532_d_n7, assign10020_e12532_d_n8, assign10020_e12532_d_n9, assign10020_e12532_d_n10, assign10020_e12532_d_n11, assign10020_e12532_d_n13, assign10020_e12532_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 != 0.0)) {
        let assign10020_e12500: f64 = (1e-7 * p.p1088);
        let assign10020_e12503: f64 = (3.9 * p.p1087);
        let assign10020_e12504: f64 = (assign10020_e12500 / assign10020_e12503);
        let (assign10020_e12529,) = {
            if (!(assign10020_e12504 > 1e-38)) {
                let assign10020_e12509: f64 = (-87.498233534);
                (assign10020_e12509,)
            } else {
                let assign10020_e12512: f64 = (1e-7 * p.p1088);
                let assign10020_e12515: f64 = (3.9 * p.p1087);
                let assign10020_e12516: f64 = (assign10020_e12512 / assign10020_e12515);
                let (assign10020_e12528,) = {
                    if (assign10020_e12516 > 1e-38) {
                        let assign10020_e12521: f64 = (1e-7 * p.p1088);
                        let assign10020_e12524: f64 = (3.9 * p.p1087);
                        let assign10020_e12525: f64 = (assign10020_e12521 / assign10020_e12524);
                        let assign10020_e12526: f64 = (assign10020_e12525).ln();
                        (assign10020_e12526,)
                    } else {
                        (0.0,)
                    }
                };
                (assign10020_e12528,)
            }
        };
        let assign10020_e12530: f64 = (3.467e-11 * assign10020_e12529);
        (assign10020_e12530, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign10020_e12532;
        locals.var_t0_dn0 = assign10020_e12532_d_n0;
        locals.var_t0_dn2 = assign10020_e12532_d_n2;
        locals.var_t0_dn3 = assign10020_e12532_d_n3;
        locals.var_t0_dn4 = assign10020_e12532_d_n4;
        locals.var_t0_dn5 = assign10020_e12532_d_n5;
        locals.var_t0_dn6 = assign10020_e12532_d_n6;
        locals.var_t0_dn7 = assign10020_e12532_d_n7;
        locals.var_t0_dn8 = assign10020_e12532_d_n8;
        locals.var_t0_dn9 = assign10020_e12532_d_n9;
        locals.var_t0_dn10 = assign10020_e12532_d_n10;
        locals.var_t0_dn11 = assign10020_e12532_d_n11;
        locals.var_t0_dn13 = assign10020_e12532_d_n13;
        locals.var_t0_dn14 = assign10020_e12532_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign10030_e12544, assign10030_e12544_d_n0, assign10030_e12544_d_n2, assign10030_e12544_d_n3, assign10030_e12544_d_n4, assign10030_e12544_d_n5, assign10030_e12544_d_n6, assign10030_e12544_d_n7, assign10030_e12544_d_n8, assign10030_e12544_d_n9, assign10030_e12544_d_n10, assign10030_e12544_d_n11, assign10030_e12544_d_n13, assign10030_e12544_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 != 0.0)) {
        let assign10030_e12538: f64 = (0.942 * locals.var_hrsd);
        let assign10030_e12540: f64 = (assign10030_e12538 * locals.var_epssp);
        let assign10030_e12542: f64 = (assign10030_e12540 / p.p1087);
        (assign10030_e12542, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign10030_e12544;
        locals.var_t1_dn0 = assign10030_e12544_d_n0;
        locals.var_t1_dn2 = assign10030_e12544_d_n2;
        locals.var_t1_dn3 = assign10030_e12544_d_n3;
        locals.var_t1_dn4 = assign10030_e12544_d_n4;
        locals.var_t1_dn5 = assign10030_e12544_d_n5;
        locals.var_t1_dn6 = assign10030_e12544_d_n6;
        locals.var_t1_dn7 = assign10030_e12544_d_n7;
        locals.var_t1_dn8 = assign10030_e12544_d_n8;
        locals.var_t1_dn9 = assign10030_e12544_d_n9;
        locals.var_t1_dn10 = assign10030_e12544_d_n10;
        locals.var_t1_dn11 = assign10030_e12544_d_n11;
        locals.var_t1_dn13 = assign10030_e12544_d_n13;
        locals.var_t1_dn14 = assign10030_e12544_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign10040_e12560, assign10040_e12560_d_n0, assign10040_e12560_d_n2, assign10040_e12560_d_n3, assign10040_e12560_d_n4, assign10040_e12560_d_n5, assign10040_e12560_d_n6, assign10040_e12560_d_n7, assign10040_e12560_d_n8, assign10040_e12560_d_n9, assign10040_e12560_d_n10, assign10040_e12560_d_n11, assign10040_e12560_d_n13, assign10040_e12560_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 != 0.0)) {
        let assign10040_e12550: f64 = (locals.var_t0 + locals.var_t1);
        let assign10040_e12554: f64 = (p.p4 - p.p3);
        let assign10040_e12556: f64 = (assign10040_e12554 * p.p1084);
        let assign10040_e12557: f64 = (p.p3 + assign10040_e12556);
        let assign10040_e12558: f64 = (assign10040_e12550 * assign10040_e12557);
        (assign10040_e12558, ((locals.var_t0_dn0 + locals.var_t1_dn0) * assign10040_e12557), ((locals.var_t0_dn2 + locals.var_t1_dn2) * assign10040_e12557), ((locals.var_t0_dn3 + locals.var_t1_dn3) * assign10040_e12557), ((locals.var_t0_dn4 + locals.var_t1_dn4) * assign10040_e12557), ((locals.var_t0_dn5 + locals.var_t1_dn5) * assign10040_e12557), ((locals.var_t0_dn6 + locals.var_t1_dn6) * assign10040_e12557), ((locals.var_t0_dn7 + locals.var_t1_dn7) * assign10040_e12557), ((locals.var_t0_dn8 + locals.var_t1_dn8) * assign10040_e12557), ((locals.var_t0_dn9 + locals.var_t1_dn9) * assign10040_e12557), ((locals.var_t0_dn10 + locals.var_t1_dn10) * assign10040_e12557), ((locals.var_t0_dn11 + locals.var_t1_dn11) * assign10040_e12557), ((locals.var_t0_dn13 + locals.var_t1_dn13) * assign10040_e12557), ((locals.var_t0_dn14 + locals.var_t1_dn14) * assign10040_e12557),)
    } else {
        (locals.var_cgg_top, locals.var_cgg_top_dn0, locals.var_cgg_top_dn2, locals.var_cgg_top_dn3, locals.var_cgg_top_dn4, locals.var_cgg_top_dn5, locals.var_cgg_top_dn6, locals.var_cgg_top_dn7, locals.var_cgg_top_dn8, locals.var_cgg_top_dn9, locals.var_cgg_top_dn10, locals.var_cgg_top_dn11, locals.var_cgg_top_dn13, locals.var_cgg_top_dn14,)
    }
};
        locals.var_cgg_top = assign10040_e12560;
        locals.var_cgg_top_dn0 = assign10040_e12560_d_n0;
        locals.var_cgg_top_dn2 = assign10040_e12560_d_n2;
        locals.var_cgg_top_dn3 = assign10040_e12560_d_n3;
        locals.var_cgg_top_dn4 = assign10040_e12560_d_n4;
        locals.var_cgg_top_dn5 = assign10040_e12560_d_n5;
        locals.var_cgg_top_dn6 = assign10040_e12560_d_n6;
        locals.var_cgg_top_dn7 = assign10040_e12560_d_n7;
        locals.var_cgg_top_dn8 = assign10040_e12560_d_n8;
        locals.var_cgg_top_dn9 = assign10040_e12560_d_n9;
        locals.var_cgg_top_dn10 = assign10040_e12560_d_n10;
        locals.var_cgg_top_dn11 = assign10040_e12560_d_n11;
        locals.var_cgg_top_dn13 = assign10040_e12560_d_n13;
        locals.var_cgg_top_dn14 = assign10040_e12560_d_n14;
        locals.var_cgg_top_rv = 0.0;

        let (assign10050_e12575,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10050_e12569: f64 = (locals.var_hg + p.p90);
        let assign10050_e12570: f64 = (0.2 * assign10050_e12569);
        let assign10050_e12572: f64 = (assign10050_e12570 / locals.var_hrsd);
        let assign10050_e12573: f64 = (2.3 + assign10050_e12572);
        (assign10050_e12573,)
    } else {
        (locals.var_hr,)
    }
};
        locals.var_hr = assign10050_e12575;
        locals.var_hr_rv = 0.0;

        let (assign10060_e12582,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        (1.05,)
    } else {
        (locals.var_lr,)
    }
};
        locals.var_lr = assign10060_e12582;
        locals.var_lr_rv = 0.0;

        let (assign10070_e12594,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10070_e12589: f64 = (locals.var_hg + p.p90);
        let assign10070_e12591: f64 = (assign10070_e12589 - locals.var_hrsd);
        let assign10070_e12592: f64 = (assign10070_e12591).abs();
        (assign10070_e12592,)
    } else {
        (locals.var_hgdelta,)
    }
};
        locals.var_hgdelta = assign10070_e12594;
        locals.var_hgdelta_rv = 0.0;

        let (assign10080_e12603,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10080_e12601: f64 = (p.p1087 * locals.var_lr);
        (assign10080_e12601,)
    } else {
        (locals.var_lmax,)
    }
};
        locals.var_lmax = assign10080_e12603;
        locals.var_lmax_rv = 0.0;

        let (assign10090_e12614,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10090_e12611: f64 = (locals.var_hg + p.p90);
        let assign10090_e12612: f64 = (locals.var_hrsd).min(assign10090_e12611);
        (assign10090_e12612,)
    } else {
        (locals.var_y,)
    }
};
        locals.var_y = assign10090_e12614;
        locals.var_y_rv = 0.0;

        let (assign10100_e12625,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10100_e12622: f64 = (locals.var_hr + 1.0);
        let assign10100_e12623: f64 = (p.p1087 / assign10100_e12622);
        (assign10100_e12623,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign10100_e12625;
        locals.var_x_rv = 0.0;

        let (assign10110_e12632,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        (1700000000000.0,)
    } else {
        (locals.var_cnon,)
    }
};
        locals.var_cnon = assign10110_e12632;
        locals.var_cnon_rv = 0.0;

        let (assign10120_e12645,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10120_e12640: f64 = (locals.var_y - locals.var_x);
        let assign10120_e12641: f64 = (locals.var_epssp * assign10120_e12640);
        let assign10120_e12643: f64 = (assign10120_e12641 / p.p1087);
        (assign10120_e12643,)
    } else {
        (locals.var_ccgsat,)
    }
};
        locals.var_ccgsat = assign10120_e12645;
        locals.var_ccgsat_rv = 0.0;

        let (assign10130_e12654, assign10130_e12654_d_n0, assign10130_e12654_d_n2, assign10130_e12654_d_n3, assign10130_e12654_d_n4, assign10130_e12654_d_n5, assign10130_e12654_d_n6, assign10130_e12654_d_n7, assign10130_e12654_d_n8, assign10130_e12654_d_n9, assign10130_e12654_d_n10, assign10130_e12654_d_n11, assign10130_e12654_d_n13, assign10130_e12654_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10130_e12652: f64 = (locals.var_cnon * locals.var_ccgsat);
        (assign10130_e12652, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign10130_e12654;
        locals.var_tt1_dn0 = assign10130_e12654_d_n0;
        locals.var_tt1_dn2 = assign10130_e12654_d_n2;
        locals.var_tt1_dn3 = assign10130_e12654_d_n3;
        locals.var_tt1_dn4 = assign10130_e12654_d_n4;
        locals.var_tt1_dn5 = assign10130_e12654_d_n5;
        locals.var_tt1_dn6 = assign10130_e12654_d_n6;
        locals.var_tt1_dn7 = assign10130_e12654_d_n7;
        locals.var_tt1_dn8 = assign10130_e12654_d_n8;
        locals.var_tt1_dn9 = assign10130_e12654_d_n9;
        locals.var_tt1_dn10 = assign10130_e12654_d_n10;
        locals.var_tt1_dn11 = assign10130_e12654_d_n11;
        locals.var_tt1_dn13 = assign10130_e12654_d_n13;
        locals.var_tt1_dn14 = assign10130_e12654_d_n14;
        locals.var_tt1_rv = 0.0;

        let assign10140_e12657: f64 = if locals.var_tt1 > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard207 = assign10140_e12657;
        locals.var_guard207_rv = 0.0;

        let (assign10150_e12666, assign10150_e12666_d_n0, assign10150_e12666_d_n2, assign10150_e12666_d_n3, assign10150_e12666_d_n4, assign10150_e12666_d_n5, assign10150_e12666_d_n6, assign10150_e12666_d_n7, assign10150_e12666_d_n8, assign10150_e12666_d_n9, assign10150_e12666_d_n10, assign10150_e12666_d_n11, assign10150_e12666_d_n13, assign10150_e12666_d_n14,) = {
    if (((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) && (locals.var_guard207 != 0.0)) {
        (locals.var_ccgsat, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign10150_e12666;
        locals.var_ccg1_dn0 = assign10150_e12666_d_n0;
        locals.var_ccg1_dn2 = assign10150_e12666_d_n2;
        locals.var_ccg1_dn3 = assign10150_e12666_d_n3;
        locals.var_ccg1_dn4 = assign10150_e12666_d_n4;
        locals.var_ccg1_dn5 = assign10150_e12666_d_n5;
        locals.var_ccg1_dn6 = assign10150_e12666_d_n6;
        locals.var_ccg1_dn7 = assign10150_e12666_d_n7;
        locals.var_ccg1_dn8 = assign10150_e12666_d_n8;
        locals.var_ccg1_dn9 = assign10150_e12666_d_n9;
        locals.var_ccg1_dn10 = assign10150_e12666_d_n10;
        locals.var_ccg1_dn11 = assign10150_e12666_d_n11;
        locals.var_ccg1_dn13 = assign10150_e12666_d_n13;
        locals.var_ccg1_dn14 = assign10150_e12666_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign10160_e12713, assign10160_e12713_d_n0, assign10160_e12713_d_n2, assign10160_e12713_d_n3, assign10160_e12713_d_n4, assign10160_e12713_d_n5, assign10160_e12713_d_n6, assign10160_e12713_d_n7, assign10160_e12713_d_n8, assign10160_e12713_d_n9, assign10160_e12713_d_n10, assign10160_e12713_d_n11, assign10160_e12713_d_n13, assign10160_e12713_d_n14,) = {
    if (((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) && (locals.var_guard207 == 0.0)) {
        let assign10160_e12676: f64 = (1.0 / locals.var_cnon);
        let assign10160_e12683: f64 = (-37.0);
        let (assign10160_e12710, assign10160_e12710_d_n0, assign10160_e12710_d_n2, assign10160_e12710_d_n3, assign10160_e12710_d_n4, assign10160_e12710_d_n5, assign10160_e12710_d_n6, assign10160_e12710_d_n7, assign10160_e12710_d_n8, assign10160_e12710_d_n9, assign10160_e12710_d_n10, assign10160_e12710_d_n11, assign10160_e12710_d_n13, assign10160_e12710_d_n14,) = {
            if ((!(locals.var_tt1 > 37.0)) && (!(locals.var_tt1 < assign10160_e12683))) {
                let assign10160_e12689: f64 = (locals.var_tt1).exp();
                let assign10160_e12690: f64 = (1.0 + assign10160_e12689);
                let assign10160_e12691: f64 = (assign10160_e12690).ln();
                (assign10160_e12691, ((assign10160_e12689 * locals.var_tt1_dn0) / assign10160_e12690), ((assign10160_e12689 * locals.var_tt1_dn2) / assign10160_e12690), ((assign10160_e12689 * locals.var_tt1_dn3) / assign10160_e12690), ((assign10160_e12689 * locals.var_tt1_dn4) / assign10160_e12690), ((assign10160_e12689 * locals.var_tt1_dn5) / assign10160_e12690), ((assign10160_e12689 * locals.var_tt1_dn6) / assign10160_e12690), ((assign10160_e12689 * locals.var_tt1_dn7) / assign10160_e12690), ((assign10160_e12689 * locals.var_tt1_dn8) / assign10160_e12690), ((assign10160_e12689 * locals.var_tt1_dn9) / assign10160_e12690), ((assign10160_e12689 * locals.var_tt1_dn10) / assign10160_e12690), ((assign10160_e12689 * locals.var_tt1_dn11) / assign10160_e12690), ((assign10160_e12689 * locals.var_tt1_dn13) / assign10160_e12690), ((assign10160_e12689 * locals.var_tt1_dn14) / assign10160_e12690),)
            } else {
                let assign10160_e12698: f64 = (-37.0);
                let (assign10160_e12709, assign10160_e12709_d_n0, assign10160_e12709_d_n2, assign10160_e12709_d_n3, assign10160_e12709_d_n4, assign10160_e12709_d_n5, assign10160_e12709_d_n6, assign10160_e12709_d_n7, assign10160_e12709_d_n8, assign10160_e12709_d_n9, assign10160_e12709_d_n10, assign10160_e12709_d_n11, assign10160_e12709_d_n13, assign10160_e12709_d_n14,) = {
                    if ((!(locals.var_tt1 > 37.0)) && (locals.var_tt1 < assign10160_e12698)) {
                        let assign10160_e12702: f64 = (locals.var_tt1).exp();
                        (assign10160_e12702, (assign10160_e12702 * locals.var_tt1_dn0), (assign10160_e12702 * locals.var_tt1_dn2), (assign10160_e12702 * locals.var_tt1_dn3), (assign10160_e12702 * locals.var_tt1_dn4), (assign10160_e12702 * locals.var_tt1_dn5), (assign10160_e12702 * locals.var_tt1_dn6), (assign10160_e12702 * locals.var_tt1_dn7), (assign10160_e12702 * locals.var_tt1_dn8), (assign10160_e12702 * locals.var_tt1_dn9), (assign10160_e12702 * locals.var_tt1_dn10), (assign10160_e12702 * locals.var_tt1_dn11), (assign10160_e12702 * locals.var_tt1_dn13), (assign10160_e12702 * locals.var_tt1_dn14),)
                    } else {
                        let (assign10160_e12708, assign10160_e12708_d_n0, assign10160_e12708_d_n2, assign10160_e12708_d_n3, assign10160_e12708_d_n4, assign10160_e12708_d_n5, assign10160_e12708_d_n6, assign10160_e12708_d_n7, assign10160_e12708_d_n8, assign10160_e12708_d_n9, assign10160_e12708_d_n10, assign10160_e12708_d_n11, assign10160_e12708_d_n13, assign10160_e12708_d_n14,) = {
                            if (locals.var_tt1 > 37.0) {
                                (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign10160_e12708, assign10160_e12708_d_n0, assign10160_e12708_d_n2, assign10160_e12708_d_n3, assign10160_e12708_d_n4, assign10160_e12708_d_n5, assign10160_e12708_d_n6, assign10160_e12708_d_n7, assign10160_e12708_d_n8, assign10160_e12708_d_n9, assign10160_e12708_d_n10, assign10160_e12708_d_n11, assign10160_e12708_d_n13, assign10160_e12708_d_n14,)
                    }
                };
                (assign10160_e12709, assign10160_e12709_d_n0, assign10160_e12709_d_n2, assign10160_e12709_d_n3, assign10160_e12709_d_n4, assign10160_e12709_d_n5, assign10160_e12709_d_n6, assign10160_e12709_d_n7, assign10160_e12709_d_n8, assign10160_e12709_d_n9, assign10160_e12709_d_n10, assign10160_e12709_d_n11, assign10160_e12709_d_n13, assign10160_e12709_d_n14,)
            }
        };
        let assign10160_e12711: f64 = (assign10160_e12676 * assign10160_e12710);
        (assign10160_e12711, (assign10160_e12676 * assign10160_e12710_d_n0), (assign10160_e12676 * assign10160_e12710_d_n2), (assign10160_e12676 * assign10160_e12710_d_n3), (assign10160_e12676 * assign10160_e12710_d_n4), (assign10160_e12676 * assign10160_e12710_d_n5), (assign10160_e12676 * assign10160_e12710_d_n6), (assign10160_e12676 * assign10160_e12710_d_n7), (assign10160_e12676 * assign10160_e12710_d_n8), (assign10160_e12676 * assign10160_e12710_d_n9), (assign10160_e12676 * assign10160_e12710_d_n10), (assign10160_e12676 * assign10160_e12710_d_n11), (assign10160_e12676 * assign10160_e12710_d_n13), (assign10160_e12676 * assign10160_e12710_d_n14),)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign10160_e12713;
        locals.var_ccg1_dn0 = assign10160_e12713_d_n0;
        locals.var_ccg1_dn2 = assign10160_e12713_d_n2;
        locals.var_ccg1_dn3 = assign10160_e12713_d_n3;
        locals.var_ccg1_dn4 = assign10160_e12713_d_n4;
        locals.var_ccg1_dn5 = assign10160_e12713_d_n5;
        locals.var_ccg1_dn6 = assign10160_e12713_d_n6;
        locals.var_ccg1_dn7 = assign10160_e12713_d_n7;
        locals.var_ccg1_dn8 = assign10160_e12713_d_n8;
        locals.var_ccg1_dn9 = assign10160_e12713_d_n9;
        locals.var_ccg1_dn10 = assign10160_e12713_d_n10;
        locals.var_ccg1_dn11 = assign10160_e12713_d_n11;
        locals.var_ccg1_dn13 = assign10160_e12713_d_n13;
        locals.var_ccg1_dn14 = assign10160_e12713_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign10170_e12732,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10170_e12722: f64 = (locals.var_hg + p.p90);
        let assign10170_e12723: f64 = (locals.var_hrsd / assign10170_e12722);
        let assign10170_e12726: f64 = (locals.var_hg + p.p90);
        let assign10170_e12728: f64 = (assign10170_e12726 / locals.var_hrsd);
        let assign10170_e12729: f64 = (assign10170_e12723).min(assign10170_e12728);
        let assign10170_e12730: f64 = (0.5 * assign10170_e12729);
        (assign10170_e12730,)
    } else {
        (locals.var_r1cf,)
    }
};
        locals.var_r1cf = assign10170_e12732;
        locals.var_r1cf_rv = 0.0;

        let (assign10180_e12741,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10180_e12739: f64 = (locals.var_hgdelta * locals.var_r1cf);
        (assign10180_e12739,)
    } else {
        (locals.var_rcf,)
    }
};
        locals.var_rcf = assign10180_e12741;
        locals.var_rcf_rv = 0.0;

        let (assign10190_e12791,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10190_e12748: f64 = (locals.var_epssp * 2.0);
        let assign10190_e12750: f64 = (assign10190_e12748 / 3.141592653589793);
        let assign10190_e12754: f64 = (0.5 * 3.141592653589793);
        let assign10190_e12756: f64 = (assign10190_e12754 * locals.var_rcf);
        let assign10190_e12757: f64 = (p.p1087 + assign10190_e12756);
        let assign10190_e12759: f64 = (assign10190_e12757 / p.p1087);
        let (assign10190_e12788,) = {
            if (!(assign10190_e12759 > 1e-38)) {
                let assign10190_e12764: f64 = (-87.498233534);
                (assign10190_e12764,)
            } else {
                let assign10190_e12768: f64 = (0.5 * 3.141592653589793);
                let assign10190_e12770: f64 = (assign10190_e12768 * locals.var_rcf);
                let assign10190_e12771: f64 = (p.p1087 + assign10190_e12770);
                let assign10190_e12773: f64 = (assign10190_e12771 / p.p1087);
                let (assign10190_e12787,) = {
                    if (assign10190_e12773 > 1e-38) {
                        let assign10190_e12779: f64 = (0.5 * 3.141592653589793);
                        let assign10190_e12781: f64 = (assign10190_e12779 * locals.var_rcf);
                        let assign10190_e12782: f64 = (p.p1087 + assign10190_e12781);
                        let assign10190_e12784: f64 = (assign10190_e12782 / p.p1087);
                        let assign10190_e12785: f64 = (assign10190_e12784).ln();
                        (assign10190_e12785,)
                    } else {
                        (0.0,)
                    }
                };
                (assign10190_e12787,)
            }
        };
        let assign10190_e12789: f64 = (assign10190_e12750 * assign10190_e12788);
        (assign10190_e12789,)
    } else {
        (locals.var_ccg2,)
    }
};
        locals.var_ccg2 = assign10190_e12791;
        locals.var_ccg2_rv = 0.0;

        let (assign10200_e12802, assign10200_e12802_d_n0, assign10200_e12802_d_n2, assign10200_e12802_d_n3, assign10200_e12802_d_n4, assign10200_e12802_d_n5, assign10200_e12802_d_n6, assign10200_e12802_d_n7, assign10200_e12802_d_n8, assign10200_e12802_d_n9, assign10200_e12802_d_n10, assign10200_e12802_d_n11, assign10200_e12802_d_n13, assign10200_e12802_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10200_e12799: f64 = (locals.var_ccg1 + locals.var_ccg2);
        let assign10200_e12800: f64 = (p.p3 * assign10200_e12799);
        (assign10200_e12800, (p.p3 * locals.var_ccg1_dn0), (p.p3 * locals.var_ccg1_dn2), (p.p3 * locals.var_ccg1_dn3), (p.p3 * locals.var_ccg1_dn4), (p.p3 * locals.var_ccg1_dn5), (p.p3 * locals.var_ccg1_dn6), (p.p3 * locals.var_ccg1_dn7), (p.p3 * locals.var_ccg1_dn8), (p.p3 * locals.var_ccg1_dn9), (p.p3 * locals.var_ccg1_dn10), (p.p3 * locals.var_ccg1_dn11), (p.p3 * locals.var_ccg1_dn13), (p.p3 * locals.var_ccg1_dn14),)
    } else {
        (locals.var_ccg, locals.var_ccg_dn0, locals.var_ccg_dn2, locals.var_ccg_dn3, locals.var_ccg_dn4, locals.var_ccg_dn5, locals.var_ccg_dn6, locals.var_ccg_dn7, locals.var_ccg_dn8, locals.var_ccg_dn9, locals.var_ccg_dn10, locals.var_ccg_dn11, locals.var_ccg_dn13, locals.var_ccg_dn14,)
    }
};
        locals.var_ccg = assign10200_e12802;
        locals.var_ccg_dn0 = assign10200_e12802_d_n0;
        locals.var_ccg_dn2 = assign10200_e12802_d_n2;
        locals.var_ccg_dn3 = assign10200_e12802_d_n3;
        locals.var_ccg_dn4 = assign10200_e12802_d_n4;
        locals.var_ccg_dn5 = assign10200_e12802_d_n5;
        locals.var_ccg_dn6 = assign10200_e12802_d_n6;
        locals.var_ccg_dn7 = assign10200_e12802_d_n7;
        locals.var_ccg_dn8 = assign10200_e12802_d_n8;
        locals.var_ccg_dn9 = assign10200_e12802_d_n9;
        locals.var_ccg_dn10 = assign10200_e12802_d_n10;
        locals.var_ccg_dn11 = assign10200_e12802_d_n11;
        locals.var_ccg_dn13 = assign10200_e12802_d_n13;
        locals.var_ccg_dn14 = assign10200_e12802_d_n14;
        locals.var_ccg_rv = 0.0;

        let (assign10210_e12811,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10210_e12809: f64 = (locals.var_lmax / locals.var_hg);
        (assign10210_e12809,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign10210_e12811;
        locals.var_x_rv = 0.0;

        let (assign10220_e12827,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10220_e12820: f64 = (locals.var_x + 1.0);
        let assign10220_e12821: f64 = (2.0 * assign10220_e12820);
        let assign10220_e12822: f64 = (assign10220_e12821).sqrt();
        let assign10220_e12824: f64 = (assign10220_e12822 * 3.141592653589793);
        let assign10220_e12825: f64 = (4.0 / assign10220_e12824);
        (assign10220_e12825,)
    } else {
        (locals.var_c1,)
    }
};
        locals.var_c1 = assign10220_e12827;
        locals.var_c1_rv = 0.0;

        let (assign10230_e12864,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10230_e12834: f64 = (p.p90 * p.p90);
        let assign10230_e12837: f64 = (2.0 * locals.var_hg);
        let assign10230_e12839: f64 = (assign10230_e12837 * p.p90);
        let assign10230_e12840: f64 = (assign10230_e12834 + assign10230_e12839);
        let assign10230_e12843: f64 = (locals.var_hg * locals.var_hg);
        let assign10230_e12846: f64 = (locals.var_x + 1.0);
        let assign10230_e12847: f64 = (assign10230_e12843 * assign10230_e12846);
        let assign10230_e12848: f64 = (assign10230_e12840 + assign10230_e12847);
        let assign10230_e12849: f64 = (assign10230_e12848).sqrt();
        let assign10230_e12852: f64 = (locals.var_x + 1.0);
        let assign10230_e12853: f64 = (assign10230_e12852).sqrt();
        let assign10230_e12854: f64 = (assign10230_e12849 * assign10230_e12853);
        let assign10230_e12856: f64 = (assign10230_e12854 + p.p90);
        let assign10230_e12859: f64 = (locals.var_hg * locals.var_x);
        let assign10230_e12860: f64 = (assign10230_e12856 + assign10230_e12859);
        let assign10230_e12862: f64 = (assign10230_e12860 + locals.var_hg);
        (assign10230_e12862,)
    } else {
        (locals.var_c2,)
    }
};
        locals.var_c2 = assign10230_e12864;
        locals.var_c2_rv = 0.0;

        let (assign10240_e12886,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10240_e12872: f64 = (locals.var_x + 1.0);
        let assign10240_e12875: f64 = (locals.var_x + 4.0);
        let assign10240_e12876: f64 = (assign10240_e12872 * assign10240_e12875);
        let assign10240_e12877: f64 = (assign10240_e12876).sqrt();
        let assign10240_e12878: f64 = (p.p90 * assign10240_e12877);
        let assign10240_e12882: f64 = (locals.var_x + 2.0);
        let assign10240_e12883: f64 = (p.p90 * assign10240_e12882);
        let assign10240_e12884: f64 = (assign10240_e12878 + assign10240_e12883);
        (assign10240_e12884,)
    } else {
        (locals.var_c3,)
    }
};
        locals.var_c3 = assign10240_e12886;
        locals.var_c3_rv = 0.0;

        let (assign10250_e12918,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10250_e12895: f64 = (locals.var_c2 / locals.var_c3);
        let (assign10250_e12912,) = {
            if (!(assign10250_e12895 > 1e-38)) {
                let assign10250_e12900: f64 = (-87.498233534);
                (assign10250_e12900,)
            } else {
                let assign10250_e12903: f64 = (locals.var_c2 / locals.var_c3);
                let (assign10250_e12911,) = {
                    if (assign10250_e12903 > 1e-38) {
                        let assign10250_e12908: f64 = (locals.var_c2 / locals.var_c3);
                        let assign10250_e12909: f64 = (assign10250_e12908).ln();
                        (assign10250_e12909,)
                    } else {
                        (0.0,)
                    }
                };
                (assign10250_e12911,)
            }
        };
        let assign10250_e12913: f64 = (locals.var_c1 * assign10250_e12912);
        let assign10250_e12915: f64 = (assign10250_e12913 + 12.27);
        let assign10250_e12916: f64 = (locals.var_epssp * assign10250_e12915);
        (assign10250_e12916,)
    } else {
        (locals.var_cfglog,)
    }
};
        locals.var_cfglog = assign10250_e12918;
        locals.var_cfglog_rv = 0.0;

        let (assign10260_e12927,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10260_e12925: f64 = (locals.var_hr * locals.var_lr);
        (assign10260_e12925,)
    } else {
        (locals.var_dcf,)
    }
};
        locals.var_dcf = assign10260_e12927;
        locals.var_dcf_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10270_e12939,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10270_e12934: f64 = (locals.var_dcf * locals.var_dcf);
        let assign10270_e12936: f64 = (assign10270_e12934 + 1.0);
        let assign10270_e12937: f64 = (assign10270_e12936).sqrt();
        (assign10270_e12937,)
    } else {
        (locals.var_tt0,)
    }
};
        locals.var_tt0 = assign10270_e12939;
        locals.var_tt0_rv = 0.0;

        let (assign10280_e12989, assign10280_e12989_d_n0, assign10280_e12989_d_n2, assign10280_e12989_d_n3, assign10280_e12989_d_n4, assign10280_e12989_d_n5, assign10280_e12989_d_n6, assign10280_e12989_d_n7, assign10280_e12989_d_n8, assign10280_e12989_d_n9, assign10280_e12989_d_n10, assign10280_e12989_d_n11, assign10280_e12989_d_n13, assign10280_e12989_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10280_e12946: f64 = (locals.var_dcf * locals.var_dcf);
        let assign10280_e12948: f64 = (assign10280_e12946 + 1.0);
        let assign10280_e12951: f64 = (locals.var_dcf * p.p90);
        let assign10280_e12954: f64 = (locals.var_dcf * p.p90);
        let assign10280_e12955: f64 = (assign10280_e12951 * assign10280_e12954);
        let assign10280_e12958: f64 = (2.0 * locals.var_dcf);
        let assign10280_e12960: f64 = (assign10280_e12958 * locals.var_lmax);
        let assign10280_e12962: f64 = (assign10280_e12960 * p.p90);
        let assign10280_e12963: f64 = (assign10280_e12955 + assign10280_e12962);
        let assign10280_e12966: f64 = (locals.var_dcf * locals.var_dcf);
        let assign10280_e12968: f64 = (assign10280_e12966 + 1.0);
        let assign10280_e12970: f64 = (assign10280_e12968 * locals.var_lmax);
        let assign10280_e12972: f64 = (assign10280_e12970 * locals.var_lmax);
        let assign10280_e12973: f64 = (assign10280_e12963 + assign10280_e12972);
        let assign10280_e12974: f64 = (assign10280_e12948 * assign10280_e12973);
        let assign10280_e12975: f64 = (assign10280_e12974).sqrt();
        let assign10280_e12978: f64 = (locals.var_dcf * p.p90);
        let assign10280_e12979: f64 = (assign10280_e12975 + assign10280_e12978);
        let assign10280_e12982: f64 = (locals.var_dcf * locals.var_dcf);
        let assign10280_e12984: f64 = (assign10280_e12982 * locals.var_lmax);
        let assign10280_e12985: f64 = (assign10280_e12979 + assign10280_e12984);
        let assign10280_e12987: f64 = (assign10280_e12985 + locals.var_lmax);
        (assign10280_e12987, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign10280_e12989;
        locals.var_tt1_dn0 = assign10280_e12989_d_n0;
        locals.var_tt1_dn2 = assign10280_e12989_d_n2;
        locals.var_tt1_dn3 = assign10280_e12989_d_n3;
        locals.var_tt1_dn4 = assign10280_e12989_d_n4;
        locals.var_tt1_dn5 = assign10280_e12989_d_n5;
        locals.var_tt1_dn6 = assign10280_e12989_d_n6;
        locals.var_tt1_dn7 = assign10280_e12989_d_n7;
        locals.var_tt1_dn8 = assign10280_e12989_d_n8;
        locals.var_tt1_dn9 = assign10280_e12989_d_n9;
        locals.var_tt1_dn10 = assign10280_e12989_d_n10;
        locals.var_tt1_dn11 = assign10280_e12989_d_n11;
        locals.var_tt1_dn13 = assign10280_e12989_d_n13;
        locals.var_tt1_dn14 = assign10280_e12989_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign10290_e13002,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10290_e12996: f64 = (locals.var_tt0 + 1.0);
        let assign10290_e12999: f64 = (locals.var_dcf * p.p90);
        let assign10290_e13000: f64 = (assign10290_e12996 * assign10290_e12999);
        (assign10290_e13000,)
    } else {
        (locals.var_tt2,)
    }
};
        locals.var_tt2 = assign10290_e13002;
        locals.var_tt2_rv = 0.0;

        let (assign10300_e13043, assign10300_e13043_d_n0, assign10300_e13043_d_n2, assign10300_e13043_d_n3, assign10300_e13043_d_n4, assign10300_e13043_d_n5, assign10300_e13043_d_n6, assign10300_e13043_d_n7, assign10300_e13043_d_n8, assign10300_e13043_d_n9, assign10300_e13043_d_n10, assign10300_e13043_d_n11, assign10300_e13043_d_n13, assign10300_e13043_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10300_e13009: f64 = (2.0 * locals.var_epssp);
        let assign10300_e13011: f64 = (2.0_f64).sqrt();
        let assign10300_e13012: f64 = (assign10300_e13009 * assign10300_e13011);
        let assign10300_e13014: f64 = (assign10300_e13012 / 3.141592653589793);
        let assign10300_e13016: f64 = (assign10300_e13014 * 0.85);
        let assign10300_e13018: f64 = (assign10300_e13016 * locals.var_dcf);
        let assign10300_e13020: f64 = (assign10300_e13018 / locals.var_tt0);
        let assign10300_e13023: f64 = (locals.var_tt1 / locals.var_tt2);
        let (assign10300_e13040, assign10300_e13040_d_n0, assign10300_e13040_d_n2, assign10300_e13040_d_n3, assign10300_e13040_d_n4, assign10300_e13040_d_n5, assign10300_e13040_d_n6, assign10300_e13040_d_n7, assign10300_e13040_d_n8, assign10300_e13040_d_n9, assign10300_e13040_d_n10, assign10300_e13040_d_n11, assign10300_e13040_d_n13, assign10300_e13040_d_n14,) = {
            if (!(assign10300_e13023 > 1e-38)) {
                let assign10300_e13028: f64 = (-87.498233534);
                (assign10300_e13028, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign10300_e13031: f64 = (locals.var_tt1 / locals.var_tt2);
                let (assign10300_e13039, assign10300_e13039_d_n0, assign10300_e13039_d_n2, assign10300_e13039_d_n3, assign10300_e13039_d_n4, assign10300_e13039_d_n5, assign10300_e13039_d_n6, assign10300_e13039_d_n7, assign10300_e13039_d_n8, assign10300_e13039_d_n9, assign10300_e13039_d_n10, assign10300_e13039_d_n11, assign10300_e13039_d_n13, assign10300_e13039_d_n14,) = {
                    if (assign10300_e13031 > 1e-38) {
                        let assign10300_e13036: f64 = (locals.var_tt1 / locals.var_tt2);
                        let assign10300_e13037: f64 = (assign10300_e13036).ln();
                        (assign10300_e13037, ((locals.var_tt1_dn0 / locals.var_tt2) / assign10300_e13036), ((locals.var_tt1_dn2 / locals.var_tt2) / assign10300_e13036), ((locals.var_tt1_dn3 / locals.var_tt2) / assign10300_e13036), ((locals.var_tt1_dn4 / locals.var_tt2) / assign10300_e13036), ((locals.var_tt1_dn5 / locals.var_tt2) / assign10300_e13036), ((locals.var_tt1_dn6 / locals.var_tt2) / assign10300_e13036), ((locals.var_tt1_dn7 / locals.var_tt2) / assign10300_e13036), ((locals.var_tt1_dn8 / locals.var_tt2) / assign10300_e13036), ((locals.var_tt1_dn9 / locals.var_tt2) / assign10300_e13036), ((locals.var_tt1_dn10 / locals.var_tt2) / assign10300_e13036), ((locals.var_tt1_dn11 / locals.var_tt2) / assign10300_e13036), ((locals.var_tt1_dn13 / locals.var_tt2) / assign10300_e13036), ((locals.var_tt1_dn14 / locals.var_tt2) / assign10300_e13036),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign10300_e13039, assign10300_e13039_d_n0, assign10300_e13039_d_n2, assign10300_e13039_d_n3, assign10300_e13039_d_n4, assign10300_e13039_d_n5, assign10300_e13039_d_n6, assign10300_e13039_d_n7, assign10300_e13039_d_n8, assign10300_e13039_d_n9, assign10300_e13039_d_n10, assign10300_e13039_d_n11, assign10300_e13039_d_n13, assign10300_e13039_d_n14,)
            }
        };
        let assign10300_e13041: f64 = (assign10300_e13020 * assign10300_e13040);
        (assign10300_e13041, (assign10300_e13020 * assign10300_e13040_d_n0), (assign10300_e13020 * assign10300_e13040_d_n2), (assign10300_e13020 * assign10300_e13040_d_n3), (assign10300_e13020 * assign10300_e13040_d_n4), (assign10300_e13020 * assign10300_e13040_d_n5), (assign10300_e13020 * assign10300_e13040_d_n6), (assign10300_e13020 * assign10300_e13040_d_n7), (assign10300_e13020 * assign10300_e13040_d_n8), (assign10300_e13020 * assign10300_e13040_d_n9), (assign10300_e13020 * assign10300_e13040_d_n10), (assign10300_e13020 * assign10300_e13040_d_n11), (assign10300_e13020 * assign10300_e13040_d_n13), (assign10300_e13020 * assign10300_e13040_d_n14),)
    } else {
        (locals.var_cfgsat, locals.var_cfgsat_dn0, locals.var_cfgsat_dn2, locals.var_cfgsat_dn3, locals.var_cfgsat_dn4, locals.var_cfgsat_dn5, locals.var_cfgsat_dn6, locals.var_cfgsat_dn7, locals.var_cfgsat_dn8, locals.var_cfgsat_dn9, locals.var_cfgsat_dn10, locals.var_cfgsat_dn11, locals.var_cfgsat_dn13, locals.var_cfgsat_dn14,)
    }
};
        locals.var_cfgsat = assign10300_e13043;
        locals.var_cfgsat_dn0 = assign10300_e13043_d_n0;
        locals.var_cfgsat_dn2 = assign10300_e13043_d_n2;
        locals.var_cfgsat_dn3 = assign10300_e13043_d_n3;
        locals.var_cfgsat_dn4 = assign10300_e13043_d_n4;
        locals.var_cfgsat_dn5 = assign10300_e13043_d_n5;
        locals.var_cfgsat_dn6 = assign10300_e13043_d_n6;
        locals.var_cfgsat_dn7 = assign10300_e13043_d_n7;
        locals.var_cfgsat_dn8 = assign10300_e13043_d_n8;
        locals.var_cfgsat_dn9 = assign10300_e13043_d_n9;
        locals.var_cfgsat_dn10 = assign10300_e13043_d_n10;
        locals.var_cfgsat_dn11 = assign10300_e13043_d_n11;
        locals.var_cfgsat_dn13 = assign10300_e13043_d_n13;
        locals.var_cfgsat_dn14 = assign10300_e13043_d_n14;
        locals.var_cfgsat_rv = 0.0;

        let (assign10310_e13050, assign10310_e13050_d_n0, assign10310_e13050_d_n2, assign10310_e13050_d_n3, assign10310_e13050_d_n4, assign10310_e13050_d_n5, assign10310_e13050_d_n6, assign10310_e13050_d_n7, assign10310_e13050_d_n8, assign10310_e13050_d_n9, assign10310_e13050_d_n10, assign10310_e13050_d_n11, assign10310_e13050_d_n13, assign10310_e13050_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        (1.2e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign10310_e13050;
        locals.var_delta_dn0 = assign10310_e13050_d_n0;
        locals.var_delta_dn2 = assign10310_e13050_d_n2;
        locals.var_delta_dn3 = assign10310_e13050_d_n3;
        locals.var_delta_dn4 = assign10310_e13050_d_n4;
        locals.var_delta_dn5 = assign10310_e13050_d_n5;
        locals.var_delta_dn6 = assign10310_e13050_d_n6;
        locals.var_delta_dn7 = assign10310_e13050_d_n7;
        locals.var_delta_dn8 = assign10310_e13050_d_n8;
        locals.var_delta_dn9 = assign10310_e13050_d_n9;
        locals.var_delta_dn10 = assign10310_e13050_d_n10;
        locals.var_delta_dn11 = assign10310_e13050_d_n11;
        locals.var_delta_dn13 = assign10310_e13050_d_n13;
        locals.var_delta_dn14 = assign10310_e13050_d_n14;
        locals.var_delta_rv = 0.0;

        let (assign10320_e13061, assign10320_e13061_d_n0, assign10320_e13061_d_n2, assign10320_e13061_d_n3, assign10320_e13061_d_n4, assign10320_e13061_d_n5, assign10320_e13061_d_n6, assign10320_e13061_d_n7, assign10320_e13061_d_n8, assign10320_e13061_d_n9, assign10320_e13061_d_n10, assign10320_e13061_d_n11, assign10320_e13061_d_n13, assign10320_e13061_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10320_e13057: f64 = (locals.var_cfgsat - locals.var_cfglog);
        let assign10320_e13059: f64 = (assign10320_e13057 - locals.var_delta);
        (assign10320_e13059, (locals.var_cfgsat_dn0 - locals.var_delta_dn0), (locals.var_cfgsat_dn2 - locals.var_delta_dn2), (locals.var_cfgsat_dn3 - locals.var_delta_dn3), (locals.var_cfgsat_dn4 - locals.var_delta_dn4), (locals.var_cfgsat_dn5 - locals.var_delta_dn5), (locals.var_cfgsat_dn6 - locals.var_delta_dn6), (locals.var_cfgsat_dn7 - locals.var_delta_dn7), (locals.var_cfgsat_dn8 - locals.var_delta_dn8), (locals.var_cfgsat_dn9 - locals.var_delta_dn9), (locals.var_cfgsat_dn10 - locals.var_delta_dn10), (locals.var_cfgsat_dn11 - locals.var_delta_dn11), (locals.var_cfgsat_dn13 - locals.var_delta_dn13), (locals.var_cfgsat_dn14 - locals.var_delta_dn14),)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign10320_e13061;
        locals.var_tt1_dn0 = assign10320_e13061_d_n0;
        locals.var_tt1_dn2 = assign10320_e13061_d_n2;
        locals.var_tt1_dn3 = assign10320_e13061_d_n3;
        locals.var_tt1_dn4 = assign10320_e13061_d_n4;
        locals.var_tt1_dn5 = assign10320_e13061_d_n5;
        locals.var_tt1_dn6 = assign10320_e13061_d_n6;
        locals.var_tt1_dn7 = assign10320_e13061_d_n7;
        locals.var_tt1_dn8 = assign10320_e13061_d_n8;
        locals.var_tt1_dn9 = assign10320_e13061_d_n9;
        locals.var_tt1_dn10 = assign10320_e13061_d_n10;
        locals.var_tt1_dn11 = assign10320_e13061_d_n11;
        locals.var_tt1_dn13 = assign10320_e13061_d_n13;
        locals.var_tt1_dn14 = assign10320_e13061_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign10330_e13085, assign10330_e13085_d_n0, assign10330_e13085_d_n2, assign10330_e13085_d_n3, assign10330_e13085_d_n4, assign10330_e13085_d_n5, assign10330_e13085_d_n6, assign10330_e13085_d_n7, assign10330_e13085_d_n8, assign10330_e13085_d_n9, assign10330_e13085_d_n10, assign10330_e13085_d_n11, assign10330_e13085_d_n13, assign10330_e13085_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10330_e13072: f64 = (locals.var_tt1 * locals.var_tt1);
        let assign10330_e13075: f64 = (4.0 * locals.var_delta);
        let assign10330_e13077: f64 = (assign10330_e13075 * locals.var_cfgsat);
        let assign10330_e13078: f64 = (assign10330_e13072 + assign10330_e13077);
        let assign10330_e13079: f64 = (assign10330_e13078).sqrt();
        let assign10330_e13080: f64 = (locals.var_tt1 + assign10330_e13079);
        let assign10330_e13081: f64 = (0.5 * assign10330_e13080);
        let assign10330_e13082: f64 = (locals.var_cfgsat - assign10330_e13081);
        let assign10330_e13083: f64 = (p.p3 * assign10330_e13082);
        (assign10330_e13083, (p.p3 * (locals.var_cfgsat_dn0 - (0.5 * (locals.var_tt1_dn0 + ((((locals.var_tt1_dn0 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn0))) / (2.0 * assign10330_e13079)))))), (p.p3 * (locals.var_cfgsat_dn2 - (0.5 * (locals.var_tt1_dn2 + ((((locals.var_tt1_dn2 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn2))) / (2.0 * assign10330_e13079)))))), (p.p3 * (locals.var_cfgsat_dn3 - (0.5 * (locals.var_tt1_dn3 + ((((locals.var_tt1_dn3 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn3)) + (((4.0 * locals.var_delta_dn3) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn3))) / (2.0 * assign10330_e13079)))))), (p.p3 * (locals.var_cfgsat_dn4 - (0.5 * (locals.var_tt1_dn4 + ((((locals.var_tt1_dn4 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn4)) + (((4.0 * locals.var_delta_dn4) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn4))) / (2.0 * assign10330_e13079)))))), (p.p3 * (locals.var_cfgsat_dn5 - (0.5 * (locals.var_tt1_dn5 + ((((locals.var_tt1_dn5 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn5)) + (((4.0 * locals.var_delta_dn5) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn5))) / (2.0 * assign10330_e13079)))))), (p.p3 * (locals.var_cfgsat_dn6 - (0.5 * (locals.var_tt1_dn6 + ((((locals.var_tt1_dn6 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn6))) / (2.0 * assign10330_e13079)))))), (p.p3 * (locals.var_cfgsat_dn7 - (0.5 * (locals.var_tt1_dn7 + ((((locals.var_tt1_dn7 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn7))) / (2.0 * assign10330_e13079)))))), (p.p3 * (locals.var_cfgsat_dn8 - (0.5 * (locals.var_tt1_dn8 + ((((locals.var_tt1_dn8 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn8)) + (((4.0 * locals.var_delta_dn8) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn8))) / (2.0 * assign10330_e13079)))))), (p.p3 * (locals.var_cfgsat_dn9 - (0.5 * (locals.var_tt1_dn9 + ((((locals.var_tt1_dn9 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn9)) + (((4.0 * locals.var_delta_dn9) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn9))) / (2.0 * assign10330_e13079)))))), (p.p3 * (locals.var_cfgsat_dn10 - (0.5 * (locals.var_tt1_dn10 + ((((locals.var_tt1_dn10 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn10))) / (2.0 * assign10330_e13079)))))), (p.p3 * (locals.var_cfgsat_dn11 - (0.5 * (locals.var_tt1_dn11 + ((((locals.var_tt1_dn11 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn11))) / (2.0 * assign10330_e13079)))))), (p.p3 * (locals.var_cfgsat_dn13 - (0.5 * (locals.var_tt1_dn13 + ((((locals.var_tt1_dn13 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn13)) + (((4.0 * locals.var_delta_dn13) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn13))) / (2.0 * assign10330_e13079)))))), (p.p3 * (locals.var_cfgsat_dn14 - (0.5 * (locals.var_tt1_dn14 + ((((locals.var_tt1_dn14 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn14)) + (((4.0 * locals.var_delta_dn14) * locals.var_cfgsat) + (assign10330_e13075 * locals.var_cfgsat_dn14))) / (2.0 * assign10330_e13079)))))),)
    } else {
        (locals.var_cfg, locals.var_cfg_dn0, locals.var_cfg_dn2, locals.var_cfg_dn3, locals.var_cfg_dn4, locals.var_cfg_dn5, locals.var_cfg_dn6, locals.var_cfg_dn7, locals.var_cfg_dn8, locals.var_cfg_dn9, locals.var_cfg_dn10, locals.var_cfg_dn11, locals.var_cfg_dn13, locals.var_cfg_dn14,)
    }
};
        locals.var_cfg = assign10330_e13085;
        locals.var_cfg_dn0 = assign10330_e13085_d_n0;
        locals.var_cfg_dn2 = assign10330_e13085_d_n2;
        locals.var_cfg_dn3 = assign10330_e13085_d_n3;
        locals.var_cfg_dn4 = assign10330_e13085_d_n4;
        locals.var_cfg_dn5 = assign10330_e13085_d_n5;
        locals.var_cfg_dn6 = assign10330_e13085_d_n6;
        locals.var_cfg_dn7 = assign10330_e13085_d_n7;
        locals.var_cfg_dn8 = assign10330_e13085_d_n8;
        locals.var_cfg_dn9 = assign10330_e13085_d_n9;
        locals.var_cfg_dn10 = assign10330_e13085_d_n10;
        locals.var_cfg_dn11 = assign10330_e13085_d_n11;
        locals.var_cfg_dn13 = assign10330_e13085_d_n13;
        locals.var_cfg_dn14 = assign10330_e13085_d_n14;
        locals.var_cfg_rv = 0.0;

        let (assign10340_e13094, assign10340_e13094_d_n0, assign10340_e13094_d_n2, assign10340_e13094_d_n3, assign10340_e13094_d_n4, assign10340_e13094_d_n5, assign10340_e13094_d_n6, assign10340_e13094_d_n7, assign10340_e13094_d_n8, assign10340_e13094_d_n9, assign10340_e13094_d_n10, assign10340_e13094_d_n11, assign10340_e13094_d_n13, assign10340_e13094_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard206 == 0.0)) {
        let assign10340_e13092: f64 = (locals.var_ccg + locals.var_cfg);
        (assign10340_e13092, (locals.var_ccg_dn0 + locals.var_cfg_dn0), (locals.var_ccg_dn2 + locals.var_cfg_dn2), (locals.var_ccg_dn3 + locals.var_cfg_dn3), (locals.var_ccg_dn4 + locals.var_cfg_dn4), (locals.var_ccg_dn5 + locals.var_cfg_dn5), (locals.var_ccg_dn6 + locals.var_cfg_dn6), (locals.var_ccg_dn7 + locals.var_cfg_dn7), (locals.var_ccg_dn8 + locals.var_cfg_dn8), (locals.var_ccg_dn9 + locals.var_cfg_dn9), (locals.var_ccg_dn10 + locals.var_cfg_dn10), (locals.var_ccg_dn11 + locals.var_cfg_dn11), (locals.var_ccg_dn13 + locals.var_cfg_dn13), (locals.var_ccg_dn14 + locals.var_cfg_dn14),)
    } else {
        (locals.var_cgg_top, locals.var_cgg_top_dn0, locals.var_cgg_top_dn2, locals.var_cgg_top_dn3, locals.var_cgg_top_dn4, locals.var_cgg_top_dn5, locals.var_cgg_top_dn6, locals.var_cgg_top_dn7, locals.var_cgg_top_dn8, locals.var_cgg_top_dn9, locals.var_cgg_top_dn10, locals.var_cgg_top_dn11, locals.var_cgg_top_dn13, locals.var_cgg_top_dn14,)
    }
};
        locals.var_cgg_top = assign10340_e13094;
        locals.var_cgg_top_dn0 = assign10340_e13094_d_n0;
        locals.var_cgg_top_dn2 = assign10340_e13094_d_n2;
        locals.var_cgg_top_dn3 = assign10340_e13094_d_n3;
        locals.var_cgg_top_dn4 = assign10340_e13094_d_n4;
        locals.var_cgg_top_dn5 = assign10340_e13094_d_n5;
        locals.var_cgg_top_dn6 = assign10340_e13094_d_n6;
        locals.var_cgg_top_dn7 = assign10340_e13094_d_n7;
        locals.var_cgg_top_dn8 = assign10340_e13094_d_n8;
        locals.var_cgg_top_dn9 = assign10340_e13094_d_n9;
        locals.var_cgg_top_dn10 = assign10340_e13094_d_n10;
        locals.var_cgg_top_dn11 = assign10340_e13094_d_n11;
        locals.var_cgg_top_dn13 = assign10340_e13094_d_n13;
        locals.var_cgg_top_dn14 = assign10340_e13094_d_n14;
        locals.var_cgg_top_rv = 0.0;

        let assign10350_e13097: f64 = if p.p1090 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard208 = assign10350_e13097;
        locals.var_guard208_rv = 0.0;

        let (assign10360_e13111,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10360_e13105: f64 = (locals.var_wg + p.p90);
        let assign10360_e13106: f64 = (0.2 * assign10360_e13105);
        let assign10360_e13108: f64 = (assign10360_e13106 / locals.var_trsd);
        let assign10360_e13109: f64 = (2.3 + assign10360_e13108);
        (assign10360_e13109,)
    } else {
        (locals.var_hr,)
    }
};
        locals.var_hr = assign10360_e13111;
        locals.var_hr_rv = 0.0;

        let (assign10370_e13117,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        (1.05,)
    } else {
        (locals.var_lr,)
    }
};
        locals.var_lr = assign10370_e13117;
        locals.var_lr_rv = 0.0;

        let (assign10380_e13128,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10380_e13123: f64 = (locals.var_wg + p.p90);
        let assign10380_e13125: f64 = (assign10380_e13123 - locals.var_trsd);
        let assign10380_e13126: f64 = (assign10380_e13125).abs();
        (assign10380_e13126,)
    } else {
        (locals.var_hgdelta,)
    }
};
        locals.var_hgdelta = assign10380_e13128;
        locals.var_hgdelta_rv = 0.0;

        let (assign10390_e13136,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10390_e13134: f64 = (p.p1087 * locals.var_lr);
        (assign10390_e13134,)
    } else {
        (locals.var_lmax,)
    }
};
        locals.var_lmax = assign10390_e13136;
        locals.var_lmax_rv = 0.0;

        let (assign10400_e13146,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10400_e13143: f64 = (locals.var_wg + p.p90);
        let assign10400_e13144: f64 = (locals.var_trsd).min(assign10400_e13143);
        (assign10400_e13144,)
    } else {
        (locals.var_y,)
    }
};
        locals.var_y = assign10400_e13146;
        locals.var_y_rv = 0.0;

        let (assign10410_e13156,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10410_e13153: f64 = (locals.var_hr + 1.0);
        let assign10410_e13154: f64 = (p.p1087 / assign10410_e13153);
        (assign10410_e13154,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign10410_e13156;
        locals.var_x_rv = 0.0;

        let (assign10420_e13162,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        (1700000000000.0,)
    } else {
        (locals.var_cnon,)
    }
};
        locals.var_cnon = assign10420_e13162;
        locals.var_cnon_rv = 0.0;

        let (assign10430_e13174,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10430_e13169: f64 = (locals.var_y - locals.var_x);
        let assign10430_e13170: f64 = (locals.var_epssp * assign10430_e13169);
        let assign10430_e13172: f64 = (assign10430_e13170 / p.p1087);
        (assign10430_e13172,)
    } else {
        (locals.var_ccgsat,)
    }
};
        locals.var_ccgsat = assign10430_e13174;
        locals.var_ccgsat_rv = 0.0;

        let (assign10440_e13182, assign10440_e13182_d_n0, assign10440_e13182_d_n2, assign10440_e13182_d_n3, assign10440_e13182_d_n4, assign10440_e13182_d_n5, assign10440_e13182_d_n6, assign10440_e13182_d_n7, assign10440_e13182_d_n8, assign10440_e13182_d_n9, assign10440_e13182_d_n10, assign10440_e13182_d_n11, assign10440_e13182_d_n13, assign10440_e13182_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10440_e13180: f64 = (locals.var_cnon * locals.var_ccgsat);
        (assign10440_e13180, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign10440_e13182;
        locals.var_tt1_dn0 = assign10440_e13182_d_n0;
        locals.var_tt1_dn2 = assign10440_e13182_d_n2;
        locals.var_tt1_dn3 = assign10440_e13182_d_n3;
        locals.var_tt1_dn4 = assign10440_e13182_d_n4;
        locals.var_tt1_dn5 = assign10440_e13182_d_n5;
        locals.var_tt1_dn6 = assign10440_e13182_d_n6;
        locals.var_tt1_dn7 = assign10440_e13182_d_n7;
        locals.var_tt1_dn8 = assign10440_e13182_d_n8;
        locals.var_tt1_dn9 = assign10440_e13182_d_n9;
        locals.var_tt1_dn10 = assign10440_e13182_d_n10;
        locals.var_tt1_dn11 = assign10440_e13182_d_n11;
        locals.var_tt1_dn13 = assign10440_e13182_d_n13;
        locals.var_tt1_dn14 = assign10440_e13182_d_n14;
        locals.var_tt1_rv = 0.0;

        let assign10450_e13185: f64 = if locals.var_tt1 > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard209 = assign10450_e13185;
        locals.var_guard209_rv = 0.0;

        let (assign10460_e13193, assign10460_e13193_d_n0, assign10460_e13193_d_n2, assign10460_e13193_d_n3, assign10460_e13193_d_n4, assign10460_e13193_d_n5, assign10460_e13193_d_n6, assign10460_e13193_d_n7, assign10460_e13193_d_n8, assign10460_e13193_d_n9, assign10460_e13193_d_n10, assign10460_e13193_d_n11, assign10460_e13193_d_n13, assign10460_e13193_d_n14,) = {
    if (((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) && (locals.var_guard209 != 0.0)) {
        (locals.var_ccgsat, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign10460_e13193;
        locals.var_ccg1_dn0 = assign10460_e13193_d_n0;
        locals.var_ccg1_dn2 = assign10460_e13193_d_n2;
        locals.var_ccg1_dn3 = assign10460_e13193_d_n3;
        locals.var_ccg1_dn4 = assign10460_e13193_d_n4;
        locals.var_ccg1_dn5 = assign10460_e13193_d_n5;
        locals.var_ccg1_dn6 = assign10460_e13193_d_n6;
        locals.var_ccg1_dn7 = assign10460_e13193_d_n7;
        locals.var_ccg1_dn8 = assign10460_e13193_d_n8;
        locals.var_ccg1_dn9 = assign10460_e13193_d_n9;
        locals.var_ccg1_dn10 = assign10460_e13193_d_n10;
        locals.var_ccg1_dn11 = assign10460_e13193_d_n11;
        locals.var_ccg1_dn13 = assign10460_e13193_d_n13;
        locals.var_ccg1_dn14 = assign10460_e13193_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign10470_e13239, assign10470_e13239_d_n0, assign10470_e13239_d_n2, assign10470_e13239_d_n3, assign10470_e13239_d_n4, assign10470_e13239_d_n5, assign10470_e13239_d_n6, assign10470_e13239_d_n7, assign10470_e13239_d_n8, assign10470_e13239_d_n9, assign10470_e13239_d_n10, assign10470_e13239_d_n11, assign10470_e13239_d_n13, assign10470_e13239_d_n14,) = {
    if (((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) && (locals.var_guard209 == 0.0)) {
        let assign10470_e13202: f64 = (1.0 / locals.var_cnon);
        let assign10470_e13209: f64 = (-37.0);
        let (assign10470_e13236, assign10470_e13236_d_n0, assign10470_e13236_d_n2, assign10470_e13236_d_n3, assign10470_e13236_d_n4, assign10470_e13236_d_n5, assign10470_e13236_d_n6, assign10470_e13236_d_n7, assign10470_e13236_d_n8, assign10470_e13236_d_n9, assign10470_e13236_d_n10, assign10470_e13236_d_n11, assign10470_e13236_d_n13, assign10470_e13236_d_n14,) = {
            if ((!(locals.var_tt1 > 37.0)) && (!(locals.var_tt1 < assign10470_e13209))) {
                let assign10470_e13215: f64 = (locals.var_tt1).exp();
                let assign10470_e13216: f64 = (1.0 + assign10470_e13215);
                let assign10470_e13217: f64 = (assign10470_e13216).ln();
                (assign10470_e13217, ((assign10470_e13215 * locals.var_tt1_dn0) / assign10470_e13216), ((assign10470_e13215 * locals.var_tt1_dn2) / assign10470_e13216), ((assign10470_e13215 * locals.var_tt1_dn3) / assign10470_e13216), ((assign10470_e13215 * locals.var_tt1_dn4) / assign10470_e13216), ((assign10470_e13215 * locals.var_tt1_dn5) / assign10470_e13216), ((assign10470_e13215 * locals.var_tt1_dn6) / assign10470_e13216), ((assign10470_e13215 * locals.var_tt1_dn7) / assign10470_e13216), ((assign10470_e13215 * locals.var_tt1_dn8) / assign10470_e13216), ((assign10470_e13215 * locals.var_tt1_dn9) / assign10470_e13216), ((assign10470_e13215 * locals.var_tt1_dn10) / assign10470_e13216), ((assign10470_e13215 * locals.var_tt1_dn11) / assign10470_e13216), ((assign10470_e13215 * locals.var_tt1_dn13) / assign10470_e13216), ((assign10470_e13215 * locals.var_tt1_dn14) / assign10470_e13216),)
            } else {
                let assign10470_e13224: f64 = (-37.0);
                let (assign10470_e13235, assign10470_e13235_d_n0, assign10470_e13235_d_n2, assign10470_e13235_d_n3, assign10470_e13235_d_n4, assign10470_e13235_d_n5, assign10470_e13235_d_n6, assign10470_e13235_d_n7, assign10470_e13235_d_n8, assign10470_e13235_d_n9, assign10470_e13235_d_n10, assign10470_e13235_d_n11, assign10470_e13235_d_n13, assign10470_e13235_d_n14,) = {
                    if ((!(locals.var_tt1 > 37.0)) && (locals.var_tt1 < assign10470_e13224)) {
                        let assign10470_e13228: f64 = (locals.var_tt1).exp();
                        (assign10470_e13228, (assign10470_e13228 * locals.var_tt1_dn0), (assign10470_e13228 * locals.var_tt1_dn2), (assign10470_e13228 * locals.var_tt1_dn3), (assign10470_e13228 * locals.var_tt1_dn4), (assign10470_e13228 * locals.var_tt1_dn5), (assign10470_e13228 * locals.var_tt1_dn6), (assign10470_e13228 * locals.var_tt1_dn7), (assign10470_e13228 * locals.var_tt1_dn8), (assign10470_e13228 * locals.var_tt1_dn9), (assign10470_e13228 * locals.var_tt1_dn10), (assign10470_e13228 * locals.var_tt1_dn11), (assign10470_e13228 * locals.var_tt1_dn13), (assign10470_e13228 * locals.var_tt1_dn14),)
                    } else {
                        let (assign10470_e13234, assign10470_e13234_d_n0, assign10470_e13234_d_n2, assign10470_e13234_d_n3, assign10470_e13234_d_n4, assign10470_e13234_d_n5, assign10470_e13234_d_n6, assign10470_e13234_d_n7, assign10470_e13234_d_n8, assign10470_e13234_d_n9, assign10470_e13234_d_n10, assign10470_e13234_d_n11, assign10470_e13234_d_n13, assign10470_e13234_d_n14,) = {
                            if (locals.var_tt1 > 37.0) {
                                (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign10470_e13234, assign10470_e13234_d_n0, assign10470_e13234_d_n2, assign10470_e13234_d_n3, assign10470_e13234_d_n4, assign10470_e13234_d_n5, assign10470_e13234_d_n6, assign10470_e13234_d_n7, assign10470_e13234_d_n8, assign10470_e13234_d_n9, assign10470_e13234_d_n10, assign10470_e13234_d_n11, assign10470_e13234_d_n13, assign10470_e13234_d_n14,)
                    }
                };
                (assign10470_e13235, assign10470_e13235_d_n0, assign10470_e13235_d_n2, assign10470_e13235_d_n3, assign10470_e13235_d_n4, assign10470_e13235_d_n5, assign10470_e13235_d_n6, assign10470_e13235_d_n7, assign10470_e13235_d_n8, assign10470_e13235_d_n9, assign10470_e13235_d_n10, assign10470_e13235_d_n11, assign10470_e13235_d_n13, assign10470_e13235_d_n14,)
            }
        };
        let assign10470_e13237: f64 = (assign10470_e13202 * assign10470_e13236);
        (assign10470_e13237, (assign10470_e13202 * assign10470_e13236_d_n0), (assign10470_e13202 * assign10470_e13236_d_n2), (assign10470_e13202 * assign10470_e13236_d_n3), (assign10470_e13202 * assign10470_e13236_d_n4), (assign10470_e13202 * assign10470_e13236_d_n5), (assign10470_e13202 * assign10470_e13236_d_n6), (assign10470_e13202 * assign10470_e13236_d_n7), (assign10470_e13202 * assign10470_e13236_d_n8), (assign10470_e13202 * assign10470_e13236_d_n9), (assign10470_e13202 * assign10470_e13236_d_n10), (assign10470_e13202 * assign10470_e13236_d_n11), (assign10470_e13202 * assign10470_e13236_d_n13), (assign10470_e13202 * assign10470_e13236_d_n14),)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign10470_e13239;
        locals.var_ccg1_dn0 = assign10470_e13239_d_n0;
        locals.var_ccg1_dn2 = assign10470_e13239_d_n2;
        locals.var_ccg1_dn3 = assign10470_e13239_d_n3;
        locals.var_ccg1_dn4 = assign10470_e13239_d_n4;
        locals.var_ccg1_dn5 = assign10470_e13239_d_n5;
        locals.var_ccg1_dn6 = assign10470_e13239_d_n6;
        locals.var_ccg1_dn7 = assign10470_e13239_d_n7;
        locals.var_ccg1_dn8 = assign10470_e13239_d_n8;
        locals.var_ccg1_dn9 = assign10470_e13239_d_n9;
        locals.var_ccg1_dn10 = assign10470_e13239_d_n10;
        locals.var_ccg1_dn11 = assign10470_e13239_d_n11;
        locals.var_ccg1_dn13 = assign10470_e13239_d_n13;
        locals.var_ccg1_dn14 = assign10470_e13239_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign10480_e13257,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10480_e13247: f64 = (locals.var_wg + p.p90);
        let assign10480_e13248: f64 = (locals.var_trsd / assign10480_e13247);
        let assign10480_e13251: f64 = (locals.var_wg + p.p90);
        let assign10480_e13253: f64 = (assign10480_e13251 / locals.var_trsd);
        let assign10480_e13254: f64 = (assign10480_e13248).min(assign10480_e13253);
        let assign10480_e13255: f64 = (0.5 * assign10480_e13254);
        (assign10480_e13255,)
    } else {
        (locals.var_r1cf,)
    }
};
        locals.var_r1cf = assign10480_e13257;
        locals.var_r1cf_rv = 0.0;

        let (assign10490_e13265,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10490_e13263: f64 = (locals.var_hgdelta * locals.var_r1cf);
        (assign10490_e13263,)
    } else {
        (locals.var_rcf,)
    }
};
        locals.var_rcf = assign10490_e13265;
        locals.var_rcf_rv = 0.0;

        let (assign10500_e13314,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10500_e13271: f64 = (locals.var_epssp * 2.0);
        let assign10500_e13273: f64 = (assign10500_e13271 / 3.141592653589793);
        let assign10500_e13277: f64 = (0.5 * 3.141592653589793);
        let assign10500_e13279: f64 = (assign10500_e13277 * locals.var_rcf);
        let assign10500_e13280: f64 = (p.p1087 + assign10500_e13279);
        let assign10500_e13282: f64 = (assign10500_e13280 / p.p1087);
        let (assign10500_e13311,) = {
            if (!(assign10500_e13282 > 1e-38)) {
                let assign10500_e13287: f64 = (-87.498233534);
                (assign10500_e13287,)
            } else {
                let assign10500_e13291: f64 = (0.5 * 3.141592653589793);
                let assign10500_e13293: f64 = (assign10500_e13291 * locals.var_rcf);
                let assign10500_e13294: f64 = (p.p1087 + assign10500_e13293);
                let assign10500_e13296: f64 = (assign10500_e13294 / p.p1087);
                let (assign10500_e13310,) = {
                    if (assign10500_e13296 > 1e-38) {
                        let assign10500_e13302: f64 = (0.5 * 3.141592653589793);
                        let assign10500_e13304: f64 = (assign10500_e13302 * locals.var_rcf);
                        let assign10500_e13305: f64 = (p.p1087 + assign10500_e13304);
                        let assign10500_e13307: f64 = (assign10500_e13305 / p.p1087);
                        let assign10500_e13308: f64 = (assign10500_e13307).ln();
                        (assign10500_e13308,)
                    } else {
                        (0.0,)
                    }
                };
                (assign10500_e13310,)
            }
        };
        let assign10500_e13312: f64 = (assign10500_e13273 * assign10500_e13311);
        (assign10500_e13312,)
    } else {
        (locals.var_ccg2,)
    }
};
        locals.var_ccg2 = assign10500_e13314;
        locals.var_ccg2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10510_e13324, assign10510_e13324_d_n0, assign10510_e13324_d_n2, assign10510_e13324_d_n3, assign10510_e13324_d_n4, assign10510_e13324_d_n5, assign10510_e13324_d_n6, assign10510_e13324_d_n7, assign10510_e13324_d_n8, assign10510_e13324_d_n9, assign10510_e13324_d_n10, assign10510_e13324_d_n11, assign10510_e13324_d_n13, assign10510_e13324_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10510_e13321: f64 = (locals.var_ccg1 + locals.var_ccg2);
        let assign10510_e13322: f64 = (p.p92 * assign10510_e13321);
        (assign10510_e13322, (p.p92 * locals.var_ccg1_dn0), (p.p92 * locals.var_ccg1_dn2), (p.p92 * locals.var_ccg1_dn3), (p.p92 * locals.var_ccg1_dn4), (p.p92 * locals.var_ccg1_dn5), (p.p92 * locals.var_ccg1_dn6), (p.p92 * locals.var_ccg1_dn7), (p.p92 * locals.var_ccg1_dn8), (p.p92 * locals.var_ccg1_dn9), (p.p92 * locals.var_ccg1_dn10), (p.p92 * locals.var_ccg1_dn11), (p.p92 * locals.var_ccg1_dn13), (p.p92 * locals.var_ccg1_dn14),)
    } else {
        (locals.var_ccg, locals.var_ccg_dn0, locals.var_ccg_dn2, locals.var_ccg_dn3, locals.var_ccg_dn4, locals.var_ccg_dn5, locals.var_ccg_dn6, locals.var_ccg_dn7, locals.var_ccg_dn8, locals.var_ccg_dn9, locals.var_ccg_dn10, locals.var_ccg_dn11, locals.var_ccg_dn13, locals.var_ccg_dn14,)
    }
};
        locals.var_ccg = assign10510_e13324;
        locals.var_ccg_dn0 = assign10510_e13324_d_n0;
        locals.var_ccg_dn2 = assign10510_e13324_d_n2;
        locals.var_ccg_dn3 = assign10510_e13324_d_n3;
        locals.var_ccg_dn4 = assign10510_e13324_d_n4;
        locals.var_ccg_dn5 = assign10510_e13324_d_n5;
        locals.var_ccg_dn6 = assign10510_e13324_d_n6;
        locals.var_ccg_dn7 = assign10510_e13324_d_n7;
        locals.var_ccg_dn8 = assign10510_e13324_d_n8;
        locals.var_ccg_dn9 = assign10510_e13324_d_n9;
        locals.var_ccg_dn10 = assign10510_e13324_d_n10;
        locals.var_ccg_dn11 = assign10510_e13324_d_n11;
        locals.var_ccg_dn13 = assign10510_e13324_d_n13;
        locals.var_ccg_dn14 = assign10510_e13324_d_n14;
        locals.var_ccg_rv = 0.0;

        let (assign10520_e13332,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10520_e13330: f64 = (locals.var_lmax / locals.var_wg);
        (assign10520_e13330,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign10520_e13332;
        locals.var_x_rv = 0.0;

        let (assign10530_e13347,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10530_e13340: f64 = (locals.var_x + 1.0);
        let assign10530_e13341: f64 = (2.0 * assign10530_e13340);
        let assign10530_e13342: f64 = (assign10530_e13341).sqrt();
        let assign10530_e13344: f64 = (assign10530_e13342 * 3.141592653589793);
        let assign10530_e13345: f64 = (4.0 / assign10530_e13344);
        (assign10530_e13345,)
    } else {
        (locals.var_c1,)
    }
};
        locals.var_c1 = assign10530_e13347;
        locals.var_c1_rv = 0.0;

        let (assign10540_e13383,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10540_e13353: f64 = (p.p90 * p.p90);
        let assign10540_e13356: f64 = (2.0 * locals.var_wg);
        let assign10540_e13358: f64 = (assign10540_e13356 * p.p90);
        let assign10540_e13359: f64 = (assign10540_e13353 + assign10540_e13358);
        let assign10540_e13362: f64 = (locals.var_wg * locals.var_wg);
        let assign10540_e13365: f64 = (locals.var_x + 1.0);
        let assign10540_e13366: f64 = (assign10540_e13362 * assign10540_e13365);
        let assign10540_e13367: f64 = (assign10540_e13359 + assign10540_e13366);
        let assign10540_e13368: f64 = (assign10540_e13367).sqrt();
        let assign10540_e13371: f64 = (locals.var_x + 1.0);
        let assign10540_e13372: f64 = (assign10540_e13371).sqrt();
        let assign10540_e13373: f64 = (assign10540_e13368 * assign10540_e13372);
        let assign10540_e13375: f64 = (assign10540_e13373 + p.p90);
        let assign10540_e13378: f64 = (locals.var_wg * locals.var_x);
        let assign10540_e13379: f64 = (assign10540_e13375 + assign10540_e13378);
        let assign10540_e13381: f64 = (assign10540_e13379 + locals.var_wg);
        (assign10540_e13381,)
    } else {
        (locals.var_c2,)
    }
};
        locals.var_c2 = assign10540_e13383;
        locals.var_c2_rv = 0.0;

        let (assign10550_e13404,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10550_e13390: f64 = (locals.var_x + 1.0);
        let assign10550_e13393: f64 = (locals.var_x + 4.0);
        let assign10550_e13394: f64 = (assign10550_e13390 * assign10550_e13393);
        let assign10550_e13395: f64 = (assign10550_e13394).sqrt();
        let assign10550_e13396: f64 = (p.p90 * assign10550_e13395);
        let assign10550_e13400: f64 = (locals.var_x + 2.0);
        let assign10550_e13401: f64 = (p.p90 * assign10550_e13400);
        let assign10550_e13402: f64 = (assign10550_e13396 + assign10550_e13401);
        (assign10550_e13402,)
    } else {
        (locals.var_c3,)
    }
};
        locals.var_c3 = assign10550_e13404;
        locals.var_c3_rv = 0.0;

        let (assign10560_e13435,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10560_e13412: f64 = (locals.var_c2 / locals.var_c3);
        let (assign10560_e13429,) = {
            if (!(assign10560_e13412 > 1e-38)) {
                let assign10560_e13417: f64 = (-87.498233534);
                (assign10560_e13417,)
            } else {
                let assign10560_e13420: f64 = (locals.var_c2 / locals.var_c3);
                let (assign10560_e13428,) = {
                    if (assign10560_e13420 > 1e-38) {
                        let assign10560_e13425: f64 = (locals.var_c2 / locals.var_c3);
                        let assign10560_e13426: f64 = (assign10560_e13425).ln();
                        (assign10560_e13426,)
                    } else {
                        (0.0,)
                    }
                };
                (assign10560_e13428,)
            }
        };
        let assign10560_e13430: f64 = (locals.var_c1 * assign10560_e13429);
        let assign10560_e13432: f64 = (assign10560_e13430 + 12.27);
        let assign10560_e13433: f64 = (locals.var_epssp * assign10560_e13432);
        (assign10560_e13433,)
    } else {
        (locals.var_cfglog,)
    }
};
        locals.var_cfglog = assign10560_e13435;
        locals.var_cfglog_rv = 0.0;

        let (assign10570_e13443,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10570_e13441: f64 = (locals.var_hr * locals.var_lr);
        (assign10570_e13441,)
    } else {
        (locals.var_dcf,)
    }
};
        locals.var_dcf = assign10570_e13443;
        locals.var_dcf_rv = 0.0;

        let (assign10580_e13454,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10580_e13449: f64 = (locals.var_dcf * locals.var_dcf);
        let assign10580_e13451: f64 = (assign10580_e13449 + 1.0);
        let assign10580_e13452: f64 = (assign10580_e13451).sqrt();
        (assign10580_e13452,)
    } else {
        (locals.var_tt0,)
    }
};
        locals.var_tt0 = assign10580_e13454;
        locals.var_tt0_rv = 0.0;

        let (assign10590_e13503, assign10590_e13503_d_n0, assign10590_e13503_d_n2, assign10590_e13503_d_n3, assign10590_e13503_d_n4, assign10590_e13503_d_n5, assign10590_e13503_d_n6, assign10590_e13503_d_n7, assign10590_e13503_d_n8, assign10590_e13503_d_n9, assign10590_e13503_d_n10, assign10590_e13503_d_n11, assign10590_e13503_d_n13, assign10590_e13503_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10590_e13460: f64 = (locals.var_dcf * locals.var_dcf);
        let assign10590_e13462: f64 = (assign10590_e13460 + 1.0);
        let assign10590_e13465: f64 = (locals.var_dcf * p.p90);
        let assign10590_e13468: f64 = (locals.var_dcf * p.p90);
        let assign10590_e13469: f64 = (assign10590_e13465 * assign10590_e13468);
        let assign10590_e13472: f64 = (2.0 * locals.var_dcf);
        let assign10590_e13474: f64 = (assign10590_e13472 * locals.var_lmax);
        let assign10590_e13476: f64 = (assign10590_e13474 * p.p90);
        let assign10590_e13477: f64 = (assign10590_e13469 + assign10590_e13476);
        let assign10590_e13480: f64 = (locals.var_dcf * locals.var_dcf);
        let assign10590_e13482: f64 = (assign10590_e13480 + 1.0);
        let assign10590_e13484: f64 = (assign10590_e13482 * locals.var_lmax);
        let assign10590_e13486: f64 = (assign10590_e13484 * locals.var_lmax);
        let assign10590_e13487: f64 = (assign10590_e13477 + assign10590_e13486);
        let assign10590_e13488: f64 = (assign10590_e13462 * assign10590_e13487);
        let assign10590_e13489: f64 = (assign10590_e13488).sqrt();
        let assign10590_e13492: f64 = (locals.var_dcf * p.p90);
        let assign10590_e13493: f64 = (assign10590_e13489 + assign10590_e13492);
        let assign10590_e13496: f64 = (locals.var_dcf * locals.var_dcf);
        let assign10590_e13498: f64 = (assign10590_e13496 * locals.var_lmax);
        let assign10590_e13499: f64 = (assign10590_e13493 + assign10590_e13498);
        let assign10590_e13501: f64 = (assign10590_e13499 + locals.var_lmax);
        (assign10590_e13501, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign10590_e13503;
        locals.var_tt1_dn0 = assign10590_e13503_d_n0;
        locals.var_tt1_dn2 = assign10590_e13503_d_n2;
        locals.var_tt1_dn3 = assign10590_e13503_d_n3;
        locals.var_tt1_dn4 = assign10590_e13503_d_n4;
        locals.var_tt1_dn5 = assign10590_e13503_d_n5;
        locals.var_tt1_dn6 = assign10590_e13503_d_n6;
        locals.var_tt1_dn7 = assign10590_e13503_d_n7;
        locals.var_tt1_dn8 = assign10590_e13503_d_n8;
        locals.var_tt1_dn9 = assign10590_e13503_d_n9;
        locals.var_tt1_dn10 = assign10590_e13503_d_n10;
        locals.var_tt1_dn11 = assign10590_e13503_d_n11;
        locals.var_tt1_dn13 = assign10590_e13503_d_n13;
        locals.var_tt1_dn14 = assign10590_e13503_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign10600_e13515,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10600_e13509: f64 = (locals.var_tt0 + 1.0);
        let assign10600_e13512: f64 = (locals.var_dcf * p.p90);
        let assign10600_e13513: f64 = (assign10600_e13509 * assign10600_e13512);
        (assign10600_e13513,)
    } else {
        (locals.var_tt2,)
    }
};
        locals.var_tt2 = assign10600_e13515;
        locals.var_tt2_rv = 0.0;

        let (assign10610_e13555, assign10610_e13555_d_n0, assign10610_e13555_d_n2, assign10610_e13555_d_n3, assign10610_e13555_d_n4, assign10610_e13555_d_n5, assign10610_e13555_d_n6, assign10610_e13555_d_n7, assign10610_e13555_d_n8, assign10610_e13555_d_n9, assign10610_e13555_d_n10, assign10610_e13555_d_n11, assign10610_e13555_d_n13, assign10610_e13555_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10610_e13521: f64 = (2.0 * locals.var_epssp);
        let assign10610_e13523: f64 = (2.0_f64).sqrt();
        let assign10610_e13524: f64 = (assign10610_e13521 * assign10610_e13523);
        let assign10610_e13526: f64 = (assign10610_e13524 / 3.141592653589793);
        let assign10610_e13528: f64 = (assign10610_e13526 * 0.7);
        let assign10610_e13530: f64 = (assign10610_e13528 * locals.var_dcf);
        let assign10610_e13532: f64 = (assign10610_e13530 / locals.var_tt0);
        let assign10610_e13535: f64 = (locals.var_tt1 / locals.var_tt2);
        let (assign10610_e13552, assign10610_e13552_d_n0, assign10610_e13552_d_n2, assign10610_e13552_d_n3, assign10610_e13552_d_n4, assign10610_e13552_d_n5, assign10610_e13552_d_n6, assign10610_e13552_d_n7, assign10610_e13552_d_n8, assign10610_e13552_d_n9, assign10610_e13552_d_n10, assign10610_e13552_d_n11, assign10610_e13552_d_n13, assign10610_e13552_d_n14,) = {
            if (!(assign10610_e13535 > 1e-38)) {
                let assign10610_e13540: f64 = (-87.498233534);
                (assign10610_e13540, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign10610_e13543: f64 = (locals.var_tt1 / locals.var_tt2);
                let (assign10610_e13551, assign10610_e13551_d_n0, assign10610_e13551_d_n2, assign10610_e13551_d_n3, assign10610_e13551_d_n4, assign10610_e13551_d_n5, assign10610_e13551_d_n6, assign10610_e13551_d_n7, assign10610_e13551_d_n8, assign10610_e13551_d_n9, assign10610_e13551_d_n10, assign10610_e13551_d_n11, assign10610_e13551_d_n13, assign10610_e13551_d_n14,) = {
                    if (assign10610_e13543 > 1e-38) {
                        let assign10610_e13548: f64 = (locals.var_tt1 / locals.var_tt2);
                        let assign10610_e13549: f64 = (assign10610_e13548).ln();
                        (assign10610_e13549, ((locals.var_tt1_dn0 / locals.var_tt2) / assign10610_e13548), ((locals.var_tt1_dn2 / locals.var_tt2) / assign10610_e13548), ((locals.var_tt1_dn3 / locals.var_tt2) / assign10610_e13548), ((locals.var_tt1_dn4 / locals.var_tt2) / assign10610_e13548), ((locals.var_tt1_dn5 / locals.var_tt2) / assign10610_e13548), ((locals.var_tt1_dn6 / locals.var_tt2) / assign10610_e13548), ((locals.var_tt1_dn7 / locals.var_tt2) / assign10610_e13548), ((locals.var_tt1_dn8 / locals.var_tt2) / assign10610_e13548), ((locals.var_tt1_dn9 / locals.var_tt2) / assign10610_e13548), ((locals.var_tt1_dn10 / locals.var_tt2) / assign10610_e13548), ((locals.var_tt1_dn11 / locals.var_tt2) / assign10610_e13548), ((locals.var_tt1_dn13 / locals.var_tt2) / assign10610_e13548), ((locals.var_tt1_dn14 / locals.var_tt2) / assign10610_e13548),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign10610_e13551, assign10610_e13551_d_n0, assign10610_e13551_d_n2, assign10610_e13551_d_n3, assign10610_e13551_d_n4, assign10610_e13551_d_n5, assign10610_e13551_d_n6, assign10610_e13551_d_n7, assign10610_e13551_d_n8, assign10610_e13551_d_n9, assign10610_e13551_d_n10, assign10610_e13551_d_n11, assign10610_e13551_d_n13, assign10610_e13551_d_n14,)
            }
        };
        let assign10610_e13553: f64 = (assign10610_e13532 * assign10610_e13552);
        (assign10610_e13553, (assign10610_e13532 * assign10610_e13552_d_n0), (assign10610_e13532 * assign10610_e13552_d_n2), (assign10610_e13532 * assign10610_e13552_d_n3), (assign10610_e13532 * assign10610_e13552_d_n4), (assign10610_e13532 * assign10610_e13552_d_n5), (assign10610_e13532 * assign10610_e13552_d_n6), (assign10610_e13532 * assign10610_e13552_d_n7), (assign10610_e13532 * assign10610_e13552_d_n8), (assign10610_e13532 * assign10610_e13552_d_n9), (assign10610_e13532 * assign10610_e13552_d_n10), (assign10610_e13532 * assign10610_e13552_d_n11), (assign10610_e13532 * assign10610_e13552_d_n13), (assign10610_e13532 * assign10610_e13552_d_n14),)
    } else {
        (locals.var_cfgsat, locals.var_cfgsat_dn0, locals.var_cfgsat_dn2, locals.var_cfgsat_dn3, locals.var_cfgsat_dn4, locals.var_cfgsat_dn5, locals.var_cfgsat_dn6, locals.var_cfgsat_dn7, locals.var_cfgsat_dn8, locals.var_cfgsat_dn9, locals.var_cfgsat_dn10, locals.var_cfgsat_dn11, locals.var_cfgsat_dn13, locals.var_cfgsat_dn14,)
    }
};
        locals.var_cfgsat = assign10610_e13555;
        locals.var_cfgsat_dn0 = assign10610_e13555_d_n0;
        locals.var_cfgsat_dn2 = assign10610_e13555_d_n2;
        locals.var_cfgsat_dn3 = assign10610_e13555_d_n3;
        locals.var_cfgsat_dn4 = assign10610_e13555_d_n4;
        locals.var_cfgsat_dn5 = assign10610_e13555_d_n5;
        locals.var_cfgsat_dn6 = assign10610_e13555_d_n6;
        locals.var_cfgsat_dn7 = assign10610_e13555_d_n7;
        locals.var_cfgsat_dn8 = assign10610_e13555_d_n8;
        locals.var_cfgsat_dn9 = assign10610_e13555_d_n9;
        locals.var_cfgsat_dn10 = assign10610_e13555_d_n10;
        locals.var_cfgsat_dn11 = assign10610_e13555_d_n11;
        locals.var_cfgsat_dn13 = assign10610_e13555_d_n13;
        locals.var_cfgsat_dn14 = assign10610_e13555_d_n14;
        locals.var_cfgsat_rv = 0.0;

        let (assign10620_e13561, assign10620_e13561_d_n0, assign10620_e13561_d_n2, assign10620_e13561_d_n3, assign10620_e13561_d_n4, assign10620_e13561_d_n5, assign10620_e13561_d_n6, assign10620_e13561_d_n7, assign10620_e13561_d_n8, assign10620_e13561_d_n9, assign10620_e13561_d_n10, assign10620_e13561_d_n11, assign10620_e13561_d_n13, assign10620_e13561_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        (1.2e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign10620_e13561;
        locals.var_delta_dn0 = assign10620_e13561_d_n0;
        locals.var_delta_dn2 = assign10620_e13561_d_n2;
        locals.var_delta_dn3 = assign10620_e13561_d_n3;
        locals.var_delta_dn4 = assign10620_e13561_d_n4;
        locals.var_delta_dn5 = assign10620_e13561_d_n5;
        locals.var_delta_dn6 = assign10620_e13561_d_n6;
        locals.var_delta_dn7 = assign10620_e13561_d_n7;
        locals.var_delta_dn8 = assign10620_e13561_d_n8;
        locals.var_delta_dn9 = assign10620_e13561_d_n9;
        locals.var_delta_dn10 = assign10620_e13561_d_n10;
        locals.var_delta_dn11 = assign10620_e13561_d_n11;
        locals.var_delta_dn13 = assign10620_e13561_d_n13;
        locals.var_delta_dn14 = assign10620_e13561_d_n14;
        locals.var_delta_rv = 0.0;

        let (assign10630_e13571, assign10630_e13571_d_n0, assign10630_e13571_d_n2, assign10630_e13571_d_n3, assign10630_e13571_d_n4, assign10630_e13571_d_n5, assign10630_e13571_d_n6, assign10630_e13571_d_n7, assign10630_e13571_d_n8, assign10630_e13571_d_n9, assign10630_e13571_d_n10, assign10630_e13571_d_n11, assign10630_e13571_d_n13, assign10630_e13571_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10630_e13567: f64 = (locals.var_cfgsat - locals.var_cfglog);
        let assign10630_e13569: f64 = (assign10630_e13567 - locals.var_delta);
        (assign10630_e13569, (locals.var_cfgsat_dn0 - locals.var_delta_dn0), (locals.var_cfgsat_dn2 - locals.var_delta_dn2), (locals.var_cfgsat_dn3 - locals.var_delta_dn3), (locals.var_cfgsat_dn4 - locals.var_delta_dn4), (locals.var_cfgsat_dn5 - locals.var_delta_dn5), (locals.var_cfgsat_dn6 - locals.var_delta_dn6), (locals.var_cfgsat_dn7 - locals.var_delta_dn7), (locals.var_cfgsat_dn8 - locals.var_delta_dn8), (locals.var_cfgsat_dn9 - locals.var_delta_dn9), (locals.var_cfgsat_dn10 - locals.var_delta_dn10), (locals.var_cfgsat_dn11 - locals.var_delta_dn11), (locals.var_cfgsat_dn13 - locals.var_delta_dn13), (locals.var_cfgsat_dn14 - locals.var_delta_dn14),)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign10630_e13571;
        locals.var_tt1_dn0 = assign10630_e13571_d_n0;
        locals.var_tt1_dn2 = assign10630_e13571_d_n2;
        locals.var_tt1_dn3 = assign10630_e13571_d_n3;
        locals.var_tt1_dn4 = assign10630_e13571_d_n4;
        locals.var_tt1_dn5 = assign10630_e13571_d_n5;
        locals.var_tt1_dn6 = assign10630_e13571_d_n6;
        locals.var_tt1_dn7 = assign10630_e13571_d_n7;
        locals.var_tt1_dn8 = assign10630_e13571_d_n8;
        locals.var_tt1_dn9 = assign10630_e13571_d_n9;
        locals.var_tt1_dn10 = assign10630_e13571_d_n10;
        locals.var_tt1_dn11 = assign10630_e13571_d_n11;
        locals.var_tt1_dn13 = assign10630_e13571_d_n13;
        locals.var_tt1_dn14 = assign10630_e13571_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign10640_e13594, assign10640_e13594_d_n0, assign10640_e13594_d_n2, assign10640_e13594_d_n3, assign10640_e13594_d_n4, assign10640_e13594_d_n5, assign10640_e13594_d_n6, assign10640_e13594_d_n7, assign10640_e13594_d_n8, assign10640_e13594_d_n9, assign10640_e13594_d_n10, assign10640_e13594_d_n11, assign10640_e13594_d_n13, assign10640_e13594_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10640_e13581: f64 = (locals.var_tt1 * locals.var_tt1);
        let assign10640_e13584: f64 = (4.0 * locals.var_delta);
        let assign10640_e13586: f64 = (assign10640_e13584 * locals.var_cfgsat);
        let assign10640_e13587: f64 = (assign10640_e13581 + assign10640_e13586);
        let assign10640_e13588: f64 = (assign10640_e13587).sqrt();
        let assign10640_e13589: f64 = (locals.var_tt1 + assign10640_e13588);
        let assign10640_e13590: f64 = (0.5 * assign10640_e13589);
        let assign10640_e13591: f64 = (locals.var_cfgsat - assign10640_e13590);
        let assign10640_e13592: f64 = (p.p92 * assign10640_e13591);
        (assign10640_e13592, (p.p92 * (locals.var_cfgsat_dn0 - (0.5 * (locals.var_tt1_dn0 + ((((locals.var_tt1_dn0 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn0))) / (2.0 * assign10640_e13588)))))), (p.p92 * (locals.var_cfgsat_dn2 - (0.5 * (locals.var_tt1_dn2 + ((((locals.var_tt1_dn2 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn2))) / (2.0 * assign10640_e13588)))))), (p.p92 * (locals.var_cfgsat_dn3 - (0.5 * (locals.var_tt1_dn3 + ((((locals.var_tt1_dn3 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn3)) + (((4.0 * locals.var_delta_dn3) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn3))) / (2.0 * assign10640_e13588)))))), (p.p92 * (locals.var_cfgsat_dn4 - (0.5 * (locals.var_tt1_dn4 + ((((locals.var_tt1_dn4 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn4)) + (((4.0 * locals.var_delta_dn4) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn4))) / (2.0 * assign10640_e13588)))))), (p.p92 * (locals.var_cfgsat_dn5 - (0.5 * (locals.var_tt1_dn5 + ((((locals.var_tt1_dn5 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn5)) + (((4.0 * locals.var_delta_dn5) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn5))) / (2.0 * assign10640_e13588)))))), (p.p92 * (locals.var_cfgsat_dn6 - (0.5 * (locals.var_tt1_dn6 + ((((locals.var_tt1_dn6 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn6))) / (2.0 * assign10640_e13588)))))), (p.p92 * (locals.var_cfgsat_dn7 - (0.5 * (locals.var_tt1_dn7 + ((((locals.var_tt1_dn7 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn7))) / (2.0 * assign10640_e13588)))))), (p.p92 * (locals.var_cfgsat_dn8 - (0.5 * (locals.var_tt1_dn8 + ((((locals.var_tt1_dn8 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn8)) + (((4.0 * locals.var_delta_dn8) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn8))) / (2.0 * assign10640_e13588)))))), (p.p92 * (locals.var_cfgsat_dn9 - (0.5 * (locals.var_tt1_dn9 + ((((locals.var_tt1_dn9 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn9)) + (((4.0 * locals.var_delta_dn9) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn9))) / (2.0 * assign10640_e13588)))))), (p.p92 * (locals.var_cfgsat_dn10 - (0.5 * (locals.var_tt1_dn10 + ((((locals.var_tt1_dn10 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn10))) / (2.0 * assign10640_e13588)))))), (p.p92 * (locals.var_cfgsat_dn11 - (0.5 * (locals.var_tt1_dn11 + ((((locals.var_tt1_dn11 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn11))) / (2.0 * assign10640_e13588)))))), (p.p92 * (locals.var_cfgsat_dn13 - (0.5 * (locals.var_tt1_dn13 + ((((locals.var_tt1_dn13 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn13)) + (((4.0 * locals.var_delta_dn13) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn13))) / (2.0 * assign10640_e13588)))))), (p.p92 * (locals.var_cfgsat_dn14 - (0.5 * (locals.var_tt1_dn14 + ((((locals.var_tt1_dn14 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn14)) + (((4.0 * locals.var_delta_dn14) * locals.var_cfgsat) + (assign10640_e13584 * locals.var_cfgsat_dn14))) / (2.0 * assign10640_e13588)))))),)
    } else {
        (locals.var_cfg, locals.var_cfg_dn0, locals.var_cfg_dn2, locals.var_cfg_dn3, locals.var_cfg_dn4, locals.var_cfg_dn5, locals.var_cfg_dn6, locals.var_cfg_dn7, locals.var_cfg_dn8, locals.var_cfg_dn9, locals.var_cfg_dn10, locals.var_cfg_dn11, locals.var_cfg_dn13, locals.var_cfg_dn14,)
    }
};
        locals.var_cfg = assign10640_e13594;
        locals.var_cfg_dn0 = assign10640_e13594_d_n0;
        locals.var_cfg_dn2 = assign10640_e13594_d_n2;
        locals.var_cfg_dn3 = assign10640_e13594_d_n3;
        locals.var_cfg_dn4 = assign10640_e13594_d_n4;
        locals.var_cfg_dn5 = assign10640_e13594_d_n5;
        locals.var_cfg_dn6 = assign10640_e13594_d_n6;
        locals.var_cfg_dn7 = assign10640_e13594_d_n7;
        locals.var_cfg_dn8 = assign10640_e13594_d_n8;
        locals.var_cfg_dn9 = assign10640_e13594_d_n9;
        locals.var_cfg_dn10 = assign10640_e13594_d_n10;
        locals.var_cfg_dn11 = assign10640_e13594_d_n11;
        locals.var_cfg_dn13 = assign10640_e13594_d_n13;
        locals.var_cfg_dn14 = assign10640_e13594_d_n14;
        locals.var_cfg_rv = 0.0;

        let (assign10650_e13602, assign10650_e13602_d_n0, assign10650_e13602_d_n2, assign10650_e13602_d_n3, assign10650_e13602_d_n4, assign10650_e13602_d_n5, assign10650_e13602_d_n6, assign10650_e13602_d_n7, assign10650_e13602_d_n8, assign10650_e13602_d_n9, assign10650_e13602_d_n10, assign10650_e13602_d_n11, assign10650_e13602_d_n13, assign10650_e13602_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 != 0.0)) {
        let assign10650_e13600: f64 = (locals.var_ccg + locals.var_cfg);
        (assign10650_e13600, (locals.var_ccg_dn0 + locals.var_cfg_dn0), (locals.var_ccg_dn2 + locals.var_cfg_dn2), (locals.var_ccg_dn3 + locals.var_cfg_dn3), (locals.var_ccg_dn4 + locals.var_cfg_dn4), (locals.var_ccg_dn5 + locals.var_cfg_dn5), (locals.var_ccg_dn6 + locals.var_cfg_dn6), (locals.var_ccg_dn7 + locals.var_cfg_dn7), (locals.var_ccg_dn8 + locals.var_cfg_dn8), (locals.var_ccg_dn9 + locals.var_cfg_dn9), (locals.var_ccg_dn10 + locals.var_cfg_dn10), (locals.var_ccg_dn11 + locals.var_cfg_dn11), (locals.var_ccg_dn13 + locals.var_cfg_dn13), (locals.var_ccg_dn14 + locals.var_cfg_dn14),)
    } else {
        (locals.var_cgg_side, locals.var_cgg_side_dn0, locals.var_cgg_side_dn2, locals.var_cgg_side_dn3, locals.var_cgg_side_dn4, locals.var_cgg_side_dn5, locals.var_cgg_side_dn6, locals.var_cgg_side_dn7, locals.var_cgg_side_dn8, locals.var_cgg_side_dn9, locals.var_cgg_side_dn10, locals.var_cgg_side_dn11, locals.var_cgg_side_dn13, locals.var_cgg_side_dn14,)
    }
};
        locals.var_cgg_side = assign10650_e13602;
        locals.var_cgg_side_dn0 = assign10650_e13602_d_n0;
        locals.var_cgg_side_dn2 = assign10650_e13602_d_n2;
        locals.var_cgg_side_dn3 = assign10650_e13602_d_n3;
        locals.var_cgg_side_dn4 = assign10650_e13602_d_n4;
        locals.var_cgg_side_dn5 = assign10650_e13602_d_n5;
        locals.var_cgg_side_dn6 = assign10650_e13602_d_n6;
        locals.var_cgg_side_dn7 = assign10650_e13602_d_n7;
        locals.var_cgg_side_dn8 = assign10650_e13602_d_n8;
        locals.var_cgg_side_dn9 = assign10650_e13602_d_n9;
        locals.var_cgg_side_dn10 = assign10650_e13602_d_n10;
        locals.var_cgg_side_dn11 = assign10650_e13602_d_n11;
        locals.var_cgg_side_dn13 = assign10650_e13602_d_n13;
        locals.var_cgg_side_dn14 = assign10650_e13602_d_n14;
        locals.var_cgg_side_rv = 0.0;

        let (assign10660_e13617,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10660_e13611: f64 = (locals.var_wg + p.p90);
        let assign10660_e13612: f64 = (0.2 * assign10660_e13611);
        let assign10660_e13614: f64 = (assign10660_e13612 / locals.var_trsd);
        let assign10660_e13615: f64 = (2.3 + assign10660_e13614);
        (assign10660_e13615,)
    } else {
        (locals.var_hr,)
    }
};
        locals.var_hr = assign10660_e13617;
        locals.var_hr_rv = 0.0;

        let (assign10670_e13624,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        (1.05,)
    } else {
        (locals.var_lr,)
    }
};
        locals.var_lr = assign10670_e13624;
        locals.var_lr_rv = 0.0;

        let (assign10680_e13636,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10680_e13631: f64 = (locals.var_wg + p.p90);
        let assign10680_e13633: f64 = (assign10680_e13631 - locals.var_trsd);
        let assign10680_e13634: f64 = (assign10680_e13633).abs();
        (assign10680_e13634,)
    } else {
        (locals.var_hgdelta,)
    }
};
        locals.var_hgdelta = assign10680_e13636;
        locals.var_hgdelta_rv = 0.0;

        let (assign10690_e13645,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10690_e13643: f64 = (p.p1087 * locals.var_lr);
        (assign10690_e13643,)
    } else {
        (locals.var_lmax,)
    }
};
        locals.var_lmax = assign10690_e13645;
        locals.var_lmax_rv = 0.0;

        let (assign10700_e13656,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10700_e13653: f64 = (locals.var_wg + p.p90);
        let assign10700_e13654: f64 = (locals.var_trsd).min(assign10700_e13653);
        (assign10700_e13654,)
    } else {
        (locals.var_y,)
    }
};
        locals.var_y = assign10700_e13656;
        locals.var_y_rv = 0.0;

        let (assign10710_e13667,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10710_e13664: f64 = (locals.var_hr + 1.0);
        let assign10710_e13665: f64 = (p.p1087 / assign10710_e13664);
        (assign10710_e13665,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign10710_e13667;
        locals.var_x_rv = 0.0;

        let (assign10720_e13674,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        (1700000000000.0,)
    } else {
        (locals.var_cnon,)
    }
};
        locals.var_cnon = assign10720_e13674;
        locals.var_cnon_rv = 0.0;

        let (assign10730_e13687,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10730_e13682: f64 = (locals.var_y - locals.var_x);
        let assign10730_e13683: f64 = (locals.var_epssp * assign10730_e13682);
        let assign10730_e13685: f64 = (assign10730_e13683 / p.p1087);
        (assign10730_e13685,)
    } else {
        (locals.var_ccgsat,)
    }
};
        locals.var_ccgsat = assign10730_e13687;
        locals.var_ccgsat_rv = 0.0;

        let (assign10740_e13696, assign10740_e13696_d_n0, assign10740_e13696_d_n2, assign10740_e13696_d_n3, assign10740_e13696_d_n4, assign10740_e13696_d_n5, assign10740_e13696_d_n6, assign10740_e13696_d_n7, assign10740_e13696_d_n8, assign10740_e13696_d_n9, assign10740_e13696_d_n10, assign10740_e13696_d_n11, assign10740_e13696_d_n13, assign10740_e13696_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10740_e13694: f64 = (locals.var_cnon * locals.var_ccgsat);
        (assign10740_e13694, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign10740_e13696;
        locals.var_tt1_dn0 = assign10740_e13696_d_n0;
        locals.var_tt1_dn2 = assign10740_e13696_d_n2;
        locals.var_tt1_dn3 = assign10740_e13696_d_n3;
        locals.var_tt1_dn4 = assign10740_e13696_d_n4;
        locals.var_tt1_dn5 = assign10740_e13696_d_n5;
        locals.var_tt1_dn6 = assign10740_e13696_d_n6;
        locals.var_tt1_dn7 = assign10740_e13696_d_n7;
        locals.var_tt1_dn8 = assign10740_e13696_d_n8;
        locals.var_tt1_dn9 = assign10740_e13696_d_n9;
        locals.var_tt1_dn10 = assign10740_e13696_d_n10;
        locals.var_tt1_dn11 = assign10740_e13696_d_n11;
        locals.var_tt1_dn13 = assign10740_e13696_d_n13;
        locals.var_tt1_dn14 = assign10740_e13696_d_n14;
        locals.var_tt1_rv = 0.0;

        let assign10750_e13699: f64 = if locals.var_tt1 > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard210 = assign10750_e13699;
        locals.var_guard210_rv = 0.0;

        let (assign10760_e13708, assign10760_e13708_d_n0, assign10760_e13708_d_n2, assign10760_e13708_d_n3, assign10760_e13708_d_n4, assign10760_e13708_d_n5, assign10760_e13708_d_n6, assign10760_e13708_d_n7, assign10760_e13708_d_n8, assign10760_e13708_d_n9, assign10760_e13708_d_n10, assign10760_e13708_d_n11, assign10760_e13708_d_n13, assign10760_e13708_d_n14,) = {
    if (((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) && (locals.var_guard210 != 0.0)) {
        (locals.var_ccgsat, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign10760_e13708;
        locals.var_ccg1_dn0 = assign10760_e13708_d_n0;
        locals.var_ccg1_dn2 = assign10760_e13708_d_n2;
        locals.var_ccg1_dn3 = assign10760_e13708_d_n3;
        locals.var_ccg1_dn4 = assign10760_e13708_d_n4;
        locals.var_ccg1_dn5 = assign10760_e13708_d_n5;
        locals.var_ccg1_dn6 = assign10760_e13708_d_n6;
        locals.var_ccg1_dn7 = assign10760_e13708_d_n7;
        locals.var_ccg1_dn8 = assign10760_e13708_d_n8;
        locals.var_ccg1_dn9 = assign10760_e13708_d_n9;
        locals.var_ccg1_dn10 = assign10760_e13708_d_n10;
        locals.var_ccg1_dn11 = assign10760_e13708_d_n11;
        locals.var_ccg1_dn13 = assign10760_e13708_d_n13;
        locals.var_ccg1_dn14 = assign10760_e13708_d_n14;
        locals.var_ccg1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10770_e13755, assign10770_e13755_d_n0, assign10770_e13755_d_n2, assign10770_e13755_d_n3, assign10770_e13755_d_n4, assign10770_e13755_d_n5, assign10770_e13755_d_n6, assign10770_e13755_d_n7, assign10770_e13755_d_n8, assign10770_e13755_d_n9, assign10770_e13755_d_n10, assign10770_e13755_d_n11, assign10770_e13755_d_n13, assign10770_e13755_d_n14,) = {
    if (((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) && (locals.var_guard210 == 0.0)) {
        let assign10770_e13718: f64 = (1.0 / locals.var_cnon);
        let assign10770_e13725: f64 = (-37.0);
        let (assign10770_e13752, assign10770_e13752_d_n0, assign10770_e13752_d_n2, assign10770_e13752_d_n3, assign10770_e13752_d_n4, assign10770_e13752_d_n5, assign10770_e13752_d_n6, assign10770_e13752_d_n7, assign10770_e13752_d_n8, assign10770_e13752_d_n9, assign10770_e13752_d_n10, assign10770_e13752_d_n11, assign10770_e13752_d_n13, assign10770_e13752_d_n14,) = {
            if ((!(locals.var_tt1 > 37.0)) && (!(locals.var_tt1 < assign10770_e13725))) {
                let assign10770_e13731: f64 = (locals.var_tt1).exp();
                let assign10770_e13732: f64 = (1.0 + assign10770_e13731);
                let assign10770_e13733: f64 = (assign10770_e13732).ln();
                (assign10770_e13733, ((assign10770_e13731 * locals.var_tt1_dn0) / assign10770_e13732), ((assign10770_e13731 * locals.var_tt1_dn2) / assign10770_e13732), ((assign10770_e13731 * locals.var_tt1_dn3) / assign10770_e13732), ((assign10770_e13731 * locals.var_tt1_dn4) / assign10770_e13732), ((assign10770_e13731 * locals.var_tt1_dn5) / assign10770_e13732), ((assign10770_e13731 * locals.var_tt1_dn6) / assign10770_e13732), ((assign10770_e13731 * locals.var_tt1_dn7) / assign10770_e13732), ((assign10770_e13731 * locals.var_tt1_dn8) / assign10770_e13732), ((assign10770_e13731 * locals.var_tt1_dn9) / assign10770_e13732), ((assign10770_e13731 * locals.var_tt1_dn10) / assign10770_e13732), ((assign10770_e13731 * locals.var_tt1_dn11) / assign10770_e13732), ((assign10770_e13731 * locals.var_tt1_dn13) / assign10770_e13732), ((assign10770_e13731 * locals.var_tt1_dn14) / assign10770_e13732),)
            } else {
                let assign10770_e13740: f64 = (-37.0);
                let (assign10770_e13751, assign10770_e13751_d_n0, assign10770_e13751_d_n2, assign10770_e13751_d_n3, assign10770_e13751_d_n4, assign10770_e13751_d_n5, assign10770_e13751_d_n6, assign10770_e13751_d_n7, assign10770_e13751_d_n8, assign10770_e13751_d_n9, assign10770_e13751_d_n10, assign10770_e13751_d_n11, assign10770_e13751_d_n13, assign10770_e13751_d_n14,) = {
                    if ((!(locals.var_tt1 > 37.0)) && (locals.var_tt1 < assign10770_e13740)) {
                        let assign10770_e13744: f64 = (locals.var_tt1).exp();
                        (assign10770_e13744, (assign10770_e13744 * locals.var_tt1_dn0), (assign10770_e13744 * locals.var_tt1_dn2), (assign10770_e13744 * locals.var_tt1_dn3), (assign10770_e13744 * locals.var_tt1_dn4), (assign10770_e13744 * locals.var_tt1_dn5), (assign10770_e13744 * locals.var_tt1_dn6), (assign10770_e13744 * locals.var_tt1_dn7), (assign10770_e13744 * locals.var_tt1_dn8), (assign10770_e13744 * locals.var_tt1_dn9), (assign10770_e13744 * locals.var_tt1_dn10), (assign10770_e13744 * locals.var_tt1_dn11), (assign10770_e13744 * locals.var_tt1_dn13), (assign10770_e13744 * locals.var_tt1_dn14),)
                    } else {
                        let (assign10770_e13750, assign10770_e13750_d_n0, assign10770_e13750_d_n2, assign10770_e13750_d_n3, assign10770_e13750_d_n4, assign10770_e13750_d_n5, assign10770_e13750_d_n6, assign10770_e13750_d_n7, assign10770_e13750_d_n8, assign10770_e13750_d_n9, assign10770_e13750_d_n10, assign10770_e13750_d_n11, assign10770_e13750_d_n13, assign10770_e13750_d_n14,) = {
                            if (locals.var_tt1 > 37.0) {
                                (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign10770_e13750, assign10770_e13750_d_n0, assign10770_e13750_d_n2, assign10770_e13750_d_n3, assign10770_e13750_d_n4, assign10770_e13750_d_n5, assign10770_e13750_d_n6, assign10770_e13750_d_n7, assign10770_e13750_d_n8, assign10770_e13750_d_n9, assign10770_e13750_d_n10, assign10770_e13750_d_n11, assign10770_e13750_d_n13, assign10770_e13750_d_n14,)
                    }
                };
                (assign10770_e13751, assign10770_e13751_d_n0, assign10770_e13751_d_n2, assign10770_e13751_d_n3, assign10770_e13751_d_n4, assign10770_e13751_d_n5, assign10770_e13751_d_n6, assign10770_e13751_d_n7, assign10770_e13751_d_n8, assign10770_e13751_d_n9, assign10770_e13751_d_n10, assign10770_e13751_d_n11, assign10770_e13751_d_n13, assign10770_e13751_d_n14,)
            }
        };
        let assign10770_e13753: f64 = (assign10770_e13718 * assign10770_e13752);
        (assign10770_e13753, (assign10770_e13718 * assign10770_e13752_d_n0), (assign10770_e13718 * assign10770_e13752_d_n2), (assign10770_e13718 * assign10770_e13752_d_n3), (assign10770_e13718 * assign10770_e13752_d_n4), (assign10770_e13718 * assign10770_e13752_d_n5), (assign10770_e13718 * assign10770_e13752_d_n6), (assign10770_e13718 * assign10770_e13752_d_n7), (assign10770_e13718 * assign10770_e13752_d_n8), (assign10770_e13718 * assign10770_e13752_d_n9), (assign10770_e13718 * assign10770_e13752_d_n10), (assign10770_e13718 * assign10770_e13752_d_n11), (assign10770_e13718 * assign10770_e13752_d_n13), (assign10770_e13718 * assign10770_e13752_d_n14),)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign10770_e13755;
        locals.var_ccg1_dn0 = assign10770_e13755_d_n0;
        locals.var_ccg1_dn2 = assign10770_e13755_d_n2;
        locals.var_ccg1_dn3 = assign10770_e13755_d_n3;
        locals.var_ccg1_dn4 = assign10770_e13755_d_n4;
        locals.var_ccg1_dn5 = assign10770_e13755_d_n5;
        locals.var_ccg1_dn6 = assign10770_e13755_d_n6;
        locals.var_ccg1_dn7 = assign10770_e13755_d_n7;
        locals.var_ccg1_dn8 = assign10770_e13755_d_n8;
        locals.var_ccg1_dn9 = assign10770_e13755_d_n9;
        locals.var_ccg1_dn10 = assign10770_e13755_d_n10;
        locals.var_ccg1_dn11 = assign10770_e13755_d_n11;
        locals.var_ccg1_dn13 = assign10770_e13755_d_n13;
        locals.var_ccg1_dn14 = assign10770_e13755_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign10780_e13774,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10780_e13764: f64 = (locals.var_wg + p.p90);
        let assign10780_e13765: f64 = (locals.var_trsd / assign10780_e13764);
        let assign10780_e13768: f64 = (locals.var_wg + p.p90);
        let assign10780_e13770: f64 = (assign10780_e13768 / locals.var_trsd);
        let assign10780_e13771: f64 = (assign10780_e13765).min(assign10780_e13770);
        let assign10780_e13772: f64 = (0.5 * assign10780_e13771);
        (assign10780_e13772,)
    } else {
        (locals.var_r1cf,)
    }
};
        locals.var_r1cf = assign10780_e13774;
        locals.var_r1cf_rv = 0.0;

        let (assign10790_e13783,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10790_e13781: f64 = (locals.var_hgdelta * locals.var_r1cf);
        (assign10790_e13781,)
    } else {
        (locals.var_rcf,)
    }
};
        locals.var_rcf = assign10790_e13783;
        locals.var_rcf_rv = 0.0;

        let (assign10800_e13833,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10800_e13790: f64 = (locals.var_epssp * 2.0);
        let assign10800_e13792: f64 = (assign10800_e13790 / 3.141592653589793);
        let assign10800_e13796: f64 = (0.5 * 3.141592653589793);
        let assign10800_e13798: f64 = (assign10800_e13796 * locals.var_rcf);
        let assign10800_e13799: f64 = (p.p1087 + assign10800_e13798);
        let assign10800_e13801: f64 = (assign10800_e13799 / p.p1087);
        let (assign10800_e13830,) = {
            if (!(assign10800_e13801 > 1e-38)) {
                let assign10800_e13806: f64 = (-87.498233534);
                (assign10800_e13806,)
            } else {
                let assign10800_e13810: f64 = (0.5 * 3.141592653589793);
                let assign10800_e13812: f64 = (assign10800_e13810 * locals.var_rcf);
                let assign10800_e13813: f64 = (p.p1087 + assign10800_e13812);
                let assign10800_e13815: f64 = (assign10800_e13813 / p.p1087);
                let (assign10800_e13829,) = {
                    if (assign10800_e13815 > 1e-38) {
                        let assign10800_e13821: f64 = (0.5 * 3.141592653589793);
                        let assign10800_e13823: f64 = (assign10800_e13821 * locals.var_rcf);
                        let assign10800_e13824: f64 = (p.p1087 + assign10800_e13823);
                        let assign10800_e13826: f64 = (assign10800_e13824 / p.p1087);
                        let assign10800_e13827: f64 = (assign10800_e13826).ln();
                        (assign10800_e13827,)
                    } else {
                        (0.0,)
                    }
                };
                (assign10800_e13829,)
            }
        };
        let assign10800_e13831: f64 = (assign10800_e13792 * assign10800_e13830);
        (assign10800_e13831,)
    } else {
        (locals.var_ccg2,)
    }
};
        locals.var_ccg2 = assign10800_e13833;
        locals.var_ccg2_rv = 0.0;

        let (assign10810_e13844, assign10810_e13844_d_n0, assign10810_e13844_d_n2, assign10810_e13844_d_n3, assign10810_e13844_d_n4, assign10810_e13844_d_n5, assign10810_e13844_d_n6, assign10810_e13844_d_n7, assign10810_e13844_d_n8, assign10810_e13844_d_n9, assign10810_e13844_d_n10, assign10810_e13844_d_n11, assign10810_e13844_d_n13, assign10810_e13844_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10810_e13841: f64 = (locals.var_ccg1 + locals.var_ccg2);
        let assign10810_e13842: f64 = (p.p92 * assign10810_e13841);
        (assign10810_e13842, (p.p92 * locals.var_ccg1_dn0), (p.p92 * locals.var_ccg1_dn2), (p.p92 * locals.var_ccg1_dn3), (p.p92 * locals.var_ccg1_dn4), (p.p92 * locals.var_ccg1_dn5), (p.p92 * locals.var_ccg1_dn6), (p.p92 * locals.var_ccg1_dn7), (p.p92 * locals.var_ccg1_dn8), (p.p92 * locals.var_ccg1_dn9), (p.p92 * locals.var_ccg1_dn10), (p.p92 * locals.var_ccg1_dn11), (p.p92 * locals.var_ccg1_dn13), (p.p92 * locals.var_ccg1_dn14),)
    } else {
        (locals.var_ccg, locals.var_ccg_dn0, locals.var_ccg_dn2, locals.var_ccg_dn3, locals.var_ccg_dn4, locals.var_ccg_dn5, locals.var_ccg_dn6, locals.var_ccg_dn7, locals.var_ccg_dn8, locals.var_ccg_dn9, locals.var_ccg_dn10, locals.var_ccg_dn11, locals.var_ccg_dn13, locals.var_ccg_dn14,)
    }
};
        locals.var_ccg = assign10810_e13844;
        locals.var_ccg_dn0 = assign10810_e13844_d_n0;
        locals.var_ccg_dn2 = assign10810_e13844_d_n2;
        locals.var_ccg_dn3 = assign10810_e13844_d_n3;
        locals.var_ccg_dn4 = assign10810_e13844_d_n4;
        locals.var_ccg_dn5 = assign10810_e13844_d_n5;
        locals.var_ccg_dn6 = assign10810_e13844_d_n6;
        locals.var_ccg_dn7 = assign10810_e13844_d_n7;
        locals.var_ccg_dn8 = assign10810_e13844_d_n8;
        locals.var_ccg_dn9 = assign10810_e13844_d_n9;
        locals.var_ccg_dn10 = assign10810_e13844_d_n10;
        locals.var_ccg_dn11 = assign10810_e13844_d_n11;
        locals.var_ccg_dn13 = assign10810_e13844_d_n13;
        locals.var_ccg_dn14 = assign10810_e13844_d_n14;
        locals.var_ccg_rv = 0.0;

        let (assign10820_e13853,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10820_e13851: f64 = (locals.var_lmax / locals.var_wg);
        (assign10820_e13851,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign10820_e13853;
        locals.var_x_rv = 0.0;

        let (assign10830_e13869,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10830_e13862: f64 = (locals.var_x + 1.0);
        let assign10830_e13863: f64 = (2.0 * assign10830_e13862);
        let assign10830_e13864: f64 = (assign10830_e13863).sqrt();
        let assign10830_e13866: f64 = (assign10830_e13864 * 3.141592653589793);
        let assign10830_e13867: f64 = (4.0 / assign10830_e13866);
        (assign10830_e13867,)
    } else {
        (locals.var_c1,)
    }
};
        locals.var_c1 = assign10830_e13869;
        locals.var_c1_rv = 0.0;

        let (assign10840_e13906,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10840_e13876: f64 = (p.p90 * p.p90);
        let assign10840_e13879: f64 = (2.0 * locals.var_wg);
        let assign10840_e13881: f64 = (assign10840_e13879 * p.p90);
        let assign10840_e13882: f64 = (assign10840_e13876 + assign10840_e13881);
        let assign10840_e13885: f64 = (locals.var_wg * locals.var_wg);
        let assign10840_e13888: f64 = (locals.var_x + 1.0);
        let assign10840_e13889: f64 = (assign10840_e13885 * assign10840_e13888);
        let assign10840_e13890: f64 = (assign10840_e13882 + assign10840_e13889);
        let assign10840_e13891: f64 = (assign10840_e13890).sqrt();
        let assign10840_e13894: f64 = (locals.var_x + 1.0);
        let assign10840_e13895: f64 = (assign10840_e13894).sqrt();
        let assign10840_e13896: f64 = (assign10840_e13891 * assign10840_e13895);
        let assign10840_e13898: f64 = (assign10840_e13896 + p.p90);
        let assign10840_e13901: f64 = (locals.var_wg * locals.var_x);
        let assign10840_e13902: f64 = (assign10840_e13898 + assign10840_e13901);
        let assign10840_e13904: f64 = (assign10840_e13902 + locals.var_wg);
        (assign10840_e13904,)
    } else {
        (locals.var_c2,)
    }
};
        locals.var_c2 = assign10840_e13906;
        locals.var_c2_rv = 0.0;

        let (assign10850_e13928,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10850_e13914: f64 = (locals.var_x + 1.0);
        let assign10850_e13917: f64 = (locals.var_x + 4.0);
        let assign10850_e13918: f64 = (assign10850_e13914 * assign10850_e13917);
        let assign10850_e13919: f64 = (assign10850_e13918).sqrt();
        let assign10850_e13920: f64 = (p.p90 * assign10850_e13919);
        let assign10850_e13924: f64 = (locals.var_x + 2.0);
        let assign10850_e13925: f64 = (p.p90 * assign10850_e13924);
        let assign10850_e13926: f64 = (assign10850_e13920 + assign10850_e13925);
        (assign10850_e13926,)
    } else {
        (locals.var_c3,)
    }
};
        locals.var_c3 = assign10850_e13928;
        locals.var_c3_rv = 0.0;

        let (assign10860_e13960,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10860_e13937: f64 = (locals.var_c2 / locals.var_c3);
        let (assign10860_e13954,) = {
            if (!(assign10860_e13937 > 1e-38)) {
                let assign10860_e13942: f64 = (-87.498233534);
                (assign10860_e13942,)
            } else {
                let assign10860_e13945: f64 = (locals.var_c2 / locals.var_c3);
                let (assign10860_e13953,) = {
                    if (assign10860_e13945 > 1e-38) {
                        let assign10860_e13950: f64 = (locals.var_c2 / locals.var_c3);
                        let assign10860_e13951: f64 = (assign10860_e13950).ln();
                        (assign10860_e13951,)
                    } else {
                        (0.0,)
                    }
                };
                (assign10860_e13953,)
            }
        };
        let assign10860_e13955: f64 = (locals.var_c1 * assign10860_e13954);
        let assign10860_e13957: f64 = (assign10860_e13955 + 12.27);
        let assign10860_e13958: f64 = (locals.var_epssp * assign10860_e13957);
        (assign10860_e13958,)
    } else {
        (locals.var_cfglog,)
    }
};
        locals.var_cfglog = assign10860_e13960;
        locals.var_cfglog_rv = 0.0;

        let (assign10870_e13969,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10870_e13967: f64 = (locals.var_hr * locals.var_lr);
        (assign10870_e13967,)
    } else {
        (locals.var_dcf,)
    }
};
        locals.var_dcf = assign10870_e13969;
        locals.var_dcf_rv = 0.0;

        let (assign10880_e13981,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10880_e13976: f64 = (locals.var_dcf * locals.var_dcf);
        let assign10880_e13978: f64 = (assign10880_e13976 + 1.0);
        let assign10880_e13979: f64 = (assign10880_e13978).sqrt();
        (assign10880_e13979,)
    } else {
        (locals.var_tt0,)
    }
};
        locals.var_tt0 = assign10880_e13981;
        locals.var_tt0_rv = 0.0;

        let (assign10890_e14031, assign10890_e14031_d_n0, assign10890_e14031_d_n2, assign10890_e14031_d_n3, assign10890_e14031_d_n4, assign10890_e14031_d_n5, assign10890_e14031_d_n6, assign10890_e14031_d_n7, assign10890_e14031_d_n8, assign10890_e14031_d_n9, assign10890_e14031_d_n10, assign10890_e14031_d_n11, assign10890_e14031_d_n13, assign10890_e14031_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10890_e13988: f64 = (locals.var_dcf * locals.var_dcf);
        let assign10890_e13990: f64 = (assign10890_e13988 + 1.0);
        let assign10890_e13993: f64 = (locals.var_dcf * p.p90);
        let assign10890_e13996: f64 = (locals.var_dcf * p.p90);
        let assign10890_e13997: f64 = (assign10890_e13993 * assign10890_e13996);
        let assign10890_e14000: f64 = (2.0 * locals.var_dcf);
        let assign10890_e14002: f64 = (assign10890_e14000 * locals.var_lmax);
        let assign10890_e14004: f64 = (assign10890_e14002 * p.p90);
        let assign10890_e14005: f64 = (assign10890_e13997 + assign10890_e14004);
        let assign10890_e14008: f64 = (locals.var_dcf * locals.var_dcf);
        let assign10890_e14010: f64 = (assign10890_e14008 + 1.0);
        let assign10890_e14012: f64 = (assign10890_e14010 * locals.var_lmax);
        let assign10890_e14014: f64 = (assign10890_e14012 * locals.var_lmax);
        let assign10890_e14015: f64 = (assign10890_e14005 + assign10890_e14014);
        let assign10890_e14016: f64 = (assign10890_e13990 * assign10890_e14015);
        let assign10890_e14017: f64 = (assign10890_e14016).sqrt();
        let assign10890_e14020: f64 = (locals.var_dcf * p.p90);
        let assign10890_e14021: f64 = (assign10890_e14017 + assign10890_e14020);
        let assign10890_e14024: f64 = (locals.var_dcf * locals.var_dcf);
        let assign10890_e14026: f64 = (assign10890_e14024 * locals.var_lmax);
        let assign10890_e14027: f64 = (assign10890_e14021 + assign10890_e14026);
        let assign10890_e14029: f64 = (assign10890_e14027 + locals.var_lmax);
        (assign10890_e14029, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign10890_e14031;
        locals.var_tt1_dn0 = assign10890_e14031_d_n0;
        locals.var_tt1_dn2 = assign10890_e14031_d_n2;
        locals.var_tt1_dn3 = assign10890_e14031_d_n3;
        locals.var_tt1_dn4 = assign10890_e14031_d_n4;
        locals.var_tt1_dn5 = assign10890_e14031_d_n5;
        locals.var_tt1_dn6 = assign10890_e14031_d_n6;
        locals.var_tt1_dn7 = assign10890_e14031_d_n7;
        locals.var_tt1_dn8 = assign10890_e14031_d_n8;
        locals.var_tt1_dn9 = assign10890_e14031_d_n9;
        locals.var_tt1_dn10 = assign10890_e14031_d_n10;
        locals.var_tt1_dn11 = assign10890_e14031_d_n11;
        locals.var_tt1_dn13 = assign10890_e14031_d_n13;
        locals.var_tt1_dn14 = assign10890_e14031_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign10900_e14044,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10900_e14038: f64 = (locals.var_tt0 + 1.0);
        let assign10900_e14041: f64 = (locals.var_dcf * p.p90);
        let assign10900_e14042: f64 = (assign10900_e14038 * assign10900_e14041);
        (assign10900_e14042,)
    } else {
        (locals.var_tt2,)
    }
};
        locals.var_tt2 = assign10900_e14044;
        locals.var_tt2_rv = 0.0;

        let (assign10910_e14085, assign10910_e14085_d_n0, assign10910_e14085_d_n2, assign10910_e14085_d_n3, assign10910_e14085_d_n4, assign10910_e14085_d_n5, assign10910_e14085_d_n6, assign10910_e14085_d_n7, assign10910_e14085_d_n8, assign10910_e14085_d_n9, assign10910_e14085_d_n10, assign10910_e14085_d_n11, assign10910_e14085_d_n13, assign10910_e14085_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10910_e14051: f64 = (2.0 * locals.var_epssp);
        let assign10910_e14053: f64 = (2.0_f64).sqrt();
        let assign10910_e14054: f64 = (assign10910_e14051 * assign10910_e14053);
        let assign10910_e14056: f64 = (assign10910_e14054 / 3.141592653589793);
        let assign10910_e14058: f64 = (assign10910_e14056 * 0.85);
        let assign10910_e14060: f64 = (assign10910_e14058 * locals.var_dcf);
        let assign10910_e14062: f64 = (assign10910_e14060 / locals.var_tt0);
        let assign10910_e14065: f64 = (locals.var_tt1 / locals.var_tt2);
        let (assign10910_e14082, assign10910_e14082_d_n0, assign10910_e14082_d_n2, assign10910_e14082_d_n3, assign10910_e14082_d_n4, assign10910_e14082_d_n5, assign10910_e14082_d_n6, assign10910_e14082_d_n7, assign10910_e14082_d_n8, assign10910_e14082_d_n9, assign10910_e14082_d_n10, assign10910_e14082_d_n11, assign10910_e14082_d_n13, assign10910_e14082_d_n14,) = {
            if (!(assign10910_e14065 > 1e-38)) {
                let assign10910_e14070: f64 = (-87.498233534);
                (assign10910_e14070, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign10910_e14073: f64 = (locals.var_tt1 / locals.var_tt2);
                let (assign10910_e14081, assign10910_e14081_d_n0, assign10910_e14081_d_n2, assign10910_e14081_d_n3, assign10910_e14081_d_n4, assign10910_e14081_d_n5, assign10910_e14081_d_n6, assign10910_e14081_d_n7, assign10910_e14081_d_n8, assign10910_e14081_d_n9, assign10910_e14081_d_n10, assign10910_e14081_d_n11, assign10910_e14081_d_n13, assign10910_e14081_d_n14,) = {
                    if (assign10910_e14073 > 1e-38) {
                        let assign10910_e14078: f64 = (locals.var_tt1 / locals.var_tt2);
                        let assign10910_e14079: f64 = (assign10910_e14078).ln();
                        (assign10910_e14079, ((locals.var_tt1_dn0 / locals.var_tt2) / assign10910_e14078), ((locals.var_tt1_dn2 / locals.var_tt2) / assign10910_e14078), ((locals.var_tt1_dn3 / locals.var_tt2) / assign10910_e14078), ((locals.var_tt1_dn4 / locals.var_tt2) / assign10910_e14078), ((locals.var_tt1_dn5 / locals.var_tt2) / assign10910_e14078), ((locals.var_tt1_dn6 / locals.var_tt2) / assign10910_e14078), ((locals.var_tt1_dn7 / locals.var_tt2) / assign10910_e14078), ((locals.var_tt1_dn8 / locals.var_tt2) / assign10910_e14078), ((locals.var_tt1_dn9 / locals.var_tt2) / assign10910_e14078), ((locals.var_tt1_dn10 / locals.var_tt2) / assign10910_e14078), ((locals.var_tt1_dn11 / locals.var_tt2) / assign10910_e14078), ((locals.var_tt1_dn13 / locals.var_tt2) / assign10910_e14078), ((locals.var_tt1_dn14 / locals.var_tt2) / assign10910_e14078),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign10910_e14081, assign10910_e14081_d_n0, assign10910_e14081_d_n2, assign10910_e14081_d_n3, assign10910_e14081_d_n4, assign10910_e14081_d_n5, assign10910_e14081_d_n6, assign10910_e14081_d_n7, assign10910_e14081_d_n8, assign10910_e14081_d_n9, assign10910_e14081_d_n10, assign10910_e14081_d_n11, assign10910_e14081_d_n13, assign10910_e14081_d_n14,)
            }
        };
        let assign10910_e14083: f64 = (assign10910_e14062 * assign10910_e14082);
        (assign10910_e14083, (assign10910_e14062 * assign10910_e14082_d_n0), (assign10910_e14062 * assign10910_e14082_d_n2), (assign10910_e14062 * assign10910_e14082_d_n3), (assign10910_e14062 * assign10910_e14082_d_n4), (assign10910_e14062 * assign10910_e14082_d_n5), (assign10910_e14062 * assign10910_e14082_d_n6), (assign10910_e14062 * assign10910_e14082_d_n7), (assign10910_e14062 * assign10910_e14082_d_n8), (assign10910_e14062 * assign10910_e14082_d_n9), (assign10910_e14062 * assign10910_e14082_d_n10), (assign10910_e14062 * assign10910_e14082_d_n11), (assign10910_e14062 * assign10910_e14082_d_n13), (assign10910_e14062 * assign10910_e14082_d_n14),)
    } else {
        (locals.var_cfgsat, locals.var_cfgsat_dn0, locals.var_cfgsat_dn2, locals.var_cfgsat_dn3, locals.var_cfgsat_dn4, locals.var_cfgsat_dn5, locals.var_cfgsat_dn6, locals.var_cfgsat_dn7, locals.var_cfgsat_dn8, locals.var_cfgsat_dn9, locals.var_cfgsat_dn10, locals.var_cfgsat_dn11, locals.var_cfgsat_dn13, locals.var_cfgsat_dn14,)
    }
};
        locals.var_cfgsat = assign10910_e14085;
        locals.var_cfgsat_dn0 = assign10910_e14085_d_n0;
        locals.var_cfgsat_dn2 = assign10910_e14085_d_n2;
        locals.var_cfgsat_dn3 = assign10910_e14085_d_n3;
        locals.var_cfgsat_dn4 = assign10910_e14085_d_n4;
        locals.var_cfgsat_dn5 = assign10910_e14085_d_n5;
        locals.var_cfgsat_dn6 = assign10910_e14085_d_n6;
        locals.var_cfgsat_dn7 = assign10910_e14085_d_n7;
        locals.var_cfgsat_dn8 = assign10910_e14085_d_n8;
        locals.var_cfgsat_dn9 = assign10910_e14085_d_n9;
        locals.var_cfgsat_dn10 = assign10910_e14085_d_n10;
        locals.var_cfgsat_dn11 = assign10910_e14085_d_n11;
        locals.var_cfgsat_dn13 = assign10910_e14085_d_n13;
        locals.var_cfgsat_dn14 = assign10910_e14085_d_n14;
        locals.var_cfgsat_rv = 0.0;

        let (assign10920_e14092, assign10920_e14092_d_n0, assign10920_e14092_d_n2, assign10920_e14092_d_n3, assign10920_e14092_d_n4, assign10920_e14092_d_n5, assign10920_e14092_d_n6, assign10920_e14092_d_n7, assign10920_e14092_d_n8, assign10920_e14092_d_n9, assign10920_e14092_d_n10, assign10920_e14092_d_n11, assign10920_e14092_d_n13, assign10920_e14092_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        (1.2e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign10920_e14092;
        locals.var_delta_dn0 = assign10920_e14092_d_n0;
        locals.var_delta_dn2 = assign10920_e14092_d_n2;
        locals.var_delta_dn3 = assign10920_e14092_d_n3;
        locals.var_delta_dn4 = assign10920_e14092_d_n4;
        locals.var_delta_dn5 = assign10920_e14092_d_n5;
        locals.var_delta_dn6 = assign10920_e14092_d_n6;
        locals.var_delta_dn7 = assign10920_e14092_d_n7;
        locals.var_delta_dn8 = assign10920_e14092_d_n8;
        locals.var_delta_dn9 = assign10920_e14092_d_n9;
        locals.var_delta_dn10 = assign10920_e14092_d_n10;
        locals.var_delta_dn11 = assign10920_e14092_d_n11;
        locals.var_delta_dn13 = assign10920_e14092_d_n13;
        locals.var_delta_dn14 = assign10920_e14092_d_n14;
        locals.var_delta_rv = 0.0;

        let (assign10930_e14103, assign10930_e14103_d_n0, assign10930_e14103_d_n2, assign10930_e14103_d_n3, assign10930_e14103_d_n4, assign10930_e14103_d_n5, assign10930_e14103_d_n6, assign10930_e14103_d_n7, assign10930_e14103_d_n8, assign10930_e14103_d_n9, assign10930_e14103_d_n10, assign10930_e14103_d_n11, assign10930_e14103_d_n13, assign10930_e14103_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10930_e14099: f64 = (locals.var_cfgsat - locals.var_cfglog);
        let assign10930_e14101: f64 = (assign10930_e14099 - locals.var_delta);
        (assign10930_e14101, (locals.var_cfgsat_dn0 - locals.var_delta_dn0), (locals.var_cfgsat_dn2 - locals.var_delta_dn2), (locals.var_cfgsat_dn3 - locals.var_delta_dn3), (locals.var_cfgsat_dn4 - locals.var_delta_dn4), (locals.var_cfgsat_dn5 - locals.var_delta_dn5), (locals.var_cfgsat_dn6 - locals.var_delta_dn6), (locals.var_cfgsat_dn7 - locals.var_delta_dn7), (locals.var_cfgsat_dn8 - locals.var_delta_dn8), (locals.var_cfgsat_dn9 - locals.var_delta_dn9), (locals.var_cfgsat_dn10 - locals.var_delta_dn10), (locals.var_cfgsat_dn11 - locals.var_delta_dn11), (locals.var_cfgsat_dn13 - locals.var_delta_dn13), (locals.var_cfgsat_dn14 - locals.var_delta_dn14),)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign10930_e14103;
        locals.var_tt1_dn0 = assign10930_e14103_d_n0;
        locals.var_tt1_dn2 = assign10930_e14103_d_n2;
        locals.var_tt1_dn3 = assign10930_e14103_d_n3;
        locals.var_tt1_dn4 = assign10930_e14103_d_n4;
        locals.var_tt1_dn5 = assign10930_e14103_d_n5;
        locals.var_tt1_dn6 = assign10930_e14103_d_n6;
        locals.var_tt1_dn7 = assign10930_e14103_d_n7;
        locals.var_tt1_dn8 = assign10930_e14103_d_n8;
        locals.var_tt1_dn9 = assign10930_e14103_d_n9;
        locals.var_tt1_dn10 = assign10930_e14103_d_n10;
        locals.var_tt1_dn11 = assign10930_e14103_d_n11;
        locals.var_tt1_dn13 = assign10930_e14103_d_n13;
        locals.var_tt1_dn14 = assign10930_e14103_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign10940_e14127, assign10940_e14127_d_n0, assign10940_e14127_d_n2, assign10940_e14127_d_n3, assign10940_e14127_d_n4, assign10940_e14127_d_n5, assign10940_e14127_d_n6, assign10940_e14127_d_n7, assign10940_e14127_d_n8, assign10940_e14127_d_n9, assign10940_e14127_d_n10, assign10940_e14127_d_n11, assign10940_e14127_d_n13, assign10940_e14127_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10940_e14114: f64 = (locals.var_tt1 * locals.var_tt1);
        let assign10940_e14117: f64 = (4.0 * locals.var_delta);
        let assign10940_e14119: f64 = (assign10940_e14117 * locals.var_cfgsat);
        let assign10940_e14120: f64 = (assign10940_e14114 + assign10940_e14119);
        let assign10940_e14121: f64 = (assign10940_e14120).sqrt();
        let assign10940_e14122: f64 = (locals.var_tt1 + assign10940_e14121);
        let assign10940_e14123: f64 = (0.5 * assign10940_e14122);
        let assign10940_e14124: f64 = (locals.var_cfgsat - assign10940_e14123);
        let assign10940_e14125: f64 = (p.p92 * assign10940_e14124);
        (assign10940_e14125, (p.p92 * (locals.var_cfgsat_dn0 - (0.5 * (locals.var_tt1_dn0 + ((((locals.var_tt1_dn0 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn0))) / (2.0 * assign10940_e14121)))))), (p.p92 * (locals.var_cfgsat_dn2 - (0.5 * (locals.var_tt1_dn2 + ((((locals.var_tt1_dn2 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn2))) / (2.0 * assign10940_e14121)))))), (p.p92 * (locals.var_cfgsat_dn3 - (0.5 * (locals.var_tt1_dn3 + ((((locals.var_tt1_dn3 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn3)) + (((4.0 * locals.var_delta_dn3) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn3))) / (2.0 * assign10940_e14121)))))), (p.p92 * (locals.var_cfgsat_dn4 - (0.5 * (locals.var_tt1_dn4 + ((((locals.var_tt1_dn4 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn4)) + (((4.0 * locals.var_delta_dn4) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn4))) / (2.0 * assign10940_e14121)))))), (p.p92 * (locals.var_cfgsat_dn5 - (0.5 * (locals.var_tt1_dn5 + ((((locals.var_tt1_dn5 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn5)) + (((4.0 * locals.var_delta_dn5) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn5))) / (2.0 * assign10940_e14121)))))), (p.p92 * (locals.var_cfgsat_dn6 - (0.5 * (locals.var_tt1_dn6 + ((((locals.var_tt1_dn6 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn6))) / (2.0 * assign10940_e14121)))))), (p.p92 * (locals.var_cfgsat_dn7 - (0.5 * (locals.var_tt1_dn7 + ((((locals.var_tt1_dn7 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn7))) / (2.0 * assign10940_e14121)))))), (p.p92 * (locals.var_cfgsat_dn8 - (0.5 * (locals.var_tt1_dn8 + ((((locals.var_tt1_dn8 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn8)) + (((4.0 * locals.var_delta_dn8) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn8))) / (2.0 * assign10940_e14121)))))), (p.p92 * (locals.var_cfgsat_dn9 - (0.5 * (locals.var_tt1_dn9 + ((((locals.var_tt1_dn9 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn9)) + (((4.0 * locals.var_delta_dn9) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn9))) / (2.0 * assign10940_e14121)))))), (p.p92 * (locals.var_cfgsat_dn10 - (0.5 * (locals.var_tt1_dn10 + ((((locals.var_tt1_dn10 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn10))) / (2.0 * assign10940_e14121)))))), (p.p92 * (locals.var_cfgsat_dn11 - (0.5 * (locals.var_tt1_dn11 + ((((locals.var_tt1_dn11 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn11))) / (2.0 * assign10940_e14121)))))), (p.p92 * (locals.var_cfgsat_dn13 - (0.5 * (locals.var_tt1_dn13 + ((((locals.var_tt1_dn13 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn13)) + (((4.0 * locals.var_delta_dn13) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn13))) / (2.0 * assign10940_e14121)))))), (p.p92 * (locals.var_cfgsat_dn14 - (0.5 * (locals.var_tt1_dn14 + ((((locals.var_tt1_dn14 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn14)) + (((4.0 * locals.var_delta_dn14) * locals.var_cfgsat) + (assign10940_e14117 * locals.var_cfgsat_dn14))) / (2.0 * assign10940_e14121)))))),)
    } else {
        (locals.var_cfg, locals.var_cfg_dn0, locals.var_cfg_dn2, locals.var_cfg_dn3, locals.var_cfg_dn4, locals.var_cfg_dn5, locals.var_cfg_dn6, locals.var_cfg_dn7, locals.var_cfg_dn8, locals.var_cfg_dn9, locals.var_cfg_dn10, locals.var_cfg_dn11, locals.var_cfg_dn13, locals.var_cfg_dn14,)
    }
};
        locals.var_cfg = assign10940_e14127;
        locals.var_cfg_dn0 = assign10940_e14127_d_n0;
        locals.var_cfg_dn2 = assign10940_e14127_d_n2;
        locals.var_cfg_dn3 = assign10940_e14127_d_n3;
        locals.var_cfg_dn4 = assign10940_e14127_d_n4;
        locals.var_cfg_dn5 = assign10940_e14127_d_n5;
        locals.var_cfg_dn6 = assign10940_e14127_d_n6;
        locals.var_cfg_dn7 = assign10940_e14127_d_n7;
        locals.var_cfg_dn8 = assign10940_e14127_d_n8;
        locals.var_cfg_dn9 = assign10940_e14127_d_n9;
        locals.var_cfg_dn10 = assign10940_e14127_d_n10;
        locals.var_cfg_dn11 = assign10940_e14127_d_n11;
        locals.var_cfg_dn13 = assign10940_e14127_d_n13;
        locals.var_cfg_dn14 = assign10940_e14127_d_n14;
        locals.var_cfg_rv = 0.0;

        let (assign10950_e14136, assign10950_e14136_d_n0, assign10950_e14136_d_n2, assign10950_e14136_d_n3, assign10950_e14136_d_n4, assign10950_e14136_d_n5, assign10950_e14136_d_n6, assign10950_e14136_d_n7, assign10950_e14136_d_n8, assign10950_e14136_d_n9, assign10950_e14136_d_n10, assign10950_e14136_d_n11, assign10950_e14136_d_n13, assign10950_e14136_d_n14,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard208 == 0.0)) {
        let assign10950_e14134: f64 = (locals.var_ccg + locals.var_cfg);
        (assign10950_e14134, (locals.var_ccg_dn0 + locals.var_cfg_dn0), (locals.var_ccg_dn2 + locals.var_cfg_dn2), (locals.var_ccg_dn3 + locals.var_cfg_dn3), (locals.var_ccg_dn4 + locals.var_cfg_dn4), (locals.var_ccg_dn5 + locals.var_cfg_dn5), (locals.var_ccg_dn6 + locals.var_cfg_dn6), (locals.var_ccg_dn7 + locals.var_cfg_dn7), (locals.var_ccg_dn8 + locals.var_cfg_dn8), (locals.var_ccg_dn9 + locals.var_cfg_dn9), (locals.var_ccg_dn10 + locals.var_cfg_dn10), (locals.var_ccg_dn11 + locals.var_cfg_dn11), (locals.var_ccg_dn13 + locals.var_cfg_dn13), (locals.var_ccg_dn14 + locals.var_cfg_dn14),)
    } else {
        (locals.var_cgg_side, locals.var_cgg_side_dn0, locals.var_cgg_side_dn2, locals.var_cgg_side_dn3, locals.var_cgg_side_dn4, locals.var_cgg_side_dn5, locals.var_cgg_side_dn6, locals.var_cgg_side_dn7, locals.var_cgg_side_dn8, locals.var_cgg_side_dn9, locals.var_cgg_side_dn10, locals.var_cgg_side_dn11, locals.var_cgg_side_dn13, locals.var_cgg_side_dn14,)
    }
};
        locals.var_cgg_side = assign10950_e14136;
        locals.var_cgg_side_dn0 = assign10950_e14136_d_n0;
        locals.var_cgg_side_dn2 = assign10950_e14136_d_n2;
        locals.var_cgg_side_dn3 = assign10950_e14136_d_n3;
        locals.var_cgg_side_dn4 = assign10950_e14136_d_n4;
        locals.var_cgg_side_dn5 = assign10950_e14136_d_n5;
        locals.var_cgg_side_dn6 = assign10950_e14136_d_n6;
        locals.var_cgg_side_dn7 = assign10950_e14136_d_n7;
        locals.var_cgg_side_dn8 = assign10950_e14136_d_n8;
        locals.var_cgg_side_dn9 = assign10950_e14136_d_n9;
        locals.var_cgg_side_dn10 = assign10950_e14136_d_n10;
        locals.var_cgg_side_dn11 = assign10950_e14136_d_n11;
        locals.var_cgg_side_dn13 = assign10950_e14136_d_n13;
        locals.var_cgg_side_dn14 = assign10950_e14136_d_n14;
        locals.var_cgg_side_rv = 0.0;

        let assign10960_e14139: f64 = if p.p1090 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard211 = assign10960_e14139;
        locals.var_guard211_rv = 0.0;

        let (assign10970_e14145,) = {
    if ((locals.var_guard205 != 0.0) && (locals.var_guard211 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_acorner,)
    }
};
        locals.var_acorner = assign10970_e14145;
        locals.var_acorner_rv = 0.0;

        let assign10980_e14148: f64 = if p.p1080 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard212 = assign10980_e14148;
        locals.var_guard212_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10990_e14165,) = {
    if (((locals.var_guard205 != 0.0) && (locals.var_guard211 == 0.0)) && (locals.var_guard212 != 0.0)) {
        let assign10990_e14157: f64 = (p.p4 - p.p3);
        let assign10990_e14160: f64 = (p.p1080 * p.p1084);
        let assign10990_e14162: f64 = (assign10990_e14160 + p.p1081);
        let assign10990_e14163: f64 = (assign10990_e14157 * assign10990_e14162);
        (assign10990_e14163,)
    } else {
        (locals.var_acorner,)
    }
};
        locals.var_acorner = assign10990_e14165;
        locals.var_acorner_rv = 0.0;

        let (assign11000_e14179,) = {
    if (((locals.var_guard205 != 0.0) && (locals.var_guard211 == 0.0)) && (locals.var_guard212 == 0.0)) {
        let assign11000_e14175: f64 = (p.p4 - p.p3);
        let assign11000_e14177: f64 = (assign11000_e14175 * locals.var_hrsd);
        (assign11000_e14177,)
    } else {
        (locals.var_acorner,)
    }
};
        locals.var_acorner = assign11000_e14179;
        locals.var_acorner_rv = 0.0;

        let (assign11010_e14193,) = {
    if (locals.var_guard205 != 0.0) {
        let assign11010_e14183: f64 = (p.p5 * locals.var_acorner);
        let assign11010_e14185: f64 = (assign11010_e14183 + p.p1092);
        let assign11010_e14187: f64 = (assign11010_e14185 + p.p1091);
        let assign11010_e14189: f64 = (assign11010_e14187 * locals.var_epssp);
        let assign11010_e14191: f64 = (assign11010_e14189 / p.p1087);
        (assign11010_e14191,)
    } else {
        (locals.var_ccorner,)
    }
};
        locals.var_ccorner = assign11010_e14193;
        locals.var_ccorner_rv = 0.0;

        let (assign11020_e14211, assign11020_e14211_d_n0, assign11020_e14211_d_n2, assign11020_e14211_d_n3, assign11020_e14211_d_n4, assign11020_e14211_d_n5, assign11020_e14211_d_n6, assign11020_e14211_d_n7, assign11020_e14211_d_n8, assign11020_e14211_d_n9, assign11020_e14211_d_n10, assign11020_e14211_d_n11, assign11020_e14211_d_n13, assign11020_e14211_d_n14,) = {
    if (locals.var_guard205 != 0.0) {
        let assign11020_e14198: f64 = (locals.var_cgg_top * p.p5);
        let assign11020_e14199: f64 = (locals.var_ccorner + assign11020_e14198);
        let assign11020_e14202: f64 = (p.p1103 * locals.var_cgg_side);
        let assign11020_e14204: f64 = (assign11020_e14202 * p.p5);
        let assign11020_e14206: f64 = (assign11020_e14204 * 2.0);
        let assign11020_e14207: f64 = (assign11020_e14199 + assign11020_e14206);
        let assign11020_e14209: f64 = (assign11020_e14207 * p.p59);
        (assign11020_e14209, (((locals.var_cgg_top_dn0 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn0) * p.p5) * 2.0)) * p.p59), (((locals.var_cgg_top_dn2 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn2) * p.p5) * 2.0)) * p.p59), (((locals.var_cgg_top_dn3 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn3) * p.p5) * 2.0)) * p.p59), (((locals.var_cgg_top_dn4 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn4) * p.p5) * 2.0)) * p.p59), (((locals.var_cgg_top_dn5 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn5) * p.p5) * 2.0)) * p.p59), (((locals.var_cgg_top_dn6 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn6) * p.p5) * 2.0)) * p.p59), (((locals.var_cgg_top_dn7 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn7) * p.p5) * 2.0)) * p.p59), (((locals.var_cgg_top_dn8 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn8) * p.p5) * 2.0)) * p.p59), (((locals.var_cgg_top_dn9 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn9) * p.p5) * 2.0)) * p.p59), (((locals.var_cgg_top_dn10 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn10) * p.p5) * 2.0)) * p.p59), (((locals.var_cgg_top_dn11 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn11) * p.p5) * 2.0)) * p.p59), (((locals.var_cgg_top_dn13 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn13) * p.p5) * 2.0)) * p.p59), (((locals.var_cgg_top_dn14 * p.p5) + (((p.p1103 * locals.var_cgg_side_dn14) * p.p5) * 2.0)) * p.p59),)
    } else {
        (locals.var_cfr_geo, locals.var_cfr_geo_dn0, locals.var_cfr_geo_dn2, locals.var_cfr_geo_dn3, locals.var_cfr_geo_dn4, locals.var_cfr_geo_dn5, locals.var_cfr_geo_dn6, locals.var_cfr_geo_dn7, locals.var_cfr_geo_dn8, locals.var_cfr_geo_dn9, locals.var_cfr_geo_dn10, locals.var_cfr_geo_dn11, locals.var_cfr_geo_dn13, locals.var_cfr_geo_dn14,)
    }
};
        locals.var_cfr_geo = assign11020_e14211;
        locals.var_cfr_geo_dn0 = assign11020_e14211_d_n0;
        locals.var_cfr_geo_dn2 = assign11020_e14211_d_n2;
        locals.var_cfr_geo_dn3 = assign11020_e14211_d_n3;
        locals.var_cfr_geo_dn4 = assign11020_e14211_d_n4;
        locals.var_cfr_geo_dn5 = assign11020_e14211_d_n5;
        locals.var_cfr_geo_dn6 = assign11020_e14211_d_n6;
        locals.var_cfr_geo_dn7 = assign11020_e14211_d_n7;
        locals.var_cfr_geo_dn8 = assign11020_e14211_d_n8;
        locals.var_cfr_geo_dn9 = assign11020_e14211_d_n9;
        locals.var_cfr_geo_dn10 = assign11020_e14211_d_n10;
        locals.var_cfr_geo_dn11 = assign11020_e14211_d_n11;
        locals.var_cfr_geo_dn13 = assign11020_e14211_d_n13;
        locals.var_cfr_geo_dn14 = assign11020_e14211_d_n14;
        locals.var_cfr_geo_rv = 0.0;

        let (assign11030_e14231, assign11030_e14231_d_n0, assign11030_e14231_d_n2, assign11030_e14231_d_n3, assign11030_e14231_d_n4, assign11030_e14231_d_n5, assign11030_e14231_d_n6, assign11030_e14231_d_n7, assign11030_e14231_d_n8, assign11030_e14231_d_n9, assign11030_e14231_d_n10, assign11030_e14231_d_n11, assign11030_e14231_d_n13, assign11030_e14231_d_n14,) = {
    if (locals.var_guard205 != 0.0) {
        let assign11030_e14218: f64 = (p.p1100 * p.p3);
        let assign11030_e14219: f64 = (p.p1099 + assign11030_e14218);
        let assign11030_e14222: f64 = (p.p1101 * p.p4);
        let assign11030_e14223: f64 = (assign11030_e14219 + assign11030_e14222);
        let assign11030_e14226: f64 = (p.p1102 * p.p20);
        let assign11030_e14227: f64 = (assign11030_e14223 + assign11030_e14226);
        let assign11030_e14228: f64 = (0.0_f64).max(assign11030_e14227);
        let assign11030_e14229: f64 = (locals.var_cfr_geo * assign11030_e14228);
        (assign11030_e14229, (locals.var_cfr_geo_dn0 * assign11030_e14228), (locals.var_cfr_geo_dn2 * assign11030_e14228), (locals.var_cfr_geo_dn3 * assign11030_e14228), (locals.var_cfr_geo_dn4 * assign11030_e14228), (locals.var_cfr_geo_dn5 * assign11030_e14228), (locals.var_cfr_geo_dn6 * assign11030_e14228), (locals.var_cfr_geo_dn7 * assign11030_e14228), (locals.var_cfr_geo_dn8 * assign11030_e14228), (locals.var_cfr_geo_dn9 * assign11030_e14228), (locals.var_cfr_geo_dn10 * assign11030_e14228), (locals.var_cfr_geo_dn11 * assign11030_e14228), (locals.var_cfr_geo_dn13 * assign11030_e14228), (locals.var_cfr_geo_dn14 * assign11030_e14228),)
    } else {
        (locals.var_cfr_geo, locals.var_cfr_geo_dn0, locals.var_cfr_geo_dn2, locals.var_cfr_geo_dn3, locals.var_cfr_geo_dn4, locals.var_cfr_geo_dn5, locals.var_cfr_geo_dn6, locals.var_cfr_geo_dn7, locals.var_cfr_geo_dn8, locals.var_cfr_geo_dn9, locals.var_cfr_geo_dn10, locals.var_cfr_geo_dn11, locals.var_cfr_geo_dn13, locals.var_cfr_geo_dn14,)
    }
};
        locals.var_cfr_geo = assign11030_e14231;
        locals.var_cfr_geo_dn0 = assign11030_e14231_d_n0;
        locals.var_cfr_geo_dn2 = assign11030_e14231_d_n2;
        locals.var_cfr_geo_dn3 = assign11030_e14231_d_n3;
        locals.var_cfr_geo_dn4 = assign11030_e14231_d_n4;
        locals.var_cfr_geo_dn5 = assign11030_e14231_d_n5;
        locals.var_cfr_geo_dn6 = assign11030_e14231_d_n6;
        locals.var_cfr_geo_dn7 = assign11030_e14231_d_n7;
        locals.var_cfr_geo_dn8 = assign11030_e14231_d_n8;
        locals.var_cfr_geo_dn9 = assign11030_e14231_d_n9;
        locals.var_cfr_geo_dn10 = assign11030_e14231_d_n10;
        locals.var_cfr_geo_dn11 = assign11030_e14231_d_n11;
        locals.var_cfr_geo_dn13 = assign11030_e14231_d_n13;
        locals.var_cfr_geo_dn14 = assign11030_e14231_d_n14;
        locals.var_cfr_geo_rv = 0.0;

        let assign11040_e14234: f64 = if p.p78 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard213 = assign11040_e14234;
        locals.var_guard213_rv = 0.0;

        let (assign11050_e14240,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11050_e14238: f64 = (p.p1089 + p.p1090);
        (assign11050_e14238,)
    } else {
        (locals.var_hg,)
    }
};
        locals.var_hg = assign11050_e14240;
        locals.var_hg_rv = 0.0;

        let (assign11060_e14248,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11060_e14245: f64 = (p.p4 - p.p43);
        let assign11060_e14246: f64 = (0.5 * assign11060_e14245);
        (assign11060_e14246,)
    } else {
        (locals.var_trsd,)
    }
};
        locals.var_trsd = assign11060_e14248;
        locals.var_trsd_rv = 0.0;

        let (assign11070_e14256,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11070_e14253: f64 = (locals.var_trsd - p.p90);
        let assign11070_e14254: f64 = (0.0_f64).max(assign11070_e14253);
        (assign11070_e14254,)
    } else {
        (locals.var_wg,)
    }
};
        locals.var_wg = assign11070_e14256;
        locals.var_wg_rv = 0.0;

        let (assign11080_e14264,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11080_e14261: f64 = (p.p1080 + p.p1081);
        let assign11080_e14262: f64 = (0.0_f64).max(assign11080_e14261);
        (assign11080_e14262,)
    } else {
        (locals.var_hrsd,)
    }
};
        locals.var_hrsd = assign11080_e14264;
        locals.var_hrsd_rv = 0.0;

        let (assign11090_e14270,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11090_e14268: f64 = (0.5 * p.p41);
        (assign11090_e14268,)
    } else {
        (locals.var_hrsd2,)
    }
};
        locals.var_hrsd2 = assign11090_e14270;
        locals.var_hrsd2_rv = 0.0;

        let assign11100_e14273: f64 = if p.p1090 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard214 = assign11100_e14273;
        locals.var_guard214_rv = 0.0;

        let (assign11110_e14312, assign11110_e14312_d_n0, assign11110_e14312_d_n2, assign11110_e14312_d_n3, assign11110_e14312_d_n4, assign11110_e14312_d_n5, assign11110_e14312_d_n6, assign11110_e14312_d_n7, assign11110_e14312_d_n8, assign11110_e14312_d_n9, assign11110_e14312_d_n10, assign11110_e14312_d_n11, assign11110_e14312_d_n13, assign11110_e14312_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 != 0.0)) {
        let assign11110_e14280: f64 = (1e-7 * p.p1088);
        let assign11110_e14283: f64 = (3.9 * p.p1087);
        let assign11110_e14284: f64 = (assign11110_e14280 / assign11110_e14283);
        let (assign11110_e14309,) = {
            if (!(assign11110_e14284 > 1e-38)) {
                let assign11110_e14289: f64 = (-87.498233534);
                (assign11110_e14289,)
            } else {
                let assign11110_e14292: f64 = (1e-7 * p.p1088);
                let assign11110_e14295: f64 = (3.9 * p.p1087);
                let assign11110_e14296: f64 = (assign11110_e14292 / assign11110_e14295);
                let (assign11110_e14308,) = {
                    if (assign11110_e14296 > 1e-38) {
                        let assign11110_e14301: f64 = (1e-7 * p.p1088);
                        let assign11110_e14304: f64 = (3.9 * p.p1087);
                        let assign11110_e14305: f64 = (assign11110_e14301 / assign11110_e14304);
                        let assign11110_e14306: f64 = (assign11110_e14305).ln();
                        (assign11110_e14306,)
                    } else {
                        (0.0,)
                    }
                };
                (assign11110_e14308,)
            }
        };
        let assign11110_e14310: f64 = (3.467e-11 * assign11110_e14309);
        (assign11110_e14310, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign11110_e14312;
        locals.var_t0_dn0 = assign11110_e14312_d_n0;
        locals.var_t0_dn2 = assign11110_e14312_d_n2;
        locals.var_t0_dn3 = assign11110_e14312_d_n3;
        locals.var_t0_dn4 = assign11110_e14312_d_n4;
        locals.var_t0_dn5 = assign11110_e14312_d_n5;
        locals.var_t0_dn6 = assign11110_e14312_d_n6;
        locals.var_t0_dn7 = assign11110_e14312_d_n7;
        locals.var_t0_dn8 = assign11110_e14312_d_n8;
        locals.var_t0_dn9 = assign11110_e14312_d_n9;
        locals.var_t0_dn10 = assign11110_e14312_d_n10;
        locals.var_t0_dn11 = assign11110_e14312_d_n11;
        locals.var_t0_dn13 = assign11110_e14312_d_n13;
        locals.var_t0_dn14 = assign11110_e14312_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign11120_e14324, assign11120_e14324_d_n0, assign11120_e14324_d_n2, assign11120_e14324_d_n3, assign11120_e14324_d_n4, assign11120_e14324_d_n5, assign11120_e14324_d_n6, assign11120_e14324_d_n7, assign11120_e14324_d_n8, assign11120_e14324_d_n9, assign11120_e14324_d_n10, assign11120_e14324_d_n11, assign11120_e14324_d_n13, assign11120_e14324_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 != 0.0)) {
        let assign11120_e14318: f64 = (0.942 * locals.var_hrsd);
        let assign11120_e14320: f64 = (assign11120_e14318 * locals.var_epssp);
        let assign11120_e14322: f64 = (assign11120_e14320 / p.p1087);
        (assign11120_e14322, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign11120_e14324;
        locals.var_t1_dn0 = assign11120_e14324_d_n0;
        locals.var_t1_dn2 = assign11120_e14324_d_n2;
        locals.var_t1_dn3 = assign11120_e14324_d_n3;
        locals.var_t1_dn4 = assign11120_e14324_d_n4;
        locals.var_t1_dn5 = assign11120_e14324_d_n5;
        locals.var_t1_dn6 = assign11120_e14324_d_n6;
        locals.var_t1_dn7 = assign11120_e14324_d_n7;
        locals.var_t1_dn8 = assign11120_e14324_d_n8;
        locals.var_t1_dn9 = assign11120_e14324_d_n9;
        locals.var_t1_dn10 = assign11120_e14324_d_n10;
        locals.var_t1_dn11 = assign11120_e14324_d_n11;
        locals.var_t1_dn13 = assign11120_e14324_d_n13;
        locals.var_t1_dn14 = assign11120_e14324_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign11130_e14340, assign11130_e14340_d_n0, assign11130_e14340_d_n2, assign11130_e14340_d_n3, assign11130_e14340_d_n4, assign11130_e14340_d_n5, assign11130_e14340_d_n6, assign11130_e14340_d_n7, assign11130_e14340_d_n8, assign11130_e14340_d_n9, assign11130_e14340_d_n10, assign11130_e14340_d_n11, assign11130_e14340_d_n13, assign11130_e14340_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 != 0.0)) {
        let assign11130_e14330: f64 = (locals.var_t0 + locals.var_t1);
        let assign11130_e14334: f64 = (p.p4 - p.p43);
        let assign11130_e14336: f64 = (assign11130_e14334 * p.p1084);
        let assign11130_e14337: f64 = (p.p43 + assign11130_e14336);
        let assign11130_e14338: f64 = (assign11130_e14330 * assign11130_e14337);
        (assign11130_e14338, ((locals.var_t0_dn0 + locals.var_t1_dn0) * assign11130_e14337), ((locals.var_t0_dn2 + locals.var_t1_dn2) * assign11130_e14337), ((locals.var_t0_dn3 + locals.var_t1_dn3) * assign11130_e14337), ((locals.var_t0_dn4 + locals.var_t1_dn4) * assign11130_e14337), ((locals.var_t0_dn5 + locals.var_t1_dn5) * assign11130_e14337), ((locals.var_t0_dn6 + locals.var_t1_dn6) * assign11130_e14337), ((locals.var_t0_dn7 + locals.var_t1_dn7) * assign11130_e14337), ((locals.var_t0_dn8 + locals.var_t1_dn8) * assign11130_e14337), ((locals.var_t0_dn9 + locals.var_t1_dn9) * assign11130_e14337), ((locals.var_t0_dn10 + locals.var_t1_dn10) * assign11130_e14337), ((locals.var_t0_dn11 + locals.var_t1_dn11) * assign11130_e14337), ((locals.var_t0_dn13 + locals.var_t1_dn13) * assign11130_e14337), ((locals.var_t0_dn14 + locals.var_t1_dn14) * assign11130_e14337),)
    } else {
        (locals.var_cgg_topm, locals.var_cgg_topm_dn0, locals.var_cgg_topm_dn2, locals.var_cgg_topm_dn3, locals.var_cgg_topm_dn4, locals.var_cgg_topm_dn5, locals.var_cgg_topm_dn6, locals.var_cgg_topm_dn7, locals.var_cgg_topm_dn8, locals.var_cgg_topm_dn9, locals.var_cgg_topm_dn10, locals.var_cgg_topm_dn11, locals.var_cgg_topm_dn13, locals.var_cgg_topm_dn14,)
    }
};
        locals.var_cgg_topm = assign11130_e14340;
        locals.var_cgg_topm_dn0 = assign11130_e14340_d_n0;
        locals.var_cgg_topm_dn2 = assign11130_e14340_d_n2;
        locals.var_cgg_topm_dn3 = assign11130_e14340_d_n3;
        locals.var_cgg_topm_dn4 = assign11130_e14340_d_n4;
        locals.var_cgg_topm_dn5 = assign11130_e14340_d_n5;
        locals.var_cgg_topm_dn6 = assign11130_e14340_d_n6;
        locals.var_cgg_topm_dn7 = assign11130_e14340_d_n7;
        locals.var_cgg_topm_dn8 = assign11130_e14340_d_n8;
        locals.var_cgg_topm_dn9 = assign11130_e14340_d_n9;
        locals.var_cgg_topm_dn10 = assign11130_e14340_d_n10;
        locals.var_cgg_topm_dn11 = assign11130_e14340_d_n11;
        locals.var_cgg_topm_dn13 = assign11130_e14340_d_n13;
        locals.var_cgg_topm_dn14 = assign11130_e14340_d_n14;
        locals.var_cgg_topm_rv = 0.0;

        let (assign11140_e14355,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11140_e14349: f64 = (locals.var_hg + p.p90);
        let assign11140_e14350: f64 = (0.2 * assign11140_e14349);
        let assign11140_e14352: f64 = (assign11140_e14350 / locals.var_hrsd);
        let assign11140_e14353: f64 = (2.3 + assign11140_e14352);
        (assign11140_e14353,)
    } else {
        (locals.var_hr,)
    }
};
        locals.var_hr = assign11140_e14355;
        locals.var_hr_rv = 0.0;

        let (assign11150_e14362,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        (1.05,)
    } else {
        (locals.var_lr,)
    }
};
        locals.var_lr = assign11150_e14362;
        locals.var_lr_rv = 0.0;

        let (assign11160_e14374,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11160_e14369: f64 = (locals.var_hg + p.p90);
        let assign11160_e14371: f64 = (assign11160_e14369 - locals.var_hrsd);
        let assign11160_e14372: f64 = (assign11160_e14371).abs();
        (assign11160_e14372,)
    } else {
        (locals.var_hgdelta,)
    }
};
        locals.var_hgdelta = assign11160_e14374;
        locals.var_hgdelta_rv = 0.0;

        let (assign11170_e14383,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11170_e14381: f64 = (p.p1087 * locals.var_lr);
        (assign11170_e14381,)
    } else {
        (locals.var_lmax,)
    }
};
        locals.var_lmax = assign11170_e14383;
        locals.var_lmax_rv = 0.0;

        let (assign11180_e14394,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11180_e14391: f64 = (locals.var_hg + p.p90);
        let assign11180_e14392: f64 = (locals.var_hrsd).min(assign11180_e14391);
        (assign11180_e14392,)
    } else {
        (locals.var_y,)
    }
};
        locals.var_y = assign11180_e14394;
        locals.var_y_rv = 0.0;

        let (assign11190_e14405,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11190_e14402: f64 = (locals.var_hr + 1.0);
        let assign11190_e14403: f64 = (p.p1087 / assign11190_e14402);
        (assign11190_e14403,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign11190_e14405;
        locals.var_x_rv = 0.0;

        let (assign11200_e14412,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        (1700000000000.0,)
    } else {
        (locals.var_cnon,)
    }
};
        locals.var_cnon = assign11200_e14412;
        locals.var_cnon_rv = 0.0;

        let (assign11210_e14425,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11210_e14420: f64 = (locals.var_y - locals.var_x);
        let assign11210_e14421: f64 = (locals.var_epssp * assign11210_e14420);
        let assign11210_e14423: f64 = (assign11210_e14421 / p.p1087);
        (assign11210_e14423,)
    } else {
        (locals.var_ccgsat,)
    }
};
        locals.var_ccgsat = assign11210_e14425;
        locals.var_ccgsat_rv = 0.0;

        let (assign11220_e14434, assign11220_e14434_d_n0, assign11220_e14434_d_n2, assign11220_e14434_d_n3, assign11220_e14434_d_n4, assign11220_e14434_d_n5, assign11220_e14434_d_n6, assign11220_e14434_d_n7, assign11220_e14434_d_n8, assign11220_e14434_d_n9, assign11220_e14434_d_n10, assign11220_e14434_d_n11, assign11220_e14434_d_n13, assign11220_e14434_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11220_e14432: f64 = (locals.var_cnon * locals.var_ccgsat);
        (assign11220_e14432, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign11220_e14434;
        locals.var_tt1_dn0 = assign11220_e14434_d_n0;
        locals.var_tt1_dn2 = assign11220_e14434_d_n2;
        locals.var_tt1_dn3 = assign11220_e14434_d_n3;
        locals.var_tt1_dn4 = assign11220_e14434_d_n4;
        locals.var_tt1_dn5 = assign11220_e14434_d_n5;
        locals.var_tt1_dn6 = assign11220_e14434_d_n6;
        locals.var_tt1_dn7 = assign11220_e14434_d_n7;
        locals.var_tt1_dn8 = assign11220_e14434_d_n8;
        locals.var_tt1_dn9 = assign11220_e14434_d_n9;
        locals.var_tt1_dn10 = assign11220_e14434_d_n10;
        locals.var_tt1_dn11 = assign11220_e14434_d_n11;
        locals.var_tt1_dn13 = assign11220_e14434_d_n13;
        locals.var_tt1_dn14 = assign11220_e14434_d_n14;
        locals.var_tt1_rv = 0.0;

        let assign11230_e14437: f64 = if locals.var_tt1 > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard215 = assign11230_e14437;
        locals.var_guard215_rv = 0.0;

        let (assign11240_e14446, assign11240_e14446_d_n0, assign11240_e14446_d_n2, assign11240_e14446_d_n3, assign11240_e14446_d_n4, assign11240_e14446_d_n5, assign11240_e14446_d_n6, assign11240_e14446_d_n7, assign11240_e14446_d_n8, assign11240_e14446_d_n9, assign11240_e14446_d_n10, assign11240_e14446_d_n11, assign11240_e14446_d_n13, assign11240_e14446_d_n14,) = {
    if (((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 != 0.0)) {
        (locals.var_ccgsat, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign11240_e14446;
        locals.var_ccg1_dn0 = assign11240_e14446_d_n0;
        locals.var_ccg1_dn2 = assign11240_e14446_d_n2;
        locals.var_ccg1_dn3 = assign11240_e14446_d_n3;
        locals.var_ccg1_dn4 = assign11240_e14446_d_n4;
        locals.var_ccg1_dn5 = assign11240_e14446_d_n5;
        locals.var_ccg1_dn6 = assign11240_e14446_d_n6;
        locals.var_ccg1_dn7 = assign11240_e14446_d_n7;
        locals.var_ccg1_dn8 = assign11240_e14446_d_n8;
        locals.var_ccg1_dn9 = assign11240_e14446_d_n9;
        locals.var_ccg1_dn10 = assign11240_e14446_d_n10;
        locals.var_ccg1_dn11 = assign11240_e14446_d_n11;
        locals.var_ccg1_dn13 = assign11240_e14446_d_n13;
        locals.var_ccg1_dn14 = assign11240_e14446_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign11250_e14493, assign11250_e14493_d_n0, assign11250_e14493_d_n2, assign11250_e14493_d_n3, assign11250_e14493_d_n4, assign11250_e14493_d_n5, assign11250_e14493_d_n6, assign11250_e14493_d_n7, assign11250_e14493_d_n8, assign11250_e14493_d_n9, assign11250_e14493_d_n10, assign11250_e14493_d_n11, assign11250_e14493_d_n13, assign11250_e14493_d_n14,) = {
    if (((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 == 0.0)) {
        let assign11250_e14456: f64 = (1.0 / locals.var_cnon);
        let assign11250_e14463: f64 = (-37.0);
        let (assign11250_e14490, assign11250_e14490_d_n0, assign11250_e14490_d_n2, assign11250_e14490_d_n3, assign11250_e14490_d_n4, assign11250_e14490_d_n5, assign11250_e14490_d_n6, assign11250_e14490_d_n7, assign11250_e14490_d_n8, assign11250_e14490_d_n9, assign11250_e14490_d_n10, assign11250_e14490_d_n11, assign11250_e14490_d_n13, assign11250_e14490_d_n14,) = {
            if ((!(locals.var_tt1 > 37.0)) && (!(locals.var_tt1 < assign11250_e14463))) {
                let assign11250_e14469: f64 = (locals.var_tt1).exp();
                let assign11250_e14470: f64 = (1.0 + assign11250_e14469);
                let assign11250_e14471: f64 = (assign11250_e14470).ln();
                (assign11250_e14471, ((assign11250_e14469 * locals.var_tt1_dn0) / assign11250_e14470), ((assign11250_e14469 * locals.var_tt1_dn2) / assign11250_e14470), ((assign11250_e14469 * locals.var_tt1_dn3) / assign11250_e14470), ((assign11250_e14469 * locals.var_tt1_dn4) / assign11250_e14470), ((assign11250_e14469 * locals.var_tt1_dn5) / assign11250_e14470), ((assign11250_e14469 * locals.var_tt1_dn6) / assign11250_e14470), ((assign11250_e14469 * locals.var_tt1_dn7) / assign11250_e14470), ((assign11250_e14469 * locals.var_tt1_dn8) / assign11250_e14470), ((assign11250_e14469 * locals.var_tt1_dn9) / assign11250_e14470), ((assign11250_e14469 * locals.var_tt1_dn10) / assign11250_e14470), ((assign11250_e14469 * locals.var_tt1_dn11) / assign11250_e14470), ((assign11250_e14469 * locals.var_tt1_dn13) / assign11250_e14470), ((assign11250_e14469 * locals.var_tt1_dn14) / assign11250_e14470),)
            } else {
                let assign11250_e14478: f64 = (-37.0);
                let (assign11250_e14489, assign11250_e14489_d_n0, assign11250_e14489_d_n2, assign11250_e14489_d_n3, assign11250_e14489_d_n4, assign11250_e14489_d_n5, assign11250_e14489_d_n6, assign11250_e14489_d_n7, assign11250_e14489_d_n8, assign11250_e14489_d_n9, assign11250_e14489_d_n10, assign11250_e14489_d_n11, assign11250_e14489_d_n13, assign11250_e14489_d_n14,) = {
                    if ((!(locals.var_tt1 > 37.0)) && (locals.var_tt1 < assign11250_e14478)) {
                        let assign11250_e14482: f64 = (locals.var_tt1).exp();
                        (assign11250_e14482, (assign11250_e14482 * locals.var_tt1_dn0), (assign11250_e14482 * locals.var_tt1_dn2), (assign11250_e14482 * locals.var_tt1_dn3), (assign11250_e14482 * locals.var_tt1_dn4), (assign11250_e14482 * locals.var_tt1_dn5), (assign11250_e14482 * locals.var_tt1_dn6), (assign11250_e14482 * locals.var_tt1_dn7), (assign11250_e14482 * locals.var_tt1_dn8), (assign11250_e14482 * locals.var_tt1_dn9), (assign11250_e14482 * locals.var_tt1_dn10), (assign11250_e14482 * locals.var_tt1_dn11), (assign11250_e14482 * locals.var_tt1_dn13), (assign11250_e14482 * locals.var_tt1_dn14),)
                    } else {
                        let (assign11250_e14488, assign11250_e14488_d_n0, assign11250_e14488_d_n2, assign11250_e14488_d_n3, assign11250_e14488_d_n4, assign11250_e14488_d_n5, assign11250_e14488_d_n6, assign11250_e14488_d_n7, assign11250_e14488_d_n8, assign11250_e14488_d_n9, assign11250_e14488_d_n10, assign11250_e14488_d_n11, assign11250_e14488_d_n13, assign11250_e14488_d_n14,) = {
                            if (locals.var_tt1 > 37.0) {
                                (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign11250_e14488, assign11250_e14488_d_n0, assign11250_e14488_d_n2, assign11250_e14488_d_n3, assign11250_e14488_d_n4, assign11250_e14488_d_n5, assign11250_e14488_d_n6, assign11250_e14488_d_n7, assign11250_e14488_d_n8, assign11250_e14488_d_n9, assign11250_e14488_d_n10, assign11250_e14488_d_n11, assign11250_e14488_d_n13, assign11250_e14488_d_n14,)
                    }
                };
                (assign11250_e14489, assign11250_e14489_d_n0, assign11250_e14489_d_n2, assign11250_e14489_d_n3, assign11250_e14489_d_n4, assign11250_e14489_d_n5, assign11250_e14489_d_n6, assign11250_e14489_d_n7, assign11250_e14489_d_n8, assign11250_e14489_d_n9, assign11250_e14489_d_n10, assign11250_e14489_d_n11, assign11250_e14489_d_n13, assign11250_e14489_d_n14,)
            }
        };
        let assign11250_e14491: f64 = (assign11250_e14456 * assign11250_e14490);
        (assign11250_e14491, (assign11250_e14456 * assign11250_e14490_d_n0), (assign11250_e14456 * assign11250_e14490_d_n2), (assign11250_e14456 * assign11250_e14490_d_n3), (assign11250_e14456 * assign11250_e14490_d_n4), (assign11250_e14456 * assign11250_e14490_d_n5), (assign11250_e14456 * assign11250_e14490_d_n6), (assign11250_e14456 * assign11250_e14490_d_n7), (assign11250_e14456 * assign11250_e14490_d_n8), (assign11250_e14456 * assign11250_e14490_d_n9), (assign11250_e14456 * assign11250_e14490_d_n10), (assign11250_e14456 * assign11250_e14490_d_n11), (assign11250_e14456 * assign11250_e14490_d_n13), (assign11250_e14456 * assign11250_e14490_d_n14),)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign11250_e14493;
        locals.var_ccg1_dn0 = assign11250_e14493_d_n0;
        locals.var_ccg1_dn2 = assign11250_e14493_d_n2;
        locals.var_ccg1_dn3 = assign11250_e14493_d_n3;
        locals.var_ccg1_dn4 = assign11250_e14493_d_n4;
        locals.var_ccg1_dn5 = assign11250_e14493_d_n5;
        locals.var_ccg1_dn6 = assign11250_e14493_d_n6;
        locals.var_ccg1_dn7 = assign11250_e14493_d_n7;
        locals.var_ccg1_dn8 = assign11250_e14493_d_n8;
        locals.var_ccg1_dn9 = assign11250_e14493_d_n9;
        locals.var_ccg1_dn10 = assign11250_e14493_d_n10;
        locals.var_ccg1_dn11 = assign11250_e14493_d_n11;
        locals.var_ccg1_dn13 = assign11250_e14493_d_n13;
        locals.var_ccg1_dn14 = assign11250_e14493_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign11260_e14512,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11260_e14502: f64 = (locals.var_hg + p.p90);
        let assign11260_e14503: f64 = (locals.var_hrsd / assign11260_e14502);
        let assign11260_e14506: f64 = (locals.var_hg + p.p90);
        let assign11260_e14508: f64 = (assign11260_e14506 / locals.var_hrsd);
        let assign11260_e14509: f64 = (assign11260_e14503).min(assign11260_e14508);
        let assign11260_e14510: f64 = (0.5 * assign11260_e14509);
        (assign11260_e14510,)
    } else {
        (locals.var_r1cf,)
    }
};
        locals.var_r1cf = assign11260_e14512;
        locals.var_r1cf_rv = 0.0;

        let (assign11270_e14521,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11270_e14519: f64 = (locals.var_hgdelta * locals.var_r1cf);
        (assign11270_e14519,)
    } else {
        (locals.var_rcf,)
    }
};
        locals.var_rcf = assign11270_e14521;
        locals.var_rcf_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11280_e14571,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11280_e14528: f64 = (locals.var_epssp * 2.0);
        let assign11280_e14530: f64 = (assign11280_e14528 / 3.141592653589793);
        let assign11280_e14534: f64 = (0.5 * 3.141592653589793);
        let assign11280_e14536: f64 = (assign11280_e14534 * locals.var_rcf);
        let assign11280_e14537: f64 = (p.p1087 + assign11280_e14536);
        let assign11280_e14539: f64 = (assign11280_e14537 / p.p1087);
        let (assign11280_e14568,) = {
            if (!(assign11280_e14539 > 1e-38)) {
                let assign11280_e14544: f64 = (-87.498233534);
                (assign11280_e14544,)
            } else {
                let assign11280_e14548: f64 = (0.5 * 3.141592653589793);
                let assign11280_e14550: f64 = (assign11280_e14548 * locals.var_rcf);
                let assign11280_e14551: f64 = (p.p1087 + assign11280_e14550);
                let assign11280_e14553: f64 = (assign11280_e14551 / p.p1087);
                let (assign11280_e14567,) = {
                    if (assign11280_e14553 > 1e-38) {
                        let assign11280_e14559: f64 = (0.5 * 3.141592653589793);
                        let assign11280_e14561: f64 = (assign11280_e14559 * locals.var_rcf);
                        let assign11280_e14562: f64 = (p.p1087 + assign11280_e14561);
                        let assign11280_e14564: f64 = (assign11280_e14562 / p.p1087);
                        let assign11280_e14565: f64 = (assign11280_e14564).ln();
                        (assign11280_e14565,)
                    } else {
                        (0.0,)
                    }
                };
                (assign11280_e14567,)
            }
        };
        let assign11280_e14569: f64 = (assign11280_e14530 * assign11280_e14568);
        (assign11280_e14569,)
    } else {
        (locals.var_ccg2,)
    }
};
        locals.var_ccg2 = assign11280_e14571;
        locals.var_ccg2_rv = 0.0;

        let (assign11290_e14582, assign11290_e14582_d_n0, assign11290_e14582_d_n2, assign11290_e14582_d_n3, assign11290_e14582_d_n4, assign11290_e14582_d_n5, assign11290_e14582_d_n6, assign11290_e14582_d_n7, assign11290_e14582_d_n8, assign11290_e14582_d_n9, assign11290_e14582_d_n10, assign11290_e14582_d_n11, assign11290_e14582_d_n13, assign11290_e14582_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11290_e14579: f64 = (locals.var_ccg1 + locals.var_ccg2);
        let assign11290_e14580: f64 = (p.p43 * assign11290_e14579);
        (assign11290_e14580, (p.p43 * locals.var_ccg1_dn0), (p.p43 * locals.var_ccg1_dn2), (p.p43 * locals.var_ccg1_dn3), (p.p43 * locals.var_ccg1_dn4), (p.p43 * locals.var_ccg1_dn5), (p.p43 * locals.var_ccg1_dn6), (p.p43 * locals.var_ccg1_dn7), (p.p43 * locals.var_ccg1_dn8), (p.p43 * locals.var_ccg1_dn9), (p.p43 * locals.var_ccg1_dn10), (p.p43 * locals.var_ccg1_dn11), (p.p43 * locals.var_ccg1_dn13), (p.p43 * locals.var_ccg1_dn14),)
    } else {
        (locals.var_ccg, locals.var_ccg_dn0, locals.var_ccg_dn2, locals.var_ccg_dn3, locals.var_ccg_dn4, locals.var_ccg_dn5, locals.var_ccg_dn6, locals.var_ccg_dn7, locals.var_ccg_dn8, locals.var_ccg_dn9, locals.var_ccg_dn10, locals.var_ccg_dn11, locals.var_ccg_dn13, locals.var_ccg_dn14,)
    }
};
        locals.var_ccg = assign11290_e14582;
        locals.var_ccg_dn0 = assign11290_e14582_d_n0;
        locals.var_ccg_dn2 = assign11290_e14582_d_n2;
        locals.var_ccg_dn3 = assign11290_e14582_d_n3;
        locals.var_ccg_dn4 = assign11290_e14582_d_n4;
        locals.var_ccg_dn5 = assign11290_e14582_d_n5;
        locals.var_ccg_dn6 = assign11290_e14582_d_n6;
        locals.var_ccg_dn7 = assign11290_e14582_d_n7;
        locals.var_ccg_dn8 = assign11290_e14582_d_n8;
        locals.var_ccg_dn9 = assign11290_e14582_d_n9;
        locals.var_ccg_dn10 = assign11290_e14582_d_n10;
        locals.var_ccg_dn11 = assign11290_e14582_d_n11;
        locals.var_ccg_dn13 = assign11290_e14582_d_n13;
        locals.var_ccg_dn14 = assign11290_e14582_d_n14;
        locals.var_ccg_rv = 0.0;

        let (assign11300_e14591,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11300_e14589: f64 = (locals.var_lmax / locals.var_hg);
        (assign11300_e14589,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign11300_e14591;
        locals.var_x_rv = 0.0;

        let (assign11310_e14607,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11310_e14600: f64 = (locals.var_x + 1.0);
        let assign11310_e14601: f64 = (2.0 * assign11310_e14600);
        let assign11310_e14602: f64 = (assign11310_e14601).sqrt();
        let assign11310_e14604: f64 = (assign11310_e14602 * 3.141592653589793);
        let assign11310_e14605: f64 = (4.0 / assign11310_e14604);
        (assign11310_e14605,)
    } else {
        (locals.var_c1,)
    }
};
        locals.var_c1 = assign11310_e14607;
        locals.var_c1_rv = 0.0;

        let (assign11320_e14644,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11320_e14614: f64 = (p.p90 * p.p90);
        let assign11320_e14617: f64 = (2.0 * locals.var_hg);
        let assign11320_e14619: f64 = (assign11320_e14617 * p.p90);
        let assign11320_e14620: f64 = (assign11320_e14614 + assign11320_e14619);
        let assign11320_e14623: f64 = (locals.var_hg * locals.var_hg);
        let assign11320_e14626: f64 = (locals.var_x + 1.0);
        let assign11320_e14627: f64 = (assign11320_e14623 * assign11320_e14626);
        let assign11320_e14628: f64 = (assign11320_e14620 + assign11320_e14627);
        let assign11320_e14629: f64 = (assign11320_e14628).sqrt();
        let assign11320_e14632: f64 = (locals.var_x + 1.0);
        let assign11320_e14633: f64 = (assign11320_e14632).sqrt();
        let assign11320_e14634: f64 = (assign11320_e14629 * assign11320_e14633);
        let assign11320_e14636: f64 = (assign11320_e14634 + p.p90);
        let assign11320_e14639: f64 = (locals.var_hg * locals.var_x);
        let assign11320_e14640: f64 = (assign11320_e14636 + assign11320_e14639);
        let assign11320_e14642: f64 = (assign11320_e14640 + locals.var_hg);
        (assign11320_e14642,)
    } else {
        (locals.var_c2,)
    }
};
        locals.var_c2 = assign11320_e14644;
        locals.var_c2_rv = 0.0;

        let (assign11330_e14666,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11330_e14652: f64 = (locals.var_x + 1.0);
        let assign11330_e14655: f64 = (locals.var_x + 4.0);
        let assign11330_e14656: f64 = (assign11330_e14652 * assign11330_e14655);
        let assign11330_e14657: f64 = (assign11330_e14656).sqrt();
        let assign11330_e14658: f64 = (p.p90 * assign11330_e14657);
        let assign11330_e14662: f64 = (locals.var_x + 2.0);
        let assign11330_e14663: f64 = (p.p90 * assign11330_e14662);
        let assign11330_e14664: f64 = (assign11330_e14658 + assign11330_e14663);
        (assign11330_e14664,)
    } else {
        (locals.var_c3,)
    }
};
        locals.var_c3 = assign11330_e14666;
        locals.var_c3_rv = 0.0;

        let (assign11340_e14698,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11340_e14675: f64 = (locals.var_c2 / locals.var_c3);
        let (assign11340_e14692,) = {
            if (!(assign11340_e14675 > 1e-38)) {
                let assign11340_e14680: f64 = (-87.498233534);
                (assign11340_e14680,)
            } else {
                let assign11340_e14683: f64 = (locals.var_c2 / locals.var_c3);
                let (assign11340_e14691,) = {
                    if (assign11340_e14683 > 1e-38) {
                        let assign11340_e14688: f64 = (locals.var_c2 / locals.var_c3);
                        let assign11340_e14689: f64 = (assign11340_e14688).ln();
                        (assign11340_e14689,)
                    } else {
                        (0.0,)
                    }
                };
                (assign11340_e14691,)
            }
        };
        let assign11340_e14693: f64 = (locals.var_c1 * assign11340_e14692);
        let assign11340_e14695: f64 = (assign11340_e14693 + 12.27);
        let assign11340_e14696: f64 = (locals.var_epssp * assign11340_e14695);
        (assign11340_e14696,)
    } else {
        (locals.var_cfglog,)
    }
};
        locals.var_cfglog = assign11340_e14698;
        locals.var_cfglog_rv = 0.0;

        let (assign11350_e14707,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11350_e14705: f64 = (locals.var_hr * locals.var_lr);
        (assign11350_e14705,)
    } else {
        (locals.var_dcf,)
    }
};
        locals.var_dcf = assign11350_e14707;
        locals.var_dcf_rv = 0.0;

        let (assign11360_e14719,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11360_e14714: f64 = (locals.var_dcf * locals.var_dcf);
        let assign11360_e14716: f64 = (assign11360_e14714 + 1.0);
        let assign11360_e14717: f64 = (assign11360_e14716).sqrt();
        (assign11360_e14717,)
    } else {
        (locals.var_tt0,)
    }
};
        locals.var_tt0 = assign11360_e14719;
        locals.var_tt0_rv = 0.0;

        let (assign11370_e14769, assign11370_e14769_d_n0, assign11370_e14769_d_n2, assign11370_e14769_d_n3, assign11370_e14769_d_n4, assign11370_e14769_d_n5, assign11370_e14769_d_n6, assign11370_e14769_d_n7, assign11370_e14769_d_n8, assign11370_e14769_d_n9, assign11370_e14769_d_n10, assign11370_e14769_d_n11, assign11370_e14769_d_n13, assign11370_e14769_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11370_e14726: f64 = (locals.var_dcf * locals.var_dcf);
        let assign11370_e14728: f64 = (assign11370_e14726 + 1.0);
        let assign11370_e14731: f64 = (locals.var_dcf * p.p90);
        let assign11370_e14734: f64 = (locals.var_dcf * p.p90);
        let assign11370_e14735: f64 = (assign11370_e14731 * assign11370_e14734);
        let assign11370_e14738: f64 = (2.0 * locals.var_dcf);
        let assign11370_e14740: f64 = (assign11370_e14738 * locals.var_lmax);
        let assign11370_e14742: f64 = (assign11370_e14740 * p.p90);
        let assign11370_e14743: f64 = (assign11370_e14735 + assign11370_e14742);
        let assign11370_e14746: f64 = (locals.var_dcf * locals.var_dcf);
        let assign11370_e14748: f64 = (assign11370_e14746 + 1.0);
        let assign11370_e14750: f64 = (assign11370_e14748 * locals.var_lmax);
        let assign11370_e14752: f64 = (assign11370_e14750 * locals.var_lmax);
        let assign11370_e14753: f64 = (assign11370_e14743 + assign11370_e14752);
        let assign11370_e14754: f64 = (assign11370_e14728 * assign11370_e14753);
        let assign11370_e14755: f64 = (assign11370_e14754).sqrt();
        let assign11370_e14758: f64 = (locals.var_dcf * p.p90);
        let assign11370_e14759: f64 = (assign11370_e14755 + assign11370_e14758);
        let assign11370_e14762: f64 = (locals.var_dcf * locals.var_dcf);
        let assign11370_e14764: f64 = (assign11370_e14762 * locals.var_lmax);
        let assign11370_e14765: f64 = (assign11370_e14759 + assign11370_e14764);
        let assign11370_e14767: f64 = (assign11370_e14765 + locals.var_lmax);
        (assign11370_e14767, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign11370_e14769;
        locals.var_tt1_dn0 = assign11370_e14769_d_n0;
        locals.var_tt1_dn2 = assign11370_e14769_d_n2;
        locals.var_tt1_dn3 = assign11370_e14769_d_n3;
        locals.var_tt1_dn4 = assign11370_e14769_d_n4;
        locals.var_tt1_dn5 = assign11370_e14769_d_n5;
        locals.var_tt1_dn6 = assign11370_e14769_d_n6;
        locals.var_tt1_dn7 = assign11370_e14769_d_n7;
        locals.var_tt1_dn8 = assign11370_e14769_d_n8;
        locals.var_tt1_dn9 = assign11370_e14769_d_n9;
        locals.var_tt1_dn10 = assign11370_e14769_d_n10;
        locals.var_tt1_dn11 = assign11370_e14769_d_n11;
        locals.var_tt1_dn13 = assign11370_e14769_d_n13;
        locals.var_tt1_dn14 = assign11370_e14769_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign11380_e14782,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11380_e14776: f64 = (locals.var_tt0 + 1.0);
        let assign11380_e14779: f64 = (locals.var_dcf * p.p90);
        let assign11380_e14780: f64 = (assign11380_e14776 * assign11380_e14779);
        (assign11380_e14780,)
    } else {
        (locals.var_tt2,)
    }
};
        locals.var_tt2 = assign11380_e14782;
        locals.var_tt2_rv = 0.0;

        let (assign11390_e14823, assign11390_e14823_d_n0, assign11390_e14823_d_n2, assign11390_e14823_d_n3, assign11390_e14823_d_n4, assign11390_e14823_d_n5, assign11390_e14823_d_n6, assign11390_e14823_d_n7, assign11390_e14823_d_n8, assign11390_e14823_d_n9, assign11390_e14823_d_n10, assign11390_e14823_d_n11, assign11390_e14823_d_n13, assign11390_e14823_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11390_e14789: f64 = (2.0 * locals.var_epssp);
        let assign11390_e14791: f64 = (2.0_f64).sqrt();
        let assign11390_e14792: f64 = (assign11390_e14789 * assign11390_e14791);
        let assign11390_e14794: f64 = (assign11390_e14792 / 3.141592653589793);
        let assign11390_e14796: f64 = (assign11390_e14794 * 0.85);
        let assign11390_e14798: f64 = (assign11390_e14796 * locals.var_dcf);
        let assign11390_e14800: f64 = (assign11390_e14798 / locals.var_tt0);
        let assign11390_e14803: f64 = (locals.var_tt1 / locals.var_tt2);
        let (assign11390_e14820, assign11390_e14820_d_n0, assign11390_e14820_d_n2, assign11390_e14820_d_n3, assign11390_e14820_d_n4, assign11390_e14820_d_n5, assign11390_e14820_d_n6, assign11390_e14820_d_n7, assign11390_e14820_d_n8, assign11390_e14820_d_n9, assign11390_e14820_d_n10, assign11390_e14820_d_n11, assign11390_e14820_d_n13, assign11390_e14820_d_n14,) = {
            if (!(assign11390_e14803 > 1e-38)) {
                let assign11390_e14808: f64 = (-87.498233534);
                (assign11390_e14808, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign11390_e14811: f64 = (locals.var_tt1 / locals.var_tt2);
                let (assign11390_e14819, assign11390_e14819_d_n0, assign11390_e14819_d_n2, assign11390_e14819_d_n3, assign11390_e14819_d_n4, assign11390_e14819_d_n5, assign11390_e14819_d_n6, assign11390_e14819_d_n7, assign11390_e14819_d_n8, assign11390_e14819_d_n9, assign11390_e14819_d_n10, assign11390_e14819_d_n11, assign11390_e14819_d_n13, assign11390_e14819_d_n14,) = {
                    if (assign11390_e14811 > 1e-38) {
                        let assign11390_e14816: f64 = (locals.var_tt1 / locals.var_tt2);
                        let assign11390_e14817: f64 = (assign11390_e14816).ln();
                        (assign11390_e14817, ((locals.var_tt1_dn0 / locals.var_tt2) / assign11390_e14816), ((locals.var_tt1_dn2 / locals.var_tt2) / assign11390_e14816), ((locals.var_tt1_dn3 / locals.var_tt2) / assign11390_e14816), ((locals.var_tt1_dn4 / locals.var_tt2) / assign11390_e14816), ((locals.var_tt1_dn5 / locals.var_tt2) / assign11390_e14816), ((locals.var_tt1_dn6 / locals.var_tt2) / assign11390_e14816), ((locals.var_tt1_dn7 / locals.var_tt2) / assign11390_e14816), ((locals.var_tt1_dn8 / locals.var_tt2) / assign11390_e14816), ((locals.var_tt1_dn9 / locals.var_tt2) / assign11390_e14816), ((locals.var_tt1_dn10 / locals.var_tt2) / assign11390_e14816), ((locals.var_tt1_dn11 / locals.var_tt2) / assign11390_e14816), ((locals.var_tt1_dn13 / locals.var_tt2) / assign11390_e14816), ((locals.var_tt1_dn14 / locals.var_tt2) / assign11390_e14816),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign11390_e14819, assign11390_e14819_d_n0, assign11390_e14819_d_n2, assign11390_e14819_d_n3, assign11390_e14819_d_n4, assign11390_e14819_d_n5, assign11390_e14819_d_n6, assign11390_e14819_d_n7, assign11390_e14819_d_n8, assign11390_e14819_d_n9, assign11390_e14819_d_n10, assign11390_e14819_d_n11, assign11390_e14819_d_n13, assign11390_e14819_d_n14,)
            }
        };
        let assign11390_e14821: f64 = (assign11390_e14800 * assign11390_e14820);
        (assign11390_e14821, (assign11390_e14800 * assign11390_e14820_d_n0), (assign11390_e14800 * assign11390_e14820_d_n2), (assign11390_e14800 * assign11390_e14820_d_n3), (assign11390_e14800 * assign11390_e14820_d_n4), (assign11390_e14800 * assign11390_e14820_d_n5), (assign11390_e14800 * assign11390_e14820_d_n6), (assign11390_e14800 * assign11390_e14820_d_n7), (assign11390_e14800 * assign11390_e14820_d_n8), (assign11390_e14800 * assign11390_e14820_d_n9), (assign11390_e14800 * assign11390_e14820_d_n10), (assign11390_e14800 * assign11390_e14820_d_n11), (assign11390_e14800 * assign11390_e14820_d_n13), (assign11390_e14800 * assign11390_e14820_d_n14),)
    } else {
        (locals.var_cfgsat, locals.var_cfgsat_dn0, locals.var_cfgsat_dn2, locals.var_cfgsat_dn3, locals.var_cfgsat_dn4, locals.var_cfgsat_dn5, locals.var_cfgsat_dn6, locals.var_cfgsat_dn7, locals.var_cfgsat_dn8, locals.var_cfgsat_dn9, locals.var_cfgsat_dn10, locals.var_cfgsat_dn11, locals.var_cfgsat_dn13, locals.var_cfgsat_dn14,)
    }
};
        locals.var_cfgsat = assign11390_e14823;
        locals.var_cfgsat_dn0 = assign11390_e14823_d_n0;
        locals.var_cfgsat_dn2 = assign11390_e14823_d_n2;
        locals.var_cfgsat_dn3 = assign11390_e14823_d_n3;
        locals.var_cfgsat_dn4 = assign11390_e14823_d_n4;
        locals.var_cfgsat_dn5 = assign11390_e14823_d_n5;
        locals.var_cfgsat_dn6 = assign11390_e14823_d_n6;
        locals.var_cfgsat_dn7 = assign11390_e14823_d_n7;
        locals.var_cfgsat_dn8 = assign11390_e14823_d_n8;
        locals.var_cfgsat_dn9 = assign11390_e14823_d_n9;
        locals.var_cfgsat_dn10 = assign11390_e14823_d_n10;
        locals.var_cfgsat_dn11 = assign11390_e14823_d_n11;
        locals.var_cfgsat_dn13 = assign11390_e14823_d_n13;
        locals.var_cfgsat_dn14 = assign11390_e14823_d_n14;
        locals.var_cfgsat_rv = 0.0;

        let (assign11400_e14830, assign11400_e14830_d_n0, assign11400_e14830_d_n2, assign11400_e14830_d_n3, assign11400_e14830_d_n4, assign11400_e14830_d_n5, assign11400_e14830_d_n6, assign11400_e14830_d_n7, assign11400_e14830_d_n8, assign11400_e14830_d_n9, assign11400_e14830_d_n10, assign11400_e14830_d_n11, assign11400_e14830_d_n13, assign11400_e14830_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        (1.2e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign11400_e14830;
        locals.var_delta_dn0 = assign11400_e14830_d_n0;
        locals.var_delta_dn2 = assign11400_e14830_d_n2;
        locals.var_delta_dn3 = assign11400_e14830_d_n3;
        locals.var_delta_dn4 = assign11400_e14830_d_n4;
        locals.var_delta_dn5 = assign11400_e14830_d_n5;
        locals.var_delta_dn6 = assign11400_e14830_d_n6;
        locals.var_delta_dn7 = assign11400_e14830_d_n7;
        locals.var_delta_dn8 = assign11400_e14830_d_n8;
        locals.var_delta_dn9 = assign11400_e14830_d_n9;
        locals.var_delta_dn10 = assign11400_e14830_d_n10;
        locals.var_delta_dn11 = assign11400_e14830_d_n11;
        locals.var_delta_dn13 = assign11400_e14830_d_n13;
        locals.var_delta_dn14 = assign11400_e14830_d_n14;
        locals.var_delta_rv = 0.0;

        let (assign11410_e14841, assign11410_e14841_d_n0, assign11410_e14841_d_n2, assign11410_e14841_d_n3, assign11410_e14841_d_n4, assign11410_e14841_d_n5, assign11410_e14841_d_n6, assign11410_e14841_d_n7, assign11410_e14841_d_n8, assign11410_e14841_d_n9, assign11410_e14841_d_n10, assign11410_e14841_d_n11, assign11410_e14841_d_n13, assign11410_e14841_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11410_e14837: f64 = (locals.var_cfgsat - locals.var_cfglog);
        let assign11410_e14839: f64 = (assign11410_e14837 - locals.var_delta);
        (assign11410_e14839, (locals.var_cfgsat_dn0 - locals.var_delta_dn0), (locals.var_cfgsat_dn2 - locals.var_delta_dn2), (locals.var_cfgsat_dn3 - locals.var_delta_dn3), (locals.var_cfgsat_dn4 - locals.var_delta_dn4), (locals.var_cfgsat_dn5 - locals.var_delta_dn5), (locals.var_cfgsat_dn6 - locals.var_delta_dn6), (locals.var_cfgsat_dn7 - locals.var_delta_dn7), (locals.var_cfgsat_dn8 - locals.var_delta_dn8), (locals.var_cfgsat_dn9 - locals.var_delta_dn9), (locals.var_cfgsat_dn10 - locals.var_delta_dn10), (locals.var_cfgsat_dn11 - locals.var_delta_dn11), (locals.var_cfgsat_dn13 - locals.var_delta_dn13), (locals.var_cfgsat_dn14 - locals.var_delta_dn14),)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign11410_e14841;
        locals.var_tt1_dn0 = assign11410_e14841_d_n0;
        locals.var_tt1_dn2 = assign11410_e14841_d_n2;
        locals.var_tt1_dn3 = assign11410_e14841_d_n3;
        locals.var_tt1_dn4 = assign11410_e14841_d_n4;
        locals.var_tt1_dn5 = assign11410_e14841_d_n5;
        locals.var_tt1_dn6 = assign11410_e14841_d_n6;
        locals.var_tt1_dn7 = assign11410_e14841_d_n7;
        locals.var_tt1_dn8 = assign11410_e14841_d_n8;
        locals.var_tt1_dn9 = assign11410_e14841_d_n9;
        locals.var_tt1_dn10 = assign11410_e14841_d_n10;
        locals.var_tt1_dn11 = assign11410_e14841_d_n11;
        locals.var_tt1_dn13 = assign11410_e14841_d_n13;
        locals.var_tt1_dn14 = assign11410_e14841_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign11420_e14865, assign11420_e14865_d_n0, assign11420_e14865_d_n2, assign11420_e14865_d_n3, assign11420_e14865_d_n4, assign11420_e14865_d_n5, assign11420_e14865_d_n6, assign11420_e14865_d_n7, assign11420_e14865_d_n8, assign11420_e14865_d_n9, assign11420_e14865_d_n10, assign11420_e14865_d_n11, assign11420_e14865_d_n13, assign11420_e14865_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11420_e14852: f64 = (locals.var_tt1 * locals.var_tt1);
        let assign11420_e14855: f64 = (4.0 * locals.var_delta);
        let assign11420_e14857: f64 = (assign11420_e14855 * locals.var_cfgsat);
        let assign11420_e14858: f64 = (assign11420_e14852 + assign11420_e14857);
        let assign11420_e14859: f64 = (assign11420_e14858).sqrt();
        let assign11420_e14860: f64 = (locals.var_tt1 + assign11420_e14859);
        let assign11420_e14861: f64 = (0.5 * assign11420_e14860);
        let assign11420_e14862: f64 = (locals.var_cfgsat - assign11420_e14861);
        let assign11420_e14863: f64 = (p.p43 * assign11420_e14862);
        (assign11420_e14863, (p.p43 * (locals.var_cfgsat_dn0 - (0.5 * (locals.var_tt1_dn0 + ((((locals.var_tt1_dn0 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn0))) / (2.0 * assign11420_e14859)))))), (p.p43 * (locals.var_cfgsat_dn2 - (0.5 * (locals.var_tt1_dn2 + ((((locals.var_tt1_dn2 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn2))) / (2.0 * assign11420_e14859)))))), (p.p43 * (locals.var_cfgsat_dn3 - (0.5 * (locals.var_tt1_dn3 + ((((locals.var_tt1_dn3 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn3)) + (((4.0 * locals.var_delta_dn3) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn3))) / (2.0 * assign11420_e14859)))))), (p.p43 * (locals.var_cfgsat_dn4 - (0.5 * (locals.var_tt1_dn4 + ((((locals.var_tt1_dn4 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn4)) + (((4.0 * locals.var_delta_dn4) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn4))) / (2.0 * assign11420_e14859)))))), (p.p43 * (locals.var_cfgsat_dn5 - (0.5 * (locals.var_tt1_dn5 + ((((locals.var_tt1_dn5 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn5)) + (((4.0 * locals.var_delta_dn5) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn5))) / (2.0 * assign11420_e14859)))))), (p.p43 * (locals.var_cfgsat_dn6 - (0.5 * (locals.var_tt1_dn6 + ((((locals.var_tt1_dn6 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn6))) / (2.0 * assign11420_e14859)))))), (p.p43 * (locals.var_cfgsat_dn7 - (0.5 * (locals.var_tt1_dn7 + ((((locals.var_tt1_dn7 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn7))) / (2.0 * assign11420_e14859)))))), (p.p43 * (locals.var_cfgsat_dn8 - (0.5 * (locals.var_tt1_dn8 + ((((locals.var_tt1_dn8 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn8)) + (((4.0 * locals.var_delta_dn8) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn8))) / (2.0 * assign11420_e14859)))))), (p.p43 * (locals.var_cfgsat_dn9 - (0.5 * (locals.var_tt1_dn9 + ((((locals.var_tt1_dn9 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn9)) + (((4.0 * locals.var_delta_dn9) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn9))) / (2.0 * assign11420_e14859)))))), (p.p43 * (locals.var_cfgsat_dn10 - (0.5 * (locals.var_tt1_dn10 + ((((locals.var_tt1_dn10 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn10))) / (2.0 * assign11420_e14859)))))), (p.p43 * (locals.var_cfgsat_dn11 - (0.5 * (locals.var_tt1_dn11 + ((((locals.var_tt1_dn11 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn11))) / (2.0 * assign11420_e14859)))))), (p.p43 * (locals.var_cfgsat_dn13 - (0.5 * (locals.var_tt1_dn13 + ((((locals.var_tt1_dn13 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn13)) + (((4.0 * locals.var_delta_dn13) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn13))) / (2.0 * assign11420_e14859)))))), (p.p43 * (locals.var_cfgsat_dn14 - (0.5 * (locals.var_tt1_dn14 + ((((locals.var_tt1_dn14 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn14)) + (((4.0 * locals.var_delta_dn14) * locals.var_cfgsat) + (assign11420_e14855 * locals.var_cfgsat_dn14))) / (2.0 * assign11420_e14859)))))),)
    } else {
        (locals.var_cfg, locals.var_cfg_dn0, locals.var_cfg_dn2, locals.var_cfg_dn3, locals.var_cfg_dn4, locals.var_cfg_dn5, locals.var_cfg_dn6, locals.var_cfg_dn7, locals.var_cfg_dn8, locals.var_cfg_dn9, locals.var_cfg_dn10, locals.var_cfg_dn11, locals.var_cfg_dn13, locals.var_cfg_dn14,)
    }
};
        locals.var_cfg = assign11420_e14865;
        locals.var_cfg_dn0 = assign11420_e14865_d_n0;
        locals.var_cfg_dn2 = assign11420_e14865_d_n2;
        locals.var_cfg_dn3 = assign11420_e14865_d_n3;
        locals.var_cfg_dn4 = assign11420_e14865_d_n4;
        locals.var_cfg_dn5 = assign11420_e14865_d_n5;
        locals.var_cfg_dn6 = assign11420_e14865_d_n6;
        locals.var_cfg_dn7 = assign11420_e14865_d_n7;
        locals.var_cfg_dn8 = assign11420_e14865_d_n8;
        locals.var_cfg_dn9 = assign11420_e14865_d_n9;
        locals.var_cfg_dn10 = assign11420_e14865_d_n10;
        locals.var_cfg_dn11 = assign11420_e14865_d_n11;
        locals.var_cfg_dn13 = assign11420_e14865_d_n13;
        locals.var_cfg_dn14 = assign11420_e14865_d_n14;
        locals.var_cfg_rv = 0.0;

        let (assign11430_e14874, assign11430_e14874_d_n0, assign11430_e14874_d_n2, assign11430_e14874_d_n3, assign11430_e14874_d_n4, assign11430_e14874_d_n5, assign11430_e14874_d_n6, assign11430_e14874_d_n7, assign11430_e14874_d_n8, assign11430_e14874_d_n9, assign11430_e14874_d_n10, assign11430_e14874_d_n11, assign11430_e14874_d_n13, assign11430_e14874_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard214 == 0.0)) {
        let assign11430_e14872: f64 = (locals.var_ccg + locals.var_cfg);
        (assign11430_e14872, (locals.var_ccg_dn0 + locals.var_cfg_dn0), (locals.var_ccg_dn2 + locals.var_cfg_dn2), (locals.var_ccg_dn3 + locals.var_cfg_dn3), (locals.var_ccg_dn4 + locals.var_cfg_dn4), (locals.var_ccg_dn5 + locals.var_cfg_dn5), (locals.var_ccg_dn6 + locals.var_cfg_dn6), (locals.var_ccg_dn7 + locals.var_cfg_dn7), (locals.var_ccg_dn8 + locals.var_cfg_dn8), (locals.var_ccg_dn9 + locals.var_cfg_dn9), (locals.var_ccg_dn10 + locals.var_cfg_dn10), (locals.var_ccg_dn11 + locals.var_cfg_dn11), (locals.var_ccg_dn13 + locals.var_cfg_dn13), (locals.var_ccg_dn14 + locals.var_cfg_dn14),)
    } else {
        (locals.var_cgg_topm, locals.var_cgg_topm_dn0, locals.var_cgg_topm_dn2, locals.var_cgg_topm_dn3, locals.var_cgg_topm_dn4, locals.var_cgg_topm_dn5, locals.var_cgg_topm_dn6, locals.var_cgg_topm_dn7, locals.var_cgg_topm_dn8, locals.var_cgg_topm_dn9, locals.var_cgg_topm_dn10, locals.var_cgg_topm_dn11, locals.var_cgg_topm_dn13, locals.var_cgg_topm_dn14,)
    }
};
        locals.var_cgg_topm = assign11430_e14874;
        locals.var_cgg_topm_dn0 = assign11430_e14874_d_n0;
        locals.var_cgg_topm_dn2 = assign11430_e14874_d_n2;
        locals.var_cgg_topm_dn3 = assign11430_e14874_d_n3;
        locals.var_cgg_topm_dn4 = assign11430_e14874_d_n4;
        locals.var_cgg_topm_dn5 = assign11430_e14874_d_n5;
        locals.var_cgg_topm_dn6 = assign11430_e14874_d_n6;
        locals.var_cgg_topm_dn7 = assign11430_e14874_d_n7;
        locals.var_cgg_topm_dn8 = assign11430_e14874_d_n8;
        locals.var_cgg_topm_dn9 = assign11430_e14874_d_n9;
        locals.var_cgg_topm_dn10 = assign11430_e14874_d_n10;
        locals.var_cgg_topm_dn11 = assign11430_e14874_d_n11;
        locals.var_cgg_topm_dn13 = assign11430_e14874_d_n13;
        locals.var_cgg_topm_dn14 = assign11430_e14874_d_n14;
        locals.var_cgg_topm_rv = 0.0;

        let (assign11440_e14886,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11440_e14880: f64 = (p.p1089 + p.p90);
        let assign11440_e14881: f64 = (0.2 * assign11440_e14880);
        let assign11440_e14883: f64 = (assign11440_e14881 / locals.var_hrsd2);
        let assign11440_e14884: f64 = (2.3 + assign11440_e14883);
        (assign11440_e14884,)
    } else {
        (locals.var_hr,)
    }
};
        locals.var_hr = assign11440_e14886;
        locals.var_hr_rv = 0.0;

        let (assign11450_e14890,) = {
    if (locals.var_guard213 != 0.0) {
        (1.05,)
    } else {
        (locals.var_lr,)
    }
};
        locals.var_lr = assign11450_e14890;
        locals.var_lr_rv = 0.0;

        let (assign11460_e14899,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11460_e14894: f64 = (p.p1089 + p.p90);
        let assign11460_e14896: f64 = (assign11460_e14894 - locals.var_hrsd2);
        let assign11460_e14897: f64 = (assign11460_e14896).abs();
        (assign11460_e14897,)
    } else {
        (locals.var_hgdelta,)
    }
};
        locals.var_hgdelta = assign11460_e14899;
        locals.var_hgdelta_rv = 0.0;

        let (assign11470_e14905,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11470_e14903: f64 = (p.p1087 * locals.var_lr);
        (assign11470_e14903,)
    } else {
        (locals.var_lmax,)
    }
};
        locals.var_lmax = assign11470_e14905;
        locals.var_lmax_rv = 0.0;

        let (assign11480_e14913,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11480_e14910: f64 = (p.p1089 + p.p90);
        let assign11480_e14911: f64 = (locals.var_hrsd2).min(assign11480_e14910);
        (assign11480_e14911,)
    } else {
        (locals.var_y,)
    }
};
        locals.var_y = assign11480_e14913;
        locals.var_y_rv = 0.0;

        let (assign11490_e14921,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11490_e14918: f64 = (locals.var_hr + 1.0);
        let assign11490_e14919: f64 = (p.p1087 / assign11490_e14918);
        (assign11490_e14919,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign11490_e14921;
        locals.var_x_rv = 0.0;

        let (assign11500_e14925,) = {
    if (locals.var_guard213 != 0.0) {
        (1700000000000.0,)
    } else {
        (locals.var_cnon,)
    }
};
        locals.var_cnon = assign11500_e14925;
        locals.var_cnon_rv = 0.0;

        let (assign11510_e14935,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11510_e14930: f64 = (locals.var_y - locals.var_x);
        let assign11510_e14931: f64 = (locals.var_epssp * assign11510_e14930);
        let assign11510_e14933: f64 = (assign11510_e14931 / p.p1087);
        (assign11510_e14933,)
    } else {
        (locals.var_ccgsat,)
    }
};
        locals.var_ccgsat = assign11510_e14935;
        locals.var_ccgsat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11520_e14941, assign11520_e14941_d_n0, assign11520_e14941_d_n2, assign11520_e14941_d_n3, assign11520_e14941_d_n4, assign11520_e14941_d_n5, assign11520_e14941_d_n6, assign11520_e14941_d_n7, assign11520_e14941_d_n8, assign11520_e14941_d_n9, assign11520_e14941_d_n10, assign11520_e14941_d_n11, assign11520_e14941_d_n13, assign11520_e14941_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11520_e14939: f64 = (locals.var_cnon * locals.var_ccgsat);
        (assign11520_e14939, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign11520_e14941;
        locals.var_tt1_dn0 = assign11520_e14941_d_n0;
        locals.var_tt1_dn2 = assign11520_e14941_d_n2;
        locals.var_tt1_dn3 = assign11520_e14941_d_n3;
        locals.var_tt1_dn4 = assign11520_e14941_d_n4;
        locals.var_tt1_dn5 = assign11520_e14941_d_n5;
        locals.var_tt1_dn6 = assign11520_e14941_d_n6;
        locals.var_tt1_dn7 = assign11520_e14941_d_n7;
        locals.var_tt1_dn8 = assign11520_e14941_d_n8;
        locals.var_tt1_dn9 = assign11520_e14941_d_n9;
        locals.var_tt1_dn10 = assign11520_e14941_d_n10;
        locals.var_tt1_dn11 = assign11520_e14941_d_n11;
        locals.var_tt1_dn13 = assign11520_e14941_d_n13;
        locals.var_tt1_dn14 = assign11520_e14941_d_n14;
        locals.var_tt1_rv = 0.0;

        let assign11530_e14944: f64 = if locals.var_tt1 > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard216 = assign11530_e14944;
        locals.var_guard216_rv = 0.0;

        let (assign11540_e14950, assign11540_e14950_d_n0, assign11540_e14950_d_n2, assign11540_e14950_d_n3, assign11540_e14950_d_n4, assign11540_e14950_d_n5, assign11540_e14950_d_n6, assign11540_e14950_d_n7, assign11540_e14950_d_n8, assign11540_e14950_d_n9, assign11540_e14950_d_n10, assign11540_e14950_d_n11, assign11540_e14950_d_n13, assign11540_e14950_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard216 != 0.0)) {
        (locals.var_ccgsat, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign11540_e14950;
        locals.var_ccg1_dn0 = assign11540_e14950_d_n0;
        locals.var_ccg1_dn2 = assign11540_e14950_d_n2;
        locals.var_ccg1_dn3 = assign11540_e14950_d_n3;
        locals.var_ccg1_dn4 = assign11540_e14950_d_n4;
        locals.var_ccg1_dn5 = assign11540_e14950_d_n5;
        locals.var_ccg1_dn6 = assign11540_e14950_d_n6;
        locals.var_ccg1_dn7 = assign11540_e14950_d_n7;
        locals.var_ccg1_dn8 = assign11540_e14950_d_n8;
        locals.var_ccg1_dn9 = assign11540_e14950_d_n9;
        locals.var_ccg1_dn10 = assign11540_e14950_d_n10;
        locals.var_ccg1_dn11 = assign11540_e14950_d_n11;
        locals.var_ccg1_dn13 = assign11540_e14950_d_n13;
        locals.var_ccg1_dn14 = assign11540_e14950_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign11550_e14994, assign11550_e14994_d_n0, assign11550_e14994_d_n2, assign11550_e14994_d_n3, assign11550_e14994_d_n4, assign11550_e14994_d_n5, assign11550_e14994_d_n6, assign11550_e14994_d_n7, assign11550_e14994_d_n8, assign11550_e14994_d_n9, assign11550_e14994_d_n10, assign11550_e14994_d_n11, assign11550_e14994_d_n13, assign11550_e14994_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard216 == 0.0)) {
        let assign11550_e14957: f64 = (1.0 / locals.var_cnon);
        let assign11550_e14964: f64 = (-37.0);
        let (assign11550_e14991, assign11550_e14991_d_n0, assign11550_e14991_d_n2, assign11550_e14991_d_n3, assign11550_e14991_d_n4, assign11550_e14991_d_n5, assign11550_e14991_d_n6, assign11550_e14991_d_n7, assign11550_e14991_d_n8, assign11550_e14991_d_n9, assign11550_e14991_d_n10, assign11550_e14991_d_n11, assign11550_e14991_d_n13, assign11550_e14991_d_n14,) = {
            if ((!(locals.var_tt1 > 37.0)) && (!(locals.var_tt1 < assign11550_e14964))) {
                let assign11550_e14970: f64 = (locals.var_tt1).exp();
                let assign11550_e14971: f64 = (1.0 + assign11550_e14970);
                let assign11550_e14972: f64 = (assign11550_e14971).ln();
                (assign11550_e14972, ((assign11550_e14970 * locals.var_tt1_dn0) / assign11550_e14971), ((assign11550_e14970 * locals.var_tt1_dn2) / assign11550_e14971), ((assign11550_e14970 * locals.var_tt1_dn3) / assign11550_e14971), ((assign11550_e14970 * locals.var_tt1_dn4) / assign11550_e14971), ((assign11550_e14970 * locals.var_tt1_dn5) / assign11550_e14971), ((assign11550_e14970 * locals.var_tt1_dn6) / assign11550_e14971), ((assign11550_e14970 * locals.var_tt1_dn7) / assign11550_e14971), ((assign11550_e14970 * locals.var_tt1_dn8) / assign11550_e14971), ((assign11550_e14970 * locals.var_tt1_dn9) / assign11550_e14971), ((assign11550_e14970 * locals.var_tt1_dn10) / assign11550_e14971), ((assign11550_e14970 * locals.var_tt1_dn11) / assign11550_e14971), ((assign11550_e14970 * locals.var_tt1_dn13) / assign11550_e14971), ((assign11550_e14970 * locals.var_tt1_dn14) / assign11550_e14971),)
            } else {
                let assign11550_e14979: f64 = (-37.0);
                let (assign11550_e14990, assign11550_e14990_d_n0, assign11550_e14990_d_n2, assign11550_e14990_d_n3, assign11550_e14990_d_n4, assign11550_e14990_d_n5, assign11550_e14990_d_n6, assign11550_e14990_d_n7, assign11550_e14990_d_n8, assign11550_e14990_d_n9, assign11550_e14990_d_n10, assign11550_e14990_d_n11, assign11550_e14990_d_n13, assign11550_e14990_d_n14,) = {
                    if ((!(locals.var_tt1 > 37.0)) && (locals.var_tt1 < assign11550_e14979)) {
                        let assign11550_e14983: f64 = (locals.var_tt1).exp();
                        (assign11550_e14983, (assign11550_e14983 * locals.var_tt1_dn0), (assign11550_e14983 * locals.var_tt1_dn2), (assign11550_e14983 * locals.var_tt1_dn3), (assign11550_e14983 * locals.var_tt1_dn4), (assign11550_e14983 * locals.var_tt1_dn5), (assign11550_e14983 * locals.var_tt1_dn6), (assign11550_e14983 * locals.var_tt1_dn7), (assign11550_e14983 * locals.var_tt1_dn8), (assign11550_e14983 * locals.var_tt1_dn9), (assign11550_e14983 * locals.var_tt1_dn10), (assign11550_e14983 * locals.var_tt1_dn11), (assign11550_e14983 * locals.var_tt1_dn13), (assign11550_e14983 * locals.var_tt1_dn14),)
                    } else {
                        let (assign11550_e14989, assign11550_e14989_d_n0, assign11550_e14989_d_n2, assign11550_e14989_d_n3, assign11550_e14989_d_n4, assign11550_e14989_d_n5, assign11550_e14989_d_n6, assign11550_e14989_d_n7, assign11550_e14989_d_n8, assign11550_e14989_d_n9, assign11550_e14989_d_n10, assign11550_e14989_d_n11, assign11550_e14989_d_n13, assign11550_e14989_d_n14,) = {
                            if (locals.var_tt1 > 37.0) {
                                (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign11550_e14989, assign11550_e14989_d_n0, assign11550_e14989_d_n2, assign11550_e14989_d_n3, assign11550_e14989_d_n4, assign11550_e14989_d_n5, assign11550_e14989_d_n6, assign11550_e14989_d_n7, assign11550_e14989_d_n8, assign11550_e14989_d_n9, assign11550_e14989_d_n10, assign11550_e14989_d_n11, assign11550_e14989_d_n13, assign11550_e14989_d_n14,)
                    }
                };
                (assign11550_e14990, assign11550_e14990_d_n0, assign11550_e14990_d_n2, assign11550_e14990_d_n3, assign11550_e14990_d_n4, assign11550_e14990_d_n5, assign11550_e14990_d_n6, assign11550_e14990_d_n7, assign11550_e14990_d_n8, assign11550_e14990_d_n9, assign11550_e14990_d_n10, assign11550_e14990_d_n11, assign11550_e14990_d_n13, assign11550_e14990_d_n14,)
            }
        };
        let assign11550_e14992: f64 = (assign11550_e14957 * assign11550_e14991);
        (assign11550_e14992, (assign11550_e14957 * assign11550_e14991_d_n0), (assign11550_e14957 * assign11550_e14991_d_n2), (assign11550_e14957 * assign11550_e14991_d_n3), (assign11550_e14957 * assign11550_e14991_d_n4), (assign11550_e14957 * assign11550_e14991_d_n5), (assign11550_e14957 * assign11550_e14991_d_n6), (assign11550_e14957 * assign11550_e14991_d_n7), (assign11550_e14957 * assign11550_e14991_d_n8), (assign11550_e14957 * assign11550_e14991_d_n9), (assign11550_e14957 * assign11550_e14991_d_n10), (assign11550_e14957 * assign11550_e14991_d_n11), (assign11550_e14957 * assign11550_e14991_d_n13), (assign11550_e14957 * assign11550_e14991_d_n14),)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign11550_e14994;
        locals.var_ccg1_dn0 = assign11550_e14994_d_n0;
        locals.var_ccg1_dn2 = assign11550_e14994_d_n2;
        locals.var_ccg1_dn3 = assign11550_e14994_d_n3;
        locals.var_ccg1_dn4 = assign11550_e14994_d_n4;
        locals.var_ccg1_dn5 = assign11550_e14994_d_n5;
        locals.var_ccg1_dn6 = assign11550_e14994_d_n6;
        locals.var_ccg1_dn7 = assign11550_e14994_d_n7;
        locals.var_ccg1_dn8 = assign11550_e14994_d_n8;
        locals.var_ccg1_dn9 = assign11550_e14994_d_n9;
        locals.var_ccg1_dn10 = assign11550_e14994_d_n10;
        locals.var_ccg1_dn11 = assign11550_e14994_d_n11;
        locals.var_ccg1_dn13 = assign11550_e14994_d_n13;
        locals.var_ccg1_dn14 = assign11550_e14994_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign11560_e15010,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11560_e15000: f64 = (p.p1089 + p.p90);
        let assign11560_e15001: f64 = (locals.var_hrsd2 / assign11560_e15000);
        let assign11560_e15004: f64 = (p.p1089 + p.p90);
        let assign11560_e15006: f64 = (assign11560_e15004 / locals.var_hrsd2);
        let assign11560_e15007: f64 = (assign11560_e15001).min(assign11560_e15006);
        let assign11560_e15008: f64 = (0.5 * assign11560_e15007);
        (assign11560_e15008,)
    } else {
        (locals.var_r1cf,)
    }
};
        locals.var_r1cf = assign11560_e15010;
        locals.var_r1cf_rv = 0.0;

        let (assign11570_e15016,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11570_e15014: f64 = (locals.var_hgdelta * locals.var_r1cf);
        (assign11570_e15014,)
    } else {
        (locals.var_rcf,)
    }
};
        locals.var_rcf = assign11570_e15016;
        locals.var_rcf_rv = 0.0;

        let (assign11580_e15063,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11580_e15020: f64 = (locals.var_epssp * 2.0);
        let assign11580_e15022: f64 = (assign11580_e15020 / 3.141592653589793);
        let assign11580_e15026: f64 = (0.5 * 3.141592653589793);
        let assign11580_e15028: f64 = (assign11580_e15026 * locals.var_rcf);
        let assign11580_e15029: f64 = (p.p1087 + assign11580_e15028);
        let assign11580_e15031: f64 = (assign11580_e15029 / p.p1087);
        let (assign11580_e15060,) = {
            if (!(assign11580_e15031 > 1e-38)) {
                let assign11580_e15036: f64 = (-87.498233534);
                (assign11580_e15036,)
            } else {
                let assign11580_e15040: f64 = (0.5 * 3.141592653589793);
                let assign11580_e15042: f64 = (assign11580_e15040 * locals.var_rcf);
                let assign11580_e15043: f64 = (p.p1087 + assign11580_e15042);
                let assign11580_e15045: f64 = (assign11580_e15043 / p.p1087);
                let (assign11580_e15059,) = {
                    if (assign11580_e15045 > 1e-38) {
                        let assign11580_e15051: f64 = (0.5 * 3.141592653589793);
                        let assign11580_e15053: f64 = (assign11580_e15051 * locals.var_rcf);
                        let assign11580_e15054: f64 = (p.p1087 + assign11580_e15053);
                        let assign11580_e15056: f64 = (assign11580_e15054 / p.p1087);
                        let assign11580_e15057: f64 = (assign11580_e15056).ln();
                        (assign11580_e15057,)
                    } else {
                        (0.0,)
                    }
                };
                (assign11580_e15059,)
            }
        };
        let assign11580_e15061: f64 = (assign11580_e15022 * assign11580_e15060);
        (assign11580_e15061,)
    } else {
        (locals.var_ccg2,)
    }
};
        locals.var_ccg2 = assign11580_e15063;
        locals.var_ccg2_rv = 0.0;

        let (assign11590_e15071, assign11590_e15071_d_n0, assign11590_e15071_d_n2, assign11590_e15071_d_n3, assign11590_e15071_d_n4, assign11590_e15071_d_n5, assign11590_e15071_d_n6, assign11590_e15071_d_n7, assign11590_e15071_d_n8, assign11590_e15071_d_n9, assign11590_e15071_d_n10, assign11590_e15071_d_n11, assign11590_e15071_d_n13, assign11590_e15071_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11590_e15068: f64 = (locals.var_ccg1 + locals.var_ccg2);
        let assign11590_e15069: f64 = (p.p43 * assign11590_e15068);
        (assign11590_e15069, (p.p43 * locals.var_ccg1_dn0), (p.p43 * locals.var_ccg1_dn2), (p.p43 * locals.var_ccg1_dn3), (p.p43 * locals.var_ccg1_dn4), (p.p43 * locals.var_ccg1_dn5), (p.p43 * locals.var_ccg1_dn6), (p.p43 * locals.var_ccg1_dn7), (p.p43 * locals.var_ccg1_dn8), (p.p43 * locals.var_ccg1_dn9), (p.p43 * locals.var_ccg1_dn10), (p.p43 * locals.var_ccg1_dn11), (p.p43 * locals.var_ccg1_dn13), (p.p43 * locals.var_ccg1_dn14),)
    } else {
        (locals.var_ccg, locals.var_ccg_dn0, locals.var_ccg_dn2, locals.var_ccg_dn3, locals.var_ccg_dn4, locals.var_ccg_dn5, locals.var_ccg_dn6, locals.var_ccg_dn7, locals.var_ccg_dn8, locals.var_ccg_dn9, locals.var_ccg_dn10, locals.var_ccg_dn11, locals.var_ccg_dn13, locals.var_ccg_dn14,)
    }
};
        locals.var_ccg = assign11590_e15071;
        locals.var_ccg_dn0 = assign11590_e15071_d_n0;
        locals.var_ccg_dn2 = assign11590_e15071_d_n2;
        locals.var_ccg_dn3 = assign11590_e15071_d_n3;
        locals.var_ccg_dn4 = assign11590_e15071_d_n4;
        locals.var_ccg_dn5 = assign11590_e15071_d_n5;
        locals.var_ccg_dn6 = assign11590_e15071_d_n6;
        locals.var_ccg_dn7 = assign11590_e15071_d_n7;
        locals.var_ccg_dn8 = assign11590_e15071_d_n8;
        locals.var_ccg_dn9 = assign11590_e15071_d_n9;
        locals.var_ccg_dn10 = assign11590_e15071_d_n10;
        locals.var_ccg_dn11 = assign11590_e15071_d_n11;
        locals.var_ccg_dn13 = assign11590_e15071_d_n13;
        locals.var_ccg_dn14 = assign11590_e15071_d_n14;
        locals.var_ccg_rv = 0.0;

        let (assign11600_e15077,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11600_e15075: f64 = (locals.var_lmax / p.p1089);
        (assign11600_e15075,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign11600_e15077;
        locals.var_x_rv = 0.0;

        let (assign11610_e15090,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11610_e15083: f64 = (locals.var_x + 1.0);
        let assign11610_e15084: f64 = (2.0 * assign11610_e15083);
        let assign11610_e15085: f64 = (assign11610_e15084).sqrt();
        let assign11610_e15087: f64 = (assign11610_e15085 * 3.141592653589793);
        let assign11610_e15088: f64 = (4.0 / assign11610_e15087);
        (assign11610_e15088,)
    } else {
        (locals.var_c1,)
    }
};
        locals.var_c1 = assign11610_e15090;
        locals.var_c1_rv = 0.0;

        let (assign11620_e15124,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11620_e15094: f64 = (p.p90 * p.p90);
        let assign11620_e15097: f64 = (2.0 * p.p1089);
        let assign11620_e15099: f64 = (assign11620_e15097 * p.p90);
        let assign11620_e15100: f64 = (assign11620_e15094 + assign11620_e15099);
        let assign11620_e15103: f64 = (p.p1089 * p.p1089);
        let assign11620_e15106: f64 = (locals.var_x + 1.0);
        let assign11620_e15107: f64 = (assign11620_e15103 * assign11620_e15106);
        let assign11620_e15108: f64 = (assign11620_e15100 + assign11620_e15107);
        let assign11620_e15109: f64 = (assign11620_e15108).sqrt();
        let assign11620_e15112: f64 = (locals.var_x + 1.0);
        let assign11620_e15113: f64 = (assign11620_e15112).sqrt();
        let assign11620_e15114: f64 = (assign11620_e15109 * assign11620_e15113);
        let assign11620_e15116: f64 = (assign11620_e15114 + p.p90);
        let assign11620_e15119: f64 = (p.p1089 * locals.var_x);
        let assign11620_e15120: f64 = (assign11620_e15116 + assign11620_e15119);
        let assign11620_e15122: f64 = (assign11620_e15120 + p.p1089);
        (assign11620_e15122,)
    } else {
        (locals.var_c2,)
    }
};
        locals.var_c2 = assign11620_e15124;
        locals.var_c2_rv = 0.0;

        let (assign11630_e15143,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11630_e15129: f64 = (locals.var_x + 1.0);
        let assign11630_e15132: f64 = (locals.var_x + 4.0);
        let assign11630_e15133: f64 = (assign11630_e15129 * assign11630_e15132);
        let assign11630_e15134: f64 = (assign11630_e15133).sqrt();
        let assign11630_e15135: f64 = (p.p90 * assign11630_e15134);
        let assign11630_e15139: f64 = (locals.var_x + 2.0);
        let assign11630_e15140: f64 = (p.p90 * assign11630_e15139);
        let assign11630_e15141: f64 = (assign11630_e15135 + assign11630_e15140);
        (assign11630_e15141,)
    } else {
        (locals.var_c3,)
    }
};
        locals.var_c3 = assign11630_e15143;
        locals.var_c3_rv = 0.0;

        let (assign11640_e15172,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11640_e15149: f64 = (locals.var_c2 / locals.var_c3);
        let (assign11640_e15166,) = {
            if (!(assign11640_e15149 > 1e-38)) {
                let assign11640_e15154: f64 = (-87.498233534);
                (assign11640_e15154,)
            } else {
                let assign11640_e15157: f64 = (locals.var_c2 / locals.var_c3);
                let (assign11640_e15165,) = {
                    if (assign11640_e15157 > 1e-38) {
                        let assign11640_e15162: f64 = (locals.var_c2 / locals.var_c3);
                        let assign11640_e15163: f64 = (assign11640_e15162).ln();
                        (assign11640_e15163,)
                    } else {
                        (0.0,)
                    }
                };
                (assign11640_e15165,)
            }
        };
        let assign11640_e15167: f64 = (locals.var_c1 * assign11640_e15166);
        let assign11640_e15169: f64 = (assign11640_e15167 + 12.27);
        let assign11640_e15170: f64 = (locals.var_epssp * assign11640_e15169);
        (assign11640_e15170,)
    } else {
        (locals.var_cfglog,)
    }
};
        locals.var_cfglog = assign11640_e15172;
        locals.var_cfglog_rv = 0.0;

        let (assign11650_e15178,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11650_e15176: f64 = (locals.var_hr * locals.var_lr);
        (assign11650_e15176,)
    } else {
        (locals.var_dcf,)
    }
};
        locals.var_dcf = assign11650_e15178;
        locals.var_dcf_rv = 0.0;

        let (assign11660_e15187,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11660_e15182: f64 = (locals.var_dcf * locals.var_dcf);
        let assign11660_e15184: f64 = (assign11660_e15182 + 1.0);
        let assign11660_e15185: f64 = (assign11660_e15184).sqrt();
        (assign11660_e15185,)
    } else {
        (locals.var_tt0,)
    }
};
        locals.var_tt0 = assign11660_e15187;
        locals.var_tt0_rv = 0.0;

        let (assign11670_e15234, assign11670_e15234_d_n0, assign11670_e15234_d_n2, assign11670_e15234_d_n3, assign11670_e15234_d_n4, assign11670_e15234_d_n5, assign11670_e15234_d_n6, assign11670_e15234_d_n7, assign11670_e15234_d_n8, assign11670_e15234_d_n9, assign11670_e15234_d_n10, assign11670_e15234_d_n11, assign11670_e15234_d_n13, assign11670_e15234_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11670_e15191: f64 = (locals.var_dcf * locals.var_dcf);
        let assign11670_e15193: f64 = (assign11670_e15191 + 1.0);
        let assign11670_e15196: f64 = (locals.var_dcf * p.p90);
        let assign11670_e15199: f64 = (locals.var_dcf * p.p90);
        let assign11670_e15200: f64 = (assign11670_e15196 * assign11670_e15199);
        let assign11670_e15203: f64 = (2.0 * locals.var_dcf);
        let assign11670_e15205: f64 = (assign11670_e15203 * locals.var_lmax);
        let assign11670_e15207: f64 = (assign11670_e15205 * p.p90);
        let assign11670_e15208: f64 = (assign11670_e15200 + assign11670_e15207);
        let assign11670_e15211: f64 = (locals.var_dcf * locals.var_dcf);
        let assign11670_e15213: f64 = (assign11670_e15211 + 1.0);
        let assign11670_e15215: f64 = (assign11670_e15213 * locals.var_lmax);
        let assign11670_e15217: f64 = (assign11670_e15215 * locals.var_lmax);
        let assign11670_e15218: f64 = (assign11670_e15208 + assign11670_e15217);
        let assign11670_e15219: f64 = (assign11670_e15193 * assign11670_e15218);
        let assign11670_e15220: f64 = (assign11670_e15219).sqrt();
        let assign11670_e15223: f64 = (locals.var_dcf * p.p90);
        let assign11670_e15224: f64 = (assign11670_e15220 + assign11670_e15223);
        let assign11670_e15227: f64 = (locals.var_dcf * locals.var_dcf);
        let assign11670_e15229: f64 = (assign11670_e15227 * locals.var_lmax);
        let assign11670_e15230: f64 = (assign11670_e15224 + assign11670_e15229);
        let assign11670_e15232: f64 = (assign11670_e15230 + locals.var_lmax);
        (assign11670_e15232, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign11670_e15234;
        locals.var_tt1_dn0 = assign11670_e15234_d_n0;
        locals.var_tt1_dn2 = assign11670_e15234_d_n2;
        locals.var_tt1_dn3 = assign11670_e15234_d_n3;
        locals.var_tt1_dn4 = assign11670_e15234_d_n4;
        locals.var_tt1_dn5 = assign11670_e15234_d_n5;
        locals.var_tt1_dn6 = assign11670_e15234_d_n6;
        locals.var_tt1_dn7 = assign11670_e15234_d_n7;
        locals.var_tt1_dn8 = assign11670_e15234_d_n8;
        locals.var_tt1_dn9 = assign11670_e15234_d_n9;
        locals.var_tt1_dn10 = assign11670_e15234_d_n10;
        locals.var_tt1_dn11 = assign11670_e15234_d_n11;
        locals.var_tt1_dn13 = assign11670_e15234_d_n13;
        locals.var_tt1_dn14 = assign11670_e15234_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign11680_e15244,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11680_e15238: f64 = (locals.var_tt0 + 1.0);
        let assign11680_e15241: f64 = (locals.var_dcf * p.p90);
        let assign11680_e15242: f64 = (assign11680_e15238 * assign11680_e15241);
        (assign11680_e15242,)
    } else {
        (locals.var_tt2,)
    }
};
        locals.var_tt2 = assign11680_e15244;
        locals.var_tt2_rv = 0.0;

        let (assign11690_e15282, assign11690_e15282_d_n0, assign11690_e15282_d_n2, assign11690_e15282_d_n3, assign11690_e15282_d_n4, assign11690_e15282_d_n5, assign11690_e15282_d_n6, assign11690_e15282_d_n7, assign11690_e15282_d_n8, assign11690_e15282_d_n9, assign11690_e15282_d_n10, assign11690_e15282_d_n11, assign11690_e15282_d_n13, assign11690_e15282_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11690_e15248: f64 = (2.0 * locals.var_epssp);
        let assign11690_e15250: f64 = (2.0_f64).sqrt();
        let assign11690_e15251: f64 = (assign11690_e15248 * assign11690_e15250);
        let assign11690_e15253: f64 = (assign11690_e15251 / 3.141592653589793);
        let assign11690_e15255: f64 = (assign11690_e15253 * 0.85);
        let assign11690_e15257: f64 = (assign11690_e15255 * locals.var_dcf);
        let assign11690_e15259: f64 = (assign11690_e15257 / locals.var_tt0);
        let assign11690_e15262: f64 = (locals.var_tt1 / locals.var_tt2);
        let (assign11690_e15279, assign11690_e15279_d_n0, assign11690_e15279_d_n2, assign11690_e15279_d_n3, assign11690_e15279_d_n4, assign11690_e15279_d_n5, assign11690_e15279_d_n6, assign11690_e15279_d_n7, assign11690_e15279_d_n8, assign11690_e15279_d_n9, assign11690_e15279_d_n10, assign11690_e15279_d_n11, assign11690_e15279_d_n13, assign11690_e15279_d_n14,) = {
            if (!(assign11690_e15262 > 1e-38)) {
                let assign11690_e15267: f64 = (-87.498233534);
                (assign11690_e15267, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign11690_e15270: f64 = (locals.var_tt1 / locals.var_tt2);
                let (assign11690_e15278, assign11690_e15278_d_n0, assign11690_e15278_d_n2, assign11690_e15278_d_n3, assign11690_e15278_d_n4, assign11690_e15278_d_n5, assign11690_e15278_d_n6, assign11690_e15278_d_n7, assign11690_e15278_d_n8, assign11690_e15278_d_n9, assign11690_e15278_d_n10, assign11690_e15278_d_n11, assign11690_e15278_d_n13, assign11690_e15278_d_n14,) = {
                    if (assign11690_e15270 > 1e-38) {
                        let assign11690_e15275: f64 = (locals.var_tt1 / locals.var_tt2);
                        let assign11690_e15276: f64 = (assign11690_e15275).ln();
                        (assign11690_e15276, ((locals.var_tt1_dn0 / locals.var_tt2) / assign11690_e15275), ((locals.var_tt1_dn2 / locals.var_tt2) / assign11690_e15275), ((locals.var_tt1_dn3 / locals.var_tt2) / assign11690_e15275), ((locals.var_tt1_dn4 / locals.var_tt2) / assign11690_e15275), ((locals.var_tt1_dn5 / locals.var_tt2) / assign11690_e15275), ((locals.var_tt1_dn6 / locals.var_tt2) / assign11690_e15275), ((locals.var_tt1_dn7 / locals.var_tt2) / assign11690_e15275), ((locals.var_tt1_dn8 / locals.var_tt2) / assign11690_e15275), ((locals.var_tt1_dn9 / locals.var_tt2) / assign11690_e15275), ((locals.var_tt1_dn10 / locals.var_tt2) / assign11690_e15275), ((locals.var_tt1_dn11 / locals.var_tt2) / assign11690_e15275), ((locals.var_tt1_dn13 / locals.var_tt2) / assign11690_e15275), ((locals.var_tt1_dn14 / locals.var_tt2) / assign11690_e15275),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign11690_e15278, assign11690_e15278_d_n0, assign11690_e15278_d_n2, assign11690_e15278_d_n3, assign11690_e15278_d_n4, assign11690_e15278_d_n5, assign11690_e15278_d_n6, assign11690_e15278_d_n7, assign11690_e15278_d_n8, assign11690_e15278_d_n9, assign11690_e15278_d_n10, assign11690_e15278_d_n11, assign11690_e15278_d_n13, assign11690_e15278_d_n14,)
            }
        };
        let assign11690_e15280: f64 = (assign11690_e15259 * assign11690_e15279);
        (assign11690_e15280, (assign11690_e15259 * assign11690_e15279_d_n0), (assign11690_e15259 * assign11690_e15279_d_n2), (assign11690_e15259 * assign11690_e15279_d_n3), (assign11690_e15259 * assign11690_e15279_d_n4), (assign11690_e15259 * assign11690_e15279_d_n5), (assign11690_e15259 * assign11690_e15279_d_n6), (assign11690_e15259 * assign11690_e15279_d_n7), (assign11690_e15259 * assign11690_e15279_d_n8), (assign11690_e15259 * assign11690_e15279_d_n9), (assign11690_e15259 * assign11690_e15279_d_n10), (assign11690_e15259 * assign11690_e15279_d_n11), (assign11690_e15259 * assign11690_e15279_d_n13), (assign11690_e15259 * assign11690_e15279_d_n14),)
    } else {
        (locals.var_cfgsat, locals.var_cfgsat_dn0, locals.var_cfgsat_dn2, locals.var_cfgsat_dn3, locals.var_cfgsat_dn4, locals.var_cfgsat_dn5, locals.var_cfgsat_dn6, locals.var_cfgsat_dn7, locals.var_cfgsat_dn8, locals.var_cfgsat_dn9, locals.var_cfgsat_dn10, locals.var_cfgsat_dn11, locals.var_cfgsat_dn13, locals.var_cfgsat_dn14,)
    }
};
        locals.var_cfgsat = assign11690_e15282;
        locals.var_cfgsat_dn0 = assign11690_e15282_d_n0;
        locals.var_cfgsat_dn2 = assign11690_e15282_d_n2;
        locals.var_cfgsat_dn3 = assign11690_e15282_d_n3;
        locals.var_cfgsat_dn4 = assign11690_e15282_d_n4;
        locals.var_cfgsat_dn5 = assign11690_e15282_d_n5;
        locals.var_cfgsat_dn6 = assign11690_e15282_d_n6;
        locals.var_cfgsat_dn7 = assign11690_e15282_d_n7;
        locals.var_cfgsat_dn8 = assign11690_e15282_d_n8;
        locals.var_cfgsat_dn9 = assign11690_e15282_d_n9;
        locals.var_cfgsat_dn10 = assign11690_e15282_d_n10;
        locals.var_cfgsat_dn11 = assign11690_e15282_d_n11;
        locals.var_cfgsat_dn13 = assign11690_e15282_d_n13;
        locals.var_cfgsat_dn14 = assign11690_e15282_d_n14;
        locals.var_cfgsat_rv = 0.0;

        let (assign11700_e15286, assign11700_e15286_d_n0, assign11700_e15286_d_n2, assign11700_e15286_d_n3, assign11700_e15286_d_n4, assign11700_e15286_d_n5, assign11700_e15286_d_n6, assign11700_e15286_d_n7, assign11700_e15286_d_n8, assign11700_e15286_d_n9, assign11700_e15286_d_n10, assign11700_e15286_d_n11, assign11700_e15286_d_n13, assign11700_e15286_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        (1.2e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign11700_e15286;
        locals.var_delta_dn0 = assign11700_e15286_d_n0;
        locals.var_delta_dn2 = assign11700_e15286_d_n2;
        locals.var_delta_dn3 = assign11700_e15286_d_n3;
        locals.var_delta_dn4 = assign11700_e15286_d_n4;
        locals.var_delta_dn5 = assign11700_e15286_d_n5;
        locals.var_delta_dn6 = assign11700_e15286_d_n6;
        locals.var_delta_dn7 = assign11700_e15286_d_n7;
        locals.var_delta_dn8 = assign11700_e15286_d_n8;
        locals.var_delta_dn9 = assign11700_e15286_d_n9;
        locals.var_delta_dn10 = assign11700_e15286_d_n10;
        locals.var_delta_dn11 = assign11700_e15286_d_n11;
        locals.var_delta_dn13 = assign11700_e15286_d_n13;
        locals.var_delta_dn14 = assign11700_e15286_d_n14;
        locals.var_delta_rv = 0.0;

        let (assign11710_e15294, assign11710_e15294_d_n0, assign11710_e15294_d_n2, assign11710_e15294_d_n3, assign11710_e15294_d_n4, assign11710_e15294_d_n5, assign11710_e15294_d_n6, assign11710_e15294_d_n7, assign11710_e15294_d_n8, assign11710_e15294_d_n9, assign11710_e15294_d_n10, assign11710_e15294_d_n11, assign11710_e15294_d_n13, assign11710_e15294_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11710_e15290: f64 = (locals.var_cfgsat - locals.var_cfglog);
        let assign11710_e15292: f64 = (assign11710_e15290 - locals.var_delta);
        (assign11710_e15292, (locals.var_cfgsat_dn0 - locals.var_delta_dn0), (locals.var_cfgsat_dn2 - locals.var_delta_dn2), (locals.var_cfgsat_dn3 - locals.var_delta_dn3), (locals.var_cfgsat_dn4 - locals.var_delta_dn4), (locals.var_cfgsat_dn5 - locals.var_delta_dn5), (locals.var_cfgsat_dn6 - locals.var_delta_dn6), (locals.var_cfgsat_dn7 - locals.var_delta_dn7), (locals.var_cfgsat_dn8 - locals.var_delta_dn8), (locals.var_cfgsat_dn9 - locals.var_delta_dn9), (locals.var_cfgsat_dn10 - locals.var_delta_dn10), (locals.var_cfgsat_dn11 - locals.var_delta_dn11), (locals.var_cfgsat_dn13 - locals.var_delta_dn13), (locals.var_cfgsat_dn14 - locals.var_delta_dn14),)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign11710_e15294;
        locals.var_tt1_dn0 = assign11710_e15294_d_n0;
        locals.var_tt1_dn2 = assign11710_e15294_d_n2;
        locals.var_tt1_dn3 = assign11710_e15294_d_n3;
        locals.var_tt1_dn4 = assign11710_e15294_d_n4;
        locals.var_tt1_dn5 = assign11710_e15294_d_n5;
        locals.var_tt1_dn6 = assign11710_e15294_d_n6;
        locals.var_tt1_dn7 = assign11710_e15294_d_n7;
        locals.var_tt1_dn8 = assign11710_e15294_d_n8;
        locals.var_tt1_dn9 = assign11710_e15294_d_n9;
        locals.var_tt1_dn10 = assign11710_e15294_d_n10;
        locals.var_tt1_dn11 = assign11710_e15294_d_n11;
        locals.var_tt1_dn13 = assign11710_e15294_d_n13;
        locals.var_tt1_dn14 = assign11710_e15294_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign11720_e15315, assign11720_e15315_d_n0, assign11720_e15315_d_n2, assign11720_e15315_d_n3, assign11720_e15315_d_n4, assign11720_e15315_d_n5, assign11720_e15315_d_n6, assign11720_e15315_d_n7, assign11720_e15315_d_n8, assign11720_e15315_d_n9, assign11720_e15315_d_n10, assign11720_e15315_d_n11, assign11720_e15315_d_n13, assign11720_e15315_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11720_e15302: f64 = (locals.var_tt1 * locals.var_tt1);
        let assign11720_e15305: f64 = (4.0 * locals.var_delta);
        let assign11720_e15307: f64 = (assign11720_e15305 * locals.var_cfgsat);
        let assign11720_e15308: f64 = (assign11720_e15302 + assign11720_e15307);
        let assign11720_e15309: f64 = (assign11720_e15308).sqrt();
        let assign11720_e15310: f64 = (locals.var_tt1 + assign11720_e15309);
        let assign11720_e15311: f64 = (0.5 * assign11720_e15310);
        let assign11720_e15312: f64 = (locals.var_cfgsat - assign11720_e15311);
        let assign11720_e15313: f64 = (p.p43 * assign11720_e15312);
        (assign11720_e15313, (p.p43 * (locals.var_cfgsat_dn0 - (0.5 * (locals.var_tt1_dn0 + ((((locals.var_tt1_dn0 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn0))) / (2.0 * assign11720_e15309)))))), (p.p43 * (locals.var_cfgsat_dn2 - (0.5 * (locals.var_tt1_dn2 + ((((locals.var_tt1_dn2 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn2))) / (2.0 * assign11720_e15309)))))), (p.p43 * (locals.var_cfgsat_dn3 - (0.5 * (locals.var_tt1_dn3 + ((((locals.var_tt1_dn3 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn3)) + (((4.0 * locals.var_delta_dn3) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn3))) / (2.0 * assign11720_e15309)))))), (p.p43 * (locals.var_cfgsat_dn4 - (0.5 * (locals.var_tt1_dn4 + ((((locals.var_tt1_dn4 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn4)) + (((4.0 * locals.var_delta_dn4) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn4))) / (2.0 * assign11720_e15309)))))), (p.p43 * (locals.var_cfgsat_dn5 - (0.5 * (locals.var_tt1_dn5 + ((((locals.var_tt1_dn5 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn5)) + (((4.0 * locals.var_delta_dn5) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn5))) / (2.0 * assign11720_e15309)))))), (p.p43 * (locals.var_cfgsat_dn6 - (0.5 * (locals.var_tt1_dn6 + ((((locals.var_tt1_dn6 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn6))) / (2.0 * assign11720_e15309)))))), (p.p43 * (locals.var_cfgsat_dn7 - (0.5 * (locals.var_tt1_dn7 + ((((locals.var_tt1_dn7 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn7))) / (2.0 * assign11720_e15309)))))), (p.p43 * (locals.var_cfgsat_dn8 - (0.5 * (locals.var_tt1_dn8 + ((((locals.var_tt1_dn8 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn8)) + (((4.0 * locals.var_delta_dn8) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn8))) / (2.0 * assign11720_e15309)))))), (p.p43 * (locals.var_cfgsat_dn9 - (0.5 * (locals.var_tt1_dn9 + ((((locals.var_tt1_dn9 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn9)) + (((4.0 * locals.var_delta_dn9) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn9))) / (2.0 * assign11720_e15309)))))), (p.p43 * (locals.var_cfgsat_dn10 - (0.5 * (locals.var_tt1_dn10 + ((((locals.var_tt1_dn10 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn10))) / (2.0 * assign11720_e15309)))))), (p.p43 * (locals.var_cfgsat_dn11 - (0.5 * (locals.var_tt1_dn11 + ((((locals.var_tt1_dn11 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn11))) / (2.0 * assign11720_e15309)))))), (p.p43 * (locals.var_cfgsat_dn13 - (0.5 * (locals.var_tt1_dn13 + ((((locals.var_tt1_dn13 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn13)) + (((4.0 * locals.var_delta_dn13) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn13))) / (2.0 * assign11720_e15309)))))), (p.p43 * (locals.var_cfgsat_dn14 - (0.5 * (locals.var_tt1_dn14 + ((((locals.var_tt1_dn14 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn14)) + (((4.0 * locals.var_delta_dn14) * locals.var_cfgsat) + (assign11720_e15305 * locals.var_cfgsat_dn14))) / (2.0 * assign11720_e15309)))))),)
    } else {
        (locals.var_cfg, locals.var_cfg_dn0, locals.var_cfg_dn2, locals.var_cfg_dn3, locals.var_cfg_dn4, locals.var_cfg_dn5, locals.var_cfg_dn6, locals.var_cfg_dn7, locals.var_cfg_dn8, locals.var_cfg_dn9, locals.var_cfg_dn10, locals.var_cfg_dn11, locals.var_cfg_dn13, locals.var_cfg_dn14,)
    }
};
        locals.var_cfg = assign11720_e15315;
        locals.var_cfg_dn0 = assign11720_e15315_d_n0;
        locals.var_cfg_dn2 = assign11720_e15315_d_n2;
        locals.var_cfg_dn3 = assign11720_e15315_d_n3;
        locals.var_cfg_dn4 = assign11720_e15315_d_n4;
        locals.var_cfg_dn5 = assign11720_e15315_d_n5;
        locals.var_cfg_dn6 = assign11720_e15315_d_n6;
        locals.var_cfg_dn7 = assign11720_e15315_d_n7;
        locals.var_cfg_dn8 = assign11720_e15315_d_n8;
        locals.var_cfg_dn9 = assign11720_e15315_d_n9;
        locals.var_cfg_dn10 = assign11720_e15315_d_n10;
        locals.var_cfg_dn11 = assign11720_e15315_d_n11;
        locals.var_cfg_dn13 = assign11720_e15315_d_n13;
        locals.var_cfg_dn14 = assign11720_e15315_d_n14;
        locals.var_cfg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11730_e15321, assign11730_e15321_d_n0, assign11730_e15321_d_n2, assign11730_e15321_d_n3, assign11730_e15321_d_n4, assign11730_e15321_d_n5, assign11730_e15321_d_n6, assign11730_e15321_d_n7, assign11730_e15321_d_n8, assign11730_e15321_d_n9, assign11730_e15321_d_n10, assign11730_e15321_d_n11, assign11730_e15321_d_n13, assign11730_e15321_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign11730_e15319: f64 = (locals.var_ccg + locals.var_cfg);
        (assign11730_e15319, (locals.var_ccg_dn0 + locals.var_cfg_dn0), (locals.var_ccg_dn2 + locals.var_cfg_dn2), (locals.var_ccg_dn3 + locals.var_cfg_dn3), (locals.var_ccg_dn4 + locals.var_cfg_dn4), (locals.var_ccg_dn5 + locals.var_cfg_dn5), (locals.var_ccg_dn6 + locals.var_cfg_dn6), (locals.var_ccg_dn7 + locals.var_cfg_dn7), (locals.var_ccg_dn8 + locals.var_cfg_dn8), (locals.var_ccg_dn9 + locals.var_cfg_dn9), (locals.var_ccg_dn10 + locals.var_cfg_dn10), (locals.var_ccg_dn11 + locals.var_cfg_dn11), (locals.var_ccg_dn13 + locals.var_cfg_dn13), (locals.var_ccg_dn14 + locals.var_cfg_dn14),)
    } else {
        (locals.var_cgg_tb, locals.var_cgg_tb_dn0, locals.var_cgg_tb_dn2, locals.var_cgg_tb_dn3, locals.var_cgg_tb_dn4, locals.var_cgg_tb_dn5, locals.var_cgg_tb_dn6, locals.var_cgg_tb_dn7, locals.var_cgg_tb_dn8, locals.var_cgg_tb_dn9, locals.var_cgg_tb_dn10, locals.var_cgg_tb_dn11, locals.var_cgg_tb_dn13, locals.var_cgg_tb_dn14,)
    }
};
        locals.var_cgg_tb = assign11730_e15321;
        locals.var_cgg_tb_dn0 = assign11730_e15321_d_n0;
        locals.var_cgg_tb_dn2 = assign11730_e15321_d_n2;
        locals.var_cgg_tb_dn3 = assign11730_e15321_d_n3;
        locals.var_cgg_tb_dn4 = assign11730_e15321_d_n4;
        locals.var_cgg_tb_dn5 = assign11730_e15321_d_n5;
        locals.var_cgg_tb_dn6 = assign11730_e15321_d_n6;
        locals.var_cgg_tb_dn7 = assign11730_e15321_d_n7;
        locals.var_cgg_tb_dn8 = assign11730_e15321_d_n8;
        locals.var_cgg_tb_dn9 = assign11730_e15321_d_n9;
        locals.var_cgg_tb_dn10 = assign11730_e15321_d_n10;
        locals.var_cgg_tb_dn11 = assign11730_e15321_d_n11;
        locals.var_cgg_tb_dn13 = assign11730_e15321_d_n13;
        locals.var_cgg_tb_dn14 = assign11730_e15321_d_n14;
        locals.var_cgg_tb_rv = 0.0;

        let assign11740_e15324: f64 = if p.p1090 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard217 = assign11740_e15324;
        locals.var_guard217_rv = 0.0;

        let (assign11750_e15338,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11750_e15332: f64 = (locals.var_wg + p.p90);
        let assign11750_e15333: f64 = (0.2 * assign11750_e15332);
        let assign11750_e15335: f64 = (assign11750_e15333 / locals.var_trsd);
        let assign11750_e15336: f64 = (2.3 + assign11750_e15335);
        (assign11750_e15336,)
    } else {
        (locals.var_hr,)
    }
};
        locals.var_hr = assign11750_e15338;
        locals.var_hr_rv = 0.0;

        let (assign11760_e15344,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        (1.05,)
    } else {
        (locals.var_lr,)
    }
};
        locals.var_lr = assign11760_e15344;
        locals.var_lr_rv = 0.0;

        let (assign11770_e15355,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11770_e15350: f64 = (locals.var_wg + p.p90);
        let assign11770_e15352: f64 = (assign11770_e15350 - locals.var_trsd);
        let assign11770_e15353: f64 = (assign11770_e15352).abs();
        (assign11770_e15353,)
    } else {
        (locals.var_hgdelta,)
    }
};
        locals.var_hgdelta = assign11770_e15355;
        locals.var_hgdelta_rv = 0.0;

        let (assign11780_e15363,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11780_e15361: f64 = (p.p1087 * locals.var_lr);
        (assign11780_e15361,)
    } else {
        (locals.var_lmax,)
    }
};
        locals.var_lmax = assign11780_e15363;
        locals.var_lmax_rv = 0.0;

        let (assign11790_e15373,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11790_e15370: f64 = (locals.var_wg + p.p90);
        let assign11790_e15371: f64 = (locals.var_trsd).min(assign11790_e15370);
        (assign11790_e15371,)
    } else {
        (locals.var_y,)
    }
};
        locals.var_y = assign11790_e15373;
        locals.var_y_rv = 0.0;

        let (assign11800_e15383,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11800_e15380: f64 = (locals.var_hr + 1.0);
        let assign11800_e15381: f64 = (p.p1087 / assign11800_e15380);
        (assign11800_e15381,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign11800_e15383;
        locals.var_x_rv = 0.0;

        let (assign11810_e15389,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        (1700000000000.0,)
    } else {
        (locals.var_cnon,)
    }
};
        locals.var_cnon = assign11810_e15389;
        locals.var_cnon_rv = 0.0;

        let (assign11820_e15401,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11820_e15396: f64 = (locals.var_y - locals.var_x);
        let assign11820_e15397: f64 = (locals.var_epssp * assign11820_e15396);
        let assign11820_e15399: f64 = (assign11820_e15397 / p.p1087);
        (assign11820_e15399,)
    } else {
        (locals.var_ccgsat,)
    }
};
        locals.var_ccgsat = assign11820_e15401;
        locals.var_ccgsat_rv = 0.0;

        let (assign11830_e15409, assign11830_e15409_d_n0, assign11830_e15409_d_n2, assign11830_e15409_d_n3, assign11830_e15409_d_n4, assign11830_e15409_d_n5, assign11830_e15409_d_n6, assign11830_e15409_d_n7, assign11830_e15409_d_n8, assign11830_e15409_d_n9, assign11830_e15409_d_n10, assign11830_e15409_d_n11, assign11830_e15409_d_n13, assign11830_e15409_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11830_e15407: f64 = (locals.var_cnon * locals.var_ccgsat);
        (assign11830_e15407, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign11830_e15409;
        locals.var_tt1_dn0 = assign11830_e15409_d_n0;
        locals.var_tt1_dn2 = assign11830_e15409_d_n2;
        locals.var_tt1_dn3 = assign11830_e15409_d_n3;
        locals.var_tt1_dn4 = assign11830_e15409_d_n4;
        locals.var_tt1_dn5 = assign11830_e15409_d_n5;
        locals.var_tt1_dn6 = assign11830_e15409_d_n6;
        locals.var_tt1_dn7 = assign11830_e15409_d_n7;
        locals.var_tt1_dn8 = assign11830_e15409_d_n8;
        locals.var_tt1_dn9 = assign11830_e15409_d_n9;
        locals.var_tt1_dn10 = assign11830_e15409_d_n10;
        locals.var_tt1_dn11 = assign11830_e15409_d_n11;
        locals.var_tt1_dn13 = assign11830_e15409_d_n13;
        locals.var_tt1_dn14 = assign11830_e15409_d_n14;
        locals.var_tt1_rv = 0.0;

        let assign11840_e15412: f64 = if locals.var_tt1 > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard218 = assign11840_e15412;
        locals.var_guard218_rv = 0.0;

        let (assign11850_e15420, assign11850_e15420_d_n0, assign11850_e15420_d_n2, assign11850_e15420_d_n3, assign11850_e15420_d_n4, assign11850_e15420_d_n5, assign11850_e15420_d_n6, assign11850_e15420_d_n7, assign11850_e15420_d_n8, assign11850_e15420_d_n9, assign11850_e15420_d_n10, assign11850_e15420_d_n11, assign11850_e15420_d_n13, assign11850_e15420_d_n14,) = {
    if (((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) && (locals.var_guard218 != 0.0)) {
        (locals.var_ccgsat, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign11850_e15420;
        locals.var_ccg1_dn0 = assign11850_e15420_d_n0;
        locals.var_ccg1_dn2 = assign11850_e15420_d_n2;
        locals.var_ccg1_dn3 = assign11850_e15420_d_n3;
        locals.var_ccg1_dn4 = assign11850_e15420_d_n4;
        locals.var_ccg1_dn5 = assign11850_e15420_d_n5;
        locals.var_ccg1_dn6 = assign11850_e15420_d_n6;
        locals.var_ccg1_dn7 = assign11850_e15420_d_n7;
        locals.var_ccg1_dn8 = assign11850_e15420_d_n8;
        locals.var_ccg1_dn9 = assign11850_e15420_d_n9;
        locals.var_ccg1_dn10 = assign11850_e15420_d_n10;
        locals.var_ccg1_dn11 = assign11850_e15420_d_n11;
        locals.var_ccg1_dn13 = assign11850_e15420_d_n13;
        locals.var_ccg1_dn14 = assign11850_e15420_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign11860_e15466, assign11860_e15466_d_n0, assign11860_e15466_d_n2, assign11860_e15466_d_n3, assign11860_e15466_d_n4, assign11860_e15466_d_n5, assign11860_e15466_d_n6, assign11860_e15466_d_n7, assign11860_e15466_d_n8, assign11860_e15466_d_n9, assign11860_e15466_d_n10, assign11860_e15466_d_n11, assign11860_e15466_d_n13, assign11860_e15466_d_n14,) = {
    if (((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) && (locals.var_guard218 == 0.0)) {
        let assign11860_e15429: f64 = (1.0 / locals.var_cnon);
        let assign11860_e15436: f64 = (-37.0);
        let (assign11860_e15463, assign11860_e15463_d_n0, assign11860_e15463_d_n2, assign11860_e15463_d_n3, assign11860_e15463_d_n4, assign11860_e15463_d_n5, assign11860_e15463_d_n6, assign11860_e15463_d_n7, assign11860_e15463_d_n8, assign11860_e15463_d_n9, assign11860_e15463_d_n10, assign11860_e15463_d_n11, assign11860_e15463_d_n13, assign11860_e15463_d_n14,) = {
            if ((!(locals.var_tt1 > 37.0)) && (!(locals.var_tt1 < assign11860_e15436))) {
                let assign11860_e15442: f64 = (locals.var_tt1).exp();
                let assign11860_e15443: f64 = (1.0 + assign11860_e15442);
                let assign11860_e15444: f64 = (assign11860_e15443).ln();
                (assign11860_e15444, ((assign11860_e15442 * locals.var_tt1_dn0) / assign11860_e15443), ((assign11860_e15442 * locals.var_tt1_dn2) / assign11860_e15443), ((assign11860_e15442 * locals.var_tt1_dn3) / assign11860_e15443), ((assign11860_e15442 * locals.var_tt1_dn4) / assign11860_e15443), ((assign11860_e15442 * locals.var_tt1_dn5) / assign11860_e15443), ((assign11860_e15442 * locals.var_tt1_dn6) / assign11860_e15443), ((assign11860_e15442 * locals.var_tt1_dn7) / assign11860_e15443), ((assign11860_e15442 * locals.var_tt1_dn8) / assign11860_e15443), ((assign11860_e15442 * locals.var_tt1_dn9) / assign11860_e15443), ((assign11860_e15442 * locals.var_tt1_dn10) / assign11860_e15443), ((assign11860_e15442 * locals.var_tt1_dn11) / assign11860_e15443), ((assign11860_e15442 * locals.var_tt1_dn13) / assign11860_e15443), ((assign11860_e15442 * locals.var_tt1_dn14) / assign11860_e15443),)
            } else {
                let assign11860_e15451: f64 = (-37.0);
                let (assign11860_e15462, assign11860_e15462_d_n0, assign11860_e15462_d_n2, assign11860_e15462_d_n3, assign11860_e15462_d_n4, assign11860_e15462_d_n5, assign11860_e15462_d_n6, assign11860_e15462_d_n7, assign11860_e15462_d_n8, assign11860_e15462_d_n9, assign11860_e15462_d_n10, assign11860_e15462_d_n11, assign11860_e15462_d_n13, assign11860_e15462_d_n14,) = {
                    if ((!(locals.var_tt1 > 37.0)) && (locals.var_tt1 < assign11860_e15451)) {
                        let assign11860_e15455: f64 = (locals.var_tt1).exp();
                        (assign11860_e15455, (assign11860_e15455 * locals.var_tt1_dn0), (assign11860_e15455 * locals.var_tt1_dn2), (assign11860_e15455 * locals.var_tt1_dn3), (assign11860_e15455 * locals.var_tt1_dn4), (assign11860_e15455 * locals.var_tt1_dn5), (assign11860_e15455 * locals.var_tt1_dn6), (assign11860_e15455 * locals.var_tt1_dn7), (assign11860_e15455 * locals.var_tt1_dn8), (assign11860_e15455 * locals.var_tt1_dn9), (assign11860_e15455 * locals.var_tt1_dn10), (assign11860_e15455 * locals.var_tt1_dn11), (assign11860_e15455 * locals.var_tt1_dn13), (assign11860_e15455 * locals.var_tt1_dn14),)
                    } else {
                        let (assign11860_e15461, assign11860_e15461_d_n0, assign11860_e15461_d_n2, assign11860_e15461_d_n3, assign11860_e15461_d_n4, assign11860_e15461_d_n5, assign11860_e15461_d_n6, assign11860_e15461_d_n7, assign11860_e15461_d_n8, assign11860_e15461_d_n9, assign11860_e15461_d_n10, assign11860_e15461_d_n11, assign11860_e15461_d_n13, assign11860_e15461_d_n14,) = {
                            if (locals.var_tt1 > 37.0) {
                                (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign11860_e15461, assign11860_e15461_d_n0, assign11860_e15461_d_n2, assign11860_e15461_d_n3, assign11860_e15461_d_n4, assign11860_e15461_d_n5, assign11860_e15461_d_n6, assign11860_e15461_d_n7, assign11860_e15461_d_n8, assign11860_e15461_d_n9, assign11860_e15461_d_n10, assign11860_e15461_d_n11, assign11860_e15461_d_n13, assign11860_e15461_d_n14,)
                    }
                };
                (assign11860_e15462, assign11860_e15462_d_n0, assign11860_e15462_d_n2, assign11860_e15462_d_n3, assign11860_e15462_d_n4, assign11860_e15462_d_n5, assign11860_e15462_d_n6, assign11860_e15462_d_n7, assign11860_e15462_d_n8, assign11860_e15462_d_n9, assign11860_e15462_d_n10, assign11860_e15462_d_n11, assign11860_e15462_d_n13, assign11860_e15462_d_n14,)
            }
        };
        let assign11860_e15464: f64 = (assign11860_e15429 * assign11860_e15463);
        (assign11860_e15464, (assign11860_e15429 * assign11860_e15463_d_n0), (assign11860_e15429 * assign11860_e15463_d_n2), (assign11860_e15429 * assign11860_e15463_d_n3), (assign11860_e15429 * assign11860_e15463_d_n4), (assign11860_e15429 * assign11860_e15463_d_n5), (assign11860_e15429 * assign11860_e15463_d_n6), (assign11860_e15429 * assign11860_e15463_d_n7), (assign11860_e15429 * assign11860_e15463_d_n8), (assign11860_e15429 * assign11860_e15463_d_n9), (assign11860_e15429 * assign11860_e15463_d_n10), (assign11860_e15429 * assign11860_e15463_d_n11), (assign11860_e15429 * assign11860_e15463_d_n13), (assign11860_e15429 * assign11860_e15463_d_n14),)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign11860_e15466;
        locals.var_ccg1_dn0 = assign11860_e15466_d_n0;
        locals.var_ccg1_dn2 = assign11860_e15466_d_n2;
        locals.var_ccg1_dn3 = assign11860_e15466_d_n3;
        locals.var_ccg1_dn4 = assign11860_e15466_d_n4;
        locals.var_ccg1_dn5 = assign11860_e15466_d_n5;
        locals.var_ccg1_dn6 = assign11860_e15466_d_n6;
        locals.var_ccg1_dn7 = assign11860_e15466_d_n7;
        locals.var_ccg1_dn8 = assign11860_e15466_d_n8;
        locals.var_ccg1_dn9 = assign11860_e15466_d_n9;
        locals.var_ccg1_dn10 = assign11860_e15466_d_n10;
        locals.var_ccg1_dn11 = assign11860_e15466_d_n11;
        locals.var_ccg1_dn13 = assign11860_e15466_d_n13;
        locals.var_ccg1_dn14 = assign11860_e15466_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign11870_e15484,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11870_e15474: f64 = (locals.var_wg + p.p90);
        let assign11870_e15475: f64 = (locals.var_trsd / assign11870_e15474);
        let assign11870_e15478: f64 = (locals.var_wg + p.p90);
        let assign11870_e15480: f64 = (assign11870_e15478 / locals.var_trsd);
        let assign11870_e15481: f64 = (assign11870_e15475).min(assign11870_e15480);
        let assign11870_e15482: f64 = (0.5 * assign11870_e15481);
        (assign11870_e15482,)
    } else {
        (locals.var_r1cf,)
    }
};
        locals.var_r1cf = assign11870_e15484;
        locals.var_r1cf_rv = 0.0;

        let (assign11880_e15492,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11880_e15490: f64 = (locals.var_hgdelta * locals.var_r1cf);
        (assign11880_e15490,)
    } else {
        (locals.var_rcf,)
    }
};
        locals.var_rcf = assign11880_e15492;
        locals.var_rcf_rv = 0.0;

        let (assign11890_e15541,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11890_e15498: f64 = (locals.var_epssp * 2.0);
        let assign11890_e15500: f64 = (assign11890_e15498 / 3.141592653589793);
        let assign11890_e15504: f64 = (0.5 * 3.141592653589793);
        let assign11890_e15506: f64 = (assign11890_e15504 * locals.var_rcf);
        let assign11890_e15507: f64 = (p.p1087 + assign11890_e15506);
        let assign11890_e15509: f64 = (assign11890_e15507 / p.p1087);
        let (assign11890_e15538,) = {
            if (!(assign11890_e15509 > 1e-38)) {
                let assign11890_e15514: f64 = (-87.498233534);
                (assign11890_e15514,)
            } else {
                let assign11890_e15518: f64 = (0.5 * 3.141592653589793);
                let assign11890_e15520: f64 = (assign11890_e15518 * locals.var_rcf);
                let assign11890_e15521: f64 = (p.p1087 + assign11890_e15520);
                let assign11890_e15523: f64 = (assign11890_e15521 / p.p1087);
                let (assign11890_e15537,) = {
                    if (assign11890_e15523 > 1e-38) {
                        let assign11890_e15529: f64 = (0.5 * 3.141592653589793);
                        let assign11890_e15531: f64 = (assign11890_e15529 * locals.var_rcf);
                        let assign11890_e15532: f64 = (p.p1087 + assign11890_e15531);
                        let assign11890_e15534: f64 = (assign11890_e15532 / p.p1087);
                        let assign11890_e15535: f64 = (assign11890_e15534).ln();
                        (assign11890_e15535,)
                    } else {
                        (0.0,)
                    }
                };
                (assign11890_e15537,)
            }
        };
        let assign11890_e15539: f64 = (assign11890_e15500 * assign11890_e15538);
        (assign11890_e15539,)
    } else {
        (locals.var_ccg2,)
    }
};
        locals.var_ccg2 = assign11890_e15541;
        locals.var_ccg2_rv = 0.0;

        let (assign11900_e15551, assign11900_e15551_d_n0, assign11900_e15551_d_n2, assign11900_e15551_d_n3, assign11900_e15551_d_n4, assign11900_e15551_d_n5, assign11900_e15551_d_n6, assign11900_e15551_d_n7, assign11900_e15551_d_n8, assign11900_e15551_d_n9, assign11900_e15551_d_n10, assign11900_e15551_d_n11, assign11900_e15551_d_n13, assign11900_e15551_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11900_e15548: f64 = (locals.var_ccg1 + locals.var_ccg2);
        let assign11900_e15549: f64 = (p.p40 * assign11900_e15548);
        (assign11900_e15549, (p.p40 * locals.var_ccg1_dn0), (p.p40 * locals.var_ccg1_dn2), (p.p40 * locals.var_ccg1_dn3), (p.p40 * locals.var_ccg1_dn4), (p.p40 * locals.var_ccg1_dn5), (p.p40 * locals.var_ccg1_dn6), (p.p40 * locals.var_ccg1_dn7), (p.p40 * locals.var_ccg1_dn8), (p.p40 * locals.var_ccg1_dn9), (p.p40 * locals.var_ccg1_dn10), (p.p40 * locals.var_ccg1_dn11), (p.p40 * locals.var_ccg1_dn13), (p.p40 * locals.var_ccg1_dn14),)
    } else {
        (locals.var_ccg, locals.var_ccg_dn0, locals.var_ccg_dn2, locals.var_ccg_dn3, locals.var_ccg_dn4, locals.var_ccg_dn5, locals.var_ccg_dn6, locals.var_ccg_dn7, locals.var_ccg_dn8, locals.var_ccg_dn9, locals.var_ccg_dn10, locals.var_ccg_dn11, locals.var_ccg_dn13, locals.var_ccg_dn14,)
    }
};
        locals.var_ccg = assign11900_e15551;
        locals.var_ccg_dn0 = assign11900_e15551_d_n0;
        locals.var_ccg_dn2 = assign11900_e15551_d_n2;
        locals.var_ccg_dn3 = assign11900_e15551_d_n3;
        locals.var_ccg_dn4 = assign11900_e15551_d_n4;
        locals.var_ccg_dn5 = assign11900_e15551_d_n5;
        locals.var_ccg_dn6 = assign11900_e15551_d_n6;
        locals.var_ccg_dn7 = assign11900_e15551_d_n7;
        locals.var_ccg_dn8 = assign11900_e15551_d_n8;
        locals.var_ccg_dn9 = assign11900_e15551_d_n9;
        locals.var_ccg_dn10 = assign11900_e15551_d_n10;
        locals.var_ccg_dn11 = assign11900_e15551_d_n11;
        locals.var_ccg_dn13 = assign11900_e15551_d_n13;
        locals.var_ccg_dn14 = assign11900_e15551_d_n14;
        locals.var_ccg_rv = 0.0;

        let (assign11910_e15559,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11910_e15557: f64 = (locals.var_lmax / locals.var_wg);
        (assign11910_e15557,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign11910_e15559;
        locals.var_x_rv = 0.0;

        let (assign11920_e15574,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11920_e15567: f64 = (locals.var_x + 1.0);
        let assign11920_e15568: f64 = (2.0 * assign11920_e15567);
        let assign11920_e15569: f64 = (assign11920_e15568).sqrt();
        let assign11920_e15571: f64 = (assign11920_e15569 * 3.141592653589793);
        let assign11920_e15572: f64 = (4.0 / assign11920_e15571);
        (assign11920_e15572,)
    } else {
        (locals.var_c1,)
    }
};
        locals.var_c1 = assign11920_e15574;
        locals.var_c1_rv = 0.0;

        let (assign11930_e15610,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11930_e15580: f64 = (p.p90 * p.p90);
        let assign11930_e15583: f64 = (2.0 * locals.var_wg);
        let assign11930_e15585: f64 = (assign11930_e15583 * p.p90);
        let assign11930_e15586: f64 = (assign11930_e15580 + assign11930_e15585);
        let assign11930_e15589: f64 = (locals.var_wg * locals.var_wg);
        let assign11930_e15592: f64 = (locals.var_x + 1.0);
        let assign11930_e15593: f64 = (assign11930_e15589 * assign11930_e15592);
        let assign11930_e15594: f64 = (assign11930_e15586 + assign11930_e15593);
        let assign11930_e15595: f64 = (assign11930_e15594).sqrt();
        let assign11930_e15598: f64 = (locals.var_x + 1.0);
        let assign11930_e15599: f64 = (assign11930_e15598).sqrt();
        let assign11930_e15600: f64 = (assign11930_e15595 * assign11930_e15599);
        let assign11930_e15602: f64 = (assign11930_e15600 + p.p90);
        let assign11930_e15605: f64 = (locals.var_wg * locals.var_x);
        let assign11930_e15606: f64 = (assign11930_e15602 + assign11930_e15605);
        let assign11930_e15608: f64 = (assign11930_e15606 + locals.var_wg);
        (assign11930_e15608,)
    } else {
        (locals.var_c2,)
    }
};
        locals.var_c2 = assign11930_e15610;
        locals.var_c2_rv = 0.0;

        let (assign11940_e15631,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11940_e15617: f64 = (locals.var_x + 1.0);
        let assign11940_e15620: f64 = (locals.var_x + 4.0);
        let assign11940_e15621: f64 = (assign11940_e15617 * assign11940_e15620);
        let assign11940_e15622: f64 = (assign11940_e15621).sqrt();
        let assign11940_e15623: f64 = (p.p90 * assign11940_e15622);
        let assign11940_e15627: f64 = (locals.var_x + 2.0);
        let assign11940_e15628: f64 = (p.p90 * assign11940_e15627);
        let assign11940_e15629: f64 = (assign11940_e15623 + assign11940_e15628);
        (assign11940_e15629,)
    } else {
        (locals.var_c3,)
    }
};
        locals.var_c3 = assign11940_e15631;
        locals.var_c3_rv = 0.0;

        let (assign11950_e15662,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11950_e15639: f64 = (locals.var_c2 / locals.var_c3);
        let (assign11950_e15656,) = {
            if (!(assign11950_e15639 > 1e-38)) {
                let assign11950_e15644: f64 = (-87.498233534);
                (assign11950_e15644,)
            } else {
                let assign11950_e15647: f64 = (locals.var_c2 / locals.var_c3);
                let (assign11950_e15655,) = {
                    if (assign11950_e15647 > 1e-38) {
                        let assign11950_e15652: f64 = (locals.var_c2 / locals.var_c3);
                        let assign11950_e15653: f64 = (assign11950_e15652).ln();
                        (assign11950_e15653,)
                    } else {
                        (0.0,)
                    }
                };
                (assign11950_e15655,)
            }
        };
        let assign11950_e15657: f64 = (locals.var_c1 * assign11950_e15656);
        let assign11950_e15659: f64 = (assign11950_e15657 + 12.27);
        let assign11950_e15660: f64 = (locals.var_epssp * assign11950_e15659);
        (assign11950_e15660,)
    } else {
        (locals.var_cfglog,)
    }
};
        locals.var_cfglog = assign11950_e15662;
        locals.var_cfglog_rv = 0.0;

        let (assign11960_e15670,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11960_e15668: f64 = (locals.var_hr * locals.var_lr);
        (assign11960_e15668,)
    } else {
        (locals.var_dcf,)
    }
};
        locals.var_dcf = assign11960_e15670;
        locals.var_dcf_rv = 0.0;

        let (assign11970_e15681,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11970_e15676: f64 = (locals.var_dcf * locals.var_dcf);
        let assign11970_e15678: f64 = (assign11970_e15676 + 1.0);
        let assign11970_e15679: f64 = (assign11970_e15678).sqrt();
        (assign11970_e15679,)
    } else {
        (locals.var_tt0,)
    }
};
        locals.var_tt0 = assign11970_e15681;
        locals.var_tt0_rv = 0.0;

        let (assign11980_e15730, assign11980_e15730_d_n0, assign11980_e15730_d_n2, assign11980_e15730_d_n3, assign11980_e15730_d_n4, assign11980_e15730_d_n5, assign11980_e15730_d_n6, assign11980_e15730_d_n7, assign11980_e15730_d_n8, assign11980_e15730_d_n9, assign11980_e15730_d_n10, assign11980_e15730_d_n11, assign11980_e15730_d_n13, assign11980_e15730_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11980_e15687: f64 = (locals.var_dcf * locals.var_dcf);
        let assign11980_e15689: f64 = (assign11980_e15687 + 1.0);
        let assign11980_e15692: f64 = (locals.var_dcf * p.p90);
        let assign11980_e15695: f64 = (locals.var_dcf * p.p90);
        let assign11980_e15696: f64 = (assign11980_e15692 * assign11980_e15695);
        let assign11980_e15699: f64 = (2.0 * locals.var_dcf);
        let assign11980_e15701: f64 = (assign11980_e15699 * locals.var_lmax);
        let assign11980_e15703: f64 = (assign11980_e15701 * p.p90);
        let assign11980_e15704: f64 = (assign11980_e15696 + assign11980_e15703);
        let assign11980_e15707: f64 = (locals.var_dcf * locals.var_dcf);
        let assign11980_e15709: f64 = (assign11980_e15707 + 1.0);
        let assign11980_e15711: f64 = (assign11980_e15709 * locals.var_lmax);
        let assign11980_e15713: f64 = (assign11980_e15711 * locals.var_lmax);
        let assign11980_e15714: f64 = (assign11980_e15704 + assign11980_e15713);
        let assign11980_e15715: f64 = (assign11980_e15689 * assign11980_e15714);
        let assign11980_e15716: f64 = (assign11980_e15715).sqrt();
        let assign11980_e15719: f64 = (locals.var_dcf * p.p90);
        let assign11980_e15720: f64 = (assign11980_e15716 + assign11980_e15719);
        let assign11980_e15723: f64 = (locals.var_dcf * locals.var_dcf);
        let assign11980_e15725: f64 = (assign11980_e15723 * locals.var_lmax);
        let assign11980_e15726: f64 = (assign11980_e15720 + assign11980_e15725);
        let assign11980_e15728: f64 = (assign11980_e15726 + locals.var_lmax);
        (assign11980_e15728, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign11980_e15730;
        locals.var_tt1_dn0 = assign11980_e15730_d_n0;
        locals.var_tt1_dn2 = assign11980_e15730_d_n2;
        locals.var_tt1_dn3 = assign11980_e15730_d_n3;
        locals.var_tt1_dn4 = assign11980_e15730_d_n4;
        locals.var_tt1_dn5 = assign11980_e15730_d_n5;
        locals.var_tt1_dn6 = assign11980_e15730_d_n6;
        locals.var_tt1_dn7 = assign11980_e15730_d_n7;
        locals.var_tt1_dn8 = assign11980_e15730_d_n8;
        locals.var_tt1_dn9 = assign11980_e15730_d_n9;
        locals.var_tt1_dn10 = assign11980_e15730_d_n10;
        locals.var_tt1_dn11 = assign11980_e15730_d_n11;
        locals.var_tt1_dn13 = assign11980_e15730_d_n13;
        locals.var_tt1_dn14 = assign11980_e15730_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign11990_e15742,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign11990_e15736: f64 = (locals.var_tt0 + 1.0);
        let assign11990_e15739: f64 = (locals.var_dcf * p.p90);
        let assign11990_e15740: f64 = (assign11990_e15736 * assign11990_e15739);
        (assign11990_e15740,)
    } else {
        (locals.var_tt2,)
    }
};
        locals.var_tt2 = assign11990_e15742;
        locals.var_tt2_rv = 0.0;

    }
}

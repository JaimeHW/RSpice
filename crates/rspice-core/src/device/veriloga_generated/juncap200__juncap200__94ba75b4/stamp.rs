#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

#[inline(always)]
fn rspice_limexp(x: f64) -> f64 {
    if x < 80.0 { x.exp() } else { (80.0f64).exp() * (x - 80.0 + 1.0) }
}

#[inline(always)]
fn rspice_limited_exp(x: f64) -> f64 {
    if x > 80.0 {
        5.54062238439351e34 * (x - 80.0 + 1.0)
    } else if x < -80.0 {
        1.804851387e-35
    } else {
        x.exp()
    }
}

#[inline(always)]
fn rspice_limited_exp_derivative(x: f64) -> f64 {
    if x > 80.0 {
        5.54062238439351e34
    } else if x < -80.0 {
        0.0
    } else {
        x.exp()
    }
}

/// A packed derivative: one partial per unknown the value can reach.
///
/// A newtype rather than a bare `[f64; N]` so the elementwise rules emit as
/// `a + b` and `a * s` instead of named calls. That is not cosmetic — these
/// operations are most of a large model's generated source, and an operator is
/// a dozen characters shorter than a call at every one of them.
#[derive(Clone, Copy)]
struct Lanes<const N: usize>([f64; N]);

impl<const N: usize> core::ops::Add for Lanes<N> {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] + rhs.0[i];
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Sub for Lanes<N> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] - rhs.0[i];
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Mul<f64> for Lanes<N> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] * rhs;
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Div<f64> for Lanes<N> {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] / rhs;
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Index<usize> for Lanes<N> {
    type Output = f64;
    #[inline(always)]
    fn index(&self, index: usize) -> &f64 {
        &self.0[index]
    }
}

#[inline]
fn rspice_eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    active: bool,
    scale: f64,
    previous_value_scale: f64,
    older_value_scale: f64,
    previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    let older_value = if initialized[slot] { older[slot] } else { value };
    current[slot] = value;
    if active {
        let result = value * scale
            - previous_value * previous_value_scale
            - older_value * older_value_scale
            - derivative_previous[slot] * previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
}

impl Instance {
    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 333] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = parameters[62];
                let v1 = 5e-1f64;
                let v3 = 1e0f64;
                let v4 = 0e0f64;
                let v6 = 2.7315e2f64;
                let v7 = parameters[13];
                let v9 = 8.61726105451295e-5f64;
                let v12 = 7.02e-4f64;
                let v16 = 1.108e3f64;
                let v19 = parameters[24];
                let v21 = parameters[25];
                let v23 = parameters[26];
                let v25 = parameters[21];
                let v27 = parameters[22];
                let v29 = parameters[23];
                let v34 = 1.0447941624768001e-10f64;
                let v35 = parameters[15];
                let v37 = parameters[33];
                let v39 = parameters[16];
                let v41 = parameters[34];
                let v43 = parameters[17];
                let v48 = parameters[18];
                let v50 = parameters[19];
                let v52 = parameters[20];
                let v54 = parameters[14];
                let v57 = parameters[53];
                let v61 = parameters[54];
                let v65 = parameters[55];
                let v69 = parameters[50];
                let v71 = parameters[51];
                let v73 = parameters[52];
                let v96 = parameters[56];
                let v98 = parameters[57];
                let v101 = parameters[58];
                let v104 = parameters[59];
                let v110 = 1e-18f64;
                let v112 = 0e0f64;
                let v113 = 0e0f64;
                let v114 = 0e0f64;
                let v115 = 0e0f64;
                let v116 = 0e0f64;
                let v117 = 0e0f64;
                let v118 = 0e0f64;
                let v129 = 3.2e1f64;
                let v130 = parameters[38];
                let v132 = 9.1093826e-31f64;
                let v134 = 1.6021918e-19f64;
                let v136 = parameters[39];
                let v140 = parameters[40];
                let v146 = 5e-2f64;
                let v152 = 9.5e-1f64;
                let v161 = parameters[3];
                let v164 = parameters[4];
                let v167 = parameters[5];
                let v170 = parameters[6];
                let v175 = 9e-1f64;
                let v194 = -1e0f64;
                let v196 = 2e0f64;
                let v203 = -4e-1f64;
                let v204 = parameters[63];
                let v206 = -6.5e-1f64;
                let v208 = -8e-1f64;
                let v220 = 4e0f64;
                let v228 = 4e-12f64;
                let v233 = parameters[30];
                let v235 = parameters[35];
                let v243 = -1e0f64;
                let v245 = parameters[41];
                let v248 = 1e3f64;
                let v283 = parameters[31];
                let v285 = parameters[36];
                let v293 = -1e0f64;
                let v295 = parameters[42];
                let v332 = parameters[32];
                let v334 = parameters[37];
                let v342 = -1e0f64;
                let v344 = parameters[43];
                let v395 = 4e-12f64;
                let v408 = -1e0f64;
                let v454 = -1e0f64;
                let v500 = -1e0f64;
                let v552 = 4e-12f64;
                let v565 = -1e0f64;
                let v611 = -1e0f64;
                let v657 = -1e0f64;
                let v697 = 1e-1f64;
                let v707 = -1.000000082740371e-11f64;
                let v716 = -1e0f64;
                let v762 = -1e0f64;
                let v808 = -1e0f64;
                let v848 = 2e-1f64;
                let v858 = -5.000000413701855e-12f64;
                let v867 = -1e0f64;
                let v913 = -1e0f64;
                let v959 = -1e0f64;
                let v997 = parameters[12];
                let v1002 = parameters[1];
                let v1004 = parameters[7];
                let v1006 = parameters[8];
                let v1016 = 1e0f64;
                let v1020 = -1e0f64;
                let v1040 = -1e0f64;
                let v1060 = -1e0f64;
                let mut out111: f64 = 0.0;
                let mut out147: f64 = 0.0;
                let mut out150: f64 = 0.0;
                let mut out153: f64 = 0.0;
                let mut out160: f64 = 0.0;
                let mut out205: f64 = 0.0;
                let mut out207: f64 = 0.0;
                let mut out209: f64 = 0.0;
                let mut out212: f64 = 0.0;
                let mut out215: f64 = 0.0;
                let mut out216: f64 = 0.0;
                let mut out236: f64 = 0.0;
                let mut out237: f64 = 0.0;
                let mut out238: f64 = 0.0;
                let mut out240: f64 = 0.0;
                let mut out242: f64 = 0.0;
                let mut out244: f64 = 0.0;
                let mut out246: f64 = 0.0;
                let mut out247: f64 = 0.0;
                let mut out249: f64 = 0.0;
                let mut out260: f64 = 0.0;
                let mut out262: f64 = 0.0;
                let mut out265: f64 = 0.0;
                let mut out266: f64 = 0.0;
                let mut out267: f64 = 0.0;
                let mut out286: f64 = 0.0;
                let mut out287: f64 = 0.0;
                let mut out288: f64 = 0.0;
                let mut out290: f64 = 0.0;
                let mut out292: f64 = 0.0;
                let mut out294: f64 = 0.0;
                let mut out296: f64 = 0.0;
                let mut out297: f64 = 0.0;
                let mut out298: f64 = 0.0;
                let mut out309: f64 = 0.0;
                let mut out311: f64 = 0.0;
                let mut out314: f64 = 0.0;
                let mut out315: f64 = 0.0;
                let mut out316: f64 = 0.0;
                let mut out335: f64 = 0.0;
                let mut out336: f64 = 0.0;
                let mut out337: f64 = 0.0;
                let mut out339: f64 = 0.0;
                let mut out341: f64 = 0.0;
                let mut out343: f64 = 0.0;
                let mut out345: f64 = 0.0;
                let mut out346: f64 = 0.0;
                let mut out347: f64 = 0.0;
                let mut out358: f64 = 0.0;
                let mut out360: f64 = 0.0;
                let mut out363: f64 = 0.0;
                let mut out364: f64 = 0.0;
                let mut out365: f64 = 0.0;
                let mut out383: f64 = 0.0;
                let mut out384: f64 = 0.0;
                let mut out401: f64 = 0.0;
                let mut out402: f64 = 0.0;
                let mut out403: f64 = 0.0;
                let mut out405: f64 = 0.0;
                let mut out407: f64 = 0.0;
                let mut out409: f64 = 0.0;
                let mut out410: f64 = 0.0;
                let mut out411: f64 = 0.0;
                let mut out412: f64 = 0.0;
                let mut out423: f64 = 0.0;
                let mut out425: f64 = 0.0;
                let mut out428: f64 = 0.0;
                let mut out429: f64 = 0.0;
                let mut out430: f64 = 0.0;
                let mut out447: f64 = 0.0;
                let mut out448: f64 = 0.0;
                let mut out449: f64 = 0.0;
                let mut out451: f64 = 0.0;
                let mut out453: f64 = 0.0;
                let mut out455: f64 = 0.0;
                let mut out456: f64 = 0.0;
                let mut out457: f64 = 0.0;
                let mut out458: f64 = 0.0;
                let mut out469: f64 = 0.0;
                let mut out471: f64 = 0.0;
                let mut out474: f64 = 0.0;
                let mut out475: f64 = 0.0;
                let mut out476: f64 = 0.0;
                let mut out493: f64 = 0.0;
                let mut out494: f64 = 0.0;
                let mut out495: f64 = 0.0;
                let mut out497: f64 = 0.0;
                let mut out499: f64 = 0.0;
                let mut out501: f64 = 0.0;
                let mut out502: f64 = 0.0;
                let mut out503: f64 = 0.0;
                let mut out504: f64 = 0.0;
                let mut out515: f64 = 0.0;
                let mut out517: f64 = 0.0;
                let mut out520: f64 = 0.0;
                let mut out521: f64 = 0.0;
                let mut out522: f64 = 0.0;
                let mut out540: f64 = 0.0;
                let mut out541: f64 = 0.0;
                let mut out558: f64 = 0.0;
                let mut out559: f64 = 0.0;
                let mut out560: f64 = 0.0;
                let mut out562: f64 = 0.0;
                let mut out564: f64 = 0.0;
                let mut out566: f64 = 0.0;
                let mut out567: f64 = 0.0;
                let mut out568: f64 = 0.0;
                let mut out569: f64 = 0.0;
                let mut out580: f64 = 0.0;
                let mut out582: f64 = 0.0;
                let mut out585: f64 = 0.0;
                let mut out586: f64 = 0.0;
                let mut out587: f64 = 0.0;
                let mut out604: f64 = 0.0;
                let mut out605: f64 = 0.0;
                let mut out606: f64 = 0.0;
                let mut out608: f64 = 0.0;
                let mut out610: f64 = 0.0;
                let mut out612: f64 = 0.0;
                let mut out613: f64 = 0.0;
                let mut out614: f64 = 0.0;
                let mut out615: f64 = 0.0;
                let mut out626: f64 = 0.0;
                let mut out628: f64 = 0.0;
                let mut out631: f64 = 0.0;
                let mut out632: f64 = 0.0;
                let mut out633: f64 = 0.0;
                let mut out650: f64 = 0.0;
                let mut out651: f64 = 0.0;
                let mut out652: f64 = 0.0;
                let mut out654: f64 = 0.0;
                let mut out656: f64 = 0.0;
                let mut out658: f64 = 0.0;
                let mut out659: f64 = 0.0;
                let mut out660: f64 = 0.0;
                let mut out661: f64 = 0.0;
                let mut out672: f64 = 0.0;
                let mut out674: f64 = 0.0;
                let mut out677: f64 = 0.0;
                let mut out678: f64 = 0.0;
                let mut out679: f64 = 0.0;
                let mut out709: f64 = 0.0;
                let mut out710: f64 = 0.0;
                let mut out711: f64 = 0.0;
                let mut out713: f64 = 0.0;
                let mut out715: f64 = 0.0;
                let mut out717: f64 = 0.0;
                let mut out718: f64 = 0.0;
                let mut out719: f64 = 0.0;
                let mut out720: f64 = 0.0;
                let mut out731: f64 = 0.0;
                let mut out733: f64 = 0.0;
                let mut out736: f64 = 0.0;
                let mut out737: f64 = 0.0;
                let mut out738: f64 = 0.0;
                let mut out755: f64 = 0.0;
                let mut out756: f64 = 0.0;
                let mut out757: f64 = 0.0;
                let mut out759: f64 = 0.0;
                let mut out761: f64 = 0.0;
                let mut out763: f64 = 0.0;
                let mut out764: f64 = 0.0;
                let mut out765: f64 = 0.0;
                let mut out766: f64 = 0.0;
                let mut out777: f64 = 0.0;
                let mut out779: f64 = 0.0;
                let mut out782: f64 = 0.0;
                let mut out783: f64 = 0.0;
                let mut out784: f64 = 0.0;
                let mut out801: f64 = 0.0;
                let mut out802: f64 = 0.0;
                let mut out803: f64 = 0.0;
                let mut out805: f64 = 0.0;
                let mut out807: f64 = 0.0;
                let mut out809: f64 = 0.0;
                let mut out810: f64 = 0.0;
                let mut out811: f64 = 0.0;
                let mut out812: f64 = 0.0;
                let mut out823: f64 = 0.0;
                let mut out825: f64 = 0.0;
                let mut out828: f64 = 0.0;
                let mut out829: f64 = 0.0;
                let mut out830: f64 = 0.0;
                let mut out860: f64 = 0.0;
                let mut out861: f64 = 0.0;
                let mut out862: f64 = 0.0;
                let mut out864: f64 = 0.0;
                let mut out866: f64 = 0.0;
                let mut out868: f64 = 0.0;
                let mut out869: f64 = 0.0;
                let mut out870: f64 = 0.0;
                let mut out871: f64 = 0.0;
                let mut out882: f64 = 0.0;
                let mut out884: f64 = 0.0;
                let mut out887: f64 = 0.0;
                let mut out888: f64 = 0.0;
                let mut out889: f64 = 0.0;
                let mut out906: f64 = 0.0;
                let mut out907: f64 = 0.0;
                let mut out908: f64 = 0.0;
                let mut out910: f64 = 0.0;
                let mut out912: f64 = 0.0;
                let mut out914: f64 = 0.0;
                let mut out915: f64 = 0.0;
                let mut out916: f64 = 0.0;
                let mut out917: f64 = 0.0;
                let mut out928: f64 = 0.0;
                let mut out930: f64 = 0.0;
                let mut out933: f64 = 0.0;
                let mut out934: f64 = 0.0;
                let mut out935: f64 = 0.0;
                let mut out952: f64 = 0.0;
                let mut out953: f64 = 0.0;
                let mut out954: f64 = 0.0;
                let mut out956: f64 = 0.0;
                let mut out958: f64 = 0.0;
                let mut out960: f64 = 0.0;
                let mut out961: f64 = 0.0;
                let mut out962: f64 = 0.0;
                let mut out963: f64 = 0.0;
                let mut out974: f64 = 0.0;
                let mut out976: f64 = 0.0;
                let mut out979: f64 = 0.0;
                let mut out980: f64 = 0.0;
                let mut out981: f64 = 0.0;
                let mut out998: f64 = 0.0;
                let mut out1001: f64 = 0.0;
                let mut out1009: f64 = 0.0;
                let mut out1011: f64 = 0.0;
                let mut out1012: f64 = 0.0;
                let mut out1013: f64 = 0.0;
                let mut out1015: f64 = 0.0;
                let mut out1017: f64 = 0.0;
                let mut out1019: f64 = 0.0;
                let mut out1021: f64 = 0.0;
                let mut out1022: f64 = 0.0;
                let mut out1023: f64 = 0.0;
                let mut out1024: f64 = 0.0;
                let mut out1025: f64 = 0.0;
                let mut out1026: f64 = 0.0;
                let mut out1028: f64 = 0.0;
                let mut out1029: f64 = 0.0;
                let mut out1030: f64 = 0.0;
                let mut out1032: f64 = 0.0;
                let mut out1033: f64 = 0.0;
                let mut out1034: f64 = 0.0;
                let mut out1036: f64 = 0.0;
                let mut out1037: f64 = 0.0;
                let mut out1039: f64 = 0.0;
                let mut out1041: f64 = 0.0;
                let mut out1042: f64 = 0.0;
                let mut out1043: f64 = 0.0;
                let mut out1044: f64 = 0.0;
                let mut out1045: f64 = 0.0;
                let mut out1046: f64 = 0.0;
                let mut out1048: f64 = 0.0;
                let mut out1049: f64 = 0.0;
                let mut out1050: f64 = 0.0;
                let mut out1052: f64 = 0.0;
                let mut out1053: f64 = 0.0;
                let mut out1054: f64 = 0.0;
                let mut out1056: f64 = 0.0;
                let mut out1057: f64 = 0.0;
                let mut out1059: f64 = 0.0;
                let mut out1061: f64 = 0.0;
                let mut out1062: f64 = 0.0;
                let mut out1063: f64 = 0.0;
                let mut out1064: f64 = 0.0;
                let mut out1065: f64 = 0.0;
                let mut out1066: f64 = 0.0;
                let mut out1068: f64 = 0.0;
                let mut out1069: f64 = 0.0;
                let mut out1070: f64 = 0.0;
                let mut out1071: f64 = 0.0;
                let mut out1072: f64 = 0.0;
                let mut out1073: f64 = 0.0;
                let mut out1074: f64 = 0.0;
                let v2 = if v0 > v1 { 1.0 } else { 0.0 };
                let v5: f64;
                if v2 != 0.0 {
                    v5 = v3;
                } else {
                    v5 = v4;
                }
                let v8 = v6 + v7;
                let v10 = v9 * v8;
                let v11 = v3 / v10;
                let v18 = (-((v12 * v8) * v8)) / (v16 + v8);
                let v20 = v19 + v18;
                let v22 = v21 + v18;
                let v24 = v23 + v18;
                let v26 = v3 - v25;
                let v28 = v3 - v27;
                let v30 = v3 - v29;
                let v31 = v3 / v26;
                let v32 = v3 / v28;
                let v33 = v3 / v30;
                let v36 = v34 / v35;
                let v40 = (v37 * v34) / v39;
                let v44 = (v41 * v34) / v43;
                let v45 = v3 / v36;
                let v46 = v3 / v40;
                let v47 = v3 / v44;
                let v49 = v3 / v48;
                let v51 = v3 / v50;
                let v53 = v3 / v52;
                let v56 = v3 - (v3 / v54);
                let v60 = v3 / (v3 - (v56.powf(v57)));
                let v64 = v3 / (v3 - (v56.powf(v61)));
                let v68 = v3 / (v3 - (v56.powf(v65)));
                let v70 = v3 / v69;
                let v72 = v3 / v71;
                let v74 = v3 / v73;
                let v81 = ((-((v60 * v60) * (v56.powf((v57 - v3))))) * v57) * v70;
                let v88 = ((-((v64 * v64) * (v56.powf((v61 - v3))))) * v61) * v72;
                let v95 = ((-((v68 * v68) * (v56.powf((v65 - v3))))) * v65) * v74;
                let v106 = if (if (if (if v96 != v3 { 1.0 } else { 0.0 }) != 0.0 || (if v98 != v3 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v101 != v3 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v104 != v3 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v107: f64;
                if v106 != 0.0 {
                    v107 = v3;
                } else {
                    v107 = v4;
                }
                let v108 = if v107 == v3 { 1.0 } else { 0.0 };
                let v119: f64;
                let v120: f64;
                let v121: f64;
                let v122: f64;
                let v123: f64;
                let v124: f64;
                let v125: f64;
                if v108 != 0.0 {
                    let v109 = v43 * v96;
                    let v111 = if v109 > v110 { 1.0 } else { 0.0 };
                    out111 = v111;
                    let v144: f64;
                    if v111 != 0.0 {
                        v144 = v109;
                    } else {
                        v144 = v110;
                    }
                    let v145 = v52 * v98;
                    let v147 = if v145 > v146 { 1.0 } else { 0.0 };
                    out147 = v147;
                    let v148: f64;
                    if v147 != 0.0 {
                        v148 = v145;
                    } else {
                        v148 = v146;
                    }
                    let v149 = v29 * v101;
                    let v150 = if v149 > v146 { 1.0 } else { 0.0 };
                    out150 = v150;
                    let v151: f64;
                    if v150 != 0.0 {
                        v151 = v149;
                    } else {
                        v151 = v146;
                    }
                    let v153 = if v151 < v152 { 1.0 } else { 0.0 };
                    out153 = v153;
                    let v154: f64;
                    if v153 != 0.0 {
                        let v159: f64;
                        if v150 != 0.0 {
                            v159 = v149;
                        } else {
                            v159 = v146;
                        }
                        v154 = v159;
                    } else {
                        v154 = v152;
                    }
                    let v155 = v23 * v104;
                    let v156 = v155 + v18;
                    let v157 = v3 - v154;
                    let v158 = v3 / v157;
                    v119 = v155;
                    v120 = v156;
                    v121 = v148;
                    v122 = v144;
                    v123 = v154;
                    v124 = v158;
                    v125 = v157;
                } else {
                    v119 = v112;
                    v120 = v113;
                    v121 = v114;
                    v122 = v115;
                    v123 = v116;
                    v124 = v117;
                    v125 = v118;
                }
                let v126 = v20 * v11;
                let v127 = v22 * v11;
                let v128 = v24 * v11;
                let v135 = ((v129 * v130) * v132) * v134;
                let v139 = ((v129 * v136) * v132) * v134;
                let v143 = ((v129 * v140) * v132) * v134;
                if v108 != 0.0 {
                    let v160 = v120 * v11;
                    out160 = v160;
                } else {
                }
                let v162 = if v161 > v4 { 1.0 } else { 0.0 };
                let v163: f64;
                if v162 != 0.0 {
                    v163 = v161;
                } else {
                    v163 = v4;
                }
                let v165 = if v164 > v4 { 1.0 } else { 0.0 };
                let v166: f64;
                if v165 != 0.0 {
                    v166 = v164;
                } else {
                    v166 = v4;
                }
                let v168 = if v167 > v4 { 1.0 } else { 0.0 };
                let v169: f64;
                if v168 != 0.0 {
                    v169 = v167;
                } else {
                    v169 = v4;
                }
                let v171 = if v170 > v4 { 1.0 } else { 0.0 };
                let v172: f64;
                if v171 != 0.0 {
                    v172 = v170;
                } else {
                    v172 = v4;
                }
                let v173 = if v163 == v4 { 1.0 } else { 0.0 };
                let v178: f64;
                let v179: f64;
                if v173 != 0.0 {
                    let v176 = v175 * (if v27 <= v29 { v27 } else { v29 });
                    let v177 = v50 + v52;
                    v178 = v176;
                    v179 = v177;
                } else {
                    v178 = v25;
                    v179 = v48;
                }
                let v180 = if v166 == v4 { 1.0 } else { 0.0 };
                let v184: f64;
                let v185: f64;
                if v180 != 0.0 {
                    let v182 = v175 * (if v25 <= v29 { v25 } else { v29 });
                    let v183 = v48 + v52;
                    v184 = v182;
                    v185 = v183;
                } else {
                    v184 = v27;
                    v185 = v50;
                }
                let v186 = if v169 == v4 { 1.0 } else { 0.0 };
                let v190: f64;
                let v191: f64;
                if v186 != 0.0 {
                    let v188 = v175 * (if v25 <= v27 { v25 } else { v27 });
                    let v189 = v48 + v50;
                    v190 = v188;
                    v191 = v189;
                } else {
                    v190 = v29;
                    v191 = v52;
                }
                let v198 = v3 - (v196.powf((v194 / (if (if v178 >= v184 { v178 } else { v184 }) >= v190 { (if v178 >= v184 { v178 } else { v184 }) } else { v190 }))));
                let v201 = (if (if v179 <= v185 { v179 } else { v185 }) <= v191 { (if v179 <= v185 { v179 } else { v185 }) } else { v191 }) - v146;
                let v202 = if v5 == v3 { 1.0 } else { 0.0 };
                if v202 != 0.0 {
                    let v205 = v203 * v204;
                    out205 = v205;
                    let v207 = v206 * v204;
                    out207 = v207;
                    let v209 = v208 * v204;
                    out209 = v209;
                    let v212 = if (if (if v173 != 0.0 && v180 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v186 != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                    out212 = v212;
                    let v213: f64;
                    let v214: f64;
                    if v212 != 0.0 {
                        let v215 = if v205 > v4 { 1.0 } else { 0.0 };
                        out215 = v215;
                        if v215 != 0.0 {
                        } else {
                            let v216 = -v205;
                            out216 = v216;
                        }
                        let v218 = v205 - v201;
                        let v226 = v1 * ((v205 + v201) - (((v218 * v218) + ((v220 * v10) * v10)).sqrt()));
                        let v232 = v1 * (v205 - (((v205 * v205) + v228).sqrt()));
                        v213 = v226;
                        v214 = v232;
                    } else {
                        v213 = v4;
                        v214 = v4;
                    }
                    if v173 != 0.0 {
                    } else {
                        let v236 = if v235 == v4 { 1.0 } else { 0.0 };
                        out236 = v236;
                        let v237 = if (if v233 == v4 { 1.0 } else { 0.0 }) != 0.0 && v236 != 0.0 { 1.0 } else { 0.0 };
                        out237 = v237;
                        if v237 != 0.0 {
                        } else {
                            let v238 = if v25 == v1 { 1.0 } else { 0.0 };
                            out238 = v238;
                            if v238 != 0.0 {
                            } else {
                                let v240 = v3 - (v196 * v25);
                                out240 = v240;
                            }
                        }
                        if v236 != 0.0 {
                        } else {
                            let v242 = (-v25) * v31;
                            out242 = v242;
                            let v244 = if v242 == v243 { 1.0 } else { 0.0 };
                            out244 = v244;
                        }
                        let v246 = if v245 == v4 { 1.0 } else { 0.0 };
                        out246 = v246;
                        if v246 != 0.0 {
                        } else {
                            let v247 = if v25 == v1 { 1.0 } else { 0.0 };
                            out247 = v247;
                            let v256: f64;
                            if v247 != 0.0 {
                                let v252 = ((v48 - v213) * v49).sqrt();
                                v256 = v252;
                            } else {
                                let v255 = ((v48 - v213) * v49).powf(v25);
                                v256 = v255;
                            }
                            let v260 = v31 * (((v48 - v213) * v45) / v256);
                            out260 = v260;
                            let v262 = (v205 * v260) * v260;
                            out262 = v262;
                        }
                        let v249 = if v69 > v248 { 1.0 } else { 0.0 };
                        out249 = v249;
                        let v266: f64;
                        if v249 != 0.0 {
                            v266 = v3;
                        } else {
                            let v265 = if v214 > ((-v56) * v69) { 1.0 } else { 0.0 };
                            out265 = v265;
                            let v272: f64;
                            if v265 != 0.0 {
                                let v267 = if v57 == v220 { 1.0 } else { 0.0 };
                                out267 = v267;
                                let v280: f64;
                                if v267 != 0.0 {
                                    let v273 = v214 * v70;
                                    let v276 = ((v273 * v273) * v273) * v273;
                                    v280 = v276;
                                } else {
                                    let v279 = ((v214 * v70).abs()).powf(v57);
                                    v280 = v279;
                                }
                                let v282 = v3 / (v3 - v280);
                                v272 = v282;
                            } else {
                                let v271 = v60 + ((v214 + (v56 * v69)) * v81);
                                v272 = v271;
                            }
                            v266 = v272;
                        }
                        out266 = v266;
                    }
                    if v180 != 0.0 {
                    } else {
                        let v286 = if v285 == v4 { 1.0 } else { 0.0 };
                        out286 = v286;
                        let v287 = if (if v283 == v4 { 1.0 } else { 0.0 }) != 0.0 && v286 != 0.0 { 1.0 } else { 0.0 };
                        out287 = v287;
                        if v287 != 0.0 {
                        } else {
                            let v288 = if v27 == v1 { 1.0 } else { 0.0 };
                            out288 = v288;
                            if v288 != 0.0 {
                            } else {
                                let v290 = v3 - (v196 * v27);
                                out290 = v290;
                            }
                        }
                        if v286 != 0.0 {
                        } else {
                            let v292 = (-v27) * v32;
                            out292 = v292;
                            let v294 = if v292 == v293 { 1.0 } else { 0.0 };
                            out294 = v294;
                        }
                        let v296 = if v295 == v4 { 1.0 } else { 0.0 };
                        out296 = v296;
                        if v296 != 0.0 {
                        } else {
                            let v297 = if v27 == v1 { 1.0 } else { 0.0 };
                            out297 = v297;
                            let v305: f64;
                            if v297 != 0.0 {
                                let v301 = ((v50 - v213) * v51).sqrt();
                                v305 = v301;
                            } else {
                                let v304 = ((v50 - v213) * v51).powf(v27);
                                v305 = v304;
                            }
                            let v309 = v32 * (((v50 - v213) * v46) / v305);
                            out309 = v309;
                            let v311 = (v205 * v309) * v309;
                            out311 = v311;
                        }
                        let v298 = if v71 > v248 { 1.0 } else { 0.0 };
                        out298 = v298;
                        let v315: f64;
                        if v298 != 0.0 {
                            v315 = v3;
                        } else {
                            let v314 = if v214 > ((-v56) * v71) { 1.0 } else { 0.0 };
                            out314 = v314;
                            let v321: f64;
                            if v314 != 0.0 {
                                let v316 = if v61 == v220 { 1.0 } else { 0.0 };
                                out316 = v316;
                                let v329: f64;
                                if v316 != 0.0 {
                                    let v322 = v214 * v72;
                                    let v325 = ((v322 * v322) * v322) * v322;
                                    v329 = v325;
                                } else {
                                    let v328 = ((v214 * v72).abs()).powf(v61);
                                    v329 = v328;
                                }
                                let v331 = v3 / (v3 - v329);
                                v321 = v331;
                            } else {
                                let v320 = v64 + ((v214 + (v56 * v71)) * v88);
                                v321 = v320;
                            }
                            v315 = v321;
                        }
                        out315 = v315;
                    }
                    if v186 != 0.0 {
                    } else {
                        let v335 = if v334 == v4 { 1.0 } else { 0.0 };
                        out335 = v335;
                        let v336 = if (if v332 == v4 { 1.0 } else { 0.0 }) != 0.0 && v335 != 0.0 { 1.0 } else { 0.0 };
                        out336 = v336;
                        if v336 != 0.0 {
                        } else {
                            let v337 = if v29 == v1 { 1.0 } else { 0.0 };
                            out337 = v337;
                            if v337 != 0.0 {
                            } else {
                                let v339 = v3 - (v196 * v29);
                                out339 = v339;
                            }
                        }
                        if v335 != 0.0 {
                        } else {
                            let v341 = (-v29) * v33;
                            out341 = v341;
                            let v343 = if v341 == v342 { 1.0 } else { 0.0 };
                            out343 = v343;
                        }
                        let v345 = if v344 == v4 { 1.0 } else { 0.0 };
                        out345 = v345;
                        if v345 != 0.0 {
                        } else {
                            let v346 = if v29 == v1 { 1.0 } else { 0.0 };
                            out346 = v346;
                            let v354: f64;
                            if v346 != 0.0 {
                                let v350 = ((v52 - v213) * v53).sqrt();
                                v354 = v350;
                            } else {
                                let v353 = ((v52 - v213) * v53).powf(v29);
                                v354 = v353;
                            }
                            let v358 = v33 * (((v52 - v213) * v47) / v354);
                            out358 = v358;
                            let v360 = (v205 * v358) * v358;
                            out360 = v360;
                        }
                        let v347 = if v73 > v248 { 1.0 } else { 0.0 };
                        out347 = v347;
                        let v364: f64;
                        if v347 != 0.0 {
                            v364 = v3;
                        } else {
                            let v363 = if v214 > ((-v56) * v73) { 1.0 } else { 0.0 };
                            out363 = v363;
                            let v370: f64;
                            if v363 != 0.0 {
                                let v365 = if v65 == v220 { 1.0 } else { 0.0 };
                                out365 = v365;
                                let v378: f64;
                                if v365 != 0.0 {
                                    let v371 = v214 * v74;
                                    let v374 = ((v371 * v371) * v371) * v371;
                                    v378 = v374;
                                } else {
                                    let v377 = ((v214 * v74).abs()).powf(v65);
                                    v378 = v377;
                                }
                                let v380 = v3 / (v3 - v378);
                                v370 = v380;
                            } else {
                                let v369 = v68 + ((v214 + (v56 * v73)) * v95);
                                v370 = v369;
                            }
                            v364 = v370;
                        }
                        out364 = v364;
                    }
                    let v381: f64;
                    let v382: f64;
                    if v212 != 0.0 {
                        let v383 = if v207 > v4 { 1.0 } else { 0.0 };
                        out383 = v383;
                        if v383 != 0.0 {
                        } else {
                            let v384 = -v207;
                            out384 = v384;
                        }
                        let v386 = v207 - v201;
                        let v393 = v1 * ((v207 + v201) - (((v386 * v386) + ((v220 * v10) * v10)).sqrt()));
                        let v399 = v1 * (v207 - (((v207 * v207) + v395).sqrt()));
                        v381 = v393;
                        v382 = v399;
                    } else {
                        v381 = v4;
                        v382 = v214;
                    }
                    if v173 != 0.0 {
                    } else {
                        let v401 = if v235 == v4 { 1.0 } else { 0.0 };
                        out401 = v401;
                        let v402 = if (if v233 == v4 { 1.0 } else { 0.0 }) != 0.0 && v401 != 0.0 { 1.0 } else { 0.0 };
                        out402 = v402;
                        if v402 != 0.0 {
                        } else {
                            let v403 = if v25 == v1 { 1.0 } else { 0.0 };
                            out403 = v403;
                            if v403 != 0.0 {
                            } else {
                                let v405 = v3 - (v196 * v25);
                                out405 = v405;
                            }
                        }
                        if v401 != 0.0 {
                        } else {
                            let v407 = (-v25) * v31;
                            out407 = v407;
                            let v409 = if v407 == v408 { 1.0 } else { 0.0 };
                            out409 = v409;
                        }
                        let v410 = if v245 == v4 { 1.0 } else { 0.0 };
                        out410 = v410;
                        if v410 != 0.0 {
                        } else {
                            let v411 = if v25 == v1 { 1.0 } else { 0.0 };
                            out411 = v411;
                            let v419: f64;
                            if v411 != 0.0 {
                                let v415 = ((v48 - v381) * v49).sqrt();
                                v419 = v415;
                            } else {
                                let v418 = ((v48 - v381) * v49).powf(v25);
                                v419 = v418;
                            }
                            let v423 = v31 * (((v48 - v381) * v45) / v419);
                            out423 = v423;
                            let v425 = (v207 * v423) * v423;
                            out425 = v425;
                        }
                        let v412 = if v69 > v248 { 1.0 } else { 0.0 };
                        out412 = v412;
                        let v429: f64;
                        if v412 != 0.0 {
                            v429 = v3;
                        } else {
                            let v428 = if v382 > ((-v56) * v69) { 1.0 } else { 0.0 };
                            out428 = v428;
                            let v435: f64;
                            if v428 != 0.0 {
                                let v430 = if v57 == v220 { 1.0 } else { 0.0 };
                                out430 = v430;
                                let v443: f64;
                                if v430 != 0.0 {
                                    let v436 = v382 * v70;
                                    let v439 = ((v436 * v436) * v436) * v436;
                                    v443 = v439;
                                } else {
                                    let v442 = ((v382 * v70).abs()).powf(v57);
                                    v443 = v442;
                                }
                                let v445 = v3 / (v3 - v443);
                                v435 = v445;
                            } else {
                                let v434 = v60 + ((v382 + (v56 * v69)) * v81);
                                v435 = v434;
                            }
                            v429 = v435;
                        }
                        out429 = v429;
                    }
                    if v180 != 0.0 {
                    } else {
                        let v447 = if v285 == v4 { 1.0 } else { 0.0 };
                        out447 = v447;
                        let v448 = if (if v283 == v4 { 1.0 } else { 0.0 }) != 0.0 && v447 != 0.0 { 1.0 } else { 0.0 };
                        out448 = v448;
                        if v448 != 0.0 {
                        } else {
                            let v449 = if v27 == v1 { 1.0 } else { 0.0 };
                            out449 = v449;
                            if v449 != 0.0 {
                            } else {
                                let v451 = v3 - (v196 * v27);
                                out451 = v451;
                            }
                        }
                        if v447 != 0.0 {
                        } else {
                            let v453 = (-v27) * v32;
                            out453 = v453;
                            let v455 = if v453 == v454 { 1.0 } else { 0.0 };
                            out455 = v455;
                        }
                        let v456 = if v295 == v4 { 1.0 } else { 0.0 };
                        out456 = v456;
                        if v456 != 0.0 {
                        } else {
                            let v457 = if v27 == v1 { 1.0 } else { 0.0 };
                            out457 = v457;
                            let v465: f64;
                            if v457 != 0.0 {
                                let v461 = ((v50 - v381) * v51).sqrt();
                                v465 = v461;
                            } else {
                                let v464 = ((v50 - v381) * v51).powf(v27);
                                v465 = v464;
                            }
                            let v469 = v32 * (((v50 - v381) * v46) / v465);
                            out469 = v469;
                            let v471 = (v207 * v469) * v469;
                            out471 = v471;
                        }
                        let v458 = if v71 > v248 { 1.0 } else { 0.0 };
                        out458 = v458;
                        let v475: f64;
                        if v458 != 0.0 {
                            v475 = v3;
                        } else {
                            let v474 = if v382 > ((-v56) * v71) { 1.0 } else { 0.0 };
                            out474 = v474;
                            let v481: f64;
                            if v474 != 0.0 {
                                let v476 = if v61 == v220 { 1.0 } else { 0.0 };
                                out476 = v476;
                                let v489: f64;
                                if v476 != 0.0 {
                                    let v482 = v382 * v72;
                                    let v485 = ((v482 * v482) * v482) * v482;
                                    v489 = v485;
                                } else {
                                    let v488 = ((v382 * v72).abs()).powf(v61);
                                    v489 = v488;
                                }
                                let v491 = v3 / (v3 - v489);
                                v481 = v491;
                            } else {
                                let v480 = v64 + ((v382 + (v56 * v71)) * v88);
                                v481 = v480;
                            }
                            v475 = v481;
                        }
                        out475 = v475;
                    }
                    if v186 != 0.0 {
                    } else {
                        let v493 = if v334 == v4 { 1.0 } else { 0.0 };
                        out493 = v493;
                        let v494 = if (if v332 == v4 { 1.0 } else { 0.0 }) != 0.0 && v493 != 0.0 { 1.0 } else { 0.0 };
                        out494 = v494;
                        if v494 != 0.0 {
                        } else {
                            let v495 = if v29 == v1 { 1.0 } else { 0.0 };
                            out495 = v495;
                            if v495 != 0.0 {
                            } else {
                                let v497 = v3 - (v196 * v29);
                                out497 = v497;
                            }
                        }
                        if v493 != 0.0 {
                        } else {
                            let v499 = (-v29) * v33;
                            out499 = v499;
                            let v501 = if v499 == v500 { 1.0 } else { 0.0 };
                            out501 = v501;
                        }
                        let v502 = if v344 == v4 { 1.0 } else { 0.0 };
                        out502 = v502;
                        if v502 != 0.0 {
                        } else {
                            let v503 = if v29 == v1 { 1.0 } else { 0.0 };
                            out503 = v503;
                            let v511: f64;
                            if v503 != 0.0 {
                                let v507 = ((v52 - v381) * v53).sqrt();
                                v511 = v507;
                            } else {
                                let v510 = ((v52 - v381) * v53).powf(v29);
                                v511 = v510;
                            }
                            let v515 = v33 * (((v52 - v381) * v47) / v511);
                            out515 = v515;
                            let v517 = (v207 * v515) * v515;
                            out517 = v517;
                        }
                        let v504 = if v73 > v248 { 1.0 } else { 0.0 };
                        out504 = v504;
                        let v521: f64;
                        if v504 != 0.0 {
                            v521 = v3;
                        } else {
                            let v520 = if v382 > ((-v56) * v73) { 1.0 } else { 0.0 };
                            out520 = v520;
                            let v527: f64;
                            if v520 != 0.0 {
                                let v522 = if v65 == v220 { 1.0 } else { 0.0 };
                                out522 = v522;
                                let v535: f64;
                                if v522 != 0.0 {
                                    let v528 = v382 * v74;
                                    let v531 = ((v528 * v528) * v528) * v528;
                                    v535 = v531;
                                } else {
                                    let v534 = ((v382 * v74).abs()).powf(v65);
                                    v535 = v534;
                                }
                                let v537 = v3 / (v3 - v535);
                                v527 = v537;
                            } else {
                                let v526 = v68 + ((v382 + (v56 * v73)) * v95);
                                v527 = v526;
                            }
                            v521 = v527;
                        }
                        out521 = v521;
                    }
                    let v538: f64;
                    let v539: f64;
                    if v212 != 0.0 {
                        let v540 = if v209 > v4 { 1.0 } else { 0.0 };
                        out540 = v540;
                        if v540 != 0.0 {
                        } else {
                            let v541 = -v209;
                            out541 = v541;
                        }
                        let v543 = v209 - v201;
                        let v550 = v1 * ((v209 + v201) - (((v543 * v543) + ((v220 * v10) * v10)).sqrt()));
                        let v556 = v1 * (v209 - (((v209 * v209) + v552).sqrt()));
                        v538 = v550;
                        v539 = v556;
                    } else {
                        v538 = v4;
                        v539 = v382;
                    }
                    if v173 != 0.0 {
                    } else {
                        let v558 = if v235 == v4 { 1.0 } else { 0.0 };
                        out558 = v558;
                        let v559 = if (if v233 == v4 { 1.0 } else { 0.0 }) != 0.0 && v558 != 0.0 { 1.0 } else { 0.0 };
                        out559 = v559;
                        if v559 != 0.0 {
                        } else {
                            let v560 = if v25 == v1 { 1.0 } else { 0.0 };
                            out560 = v560;
                            if v560 != 0.0 {
                            } else {
                                let v562 = v3 - (v196 * v25);
                                out562 = v562;
                            }
                        }
                        if v558 != 0.0 {
                        } else {
                            let v564 = (-v25) * v31;
                            out564 = v564;
                            let v566 = if v564 == v565 { 1.0 } else { 0.0 };
                            out566 = v566;
                        }
                        let v567 = if v245 == v4 { 1.0 } else { 0.0 };
                        out567 = v567;
                        if v567 != 0.0 {
                        } else {
                            let v568 = if v25 == v1 { 1.0 } else { 0.0 };
                            out568 = v568;
                            let v576: f64;
                            if v568 != 0.0 {
                                let v572 = ((v48 - v538) * v49).sqrt();
                                v576 = v572;
                            } else {
                                let v575 = ((v48 - v538) * v49).powf(v25);
                                v576 = v575;
                            }
                            let v580 = v31 * (((v48 - v538) * v45) / v576);
                            out580 = v580;
                            let v582 = (v209 * v580) * v580;
                            out582 = v582;
                        }
                        let v569 = if v69 > v248 { 1.0 } else { 0.0 };
                        out569 = v569;
                        let v586: f64;
                        if v569 != 0.0 {
                            v586 = v3;
                        } else {
                            let v585 = if v539 > ((-v56) * v69) { 1.0 } else { 0.0 };
                            out585 = v585;
                            let v592: f64;
                            if v585 != 0.0 {
                                let v587 = if v57 == v220 { 1.0 } else { 0.0 };
                                out587 = v587;
                                let v600: f64;
                                if v587 != 0.0 {
                                    let v593 = v539 * v70;
                                    let v596 = ((v593 * v593) * v593) * v593;
                                    v600 = v596;
                                } else {
                                    let v599 = ((v539 * v70).abs()).powf(v57);
                                    v600 = v599;
                                }
                                let v602 = v3 / (v3 - v600);
                                v592 = v602;
                            } else {
                                let v591 = v60 + ((v539 + (v56 * v69)) * v81);
                                v592 = v591;
                            }
                            v586 = v592;
                        }
                        out586 = v586;
                    }
                    if v180 != 0.0 {
                    } else {
                        let v604 = if v285 == v4 { 1.0 } else { 0.0 };
                        out604 = v604;
                        let v605 = if (if v283 == v4 { 1.0 } else { 0.0 }) != 0.0 && v604 != 0.0 { 1.0 } else { 0.0 };
                        out605 = v605;
                        if v605 != 0.0 {
                        } else {
                            let v606 = if v27 == v1 { 1.0 } else { 0.0 };
                            out606 = v606;
                            if v606 != 0.0 {
                            } else {
                                let v608 = v3 - (v196 * v27);
                                out608 = v608;
                            }
                        }
                        if v604 != 0.0 {
                        } else {
                            let v610 = (-v27) * v32;
                            out610 = v610;
                            let v612 = if v610 == v611 { 1.0 } else { 0.0 };
                            out612 = v612;
                        }
                        let v613 = if v295 == v4 { 1.0 } else { 0.0 };
                        out613 = v613;
                        if v613 != 0.0 {
                        } else {
                            let v614 = if v27 == v1 { 1.0 } else { 0.0 };
                            out614 = v614;
                            let v622: f64;
                            if v614 != 0.0 {
                                let v618 = ((v50 - v538) * v51).sqrt();
                                v622 = v618;
                            } else {
                                let v621 = ((v50 - v538) * v51).powf(v27);
                                v622 = v621;
                            }
                            let v626 = v32 * (((v50 - v538) * v46) / v622);
                            out626 = v626;
                            let v628 = (v209 * v626) * v626;
                            out628 = v628;
                        }
                        let v615 = if v71 > v248 { 1.0 } else { 0.0 };
                        out615 = v615;
                        let v632: f64;
                        if v615 != 0.0 {
                            v632 = v3;
                        } else {
                            let v631 = if v539 > ((-v56) * v71) { 1.0 } else { 0.0 };
                            out631 = v631;
                            let v638: f64;
                            if v631 != 0.0 {
                                let v633 = if v61 == v220 { 1.0 } else { 0.0 };
                                out633 = v633;
                                let v646: f64;
                                if v633 != 0.0 {
                                    let v639 = v539 * v72;
                                    let v642 = ((v639 * v639) * v639) * v639;
                                    v646 = v642;
                                } else {
                                    let v645 = ((v539 * v72).abs()).powf(v61);
                                    v646 = v645;
                                }
                                let v648 = v3 / (v3 - v646);
                                v638 = v648;
                            } else {
                                let v637 = v64 + ((v539 + (v56 * v71)) * v88);
                                v638 = v637;
                            }
                            v632 = v638;
                        }
                        out632 = v632;
                    }
                    if v186 != 0.0 {
                    } else {
                        let v650 = if v334 == v4 { 1.0 } else { 0.0 };
                        out650 = v650;
                        let v651 = if (if v332 == v4 { 1.0 } else { 0.0 }) != 0.0 && v650 != 0.0 { 1.0 } else { 0.0 };
                        out651 = v651;
                        if v651 != 0.0 {
                        } else {
                            let v652 = if v29 == v1 { 1.0 } else { 0.0 };
                            out652 = v652;
                            if v652 != 0.0 {
                            } else {
                                let v654 = v3 - (v196 * v29);
                                out654 = v654;
                            }
                        }
                        if v650 != 0.0 {
                        } else {
                            let v656 = (-v29) * v33;
                            out656 = v656;
                            let v658 = if v656 == v657 { 1.0 } else { 0.0 };
                            out658 = v658;
                        }
                        let v659 = if v344 == v4 { 1.0 } else { 0.0 };
                        out659 = v659;
                        if v659 != 0.0 {
                        } else {
                            let v660 = if v29 == v1 { 1.0 } else { 0.0 };
                            out660 = v660;
                            let v668: f64;
                            if v660 != 0.0 {
                                let v664 = ((v52 - v538) * v53).sqrt();
                                v668 = v664;
                            } else {
                                let v667 = ((v52 - v538) * v53).powf(v29);
                                v668 = v667;
                            }
                            let v672 = v33 * (((v52 - v538) * v47) / v668);
                            out672 = v672;
                            let v674 = (v209 * v672) * v672;
                            out674 = v674;
                        }
                        let v661 = if v73 > v248 { 1.0 } else { 0.0 };
                        out661 = v661;
                        let v678: f64;
                        if v661 != 0.0 {
                            v678 = v3;
                        } else {
                            let v677 = if v539 > ((-v56) * v73) { 1.0 } else { 0.0 };
                            out677 = v677;
                            let v684: f64;
                            if v677 != 0.0 {
                                let v679 = if v65 == v220 { 1.0 } else { 0.0 };
                                out679 = v679;
                                let v692: f64;
                                if v679 != 0.0 {
                                    let v685 = v539 * v74;
                                    let v688 = ((v685 * v685) * v685) * v685;
                                    v692 = v688;
                                } else {
                                    let v691 = ((v539 * v74).abs()).powf(v65);
                                    v692 = v691;
                                }
                                let v694 = v3 / (v3 - v692);
                                v684 = v694;
                            } else {
                                let v683 = v68 + ((v539 + (v56 * v73)) * v95);
                                v684 = v683;
                            }
                            v678 = v684;
                        }
                        out678 = v678;
                    }
                    let v695: f64;
                    let v696: f64;
                    if v212 != 0.0 {
                        let v699 = v697 - v201;
                        let v706 = v1 * ((v697 + v201) - (((v699 * v699) + ((v220 * v10) * v10)).sqrt()));
                        v695 = v706;
                        v696 = v707;
                    } else {
                        v695 = v4;
                        v696 = v539;
                    }
                    if v173 != 0.0 {
                    } else {
                        let v709 = if v235 == v4 { 1.0 } else { 0.0 };
                        out709 = v709;
                        let v710 = if (if v233 == v4 { 1.0 } else { 0.0 }) != 0.0 && v709 != 0.0 { 1.0 } else { 0.0 };
                        out710 = v710;
                        if v710 != 0.0 {
                        } else {
                            let v711 = if v25 == v1 { 1.0 } else { 0.0 };
                            out711 = v711;
                            if v711 != 0.0 {
                            } else {
                                let v713 = v3 - (v196 * v25);
                                out713 = v713;
                            }
                        }
                        if v709 != 0.0 {
                        } else {
                            let v715 = (-v25) * v31;
                            out715 = v715;
                            let v717 = if v715 == v716 { 1.0 } else { 0.0 };
                            out717 = v717;
                        }
                        let v718 = if v245 == v4 { 1.0 } else { 0.0 };
                        out718 = v718;
                        if v718 != 0.0 {
                        } else {
                            let v719 = if v25 == v1 { 1.0 } else { 0.0 };
                            out719 = v719;
                            let v727: f64;
                            if v719 != 0.0 {
                                let v723 = ((v48 - v695) * v49).sqrt();
                                v727 = v723;
                            } else {
                                let v726 = ((v48 - v695) * v49).powf(v25);
                                v727 = v726;
                            }
                            let v731 = v31 * (((v48 - v695) * v45) / v727);
                            out731 = v731;
                            let v733 = (v697 * v731) * v731;
                            out733 = v733;
                        }
                        let v720 = if v69 > v248 { 1.0 } else { 0.0 };
                        out720 = v720;
                        let v737: f64;
                        if v720 != 0.0 {
                            v737 = v3;
                        } else {
                            let v736 = if v696 > ((-v56) * v69) { 1.0 } else { 0.0 };
                            out736 = v736;
                            let v743: f64;
                            if v736 != 0.0 {
                                let v738 = if v57 == v220 { 1.0 } else { 0.0 };
                                out738 = v738;
                                let v751: f64;
                                if v738 != 0.0 {
                                    let v744 = v696 * v70;
                                    let v747 = ((v744 * v744) * v744) * v744;
                                    v751 = v747;
                                } else {
                                    let v750 = ((v696 * v70).abs()).powf(v57);
                                    v751 = v750;
                                }
                                let v753 = v3 / (v3 - v751);
                                v743 = v753;
                            } else {
                                let v742 = v60 + ((v696 + (v56 * v69)) * v81);
                                v743 = v742;
                            }
                            v737 = v743;
                        }
                        out737 = v737;
                    }
                    if v180 != 0.0 {
                    } else {
                        let v755 = if v285 == v4 { 1.0 } else { 0.0 };
                        out755 = v755;
                        let v756 = if (if v283 == v4 { 1.0 } else { 0.0 }) != 0.0 && v755 != 0.0 { 1.0 } else { 0.0 };
                        out756 = v756;
                        if v756 != 0.0 {
                        } else {
                            let v757 = if v27 == v1 { 1.0 } else { 0.0 };
                            out757 = v757;
                            if v757 != 0.0 {
                            } else {
                                let v759 = v3 - (v196 * v27);
                                out759 = v759;
                            }
                        }
                        if v755 != 0.0 {
                        } else {
                            let v761 = (-v27) * v32;
                            out761 = v761;
                            let v763 = if v761 == v762 { 1.0 } else { 0.0 };
                            out763 = v763;
                        }
                        let v764 = if v295 == v4 { 1.0 } else { 0.0 };
                        out764 = v764;
                        if v764 != 0.0 {
                        } else {
                            let v765 = if v27 == v1 { 1.0 } else { 0.0 };
                            out765 = v765;
                            let v773: f64;
                            if v765 != 0.0 {
                                let v769 = ((v50 - v695) * v51).sqrt();
                                v773 = v769;
                            } else {
                                let v772 = ((v50 - v695) * v51).powf(v27);
                                v773 = v772;
                            }
                            let v777 = v32 * (((v50 - v695) * v46) / v773);
                            out777 = v777;
                            let v779 = (v697 * v777) * v777;
                            out779 = v779;
                        }
                        let v766 = if v71 > v248 { 1.0 } else { 0.0 };
                        out766 = v766;
                        let v783: f64;
                        if v766 != 0.0 {
                            v783 = v3;
                        } else {
                            let v782 = if v696 > ((-v56) * v71) { 1.0 } else { 0.0 };
                            out782 = v782;
                            let v789: f64;
                            if v782 != 0.0 {
                                let v784 = if v61 == v220 { 1.0 } else { 0.0 };
                                out784 = v784;
                                let v797: f64;
                                if v784 != 0.0 {
                                    let v790 = v696 * v72;
                                    let v793 = ((v790 * v790) * v790) * v790;
                                    v797 = v793;
                                } else {
                                    let v796 = ((v696 * v72).abs()).powf(v61);
                                    v797 = v796;
                                }
                                let v799 = v3 / (v3 - v797);
                                v789 = v799;
                            } else {
                                let v788 = v64 + ((v696 + (v56 * v71)) * v88);
                                v789 = v788;
                            }
                            v783 = v789;
                        }
                        out783 = v783;
                    }
                    if v186 != 0.0 {
                    } else {
                        let v801 = if v334 == v4 { 1.0 } else { 0.0 };
                        out801 = v801;
                        let v802 = if (if v332 == v4 { 1.0 } else { 0.0 }) != 0.0 && v801 != 0.0 { 1.0 } else { 0.0 };
                        out802 = v802;
                        if v802 != 0.0 {
                        } else {
                            let v803 = if v29 == v1 { 1.0 } else { 0.0 };
                            out803 = v803;
                            if v803 != 0.0 {
                            } else {
                                let v805 = v3 - (v196 * v29);
                                out805 = v805;
                            }
                        }
                        if v801 != 0.0 {
                        } else {
                            let v807 = (-v29) * v33;
                            out807 = v807;
                            let v809 = if v807 == v808 { 1.0 } else { 0.0 };
                            out809 = v809;
                        }
                        let v810 = if v344 == v4 { 1.0 } else { 0.0 };
                        out810 = v810;
                        if v810 != 0.0 {
                        } else {
                            let v811 = if v29 == v1 { 1.0 } else { 0.0 };
                            out811 = v811;
                            let v819: f64;
                            if v811 != 0.0 {
                                let v815 = ((v52 - v695) * v53).sqrt();
                                v819 = v815;
                            } else {
                                let v818 = ((v52 - v695) * v53).powf(v29);
                                v819 = v818;
                            }
                            let v823 = v33 * (((v52 - v695) * v47) / v819);
                            out823 = v823;
                            let v825 = (v697 * v823) * v823;
                            out825 = v825;
                        }
                        let v812 = if v73 > v248 { 1.0 } else { 0.0 };
                        out812 = v812;
                        let v829: f64;
                        if v812 != 0.0 {
                            v829 = v3;
                        } else {
                            let v828 = if v696 > ((-v56) * v73) { 1.0 } else { 0.0 };
                            out828 = v828;
                            let v835: f64;
                            if v828 != 0.0 {
                                let v830 = if v65 == v220 { 1.0 } else { 0.0 };
                                out830 = v830;
                                let v843: f64;
                                if v830 != 0.0 {
                                    let v836 = v696 * v74;
                                    let v839 = ((v836 * v836) * v836) * v836;
                                    v843 = v839;
                                } else {
                                    let v842 = ((v696 * v74).abs()).powf(v65);
                                    v843 = v842;
                                }
                                let v845 = v3 / (v3 - v843);
                                v835 = v845;
                            } else {
                                let v834 = v68 + ((v696 + (v56 * v73)) * v95);
                                v835 = v834;
                            }
                            v829 = v835;
                        }
                        out829 = v829;
                    }
                    let v846: f64;
                    let v847: f64;
                    if v212 != 0.0 {
                        let v850 = v848 - v201;
                        let v857 = v1 * ((v848 + v201) - (((v850 * v850) + ((v220 * v10) * v10)).sqrt()));
                        v846 = v857;
                        v847 = v858;
                    } else {
                        v846 = v4;
                        v847 = v696;
                    }
                    if v173 != 0.0 {
                    } else {
                        let v860 = if v235 == v4 { 1.0 } else { 0.0 };
                        out860 = v860;
                        let v861 = if (if v233 == v4 { 1.0 } else { 0.0 }) != 0.0 && v860 != 0.0 { 1.0 } else { 0.0 };
                        out861 = v861;
                        if v861 != 0.0 {
                        } else {
                            let v862 = if v25 == v1 { 1.0 } else { 0.0 };
                            out862 = v862;
                            if v862 != 0.0 {
                            } else {
                                let v864 = v3 - (v196 * v25);
                                out864 = v864;
                            }
                        }
                        if v860 != 0.0 {
                        } else {
                            let v866 = (-v25) * v31;
                            out866 = v866;
                            let v868 = if v866 == v867 { 1.0 } else { 0.0 };
                            out868 = v868;
                        }
                        let v869 = if v245 == v4 { 1.0 } else { 0.0 };
                        out869 = v869;
                        if v869 != 0.0 {
                        } else {
                            let v870 = if v25 == v1 { 1.0 } else { 0.0 };
                            out870 = v870;
                            let v878: f64;
                            if v870 != 0.0 {
                                let v874 = ((v48 - v846) * v49).sqrt();
                                v878 = v874;
                            } else {
                                let v877 = ((v48 - v846) * v49).powf(v25);
                                v878 = v877;
                            }
                            let v882 = v31 * (((v48 - v846) * v45) / v878);
                            out882 = v882;
                            let v884 = (v848 * v882) * v882;
                            out884 = v884;
                        }
                        let v871 = if v69 > v248 { 1.0 } else { 0.0 };
                        out871 = v871;
                        let v888: f64;
                        if v871 != 0.0 {
                            v888 = v3;
                        } else {
                            let v887 = if v847 > ((-v56) * v69) { 1.0 } else { 0.0 };
                            out887 = v887;
                            let v894: f64;
                            if v887 != 0.0 {
                                let v889 = if v57 == v220 { 1.0 } else { 0.0 };
                                out889 = v889;
                                let v902: f64;
                                if v889 != 0.0 {
                                    let v895 = v847 * v70;
                                    let v898 = ((v895 * v895) * v895) * v895;
                                    v902 = v898;
                                } else {
                                    let v901 = ((v847 * v70).abs()).powf(v57);
                                    v902 = v901;
                                }
                                let v904 = v3 / (v3 - v902);
                                v894 = v904;
                            } else {
                                let v893 = v60 + ((v847 + (v56 * v69)) * v81);
                                v894 = v893;
                            }
                            v888 = v894;
                        }
                        out888 = v888;
                    }
                    if v180 != 0.0 {
                    } else {
                        let v906 = if v285 == v4 { 1.0 } else { 0.0 };
                        out906 = v906;
                        let v907 = if (if v283 == v4 { 1.0 } else { 0.0 }) != 0.0 && v906 != 0.0 { 1.0 } else { 0.0 };
                        out907 = v907;
                        if v907 != 0.0 {
                        } else {
                            let v908 = if v27 == v1 { 1.0 } else { 0.0 };
                            out908 = v908;
                            if v908 != 0.0 {
                            } else {
                                let v910 = v3 - (v196 * v27);
                                out910 = v910;
                            }
                        }
                        if v906 != 0.0 {
                        } else {
                            let v912 = (-v27) * v32;
                            out912 = v912;
                            let v914 = if v912 == v913 { 1.0 } else { 0.0 };
                            out914 = v914;
                        }
                        let v915 = if v295 == v4 { 1.0 } else { 0.0 };
                        out915 = v915;
                        if v915 != 0.0 {
                        } else {
                            let v916 = if v27 == v1 { 1.0 } else { 0.0 };
                            out916 = v916;
                            let v924: f64;
                            if v916 != 0.0 {
                                let v920 = ((v50 - v846) * v51).sqrt();
                                v924 = v920;
                            } else {
                                let v923 = ((v50 - v846) * v51).powf(v27);
                                v924 = v923;
                            }
                            let v928 = v32 * (((v50 - v846) * v46) / v924);
                            out928 = v928;
                            let v930 = (v848 * v928) * v928;
                            out930 = v930;
                        }
                        let v917 = if v71 > v248 { 1.0 } else { 0.0 };
                        out917 = v917;
                        let v934: f64;
                        if v917 != 0.0 {
                            v934 = v3;
                        } else {
                            let v933 = if v847 > ((-v56) * v71) { 1.0 } else { 0.0 };
                            out933 = v933;
                            let v940: f64;
                            if v933 != 0.0 {
                                let v935 = if v61 == v220 { 1.0 } else { 0.0 };
                                out935 = v935;
                                let v948: f64;
                                if v935 != 0.0 {
                                    let v941 = v847 * v72;
                                    let v944 = ((v941 * v941) * v941) * v941;
                                    v948 = v944;
                                } else {
                                    let v947 = ((v847 * v72).abs()).powf(v61);
                                    v948 = v947;
                                }
                                let v950 = v3 / (v3 - v948);
                                v940 = v950;
                            } else {
                                let v939 = v64 + ((v847 + (v56 * v71)) * v88);
                                v940 = v939;
                            }
                            v934 = v940;
                        }
                        out934 = v934;
                    }
                    if v186 != 0.0 {
                    } else {
                        let v952 = if v334 == v4 { 1.0 } else { 0.0 };
                        out952 = v952;
                        let v953 = if (if v332 == v4 { 1.0 } else { 0.0 }) != 0.0 && v952 != 0.0 { 1.0 } else { 0.0 };
                        out953 = v953;
                        if v953 != 0.0 {
                        } else {
                            let v954 = if v29 == v1 { 1.0 } else { 0.0 };
                            out954 = v954;
                            if v954 != 0.0 {
                            } else {
                                let v956 = v3 - (v196 * v29);
                                out956 = v956;
                            }
                        }
                        if v952 != 0.0 {
                        } else {
                            let v958 = (-v29) * v33;
                            out958 = v958;
                            let v960 = if v958 == v959 { 1.0 } else { 0.0 };
                            out960 = v960;
                        }
                        let v961 = if v344 == v4 { 1.0 } else { 0.0 };
                        out961 = v961;
                        if v961 != 0.0 {
                        } else {
                            let v962 = if v29 == v1 { 1.0 } else { 0.0 };
                            out962 = v962;
                            let v970: f64;
                            if v962 != 0.0 {
                                let v966 = ((v52 - v846) * v53).sqrt();
                                v970 = v966;
                            } else {
                                let v969 = ((v52 - v846) * v53).powf(v29);
                                v970 = v969;
                            }
                            let v974 = v33 * (((v52 - v846) * v47) / v970);
                            out974 = v974;
                            let v976 = (v848 * v974) * v974;
                            out976 = v976;
                        }
                        let v963 = if v73 > v248 { 1.0 } else { 0.0 };
                        out963 = v963;
                        let v980: f64;
                        if v963 != 0.0 {
                            v980 = v3;
                        } else {
                            let v979 = if v847 > ((-v56) * v73) { 1.0 } else { 0.0 };
                            out979 = v979;
                            let v986: f64;
                            if v979 != 0.0 {
                                let v981 = if v65 == v220 { 1.0 } else { 0.0 };
                                out981 = v981;
                                let v994: f64;
                                if v981 != 0.0 {
                                    let v987 = v847 * v74;
                                    let v990 = ((v987 * v987) * v987) * v987;
                                    v994 = v990;
                                } else {
                                    let v993 = ((v847 * v74).abs()).powf(v65);
                                    v994 = v993;
                                }
                                let v996 = v3 / (v3 - v994);
                                v986 = v996;
                            } else {
                                let v985 = v68 + ((v847 + (v56 * v73)) * v95);
                                v986 = v985;
                            }
                            v980 = v986;
                        }
                        out980 = v980;
                    }
                    if v212 != 0.0 {
                        let v998 = v1 * v997;
                        out998 = v998;
                    } else {
                    }
                } else {
                }
                if v202 != 0.0 {
                } else {
                    let v1001 = if (if (if v173 != 0.0 && v180 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v186 != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                    out1001 = v1001;
                    if v1001 != 0.0 {
                        let v1009 = (v220 * v10) * v10;
                        out1009 = v1009;
                    } else {
                    }
                    if v173 != 0.0 {
                    } else {
                        let v1011 = if v235 == v4 { 1.0 } else { 0.0 };
                        out1011 = v1011;
                        let v1012 = if (if v233 == v4 { 1.0 } else { 0.0 }) != 0.0 && v1011 != 0.0 { 1.0 } else { 0.0 };
                        out1012 = v1012;
                        if v1012 != 0.0 {
                        } else {
                            let v1013 = if v25 == v1 { 1.0 } else { 0.0 };
                            out1013 = v1013;
                            if v1013 != 0.0 {
                            } else {
                                let v1015 = v3 - (v196 * v25);
                                out1015 = v1015;
                            }
                            if v1013 != 0.0 {
                            } else {
                                let v1017 = v25 - v1016;
                                out1017 = v1017;
                            }
                        }
                        if v1011 != 0.0 {
                        } else {
                            let v1019 = (-v25) * v31;
                            out1019 = v1019;
                            let v1021 = if v1019 == v1020 { 1.0 } else { 0.0 };
                            out1021 = v1021;
                            if v1021 != 0.0 {
                            } else {
                                let v1023 = v1019 - v1016;
                                out1023 = v1023;
                            }
                        }
                        let v1022 = if v245 == v4 { 1.0 } else { 0.0 };
                        out1022 = v1022;
                        if v1022 != 0.0 {
                        } else {
                            let v1024 = if v25 == v1 { 1.0 } else { 0.0 };
                            out1024 = v1024;
                            if v1024 != 0.0 {
                            } else {
                                let v1026 = v25 - v1016;
                                out1026 = v1026;
                            }
                        }
                        let v1025 = if v69 > v248 { 1.0 } else { 0.0 };
                        out1025 = v1025;
                        if v1025 != 0.0 {
                        } else {
                            let v1028 = (-v56) * v69;
                            out1028 = v1028;
                        }
                        let v1029 = if v26 == v1 { 1.0 } else { 0.0 };
                        out1029 = v1029;
                        if v1029 != 0.0 {
                        } else {
                            let v1030 = v26 - v1016;
                            out1030 = v1030;
                        }
                    }
                    if v180 != 0.0 {
                    } else {
                        let v1032 = if v285 == v4 { 1.0 } else { 0.0 };
                        out1032 = v1032;
                        let v1033 = if (if v283 == v4 { 1.0 } else { 0.0 }) != 0.0 && v1032 != 0.0 { 1.0 } else { 0.0 };
                        out1033 = v1033;
                        if v1033 != 0.0 {
                        } else {
                            let v1034 = if v27 == v1 { 1.0 } else { 0.0 };
                            out1034 = v1034;
                            if v1034 != 0.0 {
                            } else {
                                let v1036 = v3 - (v196 * v27);
                                out1036 = v1036;
                            }
                            if v1034 != 0.0 {
                            } else {
                                let v1037 = v27 - v1016;
                                out1037 = v1037;
                            }
                        }
                        if v1032 != 0.0 {
                        } else {
                            let v1039 = (-v27) * v32;
                            out1039 = v1039;
                            let v1041 = if v1039 == v1040 { 1.0 } else { 0.0 };
                            out1041 = v1041;
                            if v1041 != 0.0 {
                            } else {
                                let v1043 = v1039 - v1016;
                                out1043 = v1043;
                            }
                        }
                        let v1042 = if v295 == v4 { 1.0 } else { 0.0 };
                        out1042 = v1042;
                        if v1042 != 0.0 {
                        } else {
                            let v1044 = if v27 == v1 { 1.0 } else { 0.0 };
                            out1044 = v1044;
                            if v1044 != 0.0 {
                            } else {
                                let v1046 = v27 - v1016;
                                out1046 = v1046;
                            }
                        }
                        let v1045 = if v71 > v248 { 1.0 } else { 0.0 };
                        out1045 = v1045;
                        if v1045 != 0.0 {
                        } else {
                            let v1048 = (-v56) * v71;
                            out1048 = v1048;
                        }
                        let v1049 = if v28 == v1 { 1.0 } else { 0.0 };
                        out1049 = v1049;
                        if v1049 != 0.0 {
                        } else {
                            let v1050 = v28 - v1016;
                            out1050 = v1050;
                        }
                    }
                    if v186 != 0.0 {
                    } else {
                        let v1052 = if v334 == v4 { 1.0 } else { 0.0 };
                        out1052 = v1052;
                        let v1053 = if (if v332 == v4 { 1.0 } else { 0.0 }) != 0.0 && v1052 != 0.0 { 1.0 } else { 0.0 };
                        out1053 = v1053;
                        if v1053 != 0.0 {
                        } else {
                            let v1054 = if v29 == v1 { 1.0 } else { 0.0 };
                            out1054 = v1054;
                            if v1054 != 0.0 {
                            } else {
                                let v1056 = v3 - (v196 * v29);
                                out1056 = v1056;
                            }
                            if v1054 != 0.0 {
                            } else {
                                let v1057 = v29 - v1016;
                                out1057 = v1057;
                            }
                        }
                        if v1052 != 0.0 {
                        } else {
                            let v1059 = (-v29) * v33;
                            out1059 = v1059;
                            let v1061 = if v1059 == v1060 { 1.0 } else { 0.0 };
                            out1061 = v1061;
                            if v1061 != 0.0 {
                            } else {
                                let v1063 = v1059 - v1016;
                                out1063 = v1063;
                            }
                        }
                        let v1062 = if v344 == v4 { 1.0 } else { 0.0 };
                        out1062 = v1062;
                        if v1062 != 0.0 {
                        } else {
                            let v1064 = if v29 == v1 { 1.0 } else { 0.0 };
                            out1064 = v1064;
                            if v1064 != 0.0 {
                            } else {
                                let v1066 = v29 - v1016;
                                out1066 = v1066;
                            }
                        }
                        let v1065 = if v73 > v248 { 1.0 } else { 0.0 };
                        out1065 = v1065;
                        if v1065 != 0.0 {
                        } else {
                            let v1068 = (-v56) * v73;
                            out1068 = v1068;
                        }
                        if v108 != 0.0 {
                            let v1070 = if v30 == v1 { 1.0 } else { 0.0 };
                            out1070 = v1070;
                            if v1070 != 0.0 {
                            } else {
                                let v1071 = v30 - v1016;
                                out1071 = v1071;
                            }
                            let v1072 = if v125 == v1 { 1.0 } else { 0.0 };
                            out1072 = v1072;
                            if v1072 != 0.0 {
                            } else {
                                let v1073 = v125 - v1016;
                                out1073 = v1073;
                            }
                        } else {
                            let v1069 = if v30 == v1 { 1.0 } else { 0.0 };
                            out1069 = v1069;
                            if v1069 != 0.0 {
                            } else {
                                let v1074 = v30 - v1016;
                                out1074 = v1074;
                            }
                        }
                    }
                }
                let v1003 = v1002 * v172;
                let v1005 = v1003 * v1004;
                let v1007 = v1003 * v1006;
            [v2, v8, v26, v28, v30, v31, v32, v33, v36, v40, v44, v45, v46, v47, v49, v51, v53, v56, v60, v64, v68, v70, v72, v74, v81, v88, v95, v106, v108, out111, out147, out150, out153, v126, v127, v128, v135, v139, v143, v119, out160, v121, v122, v123, v124, v162, v163, v165, v166, v168, v169, v171, v173, v180, v186, v198, v201, v202, out205, out207, out209, out212, out215, out216, out236, out237, out238, out240, out242, out244, out246, out247, out260, out262, out249, out265, out267, out266, out286, out287, out288, out290, out292, out294, out296, out297, out309, out311, out298, out314, out316, out315, out335, out336, out337, out339, out341, out343, out345, out346, out358, out360, out347, out363, out365, out364, out383, out384, out401, out402, out403, out405, out407, out409, out410, out411, out423, out425, out412, out428, out430, out429, out447, out448, out449, out451, out453, out455, out456, out457, out469, out471, out458, out474, out476, out475, out493, out494, out495, out497, out499, out501, out502, out503, out515, out517, out504, out520, out522, out521, out540, out541, out558, out559, out560, out562, out564, out566, out567, out568, out580, out582, out569, out585, out587, out586, out604, out605, out606, out608, out610, out612, out613, out614, out626, out628, out615, out631, out633, out632, out650, out651, out652, out654, out656, out658, out659, out660, out672, out674, out661, out677, out679, out678, out709, out710, out711, out713, out715, out717, out718, out719, out731, out733, out720, out736, out738, out737, out755, out756, out757, out759, out761, out763, out764, out765, out777, out779, out766, out782, out784, out783, out801, out802, out803, out805, out807, out809, out810, out811, out823, out825, out812, out828, out830, out829, out860, out861, out862, out864, out866, out868, out869, out870, out882, out884, out871, out887, out889, out888, out906, out907, out908, out910, out912, out914, out915, out916, out928, out930, out917, out933, out935, out934, out952, out953, out954, out956, out958, out960, out961, out962, out974, out976, out963, out979, out981, out980, out998, out1001, out1009, out1011, out1012, out1013, out1015, out1019, out1021, out1022, out1024, out1025, out1028, out1029, out1032, out1033, out1034, out1036, out1039, out1041, out1042, out1044, out1045, out1048, out1049, out1052, out1053, out1054, out1056, out1059, out1061, out1062, out1064, out1065, out1068, out1070, v125, out1072, out1069, v1005, v1007, out1017, out1023, out1026, out1030, out1037, out1043, out1046, out1050, out1057, out1063, out1066, out1074, out1071, out1073]
        };
        self.canonical_staged[213] = produced[0];
        self.canonical_staged[0] = produced[1];
        self.canonical_staged[25] = produced[2];
        self.canonical_staged[33] = produced[3];
        self.canonical_staged[41] = produced[4];
        self.canonical_staged[4] = produced[5];
        self.canonical_staged[5] = produced[6];
        self.canonical_staged[6] = produced[7];
        self.canonical_staged[24] = produced[8];
        self.canonical_staged[32] = produced[9];
        self.canonical_staged[40] = produced[10];
        self.canonical_staged[153] = produced[11];
        self.canonical_staged[168] = produced[12];
        self.canonical_staged[182] = produced[13];
        self.canonical_staged[23] = produced[14];
        self.canonical_staged[31] = produced[15];
        self.canonical_staged[39] = produced[16];
        self.canonical_staged[157] = produced[17];
        self.canonical_staged[159] = produced[18];
        self.canonical_staged[173] = produced[19];
        self.canonical_staged[187] = produced[20];
        self.canonical_staged[156] = produced[21];
        self.canonical_staged[171] = produced[22];
        self.canonical_staged[185] = produced[23];
        self.canonical_staged[158] = produced[24];
        self.canonical_staged[172] = produced[25];
        self.canonical_staged[186] = produced[26];
        self.canonical_staged[214] = produced[27];
        self.canonical_staged[215] = produced[28];
        self.canonical_staged[216] = produced[29];
        self.canonical_staged[218] = produced[30];
        self.canonical_staged[219] = produced[31];
        self.canonical_staged[220] = produced[32];
        self.canonical_staged[1] = produced[33];
        self.canonical_staged[2] = produced[34];
        self.canonical_staged[3] = produced[35];
        self.canonical_staged[7] = produced[36];
        self.canonical_staged[8] = produced[37];
        self.canonical_staged[9] = produced[38];
        self.canonical_staged[10] = produced[39];
        self.canonical_staged[11] = produced[40];
        self.canonical_staged[12] = produced[41];
        self.canonical_staged[14] = produced[42];
        self.canonical_staged[13] = produced[43];
        self.canonical_staged[15] = produced[44];
        self.canonical_staged[223] = produced[45];
        self.canonical_staged[16] = produced[46];
        self.canonical_staged[224] = produced[47];
        self.canonical_staged[17] = produced[48];
        self.canonical_staged[225] = produced[49];
        self.canonical_staged[18] = produced[50];
        self.canonical_staged[226] = produced[51];
        self.canonical_staged[232] = produced[52];
        self.canonical_staged[233] = produced[53];
        self.canonical_staged[234] = produced[54];
        self.canonical_staged[19] = produced[55];
        self.canonical_staged[143] = produced[56];
        self.canonical_staged[235] = produced[57];
        self.canonical_staged[20] = produced[58];
        self.canonical_staged[46] = produced[59];
        self.canonical_staged[63] = produced[60];
        self.canonical_staged[236] = produced[61];
        self.canonical_staged[239] = produced[62];
        self.canonical_staged[21] = produced[63];
        self.canonical_staged[243] = produced[64];
        self.canonical_staged[241] = produced[65];
        self.canonical_staged[242] = produced[66];
        self.canonical_staged[22] = produced[67];
        self.canonical_staged[26] = produced[68];
        self.canonical_staged[244] = produced[69];
        self.canonical_staged[245] = produced[70];
        self.canonical_staged[249] = produced[71];
        self.canonical_staged[27] = produced[72];
        self.canonical_staged[28] = produced[73];
        self.canonical_staged[250] = produced[74];
        self.canonical_staged[253] = produced[75];
        self.canonical_staged[254] = produced[76];
        self.canonical_staged[29] = produced[77];
        self.canonical_staged[257] = produced[78];
        self.canonical_staged[255] = produced[79];
        self.canonical_staged[256] = produced[80];
        self.canonical_staged[30] = produced[81];
        self.canonical_staged[34] = produced[82];
        self.canonical_staged[258] = produced[83];
        self.canonical_staged[259] = produced[84];
        self.canonical_staged[263] = produced[85];
        self.canonical_staged[35] = produced[86];
        self.canonical_staged[36] = produced[87];
        self.canonical_staged[264] = produced[88];
        self.canonical_staged[267] = produced[89];
        self.canonical_staged[268] = produced[90];
        self.canonical_staged[37] = produced[91];
        self.canonical_staged[271] = produced[92];
        self.canonical_staged[269] = produced[93];
        self.canonical_staged[270] = produced[94];
        self.canonical_staged[38] = produced[95];
        self.canonical_staged[42] = produced[96];
        self.canonical_staged[272] = produced[97];
        self.canonical_staged[273] = produced[98];
        self.canonical_staged[277] = produced[99];
        self.canonical_staged[43] = produced[100];
        self.canonical_staged[44] = produced[101];
        self.canonical_staged[278] = produced[102];
        self.canonical_staged[281] = produced[103];
        self.canonical_staged[282] = produced[104];
        self.canonical_staged[45] = produced[105];
        self.canonical_staged[285] = produced[106];
        self.canonical_staged[47] = produced[107];
        self.canonical_staged[289] = produced[108];
        self.canonical_staged[287] = produced[109];
        self.canonical_staged[288] = produced[110];
        self.canonical_staged[48] = produced[111];
        self.canonical_staged[49] = produced[112];
        self.canonical_staged[290] = produced[113];
        self.canonical_staged[291] = produced[114];
        self.canonical_staged[295] = produced[115];
        self.canonical_staged[50] = produced[116];
        self.canonical_staged[51] = produced[117];
        self.canonical_staged[296] = produced[118];
        self.canonical_staged[299] = produced[119];
        self.canonical_staged[300] = produced[120];
        self.canonical_staged[52] = produced[121];
        self.canonical_staged[303] = produced[122];
        self.canonical_staged[301] = produced[123];
        self.canonical_staged[302] = produced[124];
        self.canonical_staged[53] = produced[125];
        self.canonical_staged[54] = produced[126];
        self.canonical_staged[304] = produced[127];
        self.canonical_staged[305] = produced[128];
        self.canonical_staged[309] = produced[129];
        self.canonical_staged[55] = produced[130];
        self.canonical_staged[56] = produced[131];
        self.canonical_staged[310] = produced[132];
        self.canonical_staged[313] = produced[133];
        self.canonical_staged[314] = produced[134];
        self.canonical_staged[57] = produced[135];
        self.canonical_staged[317] = produced[136];
        self.canonical_staged[315] = produced[137];
        self.canonical_staged[316] = produced[138];
        self.canonical_staged[58] = produced[139];
        self.canonical_staged[59] = produced[140];
        self.canonical_staged[318] = produced[141];
        self.canonical_staged[319] = produced[142];
        self.canonical_staged[323] = produced[143];
        self.canonical_staged[60] = produced[144];
        self.canonical_staged[61] = produced[145];
        self.canonical_staged[324] = produced[146];
        self.canonical_staged[327] = produced[147];
        self.canonical_staged[328] = produced[148];
        self.canonical_staged[62] = produced[149];
        self.canonical_staged[331] = produced[150];
        self.canonical_staged[64] = produced[151];
        self.canonical_staged[335] = produced[152];
        self.canonical_staged[333] = produced[153];
        self.canonical_staged[334] = produced[154];
        self.canonical_staged[65] = produced[155];
        self.canonical_staged[66] = produced[156];
        self.canonical_staged[336] = produced[157];
        self.canonical_staged[337] = produced[158];
        self.canonical_staged[341] = produced[159];
        self.canonical_staged[67] = produced[160];
        self.canonical_staged[68] = produced[161];
        self.canonical_staged[342] = produced[162];
        self.canonical_staged[345] = produced[163];
        self.canonical_staged[346] = produced[164];
        self.canonical_staged[69] = produced[165];
        self.canonical_staged[349] = produced[166];
        self.canonical_staged[347] = produced[167];
        self.canonical_staged[348] = produced[168];
        self.canonical_staged[70] = produced[169];
        self.canonical_staged[71] = produced[170];
        self.canonical_staged[350] = produced[171];
        self.canonical_staged[351] = produced[172];
        self.canonical_staged[355] = produced[173];
        self.canonical_staged[72] = produced[174];
        self.canonical_staged[73] = produced[175];
        self.canonical_staged[356] = produced[176];
        self.canonical_staged[359] = produced[177];
        self.canonical_staged[360] = produced[178];
        self.canonical_staged[74] = produced[179];
        self.canonical_staged[363] = produced[180];
        self.canonical_staged[361] = produced[181];
        self.canonical_staged[362] = produced[182];
        self.canonical_staged[75] = produced[183];
        self.canonical_staged[76] = produced[184];
        self.canonical_staged[364] = produced[185];
        self.canonical_staged[365] = produced[186];
        self.canonical_staged[369] = produced[187];
        self.canonical_staged[77] = produced[188];
        self.canonical_staged[78] = produced[189];
        self.canonical_staged[370] = produced[190];
        self.canonical_staged[373] = produced[191];
        self.canonical_staged[374] = produced[192];
        self.canonical_staged[79] = produced[193];
        self.canonical_staged[380] = produced[194];
        self.canonical_staged[378] = produced[195];
        self.canonical_staged[379] = produced[196];
        self.canonical_staged[80] = produced[197];
        self.canonical_staged[81] = produced[198];
        self.canonical_staged[381] = produced[199];
        self.canonical_staged[382] = produced[200];
        self.canonical_staged[386] = produced[201];
        self.canonical_staged[82] = produced[202];
        self.canonical_staged[83] = produced[203];
        self.canonical_staged[387] = produced[204];
        self.canonical_staged[390] = produced[205];
        self.canonical_staged[391] = produced[206];
        self.canonical_staged[84] = produced[207];
        self.canonical_staged[394] = produced[208];
        self.canonical_staged[392] = produced[209];
        self.canonical_staged[393] = produced[210];
        self.canonical_staged[85] = produced[211];
        self.canonical_staged[86] = produced[212];
        self.canonical_staged[395] = produced[213];
        self.canonical_staged[396] = produced[214];
        self.canonical_staged[400] = produced[215];
        self.canonical_staged[87] = produced[216];
        self.canonical_staged[88] = produced[217];
        self.canonical_staged[401] = produced[218];
        self.canonical_staged[404] = produced[219];
        self.canonical_staged[405] = produced[220];
        self.canonical_staged[89] = produced[221];
        self.canonical_staged[408] = produced[222];
        self.canonical_staged[406] = produced[223];
        self.canonical_staged[407] = produced[224];
        self.canonical_staged[90] = produced[225];
        self.canonical_staged[91] = produced[226];
        self.canonical_staged[409] = produced[227];
        self.canonical_staged[410] = produced[228];
        self.canonical_staged[414] = produced[229];
        self.canonical_staged[92] = produced[230];
        self.canonical_staged[93] = produced[231];
        self.canonical_staged[415] = produced[232];
        self.canonical_staged[418] = produced[233];
        self.canonical_staged[419] = produced[234];
        self.canonical_staged[94] = produced[235];
        self.canonical_staged[425] = produced[236];
        self.canonical_staged[423] = produced[237];
        self.canonical_staged[424] = produced[238];
        self.canonical_staged[95] = produced[239];
        self.canonical_staged[96] = produced[240];
        self.canonical_staged[426] = produced[241];
        self.canonical_staged[427] = produced[242];
        self.canonical_staged[431] = produced[243];
        self.canonical_staged[97] = produced[244];
        self.canonical_staged[98] = produced[245];
        self.canonical_staged[432] = produced[246];
        self.canonical_staged[435] = produced[247];
        self.canonical_staged[436] = produced[248];
        self.canonical_staged[99] = produced[249];
        self.canonical_staged[439] = produced[250];
        self.canonical_staged[437] = produced[251];
        self.canonical_staged[438] = produced[252];
        self.canonical_staged[100] = produced[253];
        self.canonical_staged[101] = produced[254];
        self.canonical_staged[440] = produced[255];
        self.canonical_staged[441] = produced[256];
        self.canonical_staged[445] = produced[257];
        self.canonical_staged[102] = produced[258];
        self.canonical_staged[103] = produced[259];
        self.canonical_staged[446] = produced[260];
        self.canonical_staged[449] = produced[261];
        self.canonical_staged[450] = produced[262];
        self.canonical_staged[104] = produced[263];
        self.canonical_staged[453] = produced[264];
        self.canonical_staged[451] = produced[265];
        self.canonical_staged[452] = produced[266];
        self.canonical_staged[105] = produced[267];
        self.canonical_staged[106] = produced[268];
        self.canonical_staged[454] = produced[269];
        self.canonical_staged[455] = produced[270];
        self.canonical_staged[459] = produced[271];
        self.canonical_staged[107] = produced[272];
        self.canonical_staged[108] = produced[273];
        self.canonical_staged[460] = produced[274];
        self.canonical_staged[463] = produced[275];
        self.canonical_staged[464] = produced[276];
        self.canonical_staged[109] = produced[277];
        self.canonical_staged[110] = produced[278];
        self.canonical_staged[473] = produced[279];
        self.canonical_staged[144] = produced[280];
        self.canonical_staged[483] = produced[281];
        self.canonical_staged[481] = produced[282];
        self.canonical_staged[482] = produced[283];
        self.canonical_staged[147] = produced[284];
        self.canonical_staged[151] = produced[285];
        self.canonical_staged[484] = produced[286];
        self.canonical_staged[485] = produced[287];
        self.canonical_staged[486] = produced[288];
        self.canonical_staged[487] = produced[289];
        self.canonical_staged[155] = produced[290];
        self.canonical_staged[488] = produced[291];
        self.canonical_staged[491] = produced[292];
        self.canonical_staged[489] = produced[293];
        self.canonical_staged[490] = produced[294];
        self.canonical_staged[162] = produced[295];
        self.canonical_staged[166] = produced[296];
        self.canonical_staged[492] = produced[297];
        self.canonical_staged[493] = produced[298];
        self.canonical_staged[494] = produced[299];
        self.canonical_staged[495] = produced[300];
        self.canonical_staged[170] = produced[301];
        self.canonical_staged[496] = produced[302];
        self.canonical_staged[499] = produced[303];
        self.canonical_staged[497] = produced[304];
        self.canonical_staged[498] = produced[305];
        self.canonical_staged[176] = produced[306];
        self.canonical_staged[180] = produced[307];
        self.canonical_staged[500] = produced[308];
        self.canonical_staged[501] = produced[309];
        self.canonical_staged[502] = produced[310];
        self.canonical_staged[503] = produced[311];
        self.canonical_staged[184] = produced[312];
        self.canonical_staged[505] = produced[313];
        self.canonical_staged[191] = produced[314];
        self.canonical_staged[506] = produced[315];
        self.canonical_staged[504] = produced[316];
        self.canonical_staged[194] = produced[317];
        self.canonical_staged[195] = produced[318];
        self.canonical_staged[196] = produced[319];
        self.canonical_staged[197] = produced[320];
        self.canonical_staged[198] = produced[321];
        self.canonical_staged[199] = produced[322];
        self.canonical_staged[200] = produced[323];
        self.canonical_staged[201] = produced[324];
        self.canonical_staged[202] = produced[325];
        self.canonical_staged[203] = produced[326];
        self.canonical_staged[204] = produced[327];
        self.canonical_staged[205] = produced[328];
        self.canonical_staged[206] = produced[329];
        self.canonical_staged[207] = produced[330];
        self.canonical_staged[208] = produced[331];
        self.canonical_staged[209] = produced[332];
        self.canonical_instance_valid = true;
    }

    fn canonical_temperature_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        let temperature = ctx.temperature();
        let thermal_voltage = ctx.thermal_voltage();
        if self.canonical_temperature_valid
            && self.canonical_temperature == temperature
            && self.canonical_thermal_voltage == thermal_voltage
        {
            return;
        }
        let produced: [f64; 174] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let v0 = staged[215];
                let v1 = temperature;
                let v2 = parameters[2];
                let v4 = parameters[9];
                let v6 = 2.3149999999999977e1f64;
                let v8 = staged[0];
                let v10 = 8.61726105451295e-5f64;
                let v12 = 1e0f64;
                let v14 = 7.02e-4f64;
                let v18 = 1.108e3f64;
                let v21 = parameters[24];
                let v23 = parameters[25];
                let v25 = parameters[26];
                let v30 = staged[1];
                let v32 = 5e-1f64;
                let v37 = staged[2];
                let v43 = staged[3];
                let v48 = parameters[27];
                let v51 = parameters[28];
                let v54 = parameters[29];
                let v57 = parameters[18];
                let v59 = 2e0f64;
                let v64 = parameters[19];
                let v69 = parameters[20];
                let v74 = 5e-2f64;
                let v100 = parameters[21];
                let v102 = parameters[15];
                let v105 = parameters[22];
                let v107 = parameters[16];
                let v110 = parameters[23];
                let v112 = parameters[17];
                let v115 = staged[4];
                let v118 = staged[5];
                let v121 = staged[6];
                let v137 = staged[7];
                let v140 = 3.1637150399999996e-34f64;
                let v144 = staged[8];
                let v147 = 3.1637150399999996e-34f64;
                let v151 = staged[9];
                let v154 = 3.1637150399999996e-34f64;
                let v157 = parameters[47];
                let v160 = parameters[44];
                let v162 = parameters[48];
                let v165 = parameters[45];
                let v167 = parameters[49];
                let v170 = parameters[46];
                let v172 = 0e0f64;
                let v179 = staged[10];
                let v182 = staged[11];
                let v187 = staged[12];
                let v201 = staged[13];
                let v203 = staged[14];
                let v206 = staged[15];
                let v209 = 0e0f64;
                let v210 = 0e0f64;
                let v211 = 0e0f64;
                let v215 = staged[16];
                let v218 = parameters[12];
                let v223 = 1e8f64;
                let v225 = staged[17];
                let v233 = staged[18];
                let v245 = 2.3025850929940458e2f64;
                let v250 = staged[232];
                let v251 = -2.3025850929940458e2f64;
                let v253 = -2.3025850929940458e2f64;
                let v255 = -2.3025850929940458e2f64;
                let v257 = 3.333333333333333e-1f64;
                let v265 = 1e-100f64;
                let v275 = 1e100f64;
                let v280 = staged[233];
                let v283 = staged[234];
                let v288 = 1e-1f64;
                let v290 = staged[19];
                let v292 = staged[235];
                let v293 = staged[236];
                let v309 = staged[20];
                let v316 = -5e-1f64;
                let v330 = staged[239];
                let v331 = -5e-1f64;
                let v334 = -5e-1f64;
                let v340 = -5e-1f64;
                let v342 = -2.3025850929940458e2f64;
                let v344 = -5e-1f64;
                let v346 = -2.3025850929940458e2f64;
                let v348 = -5e-1f64;
                let v350 = -2.3025850929940458e2f64;
                let v360 = -5e-1f64;
                let v363 = -5e-1f64;
                let v366 = -5e-1f64;
                let v380 = 3e0f64;
                let v399 = staged[21];
                let v406 = 4e0f64;
                let v414 = staged[241];
                let v425 = staged[242];
                let v431 = staged[243];
                let v438 = staged[22];
                let v442 = staged[23];
                let v448 = staged[24];
                let v454 = parameters[30];
                let v456 = staged[25];
                let v460 = 6.66666666666667e-1f64;
                let v470 = staged[244];
                let v472 = staged[245];
                let v478 = staged[26];
                let v485 = 3.75e-1f64;
                let v502 = 5.178164370971076e-1f64;
                let v512 = -2.3025850929940458e2f64;
                let v515 = -2.3025850929940458e2f64;
                let v517 = -2.3025850929940458e2f64;
                let v519 = -2.3025850929940458e2f64;
                let v530 = 2.9214664e-1f64;
                let v533 = 2.6992878119627894e-1f64;
                let v537 = 4.3792457880372104e-1f64;
                let v541 = -2.3025850929940458e2f64;
                let v546 = 8.86226925452758e-1f64;
                let v550 = parameters[35];
                let v553 = -2.3025850929940458e2f64;
                let v555 = -2.3025850929940458e2f64;
                let v557 = -2.3025850929940458e2f64;
                let v572 = staged[27];
                let v579 = staged[28];
                let v581 = parameters[41];
                let v583 = -2.3025850929940458e2f64;
                let v585 = -2.3025850929940458e2f64;
                let v587 = -2.3025850929940458e2f64;
                let v610 = parameters[10];
                let v612 = staged[29];
                let v615 = staged[255];
                let v626 = staged[256];
                let v632 = staged[257];
                let v639 = staged[30];
                let v643 = staged[31];
                let v649 = staged[32];
                let v655 = parameters[31];
                let v657 = staged[33];
                let v670 = staged[258];
                let v672 = staged[259];
                let v678 = staged[34];
                let v710 = -2.3025850929940458e2f64;
                let v713 = -2.3025850929940458e2f64;
                let v715 = -2.3025850929940458e2f64;
                let v717 = -2.3025850929940458e2f64;
                let v736 = -2.3025850929940458e2f64;
                let v741 = 8.86226925452758e-1f64;
                let v745 = parameters[36];
                let v748 = -2.3025850929940458e2f64;
                let v750 = -2.3025850929940458e2f64;
                let v752 = -2.3025850929940458e2f64;
                let v767 = staged[35];
                let v774 = staged[36];
                let v776 = parameters[42];
                let v778 = -2.3025850929940458e2f64;
                let v780 = -2.3025850929940458e2f64;
                let v782 = -2.3025850929940458e2f64;
                let v806 = staged[37];
                let v809 = staged[269];
                let v825 = staged[270];
                let v831 = staged[271];
                let v838 = staged[38];
                let v842 = staged[39];
                let v848 = staged[40];
                let v854 = parameters[32];
                let v856 = staged[41];
                let v869 = staged[272];
                let v871 = staged[273];
                let v877 = staged[42];
                let v909 = -2.3025850929940458e2f64;
                let v912 = -2.3025850929940458e2f64;
                let v914 = -2.3025850929940458e2f64;
                let v916 = -2.3025850929940458e2f64;
                let v935 = -2.3025850929940458e2f64;
                let v940 = 8.86226925452758e-1f64;
                let v944 = parameters[37];
                let v947 = -2.3025850929940458e2f64;
                let v949 = -2.3025850929940458e2f64;
                let v951 = -2.3025850929940458e2f64;
                let v966 = staged[43];
                let v973 = staged[44];
                let v975 = parameters[43];
                let v977 = -2.3025850929940458e2f64;
                let v979 = -2.3025850929940458e2f64;
                let v981 = -2.3025850929940458e2f64;
                let v1005 = staged[45];
                let v1007 = staged[46];
                let v1014 = -5e-1f64;
                let v1028 = staged[285];
                let v1029 = -5e-1f64;
                let v1032 = -5e-1f64;
                let v1038 = -5e-1f64;
                let v1040 = -2.3025850929940458e2f64;
                let v1042 = -5e-1f64;
                let v1044 = -2.3025850929940458e2f64;
                let v1046 = -5e-1f64;
                let v1048 = -2.3025850929940458e2f64;
                let v1058 = -5e-1f64;
                let v1061 = -5e-1f64;
                let v1064 = -5e-1f64;
                let v1096 = staged[47];
                let v1110 = staged[287];
                let v1121 = staged[288];
                let v1127 = staged[289];
                let v1134 = staged[48];
                let v1161 = staged[290];
                let v1163 = staged[291];
                let v1169 = staged[49];
                let v1201 = -2.3025850929940458e2f64;
                let v1204 = -2.3025850929940458e2f64;
                let v1206 = -2.3025850929940458e2f64;
                let v1208 = -2.3025850929940458e2f64;
                let v1227 = -2.3025850929940458e2f64;
                let v1232 = 8.86226925452758e-1f64;
                let v1238 = -2.3025850929940458e2f64;
                let v1240 = -2.3025850929940458e2f64;
                let v1242 = -2.3025850929940458e2f64;
                let v1257 = staged[50];
                let v1264 = staged[51];
                let v1267 = -2.3025850929940458e2f64;
                let v1269 = -2.3025850929940458e2f64;
                let v1271 = -2.3025850929940458e2f64;
                let v1295 = staged[52];
                let v1298 = staged[301];
                let v1309 = staged[302];
                let v1315 = staged[303];
                let v1322 = staged[53];
                let v1349 = staged[304];
                let v1351 = staged[305];
                let v1357 = staged[54];
                let v1389 = -2.3025850929940458e2f64;
                let v1392 = -2.3025850929940458e2f64;
                let v1394 = -2.3025850929940458e2f64;
                let v1396 = -2.3025850929940458e2f64;
                let v1415 = -2.3025850929940458e2f64;
                let v1420 = 8.86226925452758e-1f64;
                let v1426 = -2.3025850929940458e2f64;
                let v1428 = -2.3025850929940458e2f64;
                let v1430 = -2.3025850929940458e2f64;
                let v1445 = staged[55];
                let v1452 = staged[56];
                let v1455 = -2.3025850929940458e2f64;
                let v1457 = -2.3025850929940458e2f64;
                let v1459 = -2.3025850929940458e2f64;
                let v1483 = staged[57];
                let v1486 = staged[315];
                let v1502 = staged[316];
                let v1508 = staged[317];
                let v1515 = staged[58];
                let v1542 = staged[318];
                let v1544 = staged[319];
                let v1550 = staged[59];
                let v1582 = -2.3025850929940458e2f64;
                let v1585 = -2.3025850929940458e2f64;
                let v1587 = -2.3025850929940458e2f64;
                let v1589 = -2.3025850929940458e2f64;
                let v1608 = -2.3025850929940458e2f64;
                let v1613 = 8.86226925452758e-1f64;
                let v1619 = -2.3025850929940458e2f64;
                let v1621 = -2.3025850929940458e2f64;
                let v1623 = -2.3025850929940458e2f64;
                let v1638 = staged[60];
                let v1645 = staged[61];
                let v1648 = -2.3025850929940458e2f64;
                let v1650 = -2.3025850929940458e2f64;
                let v1652 = -2.3025850929940458e2f64;
                let v1676 = staged[62];
                let v1678 = staged[63];
                let v1685 = -5e-1f64;
                let v1699 = staged[331];
                let v1700 = -5e-1f64;
                let v1703 = -5e-1f64;
                let v1709 = -5e-1f64;
                let v1711 = -2.3025850929940458e2f64;
                let v1713 = -5e-1f64;
                let v1715 = -2.3025850929940458e2f64;
                let v1717 = -5e-1f64;
                let v1719 = -2.3025850929940458e2f64;
                let v1729 = -5e-1f64;
                let v1732 = -5e-1f64;
                let v1735 = -5e-1f64;
                let v1767 = staged[64];
                let v1781 = staged[333];
                let v1792 = staged[334];
                let v1798 = staged[335];
                let v1805 = staged[65];
                let v1832 = staged[336];
                let v1834 = staged[337];
                let v1840 = staged[66];
                let v1872 = -2.3025850929940458e2f64;
                let v1875 = -2.3025850929940458e2f64;
                let v1877 = -2.3025850929940458e2f64;
                let v1879 = -2.3025850929940458e2f64;
                let v1898 = -2.3025850929940458e2f64;
                let v1903 = 8.86226925452758e-1f64;
                let v1909 = -2.3025850929940458e2f64;
                let v1911 = -2.3025850929940458e2f64;
                let v1913 = -2.3025850929940458e2f64;
                let v1928 = staged[67];
                let v1935 = staged[68];
                let v1938 = -2.3025850929940458e2f64;
                let v1940 = -2.3025850929940458e2f64;
                let v1942 = -2.3025850929940458e2f64;
                let v1966 = staged[69];
                let v1969 = staged[347];
                let v1980 = staged[348];
                let v1986 = staged[349];
                let v1993 = staged[70];
                let v2020 = staged[350];
                let v2022 = staged[351];
                let v2028 = staged[71];
                let v2060 = -2.3025850929940458e2f64;
                let v2063 = -2.3025850929940458e2f64;
                let v2065 = -2.3025850929940458e2f64;
                let v2067 = -2.3025850929940458e2f64;
                let v2086 = -2.3025850929940458e2f64;
                let v2091 = 8.86226925452758e-1f64;
                let v2097 = -2.3025850929940458e2f64;
                let v2099 = -2.3025850929940458e2f64;
                let v2101 = -2.3025850929940458e2f64;
                let v2116 = staged[72];
                let v2123 = staged[73];
                let v2126 = -2.3025850929940458e2f64;
                let v2128 = -2.3025850929940458e2f64;
                let v2130 = -2.3025850929940458e2f64;
                let v2154 = staged[74];
                let v2157 = staged[361];
                let v2173 = staged[362];
                let v2179 = staged[363];
                let v2186 = staged[75];
                let v2213 = staged[364];
                let v2215 = staged[365];
                let v2221 = staged[76];
                let v2253 = -2.3025850929940458e2f64;
                let v2256 = -2.3025850929940458e2f64;
                let v2258 = -2.3025850929940458e2f64;
                let v2260 = -2.3025850929940458e2f64;
                let v2279 = -2.3025850929940458e2f64;
                let v2284 = 8.86226925452758e-1f64;
                let v2290 = -2.3025850929940458e2f64;
                let v2292 = -2.3025850929940458e2f64;
                let v2294 = -2.3025850929940458e2f64;
                let v2309 = staged[77];
                let v2316 = staged[78];
                let v2319 = -2.3025850929940458e2f64;
                let v2321 = -2.3025850929940458e2f64;
                let v2323 = -2.3025850929940458e2f64;
                let v2347 = staged[79];
                let v2355 = -5e-1f64;
                let v2369 = 1.0f64;
                let v2370 = -5e-1f64;
                let v2373 = -5e-1f64;
                let v2379 = -5e-1f64;
                let v2381 = -2.3025850929940458e2f64;
                let v2383 = -5e-1f64;
                let v2385 = -2.3025850929940458e2f64;
                let v2387 = -5e-1f64;
                let v2389 = -2.3025850929940458e2f64;
                let v2399 = -5e-1f64;
                let v2402 = -5e-1f64;
                let v2405 = -5e-1f64;
                let v2437 = -1e-1f64;
                let v2451 = staged[378];
                let v2462 = staged[379];
                let v2468 = staged[380];
                let v2475 = staged[80];
                let v2502 = staged[381];
                let v2504 = staged[382];
                let v2510 = staged[81];
                let v2542 = -2.3025850929940458e2f64;
                let v2545 = -2.3025850929940458e2f64;
                let v2547 = -2.3025850929940458e2f64;
                let v2549 = -2.3025850929940458e2f64;
                let v2568 = -2.3025850929940458e2f64;
                let v2573 = 8.86226925452758e-1f64;
                let v2579 = -2.3025850929940458e2f64;
                let v2581 = -2.3025850929940458e2f64;
                let v2583 = -2.3025850929940458e2f64;
                let v2598 = staged[82];
                let v2605 = staged[83];
                let v2608 = -2.3025850929940458e2f64;
                let v2610 = -2.3025850929940458e2f64;
                let v2612 = -2.3025850929940458e2f64;
                let v2636 = staged[84];
                let v2639 = staged[392];
                let v2650 = staged[393];
                let v2656 = staged[394];
                let v2663 = staged[85];
                let v2690 = staged[395];
                let v2692 = staged[396];
                let v2698 = staged[86];
                let v2730 = -2.3025850929940458e2f64;
                let v2733 = -2.3025850929940458e2f64;
                let v2735 = -2.3025850929940458e2f64;
                let v2737 = -2.3025850929940458e2f64;
                let v2756 = -2.3025850929940458e2f64;
                let v2761 = 8.86226925452758e-1f64;
                let v2767 = -2.3025850929940458e2f64;
                let v2769 = -2.3025850929940458e2f64;
                let v2771 = -2.3025850929940458e2f64;
                let v2786 = staged[87];
                let v2793 = staged[88];
                let v2796 = -2.3025850929940458e2f64;
                let v2798 = -2.3025850929940458e2f64;
                let v2800 = -2.3025850929940458e2f64;
                let v2824 = staged[89];
                let v2827 = staged[406];
                let v2843 = staged[407];
                let v2849 = staged[408];
                let v2856 = staged[90];
                let v2883 = staged[409];
                let v2885 = staged[410];
                let v2891 = staged[91];
                let v2923 = -2.3025850929940458e2f64;
                let v2926 = -2.3025850929940458e2f64;
                let v2928 = -2.3025850929940458e2f64;
                let v2930 = -2.3025850929940458e2f64;
                let v2949 = -2.3025850929940458e2f64;
                let v2954 = 8.86226925452758e-1f64;
                let v2960 = -2.3025850929940458e2f64;
                let v2962 = -2.3025850929940458e2f64;
                let v2964 = -2.3025850929940458e2f64;
                let v2979 = staged[92];
                let v2986 = staged[93];
                let v2989 = -2.3025850929940458e2f64;
                let v2991 = -2.3025850929940458e2f64;
                let v2993 = -2.3025850929940458e2f64;
                let v3017 = staged[94];
                let v3019 = 2e-1f64;
                let v3026 = -5e-1f64;
                let v3040 = 1.0f64;
                let v3041 = -5e-1f64;
                let v3044 = -5e-1f64;
                let v3050 = -5e-1f64;
                let v3052 = -2.3025850929940458e2f64;
                let v3054 = -5e-1f64;
                let v3056 = -2.3025850929940458e2f64;
                let v3058 = -5e-1f64;
                let v3060 = -2.3025850929940458e2f64;
                let v3070 = -5e-1f64;
                let v3073 = -5e-1f64;
                let v3076 = -5e-1f64;
                let v3108 = -2e-1f64;
                let v3122 = staged[423];
                let v3133 = staged[424];
                let v3139 = staged[425];
                let v3146 = staged[95];
                let v3173 = staged[426];
                let v3175 = staged[427];
                let v3181 = staged[96];
                let v3213 = -2.3025850929940458e2f64;
                let v3216 = -2.3025850929940458e2f64;
                let v3218 = -2.3025850929940458e2f64;
                let v3220 = -2.3025850929940458e2f64;
                let v3239 = -2.3025850929940458e2f64;
                let v3244 = 8.86226925452758e-1f64;
                let v3250 = -2.3025850929940458e2f64;
                let v3252 = -2.3025850929940458e2f64;
                let v3254 = -2.3025850929940458e2f64;
                let v3269 = staged[97];
                let v3276 = staged[98];
                let v3279 = -2.3025850929940458e2f64;
                let v3281 = -2.3025850929940458e2f64;
                let v3283 = -2.3025850929940458e2f64;
                let v3307 = staged[99];
                let v3310 = staged[437];
                let v3321 = staged[438];
                let v3327 = staged[439];
                let v3334 = staged[100];
                let v3361 = staged[440];
                let v3363 = staged[441];
                let v3369 = staged[101];
                let v3401 = -2.3025850929940458e2f64;
                let v3404 = -2.3025850929940458e2f64;
                let v3406 = -2.3025850929940458e2f64;
                let v3408 = -2.3025850929940458e2f64;
                let v3427 = -2.3025850929940458e2f64;
                let v3432 = 8.86226925452758e-1f64;
                let v3438 = -2.3025850929940458e2f64;
                let v3440 = -2.3025850929940458e2f64;
                let v3442 = -2.3025850929940458e2f64;
                let v3457 = staged[102];
                let v3464 = staged[103];
                let v3467 = -2.3025850929940458e2f64;
                let v3469 = -2.3025850929940458e2f64;
                let v3471 = -2.3025850929940458e2f64;
                let v3495 = staged[104];
                let v3498 = staged[451];
                let v3522 = staged[452];
                let v3528 = staged[453];
                let v3535 = staged[105];
                let v3562 = staged[454];
                let v3564 = staged[455];
                let v3570 = staged[106];
                let v3602 = -2.3025850929940458e2f64;
                let v3605 = -2.3025850929940458e2f64;
                let v3607 = -2.3025850929940458e2f64;
                let v3609 = -2.3025850929940458e2f64;
                let v3628 = -2.3025850929940458e2f64;
                let v3633 = 8.86226925452758e-1f64;
                let v3639 = -2.3025850929940458e2f64;
                let v3641 = -2.3025850929940458e2f64;
                let v3643 = -2.3025850929940458e2f64;
                let v3658 = staged[107];
                let v3665 = staged[108];
                let v3668 = -2.3025850929940458e2f64;
                let v3670 = -2.3025850929940458e2f64;
                let v3672 = -2.3025850929940458e2f64;
                let v3696 = staged[109];
                let v3711 = parameters[64];
                let v3715 = 1e-3f64;
                let v3766 = -1e-1f64;
                let v3814 = 1e-6f64;
                let v3824 = -5e-1f64;
                let v3844 = 1e-21f64;
                let v3846 = staged[110];
                let v3865 = staged[473];
                let v3875 = 1e0f64;
                let v3888 = staged[483];
                let v3890 = staged[485];
                let v3892 = staged[491];
                let v3894 = staged[493];
                let v3896 = staged[499];
                let v3898 = staged[501];
                let mut out248: f64 = 0.0;
                let mut out310: f64 = 0.0;
                let mut out319: f64 = 0.0;
                let mut out336: f64 = 0.0;
                let mut out501: f64 = 0.0;
                let mut out513: f64 = 0.0;
                let mut out542: f64 = 0.0;
                let mut out575: f64 = 0.0;
                let mut out577: f64 = 0.0;
                let mut out700: f64 = 0.0;
                let mut out711: f64 = 0.0;
                let mut out737: f64 = 0.0;
                let mut out770: f64 = 0.0;
                let mut out772: f64 = 0.0;
                let mut out899: f64 = 0.0;
                let mut out910: f64 = 0.0;
                let mut out936: f64 = 0.0;
                let mut out969: f64 = 0.0;
                let mut out971: f64 = 0.0;
                let mut out1008: f64 = 0.0;
                let mut out1017: f64 = 0.0;
                let mut out1034: f64 = 0.0;
                let mut out1191: f64 = 0.0;
                let mut out1202: f64 = 0.0;
                let mut out1228: f64 = 0.0;
                let mut out1260: f64 = 0.0;
                let mut out1262: f64 = 0.0;
                let mut out1379: f64 = 0.0;
                let mut out1390: f64 = 0.0;
                let mut out1416: f64 = 0.0;
                let mut out1448: f64 = 0.0;
                let mut out1450: f64 = 0.0;
                let mut out1572: f64 = 0.0;
                let mut out1583: f64 = 0.0;
                let mut out1609: f64 = 0.0;
                let mut out1641: f64 = 0.0;
                let mut out1643: f64 = 0.0;
                let mut out1679: f64 = 0.0;
                let mut out1688: f64 = 0.0;
                let mut out1705: f64 = 0.0;
                let mut out1862: f64 = 0.0;
                let mut out1873: f64 = 0.0;
                let mut out1899: f64 = 0.0;
                let mut out1931: f64 = 0.0;
                let mut out1933: f64 = 0.0;
                let mut out2050: f64 = 0.0;
                let mut out2061: f64 = 0.0;
                let mut out2087: f64 = 0.0;
                let mut out2119: f64 = 0.0;
                let mut out2121: f64 = 0.0;
                let mut out2243: f64 = 0.0;
                let mut out2254: f64 = 0.0;
                let mut out2280: f64 = 0.0;
                let mut out2312: f64 = 0.0;
                let mut out2314: f64 = 0.0;
                let mut out2349: f64 = 0.0;
                let mut out2358: f64 = 0.0;
                let mut out2375: f64 = 0.0;
                let mut out2532: f64 = 0.0;
                let mut out2543: f64 = 0.0;
                let mut out2569: f64 = 0.0;
                let mut out2601: f64 = 0.0;
                let mut out2603: f64 = 0.0;
                let mut out2720: f64 = 0.0;
                let mut out2731: f64 = 0.0;
                let mut out2757: f64 = 0.0;
                let mut out2789: f64 = 0.0;
                let mut out2791: f64 = 0.0;
                let mut out2913: f64 = 0.0;
                let mut out2924: f64 = 0.0;
                let mut out2950: f64 = 0.0;
                let mut out2982: f64 = 0.0;
                let mut out2984: f64 = 0.0;
                let mut out3020: f64 = 0.0;
                let mut out3029: f64 = 0.0;
                let mut out3046: f64 = 0.0;
                let mut out3203: f64 = 0.0;
                let mut out3214: f64 = 0.0;
                let mut out3240: f64 = 0.0;
                let mut out3272: f64 = 0.0;
                let mut out3274: f64 = 0.0;
                let mut out3391: f64 = 0.0;
                let mut out3402: f64 = 0.0;
                let mut out3428: f64 = 0.0;
                let mut out3460: f64 = 0.0;
                let mut out3462: f64 = 0.0;
                let mut out3592: f64 = 0.0;
                let mut out3603: f64 = 0.0;
                let mut out3629: f64 = 0.0;
                let mut out3661: f64 = 0.0;
                let mut out3663: f64 = 0.0;
                let mut out3700: f64 = 0.0;
                let mut out3713: f64 = 0.0;
                let mut out3725: f64 = 0.0;
                let mut out3762: f64 = 0.0;
                let mut out3787: f64 = 0.0;
                let mut out3815: f64 = 0.0;
                let mut out3840: f64 = 0.0;
                let mut out3842: f64 = 0.0;
                let mut out3866: f64 = 0.0;
                let mut out3868: f64 = 0.0;
                let mut out3870: f64 = 0.0;
                let mut out3871: f64 = 0.0;
                let mut out3872: f64 = 0.0;
                let mut out3873: f64 = 0.0;
                let mut out3874: f64 = 0.0;
                let mut out3876: f64 = 0.0;
                let mut out3877: f64 = 0.0;
                let mut out3878: f64 = 0.0;
                let mut out3879: f64 = 0.0;
                let mut out3880: f64 = 0.0;
                let mut out3881: f64 = 0.0;
                let mut out3883: f64 = 0.0;
                let mut out3885: f64 = 0.0;
                let mut out3887: f64 = 0.0;
                let mut out3889: f64 = 0.0;
                let mut out3891: f64 = 0.0;
                let mut out3893: f64 = 0.0;
                let mut out3895: f64 = 0.0;
                let mut out3897: f64 = 0.0;
                let mut out3899: f64 = 0.0;
                let mut out3901: f64 = 0.0;
                let mut out3903: f64 = 0.0;
                let v7 = if ((v1 + v2) + v4) >= v6 { ((v1 + v2) + v4) } else { v6 };
                let v9 = v7 / v8;
                let v11 = v10 * v7;
                let v13 = v12 / v11;
                let v20 = (-((v14 * v7) * v7)) / (v18 + v7);
                let v22 = v21 + v20;
                let v24 = v23 + v20;
                let v26 = v25 + v20;
                let v28 = v9 * (v9.sqrt());
                let v35 = v28 * ((v32 * (v30 - (v22 * v13))).exp());
                let v41 = v28 * ((v32 * (v37 - (v24 * v13))).exp());
                let v47 = v28 * ((v32 * (v43 - (v26 * v13))).exp());
                let v50 = (v48 * v35) * v35;
                let v53 = (v51 * v41) * v41;
                let v56 = (v54 * v47) * v47;
                let v60 = v59 * v11;
                let v63 = (v57 * v9) - (v60 * (v35.ln()));
                let v68 = (v64 * v9) - (v60 * (v41.ln()));
                let v73 = (v69 * v9) - (v60 * (v47.ln()));
                let v81 = v63 + (v11 * ((v12 + (((v74 - v63) * v13).exp())).ln()));
                let v88 = v68 + (v11 * ((v12 + (((v74 - v68) * v13).exp())).ln()));
                let v95 = v73 + (v11 * ((v12 + (((v74 - v73) * v13).exp())).ln()));
                let v96 = v12 / v81;
                let v97 = v12 / v88;
                let v98 = v12 / v95;
                let v103 = v102 * ((v57 * v96).powf(v100));
                let v108 = v107 * ((v64 * v97).powf(v105));
                let v113 = v112 * ((v69 * v98).powf(v110));
                let v116 = (v103 * v81) * v115;
                let v119 = (v108 * v88) * v118;
                let v122 = (v113 * v95) * v121;
                let v123 = v59 * v103;
                let v124 = v59 * v108;
                let v125 = v59 * v113;
                let v127 = if (v32 * v22) >= v11 { (v32 * v22) } else { v11 };
                let v129 = if (v32 * v24) >= v11 { (v32 * v24) } else { v11 };
                let v131 = if (v32 * v26) >= v11 { (v32 * v26) } else { v11 };
                let v132 = v127 * v13;
                let v133 = v129 * v13;
                let v134 = v131 * v13;
                let v141 = ((v137 * ((v127 * v127) * v127)).sqrt()) / v140;
                let v148 = ((v144 * ((v129 * v129) * v129)).sqrt()) / v147;
                let v155 = ((v151 * ((v131 * v131) * v131)).sqrt()) / v154;
                let v156 = v7 - v8;
                let v161 = v160 * (v12 + (v157 * v156));
                let v166 = v165 * (v12 + (v162 * v156));
                let v171 = v170 * (v12 + (v167 * v156));
                let v173 = if v161 > v172 { 1.0 } else { 0.0 };
                let v174: f64;
                if v173 != 0.0 {
                    v174 = v161;
                } else {
                    v174 = v172;
                }
                let v175 = if v166 > v172 { 1.0 } else { 0.0 };
                let v176: f64;
                if v175 != 0.0 {
                    v176 = v166;
                } else {
                    v176 = v172;
                }
                let v177 = if v171 > v172 { 1.0 } else { 0.0 };
                let v178: f64;
                if v177 != 0.0 {
                    v178 = v171;
                } else {
                    v178 = v172;
                }
                let v212: f64;
                let v213: f64;
                let v214: f64;
                if v0 != 0.0 {
                    let v191 = (v187 * v9) - (v60 * ((v28 * ((v32 * (v182 - ((v179 + v20) * v13))).exp())).ln()));
                    let v198 = v191 + (v11 * ((v12 + (((v74 - v191) * v13).exp())).ln()));
                    let v199 = v12 / v198;
                    let v204 = v203 * ((v187 * v199).powf(v201));
                    let v207 = (v204 * v198) * v206;
                    let v208 = v59 * v204;
                    v212 = v199;
                    v213 = v207;
                    v214 = v208;
                } else {
                    v212 = v209;
                    v213 = v210;
                    v214 = v211;
                }
                let v216 = v50 * v215;
                let v217 = if v216 > v172 { 1.0 } else { 0.0 };
                let v224: f64;
                if v217 != 0.0 {
                    let v222 = v11 * (((v218 / v216) + v12).ln());
                    v224 = v222;
                } else {
                    v224 = v223;
                }
                let v226 = v53 * v225;
                let v227 = if v226 > v172 { 1.0 } else { 0.0 };
                let v232: f64;
                if v227 != 0.0 {
                    let v231 = v11 * (((v218 / v226) + v12).ln());
                    v232 = v231;
                } else {
                    v232 = v223;
                }
                let v234 = v56 * v233;
                let v235 = if v234 > v172 { 1.0 } else { 0.0 };
                let v240: f64;
                if v235 != 0.0 {
                    let v239 = v11 * (((v218 / v234) + v12).ln());
                    v240 = v239;
                } else {
                    v240 = v223;
                }
                let v242 = if (if v224 <= v232 { v224 } else { v232 }) <= v240 { (if v224 <= v232 { v224 } else { v232 }) } else { v240 };
                let v243 = v242 * v13;
                let v246 = if (v243.abs()) < v245 { 1.0 } else { 0.0 };
                let v249: f64;
                if v246 != 0.0 {
                    let v247 = v243.exp();
                    v249 = v247;
                } else {
                    let v248 = if v243 < v172 { 1.0 } else { 0.0 };
                    out248 = v248;
                    let v277: f64;
                    if v248 != 0.0 {
                        let v266 = v265 / (v12 + ((v251 - v243) * (v12 + (v32 * ((v253 - v243) * (v12 + ((v255 - v243) * v257)))))));
                        v277 = v266;
                    } else {
                        let v267 = v243 - v245;
                        let v276 = v275 * (v12 + (v267 * (v12 + (v32 * (v267 * (v12 + (v267 * v257)))))));
                        v277 = v276;
                    }
                    v249 = v277;
                }
                let v279: f64;
                if v250 != 0.0 {
                    let v278 = v88 + v95;
                    v279 = v278;
                } else {
                    v279 = v81;
                }
                let v282: f64;
                if v280 != 0.0 {
                    let v281 = v81 + v95;
                    v282 = v281;
                } else {
                    v282 = v88;
                }
                let v285: f64;
                if v283 != 0.0 {
                    let v284 = v81 + v88;
                    v285 = v284;
                } else {
                    v285 = v95;
                }
                let v287 = if (if v279 <= v282 { v279 } else { v282 }) <= v285 { (if v279 <= v282 { v279 } else { v282 }) } else { v285 };
                let v289 = v287 * v288;
                let v291 = v287 * v290;
                let v294: f64;
                let v295: f64;
                let v296: f64;
                let v297: f64;
                let v298: f64;
                let v299: f64;
                let v300: f64;
                let v301: f64;
                let v302: f64;
                let v303: f64;
                let v304: f64;
                let v305: f64;
                let v306: f64;
                let v307: f64;
                let v308: f64;
                if v292 != 0.0 {
                    let v311: f64;
                    let v312: f64;
                    let v313: f64;
                    let v314: f64;
                    if v293 != 0.0 {
                        let v310 = if v309 < v242 { 1.0 } else { 0.0 };
                        out310 = v310;
                        let v326: f64;
                        let v327: f64;
                        let v328: f64;
                        if v310 != 0.0 {
                            let v315 = v309 * v13;
                            let v319 = if ((v316 * v315).abs()) < v245 { 1.0 } else { 0.0 };
                            out319 = v319;
                            let v337: f64;
                            if v319 != 0.0 {
                                let v333 = (v331 * v315).exp();
                                v337 = v333;
                            } else {
                                let v336 = if (v334 * v315) < v172 { 1.0 } else { 0.0 };
                                out336 = v336;
                                let v377: f64;
                                if v336 != 0.0 {
                                    let v359 = v265 / (v12 + ((v342 - (v340 * v315)) * (v12 + (v32 * ((v346 - (v344 * v315)) * (v12 + ((v350 - (v348 * v315)) * v257)))))));
                                    v377 = v359;
                                } else {
                                    let v376 = v275 * (v12 + (((v360 * v315) - v245) * (v12 + (v32 * (((v363 * v315) - v245) * (v12 + (((v366 * v315) - v245) * v257)))))));
                                    v377 = v376;
                                }
                                v337 = v377;
                            }
                            let v338 = v12 / v337;
                            let v339 = v338 * v338;
                            v326 = v339;
                            v327 = v337;
                            v328 = v338;
                        } else {
                            let v323 = (v12 + ((v309 - v242) * v13)) * v249;
                            let v324 = v323.sqrt();
                            let v325 = v12 / v324;
                            v326 = v323;
                            v327 = v325;
                            v328 = v324;
                        }
                        let v329 = v326 - v12;
                        let v401: f64;
                        if v330 != 0.0 {
                            let v387 = v59 * (v11 * (((v59 + v327) + (((v327 + v12) * (v327 + v380)).sqrt())).ln()));
                            v401 = v387;
                        } else {
                            let v400 = v399 + (v59 * (v11 * ((((v59 * v328) + v12) + (((v12 + v328) * (v12 + (v380 * v328))).sqrt())).ln())));
                            v401 = v400;
                        }
                        let v402 = v287 - v401;
                        let v404 = v309 - v402;
                        let v412 = v32 * ((v309 + v402) - (((v404 * v404) + ((v406 * v11) * v11)).sqrt()));
                        v311 = v329;
                        v312 = v412;
                        v313 = v401;
                        v314 = v328;
                    } else {
                        v311 = v172;
                        v312 = v172;
                        v313 = v172;
                        v314 = v172;
                    }
                    let v415: f64;
                    let v416: f64;
                    let v417: f64;
                    let v418: f64;
                    let v419: f64;
                    if v250 != 0.0 {
                        v415 = v172;
                        v416 = v172;
                        v417 = v172;
                        v418 = v172;
                        v419 = v172;
                    } else {
                        let v413 = v50 * v311;
                        let v426: f64;
                        let v427: f64;
                        let v428: f64;
                        let v429: f64;
                        let v430: f64;
                        if v414 != 0.0 {
                            v426 = v172;
                            v427 = v172;
                            v428 = v172;
                            v429 = v172;
                            v430 = v172;
                        } else {
                            let v420 = v81 - v312;
                            let v424 = v12 - ((v12 - (v313 / v420)).sqrt());
                            let v440: f64;
                            if v425 != 0.0 {
                                v440 = v172;
                            } else {
                                let v439 = ((((v424 * v424) * (v424.ln())) / (v12 - v424)) + v424) * v438;
                                v440 = v439;
                            }
                            let v441 = v424 + v440;
                            let v447: f64;
                            if v425 != 0.0 {
                                let v444 = (v420 * v442).sqrt();
                                v447 = v444;
                            } else {
                                let v446 = (v420 * v442).powf(v100);
                                v447 = v446;
                            }
                            let v449 = v448 * v447;
                            let v452 = v35 * ((v314 - v12) * v449);
                            let v455 = v454 * (v452 * v441);
                            v426 = v449;
                            v427 = v420;
                            v428 = v441;
                            v429 = v452;
                            v430 = v455;
                        }
                        let v471: f64;
                        if v431 != 0.0 {
                            v471 = v172;
                        } else {
                            let v459 = v141 * ((v426 * v456) / v427);
                            let v462 = (v460 * v132) / v459;
                            let v463 = v462 * v462;
                            let v464 = v463 * v463;
                            let v467 = (v464 / (v464 + v12)).sqrt();
                            let v468 = v467.sqrt();
                            let v469 = v467 * v468;
                            let v480: f64;
                            if v470 != 0.0 {
                                let v475 = v12 / (v12 + (v459 * v469));
                                v480 = v475;
                            } else {
                                let v479 = (v12 + (v459 * v469)).powf(v478);
                                v480 = v479;
                            }
                            let v483 = (v428 * v480) / (v428 + v480);
                            let v487 = (v485 * (v459 / v468)).sqrt();
                            let v497 = (((v132 * v462) * v468) - (v132 * v467)) + (v32 * (v459 * v469));
                            let v499 = (((v59 * (v462 * v468)) - v467) - v12) * v487;
                            let v500 = v499 * v499;
                            let v501 = if v499 > v172 { 1.0 } else { 0.0 };
                            out501 = v501;
                            let v509: f64;
                            if v501 != 0.0 {
                                let v505 = v12 / (v12 + (v502 * v499));
                                v509 = v505;
                            } else {
                                let v508 = v12 / (v12 - (v502 * v499));
                                v509 = v508;
                            }
                            let v511 = (-v500) + v497;
                            let v513 = if v511 > v512 { 1.0 } else { 0.0 };
                            out513 = v513;
                            let v529: f64;
                            if v513 != 0.0 {
                                let v514 = v511.exp();
                                v529 = v514;
                            } else {
                                let v528 = v265 / (v12 + ((v515 - v511) * (v12 + (v32 * ((v517 - v511) * (v12 + ((v519 - v511) * v257)))))));
                                v529 = v528;
                            }
                            let v532 = v509 * v509;
                            let v540 = (((v530 * v509) + (v533 * v532)) + (v537 * (v532 * v509))) * v529;
                            let v543: f64;
                            if v501 != 0.0 {
                                v543 = v540;
                            } else {
                                let v542 = if v497 > v541 { 1.0 } else { 0.0 };
                                out542 = v542;
                                let v567: f64;
                                if v542 != 0.0 {
                                    let v552 = v497.exp();
                                    v567 = v552;
                                } else {
                                    let v566 = v265 / (v12 + ((v553 - v497) * (v12 + (v32 * ((v555 - v497) * (v12 + ((v557 - v497) * v257)))))));
                                    v567 = v566;
                                }
                                let v569 = (v59 * v567) - v540;
                                v543 = v569;
                            }
                            let v551 = v550 * ((v429 * (v546 * ((v132 * v543) / v487))) * v483);
                            v471 = v551;
                        }
                        let v570: f64;
                        if v472 != 0.0 {
                            v570 = v172;
                        } else {
                            let v573 = (-v174) / v572;
                            let v575 = if (v573.abs()) < v245 { 1.0 } else { 0.0 };
                            out575 = v575;
                            let v578: f64;
                            if v575 != 0.0 {
                                let v576 = v573.exp();
                                v578 = v576;
                            } else {
                                let v577 = if v573 < v172 { 1.0 } else { 0.0 };
                                out577 = v577;
                                let v606: f64;
                                if v577 != 0.0 {
                                    let v596 = v265 / (v12 + ((v583 - v573) * (v12 + (v32 * ((v585 - v573) * (v12 + ((v587 - v573) * v257)))))));
                                    v606 = v596;
                                } else {
                                    let v597 = v573 - v245;
                                    let v605 = v275 * (v12 + (v597 * (v12 + (v32 * (v597 * (v12 + (v597 * v257)))))));
                                    v606 = v605;
                                }
                                v578 = v606;
                            }
                            let v582 = v581 * (v579 * v578);
                            v570 = v582;
                        }
                        let v613 = (v610 * (((v413 + v430) + v471) + v570)) * v612;
                        v415 = v426;
                        v416 = v427;
                        v417 = v428;
                        v418 = v429;
                        v419 = v613;
                    }
                    let v616: f64;
                    let v617: f64;
                    let v618: f64;
                    let v619: f64;
                    let v620: f64;
                    if v280 != 0.0 {
                        v616 = v415;
                        v617 = v416;
                        v618 = v417;
                        v619 = v418;
                        v620 = v172;
                    } else {
                        let v614 = v53 * v311;
                        let v627: f64;
                        let v628: f64;
                        let v629: f64;
                        let v630: f64;
                        let v631: f64;
                        if v615 != 0.0 {
                            v627 = v415;
                            v628 = v416;
                            v629 = v417;
                            v630 = v418;
                            v631 = v172;
                        } else {
                            let v621 = v88 - v312;
                            let v625 = v12 - ((v12 - (v313 / v621)).sqrt());
                            let v641: f64;
                            if v626 != 0.0 {
                                v641 = v172;
                            } else {
                                let v640 = ((((v625 * v625) * (v625.ln())) / (v12 - v625)) + v625) * v639;
                                v641 = v640;
                            }
                            let v642 = v625 + v641;
                            let v648: f64;
                            if v626 != 0.0 {
                                let v645 = (v621 * v643).sqrt();
                                v648 = v645;
                            } else {
                                let v647 = (v621 * v643).powf(v105);
                                v648 = v647;
                            }
                            let v650 = v649 * v648;
                            let v653 = v41 * ((v314 - v12) * v650);
                            let v656 = v655 * (v653 * v642);
                            v627 = v650;
                            v628 = v621;
                            v629 = v642;
                            v630 = v653;
                            v631 = v656;
                        }
                        let v671: f64;
                        if v632 != 0.0 {
                            v671 = v172;
                        } else {
                            let v660 = v148 * ((v627 * v657) / v628);
                            let v662 = (v460 * v133) / v660;
                            let v663 = v662 * v662;
                            let v664 = v663 * v663;
                            let v667 = (v664 / (v664 + v12)).sqrt();
                            let v668 = v667.sqrt();
                            let v669 = v667 * v668;
                            let v680: f64;
                            if v670 != 0.0 {
                                let v675 = v12 / (v12 + (v660 * v669));
                                v680 = v675;
                            } else {
                                let v679 = (v12 + (v660 * v669)).powf(v678);
                                v680 = v679;
                            }
                            let v683 = (v629 * v680) / (v629 + v680);
                            let v686 = (v485 * (v660 / v668)).sqrt();
                            let v696 = (((v133 * v662) * v668) - (v133 * v667)) + (v32 * (v660 * v669));
                            let v698 = (((v59 * (v662 * v668)) - v667) - v12) * v686;
                            let v699 = v698 * v698;
                            let v700 = if v698 > v172 { 1.0 } else { 0.0 };
                            out700 = v700;
                            let v707: f64;
                            if v700 != 0.0 {
                                let v703 = v12 / (v12 + (v502 * v698));
                                v707 = v703;
                            } else {
                                let v706 = v12 / (v12 - (v502 * v698));
                                v707 = v706;
                            }
                            let v709 = (-v699) + v696;
                            let v711 = if v709 > v710 { 1.0 } else { 0.0 };
                            out711 = v711;
                            let v727: f64;
                            if v711 != 0.0 {
                                let v712 = v709.exp();
                                v727 = v712;
                            } else {
                                let v726 = v265 / (v12 + ((v713 - v709) * (v12 + (v32 * ((v715 - v709) * (v12 + ((v717 - v709) * v257)))))));
                                v727 = v726;
                            }
                            let v729 = v707 * v707;
                            let v735 = (((v530 * v707) + (v533 * v729)) + (v537 * (v729 * v707))) * v727;
                            let v738: f64;
                            if v700 != 0.0 {
                                v738 = v735;
                            } else {
                                let v737 = if v696 > v736 { 1.0 } else { 0.0 };
                                out737 = v737;
                                let v762: f64;
                                if v737 != 0.0 {
                                    let v747 = v696.exp();
                                    v762 = v747;
                                } else {
                                    let v761 = v265 / (v12 + ((v748 - v696) * (v12 + (v32 * ((v750 - v696) * (v12 + ((v752 - v696) * v257)))))));
                                    v762 = v761;
                                }
                                let v764 = (v59 * v762) - v735;
                                v738 = v764;
                            }
                            let v746 = v745 * ((v630 * (v741 * ((v133 * v738) / v686))) * v683);
                            v671 = v746;
                        }
                        let v765: f64;
                        if v672 != 0.0 {
                            v765 = v172;
                        } else {
                            let v768 = (-v176) / v767;
                            let v770 = if (v768.abs()) < v245 { 1.0 } else { 0.0 };
                            out770 = v770;
                            let v773: f64;
                            if v770 != 0.0 {
                                let v771 = v768.exp();
                                v773 = v771;
                            } else {
                                let v772 = if v768 < v172 { 1.0 } else { 0.0 };
                                out772 = v772;
                                let v801: f64;
                                if v772 != 0.0 {
                                    let v791 = v265 / (v12 + ((v778 - v768) * (v12 + (v32 * ((v780 - v768) * (v12 + ((v782 - v768) * v257)))))));
                                    v801 = v791;
                                } else {
                                    let v792 = v768 - v245;
                                    let v800 = v275 * (v12 + (v792 * (v12 + (v32 * (v792 * (v12 + (v792 * v257)))))));
                                    v801 = v800;
                                }
                                v773 = v801;
                            }
                            let v777 = v776 * (v774 * v773);
                            v765 = v777;
                        }
                        let v807 = (v610 * (((v614 + v631) + v671) + v765)) * v806;
                        v616 = v627;
                        v617 = v628;
                        v618 = v629;
                        v619 = v630;
                        v620 = v807;
                    }
                    let v810: f64;
                    let v811: f64;
                    let v812: f64;
                    let v813: f64;
                    let v814: f64;
                    if v283 != 0.0 {
                        v810 = v172;
                        v811 = v616;
                        v812 = v617;
                        v813 = v618;
                        v814 = v619;
                    } else {
                        let v808 = v56 * v311;
                        let v826: f64;
                        let v827: f64;
                        let v828: f64;
                        let v829: f64;
                        let v830: f64;
                        if v809 != 0.0 {
                            v826 = v616;
                            v827 = v617;
                            v828 = v618;
                            v829 = v619;
                            v830 = v172;
                        } else {
                            let v820 = v95 - v312;
                            let v824 = v12 - ((v12 - (v313 / v820)).sqrt());
                            let v840: f64;
                            if v825 != 0.0 {
                                v840 = v172;
                            } else {
                                let v839 = ((((v824 * v824) * (v824.ln())) / (v12 - v824)) + v824) * v838;
                                v840 = v839;
                            }
                            let v841 = v824 + v840;
                            let v847: f64;
                            if v825 != 0.0 {
                                let v844 = (v820 * v842).sqrt();
                                v847 = v844;
                            } else {
                                let v846 = (v820 * v842).powf(v110);
                                v847 = v846;
                            }
                            let v849 = v848 * v847;
                            let v852 = v47 * ((v314 - v12) * v849);
                            let v855 = v854 * (v852 * v841);
                            v826 = v849;
                            v827 = v820;
                            v828 = v841;
                            v829 = v852;
                            v830 = v855;
                        }
                        let v870: f64;
                        if v831 != 0.0 {
                            v870 = v172;
                        } else {
                            let v859 = v155 * ((v826 * v856) / v827);
                            let v861 = (v460 * v134) / v859;
                            let v862 = v861 * v861;
                            let v863 = v862 * v862;
                            let v866 = (v863 / (v863 + v12)).sqrt();
                            let v867 = v866.sqrt();
                            let v868 = v866 * v867;
                            let v879: f64;
                            if v869 != 0.0 {
                                let v874 = v12 / (v12 + (v859 * v868));
                                v879 = v874;
                            } else {
                                let v878 = (v12 + (v859 * v868)).powf(v877);
                                v879 = v878;
                            }
                            let v882 = (v828 * v879) / (v828 + v879);
                            let v885 = (v485 * (v859 / v867)).sqrt();
                            let v895 = (((v134 * v861) * v867) - (v134 * v866)) + (v32 * (v859 * v868));
                            let v897 = (((v59 * (v861 * v867)) - v866) - v12) * v885;
                            let v898 = v897 * v897;
                            let v899 = if v897 > v172 { 1.0 } else { 0.0 };
                            out899 = v899;
                            let v906: f64;
                            if v899 != 0.0 {
                                let v902 = v12 / (v12 + (v502 * v897));
                                v906 = v902;
                            } else {
                                let v905 = v12 / (v12 - (v502 * v897));
                                v906 = v905;
                            }
                            let v908 = (-v898) + v895;
                            let v910 = if v908 > v909 { 1.0 } else { 0.0 };
                            out910 = v910;
                            let v926: f64;
                            if v910 != 0.0 {
                                let v911 = v908.exp();
                                v926 = v911;
                            } else {
                                let v925 = v265 / (v12 + ((v912 - v908) * (v12 + (v32 * ((v914 - v908) * (v12 + ((v916 - v908) * v257)))))));
                                v926 = v925;
                            }
                            let v928 = v906 * v906;
                            let v934 = (((v530 * v906) + (v533 * v928)) + (v537 * (v928 * v906))) * v926;
                            let v937: f64;
                            if v899 != 0.0 {
                                v937 = v934;
                            } else {
                                let v936 = if v895 > v935 { 1.0 } else { 0.0 };
                                out936 = v936;
                                let v961: f64;
                                if v936 != 0.0 {
                                    let v946 = v895.exp();
                                    v961 = v946;
                                } else {
                                    let v960 = v265 / (v12 + ((v947 - v895) * (v12 + (v32 * ((v949 - v895) * (v12 + ((v951 - v895) * v257)))))));
                                    v961 = v960;
                                }
                                let v963 = (v59 * v961) - v934;
                                v937 = v963;
                            }
                            let v945 = v944 * ((v829 * (v940 * ((v134 * v937) / v885))) * v882);
                            v870 = v945;
                        }
                        let v964: f64;
                        if v871 != 0.0 {
                            v964 = v172;
                        } else {
                            let v967 = (-v178) / v966;
                            let v969 = if (v967.abs()) < v245 { 1.0 } else { 0.0 };
                            out969 = v969;
                            let v972: f64;
                            if v969 != 0.0 {
                                let v970 = v967.exp();
                                v972 = v970;
                            } else {
                                let v971 = if v967 < v172 { 1.0 } else { 0.0 };
                                out971 = v971;
                                let v1000: f64;
                                if v971 != 0.0 {
                                    let v990 = v265 / (v12 + ((v977 - v967) * (v12 + (v32 * ((v979 - v967) * (v12 + ((v981 - v967) * v257)))))));
                                    v1000 = v990;
                                } else {
                                    let v991 = v967 - v245;
                                    let v999 = v275 * (v12 + (v991 * (v12 + (v32 * (v991 * (v12 + (v991 * v257)))))));
                                    v1000 = v999;
                                }
                                v972 = v1000;
                            }
                            let v976 = v975 * (v973 * v972);
                            v964 = v976;
                        }
                        let v1006 = (v610 * (((v808 + v830) + v870) + v964)) * v1005;
                        v810 = v1006;
                        v811 = v826;
                        v812 = v827;
                        v813 = v828;
                        v814 = v829;
                    }
                    let v819 = ((v215 * v419) + (v225 * v620)) + (v233 * v810);
                    let v1009: f64;
                    let v1010: f64;
                    let v1011: f64;
                    let v1012: f64;
                    if v293 != 0.0 {
                        let v1008 = if v1007 < v242 { 1.0 } else { 0.0 };
                        out1008 = v1008;
                        let v1024: f64;
                        let v1025: f64;
                        let v1026: f64;
                        if v1008 != 0.0 {
                            let v1013 = v1007 * v13;
                            let v1017 = if ((v1014 * v1013).abs()) < v245 { 1.0 } else { 0.0 };
                            out1017 = v1017;
                            let v1035: f64;
                            if v1017 != 0.0 {
                                let v1031 = (v1029 * v1013).exp();
                                v1035 = v1031;
                            } else {
                                let v1034 = if (v1032 * v1013) < v172 { 1.0 } else { 0.0 };
                                out1034 = v1034;
                                let v1075: f64;
                                if v1034 != 0.0 {
                                    let v1057 = v265 / (v12 + ((v1040 - (v1038 * v1013)) * (v12 + (v32 * ((v1044 - (v1042 * v1013)) * (v12 + ((v1048 - (v1046 * v1013)) * v257)))))));
                                    v1075 = v1057;
                                } else {
                                    let v1074 = v275 * (v12 + (((v1058 * v1013) - v245) * (v12 + (v32 * (((v1061 * v1013) - v245) * (v12 + (((v1064 * v1013) - v245) * v257)))))));
                                    v1075 = v1074;
                                }
                                v1035 = v1075;
                            }
                            let v1036 = v12 / v1035;
                            let v1037 = v1036 * v1036;
                            v1024 = v1037;
                            v1025 = v1035;
                            v1026 = v1036;
                        } else {
                            let v1021 = (v12 + ((v1007 - v242) * v13)) * v249;
                            let v1022 = v1021.sqrt();
                            let v1023 = v12 / v1022;
                            v1024 = v1021;
                            v1025 = v1023;
                            v1026 = v1022;
                        }
                        let v1027 = v1024 - v12;
                        let v1098: f64;
                        if v1028 != 0.0 {
                            let v1084 = v59 * (v11 * (((v59 + v1025) + (((v1025 + v12) * (v1025 + v380)).sqrt())).ln()));
                            v1098 = v1084;
                        } else {
                            let v1097 = v1096 + (v59 * (v11 * ((((v59 * v1026) + v12) + (((v12 + v1026) * (v12 + (v380 * v1026))).sqrt())).ln())));
                            v1098 = v1097;
                        }
                        let v1099 = v287 - v1098;
                        let v1101 = v1007 - v1099;
                        let v1108 = v32 * ((v1007 + v1099) - (((v1101 * v1101) + ((v406 * v11) * v11)).sqrt()));
                        v1009 = v1027;
                        v1010 = v1108;
                        v1011 = v1098;
                        v1012 = v1026;
                    } else {
                        v1009 = v311;
                        v1010 = v312;
                        v1011 = v172;
                        v1012 = v314;
                    }
                    let v1111: f64;
                    let v1112: f64;
                    let v1113: f64;
                    let v1114: f64;
                    let v1115: f64;
                    if v250 != 0.0 {
                        v1111 = v811;
                        v1112 = v812;
                        v1113 = v813;
                        v1114 = v814;
                        v1115 = v172;
                    } else {
                        let v1109 = v50 * v1009;
                        let v1122: f64;
                        let v1123: f64;
                        let v1124: f64;
                        let v1125: f64;
                        let v1126: f64;
                        if v1110 != 0.0 {
                            v1122 = v811;
                            v1123 = v812;
                            v1124 = v813;
                            v1125 = v814;
                            v1126 = v172;
                        } else {
                            let v1116 = v81 - v1010;
                            let v1120 = v12 - ((v12 - (v1011 / v1116)).sqrt());
                            let v1136: f64;
                            if v1121 != 0.0 {
                                v1136 = v172;
                            } else {
                                let v1135 = ((((v1120 * v1120) * (v1120.ln())) / (v12 - v1120)) + v1120) * v1134;
                                v1136 = v1135;
                            }
                            let v1137 = v1120 + v1136;
                            let v1142: f64;
                            if v1121 != 0.0 {
                                let v1139 = (v1116 * v442).sqrt();
                                v1142 = v1139;
                            } else {
                                let v1141 = (v1116 * v442).powf(v100);
                                v1142 = v1141;
                            }
                            let v1143 = v448 * v1142;
                            let v1146 = v35 * ((v1012 - v12) * v1143);
                            let v1148 = v454 * (v1146 * v1137);
                            v1122 = v1143;
                            v1123 = v1116;
                            v1124 = v1137;
                            v1125 = v1146;
                            v1126 = v1148;
                        }
                        let v1162: f64;
                        if v1127 != 0.0 {
                            v1162 = v172;
                        } else {
                            let v1151 = v141 * ((v1122 * v456) / v1123);
                            let v1153 = (v460 * v132) / v1151;
                            let v1154 = v1153 * v1153;
                            let v1155 = v1154 * v1154;
                            let v1158 = (v1155 / (v1155 + v12)).sqrt();
                            let v1159 = v1158.sqrt();
                            let v1160 = v1158 * v1159;
                            let v1171: f64;
                            if v1161 != 0.0 {
                                let v1166 = v12 / (v12 + (v1151 * v1160));
                                v1171 = v1166;
                            } else {
                                let v1170 = (v12 + (v1151 * v1160)).powf(v1169);
                                v1171 = v1170;
                            }
                            let v1174 = (v1124 * v1171) / (v1124 + v1171);
                            let v1177 = (v485 * (v1151 / v1159)).sqrt();
                            let v1187 = (((v132 * v1153) * v1159) - (v132 * v1158)) + (v32 * (v1151 * v1160));
                            let v1189 = (((v59 * (v1153 * v1159)) - v1158) - v12) * v1177;
                            let v1190 = v1189 * v1189;
                            let v1191 = if v1189 > v172 { 1.0 } else { 0.0 };
                            out1191 = v1191;
                            let v1198: f64;
                            if v1191 != 0.0 {
                                let v1194 = v12 / (v12 + (v502 * v1189));
                                v1198 = v1194;
                            } else {
                                let v1197 = v12 / (v12 - (v502 * v1189));
                                v1198 = v1197;
                            }
                            let v1200 = (-v1190) + v1187;
                            let v1202 = if v1200 > v1201 { 1.0 } else { 0.0 };
                            out1202 = v1202;
                            let v1218: f64;
                            if v1202 != 0.0 {
                                let v1203 = v1200.exp();
                                v1218 = v1203;
                            } else {
                                let v1217 = v265 / (v12 + ((v1204 - v1200) * (v12 + (v32 * ((v1206 - v1200) * (v12 + ((v1208 - v1200) * v257)))))));
                                v1218 = v1217;
                            }
                            let v1220 = v1198 * v1198;
                            let v1226 = (((v530 * v1198) + (v533 * v1220)) + (v537 * (v1220 * v1198))) * v1218;
                            let v1229: f64;
                            if v1191 != 0.0 {
                                v1229 = v1226;
                            } else {
                                let v1228 = if v1187 > v1227 { 1.0 } else { 0.0 };
                                out1228 = v1228;
                                let v1252: f64;
                                if v1228 != 0.0 {
                                    let v1237 = v1187.exp();
                                    v1252 = v1237;
                                } else {
                                    let v1251 = v265 / (v12 + ((v1238 - v1187) * (v12 + (v32 * ((v1240 - v1187) * (v12 + ((v1242 - v1187) * v257)))))));
                                    v1252 = v1251;
                                }
                                let v1254 = (v59 * v1252) - v1226;
                                v1229 = v1254;
                            }
                            let v1236 = v550 * ((v1125 * (v1232 * ((v132 * v1229) / v1177))) * v1174);
                            v1162 = v1236;
                        }
                        let v1255: f64;
                        if v1163 != 0.0 {
                            v1255 = v172;
                        } else {
                            let v1258 = (-v174) / v1257;
                            let v1260 = if (v1258.abs()) < v245 { 1.0 } else { 0.0 };
                            out1260 = v1260;
                            let v1263: f64;
                            if v1260 != 0.0 {
                                let v1261 = v1258.exp();
                                v1263 = v1261;
                            } else {
                                let v1262 = if v1258 < v172 { 1.0 } else { 0.0 };
                                out1262 = v1262;
                                let v1290: f64;
                                if v1262 != 0.0 {
                                    let v1280 = v265 / (v12 + ((v1267 - v1258) * (v12 + (v32 * ((v1269 - v1258) * (v12 + ((v1271 - v1258) * v257)))))));
                                    v1290 = v1280;
                                } else {
                                    let v1281 = v1258 - v245;
                                    let v1289 = v275 * (v12 + (v1281 * (v12 + (v32 * (v1281 * (v12 + (v1281 * v257)))))));
                                    v1290 = v1289;
                                }
                                v1263 = v1290;
                            }
                            let v1266 = v581 * (v1264 * v1263);
                            v1255 = v1266;
                        }
                        let v1296 = (v610 * (((v1109 + v1126) + v1162) + v1255)) * v1295;
                        v1111 = v1122;
                        v1112 = v1123;
                        v1113 = v1124;
                        v1114 = v1125;
                        v1115 = v1296;
                    }
                    let v1299: f64;
                    let v1300: f64;
                    let v1301: f64;
                    let v1302: f64;
                    let v1303: f64;
                    if v280 != 0.0 {
                        v1299 = v1111;
                        v1300 = v1112;
                        v1301 = v1113;
                        v1302 = v1114;
                        v1303 = v172;
                    } else {
                        let v1297 = v53 * v1009;
                        let v1310: f64;
                        let v1311: f64;
                        let v1312: f64;
                        let v1313: f64;
                        let v1314: f64;
                        if v1298 != 0.0 {
                            v1310 = v1111;
                            v1311 = v1112;
                            v1312 = v1113;
                            v1313 = v1114;
                            v1314 = v172;
                        } else {
                            let v1304 = v88 - v1010;
                            let v1308 = v12 - ((v12 - (v1011 / v1304)).sqrt());
                            let v1324: f64;
                            if v1309 != 0.0 {
                                v1324 = v172;
                            } else {
                                let v1323 = ((((v1308 * v1308) * (v1308.ln())) / (v12 - v1308)) + v1308) * v1322;
                                v1324 = v1323;
                            }
                            let v1325 = v1308 + v1324;
                            let v1330: f64;
                            if v1309 != 0.0 {
                                let v1327 = (v1304 * v643).sqrt();
                                v1330 = v1327;
                            } else {
                                let v1329 = (v1304 * v643).powf(v105);
                                v1330 = v1329;
                            }
                            let v1331 = v649 * v1330;
                            let v1334 = v41 * ((v1012 - v12) * v1331);
                            let v1336 = v655 * (v1334 * v1325);
                            v1310 = v1331;
                            v1311 = v1304;
                            v1312 = v1325;
                            v1313 = v1334;
                            v1314 = v1336;
                        }
                        let v1350: f64;
                        if v1315 != 0.0 {
                            v1350 = v172;
                        } else {
                            let v1339 = v148 * ((v1310 * v657) / v1311);
                            let v1341 = (v460 * v133) / v1339;
                            let v1342 = v1341 * v1341;
                            let v1343 = v1342 * v1342;
                            let v1346 = (v1343 / (v1343 + v12)).sqrt();
                            let v1347 = v1346.sqrt();
                            let v1348 = v1346 * v1347;
                            let v1359: f64;
                            if v1349 != 0.0 {
                                let v1354 = v12 / (v12 + (v1339 * v1348));
                                v1359 = v1354;
                            } else {
                                let v1358 = (v12 + (v1339 * v1348)).powf(v1357);
                                v1359 = v1358;
                            }
                            let v1362 = (v1312 * v1359) / (v1312 + v1359);
                            let v1365 = (v485 * (v1339 / v1347)).sqrt();
                            let v1375 = (((v133 * v1341) * v1347) - (v133 * v1346)) + (v32 * (v1339 * v1348));
                            let v1377 = (((v59 * (v1341 * v1347)) - v1346) - v12) * v1365;
                            let v1378 = v1377 * v1377;
                            let v1379 = if v1377 > v172 { 1.0 } else { 0.0 };
                            out1379 = v1379;
                            let v1386: f64;
                            if v1379 != 0.0 {
                                let v1382 = v12 / (v12 + (v502 * v1377));
                                v1386 = v1382;
                            } else {
                                let v1385 = v12 / (v12 - (v502 * v1377));
                                v1386 = v1385;
                            }
                            let v1388 = (-v1378) + v1375;
                            let v1390 = if v1388 > v1389 { 1.0 } else { 0.0 };
                            out1390 = v1390;
                            let v1406: f64;
                            if v1390 != 0.0 {
                                let v1391 = v1388.exp();
                                v1406 = v1391;
                            } else {
                                let v1405 = v265 / (v12 + ((v1392 - v1388) * (v12 + (v32 * ((v1394 - v1388) * (v12 + ((v1396 - v1388) * v257)))))));
                                v1406 = v1405;
                            }
                            let v1408 = v1386 * v1386;
                            let v1414 = (((v530 * v1386) + (v533 * v1408)) + (v537 * (v1408 * v1386))) * v1406;
                            let v1417: f64;
                            if v1379 != 0.0 {
                                v1417 = v1414;
                            } else {
                                let v1416 = if v1375 > v1415 { 1.0 } else { 0.0 };
                                out1416 = v1416;
                                let v1440: f64;
                                if v1416 != 0.0 {
                                    let v1425 = v1375.exp();
                                    v1440 = v1425;
                                } else {
                                    let v1439 = v265 / (v12 + ((v1426 - v1375) * (v12 + (v32 * ((v1428 - v1375) * (v12 + ((v1430 - v1375) * v257)))))));
                                    v1440 = v1439;
                                }
                                let v1442 = (v59 * v1440) - v1414;
                                v1417 = v1442;
                            }
                            let v1424 = v745 * ((v1313 * (v1420 * ((v133 * v1417) / v1365))) * v1362);
                            v1350 = v1424;
                        }
                        let v1443: f64;
                        if v1351 != 0.0 {
                            v1443 = v172;
                        } else {
                            let v1446 = (-v176) / v1445;
                            let v1448 = if (v1446.abs()) < v245 { 1.0 } else { 0.0 };
                            out1448 = v1448;
                            let v1451: f64;
                            if v1448 != 0.0 {
                                let v1449 = v1446.exp();
                                v1451 = v1449;
                            } else {
                                let v1450 = if v1446 < v172 { 1.0 } else { 0.0 };
                                out1450 = v1450;
                                let v1478: f64;
                                if v1450 != 0.0 {
                                    let v1468 = v265 / (v12 + ((v1455 - v1446) * (v12 + (v32 * ((v1457 - v1446) * (v12 + ((v1459 - v1446) * v257)))))));
                                    v1478 = v1468;
                                } else {
                                    let v1469 = v1446 - v245;
                                    let v1477 = v275 * (v12 + (v1469 * (v12 + (v32 * (v1469 * (v12 + (v1469 * v257)))))));
                                    v1478 = v1477;
                                }
                                v1451 = v1478;
                            }
                            let v1454 = v776 * (v1452 * v1451);
                            v1443 = v1454;
                        }
                        let v1484 = (v610 * (((v1297 + v1314) + v1350) + v1443)) * v1483;
                        v1299 = v1310;
                        v1300 = v1311;
                        v1301 = v1312;
                        v1302 = v1313;
                        v1303 = v1484;
                    }
                    let v1487: f64;
                    let v1488: f64;
                    let v1489: f64;
                    let v1490: f64;
                    let v1491: f64;
                    if v283 != 0.0 {
                        v1487 = v172;
                        v1488 = v1299;
                        v1489 = v1300;
                        v1490 = v1301;
                        v1491 = v1302;
                    } else {
                        let v1485 = v56 * v1009;
                        let v1503: f64;
                        let v1504: f64;
                        let v1505: f64;
                        let v1506: f64;
                        let v1507: f64;
                        if v1486 != 0.0 {
                            v1503 = v1299;
                            v1504 = v1300;
                            v1505 = v1301;
                            v1506 = v1302;
                            v1507 = v172;
                        } else {
                            let v1497 = v95 - v1010;
                            let v1501 = v12 - ((v12 - (v1011 / v1497)).sqrt());
                            let v1517: f64;
                            if v1502 != 0.0 {
                                v1517 = v172;
                            } else {
                                let v1516 = ((((v1501 * v1501) * (v1501.ln())) / (v12 - v1501)) + v1501) * v1515;
                                v1517 = v1516;
                            }
                            let v1518 = v1501 + v1517;
                            let v1523: f64;
                            if v1502 != 0.0 {
                                let v1520 = (v1497 * v842).sqrt();
                                v1523 = v1520;
                            } else {
                                let v1522 = (v1497 * v842).powf(v110);
                                v1523 = v1522;
                            }
                            let v1524 = v848 * v1523;
                            let v1527 = v47 * ((v1012 - v12) * v1524);
                            let v1529 = v854 * (v1527 * v1518);
                            v1503 = v1524;
                            v1504 = v1497;
                            v1505 = v1518;
                            v1506 = v1527;
                            v1507 = v1529;
                        }
                        let v1543: f64;
                        if v1508 != 0.0 {
                            v1543 = v172;
                        } else {
                            let v1532 = v155 * ((v1503 * v856) / v1504);
                            let v1534 = (v460 * v134) / v1532;
                            let v1535 = v1534 * v1534;
                            let v1536 = v1535 * v1535;
                            let v1539 = (v1536 / (v1536 + v12)).sqrt();
                            let v1540 = v1539.sqrt();
                            let v1541 = v1539 * v1540;
                            let v1552: f64;
                            if v1542 != 0.0 {
                                let v1547 = v12 / (v12 + (v1532 * v1541));
                                v1552 = v1547;
                            } else {
                                let v1551 = (v12 + (v1532 * v1541)).powf(v1550);
                                v1552 = v1551;
                            }
                            let v1555 = (v1505 * v1552) / (v1505 + v1552);
                            let v1558 = (v485 * (v1532 / v1540)).sqrt();
                            let v1568 = (((v134 * v1534) * v1540) - (v134 * v1539)) + (v32 * (v1532 * v1541));
                            let v1570 = (((v59 * (v1534 * v1540)) - v1539) - v12) * v1558;
                            let v1571 = v1570 * v1570;
                            let v1572 = if v1570 > v172 { 1.0 } else { 0.0 };
                            out1572 = v1572;
                            let v1579: f64;
                            if v1572 != 0.0 {
                                let v1575 = v12 / (v12 + (v502 * v1570));
                                v1579 = v1575;
                            } else {
                                let v1578 = v12 / (v12 - (v502 * v1570));
                                v1579 = v1578;
                            }
                            let v1581 = (-v1571) + v1568;
                            let v1583 = if v1581 > v1582 { 1.0 } else { 0.0 };
                            out1583 = v1583;
                            let v1599: f64;
                            if v1583 != 0.0 {
                                let v1584 = v1581.exp();
                                v1599 = v1584;
                            } else {
                                let v1598 = v265 / (v12 + ((v1585 - v1581) * (v12 + (v32 * ((v1587 - v1581) * (v12 + ((v1589 - v1581) * v257)))))));
                                v1599 = v1598;
                            }
                            let v1601 = v1579 * v1579;
                            let v1607 = (((v530 * v1579) + (v533 * v1601)) + (v537 * (v1601 * v1579))) * v1599;
                            let v1610: f64;
                            if v1572 != 0.0 {
                                v1610 = v1607;
                            } else {
                                let v1609 = if v1568 > v1608 { 1.0 } else { 0.0 };
                                out1609 = v1609;
                                let v1633: f64;
                                if v1609 != 0.0 {
                                    let v1618 = v1568.exp();
                                    v1633 = v1618;
                                } else {
                                    let v1632 = v265 / (v12 + ((v1619 - v1568) * (v12 + (v32 * ((v1621 - v1568) * (v12 + ((v1623 - v1568) * v257)))))));
                                    v1633 = v1632;
                                }
                                let v1635 = (v59 * v1633) - v1607;
                                v1610 = v1635;
                            }
                            let v1617 = v944 * ((v1506 * (v1613 * ((v134 * v1610) / v1558))) * v1555);
                            v1543 = v1617;
                        }
                        let v1636: f64;
                        if v1544 != 0.0 {
                            v1636 = v172;
                        } else {
                            let v1639 = (-v178) / v1638;
                            let v1641 = if (v1639.abs()) < v245 { 1.0 } else { 0.0 };
                            out1641 = v1641;
                            let v1644: f64;
                            if v1641 != 0.0 {
                                let v1642 = v1639.exp();
                                v1644 = v1642;
                            } else {
                                let v1643 = if v1639 < v172 { 1.0 } else { 0.0 };
                                out1643 = v1643;
                                let v1671: f64;
                                if v1643 != 0.0 {
                                    let v1661 = v265 / (v12 + ((v1648 - v1639) * (v12 + (v32 * ((v1650 - v1639) * (v12 + ((v1652 - v1639) * v257)))))));
                                    v1671 = v1661;
                                } else {
                                    let v1662 = v1639 - v245;
                                    let v1670 = v275 * (v12 + (v1662 * (v12 + (v32 * (v1662 * (v12 + (v1662 * v257)))))));
                                    v1671 = v1670;
                                }
                                v1644 = v1671;
                            }
                            let v1647 = v975 * (v1645 * v1644);
                            v1636 = v1647;
                        }
                        let v1677 = (v610 * (((v1485 + v1507) + v1543) + v1636)) * v1676;
                        v1487 = v1677;
                        v1488 = v1503;
                        v1489 = v1504;
                        v1490 = v1505;
                        v1491 = v1506;
                    }
                    let v1496 = ((v215 * v1115) + (v225 * v1303)) + (v233 * v1487);
                    let v1680: f64;
                    let v1681: f64;
                    let v1682: f64;
                    let v1683: f64;
                    if v293 != 0.0 {
                        let v1679 = if v1678 < v242 { 1.0 } else { 0.0 };
                        out1679 = v1679;
                        let v1695: f64;
                        let v1696: f64;
                        let v1697: f64;
                        if v1679 != 0.0 {
                            let v1684 = v1678 * v13;
                            let v1688 = if ((v1685 * v1684).abs()) < v245 { 1.0 } else { 0.0 };
                            out1688 = v1688;
                            let v1706: f64;
                            if v1688 != 0.0 {
                                let v1702 = (v1700 * v1684).exp();
                                v1706 = v1702;
                            } else {
                                let v1705 = if (v1703 * v1684) < v172 { 1.0 } else { 0.0 };
                                out1705 = v1705;
                                let v1746: f64;
                                if v1705 != 0.0 {
                                    let v1728 = v265 / (v12 + ((v1711 - (v1709 * v1684)) * (v12 + (v32 * ((v1715 - (v1713 * v1684)) * (v12 + ((v1719 - (v1717 * v1684)) * v257)))))));
                                    v1746 = v1728;
                                } else {
                                    let v1745 = v275 * (v12 + (((v1729 * v1684) - v245) * (v12 + (v32 * (((v1732 * v1684) - v245) * (v12 + (((v1735 * v1684) - v245) * v257)))))));
                                    v1746 = v1745;
                                }
                                v1706 = v1746;
                            }
                            let v1707 = v12 / v1706;
                            let v1708 = v1707 * v1707;
                            v1695 = v1708;
                            v1696 = v1706;
                            v1697 = v1707;
                        } else {
                            let v1692 = (v12 + ((v1678 - v242) * v13)) * v249;
                            let v1693 = v1692.sqrt();
                            let v1694 = v12 / v1693;
                            v1695 = v1692;
                            v1696 = v1694;
                            v1697 = v1693;
                        }
                        let v1698 = v1695 - v12;
                        let v1769: f64;
                        if v1699 != 0.0 {
                            let v1755 = v59 * (v11 * (((v59 + v1696) + (((v1696 + v12) * (v1696 + v380)).sqrt())).ln()));
                            v1769 = v1755;
                        } else {
                            let v1768 = v1767 + (v59 * (v11 * ((((v59 * v1697) + v12) + (((v12 + v1697) * (v12 + (v380 * v1697))).sqrt())).ln())));
                            v1769 = v1768;
                        }
                        let v1770 = v287 - v1769;
                        let v1772 = v1678 - v1770;
                        let v1779 = v32 * ((v1678 + v1770) - (((v1772 * v1772) + ((v406 * v11) * v11)).sqrt()));
                        v1680 = v1698;
                        v1681 = v1779;
                        v1682 = v1769;
                        v1683 = v1697;
                    } else {
                        v1680 = v1009;
                        v1681 = v1010;
                        v1682 = v172;
                        v1683 = v1012;
                    }
                    let v1782: f64;
                    let v1783: f64;
                    let v1784: f64;
                    let v1785: f64;
                    let v1786: f64;
                    if v250 != 0.0 {
                        v1782 = v1488;
                        v1783 = v1489;
                        v1784 = v1490;
                        v1785 = v1491;
                        v1786 = v172;
                    } else {
                        let v1780 = v50 * v1680;
                        let v1793: f64;
                        let v1794: f64;
                        let v1795: f64;
                        let v1796: f64;
                        let v1797: f64;
                        if v1781 != 0.0 {
                            v1793 = v1488;
                            v1794 = v1489;
                            v1795 = v1490;
                            v1796 = v1491;
                            v1797 = v172;
                        } else {
                            let v1787 = v81 - v1681;
                            let v1791 = v12 - ((v12 - (v1682 / v1787)).sqrt());
                            let v1807: f64;
                            if v1792 != 0.0 {
                                v1807 = v172;
                            } else {
                                let v1806 = ((((v1791 * v1791) * (v1791.ln())) / (v12 - v1791)) + v1791) * v1805;
                                v1807 = v1806;
                            }
                            let v1808 = v1791 + v1807;
                            let v1813: f64;
                            if v1792 != 0.0 {
                                let v1810 = (v1787 * v442).sqrt();
                                v1813 = v1810;
                            } else {
                                let v1812 = (v1787 * v442).powf(v100);
                                v1813 = v1812;
                            }
                            let v1814 = v448 * v1813;
                            let v1817 = v35 * ((v1683 - v12) * v1814);
                            let v1819 = v454 * (v1817 * v1808);
                            v1793 = v1814;
                            v1794 = v1787;
                            v1795 = v1808;
                            v1796 = v1817;
                            v1797 = v1819;
                        }
                        let v1833: f64;
                        if v1798 != 0.0 {
                            v1833 = v172;
                        } else {
                            let v1822 = v141 * ((v1793 * v456) / v1794);
                            let v1824 = (v460 * v132) / v1822;
                            let v1825 = v1824 * v1824;
                            let v1826 = v1825 * v1825;
                            let v1829 = (v1826 / (v1826 + v12)).sqrt();
                            let v1830 = v1829.sqrt();
                            let v1831 = v1829 * v1830;
                            let v1842: f64;
                            if v1832 != 0.0 {
                                let v1837 = v12 / (v12 + (v1822 * v1831));
                                v1842 = v1837;
                            } else {
                                let v1841 = (v12 + (v1822 * v1831)).powf(v1840);
                                v1842 = v1841;
                            }
                            let v1845 = (v1795 * v1842) / (v1795 + v1842);
                            let v1848 = (v485 * (v1822 / v1830)).sqrt();
                            let v1858 = (((v132 * v1824) * v1830) - (v132 * v1829)) + (v32 * (v1822 * v1831));
                            let v1860 = (((v59 * (v1824 * v1830)) - v1829) - v12) * v1848;
                            let v1861 = v1860 * v1860;
                            let v1862 = if v1860 > v172 { 1.0 } else { 0.0 };
                            out1862 = v1862;
                            let v1869: f64;
                            if v1862 != 0.0 {
                                let v1865 = v12 / (v12 + (v502 * v1860));
                                v1869 = v1865;
                            } else {
                                let v1868 = v12 / (v12 - (v502 * v1860));
                                v1869 = v1868;
                            }
                            let v1871 = (-v1861) + v1858;
                            let v1873 = if v1871 > v1872 { 1.0 } else { 0.0 };
                            out1873 = v1873;
                            let v1889: f64;
                            if v1873 != 0.0 {
                                let v1874 = v1871.exp();
                                v1889 = v1874;
                            } else {
                                let v1888 = v265 / (v12 + ((v1875 - v1871) * (v12 + (v32 * ((v1877 - v1871) * (v12 + ((v1879 - v1871) * v257)))))));
                                v1889 = v1888;
                            }
                            let v1891 = v1869 * v1869;
                            let v1897 = (((v530 * v1869) + (v533 * v1891)) + (v537 * (v1891 * v1869))) * v1889;
                            let v1900: f64;
                            if v1862 != 0.0 {
                                v1900 = v1897;
                            } else {
                                let v1899 = if v1858 > v1898 { 1.0 } else { 0.0 };
                                out1899 = v1899;
                                let v1923: f64;
                                if v1899 != 0.0 {
                                    let v1908 = v1858.exp();
                                    v1923 = v1908;
                                } else {
                                    let v1922 = v265 / (v12 + ((v1909 - v1858) * (v12 + (v32 * ((v1911 - v1858) * (v12 + ((v1913 - v1858) * v257)))))));
                                    v1923 = v1922;
                                }
                                let v1925 = (v59 * v1923) - v1897;
                                v1900 = v1925;
                            }
                            let v1907 = v550 * ((v1796 * (v1903 * ((v132 * v1900) / v1848))) * v1845);
                            v1833 = v1907;
                        }
                        let v1926: f64;
                        if v1834 != 0.0 {
                            v1926 = v172;
                        } else {
                            let v1929 = (-v174) / v1928;
                            let v1931 = if (v1929.abs()) < v245 { 1.0 } else { 0.0 };
                            out1931 = v1931;
                            let v1934: f64;
                            if v1931 != 0.0 {
                                let v1932 = v1929.exp();
                                v1934 = v1932;
                            } else {
                                let v1933 = if v1929 < v172 { 1.0 } else { 0.0 };
                                out1933 = v1933;
                                let v1961: f64;
                                if v1933 != 0.0 {
                                    let v1951 = v265 / (v12 + ((v1938 - v1929) * (v12 + (v32 * ((v1940 - v1929) * (v12 + ((v1942 - v1929) * v257)))))));
                                    v1961 = v1951;
                                } else {
                                    let v1952 = v1929 - v245;
                                    let v1960 = v275 * (v12 + (v1952 * (v12 + (v32 * (v1952 * (v12 + (v1952 * v257)))))));
                                    v1961 = v1960;
                                }
                                v1934 = v1961;
                            }
                            let v1937 = v581 * (v1935 * v1934);
                            v1926 = v1937;
                        }
                        let v1967 = (v610 * (((v1780 + v1797) + v1833) + v1926)) * v1966;
                        v1782 = v1793;
                        v1783 = v1794;
                        v1784 = v1795;
                        v1785 = v1796;
                        v1786 = v1967;
                    }
                    let v1970: f64;
                    let v1971: f64;
                    let v1972: f64;
                    let v1973: f64;
                    let v1974: f64;
                    if v280 != 0.0 {
                        v1970 = v1782;
                        v1971 = v1783;
                        v1972 = v1784;
                        v1973 = v1785;
                        v1974 = v172;
                    } else {
                        let v1968 = v53 * v1680;
                        let v1981: f64;
                        let v1982: f64;
                        let v1983: f64;
                        let v1984: f64;
                        let v1985: f64;
                        if v1969 != 0.0 {
                            v1981 = v1782;
                            v1982 = v1783;
                            v1983 = v1784;
                            v1984 = v1785;
                            v1985 = v172;
                        } else {
                            let v1975 = v88 - v1681;
                            let v1979 = v12 - ((v12 - (v1682 / v1975)).sqrt());
                            let v1995: f64;
                            if v1980 != 0.0 {
                                v1995 = v172;
                            } else {
                                let v1994 = ((((v1979 * v1979) * (v1979.ln())) / (v12 - v1979)) + v1979) * v1993;
                                v1995 = v1994;
                            }
                            let v1996 = v1979 + v1995;
                            let v2001: f64;
                            if v1980 != 0.0 {
                                let v1998 = (v1975 * v643).sqrt();
                                v2001 = v1998;
                            } else {
                                let v2000 = (v1975 * v643).powf(v105);
                                v2001 = v2000;
                            }
                            let v2002 = v649 * v2001;
                            let v2005 = v41 * ((v1683 - v12) * v2002);
                            let v2007 = v655 * (v2005 * v1996);
                            v1981 = v2002;
                            v1982 = v1975;
                            v1983 = v1996;
                            v1984 = v2005;
                            v1985 = v2007;
                        }
                        let v2021: f64;
                        if v1986 != 0.0 {
                            v2021 = v172;
                        } else {
                            let v2010 = v148 * ((v1981 * v657) / v1982);
                            let v2012 = (v460 * v133) / v2010;
                            let v2013 = v2012 * v2012;
                            let v2014 = v2013 * v2013;
                            let v2017 = (v2014 / (v2014 + v12)).sqrt();
                            let v2018 = v2017.sqrt();
                            let v2019 = v2017 * v2018;
                            let v2030: f64;
                            if v2020 != 0.0 {
                                let v2025 = v12 / (v12 + (v2010 * v2019));
                                v2030 = v2025;
                            } else {
                                let v2029 = (v12 + (v2010 * v2019)).powf(v2028);
                                v2030 = v2029;
                            }
                            let v2033 = (v1983 * v2030) / (v1983 + v2030);
                            let v2036 = (v485 * (v2010 / v2018)).sqrt();
                            let v2046 = (((v133 * v2012) * v2018) - (v133 * v2017)) + (v32 * (v2010 * v2019));
                            let v2048 = (((v59 * (v2012 * v2018)) - v2017) - v12) * v2036;
                            let v2049 = v2048 * v2048;
                            let v2050 = if v2048 > v172 { 1.0 } else { 0.0 };
                            out2050 = v2050;
                            let v2057: f64;
                            if v2050 != 0.0 {
                                let v2053 = v12 / (v12 + (v502 * v2048));
                                v2057 = v2053;
                            } else {
                                let v2056 = v12 / (v12 - (v502 * v2048));
                                v2057 = v2056;
                            }
                            let v2059 = (-v2049) + v2046;
                            let v2061 = if v2059 > v2060 { 1.0 } else { 0.0 };
                            out2061 = v2061;
                            let v2077: f64;
                            if v2061 != 0.0 {
                                let v2062 = v2059.exp();
                                v2077 = v2062;
                            } else {
                                let v2076 = v265 / (v12 + ((v2063 - v2059) * (v12 + (v32 * ((v2065 - v2059) * (v12 + ((v2067 - v2059) * v257)))))));
                                v2077 = v2076;
                            }
                            let v2079 = v2057 * v2057;
                            let v2085 = (((v530 * v2057) + (v533 * v2079)) + (v537 * (v2079 * v2057))) * v2077;
                            let v2088: f64;
                            if v2050 != 0.0 {
                                v2088 = v2085;
                            } else {
                                let v2087 = if v2046 > v2086 { 1.0 } else { 0.0 };
                                out2087 = v2087;
                                let v2111: f64;
                                if v2087 != 0.0 {
                                    let v2096 = v2046.exp();
                                    v2111 = v2096;
                                } else {
                                    let v2110 = v265 / (v12 + ((v2097 - v2046) * (v12 + (v32 * ((v2099 - v2046) * (v12 + ((v2101 - v2046) * v257)))))));
                                    v2111 = v2110;
                                }
                                let v2113 = (v59 * v2111) - v2085;
                                v2088 = v2113;
                            }
                            let v2095 = v745 * ((v1984 * (v2091 * ((v133 * v2088) / v2036))) * v2033);
                            v2021 = v2095;
                        }
                        let v2114: f64;
                        if v2022 != 0.0 {
                            v2114 = v172;
                        } else {
                            let v2117 = (-v176) / v2116;
                            let v2119 = if (v2117.abs()) < v245 { 1.0 } else { 0.0 };
                            out2119 = v2119;
                            let v2122: f64;
                            if v2119 != 0.0 {
                                let v2120 = v2117.exp();
                                v2122 = v2120;
                            } else {
                                let v2121 = if v2117 < v172 { 1.0 } else { 0.0 };
                                out2121 = v2121;
                                let v2149: f64;
                                if v2121 != 0.0 {
                                    let v2139 = v265 / (v12 + ((v2126 - v2117) * (v12 + (v32 * ((v2128 - v2117) * (v12 + ((v2130 - v2117) * v257)))))));
                                    v2149 = v2139;
                                } else {
                                    let v2140 = v2117 - v245;
                                    let v2148 = v275 * (v12 + (v2140 * (v12 + (v32 * (v2140 * (v12 + (v2140 * v257)))))));
                                    v2149 = v2148;
                                }
                                v2122 = v2149;
                            }
                            let v2125 = v776 * (v2123 * v2122);
                            v2114 = v2125;
                        }
                        let v2155 = (v610 * (((v1968 + v1985) + v2021) + v2114)) * v2154;
                        v1970 = v1981;
                        v1971 = v1982;
                        v1972 = v1983;
                        v1973 = v1984;
                        v1974 = v2155;
                    }
                    let v2158: f64;
                    let v2159: f64;
                    let v2160: f64;
                    let v2161: f64;
                    let v2162: f64;
                    if v283 != 0.0 {
                        v2158 = v172;
                        v2159 = v1970;
                        v2160 = v1971;
                        v2161 = v1972;
                        v2162 = v1973;
                    } else {
                        let v2156 = v56 * v1680;
                        let v2174: f64;
                        let v2175: f64;
                        let v2176: f64;
                        let v2177: f64;
                        let v2178: f64;
                        if v2157 != 0.0 {
                            v2174 = v1970;
                            v2175 = v1971;
                            v2176 = v1972;
                            v2177 = v1973;
                            v2178 = v172;
                        } else {
                            let v2168 = v95 - v1681;
                            let v2172 = v12 - ((v12 - (v1682 / v2168)).sqrt());
                            let v2188: f64;
                            if v2173 != 0.0 {
                                v2188 = v172;
                            } else {
                                let v2187 = ((((v2172 * v2172) * (v2172.ln())) / (v12 - v2172)) + v2172) * v2186;
                                v2188 = v2187;
                            }
                            let v2189 = v2172 + v2188;
                            let v2194: f64;
                            if v2173 != 0.0 {
                                let v2191 = (v2168 * v842).sqrt();
                                v2194 = v2191;
                            } else {
                                let v2193 = (v2168 * v842).powf(v110);
                                v2194 = v2193;
                            }
                            let v2195 = v848 * v2194;
                            let v2198 = v47 * ((v1683 - v12) * v2195);
                            let v2200 = v854 * (v2198 * v2189);
                            v2174 = v2195;
                            v2175 = v2168;
                            v2176 = v2189;
                            v2177 = v2198;
                            v2178 = v2200;
                        }
                        let v2214: f64;
                        if v2179 != 0.0 {
                            v2214 = v172;
                        } else {
                            let v2203 = v155 * ((v2174 * v856) / v2175);
                            let v2205 = (v460 * v134) / v2203;
                            let v2206 = v2205 * v2205;
                            let v2207 = v2206 * v2206;
                            let v2210 = (v2207 / (v2207 + v12)).sqrt();
                            let v2211 = v2210.sqrt();
                            let v2212 = v2210 * v2211;
                            let v2223: f64;
                            if v2213 != 0.0 {
                                let v2218 = v12 / (v12 + (v2203 * v2212));
                                v2223 = v2218;
                            } else {
                                let v2222 = (v12 + (v2203 * v2212)).powf(v2221);
                                v2223 = v2222;
                            }
                            let v2226 = (v2176 * v2223) / (v2176 + v2223);
                            let v2229 = (v485 * (v2203 / v2211)).sqrt();
                            let v2239 = (((v134 * v2205) * v2211) - (v134 * v2210)) + (v32 * (v2203 * v2212));
                            let v2241 = (((v59 * (v2205 * v2211)) - v2210) - v12) * v2229;
                            let v2242 = v2241 * v2241;
                            let v2243 = if v2241 > v172 { 1.0 } else { 0.0 };
                            out2243 = v2243;
                            let v2250: f64;
                            if v2243 != 0.0 {
                                let v2246 = v12 / (v12 + (v502 * v2241));
                                v2250 = v2246;
                            } else {
                                let v2249 = v12 / (v12 - (v502 * v2241));
                                v2250 = v2249;
                            }
                            let v2252 = (-v2242) + v2239;
                            let v2254 = if v2252 > v2253 { 1.0 } else { 0.0 };
                            out2254 = v2254;
                            let v2270: f64;
                            if v2254 != 0.0 {
                                let v2255 = v2252.exp();
                                v2270 = v2255;
                            } else {
                                let v2269 = v265 / (v12 + ((v2256 - v2252) * (v12 + (v32 * ((v2258 - v2252) * (v12 + ((v2260 - v2252) * v257)))))));
                                v2270 = v2269;
                            }
                            let v2272 = v2250 * v2250;
                            let v2278 = (((v530 * v2250) + (v533 * v2272)) + (v537 * (v2272 * v2250))) * v2270;
                            let v2281: f64;
                            if v2243 != 0.0 {
                                v2281 = v2278;
                            } else {
                                let v2280 = if v2239 > v2279 { 1.0 } else { 0.0 };
                                out2280 = v2280;
                                let v2304: f64;
                                if v2280 != 0.0 {
                                    let v2289 = v2239.exp();
                                    v2304 = v2289;
                                } else {
                                    let v2303 = v265 / (v12 + ((v2290 - v2239) * (v12 + (v32 * ((v2292 - v2239) * (v12 + ((v2294 - v2239) * v257)))))));
                                    v2304 = v2303;
                                }
                                let v2306 = (v59 * v2304) - v2278;
                                v2281 = v2306;
                            }
                            let v2288 = v944 * ((v2177 * (v2284 * ((v134 * v2281) / v2229))) * v2226);
                            v2214 = v2288;
                        }
                        let v2307: f64;
                        if v2215 != 0.0 {
                            v2307 = v172;
                        } else {
                            let v2310 = (-v178) / v2309;
                            let v2312 = if (v2310.abs()) < v245 { 1.0 } else { 0.0 };
                            out2312 = v2312;
                            let v2315: f64;
                            if v2312 != 0.0 {
                                let v2313 = v2310.exp();
                                v2315 = v2313;
                            } else {
                                let v2314 = if v2310 < v172 { 1.0 } else { 0.0 };
                                out2314 = v2314;
                                let v2342: f64;
                                if v2314 != 0.0 {
                                    let v2332 = v265 / (v12 + ((v2319 - v2310) * (v12 + (v32 * ((v2321 - v2310) * (v12 + ((v2323 - v2310) * v257)))))));
                                    v2342 = v2332;
                                } else {
                                    let v2333 = v2310 - v245;
                                    let v2341 = v275 * (v12 + (v2333 * (v12 + (v32 * (v2333 * (v12 + (v2333 * v257)))))));
                                    v2342 = v2341;
                                }
                                v2315 = v2342;
                            }
                            let v2318 = v975 * (v2316 * v2315);
                            v2307 = v2318;
                        }
                        let v2348 = (v610 * (((v2156 + v2178) + v2214) + v2307)) * v2347;
                        v2158 = v2348;
                        v2159 = v2174;
                        v2160 = v2175;
                        v2161 = v2176;
                        v2162 = v2177;
                    }
                    let v2167 = ((v215 * v1786) + (v225 * v1974)) + (v233 * v2158);
                    let v2350: f64;
                    let v2351: f64;
                    let v2352: f64;
                    let v2353: f64;
                    if v293 != 0.0 {
                        let v2349 = if v288 < v242 { 1.0 } else { 0.0 };
                        out2349 = v2349;
                        let v2365: f64;
                        let v2366: f64;
                        let v2367: f64;
                        if v2349 != 0.0 {
                            let v2354 = v288 * v13;
                            let v2358 = if ((v2355 * v2354).abs()) < v245 { 1.0 } else { 0.0 };
                            out2358 = v2358;
                            let v2376: f64;
                            if v2358 != 0.0 {
                                let v2372 = (v2370 * v2354).exp();
                                v2376 = v2372;
                            } else {
                                let v2375 = if (v2373 * v2354) < v172 { 1.0 } else { 0.0 };
                                out2375 = v2375;
                                let v2416: f64;
                                if v2375 != 0.0 {
                                    let v2398 = v265 / (v12 + ((v2381 - (v2379 * v2354)) * (v12 + (v32 * ((v2385 - (v2383 * v2354)) * (v12 + ((v2389 - (v2387 * v2354)) * v257)))))));
                                    v2416 = v2398;
                                } else {
                                    let v2415 = v275 * (v12 + (((v2399 * v2354) - v245) * (v12 + (v32 * (((v2402 * v2354) - v245) * (v12 + (((v2405 * v2354) - v245) * v257)))))));
                                    v2416 = v2415;
                                }
                                v2376 = v2416;
                            }
                            let v2377 = v12 / v2376;
                            let v2378 = v2377 * v2377;
                            v2365 = v2378;
                            v2366 = v2376;
                            v2367 = v2377;
                        } else {
                            let v2362 = (v12 + ((v288 - v242) * v13)) * v249;
                            let v2363 = v2362.sqrt();
                            let v2364 = v12 / v2363;
                            v2365 = v2362;
                            v2366 = v2364;
                            v2367 = v2363;
                        }
                        let v2368 = v2365 - v12;
                        let v2439: f64;
                        if v2369 != 0.0 {
                            let v2425 = v59 * (v11 * (((v59 + v2366) + (((v2366 + v12) * (v2366 + v380)).sqrt())).ln()));
                            v2439 = v2425;
                        } else {
                            let v2438 = v2437 + (v59 * (v11 * ((((v59 * v2367) + v12) + (((v12 + v2367) * (v12 + (v380 * v2367))).sqrt())).ln())));
                            v2439 = v2438;
                        }
                        let v2440 = v287 - v2439;
                        let v2442 = v288 - v2440;
                        let v2449 = v32 * ((v288 + v2440) - (((v2442 * v2442) + ((v406 * v11) * v11)).sqrt()));
                        v2350 = v2368;
                        v2351 = v2449;
                        v2352 = v2439;
                        v2353 = v2367;
                    } else {
                        v2350 = v1680;
                        v2351 = v1681;
                        v2352 = v172;
                        v2353 = v1683;
                    }
                    let v2452: f64;
                    let v2453: f64;
                    let v2454: f64;
                    let v2455: f64;
                    let v2456: f64;
                    if v250 != 0.0 {
                        v2452 = v2159;
                        v2453 = v2160;
                        v2454 = v2161;
                        v2455 = v2162;
                        v2456 = v172;
                    } else {
                        let v2450 = v50 * v2350;
                        let v2463: f64;
                        let v2464: f64;
                        let v2465: f64;
                        let v2466: f64;
                        let v2467: f64;
                        if v2451 != 0.0 {
                            v2463 = v2159;
                            v2464 = v2160;
                            v2465 = v2161;
                            v2466 = v2162;
                            v2467 = v172;
                        } else {
                            let v2457 = v81 - v2351;
                            let v2461 = v12 - ((v12 - (v2352 / v2457)).sqrt());
                            let v2477: f64;
                            if v2462 != 0.0 {
                                v2477 = v172;
                            } else {
                                let v2476 = ((((v2461 * v2461) * (v2461.ln())) / (v12 - v2461)) + v2461) * v2475;
                                v2477 = v2476;
                            }
                            let v2478 = v2461 + v2477;
                            let v2483: f64;
                            if v2462 != 0.0 {
                                let v2480 = (v2457 * v442).sqrt();
                                v2483 = v2480;
                            } else {
                                let v2482 = (v2457 * v442).powf(v100);
                                v2483 = v2482;
                            }
                            let v2484 = v448 * v2483;
                            let v2487 = v35 * ((v2353 - v12) * v2484);
                            let v2489 = v454 * (v2487 * v2478);
                            v2463 = v2484;
                            v2464 = v2457;
                            v2465 = v2478;
                            v2466 = v2487;
                            v2467 = v2489;
                        }
                        let v2503: f64;
                        if v2468 != 0.0 {
                            v2503 = v172;
                        } else {
                            let v2492 = v141 * ((v2463 * v456) / v2464);
                            let v2494 = (v460 * v132) / v2492;
                            let v2495 = v2494 * v2494;
                            let v2496 = v2495 * v2495;
                            let v2499 = (v2496 / (v2496 + v12)).sqrt();
                            let v2500 = v2499.sqrt();
                            let v2501 = v2499 * v2500;
                            let v2512: f64;
                            if v2502 != 0.0 {
                                let v2507 = v12 / (v12 + (v2492 * v2501));
                                v2512 = v2507;
                            } else {
                                let v2511 = (v12 + (v2492 * v2501)).powf(v2510);
                                v2512 = v2511;
                            }
                            let v2515 = (v2465 * v2512) / (v2465 + v2512);
                            let v2518 = (v485 * (v2492 / v2500)).sqrt();
                            let v2528 = (((v132 * v2494) * v2500) - (v132 * v2499)) + (v32 * (v2492 * v2501));
                            let v2530 = (((v59 * (v2494 * v2500)) - v2499) - v12) * v2518;
                            let v2531 = v2530 * v2530;
                            let v2532 = if v2530 > v172 { 1.0 } else { 0.0 };
                            out2532 = v2532;
                            let v2539: f64;
                            if v2532 != 0.0 {
                                let v2535 = v12 / (v12 + (v502 * v2530));
                                v2539 = v2535;
                            } else {
                                let v2538 = v12 / (v12 - (v502 * v2530));
                                v2539 = v2538;
                            }
                            let v2541 = (-v2531) + v2528;
                            let v2543 = if v2541 > v2542 { 1.0 } else { 0.0 };
                            out2543 = v2543;
                            let v2559: f64;
                            if v2543 != 0.0 {
                                let v2544 = v2541.exp();
                                v2559 = v2544;
                            } else {
                                let v2558 = v265 / (v12 + ((v2545 - v2541) * (v12 + (v32 * ((v2547 - v2541) * (v12 + ((v2549 - v2541) * v257)))))));
                                v2559 = v2558;
                            }
                            let v2561 = v2539 * v2539;
                            let v2567 = (((v530 * v2539) + (v533 * v2561)) + (v537 * (v2561 * v2539))) * v2559;
                            let v2570: f64;
                            if v2532 != 0.0 {
                                v2570 = v2567;
                            } else {
                                let v2569 = if v2528 > v2568 { 1.0 } else { 0.0 };
                                out2569 = v2569;
                                let v2593: f64;
                                if v2569 != 0.0 {
                                    let v2578 = v2528.exp();
                                    v2593 = v2578;
                                } else {
                                    let v2592 = v265 / (v12 + ((v2579 - v2528) * (v12 + (v32 * ((v2581 - v2528) * (v12 + ((v2583 - v2528) * v257)))))));
                                    v2593 = v2592;
                                }
                                let v2595 = (v59 * v2593) - v2567;
                                v2570 = v2595;
                            }
                            let v2577 = v550 * ((v2466 * (v2573 * ((v132 * v2570) / v2518))) * v2515);
                            v2503 = v2577;
                        }
                        let v2596: f64;
                        if v2504 != 0.0 {
                            v2596 = v172;
                        } else {
                            let v2599 = (-v174) / v2598;
                            let v2601 = if (v2599.abs()) < v245 { 1.0 } else { 0.0 };
                            out2601 = v2601;
                            let v2604: f64;
                            if v2601 != 0.0 {
                                let v2602 = v2599.exp();
                                v2604 = v2602;
                            } else {
                                let v2603 = if v2599 < v172 { 1.0 } else { 0.0 };
                                out2603 = v2603;
                                let v2631: f64;
                                if v2603 != 0.0 {
                                    let v2621 = v265 / (v12 + ((v2608 - v2599) * (v12 + (v32 * ((v2610 - v2599) * (v12 + ((v2612 - v2599) * v257)))))));
                                    v2631 = v2621;
                                } else {
                                    let v2622 = v2599 - v245;
                                    let v2630 = v275 * (v12 + (v2622 * (v12 + (v32 * (v2622 * (v12 + (v2622 * v257)))))));
                                    v2631 = v2630;
                                }
                                v2604 = v2631;
                            }
                            let v2607 = v581 * (v2605 * v2604);
                            v2596 = v2607;
                        }
                        let v2637 = (v610 * (((v2450 + v2467) + v2503) + v2596)) * v2636;
                        v2452 = v2463;
                        v2453 = v2464;
                        v2454 = v2465;
                        v2455 = v2466;
                        v2456 = v2637;
                    }
                    let v2640: f64;
                    let v2641: f64;
                    let v2642: f64;
                    let v2643: f64;
                    let v2644: f64;
                    if v280 != 0.0 {
                        v2640 = v2452;
                        v2641 = v2453;
                        v2642 = v2454;
                        v2643 = v2455;
                        v2644 = v172;
                    } else {
                        let v2638 = v53 * v2350;
                        let v2651: f64;
                        let v2652: f64;
                        let v2653: f64;
                        let v2654: f64;
                        let v2655: f64;
                        if v2639 != 0.0 {
                            v2651 = v2452;
                            v2652 = v2453;
                            v2653 = v2454;
                            v2654 = v2455;
                            v2655 = v172;
                        } else {
                            let v2645 = v88 - v2351;
                            let v2649 = v12 - ((v12 - (v2352 / v2645)).sqrt());
                            let v2665: f64;
                            if v2650 != 0.0 {
                                v2665 = v172;
                            } else {
                                let v2664 = ((((v2649 * v2649) * (v2649.ln())) / (v12 - v2649)) + v2649) * v2663;
                                v2665 = v2664;
                            }
                            let v2666 = v2649 + v2665;
                            let v2671: f64;
                            if v2650 != 0.0 {
                                let v2668 = (v2645 * v643).sqrt();
                                v2671 = v2668;
                            } else {
                                let v2670 = (v2645 * v643).powf(v105);
                                v2671 = v2670;
                            }
                            let v2672 = v649 * v2671;
                            let v2675 = v41 * ((v2353 - v12) * v2672);
                            let v2677 = v655 * (v2675 * v2666);
                            v2651 = v2672;
                            v2652 = v2645;
                            v2653 = v2666;
                            v2654 = v2675;
                            v2655 = v2677;
                        }
                        let v2691: f64;
                        if v2656 != 0.0 {
                            v2691 = v172;
                        } else {
                            let v2680 = v148 * ((v2651 * v657) / v2652);
                            let v2682 = (v460 * v133) / v2680;
                            let v2683 = v2682 * v2682;
                            let v2684 = v2683 * v2683;
                            let v2687 = (v2684 / (v2684 + v12)).sqrt();
                            let v2688 = v2687.sqrt();
                            let v2689 = v2687 * v2688;
                            let v2700: f64;
                            if v2690 != 0.0 {
                                let v2695 = v12 / (v12 + (v2680 * v2689));
                                v2700 = v2695;
                            } else {
                                let v2699 = (v12 + (v2680 * v2689)).powf(v2698);
                                v2700 = v2699;
                            }
                            let v2703 = (v2653 * v2700) / (v2653 + v2700);
                            let v2706 = (v485 * (v2680 / v2688)).sqrt();
                            let v2716 = (((v133 * v2682) * v2688) - (v133 * v2687)) + (v32 * (v2680 * v2689));
                            let v2718 = (((v59 * (v2682 * v2688)) - v2687) - v12) * v2706;
                            let v2719 = v2718 * v2718;
                            let v2720 = if v2718 > v172 { 1.0 } else { 0.0 };
                            out2720 = v2720;
                            let v2727: f64;
                            if v2720 != 0.0 {
                                let v2723 = v12 / (v12 + (v502 * v2718));
                                v2727 = v2723;
                            } else {
                                let v2726 = v12 / (v12 - (v502 * v2718));
                                v2727 = v2726;
                            }
                            let v2729 = (-v2719) + v2716;
                            let v2731 = if v2729 > v2730 { 1.0 } else { 0.0 };
                            out2731 = v2731;
                            let v2747: f64;
                            if v2731 != 0.0 {
                                let v2732 = v2729.exp();
                                v2747 = v2732;
                            } else {
                                let v2746 = v265 / (v12 + ((v2733 - v2729) * (v12 + (v32 * ((v2735 - v2729) * (v12 + ((v2737 - v2729) * v257)))))));
                                v2747 = v2746;
                            }
                            let v2749 = v2727 * v2727;
                            let v2755 = (((v530 * v2727) + (v533 * v2749)) + (v537 * (v2749 * v2727))) * v2747;
                            let v2758: f64;
                            if v2720 != 0.0 {
                                v2758 = v2755;
                            } else {
                                let v2757 = if v2716 > v2756 { 1.0 } else { 0.0 };
                                out2757 = v2757;
                                let v2781: f64;
                                if v2757 != 0.0 {
                                    let v2766 = v2716.exp();
                                    v2781 = v2766;
                                } else {
                                    let v2780 = v265 / (v12 + ((v2767 - v2716) * (v12 + (v32 * ((v2769 - v2716) * (v12 + ((v2771 - v2716) * v257)))))));
                                    v2781 = v2780;
                                }
                                let v2783 = (v59 * v2781) - v2755;
                                v2758 = v2783;
                            }
                            let v2765 = v745 * ((v2654 * (v2761 * ((v133 * v2758) / v2706))) * v2703);
                            v2691 = v2765;
                        }
                        let v2784: f64;
                        if v2692 != 0.0 {
                            v2784 = v172;
                        } else {
                            let v2787 = (-v176) / v2786;
                            let v2789 = if (v2787.abs()) < v245 { 1.0 } else { 0.0 };
                            out2789 = v2789;
                            let v2792: f64;
                            if v2789 != 0.0 {
                                let v2790 = v2787.exp();
                                v2792 = v2790;
                            } else {
                                let v2791 = if v2787 < v172 { 1.0 } else { 0.0 };
                                out2791 = v2791;
                                let v2819: f64;
                                if v2791 != 0.0 {
                                    let v2809 = v265 / (v12 + ((v2796 - v2787) * (v12 + (v32 * ((v2798 - v2787) * (v12 + ((v2800 - v2787) * v257)))))));
                                    v2819 = v2809;
                                } else {
                                    let v2810 = v2787 - v245;
                                    let v2818 = v275 * (v12 + (v2810 * (v12 + (v32 * (v2810 * (v12 + (v2810 * v257)))))));
                                    v2819 = v2818;
                                }
                                v2792 = v2819;
                            }
                            let v2795 = v776 * (v2793 * v2792);
                            v2784 = v2795;
                        }
                        let v2825 = (v610 * (((v2638 + v2655) + v2691) + v2784)) * v2824;
                        v2640 = v2651;
                        v2641 = v2652;
                        v2642 = v2653;
                        v2643 = v2654;
                        v2644 = v2825;
                    }
                    let v2828: f64;
                    let v2829: f64;
                    let v2830: f64;
                    let v2831: f64;
                    let v2832: f64;
                    if v283 != 0.0 {
                        v2828 = v172;
                        v2829 = v2640;
                        v2830 = v2641;
                        v2831 = v2642;
                        v2832 = v2643;
                    } else {
                        let v2826 = v56 * v2350;
                        let v2844: f64;
                        let v2845: f64;
                        let v2846: f64;
                        let v2847: f64;
                        let v2848: f64;
                        if v2827 != 0.0 {
                            v2844 = v2640;
                            v2845 = v2641;
                            v2846 = v2642;
                            v2847 = v2643;
                            v2848 = v172;
                        } else {
                            let v2838 = v95 - v2351;
                            let v2842 = v12 - ((v12 - (v2352 / v2838)).sqrt());
                            let v2858: f64;
                            if v2843 != 0.0 {
                                v2858 = v172;
                            } else {
                                let v2857 = ((((v2842 * v2842) * (v2842.ln())) / (v12 - v2842)) + v2842) * v2856;
                                v2858 = v2857;
                            }
                            let v2859 = v2842 + v2858;
                            let v2864: f64;
                            if v2843 != 0.0 {
                                let v2861 = (v2838 * v842).sqrt();
                                v2864 = v2861;
                            } else {
                                let v2863 = (v2838 * v842).powf(v110);
                                v2864 = v2863;
                            }
                            let v2865 = v848 * v2864;
                            let v2868 = v47 * ((v2353 - v12) * v2865);
                            let v2870 = v854 * (v2868 * v2859);
                            v2844 = v2865;
                            v2845 = v2838;
                            v2846 = v2859;
                            v2847 = v2868;
                            v2848 = v2870;
                        }
                        let v2884: f64;
                        if v2849 != 0.0 {
                            v2884 = v172;
                        } else {
                            let v2873 = v155 * ((v2844 * v856) / v2845);
                            let v2875 = (v460 * v134) / v2873;
                            let v2876 = v2875 * v2875;
                            let v2877 = v2876 * v2876;
                            let v2880 = (v2877 / (v2877 + v12)).sqrt();
                            let v2881 = v2880.sqrt();
                            let v2882 = v2880 * v2881;
                            let v2893: f64;
                            if v2883 != 0.0 {
                                let v2888 = v12 / (v12 + (v2873 * v2882));
                                v2893 = v2888;
                            } else {
                                let v2892 = (v12 + (v2873 * v2882)).powf(v2891);
                                v2893 = v2892;
                            }
                            let v2896 = (v2846 * v2893) / (v2846 + v2893);
                            let v2899 = (v485 * (v2873 / v2881)).sqrt();
                            let v2909 = (((v134 * v2875) * v2881) - (v134 * v2880)) + (v32 * (v2873 * v2882));
                            let v2911 = (((v59 * (v2875 * v2881)) - v2880) - v12) * v2899;
                            let v2912 = v2911 * v2911;
                            let v2913 = if v2911 > v172 { 1.0 } else { 0.0 };
                            out2913 = v2913;
                            let v2920: f64;
                            if v2913 != 0.0 {
                                let v2916 = v12 / (v12 + (v502 * v2911));
                                v2920 = v2916;
                            } else {
                                let v2919 = v12 / (v12 - (v502 * v2911));
                                v2920 = v2919;
                            }
                            let v2922 = (-v2912) + v2909;
                            let v2924 = if v2922 > v2923 { 1.0 } else { 0.0 };
                            out2924 = v2924;
                            let v2940: f64;
                            if v2924 != 0.0 {
                                let v2925 = v2922.exp();
                                v2940 = v2925;
                            } else {
                                let v2939 = v265 / (v12 + ((v2926 - v2922) * (v12 + (v32 * ((v2928 - v2922) * (v12 + ((v2930 - v2922) * v257)))))));
                                v2940 = v2939;
                            }
                            let v2942 = v2920 * v2920;
                            let v2948 = (((v530 * v2920) + (v533 * v2942)) + (v537 * (v2942 * v2920))) * v2940;
                            let v2951: f64;
                            if v2913 != 0.0 {
                                v2951 = v2948;
                            } else {
                                let v2950 = if v2909 > v2949 { 1.0 } else { 0.0 };
                                out2950 = v2950;
                                let v2974: f64;
                                if v2950 != 0.0 {
                                    let v2959 = v2909.exp();
                                    v2974 = v2959;
                                } else {
                                    let v2973 = v265 / (v12 + ((v2960 - v2909) * (v12 + (v32 * ((v2962 - v2909) * (v12 + ((v2964 - v2909) * v257)))))));
                                    v2974 = v2973;
                                }
                                let v2976 = (v59 * v2974) - v2948;
                                v2951 = v2976;
                            }
                            let v2958 = v944 * ((v2847 * (v2954 * ((v134 * v2951) / v2899))) * v2896);
                            v2884 = v2958;
                        }
                        let v2977: f64;
                        if v2885 != 0.0 {
                            v2977 = v172;
                        } else {
                            let v2980 = (-v178) / v2979;
                            let v2982 = if (v2980.abs()) < v245 { 1.0 } else { 0.0 };
                            out2982 = v2982;
                            let v2985: f64;
                            if v2982 != 0.0 {
                                let v2983 = v2980.exp();
                                v2985 = v2983;
                            } else {
                                let v2984 = if v2980 < v172 { 1.0 } else { 0.0 };
                                out2984 = v2984;
                                let v3012: f64;
                                if v2984 != 0.0 {
                                    let v3002 = v265 / (v12 + ((v2989 - v2980) * (v12 + (v32 * ((v2991 - v2980) * (v12 + ((v2993 - v2980) * v257)))))));
                                    v3012 = v3002;
                                } else {
                                    let v3003 = v2980 - v245;
                                    let v3011 = v275 * (v12 + (v3003 * (v12 + (v32 * (v3003 * (v12 + (v3003 * v257)))))));
                                    v3012 = v3011;
                                }
                                v2985 = v3012;
                            }
                            let v2988 = v975 * (v2986 * v2985);
                            v2977 = v2988;
                        }
                        let v3018 = (v610 * (((v2826 + v2848) + v2884) + v2977)) * v3017;
                        v2828 = v3018;
                        v2829 = v2844;
                        v2830 = v2845;
                        v2831 = v2846;
                        v2832 = v2847;
                    }
                    let v2837 = ((v215 * v2456) + (v225 * v2644)) + (v233 * v2828);
                    let v3021: f64;
                    let v3022: f64;
                    let v3023: f64;
                    let v3024: f64;
                    if v293 != 0.0 {
                        let v3020 = if v3019 < v242 { 1.0 } else { 0.0 };
                        out3020 = v3020;
                        let v3036: f64;
                        let v3037: f64;
                        let v3038: f64;
                        if v3020 != 0.0 {
                            let v3025 = v3019 * v13;
                            let v3029 = if ((v3026 * v3025).abs()) < v245 { 1.0 } else { 0.0 };
                            out3029 = v3029;
                            let v3047: f64;
                            if v3029 != 0.0 {
                                let v3043 = (v3041 * v3025).exp();
                                v3047 = v3043;
                            } else {
                                let v3046 = if (v3044 * v3025) < v172 { 1.0 } else { 0.0 };
                                out3046 = v3046;
                                let v3087: f64;
                                if v3046 != 0.0 {
                                    let v3069 = v265 / (v12 + ((v3052 - (v3050 * v3025)) * (v12 + (v32 * ((v3056 - (v3054 * v3025)) * (v12 + ((v3060 - (v3058 * v3025)) * v257)))))));
                                    v3087 = v3069;
                                } else {
                                    let v3086 = v275 * (v12 + (((v3070 * v3025) - v245) * (v12 + (v32 * (((v3073 * v3025) - v245) * (v12 + (((v3076 * v3025) - v245) * v257)))))));
                                    v3087 = v3086;
                                }
                                v3047 = v3087;
                            }
                            let v3048 = v12 / v3047;
                            let v3049 = v3048 * v3048;
                            v3036 = v3049;
                            v3037 = v3047;
                            v3038 = v3048;
                        } else {
                            let v3033 = (v12 + ((v3019 - v242) * v13)) * v249;
                            let v3034 = v3033.sqrt();
                            let v3035 = v12 / v3034;
                            v3036 = v3033;
                            v3037 = v3035;
                            v3038 = v3034;
                        }
                        let v3039 = v3036 - v12;
                        let v3110: f64;
                        if v3040 != 0.0 {
                            let v3096 = v59 * (v11 * (((v59 + v3037) + (((v3037 + v12) * (v3037 + v380)).sqrt())).ln()));
                            v3110 = v3096;
                        } else {
                            let v3109 = v3108 + (v59 * (v11 * ((((v59 * v3038) + v12) + (((v12 + v3038) * (v12 + (v380 * v3038))).sqrt())).ln())));
                            v3110 = v3109;
                        }
                        let v3111 = v287 - v3110;
                        let v3113 = v3019 - v3111;
                        let v3120 = v32 * ((v3019 + v3111) - (((v3113 * v3113) + ((v406 * v11) * v11)).sqrt()));
                        v3021 = v3039;
                        v3022 = v3120;
                        v3023 = v3110;
                        v3024 = v3038;
                    } else {
                        v3021 = v2350;
                        v3022 = v2351;
                        v3023 = v172;
                        v3024 = v2353;
                    }
                    let v3123: f64;
                    let v3124: f64;
                    let v3125: f64;
                    let v3126: f64;
                    let v3127: f64;
                    if v250 != 0.0 {
                        v3123 = v2829;
                        v3124 = v2830;
                        v3125 = v2831;
                        v3126 = v2832;
                        v3127 = v172;
                    } else {
                        let v3121 = v50 * v3021;
                        let v3134: f64;
                        let v3135: f64;
                        let v3136: f64;
                        let v3137: f64;
                        let v3138: f64;
                        if v3122 != 0.0 {
                            v3134 = v2829;
                            v3135 = v2830;
                            v3136 = v2831;
                            v3137 = v2832;
                            v3138 = v172;
                        } else {
                            let v3128 = v81 - v3022;
                            let v3132 = v12 - ((v12 - (v3023 / v3128)).sqrt());
                            let v3148: f64;
                            if v3133 != 0.0 {
                                v3148 = v172;
                            } else {
                                let v3147 = ((((v3132 * v3132) * (v3132.ln())) / (v12 - v3132)) + v3132) * v3146;
                                v3148 = v3147;
                            }
                            let v3149 = v3132 + v3148;
                            let v3154: f64;
                            if v3133 != 0.0 {
                                let v3151 = (v3128 * v442).sqrt();
                                v3154 = v3151;
                            } else {
                                let v3153 = (v3128 * v442).powf(v100);
                                v3154 = v3153;
                            }
                            let v3155 = v448 * v3154;
                            let v3158 = v35 * ((v3024 - v12) * v3155);
                            let v3160 = v454 * (v3158 * v3149);
                            v3134 = v3155;
                            v3135 = v3128;
                            v3136 = v3149;
                            v3137 = v3158;
                            v3138 = v3160;
                        }
                        let v3174: f64;
                        if v3139 != 0.0 {
                            v3174 = v172;
                        } else {
                            let v3163 = v141 * ((v3134 * v456) / v3135);
                            let v3165 = (v460 * v132) / v3163;
                            let v3166 = v3165 * v3165;
                            let v3167 = v3166 * v3166;
                            let v3170 = (v3167 / (v3167 + v12)).sqrt();
                            let v3171 = v3170.sqrt();
                            let v3172 = v3170 * v3171;
                            let v3183: f64;
                            if v3173 != 0.0 {
                                let v3178 = v12 / (v12 + (v3163 * v3172));
                                v3183 = v3178;
                            } else {
                                let v3182 = (v12 + (v3163 * v3172)).powf(v3181);
                                v3183 = v3182;
                            }
                            let v3186 = (v3136 * v3183) / (v3136 + v3183);
                            let v3189 = (v485 * (v3163 / v3171)).sqrt();
                            let v3199 = (((v132 * v3165) * v3171) - (v132 * v3170)) + (v32 * (v3163 * v3172));
                            let v3201 = (((v59 * (v3165 * v3171)) - v3170) - v12) * v3189;
                            let v3202 = v3201 * v3201;
                            let v3203 = if v3201 > v172 { 1.0 } else { 0.0 };
                            out3203 = v3203;
                            let v3210: f64;
                            if v3203 != 0.0 {
                                let v3206 = v12 / (v12 + (v502 * v3201));
                                v3210 = v3206;
                            } else {
                                let v3209 = v12 / (v12 - (v502 * v3201));
                                v3210 = v3209;
                            }
                            let v3212 = (-v3202) + v3199;
                            let v3214 = if v3212 > v3213 { 1.0 } else { 0.0 };
                            out3214 = v3214;
                            let v3230: f64;
                            if v3214 != 0.0 {
                                let v3215 = v3212.exp();
                                v3230 = v3215;
                            } else {
                                let v3229 = v265 / (v12 + ((v3216 - v3212) * (v12 + (v32 * ((v3218 - v3212) * (v12 + ((v3220 - v3212) * v257)))))));
                                v3230 = v3229;
                            }
                            let v3232 = v3210 * v3210;
                            let v3238 = (((v530 * v3210) + (v533 * v3232)) + (v537 * (v3232 * v3210))) * v3230;
                            let v3241: f64;
                            if v3203 != 0.0 {
                                v3241 = v3238;
                            } else {
                                let v3240 = if v3199 > v3239 { 1.0 } else { 0.0 };
                                out3240 = v3240;
                                let v3264: f64;
                                if v3240 != 0.0 {
                                    let v3249 = v3199.exp();
                                    v3264 = v3249;
                                } else {
                                    let v3263 = v265 / (v12 + ((v3250 - v3199) * (v12 + (v32 * ((v3252 - v3199) * (v12 + ((v3254 - v3199) * v257)))))));
                                    v3264 = v3263;
                                }
                                let v3266 = (v59 * v3264) - v3238;
                                v3241 = v3266;
                            }
                            let v3248 = v550 * ((v3137 * (v3244 * ((v132 * v3241) / v3189))) * v3186);
                            v3174 = v3248;
                        }
                        let v3267: f64;
                        if v3175 != 0.0 {
                            v3267 = v172;
                        } else {
                            let v3270 = (-v174) / v3269;
                            let v3272 = if (v3270.abs()) < v245 { 1.0 } else { 0.0 };
                            out3272 = v3272;
                            let v3275: f64;
                            if v3272 != 0.0 {
                                let v3273 = v3270.exp();
                                v3275 = v3273;
                            } else {
                                let v3274 = if v3270 < v172 { 1.0 } else { 0.0 };
                                out3274 = v3274;
                                let v3302: f64;
                                if v3274 != 0.0 {
                                    let v3292 = v265 / (v12 + ((v3279 - v3270) * (v12 + (v32 * ((v3281 - v3270) * (v12 + ((v3283 - v3270) * v257)))))));
                                    v3302 = v3292;
                                } else {
                                    let v3293 = v3270 - v245;
                                    let v3301 = v275 * (v12 + (v3293 * (v12 + (v32 * (v3293 * (v12 + (v3293 * v257)))))));
                                    v3302 = v3301;
                                }
                                v3275 = v3302;
                            }
                            let v3278 = v581 * (v3276 * v3275);
                            v3267 = v3278;
                        }
                        let v3308 = (v610 * (((v3121 + v3138) + v3174) + v3267)) * v3307;
                        v3123 = v3134;
                        v3124 = v3135;
                        v3125 = v3136;
                        v3126 = v3137;
                        v3127 = v3308;
                    }
                    let v3311: f64;
                    let v3312: f64;
                    let v3313: f64;
                    let v3314: f64;
                    let v3315: f64;
                    if v280 != 0.0 {
                        v3311 = v3123;
                        v3312 = v3124;
                        v3313 = v3125;
                        v3314 = v3126;
                        v3315 = v172;
                    } else {
                        let v3309 = v53 * v3021;
                        let v3322: f64;
                        let v3323: f64;
                        let v3324: f64;
                        let v3325: f64;
                        let v3326: f64;
                        if v3310 != 0.0 {
                            v3322 = v3123;
                            v3323 = v3124;
                            v3324 = v3125;
                            v3325 = v3126;
                            v3326 = v172;
                        } else {
                            let v3316 = v88 - v3022;
                            let v3320 = v12 - ((v12 - (v3023 / v3316)).sqrt());
                            let v3336: f64;
                            if v3321 != 0.0 {
                                v3336 = v172;
                            } else {
                                let v3335 = ((((v3320 * v3320) * (v3320.ln())) / (v12 - v3320)) + v3320) * v3334;
                                v3336 = v3335;
                            }
                            let v3337 = v3320 + v3336;
                            let v3342: f64;
                            if v3321 != 0.0 {
                                let v3339 = (v3316 * v643).sqrt();
                                v3342 = v3339;
                            } else {
                                let v3341 = (v3316 * v643).powf(v105);
                                v3342 = v3341;
                            }
                            let v3343 = v649 * v3342;
                            let v3346 = v41 * ((v3024 - v12) * v3343);
                            let v3348 = v655 * (v3346 * v3337);
                            v3322 = v3343;
                            v3323 = v3316;
                            v3324 = v3337;
                            v3325 = v3346;
                            v3326 = v3348;
                        }
                        let v3362: f64;
                        if v3327 != 0.0 {
                            v3362 = v172;
                        } else {
                            let v3351 = v148 * ((v3322 * v657) / v3323);
                            let v3353 = (v460 * v133) / v3351;
                            let v3354 = v3353 * v3353;
                            let v3355 = v3354 * v3354;
                            let v3358 = (v3355 / (v3355 + v12)).sqrt();
                            let v3359 = v3358.sqrt();
                            let v3360 = v3358 * v3359;
                            let v3371: f64;
                            if v3361 != 0.0 {
                                let v3366 = v12 / (v12 + (v3351 * v3360));
                                v3371 = v3366;
                            } else {
                                let v3370 = (v12 + (v3351 * v3360)).powf(v3369);
                                v3371 = v3370;
                            }
                            let v3374 = (v3324 * v3371) / (v3324 + v3371);
                            let v3377 = (v485 * (v3351 / v3359)).sqrt();
                            let v3387 = (((v133 * v3353) * v3359) - (v133 * v3358)) + (v32 * (v3351 * v3360));
                            let v3389 = (((v59 * (v3353 * v3359)) - v3358) - v12) * v3377;
                            let v3390 = v3389 * v3389;
                            let v3391 = if v3389 > v172 { 1.0 } else { 0.0 };
                            out3391 = v3391;
                            let v3398: f64;
                            if v3391 != 0.0 {
                                let v3394 = v12 / (v12 + (v502 * v3389));
                                v3398 = v3394;
                            } else {
                                let v3397 = v12 / (v12 - (v502 * v3389));
                                v3398 = v3397;
                            }
                            let v3400 = (-v3390) + v3387;
                            let v3402 = if v3400 > v3401 { 1.0 } else { 0.0 };
                            out3402 = v3402;
                            let v3418: f64;
                            if v3402 != 0.0 {
                                let v3403 = v3400.exp();
                                v3418 = v3403;
                            } else {
                                let v3417 = v265 / (v12 + ((v3404 - v3400) * (v12 + (v32 * ((v3406 - v3400) * (v12 + ((v3408 - v3400) * v257)))))));
                                v3418 = v3417;
                            }
                            let v3420 = v3398 * v3398;
                            let v3426 = (((v530 * v3398) + (v533 * v3420)) + (v537 * (v3420 * v3398))) * v3418;
                            let v3429: f64;
                            if v3391 != 0.0 {
                                v3429 = v3426;
                            } else {
                                let v3428 = if v3387 > v3427 { 1.0 } else { 0.0 };
                                out3428 = v3428;
                                let v3452: f64;
                                if v3428 != 0.0 {
                                    let v3437 = v3387.exp();
                                    v3452 = v3437;
                                } else {
                                    let v3451 = v265 / (v12 + ((v3438 - v3387) * (v12 + (v32 * ((v3440 - v3387) * (v12 + ((v3442 - v3387) * v257)))))));
                                    v3452 = v3451;
                                }
                                let v3454 = (v59 * v3452) - v3426;
                                v3429 = v3454;
                            }
                            let v3436 = v745 * ((v3325 * (v3432 * ((v133 * v3429) / v3377))) * v3374);
                            v3362 = v3436;
                        }
                        let v3455: f64;
                        if v3363 != 0.0 {
                            v3455 = v172;
                        } else {
                            let v3458 = (-v176) / v3457;
                            let v3460 = if (v3458.abs()) < v245 { 1.0 } else { 0.0 };
                            out3460 = v3460;
                            let v3463: f64;
                            if v3460 != 0.0 {
                                let v3461 = v3458.exp();
                                v3463 = v3461;
                            } else {
                                let v3462 = if v3458 < v172 { 1.0 } else { 0.0 };
                                out3462 = v3462;
                                let v3490: f64;
                                if v3462 != 0.0 {
                                    let v3480 = v265 / (v12 + ((v3467 - v3458) * (v12 + (v32 * ((v3469 - v3458) * (v12 + ((v3471 - v3458) * v257)))))));
                                    v3490 = v3480;
                                } else {
                                    let v3481 = v3458 - v245;
                                    let v3489 = v275 * (v12 + (v3481 * (v12 + (v32 * (v3481 * (v12 + (v3481 * v257)))))));
                                    v3490 = v3489;
                                }
                                v3463 = v3490;
                            }
                            let v3466 = v776 * (v3464 * v3463);
                            v3455 = v3466;
                        }
                        let v3496 = (v610 * (((v3309 + v3326) + v3362) + v3455)) * v3495;
                        v3311 = v3322;
                        v3312 = v3323;
                        v3313 = v3324;
                        v3314 = v3325;
                        v3315 = v3496;
                    }
                    let v3499: f64;
                    if v283 != 0.0 {
                        v3499 = v172;
                    } else {
                        let v3497 = v56 * v3021;
                        let v3523: f64;
                        let v3524: f64;
                        let v3525: f64;
                        let v3526: f64;
                        let v3527: f64;
                        if v3498 != 0.0 {
                            v3523 = v3311;
                            v3524 = v3312;
                            v3525 = v3313;
                            v3526 = v3314;
                            v3527 = v172;
                        } else {
                            let v3517 = v95 - v3022;
                            let v3521 = v12 - ((v12 - (v3023 / v3517)).sqrt());
                            let v3537: f64;
                            if v3522 != 0.0 {
                                v3537 = v172;
                            } else {
                                let v3536 = ((((v3521 * v3521) * (v3521.ln())) / (v12 - v3521)) + v3521) * v3535;
                                v3537 = v3536;
                            }
                            let v3538 = v3521 + v3537;
                            let v3543: f64;
                            if v3522 != 0.0 {
                                let v3540 = (v3517 * v842).sqrt();
                                v3543 = v3540;
                            } else {
                                let v3542 = (v3517 * v842).powf(v110);
                                v3543 = v3542;
                            }
                            let v3544 = v848 * v3543;
                            let v3547 = v47 * ((v3024 - v12) * v3544);
                            let v3549 = v854 * (v3547 * v3538);
                            v3523 = v3544;
                            v3524 = v3517;
                            v3525 = v3538;
                            v3526 = v3547;
                            v3527 = v3549;
                        }
                        let v3563: f64;
                        if v3528 != 0.0 {
                            v3563 = v172;
                        } else {
                            let v3552 = v155 * ((v3523 * v856) / v3524);
                            let v3554 = (v460 * v134) / v3552;
                            let v3555 = v3554 * v3554;
                            let v3556 = v3555 * v3555;
                            let v3559 = (v3556 / (v3556 + v12)).sqrt();
                            let v3560 = v3559.sqrt();
                            let v3561 = v3559 * v3560;
                            let v3572: f64;
                            if v3562 != 0.0 {
                                let v3567 = v12 / (v12 + (v3552 * v3561));
                                v3572 = v3567;
                            } else {
                                let v3571 = (v12 + (v3552 * v3561)).powf(v3570);
                                v3572 = v3571;
                            }
                            let v3575 = (v3525 * v3572) / (v3525 + v3572);
                            let v3578 = (v485 * (v3552 / v3560)).sqrt();
                            let v3588 = (((v134 * v3554) * v3560) - (v134 * v3559)) + (v32 * (v3552 * v3561));
                            let v3590 = (((v59 * (v3554 * v3560)) - v3559) - v12) * v3578;
                            let v3591 = v3590 * v3590;
                            let v3592 = if v3590 > v172 { 1.0 } else { 0.0 };
                            out3592 = v3592;
                            let v3599: f64;
                            if v3592 != 0.0 {
                                let v3595 = v12 / (v12 + (v502 * v3590));
                                v3599 = v3595;
                            } else {
                                let v3598 = v12 / (v12 - (v502 * v3590));
                                v3599 = v3598;
                            }
                            let v3601 = (-v3591) + v3588;
                            let v3603 = if v3601 > v3602 { 1.0 } else { 0.0 };
                            out3603 = v3603;
                            let v3619: f64;
                            if v3603 != 0.0 {
                                let v3604 = v3601.exp();
                                v3619 = v3604;
                            } else {
                                let v3618 = v265 / (v12 + ((v3605 - v3601) * (v12 + (v32 * ((v3607 - v3601) * (v12 + ((v3609 - v3601) * v257)))))));
                                v3619 = v3618;
                            }
                            let v3621 = v3599 * v3599;
                            let v3627 = (((v530 * v3599) + (v533 * v3621)) + (v537 * (v3621 * v3599))) * v3619;
                            let v3630: f64;
                            if v3592 != 0.0 {
                                v3630 = v3627;
                            } else {
                                let v3629 = if v3588 > v3628 { 1.0 } else { 0.0 };
                                out3629 = v3629;
                                let v3653: f64;
                                if v3629 != 0.0 {
                                    let v3638 = v3588.exp();
                                    v3653 = v3638;
                                } else {
                                    let v3652 = v265 / (v12 + ((v3639 - v3588) * (v12 + (v32 * ((v3641 - v3588) * (v12 + ((v3643 - v3588) * v257)))))));
                                    v3653 = v3652;
                                }
                                let v3655 = (v59 * v3653) - v3627;
                                v3630 = v3655;
                            }
                            let v3637 = v944 * ((v3526 * (v3633 * ((v134 * v3630) / v3578))) * v3575);
                            v3563 = v3637;
                        }
                        let v3656: f64;
                        if v3564 != 0.0 {
                            v3656 = v172;
                        } else {
                            let v3659 = (-v178) / v3658;
                            let v3661 = if (v3659.abs()) < v245 { 1.0 } else { 0.0 };
                            out3661 = v3661;
                            let v3664: f64;
                            if v3661 != 0.0 {
                                let v3662 = v3659.exp();
                                v3664 = v3662;
                            } else {
                                let v3663 = if v3659 < v172 { 1.0 } else { 0.0 };
                                out3663 = v3663;
                                let v3691: f64;
                                if v3663 != 0.0 {
                                    let v3681 = v265 / (v12 + ((v3668 - v3659) * (v12 + (v32 * ((v3670 - v3659) * (v12 + ((v3672 - v3659) * v257)))))));
                                    v3691 = v3681;
                                } else {
                                    let v3682 = v3659 - v245;
                                    let v3690 = v275 * (v12 + (v3682 * (v12 + (v32 * (v3682 * (v12 + (v3682 * v257)))))));
                                    v3691 = v3690;
                                }
                                v3664 = v3691;
                            }
                            let v3667 = v975 * (v3665 * v3664);
                            v3656 = v3667;
                        }
                        let v3697 = (v610 * (((v3497 + v3527) + v3563) + v3656)) * v3696;
                        v3499 = v3697;
                    }
                    let v3504 = ((v215 * v3127) + (v225 * v3315)) + (v233 * v3499);
                    let v3506 = (v216 + v226) + v234;
                    let v3507 = v288 * v13;
                    let v3511 = v2837 - (v3506 * ((v3507.exp()) - v12));
                    let v3516 = v3504 - (v3506 * (((v3019 * v13).exp()) - v12));
                    let v3701: f64;
                    let v3702: f64;
                    let v3703: f64;
                    let v3704: f64;
                    let v3705: f64;
                    if v293 != 0.0 {
                        let v3700 = if (if v2837 > v172 { 1.0 } else { 0.0 }) != 0.0 && (if v3504 > v172 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        out3700 = v3700;
                        let v3726: f64;
                        let v3727: f64;
                        if v3700 != 0.0 {
                            let v3725 = if (if (if (if (if (v3511 / v2837) > v3715 { 1.0 } else { 0.0 }) != 0.0 || (if (v3516 / v3504) > v3715 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v3511 > v172 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v3516 > v172 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v3516 > v3511 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            out3725 = v3725;
                            let v3772: f64;
                            let v3773: f64;
                            if v3725 != 0.0 {
                                let v3767 = (v11 * ((v3511 / v3516).ln())) / v3766;
                                let v3771 = v3511 / (((v3507 * v3767).exp()) - v12);
                                v3772 = v3771;
                                v3773 = v3767;
                            } else {
                                v3772 = v172;
                                v3773 = v12;
                            }
                            v3726 = v3772;
                            v3727 = v3773;
                        } else {
                            v3726 = v172;
                            v3727 = v12;
                        }
                        let v3728 = v309 * v13;
                        let v3737 = (v819 - (v3506 * ((v3728.exp()) - v12))) - (v3726 * (((v3728 * v3727).exp()) - v12));
                        let v3738 = v1007 * v13;
                        let v3747 = (v1496 - (v3506 * ((v3738.exp()) - v12))) - (v3726 * (((v3738 * v3727).exp()) - v12));
                        let v3748 = v1678 * v13;
                        let v3757 = (v2167 - (v3506 * ((v3748.exp()) - v12))) - (v3726 * (((v3748 * v3727).exp()) - v12));
                        let v3762 = if (if (if v819 < v172 { 1.0 } else { 0.0 }) != 0.0 && (if v1496 < v172 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v2167 < v172 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        out3762 = v3762;
                        let v3788: f64;
                        let v3789: f64;
                        let v3790: f64;
                        if v3762 != 0.0 {
                            let v3787 = if (if (if (if (if (if (v3737 / v819) > v3715 { 1.0 } else { 0.0 }) != 0.0 || (if (v3747 / v1496) > v3715 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (v3757 / v2167) > v3715 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v3737 < v172 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v3747 < v172 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v3757 < v172 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            out3787 = v3787;
                            let v3816: f64;
                            let v3817: f64;
                            let v3818: f64;
                            if v3787 != 0.0 {
                                let v3791 = v3737 / v3747;
                                let v3795 = v309 - v1007;
                                let v3797 = v1007 - v309;
                                let v3811 = (((-v11) * (v3791.ln())) / v3795) + (((v11 * (v3791 - v12)) * ((v3791.powf((v1007 / v3797))) - v12)) / ((((v3791.powf((v309 / v3795))) * v3797) + (v3791 * v309)) - v1007));
                                let v3815 = if ((v3748 * v3811).abs()) < v3814 { 1.0 } else { 0.0 };
                                out3815 = v3815;
                                let v3836: f64;
                                let v3837: f64;
                                let v3838: f64;
                                if v3815 != 0.0 {
                                    let v3823 = v3757 * ((v12 / v1678) + ((v32 * v13) * v3811));
                                    let v3828 = (((v3824 * v3757) * v3811) * v13) / v1678;
                                    v3836 = v3823;
                                    v3837 = v12;
                                    v3838 = v3828;
                                } else {
                                    let v3835 = (-v3757) / (((((-v1678) * v13) * v3811).exp()) - v12);
                                    v3836 = v3835;
                                    v3837 = v172;
                                    v3838 = v3811;
                                }
                                v3816 = v3836;
                                v3817 = v3837;
                                v3818 = v3838;
                            } else {
                                v3816 = v172;
                                v3817 = v172;
                                v3818 = v12;
                            }
                            v3788 = v3816;
                            v3789 = v3817;
                            v3790 = v3818;
                        } else {
                            v3788 = v172;
                            v3789 = v172;
                            v3790 = v12;
                        }
                        v3701 = v3726;
                        v3702 = v3788;
                        v3703 = v3727;
                        v3704 = v3789;
                        v3705 = v3790;
                    } else {
                        v3701 = v172;
                        v3702 = v172;
                        v3703 = v12;
                        v3704 = v172;
                        v3705 = v12;
                    }
                    let v3706 = v215 * v103;
                    let v3707 = v225 * v108;
                    let v3709 = v233 * v113;
                    let v3712 = v3711 * ((v3706 + v3707) + v3709);
                    let v3713 = if v3706 <= v3712 { 1.0 } else { 0.0 };
                    out3713 = v3713;
                    let v3839: f64;
                    if v3713 != 0.0 {
                        v3839 = v172;
                    } else {
                        v3839 = v12;
                    }
                    let v3840 = if v3707 <= v3712 { 1.0 } else { 0.0 };
                    out3840 = v3840;
                    let v3841: f64;
                    if v3840 != 0.0 {
                        v3841 = v172;
                    } else {
                        v3841 = v12;
                    }
                    let v3842 = if v3709 <= v3712 { 1.0 } else { 0.0 };
                    out3842 = v3842;
                    let v3843: f64;
                    if v3842 != 0.0 {
                        v3843 = v172;
                    } else {
                        v3843 = v12;
                    }
                    let v3856: f64;
                    let v3857: f64;
                    let v3858: f64;
                    if v293 != 0.0 {
                        let v3848 = (v3846 / (v3506 + v3844)).ln();
                        let v3851 = (v3846 / (v3701 + v3844)).ln();
                        let v3855 = (v3846 / ((v3702.abs()) + v3844)).ln();
                        v3856 = v3848;
                        v3857 = v3851;
                        v3858 = v3855;
                    } else {
                        v3856 = v172;
                        v3857 = v172;
                        v3858 = v172;
                    }
                    let v3859 = if v3856 <= v245 { v3856 } else { v245 };
                    let v3860 = v3859.exp();
                    let v3861 = if v3857 <= v245 { v3857 } else { v245 };
                    let v3862 = v3861.exp();
                    let v3863 = if v3858 <= v245 { v3858 } else { v245 };
                    let v3864 = v3863.exp();
                    v294 = v3859;
                    v295 = v3860;
                    v296 = v3506;
                    v297 = v3703;
                    v298 = v3861;
                    v299 = v3862;
                    v300 = v3701;
                    v301 = v3704;
                    v302 = v3702;
                    v303 = v3705;
                    v304 = v3863;
                    v305 = v3864;
                    v306 = v3839;
                    v307 = v3841;
                    v308 = v3843;
                } else {
                    v294 = v172;
                    v295 = v172;
                    v296 = v172;
                    v297 = v12;
                    v298 = v172;
                    v299 = v172;
                    v300 = v172;
                    v301 = v172;
                    v302 = v172;
                    v303 = v12;
                    v304 = v172;
                    v305 = v172;
                    v306 = v12;
                    v307 = v12;
                    v308 = v12;
                }
                if v292 != 0.0 {
                    let v3866 = if v301 > v172 { 1.0 } else { 0.0 };
                    out3866 = v3866;
                    if v3866 != 0.0 {
                    } else {
                        let v3872 = -v302;
                        out3872 = v3872;
                    }
                    let v3868 = (v406 * v289) * v289;
                    out3868 = v3868;
                    let v3870 = v289 * (v289 / v291);
                    out3870 = v3870;
                    let v3871 = if v306 > v32 { 1.0 } else { 0.0 };
                    out3871 = v3871;
                    if v3871 != 0.0 {
                        let v3873 = if v456 == v32 { 1.0 } else { 0.0 };
                        out3873 = v3873;
                        if v3873 != 0.0 {
                        } else {
                            let v3876 = v456 - v3875;
                            out3876 = v3876;
                        }
                    } else {
                    }
                    let v3874 = if v307 > v32 { 1.0 } else { 0.0 };
                    out3874 = v3874;
                    if v3874 != 0.0 {
                        let v3877 = if v657 == v32 { 1.0 } else { 0.0 };
                        out3877 = v3877;
                        if v3877 != 0.0 {
                        } else {
                            let v3879 = v657 - v3875;
                            out3879 = v3879;
                        }
                    } else {
                    }
                    let v3878 = if v308 > v32 { 1.0 } else { 0.0 };
                    out3878 = v3878;
                    if v3878 != 0.0 {
                        let v3880 = if v856 == v32 { 1.0 } else { 0.0 };
                        out3880 = v3880;
                        if v3880 != 0.0 {
                        } else {
                            let v3881 = v856 - v3875;
                            out3881 = v3881;
                        }
                    } else {
                    }
                } else {
                    if v3865 != 0.0 {
                        let v3883 = (v406 * v289) * v289;
                        out3883 = v3883;
                        let v3885 = v289 * (v289 / v291);
                        out3885 = v3885;
                        let v3887 = (v406 * v11) * v11;
                        out3887 = v3887;
                    } else {
                    }
                    if v250 != 0.0 {
                    } else {
                        if v3888 != 0.0 {
                        } else {
                            let v3889 = v460 * v132;
                            out3889 = v3889;
                        }
                        if v3890 != 0.0 {
                        } else {
                            let v3891 = -v174;
                            out3891 = v3891;
                        }
                    }
                    if v280 != 0.0 {
                    } else {
                        if v3892 != 0.0 {
                        } else {
                            let v3893 = v460 * v133;
                            out3893 = v3893;
                        }
                        if v3894 != 0.0 {
                        } else {
                            let v3895 = -v176;
                            out3895 = v3895;
                        }
                    }
                    if v283 != 0.0 {
                    } else {
                        if v3896 != 0.0 {
                        } else {
                            let v3897 = v460 * v134;
                            out3897 = v3897;
                        }
                        if v3898 != 0.0 {
                        } else {
                            let v3899 = -v178;
                            out3899 = v3899;
                        }
                        if v0 != 0.0 {
                            let v3901 = (v406 * v289) * v289;
                            out3901 = v3901;
                            let v3903 = v289 * (v289 / v291);
                            out3903 = v3903;
                        } else {
                        }
                    }
                }
            [v11, v13, v35, v41, v47, v50, v53, v56, v81, v88, v95, v96, v97, v98, v116, v119, v122, v123, v124, v125, v132, v133, v134, v141, v148, v155, v173, v175, v177, v217, v227, v235, v242, v246, out248, v287, v291, out310, out319, out336, v249, out501, out513, out542, out575, out577, out700, out711, out737, out770, out772, out899, out910, out936, out969, out971, out1008, out1017, out1034, out1191, out1202, out1228, out1260, out1262, out1379, out1390, out1416, out1448, out1450, out1572, out1583, out1609, out1641, out1643, out1679, out1688, out1705, out1862, out1873, out1899, out1931, out1933, out2050, out2061, out2087, out2119, out2121, out2243, out2254, out2280, out2312, out2314, out2349, out2358, out2375, out2532, out2543, out2569, out2601, out2603, out2720, out2731, out2757, out2789, out2791, out2913, out2924, out2950, out2982, out2984, out3020, out3029, out3046, out3203, out3214, out3240, out3272, out3274, out3391, out3402, out3428, out3460, out3462, out3592, out3603, out3629, out3661, out3663, out3700, out3725, out3762, out3787, out3815, out3713, out3840, out3842, v294, v295, v296, v297, v298, v299, v300, out3866, v302, v303, v304, v305, out3872, out3868, out3870, out3871, out3873, out3874, out3877, out3878, out3880, out3883, out3885, out3887, out3889, out3891, out3893, out3895, out3897, out3899, out3901, out3903, v212, v213, v214, out3876, out3879, out3881]
        };
        self.canonical_staged[140] = produced[0];
        self.canonical_staged[111] = produced[1];
        self.canonical_staged[148] = produced[2];
        self.canonical_staged[163] = produced[3];
        self.canonical_staged[177] = produced[4];
        self.canonical_staged[145] = produced[5];
        self.canonical_staged[160] = produced[6];
        self.canonical_staged[174] = produced[7];
        self.canonical_staged[146] = produced[8];
        self.canonical_staged[161] = produced[9];
        self.canonical_staged[175] = produced[10];
        self.canonical_staged[127] = produced[11];
        self.canonical_staged[130] = produced[12];
        self.canonical_staged[133] = produced[13];
        self.canonical_staged[128] = produced[14];
        self.canonical_staged[131] = produced[15];
        self.canonical_staged[134] = produced[16];
        self.canonical_staged[129] = produced[17];
        self.canonical_staged[132] = produced[18];
        self.canonical_staged[135] = produced[19];
        self.canonical_staged[152] = produced[20];
        self.canonical_staged[167] = produced[21];
        self.canonical_staged[181] = produced[22];
        self.canonical_staged[149] = produced[23];
        self.canonical_staged[164] = produced[24];
        self.canonical_staged[178] = produced[25];
        self.canonical_staged[217] = produced[26];
        self.canonical_staged[221] = produced[27];
        self.canonical_staged[222] = produced[28];
        self.canonical_staged[227] = produced[29];
        self.canonical_staged[228] = produced[30];
        self.canonical_staged[229] = produced[31];
        self.canonical_staged[138] = produced[32];
        self.canonical_staged[230] = produced[33];
        self.canonical_staged[231] = produced[34];
        self.canonical_staged[141] = produced[35];
        self.canonical_staged[125] = produced[36];
        self.canonical_staged[237] = produced[37];
        self.canonical_staged[238] = produced[38];
        self.canonical_staged[240] = produced[39];
        self.canonical_staged[139] = produced[40];
        self.canonical_staged[246] = produced[41];
        self.canonical_staged[247] = produced[42];
        self.canonical_staged[248] = produced[43];
        self.canonical_staged[251] = produced[44];
        self.canonical_staged[252] = produced[45];
        self.canonical_staged[260] = produced[46];
        self.canonical_staged[261] = produced[47];
        self.canonical_staged[262] = produced[48];
        self.canonical_staged[265] = produced[49];
        self.canonical_staged[266] = produced[50];
        self.canonical_staged[274] = produced[51];
        self.canonical_staged[275] = produced[52];
        self.canonical_staged[276] = produced[53];
        self.canonical_staged[279] = produced[54];
        self.canonical_staged[280] = produced[55];
        self.canonical_staged[283] = produced[56];
        self.canonical_staged[284] = produced[57];
        self.canonical_staged[286] = produced[58];
        self.canonical_staged[292] = produced[59];
        self.canonical_staged[293] = produced[60];
        self.canonical_staged[294] = produced[61];
        self.canonical_staged[297] = produced[62];
        self.canonical_staged[298] = produced[63];
        self.canonical_staged[306] = produced[64];
        self.canonical_staged[307] = produced[65];
        self.canonical_staged[308] = produced[66];
        self.canonical_staged[311] = produced[67];
        self.canonical_staged[312] = produced[68];
        self.canonical_staged[320] = produced[69];
        self.canonical_staged[321] = produced[70];
        self.canonical_staged[322] = produced[71];
        self.canonical_staged[325] = produced[72];
        self.canonical_staged[326] = produced[73];
        self.canonical_staged[329] = produced[74];
        self.canonical_staged[330] = produced[75];
        self.canonical_staged[332] = produced[76];
        self.canonical_staged[338] = produced[77];
        self.canonical_staged[339] = produced[78];
        self.canonical_staged[340] = produced[79];
        self.canonical_staged[343] = produced[80];
        self.canonical_staged[344] = produced[81];
        self.canonical_staged[352] = produced[82];
        self.canonical_staged[353] = produced[83];
        self.canonical_staged[354] = produced[84];
        self.canonical_staged[357] = produced[85];
        self.canonical_staged[358] = produced[86];
        self.canonical_staged[366] = produced[87];
        self.canonical_staged[367] = produced[88];
        self.canonical_staged[368] = produced[89];
        self.canonical_staged[371] = produced[90];
        self.canonical_staged[372] = produced[91];
        self.canonical_staged[375] = produced[92];
        self.canonical_staged[376] = produced[93];
        self.canonical_staged[377] = produced[94];
        self.canonical_staged[383] = produced[95];
        self.canonical_staged[384] = produced[96];
        self.canonical_staged[385] = produced[97];
        self.canonical_staged[388] = produced[98];
        self.canonical_staged[389] = produced[99];
        self.canonical_staged[397] = produced[100];
        self.canonical_staged[398] = produced[101];
        self.canonical_staged[399] = produced[102];
        self.canonical_staged[402] = produced[103];
        self.canonical_staged[403] = produced[104];
        self.canonical_staged[411] = produced[105];
        self.canonical_staged[412] = produced[106];
        self.canonical_staged[413] = produced[107];
        self.canonical_staged[416] = produced[108];
        self.canonical_staged[417] = produced[109];
        self.canonical_staged[420] = produced[110];
        self.canonical_staged[421] = produced[111];
        self.canonical_staged[422] = produced[112];
        self.canonical_staged[428] = produced[113];
        self.canonical_staged[429] = produced[114];
        self.canonical_staged[430] = produced[115];
        self.canonical_staged[433] = produced[116];
        self.canonical_staged[434] = produced[117];
        self.canonical_staged[442] = produced[118];
        self.canonical_staged[443] = produced[119];
        self.canonical_staged[444] = produced[120];
        self.canonical_staged[447] = produced[121];
        self.canonical_staged[448] = produced[122];
        self.canonical_staged[456] = produced[123];
        self.canonical_staged[457] = produced[124];
        self.canonical_staged[458] = produced[125];
        self.canonical_staged[461] = produced[126];
        self.canonical_staged[462] = produced[127];
        self.canonical_staged[465] = produced[128];
        self.canonical_staged[467] = produced[129];
        self.canonical_staged[468] = produced[130];
        self.canonical_staged[469] = produced[131];
        self.canonical_staged[470] = produced[132];
        self.canonical_staged[466] = produced[133];
        self.canonical_staged[471] = produced[134];
        self.canonical_staged[472] = produced[135];
        self.canonical_staged[112] = produced[136];
        self.canonical_staged[113] = produced[137];
        self.canonical_staged[114] = produced[138];
        self.canonical_staged[115] = produced[139];
        self.canonical_staged[116] = produced[140];
        self.canonical_staged[117] = produced[141];
        self.canonical_staged[118] = produced[142];
        self.canonical_staged[474] = produced[143];
        self.canonical_staged[120] = produced[144];
        self.canonical_staged[119] = produced[145];
        self.canonical_staged[121] = produced[146];
        self.canonical_staged[122] = produced[147];
        self.canonical_staged[123] = produced[148];
        self.canonical_staged[126] = produced[149];
        self.canonical_staged[124] = produced[150];
        self.canonical_staged[475] = produced[151];
        self.canonical_staged[476] = produced[152];
        self.canonical_staged[477] = produced[153];
        self.canonical_staged[478] = produced[154];
        self.canonical_staged[479] = produced[155];
        self.canonical_staged[480] = produced[156];
        self.canonical_staged[137] = produced[157];
        self.canonical_staged[136] = produced[158];
        self.canonical_staged[142] = produced[159];
        self.canonical_staged[150] = produced[160];
        self.canonical_staged[154] = produced[161];
        self.canonical_staged[165] = produced[162];
        self.canonical_staged[169] = produced[163];
        self.canonical_staged[179] = produced[164];
        self.canonical_staged[183] = produced[165];
        self.canonical_staged[189] = produced[166];
        self.canonical_staged[188] = produced[167];
        self.canonical_staged[190] = produced[168];
        self.canonical_staged[192] = produced[169];
        self.canonical_staged[193] = produced[170];
        self.canonical_staged[210] = produced[171];
        self.canonical_staged[211] = produced[172];
        self.canonical_staged[212] = produced[173];
        self.canonical_temperature = temperature;
        self.canonical_thermal_voltage = thermal_voltage;
        self.canonical_temperature_valid = true;
    }

    fn canonical_timestep_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        let produced: [f64; 1] = {
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
            [0.0]
        };
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        self.canonical_instance_stage(ctx);
        self.canonical_temperature_stage(ctx);
        self.canonical_timestep_stage(ctx);
        let parameters = &self.params.values;
        let multiplicity = self.multiplicity;
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = 0usize;
            rspice_eval_ddt(
                &mut ddt_state.ddt_current,
                &mut ddt_state.ddt_previous,
                &mut ddt_state.ddt_older,
                &mut ddt_state.ddt_initialized,
                &mut ddt_state.ddt_derivative_current,
                &mut ddt_state.ddt_derivative_previous,
                ddt_active,
                ddt_coefficients.derivative_scale,
                ddt_coefficients.previous_value_scale,
                ddt_coefficients.older_value_scale,
                ddt_coefficients.previous_derivative_scale,
                slot,
                value,
            )
        };
            let v0 = staged[215];
            let v1 = staged[232];
            let v2 = staged[233];
            let v3 = staged[234];
            let v4 = staged[235];
            let v5 = node_potentials[0];
            let v6 = node_potentials[1];
            let v8 = Lanes([1e0f64; 1]);
            let v10 = Lanes([1e0f64; 1]);
            let v13 = parameters[1];
            let v16 = staged[111];
            let v19 = -2.3025850929940458e2f64;
            let v21 = staged[473];
            let v30 = staged[16];
            let v33 = staged[17];
            let v38 = staged[18];
            let v43 = staged[194];
            let v46 = staged[195];
            let v50 = ddt_scale();
            let v52 = -2.3025850929940458e2f64;
            let v54 = -1e0f64;
            let v56 = 1e0f64;
            let v58 = 1e-100f64;
            let v63 = staged[112];
            let v68 = staged[114];
            let v71 = staged[115];
            let v74 = -2.3025850929940458e2f64;
            let v78 = staged[113];
            let v85 = -2.3025850929940458e2f64;
            let v93 = staged[116];
            let v98 = staged[118];
            let v101 = staged[474];
            let v104 = staged[117];
            let v111 = staged[119];
            let v114 = staged[120];
            let v126 = -2.3025850929940458e2f64;
            let v134 = staged[124];
            let v136 = staged[125];
            let v143 = staged[126];
            let v146 = 2e0f64;
            let v148 = 1e0f64;
            let v159 = 2e0f64;
            let v162 = staged[475];
            let v163 = -2.3025850929940458e2f64;
            let v171 = staged[121];
            let v176 = staged[123];
            let v181 = staged[122];
            let v188 = staged[476];
            let v189 = 0e0f64;
            let v190 = Lanes([0e0f64; 2]);
            let v193 = staged[477];
            let v194 = staged[127];
            let v207 = staged[25];
            let v209 = staged[210];
            let v217 = staged[128];
            let v222 = staged[129];
            let v227 = staged[478];
            let v230 = staged[479];
            let v231 = staged[130];
            let v244 = staged[33];
            let v246 = staged[211];
            let v254 = staged[131];
            let v259 = staged[132];
            let v264 = staged[480];
            let v267 = staged[133];
            let v280 = staged[41];
            let v282 = staged[212];
            let v290 = staged[134];
            let v295 = staged[135];
            let v300 = staged[136];
            let v308 = staged[137];
            let v324 = staged[138];
            let v342 = -5e-1f64;
            let v345 = 2.3025850929940458e2f64;
            let v351 = staged[139];
            let v370 = -5e-1f64;
            let v375 = -5e-1f64;
            let v387 = -5e-1f64;
            let v390 = -2.3025850929940458e2f64;
            let v393 = -5e-1f64;
            let v396 = -2.3025850929940458e2f64;
            let v399 = -5e-1f64;
            let v402 = -2.3025850929940458e2f64;
            let v405 = 3.333333333333333e-1f64;
            let v413 = 5e-1f64;
            let v426 = -5e-1f64;
            let v430 = -5e-1f64;
            let v434 = -5e-1f64;
            let v453 = 1e100f64;
            let v460 = 3e0f64;
            let v475 = staged[140];
            let v509 = staged[141];
            let v519 = staged[142];
            let v529 = staged[143];
            let v535 = staged[144];
            let v548 = 4e-12f64;
            let v558 = staged[145];
            let v561 = staged[481];
            let v574 = staged[146];
            let v589 = staged[482];
            let v600 = staged[483];
            let v619 = staged[147];
            let v626 = staged[23];
            let v635 = parameters[21];
            let v637 = staged[196];
            let v643 = staged[24];
            let v651 = staged[148];
            let v658 = parameters[30];
            let v667 = staged[149];
            let v670 = staged[150];
            let v698 = staged[484];
            let v701 = staged[485];
            let v716 = staged[151];
            let v718 = staged[197];
            let v738 = 3.75e-1f64;
            let v753 = staged[152];
            let v781 = 5.178164370971076e-1f64;
            let v803 = -2.3025850929940458e2f64;
            let v807 = -2.3025850929940458e2f64;
            let v810 = -2.3025850929940458e2f64;
            let v812 = -2.3025850929940458e2f64;
            let v835 = 2.9214664e-1f64;
            let v841 = 2.6992878119627894e-1f64;
            let v850 = 4.3792457880372104e-1f64;
            let v859 = -2.3025850929940458e2f64;
            let v869 = 8.86226925452758e-1f64;
            let v880 = parameters[35];
            let v885 = -2.3025850929940458e2f64;
            let v888 = -2.3025850929940458e2f64;
            let v890 = -2.3025850929940458e2f64;
            let v917 = staged[486];
            let v920 = staged[487];
            let v921 = parameters[18];
            let v935 = staged[198];
            let v943 = staged[153];
            let v950 = staged[4];
            let v953 = staged[154];
            let v977 = parameters[41];
            let v980 = -2.3025850929940458e2f64;
            let v983 = -2.3025850929940458e2f64;
            let v985 = -2.3025850929940458e2f64;
            let v1026 = staged[155];
            let v1036 = parameters[10];
            let v1043 = staged[488];
            let v1044 = parameters[53];
            let v1045 = 4e0f64;
            let v1047 = staged[157];
            let v1048 = parameters[50];
            let v1051 = staged[158];
            let v1054 = staged[159];
            let v1058 = staged[156];
            let v1075 = 0e0f64;
            let v1106 = staged[199];
            let v1122 = parameters[11];
            let v1125 = staged[160];
            let v1128 = staged[489];
            let v1141 = staged[161];
            let v1156 = staged[490];
            let v1167 = staged[491];
            let v1186 = staged[162];
            let v1193 = staged[31];
            let v1202 = parameters[22];
            let v1204 = staged[200];
            let v1210 = staged[32];
            let v1218 = staged[163];
            let v1225 = parameters[31];
            let v1234 = staged[164];
            let v1237 = staged[165];
            let v1265 = staged[492];
            let v1268 = staged[493];
            let v1283 = staged[166];
            let v1285 = staged[201];
            let v1319 = staged[167];
            let v1368 = -2.3025850929940458e2f64;
            let v1372 = -2.3025850929940458e2f64;
            let v1375 = -2.3025850929940458e2f64;
            let v1377 = -2.3025850929940458e2f64;
            let v1421 = -2.3025850929940458e2f64;
            let v1431 = 8.86226925452758e-1f64;
            let v1442 = parameters[36];
            let v1447 = -2.3025850929940458e2f64;
            let v1450 = -2.3025850929940458e2f64;
            let v1452 = -2.3025850929940458e2f64;
            let v1479 = staged[494];
            let v1482 = staged[495];
            let v1483 = parameters[19];
            let v1497 = staged[202];
            let v1505 = staged[168];
            let v1512 = staged[5];
            let v1515 = staged[169];
            let v1539 = parameters[42];
            let v1542 = -2.3025850929940458e2f64;
            let v1545 = -2.3025850929940458e2f64;
            let v1547 = -2.3025850929940458e2f64;
            let v1588 = staged[170];
            let v1604 = staged[496];
            let v1605 = parameters[54];
            let v1607 = parameters[51];
            let v1610 = staged[172];
            let v1613 = staged[173];
            let v1617 = staged[171];
            let v1664 = staged[203];
            let v1682 = staged[174];
            let v1685 = staged[497];
            let v1700 = staged[175];
            let v1715 = staged[498];
            let v1726 = staged[499];
            let v1745 = staged[176];
            let v1752 = staged[39];
            let v1761 = parameters[23];
            let v1763 = staged[204];
            let v1769 = staged[40];
            let v1777 = staged[177];
            let v1784 = parameters[32];
            let v1793 = staged[178];
            let v1796 = staged[179];
            let v1824 = staged[500];
            let v1827 = staged[501];
            let v1842 = staged[180];
            let v1844 = staged[205];
            let v1878 = staged[181];
            let v1927 = -2.3025850929940458e2f64;
            let v1931 = -2.3025850929940458e2f64;
            let v1934 = -2.3025850929940458e2f64;
            let v1936 = -2.3025850929940458e2f64;
            let v1980 = -2.3025850929940458e2f64;
            let v1990 = 8.86226925452758e-1f64;
            let v2001 = parameters[37];
            let v2006 = -2.3025850929940458e2f64;
            let v2009 = -2.3025850929940458e2f64;
            let v2011 = -2.3025850929940458e2f64;
            let v2038 = staged[502];
            let v2041 = staged[503];
            let v2042 = parameters[20];
            let v2056 = staged[206];
            let v2064 = staged[182];
            let v2071 = staged[6];
            let v2074 = staged[183];
            let v2098 = parameters[43];
            let v2101 = -2.3025850929940458e2f64;
            let v2104 = -2.3025850929940458e2f64;
            let v2106 = -2.3025850929940458e2f64;
            let v2147 = staged[184];
            let v2163 = parameters[55];
            let v2165 = parameters[52];
            let v2168 = staged[186];
            let v2171 = staged[187];
            let v2175 = staged[185];
            let v2209 = parameters[60];
            let v2211 = staged[504];
            let v2215 = parameters[61];
            let v2218 = -3.7e1f64;
            let v2222 = 3.7e1f64;
            let v2226 = staged[188];
            let v2234 = staged[189];
            let v2250 = staged[505];
            let v2291 = staged[208];
            let v2334 = staged[506];
            let v2335 = staged[190];
            let v2348 = staged[191];
            let v2350 = staged[209];
            let v2358 = staged[192];
            let v2363 = staged[193];
            let v2385 = staged[207];
            let v2409 = 0e0f64;
            let v14 = v13 * (v5 - v6);
            let v15 = ((Lanes([v8[0], 0.0])) - (Lanes([0.0, v10[0]]))) * v13;
            let v22: f64;
            let v23: f64;
            let v24: f64;
            let v25: f64;
            let v26: Lanes<2>;
            let v27: Lanes<2>;
            let v28: Lanes<2>;
            let v29: Lanes<2>;
            if v4 != 0.0 {
                let v17 = v14 * v16;
                let v18 = v15 * v16;
                let v20 = if v17 < v19 { 1.0 } else { 0.0 };
                let v65: f64;
                let v66: Lanes<2>;
                if v20 != 0.0 {
                    let v57 = (v52 - v17) + v56;
                    let v59 = v58 / v57;
                    let v62 = (((v18 * v54) * v59) * v54) / v57;
                    v65 = v59;
                    v66 = v62;
                } else {
                    let v64 = if v17 > v63 { 1.0 } else { 0.0 };
                    let v83: f64;
                    let v84: Lanes<2>;
                    if v64 != 0.0 {
                        let v79 = v78 * ((v17 - v63) + v56);
                        let v80 = v18 * v78;
                        v83 = v79;
                        v84 = v80;
                    } else {
                        let v81 = v17.exp();
                        let v82 = v18 * v81;
                        v83 = v81;
                        v84 = v82;
                    }
                    v65 = v83;
                    v66 = v84;
                }
                let v69 = v68 * (v65 - v56);
                let v70 = v66 * v68;
                let v72 = v17 * v71;
                let v73 = v18 * v71;
                let v75 = if v72 < v74 { 1.0 } else { 0.0 };
                let v95: f64;
                let v96: Lanes<2>;
                if v75 != 0.0 {
                    let v88 = (v85 - v72) + v56;
                    let v89 = v58 / v88;
                    let v92 = (((v73 * v54) * v89) * v54) / v88;
                    v95 = v89;
                    v96 = v92;
                } else {
                    let v94 = if v72 > v93 { 1.0 } else { 0.0 };
                    let v109: f64;
                    let v110: Lanes<2>;
                    if v94 != 0.0 {
                        let v105 = v104 * ((v72 - v93) + v56);
                        let v106 = v73 * v104;
                        v109 = v105;
                        v110 = v106;
                    } else {
                        let v107 = v72.exp();
                        let v108 = v73 * v107;
                        v109 = v107;
                        v110 = v108;
                    }
                    v95 = v109;
                    v96 = v110;
                }
                let v99 = v98 * (v95 - v56);
                let v100 = v96 * v98;
                let v128: f64;
                let v129: Lanes<2>;
                if v101 != 0.0 {
                    let v115 = v114 + (v14 * v111);
                    let v116 = v14 * v115;
                    let v119 = (v15 * v115) + ((v15 * v111) * v14);
                    v128 = v116;
                    v129 = v119;
                } else {
                    let v124 = ((-v14) * v16) * v111;
                    let v125 = ((v15 * v54) * v16) * v111;
                    let v127 = if v124 < v126 { 1.0 } else { 0.0 };
                    let v173: f64;
                    let v174: Lanes<2>;
                    if v127 != 0.0 {
                        let v166 = (v163 - v124) + v56;
                        let v167 = v58 / v166;
                        let v170 = (((v125 * v54) * v167) * v54) / v166;
                        v173 = v167;
                        v174 = v170;
                    } else {
                        let v172 = if v124 > v171 { 1.0 } else { 0.0 };
                        let v186: f64;
                        let v187: Lanes<2>;
                        if v172 != 0.0 {
                            let v182 = v181 * ((v124 - v171) + v56);
                            let v183 = v125 * v181;
                            v186 = v182;
                            v187 = v183;
                        } else {
                            let v184 = v124.exp();
                            let v185 = v125 * v184;
                            v186 = v184;
                            v187 = v185;
                        }
                        v173 = v186;
                        v174 = v187;
                    }
                    let v177 = v176 * (v173 - v56);
                    let v178 = v174 * v176;
                    v128 = v177;
                    v129 = v178;
                }
                let v132 = (v69 + v99) + v128;
                let v133 = (v70 + v100) + v129;
                let v135 = v14 + v134;
                let v138 = v136 - v135;
                let v141 = (v15 * v54) * v138;
                let v145 = ((v138 * v138) + v143).sqrt();
                let v153 = (v136 + v135) + v145;
                let v155 = (v14 * v136) / v153;
                let v160 = v159 * v155;
                let v161 = (((v15 * v136) - ((v15 + ((v141 + v141) * (v148 / (v146 * v145)))) * v155)) / v153) * v159;
                let v191: f64;
                let v192: Lanes<2>;
                if v162 != 0.0 {
                    let v213: f64;
                    let v214: Lanes<2>;
                    if v188 != 0.0 {
                        let v199 = (v56 - (v160 * v194)).sqrt();
                        let v202 = ((v161 * v194) * v54) * (v148 / (v146 * v199));
                        v213 = v199;
                        v214 = v202;
                    } else {
                        let v205 = v56 - (v160 * v194);
                        let v208 = v205.powf(v207);
                        let v212 = ((v161 * v194) * v54) * (v207 * (v205.powf(v209)));
                        v213 = v208;
                        v214 = v212;
                    }
                    let v225 = (v217 * (v56 - v213)) + (v222 * (v14 - v160));
                    let v226 = ((v214 * v54) * v217) + ((v15 - v161) * v222);
                    v191 = v225;
                    v192 = v226;
                } else {
                    v191 = v189;
                    v192 = v190;
                }
                let v228: f64;
                let v229: Lanes<2>;
                if v193 != 0.0 {
                    let v250: f64;
                    let v251: Lanes<2>;
                    if v227 != 0.0 {
                        let v236 = (v56 - (v160 * v231)).sqrt();
                        let v239 = ((v161 * v231) * v54) * (v148 / (v146 * v236));
                        v250 = v236;
                        v251 = v239;
                    } else {
                        let v242 = v56 - (v160 * v231);
                        let v245 = v242.powf(v244);
                        let v249 = ((v161 * v231) * v54) * (v244 * (v242.powf(v246)));
                        v250 = v245;
                        v251 = v249;
                    }
                    let v262 = (v254 * (v56 - v250)) + (v259 * (v14 - v160));
                    let v263 = ((v251 * v54) * v254) + ((v15 - v161) * v259);
                    v228 = v262;
                    v229 = v263;
                } else {
                    v228 = v189;
                    v229 = v190;
                }
                let v265: f64;
                let v266: Lanes<2>;
                if v230 != 0.0 {
                    let v286: f64;
                    let v287: Lanes<2>;
                    if v264 != 0.0 {
                        let v272 = (v56 - (v160 * v267)).sqrt();
                        let v275 = ((v161 * v267) * v54) * (v148 / (v146 * v272));
                        v286 = v272;
                        v287 = v275;
                    } else {
                        let v278 = v56 - (v160 * v267);
                        let v281 = v278.powf(v280);
                        let v285 = ((v161 * v267) * v54) * (v280 * (v278.powf(v282)));
                        v286 = v281;
                        v287 = v285;
                    }
                    let v298 = (v290 * (v56 - v286)) + (v295 * (v14 - v160));
                    let v299 = ((v287 * v54) * v290) + ((v15 - v161) * v295);
                    v265 = v298;
                    v266 = v299;
                } else {
                    v265 = v189;
                    v266 = v190;
                }
                v22 = v191;
                v23 = v228;
                v24 = v265;
                v25 = v132;
                v26 = v192;
                v27 = v229;
                v28 = v266;
                v29 = v133;
            } else {
                let v326: f64;
                let v327: f64;
                let v328: f64;
                let v329: f64;
                let v330: f64;
                let v331: f64;
                let v332: f64;
                let v333: Lanes<2>;
                let v334: Lanes<2>;
                let v335: Lanes<2>;
                let v336: Lanes<2>;
                let v337: Lanes<2>;
                let v338: Lanes<2>;
                let v339: Lanes<2>;
                if v21 != 0.0 {
                    let v301 = v14 + v300;
                    let v303 = v136 - v301;
                    let v304 = v15 * v54;
                    let v306 = v304 * v303;
                    let v310 = ((v303 * v303) + v308).sqrt();
                    let v316 = (v136 + v301) + v310;
                    let v318 = (v14 * v136) / v316;
                    let v322 = v159 * v318;
                    let v323 = (((v15 * v136) - ((v15 + ((v306 + v306) * (v148 / (v146 * v310)))) * v318)) / v316) * v159;
                    let v325 = if v14 < v324 { 1.0 } else { 0.0 };
                    let v362: f64;
                    let v363: f64;
                    let v364: f64;
                    let v365: Lanes<2>;
                    let v366: Lanes<2>;
                    let v367: Lanes<2>;
                    if v325 != 0.0 {
                        let v340 = v14 * v16;
                        let v341 = v15 * v16;
                        let v346 = if ((v342 * v340).abs()) < v345 { 1.0 } else { 0.0 };
                        let v378: f64;
                        let v379: Lanes<2>;
                        if v346 != 0.0 {
                            let v373 = (v370 * v340).exp();
                            let v374 = (v341 * v370) * v373;
                            v378 = v373;
                            v379 = v374;
                        } else {
                            let v377 = if (v375 * v340) < v189 { 1.0 } else { 0.0 };
                            let v456: f64;
                            let v457: Lanes<2>;
                            if v377 != 0.0 {
                                let v391 = v390 - (v387 * v340);
                                let v397 = v396 - (v393 * v340);
                                let v408 = v56 + ((v402 - (v399 * v340)) * v405);
                                let v416 = v56 + (v413 * (v397 * v408));
                                let v421 = v56 + (v391 * v416);
                                let v422 = v58 / v421;
                                let v425 = ((((((v341 * v387) * v54) * v416) + ((((((v341 * v393) * v54) * v408) + ((((v341 * v399) * v54) * v405) * v397)) * v413) * v391)) * v422) * v54) / v421;
                                v456 = v422;
                                v457 = v425;
                            } else {
                                let v429 = (v426 * v340) - v345;
                                let v433 = (v430 * v340) - v345;
                                let v440 = v56 + (((v434 * v340) - v345) * v405);
                                let v447 = v56 + (v413 * (v433 * v440));
                                let v454 = v453 * (v56 + (v429 * v447));
                                let v455 = (((v341 * v426) * v447) + (((((v341 * v430) * v440) + (((v341 * v434) * v405) * v433)) * v413) * v429)) * v453;
                                v456 = v454;
                                v457 = v455;
                            }
                            v378 = v456;
                            v379 = v457;
                        }
                        let v380 = v56 / v378;
                        let v383 = ((v379 * v380) * v54) / v378;
                        let v384 = v380 * v380;
                        let v385 = v383 * v380;
                        let v386 = v385 + v385;
                        v362 = v384;
                        v363 = v378;
                        v364 = v380;
                        v365 = v386;
                        v366 = v379;
                        v367 = v383;
                    } else {
                        let v352 = (v56 + ((v14 - v324) * v16)) * v351;
                        let v353 = (v15 * v16) * v351;
                        let v354 = v352.sqrt();
                        let v357 = v353 * (v148 / (v146 * v354));
                        let v358 = v56 / v354;
                        let v361 = ((v357 * v358) * v54) / v354;
                        v362 = v352;
                        v363 = v358;
                        v364 = v354;
                        v365 = v353;
                        v366 = v361;
                        v367 = v357;
                    }
                    let v368 = v362 - v56;
                    let v369 = if v14 > v189 { 1.0 } else { 0.0 };
                    let v507: f64;
                    let v508: Lanes<2>;
                    if v369 != 0.0 {
                        let v459 = v363 + v56;
                        let v461 = v363 + v460;
                        let v466 = (v459 * v461).sqrt();
                        let v470 = (v159 + v363) + v466;
                        let v478 = v159 * (v475 * (v470.ln()));
                        let v479 = (((v366 + (((v366 * v461) + (v366 * v459)) * (v148 / (v146 * v466)))) * (v148 / v470)) * v475) * v159;
                        v507 = v478;
                        v508 = v479;
                    } else {
                        let v484 = v56 + v364;
                        let v487 = v56 + (v460 * v364);
                        let v492 = (v484 * v487).sqrt();
                        let v496 = ((v159 * v364) + v56) + v492;
                        let v505 = (-v14) + (v159 * (v475 * (v496.ln())));
                        let v506 = v304 + (((((v367 * v159) + (((v367 * v487) + ((v367 * v460) * v484)) * (v148 / (v146 * v492)))) * (v148 / v496)) * v475) * v159);
                        v507 = v505;
                        v508 = v506;
                    }
                    let v510 = v509 - v507;
                    let v511 = v508 * v54;
                    let v514 = v14 - v510;
                    let v517 = (v15 - v511) * v514;
                    let v521 = ((v514 * v514) + v519).sqrt();
                    let v527 = v413 * ((v14 + v510) - v521);
                    let v528 = ((v15 + v511) - ((v517 + v517) * (v148 / (v146 * v521)))) * v413;
                    let v531 = v14 - v529;
                    let v533 = v15 * v531;
                    let v537 = ((v531 * v531) + v535).sqrt();
                    let v543 = v413 * ((v14 + v529) - v537);
                    let v544 = (v15 - ((v533 + v533) * (v148 / (v146 * v537)))) * v413;
                    let v546 = v15 * v14;
                    let v550 = ((v14 * v14) + v548).sqrt();
                    let v556 = v413 * (v14 - v550);
                    let v557 = (v15 - ((v546 + v546) * (v148 / (v146 * v550)))) * v413;
                    v326 = v368;
                    v327 = v527;
                    v328 = v507;
                    v329 = v364;
                    v330 = v543;
                    v331 = v556;
                    v332 = v322;
                    v333 = v365;
                    v334 = v528;
                    v335 = v508;
                    v336 = v367;
                    v337 = v544;
                    v338 = v557;
                    v339 = v323;
                } else {
                    v326 = v189;
                    v327 = v189;
                    v328 = v189;
                    v329 = v189;
                    v330 = v189;
                    v331 = v189;
                    v332 = v189;
                    v333 = v190;
                    v334 = v190;
                    v335 = v190;
                    v336 = v190;
                    v337 = v190;
                    v338 = v190;
                    v339 = v190;
                }
                let v562: f64;
                let v563: f64;
                let v564: f64;
                let v565: f64;
                let v566: f64;
                let v567: f64;
                let v568: Lanes<2>;
                let v569: Lanes<2>;
                let v570: Lanes<2>;
                let v571: Lanes<2>;
                let v572: Lanes<2>;
                let v573: Lanes<2>;
                if v1 != 0.0 {
                    v562 = v189;
                    v563 = v189;
                    v564 = v189;
                    v565 = v189;
                    v566 = v189;
                    v567 = v189;
                    v568 = v190;
                    v569 = v190;
                    v570 = v190;
                    v571 = v190;
                    v572 = v190;
                    v573 = v190;
                } else {
                    let v559 = v558 * v326;
                    let v560 = v333 * v558;
                    let v590: f64;
                    let v591: f64;
                    let v592: f64;
                    let v593: f64;
                    let v594: f64;
                    let v595: Lanes<2>;
                    let v596: Lanes<2>;
                    let v597: Lanes<2>;
                    let v598: Lanes<2>;
                    let v599: Lanes<2>;
                    if v561 != 0.0 {
                        v590 = v189;
                        v591 = v189;
                        v592 = v189;
                        v593 = v189;
                        v594 = v189;
                        v595 = v190;
                        v596 = v190;
                        v597 = v190;
                        v598 = v190;
                        v599 = v190;
                    } else {
                        let v575 = v574 - v327;
                        let v576 = v334 * v54;
                        let v577 = v328 / v575;
                        let v583 = (v56 - v577).sqrt();
                        let v587 = v56 - v583;
                        let v588 = ((((v335 - (v576 * v577)) / v575) * v54) * (v148 / (v146 * v583))) * v54;
                        let v622: f64;
                        let v623: Lanes<2>;
                        if v589 != 0.0 {
                            v622 = v189;
                            v623 = v190;
                        } else {
                            let v601 = v587 * v587;
                            let v602 = v588 * v587;
                            let v604 = v587.ln();
                            let v611 = v56 - v587;
                            let v613 = (v601 * v604) / v611;
                            let v620 = (v613 + v587) * v619;
                            let v621 = ((((((v602 + v602) * v604) + ((v588 * (v148 / v587)) * v601)) - ((v588 * v54) * v613)) / v611) + v588) * v619;
                            v622 = v620;
                            v623 = v621;
                        }
                        let v624 = v587 + v622;
                        let v625 = v588 + v623;
                        let v641: f64;
                        let v642: Lanes<2>;
                        if v589 != 0.0 {
                            let v629 = (v575 * v626).sqrt();
                            let v632 = (v576 * v626) * (v148 / (v146 * v629));
                            v641 = v629;
                            v642 = v632;
                        } else {
                            let v633 = v575 * v626;
                            let v636 = v633.powf(v635);
                            let v640 = (v576 * v626) * (v635 * (v633.powf(v637)));
                            v641 = v636;
                            v642 = v640;
                        }
                        let v644 = v643 * v641;
                        let v645 = v642 * v643;
                        let v646 = v329 - v56;
                        let v652 = v651 * (v646 * v644);
                        let v653 = ((v336 * v644) + (v645 * v646)) * v651;
                        let v659 = v658 * (v652 * v624);
                        let v660 = ((v653 * v624) + (v625 * v652)) * v658;
                        v590 = v644;
                        v591 = v575;
                        v592 = v624;
                        v593 = v652;
                        v594 = v659;
                        v595 = v645;
                        v596 = v576;
                        v597 = v625;
                        v598 = v653;
                        v599 = v660;
                    }
                    let v699: f64;
                    let v700: Lanes<2>;
                    if v600 != 0.0 {
                        v699 = v189;
                        v700 = v190;
                    } else {
                        let v663 = (v590 * v207) / v591;
                        let v668 = v667 * v663;
                        let v669 = (((v595 * v207) - (v596 * v663)) / v591) * v667;
                        let v671 = v670 / v668;
                        let v674 = ((v669 * v671) * v54) / v668;
                        let v675 = v671 * v671;
                        let v676 = v674 * v671;
                        let v678 = v675 * v675;
                        let v679 = (v676 + v676) * v675;
                        let v680 = v679 + v679;
                        let v681 = v678 + v56;
                        let v682 = v678 / v681;
                        let v686 = v682.sqrt();
                        let v689 = ((v680 - (v680 * v682)) / v681) * (v148 / (v146 * v686));
                        let v690 = v686.sqrt();
                        let v693 = v689 * (v148 / (v146 * v690));
                        let v694 = v686 * v690;
                        let v697 = (v689 * v690) + (v693 * v686);
                        let v722: f64;
                        let v723: Lanes<2>;
                        if v698 != 0.0 {
                            let v706 = v56 + (v668 * v694);
                            let v707 = v56 / v706;
                            let v710 = ((((v669 * v694) + (v697 * v668)) * v707) * v54) / v706;
                            v722 = v707;
                            v723 = v710;
                        } else {
                            let v715 = v56 + (v668 * v694);
                            let v717 = v715.powf(v716);
                            let v721 = ((v669 * v694) + (v697 * v668)) * (v716 * (v715.powf(v718)));
                            v722 = v717;
                            v723 = v721;
                        }
                        let v728 = v592 + v722;
                        let v730 = (v592 * v722) / v728;
                        let v733 = (((v597 * v722) + (v723 * v592)) - ((v597 + v723) * v730)) / v728;
                        let v734 = v668 / v690;
                        let v741 = (v738 * v734).sqrt();
                        let v744 = (((v669 - (v693 * v734)) / v690) * v738) * (v148 / (v146 * v741));
                        let v754 = v753 * v671;
                        let v770 = ((v754 * v690) - (v753 * v686)) + (v413 * (v668 * v694));
                        let v771 = ((((v674 * v753) * v690) + (v693 * v754)) - (v689 * v753)) + (((v669 * v694) + (v697 * v668)) * v413);
                        let v772 = ((v159 * (v671 * v690)) - v686) - v56;
                        let v773 = v772 * v741;
                        let v776 = (((((v674 * v690) + (v693 * v671)) * v159) - v689) * v741) + (v744 * v772);
                        let v777 = v773 * v773;
                        let v778 = v776 * v773;
                        let v779 = v778 + v778;
                        let v780 = if v773 > v189 { 1.0 } else { 0.0 };
                        let v797: f64;
                        let v798: Lanes<2>;
                        if v780 != 0.0 {
                            let v784 = v56 + (v781 * v773);
                            let v785 = v56 / v784;
                            let v788 = (((v776 * v781) * v785) * v54) / v784;
                            v797 = v785;
                            v798 = v788;
                        } else {
                            let v791 = v56 - (v781 * v773);
                            let v793 = v56 / v791;
                            let v796 = ((((v776 * v781) * v54) * v793) * v54) / v791;
                            v797 = v793;
                            v798 = v796;
                        }
                        let v801 = (-v777) + v770;
                        let v802 = (v779 * v54) + v771;
                        let v804 = if v801 > v803 { 1.0 } else { 0.0 };
                        let v833: f64;
                        let v834: Lanes<2>;
                        if v804 != 0.0 {
                            let v805 = v801.exp();
                            let v806 = v802 * v805;
                            v833 = v805;
                            v834 = v806;
                        } else {
                            let v808 = v807 - v801;
                            let v809 = v802 * v54;
                            let v811 = v810 - v801;
                            let v816 = v56 + ((v812 - v801) * v405);
                            let v823 = v56 + (v413 * (v811 * v816));
                            let v828 = v56 + (v808 * v823);
                            let v829 = v58 / v828;
                            let v832 = ((((v809 * v823) + ((((v809 * v816) + ((v809 * v405) * v811)) * v413) * v808)) * v829) * v54) / v828;
                            v833 = v829;
                            v834 = v832;
                        }
                        let v838 = v797 * v797;
                        let v839 = v798 * v797;
                        let v840 = v839 + v839;
                        let v853 = ((v835 * v797) + (v841 * v838)) + (v850 * (v838 * v797));
                        let v855 = v853 * v833;
                        let v858 = ((((v798 * v835) + (v840 * v841)) + (((v840 * v797) + (v798 * v838)) * v850)) * v833) + (v834 * v853);
                        let v861: f64;
                        let v862: Lanes<2>;
                        if v780 != 0.0 {
                            v861 = v855;
                            v862 = v858;
                        } else {
                            let v860 = if v770 > v859 { 1.0 } else { 0.0 };
                            let v911: f64;
                            let v912: Lanes<2>;
                            if v860 != 0.0 {
                                let v883 = v770.exp();
                                let v884 = v771 * v883;
                                v911 = v883;
                                v912 = v884;
                            } else {
                                let v886 = v885 - v770;
                                let v887 = v771 * v54;
                                let v889 = v888 - v770;
                                let v894 = v56 + ((v890 - v770) * v405);
                                let v901 = v56 + (v413 * (v889 * v894));
                                let v906 = v56 + (v886 * v901);
                                let v907 = v58 / v906;
                                let v910 = ((((v887 * v901) + ((((v887 * v894) + ((v887 * v405) * v889)) * v413) * v886)) * v907) * v54) / v906;
                                v911 = v907;
                                v912 = v910;
                            }
                            let v915 = (v159 * v911) - v855;
                            let v916 = (v912 * v159) - v858;
                            v861 = v915;
                            v862 = v916;
                        }
                        let v865 = (v753 * v861) / v741;
                        let v870 = v869 * v865;
                        let v872 = v593 * v870;
                        let v881 = v880 * (v872 * v730);
                        let v882 = ((((v598 * v870) + (((((v862 * v753) - (v744 * v865)) / v741) * v869) * v593)) * v730) + (v733 * v872)) * v880;
                        v699 = v881;
                        v700 = v882;
                    }
                    let v918: f64;
                    let v919: Lanes<2>;
                    if v701 != 0.0 {
                        v918 = v189;
                        v919 = v190;
                    } else {
                        let v939: f64;
                        let v940: Lanes<2>;
                        if v917 != 0.0 {
                            let v926 = ((v921 - v330) * v626).sqrt();
                            let v929 = ((v337 * v54) * v626) * (v148 / (v146 * v926));
                            v939 = v926;
                            v940 = v929;
                        } else {
                            let v932 = (v921 - v330) * v626;
                            let v934 = v932.powf(v635);
                            let v938 = ((v337 * v54) * v626) * (v635 * (v932.powf(v935)));
                            v939 = v934;
                            v940 = v938;
                        }
                        let v946 = ((v921 - v330) * v943) / v939;
                        let v951 = v950 * v946;
                        let v952 = ((((v337 * v54) * v943) - (v940 * v946)) / v939) * v950;
                        let v954 = v953 / v951;
                        let v957 = ((v952 * v954) * v54) / v951;
                        let v959 = if (v954.abs()) < v345 { 1.0 } else { 0.0 };
                        let v963: f64;
                        let v964: Lanes<2>;
                        if v959 != 0.0 {
                            let v960 = v954.exp();
                            let v961 = v957 * v960;
                            v963 = v960;
                            v964 = v961;
                        } else {
                            let v962 = if v954 < v189 { 1.0 } else { 0.0 };
                            let v1024: f64;
                            let v1025: Lanes<2>;
                            if v962 != 0.0 {
                                let v981 = v980 - v954;
                                let v982 = v957 * v54;
                                let v984 = v983 - v954;
                                let v989 = v56 + ((v985 - v954) * v405);
                                let v996 = v56 + (v413 * (v984 * v989));
                                let v1001 = v56 + (v981 * v996);
                                let v1002 = v58 / v1001;
                                let v1005 = ((((v982 * v996) + ((((v982 * v989) + ((v982 * v405) * v984)) * v413) * v981)) * v1002) * v54) / v1001;
                                v1024 = v1002;
                                v1025 = v1005;
                            } else {
                                let v1006 = v954 - v345;
                                let v1009 = v56 + (v1006 * v405);
                                let v1016 = v56 + (v413 * (v1006 * v1009));
                                let v1022 = v453 * (v56 + (v1006 * v1016));
                                let v1023 = ((v957 * v1016) + ((((v957 * v1009) + ((v957 * v405) * v1006)) * v413) * v1006)) * v453;
                                v1024 = v1022;
                                v1025 = v1023;
                            }
                            v963 = v1024;
                            v964 = v1025;
                        }
                        let v965 = v14 * v951;
                        let v969 = v965 * v951;
                        let v978 = v977 * (v969 * v963);
                        let v979 = ((((((v15 * v951) + (v952 * v14)) * v951) + (v952 * v965)) * v963) + (v964 * v969)) * v977;
                        v918 = v978;
                        v919 = v979;
                    }
                    let v1028: f64;
                    let v1029: Lanes<2>;
                    if v920 != 0.0 {
                        v1028 = v56;
                        v1029 = v190;
                    } else {
                        let v1027 = if v331 > v1026 { 1.0 } else { 0.0 };
                        let v1056: f64;
                        let v1057: Lanes<2>;
                        if v1027 != 0.0 {
                            let v1046 = if v1044 == v1045 { 1.0 } else { 0.0 };
                            let v1085: f64;
                            let v1086: Lanes<2>;
                            if v1046 != 0.0 {
                                let v1059 = v331 * v1058;
                                let v1060 = v338 * v1058;
                                let v1061 = v1059 * v1059;
                                let v1062 = v1060 * v1059;
                                let v1064 = v1061 * v1059;
                                let v1068 = v1064 * v1059;
                                let v1071 = ((((v1062 + v1062) * v1059) + (v1060 * v1061)) * v1059) + (v1060 * v1064);
                                v1085 = v1068;
                                v1086 = v1071;
                            } else {
                                let v1072 = v331 * v1058;
                                let v1074 = v1072.abs();
                                let v1080 = v1074.powf(v1044);
                                let v1084 = ((v338 * v1058) * ((v146 * (if v1072 >= v1075 { 1.0 } else { 0.0 })) - v148)) * (v1044 * (v1074.powf((v1044 - v148))));
                                v1085 = v1080;
                                v1086 = v1084;
                            }
                            let v1087 = v56 - v1085;
                            let v1089 = v56 / v1087;
                            let v1092 = (((v1086 * v54) * v1089) * v54) / v1087;
                            v1056 = v1089;
                            v1057 = v1092;
                        } else {
                            let v1053 = v338 * v1051;
                            let v1055 = v1054 + ((v331 + (v1047 * v1048)) * v1051);
                            v1056 = v1055;
                            v1057 = v1053;
                        }
                        v1028 = v1056;
                        v1029 = v1057;
                    }
                    let v1037 = v1036 * (((v559 + v594) + v699) + v918);
                    let v1039 = v1037 * v1028;
                    let v1042 = (((((v560 + v599) + v700) + v919) * v1036) * v1028) + (v1029 * v1037);
                    let v1110: f64;
                    let v1111: Lanes<2>;
                    if v1043 != 0.0 {
                        let v1097 = (v56 - (v332 * v194)).sqrt();
                        let v1100 = ((v339 * v194) * v54) * (v148 / (v146 * v1097));
                        v1110 = v1097;
                        v1111 = v1100;
                    } else {
                        let v1103 = v56 - (v332 * v194);
                        let v1105 = v1103.powf(v207);
                        let v1109 = ((v339 * v194) * v54) * (v207 * (v1103.powf(v1106)));
                        v1110 = v1105;
                        v1111 = v1109;
                    }
                    let v1123 = v1122 * ((v217 * (v56 - v1110)) + (v222 * (v14 - v332)));
                    let v1124 = (((v1111 * v54) * v217) + ((v15 - v339) * v222)) * v1122;
                    v562 = v590;
                    v563 = v591;
                    v564 = v592;
                    v565 = v593;
                    v566 = v1039;
                    v567 = v1123;
                    v568 = v595;
                    v569 = v596;
                    v570 = v597;
                    v571 = v598;
                    v572 = v1042;
                    v573 = v1124;
                }
                let v1129: f64;
                let v1130: f64;
                let v1131: f64;
                let v1132: f64;
                let v1133: f64;
                let v1134: f64;
                let v1135: Lanes<2>;
                let v1136: Lanes<2>;
                let v1137: Lanes<2>;
                let v1138: Lanes<2>;
                let v1139: Lanes<2>;
                let v1140: Lanes<2>;
                if v2 != 0.0 {
                    v1129 = v562;
                    v1130 = v563;
                    v1131 = v564;
                    v1132 = v565;
                    v1133 = v189;
                    v1134 = v189;
                    v1135 = v568;
                    v1136 = v569;
                    v1137 = v570;
                    v1138 = v571;
                    v1139 = v190;
                    v1140 = v190;
                } else {
                    let v1126 = v1125 * v326;
                    let v1127 = v333 * v1125;
                    let v1157: f64;
                    let v1158: f64;
                    let v1159: f64;
                    let v1160: f64;
                    let v1161: f64;
                    let v1162: Lanes<2>;
                    let v1163: Lanes<2>;
                    let v1164: Lanes<2>;
                    let v1165: Lanes<2>;
                    let v1166: Lanes<2>;
                    if v1128 != 0.0 {
                        v1157 = v562;
                        v1158 = v563;
                        v1159 = v564;
                        v1160 = v565;
                        v1161 = v189;
                        v1162 = v568;
                        v1163 = v569;
                        v1164 = v570;
                        v1165 = v571;
                        v1166 = v190;
                    } else {
                        let v1142 = v1141 - v327;
                        let v1143 = v334 * v54;
                        let v1144 = v328 / v1142;
                        let v1150 = (v56 - v1144).sqrt();
                        let v1154 = v56 - v1150;
                        let v1155 = ((((v335 - (v1143 * v1144)) / v1142) * v54) * (v148 / (v146 * v1150))) * v54;
                        let v1189: f64;
                        let v1190: Lanes<2>;
                        if v1156 != 0.0 {
                            v1189 = v189;
                            v1190 = v190;
                        } else {
                            let v1168 = v1154 * v1154;
                            let v1169 = v1155 * v1154;
                            let v1171 = v1154.ln();
                            let v1178 = v56 - v1154;
                            let v1180 = (v1168 * v1171) / v1178;
                            let v1187 = (v1180 + v1154) * v1186;
                            let v1188 = ((((((v1169 + v1169) * v1171) + ((v1155 * (v148 / v1154)) * v1168)) - ((v1155 * v54) * v1180)) / v1178) + v1155) * v1186;
                            v1189 = v1187;
                            v1190 = v1188;
                        }
                        let v1191 = v1154 + v1189;
                        let v1192 = v1155 + v1190;
                        let v1208: f64;
                        let v1209: Lanes<2>;
                        if v1156 != 0.0 {
                            let v1196 = (v1142 * v1193).sqrt();
                            let v1199 = (v1143 * v1193) * (v148 / (v146 * v1196));
                            v1208 = v1196;
                            v1209 = v1199;
                        } else {
                            let v1200 = v1142 * v1193;
                            let v1203 = v1200.powf(v1202);
                            let v1207 = (v1143 * v1193) * (v1202 * (v1200.powf(v1204)));
                            v1208 = v1203;
                            v1209 = v1207;
                        }
                        let v1211 = v1210 * v1208;
                        let v1212 = v1209 * v1210;
                        let v1213 = v329 - v56;
                        let v1219 = v1218 * (v1213 * v1211);
                        let v1220 = ((v336 * v1211) + (v1212 * v1213)) * v1218;
                        let v1226 = v1225 * (v1219 * v1191);
                        let v1227 = ((v1220 * v1191) + (v1192 * v1219)) * v1225;
                        v1157 = v1211;
                        v1158 = v1142;
                        v1159 = v1191;
                        v1160 = v1219;
                        v1161 = v1226;
                        v1162 = v1212;
                        v1163 = v1143;
                        v1164 = v1192;
                        v1165 = v1220;
                        v1166 = v1227;
                    }
                    let v1266: f64;
                    let v1267: Lanes<2>;
                    if v1167 != 0.0 {
                        v1266 = v189;
                        v1267 = v190;
                    } else {
                        let v1230 = (v1157 * v244) / v1158;
                        let v1235 = v1234 * v1230;
                        let v1236 = (((v1162 * v244) - (v1163 * v1230)) / v1158) * v1234;
                        let v1238 = v1237 / v1235;
                        let v1241 = ((v1236 * v1238) * v54) / v1235;
                        let v1242 = v1238 * v1238;
                        let v1243 = v1241 * v1238;
                        let v1245 = v1242 * v1242;
                        let v1246 = (v1243 + v1243) * v1242;
                        let v1247 = v1246 + v1246;
                        let v1248 = v1245 + v56;
                        let v1249 = v1245 / v1248;
                        let v1253 = v1249.sqrt();
                        let v1256 = ((v1247 - (v1247 * v1249)) / v1248) * (v148 / (v146 * v1253));
                        let v1257 = v1253.sqrt();
                        let v1260 = v1256 * (v148 / (v146 * v1257));
                        let v1261 = v1253 * v1257;
                        let v1264 = (v1256 * v1257) + (v1260 * v1253);
                        let v1289: f64;
                        let v1290: Lanes<2>;
                        if v1265 != 0.0 {
                            let v1273 = v56 + (v1235 * v1261);
                            let v1274 = v56 / v1273;
                            let v1277 = ((((v1236 * v1261) + (v1264 * v1235)) * v1274) * v54) / v1273;
                            v1289 = v1274;
                            v1290 = v1277;
                        } else {
                            let v1282 = v56 + (v1235 * v1261);
                            let v1284 = v1282.powf(v1283);
                            let v1288 = ((v1236 * v1261) + (v1264 * v1235)) * (v1283 * (v1282.powf(v1285)));
                            v1289 = v1284;
                            v1290 = v1288;
                        }
                        let v1295 = v1159 + v1289;
                        let v1297 = (v1159 * v1289) / v1295;
                        let v1300 = (((v1164 * v1289) + (v1290 * v1159)) - ((v1164 + v1290) * v1297)) / v1295;
                        let v1301 = v1235 / v1257;
                        let v1307 = (v738 * v1301).sqrt();
                        let v1310 = (((v1236 - (v1260 * v1301)) / v1257) * v738) * (v148 / (v146 * v1307));
                        let v1320 = v1319 * v1238;
                        let v1336 = ((v1320 * v1257) - (v1319 * v1253)) + (v413 * (v1235 * v1261));
                        let v1337 = ((((v1241 * v1319) * v1257) + (v1260 * v1320)) - (v1256 * v1319)) + (((v1236 * v1261) + (v1264 * v1235)) * v413);
                        let v1338 = ((v159 * (v1238 * v1257)) - v1253) - v56;
                        let v1339 = v1338 * v1307;
                        let v1342 = (((((v1241 * v1257) + (v1260 * v1238)) * v159) - v1256) * v1307) + (v1310 * v1338);
                        let v1343 = v1339 * v1339;
                        let v1344 = v1342 * v1339;
                        let v1345 = v1344 + v1344;
                        let v1346 = if v1339 > v189 { 1.0 } else { 0.0 };
                        let v1362: f64;
                        let v1363: Lanes<2>;
                        if v1346 != 0.0 {
                            let v1349 = v56 + (v781 * v1339);
                            let v1350 = v56 / v1349;
                            let v1353 = (((v1342 * v781) * v1350) * v54) / v1349;
                            v1362 = v1350;
                            v1363 = v1353;
                        } else {
                            let v1356 = v56 - (v781 * v1339);
                            let v1358 = v56 / v1356;
                            let v1361 = ((((v1342 * v781) * v54) * v1358) * v54) / v1356;
                            v1362 = v1358;
                            v1363 = v1361;
                        }
                        let v1366 = (-v1343) + v1336;
                        let v1367 = (v1345 * v54) + v1337;
                        let v1369 = if v1366 > v1368 { 1.0 } else { 0.0 };
                        let v1398: f64;
                        let v1399: Lanes<2>;
                        if v1369 != 0.0 {
                            let v1370 = v1366.exp();
                            let v1371 = v1367 * v1370;
                            v1398 = v1370;
                            v1399 = v1371;
                        } else {
                            let v1373 = v1372 - v1366;
                            let v1374 = v1367 * v54;
                            let v1376 = v1375 - v1366;
                            let v1381 = v56 + ((v1377 - v1366) * v405);
                            let v1388 = v56 + (v413 * (v1376 * v1381));
                            let v1393 = v56 + (v1373 * v1388);
                            let v1394 = v58 / v1393;
                            let v1397 = ((((v1374 * v1388) + ((((v1374 * v1381) + ((v1374 * v405) * v1376)) * v413) * v1373)) * v1394) * v54) / v1393;
                            v1398 = v1394;
                            v1399 = v1397;
                        }
                        let v1402 = v1362 * v1362;
                        let v1403 = v1363 * v1362;
                        let v1404 = v1403 + v1403;
                        let v1415 = ((v835 * v1362) + (v841 * v1402)) + (v850 * (v1402 * v1362));
                        let v1417 = v1415 * v1398;
                        let v1420 = ((((v1363 * v835) + (v1404 * v841)) + (((v1404 * v1362) + (v1363 * v1402)) * v850)) * v1398) + (v1399 * v1415);
                        let v1423: f64;
                        let v1424: Lanes<2>;
                        if v1346 != 0.0 {
                            v1423 = v1417;
                            v1424 = v1420;
                        } else {
                            let v1422 = if v1336 > v1421 { 1.0 } else { 0.0 };
                            let v1473: f64;
                            let v1474: Lanes<2>;
                            if v1422 != 0.0 {
                                let v1445 = v1336.exp();
                                let v1446 = v1337 * v1445;
                                v1473 = v1445;
                                v1474 = v1446;
                            } else {
                                let v1448 = v1447 - v1336;
                                let v1449 = v1337 * v54;
                                let v1451 = v1450 - v1336;
                                let v1456 = v56 + ((v1452 - v1336) * v405);
                                let v1463 = v56 + (v413 * (v1451 * v1456));
                                let v1468 = v56 + (v1448 * v1463);
                                let v1469 = v58 / v1468;
                                let v1472 = ((((v1449 * v1463) + ((((v1449 * v1456) + ((v1449 * v405) * v1451)) * v413) * v1448)) * v1469) * v54) / v1468;
                                v1473 = v1469;
                                v1474 = v1472;
                            }
                            let v1477 = (v159 * v1473) - v1417;
                            let v1478 = (v1474 * v159) - v1420;
                            v1423 = v1477;
                            v1424 = v1478;
                        }
                        let v1427 = (v1319 * v1423) / v1307;
                        let v1432 = v1431 * v1427;
                        let v1434 = v1160 * v1432;
                        let v1443 = v1442 * (v1434 * v1297);
                        let v1444 = ((((v1165 * v1432) + (((((v1424 * v1319) - (v1310 * v1427)) / v1307) * v1431) * v1160)) * v1297) + (v1300 * v1434)) * v1442;
                        v1266 = v1443;
                        v1267 = v1444;
                    }
                    let v1480: f64;
                    let v1481: Lanes<2>;
                    if v1268 != 0.0 {
                        v1480 = v189;
                        v1481 = v190;
                    } else {
                        let v1501: f64;
                        let v1502: Lanes<2>;
                        if v1479 != 0.0 {
                            let v1488 = ((v1483 - v330) * v1193).sqrt();
                            let v1491 = ((v337 * v54) * v1193) * (v148 / (v146 * v1488));
                            v1501 = v1488;
                            v1502 = v1491;
                        } else {
                            let v1494 = (v1483 - v330) * v1193;
                            let v1496 = v1494.powf(v1202);
                            let v1500 = ((v337 * v54) * v1193) * (v1202 * (v1494.powf(v1497)));
                            v1501 = v1496;
                            v1502 = v1500;
                        }
                        let v1508 = ((v1483 - v330) * v1505) / v1501;
                        let v1513 = v1512 * v1508;
                        let v1514 = ((((v337 * v54) * v1505) - (v1502 * v1508)) / v1501) * v1512;
                        let v1516 = v1515 / v1513;
                        let v1519 = ((v1514 * v1516) * v54) / v1513;
                        let v1521 = if (v1516.abs()) < v345 { 1.0 } else { 0.0 };
                        let v1525: f64;
                        let v1526: Lanes<2>;
                        if v1521 != 0.0 {
                            let v1522 = v1516.exp();
                            let v1523 = v1519 * v1522;
                            v1525 = v1522;
                            v1526 = v1523;
                        } else {
                            let v1524 = if v1516 < v189 { 1.0 } else { 0.0 };
                            let v1586: f64;
                            let v1587: Lanes<2>;
                            if v1524 != 0.0 {
                                let v1543 = v1542 - v1516;
                                let v1544 = v1519 * v54;
                                let v1546 = v1545 - v1516;
                                let v1551 = v56 + ((v1547 - v1516) * v405);
                                let v1558 = v56 + (v413 * (v1546 * v1551));
                                let v1563 = v56 + (v1543 * v1558);
                                let v1564 = v58 / v1563;
                                let v1567 = ((((v1544 * v1558) + ((((v1544 * v1551) + ((v1544 * v405) * v1546)) * v413) * v1543)) * v1564) * v54) / v1563;
                                v1586 = v1564;
                                v1587 = v1567;
                            } else {
                                let v1568 = v1516 - v345;
                                let v1571 = v56 + (v1568 * v405);
                                let v1578 = v56 + (v413 * (v1568 * v1571));
                                let v1584 = v453 * (v56 + (v1568 * v1578));
                                let v1585 = ((v1519 * v1578) + ((((v1519 * v1571) + ((v1519 * v405) * v1568)) * v413) * v1568)) * v453;
                                v1586 = v1584;
                                v1587 = v1585;
                            }
                            v1525 = v1586;
                            v1526 = v1587;
                        }
                        let v1527 = v14 * v1513;
                        let v1531 = v1527 * v1513;
                        let v1540 = v1539 * (v1531 * v1525);
                        let v1541 = ((((((v15 * v1513) + (v1514 * v14)) * v1513) + (v1514 * v1527)) * v1525) + (v1526 * v1531)) * v1539;
                        v1480 = v1540;
                        v1481 = v1541;
                    }
                    let v1590: f64;
                    let v1591: Lanes<2>;
                    if v1482 != 0.0 {
                        v1590 = v56;
                        v1591 = v190;
                    } else {
                        let v1589 = if v331 > v1588 { 1.0 } else { 0.0 };
                        let v1615: f64;
                        let v1616: Lanes<2>;
                        if v1589 != 0.0 {
                            let v1606 = if v1605 == v1045 { 1.0 } else { 0.0 };
                            let v1643: f64;
                            let v1644: Lanes<2>;
                            if v1606 != 0.0 {
                                let v1618 = v331 * v1617;
                                let v1619 = v338 * v1617;
                                let v1620 = v1618 * v1618;
                                let v1621 = v1619 * v1618;
                                let v1623 = v1620 * v1618;
                                let v1627 = v1623 * v1618;
                                let v1630 = ((((v1621 + v1621) * v1618) + (v1619 * v1620)) * v1618) + (v1619 * v1623);
                                v1643 = v1627;
                                v1644 = v1630;
                            } else {
                                let v1631 = v331 * v1617;
                                let v1633 = v1631.abs();
                                let v1638 = v1633.powf(v1605);
                                let v1642 = ((v338 * v1617) * ((v146 * (if v1631 >= v1075 { 1.0 } else { 0.0 })) - v148)) * (v1605 * (v1633.powf((v1605 - v148))));
                                v1643 = v1638;
                                v1644 = v1642;
                            }
                            let v1645 = v56 - v1643;
                            let v1647 = v56 / v1645;
                            let v1650 = (((v1644 * v54) * v1647) * v54) / v1645;
                            v1615 = v1647;
                            v1616 = v1650;
                        } else {
                            let v1612 = v338 * v1610;
                            let v1614 = v1613 + ((v331 + (v1047 * v1607)) * v1610);
                            v1615 = v1614;
                            v1616 = v1612;
                        }
                        v1590 = v1615;
                        v1591 = v1616;
                    }
                    let v1598 = v1036 * (((v1126 + v1161) + v1266) + v1480);
                    let v1600 = v1598 * v1590;
                    let v1603 = (((((v1127 + v1166) + v1267) + v1481) * v1036) * v1590) + (v1591 * v1598);
                    let v1668: f64;
                    let v1669: Lanes<2>;
                    if v1604 != 0.0 {
                        let v1655 = (v56 - (v332 * v231)).sqrt();
                        let v1658 = ((v339 * v231) * v54) * (v148 / (v146 * v1655));
                        v1668 = v1655;
                        v1669 = v1658;
                    } else {
                        let v1661 = v56 - (v332 * v231);
                        let v1663 = v1661.powf(v244);
                        let v1667 = ((v339 * v231) * v54) * (v244 * (v1661.powf(v1664)));
                        v1668 = v1663;
                        v1669 = v1667;
                    }
                    let v1680 = v1122 * ((v254 * (v56 - v1668)) + (v259 * (v14 - v332)));
                    let v1681 = (((v1669 * v54) * v254) + ((v15 - v339) * v259)) * v1122;
                    v1129 = v1157;
                    v1130 = v1158;
                    v1131 = v1159;
                    v1132 = v1160;
                    v1133 = v1600;
                    v1134 = v1680;
                    v1135 = v1162;
                    v1136 = v1163;
                    v1137 = v1164;
                    v1138 = v1165;
                    v1139 = v1603;
                    v1140 = v1681;
                }
                let v1686: f64;
                let v1687: f64;
                let v1688: Lanes<2>;
                let v1689: Lanes<2>;
                if v3 != 0.0 {
                    v1686 = v189;
                    v1687 = v189;
                    v1688 = v190;
                    v1689 = v190;
                } else {
                    let v1683 = v1682 * v326;
                    let v1684 = v333 * v1682;
                    let v1716: f64;
                    let v1717: f64;
                    let v1718: f64;
                    let v1719: f64;
                    let v1720: f64;
                    let v1721: Lanes<2>;
                    let v1722: Lanes<2>;
                    let v1723: Lanes<2>;
                    let v1724: Lanes<2>;
                    let v1725: Lanes<2>;
                    if v1685 != 0.0 {
                        v1716 = v1129;
                        v1717 = v1130;
                        v1718 = v1131;
                        v1719 = v1132;
                        v1720 = v189;
                        v1721 = v1135;
                        v1722 = v1136;
                        v1723 = v1137;
                        v1724 = v1138;
                        v1725 = v190;
                    } else {
                        let v1701 = v1700 - v327;
                        let v1702 = v334 * v54;
                        let v1703 = v328 / v1701;
                        let v1709 = (v56 - v1703).sqrt();
                        let v1713 = v56 - v1709;
                        let v1714 = ((((v335 - (v1702 * v1703)) / v1701) * v54) * (v148 / (v146 * v1709))) * v54;
                        let v1748: f64;
                        let v1749: Lanes<2>;
                        if v1715 != 0.0 {
                            v1748 = v189;
                            v1749 = v190;
                        } else {
                            let v1727 = v1713 * v1713;
                            let v1728 = v1714 * v1713;
                            let v1730 = v1713.ln();
                            let v1737 = v56 - v1713;
                            let v1739 = (v1727 * v1730) / v1737;
                            let v1746 = (v1739 + v1713) * v1745;
                            let v1747 = ((((((v1728 + v1728) * v1730) + ((v1714 * (v148 / v1713)) * v1727)) - ((v1714 * v54) * v1739)) / v1737) + v1714) * v1745;
                            v1748 = v1746;
                            v1749 = v1747;
                        }
                        let v1750 = v1713 + v1748;
                        let v1751 = v1714 + v1749;
                        let v1767: f64;
                        let v1768: Lanes<2>;
                        if v1715 != 0.0 {
                            let v1755 = (v1701 * v1752).sqrt();
                            let v1758 = (v1702 * v1752) * (v148 / (v146 * v1755));
                            v1767 = v1755;
                            v1768 = v1758;
                        } else {
                            let v1759 = v1701 * v1752;
                            let v1762 = v1759.powf(v1761);
                            let v1766 = (v1702 * v1752) * (v1761 * (v1759.powf(v1763)));
                            v1767 = v1762;
                            v1768 = v1766;
                        }
                        let v1770 = v1769 * v1767;
                        let v1771 = v1768 * v1769;
                        let v1772 = v329 - v56;
                        let v1778 = v1777 * (v1772 * v1770);
                        let v1779 = ((v336 * v1770) + (v1771 * v1772)) * v1777;
                        let v1785 = v1784 * (v1778 * v1750);
                        let v1786 = ((v1779 * v1750) + (v1751 * v1778)) * v1784;
                        v1716 = v1770;
                        v1717 = v1701;
                        v1718 = v1750;
                        v1719 = v1778;
                        v1720 = v1785;
                        v1721 = v1771;
                        v1722 = v1702;
                        v1723 = v1751;
                        v1724 = v1779;
                        v1725 = v1786;
                    }
                    let v1825: f64;
                    let v1826: Lanes<2>;
                    if v1726 != 0.0 {
                        v1825 = v189;
                        v1826 = v190;
                    } else {
                        let v1789 = (v1716 * v280) / v1717;
                        let v1794 = v1793 * v1789;
                        let v1795 = (((v1721 * v280) - (v1722 * v1789)) / v1717) * v1793;
                        let v1797 = v1796 / v1794;
                        let v1800 = ((v1795 * v1797) * v54) / v1794;
                        let v1801 = v1797 * v1797;
                        let v1802 = v1800 * v1797;
                        let v1804 = v1801 * v1801;
                        let v1805 = (v1802 + v1802) * v1801;
                        let v1806 = v1805 + v1805;
                        let v1807 = v1804 + v56;
                        let v1808 = v1804 / v1807;
                        let v1812 = v1808.sqrt();
                        let v1815 = ((v1806 - (v1806 * v1808)) / v1807) * (v148 / (v146 * v1812));
                        let v1816 = v1812.sqrt();
                        let v1819 = v1815 * (v148 / (v146 * v1816));
                        let v1820 = v1812 * v1816;
                        let v1823 = (v1815 * v1816) + (v1819 * v1812);
                        let v1848: f64;
                        let v1849: Lanes<2>;
                        if v1824 != 0.0 {
                            let v1832 = v56 + (v1794 * v1820);
                            let v1833 = v56 / v1832;
                            let v1836 = ((((v1795 * v1820) + (v1823 * v1794)) * v1833) * v54) / v1832;
                            v1848 = v1833;
                            v1849 = v1836;
                        } else {
                            let v1841 = v56 + (v1794 * v1820);
                            let v1843 = v1841.powf(v1842);
                            let v1847 = ((v1795 * v1820) + (v1823 * v1794)) * (v1842 * (v1841.powf(v1844)));
                            v1848 = v1843;
                            v1849 = v1847;
                        }
                        let v1854 = v1718 + v1848;
                        let v1856 = (v1718 * v1848) / v1854;
                        let v1859 = (((v1723 * v1848) + (v1849 * v1718)) - ((v1723 + v1849) * v1856)) / v1854;
                        let v1860 = v1794 / v1816;
                        let v1866 = (v738 * v1860).sqrt();
                        let v1869 = (((v1795 - (v1819 * v1860)) / v1816) * v738) * (v148 / (v146 * v1866));
                        let v1879 = v1878 * v1797;
                        let v1895 = ((v1879 * v1816) - (v1878 * v1812)) + (v413 * (v1794 * v1820));
                        let v1896 = ((((v1800 * v1878) * v1816) + (v1819 * v1879)) - (v1815 * v1878)) + (((v1795 * v1820) + (v1823 * v1794)) * v413);
                        let v1897 = ((v159 * (v1797 * v1816)) - v1812) - v56;
                        let v1898 = v1897 * v1866;
                        let v1901 = (((((v1800 * v1816) + (v1819 * v1797)) * v159) - v1815) * v1866) + (v1869 * v1897);
                        let v1902 = v1898 * v1898;
                        let v1903 = v1901 * v1898;
                        let v1904 = v1903 + v1903;
                        let v1905 = if v1898 > v189 { 1.0 } else { 0.0 };
                        let v1921: f64;
                        let v1922: Lanes<2>;
                        if v1905 != 0.0 {
                            let v1908 = v56 + (v781 * v1898);
                            let v1909 = v56 / v1908;
                            let v1912 = (((v1901 * v781) * v1909) * v54) / v1908;
                            v1921 = v1909;
                            v1922 = v1912;
                        } else {
                            let v1915 = v56 - (v781 * v1898);
                            let v1917 = v56 / v1915;
                            let v1920 = ((((v1901 * v781) * v54) * v1917) * v54) / v1915;
                            v1921 = v1917;
                            v1922 = v1920;
                        }
                        let v1925 = (-v1902) + v1895;
                        let v1926 = (v1904 * v54) + v1896;
                        let v1928 = if v1925 > v1927 { 1.0 } else { 0.0 };
                        let v1957: f64;
                        let v1958: Lanes<2>;
                        if v1928 != 0.0 {
                            let v1929 = v1925.exp();
                            let v1930 = v1926 * v1929;
                            v1957 = v1929;
                            v1958 = v1930;
                        } else {
                            let v1932 = v1931 - v1925;
                            let v1933 = v1926 * v54;
                            let v1935 = v1934 - v1925;
                            let v1940 = v56 + ((v1936 - v1925) * v405);
                            let v1947 = v56 + (v413 * (v1935 * v1940));
                            let v1952 = v56 + (v1932 * v1947);
                            let v1953 = v58 / v1952;
                            let v1956 = ((((v1933 * v1947) + ((((v1933 * v1940) + ((v1933 * v405) * v1935)) * v413) * v1932)) * v1953) * v54) / v1952;
                            v1957 = v1953;
                            v1958 = v1956;
                        }
                        let v1961 = v1921 * v1921;
                        let v1962 = v1922 * v1921;
                        let v1963 = v1962 + v1962;
                        let v1974 = ((v835 * v1921) + (v841 * v1961)) + (v850 * (v1961 * v1921));
                        let v1976 = v1974 * v1957;
                        let v1979 = ((((v1922 * v835) + (v1963 * v841)) + (((v1963 * v1921) + (v1922 * v1961)) * v850)) * v1957) + (v1958 * v1974);
                        let v1982: f64;
                        let v1983: Lanes<2>;
                        if v1905 != 0.0 {
                            v1982 = v1976;
                            v1983 = v1979;
                        } else {
                            let v1981 = if v1895 > v1980 { 1.0 } else { 0.0 };
                            let v2032: f64;
                            let v2033: Lanes<2>;
                            if v1981 != 0.0 {
                                let v2004 = v1895.exp();
                                let v2005 = v1896 * v2004;
                                v2032 = v2004;
                                v2033 = v2005;
                            } else {
                                let v2007 = v2006 - v1895;
                                let v2008 = v1896 * v54;
                                let v2010 = v2009 - v1895;
                                let v2015 = v56 + ((v2011 - v1895) * v405);
                                let v2022 = v56 + (v413 * (v2010 * v2015));
                                let v2027 = v56 + (v2007 * v2022);
                                let v2028 = v58 / v2027;
                                let v2031 = ((((v2008 * v2022) + ((((v2008 * v2015) + ((v2008 * v405) * v2010)) * v413) * v2007)) * v2028) * v54) / v2027;
                                v2032 = v2028;
                                v2033 = v2031;
                            }
                            let v2036 = (v159 * v2032) - v1976;
                            let v2037 = (v2033 * v159) - v1979;
                            v1982 = v2036;
                            v1983 = v2037;
                        }
                        let v1986 = (v1878 * v1982) / v1866;
                        let v1991 = v1990 * v1986;
                        let v1993 = v1719 * v1991;
                        let v2002 = v2001 * (v1993 * v1856);
                        let v2003 = ((((v1724 * v1991) + (((((v1983 * v1878) - (v1869 * v1986)) / v1866) * v1990) * v1719)) * v1856) + (v1859 * v1993)) * v2001;
                        v1825 = v2002;
                        v1826 = v2003;
                    }
                    let v2039: f64;
                    let v2040: Lanes<2>;
                    if v1827 != 0.0 {
                        v2039 = v189;
                        v2040 = v190;
                    } else {
                        let v2060: f64;
                        let v2061: Lanes<2>;
                        if v2038 != 0.0 {
                            let v2047 = ((v2042 - v330) * v1752).sqrt();
                            let v2050 = ((v337 * v54) * v1752) * (v148 / (v146 * v2047));
                            v2060 = v2047;
                            v2061 = v2050;
                        } else {
                            let v2053 = (v2042 - v330) * v1752;
                            let v2055 = v2053.powf(v1761);
                            let v2059 = ((v337 * v54) * v1752) * (v1761 * (v2053.powf(v2056)));
                            v2060 = v2055;
                            v2061 = v2059;
                        }
                        let v2067 = ((v2042 - v330) * v2064) / v2060;
                        let v2072 = v2071 * v2067;
                        let v2073 = ((((v337 * v54) * v2064) - (v2061 * v2067)) / v2060) * v2071;
                        let v2075 = v2074 / v2072;
                        let v2078 = ((v2073 * v2075) * v54) / v2072;
                        let v2080 = if (v2075.abs()) < v345 { 1.0 } else { 0.0 };
                        let v2084: f64;
                        let v2085: Lanes<2>;
                        if v2080 != 0.0 {
                            let v2081 = v2075.exp();
                            let v2082 = v2078 * v2081;
                            v2084 = v2081;
                            v2085 = v2082;
                        } else {
                            let v2083 = if v2075 < v189 { 1.0 } else { 0.0 };
                            let v2145: f64;
                            let v2146: Lanes<2>;
                            if v2083 != 0.0 {
                                let v2102 = v2101 - v2075;
                                let v2103 = v2078 * v54;
                                let v2105 = v2104 - v2075;
                                let v2110 = v56 + ((v2106 - v2075) * v405);
                                let v2117 = v56 + (v413 * (v2105 * v2110));
                                let v2122 = v56 + (v2102 * v2117);
                                let v2123 = v58 / v2122;
                                let v2126 = ((((v2103 * v2117) + ((((v2103 * v2110) + ((v2103 * v405) * v2105)) * v413) * v2102)) * v2123) * v54) / v2122;
                                v2145 = v2123;
                                v2146 = v2126;
                            } else {
                                let v2127 = v2075 - v345;
                                let v2130 = v56 + (v2127 * v405);
                                let v2137 = v56 + (v413 * (v2127 * v2130));
                                let v2143 = v453 * (v56 + (v2127 * v2137));
                                let v2144 = ((v2078 * v2137) + ((((v2078 * v2130) + ((v2078 * v405) * v2127)) * v413) * v2127)) * v453;
                                v2145 = v2143;
                                v2146 = v2144;
                            }
                            v2084 = v2145;
                            v2085 = v2146;
                        }
                        let v2086 = v14 * v2072;
                        let v2090 = v2086 * v2072;
                        let v2099 = v2098 * (v2090 * v2084);
                        let v2100 = ((((((v15 * v2072) + (v2073 * v14)) * v2072) + (v2073 * v2086)) * v2084) + (v2085 * v2090)) * v2098;
                        v2039 = v2099;
                        v2040 = v2100;
                    }
                    let v2149: f64;
                    let v2150: Lanes<2>;
                    if v2041 != 0.0 {
                        v2149 = v56;
                        v2150 = v190;
                    } else {
                        let v2148 = if v331 > v2147 { 1.0 } else { 0.0 };
                        let v2173: f64;
                        let v2174: Lanes<2>;
                        if v2148 != 0.0 {
                            let v2164 = if v2163 == v1045 { 1.0 } else { 0.0 };
                            let v2201: f64;
                            let v2202: Lanes<2>;
                            if v2164 != 0.0 {
                                let v2176 = v331 * v2175;
                                let v2177 = v338 * v2175;
                                let v2178 = v2176 * v2176;
                                let v2179 = v2177 * v2176;
                                let v2181 = v2178 * v2176;
                                let v2185 = v2181 * v2176;
                                let v2188 = ((((v2179 + v2179) * v2176) + (v2177 * v2178)) * v2176) + (v2177 * v2181);
                                v2201 = v2185;
                                v2202 = v2188;
                            } else {
                                let v2189 = v331 * v2175;
                                let v2191 = v2189.abs();
                                let v2196 = v2191.powf(v2163);
                                let v2200 = ((v338 * v2175) * ((v146 * (if v2189 >= v1075 { 1.0 } else { 0.0 })) - v148)) * (v2163 * (v2191.powf((v2163 - v148))));
                                v2201 = v2196;
                                v2202 = v2200;
                            }
                            let v2203 = v56 - v2201;
                            let v2205 = v56 / v2203;
                            let v2208 = (((v2202 * v54) * v2205) * v54) / v2203;
                            v2173 = v2205;
                            v2174 = v2208;
                        } else {
                            let v2170 = v338 * v2168;
                            let v2172 = v2171 + ((v331 + (v1047 * v2165)) * v2168);
                            v2173 = v2172;
                            v2174 = v2170;
                        }
                        v2149 = v2173;
                        v2150 = v2174;
                    }
                    let v2157 = v1036 * (((v1683 + v1720) + v1825) + v2039);
                    let v2159 = v2157 * v2149;
                    let v2162 = (((((v1684 + v1725) + v1826) + v2040) * v1036) * v2149) + (v2150 * v2157);
                    let v2212: f64;
                    let v2213: Lanes<2>;
                    if v0 != 0.0 {
                        let v2210 = if v14 < v2209 { 1.0 } else { 0.0 };
                        let v2224: f64;
                        let v2225: Lanes<2>;
                        if v2210 != 0.0 {
                            let v2216 = (v14 - v2209) / v2215;
                            let v2217 = v15 / v2215;
                            let v2219 = if v2216 < v2218 { 1.0 } else { 0.0 };
                            let v2260: f64;
                            let v2261: Lanes<2>;
                            if v2219 != 0.0 {
                                v2260 = v2209;
                                v2261 = v190;
                            } else {
                                let v2251 = v2216.exp();
                                let v2253 = v56 + v2251;
                                let v2258 = ((v2217 * v2251) * (v148 / v2253)) * v2215;
                                let v2259 = v2209 + ((v2253.ln()) * v2215);
                                v2260 = v2259;
                                v2261 = v2258;
                            }
                            v2224 = v2260;
                            v2225 = v2261;
                        } else {
                            let v2223 = if ((v14 - v2209) / v2215) > v2222 { 1.0 } else { 0.0 };
                            let v2276: f64;
                            let v2277: Lanes<2>;
                            if v2223 != 0.0 {
                                v2276 = v14;
                                v2277 = v15;
                            } else {
                                let v2266 = ((v2209 - v14) / v2215).exp();
                                let v2268 = v56 + v2266;
                                let v2274 = v14 + ((v2268.ln()) * v2215);
                                let v2275 = v15 + (((((v15 * v54) / v2215) * v2266) * (v148 / v2268)) * v2215);
                                v2276 = v2274;
                                v2277 = v2275;
                            }
                            v2224 = v2276;
                            v2225 = v2277;
                        }
                        let v2227 = v2224 + v2226;
                        let v2229 = v136 - v2227;
                        let v2232 = (v2225 * v54) * v2229;
                        let v2236 = ((v2229 * v2229) + v2234).sqrt();
                        let v2242 = (v136 + v2227) + v2236;
                        let v2244 = (v2224 * v136) / v2242;
                        let v2248 = v159 * v2244;
                        let v2249 = (((v2225 * v136) - ((v2225 + ((v2232 + v2232) * (v148 / (v146 * v2236)))) * v2244)) / v2242) * v159;
                        let v2295: f64;
                        let v2296: Lanes<2>;
                        if v2250 != 0.0 {
                            let v2282 = (v56 - (v2248 * v267)).sqrt();
                            let v2285 = ((v2249 * v267) * v54) * (v148 / (v146 * v2282));
                            v2295 = v2282;
                            v2296 = v2285;
                        } else {
                            let v2288 = v56 - (v2248 * v267);
                            let v2290 = v2288.powf(v280);
                            let v2294 = ((v2249 * v267) * v54) * (v280 * (v2288.powf(v2291)));
                            v2295 = v2290;
                            v2296 = v2294;
                        }
                        let v2307 = v1122 * ((v290 * (v56 - v2295)) + (v295 * (v2224 - v2248)));
                        let v2308 = (((v2296 * v54) * v290) + ((v2225 - v2249) * v295)) * v1122;
                        let v2310 = (v14 + v2209) - v2224;
                        let v2311 = v15 - v2225;
                        let v2312 = v2310 + v2226;
                        let v2314 = v136 - v2312;
                        let v2317 = (v2311 * v54) * v2314;
                        let v2320 = ((v2314 * v2314) + v2234).sqrt();
                        let v2326 = (v136 + v2312) + v2320;
                        let v2328 = (v2310 * v136) / v2326;
                        let v2332 = v159 * v2328;
                        let v2333 = (((v2311 * v136) - ((v2311 + ((v2317 + v2317) * (v148 / (v146 * v2320)))) * v2328)) / v2326) * v159;
                        let v2354: f64;
                        let v2355: Lanes<2>;
                        if v2334 != 0.0 {
                            let v2340 = (v56 - (v2332 * v2335)).sqrt();
                            let v2343 = ((v2333 * v2335) * v54) * (v148 / (v146 * v2340));
                            v2354 = v2340;
                            v2355 = v2343;
                        } else {
                            let v2346 = v56 - (v2332 * v2335);
                            let v2349 = v2346.powf(v2348);
                            let v2353 = ((v2333 * v2335) * v54) * (v2348 * (v2346.powf(v2350)));
                            v2354 = v2349;
                            v2355 = v2353;
                        }
                        let v2370 = v2307 + (v1122 * ((v2358 * (v56 - v2354)) + (v2363 * (v2310 - v2332))));
                        let v2371 = v2308 + ((((v2355 * v54) * v2358) + ((v2311 - v2333) * v2363)) * v1122);
                        v2212 = v2370;
                        v2213 = v2371;
                    } else {
                        let v2389: f64;
                        let v2390: Lanes<2>;
                        if v2211 != 0.0 {
                            let v2376 = (v56 - (v332 * v267)).sqrt();
                            let v2379 = ((v339 * v267) * v54) * (v148 / (v146 * v2376));
                            v2389 = v2376;
                            v2390 = v2379;
                        } else {
                            let v2382 = v56 - (v332 * v267);
                            let v2384 = v2382.powf(v280);
                            let v2388 = ((v339 * v267) * v54) * (v280 * (v2382.powf(v2385)));
                            v2389 = v2384;
                            v2390 = v2388;
                        }
                        let v2401 = v1122 * ((v290 * (v56 - v2389)) + (v295 * (v14 - v332)));
                        let v2402 = (((v2390 * v54) * v290) + ((v15 - v339) * v295)) * v1122;
                        v2212 = v2401;
                        v2213 = v2402;
                    }
                    v1686 = v2159;
                    v1687 = v2212;
                    v1688 = v2162;
                    v1689 = v2213;
                }
                let v1698 = ((v30 * v566) + (v33 * v1133)) + (v38 * v1686);
                let v1699 = ((v572 * v30) + (v1139 * v33)) + (v1688 * v38);
                v22 = v567;
                v23 = v1134;
                v24 = v1687;
                v25 = v1698;
                v26 = v573;
                v27 = v1140;
                v28 = v1689;
                v29 = v1699;
            }
            let v44 = v43 * v25;
            let v45 = v29 * v43;
            let v47 = v46 * (((v30 * v22) + (v33 * v23)) + (v38 * v24));
            let v48 = (((v26 * v30) + (v27 * v33)) + (v28 * v38)) * v46;
            let v49 = ddt(37556, v47);
            let v51 = v48 * v50;
            let v2403 = v45[0];
            let v2404 = v45[1];
            let v2405 = v51[0];
            let v2406 = v51[1];
            let v2407 = v48[0];
            let v2408 = v48[1];
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(1),
            multiplicity * (v44),
            [0, 1],
            [v2403, v2404],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(1),
            multiplicity * (v49),
            [0, 1],
            [v2405, v2406],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(1),
            multiplicity * (v2409),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v44;
        self.canonical_reactive[1] = v47;
        self.canonical_reactive[2] = v2407;
        self.canonical_reactive[3] = v2408;
        self.canonical_reactive[4] = v2409;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(0),
            Some(1),
            &[0, 1],
            &[cached[2], cached[3]],
            &[],
            &[],
            multiplicity,
        );
    }

}

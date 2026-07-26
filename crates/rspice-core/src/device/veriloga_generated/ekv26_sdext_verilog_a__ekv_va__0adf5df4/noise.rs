#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_S_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 8, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_D_S_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 8, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 271];
        let noise_source_0_active = {
            params.p1 != 0.0
        };
        let noise_source_1_active = {
            params.p1 != 0.0
        };
        let noise_source_active = [noise_source_0_active, noise_source_1_active];
        let noise_source_active_mask = [(noise_source_0_active as u128) | ((noise_source_1_active as u128) << 1)];
        w.fill(0.0);
        self.noise_metadata_schedule_part_0(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_1(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_2(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_3(ctx, &mut w, &noise_source_active_mask);
        if !noise_source_active[0] {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_0_psd_e2951: f64 = 1.0;
            let noise_0_psd_e2952: f64 = (noise_0_psd_e2951 * w[260]);
            let psd = noise_0_psd_e2952;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[1] {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_1_psd_e2953: f64 = 1.0;
            let noise_1_psd_e2954: f64 = (noise_1_psd_e2953 * w[259]);
            let psd = noise_1_psd_e2954;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = Some(params.p41);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 271], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_1_0_e194: f64 = (11.7 * 8.8541879239442e-12);
            w[199] = noise_metadata_schedule_1_0_e194;
        }
        if (active[0] & 0x3) != 0 {
            w[157] = 0.0;
        }
        if (active[0] & 0x3) != 0 {
            w[6] = 0.0;
        }
        if (active[0] & 0x3) != 0 {
            w[175] = 0.0;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_6_0_e201: f64 = (w[199] / params.p13);
            w[31] = noise_metadata_schedule_6_0_e201;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_7_0_e204: f64 = (w[31] * params.p14);
            let noise_metadata_schedule_7_0_e205: f64 = (noise_metadata_schedule_7_0_e204).sqrt();
            w[34] = noise_metadata_schedule_7_0_e205;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_8_0_e208: f64 = (w[34] * params.p25);
            w[35] = noise_metadata_schedule_8_0_e208;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_9_0_e211: f64 = (3.0 * w[31]);
            let noise_metadata_schedule_9_0_e213: f64 = (noise_metadata_schedule_9_0_e211 * params.p28);
            w[32] = noise_metadata_schedule_9_0_e213;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_10_0_e216: f64 = (w[31] * params.p29);
            w[33] = noise_metadata_schedule_10_0_e216;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_12_0_e223: f64 = (w[199] * params.p22);
            let noise_metadata_schedule_12_0_e224: f64 = (params.p13 / noise_metadata_schedule_12_0_e223);
            w[37] = noise_metadata_schedule_12_0_e224;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_13_0_e227: f64 = (params.p30 + params.p30);
            let noise_metadata_schedule_13_0_e229: f64 = (noise_metadata_schedule_13_0_e227 / params.p13);
            w[182] = noise_metadata_schedule_13_0_e229;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_14_0_e235,) = {
    if (params.p0 > 0.0) {
        (0.5,)
    } else {
        (0.3333333333333,)
    }
};
            w[39] = noise_metadata_schedule_14_0_e235;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_15_0_e238: f64 = (-1e21);
            let noise_metadata_schedule_15_0_e239: f64 = (-noise_metadata_schedule_15_0_e238);
            let noise_metadata_schedule_15_0_e240: f64 = if params.p3 == noise_metadata_schedule_15_0_e239 { 1.0 } else { 0.0 };
            w[238] = noise_metadata_schedule_15_0_e240;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_16_0_e246,) = {
    if (w[238] != 0.0) {
        let noise_metadata_schedule_16_0_e242: f64 = ctx.temperature();
        let noise_metadata_schedule_16_0_e244: f64 = (noise_metadata_schedule_16_0_e242 + params.p2);
        (noise_metadata_schedule_16_0_e244,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_16_0_e246;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_17_0_e253,) = {
    if (w[238] == 0.0) {
        let noise_metadata_schedule_17_0_e251: f64 = (params.p3 + 273.15);
        (noise_metadata_schedule_17_0_e251,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_17_0_e253;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_18_0_e256: f64 = (-1e21);
            let noise_metadata_schedule_18_0_e257: f64 = (-noise_metadata_schedule_18_0_e256);
            let noise_metadata_schedule_18_0_e258: f64 = if params.p4 == noise_metadata_schedule_18_0_e257 { 1.0 } else { 0.0 };
            w[239] = noise_metadata_schedule_18_0_e258;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_19_0_e264,) = {
    if (w[239] != 0.0) {
        let noise_metadata_schedule_19_0_e262: f64 = (25.0 + 273.15);
        (noise_metadata_schedule_19_0_e262,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_19_0_e264;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_20_0_e271,) = {
    if (w[239] == 0.0) {
        let noise_metadata_schedule_20_0_e269: f64 = (params.p4 + 273.15);
        (noise_metadata_schedule_20_0_e269,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_20_0_e271;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_21_0_e273: f64 = (w[49] * THERMAL_VOLTAGE_PER_K);
            w[17] = noise_metadata_schedule_21_0_e273;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_22_0_e276: f64 = (0.1 * w[17]);
            w[25] = noise_metadata_schedule_22_0_e276;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_23_0_e279: f64 = (1.0 / w[17]);
            w[24] = noise_metadata_schedule_23_0_e279;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_24_0_e282: f64 = (w[17] + w[17]);
            w[26] = noise_metadata_schedule_24_0_e282;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_25_0_e285: f64 = (w[26] + w[26]);
            w[27] = noise_metadata_schedule_25_0_e285;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_26_0_e288: f64 = (w[17] * w[17]);
            w[28] = noise_metadata_schedule_26_0_e288;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_27_0_e291: f64 = (w[28] + w[28]);
            w[29] = noise_metadata_schedule_27_0_e291;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_28_0_e294: f64 = (16.0 * w[28]);
            w[30] = noise_metadata_schedule_28_0_e294;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_29_0_e298: f64 = (0.000702 * w[49]);
            let noise_metadata_schedule_29_0_e300: f64 = (noise_metadata_schedule_29_0_e298 * w[49]);
            let noise_metadata_schedule_29_0_e303: f64 = (w[49] + 1108.0);
            let noise_metadata_schedule_29_0_e304: f64 = (noise_metadata_schedule_29_0_e300 / noise_metadata_schedule_29_0_e303);
            let noise_metadata_schedule_29_0_e305: f64 = (1.16 - noise_metadata_schedule_29_0_e304);
            w[51] = noise_metadata_schedule_29_0_e305;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_30_0_e309: f64 = (0.000702 * w[55]);
            let noise_metadata_schedule_30_0_e311: f64 = (noise_metadata_schedule_30_0_e309 * w[55]);
            let noise_metadata_schedule_30_0_e314: f64 = (w[55] + 1108.0);
            let noise_metadata_schedule_30_0_e315: f64 = (noise_metadata_schedule_30_0_e311 / noise_metadata_schedule_30_0_e314);
            let noise_metadata_schedule_30_0_e316: f64 = (1.16 - noise_metadata_schedule_30_0_e315);
            w[52] = noise_metadata_schedule_30_0_e316;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_31_0_e319: f64 = (w[49] - w[55]);
            w[53] = noise_metadata_schedule_31_0_e319;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_32_0_e322: f64 = (w[49] / w[55]);
            w[54] = noise_metadata_schedule_32_0_e322;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_33_0_e326: f64 = (params.p16 * w[53]);
            let noise_metadata_schedule_33_0_e327: f64 = (params.p15 - noise_metadata_schedule_33_0_e326);
            w[56] = noise_metadata_schedule_33_0_e327;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_34_0_e331: f64 = (w[54]).powf(params.p20);
            let noise_metadata_schedule_34_0_e332: f64 = (params.p19 * noise_metadata_schedule_34_0_e331);
            w[58] = noise_metadata_schedule_34_0_e332;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_35_0_e336: f64 = (w[54]).powf(params.p24);
            let noise_metadata_schedule_35_0_e337: f64 = (params.p23 * noise_metadata_schedule_35_0_e336);
            w[59] = noise_metadata_schedule_35_0_e337;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_37_0_e347: f64 = (params.p18 * w[54]);
            let noise_metadata_schedule_37_0_e350: f64 = (3.0 * w[17]);
            let noise_metadata_schedule_37_0_e352: f64 = (w[54]).ln();
            let noise_metadata_schedule_37_0_e353: f64 = (noise_metadata_schedule_37_0_e350 * noise_metadata_schedule_37_0_e352);
            let noise_metadata_schedule_37_0_e354: f64 = (noise_metadata_schedule_37_0_e347 - noise_metadata_schedule_37_0_e353);
            let noise_metadata_schedule_37_0_e357: f64 = (w[52] * w[54]);
            let noise_metadata_schedule_37_0_e358: f64 = (noise_metadata_schedule_37_0_e354 - noise_metadata_schedule_37_0_e357);
            let noise_metadata_schedule_37_0_e360: f64 = (noise_metadata_schedule_37_0_e358 + w[51]);
            w[61] = noise_metadata_schedule_37_0_e360;
        }
        if (active[0] & 0x3) != 0 {
            w[0] = 0.2;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_39_0_e364: f64 = (w[61] - w[0]);
            w[1] = noise_metadata_schedule_39_0_e364;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_40_0_e369: f64 = (w[1] * w[1]);
            let noise_metadata_schedule_40_0_e372: f64 = (w[17] * w[17]);
            let noise_metadata_schedule_40_0_e373: f64 = (noise_metadata_schedule_40_0_e369 + noise_metadata_schedule_40_0_e372);
            let noise_metadata_schedule_40_0_e374: f64 = (noise_metadata_schedule_40_0_e373).sqrt();
            let noise_metadata_schedule_40_0_e375: f64 = (w[1] + noise_metadata_schedule_40_0_e374);
            let noise_metadata_schedule_40_0_e376: f64 = (0.5 * noise_metadata_schedule_40_0_e375);
            let noise_metadata_schedule_40_0_e378: f64 = (noise_metadata_schedule_40_0_e376 + w[0]);
            w[61] = noise_metadata_schedule_40_0_e378;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_41_0_e380: f64 = (w[61]).sqrt();
            w[71] = noise_metadata_schedule_41_0_e380;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_42_0_e383: f64 = (1.0 / w[59]);
            w[40] = noise_metadata_schedule_42_0_e383;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_43_0_e386: f64 = (w[34] * w[59]);
            w[41] = noise_metadata_schedule_43_0_e386;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_46_0_e395: f64 = (params.p5 + params.p26);
            w[191] = noise_metadata_schedule_46_0_e395;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_47_0_e398: f64 = (params.p6 + params.p27);
            w[192] = noise_metadata_schedule_47_0_e398;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_48_0_e401: f64 = (w[59] * w[191]);
            w[158] = noise_metadata_schedule_48_0_e401;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_49_0_e405: f64 = (0.5 * w[158]);
            let noise_metadata_schedule_49_0_e407: f64 = (noise_metadata_schedule_49_0_e405 * w[24]);
            let noise_metadata_schedule_49_0_e408: f64 = (noise_metadata_schedule_49_0_e407).ln();
            let noise_metadata_schedule_49_0_e410: f64 = (noise_metadata_schedule_49_0_e408 - 0.6);
            let noise_metadata_schedule_49_0_e411: f64 = (w[17] * noise_metadata_schedule_49_0_e410);
            w[173] = noise_metadata_schedule_49_0_e411;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_50_0_e415: f64 = (w[192] * w[191]);
            let noise_metadata_schedule_50_0_e416: f64 = (noise_metadata_schedule_50_0_e415).sqrt();
            let noise_metadata_schedule_50_0_e417: f64 = (1.0 / noise_metadata_schedule_50_0_e416);
            w[48] = noise_metadata_schedule_50_0_e417;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_51_0_e420: f64 = if params.p0 > 0.0 { 1.0 } else { 0.0 };
            w[240] = noise_metadata_schedule_51_0_e420;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_52_0_e435,) = {
    if (w[240] != 0.0) {
        let (noise_metadata_schedule_52_0_e433,) = {
            if (params.p38 != 1e-6) {
                let noise_metadata_schedule_52_0_e428: f64 = (params.p38 - 1e-6);
                let noise_metadata_schedule_52_0_e429: f64 = (w[48] * noise_metadata_schedule_52_0_e428);
                let noise_metadata_schedule_52_0_e431: f64 = (noise_metadata_schedule_52_0_e429 + w[56]);
                (noise_metadata_schedule_52_0_e431,)
            } else {
                (w[56],)
            }
        };
        (noise_metadata_schedule_52_0_e433,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_52_0_e435;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_53_0_e452,) = {
    if (w[240] == 0.0) {
        let (noise_metadata_schedule_53_0_e450,) = {
            if (params.p38 != 1e-6) {
                let noise_metadata_schedule_53_0_e444: f64 = (1e-6 - params.p38);
                let noise_metadata_schedule_53_0_e445: f64 = (w[48] * noise_metadata_schedule_53_0_e444);
                let noise_metadata_schedule_53_0_e447: f64 = (noise_metadata_schedule_53_0_e445 - w[56]);
                (noise_metadata_schedule_53_0_e447,)
            } else {
                let noise_metadata_schedule_53_0_e449: f64 = (-w[56]);
                (noise_metadata_schedule_53_0_e449,)
            }
        };
        (noise_metadata_schedule_53_0_e450,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_53_0_e452;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_54_0_e467,) = {
    if (params.p39 != 1e-6) {
        let noise_metadata_schedule_54_0_e461: f64 = (params.p39 - 1e-6);
        let noise_metadata_schedule_54_0_e463: f64 = (noise_metadata_schedule_54_0_e461 * w[48]);
        let noise_metadata_schedule_54_0_e464: f64 = (1.0 + noise_metadata_schedule_54_0_e463);
        let noise_metadata_schedule_54_0_e465: f64 = (w[58] * noise_metadata_schedule_54_0_e464);
        (noise_metadata_schedule_54_0_e465,)
    } else {
        (w[58],)
    }
};
            let noise_metadata_schedule_54_0_e468: f64 = (w[192] * noise_metadata_schedule_54_0_e467);
            w[50] = noise_metadata_schedule_54_0_e468;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_55_0_e480,) = {
    if (params.p40 != 1e-6) {
        let noise_metadata_schedule_55_0_e475: f64 = (params.p40 - 1e-6);
        let noise_metadata_schedule_55_0_e477: f64 = (noise_metadata_schedule_55_0_e475 * w[48]);
        let noise_metadata_schedule_55_0_e478: f64 = (params.p17 + noise_metadata_schedule_55_0_e477);
        (noise_metadata_schedule_55_0_e478,)
    } else {
        (params.p17,)
    }
};
            w[62] = noise_metadata_schedule_55_0_e480;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_56_0_e483: f64 = (w[62] * w[71]);
            w[153] = noise_metadata_schedule_56_0_e483;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_57_0_e486: f64 = if w[182] == 0.0 { 1.0 } else { 0.0 };
            w[241] = noise_metadata_schedule_57_0_e486;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_58_0_e490,) = {
    if (w[241] != 0.0) {
        (0.0,)
    } else {
        (w[183],)
    }
};
            w[183] = noise_metadata_schedule_58_0_e490;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_59_0_e503,) = {
    if (w[241] == 0.0) {
        let noise_metadata_schedule_59_0_e497: f64 = (params.p31 * params.p8);
        let noise_metadata_schedule_59_0_e498: f64 = (w[191] / noise_metadata_schedule_59_0_e497);
        let noise_metadata_schedule_59_0_e500: f64 = (noise_metadata_schedule_59_0_e498 - 0.1);
        let noise_metadata_schedule_59_0_e501: f64 = (0.28 * noise_metadata_schedule_59_0_e500);
        (noise_metadata_schedule_59_0_e501,)
    } else {
        (w[184],)
    }
};
            w[184] = noise_metadata_schedule_59_0_e503;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_60_0_e521,) = {
    if (w[241] == 0.0) {
        let noise_metadata_schedule_60_0_e512: f64 = (w[184] * w[184]);
        let noise_metadata_schedule_60_0_e514: f64 = (noise_metadata_schedule_60_0_e512 + 0.001936);
        let noise_metadata_schedule_60_0_e515: f64 = (noise_metadata_schedule_60_0_e514).sqrt();
        let noise_metadata_schedule_60_0_e516: f64 = (w[184] + noise_metadata_schedule_60_0_e515);
        let noise_metadata_schedule_60_0_e517: f64 = (0.5 * noise_metadata_schedule_60_0_e516);
        let noise_metadata_schedule_60_0_e518: f64 = (1.0 + noise_metadata_schedule_60_0_e517);
        let noise_metadata_schedule_60_0_e519: f64 = (1.0 / noise_metadata_schedule_60_0_e518);
        (noise_metadata_schedule_60_0_e519,)
    } else {
        (w[242],)
    }
};
            w[242] = noise_metadata_schedule_60_0_e521;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_61_0_e530,) = {
    if (w[241] == 0.0) {
        let noise_metadata_schedule_61_0_e526: f64 = (w[182] * w[242]);
        let noise_metadata_schedule_61_0_e528: f64 = (noise_metadata_schedule_61_0_e526 * w[242]);
        (noise_metadata_schedule_61_0_e528,)
    } else {
        (w[183],)
    }
};
            w[183] = noise_metadata_schedule_61_0_e530;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_62_0_e533: f64 = (params.p0 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[3])));
            w[145] = noise_metadata_schedule_62_0_e533;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_63_0_e536: f64 = (params.p0 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[3])));
            w[147] = noise_metadata_schedule_63_0_e536;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_64_0_e539: f64 = (params.p0 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[3])));
            w[146] = noise_metadata_schedule_64_0_e539;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_65_0_e542: f64 = (w[146] - w[147]);
            let noise_metadata_schedule_65_0_e544: f64 = if noise_metadata_schedule_65_0_e542 < 0.0 { 1.0 } else { 0.0 };
            w[243] = noise_metadata_schedule_65_0_e544;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_67_0_e553,) = {
    if (w[243] != 0.0) {
        (w[147],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_67_0_e553;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_68_0_e557,) = {
    if (w[243] != 0.0) {
        (w[146],)
    } else {
        (w[147],)
    }
};
            w[147] = noise_metadata_schedule_68_0_e557;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_69_0_e561,) = {
    if (w[243] != 0.0) {
        (w[38],)
    } else {
        (w[146],)
    }
};
            w[146] = noise_metadata_schedule_69_0_e561;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_71_0_e569: f64 = (w[145] - w[57]);
            let noise_metadata_schedule_71_0_e571: f64 = (noise_metadata_schedule_71_0_e569 - w[183]);
            let noise_metadata_schedule_71_0_e573: f64 = (noise_metadata_schedule_71_0_e571 + w[61]);
            let noise_metadata_schedule_71_0_e575: f64 = (noise_metadata_schedule_71_0_e573 + w[153]);
            w[143] = noise_metadata_schedule_71_0_e575;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_72_0_e578: f64 = (w[143] * w[143]);
            let noise_metadata_schedule_72_0_e581: f64 = (2.0 * w[30]);
            let noise_metadata_schedule_72_0_e582: f64 = (noise_metadata_schedule_72_0_e578 + noise_metadata_schedule_72_0_e581);
            let noise_metadata_schedule_72_0_e583: f64 = (noise_metadata_schedule_72_0_e582).sqrt();
            w[144] = noise_metadata_schedule_72_0_e583;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_73_0_e587: f64 = (w[143] + w[144]);
            let noise_metadata_schedule_73_0_e588: f64 = (0.5 * noise_metadata_schedule_73_0_e587);
            w[3] = noise_metadata_schedule_73_0_e588;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_74_0_e591: f64 = (w[61] + w[147]);
            w[70] = noise_metadata_schedule_74_0_e591;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_75_0_e594: f64 = (w[70] * w[70]);
            let noise_metadata_schedule_75_0_e596: f64 = (noise_metadata_schedule_75_0_e594 + w[30]);
            let noise_metadata_schedule_75_0_e597: f64 = (noise_metadata_schedule_75_0_e596).sqrt();
            w[76] = noise_metadata_schedule_75_0_e597;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_76_0_e601: f64 = (w[70] + w[76]);
            let noise_metadata_schedule_76_0_e602: f64 = (0.5 * noise_metadata_schedule_76_0_e601);
            let noise_metadata_schedule_76_0_e603: f64 = (noise_metadata_schedule_76_0_e602).sqrt();
            w[74] = noise_metadata_schedule_76_0_e603;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_77_0_e606: f64 = (w[61] + w[146]);
            w[69] = noise_metadata_schedule_77_0_e606;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_78_0_e609: f64 = (w[69] * w[69]);
            let noise_metadata_schedule_78_0_e611: f64 = (noise_metadata_schedule_78_0_e609 + w[30]);
            let noise_metadata_schedule_78_0_e612: f64 = (noise_metadata_schedule_78_0_e611).sqrt();
            w[75] = noise_metadata_schedule_78_0_e612;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_79_0_e616: f64 = (w[69] + w[75]);
            let noise_metadata_schedule_79_0_e617: f64 = (0.5 * noise_metadata_schedule_79_0_e616);
            let noise_metadata_schedule_79_0_e618: f64 = (noise_metadata_schedule_79_0_e617).sqrt();
            w[73] = noise_metadata_schedule_79_0_e618;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_80_0_e621: f64 = (w[32] * params.p7);
            let noise_metadata_schedule_80_0_e623: f64 = (noise_metadata_schedule_80_0_e621 / w[192]);
            w[45] = noise_metadata_schedule_80_0_e623;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_81_0_e626: f64 = (w[33] * params.p8);
            let noise_metadata_schedule_81_0_e628: f64 = (noise_metadata_schedule_81_0_e626 / w[191]);
            w[46] = noise_metadata_schedule_81_0_e628;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_82_0_e632: f64 = (0.25 * w[62]);
            let noise_metadata_schedule_82_0_e634: f64 = (noise_metadata_schedule_82_0_e632 * w[62]);
            let noise_metadata_schedule_82_0_e635: f64 = (w[3] + noise_metadata_schedule_82_0_e634);
            let noise_metadata_schedule_82_0_e636: f64 = (noise_metadata_schedule_82_0_e635).sqrt();
            w[67] = noise_metadata_schedule_82_0_e636;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_83_0_e639: f64 = (w[3] - w[61]);
            let noise_metadata_schedule_83_0_e644: f64 = (0.5 * w[62]);
            let noise_metadata_schedule_83_0_e645: f64 = (w[67] - noise_metadata_schedule_83_0_e644);
            let noise_metadata_schedule_83_0_e646: f64 = (w[62] * noise_metadata_schedule_83_0_e645);
            let noise_metadata_schedule_83_0_e647: f64 = (noise_metadata_schedule_83_0_e639 - noise_metadata_schedule_83_0_e646);
            w[68] = noise_metadata_schedule_83_0_e647;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_84_0_e650: f64 = (w[68] + w[61]);
            let noise_metadata_schedule_84_0_e652: f64 = (noise_metadata_schedule_84_0_e650 + w[25]);
            let noise_metadata_schedule_84_0_e653: f64 = (noise_metadata_schedule_84_0_e652).sqrt();
            w[174] = noise_metadata_schedule_84_0_e653;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 271], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_85_0_e658: f64 = (w[74] + w[73]);
            let noise_metadata_schedule_85_0_e659: f64 = (w[46] * noise_metadata_schedule_85_0_e658);
            let noise_metadata_schedule_85_0_e660: f64 = (w[62] - noise_metadata_schedule_85_0_e659);
            let noise_metadata_schedule_85_0_e663: f64 = (w[45] * w[174]);
            let noise_metadata_schedule_85_0_e664: f64 = (noise_metadata_schedule_85_0_e660 + noise_metadata_schedule_85_0_e663);
            w[64] = noise_metadata_schedule_85_0_e664;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_86_0_e667: f64 = (w[64] * w[64]);
            let noise_metadata_schedule_86_0_e669: f64 = (noise_metadata_schedule_86_0_e667 + w[25]);
            let noise_metadata_schedule_86_0_e670: f64 = (noise_metadata_schedule_86_0_e669).sqrt();
            w[65] = noise_metadata_schedule_86_0_e670;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_87_0_e674: f64 = (w[64] + w[65]);
            let noise_metadata_schedule_87_0_e675: f64 = (0.5 * noise_metadata_schedule_87_0_e674);
            w[4] = noise_metadata_schedule_87_0_e675;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_88_0_e679: f64 = (0.25 * w[4]);
            let noise_metadata_schedule_88_0_e681: f64 = (noise_metadata_schedule_88_0_e679 * w[4]);
            let noise_metadata_schedule_88_0_e682: f64 = (w[3] + noise_metadata_schedule_88_0_e681);
            let noise_metadata_schedule_88_0_e683: f64 = (noise_metadata_schedule_88_0_e682).sqrt();
            w[66] = noise_metadata_schedule_88_0_e683;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_89_0_e686: f64 = (w[3] - w[61]);
            let noise_metadata_schedule_89_0_e691: f64 = (0.5 * w[4]);
            let noise_metadata_schedule_89_0_e692: f64 = (w[66] - noise_metadata_schedule_89_0_e691);
            let noise_metadata_schedule_89_0_e693: f64 = (w[4] * noise_metadata_schedule_89_0_e692);
            let noise_metadata_schedule_89_0_e694: f64 = (noise_metadata_schedule_89_0_e686 - noise_metadata_schedule_89_0_e693);
            w[5] = noise_metadata_schedule_89_0_e694;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_90_0_e697: f64 = (w[5] - w[147]);
            let noise_metadata_schedule_90_0_e699: f64 = (noise_metadata_schedule_90_0_e697 * w[24]);
            w[0] = noise_metadata_schedule_90_0_e699;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_91_0_e702: f64 = (-0.35);
            let noise_metadata_schedule_91_0_e703: f64 = if w[0] > noise_metadata_schedule_91_0_e702 { 1.0 } else { 0.0 };
            w[244] = noise_metadata_schedule_91_0_e703;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_92_0_e716,) = {
    if (w[244] != 0.0) {
        let noise_metadata_schedule_92_0_e708: f64 = (1.3 + w[0]);
        let noise_metadata_schedule_92_0_e711: f64 = (w[0] + 1.6);
        let noise_metadata_schedule_92_0_e712: f64 = (noise_metadata_schedule_92_0_e711).ln();
        let noise_metadata_schedule_92_0_e713: f64 = (noise_metadata_schedule_92_0_e708 - noise_metadata_schedule_92_0_e712);
        let noise_metadata_schedule_92_0_e714: f64 = (2.0 / noise_metadata_schedule_92_0_e713);
        (noise_metadata_schedule_92_0_e714,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_92_0_e716;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_93_0_e729,) = {
    if (w[244] != 0.0) {
        let noise_metadata_schedule_93_0_e720: f64 = (2.0 + w[196]);
        let noise_metadata_schedule_93_0_e723: f64 = (1.0 + w[0]);
        let noise_metadata_schedule_93_0_e725: f64 = (w[196]).ln();
        let noise_metadata_schedule_93_0_e726: f64 = (noise_metadata_schedule_93_0_e723 + noise_metadata_schedule_93_0_e725);
        let noise_metadata_schedule_93_0_e727: f64 = (noise_metadata_schedule_93_0_e720 / noise_metadata_schedule_93_0_e726);
        (noise_metadata_schedule_93_0_e727,)
    } else {
        (w[197],)
    }
};
            w[197] = noise_metadata_schedule_93_0_e729;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_94_0_e742,) = {
    if (w[244] != 0.0) {
        let noise_metadata_schedule_94_0_e733: f64 = (1.0 + w[0]);
        let noise_metadata_schedule_94_0_e735: f64 = (w[197]).ln();
        let noise_metadata_schedule_94_0_e736: f64 = (noise_metadata_schedule_94_0_e733 + noise_metadata_schedule_94_0_e735);
        let noise_metadata_schedule_94_0_e739: f64 = (2.0 + w[197]);
        let noise_metadata_schedule_94_0_e740: f64 = (noise_metadata_schedule_94_0_e736 / noise_metadata_schedule_94_0_e739);
        (noise_metadata_schedule_94_0_e740,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_94_0_e742;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_95_0_e745: f64 = (-15.0);
            let noise_metadata_schedule_95_0_e746: f64 = if w[0] > noise_metadata_schedule_95_0_e745 { 1.0 } else { 0.0 };
            w[245] = noise_metadata_schedule_95_0_e746;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_96_0_e757,) = {
    if ((w[244] == 0.0) && (w[245] != 0.0)) {
        let noise_metadata_schedule_96_0_e753: f64 = (-w[0]);
        let noise_metadata_schedule_96_0_e754: f64 = (noise_metadata_schedule_96_0_e753).exp();
        let noise_metadata_schedule_96_0_e755: f64 = (1.55 + noise_metadata_schedule_96_0_e754);
        (noise_metadata_schedule_96_0_e755,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_96_0_e757;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_97_0_e773,) = {
    if ((w[244] == 0.0) && (w[245] != 0.0)) {
        let noise_metadata_schedule_97_0_e764: f64 = (2.0 + w[196]);
        let noise_metadata_schedule_97_0_e767: f64 = (1.0 + w[0]);
        let noise_metadata_schedule_97_0_e769: f64 = (w[196]).ln();
        let noise_metadata_schedule_97_0_e770: f64 = (noise_metadata_schedule_97_0_e767 + noise_metadata_schedule_97_0_e769);
        let noise_metadata_schedule_97_0_e771: f64 = (noise_metadata_schedule_97_0_e764 / noise_metadata_schedule_97_0_e770);
        (noise_metadata_schedule_97_0_e771,)
    } else {
        (w[197],)
    }
};
            w[197] = noise_metadata_schedule_97_0_e773;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_98_0_e789,) = {
    if ((w[244] == 0.0) && (w[245] != 0.0)) {
        let noise_metadata_schedule_98_0_e780: f64 = (1.0 + w[0]);
        let noise_metadata_schedule_98_0_e782: f64 = (w[197]).ln();
        let noise_metadata_schedule_98_0_e783: f64 = (noise_metadata_schedule_98_0_e780 + noise_metadata_schedule_98_0_e782);
        let noise_metadata_schedule_98_0_e786: f64 = (2.0 + w[197]);
        let noise_metadata_schedule_98_0_e787: f64 = (noise_metadata_schedule_98_0_e783 / noise_metadata_schedule_98_0_e786);
        (noise_metadata_schedule_98_0_e787,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_98_0_e789;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_99_0_e792: f64 = (-23.0);
            let noise_metadata_schedule_99_0_e793: f64 = if w[0] > noise_metadata_schedule_99_0_e792 { 1.0 } else { 0.0 };
            w[246] = noise_metadata_schedule_99_0_e793;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_100_0_e809,) = {
    if (((w[244] == 0.0) && (w[245] == 0.0)) && (w[246] != 0.0)) {
        let noise_metadata_schedule_100_0_e804: f64 = (-w[0]);
        let noise_metadata_schedule_100_0_e805: f64 = (noise_metadata_schedule_100_0_e804).exp();
        let noise_metadata_schedule_100_0_e806: f64 = (2.0 + noise_metadata_schedule_100_0_e805);
        let noise_metadata_schedule_100_0_e807: f64 = (1.0 / noise_metadata_schedule_100_0_e806);
        (noise_metadata_schedule_100_0_e807,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_100_0_e809;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_101_0_e823,) = {
    if (((w[244] == 0.0) && (w[245] == 0.0)) && (w[246] == 0.0)) {
        let noise_metadata_schedule_101_0_e819: f64 = (w[0]).exp();
        let noise_metadata_schedule_101_0_e821: f64 = (noise_metadata_schedule_101_0_e819 + 1e-64);
        (noise_metadata_schedule_101_0_e821,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_101_0_e823;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_102_0_e827: f64 = (1.0 + w[195]);
            let noise_metadata_schedule_102_0_e828: f64 = (w[195] * noise_metadata_schedule_102_0_e827);
            w[7] = noise_metadata_schedule_102_0_e828;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_103_0_e830: f64 = (w[7]).sqrt();
            w[87] = noise_metadata_schedule_103_0_e830;
        }
        if (active[0] & 0x2) != 0 {
            w[90] = w[195];
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_105_0_e834: f64 = (w[17] / w[158]);
            w[160] = noise_metadata_schedule_105_0_e834;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_106_0_e838: f64 = (w[87] * w[160]);
            let noise_metadata_schedule_106_0_e839: f64 = (0.25 + noise_metadata_schedule_106_0_e838);
            let noise_metadata_schedule_106_0_e840: f64 = (noise_metadata_schedule_106_0_e839).sqrt();
            w[80] = noise_metadata_schedule_106_0_e840;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_107_0_e844: f64 = (w[80] - 0.5);
            let noise_metadata_schedule_107_0_e845: f64 = (w[158] * noise_metadata_schedule_107_0_e844);
            w[10] = noise_metadata_schedule_107_0_e845;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_108_0_e849: f64 = (w[146] - w[147]);
            let noise_metadata_schedule_108_0_e850: f64 = (0.5 * noise_metadata_schedule_108_0_e849);
            w[77] = noise_metadata_schedule_108_0_e850;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_109_0_e856: f64 = (w[10] * w[24]);
            let noise_metadata_schedule_109_0_e857: f64 = (w[87] - noise_metadata_schedule_109_0_e856);
            let noise_metadata_schedule_109_0_e858: f64 = (params.p25 * noise_metadata_schedule_109_0_e857);
            let noise_metadata_schedule_109_0_e860: f64 = (noise_metadata_schedule_109_0_e858 + 0.015625);
            let noise_metadata_schedule_109_0_e861: f64 = (w[30] * noise_metadata_schedule_109_0_e860);
            w[78] = noise_metadata_schedule_109_0_e861;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_110_0_e864: f64 = (w[10] * w[10]);
            let noise_metadata_schedule_110_0_e866: f64 = (noise_metadata_schedule_110_0_e864 + w[78]);
            let noise_metadata_schedule_110_0_e867: f64 = (noise_metadata_schedule_110_0_e866).sqrt();
            w[81] = noise_metadata_schedule_110_0_e867;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_111_0_e870: f64 = (w[77] - w[10]);
            let noise_metadata_schedule_111_0_e873: f64 = (w[77] - w[10]);
            let noise_metadata_schedule_111_0_e874: f64 = (noise_metadata_schedule_111_0_e870 * noise_metadata_schedule_111_0_e873);
            let noise_metadata_schedule_111_0_e876: f64 = (noise_metadata_schedule_111_0_e874 + w[78]);
            let noise_metadata_schedule_111_0_e877: f64 = (noise_metadata_schedule_111_0_e876).sqrt();
            w[82] = noise_metadata_schedule_111_0_e877;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_112_0_e880: f64 = (w[81] - w[82]);
            w[79] = noise_metadata_schedule_112_0_e880;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_113_0_e885: f64 = (w[7]).ln();
            let noise_metadata_schedule_113_0_e886: f64 = (0.75 * noise_metadata_schedule_113_0_e885);
            let noise_metadata_schedule_113_0_e887: f64 = (w[87] - noise_metadata_schedule_113_0_e886);
            let noise_metadata_schedule_113_0_e889: f64 = (noise_metadata_schedule_113_0_e887 * w[160]);
            let noise_metadata_schedule_113_0_e890: f64 = (0.25 + noise_metadata_schedule_113_0_e889);
            let noise_metadata_schedule_113_0_e891: f64 = (noise_metadata_schedule_113_0_e890).sqrt();
            w[83] = noise_metadata_schedule_113_0_e891;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_114_0_e895: f64 = (w[83] - 0.5);
            let noise_metadata_schedule_114_0_e896: f64 = (w[158] * noise_metadata_schedule_114_0_e895);
            let noise_metadata_schedule_114_0_e898: f64 = (noise_metadata_schedule_114_0_e896 + w[173]);
            w[11] = noise_metadata_schedule_114_0_e898;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_115_0_e901: f64 = (w[77] - w[11]);
            w[159] = noise_metadata_schedule_115_0_e901;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_116_0_e904: f64 = (w[11] * w[11]);
            let noise_metadata_schedule_116_0_e906: f64 = (noise_metadata_schedule_116_0_e904 + w[78]);
            let noise_metadata_schedule_116_0_e907: f64 = (noise_metadata_schedule_116_0_e906).sqrt();
            w[84] = noise_metadata_schedule_116_0_e907;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_117_0_e910: f64 = (w[159] * w[159]);
            let noise_metadata_schedule_117_0_e912: f64 = (noise_metadata_schedule_117_0_e910 + w[78]);
            let noise_metadata_schedule_117_0_e913: f64 = (noise_metadata_schedule_117_0_e912).sqrt();
            w[85] = noise_metadata_schedule_117_0_e913;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_118_0_e916: f64 = (w[5] - w[77]);
            let noise_metadata_schedule_118_0_e918: f64 = (noise_metadata_schedule_118_0_e916 - w[147]);
            let noise_metadata_schedule_118_0_e920: f64 = (noise_metadata_schedule_118_0_e918 - w[84]);
            let noise_metadata_schedule_118_0_e922: f64 = (noise_metadata_schedule_118_0_e920 + w[85]);
            let noise_metadata_schedule_118_0_e924: f64 = (noise_metadata_schedule_118_0_e922 * w[24]);
            w[0] = noise_metadata_schedule_118_0_e924;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_119_0_e927: f64 = (-0.35);
            let noise_metadata_schedule_119_0_e928: f64 = if w[0] > noise_metadata_schedule_119_0_e927 { 1.0 } else { 0.0 };
            w[247] = noise_metadata_schedule_119_0_e928;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_120_0_e941,) = {
    if (w[247] != 0.0) {
        let noise_metadata_schedule_120_0_e933: f64 = (1.3 + w[0]);
        let noise_metadata_schedule_120_0_e936: f64 = (w[0] + 1.6);
        let noise_metadata_schedule_120_0_e937: f64 = (noise_metadata_schedule_120_0_e936).ln();
        let noise_metadata_schedule_120_0_e938: f64 = (noise_metadata_schedule_120_0_e933 - noise_metadata_schedule_120_0_e937);
        let noise_metadata_schedule_120_0_e939: f64 = (2.0 / noise_metadata_schedule_120_0_e938);
        (noise_metadata_schedule_120_0_e939,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_120_0_e941;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_121_0_e954,) = {
    if (w[247] != 0.0) {
        let noise_metadata_schedule_121_0_e945: f64 = (2.0 + w[196]);
        let noise_metadata_schedule_121_0_e948: f64 = (1.0 + w[0]);
        let noise_metadata_schedule_121_0_e950: f64 = (w[196]).ln();
        let noise_metadata_schedule_121_0_e951: f64 = (noise_metadata_schedule_121_0_e948 + noise_metadata_schedule_121_0_e950);
        let noise_metadata_schedule_121_0_e952: f64 = (noise_metadata_schedule_121_0_e945 / noise_metadata_schedule_121_0_e951);
        (noise_metadata_schedule_121_0_e952,)
    } else {
        (w[197],)
    }
};
            w[197] = noise_metadata_schedule_121_0_e954;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_122_0_e967,) = {
    if (w[247] != 0.0) {
        let noise_metadata_schedule_122_0_e958: f64 = (1.0 + w[0]);
        let noise_metadata_schedule_122_0_e960: f64 = (w[197]).ln();
        let noise_metadata_schedule_122_0_e961: f64 = (noise_metadata_schedule_122_0_e958 + noise_metadata_schedule_122_0_e960);
        let noise_metadata_schedule_122_0_e964: f64 = (2.0 + w[197]);
        let noise_metadata_schedule_122_0_e965: f64 = (noise_metadata_schedule_122_0_e961 / noise_metadata_schedule_122_0_e964);
        (noise_metadata_schedule_122_0_e965,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_122_0_e967;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_123_0_e970: f64 = (-15.0);
            let noise_metadata_schedule_123_0_e971: f64 = if w[0] > noise_metadata_schedule_123_0_e970 { 1.0 } else { 0.0 };
            w[248] = noise_metadata_schedule_123_0_e971;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_124_0_e982,) = {
    if ((w[247] == 0.0) && (w[248] != 0.0)) {
        let noise_metadata_schedule_124_0_e978: f64 = (-w[0]);
        let noise_metadata_schedule_124_0_e979: f64 = (noise_metadata_schedule_124_0_e978).exp();
        let noise_metadata_schedule_124_0_e980: f64 = (1.55 + noise_metadata_schedule_124_0_e979);
        (noise_metadata_schedule_124_0_e980,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_124_0_e982;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_125_0_e998,) = {
    if ((w[247] == 0.0) && (w[248] != 0.0)) {
        let noise_metadata_schedule_125_0_e989: f64 = (2.0 + w[196]);
        let noise_metadata_schedule_125_0_e992: f64 = (1.0 + w[0]);
        let noise_metadata_schedule_125_0_e994: f64 = (w[196]).ln();
        let noise_metadata_schedule_125_0_e995: f64 = (noise_metadata_schedule_125_0_e992 + noise_metadata_schedule_125_0_e994);
        let noise_metadata_schedule_125_0_e996: f64 = (noise_metadata_schedule_125_0_e989 / noise_metadata_schedule_125_0_e995);
        (noise_metadata_schedule_125_0_e996,)
    } else {
        (w[197],)
    }
};
            w[197] = noise_metadata_schedule_125_0_e998;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_126_0_e1014,) = {
    if ((w[247] == 0.0) && (w[248] != 0.0)) {
        let noise_metadata_schedule_126_0_e1005: f64 = (1.0 + w[0]);
        let noise_metadata_schedule_126_0_e1007: f64 = (w[197]).ln();
        let noise_metadata_schedule_126_0_e1008: f64 = (noise_metadata_schedule_126_0_e1005 + noise_metadata_schedule_126_0_e1007);
        let noise_metadata_schedule_126_0_e1011: f64 = (2.0 + w[197]);
        let noise_metadata_schedule_126_0_e1012: f64 = (noise_metadata_schedule_126_0_e1008 / noise_metadata_schedule_126_0_e1011);
        (noise_metadata_schedule_126_0_e1012,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_126_0_e1014;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_127_0_e1017: f64 = (-23.0);
            let noise_metadata_schedule_127_0_e1018: f64 = if w[0] > noise_metadata_schedule_127_0_e1017 { 1.0 } else { 0.0 };
            w[249] = noise_metadata_schedule_127_0_e1018;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_128_0_e1034,) = {
    if (((w[247] == 0.0) && (w[248] == 0.0)) && (w[249] != 0.0)) {
        let noise_metadata_schedule_128_0_e1029: f64 = (-w[0]);
        let noise_metadata_schedule_128_0_e1030: f64 = (noise_metadata_schedule_128_0_e1029).exp();
        let noise_metadata_schedule_128_0_e1031: f64 = (2.0 + noise_metadata_schedule_128_0_e1030);
        let noise_metadata_schedule_128_0_e1032: f64 = (1.0 / noise_metadata_schedule_128_0_e1031);
        (noise_metadata_schedule_128_0_e1032,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_128_0_e1034;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_129_0_e1048,) = {
    if (((w[247] == 0.0) && (w[248] == 0.0)) && (w[249] == 0.0)) {
        let noise_metadata_schedule_129_0_e1044: f64 = (w[0]).exp();
        let noise_metadata_schedule_129_0_e1046: f64 = (noise_metadata_schedule_129_0_e1044 + 1e-64);
        (noise_metadata_schedule_129_0_e1046,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_129_0_e1048;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_130_0_e1052: f64 = (1.0 + w[195]);
            let noise_metadata_schedule_130_0_e1053: f64 = (w[195] * noise_metadata_schedule_130_0_e1052);
            w[9] = noise_metadata_schedule_130_0_e1053;
        }
        if (active[0] & 0x2) != 0 {
            w[92] = w[195];
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_133_0_e1061: f64 = (w[77] - w[79]);
            let noise_metadata_schedule_133_0_e1063: f64 = (noise_metadata_schedule_133_0_e1061 / w[41]);
            let noise_metadata_schedule_133_0_e1064: f64 = (1.0 + noise_metadata_schedule_133_0_e1063);
            let noise_metadata_schedule_133_0_e1065: f64 = (noise_metadata_schedule_133_0_e1064).ln();
            let noise_metadata_schedule_133_0_e1066: f64 = (w[35] * noise_metadata_schedule_133_0_e1065);
            w[12] = noise_metadata_schedule_133_0_e1066;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_134_0_e1069: f64 = (w[191] - w[12]);
            let noise_metadata_schedule_134_0_e1072: f64 = (w[77] + w[79]);
            let noise_metadata_schedule_134_0_e1074: f64 = (noise_metadata_schedule_134_0_e1072 * w[40]);
            let noise_metadata_schedule_134_0_e1075: f64 = (noise_metadata_schedule_134_0_e1069 + noise_metadata_schedule_134_0_e1074);
            w[155] = noise_metadata_schedule_134_0_e1075;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_135_0_e1078: f64 = (0.1 * w[191]);
            w[154] = noise_metadata_schedule_135_0_e1078;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_136_0_e1081: f64 = (w[155] * w[155]);
            let noise_metadata_schedule_136_0_e1084: f64 = (w[154] * w[154]);
            let noise_metadata_schedule_136_0_e1085: f64 = (noise_metadata_schedule_136_0_e1081 + noise_metadata_schedule_136_0_e1084);
            let noise_metadata_schedule_136_0_e1086: f64 = (noise_metadata_schedule_136_0_e1085).sqrt();
            w[63] = noise_metadata_schedule_136_0_e1086;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_137_0_e1090: f64 = (w[155] + w[63]);
            let noise_metadata_schedule_137_0_e1091: f64 = (0.5 * noise_metadata_schedule_137_0_e1090);
            w[13] = noise_metadata_schedule_137_0_e1091;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_138_0_e1094: f64 = (w[5] - w[146]);
            let noise_metadata_schedule_138_0_e1096: f64 = (noise_metadata_schedule_138_0_e1094 * w[24]);
            w[0] = noise_metadata_schedule_138_0_e1096;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_139_0_e1099: f64 = (-0.35);
            let noise_metadata_schedule_139_0_e1100: f64 = if w[0] > noise_metadata_schedule_139_0_e1099 { 1.0 } else { 0.0 };
            w[250] = noise_metadata_schedule_139_0_e1100;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_140_0_e1113,) = {
    if (w[250] != 0.0) {
        let noise_metadata_schedule_140_0_e1105: f64 = (1.3 + w[0]);
        let noise_metadata_schedule_140_0_e1108: f64 = (w[0] + 1.6);
        let noise_metadata_schedule_140_0_e1109: f64 = (noise_metadata_schedule_140_0_e1108).ln();
        let noise_metadata_schedule_140_0_e1110: f64 = (noise_metadata_schedule_140_0_e1105 - noise_metadata_schedule_140_0_e1109);
        let noise_metadata_schedule_140_0_e1111: f64 = (2.0 / noise_metadata_schedule_140_0_e1110);
        (noise_metadata_schedule_140_0_e1111,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_140_0_e1113;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_141_0_e1126,) = {
    if (w[250] != 0.0) {
        let noise_metadata_schedule_141_0_e1117: f64 = (2.0 + w[196]);
        let noise_metadata_schedule_141_0_e1120: f64 = (1.0 + w[0]);
        let noise_metadata_schedule_141_0_e1122: f64 = (w[196]).ln();
        let noise_metadata_schedule_141_0_e1123: f64 = (noise_metadata_schedule_141_0_e1120 + noise_metadata_schedule_141_0_e1122);
        let noise_metadata_schedule_141_0_e1124: f64 = (noise_metadata_schedule_141_0_e1117 / noise_metadata_schedule_141_0_e1123);
        (noise_metadata_schedule_141_0_e1124,)
    } else {
        (w[197],)
    }
};
            w[197] = noise_metadata_schedule_141_0_e1126;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_142_0_e1139,) = {
    if (w[250] != 0.0) {
        let noise_metadata_schedule_142_0_e1130: f64 = (1.0 + w[0]);
        let noise_metadata_schedule_142_0_e1132: f64 = (w[197]).ln();
        let noise_metadata_schedule_142_0_e1133: f64 = (noise_metadata_schedule_142_0_e1130 + noise_metadata_schedule_142_0_e1132);
        let noise_metadata_schedule_142_0_e1136: f64 = (2.0 + w[197]);
        let noise_metadata_schedule_142_0_e1137: f64 = (noise_metadata_schedule_142_0_e1133 / noise_metadata_schedule_142_0_e1136);
        (noise_metadata_schedule_142_0_e1137,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_142_0_e1139;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_143_0_e1142: f64 = (-15.0);
            let noise_metadata_schedule_143_0_e1143: f64 = if w[0] > noise_metadata_schedule_143_0_e1142 { 1.0 } else { 0.0 };
            w[251] = noise_metadata_schedule_143_0_e1143;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_144_0_e1154,) = {
    if ((w[250] == 0.0) && (w[251] != 0.0)) {
        let noise_metadata_schedule_144_0_e1150: f64 = (-w[0]);
        let noise_metadata_schedule_144_0_e1151: f64 = (noise_metadata_schedule_144_0_e1150).exp();
        let noise_metadata_schedule_144_0_e1152: f64 = (1.55 + noise_metadata_schedule_144_0_e1151);
        (noise_metadata_schedule_144_0_e1152,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_144_0_e1154;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 271], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_145_0_e1170,) = {
    if ((w[250] == 0.0) && (w[251] != 0.0)) {
        let noise_metadata_schedule_145_0_e1161: f64 = (2.0 + w[196]);
        let noise_metadata_schedule_145_0_e1164: f64 = (1.0 + w[0]);
        let noise_metadata_schedule_145_0_e1166: f64 = (w[196]).ln();
        let noise_metadata_schedule_145_0_e1167: f64 = (noise_metadata_schedule_145_0_e1164 + noise_metadata_schedule_145_0_e1166);
        let noise_metadata_schedule_145_0_e1168: f64 = (noise_metadata_schedule_145_0_e1161 / noise_metadata_schedule_145_0_e1167);
        (noise_metadata_schedule_145_0_e1168,)
    } else {
        (w[197],)
    }
};
            w[197] = noise_metadata_schedule_145_0_e1170;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_146_0_e1186,) = {
    if ((w[250] == 0.0) && (w[251] != 0.0)) {
        let noise_metadata_schedule_146_0_e1177: f64 = (1.0 + w[0]);
        let noise_metadata_schedule_146_0_e1179: f64 = (w[197]).ln();
        let noise_metadata_schedule_146_0_e1180: f64 = (noise_metadata_schedule_146_0_e1177 + noise_metadata_schedule_146_0_e1179);
        let noise_metadata_schedule_146_0_e1183: f64 = (2.0 + w[197]);
        let noise_metadata_schedule_146_0_e1184: f64 = (noise_metadata_schedule_146_0_e1180 / noise_metadata_schedule_146_0_e1183);
        (noise_metadata_schedule_146_0_e1184,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_146_0_e1186;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_147_0_e1189: f64 = (-23.0);
            let noise_metadata_schedule_147_0_e1190: f64 = if w[0] > noise_metadata_schedule_147_0_e1189 { 1.0 } else { 0.0 };
            w[252] = noise_metadata_schedule_147_0_e1190;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_148_0_e1206,) = {
    if (((w[250] == 0.0) && (w[251] == 0.0)) && (w[252] != 0.0)) {
        let noise_metadata_schedule_148_0_e1201: f64 = (-w[0]);
        let noise_metadata_schedule_148_0_e1202: f64 = (noise_metadata_schedule_148_0_e1201).exp();
        let noise_metadata_schedule_148_0_e1203: f64 = (2.0 + noise_metadata_schedule_148_0_e1202);
        let noise_metadata_schedule_148_0_e1204: f64 = (1.0 / noise_metadata_schedule_148_0_e1203);
        (noise_metadata_schedule_148_0_e1204,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_148_0_e1206;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_149_0_e1220,) = {
    if (((w[250] == 0.0) && (w[251] == 0.0)) && (w[252] == 0.0)) {
        let noise_metadata_schedule_149_0_e1216: f64 = (w[0]).exp();
        let noise_metadata_schedule_149_0_e1218: f64 = (noise_metadata_schedule_149_0_e1216 + 1e-64);
        (noise_metadata_schedule_149_0_e1218,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_149_0_e1220;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_150_0_e1224: f64 = (1.0 + w[195]);
            let noise_metadata_schedule_150_0_e1225: f64 = (w[195] * noise_metadata_schedule_150_0_e1224);
            w[8] = noise_metadata_schedule_150_0_e1225;
        }
        if (active[0] & 0x2) != 0 {
            w[91] = w[195];
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_153_0_e1231: f64 = (0.25 + w[7]);
            w[95] = noise_metadata_schedule_153_0_e1231;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_154_0_e1234: f64 = (0.25 + w[8]);
            w[96] = noise_metadata_schedule_154_0_e1234;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_155_0_e1236: f64 = (w[95]).sqrt();
            w[93] = noise_metadata_schedule_155_0_e1236;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_156_0_e1238: f64 = (w[96]).sqrt();
            w[94] = noise_metadata_schedule_156_0_e1238;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_157_0_e1241: f64 = (w[93] + w[94]);
            let noise_metadata_schedule_157_0_e1244: f64 = (w[93] + w[94]);
            let noise_metadata_schedule_157_0_e1245: f64 = (noise_metadata_schedule_157_0_e1241 * noise_metadata_schedule_157_0_e1244);
            w[99] = noise_metadata_schedule_157_0_e1245;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_158_0_e1248: f64 = (w[5] + w[61]);
            let noise_metadata_schedule_158_0_e1250: f64 = (noise_metadata_schedule_158_0_e1248 + 1e-6);
            w[107] = noise_metadata_schedule_158_0_e1250;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_159_0_e1253: f64 = (w[107]).sqrt();
            let noise_metadata_schedule_159_0_e1254: f64 = (2.0 * noise_metadata_schedule_159_0_e1253);
            w[108] = noise_metadata_schedule_159_0_e1254;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_160_0_e1257: f64 = (w[62] / w[108]);
            w[111] = noise_metadata_schedule_160_0_e1257;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_161_0_e1261: f64 = (w[108] + w[62]);
            let noise_metadata_schedule_161_0_e1262: f64 = (w[62] / noise_metadata_schedule_161_0_e1261);
            w[112] = noise_metadata_schedule_161_0_e1262;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_162_0_e1265: f64 = (1.0 + w[111]);
            let noise_metadata_schedule_162_0_e1266: f64 = (-noise_metadata_schedule_162_0_e1265);
            let noise_metadata_schedule_162_0_e1268: f64 = (noise_metadata_schedule_162_0_e1266 * w[17]);
            let noise_metadata_schedule_162_0_e1271: f64 = (0.66666666 + 0.66666666);
            let noise_metadata_schedule_162_0_e1275: f64 = (w[94] * w[93]);
            let noise_metadata_schedule_162_0_e1276: f64 = (w[96] + noise_metadata_schedule_162_0_e1275);
            let noise_metadata_schedule_162_0_e1278: f64 = (noise_metadata_schedule_162_0_e1276 + w[95]);
            let noise_metadata_schedule_162_0_e1279: f64 = (noise_metadata_schedule_162_0_e1271 * noise_metadata_schedule_162_0_e1278);
            let noise_metadata_schedule_162_0_e1282: f64 = (w[93] + w[94]);
            let noise_metadata_schedule_162_0_e1283: f64 = (noise_metadata_schedule_162_0_e1279 / noise_metadata_schedule_162_0_e1282);
            let noise_metadata_schedule_162_0_e1285: f64 = (noise_metadata_schedule_162_0_e1283 - 1.0);
            let noise_metadata_schedule_162_0_e1286: f64 = (noise_metadata_schedule_162_0_e1268 * noise_metadata_schedule_162_0_e1285);
            w[100] = noise_metadata_schedule_162_0_e1286;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_163_0_e1288: f64 = (-0.5);
            let noise_metadata_schedule_163_0_e1290: f64 = (noise_metadata_schedule_163_0_e1288 * w[62]);
            let noise_metadata_schedule_163_0_e1292: f64 = (noise_metadata_schedule_163_0_e1290 * w[108]);
            let noise_metadata_schedule_163_0_e1295: f64 = (w[112] * w[100]);
            let noise_metadata_schedule_163_0_e1296: f64 = (noise_metadata_schedule_163_0_e1292 - noise_metadata_schedule_163_0_e1295);
            w[101] = noise_metadata_schedule_163_0_e1296;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_164_0_e1299: f64 = if params.p22 == 0.0 { 1.0 } else { 0.0 };
            w[253] = noise_metadata_schedule_164_0_e1299;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_165_0_e1308,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_165_0_e1303: f64 = (w[5] * w[5]);
        let noise_metadata_schedule_165_0_e1305: f64 = (noise_metadata_schedule_165_0_e1303 + w[29]);
        let noise_metadata_schedule_165_0_e1306: f64 = (noise_metadata_schedule_165_0_e1305).sqrt();
        (noise_metadata_schedule_165_0_e1306,)
    } else {
        (w[175],)
    }
};
            w[175] = noise_metadata_schedule_165_0_e1308;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_166_0_e1316,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_166_0_e1313: f64 = (w[5] + w[175]);
        let noise_metadata_schedule_166_0_e1314: f64 = (0.5 * noise_metadata_schedule_166_0_e1313);
        (noise_metadata_schedule_166_0_e1314,)
    } else {
        (w[6],)
    }
};
            w[6] = noise_metadata_schedule_166_0_e1316;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_167_0_e1324,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_167_0_e1321: f64 = (params.p21 * w[6]);
        let noise_metadata_schedule_167_0_e1322: f64 = (1.0 + noise_metadata_schedule_167_0_e1321);
        (noise_metadata_schedule_167_0_e1322,)
    } else {
        (w[157],)
    }
};
            w[157] = noise_metadata_schedule_167_0_e1324;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_168_0_e1332,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_168_0_e1329: f64 = (w[13] * w[157]);
        let noise_metadata_schedule_168_0_e1330: f64 = (w[50] / noise_metadata_schedule_168_0_e1329);
        (noise_metadata_schedule_168_0_e1330,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_168_0_e1332;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_169_0_e1336: f64 = (w[39] * w[100]);
            let noise_metadata_schedule_169_0_e1337: f64 = (w[101] + noise_metadata_schedule_169_0_e1336);
            let noise_metadata_schedule_169_0_e1339: f64 = if noise_metadata_schedule_169_0_e1337 > 0.0 { 1.0 } else { 0.0 };
            w[254] = noise_metadata_schedule_169_0_e1339;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_170_0_e1354,) = {
    if ((w[253] == 0.0) && (w[254] != 0.0)) {
        let noise_metadata_schedule_170_0_e1349: f64 = (w[39] * w[100]);
        let noise_metadata_schedule_170_0_e1350: f64 = (w[101] + noise_metadata_schedule_170_0_e1349);
        let noise_metadata_schedule_170_0_e1351: f64 = (w[37] * noise_metadata_schedule_170_0_e1350);
        let noise_metadata_schedule_170_0_e1352: f64 = (1.0 + noise_metadata_schedule_170_0_e1351);
        (noise_metadata_schedule_170_0_e1352,)
    } else {
        (w[47],)
    }
};
            w[47] = noise_metadata_schedule_170_0_e1354;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_171_0_e1370,) = {
    if ((w[253] == 0.0) && (w[254] == 0.0)) {
        let noise_metadata_schedule_171_0_e1365: f64 = (w[39] * w[100]);
        let noise_metadata_schedule_171_0_e1366: f64 = (w[101] + noise_metadata_schedule_171_0_e1365);
        let noise_metadata_schedule_171_0_e1367: f64 = (w[37] * noise_metadata_schedule_171_0_e1366);
        let noise_metadata_schedule_171_0_e1368: f64 = (1.0 - noise_metadata_schedule_171_0_e1367);
        (noise_metadata_schedule_171_0_e1368,)
    } else {
        (w[47],)
    }
};
            w[47] = noise_metadata_schedule_171_0_e1370;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_172_0_e1379,) = {
    if (w[253] == 0.0) {
        let noise_metadata_schedule_172_0_e1376: f64 = (w[37] * w[153]);
        let noise_metadata_schedule_172_0_e1377: f64 = (1.0 + noise_metadata_schedule_172_0_e1376);
        (noise_metadata_schedule_172_0_e1377,)
    } else {
        (w[156],)
    }
};
            w[156] = noise_metadata_schedule_172_0_e1379;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_173_0_e1390,) = {
    if (w[253] == 0.0) {
        let noise_metadata_schedule_173_0_e1384: f64 = (w[50] * w[156]);
        let noise_metadata_schedule_173_0_e1387: f64 = (w[13] * w[47]);
        let noise_metadata_schedule_173_0_e1388: f64 = (noise_metadata_schedule_173_0_e1384 / noise_metadata_schedule_173_0_e1387);
        (noise_metadata_schedule_173_0_e1388,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_173_0_e1390;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_174_0_e1393: f64 = (w[61] + w[5]);
            let noise_metadata_schedule_174_0_e1395: f64 = (noise_metadata_schedule_174_0_e1393 + w[27]);
            let noise_metadata_schedule_174_0_e1396: f64 = (noise_metadata_schedule_174_0_e1395).sqrt();
            w[72] = noise_metadata_schedule_174_0_e1396;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_175_0_e1401: f64 = (2.0 * w[72]);
            let noise_metadata_schedule_175_0_e1402: f64 = (w[62] / noise_metadata_schedule_175_0_e1401);
            let noise_metadata_schedule_175_0_e1403: f64 = (1.0 + noise_metadata_schedule_175_0_e1402);
            w[15] = noise_metadata_schedule_175_0_e1403;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_176_0_e1406: f64 = (w[7] - w[9]);
            w[86] = noise_metadata_schedule_176_0_e1406;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_177_0_e1409: f64 = (w[29] * w[15]);
            let noise_metadata_schedule_177_0_e1411: f64 = (noise_metadata_schedule_177_0_e1409 * w[14]);
            w[16] = noise_metadata_schedule_177_0_e1411;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_181_0_e1431: f64 = (w[100]).abs();
            let noise_metadata_schedule_181_0_e1432: f64 = (w[14] * noise_metadata_schedule_181_0_e1431);
            w[152] = noise_metadata_schedule_181_0_e1432;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_182_0_e1436: f64 = (w[65] + w[65]);
            let noise_metadata_schedule_182_0_e1437: f64 = (w[4] / noise_metadata_schedule_182_0_e1436);
            w[0] = noise_metadata_schedule_182_0_e1437;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_183_0_e1440: f64 = (w[3] / w[144]);
            w[1] = noise_metadata_schedule_183_0_e1440;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_186_0_e1459: f64 = (w[45] * w[0]);
            let noise_metadata_schedule_186_0_e1463: f64 = (0.5 * w[62]);
            let noise_metadata_schedule_186_0_e1464: f64 = (w[67] - noise_metadata_schedule_186_0_e1463);
            let noise_metadata_schedule_186_0_e1465: f64 = (noise_metadata_schedule_186_0_e1459 * noise_metadata_schedule_186_0_e1464);
            let noise_metadata_schedule_186_0_e1468: f64 = (w[67] * w[174]);
            let noise_metadata_schedule_186_0_e1469: f64 = (noise_metadata_schedule_186_0_e1465 / noise_metadata_schedule_186_0_e1468);
            let noise_metadata_schedule_186_0_e1471: f64 = (noise_metadata_schedule_186_0_e1469 * w[1]);
            w[162] = noise_metadata_schedule_186_0_e1471;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_187_0_e1474: f64 = (w[5] + w[61]);
            let noise_metadata_schedule_187_0_e1476: f64 = (noise_metadata_schedule_187_0_e1474 / w[66]);
            w[2] = noise_metadata_schedule_187_0_e1476;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_190_0_e1486: f64 = (-w[2]);
            let noise_metadata_schedule_190_0_e1488: f64 = (noise_metadata_schedule_190_0_e1486 * w[162]);
            let noise_metadata_schedule_190_0_e1493: f64 = (w[66] + w[66]);
            let noise_metadata_schedule_190_0_e1494: f64 = (w[4] / noise_metadata_schedule_190_0_e1493);
            let noise_metadata_schedule_190_0_e1495: f64 = (1.0 - noise_metadata_schedule_190_0_e1494);
            let noise_metadata_schedule_190_0_e1497: f64 = (noise_metadata_schedule_190_0_e1495 * w[1]);
            let noise_metadata_schedule_190_0_e1498: f64 = (noise_metadata_schedule_190_0_e1488 + noise_metadata_schedule_190_0_e1497);
            w[114] = noise_metadata_schedule_190_0_e1498;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_191_0_e1501: f64 = (w[90] * w[24]);
            w[0] = noise_metadata_schedule_191_0_e1501;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_194_0_e1512: f64 = (w[0] * w[114]);
            w[118] = noise_metadata_schedule_194_0_e1512;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_195_0_e1516: f64 = (4.0 * w[80]);
            let noise_metadata_schedule_195_0_e1518: f64 = (noise_metadata_schedule_195_0_e1516 * w[87]);
            let noise_metadata_schedule_195_0_e1519: f64 = (w[17] / noise_metadata_schedule_195_0_e1518);
            w[0] = noise_metadata_schedule_195_0_e1519;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_198_0_e1528: f64 = (w[0] * w[118]);
            w[123] = noise_metadata_schedule_198_0_e1528;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_199_0_e1531: f64 = (w[27] + w[27]);
            let noise_metadata_schedule_199_0_e1533: f64 = (noise_metadata_schedule_199_0_e1531 * params.p25);
            w[0] = noise_metadata_schedule_199_0_e1533;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_200_0_e1537: f64 = (w[87] + w[87]);
            let noise_metadata_schedule_200_0_e1538: f64 = (w[17] / noise_metadata_schedule_200_0_e1537);
            w[1] = noise_metadata_schedule_200_0_e1538;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_203_0_e1556: f64 = (w[118] * w[1]);
            let noise_metadata_schedule_203_0_e1558: f64 = (noise_metadata_schedule_203_0_e1556 - w[123]);
            let noise_metadata_schedule_203_0_e1559: f64 = (w[0] * noise_metadata_schedule_203_0_e1558);
            w[126] = noise_metadata_schedule_203_0_e1559;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_204_0_e1562: f64 = (1.0 / w[81]);
            w[0] = noise_metadata_schedule_204_0_e1562;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_205_0_e1565: f64 = (1.0 / w[82]);
            w[1] = noise_metadata_schedule_205_0_e1565;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_206_0_e1568: f64 = (w[77] - w[10]);
            w[2] = noise_metadata_schedule_206_0_e1568;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_209_0_e1606: f64 = (w[10] * w[123]);
            let noise_metadata_schedule_209_0_e1608: f64 = (noise_metadata_schedule_209_0_e1606 + w[126]);
            let noise_metadata_schedule_209_0_e1610: f64 = (noise_metadata_schedule_209_0_e1608 * w[0]);
            let noise_metadata_schedule_209_0_e1613: f64 = (-w[123]);
            let noise_metadata_schedule_209_0_e1614: f64 = (w[2] * noise_metadata_schedule_209_0_e1613);
            let noise_metadata_schedule_209_0_e1616: f64 = (noise_metadata_schedule_209_0_e1614 + w[126]);
            let noise_metadata_schedule_209_0_e1618: f64 = (noise_metadata_schedule_209_0_e1616 * w[1]);
            let noise_metadata_schedule_209_0_e1619: f64 = (noise_metadata_schedule_209_0_e1610 - noise_metadata_schedule_209_0_e1618);
            w[129] = noise_metadata_schedule_209_0_e1619;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_210_0_e1623: f64 = (w[87] - 1.5);
            let noise_metadata_schedule_210_0_e1624: f64 = (w[17] * noise_metadata_schedule_210_0_e1623);
            let noise_metadata_schedule_210_0_e1627: f64 = (4.0 * w[83]);
            let noise_metadata_schedule_210_0_e1629: f64 = (noise_metadata_schedule_210_0_e1627 * w[7]);
            let noise_metadata_schedule_210_0_e1630: f64 = (noise_metadata_schedule_210_0_e1624 / noise_metadata_schedule_210_0_e1629);
            w[0] = noise_metadata_schedule_210_0_e1630;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_213_0_e1639: f64 = (w[0] * w[118]);
            w[132] = noise_metadata_schedule_213_0_e1639;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_214_0_e1642: f64 = (w[92] * w[24]);
            w[0] = noise_metadata_schedule_214_0_e1642;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_215_0_e1645: f64 = (1.0 / w[84]);
            w[1] = noise_metadata_schedule_215_0_e1645;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_216_0_e1648: f64 = (1.0 / w[85]);
            w[2] = noise_metadata_schedule_216_0_e1648;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_219_0_e1700: f64 = (w[11] * w[132]);
            let noise_metadata_schedule_219_0_e1702: f64 = (noise_metadata_schedule_219_0_e1700 + w[126]);
            let noise_metadata_schedule_219_0_e1704: f64 = (noise_metadata_schedule_219_0_e1702 * w[1]);
            let noise_metadata_schedule_219_0_e1705: f64 = (w[114] - noise_metadata_schedule_219_0_e1704);
            let noise_metadata_schedule_219_0_e1708: f64 = (-w[132]);
            let noise_metadata_schedule_219_0_e1709: f64 = (w[159] * noise_metadata_schedule_219_0_e1708);
            let noise_metadata_schedule_219_0_e1711: f64 = (noise_metadata_schedule_219_0_e1709 + w[126]);
            let noise_metadata_schedule_219_0_e1713: f64 = (noise_metadata_schedule_219_0_e1711 * w[2]);
            let noise_metadata_schedule_219_0_e1714: f64 = (noise_metadata_schedule_219_0_e1705 + noise_metadata_schedule_219_0_e1713);
            let noise_metadata_schedule_219_0_e1715: f64 = (w[0] * noise_metadata_schedule_219_0_e1714);
            w[135] = noise_metadata_schedule_219_0_e1715;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_220_0_e1719: f64 = (w[41] + w[77]);
            let noise_metadata_schedule_220_0_e1721: f64 = (noise_metadata_schedule_220_0_e1719 - w[79]);
            let noise_metadata_schedule_220_0_e1722: f64 = (w[35] / noise_metadata_schedule_220_0_e1721);
            w[0] = noise_metadata_schedule_220_0_e1722;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_223_0_e1735: f64 = (-w[0]);
            let noise_metadata_schedule_223_0_e1737: f64 = (noise_metadata_schedule_223_0_e1735 * w[129]);
            w[168] = noise_metadata_schedule_223_0_e1737;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_224_0_e1740: f64 = (1.0 / w[63]);
            w[0] = noise_metadata_schedule_224_0_e1740;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_227_0_e1764: f64 = (-w[168]);
            let noise_metadata_schedule_227_0_e1767: f64 = (w[129] * w[40]);
            let noise_metadata_schedule_227_0_e1768: f64 = (noise_metadata_schedule_227_0_e1764 + noise_metadata_schedule_227_0_e1767);
            let noise_metadata_schedule_227_0_e1769: f64 = (w[0] * noise_metadata_schedule_227_0_e1768);
            w[138] = noise_metadata_schedule_227_0_e1769;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_228_0_e1772: f64 = (w[91] * w[24]);
            w[0] = noise_metadata_schedule_228_0_e1772;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_231_0_e1783: f64 = (w[0] * w[114]);
            w[121] = noise_metadata_schedule_231_0_e1783;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_232_0_e1786: f64 = (1.0 + w[111]);
            let noise_metadata_schedule_232_0_e1787: f64 = (-noise_metadata_schedule_232_0_e1786);
            let noise_metadata_schedule_232_0_e1789: f64 = (noise_metadata_schedule_232_0_e1787 * w[17]);
            let noise_metadata_schedule_232_0_e1791: f64 = (noise_metadata_schedule_232_0_e1789 * 0.66666666);
            let noise_metadata_schedule_232_0_e1793: f64 = (noise_metadata_schedule_232_0_e1791 / w[99]);
            w[0] = noise_metadata_schedule_232_0_e1793;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_233_0_e1798: f64 = (2.0 * w[94]);
            let noise_metadata_schedule_233_0_e1799: f64 = (w[93] + noise_metadata_schedule_233_0_e1798);
            let noise_metadata_schedule_233_0_e1800: f64 = (w[0] * noise_metadata_schedule_233_0_e1799);
            w[1] = noise_metadata_schedule_233_0_e1800;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_234_0_e1805: f64 = (2.0 * w[93]);
            let noise_metadata_schedule_234_0_e1806: f64 = (w[94] + noise_metadata_schedule_234_0_e1805);
            let noise_metadata_schedule_234_0_e1807: f64 = (w[0] * noise_metadata_schedule_234_0_e1806);
            w[2] = noise_metadata_schedule_234_0_e1807;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_235_0_e1809: f64 = (-w[111]);
            let noise_metadata_schedule_235_0_e1811: f64 = (noise_metadata_schedule_235_0_e1809 * w[100]);
            let noise_metadata_schedule_235_0_e1814: f64 = (2.0 + w[111]);
            let noise_metadata_schedule_235_0_e1816: f64 = (noise_metadata_schedule_235_0_e1814 + w[111]);
            let noise_metadata_schedule_235_0_e1818: f64 = (noise_metadata_schedule_235_0_e1816 * w[107]);
            let noise_metadata_schedule_235_0_e1819: f64 = (noise_metadata_schedule_235_0_e1811 / noise_metadata_schedule_235_0_e1818);
            w[0] = noise_metadata_schedule_235_0_e1819;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_238_0_e1844: f64 = (w[0] * w[114]);
            let noise_metadata_schedule_238_0_e1847: f64 = (w[1] * w[118]);
            let noise_metadata_schedule_238_0_e1848: f64 = (noise_metadata_schedule_238_0_e1844 + noise_metadata_schedule_238_0_e1847);
            let noise_metadata_schedule_238_0_e1851: f64 = (w[2] * w[121]);
            let noise_metadata_schedule_238_0_e1852: f64 = (noise_metadata_schedule_238_0_e1848 + noise_metadata_schedule_238_0_e1851);
            w[187] = noise_metadata_schedule_238_0_e1852;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_239_0_e1855: f64 = (1.0 + w[111]);
            let noise_metadata_schedule_239_0_e1860: f64 = (1.0 + w[111]);
            let noise_metadata_schedule_239_0_e1861: f64 = (2.0 * noise_metadata_schedule_239_0_e1860);
            let noise_metadata_schedule_239_0_e1863: f64 = (noise_metadata_schedule_239_0_e1861 * w[107]);
            let noise_metadata_schedule_239_0_e1864: f64 = (w[100] / noise_metadata_schedule_239_0_e1863);
            let noise_metadata_schedule_239_0_e1865: f64 = (noise_metadata_schedule_239_0_e1855 - noise_metadata_schedule_239_0_e1864);
            w[0] = noise_metadata_schedule_239_0_e1865;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_242_0_e1883: f64 = (-w[112]);
            let noise_metadata_schedule_242_0_e1886: f64 = (w[0] * w[114]);
            let noise_metadata_schedule_242_0_e1888: f64 = (noise_metadata_schedule_242_0_e1886 + w[187]);
            let noise_metadata_schedule_242_0_e1889: f64 = (noise_metadata_schedule_242_0_e1883 * noise_metadata_schedule_242_0_e1888);
            w[190] = noise_metadata_schedule_242_0_e1889;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_243_0_e1892: f64 = if params.p22 == 0.0 { 1.0 } else { 0.0 };
            w[255] = noise_metadata_schedule_243_0_e1892;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_244_0_e1902,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_244_0_e1896: f64 = (params.p21 * w[6]);
        let noise_metadata_schedule_244_0_e1899: f64 = (w[157] * w[175]);
        let noise_metadata_schedule_244_0_e1900: f64 = (noise_metadata_schedule_244_0_e1896 / noise_metadata_schedule_244_0_e1899);
        (noise_metadata_schedule_244_0_e1900,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_244_0_e1902;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_247_0_e1920,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_247_0_e1918: f64 = (w[0] * w[114]);
        (noise_metadata_schedule_247_0_e1918,)
    } else {
        (w[165],)
    }
};
            w[165] = noise_metadata_schedule_247_0_e1920;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_3(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 271], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_250_0_e1941,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_250_0_e1937: f64 = (-w[138]);
        let noise_metadata_schedule_250_0_e1939: f64 = (noise_metadata_schedule_250_0_e1937 - w[165]);
        (noise_metadata_schedule_250_0_e1939,)
    } else {
        (w[141],)
    }
};
            w[141] = noise_metadata_schedule_250_0_e1941;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_251_0_e1948,) = {
    if (w[255] == 0.0) {
        let noise_metadata_schedule_251_0_e1946: f64 = (w[37] / w[47]);
        (noise_metadata_schedule_251_0_e1946,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_251_0_e1948;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_254_0_e1990,) = {
    if (w[255] == 0.0) {
        let noise_metadata_schedule_254_0_e1980: f64 = (-w[138]);
        let noise_metadata_schedule_254_0_e1985: f64 = (w[39] * w[187]);
        let noise_metadata_schedule_254_0_e1986: f64 = (w[190] + noise_metadata_schedule_254_0_e1985);
        let noise_metadata_schedule_254_0_e1987: f64 = (w[0] * noise_metadata_schedule_254_0_e1986);
        let noise_metadata_schedule_254_0_e1988: f64 = (noise_metadata_schedule_254_0_e1980 + noise_metadata_schedule_254_0_e1987);
        (noise_metadata_schedule_254_0_e1988,)
    } else {
        (w[141],)
    }
};
            w[141] = noise_metadata_schedule_254_0_e1990;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_255_0_e1992: f64 = (-w[62]);
            let noise_metadata_schedule_255_0_e1995: f64 = (4.0 * w[15]);
            let noise_metadata_schedule_255_0_e1997: f64 = (noise_metadata_schedule_255_0_e1995 * w[72]);
            let noise_metadata_schedule_255_0_e2000: f64 = (w[61] + w[5]);
            let noise_metadata_schedule_255_0_e2002: f64 = (noise_metadata_schedule_255_0_e2000 + w[27]);
            let noise_metadata_schedule_255_0_e2003: f64 = (noise_metadata_schedule_255_0_e1997 * noise_metadata_schedule_255_0_e2002);
            let noise_metadata_schedule_255_0_e2004: f64 = (noise_metadata_schedule_255_0_e1992 / noise_metadata_schedule_255_0_e2003);
            w[0] = noise_metadata_schedule_255_0_e2004;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_258_0_e2013: f64 = (w[0] * w[114]);
            w[171] = noise_metadata_schedule_258_0_e2013;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_261_0_e2040: f64 = (w[171] + w[141]);
            let noise_metadata_schedule_261_0_e2042: f64 = (noise_metadata_schedule_261_0_e2040 * w[86]);
            let noise_metadata_schedule_261_0_e2044: f64 = (noise_metadata_schedule_261_0_e2042 + w[118]);
            let noise_metadata_schedule_261_0_e2046: f64 = (noise_metadata_schedule_261_0_e2044 - w[135]);
            let noise_metadata_schedule_261_0_e2047: f64 = (w[16] * noise_metadata_schedule_261_0_e2046);
            w[18] = noise_metadata_schedule_261_0_e2047;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_294_0_e2289,) = {
    if (params.p1 != 0.0) {
        let noise_metadata_schedule_294_0_e2283: f64 = (4.0 * 1.3806226e-23);
        let noise_metadata_schedule_294_0_e2285: f64 = (noise_metadata_schedule_294_0_e2283 * w[49]);
        let noise_metadata_schedule_294_0_e2287: f64 = (noise_metadata_schedule_294_0_e2285 * w[152]);
        (noise_metadata_schedule_294_0_e2287,)
    } else {
        (w[260],)
    }
};
            w[260] = noise_metadata_schedule_294_0_e2289;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_295_0_e2305,) = {
    if (params.p1 != 0.0) {
        let noise_metadata_schedule_295_0_e2293: f64 = (params.p42 * w[18]);
        let noise_metadata_schedule_295_0_e2295: f64 = (noise_metadata_schedule_295_0_e2293 * w[18]);
        let noise_metadata_schedule_295_0_e2298: f64 = (w[192] * params.p8);
        let noise_metadata_schedule_295_0_e2300: f64 = (noise_metadata_schedule_295_0_e2298 * w[191]);
        let noise_metadata_schedule_295_0_e2302: f64 = (noise_metadata_schedule_295_0_e2300 * params.p13);
        let noise_metadata_schedule_295_0_e2303: f64 = (noise_metadata_schedule_295_0_e2295 / noise_metadata_schedule_295_0_e2302);
        (noise_metadata_schedule_295_0_e2303,)
    } else {
        (w[259],)
    }
};
            w[259] = noise_metadata_schedule_295_0_e2305;
        }
    }
}

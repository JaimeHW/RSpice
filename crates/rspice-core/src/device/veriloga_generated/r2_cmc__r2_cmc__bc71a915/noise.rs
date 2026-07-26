#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_N2_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 1, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_N1_N2_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 2, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 86];
        let noise_source_0_active = {
            true
        };
        let noise_source_1_active = {
            true
        };
        let noise_source_active = [noise_source_0_active, noise_source_1_active];
        let noise_source_active_mask = [(noise_source_0_active as u128) | ((noise_source_1_active as u128) << 1)];
        w.fill(0.0);
        self.noise_metadata_schedule_part_0(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_1(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_2(ctx, &mut w, &noise_source_active_mask);
        if !noise_source_active[0] {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_0_psd_e1467: f64 = 1.0;
            let noise_0_psd_e1468: f64 = (noise_0_psd_e1467 * w[26]);
            let psd = noise_0_psd_e1468;
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
            let noise_1_psd_e1470: f64 = 1.0;
            let noise_1_psd_e1471: f64 = (noise_1_psd_e1470 * w[27]);
            let psd = noise_1_psd_e1471;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = Some(params.p31);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 86], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_1_0_e60: f64 = if self.param_given[9] { 1.0 } else { 0.0 };
            w[39] = noise_metadata_schedule_1_0_e60;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_2_0_e64,) = {
    if (w[39] != 0.0) {
        (params.p9,)
    } else {
        (w[10],)
    }
};
            w[10] = noise_metadata_schedule_2_0_e64;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_3_0_e71,) = {
    if (w[39] == 0.0) {
        let noise_metadata_schedule_3_0_e69: f64 = 1.0;
        (noise_metadata_schedule_3_0_e69,)
    } else {
        (w[10],)
    }
};
            w[10] = noise_metadata_schedule_3_0_e71;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_4_0_e73: f64 = if self.param_given[10] { 1.0 } else { 0.0 };
            w[40] = noise_metadata_schedule_4_0_e73;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_5_0_e81,) = {
    if (w[40] != 0.0) {
        let noise_metadata_schedule_5_0_e78: f64 = (0.01 * params.p10);
        let noise_metadata_schedule_5_0_e79: f64 = (1.0 - noise_metadata_schedule_5_0_e78);
        (noise_metadata_schedule_5_0_e79,)
    } else {
        (w[11],)
    }
};
            w[11] = noise_metadata_schedule_5_0_e81;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_6_0_e92,) = {
    if (w[40] == 0.0) {
        let noise_metadata_schedule_6_0_e88: f64 = 0.0;
        let noise_metadata_schedule_6_0_e89: f64 = (0.01 * noise_metadata_schedule_6_0_e88);
        let noise_metadata_schedule_6_0_e90: f64 = (1.0 - noise_metadata_schedule_6_0_e89);
        (noise_metadata_schedule_6_0_e90,)
    } else {
        (w[11],)
    }
};
            w[11] = noise_metadata_schedule_6_0_e92;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_10_0_e108: f64 = (w[11] * w[10]);
            let noise_metadata_schedule_10_0_e110: f64 = (noise_metadata_schedule_10_0_e108 * 1000000.0);
            w[15] = noise_metadata_schedule_10_0_e110;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_11_0_e113: f64 = (273.15 + params.p15);
            w[8] = noise_metadata_schedule_11_0_e113;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_12_0_e114: f64 = ctx.temperature();
            let noise_metadata_schedule_12_0_e116: f64 = (noise_metadata_schedule_12_0_e114 + params.p5);
            let noise_metadata_schedule_12_0_e118: f64 = (noise_metadata_schedule_12_0_e116 - 273.15);
            w[25] = noise_metadata_schedule_12_0_e118;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_15_0_e128: f64 = (params.p34 + 1.0);
            let noise_metadata_schedule_15_0_e129: f64 = if w[25] < noise_metadata_schedule_15_0_e128 { 1.0 } else { 0.0 };
            w[44] = noise_metadata_schedule_15_0_e129;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_16_0_e140,) = {
    if (w[44] != 0.0) {
        let noise_metadata_schedule_16_0_e134: f64 = (w[25] - params.p34);
        let noise_metadata_schedule_16_0_e136: f64 = (noise_metadata_schedule_16_0_e134 - 1.0);
        let noise_metadata_schedule_16_0_e137: f64 = (noise_metadata_schedule_16_0_e136).exp();
        let noise_metadata_schedule_16_0_e138: f64 = (params.p34 + noise_metadata_schedule_16_0_e137);
        (noise_metadata_schedule_16_0_e138,)
    } else {
        (w[25],)
    }
};
            w[25] = noise_metadata_schedule_16_0_e140;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_17_0_e144: f64 = (params.p35 - 1.0);
            let noise_metadata_schedule_17_0_e145: f64 = if w[25] > noise_metadata_schedule_17_0_e144 { 1.0 } else { 0.0 };
            w[45] = noise_metadata_schedule_17_0_e145;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_18_0_e159,) = {
    if ((w[44] == 0.0) && (w[45] != 0.0)) {
        let noise_metadata_schedule_18_0_e153: f64 = (params.p35 - w[25]);
        let noise_metadata_schedule_18_0_e155: f64 = (noise_metadata_schedule_18_0_e153 - 1.0);
        let noise_metadata_schedule_18_0_e156: f64 = (noise_metadata_schedule_18_0_e155).exp();
        let noise_metadata_schedule_18_0_e157: f64 = (params.p35 - noise_metadata_schedule_18_0_e156);
        (noise_metadata_schedule_18_0_e157,)
    } else {
        (w[25],)
    }
};
            w[25] = noise_metadata_schedule_18_0_e159;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_19_0_e167,) = {
    if ((w[44] == 0.0) && (w[45] == 0.0)) {
        (w[25],)
    } else {
        (w[25],)
    }
};
            w[25] = noise_metadata_schedule_19_0_e167;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_20_0_e170: f64 = (w[25] + 273.15);
            w[9] = noise_metadata_schedule_20_0_e170;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_21_0_e173: f64 = (w[9] - w[8]);
            w[12] = noise_metadata_schedule_21_0_e173;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_22_0_e177: f64 = (w[12] * params.p42);
            let noise_metadata_schedule_22_0_e178: f64 = (1.0 + noise_metadata_schedule_22_0_e177);
            let noise_metadata_schedule_22_0_e180: f64 = (noise_metadata_schedule_22_0_e178 * params.p29);
            w[22] = noise_metadata_schedule_22_0_e180;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_23_0_e183: f64 = if w[22] < 0.0 { 1.0 } else { 0.0 };
            w[46] = noise_metadata_schedule_23_0_e183;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_24_0_e187,) = {
    if (w[46] != 0.0) {
        (0.0,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_24_0_e187;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_25_0_e190: f64 = if ((params.p3 != 0.0) && (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            w[47] = noise_metadata_schedule_25_0_e190;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_26_0_e194,) = {
    if (w[47] != 0.0) {
        (params.p22,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_26_0_e194;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_27_0_e197: f64 = if ((params.p3 != 0.0) || (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            w[48] = noise_metadata_schedule_27_0_e197;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_28_0_e206,) = {
    if ((w[47] == 0.0) && (w[48] != 0.0)) {
        let noise_metadata_schedule_28_0_e204: f64 = (params.p22 * 0.5);
        (noise_metadata_schedule_28_0_e204,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_28_0_e206;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_29_0_e214,) = {
    if ((w[47] == 0.0) && (w[48] == 0.0)) {
        (0.0,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_29_0_e214;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_30_0_e223: f64 = if ((self.param_given[1] && self.param_given[2]) && (!self.param_given[0])) { 1.0 } else { 0.0 };
            w[49] = noise_metadata_schedule_30_0_e223;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_31_0_e230: f64 = if ((params.p2 == 0.0) || (params.p1 == 0.0)) { 1.0 } else { 0.0 };
            w[50] = noise_metadata_schedule_31_0_e230;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_32_0_e236,) = {
    if ((w[49] != 0.0) && (w[50] != 0.0)) {
        (0.0,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_32_0_e236;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_33_0_e242,) = {
    if ((w[49] != 0.0) && (w[50] != 0.0)) {
        (0.0,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_33_0_e242;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_34_0_e250,) = {
    if ((w[49] != 0.0) && (w[50] != 0.0)) {
        let noise_metadata_schedule_34_0_e248: f64 = (params.p0 * w[15]);
        (noise_metadata_schedule_34_0_e248,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_34_0_e250;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_35_0_e258,) = {
    if ((w[49] != 0.0) && (w[50] != 0.0)) {
        let noise_metadata_schedule_35_0_e256: f64 = (w[17] + params.p21);
        (noise_metadata_schedule_35_0_e256,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_35_0_e258;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_36_0_e264,) = {
    if ((w[49] != 0.0) && (w[50] != 0.0)) {
        (0.0,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_36_0_e264;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_37_0_e270,) = {
    if ((w[49] != 0.0) && (w[50] != 0.0)) {
        (1e99,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_37_0_e270;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_38_0_e279,) = {
    if ((w[49] != 0.0) && (w[50] == 0.0)) {
        let noise_metadata_schedule_38_0_e277: f64 = (params.p1 * w[15]);
        (noise_metadata_schedule_38_0_e277,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_38_0_e279;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_39_0_e288,) = {
    if ((w[49] != 0.0) && (w[50] == 0.0)) {
        let noise_metadata_schedule_39_0_e286: f64 = (w[16] + w[14]);
        (noise_metadata_schedule_39_0_e286,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_39_0_e288;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_41_0_e294: f64 = if w[3] > 0.0 { 1.0 } else { 0.0 };
            w[52] = noise_metadata_schedule_41_0_e294;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_42_0_e307,) = {
    if (((w[49] != 0.0) && (w[50] == 0.0)) && (w[52] != 0.0)) {
        let noise_metadata_schedule_42_0_e303: f64 = (params.p16 / params.p2);
        let noise_metadata_schedule_42_0_e305: f64 = (noise_metadata_schedule_42_0_e303 * w[3]);
        (noise_metadata_schedule_42_0_e305,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_42_0_e307;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_43_0_e318,) = {
    if (((w[49] != 0.0) && (w[50] == 0.0)) && (w[52] != 0.0)) {
        let noise_metadata_schedule_43_0_e316: f64 = (w[4] - params.p21);
        (noise_metadata_schedule_43_0_e316,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_43_0_e318;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_45_0_e330,) = {
    if (((w[49] != 0.0) && (w[50] == 0.0)) && (w[52] != 0.0)) {
        (params.p2,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_45_0_e330;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_46_0_e341,) = {
    if (((w[49] != 0.0) && (w[50] == 0.0)) && (w[52] != 0.0)) {
        let noise_metadata_schedule_46_0_e339: f64 = (1.0 / w[5]);
        (noise_metadata_schedule_46_0_e339,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_46_0_e341;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_47_0_e353,) = {
    if (((w[49] != 0.0) && (w[50] == 0.0)) && (w[52] == 0.0)) {
        let noise_metadata_schedule_47_0_e351: f64 = (params.p0 * w[15]);
        (noise_metadata_schedule_47_0_e351,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_47_0_e353;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_48_0_e365,) = {
    if (((w[49] != 0.0) && (w[50] == 0.0)) && (w[52] == 0.0)) {
        let noise_metadata_schedule_48_0_e363: f64 = (w[17] + params.p21);
        (noise_metadata_schedule_48_0_e363,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_48_0_e365;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_49_0_e375,) = {
    if (((w[49] != 0.0) && (w[50] == 0.0)) && (w[52] == 0.0)) {
        (0.0,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_49_0_e375;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_50_0_e385,) = {
    if (((w[49] != 0.0) && (w[50] == 0.0)) && (w[52] == 0.0)) {
        (1e99,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_50_0_e385;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_51_0_e391: f64 = if (self.param_given[2] && (!self.param_given[1])) { 1.0 } else { 0.0 };
            w[54] = noise_metadata_schedule_51_0_e391;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_52_0_e394: f64 = if params.p2 == 0.0 { 1.0 } else { 0.0 };
            w[55] = noise_metadata_schedule_52_0_e394;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_53_0_e403,) = {
    if (((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] != 0.0)) {
        (0.0,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_53_0_e403;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_54_0_e412,) = {
    if (((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] != 0.0)) {
        (0.0,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_54_0_e412;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_55_0_e423,) = {
    if (((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] != 0.0)) {
        let noise_metadata_schedule_55_0_e421: f64 = (params.p0 * w[15]);
        (noise_metadata_schedule_55_0_e421,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_55_0_e423;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_56_0_e434,) = {
    if (((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] != 0.0)) {
        let noise_metadata_schedule_56_0_e432: f64 = (w[17] + params.p21);
        (noise_metadata_schedule_56_0_e432,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_56_0_e434;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_57_0_e443,) = {
    if (((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] != 0.0)) {
        (0.0,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_57_0_e443;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_58_0_e452,) = {
    if (((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] != 0.0)) {
        (1e99,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_58_0_e452;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_59_0_e455: f64 = if params.p0 == 0.0 { 1.0 } else { 0.0 };
            w[56] = noise_metadata_schedule_59_0_e455;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_60_0_e467,) = {
    if ((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] != 0.0)) {
        (0.0,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_60_0_e467;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_61_0_e479,) = {
    if ((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] != 0.0)) {
        (0.0,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_61_0_e479;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_62_0_e493,) = {
    if ((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] != 0.0)) {
        let noise_metadata_schedule_62_0_e491: f64 = (params.p1 * w[15]);
        (noise_metadata_schedule_62_0_e491,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_62_0_e493;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_63_0_e507,) = {
    if ((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] != 0.0)) {
        let noise_metadata_schedule_63_0_e505: f64 = (w[16] + w[14]);
        (noise_metadata_schedule_63_0_e505,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_63_0_e507;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_64_0_e519,) = {
    if ((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] != 0.0)) {
        (1e99,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_64_0_e519;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_65_0_e531,) = {
    if ((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] != 0.0)) {
        (0.0,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_65_0_e531;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 86], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_66_0_e546,) = {
    if ((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] == 0.0)) {
        let noise_metadata_schedule_66_0_e544: f64 = (params.p0 * w[15]);
        (noise_metadata_schedule_66_0_e544,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_66_0_e546;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_67_0_e561,) = {
    if ((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] == 0.0)) {
        let noise_metadata_schedule_67_0_e559: f64 = (w[17] + params.p21);
        (noise_metadata_schedule_67_0_e559,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_67_0_e561;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_69_0_e567: f64 = if w[4] > 0.0 { 1.0 } else { 0.0 };
            w[58] = noise_metadata_schedule_69_0_e567;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_70_0_e586,) = {
    if (((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] == 0.0)) && (w[58] != 0.0)) {
        let noise_metadata_schedule_70_0_e582: f64 = (params.p2 / params.p16);
        let noise_metadata_schedule_70_0_e584: f64 = (noise_metadata_schedule_70_0_e582 * w[4]);
        (noise_metadata_schedule_70_0_e584,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_70_0_e586;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_71_0_e603,) = {
    if (((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] == 0.0)) && (w[58] != 0.0)) {
        let noise_metadata_schedule_71_0_e601: f64 = (w[3] - w[14]);
        (noise_metadata_schedule_71_0_e601,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_71_0_e603;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_73_0_e621,) = {
    if (((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] == 0.0)) && (w[58] != 0.0)) {
        (params.p2,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_73_0_e621;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_74_0_e638,) = {
    if (((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] == 0.0)) && (w[58] != 0.0)) {
        let noise_metadata_schedule_74_0_e636: f64 = (1.0 / w[5]);
        (noise_metadata_schedule_74_0_e636,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_74_0_e638;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_75_0_e656,) = {
    if (((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] == 0.0)) && (w[58] == 0.0)) {
        let noise_metadata_schedule_75_0_e654: f64 = (params.p1 * w[15]);
        (noise_metadata_schedule_75_0_e654,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_75_0_e656;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_76_0_e674,) = {
    if (((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] == 0.0)) && (w[58] == 0.0)) {
        let noise_metadata_schedule_76_0_e672: f64 = (w[16] + w[14]);
        (noise_metadata_schedule_76_0_e672,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_76_0_e674;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_77_0_e690,) = {
    if (((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] == 0.0)) && (w[58] == 0.0)) {
        (1e99,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_77_0_e690;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_78_0_e706,) = {
    if (((((w[49] == 0.0) && (w[54] != 0.0)) && (w[55] == 0.0)) && (w[56] == 0.0)) && (w[58] == 0.0)) {
        (0.0,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_78_0_e706;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_79_0_e709: f64 = if params.p0 == 0.0 { 1.0 } else { 0.0 };
            w[60] = noise_metadata_schedule_79_0_e709;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_80_0_e719,) = {
    if (((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] != 0.0)) {
        (0.0,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_80_0_e719;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_81_0_e729,) = {
    if (((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] != 0.0)) {
        (0.0,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_81_0_e729;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_82_0_e741,) = {
    if (((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] != 0.0)) {
        let noise_metadata_schedule_82_0_e739: f64 = (params.p1 * w[15]);
        (noise_metadata_schedule_82_0_e739,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_82_0_e741;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_83_0_e753,) = {
    if (((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] != 0.0)) {
        let noise_metadata_schedule_83_0_e751: f64 = (w[16] + w[14]);
        (noise_metadata_schedule_83_0_e751,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_83_0_e753;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_84_0_e763,) = {
    if (((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] != 0.0)) {
        (1e99,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_84_0_e763;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_85_0_e773,) = {
    if (((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] != 0.0)) {
        (0.0,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_85_0_e773;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_86_0_e776: f64 = if params.p1 == 0.0 { 1.0 } else { 0.0 };
            w[61] = noise_metadata_schedule_86_0_e776;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_87_0_e789,) = {
    if ((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] != 0.0)) {
        (0.0,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_87_0_e789;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_88_0_e802,) = {
    if ((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] != 0.0)) {
        (0.0,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_88_0_e802;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_89_0_e817,) = {
    if ((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] != 0.0)) {
        let noise_metadata_schedule_89_0_e815: f64 = (params.p0 * w[15]);
        (noise_metadata_schedule_89_0_e815,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_89_0_e817;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_90_0_e832,) = {
    if ((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] != 0.0)) {
        let noise_metadata_schedule_90_0_e830: f64 = (w[17] + params.p21);
        (noise_metadata_schedule_90_0_e830,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_90_0_e832;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_91_0_e845,) = {
    if ((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] != 0.0)) {
        (0.0,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_91_0_e845;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_92_0_e858,) = {
    if ((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] != 0.0)) {
        (1e99,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_92_0_e858;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_93_0_e874,) = {
    if ((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] == 0.0)) {
        let noise_metadata_schedule_93_0_e872: f64 = (params.p0 * w[15]);
        (noise_metadata_schedule_93_0_e872,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_93_0_e874;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_94_0_e890,) = {
    if ((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] == 0.0)) {
        let noise_metadata_schedule_94_0_e888: f64 = (w[17] + params.p21);
        (noise_metadata_schedule_94_0_e888,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_94_0_e890;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_96_0_e909,) = {
    if ((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] == 0.0)) {
        let noise_metadata_schedule_96_0_e907: f64 = (params.p1 * w[15]);
        (noise_metadata_schedule_96_0_e907,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_96_0_e909;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_97_0_e925,) = {
    if ((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] == 0.0)) {
        let noise_metadata_schedule_97_0_e923: f64 = (w[16] + w[14]);
        (noise_metadata_schedule_97_0_e923,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_97_0_e925;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_98_0_e928: f64 = if w[4] > 0.0 { 1.0 } else { 0.0 };
            w[63] = noise_metadata_schedule_98_0_e928;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_100_0_e934: f64 = if w[3] > 0.0 { 1.0 } else { 0.0 };
            w[65] = noise_metadata_schedule_100_0_e934;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_101_0_e956,) = {
    if ((((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] == 0.0)) && (w[63] != 0.0)) && (w[65] != 0.0)) {
        let noise_metadata_schedule_101_0_e953: f64 = (w[3] / w[4]);
        let noise_metadata_schedule_101_0_e954: f64 = (params.p16 * noise_metadata_schedule_101_0_e953);
        (noise_metadata_schedule_101_0_e954,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_101_0_e956;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_102_0_e976,) = {
    if ((((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] == 0.0)) && (w[63] != 0.0)) && (w[65] != 0.0)) {
        let noise_metadata_schedule_102_0_e974: f64 = (1.0 / w[5]);
        (noise_metadata_schedule_102_0_e974,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_102_0_e976;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_103_0_e995,) = {
    if ((((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] == 0.0)) && (w[63] != 0.0)) && (w[65] == 0.0)) {
        (0.0,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_103_0_e995;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_104_0_e1014,) = {
    if ((((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] == 0.0)) && (w[63] != 0.0)) && (w[65] == 0.0)) {
        (1e99,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_104_0_e1014;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_105_0_e1031,) = {
    if (((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] == 0.0)) && (w[63] == 0.0)) {
        (1e99,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_105_0_e1031;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_106_0_e1048,) = {
    if (((((w[49] == 0.0) && (w[54] == 0.0)) && (w[60] == 0.0)) && (w[61] == 0.0)) && (w[63] == 0.0)) {
        (0.0,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_106_0_e1048;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_111_0_e1066,) = {
    if (params.p24 != 0.0) {
        let noise_metadata_schedule_111_0_e1064: f64 = (w[3] + params.p23);
        (noise_metadata_schedule_111_0_e1064,)
    } else {
        (w[18],)
    }
};
            w[18] = noise_metadata_schedule_111_0_e1066;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_112_0_e1073,) = {
    if (params.p24 == 0.0) {
        let noise_metadata_schedule_112_0_e1071: f64 = (w[16] + params.p23);
        (noise_metadata_schedule_112_0_e1071,)
    } else {
        (w[18],)
    }
};
            w[18] = noise_metadata_schedule_112_0_e1073;
        }
        if (active[0] & 0x3) != 0 {
            w[34] = params.p36;
        }
        if (active[0] & 0x3) != 0 {
            w[35] = params.p37;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_116_0_e1093: f64 = if w[3] > 0.0 { 1.0 } else { 0.0 };
            w[71] = noise_metadata_schedule_116_0_e1093;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_117_0_e1096: f64 = if ((params.p3 != 0.0) && (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            w[72] = noise_metadata_schedule_117_0_e1096;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_118_0_e1106,) = {
    if ((w[71] != 0.0) && (w[72] != 0.0)) {
        let noise_metadata_schedule_118_0_e1103: f64 = (params.p38 / w[3]);
        let noise_metadata_schedule_118_0_e1104: f64 = (w[34] + noise_metadata_schedule_118_0_e1103);
        (noise_metadata_schedule_118_0_e1104,)
    } else {
        (w[34],)
    }
};
            w[34] = noise_metadata_schedule_118_0_e1106;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_119_0_e1116,) = {
    if ((w[71] != 0.0) && (w[72] != 0.0)) {
        let noise_metadata_schedule_119_0_e1113: f64 = (params.p39 / w[3]);
        let noise_metadata_schedule_119_0_e1114: f64 = (w[35] + noise_metadata_schedule_119_0_e1113);
        (noise_metadata_schedule_119_0_e1114,)
    } else {
        (w[35],)
    }
};
            w[35] = noise_metadata_schedule_119_0_e1116;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_120_0_e1119: f64 = if ((params.p3 != 0.0) || (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            w[73] = noise_metadata_schedule_120_0_e1119;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_121_0_e1134,) = {
    if (((w[71] != 0.0) && (w[72] == 0.0)) && (w[73] != 0.0)) {
        let noise_metadata_schedule_121_0_e1129: f64 = (0.5 * params.p38);
        let noise_metadata_schedule_121_0_e1131: f64 = (noise_metadata_schedule_121_0_e1129 / w[3]);
        let noise_metadata_schedule_121_0_e1132: f64 = (w[34] + noise_metadata_schedule_121_0_e1131);
        (noise_metadata_schedule_121_0_e1132,)
    } else {
        (w[34],)
    }
};
            w[34] = noise_metadata_schedule_121_0_e1134;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_122_0_e1149,) = {
    if (((w[71] != 0.0) && (w[72] == 0.0)) && (w[73] != 0.0)) {
        let noise_metadata_schedule_122_0_e1144: f64 = (0.5 * params.p39);
        let noise_metadata_schedule_122_0_e1146: f64 = (noise_metadata_schedule_122_0_e1144 / w[3]);
        let noise_metadata_schedule_122_0_e1147: f64 = (w[35] + noise_metadata_schedule_122_0_e1146);
        (noise_metadata_schedule_122_0_e1147,)
    } else {
        (w[35],)
    }
};
            w[35] = noise_metadata_schedule_122_0_e1149;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_123_0_e1152: f64 = if w[4] > 0.0 { 1.0 } else { 0.0 };
            w[74] = noise_metadata_schedule_123_0_e1152;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_124_0_e1160,) = {
    if (w[74] != 0.0) {
        let noise_metadata_schedule_124_0_e1157: f64 = (params.p40 / w[4]);
        let noise_metadata_schedule_124_0_e1158: f64 = (w[34] + noise_metadata_schedule_124_0_e1157);
        (noise_metadata_schedule_124_0_e1158,)
    } else {
        (w[34],)
    }
};
            w[34] = noise_metadata_schedule_124_0_e1160;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_125_0_e1168,) = {
    if (w[74] != 0.0) {
        let noise_metadata_schedule_125_0_e1165: f64 = (params.p41 / w[4]);
        let noise_metadata_schedule_125_0_e1166: f64 = (w[35] + noise_metadata_schedule_125_0_e1165);
        (noise_metadata_schedule_125_0_e1166,)
    } else {
        (w[35],)
    }
};
            w[35] = noise_metadata_schedule_125_0_e1168;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_129_0_e1188: f64 = (w[12] * w[35]);
            let noise_metadata_schedule_129_0_e1189: f64 = (w[34] + noise_metadata_schedule_129_0_e1188);
            let noise_metadata_schedule_129_0_e1190: f64 = (w[12] * noise_metadata_schedule_129_0_e1189);
            let noise_metadata_schedule_129_0_e1191: f64 = (1.0 + noise_metadata_schedule_129_0_e1190);
            w[13] = noise_metadata_schedule_129_0_e1191;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_130_0_e1195: f64 = (0.01 + 0.1);
            let noise_metadata_schedule_130_0_e1196: f64 = if w[13] < noise_metadata_schedule_130_0_e1195 { 1.0 } else { 0.0 };
            w[76] = noise_metadata_schedule_130_0_e1196;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_131_0_e1211,) = {
    if (w[76] != 0.0) {
        let noise_metadata_schedule_131_0_e1203: f64 = (w[13] - 0.01);
        let noise_metadata_schedule_131_0_e1204: f64 = (10.0 * noise_metadata_schedule_131_0_e1203);
        let noise_metadata_schedule_131_0_e1206: f64 = (noise_metadata_schedule_131_0_e1204 - 1.0);
        let noise_metadata_schedule_131_0_e1207: f64 = (noise_metadata_schedule_131_0_e1206).exp();
        let noise_metadata_schedule_131_0_e1208: f64 = (0.1 * noise_metadata_schedule_131_0_e1207);
        let noise_metadata_schedule_131_0_e1209: f64 = (0.01 + noise_metadata_schedule_131_0_e1208);
        (noise_metadata_schedule_131_0_e1209,)
    } else {
        (w[13],)
    }
};
            w[13] = noise_metadata_schedule_131_0_e1211;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 86], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_132_0_e1216,) = {
    if (w[76] == 0.0) {
        (w[13],)
    } else {
        (w[13],)
    }
};
            w[13] = noise_metadata_schedule_132_0_e1216;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_133_0_e1219: f64 = (w[5] * w[13]);
            w[20] = noise_metadata_schedule_133_0_e1219;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_134_0_e1222: f64 = (w[19] / w[13]);
            w[21] = noise_metadata_schedule_134_0_e1222;
        }
        if (active[0] & 0x3) != 0 {
            w[30] = (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[1]));
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_136_0_e1234: f64 = if ((w[5] > 0.0) && ((params.p28 > 0.0) || (params.p26 > 0.0))) { 1.0 } else { 0.0 };
            w[77] = noise_metadata_schedule_136_0_e1234;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_137_0_e1240,) = {
    if (w[77] != 0.0) {
        let noise_metadata_schedule_137_0_e1238: f64 = (w[30] / w[18]);
        (noise_metadata_schedule_137_0_e1238,)
    } else {
        (w[31],)
    }
};
            w[31] = noise_metadata_schedule_137_0_e1240;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_138_0_e1246,) = {
    if (w[77] != 0.0) {
        let noise_metadata_schedule_138_0_e1244: f64 = (params.p27 * w[31]);
        (noise_metadata_schedule_138_0_e1244,)
    } else {
        (w[32],)
    }
};
            w[32] = noise_metadata_schedule_138_0_e1246;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_139_0_e1255,) = {
    if (w[77] != 0.0) {
        let noise_metadata_schedule_139_0_e1251: f64 = (w[32] * w[32]);
        let noise_metadata_schedule_139_0_e1252: f64 = (1.0 + noise_metadata_schedule_139_0_e1251);
        let noise_metadata_schedule_139_0_e1253: f64 = (noise_metadata_schedule_139_0_e1252).sqrt();
        (noise_metadata_schedule_139_0_e1253,)
    } else {
        (w[23],)
    }
};
            w[23] = noise_metadata_schedule_139_0_e1255;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_140_0_e1262,) = {
    if (w[77] != 0.0) {
        let noise_metadata_schedule_140_0_e1259: f64 = (w[31]).abs();
        let noise_metadata_schedule_140_0_e1260: f64 = (params.p25 * noise_metadata_schedule_140_0_e1259);
        (noise_metadata_schedule_140_0_e1260,)
    } else {
        (w[33],)
    }
};
            w[33] = noise_metadata_schedule_140_0_e1262;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_141_0_e1274,) = {
    if (w[77] != 0.0) {
        let noise_metadata_schedule_141_0_e1267: f64 = (w[33] * w[33]);
        let noise_metadata_schedule_141_0_e1269: f64 = (noise_metadata_schedule_141_0_e1267 * w[33]);
        let noise_metadata_schedule_141_0_e1270: f64 = (1.0 + noise_metadata_schedule_141_0_e1269);
        let noise_metadata_schedule_141_0_e1272: f64 = (noise_metadata_schedule_141_0_e1270).powf(0.3333333333333333);
        (noise_metadata_schedule_141_0_e1272,)
    } else {
        (w[24],)
    }
};
            w[24] = noise_metadata_schedule_141_0_e1274;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_142_0_e1290,) = {
    if (w[77] != 0.0) {
        let noise_metadata_schedule_142_0_e1278: f64 = (1.0 - params.p28);
        let noise_metadata_schedule_142_0_e1280: f64 = (noise_metadata_schedule_142_0_e1278 - params.p26);
        let noise_metadata_schedule_142_0_e1283: f64 = (params.p28 * w[23]);
        let noise_metadata_schedule_142_0_e1284: f64 = (noise_metadata_schedule_142_0_e1280 + noise_metadata_schedule_142_0_e1283);
        let noise_metadata_schedule_142_0_e1287: f64 = (params.p26 * w[24]);
        let noise_metadata_schedule_142_0_e1288: f64 = (noise_metadata_schedule_142_0_e1284 + noise_metadata_schedule_142_0_e1287);
        (noise_metadata_schedule_142_0_e1288,)
    } else {
        (w[29],)
    }
};
            w[29] = noise_metadata_schedule_142_0_e1290;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_143_0_e1295,) = {
    if (w[77] == 0.0) {
        (1.0,)
    } else {
        (w[29],)
    }
};
            w[29] = noise_metadata_schedule_143_0_e1295;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_144_0_e1298: f64 = (w[20] * w[29]);
            w[6] = noise_metadata_schedule_144_0_e1298;
        }
        if (active[0] & 0x2) != 0 {
            w[0] = w[30];
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_146_0_e1302: f64 = (w[0] / w[6]);
            w[1] = noise_metadata_schedule_146_0_e1302;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_149_0_e1320: f64 = if (((params.p6 != 0.0) && (w[5] > 0.0)) && (w[19] > 0.0)) { 1.0 } else { 0.0 };
            w[80] = noise_metadata_schedule_149_0_e1320;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_150_0_e1332,) = {
    if (w[80] != 0.0) {
        let noise_metadata_schedule_150_0_e1324: f64 = (4.0 * 1.3806505e-23);
        let noise_metadata_schedule_150_0_e1326: f64 = (noise_metadata_schedule_150_0_e1324 * w[9]);
        let noise_metadata_schedule_150_0_e1328: f64 = (noise_metadata_schedule_150_0_e1326 * w[21]);
        let noise_metadata_schedule_150_0_e1330: f64 = (noise_metadata_schedule_150_0_e1328 / w[29]);
        (noise_metadata_schedule_150_0_e1330,)
    } else {
        (w[26],)
    }
};
            w[26] = noise_metadata_schedule_150_0_e1332;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_151_0_e1341: f64 = if (((params.p32 != 0.0) && (w[3] > 0.0)) && (w[4] > 0.0)) { 1.0 } else { 0.0 };
            w[81] = noise_metadata_schedule_151_0_e1341;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_152_0_e1358,) = {
    if ((w[80] != 0.0) && (w[81] != 0.0)) {
        let noise_metadata_schedule_152_0_e1348: f64 = (w[1] / w[4]);
        let noise_metadata_schedule_152_0_e1349: f64 = (noise_metadata_schedule_152_0_e1348).abs();
        let noise_metadata_schedule_152_0_e1351: f64 = (noise_metadata_schedule_152_0_e1349).powf(params.p30);
        let noise_metadata_schedule_152_0_e1352: f64 = (w[22] * noise_metadata_schedule_152_0_e1351);
        let noise_metadata_schedule_152_0_e1354: f64 = (noise_metadata_schedule_152_0_e1352 * w[4]);
        let noise_metadata_schedule_152_0_e1356: f64 = (noise_metadata_schedule_152_0_e1354 / w[3]);
        (noise_metadata_schedule_152_0_e1356,)
    } else {
        (w[27],)
    }
};
            w[27] = noise_metadata_schedule_152_0_e1358;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_153_0_e1365: f64 = if ((w[16] > 0.0) && (w[17] > 0.0)) { 1.0 } else { 0.0 };
            w[82] = noise_metadata_schedule_153_0_e1365;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_154_0_e1385,) = {
    if (((w[80] != 0.0) && (w[81] == 0.0)) && (w[82] != 0.0)) {
        let noise_metadata_schedule_154_0_e1375: f64 = (w[1] / w[17]);
        let noise_metadata_schedule_154_0_e1376: f64 = (noise_metadata_schedule_154_0_e1375).abs();
        let noise_metadata_schedule_154_0_e1378: f64 = (noise_metadata_schedule_154_0_e1376).powf(params.p30);
        let noise_metadata_schedule_154_0_e1379: f64 = (w[22] * noise_metadata_schedule_154_0_e1378);
        let noise_metadata_schedule_154_0_e1381: f64 = (noise_metadata_schedule_154_0_e1379 * w[17]);
        let noise_metadata_schedule_154_0_e1383: f64 = (noise_metadata_schedule_154_0_e1381 / w[16]);
        (noise_metadata_schedule_154_0_e1383,)
    } else {
        (w[27],)
    }
};
            w[27] = noise_metadata_schedule_154_0_e1385;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_155_0_e1395,) = {
    if (((w[80] != 0.0) && (w[81] == 0.0)) && (w[82] == 0.0)) {
        (0.0,)
    } else {
        (w[27],)
    }
};
            w[27] = noise_metadata_schedule_155_0_e1395;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_156_0_e1398: f64 = if w[1] < 0.0 { 1.0 } else { 0.0 };
            w[83] = noise_metadata_schedule_156_0_e1398;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_157_0_e1405,) = {
    if ((w[80] != 0.0) && (w[83] != 0.0)) {
        let noise_metadata_schedule_157_0_e1403: f64 = (-w[27]);
        (noise_metadata_schedule_157_0_e1403,)
    } else {
        (w[27],)
    }
};
            w[27] = noise_metadata_schedule_157_0_e1405;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_158_0_e1410,) = {
    if (w[80] == 0.0) {
        (0.0,)
    } else {
        (w[26],)
    }
};
            w[26] = noise_metadata_schedule_158_0_e1410;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_159_0_e1415,) = {
    if (w[80] == 0.0) {
        (0.0,)
    } else {
        (w[27],)
    }
};
            w[27] = noise_metadata_schedule_159_0_e1415;
        }
    }
}

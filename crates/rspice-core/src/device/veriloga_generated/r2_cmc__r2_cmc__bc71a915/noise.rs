#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_N2_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 1, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_N1_N2_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 2, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_source(&self, source_index: usize, ctx: &GeneratedEvalContext<'_>) -> Result<GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError> {
        if source_index >= NOISE_SOURCES.len() {
            return Err(GeneratedNoiseEvaluationError::SourceIndexOutOfRange { index: source_index, count: NOISE_SOURCES.len() });
        }
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut noise_variable_0 = 0.0;
        let mut noise_variable_1 = 0.0;
        let mut noise_variable_2 = 0.0;
        let mut noise_variable_3 = 0.0;
        let mut noise_variable_4 = 0.0;
        let mut noise_variable_5 = 0.0;
        let mut noise_variable_6 = 0.0;
        let mut noise_variable_7 = 0.0;
        let mut noise_variable_8 = 0.0;
        let mut noise_variable_9 = 0.0;
        let mut noise_variable_10 = 0.0;
        let mut noise_variable_11 = 0.0;
        let mut noise_variable_12 = 0.0;
        let mut noise_variable_13 = 0.0;
        let mut noise_variable_14 = 0.0;
        let mut noise_variable_15 = 0.0;
        let mut noise_variable_16 = 0.0;
        let mut noise_variable_17 = 0.0;
        let mut noise_variable_18 = 0.0;
        let mut noise_variable_19 = 0.0;
        let mut noise_variable_20 = 0.0;
        let mut noise_variable_21 = 0.0;
        let mut noise_variable_22 = 0.0;
        let mut noise_variable_23 = 0.0;
        let mut noise_variable_24 = 0.0;
        let mut noise_variable_25 = 0.0;
        let mut noise_variable_26 = 0.0;
        let mut noise_variable_27 = 0.0;
        let mut noise_variable_28 = 0.0;
        let mut noise_variable_29 = 0.0;
        let mut noise_variable_30 = 0.0;
        let mut noise_variable_31 = 0.0;
        let mut noise_variable_32 = 0.0;
        let mut noise_variable_33 = 0.0;
        let mut noise_variable_34 = 0.0;
        let mut noise_variable_35 = 0.0;
        let mut noise_variable_36 = 0.0;
        let mut noise_variable_37 = 0.0;
        let mut noise_variable_38 = 0.0;
        let mut noise_variable_39 = 0.0;
        let mut noise_variable_40 = 0.0;
        let mut noise_variable_41 = 0.0;
        let mut noise_variable_42 = 0.0;
        let mut noise_variable_43 = 0.0;
        let mut noise_variable_44 = 0.0;
        let mut noise_variable_45 = 0.0;
        let mut noise_variable_46 = 0.0;
        let mut noise_variable_47 = 0.0;
        let mut noise_variable_48 = 0.0;
        let mut noise_variable_49 = 0.0;
        let mut noise_variable_50 = 0.0;
        let mut noise_variable_51 = 0.0;
        let mut noise_variable_52 = 0.0;
        let mut noise_variable_53 = 0.0;
        let mut noise_variable_54 = 0.0;
        let mut noise_variable_55 = 0.0;
        let mut noise_variable_56 = 0.0;
        let mut noise_variable_57 = 0.0;
        let mut noise_variable_58 = 0.0;
        let mut noise_variable_59 = 0.0;
        let mut noise_variable_60 = 0.0;
        let mut noise_variable_61 = 0.0;
        let mut noise_variable_62 = 0.0;
        let mut noise_variable_63 = 0.0;
        let mut noise_variable_64 = 0.0;
        let mut noise_variable_65 = 0.0;
        let mut noise_variable_66 = 0.0;
        let mut noise_variable_67 = 0.0;
        let mut noise_variable_68 = 0.0;
        let mut noise_variable_69 = 0.0;
        let mut noise_variable_70 = 0.0;
        let mut noise_variable_71 = 0.0;
        let mut noise_variable_72 = 0.0;
        let mut noise_variable_73 = 0.0;
        let mut noise_variable_74 = 0.0;
        let mut noise_variable_75 = 0.0;
        let mut noise_variable_76 = 0.0;
        let mut noise_variable_77 = 0.0;
        let mut noise_variable_78 = 0.0;
        let mut noise_variable_79 = 0.0;
        let mut noise_variable_80 = 0.0;
        let mut noise_variable_81 = 0.0;
        let mut noise_variable_82 = 0.0;
        let mut noise_variable_83 = 0.0;
        let mut noise_variable_84 = 0.0;
        let mut noise_variable_85 = 0.0;
        let noise_source_active = match source_index {
            0 => {
                true
            }
            1 => {
                true
            }
            _ => unreachable!("noise source index was range checked"),
        };
        if !noise_source_active { return Ok(GeneratedNoiseEvaluation { active: false, psd: 0.0, exponent: None, table_operands: Vec::new() }); }
        noise_variable_0 = 0.0;
        noise_variable_1 = 0.0;
        noise_variable_2 = 0.0;
        noise_variable_3 = 0.0;
        noise_variable_4 = 0.0;
        noise_variable_5 = 0.0;
        noise_variable_6 = 0.0;
        noise_variable_7 = 0.0;
        noise_variable_8 = 0.0;
        noise_variable_9 = 0.0;
        noise_variable_10 = 0.0;
        noise_variable_11 = 0.0;
        noise_variable_12 = 0.0;
        noise_variable_13 = 0.0;
        noise_variable_14 = 0.0;
        noise_variable_15 = 0.0;
        noise_variable_16 = 0.0;
        noise_variable_17 = 0.0;
        noise_variable_18 = 0.0;
        noise_variable_19 = 0.0;
        noise_variable_20 = 0.0;
        noise_variable_21 = 0.0;
        noise_variable_22 = 0.0;
        noise_variable_23 = 0.0;
        noise_variable_24 = 0.0;
        noise_variable_25 = 0.0;
        noise_variable_26 = 0.0;
        noise_variable_27 = 0.0;
        noise_variable_28 = 0.0;
        noise_variable_29 = 0.0;
        noise_variable_30 = 0.0;
        noise_variable_31 = 0.0;
        noise_variable_32 = 0.0;
        noise_variable_33 = 0.0;
        noise_variable_34 = 0.0;
        noise_variable_35 = 0.0;
        noise_variable_36 = 0.0;
        noise_variable_37 = 0.0;
        noise_variable_38 = 0.0;
        noise_variable_39 = 0.0;
        noise_variable_40 = 0.0;
        noise_variable_41 = 0.0;
        noise_variable_42 = 0.0;
        noise_variable_43 = 0.0;
        noise_variable_44 = 0.0;
        noise_variable_45 = 0.0;
        noise_variable_46 = 0.0;
        noise_variable_47 = 0.0;
        noise_variable_48 = 0.0;
        noise_variable_49 = 0.0;
        noise_variable_50 = 0.0;
        noise_variable_51 = 0.0;
        noise_variable_52 = 0.0;
        noise_variable_53 = 0.0;
        noise_variable_54 = 0.0;
        noise_variable_55 = 0.0;
        noise_variable_56 = 0.0;
        noise_variable_57 = 0.0;
        noise_variable_58 = 0.0;
        noise_variable_59 = 0.0;
        noise_variable_60 = 0.0;
        noise_variable_61 = 0.0;
        noise_variable_62 = 0.0;
        noise_variable_63 = 0.0;
        noise_variable_64 = 0.0;
        noise_variable_65 = 0.0;
        noise_variable_66 = 0.0;
        noise_variable_67 = 0.0;
        noise_variable_68 = 0.0;
        noise_variable_69 = 0.0;
        noise_variable_70 = 0.0;
        noise_variable_71 = 0.0;
        noise_variable_72 = 0.0;
        noise_variable_73 = 0.0;
        noise_variable_74 = 0.0;
        noise_variable_75 = 0.0;
        noise_variable_76 = 0.0;
        noise_variable_77 = 0.0;
        noise_variable_78 = 0.0;
        noise_variable_79 = 0.0;
        noise_variable_80 = 0.0;
        noise_variable_81 = 0.0;
        noise_variable_82 = 0.0;
        noise_variable_83 = 0.0;
        noise_variable_84 = 0.0;
        noise_variable_85 = 0.0;
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_1_e60: f64 = if self.param_given[9] { 1.0 } else { 0.0 };
            noise_variable_39 = noise_metadata_schedule_1_e60;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_2_e64,) = {
    if (noise_variable_39 != 0.0) {
        (params.p9,)
    } else {
        (noise_variable_10,)
    }
};
            noise_variable_10 = noise_metadata_schedule_2_e64;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_3_e71,) = {
    if (noise_variable_39 == 0.0) {
        let noise_metadata_schedule_3_e69: f64 = 1.0;
        (noise_metadata_schedule_3_e69,)
    } else {
        (noise_variable_10,)
    }
};
            noise_variable_10 = noise_metadata_schedule_3_e71;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_4_e73: f64 = if self.param_given[10] { 1.0 } else { 0.0 };
            noise_variable_40 = noise_metadata_schedule_4_e73;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_5_e81,) = {
    if (noise_variable_40 != 0.0) {
        let noise_metadata_schedule_5_e78: f64 = (0.01 * params.p10);
        let noise_metadata_schedule_5_e79: f64 = (1.0 - noise_metadata_schedule_5_e78);
        (noise_metadata_schedule_5_e79,)
    } else {
        (noise_variable_11,)
    }
};
            noise_variable_11 = noise_metadata_schedule_5_e81;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_6_e92,) = {
    if (noise_variable_40 == 0.0) {
        let noise_metadata_schedule_6_e88: f64 = 0.0;
        let noise_metadata_schedule_6_e89: f64 = (0.01 * noise_metadata_schedule_6_e88);
        let noise_metadata_schedule_6_e90: f64 = (1.0 - noise_metadata_schedule_6_e89);
        (noise_metadata_schedule_6_e90,)
    } else {
        (noise_variable_11,)
    }
};
            noise_variable_11 = noise_metadata_schedule_6_e92;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_10_e108: f64 = (noise_variable_11 * noise_variable_10);
            let noise_metadata_schedule_10_e110: f64 = (noise_metadata_schedule_10_e108 * 1000000.0);
            noise_variable_15 = noise_metadata_schedule_10_e110;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_11_e113: f64 = (273.15 + params.p15);
            noise_variable_8 = noise_metadata_schedule_11_e113;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_12_e114: f64 = ctx.temperature();
            let noise_metadata_schedule_12_e116: f64 = (noise_metadata_schedule_12_e114 + params.p5);
            let noise_metadata_schedule_12_e118: f64 = (noise_metadata_schedule_12_e116 - 273.15);
            noise_variable_25 = noise_metadata_schedule_12_e118;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_15_e128: f64 = (params.p34 + 1.0);
            let noise_metadata_schedule_15_e129: f64 = if noise_variable_25 < noise_metadata_schedule_15_e128 { 1.0 } else { 0.0 };
            noise_variable_44 = noise_metadata_schedule_15_e129;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_16_e140,) = {
    if (noise_variable_44 != 0.0) {
        let noise_metadata_schedule_16_e134: f64 = (noise_variable_25 - params.p34);
        let noise_metadata_schedule_16_e136: f64 = (noise_metadata_schedule_16_e134 - 1.0);
        let noise_metadata_schedule_16_e137: f64 = (noise_metadata_schedule_16_e136).exp();
        let noise_metadata_schedule_16_e138: f64 = (params.p34 + noise_metadata_schedule_16_e137);
        (noise_metadata_schedule_16_e138,)
    } else {
        (noise_variable_25,)
    }
};
            noise_variable_25 = noise_metadata_schedule_16_e140;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_17_e144: f64 = (params.p35 - 1.0);
            let noise_metadata_schedule_17_e145: f64 = if noise_variable_25 > noise_metadata_schedule_17_e144 { 1.0 } else { 0.0 };
            noise_variable_45 = noise_metadata_schedule_17_e145;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_18_e159,) = {
    if ((noise_variable_44 == 0.0) && (noise_variable_45 != 0.0)) {
        let noise_metadata_schedule_18_e153: f64 = (params.p35 - noise_variable_25);
        let noise_metadata_schedule_18_e155: f64 = (noise_metadata_schedule_18_e153 - 1.0);
        let noise_metadata_schedule_18_e156: f64 = (noise_metadata_schedule_18_e155).exp();
        let noise_metadata_schedule_18_e157: f64 = (params.p35 - noise_metadata_schedule_18_e156);
        (noise_metadata_schedule_18_e157,)
    } else {
        (noise_variable_25,)
    }
};
            noise_variable_25 = noise_metadata_schedule_18_e159;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_19_e167,) = {
    if ((noise_variable_44 == 0.0) && (noise_variable_45 == 0.0)) {
        (noise_variable_25,)
    } else {
        (noise_variable_25,)
    }
};
            noise_variable_25 = noise_metadata_schedule_19_e167;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_20_e170: f64 = (noise_variable_25 + 273.15);
            noise_variable_9 = noise_metadata_schedule_20_e170;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_21_e173: f64 = (noise_variable_9 - noise_variable_8);
            noise_variable_12 = noise_metadata_schedule_21_e173;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_22_e177: f64 = (noise_variable_12 * params.p42);
            let noise_metadata_schedule_22_e178: f64 = (1.0 + noise_metadata_schedule_22_e177);
            let noise_metadata_schedule_22_e180: f64 = (noise_metadata_schedule_22_e178 * params.p29);
            noise_variable_22 = noise_metadata_schedule_22_e180;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_23_e183: f64 = if noise_variable_22 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_46 = noise_metadata_schedule_23_e183;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_24_e187,) = {
    if (noise_variable_46 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_24_e187;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_25_e190: f64 = if ((params.p3 != 0.0) && (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            noise_variable_47 = noise_metadata_schedule_25_e190;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_26_e194,) = {
    if (noise_variable_47 != 0.0) {
        (params.p22,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_26_e194;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_27_e197: f64 = if ((params.p3 != 0.0) || (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            noise_variable_48 = noise_metadata_schedule_27_e197;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_28_e206,) = {
    if ((noise_variable_47 == 0.0) && (noise_variable_48 != 0.0)) {
        let noise_metadata_schedule_28_e204: f64 = (params.p22 * 0.5);
        (noise_metadata_schedule_28_e204,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_28_e206;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_29_e214,) = {
    if ((noise_variable_47 == 0.0) && (noise_variable_48 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_29_e214;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_30_e223: f64 = if ((self.param_given[1] && self.param_given[2]) && (!self.param_given[0])) { 1.0 } else { 0.0 };
            noise_variable_49 = noise_metadata_schedule_30_e223;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_31_e230: f64 = if ((params.p2 == 0.0) || (params.p1 == 0.0)) { 1.0 } else { 0.0 };
            noise_variable_50 = noise_metadata_schedule_31_e230;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_32_e236,) = {
    if ((noise_variable_49 != 0.0) && (noise_variable_50 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_32_e236;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_33_e242,) = {
    if ((noise_variable_49 != 0.0) && (noise_variable_50 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_33_e242;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_34_e250,) = {
    if ((noise_variable_49 != 0.0) && (noise_variable_50 != 0.0)) {
        let noise_metadata_schedule_34_e248: f64 = (params.p0 * noise_variable_15);
        (noise_metadata_schedule_34_e248,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_34_e250;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_35_e258,) = {
    if ((noise_variable_49 != 0.0) && (noise_variable_50 != 0.0)) {
        let noise_metadata_schedule_35_e256: f64 = (noise_variable_17 + params.p21);
        (noise_metadata_schedule_35_e256,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_35_e258;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_36_e264,) = {
    if ((noise_variable_49 != 0.0) && (noise_variable_50 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_36_e264;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_37_e270,) = {
    if ((noise_variable_49 != 0.0) && (noise_variable_50 != 0.0)) {
        (1e99,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_37_e270;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_38_e279,) = {
    if ((noise_variable_49 != 0.0) && (noise_variable_50 == 0.0)) {
        let noise_metadata_schedule_38_e277: f64 = (params.p1 * noise_variable_15);
        (noise_metadata_schedule_38_e277,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_38_e279;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_39_e288,) = {
    if ((noise_variable_49 != 0.0) && (noise_variable_50 == 0.0)) {
        let noise_metadata_schedule_39_e286: f64 = (noise_variable_16 + noise_variable_14);
        (noise_metadata_schedule_39_e286,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_39_e288;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_41_e294: f64 = if noise_variable_3 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_52 = noise_metadata_schedule_41_e294;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_42_e307,) = {
    if (((noise_variable_49 != 0.0) && (noise_variable_50 == 0.0)) && (noise_variable_52 != 0.0)) {
        let noise_metadata_schedule_42_e303: f64 = (params.p16 / params.p2);
        let noise_metadata_schedule_42_e305: f64 = (noise_metadata_schedule_42_e303 * noise_variable_3);
        (noise_metadata_schedule_42_e305,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_42_e307;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_43_e318,) = {
    if (((noise_variable_49 != 0.0) && (noise_variable_50 == 0.0)) && (noise_variable_52 != 0.0)) {
        let noise_metadata_schedule_43_e316: f64 = (noise_variable_4 - params.p21);
        (noise_metadata_schedule_43_e316,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_43_e318;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_45_e330,) = {
    if (((noise_variable_49 != 0.0) && (noise_variable_50 == 0.0)) && (noise_variable_52 != 0.0)) {
        (params.p2,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_45_e330;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_46_e341,) = {
    if (((noise_variable_49 != 0.0) && (noise_variable_50 == 0.0)) && (noise_variable_52 != 0.0)) {
        let noise_metadata_schedule_46_e339: f64 = (1.0 / noise_variable_5);
        (noise_metadata_schedule_46_e339,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_46_e341;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_47_e353,) = {
    if (((noise_variable_49 != 0.0) && (noise_variable_50 == 0.0)) && (noise_variable_52 == 0.0)) {
        let noise_metadata_schedule_47_e351: f64 = (params.p0 * noise_variable_15);
        (noise_metadata_schedule_47_e351,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_47_e353;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_48_e365,) = {
    if (((noise_variable_49 != 0.0) && (noise_variable_50 == 0.0)) && (noise_variable_52 == 0.0)) {
        let noise_metadata_schedule_48_e363: f64 = (noise_variable_17 + params.p21);
        (noise_metadata_schedule_48_e363,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_48_e365;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_49_e375,) = {
    if (((noise_variable_49 != 0.0) && (noise_variable_50 == 0.0)) && (noise_variable_52 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_49_e375;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_50_e385,) = {
    if (((noise_variable_49 != 0.0) && (noise_variable_50 == 0.0)) && (noise_variable_52 == 0.0)) {
        (1e99,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_50_e385;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_51_e391: f64 = if (self.param_given[2] && (!self.param_given[1])) { 1.0 } else { 0.0 };
            noise_variable_54 = noise_metadata_schedule_51_e391;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_52_e394: f64 = if params.p2 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_55 = noise_metadata_schedule_52_e394;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_53_e403,) = {
    if (((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_53_e403;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_54_e412,) = {
    if (((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_54_e412;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_55_e423,) = {
    if (((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 != 0.0)) {
        let noise_metadata_schedule_55_e421: f64 = (params.p0 * noise_variable_15);
        (noise_metadata_schedule_55_e421,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_55_e423;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_56_e434,) = {
    if (((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 != 0.0)) {
        let noise_metadata_schedule_56_e432: f64 = (noise_variable_17 + params.p21);
        (noise_metadata_schedule_56_e432,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_56_e434;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_57_e443,) = {
    if (((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_57_e443;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_58_e452,) = {
    if (((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 != 0.0)) {
        (1e99,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_58_e452;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_59_e455: f64 = if params.p0 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_56 = noise_metadata_schedule_59_e455;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_60_e467,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_60_e467;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_61_e479,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_61_e479;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_62_e493,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 != 0.0)) {
        let noise_metadata_schedule_62_e491: f64 = (params.p1 * noise_variable_15);
        (noise_metadata_schedule_62_e491,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_62_e493;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_63_e507,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 != 0.0)) {
        let noise_metadata_schedule_63_e505: f64 = (noise_variable_16 + noise_variable_14);
        (noise_metadata_schedule_63_e505,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_63_e507;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_64_e519,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 != 0.0)) {
        (1e99,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_64_e519;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_65_e531,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_65_e531;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_66_e546,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 == 0.0)) {
        let noise_metadata_schedule_66_e544: f64 = (params.p0 * noise_variable_15);
        (noise_metadata_schedule_66_e544,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_66_e546;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_67_e561,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 == 0.0)) {
        let noise_metadata_schedule_67_e559: f64 = (noise_variable_17 + params.p21);
        (noise_metadata_schedule_67_e559,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_67_e561;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_69_e567: f64 = if noise_variable_4 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_58 = noise_metadata_schedule_69_e567;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_70_e586,) = {
    if (((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 == 0.0)) && (noise_variable_58 != 0.0)) {
        let noise_metadata_schedule_70_e582: f64 = (params.p2 / params.p16);
        let noise_metadata_schedule_70_e584: f64 = (noise_metadata_schedule_70_e582 * noise_variable_4);
        (noise_metadata_schedule_70_e584,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_70_e586;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_71_e603,) = {
    if (((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 == 0.0)) && (noise_variable_58 != 0.0)) {
        let noise_metadata_schedule_71_e601: f64 = (noise_variable_3 - noise_variable_14);
        (noise_metadata_schedule_71_e601,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_71_e603;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_73_e621,) = {
    if (((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 == 0.0)) && (noise_variable_58 != 0.0)) {
        (params.p2,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_73_e621;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_74_e638,) = {
    if (((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 == 0.0)) && (noise_variable_58 != 0.0)) {
        let noise_metadata_schedule_74_e636: f64 = (1.0 / noise_variable_5);
        (noise_metadata_schedule_74_e636,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_74_e638;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_75_e656,) = {
    if (((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 == 0.0)) && (noise_variable_58 == 0.0)) {
        let noise_metadata_schedule_75_e654: f64 = (params.p1 * noise_variable_15);
        (noise_metadata_schedule_75_e654,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_75_e656;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_76_e674,) = {
    if (((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 == 0.0)) && (noise_variable_58 == 0.0)) {
        let noise_metadata_schedule_76_e672: f64 = (noise_variable_16 + noise_variable_14);
        (noise_metadata_schedule_76_e672,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_76_e674;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_77_e690,) = {
    if (((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 == 0.0)) && (noise_variable_58 == 0.0)) {
        (1e99,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_77_e690;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_78_e706,) = {
    if (((((noise_variable_49 == 0.0) && (noise_variable_54 != 0.0)) && (noise_variable_55 == 0.0)) && (noise_variable_56 == 0.0)) && (noise_variable_58 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_78_e706;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_79_e709: f64 = if params.p0 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_60 = noise_metadata_schedule_79_e709;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_80_e719,) = {
    if (((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_80_e719;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_81_e729,) = {
    if (((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_81_e729;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_82_e741,) = {
    if (((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 != 0.0)) {
        let noise_metadata_schedule_82_e739: f64 = (params.p1 * noise_variable_15);
        (noise_metadata_schedule_82_e739,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_82_e741;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_83_e753,) = {
    if (((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 != 0.0)) {
        let noise_metadata_schedule_83_e751: f64 = (noise_variable_16 + noise_variable_14);
        (noise_metadata_schedule_83_e751,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_83_e753;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_84_e763,) = {
    if (((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 != 0.0)) {
        (1e99,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_84_e763;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_85_e773,) = {
    if (((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_85_e773;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_86_e776: f64 = if params.p1 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_61 = noise_metadata_schedule_86_e776;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_87_e789,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_87_e789;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_88_e802,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_88_e802;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_89_e817,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 != 0.0)) {
        let noise_metadata_schedule_89_e815: f64 = (params.p0 * noise_variable_15);
        (noise_metadata_schedule_89_e815,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_89_e817;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_90_e832,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 != 0.0)) {
        let noise_metadata_schedule_90_e830: f64 = (noise_variable_17 + params.p21);
        (noise_metadata_schedule_90_e830,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_90_e832;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_91_e845,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_91_e845;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_92_e858,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 != 0.0)) {
        (1e99,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_92_e858;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_93_e874,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 == 0.0)) {
        let noise_metadata_schedule_93_e872: f64 = (params.p0 * noise_variable_15);
        (noise_metadata_schedule_93_e872,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_93_e874;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_94_e890,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 == 0.0)) {
        let noise_metadata_schedule_94_e888: f64 = (noise_variable_17 + params.p21);
        (noise_metadata_schedule_94_e888,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_94_e890;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_96_e909,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 == 0.0)) {
        let noise_metadata_schedule_96_e907: f64 = (params.p1 * noise_variable_15);
        (noise_metadata_schedule_96_e907,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_96_e909;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_97_e925,) = {
    if ((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 == 0.0)) {
        let noise_metadata_schedule_97_e923: f64 = (noise_variable_16 + noise_variable_14);
        (noise_metadata_schedule_97_e923,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_97_e925;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_98_e928: f64 = if noise_variable_4 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_63 = noise_metadata_schedule_98_e928;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_100_e934: f64 = if noise_variable_3 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_65 = noise_metadata_schedule_100_e934;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_101_e956,) = {
    if ((((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 == 0.0)) && (noise_variable_63 != 0.0)) && (noise_variable_65 != 0.0)) {
        let noise_metadata_schedule_101_e953: f64 = (noise_variable_3 / noise_variable_4);
        let noise_metadata_schedule_101_e954: f64 = (params.p16 * noise_metadata_schedule_101_e953);
        (noise_metadata_schedule_101_e954,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_101_e956;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_102_e976,) = {
    if ((((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 == 0.0)) && (noise_variable_63 != 0.0)) && (noise_variable_65 != 0.0)) {
        let noise_metadata_schedule_102_e974: f64 = (1.0 / noise_variable_5);
        (noise_metadata_schedule_102_e974,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_102_e976;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_103_e995,) = {
    if ((((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 == 0.0)) && (noise_variable_63 != 0.0)) && (noise_variable_65 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_103_e995;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_104_e1014,) = {
    if ((((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 == 0.0)) && (noise_variable_63 != 0.0)) && (noise_variable_65 == 0.0)) {
        (1e99,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_104_e1014;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_105_e1031,) = {
    if (((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 == 0.0)) && (noise_variable_63 == 0.0)) {
        (1e99,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_105_e1031;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_106_e1048,) = {
    if (((((noise_variable_49 == 0.0) && (noise_variable_54 == 0.0)) && (noise_variable_60 == 0.0)) && (noise_variable_61 == 0.0)) && (noise_variable_63 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_106_e1048;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_111_e1066,) = {
    if (params.p24 != 0.0) {
        let noise_metadata_schedule_111_e1064: f64 = (noise_variable_3 + params.p23);
        (noise_metadata_schedule_111_e1064,)
    } else {
        (noise_variable_18,)
    }
};
            noise_variable_18 = noise_metadata_schedule_111_e1066;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_112_e1073,) = {
    if (params.p24 == 0.0) {
        let noise_metadata_schedule_112_e1071: f64 = (noise_variable_16 + params.p23);
        (noise_metadata_schedule_112_e1071,)
    } else {
        (noise_variable_18,)
    }
};
            noise_variable_18 = noise_metadata_schedule_112_e1073;
        }
        if matches!(source_index, 0 | 1) {
            noise_variable_34 = params.p36;
        }
        if matches!(source_index, 0 | 1) {
            noise_variable_35 = params.p37;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_116_e1093: f64 = if noise_variable_3 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_71 = noise_metadata_schedule_116_e1093;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_117_e1096: f64 = if ((params.p3 != 0.0) && (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            noise_variable_72 = noise_metadata_schedule_117_e1096;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_118_e1106,) = {
    if ((noise_variable_71 != 0.0) && (noise_variable_72 != 0.0)) {
        let noise_metadata_schedule_118_e1103: f64 = (params.p38 / noise_variable_3);
        let noise_metadata_schedule_118_e1104: f64 = (noise_variable_34 + noise_metadata_schedule_118_e1103);
        (noise_metadata_schedule_118_e1104,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_118_e1106;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_119_e1116,) = {
    if ((noise_variable_71 != 0.0) && (noise_variable_72 != 0.0)) {
        let noise_metadata_schedule_119_e1113: f64 = (params.p39 / noise_variable_3);
        let noise_metadata_schedule_119_e1114: f64 = (noise_variable_35 + noise_metadata_schedule_119_e1113);
        (noise_metadata_schedule_119_e1114,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_119_e1116;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_120_e1119: f64 = if ((params.p3 != 0.0) || (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            noise_variable_73 = noise_metadata_schedule_120_e1119;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_121_e1134,) = {
    if (((noise_variable_71 != 0.0) && (noise_variable_72 == 0.0)) && (noise_variable_73 != 0.0)) {
        let noise_metadata_schedule_121_e1129: f64 = (0.5 * params.p38);
        let noise_metadata_schedule_121_e1131: f64 = (noise_metadata_schedule_121_e1129 / noise_variable_3);
        let noise_metadata_schedule_121_e1132: f64 = (noise_variable_34 + noise_metadata_schedule_121_e1131);
        (noise_metadata_schedule_121_e1132,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_121_e1134;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_122_e1149,) = {
    if (((noise_variable_71 != 0.0) && (noise_variable_72 == 0.0)) && (noise_variable_73 != 0.0)) {
        let noise_metadata_schedule_122_e1144: f64 = (0.5 * params.p39);
        let noise_metadata_schedule_122_e1146: f64 = (noise_metadata_schedule_122_e1144 / noise_variable_3);
        let noise_metadata_schedule_122_e1147: f64 = (noise_variable_35 + noise_metadata_schedule_122_e1146);
        (noise_metadata_schedule_122_e1147,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_122_e1149;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_123_e1152: f64 = if noise_variable_4 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_74 = noise_metadata_schedule_123_e1152;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_124_e1160,) = {
    if (noise_variable_74 != 0.0) {
        let noise_metadata_schedule_124_e1157: f64 = (params.p40 / noise_variable_4);
        let noise_metadata_schedule_124_e1158: f64 = (noise_variable_34 + noise_metadata_schedule_124_e1157);
        (noise_metadata_schedule_124_e1158,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_124_e1160;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_125_e1168,) = {
    if (noise_variable_74 != 0.0) {
        let noise_metadata_schedule_125_e1165: f64 = (params.p41 / noise_variable_4);
        let noise_metadata_schedule_125_e1166: f64 = (noise_variable_35 + noise_metadata_schedule_125_e1165);
        (noise_metadata_schedule_125_e1166,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_125_e1168;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_129_e1188: f64 = (noise_variable_12 * noise_variable_35);
            let noise_metadata_schedule_129_e1189: f64 = (noise_variable_34 + noise_metadata_schedule_129_e1188);
            let noise_metadata_schedule_129_e1190: f64 = (noise_variable_12 * noise_metadata_schedule_129_e1189);
            let noise_metadata_schedule_129_e1191: f64 = (1.0 + noise_metadata_schedule_129_e1190);
            noise_variable_13 = noise_metadata_schedule_129_e1191;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_130_e1195: f64 = (0.01 + 0.1);
            let noise_metadata_schedule_130_e1196: f64 = if noise_variable_13 < noise_metadata_schedule_130_e1195 { 1.0 } else { 0.0 };
            noise_variable_76 = noise_metadata_schedule_130_e1196;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_131_e1211,) = {
    if (noise_variable_76 != 0.0) {
        let noise_metadata_schedule_131_e1203: f64 = (noise_variable_13 - 0.01);
        let noise_metadata_schedule_131_e1204: f64 = (10.0 * noise_metadata_schedule_131_e1203);
        let noise_metadata_schedule_131_e1206: f64 = (noise_metadata_schedule_131_e1204 - 1.0);
        let noise_metadata_schedule_131_e1207: f64 = (noise_metadata_schedule_131_e1206).exp();
        let noise_metadata_schedule_131_e1208: f64 = (0.1 * noise_metadata_schedule_131_e1207);
        let noise_metadata_schedule_131_e1209: f64 = (0.01 + noise_metadata_schedule_131_e1208);
        (noise_metadata_schedule_131_e1209,)
    } else {
        (noise_variable_13,)
    }
};
            noise_variable_13 = noise_metadata_schedule_131_e1211;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_132_e1216,) = {
    if (noise_variable_76 == 0.0) {
        (noise_variable_13,)
    } else {
        (noise_variable_13,)
    }
};
            noise_variable_13 = noise_metadata_schedule_132_e1216;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_133_e1219: f64 = (noise_variable_5 * noise_variable_13);
            noise_variable_20 = noise_metadata_schedule_133_e1219;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_134_e1222: f64 = (noise_variable_19 / noise_variable_13);
            noise_variable_21 = noise_metadata_schedule_134_e1222;
        }
        if matches!(source_index, 0 | 1) {
            noise_variable_30 = (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[1]));
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_136_e1234: f64 = if ((noise_variable_5 > 0.0) && ((params.p28 > 0.0) || (params.p26 > 0.0))) { 1.0 } else { 0.0 };
            noise_variable_77 = noise_metadata_schedule_136_e1234;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_137_e1240,) = {
    if (noise_variable_77 != 0.0) {
        let noise_metadata_schedule_137_e1238: f64 = (noise_variable_30 / noise_variable_18);
        (noise_metadata_schedule_137_e1238,)
    } else {
        (noise_variable_31,)
    }
};
            noise_variable_31 = noise_metadata_schedule_137_e1240;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_138_e1246,) = {
    if (noise_variable_77 != 0.0) {
        let noise_metadata_schedule_138_e1244: f64 = (params.p27 * noise_variable_31);
        (noise_metadata_schedule_138_e1244,)
    } else {
        (noise_variable_32,)
    }
};
            noise_variable_32 = noise_metadata_schedule_138_e1246;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_139_e1255,) = {
    if (noise_variable_77 != 0.0) {
        let noise_metadata_schedule_139_e1251: f64 = (noise_variable_32 * noise_variable_32);
        let noise_metadata_schedule_139_e1252: f64 = (1.0 + noise_metadata_schedule_139_e1251);
        let noise_metadata_schedule_139_e1253: f64 = (noise_metadata_schedule_139_e1252).sqrt();
        (noise_metadata_schedule_139_e1253,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_139_e1255;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_140_e1262,) = {
    if (noise_variable_77 != 0.0) {
        let noise_metadata_schedule_140_e1259: f64 = (noise_variable_31).abs();
        let noise_metadata_schedule_140_e1260: f64 = (params.p25 * noise_metadata_schedule_140_e1259);
        (noise_metadata_schedule_140_e1260,)
    } else {
        (noise_variable_33,)
    }
};
            noise_variable_33 = noise_metadata_schedule_140_e1262;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_141_e1274,) = {
    if (noise_variable_77 != 0.0) {
        let noise_metadata_schedule_141_e1267: f64 = (noise_variable_33 * noise_variable_33);
        let noise_metadata_schedule_141_e1269: f64 = (noise_metadata_schedule_141_e1267 * noise_variable_33);
        let noise_metadata_schedule_141_e1270: f64 = (1.0 + noise_metadata_schedule_141_e1269);
        let noise_metadata_schedule_141_e1272: f64 = (noise_metadata_schedule_141_e1270).powf(0.3333333333333333);
        (noise_metadata_schedule_141_e1272,)
    } else {
        (noise_variable_24,)
    }
};
            noise_variable_24 = noise_metadata_schedule_141_e1274;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_142_e1290,) = {
    if (noise_variable_77 != 0.0) {
        let noise_metadata_schedule_142_e1278: f64 = (1.0 - params.p28);
        let noise_metadata_schedule_142_e1280: f64 = (noise_metadata_schedule_142_e1278 - params.p26);
        let noise_metadata_schedule_142_e1283: f64 = (params.p28 * noise_variable_23);
        let noise_metadata_schedule_142_e1284: f64 = (noise_metadata_schedule_142_e1280 + noise_metadata_schedule_142_e1283);
        let noise_metadata_schedule_142_e1287: f64 = (params.p26 * noise_variable_24);
        let noise_metadata_schedule_142_e1288: f64 = (noise_metadata_schedule_142_e1284 + noise_metadata_schedule_142_e1287);
        (noise_metadata_schedule_142_e1288,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_142_e1290;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_143_e1295,) = {
    if (noise_variable_77 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_143_e1295;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_144_e1298: f64 = (noise_variable_20 * noise_variable_29);
            noise_variable_6 = noise_metadata_schedule_144_e1298;
        }
        if matches!(source_index, 1) {
            noise_variable_0 = noise_variable_30;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_146_e1302: f64 = (noise_variable_0 / noise_variable_6);
            noise_variable_1 = noise_metadata_schedule_146_e1302;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_149_e1320: f64 = if (((params.p6 != 0.0) && (noise_variable_5 > 0.0)) && (noise_variable_19 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_80 = noise_metadata_schedule_149_e1320;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_150_e1332,) = {
    if (noise_variable_80 != 0.0) {
        let noise_metadata_schedule_150_e1324: f64 = (4.0 * 1.3806505e-23);
        let noise_metadata_schedule_150_e1326: f64 = (noise_metadata_schedule_150_e1324 * noise_variable_9);
        let noise_metadata_schedule_150_e1328: f64 = (noise_metadata_schedule_150_e1326 * noise_variable_21);
        let noise_metadata_schedule_150_e1330: f64 = (noise_metadata_schedule_150_e1328 / noise_variable_29);
        (noise_metadata_schedule_150_e1330,)
    } else {
        (noise_variable_26,)
    }
};
            noise_variable_26 = noise_metadata_schedule_150_e1332;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_151_e1341: f64 = if (((params.p32 != 0.0) && (noise_variable_3 > 0.0)) && (noise_variable_4 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_81 = noise_metadata_schedule_151_e1341;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_152_e1358,) = {
    if ((noise_variable_80 != 0.0) && (noise_variable_81 != 0.0)) {
        let noise_metadata_schedule_152_e1348: f64 = (noise_variable_1 / noise_variable_4);
        let noise_metadata_schedule_152_e1349: f64 = (noise_metadata_schedule_152_e1348).abs();
        let noise_metadata_schedule_152_e1351: f64 = (noise_metadata_schedule_152_e1349).powf(params.p30);
        let noise_metadata_schedule_152_e1352: f64 = (noise_variable_22 * noise_metadata_schedule_152_e1351);
        let noise_metadata_schedule_152_e1354: f64 = (noise_metadata_schedule_152_e1352 * noise_variable_4);
        let noise_metadata_schedule_152_e1356: f64 = (noise_metadata_schedule_152_e1354 / noise_variable_3);
        (noise_metadata_schedule_152_e1356,)
    } else {
        (noise_variable_27,)
    }
};
            noise_variable_27 = noise_metadata_schedule_152_e1358;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_153_e1365: f64 = if ((noise_variable_16 > 0.0) && (noise_variable_17 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_82 = noise_metadata_schedule_153_e1365;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_154_e1385,) = {
    if (((noise_variable_80 != 0.0) && (noise_variable_81 == 0.0)) && (noise_variable_82 != 0.0)) {
        let noise_metadata_schedule_154_e1375: f64 = (noise_variable_1 / noise_variable_17);
        let noise_metadata_schedule_154_e1376: f64 = (noise_metadata_schedule_154_e1375).abs();
        let noise_metadata_schedule_154_e1378: f64 = (noise_metadata_schedule_154_e1376).powf(params.p30);
        let noise_metadata_schedule_154_e1379: f64 = (noise_variable_22 * noise_metadata_schedule_154_e1378);
        let noise_metadata_schedule_154_e1381: f64 = (noise_metadata_schedule_154_e1379 * noise_variable_17);
        let noise_metadata_schedule_154_e1383: f64 = (noise_metadata_schedule_154_e1381 / noise_variable_16);
        (noise_metadata_schedule_154_e1383,)
    } else {
        (noise_variable_27,)
    }
};
            noise_variable_27 = noise_metadata_schedule_154_e1385;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_155_e1395,) = {
    if (((noise_variable_80 != 0.0) && (noise_variable_81 == 0.0)) && (noise_variable_82 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_27,)
    }
};
            noise_variable_27 = noise_metadata_schedule_155_e1395;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_156_e1398: f64 = if noise_variable_1 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_83 = noise_metadata_schedule_156_e1398;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_157_e1405,) = {
    if ((noise_variable_80 != 0.0) && (noise_variable_83 != 0.0)) {
        let noise_metadata_schedule_157_e1403: f64 = (-noise_variable_27);
        (noise_metadata_schedule_157_e1403,)
    } else {
        (noise_variable_27,)
    }
};
            noise_variable_27 = noise_metadata_schedule_157_e1405;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_158_e1410,) = {
    if (noise_variable_80 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_26,)
    }
};
            noise_variable_26 = noise_metadata_schedule_158_e1410;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_159_e1415,) = {
    if (noise_variable_80 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_27,)
    }
};
            noise_variable_27 = noise_metadata_schedule_159_e1415;
        }
        match source_index {
            0 => {
                let noise_0_psd_e1467: f64 = 1.0;
                let noise_0_psd_e1468: f64 = (noise_0_psd_e1467 * noise_variable_26);
                let psd = noise_0_psd_e1468;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            1 => {
                let noise_1_psd_e1470: f64 = 1.0;
                let noise_1_psd_e1471: f64 = (noise_1_psd_e1470 * noise_variable_27);
                let psd = noise_1_psd_e1471;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
                let exponent: Option<f64> = Some(params.p31);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            _ => unreachable!("noise source index was range checked"),
        }
    }
}

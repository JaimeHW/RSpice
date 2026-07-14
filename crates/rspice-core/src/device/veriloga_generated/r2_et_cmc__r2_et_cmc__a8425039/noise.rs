#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_N2_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 5, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_N1_N2_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 6, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
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
        let mut noise_variable_86 = 0.0;
        let mut noise_variable_87 = 0.0;
        let mut noise_variable_88 = 0.0;
        let mut noise_variable_89 = 0.0;
        let mut noise_variable_90 = 0.0;
        let mut noise_variable_91 = 0.0;
        let mut noise_variable_92 = 0.0;
        let mut noise_variable_93 = 0.0;
        let mut noise_variable_94 = 0.0;
        let mut noise_variable_95 = 0.0;
        let mut noise_variable_96 = 0.0;
        let mut noise_variable_97 = 0.0;
        let mut noise_variable_98 = 0.0;
        let mut noise_variable_99 = 0.0;
        let mut noise_variable_100 = 0.0;
        let mut noise_variable_101 = 0.0;
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
        noise_variable_86 = 0.0;
        noise_variable_87 = 0.0;
        noise_variable_88 = 0.0;
        noise_variable_89 = 0.0;
        noise_variable_90 = 0.0;
        noise_variable_91 = 0.0;
        noise_variable_92 = 0.0;
        noise_variable_93 = 0.0;
        noise_variable_94 = 0.0;
        noise_variable_95 = 0.0;
        noise_variable_96 = 0.0;
        noise_variable_97 = 0.0;
        noise_variable_98 = 0.0;
        noise_variable_99 = 0.0;
        noise_variable_100 = 0.0;
        noise_variable_101 = 0.0;
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_1_e87: f64 = if self.param_given[10] { 1.0 } else { 0.0 };
            noise_variable_51 = noise_metadata_schedule_1_e87;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_2_e91,) = {
    if (noise_variable_51 != 0.0) {
        (params.p10,)
    } else {
        (noise_variable_13,)
    }
};
            noise_variable_13 = noise_metadata_schedule_2_e91;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_3_e98,) = {
    if (noise_variable_51 == 0.0) {
        let noise_metadata_schedule_3_e96: f64 = 1.0;
        (noise_metadata_schedule_3_e96,)
    } else {
        (noise_variable_13,)
    }
};
            noise_variable_13 = noise_metadata_schedule_3_e98;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_4_e100: f64 = if self.param_given[11] { 1.0 } else { 0.0 };
            noise_variable_52 = noise_metadata_schedule_4_e100;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_5_e108,) = {
    if (noise_variable_52 != 0.0) {
        let noise_metadata_schedule_5_e105: f64 = (0.01 * params.p11);
        let noise_metadata_schedule_5_e106: f64 = (1.0 - noise_metadata_schedule_5_e105);
        (noise_metadata_schedule_5_e106,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_5_e108;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_6_e119,) = {
    if (noise_variable_52 == 0.0) {
        let noise_metadata_schedule_6_e115: f64 = 0.0;
        let noise_metadata_schedule_6_e116: f64 = (0.01 * noise_metadata_schedule_6_e115);
        let noise_metadata_schedule_6_e117: f64 = (1.0 - noise_metadata_schedule_6_e116);
        (noise_metadata_schedule_6_e117,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_6_e119;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_10_e135: f64 = (noise_variable_14 * noise_variable_13);
            let noise_metadata_schedule_10_e137: f64 = (noise_metadata_schedule_10_e135 * 1000000.0);
            noise_variable_18 = noise_metadata_schedule_10_e137;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_11_e140: f64 = (273.15 + params.p16);
            noise_variable_11 = noise_metadata_schedule_11_e140;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_12_e141: f64 = ctx.temperature();
            let noise_metadata_schedule_12_e143: f64 = (noise_metadata_schedule_12_e141 + params.p5);
            let noise_metadata_schedule_12_e145: f64 = (noise_metadata_schedule_12_e143 - 273.15);
            noise_variable_28 = noise_metadata_schedule_12_e145;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_15_e154: f64 = if ((params.p3 != 0.0) && (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            noise_variable_56 = noise_metadata_schedule_15_e154;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_16_e158,) = {
    if (noise_variable_56 != 0.0) {
        (params.p23,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_16_e158;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_17_e161: f64 = if ((params.p3 != 0.0) || (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            noise_variable_57 = noise_metadata_schedule_17_e161;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_18_e170,) = {
    if ((noise_variable_56 == 0.0) && (noise_variable_57 != 0.0)) {
        let noise_metadata_schedule_18_e168: f64 = (params.p23 * 0.5);
        (noise_metadata_schedule_18_e168,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_18_e170;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_19_e178,) = {
    if ((noise_variable_56 == 0.0) && (noise_variable_57 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_19_e178;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_20_e187: f64 = if ((self.param_given[1] && self.param_given[2]) && (!self.param_given[0])) { 1.0 } else { 0.0 };
            noise_variable_58 = noise_metadata_schedule_20_e187;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_21_e194: f64 = if ((params.p2 == 0.0) || (params.p1 == 0.0)) { 1.0 } else { 0.0 };
            noise_variable_59 = noise_metadata_schedule_21_e194;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_22_e200,) = {
    if ((noise_variable_58 != 0.0) && (noise_variable_59 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_22_e200;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_23_e206,) = {
    if ((noise_variable_58 != 0.0) && (noise_variable_59 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_23_e206;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_24_e214,) = {
    if ((noise_variable_58 != 0.0) && (noise_variable_59 != 0.0)) {
        let noise_metadata_schedule_24_e212: f64 = (params.p0 * noise_variable_18);
        (noise_metadata_schedule_24_e212,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_24_e214;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_25_e222,) = {
    if ((noise_variable_58 != 0.0) && (noise_variable_59 != 0.0)) {
        let noise_metadata_schedule_25_e220: f64 = (noise_variable_20 + params.p22);
        (noise_metadata_schedule_25_e220,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_25_e222;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_26_e228,) = {
    if ((noise_variable_58 != 0.0) && (noise_variable_59 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_26_e228;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_27_e234,) = {
    if ((noise_variable_58 != 0.0) && (noise_variable_59 != 0.0)) {
        (1e99,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_27_e234;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_28_e243,) = {
    if ((noise_variable_58 != 0.0) && (noise_variable_59 == 0.0)) {
        let noise_metadata_schedule_28_e241: f64 = (params.p1 * noise_variable_18);
        (noise_metadata_schedule_28_e241,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_28_e243;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_29_e252,) = {
    if ((noise_variable_58 != 0.0) && (noise_variable_59 == 0.0)) {
        let noise_metadata_schedule_29_e250: f64 = (noise_variable_19 + noise_variable_17);
        (noise_metadata_schedule_29_e250,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_29_e252;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_31_e258: f64 = if noise_variable_3 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_61 = noise_metadata_schedule_31_e258;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_32_e271,) = {
    if (((noise_variable_58 != 0.0) && (noise_variable_59 == 0.0)) && (noise_variable_61 != 0.0)) {
        let noise_metadata_schedule_32_e267: f64 = (params.p17 / params.p2);
        let noise_metadata_schedule_32_e269: f64 = (noise_metadata_schedule_32_e267 * noise_variable_3);
        (noise_metadata_schedule_32_e269,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_32_e271;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_33_e282,) = {
    if (((noise_variable_58 != 0.0) && (noise_variable_59 == 0.0)) && (noise_variable_61 != 0.0)) {
        let noise_metadata_schedule_33_e280: f64 = (noise_variable_4 - params.p22);
        (noise_metadata_schedule_33_e280,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_33_e282;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_35_e294,) = {
    if (((noise_variable_58 != 0.0) && (noise_variable_59 == 0.0)) && (noise_variable_61 != 0.0)) {
        (params.p2,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_35_e294;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_36_e305,) = {
    if (((noise_variable_58 != 0.0) && (noise_variable_59 == 0.0)) && (noise_variable_61 != 0.0)) {
        let noise_metadata_schedule_36_e303: f64 = (1.0 / noise_variable_5);
        (noise_metadata_schedule_36_e303,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_36_e305;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_37_e317,) = {
    if (((noise_variable_58 != 0.0) && (noise_variable_59 == 0.0)) && (noise_variable_61 == 0.0)) {
        let noise_metadata_schedule_37_e315: f64 = (params.p0 * noise_variable_18);
        (noise_metadata_schedule_37_e315,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_37_e317;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_38_e329,) = {
    if (((noise_variable_58 != 0.0) && (noise_variable_59 == 0.0)) && (noise_variable_61 == 0.0)) {
        let noise_metadata_schedule_38_e327: f64 = (noise_variable_20 + params.p22);
        (noise_metadata_schedule_38_e327,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_38_e329;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_39_e339,) = {
    if (((noise_variable_58 != 0.0) && (noise_variable_59 == 0.0)) && (noise_variable_61 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_39_e339;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_40_e349,) = {
    if (((noise_variable_58 != 0.0) && (noise_variable_59 == 0.0)) && (noise_variable_61 == 0.0)) {
        (1e99,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_40_e349;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_41_e355: f64 = if (self.param_given[2] && (!self.param_given[1])) { 1.0 } else { 0.0 };
            noise_variable_63 = noise_metadata_schedule_41_e355;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_42_e358: f64 = if params.p2 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_64 = noise_metadata_schedule_42_e358;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_43_e367,) = {
    if (((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_43_e367;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_44_e376,) = {
    if (((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_44_e376;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_45_e387,) = {
    if (((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 != 0.0)) {
        let noise_metadata_schedule_45_e385: f64 = (params.p0 * noise_variable_18);
        (noise_metadata_schedule_45_e385,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_45_e387;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_46_e398,) = {
    if (((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 != 0.0)) {
        let noise_metadata_schedule_46_e396: f64 = (noise_variable_20 + params.p22);
        (noise_metadata_schedule_46_e396,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_46_e398;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_47_e407,) = {
    if (((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_47_e407;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_48_e416,) = {
    if (((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 != 0.0)) {
        (1e99,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_48_e416;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_49_e419: f64 = if params.p0 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_65 = noise_metadata_schedule_49_e419;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_50_e431,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_50_e431;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_51_e443,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_51_e443;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_52_e457,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 != 0.0)) {
        let noise_metadata_schedule_52_e455: f64 = (params.p1 * noise_variable_18);
        (noise_metadata_schedule_52_e455,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_52_e457;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_53_e471,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 != 0.0)) {
        let noise_metadata_schedule_53_e469: f64 = (noise_variable_19 + noise_variable_17);
        (noise_metadata_schedule_53_e469,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_53_e471;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_54_e483,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 != 0.0)) {
        (1e99,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_54_e483;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_55_e495,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_55_e495;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_56_e510,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 == 0.0)) {
        let noise_metadata_schedule_56_e508: f64 = (params.p0 * noise_variable_18);
        (noise_metadata_schedule_56_e508,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_56_e510;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_57_e525,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 == 0.0)) {
        let noise_metadata_schedule_57_e523: f64 = (noise_variable_20 + params.p22);
        (noise_metadata_schedule_57_e523,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_57_e525;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_59_e531: f64 = if noise_variable_4 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_67 = noise_metadata_schedule_59_e531;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_60_e550,) = {
    if (((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 == 0.0)) && (noise_variable_67 != 0.0)) {
        let noise_metadata_schedule_60_e546: f64 = (params.p2 / params.p17);
        let noise_metadata_schedule_60_e548: f64 = (noise_metadata_schedule_60_e546 * noise_variable_4);
        (noise_metadata_schedule_60_e548,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_60_e550;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_61_e567,) = {
    if (((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 == 0.0)) && (noise_variable_67 != 0.0)) {
        let noise_metadata_schedule_61_e565: f64 = (noise_variable_3 - noise_variable_17);
        (noise_metadata_schedule_61_e565,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_61_e567;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_63_e585,) = {
    if (((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 == 0.0)) && (noise_variable_67 != 0.0)) {
        (params.p2,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_63_e585;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_64_e602,) = {
    if (((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 == 0.0)) && (noise_variable_67 != 0.0)) {
        let noise_metadata_schedule_64_e600: f64 = (1.0 / noise_variable_5);
        (noise_metadata_schedule_64_e600,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_64_e602;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_65_e620,) = {
    if (((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 == 0.0)) && (noise_variable_67 == 0.0)) {
        let noise_metadata_schedule_65_e618: f64 = (params.p1 * noise_variable_18);
        (noise_metadata_schedule_65_e618,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_65_e620;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_66_e638,) = {
    if (((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 == 0.0)) && (noise_variable_67 == 0.0)) {
        let noise_metadata_schedule_66_e636: f64 = (noise_variable_19 + noise_variable_17);
        (noise_metadata_schedule_66_e636,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_66_e638;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_67_e654,) = {
    if (((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 == 0.0)) && (noise_variable_67 == 0.0)) {
        (1e99,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_67_e654;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_68_e670,) = {
    if (((((noise_variable_58 == 0.0) && (noise_variable_63 != 0.0)) && (noise_variable_64 == 0.0)) && (noise_variable_65 == 0.0)) && (noise_variable_67 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_68_e670;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_69_e673: f64 = if params.p0 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_69 = noise_metadata_schedule_69_e673;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_70_e683,) = {
    if (((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_70_e683;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_71_e693,) = {
    if (((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_71_e693;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_72_e705,) = {
    if (((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 != 0.0)) {
        let noise_metadata_schedule_72_e703: f64 = (params.p1 * noise_variable_18);
        (noise_metadata_schedule_72_e703,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_72_e705;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_73_e717,) = {
    if (((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 != 0.0)) {
        let noise_metadata_schedule_73_e715: f64 = (noise_variable_19 + noise_variable_17);
        (noise_metadata_schedule_73_e715,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_73_e717;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_74_e727,) = {
    if (((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 != 0.0)) {
        (1e99,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_74_e727;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_75_e737,) = {
    if (((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_75_e737;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_76_e740: f64 = if params.p1 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_70 = noise_metadata_schedule_76_e740;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_77_e753,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_77_e753;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_78_e766,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_78_e766;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_79_e781,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 != 0.0)) {
        let noise_metadata_schedule_79_e779: f64 = (params.p0 * noise_variable_18);
        (noise_metadata_schedule_79_e779,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_79_e781;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_80_e796,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 != 0.0)) {
        let noise_metadata_schedule_80_e794: f64 = (noise_variable_20 + params.p22);
        (noise_metadata_schedule_80_e794,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_80_e796;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_81_e809,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_81_e809;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_82_e822,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 != 0.0)) {
        (1e99,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_82_e822;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_83_e838,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 == 0.0)) {
        let noise_metadata_schedule_83_e836: f64 = (params.p0 * noise_variable_18);
        (noise_metadata_schedule_83_e836,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_83_e838;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_84_e854,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 == 0.0)) {
        let noise_metadata_schedule_84_e852: f64 = (noise_variable_20 + params.p22);
        (noise_metadata_schedule_84_e852,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_84_e854;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_86_e873,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 == 0.0)) {
        let noise_metadata_schedule_86_e871: f64 = (params.p1 * noise_variable_18);
        (noise_metadata_schedule_86_e871,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_86_e873;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_87_e889,) = {
    if ((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 == 0.0)) {
        let noise_metadata_schedule_87_e887: f64 = (noise_variable_19 + noise_variable_17);
        (noise_metadata_schedule_87_e887,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_87_e889;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_88_e892: f64 = if noise_variable_4 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_72 = noise_metadata_schedule_88_e892;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_90_e898: f64 = if noise_variable_3 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_74 = noise_metadata_schedule_90_e898;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_91_e920,) = {
    if ((((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 == 0.0)) && (noise_variable_72 != 0.0)) && (noise_variable_74 != 0.0)) {
        let noise_metadata_schedule_91_e917: f64 = (noise_variable_3 / noise_variable_4);
        let noise_metadata_schedule_91_e918: f64 = (params.p17 * noise_metadata_schedule_91_e917);
        (noise_metadata_schedule_91_e918,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_91_e920;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_92_e940,) = {
    if ((((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 == 0.0)) && (noise_variable_72 != 0.0)) && (noise_variable_74 != 0.0)) {
        let noise_metadata_schedule_92_e938: f64 = (1.0 / noise_variable_5);
        (noise_metadata_schedule_92_e938,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_92_e940;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_93_e959,) = {
    if ((((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 == 0.0)) && (noise_variable_72 != 0.0)) && (noise_variable_74 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_93_e959;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_94_e978,) = {
    if ((((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 == 0.0)) && (noise_variable_72 != 0.0)) && (noise_variable_74 == 0.0)) {
        (1e99,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_94_e978;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_95_e995,) = {
    if (((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 == 0.0)) && (noise_variable_72 == 0.0)) {
        (1e99,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_95_e995;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_96_e1012,) = {
    if (((((noise_variable_58 == 0.0) && (noise_variable_63 == 0.0)) && (noise_variable_69 == 0.0)) && (noise_variable_70 == 0.0)) && (noise_variable_72 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_96_e1012;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_101_e1030,) = {
    if (params.p25 != 0.0) {
        let noise_metadata_schedule_101_e1028: f64 = (noise_variable_3 + params.p24);
        (noise_metadata_schedule_101_e1028,)
    } else {
        (noise_variable_21,)
    }
};
            noise_variable_21 = noise_metadata_schedule_101_e1030;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_102_e1037,) = {
    if (params.p25 == 0.0) {
        let noise_metadata_schedule_102_e1035: f64 = (noise_variable_19 + params.p24);
        (noise_metadata_schedule_102_e1035,)
    } else {
        (noise_variable_21,)
    }
};
            noise_variable_21 = noise_metadata_schedule_102_e1037;
        }
        if matches!(source_index, 0 | 1) {
            noise_variable_37 = params.p37;
        }
        if matches!(source_index, 0 | 1) {
            noise_variable_38 = params.p38;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_106_e1057: f64 = if noise_variable_3 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_80 = noise_metadata_schedule_106_e1057;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_107_e1060: f64 = if ((params.p3 != 0.0) && (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            noise_variable_81 = noise_metadata_schedule_107_e1060;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_108_e1070,) = {
    if ((noise_variable_80 != 0.0) && (noise_variable_81 != 0.0)) {
        let noise_metadata_schedule_108_e1067: f64 = (params.p39 / noise_variable_3);
        let noise_metadata_schedule_108_e1068: f64 = (noise_variable_37 + noise_metadata_schedule_108_e1067);
        (noise_metadata_schedule_108_e1068,)
    } else {
        (noise_variable_37,)
    }
};
            noise_variable_37 = noise_metadata_schedule_108_e1070;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_109_e1080,) = {
    if ((noise_variable_80 != 0.0) && (noise_variable_81 != 0.0)) {
        let noise_metadata_schedule_109_e1077: f64 = (params.p40 / noise_variable_3);
        let noise_metadata_schedule_109_e1078: f64 = (noise_variable_38 + noise_metadata_schedule_109_e1077);
        (noise_metadata_schedule_109_e1078,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_109_e1080;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_110_e1083: f64 = if ((params.p3 != 0.0) || (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            noise_variable_82 = noise_metadata_schedule_110_e1083;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_111_e1098,) = {
    if (((noise_variable_80 != 0.0) && (noise_variable_81 == 0.0)) && (noise_variable_82 != 0.0)) {
        let noise_metadata_schedule_111_e1093: f64 = (0.5 * params.p39);
        let noise_metadata_schedule_111_e1095: f64 = (noise_metadata_schedule_111_e1093 / noise_variable_3);
        let noise_metadata_schedule_111_e1096: f64 = (noise_variable_37 + noise_metadata_schedule_111_e1095);
        (noise_metadata_schedule_111_e1096,)
    } else {
        (noise_variable_37,)
    }
};
            noise_variable_37 = noise_metadata_schedule_111_e1098;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_112_e1113,) = {
    if (((noise_variable_80 != 0.0) && (noise_variable_81 == 0.0)) && (noise_variable_82 != 0.0)) {
        let noise_metadata_schedule_112_e1108: f64 = (0.5 * params.p40);
        let noise_metadata_schedule_112_e1110: f64 = (noise_metadata_schedule_112_e1108 / noise_variable_3);
        let noise_metadata_schedule_112_e1111: f64 = (noise_variable_38 + noise_metadata_schedule_112_e1110);
        (noise_metadata_schedule_112_e1111,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_112_e1113;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_113_e1116: f64 = if noise_variable_4 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_83 = noise_metadata_schedule_113_e1116;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_114_e1124,) = {
    if (noise_variable_83 != 0.0) {
        let noise_metadata_schedule_114_e1121: f64 = (params.p41 / noise_variable_4);
        let noise_metadata_schedule_114_e1122: f64 = (noise_variable_37 + noise_metadata_schedule_114_e1121);
        (noise_metadata_schedule_114_e1122,)
    } else {
        (noise_variable_37,)
    }
};
            noise_variable_37 = noise_metadata_schedule_114_e1124;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_115_e1132,) = {
    if (noise_variable_83 != 0.0) {
        let noise_metadata_schedule_115_e1129: f64 = (params.p42 / noise_variable_4);
        let noise_metadata_schedule_115_e1130: f64 = (noise_variable_38 + noise_metadata_schedule_115_e1129);
        (noise_metadata_schedule_115_e1130,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_115_e1132;
        }
        if matches!(source_index, 0 | 1) {
            noise_variable_42 = (ctx.node_voltage(self.nodes[2]) - 0.0);
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_129_e1209: f64 = (params.p7 * noise_variable_42);
            let noise_metadata_schedule_129_e1210: f64 = (noise_variable_28 + noise_metadata_schedule_129_e1209);
            noise_variable_28 = noise_metadata_schedule_129_e1210;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_130_e1214: f64 = (params.p35 + 1.0);
            let noise_metadata_schedule_130_e1215: f64 = if noise_variable_28 < noise_metadata_schedule_130_e1214 { 1.0 } else { 0.0 };
            noise_variable_88 = noise_metadata_schedule_130_e1215;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_131_e1226,) = {
    if (noise_variable_88 != 0.0) {
        let noise_metadata_schedule_131_e1220: f64 = (noise_variable_28 - params.p35);
        let noise_metadata_schedule_131_e1222: f64 = (noise_metadata_schedule_131_e1220 - 1.0);
        let noise_metadata_schedule_131_e1223: f64 = (noise_metadata_schedule_131_e1222).exp();
        let noise_metadata_schedule_131_e1224: f64 = (params.p35 + noise_metadata_schedule_131_e1223);
        (noise_metadata_schedule_131_e1224,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_131_e1226;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_132_e1230: f64 = (params.p36 - 1.0);
            let noise_metadata_schedule_132_e1231: f64 = if noise_variable_28 > noise_metadata_schedule_132_e1230 { 1.0 } else { 0.0 };
            noise_variable_89 = noise_metadata_schedule_132_e1231;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_133_e1245,) = {
    if ((noise_variable_88 == 0.0) && (noise_variable_89 != 0.0)) {
        let noise_metadata_schedule_133_e1239: f64 = (params.p36 - noise_variable_28);
        let noise_metadata_schedule_133_e1241: f64 = (noise_metadata_schedule_133_e1239 - 1.0);
        let noise_metadata_schedule_133_e1242: f64 = (noise_metadata_schedule_133_e1241).exp();
        let noise_metadata_schedule_133_e1243: f64 = (params.p36 - noise_metadata_schedule_133_e1242);
        (noise_metadata_schedule_133_e1243,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_133_e1245;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_134_e1253,) = {
    if ((noise_variable_88 == 0.0) && (noise_variable_89 == 0.0)) {
        (noise_variable_28,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_134_e1253;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_135_e1256: f64 = (noise_variable_28 + 273.15);
            noise_variable_12 = noise_metadata_schedule_135_e1256;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_136_e1259: f64 = (noise_variable_12 - noise_variable_11);
            noise_variable_15 = noise_metadata_schedule_136_e1259;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_137_e1265: f64 = (noise_variable_15 * noise_variable_38);
            let noise_metadata_schedule_137_e1266: f64 = (noise_variable_37 + noise_metadata_schedule_137_e1265);
            let noise_metadata_schedule_137_e1267: f64 = (noise_variable_15 * noise_metadata_schedule_137_e1266);
            let noise_metadata_schedule_137_e1268: f64 = (1.0 + noise_metadata_schedule_137_e1267);
            noise_variable_16 = noise_metadata_schedule_137_e1268;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_138_e1272: f64 = (0.01 + 0.1);
            let noise_metadata_schedule_138_e1273: f64 = if noise_variable_16 < noise_metadata_schedule_138_e1272 { 1.0 } else { 0.0 };
            noise_variable_90 = noise_metadata_schedule_138_e1273;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_139_e1288,) = {
    if (noise_variable_90 != 0.0) {
        let noise_metadata_schedule_139_e1280: f64 = (noise_variable_16 - 0.01);
        let noise_metadata_schedule_139_e1281: f64 = (10.0 * noise_metadata_schedule_139_e1280);
        let noise_metadata_schedule_139_e1283: f64 = (noise_metadata_schedule_139_e1281 - 1.0);
        let noise_metadata_schedule_139_e1284: f64 = (noise_metadata_schedule_139_e1283).exp();
        let noise_metadata_schedule_139_e1285: f64 = (0.1 * noise_metadata_schedule_139_e1284);
        let noise_metadata_schedule_139_e1286: f64 = (0.01 + noise_metadata_schedule_139_e1285);
        (noise_metadata_schedule_139_e1286,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_139_e1288;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_140_e1293,) = {
    if (noise_variable_90 == 0.0) {
        (noise_variable_16,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_140_e1293;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_141_e1296: f64 = (noise_variable_5 * noise_variable_16);
            noise_variable_23 = noise_metadata_schedule_141_e1296;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_142_e1299: f64 = (noise_variable_22 / noise_variable_16);
            noise_variable_24 = noise_metadata_schedule_142_e1299;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_143_e1303: f64 = (noise_variable_15 * params.p43);
            let noise_metadata_schedule_143_e1304: f64 = (1.0 + noise_metadata_schedule_143_e1303);
            let noise_metadata_schedule_143_e1306: f64 = (noise_metadata_schedule_143_e1304 * params.p30);
            noise_variable_25 = noise_metadata_schedule_143_e1306;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_144_e1309: f64 = if noise_variable_25 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_91 = noise_metadata_schedule_144_e1309;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_145_e1313,) = {
    if (noise_variable_91 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_25,)
    }
};
            noise_variable_25 = noise_metadata_schedule_145_e1313;
        }
        if matches!(source_index, 0 | 1) {
            noise_variable_33 = (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[1]));
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_147_e1325: f64 = if ((noise_variable_5 > 0.0) && ((params.p29 > 0.0) || (params.p27 > 0.0))) { 1.0 } else { 0.0 };
            noise_variable_92 = noise_metadata_schedule_147_e1325;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_148_e1331,) = {
    if (noise_variable_92 != 0.0) {
        let noise_metadata_schedule_148_e1329: f64 = (noise_variable_33 / noise_variable_21);
        (noise_metadata_schedule_148_e1329,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_148_e1331;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_149_e1337,) = {
    if (noise_variable_92 != 0.0) {
        let noise_metadata_schedule_149_e1335: f64 = (params.p28 * noise_variable_34);
        (noise_metadata_schedule_149_e1335,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_149_e1337;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_150_e1346,) = {
    if (noise_variable_92 != 0.0) {
        let noise_metadata_schedule_150_e1342: f64 = (noise_variable_35 * noise_variable_35);
        let noise_metadata_schedule_150_e1343: f64 = (1.0 + noise_metadata_schedule_150_e1342);
        let noise_metadata_schedule_150_e1344: f64 = (noise_metadata_schedule_150_e1343).sqrt();
        (noise_metadata_schedule_150_e1344,)
    } else {
        (noise_variable_26,)
    }
};
            noise_variable_26 = noise_metadata_schedule_150_e1346;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_151_e1353,) = {
    if (noise_variable_92 != 0.0) {
        let noise_metadata_schedule_151_e1350: f64 = (noise_variable_34).abs();
        let noise_metadata_schedule_151_e1351: f64 = (params.p26 * noise_metadata_schedule_151_e1350);
        (noise_metadata_schedule_151_e1351,)
    } else {
        (noise_variable_36,)
    }
};
            noise_variable_36 = noise_metadata_schedule_151_e1353;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_152_e1365,) = {
    if (noise_variable_92 != 0.0) {
        let noise_metadata_schedule_152_e1358: f64 = (noise_variable_36 * noise_variable_36);
        let noise_metadata_schedule_152_e1360: f64 = (noise_metadata_schedule_152_e1358 * noise_variable_36);
        let noise_metadata_schedule_152_e1361: f64 = (1.0 + noise_metadata_schedule_152_e1360);
        let noise_metadata_schedule_152_e1363: f64 = (noise_metadata_schedule_152_e1361).powf(0.3333333333333333);
        (noise_metadata_schedule_152_e1363,)
    } else {
        (noise_variable_27,)
    }
};
            noise_variable_27 = noise_metadata_schedule_152_e1365;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_153_e1381,) = {
    if (noise_variable_92 != 0.0) {
        let noise_metadata_schedule_153_e1369: f64 = (1.0 - params.p29);
        let noise_metadata_schedule_153_e1371: f64 = (noise_metadata_schedule_153_e1369 - params.p27);
        let noise_metadata_schedule_153_e1374: f64 = (params.p29 * noise_variable_26);
        let noise_metadata_schedule_153_e1375: f64 = (noise_metadata_schedule_153_e1371 + noise_metadata_schedule_153_e1374);
        let noise_metadata_schedule_153_e1378: f64 = (params.p27 * noise_variable_27);
        let noise_metadata_schedule_153_e1379: f64 = (noise_metadata_schedule_153_e1375 + noise_metadata_schedule_153_e1378);
        (noise_metadata_schedule_153_e1379,)
    } else {
        (noise_variable_32,)
    }
};
            noise_variable_32 = noise_metadata_schedule_153_e1381;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_154_e1386,) = {
    if (noise_variable_92 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_32,)
    }
};
            noise_variable_32 = noise_metadata_schedule_154_e1386;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_155_e1389: f64 = (noise_variable_23 * noise_variable_32);
            noise_variable_6 = noise_metadata_schedule_155_e1389;
        }
        if matches!(source_index, 1) {
            noise_variable_0 = noise_variable_33;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_157_e1393: f64 = (noise_variable_0 / noise_variable_6);
            noise_variable_1 = noise_metadata_schedule_157_e1393;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_163_e1421: f64 = if (((params.p6 != 0.0) && (noise_variable_5 > 0.0)) && (noise_variable_22 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_95 = noise_metadata_schedule_163_e1421;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_164_e1433,) = {
    if (noise_variable_95 != 0.0) {
        let noise_metadata_schedule_164_e1425: f64 = (4.0 * 1.3806505e-23);
        let noise_metadata_schedule_164_e1427: f64 = (noise_metadata_schedule_164_e1425 * noise_variable_12);
        let noise_metadata_schedule_164_e1429: f64 = (noise_metadata_schedule_164_e1427 * noise_variable_24);
        let noise_metadata_schedule_164_e1431: f64 = (noise_metadata_schedule_164_e1429 / noise_variable_32);
        (noise_metadata_schedule_164_e1431,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_164_e1433;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_165_e1442: f64 = if (((params.p33 != 0.0) && (noise_variable_3 > 0.0)) && (noise_variable_4 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_96 = noise_metadata_schedule_165_e1442;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_166_e1459,) = {
    if ((noise_variable_95 != 0.0) && (noise_variable_96 != 0.0)) {
        let noise_metadata_schedule_166_e1449: f64 = (noise_variable_1 / noise_variable_4);
        let noise_metadata_schedule_166_e1450: f64 = (noise_metadata_schedule_166_e1449).abs();
        let noise_metadata_schedule_166_e1452: f64 = (noise_metadata_schedule_166_e1450).powf(params.p31);
        let noise_metadata_schedule_166_e1453: f64 = (noise_variable_25 * noise_metadata_schedule_166_e1452);
        let noise_metadata_schedule_166_e1455: f64 = (noise_metadata_schedule_166_e1453 * noise_variable_4);
        let noise_metadata_schedule_166_e1457: f64 = (noise_metadata_schedule_166_e1455 / noise_variable_3);
        (noise_metadata_schedule_166_e1457,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_166_e1459;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_167_e1466: f64 = if ((noise_variable_19 > 0.0) && (noise_variable_20 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_97 = noise_metadata_schedule_167_e1466;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_168_e1486,) = {
    if (((noise_variable_95 != 0.0) && (noise_variable_96 == 0.0)) && (noise_variable_97 != 0.0)) {
        let noise_metadata_schedule_168_e1476: f64 = (noise_variable_1 / noise_variable_20);
        let noise_metadata_schedule_168_e1477: f64 = (noise_metadata_schedule_168_e1476).abs();
        let noise_metadata_schedule_168_e1479: f64 = (noise_metadata_schedule_168_e1477).powf(params.p31);
        let noise_metadata_schedule_168_e1480: f64 = (noise_variable_25 * noise_metadata_schedule_168_e1479);
        let noise_metadata_schedule_168_e1482: f64 = (noise_metadata_schedule_168_e1480 * noise_variable_20);
        let noise_metadata_schedule_168_e1484: f64 = (noise_metadata_schedule_168_e1482 / noise_variable_19);
        (noise_metadata_schedule_168_e1484,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_168_e1486;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_169_e1496,) = {
    if (((noise_variable_95 != 0.0) && (noise_variable_96 == 0.0)) && (noise_variable_97 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_169_e1496;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_170_e1499: f64 = if noise_variable_1 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_98 = noise_metadata_schedule_170_e1499;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_171_e1506,) = {
    if ((noise_variable_95 != 0.0) && (noise_variable_98 != 0.0)) {
        let noise_metadata_schedule_171_e1504: f64 = (-noise_variable_30);
        (noise_metadata_schedule_171_e1504,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_171_e1506;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_172_e1511,) = {
    if (noise_variable_95 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_172_e1511;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_173_e1516,) = {
    if (noise_variable_95 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_173_e1516;
        }
        match source_index {
            0 => {
                let noise_0_psd_e1618: f64 = 1.0;
                let noise_0_psd_e1619: f64 = (noise_0_psd_e1618 * noise_variable_29);
                let psd = noise_0_psd_e1619;
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
                let noise_1_psd_e1621: f64 = 1.0;
                let noise_1_psd_e1622: f64 = (noise_1_psd_e1621 * noise_variable_30);
                let psd = noise_1_psd_e1622;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
                let exponent: Option<f64> = Some(params.p32);
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

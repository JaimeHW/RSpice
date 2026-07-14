#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 6] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RB", label: Some("Rb"), kind: GeneratedNoiseKind::White, equation: 24, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE", label: Some("Re"), kind: GeneratedNoiseKind::White, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_CI_RC", label: Some("Rc"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER_IBE", label: Some("flicker_Ibe"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBE", label: Some("Ibe"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_IT", label: Some("It"), kind: GeneratedNoiseKind::White, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
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
        let mut noise_variable_102 = 0.0;
        let mut noise_variable_103 = 0.0;
        let mut noise_variable_104 = 0.0;
        let mut noise_variable_105 = 0.0;
        let mut noise_variable_106 = 0.0;
        let mut noise_variable_107 = 0.0;
        let mut noise_variable_108 = 0.0;
        let mut noise_variable_109 = 0.0;
        let mut noise_variable_110 = 0.0;
        let mut noise_variable_111 = 0.0;
        let mut noise_variable_112 = 0.0;
        let mut noise_variable_113 = 0.0;
        let mut noise_variable_114 = 0.0;
        let mut noise_variable_115 = 0.0;
        let mut noise_variable_116 = 0.0;
        let mut noise_variable_117 = 0.0;
        let mut noise_variable_118 = 0.0;
        let mut noise_variable_119 = 0.0;
        let mut noise_variable_120 = 0.0;
        let mut noise_variable_121 = 0.0;
        let mut noise_variable_122 = 0.0;
        let mut noise_variable_123 = 0.0;
        let mut noise_variable_124 = 0.0;
        let mut noise_variable_125 = 0.0;
        let mut noise_variable_126 = 0.0;
        let mut noise_variable_127 = 0.0;
        if matches!(source_index, 0 | 1 | 2) {
            let noise_activation_schedule_4_e502: f64 = (params.p43 * params.p42);
            noise_variable_3 = noise_activation_schedule_4_e502;
        }
        if matches!(source_index, 0) {
            let noise_activation_schedule_196_e2249: f64 = (params.p31 * params.p13);
            let noise_activation_schedule_196_e2250: f64 = (params.p12 + noise_activation_schedule_196_e2249);
            let noise_activation_schedule_196_e2252: f64 = (noise_activation_schedule_196_e2250 / noise_variable_3);
            noise_variable_50 = noise_activation_schedule_196_e2252;
        }
        if matches!(source_index, 1) {
            let noise_activation_schedule_197_e2256: f64 = (params.p31 * params.p15);
            let noise_activation_schedule_197_e2257: f64 = (params.p14 + noise_activation_schedule_197_e2256);
            let noise_activation_schedule_197_e2259: f64 = (noise_activation_schedule_197_e2257 / noise_variable_3);
            noise_variable_48 = noise_activation_schedule_197_e2259;
        }
        if matches!(source_index, 2) {
            let noise_activation_schedule_198_e2263: f64 = (params.p31 * params.p67);
            let noise_activation_schedule_198_e2264: f64 = (params.p66 + noise_activation_schedule_198_e2263);
            let noise_activation_schedule_198_e2266: f64 = (noise_activation_schedule_198_e2264 / noise_variable_3);
            noise_variable_49 = noise_activation_schedule_198_e2266;
        }
        if matches!(source_index, 0) {
            let noise_activation_schedule_199_e2273: f64 = if ((noise_variable_50 > 0.0) && (noise_variable_50 >= params.p46)) { 1.0 } else { 0.0 };
            noise_variable_125 = noise_activation_schedule_199_e2273;
        }
        if matches!(source_index, 1) {
            let noise_activation_schedule_201_e2295: f64 = if ((noise_variable_48 > 0.0) && (noise_variable_48 >= params.p46)) { 1.0 } else { 0.0 };
            noise_variable_126 = noise_activation_schedule_201_e2295;
        }
        if matches!(source_index, 2) {
            let noise_activation_schedule_203_e2317: f64 = if ((noise_variable_49 > 0.0) && (noise_variable_49 >= params.p46)) { 1.0 } else { 0.0 };
            noise_variable_127 = noise_activation_schedule_203_e2317;
        }
        let noise_source_active = match source_index {
            0 => {
                noise_variable_125 != 0.0
            }
            1 => {
                noise_variable_126 != 0.0
            }
            2 => {
                noise_variable_127 != 0.0
            }
            3 => {
                true
            }
            4 => {
                true
            }
            5 => {
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
        noise_variable_102 = 0.0;
        noise_variable_103 = 0.0;
        noise_variable_104 = 0.0;
        noise_variable_105 = 0.0;
        noise_variable_106 = 0.0;
        noise_variable_107 = 0.0;
        noise_variable_108 = 0.0;
        noise_variable_109 = 0.0;
        noise_variable_110 = 0.0;
        noise_variable_111 = 0.0;
        noise_variable_112 = 0.0;
        noise_variable_113 = 0.0;
        noise_variable_114 = 0.0;
        noise_variable_115 = 0.0;
        noise_variable_116 = 0.0;
        noise_variable_117 = 0.0;
        noise_variable_118 = 0.0;
        noise_variable_119 = 0.0;
        noise_variable_120 = 0.0;
        noise_variable_121 = 0.0;
        noise_variable_122 = 0.0;
        noise_variable_123 = 0.0;
        noise_variable_124 = 0.0;
        noise_variable_125 = 0.0;
        noise_variable_126 = 0.0;
        noise_variable_127 = 0.0;
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_0_e456: f64 = ctx.temperature();
            let noise_metadata_schedule_0_e458: f64 = (noise_metadata_schedule_0_e456 + (ctx.node_voltage(self.nodes[3]) - 0.0));
            let noise_metadata_schedule_0_e460: f64 = (noise_metadata_schedule_0_e458 + params.p45);
            noise_variable_12 = noise_metadata_schedule_0_e460;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_1_e463: f64 = (1026.85 + 273.15);
            let noise_metadata_schedule_1_e466: f64 = (-100.0);
            let noise_metadata_schedule_1_e468: f64 = (noise_metadata_schedule_1_e466 + 273.15);
            let (noise_metadata_schedule_1_e475,) = {
    if (noise_variable_12 > noise_metadata_schedule_1_e468) {
        (noise_variable_12,)
    } else {
        let noise_metadata_schedule_1_e472: f64 = (-100.0);
        let noise_metadata_schedule_1_e474: f64 = (noise_metadata_schedule_1_e472 + 273.15);
        (noise_metadata_schedule_1_e474,)
    }
};
            let (noise_metadata_schedule_1_e492,) = {
    if (noise_metadata_schedule_1_e463 < noise_metadata_schedule_1_e475) {
        let noise_metadata_schedule_1_e479: f64 = (1026.85 + 273.15);
        (noise_metadata_schedule_1_e479,)
    } else {
        let noise_metadata_schedule_1_e482: f64 = (-100.0);
        let noise_metadata_schedule_1_e484: f64 = (noise_metadata_schedule_1_e482 + 273.15);
        let (noise_metadata_schedule_1_e491,) = {
            if (noise_variable_12 > noise_metadata_schedule_1_e484) {
                (noise_variable_12,)
            } else {
                let noise_metadata_schedule_1_e488: f64 = (-100.0);
                let noise_metadata_schedule_1_e490: f64 = (noise_metadata_schedule_1_e488 + 273.15);
                (noise_metadata_schedule_1_e490,)
            }
        };
        (noise_metadata_schedule_1_e491,)
    }
};
            noise_variable_10 = noise_metadata_schedule_1_e492;
        }
        if matches!(source_index, 0 | 1 | 2) {
            let noise_metadata_schedule_4_e502: f64 = (params.p43 * params.p42);
            noise_variable_3 = noise_metadata_schedule_4_e502;
        }
        if matches!(source_index, 3 | 4) {
            let noise_metadata_schedule_5_e505: f64 = (params.p29 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[4])));
            noise_variable_95 = noise_metadata_schedule_5_e505;
        }
        if matches!(source_index, 3 | 4) {
            let noise_metadata_schedule_6_e510: f64 = (noise_variable_95).min(0.0);
            let noise_metadata_schedule_6_e511: f64 = (-noise_metadata_schedule_6_e510);
            let noise_metadata_schedule_6_e513: f64 = (noise_metadata_schedule_6_e511).powf(params.p80);
            let noise_metadata_schedule_6_e514: f64 = (params.p79 * noise_metadata_schedule_6_e513);
            let noise_metadata_schedule_6_e515: f64 = (1.0 + noise_metadata_schedule_6_e514);
            noise_variable_94 = noise_metadata_schedule_6_e515;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_7_e518: f64 = (params.p25 + 273.15);
            noise_variable_11 = noise_metadata_schedule_7_e518;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_8_e521: f64 = (8.6170869e-5 * noise_variable_10);
            noise_variable_15 = noise_metadata_schedule_8_e521;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_9_e524: f64 = (noise_variable_10 / noise_variable_11);
            noise_variable_13 = noise_metadata_schedule_9_e524;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_10_e526: f64 = (noise_variable_13).ln();
            noise_variable_14 = noise_metadata_schedule_10_e526;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_11_e529: f64 = (params.p77 * noise_variable_14);
            let noise_metadata_schedule_11_e530: f64 = (noise_metadata_schedule_11_e529).exp();
            noise_variable_18 = noise_metadata_schedule_11_e530;
        }
        if matches!(source_index, 3 | 4) {
            let noise_metadata_schedule_12_e533: f64 = (params.p52 * noise_variable_18);
            let noise_metadata_schedule_12_e535: f64 = (noise_metadata_schedule_12_e533 * noise_variable_94);
            noise_variable_16 = noise_metadata_schedule_12_e535;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_14_e546,) = {
    if (params.p53 > 0.0) {
        let noise_metadata_schedule_14_e544: f64 = (1.0 / params.p53);
        (noise_metadata_schedule_14_e544,)
    } else {
        (0.0,)
    }
};
            noise_variable_64 = noise_metadata_schedule_14_e546;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_15_e554,) = {
    if (params.p62 > 0.0) {
        let noise_metadata_schedule_15_e552: f64 = (1.0 / params.p62);
        (noise_metadata_schedule_15_e552,)
    } else {
        (0.0,)
    }
};
            noise_variable_65 = noise_metadata_schedule_15_e554;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_16_e562,) = {
    if (params.p54 > 0.0) {
        let noise_metadata_schedule_16_e560: f64 = (1.0 / params.p54);
        (noise_metadata_schedule_16_e560,)
    } else {
        (0.0,)
    }
};
            noise_variable_66 = noise_metadata_schedule_16_e562;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_17_e570,) = {
    if (params.p63 > 0.0) {
        let noise_metadata_schedule_17_e568: f64 = (1.0 / params.p63);
        (noise_metadata_schedule_17_e568,)
    } else {
        (0.0,)
    }
};
            noise_variable_67 = noise_metadata_schedule_17_e570;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_18_e573: f64 = (params.p22 * noise_variable_14);
            let noise_metadata_schedule_18_e577: f64 = (noise_variable_13 - 1.0);
            let noise_metadata_schedule_18_e578: f64 = (params.p21 * noise_metadata_schedule_18_e577);
            let noise_metadata_schedule_18_e580: f64 = (noise_metadata_schedule_18_e578 / noise_variable_15);
            let noise_metadata_schedule_18_e581: f64 = (noise_metadata_schedule_18_e573 + noise_metadata_schedule_18_e580);
            noise_variable_68 = noise_metadata_schedule_18_e581;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_19_e584: f64 = (params.p23 * noise_variable_14);
            noise_variable_92 = noise_metadata_schedule_19_e584;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_20_e587: f64 = (noise_variable_68).exp();
            let noise_metadata_schedule_20_e588: f64 = (params.p0 * noise_metadata_schedule_20_e587);
            noise_variable_19 = noise_metadata_schedule_20_e588;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_21_e591: f64 = (noise_variable_92).exp();
            let noise_metadata_schedule_21_e592: f64 = (params.p2 * noise_metadata_schedule_21_e591);
            noise_variable_93 = noise_metadata_schedule_21_e592;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_22_e596: f64 = (noise_variable_68 / params.p59);
            let noise_metadata_schedule_22_e597: f64 = (noise_metadata_schedule_22_e596).exp();
            let noise_metadata_schedule_22_e598: f64 = (params.p58 * noise_metadata_schedule_22_e597);
            let noise_metadata_schedule_22_e600: f64 = (noise_metadata_schedule_22_e598 / noise_variable_18);
            noise_variable_20 = noise_metadata_schedule_22_e600;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_24_e614: f64 = (noise_variable_13 - 1.0);
            let noise_metadata_schedule_24_e615: f64 = (params.p7 * noise_metadata_schedule_24_e614);
            let noise_metadata_schedule_24_e616: f64 = (1.0 + noise_metadata_schedule_24_e615);
            let noise_metadata_schedule_24_e617: f64 = (params.p47 * noise_metadata_schedule_24_e616);
            noise_variable_28 = noise_metadata_schedule_24_e617;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_25_e623: f64 = (noise_variable_13 - 1.0);
            let noise_metadata_schedule_25_e624: f64 = (params.p6 * noise_metadata_schedule_25_e623);
            let noise_metadata_schedule_25_e625: f64 = (1.0 + noise_metadata_schedule_25_e624);
            let noise_metadata_schedule_25_e626: f64 = (params.p5 * noise_metadata_schedule_25_e625);
            noise_variable_30 = noise_metadata_schedule_25_e626;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_26_e632: f64 = (noise_variable_13 - 1.0);
            let noise_metadata_schedule_26_e633: f64 = (params.p10 * noise_metadata_schedule_26_e632);
            let noise_metadata_schedule_26_e634: f64 = (1.0 + noise_metadata_schedule_26_e633);
            let noise_metadata_schedule_26_e635: f64 = (params.p9 * noise_metadata_schedule_26_e634);
            noise_variable_31 = noise_metadata_schedule_26_e635;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_27_e641: f64 = (noise_variable_13 - 1.0);
            let noise_metadata_schedule_27_e642: f64 = (params.p55 * noise_metadata_schedule_27_e641);
            let noise_metadata_schedule_27_e643: f64 = (1.0 + noise_metadata_schedule_27_e642);
            let noise_metadata_schedule_27_e644: f64 = (params.p56 * noise_metadata_schedule_27_e643);
            noise_variable_29 = noise_metadata_schedule_27_e644;
        }
        if matches!(source_index, 0 | 1 | 3 | 4 | 5) {
            noise_variable_9 = params.p29;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_66_e930: f64 = (noise_variable_9 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[6])));
            noise_variable_76 = noise_metadata_schedule_66_e930;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_67_e933: f64 = (noise_variable_9 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[4])));
            noise_variable_77 = noise_metadata_schedule_67_e933;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_69_e939: f64 = (noise_variable_9 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_79 = noise_metadata_schedule_69_e939;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_70_e942: f64 = (noise_variable_9 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
            noise_variable_80 = noise_metadata_schedule_70_e942;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_71_e945: f64 = if noise_variable_19 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_105 = noise_metadata_schedule_71_e945;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_72_e953,) = {
    if (noise_variable_105 != 0.0) {
        let noise_metadata_schedule_72_e950: f64 = (params.p1 * noise_variable_15);
        let noise_metadata_schedule_72_e951: f64 = (noise_variable_76 / noise_metadata_schedule_72_e950);
        (noise_metadata_schedule_72_e951,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_72_e953;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_73_e964,) = {
    if (noise_variable_105 != 0.0) {
        let noise_metadata_schedule_73_e956: f64 = (-noise_variable_76);
        let noise_metadata_schedule_73_e958: f64 = (noise_metadata_schedule_73_e956 - noise_variable_30);
        let noise_metadata_schedule_73_e961: f64 = (params.p11 * noise_variable_15);
        let noise_metadata_schedule_73_e962: f64 = (noise_metadata_schedule_73_e958 / noise_metadata_schedule_73_e961);
        (noise_metadata_schedule_73_e962,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_73_e964;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_74_e973,) = {
    if (noise_variable_105 != 0.0) {
        let noise_metadata_schedule_74_e967: f64 = (-noise_variable_30);
        let noise_metadata_schedule_74_e970: f64 = (params.p11 * noise_variable_15);
        let noise_metadata_schedule_74_e971: f64 = (noise_metadata_schedule_74_e967 / noise_metadata_schedule_74_e970);
        (noise_metadata_schedule_74_e971,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_74_e973;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_75_e976: f64 = if noise_variable_0 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_106 = noise_metadata_schedule_75_e976;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_76_e986,) = {
    if ((noise_variable_105 != 0.0) && (noise_variable_106 != 0.0)) {
        let noise_metadata_schedule_76_e983: f64 = (noise_variable_0 - 80.0);
        let noise_metadata_schedule_76_e984: f64 = (1.0 + noise_metadata_schedule_76_e983);
        (noise_metadata_schedule_76_e984,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_76_e986;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_77_e992,) = {
    if ((noise_variable_105 != 0.0) && (noise_variable_106 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_77_e992;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_78_e999,) = {
    if ((noise_variable_105 != 0.0) && (noise_variable_106 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_78_e999;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_79_e1006,) = {
    if (noise_variable_105 != 0.0) {
        let noise_metadata_schedule_79_e1003: f64 = (noise_variable_0).exp();
        let noise_metadata_schedule_79_e1004: f64 = (noise_variable_1 * noise_metadata_schedule_79_e1003);
        (noise_metadata_schedule_79_e1004,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_79_e1006;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_80_e1078,) = {
    if (noise_variable_105 != 0.0) {
        let noise_metadata_schedule_80_e1014: f64 = (-37.0);
        let (noise_metadata_schedule_80_e1041,) = {
            if ((!(noise_variable_90 >= 37.0)) && (!(noise_variable_90 <= noise_metadata_schedule_80_e1014))) {
                let noise_metadata_schedule_80_e1019: f64 = (noise_variable_90).exp();
                let noise_metadata_schedule_80_e1021: f64 = (noise_metadata_schedule_80_e1019 + 1.0);
                let noise_metadata_schedule_80_e1022: f64 = (noise_metadata_schedule_80_e1021).ln();
                (noise_metadata_schedule_80_e1022,)
            } else {
                let noise_metadata_schedule_80_e1029: f64 = (-37.0);
                let (noise_metadata_schedule_80_e1040,) = {
                    if ((!(noise_variable_90 >= 37.0)) && (noise_variable_90 <= noise_metadata_schedule_80_e1029)) {
                        let noise_metadata_schedule_80_e1033: f64 = (noise_variable_90).exp();
                        (noise_metadata_schedule_80_e1033,)
                    } else {
                        let (noise_metadata_schedule_80_e1039,) = {
                            if (noise_variable_90 >= 37.0) {
                                (noise_variable_90,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_80_e1039,)
                    }
                };
                (noise_metadata_schedule_80_e1040,)
            }
        };
        let noise_metadata_schedule_80_e1048: f64 = (-37.0);
        let (noise_metadata_schedule_80_e1075,) = {
            if ((!(noise_variable_91 >= 37.0)) && (!(noise_variable_91 <= noise_metadata_schedule_80_e1048))) {
                let noise_metadata_schedule_80_e1053: f64 = (noise_variable_91).exp();
                let noise_metadata_schedule_80_e1055: f64 = (noise_metadata_schedule_80_e1053 + 1.0);
                let noise_metadata_schedule_80_e1056: f64 = (noise_metadata_schedule_80_e1055).ln();
                (noise_metadata_schedule_80_e1056,)
            } else {
                let noise_metadata_schedule_80_e1063: f64 = (-37.0);
                let (noise_metadata_schedule_80_e1074,) = {
                    if ((!(noise_variable_91 >= 37.0)) && (noise_variable_91 <= noise_metadata_schedule_80_e1063)) {
                        let noise_metadata_schedule_80_e1067: f64 = (noise_variable_91).exp();
                        (noise_metadata_schedule_80_e1067,)
                    } else {
                        let (noise_metadata_schedule_80_e1073,) = {
                            if (noise_variable_91 >= 37.0) {
                                (noise_variable_91,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_80_e1073,)
                    }
                };
                (noise_metadata_schedule_80_e1074,)
            }
        };
        let noise_metadata_schedule_80_e1076: f64 = (noise_metadata_schedule_80_e1041 - noise_metadata_schedule_80_e1075);
        (noise_metadata_schedule_80_e1076,)
    } else {
        (noise_variable_2,)
    }
};
            noise_variable_2 = noise_metadata_schedule_80_e1078;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_81_e1099,) = {
    if (noise_variable_105 != 0.0) {
        let noise_metadata_schedule_81_e1083: f64 = (noise_variable_1 - 1.0);
        let noise_metadata_schedule_81_e1084: f64 = (noise_variable_19 * noise_metadata_schedule_81_e1083);
        let noise_metadata_schedule_81_e1087: f64 = (noise_variable_28 * noise_variable_2);
        let noise_metadata_schedule_81_e1091: f64 = (noise_variable_76).abs();
        let noise_metadata_schedule_81_e1093: f64 = (noise_metadata_schedule_81_e1091).powf(noise_variable_31);
        let noise_metadata_schedule_81_e1094: f64 = (params.p8 * noise_metadata_schedule_81_e1093);
        let noise_metadata_schedule_81_e1095: f64 = (1.0 + noise_metadata_schedule_81_e1094);
        let noise_metadata_schedule_81_e1096: f64 = (noise_metadata_schedule_81_e1087 / noise_metadata_schedule_81_e1095);
        let noise_metadata_schedule_81_e1097: f64 = (noise_metadata_schedule_81_e1084 - noise_metadata_schedule_81_e1096);
        (noise_metadata_schedule_81_e1097,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_81_e1099;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_82_e1104,) = {
    if (noise_variable_105 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_82_e1104;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_83_e1107: f64 = if noise_variable_93 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_107 = noise_metadata_schedule_83_e1107;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_84_e1115,) = {
    if (noise_variable_107 != 0.0) {
        let noise_metadata_schedule_84_e1111: f64 = (params.p4 - noise_variable_76);
        let noise_metadata_schedule_84_e1113: f64 = (noise_metadata_schedule_84_e1111).max(0.001);
        (noise_metadata_schedule_84_e1113,)
    } else {
        (noise_variable_101,)
    }
};
            noise_variable_101 = noise_metadata_schedule_84_e1115;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_85_e1130,) = {
    if (noise_variable_107 != 0.0) {
        let noise_metadata_schedule_85_e1118: f64 = (-1.0);
        let noise_metadata_schedule_85_e1120: f64 = (noise_metadata_schedule_85_e1118 * noise_variable_76);
        let noise_metadata_schedule_85_e1122: f64 = (noise_metadata_schedule_85_e1120 * params.p4);
        let noise_metadata_schedule_85_e1125: f64 = (params.p3 * noise_variable_15);
        let noise_metadata_schedule_85_e1127: f64 = (noise_metadata_schedule_85_e1125 * noise_variable_101);
        let noise_metadata_schedule_85_e1128: f64 = (noise_metadata_schedule_85_e1122 / noise_metadata_schedule_85_e1127);
        (noise_metadata_schedule_85_e1128,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_85_e1130;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_86_e1133: f64 = if noise_variable_0 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_108 = noise_metadata_schedule_86_e1133;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_87_e1143,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_108 != 0.0)) {
        let noise_metadata_schedule_87_e1140: f64 = (noise_variable_0 - 80.0);
        let noise_metadata_schedule_87_e1141: f64 = (1.0 + noise_metadata_schedule_87_e1140);
        (noise_metadata_schedule_87_e1141,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_87_e1143;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_88_e1149,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_108 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_88_e1149;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_89_e1156,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_108 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_89_e1156;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_90_e1163,) = {
    if (noise_variable_107 != 0.0) {
        let noise_metadata_schedule_90_e1160: f64 = (noise_variable_0).exp();
        let noise_metadata_schedule_90_e1161: f64 = (noise_variable_1 * noise_metadata_schedule_90_e1160);
        (noise_metadata_schedule_90_e1161,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_90_e1163;
        }
        if matches!(source_index, 3 | 4) {
            let (noise_metadata_schedule_91_e1171,) = {
    if (noise_variable_107 != 0.0) {
        let noise_metadata_schedule_91_e1168: f64 = (noise_variable_1 - 1.0);
        let noise_metadata_schedule_91_e1169: f64 = (noise_variable_93 * noise_metadata_schedule_91_e1168);
        (noise_metadata_schedule_91_e1169,)
    } else {
        (noise_variable_47,)
    }
};
            noise_variable_47 = noise_metadata_schedule_91_e1171;
        }
        if matches!(source_index, 3 | 4) {
            let (noise_metadata_schedule_92_e1176,) = {
    if (noise_variable_107 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_47,)
    }
};
            noise_variable_47 = noise_metadata_schedule_92_e1176;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_93_e1179: f64 = if noise_variable_20 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_109 = noise_metadata_schedule_93_e1179;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_94_e1187,) = {
    if (noise_variable_109 != 0.0) {
        let noise_metadata_schedule_94_e1184: f64 = (params.p59 * noise_variable_15);
        let noise_metadata_schedule_94_e1185: f64 = (noise_variable_76 / noise_metadata_schedule_94_e1184);
        (noise_metadata_schedule_94_e1185,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_94_e1187;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_95_e1198,) = {
    if (noise_variable_109 != 0.0) {
        let noise_metadata_schedule_95_e1190: f64 = (-noise_variable_76);
        let noise_metadata_schedule_95_e1192: f64 = (noise_metadata_schedule_95_e1190 - noise_variable_30);
        let noise_metadata_schedule_95_e1195: f64 = (params.p57 * noise_variable_15);
        let noise_metadata_schedule_95_e1196: f64 = (noise_metadata_schedule_95_e1192 / noise_metadata_schedule_95_e1195);
        (noise_metadata_schedule_95_e1196,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_95_e1198;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_96_e1207,) = {
    if (noise_variable_109 != 0.0) {
        let noise_metadata_schedule_96_e1201: f64 = (-noise_variable_30);
        let noise_metadata_schedule_96_e1204: f64 = (params.p57 * noise_variable_15);
        let noise_metadata_schedule_96_e1205: f64 = (noise_metadata_schedule_96_e1201 / noise_metadata_schedule_96_e1204);
        (noise_metadata_schedule_96_e1205,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_96_e1207;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let noise_metadata_schedule_97_e1210: f64 = if noise_variable_0 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_110 = noise_metadata_schedule_97_e1210;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_98_e1220,) = {
    if ((noise_variable_109 != 0.0) && (noise_variable_110 != 0.0)) {
        let noise_metadata_schedule_98_e1217: f64 = (noise_variable_0 - 80.0);
        let noise_metadata_schedule_98_e1218: f64 = (1.0 + noise_metadata_schedule_98_e1217);
        (noise_metadata_schedule_98_e1218,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_98_e1220;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_99_e1226,) = {
    if ((noise_variable_109 != 0.0) && (noise_variable_110 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_99_e1226;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_100_e1233,) = {
    if ((noise_variable_109 != 0.0) && (noise_variable_110 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_100_e1233;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_101_e1240,) = {
    if (noise_variable_109 != 0.0) {
        let noise_metadata_schedule_101_e1237: f64 = (noise_variable_0).exp();
        let noise_metadata_schedule_101_e1238: f64 = (noise_variable_1 * noise_metadata_schedule_101_e1237);
        (noise_metadata_schedule_101_e1238,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_101_e1240;
        }
        if matches!(source_index, 3 | 4 | 5) {
            let (noise_metadata_schedule_102_e1312,) = {
    if (noise_variable_109 != 0.0) {
        let noise_metadata_schedule_102_e1248: f64 = (-37.0);
        let (noise_metadata_schedule_102_e1275,) = {
            if ((!(noise_variable_90 >= 37.0)) && (!(noise_variable_90 <= noise_metadata_schedule_102_e1248))) {
                let noise_metadata_schedule_102_e1253: f64 = (noise_variable_90).exp();
                let noise_metadata_schedule_102_e1255: f64 = (noise_metadata_schedule_102_e1253 + 1.0);
                let noise_metadata_schedule_102_e1256: f64 = (noise_metadata_schedule_102_e1255).ln();
                (noise_metadata_schedule_102_e1256,)
            } else {
                let noise_metadata_schedule_102_e1263: f64 = (-37.0);
                let (noise_metadata_schedule_102_e1274,) = {
                    if ((!(noise_variable_90 >= 37.0)) && (noise_variable_90 <= noise_metadata_schedule_102_e1263)) {
                        let noise_metadata_schedule_102_e1267: f64 = (noise_variable_90).exp();
                        (noise_metadata_schedule_102_e1267,)
                    } else {
                        let (noise_metadata_schedule_102_e1273,) = {
                            if (noise_variable_90 >= 37.0) {
                                (noise_variable_90,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_102_e1273,)
                    }
                };
                (noise_metadata_schedule_102_e1274,)
            }
        };
        let noise_metadata_schedule_102_e1282: f64 = (-37.0);
        let (noise_metadata_schedule_102_e1309,) = {
            if ((!(noise_variable_91 >= 37.0)) && (!(noise_variable_91 <= noise_metadata_schedule_102_e1282))) {
                let noise_metadata_schedule_102_e1287: f64 = (noise_variable_91).exp();
                let noise_metadata_schedule_102_e1289: f64 = (noise_metadata_schedule_102_e1287 + 1.0);
                let noise_metadata_schedule_102_e1290: f64 = (noise_metadata_schedule_102_e1289).ln();
                (noise_metadata_schedule_102_e1290,)
            } else {
                let noise_metadata_schedule_102_e1297: f64 = (-37.0);
                let (noise_metadata_schedule_102_e1308,) = {
                    if ((!(noise_variable_91 >= 37.0)) && (noise_variable_91 <= noise_metadata_schedule_102_e1297)) {
                        let noise_metadata_schedule_102_e1301: f64 = (noise_variable_91).exp();
                        (noise_metadata_schedule_102_e1301,)
                    } else {
                        let (noise_metadata_schedule_102_e1307,) = {
                            if (noise_variable_91 >= 37.0) {
                                (noise_variable_91,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_102_e1307,)
                    }
                };
                (noise_metadata_schedule_102_e1308,)
            }
        };
        let noise_metadata_schedule_102_e1310: f64 = (noise_metadata_schedule_102_e1275 - noise_metadata_schedule_102_e1309);
        (noise_metadata_schedule_102_e1310,)
    } else {
        (noise_variable_2,)
    }
};
            noise_variable_2 = noise_metadata_schedule_102_e1312;
        }
        if matches!(source_index, 3 | 4) {
            let (noise_metadata_schedule_103_e1333,) = {
    if (noise_variable_109 != 0.0) {
        let noise_metadata_schedule_103_e1317: f64 = (noise_variable_1 - 1.0);
        let noise_metadata_schedule_103_e1318: f64 = (noise_variable_20 * noise_metadata_schedule_103_e1317);
        let noise_metadata_schedule_103_e1325: f64 = (noise_variable_76).abs();
        let noise_metadata_schedule_103_e1327: f64 = (noise_metadata_schedule_103_e1325).powf(noise_variable_31);
        let noise_metadata_schedule_103_e1328: f64 = (params.p8 * noise_metadata_schedule_103_e1327);
        let noise_metadata_schedule_103_e1329: f64 = (1.0 + noise_metadata_schedule_103_e1328);
        let noise_metadata_schedule_103_e1330: f64 = 0.0;
        let noise_metadata_schedule_103_e1331: f64 = (noise_metadata_schedule_103_e1318 - noise_metadata_schedule_103_e1330);
        (noise_metadata_schedule_103_e1331,)
    } else {
        (noise_variable_36,)
    }
};
            noise_variable_36 = noise_metadata_schedule_103_e1333;
        }
        if matches!(source_index, 3 | 4) {
            let (noise_metadata_schedule_104_e1338,) = {
    if (noise_variable_109 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_36,)
    }
};
            noise_variable_36 = noise_metadata_schedule_104_e1338;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_105_e1341: f64 = if noise_variable_19 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_111 = noise_metadata_schedule_105_e1341;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_106_e1349,) = {
    if (noise_variable_111 != 0.0) {
        let noise_metadata_schedule_106_e1346: f64 = (params.p61 * noise_variable_15);
        let noise_metadata_schedule_106_e1347: f64 = (noise_variable_77 / noise_metadata_schedule_106_e1346);
        (noise_metadata_schedule_106_e1347,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_106_e1349;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_107_e1360,) = {
    if (noise_variable_111 != 0.0) {
        let noise_metadata_schedule_107_e1352: f64 = (-noise_variable_77);
        let noise_metadata_schedule_107_e1354: f64 = (noise_metadata_schedule_107_e1352 - noise_variable_30);
        let noise_metadata_schedule_107_e1357: f64 = (params.p57 * noise_variable_15);
        let noise_metadata_schedule_107_e1358: f64 = (noise_metadata_schedule_107_e1354 / noise_metadata_schedule_107_e1357);
        (noise_metadata_schedule_107_e1358,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_107_e1360;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_108_e1369,) = {
    if (noise_variable_111 != 0.0) {
        let noise_metadata_schedule_108_e1363: f64 = (-noise_variable_30);
        let noise_metadata_schedule_108_e1366: f64 = (params.p57 * noise_variable_15);
        let noise_metadata_schedule_108_e1367: f64 = (noise_metadata_schedule_108_e1363 / noise_metadata_schedule_108_e1366);
        (noise_metadata_schedule_108_e1367,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_108_e1369;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_109_e1372: f64 = if noise_variable_0 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_112 = noise_metadata_schedule_109_e1372;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_110_e1382,) = {
    if ((noise_variable_111 != 0.0) && (noise_variable_112 != 0.0)) {
        let noise_metadata_schedule_110_e1379: f64 = (noise_variable_0 - 80.0);
        let noise_metadata_schedule_110_e1380: f64 = (1.0 + noise_metadata_schedule_110_e1379);
        (noise_metadata_schedule_110_e1380,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_110_e1382;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_111_e1388,) = {
    if ((noise_variable_111 != 0.0) && (noise_variable_112 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_111_e1388;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_112_e1395,) = {
    if ((noise_variable_111 != 0.0) && (noise_variable_112 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_112_e1395;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_113_e1402,) = {
    if (noise_variable_111 != 0.0) {
        let noise_metadata_schedule_113_e1399: f64 = (noise_variable_0).exp();
        let noise_metadata_schedule_113_e1400: f64 = (noise_variable_1 * noise_metadata_schedule_113_e1399);
        (noise_metadata_schedule_113_e1400,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_113_e1402;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_114_e1474,) = {
    if (noise_variable_111 != 0.0) {
        let noise_metadata_schedule_114_e1410: f64 = (-37.0);
        let (noise_metadata_schedule_114_e1437,) = {
            if ((!(noise_variable_90 >= 37.0)) && (!(noise_variable_90 <= noise_metadata_schedule_114_e1410))) {
                let noise_metadata_schedule_114_e1415: f64 = (noise_variable_90).exp();
                let noise_metadata_schedule_114_e1417: f64 = (noise_metadata_schedule_114_e1415 + 1.0);
                let noise_metadata_schedule_114_e1418: f64 = (noise_metadata_schedule_114_e1417).ln();
                (noise_metadata_schedule_114_e1418,)
            } else {
                let noise_metadata_schedule_114_e1425: f64 = (-37.0);
                let (noise_metadata_schedule_114_e1436,) = {
                    if ((!(noise_variable_90 >= 37.0)) && (noise_variable_90 <= noise_metadata_schedule_114_e1425)) {
                        let noise_metadata_schedule_114_e1429: f64 = (noise_variable_90).exp();
                        (noise_metadata_schedule_114_e1429,)
                    } else {
                        let (noise_metadata_schedule_114_e1435,) = {
                            if (noise_variable_90 >= 37.0) {
                                (noise_variable_90,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_114_e1435,)
                    }
                };
                (noise_metadata_schedule_114_e1436,)
            }
        };
        let noise_metadata_schedule_114_e1444: f64 = (-37.0);
        let (noise_metadata_schedule_114_e1471,) = {
            if ((!(noise_variable_91 >= 37.0)) && (!(noise_variable_91 <= noise_metadata_schedule_114_e1444))) {
                let noise_metadata_schedule_114_e1449: f64 = (noise_variable_91).exp();
                let noise_metadata_schedule_114_e1451: f64 = (noise_metadata_schedule_114_e1449 + 1.0);
                let noise_metadata_schedule_114_e1452: f64 = (noise_metadata_schedule_114_e1451).ln();
                (noise_metadata_schedule_114_e1452,)
            } else {
                let noise_metadata_schedule_114_e1459: f64 = (-37.0);
                let (noise_metadata_schedule_114_e1470,) = {
                    if ((!(noise_variable_91 >= 37.0)) && (noise_variable_91 <= noise_metadata_schedule_114_e1459)) {
                        let noise_metadata_schedule_114_e1463: f64 = (noise_variable_91).exp();
                        (noise_metadata_schedule_114_e1463,)
                    } else {
                        let (noise_metadata_schedule_114_e1469,) = {
                            if (noise_variable_91 >= 37.0) {
                                (noise_variable_91,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_114_e1469,)
                    }
                };
                (noise_metadata_schedule_114_e1470,)
            }
        };
        let noise_metadata_schedule_114_e1472: f64 = (noise_metadata_schedule_114_e1437 - noise_metadata_schedule_114_e1471);
        (noise_metadata_schedule_114_e1472,)
    } else {
        (noise_variable_2,)
    }
};
            noise_variable_2 = noise_metadata_schedule_114_e1474;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_115_e1495,) = {
    if (noise_variable_111 != 0.0) {
        let noise_metadata_schedule_115_e1479: f64 = (noise_variable_1 - 1.0);
        let noise_metadata_schedule_115_e1480: f64 = (noise_variable_19 * noise_metadata_schedule_115_e1479);
        let noise_metadata_schedule_115_e1483: f64 = (noise_variable_29 * noise_variable_2);
        let noise_metadata_schedule_115_e1487: f64 = (noise_variable_77).abs();
        let noise_metadata_schedule_115_e1489: f64 = (noise_metadata_schedule_115_e1487).powf(noise_variable_31);
        let noise_metadata_schedule_115_e1490: f64 = (params.p8 * noise_metadata_schedule_115_e1489);
        let noise_metadata_schedule_115_e1491: f64 = (1.0 + noise_metadata_schedule_115_e1490);
        let noise_metadata_schedule_115_e1492: f64 = (noise_metadata_schedule_115_e1483 / noise_metadata_schedule_115_e1491);
        let noise_metadata_schedule_115_e1493: f64 = (noise_metadata_schedule_115_e1480 - noise_metadata_schedule_115_e1492);
        (noise_metadata_schedule_115_e1493,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_115_e1495;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_116_e1500,) = {
    if (noise_variable_111 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_116_e1500;
        }
        if matches!(source_index, 3 | 4) {
            let noise_metadata_schedule_130_e1674: f64 = (noise_variable_35 - noise_variable_47);
            let noise_metadata_schedule_130_e1676: f64 = (noise_metadata_schedule_130_e1674 / noise_variable_16);
            let noise_metadata_schedule_130_e1678: f64 = (noise_metadata_schedule_130_e1676 + noise_variable_36);
            noise_variable_37 = noise_metadata_schedule_130_e1678;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_132_e1688: f64 = (noise_variable_77 * params.p81);
            let noise_metadata_schedule_132_e1689: f64 = (1.0 + noise_metadata_schedule_132_e1688);
            let noise_metadata_schedule_132_e1690: f64 = (noise_variable_66 * noise_metadata_schedule_132_e1689);
            noise_variable_66 = noise_metadata_schedule_132_e1690;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_133_e1693: f64 = (noise_variable_35 * noise_variable_66);
            let noise_metadata_schedule_133_e1696: f64 = (noise_variable_38 * noise_variable_67);
            let noise_metadata_schedule_133_e1697: f64 = (noise_metadata_schedule_133_e1693 + noise_metadata_schedule_133_e1696);
            noise_variable_42 = noise_metadata_schedule_133_e1697;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_134_e1701: f64 = (noise_variable_76 * noise_variable_65);
            let noise_metadata_schedule_134_e1702: f64 = (1.0 - noise_metadata_schedule_134_e1701);
            let noise_metadata_schedule_134_e1705: f64 = (noise_variable_77 * noise_variable_64);
            let noise_metadata_schedule_134_e1706: f64 = (noise_metadata_schedule_134_e1702 - noise_metadata_schedule_134_e1705);
            noise_variable_41 = noise_metadata_schedule_134_e1706;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_135_e1711: f64 = (4.0 * noise_variable_42);
            let noise_metadata_schedule_135_e1712: f64 = (1.0 + noise_metadata_schedule_135_e1711);
            let noise_metadata_schedule_135_e1713: f64 = (noise_metadata_schedule_135_e1712).abs();
            let noise_metadata_schedule_135_e1715: f64 = (noise_metadata_schedule_135_e1713).powf(params.p82);
            let noise_metadata_schedule_135_e1716: f64 = (1.0 + noise_metadata_schedule_135_e1715);
            noise_variable_96 = noise_metadata_schedule_135_e1716;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_136_e1719: f64 = (2.0 * noise_variable_41);
            let noise_metadata_schedule_136_e1721: f64 = (noise_metadata_schedule_136_e1719 / noise_variable_96);
            noise_variable_43 = noise_metadata_schedule_136_e1721;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_137_e1724: f64 = (noise_variable_38 * noise_variable_43);
            noise_variable_45 = noise_metadata_schedule_137_e1724;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_138_e1727: f64 = (noise_variable_35 * noise_variable_43);
            noise_variable_44 = noise_metadata_schedule_138_e1727;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_140_e1746: f64 = (noise_variable_79 / params.p48);
            let noise_metadata_schedule_140_e1747: f64 = (noise_metadata_schedule_140_e1746).abs();
            let noise_metadata_schedule_140_e1749: f64 = (noise_metadata_schedule_140_e1747).powf(params.p49);
            let noise_metadata_schedule_140_e1750: f64 = (1.0 + noise_metadata_schedule_140_e1749);
            noise_variable_99 = noise_metadata_schedule_140_e1750;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_141_e1754: f64 = (noise_variable_80 / params.p50);
            let noise_metadata_schedule_141_e1755: f64 = (noise_metadata_schedule_141_e1754).abs();
            let noise_metadata_schedule_141_e1757: f64 = (noise_metadata_schedule_141_e1755).powf(params.p51);
            let noise_metadata_schedule_141_e1758: f64 = (1.0 + noise_metadata_schedule_141_e1757);
            noise_variable_100 = noise_metadata_schedule_141_e1758;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_142_e1762: f64 = (noise_variable_14 * params.p37);
            let noise_metadata_schedule_142_e1763: f64 = (noise_metadata_schedule_142_e1762).exp();
            let noise_metadata_schedule_142_e1764: f64 = (params.p12 * noise_metadata_schedule_142_e1763);
            let noise_metadata_schedule_142_e1768: f64 = (1.0 / params.p49);
            let noise_metadata_schedule_142_e1769: f64 = (noise_variable_99).powf(noise_metadata_schedule_142_e1768);
            let noise_metadata_schedule_142_e1770: f64 = (noise_metadata_schedule_142_e1764 * noise_metadata_schedule_142_e1769);
            noise_variable_51 = noise_metadata_schedule_142_e1770;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_143_e1774: f64 = (noise_variable_14 * params.p78);
            let noise_metadata_schedule_143_e1775: f64 = (noise_metadata_schedule_143_e1774).exp();
            let noise_metadata_schedule_143_e1776: f64 = (params.p66 * noise_metadata_schedule_143_e1775);
            noise_variable_52 = noise_metadata_schedule_143_e1776;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_144_e1780: f64 = (noise_variable_14 * params.p38);
            let noise_metadata_schedule_144_e1781: f64 = (noise_metadata_schedule_144_e1780).exp();
            let noise_metadata_schedule_144_e1782: f64 = (params.p14 * noise_metadata_schedule_144_e1781);
            let noise_metadata_schedule_144_e1786: f64 = (1.0 / params.p51);
            let noise_metadata_schedule_144_e1787: f64 = (noise_variable_100).powf(noise_metadata_schedule_144_e1786);
            let noise_metadata_schedule_144_e1788: f64 = (noise_metadata_schedule_144_e1782 * noise_metadata_schedule_144_e1787);
            noise_variable_53 = noise_metadata_schedule_144_e1788;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_150_e1819: f64 = if params.p32 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_115 = noise_metadata_schedule_150_e1819;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_151_e1832,) = {
    if (noise_variable_115 != 0.0) {
        let noise_metadata_schedule_151_e1824: f64 = ((ctx.node_voltage(self.nodes[8]) - 0.0)).abs();
        let noise_metadata_schedule_151_e1826: f64 = (noise_metadata_schedule_151_e1824 / params.p20);
        let noise_metadata_schedule_151_e1828: f64 = (noise_metadata_schedule_151_e1826).powf(params.p44);
        let noise_metadata_schedule_151_e1829: f64 = (1.0 + noise_metadata_schedule_151_e1828);
        let noise_metadata_schedule_151_e1830: f64 = (noise_variable_51 / noise_metadata_schedule_151_e1829);
        (noise_metadata_schedule_151_e1830,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_151_e1832;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_152_e1837,) = {
    if (noise_variable_115 == 0.0) {
        (noise_variable_51,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_152_e1837;
        }
        if matches!(source_index, 0 | 1 | 2) {
            let noise_metadata_schedule_153_e1840: f64 = if params.p31 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_116 = noise_metadata_schedule_153_e1840;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_154_e1846,) = {
    if (noise_variable_116 != 0.0) {
        let noise_metadata_schedule_154_e1844: f64 = (noise_variable_51 + params.p13);
        (noise_metadata_schedule_154_e1844,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_154_e1846;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_155_e1852,) = {
    if (noise_variable_116 != 0.0) {
        let noise_metadata_schedule_155_e1850: f64 = (noise_variable_52 + params.p67);
        (noise_metadata_schedule_155_e1850,)
    } else {
        (noise_variable_52,)
    }
};
            noise_variable_52 = noise_metadata_schedule_155_e1852;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_156_e1858,) = {
    if (noise_variable_116 != 0.0) {
        let noise_metadata_schedule_156_e1856: f64 = (noise_variable_53 + params.p15);
        (noise_metadata_schedule_156_e1856,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_156_e1858;
        }
        if matches!(source_index, 0 | 1 | 2) {
            let noise_metadata_schedule_195_e2243: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_195_e2245: f64 = (noise_metadata_schedule_195_e2243 * noise_variable_10);
            noise_variable_69 = noise_metadata_schedule_195_e2245;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_196_e2249: f64 = (params.p31 * params.p13);
            let noise_metadata_schedule_196_e2250: f64 = (params.p12 + noise_metadata_schedule_196_e2249);
            let noise_metadata_schedule_196_e2252: f64 = (noise_metadata_schedule_196_e2250 / noise_variable_3);
            noise_variable_50 = noise_metadata_schedule_196_e2252;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_197_e2256: f64 = (params.p31 * params.p15);
            let noise_metadata_schedule_197_e2257: f64 = (params.p14 + noise_metadata_schedule_197_e2256);
            let noise_metadata_schedule_197_e2259: f64 = (noise_metadata_schedule_197_e2257 / noise_variable_3);
            noise_variable_48 = noise_metadata_schedule_197_e2259;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_198_e2263: f64 = (params.p31 * params.p67);
            let noise_metadata_schedule_198_e2264: f64 = (params.p66 + noise_metadata_schedule_198_e2263);
            let noise_metadata_schedule_198_e2266: f64 = (noise_metadata_schedule_198_e2264 / noise_variable_3);
            noise_variable_49 = noise_metadata_schedule_198_e2266;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_199_e2273: f64 = if ((noise_variable_50 > 0.0) && (noise_variable_50 >= params.p46)) { 1.0 } else { 0.0 };
            noise_variable_125 = noise_metadata_schedule_199_e2273;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_200_e2288,) = {
    if (noise_variable_125 != 0.0) {
        let noise_metadata_schedule_200_e2277: f64 = (noise_variable_51 / noise_variable_3);
        let (noise_metadata_schedule_200_e2286,) = {
            if (noise_metadata_schedule_200_e2277 >= params.p46) {
                let noise_metadata_schedule_200_e2283: f64 = (noise_variable_51 / noise_variable_3);
                let noise_metadata_schedule_200_e2284: f64 = (noise_variable_69 / noise_metadata_schedule_200_e2283);
                (noise_metadata_schedule_200_e2284,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_200_e2286,)
    } else {
        (noise_variable_72,)
    }
};
            noise_variable_72 = noise_metadata_schedule_200_e2288;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_201_e2295: f64 = if ((noise_variable_48 > 0.0) && (noise_variable_48 >= params.p46)) { 1.0 } else { 0.0 };
            noise_variable_126 = noise_metadata_schedule_201_e2295;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_202_e2310,) = {
    if (noise_variable_126 != 0.0) {
        let noise_metadata_schedule_202_e2299: f64 = (noise_variable_53 / noise_variable_3);
        let (noise_metadata_schedule_202_e2308,) = {
            if (noise_metadata_schedule_202_e2299 >= params.p46) {
                let noise_metadata_schedule_202_e2305: f64 = (noise_variable_53 / noise_variable_3);
                let noise_metadata_schedule_202_e2306: f64 = (noise_variable_69 / noise_metadata_schedule_202_e2305);
                (noise_metadata_schedule_202_e2306,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_202_e2308,)
    } else {
        (noise_variable_73,)
    }
};
            noise_variable_73 = noise_metadata_schedule_202_e2310;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_203_e2317: f64 = if ((noise_variable_49 > 0.0) && (noise_variable_49 >= params.p46)) { 1.0 } else { 0.0 };
            noise_variable_127 = noise_metadata_schedule_203_e2317;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_204_e2332,) = {
    if (noise_variable_127 != 0.0) {
        let noise_metadata_schedule_204_e2321: f64 = (noise_variable_52 / noise_variable_3);
        let (noise_metadata_schedule_204_e2330,) = {
            if (noise_metadata_schedule_204_e2321 >= params.p46) {
                let noise_metadata_schedule_204_e2327: f64 = (noise_variable_52 / noise_variable_3);
                let noise_metadata_schedule_204_e2328: f64 = (noise_variable_69 / noise_metadata_schedule_204_e2327);
                (noise_metadata_schedule_204_e2328,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_204_e2330,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_204_e2332;
        }
        if matches!(source_index, 3) {
            let noise_metadata_schedule_205_e2339: f64 = if ((params.p28 > 0.0) && (params.p27 > 0.0)) { 1.0 } else { 0.0 };
            let (noise_metadata_schedule_205_e2349,) = {
    if (noise_metadata_schedule_205_e2339 > 0.0) {
        let noise_metadata_schedule_205_e2344: f64 = (noise_variable_37).abs();
        let noise_metadata_schedule_205_e2346: f64 = (noise_metadata_schedule_205_e2344).powf(params.p28);
        let noise_metadata_schedule_205_e2347: f64 = (params.p27 * noise_metadata_schedule_205_e2346);
        (noise_metadata_schedule_205_e2347,)
    } else {
        (0.0,)
    }
};
            noise_variable_71 = noise_metadata_schedule_205_e2349;
        }
        if matches!(source_index, 4 | 5) {
            let noise_metadata_schedule_206_e2352: f64 = (2.0 * 1.6021918e-19);
            noise_variable_70 = noise_metadata_schedule_206_e2352;
        }
        match source_index {
            0 => {
                let noise_0_psd_e2354: f64 = 1.0;
                let noise_0_psd_e2355: f64 = (noise_0_psd_e2354 * noise_variable_72);
                let psd = noise_0_psd_e2355;
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
                let noise_1_psd_e2357: f64 = 1.0;
                let noise_1_psd_e2358: f64 = (noise_1_psd_e2357 * noise_variable_73);
                let psd = noise_1_psd_e2358;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            2 => {
                let noise_2_psd_e2360: f64 = 1.0;
                let noise_2_psd_e2361: f64 = (noise_2_psd_e2360 * noise_variable_74);
                let psd = noise_2_psd_e2361;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            3 => {
                let noise_3_psd_e2363: f64 = 1.0;
                let noise_3_psd_e429: f64 = (noise_variable_9 * noise_variable_37);
                let (noise_3_psd_e436,) = {
    if (noise_3_psd_e429 >= 0.0) {
        let noise_3_psd_e433: f64 = 1.0;
        (noise_3_psd_e433,)
    } else {
        let noise_3_psd_e435: f64 = (-1.0);
        (noise_3_psd_e435,)
    }
};
                let noise_3_psd_e438: f64 = (noise_3_psd_e436 * noise_variable_71);
                let noise_3_psd_e2364: f64 = (noise_3_psd_e2363 * noise_3_psd_e438);
                let psd = noise_3_psd_e2364;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
                let exponent: Option<f64> = Some(1.0);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            4 => {
                let noise_4_psd_e2366: f64 = 1.0;
                let noise_4_psd_e444: f64 = (noise_variable_37).abs();
                let noise_4_psd_e445: f64 = (noise_variable_70 * noise_4_psd_e444);
                let noise_4_psd_e2367: f64 = (noise_4_psd_e2366 * noise_4_psd_e445);
                let psd = noise_4_psd_e2367;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            5 => {
                let noise_5_psd_e2369: f64 = 1.0;
                let noise_5_psd_e451: f64 = (noise_variable_44 - noise_variable_45);
                let noise_5_psd_e452: f64 = (noise_5_psd_e451).abs();
                let noise_5_psd_e453: f64 = (noise_variable_70 * noise_5_psd_e452);
                let noise_5_psd_e2370: f64 = (noise_5_psd_e2369 * noise_5_psd_e453);
                let psd = noise_5_psd_e2370;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            _ => unreachable!("noise source index was range checked"),
        }
    }
}

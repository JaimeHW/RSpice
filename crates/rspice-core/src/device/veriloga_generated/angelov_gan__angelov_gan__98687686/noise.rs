#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 16] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_GGI_GDI_RGD", label: Some("Rgd"), kind: GeneratedNoiseKind::White, equation: 25, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "ggi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "gdi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_GGI_RG", label: Some("Rg"), kind: GeneratedNoiseKind::White, equation: 29, is_current: false, branch_ordinal: Some(8), pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(13), name: "ggi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_SII_RS", label: Some("Rs"), kind: GeneratedNoiseKind::White, equation: 33, is_current: false, branch_ordinal: Some(12), pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "sii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_DII_RD", label: Some("Rd"), kind: GeneratedNoiseKind::White, equation: 37, is_current: false, branch_ordinal: Some(16), pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "dii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_IDS_NOISE", label: Some("Ids noise"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_IDS_FLICKER", label: Some("Ids flicker"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IA_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(17), name: "ia", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IB_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(18), name: "ib", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_DRAIN", label: Some("drain"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GI_SI_GATE", label: Some("gate"), kind: GeneratedNoiseKind::Flicker, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GSI_SI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GDI_DI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GSI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GDI_DI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
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
        let mut noise_variable_128 = 0.0;
        let mut noise_variable_129 = 0.0;
        let mut noise_variable_130 = 0.0;
        let mut noise_variable_131 = 0.0;
        let mut noise_variable_132 = 0.0;
        let mut noise_variable_133 = 0.0;
        let mut noise_variable_134 = 0.0;
        let mut noise_variable_135 = 0.0;
        let mut noise_variable_136 = 0.0;
        let mut noise_variable_137 = 0.0;
        let mut noise_variable_138 = 0.0;
        let mut noise_variable_139 = 0.0;
        let mut noise_variable_140 = 0.0;
        let mut noise_variable_141 = 0.0;
        let mut noise_variable_142 = 0.0;
        let mut noise_variable_143 = 0.0;
        let mut noise_variable_144 = 0.0;
        if matches!(source_index, 1) {
            let noise_activation_schedule_215_e2856: f64 = if params.p46 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_125 = noise_activation_schedule_215_e2856;
        }
        if matches!(source_index, 2) {
            let noise_activation_schedule_216_e2859: f64 = if params.p50 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_126 = noise_activation_schedule_216_e2859;
        }
        if matches!(source_index, 3) {
            let noise_activation_schedule_217_e2866: f64 = if ((params.p47 > 0.0) || (params.p48 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_127 = noise_activation_schedule_217_e2866;
        }
        if matches!(source_index, 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11) {
            let noise_activation_schedule_221_e2882: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_128 = noise_activation_schedule_221_e2882;
        }
        if matches!(source_index, 6 | 7 | 8 | 9 | 10 | 11) {
            let noise_activation_schedule_222_e2885: f64 = if params.p7 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_129 = noise_activation_schedule_222_e2885;
        }
        if matches!(source_index, 11) {
            let noise_activation_schedule_236_e3077: f64 = if params.p90 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_142 = noise_activation_schedule_236_e3077;
        }
        if matches!(source_index, 14 | 15) {
            let noise_activation_schedule_237_e3080: f64 = if params.p90 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_143 = noise_activation_schedule_237_e3080;
        }
        let noise_source_active = match source_index {
            0 => {
                params.p0 != 0.0
            }
            1 => {
                let noise_1_activation_e250: f64 = if ((noise_variable_125 != 0.0) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_1_activation_e250 != 0.0
            }
            2 => {
                let noise_2_activation_e279: f64 = if ((noise_variable_126 != 0.0) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_2_activation_e279 != 0.0
            }
            3 => {
                let noise_3_activation_e308: f64 = if ((noise_variable_127 != 0.0) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_3_activation_e308 != 0.0
            }
            4 => {
                let noise_4_activation_e336: f64 = if ((noise_variable_128 != 0.0) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_4_activation_e336 != 0.0
            }
            5 => {
                let noise_5_activation_e344: f64 = if ((noise_variable_128 != 0.0) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_5_activation_e344 != 0.0
            }
            6 => {
                let noise_6_activation_e358: f64 = if (((noise_variable_129 != 0.0) && (noise_variable_128 == 0.0)) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_6_activation_e358 != 0.0
            }
            7 => {
                let noise_7_activation_e378: f64 = if (((noise_variable_129 != 0.0) && (noise_variable_128 == 0.0)) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_7_activation_e378 != 0.0
            }
            8 => {
                let noise_8_activation_e435: f64 = if (((noise_variable_129 != 0.0) && (noise_variable_128 == 0.0)) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_8_activation_e435 != 0.0
            }
            9 => {
                let noise_9_activation_e446: f64 = if (((noise_variable_129 != 0.0) && (noise_variable_128 == 0.0)) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_9_activation_e446 != 0.0
            }
            10 => {
                let noise_10_activation_e458: f64 = if (((noise_variable_129 != 0.0) && (noise_variable_128 == 0.0)) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_10_activation_e458 != 0.0
            }
            11 => {
                let noise_11_activation_e472: f64 = if ((((noise_variable_129 != 0.0) && (noise_variable_128 == 0.0)) && (params.p0 != 0.0)) && (noise_variable_142 != 0.0)) { 1.0 } else { 0.0 };
                noise_11_activation_e472 != 0.0
            }
            12 => {
                params.p0 != 0.0
            }
            13 => {
                params.p0 != 0.0
            }
            14 => {
                let noise_14_activation_e509: f64 = if ((params.p0 != 0.0) && (noise_variable_143 != 0.0)) { 1.0 } else { 0.0 };
                noise_14_activation_e509 != 0.0
            }
            15 => {
                let noise_15_activation_e523: f64 = if ((params.p0 != 0.0) && (noise_variable_143 != 0.0)) { 1.0 } else { 0.0 };
                noise_15_activation_e523 != 0.0
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
        noise_variable_128 = 0.0;
        noise_variable_129 = 0.0;
        noise_variable_130 = 0.0;
        noise_variable_131 = 0.0;
        noise_variable_132 = 0.0;
        noise_variable_133 = 0.0;
        noise_variable_134 = 0.0;
        noise_variable_135 = 0.0;
        noise_variable_136 = 0.0;
        noise_variable_137 = 0.0;
        noise_variable_138 = 0.0;
        noise_variable_139 = 0.0;
        noise_variable_140 = 0.0;
        noise_variable_141 = 0.0;
        noise_variable_142 = 0.0;
        noise_variable_143 = 0.0;
        noise_variable_144 = 0.0;
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            noise_variable_3 = (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[8]));
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            noise_variable_4 = (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[5]));
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_2_e573: f64 = (-noise_variable_4);
            noise_variable_6 = noise_metadata_schedule_2_e573;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            noise_variable_5 = (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[8]));
        }
        if matches!(source_index, 12 | 14) {
            noise_variable_96 = (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[8]));
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            noise_variable_97 = noise_variable_4;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            noise_variable_11 = (ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[8]));
        }
        if matches!(source_index, 4 | 5 | 11) {
            noise_variable_18 = (ctx.node_voltage(self.nodes[16]) - 0.0);
        }
        if matches!(source_index, 8 | 9 | 10) {
            noise_variable_98 = 0.0;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            noise_variable_25 = 0.0;
        }
        if matches!(source_index, 12 | 14) {
            noise_variable_24 = 0.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_15_e587: f64 = if self.param_given[3] { 1.0 } else { 0.0 };
            noise_variable_101 = noise_metadata_schedule_15_e587;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_16_e593,) = {
    if (noise_variable_101 != 0.0) {
        let noise_metadata_schedule_16_e591: f64 = (params.p3 + 273.15);
        (noise_metadata_schedule_16_e591,)
    } else {
        (noise_variable_15,)
    }
};
            noise_variable_15 = noise_metadata_schedule_16_e593;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_17_e600,) = {
    if (noise_variable_101 == 0.0) {
        let noise_metadata_schedule_17_e596: f64 = ctx.temperature();
        let noise_metadata_schedule_17_e598: f64 = (noise_metadata_schedule_17_e596 + params.p2);
        (noise_metadata_schedule_17_e598,)
    } else {
        (noise_variable_15,)
    }
};
            noise_variable_15 = noise_metadata_schedule_17_e600;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_18_e602: f64 = if self.param_given[100] { 1.0 } else { 0.0 };
            noise_variable_102 = noise_metadata_schedule_18_e602;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_19_e608,) = {
    if (noise_variable_102 != 0.0) {
        let noise_metadata_schedule_19_e606: f64 = (params.p100 + 273.15);
        (noise_metadata_schedule_19_e606,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_19_e608;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_20_e615,) = {
    if (noise_variable_102 == 0.0) {
        let noise_metadata_schedule_20_e613: f64 = (27.0 + 273.15);
        (noise_metadata_schedule_20_e613,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_20_e615;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_21_e622,) = {
    if (params.p1 != 0.0) {
        let noise_metadata_schedule_21_e619: f64 = ((ctx.node_voltage(self.nodes[3]) - 0.0)).abs();
        let noise_metadata_schedule_21_e620: f64 = (noise_variable_15 + noise_metadata_schedule_21_e619);
        (noise_metadata_schedule_21_e620,)
    } else {
        (noise_variable_15,)
    }
};
            noise_variable_15 = noise_metadata_schedule_21_e622;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_22_e624: f64 = (noise_variable_15 * THERMAL_VOLTAGE_PER_K);
            noise_variable_13 = noise_metadata_schedule_22_e624;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_23_e627: f64 = (noise_variable_15 - noise_variable_14);
            let noise_metadata_schedule_23_e628: f64 = (noise_metadata_schedule_23_e627).abs();
            noise_variable_16 = noise_metadata_schedule_23_e628;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_24_e635: f64 = if ((noise_variable_16 > 0.0) || (params.p66 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_103 = noise_metadata_schedule_24_e635;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_26_e657,) = {
    if (noise_variable_103 != 0.0) {
        let noise_metadata_schedule_26_e652: f64 = (noise_variable_16).abs();
        let noise_metadata_schedule_26_e653: f64 = (params.p68 * noise_metadata_schedule_26_e652);
        let noise_metadata_schedule_26_e654: f64 = (1.0 + noise_metadata_schedule_26_e653);
        let noise_metadata_schedule_26_e655: f64 = (params.p8 * noise_metadata_schedule_26_e654);
        (noise_metadata_schedule_26_e655,)
    } else {
        (noise_variable_39,)
    }
};
            noise_variable_39 = noise_metadata_schedule_26_e657;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_27_e668,) = {
    if (noise_variable_103 != 0.0) {
        let noise_metadata_schedule_27_e663: f64 = (noise_variable_16).abs();
        let noise_metadata_schedule_27_e664: f64 = (params.p80 * noise_metadata_schedule_27_e663);
        let noise_metadata_schedule_27_e665: f64 = (1.0 + noise_metadata_schedule_27_e664);
        let noise_metadata_schedule_27_e666: f64 = (params.p20 * noise_metadata_schedule_27_e665);
        (noise_metadata_schedule_27_e666,)
    } else {
        (noise_variable_43,)
    }
};
            noise_variable_43 = noise_metadata_schedule_27_e668;
        }
        if matches!(source_index, 6 | 7 | 9) {
            let (noise_metadata_schedule_28_e679,) = {
    if (noise_variable_103 != 0.0) {
        let noise_metadata_schedule_28_e674: f64 = (noise_variable_16).abs();
        let noise_metadata_schedule_28_e675: f64 = (params.p72 * noise_metadata_schedule_28_e674);
        let noise_metadata_schedule_28_e676: f64 = (1.0 + noise_metadata_schedule_28_e675);
        let noise_metadata_schedule_28_e677: f64 = (params.p26 * noise_metadata_schedule_28_e676);
        (noise_metadata_schedule_28_e677,)
    } else {
        (noise_variable_44,)
    }
};
            noise_variable_44 = noise_metadata_schedule_28_e679;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_32_e720,) = {
    if (noise_variable_103 != 0.0) {
        let noise_metadata_schedule_32_e717: f64 = (params.p78 * noise_variable_16);
        let noise_metadata_schedule_32_e718: f64 = (params.p9 + noise_metadata_schedule_32_e717);
        (noise_metadata_schedule_32_e718,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_32_e720;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_35_e748,) = {
    if (noise_variable_103 != 0.0) {
        let noise_metadata_schedule_35_e745: f64 = (params.p79 * noise_variable_16);
        let noise_metadata_schedule_35_e746: f64 = (params.p45 + noise_metadata_schedule_35_e745);
        (noise_metadata_schedule_35_e746,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_35_e748;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_36_e756,) = {
    if (noise_variable_103 != 0.0) {
        let noise_metadata_schedule_36_e753: f64 = (params.p81 * noise_variable_16);
        let noise_metadata_schedule_36_e754: f64 = (params.p21 + noise_metadata_schedule_36_e753);
        (noise_metadata_schedule_36_e754,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_36_e756;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_42_e828,) = {
    if (noise_variable_103 == 0.0) {
        (params.p8,)
    } else {
        (noise_variable_39,)
    }
};
            noise_variable_39 = noise_metadata_schedule_42_e828;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_43_e833,) = {
    if (noise_variable_103 == 0.0) {
        (params.p20,)
    } else {
        (noise_variable_43,)
    }
};
            noise_variable_43 = noise_metadata_schedule_43_e833;
        }
        if matches!(source_index, 6 | 7 | 9) {
            let (noise_metadata_schedule_44_e838,) = {
    if (noise_variable_103 == 0.0) {
        (params.p26,)
    } else {
        (noise_variable_44,)
    }
};
            noise_variable_44 = noise_metadata_schedule_44_e838;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_50_e868,) = {
    if (noise_variable_103 == 0.0) {
        (params.p9,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_50_e868;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_53_e883,) = {
    if (noise_variable_103 == 0.0) {
        (params.p45,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_53_e883;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_54_e888,) = {
    if (noise_variable_103 == 0.0) {
        (params.p21,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_54_e888;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_55_e894: f64 = if ((!self.param_given[43]) && self.param_given[44]) { 1.0 } else { 0.0 };
            noise_variable_105 = noise_metadata_schedule_55_e894;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_56_e902,) = {
    if (noise_variable_105 != 0.0) {
        let noise_metadata_schedule_56_e898: f64 = (0.5 / params.p44);
        let noise_metadata_schedule_56_e900: f64 = (noise_metadata_schedule_56_e898 / noise_variable_13);
        (noise_metadata_schedule_56_e900,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_56_e902;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_57_e907,) = {
    if (noise_variable_105 == 0.0) {
        (params.p43,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_57_e907;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_58_e910: f64 = (params.p19 * noise_variable_5);
            let noise_metadata_schedule_58_e911: f64 = (noise_metadata_schedule_58_e910).cosh();
            noise_variable_63 = noise_metadata_schedule_58_e911;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_59_e914: f64 = (params.p64 * noise_variable_11);
            noise_variable_12 = noise_metadata_schedule_59_e914;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_60_e921: f64 = (noise_variable_63 * noise_variable_63);
            let noise_metadata_schedule_60_e922: f64 = (1e-12 + noise_metadata_schedule_60_e921);
            let noise_metadata_schedule_60_e923: f64 = (params.p18 / noise_metadata_schedule_60_e922);
            let noise_metadata_schedule_60_e924: f64 = (1.0 + noise_metadata_schedule_60_e923);
            let noise_metadata_schedule_60_e925: f64 = (params.p11 * noise_metadata_schedule_60_e924);
            noise_variable_59 = noise_metadata_schedule_60_e925;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_61_e930: f64 = (noise_variable_16).abs();
            let noise_metadata_schedule_61_e931: f64 = (params.p69 * noise_metadata_schedule_61_e930);
            let noise_metadata_schedule_61_e932: f64 = (1.0 + noise_metadata_schedule_61_e931);
            let noise_metadata_schedule_61_e933: f64 = (noise_variable_59 * noise_metadata_schedule_61_e932);
            noise_variable_60 = noise_metadata_schedule_61_e933;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_62_e938: f64 = (noise_variable_16).abs();
            let noise_metadata_schedule_62_e939: f64 = (params.p70 * noise_metadata_schedule_62_e938);
            let noise_metadata_schedule_62_e940: f64 = (1.0 + noise_metadata_schedule_62_e939);
            let noise_metadata_schedule_62_e941: f64 = (params.p13 * noise_metadata_schedule_62_e940);
            noise_variable_61 = noise_metadata_schedule_62_e941;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_63_e944: f64 = (noise_variable_54 - params.p10);
            let noise_metadata_schedule_63_e948: f64 = (params.p15 * noise_variable_5);
            let noise_metadata_schedule_63_e949: f64 = (noise_metadata_schedule_63_e948).tanh();
            let noise_metadata_schedule_63_e950: f64 = (params.p10 * noise_metadata_schedule_63_e949);
            let noise_metadata_schedule_63_e951: f64 = (noise_metadata_schedule_63_e944 + noise_metadata_schedule_63_e950);
            let noise_metadata_schedule_63_e953: f64 = (noise_metadata_schedule_63_e951 - noise_variable_12);
            let noise_metadata_schedule_63_e957: f64 = (noise_variable_6 - noise_variable_53);
            let noise_metadata_schedule_63_e958: f64 = (params.p22 * noise_metadata_schedule_63_e957);
            let noise_metadata_schedule_63_e961: f64 = (noise_variable_6 - noise_variable_53);
            let noise_metadata_schedule_63_e962: f64 = (noise_metadata_schedule_63_e958 * noise_metadata_schedule_63_e961);
            let noise_metadata_schedule_63_e963: f64 = (noise_metadata_schedule_63_e953 - noise_metadata_schedule_63_e962);
            noise_variable_62 = noise_metadata_schedule_63_e963;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_64_e968: f64 = (noise_variable_16).abs();
            let noise_metadata_schedule_64_e969: f64 = (params.p78 * noise_metadata_schedule_64_e968);
            let noise_metadata_schedule_64_e970: f64 = (1.0 + noise_metadata_schedule_64_e969);
            let noise_metadata_schedule_64_e971: f64 = (noise_variable_62 * noise_metadata_schedule_64_e970);
            noise_variable_58 = noise_metadata_schedule_64_e971;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_65_e974: f64 = (noise_variable_3 - noise_variable_58);
            noise_variable_64 = noise_metadata_schedule_65_e974;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_66_e977: f64 = (noise_variable_64 * noise_variable_64);
            noise_variable_65 = noise_metadata_schedule_66_e977;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_67_e980: f64 = (noise_variable_60 * noise_variable_64);
            let noise_metadata_schedule_67_e983: f64 = (params.p12 * noise_variable_65);
            let noise_metadata_schedule_67_e984: f64 = (noise_metadata_schedule_67_e980 + noise_metadata_schedule_67_e983);
            let noise_metadata_schedule_67_e987: f64 = (noise_variable_61 * noise_variable_64);
            let noise_metadata_schedule_67_e989: f64 = (noise_metadata_schedule_67_e987 * noise_variable_65);
            let noise_metadata_schedule_67_e990: f64 = (noise_metadata_schedule_67_e984 + noise_metadata_schedule_67_e989);
            noise_variable_17 = noise_metadata_schedule_67_e990;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_68_e993: f64 = (noise_variable_17).tanh();
            let noise_metadata_schedule_68_e994: f64 = (1.0 + noise_metadata_schedule_68_e993);
            noise_variable_75 = noise_metadata_schedule_68_e994;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let noise_metadata_schedule_69_e998: f64 = { let limexp_arg = noise_variable_17; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_69_e1000: f64 = (-noise_variable_17);
            let noise_metadata_schedule_69_e1001: f64 = { let limexp_arg = noise_metadata_schedule_69_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_69_e1002: f64 = (noise_metadata_schedule_69_e998 - noise_metadata_schedule_69_e1001);
            let noise_metadata_schedule_69_e1003: f64 = (0.5 * noise_metadata_schedule_69_e1002);
            let noise_metadata_schedule_69_e1004: f64 = (noise_metadata_schedule_69_e1003).tanh();
            let noise_metadata_schedule_69_e1005: f64 = (1.0 + noise_metadata_schedule_69_e1004);
            noise_variable_76 = noise_metadata_schedule_69_e1005;
        }
        if matches!(source_index, 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_70_e1009: f64 = (params.p15 * noise_variable_75);
            let noise_metadata_schedule_70_e1010: f64 = (params.p14 + noise_metadata_schedule_70_e1009);
            noise_variable_0 = noise_metadata_schedule_70_e1010;
        }
        if matches!(source_index, 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_71_e1013: f64 = (noise_variable_0 * noise_variable_5);
            let noise_metadata_schedule_71_e1014: f64 = (noise_metadata_schedule_71_e1013).tanh();
            noise_variable_79 = noise_metadata_schedule_71_e1014;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_72_e1017: f64 = if params.p4 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_106 = noise_metadata_schedule_72_e1017;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_73_e1020: f64 = if params.p4 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_107 = noise_metadata_schedule_73_e1020;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_74_e1023: f64 = if params.p4 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_108 = noise_metadata_schedule_74_e1023;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_75_e1026: f64 = if params.p4 == 3.0 { 1.0 } else { 0.0 };
            noise_variable_109 = noise_metadata_schedule_75_e1026;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let noise_metadata_schedule_76_e1029: f64 = if params.p4 == 4.0 { 1.0 } else { 0.0 };
            noise_variable_110 = noise_metadata_schedule_76_e1029;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_77_e1050,) = {
    if (noise_variable_106 != 0.0) {
        let noise_metadata_schedule_77_e1033: f64 = (noise_variable_39 * noise_variable_75);
        let noise_metadata_schedule_77_e1035: f64 = (noise_metadata_schedule_77_e1033 * noise_variable_79);
        let noise_metadata_schedule_77_e1039: f64 = (params.p16 * noise_variable_5);
        let noise_metadata_schedule_77_e1040: f64 = (1.0 + noise_metadata_schedule_77_e1039);
        let noise_metadata_schedule_77_e1044: f64 = (noise_variable_6 - noise_variable_53);
        let noise_metadata_schedule_77_e1045: f64 = { let limexp_arg = noise_metadata_schedule_77_e1044; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_77_e1046: f64 = (noise_variable_43 * noise_metadata_schedule_77_e1045);
        let noise_metadata_schedule_77_e1047: f64 = (noise_metadata_schedule_77_e1040 + noise_metadata_schedule_77_e1046);
        let noise_metadata_schedule_77_e1048: f64 = (noise_metadata_schedule_77_e1035 * noise_metadata_schedule_77_e1047);
        (noise_metadata_schedule_77_e1048,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_77_e1050;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_78_e1059,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_106 == 0.0)) {
        let noise_metadata_schedule_78_e1057: f64 = (noise_variable_4 - noise_variable_58);
        (noise_metadata_schedule_78_e1057,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_78_e1059;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_79_e1068,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_106 == 0.0)) {
        let noise_metadata_schedule_79_e1066: f64 = (noise_variable_63 * noise_variable_63);
        (noise_metadata_schedule_79_e1066,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_79_e1068;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_80_e1077,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_106 == 0.0)) {
        let noise_metadata_schedule_80_e1075: f64 = (noise_variable_64 * noise_variable_63);
        (noise_metadata_schedule_80_e1075,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_80_e1077;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_81_e1094,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_106 == 0.0)) {
        let noise_metadata_schedule_81_e1084: f64 = (noise_variable_60 * noise_variable_63);
        let noise_metadata_schedule_81_e1087: f64 = (params.p12 * noise_variable_64);
        let noise_metadata_schedule_81_e1088: f64 = (noise_metadata_schedule_81_e1084 + noise_metadata_schedule_81_e1087);
        let noise_metadata_schedule_81_e1091: f64 = (noise_variable_61 * noise_variable_65);
        let noise_metadata_schedule_81_e1092: f64 = (noise_metadata_schedule_81_e1088 + noise_metadata_schedule_81_e1091);
        (noise_metadata_schedule_81_e1092,)
    } else {
        (noise_variable_71,)
    }
};
            noise_variable_71 = noise_metadata_schedule_81_e1094;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_82_e1104,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_106 == 0.0)) {
        let noise_metadata_schedule_82_e1101: f64 = (noise_variable_71).tanh();
        let noise_metadata_schedule_82_e1102: f64 = (1.0 + noise_metadata_schedule_82_e1101);
        (noise_metadata_schedule_82_e1102,)
    } else {
        (noise_variable_77,)
    }
};
            noise_variable_77 = noise_metadata_schedule_82_e1104;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_83_e1115,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_106 == 0.0)) {
        let noise_metadata_schedule_83_e1112: f64 = (params.p15 * noise_variable_77);
        let noise_metadata_schedule_83_e1113: f64 = (params.p14 + noise_metadata_schedule_83_e1112);
        (noise_metadata_schedule_83_e1113,)
    } else {
        (noise_variable_72,)
    }
};
            noise_variable_72 = noise_metadata_schedule_83_e1115;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_84_e1126,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_106 == 0.0)) {
        let noise_metadata_schedule_84_e1123: f64 = (params.p17 * noise_variable_75);
        let noise_metadata_schedule_84_e1124: f64 = (params.p16 + noise_metadata_schedule_84_e1123);
        (noise_metadata_schedule_84_e1124,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_84_e1126;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_85_e1154,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_106 == 0.0)) {
        let noise_metadata_schedule_85_e1133: f64 = (noise_variable_39 * noise_variable_75);
        let noise_metadata_schedule_85_e1136: f64 = (1.0 + noise_variable_79);
        let noise_metadata_schedule_85_e1137: f64 = (noise_metadata_schedule_85_e1133 * noise_metadata_schedule_85_e1136);
        let noise_metadata_schedule_85_e1141: f64 = (noise_variable_69 * noise_variable_5);
        let noise_metadata_schedule_85_e1142: f64 = (1.0 + noise_metadata_schedule_85_e1141);
        let noise_metadata_schedule_85_e1147: f64 = (noise_variable_5 - noise_variable_53);
        let noise_metadata_schedule_85_e1148: f64 = (params.p23 * noise_metadata_schedule_85_e1147);
        let noise_metadata_schedule_85_e1149: f64 = { let limexp_arg = noise_metadata_schedule_85_e1148; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_85_e1150: f64 = (noise_variable_43 * noise_metadata_schedule_85_e1149);
        let noise_metadata_schedule_85_e1151: f64 = (noise_metadata_schedule_85_e1142 + noise_metadata_schedule_85_e1150);
        let noise_metadata_schedule_85_e1152: f64 = (noise_metadata_schedule_85_e1137 * noise_metadata_schedule_85_e1151);
        (noise_metadata_schedule_85_e1152,)
    } else {
        (noise_variable_73,)
    }
};
            noise_variable_73 = noise_metadata_schedule_85_e1154;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_86_e1165,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_106 == 0.0)) {
        let noise_metadata_schedule_86_e1162: f64 = (params.p17 * noise_variable_77);
        let noise_metadata_schedule_86_e1163: f64 = (params.p16 + noise_metadata_schedule_86_e1162);
        (noise_metadata_schedule_86_e1163,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_86_e1165;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_87_e1175,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_106 == 0.0)) {
        let noise_metadata_schedule_87_e1172: f64 = (noise_variable_72 * noise_variable_5);
        let noise_metadata_schedule_87_e1173: f64 = (noise_metadata_schedule_87_e1172).tanh();
        (noise_metadata_schedule_87_e1173,)
    } else {
        (noise_variable_80,)
    }
};
            noise_variable_80 = noise_metadata_schedule_87_e1175;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_88_e1194,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_106 == 0.0)) {
        let noise_metadata_schedule_88_e1182: f64 = (noise_variable_39 * noise_variable_77);
        let noise_metadata_schedule_88_e1185: f64 = (1.0 - noise_variable_80);
        let noise_metadata_schedule_88_e1186: f64 = (noise_metadata_schedule_88_e1182 * noise_metadata_schedule_88_e1185);
        let noise_metadata_schedule_88_e1190: f64 = (noise_variable_67 * noise_variable_5);
        let noise_metadata_schedule_88_e1191: f64 = (1.0 - noise_metadata_schedule_88_e1190);
        let noise_metadata_schedule_88_e1192: f64 = (noise_metadata_schedule_88_e1186 * noise_metadata_schedule_88_e1191);
        (noise_metadata_schedule_88_e1192,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_88_e1194;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_89_e1205,) = {
    if ((noise_variable_107 != 0.0) && (noise_variable_106 == 0.0)) {
        let noise_metadata_schedule_89_e1202: f64 = (noise_variable_73 - noise_variable_74);
        let noise_metadata_schedule_89_e1203: f64 = (0.5 * noise_metadata_schedule_89_e1202);
        (noise_metadata_schedule_89_e1203,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_89_e1205;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_90_e1216,) = {
    if ((noise_variable_108 != 0.0) && (!((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)))) {
        let noise_metadata_schedule_90_e1214: f64 = (noise_variable_3 - noise_variable_58);
        (noise_metadata_schedule_90_e1214,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_90_e1216;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_91_e1227,) = {
    if ((noise_variable_108 != 0.0) && (!((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)))) {
        let noise_metadata_schedule_91_e1225: f64 = (noise_variable_63 * noise_variable_63);
        (noise_metadata_schedule_91_e1225,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_91_e1227;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_92_e1248,) = {
    if ((noise_variable_108 != 0.0) && (!((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)))) {
        let noise_metadata_schedule_92_e1238: f64 = (params.p12 * noise_variable_64);
        let noise_metadata_schedule_92_e1239: f64 = (noise_variable_63 + noise_metadata_schedule_92_e1238);
        let noise_metadata_schedule_92_e1242: f64 = (noise_variable_61 * noise_variable_64);
        let noise_metadata_schedule_92_e1244: f64 = (noise_metadata_schedule_92_e1242 * noise_variable_63);
        let noise_metadata_schedule_92_e1245: f64 = (noise_metadata_schedule_92_e1239 + noise_metadata_schedule_92_e1244);
        let noise_metadata_schedule_92_e1246: f64 = (noise_variable_60 * noise_metadata_schedule_92_e1245);
        (noise_metadata_schedule_92_e1246,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_92_e1248;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_93_e1267,) = {
    if ((noise_variable_108 != 0.0) && (!((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)))) {
        let noise_metadata_schedule_93_e1258: f64 = { let limexp_arg = noise_variable_17; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_93_e1260: f64 = (-noise_variable_17);
        let noise_metadata_schedule_93_e1261: f64 = { let limexp_arg = noise_metadata_schedule_93_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_93_e1262: f64 = (noise_metadata_schedule_93_e1258 - noise_metadata_schedule_93_e1261);
        let noise_metadata_schedule_93_e1263: f64 = (0.5 * noise_metadata_schedule_93_e1262);
        let noise_metadata_schedule_93_e1264: f64 = (noise_metadata_schedule_93_e1263).tanh();
        let noise_metadata_schedule_93_e1265: f64 = (1.0 + noise_metadata_schedule_93_e1264);
        (noise_metadata_schedule_93_e1265,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_93_e1267;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_94_e1280,) = {
    if ((noise_variable_108 != 0.0) && (!((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)))) {
        let noise_metadata_schedule_94_e1277: f64 = (params.p15 * noise_variable_76);
        let noise_metadata_schedule_94_e1278: f64 = (params.p14 + noise_metadata_schedule_94_e1277);
        (noise_metadata_schedule_94_e1278,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_94_e1280;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_95_e1292,) = {
    if ((noise_variable_108 != 0.0) && (!((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)))) {
        let noise_metadata_schedule_95_e1289: f64 = (noise_variable_1 * noise_variable_5);
        let noise_metadata_schedule_95_e1290: f64 = (noise_metadata_schedule_95_e1289).tanh();
        (noise_metadata_schedule_95_e1290,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_95_e1292;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_96_e1305,) = {
    if ((noise_variable_108 != 0.0) && (!((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)))) {
        let noise_metadata_schedule_96_e1302: f64 = (params.p17 * noise_variable_76);
        let noise_metadata_schedule_96_e1303: f64 = (params.p16 + noise_metadata_schedule_96_e1302);
        (noise_metadata_schedule_96_e1303,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_96_e1305;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_97_e1333,) = {
    if ((noise_variable_108 != 0.0) && (!((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)))) {
        let noise_metadata_schedule_97_e1314: f64 = (noise_variable_39 * noise_variable_76);
        let noise_metadata_schedule_97_e1316: f64 = (noise_metadata_schedule_97_e1314 * noise_variable_81);
        let noise_metadata_schedule_97_e1320: f64 = (noise_variable_69 * noise_variable_5);
        let noise_metadata_schedule_97_e1321: f64 = (1.0 + noise_metadata_schedule_97_e1320);
        let noise_metadata_schedule_97_e1326: f64 = (noise_variable_6 - noise_variable_53);
        let noise_metadata_schedule_97_e1327: f64 = (params.p23 * noise_metadata_schedule_97_e1326);
        let noise_metadata_schedule_97_e1328: f64 = { let limexp_arg = noise_metadata_schedule_97_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_97_e1329: f64 = (noise_variable_43 * noise_metadata_schedule_97_e1328);
        let noise_metadata_schedule_97_e1330: f64 = (noise_metadata_schedule_97_e1321 + noise_metadata_schedule_97_e1329);
        let noise_metadata_schedule_97_e1331: f64 = (noise_metadata_schedule_97_e1316 * noise_metadata_schedule_97_e1330);
        (noise_metadata_schedule_97_e1331,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_97_e1333;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_98_e1346,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_98_e1344: f64 = (noise_variable_3 - noise_variable_58);
        (noise_metadata_schedule_98_e1344,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_98_e1346;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_99_e1359,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_99_e1357: f64 = (noise_variable_63 * noise_variable_63);
        (noise_metadata_schedule_99_e1357,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_99_e1359;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_100_e1382,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_100_e1372: f64 = (params.p12 * noise_variable_64);
        let noise_metadata_schedule_100_e1373: f64 = (noise_variable_63 + noise_metadata_schedule_100_e1372);
        let noise_metadata_schedule_100_e1376: f64 = (noise_variable_61 * noise_variable_64);
        let noise_metadata_schedule_100_e1378: f64 = (noise_metadata_schedule_100_e1376 * noise_variable_63);
        let noise_metadata_schedule_100_e1379: f64 = (noise_metadata_schedule_100_e1373 + noise_metadata_schedule_100_e1378);
        let noise_metadata_schedule_100_e1380: f64 = (noise_variable_60 * noise_metadata_schedule_100_e1379);
        (noise_metadata_schedule_100_e1380,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_100_e1382;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_101_e1395,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_101_e1393: f64 = (noise_variable_4 - noise_variable_58);
        (noise_metadata_schedule_101_e1393,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_101_e1395;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_102_e1408,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_102_e1406: f64 = (noise_variable_65 * noise_variable_65);
        (noise_metadata_schedule_102_e1406,)
    } else {
        (noise_variable_66,)
    }
};
            noise_variable_66 = noise_metadata_schedule_102_e1408;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_103_e1431,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_103_e1421: f64 = (params.p12 * noise_variable_66);
        let noise_metadata_schedule_103_e1422: f64 = (noise_variable_65 + noise_metadata_schedule_103_e1421);
        let noise_metadata_schedule_103_e1425: f64 = (noise_variable_61 * noise_variable_65);
        let noise_metadata_schedule_103_e1427: f64 = (noise_metadata_schedule_103_e1425 * noise_variable_66);
        let noise_metadata_schedule_103_e1428: f64 = (noise_metadata_schedule_103_e1422 + noise_metadata_schedule_103_e1427);
        let noise_metadata_schedule_103_e1429: f64 = (noise_variable_60 * noise_metadata_schedule_103_e1428);
        (noise_metadata_schedule_103_e1429,)
    } else {
        (noise_variable_71,)
    }
};
            noise_variable_71 = noise_metadata_schedule_103_e1431;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_104_e1452,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_104_e1443: f64 = { let limexp_arg = noise_variable_17; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_104_e1445: f64 = (-noise_variable_17);
        let noise_metadata_schedule_104_e1446: f64 = { let limexp_arg = noise_metadata_schedule_104_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_104_e1447: f64 = (noise_metadata_schedule_104_e1443 - noise_metadata_schedule_104_e1446);
        let noise_metadata_schedule_104_e1448: f64 = (0.5 * noise_metadata_schedule_104_e1447);
        let noise_metadata_schedule_104_e1449: f64 = (noise_metadata_schedule_104_e1448).tanh();
        let noise_metadata_schedule_104_e1450: f64 = (1.0 + noise_metadata_schedule_104_e1449);
        (noise_metadata_schedule_104_e1450,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_104_e1452;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_105_e1473,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_105_e1464: f64 = { let limexp_arg = noise_variable_71; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_105_e1466: f64 = (-noise_variable_71);
        let noise_metadata_schedule_105_e1467: f64 = { let limexp_arg = noise_metadata_schedule_105_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_105_e1468: f64 = (noise_metadata_schedule_105_e1464 - noise_metadata_schedule_105_e1467);
        let noise_metadata_schedule_105_e1469: f64 = (0.5 * noise_metadata_schedule_105_e1468);
        let noise_metadata_schedule_105_e1470: f64 = (noise_metadata_schedule_105_e1469).tanh();
        let noise_metadata_schedule_105_e1471: f64 = (1.0 + noise_metadata_schedule_105_e1470);
        (noise_metadata_schedule_105_e1471,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_105_e1473;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_106_e1488,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_106_e1485: f64 = (params.p15 * noise_variable_76);
        let noise_metadata_schedule_106_e1486: f64 = (params.p14 + noise_metadata_schedule_106_e1485);
        (noise_metadata_schedule_106_e1486,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_106_e1488;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_107_e1503,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_107_e1500: f64 = (params.p15 * noise_variable_78);
        let noise_metadata_schedule_107_e1501: f64 = (params.p14 + noise_metadata_schedule_107_e1500);
        (noise_metadata_schedule_107_e1501,)
    } else {
        (noise_variable_2,)
    }
};
            noise_variable_2 = noise_metadata_schedule_107_e1503;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_108_e1517,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_108_e1514: f64 = (noise_variable_1 * noise_variable_5);
        let noise_metadata_schedule_108_e1515: f64 = (noise_metadata_schedule_108_e1514).tanh();
        (noise_metadata_schedule_108_e1515,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_108_e1517;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_109_e1531,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_109_e1528: f64 = (noise_variable_2 * noise_variable_5);
        let noise_metadata_schedule_109_e1529: f64 = (noise_metadata_schedule_109_e1528).tanh();
        (noise_metadata_schedule_109_e1529,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_109_e1531;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_110_e1546,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_110_e1543: f64 = (params.p17 * noise_variable_78);
        let noise_metadata_schedule_110_e1544: f64 = (params.p16 + noise_metadata_schedule_110_e1543);
        (noise_metadata_schedule_110_e1544,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_110_e1546;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_111_e1561,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_111_e1558: f64 = (params.p17 * noise_variable_76);
        let noise_metadata_schedule_111_e1559: f64 = (params.p16 + noise_metadata_schedule_111_e1558);
        (noise_metadata_schedule_111_e1559,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_111_e1561;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_112_e1593,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_112_e1572: f64 = (noise_variable_39 * noise_variable_76);
        let noise_metadata_schedule_112_e1575: f64 = (1.0 + noise_variable_81);
        let noise_metadata_schedule_112_e1576: f64 = (noise_metadata_schedule_112_e1572 * noise_metadata_schedule_112_e1575);
        let noise_metadata_schedule_112_e1580: f64 = (noise_variable_70 * noise_variable_5);
        let noise_metadata_schedule_112_e1581: f64 = (1.0 + noise_metadata_schedule_112_e1580);
        let noise_metadata_schedule_112_e1586: f64 = (noise_variable_5 - noise_variable_53);
        let noise_metadata_schedule_112_e1587: f64 = (params.p23 * noise_metadata_schedule_112_e1586);
        let noise_metadata_schedule_112_e1588: f64 = { let limexp_arg = noise_metadata_schedule_112_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_112_e1589: f64 = (noise_variable_43 * noise_metadata_schedule_112_e1588);
        let noise_metadata_schedule_112_e1590: f64 = (noise_metadata_schedule_112_e1581 + noise_metadata_schedule_112_e1589);
        let noise_metadata_schedule_112_e1591: f64 = (noise_metadata_schedule_112_e1576 * noise_metadata_schedule_112_e1590);
        (noise_metadata_schedule_112_e1591,)
    } else {
        (noise_variable_73,)
    }
};
            noise_variable_73 = noise_metadata_schedule_112_e1593;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_113_e1616,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_113_e1604: f64 = (noise_variable_39 * noise_variable_78);
        let noise_metadata_schedule_113_e1607: f64 = (1.0 - noise_variable_82);
        let noise_metadata_schedule_113_e1608: f64 = (noise_metadata_schedule_113_e1604 * noise_metadata_schedule_113_e1607);
        let noise_metadata_schedule_113_e1612: f64 = (noise_variable_68 * noise_variable_5);
        let noise_metadata_schedule_113_e1613: f64 = (1.0 - noise_metadata_schedule_113_e1612);
        let noise_metadata_schedule_113_e1614: f64 = (noise_metadata_schedule_113_e1608 * noise_metadata_schedule_113_e1613);
        (noise_metadata_schedule_113_e1614,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_113_e1616;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_114_e1631,) = {
    if ((noise_variable_109 != 0.0) && (!(((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)))) {
        let noise_metadata_schedule_114_e1628: f64 = (noise_variable_73 - noise_variable_74);
        let noise_metadata_schedule_114_e1629: f64 = (0.5 * noise_metadata_schedule_114_e1628);
        (noise_metadata_schedule_114_e1629,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_114_e1631;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_115_e1648,) = {
    if ((noise_variable_110 != 0.0) && (!((((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)) || (noise_variable_109 != 0.0)))) {
        let noise_metadata_schedule_115_e1645: f64 = (params.p17 * noise_variable_75);
        let noise_metadata_schedule_115_e1646: f64 = (params.p16 + noise_metadata_schedule_115_e1645);
        (noise_metadata_schedule_115_e1646,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_115_e1648;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_116_e1665,) = {
    if ((noise_variable_110 != 0.0) && (!((((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)) || (noise_variable_109 != 0.0)))) {
        let noise_metadata_schedule_116_e1662: f64 = (params.p15 * noise_variable_76);
        let noise_metadata_schedule_116_e1663: f64 = (params.p14 + noise_metadata_schedule_116_e1662);
        (noise_metadata_schedule_116_e1663,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_116_e1665;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_117_e1681,) = {
    if ((noise_variable_110 != 0.0) && (!((((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)) || (noise_variable_109 != 0.0)))) {
        let noise_metadata_schedule_117_e1678: f64 = (noise_variable_1 * noise_variable_5);
        let noise_metadata_schedule_117_e1679: f64 = (noise_metadata_schedule_117_e1678).tanh();
        (noise_metadata_schedule_117_e1679,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_117_e1681;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_118_e1697,) = {
    if ((noise_variable_110 != 0.0) && (!((((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)) || (noise_variable_109 != 0.0)))) {
        let noise_metadata_schedule_118_e1694: f64 = (noise_variable_1 * noise_variable_11);
        let noise_metadata_schedule_118_e1695: f64 = (noise_metadata_schedule_118_e1694).tanh();
        (noise_metadata_schedule_118_e1695,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_118_e1697;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_119_e1737,) = {
    if ((noise_variable_110 != 0.0) && (!((((noise_variable_106 != 0.0) || (noise_variable_107 != 0.0)) || (noise_variable_108 != 0.0)) || (noise_variable_109 != 0.0)))) {
        let noise_metadata_schedule_119_e1710: f64 = (noise_variable_39 * noise_variable_75);
        let noise_metadata_schedule_119_e1714: f64 = (params.p65 * noise_variable_83);
        let noise_metadata_schedule_119_e1715: f64 = (noise_variable_81 + noise_metadata_schedule_119_e1714);
        let noise_metadata_schedule_119_e1716: f64 = (noise_metadata_schedule_119_e1710 * noise_metadata_schedule_119_e1715);
        let noise_metadata_schedule_119_e1722: f64 = (params.p65 * noise_variable_11);
        let noise_metadata_schedule_119_e1723: f64 = (noise_variable_5 + noise_metadata_schedule_119_e1722);
        let noise_metadata_schedule_119_e1724: f64 = (noise_variable_69 * noise_metadata_schedule_119_e1723);
        let noise_metadata_schedule_119_e1725: f64 = (1.0 + noise_metadata_schedule_119_e1724);
        let noise_metadata_schedule_119_e1730: f64 = (noise_variable_5 - noise_variable_53);
        let noise_metadata_schedule_119_e1731: f64 = (params.p23 * noise_metadata_schedule_119_e1730);
        let noise_metadata_schedule_119_e1732: f64 = { let limexp_arg = noise_metadata_schedule_119_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_119_e1733: f64 = (noise_variable_43 * noise_metadata_schedule_119_e1732);
        let noise_metadata_schedule_119_e1734: f64 = (noise_metadata_schedule_119_e1725 + noise_metadata_schedule_119_e1733);
        let noise_metadata_schedule_119_e1735: f64 = (noise_metadata_schedule_119_e1716 * noise_metadata_schedule_119_e1734);
        (noise_metadata_schedule_119_e1735,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_119_e1737;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_120_e1748: f64 = if (((params.p4 == 0.0) || (params.p4 == 1.0)) || (params.p4 == 4.0)) { 1.0 } else { 0.0 };
            noise_variable_111 = noise_metadata_schedule_120_e1748;
        }
        if matches!(source_index, 3) {
            let (noise_metadata_schedule_122_e1766,) = {
    if (noise_variable_111 != 0.0) {
        let noise_metadata_schedule_122_e1763: f64 = (params.p48 * noise_variable_75);
        let noise_metadata_schedule_122_e1764: f64 = (params.p47 + noise_metadata_schedule_122_e1763);
        (noise_metadata_schedule_122_e1764,)
    } else {
        (noise_variable_41,)
    }
};
            noise_variable_41 = noise_metadata_schedule_122_e1766;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_123_e1774,) = {
    if (noise_variable_111 != 0.0) {
        let noise_metadata_schedule_123_e1771: f64 = (params.p48 * noise_variable_75);
        let noise_metadata_schedule_123_e1772: f64 = (params.p50 + noise_metadata_schedule_123_e1771);
        (noise_metadata_schedule_123_e1772,)
    } else {
        (noise_variable_42,)
    }
};
            noise_variable_42 = noise_metadata_schedule_123_e1774;
        }
        if matches!(source_index, 3) {
            let (noise_metadata_schedule_125_e1794,) = {
    if (noise_variable_111 == 0.0) {
        let noise_metadata_schedule_125_e1791: f64 = (params.p48 * noise_variable_76);
        let noise_metadata_schedule_125_e1792: f64 = (params.p47 + noise_metadata_schedule_125_e1791);
        (noise_metadata_schedule_125_e1792,)
    } else {
        (noise_variable_41,)
    }
};
            noise_variable_41 = noise_metadata_schedule_125_e1794;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_126_e1803,) = {
    if (noise_variable_111 == 0.0) {
        let noise_metadata_schedule_126_e1800: f64 = (params.p48 * noise_variable_76);
        let noise_metadata_schedule_126_e1801: f64 = (params.p50 + noise_metadata_schedule_126_e1800);
        (noise_metadata_schedule_126_e1801,)
    } else {
        (noise_variable_42,)
    }
};
            noise_variable_42 = noise_metadata_schedule_126_e1803;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_127_e1808: f64 = (noise_variable_16).abs();
            let noise_metadata_schedule_127_e1809: f64 = (params.p76 * noise_metadata_schedule_127_e1808);
            let noise_metadata_schedule_127_e1810: f64 = (1.0 + noise_metadata_schedule_127_e1809);
            let noise_metadata_schedule_127_e1811: f64 = (noise_variable_42 * noise_metadata_schedule_127_e1810);
            noise_variable_50 = noise_metadata_schedule_127_e1811;
        }
        if matches!(source_index, 3) {
            let noise_metadata_schedule_128_e1816: f64 = (noise_variable_16).abs();
            let noise_metadata_schedule_128_e1817: f64 = (params.p76 * noise_metadata_schedule_128_e1816);
            let noise_metadata_schedule_128_e1818: f64 = (1.0 + noise_metadata_schedule_128_e1817);
            let noise_metadata_schedule_128_e1819: f64 = (noise_variable_41 * noise_metadata_schedule_128_e1818);
            noise_variable_49 = noise_metadata_schedule_128_e1819;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_130_e1830: f64 = if params.p5 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_112 = noise_metadata_schedule_130_e1830;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_131_e1841,) = {
    if (noise_variable_112 != 0.0) {
        let noise_metadata_schedule_131_e1834: f64 = (-1.0);
        let noise_metadata_schedule_131_e1836: f64 = (noise_metadata_schedule_131_e1834 * noise_variable_57);
        let noise_metadata_schedule_131_e1837: f64 = (noise_metadata_schedule_131_e1836).tanh();
        let noise_metadata_schedule_131_e1838: f64 = (noise_variable_19 * noise_metadata_schedule_131_e1837);
        let noise_metadata_schedule_131_e1839: f64 = { let limexp_arg = noise_metadata_schedule_131_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (noise_metadata_schedule_131_e1839,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_131_e1841;
        }
        if matches!(source_index, 12 | 14) {
            let (noise_metadata_schedule_132_e1849,) = {
    if (noise_variable_112 != 0.0) {
        let noise_metadata_schedule_132_e1846: f64 = (noise_variable_96 - noise_variable_57);
        let noise_metadata_schedule_132_e1847: f64 = noise_metadata_schedule_132_e1846;
        (noise_metadata_schedule_132_e1847,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_132_e1849;
        }
        if matches!(source_index, 12 | 14) {
            let (noise_metadata_schedule_133_e1856,) = {
    if (noise_variable_112 != 0.0) {
        let noise_metadata_schedule_133_e1852: f64 = (-noise_variable_96);
        let noise_metadata_schedule_133_e1854: f64 = (noise_metadata_schedule_133_e1852 - params.p83);
        (noise_metadata_schedule_133_e1854,)
    } else {
        (noise_variable_21,)
    }
};
            noise_variable_21 = noise_metadata_schedule_133_e1856;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            let (noise_metadata_schedule_134_e1864,) = {
    if (noise_variable_112 != 0.0) {
        let noise_metadata_schedule_134_e1861: f64 = (noise_variable_97 - noise_variable_57);
        let noise_metadata_schedule_134_e1862: f64 = noise_metadata_schedule_134_e1861;
        (noise_metadata_schedule_134_e1862,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_134_e1864;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            let (noise_metadata_schedule_135_e1871,) = {
    if (noise_variable_112 != 0.0) {
        let noise_metadata_schedule_135_e1867: f64 = (-noise_variable_97);
        let noise_metadata_schedule_135_e1869: f64 = (noise_metadata_schedule_135_e1867 - params.p84);
        (noise_metadata_schedule_135_e1869,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_135_e1871;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_136_e1880,) = {
    if (noise_variable_112 == 0.0) {
        let noise_metadata_schedule_136_e1875: f64 = (-noise_variable_19);
        let noise_metadata_schedule_136_e1877: f64 = (noise_metadata_schedule_136_e1875 * noise_variable_57);
        let noise_metadata_schedule_136_e1878: f64 = { let limexp_arg = noise_metadata_schedule_136_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (noise_metadata_schedule_136_e1878,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_136_e1880;
        }
        if matches!(source_index, 12 | 14) {
            let (noise_metadata_schedule_137_e1889,) = {
    if (noise_variable_112 == 0.0) {
        let noise_metadata_schedule_137_e1884: f64 = (-params.p85);
        let noise_metadata_schedule_137_e1886: f64 = (noise_metadata_schedule_137_e1884 * params.p83);
        let noise_metadata_schedule_137_e1887: f64 = { let limexp_arg = noise_metadata_schedule_137_e1886; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (noise_metadata_schedule_137_e1887,)
    } else {
        (noise_variable_24,)
    }
};
            noise_variable_24 = noise_metadata_schedule_137_e1889;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            let (noise_metadata_schedule_138_e1898,) = {
    if (noise_variable_112 == 0.0) {
        let noise_metadata_schedule_138_e1893: f64 = (-params.p85);
        let noise_metadata_schedule_138_e1895: f64 = (noise_metadata_schedule_138_e1893 * params.p84);
        let noise_metadata_schedule_138_e1896: f64 = { let limexp_arg = noise_metadata_schedule_138_e1895; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (noise_metadata_schedule_138_e1896,)
    } else {
        (noise_variable_25,)
    }
};
            noise_variable_25 = noise_metadata_schedule_138_e1898;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_139_e1901: f64 = if params.p5 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_113 = noise_metadata_schedule_139_e1901;
        }
        if matches!(source_index, 12 | 14) {
            let (noise_metadata_schedule_140_e1911,) = {
    if ((noise_variable_112 == 0.0) && (noise_variable_113 != 0.0)) {
        let noise_metadata_schedule_140_e1908: f64 = (noise_variable_96 - noise_variable_57);
        let noise_metadata_schedule_140_e1909: f64 = (noise_metadata_schedule_140_e1908).tanh();
        (noise_metadata_schedule_140_e1909,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_140_e1911;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            let (noise_metadata_schedule_141_e1921,) = {
    if ((noise_variable_112 == 0.0) && (noise_variable_113 != 0.0)) {
        let noise_metadata_schedule_141_e1918: f64 = (noise_variable_97 - noise_variable_57);
        let noise_metadata_schedule_141_e1919: f64 = (noise_metadata_schedule_141_e1918).tanh();
        (noise_metadata_schedule_141_e1919,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_141_e1921;
        }
        if matches!(source_index, 12 | 14) {
            let (noise_metadata_schedule_142_e1931,) = {
    if ((noise_variable_112 == 0.0) && (noise_variable_113 == 0.0)) {
        let noise_metadata_schedule_142_e1929: f64 = (noise_variable_96 - noise_variable_57);
        (noise_metadata_schedule_142_e1929,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_142_e1931;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            let (noise_metadata_schedule_143_e1941,) = {
    if ((noise_variable_112 == 0.0) && (noise_variable_113 == 0.0)) {
        let noise_metadata_schedule_143_e1939: f64 = (noise_variable_97 - noise_variable_57);
        (noise_metadata_schedule_143_e1939,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_143_e1941;
        }
        if matches!(source_index, 12 | 14) {
            let (noise_metadata_schedule_144_e1949,) = {
    if (noise_variable_112 == 0.0) {
        let noise_metadata_schedule_144_e1945: f64 = (-noise_variable_96);
        let noise_metadata_schedule_144_e1947: f64 = (noise_metadata_schedule_144_e1945 - params.p83);
        (noise_metadata_schedule_144_e1947,)
    } else {
        (noise_variable_21,)
    }
};
            noise_variable_21 = noise_metadata_schedule_144_e1949;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            let (noise_metadata_schedule_145_e1957,) = {
    if (noise_variable_112 == 0.0) {
        let noise_metadata_schedule_145_e1953: f64 = (-noise_variable_97);
        let noise_metadata_schedule_145_e1955: f64 = (noise_metadata_schedule_145_e1953 - params.p84);
        (noise_metadata_schedule_145_e1955,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_145_e1957;
        }
        if matches!(source_index, 12 | 14) {
            let noise_metadata_schedule_146_e1960: f64 = (params.p85 * noise_variable_21);
            let noise_metadata_schedule_146_e1961: f64 = { let limexp_arg = noise_metadata_schedule_146_e1960; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_146_e1963: f64 = (noise_metadata_schedule_146_e1961 - noise_variable_24);
            noise_variable_8 = noise_metadata_schedule_146_e1963;
        }
        if matches!(source_index, 12 | 14) {
            let noise_metadata_schedule_147_e1967: f64 = (noise_variable_19 * noise_variable_20);
            let noise_metadata_schedule_147_e1968: f64 = { let limexp_arg = noise_metadata_schedule_147_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_147_e1971: f64 = (0.001 * params.p82);
            let noise_metadata_schedule_147_e1973: f64 = (noise_metadata_schedule_147_e1971 * noise_variable_8);
            let noise_metadata_schedule_147_e1974: f64 = (noise_metadata_schedule_147_e1968 - noise_metadata_schedule_147_e1973);
            let noise_metadata_schedule_147_e1976: f64 = (noise_metadata_schedule_147_e1974 - noise_variable_63);
            let noise_metadata_schedule_147_e1977: f64 = (params.p42 * noise_metadata_schedule_147_e1976);
            noise_variable_7 = noise_metadata_schedule_147_e1977;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            let noise_metadata_schedule_148_e1980: f64 = (params.p85 * noise_variable_23);
            let noise_metadata_schedule_148_e1981: f64 = { let limexp_arg = noise_metadata_schedule_148_e1980; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_148_e1983: f64 = (noise_metadata_schedule_148_e1981 - noise_variable_25);
            noise_variable_10 = noise_metadata_schedule_148_e1983;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            let noise_metadata_schedule_149_e1987: f64 = (noise_variable_19 * noise_variable_22);
            let noise_metadata_schedule_149_e1988: f64 = { let limexp_arg = noise_metadata_schedule_149_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_149_e1991: f64 = (0.001 * params.p82);
            let noise_metadata_schedule_149_e1993: f64 = (noise_metadata_schedule_149_e1991 * noise_variable_10);
            let noise_metadata_schedule_149_e1994: f64 = (noise_metadata_schedule_149_e1988 - noise_metadata_schedule_149_e1993);
            let noise_metadata_schedule_149_e1996: f64 = (noise_metadata_schedule_149_e1994 - noise_variable_63);
            let noise_metadata_schedule_149_e1997: f64 = (params.p42 * noise_metadata_schedule_149_e1996);
            noise_variable_9 = noise_metadata_schedule_149_e1997;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let noise_metadata_schedule_218_e2869: f64 = 0.0;
            noise_variable_99 = noise_metadata_schedule_218_e2869;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let noise_metadata_schedule_219_e2874: f64 = (noise_variable_99 * params.p50);
            let noise_metadata_schedule_219_e2875: f64 = (1.0 + noise_metadata_schedule_219_e2874);
            let noise_metadata_schedule_219_e2876: f64 = (noise_variable_99 / noise_metadata_schedule_219_e2875);
            noise_variable_99 = noise_metadata_schedule_219_e2876;
        }
        if matches!(source_index, 4 | 5 | 6 | 7 | 8 | 9 | 10) {
            let noise_metadata_schedule_221_e2882: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_128 = noise_metadata_schedule_221_e2882;
        }
        if matches!(source_index, 6 | 7 | 8 | 9 | 10) {
            let noise_metadata_schedule_222_e2885: f64 = if params.p7 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_129 = noise_metadata_schedule_222_e2885;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_224_e2896,) = {
    if (noise_variable_128 != 0.0) {
        let noise_metadata_schedule_224_e2891: f64 = (noise_variable_18).abs();
        let noise_metadata_schedule_224_e2893: f64 = (noise_variable_9).abs();
        let noise_metadata_schedule_224_e2894: f64 = (noise_metadata_schedule_224_e2891 + noise_metadata_schedule_224_e2893);
        (noise_metadata_schedule_224_e2894,)
    } else {
        (noise_variable_132,)
    }
};
            noise_variable_132 = noise_metadata_schedule_224_e2896;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_225_e2917,) = {
    if (noise_variable_128 != 0.0) {
        let noise_metadata_schedule_225_e2900: f64 = (params.p93 + 273.15);
        let noise_metadata_schedule_225_e2904: f64 = (params.p95 * noise_variable_75);
        let noise_metadata_schedule_225_e2906: f64 = (noise_variable_79).abs();
        let noise_metadata_schedule_225_e2907: f64 = (noise_metadata_schedule_225_e2904 * noise_metadata_schedule_225_e2906);
        let noise_metadata_schedule_225_e2911: f64 = (params.p16 * noise_variable_5);
        let noise_metadata_schedule_225_e2912: f64 = (1.0 + noise_metadata_schedule_225_e2911);
        let noise_metadata_schedule_225_e2913: f64 = (noise_metadata_schedule_225_e2907 * noise_metadata_schedule_225_e2912);
        let noise_metadata_schedule_225_e2914: f64 = (1.0 + noise_metadata_schedule_225_e2913);
        let noise_metadata_schedule_225_e2915: f64 = (noise_metadata_schedule_225_e2900 * noise_metadata_schedule_225_e2914);
        (noise_metadata_schedule_225_e2915,)
    } else {
        (noise_variable_133,)
    }
};
            noise_variable_133 = noise_metadata_schedule_225_e2917;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_226_e2941,) = {
    if (noise_variable_128 != 0.0) {
        let noise_metadata_schedule_226_e2921: f64 = (params.p99 * 4.0);
        let noise_metadata_schedule_226_e2923: f64 = (noise_metadata_schedule_226_e2921 * 1.3806503e-23);
        let noise_metadata_schedule_226_e2925: f64 = (noise_metadata_schedule_226_e2923 * noise_variable_15);
        let noise_metadata_schedule_226_e2928: f64 = (noise_variable_133 / noise_variable_15);
        let noise_metadata_schedule_226_e2930: f64 = (noise_metadata_schedule_226_e2928 * noise_variable_132);
        let noise_metadata_schedule_226_e2933: f64 = (params.p94 * noise_variable_132);
        let noise_metadata_schedule_226_e2935: f64 = (noise_metadata_schedule_226_e2933 * noise_variable_132);
        let noise_metadata_schedule_226_e2936: f64 = (noise_metadata_schedule_226_e2930 + noise_metadata_schedule_226_e2935);
        let noise_metadata_schedule_226_e2937: f64 = (noise_metadata_schedule_226_e2936).abs();
        let noise_metadata_schedule_226_e2938: f64 = (noise_metadata_schedule_226_e2937).sqrt();
        let noise_metadata_schedule_226_e2939: f64 = (noise_metadata_schedule_226_e2925 * noise_metadata_schedule_226_e2938);
        (noise_metadata_schedule_226_e2939,)
    } else {
        (noise_variable_131,)
    }
};
            noise_variable_131 = noise_metadata_schedule_226_e2941;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_227_e2958,) = {
    if (((noise_variable_129 != 0.0) && (noise_variable_128 == 0.0)) && (params.p0 != 0.0)) {
        let noise_metadata_schedule_227_e2950: f64 = (4.0 * 1.3806503e-23);
        let noise_metadata_schedule_227_e2952: f64 = (noise_metadata_schedule_227_e2950 * noise_variable_15);
        let noise_metadata_schedule_227_e2954: f64 = (noise_metadata_schedule_227_e2952 * noise_variable_99);
        let noise_metadata_schedule_227_e2956: f64 = (noise_metadata_schedule_227_e2954 * params.p87);
        (noise_metadata_schedule_227_e2956,)
    } else {
        (noise_variable_134,)
    }
};
            noise_variable_134 = noise_metadata_schedule_227_e2958;
        }
        if matches!(source_index, 9) {
            let noise_metadata_schedule_228_e2961: f64 = if noise_variable_99 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_136 = noise_metadata_schedule_228_e2961;
        }
        if matches!(source_index, 9) {
            let (noise_metadata_schedule_229_e2984,) = {
    if ((((noise_variable_129 != 0.0) && (noise_variable_128 == 0.0)) && (params.p0 != 0.0)) && (noise_variable_136 != 0.0)) {
        let noise_metadata_schedule_229_e2972: f64 = (noise_variable_44 * noise_variable_44);
        let noise_metadata_schedule_229_e2974: f64 = (noise_metadata_schedule_229_e2972 * 4.0);
        let noise_metadata_schedule_229_e2976: f64 = (noise_metadata_schedule_229_e2974 * 1.3806503e-23);
        let noise_metadata_schedule_229_e2978: f64 = (noise_metadata_schedule_229_e2976 * noise_variable_15);
        let noise_metadata_schedule_229_e2980: f64 = (noise_metadata_schedule_229_e2978 * params.p86);
        let noise_metadata_schedule_229_e2982: f64 = (noise_metadata_schedule_229_e2980 / noise_variable_99);
        (noise_metadata_schedule_229_e2982,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_229_e2984;
        }
        if matches!(source_index, 9) {
            let (noise_metadata_schedule_230_e2996,) = {
    if ((((noise_variable_129 != 0.0) && (noise_variable_128 == 0.0)) && (params.p0 != 0.0)) && (noise_variable_136 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_230_e2996;
        }
        if matches!(source_index, 6 | 7) {
            let (noise_metadata_schedule_231_e3018,) = {
    if (((noise_variable_129 != 0.0) && (noise_variable_128 == 0.0)) && (params.p0 != 0.0)) {
        let noise_metadata_schedule_231_e3005: f64 = (4.0 * 1.3806503e-23);
        let noise_metadata_schedule_231_e3007: f64 = (noise_metadata_schedule_231_e3005 * noise_variable_15);
        let noise_metadata_schedule_231_e3009: f64 = (noise_metadata_schedule_231_e3007 * params.p88);
        let noise_metadata_schedule_231_e3011: f64 = (noise_metadata_schedule_231_e3009 * noise_variable_44);
        let noise_metadata_schedule_231_e3014: f64 = (params.p87 * params.p86);
        let noise_metadata_schedule_231_e3015: f64 = (noise_metadata_schedule_231_e3014).sqrt();
        let noise_metadata_schedule_231_e3016: f64 = (noise_metadata_schedule_231_e3011 * noise_metadata_schedule_231_e3015);
        (noise_metadata_schedule_231_e3016,)
    } else {
        (noise_variable_140,)
    }
};
            noise_variable_140 = noise_metadata_schedule_231_e3018;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_235_e3074,) = {
    if (((noise_variable_129 != 0.0) && (noise_variable_128 == 0.0)) && (params.p0 != 0.0)) {
        let noise_metadata_schedule_235_e3064: f64 = (4.0 * 1.3806503e-23);
        let noise_metadata_schedule_235_e3066: f64 = (noise_metadata_schedule_235_e3064 * noise_variable_15);
        let noise_metadata_schedule_235_e3068: f64 = (noise_metadata_schedule_235_e3066 * noise_variable_99);
        let noise_metadata_schedule_235_e3070: f64 = (noise_metadata_schedule_235_e3068 * params.p87);
        let noise_metadata_schedule_235_e3072: f64 = (noise_metadata_schedule_235_e3070 * params.p89);
        (noise_metadata_schedule_235_e3072,)
    } else {
        (noise_variable_141,)
    }
};
            noise_variable_141 = noise_metadata_schedule_235_e3074;
        }
        match source_index {
            0 => {
                let noise_0_psd_e3085: f64 = 1.0;
                let noise_0_psd_e222: f64 = (4.0 * 1.3806503e-23);
                let noise_0_psd_e224: f64 = (noise_0_psd_e222 * noise_variable_15);
                let noise_0_psd_e226: f64 = (noise_0_psd_e224 * params.p51);
                let noise_0_psd_e3086: f64 = (noise_0_psd_e3085 * noise_0_psd_e226);
                let psd = noise_0_psd_e3086;
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
                let noise_1_psd_e3088: f64 = 1.0;
                let noise_1_psd_e253: f64 = (4.0 * 1.3806503e-23);
                let noise_1_psd_e255: f64 = (noise_1_psd_e253 * noise_variable_15);
                let noise_1_psd_e257: f64 = (noise_1_psd_e255 * params.p46);
                let noise_1_psd_e3089: f64 = (noise_1_psd_e3088 * noise_1_psd_e257);
                let psd = noise_1_psd_e3089;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd / self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            2 => {
                let noise_2_psd_e3091: f64 = 1.0;
                let noise_2_psd_e282: f64 = (4.0 * 1.3806503e-23);
                let noise_2_psd_e284: f64 = (noise_2_psd_e282 * noise_variable_15);
                let noise_2_psd_e286: f64 = (noise_2_psd_e284 * noise_variable_50);
                let noise_2_psd_e3092: f64 = (noise_2_psd_e3091 * noise_2_psd_e286);
                let psd = noise_2_psd_e3092;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd / self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            3 => {
                let noise_3_psd_e3094: f64 = 1.0;
                let noise_3_psd_e311: f64 = (4.0 * 1.3806503e-23);
                let noise_3_psd_e313: f64 = (noise_3_psd_e311 * noise_variable_15);
                let noise_3_psd_e315: f64 = (noise_3_psd_e313 * noise_variable_49);
                let noise_3_psd_e3095: f64 = (noise_3_psd_e3094 * noise_3_psd_e315);
                let psd = noise_3_psd_e3095;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd / self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            4 => {
                let noise_4_psd_e3097: f64 = 1.0;
                let noise_4_psd_e3098: f64 = (noise_4_psd_e3097 * noise_variable_131);
                let psd = noise_4_psd_e3098;
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
                let noise_5_psd_e3100: f64 = 1.0;
                let noise_5_psd_e347: f64 = (noise_variable_131 * params.p96);
                let noise_5_psd_e3101: f64 = (noise_5_psd_e3100 * noise_5_psd_e347);
                let psd = noise_5_psd_e3101;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
                let exponent: Option<f64> = Some(params.p98);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            6 => {
                let noise_6_psd_e3103: f64 = 1.0;
                let noise_6_psd_e3104: f64 = (noise_6_psd_e3103 * noise_variable_140);
                let psd = noise_6_psd_e3104;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            7 => {
                let noise_7_psd_e3106: f64 = 1.0;
                let noise_7_psd_e3107: f64 = (noise_7_psd_e3106 * noise_variable_140);
                let psd = noise_7_psd_e3107;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 7, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            8 => {
                let noise_8_psd_e3109: f64 = 1.0;
                let noise_8_psd_e3110: f64 = (noise_8_psd_e3109 * noise_variable_134);
                let psd = noise_8_psd_e3110;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 8, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            9 => {
                let noise_9_psd_e3112: f64 = 1.0;
                let noise_9_psd_e3113: f64 = (noise_9_psd_e3112 * noise_variable_135);
                let psd = noise_9_psd_e3113;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
                let exponent: Option<f64> = Some(2.0);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            10 => {
                let noise_10_psd_e3115: f64 = 1.0;
                let noise_10_psd_e3116: f64 = (noise_10_psd_e3115 * noise_variable_141);
                let psd = noise_10_psd_e3116;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 10, value: psd }); }
                let exponent: Option<f64> = Some(1.0);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            11 => {
                let noise_11_psd_e3118: f64 = 1.0;
                let noise_11_psd_e476: f64 = (noise_variable_18).powf(params.p91);
                let noise_11_psd_e477: f64 = (params.p90 * noise_11_psd_e476);
                let noise_11_psd_e3119: f64 = (noise_11_psd_e3118 * noise_11_psd_e477);
                let psd = noise_11_psd_e3119;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
                let exponent: Option<f64> = Some(params.p92);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            12 => {
                let noise_12_psd_e3121: f64 = 1.0;
                let noise_12_psd_e488: f64 = (2.0 * 1.602176462e-19);
                let noise_12_psd_e490: f64 = (noise_variable_7).abs();
                let noise_12_psd_e491: f64 = (noise_12_psd_e488 * noise_12_psd_e490);
                let noise_12_psd_e3122: f64 = (noise_12_psd_e3121 * noise_12_psd_e491);
                let psd = noise_12_psd_e3122;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 12, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            13 => {
                let noise_13_psd_e3124: f64 = 1.0;
                let noise_13_psd_e499: f64 = (2.0 * 1.602176462e-19);
                let noise_13_psd_e501: f64 = (noise_variable_9).abs();
                let noise_13_psd_e502: f64 = (noise_13_psd_e499 * noise_13_psd_e501);
                let noise_13_psd_e3125: f64 = (noise_13_psd_e3124 * noise_13_psd_e502);
                let psd = noise_13_psd_e3125;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 13, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            14 => {
                let noise_14_psd_e3127: f64 = 1.0;
                let noise_14_psd_e512: f64 = (noise_variable_7).abs();
                let noise_14_psd_e514: f64 = (noise_14_psd_e512).powf(params.p91);
                let noise_14_psd_e515: f64 = (params.p90 * noise_14_psd_e514);
                let noise_14_psd_e3128: f64 = (noise_14_psd_e3127 * noise_14_psd_e515);
                let psd = noise_14_psd_e3128;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
                let exponent: Option<f64> = Some(params.p92);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            15 => {
                let noise_15_psd_e3130: f64 = 1.0;
                let noise_15_psd_e526: f64 = (noise_variable_9).abs();
                let noise_15_psd_e528: f64 = (noise_15_psd_e526).powf(params.p91);
                let noise_15_psd_e529: f64 = (params.p90 * noise_15_psd_e528);
                let noise_15_psd_e3131: f64 = (noise_15_psd_e3130 * noise_15_psd_e529);
                let psd = noise_15_psd_e3131;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
                let exponent: Option<f64> = Some(params.p92);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            _ => unreachable!("noise source index was range checked"),
        }
    }
}

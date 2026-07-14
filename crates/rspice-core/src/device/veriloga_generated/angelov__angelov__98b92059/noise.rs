#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 16] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_GDI_RGD", label: Some("Rgd"), kind: GeneratedNoiseKind::White, equation: 20, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "gdi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GI_RG", label: Some("Rg"), kind: GeneratedNoiseKind::White, equation: 26, is_current: false, branch_ordinal: Some(7), pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_SII_RS", label: Some("Rs"), kind: GeneratedNoiseKind::White, equation: 30, is_current: false, branch_ordinal: Some(11), pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "sii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_D_RD", label: Some("Rd"), kind: GeneratedNoiseKind::White, equation: 35, is_current: false, branch_ordinal: Some(16), pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_IDS_NOISE", label: Some("Ids noise"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_IDS_FLICKER", label: Some("Ids flicker"), kind: GeneratedNoiseKind::Flicker, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IA_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "ia", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IB_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(15), name: "ib", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_DRAIN", label: Some("drain"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GI_SI_GATE", label: Some("gate"), kind: GeneratedNoiseKind::Flicker, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GSI_SI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GDI_DI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GSI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GDI_DI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
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
        if matches!(source_index, 0) {
            let noise_activation_schedule_161_e2034: f64 = if params.p47 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_100 = noise_activation_schedule_161_e2034;
        }
        if matches!(source_index, 1) {
            let noise_activation_schedule_163_e2040: f64 = if params.p42 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_102 = noise_activation_schedule_163_e2040;
        }
        if matches!(source_index, 2) {
            let noise_activation_schedule_165_e2046: f64 = if params.p46 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_104 = noise_activation_schedule_165_e2046;
        }
        if matches!(source_index, 3) {
            let noise_activation_schedule_166_e2053: f64 = if ((params.p43 > 0.0) || (params.p44 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_105 = noise_activation_schedule_166_e2053;
        }
        if matches!(source_index, 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11) {
            let noise_activation_schedule_168_e2059: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_107 = noise_activation_schedule_168_e2059;
        }
        if matches!(source_index, 6 | 7 | 8 | 9 | 10 | 11) {
            let noise_activation_schedule_169_e2062: f64 = if params.p7 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_108 = noise_activation_schedule_169_e2062;
        }
        if matches!(source_index, 11) {
            let noise_activation_schedule_184_e2265: f64 = if params.p75 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_122 = noise_activation_schedule_184_e2265;
        }
        if matches!(source_index, 14 | 15) {
            let noise_activation_schedule_185_e2268: f64 = if params.p75 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_123 = noise_activation_schedule_185_e2268;
        }
        let noise_source_active = match source_index {
            0 => {
                let noise_0_activation_e178: f64 = if ((noise_variable_100 != 0.0) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_0_activation_e178 != 0.0
            }
            1 => {
                let noise_1_activation_e221: f64 = if ((noise_variable_102 != 0.0) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_1_activation_e221 != 0.0
            }
            2 => {
                let noise_2_activation_e259: f64 = if ((noise_variable_104 != 0.0) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_2_activation_e259 != 0.0
            }
            3 => {
                let noise_3_activation_e295: f64 = if ((noise_variable_105 != 0.0) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_3_activation_e295 != 0.0
            }
            4 => {
                let noise_4_activation_e327: f64 = if ((noise_variable_107 != 0.0) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_4_activation_e327 != 0.0
            }
            5 => {
                let noise_5_activation_e335: f64 = if ((noise_variable_107 != 0.0) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_5_activation_e335 != 0.0
            }
            6 => {
                let noise_6_activation_e349: f64 = if (((noise_variable_108 != 0.0) && (noise_variable_107 == 0.0)) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_6_activation_e349 != 0.0
            }
            7 => {
                let noise_7_activation_e369: f64 = if (((noise_variable_108 != 0.0) && (noise_variable_107 == 0.0)) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_7_activation_e369 != 0.0
            }
            8 => {
                let noise_8_activation_e426: f64 = if (((noise_variable_108 != 0.0) && (noise_variable_107 == 0.0)) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_8_activation_e426 != 0.0
            }
            9 => {
                let noise_9_activation_e437: f64 = if (((noise_variable_108 != 0.0) && (noise_variable_107 == 0.0)) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_9_activation_e437 != 0.0
            }
            10 => {
                let noise_10_activation_e449: f64 = if (((noise_variable_108 != 0.0) && (noise_variable_107 == 0.0)) && (params.p0 != 0.0)) { 1.0 } else { 0.0 };
                noise_10_activation_e449 != 0.0
            }
            11 => {
                let noise_11_activation_e463: f64 = if ((((noise_variable_108 != 0.0) && (noise_variable_107 == 0.0)) && (params.p0 != 0.0)) && (noise_variable_122 != 0.0)) { 1.0 } else { 0.0 };
                noise_11_activation_e463 != 0.0
            }
            12 => {
                params.p0 != 0.0
            }
            13 => {
                params.p0 != 0.0
            }
            14 => {
                let noise_14_activation_e500: f64 = if ((params.p0 != 0.0) && (noise_variable_123 != 0.0)) { 1.0 } else { 0.0 };
                noise_14_activation_e500 != 0.0
            }
            15 => {
                let noise_15_activation_e514: f64 = if ((params.p0 != 0.0) && (noise_variable_123 != 0.0)) { 1.0 } else { 0.0 };
                noise_15_activation_e514 != 0.0
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
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            noise_variable_4 = (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[5]));
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            noise_variable_3 = (ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[3]));
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_2_e564: f64 = (-noise_variable_3);
            noise_variable_6 = noise_metadata_schedule_2_e564;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            noise_variable_5 = (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[5]));
        }
        if matches!(source_index, 12 | 14) {
            noise_variable_79 = noise_variable_4;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            noise_variable_80 = (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[3]));
        }
        if matches!(source_index, 4 | 5 | 11) {
            noise_variable_14 = (ctx.node_voltage(self.nodes[13]) - 0.0);
        }
        if matches!(source_index, 8 | 9 | 10) {
            noise_variable_81 = 0.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_12_e575: f64 = if self.param_given[3] { 1.0 } else { 0.0 };
            noise_variable_82 = noise_metadata_schedule_12_e575;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_13_e581,) = {
    if (noise_variable_82 != 0.0) {
        let noise_metadata_schedule_13_e579: f64 = (params.p3 + 273.15);
        (noise_metadata_schedule_13_e579,)
    } else {
        (noise_variable_11,)
    }
};
            noise_variable_11 = noise_metadata_schedule_13_e581;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_14_e588,) = {
    if (noise_variable_82 == 0.0) {
        let noise_metadata_schedule_14_e584: f64 = ctx.temperature();
        let noise_metadata_schedule_14_e586: f64 = (noise_metadata_schedule_14_e584 + params.p2);
        (noise_metadata_schedule_14_e586,)
    } else {
        (noise_variable_11,)
    }
};
            noise_variable_11 = noise_metadata_schedule_14_e588;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_15_e590: f64 = if self.param_given[85] { 1.0 } else { 0.0 };
            noise_variable_83 = noise_metadata_schedule_15_e590;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_16_e596,) = {
    if (noise_variable_83 != 0.0) {
        let noise_metadata_schedule_16_e594: f64 = (params.p85 + 273.15);
        (noise_metadata_schedule_16_e594,)
    } else {
        (noise_variable_10,)
    }
};
            noise_variable_10 = noise_metadata_schedule_16_e596;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_17_e603,) = {
    if (noise_variable_83 == 0.0) {
        let noise_metadata_schedule_17_e601: f64 = (27.0 + 273.15);
        (noise_metadata_schedule_17_e601,)
    } else {
        (noise_variable_10,)
    }
};
            noise_variable_10 = noise_metadata_schedule_17_e603;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_18_e610,) = {
    if (params.p1 != 0.0) {
        let noise_metadata_schedule_18_e607: f64 = ((ctx.node_voltage(self.nodes[11]) - 0.0)).abs();
        let noise_metadata_schedule_18_e608: f64 = (noise_variable_11 + noise_metadata_schedule_18_e607);
        (noise_metadata_schedule_18_e608,)
    } else {
        (noise_variable_11,)
    }
};
            noise_variable_11 = noise_metadata_schedule_18_e610;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_19_e612: f64 = (noise_variable_11 * THERMAL_VOLTAGE_PER_K);
            noise_variable_9 = noise_metadata_schedule_19_e612;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_20_e615: f64 = (noise_variable_11 - noise_variable_10);
            let noise_metadata_schedule_20_e616: f64 = (noise_metadata_schedule_20_e615).abs();
            noise_variable_12 = noise_metadata_schedule_20_e616;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_21_e623: f64 = if ((noise_variable_12 > 0.0) || (params.p57 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_84 = noise_metadata_schedule_21_e623;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_23_e643,) = {
    if (noise_variable_84 != 0.0) {
        let noise_metadata_schedule_23_e639: f64 = (params.p59 * noise_variable_12);
        let noise_metadata_schedule_23_e640: f64 = (1.0 + noise_metadata_schedule_23_e639);
        let noise_metadata_schedule_23_e641: f64 = (params.p8 * noise_metadata_schedule_23_e640);
        (noise_metadata_schedule_23_e641,)
    } else {
        (noise_variable_26,)
    }
};
            noise_variable_26 = noise_metadata_schedule_23_e643;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let (noise_metadata_schedule_24_e653,) = {
    if (noise_variable_84 != 0.0) {
        let noise_metadata_schedule_24_e649: f64 = (params.p60 * noise_variable_12);
        let noise_metadata_schedule_24_e650: f64 = (1.0 + noise_metadata_schedule_24_e649);
        let noise_metadata_schedule_24_e651: f64 = (params.p11 * noise_metadata_schedule_24_e650);
        (noise_metadata_schedule_24_e651,)
    } else {
        (noise_variable_45,)
    }
};
            noise_variable_45 = noise_metadata_schedule_24_e653;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_25_e663,) = {
    if (noise_variable_84 != 0.0) {
        let noise_metadata_schedule_25_e659: f64 = (params.p63 * noise_variable_12);
        let noise_metadata_schedule_25_e660: f64 = (1.0 + noise_metadata_schedule_25_e659);
        let noise_metadata_schedule_25_e661: f64 = (params.p20 * noise_metadata_schedule_25_e660);
        (noise_metadata_schedule_25_e661,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_25_e663;
        }
        if matches!(source_index, 6 | 7 | 9) {
            let (noise_metadata_schedule_26_e673,) = {
    if (noise_variable_84 != 0.0) {
        let noise_metadata_schedule_26_e669: f64 = (params.p61 * noise_variable_12);
        let noise_metadata_schedule_26_e670: f64 = (1.0 + noise_metadata_schedule_26_e669);
        let noise_metadata_schedule_26_e671: f64 = (params.p25 * noise_metadata_schedule_26_e670);
        (noise_metadata_schedule_26_e671,)
    } else {
        (noise_variable_31,)
    }
};
            noise_variable_31 = noise_metadata_schedule_26_e673;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_30_e711,) = {
    if (noise_variable_84 != 0.0) {
        let noise_metadata_schedule_30_e708: f64 = (params.p68 * noise_variable_12);
        let noise_metadata_schedule_30_e709: f64 = (params.p9 + noise_metadata_schedule_30_e708);
        (noise_metadata_schedule_30_e709,)
    } else {
        (noise_variable_39,)
    }
};
            noise_variable_39 = noise_metadata_schedule_30_e711;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_33_e739,) = {
    if (noise_variable_84 != 0.0) {
        let noise_metadata_schedule_33_e736: f64 = (params.p69 * noise_variable_12);
        let noise_metadata_schedule_33_e737: f64 = (params.p41 + noise_metadata_schedule_33_e736);
        (noise_metadata_schedule_33_e737,)
    } else {
        (noise_variable_42,)
    }
};
            noise_variable_42 = noise_metadata_schedule_33_e739;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_34_e747,) = {
    if (noise_variable_84 != 0.0) {
        let noise_metadata_schedule_34_e744: f64 = (params.p70 * noise_variable_12);
        let noise_metadata_schedule_34_e745: f64 = (params.p21 + noise_metadata_schedule_34_e744);
        (noise_metadata_schedule_34_e745,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_34_e747;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_35_e752,) = {
    if (noise_variable_84 == 0.0) {
        (params.p8,)
    } else {
        (noise_variable_26,)
    }
};
            noise_variable_26 = noise_metadata_schedule_35_e752;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let (noise_metadata_schedule_36_e757,) = {
    if (noise_variable_84 == 0.0) {
        (params.p11,)
    } else {
        (noise_variable_45,)
    }
};
            noise_variable_45 = noise_metadata_schedule_36_e757;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_37_e762,) = {
    if (noise_variable_84 == 0.0) {
        (params.p20,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_37_e762;
        }
        if matches!(source_index, 6 | 7 | 9) {
            let (noise_metadata_schedule_38_e767,) = {
    if (noise_variable_84 == 0.0) {
        (params.p25,)
    } else {
        (noise_variable_31,)
    }
};
            noise_variable_31 = noise_metadata_schedule_38_e767;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_42_e787,) = {
    if (noise_variable_84 == 0.0) {
        (params.p9,)
    } else {
        (noise_variable_39,)
    }
};
            noise_variable_39 = noise_metadata_schedule_42_e787;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_45_e802,) = {
    if (noise_variable_84 == 0.0) {
        (params.p41,)
    } else {
        (noise_variable_42,)
    }
};
            noise_variable_42 = noise_metadata_schedule_45_e802;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_46_e807,) = {
    if (noise_variable_84 == 0.0) {
        (params.p21,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_46_e807;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_47_e813: f64 = if ((!self.param_given[39]) && self.param_given[40]) { 1.0 } else { 0.0 };
            noise_variable_85 = noise_metadata_schedule_47_e813;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_48_e821,) = {
    if (noise_variable_85 != 0.0) {
        let noise_metadata_schedule_48_e817: f64 = (0.5 / params.p40);
        let noise_metadata_schedule_48_e819: f64 = (noise_metadata_schedule_48_e817 / noise_variable_9);
        (noise_metadata_schedule_48_e819,)
    } else {
        (noise_variable_15,)
    }
};
            noise_variable_15 = noise_metadata_schedule_48_e821;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_49_e826,) = {
    if (noise_variable_85 == 0.0) {
        (params.p39,)
    } else {
        (noise_variable_15,)
    }
};
            noise_variable_15 = noise_metadata_schedule_49_e826;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_50_e829: f64 = (params.p19 * noise_variable_5);
            let noise_metadata_schedule_50_e830: f64 = (noise_metadata_schedule_50_e829).cosh();
            noise_variable_47 = noise_metadata_schedule_50_e830;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_51_e836: f64 = (noise_variable_47 * noise_variable_47);
            let noise_metadata_schedule_51_e837: f64 = (params.p18 / noise_metadata_schedule_51_e836);
            let noise_metadata_schedule_51_e838: f64 = (1.0 + noise_metadata_schedule_51_e837);
            let noise_metadata_schedule_51_e839: f64 = (noise_variable_45 * noise_metadata_schedule_51_e838);
            noise_variable_44 = noise_metadata_schedule_51_e839;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_52_e842: f64 = (noise_variable_39 - params.p10);
            let noise_metadata_schedule_52_e846: f64 = (params.p15 * noise_variable_5);
            let noise_metadata_schedule_52_e847: f64 = (noise_metadata_schedule_52_e846).tanh();
            let noise_metadata_schedule_52_e848: f64 = (params.p10 * noise_metadata_schedule_52_e847);
            let noise_metadata_schedule_52_e849: f64 = (noise_metadata_schedule_52_e842 + noise_metadata_schedule_52_e848);
            let noise_metadata_schedule_52_e853: f64 = (noise_variable_6 - params.p21);
            let noise_metadata_schedule_52_e854: f64 = (params.p22 * noise_metadata_schedule_52_e853);
            let noise_metadata_schedule_52_e857: f64 = (noise_variable_6 - noise_variable_38);
            let noise_metadata_schedule_52_e858: f64 = (noise_metadata_schedule_52_e854 * noise_metadata_schedule_52_e857);
            let noise_metadata_schedule_52_e859: f64 = (noise_metadata_schedule_52_e849 - noise_metadata_schedule_52_e858);
            noise_variable_46 = noise_metadata_schedule_52_e859;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_53_e862: f64 = (noise_variable_4 - noise_variable_46);
            noise_variable_48 = noise_metadata_schedule_53_e862;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_54_e865: f64 = (noise_variable_48 * noise_variable_48);
            noise_variable_49 = noise_metadata_schedule_54_e865;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_55_e868: f64 = (noise_variable_44 * noise_variable_48);
            let noise_metadata_schedule_55_e871: f64 = (params.p12 * noise_variable_49);
            let noise_metadata_schedule_55_e872: f64 = (noise_metadata_schedule_55_e868 + noise_metadata_schedule_55_e871);
            let noise_metadata_schedule_55_e875: f64 = (params.p13 * noise_variable_48);
            let noise_metadata_schedule_55_e877: f64 = (noise_metadata_schedule_55_e875 * noise_variable_49);
            let noise_metadata_schedule_55_e878: f64 = (noise_metadata_schedule_55_e872 + noise_metadata_schedule_55_e877);
            noise_variable_13 = noise_metadata_schedule_55_e878;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_56_e881: f64 = (noise_variable_13).tanh();
            let noise_metadata_schedule_56_e882: f64 = (1.0 + noise_metadata_schedule_56_e881);
            noise_variable_59 = noise_metadata_schedule_56_e882;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let noise_metadata_schedule_57_e886: f64 = { let limexp_arg = noise_variable_13; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_57_e888: f64 = (-noise_variable_13);
            let noise_metadata_schedule_57_e889: f64 = { let limexp_arg = noise_metadata_schedule_57_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_57_e890: f64 = (noise_metadata_schedule_57_e886 - noise_metadata_schedule_57_e889);
            let noise_metadata_schedule_57_e891: f64 = (0.5 * noise_metadata_schedule_57_e890);
            let noise_metadata_schedule_57_e892: f64 = (noise_metadata_schedule_57_e891).tanh();
            let noise_metadata_schedule_57_e893: f64 = (1.0 + noise_metadata_schedule_57_e892);
            noise_variable_60 = noise_metadata_schedule_57_e893;
        }
        if matches!(source_index, 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_58_e897: f64 = (params.p15 * noise_variable_59);
            let noise_metadata_schedule_58_e898: f64 = (params.p14 + noise_metadata_schedule_58_e897);
            noise_variable_0 = noise_metadata_schedule_58_e898;
        }
        if matches!(source_index, 4 | 5 | 8 | 9 | 10) {
            let noise_metadata_schedule_59_e901: f64 = (noise_variable_0 * noise_variable_5);
            let noise_metadata_schedule_59_e902: f64 = (noise_metadata_schedule_59_e901).tanh();
            noise_variable_63 = noise_metadata_schedule_59_e902;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_60_e905: f64 = if params.p4 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_86 = noise_metadata_schedule_60_e905;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_61_e908: f64 = if params.p4 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_87 = noise_metadata_schedule_61_e908;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_62_e911: f64 = if params.p4 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_88 = noise_metadata_schedule_62_e911;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_63_e914: f64 = if params.p4 == 3.0 { 1.0 } else { 0.0 };
            noise_variable_89 = noise_metadata_schedule_63_e914;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_64_e935,) = {
    if (noise_variable_86 != 0.0) {
        let noise_metadata_schedule_64_e918: f64 = (noise_variable_26 * noise_variable_59);
        let noise_metadata_schedule_64_e920: f64 = (noise_metadata_schedule_64_e918 * noise_variable_63);
        let noise_metadata_schedule_64_e924: f64 = (params.p16 * noise_variable_5);
        let noise_metadata_schedule_64_e925: f64 = (1.0 + noise_metadata_schedule_64_e924);
        let noise_metadata_schedule_64_e929: f64 = (noise_variable_6 - noise_variable_38);
        let noise_metadata_schedule_64_e930: f64 = { let limexp_arg = noise_metadata_schedule_64_e929; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_64_e931: f64 = (noise_variable_30 * noise_metadata_schedule_64_e930);
        let noise_metadata_schedule_64_e932: f64 = (noise_metadata_schedule_64_e925 + noise_metadata_schedule_64_e931);
        let noise_metadata_schedule_64_e933: f64 = (noise_metadata_schedule_64_e920 * noise_metadata_schedule_64_e932);
        (noise_metadata_schedule_64_e933,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_64_e935;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_65_e944,) = {
    if ((noise_variable_87 != 0.0) && (noise_variable_86 == 0.0)) {
        let noise_metadata_schedule_65_e942: f64 = (noise_variable_3 - noise_variable_46);
        (noise_metadata_schedule_65_e942,)
    } else {
        (noise_variable_47,)
    }
};
            noise_variable_47 = noise_metadata_schedule_65_e944;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_66_e953,) = {
    if ((noise_variable_87 != 0.0) && (noise_variable_86 == 0.0)) {
        let noise_metadata_schedule_66_e951: f64 = (noise_variable_47 * noise_variable_47);
        (noise_metadata_schedule_66_e951,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_66_e953;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_67_e962,) = {
    if ((noise_variable_87 != 0.0) && (noise_variable_86 == 0.0)) {
        let noise_metadata_schedule_67_e960: f64 = (noise_variable_48 * noise_variable_47);
        (noise_metadata_schedule_67_e960,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_67_e962;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_68_e979,) = {
    if ((noise_variable_87 != 0.0) && (noise_variable_86 == 0.0)) {
        let noise_metadata_schedule_68_e969: f64 = (noise_variable_44 * noise_variable_47);
        let noise_metadata_schedule_68_e972: f64 = (params.p12 * noise_variable_48);
        let noise_metadata_schedule_68_e973: f64 = (noise_metadata_schedule_68_e969 + noise_metadata_schedule_68_e972);
        let noise_metadata_schedule_68_e976: f64 = (params.p13 * noise_variable_49);
        let noise_metadata_schedule_68_e977: f64 = (noise_metadata_schedule_68_e973 + noise_metadata_schedule_68_e976);
        (noise_metadata_schedule_68_e977,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_68_e979;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_69_e989,) = {
    if ((noise_variable_87 != 0.0) && (noise_variable_86 == 0.0)) {
        let noise_metadata_schedule_69_e986: f64 = (noise_variable_55).tanh();
        let noise_metadata_schedule_69_e987: f64 = (1.0 + noise_metadata_schedule_69_e986);
        (noise_metadata_schedule_69_e987,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_69_e989;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_70_e1000,) = {
    if ((noise_variable_87 != 0.0) && (noise_variable_86 == 0.0)) {
        let noise_metadata_schedule_70_e997: f64 = (params.p15 * noise_variable_61);
        let noise_metadata_schedule_70_e998: f64 = (params.p14 + noise_metadata_schedule_70_e997);
        (noise_metadata_schedule_70_e998,)
    } else {
        (noise_variable_56,)
    }
};
            noise_variable_56 = noise_metadata_schedule_70_e1000;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_71_e1011,) = {
    if ((noise_variable_87 != 0.0) && (noise_variable_86 == 0.0)) {
        let noise_metadata_schedule_71_e1008: f64 = (params.p17 * noise_variable_59);
        let noise_metadata_schedule_71_e1009: f64 = (params.p16 + noise_metadata_schedule_71_e1008);
        (noise_metadata_schedule_71_e1009,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_71_e1011;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_72_e1037,) = {
    if ((noise_variable_87 != 0.0) && (noise_variable_86 == 0.0)) {
        let noise_metadata_schedule_72_e1018: f64 = (noise_variable_26 * noise_variable_59);
        let noise_metadata_schedule_72_e1021: f64 = (1.0 + noise_variable_63);
        let noise_metadata_schedule_72_e1022: f64 = (noise_metadata_schedule_72_e1018 * noise_metadata_schedule_72_e1021);
        let noise_metadata_schedule_72_e1026: f64 = (noise_variable_53 * noise_variable_5);
        let noise_metadata_schedule_72_e1027: f64 = (1.0 + noise_metadata_schedule_72_e1026);
        let noise_metadata_schedule_72_e1031: f64 = (noise_variable_5 - noise_variable_38);
        let noise_metadata_schedule_72_e1032: f64 = { let limexp_arg = noise_metadata_schedule_72_e1031; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_72_e1033: f64 = (noise_variable_30 * noise_metadata_schedule_72_e1032);
        let noise_metadata_schedule_72_e1034: f64 = (noise_metadata_schedule_72_e1027 + noise_metadata_schedule_72_e1033);
        let noise_metadata_schedule_72_e1035: f64 = (noise_metadata_schedule_72_e1022 * noise_metadata_schedule_72_e1034);
        (noise_metadata_schedule_72_e1035,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_72_e1037;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_73_e1048,) = {
    if ((noise_variable_87 != 0.0) && (noise_variable_86 == 0.0)) {
        let noise_metadata_schedule_73_e1045: f64 = (params.p17 * noise_variable_61);
        let noise_metadata_schedule_73_e1046: f64 = (params.p16 + noise_metadata_schedule_73_e1045);
        (noise_metadata_schedule_73_e1046,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_73_e1048;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_74_e1058,) = {
    if ((noise_variable_87 != 0.0) && (noise_variable_86 == 0.0)) {
        let noise_metadata_schedule_74_e1055: f64 = (noise_variable_56 * noise_variable_5);
        let noise_metadata_schedule_74_e1056: f64 = (noise_metadata_schedule_74_e1055).tanh();
        (noise_metadata_schedule_74_e1056,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_74_e1058;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_75_e1077,) = {
    if ((noise_variable_87 != 0.0) && (noise_variable_86 == 0.0)) {
        let noise_metadata_schedule_75_e1065: f64 = (noise_variable_26 * noise_variable_61);
        let noise_metadata_schedule_75_e1068: f64 = (1.0 - noise_variable_64);
        let noise_metadata_schedule_75_e1069: f64 = (noise_metadata_schedule_75_e1065 * noise_metadata_schedule_75_e1068);
        let noise_metadata_schedule_75_e1073: f64 = (noise_variable_51 * noise_variable_5);
        let noise_metadata_schedule_75_e1074: f64 = (1.0 - noise_metadata_schedule_75_e1073);
        let noise_metadata_schedule_75_e1075: f64 = (noise_metadata_schedule_75_e1069 * noise_metadata_schedule_75_e1074);
        (noise_metadata_schedule_75_e1075,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_75_e1077;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_76_e1088,) = {
    if ((noise_variable_87 != 0.0) && (noise_variable_86 == 0.0)) {
        let noise_metadata_schedule_76_e1085: f64 = (noise_variable_57 - noise_variable_58);
        let noise_metadata_schedule_76_e1086: f64 = (0.5 * noise_metadata_schedule_76_e1085);
        (noise_metadata_schedule_76_e1086,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_76_e1088;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_77_e1099,) = {
    if ((noise_variable_88 != 0.0) && (!((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)))) {
        let noise_metadata_schedule_77_e1097: f64 = (noise_variable_4 - noise_variable_46);
        (noise_metadata_schedule_77_e1097,)
    } else {
        (noise_variable_47,)
    }
};
            noise_variable_47 = noise_metadata_schedule_77_e1099;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_78_e1110,) = {
    if ((noise_variable_88 != 0.0) && (!((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)))) {
        let noise_metadata_schedule_78_e1108: f64 = (noise_variable_47 * noise_variable_47);
        (noise_metadata_schedule_78_e1108,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_78_e1110;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_79_e1131,) = {
    if ((noise_variable_88 != 0.0) && (!((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)))) {
        let noise_metadata_schedule_79_e1121: f64 = (params.p12 * noise_variable_48);
        let noise_metadata_schedule_79_e1122: f64 = (noise_variable_47 + noise_metadata_schedule_79_e1121);
        let noise_metadata_schedule_79_e1125: f64 = (params.p13 * noise_variable_48);
        let noise_metadata_schedule_79_e1127: f64 = (noise_metadata_schedule_79_e1125 * noise_variable_47);
        let noise_metadata_schedule_79_e1128: f64 = (noise_metadata_schedule_79_e1122 + noise_metadata_schedule_79_e1127);
        let noise_metadata_schedule_79_e1129: f64 = (noise_variable_44 * noise_metadata_schedule_79_e1128);
        (noise_metadata_schedule_79_e1129,)
    } else {
        (noise_variable_13,)
    }
};
            noise_variable_13 = noise_metadata_schedule_79_e1131;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_80_e1150,) = {
    if ((noise_variable_88 != 0.0) && (!((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)))) {
        let noise_metadata_schedule_80_e1141: f64 = { let limexp_arg = noise_variable_13; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_80_e1143: f64 = (-noise_variable_13);
        let noise_metadata_schedule_80_e1144: f64 = { let limexp_arg = noise_metadata_schedule_80_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_80_e1145: f64 = (noise_metadata_schedule_80_e1141 - noise_metadata_schedule_80_e1144);
        let noise_metadata_schedule_80_e1146: f64 = (0.5 * noise_metadata_schedule_80_e1145);
        let noise_metadata_schedule_80_e1147: f64 = (noise_metadata_schedule_80_e1146).tanh();
        let noise_metadata_schedule_80_e1148: f64 = (1.0 + noise_metadata_schedule_80_e1147);
        (noise_metadata_schedule_80_e1148,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_80_e1150;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_81_e1163,) = {
    if ((noise_variable_88 != 0.0) && (!((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)))) {
        let noise_metadata_schedule_81_e1160: f64 = (params.p15 * noise_variable_60);
        let noise_metadata_schedule_81_e1161: f64 = (params.p14 + noise_metadata_schedule_81_e1160);
        (noise_metadata_schedule_81_e1161,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_81_e1163;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_82_e1175,) = {
    if ((noise_variable_88 != 0.0) && (!((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)))) {
        let noise_metadata_schedule_82_e1172: f64 = (noise_variable_1 * noise_variable_5);
        let noise_metadata_schedule_82_e1173: f64 = (noise_metadata_schedule_82_e1172).tanh();
        (noise_metadata_schedule_82_e1173,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_82_e1175;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_83_e1188,) = {
    if ((noise_variable_88 != 0.0) && (!((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)))) {
        let noise_metadata_schedule_83_e1185: f64 = (params.p17 * noise_variable_60);
        let noise_metadata_schedule_83_e1186: f64 = (params.p16 + noise_metadata_schedule_83_e1185);
        (noise_metadata_schedule_83_e1186,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_83_e1188;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_84_e1214,) = {
    if ((noise_variable_88 != 0.0) && (!((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)))) {
        let noise_metadata_schedule_84_e1197: f64 = (noise_variable_26 * noise_variable_60);
        let noise_metadata_schedule_84_e1199: f64 = (noise_metadata_schedule_84_e1197 * noise_variable_65);
        let noise_metadata_schedule_84_e1203: f64 = (noise_variable_53 * noise_variable_5);
        let noise_metadata_schedule_84_e1204: f64 = (1.0 + noise_metadata_schedule_84_e1203);
        let noise_metadata_schedule_84_e1208: f64 = (noise_variable_6 - noise_variable_38);
        let noise_metadata_schedule_84_e1209: f64 = { let limexp_arg = noise_metadata_schedule_84_e1208; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_84_e1210: f64 = (noise_variable_30 * noise_metadata_schedule_84_e1209);
        let noise_metadata_schedule_84_e1211: f64 = (noise_metadata_schedule_84_e1204 + noise_metadata_schedule_84_e1210);
        let noise_metadata_schedule_84_e1212: f64 = (noise_metadata_schedule_84_e1199 * noise_metadata_schedule_84_e1211);
        (noise_metadata_schedule_84_e1212,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_84_e1214;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 8 | 9 | 10 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_85_e1227,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_85_e1225: f64 = (noise_variable_4 - noise_variable_46);
        (noise_metadata_schedule_85_e1225,)
    } else {
        (noise_variable_47,)
    }
};
            noise_variable_47 = noise_metadata_schedule_85_e1227;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_86_e1240,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_86_e1238: f64 = (noise_variable_47 * noise_variable_47);
        (noise_metadata_schedule_86_e1238,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_86_e1240;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_87_e1263,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_87_e1253: f64 = (params.p12 * noise_variable_48);
        let noise_metadata_schedule_87_e1254: f64 = (noise_variable_47 + noise_metadata_schedule_87_e1253);
        let noise_metadata_schedule_87_e1257: f64 = (params.p13 * noise_variable_48);
        let noise_metadata_schedule_87_e1259: f64 = (noise_metadata_schedule_87_e1257 * noise_variable_47);
        let noise_metadata_schedule_87_e1260: f64 = (noise_metadata_schedule_87_e1254 + noise_metadata_schedule_87_e1259);
        let noise_metadata_schedule_87_e1261: f64 = (noise_variable_44 * noise_metadata_schedule_87_e1260);
        (noise_metadata_schedule_87_e1261,)
    } else {
        (noise_variable_13,)
    }
};
            noise_variable_13 = noise_metadata_schedule_87_e1263;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_88_e1276,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_88_e1274: f64 = (noise_variable_3 - noise_variable_46);
        (noise_metadata_schedule_88_e1274,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_88_e1276;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_89_e1289,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_89_e1287: f64 = (noise_variable_49 * noise_variable_49);
        (noise_metadata_schedule_89_e1287,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_89_e1289;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_90_e1312,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_90_e1302: f64 = (params.p12 * noise_variable_50);
        let noise_metadata_schedule_90_e1303: f64 = (noise_variable_49 + noise_metadata_schedule_90_e1302);
        let noise_metadata_schedule_90_e1306: f64 = (params.p13 * noise_variable_49);
        let noise_metadata_schedule_90_e1308: f64 = (noise_metadata_schedule_90_e1306 * noise_variable_50);
        let noise_metadata_schedule_90_e1309: f64 = (noise_metadata_schedule_90_e1303 + noise_metadata_schedule_90_e1308);
        let noise_metadata_schedule_90_e1310: f64 = (noise_variable_44 * noise_metadata_schedule_90_e1309);
        (noise_metadata_schedule_90_e1310,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_90_e1312;
        }
        if matches!(source_index, 2 | 3 | 8 | 9 | 10) {
            let (noise_metadata_schedule_91_e1333,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_91_e1324: f64 = { let limexp_arg = noise_variable_13; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_91_e1326: f64 = (-noise_variable_13);
        let noise_metadata_schedule_91_e1327: f64 = { let limexp_arg = noise_metadata_schedule_91_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_91_e1328: f64 = (noise_metadata_schedule_91_e1324 - noise_metadata_schedule_91_e1327);
        let noise_metadata_schedule_91_e1329: f64 = (0.5 * noise_metadata_schedule_91_e1328);
        let noise_metadata_schedule_91_e1330: f64 = (noise_metadata_schedule_91_e1329).tanh();
        let noise_metadata_schedule_91_e1331: f64 = (1.0 + noise_metadata_schedule_91_e1330);
        (noise_metadata_schedule_91_e1331,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_91_e1333;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_92_e1354,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_92_e1345: f64 = { let limexp_arg = noise_variable_55; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_92_e1347: f64 = (-noise_variable_55);
        let noise_metadata_schedule_92_e1348: f64 = { let limexp_arg = noise_metadata_schedule_92_e1347; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_92_e1349: f64 = (noise_metadata_schedule_92_e1345 - noise_metadata_schedule_92_e1348);
        let noise_metadata_schedule_92_e1350: f64 = (0.5 * noise_metadata_schedule_92_e1349);
        let noise_metadata_schedule_92_e1351: f64 = (noise_metadata_schedule_92_e1350).tanh();
        let noise_metadata_schedule_92_e1352: f64 = (1.0 + noise_metadata_schedule_92_e1351);
        (noise_metadata_schedule_92_e1352,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_92_e1354;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_93_e1369,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_93_e1366: f64 = (params.p15 * noise_variable_60);
        let noise_metadata_schedule_93_e1367: f64 = (params.p14 + noise_metadata_schedule_93_e1366);
        (noise_metadata_schedule_93_e1367,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_93_e1369;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_94_e1384,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_94_e1381: f64 = (params.p15 * noise_variable_62);
        let noise_metadata_schedule_94_e1382: f64 = (params.p14 + noise_metadata_schedule_94_e1381);
        (noise_metadata_schedule_94_e1382,)
    } else {
        (noise_variable_2,)
    }
};
            noise_variable_2 = noise_metadata_schedule_94_e1384;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_95_e1398,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_95_e1395: f64 = (noise_variable_1 * noise_variable_5);
        let noise_metadata_schedule_95_e1396: f64 = (noise_metadata_schedule_95_e1395).tanh();
        (noise_metadata_schedule_95_e1396,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_95_e1398;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_96_e1412,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_96_e1409: f64 = (noise_variable_2 * noise_variable_5);
        let noise_metadata_schedule_96_e1410: f64 = (noise_metadata_schedule_96_e1409).tanh();
        (noise_metadata_schedule_96_e1410,)
    } else {
        (noise_variable_66,)
    }
};
            noise_variable_66 = noise_metadata_schedule_96_e1412;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_97_e1427,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_97_e1424: f64 = (params.p17 * noise_variable_62);
        let noise_metadata_schedule_97_e1425: f64 = (params.p16 + noise_metadata_schedule_97_e1424);
        (noise_metadata_schedule_97_e1425,)
    } else {
        (noise_variable_52,)
    }
};
            noise_variable_52 = noise_metadata_schedule_97_e1427;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_98_e1442,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_98_e1439: f64 = (params.p17 * noise_variable_60);
        let noise_metadata_schedule_98_e1440: f64 = (params.p16 + noise_metadata_schedule_98_e1439);
        (noise_metadata_schedule_98_e1440,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_98_e1442;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_99_e1472,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_99_e1453: f64 = (noise_variable_26 * noise_variable_60);
        let noise_metadata_schedule_99_e1456: f64 = (1.0 + noise_variable_65);
        let noise_metadata_schedule_99_e1457: f64 = (noise_metadata_schedule_99_e1453 * noise_metadata_schedule_99_e1456);
        let noise_metadata_schedule_99_e1461: f64 = (noise_variable_54 * noise_variable_5);
        let noise_metadata_schedule_99_e1462: f64 = (1.0 + noise_metadata_schedule_99_e1461);
        let noise_metadata_schedule_99_e1466: f64 = (noise_variable_5 - noise_variable_38);
        let noise_metadata_schedule_99_e1467: f64 = { let limexp_arg = noise_metadata_schedule_99_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_99_e1468: f64 = (noise_variable_30 * noise_metadata_schedule_99_e1467);
        let noise_metadata_schedule_99_e1469: f64 = (noise_metadata_schedule_99_e1462 + noise_metadata_schedule_99_e1468);
        let noise_metadata_schedule_99_e1470: f64 = (noise_metadata_schedule_99_e1457 * noise_metadata_schedule_99_e1469);
        (noise_metadata_schedule_99_e1470,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_99_e1472;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_100_e1495,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_100_e1483: f64 = (noise_variable_26 * noise_variable_62);
        let noise_metadata_schedule_100_e1486: f64 = (1.0 - noise_variable_66);
        let noise_metadata_schedule_100_e1487: f64 = (noise_metadata_schedule_100_e1483 * noise_metadata_schedule_100_e1486);
        let noise_metadata_schedule_100_e1491: f64 = (noise_variable_52 * noise_variable_5);
        let noise_metadata_schedule_100_e1492: f64 = (1.0 - noise_metadata_schedule_100_e1491);
        let noise_metadata_schedule_100_e1493: f64 = (noise_metadata_schedule_100_e1487 * noise_metadata_schedule_100_e1492);
        (noise_metadata_schedule_100_e1493,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_100_e1495;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_101_e1510,) = {
    if ((noise_variable_89 != 0.0) && (!(((noise_variable_86 != 0.0) || (noise_variable_87 != 0.0)) || (noise_variable_88 != 0.0)))) {
        let noise_metadata_schedule_101_e1507: f64 = (noise_variable_57 - noise_variable_58);
        let noise_metadata_schedule_101_e1508: f64 = (0.5 * noise_metadata_schedule_101_e1507);
        (noise_metadata_schedule_101_e1508,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_101_e1510;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_102_e1517: f64 = if ((params.p4 == 0.0) || (params.p4 == 1.0)) { 1.0 } else { 0.0 };
            noise_variable_90 = noise_metadata_schedule_102_e1517;
        }
        if matches!(source_index, 3) {
            let (noise_metadata_schedule_104_e1535,) = {
    if (noise_variable_90 != 0.0) {
        let noise_metadata_schedule_104_e1532: f64 = (params.p44 * noise_variable_59);
        let noise_metadata_schedule_104_e1533: f64 = (params.p43 + noise_metadata_schedule_104_e1532);
        (noise_metadata_schedule_104_e1533,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_104_e1535;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_105_e1543,) = {
    if (noise_variable_90 != 0.0) {
        let noise_metadata_schedule_105_e1540: f64 = (params.p44 * noise_variable_59);
        let noise_metadata_schedule_105_e1541: f64 = (params.p46 + noise_metadata_schedule_105_e1540);
        (noise_metadata_schedule_105_e1541,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_105_e1543;
        }
        if matches!(source_index, 3) {
            let (noise_metadata_schedule_107_e1563,) = {
    if (noise_variable_90 == 0.0) {
        let noise_metadata_schedule_107_e1560: f64 = (params.p44 * noise_variable_60);
        let noise_metadata_schedule_107_e1561: f64 = (params.p43 + noise_metadata_schedule_107_e1560);
        (noise_metadata_schedule_107_e1561,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_107_e1563;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_108_e1572,) = {
    if (noise_variable_90 == 0.0) {
        let noise_metadata_schedule_108_e1569: f64 = (params.p44 * noise_variable_60);
        let noise_metadata_schedule_108_e1570: f64 = (params.p46 + noise_metadata_schedule_108_e1569);
        (noise_metadata_schedule_108_e1570,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_108_e1572;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_109_e1577: f64 = if ((noise_variable_12 != 0.0) || (params.p57 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_91 = noise_metadata_schedule_109_e1577;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_110_e1587,) = {
    if (noise_variable_91 != 0.0) {
        let noise_metadata_schedule_110_e1583: f64 = (params.p66 * noise_variable_12);
        let noise_metadata_schedule_110_e1584: f64 = (1.0 + noise_metadata_schedule_110_e1583);
        let noise_metadata_schedule_110_e1585: f64 = (noise_variable_29 * noise_metadata_schedule_110_e1584);
        (noise_metadata_schedule_110_e1585,)
    } else {
        (noise_variable_36,)
    }
};
            noise_variable_36 = noise_metadata_schedule_110_e1587;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_114_e1617,) = {
    if (noise_variable_91 == 0.0) {
        (noise_variable_29,)
    } else {
        (noise_variable_36,)
    }
};
            noise_variable_36 = noise_metadata_schedule_114_e1617;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_116_e1625: f64 = if params.p5 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_92 = noise_metadata_schedule_116_e1625;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_117_e1636,) = {
    if (noise_variable_92 != 0.0) {
        let noise_metadata_schedule_117_e1629: f64 = (-1.0);
        let noise_metadata_schedule_117_e1631: f64 = (noise_metadata_schedule_117_e1629 * noise_variable_42);
        let noise_metadata_schedule_117_e1632: f64 = (noise_metadata_schedule_117_e1631).tanh();
        let noise_metadata_schedule_117_e1633: f64 = (noise_variable_15 * noise_metadata_schedule_117_e1632);
        let noise_metadata_schedule_117_e1634: f64 = { let limexp_arg = noise_metadata_schedule_117_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (noise_metadata_schedule_117_e1634,)
    } else {
        (noise_variable_47,)
    }
};
            noise_variable_47 = noise_metadata_schedule_117_e1636;
        }
        if matches!(source_index, 12 | 14) {
            let (noise_metadata_schedule_118_e1644,) = {
    if (noise_variable_92 != 0.0) {
        let noise_metadata_schedule_118_e1641: f64 = (noise_variable_79 - noise_variable_42);
        let noise_metadata_schedule_118_e1642: f64 = noise_metadata_schedule_118_e1641;
        (noise_metadata_schedule_118_e1642,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_118_e1644;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            let (noise_metadata_schedule_119_e1652,) = {
    if (noise_variable_92 != 0.0) {
        let noise_metadata_schedule_119_e1649: f64 = (noise_variable_80 - noise_variable_42);
        let noise_metadata_schedule_119_e1650: f64 = noise_metadata_schedule_119_e1649;
        (noise_metadata_schedule_119_e1650,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_119_e1652;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let (noise_metadata_schedule_120_e1661,) = {
    if (noise_variable_92 == 0.0) {
        let noise_metadata_schedule_120_e1656: f64 = (-noise_variable_15);
        let noise_metadata_schedule_120_e1658: f64 = (noise_metadata_schedule_120_e1656 * noise_variable_42);
        let noise_metadata_schedule_120_e1659: f64 = { let limexp_arg = noise_metadata_schedule_120_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (noise_metadata_schedule_120_e1659,)
    } else {
        (noise_variable_47,)
    }
};
            noise_variable_47 = noise_metadata_schedule_120_e1661;
        }
        if matches!(source_index, 4 | 5 | 12 | 13 | 14 | 15) {
            let noise_metadata_schedule_121_e1664: f64 = if params.p5 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_93 = noise_metadata_schedule_121_e1664;
        }
        if matches!(source_index, 12 | 14) {
            let (noise_metadata_schedule_122_e1674,) = {
    if ((noise_variable_92 == 0.0) && (noise_variable_93 != 0.0)) {
        let noise_metadata_schedule_122_e1671: f64 = (noise_variable_79 - noise_variable_42);
        let noise_metadata_schedule_122_e1672: f64 = (noise_metadata_schedule_122_e1671).tanh();
        (noise_metadata_schedule_122_e1672,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_122_e1674;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            let (noise_metadata_schedule_123_e1684,) = {
    if ((noise_variable_92 == 0.0) && (noise_variable_93 != 0.0)) {
        let noise_metadata_schedule_123_e1681: f64 = (noise_variable_80 - noise_variable_42);
        let noise_metadata_schedule_123_e1682: f64 = (noise_metadata_schedule_123_e1681).tanh();
        (noise_metadata_schedule_123_e1682,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_123_e1684;
        }
        if matches!(source_index, 12 | 14) {
            let (noise_metadata_schedule_124_e1694,) = {
    if ((noise_variable_92 == 0.0) && (noise_variable_93 == 0.0)) {
        let noise_metadata_schedule_124_e1692: f64 = (noise_variable_79 - noise_variable_42);
        (noise_metadata_schedule_124_e1692,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_124_e1694;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            let (noise_metadata_schedule_125_e1704,) = {
    if ((noise_variable_92 == 0.0) && (noise_variable_93 == 0.0)) {
        let noise_metadata_schedule_125_e1702: f64 = (noise_variable_80 - noise_variable_42);
        (noise_metadata_schedule_125_e1702,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_125_e1704;
        }
        if matches!(source_index, 12 | 14) {
            let noise_metadata_schedule_126_e1708: f64 = (noise_variable_15 * noise_variable_16);
            let noise_metadata_schedule_126_e1709: f64 = { let limexp_arg = noise_metadata_schedule_126_e1708; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_126_e1711: f64 = (noise_metadata_schedule_126_e1709 - noise_variable_47);
            let noise_metadata_schedule_126_e1712: f64 = (params.p38 * noise_metadata_schedule_126_e1711);
            noise_variable_7 = noise_metadata_schedule_126_e1712;
        }
        if matches!(source_index, 4 | 5 | 13 | 15) {
            let noise_metadata_schedule_127_e1716: f64 = (noise_variable_15 * noise_variable_17);
            let noise_metadata_schedule_127_e1717: f64 = { let limexp_arg = noise_metadata_schedule_127_e1716; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_127_e1719: f64 = (noise_metadata_schedule_127_e1717 - noise_variable_47);
            let noise_metadata_schedule_127_e1720: f64 = (params.p38 * noise_metadata_schedule_127_e1719);
            noise_variable_8 = noise_metadata_schedule_127_e1720;
        }
        if matches!(source_index, 4 | 5 | 6 | 7 | 8 | 9 | 10) {
            let noise_metadata_schedule_168_e2059: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_107 = noise_metadata_schedule_168_e2059;
        }
        if matches!(source_index, 6 | 7 | 8 | 9 | 10) {
            let noise_metadata_schedule_169_e2062: f64 = if params.p7 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_108 = noise_metadata_schedule_169_e2062;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_171_e2073,) = {
    if (noise_variable_107 != 0.0) {
        let noise_metadata_schedule_171_e2068: f64 = (noise_variable_14).abs();
        let noise_metadata_schedule_171_e2070: f64 = (noise_variable_8).abs();
        let noise_metadata_schedule_171_e2071: f64 = (noise_metadata_schedule_171_e2068 + noise_metadata_schedule_171_e2070);
        (noise_metadata_schedule_171_e2071,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_171_e2073;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_172_e2094,) = {
    if (noise_variable_107 != 0.0) {
        let noise_metadata_schedule_172_e2077: f64 = (params.p78 + 273.15);
        let noise_metadata_schedule_172_e2081: f64 = (params.p80 * noise_variable_59);
        let noise_metadata_schedule_172_e2083: f64 = (noise_variable_63).abs();
        let noise_metadata_schedule_172_e2084: f64 = (noise_metadata_schedule_172_e2081 * noise_metadata_schedule_172_e2083);
        let noise_metadata_schedule_172_e2088: f64 = (params.p16 * noise_variable_5);
        let noise_metadata_schedule_172_e2089: f64 = (1.0 + noise_metadata_schedule_172_e2088);
        let noise_metadata_schedule_172_e2090: f64 = (noise_metadata_schedule_172_e2084 * noise_metadata_schedule_172_e2089);
        let noise_metadata_schedule_172_e2091: f64 = (1.0 + noise_metadata_schedule_172_e2090);
        let noise_metadata_schedule_172_e2092: f64 = (noise_metadata_schedule_172_e2077 * noise_metadata_schedule_172_e2091);
        (noise_metadata_schedule_172_e2092,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_172_e2094;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_173_e2118,) = {
    if (noise_variable_107 != 0.0) {
        let noise_metadata_schedule_173_e2098: f64 = (params.p84 * 4.0);
        let noise_metadata_schedule_173_e2100: f64 = (noise_metadata_schedule_173_e2098 * 1.3806503e-23);
        let noise_metadata_schedule_173_e2102: f64 = (noise_metadata_schedule_173_e2100 * noise_variable_11);
        let noise_metadata_schedule_173_e2105: f64 = (noise_variable_112 / noise_variable_11);
        let noise_metadata_schedule_173_e2107: f64 = (noise_metadata_schedule_173_e2105 * noise_variable_111);
        let noise_metadata_schedule_173_e2110: f64 = (params.p79 * noise_variable_111);
        let noise_metadata_schedule_173_e2112: f64 = (noise_metadata_schedule_173_e2110 * noise_variable_111);
        let noise_metadata_schedule_173_e2113: f64 = (noise_metadata_schedule_173_e2107 + noise_metadata_schedule_173_e2112);
        let noise_metadata_schedule_173_e2114: f64 = (noise_metadata_schedule_173_e2113).abs();
        let noise_metadata_schedule_173_e2115: f64 = (noise_metadata_schedule_173_e2114).sqrt();
        let noise_metadata_schedule_173_e2116: f64 = (noise_metadata_schedule_173_e2102 * noise_metadata_schedule_173_e2115);
        (noise_metadata_schedule_173_e2116,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_173_e2118;
        }
        if matches!(source_index, 8 | 9 | 10) {
            let (noise_metadata_schedule_174_e2129,) = {
    if (((noise_variable_108 != 0.0) && (noise_variable_107 == 0.0)) && (params.p0 != 0.0)) {
        let noise_metadata_schedule_174_e2127: f64 = 0.0;
        (noise_metadata_schedule_174_e2127,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_174_e2129;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_175_e2146,) = {
    if (((noise_variable_108 != 0.0) && (noise_variable_107 == 0.0)) && (params.p0 != 0.0)) {
        let noise_metadata_schedule_175_e2138: f64 = (4.0 * 1.3806503e-23);
        let noise_metadata_schedule_175_e2140: f64 = (noise_metadata_schedule_175_e2138 * noise_variable_11);
        let noise_metadata_schedule_175_e2142: f64 = (noise_metadata_schedule_175_e2140 * noise_variable_115);
        let noise_metadata_schedule_175_e2144: f64 = (noise_metadata_schedule_175_e2142 * params.p72);
        (noise_metadata_schedule_175_e2144,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_175_e2146;
        }
        if matches!(source_index, 9) {
            let noise_metadata_schedule_176_e2149: f64 = if noise_variable_115 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_116 = noise_metadata_schedule_176_e2149;
        }
        if matches!(source_index, 9) {
            let (noise_metadata_schedule_177_e2172,) = {
    if ((((noise_variable_108 != 0.0) && (noise_variable_107 == 0.0)) && (params.p0 != 0.0)) && (noise_variable_116 != 0.0)) {
        let noise_metadata_schedule_177_e2160: f64 = (noise_variable_31 * noise_variable_31);
        let noise_metadata_schedule_177_e2162: f64 = (noise_metadata_schedule_177_e2160 * 4.0);
        let noise_metadata_schedule_177_e2164: f64 = (noise_metadata_schedule_177_e2162 * 1.3806503e-23);
        let noise_metadata_schedule_177_e2166: f64 = (noise_metadata_schedule_177_e2164 * noise_variable_11);
        let noise_metadata_schedule_177_e2168: f64 = (noise_metadata_schedule_177_e2166 * params.p71);
        let noise_metadata_schedule_177_e2170: f64 = (noise_metadata_schedule_177_e2168 / noise_variable_115);
        (noise_metadata_schedule_177_e2170,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_177_e2172;
        }
        if matches!(source_index, 9) {
            let (noise_metadata_schedule_178_e2184,) = {
    if ((((noise_variable_108 != 0.0) && (noise_variable_107 == 0.0)) && (params.p0 != 0.0)) && (noise_variable_116 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_178_e2184;
        }
        if matches!(source_index, 6 | 7) {
            let (noise_metadata_schedule_179_e2206,) = {
    if (((noise_variable_108 != 0.0) && (noise_variable_107 == 0.0)) && (params.p0 != 0.0)) {
        let noise_metadata_schedule_179_e2193: f64 = (4.0 * 1.3806503e-23);
        let noise_metadata_schedule_179_e2195: f64 = (noise_metadata_schedule_179_e2193 * noise_variable_11);
        let noise_metadata_schedule_179_e2197: f64 = (noise_metadata_schedule_179_e2195 * params.p73);
        let noise_metadata_schedule_179_e2199: f64 = (noise_metadata_schedule_179_e2197 * noise_variable_31);
        let noise_metadata_schedule_179_e2202: f64 = (params.p72 * params.p71);
        let noise_metadata_schedule_179_e2203: f64 = (noise_metadata_schedule_179_e2202).sqrt();
        let noise_metadata_schedule_179_e2204: f64 = (noise_metadata_schedule_179_e2199 * noise_metadata_schedule_179_e2203);
        (noise_metadata_schedule_179_e2204,)
    } else {
        (noise_variable_120,)
    }
};
            noise_variable_120 = noise_metadata_schedule_179_e2206;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_183_e2262,) = {
    if (((noise_variable_108 != 0.0) && (noise_variable_107 == 0.0)) && (params.p0 != 0.0)) {
        let noise_metadata_schedule_183_e2252: f64 = (4.0 * 1.3806503e-23);
        let noise_metadata_schedule_183_e2254: f64 = (noise_metadata_schedule_183_e2252 * noise_variable_11);
        let noise_metadata_schedule_183_e2256: f64 = (noise_metadata_schedule_183_e2254 * noise_variable_115);
        let noise_metadata_schedule_183_e2258: f64 = (noise_metadata_schedule_183_e2256 * params.p72);
        let noise_metadata_schedule_183_e2260: f64 = (noise_metadata_schedule_183_e2258 * params.p74);
        (noise_metadata_schedule_183_e2260,)
    } else {
        (noise_variable_121,)
    }
};
            noise_variable_121 = noise_metadata_schedule_183_e2262;
        }
        match source_index {
            0 => {
                let noise_0_psd_e2273: f64 = 1.0;
                let noise_0_psd_e181: f64 = (4.0 * 1.3806503e-23);
                let noise_0_psd_e183: f64 = (noise_0_psd_e181 * noise_variable_11);
                let noise_0_psd_e185: f64 = (noise_0_psd_e183 * params.p47);
                let noise_0_psd_e2274: f64 = (noise_0_psd_e2273 * noise_0_psd_e185);
                let psd = noise_0_psd_e2274;
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
                let noise_1_psd_e2276: f64 = 1.0;
                let noise_1_psd_e224: f64 = (4.0 * 1.3806503e-23);
                let noise_1_psd_e226: f64 = (noise_1_psd_e224 * noise_variable_11);
                let noise_1_psd_e228: f64 = (noise_1_psd_e226 * params.p42);
                let noise_1_psd_e2277: f64 = (noise_1_psd_e2276 * noise_1_psd_e228);
                let psd = noise_1_psd_e2277;
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
                let noise_2_psd_e2279: f64 = 1.0;
                let noise_2_psd_e262: f64 = (4.0 * 1.3806503e-23);
                let noise_2_psd_e264: f64 = (noise_2_psd_e262 * noise_variable_11);
                let noise_2_psd_e266: f64 = (noise_2_psd_e264 * noise_variable_36);
                let noise_2_psd_e2280: f64 = (noise_2_psd_e2279 * noise_2_psd_e266);
                let psd = noise_2_psd_e2280;
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
                let noise_3_psd_e2282: f64 = 1.0;
                let noise_3_psd_e298: f64 = (4.0 * 1.3806503e-23);
                let noise_3_psd_e300: f64 = (noise_3_psd_e298 * noise_variable_11);
                let noise_3_psd_e302: f64 = (noise_3_psd_e300 * noise_variable_28);
                let noise_3_psd_e2283: f64 = (noise_3_psd_e2282 * noise_3_psd_e302);
                let psd = noise_3_psd_e2283;
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
                let noise_4_psd_e2285: f64 = 1.0;
                let noise_4_psd_e2286: f64 = (noise_4_psd_e2285 * noise_variable_110);
                let psd = noise_4_psd_e2286;
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
                let noise_5_psd_e2288: f64 = 1.0;
                let noise_5_psd_e338: f64 = (noise_variable_110 * params.p81);
                let noise_5_psd_e2289: f64 = (noise_5_psd_e2288 * noise_5_psd_e338);
                let psd = noise_5_psd_e2289;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
                let exponent: Option<f64> = Some(params.p83);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            6 => {
                let noise_6_psd_e2291: f64 = 1.0;
                let noise_6_psd_e2292: f64 = (noise_6_psd_e2291 * noise_variable_120);
                let psd = noise_6_psd_e2292;
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
                let noise_7_psd_e2294: f64 = 1.0;
                let noise_7_psd_e2295: f64 = (noise_7_psd_e2294 * noise_variable_120);
                let psd = noise_7_psd_e2295;
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
                let noise_8_psd_e2297: f64 = 1.0;
                let noise_8_psd_e2298: f64 = (noise_8_psd_e2297 * noise_variable_113);
                let psd = noise_8_psd_e2298;
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
                let noise_9_psd_e2300: f64 = 1.0;
                let noise_9_psd_e2301: f64 = (noise_9_psd_e2300 * noise_variable_114);
                let psd = noise_9_psd_e2301;
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
                let noise_10_psd_e2303: f64 = 1.0;
                let noise_10_psd_e2304: f64 = (noise_10_psd_e2303 * noise_variable_121);
                let psd = noise_10_psd_e2304;
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
                let noise_11_psd_e2306: f64 = 1.0;
                let noise_11_psd_e467: f64 = (noise_variable_14).powf(params.p76);
                let noise_11_psd_e468: f64 = (params.p75 * noise_11_psd_e467);
                let noise_11_psd_e2307: f64 = (noise_11_psd_e2306 * noise_11_psd_e468);
                let psd = noise_11_psd_e2307;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
                let exponent: Option<f64> = Some(params.p77);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            12 => {
                let noise_12_psd_e2309: f64 = 1.0;
                let noise_12_psd_e479: f64 = (2.0 * 1.602176462e-19);
                let noise_12_psd_e481: f64 = (noise_variable_7).abs();
                let noise_12_psd_e482: f64 = (noise_12_psd_e479 * noise_12_psd_e481);
                let noise_12_psd_e2310: f64 = (noise_12_psd_e2309 * noise_12_psd_e482);
                let psd = noise_12_psd_e2310;
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
                let noise_13_psd_e2312: f64 = 1.0;
                let noise_13_psd_e490: f64 = (2.0 * 1.602176462e-19);
                let noise_13_psd_e492: f64 = (noise_variable_8).abs();
                let noise_13_psd_e493: f64 = (noise_13_psd_e490 * noise_13_psd_e492);
                let noise_13_psd_e2313: f64 = (noise_13_psd_e2312 * noise_13_psd_e493);
                let psd = noise_13_psd_e2313;
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
                let noise_14_psd_e2315: f64 = 1.0;
                let noise_14_psd_e503: f64 = (noise_variable_7).abs();
                let noise_14_psd_e505: f64 = (noise_14_psd_e503).powf(params.p76);
                let noise_14_psd_e506: f64 = (params.p75 * noise_14_psd_e505);
                let noise_14_psd_e2316: f64 = (noise_14_psd_e2315 * noise_14_psd_e506);
                let psd = noise_14_psd_e2316;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
                let exponent: Option<f64> = Some(params.p77);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            15 => {
                let noise_15_psd_e2318: f64 = 1.0;
                let noise_15_psd_e517: f64 = (noise_variable_8).abs();
                let noise_15_psd_e519: f64 = (noise_15_psd_e517).powf(params.p76);
                let noise_15_psd_e520: f64 = (params.p75 * noise_15_psd_e519);
                let noise_15_psd_e2319: f64 = (noise_15_psd_e2318 * noise_15_psd_e520);
                let psd = noise_15_psd_e2319;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
                let exponent: Option<f64> = Some(params.p77);
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

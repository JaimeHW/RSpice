#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 28] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_NOI_GND_IN", label: Some("in"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "noi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_E1_IB2E1", label: Some("ib2e1"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_E1_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_B1_RBC", label: Some("rbc"), kind: GeneratedNoiseKind::White, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_B2_RBV", label: Some("rbv"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B2_E1_IB2E1_F", label: Some("ib2e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_E1_IB1E1_F", label: Some("ib1e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_E1_IB1E1", label: Some("ib1e1"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IB3", label: Some("ib3"), kind: GeneratedNoiseKind::White, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IB3_F", label: Some("ib3_f"), kind: GeneratedNoiseKind::Flicker, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IEX", label: Some("iex"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IEX_F", label: Some("iex_f"), kind: GeneratedNoiseKind::Flicker, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_C3_XIEX", label: Some("xiex"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B_C3_XIEX_F", label: Some("xiex_f"), kind: GeneratedNoiseKind::Flicker, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C1_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_S_ISUB_INT", label: Some("isub_int"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_S_ISUB", label: Some("isub"), kind: GeneratedNoiseKind::White, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_S_XISUB", label: Some("xisub"), kind: GeneratedNoiseKind::White, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 57, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C4_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C1_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C4_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 62, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 63, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C1_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 64, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
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
        let mut noise_variable_145 = 0.0;
        let mut noise_variable_146 = 0.0;
        let mut noise_variable_147 = 0.0;
        let mut noise_variable_148 = 0.0;
        let mut noise_variable_149 = 0.0;
        let mut noise_variable_150 = 0.0;
        let mut noise_variable_151 = 0.0;
        let mut noise_variable_152 = 0.0;
        let mut noise_variable_153 = 0.0;
        let mut noise_variable_154 = 0.0;
        let mut noise_variable_155 = 0.0;
        let mut noise_variable_156 = 0.0;
        let mut noise_variable_157 = 0.0;
        let mut noise_variable_158 = 0.0;
        let mut noise_variable_159 = 0.0;
        let mut noise_variable_160 = 0.0;
        let mut noise_variable_161 = 0.0;
        let mut noise_variable_162 = 0.0;
        let mut noise_variable_163 = 0.0;
        let mut noise_variable_164 = 0.0;
        let mut noise_variable_165 = 0.0;
        let mut noise_variable_166 = 0.0;
        let mut noise_variable_167 = 0.0;
        let mut noise_variable_168 = 0.0;
        let mut noise_variable_169 = 0.0;
        let mut noise_variable_170 = 0.0;
        let mut noise_variable_171 = 0.0;
        let mut noise_variable_172 = 0.0;
        let mut noise_variable_173 = 0.0;
        let mut noise_variable_174 = 0.0;
        let mut noise_variable_175 = 0.0;
        let mut noise_variable_176 = 0.0;
        let mut noise_variable_177 = 0.0;
        let mut noise_variable_178 = 0.0;
        let mut noise_variable_179 = 0.0;
        let mut noise_variable_180 = 0.0;
        let mut noise_variable_181 = 0.0;
        let mut noise_variable_182 = 0.0;
        let mut noise_variable_183 = 0.0;
        let mut noise_variable_184 = 0.0;
        let mut noise_variable_185 = 0.0;
        let mut noise_variable_186 = 0.0;
        let mut noise_variable_187 = 0.0;
        let mut noise_variable_188 = 0.0;
        let mut noise_variable_189 = 0.0;
        let mut noise_variable_190 = 0.0;
        let mut noise_variable_191 = 0.0;
        let mut noise_variable_192 = 0.0;
        let mut noise_variable_193 = 0.0;
        let mut noise_variable_194 = 0.0;
        let mut noise_variable_195 = 0.0;
        let mut noise_variable_196 = 0.0;
        let mut noise_variable_197 = 0.0;
        let mut noise_variable_198 = 0.0;
        let mut noise_variable_199 = 0.0;
        let mut noise_variable_200 = 0.0;
        let mut noise_variable_201 = 0.0;
        let mut noise_variable_202 = 0.0;
        let mut noise_variable_203 = 0.0;
        let mut noise_variable_204 = 0.0;
        let mut noise_variable_205 = 0.0;
        let mut noise_variable_206 = 0.0;
        let mut noise_variable_207 = 0.0;
        let mut noise_variable_208 = 0.0;
        let mut noise_variable_209 = 0.0;
        let mut noise_variable_210 = 0.0;
        let mut noise_variable_211 = 0.0;
        let mut noise_variable_212 = 0.0;
        let mut noise_variable_213 = 0.0;
        let mut noise_variable_214 = 0.0;
        let mut noise_variable_215 = 0.0;
        let mut noise_variable_216 = 0.0;
        let mut noise_variable_217 = 0.0;
        let mut noise_variable_218 = 0.0;
        let mut noise_variable_219 = 0.0;
        let mut noise_variable_220 = 0.0;
        let mut noise_variable_221 = 0.0;
        let mut noise_variable_222 = 0.0;
        let mut noise_variable_223 = 0.0;
        let mut noise_variable_224 = 0.0;
        let mut noise_variable_225 = 0.0;
        let mut noise_variable_226 = 0.0;
        let mut noise_variable_227 = 0.0;
        let mut noise_variable_228 = 0.0;
        let mut noise_variable_229 = 0.0;
        let mut noise_variable_230 = 0.0;
        let mut noise_variable_231 = 0.0;
        let mut noise_variable_232 = 0.0;
        let mut noise_variable_233 = 0.0;
        let mut noise_variable_234 = 0.0;
        let mut noise_variable_235 = 0.0;
        let mut noise_variable_236 = 0.0;
        let mut noise_variable_237 = 0.0;
        let mut noise_variable_238 = 0.0;
        let mut noise_variable_239 = 0.0;
        let mut noise_variable_240 = 0.0;
        let mut noise_variable_241 = 0.0;
        let mut noise_variable_242 = 0.0;
        let mut noise_variable_243 = 0.0;
        let mut noise_variable_244 = 0.0;
        let mut noise_variable_245 = 0.0;
        let mut noise_variable_246 = 0.0;
        let mut noise_variable_247 = 0.0;
        let mut noise_variable_248 = 0.0;
        let mut noise_variable_249 = 0.0;
        let mut noise_variable_250 = 0.0;
        let mut noise_variable_251 = 0.0;
        let mut noise_variable_252 = 0.0;
        let mut noise_variable_253 = 0.0;
        let mut noise_variable_254 = 0.0;
        let mut noise_variable_255 = 0.0;
        let mut noise_variable_256 = 0.0;
        let mut noise_variable_257 = 0.0;
        let mut noise_variable_258 = 0.0;
        let mut noise_variable_259 = 0.0;
        let mut noise_variable_260 = 0.0;
        let mut noise_variable_261 = 0.0;
        let mut noise_variable_262 = 0.0;
        let mut noise_variable_263 = 0.0;
        let mut noise_variable_264 = 0.0;
        let mut noise_variable_265 = 0.0;
        let mut noise_variable_266 = 0.0;
        let mut noise_variable_267 = 0.0;
        let mut noise_variable_268 = 0.0;
        let mut noise_variable_269 = 0.0;
        let mut noise_variable_270 = 0.0;
        let mut noise_variable_271 = 0.0;
        let mut noise_variable_272 = 0.0;
        let mut noise_variable_273 = 0.0;
        let mut noise_variable_274 = 0.0;
        let mut noise_variable_275 = 0.0;
        let mut noise_variable_276 = 0.0;
        let mut noise_variable_277 = 0.0;
        let mut noise_variable_278 = 0.0;
        let mut noise_variable_279 = 0.0;
        let mut noise_variable_280 = 0.0;
        let mut noise_variable_281 = 0.0;
        let mut noise_variable_282 = 0.0;
        let mut noise_variable_283 = 0.0;
        let mut noise_variable_284 = 0.0;
        let mut noise_variable_285 = 0.0;
        let mut noise_variable_286 = 0.0;
        let mut noise_variable_287 = 0.0;
        let mut noise_variable_288 = 0.0;
        let mut noise_variable_289 = 0.0;
        let mut noise_variable_290 = 0.0;
        let mut noise_variable_291 = 0.0;
        let mut noise_variable_292 = 0.0;
        let mut noise_variable_293 = 0.0;
        let mut noise_variable_294 = 0.0;
        let mut noise_variable_295 = 0.0;
        let mut noise_variable_296 = 0.0;
        let mut noise_variable_297 = 0.0;
        let mut noise_variable_298 = 0.0;
        let mut noise_variable_299 = 0.0;
        let mut noise_variable_300 = 0.0;
        let mut noise_variable_301 = 0.0;
        let mut noise_variable_302 = 0.0;
        let mut noise_variable_303 = 0.0;
        let mut noise_variable_304 = 0.0;
        let mut noise_variable_305 = 0.0;
        let mut noise_variable_306 = 0.0;
        let mut noise_variable_307 = 0.0;
        let mut noise_variable_308 = 0.0;
        let mut noise_variable_309 = 0.0;
        let mut noise_variable_310 = 0.0;
        let mut noise_variable_311 = 0.0;
        let mut noise_variable_312 = 0.0;
        let mut noise_variable_313 = 0.0;
        let mut noise_variable_314 = 0.0;
        let mut noise_variable_315 = 0.0;
        let mut noise_variable_316 = 0.0;
        let mut noise_variable_317 = 0.0;
        let mut noise_variable_318 = 0.0;
        let mut noise_variable_319 = 0.0;
        let mut noise_variable_320 = 0.0;
        let mut noise_variable_321 = 0.0;
        let mut noise_variable_322 = 0.0;
        let mut noise_variable_323 = 0.0;
        let mut noise_variable_324 = 0.0;
        let mut noise_variable_325 = 0.0;
        let mut noise_variable_326 = 0.0;
        let mut noise_variable_327 = 0.0;
        let mut noise_variable_328 = 0.0;
        let mut noise_variable_329 = 0.0;
        let mut noise_variable_330 = 0.0;
        let mut noise_variable_331 = 0.0;
        let mut noise_variable_332 = 0.0;
        let mut noise_variable_333 = 0.0;
        let mut noise_variable_334 = 0.0;
        let mut noise_variable_335 = 0.0;
        let mut noise_variable_336 = 0.0;
        let mut noise_variable_337 = 0.0;
        let mut noise_variable_338 = 0.0;
        let mut noise_variable_339 = 0.0;
        let mut noise_variable_340 = 0.0;
        let mut noise_variable_341 = 0.0;
        let mut noise_variable_342 = 0.0;
        let mut noise_variable_343 = 0.0;
        let mut noise_variable_344 = 0.0;
        let mut noise_variable_345 = 0.0;
        let mut noise_variable_346 = 0.0;
        let mut noise_variable_347 = 0.0;
        let mut noise_variable_348 = 0.0;
        let mut noise_variable_349 = 0.0;
        let mut noise_variable_350 = 0.0;
        let mut noise_variable_351 = 0.0;
        let mut noise_variable_352 = 0.0;
        let mut noise_variable_353 = 0.0;
        let mut noise_variable_354 = 0.0;
        let mut noise_variable_355 = 0.0;
        let mut noise_variable_356 = 0.0;
        let mut noise_variable_357 = 0.0;
        let mut noise_variable_358 = 0.0;
        let mut noise_variable_359 = 0.0;
        let mut noise_variable_360 = 0.0;
        let mut noise_variable_361 = 0.0;
        let mut noise_variable_362 = 0.0;
        let mut noise_variable_363 = 0.0;
        let mut noise_variable_364 = 0.0;
        let mut noise_variable_365 = 0.0;
        let mut noise_variable_366 = 0.0;
        let mut noise_variable_367 = 0.0;
        let mut noise_variable_368 = 0.0;
        let mut noise_variable_369 = 0.0;
        let mut noise_variable_370 = 0.0;
        let mut noise_variable_371 = 0.0;
        let mut noise_variable_372 = 0.0;
        let mut noise_variable_373 = 0.0;
        let mut noise_variable_374 = 0.0;
        let mut noise_variable_375 = 0.0;
        let mut noise_variable_376 = 0.0;
        let mut noise_variable_377 = 0.0;
        let mut noise_variable_378 = 0.0;
        let mut noise_variable_379 = 0.0;
        let mut noise_variable_380 = 0.0;
        let mut noise_variable_381 = 0.0;
        let mut noise_variable_382 = 0.0;
        let mut noise_variable_383 = 0.0;
        let mut noise_variable_384 = 0.0;
        let mut noise_variable_385 = 0.0;
        let mut noise_variable_386 = 0.0;
        let mut noise_variable_387 = 0.0;
        let mut noise_variable_388 = 0.0;
        let mut noise_variable_389 = 0.0;
        let mut noise_variable_390 = 0.0;
        let mut noise_variable_391 = 0.0;
        let mut noise_variable_392 = 0.0;
        let mut noise_variable_393 = 0.0;
        let mut noise_variable_394 = 0.0;
        let mut noise_variable_395 = 0.0;
        let mut noise_variable_396 = 0.0;
        let mut noise_variable_397 = 0.0;
        let mut noise_variable_398 = 0.0;
        let mut noise_variable_399 = 0.0;
        let mut noise_variable_400 = 0.0;
        let mut noise_variable_401 = 0.0;
        let mut noise_variable_402 = 0.0;
        let mut noise_variable_403 = 0.0;
        let mut noise_variable_404 = 0.0;
        let mut noise_variable_405 = 0.0;
        let mut noise_variable_406 = 0.0;
        let mut noise_variable_407 = 0.0;
        let mut noise_variable_408 = 0.0;
        let mut noise_variable_409 = 0.0;
        let mut noise_variable_410 = 0.0;
        let mut noise_variable_411 = 0.0;
        let mut noise_variable_412 = 0.0;
        let mut noise_variable_413 = 0.0;
        let mut noise_variable_414 = 0.0;
        let mut noise_variable_415 = 0.0;
        let mut noise_variable_416 = 0.0;
        let mut noise_variable_417 = 0.0;
        let mut noise_variable_418 = 0.0;
        let mut noise_variable_419 = 0.0;
        let mut noise_variable_420 = 0.0;
        let mut noise_variable_421 = 0.0;
        let mut noise_variable_422 = 0.0;
        let mut noise_variable_423 = 0.0;
        let mut noise_variable_424 = 0.0;
        let mut noise_variable_425 = 0.0;
        let mut noise_variable_426 = 0.0;
        let mut noise_variable_427 = 0.0;
        let mut noise_variable_428 = 0.0;
        let mut noise_variable_429 = 0.0;
        let mut noise_variable_430 = 0.0;
        let mut noise_variable_431 = 0.0;
        let mut noise_variable_432 = 0.0;
        let mut noise_variable_433 = 0.0;
        let mut noise_variable_434 = 0.0;
        let mut noise_variable_435 = 0.0;
        let mut noise_variable_436 = 0.0;
        let mut noise_variable_437 = 0.0;
        let mut noise_variable_438 = 0.0;
        let mut noise_variable_439 = 0.0;
        let mut noise_variable_440 = 0.0;
        let mut noise_variable_441 = 0.0;
        let mut noise_variable_442 = 0.0;
        let mut noise_variable_443 = 0.0;
        let mut noise_variable_444 = 0.0;
        let mut noise_variable_445 = 0.0;
        let mut noise_variable_446 = 0.0;
        let mut noise_variable_447 = 0.0;
        let mut noise_variable_448 = 0.0;
        let mut noise_variable_449 = 0.0;
        let mut noise_variable_450 = 0.0;
        let mut noise_variable_451 = 0.0;
        let mut noise_variable_452 = 0.0;
        let mut noise_variable_453 = 0.0;
        let mut noise_variable_454 = 0.0;
        let mut noise_variable_455 = 0.0;
        let mut noise_variable_456 = 0.0;
        let mut noise_variable_457 = 0.0;
        let mut noise_variable_458 = 0.0;
        let mut noise_variable_459 = 0.0;
        let mut noise_variable_460 = 0.0;
        let mut noise_variable_461 = 0.0;
        let mut noise_variable_462 = 0.0;
        let mut noise_variable_463 = 0.0;
        let mut noise_variable_464 = 0.0;
        let mut noise_variable_465 = 0.0;
        let mut noise_variable_466 = 0.0;
        let mut noise_variable_467 = 0.0;
        let mut noise_variable_468 = 0.0;
        let mut noise_variable_469 = 0.0;
        let mut noise_variable_470 = 0.0;
        let mut noise_variable_471 = 0.0;
        let mut noise_variable_472 = 0.0;
        let mut noise_variable_473 = 0.0;
        let mut noise_variable_474 = 0.0;
        let mut noise_variable_475 = 0.0;
        let mut noise_variable_476 = 0.0;
        let mut noise_variable_477 = 0.0;
        let mut noise_variable_478 = 0.0;
        let mut noise_variable_479 = 0.0;
        let mut noise_variable_480 = 0.0;
        let mut noise_variable_481 = 0.0;
        let mut noise_variable_482 = 0.0;
        let mut noise_variable_483 = 0.0;
        let mut noise_variable_484 = 0.0;
        let mut noise_variable_485 = 0.0;
        let mut noise_variable_486 = 0.0;
        let mut noise_variable_487 = 0.0;
        let mut noise_variable_488 = 0.0;
        let mut noise_variable_489 = 0.0;
        let mut noise_variable_490 = 0.0;
        let mut noise_variable_491 = 0.0;
        let mut noise_variable_492 = 0.0;
        let mut noise_variable_493 = 0.0;
        let mut noise_variable_494 = 0.0;
        let mut noise_variable_495 = 0.0;
        let mut noise_variable_496 = 0.0;
        let mut noise_variable_497 = 0.0;
        let mut noise_variable_498 = 0.0;
        let mut noise_variable_499 = 0.0;
        let mut noise_variable_500 = 0.0;
        let mut noise_variable_501 = 0.0;
        let mut noise_variable_502 = 0.0;
        let mut noise_variable_503 = 0.0;
        let mut noise_variable_504 = 0.0;
        let mut noise_variable_505 = 0.0;
        let mut noise_variable_506 = 0.0;
        let mut noise_variable_507 = 0.0;
        let mut noise_variable_508 = 0.0;
        let mut noise_variable_509 = 0.0;
        let mut noise_variable_510 = 0.0;
        let mut noise_variable_511 = 0.0;
        let mut noise_variable_512 = 0.0;
        let mut noise_variable_513 = 0.0;
        let mut noise_variable_514 = 0.0;
        let mut noise_variable_515 = 0.0;
        let mut noise_variable_516 = 0.0;
        let mut noise_variable_517 = 0.0;
        let mut noise_variable_518 = 0.0;
        let mut noise_variable_519 = 0.0;
        let mut noise_variable_520 = 0.0;
        let mut noise_variable_521 = 0.0;
        let mut noise_variable_522 = 0.0;
        let mut noise_variable_523 = 0.0;
        let mut noise_variable_524 = 0.0;
        let mut noise_variable_525 = 0.0;
        let mut noise_variable_526 = 0.0;
        let mut noise_variable_527 = 0.0;
        let mut noise_variable_528 = 0.0;
        let mut noise_variable_529 = 0.0;
        let mut noise_variable_530 = 0.0;
        let mut noise_variable_531 = 0.0;
        let mut noise_variable_532 = 0.0;
        let mut noise_variable_533 = 0.0;
        let mut noise_variable_534 = 0.0;
        let mut noise_variable_535 = 0.0;
        let mut noise_variable_536 = 0.0;
        let mut noise_variable_537 = 0.0;
        let mut noise_variable_538 = 0.0;
        let mut noise_variable_539 = 0.0;
        let mut noise_variable_540 = 0.0;
        let mut noise_variable_541 = 0.0;
        let mut noise_variable_542 = 0.0;
        let mut noise_variable_543 = 0.0;
        let mut noise_variable_544 = 0.0;
        let mut noise_variable_545 = 0.0;
        let mut noise_variable_546 = 0.0;
        let mut noise_variable_547 = 0.0;
        let mut noise_variable_548 = 0.0;
        let mut noise_variable_549 = 0.0;
        let mut noise_variable_550 = 0.0;
        let mut noise_variable_551 = 0.0;
        let mut noise_variable_552 = 0.0;
        let mut noise_variable_553 = 0.0;
        let mut noise_variable_554 = 0.0;
        let mut noise_variable_555 = 0.0;
        let mut noise_variable_556 = 0.0;
        let mut noise_variable_557 = 0.0;
        let mut noise_variable_558 = 0.0;
        let mut noise_variable_559 = 0.0;
        let mut noise_variable_560 = 0.0;
        let mut noise_variable_561 = 0.0;
        let mut noise_variable_562 = 0.0;
        let mut noise_variable_563 = 0.0;
        let mut noise_variable_564 = 0.0;
        let mut noise_variable_565 = 0.0;
        let mut noise_variable_566 = 0.0;
        let mut noise_variable_567 = 0.0;
        let mut noise_variable_568 = 0.0;
        let mut noise_variable_569 = 0.0;
        let mut noise_variable_570 = 0.0;
        let mut noise_variable_571 = 0.0;
        let mut noise_variable_572 = 0.0;
        let mut noise_variable_573 = 0.0;
        let mut noise_variable_574 = 0.0;
        let mut noise_variable_575 = 0.0;
        let mut noise_variable_576 = 0.0;
        let mut noise_variable_577 = 0.0;
        let mut noise_variable_578 = 0.0;
        let mut noise_variable_579 = 0.0;
        let mut noise_variable_580 = 0.0;
        let mut noise_variable_581 = 0.0;
        let mut noise_variable_582 = 0.0;
        let mut noise_variable_583 = 0.0;
        let mut noise_variable_584 = 0.0;
        let mut noise_variable_585 = 0.0;
        let mut noise_variable_586 = 0.0;
        let mut noise_variable_587 = 0.0;
        let mut noise_variable_588 = 0.0;
        let mut noise_variable_589 = 0.0;
        let mut noise_variable_590 = 0.0;
        let mut noise_variable_591 = 0.0;
        let mut noise_variable_592 = 0.0;
        let mut noise_variable_593 = 0.0;
        let mut noise_variable_594 = 0.0;
        let mut noise_variable_595 = 0.0;
        let mut noise_variable_596 = 0.0;
        let mut noise_variable_597 = 0.0;
        let mut noise_variable_598 = 0.0;
        let mut noise_variable_599 = 0.0;
        let mut noise_variable_600 = 0.0;
        let mut noise_variable_601 = 0.0;
        let mut noise_variable_602 = 0.0;
        let mut noise_variable_603 = 0.0;
        let mut noise_variable_604 = 0.0;
        let mut noise_variable_605 = 0.0;
        let mut noise_variable_606 = 0.0;
        let mut noise_variable_607 = 0.0;
        let mut noise_variable_608 = 0.0;
        let mut noise_variable_609 = 0.0;
        let mut noise_variable_610 = 0.0;
        let mut noise_variable_611 = 0.0;
        let mut noise_variable_612 = 0.0;
        let mut noise_variable_613 = 0.0;
        let mut noise_variable_614 = 0.0;
        let mut noise_variable_615 = 0.0;
        let mut noise_variable_616 = 0.0;
        let mut noise_variable_617 = 0.0;
        let mut noise_variable_618 = 0.0;
        let mut noise_variable_619 = 0.0;
        let mut noise_variable_620 = 0.0;
        let mut noise_variable_621 = 0.0;
        let mut noise_variable_622 = 0.0;
        let mut noise_variable_623 = 0.0;
        let mut noise_variable_624 = 0.0;
        let mut noise_variable_625 = 0.0;
        let mut noise_variable_626 = 0.0;
        let mut noise_variable_627 = 0.0;
        let mut noise_variable_628 = 0.0;
        let mut noise_variable_629 = 0.0;
        if matches!(source_index, 15 | 16) {
            let noise_activation_schedule_751_e7552: f64 = if params.p24 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_624 = noise_activation_schedule_751_e7552;
        }
        if matches!(source_index, 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_activation_schedule_752_e7555: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_625 = noise_activation_schedule_752_e7555;
        }
        if matches!(source_index, 20 | 21 | 22 | 23 | 24) {
            let noise_activation_schedule_753_e7558: f64 = if params.p59 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_626 = noise_activation_schedule_753_e7558;
        }
        if matches!(source_index, 25 | 26 | 27) {
            let noise_activation_schedule_754_e7561: f64 = if params.p59 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_627 = noise_activation_schedule_754_e7561;
        }
        let noise_source_active = match source_index {
            0 => {
                true
            }
            1 => {
                true
            }
            2 => {
                true
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
            6 => {
                true
            }
            7 => {
                true
            }
            8 => {
                true
            }
            9 => {
                true
            }
            10 => {
                true
            }
            11 => {
                true
            }
            12 => {
                true
            }
            13 => {
                true
            }
            14 => {
                true
            }
            15 => {
                noise_variable_624 != 0.0
            }
            16 => {
                let noise_16_activation_e496: f64 = if (noise_variable_624 == 0.0) { 1.0 } else { 0.0 };
                noise_16_activation_e496 != 0.0
            }
            17 => {
                true
            }
            18 => {
                true
            }
            19 => {
                true
            }
            20 => {
                let noise_20_activation_e521: f64 = if ((noise_variable_625 != 0.0) && (noise_variable_626 != 0.0)) { 1.0 } else { 0.0 };
                noise_20_activation_e521 != 0.0
            }
            21 => {
                let noise_21_activation_e531: f64 = if ((noise_variable_625 != 0.0) && (noise_variable_626 != 0.0)) { 1.0 } else { 0.0 };
                noise_21_activation_e531 != 0.0
            }
            22 => {
                let noise_22_activation_e541: f64 = if ((noise_variable_625 != 0.0) && (noise_variable_626 != 0.0)) { 1.0 } else { 0.0 };
                noise_22_activation_e541 != 0.0
            }
            23 => {
                let noise_23_activation_e552: f64 = if ((noise_variable_625 != 0.0) && (noise_variable_626 == 0.0)) { 1.0 } else { 0.0 };
                noise_23_activation_e552 != 0.0
            }
            24 => {
                let noise_24_activation_e563: f64 = if ((noise_variable_625 != 0.0) && (noise_variable_626 == 0.0)) { 1.0 } else { 0.0 };
                noise_24_activation_e563 != 0.0
            }
            25 => {
                let noise_25_activation_e574: f64 = if ((noise_variable_625 == 0.0) && (noise_variable_627 != 0.0)) { 1.0 } else { 0.0 };
                noise_25_activation_e574 != 0.0
            }
            26 => {
                let noise_26_activation_e585: f64 = if ((noise_variable_625 == 0.0) && (noise_variable_627 != 0.0)) { 1.0 } else { 0.0 };
                noise_26_activation_e585 != 0.0
            }
            27 => {
                let noise_27_activation_e597: f64 = if ((noise_variable_625 == 0.0) && (noise_variable_627 == 0.0)) { 1.0 } else { 0.0 };
                noise_27_activation_e597 != 0.0
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
        noise_variable_145 = 0.0;
        noise_variable_146 = 0.0;
        noise_variable_147 = 0.0;
        noise_variable_148 = 0.0;
        noise_variable_149 = 0.0;
        noise_variable_150 = 0.0;
        noise_variable_151 = 0.0;
        noise_variable_152 = 0.0;
        noise_variable_153 = 0.0;
        noise_variable_154 = 0.0;
        noise_variable_155 = 0.0;
        noise_variable_156 = 0.0;
        noise_variable_157 = 0.0;
        noise_variable_158 = 0.0;
        noise_variable_159 = 0.0;
        noise_variable_160 = 0.0;
        noise_variable_161 = 0.0;
        noise_variable_162 = 0.0;
        noise_variable_163 = 0.0;
        noise_variable_164 = 0.0;
        noise_variable_165 = 0.0;
        noise_variable_166 = 0.0;
        noise_variable_167 = 0.0;
        noise_variable_168 = 0.0;
        noise_variable_169 = 0.0;
        noise_variable_170 = 0.0;
        noise_variable_171 = 0.0;
        noise_variable_172 = 0.0;
        noise_variable_173 = 0.0;
        noise_variable_174 = 0.0;
        noise_variable_175 = 0.0;
        noise_variable_176 = 0.0;
        noise_variable_177 = 0.0;
        noise_variable_178 = 0.0;
        noise_variable_179 = 0.0;
        noise_variable_180 = 0.0;
        noise_variable_181 = 0.0;
        noise_variable_182 = 0.0;
        noise_variable_183 = 0.0;
        noise_variable_184 = 0.0;
        noise_variable_185 = 0.0;
        noise_variable_186 = 0.0;
        noise_variable_187 = 0.0;
        noise_variable_188 = 0.0;
        noise_variable_189 = 0.0;
        noise_variable_190 = 0.0;
        noise_variable_191 = 0.0;
        noise_variable_192 = 0.0;
        noise_variable_193 = 0.0;
        noise_variable_194 = 0.0;
        noise_variable_195 = 0.0;
        noise_variable_196 = 0.0;
        noise_variable_197 = 0.0;
        noise_variable_198 = 0.0;
        noise_variable_199 = 0.0;
        noise_variable_200 = 0.0;
        noise_variable_201 = 0.0;
        noise_variable_202 = 0.0;
        noise_variable_203 = 0.0;
        noise_variable_204 = 0.0;
        noise_variable_205 = 0.0;
        noise_variable_206 = 0.0;
        noise_variable_207 = 0.0;
        noise_variable_208 = 0.0;
        noise_variable_209 = 0.0;
        noise_variable_210 = 0.0;
        noise_variable_211 = 0.0;
        noise_variable_212 = 0.0;
        noise_variable_213 = 0.0;
        noise_variable_214 = 0.0;
        noise_variable_215 = 0.0;
        noise_variable_216 = 0.0;
        noise_variable_217 = 0.0;
        noise_variable_218 = 0.0;
        noise_variable_219 = 0.0;
        noise_variable_220 = 0.0;
        noise_variable_221 = 0.0;
        noise_variable_222 = 0.0;
        noise_variable_223 = 0.0;
        noise_variable_224 = 0.0;
        noise_variable_225 = 0.0;
        noise_variable_226 = 0.0;
        noise_variable_227 = 0.0;
        noise_variable_228 = 0.0;
        noise_variable_229 = 0.0;
        noise_variable_230 = 0.0;
        noise_variable_231 = 0.0;
        noise_variable_232 = 0.0;
        noise_variable_233 = 0.0;
        noise_variable_234 = 0.0;
        noise_variable_235 = 0.0;
        noise_variable_236 = 0.0;
        noise_variable_237 = 0.0;
        noise_variable_238 = 0.0;
        noise_variable_239 = 0.0;
        noise_variable_240 = 0.0;
        noise_variable_241 = 0.0;
        noise_variable_242 = 0.0;
        noise_variable_243 = 0.0;
        noise_variable_244 = 0.0;
        noise_variable_245 = 0.0;
        noise_variable_246 = 0.0;
        noise_variable_247 = 0.0;
        noise_variable_248 = 0.0;
        noise_variable_249 = 0.0;
        noise_variable_250 = 0.0;
        noise_variable_251 = 0.0;
        noise_variable_252 = 0.0;
        noise_variable_253 = 0.0;
        noise_variable_254 = 0.0;
        noise_variable_255 = 0.0;
        noise_variable_256 = 0.0;
        noise_variable_257 = 0.0;
        noise_variable_258 = 0.0;
        noise_variable_259 = 0.0;
        noise_variable_260 = 0.0;
        noise_variable_261 = 0.0;
        noise_variable_262 = 0.0;
        noise_variable_263 = 0.0;
        noise_variable_264 = 0.0;
        noise_variable_265 = 0.0;
        noise_variable_266 = 0.0;
        noise_variable_267 = 0.0;
        noise_variable_268 = 0.0;
        noise_variable_269 = 0.0;
        noise_variable_270 = 0.0;
        noise_variable_271 = 0.0;
        noise_variable_272 = 0.0;
        noise_variable_273 = 0.0;
        noise_variable_274 = 0.0;
        noise_variable_275 = 0.0;
        noise_variable_276 = 0.0;
        noise_variable_277 = 0.0;
        noise_variable_278 = 0.0;
        noise_variable_279 = 0.0;
        noise_variable_280 = 0.0;
        noise_variable_281 = 0.0;
        noise_variable_282 = 0.0;
        noise_variable_283 = 0.0;
        noise_variable_284 = 0.0;
        noise_variable_285 = 0.0;
        noise_variable_286 = 0.0;
        noise_variable_287 = 0.0;
        noise_variable_288 = 0.0;
        noise_variable_289 = 0.0;
        noise_variable_290 = 0.0;
        noise_variable_291 = 0.0;
        noise_variable_292 = 0.0;
        noise_variable_293 = 0.0;
        noise_variable_294 = 0.0;
        noise_variable_295 = 0.0;
        noise_variable_296 = 0.0;
        noise_variable_297 = 0.0;
        noise_variable_298 = 0.0;
        noise_variable_299 = 0.0;
        noise_variable_300 = 0.0;
        noise_variable_301 = 0.0;
        noise_variable_302 = 0.0;
        noise_variable_303 = 0.0;
        noise_variable_304 = 0.0;
        noise_variable_305 = 0.0;
        noise_variable_306 = 0.0;
        noise_variable_307 = 0.0;
        noise_variable_308 = 0.0;
        noise_variable_309 = 0.0;
        noise_variable_310 = 0.0;
        noise_variable_311 = 0.0;
        noise_variable_312 = 0.0;
        noise_variable_313 = 0.0;
        noise_variable_314 = 0.0;
        noise_variable_315 = 0.0;
        noise_variable_316 = 0.0;
        noise_variable_317 = 0.0;
        noise_variable_318 = 0.0;
        noise_variable_319 = 0.0;
        noise_variable_320 = 0.0;
        noise_variable_321 = 0.0;
        noise_variable_322 = 0.0;
        noise_variable_323 = 0.0;
        noise_variable_324 = 0.0;
        noise_variable_325 = 0.0;
        noise_variable_326 = 0.0;
        noise_variable_327 = 0.0;
        noise_variable_328 = 0.0;
        noise_variable_329 = 0.0;
        noise_variable_330 = 0.0;
        noise_variable_331 = 0.0;
        noise_variable_332 = 0.0;
        noise_variable_333 = 0.0;
        noise_variable_334 = 0.0;
        noise_variable_335 = 0.0;
        noise_variable_336 = 0.0;
        noise_variable_337 = 0.0;
        noise_variable_338 = 0.0;
        noise_variable_339 = 0.0;
        noise_variable_340 = 0.0;
        noise_variable_341 = 0.0;
        noise_variable_342 = 0.0;
        noise_variable_343 = 0.0;
        noise_variable_344 = 0.0;
        noise_variable_345 = 0.0;
        noise_variable_346 = 0.0;
        noise_variable_347 = 0.0;
        noise_variable_348 = 0.0;
        noise_variable_349 = 0.0;
        noise_variable_350 = 0.0;
        noise_variable_351 = 0.0;
        noise_variable_352 = 0.0;
        noise_variable_353 = 0.0;
        noise_variable_354 = 0.0;
        noise_variable_355 = 0.0;
        noise_variable_356 = 0.0;
        noise_variable_357 = 0.0;
        noise_variable_358 = 0.0;
        noise_variable_359 = 0.0;
        noise_variable_360 = 0.0;
        noise_variable_361 = 0.0;
        noise_variable_362 = 0.0;
        noise_variable_363 = 0.0;
        noise_variable_364 = 0.0;
        noise_variable_365 = 0.0;
        noise_variable_366 = 0.0;
        noise_variable_367 = 0.0;
        noise_variable_368 = 0.0;
        noise_variable_369 = 0.0;
        noise_variable_370 = 0.0;
        noise_variable_371 = 0.0;
        noise_variable_372 = 0.0;
        noise_variable_373 = 0.0;
        noise_variable_374 = 0.0;
        noise_variable_375 = 0.0;
        noise_variable_376 = 0.0;
        noise_variable_377 = 0.0;
        noise_variable_378 = 0.0;
        noise_variable_379 = 0.0;
        noise_variable_380 = 0.0;
        noise_variable_381 = 0.0;
        noise_variable_382 = 0.0;
        noise_variable_383 = 0.0;
        noise_variable_384 = 0.0;
        noise_variable_385 = 0.0;
        noise_variable_386 = 0.0;
        noise_variable_387 = 0.0;
        noise_variable_388 = 0.0;
        noise_variable_389 = 0.0;
        noise_variable_390 = 0.0;
        noise_variable_391 = 0.0;
        noise_variable_392 = 0.0;
        noise_variable_393 = 0.0;
        noise_variable_394 = 0.0;
        noise_variable_395 = 0.0;
        noise_variable_396 = 0.0;
        noise_variable_397 = 0.0;
        noise_variable_398 = 0.0;
        noise_variable_399 = 0.0;
        noise_variable_400 = 0.0;
        noise_variable_401 = 0.0;
        noise_variable_402 = 0.0;
        noise_variable_403 = 0.0;
        noise_variable_404 = 0.0;
        noise_variable_405 = 0.0;
        noise_variable_406 = 0.0;
        noise_variable_407 = 0.0;
        noise_variable_408 = 0.0;
        noise_variable_409 = 0.0;
        noise_variable_410 = 0.0;
        noise_variable_411 = 0.0;
        noise_variable_412 = 0.0;
        noise_variable_413 = 0.0;
        noise_variable_414 = 0.0;
        noise_variable_415 = 0.0;
        noise_variable_416 = 0.0;
        noise_variable_417 = 0.0;
        noise_variable_418 = 0.0;
        noise_variable_419 = 0.0;
        noise_variable_420 = 0.0;
        noise_variable_421 = 0.0;
        noise_variable_422 = 0.0;
        noise_variable_423 = 0.0;
        noise_variable_424 = 0.0;
        noise_variable_425 = 0.0;
        noise_variable_426 = 0.0;
        noise_variable_427 = 0.0;
        noise_variable_428 = 0.0;
        noise_variable_429 = 0.0;
        noise_variable_430 = 0.0;
        noise_variable_431 = 0.0;
        noise_variable_432 = 0.0;
        noise_variable_433 = 0.0;
        noise_variable_434 = 0.0;
        noise_variable_435 = 0.0;
        noise_variable_436 = 0.0;
        noise_variable_437 = 0.0;
        noise_variable_438 = 0.0;
        noise_variable_439 = 0.0;
        noise_variable_440 = 0.0;
        noise_variable_441 = 0.0;
        noise_variable_442 = 0.0;
        noise_variable_443 = 0.0;
        noise_variable_444 = 0.0;
        noise_variable_445 = 0.0;
        noise_variable_446 = 0.0;
        noise_variable_447 = 0.0;
        noise_variable_448 = 0.0;
        noise_variable_449 = 0.0;
        noise_variable_450 = 0.0;
        noise_variable_451 = 0.0;
        noise_variable_452 = 0.0;
        noise_variable_453 = 0.0;
        noise_variable_454 = 0.0;
        noise_variable_455 = 0.0;
        noise_variable_456 = 0.0;
        noise_variable_457 = 0.0;
        noise_variable_458 = 0.0;
        noise_variable_459 = 0.0;
        noise_variable_460 = 0.0;
        noise_variable_461 = 0.0;
        noise_variable_462 = 0.0;
        noise_variable_463 = 0.0;
        noise_variable_464 = 0.0;
        noise_variable_465 = 0.0;
        noise_variable_466 = 0.0;
        noise_variable_467 = 0.0;
        noise_variable_468 = 0.0;
        noise_variable_469 = 0.0;
        noise_variable_470 = 0.0;
        noise_variable_471 = 0.0;
        noise_variable_472 = 0.0;
        noise_variable_473 = 0.0;
        noise_variable_474 = 0.0;
        noise_variable_475 = 0.0;
        noise_variable_476 = 0.0;
        noise_variable_477 = 0.0;
        noise_variable_478 = 0.0;
        noise_variable_479 = 0.0;
        noise_variable_480 = 0.0;
        noise_variable_481 = 0.0;
        noise_variable_482 = 0.0;
        noise_variable_483 = 0.0;
        noise_variable_484 = 0.0;
        noise_variable_485 = 0.0;
        noise_variable_486 = 0.0;
        noise_variable_487 = 0.0;
        noise_variable_488 = 0.0;
        noise_variable_489 = 0.0;
        noise_variable_490 = 0.0;
        noise_variable_491 = 0.0;
        noise_variable_492 = 0.0;
        noise_variable_493 = 0.0;
        noise_variable_494 = 0.0;
        noise_variable_495 = 0.0;
        noise_variable_496 = 0.0;
        noise_variable_497 = 0.0;
        noise_variable_498 = 0.0;
        noise_variable_499 = 0.0;
        noise_variable_500 = 0.0;
        noise_variable_501 = 0.0;
        noise_variable_502 = 0.0;
        noise_variable_503 = 0.0;
        noise_variable_504 = 0.0;
        noise_variable_505 = 0.0;
        noise_variable_506 = 0.0;
        noise_variable_507 = 0.0;
        noise_variable_508 = 0.0;
        noise_variable_509 = 0.0;
        noise_variable_510 = 0.0;
        noise_variable_511 = 0.0;
        noise_variable_512 = 0.0;
        noise_variable_513 = 0.0;
        noise_variable_514 = 0.0;
        noise_variable_515 = 0.0;
        noise_variable_516 = 0.0;
        noise_variable_517 = 0.0;
        noise_variable_518 = 0.0;
        noise_variable_519 = 0.0;
        noise_variable_520 = 0.0;
        noise_variable_521 = 0.0;
        noise_variable_522 = 0.0;
        noise_variable_523 = 0.0;
        noise_variable_524 = 0.0;
        noise_variable_525 = 0.0;
        noise_variable_526 = 0.0;
        noise_variable_527 = 0.0;
        noise_variable_528 = 0.0;
        noise_variable_529 = 0.0;
        noise_variable_530 = 0.0;
        noise_variable_531 = 0.0;
        noise_variable_532 = 0.0;
        noise_variable_533 = 0.0;
        noise_variable_534 = 0.0;
        noise_variable_535 = 0.0;
        noise_variable_536 = 0.0;
        noise_variable_537 = 0.0;
        noise_variable_538 = 0.0;
        noise_variable_539 = 0.0;
        noise_variable_540 = 0.0;
        noise_variable_541 = 0.0;
        noise_variable_542 = 0.0;
        noise_variable_543 = 0.0;
        noise_variable_544 = 0.0;
        noise_variable_545 = 0.0;
        noise_variable_546 = 0.0;
        noise_variable_547 = 0.0;
        noise_variable_548 = 0.0;
        noise_variable_549 = 0.0;
        noise_variable_550 = 0.0;
        noise_variable_551 = 0.0;
        noise_variable_552 = 0.0;
        noise_variable_553 = 0.0;
        noise_variable_554 = 0.0;
        noise_variable_555 = 0.0;
        noise_variable_556 = 0.0;
        noise_variable_557 = 0.0;
        noise_variable_558 = 0.0;
        noise_variable_559 = 0.0;
        noise_variable_560 = 0.0;
        noise_variable_561 = 0.0;
        noise_variable_562 = 0.0;
        noise_variable_563 = 0.0;
        noise_variable_564 = 0.0;
        noise_variable_565 = 0.0;
        noise_variable_566 = 0.0;
        noise_variable_567 = 0.0;
        noise_variable_568 = 0.0;
        noise_variable_569 = 0.0;
        noise_variable_570 = 0.0;
        noise_variable_571 = 0.0;
        noise_variable_572 = 0.0;
        noise_variable_573 = 0.0;
        noise_variable_574 = 0.0;
        noise_variable_575 = 0.0;
        noise_variable_576 = 0.0;
        noise_variable_577 = 0.0;
        noise_variable_578 = 0.0;
        noise_variable_579 = 0.0;
        noise_variable_580 = 0.0;
        noise_variable_581 = 0.0;
        noise_variable_582 = 0.0;
        noise_variable_583 = 0.0;
        noise_variable_584 = 0.0;
        noise_variable_585 = 0.0;
        noise_variable_586 = 0.0;
        noise_variable_587 = 0.0;
        noise_variable_588 = 0.0;
        noise_variable_589 = 0.0;
        noise_variable_590 = 0.0;
        noise_variable_591 = 0.0;
        noise_variable_592 = 0.0;
        noise_variable_593 = 0.0;
        noise_variable_594 = 0.0;
        noise_variable_595 = 0.0;
        noise_variable_596 = 0.0;
        noise_variable_597 = 0.0;
        noise_variable_598 = 0.0;
        noise_variable_599 = 0.0;
        noise_variable_600 = 0.0;
        noise_variable_601 = 0.0;
        noise_variable_602 = 0.0;
        noise_variable_603 = 0.0;
        noise_variable_604 = 0.0;
        noise_variable_605 = 0.0;
        noise_variable_606 = 0.0;
        noise_variable_607 = 0.0;
        noise_variable_608 = 0.0;
        noise_variable_609 = 0.0;
        noise_variable_610 = 0.0;
        noise_variable_611 = 0.0;
        noise_variable_612 = 0.0;
        noise_variable_613 = 0.0;
        noise_variable_614 = 0.0;
        noise_variable_615 = 0.0;
        noise_variable_616 = 0.0;
        noise_variable_617 = 0.0;
        noise_variable_618 = 0.0;
        noise_variable_619 = 0.0;
        noise_variable_620 = 0.0;
        noise_variable_621 = 0.0;
        noise_variable_622 = 0.0;
        noise_variable_623 = 0.0;
        noise_variable_624 = 0.0;
        noise_variable_625 = 0.0;
        noise_variable_626 = 0.0;
        noise_variable_627 = 0.0;
        noise_variable_628 = 0.0;
        noise_variable_629 = 0.0;
        if matches!(source_index, 1) {
            let noise_metadata_schedule_0_e607: f64 = if params.p3 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_484 = noise_metadata_schedule_0_e607;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_1_e611,) = {
    if (noise_variable_484 != 0.0) {
        (70300000.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_1_e611;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_2_e615,) = {
    if (noise_variable_484 != 0.0) {
        (123000000.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_2_e615;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_3_e620,) = {
    if (noise_variable_484 == 0.0) {
        (158000000.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_3_e620;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_4_e625,) = {
    if (noise_variable_484 == 0.0) {
        (204000000.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_4_e625;
        }
        if matches!(source_index, 11 | 12 | 18) {
            let noise_metadata_schedule_5_e628: f64 = (1.0 - params.p33);
            noise_variable_160 = noise_metadata_schedule_5_e628;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_6_e631: f64 = (params.p4 + 273.15);
            noise_variable_3 = noise_metadata_schedule_6_e631;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_7_e632: f64 = ctx.temperature();
            let noise_metadata_schedule_7_e634: f64 = (noise_metadata_schedule_7_e632 + params.p0);
            noise_variable_5 = noise_metadata_schedule_7_e634;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_9_e640: f64 = if params.p154 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_485 = noise_metadata_schedule_9_e640;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let (noise_metadata_schedule_10_e644,) = {
    if (noise_variable_485 != 0.0) {
        (1e-12,)
    } else {
        (noise_variable_345,)
    }
};
            noise_variable_345 = noise_metadata_schedule_10_e644;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let (noise_metadata_schedule_11_e649,) = {
    if (noise_variable_485 == 0.0) {
        (params.p154,)
    } else {
        (noise_variable_345,)
    }
};
            noise_variable_345 = noise_metadata_schedule_11_e649;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_12_e652: f64 = (noise_variable_345 * params.p1);
            noise_variable_346 = noise_metadata_schedule_12_e652;
        }
        if matches!(source_index, 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_13_e655: f64 = (1.0 / noise_variable_346);
            noise_variable_347 = noise_metadata_schedule_13_e655;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            noise_variable_52 = 0.001;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            noise_variable_342 = 0.001;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_19_e673: f64 = (2.0 - params.p67);
            let noise_metadata_schedule_19_e674: f64 = (2.0_f64).powf(noise_metadata_schedule_19_e673);
            noise_variable_62 = noise_metadata_schedule_19_e674;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_20_e677: f64 = (1.0 / noise_variable_62);
            noise_variable_63 = noise_metadata_schedule_20_e677;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_21_e681: f64 = (params.p115 * noise_variable_3);
            let noise_metadata_schedule_21_e683: f64 = (noise_metadata_schedule_21_e681 * noise_variable_3);
            let noise_metadata_schedule_21_e686: f64 = (noise_variable_3 + params.p116);
            let noise_metadata_schedule_21_e687: f64 = (noise_metadata_schedule_21_e683 / noise_metadata_schedule_21_e686);
            let noise_metadata_schedule_21_e688: f64 = (params.p114 + noise_metadata_schedule_21_e687);
            let noise_metadata_schedule_21_e690: f64 = (noise_metadata_schedule_21_e688 - 0.05);
            let noise_metadata_schedule_21_e692: f64 = (noise_metadata_schedule_21_e690 / 0.1);
            noise_variable_285 = noise_metadata_schedule_21_e692;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_22_e696: f64 = (params.p115 * noise_variable_3);
            let noise_metadata_schedule_22_e698: f64 = (noise_metadata_schedule_22_e696 * noise_variable_3);
            let noise_metadata_schedule_22_e701: f64 = (noise_variable_3 + params.p116);
            let noise_metadata_schedule_22_e702: f64 = (noise_metadata_schedule_22_e698 / noise_metadata_schedule_22_e701);
            let noise_metadata_schedule_22_e703: f64 = (params.p114 + noise_metadata_schedule_22_e702);
            let noise_metadata_schedule_22_e705: f64 = if noise_metadata_schedule_22_e703 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_487 = noise_metadata_schedule_22_e705;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_23_e717,) = {
    if (noise_variable_487 != 0.0) {
        let noise_metadata_schedule_23_e711: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_23_e712: f64 = (1.0 + noise_metadata_schedule_23_e711);
        let noise_metadata_schedule_23_e713: f64 = (noise_metadata_schedule_23_e712).ln();
        let noise_metadata_schedule_23_e714: f64 = (0.1 * noise_metadata_schedule_23_e713);
        let noise_metadata_schedule_23_e715: f64 = (0.05 + noise_metadata_schedule_23_e714);
        (noise_metadata_schedule_23_e715,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_23_e717;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_24_e741,) = {
    if (noise_variable_487 == 0.0) {
        let noise_metadata_schedule_24_e723: f64 = (params.p115 * noise_variable_3);
        let noise_metadata_schedule_24_e725: f64 = (noise_metadata_schedule_24_e723 * noise_variable_3);
        let noise_metadata_schedule_24_e728: f64 = (noise_variable_3 + params.p116);
        let noise_metadata_schedule_24_e729: f64 = (noise_metadata_schedule_24_e725 / noise_metadata_schedule_24_e728);
        let noise_metadata_schedule_24_e730: f64 = (params.p114 + noise_metadata_schedule_24_e729);
        let noise_metadata_schedule_24_e734: f64 = (-noise_variable_285);
        let noise_metadata_schedule_24_e735: f64 = (noise_metadata_schedule_24_e734).exp();
        let noise_metadata_schedule_24_e736: f64 = (1.0 + noise_metadata_schedule_24_e735);
        let noise_metadata_schedule_24_e737: f64 = (noise_metadata_schedule_24_e736).ln();
        let noise_metadata_schedule_24_e738: f64 = (0.1 * noise_metadata_schedule_24_e737);
        let noise_metadata_schedule_24_e739: f64 = (noise_metadata_schedule_24_e730 + noise_metadata_schedule_24_e738);
        (noise_metadata_schedule_24_e739,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_24_e741;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            noise_variable_71 = params.p114;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_26_e745: f64 = (1.0 / noise_variable_71);
            noise_variable_72 = noise_metadata_schedule_26_e745;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_27_e748: f64 = (1.0 / params.p66);
            noise_variable_64 = noise_metadata_schedule_27_e748;
        }
        if matches!(source_index, 1 | 15 | 16) {
            noise_variable_75 = params.p71;
        }
        if matches!(source_index, 1 | 15 | 16) {
            noise_variable_76 = params.p72;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_30_e754: f64 = (2.0 - noise_variable_76);
            let noise_metadata_schedule_30_e755: f64 = (2.0_f64).powf(noise_metadata_schedule_30_e754);
            noise_variable_79 = noise_metadata_schedule_30_e755;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_31_e758: f64 = (1.0 / noise_variable_79);
            noise_variable_89 = noise_metadata_schedule_31_e758;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_32_e762: f64 = (params.p118 * noise_variable_3);
            let noise_metadata_schedule_32_e764: f64 = (noise_metadata_schedule_32_e762 * noise_variable_3);
            let noise_metadata_schedule_32_e767: f64 = (noise_variable_3 + params.p119);
            let noise_metadata_schedule_32_e768: f64 = (noise_metadata_schedule_32_e764 / noise_metadata_schedule_32_e767);
            let noise_metadata_schedule_32_e769: f64 = (params.p117 + noise_metadata_schedule_32_e768);
            let noise_metadata_schedule_32_e771: f64 = (noise_metadata_schedule_32_e769 - 0.05);
            let noise_metadata_schedule_32_e773: f64 = (noise_metadata_schedule_32_e771 / 0.1);
            noise_variable_285 = noise_metadata_schedule_32_e773;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_33_e777: f64 = (params.p118 * noise_variable_3);
            let noise_metadata_schedule_33_e779: f64 = (noise_metadata_schedule_33_e777 * noise_variable_3);
            let noise_metadata_schedule_33_e782: f64 = (noise_variable_3 + params.p119);
            let noise_metadata_schedule_33_e783: f64 = (noise_metadata_schedule_33_e779 / noise_metadata_schedule_33_e782);
            let noise_metadata_schedule_33_e784: f64 = (params.p117 + noise_metadata_schedule_33_e783);
            let noise_metadata_schedule_33_e786: f64 = if noise_metadata_schedule_33_e784 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_488 = noise_metadata_schedule_33_e786;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_34_e798,) = {
    if (noise_variable_488 != 0.0) {
        let noise_metadata_schedule_34_e792: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_34_e793: f64 = (1.0 + noise_metadata_schedule_34_e792);
        let noise_metadata_schedule_34_e794: f64 = (noise_metadata_schedule_34_e793).ln();
        let noise_metadata_schedule_34_e795: f64 = (0.1 * noise_metadata_schedule_34_e794);
        let noise_metadata_schedule_34_e796: f64 = (0.05 + noise_metadata_schedule_34_e795);
        (noise_metadata_schedule_34_e796,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_34_e798;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_35_e822,) = {
    if (noise_variable_488 == 0.0) {
        let noise_metadata_schedule_35_e804: f64 = (params.p118 * noise_variable_3);
        let noise_metadata_schedule_35_e806: f64 = (noise_metadata_schedule_35_e804 * noise_variable_3);
        let noise_metadata_schedule_35_e809: f64 = (noise_variable_3 + params.p119);
        let noise_metadata_schedule_35_e810: f64 = (noise_metadata_schedule_35_e806 / noise_metadata_schedule_35_e809);
        let noise_metadata_schedule_35_e811: f64 = (params.p117 + noise_metadata_schedule_35_e810);
        let noise_metadata_schedule_35_e815: f64 = (-noise_variable_285);
        let noise_metadata_schedule_35_e816: f64 = (noise_metadata_schedule_35_e815).exp();
        let noise_metadata_schedule_35_e817: f64 = (1.0 + noise_metadata_schedule_35_e816);
        let noise_metadata_schedule_35_e818: f64 = (noise_metadata_schedule_35_e817).ln();
        let noise_metadata_schedule_35_e819: f64 = (0.1 * noise_metadata_schedule_35_e818);
        let noise_metadata_schedule_35_e820: f64 = (noise_metadata_schedule_35_e811 + noise_metadata_schedule_35_e819);
        (noise_metadata_schedule_35_e820,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_35_e822;
        }
        if matches!(source_index, 1 | 15 | 16) {
            noise_variable_87 = params.p117;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_37_e826: f64 = (1.0 / noise_variable_87);
            noise_variable_86 = noise_metadata_schedule_37_e826;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_38_e829: f64 = (1.0 / noise_variable_75);
            noise_variable_66 = noise_metadata_schedule_38_e829;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_39_e833: f64 = (1.0 / params.p83);
            let noise_metadata_schedule_39_e834: f64 = (1.0 - noise_metadata_schedule_39_e833);
            noise_variable_349 = noise_metadata_schedule_39_e834;
        }
        if matches!(source_index, 2 | 6) {
            noise_variable_161 = 0.0;
        }
        if matches!(source_index, 6 | 8) {
            noise_variable_162 = 0.0;
        }
        if matches!(source_index, 13 | 14) {
            noise_variable_179 = 0.0;
        }
        if matches!(source_index, 13 | 14 | 19) {
            noise_variable_178 = 1.0;
        }
        if matches!(source_index, 1) {
            noise_variable_210 = 0.0;
        }
        if matches!(source_index, 1) {
            noise_variable_212 = 0.0;
        }
        if matches!(source_index, 2 | 6) {
            noise_variable_53 = 0.0;
        }
        if matches!(source_index, 2 | 6) {
            noise_variable_54 = 0.0;
        }
        if matches!(source_index, 6 | 8) {
            noise_variable_45 = 0.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            noise_variable_218 = (ctx.node_voltage(self.nodes[4]) - 0.0);
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_54_e851: f64 = if noise_variable_218 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_489 = noise_metadata_schedule_54_e851;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let (noise_metadata_schedule_55_e859,) = {
    if (noise_variable_489 != 0.0) {
        let noise_metadata_schedule_55_e855: f64 = (1.0 - noise_variable_218);
        let noise_metadata_schedule_55_e856: f64 = (noise_metadata_schedule_55_e855).ln();
        let noise_metadata_schedule_55_e857: f64 = (-noise_metadata_schedule_55_e856);
        (noise_metadata_schedule_55_e857,)
    } else {
        (noise_variable_218,)
    }
};
            noise_variable_218 = noise_metadata_schedule_55_e859;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_56_e862: f64 = if noise_variable_218 < params.p125 { 1.0 } else { 0.0 };
            noise_variable_490 = noise_metadata_schedule_56_e862;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let (noise_metadata_schedule_57_e866,) = {
    if (noise_variable_490 != 0.0) {
        (noise_variable_218,)
    } else {
        (noise_variable_11,)
    }
};
            noise_variable_11 = noise_metadata_schedule_57_e866;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let (noise_metadata_schedule_58_e878,) = {
    if (noise_variable_490 == 0.0) {
        let noise_metadata_schedule_58_e873: f64 = (noise_variable_218 - params.p125);
        let noise_metadata_schedule_58_e874: f64 = (1.0 + noise_metadata_schedule_58_e873);
        let noise_metadata_schedule_58_e875: f64 = (noise_metadata_schedule_58_e874).ln();
        let noise_metadata_schedule_58_e876: f64 = (params.p125 + noise_metadata_schedule_58_e875);
        (noise_metadata_schedule_58_e876,)
    } else {
        (noise_variable_11,)
    }
};
            noise_variable_11 = noise_metadata_schedule_58_e878;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_59_e881: f64 = (noise_variable_5 + noise_variable_11);
            noise_variable_2 = noise_metadata_schedule_59_e881;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_60_e884: f64 = (noise_variable_2 / noise_variable_3);
            noise_variable_4 = noise_metadata_schedule_60_e884;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_61_e887: f64 = (8.617086918058125e-5 * noise_variable_2);
            noise_variable_6 = noise_metadata_schedule_61_e887;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_62_e890: f64 = (8.617086918058125e-5 * noise_variable_3);
            noise_variable_7 = noise_metadata_schedule_62_e890;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_63_e893: f64 = (1.0 / noise_variable_6);
            noise_variable_8 = noise_metadata_schedule_63_e893;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_64_e896: f64 = (1.0 / noise_variable_7);
            noise_variable_9 = noise_metadata_schedule_64_e896;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_65_e899: f64 = (noise_variable_8 - noise_variable_9);
            noise_variable_10 = noise_metadata_schedule_65_e899;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_66_e902: f64 = (noise_variable_2 - noise_variable_3);
            noise_variable_12 = noise_metadata_schedule_66_e902;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_67_e904: f64 = (noise_variable_4).ln();
            noise_variable_280 = noise_metadata_schedule_67_e904;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_68_e908: f64 = (params.p115 * noise_variable_2);
            let noise_metadata_schedule_68_e910: f64 = (noise_metadata_schedule_68_e908 * noise_variable_2);
            let noise_metadata_schedule_68_e913: f64 = (noise_variable_2 + params.p116);
            let noise_metadata_schedule_68_e914: f64 = (noise_metadata_schedule_68_e910 / noise_metadata_schedule_68_e913);
            let noise_metadata_schedule_68_e915: f64 = (noise_variable_74 - noise_metadata_schedule_68_e914);
            let noise_metadata_schedule_68_e917: f64 = (noise_metadata_schedule_68_e915 - 0.05);
            let noise_metadata_schedule_68_e919: f64 = (noise_metadata_schedule_68_e917 / 0.1);
            noise_variable_285 = noise_metadata_schedule_68_e919;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_69_e923: f64 = (params.p115 * noise_variable_2);
            let noise_metadata_schedule_69_e925: f64 = (noise_metadata_schedule_69_e923 * noise_variable_2);
            let noise_metadata_schedule_69_e928: f64 = (noise_variable_2 + params.p116);
            let noise_metadata_schedule_69_e929: f64 = (noise_metadata_schedule_69_e925 / noise_metadata_schedule_69_e928);
            let noise_metadata_schedule_69_e930: f64 = (noise_variable_74 - noise_metadata_schedule_69_e929);
            let noise_metadata_schedule_69_e932: f64 = if noise_metadata_schedule_69_e930 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_491 = noise_metadata_schedule_69_e932;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_70_e944,) = {
    if (noise_variable_491 != 0.0) {
        let noise_metadata_schedule_70_e938: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_70_e939: f64 = (1.0 + noise_metadata_schedule_70_e938);
        let noise_metadata_schedule_70_e940: f64 = (noise_metadata_schedule_70_e939).ln();
        let noise_metadata_schedule_70_e941: f64 = (0.1 * noise_metadata_schedule_70_e940);
        let noise_metadata_schedule_70_e942: f64 = (0.05 + noise_metadata_schedule_70_e941);
        (noise_metadata_schedule_70_e942,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_70_e944;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_71_e968,) = {
    if (noise_variable_491 == 0.0) {
        let noise_metadata_schedule_71_e950: f64 = (params.p115 * noise_variable_2);
        let noise_metadata_schedule_71_e952: f64 = (noise_metadata_schedule_71_e950 * noise_variable_2);
        let noise_metadata_schedule_71_e955: f64 = (noise_variable_2 + params.p116);
        let noise_metadata_schedule_71_e956: f64 = (noise_metadata_schedule_71_e952 / noise_metadata_schedule_71_e955);
        let noise_metadata_schedule_71_e957: f64 = (noise_variable_74 - noise_metadata_schedule_71_e956);
        let noise_metadata_schedule_71_e961: f64 = (-noise_variable_285);
        let noise_metadata_schedule_71_e962: f64 = (noise_metadata_schedule_71_e961).exp();
        let noise_metadata_schedule_71_e963: f64 = (1.0 + noise_metadata_schedule_71_e962);
        let noise_metadata_schedule_71_e964: f64 = (noise_metadata_schedule_71_e963).ln();
        let noise_metadata_schedule_71_e965: f64 = (0.1 * noise_metadata_schedule_71_e964);
        let noise_metadata_schedule_71_e966: f64 = (noise_metadata_schedule_71_e957 + noise_metadata_schedule_71_e965);
        (noise_metadata_schedule_71_e966,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_71_e968;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_72_e972: f64 = (params.p118 * noise_variable_2);
            let noise_metadata_schedule_72_e974: f64 = (noise_metadata_schedule_72_e972 * noise_variable_2);
            let noise_metadata_schedule_72_e977: f64 = (noise_variable_2 + params.p119);
            let noise_metadata_schedule_72_e978: f64 = (noise_metadata_schedule_72_e974 / noise_metadata_schedule_72_e977);
            let noise_metadata_schedule_72_e979: f64 = (noise_variable_88 - noise_metadata_schedule_72_e978);
            let noise_metadata_schedule_72_e981: f64 = (noise_metadata_schedule_72_e979 - 0.05);
            let noise_metadata_schedule_72_e983: f64 = (noise_metadata_schedule_72_e981 / 0.1);
            noise_variable_285 = noise_metadata_schedule_72_e983;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_73_e987: f64 = (params.p118 * noise_variable_2);
            let noise_metadata_schedule_73_e989: f64 = (noise_metadata_schedule_73_e987 * noise_variable_2);
            let noise_metadata_schedule_73_e992: f64 = (noise_variable_2 + params.p119);
            let noise_metadata_schedule_73_e993: f64 = (noise_metadata_schedule_73_e989 / noise_metadata_schedule_73_e992);
            let noise_metadata_schedule_73_e994: f64 = (noise_variable_88 - noise_metadata_schedule_73_e993);
            let noise_metadata_schedule_73_e996: f64 = if noise_metadata_schedule_73_e994 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_492 = noise_metadata_schedule_73_e996;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_74_e1008,) = {
    if (noise_variable_492 != 0.0) {
        let noise_metadata_schedule_74_e1002: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_74_e1003: f64 = (1.0 + noise_metadata_schedule_74_e1002);
        let noise_metadata_schedule_74_e1004: f64 = (noise_metadata_schedule_74_e1003).ln();
        let noise_metadata_schedule_74_e1005: f64 = (0.1 * noise_metadata_schedule_74_e1004);
        let noise_metadata_schedule_74_e1006: f64 = (0.05 + noise_metadata_schedule_74_e1005);
        (noise_metadata_schedule_74_e1006,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_74_e1008;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_75_e1032,) = {
    if (noise_variable_492 == 0.0) {
        let noise_metadata_schedule_75_e1014: f64 = (params.p118 * noise_variable_2);
        let noise_metadata_schedule_75_e1016: f64 = (noise_metadata_schedule_75_e1014 * noise_variable_2);
        let noise_metadata_schedule_75_e1019: f64 = (noise_variable_2 + params.p119);
        let noise_metadata_schedule_75_e1020: f64 = (noise_metadata_schedule_75_e1016 / noise_metadata_schedule_75_e1019);
        let noise_metadata_schedule_75_e1021: f64 = (noise_variable_88 - noise_metadata_schedule_75_e1020);
        let noise_metadata_schedule_75_e1025: f64 = (-noise_variable_285);
        let noise_metadata_schedule_75_e1026: f64 = (noise_metadata_schedule_75_e1025).exp();
        let noise_metadata_schedule_75_e1027: f64 = (1.0 + noise_metadata_schedule_75_e1026);
        let noise_metadata_schedule_75_e1028: f64 = (noise_metadata_schedule_75_e1027).ln();
        let noise_metadata_schedule_75_e1029: f64 = (0.1 * noise_metadata_schedule_75_e1028);
        let noise_metadata_schedule_75_e1030: f64 = (noise_metadata_schedule_75_e1021 + noise_metadata_schedule_75_e1029);
        (noise_metadata_schedule_75_e1030,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_75_e1032;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_76_e1034: f64 = (-3.0);
            let noise_metadata_schedule_76_e1036: f64 = (noise_metadata_schedule_76_e1034 * noise_variable_6);
            let noise_metadata_schedule_76_e1038: f64 = (noise_metadata_schedule_76_e1036 * noise_variable_280);
            let noise_metadata_schedule_76_e1041: f64 = (params.p66 * noise_variable_4);
            let noise_metadata_schedule_76_e1042: f64 = (noise_metadata_schedule_76_e1038 + noise_metadata_schedule_76_e1041);
            let noise_metadata_schedule_76_e1045: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_76_e1047: f64 = (noise_metadata_schedule_76_e1045 * params.p105);
            let noise_metadata_schedule_76_e1048: f64 = (noise_metadata_schedule_76_e1042 + noise_metadata_schedule_76_e1047);
            noise_variable_13 = noise_metadata_schedule_76_e1048;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_77_e1051: f64 = (0.05 - noise_variable_13);
            let noise_metadata_schedule_77_e1053: f64 = (noise_metadata_schedule_77_e1051 / noise_variable_6);
            noise_variable_285 = noise_metadata_schedule_77_e1053;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_78_e1056: f64 = if 0.05 < noise_variable_13 { 1.0 } else { 0.0 };
            noise_variable_493 = noise_metadata_schedule_78_e1056;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_79_e1068,) = {
    if (noise_variable_493 != 0.0) {
        let noise_metadata_schedule_79_e1062: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_79_e1063: f64 = (1.0 + noise_metadata_schedule_79_e1062);
        let noise_metadata_schedule_79_e1064: f64 = (noise_metadata_schedule_79_e1063).ln();
        let noise_metadata_schedule_79_e1065: f64 = (noise_variable_6 * noise_metadata_schedule_79_e1064);
        let noise_metadata_schedule_79_e1066: f64 = (noise_variable_13 + noise_metadata_schedule_79_e1065);
        (noise_metadata_schedule_79_e1066,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_79_e1068;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_80_e1082,) = {
    if (noise_variable_493 == 0.0) {
        let noise_metadata_schedule_80_e1075: f64 = (-noise_variable_285);
        let noise_metadata_schedule_80_e1076: f64 = (noise_metadata_schedule_80_e1075).exp();
        let noise_metadata_schedule_80_e1077: f64 = (1.0 + noise_metadata_schedule_80_e1076);
        let noise_metadata_schedule_80_e1078: f64 = (noise_metadata_schedule_80_e1077).ln();
        let noise_metadata_schedule_80_e1079: f64 = (noise_variable_6 * noise_metadata_schedule_80_e1078);
        let noise_metadata_schedule_80_e1080: f64 = (0.05 + noise_metadata_schedule_80_e1079);
        (noise_metadata_schedule_80_e1080,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_80_e1082;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_81_e1084: f64 = (-3.0);
            let noise_metadata_schedule_81_e1086: f64 = (noise_metadata_schedule_81_e1084 * noise_variable_6);
            let noise_metadata_schedule_81_e1088: f64 = (noise_metadata_schedule_81_e1086 * noise_variable_280);
            let noise_metadata_schedule_81_e1091: f64 = (params.p64 * noise_variable_4);
            let noise_metadata_schedule_81_e1092: f64 = (noise_metadata_schedule_81_e1088 + noise_metadata_schedule_81_e1091);
            let noise_metadata_schedule_81_e1095: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_81_e1097: f64 = (noise_metadata_schedule_81_e1095 * params.p110);
            let noise_metadata_schedule_81_e1098: f64 = (noise_metadata_schedule_81_e1092 + noise_metadata_schedule_81_e1097);
            noise_variable_15 = noise_metadata_schedule_81_e1098;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_82_e1101: f64 = (0.05 - noise_variable_15);
            let noise_metadata_schedule_82_e1103: f64 = (noise_metadata_schedule_82_e1101 / noise_variable_6);
            noise_variable_285 = noise_metadata_schedule_82_e1103;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_83_e1106: f64 = if 0.05 < noise_variable_15 { 1.0 } else { 0.0 };
            noise_variable_494 = noise_metadata_schedule_83_e1106;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_84_e1118,) = {
    if (noise_variable_494 != 0.0) {
        let noise_metadata_schedule_84_e1112: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_84_e1113: f64 = (1.0 + noise_metadata_schedule_84_e1112);
        let noise_metadata_schedule_84_e1114: f64 = (noise_metadata_schedule_84_e1113).ln();
        let noise_metadata_schedule_84_e1115: f64 = (noise_variable_6 * noise_metadata_schedule_84_e1114);
        let noise_metadata_schedule_84_e1116: f64 = (noise_variable_15 + noise_metadata_schedule_84_e1115);
        (noise_metadata_schedule_84_e1116,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_84_e1118;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_85_e1132,) = {
    if (noise_variable_494 == 0.0) {
        let noise_metadata_schedule_85_e1125: f64 = (-noise_variable_285);
        let noise_metadata_schedule_85_e1126: f64 = (noise_metadata_schedule_85_e1125).exp();
        let noise_metadata_schedule_85_e1127: f64 = (1.0 + noise_metadata_schedule_85_e1126);
        let noise_metadata_schedule_85_e1128: f64 = (noise_metadata_schedule_85_e1127).ln();
        let noise_metadata_schedule_85_e1129: f64 = (noise_variable_6 * noise_metadata_schedule_85_e1128);
        let noise_metadata_schedule_85_e1130: f64 = (0.05 + noise_metadata_schedule_85_e1129);
        (noise_metadata_schedule_85_e1130,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_85_e1132;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_91_e1184: f64 = (-3.0);
            let noise_metadata_schedule_91_e1186: f64 = (noise_metadata_schedule_91_e1184 * noise_variable_6);
            let noise_metadata_schedule_91_e1188: f64 = (noise_metadata_schedule_91_e1186 * noise_variable_280);
            let noise_metadata_schedule_91_e1191: f64 = (params.p71 * noise_variable_4);
            let noise_metadata_schedule_91_e1192: f64 = (noise_metadata_schedule_91_e1188 + noise_metadata_schedule_91_e1191);
            let noise_metadata_schedule_91_e1195: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_91_e1197: f64 = (noise_metadata_schedule_91_e1195 * params.p110);
            let noise_metadata_schedule_91_e1198: f64 = (noise_metadata_schedule_91_e1192 + noise_metadata_schedule_91_e1197);
            noise_variable_18 = noise_metadata_schedule_91_e1198;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_92_e1201: f64 = (0.05 - noise_variable_18);
            let noise_metadata_schedule_92_e1203: f64 = (noise_metadata_schedule_92_e1201 / noise_variable_6);
            noise_variable_285 = noise_metadata_schedule_92_e1203;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_93_e1206: f64 = if 0.05 < noise_variable_18 { 1.0 } else { 0.0 };
            noise_variable_496 = noise_metadata_schedule_93_e1206;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_94_e1218,) = {
    if (noise_variable_496 != 0.0) {
        let noise_metadata_schedule_94_e1212: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_94_e1213: f64 = (1.0 + noise_metadata_schedule_94_e1212);
        let noise_metadata_schedule_94_e1214: f64 = (noise_metadata_schedule_94_e1213).ln();
        let noise_metadata_schedule_94_e1215: f64 = (noise_variable_6 * noise_metadata_schedule_94_e1214);
        let noise_metadata_schedule_94_e1216: f64 = (noise_variable_18 + noise_metadata_schedule_94_e1215);
        (noise_metadata_schedule_94_e1216,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_94_e1218;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_95_e1232,) = {
    if (noise_variable_496 == 0.0) {
        let noise_metadata_schedule_95_e1225: f64 = (-noise_variable_285);
        let noise_metadata_schedule_95_e1226: f64 = (noise_metadata_schedule_95_e1225).exp();
        let noise_metadata_schedule_95_e1227: f64 = (1.0 + noise_metadata_schedule_95_e1226);
        let noise_metadata_schedule_95_e1228: f64 = (noise_metadata_schedule_95_e1227).ln();
        let noise_metadata_schedule_95_e1229: f64 = (noise_variable_6 * noise_metadata_schedule_95_e1228);
        let noise_metadata_schedule_95_e1230: f64 = (0.05 + noise_metadata_schedule_95_e1229);
        (noise_metadata_schedule_95_e1230,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_95_e1232;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_96_e1234: f64 = (-3.0);
            let noise_metadata_schedule_96_e1236: f64 = (noise_metadata_schedule_96_e1234 * noise_variable_6);
            let noise_metadata_schedule_96_e1238: f64 = (noise_metadata_schedule_96_e1236 * noise_variable_280);
            let noise_metadata_schedule_96_e1241: f64 = (noise_variable_75 * noise_variable_4);
            let noise_metadata_schedule_96_e1242: f64 = (noise_metadata_schedule_96_e1238 + noise_metadata_schedule_96_e1241);
            let noise_metadata_schedule_96_e1245: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_96_e1247: f64 = (noise_metadata_schedule_96_e1245 * params.p110);
            let noise_metadata_schedule_96_e1248: f64 = (noise_metadata_schedule_96_e1242 + noise_metadata_schedule_96_e1247);
            noise_variable_20 = noise_metadata_schedule_96_e1248;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_97_e1251: f64 = (0.05 - noise_variable_20);
            let noise_metadata_schedule_97_e1253: f64 = (noise_metadata_schedule_97_e1251 / noise_variable_6);
            noise_variable_285 = noise_metadata_schedule_97_e1253;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_98_e1256: f64 = if 0.05 < noise_variable_20 { 1.0 } else { 0.0 };
            noise_variable_497 = noise_metadata_schedule_98_e1256;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_99_e1268,) = {
    if (noise_variable_497 != 0.0) {
        let noise_metadata_schedule_99_e1262: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_99_e1263: f64 = (1.0 + noise_metadata_schedule_99_e1262);
        let noise_metadata_schedule_99_e1264: f64 = (noise_metadata_schedule_99_e1263).ln();
        let noise_metadata_schedule_99_e1265: f64 = (noise_variable_6 * noise_metadata_schedule_99_e1264);
        let noise_metadata_schedule_99_e1266: f64 = (noise_variable_20 + noise_metadata_schedule_99_e1265);
        (noise_metadata_schedule_99_e1266,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_99_e1268;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_100_e1282,) = {
    if (noise_variable_497 == 0.0) {
        let noise_metadata_schedule_100_e1275: f64 = (-noise_variable_285);
        let noise_metadata_schedule_100_e1276: f64 = (noise_metadata_schedule_100_e1275).exp();
        let noise_metadata_schedule_100_e1277: f64 = (1.0 + noise_metadata_schedule_100_e1276);
        let noise_metadata_schedule_100_e1278: f64 = (noise_metadata_schedule_100_e1277).ln();
        let noise_metadata_schedule_100_e1279: f64 = (noise_variable_6 * noise_metadata_schedule_100_e1278);
        let noise_metadata_schedule_100_e1280: f64 = (0.05 + noise_metadata_schedule_100_e1279);
        (noise_metadata_schedule_100_e1280,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_100_e1282;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_101_e1284: f64 = (-3.0);
            let noise_metadata_schedule_101_e1286: f64 = (noise_metadata_schedule_101_e1284 * noise_variable_6);
            let noise_metadata_schedule_101_e1288: f64 = (noise_metadata_schedule_101_e1286 * noise_variable_280);
            let noise_metadata_schedule_101_e1291: f64 = (params.p27 * noise_variable_4);
            let noise_metadata_schedule_101_e1292: f64 = (noise_metadata_schedule_101_e1288 + noise_metadata_schedule_101_e1291);
            let noise_metadata_schedule_101_e1295: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_101_e1297: f64 = (noise_metadata_schedule_101_e1295 * params.p109);
            let noise_metadata_schedule_101_e1298: f64 = (noise_metadata_schedule_101_e1292 + noise_metadata_schedule_101_e1297);
            noise_variable_56 = noise_metadata_schedule_101_e1298;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_102_e1301: f64 = (0.05 - noise_variable_56);
            let noise_metadata_schedule_102_e1303: f64 = (noise_metadata_schedule_102_e1301 / noise_variable_6);
            noise_variable_285 = noise_metadata_schedule_102_e1303;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_103_e1306: f64 = if 0.05 < noise_variable_56 { 1.0 } else { 0.0 };
            noise_variable_498 = noise_metadata_schedule_103_e1306;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_104_e1318,) = {
    if (noise_variable_498 != 0.0) {
        let noise_metadata_schedule_104_e1312: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_104_e1313: f64 = (1.0 + noise_metadata_schedule_104_e1312);
        let noise_metadata_schedule_104_e1314: f64 = (noise_metadata_schedule_104_e1313).ln();
        let noise_metadata_schedule_104_e1315: f64 = (noise_variable_6 * noise_metadata_schedule_104_e1314);
        let noise_metadata_schedule_104_e1316: f64 = (noise_variable_56 + noise_metadata_schedule_104_e1315);
        (noise_metadata_schedule_104_e1316,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_104_e1318;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_105_e1332,) = {
    if (noise_variable_498 == 0.0) {
        let noise_metadata_schedule_105_e1325: f64 = (-noise_variable_285);
        let noise_metadata_schedule_105_e1326: f64 = (noise_metadata_schedule_105_e1325).exp();
        let noise_metadata_schedule_105_e1327: f64 = (1.0 + noise_metadata_schedule_105_e1326);
        let noise_metadata_schedule_105_e1328: f64 = (noise_metadata_schedule_105_e1327).ln();
        let noise_metadata_schedule_105_e1329: f64 = (noise_variable_6 * noise_metadata_schedule_105_e1328);
        let noise_metadata_schedule_105_e1330: f64 = (0.05 + noise_metadata_schedule_105_e1329);
        (noise_metadata_schedule_105_e1330,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_105_e1332;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_106_e1334: f64 = (-3.0);
            let noise_metadata_schedule_106_e1336: f64 = (noise_metadata_schedule_106_e1334 * noise_variable_6);
            let noise_metadata_schedule_106_e1338: f64 = (noise_metadata_schedule_106_e1336 * noise_variable_280);
            let noise_metadata_schedule_106_e1341: f64 = (params.p138 * noise_variable_4);
            let noise_metadata_schedule_106_e1342: f64 = (noise_metadata_schedule_106_e1338 + noise_metadata_schedule_106_e1341);
            let noise_metadata_schedule_106_e1345: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_106_e1347: f64 = (noise_metadata_schedule_106_e1345 * params.p140);
            let noise_metadata_schedule_106_e1348: f64 = (noise_metadata_schedule_106_e1342 + noise_metadata_schedule_106_e1347);
            noise_variable_104 = noise_metadata_schedule_106_e1348;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_107_e1351: f64 = (0.05 - noise_variable_104);
            let noise_metadata_schedule_107_e1353: f64 = (noise_metadata_schedule_107_e1351 / noise_variable_6);
            noise_variable_285 = noise_metadata_schedule_107_e1353;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_111_e1385: f64 = (1.0 / noise_variable_14);
            noise_variable_65 = noise_metadata_schedule_111_e1385;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_112_e1388: f64 = (1.0 / noise_variable_19);
            noise_variable_67 = noise_metadata_schedule_112_e1388;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_113_e1391: f64 = (params.p66 * noise_variable_65);
            let noise_metadata_schedule_113_e1393: f64 = (noise_metadata_schedule_113_e1391).powf(params.p67);
            noise_variable_73 = noise_metadata_schedule_113_e1393;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_114_e1396: f64 = (noise_variable_75 * noise_variable_67);
            let noise_metadata_schedule_114_e1398: f64 = (noise_metadata_schedule_114_e1396).powf(noise_variable_76);
            noise_variable_90 = noise_metadata_schedule_114_e1398;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_117_e1411: f64 = (1.0 - params.p75);
            let noise_metadata_schedule_117_e1414: f64 = (params.p71 / noise_variable_17);
            let noise_metadata_schedule_117_e1416: f64 = (noise_metadata_schedule_117_e1414).powf(params.p72);
            let noise_metadata_schedule_117_e1417: f64 = (noise_metadata_schedule_117_e1411 * noise_metadata_schedule_117_e1416);
            let noise_metadata_schedule_117_e1419: f64 = (noise_metadata_schedule_117_e1417 + params.p75);
            noise_variable_26 = noise_metadata_schedule_117_e1419;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_118_e1422: f64 = (1.0 / noise_variable_26);
            noise_variable_27 = noise_metadata_schedule_118_e1422;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_120_e1428: f64 = (params.p75 * noise_variable_27);
            noise_variable_25 = noise_metadata_schedule_120_e1428;
        }
        if matches!(source_index, 1 | 3) {
            let noise_metadata_schedule_121_e1432: f64 = (noise_variable_280 * params.p97);
            let noise_metadata_schedule_121_e1433: f64 = (noise_metadata_schedule_121_e1432).exp();
            let noise_metadata_schedule_121_e1434: f64 = (params.p54 * noise_metadata_schedule_121_e1433);
            noise_variable_28 = noise_metadata_schedule_121_e1434;
        }
        if matches!(source_index, 1 | 3) {
            let noise_metadata_schedule_122_e1437: f64 = if noise_variable_28 < noise_variable_346 { 1.0 } else { 0.0 };
            noise_variable_500 = noise_metadata_schedule_122_e1437;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_123_e1441,) = {
    if (noise_variable_500 != 0.0) {
        (noise_variable_346,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_123_e1441;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_124_e1446: f64 = (params.p98 - params.p96);
            let noise_metadata_schedule_124_e1447: f64 = (noise_variable_280 * noise_metadata_schedule_124_e1446);
            let noise_metadata_schedule_124_e1448: f64 = (noise_metadata_schedule_124_e1447).exp();
            let noise_metadata_schedule_124_e1449: f64 = (params.p56 * noise_metadata_schedule_124_e1448);
            noise_variable_29 = noise_metadata_schedule_124_e1449;
        }
        if matches!(source_index, 1 | 4) {
            let noise_metadata_schedule_125_e1453: f64 = (noise_variable_280 * params.p101);
            let noise_metadata_schedule_125_e1454: f64 = (noise_metadata_schedule_125_e1453).exp();
            let noise_metadata_schedule_125_e1455: f64 = (params.p55 * noise_metadata_schedule_125_e1454);
            noise_variable_30 = noise_metadata_schedule_125_e1455;
        }
        if matches!(source_index, 1 | 4) {
            let noise_metadata_schedule_126_e1458: f64 = if noise_variable_30 < noise_variable_346 { 1.0 } else { 0.0 };
            noise_variable_501 = noise_metadata_schedule_126_e1458;
        }
        if matches!(source_index, 1 | 4) {
            let (noise_metadata_schedule_127_e1462,) = {
    if (noise_variable_501 != 0.0) {
        (noise_variable_346,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_127_e1462;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19 | 20 | 23 | 25 | 27) {
            let noise_metadata_schedule_128_e1466: f64 = (noise_variable_280 * params.p102);
            let noise_metadata_schedule_128_e1467: f64 = (noise_metadata_schedule_128_e1466).exp();
            let noise_metadata_schedule_128_e1468: f64 = (params.p57 * noise_metadata_schedule_128_e1467);
            noise_variable_32 = noise_metadata_schedule_128_e1468;
        }
        if matches!(source_index, 21 | 24) {
            let noise_metadata_schedule_129_e1472: f64 = (noise_variable_280 * params.p104);
            let noise_metadata_schedule_129_e1473: f64 = (noise_metadata_schedule_129_e1472).exp();
            let noise_metadata_schedule_129_e1474: f64 = (params.p58 * noise_metadata_schedule_129_e1473);
            noise_variable_33 = noise_metadata_schedule_129_e1474;
        }
        if matches!(source_index, 22 | 26) {
            let noise_metadata_schedule_130_e1478: f64 = (noise_variable_280 * params.p104);
            let noise_metadata_schedule_130_e1479: f64 = (noise_metadata_schedule_130_e1478).exp();
            let noise_metadata_schedule_130_e1480: f64 = (params.p59 * noise_metadata_schedule_130_e1479);
            noise_variable_34 = noise_metadata_schedule_130_e1480;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_131_e1484: f64 = (noise_variable_280 * params.p99);
            let noise_metadata_schedule_131_e1485: f64 = (noise_metadata_schedule_131_e1484).exp();
            let noise_metadata_schedule_131_e1486: f64 = (params.p60 * noise_metadata_schedule_131_e1485);
            noise_variable_31 = noise_metadata_schedule_131_e1486;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_132_e1489: f64 = if params.p122 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_502 = noise_metadata_schedule_132_e1489;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_133_e1499,) = {
    if (noise_variable_502 != 0.0) {
        let noise_metadata_schedule_133_e1495: f64 = (noise_variable_12 * params.p122);
        let noise_metadata_schedule_133_e1496: f64 = (1.0 + noise_metadata_schedule_133_e1495);
        let noise_metadata_schedule_133_e1497: f64 = (params.p10 * noise_metadata_schedule_133_e1496);
        (noise_metadata_schedule_133_e1497,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_133_e1499;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_134_e1507,) = {
    if (noise_variable_502 != 0.0) {
        let noise_metadata_schedule_134_e1503: f64 = (noise_variable_50 - 1.0);
        let noise_metadata_schedule_134_e1505: f64 = (noise_metadata_schedule_134_e1503 / noise_variable_52);
        (noise_metadata_schedule_134_e1505,)
    } else {
        (noise_variable_285,)
    }
};
            noise_variable_285 = noise_metadata_schedule_134_e1507;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_135_e1510: f64 = if noise_variable_50 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_503 = noise_metadata_schedule_135_e1510;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_136_e1524,) = {
    if ((noise_variable_502 != 0.0) && (noise_variable_503 != 0.0)) {
        let noise_metadata_schedule_136_e1518: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_136_e1519: f64 = (1.0 + noise_metadata_schedule_136_e1518);
        let noise_metadata_schedule_136_e1520: f64 = (noise_metadata_schedule_136_e1519).ln();
        let noise_metadata_schedule_136_e1521: f64 = (noise_variable_52 * noise_metadata_schedule_136_e1520);
        let noise_metadata_schedule_136_e1522: f64 = (1.0 + noise_metadata_schedule_136_e1521);
        (noise_metadata_schedule_136_e1522,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_136_e1524;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_137_e1540,) = {
    if ((noise_variable_502 != 0.0) && (noise_variable_503 == 0.0)) {
        let noise_metadata_schedule_137_e1533: f64 = (-noise_variable_285);
        let noise_metadata_schedule_137_e1534: f64 = (noise_metadata_schedule_137_e1533).exp();
        let noise_metadata_schedule_137_e1535: f64 = (1.0 + noise_metadata_schedule_137_e1534);
        let noise_metadata_schedule_137_e1536: f64 = (noise_metadata_schedule_137_e1535).ln();
        let noise_metadata_schedule_137_e1537: f64 = (noise_variable_52 * noise_metadata_schedule_137_e1536);
        let noise_metadata_schedule_137_e1538: f64 = (noise_variable_50 + noise_metadata_schedule_137_e1537);
        (noise_metadata_schedule_137_e1538,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_137_e1540;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_138_e1548,) = {
    if (noise_variable_502 != 0.0) {
        let noise_metadata_schedule_138_e1545: f64 = (noise_variable_52 * 0.6931471805599453);
        let noise_metadata_schedule_138_e1546: f64 = (noise_variable_50 - noise_metadata_schedule_138_e1545);
        (noise_metadata_schedule_138_e1546,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_138_e1548;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_139_e1553,) = {
    if (noise_variable_502 == 0.0) {
        (params.p10,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_139_e1553;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_140_e1556: f64 = if params.p123 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_504 = noise_metadata_schedule_140_e1556;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_141_e1566,) = {
    if (noise_variable_504 != 0.0) {
        let noise_metadata_schedule_141_e1562: f64 = (noise_variable_12 * params.p123);
        let noise_metadata_schedule_141_e1563: f64 = (1.0 + noise_metadata_schedule_141_e1562);
        let noise_metadata_schedule_141_e1564: f64 = (params.p11 * noise_metadata_schedule_141_e1563);
        (noise_metadata_schedule_141_e1564,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_141_e1566;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_142_e1574,) = {
    if (noise_variable_504 != 0.0) {
        let noise_metadata_schedule_142_e1570: f64 = (noise_variable_51 - 1.0);
        let noise_metadata_schedule_142_e1572: f64 = (noise_metadata_schedule_142_e1570 / noise_variable_52);
        (noise_metadata_schedule_142_e1572,)
    } else {
        (noise_variable_285,)
    }
};
            noise_variable_285 = noise_metadata_schedule_142_e1574;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_143_e1577: f64 = if noise_variable_51 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_505 = noise_metadata_schedule_143_e1577;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_144_e1591,) = {
    if ((noise_variable_504 != 0.0) && (noise_variable_505 != 0.0)) {
        let noise_metadata_schedule_144_e1585: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_144_e1586: f64 = (1.0 + noise_metadata_schedule_144_e1585);
        let noise_metadata_schedule_144_e1587: f64 = (noise_metadata_schedule_144_e1586).ln();
        let noise_metadata_schedule_144_e1588: f64 = (noise_variable_52 * noise_metadata_schedule_144_e1587);
        let noise_metadata_schedule_144_e1589: f64 = (1.0 + noise_metadata_schedule_144_e1588);
        (noise_metadata_schedule_144_e1589,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_144_e1591;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_145_e1607,) = {
    if ((noise_variable_504 != 0.0) && (noise_variable_505 == 0.0)) {
        let noise_metadata_schedule_145_e1600: f64 = (-noise_variable_285);
        let noise_metadata_schedule_145_e1601: f64 = (noise_metadata_schedule_145_e1600).exp();
        let noise_metadata_schedule_145_e1602: f64 = (1.0 + noise_metadata_schedule_145_e1601);
        let noise_metadata_schedule_145_e1603: f64 = (noise_metadata_schedule_145_e1602).ln();
        let noise_metadata_schedule_145_e1604: f64 = (noise_variable_52 * noise_metadata_schedule_145_e1603);
        let noise_metadata_schedule_145_e1605: f64 = (noise_variable_51 + noise_metadata_schedule_145_e1604);
        (noise_metadata_schedule_145_e1605,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_145_e1607;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_146_e1615,) = {
    if (noise_variable_504 != 0.0) {
        let noise_metadata_schedule_146_e1612: f64 = (noise_variable_52 * 0.6931471805599453);
        let noise_metadata_schedule_146_e1613: f64 = (noise_variable_51 - noise_metadata_schedule_146_e1612);
        (noise_metadata_schedule_146_e1613,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_146_e1615;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_147_e1620,) = {
    if (noise_variable_504 == 0.0) {
        (params.p11,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_147_e1620;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_148_e1625: f64 = (params.p124 * noise_variable_12);
            let noise_metadata_schedule_148_e1626: f64 = (1.0 + noise_metadata_schedule_148_e1625);
            let noise_metadata_schedule_148_e1627: f64 = (params.p43 * noise_metadata_schedule_148_e1626);
            noise_variable_341 = noise_metadata_schedule_148_e1627;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_149_e1630: f64 = (noise_variable_342 * noise_variable_342);
            noise_variable_287 = noise_metadata_schedule_149_e1630;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_150_e1633: f64 = (noise_variable_341 * noise_variable_341);
            noise_variable_288 = noise_metadata_schedule_150_e1633;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_151_e1636: f64 = if noise_variable_341 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_506 = noise_metadata_schedule_151_e1636;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_152_e1649,) = {
    if (noise_variable_506 != 0.0) {
        let noise_metadata_schedule_152_e1640: f64 = (0.5 * noise_variable_287);
        let noise_metadata_schedule_152_e1643: f64 = (noise_variable_288 + noise_variable_287);
        let noise_metadata_schedule_152_e1644: f64 = (noise_metadata_schedule_152_e1643).sqrt();
        let noise_metadata_schedule_152_e1646: f64 = (noise_metadata_schedule_152_e1644 - noise_variable_341);
        let noise_metadata_schedule_152_e1647: f64 = (noise_metadata_schedule_152_e1640 / noise_metadata_schedule_152_e1646);
        (noise_metadata_schedule_152_e1647,)
    } else {
        (noise_variable_340,)
    }
};
            noise_variable_340 = noise_metadata_schedule_152_e1649;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_153_e1661,) = {
    if (noise_variable_506 == 0.0) {
        let noise_metadata_schedule_153_e1655: f64 = (noise_variable_288 + noise_variable_287);
        let noise_metadata_schedule_153_e1656: f64 = (noise_metadata_schedule_153_e1655).sqrt();
        let noise_metadata_schedule_153_e1658: f64 = (noise_metadata_schedule_153_e1656 + noise_variable_341);
        let noise_metadata_schedule_153_e1659: f64 = (0.5 * noise_metadata_schedule_153_e1658);
        (noise_metadata_schedule_153_e1659,)
    } else {
        (noise_variable_340,)
    }
};
            noise_variable_340 = noise_metadata_schedule_153_e1661;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_154_e1666: f64 = (4.0 - params.p98);
            let noise_metadata_schedule_154_e1668: f64 = (noise_metadata_schedule_154_e1666 - params.p96);
            let noise_metadata_schedule_154_e1670: f64 = (noise_metadata_schedule_154_e1668 + params.p121);
            let noise_metadata_schedule_154_e1671: f64 = (noise_variable_280 * noise_metadata_schedule_154_e1670);
            let noise_metadata_schedule_154_e1673: f64 = (noise_metadata_schedule_154_e1671 / noise_variable_48);
            let noise_metadata_schedule_154_e1674: f64 = (noise_metadata_schedule_154_e1673).exp();
            let noise_metadata_schedule_154_e1675: f64 = (params.p9 * noise_metadata_schedule_154_e1674);
            let noise_metadata_schedule_154_e1677: f64 = (-params.p105);
            let noise_metadata_schedule_154_e1679: f64 = (noise_metadata_schedule_154_e1677 * noise_variable_10);
            let noise_metadata_schedule_154_e1681: f64 = (noise_metadata_schedule_154_e1679 / noise_variable_48);
            let noise_metadata_schedule_154_e1682: f64 = (noise_metadata_schedule_154_e1681).exp();
            let noise_metadata_schedule_154_e1683: f64 = (noise_metadata_schedule_154_e1675 * noise_metadata_schedule_154_e1682);
            noise_variable_35 = noise_metadata_schedule_154_e1683;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_155_e1688: f64 = (1.0 - params.p98);
            let noise_metadata_schedule_155_e1689: f64 = (noise_variable_280 * noise_metadata_schedule_155_e1688);
            let noise_metadata_schedule_155_e1690: f64 = (noise_metadata_schedule_155_e1689).exp();
            let noise_metadata_schedule_155_e1691: f64 = (params.p12 * noise_metadata_schedule_155_e1690);
            noise_variable_36 = noise_metadata_schedule_155_e1691;
        }
        if matches!(source_index, 11 | 12 | 13 | 14 | 19) {
            let noise_metadata_schedule_156_e1696: f64 = (1.0 - params.p103);
            let noise_metadata_schedule_156_e1697: f64 = (noise_variable_280 * noise_metadata_schedule_156_e1696);
            let noise_metadata_schedule_156_e1698: f64 = (noise_metadata_schedule_156_e1697).exp();
            let noise_metadata_schedule_156_e1699: f64 = (params.p30 * noise_metadata_schedule_156_e1698);
            noise_variable_37 = noise_metadata_schedule_156_e1699;
        }
        if matches!(source_index, 2 | 7) {
            let noise_metadata_schedule_157_e1705: f64 = (2.0 * params.p21);
            let noise_metadata_schedule_157_e1706: f64 = (6.0 - noise_metadata_schedule_157_e1705);
            let noise_metadata_schedule_157_e1707: f64 = (noise_variable_280 * noise_metadata_schedule_157_e1706);
            let noise_metadata_schedule_157_e1708: f64 = (noise_metadata_schedule_157_e1707).exp();
            let noise_metadata_schedule_157_e1709: f64 = (params.p20 * noise_metadata_schedule_157_e1708);
            let noise_metadata_schedule_157_e1711: f64 = (-params.p113);
            let noise_metadata_schedule_157_e1713: f64 = (noise_metadata_schedule_157_e1711 * noise_variable_10);
            let noise_metadata_schedule_157_e1715: f64 = (noise_metadata_schedule_157_e1713 / params.p21);
            let noise_metadata_schedule_157_e1716: f64 = (noise_metadata_schedule_157_e1715).exp();
            let noise_metadata_schedule_157_e1717: f64 = (noise_metadata_schedule_157_e1709 * noise_metadata_schedule_157_e1716);
            noise_variable_38 = noise_metadata_schedule_157_e1717;
        }
        if matches!(source_index, 9 | 10) {
            let noise_metadata_schedule_158_e1723: f64 = (2.0 * params.p32);
            let noise_metadata_schedule_158_e1724: f64 = (6.0 - noise_metadata_schedule_158_e1723);
            let noise_metadata_schedule_158_e1725: f64 = (noise_variable_280 * noise_metadata_schedule_158_e1724);
            let noise_metadata_schedule_158_e1726: f64 = (noise_metadata_schedule_158_e1725).exp();
            let noise_metadata_schedule_158_e1727: f64 = (params.p31 * noise_metadata_schedule_158_e1726);
            let noise_metadata_schedule_158_e1729: f64 = (-params.p110);
            let noise_metadata_schedule_158_e1731: f64 = (noise_metadata_schedule_158_e1729 * noise_variable_10);
            let noise_metadata_schedule_158_e1733: f64 = (noise_metadata_schedule_158_e1731 / params.p32);
            let noise_metadata_schedule_158_e1734: f64 = (noise_metadata_schedule_158_e1733).exp();
            let noise_metadata_schedule_158_e1735: f64 = (noise_metadata_schedule_158_e1727 * noise_metadata_schedule_158_e1734);
            noise_variable_39 = noise_metadata_schedule_158_e1735;
        }
        if matches!(source_index, 1 | 2 | 6) {
            let noise_metadata_schedule_159_e1740: f64 = (4.0 - params.p97);
            let noise_metadata_schedule_159_e1742: f64 = (noise_metadata_schedule_159_e1740 + params.p121);
            let noise_metadata_schedule_159_e1743: f64 = (noise_variable_280 * noise_metadata_schedule_159_e1742);
            let noise_metadata_schedule_159_e1745: f64 = (noise_metadata_schedule_159_e1743 / params.p17);
            let noise_metadata_schedule_159_e1746: f64 = (noise_metadata_schedule_159_e1745).exp();
            let noise_metadata_schedule_159_e1747: f64 = (params.p16 * noise_metadata_schedule_159_e1746);
            let noise_metadata_schedule_159_e1749: f64 = (-params.p111);
            let noise_metadata_schedule_159_e1751: f64 = (noise_metadata_schedule_159_e1749 * noise_variable_10);
            let noise_metadata_schedule_159_e1753: f64 = (noise_metadata_schedule_159_e1751 / params.p17);
            let noise_metadata_schedule_159_e1754: f64 = (noise_metadata_schedule_159_e1753).exp();
            let noise_metadata_schedule_159_e1755: f64 = (noise_metadata_schedule_159_e1747 * noise_metadata_schedule_159_e1754);
            noise_variable_42 = noise_metadata_schedule_159_e1755;
        }
        if matches!(source_index, 6 | 8) {
            let noise_metadata_schedule_160_e1760: f64 = (4.0 - params.p97);
            let noise_metadata_schedule_160_e1762: f64 = (noise_metadata_schedule_160_e1760 + params.p121);
            let noise_metadata_schedule_160_e1763: f64 = (noise_variable_280 * noise_metadata_schedule_160_e1762);
            let noise_metadata_schedule_160_e1765: f64 = (noise_metadata_schedule_160_e1763 / params.p19);
            let noise_metadata_schedule_160_e1766: f64 = (noise_metadata_schedule_160_e1765).exp();
            let noise_metadata_schedule_160_e1767: f64 = (params.p18 * noise_metadata_schedule_160_e1766);
            let noise_metadata_schedule_160_e1769: f64 = (-params.p111);
            let noise_metadata_schedule_160_e1771: f64 = (noise_metadata_schedule_160_e1769 * noise_variable_10);
            let noise_metadata_schedule_160_e1773: f64 = (noise_metadata_schedule_160_e1771 / params.p19);
            let noise_metadata_schedule_160_e1774: f64 = (noise_metadata_schedule_160_e1773).exp();
            let noise_metadata_schedule_160_e1775: f64 = (noise_metadata_schedule_160_e1767 * noise_metadata_schedule_160_e1774);
            noise_variable_44 = noise_metadata_schedule_160_e1775;
        }
        if matches!(source_index, 2 | 6 | 8) {
            let noise_metadata_schedule_161_e1778: f64 = if params.p24 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_507 = noise_metadata_schedule_161_e1778;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_162_e1790,) = {
    if (noise_variable_507 != 0.0) {
        let noise_metadata_schedule_162_e1782: f64 = (-params.p107);
        let noise_metadata_schedule_162_e1784: f64 = (noise_metadata_schedule_162_e1782 * noise_variable_10);
        let noise_metadata_schedule_162_e1786: f64 = (noise_metadata_schedule_162_e1784 / params.p17);
        let noise_metadata_schedule_162_e1787: f64 = (noise_metadata_schedule_162_e1786).exp();
        let noise_metadata_schedule_162_e1788: f64 = (params.p25 * noise_metadata_schedule_162_e1787);
        (noise_metadata_schedule_162_e1788,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_162_e1790;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_163_e1800,) = {
    if (noise_variable_507 != 0.0) {
        let noise_metadata_schedule_163_e1794: f64 = (-params.p106);
        let noise_metadata_schedule_163_e1796: f64 = (noise_metadata_schedule_163_e1794 * noise_variable_10);
        let noise_metadata_schedule_163_e1797: f64 = (noise_metadata_schedule_163_e1796).exp();
        let noise_metadata_schedule_163_e1798: f64 = (params.p28 * noise_metadata_schedule_163_e1797);
        (noise_metadata_schedule_163_e1798,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_163_e1800;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_164_e1812,) = {
    if (noise_variable_507 != 0.0) {
        let noise_metadata_schedule_164_e1804: f64 = (-params.p108);
        let noise_metadata_schedule_164_e1806: f64 = (noise_metadata_schedule_164_e1804 * noise_variable_10);
        let noise_metadata_schedule_164_e1808: f64 = (noise_metadata_schedule_164_e1806 / params.p19);
        let noise_metadata_schedule_164_e1809: f64 = (noise_metadata_schedule_164_e1808).exp();
        let noise_metadata_schedule_164_e1810: f64 = (params.p26 * noise_metadata_schedule_164_e1809);
        (noise_metadata_schedule_164_e1810,)
    } else {
        (noise_variable_45,)
    }
};
            noise_variable_45 = noise_metadata_schedule_164_e1812;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_165_e1817: f64 = (4.0 - params.p103);
            let noise_metadata_schedule_165_e1819: f64 = (noise_metadata_schedule_165_e1817 + params.p121);
            let noise_metadata_schedule_165_e1820: f64 = (noise_variable_280 * noise_metadata_schedule_165_e1819);
            let noise_metadata_schedule_165_e1821: f64 = (noise_metadata_schedule_165_e1820).exp();
            let noise_metadata_schedule_165_e1822: f64 = (params.p29 * noise_metadata_schedule_165_e1821);
            let noise_metadata_schedule_165_e1824: f64 = (-params.p112);
            let noise_metadata_schedule_165_e1826: f64 = (noise_metadata_schedule_165_e1824 * noise_variable_10);
            let noise_metadata_schedule_165_e1827: f64 = (noise_metadata_schedule_165_e1826).exp();
            let noise_metadata_schedule_165_e1828: f64 = (noise_metadata_schedule_165_e1822 * noise_metadata_schedule_165_e1827);
            noise_variable_43 = noise_metadata_schedule_165_e1828;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_166_e1834: f64 = (2.0 * params.p23);
            let noise_metadata_schedule_166_e1835: f64 = (6.0 - noise_metadata_schedule_166_e1834);
            let noise_metadata_schedule_166_e1836: f64 = (noise_variable_280 * noise_metadata_schedule_166_e1835);
            let noise_metadata_schedule_166_e1837: f64 = (noise_metadata_schedule_166_e1836).exp();
            let noise_metadata_schedule_166_e1838: f64 = (params.p22 * noise_metadata_schedule_166_e1837);
            let noise_metadata_schedule_166_e1840: f64 = (-params.p113);
            let noise_metadata_schedule_166_e1842: f64 = (noise_metadata_schedule_166_e1840 * noise_variable_10);
            let noise_metadata_schedule_166_e1844: f64 = (noise_metadata_schedule_166_e1842 / params.p23);
            let noise_metadata_schedule_166_e1845: f64 = (noise_metadata_schedule_166_e1844).exp();
            let noise_metadata_schedule_166_e1846: f64 = (noise_metadata_schedule_166_e1838 * noise_metadata_schedule_166_e1845);
            noise_variable_46 = noise_metadata_schedule_166_e1846;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_167_e1851: f64 = (4.0 / params.p150);
            let noise_metadata_schedule_167_e1852: f64 = (noise_variable_280 * noise_metadata_schedule_167_e1851);
            let noise_metadata_schedule_167_e1853: f64 = (noise_metadata_schedule_167_e1852).exp();
            let noise_metadata_schedule_167_e1854: f64 = (params.p149 * noise_metadata_schedule_167_e1853);
            let noise_metadata_schedule_167_e1856: f64 = (-params.p113);
            let noise_metadata_schedule_167_e1858: f64 = (noise_metadata_schedule_167_e1856 * noise_variable_10);
            let noise_metadata_schedule_167_e1860: f64 = (noise_metadata_schedule_167_e1858 / params.p150);
            let noise_metadata_schedule_167_e1861: f64 = (noise_metadata_schedule_167_e1860).exp();
            let noise_metadata_schedule_167_e1862: f64 = (noise_metadata_schedule_167_e1854 * noise_metadata_schedule_167_e1861);
            noise_variable_47 = noise_metadata_schedule_167_e1862;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_168_e1865: f64 = (noise_variable_4).sqrt();
            let noise_metadata_schedule_168_e1866: f64 = (params.p155 * noise_metadata_schedule_168_e1865);
            let noise_metadata_schedule_168_e1869: f64 = (params.p157 * noise_variable_12);
            let noise_metadata_schedule_168_e1870: f64 = (noise_metadata_schedule_168_e1869).exp();
            let noise_metadata_schedule_168_e1871: f64 = (noise_metadata_schedule_168_e1866 * noise_metadata_schedule_168_e1870);
            noise_variable_357 = noise_metadata_schedule_168_e1871;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_169_e1874: f64 = (noise_variable_70 * noise_variable_72);
            let noise_metadata_schedule_169_e1876: f64 = (-0.5);
            let noise_metadata_schedule_169_e1877: f64 = (noise_metadata_schedule_169_e1874).powf(noise_metadata_schedule_169_e1876);
            noise_variable_281 = noise_metadata_schedule_169_e1877;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_170_e1880: f64 = (1.0 / noise_variable_73);
            noise_variable_282 = noise_metadata_schedule_170_e1880;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_171_e1883: f64 = (params.p35 * noise_variable_70);
            let noise_metadata_schedule_171_e1885: f64 = (noise_metadata_schedule_171_e1883 * noise_variable_70);
            let noise_metadata_schedule_171_e1887: f64 = (noise_metadata_schedule_171_e1885 * noise_variable_281);
            let noise_metadata_schedule_171_e1889: f64 = (noise_metadata_schedule_171_e1887 * noise_variable_282);
            let noise_metadata_schedule_171_e1891: f64 = (noise_metadata_schedule_171_e1889 * params.p66);
            let noise_metadata_schedule_171_e1893: f64 = (noise_metadata_schedule_171_e1891 * noise_variable_65);
            let noise_metadata_schedule_171_e1895: f64 = (noise_metadata_schedule_171_e1893 * noise_variable_72);
            let noise_metadata_schedule_171_e1897: f64 = (noise_metadata_schedule_171_e1895 * noise_variable_72);
            noise_variable_61 = noise_metadata_schedule_171_e1897;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_172_e1900: f64 = (params.p34 * noise_variable_281);
            let noise_metadata_schedule_172_e1902: f64 = (noise_metadata_schedule_172_e1900 * noise_variable_14);
            let noise_metadata_schedule_172_e1904: f64 = (noise_metadata_schedule_172_e1902 * noise_variable_14);
            let noise_metadata_schedule_172_e1906: f64 = (noise_metadata_schedule_172_e1904 * noise_variable_64);
            let noise_metadata_schedule_172_e1908: f64 = (noise_metadata_schedule_172_e1906 * noise_variable_64);
            let noise_metadata_schedule_172_e1910: f64 = (noise_metadata_schedule_172_e1908 * noise_variable_73);
            let noise_metadata_schedule_172_e1913: f64 = (params.p35 - noise_variable_61);
            let noise_metadata_schedule_172_e1914: f64 = (noise_metadata_schedule_172_e1913).exp();
            let noise_metadata_schedule_172_e1915: f64 = (noise_metadata_schedule_172_e1910 * noise_metadata_schedule_172_e1914);
            noise_variable_58 = noise_metadata_schedule_172_e1915;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_173_e1918: f64 = (1.0 / noise_variable_19);
            noise_variable_67 = noise_metadata_schedule_173_e1918;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_174_e1921: f64 = (noise_variable_85 * noise_variable_86);
            let noise_metadata_schedule_174_e1923: f64 = (-0.5);
            let noise_metadata_schedule_174_e1924: f64 = (noise_metadata_schedule_174_e1921).powf(noise_metadata_schedule_174_e1923);
            noise_variable_283 = noise_metadata_schedule_174_e1924;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_175_e1927: f64 = (1.0 / noise_variable_90);
            noise_variable_284 = noise_metadata_schedule_175_e1927;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_176_e1930: f64 = (params.p37 * noise_variable_85);
            let noise_metadata_schedule_176_e1932: f64 = (noise_metadata_schedule_176_e1930 * noise_variable_85);
            let noise_metadata_schedule_176_e1934: f64 = (noise_metadata_schedule_176_e1932 * noise_variable_283);
            let noise_metadata_schedule_176_e1936: f64 = (noise_metadata_schedule_176_e1934 * noise_variable_284);
            let noise_metadata_schedule_176_e1938: f64 = (noise_metadata_schedule_176_e1936 * noise_variable_75);
            let noise_metadata_schedule_176_e1940: f64 = (noise_metadata_schedule_176_e1938 * noise_variable_67);
            let noise_metadata_schedule_176_e1942: f64 = (noise_metadata_schedule_176_e1940 * noise_variable_86);
            let noise_metadata_schedule_176_e1944: f64 = (noise_metadata_schedule_176_e1942 * noise_variable_86);
            noise_variable_83 = noise_metadata_schedule_176_e1944;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_177_e1947: f64 = (params.p36 * noise_variable_283);
            let noise_metadata_schedule_177_e1949: f64 = (noise_metadata_schedule_177_e1947 * noise_variable_19);
            let noise_metadata_schedule_177_e1951: f64 = (noise_metadata_schedule_177_e1949 * noise_variable_19);
            let noise_metadata_schedule_177_e1953: f64 = (noise_metadata_schedule_177_e1951 * noise_variable_66);
            let noise_metadata_schedule_177_e1955: f64 = (noise_metadata_schedule_177_e1953 * noise_variable_66);
            let noise_metadata_schedule_177_e1957: f64 = (noise_metadata_schedule_177_e1955 * noise_variable_90);
            let noise_metadata_schedule_177_e1960: f64 = (params.p37 - noise_variable_83);
            let noise_metadata_schedule_177_e1961: f64 = (noise_metadata_schedule_177_e1960).exp();
            let noise_metadata_schedule_177_e1962: f64 = (noise_metadata_schedule_177_e1957 * noise_metadata_schedule_177_e1961);
            noise_variable_84 = noise_metadata_schedule_177_e1962;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_178_e1965: f64 = (noise_variable_280 * params.p96);
            let noise_metadata_schedule_178_e1966: f64 = (noise_metadata_schedule_178_e1965).exp();
            noise_variable_281 = noise_metadata_schedule_178_e1966;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_179_e1969: f64 = (params.p14 * noise_variable_281);
            let noise_metadata_schedule_179_e1971: f64 = (noise_metadata_schedule_179_e1969 * noise_variable_27);
            noise_variable_40 = noise_metadata_schedule_179_e1971;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_180_e1974: f64 = (params.p13 * noise_variable_281);
            let noise_metadata_schedule_180_e1976: f64 = (noise_metadata_schedule_180_e1974 * noise_variable_282);
            noise_variable_41 = noise_metadata_schedule_180_e1976;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_181_e1981: f64 = (4.0 - params.p141);
            let noise_metadata_schedule_181_e1982: f64 = (noise_variable_280 * noise_metadata_schedule_181_e1981);
            let noise_metadata_schedule_181_e1983: f64 = (noise_metadata_schedule_181_e1982).exp();
            let noise_metadata_schedule_181_e1984: f64 = (params.p133 * noise_metadata_schedule_181_e1983);
            let noise_metadata_schedule_181_e1986: f64 = (-params.p140);
            let noise_metadata_schedule_181_e1988: f64 = (noise_metadata_schedule_181_e1986 * noise_variable_10);
            let noise_metadata_schedule_181_e1989: f64 = (noise_metadata_schedule_181_e1988).exp();
            let noise_metadata_schedule_181_e1990: f64 = (noise_metadata_schedule_181_e1984 * noise_metadata_schedule_181_e1989);
            noise_variable_107 = noise_metadata_schedule_181_e1990;
        }
        if matches!(source_index, 13 | 14 | 17 | 18 | 19) {
            let noise_metadata_schedule_183_e2011: f64 = (1.0 - params.p141);
            let noise_metadata_schedule_183_e2012: f64 = (noise_variable_280 * noise_metadata_schedule_183_e2011);
            let noise_metadata_schedule_183_e2013: f64 = (noise_metadata_schedule_183_e2012).exp();
            let noise_metadata_schedule_183_e2014: f64 = (params.p135 * noise_metadata_schedule_183_e2013);
            noise_variable_109 = noise_metadata_schedule_183_e2014;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_190_e2074: f64 = (noise_variable_2 - 300.0);
            noise_variable_101 = noise_metadata_schedule_190_e2074;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_191_e2077: f64 = if noise_variable_2 < 525.0 { 1.0 } else { 0.0 };
            noise_variable_508 = noise_metadata_schedule_191_e2077;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_192_e2093,) = {
    if (noise_variable_508 != 0.0) {
        let noise_metadata_schedule_192_e2083: f64 = (0.00072 * noise_variable_101);
        let noise_metadata_schedule_192_e2084: f64 = (1.0 + noise_metadata_schedule_192_e2083);
        let noise_metadata_schedule_192_e2087: f64 = (1.6e-6 * noise_variable_101);
        let noise_metadata_schedule_192_e2089: f64 = (noise_metadata_schedule_192_e2087 * noise_variable_101);
        let noise_metadata_schedule_192_e2090: f64 = (noise_metadata_schedule_192_e2084 - noise_metadata_schedule_192_e2089);
        let noise_metadata_schedule_192_e2091: f64 = (noise_variable_1 * noise_metadata_schedule_192_e2090);
        (noise_metadata_schedule_192_e2091,)
    } else {
        (noise_variable_99,)
    }
};
            noise_variable_99 = noise_metadata_schedule_192_e2093;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_193_e2100,) = {
    if (noise_variable_508 == 0.0) {
        let noise_metadata_schedule_193_e2098: f64 = (noise_variable_1 * 1.081);
        (noise_metadata_schedule_193_e2098,)
    } else {
        (noise_variable_99,)
    }
};
            noise_variable_99 = noise_metadata_schedule_193_e2100;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_194_e2104: f64 = (noise_variable_280 * params.p96);
            let noise_metadata_schedule_194_e2105: f64 = (noise_metadata_schedule_194_e2104).exp();
            let noise_metadata_schedule_194_e2106: f64 = (params.p92 * noise_metadata_schedule_194_e2105);
            noise_variable_100 = noise_metadata_schedule_194_e2106;
        }
        if matches!(source_index, 20 | 23 | 25 | 27) {
            let noise_metadata_schedule_196_e2116: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_509 = noise_metadata_schedule_196_e2116;
        }
        if matches!(source_index, 20 | 23 | 25 | 27) {
            let (noise_metadata_schedule_197_e2122,) = {
    if (noise_variable_509 != 0.0) {
        let noise_metadata_schedule_197_e2120: f64 = (1.0 / noise_variable_32);
        (noise_metadata_schedule_197_e2120,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_197_e2122;
        }
        if matches!(source_index, 20 | 23 | 25 | 27) {
            let noise_metadata_schedule_198_e2125: f64 = if noise_variable_111 > noise_variable_347 { 1.0 } else { 0.0 };
            noise_variable_510 = noise_metadata_schedule_198_e2125;
        }
        if matches!(source_index, 20 | 23 | 25 | 27) {
            let (noise_metadata_schedule_199_e2131,) = {
    if ((noise_variable_509 != 0.0) && (noise_variable_510 != 0.0)) {
        (noise_variable_347,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_199_e2131;
        }
        if matches!(source_index, 20 | 23 | 25 | 27) {
            let (noise_metadata_schedule_200_e2136,) = {
    if (noise_variable_509 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_200_e2136;
        }
        if matches!(source_index, 21 | 24) {
            let noise_metadata_schedule_201_e2139: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_511 = noise_metadata_schedule_201_e2139;
        }
        if matches!(source_index, 21 | 24) {
            let (noise_metadata_schedule_202_e2145,) = {
    if (noise_variable_511 != 0.0) {
        let noise_metadata_schedule_202_e2143: f64 = (1.0 / noise_variable_33);
        (noise_metadata_schedule_202_e2143,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_202_e2145;
        }
        if matches!(source_index, 21 | 24) {
            let noise_metadata_schedule_203_e2148: f64 = if noise_variable_112 > noise_variable_347 { 1.0 } else { 0.0 };
            noise_variable_512 = noise_metadata_schedule_203_e2148;
        }
        if matches!(source_index, 21 | 24) {
            let (noise_metadata_schedule_204_e2154,) = {
    if ((noise_variable_511 != 0.0) && (noise_variable_512 != 0.0)) {
        (noise_variable_347,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_204_e2154;
        }
        if matches!(source_index, 21 | 24) {
            let (noise_metadata_schedule_205_e2159,) = {
    if (noise_variable_511 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_205_e2159;
        }
        if matches!(source_index, 22 | 26) {
            let noise_metadata_schedule_206_e2162: f64 = if params.p59 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_513 = noise_metadata_schedule_206_e2162;
        }
        if matches!(source_index, 22 | 26) {
            let (noise_metadata_schedule_207_e2168,) = {
    if (noise_variable_513 != 0.0) {
        let noise_metadata_schedule_207_e2166: f64 = (1.0 / noise_variable_34);
        (noise_metadata_schedule_207_e2166,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_207_e2168;
        }
        if matches!(source_index, 22 | 26) {
            let noise_metadata_schedule_208_e2171: f64 = if noise_variable_113 > noise_variable_347 { 1.0 } else { 0.0 };
            noise_variable_514 = noise_metadata_schedule_208_e2171;
        }
        if matches!(source_index, 22 | 26) {
            let (noise_metadata_schedule_209_e2177,) = {
    if ((noise_variable_513 != 0.0) && (noise_variable_514 != 0.0)) {
        (noise_variable_347,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_209_e2177;
        }
        if matches!(source_index, 22 | 26) {
            let (noise_metadata_schedule_210_e2182,) = {
    if (noise_variable_513 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_210_e2182;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_211_e2185: f64 = (params.p3 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[8])));
            noise_variable_250 = noise_metadata_schedule_211_e2185;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_212_e2188: f64 = (params.p3 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[9])));
            noise_variable_251 = noise_metadata_schedule_212_e2188;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_213_e2191: f64 = (params.p3 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_252 = noise_metadata_schedule_213_e2191;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_214_e2194: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_253 = noise_metadata_schedule_214_e2194;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_215_e2197: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[7])));
            noise_variable_254 = noise_metadata_schedule_215_e2197;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_216_e2200: f64 = (params.p3 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[8])));
            noise_variable_259 = noise_metadata_schedule_216_e2200;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_217_e2203: f64 = (params.p3 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
            noise_variable_256 = noise_metadata_schedule_217_e2203;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_219_e2209: f64 = (params.p3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
            noise_variable_266 = noise_metadata_schedule_219_e2209;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_221_e2215: f64 = (params.p3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[0])));
            noise_variable_270 = noise_metadata_schedule_221_e2215;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_222_e2218: f64 = (params.p3 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[8])));
            noise_variable_258 = noise_metadata_schedule_222_e2218;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_223_e2221: f64 = (params.p3 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[11])));
            noise_variable_257 = noise_metadata_schedule_223_e2221;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_224_e2224: f64 = (noise_variable_254 + noise_variable_251);
            let noise_metadata_schedule_224_e2226: f64 = (noise_metadata_schedule_224_e2224 - noise_variable_256);
            let noise_metadata_schedule_224_e2228: f64 = (noise_metadata_schedule_224_e2226 - noise_variable_258);
            noise_variable_255 = noise_metadata_schedule_224_e2228;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_225_e2230: f64 = (-noise_variable_270);
            let noise_metadata_schedule_225_e2232: f64 = (noise_metadata_schedule_225_e2230 + noise_variable_266);
            let noise_metadata_schedule_225_e2234: f64 = (noise_metadata_schedule_225_e2232 + noise_variable_255);
            let noise_metadata_schedule_225_e2236: f64 = (noise_metadata_schedule_225_e2234 - noise_variable_257);
            noise_variable_268 = noise_metadata_schedule_225_e2236;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_226_e2239: f64 = (noise_variable_270 + noise_variable_268);
            noise_variable_267 = noise_metadata_schedule_226_e2239;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let noise_metadata_schedule_227_e2242: f64 = (noise_variable_259 - noise_variable_258);
            noise_variable_261 = noise_metadata_schedule_227_e2242;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let noise_metadata_schedule_228_e2245: f64 = (noise_variable_261 - noise_variable_257);
            noise_variable_260 = noise_metadata_schedule_228_e2245;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_229_e2248: f64 = (noise_variable_251 * noise_variable_8);
            let noise_metadata_schedule_229_e2250: f64 = if noise_metadata_schedule_229_e2248 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_515 = noise_metadata_schedule_229_e2250;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16 | 17) {
            let (noise_metadata_schedule_230_e2257,) = {
    if (noise_variable_515 != 0.0) {
        let noise_metadata_schedule_230_e2254: f64 = (noise_variable_251 * noise_variable_8);
        let noise_metadata_schedule_230_e2255: f64 = (noise_metadata_schedule_230_e2254).exp();
        (noise_metadata_schedule_230_e2255,)
    } else {
        (noise_variable_271,)
    }
};
            noise_variable_271 = noise_metadata_schedule_230_e2257;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_231_e2263,) = {
    if (noise_variable_515 == 0.0) {
        let noise_metadata_schedule_231_e2261: f64 = (params.p151).exp();
        (noise_metadata_schedule_231_e2261,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_231_e2263;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16 | 17) {
            let (noise_metadata_schedule_232_e2276,) = {
    if (noise_variable_515 == 0.0) {
        let noise_metadata_schedule_232_e2270: f64 = (noise_variable_251 * noise_variable_8);
        let noise_metadata_schedule_232_e2272: f64 = (noise_metadata_schedule_232_e2270 - params.p151);
        let noise_metadata_schedule_232_e2273: f64 = (1.0 + noise_metadata_schedule_232_e2272);
        let noise_metadata_schedule_232_e2274: f64 = (noise_variable_301 * noise_metadata_schedule_232_e2273);
        (noise_metadata_schedule_232_e2274,)
    } else {
        (noise_variable_271,)
    }
};
            noise_variable_271 = noise_metadata_schedule_232_e2276;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_233_e2279: f64 = (noise_variable_252 * noise_variable_8);
            let noise_metadata_schedule_233_e2281: f64 = (noise_metadata_schedule_233_e2279 / noise_variable_48);
            let noise_metadata_schedule_233_e2283: f64 = if noise_metadata_schedule_233_e2281 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_516 = noise_metadata_schedule_233_e2283;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_234_e2292,) = {
    if (noise_variable_516 != 0.0) {
        let noise_metadata_schedule_234_e2287: f64 = (noise_variable_252 * noise_variable_8);
        let noise_metadata_schedule_234_e2289: f64 = (noise_metadata_schedule_234_e2287 / noise_variable_48);
        let noise_metadata_schedule_234_e2290: f64 = (noise_metadata_schedule_234_e2289).exp();
        (noise_metadata_schedule_234_e2290,)
    } else {
        (noise_variable_272,)
    }
};
            noise_variable_272 = noise_metadata_schedule_234_e2292;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_235_e2298,) = {
    if (noise_variable_516 == 0.0) {
        let noise_metadata_schedule_235_e2296: f64 = (params.p151).exp();
        (noise_metadata_schedule_235_e2296,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_235_e2298;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_236_e2313,) = {
    if (noise_variable_516 == 0.0) {
        let noise_metadata_schedule_236_e2305: f64 = (noise_variable_252 * noise_variable_8);
        let noise_metadata_schedule_236_e2307: f64 = (noise_metadata_schedule_236_e2305 / noise_variable_48);
        let noise_metadata_schedule_236_e2309: f64 = (noise_metadata_schedule_236_e2307 - params.p151);
        let noise_metadata_schedule_236_e2310: f64 = (1.0 + noise_metadata_schedule_236_e2309);
        let noise_metadata_schedule_236_e2311: f64 = (noise_variable_301 * noise_metadata_schedule_236_e2310);
        (noise_metadata_schedule_236_e2311,)
    } else {
        (noise_variable_272,)
    }
};
            noise_variable_272 = noise_metadata_schedule_236_e2313;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_237_e2316: f64 = (noise_variable_255 * noise_variable_8);
            let noise_metadata_schedule_237_e2318: f64 = if noise_metadata_schedule_237_e2316 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_517 = noise_metadata_schedule_237_e2318;
        }
        if matches!(source_index, 11 | 12 | 18) {
            let (noise_metadata_schedule_238_e2325,) = {
    if (noise_variable_517 != 0.0) {
        let noise_metadata_schedule_238_e2322: f64 = (noise_variable_255 * noise_variable_8);
        let noise_metadata_schedule_238_e2323: f64 = (noise_metadata_schedule_238_e2322).exp();
        (noise_metadata_schedule_238_e2323,)
    } else {
        (noise_variable_274,)
    }
};
            noise_variable_274 = noise_metadata_schedule_238_e2325;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_239_e2331,) = {
    if (noise_variable_517 == 0.0) {
        let noise_metadata_schedule_239_e2329: f64 = (params.p151).exp();
        (noise_metadata_schedule_239_e2329,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_239_e2331;
        }
        if matches!(source_index, 11 | 12 | 18) {
            let (noise_metadata_schedule_240_e2344,) = {
    if (noise_variable_517 == 0.0) {
        let noise_metadata_schedule_240_e2338: f64 = (noise_variable_255 * noise_variable_8);
        let noise_metadata_schedule_240_e2340: f64 = (noise_metadata_schedule_240_e2338 - params.p151);
        let noise_metadata_schedule_240_e2341: f64 = (1.0 + noise_metadata_schedule_240_e2340);
        let noise_metadata_schedule_240_e2342: f64 = (noise_variable_301 * noise_metadata_schedule_240_e2341);
        (noise_metadata_schedule_240_e2342,)
    } else {
        (noise_variable_274,)
    }
};
            noise_variable_274 = noise_metadata_schedule_240_e2344;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_241_e2347: f64 = (noise_variable_254 * noise_variable_8);
            let noise_metadata_schedule_241_e2349: f64 = if noise_metadata_schedule_241_e2347 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_518 = noise_metadata_schedule_241_e2349;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_242_e2356,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_242_e2353: f64 = (noise_variable_254 * noise_variable_8);
        let noise_metadata_schedule_242_e2354: f64 = (noise_metadata_schedule_242_e2353).exp();
        (noise_metadata_schedule_242_e2354,)
    } else {
        (noise_variable_273,)
    }
};
            noise_variable_273 = noise_metadata_schedule_242_e2356;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_243_e2362,) = {
    if (noise_variable_518 == 0.0) {
        let noise_metadata_schedule_243_e2360: f64 = (params.p151).exp();
        (noise_metadata_schedule_243_e2360,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_243_e2362;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_244_e2375,) = {
    if (noise_variable_518 == 0.0) {
        let noise_metadata_schedule_244_e2369: f64 = (noise_variable_254 * noise_variable_8);
        let noise_metadata_schedule_244_e2371: f64 = (noise_metadata_schedule_244_e2369 - params.p151);
        let noise_metadata_schedule_244_e2372: f64 = (1.0 + noise_metadata_schedule_244_e2371);
        let noise_metadata_schedule_244_e2373: f64 = (noise_variable_301 * noise_metadata_schedule_244_e2372);
        (noise_metadata_schedule_244_e2373,)
    } else {
        (noise_variable_273,)
    }
};
            noise_variable_273 = noise_metadata_schedule_244_e2375;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_245_e2378: f64 = (noise_variable_267 * noise_variable_8);
            let noise_metadata_schedule_245_e2380: f64 = if noise_metadata_schedule_245_e2378 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_519 = noise_metadata_schedule_245_e2380;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_246_e2387,) = {
    if (noise_variable_519 != 0.0) {
        let noise_metadata_schedule_246_e2384: f64 = (noise_variable_267 * noise_variable_8);
        let noise_metadata_schedule_246_e2385: f64 = (noise_metadata_schedule_246_e2384).exp();
        (noise_metadata_schedule_246_e2385,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_246_e2387;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_247_e2393,) = {
    if (noise_variable_519 == 0.0) {
        let noise_metadata_schedule_247_e2391: f64 = (params.p151).exp();
        (noise_metadata_schedule_247_e2391,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_247_e2393;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_248_e2406,) = {
    if (noise_variable_519 == 0.0) {
        let noise_metadata_schedule_248_e2400: f64 = (noise_variable_267 * noise_variable_8);
        let noise_metadata_schedule_248_e2402: f64 = (noise_metadata_schedule_248_e2400 - params.p151);
        let noise_metadata_schedule_248_e2403: f64 = (1.0 + noise_metadata_schedule_248_e2402);
        let noise_metadata_schedule_248_e2404: f64 = (noise_variable_301 * noise_metadata_schedule_248_e2403);
        (noise_metadata_schedule_248_e2404,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_248_e2406;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_249_e2409: f64 = (noise_variable_259 * noise_variable_8);
            let noise_metadata_schedule_249_e2411: f64 = if noise_metadata_schedule_249_e2409 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_520 = noise_metadata_schedule_249_e2411;
        }
        if matches!(source_index, 17) {
            let (noise_metadata_schedule_250_e2418,) = {
    if (noise_variable_520 != 0.0) {
        let noise_metadata_schedule_250_e2415: f64 = (noise_variable_259 * noise_variable_8);
        let noise_metadata_schedule_250_e2416: f64 = (noise_metadata_schedule_250_e2415).exp();
        (noise_metadata_schedule_250_e2416,)
    } else {
        (noise_variable_262,)
    }
};
            noise_variable_262 = noise_metadata_schedule_250_e2418;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_251_e2424,) = {
    if (noise_variable_520 == 0.0) {
        let noise_metadata_schedule_251_e2422: f64 = (params.p151).exp();
        (noise_metadata_schedule_251_e2422,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_251_e2424;
        }
        if matches!(source_index, 17) {
            let (noise_metadata_schedule_252_e2437,) = {
    if (noise_variable_520 == 0.0) {
        let noise_metadata_schedule_252_e2431: f64 = (noise_variable_259 * noise_variable_8);
        let noise_metadata_schedule_252_e2433: f64 = (noise_metadata_schedule_252_e2431 - params.p151);
        let noise_metadata_schedule_252_e2434: f64 = (1.0 + noise_metadata_schedule_252_e2433);
        let noise_metadata_schedule_252_e2435: f64 = (noise_variable_301 * noise_metadata_schedule_252_e2434);
        (noise_metadata_schedule_252_e2435,)
    } else {
        (noise_variable_262,)
    }
};
            noise_variable_262 = noise_metadata_schedule_252_e2437;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let noise_metadata_schedule_253_e2440: f64 = (noise_variable_260 * noise_variable_8);
            let noise_metadata_schedule_253_e2442: f64 = if noise_metadata_schedule_253_e2440 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_521 = noise_metadata_schedule_253_e2442;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_254_e2449,) = {
    if (noise_variable_521 != 0.0) {
        let noise_metadata_schedule_254_e2446: f64 = (noise_variable_260 * noise_variable_8);
        let noise_metadata_schedule_254_e2447: f64 = (noise_metadata_schedule_254_e2446).exp();
        (noise_metadata_schedule_254_e2447,)
    } else {
        (noise_variable_263,)
    }
};
            noise_variable_263 = noise_metadata_schedule_254_e2449;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let (noise_metadata_schedule_255_e2455,) = {
    if (noise_variable_521 == 0.0) {
        let noise_metadata_schedule_255_e2453: f64 = (params.p151).exp();
        (noise_metadata_schedule_255_e2453,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_255_e2455;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_256_e2468,) = {
    if (noise_variable_521 == 0.0) {
        let noise_metadata_schedule_256_e2462: f64 = (noise_variable_260 * noise_variable_8);
        let noise_metadata_schedule_256_e2464: f64 = (noise_metadata_schedule_256_e2462 - params.p151);
        let noise_metadata_schedule_256_e2465: f64 = (1.0 + noise_metadata_schedule_256_e2464);
        let noise_metadata_schedule_256_e2466: f64 = (noise_variable_301 * noise_metadata_schedule_256_e2465);
        (noise_metadata_schedule_256_e2466,)
    } else {
        (noise_variable_263,)
    }
};
            noise_variable_263 = noise_metadata_schedule_256_e2468;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let noise_metadata_schedule_257_e2471: f64 = (noise_variable_261 * noise_variable_8);
            let noise_metadata_schedule_257_e2473: f64 = if noise_metadata_schedule_257_e2471 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_522 = noise_metadata_schedule_257_e2473;
        }
        if matches!(source_index, 18) {
            let (noise_metadata_schedule_258_e2480,) = {
    if (noise_variable_522 != 0.0) {
        let noise_metadata_schedule_258_e2477: f64 = (noise_variable_261 * noise_variable_8);
        let noise_metadata_schedule_258_e2478: f64 = (noise_metadata_schedule_258_e2477).exp();
        (noise_metadata_schedule_258_e2478,)
    } else {
        (noise_variable_264,)
    }
};
            noise_variable_264 = noise_metadata_schedule_258_e2480;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let (noise_metadata_schedule_259_e2486,) = {
    if (noise_variable_522 == 0.0) {
        let noise_metadata_schedule_259_e2484: f64 = (params.p151).exp();
        (noise_metadata_schedule_259_e2484,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_259_e2486;
        }
        if matches!(source_index, 18) {
            let (noise_metadata_schedule_260_e2499,) = {
    if (noise_variable_522 == 0.0) {
        let noise_metadata_schedule_260_e2493: f64 = (noise_variable_261 * noise_variable_8);
        let noise_metadata_schedule_260_e2495: f64 = (noise_metadata_schedule_260_e2493 - params.p151);
        let noise_metadata_schedule_260_e2496: f64 = (1.0 + noise_metadata_schedule_260_e2495);
        let noise_metadata_schedule_260_e2497: f64 = (noise_variable_301 * noise_metadata_schedule_260_e2496);
        (noise_metadata_schedule_260_e2497,)
    } else {
        (noise_variable_264,)
    }
};
            noise_variable_264 = noise_metadata_schedule_260_e2499;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_261_e2502: f64 = (noise_variable_267 - noise_variable_16);
            let noise_metadata_schedule_261_e2504: f64 = (noise_metadata_schedule_261_e2502 * noise_variable_8);
            let noise_metadata_schedule_261_e2506: f64 = if noise_metadata_schedule_261_e2504 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_523 = noise_metadata_schedule_261_e2506;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_263_e2521,) = {
    if (noise_variable_523 == 0.0) {
        let noise_metadata_schedule_263_e2519: f64 = (params.p151).exp();
        (noise_metadata_schedule_263_e2519,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_263_e2521;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_265_e2539: f64 = (noise_variable_255 - noise_variable_16);
            let noise_metadata_schedule_265_e2541: f64 = (noise_metadata_schedule_265_e2539 * noise_variable_8);
            let noise_metadata_schedule_265_e2543: f64 = if noise_metadata_schedule_265_e2541 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_524 = noise_metadata_schedule_265_e2543;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_267_e2558,) = {
    if (noise_variable_524 == 0.0) {
        let noise_metadata_schedule_267_e2556: f64 = (params.p151).exp();
        (noise_metadata_schedule_267_e2556,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_267_e2558;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_269_e2576: f64 = (noise_variable_251 - noise_variable_16);
            let noise_metadata_schedule_269_e2578: f64 = (noise_metadata_schedule_269_e2576 * noise_variable_8);
            let noise_metadata_schedule_269_e2580: f64 = if noise_metadata_schedule_269_e2578 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_525 = noise_metadata_schedule_269_e2580;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_270_e2589,) = {
    if (noise_variable_525 != 0.0) {
        let noise_metadata_schedule_270_e2584: f64 = (noise_variable_251 - noise_variable_16);
        let noise_metadata_schedule_270_e2586: f64 = (noise_metadata_schedule_270_e2584 * noise_variable_8);
        let noise_metadata_schedule_270_e2587: f64 = (noise_metadata_schedule_270_e2586).exp();
        (noise_metadata_schedule_270_e2587,)
    } else {
        (noise_variable_277,)
    }
};
            noise_variable_277 = noise_metadata_schedule_270_e2589;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_271_e2595,) = {
    if (noise_variable_525 == 0.0) {
        let noise_metadata_schedule_271_e2593: f64 = (params.p151).exp();
        (noise_metadata_schedule_271_e2593,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_271_e2595;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_272_e2610,) = {
    if (noise_variable_525 == 0.0) {
        let noise_metadata_schedule_272_e2602: f64 = (noise_variable_251 - noise_variable_16);
        let noise_metadata_schedule_272_e2604: f64 = (noise_metadata_schedule_272_e2602 * noise_variable_8);
        let noise_metadata_schedule_272_e2606: f64 = (noise_metadata_schedule_272_e2604 - params.p151);
        let noise_metadata_schedule_272_e2607: f64 = (1.0 + noise_metadata_schedule_272_e2606);
        let noise_metadata_schedule_272_e2608: f64 = (noise_variable_301 * noise_metadata_schedule_272_e2607);
        (noise_metadata_schedule_272_e2608,)
    } else {
        (noise_variable_277,)
    }
};
            noise_variable_277 = noise_metadata_schedule_272_e2610;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_273_e2613: f64 = (noise_variable_250 - noise_variable_16);
            let noise_metadata_schedule_273_e2615: f64 = (noise_metadata_schedule_273_e2613 * noise_variable_8);
            let noise_metadata_schedule_273_e2617: f64 = if noise_metadata_schedule_273_e2615 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_526 = noise_metadata_schedule_273_e2617;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_274_e2626,) = {
    if (noise_variable_526 != 0.0) {
        let noise_metadata_schedule_274_e2621: f64 = (noise_variable_250 - noise_variable_16);
        let noise_metadata_schedule_274_e2623: f64 = (noise_metadata_schedule_274_e2621 * noise_variable_8);
        let noise_metadata_schedule_274_e2624: f64 = (noise_metadata_schedule_274_e2623).exp();
        (noise_metadata_schedule_274_e2624,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_274_e2626;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_275_e2632,) = {
    if (noise_variable_526 == 0.0) {
        let noise_metadata_schedule_275_e2630: f64 = (params.p151).exp();
        (noise_metadata_schedule_275_e2630,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_275_e2632;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_276_e2647,) = {
    if (noise_variable_526 == 0.0) {
        let noise_metadata_schedule_276_e2639: f64 = (noise_variable_250 - noise_variable_16);
        let noise_metadata_schedule_276_e2641: f64 = (noise_metadata_schedule_276_e2639 * noise_variable_8);
        let noise_metadata_schedule_276_e2643: f64 = (noise_metadata_schedule_276_e2641 - params.p151);
        let noise_metadata_schedule_276_e2644: f64 = (1.0 + noise_metadata_schedule_276_e2643);
        let noise_metadata_schedule_276_e2645: f64 = (noise_variable_301 * noise_metadata_schedule_276_e2644);
        (noise_metadata_schedule_276_e2645,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_276_e2647;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_277_e2651: f64 = (4.0 * noise_variable_277);
            let noise_metadata_schedule_277_e2652: f64 = (1.0 + noise_metadata_schedule_277_e2651);
            let noise_metadata_schedule_277_e2653: f64 = (noise_metadata_schedule_277_e2652).sqrt();
            noise_variable_114 = noise_metadata_schedule_277_e2653;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_278_e2657: f64 = (4.0 * noise_variable_279);
            let noise_metadata_schedule_278_e2658: f64 = (1.0 + noise_metadata_schedule_278_e2657);
            let noise_metadata_schedule_278_e2659: f64 = (noise_metadata_schedule_278_e2658).sqrt();
            noise_variable_115 = noise_metadata_schedule_278_e2659;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_279_e2662: f64 = (2.0 * noise_variable_279);
            let noise_metadata_schedule_279_e2665: f64 = (1.0 + noise_variable_115);
            let noise_metadata_schedule_279_e2666: f64 = (noise_metadata_schedule_279_e2662 / noise_metadata_schedule_279_e2665);
            noise_variable_116 = noise_metadata_schedule_279_e2666;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_280_e2669: f64 = if noise_variable_116 < params.p153 { 1.0 } else { 0.0 };
            noise_variable_527 = noise_metadata_schedule_280_e2669;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_281_e2673,) = {
    if (noise_variable_527 != 0.0) {
        (params.p153,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_281_e2673;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_282_e2677: f64 = (noise_variable_114 - noise_variable_115);
            let noise_metadata_schedule_282_e2680: f64 = (noise_variable_114 + 1.0);
            let noise_metadata_schedule_282_e2683: f64 = (noise_variable_115 + 1.0);
            let noise_metadata_schedule_282_e2684: f64 = (noise_metadata_schedule_282_e2680 / noise_metadata_schedule_282_e2683);
            let noise_metadata_schedule_282_e2685: f64 = (noise_metadata_schedule_282_e2684).ln();
            let noise_metadata_schedule_282_e2686: f64 = (noise_metadata_schedule_282_e2677 - noise_metadata_schedule_282_e2685);
            let noise_metadata_schedule_282_e2687: f64 = (noise_variable_6 * noise_metadata_schedule_282_e2686);
            noise_variable_117 = noise_metadata_schedule_282_e2687;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_283_e2690: f64 = (noise_variable_117 + noise_variable_256);
            let noise_metadata_schedule_283_e2692: f64 = (noise_metadata_schedule_283_e2690 / noise_variable_31);
            noise_variable_118 = noise_metadata_schedule_283_e2692;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_284_e2695: f64 = if noise_variable_118 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_528 = noise_metadata_schedule_284_e2695;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_285_e2698: f64 = if noise_variable_250 < 100.0 { 1.0 } else { 0.0 };
            noise_variable_529 = noise_metadata_schedule_285_e2698;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_286_e2704,) = {
    if ((noise_variable_528 != 0.0) && (noise_variable_529 != 0.0)) {
        (noise_variable_250,)
    } else {
        (noise_variable_303,)
    }
};
            noise_variable_303 = noise_metadata_schedule_286_e2704;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_287_e2718,) = {
    if ((noise_variable_528 != 0.0) && (noise_variable_529 == 0.0)) {
        let noise_metadata_schedule_287_e2713: f64 = (noise_variable_250 - 100.0);
        let noise_metadata_schedule_287_e2714: f64 = (1.0 + noise_metadata_schedule_287_e2713);
        let noise_metadata_schedule_287_e2715: f64 = (noise_metadata_schedule_287_e2714).ln();
        let noise_metadata_schedule_287_e2716: f64 = (100.0 + noise_metadata_schedule_287_e2715);
        (noise_metadata_schedule_287_e2716,)
    } else {
        (noise_variable_303,)
    }
};
            noise_variable_303 = noise_metadata_schedule_287_e2718;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_288_e2739,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_288_e2723: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_288_e2726: f64 = (0.5 * noise_variable_118);
        let noise_metadata_schedule_288_e2728: f64 = (noise_metadata_schedule_288_e2726 * noise_variable_31);
        let noise_metadata_schedule_288_e2730: f64 = (noise_metadata_schedule_288_e2728 * noise_variable_8);
        let noise_metadata_schedule_288_e2732: f64 = (noise_metadata_schedule_288_e2730 + 1.0);
        let noise_metadata_schedule_288_e2733: f64 = (noise_metadata_schedule_288_e2732).ln();
        let noise_metadata_schedule_288_e2734: f64 = (noise_metadata_schedule_288_e2723 * noise_metadata_schedule_288_e2733);
        let noise_metadata_schedule_288_e2735: f64 = (noise_variable_16 + noise_metadata_schedule_288_e2734);
        let noise_metadata_schedule_288_e2737: f64 = (noise_metadata_schedule_288_e2735 - noise_variable_303);
        (noise_metadata_schedule_288_e2737,)
    } else {
        (noise_variable_119,)
    }
};
            noise_variable_119 = noise_metadata_schedule_288_e2739;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_289_e2745,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_289_e2743: f64 = (0.2 * noise_variable_16);
        (noise_metadata_schedule_289_e2743,)
    } else {
        (noise_variable_298,)
    }
};
            noise_variable_298 = noise_metadata_schedule_289_e2745;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_290_e2751,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_290_e2749: f64 = (noise_variable_298 * noise_variable_298);
        (noise_metadata_schedule_290_e2749,)
    } else {
        (noise_variable_287,)
    }
};
            noise_variable_287 = noise_metadata_schedule_290_e2751;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_291_e2757,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_291_e2755: f64 = (noise_variable_119 * noise_variable_119);
        (noise_metadata_schedule_291_e2755,)
    } else {
        (noise_variable_288,)
    }
};
            noise_variable_288 = noise_metadata_schedule_291_e2757;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_292_e2760: f64 = if noise_variable_119 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_530 = noise_metadata_schedule_292_e2760;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_293_e2775,) = {
    if ((noise_variable_528 != 0.0) && (noise_variable_530 != 0.0)) {
        let noise_metadata_schedule_293_e2766: f64 = (0.5 * noise_variable_287);
        let noise_metadata_schedule_293_e2769: f64 = (noise_variable_288 + noise_variable_287);
        let noise_metadata_schedule_293_e2770: f64 = (noise_metadata_schedule_293_e2769).sqrt();
        let noise_metadata_schedule_293_e2772: f64 = (noise_metadata_schedule_293_e2770 - noise_variable_119);
        let noise_metadata_schedule_293_e2773: f64 = (noise_metadata_schedule_293_e2766 / noise_metadata_schedule_293_e2772);
        (noise_metadata_schedule_293_e2773,)
    } else {
        (noise_variable_120,)
    }
};
            noise_variable_120 = noise_metadata_schedule_293_e2775;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_294_e2789,) = {
    if ((noise_variable_528 != 0.0) && (noise_variable_530 == 0.0)) {
        let noise_metadata_schedule_294_e2783: f64 = (noise_variable_288 + noise_variable_287);
        let noise_metadata_schedule_294_e2784: f64 = (noise_metadata_schedule_294_e2783).sqrt();
        let noise_metadata_schedule_294_e2786: f64 = (noise_metadata_schedule_294_e2784 + noise_variable_119);
        let noise_metadata_schedule_294_e2787: f64 = (0.5 * noise_metadata_schedule_294_e2786);
        (noise_metadata_schedule_294_e2787,)
    } else {
        (noise_variable_120,)
    }
};
            noise_variable_120 = noise_metadata_schedule_294_e2789;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_295_e2807,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_295_e2795: f64 = (params.p62 * params.p61);
        let noise_metadata_schedule_295_e2796: f64 = (noise_variable_120 + noise_metadata_schedule_295_e2795);
        let noise_metadata_schedule_295_e2797: f64 = (noise_variable_120 * noise_metadata_schedule_295_e2796);
        let noise_metadata_schedule_295_e2802: f64 = (params.p62 * noise_variable_31);
        let noise_metadata_schedule_295_e2803: f64 = (noise_variable_120 + noise_metadata_schedule_295_e2802);
        let noise_metadata_schedule_295_e2804: f64 = (params.p61 * noise_metadata_schedule_295_e2803);
        let noise_metadata_schedule_295_e2805: f64 = (noise_metadata_schedule_295_e2797 / noise_metadata_schedule_295_e2804);
        (noise_metadata_schedule_295_e2805,)
    } else {
        (noise_variable_121,)
    }
};
            noise_variable_121 = noise_metadata_schedule_295_e2807;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_296_e2813,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_296_e2811: f64 = (noise_variable_118 / noise_variable_121);
        (noise_metadata_schedule_296_e2811,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_296_e2813;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_297_e2821,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_297_e2817: f64 = (noise_variable_291 - 1.0);
        let noise_metadata_schedule_297_e2819: f64 = (noise_metadata_schedule_297_e2817 / params.p63);
        (noise_metadata_schedule_297_e2819,)
    } else {
        (noise_variable_285,)
    }
};
            noise_variable_285 = noise_metadata_schedule_297_e2821;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_298_e2824: f64 = if noise_variable_291 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_531 = noise_metadata_schedule_298_e2824;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_299_e2838,) = {
    if ((noise_variable_528 != 0.0) && (noise_variable_531 != 0.0)) {
        let noise_metadata_schedule_299_e2832: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_299_e2833: f64 = (1.0 + noise_metadata_schedule_299_e2832);
        let noise_metadata_schedule_299_e2834: f64 = (noise_metadata_schedule_299_e2833).ln();
        let noise_metadata_schedule_299_e2835: f64 = (params.p63 * noise_metadata_schedule_299_e2834);
        let noise_metadata_schedule_299_e2836: f64 = (1.0 + noise_metadata_schedule_299_e2835);
        (noise_metadata_schedule_299_e2836,)
    } else {
        (noise_variable_289,)
    }
};
            noise_variable_289 = noise_metadata_schedule_299_e2838;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_300_e2854,) = {
    if ((noise_variable_528 != 0.0) && (noise_variable_531 == 0.0)) {
        let noise_metadata_schedule_300_e2847: f64 = (-noise_variable_285);
        let noise_metadata_schedule_300_e2848: f64 = (noise_metadata_schedule_300_e2847).exp();
        let noise_metadata_schedule_300_e2849: f64 = (1.0 + noise_metadata_schedule_300_e2848);
        let noise_metadata_schedule_300_e2850: f64 = (noise_metadata_schedule_300_e2849).ln();
        let noise_metadata_schedule_300_e2851: f64 = (params.p63 * noise_metadata_schedule_300_e2850);
        let noise_metadata_schedule_300_e2852: f64 = (noise_variable_291 + noise_metadata_schedule_300_e2851);
        (noise_metadata_schedule_300_e2852,)
    } else {
        (noise_variable_289,)
    }
};
            noise_variable_289 = noise_metadata_schedule_300_e2854;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_301_e2871,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_301_e2861: f64 = (-1.0);
        let noise_metadata_schedule_301_e2863: f64 = (noise_metadata_schedule_301_e2861 / params.p63);
        let noise_metadata_schedule_301_e2864: f64 = (noise_metadata_schedule_301_e2863).exp();
        let noise_metadata_schedule_301_e2865: f64 = (1.0 + noise_metadata_schedule_301_e2864);
        let noise_metadata_schedule_301_e2866: f64 = (noise_metadata_schedule_301_e2865).ln();
        let noise_metadata_schedule_301_e2867: f64 = (params.p63 * noise_metadata_schedule_301_e2866);
        let noise_metadata_schedule_301_e2868: f64 = (1.0 + noise_metadata_schedule_301_e2867);
        let noise_metadata_schedule_301_e2869: f64 = (noise_variable_289 / noise_metadata_schedule_301_e2868);
        (noise_metadata_schedule_301_e2869,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_301_e2871;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_302_e2879,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_302_e2876: f64 = (params.p62 * params.p61);
        let noise_metadata_schedule_302_e2877: f64 = (noise_variable_120 / noise_metadata_schedule_302_e2876);
        (noise_metadata_schedule_302_e2877,)
    } else {
        (noise_variable_123,)
    }
};
            noise_variable_123 = noise_metadata_schedule_302_e2879;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_303_e2904,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_303_e2885: f64 = (4.0 * noise_variable_122);
        let noise_metadata_schedule_303_e2887: f64 = (noise_metadata_schedule_303_e2885 * noise_variable_123);
        let noise_metadata_schedule_303_e2890: f64 = (1.0 + noise_variable_123);
        let noise_metadata_schedule_303_e2891: f64 = (noise_metadata_schedule_303_e2887 * noise_metadata_schedule_303_e2890);
        let noise_metadata_schedule_303_e2892: f64 = (1.0 + noise_metadata_schedule_303_e2891);
        let noise_metadata_schedule_303_e2893: f64 = (noise_metadata_schedule_303_e2892).sqrt();
        let noise_metadata_schedule_303_e2894: f64 = (1.0 + noise_metadata_schedule_303_e2893);
        let noise_metadata_schedule_303_e2897: f64 = (2.0 * noise_variable_122);
        let noise_metadata_schedule_303_e2900: f64 = (1.0 + noise_variable_123);
        let noise_metadata_schedule_303_e2901: f64 = (noise_metadata_schedule_303_e2897 * noise_metadata_schedule_303_e2900);
        let noise_metadata_schedule_303_e2902: f64 = (noise_metadata_schedule_303_e2894 / noise_metadata_schedule_303_e2901);
        (noise_metadata_schedule_303_e2902,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_303_e2904;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_304_e2920,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_304_e2908: f64 = (1.0 - noise_variable_124);
        let noise_metadata_schedule_304_e2911: f64 = (noise_variable_116 * noise_variable_124);
        let noise_metadata_schedule_304_e2912: f64 = (noise_metadata_schedule_304_e2908 + noise_metadata_schedule_304_e2911);
        let noise_metadata_schedule_304_e2916: f64 = (noise_variable_116 * noise_variable_124);
        let noise_metadata_schedule_304_e2917: f64 = (1.0 + noise_metadata_schedule_304_e2916);
        let noise_metadata_schedule_304_e2918: f64 = (noise_metadata_schedule_304_e2912 / noise_metadata_schedule_304_e2917);
        (noise_metadata_schedule_304_e2918,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_304_e2920;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_305_e2932,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_305_e2924: f64 = (0.5 * noise_variable_118);
        let noise_metadata_schedule_305_e2926: f64 = (noise_metadata_schedule_305_e2924 * noise_variable_31);
        let noise_metadata_schedule_305_e2928: f64 = (noise_metadata_schedule_305_e2926 * noise_variable_125);
        let noise_metadata_schedule_305_e2930: f64 = (noise_metadata_schedule_305_e2928 * noise_variable_8);
        (noise_metadata_schedule_305_e2930,)
    } else {
        (noise_variable_127,)
    }
};
            noise_variable_127 = noise_metadata_schedule_305_e2932;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_306_e2946,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_306_e2936: f64 = (2.0 * noise_variable_127);
        let noise_metadata_schedule_306_e2940: f64 = (noise_variable_116 + noise_variable_127);
        let noise_metadata_schedule_306_e2942: f64 = (noise_metadata_schedule_306_e2940 + 1.0);
        let noise_metadata_schedule_306_e2943: f64 = (noise_variable_116 * noise_metadata_schedule_306_e2942);
        let noise_metadata_schedule_306_e2944: f64 = (noise_metadata_schedule_306_e2936 + noise_metadata_schedule_306_e2943);
        (noise_metadata_schedule_306_e2944,)
    } else {
        (noise_variable_292,)
    }
};
            noise_variable_292 = noise_metadata_schedule_306_e2946;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_307_e2954,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_307_e2951: f64 = (noise_variable_127 - 1.0);
        let noise_metadata_schedule_307_e2952: f64 = (0.5 * noise_metadata_schedule_307_e2951);
        (noise_metadata_schedule_307_e2952,)
    } else {
        (noise_variable_128,)
    }
};
            noise_variable_128 = noise_metadata_schedule_307_e2954;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_308_e2962,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_308_e2958: f64 = (noise_variable_128 * noise_variable_128);
        let noise_metadata_schedule_308_e2960: f64 = (noise_metadata_schedule_308_e2958 + noise_variable_292);
        (noise_metadata_schedule_308_e2960,)
    } else {
        (noise_variable_286,)
    }
};
            noise_variable_286 = noise_metadata_schedule_308_e2962;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_309_e2965: f64 = if noise_variable_127 >= 1.0 { 1.0 } else { 0.0 };
            noise_variable_532 = noise_metadata_schedule_309_e2965;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_310_e2974,) = {
    if ((noise_variable_528 != 0.0) && (noise_variable_532 != 0.0)) {
        let noise_metadata_schedule_310_e2971: f64 = (noise_variable_286).sqrt();
        let noise_metadata_schedule_310_e2972: f64 = (noise_variable_128 + noise_metadata_schedule_310_e2971);
        (noise_metadata_schedule_310_e2972,)
    } else {
        (noise_variable_129,)
    }
};
            noise_variable_129 = noise_metadata_schedule_310_e2974;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_311_e2986,) = {
    if ((noise_variable_528 != 0.0) && (noise_variable_532 == 0.0)) {
        let noise_metadata_schedule_311_e2981: f64 = (noise_variable_286).sqrt();
        let noise_metadata_schedule_311_e2983: f64 = (noise_metadata_schedule_311_e2981 - noise_variable_128);
        let noise_metadata_schedule_311_e2984: f64 = (noise_variable_292 / noise_metadata_schedule_311_e2983);
        (noise_metadata_schedule_311_e2984,)
    } else {
        (noise_variable_129,)
    }
};
            noise_variable_129 = noise_metadata_schedule_311_e2986;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_312_e2989: f64 = if noise_variable_129 < params.p152 { 1.0 } else { 0.0 };
            noise_variable_533 = noise_metadata_schedule_312_e2989;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_313_e2995,) = {
    if ((noise_variable_528 != 0.0) && (noise_variable_533 != 0.0)) {
        (params.p152,)
    } else {
        (noise_variable_129,)
    }
};
            noise_variable_129 = noise_metadata_schedule_313_e2995;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_314_e3008,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_314_e3000: f64 = (noise_variable_129 + 1.0);
        let noise_metadata_schedule_314_e3001: f64 = (noise_variable_129 * noise_metadata_schedule_314_e3000);
        let noise_metadata_schedule_314_e3004: f64 = (noise_variable_16 * noise_variable_8);
        let noise_metadata_schedule_314_e3005: f64 = (noise_metadata_schedule_314_e3004).exp();
        let noise_metadata_schedule_314_e3006: f64 = (noise_metadata_schedule_314_e3001 * noise_metadata_schedule_314_e3005);
        (noise_metadata_schedule_314_e3006,)
    } else {
        (noise_variable_131,)
    }
};
            noise_variable_131 = noise_metadata_schedule_314_e3008;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_315_e3018,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_315_e3012: f64 = (0.5 * params.p61);
        let noise_metadata_schedule_315_e3015: f64 = (noise_variable_118 - params.p62);
        let noise_metadata_schedule_315_e3016: f64 = (noise_metadata_schedule_315_e3012 * noise_metadata_schedule_315_e3015);
        (noise_metadata_schedule_315_e3016,)
    } else {
        (noise_variable_133,)
    }
};
            noise_variable_133 = noise_metadata_schedule_315_e3018;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_316_e3028,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_316_e3022: f64 = (params.p61 * noise_variable_31);
        let noise_metadata_schedule_316_e3024: f64 = (noise_metadata_schedule_316_e3022 * params.p62);
        let noise_metadata_schedule_316_e3026: f64 = (noise_metadata_schedule_316_e3024 * noise_variable_118);
        (noise_metadata_schedule_316_e3026,)
    } else {
        (noise_variable_134,)
    }
};
            noise_variable_134 = noise_metadata_schedule_316_e3028;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_317_e3039,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_317_e3033: f64 = (noise_variable_133 * noise_variable_133);
        let noise_metadata_schedule_317_e3035: f64 = (noise_metadata_schedule_317_e3033 + noise_variable_134);
        let noise_metadata_schedule_317_e3036: f64 = (noise_metadata_schedule_317_e3035).sqrt();
        let noise_metadata_schedule_317_e3037: f64 = (noise_variable_133 + noise_metadata_schedule_317_e3036);
        (noise_metadata_schedule_317_e3037,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_317_e3039;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_318_e3042: f64 = if params.p73 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_534 = noise_metadata_schedule_318_e3042;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_319_e3050,) = {
    if ((noise_variable_528 != 0.0) && (noise_variable_534 != 0.0)) {
        let noise_metadata_schedule_319_e3048: f64 = (noise_variable_17 * 0.1);
        (noise_metadata_schedule_319_e3048,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_319_e3050;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_320_e3067,) = {
    if ((noise_variable_528 != 0.0) && (noise_variable_534 == 0.0)) {
        let noise_metadata_schedule_320_e3059: f64 = (2.0 * noise_variable_118);
        let noise_metadata_schedule_320_e3062: f64 = (noise_variable_118 + noise_variable_121);
        let noise_metadata_schedule_320_e3063: f64 = (noise_metadata_schedule_320_e3059 / noise_metadata_schedule_320_e3062);
        let noise_metadata_schedule_320_e3064: f64 = (0.1 + noise_metadata_schedule_320_e3063);
        let noise_metadata_schedule_320_e3065: f64 = (noise_variable_17 * noise_metadata_schedule_320_e3064);
        (noise_metadata_schedule_320_e3065,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_320_e3067;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_321_e3077,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_321_e3071: f64 = (params.p62 * noise_variable_118);
        let noise_metadata_schedule_321_e3074: f64 = (params.p62 + noise_variable_118);
        let noise_metadata_schedule_321_e3075: f64 = (noise_metadata_schedule_321_e3071 / noise_metadata_schedule_321_e3074);
        (noise_metadata_schedule_321_e3075,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_321_e3077;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_322_e3085,) = {
    if (noise_variable_528 != 0.0) {
        let noise_metadata_schedule_322_e3082: f64 = (params.p62 + noise_variable_118);
        let noise_metadata_schedule_322_e3083: f64 = (params.p62 / noise_metadata_schedule_322_e3082);
        (noise_metadata_schedule_322_e3083,)
    } else {
        (noise_variable_213,)
    }
};
            noise_variable_213 = noise_metadata_schedule_322_e3085;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_324_e3101,) = {
    if (noise_variable_528 == 0.0) {
        let noise_metadata_schedule_324_e3095: f64 = (2.0 * noise_variable_277);
        let noise_metadata_schedule_324_e3098: f64 = (1.0 + noise_variable_114);
        let noise_metadata_schedule_324_e3099: f64 = (noise_metadata_schedule_324_e3095 / noise_metadata_schedule_324_e3098);
        (noise_metadata_schedule_324_e3099,)
    } else {
        (noise_variable_129,)
    }
};
            noise_variable_129 = noise_metadata_schedule_324_e3101;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_325_e3106,) = {
    if (noise_variable_528 == 0.0) {
        (noise_variable_271,)
    } else {
        (noise_variable_131,)
    }
};
            noise_variable_131 = noise_metadata_schedule_325_e3106;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_326_e3108: f64 = (noise_variable_256).abs();
            let noise_metadata_schedule_326_e3111: f64 = (1e-5 * noise_variable_6);
            let noise_metadata_schedule_326_e3114: f64 = (noise_variable_117).abs();
            let noise_metadata_schedule_326_e3117: f64 = (1e-40 * noise_variable_6);
            let noise_metadata_schedule_326_e3120: f64 = (noise_variable_114 + noise_variable_115);
            let noise_metadata_schedule_326_e3121: f64 = (noise_metadata_schedule_326_e3117 * noise_metadata_schedule_326_e3120);
            let noise_metadata_schedule_326_e3123: f64 = if ((noise_metadata_schedule_326_e3108 < noise_metadata_schedule_326_e3111) || (noise_metadata_schedule_326_e3114 < noise_metadata_schedule_326_e3121)) { 1.0 } else { 0.0 };
            noise_variable_535 = noise_metadata_schedule_326_e3123;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_327_e3134,) = {
    if ((noise_variable_528 == 0.0) && (noise_variable_535 != 0.0)) {
        let noise_metadata_schedule_327_e3131: f64 = (noise_variable_129 + noise_variable_116);
        let noise_metadata_schedule_327_e3132: f64 = (0.5 * noise_metadata_schedule_327_e3131);
        (noise_metadata_schedule_327_e3132,)
    } else {
        (noise_variable_138,)
    }
};
            noise_variable_138 = noise_metadata_schedule_327_e3134;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_328_e3145,) = {
    if ((noise_variable_528 == 0.0) && (noise_variable_535 != 0.0)) {
        let noise_metadata_schedule_328_e3142: f64 = (noise_variable_138 + 1.0);
        let noise_metadata_schedule_328_e3143: f64 = (noise_variable_138 / noise_metadata_schedule_328_e3142);
        (noise_metadata_schedule_328_e3143,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_328_e3145;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_329_e3159,) = {
    if ((noise_variable_528 == 0.0) && (noise_variable_535 == 0.0)) {
        let noise_metadata_schedule_329_e3154: f64 = (noise_variable_117 + noise_variable_251);
        let noise_metadata_schedule_329_e3156: f64 = (noise_metadata_schedule_329_e3154 - noise_variable_250);
        let noise_metadata_schedule_329_e3157: f64 = (noise_variable_117 / noise_metadata_schedule_329_e3156);
        (noise_metadata_schedule_329_e3157,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_329_e3159;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_330_e3164,) = {
    if (noise_variable_528 == 0.0) {
        (noise_variable_256,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_330_e3164;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_331_e3171,) = {
    if (noise_variable_528 == 0.0) {
        let noise_metadata_schedule_331_e3169: f64 = (0.1 * noise_variable_17);
        (noise_metadata_schedule_331_e3169,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_331_e3171;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_332_e3176,) = {
    if (noise_variable_528 == 0.0) {
        (noise_variable_118,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_332_e3176;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_333_e3185,) = {
    if (noise_variable_528 == 0.0) {
        let noise_metadata_schedule_333_e3182: f64 = (noise_variable_137 / params.p62);
        let noise_metadata_schedule_333_e3183: f64 = (1.0 - noise_metadata_schedule_333_e3182);
        (noise_metadata_schedule_333_e3183,)
    } else {
        (noise_variable_213,)
    }
};
            noise_variable_213 = noise_metadata_schedule_333_e3185;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_334_e3190: f64 = (-1.0);
            let noise_metadata_schedule_334_e3192: f64 = (noise_metadata_schedule_334_e3190 / params.p67);
            let noise_metadata_schedule_334_e3193: f64 = (3.0_f64).powf(noise_metadata_schedule_334_e3192);
            let noise_metadata_schedule_334_e3194: f64 = (1.0 - noise_metadata_schedule_334_e3193);
            let noise_metadata_schedule_334_e3195: f64 = (noise_variable_14 * noise_metadata_schedule_334_e3194);
            noise_variable_139 = noise_metadata_schedule_334_e3195;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_335_e3198: f64 = (0.1 * noise_variable_14);
            noise_variable_299 = noise_metadata_schedule_335_e3198;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_336_e3201: f64 = (noise_variable_252 - noise_variable_139);
            let noise_metadata_schedule_336_e3203: f64 = (noise_metadata_schedule_336_e3201 / noise_variable_299);
            noise_variable_285 = noise_metadata_schedule_336_e3203;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_337_e3206: f64 = if noise_variable_252 < noise_variable_139 { 1.0 } else { 0.0 };
            noise_variable_536 = noise_metadata_schedule_337_e3206;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_338_e3218,) = {
    if (noise_variable_536 != 0.0) {
        let noise_metadata_schedule_338_e3212: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_338_e3213: f64 = (1.0 + noise_metadata_schedule_338_e3212);
        let noise_metadata_schedule_338_e3214: f64 = (noise_metadata_schedule_338_e3213).ln();
        let noise_metadata_schedule_338_e3215: f64 = (noise_variable_299 * noise_metadata_schedule_338_e3214);
        let noise_metadata_schedule_338_e3216: f64 = (noise_variable_252 - noise_metadata_schedule_338_e3215);
        (noise_metadata_schedule_338_e3216,)
    } else {
        (noise_variable_140,)
    }
};
            noise_variable_140 = noise_metadata_schedule_338_e3218;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_339_e3232,) = {
    if (noise_variable_536 == 0.0) {
        let noise_metadata_schedule_339_e3225: f64 = (-noise_variable_285);
        let noise_metadata_schedule_339_e3226: f64 = (noise_metadata_schedule_339_e3225).exp();
        let noise_metadata_schedule_339_e3227: f64 = (1.0 + noise_metadata_schedule_339_e3226);
        let noise_metadata_schedule_339_e3228: f64 = (noise_metadata_schedule_339_e3227).ln();
        let noise_metadata_schedule_339_e3229: f64 = (noise_variable_299 * noise_metadata_schedule_339_e3228);
        let noise_metadata_schedule_339_e3230: f64 = (noise_variable_139 - noise_metadata_schedule_339_e3229);
        (noise_metadata_schedule_339_e3230,)
    } else {
        (noise_variable_140,)
    }
};
            noise_variable_140 = noise_metadata_schedule_339_e3232;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_340_e3236: f64 = (noise_variable_140 * noise_variable_65);
            let noise_metadata_schedule_340_e3237: f64 = (1.0 - noise_metadata_schedule_340_e3236);
            let noise_metadata_schedule_340_e3240: f64 = (1.0 - params.p67);
            let noise_metadata_schedule_340_e3241: f64 = (noise_metadata_schedule_340_e3237).powf(noise_metadata_schedule_340_e3240);
            noise_variable_59 = noise_metadata_schedule_340_e3241;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_341_e3245: f64 = (1.0 - params.p67);
            let noise_metadata_schedule_341_e3246: f64 = (noise_variable_14 / noise_metadata_schedule_341_e3245);
            let noise_metadata_schedule_341_e3249: f64 = (1.0 - noise_variable_59);
            let noise_metadata_schedule_341_e3250: f64 = (noise_metadata_schedule_341_e3246 * noise_metadata_schedule_341_e3249);
            let noise_metadata_schedule_341_e3254: f64 = (noise_variable_252 - noise_variable_140);
            let noise_metadata_schedule_341_e3255: f64 = (3.0 * noise_metadata_schedule_341_e3254);
            let noise_metadata_schedule_341_e3256: f64 = (noise_metadata_schedule_341_e3250 + noise_metadata_schedule_341_e3255);
            noise_variable_141 = noise_metadata_schedule_341_e3256;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_342_e3259: f64 = if params.p74 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_537 = noise_metadata_schedule_342_e3259;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_343_e3263,) = {
    if (noise_variable_537 != 0.0) {
        (noise_variable_250,)
    } else {
        (noise_variable_142,)
    }
};
            noise_variable_142 = noise_metadata_schedule_343_e3263;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_344_e3266: f64 = if params.p74 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_538 = noise_metadata_schedule_344_e3266;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_345_e3275,) = {
    if ((noise_variable_537 == 0.0) && (noise_variable_538 != 0.0)) {
        let noise_metadata_schedule_345_e3273: f64 = (noise_variable_250 + noise_variable_135);
        (noise_metadata_schedule_345_e3273,)
    } else {
        (noise_variable_142,)
    }
};
            noise_variable_142 = noise_metadata_schedule_345_e3275;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_346_e3283,) = {
    if ((noise_variable_537 == 0.0) && (noise_variable_538 == 0.0)) {
        (noise_variable_251,)
    } else {
        (noise_variable_142,)
    }
};
            noise_variable_142 = noise_metadata_schedule_346_e3283;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_347_e3286: f64 = (2.0 - noise_variable_25);
            let noise_metadata_schedule_347_e3289: f64 = (1.0 - noise_variable_25);
            let noise_metadata_schedule_347_e3290: f64 = (noise_metadata_schedule_347_e3286 / noise_metadata_schedule_347_e3289);
            noise_variable_143 = noise_metadata_schedule_347_e3290;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_348_e3295: f64 = (-1.0);
            let noise_metadata_schedule_348_e3297: f64 = (noise_metadata_schedule_348_e3295 / params.p72);
            let noise_metadata_schedule_348_e3298: f64 = (noise_variable_143).powf(noise_metadata_schedule_348_e3297);
            let noise_metadata_schedule_348_e3299: f64 = (1.0 - noise_metadata_schedule_348_e3298);
            let noise_metadata_schedule_348_e3300: f64 = (noise_variable_17 * noise_metadata_schedule_348_e3299);
            noise_variable_144 = noise_metadata_schedule_348_e3300;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_349_e3303: f64 = (noise_variable_142 - noise_variable_144);
            let noise_metadata_schedule_349_e3305: f64 = (noise_metadata_schedule_349_e3303 / noise_variable_136);
            noise_variable_285 = noise_metadata_schedule_349_e3305;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_350_e3308: f64 = if noise_variable_142 < noise_variable_144 { 1.0 } else { 0.0 };
            noise_variable_539 = noise_metadata_schedule_350_e3308;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_351_e3320,) = {
    if (noise_variable_539 != 0.0) {
        let noise_metadata_schedule_351_e3314: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_351_e3315: f64 = (1.0 + noise_metadata_schedule_351_e3314);
        let noise_metadata_schedule_351_e3316: f64 = (noise_metadata_schedule_351_e3315).ln();
        let noise_metadata_schedule_351_e3317: f64 = (noise_variable_136 * noise_metadata_schedule_351_e3316);
        let noise_metadata_schedule_351_e3318: f64 = (noise_variable_142 - noise_metadata_schedule_351_e3317);
        (noise_metadata_schedule_351_e3318,)
    } else {
        (noise_variable_145,)
    }
};
            noise_variable_145 = noise_metadata_schedule_351_e3320;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_352_e3334,) = {
    if (noise_variable_539 == 0.0) {
        let noise_metadata_schedule_352_e3327: f64 = (-noise_variable_285);
        let noise_metadata_schedule_352_e3328: f64 = (noise_metadata_schedule_352_e3327).exp();
        let noise_metadata_schedule_352_e3329: f64 = (1.0 + noise_metadata_schedule_352_e3328);
        let noise_metadata_schedule_352_e3330: f64 = (noise_metadata_schedule_352_e3329).ln();
        let noise_metadata_schedule_352_e3331: f64 = (noise_variable_136 * noise_metadata_schedule_352_e3330);
        let noise_metadata_schedule_352_e3332: f64 = (noise_variable_144 - noise_metadata_schedule_352_e3331);
        (noise_metadata_schedule_352_e3332,)
    } else {
        (noise_variable_145,)
    }
};
            noise_variable_145 = noise_metadata_schedule_352_e3334;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_353_e3337: f64 = (noise_variable_213).powf(params.p76);
            noise_variable_146 = noise_metadata_schedule_353_e3337;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_354_e3341: f64 = (1.0 - params.p72);
            let noise_metadata_schedule_354_e3342: f64 = (noise_variable_17 / noise_metadata_schedule_354_e3341);
            let noise_metadata_schedule_354_e3348: f64 = (noise_variable_145 / noise_variable_17);
            let noise_metadata_schedule_354_e3349: f64 = (1.0 - noise_metadata_schedule_354_e3348);
            let noise_metadata_schedule_354_e3352: f64 = (1.0 - params.p72);
            let noise_metadata_schedule_354_e3353: f64 = (noise_metadata_schedule_354_e3349).powf(noise_metadata_schedule_354_e3352);
            let noise_metadata_schedule_354_e3354: f64 = (noise_variable_146 * noise_metadata_schedule_354_e3353);
            let noise_metadata_schedule_354_e3355: f64 = (1.0 - noise_metadata_schedule_354_e3354);
            let noise_metadata_schedule_354_e3356: f64 = (noise_metadata_schedule_354_e3342 * noise_metadata_schedule_354_e3355);
            let noise_metadata_schedule_354_e3359: f64 = (noise_variable_146 * noise_variable_143);
            let noise_metadata_schedule_354_e3362: f64 = (noise_variable_142 - noise_variable_145);
            let noise_metadata_schedule_354_e3363: f64 = (noise_metadata_schedule_354_e3359 * noise_metadata_schedule_354_e3362);
            let noise_metadata_schedule_354_e3364: f64 = (noise_metadata_schedule_354_e3356 + noise_metadata_schedule_354_e3363);
            noise_variable_147 = noise_metadata_schedule_354_e3364;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_355_e3367: f64 = (1.0 - noise_variable_25);
            let noise_metadata_schedule_355_e3369: f64 = (noise_metadata_schedule_355_e3367 * noise_variable_147);
            let noise_metadata_schedule_355_e3372: f64 = (noise_variable_25 * noise_variable_250);
            let noise_metadata_schedule_355_e3373: f64 = (noise_metadata_schedule_355_e3369 + noise_metadata_schedule_355_e3372);
            noise_variable_148 = noise_metadata_schedule_355_e3373;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_356_e3376: f64 = (4.0 * noise_variable_35);
            let noise_metadata_schedule_356_e3378: f64 = (noise_metadata_schedule_356_e3376 / noise_variable_36);
            noise_variable_149 = noise_metadata_schedule_356_e3378;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_357_e3381: f64 = (noise_variable_149 * noise_variable_272);
            noise_variable_150 = noise_metadata_schedule_357_e3381;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_358_e3386: f64 = (1.0 + noise_variable_150);
            let noise_metadata_schedule_358_e3387: f64 = (noise_metadata_schedule_358_e3386).sqrt();
            let noise_metadata_schedule_358_e3388: f64 = (1.0 + noise_metadata_schedule_358_e3387);
            let noise_metadata_schedule_358_e3389: f64 = (noise_variable_150 / noise_metadata_schedule_358_e3388);
            noise_variable_152 = noise_metadata_schedule_358_e3389;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_359_e3393: f64 = (1.0 / noise_variable_49);
            let noise_metadata_schedule_359_e3394: f64 = (noise_variable_131).powf(noise_metadata_schedule_359_e3393);
            noise_variable_132 = noise_metadata_schedule_359_e3394;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_360_e3397: f64 = (noise_variable_149 * noise_variable_132);
            noise_variable_151 = noise_metadata_schedule_360_e3397;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_361_e3402: f64 = (1.0 + noise_variable_151);
            let noise_metadata_schedule_361_e3403: f64 = (noise_metadata_schedule_361_e3402).sqrt();
            let noise_metadata_schedule_361_e3404: f64 = (1.0 + noise_metadata_schedule_361_e3403);
            let noise_metadata_schedule_361_e3405: f64 = (noise_variable_151 / noise_metadata_schedule_361_e3404);
            noise_variable_153 = noise_metadata_schedule_361_e3405;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_362_e3408: f64 = if params.p92 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_540 = noise_metadata_schedule_362_e3408;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_363_e3420,) = {
    if (noise_variable_540 != 0.0) {
        let noise_metadata_schedule_363_e3413: f64 = (noise_variable_141 / noise_variable_41);
        let noise_metadata_schedule_363_e3414: f64 = (1.0 + noise_metadata_schedule_363_e3413);
        let noise_metadata_schedule_363_e3417: f64 = (noise_variable_148 / noise_variable_40);
        let noise_metadata_schedule_363_e3418: f64 = (noise_metadata_schedule_363_e3414 + noise_metadata_schedule_363_e3417);
        (noise_metadata_schedule_363_e3418,)
    } else {
        (noise_variable_154,)
    }
};
            noise_variable_154 = noise_metadata_schedule_363_e3420;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_364_e3433,) = {
    if (noise_variable_540 == 0.0) {
        let noise_metadata_schedule_364_e3425: f64 = (noise_variable_141 / noise_variable_41);
        let noise_metadata_schedule_364_e3427: f64 = (noise_metadata_schedule_364_e3425 + 1.0);
        let noise_metadata_schedule_364_e3429: f64 = (noise_metadata_schedule_364_e3427 * noise_variable_100);
        let noise_metadata_schedule_364_e3431: f64 = (noise_metadata_schedule_364_e3429 * noise_variable_8);
        (noise_metadata_schedule_364_e3431,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_364_e3433;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_365_e3445,) = {
    if (noise_variable_540 == 0.0) {
        let noise_metadata_schedule_365_e3437: f64 = (-noise_variable_148);
        let noise_metadata_schedule_365_e3439: f64 = (noise_metadata_schedule_365_e3437 / noise_variable_40);
        let noise_metadata_schedule_365_e3441: f64 = (noise_metadata_schedule_365_e3439 * noise_variable_100);
        let noise_metadata_schedule_365_e3443: f64 = (noise_metadata_schedule_365_e3441 * noise_variable_8);
        (noise_metadata_schedule_365_e3443,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_365_e3445;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_366_e3461,) = {
    if (noise_variable_540 == 0.0) {
        let noise_metadata_schedule_366_e3449: f64 = (noise_variable_295).exp();
        let noise_metadata_schedule_366_e3451: f64 = (noise_variable_296).exp();
        let noise_metadata_schedule_366_e3452: f64 = (noise_metadata_schedule_366_e3449 - noise_metadata_schedule_366_e3451);
        let noise_metadata_schedule_366_e3455: f64 = (noise_variable_100 * noise_variable_8);
        let noise_metadata_schedule_366_e3456: f64 = (noise_metadata_schedule_366_e3455).exp();
        let noise_metadata_schedule_366_e3458: f64 = (noise_metadata_schedule_366_e3456 - 1.0);
        let noise_metadata_schedule_366_e3459: f64 = (noise_metadata_schedule_366_e3452 / noise_metadata_schedule_366_e3458);
        (noise_metadata_schedule_366_e3459,)
    } else {
        (noise_variable_154,)
    }
};
            noise_variable_154 = noise_metadata_schedule_366_e3461;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_367_e3464: f64 = (0.1 * 0.1);
            noise_variable_287 = noise_metadata_schedule_367_e3464;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_368_e3467: f64 = (noise_variable_154 * noise_variable_154);
            noise_variable_288 = noise_metadata_schedule_368_e3467;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_369_e3470: f64 = if noise_variable_154 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_541 = noise_metadata_schedule_369_e3470;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_370_e3483,) = {
    if (noise_variable_541 != 0.0) {
        let noise_metadata_schedule_370_e3474: f64 = (0.5 * noise_variable_287);
        let noise_metadata_schedule_370_e3477: f64 = (noise_variable_288 + noise_variable_287);
        let noise_metadata_schedule_370_e3478: f64 = (noise_metadata_schedule_370_e3477).sqrt();
        let noise_metadata_schedule_370_e3480: f64 = (noise_metadata_schedule_370_e3478 - noise_variable_154);
        let noise_metadata_schedule_370_e3481: f64 = (noise_metadata_schedule_370_e3474 / noise_metadata_schedule_370_e3480);
        (noise_metadata_schedule_370_e3481,)
    } else {
        (noise_variable_155,)
    }
};
            noise_variable_155 = noise_metadata_schedule_370_e3483;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_371_e3495,) = {
    if (noise_variable_541 == 0.0) {
        let noise_metadata_schedule_371_e3489: f64 = (noise_variable_288 + noise_variable_287);
        let noise_metadata_schedule_371_e3490: f64 = (noise_metadata_schedule_371_e3489).sqrt();
        let noise_metadata_schedule_371_e3492: f64 = (noise_metadata_schedule_371_e3490 + noise_variable_154);
        let noise_metadata_schedule_371_e3493: f64 = (0.5 * noise_metadata_schedule_371_e3492);
        (noise_metadata_schedule_371_e3493,)
    } else {
        (noise_variable_155,)
    }
};
            noise_variable_155 = noise_metadata_schedule_371_e3495;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_372_e3501: f64 = (noise_variable_152 + noise_variable_153);
            let noise_metadata_schedule_372_e3502: f64 = (0.5 * noise_metadata_schedule_372_e3501);
            let noise_metadata_schedule_372_e3503: f64 = (1.0 + noise_metadata_schedule_372_e3502);
            let noise_metadata_schedule_372_e3504: f64 = (noise_variable_155 * noise_metadata_schedule_372_e3503);
            noise_variable_156 = noise_metadata_schedule_372_e3504;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_373_e3507: f64 = (params.p15 * noise_variable_35);
            let noise_metadata_schedule_373_e3509: f64 = (noise_metadata_schedule_373_e3507 * noise_variable_132);
            noise_variable_157 = noise_metadata_schedule_373_e3509;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_374_e3512: f64 = (noise_variable_35 * noise_variable_272);
            noise_variable_158 = noise_metadata_schedule_374_e3512;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_375_e3515: f64 = (noise_variable_158 - noise_variable_157);
            let noise_metadata_schedule_375_e3517: f64 = (noise_metadata_schedule_375_e3515 / noise_variable_156);
            noise_variable_159 = noise_metadata_schedule_375_e3517;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_376_e3520: f64 = noise_variable_252;
            let noise_metadata_schedule_376_e3522: f64 = (noise_metadata_schedule_376_e3520 / 0.0001);
            noise_variable_285 = noise_metadata_schedule_376_e3522;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_377_e3525: f64 = if noise_variable_252 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_542 = noise_metadata_schedule_377_e3525;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_378_e3537,) = {
    if (noise_variable_542 != 0.0) {
        let noise_metadata_schedule_378_e3531: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_378_e3532: f64 = (1.0 + noise_metadata_schedule_378_e3531);
        let noise_metadata_schedule_378_e3533: f64 = (noise_metadata_schedule_378_e3532).ln();
        let noise_metadata_schedule_378_e3534: f64 = (0.0001 * noise_metadata_schedule_378_e3533);
        let noise_metadata_schedule_378_e3535: f64 = noise_metadata_schedule_378_e3534;
        (noise_metadata_schedule_378_e3535,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_378_e3537;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_379_e3551,) = {
    if (noise_variable_542 == 0.0) {
        let noise_metadata_schedule_379_e3544: f64 = (-noise_variable_285);
        let noise_metadata_schedule_379_e3545: f64 = (noise_metadata_schedule_379_e3544).exp();
        let noise_metadata_schedule_379_e3546: f64 = (1.0 + noise_metadata_schedule_379_e3545);
        let noise_metadata_schedule_379_e3547: f64 = (noise_metadata_schedule_379_e3546).ln();
        let noise_metadata_schedule_379_e3548: f64 = (0.0001 * noise_metadata_schedule_379_e3547);
        let noise_metadata_schedule_379_e3549: f64 = (noise_variable_252 + noise_metadata_schedule_379_e3548);
        (noise_metadata_schedule_379_e3549,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_379_e3551;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_380_e3554: f64 = (noise_variable_302 / params.p156);
            noise_variable_304 = noise_metadata_schedule_380_e3554;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_381_e3557: f64 = if noise_variable_304 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_543 = noise_metadata_schedule_381_e3557;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_382_e3562,) = {
    if (noise_variable_543 != 0.0) {
        let noise_metadata_schedule_382_e3560: f64 = (noise_variable_304).exp();
        (noise_metadata_schedule_382_e3560,)
    } else {
        (noise_variable_305,)
    }
};
            noise_variable_305 = noise_metadata_schedule_382_e3562;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_383_e3568,) = {
    if (noise_variable_543 == 0.0) {
        let noise_metadata_schedule_383_e3566: f64 = (params.p151).exp();
        (noise_metadata_schedule_383_e3566,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_383_e3568;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_384_e3579,) = {
    if (noise_variable_543 == 0.0) {
        let noise_metadata_schedule_384_e3575: f64 = (noise_variable_304 - params.p151);
        let noise_metadata_schedule_384_e3576: f64 = (1.0 + noise_metadata_schedule_384_e3575);
        let noise_metadata_schedule_384_e3577: f64 = (noise_variable_301 * noise_metadata_schedule_384_e3576);
        (noise_metadata_schedule_384_e3577,)
    } else {
        (noise_variable_305,)
    }
};
            noise_variable_305 = noise_metadata_schedule_384_e3579;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_385_e3583: f64 = (noise_variable_305 - 1.0);
            let noise_metadata_schedule_385_e3584: f64 = (noise_variable_357 * noise_metadata_schedule_385_e3583);
            noise_variable_358 = noise_metadata_schedule_385_e3584;
        }
        if matches!(source_index, 1 | 2) {
            let noise_metadata_schedule_386_e3587: f64 = (noise_variable_252 - params.p158);
            let noise_metadata_schedule_386_e3589: f64 = (noise_metadata_schedule_386_e3587 / 0.001);
            noise_variable_285 = noise_metadata_schedule_386_e3589;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_387_e3592: f64 = if noise_variable_252 < params.p158 { 1.0 } else { 0.0 };
            noise_variable_544 = noise_metadata_schedule_387_e3592;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_388_e3604,) = {
    if (noise_variable_544 != 0.0) {
        let noise_metadata_schedule_388_e3598: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_388_e3599: f64 = (1.0 + noise_metadata_schedule_388_e3598);
        let noise_metadata_schedule_388_e3600: f64 = (noise_metadata_schedule_388_e3599).ln();
        let noise_metadata_schedule_388_e3601: f64 = (0.001 * noise_metadata_schedule_388_e3600);
        let noise_metadata_schedule_388_e3602: f64 = (noise_variable_252 - noise_metadata_schedule_388_e3601);
        (noise_metadata_schedule_388_e3602,)
    } else {
        (noise_variable_306,)
    }
};
            noise_variable_306 = noise_metadata_schedule_388_e3604;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_389_e3618,) = {
    if (noise_variable_544 == 0.0) {
        let noise_metadata_schedule_389_e3611: f64 = (-noise_variable_285);
        let noise_metadata_schedule_389_e3612: f64 = (noise_metadata_schedule_389_e3611).exp();
        let noise_metadata_schedule_389_e3613: f64 = (1.0 + noise_metadata_schedule_389_e3612);
        let noise_metadata_schedule_389_e3614: f64 = (noise_metadata_schedule_389_e3613).ln();
        let noise_metadata_schedule_389_e3615: f64 = (0.001 * noise_metadata_schedule_389_e3614);
        let noise_metadata_schedule_389_e3616: f64 = (params.p158 - noise_metadata_schedule_389_e3615);
        (noise_metadata_schedule_389_e3616,)
    } else {
        (noise_variable_306,)
    }
};
            noise_variable_306 = noise_metadata_schedule_389_e3618;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_390_e3621: f64 = (params.p159 * noise_variable_306);
            let noise_metadata_schedule_390_e3624: f64 = (params.p158 - noise_variable_306);
            let noise_metadata_schedule_390_e3626: f64 = {let pb=noise_metadata_schedule_390_e3624;pb*pb};
            let noise_metadata_schedule_390_e3627: f64 = (noise_metadata_schedule_390_e3621 * noise_metadata_schedule_390_e3626);
            noise_variable_359 = noise_metadata_schedule_390_e3627;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_391_e3630: f64 = (noise_variable_252 * noise_variable_8);
            let noise_metadata_schedule_391_e3632: f64 = (noise_metadata_schedule_391_e3630 / params.p17);
            let noise_metadata_schedule_391_e3634: f64 = if noise_metadata_schedule_391_e3632 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_545 = noise_metadata_schedule_391_e3634;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_392_e3643,) = {
    if (noise_variable_545 != 0.0) {
        let noise_metadata_schedule_392_e3638: f64 = (noise_variable_252 * noise_variable_8);
        let noise_metadata_schedule_392_e3640: f64 = (noise_metadata_schedule_392_e3638 / params.p17);
        let noise_metadata_schedule_392_e3641: f64 = (noise_metadata_schedule_392_e3640).exp();
        (noise_metadata_schedule_392_e3641,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_392_e3643;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_393_e3649,) = {
    if (noise_variable_545 == 0.0) {
        let noise_metadata_schedule_393_e3647: f64 = (params.p151).exp();
        (noise_metadata_schedule_393_e3647,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_393_e3649;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_394_e3664,) = {
    if (noise_variable_545 == 0.0) {
        let noise_metadata_schedule_394_e3656: f64 = (noise_variable_252 * noise_variable_8);
        let noise_metadata_schedule_394_e3658: f64 = (noise_metadata_schedule_394_e3656 / params.p17);
        let noise_metadata_schedule_394_e3660: f64 = (noise_metadata_schedule_394_e3658 - params.p151);
        let noise_metadata_schedule_394_e3661: f64 = (1.0 + noise_metadata_schedule_394_e3660);
        let noise_metadata_schedule_394_e3662: f64 = (noise_variable_301 * noise_metadata_schedule_394_e3661);
        (noise_metadata_schedule_394_e3662,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_394_e3664;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_395_e3667: f64 = if params.p24 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_546 = noise_metadata_schedule_395_e3667;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_396_e3670: f64 = (noise_variable_252 - noise_variable_55);
            let noise_metadata_schedule_396_e3672: f64 = (noise_metadata_schedule_396_e3670 * noise_variable_8);
            let noise_metadata_schedule_396_e3674: f64 = if noise_metadata_schedule_396_e3672 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_547 = noise_metadata_schedule_396_e3674;
        }
        if matches!(source_index, 2 | 6 | 8) {
            let (noise_metadata_schedule_397_e3685,) = {
    if ((noise_variable_546 != 0.0) && (noise_variable_547 != 0.0)) {
        let noise_metadata_schedule_397_e3680: f64 = (noise_variable_252 - noise_variable_55);
        let noise_metadata_schedule_397_e3682: f64 = (noise_metadata_schedule_397_e3680 * noise_variable_8);
        let noise_metadata_schedule_397_e3683: f64 = (noise_metadata_schedule_397_e3682).exp();
        (noise_metadata_schedule_397_e3683,)
    } else {
        (noise_variable_304,)
    }
};
            noise_variable_304 = noise_metadata_schedule_397_e3685;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_398_e3693,) = {
    if ((noise_variable_546 != 0.0) && (noise_variable_547 == 0.0)) {
        let noise_metadata_schedule_398_e3691: f64 = (params.p151).exp();
        (noise_metadata_schedule_398_e3691,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_398_e3693;
        }
        if matches!(source_index, 2 | 6 | 8) {
            let (noise_metadata_schedule_399_e3710,) = {
    if ((noise_variable_546 != 0.0) && (noise_variable_547 == 0.0)) {
        let noise_metadata_schedule_399_e3702: f64 = (noise_variable_252 - noise_variable_55);
        let noise_metadata_schedule_399_e3704: f64 = (noise_metadata_schedule_399_e3702 * noise_variable_8);
        let noise_metadata_schedule_399_e3706: f64 = (noise_metadata_schedule_399_e3704 - params.p151);
        let noise_metadata_schedule_399_e3707: f64 = (1.0 + noise_metadata_schedule_399_e3706);
        let noise_metadata_schedule_399_e3708: f64 = (noise_variable_301 * noise_metadata_schedule_399_e3707);
        (noise_metadata_schedule_399_e3708,)
    } else {
        (noise_variable_304,)
    }
};
            noise_variable_304 = noise_metadata_schedule_399_e3710;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_400_e3713: f64 = (noise_variable_159 / noise_variable_35);
            let noise_metadata_schedule_400_e3715: f64 = (noise_metadata_schedule_400_e3713 - 1000.0);
            let noise_metadata_schedule_400_e3717: f64 = if noise_metadata_schedule_400_e3715 < 40.0 { 1.0 } else { 0.0 };
            noise_variable_548 = noise_metadata_schedule_400_e3717;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_401_e3728,) = {
    if ((noise_variable_546 != 0.0) && (noise_variable_548 != 0.0)) {
        let noise_metadata_schedule_401_e3723: f64 = (noise_variable_159 / noise_variable_35);
        let noise_metadata_schedule_401_e3725: f64 = (noise_metadata_schedule_401_e3723 - 1000.0);
        let noise_metadata_schedule_401_e3726: f64 = (noise_metadata_schedule_401_e3725).exp();
        (noise_metadata_schedule_401_e3726,)
    } else {
        (noise_variable_305,)
    }
};
            noise_variable_305 = noise_metadata_schedule_401_e3728;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_402_e3736,) = {
    if ((noise_variable_546 != 0.0) && (noise_variable_548 == 0.0)) {
        let noise_metadata_schedule_402_e3734: f64 = (40.0_f64).exp();
        (noise_metadata_schedule_402_e3734,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_402_e3736;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_403_e3753,) = {
    if ((noise_variable_546 != 0.0) && (noise_variable_548 == 0.0)) {
        let noise_metadata_schedule_403_e3745: f64 = (noise_variable_159 / noise_variable_35);
        let noise_metadata_schedule_403_e3747: f64 = (noise_metadata_schedule_403_e3745 - 1000.0);
        let noise_metadata_schedule_403_e3749: f64 = (noise_metadata_schedule_403_e3747 - 40.0);
        let noise_metadata_schedule_403_e3750: f64 = (1.0 + noise_metadata_schedule_403_e3749);
        let noise_metadata_schedule_403_e3751: f64 = (noise_variable_301 * noise_metadata_schedule_403_e3750);
        (noise_metadata_schedule_403_e3751,)
    } else {
        (noise_variable_305,)
    }
};
            noise_variable_305 = noise_metadata_schedule_403_e3753;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_404_e3796,) = {
    if (noise_variable_546 != 0.0) {
        let noise_metadata_schedule_404_e3758: f64 = (noise_variable_302 - 1.0);
        let noise_metadata_schedule_404_e3759: f64 = (noise_variable_42 * noise_metadata_schedule_404_e3758);
        let noise_metadata_schedule_404_e3762: f64 = (noise_variable_53 * 2.0);
        let noise_metadata_schedule_404_e3765: f64 = (noise_variable_302 - 1.0);
        let noise_metadata_schedule_404_e3766: f64 = (noise_metadata_schedule_404_e3762 * noise_metadata_schedule_404_e3765);
        let noise_metadata_schedule_404_e3771: f64 = (4.0 * noise_variable_304);
        let noise_metadata_schedule_404_e3772: f64 = (1.0 + noise_metadata_schedule_404_e3771);
        let noise_metadata_schedule_404_e3773: f64 = (noise_metadata_schedule_404_e3772).sqrt();
        let noise_metadata_schedule_404_e3774: f64 = (1.0 + noise_metadata_schedule_404_e3773);
        let noise_metadata_schedule_404_e3775: f64 = (noise_metadata_schedule_404_e3766 / noise_metadata_schedule_404_e3774);
        let noise_metadata_schedule_404_e3779: f64 = (noise_variable_148 / noise_variable_40);
        let noise_metadata_schedule_404_e3780: f64 = (1.0 + noise_metadata_schedule_404_e3779);
        let noise_metadata_schedule_404_e3781: f64 = (noise_metadata_schedule_404_e3775 * noise_metadata_schedule_404_e3780);
        let noise_metadata_schedule_404_e3782: f64 = (noise_metadata_schedule_404_e3759 + noise_metadata_schedule_404_e3781);
        let noise_metadata_schedule_404_e3786: f64 = (noise_variable_131 - 1.0);
        let noise_metadata_schedule_404_e3787: f64 = (noise_variable_54 * noise_metadata_schedule_404_e3786);
        let noise_metadata_schedule_404_e3789: f64 = (noise_metadata_schedule_404_e3787 * noise_variable_305);
        let noise_metadata_schedule_404_e3792: f64 = (1.0 + noise_variable_305);
        let noise_metadata_schedule_404_e3793: f64 = (noise_metadata_schedule_404_e3789 / noise_metadata_schedule_404_e3792);
        let noise_metadata_schedule_404_e3794: f64 = (noise_metadata_schedule_404_e3782 + noise_metadata_schedule_404_e3793);
        (noise_metadata_schedule_404_e3794,)
    } else {
        (noise_variable_161,)
    }
};
            noise_variable_161 = noise_metadata_schedule_404_e3796;
        }
        if matches!(source_index, 2 | 6) {
            let noise_metadata_schedule_405_e3799: f64 = if params.p93 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_549 = noise_metadata_schedule_405_e3799;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_406_e3810,) = {
    if ((noise_variable_546 == 0.0) && (noise_variable_549 != 0.0)) {
        let noise_metadata_schedule_406_e3807: f64 = (noise_variable_302 - 1.0);
        let noise_metadata_schedule_406_e3808: f64 = (noise_variable_42 * noise_metadata_schedule_406_e3807);
        (noise_metadata_schedule_406_e3808,)
    } else {
        (noise_variable_161,)
    }
};
            noise_variable_161 = noise_metadata_schedule_406_e3810;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_407_e3840,) = {
    if ((noise_variable_546 == 0.0) && (noise_variable_549 == 0.0)) {
        let noise_metadata_schedule_407_e3819: f64 = (1.0 - params.p93);
        let noise_metadata_schedule_407_e3822: f64 = (noise_variable_302 - 1.0);
        let noise_metadata_schedule_407_e3823: f64 = (noise_metadata_schedule_407_e3819 * noise_metadata_schedule_407_e3822);
        let noise_metadata_schedule_407_e3827: f64 = (noise_variable_302 + noise_variable_131);
        let noise_metadata_schedule_407_e3829: f64 = (noise_metadata_schedule_407_e3827 - 2.0);
        let noise_metadata_schedule_407_e3830: f64 = (params.p93 * noise_metadata_schedule_407_e3829);
        let noise_metadata_schedule_407_e3834: f64 = (noise_variable_148 / noise_variable_40);
        let noise_metadata_schedule_407_e3835: f64 = (1.0 + noise_metadata_schedule_407_e3834);
        let noise_metadata_schedule_407_e3836: f64 = (noise_metadata_schedule_407_e3830 * noise_metadata_schedule_407_e3835);
        let noise_metadata_schedule_407_e3837: f64 = (noise_metadata_schedule_407_e3823 + noise_metadata_schedule_407_e3836);
        let noise_metadata_schedule_407_e3838: f64 = (noise_variable_42 * noise_metadata_schedule_407_e3837);
        (noise_metadata_schedule_407_e3838,)
    } else {
        (noise_variable_161,)
    }
};
            noise_variable_161 = noise_metadata_schedule_407_e3840;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_408_e3843: f64 = (noise_variable_253 * noise_variable_8);
            let noise_metadata_schedule_408_e3845: f64 = (noise_metadata_schedule_408_e3843 / params.p19);
            let noise_metadata_schedule_408_e3847: f64 = if noise_metadata_schedule_408_e3845 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_550 = noise_metadata_schedule_408_e3847;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_409_e3856,) = {
    if (noise_variable_550 != 0.0) {
        let noise_metadata_schedule_409_e3851: f64 = (noise_variable_253 * noise_variable_8);
        let noise_metadata_schedule_409_e3853: f64 = (noise_metadata_schedule_409_e3851 / params.p19);
        let noise_metadata_schedule_409_e3854: f64 = (noise_metadata_schedule_409_e3853).exp();
        (noise_metadata_schedule_409_e3854,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_409_e3856;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_410_e3862,) = {
    if (noise_variable_550 == 0.0) {
        let noise_metadata_schedule_410_e3860: f64 = (params.p151).exp();
        (noise_metadata_schedule_410_e3860,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_410_e3862;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_411_e3877,) = {
    if (noise_variable_550 == 0.0) {
        let noise_metadata_schedule_411_e3869: f64 = (noise_variable_253 * noise_variable_8);
        let noise_metadata_schedule_411_e3871: f64 = (noise_metadata_schedule_411_e3869 / params.p19);
        let noise_metadata_schedule_411_e3873: f64 = (noise_metadata_schedule_411_e3871 - params.p151);
        let noise_metadata_schedule_411_e3874: f64 = (1.0 + noise_metadata_schedule_411_e3873);
        let noise_metadata_schedule_411_e3875: f64 = (noise_variable_301 * noise_metadata_schedule_411_e3874);
        (noise_metadata_schedule_411_e3875,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_411_e3877;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_412_e3880: f64 = if params.p24 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_551 = noise_metadata_schedule_412_e3880;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_413_e3883: f64 = (noise_variable_253 - noise_variable_55);
            let noise_metadata_schedule_413_e3885: f64 = (noise_metadata_schedule_413_e3883 * noise_variable_8);
            let noise_metadata_schedule_413_e3887: f64 = if noise_metadata_schedule_413_e3885 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_552 = noise_metadata_schedule_413_e3887;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_414_e3898,) = {
    if ((noise_variable_551 != 0.0) && (noise_variable_552 != 0.0)) {
        let noise_metadata_schedule_414_e3893: f64 = (noise_variable_253 - noise_variable_55);
        let noise_metadata_schedule_414_e3895: f64 = (noise_metadata_schedule_414_e3893 * noise_variable_8);
        let noise_metadata_schedule_414_e3896: f64 = (noise_metadata_schedule_414_e3895).exp();
        (noise_metadata_schedule_414_e3896,)
    } else {
        (noise_variable_304,)
    }
};
            noise_variable_304 = noise_metadata_schedule_414_e3898;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_415_e3906,) = {
    if ((noise_variable_551 != 0.0) && (noise_variable_552 == 0.0)) {
        let noise_metadata_schedule_415_e3904: f64 = (params.p151).exp();
        (noise_metadata_schedule_415_e3904,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_415_e3906;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_416_e3923,) = {
    if ((noise_variable_551 != 0.0) && (noise_variable_552 == 0.0)) {
        let noise_metadata_schedule_416_e3915: f64 = (noise_variable_253 - noise_variable_55);
        let noise_metadata_schedule_416_e3917: f64 = (noise_metadata_schedule_416_e3915 * noise_variable_8);
        let noise_metadata_schedule_416_e3919: f64 = (noise_metadata_schedule_416_e3917 - params.p151);
        let noise_metadata_schedule_416_e3920: f64 = (1.0 + noise_metadata_schedule_416_e3919);
        let noise_metadata_schedule_416_e3921: f64 = (noise_variable_301 * noise_metadata_schedule_416_e3920);
        (noise_metadata_schedule_416_e3921,)
    } else {
        (noise_variable_304,)
    }
};
            noise_variable_304 = noise_metadata_schedule_416_e3923;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_417_e3948,) = {
    if (noise_variable_551 != 0.0) {
        let noise_metadata_schedule_417_e3928: f64 = (noise_variable_302 - 1.0);
        let noise_metadata_schedule_417_e3929: f64 = (noise_variable_44 * noise_metadata_schedule_417_e3928);
        let noise_metadata_schedule_417_e3932: f64 = (noise_variable_45 * 2.0);
        let noise_metadata_schedule_417_e3935: f64 = (noise_variable_302 - 1.0);
        let noise_metadata_schedule_417_e3936: f64 = (noise_metadata_schedule_417_e3932 * noise_metadata_schedule_417_e3935);
        let noise_metadata_schedule_417_e3941: f64 = (4.0 * noise_variable_304);
        let noise_metadata_schedule_417_e3942: f64 = (1.0 + noise_metadata_schedule_417_e3941);
        let noise_metadata_schedule_417_e3943: f64 = (noise_metadata_schedule_417_e3942).sqrt();
        let noise_metadata_schedule_417_e3944: f64 = (1.0 + noise_metadata_schedule_417_e3943);
        let noise_metadata_schedule_417_e3945: f64 = (noise_metadata_schedule_417_e3936 / noise_metadata_schedule_417_e3944);
        let noise_metadata_schedule_417_e3946: f64 = (noise_metadata_schedule_417_e3929 + noise_metadata_schedule_417_e3945);
        (noise_metadata_schedule_417_e3946,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_417_e3948;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_418_e3957,) = {
    if (noise_variable_551 == 0.0) {
        let noise_metadata_schedule_418_e3954: f64 = (noise_variable_302 - 1.0);
        let noise_metadata_schedule_418_e3955: f64 = (noise_variable_44 * noise_metadata_schedule_418_e3954);
        (noise_metadata_schedule_418_e3955,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_418_e3957;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_419_e3960: f64 = (noise_variable_252 * noise_variable_8);
            let noise_metadata_schedule_419_e3962: f64 = (noise_metadata_schedule_419_e3960 / params.p21);
            let noise_metadata_schedule_419_e3964: f64 = if noise_metadata_schedule_419_e3962 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_553 = noise_metadata_schedule_419_e3964;
        }
        if matches!(source_index, 2 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_420_e3973,) = {
    if (noise_variable_553 != 0.0) {
        let noise_metadata_schedule_420_e3968: f64 = (noise_variable_252 * noise_variable_8);
        let noise_metadata_schedule_420_e3970: f64 = (noise_metadata_schedule_420_e3968 / params.p21);
        let noise_metadata_schedule_420_e3971: f64 = (noise_metadata_schedule_420_e3970).exp();
        (noise_metadata_schedule_420_e3971,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_420_e3973;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_421_e3979,) = {
    if (noise_variable_553 == 0.0) {
        let noise_metadata_schedule_421_e3977: f64 = (params.p151).exp();
        (noise_metadata_schedule_421_e3977,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_421_e3979;
        }
        if matches!(source_index, 2 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_422_e3994,) = {
    if (noise_variable_553 == 0.0) {
        let noise_metadata_schedule_422_e3986: f64 = (noise_variable_252 * noise_variable_8);
        let noise_metadata_schedule_422_e3988: f64 = (noise_metadata_schedule_422_e3986 / params.p21);
        let noise_metadata_schedule_422_e3990: f64 = (noise_metadata_schedule_422_e3988 - params.p151);
        let noise_metadata_schedule_422_e3991: f64 = (1.0 + noise_metadata_schedule_422_e3990);
        let noise_metadata_schedule_422_e3992: f64 = (noise_variable_301 * noise_metadata_schedule_422_e3991);
        (noise_metadata_schedule_422_e3992,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_422_e3994;
        }
        if matches!(source_index, 2 | 7) {
            let noise_metadata_schedule_423_e3998: f64 = (noise_variable_302 - 1.0);
            let noise_metadata_schedule_423_e3999: f64 = (noise_variable_38 * noise_metadata_schedule_423_e3998);
            noise_variable_163 = noise_metadata_schedule_423_e3999;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_424_e4002: f64 = (noise_variable_253 * noise_variable_8);
            let noise_metadata_schedule_424_e4004: f64 = (noise_metadata_schedule_424_e4002 / params.p23);
            let noise_metadata_schedule_424_e4006: f64 = if noise_metadata_schedule_424_e4004 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_554 = noise_metadata_schedule_424_e4006;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_425_e4015,) = {
    if (noise_variable_554 != 0.0) {
        let noise_metadata_schedule_425_e4010: f64 = (noise_variable_253 * noise_variable_8);
        let noise_metadata_schedule_425_e4012: f64 = (noise_metadata_schedule_425_e4010 / params.p23);
        let noise_metadata_schedule_425_e4013: f64 = (noise_metadata_schedule_425_e4012).exp();
        (noise_metadata_schedule_425_e4013,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_425_e4015;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_426_e4021,) = {
    if (noise_variable_554 == 0.0) {
        let noise_metadata_schedule_426_e4019: f64 = (params.p151).exp();
        (noise_metadata_schedule_426_e4019,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_426_e4021;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_427_e4036,) = {
    if (noise_variable_554 == 0.0) {
        let noise_metadata_schedule_427_e4028: f64 = (noise_variable_253 * noise_variable_8);
        let noise_metadata_schedule_427_e4030: f64 = (noise_metadata_schedule_427_e4028 / params.p23);
        let noise_metadata_schedule_427_e4032: f64 = (noise_metadata_schedule_427_e4030 - params.p151);
        let noise_metadata_schedule_427_e4033: f64 = (1.0 + noise_metadata_schedule_427_e4032);
        let noise_metadata_schedule_427_e4034: f64 = (noise_variable_301 * noise_metadata_schedule_427_e4033);
        (noise_metadata_schedule_427_e4034,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_427_e4036;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_428_e4040: f64 = (noise_variable_302 - 1.0);
            let noise_metadata_schedule_428_e4041: f64 = (noise_variable_46 * noise_metadata_schedule_428_e4040);
            noise_variable_165 = noise_metadata_schedule_428_e4041;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_429_e4044: f64 = (noise_variable_255 * noise_variable_8);
            let noise_metadata_schedule_429_e4046: f64 = (noise_metadata_schedule_429_e4044 / params.p32);
            let noise_metadata_schedule_429_e4048: f64 = if noise_metadata_schedule_429_e4046 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_555 = noise_metadata_schedule_429_e4048;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_430_e4057,) = {
    if (noise_variable_555 != 0.0) {
        let noise_metadata_schedule_430_e4052: f64 = (noise_variable_255 * noise_variable_8);
        let noise_metadata_schedule_430_e4054: f64 = (noise_metadata_schedule_430_e4052 / params.p32);
        let noise_metadata_schedule_430_e4055: f64 = (noise_metadata_schedule_430_e4054).exp();
        (noise_metadata_schedule_430_e4055,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_430_e4057;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_431_e4063,) = {
    if (noise_variable_555 == 0.0) {
        let noise_metadata_schedule_431_e4061: f64 = (params.p151).exp();
        (noise_metadata_schedule_431_e4061,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_431_e4063;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_432_e4078,) = {
    if (noise_variable_555 == 0.0) {
        let noise_metadata_schedule_432_e4070: f64 = (noise_variable_255 * noise_variable_8);
        let noise_metadata_schedule_432_e4072: f64 = (noise_metadata_schedule_432_e4070 / params.p32);
        let noise_metadata_schedule_432_e4074: f64 = (noise_metadata_schedule_432_e4072 - params.p151);
        let noise_metadata_schedule_432_e4075: f64 = (1.0 + noise_metadata_schedule_432_e4074);
        let noise_metadata_schedule_432_e4076: f64 = (noise_variable_301 * noise_metadata_schedule_432_e4075);
        (noise_metadata_schedule_432_e4076,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_432_e4078;
        }
        if matches!(source_index, 9 | 10) {
            let noise_metadata_schedule_433_e4082: f64 = (noise_variable_302 - 1.0);
            let noise_metadata_schedule_433_e4083: f64 = (noise_variable_39 * noise_metadata_schedule_433_e4082);
            noise_variable_164 = noise_metadata_schedule_433_e4083;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 15 | 16) {
            let noise_metadata_schedule_434_e4086: f64 = (noise_variable_253 * noise_variable_8);
            let noise_metadata_schedule_434_e4088: f64 = (noise_metadata_schedule_434_e4086 / params.p150);
            let noise_metadata_schedule_434_e4090: f64 = if noise_metadata_schedule_434_e4088 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_556 = noise_metadata_schedule_434_e4090;
        }
        if matches!(source_index, 7 | 8) {
            let (noise_metadata_schedule_435_e4099,) = {
    if (noise_variable_556 != 0.0) {
        let noise_metadata_schedule_435_e4094: f64 = (noise_variable_253 * noise_variable_8);
        let noise_metadata_schedule_435_e4096: f64 = (noise_metadata_schedule_435_e4094 / params.p150);
        let noise_metadata_schedule_435_e4097: f64 = (noise_metadata_schedule_435_e4096).exp();
        (noise_metadata_schedule_435_e4097,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_435_e4099;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 15 | 16) {
            let (noise_metadata_schedule_436_e4105,) = {
    if (noise_variable_556 == 0.0) {
        let noise_metadata_schedule_436_e4103: f64 = (params.p151).exp();
        (noise_metadata_schedule_436_e4103,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_436_e4105;
        }
        if matches!(source_index, 7 | 8) {
            let (noise_metadata_schedule_437_e4120,) = {
    if (noise_variable_556 == 0.0) {
        let noise_metadata_schedule_437_e4112: f64 = (noise_variable_253 * noise_variable_8);
        let noise_metadata_schedule_437_e4114: f64 = (noise_metadata_schedule_437_e4112 / params.p150);
        let noise_metadata_schedule_437_e4116: f64 = (noise_metadata_schedule_437_e4114 - params.p151);
        let noise_metadata_schedule_437_e4117: f64 = (1.0 + noise_metadata_schedule_437_e4116);
        let noise_metadata_schedule_437_e4118: f64 = (noise_variable_301 * noise_metadata_schedule_437_e4117);
        (noise_metadata_schedule_437_e4118,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_437_e4120;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_438_e4124: f64 = (noise_variable_302 - 1.0);
            let noise_metadata_schedule_438_e4125: f64 = (noise_variable_47 * noise_metadata_schedule_438_e4124);
            noise_variable_166 = noise_metadata_schedule_438_e4125;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_439_e4136: f64 = if (((params.p34 > 0.0) && (params.p35 > 0.0)) && (noise_variable_252 < 0.0)) { 1.0 } else { 0.0 };
            noise_variable_557 = noise_metadata_schedule_439_e4136;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_440_e4142: f64 = (2.0 * noise_variable_59);
            let noise_metadata_schedule_440_e4143: f64 = (noise_variable_62 / noise_metadata_schedule_440_e4142);
            let noise_metadata_schedule_440_e4144: f64 = (1.0 - noise_metadata_schedule_440_e4143);
            let noise_metadata_schedule_440_e4145: f64 = (noise_variable_61 * noise_metadata_schedule_440_e4144);
            let noise_metadata_schedule_440_e4147: f64 = if noise_metadata_schedule_440_e4145 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_558 = noise_metadata_schedule_440_e4147;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_441_e4162,) = {
    if ((noise_variable_557 != 0.0) && (noise_variable_558 != 0.0)) {
        let noise_metadata_schedule_441_e4156: f64 = (2.0 * noise_variable_59);
        let noise_metadata_schedule_441_e4157: f64 = (noise_variable_62 / noise_metadata_schedule_441_e4156);
        let noise_metadata_schedule_441_e4158: f64 = (1.0 - noise_metadata_schedule_441_e4157);
        let noise_metadata_schedule_441_e4159: f64 = (noise_variable_61 * noise_metadata_schedule_441_e4158);
        let noise_metadata_schedule_441_e4160: f64 = (noise_metadata_schedule_441_e4159).exp();
        (noise_metadata_schedule_441_e4160,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_441_e4162;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_442_e4170,) = {
    if ((noise_variable_557 != 0.0) && (noise_variable_558 == 0.0)) {
        let noise_metadata_schedule_442_e4168: f64 = (params.p151).exp();
        (noise_metadata_schedule_442_e4168,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_442_e4170;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_443_e4191,) = {
    if ((noise_variable_557 != 0.0) && (noise_variable_558 == 0.0)) {
        let noise_metadata_schedule_443_e4182: f64 = (2.0 * noise_variable_59);
        let noise_metadata_schedule_443_e4183: f64 = (noise_variable_62 / noise_metadata_schedule_443_e4182);
        let noise_metadata_schedule_443_e4184: f64 = (1.0 - noise_metadata_schedule_443_e4183);
        let noise_metadata_schedule_443_e4185: f64 = (noise_variable_61 * noise_metadata_schedule_443_e4184);
        let noise_metadata_schedule_443_e4187: f64 = (noise_metadata_schedule_443_e4185 - params.p151);
        let noise_metadata_schedule_443_e4188: f64 = (1.0 + noise_metadata_schedule_443_e4187);
        let noise_metadata_schedule_443_e4189: f64 = (noise_variable_301 * noise_metadata_schedule_443_e4188);
        (noise_metadata_schedule_443_e4189,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_443_e4191;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_444_e4197,) = {
    if (noise_variable_557 != 0.0) {
        let noise_metadata_schedule_444_e4195: f64 = (noise_variable_252 * noise_variable_65);
        (noise_metadata_schedule_444_e4195,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_444_e4197;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_445_e4241,) = {
    if (noise_variable_557 != 0.0) {
        let noise_metadata_schedule_445_e4201: f64 = (noise_variable_281 * noise_variable_281);
        let noise_metadata_schedule_445_e4203: f64 = (noise_metadata_schedule_445_e4201 + 1e-30);
        let noise_metadata_schedule_445_e4204: f64 = (noise_metadata_schedule_445_e4203).sqrt();
        let noise_metadata_schedule_445_e4206: f64 = (-2.0);
        let noise_metadata_schedule_445_e4208: f64 = (noise_metadata_schedule_445_e4206 - params.p67);
        let noise_metadata_schedule_445_e4209: f64 = (noise_metadata_schedule_445_e4204).powf(noise_metadata_schedule_445_e4208);
        let noise_metadata_schedule_445_e4214: f64 = (params.p67 * params.p67);
        let noise_metadata_schedule_445_e4215: f64 = (1.0 - noise_metadata_schedule_445_e4214);
        let noise_metadata_schedule_445_e4218: f64 = (3.0 * noise_variable_281);
        let noise_metadata_schedule_445_e4221: f64 = (params.p67 - 1.0);
        let noise_metadata_schedule_445_e4222: f64 = (noise_metadata_schedule_445_e4218 * noise_metadata_schedule_445_e4221);
        let noise_metadata_schedule_445_e4223: f64 = (noise_metadata_schedule_445_e4215 - noise_metadata_schedule_445_e4222);
        let noise_metadata_schedule_445_e4224: f64 = (params.p67 * noise_metadata_schedule_445_e4223);
        let noise_metadata_schedule_445_e4227: f64 = (6.0 * noise_variable_281);
        let noise_metadata_schedule_445_e4229: f64 = (noise_metadata_schedule_445_e4227 * noise_variable_281);
        let noise_metadata_schedule_445_e4232: f64 = (params.p67 - 1.0);
        let noise_metadata_schedule_445_e4234: f64 = (noise_metadata_schedule_445_e4232 + noise_variable_281);
        let noise_metadata_schedule_445_e4235: f64 = (noise_metadata_schedule_445_e4229 * noise_metadata_schedule_445_e4234);
        let noise_metadata_schedule_445_e4236: f64 = (noise_metadata_schedule_445_e4224 - noise_metadata_schedule_445_e4235);
        let noise_metadata_schedule_445_e4237: f64 = (noise_metadata_schedule_445_e4209 * noise_metadata_schedule_445_e4236);
        let noise_metadata_schedule_445_e4239: f64 = (noise_metadata_schedule_445_e4237 * 0.16666666666666666);
        (noise_metadata_schedule_445_e4239,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_445_e4241;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_446_e4253,) = {
    if (noise_variable_557 != 0.0) {
        let noise_metadata_schedule_446_e4245: f64 = (noise_variable_252 * noise_variable_62);
        let noise_metadata_schedule_446_e4247: f64 = (noise_metadata_schedule_446_e4245 * noise_variable_61);
        let noise_metadata_schedule_446_e4250: f64 = (noise_variable_70 * noise_variable_60);
        let noise_metadata_schedule_446_e4251: f64 = (noise_metadata_schedule_446_e4247 / noise_metadata_schedule_446_e4250);
        (noise_metadata_schedule_446_e4251,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_446_e4253;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_447_e4256: f64 = (-0.001);
            let noise_metadata_schedule_447_e4257: f64 = if noise_variable_281 < noise_metadata_schedule_447_e4256 { 1.0 } else { 0.0 };
            noise_variable_559 = noise_metadata_schedule_447_e4257;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_448_e4260: f64 = if noise_variable_281 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_560 = noise_metadata_schedule_448_e4260;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_449_e4269,) = {
    if (((noise_variable_557 != 0.0) && (noise_variable_559 != 0.0)) && (noise_variable_560 != 0.0)) {
        let noise_metadata_schedule_449_e4267: f64 = (noise_variable_281).exp();
        (noise_metadata_schedule_449_e4267,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_449_e4269;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_450_e4279,) = {
    if (((noise_variable_557 != 0.0) && (noise_variable_559 != 0.0)) && (noise_variable_560 == 0.0)) {
        let noise_metadata_schedule_450_e4277: f64 = (params.p151).exp();
        (noise_metadata_schedule_450_e4277,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_450_e4279;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_451_e4294,) = {
    if (((noise_variable_557 != 0.0) && (noise_variable_559 != 0.0)) && (noise_variable_560 == 0.0)) {
        let noise_metadata_schedule_451_e4290: f64 = (noise_variable_281 - params.p151);
        let noise_metadata_schedule_451_e4291: f64 = (1.0 + noise_metadata_schedule_451_e4290);
        let noise_metadata_schedule_451_e4292: f64 = (noise_variable_301 * noise_metadata_schedule_451_e4291);
        (noise_metadata_schedule_451_e4292,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_451_e4294;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_452_e4309,) = {
    if ((noise_variable_557 != 0.0) && (noise_variable_559 != 0.0)) {
        let noise_metadata_schedule_452_e4299: f64 = (-noise_variable_252);
        let noise_metadata_schedule_452_e4303: f64 = (1.0 - noise_variable_91);
        let noise_metadata_schedule_452_e4305: f64 = (noise_metadata_schedule_452_e4303 / noise_variable_281);
        let noise_metadata_schedule_452_e4306: f64 = (1.0 + noise_metadata_schedule_452_e4305);
        let noise_metadata_schedule_452_e4307: f64 = (noise_metadata_schedule_452_e4299 * noise_metadata_schedule_452_e4306);
        (noise_metadata_schedule_452_e4307,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_452_e4309;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_453_e4332,) = {
    if ((noise_variable_557 != 0.0) && (noise_variable_559 == 0.0)) {
        let noise_metadata_schedule_453_e4316: f64 = (noise_variable_252 * 0.5);
        let noise_metadata_schedule_453_e4318: f64 = (noise_metadata_schedule_453_e4316 * noise_variable_281);
        let noise_metadata_schedule_453_e4322: f64 = (noise_variable_281 * 0.3333333333333333);
        let noise_metadata_schedule_453_e4326: f64 = (0.25 * noise_variable_281);
        let noise_metadata_schedule_453_e4327: f64 = (1.0 + noise_metadata_schedule_453_e4326);
        let noise_metadata_schedule_453_e4328: f64 = (noise_metadata_schedule_453_e4322 * noise_metadata_schedule_453_e4327);
        let noise_metadata_schedule_453_e4329: f64 = (1.0 + noise_metadata_schedule_453_e4328);
        let noise_metadata_schedule_453_e4330: f64 = (noise_metadata_schedule_453_e4318 * noise_metadata_schedule_453_e4329);
        (noise_metadata_schedule_453_e4330,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_453_e4332;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_454_e4348,) = {
    if (noise_variable_557 != 0.0) {
        let noise_metadata_schedule_454_e4336: f64 = (2.0 * noise_variable_58);
        let noise_metadata_schedule_454_e4338: f64 = (noise_metadata_schedule_454_e4336 * noise_variable_69);
        let noise_metadata_schedule_454_e4340: f64 = (noise_metadata_schedule_454_e4338 * noise_variable_59);
        let noise_metadata_schedule_454_e4342: f64 = (noise_metadata_schedule_454_e4340 * noise_variable_68);
        let noise_metadata_schedule_454_e4344: f64 = (noise_metadata_schedule_454_e4342 * noise_variable_65);
        let noise_metadata_schedule_454_e4346: f64 = (noise_metadata_schedule_454_e4344 * noise_variable_63);
        (noise_metadata_schedule_454_e4346,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_454_e4348;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_456_e4358,) = {
    if (noise_variable_557 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_456_e4358;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_457_e4369: f64 = if (((params.p36 > 0.0) && (params.p37 > 0.0)) && (noise_variable_250 < 0.0)) { 1.0 } else { 0.0 };
            noise_variable_561 = noise_metadata_schedule_457_e4369;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_458_e4381,) = {
    if (noise_variable_561 != 0.0) {
        let noise_metadata_schedule_458_e4374: f64 = (noise_variable_250 * noise_variable_67);
        let noise_metadata_schedule_458_e4375: f64 = (1.0 - noise_metadata_schedule_458_e4374);
        let noise_metadata_schedule_458_e4378: f64 = (1.0 - noise_variable_76);
        let noise_metadata_schedule_458_e4379: f64 = (noise_metadata_schedule_458_e4375).powf(noise_metadata_schedule_458_e4378);
        (noise_metadata_schedule_458_e4379,)
    } else {
        (noise_variable_77,)
    }
};
            noise_variable_77 = noise_metadata_schedule_458_e4381;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_459_e4387: f64 = (2.0 * noise_variable_77);
            let noise_metadata_schedule_459_e4388: f64 = (noise_variable_79 / noise_metadata_schedule_459_e4387);
            let noise_metadata_schedule_459_e4389: f64 = (1.0 - noise_metadata_schedule_459_e4388);
            let noise_metadata_schedule_459_e4390: f64 = (noise_variable_83 * noise_metadata_schedule_459_e4389);
            let noise_metadata_schedule_459_e4392: f64 = if noise_metadata_schedule_459_e4390 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_562 = noise_metadata_schedule_459_e4392;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_460_e4407,) = {
    if ((noise_variable_561 != 0.0) && (noise_variable_562 != 0.0)) {
        let noise_metadata_schedule_460_e4401: f64 = (2.0 * noise_variable_77);
        let noise_metadata_schedule_460_e4402: f64 = (noise_variable_79 / noise_metadata_schedule_460_e4401);
        let noise_metadata_schedule_460_e4403: f64 = (1.0 - noise_metadata_schedule_460_e4402);
        let noise_metadata_schedule_460_e4404: f64 = (noise_variable_83 * noise_metadata_schedule_460_e4403);
        let noise_metadata_schedule_460_e4405: f64 = (noise_metadata_schedule_460_e4404).exp();
        (noise_metadata_schedule_460_e4405,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_460_e4407;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_461_e4415,) = {
    if ((noise_variable_561 != 0.0) && (noise_variable_562 == 0.0)) {
        let noise_metadata_schedule_461_e4413: f64 = (params.p151).exp();
        (noise_metadata_schedule_461_e4413,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_461_e4415;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_462_e4436,) = {
    if ((noise_variable_561 != 0.0) && (noise_variable_562 == 0.0)) {
        let noise_metadata_schedule_462_e4427: f64 = (2.0 * noise_variable_77);
        let noise_metadata_schedule_462_e4428: f64 = (noise_variable_79 / noise_metadata_schedule_462_e4427);
        let noise_metadata_schedule_462_e4429: f64 = (1.0 - noise_metadata_schedule_462_e4428);
        let noise_metadata_schedule_462_e4430: f64 = (noise_variable_83 * noise_metadata_schedule_462_e4429);
        let noise_metadata_schedule_462_e4432: f64 = (noise_metadata_schedule_462_e4430 - params.p151);
        let noise_metadata_schedule_462_e4433: f64 = (1.0 + noise_metadata_schedule_462_e4432);
        let noise_metadata_schedule_462_e4434: f64 = (noise_variable_301 * noise_metadata_schedule_462_e4433);
        (noise_metadata_schedule_462_e4434,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_462_e4436;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_463_e4442,) = {
    if (noise_variable_561 != 0.0) {
        let noise_metadata_schedule_463_e4440: f64 = (noise_variable_250 * noise_variable_67);
        (noise_metadata_schedule_463_e4440,)
    } else {
        (noise_variable_283,)
    }
};
            noise_variable_283 = noise_metadata_schedule_463_e4442;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_464_e4486,) = {
    if (noise_variable_561 != 0.0) {
        let noise_metadata_schedule_464_e4446: f64 = (noise_variable_283 * noise_variable_283);
        let noise_metadata_schedule_464_e4448: f64 = (noise_metadata_schedule_464_e4446 + 1e-30);
        let noise_metadata_schedule_464_e4449: f64 = (noise_metadata_schedule_464_e4448).sqrt();
        let noise_metadata_schedule_464_e4451: f64 = (-2.0);
        let noise_metadata_schedule_464_e4453: f64 = (noise_metadata_schedule_464_e4451 - noise_variable_76);
        let noise_metadata_schedule_464_e4454: f64 = (noise_metadata_schedule_464_e4449).powf(noise_metadata_schedule_464_e4453);
        let noise_metadata_schedule_464_e4459: f64 = (noise_variable_76 * noise_variable_76);
        let noise_metadata_schedule_464_e4460: f64 = (1.0 - noise_metadata_schedule_464_e4459);
        let noise_metadata_schedule_464_e4463: f64 = (3.0 * noise_variable_283);
        let noise_metadata_schedule_464_e4466: f64 = (noise_variable_76 - 1.0);
        let noise_metadata_schedule_464_e4467: f64 = (noise_metadata_schedule_464_e4463 * noise_metadata_schedule_464_e4466);
        let noise_metadata_schedule_464_e4468: f64 = (noise_metadata_schedule_464_e4460 - noise_metadata_schedule_464_e4467);
        let noise_metadata_schedule_464_e4469: f64 = (noise_variable_76 * noise_metadata_schedule_464_e4468);
        let noise_metadata_schedule_464_e4472: f64 = (6.0 * noise_variable_283);
        let noise_metadata_schedule_464_e4474: f64 = (noise_metadata_schedule_464_e4472 * noise_variable_283);
        let noise_metadata_schedule_464_e4477: f64 = (noise_variable_76 - 1.0);
        let noise_metadata_schedule_464_e4479: f64 = (noise_metadata_schedule_464_e4477 + noise_variable_283);
        let noise_metadata_schedule_464_e4480: f64 = (noise_metadata_schedule_464_e4474 * noise_metadata_schedule_464_e4479);
        let noise_metadata_schedule_464_e4481: f64 = (noise_metadata_schedule_464_e4469 - noise_metadata_schedule_464_e4480);
        let noise_metadata_schedule_464_e4482: f64 = (noise_metadata_schedule_464_e4454 * noise_metadata_schedule_464_e4481);
        let noise_metadata_schedule_464_e4484: f64 = (noise_metadata_schedule_464_e4482 * 0.16666666666666666);
        (noise_metadata_schedule_464_e4484,)
    } else {
        (noise_variable_80,)
    }
};
            noise_variable_80 = noise_metadata_schedule_464_e4486;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_465_e4498,) = {
    if (noise_variable_561 != 0.0) {
        let noise_metadata_schedule_465_e4490: f64 = (noise_variable_250 * noise_variable_79);
        let noise_metadata_schedule_465_e4492: f64 = (noise_metadata_schedule_465_e4490 * noise_variable_83);
        let noise_metadata_schedule_465_e4495: f64 = (noise_variable_85 * noise_variable_80);
        let noise_metadata_schedule_465_e4496: f64 = (noise_metadata_schedule_465_e4492 / noise_metadata_schedule_465_e4495);
        (noise_metadata_schedule_465_e4496,)
    } else {
        (noise_variable_283,)
    }
};
            noise_variable_283 = noise_metadata_schedule_465_e4498;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_466_e4501: f64 = (-0.001);
            let noise_metadata_schedule_466_e4502: f64 = if noise_variable_283 < noise_metadata_schedule_466_e4501 { 1.0 } else { 0.0 };
            noise_variable_563 = noise_metadata_schedule_466_e4502;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_467_e4505: f64 = if noise_variable_283 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_564 = noise_metadata_schedule_467_e4505;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_468_e4514,) = {
    if (((noise_variable_561 != 0.0) && (noise_variable_563 != 0.0)) && (noise_variable_564 != 0.0)) {
        let noise_metadata_schedule_468_e4512: f64 = (noise_variable_283).exp();
        (noise_metadata_schedule_468_e4512,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_468_e4514;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_469_e4524,) = {
    if (((noise_variable_561 != 0.0) && (noise_variable_563 != 0.0)) && (noise_variable_564 == 0.0)) {
        let noise_metadata_schedule_469_e4522: f64 = (params.p151).exp();
        (noise_metadata_schedule_469_e4522,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_469_e4524;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_470_e4539,) = {
    if (((noise_variable_561 != 0.0) && (noise_variable_563 != 0.0)) && (noise_variable_564 == 0.0)) {
        let noise_metadata_schedule_470_e4535: f64 = (noise_variable_283 - params.p151);
        let noise_metadata_schedule_470_e4536: f64 = (1.0 + noise_metadata_schedule_470_e4535);
        let noise_metadata_schedule_470_e4537: f64 = (noise_variable_301 * noise_metadata_schedule_470_e4536);
        (noise_metadata_schedule_470_e4537,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_470_e4539;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_471_e4554,) = {
    if ((noise_variable_561 != 0.0) && (noise_variable_563 != 0.0)) {
        let noise_metadata_schedule_471_e4544: f64 = (-noise_variable_250);
        let noise_metadata_schedule_471_e4548: f64 = (1.0 - noise_variable_92);
        let noise_metadata_schedule_471_e4550: f64 = (noise_metadata_schedule_471_e4548 / noise_variable_283);
        let noise_metadata_schedule_471_e4551: f64 = (1.0 + noise_metadata_schedule_471_e4550);
        let noise_metadata_schedule_471_e4552: f64 = (noise_metadata_schedule_471_e4544 * noise_metadata_schedule_471_e4551);
        (noise_metadata_schedule_471_e4552,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_471_e4554;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_472_e4577,) = {
    if ((noise_variable_561 != 0.0) && (noise_variable_563 == 0.0)) {
        let noise_metadata_schedule_472_e4561: f64 = (noise_variable_250 * 0.5);
        let noise_metadata_schedule_472_e4563: f64 = (noise_metadata_schedule_472_e4561 * noise_variable_283);
        let noise_metadata_schedule_472_e4567: f64 = (noise_variable_283 * 0.3333333333333333);
        let noise_metadata_schedule_472_e4571: f64 = (0.25 * noise_variable_283);
        let noise_metadata_schedule_472_e4572: f64 = (1.0 + noise_metadata_schedule_472_e4571);
        let noise_metadata_schedule_472_e4573: f64 = (noise_metadata_schedule_472_e4567 * noise_metadata_schedule_472_e4572);
        let noise_metadata_schedule_472_e4574: f64 = (1.0 + noise_metadata_schedule_472_e4573);
        let noise_metadata_schedule_472_e4575: f64 = (noise_metadata_schedule_472_e4563 * noise_metadata_schedule_472_e4574);
        (noise_metadata_schedule_472_e4575,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_472_e4577;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_473_e4593,) = {
    if (noise_variable_561 != 0.0) {
        let noise_metadata_schedule_473_e4581: f64 = (2.0 * noise_variable_84);
        let noise_metadata_schedule_473_e4583: f64 = (noise_metadata_schedule_473_e4581 * noise_variable_81);
        let noise_metadata_schedule_473_e4585: f64 = (noise_metadata_schedule_473_e4583 * noise_variable_77);
        let noise_metadata_schedule_473_e4587: f64 = (noise_metadata_schedule_473_e4585 * noise_variable_78);
        let noise_metadata_schedule_473_e4589: f64 = (noise_metadata_schedule_473_e4587 * noise_variable_67);
        let noise_metadata_schedule_473_e4591: f64 = (noise_metadata_schedule_473_e4589 * noise_variable_89);
        (noise_metadata_schedule_473_e4591,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_473_e4593;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_475_e4603,) = {
    if (noise_variable_561 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_475_e4603;
        }
        if matches!(source_index, 11 | 12) {
            let noise_metadata_schedule_480_e4630: f64 = (2.0 * noise_variable_43);
            let noise_metadata_schedule_480_e4633: f64 = (noise_variable_274 - 1.0);
            let noise_metadata_schedule_480_e4634: f64 = (noise_metadata_schedule_480_e4630 * noise_metadata_schedule_480_e4633);
            let noise_metadata_schedule_480_e4639: f64 = (4.0 * noise_variable_43);
            let noise_metadata_schedule_480_e4641: f64 = (noise_metadata_schedule_480_e4639 / noise_variable_37);
            let noise_metadata_schedule_480_e4643: f64 = (noise_metadata_schedule_480_e4641 * noise_variable_274);
            let noise_metadata_schedule_480_e4644: f64 = (1.0 + noise_metadata_schedule_480_e4643);
            let noise_metadata_schedule_480_e4645: f64 = (noise_metadata_schedule_480_e4644).sqrt();
            let noise_metadata_schedule_480_e4646: f64 = (1.0 + noise_metadata_schedule_480_e4645);
            let noise_metadata_schedule_480_e4647: f64 = (noise_metadata_schedule_480_e4634 / noise_metadata_schedule_480_e4646);
            noise_variable_167 = noise_metadata_schedule_480_e4647;
        }
        if matches!(source_index, 17 | 18) {
            let noise_metadata_schedule_481_e4650: f64 = if params.p8 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_565 = noise_metadata_schedule_481_e4650;
        }
        if matches!(source_index, 17) {
            let (noise_metadata_schedule_482_e4679,) = {
    if (noise_variable_565 != 0.0) {
        let noise_metadata_schedule_482_e4654: f64 = (params.p143 * 2.0);
        let noise_metadata_schedule_482_e4656: f64 = (noise_metadata_schedule_482_e4654 * noise_variable_107);
        let noise_metadata_schedule_482_e4659: f64 = (noise_variable_271 - noise_variable_262);
        let noise_metadata_schedule_482_e4660: f64 = (noise_metadata_schedule_482_e4656 * noise_metadata_schedule_482_e4659);
        let noise_metadata_schedule_482_e4666: f64 = (noise_variable_107 / noise_variable_109);
        let noise_metadata_schedule_482_e4667: f64 = (4.0 * noise_metadata_schedule_482_e4666);
        let noise_metadata_schedule_482_e4671: f64 = (params.p144 * noise_variable_262);
        let noise_metadata_schedule_482_e4672: f64 = (noise_variable_271 + noise_metadata_schedule_482_e4671);
        let noise_metadata_schedule_482_e4673: f64 = (noise_metadata_schedule_482_e4667 * noise_metadata_schedule_482_e4672);
        let noise_metadata_schedule_482_e4674: f64 = (1.0 + noise_metadata_schedule_482_e4673);
        let noise_metadata_schedule_482_e4675: f64 = (noise_metadata_schedule_482_e4674).sqrt();
        let noise_metadata_schedule_482_e4676: f64 = (1.0 + noise_metadata_schedule_482_e4675);
        let noise_metadata_schedule_482_e4677: f64 = (noise_metadata_schedule_482_e4660 / noise_metadata_schedule_482_e4676);
        (noise_metadata_schedule_482_e4677,)
    } else {
        (noise_variable_185,)
    }
};
            noise_variable_185 = noise_metadata_schedule_482_e4679;
        }
        if matches!(source_index, 18) {
            let (noise_metadata_schedule_483_e4710,) = {
    if (noise_variable_565 != 0.0) {
        let noise_metadata_schedule_483_e4683: f64 = (1.0 - params.p143);
        let noise_metadata_schedule_483_e4685: f64 = (noise_metadata_schedule_483_e4683 * 2.0);
        let noise_metadata_schedule_483_e4687: f64 = (noise_metadata_schedule_483_e4685 * noise_variable_107);
        let noise_metadata_schedule_483_e4690: f64 = (noise_variable_274 - noise_variable_264);
        let noise_metadata_schedule_483_e4691: f64 = (noise_metadata_schedule_483_e4687 * noise_metadata_schedule_483_e4690);
        let noise_metadata_schedule_483_e4697: f64 = (noise_variable_107 / noise_variable_109);
        let noise_metadata_schedule_483_e4698: f64 = (4.0 * noise_metadata_schedule_483_e4697);
        let noise_metadata_schedule_483_e4702: f64 = (params.p144 * noise_variable_264);
        let noise_metadata_schedule_483_e4703: f64 = (noise_variable_274 + noise_metadata_schedule_483_e4702);
        let noise_metadata_schedule_483_e4704: f64 = (noise_metadata_schedule_483_e4698 * noise_metadata_schedule_483_e4703);
        let noise_metadata_schedule_483_e4705: f64 = (1.0 + noise_metadata_schedule_483_e4704);
        let noise_metadata_schedule_483_e4706: f64 = (noise_metadata_schedule_483_e4705).sqrt();
        let noise_metadata_schedule_483_e4707: f64 = (1.0 + noise_metadata_schedule_483_e4706);
        let noise_metadata_schedule_483_e4708: f64 = (noise_metadata_schedule_483_e4691 / noise_metadata_schedule_483_e4707);
        (noise_metadata_schedule_483_e4708,)
    } else {
        (noise_variable_182,)
    }
};
            noise_variable_182 = noise_metadata_schedule_483_e4710;
        }
        if matches!(source_index, 17) {
            let (noise_metadata_schedule_484_e4736,) = {
    if (noise_variable_565 == 0.0) {
        let noise_metadata_schedule_484_e4715: f64 = (params.p143 * 2.0);
        let noise_metadata_schedule_484_e4717: f64 = (noise_metadata_schedule_484_e4715 * noise_variable_107);
        let noise_metadata_schedule_484_e4720: f64 = (noise_variable_271 - 1.0);
        let noise_metadata_schedule_484_e4721: f64 = (noise_metadata_schedule_484_e4717 * noise_metadata_schedule_484_e4720);
        let noise_metadata_schedule_484_e4727: f64 = (noise_variable_107 / noise_variable_109);
        let noise_metadata_schedule_484_e4728: f64 = (4.0 * noise_metadata_schedule_484_e4727);
        let noise_metadata_schedule_484_e4730: f64 = (noise_metadata_schedule_484_e4728 * noise_variable_271);
        let noise_metadata_schedule_484_e4731: f64 = (1.0 + noise_metadata_schedule_484_e4730);
        let noise_metadata_schedule_484_e4732: f64 = (noise_metadata_schedule_484_e4731).sqrt();
        let noise_metadata_schedule_484_e4733: f64 = (1.0 + noise_metadata_schedule_484_e4732);
        let noise_metadata_schedule_484_e4734: f64 = (noise_metadata_schedule_484_e4721 / noise_metadata_schedule_484_e4733);
        (noise_metadata_schedule_484_e4734,)
    } else {
        (noise_variable_185,)
    }
};
            noise_variable_185 = noise_metadata_schedule_484_e4736;
        }
        if matches!(source_index, 18) {
            let (noise_metadata_schedule_485_e4764,) = {
    if (noise_variable_565 == 0.0) {
        let noise_metadata_schedule_485_e4741: f64 = (1.0 - params.p143);
        let noise_metadata_schedule_485_e4743: f64 = (noise_metadata_schedule_485_e4741 * 2.0);
        let noise_metadata_schedule_485_e4745: f64 = (noise_metadata_schedule_485_e4743 * noise_variable_107);
        let noise_metadata_schedule_485_e4748: f64 = (noise_variable_274 - 1.0);
        let noise_metadata_schedule_485_e4749: f64 = (noise_metadata_schedule_485_e4745 * noise_metadata_schedule_485_e4748);
        let noise_metadata_schedule_485_e4755: f64 = (noise_variable_107 / noise_variable_109);
        let noise_metadata_schedule_485_e4756: f64 = (4.0 * noise_metadata_schedule_485_e4755);
        let noise_metadata_schedule_485_e4758: f64 = (noise_metadata_schedule_485_e4756 * noise_variable_274);
        let noise_metadata_schedule_485_e4759: f64 = (1.0 + noise_metadata_schedule_485_e4758);
        let noise_metadata_schedule_485_e4760: f64 = (noise_metadata_schedule_485_e4759).sqrt();
        let noise_metadata_schedule_485_e4761: f64 = (1.0 + noise_metadata_schedule_485_e4760);
        let noise_metadata_schedule_485_e4762: f64 = (noise_metadata_schedule_485_e4749 / noise_metadata_schedule_485_e4761);
        (noise_metadata_schedule_485_e4762,)
    } else {
        (noise_variable_182,)
    }
};
            noise_variable_182 = noise_metadata_schedule_485_e4764;
        }
        if matches!(source_index, 19) {
            noise_variable_183 = 0.0;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let noise_metadata_schedule_488_e4798: f64 = if ((params.p5 > 0.0) && (params.p33 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_566 = noise_metadata_schedule_488_e4798;
        }
        if matches!(source_index, 11 | 12) {
            let (noise_metadata_schedule_489_e4804,) = {
    if (noise_variable_566 != 0.0) {
        let noise_metadata_schedule_489_e4802: f64 = (noise_variable_167 * noise_variable_160);
        (noise_metadata_schedule_489_e4802,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_489_e4804;
        }
        if matches!(source_index, 18) {
            let (noise_metadata_schedule_490_e4810,) = {
    if (noise_variable_566 != 0.0) {
        let noise_metadata_schedule_490_e4808: f64 = (noise_variable_182 * noise_variable_160);
        (noise_metadata_schedule_490_e4808,)
    } else {
        (noise_variable_182,)
    }
};
            noise_variable_182 = noise_metadata_schedule_490_e4810;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_491_e4835,) = {
    if (noise_variable_566 != 0.0) {
        let noise_metadata_schedule_491_e4814: f64 = (params.p33 * 2.0);
        let noise_metadata_schedule_491_e4816: f64 = (noise_metadata_schedule_491_e4814 * noise_variable_43);
        let noise_metadata_schedule_491_e4819: f64 = (noise_variable_275 - 1.0);
        let noise_metadata_schedule_491_e4820: f64 = (noise_metadata_schedule_491_e4816 * noise_metadata_schedule_491_e4819);
        let noise_metadata_schedule_491_e4825: f64 = (4.0 * noise_variable_43);
        let noise_metadata_schedule_491_e4827: f64 = (noise_metadata_schedule_491_e4825 / noise_variable_37);
        let noise_metadata_schedule_491_e4829: f64 = (noise_metadata_schedule_491_e4827 * noise_variable_275);
        let noise_metadata_schedule_491_e4830: f64 = (1.0 + noise_metadata_schedule_491_e4829);
        let noise_metadata_schedule_491_e4831: f64 = (noise_metadata_schedule_491_e4830).sqrt();
        let noise_metadata_schedule_491_e4832: f64 = (1.0 + noise_metadata_schedule_491_e4831);
        let noise_metadata_schedule_491_e4833: f64 = (noise_metadata_schedule_491_e4820 / noise_metadata_schedule_491_e4832);
        (noise_metadata_schedule_491_e4833,)
    } else {
        (noise_variable_174,)
    }
};
            noise_variable_174 = noise_metadata_schedule_491_e4835;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let noise_metadata_schedule_492_e4838: f64 = if params.p8 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_567 = noise_metadata_schedule_492_e4838;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_493_e4873,) = {
    if ((noise_variable_566 != 0.0) && (noise_variable_567 != 0.0)) {
        let noise_metadata_schedule_493_e4844: f64 = (1.0 - params.p143);
        let noise_metadata_schedule_493_e4846: f64 = (noise_metadata_schedule_493_e4844 * params.p33);
        let noise_metadata_schedule_493_e4848: f64 = (noise_metadata_schedule_493_e4846 * 2.0);
        let noise_metadata_schedule_493_e4850: f64 = (noise_metadata_schedule_493_e4848 * noise_variable_107);
        let noise_metadata_schedule_493_e4853: f64 = (noise_variable_275 - noise_variable_263);
        let noise_metadata_schedule_493_e4854: f64 = (noise_metadata_schedule_493_e4850 * noise_metadata_schedule_493_e4853);
        let noise_metadata_schedule_493_e4859: f64 = (4.0 * noise_variable_107);
        let noise_metadata_schedule_493_e4861: f64 = (noise_metadata_schedule_493_e4859 / noise_variable_109);
        let noise_metadata_schedule_493_e4865: f64 = (params.p144 * noise_variable_263);
        let noise_metadata_schedule_493_e4866: f64 = (noise_variable_275 + noise_metadata_schedule_493_e4865);
        let noise_metadata_schedule_493_e4867: f64 = (noise_metadata_schedule_493_e4861 * noise_metadata_schedule_493_e4866);
        let noise_metadata_schedule_493_e4868: f64 = (1.0 + noise_metadata_schedule_493_e4867);
        let noise_metadata_schedule_493_e4869: f64 = (noise_metadata_schedule_493_e4868).sqrt();
        let noise_metadata_schedule_493_e4870: f64 = (1.0 + noise_metadata_schedule_493_e4869);
        let noise_metadata_schedule_493_e4871: f64 = (noise_metadata_schedule_493_e4854 / noise_metadata_schedule_493_e4870);
        (noise_metadata_schedule_493_e4871,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_493_e4873;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_494_e4905,) = {
    if ((noise_variable_566 != 0.0) && (noise_variable_567 == 0.0)) {
        let noise_metadata_schedule_494_e4880: f64 = (1.0 - params.p143);
        let noise_metadata_schedule_494_e4882: f64 = (noise_metadata_schedule_494_e4880 * params.p33);
        let noise_metadata_schedule_494_e4884: f64 = (noise_metadata_schedule_494_e4882 * 2.0);
        let noise_metadata_schedule_494_e4886: f64 = (noise_metadata_schedule_494_e4884 * noise_variable_107);
        let noise_metadata_schedule_494_e4889: f64 = (noise_variable_275 - 1.0);
        let noise_metadata_schedule_494_e4890: f64 = (noise_metadata_schedule_494_e4886 * noise_metadata_schedule_494_e4889);
        let noise_metadata_schedule_494_e4895: f64 = (4.0 * noise_variable_107);
        let noise_metadata_schedule_494_e4897: f64 = (noise_metadata_schedule_494_e4895 / noise_variable_109);
        let noise_metadata_schedule_494_e4899: f64 = (noise_metadata_schedule_494_e4897 * noise_variable_275);
        let noise_metadata_schedule_494_e4900: f64 = (1.0 + noise_metadata_schedule_494_e4899);
        let noise_metadata_schedule_494_e4901: f64 = (noise_metadata_schedule_494_e4900).sqrt();
        let noise_metadata_schedule_494_e4902: f64 = (1.0 + noise_metadata_schedule_494_e4901);
        let noise_metadata_schedule_494_e4903: f64 = (noise_metadata_schedule_494_e4890 / noise_metadata_schedule_494_e4902);
        (noise_metadata_schedule_494_e4903,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_494_e4905;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_495_e4908: f64 = if params.p5 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_568 = noise_metadata_schedule_495_e4908;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_496_e4920,) = {
    if ((noise_variable_566 != 0.0) && (noise_variable_568 != 0.0)) {
        let noise_metadata_schedule_496_e4915: f64 = (noise_variable_43 + noise_variable_107);
        let noise_metadata_schedule_496_e4916: f64 = (params.p33 * noise_metadata_schedule_496_e4915);
        let noise_metadata_schedule_496_e4918: f64 = (noise_metadata_schedule_496_e4916 * noise_variable_32);
        (noise_metadata_schedule_496_e4918,)
    } else {
        (noise_variable_297,)
    }
};
            noise_variable_297 = noise_metadata_schedule_496_e4920;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_497_e4933,) = {
    if ((noise_variable_566 != 0.0) && (noise_variable_568 != 0.0)) {
        let noise_metadata_schedule_497_e4928: f64 = (noise_variable_297 * noise_variable_8);
        let noise_metadata_schedule_497_e4929: f64 = (noise_metadata_schedule_497_e4928).ln();
        let noise_metadata_schedule_497_e4930: f64 = (2.0 - noise_metadata_schedule_497_e4929);
        let noise_metadata_schedule_497_e4931: f64 = (noise_variable_6 * noise_metadata_schedule_497_e4930);
        (noise_metadata_schedule_497_e4931,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_497_e4933;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_498_e4941,) = {
    if ((noise_variable_566 != 0.0) && (noise_variable_568 != 0.0)) {
        let noise_metadata_schedule_498_e4939: f64 = (noise_variable_267 - noise_variable_176);
        (noise_metadata_schedule_498_e4939,)
    } else {
        (noise_variable_290,)
    }
};
            noise_variable_290 = noise_metadata_schedule_498_e4941;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_499_e4949,) = {
    if ((noise_variable_566 != 0.0) && (noise_variable_568 != 0.0)) {
        let noise_metadata_schedule_499_e4947: f64 = (0.11 * 0.11);
        (noise_metadata_schedule_499_e4947,)
    } else {
        (noise_variable_287,)
    }
};
            noise_variable_287 = noise_metadata_schedule_499_e4949;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_500_e4957,) = {
    if ((noise_variable_566 != 0.0) && (noise_variable_568 != 0.0)) {
        let noise_metadata_schedule_500_e4955: f64 = (noise_variable_290 * noise_variable_290);
        (noise_metadata_schedule_500_e4955,)
    } else {
        (noise_variable_288,)
    }
};
            noise_variable_288 = noise_metadata_schedule_500_e4957;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let noise_metadata_schedule_501_e4960: f64 = if noise_variable_290 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_569 = noise_metadata_schedule_501_e4960;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_502_e4977,) = {
    if (((noise_variable_566 != 0.0) && (noise_variable_568 != 0.0)) && (noise_variable_569 != 0.0)) {
        let noise_metadata_schedule_502_e4968: f64 = (0.5 * noise_variable_287);
        let noise_metadata_schedule_502_e4971: f64 = (noise_variable_288 + noise_variable_287);
        let noise_metadata_schedule_502_e4972: f64 = (noise_metadata_schedule_502_e4971).sqrt();
        let noise_metadata_schedule_502_e4974: f64 = (noise_metadata_schedule_502_e4972 - noise_variable_290);
        let noise_metadata_schedule_502_e4975: f64 = (noise_metadata_schedule_502_e4968 / noise_metadata_schedule_502_e4974);
        (noise_metadata_schedule_502_e4975,)
    } else {
        (noise_variable_177,)
    }
};
            noise_variable_177 = noise_metadata_schedule_502_e4977;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_503_e4993,) = {
    if (((noise_variable_566 != 0.0) && (noise_variable_568 != 0.0)) && (noise_variable_569 == 0.0)) {
        let noise_metadata_schedule_503_e4987: f64 = (noise_variable_288 + noise_variable_287);
        let noise_metadata_schedule_503_e4988: f64 = (noise_metadata_schedule_503_e4987).sqrt();
        let noise_metadata_schedule_503_e4990: f64 = (noise_metadata_schedule_503_e4988 + noise_variable_290);
        let noise_metadata_schedule_503_e4991: f64 = (0.5 * noise_metadata_schedule_503_e4990);
        (noise_metadata_schedule_503_e4991,)
    } else {
        (noise_variable_177,)
    }
};
            noise_variable_177 = noise_metadata_schedule_503_e4993;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_504_e5009,) = {
    if ((noise_variable_566 != 0.0) && (noise_variable_568 != 0.0)) {
        let noise_metadata_schedule_504_e5001: f64 = (noise_variable_174 + noise_variable_175);
        let noise_metadata_schedule_504_e5003: f64 = (noise_metadata_schedule_504_e5001 * noise_variable_32);
        let noise_metadata_schedule_504_e5004: f64 = (noise_variable_297 + noise_metadata_schedule_504_e5003);
        let noise_metadata_schedule_504_e5006: f64 = (noise_metadata_schedule_504_e5004 + noise_variable_177);
        let noise_metadata_schedule_504_e5007: f64 = (noise_variable_177 / noise_metadata_schedule_504_e5006);
        (noise_metadata_schedule_504_e5007,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_504_e5009;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_508_e5037,) = {
    if ((noise_variable_566 != 0.0) && (noise_variable_568 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_508_e5037;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_509_e5043,) = {
    if (noise_variable_566 != 0.0) {
        let noise_metadata_schedule_509_e5041: f64 = (noise_variable_178 * noise_variable_174);
        (noise_metadata_schedule_509_e5041,)
    } else {
        (noise_variable_179,)
    }
};
            noise_variable_179 = noise_metadata_schedule_509_e5043;
        }
        if matches!(source_index, 19) {
            let (noise_metadata_schedule_510_e5049,) = {
    if (noise_variable_566 != 0.0) {
        let noise_metadata_schedule_510_e5047: f64 = (noise_variable_178 * noise_variable_175);
        (noise_metadata_schedule_510_e5047,)
    } else {
        (noise_variable_183,)
    }
};
            noise_variable_183 = noise_metadata_schedule_510_e5049;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_511_e5052: f64 = if params.p84 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_570 = noise_metadata_schedule_511_e5052;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_512_e5058,) = {
    if (noise_variable_570 != 0.0) {
        let noise_metadata_schedule_512_e5056: f64 = (noise_variable_254 + noise_variable_250);
        (noise_metadata_schedule_512_e5056,)
    } else {
        (noise_variable_353,)
    }
};
            noise_variable_353 = noise_metadata_schedule_512_e5058;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_513_e5064,) = {
    if (noise_variable_570 != 0.0) {
        let noise_metadata_schedule_513_e5062: f64 = (1e-6 * 1e-6);
        (noise_metadata_schedule_513_e5062,)
    } else {
        (noise_variable_287,)
    }
};
            noise_variable_287 = noise_metadata_schedule_513_e5064;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_514_e5076,) = {
    if (noise_variable_570 != 0.0) {
        let noise_metadata_schedule_514_e5067: f64 = (-1.0);
        let noise_metadata_schedule_514_e5069: f64 = (noise_metadata_schedule_514_e5067 * noise_variable_353);
        let noise_metadata_schedule_514_e5071: f64 = (-1.0);
        let noise_metadata_schedule_514_e5072: f64 = (noise_metadata_schedule_514_e5069 * noise_metadata_schedule_514_e5071);
        let noise_metadata_schedule_514_e5074: f64 = (noise_metadata_schedule_514_e5072 * noise_variable_353);
        (noise_metadata_schedule_514_e5074,)
    } else {
        (noise_variable_288,)
    }
};
            noise_variable_288 = noise_metadata_schedule_514_e5076;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_515_e5078: f64 = (-1.0);
            let noise_metadata_schedule_515_e5080: f64 = (noise_metadata_schedule_515_e5078 * noise_variable_353);
            let noise_metadata_schedule_515_e5082: f64 = if noise_metadata_schedule_515_e5080 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_571 = noise_metadata_schedule_515_e5082;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_516_e5100,) = {
    if ((noise_variable_570 != 0.0) && (noise_variable_571 != 0.0)) {
        let noise_metadata_schedule_516_e5088: f64 = (0.5 * noise_variable_287);
        let noise_metadata_schedule_516_e5091: f64 = (noise_variable_288 + noise_variable_287);
        let noise_metadata_schedule_516_e5092: f64 = (noise_metadata_schedule_516_e5091).sqrt();
        let noise_metadata_schedule_516_e5094: f64 = (-1.0);
        let noise_metadata_schedule_516_e5096: f64 = (noise_metadata_schedule_516_e5094 * noise_variable_353);
        let noise_metadata_schedule_516_e5097: f64 = (noise_metadata_schedule_516_e5092 - noise_metadata_schedule_516_e5096);
        let noise_metadata_schedule_516_e5098: f64 = (noise_metadata_schedule_516_e5088 / noise_metadata_schedule_516_e5097);
        (noise_metadata_schedule_516_e5098,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_516_e5100;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_517_e5117,) = {
    if ((noise_variable_570 != 0.0) && (noise_variable_571 == 0.0)) {
        let noise_metadata_schedule_517_e5108: f64 = (noise_variable_288 + noise_variable_287);
        let noise_metadata_schedule_517_e5109: f64 = (noise_metadata_schedule_517_e5108).sqrt();
        let noise_metadata_schedule_517_e5111: f64 = (-1.0);
        let noise_metadata_schedule_517_e5113: f64 = (noise_metadata_schedule_517_e5111 * noise_variable_353);
        let noise_metadata_schedule_517_e5114: f64 = (noise_metadata_schedule_517_e5109 + noise_metadata_schedule_517_e5113);
        let noise_metadata_schedule_517_e5115: f64 = (0.5 * noise_metadata_schedule_517_e5114);
        (noise_metadata_schedule_517_e5115,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_517_e5117;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_518_e5127,) = {
    if (noise_variable_570 != 0.0) {
        let noise_metadata_schedule_518_e5123: f64 = (noise_variable_349).powf(params.p82);
        let noise_metadata_schedule_518_e5124: f64 = (1.0 - noise_metadata_schedule_518_e5123);
        let noise_metadata_schedule_518_e5125: f64 = (1.0 / noise_metadata_schedule_518_e5124);
        (noise_metadata_schedule_518_e5125,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_518_e5127;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_519_e5133,) = {
    if (noise_variable_570 != 0.0) {
        let noise_metadata_schedule_519_e5131: f64 = (noise_variable_349 * params.p81);
        (noise_metadata_schedule_519_e5131,)
    } else {
        (noise_variable_350,)
    }
};
            noise_variable_350 = noise_metadata_schedule_519_e5133;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_520_e5149,) = {
    if (noise_variable_570 != 0.0) {
        let noise_metadata_schedule_520_e5137: f64 = (noise_variable_355 * noise_variable_355);
        let noise_metadata_schedule_520_e5141: f64 = (params.p82 - 1.0);
        let noise_metadata_schedule_520_e5142: f64 = (noise_variable_349).powf(noise_metadata_schedule_520_e5141);
        let noise_metadata_schedule_520_e5143: f64 = (noise_metadata_schedule_520_e5137 * noise_metadata_schedule_520_e5142);
        let noise_metadata_schedule_520_e5145: f64 = (noise_metadata_schedule_520_e5143 * params.p82);
        let noise_metadata_schedule_520_e5147: f64 = (noise_metadata_schedule_520_e5145 / params.p81);
        (noise_metadata_schedule_520_e5147,)
    } else {
        (noise_variable_352,)
    }
};
            noise_variable_352 = noise_metadata_schedule_520_e5149;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_521_e5152: f64 = if noise_variable_354 < noise_variable_350 { 1.0 } else { 0.0 };
            noise_variable_572 = noise_metadata_schedule_521_e5152;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_522_e5166,) = {
    if ((noise_variable_570 != 0.0) && (noise_variable_572 != 0.0)) {
        let noise_metadata_schedule_522_e5160: f64 = (noise_variable_354 / params.p81);
        let noise_metadata_schedule_522_e5162: f64 = (noise_metadata_schedule_522_e5160).powf(params.p82);
        let noise_metadata_schedule_522_e5163: f64 = (1.0 - noise_metadata_schedule_522_e5162);
        let noise_metadata_schedule_522_e5164: f64 = (1.0 / noise_metadata_schedule_522_e5163);
        (noise_metadata_schedule_522_e5164,)
    } else {
        (noise_variable_351,)
    }
};
            noise_variable_351 = noise_metadata_schedule_522_e5166;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_523_e5179,) = {
    if ((noise_variable_570 != 0.0) && (noise_variable_572 == 0.0)) {
        let noise_metadata_schedule_523_e5174: f64 = (noise_variable_354 - noise_variable_350);
        let noise_metadata_schedule_523_e5176: f64 = (noise_metadata_schedule_523_e5174 * noise_variable_352);
        let noise_metadata_schedule_523_e5177: f64 = (noise_variable_355 + noise_metadata_schedule_523_e5176);
        (noise_metadata_schedule_523_e5177,)
    } else {
        (noise_variable_351,)
    }
};
            noise_variable_351 = noise_metadata_schedule_523_e5179;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_524_e5184,) = {
    if (noise_variable_570 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_351,)
    }
};
            noise_variable_351 = noise_metadata_schedule_524_e5184;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_525_e5187: f64 = (noise_variable_82 * noise_variable_351);
            noise_variable_82 = noise_metadata_schedule_525_e5187;
        }
        if matches!(source_index, 11 | 12) {
            let noise_metadata_schedule_526_e5190: f64 = (noise_variable_167 * noise_variable_351);
            noise_variable_167 = noise_metadata_schedule_526_e5190;
        }
        if matches!(source_index, 9 | 10) {
            let noise_metadata_schedule_527_e5193: f64 = (noise_variable_164 * noise_variable_351);
            noise_variable_164 = noise_metadata_schedule_527_e5193;
        }
        if matches!(source_index, 13 | 14) {
            let noise_metadata_schedule_528_e5196: f64 = (noise_variable_179 * noise_variable_351);
            noise_variable_179 = noise_metadata_schedule_528_e5196;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_529_e5200: f64 = (noise_variable_141 / noise_variable_41);
            let noise_metadata_schedule_529_e5201: f64 = (1.0 + noise_metadata_schedule_529_e5200);
            let noise_metadata_schedule_529_e5204: f64 = (noise_variable_148 / noise_variable_40);
            let noise_metadata_schedule_529_e5205: f64 = (noise_metadata_schedule_529_e5201 + noise_metadata_schedule_529_e5204);
            noise_variable_186 = noise_metadata_schedule_529_e5205;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_530_e5208: f64 = (0.1 * 0.1);
            noise_variable_287 = noise_metadata_schedule_530_e5208;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_531_e5211: f64 = (noise_variable_186 * noise_variable_186);
            noise_variable_288 = noise_metadata_schedule_531_e5211;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_532_e5214: f64 = if noise_variable_186 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_573 = noise_metadata_schedule_532_e5214;
        }
        if matches!(source_index, 1 | 5) {
            let (noise_metadata_schedule_533_e5227,) = {
    if (noise_variable_573 != 0.0) {
        let noise_metadata_schedule_533_e5218: f64 = (0.5 * noise_variable_287);
        let noise_metadata_schedule_533_e5221: f64 = (noise_variable_288 + noise_variable_287);
        let noise_metadata_schedule_533_e5222: f64 = (noise_metadata_schedule_533_e5221).sqrt();
        let noise_metadata_schedule_533_e5224: f64 = (noise_metadata_schedule_533_e5222 - noise_variable_186);
        let noise_metadata_schedule_533_e5225: f64 = (noise_metadata_schedule_533_e5218 / noise_metadata_schedule_533_e5224);
        (noise_metadata_schedule_533_e5225,)
    } else {
        (noise_variable_187,)
    }
};
            noise_variable_187 = noise_metadata_schedule_533_e5227;
        }
        if matches!(source_index, 1 | 5) {
            let (noise_metadata_schedule_534_e5239,) = {
    if (noise_variable_573 == 0.0) {
        let noise_metadata_schedule_534_e5233: f64 = (noise_variable_288 + noise_variable_287);
        let noise_metadata_schedule_534_e5234: f64 = (noise_metadata_schedule_534_e5233).sqrt();
        let noise_metadata_schedule_534_e5236: f64 = (noise_metadata_schedule_534_e5234 + noise_variable_186);
        let noise_metadata_schedule_534_e5237: f64 = (0.5 * noise_metadata_schedule_534_e5236);
        (noise_metadata_schedule_534_e5237,)
    } else {
        (noise_variable_187,)
    }
};
            noise_variable_187 = noise_metadata_schedule_534_e5239;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_535_e5245: f64 = (noise_variable_152 + noise_variable_153);
            let noise_metadata_schedule_535_e5246: f64 = (0.5 * noise_metadata_schedule_535_e5245);
            let noise_metadata_schedule_535_e5247: f64 = (1.0 + noise_metadata_schedule_535_e5246);
            let noise_metadata_schedule_535_e5248: f64 = (noise_variable_187 * noise_metadata_schedule_535_e5247);
            noise_variable_188 = noise_metadata_schedule_535_e5248;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_536_e5251: f64 = (noise_variable_29 / noise_variable_188);
            noise_variable_190 = noise_metadata_schedule_536_e5251;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_537_e5254: f64 = if noise_variable_190 < noise_variable_346 { 1.0 } else { 0.0 };
            noise_variable_574 = noise_metadata_schedule_537_e5254;
        }
        if matches!(source_index, 1 | 5) {
            let (noise_metadata_schedule_538_e5258,) = {
    if (noise_variable_574 != 0.0) {
        (noise_variable_346,)
    } else {
        (noise_variable_190,)
    }
};
            noise_variable_190 = noise_metadata_schedule_538_e5258;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_539_e5261: f64 = (3.0 * noise_variable_190);
            noise_variable_189 = noise_metadata_schedule_539_e5261;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_541_e5275: f64 = if noise_variable_159 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_575 = noise_metadata_schedule_541_e5275;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_542_e5278: f64 = if params.p39 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_576 = noise_metadata_schedule_542_e5278;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_543_e5281: f64 = if noise_variable_250 < params.p44 { 1.0 } else { 0.0 };
            noise_variable_577 = noise_metadata_schedule_543_e5281;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_544_e5283: f64 = (-noise_variable_159);
            let noise_metadata_schedule_544_e5285: f64 = (noise_metadata_schedule_544_e5283 / params.p42);
            let noise_metadata_schedule_544_e5287: f64 = if noise_metadata_schedule_544_e5285 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_578 = noise_metadata_schedule_544_e5287;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_545_e5301,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 != 0.0)) && (noise_variable_577 != 0.0)) && (noise_variable_578 != 0.0)) {
        let noise_metadata_schedule_545_e5296: f64 = (-noise_variable_159);
        let noise_metadata_schedule_545_e5298: f64 = (noise_metadata_schedule_545_e5296 / params.p42);
        let noise_metadata_schedule_545_e5299: f64 = (noise_metadata_schedule_545_e5298).exp();
        (noise_metadata_schedule_545_e5299,)
    } else {
        (noise_variable_338,)
    }
};
            noise_variable_338 = noise_metadata_schedule_545_e5301;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_546_e5313,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 != 0.0)) && (noise_variable_577 != 0.0)) && (noise_variable_578 == 0.0)) {
        let noise_metadata_schedule_546_e5311: f64 = (params.p151).exp();
        (noise_metadata_schedule_546_e5311,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_546_e5313;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_547_e5333,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 != 0.0)) && (noise_variable_577 != 0.0)) && (noise_variable_578 == 0.0)) {
        let noise_metadata_schedule_547_e5325: f64 = (-noise_variable_159);
        let noise_metadata_schedule_547_e5327: f64 = (noise_metadata_schedule_547_e5325 / params.p42);
        let noise_metadata_schedule_547_e5329: f64 = (noise_metadata_schedule_547_e5327 - params.p151);
        let noise_metadata_schedule_547_e5330: f64 = (1.0 + noise_metadata_schedule_547_e5329);
        let noise_metadata_schedule_547_e5331: f64 = (noise_variable_301 * noise_metadata_schedule_547_e5330);
        (noise_metadata_schedule_547_e5331,)
    } else {
        (noise_variable_338,)
    }
};
            noise_variable_338 = noise_metadata_schedule_547_e5333;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_548_e5345,) = {
    if (((noise_variable_575 != 0.0) && (noise_variable_576 != 0.0)) && (noise_variable_577 != 0.0)) {
        let noise_metadata_schedule_548_e5341: f64 = (params.p44 - noise_variable_250);
        let noise_metadata_schedule_548_e5343: f64 = (noise_metadata_schedule_548_e5341 * noise_variable_338);
        (noise_metadata_schedule_548_e5343,)
    } else {
        (noise_variable_339,)
    }
};
            noise_variable_339 = noise_metadata_schedule_548_e5345;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_549_e5347: f64 = (-noise_variable_340);
            let noise_metadata_schedule_549_e5350: f64 = (noise_variable_339).powf(params.p41);
            let noise_metadata_schedule_549_e5351: f64 = (noise_metadata_schedule_549_e5347 * noise_metadata_schedule_549_e5350);
            let noise_metadata_schedule_549_e5353: f64 = if noise_metadata_schedule_549_e5351 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_579 = noise_metadata_schedule_549_e5353;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_550_e5369,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 != 0.0)) && (noise_variable_577 != 0.0)) && (noise_variable_579 != 0.0)) {
        let noise_metadata_schedule_550_e5362: f64 = (-noise_variable_340);
        let noise_metadata_schedule_550_e5365: f64 = (noise_variable_339).powf(params.p41);
        let noise_metadata_schedule_550_e5366: f64 = (noise_metadata_schedule_550_e5362 * noise_metadata_schedule_550_e5365);
        let noise_metadata_schedule_550_e5367: f64 = (noise_metadata_schedule_550_e5366).exp();
        (noise_metadata_schedule_550_e5367,)
    } else {
        (noise_variable_343,)
    }
};
            noise_variable_343 = noise_metadata_schedule_550_e5369;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_551_e5381,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 != 0.0)) && (noise_variable_577 != 0.0)) && (noise_variable_579 == 0.0)) {
        let noise_metadata_schedule_551_e5379: f64 = (params.p151).exp();
        (noise_metadata_schedule_551_e5379,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_551_e5381;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_552_e5403,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 != 0.0)) && (noise_variable_577 != 0.0)) && (noise_variable_579 == 0.0)) {
        let noise_metadata_schedule_552_e5393: f64 = (-noise_variable_340);
        let noise_metadata_schedule_552_e5396: f64 = (noise_variable_339).powf(params.p41);
        let noise_metadata_schedule_552_e5397: f64 = (noise_metadata_schedule_552_e5393 * noise_metadata_schedule_552_e5396);
        let noise_metadata_schedule_552_e5399: f64 = (noise_metadata_schedule_552_e5397 - params.p151);
        let noise_metadata_schedule_552_e5400: f64 = (1.0 + noise_metadata_schedule_552_e5399);
        let noise_metadata_schedule_552_e5401: f64 = (noise_variable_301 * noise_metadata_schedule_552_e5400);
        (noise_metadata_schedule_552_e5401,)
    } else {
        (noise_variable_343,)
    }
};
            noise_variable_343 = noise_metadata_schedule_552_e5403;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_553_e5417,) = {
    if (((noise_variable_575 != 0.0) && (noise_variable_576 != 0.0)) && (noise_variable_577 != 0.0)) {
        let noise_metadata_schedule_553_e5411: f64 = (params.p40 / noise_variable_340);
        let noise_metadata_schedule_553_e5413: f64 = (noise_metadata_schedule_553_e5411 * noise_variable_339);
        let noise_metadata_schedule_553_e5415: f64 = (noise_metadata_schedule_553_e5413 * noise_variable_343);
        (noise_metadata_schedule_553_e5415,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_553_e5417;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_554_e5420: f64 = if params.p39 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_580 = noise_metadata_schedule_554_e5420;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_555_e5423: f64 = if noise_variable_250 < noise_variable_16 { 1.0 } else { 0.0 };
            noise_variable_581 = noise_metadata_schedule_555_e5423;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_556_e5440,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) {
        let noise_metadata_schedule_556_e5434: f64 = (2.0 * params.p46);
        let noise_metadata_schedule_556_e5437: f64 = (params.p45 * params.p45);
        let noise_metadata_schedule_556_e5438: f64 = (noise_metadata_schedule_556_e5434 / noise_metadata_schedule_556_e5437);
        (noise_metadata_schedule_556_e5438,)
    } else {
        (noise_variable_199,)
    }
};
            noise_variable_199 = noise_metadata_schedule_556_e5440;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_557_e5455,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) {
        let noise_metadata_schedule_557_e5451: f64 = (noise_variable_16 - noise_variable_250);
        let noise_metadata_schedule_557_e5453: f64 = (noise_metadata_schedule_557_e5451 / noise_variable_213);
        (noise_metadata_schedule_557_e5453,)
    } else {
        (noise_variable_286,)
    }
};
            noise_variable_286 = noise_metadata_schedule_557_e5455;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_558_e5471,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) {
        let noise_metadata_schedule_558_e5466: f64 = (2.0 * noise_variable_286);
        let noise_metadata_schedule_558_e5468: f64 = (noise_metadata_schedule_558_e5466 / noise_variable_199);
        let noise_metadata_schedule_558_e5469: f64 = (noise_metadata_schedule_558_e5468).sqrt();
        (noise_metadata_schedule_558_e5469,)
    } else {
        (noise_variable_200,)
    }
};
            noise_variable_200 = noise_metadata_schedule_558_e5471;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_559_e5474: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_582 = noise_metadata_schedule_559_e5474;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_560_e5487,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_582 != 0.0)) {
        (params.p45,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_560_e5487;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_561_e5505,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_582 == 0.0)) {
        let noise_metadata_schedule_561_e5502: f64 = (0.5 * noise_variable_125);
        let noise_metadata_schedule_561_e5503: f64 = (1.0 - noise_metadata_schedule_561_e5502);
        (noise_metadata_schedule_561_e5503,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_561_e5505;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_562_e5523,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_582 == 0.0)) {
        let noise_metadata_schedule_562_e5519: f64 = (params.p45 * noise_variable_126);
        let noise_metadata_schedule_562_e5521: f64 = (noise_metadata_schedule_562_e5519 * noise_variable_126);
        (noise_metadata_schedule_562_e5521,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_562_e5523;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_563_e5545,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) {
        let noise_metadata_schedule_563_e5534: f64 = (noise_variable_200 * noise_variable_201);
        let noise_metadata_schedule_563_e5537: f64 = (noise_variable_200 * noise_variable_200);
        let noise_metadata_schedule_563_e5540: f64 = (noise_variable_201 * noise_variable_201);
        let noise_metadata_schedule_563_e5541: f64 = (noise_metadata_schedule_563_e5537 + noise_metadata_schedule_563_e5540);
        let noise_metadata_schedule_563_e5542: f64 = (noise_metadata_schedule_563_e5541).sqrt();
        let noise_metadata_schedule_563_e5543: f64 = (noise_metadata_schedule_563_e5534 / noise_metadata_schedule_563_e5542);
        (noise_metadata_schedule_563_e5543,)
    } else {
        (noise_variable_202,)
    }
};
            noise_variable_202 = noise_metadata_schedule_563_e5545;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_564_e5560,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) {
        let noise_metadata_schedule_564_e5556: f64 = (noise_variable_16 - noise_variable_250);
        let noise_metadata_schedule_564_e5558: f64 = (noise_metadata_schedule_564_e5556 / noise_variable_202);
        (noise_metadata_schedule_564_e5558,)
    } else {
        (noise_variable_203,)
    }
};
            noise_variable_203 = noise_metadata_schedule_564_e5560;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_565_e5579,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) {
        let noise_metadata_schedule_565_e5572: f64 = (0.5 * noise_variable_202);
        let noise_metadata_schedule_565_e5574: f64 = (noise_metadata_schedule_565_e5572 * noise_variable_199);
        let noise_metadata_schedule_565_e5576: f64 = (noise_metadata_schedule_565_e5574 * noise_variable_213);
        let noise_metadata_schedule_565_e5577: f64 = (noise_variable_203 + noise_metadata_schedule_565_e5576);
        (noise_metadata_schedule_565_e5577,)
    } else {
        (noise_variable_204,)
    }
};
            noise_variable_204 = noise_metadata_schedule_565_e5579;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_566_e5582: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_583 = noise_metadata_schedule_566_e5582;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_567_e5595,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_583 != 0.0)) {
        (noise_variable_204,)
    } else {
        (noise_variable_205,)
    }
};
            noise_variable_205 = noise_metadata_schedule_567_e5595;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_568_e5619,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_583 == 0.0)) {
        let noise_metadata_schedule_568_e5610: f64 = (2.0 * params.p47);
        let noise_metadata_schedule_568_e5614: f64 = (2.0 * noise_variable_125);
        let noise_metadata_schedule_568_e5615: f64 = (1.0 + noise_metadata_schedule_568_e5614);
        let noise_metadata_schedule_568_e5616: f64 = (noise_metadata_schedule_568_e5610 * noise_metadata_schedule_568_e5615);
        let noise_metadata_schedule_568_e5617: f64 = (1.0 + noise_metadata_schedule_568_e5616);
        (noise_metadata_schedule_568_e5617,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_568_e5619;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_569_e5641,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_583 == 0.0)) {
        let noise_metadata_schedule_569_e5633: f64 = (1.0 + params.p47);
        let noise_metadata_schedule_569_e5637: f64 = (2.0 * params.p47);
        let noise_metadata_schedule_569_e5638: f64 = (1.0 + noise_metadata_schedule_569_e5637);
        let noise_metadata_schedule_569_e5639: f64 = (noise_metadata_schedule_569_e5633 / noise_metadata_schedule_569_e5638);
        (noise_metadata_schedule_569_e5639,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_569_e5641;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_570_e5669,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_583 == 0.0)) {
        let noise_metadata_schedule_570_e5656: f64 = (0.5 * noise_variable_202);
        let noise_metadata_schedule_570_e5658: f64 = (noise_metadata_schedule_570_e5656 * noise_variable_199);
        let noise_metadata_schedule_570_e5663: f64 = (params.p62 * noise_variable_206);
        let noise_metadata_schedule_570_e5664: f64 = (noise_variable_159 / noise_metadata_schedule_570_e5663);
        let noise_metadata_schedule_570_e5665: f64 = (noise_variable_207 - noise_metadata_schedule_570_e5664);
        let noise_metadata_schedule_570_e5666: f64 = (noise_metadata_schedule_570_e5658 * noise_metadata_schedule_570_e5665);
        let noise_metadata_schedule_570_e5667: f64 = (noise_variable_203 - noise_metadata_schedule_570_e5666);
        (noise_metadata_schedule_570_e5667,)
    } else {
        (noise_variable_208,)
    }
};
            noise_variable_208 = noise_metadata_schedule_570_e5669;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_571_e5699,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_583 == 0.0)) {
        let noise_metadata_schedule_571_e5683: f64 = (noise_variable_208 - noise_variable_204);
        let noise_metadata_schedule_571_e5686: f64 = (noise_variable_208 - noise_variable_204);
        let noise_metadata_schedule_571_e5687: f64 = (noise_metadata_schedule_571_e5683 * noise_metadata_schedule_571_e5686);
        let noise_metadata_schedule_571_e5690: f64 = (0.1 * noise_variable_203);
        let noise_metadata_schedule_571_e5692: f64 = (noise_metadata_schedule_571_e5690 * noise_variable_203);
        let noise_metadata_schedule_571_e5694: f64 = (noise_metadata_schedule_571_e5692 * noise_variable_137);
        let noise_metadata_schedule_571_e5696: f64 = (noise_metadata_schedule_571_e5694 / params.p62);
        let noise_metadata_schedule_571_e5697: f64 = (noise_metadata_schedule_571_e5687 + noise_metadata_schedule_571_e5696);
        (noise_metadata_schedule_571_e5697,)
    } else {
        (noise_variable_286,)
    }
};
            noise_variable_286 = noise_metadata_schedule_571_e5699;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_572_e5720,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_583 == 0.0)) {
        let noise_metadata_schedule_572_e5714: f64 = (noise_variable_208 + noise_variable_204);
        let noise_metadata_schedule_572_e5716: f64 = (noise_variable_286).sqrt();
        let noise_metadata_schedule_572_e5717: f64 = (noise_metadata_schedule_572_e5714 + noise_metadata_schedule_572_e5716);
        let noise_metadata_schedule_572_e5718: f64 = (0.5 * noise_metadata_schedule_572_e5717);
        (noise_metadata_schedule_572_e5718,)
    } else {
        (noise_variable_205,)
    }
};
            noise_variable_205 = noise_metadata_schedule_572_e5720;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_573_e5735,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) {
        let noise_metadata_schedule_573_e5731: f64 = (noise_variable_205 - noise_variable_203);
        let noise_metadata_schedule_573_e5733: f64 = (noise_metadata_schedule_573_e5731 / noise_variable_205);
        (noise_metadata_schedule_573_e5733,)
    } else {
        (noise_variable_293,)
    }
};
            noise_variable_293 = noise_metadata_schedule_573_e5735;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_574_e5737: f64 = (noise_variable_293).abs();
            let noise_metadata_schedule_574_e5739: f64 = if noise_metadata_schedule_574_e5737 > 1e-7 { 1.0 } else { 0.0 };
            noise_variable_584 = noise_metadata_schedule_574_e5739;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_575_e5756,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_584 != 0.0)) {
        let noise_metadata_schedule_575_e5752: f64 = (0.5 * noise_variable_202);
        let noise_metadata_schedule_575_e5754: f64 = (noise_metadata_schedule_575_e5752 / noise_variable_293);
        (noise_metadata_schedule_575_e5754,)
    } else {
        (noise_variable_209,)
    }
};
            noise_variable_209 = noise_metadata_schedule_575_e5756;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_576_e5793,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_584 != 0.0)) {
        let noise_metadata_schedule_576_e5769: f64 = (noise_variable_0 / noise_variable_99);
        let noise_metadata_schedule_576_e5771: f64 = (noise_metadata_schedule_576_e5769 * noise_variable_205);
        let noise_metadata_schedule_576_e5773: f64 = (noise_metadata_schedule_576_e5771 * noise_variable_209);
        let noise_metadata_schedule_576_e5775: f64 = (-noise_variable_99);
        let noise_metadata_schedule_576_e5777: f64 = (noise_metadata_schedule_576_e5775 / noise_variable_205);
        let noise_metadata_schedule_576_e5778: f64 = (noise_metadata_schedule_576_e5777).exp();
        let noise_metadata_schedule_576_e5780: f64 = (-noise_variable_99);
        let noise_metadata_schedule_576_e5782: f64 = (noise_metadata_schedule_576_e5780 / noise_variable_205);
        let noise_metadata_schedule_576_e5786: f64 = (noise_variable_201 / noise_variable_209);
        let noise_metadata_schedule_576_e5787: f64 = (1.0 + noise_metadata_schedule_576_e5786);
        let noise_metadata_schedule_576_e5788: f64 = (noise_metadata_schedule_576_e5782 * noise_metadata_schedule_576_e5787);
        let noise_metadata_schedule_576_e5789: f64 = (noise_metadata_schedule_576_e5788).exp();
        let noise_metadata_schedule_576_e5790: f64 = (noise_metadata_schedule_576_e5778 - noise_metadata_schedule_576_e5789);
        let noise_metadata_schedule_576_e5791: f64 = (noise_metadata_schedule_576_e5773 * noise_metadata_schedule_576_e5790);
        (noise_metadata_schedule_576_e5791,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_576_e5793;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_577_e5815,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_584 == 0.0)) {
        let noise_metadata_schedule_577_e5807: f64 = (noise_variable_0 * noise_variable_201);
        let noise_metadata_schedule_577_e5809: f64 = (-noise_variable_99);
        let noise_metadata_schedule_577_e5811: f64 = (noise_metadata_schedule_577_e5809 / noise_variable_205);
        let noise_metadata_schedule_577_e5812: f64 = (noise_metadata_schedule_577_e5811).exp();
        let noise_metadata_schedule_577_e5813: f64 = (noise_metadata_schedule_577_e5807 * noise_metadata_schedule_577_e5812);
        (noise_metadata_schedule_577_e5813,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_577_e5815;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_578_e5818: f64 = if params.p39 == 3.0 { 1.0 } else { 0.0 };
            noise_variable_585 = noise_metadata_schedule_578_e5818;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_579_e5821: f64 = if noise_variable_250 < params.p44 { 1.0 } else { 0.0 };
            noise_variable_586 = noise_metadata_schedule_579_e5821;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_580_e5849,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 == 0.0)) && (noise_variable_585 != 0.0)) && (noise_variable_586 != 0.0)) {
        let noise_metadata_schedule_580_e5835: f64 = (params.p44 - noise_variable_250);
        let noise_metadata_schedule_580_e5837: f64 = (noise_metadata_schedule_580_e5835).powf(params.p41);
        let noise_metadata_schedule_580_e5842: f64 = (params.p48 + noise_variable_159);
        let noise_metadata_schedule_580_e5843: f64 = (noise_variable_159 / noise_metadata_schedule_580_e5842);
        let noise_metadata_schedule_580_e5844: f64 = (1.0 - noise_metadata_schedule_580_e5843);
        let noise_metadata_schedule_580_e5846: f64 = (noise_metadata_schedule_580_e5844).powf(params.p49);
        let noise_metadata_schedule_580_e5847: f64 = (noise_metadata_schedule_580_e5837 * noise_metadata_schedule_580_e5846);
        (noise_metadata_schedule_580_e5847,)
    } else {
        (noise_variable_214,)
    }
};
            noise_variable_214 = noise_metadata_schedule_580_e5849;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_581_e5852: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_587 = noise_metadata_schedule_581_e5852;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_582_e5868,) = {
    if ((((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 == 0.0)) && (noise_variable_585 != 0.0)) && (noise_variable_586 != 0.0)) && (noise_variable_587 != 0.0)) {
        (noise_variable_214,)
    } else {
        (noise_variable_215,)
    }
};
            noise_variable_215 = noise_metadata_schedule_582_e5868;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_583_e5889,) = {
    if ((((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 == 0.0)) && (noise_variable_585 != 0.0)) && (noise_variable_586 != 0.0)) && (noise_variable_587 == 0.0)) {
        let noise_metadata_schedule_583_e5885: f64 = (noise_variable_159 - params.p52);
        let noise_metadata_schedule_583_e5887: f64 = (noise_metadata_schedule_583_e5885 / params.p48);
        (noise_metadata_schedule_583_e5887,)
    } else {
        (noise_variable_216,)
    }
};
            noise_variable_216 = noise_metadata_schedule_583_e5889;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_584_e5910,) = {
    if ((((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 == 0.0)) && (noise_variable_585 != 0.0)) && (noise_variable_586 != 0.0)) && (noise_variable_587 == 0.0)) {
        let noise_metadata_schedule_584_e5906: f64 = (noise_variable_216 - 1.0);
        let noise_metadata_schedule_584_e5908: f64 = (noise_metadata_schedule_584_e5906 / params.p51);
        (noise_metadata_schedule_584_e5908,)
    } else {
        (noise_variable_285,)
    }
};
            noise_variable_285 = noise_metadata_schedule_584_e5910;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_585_e5913: f64 = if noise_variable_216 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_588 = noise_metadata_schedule_585_e5913;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_586_e5940,) = {
    if (((((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 == 0.0)) && (noise_variable_585 != 0.0)) && (noise_variable_586 != 0.0)) && (noise_variable_587 == 0.0)) && (noise_variable_588 != 0.0)) {
        let noise_metadata_schedule_586_e5934: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_586_e5935: f64 = (1.0 + noise_metadata_schedule_586_e5934);
        let noise_metadata_schedule_586_e5936: f64 = (noise_metadata_schedule_586_e5935).ln();
        let noise_metadata_schedule_586_e5937: f64 = (params.p51 * noise_metadata_schedule_586_e5936);
        let noise_metadata_schedule_586_e5938: f64 = (1.0 + noise_metadata_schedule_586_e5937);
        (noise_metadata_schedule_586_e5938,)
    } else {
        (noise_variable_217,)
    }
};
            noise_variable_217 = noise_metadata_schedule_586_e5940;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_587_e5969,) = {
    if (((((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 == 0.0)) && (noise_variable_585 != 0.0)) && (noise_variable_586 != 0.0)) && (noise_variable_587 == 0.0)) && (noise_variable_588 == 0.0)) {
        let noise_metadata_schedule_587_e5962: f64 = (-noise_variable_285);
        let noise_metadata_schedule_587_e5963: f64 = (noise_metadata_schedule_587_e5962).exp();
        let noise_metadata_schedule_587_e5964: f64 = (1.0 + noise_metadata_schedule_587_e5963);
        let noise_metadata_schedule_587_e5965: f64 = (noise_metadata_schedule_587_e5964).ln();
        let noise_metadata_schedule_587_e5966: f64 = (params.p51 * noise_metadata_schedule_587_e5965);
        let noise_metadata_schedule_587_e5967: f64 = (noise_variable_216 + noise_metadata_schedule_587_e5966);
        (noise_metadata_schedule_587_e5967,)
    } else {
        (noise_variable_217,)
    }
};
            noise_variable_217 = noise_metadata_schedule_587_e5969;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_588_e5990,) = {
    if ((((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 == 0.0)) && (noise_variable_585 != 0.0)) && (noise_variable_586 != 0.0)) && (noise_variable_587 == 0.0)) {
        let noise_metadata_schedule_588_e5987: f64 = (noise_variable_217).powf(params.p50);
        let noise_metadata_schedule_588_e5988: f64 = (noise_variable_214 * noise_metadata_schedule_588_e5987);
        (noise_metadata_schedule_588_e5988,)
    } else {
        (noise_variable_215,)
    }
};
            noise_variable_215 = noise_metadata_schedule_588_e5990;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_589_e5992: f64 = (-noise_variable_340);
            let noise_metadata_schedule_589_e5994: f64 = (noise_metadata_schedule_589_e5992 * noise_variable_215);
            let noise_metadata_schedule_589_e5996: f64 = if noise_metadata_schedule_589_e5994 < params.p151 { 1.0 } else { 0.0 };
            noise_variable_589 = noise_metadata_schedule_589_e5996;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_590_e6016,) = {
    if ((((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 == 0.0)) && (noise_variable_585 != 0.0)) && (noise_variable_586 != 0.0)) && (noise_variable_589 != 0.0)) {
        let noise_metadata_schedule_590_e6011: f64 = (-noise_variable_340);
        let noise_metadata_schedule_590_e6013: f64 = (noise_metadata_schedule_590_e6011 * noise_variable_215);
        let noise_metadata_schedule_590_e6014: f64 = (noise_metadata_schedule_590_e6013).exp();
        (noise_metadata_schedule_590_e6014,)
    } else {
        (noise_variable_343,)
    }
};
            noise_variable_343 = noise_metadata_schedule_590_e6016;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_591_e6034,) = {
    if ((((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 == 0.0)) && (noise_variable_585 != 0.0)) && (noise_variable_586 != 0.0)) && (noise_variable_589 == 0.0)) {
        let noise_metadata_schedule_591_e6032: f64 = (params.p151).exp();
        (noise_metadata_schedule_591_e6032,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_591_e6034;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_592_e6060,) = {
    if ((((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 == 0.0)) && (noise_variable_585 != 0.0)) && (noise_variable_586 != 0.0)) && (noise_variable_589 == 0.0)) {
        let noise_metadata_schedule_592_e6052: f64 = (-noise_variable_340);
        let noise_metadata_schedule_592_e6054: f64 = (noise_metadata_schedule_592_e6052 * noise_variable_215);
        let noise_metadata_schedule_592_e6056: f64 = (noise_metadata_schedule_592_e6054 - params.p151);
        let noise_metadata_schedule_592_e6057: f64 = (1.0 + noise_metadata_schedule_592_e6056);
        let noise_metadata_schedule_592_e6058: f64 = (noise_variable_301 * noise_metadata_schedule_592_e6057);
        (noise_metadata_schedule_592_e6058,)
    } else {
        (noise_variable_343,)
    }
};
            noise_variable_343 = noise_metadata_schedule_592_e6060;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_593_e6082,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_576 == 0.0)) && (noise_variable_580 == 0.0)) && (noise_variable_585 != 0.0)) && (noise_variable_586 != 0.0)) {
        let noise_metadata_schedule_593_e6074: f64 = (params.p40 / noise_variable_340);
        let noise_metadata_schedule_593_e6077: f64 = (params.p44 - noise_variable_250);
        let noise_metadata_schedule_593_e6078: f64 = (noise_metadata_schedule_593_e6074 * noise_metadata_schedule_593_e6077);
        let noise_metadata_schedule_593_e6080: f64 = (noise_metadata_schedule_593_e6078 * noise_variable_343);
        (noise_metadata_schedule_593_e6080,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_593_e6082;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_594_e6085: f64 = if noise_variable_210 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_590 = noise_metadata_schedule_594_e6085;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_595_e6088: f64 = if params.p53 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_591 = noise_metadata_schedule_595_e6088;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_596_e6114,) = {
    if (((noise_variable_575 != 0.0) && (noise_variable_590 != 0.0)) && (noise_variable_591 != 0.0)) {
        let noise_metadata_schedule_596_e6098: f64 = (noise_variable_30 + noise_variable_189);
        let noise_metadata_schedule_596_e6099: f64 = (noise_variable_159 * noise_metadata_schedule_596_e6098);
        let noise_metadata_schedule_596_e6100: f64 = (noise_variable_6 / noise_metadata_schedule_596_e6099);
        let noise_metadata_schedule_596_e6103: f64 = (noise_variable_156 / noise_variable_35);
        let noise_metadata_schedule_596_e6105: f64 = (noise_metadata_schedule_596_e6103 * noise_variable_42);
        let noise_metadata_schedule_596_e6106: f64 = (noise_metadata_schedule_596_e6100 + noise_metadata_schedule_596_e6105);
        let noise_metadata_schedule_596_e6110: f64 = (noise_variable_30 + noise_variable_189);
        let noise_metadata_schedule_596_e6111: f64 = (noise_variable_28 / noise_metadata_schedule_596_e6110);
        let noise_metadata_schedule_596_e6112: f64 = (noise_metadata_schedule_596_e6106 + noise_metadata_schedule_596_e6111);
        (noise_metadata_schedule_596_e6112,)
    } else {
        (noise_variable_211,)
    }
};
            noise_variable_211 = noise_metadata_schedule_596_e6114;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_597_e6117: f64 = if params.p39 == 3.0 { 1.0 } else { 0.0 };
            noise_variable_592 = noise_metadata_schedule_597_e6117;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_598_e6131,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_590 != 0.0)) && (noise_variable_591 != 0.0)) && (noise_variable_592 != 0.0)) {
        let noise_metadata_schedule_598_e6127: f64 = (noise_variable_210 - noise_variable_211);
        let noise_metadata_schedule_598_e6129: f64 = (noise_metadata_schedule_598_e6127 / 1e-6);
        (noise_metadata_schedule_598_e6129,)
    } else {
        (noise_variable_285,)
    }
};
            noise_variable_285 = noise_metadata_schedule_598_e6131;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_599_e6134: f64 = if noise_variable_210 < noise_variable_211 { 1.0 } else { 0.0 };
            noise_variable_593 = noise_metadata_schedule_599_e6134;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_600_e6154,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_590 != 0.0)) && (noise_variable_591 != 0.0)) && (noise_variable_592 != 0.0)) && (noise_variable_593 != 0.0)) {
        let noise_metadata_schedule_600_e6148: f64 = (noise_variable_285).exp();
        let noise_metadata_schedule_600_e6149: f64 = (1.0 + noise_metadata_schedule_600_e6148);
        let noise_metadata_schedule_600_e6150: f64 = (noise_metadata_schedule_600_e6149).ln();
        let noise_metadata_schedule_600_e6151: f64 = (1e-6 * noise_metadata_schedule_600_e6150);
        let noise_metadata_schedule_600_e6152: f64 = (noise_variable_210 - noise_metadata_schedule_600_e6151);
        (noise_metadata_schedule_600_e6152,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_600_e6154;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_601_e6176,) = {
    if (((((noise_variable_575 != 0.0) && (noise_variable_590 != 0.0)) && (noise_variable_591 != 0.0)) && (noise_variable_592 != 0.0)) && (noise_variable_593 == 0.0)) {
        let noise_metadata_schedule_601_e6169: f64 = (-noise_variable_285);
        let noise_metadata_schedule_601_e6170: f64 = (noise_metadata_schedule_601_e6169).exp();
        let noise_metadata_schedule_601_e6171: f64 = (1.0 + noise_metadata_schedule_601_e6170);
        let noise_metadata_schedule_601_e6172: f64 = (noise_metadata_schedule_601_e6171).ln();
        let noise_metadata_schedule_601_e6173: f64 = (1e-6 * noise_metadata_schedule_601_e6172);
        let noise_metadata_schedule_601_e6174: f64 = (noise_variable_211 - noise_metadata_schedule_601_e6173);
        (noise_metadata_schedule_601_e6174,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_601_e6176;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_602_e6188,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_590 != 0.0)) && (noise_variable_591 != 0.0)) && (noise_variable_592 != 0.0)) {
        let noise_metadata_schedule_602_e6186: f64 = (noise_variable_159 * noise_variable_210);
        (noise_metadata_schedule_602_e6186,)
    } else {
        (noise_variable_212,)
    }
};
            noise_variable_212 = noise_metadata_schedule_602_e6188;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_603_e6207,) = {
    if ((((noise_variable_575 != 0.0) && (noise_variable_590 != 0.0)) && (noise_variable_591 != 0.0)) && (noise_variable_592 == 0.0)) {
        let noise_metadata_schedule_603_e6199: f64 = (noise_variable_159 * noise_variable_210);
        let noise_metadata_schedule_603_e6201: f64 = (noise_metadata_schedule_603_e6199 * noise_variable_211);
        let noise_metadata_schedule_603_e6204: f64 = (noise_variable_210 + noise_variable_211);
        let noise_metadata_schedule_603_e6205: f64 = (noise_metadata_schedule_603_e6201 / noise_metadata_schedule_603_e6204);
        (noise_metadata_schedule_603_e6205,)
    } else {
        (noise_variable_212,)
    }
};
            noise_variable_212 = noise_metadata_schedule_603_e6207;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_604_e6218,) = {
    if (((noise_variable_575 != 0.0) && (noise_variable_590 != 0.0)) && (noise_variable_591 == 0.0)) {
        let noise_metadata_schedule_604_e6216: f64 = (noise_variable_159 * noise_variable_210);
        (noise_metadata_schedule_604_e6216,)
    } else {
        (noise_variable_212,)
    }
};
            noise_variable_212 = noise_metadata_schedule_604_e6218;
        }
        if matches!(source_index, 3 | 4 | 5 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_702_e7262: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_702_e7264: f64 = (noise_metadata_schedule_702_e7262 * noise_variable_2);
            noise_variable_308 = noise_metadata_schedule_702_e7264;
        }
        if matches!(source_index, 3) {
            let noise_metadata_schedule_703_e7267: f64 = (noise_variable_308 / noise_variable_28);
            noise_variable_309 = noise_metadata_schedule_703_e7267;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_704_e7270: f64 = (noise_variable_308 / noise_variable_30);
            noise_variable_310 = noise_metadata_schedule_704_e7270;
        }
        if matches!(source_index, 20 | 23 | 25 | 27) {
            let noise_metadata_schedule_705_e7273: f64 = (noise_variable_308 * noise_variable_111);
            noise_variable_311 = noise_metadata_schedule_705_e7273;
        }
        if matches!(source_index, 21 | 24) {
            let noise_metadata_schedule_706_e7276: f64 = (noise_variable_308 * noise_variable_112);
            noise_variable_312 = noise_metadata_schedule_706_e7276;
        }
        if matches!(source_index, 22 | 26) {
            let noise_metadata_schedule_707_e7279: f64 = (noise_variable_308 * noise_variable_113);
            noise_variable_313 = noise_metadata_schedule_707_e7279;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_708_e7282: f64 = (noise_variable_308 / noise_variable_189);
            let noise_metadata_schedule_708_e7285: f64 = (4.0 * noise_variable_273);
            let noise_metadata_schedule_708_e7287: f64 = (noise_metadata_schedule_708_e7285 + 5.0);
            let noise_metadata_schedule_708_e7288: f64 = (noise_metadata_schedule_708_e7282 * noise_metadata_schedule_708_e7287);
            let noise_metadata_schedule_708_e7290: f64 = (noise_metadata_schedule_708_e7288 * 0.3333333333333333);
            noise_variable_314 = noise_metadata_schedule_708_e7290;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_709_e7293: f64 = (noise_variable_158 + noise_variable_157);
            let noise_metadata_schedule_709_e7295: f64 = (noise_metadata_schedule_709_e7293 / noise_variable_156);
            noise_variable_333 = noise_metadata_schedule_709_e7295;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_710_e7298: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_710_e7300: f64 = (noise_variable_333).abs();
            let noise_metadata_schedule_710_e7301: f64 = (noise_metadata_schedule_710_e7298 * noise_metadata_schedule_710_e7300);
            noise_variable_315 = noise_metadata_schedule_710_e7301;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_711_e7304: f64 = if params.p130 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_614 = noise_metadata_schedule_711_e7304;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_712_e7311,) = {
    if (noise_variable_614 != 0.0) {
        let noise_metadata_schedule_712_e7308: f64 = (noise_variable_212 / noise_variable_333);
        let noise_metadata_schedule_712_e7309: f64 = (noise_metadata_schedule_712_e7308).abs();
        (noise_metadata_schedule_712_e7309,)
    } else {
        (noise_variable_334,)
    }
};
            noise_variable_334 = noise_metadata_schedule_712_e7311;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_713_e7316,) = {
    if (noise_variable_614 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_334,)
    }
};
            noise_variable_334 = noise_metadata_schedule_713_e7316;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_714_e7319: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_714_e7321: f64 = (noise_metadata_schedule_714_e7319 * noise_variable_212);
            let noise_metadata_schedule_714_e7324: f64 = (noise_variable_334 + 1.0);
            let noise_metadata_schedule_714_e7325: f64 = (noise_metadata_schedule_714_e7321 * noise_metadata_schedule_714_e7324);
            noise_variable_327 = noise_metadata_schedule_714_e7325;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_723_e7377: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_723_e7380: f64 = (noise_variable_161 + noise_variable_163);
            let noise_metadata_schedule_723_e7382: f64 = (noise_metadata_schedule_723_e7380 - noise_variable_57);
            let noise_metadata_schedule_723_e7384: f64 = (noise_metadata_schedule_723_e7382 + noise_variable_359);
            let noise_metadata_schedule_723_e7386: f64 = (noise_metadata_schedule_723_e7384 + noise_variable_358);
            let noise_metadata_schedule_723_e7387: f64 = (noise_metadata_schedule_723_e7386).abs();
            let noise_metadata_schedule_723_e7388: f64 = (noise_metadata_schedule_723_e7377 * noise_metadata_schedule_723_e7387);
            noise_variable_316 = noise_metadata_schedule_723_e7388;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_724_e7391: f64 = (noise_variable_161 + noise_variable_162);
            noise_variable_328 = noise_metadata_schedule_724_e7391;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_725_e7394: f64 = (noise_variable_328).abs();
            let noise_metadata_schedule_725_e7396: f64 = (noise_metadata_schedule_725_e7394).powf(params.p126);
            let noise_metadata_schedule_725_e7397: f64 = (params.p128 * noise_metadata_schedule_725_e7396);
            noise_variable_317 = noise_metadata_schedule_725_e7397;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_726_e7400: f64 = if noise_variable_328 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_618 = noise_metadata_schedule_726_e7400;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_727_e7405,) = {
    if (noise_variable_618 != 0.0) {
        let noise_metadata_schedule_727_e7403: f64 = (-noise_variable_317);
        (noise_metadata_schedule_727_e7403,)
    } else {
        (noise_variable_317,)
    }
};
            noise_variable_317 = noise_metadata_schedule_727_e7405;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_728_e7408: f64 = (noise_variable_163 + noise_variable_165);
            let noise_metadata_schedule_728_e7410: f64 = (noise_metadata_schedule_728_e7408 + noise_variable_166);
            noise_variable_329 = noise_metadata_schedule_728_e7410;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_729_e7413: f64 = (noise_variable_329).abs();
            let noise_metadata_schedule_729_e7415: f64 = (noise_metadata_schedule_729_e7413).powf(params.p127);
            let noise_metadata_schedule_729_e7416: f64 = (params.p129 * noise_metadata_schedule_729_e7415);
            noise_variable_318 = noise_metadata_schedule_729_e7416;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_730_e7419: f64 = if noise_variable_329 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_619 = noise_metadata_schedule_730_e7419;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_731_e7424,) = {
    if (noise_variable_619 != 0.0) {
        let noise_metadata_schedule_731_e7422: f64 = (-noise_variable_318);
        (noise_metadata_schedule_731_e7422,)
    } else {
        (noise_variable_318,)
    }
};
            noise_variable_318 = noise_metadata_schedule_731_e7424;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_732_e7427: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_732_e7430: f64 = (noise_variable_162 + noise_variable_165);
            let noise_metadata_schedule_732_e7432: f64 = (noise_metadata_schedule_732_e7430 + noise_variable_166);
            let noise_metadata_schedule_732_e7433: f64 = (noise_metadata_schedule_732_e7432).abs();
            let noise_metadata_schedule_732_e7434: f64 = (noise_metadata_schedule_732_e7427 * noise_metadata_schedule_732_e7433);
            noise_variable_319 = noise_metadata_schedule_732_e7434;
        }
        if matches!(source_index, 9) {
            let noise_metadata_schedule_733_e7437: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_733_e7439: f64 = (noise_variable_164).abs();
            let noise_metadata_schedule_733_e7440: f64 = (noise_metadata_schedule_733_e7437 * noise_metadata_schedule_733_e7439);
            noise_variable_320 = noise_metadata_schedule_733_e7440;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_734_e7443: f64 = (noise_variable_164).abs();
            let noise_metadata_schedule_734_e7445: f64 = (noise_metadata_schedule_734_e7443).powf(params.p126);
            let noise_metadata_schedule_734_e7446: f64 = (params.p128 * noise_metadata_schedule_734_e7445);
            noise_variable_321 = noise_metadata_schedule_734_e7446;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_735_e7449: f64 = if noise_variable_164 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_620 = noise_metadata_schedule_735_e7449;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_736_e7454,) = {
    if (noise_variable_620 != 0.0) {
        let noise_metadata_schedule_736_e7452: f64 = (-noise_variable_321);
        (noise_metadata_schedule_736_e7452,)
    } else {
        (noise_variable_321,)
    }
};
            noise_variable_321 = noise_metadata_schedule_736_e7454;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_737_e7457: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_737_e7459: f64 = (noise_variable_82).abs();
            let noise_metadata_schedule_737_e7460: f64 = (noise_metadata_schedule_737_e7457 * noise_metadata_schedule_737_e7459);
            noise_variable_322 = noise_metadata_schedule_737_e7460;
        }
        if matches!(source_index, 11) {
            let noise_metadata_schedule_738_e7463: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_738_e7465: f64 = (noise_variable_167).abs();
            let noise_metadata_schedule_738_e7466: f64 = (noise_metadata_schedule_738_e7463 * noise_metadata_schedule_738_e7465);
            noise_variable_323 = noise_metadata_schedule_738_e7466;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_739_e7471: f64 = (params.p5 * params.p33);
            let noise_metadata_schedule_739_e7472: f64 = (1.0 - noise_metadata_schedule_739_e7471);
            let noise_metadata_schedule_739_e7473: f64 = (params.p128 * noise_metadata_schedule_739_e7472);
            let noise_metadata_schedule_739_e7475: f64 = (noise_variable_167).abs();
            let noise_metadata_schedule_739_e7479: f64 = (params.p5 * params.p33);
            let noise_metadata_schedule_739_e7480: f64 = (1.0 - noise_metadata_schedule_739_e7479);
            let noise_metadata_schedule_739_e7481: f64 = (noise_metadata_schedule_739_e7475 / noise_metadata_schedule_739_e7480);
            let noise_metadata_schedule_739_e7483: f64 = (noise_metadata_schedule_739_e7481).powf(params.p126);
            let noise_metadata_schedule_739_e7484: f64 = (noise_metadata_schedule_739_e7473 * noise_metadata_schedule_739_e7483);
            noise_variable_325 = noise_metadata_schedule_739_e7484;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_740_e7487: f64 = if noise_variable_167 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_621 = noise_metadata_schedule_740_e7487;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_741_e7492,) = {
    if (noise_variable_621 != 0.0) {
        let noise_metadata_schedule_741_e7490: f64 = (-noise_variable_325);
        (noise_metadata_schedule_741_e7490,)
    } else {
        (noise_variable_325,)
    }
};
            noise_variable_325 = noise_metadata_schedule_741_e7492;
        }
        if matches!(source_index, 13) {
            let noise_metadata_schedule_742_e7495: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_742_e7497: f64 = (noise_variable_179).abs();
            let noise_metadata_schedule_742_e7498: f64 = (noise_metadata_schedule_742_e7495 * noise_metadata_schedule_742_e7497);
            let noise_metadata_schedule_742_e7500: f64 = (noise_metadata_schedule_742_e7498 * params.p5);
            noise_variable_324 = noise_metadata_schedule_742_e7500;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_743_e7503: f64 = if params.p33 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_622 = noise_metadata_schedule_743_e7503;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_744_e7507,) = {
    if (noise_variable_622 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_326,)
    }
};
            noise_variable_326 = noise_metadata_schedule_744_e7507;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_745_e7523,) = {
    if (noise_variable_622 == 0.0) {
        let noise_metadata_schedule_745_e7512: f64 = (params.p128 * params.p5);
        let noise_metadata_schedule_745_e7514: f64 = (noise_metadata_schedule_745_e7512 * params.p33);
        let noise_metadata_schedule_745_e7516: f64 = (noise_variable_179).abs();
        let noise_metadata_schedule_745_e7518: f64 = (noise_metadata_schedule_745_e7516 / params.p33);
        let noise_metadata_schedule_745_e7520: f64 = (noise_metadata_schedule_745_e7518).powf(params.p126);
        let noise_metadata_schedule_745_e7521: f64 = (noise_metadata_schedule_745_e7514 * noise_metadata_schedule_745_e7520);
        (noise_metadata_schedule_745_e7521,)
    } else {
        (noise_variable_326,)
    }
};
            noise_variable_326 = noise_metadata_schedule_745_e7523;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_746_e7526: f64 = if noise_variable_179 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_623 = noise_metadata_schedule_746_e7526;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_747_e7531,) = {
    if (noise_variable_623 != 0.0) {
        let noise_metadata_schedule_747_e7529: f64 = (-noise_variable_326);
        (noise_metadata_schedule_747_e7529,)
    } else {
        (noise_variable_326,)
    }
};
            noise_variable_326 = noise_metadata_schedule_747_e7531;
        }
        if matches!(source_index, 17) {
            let noise_metadata_schedule_748_e7534: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_748_e7536: f64 = (noise_variable_185).abs();
            let noise_metadata_schedule_748_e7537: f64 = (noise_metadata_schedule_748_e7534 * noise_metadata_schedule_748_e7536);
            noise_variable_330 = noise_metadata_schedule_748_e7537;
        }
        if matches!(source_index, 18) {
            let noise_metadata_schedule_749_e7540: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_749_e7542: f64 = (noise_variable_182).abs();
            let noise_metadata_schedule_749_e7543: f64 = (noise_metadata_schedule_749_e7540 * noise_metadata_schedule_749_e7542);
            noise_variable_331 = noise_metadata_schedule_749_e7543;
        }
        if matches!(source_index, 19) {
            let noise_metadata_schedule_750_e7546: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_750_e7548: f64 = (noise_variable_183).abs();
            let noise_metadata_schedule_750_e7549: f64 = (noise_metadata_schedule_750_e7546 * noise_metadata_schedule_750_e7548);
            noise_variable_332 = noise_metadata_schedule_750_e7549;
        }
        match source_index {
            0 => {
                let noise_0_psd_e8670: f64 = 1.0;
                let noise_0_psd_e400: f64 = (noise_variable_315 * params.p1);
                let noise_0_psd_e8671: f64 = (noise_0_psd_e8670 * noise_0_psd_e400);
                let psd = noise_0_psd_e8671;
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
                let noise_1_psd_e8673: f64 = 1.0;
                let noise_1_psd_e414: f64 = (noise_variable_327 * params.p1);
                let noise_1_psd_e8674: f64 = (noise_1_psd_e8673 * noise_1_psd_e414);
                let psd = noise_1_psd_e8674;
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
                let noise_2_psd_e8676: f64 = 1.0;
                let noise_2_psd_e419: f64 = (noise_variable_316 * params.p1);
                let noise_2_psd_e8677: f64 = (noise_2_psd_e8676 * noise_2_psd_e419);
                let psd = noise_2_psd_e8677;
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
                let noise_3_psd_e8679: f64 = 1.0;
                let noise_3_psd_e424: f64 = (noise_variable_309 * params.p1);
                let noise_3_psd_e8680: f64 = (noise_3_psd_e8679 * noise_3_psd_e424);
                let psd = noise_3_psd_e8680;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            4 => {
                let noise_4_psd_e8682: f64 = 1.0;
                let noise_4_psd_e429: f64 = (noise_variable_310 * params.p1);
                let noise_4_psd_e8683: f64 = (noise_4_psd_e8682 * noise_4_psd_e429);
                let psd = noise_4_psd_e8683;
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
                let noise_5_psd_e8685: f64 = 1.0;
                let noise_5_psd_e434: f64 = (noise_variable_314 * params.p1);
                let noise_5_psd_e8686: f64 = (noise_5_psd_e8685 * noise_5_psd_e434);
                let psd = noise_5_psd_e8686;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            6 => {
                let noise_6_psd_e8688: f64 = 1.0;
                let noise_6_psd_e439: f64 = (noise_variable_317 * params.p1);
                let noise_6_psd_e8689: f64 = (noise_6_psd_e8688 * noise_6_psd_e439);
                let psd = noise_6_psd_e8689;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
                let exponent: Option<f64> = Some(1.0);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            7 => {
                let noise_7_psd_e8691: f64 = 1.0;
                let noise_7_psd_e445: f64 = (noise_variable_318 * params.p1);
                let noise_7_psd_e8692: f64 = (noise_7_psd_e8691 * noise_7_psd_e445);
                let psd = noise_7_psd_e8692;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 7, value: psd }); }
                let exponent: Option<f64> = Some(1.0);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            8 => {
                let noise_8_psd_e8694: f64 = 1.0;
                let noise_8_psd_e451: f64 = (noise_variable_319 * params.p1);
                let noise_8_psd_e8695: f64 = (noise_8_psd_e8694 * noise_8_psd_e451);
                let psd = noise_8_psd_e8695;
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
                let noise_9_psd_e8697: f64 = 1.0;
                let noise_9_psd_e456: f64 = (noise_variable_320 * params.p1);
                let noise_9_psd_e8698: f64 = (noise_9_psd_e8697 * noise_9_psd_e456);
                let psd = noise_9_psd_e8698;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            10 => {
                let noise_10_psd_e8700: f64 = 1.0;
                let noise_10_psd_e461: f64 = (noise_variable_321 * params.p1);
                let noise_10_psd_e8701: f64 = (noise_10_psd_e8700 * noise_10_psd_e461);
                let psd = noise_10_psd_e8701;
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
                let noise_11_psd_e8703: f64 = 1.0;
                let noise_11_psd_e467: f64 = (noise_variable_323 * params.p1);
                let noise_11_psd_e8704: f64 = (noise_11_psd_e8703 * noise_11_psd_e467);
                let psd = noise_11_psd_e8704;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            12 => {
                let noise_12_psd_e8706: f64 = 1.0;
                let noise_12_psd_e472: f64 = (noise_variable_325 * params.p1);
                let noise_12_psd_e8707: f64 = (noise_12_psd_e8706 * noise_12_psd_e472);
                let psd = noise_12_psd_e8707;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 12, value: psd }); }
                let exponent: Option<f64> = Some(1.0);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            13 => {
                let noise_13_psd_e8709: f64 = 1.0;
                let noise_13_psd_e478: f64 = (noise_variable_324 * params.p1);
                let noise_13_psd_e8710: f64 = (noise_13_psd_e8709 * noise_13_psd_e478);
                let psd = noise_13_psd_e8710;
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
                let noise_14_psd_e8712: f64 = 1.0;
                let noise_14_psd_e483: f64 = (noise_variable_326 * params.p1);
                let noise_14_psd_e8713: f64 = (noise_14_psd_e8712 * noise_14_psd_e483);
                let psd = noise_14_psd_e8713;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
                let exponent: Option<f64> = Some(1.0);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            15 => {
                let noise_15_psd_e8715: f64 = 1.0;
                let noise_15_psd_e490: f64 = (noise_variable_322 * params.p1);
                let noise_15_psd_e8716: f64 = (noise_15_psd_e8715 * noise_15_psd_e490);
                let psd = noise_15_psd_e8716;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            16 => {
                let noise_16_psd_e8718: f64 = 1.0;
                let noise_16_psd_e499: f64 = (noise_variable_322 * params.p1);
                let noise_16_psd_e8719: f64 = (noise_16_psd_e8718 * noise_16_psd_e499);
                let psd = noise_16_psd_e8719;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 16, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            17 => {
                let noise_17_psd_e8721: f64 = 1.0;
                let noise_17_psd_e506: f64 = (noise_variable_330 * params.p1);
                let noise_17_psd_e8722: f64 = (noise_17_psd_e8721 * noise_17_psd_e506);
                let psd = noise_17_psd_e8722;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 17, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            18 => {
                let noise_18_psd_e8724: f64 = 1.0;
                let noise_18_psd_e511: f64 = (noise_variable_331 * params.p1);
                let noise_18_psd_e8725: f64 = (noise_18_psd_e8724 * noise_18_psd_e511);
                let psd = noise_18_psd_e8725;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 18, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            19 => {
                let noise_19_psd_e8727: f64 = 1.0;
                let noise_19_psd_e516: f64 = (noise_variable_332 * params.p1);
                let noise_19_psd_e8728: f64 = (noise_19_psd_e8727 * noise_19_psd_e516);
                let psd = noise_19_psd_e8728;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 19, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            20 => {
                let noise_20_psd_e8730: f64 = 1.0;
                let noise_20_psd_e524: f64 = (noise_variable_311 * params.p1);
                let noise_20_psd_e8731: f64 = (noise_20_psd_e8730 * noise_20_psd_e524);
                let psd = noise_20_psd_e8731;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 20, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            21 => {
                let noise_21_psd_e8733: f64 = 1.0;
                let noise_21_psd_e534: f64 = (noise_variable_312 * params.p1);
                let noise_21_psd_e8734: f64 = (noise_21_psd_e8733 * noise_21_psd_e534);
                let psd = noise_21_psd_e8734;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 21, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            22 => {
                let noise_22_psd_e8736: f64 = 1.0;
                let noise_22_psd_e544: f64 = (noise_variable_313 * params.p1);
                let noise_22_psd_e8737: f64 = (noise_22_psd_e8736 * noise_22_psd_e544);
                let psd = noise_22_psd_e8737;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 22, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            23 => {
                let noise_23_psd_e8739: f64 = 1.0;
                let noise_23_psd_e555: f64 = (noise_variable_311 * params.p1);
                let noise_23_psd_e8740: f64 = (noise_23_psd_e8739 * noise_23_psd_e555);
                let psd = noise_23_psd_e8740;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 23, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            24 => {
                let noise_24_psd_e8742: f64 = 1.0;
                let noise_24_psd_e566: f64 = (noise_variable_312 * params.p1);
                let noise_24_psd_e8743: f64 = (noise_24_psd_e8742 * noise_24_psd_e566);
                let psd = noise_24_psd_e8743;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 24, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            25 => {
                let noise_25_psd_e8745: f64 = 1.0;
                let noise_25_psd_e577: f64 = (noise_variable_311 * params.p1);
                let noise_25_psd_e8746: f64 = (noise_25_psd_e8745 * noise_25_psd_e577);
                let psd = noise_25_psd_e8746;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 25, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            26 => {
                let noise_26_psd_e8748: f64 = 1.0;
                let noise_26_psd_e588: f64 = (noise_variable_313 * params.p1);
                let noise_26_psd_e8749: f64 = (noise_26_psd_e8748 * noise_26_psd_e588);
                let psd = noise_26_psd_e8749;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 26, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            27 => {
                let noise_27_psd_e8751: f64 = 1.0;
                let noise_27_psd_e600: f64 = (noise_variable_311 * params.p1);
                let noise_27_psd_e8752: f64 = (noise_27_psd_e8751 * noise_27_psd_e600);
                let psd = noise_27_psd_e8752;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 27, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 27, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 27, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 27, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            _ => unreachable!("noise source index was range checked"),
        }
    }
}

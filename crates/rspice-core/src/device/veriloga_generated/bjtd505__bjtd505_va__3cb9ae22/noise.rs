#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 25] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_NOI_GND_IN", label: Some("in"), kind: GeneratedNoiseKind::White, equation: 25, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "noi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_E1_IB2E1", label: Some("ib2e1"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_E1_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_B1_RBC", label: Some("rbc"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_B2_RBV", label: Some("rbv"), kind: GeneratedNoiseKind::White, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B2_E1_IB2E1_F", label: Some("ib2e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_E1_IB1E1_F", label: Some("ib1e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_E1_IB1E1", label: Some("ib1e1"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IB3", label: Some("ib3"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IB3_F", label: Some("ib3_f"), kind: GeneratedNoiseKind::Flicker, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IEX", label: Some("iex"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IEX_F", label: Some("iex_f"), kind: GeneratedNoiseKind::Flicker, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_C3_XIEX", label: Some("xiex"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B_C3_XIEX_F", label: Some("xiex_f"), kind: GeneratedNoiseKind::Flicker, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C1_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C4_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C1_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C4_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C1_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
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
        if matches!(source_index, 15 | 16) {
            let noise_activation_schedule_683_e6711: f64 = if params.p23 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_565 = noise_activation_schedule_683_e6711;
        }
        if matches!(source_index, 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_activation_schedule_684_e6714: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_566 = noise_activation_schedule_684_e6714;
        }
        if matches!(source_index, 17 | 18 | 19 | 20 | 21) {
            let noise_activation_schedule_685_e6717: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_567 = noise_activation_schedule_685_e6717;
        }
        if matches!(source_index, 22 | 23 | 24) {
            let noise_activation_schedule_686_e6720: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_568 = noise_activation_schedule_686_e6720;
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
                noise_variable_565 != 0.0
            }
            16 => {
                let noise_16_activation_e445: f64 = if (noise_variable_565 == 0.0) { 1.0 } else { 0.0 };
                noise_16_activation_e445 != 0.0
            }
            17 => {
                let noise_17_activation_e455: f64 = if ((noise_variable_566 != 0.0) && (noise_variable_567 != 0.0)) { 1.0 } else { 0.0 };
                noise_17_activation_e455 != 0.0
            }
            18 => {
                let noise_18_activation_e465: f64 = if ((noise_variable_566 != 0.0) && (noise_variable_567 != 0.0)) { 1.0 } else { 0.0 };
                noise_18_activation_e465 != 0.0
            }
            19 => {
                let noise_19_activation_e475: f64 = if ((noise_variable_566 != 0.0) && (noise_variable_567 != 0.0)) { 1.0 } else { 0.0 };
                noise_19_activation_e475 != 0.0
            }
            20 => {
                let noise_20_activation_e486: f64 = if ((noise_variable_566 != 0.0) && (noise_variable_567 == 0.0)) { 1.0 } else { 0.0 };
                noise_20_activation_e486 != 0.0
            }
            21 => {
                let noise_21_activation_e497: f64 = if ((noise_variable_566 != 0.0) && (noise_variable_567 == 0.0)) { 1.0 } else { 0.0 };
                noise_21_activation_e497 != 0.0
            }
            22 => {
                let noise_22_activation_e508: f64 = if ((noise_variable_566 == 0.0) && (noise_variable_568 != 0.0)) { 1.0 } else { 0.0 };
                noise_22_activation_e508 != 0.0
            }
            23 => {
                let noise_23_activation_e519: f64 = if ((noise_variable_566 == 0.0) && (noise_variable_568 != 0.0)) { 1.0 } else { 0.0 };
                noise_23_activation_e519 != 0.0
            }
            24 => {
                let noise_24_activation_e531: f64 = if ((noise_variable_566 == 0.0) && (noise_variable_568 == 0.0)) { 1.0 } else { 0.0 };
                noise_24_activation_e531 != 0.0
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
        if matches!(source_index, 1) {
            let noise_metadata_schedule_0_e541: f64 = if params.p3 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_439 = noise_metadata_schedule_0_e541;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_1_e545,) = {
    if (noise_variable_439 != 0.0) {
        (70300000.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_1_e545;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_2_e549,) = {
    if (noise_variable_439 != 0.0) {
        (123000000.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_2_e549;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_3_e554,) = {
    if (noise_variable_439 == 0.0) {
        (158000000.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_3_e554;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_4_e559,) = {
    if (noise_variable_439 == 0.0) {
        (204000000.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_4_e559;
        }
        if matches!(source_index, 11 | 12) {
            let noise_metadata_schedule_5_e562: f64 = (1.0 - params.p32);
            noise_variable_150 = noise_metadata_schedule_5_e562;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_6_e565: f64 = (params.p4 + 273.15);
            noise_variable_3 = noise_metadata_schedule_6_e565;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_7_e566: f64 = ctx.temperature();
            let noise_metadata_schedule_7_e568: f64 = (noise_metadata_schedule_7_e566 + params.p0);
            noise_variable_5 = noise_metadata_schedule_7_e568;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_9_e574: f64 = if params.p137 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_440 = noise_metadata_schedule_9_e574;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let (noise_metadata_schedule_10_e578,) = {
    if (noise_variable_440 != 0.0) {
        (1e-12,)
    } else {
        (noise_variable_315,)
    }
};
            noise_variable_315 = noise_metadata_schedule_10_e578;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let (noise_metadata_schedule_11_e583,) = {
    if (noise_variable_440 == 0.0) {
        (params.p137,)
    } else {
        (noise_variable_315,)
    }
};
            noise_variable_315 = noise_metadata_schedule_11_e583;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_12_e586: f64 = (noise_variable_315 * params.p1);
            noise_variable_316 = noise_metadata_schedule_12_e586;
        }
        if matches!(source_index, 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_13_e589: f64 = (1.0 / noise_variable_316);
            noise_variable_317 = noise_metadata_schedule_13_e589;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            noise_variable_52 = 0.001;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            noise_variable_312 = 0.001;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_16_e595: f64 = (2.0 - params.p66);
            let noise_metadata_schedule_16_e596: f64 = (2.0_f64).powf(noise_metadata_schedule_16_e595);
            noise_variable_62 = noise_metadata_schedule_16_e596;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_17_e599: f64 = (1.0 / noise_variable_62);
            noise_variable_63 = noise_metadata_schedule_17_e599;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_18_e603: f64 = (params.p114 * noise_variable_3);
            let noise_metadata_schedule_18_e605: f64 = (noise_metadata_schedule_18_e603 * noise_variable_3);
            let noise_metadata_schedule_18_e608: f64 = (noise_variable_3 + params.p115);
            let noise_metadata_schedule_18_e609: f64 = (noise_metadata_schedule_18_e605 / noise_metadata_schedule_18_e608);
            let noise_metadata_schedule_18_e610: f64 = (params.p113 + noise_metadata_schedule_18_e609);
            let noise_metadata_schedule_18_e612: f64 = (noise_metadata_schedule_18_e610 - 0.05);
            let noise_metadata_schedule_18_e614: f64 = (noise_metadata_schedule_18_e612 / 0.1);
            noise_variable_259 = noise_metadata_schedule_18_e614;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_19_e618: f64 = (params.p114 * noise_variable_3);
            let noise_metadata_schedule_19_e620: f64 = (noise_metadata_schedule_19_e618 * noise_variable_3);
            let noise_metadata_schedule_19_e623: f64 = (noise_variable_3 + params.p115);
            let noise_metadata_schedule_19_e624: f64 = (noise_metadata_schedule_19_e620 / noise_metadata_schedule_19_e623);
            let noise_metadata_schedule_19_e625: f64 = (params.p113 + noise_metadata_schedule_19_e624);
            let noise_metadata_schedule_19_e627: f64 = if noise_metadata_schedule_19_e625 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_441 = noise_metadata_schedule_19_e627;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_20_e639,) = {
    if (noise_variable_441 != 0.0) {
        let noise_metadata_schedule_20_e633: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_20_e634: f64 = (1.0 + noise_metadata_schedule_20_e633);
        let noise_metadata_schedule_20_e635: f64 = (noise_metadata_schedule_20_e634).ln();
        let noise_metadata_schedule_20_e636: f64 = (0.1 * noise_metadata_schedule_20_e635);
        let noise_metadata_schedule_20_e637: f64 = (0.05 + noise_metadata_schedule_20_e636);
        (noise_metadata_schedule_20_e637,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_20_e639;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_21_e663,) = {
    if (noise_variable_441 == 0.0) {
        let noise_metadata_schedule_21_e645: f64 = (params.p114 * noise_variable_3);
        let noise_metadata_schedule_21_e647: f64 = (noise_metadata_schedule_21_e645 * noise_variable_3);
        let noise_metadata_schedule_21_e650: f64 = (noise_variable_3 + params.p115);
        let noise_metadata_schedule_21_e651: f64 = (noise_metadata_schedule_21_e647 / noise_metadata_schedule_21_e650);
        let noise_metadata_schedule_21_e652: f64 = (params.p113 + noise_metadata_schedule_21_e651);
        let noise_metadata_schedule_21_e656: f64 = (-noise_variable_259);
        let noise_metadata_schedule_21_e657: f64 = (noise_metadata_schedule_21_e656).exp();
        let noise_metadata_schedule_21_e658: f64 = (1.0 + noise_metadata_schedule_21_e657);
        let noise_metadata_schedule_21_e659: f64 = (noise_metadata_schedule_21_e658).ln();
        let noise_metadata_schedule_21_e660: f64 = (0.1 * noise_metadata_schedule_21_e659);
        let noise_metadata_schedule_21_e661: f64 = (noise_metadata_schedule_21_e652 + noise_metadata_schedule_21_e660);
        (noise_metadata_schedule_21_e661,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_21_e663;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            noise_variable_71 = params.p113;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_23_e667: f64 = (1.0 / noise_variable_71);
            noise_variable_72 = noise_metadata_schedule_23_e667;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_24_e670: f64 = (1.0 / params.p65);
            noise_variable_64 = noise_metadata_schedule_24_e670;
        }
        if matches!(source_index, 1 | 15 | 16) {
            noise_variable_75 = params.p70;
        }
        if matches!(source_index, 1 | 15 | 16) {
            noise_variable_76 = params.p71;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_27_e676: f64 = (2.0 - noise_variable_76);
            let noise_metadata_schedule_27_e677: f64 = (2.0_f64).powf(noise_metadata_schedule_27_e676);
            noise_variable_79 = noise_metadata_schedule_27_e677;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_28_e680: f64 = (1.0 / noise_variable_79);
            noise_variable_89 = noise_metadata_schedule_28_e680;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_29_e684: f64 = (params.p117 * noise_variable_3);
            let noise_metadata_schedule_29_e686: f64 = (noise_metadata_schedule_29_e684 * noise_variable_3);
            let noise_metadata_schedule_29_e689: f64 = (noise_variable_3 + params.p118);
            let noise_metadata_schedule_29_e690: f64 = (noise_metadata_schedule_29_e686 / noise_metadata_schedule_29_e689);
            let noise_metadata_schedule_29_e691: f64 = (params.p116 + noise_metadata_schedule_29_e690);
            let noise_metadata_schedule_29_e693: f64 = (noise_metadata_schedule_29_e691 - 0.05);
            let noise_metadata_schedule_29_e695: f64 = (noise_metadata_schedule_29_e693 / 0.1);
            noise_variable_259 = noise_metadata_schedule_29_e695;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_30_e699: f64 = (params.p117 * noise_variable_3);
            let noise_metadata_schedule_30_e701: f64 = (noise_metadata_schedule_30_e699 * noise_variable_3);
            let noise_metadata_schedule_30_e704: f64 = (noise_variable_3 + params.p118);
            let noise_metadata_schedule_30_e705: f64 = (noise_metadata_schedule_30_e701 / noise_metadata_schedule_30_e704);
            let noise_metadata_schedule_30_e706: f64 = (params.p116 + noise_metadata_schedule_30_e705);
            let noise_metadata_schedule_30_e708: f64 = if noise_metadata_schedule_30_e706 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_442 = noise_metadata_schedule_30_e708;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_31_e720,) = {
    if (noise_variable_442 != 0.0) {
        let noise_metadata_schedule_31_e714: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_31_e715: f64 = (1.0 + noise_metadata_schedule_31_e714);
        let noise_metadata_schedule_31_e716: f64 = (noise_metadata_schedule_31_e715).ln();
        let noise_metadata_schedule_31_e717: f64 = (0.1 * noise_metadata_schedule_31_e716);
        let noise_metadata_schedule_31_e718: f64 = (0.05 + noise_metadata_schedule_31_e717);
        (noise_metadata_schedule_31_e718,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_31_e720;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_32_e744,) = {
    if (noise_variable_442 == 0.0) {
        let noise_metadata_schedule_32_e726: f64 = (params.p117 * noise_variable_3);
        let noise_metadata_schedule_32_e728: f64 = (noise_metadata_schedule_32_e726 * noise_variable_3);
        let noise_metadata_schedule_32_e731: f64 = (noise_variable_3 + params.p118);
        let noise_metadata_schedule_32_e732: f64 = (noise_metadata_schedule_32_e728 / noise_metadata_schedule_32_e731);
        let noise_metadata_schedule_32_e733: f64 = (params.p116 + noise_metadata_schedule_32_e732);
        let noise_metadata_schedule_32_e737: f64 = (-noise_variable_259);
        let noise_metadata_schedule_32_e738: f64 = (noise_metadata_schedule_32_e737).exp();
        let noise_metadata_schedule_32_e739: f64 = (1.0 + noise_metadata_schedule_32_e738);
        let noise_metadata_schedule_32_e740: f64 = (noise_metadata_schedule_32_e739).ln();
        let noise_metadata_schedule_32_e741: f64 = (0.1 * noise_metadata_schedule_32_e740);
        let noise_metadata_schedule_32_e742: f64 = (noise_metadata_schedule_32_e733 + noise_metadata_schedule_32_e741);
        (noise_metadata_schedule_32_e742,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_32_e744;
        }
        if matches!(source_index, 1 | 15 | 16) {
            noise_variable_87 = params.p116;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_34_e748: f64 = (1.0 / noise_variable_87);
            noise_variable_86 = noise_metadata_schedule_34_e748;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_35_e751: f64 = (1.0 / noise_variable_75);
            noise_variable_66 = noise_metadata_schedule_35_e751;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_36_e755: f64 = (1.0 / params.p82);
            let noise_metadata_schedule_36_e756: f64 = (1.0 - noise_metadata_schedule_36_e755);
            noise_variable_318 = noise_metadata_schedule_36_e756;
        }
        if matches!(source_index, 2 | 6) {
            noise_variable_151 = 0.0;
        }
        if matches!(source_index, 6 | 8) {
            noise_variable_152 = 0.0;
        }
        if matches!(source_index, 13 | 14) {
            noise_variable_169 = 0.0;
        }
        if matches!(source_index, 13 | 14) {
            noise_variable_168 = 1.0;
        }
        if matches!(source_index, 1) {
            noise_variable_196 = 0.0;
        }
        if matches!(source_index, 1) {
            noise_variable_198 = 0.0;
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
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            noise_variable_11 = 0.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_51_e773: f64 = (noise_variable_5 + noise_variable_11);
            noise_variable_2 = noise_metadata_schedule_51_e773;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_52_e776: f64 = (noise_variable_2 / noise_variable_3);
            noise_variable_4 = noise_metadata_schedule_52_e776;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_53_e779: f64 = (8.617086918058125e-5 * noise_variable_2);
            noise_variable_6 = noise_metadata_schedule_53_e779;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_54_e782: f64 = (8.617086918058125e-5 * noise_variable_3);
            noise_variable_7 = noise_metadata_schedule_54_e782;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_55_e785: f64 = (1.0 / noise_variable_6);
            noise_variable_8 = noise_metadata_schedule_55_e785;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_56_e788: f64 = (1.0 / noise_variable_7);
            noise_variable_9 = noise_metadata_schedule_56_e788;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_57_e791: f64 = (noise_variable_8 - noise_variable_9);
            noise_variable_10 = noise_metadata_schedule_57_e791;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_58_e794: f64 = (noise_variable_2 - noise_variable_3);
            noise_variable_12 = noise_metadata_schedule_58_e794;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_59_e796: f64 = (noise_variable_4).ln();
            noise_variable_254 = noise_metadata_schedule_59_e796;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_60_e800: f64 = (params.p114 * noise_variable_2);
            let noise_metadata_schedule_60_e802: f64 = (noise_metadata_schedule_60_e800 * noise_variable_2);
            let noise_metadata_schedule_60_e805: f64 = (noise_variable_2 + params.p115);
            let noise_metadata_schedule_60_e806: f64 = (noise_metadata_schedule_60_e802 / noise_metadata_schedule_60_e805);
            let noise_metadata_schedule_60_e807: f64 = (noise_variable_74 - noise_metadata_schedule_60_e806);
            let noise_metadata_schedule_60_e809: f64 = (noise_metadata_schedule_60_e807 - 0.05);
            let noise_metadata_schedule_60_e811: f64 = (noise_metadata_schedule_60_e809 / 0.1);
            noise_variable_259 = noise_metadata_schedule_60_e811;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_61_e815: f64 = (params.p114 * noise_variable_2);
            let noise_metadata_schedule_61_e817: f64 = (noise_metadata_schedule_61_e815 * noise_variable_2);
            let noise_metadata_schedule_61_e820: f64 = (noise_variable_2 + params.p115);
            let noise_metadata_schedule_61_e821: f64 = (noise_metadata_schedule_61_e817 / noise_metadata_schedule_61_e820);
            let noise_metadata_schedule_61_e822: f64 = (noise_variable_74 - noise_metadata_schedule_61_e821);
            let noise_metadata_schedule_61_e824: f64 = if noise_metadata_schedule_61_e822 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_443 = noise_metadata_schedule_61_e824;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_62_e836,) = {
    if (noise_variable_443 != 0.0) {
        let noise_metadata_schedule_62_e830: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_62_e831: f64 = (1.0 + noise_metadata_schedule_62_e830);
        let noise_metadata_schedule_62_e832: f64 = (noise_metadata_schedule_62_e831).ln();
        let noise_metadata_schedule_62_e833: f64 = (0.1 * noise_metadata_schedule_62_e832);
        let noise_metadata_schedule_62_e834: f64 = (0.05 + noise_metadata_schedule_62_e833);
        (noise_metadata_schedule_62_e834,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_62_e836;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_63_e860,) = {
    if (noise_variable_443 == 0.0) {
        let noise_metadata_schedule_63_e842: f64 = (params.p114 * noise_variable_2);
        let noise_metadata_schedule_63_e844: f64 = (noise_metadata_schedule_63_e842 * noise_variable_2);
        let noise_metadata_schedule_63_e847: f64 = (noise_variable_2 + params.p115);
        let noise_metadata_schedule_63_e848: f64 = (noise_metadata_schedule_63_e844 / noise_metadata_schedule_63_e847);
        let noise_metadata_schedule_63_e849: f64 = (noise_variable_74 - noise_metadata_schedule_63_e848);
        let noise_metadata_schedule_63_e853: f64 = (-noise_variable_259);
        let noise_metadata_schedule_63_e854: f64 = (noise_metadata_schedule_63_e853).exp();
        let noise_metadata_schedule_63_e855: f64 = (1.0 + noise_metadata_schedule_63_e854);
        let noise_metadata_schedule_63_e856: f64 = (noise_metadata_schedule_63_e855).ln();
        let noise_metadata_schedule_63_e857: f64 = (0.1 * noise_metadata_schedule_63_e856);
        let noise_metadata_schedule_63_e858: f64 = (noise_metadata_schedule_63_e849 + noise_metadata_schedule_63_e857);
        (noise_metadata_schedule_63_e858,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_63_e860;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_64_e864: f64 = (params.p117 * noise_variable_2);
            let noise_metadata_schedule_64_e866: f64 = (noise_metadata_schedule_64_e864 * noise_variable_2);
            let noise_metadata_schedule_64_e869: f64 = (noise_variable_2 + params.p118);
            let noise_metadata_schedule_64_e870: f64 = (noise_metadata_schedule_64_e866 / noise_metadata_schedule_64_e869);
            let noise_metadata_schedule_64_e871: f64 = (noise_variable_88 - noise_metadata_schedule_64_e870);
            let noise_metadata_schedule_64_e873: f64 = (noise_metadata_schedule_64_e871 - 0.05);
            let noise_metadata_schedule_64_e875: f64 = (noise_metadata_schedule_64_e873 / 0.1);
            noise_variable_259 = noise_metadata_schedule_64_e875;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_65_e879: f64 = (params.p117 * noise_variable_2);
            let noise_metadata_schedule_65_e881: f64 = (noise_metadata_schedule_65_e879 * noise_variable_2);
            let noise_metadata_schedule_65_e884: f64 = (noise_variable_2 + params.p118);
            let noise_metadata_schedule_65_e885: f64 = (noise_metadata_schedule_65_e881 / noise_metadata_schedule_65_e884);
            let noise_metadata_schedule_65_e886: f64 = (noise_variable_88 - noise_metadata_schedule_65_e885);
            let noise_metadata_schedule_65_e888: f64 = if noise_metadata_schedule_65_e886 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_444 = noise_metadata_schedule_65_e888;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_66_e900,) = {
    if (noise_variable_444 != 0.0) {
        let noise_metadata_schedule_66_e894: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_66_e895: f64 = (1.0 + noise_metadata_schedule_66_e894);
        let noise_metadata_schedule_66_e896: f64 = (noise_metadata_schedule_66_e895).ln();
        let noise_metadata_schedule_66_e897: f64 = (0.1 * noise_metadata_schedule_66_e896);
        let noise_metadata_schedule_66_e898: f64 = (0.05 + noise_metadata_schedule_66_e897);
        (noise_metadata_schedule_66_e898,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_66_e900;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_67_e924,) = {
    if (noise_variable_444 == 0.0) {
        let noise_metadata_schedule_67_e906: f64 = (params.p117 * noise_variable_2);
        let noise_metadata_schedule_67_e908: f64 = (noise_metadata_schedule_67_e906 * noise_variable_2);
        let noise_metadata_schedule_67_e911: f64 = (noise_variable_2 + params.p118);
        let noise_metadata_schedule_67_e912: f64 = (noise_metadata_schedule_67_e908 / noise_metadata_schedule_67_e911);
        let noise_metadata_schedule_67_e913: f64 = (noise_variable_88 - noise_metadata_schedule_67_e912);
        let noise_metadata_schedule_67_e917: f64 = (-noise_variable_259);
        let noise_metadata_schedule_67_e918: f64 = (noise_metadata_schedule_67_e917).exp();
        let noise_metadata_schedule_67_e919: f64 = (1.0 + noise_metadata_schedule_67_e918);
        let noise_metadata_schedule_67_e920: f64 = (noise_metadata_schedule_67_e919).ln();
        let noise_metadata_schedule_67_e921: f64 = (0.1 * noise_metadata_schedule_67_e920);
        let noise_metadata_schedule_67_e922: f64 = (noise_metadata_schedule_67_e913 + noise_metadata_schedule_67_e921);
        (noise_metadata_schedule_67_e922,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_67_e924;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_68_e926: f64 = (-3.0);
            let noise_metadata_schedule_68_e928: f64 = (noise_metadata_schedule_68_e926 * noise_variable_6);
            let noise_metadata_schedule_68_e930: f64 = (noise_metadata_schedule_68_e928 * noise_variable_254);
            let noise_metadata_schedule_68_e933: f64 = (params.p65 * noise_variable_4);
            let noise_metadata_schedule_68_e934: f64 = (noise_metadata_schedule_68_e930 + noise_metadata_schedule_68_e933);
            let noise_metadata_schedule_68_e937: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_68_e939: f64 = (noise_metadata_schedule_68_e937 * params.p104);
            let noise_metadata_schedule_68_e940: f64 = (noise_metadata_schedule_68_e934 + noise_metadata_schedule_68_e939);
            noise_variable_13 = noise_metadata_schedule_68_e940;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_69_e943: f64 = (0.05 - noise_variable_13);
            let noise_metadata_schedule_69_e945: f64 = (noise_metadata_schedule_69_e943 / noise_variable_6);
            noise_variable_259 = noise_metadata_schedule_69_e945;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_70_e948: f64 = if 0.05 < noise_variable_13 { 1.0 } else { 0.0 };
            noise_variable_445 = noise_metadata_schedule_70_e948;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_71_e960,) = {
    if (noise_variable_445 != 0.0) {
        let noise_metadata_schedule_71_e954: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_71_e955: f64 = (1.0 + noise_metadata_schedule_71_e954);
        let noise_metadata_schedule_71_e956: f64 = (noise_metadata_schedule_71_e955).ln();
        let noise_metadata_schedule_71_e957: f64 = (noise_variable_6 * noise_metadata_schedule_71_e956);
        let noise_metadata_schedule_71_e958: f64 = (noise_variable_13 + noise_metadata_schedule_71_e957);
        (noise_metadata_schedule_71_e958,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_71_e960;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_72_e974,) = {
    if (noise_variable_445 == 0.0) {
        let noise_metadata_schedule_72_e967: f64 = (-noise_variable_259);
        let noise_metadata_schedule_72_e968: f64 = (noise_metadata_schedule_72_e967).exp();
        let noise_metadata_schedule_72_e969: f64 = (1.0 + noise_metadata_schedule_72_e968);
        let noise_metadata_schedule_72_e970: f64 = (noise_metadata_schedule_72_e969).ln();
        let noise_metadata_schedule_72_e971: f64 = (noise_variable_6 * noise_metadata_schedule_72_e970);
        let noise_metadata_schedule_72_e972: f64 = (0.05 + noise_metadata_schedule_72_e971);
        (noise_metadata_schedule_72_e972,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_72_e974;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_73_e976: f64 = (-3.0);
            let noise_metadata_schedule_73_e978: f64 = (noise_metadata_schedule_73_e976 * noise_variable_6);
            let noise_metadata_schedule_73_e980: f64 = (noise_metadata_schedule_73_e978 * noise_variable_254);
            let noise_metadata_schedule_73_e983: f64 = (params.p63 * noise_variable_4);
            let noise_metadata_schedule_73_e984: f64 = (noise_metadata_schedule_73_e980 + noise_metadata_schedule_73_e983);
            let noise_metadata_schedule_73_e987: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_73_e989: f64 = (noise_metadata_schedule_73_e987 * params.p109);
            let noise_metadata_schedule_73_e990: f64 = (noise_metadata_schedule_73_e984 + noise_metadata_schedule_73_e989);
            noise_variable_15 = noise_metadata_schedule_73_e990;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_74_e993: f64 = (0.05 - noise_variable_15);
            let noise_metadata_schedule_74_e995: f64 = (noise_metadata_schedule_74_e993 / noise_variable_6);
            noise_variable_259 = noise_metadata_schedule_74_e995;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_75_e998: f64 = if 0.05 < noise_variable_15 { 1.0 } else { 0.0 };
            noise_variable_446 = noise_metadata_schedule_75_e998;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_76_e1010,) = {
    if (noise_variable_446 != 0.0) {
        let noise_metadata_schedule_76_e1004: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_76_e1005: f64 = (1.0 + noise_metadata_schedule_76_e1004);
        let noise_metadata_schedule_76_e1006: f64 = (noise_metadata_schedule_76_e1005).ln();
        let noise_metadata_schedule_76_e1007: f64 = (noise_variable_6 * noise_metadata_schedule_76_e1006);
        let noise_metadata_schedule_76_e1008: f64 = (noise_variable_15 + noise_metadata_schedule_76_e1007);
        (noise_metadata_schedule_76_e1008,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_76_e1010;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_77_e1024,) = {
    if (noise_variable_446 == 0.0) {
        let noise_metadata_schedule_77_e1017: f64 = (-noise_variable_259);
        let noise_metadata_schedule_77_e1018: f64 = (noise_metadata_schedule_77_e1017).exp();
        let noise_metadata_schedule_77_e1019: f64 = (1.0 + noise_metadata_schedule_77_e1018);
        let noise_metadata_schedule_77_e1020: f64 = (noise_metadata_schedule_77_e1019).ln();
        let noise_metadata_schedule_77_e1021: f64 = (noise_variable_6 * noise_metadata_schedule_77_e1020);
        let noise_metadata_schedule_77_e1022: f64 = (0.05 + noise_metadata_schedule_77_e1021);
        (noise_metadata_schedule_77_e1022,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_77_e1024;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_83_e1076: f64 = (-3.0);
            let noise_metadata_schedule_83_e1078: f64 = (noise_metadata_schedule_83_e1076 * noise_variable_6);
            let noise_metadata_schedule_83_e1080: f64 = (noise_metadata_schedule_83_e1078 * noise_variable_254);
            let noise_metadata_schedule_83_e1083: f64 = (params.p70 * noise_variable_4);
            let noise_metadata_schedule_83_e1084: f64 = (noise_metadata_schedule_83_e1080 + noise_metadata_schedule_83_e1083);
            let noise_metadata_schedule_83_e1087: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_83_e1089: f64 = (noise_metadata_schedule_83_e1087 * params.p109);
            let noise_metadata_schedule_83_e1090: f64 = (noise_metadata_schedule_83_e1084 + noise_metadata_schedule_83_e1089);
            noise_variable_18 = noise_metadata_schedule_83_e1090;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_84_e1093: f64 = (0.05 - noise_variable_18);
            let noise_metadata_schedule_84_e1095: f64 = (noise_metadata_schedule_84_e1093 / noise_variable_6);
            noise_variable_259 = noise_metadata_schedule_84_e1095;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_85_e1098: f64 = if 0.05 < noise_variable_18 { 1.0 } else { 0.0 };
            noise_variable_448 = noise_metadata_schedule_85_e1098;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_86_e1110,) = {
    if (noise_variable_448 != 0.0) {
        let noise_metadata_schedule_86_e1104: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_86_e1105: f64 = (1.0 + noise_metadata_schedule_86_e1104);
        let noise_metadata_schedule_86_e1106: f64 = (noise_metadata_schedule_86_e1105).ln();
        let noise_metadata_schedule_86_e1107: f64 = (noise_variable_6 * noise_metadata_schedule_86_e1106);
        let noise_metadata_schedule_86_e1108: f64 = (noise_variable_18 + noise_metadata_schedule_86_e1107);
        (noise_metadata_schedule_86_e1108,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_86_e1110;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_87_e1124,) = {
    if (noise_variable_448 == 0.0) {
        let noise_metadata_schedule_87_e1117: f64 = (-noise_variable_259);
        let noise_metadata_schedule_87_e1118: f64 = (noise_metadata_schedule_87_e1117).exp();
        let noise_metadata_schedule_87_e1119: f64 = (1.0 + noise_metadata_schedule_87_e1118);
        let noise_metadata_schedule_87_e1120: f64 = (noise_metadata_schedule_87_e1119).ln();
        let noise_metadata_schedule_87_e1121: f64 = (noise_variable_6 * noise_metadata_schedule_87_e1120);
        let noise_metadata_schedule_87_e1122: f64 = (0.05 + noise_metadata_schedule_87_e1121);
        (noise_metadata_schedule_87_e1122,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_87_e1124;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_88_e1126: f64 = (-3.0);
            let noise_metadata_schedule_88_e1128: f64 = (noise_metadata_schedule_88_e1126 * noise_variable_6);
            let noise_metadata_schedule_88_e1130: f64 = (noise_metadata_schedule_88_e1128 * noise_variable_254);
            let noise_metadata_schedule_88_e1133: f64 = (noise_variable_75 * noise_variable_4);
            let noise_metadata_schedule_88_e1134: f64 = (noise_metadata_schedule_88_e1130 + noise_metadata_schedule_88_e1133);
            let noise_metadata_schedule_88_e1137: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_88_e1139: f64 = (noise_metadata_schedule_88_e1137 * params.p109);
            let noise_metadata_schedule_88_e1140: f64 = (noise_metadata_schedule_88_e1134 + noise_metadata_schedule_88_e1139);
            noise_variable_20 = noise_metadata_schedule_88_e1140;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_89_e1143: f64 = (0.05 - noise_variable_20);
            let noise_metadata_schedule_89_e1145: f64 = (noise_metadata_schedule_89_e1143 / noise_variable_6);
            noise_variable_259 = noise_metadata_schedule_89_e1145;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_90_e1148: f64 = if 0.05 < noise_variable_20 { 1.0 } else { 0.0 };
            noise_variable_449 = noise_metadata_schedule_90_e1148;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_91_e1160,) = {
    if (noise_variable_449 != 0.0) {
        let noise_metadata_schedule_91_e1154: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_91_e1155: f64 = (1.0 + noise_metadata_schedule_91_e1154);
        let noise_metadata_schedule_91_e1156: f64 = (noise_metadata_schedule_91_e1155).ln();
        let noise_metadata_schedule_91_e1157: f64 = (noise_variable_6 * noise_metadata_schedule_91_e1156);
        let noise_metadata_schedule_91_e1158: f64 = (noise_variable_20 + noise_metadata_schedule_91_e1157);
        (noise_metadata_schedule_91_e1158,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_91_e1160;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_92_e1174,) = {
    if (noise_variable_449 == 0.0) {
        let noise_metadata_schedule_92_e1167: f64 = (-noise_variable_259);
        let noise_metadata_schedule_92_e1168: f64 = (noise_metadata_schedule_92_e1167).exp();
        let noise_metadata_schedule_92_e1169: f64 = (1.0 + noise_metadata_schedule_92_e1168);
        let noise_metadata_schedule_92_e1170: f64 = (noise_metadata_schedule_92_e1169).ln();
        let noise_metadata_schedule_92_e1171: f64 = (noise_variable_6 * noise_metadata_schedule_92_e1170);
        let noise_metadata_schedule_92_e1172: f64 = (0.05 + noise_metadata_schedule_92_e1171);
        (noise_metadata_schedule_92_e1172,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_92_e1174;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_93_e1176: f64 = (-3.0);
            let noise_metadata_schedule_93_e1178: f64 = (noise_metadata_schedule_93_e1176 * noise_variable_6);
            let noise_metadata_schedule_93_e1180: f64 = (noise_metadata_schedule_93_e1178 * noise_variable_254);
            let noise_metadata_schedule_93_e1183: f64 = (params.p26 * noise_variable_4);
            let noise_metadata_schedule_93_e1184: f64 = (noise_metadata_schedule_93_e1180 + noise_metadata_schedule_93_e1183);
            let noise_metadata_schedule_93_e1187: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_93_e1189: f64 = (noise_metadata_schedule_93_e1187 * params.p108);
            let noise_metadata_schedule_93_e1190: f64 = (noise_metadata_schedule_93_e1184 + noise_metadata_schedule_93_e1189);
            noise_variable_56 = noise_metadata_schedule_93_e1190;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_94_e1193: f64 = (0.05 - noise_variable_56);
            let noise_metadata_schedule_94_e1195: f64 = (noise_metadata_schedule_94_e1193 / noise_variable_6);
            noise_variable_259 = noise_metadata_schedule_94_e1195;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_95_e1198: f64 = if 0.05 < noise_variable_56 { 1.0 } else { 0.0 };
            noise_variable_450 = noise_metadata_schedule_95_e1198;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_96_e1210,) = {
    if (noise_variable_450 != 0.0) {
        let noise_metadata_schedule_96_e1204: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_96_e1205: f64 = (1.0 + noise_metadata_schedule_96_e1204);
        let noise_metadata_schedule_96_e1206: f64 = (noise_metadata_schedule_96_e1205).ln();
        let noise_metadata_schedule_96_e1207: f64 = (noise_variable_6 * noise_metadata_schedule_96_e1206);
        let noise_metadata_schedule_96_e1208: f64 = (noise_variable_56 + noise_metadata_schedule_96_e1207);
        (noise_metadata_schedule_96_e1208,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_96_e1210;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_97_e1224,) = {
    if (noise_variable_450 == 0.0) {
        let noise_metadata_schedule_97_e1217: f64 = (-noise_variable_259);
        let noise_metadata_schedule_97_e1218: f64 = (noise_metadata_schedule_97_e1217).exp();
        let noise_metadata_schedule_97_e1219: f64 = (1.0 + noise_metadata_schedule_97_e1218);
        let noise_metadata_schedule_97_e1220: f64 = (noise_metadata_schedule_97_e1219).ln();
        let noise_metadata_schedule_97_e1221: f64 = (noise_variable_6 * noise_metadata_schedule_97_e1220);
        let noise_metadata_schedule_97_e1222: f64 = (0.05 + noise_metadata_schedule_97_e1221);
        (noise_metadata_schedule_97_e1222,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_97_e1224;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_98_e1227: f64 = (1.0 / noise_variable_14);
            noise_variable_65 = noise_metadata_schedule_98_e1227;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_99_e1230: f64 = (1.0 / noise_variable_19);
            noise_variable_67 = noise_metadata_schedule_99_e1230;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_100_e1233: f64 = (params.p65 * noise_variable_65);
            let noise_metadata_schedule_100_e1235: f64 = (noise_metadata_schedule_100_e1233).powf(params.p66);
            noise_variable_73 = noise_metadata_schedule_100_e1235;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_101_e1238: f64 = (noise_variable_75 * noise_variable_67);
            let noise_metadata_schedule_101_e1240: f64 = (noise_metadata_schedule_101_e1238).powf(noise_variable_76);
            noise_variable_90 = noise_metadata_schedule_101_e1240;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_103_e1246: f64 = (1.0 - params.p74);
            let noise_metadata_schedule_103_e1249: f64 = (params.p70 / noise_variable_17);
            let noise_metadata_schedule_103_e1251: f64 = (noise_metadata_schedule_103_e1249).powf(params.p71);
            let noise_metadata_schedule_103_e1252: f64 = (noise_metadata_schedule_103_e1246 * noise_metadata_schedule_103_e1251);
            let noise_metadata_schedule_103_e1254: f64 = (noise_metadata_schedule_103_e1252 + params.p74);
            noise_variable_26 = noise_metadata_schedule_103_e1254;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_104_e1257: f64 = (1.0 / noise_variable_26);
            noise_variable_27 = noise_metadata_schedule_104_e1257;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_106_e1263: f64 = (params.p74 * noise_variable_27);
            noise_variable_25 = noise_metadata_schedule_106_e1263;
        }
        if matches!(source_index, 1 | 3) {
            let noise_metadata_schedule_107_e1267: f64 = (noise_variable_254 * params.p96);
            let noise_metadata_schedule_107_e1268: f64 = (noise_metadata_schedule_107_e1267).exp();
            let noise_metadata_schedule_107_e1269: f64 = (params.p53 * noise_metadata_schedule_107_e1268);
            noise_variable_28 = noise_metadata_schedule_107_e1269;
        }
        if matches!(source_index, 1 | 3) {
            let noise_metadata_schedule_108_e1272: f64 = if noise_variable_28 < noise_variable_316 { 1.0 } else { 0.0 };
            noise_variable_451 = noise_metadata_schedule_108_e1272;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_109_e1276,) = {
    if (noise_variable_451 != 0.0) {
        (noise_variable_316,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_109_e1276;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_110_e1281: f64 = (params.p97 - params.p95);
            let noise_metadata_schedule_110_e1282: f64 = (noise_variable_254 * noise_metadata_schedule_110_e1281);
            let noise_metadata_schedule_110_e1283: f64 = (noise_metadata_schedule_110_e1282).exp();
            let noise_metadata_schedule_110_e1284: f64 = (params.p55 * noise_metadata_schedule_110_e1283);
            noise_variable_29 = noise_metadata_schedule_110_e1284;
        }
        if matches!(source_index, 1 | 4) {
            let noise_metadata_schedule_111_e1288: f64 = (noise_variable_254 * params.p100);
            let noise_metadata_schedule_111_e1289: f64 = (noise_metadata_schedule_111_e1288).exp();
            let noise_metadata_schedule_111_e1290: f64 = (params.p54 * noise_metadata_schedule_111_e1289);
            noise_variable_30 = noise_metadata_schedule_111_e1290;
        }
        if matches!(source_index, 1 | 4) {
            let noise_metadata_schedule_112_e1293: f64 = if noise_variable_30 < noise_variable_316 { 1.0 } else { 0.0 };
            noise_variable_452 = noise_metadata_schedule_112_e1293;
        }
        if matches!(source_index, 1 | 4) {
            let (noise_metadata_schedule_113_e1297,) = {
    if (noise_variable_452 != 0.0) {
        (noise_variable_316,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_113_e1297;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 20 | 22 | 24) {
            let noise_metadata_schedule_114_e1301: f64 = (noise_variable_254 * params.p101);
            let noise_metadata_schedule_114_e1302: f64 = (noise_metadata_schedule_114_e1301).exp();
            let noise_metadata_schedule_114_e1303: f64 = (params.p56 * noise_metadata_schedule_114_e1302);
            noise_variable_32 = noise_metadata_schedule_114_e1303;
        }
        if matches!(source_index, 18 | 21) {
            let noise_metadata_schedule_115_e1307: f64 = (noise_variable_254 * params.p103);
            let noise_metadata_schedule_115_e1308: f64 = (noise_metadata_schedule_115_e1307).exp();
            let noise_metadata_schedule_115_e1309: f64 = (params.p57 * noise_metadata_schedule_115_e1308);
            noise_variable_33 = noise_metadata_schedule_115_e1309;
        }
        if matches!(source_index, 19 | 23) {
            let noise_metadata_schedule_116_e1313: f64 = (noise_variable_254 * params.p103);
            let noise_metadata_schedule_116_e1314: f64 = (noise_metadata_schedule_116_e1313).exp();
            let noise_metadata_schedule_116_e1315: f64 = (params.p58 * noise_metadata_schedule_116_e1314);
            noise_variable_34 = noise_metadata_schedule_116_e1315;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_117_e1319: f64 = (noise_variable_254 * params.p98);
            let noise_metadata_schedule_117_e1320: f64 = (noise_metadata_schedule_117_e1319).exp();
            let noise_metadata_schedule_117_e1321: f64 = (params.p59 * noise_metadata_schedule_117_e1320);
            noise_variable_31 = noise_metadata_schedule_117_e1321;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_118_e1324: f64 = if params.p121 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_453 = noise_metadata_schedule_118_e1324;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_119_e1334,) = {
    if (noise_variable_453 != 0.0) {
        let noise_metadata_schedule_119_e1330: f64 = (noise_variable_12 * params.p121);
        let noise_metadata_schedule_119_e1331: f64 = (1.0 + noise_metadata_schedule_119_e1330);
        let noise_metadata_schedule_119_e1332: f64 = (params.p9 * noise_metadata_schedule_119_e1331);
        (noise_metadata_schedule_119_e1332,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_119_e1334;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_120_e1342,) = {
    if (noise_variable_453 != 0.0) {
        let noise_metadata_schedule_120_e1338: f64 = (noise_variable_50 - 1.0);
        let noise_metadata_schedule_120_e1340: f64 = (noise_metadata_schedule_120_e1338 / noise_variable_52);
        (noise_metadata_schedule_120_e1340,)
    } else {
        (noise_variable_259,)
    }
};
            noise_variable_259 = noise_metadata_schedule_120_e1342;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_121_e1345: f64 = if noise_variable_50 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_454 = noise_metadata_schedule_121_e1345;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_122_e1359,) = {
    if ((noise_variable_453 != 0.0) && (noise_variable_454 != 0.0)) {
        let noise_metadata_schedule_122_e1353: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_122_e1354: f64 = (1.0 + noise_metadata_schedule_122_e1353);
        let noise_metadata_schedule_122_e1355: f64 = (noise_metadata_schedule_122_e1354).ln();
        let noise_metadata_schedule_122_e1356: f64 = (noise_variable_52 * noise_metadata_schedule_122_e1355);
        let noise_metadata_schedule_122_e1357: f64 = (1.0 + noise_metadata_schedule_122_e1356);
        (noise_metadata_schedule_122_e1357,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_122_e1359;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_123_e1375,) = {
    if ((noise_variable_453 != 0.0) && (noise_variable_454 == 0.0)) {
        let noise_metadata_schedule_123_e1368: f64 = (-noise_variable_259);
        let noise_metadata_schedule_123_e1369: f64 = (noise_metadata_schedule_123_e1368).exp();
        let noise_metadata_schedule_123_e1370: f64 = (1.0 + noise_metadata_schedule_123_e1369);
        let noise_metadata_schedule_123_e1371: f64 = (noise_metadata_schedule_123_e1370).ln();
        let noise_metadata_schedule_123_e1372: f64 = (noise_variable_52 * noise_metadata_schedule_123_e1371);
        let noise_metadata_schedule_123_e1373: f64 = (noise_variable_50 + noise_metadata_schedule_123_e1372);
        (noise_metadata_schedule_123_e1373,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_123_e1375;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_124_e1383,) = {
    if (noise_variable_453 != 0.0) {
        let noise_metadata_schedule_124_e1380: f64 = (noise_variable_52 * 0.6931471805599453);
        let noise_metadata_schedule_124_e1381: f64 = (noise_variable_50 - noise_metadata_schedule_124_e1380);
        (noise_metadata_schedule_124_e1381,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_124_e1383;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_125_e1388,) = {
    if (noise_variable_453 == 0.0) {
        (params.p9,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_125_e1388;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_126_e1391: f64 = if params.p122 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_455 = noise_metadata_schedule_126_e1391;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_127_e1401,) = {
    if (noise_variable_455 != 0.0) {
        let noise_metadata_schedule_127_e1397: f64 = (noise_variable_12 * params.p122);
        let noise_metadata_schedule_127_e1398: f64 = (1.0 + noise_metadata_schedule_127_e1397);
        let noise_metadata_schedule_127_e1399: f64 = (params.p10 * noise_metadata_schedule_127_e1398);
        (noise_metadata_schedule_127_e1399,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_127_e1401;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_128_e1409,) = {
    if (noise_variable_455 != 0.0) {
        let noise_metadata_schedule_128_e1405: f64 = (noise_variable_51 - 1.0);
        let noise_metadata_schedule_128_e1407: f64 = (noise_metadata_schedule_128_e1405 / noise_variable_52);
        (noise_metadata_schedule_128_e1407,)
    } else {
        (noise_variable_259,)
    }
};
            noise_variable_259 = noise_metadata_schedule_128_e1409;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_129_e1412: f64 = if noise_variable_51 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_456 = noise_metadata_schedule_129_e1412;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_130_e1426,) = {
    if ((noise_variable_455 != 0.0) && (noise_variable_456 != 0.0)) {
        let noise_metadata_schedule_130_e1420: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_130_e1421: f64 = (1.0 + noise_metadata_schedule_130_e1420);
        let noise_metadata_schedule_130_e1422: f64 = (noise_metadata_schedule_130_e1421).ln();
        let noise_metadata_schedule_130_e1423: f64 = (noise_variable_52 * noise_metadata_schedule_130_e1422);
        let noise_metadata_schedule_130_e1424: f64 = (1.0 + noise_metadata_schedule_130_e1423);
        (noise_metadata_schedule_130_e1424,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_130_e1426;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_131_e1442,) = {
    if ((noise_variable_455 != 0.0) && (noise_variable_456 == 0.0)) {
        let noise_metadata_schedule_131_e1435: f64 = (-noise_variable_259);
        let noise_metadata_schedule_131_e1436: f64 = (noise_metadata_schedule_131_e1435).exp();
        let noise_metadata_schedule_131_e1437: f64 = (1.0 + noise_metadata_schedule_131_e1436);
        let noise_metadata_schedule_131_e1438: f64 = (noise_metadata_schedule_131_e1437).ln();
        let noise_metadata_schedule_131_e1439: f64 = (noise_variable_52 * noise_metadata_schedule_131_e1438);
        let noise_metadata_schedule_131_e1440: f64 = (noise_variable_51 + noise_metadata_schedule_131_e1439);
        (noise_metadata_schedule_131_e1440,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_131_e1442;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_132_e1450,) = {
    if (noise_variable_455 != 0.0) {
        let noise_metadata_schedule_132_e1447: f64 = (noise_variable_52 * 0.6931471805599453);
        let noise_metadata_schedule_132_e1448: f64 = (noise_variable_51 - noise_metadata_schedule_132_e1447);
        (noise_metadata_schedule_132_e1448,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_132_e1450;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_133_e1455,) = {
    if (noise_variable_455 == 0.0) {
        (params.p10,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_133_e1455;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_134_e1460: f64 = (params.p123 * noise_variable_12);
            let noise_metadata_schedule_134_e1461: f64 = (1.0 + noise_metadata_schedule_134_e1460);
            let noise_metadata_schedule_134_e1462: f64 = (params.p42 * noise_metadata_schedule_134_e1461);
            noise_variable_311 = noise_metadata_schedule_134_e1462;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_135_e1465: f64 = (noise_variable_312 * noise_variable_312);
            noise_variable_261 = noise_metadata_schedule_135_e1465;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_136_e1468: f64 = (noise_variable_311 * noise_variable_311);
            noise_variable_262 = noise_metadata_schedule_136_e1468;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_137_e1471: f64 = if noise_variable_311 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_457 = noise_metadata_schedule_137_e1471;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_138_e1484,) = {
    if (noise_variable_457 != 0.0) {
        let noise_metadata_schedule_138_e1475: f64 = (0.5 * noise_variable_261);
        let noise_metadata_schedule_138_e1478: f64 = (noise_variable_262 + noise_variable_261);
        let noise_metadata_schedule_138_e1479: f64 = (noise_metadata_schedule_138_e1478).sqrt();
        let noise_metadata_schedule_138_e1481: f64 = (noise_metadata_schedule_138_e1479 - noise_variable_311);
        let noise_metadata_schedule_138_e1482: f64 = (noise_metadata_schedule_138_e1475 / noise_metadata_schedule_138_e1481);
        (noise_metadata_schedule_138_e1482,)
    } else {
        (noise_variable_310,)
    }
};
            noise_variable_310 = noise_metadata_schedule_138_e1484;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_139_e1496,) = {
    if (noise_variable_457 == 0.0) {
        let noise_metadata_schedule_139_e1490: f64 = (noise_variable_262 + noise_variable_261);
        let noise_metadata_schedule_139_e1491: f64 = (noise_metadata_schedule_139_e1490).sqrt();
        let noise_metadata_schedule_139_e1493: f64 = (noise_metadata_schedule_139_e1491 + noise_variable_311);
        let noise_metadata_schedule_139_e1494: f64 = (0.5 * noise_metadata_schedule_139_e1493);
        (noise_metadata_schedule_139_e1494,)
    } else {
        (noise_variable_310,)
    }
};
            noise_variable_310 = noise_metadata_schedule_139_e1496;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_140_e1501: f64 = (4.0 - params.p97);
            let noise_metadata_schedule_140_e1503: f64 = (noise_metadata_schedule_140_e1501 - params.p95);
            let noise_metadata_schedule_140_e1505: f64 = (noise_metadata_schedule_140_e1503 + params.p120);
            let noise_metadata_schedule_140_e1506: f64 = (noise_variable_254 * noise_metadata_schedule_140_e1505);
            let noise_metadata_schedule_140_e1508: f64 = (noise_metadata_schedule_140_e1506 / noise_variable_48);
            let noise_metadata_schedule_140_e1509: f64 = (noise_metadata_schedule_140_e1508).exp();
            let noise_metadata_schedule_140_e1510: f64 = (params.p8 * noise_metadata_schedule_140_e1509);
            let noise_metadata_schedule_140_e1512: f64 = (-params.p104);
            let noise_metadata_schedule_140_e1514: f64 = (noise_metadata_schedule_140_e1512 * noise_variable_10);
            let noise_metadata_schedule_140_e1516: f64 = (noise_metadata_schedule_140_e1514 / noise_variable_48);
            let noise_metadata_schedule_140_e1517: f64 = (noise_metadata_schedule_140_e1516).exp();
            let noise_metadata_schedule_140_e1518: f64 = (noise_metadata_schedule_140_e1510 * noise_metadata_schedule_140_e1517);
            noise_variable_35 = noise_metadata_schedule_140_e1518;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_141_e1523: f64 = (1.0 - params.p97);
            let noise_metadata_schedule_141_e1524: f64 = (noise_variable_254 * noise_metadata_schedule_141_e1523);
            let noise_metadata_schedule_141_e1525: f64 = (noise_metadata_schedule_141_e1524).exp();
            let noise_metadata_schedule_141_e1526: f64 = (params.p11 * noise_metadata_schedule_141_e1525);
            noise_variable_36 = noise_metadata_schedule_141_e1526;
        }
        if matches!(source_index, 11 | 12 | 13 | 14) {
            let noise_metadata_schedule_142_e1531: f64 = (1.0 - params.p102);
            let noise_metadata_schedule_142_e1532: f64 = (noise_variable_254 * noise_metadata_schedule_142_e1531);
            let noise_metadata_schedule_142_e1533: f64 = (noise_metadata_schedule_142_e1532).exp();
            let noise_metadata_schedule_142_e1534: f64 = (params.p29 * noise_metadata_schedule_142_e1533);
            noise_variable_37 = noise_metadata_schedule_142_e1534;
        }
        if matches!(source_index, 2 | 7) {
            let noise_metadata_schedule_143_e1540: f64 = (2.0 * params.p20);
            let noise_metadata_schedule_143_e1541: f64 = (6.0 - noise_metadata_schedule_143_e1540);
            let noise_metadata_schedule_143_e1542: f64 = (noise_variable_254 * noise_metadata_schedule_143_e1541);
            let noise_metadata_schedule_143_e1543: f64 = (noise_metadata_schedule_143_e1542).exp();
            let noise_metadata_schedule_143_e1544: f64 = (params.p19 * noise_metadata_schedule_143_e1543);
            let noise_metadata_schedule_143_e1546: f64 = (-params.p112);
            let noise_metadata_schedule_143_e1548: f64 = (noise_metadata_schedule_143_e1546 * noise_variable_10);
            let noise_metadata_schedule_143_e1550: f64 = (noise_metadata_schedule_143_e1548 / params.p20);
            let noise_metadata_schedule_143_e1551: f64 = (noise_metadata_schedule_143_e1550).exp();
            let noise_metadata_schedule_143_e1552: f64 = (noise_metadata_schedule_143_e1544 * noise_metadata_schedule_143_e1551);
            noise_variable_38 = noise_metadata_schedule_143_e1552;
        }
        if matches!(source_index, 9 | 10) {
            let noise_metadata_schedule_144_e1558: f64 = (2.0 * params.p31);
            let noise_metadata_schedule_144_e1559: f64 = (6.0 - noise_metadata_schedule_144_e1558);
            let noise_metadata_schedule_144_e1560: f64 = (noise_variable_254 * noise_metadata_schedule_144_e1559);
            let noise_metadata_schedule_144_e1561: f64 = (noise_metadata_schedule_144_e1560).exp();
            let noise_metadata_schedule_144_e1562: f64 = (params.p30 * noise_metadata_schedule_144_e1561);
            let noise_metadata_schedule_144_e1564: f64 = (-params.p109);
            let noise_metadata_schedule_144_e1566: f64 = (noise_metadata_schedule_144_e1564 * noise_variable_10);
            let noise_metadata_schedule_144_e1568: f64 = (noise_metadata_schedule_144_e1566 / params.p31);
            let noise_metadata_schedule_144_e1569: f64 = (noise_metadata_schedule_144_e1568).exp();
            let noise_metadata_schedule_144_e1570: f64 = (noise_metadata_schedule_144_e1562 * noise_metadata_schedule_144_e1569);
            noise_variable_39 = noise_metadata_schedule_144_e1570;
        }
        if matches!(source_index, 1 | 2 | 6) {
            let noise_metadata_schedule_145_e1575: f64 = (4.0 - params.p96);
            let noise_metadata_schedule_145_e1577: f64 = (noise_metadata_schedule_145_e1575 + params.p120);
            let noise_metadata_schedule_145_e1578: f64 = (noise_variable_254 * noise_metadata_schedule_145_e1577);
            let noise_metadata_schedule_145_e1580: f64 = (noise_metadata_schedule_145_e1578 / params.p16);
            let noise_metadata_schedule_145_e1581: f64 = (noise_metadata_schedule_145_e1580).exp();
            let noise_metadata_schedule_145_e1582: f64 = (params.p15 * noise_metadata_schedule_145_e1581);
            let noise_metadata_schedule_145_e1584: f64 = (-params.p110);
            let noise_metadata_schedule_145_e1586: f64 = (noise_metadata_schedule_145_e1584 * noise_variable_10);
            let noise_metadata_schedule_145_e1588: f64 = (noise_metadata_schedule_145_e1586 / params.p16);
            let noise_metadata_schedule_145_e1589: f64 = (noise_metadata_schedule_145_e1588).exp();
            let noise_metadata_schedule_145_e1590: f64 = (noise_metadata_schedule_145_e1582 * noise_metadata_schedule_145_e1589);
            noise_variable_42 = noise_metadata_schedule_145_e1590;
        }
        if matches!(source_index, 6 | 8) {
            let noise_metadata_schedule_146_e1595: f64 = (4.0 - params.p96);
            let noise_metadata_schedule_146_e1597: f64 = (noise_metadata_schedule_146_e1595 + params.p120);
            let noise_metadata_schedule_146_e1598: f64 = (noise_variable_254 * noise_metadata_schedule_146_e1597);
            let noise_metadata_schedule_146_e1600: f64 = (noise_metadata_schedule_146_e1598 / params.p18);
            let noise_metadata_schedule_146_e1601: f64 = (noise_metadata_schedule_146_e1600).exp();
            let noise_metadata_schedule_146_e1602: f64 = (params.p17 * noise_metadata_schedule_146_e1601);
            let noise_metadata_schedule_146_e1604: f64 = (-params.p110);
            let noise_metadata_schedule_146_e1606: f64 = (noise_metadata_schedule_146_e1604 * noise_variable_10);
            let noise_metadata_schedule_146_e1608: f64 = (noise_metadata_schedule_146_e1606 / params.p18);
            let noise_metadata_schedule_146_e1609: f64 = (noise_metadata_schedule_146_e1608).exp();
            let noise_metadata_schedule_146_e1610: f64 = (noise_metadata_schedule_146_e1602 * noise_metadata_schedule_146_e1609);
            noise_variable_44 = noise_metadata_schedule_146_e1610;
        }
        if matches!(source_index, 2 | 6 | 8) {
            let noise_metadata_schedule_147_e1613: f64 = if params.p23 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_458 = noise_metadata_schedule_147_e1613;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_148_e1625,) = {
    if (noise_variable_458 != 0.0) {
        let noise_metadata_schedule_148_e1617: f64 = (-params.p106);
        let noise_metadata_schedule_148_e1619: f64 = (noise_metadata_schedule_148_e1617 * noise_variable_10);
        let noise_metadata_schedule_148_e1621: f64 = (noise_metadata_schedule_148_e1619 / params.p16);
        let noise_metadata_schedule_148_e1622: f64 = (noise_metadata_schedule_148_e1621).exp();
        let noise_metadata_schedule_148_e1623: f64 = (params.p24 * noise_metadata_schedule_148_e1622);
        (noise_metadata_schedule_148_e1623,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_148_e1625;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_149_e1635,) = {
    if (noise_variable_458 != 0.0) {
        let noise_metadata_schedule_149_e1629: f64 = (-params.p105);
        let noise_metadata_schedule_149_e1631: f64 = (noise_metadata_schedule_149_e1629 * noise_variable_10);
        let noise_metadata_schedule_149_e1632: f64 = (noise_metadata_schedule_149_e1631).exp();
        let noise_metadata_schedule_149_e1633: f64 = (params.p27 * noise_metadata_schedule_149_e1632);
        (noise_metadata_schedule_149_e1633,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_149_e1635;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_150_e1647,) = {
    if (noise_variable_458 != 0.0) {
        let noise_metadata_schedule_150_e1639: f64 = (-params.p107);
        let noise_metadata_schedule_150_e1641: f64 = (noise_metadata_schedule_150_e1639 * noise_variable_10);
        let noise_metadata_schedule_150_e1643: f64 = (noise_metadata_schedule_150_e1641 / params.p18);
        let noise_metadata_schedule_150_e1644: f64 = (noise_metadata_schedule_150_e1643).exp();
        let noise_metadata_schedule_150_e1645: f64 = (params.p25 * noise_metadata_schedule_150_e1644);
        (noise_metadata_schedule_150_e1645,)
    } else {
        (noise_variable_45,)
    }
};
            noise_variable_45 = noise_metadata_schedule_150_e1647;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_151_e1652: f64 = (4.0 - params.p102);
            let noise_metadata_schedule_151_e1654: f64 = (noise_metadata_schedule_151_e1652 + params.p120);
            let noise_metadata_schedule_151_e1655: f64 = (noise_variable_254 * noise_metadata_schedule_151_e1654);
            let noise_metadata_schedule_151_e1656: f64 = (noise_metadata_schedule_151_e1655).exp();
            let noise_metadata_schedule_151_e1657: f64 = (params.p28 * noise_metadata_schedule_151_e1656);
            let noise_metadata_schedule_151_e1659: f64 = (-params.p111);
            let noise_metadata_schedule_151_e1661: f64 = (noise_metadata_schedule_151_e1659 * noise_variable_10);
            let noise_metadata_schedule_151_e1662: f64 = (noise_metadata_schedule_151_e1661).exp();
            let noise_metadata_schedule_151_e1663: f64 = (noise_metadata_schedule_151_e1657 * noise_metadata_schedule_151_e1662);
            noise_variable_43 = noise_metadata_schedule_151_e1663;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_152_e1669: f64 = (2.0 * params.p22);
            let noise_metadata_schedule_152_e1670: f64 = (6.0 - noise_metadata_schedule_152_e1669);
            let noise_metadata_schedule_152_e1671: f64 = (noise_variable_254 * noise_metadata_schedule_152_e1670);
            let noise_metadata_schedule_152_e1672: f64 = (noise_metadata_schedule_152_e1671).exp();
            let noise_metadata_schedule_152_e1673: f64 = (params.p21 * noise_metadata_schedule_152_e1672);
            let noise_metadata_schedule_152_e1675: f64 = (-params.p112);
            let noise_metadata_schedule_152_e1677: f64 = (noise_metadata_schedule_152_e1675 * noise_variable_10);
            let noise_metadata_schedule_152_e1679: f64 = (noise_metadata_schedule_152_e1677 / params.p22);
            let noise_metadata_schedule_152_e1680: f64 = (noise_metadata_schedule_152_e1679).exp();
            let noise_metadata_schedule_152_e1681: f64 = (noise_metadata_schedule_152_e1673 * noise_metadata_schedule_152_e1680);
            noise_variable_46 = noise_metadata_schedule_152_e1681;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_153_e1686: f64 = (4.0 / params.p133);
            let noise_metadata_schedule_153_e1687: f64 = (noise_variable_254 * noise_metadata_schedule_153_e1686);
            let noise_metadata_schedule_153_e1688: f64 = (noise_metadata_schedule_153_e1687).exp();
            let noise_metadata_schedule_153_e1689: f64 = (params.p132 * noise_metadata_schedule_153_e1688);
            let noise_metadata_schedule_153_e1691: f64 = (-params.p112);
            let noise_metadata_schedule_153_e1693: f64 = (noise_metadata_schedule_153_e1691 * noise_variable_10);
            let noise_metadata_schedule_153_e1695: f64 = (noise_metadata_schedule_153_e1693 / params.p133);
            let noise_metadata_schedule_153_e1696: f64 = (noise_metadata_schedule_153_e1695).exp();
            let noise_metadata_schedule_153_e1697: f64 = (noise_metadata_schedule_153_e1689 * noise_metadata_schedule_153_e1696);
            noise_variable_47 = noise_metadata_schedule_153_e1697;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_154_e1700: f64 = (noise_variable_4).sqrt();
            let noise_metadata_schedule_154_e1701: f64 = (params.p138 * noise_metadata_schedule_154_e1700);
            let noise_metadata_schedule_154_e1704: f64 = (params.p140 * noise_variable_12);
            let noise_metadata_schedule_154_e1705: f64 = (noise_metadata_schedule_154_e1704).exp();
            let noise_metadata_schedule_154_e1706: f64 = (noise_metadata_schedule_154_e1701 * noise_metadata_schedule_154_e1705);
            noise_variable_325 = noise_metadata_schedule_154_e1706;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_155_e1709: f64 = (noise_variable_70 * noise_variable_72);
            let noise_metadata_schedule_155_e1711: f64 = (-0.5);
            let noise_metadata_schedule_155_e1712: f64 = (noise_metadata_schedule_155_e1709).powf(noise_metadata_schedule_155_e1711);
            noise_variable_255 = noise_metadata_schedule_155_e1712;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_156_e1715: f64 = (1.0 / noise_variable_73);
            noise_variable_256 = noise_metadata_schedule_156_e1715;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_157_e1718: f64 = (params.p34 * noise_variable_70);
            let noise_metadata_schedule_157_e1720: f64 = (noise_metadata_schedule_157_e1718 * noise_variable_70);
            let noise_metadata_schedule_157_e1722: f64 = (noise_metadata_schedule_157_e1720 * noise_variable_255);
            let noise_metadata_schedule_157_e1724: f64 = (noise_metadata_schedule_157_e1722 * noise_variable_256);
            let noise_metadata_schedule_157_e1726: f64 = (noise_metadata_schedule_157_e1724 * params.p65);
            let noise_metadata_schedule_157_e1728: f64 = (noise_metadata_schedule_157_e1726 * noise_variable_65);
            let noise_metadata_schedule_157_e1730: f64 = (noise_metadata_schedule_157_e1728 * noise_variable_72);
            let noise_metadata_schedule_157_e1732: f64 = (noise_metadata_schedule_157_e1730 * noise_variable_72);
            noise_variable_61 = noise_metadata_schedule_157_e1732;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_158_e1735: f64 = (params.p33 * noise_variable_255);
            let noise_metadata_schedule_158_e1737: f64 = (noise_metadata_schedule_158_e1735 * noise_variable_14);
            let noise_metadata_schedule_158_e1739: f64 = (noise_metadata_schedule_158_e1737 * noise_variable_14);
            let noise_metadata_schedule_158_e1741: f64 = (noise_metadata_schedule_158_e1739 * noise_variable_64);
            let noise_metadata_schedule_158_e1743: f64 = (noise_metadata_schedule_158_e1741 * noise_variable_64);
            let noise_metadata_schedule_158_e1745: f64 = (noise_metadata_schedule_158_e1743 * noise_variable_73);
            let noise_metadata_schedule_158_e1748: f64 = (params.p34 - noise_variable_61);
            let noise_metadata_schedule_158_e1749: f64 = (noise_metadata_schedule_158_e1748).exp();
            let noise_metadata_schedule_158_e1750: f64 = (noise_metadata_schedule_158_e1745 * noise_metadata_schedule_158_e1749);
            noise_variable_58 = noise_metadata_schedule_158_e1750;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_159_e1753: f64 = (1.0 / noise_variable_19);
            noise_variable_67 = noise_metadata_schedule_159_e1753;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_160_e1756: f64 = (noise_variable_85 * noise_variable_86);
            let noise_metadata_schedule_160_e1758: f64 = (-0.5);
            let noise_metadata_schedule_160_e1759: f64 = (noise_metadata_schedule_160_e1756).powf(noise_metadata_schedule_160_e1758);
            noise_variable_257 = noise_metadata_schedule_160_e1759;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_161_e1762: f64 = (1.0 / noise_variable_90);
            noise_variable_258 = noise_metadata_schedule_161_e1762;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_162_e1765: f64 = (params.p36 * noise_variable_85);
            let noise_metadata_schedule_162_e1767: f64 = (noise_metadata_schedule_162_e1765 * noise_variable_85);
            let noise_metadata_schedule_162_e1769: f64 = (noise_metadata_schedule_162_e1767 * noise_variable_257);
            let noise_metadata_schedule_162_e1771: f64 = (noise_metadata_schedule_162_e1769 * noise_variable_258);
            let noise_metadata_schedule_162_e1773: f64 = (noise_metadata_schedule_162_e1771 * noise_variable_75);
            let noise_metadata_schedule_162_e1775: f64 = (noise_metadata_schedule_162_e1773 * noise_variable_67);
            let noise_metadata_schedule_162_e1777: f64 = (noise_metadata_schedule_162_e1775 * noise_variable_86);
            let noise_metadata_schedule_162_e1779: f64 = (noise_metadata_schedule_162_e1777 * noise_variable_86);
            noise_variable_83 = noise_metadata_schedule_162_e1779;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_163_e1782: f64 = (params.p35 * noise_variable_257);
            let noise_metadata_schedule_163_e1784: f64 = (noise_metadata_schedule_163_e1782 * noise_variable_19);
            let noise_metadata_schedule_163_e1786: f64 = (noise_metadata_schedule_163_e1784 * noise_variable_19);
            let noise_metadata_schedule_163_e1788: f64 = (noise_metadata_schedule_163_e1786 * noise_variable_66);
            let noise_metadata_schedule_163_e1790: f64 = (noise_metadata_schedule_163_e1788 * noise_variable_66);
            let noise_metadata_schedule_163_e1792: f64 = (noise_metadata_schedule_163_e1790 * noise_variable_90);
            let noise_metadata_schedule_163_e1795: f64 = (params.p36 - noise_variable_83);
            let noise_metadata_schedule_163_e1796: f64 = (noise_metadata_schedule_163_e1795).exp();
            let noise_metadata_schedule_163_e1797: f64 = (noise_metadata_schedule_163_e1792 * noise_metadata_schedule_163_e1796);
            noise_variable_84 = noise_metadata_schedule_163_e1797;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_164_e1800: f64 = (noise_variable_254 * params.p95);
            let noise_metadata_schedule_164_e1801: f64 = (noise_metadata_schedule_164_e1800).exp();
            noise_variable_255 = noise_metadata_schedule_164_e1801;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_165_e1804: f64 = (params.p13 * noise_variable_255);
            let noise_metadata_schedule_165_e1806: f64 = (noise_metadata_schedule_165_e1804 * noise_variable_27);
            noise_variable_40 = noise_metadata_schedule_165_e1806;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_166_e1809: f64 = (params.p12 * noise_variable_255);
            let noise_metadata_schedule_166_e1811: f64 = (noise_metadata_schedule_166_e1809 * noise_variable_256);
            noise_variable_41 = noise_metadata_schedule_166_e1811;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_172_e1863: f64 = (noise_variable_2 - 300.0);
            noise_variable_100 = noise_metadata_schedule_172_e1863;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_173_e1866: f64 = if noise_variable_2 < 525.0 { 1.0 } else { 0.0 };
            noise_variable_459 = noise_metadata_schedule_173_e1866;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_174_e1882,) = {
    if (noise_variable_459 != 0.0) {
        let noise_metadata_schedule_174_e1872: f64 = (0.00072 * noise_variable_100);
        let noise_metadata_schedule_174_e1873: f64 = (1.0 + noise_metadata_schedule_174_e1872);
        let noise_metadata_schedule_174_e1876: f64 = (1.6e-6 * noise_variable_100);
        let noise_metadata_schedule_174_e1878: f64 = (noise_metadata_schedule_174_e1876 * noise_variable_100);
        let noise_metadata_schedule_174_e1879: f64 = (noise_metadata_schedule_174_e1873 - noise_metadata_schedule_174_e1878);
        let noise_metadata_schedule_174_e1880: f64 = (noise_variable_1 * noise_metadata_schedule_174_e1879);
        (noise_metadata_schedule_174_e1880,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_174_e1882;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_175_e1889,) = {
    if (noise_variable_459 == 0.0) {
        let noise_metadata_schedule_175_e1887: f64 = (noise_variable_1 * 1.081);
        (noise_metadata_schedule_175_e1887,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_175_e1889;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_176_e1893: f64 = (noise_variable_254 * params.p95);
            let noise_metadata_schedule_176_e1894: f64 = (noise_metadata_schedule_176_e1893).exp();
            let noise_metadata_schedule_176_e1895: f64 = (params.p91 * noise_metadata_schedule_176_e1894);
            noise_variable_99 = noise_metadata_schedule_176_e1895;
        }
        if matches!(source_index, 17 | 20 | 22 | 24) {
            let noise_metadata_schedule_177_e1898: f64 = if params.p56 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_460 = noise_metadata_schedule_177_e1898;
        }
        if matches!(source_index, 17 | 20 | 22 | 24) {
            let (noise_metadata_schedule_178_e1904,) = {
    if (noise_variable_460 != 0.0) {
        let noise_metadata_schedule_178_e1902: f64 = (1.0 / noise_variable_32);
        (noise_metadata_schedule_178_e1902,)
    } else {
        (noise_variable_101,)
    }
};
            noise_variable_101 = noise_metadata_schedule_178_e1904;
        }
        if matches!(source_index, 17 | 20 | 22 | 24) {
            let noise_metadata_schedule_179_e1907: f64 = if noise_variable_101 > noise_variable_317 { 1.0 } else { 0.0 };
            noise_variable_461 = noise_metadata_schedule_179_e1907;
        }
        if matches!(source_index, 17 | 20 | 22 | 24) {
            let (noise_metadata_schedule_180_e1913,) = {
    if ((noise_variable_460 != 0.0) && (noise_variable_461 != 0.0)) {
        (noise_variable_317,)
    } else {
        (noise_variable_101,)
    }
};
            noise_variable_101 = noise_metadata_schedule_180_e1913;
        }
        if matches!(source_index, 17 | 20 | 22 | 24) {
            let (noise_metadata_schedule_181_e1918,) = {
    if (noise_variable_460 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_101,)
    }
};
            noise_variable_101 = noise_metadata_schedule_181_e1918;
        }
        if matches!(source_index, 18 | 21) {
            let noise_metadata_schedule_182_e1921: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_462 = noise_metadata_schedule_182_e1921;
        }
        if matches!(source_index, 18 | 21) {
            let (noise_metadata_schedule_183_e1927,) = {
    if (noise_variable_462 != 0.0) {
        let noise_metadata_schedule_183_e1925: f64 = (1.0 / noise_variable_33);
        (noise_metadata_schedule_183_e1925,)
    } else {
        (noise_variable_102,)
    }
};
            noise_variable_102 = noise_metadata_schedule_183_e1927;
        }
        if matches!(source_index, 18 | 21) {
            let noise_metadata_schedule_184_e1930: f64 = if noise_variable_102 > noise_variable_317 { 1.0 } else { 0.0 };
            noise_variable_463 = noise_metadata_schedule_184_e1930;
        }
        if matches!(source_index, 18 | 21) {
            let (noise_metadata_schedule_185_e1936,) = {
    if ((noise_variable_462 != 0.0) && (noise_variable_463 != 0.0)) {
        (noise_variable_317,)
    } else {
        (noise_variable_102,)
    }
};
            noise_variable_102 = noise_metadata_schedule_185_e1936;
        }
        if matches!(source_index, 18 | 21) {
            let (noise_metadata_schedule_186_e1941,) = {
    if (noise_variable_462 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_102,)
    }
};
            noise_variable_102 = noise_metadata_schedule_186_e1941;
        }
        if matches!(source_index, 19 | 23) {
            let noise_metadata_schedule_187_e1944: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_464 = noise_metadata_schedule_187_e1944;
        }
        if matches!(source_index, 19 | 23) {
            let (noise_metadata_schedule_188_e1950,) = {
    if (noise_variable_464 != 0.0) {
        let noise_metadata_schedule_188_e1948: f64 = (1.0 / noise_variable_34);
        (noise_metadata_schedule_188_e1948,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_188_e1950;
        }
        if matches!(source_index, 19 | 23) {
            let noise_metadata_schedule_189_e1953: f64 = if noise_variable_103 > noise_variable_317 { 1.0 } else { 0.0 };
            noise_variable_465 = noise_metadata_schedule_189_e1953;
        }
        if matches!(source_index, 19 | 23) {
            let (noise_metadata_schedule_190_e1959,) = {
    if ((noise_variable_464 != 0.0) && (noise_variable_465 != 0.0)) {
        (noise_variable_317,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_190_e1959;
        }
        if matches!(source_index, 19 | 23) {
            let (noise_metadata_schedule_191_e1964,) = {
    if (noise_variable_464 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_191_e1964;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_192_e1967: f64 = (params.p3 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[6])));
            noise_variable_230 = noise_metadata_schedule_192_e1967;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_193_e1970: f64 = (params.p3 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[7])));
            noise_variable_231 = noise_metadata_schedule_193_e1970;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_194_e1973: f64 = (params.p3 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[3])));
            noise_variable_232 = noise_metadata_schedule_194_e1973;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_195_e1976: f64 = (params.p3 * (ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[3])));
            noise_variable_233 = noise_metadata_schedule_195_e1976;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_196_e1979: f64 = (params.p3 * (ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_234 = noise_metadata_schedule_196_e1979;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_197_e1982: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[7])));
            noise_variable_236 = noise_metadata_schedule_197_e1982;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_199_e1988: f64 = (params.p3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[4])));
            noise_variable_240 = noise_metadata_schedule_199_e1988;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_201_e1994: f64 = (params.p3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[0])));
            noise_variable_244 = noise_metadata_schedule_201_e1994;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_202_e1997: f64 = (params.p3 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[6])));
            noise_variable_238 = noise_metadata_schedule_202_e1997;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_203_e2000: f64 = (params.p3 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
            noise_variable_237 = noise_metadata_schedule_203_e2000;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_204_e2003: f64 = (noise_variable_234 + noise_variable_231);
            let noise_metadata_schedule_204_e2005: f64 = (noise_metadata_schedule_204_e2003 - noise_variable_236);
            let noise_metadata_schedule_204_e2007: f64 = (noise_metadata_schedule_204_e2005 - noise_variable_238);
            noise_variable_235 = noise_metadata_schedule_204_e2007;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_205_e2009: f64 = (-noise_variable_244);
            let noise_metadata_schedule_205_e2011: f64 = (noise_metadata_schedule_205_e2009 + noise_variable_240);
            let noise_metadata_schedule_205_e2013: f64 = (noise_metadata_schedule_205_e2011 + noise_variable_235);
            let noise_metadata_schedule_205_e2015: f64 = (noise_metadata_schedule_205_e2013 - noise_variable_237);
            noise_variable_242 = noise_metadata_schedule_205_e2015;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_206_e2018: f64 = (noise_variable_244 + noise_variable_242);
            noise_variable_241 = noise_metadata_schedule_206_e2018;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_207_e2021: f64 = (noise_variable_231 * noise_variable_8);
            let noise_metadata_schedule_207_e2023: f64 = if noise_metadata_schedule_207_e2021 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_466 = noise_metadata_schedule_207_e2023;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_208_e2030,) = {
    if (noise_variable_466 != 0.0) {
        let noise_metadata_schedule_208_e2027: f64 = (noise_variable_231 * noise_variable_8);
        let noise_metadata_schedule_208_e2028: f64 = (noise_metadata_schedule_208_e2027).exp();
        (noise_metadata_schedule_208_e2028,)
    } else {
        (noise_variable_245,)
    }
};
            noise_variable_245 = noise_metadata_schedule_208_e2030;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_209_e2036,) = {
    if (noise_variable_466 == 0.0) {
        let noise_metadata_schedule_209_e2034: f64 = (params.p134).exp();
        (noise_metadata_schedule_209_e2034,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_209_e2036;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_210_e2049,) = {
    if (noise_variable_466 == 0.0) {
        let noise_metadata_schedule_210_e2043: f64 = (noise_variable_231 * noise_variable_8);
        let noise_metadata_schedule_210_e2045: f64 = (noise_metadata_schedule_210_e2043 - params.p134);
        let noise_metadata_schedule_210_e2046: f64 = (1.0 + noise_metadata_schedule_210_e2045);
        let noise_metadata_schedule_210_e2047: f64 = (noise_variable_275 * noise_metadata_schedule_210_e2046);
        (noise_metadata_schedule_210_e2047,)
    } else {
        (noise_variable_245,)
    }
};
            noise_variable_245 = noise_metadata_schedule_210_e2049;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_211_e2052: f64 = (noise_variable_232 * noise_variable_8);
            let noise_metadata_schedule_211_e2054: f64 = (noise_metadata_schedule_211_e2052 / noise_variable_48);
            let noise_metadata_schedule_211_e2056: f64 = if noise_metadata_schedule_211_e2054 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_467 = noise_metadata_schedule_211_e2056;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_212_e2065,) = {
    if (noise_variable_467 != 0.0) {
        let noise_metadata_schedule_212_e2060: f64 = (noise_variable_232 * noise_variable_8);
        let noise_metadata_schedule_212_e2062: f64 = (noise_metadata_schedule_212_e2060 / noise_variable_48);
        let noise_metadata_schedule_212_e2063: f64 = (noise_metadata_schedule_212_e2062).exp();
        (noise_metadata_schedule_212_e2063,)
    } else {
        (noise_variable_246,)
    }
};
            noise_variable_246 = noise_metadata_schedule_212_e2065;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_213_e2071,) = {
    if (noise_variable_467 == 0.0) {
        let noise_metadata_schedule_213_e2069: f64 = (params.p134).exp();
        (noise_metadata_schedule_213_e2069,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_213_e2071;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_214_e2086,) = {
    if (noise_variable_467 == 0.0) {
        let noise_metadata_schedule_214_e2078: f64 = (noise_variable_232 * noise_variable_8);
        let noise_metadata_schedule_214_e2080: f64 = (noise_metadata_schedule_214_e2078 / noise_variable_48);
        let noise_metadata_schedule_214_e2082: f64 = (noise_metadata_schedule_214_e2080 - params.p134);
        let noise_metadata_schedule_214_e2083: f64 = (1.0 + noise_metadata_schedule_214_e2082);
        let noise_metadata_schedule_214_e2084: f64 = (noise_variable_275 * noise_metadata_schedule_214_e2083);
        (noise_metadata_schedule_214_e2084,)
    } else {
        (noise_variable_246,)
    }
};
            noise_variable_246 = noise_metadata_schedule_214_e2086;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_215_e2089: f64 = (noise_variable_235 * noise_variable_8);
            let noise_metadata_schedule_215_e2091: f64 = if noise_metadata_schedule_215_e2089 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_468 = noise_metadata_schedule_215_e2091;
        }
        if matches!(source_index, 11 | 12) {
            let (noise_metadata_schedule_216_e2098,) = {
    if (noise_variable_468 != 0.0) {
        let noise_metadata_schedule_216_e2095: f64 = (noise_variable_235 * noise_variable_8);
        let noise_metadata_schedule_216_e2096: f64 = (noise_metadata_schedule_216_e2095).exp();
        (noise_metadata_schedule_216_e2096,)
    } else {
        (noise_variable_248,)
    }
};
            noise_variable_248 = noise_metadata_schedule_216_e2098;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_217_e2104,) = {
    if (noise_variable_468 == 0.0) {
        let noise_metadata_schedule_217_e2102: f64 = (params.p134).exp();
        (noise_metadata_schedule_217_e2102,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_217_e2104;
        }
        if matches!(source_index, 11 | 12) {
            let (noise_metadata_schedule_218_e2117,) = {
    if (noise_variable_468 == 0.0) {
        let noise_metadata_schedule_218_e2111: f64 = (noise_variable_235 * noise_variable_8);
        let noise_metadata_schedule_218_e2113: f64 = (noise_metadata_schedule_218_e2111 - params.p134);
        let noise_metadata_schedule_218_e2114: f64 = (1.0 + noise_metadata_schedule_218_e2113);
        let noise_metadata_schedule_218_e2115: f64 = (noise_variable_275 * noise_metadata_schedule_218_e2114);
        (noise_metadata_schedule_218_e2115,)
    } else {
        (noise_variable_248,)
    }
};
            noise_variable_248 = noise_metadata_schedule_218_e2117;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_219_e2120: f64 = (noise_variable_234 * noise_variable_8);
            let noise_metadata_schedule_219_e2122: f64 = if noise_metadata_schedule_219_e2120 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_469 = noise_metadata_schedule_219_e2122;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_220_e2129,) = {
    if (noise_variable_469 != 0.0) {
        let noise_metadata_schedule_220_e2126: f64 = (noise_variable_234 * noise_variable_8);
        let noise_metadata_schedule_220_e2127: f64 = (noise_metadata_schedule_220_e2126).exp();
        (noise_metadata_schedule_220_e2127,)
    } else {
        (noise_variable_247,)
    }
};
            noise_variable_247 = noise_metadata_schedule_220_e2129;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_221_e2135,) = {
    if (noise_variable_469 == 0.0) {
        let noise_metadata_schedule_221_e2133: f64 = (params.p134).exp();
        (noise_metadata_schedule_221_e2133,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_221_e2135;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_222_e2148,) = {
    if (noise_variable_469 == 0.0) {
        let noise_metadata_schedule_222_e2142: f64 = (noise_variable_234 * noise_variable_8);
        let noise_metadata_schedule_222_e2144: f64 = (noise_metadata_schedule_222_e2142 - params.p134);
        let noise_metadata_schedule_222_e2145: f64 = (1.0 + noise_metadata_schedule_222_e2144);
        let noise_metadata_schedule_222_e2146: f64 = (noise_variable_275 * noise_metadata_schedule_222_e2145);
        (noise_metadata_schedule_222_e2146,)
    } else {
        (noise_variable_247,)
    }
};
            noise_variable_247 = noise_metadata_schedule_222_e2148;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_223_e2151: f64 = (noise_variable_241 * noise_variable_8);
            let noise_metadata_schedule_223_e2153: f64 = if noise_metadata_schedule_223_e2151 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_470 = noise_metadata_schedule_223_e2153;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_224_e2160,) = {
    if (noise_variable_470 != 0.0) {
        let noise_metadata_schedule_224_e2157: f64 = (noise_variable_241 * noise_variable_8);
        let noise_metadata_schedule_224_e2158: f64 = (noise_metadata_schedule_224_e2157).exp();
        (noise_metadata_schedule_224_e2158,)
    } else {
        (noise_variable_249,)
    }
};
            noise_variable_249 = noise_metadata_schedule_224_e2160;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_225_e2166,) = {
    if (noise_variable_470 == 0.0) {
        let noise_metadata_schedule_225_e2164: f64 = (params.p134).exp();
        (noise_metadata_schedule_225_e2164,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_225_e2166;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_226_e2179,) = {
    if (noise_variable_470 == 0.0) {
        let noise_metadata_schedule_226_e2173: f64 = (noise_variable_241 * noise_variable_8);
        let noise_metadata_schedule_226_e2175: f64 = (noise_metadata_schedule_226_e2173 - params.p134);
        let noise_metadata_schedule_226_e2176: f64 = (1.0 + noise_metadata_schedule_226_e2175);
        let noise_metadata_schedule_226_e2177: f64 = (noise_variable_275 * noise_metadata_schedule_226_e2176);
        (noise_metadata_schedule_226_e2177,)
    } else {
        (noise_variable_249,)
    }
};
            noise_variable_249 = noise_metadata_schedule_226_e2179;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_227_e2182: f64 = (noise_variable_241 - noise_variable_16);
            let noise_metadata_schedule_227_e2184: f64 = (noise_metadata_schedule_227_e2182 * noise_variable_8);
            let noise_metadata_schedule_227_e2186: f64 = if noise_metadata_schedule_227_e2184 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_471 = noise_metadata_schedule_227_e2186;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_229_e2201,) = {
    if (noise_variable_471 == 0.0) {
        let noise_metadata_schedule_229_e2199: f64 = (params.p134).exp();
        (noise_metadata_schedule_229_e2199,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_229_e2201;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_231_e2219: f64 = (noise_variable_235 - noise_variable_16);
            let noise_metadata_schedule_231_e2221: f64 = (noise_metadata_schedule_231_e2219 * noise_variable_8);
            let noise_metadata_schedule_231_e2223: f64 = if noise_metadata_schedule_231_e2221 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_472 = noise_metadata_schedule_231_e2223;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_233_e2238,) = {
    if (noise_variable_472 == 0.0) {
        let noise_metadata_schedule_233_e2236: f64 = (params.p134).exp();
        (noise_metadata_schedule_233_e2236,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_233_e2238;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_235_e2256: f64 = (noise_variable_231 - noise_variable_16);
            let noise_metadata_schedule_235_e2258: f64 = (noise_metadata_schedule_235_e2256 * noise_variable_8);
            let noise_metadata_schedule_235_e2260: f64 = if noise_metadata_schedule_235_e2258 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_473 = noise_metadata_schedule_235_e2260;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_236_e2269,) = {
    if (noise_variable_473 != 0.0) {
        let noise_metadata_schedule_236_e2264: f64 = (noise_variable_231 - noise_variable_16);
        let noise_metadata_schedule_236_e2266: f64 = (noise_metadata_schedule_236_e2264 * noise_variable_8);
        let noise_metadata_schedule_236_e2267: f64 = (noise_metadata_schedule_236_e2266).exp();
        (noise_metadata_schedule_236_e2267,)
    } else {
        (noise_variable_251,)
    }
};
            noise_variable_251 = noise_metadata_schedule_236_e2269;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_237_e2275,) = {
    if (noise_variable_473 == 0.0) {
        let noise_metadata_schedule_237_e2273: f64 = (params.p134).exp();
        (noise_metadata_schedule_237_e2273,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_237_e2275;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_238_e2290,) = {
    if (noise_variable_473 == 0.0) {
        let noise_metadata_schedule_238_e2282: f64 = (noise_variable_231 - noise_variable_16);
        let noise_metadata_schedule_238_e2284: f64 = (noise_metadata_schedule_238_e2282 * noise_variable_8);
        let noise_metadata_schedule_238_e2286: f64 = (noise_metadata_schedule_238_e2284 - params.p134);
        let noise_metadata_schedule_238_e2287: f64 = (1.0 + noise_metadata_schedule_238_e2286);
        let noise_metadata_schedule_238_e2288: f64 = (noise_variable_275 * noise_metadata_schedule_238_e2287);
        (noise_metadata_schedule_238_e2288,)
    } else {
        (noise_variable_251,)
    }
};
            noise_variable_251 = noise_metadata_schedule_238_e2290;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_239_e2293: f64 = (noise_variable_230 - noise_variable_16);
            let noise_metadata_schedule_239_e2295: f64 = (noise_metadata_schedule_239_e2293 * noise_variable_8);
            let noise_metadata_schedule_239_e2297: f64 = if noise_metadata_schedule_239_e2295 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_474 = noise_metadata_schedule_239_e2297;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_240_e2306,) = {
    if (noise_variable_474 != 0.0) {
        let noise_metadata_schedule_240_e2301: f64 = (noise_variable_230 - noise_variable_16);
        let noise_metadata_schedule_240_e2303: f64 = (noise_metadata_schedule_240_e2301 * noise_variable_8);
        let noise_metadata_schedule_240_e2304: f64 = (noise_metadata_schedule_240_e2303).exp();
        (noise_metadata_schedule_240_e2304,)
    } else {
        (noise_variable_253,)
    }
};
            noise_variable_253 = noise_metadata_schedule_240_e2306;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_241_e2312,) = {
    if (noise_variable_474 == 0.0) {
        let noise_metadata_schedule_241_e2310: f64 = (params.p134).exp();
        (noise_metadata_schedule_241_e2310,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_241_e2312;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_242_e2327,) = {
    if (noise_variable_474 == 0.0) {
        let noise_metadata_schedule_242_e2319: f64 = (noise_variable_230 - noise_variable_16);
        let noise_metadata_schedule_242_e2321: f64 = (noise_metadata_schedule_242_e2319 * noise_variable_8);
        let noise_metadata_schedule_242_e2323: f64 = (noise_metadata_schedule_242_e2321 - params.p134);
        let noise_metadata_schedule_242_e2324: f64 = (1.0 + noise_metadata_schedule_242_e2323);
        let noise_metadata_schedule_242_e2325: f64 = (noise_variable_275 * noise_metadata_schedule_242_e2324);
        (noise_metadata_schedule_242_e2325,)
    } else {
        (noise_variable_253,)
    }
};
            noise_variable_253 = noise_metadata_schedule_242_e2327;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_243_e2331: f64 = (4.0 * noise_variable_251);
            let noise_metadata_schedule_243_e2332: f64 = (1.0 + noise_metadata_schedule_243_e2331);
            let noise_metadata_schedule_243_e2333: f64 = (noise_metadata_schedule_243_e2332).sqrt();
            noise_variable_104 = noise_metadata_schedule_243_e2333;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_244_e2337: f64 = (4.0 * noise_variable_253);
            let noise_metadata_schedule_244_e2338: f64 = (1.0 + noise_metadata_schedule_244_e2337);
            let noise_metadata_schedule_244_e2339: f64 = (noise_metadata_schedule_244_e2338).sqrt();
            noise_variable_105 = noise_metadata_schedule_244_e2339;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_245_e2342: f64 = (2.0 * noise_variable_253);
            let noise_metadata_schedule_245_e2345: f64 = (1.0 + noise_variable_105);
            let noise_metadata_schedule_245_e2346: f64 = (noise_metadata_schedule_245_e2342 / noise_metadata_schedule_245_e2345);
            noise_variable_106 = noise_metadata_schedule_245_e2346;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_246_e2349: f64 = if noise_variable_106 < params.p136 { 1.0 } else { 0.0 };
            noise_variable_475 = noise_metadata_schedule_246_e2349;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_247_e2353,) = {
    if (noise_variable_475 != 0.0) {
        (params.p136,)
    } else {
        (noise_variable_106,)
    }
};
            noise_variable_106 = noise_metadata_schedule_247_e2353;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_248_e2357: f64 = (noise_variable_104 - noise_variable_105);
            let noise_metadata_schedule_248_e2360: f64 = (noise_variable_104 + 1.0);
            let noise_metadata_schedule_248_e2363: f64 = (noise_variable_105 + 1.0);
            let noise_metadata_schedule_248_e2364: f64 = (noise_metadata_schedule_248_e2360 / noise_metadata_schedule_248_e2363);
            let noise_metadata_schedule_248_e2365: f64 = (noise_metadata_schedule_248_e2364).ln();
            let noise_metadata_schedule_248_e2366: f64 = (noise_metadata_schedule_248_e2357 - noise_metadata_schedule_248_e2365);
            let noise_metadata_schedule_248_e2367: f64 = (noise_variable_6 * noise_metadata_schedule_248_e2366);
            noise_variable_107 = noise_metadata_schedule_248_e2367;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_249_e2370: f64 = (noise_variable_107 + noise_variable_236);
            let noise_metadata_schedule_249_e2372: f64 = (noise_metadata_schedule_249_e2370 / noise_variable_31);
            noise_variable_108 = noise_metadata_schedule_249_e2372;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_250_e2375: f64 = if noise_variable_108 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_476 = noise_metadata_schedule_250_e2375;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_251_e2378: f64 = if noise_variable_230 < 100.0 { 1.0 } else { 0.0 };
            noise_variable_477 = noise_metadata_schedule_251_e2378;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_252_e2384,) = {
    if ((noise_variable_476 != 0.0) && (noise_variable_477 != 0.0)) {
        (noise_variable_230,)
    } else {
        (noise_variable_277,)
    }
};
            noise_variable_277 = noise_metadata_schedule_252_e2384;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_253_e2398,) = {
    if ((noise_variable_476 != 0.0) && (noise_variable_477 == 0.0)) {
        let noise_metadata_schedule_253_e2393: f64 = (noise_variable_230 - 100.0);
        let noise_metadata_schedule_253_e2394: f64 = (1.0 + noise_metadata_schedule_253_e2393);
        let noise_metadata_schedule_253_e2395: f64 = (noise_metadata_schedule_253_e2394).ln();
        let noise_metadata_schedule_253_e2396: f64 = (100.0 + noise_metadata_schedule_253_e2395);
        (noise_metadata_schedule_253_e2396,)
    } else {
        (noise_variable_277,)
    }
};
            noise_variable_277 = noise_metadata_schedule_253_e2398;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_254_e2419,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_254_e2403: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_254_e2406: f64 = (0.5 * noise_variable_108);
        let noise_metadata_schedule_254_e2408: f64 = (noise_metadata_schedule_254_e2406 * noise_variable_31);
        let noise_metadata_schedule_254_e2410: f64 = (noise_metadata_schedule_254_e2408 * noise_variable_8);
        let noise_metadata_schedule_254_e2412: f64 = (noise_metadata_schedule_254_e2410 + 1.0);
        let noise_metadata_schedule_254_e2413: f64 = (noise_metadata_schedule_254_e2412).ln();
        let noise_metadata_schedule_254_e2414: f64 = (noise_metadata_schedule_254_e2403 * noise_metadata_schedule_254_e2413);
        let noise_metadata_schedule_254_e2415: f64 = (noise_variable_16 + noise_metadata_schedule_254_e2414);
        let noise_metadata_schedule_254_e2417: f64 = (noise_metadata_schedule_254_e2415 - noise_variable_277);
        (noise_metadata_schedule_254_e2417,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_254_e2419;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_255_e2425,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_255_e2423: f64 = (0.2 * noise_variable_16);
        (noise_metadata_schedule_255_e2423,)
    } else {
        (noise_variable_272,)
    }
};
            noise_variable_272 = noise_metadata_schedule_255_e2425;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_256_e2431,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_256_e2429: f64 = (noise_variable_272 * noise_variable_272);
        (noise_metadata_schedule_256_e2429,)
    } else {
        (noise_variable_261,)
    }
};
            noise_variable_261 = noise_metadata_schedule_256_e2431;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_257_e2437,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_257_e2435: f64 = (noise_variable_109 * noise_variable_109);
        (noise_metadata_schedule_257_e2435,)
    } else {
        (noise_variable_262,)
    }
};
            noise_variable_262 = noise_metadata_schedule_257_e2437;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_258_e2440: f64 = if noise_variable_109 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_478 = noise_metadata_schedule_258_e2440;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_259_e2455,) = {
    if ((noise_variable_476 != 0.0) && (noise_variable_478 != 0.0)) {
        let noise_metadata_schedule_259_e2446: f64 = (0.5 * noise_variable_261);
        let noise_metadata_schedule_259_e2449: f64 = (noise_variable_262 + noise_variable_261);
        let noise_metadata_schedule_259_e2450: f64 = (noise_metadata_schedule_259_e2449).sqrt();
        let noise_metadata_schedule_259_e2452: f64 = (noise_metadata_schedule_259_e2450 - noise_variable_109);
        let noise_metadata_schedule_259_e2453: f64 = (noise_metadata_schedule_259_e2446 / noise_metadata_schedule_259_e2452);
        (noise_metadata_schedule_259_e2453,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_259_e2455;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_260_e2469,) = {
    if ((noise_variable_476 != 0.0) && (noise_variable_478 == 0.0)) {
        let noise_metadata_schedule_260_e2463: f64 = (noise_variable_262 + noise_variable_261);
        let noise_metadata_schedule_260_e2464: f64 = (noise_metadata_schedule_260_e2463).sqrt();
        let noise_metadata_schedule_260_e2466: f64 = (noise_metadata_schedule_260_e2464 + noise_variable_109);
        let noise_metadata_schedule_260_e2467: f64 = (0.5 * noise_metadata_schedule_260_e2466);
        (noise_metadata_schedule_260_e2467,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_260_e2469;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_261_e2487,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_261_e2475: f64 = (params.p61 * params.p60);
        let noise_metadata_schedule_261_e2476: f64 = (noise_variable_110 + noise_metadata_schedule_261_e2475);
        let noise_metadata_schedule_261_e2477: f64 = (noise_variable_110 * noise_metadata_schedule_261_e2476);
        let noise_metadata_schedule_261_e2482: f64 = (params.p61 * noise_variable_31);
        let noise_metadata_schedule_261_e2483: f64 = (noise_variable_110 + noise_metadata_schedule_261_e2482);
        let noise_metadata_schedule_261_e2484: f64 = (params.p60 * noise_metadata_schedule_261_e2483);
        let noise_metadata_schedule_261_e2485: f64 = (noise_metadata_schedule_261_e2477 / noise_metadata_schedule_261_e2484);
        (noise_metadata_schedule_261_e2485,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_261_e2487;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_262_e2493,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_262_e2491: f64 = (noise_variable_108 / noise_variable_111);
        (noise_metadata_schedule_262_e2491,)
    } else {
        (noise_variable_265,)
    }
};
            noise_variable_265 = noise_metadata_schedule_262_e2493;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_263_e2501,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_263_e2497: f64 = (noise_variable_265 - 1.0);
        let noise_metadata_schedule_263_e2499: f64 = (noise_metadata_schedule_263_e2497 / params.p62);
        (noise_metadata_schedule_263_e2499,)
    } else {
        (noise_variable_259,)
    }
};
            noise_variable_259 = noise_metadata_schedule_263_e2501;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_264_e2504: f64 = if noise_variable_265 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_479 = noise_metadata_schedule_264_e2504;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_265_e2518,) = {
    if ((noise_variable_476 != 0.0) && (noise_variable_479 != 0.0)) {
        let noise_metadata_schedule_265_e2512: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_265_e2513: f64 = (1.0 + noise_metadata_schedule_265_e2512);
        let noise_metadata_schedule_265_e2514: f64 = (noise_metadata_schedule_265_e2513).ln();
        let noise_metadata_schedule_265_e2515: f64 = (params.p62 * noise_metadata_schedule_265_e2514);
        let noise_metadata_schedule_265_e2516: f64 = (1.0 + noise_metadata_schedule_265_e2515);
        (noise_metadata_schedule_265_e2516,)
    } else {
        (noise_variable_263,)
    }
};
            noise_variable_263 = noise_metadata_schedule_265_e2518;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_266_e2534,) = {
    if ((noise_variable_476 != 0.0) && (noise_variable_479 == 0.0)) {
        let noise_metadata_schedule_266_e2527: f64 = (-noise_variable_259);
        let noise_metadata_schedule_266_e2528: f64 = (noise_metadata_schedule_266_e2527).exp();
        let noise_metadata_schedule_266_e2529: f64 = (1.0 + noise_metadata_schedule_266_e2528);
        let noise_metadata_schedule_266_e2530: f64 = (noise_metadata_schedule_266_e2529).ln();
        let noise_metadata_schedule_266_e2531: f64 = (params.p62 * noise_metadata_schedule_266_e2530);
        let noise_metadata_schedule_266_e2532: f64 = (noise_variable_265 + noise_metadata_schedule_266_e2531);
        (noise_metadata_schedule_266_e2532,)
    } else {
        (noise_variable_263,)
    }
};
            noise_variable_263 = noise_metadata_schedule_266_e2534;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_267_e2551,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_267_e2541: f64 = (-1.0);
        let noise_metadata_schedule_267_e2543: f64 = (noise_metadata_schedule_267_e2541 / params.p62);
        let noise_metadata_schedule_267_e2544: f64 = (noise_metadata_schedule_267_e2543).exp();
        let noise_metadata_schedule_267_e2545: f64 = (1.0 + noise_metadata_schedule_267_e2544);
        let noise_metadata_schedule_267_e2546: f64 = (noise_metadata_schedule_267_e2545).ln();
        let noise_metadata_schedule_267_e2547: f64 = (params.p62 * noise_metadata_schedule_267_e2546);
        let noise_metadata_schedule_267_e2548: f64 = (1.0 + noise_metadata_schedule_267_e2547);
        let noise_metadata_schedule_267_e2549: f64 = (noise_variable_263 / noise_metadata_schedule_267_e2548);
        (noise_metadata_schedule_267_e2549,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_267_e2551;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_268_e2559,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_268_e2556: f64 = (params.p61 * params.p60);
        let noise_metadata_schedule_268_e2557: f64 = (noise_variable_110 / noise_metadata_schedule_268_e2556);
        (noise_metadata_schedule_268_e2557,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_268_e2559;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_269_e2584,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_269_e2565: f64 = (4.0 * noise_variable_112);
        let noise_metadata_schedule_269_e2567: f64 = (noise_metadata_schedule_269_e2565 * noise_variable_113);
        let noise_metadata_schedule_269_e2570: f64 = (1.0 + noise_variable_113);
        let noise_metadata_schedule_269_e2571: f64 = (noise_metadata_schedule_269_e2567 * noise_metadata_schedule_269_e2570);
        let noise_metadata_schedule_269_e2572: f64 = (1.0 + noise_metadata_schedule_269_e2571);
        let noise_metadata_schedule_269_e2573: f64 = (noise_metadata_schedule_269_e2572).sqrt();
        let noise_metadata_schedule_269_e2574: f64 = (1.0 + noise_metadata_schedule_269_e2573);
        let noise_metadata_schedule_269_e2577: f64 = (2.0 * noise_variable_112);
        let noise_metadata_schedule_269_e2580: f64 = (1.0 + noise_variable_113);
        let noise_metadata_schedule_269_e2581: f64 = (noise_metadata_schedule_269_e2577 * noise_metadata_schedule_269_e2580);
        let noise_metadata_schedule_269_e2582: f64 = (noise_metadata_schedule_269_e2574 / noise_metadata_schedule_269_e2581);
        (noise_metadata_schedule_269_e2582,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_269_e2584;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_270_e2600,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_270_e2588: f64 = (1.0 - noise_variable_114);
        let noise_metadata_schedule_270_e2591: f64 = (noise_variable_106 * noise_variable_114);
        let noise_metadata_schedule_270_e2592: f64 = (noise_metadata_schedule_270_e2588 + noise_metadata_schedule_270_e2591);
        let noise_metadata_schedule_270_e2596: f64 = (noise_variable_106 * noise_variable_114);
        let noise_metadata_schedule_270_e2597: f64 = (1.0 + noise_metadata_schedule_270_e2596);
        let noise_metadata_schedule_270_e2598: f64 = (noise_metadata_schedule_270_e2592 / noise_metadata_schedule_270_e2597);
        (noise_metadata_schedule_270_e2598,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_270_e2600;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_271_e2612,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_271_e2604: f64 = (0.5 * noise_variable_108);
        let noise_metadata_schedule_271_e2606: f64 = (noise_metadata_schedule_271_e2604 * noise_variable_31);
        let noise_metadata_schedule_271_e2608: f64 = (noise_metadata_schedule_271_e2606 * noise_variable_115);
        let noise_metadata_schedule_271_e2610: f64 = (noise_metadata_schedule_271_e2608 * noise_variable_8);
        (noise_metadata_schedule_271_e2610,)
    } else {
        (noise_variable_117,)
    }
};
            noise_variable_117 = noise_metadata_schedule_271_e2612;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_272_e2626,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_272_e2616: f64 = (2.0 * noise_variable_117);
        let noise_metadata_schedule_272_e2620: f64 = (noise_variable_106 + noise_variable_117);
        let noise_metadata_schedule_272_e2622: f64 = (noise_metadata_schedule_272_e2620 + 1.0);
        let noise_metadata_schedule_272_e2623: f64 = (noise_variable_106 * noise_metadata_schedule_272_e2622);
        let noise_metadata_schedule_272_e2624: f64 = (noise_metadata_schedule_272_e2616 + noise_metadata_schedule_272_e2623);
        (noise_metadata_schedule_272_e2624,)
    } else {
        (noise_variable_266,)
    }
};
            noise_variable_266 = noise_metadata_schedule_272_e2626;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_273_e2634,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_273_e2631: f64 = (noise_variable_117 - 1.0);
        let noise_metadata_schedule_273_e2632: f64 = (0.5 * noise_metadata_schedule_273_e2631);
        (noise_metadata_schedule_273_e2632,)
    } else {
        (noise_variable_118,)
    }
};
            noise_variable_118 = noise_metadata_schedule_273_e2634;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_274_e2642,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_274_e2638: f64 = (noise_variable_118 * noise_variable_118);
        let noise_metadata_schedule_274_e2640: f64 = (noise_metadata_schedule_274_e2638 + noise_variable_266);
        (noise_metadata_schedule_274_e2640,)
    } else {
        (noise_variable_260,)
    }
};
            noise_variable_260 = noise_metadata_schedule_274_e2642;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_275_e2645: f64 = if noise_variable_117 >= 1.0 { 1.0 } else { 0.0 };
            noise_variable_480 = noise_metadata_schedule_275_e2645;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_276_e2654,) = {
    if ((noise_variable_476 != 0.0) && (noise_variable_480 != 0.0)) {
        let noise_metadata_schedule_276_e2651: f64 = (noise_variable_260).sqrt();
        let noise_metadata_schedule_276_e2652: f64 = (noise_variable_118 + noise_metadata_schedule_276_e2651);
        (noise_metadata_schedule_276_e2652,)
    } else {
        (noise_variable_119,)
    }
};
            noise_variable_119 = noise_metadata_schedule_276_e2654;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_277_e2666,) = {
    if ((noise_variable_476 != 0.0) && (noise_variable_480 == 0.0)) {
        let noise_metadata_schedule_277_e2661: f64 = (noise_variable_260).sqrt();
        let noise_metadata_schedule_277_e2663: f64 = (noise_metadata_schedule_277_e2661 - noise_variable_118);
        let noise_metadata_schedule_277_e2664: f64 = (noise_variable_266 / noise_metadata_schedule_277_e2663);
        (noise_metadata_schedule_277_e2664,)
    } else {
        (noise_variable_119,)
    }
};
            noise_variable_119 = noise_metadata_schedule_277_e2666;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_278_e2669: f64 = if noise_variable_119 < params.p135 { 1.0 } else { 0.0 };
            noise_variable_481 = noise_metadata_schedule_278_e2669;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_279_e2675,) = {
    if ((noise_variable_476 != 0.0) && (noise_variable_481 != 0.0)) {
        (params.p135,)
    } else {
        (noise_variable_119,)
    }
};
            noise_variable_119 = noise_metadata_schedule_279_e2675;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_280_e2688,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_280_e2680: f64 = (noise_variable_119 + 1.0);
        let noise_metadata_schedule_280_e2681: f64 = (noise_variable_119 * noise_metadata_schedule_280_e2680);
        let noise_metadata_schedule_280_e2684: f64 = (noise_variable_16 * noise_variable_8);
        let noise_metadata_schedule_280_e2685: f64 = (noise_metadata_schedule_280_e2684).exp();
        let noise_metadata_schedule_280_e2686: f64 = (noise_metadata_schedule_280_e2681 * noise_metadata_schedule_280_e2685);
        (noise_metadata_schedule_280_e2686,)
    } else {
        (noise_variable_121,)
    }
};
            noise_variable_121 = noise_metadata_schedule_280_e2688;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_281_e2698,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_281_e2692: f64 = (0.5 * params.p60);
        let noise_metadata_schedule_281_e2695: f64 = (noise_variable_108 - params.p61);
        let noise_metadata_schedule_281_e2696: f64 = (noise_metadata_schedule_281_e2692 * noise_metadata_schedule_281_e2695);
        (noise_metadata_schedule_281_e2696,)
    } else {
        (noise_variable_123,)
    }
};
            noise_variable_123 = noise_metadata_schedule_281_e2698;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_282_e2708,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_282_e2702: f64 = (params.p60 * noise_variable_31);
        let noise_metadata_schedule_282_e2704: f64 = (noise_metadata_schedule_282_e2702 * params.p61);
        let noise_metadata_schedule_282_e2706: f64 = (noise_metadata_schedule_282_e2704 * noise_variable_108);
        (noise_metadata_schedule_282_e2706,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_282_e2708;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_283_e2719,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_283_e2713: f64 = (noise_variable_123 * noise_variable_123);
        let noise_metadata_schedule_283_e2715: f64 = (noise_metadata_schedule_283_e2713 + noise_variable_124);
        let noise_metadata_schedule_283_e2716: f64 = (noise_metadata_schedule_283_e2715).sqrt();
        let noise_metadata_schedule_283_e2717: f64 = (noise_variable_123 + noise_metadata_schedule_283_e2716);
        (noise_metadata_schedule_283_e2717,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_283_e2719;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_284_e2722: f64 = if params.p72 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_482 = noise_metadata_schedule_284_e2722;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_285_e2730,) = {
    if ((noise_variable_476 != 0.0) && (noise_variable_482 != 0.0)) {
        let noise_metadata_schedule_285_e2728: f64 = (noise_variable_17 * 0.1);
        (noise_metadata_schedule_285_e2728,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_285_e2730;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_286_e2747,) = {
    if ((noise_variable_476 != 0.0) && (noise_variable_482 == 0.0)) {
        let noise_metadata_schedule_286_e2739: f64 = (2.0 * noise_variable_108);
        let noise_metadata_schedule_286_e2742: f64 = (noise_variable_108 + noise_variable_111);
        let noise_metadata_schedule_286_e2743: f64 = (noise_metadata_schedule_286_e2739 / noise_metadata_schedule_286_e2742);
        let noise_metadata_schedule_286_e2744: f64 = (0.1 + noise_metadata_schedule_286_e2743);
        let noise_metadata_schedule_286_e2745: f64 = (noise_variable_17 * noise_metadata_schedule_286_e2744);
        (noise_metadata_schedule_286_e2745,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_286_e2747;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_287_e2757,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_287_e2751: f64 = (params.p61 * noise_variable_108);
        let noise_metadata_schedule_287_e2754: f64 = (params.p61 + noise_variable_108);
        let noise_metadata_schedule_287_e2755: f64 = (noise_metadata_schedule_287_e2751 / noise_metadata_schedule_287_e2754);
        (noise_metadata_schedule_287_e2755,)
    } else {
        (noise_variable_127,)
    }
};
            noise_variable_127 = noise_metadata_schedule_287_e2757;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_288_e2765,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_288_e2762: f64 = (params.p61 + noise_variable_108);
        let noise_metadata_schedule_288_e2763: f64 = (params.p61 / noise_metadata_schedule_288_e2762);
        (noise_metadata_schedule_288_e2763,)
    } else {
        (noise_variable_199,)
    }
};
            noise_variable_199 = noise_metadata_schedule_288_e2765;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_290_e2781,) = {
    if (noise_variable_476 == 0.0) {
        let noise_metadata_schedule_290_e2775: f64 = (2.0 * noise_variable_251);
        let noise_metadata_schedule_290_e2778: f64 = (1.0 + noise_variable_104);
        let noise_metadata_schedule_290_e2779: f64 = (noise_metadata_schedule_290_e2775 / noise_metadata_schedule_290_e2778);
        (noise_metadata_schedule_290_e2779,)
    } else {
        (noise_variable_119,)
    }
};
            noise_variable_119 = noise_metadata_schedule_290_e2781;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_291_e2786,) = {
    if (noise_variable_476 == 0.0) {
        (noise_variable_245,)
    } else {
        (noise_variable_121,)
    }
};
            noise_variable_121 = noise_metadata_schedule_291_e2786;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_292_e2788: f64 = (noise_variable_236).abs();
            let noise_metadata_schedule_292_e2791: f64 = (1e-5 * noise_variable_6);
            let noise_metadata_schedule_292_e2794: f64 = (noise_variable_107).abs();
            let noise_metadata_schedule_292_e2797: f64 = (1e-40 * noise_variable_6);
            let noise_metadata_schedule_292_e2800: f64 = (noise_variable_104 + noise_variable_105);
            let noise_metadata_schedule_292_e2801: f64 = (noise_metadata_schedule_292_e2797 * noise_metadata_schedule_292_e2800);
            let noise_metadata_schedule_292_e2803: f64 = if ((noise_metadata_schedule_292_e2788 < noise_metadata_schedule_292_e2791) || (noise_metadata_schedule_292_e2794 < noise_metadata_schedule_292_e2801)) { 1.0 } else { 0.0 };
            noise_variable_483 = noise_metadata_schedule_292_e2803;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_293_e2814,) = {
    if ((noise_variable_476 == 0.0) && (noise_variable_483 != 0.0)) {
        let noise_metadata_schedule_293_e2811: f64 = (noise_variable_119 + noise_variable_106);
        let noise_metadata_schedule_293_e2812: f64 = (0.5 * noise_metadata_schedule_293_e2811);
        (noise_metadata_schedule_293_e2812,)
    } else {
        (noise_variable_128,)
    }
};
            noise_variable_128 = noise_metadata_schedule_293_e2814;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_294_e2825,) = {
    if ((noise_variable_476 == 0.0) && (noise_variable_483 != 0.0)) {
        let noise_metadata_schedule_294_e2822: f64 = (noise_variable_128 + 1.0);
        let noise_metadata_schedule_294_e2823: f64 = (noise_variable_128 / noise_metadata_schedule_294_e2822);
        (noise_metadata_schedule_294_e2823,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_294_e2825;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_295_e2839,) = {
    if ((noise_variable_476 == 0.0) && (noise_variable_483 == 0.0)) {
        let noise_metadata_schedule_295_e2834: f64 = (noise_variable_107 + noise_variable_231);
        let noise_metadata_schedule_295_e2836: f64 = (noise_metadata_schedule_295_e2834 - noise_variable_230);
        let noise_metadata_schedule_295_e2837: f64 = (noise_variable_107 / noise_metadata_schedule_295_e2836);
        (noise_metadata_schedule_295_e2837,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_295_e2839;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_296_e2844,) = {
    if (noise_variable_476 == 0.0) {
        (noise_variable_236,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_296_e2844;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_297_e2851,) = {
    if (noise_variable_476 == 0.0) {
        let noise_metadata_schedule_297_e2849: f64 = (0.1 * noise_variable_17);
        (noise_metadata_schedule_297_e2849,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_297_e2851;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_298_e2856,) = {
    if (noise_variable_476 == 0.0) {
        (noise_variable_108,)
    } else {
        (noise_variable_127,)
    }
};
            noise_variable_127 = noise_metadata_schedule_298_e2856;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_299_e2865,) = {
    if (noise_variable_476 == 0.0) {
        let noise_metadata_schedule_299_e2862: f64 = (noise_variable_127 / params.p61);
        let noise_metadata_schedule_299_e2863: f64 = (1.0 - noise_metadata_schedule_299_e2862);
        (noise_metadata_schedule_299_e2863,)
    } else {
        (noise_variable_199,)
    }
};
            noise_variable_199 = noise_metadata_schedule_299_e2865;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_300_e2870: f64 = (-1.0);
            let noise_metadata_schedule_300_e2872: f64 = (noise_metadata_schedule_300_e2870 / params.p66);
            let noise_metadata_schedule_300_e2873: f64 = (3.0_f64).powf(noise_metadata_schedule_300_e2872);
            let noise_metadata_schedule_300_e2874: f64 = (1.0 - noise_metadata_schedule_300_e2873);
            let noise_metadata_schedule_300_e2875: f64 = (noise_variable_14 * noise_metadata_schedule_300_e2874);
            noise_variable_129 = noise_metadata_schedule_300_e2875;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_301_e2878: f64 = (0.1 * noise_variable_14);
            noise_variable_273 = noise_metadata_schedule_301_e2878;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_302_e2881: f64 = (noise_variable_232 - noise_variable_129);
            let noise_metadata_schedule_302_e2883: f64 = (noise_metadata_schedule_302_e2881 / noise_variable_273);
            noise_variable_259 = noise_metadata_schedule_302_e2883;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_303_e2886: f64 = if noise_variable_232 < noise_variable_129 { 1.0 } else { 0.0 };
            noise_variable_484 = noise_metadata_schedule_303_e2886;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_304_e2898,) = {
    if (noise_variable_484 != 0.0) {
        let noise_metadata_schedule_304_e2892: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_304_e2893: f64 = (1.0 + noise_metadata_schedule_304_e2892);
        let noise_metadata_schedule_304_e2894: f64 = (noise_metadata_schedule_304_e2893).ln();
        let noise_metadata_schedule_304_e2895: f64 = (noise_variable_273 * noise_metadata_schedule_304_e2894);
        let noise_metadata_schedule_304_e2896: f64 = (noise_variable_232 - noise_metadata_schedule_304_e2895);
        (noise_metadata_schedule_304_e2896,)
    } else {
        (noise_variable_130,)
    }
};
            noise_variable_130 = noise_metadata_schedule_304_e2898;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_305_e2912,) = {
    if (noise_variable_484 == 0.0) {
        let noise_metadata_schedule_305_e2905: f64 = (-noise_variable_259);
        let noise_metadata_schedule_305_e2906: f64 = (noise_metadata_schedule_305_e2905).exp();
        let noise_metadata_schedule_305_e2907: f64 = (1.0 + noise_metadata_schedule_305_e2906);
        let noise_metadata_schedule_305_e2908: f64 = (noise_metadata_schedule_305_e2907).ln();
        let noise_metadata_schedule_305_e2909: f64 = (noise_variable_273 * noise_metadata_schedule_305_e2908);
        let noise_metadata_schedule_305_e2910: f64 = (noise_variable_129 - noise_metadata_schedule_305_e2909);
        (noise_metadata_schedule_305_e2910,)
    } else {
        (noise_variable_130,)
    }
};
            noise_variable_130 = noise_metadata_schedule_305_e2912;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_306_e2916: f64 = (noise_variable_130 * noise_variable_65);
            let noise_metadata_schedule_306_e2917: f64 = (1.0 - noise_metadata_schedule_306_e2916);
            let noise_metadata_schedule_306_e2920: f64 = (1.0 - params.p66);
            let noise_metadata_schedule_306_e2921: f64 = (noise_metadata_schedule_306_e2917).powf(noise_metadata_schedule_306_e2920);
            noise_variable_59 = noise_metadata_schedule_306_e2921;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_307_e2925: f64 = (1.0 - params.p66);
            let noise_metadata_schedule_307_e2926: f64 = (noise_variable_14 / noise_metadata_schedule_307_e2925);
            let noise_metadata_schedule_307_e2929: f64 = (1.0 - noise_variable_59);
            let noise_metadata_schedule_307_e2930: f64 = (noise_metadata_schedule_307_e2926 * noise_metadata_schedule_307_e2929);
            let noise_metadata_schedule_307_e2934: f64 = (noise_variable_232 - noise_variable_130);
            let noise_metadata_schedule_307_e2935: f64 = (3.0 * noise_metadata_schedule_307_e2934);
            let noise_metadata_schedule_307_e2936: f64 = (noise_metadata_schedule_307_e2930 + noise_metadata_schedule_307_e2935);
            noise_variable_131 = noise_metadata_schedule_307_e2936;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_308_e2939: f64 = if params.p73 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_485 = noise_metadata_schedule_308_e2939;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_309_e2943,) = {
    if (noise_variable_485 != 0.0) {
        (noise_variable_230,)
    } else {
        (noise_variable_132,)
    }
};
            noise_variable_132 = noise_metadata_schedule_309_e2943;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_310_e2946: f64 = if params.p73 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_486 = noise_metadata_schedule_310_e2946;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_311_e2955,) = {
    if ((noise_variable_485 == 0.0) && (noise_variable_486 != 0.0)) {
        let noise_metadata_schedule_311_e2953: f64 = (noise_variable_230 + noise_variable_125);
        (noise_metadata_schedule_311_e2953,)
    } else {
        (noise_variable_132,)
    }
};
            noise_variable_132 = noise_metadata_schedule_311_e2955;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_312_e2963,) = {
    if ((noise_variable_485 == 0.0) && (noise_variable_486 == 0.0)) {
        (noise_variable_231,)
    } else {
        (noise_variable_132,)
    }
};
            noise_variable_132 = noise_metadata_schedule_312_e2963;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_313_e2966: f64 = (2.0 - noise_variable_25);
            let noise_metadata_schedule_313_e2969: f64 = (1.0 - noise_variable_25);
            let noise_metadata_schedule_313_e2970: f64 = (noise_metadata_schedule_313_e2966 / noise_metadata_schedule_313_e2969);
            noise_variable_133 = noise_metadata_schedule_313_e2970;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_314_e2975: f64 = (-1.0);
            let noise_metadata_schedule_314_e2977: f64 = (noise_metadata_schedule_314_e2975 / params.p71);
            let noise_metadata_schedule_314_e2978: f64 = (noise_variable_133).powf(noise_metadata_schedule_314_e2977);
            let noise_metadata_schedule_314_e2979: f64 = (1.0 - noise_metadata_schedule_314_e2978);
            let noise_metadata_schedule_314_e2980: f64 = (noise_variable_17 * noise_metadata_schedule_314_e2979);
            noise_variable_134 = noise_metadata_schedule_314_e2980;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_315_e2983: f64 = (noise_variable_132 - noise_variable_134);
            let noise_metadata_schedule_315_e2985: f64 = (noise_metadata_schedule_315_e2983 / noise_variable_126);
            noise_variable_259 = noise_metadata_schedule_315_e2985;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_316_e2988: f64 = if noise_variable_132 < noise_variable_134 { 1.0 } else { 0.0 };
            noise_variable_487 = noise_metadata_schedule_316_e2988;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_317_e3000,) = {
    if (noise_variable_487 != 0.0) {
        let noise_metadata_schedule_317_e2994: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_317_e2995: f64 = (1.0 + noise_metadata_schedule_317_e2994);
        let noise_metadata_schedule_317_e2996: f64 = (noise_metadata_schedule_317_e2995).ln();
        let noise_metadata_schedule_317_e2997: f64 = (noise_variable_126 * noise_metadata_schedule_317_e2996);
        let noise_metadata_schedule_317_e2998: f64 = (noise_variable_132 - noise_metadata_schedule_317_e2997);
        (noise_metadata_schedule_317_e2998,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_317_e3000;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_318_e3014,) = {
    if (noise_variable_487 == 0.0) {
        let noise_metadata_schedule_318_e3007: f64 = (-noise_variable_259);
        let noise_metadata_schedule_318_e3008: f64 = (noise_metadata_schedule_318_e3007).exp();
        let noise_metadata_schedule_318_e3009: f64 = (1.0 + noise_metadata_schedule_318_e3008);
        let noise_metadata_schedule_318_e3010: f64 = (noise_metadata_schedule_318_e3009).ln();
        let noise_metadata_schedule_318_e3011: f64 = (noise_variable_126 * noise_metadata_schedule_318_e3010);
        let noise_metadata_schedule_318_e3012: f64 = (noise_variable_134 - noise_metadata_schedule_318_e3011);
        (noise_metadata_schedule_318_e3012,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_318_e3014;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_319_e3017: f64 = (noise_variable_199).powf(params.p75);
            noise_variable_136 = noise_metadata_schedule_319_e3017;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_320_e3021: f64 = (1.0 - params.p71);
            let noise_metadata_schedule_320_e3022: f64 = (noise_variable_17 / noise_metadata_schedule_320_e3021);
            let noise_metadata_schedule_320_e3028: f64 = (noise_variable_135 / noise_variable_17);
            let noise_metadata_schedule_320_e3029: f64 = (1.0 - noise_metadata_schedule_320_e3028);
            let noise_metadata_schedule_320_e3032: f64 = (1.0 - params.p71);
            let noise_metadata_schedule_320_e3033: f64 = (noise_metadata_schedule_320_e3029).powf(noise_metadata_schedule_320_e3032);
            let noise_metadata_schedule_320_e3034: f64 = (noise_variable_136 * noise_metadata_schedule_320_e3033);
            let noise_metadata_schedule_320_e3035: f64 = (1.0 - noise_metadata_schedule_320_e3034);
            let noise_metadata_schedule_320_e3036: f64 = (noise_metadata_schedule_320_e3022 * noise_metadata_schedule_320_e3035);
            let noise_metadata_schedule_320_e3039: f64 = (noise_variable_136 * noise_variable_133);
            let noise_metadata_schedule_320_e3042: f64 = (noise_variable_132 - noise_variable_135);
            let noise_metadata_schedule_320_e3043: f64 = (noise_metadata_schedule_320_e3039 * noise_metadata_schedule_320_e3042);
            let noise_metadata_schedule_320_e3044: f64 = (noise_metadata_schedule_320_e3036 + noise_metadata_schedule_320_e3043);
            noise_variable_137 = noise_metadata_schedule_320_e3044;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_321_e3047: f64 = (1.0 - noise_variable_25);
            let noise_metadata_schedule_321_e3049: f64 = (noise_metadata_schedule_321_e3047 * noise_variable_137);
            let noise_metadata_schedule_321_e3052: f64 = (noise_variable_25 * noise_variable_230);
            let noise_metadata_schedule_321_e3053: f64 = (noise_metadata_schedule_321_e3049 + noise_metadata_schedule_321_e3052);
            noise_variable_138 = noise_metadata_schedule_321_e3053;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_322_e3056: f64 = (4.0 * noise_variable_35);
            let noise_metadata_schedule_322_e3058: f64 = (noise_metadata_schedule_322_e3056 / noise_variable_36);
            noise_variable_139 = noise_metadata_schedule_322_e3058;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_323_e3061: f64 = (noise_variable_139 * noise_variable_246);
            noise_variable_140 = noise_metadata_schedule_323_e3061;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_324_e3066: f64 = (1.0 + noise_variable_140);
            let noise_metadata_schedule_324_e3067: f64 = (noise_metadata_schedule_324_e3066).sqrt();
            let noise_metadata_schedule_324_e3068: f64 = (1.0 + noise_metadata_schedule_324_e3067);
            let noise_metadata_schedule_324_e3069: f64 = (noise_variable_140 / noise_metadata_schedule_324_e3068);
            noise_variable_142 = noise_metadata_schedule_324_e3069;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_325_e3073: f64 = (1.0 / noise_variable_49);
            let noise_metadata_schedule_325_e3074: f64 = (noise_variable_121).powf(noise_metadata_schedule_325_e3073);
            noise_variable_122 = noise_metadata_schedule_325_e3074;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_326_e3077: f64 = (noise_variable_139 * noise_variable_122);
            noise_variable_141 = noise_metadata_schedule_326_e3077;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_327_e3082: f64 = (1.0 + noise_variable_141);
            let noise_metadata_schedule_327_e3083: f64 = (noise_metadata_schedule_327_e3082).sqrt();
            let noise_metadata_schedule_327_e3084: f64 = (1.0 + noise_metadata_schedule_327_e3083);
            let noise_metadata_schedule_327_e3085: f64 = (noise_variable_141 / noise_metadata_schedule_327_e3084);
            noise_variable_143 = noise_metadata_schedule_327_e3085;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_328_e3088: f64 = if params.p91 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_488 = noise_metadata_schedule_328_e3088;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_329_e3100,) = {
    if (noise_variable_488 != 0.0) {
        let noise_metadata_schedule_329_e3093: f64 = (noise_variable_131 / noise_variable_41);
        let noise_metadata_schedule_329_e3094: f64 = (1.0 + noise_metadata_schedule_329_e3093);
        let noise_metadata_schedule_329_e3097: f64 = (noise_variable_138 / noise_variable_40);
        let noise_metadata_schedule_329_e3098: f64 = (noise_metadata_schedule_329_e3094 + noise_metadata_schedule_329_e3097);
        (noise_metadata_schedule_329_e3098,)
    } else {
        (noise_variable_144,)
    }
};
            noise_variable_144 = noise_metadata_schedule_329_e3100;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_330_e3113,) = {
    if (noise_variable_488 == 0.0) {
        let noise_metadata_schedule_330_e3105: f64 = (noise_variable_131 / noise_variable_41);
        let noise_metadata_schedule_330_e3107: f64 = (noise_metadata_schedule_330_e3105 + 1.0);
        let noise_metadata_schedule_330_e3109: f64 = (noise_metadata_schedule_330_e3107 * noise_variable_99);
        let noise_metadata_schedule_330_e3111: f64 = (noise_metadata_schedule_330_e3109 * noise_variable_8);
        (noise_metadata_schedule_330_e3111,)
    } else {
        (noise_variable_269,)
    }
};
            noise_variable_269 = noise_metadata_schedule_330_e3113;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_331_e3125,) = {
    if (noise_variable_488 == 0.0) {
        let noise_metadata_schedule_331_e3117: f64 = (-noise_variable_138);
        let noise_metadata_schedule_331_e3119: f64 = (noise_metadata_schedule_331_e3117 / noise_variable_40);
        let noise_metadata_schedule_331_e3121: f64 = (noise_metadata_schedule_331_e3119 * noise_variable_99);
        let noise_metadata_schedule_331_e3123: f64 = (noise_metadata_schedule_331_e3121 * noise_variable_8);
        (noise_metadata_schedule_331_e3123,)
    } else {
        (noise_variable_270,)
    }
};
            noise_variable_270 = noise_metadata_schedule_331_e3125;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_332_e3141,) = {
    if (noise_variable_488 == 0.0) {
        let noise_metadata_schedule_332_e3129: f64 = (noise_variable_269).exp();
        let noise_metadata_schedule_332_e3131: f64 = (noise_variable_270).exp();
        let noise_metadata_schedule_332_e3132: f64 = (noise_metadata_schedule_332_e3129 - noise_metadata_schedule_332_e3131);
        let noise_metadata_schedule_332_e3135: f64 = (noise_variable_99 * noise_variable_8);
        let noise_metadata_schedule_332_e3136: f64 = (noise_metadata_schedule_332_e3135).exp();
        let noise_metadata_schedule_332_e3138: f64 = (noise_metadata_schedule_332_e3136 - 1.0);
        let noise_metadata_schedule_332_e3139: f64 = (noise_metadata_schedule_332_e3132 / noise_metadata_schedule_332_e3138);
        (noise_metadata_schedule_332_e3139,)
    } else {
        (noise_variable_144,)
    }
};
            noise_variable_144 = noise_metadata_schedule_332_e3141;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_333_e3144: f64 = (0.1 * 0.1);
            noise_variable_261 = noise_metadata_schedule_333_e3144;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_334_e3147: f64 = (noise_variable_144 * noise_variable_144);
            noise_variable_262 = noise_metadata_schedule_334_e3147;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_335_e3150: f64 = if noise_variable_144 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_489 = noise_metadata_schedule_335_e3150;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_336_e3163,) = {
    if (noise_variable_489 != 0.0) {
        let noise_metadata_schedule_336_e3154: f64 = (0.5 * noise_variable_261);
        let noise_metadata_schedule_336_e3157: f64 = (noise_variable_262 + noise_variable_261);
        let noise_metadata_schedule_336_e3158: f64 = (noise_metadata_schedule_336_e3157).sqrt();
        let noise_metadata_schedule_336_e3160: f64 = (noise_metadata_schedule_336_e3158 - noise_variable_144);
        let noise_metadata_schedule_336_e3161: f64 = (noise_metadata_schedule_336_e3154 / noise_metadata_schedule_336_e3160);
        (noise_metadata_schedule_336_e3161,)
    } else {
        (noise_variable_145,)
    }
};
            noise_variable_145 = noise_metadata_schedule_336_e3163;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_337_e3175,) = {
    if (noise_variable_489 == 0.0) {
        let noise_metadata_schedule_337_e3169: f64 = (noise_variable_262 + noise_variable_261);
        let noise_metadata_schedule_337_e3170: f64 = (noise_metadata_schedule_337_e3169).sqrt();
        let noise_metadata_schedule_337_e3172: f64 = (noise_metadata_schedule_337_e3170 + noise_variable_144);
        let noise_metadata_schedule_337_e3173: f64 = (0.5 * noise_metadata_schedule_337_e3172);
        (noise_metadata_schedule_337_e3173,)
    } else {
        (noise_variable_145,)
    }
};
            noise_variable_145 = noise_metadata_schedule_337_e3175;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_338_e3181: f64 = (noise_variable_142 + noise_variable_143);
            let noise_metadata_schedule_338_e3182: f64 = (0.5 * noise_metadata_schedule_338_e3181);
            let noise_metadata_schedule_338_e3183: f64 = (1.0 + noise_metadata_schedule_338_e3182);
            let noise_metadata_schedule_338_e3184: f64 = (noise_variable_145 * noise_metadata_schedule_338_e3183);
            noise_variable_146 = noise_metadata_schedule_338_e3184;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_339_e3187: f64 = (params.p14 * noise_variable_35);
            let noise_metadata_schedule_339_e3189: f64 = (noise_metadata_schedule_339_e3187 * noise_variable_122);
            noise_variable_147 = noise_metadata_schedule_339_e3189;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_340_e3192: f64 = (noise_variable_35 * noise_variable_246);
            noise_variable_148 = noise_metadata_schedule_340_e3192;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_341_e3195: f64 = (noise_variable_148 - noise_variable_147);
            let noise_metadata_schedule_341_e3197: f64 = (noise_metadata_schedule_341_e3195 / noise_variable_146);
            noise_variable_149 = noise_metadata_schedule_341_e3197;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_342_e3200: f64 = noise_variable_232;
            let noise_metadata_schedule_342_e3202: f64 = (noise_metadata_schedule_342_e3200 / 0.0001);
            noise_variable_259 = noise_metadata_schedule_342_e3202;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_343_e3205: f64 = if noise_variable_232 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_490 = noise_metadata_schedule_343_e3205;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_344_e3217,) = {
    if (noise_variable_490 != 0.0) {
        let noise_metadata_schedule_344_e3211: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_344_e3212: f64 = (1.0 + noise_metadata_schedule_344_e3211);
        let noise_metadata_schedule_344_e3213: f64 = (noise_metadata_schedule_344_e3212).ln();
        let noise_metadata_schedule_344_e3214: f64 = (0.0001 * noise_metadata_schedule_344_e3213);
        let noise_metadata_schedule_344_e3215: f64 = noise_metadata_schedule_344_e3214;
        (noise_metadata_schedule_344_e3215,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_344_e3217;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_345_e3231,) = {
    if (noise_variable_490 == 0.0) {
        let noise_metadata_schedule_345_e3224: f64 = (-noise_variable_259);
        let noise_metadata_schedule_345_e3225: f64 = (noise_metadata_schedule_345_e3224).exp();
        let noise_metadata_schedule_345_e3226: f64 = (1.0 + noise_metadata_schedule_345_e3225);
        let noise_metadata_schedule_345_e3227: f64 = (noise_metadata_schedule_345_e3226).ln();
        let noise_metadata_schedule_345_e3228: f64 = (0.0001 * noise_metadata_schedule_345_e3227);
        let noise_metadata_schedule_345_e3229: f64 = (noise_variable_232 + noise_metadata_schedule_345_e3228);
        (noise_metadata_schedule_345_e3229,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_345_e3231;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_346_e3234: f64 = (noise_variable_276 / params.p139);
            noise_variable_278 = noise_metadata_schedule_346_e3234;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_347_e3237: f64 = if noise_variable_278 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_491 = noise_metadata_schedule_347_e3237;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_348_e3242,) = {
    if (noise_variable_491 != 0.0) {
        let noise_metadata_schedule_348_e3240: f64 = (noise_variable_278).exp();
        (noise_metadata_schedule_348_e3240,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_348_e3242;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_349_e3248,) = {
    if (noise_variable_491 == 0.0) {
        let noise_metadata_schedule_349_e3246: f64 = (params.p134).exp();
        (noise_metadata_schedule_349_e3246,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_349_e3248;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_350_e3259,) = {
    if (noise_variable_491 == 0.0) {
        let noise_metadata_schedule_350_e3255: f64 = (noise_variable_278 - params.p134);
        let noise_metadata_schedule_350_e3256: f64 = (1.0 + noise_metadata_schedule_350_e3255);
        let noise_metadata_schedule_350_e3257: f64 = (noise_variable_275 * noise_metadata_schedule_350_e3256);
        (noise_metadata_schedule_350_e3257,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_350_e3259;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_351_e3263: f64 = (noise_variable_279 - 1.0);
            let noise_metadata_schedule_351_e3264: f64 = (noise_variable_325 * noise_metadata_schedule_351_e3263);
            noise_variable_326 = noise_metadata_schedule_351_e3264;
        }
        if matches!(source_index, 1 | 2) {
            let noise_metadata_schedule_352_e3267: f64 = (noise_variable_232 - params.p141);
            let noise_metadata_schedule_352_e3269: f64 = (noise_metadata_schedule_352_e3267 / 0.001);
            noise_variable_259 = noise_metadata_schedule_352_e3269;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_353_e3272: f64 = if noise_variable_232 < params.p141 { 1.0 } else { 0.0 };
            noise_variable_492 = noise_metadata_schedule_353_e3272;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_354_e3284,) = {
    if (noise_variable_492 != 0.0) {
        let noise_metadata_schedule_354_e3278: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_354_e3279: f64 = (1.0 + noise_metadata_schedule_354_e3278);
        let noise_metadata_schedule_354_e3280: f64 = (noise_metadata_schedule_354_e3279).ln();
        let noise_metadata_schedule_354_e3281: f64 = (0.001 * noise_metadata_schedule_354_e3280);
        let noise_metadata_schedule_354_e3282: f64 = (noise_variable_232 - noise_metadata_schedule_354_e3281);
        (noise_metadata_schedule_354_e3282,)
    } else {
        (noise_variable_280,)
    }
};
            noise_variable_280 = noise_metadata_schedule_354_e3284;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_355_e3298,) = {
    if (noise_variable_492 == 0.0) {
        let noise_metadata_schedule_355_e3291: f64 = (-noise_variable_259);
        let noise_metadata_schedule_355_e3292: f64 = (noise_metadata_schedule_355_e3291).exp();
        let noise_metadata_schedule_355_e3293: f64 = (1.0 + noise_metadata_schedule_355_e3292);
        let noise_metadata_schedule_355_e3294: f64 = (noise_metadata_schedule_355_e3293).ln();
        let noise_metadata_schedule_355_e3295: f64 = (0.001 * noise_metadata_schedule_355_e3294);
        let noise_metadata_schedule_355_e3296: f64 = (params.p141 - noise_metadata_schedule_355_e3295);
        (noise_metadata_schedule_355_e3296,)
    } else {
        (noise_variable_280,)
    }
};
            noise_variable_280 = noise_metadata_schedule_355_e3298;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_356_e3301: f64 = (params.p142 * noise_variable_280);
            let noise_metadata_schedule_356_e3304: f64 = (params.p141 - noise_variable_280);
            let noise_metadata_schedule_356_e3306: f64 = {let pb=noise_metadata_schedule_356_e3304;pb*pb};
            let noise_metadata_schedule_356_e3307: f64 = (noise_metadata_schedule_356_e3301 * noise_metadata_schedule_356_e3306);
            noise_variable_327 = noise_metadata_schedule_356_e3307;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_357_e3310: f64 = (noise_variable_232 * noise_variable_8);
            let noise_metadata_schedule_357_e3312: f64 = (noise_metadata_schedule_357_e3310 / params.p16);
            let noise_metadata_schedule_357_e3314: f64 = if noise_metadata_schedule_357_e3312 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_493 = noise_metadata_schedule_357_e3314;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_358_e3323,) = {
    if (noise_variable_493 != 0.0) {
        let noise_metadata_schedule_358_e3318: f64 = (noise_variable_232 * noise_variable_8);
        let noise_metadata_schedule_358_e3320: f64 = (noise_metadata_schedule_358_e3318 / params.p16);
        let noise_metadata_schedule_358_e3321: f64 = (noise_metadata_schedule_358_e3320).exp();
        (noise_metadata_schedule_358_e3321,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_358_e3323;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_359_e3329,) = {
    if (noise_variable_493 == 0.0) {
        let noise_metadata_schedule_359_e3327: f64 = (params.p134).exp();
        (noise_metadata_schedule_359_e3327,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_359_e3329;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_360_e3344,) = {
    if (noise_variable_493 == 0.0) {
        let noise_metadata_schedule_360_e3336: f64 = (noise_variable_232 * noise_variable_8);
        let noise_metadata_schedule_360_e3338: f64 = (noise_metadata_schedule_360_e3336 / params.p16);
        let noise_metadata_schedule_360_e3340: f64 = (noise_metadata_schedule_360_e3338 - params.p134);
        let noise_metadata_schedule_360_e3341: f64 = (1.0 + noise_metadata_schedule_360_e3340);
        let noise_metadata_schedule_360_e3342: f64 = (noise_variable_275 * noise_metadata_schedule_360_e3341);
        (noise_metadata_schedule_360_e3342,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_360_e3344;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_361_e3347: f64 = if params.p23 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_494 = noise_metadata_schedule_361_e3347;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_362_e3350: f64 = (noise_variable_232 - noise_variable_55);
            let noise_metadata_schedule_362_e3352: f64 = (noise_metadata_schedule_362_e3350 * noise_variable_8);
            let noise_metadata_schedule_362_e3354: f64 = if noise_metadata_schedule_362_e3352 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_495 = noise_metadata_schedule_362_e3354;
        }
        if matches!(source_index, 2 | 6 | 8) {
            let (noise_metadata_schedule_363_e3365,) = {
    if ((noise_variable_494 != 0.0) && (noise_variable_495 != 0.0)) {
        let noise_metadata_schedule_363_e3360: f64 = (noise_variable_232 - noise_variable_55);
        let noise_metadata_schedule_363_e3362: f64 = (noise_metadata_schedule_363_e3360 * noise_variable_8);
        let noise_metadata_schedule_363_e3363: f64 = (noise_metadata_schedule_363_e3362).exp();
        (noise_metadata_schedule_363_e3363,)
    } else {
        (noise_variable_278,)
    }
};
            noise_variable_278 = noise_metadata_schedule_363_e3365;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_364_e3373,) = {
    if ((noise_variable_494 != 0.0) && (noise_variable_495 == 0.0)) {
        let noise_metadata_schedule_364_e3371: f64 = (params.p134).exp();
        (noise_metadata_schedule_364_e3371,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_364_e3373;
        }
        if matches!(source_index, 2 | 6 | 8) {
            let (noise_metadata_schedule_365_e3390,) = {
    if ((noise_variable_494 != 0.0) && (noise_variable_495 == 0.0)) {
        let noise_metadata_schedule_365_e3382: f64 = (noise_variable_232 - noise_variable_55);
        let noise_metadata_schedule_365_e3384: f64 = (noise_metadata_schedule_365_e3382 * noise_variable_8);
        let noise_metadata_schedule_365_e3386: f64 = (noise_metadata_schedule_365_e3384 - params.p134);
        let noise_metadata_schedule_365_e3387: f64 = (1.0 + noise_metadata_schedule_365_e3386);
        let noise_metadata_schedule_365_e3388: f64 = (noise_variable_275 * noise_metadata_schedule_365_e3387);
        (noise_metadata_schedule_365_e3388,)
    } else {
        (noise_variable_278,)
    }
};
            noise_variable_278 = noise_metadata_schedule_365_e3390;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_366_e3393: f64 = (noise_variable_149 / noise_variable_35);
            let noise_metadata_schedule_366_e3395: f64 = (noise_metadata_schedule_366_e3393 - 1000.0);
            let noise_metadata_schedule_366_e3397: f64 = if noise_metadata_schedule_366_e3395 < 40.0 { 1.0 } else { 0.0 };
            noise_variable_496 = noise_metadata_schedule_366_e3397;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_367_e3408,) = {
    if ((noise_variable_494 != 0.0) && (noise_variable_496 != 0.0)) {
        let noise_metadata_schedule_367_e3403: f64 = (noise_variable_149 / noise_variable_35);
        let noise_metadata_schedule_367_e3405: f64 = (noise_metadata_schedule_367_e3403 - 1000.0);
        let noise_metadata_schedule_367_e3406: f64 = (noise_metadata_schedule_367_e3405).exp();
        (noise_metadata_schedule_367_e3406,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_367_e3408;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_368_e3416,) = {
    if ((noise_variable_494 != 0.0) && (noise_variable_496 == 0.0)) {
        let noise_metadata_schedule_368_e3414: f64 = (40.0_f64).exp();
        (noise_metadata_schedule_368_e3414,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_368_e3416;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_369_e3433,) = {
    if ((noise_variable_494 != 0.0) && (noise_variable_496 == 0.0)) {
        let noise_metadata_schedule_369_e3425: f64 = (noise_variable_149 / noise_variable_35);
        let noise_metadata_schedule_369_e3427: f64 = (noise_metadata_schedule_369_e3425 - 1000.0);
        let noise_metadata_schedule_369_e3429: f64 = (noise_metadata_schedule_369_e3427 - 40.0);
        let noise_metadata_schedule_369_e3430: f64 = (1.0 + noise_metadata_schedule_369_e3429);
        let noise_metadata_schedule_369_e3431: f64 = (noise_variable_275 * noise_metadata_schedule_369_e3430);
        (noise_metadata_schedule_369_e3431,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_369_e3433;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_370_e3476,) = {
    if (noise_variable_494 != 0.0) {
        let noise_metadata_schedule_370_e3438: f64 = (noise_variable_276 - 1.0);
        let noise_metadata_schedule_370_e3439: f64 = (noise_variable_42 * noise_metadata_schedule_370_e3438);
        let noise_metadata_schedule_370_e3442: f64 = (noise_variable_53 * 2.0);
        let noise_metadata_schedule_370_e3445: f64 = (noise_variable_276 - 1.0);
        let noise_metadata_schedule_370_e3446: f64 = (noise_metadata_schedule_370_e3442 * noise_metadata_schedule_370_e3445);
        let noise_metadata_schedule_370_e3451: f64 = (4.0 * noise_variable_278);
        let noise_metadata_schedule_370_e3452: f64 = (1.0 + noise_metadata_schedule_370_e3451);
        let noise_metadata_schedule_370_e3453: f64 = (noise_metadata_schedule_370_e3452).sqrt();
        let noise_metadata_schedule_370_e3454: f64 = (1.0 + noise_metadata_schedule_370_e3453);
        let noise_metadata_schedule_370_e3455: f64 = (noise_metadata_schedule_370_e3446 / noise_metadata_schedule_370_e3454);
        let noise_metadata_schedule_370_e3459: f64 = (noise_variable_138 / noise_variable_40);
        let noise_metadata_schedule_370_e3460: f64 = (1.0 + noise_metadata_schedule_370_e3459);
        let noise_metadata_schedule_370_e3461: f64 = (noise_metadata_schedule_370_e3455 * noise_metadata_schedule_370_e3460);
        let noise_metadata_schedule_370_e3462: f64 = (noise_metadata_schedule_370_e3439 + noise_metadata_schedule_370_e3461);
        let noise_metadata_schedule_370_e3466: f64 = (noise_variable_121 - 1.0);
        let noise_metadata_schedule_370_e3467: f64 = (noise_variable_54 * noise_metadata_schedule_370_e3466);
        let noise_metadata_schedule_370_e3469: f64 = (noise_metadata_schedule_370_e3467 * noise_variable_279);
        let noise_metadata_schedule_370_e3472: f64 = (1.0 + noise_variable_279);
        let noise_metadata_schedule_370_e3473: f64 = (noise_metadata_schedule_370_e3469 / noise_metadata_schedule_370_e3472);
        let noise_metadata_schedule_370_e3474: f64 = (noise_metadata_schedule_370_e3462 + noise_metadata_schedule_370_e3473);
        (noise_metadata_schedule_370_e3474,)
    } else {
        (noise_variable_151,)
    }
};
            noise_variable_151 = noise_metadata_schedule_370_e3476;
        }
        if matches!(source_index, 2 | 6) {
            let noise_metadata_schedule_371_e3479: f64 = if params.p92 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_497 = noise_metadata_schedule_371_e3479;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_372_e3490,) = {
    if ((noise_variable_494 == 0.0) && (noise_variable_497 != 0.0)) {
        let noise_metadata_schedule_372_e3487: f64 = (noise_variable_276 - 1.0);
        let noise_metadata_schedule_372_e3488: f64 = (noise_variable_42 * noise_metadata_schedule_372_e3487);
        (noise_metadata_schedule_372_e3488,)
    } else {
        (noise_variable_151,)
    }
};
            noise_variable_151 = noise_metadata_schedule_372_e3490;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_373_e3520,) = {
    if ((noise_variable_494 == 0.0) && (noise_variable_497 == 0.0)) {
        let noise_metadata_schedule_373_e3499: f64 = (1.0 - params.p92);
        let noise_metadata_schedule_373_e3502: f64 = (noise_variable_276 - 1.0);
        let noise_metadata_schedule_373_e3503: f64 = (noise_metadata_schedule_373_e3499 * noise_metadata_schedule_373_e3502);
        let noise_metadata_schedule_373_e3507: f64 = (noise_variable_276 + noise_variable_121);
        let noise_metadata_schedule_373_e3509: f64 = (noise_metadata_schedule_373_e3507 - 2.0);
        let noise_metadata_schedule_373_e3510: f64 = (params.p92 * noise_metadata_schedule_373_e3509);
        let noise_metadata_schedule_373_e3514: f64 = (noise_variable_138 / noise_variable_40);
        let noise_metadata_schedule_373_e3515: f64 = (1.0 + noise_metadata_schedule_373_e3514);
        let noise_metadata_schedule_373_e3516: f64 = (noise_metadata_schedule_373_e3510 * noise_metadata_schedule_373_e3515);
        let noise_metadata_schedule_373_e3517: f64 = (noise_metadata_schedule_373_e3503 + noise_metadata_schedule_373_e3516);
        let noise_metadata_schedule_373_e3518: f64 = (noise_variable_42 * noise_metadata_schedule_373_e3517);
        (noise_metadata_schedule_373_e3518,)
    } else {
        (noise_variable_151,)
    }
};
            noise_variable_151 = noise_metadata_schedule_373_e3520;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_374_e3523: f64 = (noise_variable_233 * noise_variable_8);
            let noise_metadata_schedule_374_e3525: f64 = (noise_metadata_schedule_374_e3523 / params.p18);
            let noise_metadata_schedule_374_e3527: f64 = if noise_metadata_schedule_374_e3525 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_498 = noise_metadata_schedule_374_e3527;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_375_e3536,) = {
    if (noise_variable_498 != 0.0) {
        let noise_metadata_schedule_375_e3531: f64 = (noise_variable_233 * noise_variable_8);
        let noise_metadata_schedule_375_e3533: f64 = (noise_metadata_schedule_375_e3531 / params.p18);
        let noise_metadata_schedule_375_e3534: f64 = (noise_metadata_schedule_375_e3533).exp();
        (noise_metadata_schedule_375_e3534,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_375_e3536;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_376_e3542,) = {
    if (noise_variable_498 == 0.0) {
        let noise_metadata_schedule_376_e3540: f64 = (params.p134).exp();
        (noise_metadata_schedule_376_e3540,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_376_e3542;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_377_e3557,) = {
    if (noise_variable_498 == 0.0) {
        let noise_metadata_schedule_377_e3549: f64 = (noise_variable_233 * noise_variable_8);
        let noise_metadata_schedule_377_e3551: f64 = (noise_metadata_schedule_377_e3549 / params.p18);
        let noise_metadata_schedule_377_e3553: f64 = (noise_metadata_schedule_377_e3551 - params.p134);
        let noise_metadata_schedule_377_e3554: f64 = (1.0 + noise_metadata_schedule_377_e3553);
        let noise_metadata_schedule_377_e3555: f64 = (noise_variable_275 * noise_metadata_schedule_377_e3554);
        (noise_metadata_schedule_377_e3555,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_377_e3557;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_378_e3560: f64 = if params.p23 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_499 = noise_metadata_schedule_378_e3560;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_379_e3563: f64 = (noise_variable_233 - noise_variable_55);
            let noise_metadata_schedule_379_e3565: f64 = (noise_metadata_schedule_379_e3563 * noise_variable_8);
            let noise_metadata_schedule_379_e3567: f64 = if noise_metadata_schedule_379_e3565 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_500 = noise_metadata_schedule_379_e3567;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_380_e3578,) = {
    if ((noise_variable_499 != 0.0) && (noise_variable_500 != 0.0)) {
        let noise_metadata_schedule_380_e3573: f64 = (noise_variable_233 - noise_variable_55);
        let noise_metadata_schedule_380_e3575: f64 = (noise_metadata_schedule_380_e3573 * noise_variable_8);
        let noise_metadata_schedule_380_e3576: f64 = (noise_metadata_schedule_380_e3575).exp();
        (noise_metadata_schedule_380_e3576,)
    } else {
        (noise_variable_278,)
    }
};
            noise_variable_278 = noise_metadata_schedule_380_e3578;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_381_e3586,) = {
    if ((noise_variable_499 != 0.0) && (noise_variable_500 == 0.0)) {
        let noise_metadata_schedule_381_e3584: f64 = (params.p134).exp();
        (noise_metadata_schedule_381_e3584,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_381_e3586;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_382_e3603,) = {
    if ((noise_variable_499 != 0.0) && (noise_variable_500 == 0.0)) {
        let noise_metadata_schedule_382_e3595: f64 = (noise_variable_233 - noise_variable_55);
        let noise_metadata_schedule_382_e3597: f64 = (noise_metadata_schedule_382_e3595 * noise_variable_8);
        let noise_metadata_schedule_382_e3599: f64 = (noise_metadata_schedule_382_e3597 - params.p134);
        let noise_metadata_schedule_382_e3600: f64 = (1.0 + noise_metadata_schedule_382_e3599);
        let noise_metadata_schedule_382_e3601: f64 = (noise_variable_275 * noise_metadata_schedule_382_e3600);
        (noise_metadata_schedule_382_e3601,)
    } else {
        (noise_variable_278,)
    }
};
            noise_variable_278 = noise_metadata_schedule_382_e3603;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_383_e3628,) = {
    if (noise_variable_499 != 0.0) {
        let noise_metadata_schedule_383_e3608: f64 = (noise_variable_276 - 1.0);
        let noise_metadata_schedule_383_e3609: f64 = (noise_variable_44 * noise_metadata_schedule_383_e3608);
        let noise_metadata_schedule_383_e3612: f64 = (noise_variable_45 * 2.0);
        let noise_metadata_schedule_383_e3615: f64 = (noise_variable_276 - 1.0);
        let noise_metadata_schedule_383_e3616: f64 = (noise_metadata_schedule_383_e3612 * noise_metadata_schedule_383_e3615);
        let noise_metadata_schedule_383_e3621: f64 = (4.0 * noise_variable_278);
        let noise_metadata_schedule_383_e3622: f64 = (1.0 + noise_metadata_schedule_383_e3621);
        let noise_metadata_schedule_383_e3623: f64 = (noise_metadata_schedule_383_e3622).sqrt();
        let noise_metadata_schedule_383_e3624: f64 = (1.0 + noise_metadata_schedule_383_e3623);
        let noise_metadata_schedule_383_e3625: f64 = (noise_metadata_schedule_383_e3616 / noise_metadata_schedule_383_e3624);
        let noise_metadata_schedule_383_e3626: f64 = (noise_metadata_schedule_383_e3609 + noise_metadata_schedule_383_e3625);
        (noise_metadata_schedule_383_e3626,)
    } else {
        (noise_variable_152,)
    }
};
            noise_variable_152 = noise_metadata_schedule_383_e3628;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_384_e3637,) = {
    if (noise_variable_499 == 0.0) {
        let noise_metadata_schedule_384_e3634: f64 = (noise_variable_276 - 1.0);
        let noise_metadata_schedule_384_e3635: f64 = (noise_variable_44 * noise_metadata_schedule_384_e3634);
        (noise_metadata_schedule_384_e3635,)
    } else {
        (noise_variable_152,)
    }
};
            noise_variable_152 = noise_metadata_schedule_384_e3637;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_385_e3640: f64 = (noise_variable_232 * noise_variable_8);
            let noise_metadata_schedule_385_e3642: f64 = (noise_metadata_schedule_385_e3640 / params.p20);
            let noise_metadata_schedule_385_e3644: f64 = if noise_metadata_schedule_385_e3642 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_501 = noise_metadata_schedule_385_e3644;
        }
        if matches!(source_index, 2 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_386_e3653,) = {
    if (noise_variable_501 != 0.0) {
        let noise_metadata_schedule_386_e3648: f64 = (noise_variable_232 * noise_variable_8);
        let noise_metadata_schedule_386_e3650: f64 = (noise_metadata_schedule_386_e3648 / params.p20);
        let noise_metadata_schedule_386_e3651: f64 = (noise_metadata_schedule_386_e3650).exp();
        (noise_metadata_schedule_386_e3651,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_386_e3653;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_387_e3659,) = {
    if (noise_variable_501 == 0.0) {
        let noise_metadata_schedule_387_e3657: f64 = (params.p134).exp();
        (noise_metadata_schedule_387_e3657,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_387_e3659;
        }
        if matches!(source_index, 2 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_388_e3674,) = {
    if (noise_variable_501 == 0.0) {
        let noise_metadata_schedule_388_e3666: f64 = (noise_variable_232 * noise_variable_8);
        let noise_metadata_schedule_388_e3668: f64 = (noise_metadata_schedule_388_e3666 / params.p20);
        let noise_metadata_schedule_388_e3670: f64 = (noise_metadata_schedule_388_e3668 - params.p134);
        let noise_metadata_schedule_388_e3671: f64 = (1.0 + noise_metadata_schedule_388_e3670);
        let noise_metadata_schedule_388_e3672: f64 = (noise_variable_275 * noise_metadata_schedule_388_e3671);
        (noise_metadata_schedule_388_e3672,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_388_e3674;
        }
        if matches!(source_index, 2 | 7) {
            let noise_metadata_schedule_389_e3678: f64 = (noise_variable_276 - 1.0);
            let noise_metadata_schedule_389_e3679: f64 = (noise_variable_38 * noise_metadata_schedule_389_e3678);
            noise_variable_153 = noise_metadata_schedule_389_e3679;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_390_e3682: f64 = (noise_variable_233 * noise_variable_8);
            let noise_metadata_schedule_390_e3684: f64 = (noise_metadata_schedule_390_e3682 / params.p22);
            let noise_metadata_schedule_390_e3686: f64 = if noise_metadata_schedule_390_e3684 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_502 = noise_metadata_schedule_390_e3686;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_391_e3695,) = {
    if (noise_variable_502 != 0.0) {
        let noise_metadata_schedule_391_e3690: f64 = (noise_variable_233 * noise_variable_8);
        let noise_metadata_schedule_391_e3692: f64 = (noise_metadata_schedule_391_e3690 / params.p22);
        let noise_metadata_schedule_391_e3693: f64 = (noise_metadata_schedule_391_e3692).exp();
        (noise_metadata_schedule_391_e3693,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_391_e3695;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_392_e3701,) = {
    if (noise_variable_502 == 0.0) {
        let noise_metadata_schedule_392_e3699: f64 = (params.p134).exp();
        (noise_metadata_schedule_392_e3699,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_392_e3701;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_393_e3716,) = {
    if (noise_variable_502 == 0.0) {
        let noise_metadata_schedule_393_e3708: f64 = (noise_variable_233 * noise_variable_8);
        let noise_metadata_schedule_393_e3710: f64 = (noise_metadata_schedule_393_e3708 / params.p22);
        let noise_metadata_schedule_393_e3712: f64 = (noise_metadata_schedule_393_e3710 - params.p134);
        let noise_metadata_schedule_393_e3713: f64 = (1.0 + noise_metadata_schedule_393_e3712);
        let noise_metadata_schedule_393_e3714: f64 = (noise_variable_275 * noise_metadata_schedule_393_e3713);
        (noise_metadata_schedule_393_e3714,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_393_e3716;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_394_e3720: f64 = (noise_variable_276 - 1.0);
            let noise_metadata_schedule_394_e3721: f64 = (noise_variable_46 * noise_metadata_schedule_394_e3720);
            noise_variable_155 = noise_metadata_schedule_394_e3721;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_395_e3724: f64 = (noise_variable_235 * noise_variable_8);
            let noise_metadata_schedule_395_e3726: f64 = (noise_metadata_schedule_395_e3724 / params.p31);
            let noise_metadata_schedule_395_e3728: f64 = if noise_metadata_schedule_395_e3726 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_503 = noise_metadata_schedule_395_e3728;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_396_e3737,) = {
    if (noise_variable_503 != 0.0) {
        let noise_metadata_schedule_396_e3732: f64 = (noise_variable_235 * noise_variable_8);
        let noise_metadata_schedule_396_e3734: f64 = (noise_metadata_schedule_396_e3732 / params.p31);
        let noise_metadata_schedule_396_e3735: f64 = (noise_metadata_schedule_396_e3734).exp();
        (noise_metadata_schedule_396_e3735,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_396_e3737;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_397_e3743,) = {
    if (noise_variable_503 == 0.0) {
        let noise_metadata_schedule_397_e3741: f64 = (params.p134).exp();
        (noise_metadata_schedule_397_e3741,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_397_e3743;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_398_e3758,) = {
    if (noise_variable_503 == 0.0) {
        let noise_metadata_schedule_398_e3750: f64 = (noise_variable_235 * noise_variable_8);
        let noise_metadata_schedule_398_e3752: f64 = (noise_metadata_schedule_398_e3750 / params.p31);
        let noise_metadata_schedule_398_e3754: f64 = (noise_metadata_schedule_398_e3752 - params.p134);
        let noise_metadata_schedule_398_e3755: f64 = (1.0 + noise_metadata_schedule_398_e3754);
        let noise_metadata_schedule_398_e3756: f64 = (noise_variable_275 * noise_metadata_schedule_398_e3755);
        (noise_metadata_schedule_398_e3756,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_398_e3758;
        }
        if matches!(source_index, 9 | 10) {
            let noise_metadata_schedule_399_e3762: f64 = (noise_variable_276 - 1.0);
            let noise_metadata_schedule_399_e3763: f64 = (noise_variable_39 * noise_metadata_schedule_399_e3762);
            noise_variable_154 = noise_metadata_schedule_399_e3763;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 15 | 16) {
            let noise_metadata_schedule_400_e3766: f64 = (noise_variable_233 * noise_variable_8);
            let noise_metadata_schedule_400_e3768: f64 = (noise_metadata_schedule_400_e3766 / params.p133);
            let noise_metadata_schedule_400_e3770: f64 = if noise_metadata_schedule_400_e3768 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_504 = noise_metadata_schedule_400_e3770;
        }
        if matches!(source_index, 7 | 8) {
            let (noise_metadata_schedule_401_e3779,) = {
    if (noise_variable_504 != 0.0) {
        let noise_metadata_schedule_401_e3774: f64 = (noise_variable_233 * noise_variable_8);
        let noise_metadata_schedule_401_e3776: f64 = (noise_metadata_schedule_401_e3774 / params.p133);
        let noise_metadata_schedule_401_e3777: f64 = (noise_metadata_schedule_401_e3776).exp();
        (noise_metadata_schedule_401_e3777,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_401_e3779;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 15 | 16) {
            let (noise_metadata_schedule_402_e3785,) = {
    if (noise_variable_504 == 0.0) {
        let noise_metadata_schedule_402_e3783: f64 = (params.p134).exp();
        (noise_metadata_schedule_402_e3783,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_402_e3785;
        }
        if matches!(source_index, 7 | 8) {
            let (noise_metadata_schedule_403_e3800,) = {
    if (noise_variable_504 == 0.0) {
        let noise_metadata_schedule_403_e3792: f64 = (noise_variable_233 * noise_variable_8);
        let noise_metadata_schedule_403_e3794: f64 = (noise_metadata_schedule_403_e3792 / params.p133);
        let noise_metadata_schedule_403_e3796: f64 = (noise_metadata_schedule_403_e3794 - params.p134);
        let noise_metadata_schedule_403_e3797: f64 = (1.0 + noise_metadata_schedule_403_e3796);
        let noise_metadata_schedule_403_e3798: f64 = (noise_variable_275 * noise_metadata_schedule_403_e3797);
        (noise_metadata_schedule_403_e3798,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_403_e3800;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_404_e3804: f64 = (noise_variable_276 - 1.0);
            let noise_metadata_schedule_404_e3805: f64 = (noise_variable_47 * noise_metadata_schedule_404_e3804);
            noise_variable_156 = noise_metadata_schedule_404_e3805;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_405_e3816: f64 = if (((params.p33 > 0.0) && (params.p34 > 0.0)) && (noise_variable_232 < 0.0)) { 1.0 } else { 0.0 };
            noise_variable_505 = noise_metadata_schedule_405_e3816;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_406_e3822: f64 = (2.0 * noise_variable_59);
            let noise_metadata_schedule_406_e3823: f64 = (noise_variable_62 / noise_metadata_schedule_406_e3822);
            let noise_metadata_schedule_406_e3824: f64 = (1.0 - noise_metadata_schedule_406_e3823);
            let noise_metadata_schedule_406_e3825: f64 = (noise_variable_61 * noise_metadata_schedule_406_e3824);
            let noise_metadata_schedule_406_e3827: f64 = if noise_metadata_schedule_406_e3825 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_506 = noise_metadata_schedule_406_e3827;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_407_e3842,) = {
    if ((noise_variable_505 != 0.0) && (noise_variable_506 != 0.0)) {
        let noise_metadata_schedule_407_e3836: f64 = (2.0 * noise_variable_59);
        let noise_metadata_schedule_407_e3837: f64 = (noise_variable_62 / noise_metadata_schedule_407_e3836);
        let noise_metadata_schedule_407_e3838: f64 = (1.0 - noise_metadata_schedule_407_e3837);
        let noise_metadata_schedule_407_e3839: f64 = (noise_variable_61 * noise_metadata_schedule_407_e3838);
        let noise_metadata_schedule_407_e3840: f64 = (noise_metadata_schedule_407_e3839).exp();
        (noise_metadata_schedule_407_e3840,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_407_e3842;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_408_e3850,) = {
    if ((noise_variable_505 != 0.0) && (noise_variable_506 == 0.0)) {
        let noise_metadata_schedule_408_e3848: f64 = (params.p134).exp();
        (noise_metadata_schedule_408_e3848,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_408_e3850;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_409_e3871,) = {
    if ((noise_variable_505 != 0.0) && (noise_variable_506 == 0.0)) {
        let noise_metadata_schedule_409_e3862: f64 = (2.0 * noise_variable_59);
        let noise_metadata_schedule_409_e3863: f64 = (noise_variable_62 / noise_metadata_schedule_409_e3862);
        let noise_metadata_schedule_409_e3864: f64 = (1.0 - noise_metadata_schedule_409_e3863);
        let noise_metadata_schedule_409_e3865: f64 = (noise_variable_61 * noise_metadata_schedule_409_e3864);
        let noise_metadata_schedule_409_e3867: f64 = (noise_metadata_schedule_409_e3865 - params.p134);
        let noise_metadata_schedule_409_e3868: f64 = (1.0 + noise_metadata_schedule_409_e3867);
        let noise_metadata_schedule_409_e3869: f64 = (noise_variable_275 * noise_metadata_schedule_409_e3868);
        (noise_metadata_schedule_409_e3869,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_409_e3871;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_410_e3877,) = {
    if (noise_variable_505 != 0.0) {
        let noise_metadata_schedule_410_e3875: f64 = (noise_variable_232 * noise_variable_65);
        (noise_metadata_schedule_410_e3875,)
    } else {
        (noise_variable_255,)
    }
};
            noise_variable_255 = noise_metadata_schedule_410_e3877;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_411_e3921,) = {
    if (noise_variable_505 != 0.0) {
        let noise_metadata_schedule_411_e3881: f64 = (noise_variable_255 * noise_variable_255);
        let noise_metadata_schedule_411_e3883: f64 = (noise_metadata_schedule_411_e3881 + 1e-30);
        let noise_metadata_schedule_411_e3884: f64 = (noise_metadata_schedule_411_e3883).sqrt();
        let noise_metadata_schedule_411_e3886: f64 = (-2.0);
        let noise_metadata_schedule_411_e3888: f64 = (noise_metadata_schedule_411_e3886 - params.p66);
        let noise_metadata_schedule_411_e3889: f64 = (noise_metadata_schedule_411_e3884).powf(noise_metadata_schedule_411_e3888);
        let noise_metadata_schedule_411_e3894: f64 = (params.p66 * params.p66);
        let noise_metadata_schedule_411_e3895: f64 = (1.0 - noise_metadata_schedule_411_e3894);
        let noise_metadata_schedule_411_e3898: f64 = (3.0 * noise_variable_255);
        let noise_metadata_schedule_411_e3901: f64 = (params.p66 - 1.0);
        let noise_metadata_schedule_411_e3902: f64 = (noise_metadata_schedule_411_e3898 * noise_metadata_schedule_411_e3901);
        let noise_metadata_schedule_411_e3903: f64 = (noise_metadata_schedule_411_e3895 - noise_metadata_schedule_411_e3902);
        let noise_metadata_schedule_411_e3904: f64 = (params.p66 * noise_metadata_schedule_411_e3903);
        let noise_metadata_schedule_411_e3907: f64 = (6.0 * noise_variable_255);
        let noise_metadata_schedule_411_e3909: f64 = (noise_metadata_schedule_411_e3907 * noise_variable_255);
        let noise_metadata_schedule_411_e3912: f64 = (params.p66 - 1.0);
        let noise_metadata_schedule_411_e3914: f64 = (noise_metadata_schedule_411_e3912 + noise_variable_255);
        let noise_metadata_schedule_411_e3915: f64 = (noise_metadata_schedule_411_e3909 * noise_metadata_schedule_411_e3914);
        let noise_metadata_schedule_411_e3916: f64 = (noise_metadata_schedule_411_e3904 - noise_metadata_schedule_411_e3915);
        let noise_metadata_schedule_411_e3917: f64 = (noise_metadata_schedule_411_e3889 * noise_metadata_schedule_411_e3916);
        let noise_metadata_schedule_411_e3919: f64 = (noise_metadata_schedule_411_e3917 * 0.16666666666666666);
        (noise_metadata_schedule_411_e3919,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_411_e3921;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_412_e3933,) = {
    if (noise_variable_505 != 0.0) {
        let noise_metadata_schedule_412_e3925: f64 = (noise_variable_232 * noise_variable_62);
        let noise_metadata_schedule_412_e3927: f64 = (noise_metadata_schedule_412_e3925 * noise_variable_61);
        let noise_metadata_schedule_412_e3930: f64 = (noise_variable_70 * noise_variable_60);
        let noise_metadata_schedule_412_e3931: f64 = (noise_metadata_schedule_412_e3927 / noise_metadata_schedule_412_e3930);
        (noise_metadata_schedule_412_e3931,)
    } else {
        (noise_variable_255,)
    }
};
            noise_variable_255 = noise_metadata_schedule_412_e3933;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_413_e3936: f64 = (-0.001);
            let noise_metadata_schedule_413_e3937: f64 = if noise_variable_255 < noise_metadata_schedule_413_e3936 { 1.0 } else { 0.0 };
            noise_variable_507 = noise_metadata_schedule_413_e3937;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_414_e3940: f64 = if noise_variable_255 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_508 = noise_metadata_schedule_414_e3940;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_415_e3949,) = {
    if (((noise_variable_505 != 0.0) && (noise_variable_507 != 0.0)) && (noise_variable_508 != 0.0)) {
        let noise_metadata_schedule_415_e3947: f64 = (noise_variable_255).exp();
        (noise_metadata_schedule_415_e3947,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_415_e3949;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_416_e3959,) = {
    if (((noise_variable_505 != 0.0) && (noise_variable_507 != 0.0)) && (noise_variable_508 == 0.0)) {
        let noise_metadata_schedule_416_e3957: f64 = (params.p134).exp();
        (noise_metadata_schedule_416_e3957,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_416_e3959;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_417_e3974,) = {
    if (((noise_variable_505 != 0.0) && (noise_variable_507 != 0.0)) && (noise_variable_508 == 0.0)) {
        let noise_metadata_schedule_417_e3970: f64 = (noise_variable_255 - params.p134);
        let noise_metadata_schedule_417_e3971: f64 = (1.0 + noise_metadata_schedule_417_e3970);
        let noise_metadata_schedule_417_e3972: f64 = (noise_variable_275 * noise_metadata_schedule_417_e3971);
        (noise_metadata_schedule_417_e3972,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_417_e3974;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_418_e3989,) = {
    if ((noise_variable_505 != 0.0) && (noise_variable_507 != 0.0)) {
        let noise_metadata_schedule_418_e3979: f64 = (-noise_variable_232);
        let noise_metadata_schedule_418_e3983: f64 = (1.0 - noise_variable_91);
        let noise_metadata_schedule_418_e3985: f64 = (noise_metadata_schedule_418_e3983 / noise_variable_255);
        let noise_metadata_schedule_418_e3986: f64 = (1.0 + noise_metadata_schedule_418_e3985);
        let noise_metadata_schedule_418_e3987: f64 = (noise_metadata_schedule_418_e3979 * noise_metadata_schedule_418_e3986);
        (noise_metadata_schedule_418_e3987,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_418_e3989;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_419_e4012,) = {
    if ((noise_variable_505 != 0.0) && (noise_variable_507 == 0.0)) {
        let noise_metadata_schedule_419_e3996: f64 = (noise_variable_232 * 0.5);
        let noise_metadata_schedule_419_e3998: f64 = (noise_metadata_schedule_419_e3996 * noise_variable_255);
        let noise_metadata_schedule_419_e4002: f64 = (noise_variable_255 * 0.3333333333333333);
        let noise_metadata_schedule_419_e4006: f64 = (0.25 * noise_variable_255);
        let noise_metadata_schedule_419_e4007: f64 = (1.0 + noise_metadata_schedule_419_e4006);
        let noise_metadata_schedule_419_e4008: f64 = (noise_metadata_schedule_419_e4002 * noise_metadata_schedule_419_e4007);
        let noise_metadata_schedule_419_e4009: f64 = (1.0 + noise_metadata_schedule_419_e4008);
        let noise_metadata_schedule_419_e4010: f64 = (noise_metadata_schedule_419_e3998 * noise_metadata_schedule_419_e4009);
        (noise_metadata_schedule_419_e4010,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_419_e4012;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_420_e4028,) = {
    if (noise_variable_505 != 0.0) {
        let noise_metadata_schedule_420_e4016: f64 = (2.0 * noise_variable_58);
        let noise_metadata_schedule_420_e4018: f64 = (noise_metadata_schedule_420_e4016 * noise_variable_69);
        let noise_metadata_schedule_420_e4020: f64 = (noise_metadata_schedule_420_e4018 * noise_variable_59);
        let noise_metadata_schedule_420_e4022: f64 = (noise_metadata_schedule_420_e4020 * noise_variable_68);
        let noise_metadata_schedule_420_e4024: f64 = (noise_metadata_schedule_420_e4022 * noise_variable_65);
        let noise_metadata_schedule_420_e4026: f64 = (noise_metadata_schedule_420_e4024 * noise_variable_63);
        (noise_metadata_schedule_420_e4026,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_420_e4028;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_422_e4038,) = {
    if (noise_variable_505 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_422_e4038;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_423_e4049: f64 = if (((params.p35 > 0.0) && (params.p36 > 0.0)) && (noise_variable_230 < 0.0)) { 1.0 } else { 0.0 };
            noise_variable_509 = noise_metadata_schedule_423_e4049;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_424_e4061,) = {
    if (noise_variable_509 != 0.0) {
        let noise_metadata_schedule_424_e4054: f64 = (noise_variable_230 * noise_variable_67);
        let noise_metadata_schedule_424_e4055: f64 = (1.0 - noise_metadata_schedule_424_e4054);
        let noise_metadata_schedule_424_e4058: f64 = (1.0 - noise_variable_76);
        let noise_metadata_schedule_424_e4059: f64 = (noise_metadata_schedule_424_e4055).powf(noise_metadata_schedule_424_e4058);
        (noise_metadata_schedule_424_e4059,)
    } else {
        (noise_variable_77,)
    }
};
            noise_variable_77 = noise_metadata_schedule_424_e4061;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_425_e4067: f64 = (2.0 * noise_variable_77);
            let noise_metadata_schedule_425_e4068: f64 = (noise_variable_79 / noise_metadata_schedule_425_e4067);
            let noise_metadata_schedule_425_e4069: f64 = (1.0 - noise_metadata_schedule_425_e4068);
            let noise_metadata_schedule_425_e4070: f64 = (noise_variable_83 * noise_metadata_schedule_425_e4069);
            let noise_metadata_schedule_425_e4072: f64 = if noise_metadata_schedule_425_e4070 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_510 = noise_metadata_schedule_425_e4072;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_426_e4087,) = {
    if ((noise_variable_509 != 0.0) && (noise_variable_510 != 0.0)) {
        let noise_metadata_schedule_426_e4081: f64 = (2.0 * noise_variable_77);
        let noise_metadata_schedule_426_e4082: f64 = (noise_variable_79 / noise_metadata_schedule_426_e4081);
        let noise_metadata_schedule_426_e4083: f64 = (1.0 - noise_metadata_schedule_426_e4082);
        let noise_metadata_schedule_426_e4084: f64 = (noise_variable_83 * noise_metadata_schedule_426_e4083);
        let noise_metadata_schedule_426_e4085: f64 = (noise_metadata_schedule_426_e4084).exp();
        (noise_metadata_schedule_426_e4085,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_426_e4087;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_427_e4095,) = {
    if ((noise_variable_509 != 0.0) && (noise_variable_510 == 0.0)) {
        let noise_metadata_schedule_427_e4093: f64 = (params.p134).exp();
        (noise_metadata_schedule_427_e4093,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_427_e4095;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_428_e4116,) = {
    if ((noise_variable_509 != 0.0) && (noise_variable_510 == 0.0)) {
        let noise_metadata_schedule_428_e4107: f64 = (2.0 * noise_variable_77);
        let noise_metadata_schedule_428_e4108: f64 = (noise_variable_79 / noise_metadata_schedule_428_e4107);
        let noise_metadata_schedule_428_e4109: f64 = (1.0 - noise_metadata_schedule_428_e4108);
        let noise_metadata_schedule_428_e4110: f64 = (noise_variable_83 * noise_metadata_schedule_428_e4109);
        let noise_metadata_schedule_428_e4112: f64 = (noise_metadata_schedule_428_e4110 - params.p134);
        let noise_metadata_schedule_428_e4113: f64 = (1.0 + noise_metadata_schedule_428_e4112);
        let noise_metadata_schedule_428_e4114: f64 = (noise_variable_275 * noise_metadata_schedule_428_e4113);
        (noise_metadata_schedule_428_e4114,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_428_e4116;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_429_e4122,) = {
    if (noise_variable_509 != 0.0) {
        let noise_metadata_schedule_429_e4120: f64 = (noise_variable_230 * noise_variable_67);
        (noise_metadata_schedule_429_e4120,)
    } else {
        (noise_variable_257,)
    }
};
            noise_variable_257 = noise_metadata_schedule_429_e4122;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_430_e4166,) = {
    if (noise_variable_509 != 0.0) {
        let noise_metadata_schedule_430_e4126: f64 = (noise_variable_257 * noise_variable_257);
        let noise_metadata_schedule_430_e4128: f64 = (noise_metadata_schedule_430_e4126 + 1e-30);
        let noise_metadata_schedule_430_e4129: f64 = (noise_metadata_schedule_430_e4128).sqrt();
        let noise_metadata_schedule_430_e4131: f64 = (-2.0);
        let noise_metadata_schedule_430_e4133: f64 = (noise_metadata_schedule_430_e4131 - noise_variable_76);
        let noise_metadata_schedule_430_e4134: f64 = (noise_metadata_schedule_430_e4129).powf(noise_metadata_schedule_430_e4133);
        let noise_metadata_schedule_430_e4139: f64 = (noise_variable_76 * noise_variable_76);
        let noise_metadata_schedule_430_e4140: f64 = (1.0 - noise_metadata_schedule_430_e4139);
        let noise_metadata_schedule_430_e4143: f64 = (3.0 * noise_variable_257);
        let noise_metadata_schedule_430_e4146: f64 = (noise_variable_76 - 1.0);
        let noise_metadata_schedule_430_e4147: f64 = (noise_metadata_schedule_430_e4143 * noise_metadata_schedule_430_e4146);
        let noise_metadata_schedule_430_e4148: f64 = (noise_metadata_schedule_430_e4140 - noise_metadata_schedule_430_e4147);
        let noise_metadata_schedule_430_e4149: f64 = (noise_variable_76 * noise_metadata_schedule_430_e4148);
        let noise_metadata_schedule_430_e4152: f64 = (6.0 * noise_variable_257);
        let noise_metadata_schedule_430_e4154: f64 = (noise_metadata_schedule_430_e4152 * noise_variable_257);
        let noise_metadata_schedule_430_e4157: f64 = (noise_variable_76 - 1.0);
        let noise_metadata_schedule_430_e4159: f64 = (noise_metadata_schedule_430_e4157 + noise_variable_257);
        let noise_metadata_schedule_430_e4160: f64 = (noise_metadata_schedule_430_e4154 * noise_metadata_schedule_430_e4159);
        let noise_metadata_schedule_430_e4161: f64 = (noise_metadata_schedule_430_e4149 - noise_metadata_schedule_430_e4160);
        let noise_metadata_schedule_430_e4162: f64 = (noise_metadata_schedule_430_e4134 * noise_metadata_schedule_430_e4161);
        let noise_metadata_schedule_430_e4164: f64 = (noise_metadata_schedule_430_e4162 * 0.16666666666666666);
        (noise_metadata_schedule_430_e4164,)
    } else {
        (noise_variable_80,)
    }
};
            noise_variable_80 = noise_metadata_schedule_430_e4166;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_431_e4178,) = {
    if (noise_variable_509 != 0.0) {
        let noise_metadata_schedule_431_e4170: f64 = (noise_variable_230 * noise_variable_79);
        let noise_metadata_schedule_431_e4172: f64 = (noise_metadata_schedule_431_e4170 * noise_variable_83);
        let noise_metadata_schedule_431_e4175: f64 = (noise_variable_85 * noise_variable_80);
        let noise_metadata_schedule_431_e4176: f64 = (noise_metadata_schedule_431_e4172 / noise_metadata_schedule_431_e4175);
        (noise_metadata_schedule_431_e4176,)
    } else {
        (noise_variable_257,)
    }
};
            noise_variable_257 = noise_metadata_schedule_431_e4178;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_432_e4181: f64 = (-0.001);
            let noise_metadata_schedule_432_e4182: f64 = if noise_variable_257 < noise_metadata_schedule_432_e4181 { 1.0 } else { 0.0 };
            noise_variable_511 = noise_metadata_schedule_432_e4182;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_433_e4185: f64 = if noise_variable_257 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_512 = noise_metadata_schedule_433_e4185;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_434_e4194,) = {
    if (((noise_variable_509 != 0.0) && (noise_variable_511 != 0.0)) && (noise_variable_512 != 0.0)) {
        let noise_metadata_schedule_434_e4192: f64 = (noise_variable_257).exp();
        (noise_metadata_schedule_434_e4192,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_434_e4194;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_435_e4204,) = {
    if (((noise_variable_509 != 0.0) && (noise_variable_511 != 0.0)) && (noise_variable_512 == 0.0)) {
        let noise_metadata_schedule_435_e4202: f64 = (params.p134).exp();
        (noise_metadata_schedule_435_e4202,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_435_e4204;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_436_e4219,) = {
    if (((noise_variable_509 != 0.0) && (noise_variable_511 != 0.0)) && (noise_variable_512 == 0.0)) {
        let noise_metadata_schedule_436_e4215: f64 = (noise_variable_257 - params.p134);
        let noise_metadata_schedule_436_e4216: f64 = (1.0 + noise_metadata_schedule_436_e4215);
        let noise_metadata_schedule_436_e4217: f64 = (noise_variable_275 * noise_metadata_schedule_436_e4216);
        (noise_metadata_schedule_436_e4217,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_436_e4219;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_437_e4234,) = {
    if ((noise_variable_509 != 0.0) && (noise_variable_511 != 0.0)) {
        let noise_metadata_schedule_437_e4224: f64 = (-noise_variable_230);
        let noise_metadata_schedule_437_e4228: f64 = (1.0 - noise_variable_92);
        let noise_metadata_schedule_437_e4230: f64 = (noise_metadata_schedule_437_e4228 / noise_variable_257);
        let noise_metadata_schedule_437_e4231: f64 = (1.0 + noise_metadata_schedule_437_e4230);
        let noise_metadata_schedule_437_e4232: f64 = (noise_metadata_schedule_437_e4224 * noise_metadata_schedule_437_e4231);
        (noise_metadata_schedule_437_e4232,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_437_e4234;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_438_e4257,) = {
    if ((noise_variable_509 != 0.0) && (noise_variable_511 == 0.0)) {
        let noise_metadata_schedule_438_e4241: f64 = (noise_variable_230 * 0.5);
        let noise_metadata_schedule_438_e4243: f64 = (noise_metadata_schedule_438_e4241 * noise_variable_257);
        let noise_metadata_schedule_438_e4247: f64 = (noise_variable_257 * 0.3333333333333333);
        let noise_metadata_schedule_438_e4251: f64 = (0.25 * noise_variable_257);
        let noise_metadata_schedule_438_e4252: f64 = (1.0 + noise_metadata_schedule_438_e4251);
        let noise_metadata_schedule_438_e4253: f64 = (noise_metadata_schedule_438_e4247 * noise_metadata_schedule_438_e4252);
        let noise_metadata_schedule_438_e4254: f64 = (1.0 + noise_metadata_schedule_438_e4253);
        let noise_metadata_schedule_438_e4255: f64 = (noise_metadata_schedule_438_e4243 * noise_metadata_schedule_438_e4254);
        (noise_metadata_schedule_438_e4255,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_438_e4257;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_439_e4273,) = {
    if (noise_variable_509 != 0.0) {
        let noise_metadata_schedule_439_e4261: f64 = (2.0 * noise_variable_84);
        let noise_metadata_schedule_439_e4263: f64 = (noise_metadata_schedule_439_e4261 * noise_variable_81);
        let noise_metadata_schedule_439_e4265: f64 = (noise_metadata_schedule_439_e4263 * noise_variable_77);
        let noise_metadata_schedule_439_e4267: f64 = (noise_metadata_schedule_439_e4265 * noise_variable_78);
        let noise_metadata_schedule_439_e4269: f64 = (noise_metadata_schedule_439_e4267 * noise_variable_67);
        let noise_metadata_schedule_439_e4271: f64 = (noise_metadata_schedule_439_e4269 * noise_variable_89);
        (noise_metadata_schedule_439_e4271,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_439_e4273;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_441_e4283,) = {
    if (noise_variable_509 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_441_e4283;
        }
        if matches!(source_index, 11 | 12) {
            let noise_metadata_schedule_446_e4310: f64 = (2.0 * noise_variable_43);
            let noise_metadata_schedule_446_e4313: f64 = (noise_variable_248 - 1.0);
            let noise_metadata_schedule_446_e4314: f64 = (noise_metadata_schedule_446_e4310 * noise_metadata_schedule_446_e4313);
            let noise_metadata_schedule_446_e4319: f64 = (4.0 * noise_variable_43);
            let noise_metadata_schedule_446_e4321: f64 = (noise_metadata_schedule_446_e4319 / noise_variable_37);
            let noise_metadata_schedule_446_e4323: f64 = (noise_metadata_schedule_446_e4321 * noise_variable_248);
            let noise_metadata_schedule_446_e4324: f64 = (1.0 + noise_metadata_schedule_446_e4323);
            let noise_metadata_schedule_446_e4325: f64 = (noise_metadata_schedule_446_e4324).sqrt();
            let noise_metadata_schedule_446_e4326: f64 = (1.0 + noise_metadata_schedule_446_e4325);
            let noise_metadata_schedule_446_e4327: f64 = (noise_metadata_schedule_446_e4314 / noise_metadata_schedule_446_e4326);
            noise_variable_157 = noise_metadata_schedule_446_e4327;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_447_e4334: f64 = if ((params.p5 > 0.0) && (params.p32 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_513 = noise_metadata_schedule_447_e4334;
        }
        if matches!(source_index, 11 | 12) {
            let (noise_metadata_schedule_448_e4340,) = {
    if (noise_variable_513 != 0.0) {
        let noise_metadata_schedule_448_e4338: f64 = (noise_variable_157 * noise_variable_150);
        (noise_metadata_schedule_448_e4338,)
    } else {
        (noise_variable_157,)
    }
};
            noise_variable_157 = noise_metadata_schedule_448_e4340;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_449_e4365,) = {
    if (noise_variable_513 != 0.0) {
        let noise_metadata_schedule_449_e4344: f64 = (params.p32 * 2.0);
        let noise_metadata_schedule_449_e4346: f64 = (noise_metadata_schedule_449_e4344 * noise_variable_43);
        let noise_metadata_schedule_449_e4349: f64 = (noise_variable_249 - 1.0);
        let noise_metadata_schedule_449_e4350: f64 = (noise_metadata_schedule_449_e4346 * noise_metadata_schedule_449_e4349);
        let noise_metadata_schedule_449_e4355: f64 = (4.0 * noise_variable_43);
        let noise_metadata_schedule_449_e4357: f64 = (noise_metadata_schedule_449_e4355 / noise_variable_37);
        let noise_metadata_schedule_449_e4359: f64 = (noise_metadata_schedule_449_e4357 * noise_variable_249);
        let noise_metadata_schedule_449_e4360: f64 = (1.0 + noise_metadata_schedule_449_e4359);
        let noise_metadata_schedule_449_e4361: f64 = (noise_metadata_schedule_449_e4360).sqrt();
        let noise_metadata_schedule_449_e4362: f64 = (1.0 + noise_metadata_schedule_449_e4361);
        let noise_metadata_schedule_449_e4363: f64 = (noise_metadata_schedule_449_e4350 / noise_metadata_schedule_449_e4362);
        (noise_metadata_schedule_449_e4363,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_449_e4365;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_450_e4369,) = {
    if (noise_variable_513 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_450_e4369;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_451_e4372: f64 = if params.p5 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_514 = noise_metadata_schedule_451_e4372;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_452_e4382,) = {
    if ((noise_variable_513 != 0.0) && (noise_variable_514 != 0.0)) {
        let noise_metadata_schedule_452_e4378: f64 = (params.p32 * noise_variable_43);
        let noise_metadata_schedule_452_e4380: f64 = (noise_metadata_schedule_452_e4378 * noise_variable_32);
        (noise_metadata_schedule_452_e4380,)
    } else {
        (noise_variable_271,)
    }
};
            noise_variable_271 = noise_metadata_schedule_452_e4382;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_453_e4395,) = {
    if ((noise_variable_513 != 0.0) && (noise_variable_514 != 0.0)) {
        let noise_metadata_schedule_453_e4390: f64 = (noise_variable_271 * noise_variable_8);
        let noise_metadata_schedule_453_e4391: f64 = (noise_metadata_schedule_453_e4390).ln();
        let noise_metadata_schedule_453_e4392: f64 = (2.0 - noise_metadata_schedule_453_e4391);
        let noise_metadata_schedule_453_e4393: f64 = (noise_variable_6 * noise_metadata_schedule_453_e4392);
        (noise_metadata_schedule_453_e4393,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_453_e4395;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_454_e4403,) = {
    if ((noise_variable_513 != 0.0) && (noise_variable_514 != 0.0)) {
        let noise_metadata_schedule_454_e4401: f64 = (noise_variable_241 - noise_variable_166);
        (noise_metadata_schedule_454_e4401,)
    } else {
        (noise_variable_264,)
    }
};
            noise_variable_264 = noise_metadata_schedule_454_e4403;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_455_e4411,) = {
    if ((noise_variable_513 != 0.0) && (noise_variable_514 != 0.0)) {
        let noise_metadata_schedule_455_e4409: f64 = (0.11 * 0.11);
        (noise_metadata_schedule_455_e4409,)
    } else {
        (noise_variable_261,)
    }
};
            noise_variable_261 = noise_metadata_schedule_455_e4411;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_456_e4419,) = {
    if ((noise_variable_513 != 0.0) && (noise_variable_514 != 0.0)) {
        let noise_metadata_schedule_456_e4417: f64 = (noise_variable_264 * noise_variable_264);
        (noise_metadata_schedule_456_e4417,)
    } else {
        (noise_variable_262,)
    }
};
            noise_variable_262 = noise_metadata_schedule_456_e4419;
        }
        if matches!(source_index, 13 | 14) {
            let noise_metadata_schedule_457_e4422: f64 = if noise_variable_264 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_515 = noise_metadata_schedule_457_e4422;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_458_e4439,) = {
    if (((noise_variable_513 != 0.0) && (noise_variable_514 != 0.0)) && (noise_variable_515 != 0.0)) {
        let noise_metadata_schedule_458_e4430: f64 = (0.5 * noise_variable_261);
        let noise_metadata_schedule_458_e4433: f64 = (noise_variable_262 + noise_variable_261);
        let noise_metadata_schedule_458_e4434: f64 = (noise_metadata_schedule_458_e4433).sqrt();
        let noise_metadata_schedule_458_e4436: f64 = (noise_metadata_schedule_458_e4434 - noise_variable_264);
        let noise_metadata_schedule_458_e4437: f64 = (noise_metadata_schedule_458_e4430 / noise_metadata_schedule_458_e4436);
        (noise_metadata_schedule_458_e4437,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_458_e4439;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_459_e4455,) = {
    if (((noise_variable_513 != 0.0) && (noise_variable_514 != 0.0)) && (noise_variable_515 == 0.0)) {
        let noise_metadata_schedule_459_e4449: f64 = (noise_variable_262 + noise_variable_261);
        let noise_metadata_schedule_459_e4450: f64 = (noise_metadata_schedule_459_e4449).sqrt();
        let noise_metadata_schedule_459_e4452: f64 = (noise_metadata_schedule_459_e4450 + noise_variable_264);
        let noise_metadata_schedule_459_e4453: f64 = (0.5 * noise_metadata_schedule_459_e4452);
        (noise_metadata_schedule_459_e4453,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_459_e4455;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_460_e4471,) = {
    if ((noise_variable_513 != 0.0) && (noise_variable_514 != 0.0)) {
        let noise_metadata_schedule_460_e4463: f64 = (noise_variable_164 + noise_variable_165);
        let noise_metadata_schedule_460_e4465: f64 = (noise_metadata_schedule_460_e4463 * noise_variable_32);
        let noise_metadata_schedule_460_e4466: f64 = (noise_variable_271 + noise_metadata_schedule_460_e4465);
        let noise_metadata_schedule_460_e4468: f64 = (noise_metadata_schedule_460_e4466 + noise_variable_167);
        let noise_metadata_schedule_460_e4469: f64 = (noise_variable_167 / noise_metadata_schedule_460_e4468);
        (noise_metadata_schedule_460_e4469,)
    } else {
        (noise_variable_168,)
    }
};
            noise_variable_168 = noise_metadata_schedule_460_e4471;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_464_e4499,) = {
    if ((noise_variable_513 != 0.0) && (noise_variable_514 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_168,)
    }
};
            noise_variable_168 = noise_metadata_schedule_464_e4499;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_465_e4505,) = {
    if (noise_variable_513 != 0.0) {
        let noise_metadata_schedule_465_e4503: f64 = (noise_variable_168 * noise_variable_164);
        (noise_metadata_schedule_465_e4503,)
    } else {
        (noise_variable_169,)
    }
};
            noise_variable_169 = noise_metadata_schedule_465_e4505;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_466_e4508: f64 = if params.p83 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_516 = noise_metadata_schedule_466_e4508;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_467_e4514,) = {
    if (noise_variable_516 != 0.0) {
        let noise_metadata_schedule_467_e4512: f64 = (noise_variable_234 + noise_variable_230);
        (noise_metadata_schedule_467_e4512,)
    } else {
        (noise_variable_322,)
    }
};
            noise_variable_322 = noise_metadata_schedule_467_e4514;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_468_e4520,) = {
    if (noise_variable_516 != 0.0) {
        let noise_metadata_schedule_468_e4518: f64 = (1e-6 * 1e-6);
        (noise_metadata_schedule_468_e4518,)
    } else {
        (noise_variable_261,)
    }
};
            noise_variable_261 = noise_metadata_schedule_468_e4520;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_469_e4532,) = {
    if (noise_variable_516 != 0.0) {
        let noise_metadata_schedule_469_e4523: f64 = (-1.0);
        let noise_metadata_schedule_469_e4525: f64 = (noise_metadata_schedule_469_e4523 * noise_variable_322);
        let noise_metadata_schedule_469_e4527: f64 = (-1.0);
        let noise_metadata_schedule_469_e4528: f64 = (noise_metadata_schedule_469_e4525 * noise_metadata_schedule_469_e4527);
        let noise_metadata_schedule_469_e4530: f64 = (noise_metadata_schedule_469_e4528 * noise_variable_322);
        (noise_metadata_schedule_469_e4530,)
    } else {
        (noise_variable_262,)
    }
};
            noise_variable_262 = noise_metadata_schedule_469_e4532;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_470_e4534: f64 = (-1.0);
            let noise_metadata_schedule_470_e4536: f64 = (noise_metadata_schedule_470_e4534 * noise_variable_322);
            let noise_metadata_schedule_470_e4538: f64 = if noise_metadata_schedule_470_e4536 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_517 = noise_metadata_schedule_470_e4538;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_471_e4556,) = {
    if ((noise_variable_516 != 0.0) && (noise_variable_517 != 0.0)) {
        let noise_metadata_schedule_471_e4544: f64 = (0.5 * noise_variable_261);
        let noise_metadata_schedule_471_e4547: f64 = (noise_variable_262 + noise_variable_261);
        let noise_metadata_schedule_471_e4548: f64 = (noise_metadata_schedule_471_e4547).sqrt();
        let noise_metadata_schedule_471_e4550: f64 = (-1.0);
        let noise_metadata_schedule_471_e4552: f64 = (noise_metadata_schedule_471_e4550 * noise_variable_322);
        let noise_metadata_schedule_471_e4553: f64 = (noise_metadata_schedule_471_e4548 - noise_metadata_schedule_471_e4552);
        let noise_metadata_schedule_471_e4554: f64 = (noise_metadata_schedule_471_e4544 / noise_metadata_schedule_471_e4553);
        (noise_metadata_schedule_471_e4554,)
    } else {
        (noise_variable_323,)
    }
};
            noise_variable_323 = noise_metadata_schedule_471_e4556;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_472_e4573,) = {
    if ((noise_variable_516 != 0.0) && (noise_variable_517 == 0.0)) {
        let noise_metadata_schedule_472_e4564: f64 = (noise_variable_262 + noise_variable_261);
        let noise_metadata_schedule_472_e4565: f64 = (noise_metadata_schedule_472_e4564).sqrt();
        let noise_metadata_schedule_472_e4567: f64 = (-1.0);
        let noise_metadata_schedule_472_e4569: f64 = (noise_metadata_schedule_472_e4567 * noise_variable_322);
        let noise_metadata_schedule_472_e4570: f64 = (noise_metadata_schedule_472_e4565 + noise_metadata_schedule_472_e4569);
        let noise_metadata_schedule_472_e4571: f64 = (0.5 * noise_metadata_schedule_472_e4570);
        (noise_metadata_schedule_472_e4571,)
    } else {
        (noise_variable_323,)
    }
};
            noise_variable_323 = noise_metadata_schedule_472_e4573;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_473_e4583,) = {
    if (noise_variable_516 != 0.0) {
        let noise_metadata_schedule_473_e4579: f64 = (noise_variable_318).powf(params.p81);
        let noise_metadata_schedule_473_e4580: f64 = (1.0 - noise_metadata_schedule_473_e4579);
        let noise_metadata_schedule_473_e4581: f64 = (1.0 / noise_metadata_schedule_473_e4580);
        (noise_metadata_schedule_473_e4581,)
    } else {
        (noise_variable_324,)
    }
};
            noise_variable_324 = noise_metadata_schedule_473_e4583;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_474_e4589,) = {
    if (noise_variable_516 != 0.0) {
        let noise_metadata_schedule_474_e4587: f64 = (noise_variable_318 * params.p80);
        (noise_metadata_schedule_474_e4587,)
    } else {
        (noise_variable_319,)
    }
};
            noise_variable_319 = noise_metadata_schedule_474_e4589;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_475_e4605,) = {
    if (noise_variable_516 != 0.0) {
        let noise_metadata_schedule_475_e4593: f64 = (noise_variable_324 * noise_variable_324);
        let noise_metadata_schedule_475_e4597: f64 = (params.p81 - 1.0);
        let noise_metadata_schedule_475_e4598: f64 = (noise_variable_318).powf(noise_metadata_schedule_475_e4597);
        let noise_metadata_schedule_475_e4599: f64 = (noise_metadata_schedule_475_e4593 * noise_metadata_schedule_475_e4598);
        let noise_metadata_schedule_475_e4601: f64 = (noise_metadata_schedule_475_e4599 * params.p81);
        let noise_metadata_schedule_475_e4603: f64 = (noise_metadata_schedule_475_e4601 / params.p80);
        (noise_metadata_schedule_475_e4603,)
    } else {
        (noise_variable_321,)
    }
};
            noise_variable_321 = noise_metadata_schedule_475_e4605;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_476_e4608: f64 = if noise_variable_323 < noise_variable_319 { 1.0 } else { 0.0 };
            noise_variable_518 = noise_metadata_schedule_476_e4608;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_477_e4622,) = {
    if ((noise_variable_516 != 0.0) && (noise_variable_518 != 0.0)) {
        let noise_metadata_schedule_477_e4616: f64 = (noise_variable_323 / params.p80);
        let noise_metadata_schedule_477_e4618: f64 = (noise_metadata_schedule_477_e4616).powf(params.p81);
        let noise_metadata_schedule_477_e4619: f64 = (1.0 - noise_metadata_schedule_477_e4618);
        let noise_metadata_schedule_477_e4620: f64 = (1.0 / noise_metadata_schedule_477_e4619);
        (noise_metadata_schedule_477_e4620,)
    } else {
        (noise_variable_320,)
    }
};
            noise_variable_320 = noise_metadata_schedule_477_e4622;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_478_e4635,) = {
    if ((noise_variable_516 != 0.0) && (noise_variable_518 == 0.0)) {
        let noise_metadata_schedule_478_e4630: f64 = (noise_variable_323 - noise_variable_319);
        let noise_metadata_schedule_478_e4632: f64 = (noise_metadata_schedule_478_e4630 * noise_variable_321);
        let noise_metadata_schedule_478_e4633: f64 = (noise_variable_324 + noise_metadata_schedule_478_e4632);
        (noise_metadata_schedule_478_e4633,)
    } else {
        (noise_variable_320,)
    }
};
            noise_variable_320 = noise_metadata_schedule_478_e4635;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_479_e4640,) = {
    if (noise_variable_516 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_320,)
    }
};
            noise_variable_320 = noise_metadata_schedule_479_e4640;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_480_e4643: f64 = (noise_variable_82 * noise_variable_320);
            noise_variable_82 = noise_metadata_schedule_480_e4643;
        }
        if matches!(source_index, 11 | 12) {
            let noise_metadata_schedule_481_e4646: f64 = (noise_variable_157 * noise_variable_320);
            noise_variable_157 = noise_metadata_schedule_481_e4646;
        }
        if matches!(source_index, 9 | 10) {
            let noise_metadata_schedule_482_e4649: f64 = (noise_variable_154 * noise_variable_320);
            noise_variable_154 = noise_metadata_schedule_482_e4649;
        }
        if matches!(source_index, 13 | 14) {
            let noise_metadata_schedule_483_e4652: f64 = (noise_variable_169 * noise_variable_320);
            noise_variable_169 = noise_metadata_schedule_483_e4652;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_484_e4656: f64 = (noise_variable_131 / noise_variable_41);
            let noise_metadata_schedule_484_e4657: f64 = (1.0 + noise_metadata_schedule_484_e4656);
            let noise_metadata_schedule_484_e4660: f64 = (noise_variable_138 / noise_variable_40);
            let noise_metadata_schedule_484_e4661: f64 = (noise_metadata_schedule_484_e4657 + noise_metadata_schedule_484_e4660);
            noise_variable_172 = noise_metadata_schedule_484_e4661;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_485_e4664: f64 = (0.1 * 0.1);
            noise_variable_261 = noise_metadata_schedule_485_e4664;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_486_e4667: f64 = (noise_variable_172 * noise_variable_172);
            noise_variable_262 = noise_metadata_schedule_486_e4667;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_487_e4670: f64 = if noise_variable_172 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_519 = noise_metadata_schedule_487_e4670;
        }
        if matches!(source_index, 1 | 5) {
            let (noise_metadata_schedule_488_e4683,) = {
    if (noise_variable_519 != 0.0) {
        let noise_metadata_schedule_488_e4674: f64 = (0.5 * noise_variable_261);
        let noise_metadata_schedule_488_e4677: f64 = (noise_variable_262 + noise_variable_261);
        let noise_metadata_schedule_488_e4678: f64 = (noise_metadata_schedule_488_e4677).sqrt();
        let noise_metadata_schedule_488_e4680: f64 = (noise_metadata_schedule_488_e4678 - noise_variable_172);
        let noise_metadata_schedule_488_e4681: f64 = (noise_metadata_schedule_488_e4674 / noise_metadata_schedule_488_e4680);
        (noise_metadata_schedule_488_e4681,)
    } else {
        (noise_variable_173,)
    }
};
            noise_variable_173 = noise_metadata_schedule_488_e4683;
        }
        if matches!(source_index, 1 | 5) {
            let (noise_metadata_schedule_489_e4695,) = {
    if (noise_variable_519 == 0.0) {
        let noise_metadata_schedule_489_e4689: f64 = (noise_variable_262 + noise_variable_261);
        let noise_metadata_schedule_489_e4690: f64 = (noise_metadata_schedule_489_e4689).sqrt();
        let noise_metadata_schedule_489_e4692: f64 = (noise_metadata_schedule_489_e4690 + noise_variable_172);
        let noise_metadata_schedule_489_e4693: f64 = (0.5 * noise_metadata_schedule_489_e4692);
        (noise_metadata_schedule_489_e4693,)
    } else {
        (noise_variable_173,)
    }
};
            noise_variable_173 = noise_metadata_schedule_489_e4695;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_490_e4701: f64 = (noise_variable_142 + noise_variable_143);
            let noise_metadata_schedule_490_e4702: f64 = (0.5 * noise_metadata_schedule_490_e4701);
            let noise_metadata_schedule_490_e4703: f64 = (1.0 + noise_metadata_schedule_490_e4702);
            let noise_metadata_schedule_490_e4704: f64 = (noise_variable_173 * noise_metadata_schedule_490_e4703);
            noise_variable_174 = noise_metadata_schedule_490_e4704;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_491_e4707: f64 = (noise_variable_29 / noise_variable_174);
            noise_variable_176 = noise_metadata_schedule_491_e4707;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_492_e4710: f64 = if noise_variable_176 < noise_variable_316 { 1.0 } else { 0.0 };
            noise_variable_520 = noise_metadata_schedule_492_e4710;
        }
        if matches!(source_index, 1 | 5) {
            let (noise_metadata_schedule_493_e4714,) = {
    if (noise_variable_520 != 0.0) {
        (noise_variable_316,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_493_e4714;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_494_e4717: f64 = (3.0 * noise_variable_176);
            noise_variable_175 = noise_metadata_schedule_494_e4717;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_496_e4731: f64 = if noise_variable_149 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_521 = noise_metadata_schedule_496_e4731;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_497_e4734: f64 = if params.p38 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_522 = noise_metadata_schedule_497_e4734;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_498_e4737: f64 = if noise_variable_230 < params.p43 { 1.0 } else { 0.0 };
            noise_variable_523 = noise_metadata_schedule_498_e4737;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_499_e4739: f64 = (-noise_variable_149);
            let noise_metadata_schedule_499_e4741: f64 = (noise_metadata_schedule_499_e4739 / params.p41);
            let noise_metadata_schedule_499_e4743: f64 = if noise_metadata_schedule_499_e4741 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_524 = noise_metadata_schedule_499_e4743;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_500_e4757,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 != 0.0)) && (noise_variable_523 != 0.0)) && (noise_variable_524 != 0.0)) {
        let noise_metadata_schedule_500_e4752: f64 = (-noise_variable_149);
        let noise_metadata_schedule_500_e4754: f64 = (noise_metadata_schedule_500_e4752 / params.p41);
        let noise_metadata_schedule_500_e4755: f64 = (noise_metadata_schedule_500_e4754).exp();
        (noise_metadata_schedule_500_e4755,)
    } else {
        (noise_variable_308,)
    }
};
            noise_variable_308 = noise_metadata_schedule_500_e4757;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_501_e4769,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 != 0.0)) && (noise_variable_523 != 0.0)) && (noise_variable_524 == 0.0)) {
        let noise_metadata_schedule_501_e4767: f64 = (params.p134).exp();
        (noise_metadata_schedule_501_e4767,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_501_e4769;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_502_e4789,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 != 0.0)) && (noise_variable_523 != 0.0)) && (noise_variable_524 == 0.0)) {
        let noise_metadata_schedule_502_e4781: f64 = (-noise_variable_149);
        let noise_metadata_schedule_502_e4783: f64 = (noise_metadata_schedule_502_e4781 / params.p41);
        let noise_metadata_schedule_502_e4785: f64 = (noise_metadata_schedule_502_e4783 - params.p134);
        let noise_metadata_schedule_502_e4786: f64 = (1.0 + noise_metadata_schedule_502_e4785);
        let noise_metadata_schedule_502_e4787: f64 = (noise_variable_275 * noise_metadata_schedule_502_e4786);
        (noise_metadata_schedule_502_e4787,)
    } else {
        (noise_variable_308,)
    }
};
            noise_variable_308 = noise_metadata_schedule_502_e4789;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_503_e4801,) = {
    if (((noise_variable_521 != 0.0) && (noise_variable_522 != 0.0)) && (noise_variable_523 != 0.0)) {
        let noise_metadata_schedule_503_e4797: f64 = (params.p43 - noise_variable_230);
        let noise_metadata_schedule_503_e4799: f64 = (noise_metadata_schedule_503_e4797 * noise_variable_308);
        (noise_metadata_schedule_503_e4799,)
    } else {
        (noise_variable_309,)
    }
};
            noise_variable_309 = noise_metadata_schedule_503_e4801;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_504_e4803: f64 = (-noise_variable_310);
            let noise_metadata_schedule_504_e4806: f64 = (noise_variable_309).powf(params.p40);
            let noise_metadata_schedule_504_e4807: f64 = (noise_metadata_schedule_504_e4803 * noise_metadata_schedule_504_e4806);
            let noise_metadata_schedule_504_e4809: f64 = if noise_metadata_schedule_504_e4807 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_525 = noise_metadata_schedule_504_e4809;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_505_e4825,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 != 0.0)) && (noise_variable_523 != 0.0)) && (noise_variable_525 != 0.0)) {
        let noise_metadata_schedule_505_e4818: f64 = (-noise_variable_310);
        let noise_metadata_schedule_505_e4821: f64 = (noise_variable_309).powf(params.p40);
        let noise_metadata_schedule_505_e4822: f64 = (noise_metadata_schedule_505_e4818 * noise_metadata_schedule_505_e4821);
        let noise_metadata_schedule_505_e4823: f64 = (noise_metadata_schedule_505_e4822).exp();
        (noise_metadata_schedule_505_e4823,)
    } else {
        (noise_variable_313,)
    }
};
            noise_variable_313 = noise_metadata_schedule_505_e4825;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_506_e4837,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 != 0.0)) && (noise_variable_523 != 0.0)) && (noise_variable_525 == 0.0)) {
        let noise_metadata_schedule_506_e4835: f64 = (params.p134).exp();
        (noise_metadata_schedule_506_e4835,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_506_e4837;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_507_e4859,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 != 0.0)) && (noise_variable_523 != 0.0)) && (noise_variable_525 == 0.0)) {
        let noise_metadata_schedule_507_e4849: f64 = (-noise_variable_310);
        let noise_metadata_schedule_507_e4852: f64 = (noise_variable_309).powf(params.p40);
        let noise_metadata_schedule_507_e4853: f64 = (noise_metadata_schedule_507_e4849 * noise_metadata_schedule_507_e4852);
        let noise_metadata_schedule_507_e4855: f64 = (noise_metadata_schedule_507_e4853 - params.p134);
        let noise_metadata_schedule_507_e4856: f64 = (1.0 + noise_metadata_schedule_507_e4855);
        let noise_metadata_schedule_507_e4857: f64 = (noise_variable_275 * noise_metadata_schedule_507_e4856);
        (noise_metadata_schedule_507_e4857,)
    } else {
        (noise_variable_313,)
    }
};
            noise_variable_313 = noise_metadata_schedule_507_e4859;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_508_e4873,) = {
    if (((noise_variable_521 != 0.0) && (noise_variable_522 != 0.0)) && (noise_variable_523 != 0.0)) {
        let noise_metadata_schedule_508_e4867: f64 = (params.p39 / noise_variable_310);
        let noise_metadata_schedule_508_e4869: f64 = (noise_metadata_schedule_508_e4867 * noise_variable_309);
        let noise_metadata_schedule_508_e4871: f64 = (noise_metadata_schedule_508_e4869 * noise_variable_313);
        (noise_metadata_schedule_508_e4871,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_508_e4873;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_509_e4876: f64 = if params.p38 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_526 = noise_metadata_schedule_509_e4876;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_510_e4879: f64 = if noise_variable_230 < noise_variable_16 { 1.0 } else { 0.0 };
            noise_variable_527 = noise_metadata_schedule_510_e4879;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_511_e4896,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) {
        let noise_metadata_schedule_511_e4890: f64 = (2.0 * params.p45);
        let noise_metadata_schedule_511_e4893: f64 = (params.p44 * params.p44);
        let noise_metadata_schedule_511_e4894: f64 = (noise_metadata_schedule_511_e4890 / noise_metadata_schedule_511_e4893);
        (noise_metadata_schedule_511_e4894,)
    } else {
        (noise_variable_185,)
    }
};
            noise_variable_185 = noise_metadata_schedule_511_e4896;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_512_e4911,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) {
        let noise_metadata_schedule_512_e4907: f64 = (noise_variable_16 - noise_variable_230);
        let noise_metadata_schedule_512_e4909: f64 = (noise_metadata_schedule_512_e4907 / noise_variable_199);
        (noise_metadata_schedule_512_e4909,)
    } else {
        (noise_variable_260,)
    }
};
            noise_variable_260 = noise_metadata_schedule_512_e4911;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_513_e4927,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) {
        let noise_metadata_schedule_513_e4922: f64 = (2.0 * noise_variable_260);
        let noise_metadata_schedule_513_e4924: f64 = (noise_metadata_schedule_513_e4922 / noise_variable_185);
        let noise_metadata_schedule_513_e4925: f64 = (noise_metadata_schedule_513_e4924).sqrt();
        (noise_metadata_schedule_513_e4925,)
    } else {
        (noise_variable_186,)
    }
};
            noise_variable_186 = noise_metadata_schedule_513_e4927;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_514_e4930: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_528 = noise_metadata_schedule_514_e4930;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_515_e4943,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) && (noise_variable_528 != 0.0)) {
        (params.p44,)
    } else {
        (noise_variable_187,)
    }
};
            noise_variable_187 = noise_metadata_schedule_515_e4943;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_516_e4961,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) && (noise_variable_528 == 0.0)) {
        let noise_metadata_schedule_516_e4958: f64 = (0.5 * noise_variable_115);
        let noise_metadata_schedule_516_e4959: f64 = (1.0 - noise_metadata_schedule_516_e4958);
        (noise_metadata_schedule_516_e4959,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_516_e4961;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_517_e4979,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) && (noise_variable_528 == 0.0)) {
        let noise_metadata_schedule_517_e4975: f64 = (params.p44 * noise_variable_116);
        let noise_metadata_schedule_517_e4977: f64 = (noise_metadata_schedule_517_e4975 * noise_variable_116);
        (noise_metadata_schedule_517_e4977,)
    } else {
        (noise_variable_187,)
    }
};
            noise_variable_187 = noise_metadata_schedule_517_e4979;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_518_e5001,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) {
        let noise_metadata_schedule_518_e4990: f64 = (noise_variable_186 * noise_variable_187);
        let noise_metadata_schedule_518_e4993: f64 = (noise_variable_186 * noise_variable_186);
        let noise_metadata_schedule_518_e4996: f64 = (noise_variable_187 * noise_variable_187);
        let noise_metadata_schedule_518_e4997: f64 = (noise_metadata_schedule_518_e4993 + noise_metadata_schedule_518_e4996);
        let noise_metadata_schedule_518_e4998: f64 = (noise_metadata_schedule_518_e4997).sqrt();
        let noise_metadata_schedule_518_e4999: f64 = (noise_metadata_schedule_518_e4990 / noise_metadata_schedule_518_e4998);
        (noise_metadata_schedule_518_e4999,)
    } else {
        (noise_variable_188,)
    }
};
            noise_variable_188 = noise_metadata_schedule_518_e5001;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_519_e5016,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) {
        let noise_metadata_schedule_519_e5012: f64 = (noise_variable_16 - noise_variable_230);
        let noise_metadata_schedule_519_e5014: f64 = (noise_metadata_schedule_519_e5012 / noise_variable_188);
        (noise_metadata_schedule_519_e5014,)
    } else {
        (noise_variable_189,)
    }
};
            noise_variable_189 = noise_metadata_schedule_519_e5016;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_520_e5035,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) {
        let noise_metadata_schedule_520_e5028: f64 = (0.5 * noise_variable_188);
        let noise_metadata_schedule_520_e5030: f64 = (noise_metadata_schedule_520_e5028 * noise_variable_185);
        let noise_metadata_schedule_520_e5032: f64 = (noise_metadata_schedule_520_e5030 * noise_variable_199);
        let noise_metadata_schedule_520_e5033: f64 = (noise_variable_189 + noise_metadata_schedule_520_e5032);
        (noise_metadata_schedule_520_e5033,)
    } else {
        (noise_variable_190,)
    }
};
            noise_variable_190 = noise_metadata_schedule_520_e5035;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_521_e5038: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_529 = noise_metadata_schedule_521_e5038;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_522_e5051,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) && (noise_variable_529 != 0.0)) {
        (noise_variable_190,)
    } else {
        (noise_variable_191,)
    }
};
            noise_variable_191 = noise_metadata_schedule_522_e5051;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_523_e5075,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) && (noise_variable_529 == 0.0)) {
        let noise_metadata_schedule_523_e5066: f64 = (2.0 * params.p46);
        let noise_metadata_schedule_523_e5070: f64 = (2.0 * noise_variable_115);
        let noise_metadata_schedule_523_e5071: f64 = (1.0 + noise_metadata_schedule_523_e5070);
        let noise_metadata_schedule_523_e5072: f64 = (noise_metadata_schedule_523_e5066 * noise_metadata_schedule_523_e5071);
        let noise_metadata_schedule_523_e5073: f64 = (1.0 + noise_metadata_schedule_523_e5072);
        (noise_metadata_schedule_523_e5073,)
    } else {
        (noise_variable_192,)
    }
};
            noise_variable_192 = noise_metadata_schedule_523_e5075;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_524_e5097,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) && (noise_variable_529 == 0.0)) {
        let noise_metadata_schedule_524_e5089: f64 = (1.0 + params.p46);
        let noise_metadata_schedule_524_e5093: f64 = (2.0 * params.p46);
        let noise_metadata_schedule_524_e5094: f64 = (1.0 + noise_metadata_schedule_524_e5093);
        let noise_metadata_schedule_524_e5095: f64 = (noise_metadata_schedule_524_e5089 / noise_metadata_schedule_524_e5094);
        (noise_metadata_schedule_524_e5095,)
    } else {
        (noise_variable_193,)
    }
};
            noise_variable_193 = noise_metadata_schedule_524_e5097;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_525_e5125,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) && (noise_variable_529 == 0.0)) {
        let noise_metadata_schedule_525_e5112: f64 = (0.5 * noise_variable_188);
        let noise_metadata_schedule_525_e5114: f64 = (noise_metadata_schedule_525_e5112 * noise_variable_185);
        let noise_metadata_schedule_525_e5119: f64 = (params.p61 * noise_variable_192);
        let noise_metadata_schedule_525_e5120: f64 = (noise_variable_149 / noise_metadata_schedule_525_e5119);
        let noise_metadata_schedule_525_e5121: f64 = (noise_variable_193 - noise_metadata_schedule_525_e5120);
        let noise_metadata_schedule_525_e5122: f64 = (noise_metadata_schedule_525_e5114 * noise_metadata_schedule_525_e5121);
        let noise_metadata_schedule_525_e5123: f64 = (noise_variable_189 - noise_metadata_schedule_525_e5122);
        (noise_metadata_schedule_525_e5123,)
    } else {
        (noise_variable_194,)
    }
};
            noise_variable_194 = noise_metadata_schedule_525_e5125;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_526_e5155,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) && (noise_variable_529 == 0.0)) {
        let noise_metadata_schedule_526_e5139: f64 = (noise_variable_194 - noise_variable_190);
        let noise_metadata_schedule_526_e5142: f64 = (noise_variable_194 - noise_variable_190);
        let noise_metadata_schedule_526_e5143: f64 = (noise_metadata_schedule_526_e5139 * noise_metadata_schedule_526_e5142);
        let noise_metadata_schedule_526_e5146: f64 = (0.1 * noise_variable_189);
        let noise_metadata_schedule_526_e5148: f64 = (noise_metadata_schedule_526_e5146 * noise_variable_189);
        let noise_metadata_schedule_526_e5150: f64 = (noise_metadata_schedule_526_e5148 * noise_variable_127);
        let noise_metadata_schedule_526_e5152: f64 = (noise_metadata_schedule_526_e5150 / params.p61);
        let noise_metadata_schedule_526_e5153: f64 = (noise_metadata_schedule_526_e5143 + noise_metadata_schedule_526_e5152);
        (noise_metadata_schedule_526_e5153,)
    } else {
        (noise_variable_260,)
    }
};
            noise_variable_260 = noise_metadata_schedule_526_e5155;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_527_e5176,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) && (noise_variable_529 == 0.0)) {
        let noise_metadata_schedule_527_e5170: f64 = (noise_variable_194 + noise_variable_190);
        let noise_metadata_schedule_527_e5172: f64 = (noise_variable_260).sqrt();
        let noise_metadata_schedule_527_e5173: f64 = (noise_metadata_schedule_527_e5170 + noise_metadata_schedule_527_e5172);
        let noise_metadata_schedule_527_e5174: f64 = (0.5 * noise_metadata_schedule_527_e5173);
        (noise_metadata_schedule_527_e5174,)
    } else {
        (noise_variable_191,)
    }
};
            noise_variable_191 = noise_metadata_schedule_527_e5176;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_528_e5191,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) {
        let noise_metadata_schedule_528_e5187: f64 = (noise_variable_191 - noise_variable_189);
        let noise_metadata_schedule_528_e5189: f64 = (noise_metadata_schedule_528_e5187 / noise_variable_191);
        (noise_metadata_schedule_528_e5189,)
    } else {
        (noise_variable_267,)
    }
};
            noise_variable_267 = noise_metadata_schedule_528_e5191;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_529_e5193: f64 = (noise_variable_267).abs();
            let noise_metadata_schedule_529_e5195: f64 = if noise_metadata_schedule_529_e5193 > 1e-7 { 1.0 } else { 0.0 };
            noise_variable_530 = noise_metadata_schedule_529_e5195;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_530_e5212,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) && (noise_variable_530 != 0.0)) {
        let noise_metadata_schedule_530_e5208: f64 = (0.5 * noise_variable_188);
        let noise_metadata_schedule_530_e5210: f64 = (noise_metadata_schedule_530_e5208 / noise_variable_267);
        (noise_metadata_schedule_530_e5210,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_530_e5212;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_531_e5249,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) && (noise_variable_530 != 0.0)) {
        let noise_metadata_schedule_531_e5225: f64 = (noise_variable_0 / noise_variable_98);
        let noise_metadata_schedule_531_e5227: f64 = (noise_metadata_schedule_531_e5225 * noise_variable_191);
        let noise_metadata_schedule_531_e5229: f64 = (noise_metadata_schedule_531_e5227 * noise_variable_195);
        let noise_metadata_schedule_531_e5231: f64 = (-noise_variable_98);
        let noise_metadata_schedule_531_e5233: f64 = (noise_metadata_schedule_531_e5231 / noise_variable_191);
        let noise_metadata_schedule_531_e5234: f64 = (noise_metadata_schedule_531_e5233).exp();
        let noise_metadata_schedule_531_e5236: f64 = (-noise_variable_98);
        let noise_metadata_schedule_531_e5238: f64 = (noise_metadata_schedule_531_e5236 / noise_variable_191);
        let noise_metadata_schedule_531_e5242: f64 = (noise_variable_187 / noise_variable_195);
        let noise_metadata_schedule_531_e5243: f64 = (1.0 + noise_metadata_schedule_531_e5242);
        let noise_metadata_schedule_531_e5244: f64 = (noise_metadata_schedule_531_e5238 * noise_metadata_schedule_531_e5243);
        let noise_metadata_schedule_531_e5245: f64 = (noise_metadata_schedule_531_e5244).exp();
        let noise_metadata_schedule_531_e5246: f64 = (noise_metadata_schedule_531_e5234 - noise_metadata_schedule_531_e5245);
        let noise_metadata_schedule_531_e5247: f64 = (noise_metadata_schedule_531_e5229 * noise_metadata_schedule_531_e5246);
        (noise_metadata_schedule_531_e5247,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_531_e5249;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_532_e5271,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 != 0.0)) && (noise_variable_527 != 0.0)) && (noise_variable_530 == 0.0)) {
        let noise_metadata_schedule_532_e5263: f64 = (noise_variable_0 * noise_variable_187);
        let noise_metadata_schedule_532_e5265: f64 = (-noise_variable_98);
        let noise_metadata_schedule_532_e5267: f64 = (noise_metadata_schedule_532_e5265 / noise_variable_191);
        let noise_metadata_schedule_532_e5268: f64 = (noise_metadata_schedule_532_e5267).exp();
        let noise_metadata_schedule_532_e5269: f64 = (noise_metadata_schedule_532_e5263 * noise_metadata_schedule_532_e5268);
        (noise_metadata_schedule_532_e5269,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_532_e5271;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_533_e5274: f64 = if params.p38 == 3.0 { 1.0 } else { 0.0 };
            noise_variable_531 = noise_metadata_schedule_533_e5274;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_534_e5277: f64 = if noise_variable_230 < params.p43 { 1.0 } else { 0.0 };
            noise_variable_532 = noise_metadata_schedule_534_e5277;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_535_e5305,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 == 0.0)) && (noise_variable_531 != 0.0)) && (noise_variable_532 != 0.0)) {
        let noise_metadata_schedule_535_e5291: f64 = (params.p43 - noise_variable_230);
        let noise_metadata_schedule_535_e5293: f64 = (noise_metadata_schedule_535_e5291).powf(params.p40);
        let noise_metadata_schedule_535_e5298: f64 = (params.p47 + noise_variable_149);
        let noise_metadata_schedule_535_e5299: f64 = (noise_variable_149 / noise_metadata_schedule_535_e5298);
        let noise_metadata_schedule_535_e5300: f64 = (1.0 - noise_metadata_schedule_535_e5299);
        let noise_metadata_schedule_535_e5302: f64 = (noise_metadata_schedule_535_e5300).powf(params.p48);
        let noise_metadata_schedule_535_e5303: f64 = (noise_metadata_schedule_535_e5293 * noise_metadata_schedule_535_e5302);
        (noise_metadata_schedule_535_e5303,)
    } else {
        (noise_variable_200,)
    }
};
            noise_variable_200 = noise_metadata_schedule_535_e5305;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_536_e5308: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_533 = noise_metadata_schedule_536_e5308;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_537_e5324,) = {
    if ((((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 == 0.0)) && (noise_variable_531 != 0.0)) && (noise_variable_532 != 0.0)) && (noise_variable_533 != 0.0)) {
        (noise_variable_200,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_537_e5324;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_538_e5345,) = {
    if ((((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 == 0.0)) && (noise_variable_531 != 0.0)) && (noise_variable_532 != 0.0)) && (noise_variable_533 == 0.0)) {
        let noise_metadata_schedule_538_e5341: f64 = (noise_variable_149 - params.p51);
        let noise_metadata_schedule_538_e5343: f64 = (noise_metadata_schedule_538_e5341 / params.p47);
        (noise_metadata_schedule_538_e5343,)
    } else {
        (noise_variable_202,)
    }
};
            noise_variable_202 = noise_metadata_schedule_538_e5345;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_539_e5366,) = {
    if ((((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 == 0.0)) && (noise_variable_531 != 0.0)) && (noise_variable_532 != 0.0)) && (noise_variable_533 == 0.0)) {
        let noise_metadata_schedule_539_e5362: f64 = (noise_variable_202 - 1.0);
        let noise_metadata_schedule_539_e5364: f64 = (noise_metadata_schedule_539_e5362 / params.p50);
        (noise_metadata_schedule_539_e5364,)
    } else {
        (noise_variable_259,)
    }
};
            noise_variable_259 = noise_metadata_schedule_539_e5366;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_540_e5369: f64 = if noise_variable_202 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_534 = noise_metadata_schedule_540_e5369;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_541_e5396,) = {
    if (((((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 == 0.0)) && (noise_variable_531 != 0.0)) && (noise_variable_532 != 0.0)) && (noise_variable_533 == 0.0)) && (noise_variable_534 != 0.0)) {
        let noise_metadata_schedule_541_e5390: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_541_e5391: f64 = (1.0 + noise_metadata_schedule_541_e5390);
        let noise_metadata_schedule_541_e5392: f64 = (noise_metadata_schedule_541_e5391).ln();
        let noise_metadata_schedule_541_e5393: f64 = (params.p50 * noise_metadata_schedule_541_e5392);
        let noise_metadata_schedule_541_e5394: f64 = (1.0 + noise_metadata_schedule_541_e5393);
        (noise_metadata_schedule_541_e5394,)
    } else {
        (noise_variable_203,)
    }
};
            noise_variable_203 = noise_metadata_schedule_541_e5396;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_542_e5425,) = {
    if (((((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 == 0.0)) && (noise_variable_531 != 0.0)) && (noise_variable_532 != 0.0)) && (noise_variable_533 == 0.0)) && (noise_variable_534 == 0.0)) {
        let noise_metadata_schedule_542_e5418: f64 = (-noise_variable_259);
        let noise_metadata_schedule_542_e5419: f64 = (noise_metadata_schedule_542_e5418).exp();
        let noise_metadata_schedule_542_e5420: f64 = (1.0 + noise_metadata_schedule_542_e5419);
        let noise_metadata_schedule_542_e5421: f64 = (noise_metadata_schedule_542_e5420).ln();
        let noise_metadata_schedule_542_e5422: f64 = (params.p50 * noise_metadata_schedule_542_e5421);
        let noise_metadata_schedule_542_e5423: f64 = (noise_variable_202 + noise_metadata_schedule_542_e5422);
        (noise_metadata_schedule_542_e5423,)
    } else {
        (noise_variable_203,)
    }
};
            noise_variable_203 = noise_metadata_schedule_542_e5425;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_543_e5446,) = {
    if ((((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 == 0.0)) && (noise_variable_531 != 0.0)) && (noise_variable_532 != 0.0)) && (noise_variable_533 == 0.0)) {
        let noise_metadata_schedule_543_e5443: f64 = (noise_variable_203).powf(params.p49);
        let noise_metadata_schedule_543_e5444: f64 = (noise_variable_200 * noise_metadata_schedule_543_e5443);
        (noise_metadata_schedule_543_e5444,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_543_e5446;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_544_e5448: f64 = (-noise_variable_310);
            let noise_metadata_schedule_544_e5450: f64 = (noise_metadata_schedule_544_e5448 * noise_variable_201);
            let noise_metadata_schedule_544_e5452: f64 = if noise_metadata_schedule_544_e5450 < params.p134 { 1.0 } else { 0.0 };
            noise_variable_535 = noise_metadata_schedule_544_e5452;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_545_e5472,) = {
    if ((((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 == 0.0)) && (noise_variable_531 != 0.0)) && (noise_variable_532 != 0.0)) && (noise_variable_535 != 0.0)) {
        let noise_metadata_schedule_545_e5467: f64 = (-noise_variable_310);
        let noise_metadata_schedule_545_e5469: f64 = (noise_metadata_schedule_545_e5467 * noise_variable_201);
        let noise_metadata_schedule_545_e5470: f64 = (noise_metadata_schedule_545_e5469).exp();
        (noise_metadata_schedule_545_e5470,)
    } else {
        (noise_variable_313,)
    }
};
            noise_variable_313 = noise_metadata_schedule_545_e5472;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_546_e5490,) = {
    if ((((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 == 0.0)) && (noise_variable_531 != 0.0)) && (noise_variable_532 != 0.0)) && (noise_variable_535 == 0.0)) {
        let noise_metadata_schedule_546_e5488: f64 = (params.p134).exp();
        (noise_metadata_schedule_546_e5488,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_546_e5490;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_547_e5516,) = {
    if ((((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 == 0.0)) && (noise_variable_531 != 0.0)) && (noise_variable_532 != 0.0)) && (noise_variable_535 == 0.0)) {
        let noise_metadata_schedule_547_e5508: f64 = (-noise_variable_310);
        let noise_metadata_schedule_547_e5510: f64 = (noise_metadata_schedule_547_e5508 * noise_variable_201);
        let noise_metadata_schedule_547_e5512: f64 = (noise_metadata_schedule_547_e5510 - params.p134);
        let noise_metadata_schedule_547_e5513: f64 = (1.0 + noise_metadata_schedule_547_e5512);
        let noise_metadata_schedule_547_e5514: f64 = (noise_variable_275 * noise_metadata_schedule_547_e5513);
        (noise_metadata_schedule_547_e5514,)
    } else {
        (noise_variable_313,)
    }
};
            noise_variable_313 = noise_metadata_schedule_547_e5516;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_548_e5538,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_522 == 0.0)) && (noise_variable_526 == 0.0)) && (noise_variable_531 != 0.0)) && (noise_variable_532 != 0.0)) {
        let noise_metadata_schedule_548_e5530: f64 = (params.p39 / noise_variable_310);
        let noise_metadata_schedule_548_e5533: f64 = (params.p43 - noise_variable_230);
        let noise_metadata_schedule_548_e5534: f64 = (noise_metadata_schedule_548_e5530 * noise_metadata_schedule_548_e5533);
        let noise_metadata_schedule_548_e5536: f64 = (noise_metadata_schedule_548_e5534 * noise_variable_313);
        (noise_metadata_schedule_548_e5536,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_548_e5538;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_549_e5541: f64 = if noise_variable_196 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_536 = noise_metadata_schedule_549_e5541;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_550_e5544: f64 = if params.p52 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_537 = noise_metadata_schedule_550_e5544;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_551_e5570,) = {
    if (((noise_variable_521 != 0.0) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) {
        let noise_metadata_schedule_551_e5554: f64 = (noise_variable_30 + noise_variable_175);
        let noise_metadata_schedule_551_e5555: f64 = (noise_variable_149 * noise_metadata_schedule_551_e5554);
        let noise_metadata_schedule_551_e5556: f64 = (noise_variable_6 / noise_metadata_schedule_551_e5555);
        let noise_metadata_schedule_551_e5559: f64 = (noise_variable_146 / noise_variable_35);
        let noise_metadata_schedule_551_e5561: f64 = (noise_metadata_schedule_551_e5559 * noise_variable_42);
        let noise_metadata_schedule_551_e5562: f64 = (noise_metadata_schedule_551_e5556 + noise_metadata_schedule_551_e5561);
        let noise_metadata_schedule_551_e5566: f64 = (noise_variable_30 + noise_variable_175);
        let noise_metadata_schedule_551_e5567: f64 = (noise_variable_28 / noise_metadata_schedule_551_e5566);
        let noise_metadata_schedule_551_e5568: f64 = (noise_metadata_schedule_551_e5562 + noise_metadata_schedule_551_e5567);
        (noise_metadata_schedule_551_e5568,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_551_e5570;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_552_e5573: f64 = if params.p38 == 3.0 { 1.0 } else { 0.0 };
            noise_variable_538 = noise_metadata_schedule_552_e5573;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_553_e5587,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_538 != 0.0)) {
        let noise_metadata_schedule_553_e5583: f64 = (noise_variable_196 - noise_variable_197);
        let noise_metadata_schedule_553_e5585: f64 = (noise_metadata_schedule_553_e5583 / 1e-6);
        (noise_metadata_schedule_553_e5585,)
    } else {
        (noise_variable_259,)
    }
};
            noise_variable_259 = noise_metadata_schedule_553_e5587;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_554_e5590: f64 = if noise_variable_196 < noise_variable_197 { 1.0 } else { 0.0 };
            noise_variable_539 = noise_metadata_schedule_554_e5590;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_555_e5610,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_538 != 0.0)) && (noise_variable_539 != 0.0)) {
        let noise_metadata_schedule_555_e5604: f64 = (noise_variable_259).exp();
        let noise_metadata_schedule_555_e5605: f64 = (1.0 + noise_metadata_schedule_555_e5604);
        let noise_metadata_schedule_555_e5606: f64 = (noise_metadata_schedule_555_e5605).ln();
        let noise_metadata_schedule_555_e5607: f64 = (1e-6 * noise_metadata_schedule_555_e5606);
        let noise_metadata_schedule_555_e5608: f64 = (noise_variable_196 - noise_metadata_schedule_555_e5607);
        (noise_metadata_schedule_555_e5608,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_555_e5610;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_556_e5632,) = {
    if (((((noise_variable_521 != 0.0) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_538 != 0.0)) && (noise_variable_539 == 0.0)) {
        let noise_metadata_schedule_556_e5625: f64 = (-noise_variable_259);
        let noise_metadata_schedule_556_e5626: f64 = (noise_metadata_schedule_556_e5625).exp();
        let noise_metadata_schedule_556_e5627: f64 = (1.0 + noise_metadata_schedule_556_e5626);
        let noise_metadata_schedule_556_e5628: f64 = (noise_metadata_schedule_556_e5627).ln();
        let noise_metadata_schedule_556_e5629: f64 = (1e-6 * noise_metadata_schedule_556_e5628);
        let noise_metadata_schedule_556_e5630: f64 = (noise_variable_197 - noise_metadata_schedule_556_e5629);
        (noise_metadata_schedule_556_e5630,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_556_e5632;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_557_e5644,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_538 != 0.0)) {
        let noise_metadata_schedule_557_e5642: f64 = (noise_variable_149 * noise_variable_196);
        (noise_metadata_schedule_557_e5642,)
    } else {
        (noise_variable_198,)
    }
};
            noise_variable_198 = noise_metadata_schedule_557_e5644;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_558_e5663,) = {
    if ((((noise_variable_521 != 0.0) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_538 == 0.0)) {
        let noise_metadata_schedule_558_e5655: f64 = (noise_variable_149 * noise_variable_196);
        let noise_metadata_schedule_558_e5657: f64 = (noise_metadata_schedule_558_e5655 * noise_variable_197);
        let noise_metadata_schedule_558_e5660: f64 = (noise_variable_196 + noise_variable_197);
        let noise_metadata_schedule_558_e5661: f64 = (noise_metadata_schedule_558_e5657 / noise_metadata_schedule_558_e5660);
        (noise_metadata_schedule_558_e5661,)
    } else {
        (noise_variable_198,)
    }
};
            noise_variable_198 = noise_metadata_schedule_558_e5663;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_559_e5674,) = {
    if (((noise_variable_521 != 0.0) && (noise_variable_536 != 0.0)) && (noise_variable_537 == 0.0)) {
        let noise_metadata_schedule_559_e5672: f64 = (noise_variable_149 * noise_variable_196);
        (noise_metadata_schedule_559_e5672,)
    } else {
        (noise_variable_198,)
    }
};
            noise_variable_198 = noise_metadata_schedule_559_e5674;
        }
        if matches!(source_index, 3 | 4 | 5 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_637_e6439: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_637_e6441: f64 = (noise_metadata_schedule_637_e6439 * noise_variable_2);
            noise_variable_281 = noise_metadata_schedule_637_e6441;
        }
        if matches!(source_index, 3) {
            let noise_metadata_schedule_638_e6444: f64 = (noise_variable_281 / noise_variable_28);
            noise_variable_282 = noise_metadata_schedule_638_e6444;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_639_e6447: f64 = (noise_variable_281 / noise_variable_30);
            noise_variable_283 = noise_metadata_schedule_639_e6447;
        }
        if matches!(source_index, 17 | 20 | 22 | 24) {
            let noise_metadata_schedule_640_e6450: f64 = (noise_variable_281 * noise_variable_101);
            noise_variable_284 = noise_metadata_schedule_640_e6450;
        }
        if matches!(source_index, 18 | 21) {
            let noise_metadata_schedule_641_e6453: f64 = (noise_variable_281 * noise_variable_102);
            noise_variable_285 = noise_metadata_schedule_641_e6453;
        }
        if matches!(source_index, 19 | 23) {
            let noise_metadata_schedule_642_e6456: f64 = (noise_variable_281 * noise_variable_103);
            noise_variable_286 = noise_metadata_schedule_642_e6456;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_643_e6459: f64 = (noise_variable_281 / noise_variable_175);
            let noise_metadata_schedule_643_e6462: f64 = (4.0 * noise_variable_247);
            let noise_metadata_schedule_643_e6464: f64 = (noise_metadata_schedule_643_e6462 + 5.0);
            let noise_metadata_schedule_643_e6465: f64 = (noise_metadata_schedule_643_e6459 * noise_metadata_schedule_643_e6464);
            let noise_metadata_schedule_643_e6467: f64 = (noise_metadata_schedule_643_e6465 * 0.3333333333333333);
            noise_variable_287 = noise_metadata_schedule_643_e6467;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_644_e6470: f64 = (noise_variable_148 + noise_variable_147);
            let noise_metadata_schedule_644_e6472: f64 = (noise_metadata_schedule_644_e6470 / noise_variable_146);
            noise_variable_303 = noise_metadata_schedule_644_e6472;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_645_e6475: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_645_e6477: f64 = (noise_variable_303).abs();
            let noise_metadata_schedule_645_e6478: f64 = (noise_metadata_schedule_645_e6475 * noise_metadata_schedule_645_e6477);
            noise_variable_288 = noise_metadata_schedule_645_e6478;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_646_e6481: f64 = if params.p129 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_555 = noise_metadata_schedule_646_e6481;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_647_e6488,) = {
    if (noise_variable_555 != 0.0) {
        let noise_metadata_schedule_647_e6485: f64 = (noise_variable_198 / noise_variable_303);
        let noise_metadata_schedule_647_e6486: f64 = (noise_metadata_schedule_647_e6485).abs();
        (noise_metadata_schedule_647_e6486,)
    } else {
        (noise_variable_304,)
    }
};
            noise_variable_304 = noise_metadata_schedule_647_e6488;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_648_e6493,) = {
    if (noise_variable_555 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_304,)
    }
};
            noise_variable_304 = noise_metadata_schedule_648_e6493;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_649_e6496: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_649_e6498: f64 = (noise_metadata_schedule_649_e6496 * noise_variable_198);
            let noise_metadata_schedule_649_e6501: f64 = (noise_variable_304 + 1.0);
            let noise_metadata_schedule_649_e6502: f64 = (noise_metadata_schedule_649_e6498 * noise_metadata_schedule_649_e6501);
            noise_variable_300 = noise_metadata_schedule_649_e6502;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_658_e6554: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_658_e6557: f64 = (noise_variable_151 + noise_variable_153);
            let noise_metadata_schedule_658_e6559: f64 = (noise_metadata_schedule_658_e6557 - noise_variable_57);
            let noise_metadata_schedule_658_e6561: f64 = (noise_metadata_schedule_658_e6559 + noise_variable_327);
            let noise_metadata_schedule_658_e6563: f64 = (noise_metadata_schedule_658_e6561 + noise_variable_326);
            let noise_metadata_schedule_658_e6564: f64 = (noise_metadata_schedule_658_e6563).abs();
            let noise_metadata_schedule_658_e6565: f64 = (noise_metadata_schedule_658_e6554 * noise_metadata_schedule_658_e6564);
            noise_variable_289 = noise_metadata_schedule_658_e6565;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_659_e6568: f64 = (noise_variable_151 + noise_variable_152);
            noise_variable_301 = noise_metadata_schedule_659_e6568;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_660_e6571: f64 = (noise_variable_301).abs();
            let noise_metadata_schedule_660_e6573: f64 = (noise_metadata_schedule_660_e6571).powf(params.p125);
            let noise_metadata_schedule_660_e6574: f64 = (params.p127 * noise_metadata_schedule_660_e6573);
            noise_variable_290 = noise_metadata_schedule_660_e6574;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_661_e6577: f64 = if noise_variable_301 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_559 = noise_metadata_schedule_661_e6577;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_662_e6582,) = {
    if (noise_variable_559 != 0.0) {
        let noise_metadata_schedule_662_e6580: f64 = (-noise_variable_290);
        (noise_metadata_schedule_662_e6580,)
    } else {
        (noise_variable_290,)
    }
};
            noise_variable_290 = noise_metadata_schedule_662_e6582;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_663_e6585: f64 = (noise_variable_153 + noise_variable_155);
            let noise_metadata_schedule_663_e6587: f64 = (noise_metadata_schedule_663_e6585 + noise_variable_156);
            noise_variable_302 = noise_metadata_schedule_663_e6587;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_664_e6590: f64 = (noise_variable_302).abs();
            let noise_metadata_schedule_664_e6592: f64 = (noise_metadata_schedule_664_e6590).powf(params.p126);
            let noise_metadata_schedule_664_e6593: f64 = (params.p128 * noise_metadata_schedule_664_e6592);
            noise_variable_291 = noise_metadata_schedule_664_e6593;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_665_e6596: f64 = if noise_variable_302 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_560 = noise_metadata_schedule_665_e6596;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_666_e6601,) = {
    if (noise_variable_560 != 0.0) {
        let noise_metadata_schedule_666_e6599: f64 = (-noise_variable_291);
        (noise_metadata_schedule_666_e6599,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_666_e6601;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_667_e6604: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_667_e6607: f64 = (noise_variable_152 + noise_variable_155);
            let noise_metadata_schedule_667_e6609: f64 = (noise_metadata_schedule_667_e6607 + noise_variable_156);
            let noise_metadata_schedule_667_e6610: f64 = (noise_metadata_schedule_667_e6609).abs();
            let noise_metadata_schedule_667_e6611: f64 = (noise_metadata_schedule_667_e6604 * noise_metadata_schedule_667_e6610);
            noise_variable_292 = noise_metadata_schedule_667_e6611;
        }
        if matches!(source_index, 9) {
            let noise_metadata_schedule_668_e6614: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_668_e6616: f64 = (noise_variable_154).abs();
            let noise_metadata_schedule_668_e6617: f64 = (noise_metadata_schedule_668_e6614 * noise_metadata_schedule_668_e6616);
            noise_variable_293 = noise_metadata_schedule_668_e6617;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_669_e6620: f64 = (noise_variable_154).abs();
            let noise_metadata_schedule_669_e6622: f64 = (noise_metadata_schedule_669_e6620).powf(params.p125);
            let noise_metadata_schedule_669_e6623: f64 = (params.p127 * noise_metadata_schedule_669_e6622);
            noise_variable_294 = noise_metadata_schedule_669_e6623;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_670_e6626: f64 = if noise_variable_154 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_561 = noise_metadata_schedule_670_e6626;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_671_e6631,) = {
    if (noise_variable_561 != 0.0) {
        let noise_metadata_schedule_671_e6629: f64 = (-noise_variable_294);
        (noise_metadata_schedule_671_e6629,)
    } else {
        (noise_variable_294,)
    }
};
            noise_variable_294 = noise_metadata_schedule_671_e6631;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_672_e6634: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_672_e6636: f64 = (noise_variable_82).abs();
            let noise_metadata_schedule_672_e6637: f64 = (noise_metadata_schedule_672_e6634 * noise_metadata_schedule_672_e6636);
            noise_variable_295 = noise_metadata_schedule_672_e6637;
        }
        if matches!(source_index, 11) {
            let noise_metadata_schedule_673_e6640: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_673_e6642: f64 = (noise_variable_157).abs();
            let noise_metadata_schedule_673_e6643: f64 = (noise_metadata_schedule_673_e6640 * noise_metadata_schedule_673_e6642);
            noise_variable_296 = noise_metadata_schedule_673_e6643;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_674_e6648: f64 = (params.p5 * params.p32);
            let noise_metadata_schedule_674_e6649: f64 = (1.0 - noise_metadata_schedule_674_e6648);
            let noise_metadata_schedule_674_e6650: f64 = (params.p127 * noise_metadata_schedule_674_e6649);
            let noise_metadata_schedule_674_e6652: f64 = (noise_variable_157).abs();
            let noise_metadata_schedule_674_e6656: f64 = (params.p5 * params.p32);
            let noise_metadata_schedule_674_e6657: f64 = (1.0 - noise_metadata_schedule_674_e6656);
            let noise_metadata_schedule_674_e6658: f64 = (noise_metadata_schedule_674_e6652 / noise_metadata_schedule_674_e6657);
            let noise_metadata_schedule_674_e6660: f64 = (noise_metadata_schedule_674_e6658).powf(params.p125);
            let noise_metadata_schedule_674_e6661: f64 = (noise_metadata_schedule_674_e6650 * noise_metadata_schedule_674_e6660);
            noise_variable_298 = noise_metadata_schedule_674_e6661;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_675_e6664: f64 = if noise_variable_157 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_562 = noise_metadata_schedule_675_e6664;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_676_e6669,) = {
    if (noise_variable_562 != 0.0) {
        let noise_metadata_schedule_676_e6667: f64 = (-noise_variable_298);
        (noise_metadata_schedule_676_e6667,)
    } else {
        (noise_variable_298,)
    }
};
            noise_variable_298 = noise_metadata_schedule_676_e6669;
        }
        if matches!(source_index, 13) {
            let noise_metadata_schedule_677_e6672: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_677_e6674: f64 = (noise_variable_169).abs();
            let noise_metadata_schedule_677_e6675: f64 = (noise_metadata_schedule_677_e6672 * noise_metadata_schedule_677_e6674);
            let noise_metadata_schedule_677_e6677: f64 = (noise_metadata_schedule_677_e6675 * params.p5);
            noise_variable_297 = noise_metadata_schedule_677_e6677;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_678_e6680: f64 = if params.p32 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_563 = noise_metadata_schedule_678_e6680;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_679_e6684,) = {
    if (noise_variable_563 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_299,)
    }
};
            noise_variable_299 = noise_metadata_schedule_679_e6684;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_680_e6700,) = {
    if (noise_variable_563 == 0.0) {
        let noise_metadata_schedule_680_e6689: f64 = (params.p127 * params.p5);
        let noise_metadata_schedule_680_e6691: f64 = (noise_metadata_schedule_680_e6689 * params.p32);
        let noise_metadata_schedule_680_e6693: f64 = (noise_variable_169).abs();
        let noise_metadata_schedule_680_e6695: f64 = (noise_metadata_schedule_680_e6693 / params.p32);
        let noise_metadata_schedule_680_e6697: f64 = (noise_metadata_schedule_680_e6695).powf(params.p125);
        let noise_metadata_schedule_680_e6698: f64 = (noise_metadata_schedule_680_e6691 * noise_metadata_schedule_680_e6697);
        (noise_metadata_schedule_680_e6698,)
    } else {
        (noise_variable_299,)
    }
};
            noise_variable_299 = noise_metadata_schedule_680_e6700;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_681_e6703: f64 = if noise_variable_169 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_564 = noise_metadata_schedule_681_e6703;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_682_e6708,) = {
    if (noise_variable_564 != 0.0) {
        let noise_metadata_schedule_682_e6706: f64 = (-noise_variable_299);
        (noise_metadata_schedule_682_e6706,)
    } else {
        (noise_variable_299,)
    }
};
            noise_variable_299 = noise_metadata_schedule_682_e6708;
        }
        match source_index {
            0 => {
                let noise_0_psd_e7754: f64 = 1.0;
                let noise_0_psd_e349: f64 = (noise_variable_288 * params.p1);
                let noise_0_psd_e7755: f64 = (noise_0_psd_e7754 * noise_0_psd_e349);
                let psd = noise_0_psd_e7755;
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
                let noise_1_psd_e7757: f64 = 1.0;
                let noise_1_psd_e363: f64 = (noise_variable_300 * params.p1);
                let noise_1_psd_e7758: f64 = (noise_1_psd_e7757 * noise_1_psd_e363);
                let psd = noise_1_psd_e7758;
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
                let noise_2_psd_e7760: f64 = 1.0;
                let noise_2_psd_e368: f64 = (noise_variable_289 * params.p1);
                let noise_2_psd_e7761: f64 = (noise_2_psd_e7760 * noise_2_psd_e368);
                let psd = noise_2_psd_e7761;
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
                let noise_3_psd_e7763: f64 = 1.0;
                let noise_3_psd_e373: f64 = (noise_variable_282 * params.p1);
                let noise_3_psd_e7764: f64 = (noise_3_psd_e7763 * noise_3_psd_e373);
                let psd = noise_3_psd_e7764;
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
                let noise_4_psd_e7766: f64 = 1.0;
                let noise_4_psd_e378: f64 = (noise_variable_283 * params.p1);
                let noise_4_psd_e7767: f64 = (noise_4_psd_e7766 * noise_4_psd_e378);
                let psd = noise_4_psd_e7767;
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
                let noise_5_psd_e7769: f64 = 1.0;
                let noise_5_psd_e383: f64 = (noise_variable_287 * params.p1);
                let noise_5_psd_e7770: f64 = (noise_5_psd_e7769 * noise_5_psd_e383);
                let psd = noise_5_psd_e7770;
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
                let noise_6_psd_e7772: f64 = 1.0;
                let noise_6_psd_e388: f64 = (noise_variable_290 * params.p1);
                let noise_6_psd_e7773: f64 = (noise_6_psd_e7772 * noise_6_psd_e388);
                let psd = noise_6_psd_e7773;
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
                let noise_7_psd_e7775: f64 = 1.0;
                let noise_7_psd_e394: f64 = (noise_variable_291 * params.p1);
                let noise_7_psd_e7776: f64 = (noise_7_psd_e7775 * noise_7_psd_e394);
                let psd = noise_7_psd_e7776;
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
                let noise_8_psd_e7778: f64 = 1.0;
                let noise_8_psd_e400: f64 = (noise_variable_292 * params.p1);
                let noise_8_psd_e7779: f64 = (noise_8_psd_e7778 * noise_8_psd_e400);
                let psd = noise_8_psd_e7779;
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
                let noise_9_psd_e7781: f64 = 1.0;
                let noise_9_psd_e405: f64 = (noise_variable_293 * params.p1);
                let noise_9_psd_e7782: f64 = (noise_9_psd_e7781 * noise_9_psd_e405);
                let psd = noise_9_psd_e7782;
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
                let noise_10_psd_e7784: f64 = 1.0;
                let noise_10_psd_e410: f64 = (noise_variable_294 * params.p1);
                let noise_10_psd_e7785: f64 = (noise_10_psd_e7784 * noise_10_psd_e410);
                let psd = noise_10_psd_e7785;
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
                let noise_11_psd_e7787: f64 = 1.0;
                let noise_11_psd_e416: f64 = (noise_variable_296 * params.p1);
                let noise_11_psd_e7788: f64 = (noise_11_psd_e7787 * noise_11_psd_e416);
                let psd = noise_11_psd_e7788;
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
                let noise_12_psd_e7790: f64 = 1.0;
                let noise_12_psd_e421: f64 = (noise_variable_298 * params.p1);
                let noise_12_psd_e7791: f64 = (noise_12_psd_e7790 * noise_12_psd_e421);
                let psd = noise_12_psd_e7791;
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
                let noise_13_psd_e7793: f64 = 1.0;
                let noise_13_psd_e427: f64 = (noise_variable_297 * params.p1);
                let noise_13_psd_e7794: f64 = (noise_13_psd_e7793 * noise_13_psd_e427);
                let psd = noise_13_psd_e7794;
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
                let noise_14_psd_e7796: f64 = 1.0;
                let noise_14_psd_e432: f64 = (noise_variable_299 * params.p1);
                let noise_14_psd_e7797: f64 = (noise_14_psd_e7796 * noise_14_psd_e432);
                let psd = noise_14_psd_e7797;
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
                let noise_15_psd_e7799: f64 = 1.0;
                let noise_15_psd_e439: f64 = (noise_variable_295 * params.p1);
                let noise_15_psd_e7800: f64 = (noise_15_psd_e7799 * noise_15_psd_e439);
                let psd = noise_15_psd_e7800;
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
                let noise_16_psd_e7802: f64 = 1.0;
                let noise_16_psd_e448: f64 = (noise_variable_295 * params.p1);
                let noise_16_psd_e7803: f64 = (noise_16_psd_e7802 * noise_16_psd_e448);
                let psd = noise_16_psd_e7803;
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
                let noise_17_psd_e7805: f64 = 1.0;
                let noise_17_psd_e458: f64 = (noise_variable_284 * params.p1);
                let noise_17_psd_e7806: f64 = (noise_17_psd_e7805 * noise_17_psd_e458);
                let psd = noise_17_psd_e7806;
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
                let noise_18_psd_e7808: f64 = 1.0;
                let noise_18_psd_e468: f64 = (noise_variable_285 * params.p1);
                let noise_18_psd_e7809: f64 = (noise_18_psd_e7808 * noise_18_psd_e468);
                let psd = noise_18_psd_e7809;
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
                let noise_19_psd_e7811: f64 = 1.0;
                let noise_19_psd_e478: f64 = (noise_variable_286 * params.p1);
                let noise_19_psd_e7812: f64 = (noise_19_psd_e7811 * noise_19_psd_e478);
                let psd = noise_19_psd_e7812;
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
                let noise_20_psd_e7814: f64 = 1.0;
                let noise_20_psd_e489: f64 = (noise_variable_284 * params.p1);
                let noise_20_psd_e7815: f64 = (noise_20_psd_e7814 * noise_20_psd_e489);
                let psd = noise_20_psd_e7815;
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
                let noise_21_psd_e7817: f64 = 1.0;
                let noise_21_psd_e500: f64 = (noise_variable_285 * params.p1);
                let noise_21_psd_e7818: f64 = (noise_21_psd_e7817 * noise_21_psd_e500);
                let psd = noise_21_psd_e7818;
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
                let noise_22_psd_e7820: f64 = 1.0;
                let noise_22_psd_e511: f64 = (noise_variable_284 * params.p1);
                let noise_22_psd_e7821: f64 = (noise_22_psd_e7820 * noise_22_psd_e511);
                let psd = noise_22_psd_e7821;
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
                let noise_23_psd_e7823: f64 = 1.0;
                let noise_23_psd_e522: f64 = (noise_variable_286 * params.p1);
                let noise_23_psd_e7824: f64 = (noise_23_psd_e7823 * noise_23_psd_e522);
                let psd = noise_23_psd_e7824;
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
                let noise_24_psd_e7826: f64 = 1.0;
                let noise_24_psd_e534: f64 = (noise_variable_284 * params.p1);
                let noise_24_psd_e7827: f64 = (noise_24_psd_e7826 * noise_24_psd_e534);
                let psd = noise_24_psd_e7827;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 24, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            _ => unreachable!("noise source index was range checked"),
        }
    }
}

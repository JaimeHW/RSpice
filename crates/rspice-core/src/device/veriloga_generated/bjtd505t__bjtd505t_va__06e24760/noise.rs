#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 25] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_NOI_GND_IN", label: Some("in"), kind: GeneratedNoiseKind::White, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "noi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_E1_IB2E1", label: Some("ib2e1"), kind: GeneratedNoiseKind::White, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_E1_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_B1_RBC", label: Some("rbc"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_B2_RBV", label: Some("rbv"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B2_E1_IB2E1_F", label: Some("ib2e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_E1_IB1E1_F", label: Some("ib1e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_E1_IB1E1", label: Some("ib1e1"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IB3", label: Some("ib3"), kind: GeneratedNoiseKind::White, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IB3_F", label: Some("ib3_f"), kind: GeneratedNoiseKind::Flicker, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IEX", label: Some("iex"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IEX_F", label: Some("iex_f"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_C3_XIEX", label: Some("xiex"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B_C3_XIEX_F", label: Some("xiex_f"), kind: GeneratedNoiseKind::Flicker, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C1_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C4_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C1_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C4_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C1_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
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
        if matches!(source_index, 15 | 16) {
            let noise_activation_schedule_702_e6945: f64 = if params.p23 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_579 = noise_activation_schedule_702_e6945;
        }
        if matches!(source_index, 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_activation_schedule_703_e6948: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_580 = noise_activation_schedule_703_e6948;
        }
        if matches!(source_index, 17 | 18 | 19 | 20 | 21) {
            let noise_activation_schedule_704_e6951: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_581 = noise_activation_schedule_704_e6951;
        }
        if matches!(source_index, 22 | 23 | 24) {
            let noise_activation_schedule_705_e6954: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_582 = noise_activation_schedule_705_e6954;
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
                noise_variable_579 != 0.0
            }
            16 => {
                let noise_16_activation_e457: f64 = if (noise_variable_579 == 0.0) { 1.0 } else { 0.0 };
                noise_16_activation_e457 != 0.0
            }
            17 => {
                let noise_17_activation_e467: f64 = if ((noise_variable_580 != 0.0) && (noise_variable_581 != 0.0)) { 1.0 } else { 0.0 };
                noise_17_activation_e467 != 0.0
            }
            18 => {
                let noise_18_activation_e477: f64 = if ((noise_variable_580 != 0.0) && (noise_variable_581 != 0.0)) { 1.0 } else { 0.0 };
                noise_18_activation_e477 != 0.0
            }
            19 => {
                let noise_19_activation_e487: f64 = if ((noise_variable_580 != 0.0) && (noise_variable_581 != 0.0)) { 1.0 } else { 0.0 };
                noise_19_activation_e487 != 0.0
            }
            20 => {
                let noise_20_activation_e498: f64 = if ((noise_variable_580 != 0.0) && (noise_variable_581 == 0.0)) { 1.0 } else { 0.0 };
                noise_20_activation_e498 != 0.0
            }
            21 => {
                let noise_21_activation_e509: f64 = if ((noise_variable_580 != 0.0) && (noise_variable_581 == 0.0)) { 1.0 } else { 0.0 };
                noise_21_activation_e509 != 0.0
            }
            22 => {
                let noise_22_activation_e520: f64 = if ((noise_variable_580 == 0.0) && (noise_variable_582 != 0.0)) { 1.0 } else { 0.0 };
                noise_22_activation_e520 != 0.0
            }
            23 => {
                let noise_23_activation_e531: f64 = if ((noise_variable_580 == 0.0) && (noise_variable_582 != 0.0)) { 1.0 } else { 0.0 };
                noise_23_activation_e531 != 0.0
            }
            24 => {
                let noise_24_activation_e543: f64 = if ((noise_variable_580 == 0.0) && (noise_variable_582 == 0.0)) { 1.0 } else { 0.0 };
                noise_24_activation_e543 != 0.0
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
        if matches!(source_index, 1) {
            let noise_metadata_schedule_0_e553: f64 = if params.p3 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_447 = noise_metadata_schedule_0_e553;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_1_e557,) = {
    if (noise_variable_447 != 0.0) {
        (70300000.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_1_e557;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_2_e561,) = {
    if (noise_variable_447 != 0.0) {
        (123000000.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_2_e561;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_3_e566,) = {
    if (noise_variable_447 == 0.0) {
        (158000000.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_3_e566;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_4_e571,) = {
    if (noise_variable_447 == 0.0) {
        (204000000.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_4_e571;
        }
        if matches!(source_index, 11 | 12) {
            let noise_metadata_schedule_5_e574: f64 = (1.0 - params.p32);
            noise_variable_153 = noise_metadata_schedule_5_e574;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_6_e577: f64 = (params.p4 + 273.15);
            noise_variable_3 = noise_metadata_schedule_6_e577;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_7_e578: f64 = ctx.temperature();
            let noise_metadata_schedule_7_e580: f64 = (noise_metadata_schedule_7_e578 + params.p0);
            noise_variable_5 = noise_metadata_schedule_7_e580;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_9_e586: f64 = if params.p141 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_448 = noise_metadata_schedule_9_e586;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let (noise_metadata_schedule_10_e590,) = {
    if (noise_variable_448 != 0.0) {
        (1e-12,)
    } else {
        (noise_variable_321,)
    }
};
            noise_variable_321 = noise_metadata_schedule_10_e590;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let (noise_metadata_schedule_11_e595,) = {
    if (noise_variable_448 == 0.0) {
        (params.p141,)
    } else {
        (noise_variable_321,)
    }
};
            noise_variable_321 = noise_metadata_schedule_11_e595;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_12_e598: f64 = (noise_variable_321 * params.p1);
            noise_variable_322 = noise_metadata_schedule_12_e598;
        }
        if matches!(source_index, 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_13_e601: f64 = (1.0 / noise_variable_322);
            noise_variable_323 = noise_metadata_schedule_13_e601;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            noise_variable_52 = 0.001;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            noise_variable_318 = 0.001;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_16_e607: f64 = (2.0 - params.p66);
            let noise_metadata_schedule_16_e608: f64 = (2.0_f64).powf(noise_metadata_schedule_16_e607);
            noise_variable_62 = noise_metadata_schedule_16_e608;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_17_e611: f64 = (1.0 / noise_variable_62);
            noise_variable_63 = noise_metadata_schedule_17_e611;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_18_e615: f64 = (params.p114 * noise_variable_3);
            let noise_metadata_schedule_18_e617: f64 = (noise_metadata_schedule_18_e615 * noise_variable_3);
            let noise_metadata_schedule_18_e620: f64 = (noise_variable_3 + params.p115);
            let noise_metadata_schedule_18_e621: f64 = (noise_metadata_schedule_18_e617 / noise_metadata_schedule_18_e620);
            let noise_metadata_schedule_18_e622: f64 = (params.p113 + noise_metadata_schedule_18_e621);
            let noise_metadata_schedule_18_e624: f64 = (noise_metadata_schedule_18_e622 - 0.05);
            let noise_metadata_schedule_18_e626: f64 = (noise_metadata_schedule_18_e624 / 0.1);
            noise_variable_265 = noise_metadata_schedule_18_e626;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_19_e630: f64 = (params.p114 * noise_variable_3);
            let noise_metadata_schedule_19_e632: f64 = (noise_metadata_schedule_19_e630 * noise_variable_3);
            let noise_metadata_schedule_19_e635: f64 = (noise_variable_3 + params.p115);
            let noise_metadata_schedule_19_e636: f64 = (noise_metadata_schedule_19_e632 / noise_metadata_schedule_19_e635);
            let noise_metadata_schedule_19_e637: f64 = (params.p113 + noise_metadata_schedule_19_e636);
            let noise_metadata_schedule_19_e639: f64 = if noise_metadata_schedule_19_e637 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_449 = noise_metadata_schedule_19_e639;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_20_e651,) = {
    if (noise_variable_449 != 0.0) {
        let noise_metadata_schedule_20_e645: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_20_e646: f64 = (1.0 + noise_metadata_schedule_20_e645);
        let noise_metadata_schedule_20_e647: f64 = (noise_metadata_schedule_20_e646).ln();
        let noise_metadata_schedule_20_e648: f64 = (0.1 * noise_metadata_schedule_20_e647);
        let noise_metadata_schedule_20_e649: f64 = (0.05 + noise_metadata_schedule_20_e648);
        (noise_metadata_schedule_20_e649,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_20_e651;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_21_e675,) = {
    if (noise_variable_449 == 0.0) {
        let noise_metadata_schedule_21_e657: f64 = (params.p114 * noise_variable_3);
        let noise_metadata_schedule_21_e659: f64 = (noise_metadata_schedule_21_e657 * noise_variable_3);
        let noise_metadata_schedule_21_e662: f64 = (noise_variable_3 + params.p115);
        let noise_metadata_schedule_21_e663: f64 = (noise_metadata_schedule_21_e659 / noise_metadata_schedule_21_e662);
        let noise_metadata_schedule_21_e664: f64 = (params.p113 + noise_metadata_schedule_21_e663);
        let noise_metadata_schedule_21_e668: f64 = (-noise_variable_265);
        let noise_metadata_schedule_21_e669: f64 = (noise_metadata_schedule_21_e668).exp();
        let noise_metadata_schedule_21_e670: f64 = (1.0 + noise_metadata_schedule_21_e669);
        let noise_metadata_schedule_21_e671: f64 = (noise_metadata_schedule_21_e670).ln();
        let noise_metadata_schedule_21_e672: f64 = (0.1 * noise_metadata_schedule_21_e671);
        let noise_metadata_schedule_21_e673: f64 = (noise_metadata_schedule_21_e664 + noise_metadata_schedule_21_e672);
        (noise_metadata_schedule_21_e673,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_21_e675;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            noise_variable_71 = params.p113;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_23_e679: f64 = (1.0 / noise_variable_71);
            noise_variable_72 = noise_metadata_schedule_23_e679;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_24_e682: f64 = (1.0 / params.p65);
            noise_variable_64 = noise_metadata_schedule_24_e682;
        }
        if matches!(source_index, 1 | 15 | 16) {
            noise_variable_75 = params.p70;
        }
        if matches!(source_index, 1 | 15 | 16) {
            noise_variable_76 = params.p71;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_27_e688: f64 = (2.0 - noise_variable_76);
            let noise_metadata_schedule_27_e689: f64 = (2.0_f64).powf(noise_metadata_schedule_27_e688);
            noise_variable_79 = noise_metadata_schedule_27_e689;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_28_e692: f64 = (1.0 / noise_variable_79);
            noise_variable_89 = noise_metadata_schedule_28_e692;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_29_e696: f64 = (params.p117 * noise_variable_3);
            let noise_metadata_schedule_29_e698: f64 = (noise_metadata_schedule_29_e696 * noise_variable_3);
            let noise_metadata_schedule_29_e701: f64 = (noise_variable_3 + params.p118);
            let noise_metadata_schedule_29_e702: f64 = (noise_metadata_schedule_29_e698 / noise_metadata_schedule_29_e701);
            let noise_metadata_schedule_29_e703: f64 = (params.p116 + noise_metadata_schedule_29_e702);
            let noise_metadata_schedule_29_e705: f64 = (noise_metadata_schedule_29_e703 - 0.05);
            let noise_metadata_schedule_29_e707: f64 = (noise_metadata_schedule_29_e705 / 0.1);
            noise_variable_265 = noise_metadata_schedule_29_e707;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_30_e711: f64 = (params.p117 * noise_variable_3);
            let noise_metadata_schedule_30_e713: f64 = (noise_metadata_schedule_30_e711 * noise_variable_3);
            let noise_metadata_schedule_30_e716: f64 = (noise_variable_3 + params.p118);
            let noise_metadata_schedule_30_e717: f64 = (noise_metadata_schedule_30_e713 / noise_metadata_schedule_30_e716);
            let noise_metadata_schedule_30_e718: f64 = (params.p116 + noise_metadata_schedule_30_e717);
            let noise_metadata_schedule_30_e720: f64 = if noise_metadata_schedule_30_e718 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_450 = noise_metadata_schedule_30_e720;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_31_e732,) = {
    if (noise_variable_450 != 0.0) {
        let noise_metadata_schedule_31_e726: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_31_e727: f64 = (1.0 + noise_metadata_schedule_31_e726);
        let noise_metadata_schedule_31_e728: f64 = (noise_metadata_schedule_31_e727).ln();
        let noise_metadata_schedule_31_e729: f64 = (0.1 * noise_metadata_schedule_31_e728);
        let noise_metadata_schedule_31_e730: f64 = (0.05 + noise_metadata_schedule_31_e729);
        (noise_metadata_schedule_31_e730,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_31_e732;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_32_e756,) = {
    if (noise_variable_450 == 0.0) {
        let noise_metadata_schedule_32_e738: f64 = (params.p117 * noise_variable_3);
        let noise_metadata_schedule_32_e740: f64 = (noise_metadata_schedule_32_e738 * noise_variable_3);
        let noise_metadata_schedule_32_e743: f64 = (noise_variable_3 + params.p118);
        let noise_metadata_schedule_32_e744: f64 = (noise_metadata_schedule_32_e740 / noise_metadata_schedule_32_e743);
        let noise_metadata_schedule_32_e745: f64 = (params.p116 + noise_metadata_schedule_32_e744);
        let noise_metadata_schedule_32_e749: f64 = (-noise_variable_265);
        let noise_metadata_schedule_32_e750: f64 = (noise_metadata_schedule_32_e749).exp();
        let noise_metadata_schedule_32_e751: f64 = (1.0 + noise_metadata_schedule_32_e750);
        let noise_metadata_schedule_32_e752: f64 = (noise_metadata_schedule_32_e751).ln();
        let noise_metadata_schedule_32_e753: f64 = (0.1 * noise_metadata_schedule_32_e752);
        let noise_metadata_schedule_32_e754: f64 = (noise_metadata_schedule_32_e745 + noise_metadata_schedule_32_e753);
        (noise_metadata_schedule_32_e754,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_32_e756;
        }
        if matches!(source_index, 1 | 15 | 16) {
            noise_variable_87 = params.p116;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_34_e760: f64 = (1.0 / noise_variable_87);
            noise_variable_86 = noise_metadata_schedule_34_e760;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_35_e763: f64 = (1.0 / noise_variable_75);
            noise_variable_66 = noise_metadata_schedule_35_e763;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_36_e767: f64 = (1.0 / params.p82);
            let noise_metadata_schedule_36_e768: f64 = (1.0 - noise_metadata_schedule_36_e767);
            noise_variable_324 = noise_metadata_schedule_36_e768;
        }
        if matches!(source_index, 2 | 6) {
            noise_variable_154 = 0.0;
        }
        if matches!(source_index, 6 | 8) {
            noise_variable_155 = 0.0;
        }
        if matches!(source_index, 13 | 14) {
            noise_variable_172 = 0.0;
        }
        if matches!(source_index, 13 | 14) {
            noise_variable_171 = 1.0;
        }
        if matches!(source_index, 1) {
            noise_variable_199 = 0.0;
        }
        if matches!(source_index, 1) {
            noise_variable_201 = 0.0;
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
            noise_variable_207 = (ctx.node_voltage(self.nodes[3]) - 0.0);
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_51_e785: f64 = if noise_variable_207 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_451 = noise_metadata_schedule_51_e785;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let (noise_metadata_schedule_52_e793,) = {
    if (noise_variable_451 != 0.0) {
        let noise_metadata_schedule_52_e789: f64 = (1.0 - noise_variable_207);
        let noise_metadata_schedule_52_e790: f64 = (noise_metadata_schedule_52_e789).ln();
        let noise_metadata_schedule_52_e791: f64 = (-noise_metadata_schedule_52_e790);
        (noise_metadata_schedule_52_e791,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_52_e793;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_53_e796: f64 = if noise_variable_207 < params.p124 { 1.0 } else { 0.0 };
            noise_variable_452 = noise_metadata_schedule_53_e796;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let (noise_metadata_schedule_54_e800,) = {
    if (noise_variable_452 != 0.0) {
        (noise_variable_207,)
    } else {
        (noise_variable_11,)
    }
};
            noise_variable_11 = noise_metadata_schedule_54_e800;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let (noise_metadata_schedule_55_e812,) = {
    if (noise_variable_452 == 0.0) {
        let noise_metadata_schedule_55_e807: f64 = (noise_variable_207 - params.p124);
        let noise_metadata_schedule_55_e808: f64 = (1.0 + noise_metadata_schedule_55_e807);
        let noise_metadata_schedule_55_e809: f64 = (noise_metadata_schedule_55_e808).ln();
        let noise_metadata_schedule_55_e810: f64 = (params.p124 + noise_metadata_schedule_55_e809);
        (noise_metadata_schedule_55_e810,)
    } else {
        (noise_variable_11,)
    }
};
            noise_variable_11 = noise_metadata_schedule_55_e812;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_56_e815: f64 = (noise_variable_5 + noise_variable_11);
            noise_variable_2 = noise_metadata_schedule_56_e815;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_57_e818: f64 = (noise_variable_2 / noise_variable_3);
            noise_variable_4 = noise_metadata_schedule_57_e818;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_58_e821: f64 = (8.617086918058125e-5 * noise_variable_2);
            noise_variable_6 = noise_metadata_schedule_58_e821;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_59_e824: f64 = (8.617086918058125e-5 * noise_variable_3);
            noise_variable_7 = noise_metadata_schedule_59_e824;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_60_e827: f64 = (1.0 / noise_variable_6);
            noise_variable_8 = noise_metadata_schedule_60_e827;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_61_e830: f64 = (1.0 / noise_variable_7);
            noise_variable_9 = noise_metadata_schedule_61_e830;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_62_e833: f64 = (noise_variable_8 - noise_variable_9);
            noise_variable_10 = noise_metadata_schedule_62_e833;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_63_e836: f64 = (noise_variable_2 - noise_variable_3);
            noise_variable_12 = noise_metadata_schedule_63_e836;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_64_e838: f64 = (noise_variable_4).ln();
            noise_variable_260 = noise_metadata_schedule_64_e838;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_65_e842: f64 = (params.p114 * noise_variable_2);
            let noise_metadata_schedule_65_e844: f64 = (noise_metadata_schedule_65_e842 * noise_variable_2);
            let noise_metadata_schedule_65_e847: f64 = (noise_variable_2 + params.p115);
            let noise_metadata_schedule_65_e848: f64 = (noise_metadata_schedule_65_e844 / noise_metadata_schedule_65_e847);
            let noise_metadata_schedule_65_e849: f64 = (noise_variable_74 - noise_metadata_schedule_65_e848);
            let noise_metadata_schedule_65_e851: f64 = (noise_metadata_schedule_65_e849 - 0.05);
            let noise_metadata_schedule_65_e853: f64 = (noise_metadata_schedule_65_e851 / 0.1);
            noise_variable_265 = noise_metadata_schedule_65_e853;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_66_e857: f64 = (params.p114 * noise_variable_2);
            let noise_metadata_schedule_66_e859: f64 = (noise_metadata_schedule_66_e857 * noise_variable_2);
            let noise_metadata_schedule_66_e862: f64 = (noise_variable_2 + params.p115);
            let noise_metadata_schedule_66_e863: f64 = (noise_metadata_schedule_66_e859 / noise_metadata_schedule_66_e862);
            let noise_metadata_schedule_66_e864: f64 = (noise_variable_74 - noise_metadata_schedule_66_e863);
            let noise_metadata_schedule_66_e866: f64 = if noise_metadata_schedule_66_e864 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_453 = noise_metadata_schedule_66_e866;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_67_e878,) = {
    if (noise_variable_453 != 0.0) {
        let noise_metadata_schedule_67_e872: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_67_e873: f64 = (1.0 + noise_metadata_schedule_67_e872);
        let noise_metadata_schedule_67_e874: f64 = (noise_metadata_schedule_67_e873).ln();
        let noise_metadata_schedule_67_e875: f64 = (0.1 * noise_metadata_schedule_67_e874);
        let noise_metadata_schedule_67_e876: f64 = (0.05 + noise_metadata_schedule_67_e875);
        (noise_metadata_schedule_67_e876,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_67_e878;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_68_e902,) = {
    if (noise_variable_453 == 0.0) {
        let noise_metadata_schedule_68_e884: f64 = (params.p114 * noise_variable_2);
        let noise_metadata_schedule_68_e886: f64 = (noise_metadata_schedule_68_e884 * noise_variable_2);
        let noise_metadata_schedule_68_e889: f64 = (noise_variable_2 + params.p115);
        let noise_metadata_schedule_68_e890: f64 = (noise_metadata_schedule_68_e886 / noise_metadata_schedule_68_e889);
        let noise_metadata_schedule_68_e891: f64 = (noise_variable_74 - noise_metadata_schedule_68_e890);
        let noise_metadata_schedule_68_e895: f64 = (-noise_variable_265);
        let noise_metadata_schedule_68_e896: f64 = (noise_metadata_schedule_68_e895).exp();
        let noise_metadata_schedule_68_e897: f64 = (1.0 + noise_metadata_schedule_68_e896);
        let noise_metadata_schedule_68_e898: f64 = (noise_metadata_schedule_68_e897).ln();
        let noise_metadata_schedule_68_e899: f64 = (0.1 * noise_metadata_schedule_68_e898);
        let noise_metadata_schedule_68_e900: f64 = (noise_metadata_schedule_68_e891 + noise_metadata_schedule_68_e899);
        (noise_metadata_schedule_68_e900,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_68_e902;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_69_e906: f64 = (params.p117 * noise_variable_2);
            let noise_metadata_schedule_69_e908: f64 = (noise_metadata_schedule_69_e906 * noise_variable_2);
            let noise_metadata_schedule_69_e911: f64 = (noise_variable_2 + params.p118);
            let noise_metadata_schedule_69_e912: f64 = (noise_metadata_schedule_69_e908 / noise_metadata_schedule_69_e911);
            let noise_metadata_schedule_69_e913: f64 = (noise_variable_88 - noise_metadata_schedule_69_e912);
            let noise_metadata_schedule_69_e915: f64 = (noise_metadata_schedule_69_e913 - 0.05);
            let noise_metadata_schedule_69_e917: f64 = (noise_metadata_schedule_69_e915 / 0.1);
            noise_variable_265 = noise_metadata_schedule_69_e917;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_70_e921: f64 = (params.p117 * noise_variable_2);
            let noise_metadata_schedule_70_e923: f64 = (noise_metadata_schedule_70_e921 * noise_variable_2);
            let noise_metadata_schedule_70_e926: f64 = (noise_variable_2 + params.p118);
            let noise_metadata_schedule_70_e927: f64 = (noise_metadata_schedule_70_e923 / noise_metadata_schedule_70_e926);
            let noise_metadata_schedule_70_e928: f64 = (noise_variable_88 - noise_metadata_schedule_70_e927);
            let noise_metadata_schedule_70_e930: f64 = if noise_metadata_schedule_70_e928 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_454 = noise_metadata_schedule_70_e930;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_71_e942,) = {
    if (noise_variable_454 != 0.0) {
        let noise_metadata_schedule_71_e936: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_71_e937: f64 = (1.0 + noise_metadata_schedule_71_e936);
        let noise_metadata_schedule_71_e938: f64 = (noise_metadata_schedule_71_e937).ln();
        let noise_metadata_schedule_71_e939: f64 = (0.1 * noise_metadata_schedule_71_e938);
        let noise_metadata_schedule_71_e940: f64 = (0.05 + noise_metadata_schedule_71_e939);
        (noise_metadata_schedule_71_e940,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_71_e942;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_72_e966,) = {
    if (noise_variable_454 == 0.0) {
        let noise_metadata_schedule_72_e948: f64 = (params.p117 * noise_variable_2);
        let noise_metadata_schedule_72_e950: f64 = (noise_metadata_schedule_72_e948 * noise_variable_2);
        let noise_metadata_schedule_72_e953: f64 = (noise_variable_2 + params.p118);
        let noise_metadata_schedule_72_e954: f64 = (noise_metadata_schedule_72_e950 / noise_metadata_schedule_72_e953);
        let noise_metadata_schedule_72_e955: f64 = (noise_variable_88 - noise_metadata_schedule_72_e954);
        let noise_metadata_schedule_72_e959: f64 = (-noise_variable_265);
        let noise_metadata_schedule_72_e960: f64 = (noise_metadata_schedule_72_e959).exp();
        let noise_metadata_schedule_72_e961: f64 = (1.0 + noise_metadata_schedule_72_e960);
        let noise_metadata_schedule_72_e962: f64 = (noise_metadata_schedule_72_e961).ln();
        let noise_metadata_schedule_72_e963: f64 = (0.1 * noise_metadata_schedule_72_e962);
        let noise_metadata_schedule_72_e964: f64 = (noise_metadata_schedule_72_e955 + noise_metadata_schedule_72_e963);
        (noise_metadata_schedule_72_e964,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_72_e966;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_73_e968: f64 = (-3.0);
            let noise_metadata_schedule_73_e970: f64 = (noise_metadata_schedule_73_e968 * noise_variable_6);
            let noise_metadata_schedule_73_e972: f64 = (noise_metadata_schedule_73_e970 * noise_variable_260);
            let noise_metadata_schedule_73_e975: f64 = (params.p65 * noise_variable_4);
            let noise_metadata_schedule_73_e976: f64 = (noise_metadata_schedule_73_e972 + noise_metadata_schedule_73_e975);
            let noise_metadata_schedule_73_e979: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_73_e981: f64 = (noise_metadata_schedule_73_e979 * params.p104);
            let noise_metadata_schedule_73_e982: f64 = (noise_metadata_schedule_73_e976 + noise_metadata_schedule_73_e981);
            noise_variable_13 = noise_metadata_schedule_73_e982;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_74_e985: f64 = (0.05 - noise_variable_13);
            let noise_metadata_schedule_74_e987: f64 = (noise_metadata_schedule_74_e985 / noise_variable_6);
            noise_variable_265 = noise_metadata_schedule_74_e987;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_75_e990: f64 = if 0.05 < noise_variable_13 { 1.0 } else { 0.0 };
            noise_variable_455 = noise_metadata_schedule_75_e990;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_76_e1002,) = {
    if (noise_variable_455 != 0.0) {
        let noise_metadata_schedule_76_e996: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_76_e997: f64 = (1.0 + noise_metadata_schedule_76_e996);
        let noise_metadata_schedule_76_e998: f64 = (noise_metadata_schedule_76_e997).ln();
        let noise_metadata_schedule_76_e999: f64 = (noise_variable_6 * noise_metadata_schedule_76_e998);
        let noise_metadata_schedule_76_e1000: f64 = (noise_variable_13 + noise_metadata_schedule_76_e999);
        (noise_metadata_schedule_76_e1000,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_76_e1002;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_77_e1016,) = {
    if (noise_variable_455 == 0.0) {
        let noise_metadata_schedule_77_e1009: f64 = (-noise_variable_265);
        let noise_metadata_schedule_77_e1010: f64 = (noise_metadata_schedule_77_e1009).exp();
        let noise_metadata_schedule_77_e1011: f64 = (1.0 + noise_metadata_schedule_77_e1010);
        let noise_metadata_schedule_77_e1012: f64 = (noise_metadata_schedule_77_e1011).ln();
        let noise_metadata_schedule_77_e1013: f64 = (noise_variable_6 * noise_metadata_schedule_77_e1012);
        let noise_metadata_schedule_77_e1014: f64 = (0.05 + noise_metadata_schedule_77_e1013);
        (noise_metadata_schedule_77_e1014,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_77_e1016;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_78_e1018: f64 = (-3.0);
            let noise_metadata_schedule_78_e1020: f64 = (noise_metadata_schedule_78_e1018 * noise_variable_6);
            let noise_metadata_schedule_78_e1022: f64 = (noise_metadata_schedule_78_e1020 * noise_variable_260);
            let noise_metadata_schedule_78_e1025: f64 = (params.p63 * noise_variable_4);
            let noise_metadata_schedule_78_e1026: f64 = (noise_metadata_schedule_78_e1022 + noise_metadata_schedule_78_e1025);
            let noise_metadata_schedule_78_e1029: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_78_e1031: f64 = (noise_metadata_schedule_78_e1029 * params.p109);
            let noise_metadata_schedule_78_e1032: f64 = (noise_metadata_schedule_78_e1026 + noise_metadata_schedule_78_e1031);
            noise_variable_15 = noise_metadata_schedule_78_e1032;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_79_e1035: f64 = (0.05 - noise_variable_15);
            let noise_metadata_schedule_79_e1037: f64 = (noise_metadata_schedule_79_e1035 / noise_variable_6);
            noise_variable_265 = noise_metadata_schedule_79_e1037;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_80_e1040: f64 = if 0.05 < noise_variable_15 { 1.0 } else { 0.0 };
            noise_variable_456 = noise_metadata_schedule_80_e1040;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_81_e1052,) = {
    if (noise_variable_456 != 0.0) {
        let noise_metadata_schedule_81_e1046: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_81_e1047: f64 = (1.0 + noise_metadata_schedule_81_e1046);
        let noise_metadata_schedule_81_e1048: f64 = (noise_metadata_schedule_81_e1047).ln();
        let noise_metadata_schedule_81_e1049: f64 = (noise_variable_6 * noise_metadata_schedule_81_e1048);
        let noise_metadata_schedule_81_e1050: f64 = (noise_variable_15 + noise_metadata_schedule_81_e1049);
        (noise_metadata_schedule_81_e1050,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_81_e1052;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_82_e1066,) = {
    if (noise_variable_456 == 0.0) {
        let noise_metadata_schedule_82_e1059: f64 = (-noise_variable_265);
        let noise_metadata_schedule_82_e1060: f64 = (noise_metadata_schedule_82_e1059).exp();
        let noise_metadata_schedule_82_e1061: f64 = (1.0 + noise_metadata_schedule_82_e1060);
        let noise_metadata_schedule_82_e1062: f64 = (noise_metadata_schedule_82_e1061).ln();
        let noise_metadata_schedule_82_e1063: f64 = (noise_variable_6 * noise_metadata_schedule_82_e1062);
        let noise_metadata_schedule_82_e1064: f64 = (0.05 + noise_metadata_schedule_82_e1063);
        (noise_metadata_schedule_82_e1064,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_82_e1066;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_88_e1118: f64 = (-3.0);
            let noise_metadata_schedule_88_e1120: f64 = (noise_metadata_schedule_88_e1118 * noise_variable_6);
            let noise_metadata_schedule_88_e1122: f64 = (noise_metadata_schedule_88_e1120 * noise_variable_260);
            let noise_metadata_schedule_88_e1125: f64 = (params.p70 * noise_variable_4);
            let noise_metadata_schedule_88_e1126: f64 = (noise_metadata_schedule_88_e1122 + noise_metadata_schedule_88_e1125);
            let noise_metadata_schedule_88_e1129: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_88_e1131: f64 = (noise_metadata_schedule_88_e1129 * params.p109);
            let noise_metadata_schedule_88_e1132: f64 = (noise_metadata_schedule_88_e1126 + noise_metadata_schedule_88_e1131);
            noise_variable_18 = noise_metadata_schedule_88_e1132;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_89_e1135: f64 = (0.05 - noise_variable_18);
            let noise_metadata_schedule_89_e1137: f64 = (noise_metadata_schedule_89_e1135 / noise_variable_6);
            noise_variable_265 = noise_metadata_schedule_89_e1137;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_90_e1140: f64 = if 0.05 < noise_variable_18 { 1.0 } else { 0.0 };
            noise_variable_458 = noise_metadata_schedule_90_e1140;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_91_e1152,) = {
    if (noise_variable_458 != 0.0) {
        let noise_metadata_schedule_91_e1146: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_91_e1147: f64 = (1.0 + noise_metadata_schedule_91_e1146);
        let noise_metadata_schedule_91_e1148: f64 = (noise_metadata_schedule_91_e1147).ln();
        let noise_metadata_schedule_91_e1149: f64 = (noise_variable_6 * noise_metadata_schedule_91_e1148);
        let noise_metadata_schedule_91_e1150: f64 = (noise_variable_18 + noise_metadata_schedule_91_e1149);
        (noise_metadata_schedule_91_e1150,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_91_e1152;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_92_e1166,) = {
    if (noise_variable_458 == 0.0) {
        let noise_metadata_schedule_92_e1159: f64 = (-noise_variable_265);
        let noise_metadata_schedule_92_e1160: f64 = (noise_metadata_schedule_92_e1159).exp();
        let noise_metadata_schedule_92_e1161: f64 = (1.0 + noise_metadata_schedule_92_e1160);
        let noise_metadata_schedule_92_e1162: f64 = (noise_metadata_schedule_92_e1161).ln();
        let noise_metadata_schedule_92_e1163: f64 = (noise_variable_6 * noise_metadata_schedule_92_e1162);
        let noise_metadata_schedule_92_e1164: f64 = (0.05 + noise_metadata_schedule_92_e1163);
        (noise_metadata_schedule_92_e1164,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_92_e1166;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_93_e1168: f64 = (-3.0);
            let noise_metadata_schedule_93_e1170: f64 = (noise_metadata_schedule_93_e1168 * noise_variable_6);
            let noise_metadata_schedule_93_e1172: f64 = (noise_metadata_schedule_93_e1170 * noise_variable_260);
            let noise_metadata_schedule_93_e1175: f64 = (noise_variable_75 * noise_variable_4);
            let noise_metadata_schedule_93_e1176: f64 = (noise_metadata_schedule_93_e1172 + noise_metadata_schedule_93_e1175);
            let noise_metadata_schedule_93_e1179: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_93_e1181: f64 = (noise_metadata_schedule_93_e1179 * params.p109);
            let noise_metadata_schedule_93_e1182: f64 = (noise_metadata_schedule_93_e1176 + noise_metadata_schedule_93_e1181);
            noise_variable_20 = noise_metadata_schedule_93_e1182;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_94_e1185: f64 = (0.05 - noise_variable_20);
            let noise_metadata_schedule_94_e1187: f64 = (noise_metadata_schedule_94_e1185 / noise_variable_6);
            noise_variable_265 = noise_metadata_schedule_94_e1187;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_95_e1190: f64 = if 0.05 < noise_variable_20 { 1.0 } else { 0.0 };
            noise_variable_459 = noise_metadata_schedule_95_e1190;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_96_e1202,) = {
    if (noise_variable_459 != 0.0) {
        let noise_metadata_schedule_96_e1196: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_96_e1197: f64 = (1.0 + noise_metadata_schedule_96_e1196);
        let noise_metadata_schedule_96_e1198: f64 = (noise_metadata_schedule_96_e1197).ln();
        let noise_metadata_schedule_96_e1199: f64 = (noise_variable_6 * noise_metadata_schedule_96_e1198);
        let noise_metadata_schedule_96_e1200: f64 = (noise_variable_20 + noise_metadata_schedule_96_e1199);
        (noise_metadata_schedule_96_e1200,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_96_e1202;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_97_e1216,) = {
    if (noise_variable_459 == 0.0) {
        let noise_metadata_schedule_97_e1209: f64 = (-noise_variable_265);
        let noise_metadata_schedule_97_e1210: f64 = (noise_metadata_schedule_97_e1209).exp();
        let noise_metadata_schedule_97_e1211: f64 = (1.0 + noise_metadata_schedule_97_e1210);
        let noise_metadata_schedule_97_e1212: f64 = (noise_metadata_schedule_97_e1211).ln();
        let noise_metadata_schedule_97_e1213: f64 = (noise_variable_6 * noise_metadata_schedule_97_e1212);
        let noise_metadata_schedule_97_e1214: f64 = (0.05 + noise_metadata_schedule_97_e1213);
        (noise_metadata_schedule_97_e1214,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_97_e1216;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_98_e1218: f64 = (-3.0);
            let noise_metadata_schedule_98_e1220: f64 = (noise_metadata_schedule_98_e1218 * noise_variable_6);
            let noise_metadata_schedule_98_e1222: f64 = (noise_metadata_schedule_98_e1220 * noise_variable_260);
            let noise_metadata_schedule_98_e1225: f64 = (params.p26 * noise_variable_4);
            let noise_metadata_schedule_98_e1226: f64 = (noise_metadata_schedule_98_e1222 + noise_metadata_schedule_98_e1225);
            let noise_metadata_schedule_98_e1229: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_98_e1231: f64 = (noise_metadata_schedule_98_e1229 * params.p108);
            let noise_metadata_schedule_98_e1232: f64 = (noise_metadata_schedule_98_e1226 + noise_metadata_schedule_98_e1231);
            noise_variable_56 = noise_metadata_schedule_98_e1232;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_99_e1235: f64 = (0.05 - noise_variable_56);
            let noise_metadata_schedule_99_e1237: f64 = (noise_metadata_schedule_99_e1235 / noise_variable_6);
            noise_variable_265 = noise_metadata_schedule_99_e1237;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_100_e1240: f64 = if 0.05 < noise_variable_56 { 1.0 } else { 0.0 };
            noise_variable_460 = noise_metadata_schedule_100_e1240;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_101_e1252,) = {
    if (noise_variable_460 != 0.0) {
        let noise_metadata_schedule_101_e1246: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_101_e1247: f64 = (1.0 + noise_metadata_schedule_101_e1246);
        let noise_metadata_schedule_101_e1248: f64 = (noise_metadata_schedule_101_e1247).ln();
        let noise_metadata_schedule_101_e1249: f64 = (noise_variable_6 * noise_metadata_schedule_101_e1248);
        let noise_metadata_schedule_101_e1250: f64 = (noise_variable_56 + noise_metadata_schedule_101_e1249);
        (noise_metadata_schedule_101_e1250,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_101_e1252;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_102_e1266,) = {
    if (noise_variable_460 == 0.0) {
        let noise_metadata_schedule_102_e1259: f64 = (-noise_variable_265);
        let noise_metadata_schedule_102_e1260: f64 = (noise_metadata_schedule_102_e1259).exp();
        let noise_metadata_schedule_102_e1261: f64 = (1.0 + noise_metadata_schedule_102_e1260);
        let noise_metadata_schedule_102_e1262: f64 = (noise_metadata_schedule_102_e1261).ln();
        let noise_metadata_schedule_102_e1263: f64 = (noise_variable_6 * noise_metadata_schedule_102_e1262);
        let noise_metadata_schedule_102_e1264: f64 = (0.05 + noise_metadata_schedule_102_e1263);
        (noise_metadata_schedule_102_e1264,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_102_e1266;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_103_e1269: f64 = (1.0 / noise_variable_14);
            noise_variable_65 = noise_metadata_schedule_103_e1269;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_104_e1272: f64 = (1.0 / noise_variable_19);
            noise_variable_67 = noise_metadata_schedule_104_e1272;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_105_e1275: f64 = (params.p65 * noise_variable_65);
            let noise_metadata_schedule_105_e1277: f64 = (noise_metadata_schedule_105_e1275).powf(params.p66);
            noise_variable_73 = noise_metadata_schedule_105_e1277;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_106_e1280: f64 = (noise_variable_75 * noise_variable_67);
            let noise_metadata_schedule_106_e1282: f64 = (noise_metadata_schedule_106_e1280).powf(noise_variable_76);
            noise_variable_90 = noise_metadata_schedule_106_e1282;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_108_e1288: f64 = (1.0 - params.p74);
            let noise_metadata_schedule_108_e1291: f64 = (params.p70 / noise_variable_17);
            let noise_metadata_schedule_108_e1293: f64 = (noise_metadata_schedule_108_e1291).powf(params.p71);
            let noise_metadata_schedule_108_e1294: f64 = (noise_metadata_schedule_108_e1288 * noise_metadata_schedule_108_e1293);
            let noise_metadata_schedule_108_e1296: f64 = (noise_metadata_schedule_108_e1294 + params.p74);
            noise_variable_26 = noise_metadata_schedule_108_e1296;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_109_e1299: f64 = (1.0 / noise_variable_26);
            noise_variable_27 = noise_metadata_schedule_109_e1299;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_111_e1305: f64 = (params.p74 * noise_variable_27);
            noise_variable_25 = noise_metadata_schedule_111_e1305;
        }
        if matches!(source_index, 1 | 3) {
            let noise_metadata_schedule_112_e1309: f64 = (noise_variable_260 * params.p96);
            let noise_metadata_schedule_112_e1310: f64 = (noise_metadata_schedule_112_e1309).exp();
            let noise_metadata_schedule_112_e1311: f64 = (params.p53 * noise_metadata_schedule_112_e1310);
            noise_variable_28 = noise_metadata_schedule_112_e1311;
        }
        if matches!(source_index, 1 | 3) {
            let noise_metadata_schedule_113_e1314: f64 = if noise_variable_28 < noise_variable_322 { 1.0 } else { 0.0 };
            noise_variable_461 = noise_metadata_schedule_113_e1314;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_114_e1318,) = {
    if (noise_variable_461 != 0.0) {
        (noise_variable_322,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_114_e1318;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_115_e1323: f64 = (params.p97 - params.p95);
            let noise_metadata_schedule_115_e1324: f64 = (noise_variable_260 * noise_metadata_schedule_115_e1323);
            let noise_metadata_schedule_115_e1325: f64 = (noise_metadata_schedule_115_e1324).exp();
            let noise_metadata_schedule_115_e1326: f64 = (params.p55 * noise_metadata_schedule_115_e1325);
            noise_variable_29 = noise_metadata_schedule_115_e1326;
        }
        if matches!(source_index, 1 | 4) {
            let noise_metadata_schedule_116_e1330: f64 = (noise_variable_260 * params.p100);
            let noise_metadata_schedule_116_e1331: f64 = (noise_metadata_schedule_116_e1330).exp();
            let noise_metadata_schedule_116_e1332: f64 = (params.p54 * noise_metadata_schedule_116_e1331);
            noise_variable_30 = noise_metadata_schedule_116_e1332;
        }
        if matches!(source_index, 1 | 4) {
            let noise_metadata_schedule_117_e1335: f64 = if noise_variable_30 < noise_variable_322 { 1.0 } else { 0.0 };
            noise_variable_462 = noise_metadata_schedule_117_e1335;
        }
        if matches!(source_index, 1 | 4) {
            let (noise_metadata_schedule_118_e1339,) = {
    if (noise_variable_462 != 0.0) {
        (noise_variable_322,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_118_e1339;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 20 | 22 | 24) {
            let noise_metadata_schedule_119_e1343: f64 = (noise_variable_260 * params.p101);
            let noise_metadata_schedule_119_e1344: f64 = (noise_metadata_schedule_119_e1343).exp();
            let noise_metadata_schedule_119_e1345: f64 = (params.p56 * noise_metadata_schedule_119_e1344);
            noise_variable_32 = noise_metadata_schedule_119_e1345;
        }
        if matches!(source_index, 18 | 21) {
            let noise_metadata_schedule_120_e1349: f64 = (noise_variable_260 * params.p103);
            let noise_metadata_schedule_120_e1350: f64 = (noise_metadata_schedule_120_e1349).exp();
            let noise_metadata_schedule_120_e1351: f64 = (params.p57 * noise_metadata_schedule_120_e1350);
            noise_variable_33 = noise_metadata_schedule_120_e1351;
        }
        if matches!(source_index, 19 | 23) {
            let noise_metadata_schedule_121_e1355: f64 = (noise_variable_260 * params.p103);
            let noise_metadata_schedule_121_e1356: f64 = (noise_metadata_schedule_121_e1355).exp();
            let noise_metadata_schedule_121_e1357: f64 = (params.p58 * noise_metadata_schedule_121_e1356);
            noise_variable_34 = noise_metadata_schedule_121_e1357;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_122_e1361: f64 = (noise_variable_260 * params.p98);
            let noise_metadata_schedule_122_e1362: f64 = (noise_metadata_schedule_122_e1361).exp();
            let noise_metadata_schedule_122_e1363: f64 = (params.p59 * noise_metadata_schedule_122_e1362);
            noise_variable_31 = noise_metadata_schedule_122_e1363;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_123_e1366: f64 = if params.p121 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_463 = noise_metadata_schedule_123_e1366;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_124_e1376,) = {
    if (noise_variable_463 != 0.0) {
        let noise_metadata_schedule_124_e1372: f64 = (noise_variable_12 * params.p121);
        let noise_metadata_schedule_124_e1373: f64 = (1.0 + noise_metadata_schedule_124_e1372);
        let noise_metadata_schedule_124_e1374: f64 = (params.p9 * noise_metadata_schedule_124_e1373);
        (noise_metadata_schedule_124_e1374,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_124_e1376;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_125_e1384,) = {
    if (noise_variable_463 != 0.0) {
        let noise_metadata_schedule_125_e1380: f64 = (noise_variable_50 - 1.0);
        let noise_metadata_schedule_125_e1382: f64 = (noise_metadata_schedule_125_e1380 / noise_variable_52);
        (noise_metadata_schedule_125_e1382,)
    } else {
        (noise_variable_265,)
    }
};
            noise_variable_265 = noise_metadata_schedule_125_e1384;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_126_e1387: f64 = if noise_variable_50 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_464 = noise_metadata_schedule_126_e1387;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_127_e1401,) = {
    if ((noise_variable_463 != 0.0) && (noise_variable_464 != 0.0)) {
        let noise_metadata_schedule_127_e1395: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_127_e1396: f64 = (1.0 + noise_metadata_schedule_127_e1395);
        let noise_metadata_schedule_127_e1397: f64 = (noise_metadata_schedule_127_e1396).ln();
        let noise_metadata_schedule_127_e1398: f64 = (noise_variable_52 * noise_metadata_schedule_127_e1397);
        let noise_metadata_schedule_127_e1399: f64 = (1.0 + noise_metadata_schedule_127_e1398);
        (noise_metadata_schedule_127_e1399,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_127_e1401;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_128_e1417,) = {
    if ((noise_variable_463 != 0.0) && (noise_variable_464 == 0.0)) {
        let noise_metadata_schedule_128_e1410: f64 = (-noise_variable_265);
        let noise_metadata_schedule_128_e1411: f64 = (noise_metadata_schedule_128_e1410).exp();
        let noise_metadata_schedule_128_e1412: f64 = (1.0 + noise_metadata_schedule_128_e1411);
        let noise_metadata_schedule_128_e1413: f64 = (noise_metadata_schedule_128_e1412).ln();
        let noise_metadata_schedule_128_e1414: f64 = (noise_variable_52 * noise_metadata_schedule_128_e1413);
        let noise_metadata_schedule_128_e1415: f64 = (noise_variable_50 + noise_metadata_schedule_128_e1414);
        (noise_metadata_schedule_128_e1415,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_128_e1417;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_129_e1425,) = {
    if (noise_variable_463 != 0.0) {
        let noise_metadata_schedule_129_e1422: f64 = (noise_variable_52 * 0.6931471805599453);
        let noise_metadata_schedule_129_e1423: f64 = (noise_variable_50 - noise_metadata_schedule_129_e1422);
        (noise_metadata_schedule_129_e1423,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_129_e1425;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_130_e1430,) = {
    if (noise_variable_463 == 0.0) {
        (params.p9,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_130_e1430;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_131_e1433: f64 = if params.p122 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_465 = noise_metadata_schedule_131_e1433;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_132_e1443,) = {
    if (noise_variable_465 != 0.0) {
        let noise_metadata_schedule_132_e1439: f64 = (noise_variable_12 * params.p122);
        let noise_metadata_schedule_132_e1440: f64 = (1.0 + noise_metadata_schedule_132_e1439);
        let noise_metadata_schedule_132_e1441: f64 = (params.p10 * noise_metadata_schedule_132_e1440);
        (noise_metadata_schedule_132_e1441,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_132_e1443;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_133_e1451,) = {
    if (noise_variable_465 != 0.0) {
        let noise_metadata_schedule_133_e1447: f64 = (noise_variable_51 - 1.0);
        let noise_metadata_schedule_133_e1449: f64 = (noise_metadata_schedule_133_e1447 / noise_variable_52);
        (noise_metadata_schedule_133_e1449,)
    } else {
        (noise_variable_265,)
    }
};
            noise_variable_265 = noise_metadata_schedule_133_e1451;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_134_e1454: f64 = if noise_variable_51 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_466 = noise_metadata_schedule_134_e1454;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_135_e1468,) = {
    if ((noise_variable_465 != 0.0) && (noise_variable_466 != 0.0)) {
        let noise_metadata_schedule_135_e1462: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_135_e1463: f64 = (1.0 + noise_metadata_schedule_135_e1462);
        let noise_metadata_schedule_135_e1464: f64 = (noise_metadata_schedule_135_e1463).ln();
        let noise_metadata_schedule_135_e1465: f64 = (noise_variable_52 * noise_metadata_schedule_135_e1464);
        let noise_metadata_schedule_135_e1466: f64 = (1.0 + noise_metadata_schedule_135_e1465);
        (noise_metadata_schedule_135_e1466,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_135_e1468;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_136_e1484,) = {
    if ((noise_variable_465 != 0.0) && (noise_variable_466 == 0.0)) {
        let noise_metadata_schedule_136_e1477: f64 = (-noise_variable_265);
        let noise_metadata_schedule_136_e1478: f64 = (noise_metadata_schedule_136_e1477).exp();
        let noise_metadata_schedule_136_e1479: f64 = (1.0 + noise_metadata_schedule_136_e1478);
        let noise_metadata_schedule_136_e1480: f64 = (noise_metadata_schedule_136_e1479).ln();
        let noise_metadata_schedule_136_e1481: f64 = (noise_variable_52 * noise_metadata_schedule_136_e1480);
        let noise_metadata_schedule_136_e1482: f64 = (noise_variable_51 + noise_metadata_schedule_136_e1481);
        (noise_metadata_schedule_136_e1482,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_136_e1484;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_137_e1492,) = {
    if (noise_variable_465 != 0.0) {
        let noise_metadata_schedule_137_e1489: f64 = (noise_variable_52 * 0.6931471805599453);
        let noise_metadata_schedule_137_e1490: f64 = (noise_variable_51 - noise_metadata_schedule_137_e1489);
        (noise_metadata_schedule_137_e1490,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_137_e1492;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_138_e1497,) = {
    if (noise_variable_465 == 0.0) {
        (params.p10,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_138_e1497;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_139_e1502: f64 = (params.p123 * noise_variable_12);
            let noise_metadata_schedule_139_e1503: f64 = (1.0 + noise_metadata_schedule_139_e1502);
            let noise_metadata_schedule_139_e1504: f64 = (params.p42 * noise_metadata_schedule_139_e1503);
            noise_variable_317 = noise_metadata_schedule_139_e1504;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_140_e1507: f64 = (noise_variable_318 * noise_variable_318);
            noise_variable_267 = noise_metadata_schedule_140_e1507;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_141_e1510: f64 = (noise_variable_317 * noise_variable_317);
            noise_variable_268 = noise_metadata_schedule_141_e1510;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_142_e1513: f64 = if noise_variable_317 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_467 = noise_metadata_schedule_142_e1513;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_143_e1526,) = {
    if (noise_variable_467 != 0.0) {
        let noise_metadata_schedule_143_e1517: f64 = (0.5 * noise_variable_267);
        let noise_metadata_schedule_143_e1520: f64 = (noise_variable_268 + noise_variable_267);
        let noise_metadata_schedule_143_e1521: f64 = (noise_metadata_schedule_143_e1520).sqrt();
        let noise_metadata_schedule_143_e1523: f64 = (noise_metadata_schedule_143_e1521 - noise_variable_317);
        let noise_metadata_schedule_143_e1524: f64 = (noise_metadata_schedule_143_e1517 / noise_metadata_schedule_143_e1523);
        (noise_metadata_schedule_143_e1524,)
    } else {
        (noise_variable_316,)
    }
};
            noise_variable_316 = noise_metadata_schedule_143_e1526;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_144_e1538,) = {
    if (noise_variable_467 == 0.0) {
        let noise_metadata_schedule_144_e1532: f64 = (noise_variable_268 + noise_variable_267);
        let noise_metadata_schedule_144_e1533: f64 = (noise_metadata_schedule_144_e1532).sqrt();
        let noise_metadata_schedule_144_e1535: f64 = (noise_metadata_schedule_144_e1533 + noise_variable_317);
        let noise_metadata_schedule_144_e1536: f64 = (0.5 * noise_metadata_schedule_144_e1535);
        (noise_metadata_schedule_144_e1536,)
    } else {
        (noise_variable_316,)
    }
};
            noise_variable_316 = noise_metadata_schedule_144_e1538;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_145_e1543: f64 = (4.0 - params.p97);
            let noise_metadata_schedule_145_e1545: f64 = (noise_metadata_schedule_145_e1543 - params.p95);
            let noise_metadata_schedule_145_e1547: f64 = (noise_metadata_schedule_145_e1545 + params.p120);
            let noise_metadata_schedule_145_e1548: f64 = (noise_variable_260 * noise_metadata_schedule_145_e1547);
            let noise_metadata_schedule_145_e1550: f64 = (noise_metadata_schedule_145_e1548 / noise_variable_48);
            let noise_metadata_schedule_145_e1551: f64 = (noise_metadata_schedule_145_e1550).exp();
            let noise_metadata_schedule_145_e1552: f64 = (params.p8 * noise_metadata_schedule_145_e1551);
            let noise_metadata_schedule_145_e1554: f64 = (-params.p104);
            let noise_metadata_schedule_145_e1556: f64 = (noise_metadata_schedule_145_e1554 * noise_variable_10);
            let noise_metadata_schedule_145_e1558: f64 = (noise_metadata_schedule_145_e1556 / noise_variable_48);
            let noise_metadata_schedule_145_e1559: f64 = (noise_metadata_schedule_145_e1558).exp();
            let noise_metadata_schedule_145_e1560: f64 = (noise_metadata_schedule_145_e1552 * noise_metadata_schedule_145_e1559);
            noise_variable_35 = noise_metadata_schedule_145_e1560;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_146_e1565: f64 = (1.0 - params.p97);
            let noise_metadata_schedule_146_e1566: f64 = (noise_variable_260 * noise_metadata_schedule_146_e1565);
            let noise_metadata_schedule_146_e1567: f64 = (noise_metadata_schedule_146_e1566).exp();
            let noise_metadata_schedule_146_e1568: f64 = (params.p11 * noise_metadata_schedule_146_e1567);
            noise_variable_36 = noise_metadata_schedule_146_e1568;
        }
        if matches!(source_index, 11 | 12 | 13 | 14) {
            let noise_metadata_schedule_147_e1573: f64 = (1.0 - params.p102);
            let noise_metadata_schedule_147_e1574: f64 = (noise_variable_260 * noise_metadata_schedule_147_e1573);
            let noise_metadata_schedule_147_e1575: f64 = (noise_metadata_schedule_147_e1574).exp();
            let noise_metadata_schedule_147_e1576: f64 = (params.p29 * noise_metadata_schedule_147_e1575);
            noise_variable_37 = noise_metadata_schedule_147_e1576;
        }
        if matches!(source_index, 2 | 7) {
            let noise_metadata_schedule_148_e1582: f64 = (2.0 * params.p20);
            let noise_metadata_schedule_148_e1583: f64 = (6.0 - noise_metadata_schedule_148_e1582);
            let noise_metadata_schedule_148_e1584: f64 = (noise_variable_260 * noise_metadata_schedule_148_e1583);
            let noise_metadata_schedule_148_e1585: f64 = (noise_metadata_schedule_148_e1584).exp();
            let noise_metadata_schedule_148_e1586: f64 = (params.p19 * noise_metadata_schedule_148_e1585);
            let noise_metadata_schedule_148_e1588: f64 = (-params.p112);
            let noise_metadata_schedule_148_e1590: f64 = (noise_metadata_schedule_148_e1588 * noise_variable_10);
            let noise_metadata_schedule_148_e1592: f64 = (noise_metadata_schedule_148_e1590 / params.p20);
            let noise_metadata_schedule_148_e1593: f64 = (noise_metadata_schedule_148_e1592).exp();
            let noise_metadata_schedule_148_e1594: f64 = (noise_metadata_schedule_148_e1586 * noise_metadata_schedule_148_e1593);
            noise_variable_38 = noise_metadata_schedule_148_e1594;
        }
        if matches!(source_index, 9 | 10) {
            let noise_metadata_schedule_149_e1600: f64 = (2.0 * params.p31);
            let noise_metadata_schedule_149_e1601: f64 = (6.0 - noise_metadata_schedule_149_e1600);
            let noise_metadata_schedule_149_e1602: f64 = (noise_variable_260 * noise_metadata_schedule_149_e1601);
            let noise_metadata_schedule_149_e1603: f64 = (noise_metadata_schedule_149_e1602).exp();
            let noise_metadata_schedule_149_e1604: f64 = (params.p30 * noise_metadata_schedule_149_e1603);
            let noise_metadata_schedule_149_e1606: f64 = (-params.p109);
            let noise_metadata_schedule_149_e1608: f64 = (noise_metadata_schedule_149_e1606 * noise_variable_10);
            let noise_metadata_schedule_149_e1610: f64 = (noise_metadata_schedule_149_e1608 / params.p31);
            let noise_metadata_schedule_149_e1611: f64 = (noise_metadata_schedule_149_e1610).exp();
            let noise_metadata_schedule_149_e1612: f64 = (noise_metadata_schedule_149_e1604 * noise_metadata_schedule_149_e1611);
            noise_variable_39 = noise_metadata_schedule_149_e1612;
        }
        if matches!(source_index, 1 | 2 | 6) {
            let noise_metadata_schedule_150_e1617: f64 = (4.0 - params.p96);
            let noise_metadata_schedule_150_e1619: f64 = (noise_metadata_schedule_150_e1617 + params.p120);
            let noise_metadata_schedule_150_e1620: f64 = (noise_variable_260 * noise_metadata_schedule_150_e1619);
            let noise_metadata_schedule_150_e1622: f64 = (noise_metadata_schedule_150_e1620 / params.p16);
            let noise_metadata_schedule_150_e1623: f64 = (noise_metadata_schedule_150_e1622).exp();
            let noise_metadata_schedule_150_e1624: f64 = (params.p15 * noise_metadata_schedule_150_e1623);
            let noise_metadata_schedule_150_e1626: f64 = (-params.p110);
            let noise_metadata_schedule_150_e1628: f64 = (noise_metadata_schedule_150_e1626 * noise_variable_10);
            let noise_metadata_schedule_150_e1630: f64 = (noise_metadata_schedule_150_e1628 / params.p16);
            let noise_metadata_schedule_150_e1631: f64 = (noise_metadata_schedule_150_e1630).exp();
            let noise_metadata_schedule_150_e1632: f64 = (noise_metadata_schedule_150_e1624 * noise_metadata_schedule_150_e1631);
            noise_variable_42 = noise_metadata_schedule_150_e1632;
        }
        if matches!(source_index, 6 | 8) {
            let noise_metadata_schedule_151_e1637: f64 = (4.0 - params.p96);
            let noise_metadata_schedule_151_e1639: f64 = (noise_metadata_schedule_151_e1637 + params.p120);
            let noise_metadata_schedule_151_e1640: f64 = (noise_variable_260 * noise_metadata_schedule_151_e1639);
            let noise_metadata_schedule_151_e1642: f64 = (noise_metadata_schedule_151_e1640 / params.p18);
            let noise_metadata_schedule_151_e1643: f64 = (noise_metadata_schedule_151_e1642).exp();
            let noise_metadata_schedule_151_e1644: f64 = (params.p17 * noise_metadata_schedule_151_e1643);
            let noise_metadata_schedule_151_e1646: f64 = (-params.p110);
            let noise_metadata_schedule_151_e1648: f64 = (noise_metadata_schedule_151_e1646 * noise_variable_10);
            let noise_metadata_schedule_151_e1650: f64 = (noise_metadata_schedule_151_e1648 / params.p18);
            let noise_metadata_schedule_151_e1651: f64 = (noise_metadata_schedule_151_e1650).exp();
            let noise_metadata_schedule_151_e1652: f64 = (noise_metadata_schedule_151_e1644 * noise_metadata_schedule_151_e1651);
            noise_variable_44 = noise_metadata_schedule_151_e1652;
        }
        if matches!(source_index, 2 | 6 | 8) {
            let noise_metadata_schedule_152_e1655: f64 = if params.p23 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_468 = noise_metadata_schedule_152_e1655;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_153_e1667,) = {
    if (noise_variable_468 != 0.0) {
        let noise_metadata_schedule_153_e1659: f64 = (-params.p106);
        let noise_metadata_schedule_153_e1661: f64 = (noise_metadata_schedule_153_e1659 * noise_variable_10);
        let noise_metadata_schedule_153_e1663: f64 = (noise_metadata_schedule_153_e1661 / params.p16);
        let noise_metadata_schedule_153_e1664: f64 = (noise_metadata_schedule_153_e1663).exp();
        let noise_metadata_schedule_153_e1665: f64 = (params.p24 * noise_metadata_schedule_153_e1664);
        (noise_metadata_schedule_153_e1665,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_153_e1667;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_154_e1677,) = {
    if (noise_variable_468 != 0.0) {
        let noise_metadata_schedule_154_e1671: f64 = (-params.p105);
        let noise_metadata_schedule_154_e1673: f64 = (noise_metadata_schedule_154_e1671 * noise_variable_10);
        let noise_metadata_schedule_154_e1674: f64 = (noise_metadata_schedule_154_e1673).exp();
        let noise_metadata_schedule_154_e1675: f64 = (params.p27 * noise_metadata_schedule_154_e1674);
        (noise_metadata_schedule_154_e1675,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_154_e1677;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_155_e1689,) = {
    if (noise_variable_468 != 0.0) {
        let noise_metadata_schedule_155_e1681: f64 = (-params.p107);
        let noise_metadata_schedule_155_e1683: f64 = (noise_metadata_schedule_155_e1681 * noise_variable_10);
        let noise_metadata_schedule_155_e1685: f64 = (noise_metadata_schedule_155_e1683 / params.p18);
        let noise_metadata_schedule_155_e1686: f64 = (noise_metadata_schedule_155_e1685).exp();
        let noise_metadata_schedule_155_e1687: f64 = (params.p25 * noise_metadata_schedule_155_e1686);
        (noise_metadata_schedule_155_e1687,)
    } else {
        (noise_variable_45,)
    }
};
            noise_variable_45 = noise_metadata_schedule_155_e1689;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_156_e1694: f64 = (4.0 - params.p102);
            let noise_metadata_schedule_156_e1696: f64 = (noise_metadata_schedule_156_e1694 + params.p120);
            let noise_metadata_schedule_156_e1697: f64 = (noise_variable_260 * noise_metadata_schedule_156_e1696);
            let noise_metadata_schedule_156_e1698: f64 = (noise_metadata_schedule_156_e1697).exp();
            let noise_metadata_schedule_156_e1699: f64 = (params.p28 * noise_metadata_schedule_156_e1698);
            let noise_metadata_schedule_156_e1701: f64 = (-params.p111);
            let noise_metadata_schedule_156_e1703: f64 = (noise_metadata_schedule_156_e1701 * noise_variable_10);
            let noise_metadata_schedule_156_e1704: f64 = (noise_metadata_schedule_156_e1703).exp();
            let noise_metadata_schedule_156_e1705: f64 = (noise_metadata_schedule_156_e1699 * noise_metadata_schedule_156_e1704);
            noise_variable_43 = noise_metadata_schedule_156_e1705;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_157_e1711: f64 = (2.0 * params.p22);
            let noise_metadata_schedule_157_e1712: f64 = (6.0 - noise_metadata_schedule_157_e1711);
            let noise_metadata_schedule_157_e1713: f64 = (noise_variable_260 * noise_metadata_schedule_157_e1712);
            let noise_metadata_schedule_157_e1714: f64 = (noise_metadata_schedule_157_e1713).exp();
            let noise_metadata_schedule_157_e1715: f64 = (params.p21 * noise_metadata_schedule_157_e1714);
            let noise_metadata_schedule_157_e1717: f64 = (-params.p112);
            let noise_metadata_schedule_157_e1719: f64 = (noise_metadata_schedule_157_e1717 * noise_variable_10);
            let noise_metadata_schedule_157_e1721: f64 = (noise_metadata_schedule_157_e1719 / params.p22);
            let noise_metadata_schedule_157_e1722: f64 = (noise_metadata_schedule_157_e1721).exp();
            let noise_metadata_schedule_157_e1723: f64 = (noise_metadata_schedule_157_e1715 * noise_metadata_schedule_157_e1722);
            noise_variable_46 = noise_metadata_schedule_157_e1723;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_158_e1728: f64 = (4.0 / params.p137);
            let noise_metadata_schedule_158_e1729: f64 = (noise_variable_260 * noise_metadata_schedule_158_e1728);
            let noise_metadata_schedule_158_e1730: f64 = (noise_metadata_schedule_158_e1729).exp();
            let noise_metadata_schedule_158_e1731: f64 = (params.p136 * noise_metadata_schedule_158_e1730);
            let noise_metadata_schedule_158_e1733: f64 = (-params.p112);
            let noise_metadata_schedule_158_e1735: f64 = (noise_metadata_schedule_158_e1733 * noise_variable_10);
            let noise_metadata_schedule_158_e1737: f64 = (noise_metadata_schedule_158_e1735 / params.p137);
            let noise_metadata_schedule_158_e1738: f64 = (noise_metadata_schedule_158_e1737).exp();
            let noise_metadata_schedule_158_e1739: f64 = (noise_metadata_schedule_158_e1731 * noise_metadata_schedule_158_e1738);
            noise_variable_47 = noise_metadata_schedule_158_e1739;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_159_e1742: f64 = (noise_variable_4).sqrt();
            let noise_metadata_schedule_159_e1743: f64 = (params.p142 * noise_metadata_schedule_159_e1742);
            let noise_metadata_schedule_159_e1746: f64 = (params.p144 * noise_variable_12);
            let noise_metadata_schedule_159_e1747: f64 = (noise_metadata_schedule_159_e1746).exp();
            let noise_metadata_schedule_159_e1748: f64 = (noise_metadata_schedule_159_e1743 * noise_metadata_schedule_159_e1747);
            noise_variable_332 = noise_metadata_schedule_159_e1748;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_160_e1751: f64 = (noise_variable_70 * noise_variable_72);
            let noise_metadata_schedule_160_e1753: f64 = (-0.5);
            let noise_metadata_schedule_160_e1754: f64 = (noise_metadata_schedule_160_e1751).powf(noise_metadata_schedule_160_e1753);
            noise_variable_261 = noise_metadata_schedule_160_e1754;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_161_e1757: f64 = (1.0 / noise_variable_73);
            noise_variable_262 = noise_metadata_schedule_161_e1757;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_162_e1760: f64 = (params.p34 * noise_variable_70);
            let noise_metadata_schedule_162_e1762: f64 = (noise_metadata_schedule_162_e1760 * noise_variable_70);
            let noise_metadata_schedule_162_e1764: f64 = (noise_metadata_schedule_162_e1762 * noise_variable_261);
            let noise_metadata_schedule_162_e1766: f64 = (noise_metadata_schedule_162_e1764 * noise_variable_262);
            let noise_metadata_schedule_162_e1768: f64 = (noise_metadata_schedule_162_e1766 * params.p65);
            let noise_metadata_schedule_162_e1770: f64 = (noise_metadata_schedule_162_e1768 * noise_variable_65);
            let noise_metadata_schedule_162_e1772: f64 = (noise_metadata_schedule_162_e1770 * noise_variable_72);
            let noise_metadata_schedule_162_e1774: f64 = (noise_metadata_schedule_162_e1772 * noise_variable_72);
            noise_variable_61 = noise_metadata_schedule_162_e1774;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_163_e1777: f64 = (params.p33 * noise_variable_261);
            let noise_metadata_schedule_163_e1779: f64 = (noise_metadata_schedule_163_e1777 * noise_variable_14);
            let noise_metadata_schedule_163_e1781: f64 = (noise_metadata_schedule_163_e1779 * noise_variable_14);
            let noise_metadata_schedule_163_e1783: f64 = (noise_metadata_schedule_163_e1781 * noise_variable_64);
            let noise_metadata_schedule_163_e1785: f64 = (noise_metadata_schedule_163_e1783 * noise_variable_64);
            let noise_metadata_schedule_163_e1787: f64 = (noise_metadata_schedule_163_e1785 * noise_variable_73);
            let noise_metadata_schedule_163_e1790: f64 = (params.p34 - noise_variable_61);
            let noise_metadata_schedule_163_e1791: f64 = (noise_metadata_schedule_163_e1790).exp();
            let noise_metadata_schedule_163_e1792: f64 = (noise_metadata_schedule_163_e1787 * noise_metadata_schedule_163_e1791);
            noise_variable_58 = noise_metadata_schedule_163_e1792;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_164_e1795: f64 = (1.0 / noise_variable_19);
            noise_variable_67 = noise_metadata_schedule_164_e1795;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_165_e1798: f64 = (noise_variable_85 * noise_variable_86);
            let noise_metadata_schedule_165_e1800: f64 = (-0.5);
            let noise_metadata_schedule_165_e1801: f64 = (noise_metadata_schedule_165_e1798).powf(noise_metadata_schedule_165_e1800);
            noise_variable_263 = noise_metadata_schedule_165_e1801;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_166_e1804: f64 = (1.0 / noise_variable_90);
            noise_variable_264 = noise_metadata_schedule_166_e1804;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_167_e1807: f64 = (params.p36 * noise_variable_85);
            let noise_metadata_schedule_167_e1809: f64 = (noise_metadata_schedule_167_e1807 * noise_variable_85);
            let noise_metadata_schedule_167_e1811: f64 = (noise_metadata_schedule_167_e1809 * noise_variable_263);
            let noise_metadata_schedule_167_e1813: f64 = (noise_metadata_schedule_167_e1811 * noise_variable_264);
            let noise_metadata_schedule_167_e1815: f64 = (noise_metadata_schedule_167_e1813 * noise_variable_75);
            let noise_metadata_schedule_167_e1817: f64 = (noise_metadata_schedule_167_e1815 * noise_variable_67);
            let noise_metadata_schedule_167_e1819: f64 = (noise_metadata_schedule_167_e1817 * noise_variable_86);
            let noise_metadata_schedule_167_e1821: f64 = (noise_metadata_schedule_167_e1819 * noise_variable_86);
            noise_variable_83 = noise_metadata_schedule_167_e1821;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_168_e1824: f64 = (params.p35 * noise_variable_263);
            let noise_metadata_schedule_168_e1826: f64 = (noise_metadata_schedule_168_e1824 * noise_variable_19);
            let noise_metadata_schedule_168_e1828: f64 = (noise_metadata_schedule_168_e1826 * noise_variable_19);
            let noise_metadata_schedule_168_e1830: f64 = (noise_metadata_schedule_168_e1828 * noise_variable_66);
            let noise_metadata_schedule_168_e1832: f64 = (noise_metadata_schedule_168_e1830 * noise_variable_66);
            let noise_metadata_schedule_168_e1834: f64 = (noise_metadata_schedule_168_e1832 * noise_variable_90);
            let noise_metadata_schedule_168_e1837: f64 = (params.p36 - noise_variable_83);
            let noise_metadata_schedule_168_e1838: f64 = (noise_metadata_schedule_168_e1837).exp();
            let noise_metadata_schedule_168_e1839: f64 = (noise_metadata_schedule_168_e1834 * noise_metadata_schedule_168_e1838);
            noise_variable_84 = noise_metadata_schedule_168_e1839;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_169_e1842: f64 = (noise_variable_260 * params.p95);
            let noise_metadata_schedule_169_e1843: f64 = (noise_metadata_schedule_169_e1842).exp();
            noise_variable_261 = noise_metadata_schedule_169_e1843;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_170_e1846: f64 = (params.p13 * noise_variable_261);
            let noise_metadata_schedule_170_e1848: f64 = (noise_metadata_schedule_170_e1846 * noise_variable_27);
            noise_variable_40 = noise_metadata_schedule_170_e1848;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_171_e1851: f64 = (params.p12 * noise_variable_261);
            let noise_metadata_schedule_171_e1853: f64 = (noise_metadata_schedule_171_e1851 * noise_variable_262);
            noise_variable_41 = noise_metadata_schedule_171_e1853;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_177_e1905: f64 = (noise_variable_2 - 300.0);
            noise_variable_101 = noise_metadata_schedule_177_e1905;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_178_e1908: f64 = if noise_variable_2 < 525.0 { 1.0 } else { 0.0 };
            noise_variable_469 = noise_metadata_schedule_178_e1908;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_179_e1924,) = {
    if (noise_variable_469 != 0.0) {
        let noise_metadata_schedule_179_e1914: f64 = (0.00072 * noise_variable_101);
        let noise_metadata_schedule_179_e1915: f64 = (1.0 + noise_metadata_schedule_179_e1914);
        let noise_metadata_schedule_179_e1918: f64 = (1.6e-6 * noise_variable_101);
        let noise_metadata_schedule_179_e1920: f64 = (noise_metadata_schedule_179_e1918 * noise_variable_101);
        let noise_metadata_schedule_179_e1921: f64 = (noise_metadata_schedule_179_e1915 - noise_metadata_schedule_179_e1920);
        let noise_metadata_schedule_179_e1922: f64 = (noise_variable_1 * noise_metadata_schedule_179_e1921);
        (noise_metadata_schedule_179_e1922,)
    } else {
        (noise_variable_99,)
    }
};
            noise_variable_99 = noise_metadata_schedule_179_e1924;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_180_e1931,) = {
    if (noise_variable_469 == 0.0) {
        let noise_metadata_schedule_180_e1929: f64 = (noise_variable_1 * 1.081);
        (noise_metadata_schedule_180_e1929,)
    } else {
        (noise_variable_99,)
    }
};
            noise_variable_99 = noise_metadata_schedule_180_e1931;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_181_e1935: f64 = (noise_variable_260 * params.p95);
            let noise_metadata_schedule_181_e1936: f64 = (noise_metadata_schedule_181_e1935).exp();
            let noise_metadata_schedule_181_e1937: f64 = (params.p91 * noise_metadata_schedule_181_e1936);
            noise_variable_100 = noise_metadata_schedule_181_e1937;
        }
        if matches!(source_index, 17 | 20 | 22 | 24) {
            let noise_metadata_schedule_183_e1947: f64 = if params.p56 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_470 = noise_metadata_schedule_183_e1947;
        }
        if matches!(source_index, 17 | 20 | 22 | 24) {
            let (noise_metadata_schedule_184_e1953,) = {
    if (noise_variable_470 != 0.0) {
        let noise_metadata_schedule_184_e1951: f64 = (1.0 / noise_variable_32);
        (noise_metadata_schedule_184_e1951,)
    } else {
        (noise_variable_104,)
    }
};
            noise_variable_104 = noise_metadata_schedule_184_e1953;
        }
        if matches!(source_index, 17 | 20 | 22 | 24) {
            let noise_metadata_schedule_185_e1956: f64 = if noise_variable_104 > noise_variable_323 { 1.0 } else { 0.0 };
            noise_variable_471 = noise_metadata_schedule_185_e1956;
        }
        if matches!(source_index, 17 | 20 | 22 | 24) {
            let (noise_metadata_schedule_186_e1962,) = {
    if ((noise_variable_470 != 0.0) && (noise_variable_471 != 0.0)) {
        (noise_variable_323,)
    } else {
        (noise_variable_104,)
    }
};
            noise_variable_104 = noise_metadata_schedule_186_e1962;
        }
        if matches!(source_index, 17 | 20 | 22 | 24) {
            let (noise_metadata_schedule_187_e1967,) = {
    if (noise_variable_470 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_104,)
    }
};
            noise_variable_104 = noise_metadata_schedule_187_e1967;
        }
        if matches!(source_index, 18 | 21) {
            let noise_metadata_schedule_188_e1970: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_472 = noise_metadata_schedule_188_e1970;
        }
        if matches!(source_index, 18 | 21) {
            let (noise_metadata_schedule_189_e1976,) = {
    if (noise_variable_472 != 0.0) {
        let noise_metadata_schedule_189_e1974: f64 = (1.0 / noise_variable_33);
        (noise_metadata_schedule_189_e1974,)
    } else {
        (noise_variable_105,)
    }
};
            noise_variable_105 = noise_metadata_schedule_189_e1976;
        }
        if matches!(source_index, 18 | 21) {
            let noise_metadata_schedule_190_e1979: f64 = if noise_variable_105 > noise_variable_323 { 1.0 } else { 0.0 };
            noise_variable_473 = noise_metadata_schedule_190_e1979;
        }
        if matches!(source_index, 18 | 21) {
            let (noise_metadata_schedule_191_e1985,) = {
    if ((noise_variable_472 != 0.0) && (noise_variable_473 != 0.0)) {
        (noise_variable_323,)
    } else {
        (noise_variable_105,)
    }
};
            noise_variable_105 = noise_metadata_schedule_191_e1985;
        }
        if matches!(source_index, 18 | 21) {
            let (noise_metadata_schedule_192_e1990,) = {
    if (noise_variable_472 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_105,)
    }
};
            noise_variable_105 = noise_metadata_schedule_192_e1990;
        }
        if matches!(source_index, 19 | 23) {
            let noise_metadata_schedule_193_e1993: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_474 = noise_metadata_schedule_193_e1993;
        }
        if matches!(source_index, 19 | 23) {
            let (noise_metadata_schedule_194_e1999,) = {
    if (noise_variable_474 != 0.0) {
        let noise_metadata_schedule_194_e1997: f64 = (1.0 / noise_variable_34);
        (noise_metadata_schedule_194_e1997,)
    } else {
        (noise_variable_106,)
    }
};
            noise_variable_106 = noise_metadata_schedule_194_e1999;
        }
        if matches!(source_index, 19 | 23) {
            let noise_metadata_schedule_195_e2002: f64 = if noise_variable_106 > noise_variable_323 { 1.0 } else { 0.0 };
            noise_variable_475 = noise_metadata_schedule_195_e2002;
        }
        if matches!(source_index, 19 | 23) {
            let (noise_metadata_schedule_196_e2008,) = {
    if ((noise_variable_474 != 0.0) && (noise_variable_475 != 0.0)) {
        (noise_variable_323,)
    } else {
        (noise_variable_106,)
    }
};
            noise_variable_106 = noise_metadata_schedule_196_e2008;
        }
        if matches!(source_index, 19 | 23) {
            let (noise_metadata_schedule_197_e2013,) = {
    if (noise_variable_474 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_106,)
    }
};
            noise_variable_106 = noise_metadata_schedule_197_e2013;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_198_e2016: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[7])));
            noise_variable_236 = noise_metadata_schedule_198_e2016;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_199_e2019: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[8])));
            noise_variable_237 = noise_metadata_schedule_199_e2019;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_200_e2022: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[4])));
            noise_variable_238 = noise_metadata_schedule_200_e2022;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_201_e2025: f64 = (params.p3 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[4])));
            noise_variable_239 = noise_metadata_schedule_201_e2025;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_202_e2028: f64 = (params.p3 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[6])));
            noise_variable_240 = noise_metadata_schedule_202_e2028;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_203_e2031: f64 = (params.p3 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[8])));
            noise_variable_242 = noise_metadata_schedule_203_e2031;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_205_e2037: f64 = (params.p3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_246 = noise_metadata_schedule_205_e2037;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_207_e2043: f64 = (params.p3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[0])));
            noise_variable_250 = noise_metadata_schedule_207_e2043;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_208_e2046: f64 = (params.p3 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[7])));
            noise_variable_244 = noise_metadata_schedule_208_e2046;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_209_e2049: f64 = (params.p3 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
            noise_variable_243 = noise_metadata_schedule_209_e2049;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_210_e2052: f64 = (noise_variable_240 + noise_variable_237);
            let noise_metadata_schedule_210_e2054: f64 = (noise_metadata_schedule_210_e2052 - noise_variable_242);
            let noise_metadata_schedule_210_e2056: f64 = (noise_metadata_schedule_210_e2054 - noise_variable_244);
            noise_variable_241 = noise_metadata_schedule_210_e2056;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_211_e2058: f64 = (-noise_variable_250);
            let noise_metadata_schedule_211_e2060: f64 = (noise_metadata_schedule_211_e2058 + noise_variable_246);
            let noise_metadata_schedule_211_e2062: f64 = (noise_metadata_schedule_211_e2060 + noise_variable_241);
            let noise_metadata_schedule_211_e2064: f64 = (noise_metadata_schedule_211_e2062 - noise_variable_243);
            noise_variable_248 = noise_metadata_schedule_211_e2064;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_212_e2067: f64 = (noise_variable_250 + noise_variable_248);
            noise_variable_247 = noise_metadata_schedule_212_e2067;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_213_e2070: f64 = (noise_variable_237 * noise_variable_8);
            let noise_metadata_schedule_213_e2072: f64 = if noise_metadata_schedule_213_e2070 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_476 = noise_metadata_schedule_213_e2072;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_214_e2079,) = {
    if (noise_variable_476 != 0.0) {
        let noise_metadata_schedule_214_e2076: f64 = (noise_variable_237 * noise_variable_8);
        let noise_metadata_schedule_214_e2077: f64 = (noise_metadata_schedule_214_e2076).exp();
        (noise_metadata_schedule_214_e2077,)
    } else {
        (noise_variable_251,)
    }
};
            noise_variable_251 = noise_metadata_schedule_214_e2079;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_215_e2085,) = {
    if (noise_variable_476 == 0.0) {
        let noise_metadata_schedule_215_e2083: f64 = (params.p138).exp();
        (noise_metadata_schedule_215_e2083,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_215_e2085;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_216_e2098,) = {
    if (noise_variable_476 == 0.0) {
        let noise_metadata_schedule_216_e2092: f64 = (noise_variable_237 * noise_variable_8);
        let noise_metadata_schedule_216_e2094: f64 = (noise_metadata_schedule_216_e2092 - params.p138);
        let noise_metadata_schedule_216_e2095: f64 = (1.0 + noise_metadata_schedule_216_e2094);
        let noise_metadata_schedule_216_e2096: f64 = (noise_variable_281 * noise_metadata_schedule_216_e2095);
        (noise_metadata_schedule_216_e2096,)
    } else {
        (noise_variable_251,)
    }
};
            noise_variable_251 = noise_metadata_schedule_216_e2098;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_217_e2101: f64 = (noise_variable_238 * noise_variable_8);
            let noise_metadata_schedule_217_e2103: f64 = (noise_metadata_schedule_217_e2101 / noise_variable_48);
            let noise_metadata_schedule_217_e2105: f64 = if noise_metadata_schedule_217_e2103 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_477 = noise_metadata_schedule_217_e2105;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_218_e2114,) = {
    if (noise_variable_477 != 0.0) {
        let noise_metadata_schedule_218_e2109: f64 = (noise_variable_238 * noise_variable_8);
        let noise_metadata_schedule_218_e2111: f64 = (noise_metadata_schedule_218_e2109 / noise_variable_48);
        let noise_metadata_schedule_218_e2112: f64 = (noise_metadata_schedule_218_e2111).exp();
        (noise_metadata_schedule_218_e2112,)
    } else {
        (noise_variable_252,)
    }
};
            noise_variable_252 = noise_metadata_schedule_218_e2114;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_219_e2120,) = {
    if (noise_variable_477 == 0.0) {
        let noise_metadata_schedule_219_e2118: f64 = (params.p138).exp();
        (noise_metadata_schedule_219_e2118,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_219_e2120;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_220_e2135,) = {
    if (noise_variable_477 == 0.0) {
        let noise_metadata_schedule_220_e2127: f64 = (noise_variable_238 * noise_variable_8);
        let noise_metadata_schedule_220_e2129: f64 = (noise_metadata_schedule_220_e2127 / noise_variable_48);
        let noise_metadata_schedule_220_e2131: f64 = (noise_metadata_schedule_220_e2129 - params.p138);
        let noise_metadata_schedule_220_e2132: f64 = (1.0 + noise_metadata_schedule_220_e2131);
        let noise_metadata_schedule_220_e2133: f64 = (noise_variable_281 * noise_metadata_schedule_220_e2132);
        (noise_metadata_schedule_220_e2133,)
    } else {
        (noise_variable_252,)
    }
};
            noise_variable_252 = noise_metadata_schedule_220_e2135;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_221_e2138: f64 = (noise_variable_241 * noise_variable_8);
            let noise_metadata_schedule_221_e2140: f64 = if noise_metadata_schedule_221_e2138 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_478 = noise_metadata_schedule_221_e2140;
        }
        if matches!(source_index, 11 | 12) {
            let (noise_metadata_schedule_222_e2147,) = {
    if (noise_variable_478 != 0.0) {
        let noise_metadata_schedule_222_e2144: f64 = (noise_variable_241 * noise_variable_8);
        let noise_metadata_schedule_222_e2145: f64 = (noise_metadata_schedule_222_e2144).exp();
        (noise_metadata_schedule_222_e2145,)
    } else {
        (noise_variable_254,)
    }
};
            noise_variable_254 = noise_metadata_schedule_222_e2147;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_223_e2153,) = {
    if (noise_variable_478 == 0.0) {
        let noise_metadata_schedule_223_e2151: f64 = (params.p138).exp();
        (noise_metadata_schedule_223_e2151,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_223_e2153;
        }
        if matches!(source_index, 11 | 12) {
            let (noise_metadata_schedule_224_e2166,) = {
    if (noise_variable_478 == 0.0) {
        let noise_metadata_schedule_224_e2160: f64 = (noise_variable_241 * noise_variable_8);
        let noise_metadata_schedule_224_e2162: f64 = (noise_metadata_schedule_224_e2160 - params.p138);
        let noise_metadata_schedule_224_e2163: f64 = (1.0 + noise_metadata_schedule_224_e2162);
        let noise_metadata_schedule_224_e2164: f64 = (noise_variable_281 * noise_metadata_schedule_224_e2163);
        (noise_metadata_schedule_224_e2164,)
    } else {
        (noise_variable_254,)
    }
};
            noise_variable_254 = noise_metadata_schedule_224_e2166;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_225_e2169: f64 = (noise_variable_240 * noise_variable_8);
            let noise_metadata_schedule_225_e2171: f64 = if noise_metadata_schedule_225_e2169 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_479 = noise_metadata_schedule_225_e2171;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_226_e2178,) = {
    if (noise_variable_479 != 0.0) {
        let noise_metadata_schedule_226_e2175: f64 = (noise_variable_240 * noise_variable_8);
        let noise_metadata_schedule_226_e2176: f64 = (noise_metadata_schedule_226_e2175).exp();
        (noise_metadata_schedule_226_e2176,)
    } else {
        (noise_variable_253,)
    }
};
            noise_variable_253 = noise_metadata_schedule_226_e2178;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_227_e2184,) = {
    if (noise_variable_479 == 0.0) {
        let noise_metadata_schedule_227_e2182: f64 = (params.p138).exp();
        (noise_metadata_schedule_227_e2182,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_227_e2184;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_228_e2197,) = {
    if (noise_variable_479 == 0.0) {
        let noise_metadata_schedule_228_e2191: f64 = (noise_variable_240 * noise_variable_8);
        let noise_metadata_schedule_228_e2193: f64 = (noise_metadata_schedule_228_e2191 - params.p138);
        let noise_metadata_schedule_228_e2194: f64 = (1.0 + noise_metadata_schedule_228_e2193);
        let noise_metadata_schedule_228_e2195: f64 = (noise_variable_281 * noise_metadata_schedule_228_e2194);
        (noise_metadata_schedule_228_e2195,)
    } else {
        (noise_variable_253,)
    }
};
            noise_variable_253 = noise_metadata_schedule_228_e2197;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_229_e2200: f64 = (noise_variable_247 * noise_variable_8);
            let noise_metadata_schedule_229_e2202: f64 = if noise_metadata_schedule_229_e2200 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_480 = noise_metadata_schedule_229_e2202;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_230_e2209,) = {
    if (noise_variable_480 != 0.0) {
        let noise_metadata_schedule_230_e2206: f64 = (noise_variable_247 * noise_variable_8);
        let noise_metadata_schedule_230_e2207: f64 = (noise_metadata_schedule_230_e2206).exp();
        (noise_metadata_schedule_230_e2207,)
    } else {
        (noise_variable_255,)
    }
};
            noise_variable_255 = noise_metadata_schedule_230_e2209;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_231_e2215,) = {
    if (noise_variable_480 == 0.0) {
        let noise_metadata_schedule_231_e2213: f64 = (params.p138).exp();
        (noise_metadata_schedule_231_e2213,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_231_e2215;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_232_e2228,) = {
    if (noise_variable_480 == 0.0) {
        let noise_metadata_schedule_232_e2222: f64 = (noise_variable_247 * noise_variable_8);
        let noise_metadata_schedule_232_e2224: f64 = (noise_metadata_schedule_232_e2222 - params.p138);
        let noise_metadata_schedule_232_e2225: f64 = (1.0 + noise_metadata_schedule_232_e2224);
        let noise_metadata_schedule_232_e2226: f64 = (noise_variable_281 * noise_metadata_schedule_232_e2225);
        (noise_metadata_schedule_232_e2226,)
    } else {
        (noise_variable_255,)
    }
};
            noise_variable_255 = noise_metadata_schedule_232_e2228;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_233_e2231: f64 = (noise_variable_247 - noise_variable_16);
            let noise_metadata_schedule_233_e2233: f64 = (noise_metadata_schedule_233_e2231 * noise_variable_8);
            let noise_metadata_schedule_233_e2235: f64 = if noise_metadata_schedule_233_e2233 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_481 = noise_metadata_schedule_233_e2235;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_235_e2250,) = {
    if (noise_variable_481 == 0.0) {
        let noise_metadata_schedule_235_e2248: f64 = (params.p138).exp();
        (noise_metadata_schedule_235_e2248,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_235_e2250;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_237_e2268: f64 = (noise_variable_241 - noise_variable_16);
            let noise_metadata_schedule_237_e2270: f64 = (noise_metadata_schedule_237_e2268 * noise_variable_8);
            let noise_metadata_schedule_237_e2272: f64 = if noise_metadata_schedule_237_e2270 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_482 = noise_metadata_schedule_237_e2272;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_239_e2287,) = {
    if (noise_variable_482 == 0.0) {
        let noise_metadata_schedule_239_e2285: f64 = (params.p138).exp();
        (noise_metadata_schedule_239_e2285,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_239_e2287;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_241_e2305: f64 = (noise_variable_237 - noise_variable_16);
            let noise_metadata_schedule_241_e2307: f64 = (noise_metadata_schedule_241_e2305 * noise_variable_8);
            let noise_metadata_schedule_241_e2309: f64 = if noise_metadata_schedule_241_e2307 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_483 = noise_metadata_schedule_241_e2309;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_242_e2318,) = {
    if (noise_variable_483 != 0.0) {
        let noise_metadata_schedule_242_e2313: f64 = (noise_variable_237 - noise_variable_16);
        let noise_metadata_schedule_242_e2315: f64 = (noise_metadata_schedule_242_e2313 * noise_variable_8);
        let noise_metadata_schedule_242_e2316: f64 = (noise_metadata_schedule_242_e2315).exp();
        (noise_metadata_schedule_242_e2316,)
    } else {
        (noise_variable_257,)
    }
};
            noise_variable_257 = noise_metadata_schedule_242_e2318;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_243_e2324,) = {
    if (noise_variable_483 == 0.0) {
        let noise_metadata_schedule_243_e2322: f64 = (params.p138).exp();
        (noise_metadata_schedule_243_e2322,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_243_e2324;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_244_e2339,) = {
    if (noise_variable_483 == 0.0) {
        let noise_metadata_schedule_244_e2331: f64 = (noise_variable_237 - noise_variable_16);
        let noise_metadata_schedule_244_e2333: f64 = (noise_metadata_schedule_244_e2331 * noise_variable_8);
        let noise_metadata_schedule_244_e2335: f64 = (noise_metadata_schedule_244_e2333 - params.p138);
        let noise_metadata_schedule_244_e2336: f64 = (1.0 + noise_metadata_schedule_244_e2335);
        let noise_metadata_schedule_244_e2337: f64 = (noise_variable_281 * noise_metadata_schedule_244_e2336);
        (noise_metadata_schedule_244_e2337,)
    } else {
        (noise_variable_257,)
    }
};
            noise_variable_257 = noise_metadata_schedule_244_e2339;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_245_e2342: f64 = (noise_variable_236 - noise_variable_16);
            let noise_metadata_schedule_245_e2344: f64 = (noise_metadata_schedule_245_e2342 * noise_variable_8);
            let noise_metadata_schedule_245_e2346: f64 = if noise_metadata_schedule_245_e2344 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_484 = noise_metadata_schedule_245_e2346;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_246_e2355,) = {
    if (noise_variable_484 != 0.0) {
        let noise_metadata_schedule_246_e2350: f64 = (noise_variable_236 - noise_variable_16);
        let noise_metadata_schedule_246_e2352: f64 = (noise_metadata_schedule_246_e2350 * noise_variable_8);
        let noise_metadata_schedule_246_e2353: f64 = (noise_metadata_schedule_246_e2352).exp();
        (noise_metadata_schedule_246_e2353,)
    } else {
        (noise_variable_259,)
    }
};
            noise_variable_259 = noise_metadata_schedule_246_e2355;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_247_e2361,) = {
    if (noise_variable_484 == 0.0) {
        let noise_metadata_schedule_247_e2359: f64 = (params.p138).exp();
        (noise_metadata_schedule_247_e2359,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_247_e2361;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_248_e2376,) = {
    if (noise_variable_484 == 0.0) {
        let noise_metadata_schedule_248_e2368: f64 = (noise_variable_236 - noise_variable_16);
        let noise_metadata_schedule_248_e2370: f64 = (noise_metadata_schedule_248_e2368 * noise_variable_8);
        let noise_metadata_schedule_248_e2372: f64 = (noise_metadata_schedule_248_e2370 - params.p138);
        let noise_metadata_schedule_248_e2373: f64 = (1.0 + noise_metadata_schedule_248_e2372);
        let noise_metadata_schedule_248_e2374: f64 = (noise_variable_281 * noise_metadata_schedule_248_e2373);
        (noise_metadata_schedule_248_e2374,)
    } else {
        (noise_variable_259,)
    }
};
            noise_variable_259 = noise_metadata_schedule_248_e2376;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_249_e2380: f64 = (4.0 * noise_variable_257);
            let noise_metadata_schedule_249_e2381: f64 = (1.0 + noise_metadata_schedule_249_e2380);
            let noise_metadata_schedule_249_e2382: f64 = (noise_metadata_schedule_249_e2381).sqrt();
            noise_variable_107 = noise_metadata_schedule_249_e2382;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_250_e2386: f64 = (4.0 * noise_variable_259);
            let noise_metadata_schedule_250_e2387: f64 = (1.0 + noise_metadata_schedule_250_e2386);
            let noise_metadata_schedule_250_e2388: f64 = (noise_metadata_schedule_250_e2387).sqrt();
            noise_variable_108 = noise_metadata_schedule_250_e2388;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_251_e2391: f64 = (2.0 * noise_variable_259);
            let noise_metadata_schedule_251_e2394: f64 = (1.0 + noise_variable_108);
            let noise_metadata_schedule_251_e2395: f64 = (noise_metadata_schedule_251_e2391 / noise_metadata_schedule_251_e2394);
            noise_variable_109 = noise_metadata_schedule_251_e2395;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_252_e2398: f64 = if noise_variable_109 < params.p140 { 1.0 } else { 0.0 };
            noise_variable_485 = noise_metadata_schedule_252_e2398;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_253_e2402,) = {
    if (noise_variable_485 != 0.0) {
        (params.p140,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_253_e2402;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_254_e2406: f64 = (noise_variable_107 - noise_variable_108);
            let noise_metadata_schedule_254_e2409: f64 = (noise_variable_107 + 1.0);
            let noise_metadata_schedule_254_e2412: f64 = (noise_variable_108 + 1.0);
            let noise_metadata_schedule_254_e2413: f64 = (noise_metadata_schedule_254_e2409 / noise_metadata_schedule_254_e2412);
            let noise_metadata_schedule_254_e2414: f64 = (noise_metadata_schedule_254_e2413).ln();
            let noise_metadata_schedule_254_e2415: f64 = (noise_metadata_schedule_254_e2406 - noise_metadata_schedule_254_e2414);
            let noise_metadata_schedule_254_e2416: f64 = (noise_variable_6 * noise_metadata_schedule_254_e2415);
            noise_variable_110 = noise_metadata_schedule_254_e2416;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_255_e2419: f64 = (noise_variable_110 + noise_variable_242);
            let noise_metadata_schedule_255_e2421: f64 = (noise_metadata_schedule_255_e2419 / noise_variable_31);
            noise_variable_111 = noise_metadata_schedule_255_e2421;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_256_e2424: f64 = if noise_variable_111 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_486 = noise_metadata_schedule_256_e2424;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_257_e2427: f64 = if noise_variable_236 < 100.0 { 1.0 } else { 0.0 };
            noise_variable_487 = noise_metadata_schedule_257_e2427;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_258_e2433,) = {
    if ((noise_variable_486 != 0.0) && (noise_variable_487 != 0.0)) {
        (noise_variable_236,)
    } else {
        (noise_variable_283,)
    }
};
            noise_variable_283 = noise_metadata_schedule_258_e2433;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_259_e2447,) = {
    if ((noise_variable_486 != 0.0) && (noise_variable_487 == 0.0)) {
        let noise_metadata_schedule_259_e2442: f64 = (noise_variable_236 - 100.0);
        let noise_metadata_schedule_259_e2443: f64 = (1.0 + noise_metadata_schedule_259_e2442);
        let noise_metadata_schedule_259_e2444: f64 = (noise_metadata_schedule_259_e2443).ln();
        let noise_metadata_schedule_259_e2445: f64 = (100.0 + noise_metadata_schedule_259_e2444);
        (noise_metadata_schedule_259_e2445,)
    } else {
        (noise_variable_283,)
    }
};
            noise_variable_283 = noise_metadata_schedule_259_e2447;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_260_e2468,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_260_e2452: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_260_e2455: f64 = (0.5 * noise_variable_111);
        let noise_metadata_schedule_260_e2457: f64 = (noise_metadata_schedule_260_e2455 * noise_variable_31);
        let noise_metadata_schedule_260_e2459: f64 = (noise_metadata_schedule_260_e2457 * noise_variable_8);
        let noise_metadata_schedule_260_e2461: f64 = (noise_metadata_schedule_260_e2459 + 1.0);
        let noise_metadata_schedule_260_e2462: f64 = (noise_metadata_schedule_260_e2461).ln();
        let noise_metadata_schedule_260_e2463: f64 = (noise_metadata_schedule_260_e2452 * noise_metadata_schedule_260_e2462);
        let noise_metadata_schedule_260_e2464: f64 = (noise_variable_16 + noise_metadata_schedule_260_e2463);
        let noise_metadata_schedule_260_e2466: f64 = (noise_metadata_schedule_260_e2464 - noise_variable_283);
        (noise_metadata_schedule_260_e2466,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_260_e2468;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_261_e2474,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_261_e2472: f64 = (0.2 * noise_variable_16);
        (noise_metadata_schedule_261_e2472,)
    } else {
        (noise_variable_278,)
    }
};
            noise_variable_278 = noise_metadata_schedule_261_e2474;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_262_e2480,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_262_e2478: f64 = (noise_variable_278 * noise_variable_278);
        (noise_metadata_schedule_262_e2478,)
    } else {
        (noise_variable_267,)
    }
};
            noise_variable_267 = noise_metadata_schedule_262_e2480;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_263_e2486,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_263_e2484: f64 = (noise_variable_112 * noise_variable_112);
        (noise_metadata_schedule_263_e2484,)
    } else {
        (noise_variable_268,)
    }
};
            noise_variable_268 = noise_metadata_schedule_263_e2486;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_264_e2489: f64 = if noise_variable_112 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_488 = noise_metadata_schedule_264_e2489;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_265_e2504,) = {
    if ((noise_variable_486 != 0.0) && (noise_variable_488 != 0.0)) {
        let noise_metadata_schedule_265_e2495: f64 = (0.5 * noise_variable_267);
        let noise_metadata_schedule_265_e2498: f64 = (noise_variable_268 + noise_variable_267);
        let noise_metadata_schedule_265_e2499: f64 = (noise_metadata_schedule_265_e2498).sqrt();
        let noise_metadata_schedule_265_e2501: f64 = (noise_metadata_schedule_265_e2499 - noise_variable_112);
        let noise_metadata_schedule_265_e2502: f64 = (noise_metadata_schedule_265_e2495 / noise_metadata_schedule_265_e2501);
        (noise_metadata_schedule_265_e2502,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_265_e2504;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_266_e2518,) = {
    if ((noise_variable_486 != 0.0) && (noise_variable_488 == 0.0)) {
        let noise_metadata_schedule_266_e2512: f64 = (noise_variable_268 + noise_variable_267);
        let noise_metadata_schedule_266_e2513: f64 = (noise_metadata_schedule_266_e2512).sqrt();
        let noise_metadata_schedule_266_e2515: f64 = (noise_metadata_schedule_266_e2513 + noise_variable_112);
        let noise_metadata_schedule_266_e2516: f64 = (0.5 * noise_metadata_schedule_266_e2515);
        (noise_metadata_schedule_266_e2516,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_266_e2518;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_267_e2536,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_267_e2524: f64 = (params.p61 * params.p60);
        let noise_metadata_schedule_267_e2525: f64 = (noise_variable_113 + noise_metadata_schedule_267_e2524);
        let noise_metadata_schedule_267_e2526: f64 = (noise_variable_113 * noise_metadata_schedule_267_e2525);
        let noise_metadata_schedule_267_e2531: f64 = (params.p61 * noise_variable_31);
        let noise_metadata_schedule_267_e2532: f64 = (noise_variable_113 + noise_metadata_schedule_267_e2531);
        let noise_metadata_schedule_267_e2533: f64 = (params.p60 * noise_metadata_schedule_267_e2532);
        let noise_metadata_schedule_267_e2534: f64 = (noise_metadata_schedule_267_e2526 / noise_metadata_schedule_267_e2533);
        (noise_metadata_schedule_267_e2534,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_267_e2536;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_268_e2542,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_268_e2540: f64 = (noise_variable_111 / noise_variable_114);
        (noise_metadata_schedule_268_e2540,)
    } else {
        (noise_variable_271,)
    }
};
            noise_variable_271 = noise_metadata_schedule_268_e2542;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_269_e2550,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_269_e2546: f64 = (noise_variable_271 - 1.0);
        let noise_metadata_schedule_269_e2548: f64 = (noise_metadata_schedule_269_e2546 / params.p62);
        (noise_metadata_schedule_269_e2548,)
    } else {
        (noise_variable_265,)
    }
};
            noise_variable_265 = noise_metadata_schedule_269_e2550;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_270_e2553: f64 = if noise_variable_271 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_489 = noise_metadata_schedule_270_e2553;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_271_e2567,) = {
    if ((noise_variable_486 != 0.0) && (noise_variable_489 != 0.0)) {
        let noise_metadata_schedule_271_e2561: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_271_e2562: f64 = (1.0 + noise_metadata_schedule_271_e2561);
        let noise_metadata_schedule_271_e2563: f64 = (noise_metadata_schedule_271_e2562).ln();
        let noise_metadata_schedule_271_e2564: f64 = (params.p62 * noise_metadata_schedule_271_e2563);
        let noise_metadata_schedule_271_e2565: f64 = (1.0 + noise_metadata_schedule_271_e2564);
        (noise_metadata_schedule_271_e2565,)
    } else {
        (noise_variable_269,)
    }
};
            noise_variable_269 = noise_metadata_schedule_271_e2567;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_272_e2583,) = {
    if ((noise_variable_486 != 0.0) && (noise_variable_489 == 0.0)) {
        let noise_metadata_schedule_272_e2576: f64 = (-noise_variable_265);
        let noise_metadata_schedule_272_e2577: f64 = (noise_metadata_schedule_272_e2576).exp();
        let noise_metadata_schedule_272_e2578: f64 = (1.0 + noise_metadata_schedule_272_e2577);
        let noise_metadata_schedule_272_e2579: f64 = (noise_metadata_schedule_272_e2578).ln();
        let noise_metadata_schedule_272_e2580: f64 = (params.p62 * noise_metadata_schedule_272_e2579);
        let noise_metadata_schedule_272_e2581: f64 = (noise_variable_271 + noise_metadata_schedule_272_e2580);
        (noise_metadata_schedule_272_e2581,)
    } else {
        (noise_variable_269,)
    }
};
            noise_variable_269 = noise_metadata_schedule_272_e2583;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_273_e2600,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_273_e2590: f64 = (-1.0);
        let noise_metadata_schedule_273_e2592: f64 = (noise_metadata_schedule_273_e2590 / params.p62);
        let noise_metadata_schedule_273_e2593: f64 = (noise_metadata_schedule_273_e2592).exp();
        let noise_metadata_schedule_273_e2594: f64 = (1.0 + noise_metadata_schedule_273_e2593);
        let noise_metadata_schedule_273_e2595: f64 = (noise_metadata_schedule_273_e2594).ln();
        let noise_metadata_schedule_273_e2596: f64 = (params.p62 * noise_metadata_schedule_273_e2595);
        let noise_metadata_schedule_273_e2597: f64 = (1.0 + noise_metadata_schedule_273_e2596);
        let noise_metadata_schedule_273_e2598: f64 = (noise_variable_269 / noise_metadata_schedule_273_e2597);
        (noise_metadata_schedule_273_e2598,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_273_e2600;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_274_e2608,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_274_e2605: f64 = (params.p61 * params.p60);
        let noise_metadata_schedule_274_e2606: f64 = (noise_variable_113 / noise_metadata_schedule_274_e2605);
        (noise_metadata_schedule_274_e2606,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_274_e2608;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_275_e2633,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_275_e2614: f64 = (4.0 * noise_variable_115);
        let noise_metadata_schedule_275_e2616: f64 = (noise_metadata_schedule_275_e2614 * noise_variable_116);
        let noise_metadata_schedule_275_e2619: f64 = (1.0 + noise_variable_116);
        let noise_metadata_schedule_275_e2620: f64 = (noise_metadata_schedule_275_e2616 * noise_metadata_schedule_275_e2619);
        let noise_metadata_schedule_275_e2621: f64 = (1.0 + noise_metadata_schedule_275_e2620);
        let noise_metadata_schedule_275_e2622: f64 = (noise_metadata_schedule_275_e2621).sqrt();
        let noise_metadata_schedule_275_e2623: f64 = (1.0 + noise_metadata_schedule_275_e2622);
        let noise_metadata_schedule_275_e2626: f64 = (2.0 * noise_variable_115);
        let noise_metadata_schedule_275_e2629: f64 = (1.0 + noise_variable_116);
        let noise_metadata_schedule_275_e2630: f64 = (noise_metadata_schedule_275_e2626 * noise_metadata_schedule_275_e2629);
        let noise_metadata_schedule_275_e2631: f64 = (noise_metadata_schedule_275_e2623 / noise_metadata_schedule_275_e2630);
        (noise_metadata_schedule_275_e2631,)
    } else {
        (noise_variable_117,)
    }
};
            noise_variable_117 = noise_metadata_schedule_275_e2633;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_276_e2649,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_276_e2637: f64 = (1.0 - noise_variable_117);
        let noise_metadata_schedule_276_e2640: f64 = (noise_variable_109 * noise_variable_117);
        let noise_metadata_schedule_276_e2641: f64 = (noise_metadata_schedule_276_e2637 + noise_metadata_schedule_276_e2640);
        let noise_metadata_schedule_276_e2645: f64 = (noise_variable_109 * noise_variable_117);
        let noise_metadata_schedule_276_e2646: f64 = (1.0 + noise_metadata_schedule_276_e2645);
        let noise_metadata_schedule_276_e2647: f64 = (noise_metadata_schedule_276_e2641 / noise_metadata_schedule_276_e2646);
        (noise_metadata_schedule_276_e2647,)
    } else {
        (noise_variable_118,)
    }
};
            noise_variable_118 = noise_metadata_schedule_276_e2649;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_277_e2661,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_277_e2653: f64 = (0.5 * noise_variable_111);
        let noise_metadata_schedule_277_e2655: f64 = (noise_metadata_schedule_277_e2653 * noise_variable_31);
        let noise_metadata_schedule_277_e2657: f64 = (noise_metadata_schedule_277_e2655 * noise_variable_118);
        let noise_metadata_schedule_277_e2659: f64 = (noise_metadata_schedule_277_e2657 * noise_variable_8);
        (noise_metadata_schedule_277_e2659,)
    } else {
        (noise_variable_120,)
    }
};
            noise_variable_120 = noise_metadata_schedule_277_e2661;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_278_e2675,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_278_e2665: f64 = (2.0 * noise_variable_120);
        let noise_metadata_schedule_278_e2669: f64 = (noise_variable_109 + noise_variable_120);
        let noise_metadata_schedule_278_e2671: f64 = (noise_metadata_schedule_278_e2669 + 1.0);
        let noise_metadata_schedule_278_e2672: f64 = (noise_variable_109 * noise_metadata_schedule_278_e2671);
        let noise_metadata_schedule_278_e2673: f64 = (noise_metadata_schedule_278_e2665 + noise_metadata_schedule_278_e2672);
        (noise_metadata_schedule_278_e2673,)
    } else {
        (noise_variable_272,)
    }
};
            noise_variable_272 = noise_metadata_schedule_278_e2675;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_279_e2683,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_279_e2680: f64 = (noise_variable_120 - 1.0);
        let noise_metadata_schedule_279_e2681: f64 = (0.5 * noise_metadata_schedule_279_e2680);
        (noise_metadata_schedule_279_e2681,)
    } else {
        (noise_variable_121,)
    }
};
            noise_variable_121 = noise_metadata_schedule_279_e2683;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_280_e2691,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_280_e2687: f64 = (noise_variable_121 * noise_variable_121);
        let noise_metadata_schedule_280_e2689: f64 = (noise_metadata_schedule_280_e2687 + noise_variable_272);
        (noise_metadata_schedule_280_e2689,)
    } else {
        (noise_variable_266,)
    }
};
            noise_variable_266 = noise_metadata_schedule_280_e2691;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_281_e2694: f64 = if noise_variable_120 >= 1.0 { 1.0 } else { 0.0 };
            noise_variable_490 = noise_metadata_schedule_281_e2694;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_282_e2703,) = {
    if ((noise_variable_486 != 0.0) && (noise_variable_490 != 0.0)) {
        let noise_metadata_schedule_282_e2700: f64 = (noise_variable_266).sqrt();
        let noise_metadata_schedule_282_e2701: f64 = (noise_variable_121 + noise_metadata_schedule_282_e2700);
        (noise_metadata_schedule_282_e2701,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_282_e2703;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_283_e2715,) = {
    if ((noise_variable_486 != 0.0) && (noise_variable_490 == 0.0)) {
        let noise_metadata_schedule_283_e2710: f64 = (noise_variable_266).sqrt();
        let noise_metadata_schedule_283_e2712: f64 = (noise_metadata_schedule_283_e2710 - noise_variable_121);
        let noise_metadata_schedule_283_e2713: f64 = (noise_variable_272 / noise_metadata_schedule_283_e2712);
        (noise_metadata_schedule_283_e2713,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_283_e2715;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_284_e2718: f64 = if noise_variable_122 < params.p139 { 1.0 } else { 0.0 };
            noise_variable_491 = noise_metadata_schedule_284_e2718;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_285_e2724,) = {
    if ((noise_variable_486 != 0.0) && (noise_variable_491 != 0.0)) {
        (params.p139,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_285_e2724;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_286_e2737,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_286_e2729: f64 = (noise_variable_122 + 1.0);
        let noise_metadata_schedule_286_e2730: f64 = (noise_variable_122 * noise_metadata_schedule_286_e2729);
        let noise_metadata_schedule_286_e2733: f64 = (noise_variable_16 * noise_variable_8);
        let noise_metadata_schedule_286_e2734: f64 = (noise_metadata_schedule_286_e2733).exp();
        let noise_metadata_schedule_286_e2735: f64 = (noise_metadata_schedule_286_e2730 * noise_metadata_schedule_286_e2734);
        (noise_metadata_schedule_286_e2735,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_286_e2737;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_287_e2747,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_287_e2741: f64 = (0.5 * params.p60);
        let noise_metadata_schedule_287_e2744: f64 = (noise_variable_111 - params.p61);
        let noise_metadata_schedule_287_e2745: f64 = (noise_metadata_schedule_287_e2741 * noise_metadata_schedule_287_e2744);
        (noise_metadata_schedule_287_e2745,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_287_e2747;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_288_e2757,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_288_e2751: f64 = (params.p60 * noise_variable_31);
        let noise_metadata_schedule_288_e2753: f64 = (noise_metadata_schedule_288_e2751 * params.p61);
        let noise_metadata_schedule_288_e2755: f64 = (noise_metadata_schedule_288_e2753 * noise_variable_111);
        (noise_metadata_schedule_288_e2755,)
    } else {
        (noise_variable_127,)
    }
};
            noise_variable_127 = noise_metadata_schedule_288_e2757;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_289_e2768,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_289_e2762: f64 = (noise_variable_126 * noise_variable_126);
        let noise_metadata_schedule_289_e2764: f64 = (noise_metadata_schedule_289_e2762 + noise_variable_127);
        let noise_metadata_schedule_289_e2765: f64 = (noise_metadata_schedule_289_e2764).sqrt();
        let noise_metadata_schedule_289_e2766: f64 = (noise_variable_126 + noise_metadata_schedule_289_e2765);
        (noise_metadata_schedule_289_e2766,)
    } else {
        (noise_variable_128,)
    }
};
            noise_variable_128 = noise_metadata_schedule_289_e2768;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_290_e2771: f64 = if params.p72 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_492 = noise_metadata_schedule_290_e2771;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_291_e2779,) = {
    if ((noise_variable_486 != 0.0) && (noise_variable_492 != 0.0)) {
        let noise_metadata_schedule_291_e2777: f64 = (noise_variable_17 * 0.1);
        (noise_metadata_schedule_291_e2777,)
    } else {
        (noise_variable_129,)
    }
};
            noise_variable_129 = noise_metadata_schedule_291_e2779;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_292_e2796,) = {
    if ((noise_variable_486 != 0.0) && (noise_variable_492 == 0.0)) {
        let noise_metadata_schedule_292_e2788: f64 = (2.0 * noise_variable_111);
        let noise_metadata_schedule_292_e2791: f64 = (noise_variable_111 + noise_variable_114);
        let noise_metadata_schedule_292_e2792: f64 = (noise_metadata_schedule_292_e2788 / noise_metadata_schedule_292_e2791);
        let noise_metadata_schedule_292_e2793: f64 = (0.1 + noise_metadata_schedule_292_e2792);
        let noise_metadata_schedule_292_e2794: f64 = (noise_variable_17 * noise_metadata_schedule_292_e2793);
        (noise_metadata_schedule_292_e2794,)
    } else {
        (noise_variable_129,)
    }
};
            noise_variable_129 = noise_metadata_schedule_292_e2796;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_293_e2806,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_293_e2800: f64 = (params.p61 * noise_variable_111);
        let noise_metadata_schedule_293_e2803: f64 = (params.p61 + noise_variable_111);
        let noise_metadata_schedule_293_e2804: f64 = (noise_metadata_schedule_293_e2800 / noise_metadata_schedule_293_e2803);
        (noise_metadata_schedule_293_e2804,)
    } else {
        (noise_variable_130,)
    }
};
            noise_variable_130 = noise_metadata_schedule_293_e2806;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_294_e2814,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_294_e2811: f64 = (params.p61 + noise_variable_111);
        let noise_metadata_schedule_294_e2812: f64 = (params.p61 / noise_metadata_schedule_294_e2811);
        (noise_metadata_schedule_294_e2812,)
    } else {
        (noise_variable_202,)
    }
};
            noise_variable_202 = noise_metadata_schedule_294_e2814;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_296_e2830,) = {
    if (noise_variable_486 == 0.0) {
        let noise_metadata_schedule_296_e2824: f64 = (2.0 * noise_variable_257);
        let noise_metadata_schedule_296_e2827: f64 = (1.0 + noise_variable_107);
        let noise_metadata_schedule_296_e2828: f64 = (noise_metadata_schedule_296_e2824 / noise_metadata_schedule_296_e2827);
        (noise_metadata_schedule_296_e2828,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_296_e2830;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_297_e2835,) = {
    if (noise_variable_486 == 0.0) {
        (noise_variable_251,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_297_e2835;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_298_e2837: f64 = (noise_variable_242).abs();
            let noise_metadata_schedule_298_e2840: f64 = (1e-5 * noise_variable_6);
            let noise_metadata_schedule_298_e2843: f64 = (noise_variable_110).abs();
            let noise_metadata_schedule_298_e2846: f64 = (1e-40 * noise_variable_6);
            let noise_metadata_schedule_298_e2849: f64 = (noise_variable_107 + noise_variable_108);
            let noise_metadata_schedule_298_e2850: f64 = (noise_metadata_schedule_298_e2846 * noise_metadata_schedule_298_e2849);
            let noise_metadata_schedule_298_e2852: f64 = if ((noise_metadata_schedule_298_e2837 < noise_metadata_schedule_298_e2840) || (noise_metadata_schedule_298_e2843 < noise_metadata_schedule_298_e2850)) { 1.0 } else { 0.0 };
            noise_variable_493 = noise_metadata_schedule_298_e2852;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_299_e2863,) = {
    if ((noise_variable_486 == 0.0) && (noise_variable_493 != 0.0)) {
        let noise_metadata_schedule_299_e2860: f64 = (noise_variable_122 + noise_variable_109);
        let noise_metadata_schedule_299_e2861: f64 = (0.5 * noise_metadata_schedule_299_e2860);
        (noise_metadata_schedule_299_e2861,)
    } else {
        (noise_variable_131,)
    }
};
            noise_variable_131 = noise_metadata_schedule_299_e2863;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_300_e2874,) = {
    if ((noise_variable_486 == 0.0) && (noise_variable_493 != 0.0)) {
        let noise_metadata_schedule_300_e2871: f64 = (noise_variable_131 + 1.0);
        let noise_metadata_schedule_300_e2872: f64 = (noise_variable_131 / noise_metadata_schedule_300_e2871);
        (noise_metadata_schedule_300_e2872,)
    } else {
        (noise_variable_118,)
    }
};
            noise_variable_118 = noise_metadata_schedule_300_e2874;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_301_e2888,) = {
    if ((noise_variable_486 == 0.0) && (noise_variable_493 == 0.0)) {
        let noise_metadata_schedule_301_e2883: f64 = (noise_variable_110 + noise_variable_237);
        let noise_metadata_schedule_301_e2885: f64 = (noise_metadata_schedule_301_e2883 - noise_variable_236);
        let noise_metadata_schedule_301_e2886: f64 = (noise_variable_110 / noise_metadata_schedule_301_e2885);
        (noise_metadata_schedule_301_e2886,)
    } else {
        (noise_variable_118,)
    }
};
            noise_variable_118 = noise_metadata_schedule_301_e2888;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_302_e2893,) = {
    if (noise_variable_486 == 0.0) {
        (noise_variable_242,)
    } else {
        (noise_variable_128,)
    }
};
            noise_variable_128 = noise_metadata_schedule_302_e2893;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_303_e2900,) = {
    if (noise_variable_486 == 0.0) {
        let noise_metadata_schedule_303_e2898: f64 = (0.1 * noise_variable_17);
        (noise_metadata_schedule_303_e2898,)
    } else {
        (noise_variable_129,)
    }
};
            noise_variable_129 = noise_metadata_schedule_303_e2900;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_304_e2905,) = {
    if (noise_variable_486 == 0.0) {
        (noise_variable_111,)
    } else {
        (noise_variable_130,)
    }
};
            noise_variable_130 = noise_metadata_schedule_304_e2905;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_305_e2914,) = {
    if (noise_variable_486 == 0.0) {
        let noise_metadata_schedule_305_e2911: f64 = (noise_variable_130 / params.p61);
        let noise_metadata_schedule_305_e2912: f64 = (1.0 - noise_metadata_schedule_305_e2911);
        (noise_metadata_schedule_305_e2912,)
    } else {
        (noise_variable_202,)
    }
};
            noise_variable_202 = noise_metadata_schedule_305_e2914;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_306_e2919: f64 = (-1.0);
            let noise_metadata_schedule_306_e2921: f64 = (noise_metadata_schedule_306_e2919 / params.p66);
            let noise_metadata_schedule_306_e2922: f64 = (3.0_f64).powf(noise_metadata_schedule_306_e2921);
            let noise_metadata_schedule_306_e2923: f64 = (1.0 - noise_metadata_schedule_306_e2922);
            let noise_metadata_schedule_306_e2924: f64 = (noise_variable_14 * noise_metadata_schedule_306_e2923);
            noise_variable_132 = noise_metadata_schedule_306_e2924;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_307_e2927: f64 = (0.1 * noise_variable_14);
            noise_variable_279 = noise_metadata_schedule_307_e2927;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_308_e2930: f64 = (noise_variable_238 - noise_variable_132);
            let noise_metadata_schedule_308_e2932: f64 = (noise_metadata_schedule_308_e2930 / noise_variable_279);
            noise_variable_265 = noise_metadata_schedule_308_e2932;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_309_e2935: f64 = if noise_variable_238 < noise_variable_132 { 1.0 } else { 0.0 };
            noise_variable_494 = noise_metadata_schedule_309_e2935;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_310_e2947,) = {
    if (noise_variable_494 != 0.0) {
        let noise_metadata_schedule_310_e2941: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_310_e2942: f64 = (1.0 + noise_metadata_schedule_310_e2941);
        let noise_metadata_schedule_310_e2943: f64 = (noise_metadata_schedule_310_e2942).ln();
        let noise_metadata_schedule_310_e2944: f64 = (noise_variable_279 * noise_metadata_schedule_310_e2943);
        let noise_metadata_schedule_310_e2945: f64 = (noise_variable_238 - noise_metadata_schedule_310_e2944);
        (noise_metadata_schedule_310_e2945,)
    } else {
        (noise_variable_133,)
    }
};
            noise_variable_133 = noise_metadata_schedule_310_e2947;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_311_e2961,) = {
    if (noise_variable_494 == 0.0) {
        let noise_metadata_schedule_311_e2954: f64 = (-noise_variable_265);
        let noise_metadata_schedule_311_e2955: f64 = (noise_metadata_schedule_311_e2954).exp();
        let noise_metadata_schedule_311_e2956: f64 = (1.0 + noise_metadata_schedule_311_e2955);
        let noise_metadata_schedule_311_e2957: f64 = (noise_metadata_schedule_311_e2956).ln();
        let noise_metadata_schedule_311_e2958: f64 = (noise_variable_279 * noise_metadata_schedule_311_e2957);
        let noise_metadata_schedule_311_e2959: f64 = (noise_variable_132 - noise_metadata_schedule_311_e2958);
        (noise_metadata_schedule_311_e2959,)
    } else {
        (noise_variable_133,)
    }
};
            noise_variable_133 = noise_metadata_schedule_311_e2961;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_312_e2965: f64 = (noise_variable_133 * noise_variable_65);
            let noise_metadata_schedule_312_e2966: f64 = (1.0 - noise_metadata_schedule_312_e2965);
            let noise_metadata_schedule_312_e2969: f64 = (1.0 - params.p66);
            let noise_metadata_schedule_312_e2970: f64 = (noise_metadata_schedule_312_e2966).powf(noise_metadata_schedule_312_e2969);
            noise_variable_59 = noise_metadata_schedule_312_e2970;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_313_e2974: f64 = (1.0 - params.p66);
            let noise_metadata_schedule_313_e2975: f64 = (noise_variable_14 / noise_metadata_schedule_313_e2974);
            let noise_metadata_schedule_313_e2978: f64 = (1.0 - noise_variable_59);
            let noise_metadata_schedule_313_e2979: f64 = (noise_metadata_schedule_313_e2975 * noise_metadata_schedule_313_e2978);
            let noise_metadata_schedule_313_e2983: f64 = (noise_variable_238 - noise_variable_133);
            let noise_metadata_schedule_313_e2984: f64 = (3.0 * noise_metadata_schedule_313_e2983);
            let noise_metadata_schedule_313_e2985: f64 = (noise_metadata_schedule_313_e2979 + noise_metadata_schedule_313_e2984);
            noise_variable_134 = noise_metadata_schedule_313_e2985;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_314_e2988: f64 = if params.p73 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_495 = noise_metadata_schedule_314_e2988;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_315_e2992,) = {
    if (noise_variable_495 != 0.0) {
        (noise_variable_236,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_315_e2992;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_316_e2995: f64 = if params.p73 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_496 = noise_metadata_schedule_316_e2995;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_317_e3004,) = {
    if ((noise_variable_495 == 0.0) && (noise_variable_496 != 0.0)) {
        let noise_metadata_schedule_317_e3002: f64 = (noise_variable_236 + noise_variable_128);
        (noise_metadata_schedule_317_e3002,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_317_e3004;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_318_e3012,) = {
    if ((noise_variable_495 == 0.0) && (noise_variable_496 == 0.0)) {
        (noise_variable_237,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_318_e3012;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_319_e3015: f64 = (2.0 - noise_variable_25);
            let noise_metadata_schedule_319_e3018: f64 = (1.0 - noise_variable_25);
            let noise_metadata_schedule_319_e3019: f64 = (noise_metadata_schedule_319_e3015 / noise_metadata_schedule_319_e3018);
            noise_variable_136 = noise_metadata_schedule_319_e3019;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_320_e3024: f64 = (-1.0);
            let noise_metadata_schedule_320_e3026: f64 = (noise_metadata_schedule_320_e3024 / params.p71);
            let noise_metadata_schedule_320_e3027: f64 = (noise_variable_136).powf(noise_metadata_schedule_320_e3026);
            let noise_metadata_schedule_320_e3028: f64 = (1.0 - noise_metadata_schedule_320_e3027);
            let noise_metadata_schedule_320_e3029: f64 = (noise_variable_17 * noise_metadata_schedule_320_e3028);
            noise_variable_137 = noise_metadata_schedule_320_e3029;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_321_e3032: f64 = (noise_variable_135 - noise_variable_137);
            let noise_metadata_schedule_321_e3034: f64 = (noise_metadata_schedule_321_e3032 / noise_variable_129);
            noise_variable_265 = noise_metadata_schedule_321_e3034;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_322_e3037: f64 = if noise_variable_135 < noise_variable_137 { 1.0 } else { 0.0 };
            noise_variable_497 = noise_metadata_schedule_322_e3037;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_323_e3049,) = {
    if (noise_variable_497 != 0.0) {
        let noise_metadata_schedule_323_e3043: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_323_e3044: f64 = (1.0 + noise_metadata_schedule_323_e3043);
        let noise_metadata_schedule_323_e3045: f64 = (noise_metadata_schedule_323_e3044).ln();
        let noise_metadata_schedule_323_e3046: f64 = (noise_variable_129 * noise_metadata_schedule_323_e3045);
        let noise_metadata_schedule_323_e3047: f64 = (noise_variable_135 - noise_metadata_schedule_323_e3046);
        (noise_metadata_schedule_323_e3047,)
    } else {
        (noise_variable_138,)
    }
};
            noise_variable_138 = noise_metadata_schedule_323_e3049;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_324_e3063,) = {
    if (noise_variable_497 == 0.0) {
        let noise_metadata_schedule_324_e3056: f64 = (-noise_variable_265);
        let noise_metadata_schedule_324_e3057: f64 = (noise_metadata_schedule_324_e3056).exp();
        let noise_metadata_schedule_324_e3058: f64 = (1.0 + noise_metadata_schedule_324_e3057);
        let noise_metadata_schedule_324_e3059: f64 = (noise_metadata_schedule_324_e3058).ln();
        let noise_metadata_schedule_324_e3060: f64 = (noise_variable_129 * noise_metadata_schedule_324_e3059);
        let noise_metadata_schedule_324_e3061: f64 = (noise_variable_137 - noise_metadata_schedule_324_e3060);
        (noise_metadata_schedule_324_e3061,)
    } else {
        (noise_variable_138,)
    }
};
            noise_variable_138 = noise_metadata_schedule_324_e3063;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_325_e3066: f64 = (noise_variable_202).powf(params.p75);
            noise_variable_139 = noise_metadata_schedule_325_e3066;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_326_e3070: f64 = (1.0 - params.p71);
            let noise_metadata_schedule_326_e3071: f64 = (noise_variable_17 / noise_metadata_schedule_326_e3070);
            let noise_metadata_schedule_326_e3077: f64 = (noise_variable_138 / noise_variable_17);
            let noise_metadata_schedule_326_e3078: f64 = (1.0 - noise_metadata_schedule_326_e3077);
            let noise_metadata_schedule_326_e3081: f64 = (1.0 - params.p71);
            let noise_metadata_schedule_326_e3082: f64 = (noise_metadata_schedule_326_e3078).powf(noise_metadata_schedule_326_e3081);
            let noise_metadata_schedule_326_e3083: f64 = (noise_variable_139 * noise_metadata_schedule_326_e3082);
            let noise_metadata_schedule_326_e3084: f64 = (1.0 - noise_metadata_schedule_326_e3083);
            let noise_metadata_schedule_326_e3085: f64 = (noise_metadata_schedule_326_e3071 * noise_metadata_schedule_326_e3084);
            let noise_metadata_schedule_326_e3088: f64 = (noise_variable_139 * noise_variable_136);
            let noise_metadata_schedule_326_e3091: f64 = (noise_variable_135 - noise_variable_138);
            let noise_metadata_schedule_326_e3092: f64 = (noise_metadata_schedule_326_e3088 * noise_metadata_schedule_326_e3091);
            let noise_metadata_schedule_326_e3093: f64 = (noise_metadata_schedule_326_e3085 + noise_metadata_schedule_326_e3092);
            noise_variable_140 = noise_metadata_schedule_326_e3093;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_327_e3096: f64 = (1.0 - noise_variable_25);
            let noise_metadata_schedule_327_e3098: f64 = (noise_metadata_schedule_327_e3096 * noise_variable_140);
            let noise_metadata_schedule_327_e3101: f64 = (noise_variable_25 * noise_variable_236);
            let noise_metadata_schedule_327_e3102: f64 = (noise_metadata_schedule_327_e3098 + noise_metadata_schedule_327_e3101);
            noise_variable_141 = noise_metadata_schedule_327_e3102;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_328_e3105: f64 = (4.0 * noise_variable_35);
            let noise_metadata_schedule_328_e3107: f64 = (noise_metadata_schedule_328_e3105 / noise_variable_36);
            noise_variable_142 = noise_metadata_schedule_328_e3107;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_329_e3110: f64 = (noise_variable_142 * noise_variable_252);
            noise_variable_143 = noise_metadata_schedule_329_e3110;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_330_e3115: f64 = (1.0 + noise_variable_143);
            let noise_metadata_schedule_330_e3116: f64 = (noise_metadata_schedule_330_e3115).sqrt();
            let noise_metadata_schedule_330_e3117: f64 = (1.0 + noise_metadata_schedule_330_e3116);
            let noise_metadata_schedule_330_e3118: f64 = (noise_variable_143 / noise_metadata_schedule_330_e3117);
            noise_variable_145 = noise_metadata_schedule_330_e3118;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_331_e3122: f64 = (1.0 / noise_variable_49);
            let noise_metadata_schedule_331_e3123: f64 = (noise_variable_124).powf(noise_metadata_schedule_331_e3122);
            noise_variable_125 = noise_metadata_schedule_331_e3123;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_332_e3126: f64 = (noise_variable_142 * noise_variable_125);
            noise_variable_144 = noise_metadata_schedule_332_e3126;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_333_e3131: f64 = (1.0 + noise_variable_144);
            let noise_metadata_schedule_333_e3132: f64 = (noise_metadata_schedule_333_e3131).sqrt();
            let noise_metadata_schedule_333_e3133: f64 = (1.0 + noise_metadata_schedule_333_e3132);
            let noise_metadata_schedule_333_e3134: f64 = (noise_variable_144 / noise_metadata_schedule_333_e3133);
            noise_variable_146 = noise_metadata_schedule_333_e3134;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_334_e3137: f64 = if params.p91 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_498 = noise_metadata_schedule_334_e3137;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_335_e3149,) = {
    if (noise_variable_498 != 0.0) {
        let noise_metadata_schedule_335_e3142: f64 = (noise_variable_134 / noise_variable_41);
        let noise_metadata_schedule_335_e3143: f64 = (1.0 + noise_metadata_schedule_335_e3142);
        let noise_metadata_schedule_335_e3146: f64 = (noise_variable_141 / noise_variable_40);
        let noise_metadata_schedule_335_e3147: f64 = (noise_metadata_schedule_335_e3143 + noise_metadata_schedule_335_e3146);
        (noise_metadata_schedule_335_e3147,)
    } else {
        (noise_variable_147,)
    }
};
            noise_variable_147 = noise_metadata_schedule_335_e3149;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_336_e3162,) = {
    if (noise_variable_498 == 0.0) {
        let noise_metadata_schedule_336_e3154: f64 = (noise_variable_134 / noise_variable_41);
        let noise_metadata_schedule_336_e3156: f64 = (noise_metadata_schedule_336_e3154 + 1.0);
        let noise_metadata_schedule_336_e3158: f64 = (noise_metadata_schedule_336_e3156 * noise_variable_100);
        let noise_metadata_schedule_336_e3160: f64 = (noise_metadata_schedule_336_e3158 * noise_variable_8);
        (noise_metadata_schedule_336_e3160,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_336_e3162;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_337_e3174,) = {
    if (noise_variable_498 == 0.0) {
        let noise_metadata_schedule_337_e3166: f64 = (-noise_variable_141);
        let noise_metadata_schedule_337_e3168: f64 = (noise_metadata_schedule_337_e3166 / noise_variable_40);
        let noise_metadata_schedule_337_e3170: f64 = (noise_metadata_schedule_337_e3168 * noise_variable_100);
        let noise_metadata_schedule_337_e3172: f64 = (noise_metadata_schedule_337_e3170 * noise_variable_8);
        (noise_metadata_schedule_337_e3172,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_337_e3174;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_338_e3190,) = {
    if (noise_variable_498 == 0.0) {
        let noise_metadata_schedule_338_e3178: f64 = (noise_variable_275).exp();
        let noise_metadata_schedule_338_e3180: f64 = (noise_variable_276).exp();
        let noise_metadata_schedule_338_e3181: f64 = (noise_metadata_schedule_338_e3178 - noise_metadata_schedule_338_e3180);
        let noise_metadata_schedule_338_e3184: f64 = (noise_variable_100 * noise_variable_8);
        let noise_metadata_schedule_338_e3185: f64 = (noise_metadata_schedule_338_e3184).exp();
        let noise_metadata_schedule_338_e3187: f64 = (noise_metadata_schedule_338_e3185 - 1.0);
        let noise_metadata_schedule_338_e3188: f64 = (noise_metadata_schedule_338_e3181 / noise_metadata_schedule_338_e3187);
        (noise_metadata_schedule_338_e3188,)
    } else {
        (noise_variable_147,)
    }
};
            noise_variable_147 = noise_metadata_schedule_338_e3190;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_339_e3193: f64 = (0.1 * 0.1);
            noise_variable_267 = noise_metadata_schedule_339_e3193;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_340_e3196: f64 = (noise_variable_147 * noise_variable_147);
            noise_variable_268 = noise_metadata_schedule_340_e3196;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_341_e3199: f64 = if noise_variable_147 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_499 = noise_metadata_schedule_341_e3199;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_342_e3212,) = {
    if (noise_variable_499 != 0.0) {
        let noise_metadata_schedule_342_e3203: f64 = (0.5 * noise_variable_267);
        let noise_metadata_schedule_342_e3206: f64 = (noise_variable_268 + noise_variable_267);
        let noise_metadata_schedule_342_e3207: f64 = (noise_metadata_schedule_342_e3206).sqrt();
        let noise_metadata_schedule_342_e3209: f64 = (noise_metadata_schedule_342_e3207 - noise_variable_147);
        let noise_metadata_schedule_342_e3210: f64 = (noise_metadata_schedule_342_e3203 / noise_metadata_schedule_342_e3209);
        (noise_metadata_schedule_342_e3210,)
    } else {
        (noise_variable_148,)
    }
};
            noise_variable_148 = noise_metadata_schedule_342_e3212;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_343_e3224,) = {
    if (noise_variable_499 == 0.0) {
        let noise_metadata_schedule_343_e3218: f64 = (noise_variable_268 + noise_variable_267);
        let noise_metadata_schedule_343_e3219: f64 = (noise_metadata_schedule_343_e3218).sqrt();
        let noise_metadata_schedule_343_e3221: f64 = (noise_metadata_schedule_343_e3219 + noise_variable_147);
        let noise_metadata_schedule_343_e3222: f64 = (0.5 * noise_metadata_schedule_343_e3221);
        (noise_metadata_schedule_343_e3222,)
    } else {
        (noise_variable_148,)
    }
};
            noise_variable_148 = noise_metadata_schedule_343_e3224;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_344_e3230: f64 = (noise_variable_145 + noise_variable_146);
            let noise_metadata_schedule_344_e3231: f64 = (0.5 * noise_metadata_schedule_344_e3230);
            let noise_metadata_schedule_344_e3232: f64 = (1.0 + noise_metadata_schedule_344_e3231);
            let noise_metadata_schedule_344_e3233: f64 = (noise_variable_148 * noise_metadata_schedule_344_e3232);
            noise_variable_149 = noise_metadata_schedule_344_e3233;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_345_e3236: f64 = (params.p14 * noise_variable_35);
            let noise_metadata_schedule_345_e3238: f64 = (noise_metadata_schedule_345_e3236 * noise_variable_125);
            noise_variable_150 = noise_metadata_schedule_345_e3238;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_346_e3241: f64 = (noise_variable_35 * noise_variable_252);
            noise_variable_151 = noise_metadata_schedule_346_e3241;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_347_e3244: f64 = (noise_variable_151 - noise_variable_150);
            let noise_metadata_schedule_347_e3246: f64 = (noise_metadata_schedule_347_e3244 / noise_variable_149);
            noise_variable_152 = noise_metadata_schedule_347_e3246;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_348_e3249: f64 = noise_variable_238;
            let noise_metadata_schedule_348_e3251: f64 = (noise_metadata_schedule_348_e3249 / 0.0001);
            noise_variable_265 = noise_metadata_schedule_348_e3251;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_349_e3254: f64 = if noise_variable_238 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_500 = noise_metadata_schedule_349_e3254;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_350_e3266,) = {
    if (noise_variable_500 != 0.0) {
        let noise_metadata_schedule_350_e3260: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_350_e3261: f64 = (1.0 + noise_metadata_schedule_350_e3260);
        let noise_metadata_schedule_350_e3262: f64 = (noise_metadata_schedule_350_e3261).ln();
        let noise_metadata_schedule_350_e3263: f64 = (0.0001 * noise_metadata_schedule_350_e3262);
        let noise_metadata_schedule_350_e3264: f64 = noise_metadata_schedule_350_e3263;
        (noise_metadata_schedule_350_e3264,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_350_e3266;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_351_e3280,) = {
    if (noise_variable_500 == 0.0) {
        let noise_metadata_schedule_351_e3273: f64 = (-noise_variable_265);
        let noise_metadata_schedule_351_e3274: f64 = (noise_metadata_schedule_351_e3273).exp();
        let noise_metadata_schedule_351_e3275: f64 = (1.0 + noise_metadata_schedule_351_e3274);
        let noise_metadata_schedule_351_e3276: f64 = (noise_metadata_schedule_351_e3275).ln();
        let noise_metadata_schedule_351_e3277: f64 = (0.0001 * noise_metadata_schedule_351_e3276);
        let noise_metadata_schedule_351_e3278: f64 = (noise_variable_238 + noise_metadata_schedule_351_e3277);
        (noise_metadata_schedule_351_e3278,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_351_e3280;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_352_e3283: f64 = (noise_variable_282 / params.p143);
            noise_variable_284 = noise_metadata_schedule_352_e3283;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_353_e3286: f64 = if noise_variable_284 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_501 = noise_metadata_schedule_353_e3286;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_354_e3291,) = {
    if (noise_variable_501 != 0.0) {
        let noise_metadata_schedule_354_e3289: f64 = (noise_variable_284).exp();
        (noise_metadata_schedule_354_e3289,)
    } else {
        (noise_variable_285,)
    }
};
            noise_variable_285 = noise_metadata_schedule_354_e3291;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_355_e3297,) = {
    if (noise_variable_501 == 0.0) {
        let noise_metadata_schedule_355_e3295: f64 = (params.p138).exp();
        (noise_metadata_schedule_355_e3295,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_355_e3297;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_356_e3308,) = {
    if (noise_variable_501 == 0.0) {
        let noise_metadata_schedule_356_e3304: f64 = (noise_variable_284 - params.p138);
        let noise_metadata_schedule_356_e3305: f64 = (1.0 + noise_metadata_schedule_356_e3304);
        let noise_metadata_schedule_356_e3306: f64 = (noise_variable_281 * noise_metadata_schedule_356_e3305);
        (noise_metadata_schedule_356_e3306,)
    } else {
        (noise_variable_285,)
    }
};
            noise_variable_285 = noise_metadata_schedule_356_e3308;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_357_e3312: f64 = (noise_variable_285 - 1.0);
            let noise_metadata_schedule_357_e3313: f64 = (noise_variable_332 * noise_metadata_schedule_357_e3312);
            noise_variable_333 = noise_metadata_schedule_357_e3313;
        }
        if matches!(source_index, 1 | 2) {
            let noise_metadata_schedule_358_e3316: f64 = (noise_variable_238 - params.p145);
            let noise_metadata_schedule_358_e3318: f64 = (noise_metadata_schedule_358_e3316 / 0.001);
            noise_variable_265 = noise_metadata_schedule_358_e3318;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_359_e3321: f64 = if noise_variable_238 < params.p145 { 1.0 } else { 0.0 };
            noise_variable_502 = noise_metadata_schedule_359_e3321;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_360_e3333,) = {
    if (noise_variable_502 != 0.0) {
        let noise_metadata_schedule_360_e3327: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_360_e3328: f64 = (1.0 + noise_metadata_schedule_360_e3327);
        let noise_metadata_schedule_360_e3329: f64 = (noise_metadata_schedule_360_e3328).ln();
        let noise_metadata_schedule_360_e3330: f64 = (0.001 * noise_metadata_schedule_360_e3329);
        let noise_metadata_schedule_360_e3331: f64 = (noise_variable_238 - noise_metadata_schedule_360_e3330);
        (noise_metadata_schedule_360_e3331,)
    } else {
        (noise_variable_286,)
    }
};
            noise_variable_286 = noise_metadata_schedule_360_e3333;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_361_e3347,) = {
    if (noise_variable_502 == 0.0) {
        let noise_metadata_schedule_361_e3340: f64 = (-noise_variable_265);
        let noise_metadata_schedule_361_e3341: f64 = (noise_metadata_schedule_361_e3340).exp();
        let noise_metadata_schedule_361_e3342: f64 = (1.0 + noise_metadata_schedule_361_e3341);
        let noise_metadata_schedule_361_e3343: f64 = (noise_metadata_schedule_361_e3342).ln();
        let noise_metadata_schedule_361_e3344: f64 = (0.001 * noise_metadata_schedule_361_e3343);
        let noise_metadata_schedule_361_e3345: f64 = (params.p145 - noise_metadata_schedule_361_e3344);
        (noise_metadata_schedule_361_e3345,)
    } else {
        (noise_variable_286,)
    }
};
            noise_variable_286 = noise_metadata_schedule_361_e3347;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_362_e3350: f64 = (params.p146 * noise_variable_286);
            let noise_metadata_schedule_362_e3353: f64 = (params.p145 - noise_variable_286);
            let noise_metadata_schedule_362_e3355: f64 = {let pb=noise_metadata_schedule_362_e3353;pb*pb};
            let noise_metadata_schedule_362_e3356: f64 = (noise_metadata_schedule_362_e3350 * noise_metadata_schedule_362_e3355);
            noise_variable_334 = noise_metadata_schedule_362_e3356;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_363_e3359: f64 = (noise_variable_238 * noise_variable_8);
            let noise_metadata_schedule_363_e3361: f64 = (noise_metadata_schedule_363_e3359 / params.p16);
            let noise_metadata_schedule_363_e3363: f64 = if noise_metadata_schedule_363_e3361 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_503 = noise_metadata_schedule_363_e3363;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_364_e3372,) = {
    if (noise_variable_503 != 0.0) {
        let noise_metadata_schedule_364_e3367: f64 = (noise_variable_238 * noise_variable_8);
        let noise_metadata_schedule_364_e3369: f64 = (noise_metadata_schedule_364_e3367 / params.p16);
        let noise_metadata_schedule_364_e3370: f64 = (noise_metadata_schedule_364_e3369).exp();
        (noise_metadata_schedule_364_e3370,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_364_e3372;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_365_e3378,) = {
    if (noise_variable_503 == 0.0) {
        let noise_metadata_schedule_365_e3376: f64 = (params.p138).exp();
        (noise_metadata_schedule_365_e3376,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_365_e3378;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_366_e3393,) = {
    if (noise_variable_503 == 0.0) {
        let noise_metadata_schedule_366_e3385: f64 = (noise_variable_238 * noise_variable_8);
        let noise_metadata_schedule_366_e3387: f64 = (noise_metadata_schedule_366_e3385 / params.p16);
        let noise_metadata_schedule_366_e3389: f64 = (noise_metadata_schedule_366_e3387 - params.p138);
        let noise_metadata_schedule_366_e3390: f64 = (1.0 + noise_metadata_schedule_366_e3389);
        let noise_metadata_schedule_366_e3391: f64 = (noise_variable_281 * noise_metadata_schedule_366_e3390);
        (noise_metadata_schedule_366_e3391,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_366_e3393;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_367_e3396: f64 = if params.p23 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_504 = noise_metadata_schedule_367_e3396;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_368_e3399: f64 = (noise_variable_238 - noise_variable_55);
            let noise_metadata_schedule_368_e3401: f64 = (noise_metadata_schedule_368_e3399 * noise_variable_8);
            let noise_metadata_schedule_368_e3403: f64 = if noise_metadata_schedule_368_e3401 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_505 = noise_metadata_schedule_368_e3403;
        }
        if matches!(source_index, 2 | 6 | 8) {
            let (noise_metadata_schedule_369_e3414,) = {
    if ((noise_variable_504 != 0.0) && (noise_variable_505 != 0.0)) {
        let noise_metadata_schedule_369_e3409: f64 = (noise_variable_238 - noise_variable_55);
        let noise_metadata_schedule_369_e3411: f64 = (noise_metadata_schedule_369_e3409 * noise_variable_8);
        let noise_metadata_schedule_369_e3412: f64 = (noise_metadata_schedule_369_e3411).exp();
        (noise_metadata_schedule_369_e3412,)
    } else {
        (noise_variable_284,)
    }
};
            noise_variable_284 = noise_metadata_schedule_369_e3414;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_370_e3422,) = {
    if ((noise_variable_504 != 0.0) && (noise_variable_505 == 0.0)) {
        let noise_metadata_schedule_370_e3420: f64 = (params.p138).exp();
        (noise_metadata_schedule_370_e3420,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_370_e3422;
        }
        if matches!(source_index, 2 | 6 | 8) {
            let (noise_metadata_schedule_371_e3439,) = {
    if ((noise_variable_504 != 0.0) && (noise_variable_505 == 0.0)) {
        let noise_metadata_schedule_371_e3431: f64 = (noise_variable_238 - noise_variable_55);
        let noise_metadata_schedule_371_e3433: f64 = (noise_metadata_schedule_371_e3431 * noise_variable_8);
        let noise_metadata_schedule_371_e3435: f64 = (noise_metadata_schedule_371_e3433 - params.p138);
        let noise_metadata_schedule_371_e3436: f64 = (1.0 + noise_metadata_schedule_371_e3435);
        let noise_metadata_schedule_371_e3437: f64 = (noise_variable_281 * noise_metadata_schedule_371_e3436);
        (noise_metadata_schedule_371_e3437,)
    } else {
        (noise_variable_284,)
    }
};
            noise_variable_284 = noise_metadata_schedule_371_e3439;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_372_e3442: f64 = (noise_variable_152 / noise_variable_35);
            let noise_metadata_schedule_372_e3444: f64 = (noise_metadata_schedule_372_e3442 - 1000.0);
            let noise_metadata_schedule_372_e3446: f64 = if noise_metadata_schedule_372_e3444 < 40.0 { 1.0 } else { 0.0 };
            noise_variable_506 = noise_metadata_schedule_372_e3446;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_373_e3457,) = {
    if ((noise_variable_504 != 0.0) && (noise_variable_506 != 0.0)) {
        let noise_metadata_schedule_373_e3452: f64 = (noise_variable_152 / noise_variable_35);
        let noise_metadata_schedule_373_e3454: f64 = (noise_metadata_schedule_373_e3452 - 1000.0);
        let noise_metadata_schedule_373_e3455: f64 = (noise_metadata_schedule_373_e3454).exp();
        (noise_metadata_schedule_373_e3455,)
    } else {
        (noise_variable_285,)
    }
};
            noise_variable_285 = noise_metadata_schedule_373_e3457;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_374_e3465,) = {
    if ((noise_variable_504 != 0.0) && (noise_variable_506 == 0.0)) {
        let noise_metadata_schedule_374_e3463: f64 = (40.0_f64).exp();
        (noise_metadata_schedule_374_e3463,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_374_e3465;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_375_e3482,) = {
    if ((noise_variable_504 != 0.0) && (noise_variable_506 == 0.0)) {
        let noise_metadata_schedule_375_e3474: f64 = (noise_variable_152 / noise_variable_35);
        let noise_metadata_schedule_375_e3476: f64 = (noise_metadata_schedule_375_e3474 - 1000.0);
        let noise_metadata_schedule_375_e3478: f64 = (noise_metadata_schedule_375_e3476 - 40.0);
        let noise_metadata_schedule_375_e3479: f64 = (1.0 + noise_metadata_schedule_375_e3478);
        let noise_metadata_schedule_375_e3480: f64 = (noise_variable_281 * noise_metadata_schedule_375_e3479);
        (noise_metadata_schedule_375_e3480,)
    } else {
        (noise_variable_285,)
    }
};
            noise_variable_285 = noise_metadata_schedule_375_e3482;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_376_e3525,) = {
    if (noise_variable_504 != 0.0) {
        let noise_metadata_schedule_376_e3487: f64 = (noise_variable_282 - 1.0);
        let noise_metadata_schedule_376_e3488: f64 = (noise_variable_42 * noise_metadata_schedule_376_e3487);
        let noise_metadata_schedule_376_e3491: f64 = (noise_variable_53 * 2.0);
        let noise_metadata_schedule_376_e3494: f64 = (noise_variable_282 - 1.0);
        let noise_metadata_schedule_376_e3495: f64 = (noise_metadata_schedule_376_e3491 * noise_metadata_schedule_376_e3494);
        let noise_metadata_schedule_376_e3500: f64 = (4.0 * noise_variable_284);
        let noise_metadata_schedule_376_e3501: f64 = (1.0 + noise_metadata_schedule_376_e3500);
        let noise_metadata_schedule_376_e3502: f64 = (noise_metadata_schedule_376_e3501).sqrt();
        let noise_metadata_schedule_376_e3503: f64 = (1.0 + noise_metadata_schedule_376_e3502);
        let noise_metadata_schedule_376_e3504: f64 = (noise_metadata_schedule_376_e3495 / noise_metadata_schedule_376_e3503);
        let noise_metadata_schedule_376_e3508: f64 = (noise_variable_141 / noise_variable_40);
        let noise_metadata_schedule_376_e3509: f64 = (1.0 + noise_metadata_schedule_376_e3508);
        let noise_metadata_schedule_376_e3510: f64 = (noise_metadata_schedule_376_e3504 * noise_metadata_schedule_376_e3509);
        let noise_metadata_schedule_376_e3511: f64 = (noise_metadata_schedule_376_e3488 + noise_metadata_schedule_376_e3510);
        let noise_metadata_schedule_376_e3515: f64 = (noise_variable_124 - 1.0);
        let noise_metadata_schedule_376_e3516: f64 = (noise_variable_54 * noise_metadata_schedule_376_e3515);
        let noise_metadata_schedule_376_e3518: f64 = (noise_metadata_schedule_376_e3516 * noise_variable_285);
        let noise_metadata_schedule_376_e3521: f64 = (1.0 + noise_variable_285);
        let noise_metadata_schedule_376_e3522: f64 = (noise_metadata_schedule_376_e3518 / noise_metadata_schedule_376_e3521);
        let noise_metadata_schedule_376_e3523: f64 = (noise_metadata_schedule_376_e3511 + noise_metadata_schedule_376_e3522);
        (noise_metadata_schedule_376_e3523,)
    } else {
        (noise_variable_154,)
    }
};
            noise_variable_154 = noise_metadata_schedule_376_e3525;
        }
        if matches!(source_index, 2 | 6) {
            let noise_metadata_schedule_377_e3528: f64 = if params.p92 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_507 = noise_metadata_schedule_377_e3528;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_378_e3539,) = {
    if ((noise_variable_504 == 0.0) && (noise_variable_507 != 0.0)) {
        let noise_metadata_schedule_378_e3536: f64 = (noise_variable_282 - 1.0);
        let noise_metadata_schedule_378_e3537: f64 = (noise_variable_42 * noise_metadata_schedule_378_e3536);
        (noise_metadata_schedule_378_e3537,)
    } else {
        (noise_variable_154,)
    }
};
            noise_variable_154 = noise_metadata_schedule_378_e3539;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_379_e3569,) = {
    if ((noise_variable_504 == 0.0) && (noise_variable_507 == 0.0)) {
        let noise_metadata_schedule_379_e3548: f64 = (1.0 - params.p92);
        let noise_metadata_schedule_379_e3551: f64 = (noise_variable_282 - 1.0);
        let noise_metadata_schedule_379_e3552: f64 = (noise_metadata_schedule_379_e3548 * noise_metadata_schedule_379_e3551);
        let noise_metadata_schedule_379_e3556: f64 = (noise_variable_282 + noise_variable_124);
        let noise_metadata_schedule_379_e3558: f64 = (noise_metadata_schedule_379_e3556 - 2.0);
        let noise_metadata_schedule_379_e3559: f64 = (params.p92 * noise_metadata_schedule_379_e3558);
        let noise_metadata_schedule_379_e3563: f64 = (noise_variable_141 / noise_variable_40);
        let noise_metadata_schedule_379_e3564: f64 = (1.0 + noise_metadata_schedule_379_e3563);
        let noise_metadata_schedule_379_e3565: f64 = (noise_metadata_schedule_379_e3559 * noise_metadata_schedule_379_e3564);
        let noise_metadata_schedule_379_e3566: f64 = (noise_metadata_schedule_379_e3552 + noise_metadata_schedule_379_e3565);
        let noise_metadata_schedule_379_e3567: f64 = (noise_variable_42 * noise_metadata_schedule_379_e3566);
        (noise_metadata_schedule_379_e3567,)
    } else {
        (noise_variable_154,)
    }
};
            noise_variable_154 = noise_metadata_schedule_379_e3569;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_380_e3572: f64 = (noise_variable_239 * noise_variable_8);
            let noise_metadata_schedule_380_e3574: f64 = (noise_metadata_schedule_380_e3572 / params.p18);
            let noise_metadata_schedule_380_e3576: f64 = if noise_metadata_schedule_380_e3574 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_508 = noise_metadata_schedule_380_e3576;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_381_e3585,) = {
    if (noise_variable_508 != 0.0) {
        let noise_metadata_schedule_381_e3580: f64 = (noise_variable_239 * noise_variable_8);
        let noise_metadata_schedule_381_e3582: f64 = (noise_metadata_schedule_381_e3580 / params.p18);
        let noise_metadata_schedule_381_e3583: f64 = (noise_metadata_schedule_381_e3582).exp();
        (noise_metadata_schedule_381_e3583,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_381_e3585;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_382_e3591,) = {
    if (noise_variable_508 == 0.0) {
        let noise_metadata_schedule_382_e3589: f64 = (params.p138).exp();
        (noise_metadata_schedule_382_e3589,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_382_e3591;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_383_e3606,) = {
    if (noise_variable_508 == 0.0) {
        let noise_metadata_schedule_383_e3598: f64 = (noise_variable_239 * noise_variable_8);
        let noise_metadata_schedule_383_e3600: f64 = (noise_metadata_schedule_383_e3598 / params.p18);
        let noise_metadata_schedule_383_e3602: f64 = (noise_metadata_schedule_383_e3600 - params.p138);
        let noise_metadata_schedule_383_e3603: f64 = (1.0 + noise_metadata_schedule_383_e3602);
        let noise_metadata_schedule_383_e3604: f64 = (noise_variable_281 * noise_metadata_schedule_383_e3603);
        (noise_metadata_schedule_383_e3604,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_383_e3606;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_384_e3609: f64 = if params.p23 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_509 = noise_metadata_schedule_384_e3609;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_385_e3612: f64 = (noise_variable_239 - noise_variable_55);
            let noise_metadata_schedule_385_e3614: f64 = (noise_metadata_schedule_385_e3612 * noise_variable_8);
            let noise_metadata_schedule_385_e3616: f64 = if noise_metadata_schedule_385_e3614 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_510 = noise_metadata_schedule_385_e3616;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_386_e3627,) = {
    if ((noise_variable_509 != 0.0) && (noise_variable_510 != 0.0)) {
        let noise_metadata_schedule_386_e3622: f64 = (noise_variable_239 - noise_variable_55);
        let noise_metadata_schedule_386_e3624: f64 = (noise_metadata_schedule_386_e3622 * noise_variable_8);
        let noise_metadata_schedule_386_e3625: f64 = (noise_metadata_schedule_386_e3624).exp();
        (noise_metadata_schedule_386_e3625,)
    } else {
        (noise_variable_284,)
    }
};
            noise_variable_284 = noise_metadata_schedule_386_e3627;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_387_e3635,) = {
    if ((noise_variable_509 != 0.0) && (noise_variable_510 == 0.0)) {
        let noise_metadata_schedule_387_e3633: f64 = (params.p138).exp();
        (noise_metadata_schedule_387_e3633,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_387_e3635;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_388_e3652,) = {
    if ((noise_variable_509 != 0.0) && (noise_variable_510 == 0.0)) {
        let noise_metadata_schedule_388_e3644: f64 = (noise_variable_239 - noise_variable_55);
        let noise_metadata_schedule_388_e3646: f64 = (noise_metadata_schedule_388_e3644 * noise_variable_8);
        let noise_metadata_schedule_388_e3648: f64 = (noise_metadata_schedule_388_e3646 - params.p138);
        let noise_metadata_schedule_388_e3649: f64 = (1.0 + noise_metadata_schedule_388_e3648);
        let noise_metadata_schedule_388_e3650: f64 = (noise_variable_281 * noise_metadata_schedule_388_e3649);
        (noise_metadata_schedule_388_e3650,)
    } else {
        (noise_variable_284,)
    }
};
            noise_variable_284 = noise_metadata_schedule_388_e3652;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_389_e3677,) = {
    if (noise_variable_509 != 0.0) {
        let noise_metadata_schedule_389_e3657: f64 = (noise_variable_282 - 1.0);
        let noise_metadata_schedule_389_e3658: f64 = (noise_variable_44 * noise_metadata_schedule_389_e3657);
        let noise_metadata_schedule_389_e3661: f64 = (noise_variable_45 * 2.0);
        let noise_metadata_schedule_389_e3664: f64 = (noise_variable_282 - 1.0);
        let noise_metadata_schedule_389_e3665: f64 = (noise_metadata_schedule_389_e3661 * noise_metadata_schedule_389_e3664);
        let noise_metadata_schedule_389_e3670: f64 = (4.0 * noise_variable_284);
        let noise_metadata_schedule_389_e3671: f64 = (1.0 + noise_metadata_schedule_389_e3670);
        let noise_metadata_schedule_389_e3672: f64 = (noise_metadata_schedule_389_e3671).sqrt();
        let noise_metadata_schedule_389_e3673: f64 = (1.0 + noise_metadata_schedule_389_e3672);
        let noise_metadata_schedule_389_e3674: f64 = (noise_metadata_schedule_389_e3665 / noise_metadata_schedule_389_e3673);
        let noise_metadata_schedule_389_e3675: f64 = (noise_metadata_schedule_389_e3658 + noise_metadata_schedule_389_e3674);
        (noise_metadata_schedule_389_e3675,)
    } else {
        (noise_variable_155,)
    }
};
            noise_variable_155 = noise_metadata_schedule_389_e3677;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_390_e3686,) = {
    if (noise_variable_509 == 0.0) {
        let noise_metadata_schedule_390_e3683: f64 = (noise_variable_282 - 1.0);
        let noise_metadata_schedule_390_e3684: f64 = (noise_variable_44 * noise_metadata_schedule_390_e3683);
        (noise_metadata_schedule_390_e3684,)
    } else {
        (noise_variable_155,)
    }
};
            noise_variable_155 = noise_metadata_schedule_390_e3686;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_391_e3689: f64 = (noise_variable_238 * noise_variable_8);
            let noise_metadata_schedule_391_e3691: f64 = (noise_metadata_schedule_391_e3689 / params.p20);
            let noise_metadata_schedule_391_e3693: f64 = if noise_metadata_schedule_391_e3691 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_511 = noise_metadata_schedule_391_e3693;
        }
        if matches!(source_index, 2 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_392_e3702,) = {
    if (noise_variable_511 != 0.0) {
        let noise_metadata_schedule_392_e3697: f64 = (noise_variable_238 * noise_variable_8);
        let noise_metadata_schedule_392_e3699: f64 = (noise_metadata_schedule_392_e3697 / params.p20);
        let noise_metadata_schedule_392_e3700: f64 = (noise_metadata_schedule_392_e3699).exp();
        (noise_metadata_schedule_392_e3700,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_392_e3702;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_393_e3708,) = {
    if (noise_variable_511 == 0.0) {
        let noise_metadata_schedule_393_e3706: f64 = (params.p138).exp();
        (noise_metadata_schedule_393_e3706,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_393_e3708;
        }
        if matches!(source_index, 2 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_394_e3723,) = {
    if (noise_variable_511 == 0.0) {
        let noise_metadata_schedule_394_e3715: f64 = (noise_variable_238 * noise_variable_8);
        let noise_metadata_schedule_394_e3717: f64 = (noise_metadata_schedule_394_e3715 / params.p20);
        let noise_metadata_schedule_394_e3719: f64 = (noise_metadata_schedule_394_e3717 - params.p138);
        let noise_metadata_schedule_394_e3720: f64 = (1.0 + noise_metadata_schedule_394_e3719);
        let noise_metadata_schedule_394_e3721: f64 = (noise_variable_281 * noise_metadata_schedule_394_e3720);
        (noise_metadata_schedule_394_e3721,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_394_e3723;
        }
        if matches!(source_index, 2 | 7) {
            let noise_metadata_schedule_395_e3727: f64 = (noise_variable_282 - 1.0);
            let noise_metadata_schedule_395_e3728: f64 = (noise_variable_38 * noise_metadata_schedule_395_e3727);
            noise_variable_156 = noise_metadata_schedule_395_e3728;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_396_e3731: f64 = (noise_variable_239 * noise_variable_8);
            let noise_metadata_schedule_396_e3733: f64 = (noise_metadata_schedule_396_e3731 / params.p22);
            let noise_metadata_schedule_396_e3735: f64 = if noise_metadata_schedule_396_e3733 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_512 = noise_metadata_schedule_396_e3735;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_397_e3744,) = {
    if (noise_variable_512 != 0.0) {
        let noise_metadata_schedule_397_e3739: f64 = (noise_variable_239 * noise_variable_8);
        let noise_metadata_schedule_397_e3741: f64 = (noise_metadata_schedule_397_e3739 / params.p22);
        let noise_metadata_schedule_397_e3742: f64 = (noise_metadata_schedule_397_e3741).exp();
        (noise_metadata_schedule_397_e3742,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_397_e3744;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_398_e3750,) = {
    if (noise_variable_512 == 0.0) {
        let noise_metadata_schedule_398_e3748: f64 = (params.p138).exp();
        (noise_metadata_schedule_398_e3748,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_398_e3750;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_399_e3765,) = {
    if (noise_variable_512 == 0.0) {
        let noise_metadata_schedule_399_e3757: f64 = (noise_variable_239 * noise_variable_8);
        let noise_metadata_schedule_399_e3759: f64 = (noise_metadata_schedule_399_e3757 / params.p22);
        let noise_metadata_schedule_399_e3761: f64 = (noise_metadata_schedule_399_e3759 - params.p138);
        let noise_metadata_schedule_399_e3762: f64 = (1.0 + noise_metadata_schedule_399_e3761);
        let noise_metadata_schedule_399_e3763: f64 = (noise_variable_281 * noise_metadata_schedule_399_e3762);
        (noise_metadata_schedule_399_e3763,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_399_e3765;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_400_e3769: f64 = (noise_variable_282 - 1.0);
            let noise_metadata_schedule_400_e3770: f64 = (noise_variable_46 * noise_metadata_schedule_400_e3769);
            noise_variable_158 = noise_metadata_schedule_400_e3770;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_401_e3773: f64 = (noise_variable_241 * noise_variable_8);
            let noise_metadata_schedule_401_e3775: f64 = (noise_metadata_schedule_401_e3773 / params.p31);
            let noise_metadata_schedule_401_e3777: f64 = if noise_metadata_schedule_401_e3775 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_513 = noise_metadata_schedule_401_e3777;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_402_e3786,) = {
    if (noise_variable_513 != 0.0) {
        let noise_metadata_schedule_402_e3781: f64 = (noise_variable_241 * noise_variable_8);
        let noise_metadata_schedule_402_e3783: f64 = (noise_metadata_schedule_402_e3781 / params.p31);
        let noise_metadata_schedule_402_e3784: f64 = (noise_metadata_schedule_402_e3783).exp();
        (noise_metadata_schedule_402_e3784,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_402_e3786;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_403_e3792,) = {
    if (noise_variable_513 == 0.0) {
        let noise_metadata_schedule_403_e3790: f64 = (params.p138).exp();
        (noise_metadata_schedule_403_e3790,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_403_e3792;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_404_e3807,) = {
    if (noise_variable_513 == 0.0) {
        let noise_metadata_schedule_404_e3799: f64 = (noise_variable_241 * noise_variable_8);
        let noise_metadata_schedule_404_e3801: f64 = (noise_metadata_schedule_404_e3799 / params.p31);
        let noise_metadata_schedule_404_e3803: f64 = (noise_metadata_schedule_404_e3801 - params.p138);
        let noise_metadata_schedule_404_e3804: f64 = (1.0 + noise_metadata_schedule_404_e3803);
        let noise_metadata_schedule_404_e3805: f64 = (noise_variable_281 * noise_metadata_schedule_404_e3804);
        (noise_metadata_schedule_404_e3805,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_404_e3807;
        }
        if matches!(source_index, 9 | 10) {
            let noise_metadata_schedule_405_e3811: f64 = (noise_variable_282 - 1.0);
            let noise_metadata_schedule_405_e3812: f64 = (noise_variable_39 * noise_metadata_schedule_405_e3811);
            noise_variable_157 = noise_metadata_schedule_405_e3812;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 15 | 16) {
            let noise_metadata_schedule_406_e3815: f64 = (noise_variable_239 * noise_variable_8);
            let noise_metadata_schedule_406_e3817: f64 = (noise_metadata_schedule_406_e3815 / params.p137);
            let noise_metadata_schedule_406_e3819: f64 = if noise_metadata_schedule_406_e3817 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_514 = noise_metadata_schedule_406_e3819;
        }
        if matches!(source_index, 7 | 8) {
            let (noise_metadata_schedule_407_e3828,) = {
    if (noise_variable_514 != 0.0) {
        let noise_metadata_schedule_407_e3823: f64 = (noise_variable_239 * noise_variable_8);
        let noise_metadata_schedule_407_e3825: f64 = (noise_metadata_schedule_407_e3823 / params.p137);
        let noise_metadata_schedule_407_e3826: f64 = (noise_metadata_schedule_407_e3825).exp();
        (noise_metadata_schedule_407_e3826,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_407_e3828;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 15 | 16) {
            let (noise_metadata_schedule_408_e3834,) = {
    if (noise_variable_514 == 0.0) {
        let noise_metadata_schedule_408_e3832: f64 = (params.p138).exp();
        (noise_metadata_schedule_408_e3832,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_408_e3834;
        }
        if matches!(source_index, 7 | 8) {
            let (noise_metadata_schedule_409_e3849,) = {
    if (noise_variable_514 == 0.0) {
        let noise_metadata_schedule_409_e3841: f64 = (noise_variable_239 * noise_variable_8);
        let noise_metadata_schedule_409_e3843: f64 = (noise_metadata_schedule_409_e3841 / params.p137);
        let noise_metadata_schedule_409_e3845: f64 = (noise_metadata_schedule_409_e3843 - params.p138);
        let noise_metadata_schedule_409_e3846: f64 = (1.0 + noise_metadata_schedule_409_e3845);
        let noise_metadata_schedule_409_e3847: f64 = (noise_variable_281 * noise_metadata_schedule_409_e3846);
        (noise_metadata_schedule_409_e3847,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_409_e3849;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_410_e3853: f64 = (noise_variable_282 - 1.0);
            let noise_metadata_schedule_410_e3854: f64 = (noise_variable_47 * noise_metadata_schedule_410_e3853);
            noise_variable_159 = noise_metadata_schedule_410_e3854;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_411_e3865: f64 = if (((params.p33 > 0.0) && (params.p34 > 0.0)) && (noise_variable_238 < 0.0)) { 1.0 } else { 0.0 };
            noise_variable_515 = noise_metadata_schedule_411_e3865;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_412_e3871: f64 = (2.0 * noise_variable_59);
            let noise_metadata_schedule_412_e3872: f64 = (noise_variable_62 / noise_metadata_schedule_412_e3871);
            let noise_metadata_schedule_412_e3873: f64 = (1.0 - noise_metadata_schedule_412_e3872);
            let noise_metadata_schedule_412_e3874: f64 = (noise_variable_61 * noise_metadata_schedule_412_e3873);
            let noise_metadata_schedule_412_e3876: f64 = if noise_metadata_schedule_412_e3874 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_516 = noise_metadata_schedule_412_e3876;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_413_e3891,) = {
    if ((noise_variable_515 != 0.0) && (noise_variable_516 != 0.0)) {
        let noise_metadata_schedule_413_e3885: f64 = (2.0 * noise_variable_59);
        let noise_metadata_schedule_413_e3886: f64 = (noise_variable_62 / noise_metadata_schedule_413_e3885);
        let noise_metadata_schedule_413_e3887: f64 = (1.0 - noise_metadata_schedule_413_e3886);
        let noise_metadata_schedule_413_e3888: f64 = (noise_variable_61 * noise_metadata_schedule_413_e3887);
        let noise_metadata_schedule_413_e3889: f64 = (noise_metadata_schedule_413_e3888).exp();
        (noise_metadata_schedule_413_e3889,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_413_e3891;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_414_e3899,) = {
    if ((noise_variable_515 != 0.0) && (noise_variable_516 == 0.0)) {
        let noise_metadata_schedule_414_e3897: f64 = (params.p138).exp();
        (noise_metadata_schedule_414_e3897,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_414_e3899;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_415_e3920,) = {
    if ((noise_variable_515 != 0.0) && (noise_variable_516 == 0.0)) {
        let noise_metadata_schedule_415_e3911: f64 = (2.0 * noise_variable_59);
        let noise_metadata_schedule_415_e3912: f64 = (noise_variable_62 / noise_metadata_schedule_415_e3911);
        let noise_metadata_schedule_415_e3913: f64 = (1.0 - noise_metadata_schedule_415_e3912);
        let noise_metadata_schedule_415_e3914: f64 = (noise_variable_61 * noise_metadata_schedule_415_e3913);
        let noise_metadata_schedule_415_e3916: f64 = (noise_metadata_schedule_415_e3914 - params.p138);
        let noise_metadata_schedule_415_e3917: f64 = (1.0 + noise_metadata_schedule_415_e3916);
        let noise_metadata_schedule_415_e3918: f64 = (noise_variable_281 * noise_metadata_schedule_415_e3917);
        (noise_metadata_schedule_415_e3918,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_415_e3920;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_416_e3926,) = {
    if (noise_variable_515 != 0.0) {
        let noise_metadata_schedule_416_e3924: f64 = (noise_variable_238 * noise_variable_65);
        (noise_metadata_schedule_416_e3924,)
    } else {
        (noise_variable_261,)
    }
};
            noise_variable_261 = noise_metadata_schedule_416_e3926;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_417_e3970,) = {
    if (noise_variable_515 != 0.0) {
        let noise_metadata_schedule_417_e3930: f64 = (noise_variable_261 * noise_variable_261);
        let noise_metadata_schedule_417_e3932: f64 = (noise_metadata_schedule_417_e3930 + 1e-30);
        let noise_metadata_schedule_417_e3933: f64 = (noise_metadata_schedule_417_e3932).sqrt();
        let noise_metadata_schedule_417_e3935: f64 = (-2.0);
        let noise_metadata_schedule_417_e3937: f64 = (noise_metadata_schedule_417_e3935 - params.p66);
        let noise_metadata_schedule_417_e3938: f64 = (noise_metadata_schedule_417_e3933).powf(noise_metadata_schedule_417_e3937);
        let noise_metadata_schedule_417_e3943: f64 = (params.p66 * params.p66);
        let noise_metadata_schedule_417_e3944: f64 = (1.0 - noise_metadata_schedule_417_e3943);
        let noise_metadata_schedule_417_e3947: f64 = (3.0 * noise_variable_261);
        let noise_metadata_schedule_417_e3950: f64 = (params.p66 - 1.0);
        let noise_metadata_schedule_417_e3951: f64 = (noise_metadata_schedule_417_e3947 * noise_metadata_schedule_417_e3950);
        let noise_metadata_schedule_417_e3952: f64 = (noise_metadata_schedule_417_e3944 - noise_metadata_schedule_417_e3951);
        let noise_metadata_schedule_417_e3953: f64 = (params.p66 * noise_metadata_schedule_417_e3952);
        let noise_metadata_schedule_417_e3956: f64 = (6.0 * noise_variable_261);
        let noise_metadata_schedule_417_e3958: f64 = (noise_metadata_schedule_417_e3956 * noise_variable_261);
        let noise_metadata_schedule_417_e3961: f64 = (params.p66 - 1.0);
        let noise_metadata_schedule_417_e3963: f64 = (noise_metadata_schedule_417_e3961 + noise_variable_261);
        let noise_metadata_schedule_417_e3964: f64 = (noise_metadata_schedule_417_e3958 * noise_metadata_schedule_417_e3963);
        let noise_metadata_schedule_417_e3965: f64 = (noise_metadata_schedule_417_e3953 - noise_metadata_schedule_417_e3964);
        let noise_metadata_schedule_417_e3966: f64 = (noise_metadata_schedule_417_e3938 * noise_metadata_schedule_417_e3965);
        let noise_metadata_schedule_417_e3968: f64 = (noise_metadata_schedule_417_e3966 * 0.16666666666666666);
        (noise_metadata_schedule_417_e3968,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_417_e3970;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_418_e3982,) = {
    if (noise_variable_515 != 0.0) {
        let noise_metadata_schedule_418_e3974: f64 = (noise_variable_238 * noise_variable_62);
        let noise_metadata_schedule_418_e3976: f64 = (noise_metadata_schedule_418_e3974 * noise_variable_61);
        let noise_metadata_schedule_418_e3979: f64 = (noise_variable_70 * noise_variable_60);
        let noise_metadata_schedule_418_e3980: f64 = (noise_metadata_schedule_418_e3976 / noise_metadata_schedule_418_e3979);
        (noise_metadata_schedule_418_e3980,)
    } else {
        (noise_variable_261,)
    }
};
            noise_variable_261 = noise_metadata_schedule_418_e3982;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_419_e3985: f64 = (-0.001);
            let noise_metadata_schedule_419_e3986: f64 = if noise_variable_261 < noise_metadata_schedule_419_e3985 { 1.0 } else { 0.0 };
            noise_variable_517 = noise_metadata_schedule_419_e3986;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_420_e3989: f64 = if noise_variable_261 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_518 = noise_metadata_schedule_420_e3989;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_421_e3998,) = {
    if (((noise_variable_515 != 0.0) && (noise_variable_517 != 0.0)) && (noise_variable_518 != 0.0)) {
        let noise_metadata_schedule_421_e3996: f64 = (noise_variable_261).exp();
        (noise_metadata_schedule_421_e3996,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_421_e3998;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_422_e4008,) = {
    if (((noise_variable_515 != 0.0) && (noise_variable_517 != 0.0)) && (noise_variable_518 == 0.0)) {
        let noise_metadata_schedule_422_e4006: f64 = (params.p138).exp();
        (noise_metadata_schedule_422_e4006,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_422_e4008;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_423_e4023,) = {
    if (((noise_variable_515 != 0.0) && (noise_variable_517 != 0.0)) && (noise_variable_518 == 0.0)) {
        let noise_metadata_schedule_423_e4019: f64 = (noise_variable_261 - params.p138);
        let noise_metadata_schedule_423_e4020: f64 = (1.0 + noise_metadata_schedule_423_e4019);
        let noise_metadata_schedule_423_e4021: f64 = (noise_variable_281 * noise_metadata_schedule_423_e4020);
        (noise_metadata_schedule_423_e4021,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_423_e4023;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_424_e4038,) = {
    if ((noise_variable_515 != 0.0) && (noise_variable_517 != 0.0)) {
        let noise_metadata_schedule_424_e4028: f64 = (-noise_variable_238);
        let noise_metadata_schedule_424_e4032: f64 = (1.0 - noise_variable_91);
        let noise_metadata_schedule_424_e4034: f64 = (noise_metadata_schedule_424_e4032 / noise_variable_261);
        let noise_metadata_schedule_424_e4035: f64 = (1.0 + noise_metadata_schedule_424_e4034);
        let noise_metadata_schedule_424_e4036: f64 = (noise_metadata_schedule_424_e4028 * noise_metadata_schedule_424_e4035);
        (noise_metadata_schedule_424_e4036,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_424_e4038;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_425_e4061,) = {
    if ((noise_variable_515 != 0.0) && (noise_variable_517 == 0.0)) {
        let noise_metadata_schedule_425_e4045: f64 = (noise_variable_238 * 0.5);
        let noise_metadata_schedule_425_e4047: f64 = (noise_metadata_schedule_425_e4045 * noise_variable_261);
        let noise_metadata_schedule_425_e4051: f64 = (noise_variable_261 * 0.3333333333333333);
        let noise_metadata_schedule_425_e4055: f64 = (0.25 * noise_variable_261);
        let noise_metadata_schedule_425_e4056: f64 = (1.0 + noise_metadata_schedule_425_e4055);
        let noise_metadata_schedule_425_e4057: f64 = (noise_metadata_schedule_425_e4051 * noise_metadata_schedule_425_e4056);
        let noise_metadata_schedule_425_e4058: f64 = (1.0 + noise_metadata_schedule_425_e4057);
        let noise_metadata_schedule_425_e4059: f64 = (noise_metadata_schedule_425_e4047 * noise_metadata_schedule_425_e4058);
        (noise_metadata_schedule_425_e4059,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_425_e4061;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_426_e4077,) = {
    if (noise_variable_515 != 0.0) {
        let noise_metadata_schedule_426_e4065: f64 = (2.0 * noise_variable_58);
        let noise_metadata_schedule_426_e4067: f64 = (noise_metadata_schedule_426_e4065 * noise_variable_69);
        let noise_metadata_schedule_426_e4069: f64 = (noise_metadata_schedule_426_e4067 * noise_variable_59);
        let noise_metadata_schedule_426_e4071: f64 = (noise_metadata_schedule_426_e4069 * noise_variable_68);
        let noise_metadata_schedule_426_e4073: f64 = (noise_metadata_schedule_426_e4071 * noise_variable_65);
        let noise_metadata_schedule_426_e4075: f64 = (noise_metadata_schedule_426_e4073 * noise_variable_63);
        (noise_metadata_schedule_426_e4075,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_426_e4077;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_428_e4087,) = {
    if (noise_variable_515 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_428_e4087;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_429_e4098: f64 = if (((params.p35 > 0.0) && (params.p36 > 0.0)) && (noise_variable_236 < 0.0)) { 1.0 } else { 0.0 };
            noise_variable_519 = noise_metadata_schedule_429_e4098;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_430_e4110,) = {
    if (noise_variable_519 != 0.0) {
        let noise_metadata_schedule_430_e4103: f64 = (noise_variable_236 * noise_variable_67);
        let noise_metadata_schedule_430_e4104: f64 = (1.0 - noise_metadata_schedule_430_e4103);
        let noise_metadata_schedule_430_e4107: f64 = (1.0 - noise_variable_76);
        let noise_metadata_schedule_430_e4108: f64 = (noise_metadata_schedule_430_e4104).powf(noise_metadata_schedule_430_e4107);
        (noise_metadata_schedule_430_e4108,)
    } else {
        (noise_variable_77,)
    }
};
            noise_variable_77 = noise_metadata_schedule_430_e4110;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_431_e4116: f64 = (2.0 * noise_variable_77);
            let noise_metadata_schedule_431_e4117: f64 = (noise_variable_79 / noise_metadata_schedule_431_e4116);
            let noise_metadata_schedule_431_e4118: f64 = (1.0 - noise_metadata_schedule_431_e4117);
            let noise_metadata_schedule_431_e4119: f64 = (noise_variable_83 * noise_metadata_schedule_431_e4118);
            let noise_metadata_schedule_431_e4121: f64 = if noise_metadata_schedule_431_e4119 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_520 = noise_metadata_schedule_431_e4121;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_432_e4136,) = {
    if ((noise_variable_519 != 0.0) && (noise_variable_520 != 0.0)) {
        let noise_metadata_schedule_432_e4130: f64 = (2.0 * noise_variable_77);
        let noise_metadata_schedule_432_e4131: f64 = (noise_variable_79 / noise_metadata_schedule_432_e4130);
        let noise_metadata_schedule_432_e4132: f64 = (1.0 - noise_metadata_schedule_432_e4131);
        let noise_metadata_schedule_432_e4133: f64 = (noise_variable_83 * noise_metadata_schedule_432_e4132);
        let noise_metadata_schedule_432_e4134: f64 = (noise_metadata_schedule_432_e4133).exp();
        (noise_metadata_schedule_432_e4134,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_432_e4136;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_433_e4144,) = {
    if ((noise_variable_519 != 0.0) && (noise_variable_520 == 0.0)) {
        let noise_metadata_schedule_433_e4142: f64 = (params.p138).exp();
        (noise_metadata_schedule_433_e4142,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_433_e4144;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_434_e4165,) = {
    if ((noise_variable_519 != 0.0) && (noise_variable_520 == 0.0)) {
        let noise_metadata_schedule_434_e4156: f64 = (2.0 * noise_variable_77);
        let noise_metadata_schedule_434_e4157: f64 = (noise_variable_79 / noise_metadata_schedule_434_e4156);
        let noise_metadata_schedule_434_e4158: f64 = (1.0 - noise_metadata_schedule_434_e4157);
        let noise_metadata_schedule_434_e4159: f64 = (noise_variable_83 * noise_metadata_schedule_434_e4158);
        let noise_metadata_schedule_434_e4161: f64 = (noise_metadata_schedule_434_e4159 - params.p138);
        let noise_metadata_schedule_434_e4162: f64 = (1.0 + noise_metadata_schedule_434_e4161);
        let noise_metadata_schedule_434_e4163: f64 = (noise_variable_281 * noise_metadata_schedule_434_e4162);
        (noise_metadata_schedule_434_e4163,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_434_e4165;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_435_e4171,) = {
    if (noise_variable_519 != 0.0) {
        let noise_metadata_schedule_435_e4169: f64 = (noise_variable_236 * noise_variable_67);
        (noise_metadata_schedule_435_e4169,)
    } else {
        (noise_variable_263,)
    }
};
            noise_variable_263 = noise_metadata_schedule_435_e4171;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_436_e4215,) = {
    if (noise_variable_519 != 0.0) {
        let noise_metadata_schedule_436_e4175: f64 = (noise_variable_263 * noise_variable_263);
        let noise_metadata_schedule_436_e4177: f64 = (noise_metadata_schedule_436_e4175 + 1e-30);
        let noise_metadata_schedule_436_e4178: f64 = (noise_metadata_schedule_436_e4177).sqrt();
        let noise_metadata_schedule_436_e4180: f64 = (-2.0);
        let noise_metadata_schedule_436_e4182: f64 = (noise_metadata_schedule_436_e4180 - noise_variable_76);
        let noise_metadata_schedule_436_e4183: f64 = (noise_metadata_schedule_436_e4178).powf(noise_metadata_schedule_436_e4182);
        let noise_metadata_schedule_436_e4188: f64 = (noise_variable_76 * noise_variable_76);
        let noise_metadata_schedule_436_e4189: f64 = (1.0 - noise_metadata_schedule_436_e4188);
        let noise_metadata_schedule_436_e4192: f64 = (3.0 * noise_variable_263);
        let noise_metadata_schedule_436_e4195: f64 = (noise_variable_76 - 1.0);
        let noise_metadata_schedule_436_e4196: f64 = (noise_metadata_schedule_436_e4192 * noise_metadata_schedule_436_e4195);
        let noise_metadata_schedule_436_e4197: f64 = (noise_metadata_schedule_436_e4189 - noise_metadata_schedule_436_e4196);
        let noise_metadata_schedule_436_e4198: f64 = (noise_variable_76 * noise_metadata_schedule_436_e4197);
        let noise_metadata_schedule_436_e4201: f64 = (6.0 * noise_variable_263);
        let noise_metadata_schedule_436_e4203: f64 = (noise_metadata_schedule_436_e4201 * noise_variable_263);
        let noise_metadata_schedule_436_e4206: f64 = (noise_variable_76 - 1.0);
        let noise_metadata_schedule_436_e4208: f64 = (noise_metadata_schedule_436_e4206 + noise_variable_263);
        let noise_metadata_schedule_436_e4209: f64 = (noise_metadata_schedule_436_e4203 * noise_metadata_schedule_436_e4208);
        let noise_metadata_schedule_436_e4210: f64 = (noise_metadata_schedule_436_e4198 - noise_metadata_schedule_436_e4209);
        let noise_metadata_schedule_436_e4211: f64 = (noise_metadata_schedule_436_e4183 * noise_metadata_schedule_436_e4210);
        let noise_metadata_schedule_436_e4213: f64 = (noise_metadata_schedule_436_e4211 * 0.16666666666666666);
        (noise_metadata_schedule_436_e4213,)
    } else {
        (noise_variable_80,)
    }
};
            noise_variable_80 = noise_metadata_schedule_436_e4215;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_437_e4227,) = {
    if (noise_variable_519 != 0.0) {
        let noise_metadata_schedule_437_e4219: f64 = (noise_variable_236 * noise_variable_79);
        let noise_metadata_schedule_437_e4221: f64 = (noise_metadata_schedule_437_e4219 * noise_variable_83);
        let noise_metadata_schedule_437_e4224: f64 = (noise_variable_85 * noise_variable_80);
        let noise_metadata_schedule_437_e4225: f64 = (noise_metadata_schedule_437_e4221 / noise_metadata_schedule_437_e4224);
        (noise_metadata_schedule_437_e4225,)
    } else {
        (noise_variable_263,)
    }
};
            noise_variable_263 = noise_metadata_schedule_437_e4227;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_438_e4230: f64 = (-0.001);
            let noise_metadata_schedule_438_e4231: f64 = if noise_variable_263 < noise_metadata_schedule_438_e4230 { 1.0 } else { 0.0 };
            noise_variable_521 = noise_metadata_schedule_438_e4231;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_439_e4234: f64 = if noise_variable_263 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_522 = noise_metadata_schedule_439_e4234;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_440_e4243,) = {
    if (((noise_variable_519 != 0.0) && (noise_variable_521 != 0.0)) && (noise_variable_522 != 0.0)) {
        let noise_metadata_schedule_440_e4241: f64 = (noise_variable_263).exp();
        (noise_metadata_schedule_440_e4241,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_440_e4243;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_441_e4253,) = {
    if (((noise_variable_519 != 0.0) && (noise_variable_521 != 0.0)) && (noise_variable_522 == 0.0)) {
        let noise_metadata_schedule_441_e4251: f64 = (params.p138).exp();
        (noise_metadata_schedule_441_e4251,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_441_e4253;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_442_e4268,) = {
    if (((noise_variable_519 != 0.0) && (noise_variable_521 != 0.0)) && (noise_variable_522 == 0.0)) {
        let noise_metadata_schedule_442_e4264: f64 = (noise_variable_263 - params.p138);
        let noise_metadata_schedule_442_e4265: f64 = (1.0 + noise_metadata_schedule_442_e4264);
        let noise_metadata_schedule_442_e4266: f64 = (noise_variable_281 * noise_metadata_schedule_442_e4265);
        (noise_metadata_schedule_442_e4266,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_442_e4268;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_443_e4283,) = {
    if ((noise_variable_519 != 0.0) && (noise_variable_521 != 0.0)) {
        let noise_metadata_schedule_443_e4273: f64 = (-noise_variable_236);
        let noise_metadata_schedule_443_e4277: f64 = (1.0 - noise_variable_92);
        let noise_metadata_schedule_443_e4279: f64 = (noise_metadata_schedule_443_e4277 / noise_variable_263);
        let noise_metadata_schedule_443_e4280: f64 = (1.0 + noise_metadata_schedule_443_e4279);
        let noise_metadata_schedule_443_e4281: f64 = (noise_metadata_schedule_443_e4273 * noise_metadata_schedule_443_e4280);
        (noise_metadata_schedule_443_e4281,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_443_e4283;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_444_e4306,) = {
    if ((noise_variable_519 != 0.0) && (noise_variable_521 == 0.0)) {
        let noise_metadata_schedule_444_e4290: f64 = (noise_variable_236 * 0.5);
        let noise_metadata_schedule_444_e4292: f64 = (noise_metadata_schedule_444_e4290 * noise_variable_263);
        let noise_metadata_schedule_444_e4296: f64 = (noise_variable_263 * 0.3333333333333333);
        let noise_metadata_schedule_444_e4300: f64 = (0.25 * noise_variable_263);
        let noise_metadata_schedule_444_e4301: f64 = (1.0 + noise_metadata_schedule_444_e4300);
        let noise_metadata_schedule_444_e4302: f64 = (noise_metadata_schedule_444_e4296 * noise_metadata_schedule_444_e4301);
        let noise_metadata_schedule_444_e4303: f64 = (1.0 + noise_metadata_schedule_444_e4302);
        let noise_metadata_schedule_444_e4304: f64 = (noise_metadata_schedule_444_e4292 * noise_metadata_schedule_444_e4303);
        (noise_metadata_schedule_444_e4304,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_444_e4306;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_445_e4322,) = {
    if (noise_variable_519 != 0.0) {
        let noise_metadata_schedule_445_e4310: f64 = (2.0 * noise_variable_84);
        let noise_metadata_schedule_445_e4312: f64 = (noise_metadata_schedule_445_e4310 * noise_variable_81);
        let noise_metadata_schedule_445_e4314: f64 = (noise_metadata_schedule_445_e4312 * noise_variable_77);
        let noise_metadata_schedule_445_e4316: f64 = (noise_metadata_schedule_445_e4314 * noise_variable_78);
        let noise_metadata_schedule_445_e4318: f64 = (noise_metadata_schedule_445_e4316 * noise_variable_67);
        let noise_metadata_schedule_445_e4320: f64 = (noise_metadata_schedule_445_e4318 * noise_variable_89);
        (noise_metadata_schedule_445_e4320,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_445_e4322;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_447_e4332,) = {
    if (noise_variable_519 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_447_e4332;
        }
        if matches!(source_index, 11 | 12) {
            let noise_metadata_schedule_452_e4359: f64 = (2.0 * noise_variable_43);
            let noise_metadata_schedule_452_e4362: f64 = (noise_variable_254 - 1.0);
            let noise_metadata_schedule_452_e4363: f64 = (noise_metadata_schedule_452_e4359 * noise_metadata_schedule_452_e4362);
            let noise_metadata_schedule_452_e4368: f64 = (4.0 * noise_variable_43);
            let noise_metadata_schedule_452_e4370: f64 = (noise_metadata_schedule_452_e4368 / noise_variable_37);
            let noise_metadata_schedule_452_e4372: f64 = (noise_metadata_schedule_452_e4370 * noise_variable_254);
            let noise_metadata_schedule_452_e4373: f64 = (1.0 + noise_metadata_schedule_452_e4372);
            let noise_metadata_schedule_452_e4374: f64 = (noise_metadata_schedule_452_e4373).sqrt();
            let noise_metadata_schedule_452_e4375: f64 = (1.0 + noise_metadata_schedule_452_e4374);
            let noise_metadata_schedule_452_e4376: f64 = (noise_metadata_schedule_452_e4363 / noise_metadata_schedule_452_e4375);
            noise_variable_160 = noise_metadata_schedule_452_e4376;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_453_e4383: f64 = if ((params.p5 > 0.0) && (params.p32 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_523 = noise_metadata_schedule_453_e4383;
        }
        if matches!(source_index, 11 | 12) {
            let (noise_metadata_schedule_454_e4389,) = {
    if (noise_variable_523 != 0.0) {
        let noise_metadata_schedule_454_e4387: f64 = (noise_variable_160 * noise_variable_153);
        (noise_metadata_schedule_454_e4387,)
    } else {
        (noise_variable_160,)
    }
};
            noise_variable_160 = noise_metadata_schedule_454_e4389;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_455_e4414,) = {
    if (noise_variable_523 != 0.0) {
        let noise_metadata_schedule_455_e4393: f64 = (params.p32 * 2.0);
        let noise_metadata_schedule_455_e4395: f64 = (noise_metadata_schedule_455_e4393 * noise_variable_43);
        let noise_metadata_schedule_455_e4398: f64 = (noise_variable_255 - 1.0);
        let noise_metadata_schedule_455_e4399: f64 = (noise_metadata_schedule_455_e4395 * noise_metadata_schedule_455_e4398);
        let noise_metadata_schedule_455_e4404: f64 = (4.0 * noise_variable_43);
        let noise_metadata_schedule_455_e4406: f64 = (noise_metadata_schedule_455_e4404 / noise_variable_37);
        let noise_metadata_schedule_455_e4408: f64 = (noise_metadata_schedule_455_e4406 * noise_variable_255);
        let noise_metadata_schedule_455_e4409: f64 = (1.0 + noise_metadata_schedule_455_e4408);
        let noise_metadata_schedule_455_e4410: f64 = (noise_metadata_schedule_455_e4409).sqrt();
        let noise_metadata_schedule_455_e4411: f64 = (1.0 + noise_metadata_schedule_455_e4410);
        let noise_metadata_schedule_455_e4412: f64 = (noise_metadata_schedule_455_e4399 / noise_metadata_schedule_455_e4411);
        (noise_metadata_schedule_455_e4412,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_455_e4414;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_456_e4418,) = {
    if (noise_variable_523 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_168,)
    }
};
            noise_variable_168 = noise_metadata_schedule_456_e4418;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_457_e4421: f64 = if params.p5 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_524 = noise_metadata_schedule_457_e4421;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_458_e4431,) = {
    if ((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) {
        let noise_metadata_schedule_458_e4427: f64 = (params.p32 * noise_variable_43);
        let noise_metadata_schedule_458_e4429: f64 = (noise_metadata_schedule_458_e4427 * noise_variable_32);
        (noise_metadata_schedule_458_e4429,)
    } else {
        (noise_variable_277,)
    }
};
            noise_variable_277 = noise_metadata_schedule_458_e4431;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_459_e4444,) = {
    if ((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) {
        let noise_metadata_schedule_459_e4439: f64 = (noise_variable_277 * noise_variable_8);
        let noise_metadata_schedule_459_e4440: f64 = (noise_metadata_schedule_459_e4439).ln();
        let noise_metadata_schedule_459_e4441: f64 = (2.0 - noise_metadata_schedule_459_e4440);
        let noise_metadata_schedule_459_e4442: f64 = (noise_variable_6 * noise_metadata_schedule_459_e4441);
        (noise_metadata_schedule_459_e4442,)
    } else {
        (noise_variable_169,)
    }
};
            noise_variable_169 = noise_metadata_schedule_459_e4444;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_460_e4452,) = {
    if ((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) {
        let noise_metadata_schedule_460_e4450: f64 = (noise_variable_247 - noise_variable_169);
        (noise_metadata_schedule_460_e4450,)
    } else {
        (noise_variable_270,)
    }
};
            noise_variable_270 = noise_metadata_schedule_460_e4452;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_461_e4460,) = {
    if ((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) {
        let noise_metadata_schedule_461_e4458: f64 = (0.11 * 0.11);
        (noise_metadata_schedule_461_e4458,)
    } else {
        (noise_variable_267,)
    }
};
            noise_variable_267 = noise_metadata_schedule_461_e4460;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_462_e4468,) = {
    if ((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) {
        let noise_metadata_schedule_462_e4466: f64 = (noise_variable_270 * noise_variable_270);
        (noise_metadata_schedule_462_e4466,)
    } else {
        (noise_variable_268,)
    }
};
            noise_variable_268 = noise_metadata_schedule_462_e4468;
        }
        if matches!(source_index, 13 | 14) {
            let noise_metadata_schedule_463_e4471: f64 = if noise_variable_270 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_525 = noise_metadata_schedule_463_e4471;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_464_e4488,) = {
    if (((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) && (noise_variable_525 != 0.0)) {
        let noise_metadata_schedule_464_e4479: f64 = (0.5 * noise_variable_267);
        let noise_metadata_schedule_464_e4482: f64 = (noise_variable_268 + noise_variable_267);
        let noise_metadata_schedule_464_e4483: f64 = (noise_metadata_schedule_464_e4482).sqrt();
        let noise_metadata_schedule_464_e4485: f64 = (noise_metadata_schedule_464_e4483 - noise_variable_270);
        let noise_metadata_schedule_464_e4486: f64 = (noise_metadata_schedule_464_e4479 / noise_metadata_schedule_464_e4485);
        (noise_metadata_schedule_464_e4486,)
    } else {
        (noise_variable_170,)
    }
};
            noise_variable_170 = noise_metadata_schedule_464_e4488;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_465_e4504,) = {
    if (((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) && (noise_variable_525 == 0.0)) {
        let noise_metadata_schedule_465_e4498: f64 = (noise_variable_268 + noise_variable_267);
        let noise_metadata_schedule_465_e4499: f64 = (noise_metadata_schedule_465_e4498).sqrt();
        let noise_metadata_schedule_465_e4501: f64 = (noise_metadata_schedule_465_e4499 + noise_variable_270);
        let noise_metadata_schedule_465_e4502: f64 = (0.5 * noise_metadata_schedule_465_e4501);
        (noise_metadata_schedule_465_e4502,)
    } else {
        (noise_variable_170,)
    }
};
            noise_variable_170 = noise_metadata_schedule_465_e4504;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_466_e4520,) = {
    if ((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) {
        let noise_metadata_schedule_466_e4512: f64 = (noise_variable_167 + noise_variable_168);
        let noise_metadata_schedule_466_e4514: f64 = (noise_metadata_schedule_466_e4512 * noise_variable_32);
        let noise_metadata_schedule_466_e4515: f64 = (noise_variable_277 + noise_metadata_schedule_466_e4514);
        let noise_metadata_schedule_466_e4517: f64 = (noise_metadata_schedule_466_e4515 + noise_variable_170);
        let noise_metadata_schedule_466_e4518: f64 = (noise_variable_170 / noise_metadata_schedule_466_e4517);
        (noise_metadata_schedule_466_e4518,)
    } else {
        (noise_variable_171,)
    }
};
            noise_variable_171 = noise_metadata_schedule_466_e4520;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_470_e4548,) = {
    if ((noise_variable_523 != 0.0) && (noise_variable_524 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_171,)
    }
};
            noise_variable_171 = noise_metadata_schedule_470_e4548;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_471_e4554,) = {
    if (noise_variable_523 != 0.0) {
        let noise_metadata_schedule_471_e4552: f64 = (noise_variable_171 * noise_variable_167);
        (noise_metadata_schedule_471_e4552,)
    } else {
        (noise_variable_172,)
    }
};
            noise_variable_172 = noise_metadata_schedule_471_e4554;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_472_e4557: f64 = if params.p83 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_526 = noise_metadata_schedule_472_e4557;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_473_e4563,) = {
    if (noise_variable_526 != 0.0) {
        let noise_metadata_schedule_473_e4561: f64 = (noise_variable_240 + noise_variable_236);
        (noise_metadata_schedule_473_e4561,)
    } else {
        (noise_variable_328,)
    }
};
            noise_variable_328 = noise_metadata_schedule_473_e4563;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_474_e4569,) = {
    if (noise_variable_526 != 0.0) {
        let noise_metadata_schedule_474_e4567: f64 = (1e-6 * 1e-6);
        (noise_metadata_schedule_474_e4567,)
    } else {
        (noise_variable_267,)
    }
};
            noise_variable_267 = noise_metadata_schedule_474_e4569;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_475_e4581,) = {
    if (noise_variable_526 != 0.0) {
        let noise_metadata_schedule_475_e4572: f64 = (-1.0);
        let noise_metadata_schedule_475_e4574: f64 = (noise_metadata_schedule_475_e4572 * noise_variable_328);
        let noise_metadata_schedule_475_e4576: f64 = (-1.0);
        let noise_metadata_schedule_475_e4577: f64 = (noise_metadata_schedule_475_e4574 * noise_metadata_schedule_475_e4576);
        let noise_metadata_schedule_475_e4579: f64 = (noise_metadata_schedule_475_e4577 * noise_variable_328);
        (noise_metadata_schedule_475_e4579,)
    } else {
        (noise_variable_268,)
    }
};
            noise_variable_268 = noise_metadata_schedule_475_e4581;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_476_e4583: f64 = (-1.0);
            let noise_metadata_schedule_476_e4585: f64 = (noise_metadata_schedule_476_e4583 * noise_variable_328);
            let noise_metadata_schedule_476_e4587: f64 = if noise_metadata_schedule_476_e4585 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_527 = noise_metadata_schedule_476_e4587;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_477_e4605,) = {
    if ((noise_variable_526 != 0.0) && (noise_variable_527 != 0.0)) {
        let noise_metadata_schedule_477_e4593: f64 = (0.5 * noise_variable_267);
        let noise_metadata_schedule_477_e4596: f64 = (noise_variable_268 + noise_variable_267);
        let noise_metadata_schedule_477_e4597: f64 = (noise_metadata_schedule_477_e4596).sqrt();
        let noise_metadata_schedule_477_e4599: f64 = (-1.0);
        let noise_metadata_schedule_477_e4601: f64 = (noise_metadata_schedule_477_e4599 * noise_variable_328);
        let noise_metadata_schedule_477_e4602: f64 = (noise_metadata_schedule_477_e4597 - noise_metadata_schedule_477_e4601);
        let noise_metadata_schedule_477_e4603: f64 = (noise_metadata_schedule_477_e4593 / noise_metadata_schedule_477_e4602);
        (noise_metadata_schedule_477_e4603,)
    } else {
        (noise_variable_329,)
    }
};
            noise_variable_329 = noise_metadata_schedule_477_e4605;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_478_e4622,) = {
    if ((noise_variable_526 != 0.0) && (noise_variable_527 == 0.0)) {
        let noise_metadata_schedule_478_e4613: f64 = (noise_variable_268 + noise_variable_267);
        let noise_metadata_schedule_478_e4614: f64 = (noise_metadata_schedule_478_e4613).sqrt();
        let noise_metadata_schedule_478_e4616: f64 = (-1.0);
        let noise_metadata_schedule_478_e4618: f64 = (noise_metadata_schedule_478_e4616 * noise_variable_328);
        let noise_metadata_schedule_478_e4619: f64 = (noise_metadata_schedule_478_e4614 + noise_metadata_schedule_478_e4618);
        let noise_metadata_schedule_478_e4620: f64 = (0.5 * noise_metadata_schedule_478_e4619);
        (noise_metadata_schedule_478_e4620,)
    } else {
        (noise_variable_329,)
    }
};
            noise_variable_329 = noise_metadata_schedule_478_e4622;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_479_e4632,) = {
    if (noise_variable_526 != 0.0) {
        let noise_metadata_schedule_479_e4628: f64 = (noise_variable_324).powf(params.p81);
        let noise_metadata_schedule_479_e4629: f64 = (1.0 - noise_metadata_schedule_479_e4628);
        let noise_metadata_schedule_479_e4630: f64 = (1.0 / noise_metadata_schedule_479_e4629);
        (noise_metadata_schedule_479_e4630,)
    } else {
        (noise_variable_330,)
    }
};
            noise_variable_330 = noise_metadata_schedule_479_e4632;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_480_e4638,) = {
    if (noise_variable_526 != 0.0) {
        let noise_metadata_schedule_480_e4636: f64 = (noise_variable_324 * params.p80);
        (noise_metadata_schedule_480_e4636,)
    } else {
        (noise_variable_325,)
    }
};
            noise_variable_325 = noise_metadata_schedule_480_e4638;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_481_e4654,) = {
    if (noise_variable_526 != 0.0) {
        let noise_metadata_schedule_481_e4642: f64 = (noise_variable_330 * noise_variable_330);
        let noise_metadata_schedule_481_e4646: f64 = (params.p81 - 1.0);
        let noise_metadata_schedule_481_e4647: f64 = (noise_variable_324).powf(noise_metadata_schedule_481_e4646);
        let noise_metadata_schedule_481_e4648: f64 = (noise_metadata_schedule_481_e4642 * noise_metadata_schedule_481_e4647);
        let noise_metadata_schedule_481_e4650: f64 = (noise_metadata_schedule_481_e4648 * params.p81);
        let noise_metadata_schedule_481_e4652: f64 = (noise_metadata_schedule_481_e4650 / params.p80);
        (noise_metadata_schedule_481_e4652,)
    } else {
        (noise_variable_327,)
    }
};
            noise_variable_327 = noise_metadata_schedule_481_e4654;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_482_e4657: f64 = if noise_variable_329 < noise_variable_325 { 1.0 } else { 0.0 };
            noise_variable_528 = noise_metadata_schedule_482_e4657;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_483_e4671,) = {
    if ((noise_variable_526 != 0.0) && (noise_variable_528 != 0.0)) {
        let noise_metadata_schedule_483_e4665: f64 = (noise_variable_329 / params.p80);
        let noise_metadata_schedule_483_e4667: f64 = (noise_metadata_schedule_483_e4665).powf(params.p81);
        let noise_metadata_schedule_483_e4668: f64 = (1.0 - noise_metadata_schedule_483_e4667);
        let noise_metadata_schedule_483_e4669: f64 = (1.0 / noise_metadata_schedule_483_e4668);
        (noise_metadata_schedule_483_e4669,)
    } else {
        (noise_variable_326,)
    }
};
            noise_variable_326 = noise_metadata_schedule_483_e4671;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_484_e4684,) = {
    if ((noise_variable_526 != 0.0) && (noise_variable_528 == 0.0)) {
        let noise_metadata_schedule_484_e4679: f64 = (noise_variable_329 - noise_variable_325);
        let noise_metadata_schedule_484_e4681: f64 = (noise_metadata_schedule_484_e4679 * noise_variable_327);
        let noise_metadata_schedule_484_e4682: f64 = (noise_variable_330 + noise_metadata_schedule_484_e4681);
        (noise_metadata_schedule_484_e4682,)
    } else {
        (noise_variable_326,)
    }
};
            noise_variable_326 = noise_metadata_schedule_484_e4684;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_485_e4689,) = {
    if (noise_variable_526 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_326,)
    }
};
            noise_variable_326 = noise_metadata_schedule_485_e4689;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_486_e4692: f64 = (noise_variable_82 * noise_variable_326);
            noise_variable_82 = noise_metadata_schedule_486_e4692;
        }
        if matches!(source_index, 11 | 12) {
            let noise_metadata_schedule_487_e4695: f64 = (noise_variable_160 * noise_variable_326);
            noise_variable_160 = noise_metadata_schedule_487_e4695;
        }
        if matches!(source_index, 9 | 10) {
            let noise_metadata_schedule_488_e4698: f64 = (noise_variable_157 * noise_variable_326);
            noise_variable_157 = noise_metadata_schedule_488_e4698;
        }
        if matches!(source_index, 13 | 14) {
            let noise_metadata_schedule_489_e4701: f64 = (noise_variable_172 * noise_variable_326);
            noise_variable_172 = noise_metadata_schedule_489_e4701;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_490_e4705: f64 = (noise_variable_134 / noise_variable_41);
            let noise_metadata_schedule_490_e4706: f64 = (1.0 + noise_metadata_schedule_490_e4705);
            let noise_metadata_schedule_490_e4709: f64 = (noise_variable_141 / noise_variable_40);
            let noise_metadata_schedule_490_e4710: f64 = (noise_metadata_schedule_490_e4706 + noise_metadata_schedule_490_e4709);
            noise_variable_175 = noise_metadata_schedule_490_e4710;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_491_e4713: f64 = (0.1 * 0.1);
            noise_variable_267 = noise_metadata_schedule_491_e4713;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_492_e4716: f64 = (noise_variable_175 * noise_variable_175);
            noise_variable_268 = noise_metadata_schedule_492_e4716;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_493_e4719: f64 = if noise_variable_175 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_529 = noise_metadata_schedule_493_e4719;
        }
        if matches!(source_index, 1 | 5) {
            let (noise_metadata_schedule_494_e4732,) = {
    if (noise_variable_529 != 0.0) {
        let noise_metadata_schedule_494_e4723: f64 = (0.5 * noise_variable_267);
        let noise_metadata_schedule_494_e4726: f64 = (noise_variable_268 + noise_variable_267);
        let noise_metadata_schedule_494_e4727: f64 = (noise_metadata_schedule_494_e4726).sqrt();
        let noise_metadata_schedule_494_e4729: f64 = (noise_metadata_schedule_494_e4727 - noise_variable_175);
        let noise_metadata_schedule_494_e4730: f64 = (noise_metadata_schedule_494_e4723 / noise_metadata_schedule_494_e4729);
        (noise_metadata_schedule_494_e4730,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_494_e4732;
        }
        if matches!(source_index, 1 | 5) {
            let (noise_metadata_schedule_495_e4744,) = {
    if (noise_variable_529 == 0.0) {
        let noise_metadata_schedule_495_e4738: f64 = (noise_variable_268 + noise_variable_267);
        let noise_metadata_schedule_495_e4739: f64 = (noise_metadata_schedule_495_e4738).sqrt();
        let noise_metadata_schedule_495_e4741: f64 = (noise_metadata_schedule_495_e4739 + noise_variable_175);
        let noise_metadata_schedule_495_e4742: f64 = (0.5 * noise_metadata_schedule_495_e4741);
        (noise_metadata_schedule_495_e4742,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_495_e4744;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_496_e4750: f64 = (noise_variable_145 + noise_variable_146);
            let noise_metadata_schedule_496_e4751: f64 = (0.5 * noise_metadata_schedule_496_e4750);
            let noise_metadata_schedule_496_e4752: f64 = (1.0 + noise_metadata_schedule_496_e4751);
            let noise_metadata_schedule_496_e4753: f64 = (noise_variable_176 * noise_metadata_schedule_496_e4752);
            noise_variable_177 = noise_metadata_schedule_496_e4753;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_497_e4756: f64 = (noise_variable_29 / noise_variable_177);
            noise_variable_179 = noise_metadata_schedule_497_e4756;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_498_e4759: f64 = if noise_variable_179 < noise_variable_322 { 1.0 } else { 0.0 };
            noise_variable_530 = noise_metadata_schedule_498_e4759;
        }
        if matches!(source_index, 1 | 5) {
            let (noise_metadata_schedule_499_e4763,) = {
    if (noise_variable_530 != 0.0) {
        (noise_variable_322,)
    } else {
        (noise_variable_179,)
    }
};
            noise_variable_179 = noise_metadata_schedule_499_e4763;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_500_e4766: f64 = (3.0 * noise_variable_179);
            noise_variable_178 = noise_metadata_schedule_500_e4766;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_502_e4780: f64 = if noise_variable_152 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_531 = noise_metadata_schedule_502_e4780;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_503_e4783: f64 = if params.p38 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_532 = noise_metadata_schedule_503_e4783;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_504_e4786: f64 = if noise_variable_236 < params.p43 { 1.0 } else { 0.0 };
            noise_variable_533 = noise_metadata_schedule_504_e4786;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_505_e4788: f64 = (-noise_variable_152);
            let noise_metadata_schedule_505_e4790: f64 = (noise_metadata_schedule_505_e4788 / params.p41);
            let noise_metadata_schedule_505_e4792: f64 = if noise_metadata_schedule_505_e4790 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_534 = noise_metadata_schedule_505_e4792;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_506_e4806,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 != 0.0)) && (noise_variable_533 != 0.0)) && (noise_variable_534 != 0.0)) {
        let noise_metadata_schedule_506_e4801: f64 = (-noise_variable_152);
        let noise_metadata_schedule_506_e4803: f64 = (noise_metadata_schedule_506_e4801 / params.p41);
        let noise_metadata_schedule_506_e4804: f64 = (noise_metadata_schedule_506_e4803).exp();
        (noise_metadata_schedule_506_e4804,)
    } else {
        (noise_variable_314,)
    }
};
            noise_variable_314 = noise_metadata_schedule_506_e4806;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_507_e4818,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 != 0.0)) && (noise_variable_533 != 0.0)) && (noise_variable_534 == 0.0)) {
        let noise_metadata_schedule_507_e4816: f64 = (params.p138).exp();
        (noise_metadata_schedule_507_e4816,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_507_e4818;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_508_e4838,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 != 0.0)) && (noise_variable_533 != 0.0)) && (noise_variable_534 == 0.0)) {
        let noise_metadata_schedule_508_e4830: f64 = (-noise_variable_152);
        let noise_metadata_schedule_508_e4832: f64 = (noise_metadata_schedule_508_e4830 / params.p41);
        let noise_metadata_schedule_508_e4834: f64 = (noise_metadata_schedule_508_e4832 - params.p138);
        let noise_metadata_schedule_508_e4835: f64 = (1.0 + noise_metadata_schedule_508_e4834);
        let noise_metadata_schedule_508_e4836: f64 = (noise_variable_281 * noise_metadata_schedule_508_e4835);
        (noise_metadata_schedule_508_e4836,)
    } else {
        (noise_variable_314,)
    }
};
            noise_variable_314 = noise_metadata_schedule_508_e4838;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_509_e4850,) = {
    if (((noise_variable_531 != 0.0) && (noise_variable_532 != 0.0)) && (noise_variable_533 != 0.0)) {
        let noise_metadata_schedule_509_e4846: f64 = (params.p43 - noise_variable_236);
        let noise_metadata_schedule_509_e4848: f64 = (noise_metadata_schedule_509_e4846 * noise_variable_314);
        (noise_metadata_schedule_509_e4848,)
    } else {
        (noise_variable_315,)
    }
};
            noise_variable_315 = noise_metadata_schedule_509_e4850;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_510_e4852: f64 = (-noise_variable_316);
            let noise_metadata_schedule_510_e4855: f64 = (noise_variable_315).powf(params.p40);
            let noise_metadata_schedule_510_e4856: f64 = (noise_metadata_schedule_510_e4852 * noise_metadata_schedule_510_e4855);
            let noise_metadata_schedule_510_e4858: f64 = if noise_metadata_schedule_510_e4856 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_535 = noise_metadata_schedule_510_e4858;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_511_e4874,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 != 0.0)) && (noise_variable_533 != 0.0)) && (noise_variable_535 != 0.0)) {
        let noise_metadata_schedule_511_e4867: f64 = (-noise_variable_316);
        let noise_metadata_schedule_511_e4870: f64 = (noise_variable_315).powf(params.p40);
        let noise_metadata_schedule_511_e4871: f64 = (noise_metadata_schedule_511_e4867 * noise_metadata_schedule_511_e4870);
        let noise_metadata_schedule_511_e4872: f64 = (noise_metadata_schedule_511_e4871).exp();
        (noise_metadata_schedule_511_e4872,)
    } else {
        (noise_variable_319,)
    }
};
            noise_variable_319 = noise_metadata_schedule_511_e4874;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_512_e4886,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 != 0.0)) && (noise_variable_533 != 0.0)) && (noise_variable_535 == 0.0)) {
        let noise_metadata_schedule_512_e4884: f64 = (params.p138).exp();
        (noise_metadata_schedule_512_e4884,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_512_e4886;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_513_e4908,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 != 0.0)) && (noise_variable_533 != 0.0)) && (noise_variable_535 == 0.0)) {
        let noise_metadata_schedule_513_e4898: f64 = (-noise_variable_316);
        let noise_metadata_schedule_513_e4901: f64 = (noise_variable_315).powf(params.p40);
        let noise_metadata_schedule_513_e4902: f64 = (noise_metadata_schedule_513_e4898 * noise_metadata_schedule_513_e4901);
        let noise_metadata_schedule_513_e4904: f64 = (noise_metadata_schedule_513_e4902 - params.p138);
        let noise_metadata_schedule_513_e4905: f64 = (1.0 + noise_metadata_schedule_513_e4904);
        let noise_metadata_schedule_513_e4906: f64 = (noise_variable_281 * noise_metadata_schedule_513_e4905);
        (noise_metadata_schedule_513_e4906,)
    } else {
        (noise_variable_319,)
    }
};
            noise_variable_319 = noise_metadata_schedule_513_e4908;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_514_e4922,) = {
    if (((noise_variable_531 != 0.0) && (noise_variable_532 != 0.0)) && (noise_variable_533 != 0.0)) {
        let noise_metadata_schedule_514_e4916: f64 = (params.p39 / noise_variable_316);
        let noise_metadata_schedule_514_e4918: f64 = (noise_metadata_schedule_514_e4916 * noise_variable_315);
        let noise_metadata_schedule_514_e4920: f64 = (noise_metadata_schedule_514_e4918 * noise_variable_319);
        (noise_metadata_schedule_514_e4920,)
    } else {
        (noise_variable_199,)
    }
};
            noise_variable_199 = noise_metadata_schedule_514_e4922;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_515_e4925: f64 = if params.p38 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_536 = noise_metadata_schedule_515_e4925;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_516_e4928: f64 = if noise_variable_236 < noise_variable_16 { 1.0 } else { 0.0 };
            noise_variable_537 = noise_metadata_schedule_516_e4928;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_517_e4945,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) {
        let noise_metadata_schedule_517_e4939: f64 = (2.0 * params.p45);
        let noise_metadata_schedule_517_e4942: f64 = (params.p44 * params.p44);
        let noise_metadata_schedule_517_e4943: f64 = (noise_metadata_schedule_517_e4939 / noise_metadata_schedule_517_e4942);
        (noise_metadata_schedule_517_e4943,)
    } else {
        (noise_variable_188,)
    }
};
            noise_variable_188 = noise_metadata_schedule_517_e4945;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_518_e4960,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) {
        let noise_metadata_schedule_518_e4956: f64 = (noise_variable_16 - noise_variable_236);
        let noise_metadata_schedule_518_e4958: f64 = (noise_metadata_schedule_518_e4956 / noise_variable_202);
        (noise_metadata_schedule_518_e4958,)
    } else {
        (noise_variable_266,)
    }
};
            noise_variable_266 = noise_metadata_schedule_518_e4960;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_519_e4976,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) {
        let noise_metadata_schedule_519_e4971: f64 = (2.0 * noise_variable_266);
        let noise_metadata_schedule_519_e4973: f64 = (noise_metadata_schedule_519_e4971 / noise_variable_188);
        let noise_metadata_schedule_519_e4974: f64 = (noise_metadata_schedule_519_e4973).sqrt();
        (noise_metadata_schedule_519_e4974,)
    } else {
        (noise_variable_189,)
    }
};
            noise_variable_189 = noise_metadata_schedule_519_e4976;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_520_e4979: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_538 = noise_metadata_schedule_520_e4979;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_521_e4992,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_538 != 0.0)) {
        (params.p44,)
    } else {
        (noise_variable_190,)
    }
};
            noise_variable_190 = noise_metadata_schedule_521_e4992;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_522_e5010,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_538 == 0.0)) {
        let noise_metadata_schedule_522_e5007: f64 = (0.5 * noise_variable_118);
        let noise_metadata_schedule_522_e5008: f64 = (1.0 - noise_metadata_schedule_522_e5007);
        (noise_metadata_schedule_522_e5008,)
    } else {
        (noise_variable_119,)
    }
};
            noise_variable_119 = noise_metadata_schedule_522_e5010;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_523_e5028,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_538 == 0.0)) {
        let noise_metadata_schedule_523_e5024: f64 = (params.p44 * noise_variable_119);
        let noise_metadata_schedule_523_e5026: f64 = (noise_metadata_schedule_523_e5024 * noise_variable_119);
        (noise_metadata_schedule_523_e5026,)
    } else {
        (noise_variable_190,)
    }
};
            noise_variable_190 = noise_metadata_schedule_523_e5028;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_524_e5050,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) {
        let noise_metadata_schedule_524_e5039: f64 = (noise_variable_189 * noise_variable_190);
        let noise_metadata_schedule_524_e5042: f64 = (noise_variable_189 * noise_variable_189);
        let noise_metadata_schedule_524_e5045: f64 = (noise_variable_190 * noise_variable_190);
        let noise_metadata_schedule_524_e5046: f64 = (noise_metadata_schedule_524_e5042 + noise_metadata_schedule_524_e5045);
        let noise_metadata_schedule_524_e5047: f64 = (noise_metadata_schedule_524_e5046).sqrt();
        let noise_metadata_schedule_524_e5048: f64 = (noise_metadata_schedule_524_e5039 / noise_metadata_schedule_524_e5047);
        (noise_metadata_schedule_524_e5048,)
    } else {
        (noise_variable_191,)
    }
};
            noise_variable_191 = noise_metadata_schedule_524_e5050;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_525_e5065,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) {
        let noise_metadata_schedule_525_e5061: f64 = (noise_variable_16 - noise_variable_236);
        let noise_metadata_schedule_525_e5063: f64 = (noise_metadata_schedule_525_e5061 / noise_variable_191);
        (noise_metadata_schedule_525_e5063,)
    } else {
        (noise_variable_192,)
    }
};
            noise_variable_192 = noise_metadata_schedule_525_e5065;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_526_e5084,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) {
        let noise_metadata_schedule_526_e5077: f64 = (0.5 * noise_variable_191);
        let noise_metadata_schedule_526_e5079: f64 = (noise_metadata_schedule_526_e5077 * noise_variable_188);
        let noise_metadata_schedule_526_e5081: f64 = (noise_metadata_schedule_526_e5079 * noise_variable_202);
        let noise_metadata_schedule_526_e5082: f64 = (noise_variable_192 + noise_metadata_schedule_526_e5081);
        (noise_metadata_schedule_526_e5082,)
    } else {
        (noise_variable_193,)
    }
};
            noise_variable_193 = noise_metadata_schedule_526_e5084;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_527_e5087: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_539 = noise_metadata_schedule_527_e5087;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_528_e5100,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_539 != 0.0)) {
        (noise_variable_193,)
    } else {
        (noise_variable_194,)
    }
};
            noise_variable_194 = noise_metadata_schedule_528_e5100;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_529_e5124,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_539 == 0.0)) {
        let noise_metadata_schedule_529_e5115: f64 = (2.0 * params.p46);
        let noise_metadata_schedule_529_e5119: f64 = (2.0 * noise_variable_118);
        let noise_metadata_schedule_529_e5120: f64 = (1.0 + noise_metadata_schedule_529_e5119);
        let noise_metadata_schedule_529_e5121: f64 = (noise_metadata_schedule_529_e5115 * noise_metadata_schedule_529_e5120);
        let noise_metadata_schedule_529_e5122: f64 = (1.0 + noise_metadata_schedule_529_e5121);
        (noise_metadata_schedule_529_e5122,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_529_e5124;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_530_e5146,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_539 == 0.0)) {
        let noise_metadata_schedule_530_e5138: f64 = (1.0 + params.p46);
        let noise_metadata_schedule_530_e5142: f64 = (2.0 * params.p46);
        let noise_metadata_schedule_530_e5143: f64 = (1.0 + noise_metadata_schedule_530_e5142);
        let noise_metadata_schedule_530_e5144: f64 = (noise_metadata_schedule_530_e5138 / noise_metadata_schedule_530_e5143);
        (noise_metadata_schedule_530_e5144,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_530_e5146;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_531_e5174,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_539 == 0.0)) {
        let noise_metadata_schedule_531_e5161: f64 = (0.5 * noise_variable_191);
        let noise_metadata_schedule_531_e5163: f64 = (noise_metadata_schedule_531_e5161 * noise_variable_188);
        let noise_metadata_schedule_531_e5168: f64 = (params.p61 * noise_variable_195);
        let noise_metadata_schedule_531_e5169: f64 = (noise_variable_152 / noise_metadata_schedule_531_e5168);
        let noise_metadata_schedule_531_e5170: f64 = (noise_variable_196 - noise_metadata_schedule_531_e5169);
        let noise_metadata_schedule_531_e5171: f64 = (noise_metadata_schedule_531_e5163 * noise_metadata_schedule_531_e5170);
        let noise_metadata_schedule_531_e5172: f64 = (noise_variable_192 - noise_metadata_schedule_531_e5171);
        (noise_metadata_schedule_531_e5172,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_531_e5174;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_532_e5204,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_539 == 0.0)) {
        let noise_metadata_schedule_532_e5188: f64 = (noise_variable_197 - noise_variable_193);
        let noise_metadata_schedule_532_e5191: f64 = (noise_variable_197 - noise_variable_193);
        let noise_metadata_schedule_532_e5192: f64 = (noise_metadata_schedule_532_e5188 * noise_metadata_schedule_532_e5191);
        let noise_metadata_schedule_532_e5195: f64 = (0.1 * noise_variable_192);
        let noise_metadata_schedule_532_e5197: f64 = (noise_metadata_schedule_532_e5195 * noise_variable_192);
        let noise_metadata_schedule_532_e5199: f64 = (noise_metadata_schedule_532_e5197 * noise_variable_130);
        let noise_metadata_schedule_532_e5201: f64 = (noise_metadata_schedule_532_e5199 / params.p61);
        let noise_metadata_schedule_532_e5202: f64 = (noise_metadata_schedule_532_e5192 + noise_metadata_schedule_532_e5201);
        (noise_metadata_schedule_532_e5202,)
    } else {
        (noise_variable_266,)
    }
};
            noise_variable_266 = noise_metadata_schedule_532_e5204;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_533_e5225,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_539 == 0.0)) {
        let noise_metadata_schedule_533_e5219: f64 = (noise_variable_197 + noise_variable_193);
        let noise_metadata_schedule_533_e5221: f64 = (noise_variable_266).sqrt();
        let noise_metadata_schedule_533_e5222: f64 = (noise_metadata_schedule_533_e5219 + noise_metadata_schedule_533_e5221);
        let noise_metadata_schedule_533_e5223: f64 = (0.5 * noise_metadata_schedule_533_e5222);
        (noise_metadata_schedule_533_e5223,)
    } else {
        (noise_variable_194,)
    }
};
            noise_variable_194 = noise_metadata_schedule_533_e5225;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_534_e5240,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) {
        let noise_metadata_schedule_534_e5236: f64 = (noise_variable_194 - noise_variable_192);
        let noise_metadata_schedule_534_e5238: f64 = (noise_metadata_schedule_534_e5236 / noise_variable_194);
        (noise_metadata_schedule_534_e5238,)
    } else {
        (noise_variable_273,)
    }
};
            noise_variable_273 = noise_metadata_schedule_534_e5240;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_535_e5242: f64 = (noise_variable_273).abs();
            let noise_metadata_schedule_535_e5244: f64 = if noise_metadata_schedule_535_e5242 > 1e-7 { 1.0 } else { 0.0 };
            noise_variable_540 = noise_metadata_schedule_535_e5244;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_536_e5261,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_540 != 0.0)) {
        let noise_metadata_schedule_536_e5257: f64 = (0.5 * noise_variable_191);
        let noise_metadata_schedule_536_e5259: f64 = (noise_metadata_schedule_536_e5257 / noise_variable_273);
        (noise_metadata_schedule_536_e5259,)
    } else {
        (noise_variable_198,)
    }
};
            noise_variable_198 = noise_metadata_schedule_536_e5261;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_537_e5298,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_540 != 0.0)) {
        let noise_metadata_schedule_537_e5274: f64 = (noise_variable_0 / noise_variable_99);
        let noise_metadata_schedule_537_e5276: f64 = (noise_metadata_schedule_537_e5274 * noise_variable_194);
        let noise_metadata_schedule_537_e5278: f64 = (noise_metadata_schedule_537_e5276 * noise_variable_198);
        let noise_metadata_schedule_537_e5280: f64 = (-noise_variable_99);
        let noise_metadata_schedule_537_e5282: f64 = (noise_metadata_schedule_537_e5280 / noise_variable_194);
        let noise_metadata_schedule_537_e5283: f64 = (noise_metadata_schedule_537_e5282).exp();
        let noise_metadata_schedule_537_e5285: f64 = (-noise_variable_99);
        let noise_metadata_schedule_537_e5287: f64 = (noise_metadata_schedule_537_e5285 / noise_variable_194);
        let noise_metadata_schedule_537_e5291: f64 = (noise_variable_190 / noise_variable_198);
        let noise_metadata_schedule_537_e5292: f64 = (1.0 + noise_metadata_schedule_537_e5291);
        let noise_metadata_schedule_537_e5293: f64 = (noise_metadata_schedule_537_e5287 * noise_metadata_schedule_537_e5292);
        let noise_metadata_schedule_537_e5294: f64 = (noise_metadata_schedule_537_e5293).exp();
        let noise_metadata_schedule_537_e5295: f64 = (noise_metadata_schedule_537_e5283 - noise_metadata_schedule_537_e5294);
        let noise_metadata_schedule_537_e5296: f64 = (noise_metadata_schedule_537_e5278 * noise_metadata_schedule_537_e5295);
        (noise_metadata_schedule_537_e5296,)
    } else {
        (noise_variable_199,)
    }
};
            noise_variable_199 = noise_metadata_schedule_537_e5298;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_538_e5320,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 != 0.0)) && (noise_variable_537 != 0.0)) && (noise_variable_540 == 0.0)) {
        let noise_metadata_schedule_538_e5312: f64 = (noise_variable_0 * noise_variable_190);
        let noise_metadata_schedule_538_e5314: f64 = (-noise_variable_99);
        let noise_metadata_schedule_538_e5316: f64 = (noise_metadata_schedule_538_e5314 / noise_variable_194);
        let noise_metadata_schedule_538_e5317: f64 = (noise_metadata_schedule_538_e5316).exp();
        let noise_metadata_schedule_538_e5318: f64 = (noise_metadata_schedule_538_e5312 * noise_metadata_schedule_538_e5317);
        (noise_metadata_schedule_538_e5318,)
    } else {
        (noise_variable_199,)
    }
};
            noise_variable_199 = noise_metadata_schedule_538_e5320;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_539_e5323: f64 = if params.p38 == 3.0 { 1.0 } else { 0.0 };
            noise_variable_541 = noise_metadata_schedule_539_e5323;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_540_e5326: f64 = if noise_variable_236 < params.p43 { 1.0 } else { 0.0 };
            noise_variable_542 = noise_metadata_schedule_540_e5326;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_541_e5354,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 == 0.0)) && (noise_variable_541 != 0.0)) && (noise_variable_542 != 0.0)) {
        let noise_metadata_schedule_541_e5340: f64 = (params.p43 - noise_variable_236);
        let noise_metadata_schedule_541_e5342: f64 = (noise_metadata_schedule_541_e5340).powf(params.p40);
        let noise_metadata_schedule_541_e5347: f64 = (params.p47 + noise_variable_152);
        let noise_metadata_schedule_541_e5348: f64 = (noise_variable_152 / noise_metadata_schedule_541_e5347);
        let noise_metadata_schedule_541_e5349: f64 = (1.0 - noise_metadata_schedule_541_e5348);
        let noise_metadata_schedule_541_e5351: f64 = (noise_metadata_schedule_541_e5349).powf(params.p48);
        let noise_metadata_schedule_541_e5352: f64 = (noise_metadata_schedule_541_e5342 * noise_metadata_schedule_541_e5351);
        (noise_metadata_schedule_541_e5352,)
    } else {
        (noise_variable_203,)
    }
};
            noise_variable_203 = noise_metadata_schedule_541_e5354;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_542_e5357: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_543 = noise_metadata_schedule_542_e5357;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_543_e5373,) = {
    if ((((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 == 0.0)) && (noise_variable_541 != 0.0)) && (noise_variable_542 != 0.0)) && (noise_variable_543 != 0.0)) {
        (noise_variable_203,)
    } else {
        (noise_variable_204,)
    }
};
            noise_variable_204 = noise_metadata_schedule_543_e5373;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_544_e5394,) = {
    if ((((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 == 0.0)) && (noise_variable_541 != 0.0)) && (noise_variable_542 != 0.0)) && (noise_variable_543 == 0.0)) {
        let noise_metadata_schedule_544_e5390: f64 = (noise_variable_152 - params.p51);
        let noise_metadata_schedule_544_e5392: f64 = (noise_metadata_schedule_544_e5390 / params.p47);
        (noise_metadata_schedule_544_e5392,)
    } else {
        (noise_variable_205,)
    }
};
            noise_variable_205 = noise_metadata_schedule_544_e5394;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_545_e5415,) = {
    if ((((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 == 0.0)) && (noise_variable_541 != 0.0)) && (noise_variable_542 != 0.0)) && (noise_variable_543 == 0.0)) {
        let noise_metadata_schedule_545_e5411: f64 = (noise_variable_205 - 1.0);
        let noise_metadata_schedule_545_e5413: f64 = (noise_metadata_schedule_545_e5411 / params.p50);
        (noise_metadata_schedule_545_e5413,)
    } else {
        (noise_variable_265,)
    }
};
            noise_variable_265 = noise_metadata_schedule_545_e5415;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_546_e5418: f64 = if noise_variable_205 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_544 = noise_metadata_schedule_546_e5418;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_547_e5445,) = {
    if (((((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 == 0.0)) && (noise_variable_541 != 0.0)) && (noise_variable_542 != 0.0)) && (noise_variable_543 == 0.0)) && (noise_variable_544 != 0.0)) {
        let noise_metadata_schedule_547_e5439: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_547_e5440: f64 = (1.0 + noise_metadata_schedule_547_e5439);
        let noise_metadata_schedule_547_e5441: f64 = (noise_metadata_schedule_547_e5440).ln();
        let noise_metadata_schedule_547_e5442: f64 = (params.p50 * noise_metadata_schedule_547_e5441);
        let noise_metadata_schedule_547_e5443: f64 = (1.0 + noise_metadata_schedule_547_e5442);
        (noise_metadata_schedule_547_e5443,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_547_e5445;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_548_e5474,) = {
    if (((((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 == 0.0)) && (noise_variable_541 != 0.0)) && (noise_variable_542 != 0.0)) && (noise_variable_543 == 0.0)) && (noise_variable_544 == 0.0)) {
        let noise_metadata_schedule_548_e5467: f64 = (-noise_variable_265);
        let noise_metadata_schedule_548_e5468: f64 = (noise_metadata_schedule_548_e5467).exp();
        let noise_metadata_schedule_548_e5469: f64 = (1.0 + noise_metadata_schedule_548_e5468);
        let noise_metadata_schedule_548_e5470: f64 = (noise_metadata_schedule_548_e5469).ln();
        let noise_metadata_schedule_548_e5471: f64 = (params.p50 * noise_metadata_schedule_548_e5470);
        let noise_metadata_schedule_548_e5472: f64 = (noise_variable_205 + noise_metadata_schedule_548_e5471);
        (noise_metadata_schedule_548_e5472,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_548_e5474;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_549_e5495,) = {
    if ((((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 == 0.0)) && (noise_variable_541 != 0.0)) && (noise_variable_542 != 0.0)) && (noise_variable_543 == 0.0)) {
        let noise_metadata_schedule_549_e5492: f64 = (noise_variable_206).powf(params.p49);
        let noise_metadata_schedule_549_e5493: f64 = (noise_variable_203 * noise_metadata_schedule_549_e5492);
        (noise_metadata_schedule_549_e5493,)
    } else {
        (noise_variable_204,)
    }
};
            noise_variable_204 = noise_metadata_schedule_549_e5495;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_550_e5497: f64 = (-noise_variable_316);
            let noise_metadata_schedule_550_e5499: f64 = (noise_metadata_schedule_550_e5497 * noise_variable_204);
            let noise_metadata_schedule_550_e5501: f64 = if noise_metadata_schedule_550_e5499 < params.p138 { 1.0 } else { 0.0 };
            noise_variable_545 = noise_metadata_schedule_550_e5501;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_551_e5521,) = {
    if ((((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 == 0.0)) && (noise_variable_541 != 0.0)) && (noise_variable_542 != 0.0)) && (noise_variable_545 != 0.0)) {
        let noise_metadata_schedule_551_e5516: f64 = (-noise_variable_316);
        let noise_metadata_schedule_551_e5518: f64 = (noise_metadata_schedule_551_e5516 * noise_variable_204);
        let noise_metadata_schedule_551_e5519: f64 = (noise_metadata_schedule_551_e5518).exp();
        (noise_metadata_schedule_551_e5519,)
    } else {
        (noise_variable_319,)
    }
};
            noise_variable_319 = noise_metadata_schedule_551_e5521;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_552_e5539,) = {
    if ((((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 == 0.0)) && (noise_variable_541 != 0.0)) && (noise_variable_542 != 0.0)) && (noise_variable_545 == 0.0)) {
        let noise_metadata_schedule_552_e5537: f64 = (params.p138).exp();
        (noise_metadata_schedule_552_e5537,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_552_e5539;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_553_e5565,) = {
    if ((((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 == 0.0)) && (noise_variable_541 != 0.0)) && (noise_variable_542 != 0.0)) && (noise_variable_545 == 0.0)) {
        let noise_metadata_schedule_553_e5557: f64 = (-noise_variable_316);
        let noise_metadata_schedule_553_e5559: f64 = (noise_metadata_schedule_553_e5557 * noise_variable_204);
        let noise_metadata_schedule_553_e5561: f64 = (noise_metadata_schedule_553_e5559 - params.p138);
        let noise_metadata_schedule_553_e5562: f64 = (1.0 + noise_metadata_schedule_553_e5561);
        let noise_metadata_schedule_553_e5563: f64 = (noise_variable_281 * noise_metadata_schedule_553_e5562);
        (noise_metadata_schedule_553_e5563,)
    } else {
        (noise_variable_319,)
    }
};
            noise_variable_319 = noise_metadata_schedule_553_e5565;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_554_e5587,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_532 == 0.0)) && (noise_variable_536 == 0.0)) && (noise_variable_541 != 0.0)) && (noise_variable_542 != 0.0)) {
        let noise_metadata_schedule_554_e5579: f64 = (params.p39 / noise_variable_316);
        let noise_metadata_schedule_554_e5582: f64 = (params.p43 - noise_variable_236);
        let noise_metadata_schedule_554_e5583: f64 = (noise_metadata_schedule_554_e5579 * noise_metadata_schedule_554_e5582);
        let noise_metadata_schedule_554_e5585: f64 = (noise_metadata_schedule_554_e5583 * noise_variable_319);
        (noise_metadata_schedule_554_e5585,)
    } else {
        (noise_variable_199,)
    }
};
            noise_variable_199 = noise_metadata_schedule_554_e5587;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_555_e5590: f64 = if noise_variable_199 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_546 = noise_metadata_schedule_555_e5590;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_556_e5593: f64 = if params.p52 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_547 = noise_metadata_schedule_556_e5593;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_557_e5619,) = {
    if (((noise_variable_531 != 0.0) && (noise_variable_546 != 0.0)) && (noise_variable_547 != 0.0)) {
        let noise_metadata_schedule_557_e5603: f64 = (noise_variable_30 + noise_variable_178);
        let noise_metadata_schedule_557_e5604: f64 = (noise_variable_152 * noise_metadata_schedule_557_e5603);
        let noise_metadata_schedule_557_e5605: f64 = (noise_variable_6 / noise_metadata_schedule_557_e5604);
        let noise_metadata_schedule_557_e5608: f64 = (noise_variable_149 / noise_variable_35);
        let noise_metadata_schedule_557_e5610: f64 = (noise_metadata_schedule_557_e5608 * noise_variable_42);
        let noise_metadata_schedule_557_e5611: f64 = (noise_metadata_schedule_557_e5605 + noise_metadata_schedule_557_e5610);
        let noise_metadata_schedule_557_e5615: f64 = (noise_variable_30 + noise_variable_178);
        let noise_metadata_schedule_557_e5616: f64 = (noise_variable_28 / noise_metadata_schedule_557_e5615);
        let noise_metadata_schedule_557_e5617: f64 = (noise_metadata_schedule_557_e5611 + noise_metadata_schedule_557_e5616);
        (noise_metadata_schedule_557_e5617,)
    } else {
        (noise_variable_200,)
    }
};
            noise_variable_200 = noise_metadata_schedule_557_e5619;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_558_e5622: f64 = if params.p38 == 3.0 { 1.0 } else { 0.0 };
            noise_variable_548 = noise_metadata_schedule_558_e5622;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_559_e5636,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_546 != 0.0)) && (noise_variable_547 != 0.0)) && (noise_variable_548 != 0.0)) {
        let noise_metadata_schedule_559_e5632: f64 = (noise_variable_199 - noise_variable_200);
        let noise_metadata_schedule_559_e5634: f64 = (noise_metadata_schedule_559_e5632 / 1e-6);
        (noise_metadata_schedule_559_e5634,)
    } else {
        (noise_variable_265,)
    }
};
            noise_variable_265 = noise_metadata_schedule_559_e5636;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_560_e5639: f64 = if noise_variable_199 < noise_variable_200 { 1.0 } else { 0.0 };
            noise_variable_549 = noise_metadata_schedule_560_e5639;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_561_e5659,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_546 != 0.0)) && (noise_variable_547 != 0.0)) && (noise_variable_548 != 0.0)) && (noise_variable_549 != 0.0)) {
        let noise_metadata_schedule_561_e5653: f64 = (noise_variable_265).exp();
        let noise_metadata_schedule_561_e5654: f64 = (1.0 + noise_metadata_schedule_561_e5653);
        let noise_metadata_schedule_561_e5655: f64 = (noise_metadata_schedule_561_e5654).ln();
        let noise_metadata_schedule_561_e5656: f64 = (1e-6 * noise_metadata_schedule_561_e5655);
        let noise_metadata_schedule_561_e5657: f64 = (noise_variable_199 - noise_metadata_schedule_561_e5656);
        (noise_metadata_schedule_561_e5657,)
    } else {
        (noise_variable_199,)
    }
};
            noise_variable_199 = noise_metadata_schedule_561_e5659;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_562_e5681,) = {
    if (((((noise_variable_531 != 0.0) && (noise_variable_546 != 0.0)) && (noise_variable_547 != 0.0)) && (noise_variable_548 != 0.0)) && (noise_variable_549 == 0.0)) {
        let noise_metadata_schedule_562_e5674: f64 = (-noise_variable_265);
        let noise_metadata_schedule_562_e5675: f64 = (noise_metadata_schedule_562_e5674).exp();
        let noise_metadata_schedule_562_e5676: f64 = (1.0 + noise_metadata_schedule_562_e5675);
        let noise_metadata_schedule_562_e5677: f64 = (noise_metadata_schedule_562_e5676).ln();
        let noise_metadata_schedule_562_e5678: f64 = (1e-6 * noise_metadata_schedule_562_e5677);
        let noise_metadata_schedule_562_e5679: f64 = (noise_variable_200 - noise_metadata_schedule_562_e5678);
        (noise_metadata_schedule_562_e5679,)
    } else {
        (noise_variable_199,)
    }
};
            noise_variable_199 = noise_metadata_schedule_562_e5681;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_563_e5693,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_546 != 0.0)) && (noise_variable_547 != 0.0)) && (noise_variable_548 != 0.0)) {
        let noise_metadata_schedule_563_e5691: f64 = (noise_variable_152 * noise_variable_199);
        (noise_metadata_schedule_563_e5691,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_563_e5693;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_564_e5712,) = {
    if ((((noise_variable_531 != 0.0) && (noise_variable_546 != 0.0)) && (noise_variable_547 != 0.0)) && (noise_variable_548 == 0.0)) {
        let noise_metadata_schedule_564_e5704: f64 = (noise_variable_152 * noise_variable_199);
        let noise_metadata_schedule_564_e5706: f64 = (noise_metadata_schedule_564_e5704 * noise_variable_200);
        let noise_metadata_schedule_564_e5709: f64 = (noise_variable_199 + noise_variable_200);
        let noise_metadata_schedule_564_e5710: f64 = (noise_metadata_schedule_564_e5706 / noise_metadata_schedule_564_e5709);
        (noise_metadata_schedule_564_e5710,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_564_e5712;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_565_e5723,) = {
    if (((noise_variable_531 != 0.0) && (noise_variable_546 != 0.0)) && (noise_variable_547 == 0.0)) {
        let noise_metadata_schedule_565_e5721: f64 = (noise_variable_152 * noise_variable_199);
        (noise_metadata_schedule_565_e5721,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_565_e5723;
        }
        if matches!(source_index, 3 | 4 | 5 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24) {
            let noise_metadata_schedule_656_e6673: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_656_e6675: f64 = (noise_metadata_schedule_656_e6673 * noise_variable_2);
            noise_variable_287 = noise_metadata_schedule_656_e6675;
        }
        if matches!(source_index, 3) {
            let noise_metadata_schedule_657_e6678: f64 = (noise_variable_287 / noise_variable_28);
            noise_variable_288 = noise_metadata_schedule_657_e6678;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_658_e6681: f64 = (noise_variable_287 / noise_variable_30);
            noise_variable_289 = noise_metadata_schedule_658_e6681;
        }
        if matches!(source_index, 17 | 20 | 22 | 24) {
            let noise_metadata_schedule_659_e6684: f64 = (noise_variable_287 * noise_variable_104);
            noise_variable_290 = noise_metadata_schedule_659_e6684;
        }
        if matches!(source_index, 18 | 21) {
            let noise_metadata_schedule_660_e6687: f64 = (noise_variable_287 * noise_variable_105);
            noise_variable_291 = noise_metadata_schedule_660_e6687;
        }
        if matches!(source_index, 19 | 23) {
            let noise_metadata_schedule_661_e6690: f64 = (noise_variable_287 * noise_variable_106);
            noise_variable_292 = noise_metadata_schedule_661_e6690;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_662_e6693: f64 = (noise_variable_287 / noise_variable_178);
            let noise_metadata_schedule_662_e6696: f64 = (4.0 * noise_variable_253);
            let noise_metadata_schedule_662_e6698: f64 = (noise_metadata_schedule_662_e6696 + 5.0);
            let noise_metadata_schedule_662_e6699: f64 = (noise_metadata_schedule_662_e6693 * noise_metadata_schedule_662_e6698);
            let noise_metadata_schedule_662_e6701: f64 = (noise_metadata_schedule_662_e6699 * 0.3333333333333333);
            noise_variable_293 = noise_metadata_schedule_662_e6701;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_663_e6704: f64 = (noise_variable_151 + noise_variable_150);
            let noise_metadata_schedule_663_e6706: f64 = (noise_metadata_schedule_663_e6704 / noise_variable_149);
            noise_variable_309 = noise_metadata_schedule_663_e6706;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_664_e6709: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_664_e6711: f64 = (noise_variable_309).abs();
            let noise_metadata_schedule_664_e6712: f64 = (noise_metadata_schedule_664_e6709 * noise_metadata_schedule_664_e6711);
            noise_variable_294 = noise_metadata_schedule_664_e6712;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_665_e6715: f64 = if params.p129 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_569 = noise_metadata_schedule_665_e6715;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_666_e6722,) = {
    if (noise_variable_569 != 0.0) {
        let noise_metadata_schedule_666_e6719: f64 = (noise_variable_201 / noise_variable_309);
        let noise_metadata_schedule_666_e6720: f64 = (noise_metadata_schedule_666_e6719).abs();
        (noise_metadata_schedule_666_e6720,)
    } else {
        (noise_variable_310,)
    }
};
            noise_variable_310 = noise_metadata_schedule_666_e6722;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_667_e6727,) = {
    if (noise_variable_569 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_310,)
    }
};
            noise_variable_310 = noise_metadata_schedule_667_e6727;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_668_e6730: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_668_e6732: f64 = (noise_metadata_schedule_668_e6730 * noise_variable_201);
            let noise_metadata_schedule_668_e6735: f64 = (noise_variable_310 + 1.0);
            let noise_metadata_schedule_668_e6736: f64 = (noise_metadata_schedule_668_e6732 * noise_metadata_schedule_668_e6735);
            noise_variable_306 = noise_metadata_schedule_668_e6736;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_677_e6788: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_677_e6791: f64 = (noise_variable_154 + noise_variable_156);
            let noise_metadata_schedule_677_e6793: f64 = (noise_metadata_schedule_677_e6791 - noise_variable_57);
            let noise_metadata_schedule_677_e6795: f64 = (noise_metadata_schedule_677_e6793 + noise_variable_334);
            let noise_metadata_schedule_677_e6797: f64 = (noise_metadata_schedule_677_e6795 + noise_variable_333);
            let noise_metadata_schedule_677_e6798: f64 = (noise_metadata_schedule_677_e6797).abs();
            let noise_metadata_schedule_677_e6799: f64 = (noise_metadata_schedule_677_e6788 * noise_metadata_schedule_677_e6798);
            noise_variable_295 = noise_metadata_schedule_677_e6799;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_678_e6802: f64 = (noise_variable_154 + noise_variable_155);
            noise_variable_307 = noise_metadata_schedule_678_e6802;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_679_e6805: f64 = (noise_variable_307).abs();
            let noise_metadata_schedule_679_e6807: f64 = (noise_metadata_schedule_679_e6805).powf(params.p125);
            let noise_metadata_schedule_679_e6808: f64 = (params.p127 * noise_metadata_schedule_679_e6807);
            noise_variable_296 = noise_metadata_schedule_679_e6808;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_680_e6811: f64 = if noise_variable_307 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_573 = noise_metadata_schedule_680_e6811;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_681_e6816,) = {
    if (noise_variable_573 != 0.0) {
        let noise_metadata_schedule_681_e6814: f64 = (-noise_variable_296);
        (noise_metadata_schedule_681_e6814,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_681_e6816;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_682_e6819: f64 = (noise_variable_156 + noise_variable_158);
            let noise_metadata_schedule_682_e6821: f64 = (noise_metadata_schedule_682_e6819 + noise_variable_159);
            noise_variable_308 = noise_metadata_schedule_682_e6821;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_683_e6824: f64 = (noise_variable_308).abs();
            let noise_metadata_schedule_683_e6826: f64 = (noise_metadata_schedule_683_e6824).powf(params.p126);
            let noise_metadata_schedule_683_e6827: f64 = (params.p128 * noise_metadata_schedule_683_e6826);
            noise_variable_297 = noise_metadata_schedule_683_e6827;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_684_e6830: f64 = if noise_variable_308 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_574 = noise_metadata_schedule_684_e6830;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_685_e6835,) = {
    if (noise_variable_574 != 0.0) {
        let noise_metadata_schedule_685_e6833: f64 = (-noise_variable_297);
        (noise_metadata_schedule_685_e6833,)
    } else {
        (noise_variable_297,)
    }
};
            noise_variable_297 = noise_metadata_schedule_685_e6835;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_686_e6838: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_686_e6841: f64 = (noise_variable_155 + noise_variable_158);
            let noise_metadata_schedule_686_e6843: f64 = (noise_metadata_schedule_686_e6841 + noise_variable_159);
            let noise_metadata_schedule_686_e6844: f64 = (noise_metadata_schedule_686_e6843).abs();
            let noise_metadata_schedule_686_e6845: f64 = (noise_metadata_schedule_686_e6838 * noise_metadata_schedule_686_e6844);
            noise_variable_298 = noise_metadata_schedule_686_e6845;
        }
        if matches!(source_index, 9) {
            let noise_metadata_schedule_687_e6848: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_687_e6850: f64 = (noise_variable_157).abs();
            let noise_metadata_schedule_687_e6851: f64 = (noise_metadata_schedule_687_e6848 * noise_metadata_schedule_687_e6850);
            noise_variable_299 = noise_metadata_schedule_687_e6851;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_688_e6854: f64 = (noise_variable_157).abs();
            let noise_metadata_schedule_688_e6856: f64 = (noise_metadata_schedule_688_e6854).powf(params.p125);
            let noise_metadata_schedule_688_e6857: f64 = (params.p127 * noise_metadata_schedule_688_e6856);
            noise_variable_300 = noise_metadata_schedule_688_e6857;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_689_e6860: f64 = if noise_variable_157 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_575 = noise_metadata_schedule_689_e6860;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_690_e6865,) = {
    if (noise_variable_575 != 0.0) {
        let noise_metadata_schedule_690_e6863: f64 = (-noise_variable_300);
        (noise_metadata_schedule_690_e6863,)
    } else {
        (noise_variable_300,)
    }
};
            noise_variable_300 = noise_metadata_schedule_690_e6865;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_691_e6868: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_691_e6870: f64 = (noise_variable_82).abs();
            let noise_metadata_schedule_691_e6871: f64 = (noise_metadata_schedule_691_e6868 * noise_metadata_schedule_691_e6870);
            noise_variable_301 = noise_metadata_schedule_691_e6871;
        }
        if matches!(source_index, 11) {
            let noise_metadata_schedule_692_e6874: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_692_e6876: f64 = (noise_variable_160).abs();
            let noise_metadata_schedule_692_e6877: f64 = (noise_metadata_schedule_692_e6874 * noise_metadata_schedule_692_e6876);
            noise_variable_302 = noise_metadata_schedule_692_e6877;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_693_e6882: f64 = (params.p5 * params.p32);
            let noise_metadata_schedule_693_e6883: f64 = (1.0 - noise_metadata_schedule_693_e6882);
            let noise_metadata_schedule_693_e6884: f64 = (params.p127 * noise_metadata_schedule_693_e6883);
            let noise_metadata_schedule_693_e6886: f64 = (noise_variable_160).abs();
            let noise_metadata_schedule_693_e6890: f64 = (params.p5 * params.p32);
            let noise_metadata_schedule_693_e6891: f64 = (1.0 - noise_metadata_schedule_693_e6890);
            let noise_metadata_schedule_693_e6892: f64 = (noise_metadata_schedule_693_e6886 / noise_metadata_schedule_693_e6891);
            let noise_metadata_schedule_693_e6894: f64 = (noise_metadata_schedule_693_e6892).powf(params.p125);
            let noise_metadata_schedule_693_e6895: f64 = (noise_metadata_schedule_693_e6884 * noise_metadata_schedule_693_e6894);
            noise_variable_304 = noise_metadata_schedule_693_e6895;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_694_e6898: f64 = if noise_variable_160 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_576 = noise_metadata_schedule_694_e6898;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_695_e6903,) = {
    if (noise_variable_576 != 0.0) {
        let noise_metadata_schedule_695_e6901: f64 = (-noise_variable_304);
        (noise_metadata_schedule_695_e6901,)
    } else {
        (noise_variable_304,)
    }
};
            noise_variable_304 = noise_metadata_schedule_695_e6903;
        }
        if matches!(source_index, 13) {
            let noise_metadata_schedule_696_e6906: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_696_e6908: f64 = (noise_variable_172).abs();
            let noise_metadata_schedule_696_e6909: f64 = (noise_metadata_schedule_696_e6906 * noise_metadata_schedule_696_e6908);
            let noise_metadata_schedule_696_e6911: f64 = (noise_metadata_schedule_696_e6909 * params.p5);
            noise_variable_303 = noise_metadata_schedule_696_e6911;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_697_e6914: f64 = if params.p32 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_577 = noise_metadata_schedule_697_e6914;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_698_e6918,) = {
    if (noise_variable_577 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_305,)
    }
};
            noise_variable_305 = noise_metadata_schedule_698_e6918;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_699_e6934,) = {
    if (noise_variable_577 == 0.0) {
        let noise_metadata_schedule_699_e6923: f64 = (params.p127 * params.p5);
        let noise_metadata_schedule_699_e6925: f64 = (noise_metadata_schedule_699_e6923 * params.p32);
        let noise_metadata_schedule_699_e6927: f64 = (noise_variable_172).abs();
        let noise_metadata_schedule_699_e6929: f64 = (noise_metadata_schedule_699_e6927 / params.p32);
        let noise_metadata_schedule_699_e6931: f64 = (noise_metadata_schedule_699_e6929).powf(params.p125);
        let noise_metadata_schedule_699_e6932: f64 = (noise_metadata_schedule_699_e6925 * noise_metadata_schedule_699_e6931);
        (noise_metadata_schedule_699_e6932,)
    } else {
        (noise_variable_305,)
    }
};
            noise_variable_305 = noise_metadata_schedule_699_e6934;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_700_e6937: f64 = if noise_variable_172 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_578 = noise_metadata_schedule_700_e6937;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_701_e6942,) = {
    if (noise_variable_578 != 0.0) {
        let noise_metadata_schedule_701_e6940: f64 = (-noise_variable_305);
        (noise_metadata_schedule_701_e6940,)
    } else {
        (noise_variable_305,)
    }
};
            noise_variable_305 = noise_metadata_schedule_701_e6942;
        }
        match source_index {
            0 => {
                let noise_0_psd_e7995: f64 = 1.0;
                let noise_0_psd_e361: f64 = (noise_variable_294 * params.p1);
                let noise_0_psd_e7996: f64 = (noise_0_psd_e7995 * noise_0_psd_e361);
                let psd = noise_0_psd_e7996;
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
                let noise_1_psd_e7998: f64 = 1.0;
                let noise_1_psd_e375: f64 = (noise_variable_306 * params.p1);
                let noise_1_psd_e7999: f64 = (noise_1_psd_e7998 * noise_1_psd_e375);
                let psd = noise_1_psd_e7999;
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
                let noise_2_psd_e8001: f64 = 1.0;
                let noise_2_psd_e380: f64 = (noise_variable_295 * params.p1);
                let noise_2_psd_e8002: f64 = (noise_2_psd_e8001 * noise_2_psd_e380);
                let psd = noise_2_psd_e8002;
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
                let noise_3_psd_e8004: f64 = 1.0;
                let noise_3_psd_e385: f64 = (noise_variable_288 * params.p1);
                let noise_3_psd_e8005: f64 = (noise_3_psd_e8004 * noise_3_psd_e385);
                let psd = noise_3_psd_e8005;
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
                let noise_4_psd_e8007: f64 = 1.0;
                let noise_4_psd_e390: f64 = (noise_variable_289 * params.p1);
                let noise_4_psd_e8008: f64 = (noise_4_psd_e8007 * noise_4_psd_e390);
                let psd = noise_4_psd_e8008;
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
                let noise_5_psd_e8010: f64 = 1.0;
                let noise_5_psd_e395: f64 = (noise_variable_293 * params.p1);
                let noise_5_psd_e8011: f64 = (noise_5_psd_e8010 * noise_5_psd_e395);
                let psd = noise_5_psd_e8011;
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
                let noise_6_psd_e8013: f64 = 1.0;
                let noise_6_psd_e400: f64 = (noise_variable_296 * params.p1);
                let noise_6_psd_e8014: f64 = (noise_6_psd_e8013 * noise_6_psd_e400);
                let psd = noise_6_psd_e8014;
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
                let noise_7_psd_e8016: f64 = 1.0;
                let noise_7_psd_e406: f64 = (noise_variable_297 * params.p1);
                let noise_7_psd_e8017: f64 = (noise_7_psd_e8016 * noise_7_psd_e406);
                let psd = noise_7_psd_e8017;
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
                let noise_8_psd_e8019: f64 = 1.0;
                let noise_8_psd_e412: f64 = (noise_variable_298 * params.p1);
                let noise_8_psd_e8020: f64 = (noise_8_psd_e8019 * noise_8_psd_e412);
                let psd = noise_8_psd_e8020;
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
                let noise_9_psd_e8022: f64 = 1.0;
                let noise_9_psd_e417: f64 = (noise_variable_299 * params.p1);
                let noise_9_psd_e8023: f64 = (noise_9_psd_e8022 * noise_9_psd_e417);
                let psd = noise_9_psd_e8023;
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
                let noise_10_psd_e8025: f64 = 1.0;
                let noise_10_psd_e422: f64 = (noise_variable_300 * params.p1);
                let noise_10_psd_e8026: f64 = (noise_10_psd_e8025 * noise_10_psd_e422);
                let psd = noise_10_psd_e8026;
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
                let noise_11_psd_e8028: f64 = 1.0;
                let noise_11_psd_e428: f64 = (noise_variable_302 * params.p1);
                let noise_11_psd_e8029: f64 = (noise_11_psd_e8028 * noise_11_psd_e428);
                let psd = noise_11_psd_e8029;
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
                let noise_12_psd_e8031: f64 = 1.0;
                let noise_12_psd_e433: f64 = (noise_variable_304 * params.p1);
                let noise_12_psd_e8032: f64 = (noise_12_psd_e8031 * noise_12_psd_e433);
                let psd = noise_12_psd_e8032;
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
                let noise_13_psd_e8034: f64 = 1.0;
                let noise_13_psd_e439: f64 = (noise_variable_303 * params.p1);
                let noise_13_psd_e8035: f64 = (noise_13_psd_e8034 * noise_13_psd_e439);
                let psd = noise_13_psd_e8035;
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
                let noise_14_psd_e8037: f64 = 1.0;
                let noise_14_psd_e444: f64 = (noise_variable_305 * params.p1);
                let noise_14_psd_e8038: f64 = (noise_14_psd_e8037 * noise_14_psd_e444);
                let psd = noise_14_psd_e8038;
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
                let noise_15_psd_e8040: f64 = 1.0;
                let noise_15_psd_e451: f64 = (noise_variable_301 * params.p1);
                let noise_15_psd_e8041: f64 = (noise_15_psd_e8040 * noise_15_psd_e451);
                let psd = noise_15_psd_e8041;
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
                let noise_16_psd_e8043: f64 = 1.0;
                let noise_16_psd_e460: f64 = (noise_variable_301 * params.p1);
                let noise_16_psd_e8044: f64 = (noise_16_psd_e8043 * noise_16_psd_e460);
                let psd = noise_16_psd_e8044;
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
                let noise_17_psd_e8046: f64 = 1.0;
                let noise_17_psd_e470: f64 = (noise_variable_290 * params.p1);
                let noise_17_psd_e8047: f64 = (noise_17_psd_e8046 * noise_17_psd_e470);
                let psd = noise_17_psd_e8047;
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
                let noise_18_psd_e8049: f64 = 1.0;
                let noise_18_psd_e480: f64 = (noise_variable_291 * params.p1);
                let noise_18_psd_e8050: f64 = (noise_18_psd_e8049 * noise_18_psd_e480);
                let psd = noise_18_psd_e8050;
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
                let noise_19_psd_e8052: f64 = 1.0;
                let noise_19_psd_e490: f64 = (noise_variable_292 * params.p1);
                let noise_19_psd_e8053: f64 = (noise_19_psd_e8052 * noise_19_psd_e490);
                let psd = noise_19_psd_e8053;
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
                let noise_20_psd_e8055: f64 = 1.0;
                let noise_20_psd_e501: f64 = (noise_variable_290 * params.p1);
                let noise_20_psd_e8056: f64 = (noise_20_psd_e8055 * noise_20_psd_e501);
                let psd = noise_20_psd_e8056;
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
                let noise_21_psd_e8058: f64 = 1.0;
                let noise_21_psd_e512: f64 = (noise_variable_291 * params.p1);
                let noise_21_psd_e8059: f64 = (noise_21_psd_e8058 * noise_21_psd_e512);
                let psd = noise_21_psd_e8059;
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
                let noise_22_psd_e8061: f64 = 1.0;
                let noise_22_psd_e523: f64 = (noise_variable_290 * params.p1);
                let noise_22_psd_e8062: f64 = (noise_22_psd_e8061 * noise_22_psd_e523);
                let psd = noise_22_psd_e8062;
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
                let noise_23_psd_e8064: f64 = 1.0;
                let noise_23_psd_e534: f64 = (noise_variable_292 * params.p1);
                let noise_23_psd_e8065: f64 = (noise_23_psd_e8064 * noise_23_psd_e534);
                let psd = noise_23_psd_e8065;
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
                let noise_24_psd_e8067: f64 = 1.0;
                let noise_24_psd_e546: f64 = (noise_variable_290 * params.p1);
                let noise_24_psd_e8068: f64 = (noise_24_psd_e8067 * noise_24_psd_e546);
                let psd = noise_24_psd_e8068;
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

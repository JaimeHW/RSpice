#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 28] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_NOI_GND_IN", label: Some("in"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "noi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_E1_IB2E1", label: Some("ib2e1"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_E1_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_B1_RBC", label: Some("rbc"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_B2_RBV", label: Some("rbv"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B2_E1_IB2E1_F", label: Some("ib2e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_E1_IB1E1_F", label: Some("ib1e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_E1_IB1E1", label: Some("ib1e1"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IB3", label: Some("ib3"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IB3_F", label: Some("ib3_f"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IEX", label: Some("iex"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IEX_F", label: Some("iex_f"), kind: GeneratedNoiseKind::Flicker, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_C3_XIEX", label: Some("xiex"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B_C3_XIEX_F", label: Some("xiex_f"), kind: GeneratedNoiseKind::Flicker, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C1_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_S_ISUB_INT", label: Some("isub_int"), kind: GeneratedNoiseKind::White, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_S_ISUB", label: Some("isub"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_S_XISUB", label: Some("xisub"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C4_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 57, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C1_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C4_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C1_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
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
        if matches!(source_index, 15 | 16) {
            let noise_activation_schedule_732_e7296: f64 = if params.p24 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_610 = noise_activation_schedule_732_e7296;
        }
        if matches!(source_index, 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_activation_schedule_733_e7299: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_611 = noise_activation_schedule_733_e7299;
        }
        if matches!(source_index, 20 | 21 | 22 | 23 | 24) {
            let noise_activation_schedule_734_e7302: f64 = if params.p59 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_612 = noise_activation_schedule_734_e7302;
        }
        if matches!(source_index, 25 | 26 | 27) {
            let noise_activation_schedule_735_e7305: f64 = if params.p59 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_613 = noise_activation_schedule_735_e7305;
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
                noise_variable_610 != 0.0
            }
            16 => {
                let noise_16_activation_e484: f64 = if (noise_variable_610 == 0.0) { 1.0 } else { 0.0 };
                noise_16_activation_e484 != 0.0
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
                let noise_20_activation_e509: f64 = if ((noise_variable_611 != 0.0) && (noise_variable_612 != 0.0)) { 1.0 } else { 0.0 };
                noise_20_activation_e509 != 0.0
            }
            21 => {
                let noise_21_activation_e519: f64 = if ((noise_variable_611 != 0.0) && (noise_variable_612 != 0.0)) { 1.0 } else { 0.0 };
                noise_21_activation_e519 != 0.0
            }
            22 => {
                let noise_22_activation_e529: f64 = if ((noise_variable_611 != 0.0) && (noise_variable_612 != 0.0)) { 1.0 } else { 0.0 };
                noise_22_activation_e529 != 0.0
            }
            23 => {
                let noise_23_activation_e540: f64 = if ((noise_variable_611 != 0.0) && (noise_variable_612 == 0.0)) { 1.0 } else { 0.0 };
                noise_23_activation_e540 != 0.0
            }
            24 => {
                let noise_24_activation_e551: f64 = if ((noise_variable_611 != 0.0) && (noise_variable_612 == 0.0)) { 1.0 } else { 0.0 };
                noise_24_activation_e551 != 0.0
            }
            25 => {
                let noise_25_activation_e562: f64 = if ((noise_variable_611 == 0.0) && (noise_variable_613 != 0.0)) { 1.0 } else { 0.0 };
                noise_25_activation_e562 != 0.0
            }
            26 => {
                let noise_26_activation_e573: f64 = if ((noise_variable_611 == 0.0) && (noise_variable_613 != 0.0)) { 1.0 } else { 0.0 };
                noise_26_activation_e573 != 0.0
            }
            27 => {
                let noise_27_activation_e585: f64 = if ((noise_variable_611 == 0.0) && (noise_variable_613 == 0.0)) { 1.0 } else { 0.0 };
                noise_27_activation_e585 != 0.0
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
        if matches!(source_index, 1) {
            let noise_metadata_schedule_0_e595: f64 = if params.p3 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_476 = noise_metadata_schedule_0_e595;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_1_e599,) = {
    if (noise_variable_476 != 0.0) {
        (70300000.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_1_e599;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_2_e603,) = {
    if (noise_variable_476 != 0.0) {
        (123000000.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_2_e603;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_3_e608,) = {
    if (noise_variable_476 == 0.0) {
        (158000000.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_3_e608;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_4_e613,) = {
    if (noise_variable_476 == 0.0) {
        (204000000.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_4_e613;
        }
        if matches!(source_index, 11 | 12 | 18) {
            let noise_metadata_schedule_5_e616: f64 = (1.0 - params.p33);
            noise_variable_157 = noise_metadata_schedule_5_e616;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_6_e619: f64 = (params.p4 + 273.15);
            noise_variable_3 = noise_metadata_schedule_6_e619;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_7_e620: f64 = ctx.temperature();
            let noise_metadata_schedule_7_e622: f64 = (noise_metadata_schedule_7_e620 + params.p0);
            noise_variable_5 = noise_metadata_schedule_7_e622;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_9_e628: f64 = if params.p150 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_477 = noise_metadata_schedule_9_e628;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let (noise_metadata_schedule_10_e632,) = {
    if (noise_variable_477 != 0.0) {
        (1e-12,)
    } else {
        (noise_variable_339,)
    }
};
            noise_variable_339 = noise_metadata_schedule_10_e632;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let (noise_metadata_schedule_11_e637,) = {
    if (noise_variable_477 == 0.0) {
        (params.p150,)
    } else {
        (noise_variable_339,)
    }
};
            noise_variable_339 = noise_metadata_schedule_11_e637;
        }
        if matches!(source_index, 1 | 3 | 4 | 5 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_12_e640: f64 = (noise_variable_339 * params.p1);
            noise_variable_340 = noise_metadata_schedule_12_e640;
        }
        if matches!(source_index, 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_13_e643: f64 = (1.0 / noise_variable_340);
            noise_variable_341 = noise_metadata_schedule_13_e643;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            noise_variable_52 = 0.001;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            noise_variable_336 = 0.001;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_19_e661: f64 = (2.0 - params.p67);
            let noise_metadata_schedule_19_e662: f64 = (2.0_f64).powf(noise_metadata_schedule_19_e661);
            noise_variable_62 = noise_metadata_schedule_19_e662;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_20_e665: f64 = (1.0 / noise_variable_62);
            noise_variable_63 = noise_metadata_schedule_20_e665;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_21_e669: f64 = (params.p115 * noise_variable_3);
            let noise_metadata_schedule_21_e671: f64 = (noise_metadata_schedule_21_e669 * noise_variable_3);
            let noise_metadata_schedule_21_e674: f64 = (noise_variable_3 + params.p116);
            let noise_metadata_schedule_21_e675: f64 = (noise_metadata_schedule_21_e671 / noise_metadata_schedule_21_e674);
            let noise_metadata_schedule_21_e676: f64 = (params.p114 + noise_metadata_schedule_21_e675);
            let noise_metadata_schedule_21_e678: f64 = (noise_metadata_schedule_21_e676 - 0.05);
            let noise_metadata_schedule_21_e680: f64 = (noise_metadata_schedule_21_e678 / 0.1);
            noise_variable_279 = noise_metadata_schedule_21_e680;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_22_e684: f64 = (params.p115 * noise_variable_3);
            let noise_metadata_schedule_22_e686: f64 = (noise_metadata_schedule_22_e684 * noise_variable_3);
            let noise_metadata_schedule_22_e689: f64 = (noise_variable_3 + params.p116);
            let noise_metadata_schedule_22_e690: f64 = (noise_metadata_schedule_22_e686 / noise_metadata_schedule_22_e689);
            let noise_metadata_schedule_22_e691: f64 = (params.p114 + noise_metadata_schedule_22_e690);
            let noise_metadata_schedule_22_e693: f64 = if noise_metadata_schedule_22_e691 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_479 = noise_metadata_schedule_22_e693;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_23_e705,) = {
    if (noise_variable_479 != 0.0) {
        let noise_metadata_schedule_23_e699: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_23_e700: f64 = (1.0 + noise_metadata_schedule_23_e699);
        let noise_metadata_schedule_23_e701: f64 = (noise_metadata_schedule_23_e700).ln();
        let noise_metadata_schedule_23_e702: f64 = (0.1 * noise_metadata_schedule_23_e701);
        let noise_metadata_schedule_23_e703: f64 = (0.05 + noise_metadata_schedule_23_e702);
        (noise_metadata_schedule_23_e703,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_23_e705;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_24_e729,) = {
    if (noise_variable_479 == 0.0) {
        let noise_metadata_schedule_24_e711: f64 = (params.p115 * noise_variable_3);
        let noise_metadata_schedule_24_e713: f64 = (noise_metadata_schedule_24_e711 * noise_variable_3);
        let noise_metadata_schedule_24_e716: f64 = (noise_variable_3 + params.p116);
        let noise_metadata_schedule_24_e717: f64 = (noise_metadata_schedule_24_e713 / noise_metadata_schedule_24_e716);
        let noise_metadata_schedule_24_e718: f64 = (params.p114 + noise_metadata_schedule_24_e717);
        let noise_metadata_schedule_24_e722: f64 = (-noise_variable_279);
        let noise_metadata_schedule_24_e723: f64 = (noise_metadata_schedule_24_e722).exp();
        let noise_metadata_schedule_24_e724: f64 = (1.0 + noise_metadata_schedule_24_e723);
        let noise_metadata_schedule_24_e725: f64 = (noise_metadata_schedule_24_e724).ln();
        let noise_metadata_schedule_24_e726: f64 = (0.1 * noise_metadata_schedule_24_e725);
        let noise_metadata_schedule_24_e727: f64 = (noise_metadata_schedule_24_e718 + noise_metadata_schedule_24_e726);
        (noise_metadata_schedule_24_e727,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_24_e729;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            noise_variable_71 = params.p114;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_26_e733: f64 = (1.0 / noise_variable_71);
            noise_variable_72 = noise_metadata_schedule_26_e733;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_27_e736: f64 = (1.0 / params.p66);
            noise_variable_64 = noise_metadata_schedule_27_e736;
        }
        if matches!(source_index, 1 | 15 | 16) {
            noise_variable_75 = params.p71;
        }
        if matches!(source_index, 1 | 15 | 16) {
            noise_variable_76 = params.p72;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_30_e742: f64 = (2.0 - noise_variable_76);
            let noise_metadata_schedule_30_e743: f64 = (2.0_f64).powf(noise_metadata_schedule_30_e742);
            noise_variable_79 = noise_metadata_schedule_30_e743;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_31_e746: f64 = (1.0 / noise_variable_79);
            noise_variable_89 = noise_metadata_schedule_31_e746;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_32_e750: f64 = (params.p118 * noise_variable_3);
            let noise_metadata_schedule_32_e752: f64 = (noise_metadata_schedule_32_e750 * noise_variable_3);
            let noise_metadata_schedule_32_e755: f64 = (noise_variable_3 + params.p119);
            let noise_metadata_schedule_32_e756: f64 = (noise_metadata_schedule_32_e752 / noise_metadata_schedule_32_e755);
            let noise_metadata_schedule_32_e757: f64 = (params.p117 + noise_metadata_schedule_32_e756);
            let noise_metadata_schedule_32_e759: f64 = (noise_metadata_schedule_32_e757 - 0.05);
            let noise_metadata_schedule_32_e761: f64 = (noise_metadata_schedule_32_e759 / 0.1);
            noise_variable_279 = noise_metadata_schedule_32_e761;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_33_e765: f64 = (params.p118 * noise_variable_3);
            let noise_metadata_schedule_33_e767: f64 = (noise_metadata_schedule_33_e765 * noise_variable_3);
            let noise_metadata_schedule_33_e770: f64 = (noise_variable_3 + params.p119);
            let noise_metadata_schedule_33_e771: f64 = (noise_metadata_schedule_33_e767 / noise_metadata_schedule_33_e770);
            let noise_metadata_schedule_33_e772: f64 = (params.p117 + noise_metadata_schedule_33_e771);
            let noise_metadata_schedule_33_e774: f64 = if noise_metadata_schedule_33_e772 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_480 = noise_metadata_schedule_33_e774;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_34_e786,) = {
    if (noise_variable_480 != 0.0) {
        let noise_metadata_schedule_34_e780: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_34_e781: f64 = (1.0 + noise_metadata_schedule_34_e780);
        let noise_metadata_schedule_34_e782: f64 = (noise_metadata_schedule_34_e781).ln();
        let noise_metadata_schedule_34_e783: f64 = (0.1 * noise_metadata_schedule_34_e782);
        let noise_metadata_schedule_34_e784: f64 = (0.05 + noise_metadata_schedule_34_e783);
        (noise_metadata_schedule_34_e784,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_34_e786;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_35_e810,) = {
    if (noise_variable_480 == 0.0) {
        let noise_metadata_schedule_35_e792: f64 = (params.p118 * noise_variable_3);
        let noise_metadata_schedule_35_e794: f64 = (noise_metadata_schedule_35_e792 * noise_variable_3);
        let noise_metadata_schedule_35_e797: f64 = (noise_variable_3 + params.p119);
        let noise_metadata_schedule_35_e798: f64 = (noise_metadata_schedule_35_e794 / noise_metadata_schedule_35_e797);
        let noise_metadata_schedule_35_e799: f64 = (params.p117 + noise_metadata_schedule_35_e798);
        let noise_metadata_schedule_35_e803: f64 = (-noise_variable_279);
        let noise_metadata_schedule_35_e804: f64 = (noise_metadata_schedule_35_e803).exp();
        let noise_metadata_schedule_35_e805: f64 = (1.0 + noise_metadata_schedule_35_e804);
        let noise_metadata_schedule_35_e806: f64 = (noise_metadata_schedule_35_e805).ln();
        let noise_metadata_schedule_35_e807: f64 = (0.1 * noise_metadata_schedule_35_e806);
        let noise_metadata_schedule_35_e808: f64 = (noise_metadata_schedule_35_e799 + noise_metadata_schedule_35_e807);
        (noise_metadata_schedule_35_e808,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_35_e810;
        }
        if matches!(source_index, 1 | 15 | 16) {
            noise_variable_87 = params.p117;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_37_e814: f64 = (1.0 / noise_variable_87);
            noise_variable_86 = noise_metadata_schedule_37_e814;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_38_e817: f64 = (1.0 / noise_variable_75);
            noise_variable_66 = noise_metadata_schedule_38_e817;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_39_e821: f64 = (1.0 / params.p83);
            let noise_metadata_schedule_39_e822: f64 = (1.0 - noise_metadata_schedule_39_e821);
            noise_variable_343 = noise_metadata_schedule_39_e822;
        }
        if matches!(source_index, 2 | 6) {
            noise_variable_158 = 0.0;
        }
        if matches!(source_index, 6 | 8) {
            noise_variable_159 = 0.0;
        }
        if matches!(source_index, 13 | 14) {
            noise_variable_176 = 0.0;
        }
        if matches!(source_index, 13 | 14 | 19) {
            noise_variable_175 = 1.0;
        }
        if matches!(source_index, 1) {
            noise_variable_207 = 0.0;
        }
        if matches!(source_index, 1) {
            noise_variable_209 = 0.0;
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
            noise_variable_11 = 0.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_54_e839: f64 = (noise_variable_5 + noise_variable_11);
            noise_variable_2 = noise_metadata_schedule_54_e839;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_55_e842: f64 = (noise_variable_2 / noise_variable_3);
            noise_variable_4 = noise_metadata_schedule_55_e842;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_56_e845: f64 = (8.617086918058125e-5 * noise_variable_2);
            noise_variable_6 = noise_metadata_schedule_56_e845;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_57_e848: f64 = (8.617086918058125e-5 * noise_variable_3);
            noise_variable_7 = noise_metadata_schedule_57_e848;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_58_e851: f64 = (1.0 / noise_variable_6);
            noise_variable_8 = noise_metadata_schedule_58_e851;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_59_e854: f64 = (1.0 / noise_variable_7);
            noise_variable_9 = noise_metadata_schedule_59_e854;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_60_e857: f64 = (noise_variable_8 - noise_variable_9);
            noise_variable_10 = noise_metadata_schedule_60_e857;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_61_e860: f64 = (noise_variable_2 - noise_variable_3);
            noise_variable_12 = noise_metadata_schedule_61_e860;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_62_e862: f64 = (noise_variable_4).ln();
            noise_variable_274 = noise_metadata_schedule_62_e862;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_63_e866: f64 = (params.p115 * noise_variable_2);
            let noise_metadata_schedule_63_e868: f64 = (noise_metadata_schedule_63_e866 * noise_variable_2);
            let noise_metadata_schedule_63_e871: f64 = (noise_variable_2 + params.p116);
            let noise_metadata_schedule_63_e872: f64 = (noise_metadata_schedule_63_e868 / noise_metadata_schedule_63_e871);
            let noise_metadata_schedule_63_e873: f64 = (noise_variable_74 - noise_metadata_schedule_63_e872);
            let noise_metadata_schedule_63_e875: f64 = (noise_metadata_schedule_63_e873 - 0.05);
            let noise_metadata_schedule_63_e877: f64 = (noise_metadata_schedule_63_e875 / 0.1);
            noise_variable_279 = noise_metadata_schedule_63_e877;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_64_e881: f64 = (params.p115 * noise_variable_2);
            let noise_metadata_schedule_64_e883: f64 = (noise_metadata_schedule_64_e881 * noise_variable_2);
            let noise_metadata_schedule_64_e886: f64 = (noise_variable_2 + params.p116);
            let noise_metadata_schedule_64_e887: f64 = (noise_metadata_schedule_64_e883 / noise_metadata_schedule_64_e886);
            let noise_metadata_schedule_64_e888: f64 = (noise_variable_74 - noise_metadata_schedule_64_e887);
            let noise_metadata_schedule_64_e890: f64 = if noise_metadata_schedule_64_e888 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_481 = noise_metadata_schedule_64_e890;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_65_e902,) = {
    if (noise_variable_481 != 0.0) {
        let noise_metadata_schedule_65_e896: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_65_e897: f64 = (1.0 + noise_metadata_schedule_65_e896);
        let noise_metadata_schedule_65_e898: f64 = (noise_metadata_schedule_65_e897).ln();
        let noise_metadata_schedule_65_e899: f64 = (0.1 * noise_metadata_schedule_65_e898);
        let noise_metadata_schedule_65_e900: f64 = (0.05 + noise_metadata_schedule_65_e899);
        (noise_metadata_schedule_65_e900,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_65_e902;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_66_e926,) = {
    if (noise_variable_481 == 0.0) {
        let noise_metadata_schedule_66_e908: f64 = (params.p115 * noise_variable_2);
        let noise_metadata_schedule_66_e910: f64 = (noise_metadata_schedule_66_e908 * noise_variable_2);
        let noise_metadata_schedule_66_e913: f64 = (noise_variable_2 + params.p116);
        let noise_metadata_schedule_66_e914: f64 = (noise_metadata_schedule_66_e910 / noise_metadata_schedule_66_e913);
        let noise_metadata_schedule_66_e915: f64 = (noise_variable_74 - noise_metadata_schedule_66_e914);
        let noise_metadata_schedule_66_e919: f64 = (-noise_variable_279);
        let noise_metadata_schedule_66_e920: f64 = (noise_metadata_schedule_66_e919).exp();
        let noise_metadata_schedule_66_e921: f64 = (1.0 + noise_metadata_schedule_66_e920);
        let noise_metadata_schedule_66_e922: f64 = (noise_metadata_schedule_66_e921).ln();
        let noise_metadata_schedule_66_e923: f64 = (0.1 * noise_metadata_schedule_66_e922);
        let noise_metadata_schedule_66_e924: f64 = (noise_metadata_schedule_66_e915 + noise_metadata_schedule_66_e923);
        (noise_metadata_schedule_66_e924,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_66_e926;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_67_e930: f64 = (params.p118 * noise_variable_2);
            let noise_metadata_schedule_67_e932: f64 = (noise_metadata_schedule_67_e930 * noise_variable_2);
            let noise_metadata_schedule_67_e935: f64 = (noise_variable_2 + params.p119);
            let noise_metadata_schedule_67_e936: f64 = (noise_metadata_schedule_67_e932 / noise_metadata_schedule_67_e935);
            let noise_metadata_schedule_67_e937: f64 = (noise_variable_88 - noise_metadata_schedule_67_e936);
            let noise_metadata_schedule_67_e939: f64 = (noise_metadata_schedule_67_e937 - 0.05);
            let noise_metadata_schedule_67_e941: f64 = (noise_metadata_schedule_67_e939 / 0.1);
            noise_variable_279 = noise_metadata_schedule_67_e941;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_68_e945: f64 = (params.p118 * noise_variable_2);
            let noise_metadata_schedule_68_e947: f64 = (noise_metadata_schedule_68_e945 * noise_variable_2);
            let noise_metadata_schedule_68_e950: f64 = (noise_variable_2 + params.p119);
            let noise_metadata_schedule_68_e951: f64 = (noise_metadata_schedule_68_e947 / noise_metadata_schedule_68_e950);
            let noise_metadata_schedule_68_e952: f64 = (noise_variable_88 - noise_metadata_schedule_68_e951);
            let noise_metadata_schedule_68_e954: f64 = if noise_metadata_schedule_68_e952 < 0.05 { 1.0 } else { 0.0 };
            noise_variable_482 = noise_metadata_schedule_68_e954;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_69_e966,) = {
    if (noise_variable_482 != 0.0) {
        let noise_metadata_schedule_69_e960: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_69_e961: f64 = (1.0 + noise_metadata_schedule_69_e960);
        let noise_metadata_schedule_69_e962: f64 = (noise_metadata_schedule_69_e961).ln();
        let noise_metadata_schedule_69_e963: f64 = (0.1 * noise_metadata_schedule_69_e962);
        let noise_metadata_schedule_69_e964: f64 = (0.05 + noise_metadata_schedule_69_e963);
        (noise_metadata_schedule_69_e964,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_69_e966;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_70_e990,) = {
    if (noise_variable_482 == 0.0) {
        let noise_metadata_schedule_70_e972: f64 = (params.p118 * noise_variable_2);
        let noise_metadata_schedule_70_e974: f64 = (noise_metadata_schedule_70_e972 * noise_variable_2);
        let noise_metadata_schedule_70_e977: f64 = (noise_variable_2 + params.p119);
        let noise_metadata_schedule_70_e978: f64 = (noise_metadata_schedule_70_e974 / noise_metadata_schedule_70_e977);
        let noise_metadata_schedule_70_e979: f64 = (noise_variable_88 - noise_metadata_schedule_70_e978);
        let noise_metadata_schedule_70_e983: f64 = (-noise_variable_279);
        let noise_metadata_schedule_70_e984: f64 = (noise_metadata_schedule_70_e983).exp();
        let noise_metadata_schedule_70_e985: f64 = (1.0 + noise_metadata_schedule_70_e984);
        let noise_metadata_schedule_70_e986: f64 = (noise_metadata_schedule_70_e985).ln();
        let noise_metadata_schedule_70_e987: f64 = (0.1 * noise_metadata_schedule_70_e986);
        let noise_metadata_schedule_70_e988: f64 = (noise_metadata_schedule_70_e979 + noise_metadata_schedule_70_e987);
        (noise_metadata_schedule_70_e988,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_70_e990;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_71_e992: f64 = (-3.0);
            let noise_metadata_schedule_71_e994: f64 = (noise_metadata_schedule_71_e992 * noise_variable_6);
            let noise_metadata_schedule_71_e996: f64 = (noise_metadata_schedule_71_e994 * noise_variable_274);
            let noise_metadata_schedule_71_e999: f64 = (params.p66 * noise_variable_4);
            let noise_metadata_schedule_71_e1000: f64 = (noise_metadata_schedule_71_e996 + noise_metadata_schedule_71_e999);
            let noise_metadata_schedule_71_e1003: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_71_e1005: f64 = (noise_metadata_schedule_71_e1003 * params.p105);
            let noise_metadata_schedule_71_e1006: f64 = (noise_metadata_schedule_71_e1000 + noise_metadata_schedule_71_e1005);
            noise_variable_13 = noise_metadata_schedule_71_e1006;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_72_e1009: f64 = (0.05 - noise_variable_13);
            let noise_metadata_schedule_72_e1011: f64 = (noise_metadata_schedule_72_e1009 / noise_variable_6);
            noise_variable_279 = noise_metadata_schedule_72_e1011;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_73_e1014: f64 = if 0.05 < noise_variable_13 { 1.0 } else { 0.0 };
            noise_variable_483 = noise_metadata_schedule_73_e1014;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_74_e1026,) = {
    if (noise_variable_483 != 0.0) {
        let noise_metadata_schedule_74_e1020: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_74_e1021: f64 = (1.0 + noise_metadata_schedule_74_e1020);
        let noise_metadata_schedule_74_e1022: f64 = (noise_metadata_schedule_74_e1021).ln();
        let noise_metadata_schedule_74_e1023: f64 = (noise_variable_6 * noise_metadata_schedule_74_e1022);
        let noise_metadata_schedule_74_e1024: f64 = (noise_variable_13 + noise_metadata_schedule_74_e1023);
        (noise_metadata_schedule_74_e1024,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_74_e1026;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_75_e1040,) = {
    if (noise_variable_483 == 0.0) {
        let noise_metadata_schedule_75_e1033: f64 = (-noise_variable_279);
        let noise_metadata_schedule_75_e1034: f64 = (noise_metadata_schedule_75_e1033).exp();
        let noise_metadata_schedule_75_e1035: f64 = (1.0 + noise_metadata_schedule_75_e1034);
        let noise_metadata_schedule_75_e1036: f64 = (noise_metadata_schedule_75_e1035).ln();
        let noise_metadata_schedule_75_e1037: f64 = (noise_variable_6 * noise_metadata_schedule_75_e1036);
        let noise_metadata_schedule_75_e1038: f64 = (0.05 + noise_metadata_schedule_75_e1037);
        (noise_metadata_schedule_75_e1038,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_75_e1040;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_76_e1042: f64 = (-3.0);
            let noise_metadata_schedule_76_e1044: f64 = (noise_metadata_schedule_76_e1042 * noise_variable_6);
            let noise_metadata_schedule_76_e1046: f64 = (noise_metadata_schedule_76_e1044 * noise_variable_274);
            let noise_metadata_schedule_76_e1049: f64 = (params.p64 * noise_variable_4);
            let noise_metadata_schedule_76_e1050: f64 = (noise_metadata_schedule_76_e1046 + noise_metadata_schedule_76_e1049);
            let noise_metadata_schedule_76_e1053: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_76_e1055: f64 = (noise_metadata_schedule_76_e1053 * params.p110);
            let noise_metadata_schedule_76_e1056: f64 = (noise_metadata_schedule_76_e1050 + noise_metadata_schedule_76_e1055);
            noise_variable_15 = noise_metadata_schedule_76_e1056;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_77_e1059: f64 = (0.05 - noise_variable_15);
            let noise_metadata_schedule_77_e1061: f64 = (noise_metadata_schedule_77_e1059 / noise_variable_6);
            noise_variable_279 = noise_metadata_schedule_77_e1061;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_78_e1064: f64 = if 0.05 < noise_variable_15 { 1.0 } else { 0.0 };
            noise_variable_484 = noise_metadata_schedule_78_e1064;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_79_e1076,) = {
    if (noise_variable_484 != 0.0) {
        let noise_metadata_schedule_79_e1070: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_79_e1071: f64 = (1.0 + noise_metadata_schedule_79_e1070);
        let noise_metadata_schedule_79_e1072: f64 = (noise_metadata_schedule_79_e1071).ln();
        let noise_metadata_schedule_79_e1073: f64 = (noise_variable_6 * noise_metadata_schedule_79_e1072);
        let noise_metadata_schedule_79_e1074: f64 = (noise_variable_15 + noise_metadata_schedule_79_e1073);
        (noise_metadata_schedule_79_e1074,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_79_e1076;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_80_e1090,) = {
    if (noise_variable_484 == 0.0) {
        let noise_metadata_schedule_80_e1083: f64 = (-noise_variable_279);
        let noise_metadata_schedule_80_e1084: f64 = (noise_metadata_schedule_80_e1083).exp();
        let noise_metadata_schedule_80_e1085: f64 = (1.0 + noise_metadata_schedule_80_e1084);
        let noise_metadata_schedule_80_e1086: f64 = (noise_metadata_schedule_80_e1085).ln();
        let noise_metadata_schedule_80_e1087: f64 = (noise_variable_6 * noise_metadata_schedule_80_e1086);
        let noise_metadata_schedule_80_e1088: f64 = (0.05 + noise_metadata_schedule_80_e1087);
        (noise_metadata_schedule_80_e1088,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_80_e1090;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_86_e1142: f64 = (-3.0);
            let noise_metadata_schedule_86_e1144: f64 = (noise_metadata_schedule_86_e1142 * noise_variable_6);
            let noise_metadata_schedule_86_e1146: f64 = (noise_metadata_schedule_86_e1144 * noise_variable_274);
            let noise_metadata_schedule_86_e1149: f64 = (params.p71 * noise_variable_4);
            let noise_metadata_schedule_86_e1150: f64 = (noise_metadata_schedule_86_e1146 + noise_metadata_schedule_86_e1149);
            let noise_metadata_schedule_86_e1153: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_86_e1155: f64 = (noise_metadata_schedule_86_e1153 * params.p110);
            let noise_metadata_schedule_86_e1156: f64 = (noise_metadata_schedule_86_e1150 + noise_metadata_schedule_86_e1155);
            noise_variable_18 = noise_metadata_schedule_86_e1156;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_87_e1159: f64 = (0.05 - noise_variable_18);
            let noise_metadata_schedule_87_e1161: f64 = (noise_metadata_schedule_87_e1159 / noise_variable_6);
            noise_variable_279 = noise_metadata_schedule_87_e1161;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_88_e1164: f64 = if 0.05 < noise_variable_18 { 1.0 } else { 0.0 };
            noise_variable_486 = noise_metadata_schedule_88_e1164;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_89_e1176,) = {
    if (noise_variable_486 != 0.0) {
        let noise_metadata_schedule_89_e1170: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_89_e1171: f64 = (1.0 + noise_metadata_schedule_89_e1170);
        let noise_metadata_schedule_89_e1172: f64 = (noise_metadata_schedule_89_e1171).ln();
        let noise_metadata_schedule_89_e1173: f64 = (noise_variable_6 * noise_metadata_schedule_89_e1172);
        let noise_metadata_schedule_89_e1174: f64 = (noise_variable_18 + noise_metadata_schedule_89_e1173);
        (noise_metadata_schedule_89_e1174,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_89_e1176;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_90_e1190,) = {
    if (noise_variable_486 == 0.0) {
        let noise_metadata_schedule_90_e1183: f64 = (-noise_variable_279);
        let noise_metadata_schedule_90_e1184: f64 = (noise_metadata_schedule_90_e1183).exp();
        let noise_metadata_schedule_90_e1185: f64 = (1.0 + noise_metadata_schedule_90_e1184);
        let noise_metadata_schedule_90_e1186: f64 = (noise_metadata_schedule_90_e1185).ln();
        let noise_metadata_schedule_90_e1187: f64 = (noise_variable_6 * noise_metadata_schedule_90_e1186);
        let noise_metadata_schedule_90_e1188: f64 = (0.05 + noise_metadata_schedule_90_e1187);
        (noise_metadata_schedule_90_e1188,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_90_e1190;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_91_e1192: f64 = (-3.0);
            let noise_metadata_schedule_91_e1194: f64 = (noise_metadata_schedule_91_e1192 * noise_variable_6);
            let noise_metadata_schedule_91_e1196: f64 = (noise_metadata_schedule_91_e1194 * noise_variable_274);
            let noise_metadata_schedule_91_e1199: f64 = (noise_variable_75 * noise_variable_4);
            let noise_metadata_schedule_91_e1200: f64 = (noise_metadata_schedule_91_e1196 + noise_metadata_schedule_91_e1199);
            let noise_metadata_schedule_91_e1203: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_91_e1205: f64 = (noise_metadata_schedule_91_e1203 * params.p110);
            let noise_metadata_schedule_91_e1206: f64 = (noise_metadata_schedule_91_e1200 + noise_metadata_schedule_91_e1205);
            noise_variable_20 = noise_metadata_schedule_91_e1206;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_92_e1209: f64 = (0.05 - noise_variable_20);
            let noise_metadata_schedule_92_e1211: f64 = (noise_metadata_schedule_92_e1209 / noise_variable_6);
            noise_variable_279 = noise_metadata_schedule_92_e1211;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_93_e1214: f64 = if 0.05 < noise_variable_20 { 1.0 } else { 0.0 };
            noise_variable_487 = noise_metadata_schedule_93_e1214;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_94_e1226,) = {
    if (noise_variable_487 != 0.0) {
        let noise_metadata_schedule_94_e1220: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_94_e1221: f64 = (1.0 + noise_metadata_schedule_94_e1220);
        let noise_metadata_schedule_94_e1222: f64 = (noise_metadata_schedule_94_e1221).ln();
        let noise_metadata_schedule_94_e1223: f64 = (noise_variable_6 * noise_metadata_schedule_94_e1222);
        let noise_metadata_schedule_94_e1224: f64 = (noise_variable_20 + noise_metadata_schedule_94_e1223);
        (noise_metadata_schedule_94_e1224,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_94_e1226;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_95_e1240,) = {
    if (noise_variable_487 == 0.0) {
        let noise_metadata_schedule_95_e1233: f64 = (-noise_variable_279);
        let noise_metadata_schedule_95_e1234: f64 = (noise_metadata_schedule_95_e1233).exp();
        let noise_metadata_schedule_95_e1235: f64 = (1.0 + noise_metadata_schedule_95_e1234);
        let noise_metadata_schedule_95_e1236: f64 = (noise_metadata_schedule_95_e1235).ln();
        let noise_metadata_schedule_95_e1237: f64 = (noise_variable_6 * noise_metadata_schedule_95_e1236);
        let noise_metadata_schedule_95_e1238: f64 = (0.05 + noise_metadata_schedule_95_e1237);
        (noise_metadata_schedule_95_e1238,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_95_e1240;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_96_e1242: f64 = (-3.0);
            let noise_metadata_schedule_96_e1244: f64 = (noise_metadata_schedule_96_e1242 * noise_variable_6);
            let noise_metadata_schedule_96_e1246: f64 = (noise_metadata_schedule_96_e1244 * noise_variable_274);
            let noise_metadata_schedule_96_e1249: f64 = (params.p27 * noise_variable_4);
            let noise_metadata_schedule_96_e1250: f64 = (noise_metadata_schedule_96_e1246 + noise_metadata_schedule_96_e1249);
            let noise_metadata_schedule_96_e1253: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_96_e1255: f64 = (noise_metadata_schedule_96_e1253 * params.p109);
            let noise_metadata_schedule_96_e1256: f64 = (noise_metadata_schedule_96_e1250 + noise_metadata_schedule_96_e1255);
            noise_variable_56 = noise_metadata_schedule_96_e1256;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_97_e1259: f64 = (0.05 - noise_variable_56);
            let noise_metadata_schedule_97_e1261: f64 = (noise_metadata_schedule_97_e1259 / noise_variable_6);
            noise_variable_279 = noise_metadata_schedule_97_e1261;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_98_e1264: f64 = if 0.05 < noise_variable_56 { 1.0 } else { 0.0 };
            noise_variable_488 = noise_metadata_schedule_98_e1264;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_99_e1276,) = {
    if (noise_variable_488 != 0.0) {
        let noise_metadata_schedule_99_e1270: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_99_e1271: f64 = (1.0 + noise_metadata_schedule_99_e1270);
        let noise_metadata_schedule_99_e1272: f64 = (noise_metadata_schedule_99_e1271).ln();
        let noise_metadata_schedule_99_e1273: f64 = (noise_variable_6 * noise_metadata_schedule_99_e1272);
        let noise_metadata_schedule_99_e1274: f64 = (noise_variable_56 + noise_metadata_schedule_99_e1273);
        (noise_metadata_schedule_99_e1274,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_99_e1276;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_100_e1290,) = {
    if (noise_variable_488 == 0.0) {
        let noise_metadata_schedule_100_e1283: f64 = (-noise_variable_279);
        let noise_metadata_schedule_100_e1284: f64 = (noise_metadata_schedule_100_e1283).exp();
        let noise_metadata_schedule_100_e1285: f64 = (1.0 + noise_metadata_schedule_100_e1284);
        let noise_metadata_schedule_100_e1286: f64 = (noise_metadata_schedule_100_e1285).ln();
        let noise_metadata_schedule_100_e1287: f64 = (noise_variable_6 * noise_metadata_schedule_100_e1286);
        let noise_metadata_schedule_100_e1288: f64 = (0.05 + noise_metadata_schedule_100_e1287);
        (noise_metadata_schedule_100_e1288,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_100_e1290;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_101_e1292: f64 = (-3.0);
            let noise_metadata_schedule_101_e1294: f64 = (noise_metadata_schedule_101_e1292 * noise_variable_6);
            let noise_metadata_schedule_101_e1296: f64 = (noise_metadata_schedule_101_e1294 * noise_variable_274);
            let noise_metadata_schedule_101_e1299: f64 = (params.p138 * noise_variable_4);
            let noise_metadata_schedule_101_e1300: f64 = (noise_metadata_schedule_101_e1296 + noise_metadata_schedule_101_e1299);
            let noise_metadata_schedule_101_e1303: f64 = (1.0 - noise_variable_4);
            let noise_metadata_schedule_101_e1305: f64 = (noise_metadata_schedule_101_e1303 * params.p140);
            let noise_metadata_schedule_101_e1306: f64 = (noise_metadata_schedule_101_e1300 + noise_metadata_schedule_101_e1305);
            noise_variable_101 = noise_metadata_schedule_101_e1306;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_102_e1309: f64 = (0.05 - noise_variable_101);
            let noise_metadata_schedule_102_e1311: f64 = (noise_metadata_schedule_102_e1309 / noise_variable_6);
            noise_variable_279 = noise_metadata_schedule_102_e1311;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_106_e1343: f64 = (1.0 / noise_variable_14);
            noise_variable_65 = noise_metadata_schedule_106_e1343;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_107_e1346: f64 = (1.0 / noise_variable_19);
            noise_variable_67 = noise_metadata_schedule_107_e1346;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_108_e1349: f64 = (params.p66 * noise_variable_65);
            let noise_metadata_schedule_108_e1351: f64 = (noise_metadata_schedule_108_e1349).powf(params.p67);
            noise_variable_73 = noise_metadata_schedule_108_e1351;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_109_e1354: f64 = (noise_variable_75 * noise_variable_67);
            let noise_metadata_schedule_109_e1356: f64 = (noise_metadata_schedule_109_e1354).powf(noise_variable_76);
            noise_variable_90 = noise_metadata_schedule_109_e1356;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_112_e1369: f64 = (1.0 - params.p75);
            let noise_metadata_schedule_112_e1372: f64 = (params.p71 / noise_variable_17);
            let noise_metadata_schedule_112_e1374: f64 = (noise_metadata_schedule_112_e1372).powf(params.p72);
            let noise_metadata_schedule_112_e1375: f64 = (noise_metadata_schedule_112_e1369 * noise_metadata_schedule_112_e1374);
            let noise_metadata_schedule_112_e1377: f64 = (noise_metadata_schedule_112_e1375 + params.p75);
            noise_variable_26 = noise_metadata_schedule_112_e1377;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_113_e1380: f64 = (1.0 / noise_variable_26);
            noise_variable_27 = noise_metadata_schedule_113_e1380;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_115_e1386: f64 = (params.p75 * noise_variable_27);
            noise_variable_25 = noise_metadata_schedule_115_e1386;
        }
        if matches!(source_index, 1 | 3) {
            let noise_metadata_schedule_116_e1390: f64 = (noise_variable_274 * params.p97);
            let noise_metadata_schedule_116_e1391: f64 = (noise_metadata_schedule_116_e1390).exp();
            let noise_metadata_schedule_116_e1392: f64 = (params.p54 * noise_metadata_schedule_116_e1391);
            noise_variable_28 = noise_metadata_schedule_116_e1392;
        }
        if matches!(source_index, 1 | 3) {
            let noise_metadata_schedule_117_e1395: f64 = if noise_variable_28 < noise_variable_340 { 1.0 } else { 0.0 };
            noise_variable_490 = noise_metadata_schedule_117_e1395;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_118_e1399,) = {
    if (noise_variable_490 != 0.0) {
        (noise_variable_340,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_118_e1399;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_119_e1404: f64 = (params.p98 - params.p96);
            let noise_metadata_schedule_119_e1405: f64 = (noise_variable_274 * noise_metadata_schedule_119_e1404);
            let noise_metadata_schedule_119_e1406: f64 = (noise_metadata_schedule_119_e1405).exp();
            let noise_metadata_schedule_119_e1407: f64 = (params.p56 * noise_metadata_schedule_119_e1406);
            noise_variable_29 = noise_metadata_schedule_119_e1407;
        }
        if matches!(source_index, 1 | 4) {
            let noise_metadata_schedule_120_e1411: f64 = (noise_variable_274 * params.p101);
            let noise_metadata_schedule_120_e1412: f64 = (noise_metadata_schedule_120_e1411).exp();
            let noise_metadata_schedule_120_e1413: f64 = (params.p55 * noise_metadata_schedule_120_e1412);
            noise_variable_30 = noise_metadata_schedule_120_e1413;
        }
        if matches!(source_index, 1 | 4) {
            let noise_metadata_schedule_121_e1416: f64 = if noise_variable_30 < noise_variable_340 { 1.0 } else { 0.0 };
            noise_variable_491 = noise_metadata_schedule_121_e1416;
        }
        if matches!(source_index, 1 | 4) {
            let (noise_metadata_schedule_122_e1420,) = {
    if (noise_variable_491 != 0.0) {
        (noise_variable_340,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_122_e1420;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19 | 20 | 23 | 25 | 27) {
            let noise_metadata_schedule_123_e1424: f64 = (noise_variable_274 * params.p102);
            let noise_metadata_schedule_123_e1425: f64 = (noise_metadata_schedule_123_e1424).exp();
            let noise_metadata_schedule_123_e1426: f64 = (params.p57 * noise_metadata_schedule_123_e1425);
            noise_variable_32 = noise_metadata_schedule_123_e1426;
        }
        if matches!(source_index, 21 | 24) {
            let noise_metadata_schedule_124_e1430: f64 = (noise_variable_274 * params.p104);
            let noise_metadata_schedule_124_e1431: f64 = (noise_metadata_schedule_124_e1430).exp();
            let noise_metadata_schedule_124_e1432: f64 = (params.p58 * noise_metadata_schedule_124_e1431);
            noise_variable_33 = noise_metadata_schedule_124_e1432;
        }
        if matches!(source_index, 22 | 26) {
            let noise_metadata_schedule_125_e1436: f64 = (noise_variable_274 * params.p104);
            let noise_metadata_schedule_125_e1437: f64 = (noise_metadata_schedule_125_e1436).exp();
            let noise_metadata_schedule_125_e1438: f64 = (params.p59 * noise_metadata_schedule_125_e1437);
            noise_variable_34 = noise_metadata_schedule_125_e1438;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_126_e1442: f64 = (noise_variable_274 * params.p99);
            let noise_metadata_schedule_126_e1443: f64 = (noise_metadata_schedule_126_e1442).exp();
            let noise_metadata_schedule_126_e1444: f64 = (params.p60 * noise_metadata_schedule_126_e1443);
            noise_variable_31 = noise_metadata_schedule_126_e1444;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_127_e1447: f64 = if params.p122 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_492 = noise_metadata_schedule_127_e1447;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_128_e1457,) = {
    if (noise_variable_492 != 0.0) {
        let noise_metadata_schedule_128_e1453: f64 = (noise_variable_12 * params.p122);
        let noise_metadata_schedule_128_e1454: f64 = (1.0 + noise_metadata_schedule_128_e1453);
        let noise_metadata_schedule_128_e1455: f64 = (params.p10 * noise_metadata_schedule_128_e1454);
        (noise_metadata_schedule_128_e1455,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_128_e1457;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_129_e1465,) = {
    if (noise_variable_492 != 0.0) {
        let noise_metadata_schedule_129_e1461: f64 = (noise_variable_50 - 1.0);
        let noise_metadata_schedule_129_e1463: f64 = (noise_metadata_schedule_129_e1461 / noise_variable_52);
        (noise_metadata_schedule_129_e1463,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_129_e1465;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_130_e1468: f64 = if noise_variable_50 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_493 = noise_metadata_schedule_130_e1468;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_131_e1482,) = {
    if ((noise_variable_492 != 0.0) && (noise_variable_493 != 0.0)) {
        let noise_metadata_schedule_131_e1476: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_131_e1477: f64 = (1.0 + noise_metadata_schedule_131_e1476);
        let noise_metadata_schedule_131_e1478: f64 = (noise_metadata_schedule_131_e1477).ln();
        let noise_metadata_schedule_131_e1479: f64 = (noise_variable_52 * noise_metadata_schedule_131_e1478);
        let noise_metadata_schedule_131_e1480: f64 = (1.0 + noise_metadata_schedule_131_e1479);
        (noise_metadata_schedule_131_e1480,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_131_e1482;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_132_e1498,) = {
    if ((noise_variable_492 != 0.0) && (noise_variable_493 == 0.0)) {
        let noise_metadata_schedule_132_e1491: f64 = (-noise_variable_279);
        let noise_metadata_schedule_132_e1492: f64 = (noise_metadata_schedule_132_e1491).exp();
        let noise_metadata_schedule_132_e1493: f64 = (1.0 + noise_metadata_schedule_132_e1492);
        let noise_metadata_schedule_132_e1494: f64 = (noise_metadata_schedule_132_e1493).ln();
        let noise_metadata_schedule_132_e1495: f64 = (noise_variable_52 * noise_metadata_schedule_132_e1494);
        let noise_metadata_schedule_132_e1496: f64 = (noise_variable_50 + noise_metadata_schedule_132_e1495);
        (noise_metadata_schedule_132_e1496,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_132_e1498;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_133_e1506,) = {
    if (noise_variable_492 != 0.0) {
        let noise_metadata_schedule_133_e1503: f64 = (noise_variable_52 * 0.6931471805599453);
        let noise_metadata_schedule_133_e1504: f64 = (noise_variable_50 - noise_metadata_schedule_133_e1503);
        (noise_metadata_schedule_133_e1504,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_133_e1506;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_134_e1511,) = {
    if (noise_variable_492 == 0.0) {
        (params.p10,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_134_e1511;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_135_e1514: f64 = if params.p123 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_494 = noise_metadata_schedule_135_e1514;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_136_e1524,) = {
    if (noise_variable_494 != 0.0) {
        let noise_metadata_schedule_136_e1520: f64 = (noise_variable_12 * params.p123);
        let noise_metadata_schedule_136_e1521: f64 = (1.0 + noise_metadata_schedule_136_e1520);
        let noise_metadata_schedule_136_e1522: f64 = (params.p11 * noise_metadata_schedule_136_e1521);
        (noise_metadata_schedule_136_e1522,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_136_e1524;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_137_e1532,) = {
    if (noise_variable_494 != 0.0) {
        let noise_metadata_schedule_137_e1528: f64 = (noise_variable_51 - 1.0);
        let noise_metadata_schedule_137_e1530: f64 = (noise_metadata_schedule_137_e1528 / noise_variable_52);
        (noise_metadata_schedule_137_e1530,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_137_e1532;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_138_e1535: f64 = if noise_variable_51 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_495 = noise_metadata_schedule_138_e1535;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_139_e1549,) = {
    if ((noise_variable_494 != 0.0) && (noise_variable_495 != 0.0)) {
        let noise_metadata_schedule_139_e1543: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_139_e1544: f64 = (1.0 + noise_metadata_schedule_139_e1543);
        let noise_metadata_schedule_139_e1545: f64 = (noise_metadata_schedule_139_e1544).ln();
        let noise_metadata_schedule_139_e1546: f64 = (noise_variable_52 * noise_metadata_schedule_139_e1545);
        let noise_metadata_schedule_139_e1547: f64 = (1.0 + noise_metadata_schedule_139_e1546);
        (noise_metadata_schedule_139_e1547,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_139_e1549;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_140_e1565,) = {
    if ((noise_variable_494 != 0.0) && (noise_variable_495 == 0.0)) {
        let noise_metadata_schedule_140_e1558: f64 = (-noise_variable_279);
        let noise_metadata_schedule_140_e1559: f64 = (noise_metadata_schedule_140_e1558).exp();
        let noise_metadata_schedule_140_e1560: f64 = (1.0 + noise_metadata_schedule_140_e1559);
        let noise_metadata_schedule_140_e1561: f64 = (noise_metadata_schedule_140_e1560).ln();
        let noise_metadata_schedule_140_e1562: f64 = (noise_variable_52 * noise_metadata_schedule_140_e1561);
        let noise_metadata_schedule_140_e1563: f64 = (noise_variable_51 + noise_metadata_schedule_140_e1562);
        (noise_metadata_schedule_140_e1563,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_140_e1565;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_141_e1573,) = {
    if (noise_variable_494 != 0.0) {
        let noise_metadata_schedule_141_e1570: f64 = (noise_variable_52 * 0.6931471805599453);
        let noise_metadata_schedule_141_e1571: f64 = (noise_variable_51 - noise_metadata_schedule_141_e1570);
        (noise_metadata_schedule_141_e1571,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_141_e1573;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_142_e1578,) = {
    if (noise_variable_494 == 0.0) {
        (params.p11,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_142_e1578;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_143_e1583: f64 = (params.p124 * noise_variable_12);
            let noise_metadata_schedule_143_e1584: f64 = (1.0 + noise_metadata_schedule_143_e1583);
            let noise_metadata_schedule_143_e1585: f64 = (params.p43 * noise_metadata_schedule_143_e1584);
            noise_variable_335 = noise_metadata_schedule_143_e1585;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_144_e1588: f64 = (noise_variable_336 * noise_variable_336);
            noise_variable_281 = noise_metadata_schedule_144_e1588;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_145_e1591: f64 = (noise_variable_335 * noise_variable_335);
            noise_variable_282 = noise_metadata_schedule_145_e1591;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_146_e1594: f64 = if noise_variable_335 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_496 = noise_metadata_schedule_146_e1594;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_147_e1607,) = {
    if (noise_variable_496 != 0.0) {
        let noise_metadata_schedule_147_e1598: f64 = (0.5 * noise_variable_281);
        let noise_metadata_schedule_147_e1601: f64 = (noise_variable_282 + noise_variable_281);
        let noise_metadata_schedule_147_e1602: f64 = (noise_metadata_schedule_147_e1601).sqrt();
        let noise_metadata_schedule_147_e1604: f64 = (noise_metadata_schedule_147_e1602 - noise_variable_335);
        let noise_metadata_schedule_147_e1605: f64 = (noise_metadata_schedule_147_e1598 / noise_metadata_schedule_147_e1604);
        (noise_metadata_schedule_147_e1605,)
    } else {
        (noise_variable_334,)
    }
};
            noise_variable_334 = noise_metadata_schedule_147_e1607;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_148_e1619,) = {
    if (noise_variable_496 == 0.0) {
        let noise_metadata_schedule_148_e1613: f64 = (noise_variable_282 + noise_variable_281);
        let noise_metadata_schedule_148_e1614: f64 = (noise_metadata_schedule_148_e1613).sqrt();
        let noise_metadata_schedule_148_e1616: f64 = (noise_metadata_schedule_148_e1614 + noise_variable_335);
        let noise_metadata_schedule_148_e1617: f64 = (0.5 * noise_metadata_schedule_148_e1616);
        (noise_metadata_schedule_148_e1617,)
    } else {
        (noise_variable_334,)
    }
};
            noise_variable_334 = noise_metadata_schedule_148_e1619;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_149_e1624: f64 = (4.0 - params.p98);
            let noise_metadata_schedule_149_e1626: f64 = (noise_metadata_schedule_149_e1624 - params.p96);
            let noise_metadata_schedule_149_e1628: f64 = (noise_metadata_schedule_149_e1626 + params.p121);
            let noise_metadata_schedule_149_e1629: f64 = (noise_variable_274 * noise_metadata_schedule_149_e1628);
            let noise_metadata_schedule_149_e1631: f64 = (noise_metadata_schedule_149_e1629 / noise_variable_48);
            let noise_metadata_schedule_149_e1632: f64 = (noise_metadata_schedule_149_e1631).exp();
            let noise_metadata_schedule_149_e1633: f64 = (params.p9 * noise_metadata_schedule_149_e1632);
            let noise_metadata_schedule_149_e1635: f64 = (-params.p105);
            let noise_metadata_schedule_149_e1637: f64 = (noise_metadata_schedule_149_e1635 * noise_variable_10);
            let noise_metadata_schedule_149_e1639: f64 = (noise_metadata_schedule_149_e1637 / noise_variable_48);
            let noise_metadata_schedule_149_e1640: f64 = (noise_metadata_schedule_149_e1639).exp();
            let noise_metadata_schedule_149_e1641: f64 = (noise_metadata_schedule_149_e1633 * noise_metadata_schedule_149_e1640);
            noise_variable_35 = noise_metadata_schedule_149_e1641;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_150_e1646: f64 = (1.0 - params.p98);
            let noise_metadata_schedule_150_e1647: f64 = (noise_variable_274 * noise_metadata_schedule_150_e1646);
            let noise_metadata_schedule_150_e1648: f64 = (noise_metadata_schedule_150_e1647).exp();
            let noise_metadata_schedule_150_e1649: f64 = (params.p12 * noise_metadata_schedule_150_e1648);
            noise_variable_36 = noise_metadata_schedule_150_e1649;
        }
        if matches!(source_index, 11 | 12 | 13 | 14 | 19) {
            let noise_metadata_schedule_151_e1654: f64 = (1.0 - params.p103);
            let noise_metadata_schedule_151_e1655: f64 = (noise_variable_274 * noise_metadata_schedule_151_e1654);
            let noise_metadata_schedule_151_e1656: f64 = (noise_metadata_schedule_151_e1655).exp();
            let noise_metadata_schedule_151_e1657: f64 = (params.p30 * noise_metadata_schedule_151_e1656);
            noise_variable_37 = noise_metadata_schedule_151_e1657;
        }
        if matches!(source_index, 2 | 7) {
            let noise_metadata_schedule_152_e1663: f64 = (2.0 * params.p21);
            let noise_metadata_schedule_152_e1664: f64 = (6.0 - noise_metadata_schedule_152_e1663);
            let noise_metadata_schedule_152_e1665: f64 = (noise_variable_274 * noise_metadata_schedule_152_e1664);
            let noise_metadata_schedule_152_e1666: f64 = (noise_metadata_schedule_152_e1665).exp();
            let noise_metadata_schedule_152_e1667: f64 = (params.p20 * noise_metadata_schedule_152_e1666);
            let noise_metadata_schedule_152_e1669: f64 = (-params.p113);
            let noise_metadata_schedule_152_e1671: f64 = (noise_metadata_schedule_152_e1669 * noise_variable_10);
            let noise_metadata_schedule_152_e1673: f64 = (noise_metadata_schedule_152_e1671 / params.p21);
            let noise_metadata_schedule_152_e1674: f64 = (noise_metadata_schedule_152_e1673).exp();
            let noise_metadata_schedule_152_e1675: f64 = (noise_metadata_schedule_152_e1667 * noise_metadata_schedule_152_e1674);
            noise_variable_38 = noise_metadata_schedule_152_e1675;
        }
        if matches!(source_index, 9 | 10) {
            let noise_metadata_schedule_153_e1681: f64 = (2.0 * params.p32);
            let noise_metadata_schedule_153_e1682: f64 = (6.0 - noise_metadata_schedule_153_e1681);
            let noise_metadata_schedule_153_e1683: f64 = (noise_variable_274 * noise_metadata_schedule_153_e1682);
            let noise_metadata_schedule_153_e1684: f64 = (noise_metadata_schedule_153_e1683).exp();
            let noise_metadata_schedule_153_e1685: f64 = (params.p31 * noise_metadata_schedule_153_e1684);
            let noise_metadata_schedule_153_e1687: f64 = (-params.p110);
            let noise_metadata_schedule_153_e1689: f64 = (noise_metadata_schedule_153_e1687 * noise_variable_10);
            let noise_metadata_schedule_153_e1691: f64 = (noise_metadata_schedule_153_e1689 / params.p32);
            let noise_metadata_schedule_153_e1692: f64 = (noise_metadata_schedule_153_e1691).exp();
            let noise_metadata_schedule_153_e1693: f64 = (noise_metadata_schedule_153_e1685 * noise_metadata_schedule_153_e1692);
            noise_variable_39 = noise_metadata_schedule_153_e1693;
        }
        if matches!(source_index, 1 | 2 | 6) {
            let noise_metadata_schedule_154_e1698: f64 = (4.0 - params.p97);
            let noise_metadata_schedule_154_e1700: f64 = (noise_metadata_schedule_154_e1698 + params.p121);
            let noise_metadata_schedule_154_e1701: f64 = (noise_variable_274 * noise_metadata_schedule_154_e1700);
            let noise_metadata_schedule_154_e1703: f64 = (noise_metadata_schedule_154_e1701 / params.p17);
            let noise_metadata_schedule_154_e1704: f64 = (noise_metadata_schedule_154_e1703).exp();
            let noise_metadata_schedule_154_e1705: f64 = (params.p16 * noise_metadata_schedule_154_e1704);
            let noise_metadata_schedule_154_e1707: f64 = (-params.p111);
            let noise_metadata_schedule_154_e1709: f64 = (noise_metadata_schedule_154_e1707 * noise_variable_10);
            let noise_metadata_schedule_154_e1711: f64 = (noise_metadata_schedule_154_e1709 / params.p17);
            let noise_metadata_schedule_154_e1712: f64 = (noise_metadata_schedule_154_e1711).exp();
            let noise_metadata_schedule_154_e1713: f64 = (noise_metadata_schedule_154_e1705 * noise_metadata_schedule_154_e1712);
            noise_variable_42 = noise_metadata_schedule_154_e1713;
        }
        if matches!(source_index, 6 | 8) {
            let noise_metadata_schedule_155_e1718: f64 = (4.0 - params.p97);
            let noise_metadata_schedule_155_e1720: f64 = (noise_metadata_schedule_155_e1718 + params.p121);
            let noise_metadata_schedule_155_e1721: f64 = (noise_variable_274 * noise_metadata_schedule_155_e1720);
            let noise_metadata_schedule_155_e1723: f64 = (noise_metadata_schedule_155_e1721 / params.p19);
            let noise_metadata_schedule_155_e1724: f64 = (noise_metadata_schedule_155_e1723).exp();
            let noise_metadata_schedule_155_e1725: f64 = (params.p18 * noise_metadata_schedule_155_e1724);
            let noise_metadata_schedule_155_e1727: f64 = (-params.p111);
            let noise_metadata_schedule_155_e1729: f64 = (noise_metadata_schedule_155_e1727 * noise_variable_10);
            let noise_metadata_schedule_155_e1731: f64 = (noise_metadata_schedule_155_e1729 / params.p19);
            let noise_metadata_schedule_155_e1732: f64 = (noise_metadata_schedule_155_e1731).exp();
            let noise_metadata_schedule_155_e1733: f64 = (noise_metadata_schedule_155_e1725 * noise_metadata_schedule_155_e1732);
            noise_variable_44 = noise_metadata_schedule_155_e1733;
        }
        if matches!(source_index, 2 | 6 | 8) {
            let noise_metadata_schedule_156_e1736: f64 = if params.p24 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_497 = noise_metadata_schedule_156_e1736;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_157_e1748,) = {
    if (noise_variable_497 != 0.0) {
        let noise_metadata_schedule_157_e1740: f64 = (-params.p107);
        let noise_metadata_schedule_157_e1742: f64 = (noise_metadata_schedule_157_e1740 * noise_variable_10);
        let noise_metadata_schedule_157_e1744: f64 = (noise_metadata_schedule_157_e1742 / params.p17);
        let noise_metadata_schedule_157_e1745: f64 = (noise_metadata_schedule_157_e1744).exp();
        let noise_metadata_schedule_157_e1746: f64 = (params.p25 * noise_metadata_schedule_157_e1745);
        (noise_metadata_schedule_157_e1746,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_157_e1748;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_158_e1758,) = {
    if (noise_variable_497 != 0.0) {
        let noise_metadata_schedule_158_e1752: f64 = (-params.p106);
        let noise_metadata_schedule_158_e1754: f64 = (noise_metadata_schedule_158_e1752 * noise_variable_10);
        let noise_metadata_schedule_158_e1755: f64 = (noise_metadata_schedule_158_e1754).exp();
        let noise_metadata_schedule_158_e1756: f64 = (params.p28 * noise_metadata_schedule_158_e1755);
        (noise_metadata_schedule_158_e1756,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_158_e1758;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_159_e1770,) = {
    if (noise_variable_497 != 0.0) {
        let noise_metadata_schedule_159_e1762: f64 = (-params.p108);
        let noise_metadata_schedule_159_e1764: f64 = (noise_metadata_schedule_159_e1762 * noise_variable_10);
        let noise_metadata_schedule_159_e1766: f64 = (noise_metadata_schedule_159_e1764 / params.p19);
        let noise_metadata_schedule_159_e1767: f64 = (noise_metadata_schedule_159_e1766).exp();
        let noise_metadata_schedule_159_e1768: f64 = (params.p26 * noise_metadata_schedule_159_e1767);
        (noise_metadata_schedule_159_e1768,)
    } else {
        (noise_variable_45,)
    }
};
            noise_variable_45 = noise_metadata_schedule_159_e1770;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_160_e1775: f64 = (4.0 - params.p103);
            let noise_metadata_schedule_160_e1777: f64 = (noise_metadata_schedule_160_e1775 + params.p121);
            let noise_metadata_schedule_160_e1778: f64 = (noise_variable_274 * noise_metadata_schedule_160_e1777);
            let noise_metadata_schedule_160_e1779: f64 = (noise_metadata_schedule_160_e1778).exp();
            let noise_metadata_schedule_160_e1780: f64 = (params.p29 * noise_metadata_schedule_160_e1779);
            let noise_metadata_schedule_160_e1782: f64 = (-params.p112);
            let noise_metadata_schedule_160_e1784: f64 = (noise_metadata_schedule_160_e1782 * noise_variable_10);
            let noise_metadata_schedule_160_e1785: f64 = (noise_metadata_schedule_160_e1784).exp();
            let noise_metadata_schedule_160_e1786: f64 = (noise_metadata_schedule_160_e1780 * noise_metadata_schedule_160_e1785);
            noise_variable_43 = noise_metadata_schedule_160_e1786;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_161_e1792: f64 = (2.0 * params.p23);
            let noise_metadata_schedule_161_e1793: f64 = (6.0 - noise_metadata_schedule_161_e1792);
            let noise_metadata_schedule_161_e1794: f64 = (noise_variable_274 * noise_metadata_schedule_161_e1793);
            let noise_metadata_schedule_161_e1795: f64 = (noise_metadata_schedule_161_e1794).exp();
            let noise_metadata_schedule_161_e1796: f64 = (params.p22 * noise_metadata_schedule_161_e1795);
            let noise_metadata_schedule_161_e1798: f64 = (-params.p113);
            let noise_metadata_schedule_161_e1800: f64 = (noise_metadata_schedule_161_e1798 * noise_variable_10);
            let noise_metadata_schedule_161_e1802: f64 = (noise_metadata_schedule_161_e1800 / params.p23);
            let noise_metadata_schedule_161_e1803: f64 = (noise_metadata_schedule_161_e1802).exp();
            let noise_metadata_schedule_161_e1804: f64 = (noise_metadata_schedule_161_e1796 * noise_metadata_schedule_161_e1803);
            noise_variable_46 = noise_metadata_schedule_161_e1804;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_162_e1809: f64 = (4.0 / params.p146);
            let noise_metadata_schedule_162_e1810: f64 = (noise_variable_274 * noise_metadata_schedule_162_e1809);
            let noise_metadata_schedule_162_e1811: f64 = (noise_metadata_schedule_162_e1810).exp();
            let noise_metadata_schedule_162_e1812: f64 = (params.p145 * noise_metadata_schedule_162_e1811);
            let noise_metadata_schedule_162_e1814: f64 = (-params.p113);
            let noise_metadata_schedule_162_e1816: f64 = (noise_metadata_schedule_162_e1814 * noise_variable_10);
            let noise_metadata_schedule_162_e1818: f64 = (noise_metadata_schedule_162_e1816 / params.p146);
            let noise_metadata_schedule_162_e1819: f64 = (noise_metadata_schedule_162_e1818).exp();
            let noise_metadata_schedule_162_e1820: f64 = (noise_metadata_schedule_162_e1812 * noise_metadata_schedule_162_e1819);
            noise_variable_47 = noise_metadata_schedule_162_e1820;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_163_e1823: f64 = (noise_variable_4).sqrt();
            let noise_metadata_schedule_163_e1824: f64 = (params.p151 * noise_metadata_schedule_163_e1823);
            let noise_metadata_schedule_163_e1827: f64 = (params.p153 * noise_variable_12);
            let noise_metadata_schedule_163_e1828: f64 = (noise_metadata_schedule_163_e1827).exp();
            let noise_metadata_schedule_163_e1829: f64 = (noise_metadata_schedule_163_e1824 * noise_metadata_schedule_163_e1828);
            noise_variable_350 = noise_metadata_schedule_163_e1829;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_164_e1832: f64 = (noise_variable_70 * noise_variable_72);
            let noise_metadata_schedule_164_e1834: f64 = (-0.5);
            let noise_metadata_schedule_164_e1835: f64 = (noise_metadata_schedule_164_e1832).powf(noise_metadata_schedule_164_e1834);
            noise_variable_275 = noise_metadata_schedule_164_e1835;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_165_e1838: f64 = (1.0 / noise_variable_73);
            noise_variable_276 = noise_metadata_schedule_165_e1838;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_166_e1841: f64 = (params.p35 * noise_variable_70);
            let noise_metadata_schedule_166_e1843: f64 = (noise_metadata_schedule_166_e1841 * noise_variable_70);
            let noise_metadata_schedule_166_e1845: f64 = (noise_metadata_schedule_166_e1843 * noise_variable_275);
            let noise_metadata_schedule_166_e1847: f64 = (noise_metadata_schedule_166_e1845 * noise_variable_276);
            let noise_metadata_schedule_166_e1849: f64 = (noise_metadata_schedule_166_e1847 * params.p66);
            let noise_metadata_schedule_166_e1851: f64 = (noise_metadata_schedule_166_e1849 * noise_variable_65);
            let noise_metadata_schedule_166_e1853: f64 = (noise_metadata_schedule_166_e1851 * noise_variable_72);
            let noise_metadata_schedule_166_e1855: f64 = (noise_metadata_schedule_166_e1853 * noise_variable_72);
            noise_variable_61 = noise_metadata_schedule_166_e1855;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_167_e1858: f64 = (params.p34 * noise_variable_275);
            let noise_metadata_schedule_167_e1860: f64 = (noise_metadata_schedule_167_e1858 * noise_variable_14);
            let noise_metadata_schedule_167_e1862: f64 = (noise_metadata_schedule_167_e1860 * noise_variable_14);
            let noise_metadata_schedule_167_e1864: f64 = (noise_metadata_schedule_167_e1862 * noise_variable_64);
            let noise_metadata_schedule_167_e1866: f64 = (noise_metadata_schedule_167_e1864 * noise_variable_64);
            let noise_metadata_schedule_167_e1868: f64 = (noise_metadata_schedule_167_e1866 * noise_variable_73);
            let noise_metadata_schedule_167_e1871: f64 = (params.p35 - noise_variable_61);
            let noise_metadata_schedule_167_e1872: f64 = (noise_metadata_schedule_167_e1871).exp();
            let noise_metadata_schedule_167_e1873: f64 = (noise_metadata_schedule_167_e1868 * noise_metadata_schedule_167_e1872);
            noise_variable_58 = noise_metadata_schedule_167_e1873;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_168_e1876: f64 = (1.0 / noise_variable_19);
            noise_variable_67 = noise_metadata_schedule_168_e1876;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_169_e1879: f64 = (noise_variable_85 * noise_variable_86);
            let noise_metadata_schedule_169_e1881: f64 = (-0.5);
            let noise_metadata_schedule_169_e1882: f64 = (noise_metadata_schedule_169_e1879).powf(noise_metadata_schedule_169_e1881);
            noise_variable_277 = noise_metadata_schedule_169_e1882;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_170_e1885: f64 = (1.0 / noise_variable_90);
            noise_variable_278 = noise_metadata_schedule_170_e1885;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_171_e1888: f64 = (params.p37 * noise_variable_85);
            let noise_metadata_schedule_171_e1890: f64 = (noise_metadata_schedule_171_e1888 * noise_variable_85);
            let noise_metadata_schedule_171_e1892: f64 = (noise_metadata_schedule_171_e1890 * noise_variable_277);
            let noise_metadata_schedule_171_e1894: f64 = (noise_metadata_schedule_171_e1892 * noise_variable_278);
            let noise_metadata_schedule_171_e1896: f64 = (noise_metadata_schedule_171_e1894 * noise_variable_75);
            let noise_metadata_schedule_171_e1898: f64 = (noise_metadata_schedule_171_e1896 * noise_variable_67);
            let noise_metadata_schedule_171_e1900: f64 = (noise_metadata_schedule_171_e1898 * noise_variable_86);
            let noise_metadata_schedule_171_e1902: f64 = (noise_metadata_schedule_171_e1900 * noise_variable_86);
            noise_variable_83 = noise_metadata_schedule_171_e1902;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_172_e1905: f64 = (params.p36 * noise_variable_277);
            let noise_metadata_schedule_172_e1907: f64 = (noise_metadata_schedule_172_e1905 * noise_variable_19);
            let noise_metadata_schedule_172_e1909: f64 = (noise_metadata_schedule_172_e1907 * noise_variable_19);
            let noise_metadata_schedule_172_e1911: f64 = (noise_metadata_schedule_172_e1909 * noise_variable_66);
            let noise_metadata_schedule_172_e1913: f64 = (noise_metadata_schedule_172_e1911 * noise_variable_66);
            let noise_metadata_schedule_172_e1915: f64 = (noise_metadata_schedule_172_e1913 * noise_variable_90);
            let noise_metadata_schedule_172_e1918: f64 = (params.p37 - noise_variable_83);
            let noise_metadata_schedule_172_e1919: f64 = (noise_metadata_schedule_172_e1918).exp();
            let noise_metadata_schedule_172_e1920: f64 = (noise_metadata_schedule_172_e1915 * noise_metadata_schedule_172_e1919);
            noise_variable_84 = noise_metadata_schedule_172_e1920;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_173_e1923: f64 = (noise_variable_274 * params.p96);
            let noise_metadata_schedule_173_e1924: f64 = (noise_metadata_schedule_173_e1923).exp();
            noise_variable_275 = noise_metadata_schedule_173_e1924;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_174_e1927: f64 = (params.p14 * noise_variable_275);
            let noise_metadata_schedule_174_e1929: f64 = (noise_metadata_schedule_174_e1927 * noise_variable_27);
            noise_variable_40 = noise_metadata_schedule_174_e1929;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_175_e1932: f64 = (params.p13 * noise_variable_275);
            let noise_metadata_schedule_175_e1934: f64 = (noise_metadata_schedule_175_e1932 * noise_variable_276);
            noise_variable_41 = noise_metadata_schedule_175_e1934;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_176_e1939: f64 = (4.0 - params.p141);
            let noise_metadata_schedule_176_e1940: f64 = (noise_variable_274 * noise_metadata_schedule_176_e1939);
            let noise_metadata_schedule_176_e1941: f64 = (noise_metadata_schedule_176_e1940).exp();
            let noise_metadata_schedule_176_e1942: f64 = (params.p133 * noise_metadata_schedule_176_e1941);
            let noise_metadata_schedule_176_e1944: f64 = (-params.p140);
            let noise_metadata_schedule_176_e1946: f64 = (noise_metadata_schedule_176_e1944 * noise_variable_10);
            let noise_metadata_schedule_176_e1947: f64 = (noise_metadata_schedule_176_e1946).exp();
            let noise_metadata_schedule_176_e1948: f64 = (noise_metadata_schedule_176_e1942 * noise_metadata_schedule_176_e1947);
            noise_variable_104 = noise_metadata_schedule_176_e1948;
        }
        if matches!(source_index, 13 | 14 | 17 | 18 | 19) {
            let noise_metadata_schedule_178_e1969: f64 = (1.0 - params.p141);
            let noise_metadata_schedule_178_e1970: f64 = (noise_variable_274 * noise_metadata_schedule_178_e1969);
            let noise_metadata_schedule_178_e1971: f64 = (noise_metadata_schedule_178_e1970).exp();
            let noise_metadata_schedule_178_e1972: f64 = (params.p135 * noise_metadata_schedule_178_e1971);
            noise_variable_106 = noise_metadata_schedule_178_e1972;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_185_e2032: f64 = (noise_variable_2 - 300.0);
            noise_variable_100 = noise_metadata_schedule_185_e2032;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_186_e2035: f64 = if noise_variable_2 < 525.0 { 1.0 } else { 0.0 };
            noise_variable_498 = noise_metadata_schedule_186_e2035;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_187_e2051,) = {
    if (noise_variable_498 != 0.0) {
        let noise_metadata_schedule_187_e2041: f64 = (0.00072 * noise_variable_100);
        let noise_metadata_schedule_187_e2042: f64 = (1.0 + noise_metadata_schedule_187_e2041);
        let noise_metadata_schedule_187_e2045: f64 = (1.6e-6 * noise_variable_100);
        let noise_metadata_schedule_187_e2047: f64 = (noise_metadata_schedule_187_e2045 * noise_variable_100);
        let noise_metadata_schedule_187_e2048: f64 = (noise_metadata_schedule_187_e2042 - noise_metadata_schedule_187_e2047);
        let noise_metadata_schedule_187_e2049: f64 = (noise_variable_1 * noise_metadata_schedule_187_e2048);
        (noise_metadata_schedule_187_e2049,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_187_e2051;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_188_e2058,) = {
    if (noise_variable_498 == 0.0) {
        let noise_metadata_schedule_188_e2056: f64 = (noise_variable_1 * 1.081);
        (noise_metadata_schedule_188_e2056,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_188_e2058;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_189_e2062: f64 = (noise_variable_274 * params.p96);
            let noise_metadata_schedule_189_e2063: f64 = (noise_metadata_schedule_189_e2062).exp();
            let noise_metadata_schedule_189_e2064: f64 = (params.p92 * noise_metadata_schedule_189_e2063);
            noise_variable_99 = noise_metadata_schedule_189_e2064;
        }
        if matches!(source_index, 20 | 23 | 25 | 27) {
            let noise_metadata_schedule_190_e2067: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_499 = noise_metadata_schedule_190_e2067;
        }
        if matches!(source_index, 20 | 23 | 25 | 27) {
            let (noise_metadata_schedule_191_e2073,) = {
    if (noise_variable_499 != 0.0) {
        let noise_metadata_schedule_191_e2071: f64 = (1.0 / noise_variable_32);
        (noise_metadata_schedule_191_e2071,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_191_e2073;
        }
        if matches!(source_index, 20 | 23 | 25 | 27) {
            let noise_metadata_schedule_192_e2076: f64 = if noise_variable_108 > noise_variable_341 { 1.0 } else { 0.0 };
            noise_variable_500 = noise_metadata_schedule_192_e2076;
        }
        if matches!(source_index, 20 | 23 | 25 | 27) {
            let (noise_metadata_schedule_193_e2082,) = {
    if ((noise_variable_499 != 0.0) && (noise_variable_500 != 0.0)) {
        (noise_variable_341,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_193_e2082;
        }
        if matches!(source_index, 20 | 23 | 25 | 27) {
            let (noise_metadata_schedule_194_e2087,) = {
    if (noise_variable_499 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_194_e2087;
        }
        if matches!(source_index, 21 | 24) {
            let noise_metadata_schedule_195_e2090: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_501 = noise_metadata_schedule_195_e2090;
        }
        if matches!(source_index, 21 | 24) {
            let (noise_metadata_schedule_196_e2096,) = {
    if (noise_variable_501 != 0.0) {
        let noise_metadata_schedule_196_e2094: f64 = (1.0 / noise_variable_33);
        (noise_metadata_schedule_196_e2094,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_196_e2096;
        }
        if matches!(source_index, 21 | 24) {
            let noise_metadata_schedule_197_e2099: f64 = if noise_variable_109 > noise_variable_341 { 1.0 } else { 0.0 };
            noise_variable_502 = noise_metadata_schedule_197_e2099;
        }
        if matches!(source_index, 21 | 24) {
            let (noise_metadata_schedule_198_e2105,) = {
    if ((noise_variable_501 != 0.0) && (noise_variable_502 != 0.0)) {
        (noise_variable_341,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_198_e2105;
        }
        if matches!(source_index, 21 | 24) {
            let (noise_metadata_schedule_199_e2110,) = {
    if (noise_variable_501 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_199_e2110;
        }
        if matches!(source_index, 22 | 26) {
            let noise_metadata_schedule_200_e2113: f64 = if params.p59 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_503 = noise_metadata_schedule_200_e2113;
        }
        if matches!(source_index, 22 | 26) {
            let (noise_metadata_schedule_201_e2119,) = {
    if (noise_variable_503 != 0.0) {
        let noise_metadata_schedule_201_e2117: f64 = (1.0 / noise_variable_34);
        (noise_metadata_schedule_201_e2117,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_201_e2119;
        }
        if matches!(source_index, 22 | 26) {
            let noise_metadata_schedule_202_e2122: f64 = if noise_variable_110 > noise_variable_341 { 1.0 } else { 0.0 };
            noise_variable_504 = noise_metadata_schedule_202_e2122;
        }
        if matches!(source_index, 22 | 26) {
            let (noise_metadata_schedule_203_e2128,) = {
    if ((noise_variable_503 != 0.0) && (noise_variable_504 != 0.0)) {
        (noise_variable_341,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_203_e2128;
        }
        if matches!(source_index, 22 | 26) {
            let (noise_metadata_schedule_204_e2133,) = {
    if (noise_variable_503 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_204_e2133;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_205_e2136: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[7])));
            noise_variable_244 = noise_metadata_schedule_205_e2136;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_206_e2139: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[8])));
            noise_variable_245 = noise_metadata_schedule_206_e2139;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_207_e2142: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[4])));
            noise_variable_246 = noise_metadata_schedule_207_e2142;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_208_e2145: f64 = (params.p3 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[4])));
            noise_variable_247 = noise_metadata_schedule_208_e2145;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_209_e2148: f64 = (params.p3 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[6])));
            noise_variable_248 = noise_metadata_schedule_209_e2148;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_210_e2151: f64 = (params.p3 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[7])));
            noise_variable_253 = noise_metadata_schedule_210_e2151;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_211_e2154: f64 = (params.p3 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[8])));
            noise_variable_250 = noise_metadata_schedule_211_e2154;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_213_e2160: f64 = (params.p3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_260 = noise_metadata_schedule_213_e2160;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_215_e2166: f64 = (params.p3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[0])));
            noise_variable_264 = noise_metadata_schedule_215_e2166;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_216_e2169: f64 = (params.p3 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[7])));
            noise_variable_252 = noise_metadata_schedule_216_e2169;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_217_e2172: f64 = (params.p3 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
            noise_variable_251 = noise_metadata_schedule_217_e2172;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_218_e2175: f64 = (noise_variable_248 + noise_variable_245);
            let noise_metadata_schedule_218_e2177: f64 = (noise_metadata_schedule_218_e2175 - noise_variable_250);
            let noise_metadata_schedule_218_e2179: f64 = (noise_metadata_schedule_218_e2177 - noise_variable_252);
            noise_variable_249 = noise_metadata_schedule_218_e2179;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_219_e2181: f64 = (-noise_variable_264);
            let noise_metadata_schedule_219_e2183: f64 = (noise_metadata_schedule_219_e2181 + noise_variable_260);
            let noise_metadata_schedule_219_e2185: f64 = (noise_metadata_schedule_219_e2183 + noise_variable_249);
            let noise_metadata_schedule_219_e2187: f64 = (noise_metadata_schedule_219_e2185 - noise_variable_251);
            noise_variable_262 = noise_metadata_schedule_219_e2187;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_220_e2190: f64 = (noise_variable_264 + noise_variable_262);
            noise_variable_261 = noise_metadata_schedule_220_e2190;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let noise_metadata_schedule_221_e2193: f64 = (noise_variable_253 - noise_variable_252);
            noise_variable_255 = noise_metadata_schedule_221_e2193;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let noise_metadata_schedule_222_e2196: f64 = (noise_variable_255 - noise_variable_251);
            noise_variable_254 = noise_metadata_schedule_222_e2196;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_223_e2199: f64 = (noise_variable_245 * noise_variable_8);
            let noise_metadata_schedule_223_e2201: f64 = if noise_metadata_schedule_223_e2199 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_505 = noise_metadata_schedule_223_e2201;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16 | 17) {
            let (noise_metadata_schedule_224_e2208,) = {
    if (noise_variable_505 != 0.0) {
        let noise_metadata_schedule_224_e2205: f64 = (noise_variable_245 * noise_variable_8);
        let noise_metadata_schedule_224_e2206: f64 = (noise_metadata_schedule_224_e2205).exp();
        (noise_metadata_schedule_224_e2206,)
    } else {
        (noise_variable_265,)
    }
};
            noise_variable_265 = noise_metadata_schedule_224_e2208;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_225_e2214,) = {
    if (noise_variable_505 == 0.0) {
        let noise_metadata_schedule_225_e2212: f64 = (params.p147).exp();
        (noise_metadata_schedule_225_e2212,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_225_e2214;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16 | 17) {
            let (noise_metadata_schedule_226_e2227,) = {
    if (noise_variable_505 == 0.0) {
        let noise_metadata_schedule_226_e2221: f64 = (noise_variable_245 * noise_variable_8);
        let noise_metadata_schedule_226_e2223: f64 = (noise_metadata_schedule_226_e2221 - params.p147);
        let noise_metadata_schedule_226_e2224: f64 = (1.0 + noise_metadata_schedule_226_e2223);
        let noise_metadata_schedule_226_e2225: f64 = (noise_variable_295 * noise_metadata_schedule_226_e2224);
        (noise_metadata_schedule_226_e2225,)
    } else {
        (noise_variable_265,)
    }
};
            noise_variable_265 = noise_metadata_schedule_226_e2227;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_227_e2230: f64 = (noise_variable_246 * noise_variable_8);
            let noise_metadata_schedule_227_e2232: f64 = (noise_metadata_schedule_227_e2230 / noise_variable_48);
            let noise_metadata_schedule_227_e2234: f64 = if noise_metadata_schedule_227_e2232 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_506 = noise_metadata_schedule_227_e2234;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_228_e2243,) = {
    if (noise_variable_506 != 0.0) {
        let noise_metadata_schedule_228_e2238: f64 = (noise_variable_246 * noise_variable_8);
        let noise_metadata_schedule_228_e2240: f64 = (noise_metadata_schedule_228_e2238 / noise_variable_48);
        let noise_metadata_schedule_228_e2241: f64 = (noise_metadata_schedule_228_e2240).exp();
        (noise_metadata_schedule_228_e2241,)
    } else {
        (noise_variable_266,)
    }
};
            noise_variable_266 = noise_metadata_schedule_228_e2243;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_229_e2249,) = {
    if (noise_variable_506 == 0.0) {
        let noise_metadata_schedule_229_e2247: f64 = (params.p147).exp();
        (noise_metadata_schedule_229_e2247,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_229_e2249;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_230_e2264,) = {
    if (noise_variable_506 == 0.0) {
        let noise_metadata_schedule_230_e2256: f64 = (noise_variable_246 * noise_variable_8);
        let noise_metadata_schedule_230_e2258: f64 = (noise_metadata_schedule_230_e2256 / noise_variable_48);
        let noise_metadata_schedule_230_e2260: f64 = (noise_metadata_schedule_230_e2258 - params.p147);
        let noise_metadata_schedule_230_e2261: f64 = (1.0 + noise_metadata_schedule_230_e2260);
        let noise_metadata_schedule_230_e2262: f64 = (noise_variable_295 * noise_metadata_schedule_230_e2261);
        (noise_metadata_schedule_230_e2262,)
    } else {
        (noise_variable_266,)
    }
};
            noise_variable_266 = noise_metadata_schedule_230_e2264;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_231_e2267: f64 = (noise_variable_249 * noise_variable_8);
            let noise_metadata_schedule_231_e2269: f64 = if noise_metadata_schedule_231_e2267 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_507 = noise_metadata_schedule_231_e2269;
        }
        if matches!(source_index, 11 | 12 | 18) {
            let (noise_metadata_schedule_232_e2276,) = {
    if (noise_variable_507 != 0.0) {
        let noise_metadata_schedule_232_e2273: f64 = (noise_variable_249 * noise_variable_8);
        let noise_metadata_schedule_232_e2274: f64 = (noise_metadata_schedule_232_e2273).exp();
        (noise_metadata_schedule_232_e2274,)
    } else {
        (noise_variable_268,)
    }
};
            noise_variable_268 = noise_metadata_schedule_232_e2276;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_233_e2282,) = {
    if (noise_variable_507 == 0.0) {
        let noise_metadata_schedule_233_e2280: f64 = (params.p147).exp();
        (noise_metadata_schedule_233_e2280,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_233_e2282;
        }
        if matches!(source_index, 11 | 12 | 18) {
            let (noise_metadata_schedule_234_e2295,) = {
    if (noise_variable_507 == 0.0) {
        let noise_metadata_schedule_234_e2289: f64 = (noise_variable_249 * noise_variable_8);
        let noise_metadata_schedule_234_e2291: f64 = (noise_metadata_schedule_234_e2289 - params.p147);
        let noise_metadata_schedule_234_e2292: f64 = (1.0 + noise_metadata_schedule_234_e2291);
        let noise_metadata_schedule_234_e2293: f64 = (noise_variable_295 * noise_metadata_schedule_234_e2292);
        (noise_metadata_schedule_234_e2293,)
    } else {
        (noise_variable_268,)
    }
};
            noise_variable_268 = noise_metadata_schedule_234_e2295;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_235_e2298: f64 = (noise_variable_248 * noise_variable_8);
            let noise_metadata_schedule_235_e2300: f64 = if noise_metadata_schedule_235_e2298 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_508 = noise_metadata_schedule_235_e2300;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_236_e2307,) = {
    if (noise_variable_508 != 0.0) {
        let noise_metadata_schedule_236_e2304: f64 = (noise_variable_248 * noise_variable_8);
        let noise_metadata_schedule_236_e2305: f64 = (noise_metadata_schedule_236_e2304).exp();
        (noise_metadata_schedule_236_e2305,)
    } else {
        (noise_variable_267,)
    }
};
            noise_variable_267 = noise_metadata_schedule_236_e2307;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_237_e2313,) = {
    if (noise_variable_508 == 0.0) {
        let noise_metadata_schedule_237_e2311: f64 = (params.p147).exp();
        (noise_metadata_schedule_237_e2311,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_237_e2313;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_238_e2326,) = {
    if (noise_variable_508 == 0.0) {
        let noise_metadata_schedule_238_e2320: f64 = (noise_variable_248 * noise_variable_8);
        let noise_metadata_schedule_238_e2322: f64 = (noise_metadata_schedule_238_e2320 - params.p147);
        let noise_metadata_schedule_238_e2323: f64 = (1.0 + noise_metadata_schedule_238_e2322);
        let noise_metadata_schedule_238_e2324: f64 = (noise_variable_295 * noise_metadata_schedule_238_e2323);
        (noise_metadata_schedule_238_e2324,)
    } else {
        (noise_variable_267,)
    }
};
            noise_variable_267 = noise_metadata_schedule_238_e2326;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_239_e2329: f64 = (noise_variable_261 * noise_variable_8);
            let noise_metadata_schedule_239_e2331: f64 = if noise_metadata_schedule_239_e2329 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_509 = noise_metadata_schedule_239_e2331;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_240_e2338,) = {
    if (noise_variable_509 != 0.0) {
        let noise_metadata_schedule_240_e2335: f64 = (noise_variable_261 * noise_variable_8);
        let noise_metadata_schedule_240_e2336: f64 = (noise_metadata_schedule_240_e2335).exp();
        (noise_metadata_schedule_240_e2336,)
    } else {
        (noise_variable_269,)
    }
};
            noise_variable_269 = noise_metadata_schedule_240_e2338;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_241_e2344,) = {
    if (noise_variable_509 == 0.0) {
        let noise_metadata_schedule_241_e2342: f64 = (params.p147).exp();
        (noise_metadata_schedule_241_e2342,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_241_e2344;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_242_e2357,) = {
    if (noise_variable_509 == 0.0) {
        let noise_metadata_schedule_242_e2351: f64 = (noise_variable_261 * noise_variable_8);
        let noise_metadata_schedule_242_e2353: f64 = (noise_metadata_schedule_242_e2351 - params.p147);
        let noise_metadata_schedule_242_e2354: f64 = (1.0 + noise_metadata_schedule_242_e2353);
        let noise_metadata_schedule_242_e2355: f64 = (noise_variable_295 * noise_metadata_schedule_242_e2354);
        (noise_metadata_schedule_242_e2355,)
    } else {
        (noise_variable_269,)
    }
};
            noise_variable_269 = noise_metadata_schedule_242_e2357;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let noise_metadata_schedule_243_e2360: f64 = (noise_variable_253 * noise_variable_8);
            let noise_metadata_schedule_243_e2362: f64 = if noise_metadata_schedule_243_e2360 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_510 = noise_metadata_schedule_243_e2362;
        }
        if matches!(source_index, 17) {
            let (noise_metadata_schedule_244_e2369,) = {
    if (noise_variable_510 != 0.0) {
        let noise_metadata_schedule_244_e2366: f64 = (noise_variable_253 * noise_variable_8);
        let noise_metadata_schedule_244_e2367: f64 = (noise_metadata_schedule_244_e2366).exp();
        (noise_metadata_schedule_244_e2367,)
    } else {
        (noise_variable_256,)
    }
};
            noise_variable_256 = noise_metadata_schedule_244_e2369;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) {
            let (noise_metadata_schedule_245_e2375,) = {
    if (noise_variable_510 == 0.0) {
        let noise_metadata_schedule_245_e2373: f64 = (params.p147).exp();
        (noise_metadata_schedule_245_e2373,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_245_e2375;
        }
        if matches!(source_index, 17) {
            let (noise_metadata_schedule_246_e2388,) = {
    if (noise_variable_510 == 0.0) {
        let noise_metadata_schedule_246_e2382: f64 = (noise_variable_253 * noise_variable_8);
        let noise_metadata_schedule_246_e2384: f64 = (noise_metadata_schedule_246_e2382 - params.p147);
        let noise_metadata_schedule_246_e2385: f64 = (1.0 + noise_metadata_schedule_246_e2384);
        let noise_metadata_schedule_246_e2386: f64 = (noise_variable_295 * noise_metadata_schedule_246_e2385);
        (noise_metadata_schedule_246_e2386,)
    } else {
        (noise_variable_256,)
    }
};
            noise_variable_256 = noise_metadata_schedule_246_e2388;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let noise_metadata_schedule_247_e2391: f64 = (noise_variable_254 * noise_variable_8);
            let noise_metadata_schedule_247_e2393: f64 = if noise_metadata_schedule_247_e2391 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_511 = noise_metadata_schedule_247_e2393;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_248_e2400,) = {
    if (noise_variable_511 != 0.0) {
        let noise_metadata_schedule_248_e2397: f64 = (noise_variable_254 * noise_variable_8);
        let noise_metadata_schedule_248_e2398: f64 = (noise_metadata_schedule_248_e2397).exp();
        (noise_metadata_schedule_248_e2398,)
    } else {
        (noise_variable_257,)
    }
};
            noise_variable_257 = noise_metadata_schedule_248_e2400;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let (noise_metadata_schedule_249_e2406,) = {
    if (noise_variable_511 == 0.0) {
        let noise_metadata_schedule_249_e2404: f64 = (params.p147).exp();
        (noise_metadata_schedule_249_e2404,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_249_e2406;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_250_e2419,) = {
    if (noise_variable_511 == 0.0) {
        let noise_metadata_schedule_250_e2413: f64 = (noise_variable_254 * noise_variable_8);
        let noise_metadata_schedule_250_e2415: f64 = (noise_metadata_schedule_250_e2413 - params.p147);
        let noise_metadata_schedule_250_e2416: f64 = (1.0 + noise_metadata_schedule_250_e2415);
        let noise_metadata_schedule_250_e2417: f64 = (noise_variable_295 * noise_metadata_schedule_250_e2416);
        (noise_metadata_schedule_250_e2417,)
    } else {
        (noise_variable_257,)
    }
};
            noise_variable_257 = noise_metadata_schedule_250_e2419;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let noise_metadata_schedule_251_e2422: f64 = (noise_variable_255 * noise_variable_8);
            let noise_metadata_schedule_251_e2424: f64 = if noise_metadata_schedule_251_e2422 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_512 = noise_metadata_schedule_251_e2424;
        }
        if matches!(source_index, 18) {
            let (noise_metadata_schedule_252_e2431,) = {
    if (noise_variable_512 != 0.0) {
        let noise_metadata_schedule_252_e2428: f64 = (noise_variable_255 * noise_variable_8);
        let noise_metadata_schedule_252_e2429: f64 = (noise_metadata_schedule_252_e2428).exp();
        (noise_metadata_schedule_252_e2429,)
    } else {
        (noise_variable_258,)
    }
};
            noise_variable_258 = noise_metadata_schedule_252_e2431;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let (noise_metadata_schedule_253_e2437,) = {
    if (noise_variable_512 == 0.0) {
        let noise_metadata_schedule_253_e2435: f64 = (params.p147).exp();
        (noise_metadata_schedule_253_e2435,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_253_e2437;
        }
        if matches!(source_index, 18) {
            let (noise_metadata_schedule_254_e2450,) = {
    if (noise_variable_512 == 0.0) {
        let noise_metadata_schedule_254_e2444: f64 = (noise_variable_255 * noise_variable_8);
        let noise_metadata_schedule_254_e2446: f64 = (noise_metadata_schedule_254_e2444 - params.p147);
        let noise_metadata_schedule_254_e2447: f64 = (1.0 + noise_metadata_schedule_254_e2446);
        let noise_metadata_schedule_254_e2448: f64 = (noise_variable_295 * noise_metadata_schedule_254_e2447);
        (noise_metadata_schedule_254_e2448,)
    } else {
        (noise_variable_258,)
    }
};
            noise_variable_258 = noise_metadata_schedule_254_e2450;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_255_e2453: f64 = (noise_variable_261 - noise_variable_16);
            let noise_metadata_schedule_255_e2455: f64 = (noise_metadata_schedule_255_e2453 * noise_variable_8);
            let noise_metadata_schedule_255_e2457: f64 = if noise_metadata_schedule_255_e2455 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_513 = noise_metadata_schedule_255_e2457;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_257_e2472,) = {
    if (noise_variable_513 == 0.0) {
        let noise_metadata_schedule_257_e2470: f64 = (params.p147).exp();
        (noise_metadata_schedule_257_e2470,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_257_e2472;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_259_e2490: f64 = (noise_variable_249 - noise_variable_16);
            let noise_metadata_schedule_259_e2492: f64 = (noise_metadata_schedule_259_e2490 * noise_variable_8);
            let noise_metadata_schedule_259_e2494: f64 = if noise_metadata_schedule_259_e2492 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_514 = noise_metadata_schedule_259_e2494;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_261_e2509,) = {
    if (noise_variable_514 == 0.0) {
        let noise_metadata_schedule_261_e2507: f64 = (params.p147).exp();
        (noise_metadata_schedule_261_e2507,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_261_e2509;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_263_e2527: f64 = (noise_variable_245 - noise_variable_16);
            let noise_metadata_schedule_263_e2529: f64 = (noise_metadata_schedule_263_e2527 * noise_variable_8);
            let noise_metadata_schedule_263_e2531: f64 = if noise_metadata_schedule_263_e2529 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_515 = noise_metadata_schedule_263_e2531;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_264_e2540,) = {
    if (noise_variable_515 != 0.0) {
        let noise_metadata_schedule_264_e2535: f64 = (noise_variable_245 - noise_variable_16);
        let noise_metadata_schedule_264_e2537: f64 = (noise_metadata_schedule_264_e2535 * noise_variable_8);
        let noise_metadata_schedule_264_e2538: f64 = (noise_metadata_schedule_264_e2537).exp();
        (noise_metadata_schedule_264_e2538,)
    } else {
        (noise_variable_271,)
    }
};
            noise_variable_271 = noise_metadata_schedule_264_e2540;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_265_e2546,) = {
    if (noise_variable_515 == 0.0) {
        let noise_metadata_schedule_265_e2544: f64 = (params.p147).exp();
        (noise_metadata_schedule_265_e2544,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_265_e2546;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_266_e2561,) = {
    if (noise_variable_515 == 0.0) {
        let noise_metadata_schedule_266_e2553: f64 = (noise_variable_245 - noise_variable_16);
        let noise_metadata_schedule_266_e2555: f64 = (noise_metadata_schedule_266_e2553 * noise_variable_8);
        let noise_metadata_schedule_266_e2557: f64 = (noise_metadata_schedule_266_e2555 - params.p147);
        let noise_metadata_schedule_266_e2558: f64 = (1.0 + noise_metadata_schedule_266_e2557);
        let noise_metadata_schedule_266_e2559: f64 = (noise_variable_295 * noise_metadata_schedule_266_e2558);
        (noise_metadata_schedule_266_e2559,)
    } else {
        (noise_variable_271,)
    }
};
            noise_variable_271 = noise_metadata_schedule_266_e2561;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_267_e2564: f64 = (noise_variable_244 - noise_variable_16);
            let noise_metadata_schedule_267_e2566: f64 = (noise_metadata_schedule_267_e2564 * noise_variable_8);
            let noise_metadata_schedule_267_e2568: f64 = if noise_metadata_schedule_267_e2566 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_516 = noise_metadata_schedule_267_e2568;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_268_e2577,) = {
    if (noise_variable_516 != 0.0) {
        let noise_metadata_schedule_268_e2572: f64 = (noise_variable_244 - noise_variable_16);
        let noise_metadata_schedule_268_e2574: f64 = (noise_metadata_schedule_268_e2572 * noise_variable_8);
        let noise_metadata_schedule_268_e2575: f64 = (noise_metadata_schedule_268_e2574).exp();
        (noise_metadata_schedule_268_e2575,)
    } else {
        (noise_variable_273,)
    }
};
            noise_variable_273 = noise_metadata_schedule_268_e2577;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_269_e2583,) = {
    if (noise_variable_516 == 0.0) {
        let noise_metadata_schedule_269_e2581: f64 = (params.p147).exp();
        (noise_metadata_schedule_269_e2581,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_269_e2583;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_270_e2598,) = {
    if (noise_variable_516 == 0.0) {
        let noise_metadata_schedule_270_e2590: f64 = (noise_variable_244 - noise_variable_16);
        let noise_metadata_schedule_270_e2592: f64 = (noise_metadata_schedule_270_e2590 * noise_variable_8);
        let noise_metadata_schedule_270_e2594: f64 = (noise_metadata_schedule_270_e2592 - params.p147);
        let noise_metadata_schedule_270_e2595: f64 = (1.0 + noise_metadata_schedule_270_e2594);
        let noise_metadata_schedule_270_e2596: f64 = (noise_variable_295 * noise_metadata_schedule_270_e2595);
        (noise_metadata_schedule_270_e2596,)
    } else {
        (noise_variable_273,)
    }
};
            noise_variable_273 = noise_metadata_schedule_270_e2598;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_271_e2602: f64 = (4.0 * noise_variable_271);
            let noise_metadata_schedule_271_e2603: f64 = (1.0 + noise_metadata_schedule_271_e2602);
            let noise_metadata_schedule_271_e2604: f64 = (noise_metadata_schedule_271_e2603).sqrt();
            noise_variable_111 = noise_metadata_schedule_271_e2604;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_272_e2608: f64 = (4.0 * noise_variable_273);
            let noise_metadata_schedule_272_e2609: f64 = (1.0 + noise_metadata_schedule_272_e2608);
            let noise_metadata_schedule_272_e2610: f64 = (noise_metadata_schedule_272_e2609).sqrt();
            noise_variable_112 = noise_metadata_schedule_272_e2610;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_273_e2613: f64 = (2.0 * noise_variable_273);
            let noise_metadata_schedule_273_e2616: f64 = (1.0 + noise_variable_112);
            let noise_metadata_schedule_273_e2617: f64 = (noise_metadata_schedule_273_e2613 / noise_metadata_schedule_273_e2616);
            noise_variable_113 = noise_metadata_schedule_273_e2617;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_274_e2620: f64 = if noise_variable_113 < params.p149 { 1.0 } else { 0.0 };
            noise_variable_517 = noise_metadata_schedule_274_e2620;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_275_e2624,) = {
    if (noise_variable_517 != 0.0) {
        (params.p149,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_275_e2624;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_276_e2628: f64 = (noise_variable_111 - noise_variable_112);
            let noise_metadata_schedule_276_e2631: f64 = (noise_variable_111 + 1.0);
            let noise_metadata_schedule_276_e2634: f64 = (noise_variable_112 + 1.0);
            let noise_metadata_schedule_276_e2635: f64 = (noise_metadata_schedule_276_e2631 / noise_metadata_schedule_276_e2634);
            let noise_metadata_schedule_276_e2636: f64 = (noise_metadata_schedule_276_e2635).ln();
            let noise_metadata_schedule_276_e2637: f64 = (noise_metadata_schedule_276_e2628 - noise_metadata_schedule_276_e2636);
            let noise_metadata_schedule_276_e2638: f64 = (noise_variable_6 * noise_metadata_schedule_276_e2637);
            noise_variable_114 = noise_metadata_schedule_276_e2638;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_277_e2641: f64 = (noise_variable_114 + noise_variable_250);
            let noise_metadata_schedule_277_e2643: f64 = (noise_metadata_schedule_277_e2641 / noise_variable_31);
            noise_variable_115 = noise_metadata_schedule_277_e2643;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_278_e2646: f64 = if noise_variable_115 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_518 = noise_metadata_schedule_278_e2646;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_279_e2649: f64 = if noise_variable_244 < 100.0 { 1.0 } else { 0.0 };
            noise_variable_519 = noise_metadata_schedule_279_e2649;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_280_e2655,) = {
    if ((noise_variable_518 != 0.0) && (noise_variable_519 != 0.0)) {
        (noise_variable_244,)
    } else {
        (noise_variable_297,)
    }
};
            noise_variable_297 = noise_metadata_schedule_280_e2655;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_281_e2669,) = {
    if ((noise_variable_518 != 0.0) && (noise_variable_519 == 0.0)) {
        let noise_metadata_schedule_281_e2664: f64 = (noise_variable_244 - 100.0);
        let noise_metadata_schedule_281_e2665: f64 = (1.0 + noise_metadata_schedule_281_e2664);
        let noise_metadata_schedule_281_e2666: f64 = (noise_metadata_schedule_281_e2665).ln();
        let noise_metadata_schedule_281_e2667: f64 = (100.0 + noise_metadata_schedule_281_e2666);
        (noise_metadata_schedule_281_e2667,)
    } else {
        (noise_variable_297,)
    }
};
            noise_variable_297 = noise_metadata_schedule_281_e2669;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_282_e2690,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_282_e2674: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_282_e2677: f64 = (0.5 * noise_variable_115);
        let noise_metadata_schedule_282_e2679: f64 = (noise_metadata_schedule_282_e2677 * noise_variable_31);
        let noise_metadata_schedule_282_e2681: f64 = (noise_metadata_schedule_282_e2679 * noise_variable_8);
        let noise_metadata_schedule_282_e2683: f64 = (noise_metadata_schedule_282_e2681 + 1.0);
        let noise_metadata_schedule_282_e2684: f64 = (noise_metadata_schedule_282_e2683).ln();
        let noise_metadata_schedule_282_e2685: f64 = (noise_metadata_schedule_282_e2674 * noise_metadata_schedule_282_e2684);
        let noise_metadata_schedule_282_e2686: f64 = (noise_variable_16 + noise_metadata_schedule_282_e2685);
        let noise_metadata_schedule_282_e2688: f64 = (noise_metadata_schedule_282_e2686 - noise_variable_297);
        (noise_metadata_schedule_282_e2688,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_282_e2690;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_283_e2696,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_283_e2694: f64 = (0.2 * noise_variable_16);
        (noise_metadata_schedule_283_e2694,)
    } else {
        (noise_variable_292,)
    }
};
            noise_variable_292 = noise_metadata_schedule_283_e2696;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_284_e2702,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_284_e2700: f64 = (noise_variable_292 * noise_variable_292);
        (noise_metadata_schedule_284_e2700,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_284_e2702;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_285_e2708,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_285_e2706: f64 = (noise_variable_116 * noise_variable_116);
        (noise_metadata_schedule_285_e2706,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_285_e2708;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_286_e2711: f64 = if noise_variable_116 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_520 = noise_metadata_schedule_286_e2711;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_287_e2726,) = {
    if ((noise_variable_518 != 0.0) && (noise_variable_520 != 0.0)) {
        let noise_metadata_schedule_287_e2717: f64 = (0.5 * noise_variable_281);
        let noise_metadata_schedule_287_e2720: f64 = (noise_variable_282 + noise_variable_281);
        let noise_metadata_schedule_287_e2721: f64 = (noise_metadata_schedule_287_e2720).sqrt();
        let noise_metadata_schedule_287_e2723: f64 = (noise_metadata_schedule_287_e2721 - noise_variable_116);
        let noise_metadata_schedule_287_e2724: f64 = (noise_metadata_schedule_287_e2717 / noise_metadata_schedule_287_e2723);
        (noise_metadata_schedule_287_e2724,)
    } else {
        (noise_variable_117,)
    }
};
            noise_variable_117 = noise_metadata_schedule_287_e2726;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_288_e2740,) = {
    if ((noise_variable_518 != 0.0) && (noise_variable_520 == 0.0)) {
        let noise_metadata_schedule_288_e2734: f64 = (noise_variable_282 + noise_variable_281);
        let noise_metadata_schedule_288_e2735: f64 = (noise_metadata_schedule_288_e2734).sqrt();
        let noise_metadata_schedule_288_e2737: f64 = (noise_metadata_schedule_288_e2735 + noise_variable_116);
        let noise_metadata_schedule_288_e2738: f64 = (0.5 * noise_metadata_schedule_288_e2737);
        (noise_metadata_schedule_288_e2738,)
    } else {
        (noise_variable_117,)
    }
};
            noise_variable_117 = noise_metadata_schedule_288_e2740;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_289_e2758,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_289_e2746: f64 = (params.p62 * params.p61);
        let noise_metadata_schedule_289_e2747: f64 = (noise_variable_117 + noise_metadata_schedule_289_e2746);
        let noise_metadata_schedule_289_e2748: f64 = (noise_variable_117 * noise_metadata_schedule_289_e2747);
        let noise_metadata_schedule_289_e2753: f64 = (params.p62 * noise_variable_31);
        let noise_metadata_schedule_289_e2754: f64 = (noise_variable_117 + noise_metadata_schedule_289_e2753);
        let noise_metadata_schedule_289_e2755: f64 = (params.p61 * noise_metadata_schedule_289_e2754);
        let noise_metadata_schedule_289_e2756: f64 = (noise_metadata_schedule_289_e2748 / noise_metadata_schedule_289_e2755);
        (noise_metadata_schedule_289_e2756,)
    } else {
        (noise_variable_118,)
    }
};
            noise_variable_118 = noise_metadata_schedule_289_e2758;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_290_e2764,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_290_e2762: f64 = (noise_variable_115 / noise_variable_118);
        (noise_metadata_schedule_290_e2762,)
    } else {
        (noise_variable_285,)
    }
};
            noise_variable_285 = noise_metadata_schedule_290_e2764;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_291_e2772,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_291_e2768: f64 = (noise_variable_285 - 1.0);
        let noise_metadata_schedule_291_e2770: f64 = (noise_metadata_schedule_291_e2768 / params.p63);
        (noise_metadata_schedule_291_e2770,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_291_e2772;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_292_e2775: f64 = if noise_variable_285 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_521 = noise_metadata_schedule_292_e2775;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_293_e2789,) = {
    if ((noise_variable_518 != 0.0) && (noise_variable_521 != 0.0)) {
        let noise_metadata_schedule_293_e2783: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_293_e2784: f64 = (1.0 + noise_metadata_schedule_293_e2783);
        let noise_metadata_schedule_293_e2785: f64 = (noise_metadata_schedule_293_e2784).ln();
        let noise_metadata_schedule_293_e2786: f64 = (params.p63 * noise_metadata_schedule_293_e2785);
        let noise_metadata_schedule_293_e2787: f64 = (1.0 + noise_metadata_schedule_293_e2786);
        (noise_metadata_schedule_293_e2787,)
    } else {
        (noise_variable_283,)
    }
};
            noise_variable_283 = noise_metadata_schedule_293_e2789;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_294_e2805,) = {
    if ((noise_variable_518 != 0.0) && (noise_variable_521 == 0.0)) {
        let noise_metadata_schedule_294_e2798: f64 = (-noise_variable_279);
        let noise_metadata_schedule_294_e2799: f64 = (noise_metadata_schedule_294_e2798).exp();
        let noise_metadata_schedule_294_e2800: f64 = (1.0 + noise_metadata_schedule_294_e2799);
        let noise_metadata_schedule_294_e2801: f64 = (noise_metadata_schedule_294_e2800).ln();
        let noise_metadata_schedule_294_e2802: f64 = (params.p63 * noise_metadata_schedule_294_e2801);
        let noise_metadata_schedule_294_e2803: f64 = (noise_variable_285 + noise_metadata_schedule_294_e2802);
        (noise_metadata_schedule_294_e2803,)
    } else {
        (noise_variable_283,)
    }
};
            noise_variable_283 = noise_metadata_schedule_294_e2805;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_295_e2822,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_295_e2812: f64 = (-1.0);
        let noise_metadata_schedule_295_e2814: f64 = (noise_metadata_schedule_295_e2812 / params.p63);
        let noise_metadata_schedule_295_e2815: f64 = (noise_metadata_schedule_295_e2814).exp();
        let noise_metadata_schedule_295_e2816: f64 = (1.0 + noise_metadata_schedule_295_e2815);
        let noise_metadata_schedule_295_e2817: f64 = (noise_metadata_schedule_295_e2816).ln();
        let noise_metadata_schedule_295_e2818: f64 = (params.p63 * noise_metadata_schedule_295_e2817);
        let noise_metadata_schedule_295_e2819: f64 = (1.0 + noise_metadata_schedule_295_e2818);
        let noise_metadata_schedule_295_e2820: f64 = (noise_variable_283 / noise_metadata_schedule_295_e2819);
        (noise_metadata_schedule_295_e2820,)
    } else {
        (noise_variable_119,)
    }
};
            noise_variable_119 = noise_metadata_schedule_295_e2822;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_296_e2830,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_296_e2827: f64 = (params.p62 * params.p61);
        let noise_metadata_schedule_296_e2828: f64 = (noise_variable_117 / noise_metadata_schedule_296_e2827);
        (noise_metadata_schedule_296_e2828,)
    } else {
        (noise_variable_120,)
    }
};
            noise_variable_120 = noise_metadata_schedule_296_e2830;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_297_e2855,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_297_e2836: f64 = (4.0 * noise_variable_119);
        let noise_metadata_schedule_297_e2838: f64 = (noise_metadata_schedule_297_e2836 * noise_variable_120);
        let noise_metadata_schedule_297_e2841: f64 = (1.0 + noise_variable_120);
        let noise_metadata_schedule_297_e2842: f64 = (noise_metadata_schedule_297_e2838 * noise_metadata_schedule_297_e2841);
        let noise_metadata_schedule_297_e2843: f64 = (1.0 + noise_metadata_schedule_297_e2842);
        let noise_metadata_schedule_297_e2844: f64 = (noise_metadata_schedule_297_e2843).sqrt();
        let noise_metadata_schedule_297_e2845: f64 = (1.0 + noise_metadata_schedule_297_e2844);
        let noise_metadata_schedule_297_e2848: f64 = (2.0 * noise_variable_119);
        let noise_metadata_schedule_297_e2851: f64 = (1.0 + noise_variable_120);
        let noise_metadata_schedule_297_e2852: f64 = (noise_metadata_schedule_297_e2848 * noise_metadata_schedule_297_e2851);
        let noise_metadata_schedule_297_e2853: f64 = (noise_metadata_schedule_297_e2845 / noise_metadata_schedule_297_e2852);
        (noise_metadata_schedule_297_e2853,)
    } else {
        (noise_variable_121,)
    }
};
            noise_variable_121 = noise_metadata_schedule_297_e2855;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_298_e2871,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_298_e2859: f64 = (1.0 - noise_variable_121);
        let noise_metadata_schedule_298_e2862: f64 = (noise_variable_113 * noise_variable_121);
        let noise_metadata_schedule_298_e2863: f64 = (noise_metadata_schedule_298_e2859 + noise_metadata_schedule_298_e2862);
        let noise_metadata_schedule_298_e2867: f64 = (noise_variable_113 * noise_variable_121);
        let noise_metadata_schedule_298_e2868: f64 = (1.0 + noise_metadata_schedule_298_e2867);
        let noise_metadata_schedule_298_e2869: f64 = (noise_metadata_schedule_298_e2863 / noise_metadata_schedule_298_e2868);
        (noise_metadata_schedule_298_e2869,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_298_e2871;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_299_e2883,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_299_e2875: f64 = (0.5 * noise_variable_115);
        let noise_metadata_schedule_299_e2877: f64 = (noise_metadata_schedule_299_e2875 * noise_variable_31);
        let noise_metadata_schedule_299_e2879: f64 = (noise_metadata_schedule_299_e2877 * noise_variable_122);
        let noise_metadata_schedule_299_e2881: f64 = (noise_metadata_schedule_299_e2879 * noise_variable_8);
        (noise_metadata_schedule_299_e2881,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_299_e2883;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_300_e2897,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_300_e2887: f64 = (2.0 * noise_variable_124);
        let noise_metadata_schedule_300_e2891: f64 = (noise_variable_113 + noise_variable_124);
        let noise_metadata_schedule_300_e2893: f64 = (noise_metadata_schedule_300_e2891 + 1.0);
        let noise_metadata_schedule_300_e2894: f64 = (noise_variable_113 * noise_metadata_schedule_300_e2893);
        let noise_metadata_schedule_300_e2895: f64 = (noise_metadata_schedule_300_e2887 + noise_metadata_schedule_300_e2894);
        (noise_metadata_schedule_300_e2895,)
    } else {
        (noise_variable_286,)
    }
};
            noise_variable_286 = noise_metadata_schedule_300_e2897;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_301_e2905,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_301_e2902: f64 = (noise_variable_124 - 1.0);
        let noise_metadata_schedule_301_e2903: f64 = (0.5 * noise_metadata_schedule_301_e2902);
        (noise_metadata_schedule_301_e2903,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_301_e2905;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_302_e2913,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_302_e2909: f64 = (noise_variable_125 * noise_variable_125);
        let noise_metadata_schedule_302_e2911: f64 = (noise_metadata_schedule_302_e2909 + noise_variable_286);
        (noise_metadata_schedule_302_e2911,)
    } else {
        (noise_variable_280,)
    }
};
            noise_variable_280 = noise_metadata_schedule_302_e2913;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_303_e2916: f64 = if noise_variable_124 >= 1.0 { 1.0 } else { 0.0 };
            noise_variable_522 = noise_metadata_schedule_303_e2916;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_304_e2925,) = {
    if ((noise_variable_518 != 0.0) && (noise_variable_522 != 0.0)) {
        let noise_metadata_schedule_304_e2922: f64 = (noise_variable_280).sqrt();
        let noise_metadata_schedule_304_e2923: f64 = (noise_variable_125 + noise_metadata_schedule_304_e2922);
        (noise_metadata_schedule_304_e2923,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_304_e2925;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_305_e2937,) = {
    if ((noise_variable_518 != 0.0) && (noise_variable_522 == 0.0)) {
        let noise_metadata_schedule_305_e2932: f64 = (noise_variable_280).sqrt();
        let noise_metadata_schedule_305_e2934: f64 = (noise_metadata_schedule_305_e2932 - noise_variable_125);
        let noise_metadata_schedule_305_e2935: f64 = (noise_variable_286 / noise_metadata_schedule_305_e2934);
        (noise_metadata_schedule_305_e2935,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_305_e2937;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_306_e2940: f64 = if noise_variable_126 < params.p148 { 1.0 } else { 0.0 };
            noise_variable_523 = noise_metadata_schedule_306_e2940;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_307_e2946,) = {
    if ((noise_variable_518 != 0.0) && (noise_variable_523 != 0.0)) {
        (params.p148,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_307_e2946;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_308_e2959,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_308_e2951: f64 = (noise_variable_126 + 1.0);
        let noise_metadata_schedule_308_e2952: f64 = (noise_variable_126 * noise_metadata_schedule_308_e2951);
        let noise_metadata_schedule_308_e2955: f64 = (noise_variable_16 * noise_variable_8);
        let noise_metadata_schedule_308_e2956: f64 = (noise_metadata_schedule_308_e2955).exp();
        let noise_metadata_schedule_308_e2957: f64 = (noise_metadata_schedule_308_e2952 * noise_metadata_schedule_308_e2956);
        (noise_metadata_schedule_308_e2957,)
    } else {
        (noise_variable_128,)
    }
};
            noise_variable_128 = noise_metadata_schedule_308_e2959;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_309_e2969,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_309_e2963: f64 = (0.5 * params.p61);
        let noise_metadata_schedule_309_e2966: f64 = (noise_variable_115 - params.p62);
        let noise_metadata_schedule_309_e2967: f64 = (noise_metadata_schedule_309_e2963 * noise_metadata_schedule_309_e2966);
        (noise_metadata_schedule_309_e2967,)
    } else {
        (noise_variable_130,)
    }
};
            noise_variable_130 = noise_metadata_schedule_309_e2969;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_310_e2979,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_310_e2973: f64 = (params.p61 * noise_variable_31);
        let noise_metadata_schedule_310_e2975: f64 = (noise_metadata_schedule_310_e2973 * params.p62);
        let noise_metadata_schedule_310_e2977: f64 = (noise_metadata_schedule_310_e2975 * noise_variable_115);
        (noise_metadata_schedule_310_e2977,)
    } else {
        (noise_variable_131,)
    }
};
            noise_variable_131 = noise_metadata_schedule_310_e2979;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_311_e2990,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_311_e2984: f64 = (noise_variable_130 * noise_variable_130);
        let noise_metadata_schedule_311_e2986: f64 = (noise_metadata_schedule_311_e2984 + noise_variable_131);
        let noise_metadata_schedule_311_e2987: f64 = (noise_metadata_schedule_311_e2986).sqrt();
        let noise_metadata_schedule_311_e2988: f64 = (noise_variable_130 + noise_metadata_schedule_311_e2987);
        (noise_metadata_schedule_311_e2988,)
    } else {
        (noise_variable_132,)
    }
};
            noise_variable_132 = noise_metadata_schedule_311_e2990;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_312_e2993: f64 = if params.p73 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_524 = noise_metadata_schedule_312_e2993;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_313_e3001,) = {
    if ((noise_variable_518 != 0.0) && (noise_variable_524 != 0.0)) {
        let noise_metadata_schedule_313_e2999: f64 = (noise_variable_17 * 0.1);
        (noise_metadata_schedule_313_e2999,)
    } else {
        (noise_variable_133,)
    }
};
            noise_variable_133 = noise_metadata_schedule_313_e3001;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_314_e3018,) = {
    if ((noise_variable_518 != 0.0) && (noise_variable_524 == 0.0)) {
        let noise_metadata_schedule_314_e3010: f64 = (2.0 * noise_variable_115);
        let noise_metadata_schedule_314_e3013: f64 = (noise_variable_115 + noise_variable_118);
        let noise_metadata_schedule_314_e3014: f64 = (noise_metadata_schedule_314_e3010 / noise_metadata_schedule_314_e3013);
        let noise_metadata_schedule_314_e3015: f64 = (0.1 + noise_metadata_schedule_314_e3014);
        let noise_metadata_schedule_314_e3016: f64 = (noise_variable_17 * noise_metadata_schedule_314_e3015);
        (noise_metadata_schedule_314_e3016,)
    } else {
        (noise_variable_133,)
    }
};
            noise_variable_133 = noise_metadata_schedule_314_e3018;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_315_e3028,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_315_e3022: f64 = (params.p62 * noise_variable_115);
        let noise_metadata_schedule_315_e3025: f64 = (params.p62 + noise_variable_115);
        let noise_metadata_schedule_315_e3026: f64 = (noise_metadata_schedule_315_e3022 / noise_metadata_schedule_315_e3025);
        (noise_metadata_schedule_315_e3026,)
    } else {
        (noise_variable_134,)
    }
};
            noise_variable_134 = noise_metadata_schedule_315_e3028;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_316_e3036,) = {
    if (noise_variable_518 != 0.0) {
        let noise_metadata_schedule_316_e3033: f64 = (params.p62 + noise_variable_115);
        let noise_metadata_schedule_316_e3034: f64 = (params.p62 / noise_metadata_schedule_316_e3033);
        (noise_metadata_schedule_316_e3034,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_316_e3036;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_318_e3052,) = {
    if (noise_variable_518 == 0.0) {
        let noise_metadata_schedule_318_e3046: f64 = (2.0 * noise_variable_271);
        let noise_metadata_schedule_318_e3049: f64 = (1.0 + noise_variable_111);
        let noise_metadata_schedule_318_e3050: f64 = (noise_metadata_schedule_318_e3046 / noise_metadata_schedule_318_e3049);
        (noise_metadata_schedule_318_e3050,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_318_e3052;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_319_e3057,) = {
    if (noise_variable_518 == 0.0) {
        (noise_variable_265,)
    } else {
        (noise_variable_128,)
    }
};
            noise_variable_128 = noise_metadata_schedule_319_e3057;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_320_e3059: f64 = (noise_variable_250).abs();
            let noise_metadata_schedule_320_e3062: f64 = (1e-5 * noise_variable_6);
            let noise_metadata_schedule_320_e3065: f64 = (noise_variable_114).abs();
            let noise_metadata_schedule_320_e3068: f64 = (1e-40 * noise_variable_6);
            let noise_metadata_schedule_320_e3071: f64 = (noise_variable_111 + noise_variable_112);
            let noise_metadata_schedule_320_e3072: f64 = (noise_metadata_schedule_320_e3068 * noise_metadata_schedule_320_e3071);
            let noise_metadata_schedule_320_e3074: f64 = if ((noise_metadata_schedule_320_e3059 < noise_metadata_schedule_320_e3062) || (noise_metadata_schedule_320_e3065 < noise_metadata_schedule_320_e3072)) { 1.0 } else { 0.0 };
            noise_variable_525 = noise_metadata_schedule_320_e3074;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_321_e3085,) = {
    if ((noise_variable_518 == 0.0) && (noise_variable_525 != 0.0)) {
        let noise_metadata_schedule_321_e3082: f64 = (noise_variable_126 + noise_variable_113);
        let noise_metadata_schedule_321_e3083: f64 = (0.5 * noise_metadata_schedule_321_e3082);
        (noise_metadata_schedule_321_e3083,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_321_e3085;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_322_e3096,) = {
    if ((noise_variable_518 == 0.0) && (noise_variable_525 != 0.0)) {
        let noise_metadata_schedule_322_e3093: f64 = (noise_variable_135 + 1.0);
        let noise_metadata_schedule_322_e3094: f64 = (noise_variable_135 / noise_metadata_schedule_322_e3093);
        (noise_metadata_schedule_322_e3094,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_322_e3096;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_323_e3110,) = {
    if ((noise_variable_518 == 0.0) && (noise_variable_525 == 0.0)) {
        let noise_metadata_schedule_323_e3105: f64 = (noise_variable_114 + noise_variable_245);
        let noise_metadata_schedule_323_e3107: f64 = (noise_metadata_schedule_323_e3105 - noise_variable_244);
        let noise_metadata_schedule_323_e3108: f64 = (noise_variable_114 / noise_metadata_schedule_323_e3107);
        (noise_metadata_schedule_323_e3108,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_323_e3110;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_324_e3115,) = {
    if (noise_variable_518 == 0.0) {
        (noise_variable_250,)
    } else {
        (noise_variable_132,)
    }
};
            noise_variable_132 = noise_metadata_schedule_324_e3115;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_325_e3122,) = {
    if (noise_variable_518 == 0.0) {
        let noise_metadata_schedule_325_e3120: f64 = (0.1 * noise_variable_17);
        (noise_metadata_schedule_325_e3120,)
    } else {
        (noise_variable_133,)
    }
};
            noise_variable_133 = noise_metadata_schedule_325_e3122;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_326_e3127,) = {
    if (noise_variable_518 == 0.0) {
        (noise_variable_115,)
    } else {
        (noise_variable_134,)
    }
};
            noise_variable_134 = noise_metadata_schedule_326_e3127;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_327_e3136,) = {
    if (noise_variable_518 == 0.0) {
        let noise_metadata_schedule_327_e3133: f64 = (noise_variable_134 / params.p62);
        let noise_metadata_schedule_327_e3134: f64 = (1.0 - noise_metadata_schedule_327_e3133);
        (noise_metadata_schedule_327_e3134,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_327_e3136;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_328_e3141: f64 = (-1.0);
            let noise_metadata_schedule_328_e3143: f64 = (noise_metadata_schedule_328_e3141 / params.p67);
            let noise_metadata_schedule_328_e3144: f64 = (3.0_f64).powf(noise_metadata_schedule_328_e3143);
            let noise_metadata_schedule_328_e3145: f64 = (1.0 - noise_metadata_schedule_328_e3144);
            let noise_metadata_schedule_328_e3146: f64 = (noise_variable_14 * noise_metadata_schedule_328_e3145);
            noise_variable_136 = noise_metadata_schedule_328_e3146;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_329_e3149: f64 = (0.1 * noise_variable_14);
            noise_variable_293 = noise_metadata_schedule_329_e3149;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_330_e3152: f64 = (noise_variable_246 - noise_variable_136);
            let noise_metadata_schedule_330_e3154: f64 = (noise_metadata_schedule_330_e3152 / noise_variable_293);
            noise_variable_279 = noise_metadata_schedule_330_e3154;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_331_e3157: f64 = if noise_variable_246 < noise_variable_136 { 1.0 } else { 0.0 };
            noise_variable_526 = noise_metadata_schedule_331_e3157;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_332_e3169,) = {
    if (noise_variable_526 != 0.0) {
        let noise_metadata_schedule_332_e3163: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_332_e3164: f64 = (1.0 + noise_metadata_schedule_332_e3163);
        let noise_metadata_schedule_332_e3165: f64 = (noise_metadata_schedule_332_e3164).ln();
        let noise_metadata_schedule_332_e3166: f64 = (noise_variable_293 * noise_metadata_schedule_332_e3165);
        let noise_metadata_schedule_332_e3167: f64 = (noise_variable_246 - noise_metadata_schedule_332_e3166);
        (noise_metadata_schedule_332_e3167,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_332_e3169;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_333_e3183,) = {
    if (noise_variable_526 == 0.0) {
        let noise_metadata_schedule_333_e3176: f64 = (-noise_variable_279);
        let noise_metadata_schedule_333_e3177: f64 = (noise_metadata_schedule_333_e3176).exp();
        let noise_metadata_schedule_333_e3178: f64 = (1.0 + noise_metadata_schedule_333_e3177);
        let noise_metadata_schedule_333_e3179: f64 = (noise_metadata_schedule_333_e3178).ln();
        let noise_metadata_schedule_333_e3180: f64 = (noise_variable_293 * noise_metadata_schedule_333_e3179);
        let noise_metadata_schedule_333_e3181: f64 = (noise_variable_136 - noise_metadata_schedule_333_e3180);
        (noise_metadata_schedule_333_e3181,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_333_e3183;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_334_e3187: f64 = (noise_variable_137 * noise_variable_65);
            let noise_metadata_schedule_334_e3188: f64 = (1.0 - noise_metadata_schedule_334_e3187);
            let noise_metadata_schedule_334_e3191: f64 = (1.0 - params.p67);
            let noise_metadata_schedule_334_e3192: f64 = (noise_metadata_schedule_334_e3188).powf(noise_metadata_schedule_334_e3191);
            noise_variable_59 = noise_metadata_schedule_334_e3192;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_335_e3196: f64 = (1.0 - params.p67);
            let noise_metadata_schedule_335_e3197: f64 = (noise_variable_14 / noise_metadata_schedule_335_e3196);
            let noise_metadata_schedule_335_e3200: f64 = (1.0 - noise_variable_59);
            let noise_metadata_schedule_335_e3201: f64 = (noise_metadata_schedule_335_e3197 * noise_metadata_schedule_335_e3200);
            let noise_metadata_schedule_335_e3205: f64 = (noise_variable_246 - noise_variable_137);
            let noise_metadata_schedule_335_e3206: f64 = (3.0 * noise_metadata_schedule_335_e3205);
            let noise_metadata_schedule_335_e3207: f64 = (noise_metadata_schedule_335_e3201 + noise_metadata_schedule_335_e3206);
            noise_variable_138 = noise_metadata_schedule_335_e3207;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_336_e3210: f64 = if params.p74 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_527 = noise_metadata_schedule_336_e3210;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_337_e3214,) = {
    if (noise_variable_527 != 0.0) {
        (noise_variable_244,)
    } else {
        (noise_variable_139,)
    }
};
            noise_variable_139 = noise_metadata_schedule_337_e3214;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_338_e3217: f64 = if params.p74 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_528 = noise_metadata_schedule_338_e3217;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_339_e3226,) = {
    if ((noise_variable_527 == 0.0) && (noise_variable_528 != 0.0)) {
        let noise_metadata_schedule_339_e3224: f64 = (noise_variable_244 + noise_variable_132);
        (noise_metadata_schedule_339_e3224,)
    } else {
        (noise_variable_139,)
    }
};
            noise_variable_139 = noise_metadata_schedule_339_e3226;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_340_e3234,) = {
    if ((noise_variable_527 == 0.0) && (noise_variable_528 == 0.0)) {
        (noise_variable_245,)
    } else {
        (noise_variable_139,)
    }
};
            noise_variable_139 = noise_metadata_schedule_340_e3234;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_341_e3237: f64 = (2.0 - noise_variable_25);
            let noise_metadata_schedule_341_e3240: f64 = (1.0 - noise_variable_25);
            let noise_metadata_schedule_341_e3241: f64 = (noise_metadata_schedule_341_e3237 / noise_metadata_schedule_341_e3240);
            noise_variable_140 = noise_metadata_schedule_341_e3241;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_342_e3246: f64 = (-1.0);
            let noise_metadata_schedule_342_e3248: f64 = (noise_metadata_schedule_342_e3246 / params.p72);
            let noise_metadata_schedule_342_e3249: f64 = (noise_variable_140).powf(noise_metadata_schedule_342_e3248);
            let noise_metadata_schedule_342_e3250: f64 = (1.0 - noise_metadata_schedule_342_e3249);
            let noise_metadata_schedule_342_e3251: f64 = (noise_variable_17 * noise_metadata_schedule_342_e3250);
            noise_variable_141 = noise_metadata_schedule_342_e3251;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_343_e3254: f64 = (noise_variable_139 - noise_variable_141);
            let noise_metadata_schedule_343_e3256: f64 = (noise_metadata_schedule_343_e3254 / noise_variable_133);
            noise_variable_279 = noise_metadata_schedule_343_e3256;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_344_e3259: f64 = if noise_variable_139 < noise_variable_141 { 1.0 } else { 0.0 };
            noise_variable_529 = noise_metadata_schedule_344_e3259;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_345_e3271,) = {
    if (noise_variable_529 != 0.0) {
        let noise_metadata_schedule_345_e3265: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_345_e3266: f64 = (1.0 + noise_metadata_schedule_345_e3265);
        let noise_metadata_schedule_345_e3267: f64 = (noise_metadata_schedule_345_e3266).ln();
        let noise_metadata_schedule_345_e3268: f64 = (noise_variable_133 * noise_metadata_schedule_345_e3267);
        let noise_metadata_schedule_345_e3269: f64 = (noise_variable_139 - noise_metadata_schedule_345_e3268);
        (noise_metadata_schedule_345_e3269,)
    } else {
        (noise_variable_142,)
    }
};
            noise_variable_142 = noise_metadata_schedule_345_e3271;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_346_e3285,) = {
    if (noise_variable_529 == 0.0) {
        let noise_metadata_schedule_346_e3278: f64 = (-noise_variable_279);
        let noise_metadata_schedule_346_e3279: f64 = (noise_metadata_schedule_346_e3278).exp();
        let noise_metadata_schedule_346_e3280: f64 = (1.0 + noise_metadata_schedule_346_e3279);
        let noise_metadata_schedule_346_e3281: f64 = (noise_metadata_schedule_346_e3280).ln();
        let noise_metadata_schedule_346_e3282: f64 = (noise_variable_133 * noise_metadata_schedule_346_e3281);
        let noise_metadata_schedule_346_e3283: f64 = (noise_variable_141 - noise_metadata_schedule_346_e3282);
        (noise_metadata_schedule_346_e3283,)
    } else {
        (noise_variable_142,)
    }
};
            noise_variable_142 = noise_metadata_schedule_346_e3285;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_347_e3288: f64 = (noise_variable_210).powf(params.p76);
            noise_variable_143 = noise_metadata_schedule_347_e3288;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_348_e3292: f64 = (1.0 - params.p72);
            let noise_metadata_schedule_348_e3293: f64 = (noise_variable_17 / noise_metadata_schedule_348_e3292);
            let noise_metadata_schedule_348_e3299: f64 = (noise_variable_142 / noise_variable_17);
            let noise_metadata_schedule_348_e3300: f64 = (1.0 - noise_metadata_schedule_348_e3299);
            let noise_metadata_schedule_348_e3303: f64 = (1.0 - params.p72);
            let noise_metadata_schedule_348_e3304: f64 = (noise_metadata_schedule_348_e3300).powf(noise_metadata_schedule_348_e3303);
            let noise_metadata_schedule_348_e3305: f64 = (noise_variable_143 * noise_metadata_schedule_348_e3304);
            let noise_metadata_schedule_348_e3306: f64 = (1.0 - noise_metadata_schedule_348_e3305);
            let noise_metadata_schedule_348_e3307: f64 = (noise_metadata_schedule_348_e3293 * noise_metadata_schedule_348_e3306);
            let noise_metadata_schedule_348_e3310: f64 = (noise_variable_143 * noise_variable_140);
            let noise_metadata_schedule_348_e3313: f64 = (noise_variable_139 - noise_variable_142);
            let noise_metadata_schedule_348_e3314: f64 = (noise_metadata_schedule_348_e3310 * noise_metadata_schedule_348_e3313);
            let noise_metadata_schedule_348_e3315: f64 = (noise_metadata_schedule_348_e3307 + noise_metadata_schedule_348_e3314);
            noise_variable_144 = noise_metadata_schedule_348_e3315;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_349_e3318: f64 = (1.0 - noise_variable_25);
            let noise_metadata_schedule_349_e3320: f64 = (noise_metadata_schedule_349_e3318 * noise_variable_144);
            let noise_metadata_schedule_349_e3323: f64 = (noise_variable_25 * noise_variable_244);
            let noise_metadata_schedule_349_e3324: f64 = (noise_metadata_schedule_349_e3320 + noise_metadata_schedule_349_e3323);
            noise_variable_145 = noise_metadata_schedule_349_e3324;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_350_e3327: f64 = (4.0 * noise_variable_35);
            let noise_metadata_schedule_350_e3329: f64 = (noise_metadata_schedule_350_e3327 / noise_variable_36);
            noise_variable_146 = noise_metadata_schedule_350_e3329;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_351_e3332: f64 = (noise_variable_146 * noise_variable_266);
            noise_variable_147 = noise_metadata_schedule_351_e3332;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_352_e3337: f64 = (1.0 + noise_variable_147);
            let noise_metadata_schedule_352_e3338: f64 = (noise_metadata_schedule_352_e3337).sqrt();
            let noise_metadata_schedule_352_e3339: f64 = (1.0 + noise_metadata_schedule_352_e3338);
            let noise_metadata_schedule_352_e3340: f64 = (noise_variable_147 / noise_metadata_schedule_352_e3339);
            noise_variable_149 = noise_metadata_schedule_352_e3340;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_353_e3344: f64 = (1.0 / noise_variable_49);
            let noise_metadata_schedule_353_e3345: f64 = (noise_variable_128).powf(noise_metadata_schedule_353_e3344);
            noise_variable_129 = noise_metadata_schedule_353_e3345;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_354_e3348: f64 = (noise_variable_146 * noise_variable_129);
            noise_variable_148 = noise_metadata_schedule_354_e3348;
        }
        if matches!(source_index, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_355_e3353: f64 = (1.0 + noise_variable_148);
            let noise_metadata_schedule_355_e3354: f64 = (noise_metadata_schedule_355_e3353).sqrt();
            let noise_metadata_schedule_355_e3355: f64 = (1.0 + noise_metadata_schedule_355_e3354);
            let noise_metadata_schedule_355_e3356: f64 = (noise_variable_148 / noise_metadata_schedule_355_e3355);
            noise_variable_150 = noise_metadata_schedule_355_e3356;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_356_e3359: f64 = if params.p92 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_530 = noise_metadata_schedule_356_e3359;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_357_e3371,) = {
    if (noise_variable_530 != 0.0) {
        let noise_metadata_schedule_357_e3364: f64 = (noise_variable_138 / noise_variable_41);
        let noise_metadata_schedule_357_e3365: f64 = (1.0 + noise_metadata_schedule_357_e3364);
        let noise_metadata_schedule_357_e3368: f64 = (noise_variable_145 / noise_variable_40);
        let noise_metadata_schedule_357_e3369: f64 = (noise_metadata_schedule_357_e3365 + noise_metadata_schedule_357_e3368);
        (noise_metadata_schedule_357_e3369,)
    } else {
        (noise_variable_151,)
    }
};
            noise_variable_151 = noise_metadata_schedule_357_e3371;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_358_e3384,) = {
    if (noise_variable_530 == 0.0) {
        let noise_metadata_schedule_358_e3376: f64 = (noise_variable_138 / noise_variable_41);
        let noise_metadata_schedule_358_e3378: f64 = (noise_metadata_schedule_358_e3376 + 1.0);
        let noise_metadata_schedule_358_e3380: f64 = (noise_metadata_schedule_358_e3378 * noise_variable_99);
        let noise_metadata_schedule_358_e3382: f64 = (noise_metadata_schedule_358_e3380 * noise_variable_8);
        (noise_metadata_schedule_358_e3382,)
    } else {
        (noise_variable_289,)
    }
};
            noise_variable_289 = noise_metadata_schedule_358_e3384;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_359_e3396,) = {
    if (noise_variable_530 == 0.0) {
        let noise_metadata_schedule_359_e3388: f64 = (-noise_variable_145);
        let noise_metadata_schedule_359_e3390: f64 = (noise_metadata_schedule_359_e3388 / noise_variable_40);
        let noise_metadata_schedule_359_e3392: f64 = (noise_metadata_schedule_359_e3390 * noise_variable_99);
        let noise_metadata_schedule_359_e3394: f64 = (noise_metadata_schedule_359_e3392 * noise_variable_8);
        (noise_metadata_schedule_359_e3394,)
    } else {
        (noise_variable_290,)
    }
};
            noise_variable_290 = noise_metadata_schedule_359_e3396;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_360_e3412,) = {
    if (noise_variable_530 == 0.0) {
        let noise_metadata_schedule_360_e3400: f64 = (noise_variable_289).exp();
        let noise_metadata_schedule_360_e3402: f64 = (noise_variable_290).exp();
        let noise_metadata_schedule_360_e3403: f64 = (noise_metadata_schedule_360_e3400 - noise_metadata_schedule_360_e3402);
        let noise_metadata_schedule_360_e3406: f64 = (noise_variable_99 * noise_variable_8);
        let noise_metadata_schedule_360_e3407: f64 = (noise_metadata_schedule_360_e3406).exp();
        let noise_metadata_schedule_360_e3409: f64 = (noise_metadata_schedule_360_e3407 - 1.0);
        let noise_metadata_schedule_360_e3410: f64 = (noise_metadata_schedule_360_e3403 / noise_metadata_schedule_360_e3409);
        (noise_metadata_schedule_360_e3410,)
    } else {
        (noise_variable_151,)
    }
};
            noise_variable_151 = noise_metadata_schedule_360_e3412;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_361_e3415: f64 = (0.1 * 0.1);
            noise_variable_281 = noise_metadata_schedule_361_e3415;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_362_e3418: f64 = (noise_variable_151 * noise_variable_151);
            noise_variable_282 = noise_metadata_schedule_362_e3418;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_363_e3421: f64 = if noise_variable_151 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_531 = noise_metadata_schedule_363_e3421;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_364_e3434,) = {
    if (noise_variable_531 != 0.0) {
        let noise_metadata_schedule_364_e3425: f64 = (0.5 * noise_variable_281);
        let noise_metadata_schedule_364_e3428: f64 = (noise_variable_282 + noise_variable_281);
        let noise_metadata_schedule_364_e3429: f64 = (noise_metadata_schedule_364_e3428).sqrt();
        let noise_metadata_schedule_364_e3431: f64 = (noise_metadata_schedule_364_e3429 - noise_variable_151);
        let noise_metadata_schedule_364_e3432: f64 = (noise_metadata_schedule_364_e3425 / noise_metadata_schedule_364_e3431);
        (noise_metadata_schedule_364_e3432,)
    } else {
        (noise_variable_152,)
    }
};
            noise_variable_152 = noise_metadata_schedule_364_e3434;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_365_e3446,) = {
    if (noise_variable_531 == 0.0) {
        let noise_metadata_schedule_365_e3440: f64 = (noise_variable_282 + noise_variable_281);
        let noise_metadata_schedule_365_e3441: f64 = (noise_metadata_schedule_365_e3440).sqrt();
        let noise_metadata_schedule_365_e3443: f64 = (noise_metadata_schedule_365_e3441 + noise_variable_151);
        let noise_metadata_schedule_365_e3444: f64 = (0.5 * noise_metadata_schedule_365_e3443);
        (noise_metadata_schedule_365_e3444,)
    } else {
        (noise_variable_152,)
    }
};
            noise_variable_152 = noise_metadata_schedule_365_e3446;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_366_e3452: f64 = (noise_variable_149 + noise_variable_150);
            let noise_metadata_schedule_366_e3453: f64 = (0.5 * noise_metadata_schedule_366_e3452);
            let noise_metadata_schedule_366_e3454: f64 = (1.0 + noise_metadata_schedule_366_e3453);
            let noise_metadata_schedule_366_e3455: f64 = (noise_variable_152 * noise_metadata_schedule_366_e3454);
            noise_variable_153 = noise_metadata_schedule_366_e3455;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_367_e3458: f64 = (params.p15 * noise_variable_35);
            let noise_metadata_schedule_367_e3460: f64 = (noise_metadata_schedule_367_e3458 * noise_variable_129);
            noise_variable_154 = noise_metadata_schedule_367_e3460;
        }
        if matches!(source_index, 0 | 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_368_e3463: f64 = (noise_variable_35 * noise_variable_266);
            noise_variable_155 = noise_metadata_schedule_368_e3463;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_369_e3466: f64 = (noise_variable_155 - noise_variable_154);
            let noise_metadata_schedule_369_e3468: f64 = (noise_metadata_schedule_369_e3466 / noise_variable_153);
            noise_variable_156 = noise_metadata_schedule_369_e3468;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_370_e3471: f64 = noise_variable_246;
            let noise_metadata_schedule_370_e3473: f64 = (noise_metadata_schedule_370_e3471 / 0.0001);
            noise_variable_279 = noise_metadata_schedule_370_e3473;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_371_e3476: f64 = if noise_variable_246 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_532 = noise_metadata_schedule_371_e3476;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_372_e3488,) = {
    if (noise_variable_532 != 0.0) {
        let noise_metadata_schedule_372_e3482: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_372_e3483: f64 = (1.0 + noise_metadata_schedule_372_e3482);
        let noise_metadata_schedule_372_e3484: f64 = (noise_metadata_schedule_372_e3483).ln();
        let noise_metadata_schedule_372_e3485: f64 = (0.0001 * noise_metadata_schedule_372_e3484);
        let noise_metadata_schedule_372_e3486: f64 = noise_metadata_schedule_372_e3485;
        (noise_metadata_schedule_372_e3486,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_372_e3488;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_373_e3502,) = {
    if (noise_variable_532 == 0.0) {
        let noise_metadata_schedule_373_e3495: f64 = (-noise_variable_279);
        let noise_metadata_schedule_373_e3496: f64 = (noise_metadata_schedule_373_e3495).exp();
        let noise_metadata_schedule_373_e3497: f64 = (1.0 + noise_metadata_schedule_373_e3496);
        let noise_metadata_schedule_373_e3498: f64 = (noise_metadata_schedule_373_e3497).ln();
        let noise_metadata_schedule_373_e3499: f64 = (0.0001 * noise_metadata_schedule_373_e3498);
        let noise_metadata_schedule_373_e3500: f64 = (noise_variable_246 + noise_metadata_schedule_373_e3499);
        (noise_metadata_schedule_373_e3500,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_373_e3502;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_374_e3505: f64 = (noise_variable_296 / params.p152);
            noise_variable_298 = noise_metadata_schedule_374_e3505;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_375_e3508: f64 = if noise_variable_298 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_533 = noise_metadata_schedule_375_e3508;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_376_e3513,) = {
    if (noise_variable_533 != 0.0) {
        let noise_metadata_schedule_376_e3511: f64 = (noise_variable_298).exp();
        (noise_metadata_schedule_376_e3511,)
    } else {
        (noise_variable_299,)
    }
};
            noise_variable_299 = noise_metadata_schedule_376_e3513;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_377_e3519,) = {
    if (noise_variable_533 == 0.0) {
        let noise_metadata_schedule_377_e3517: f64 = (params.p147).exp();
        (noise_metadata_schedule_377_e3517,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_377_e3519;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_378_e3530,) = {
    if (noise_variable_533 == 0.0) {
        let noise_metadata_schedule_378_e3526: f64 = (noise_variable_298 - params.p147);
        let noise_metadata_schedule_378_e3527: f64 = (1.0 + noise_metadata_schedule_378_e3526);
        let noise_metadata_schedule_378_e3528: f64 = (noise_variable_295 * noise_metadata_schedule_378_e3527);
        (noise_metadata_schedule_378_e3528,)
    } else {
        (noise_variable_299,)
    }
};
            noise_variable_299 = noise_metadata_schedule_378_e3530;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_379_e3534: f64 = (noise_variable_299 - 1.0);
            let noise_metadata_schedule_379_e3535: f64 = (noise_variable_350 * noise_metadata_schedule_379_e3534);
            noise_variable_351 = noise_metadata_schedule_379_e3535;
        }
        if matches!(source_index, 1 | 2) {
            let noise_metadata_schedule_380_e3538: f64 = (noise_variable_246 - params.p154);
            let noise_metadata_schedule_380_e3540: f64 = (noise_metadata_schedule_380_e3538 / 0.001);
            noise_variable_279 = noise_metadata_schedule_380_e3540;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_381_e3543: f64 = if noise_variable_246 < params.p154 { 1.0 } else { 0.0 };
            noise_variable_534 = noise_metadata_schedule_381_e3543;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_382_e3555,) = {
    if (noise_variable_534 != 0.0) {
        let noise_metadata_schedule_382_e3549: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_382_e3550: f64 = (1.0 + noise_metadata_schedule_382_e3549);
        let noise_metadata_schedule_382_e3551: f64 = (noise_metadata_schedule_382_e3550).ln();
        let noise_metadata_schedule_382_e3552: f64 = (0.001 * noise_metadata_schedule_382_e3551);
        let noise_metadata_schedule_382_e3553: f64 = (noise_variable_246 - noise_metadata_schedule_382_e3552);
        (noise_metadata_schedule_382_e3553,)
    } else {
        (noise_variable_300,)
    }
};
            noise_variable_300 = noise_metadata_schedule_382_e3555;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_383_e3569,) = {
    if (noise_variable_534 == 0.0) {
        let noise_metadata_schedule_383_e3562: f64 = (-noise_variable_279);
        let noise_metadata_schedule_383_e3563: f64 = (noise_metadata_schedule_383_e3562).exp();
        let noise_metadata_schedule_383_e3564: f64 = (1.0 + noise_metadata_schedule_383_e3563);
        let noise_metadata_schedule_383_e3565: f64 = (noise_metadata_schedule_383_e3564).ln();
        let noise_metadata_schedule_383_e3566: f64 = (0.001 * noise_metadata_schedule_383_e3565);
        let noise_metadata_schedule_383_e3567: f64 = (params.p154 - noise_metadata_schedule_383_e3566);
        (noise_metadata_schedule_383_e3567,)
    } else {
        (noise_variable_300,)
    }
};
            noise_variable_300 = noise_metadata_schedule_383_e3569;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_384_e3572: f64 = (params.p155 * noise_variable_300);
            let noise_metadata_schedule_384_e3575: f64 = (params.p154 - noise_variable_300);
            let noise_metadata_schedule_384_e3577: f64 = {let pb=noise_metadata_schedule_384_e3575;pb*pb};
            let noise_metadata_schedule_384_e3578: f64 = (noise_metadata_schedule_384_e3572 * noise_metadata_schedule_384_e3577);
            noise_variable_352 = noise_metadata_schedule_384_e3578;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_385_e3581: f64 = (noise_variable_246 * noise_variable_8);
            let noise_metadata_schedule_385_e3583: f64 = (noise_metadata_schedule_385_e3581 / params.p17);
            let noise_metadata_schedule_385_e3585: f64 = if noise_metadata_schedule_385_e3583 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_535 = noise_metadata_schedule_385_e3585;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_386_e3594,) = {
    if (noise_variable_535 != 0.0) {
        let noise_metadata_schedule_386_e3589: f64 = (noise_variable_246 * noise_variable_8);
        let noise_metadata_schedule_386_e3591: f64 = (noise_metadata_schedule_386_e3589 / params.p17);
        let noise_metadata_schedule_386_e3592: f64 = (noise_metadata_schedule_386_e3591).exp();
        (noise_metadata_schedule_386_e3592,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_386_e3594;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_387_e3600,) = {
    if (noise_variable_535 == 0.0) {
        let noise_metadata_schedule_387_e3598: f64 = (params.p147).exp();
        (noise_metadata_schedule_387_e3598,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_387_e3600;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_388_e3615,) = {
    if (noise_variable_535 == 0.0) {
        let noise_metadata_schedule_388_e3607: f64 = (noise_variable_246 * noise_variable_8);
        let noise_metadata_schedule_388_e3609: f64 = (noise_metadata_schedule_388_e3607 / params.p17);
        let noise_metadata_schedule_388_e3611: f64 = (noise_metadata_schedule_388_e3609 - params.p147);
        let noise_metadata_schedule_388_e3612: f64 = (1.0 + noise_metadata_schedule_388_e3611);
        let noise_metadata_schedule_388_e3613: f64 = (noise_variable_295 * noise_metadata_schedule_388_e3612);
        (noise_metadata_schedule_388_e3613,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_388_e3615;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_389_e3618: f64 = if params.p24 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_536 = noise_metadata_schedule_389_e3618;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_390_e3621: f64 = (noise_variable_246 - noise_variable_55);
            let noise_metadata_schedule_390_e3623: f64 = (noise_metadata_schedule_390_e3621 * noise_variable_8);
            let noise_metadata_schedule_390_e3625: f64 = if noise_metadata_schedule_390_e3623 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_537 = noise_metadata_schedule_390_e3625;
        }
        if matches!(source_index, 2 | 6 | 8) {
            let (noise_metadata_schedule_391_e3636,) = {
    if ((noise_variable_536 != 0.0) && (noise_variable_537 != 0.0)) {
        let noise_metadata_schedule_391_e3631: f64 = (noise_variable_246 - noise_variable_55);
        let noise_metadata_schedule_391_e3633: f64 = (noise_metadata_schedule_391_e3631 * noise_variable_8);
        let noise_metadata_schedule_391_e3634: f64 = (noise_metadata_schedule_391_e3633).exp();
        (noise_metadata_schedule_391_e3634,)
    } else {
        (noise_variable_298,)
    }
};
            noise_variable_298 = noise_metadata_schedule_391_e3636;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_392_e3644,) = {
    if ((noise_variable_536 != 0.0) && (noise_variable_537 == 0.0)) {
        let noise_metadata_schedule_392_e3642: f64 = (params.p147).exp();
        (noise_metadata_schedule_392_e3642,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_392_e3644;
        }
        if matches!(source_index, 2 | 6 | 8) {
            let (noise_metadata_schedule_393_e3661,) = {
    if ((noise_variable_536 != 0.0) && (noise_variable_537 == 0.0)) {
        let noise_metadata_schedule_393_e3653: f64 = (noise_variable_246 - noise_variable_55);
        let noise_metadata_schedule_393_e3655: f64 = (noise_metadata_schedule_393_e3653 * noise_variable_8);
        let noise_metadata_schedule_393_e3657: f64 = (noise_metadata_schedule_393_e3655 - params.p147);
        let noise_metadata_schedule_393_e3658: f64 = (1.0 + noise_metadata_schedule_393_e3657);
        let noise_metadata_schedule_393_e3659: f64 = (noise_variable_295 * noise_metadata_schedule_393_e3658);
        (noise_metadata_schedule_393_e3659,)
    } else {
        (noise_variable_298,)
    }
};
            noise_variable_298 = noise_metadata_schedule_393_e3661;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_394_e3664: f64 = (noise_variable_156 / noise_variable_35);
            let noise_metadata_schedule_394_e3666: f64 = (noise_metadata_schedule_394_e3664 - 1000.0);
            let noise_metadata_schedule_394_e3668: f64 = if noise_metadata_schedule_394_e3666 < 40.0 { 1.0 } else { 0.0 };
            noise_variable_538 = noise_metadata_schedule_394_e3668;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_395_e3679,) = {
    if ((noise_variable_536 != 0.0) && (noise_variable_538 != 0.0)) {
        let noise_metadata_schedule_395_e3674: f64 = (noise_variable_156 / noise_variable_35);
        let noise_metadata_schedule_395_e3676: f64 = (noise_metadata_schedule_395_e3674 - 1000.0);
        let noise_metadata_schedule_395_e3677: f64 = (noise_metadata_schedule_395_e3676).exp();
        (noise_metadata_schedule_395_e3677,)
    } else {
        (noise_variable_299,)
    }
};
            noise_variable_299 = noise_metadata_schedule_395_e3679;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_396_e3687,) = {
    if ((noise_variable_536 != 0.0) && (noise_variable_538 == 0.0)) {
        let noise_metadata_schedule_396_e3685: f64 = (40.0_f64).exp();
        (noise_metadata_schedule_396_e3685,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_396_e3687;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_397_e3704,) = {
    if ((noise_variable_536 != 0.0) && (noise_variable_538 == 0.0)) {
        let noise_metadata_schedule_397_e3696: f64 = (noise_variable_156 / noise_variable_35);
        let noise_metadata_schedule_397_e3698: f64 = (noise_metadata_schedule_397_e3696 - 1000.0);
        let noise_metadata_schedule_397_e3700: f64 = (noise_metadata_schedule_397_e3698 - 40.0);
        let noise_metadata_schedule_397_e3701: f64 = (1.0 + noise_metadata_schedule_397_e3700);
        let noise_metadata_schedule_397_e3702: f64 = (noise_variable_295 * noise_metadata_schedule_397_e3701);
        (noise_metadata_schedule_397_e3702,)
    } else {
        (noise_variable_299,)
    }
};
            noise_variable_299 = noise_metadata_schedule_397_e3704;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_398_e3747,) = {
    if (noise_variable_536 != 0.0) {
        let noise_metadata_schedule_398_e3709: f64 = (noise_variable_296 - 1.0);
        let noise_metadata_schedule_398_e3710: f64 = (noise_variable_42 * noise_metadata_schedule_398_e3709);
        let noise_metadata_schedule_398_e3713: f64 = (noise_variable_53 * 2.0);
        let noise_metadata_schedule_398_e3716: f64 = (noise_variable_296 - 1.0);
        let noise_metadata_schedule_398_e3717: f64 = (noise_metadata_schedule_398_e3713 * noise_metadata_schedule_398_e3716);
        let noise_metadata_schedule_398_e3722: f64 = (4.0 * noise_variable_298);
        let noise_metadata_schedule_398_e3723: f64 = (1.0 + noise_metadata_schedule_398_e3722);
        let noise_metadata_schedule_398_e3724: f64 = (noise_metadata_schedule_398_e3723).sqrt();
        let noise_metadata_schedule_398_e3725: f64 = (1.0 + noise_metadata_schedule_398_e3724);
        let noise_metadata_schedule_398_e3726: f64 = (noise_metadata_schedule_398_e3717 / noise_metadata_schedule_398_e3725);
        let noise_metadata_schedule_398_e3730: f64 = (noise_variable_145 / noise_variable_40);
        let noise_metadata_schedule_398_e3731: f64 = (1.0 + noise_metadata_schedule_398_e3730);
        let noise_metadata_schedule_398_e3732: f64 = (noise_metadata_schedule_398_e3726 * noise_metadata_schedule_398_e3731);
        let noise_metadata_schedule_398_e3733: f64 = (noise_metadata_schedule_398_e3710 + noise_metadata_schedule_398_e3732);
        let noise_metadata_schedule_398_e3737: f64 = (noise_variable_128 - 1.0);
        let noise_metadata_schedule_398_e3738: f64 = (noise_variable_54 * noise_metadata_schedule_398_e3737);
        let noise_metadata_schedule_398_e3740: f64 = (noise_metadata_schedule_398_e3738 * noise_variable_299);
        let noise_metadata_schedule_398_e3743: f64 = (1.0 + noise_variable_299);
        let noise_metadata_schedule_398_e3744: f64 = (noise_metadata_schedule_398_e3740 / noise_metadata_schedule_398_e3743);
        let noise_metadata_schedule_398_e3745: f64 = (noise_metadata_schedule_398_e3733 + noise_metadata_schedule_398_e3744);
        (noise_metadata_schedule_398_e3745,)
    } else {
        (noise_variable_158,)
    }
};
            noise_variable_158 = noise_metadata_schedule_398_e3747;
        }
        if matches!(source_index, 2 | 6) {
            let noise_metadata_schedule_399_e3750: f64 = if params.p93 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_539 = noise_metadata_schedule_399_e3750;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_400_e3761,) = {
    if ((noise_variable_536 == 0.0) && (noise_variable_539 != 0.0)) {
        let noise_metadata_schedule_400_e3758: f64 = (noise_variable_296 - 1.0);
        let noise_metadata_schedule_400_e3759: f64 = (noise_variable_42 * noise_metadata_schedule_400_e3758);
        (noise_metadata_schedule_400_e3759,)
    } else {
        (noise_variable_158,)
    }
};
            noise_variable_158 = noise_metadata_schedule_400_e3761;
        }
        if matches!(source_index, 2 | 6) {
            let (noise_metadata_schedule_401_e3791,) = {
    if ((noise_variable_536 == 0.0) && (noise_variable_539 == 0.0)) {
        let noise_metadata_schedule_401_e3770: f64 = (1.0 - params.p93);
        let noise_metadata_schedule_401_e3773: f64 = (noise_variable_296 - 1.0);
        let noise_metadata_schedule_401_e3774: f64 = (noise_metadata_schedule_401_e3770 * noise_metadata_schedule_401_e3773);
        let noise_metadata_schedule_401_e3778: f64 = (noise_variable_296 + noise_variable_128);
        let noise_metadata_schedule_401_e3780: f64 = (noise_metadata_schedule_401_e3778 - 2.0);
        let noise_metadata_schedule_401_e3781: f64 = (params.p93 * noise_metadata_schedule_401_e3780);
        let noise_metadata_schedule_401_e3785: f64 = (noise_variable_145 / noise_variable_40);
        let noise_metadata_schedule_401_e3786: f64 = (1.0 + noise_metadata_schedule_401_e3785);
        let noise_metadata_schedule_401_e3787: f64 = (noise_metadata_schedule_401_e3781 * noise_metadata_schedule_401_e3786);
        let noise_metadata_schedule_401_e3788: f64 = (noise_metadata_schedule_401_e3774 + noise_metadata_schedule_401_e3787);
        let noise_metadata_schedule_401_e3789: f64 = (noise_variable_42 * noise_metadata_schedule_401_e3788);
        (noise_metadata_schedule_401_e3789,)
    } else {
        (noise_variable_158,)
    }
};
            noise_variable_158 = noise_metadata_schedule_401_e3791;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_402_e3794: f64 = (noise_variable_247 * noise_variable_8);
            let noise_metadata_schedule_402_e3796: f64 = (noise_metadata_schedule_402_e3794 / params.p19);
            let noise_metadata_schedule_402_e3798: f64 = if noise_metadata_schedule_402_e3796 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_540 = noise_metadata_schedule_402_e3798;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_403_e3807,) = {
    if (noise_variable_540 != 0.0) {
        let noise_metadata_schedule_403_e3802: f64 = (noise_variable_247 * noise_variable_8);
        let noise_metadata_schedule_403_e3804: f64 = (noise_metadata_schedule_403_e3802 / params.p19);
        let noise_metadata_schedule_403_e3805: f64 = (noise_metadata_schedule_403_e3804).exp();
        (noise_metadata_schedule_403_e3805,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_403_e3807;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_404_e3813,) = {
    if (noise_variable_540 == 0.0) {
        let noise_metadata_schedule_404_e3811: f64 = (params.p147).exp();
        (noise_metadata_schedule_404_e3811,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_404_e3813;
        }
        if matches!(source_index, 2 | 6 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_405_e3828,) = {
    if (noise_variable_540 == 0.0) {
        let noise_metadata_schedule_405_e3820: f64 = (noise_variable_247 * noise_variable_8);
        let noise_metadata_schedule_405_e3822: f64 = (noise_metadata_schedule_405_e3820 / params.p19);
        let noise_metadata_schedule_405_e3824: f64 = (noise_metadata_schedule_405_e3822 - params.p147);
        let noise_metadata_schedule_405_e3825: f64 = (1.0 + noise_metadata_schedule_405_e3824);
        let noise_metadata_schedule_405_e3826: f64 = (noise_variable_295 * noise_metadata_schedule_405_e3825);
        (noise_metadata_schedule_405_e3826,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_405_e3828;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_406_e3831: f64 = if params.p24 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_541 = noise_metadata_schedule_406_e3831;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_407_e3834: f64 = (noise_variable_247 - noise_variable_55);
            let noise_metadata_schedule_407_e3836: f64 = (noise_metadata_schedule_407_e3834 * noise_variable_8);
            let noise_metadata_schedule_407_e3838: f64 = if noise_metadata_schedule_407_e3836 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_542 = noise_metadata_schedule_407_e3838;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_408_e3849,) = {
    if ((noise_variable_541 != 0.0) && (noise_variable_542 != 0.0)) {
        let noise_metadata_schedule_408_e3844: f64 = (noise_variable_247 - noise_variable_55);
        let noise_metadata_schedule_408_e3846: f64 = (noise_metadata_schedule_408_e3844 * noise_variable_8);
        let noise_metadata_schedule_408_e3847: f64 = (noise_metadata_schedule_408_e3846).exp();
        (noise_metadata_schedule_408_e3847,)
    } else {
        (noise_variable_298,)
    }
};
            noise_variable_298 = noise_metadata_schedule_408_e3849;
        }
        if matches!(source_index, 1 | 2 | 6 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_409_e3857,) = {
    if ((noise_variable_541 != 0.0) && (noise_variable_542 == 0.0)) {
        let noise_metadata_schedule_409_e3855: f64 = (params.p147).exp();
        (noise_metadata_schedule_409_e3855,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_409_e3857;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_410_e3874,) = {
    if ((noise_variable_541 != 0.0) && (noise_variable_542 == 0.0)) {
        let noise_metadata_schedule_410_e3866: f64 = (noise_variable_247 - noise_variable_55);
        let noise_metadata_schedule_410_e3868: f64 = (noise_metadata_schedule_410_e3866 * noise_variable_8);
        let noise_metadata_schedule_410_e3870: f64 = (noise_metadata_schedule_410_e3868 - params.p147);
        let noise_metadata_schedule_410_e3871: f64 = (1.0 + noise_metadata_schedule_410_e3870);
        let noise_metadata_schedule_410_e3872: f64 = (noise_variable_295 * noise_metadata_schedule_410_e3871);
        (noise_metadata_schedule_410_e3872,)
    } else {
        (noise_variable_298,)
    }
};
            noise_variable_298 = noise_metadata_schedule_410_e3874;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_411_e3899,) = {
    if (noise_variable_541 != 0.0) {
        let noise_metadata_schedule_411_e3879: f64 = (noise_variable_296 - 1.0);
        let noise_metadata_schedule_411_e3880: f64 = (noise_variable_44 * noise_metadata_schedule_411_e3879);
        let noise_metadata_schedule_411_e3883: f64 = (noise_variable_45 * 2.0);
        let noise_metadata_schedule_411_e3886: f64 = (noise_variable_296 - 1.0);
        let noise_metadata_schedule_411_e3887: f64 = (noise_metadata_schedule_411_e3883 * noise_metadata_schedule_411_e3886);
        let noise_metadata_schedule_411_e3892: f64 = (4.0 * noise_variable_298);
        let noise_metadata_schedule_411_e3893: f64 = (1.0 + noise_metadata_schedule_411_e3892);
        let noise_metadata_schedule_411_e3894: f64 = (noise_metadata_schedule_411_e3893).sqrt();
        let noise_metadata_schedule_411_e3895: f64 = (1.0 + noise_metadata_schedule_411_e3894);
        let noise_metadata_schedule_411_e3896: f64 = (noise_metadata_schedule_411_e3887 / noise_metadata_schedule_411_e3895);
        let noise_metadata_schedule_411_e3897: f64 = (noise_metadata_schedule_411_e3880 + noise_metadata_schedule_411_e3896);
        (noise_metadata_schedule_411_e3897,)
    } else {
        (noise_variable_159,)
    }
};
            noise_variable_159 = noise_metadata_schedule_411_e3899;
        }
        if matches!(source_index, 6 | 8) {
            let (noise_metadata_schedule_412_e3908,) = {
    if (noise_variable_541 == 0.0) {
        let noise_metadata_schedule_412_e3905: f64 = (noise_variable_296 - 1.0);
        let noise_metadata_schedule_412_e3906: f64 = (noise_variable_44 * noise_metadata_schedule_412_e3905);
        (noise_metadata_schedule_412_e3906,)
    } else {
        (noise_variable_159,)
    }
};
            noise_variable_159 = noise_metadata_schedule_412_e3908;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_413_e3911: f64 = (noise_variable_246 * noise_variable_8);
            let noise_metadata_schedule_413_e3913: f64 = (noise_metadata_schedule_413_e3911 / params.p21);
            let noise_metadata_schedule_413_e3915: f64 = if noise_metadata_schedule_413_e3913 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_543 = noise_metadata_schedule_413_e3915;
        }
        if matches!(source_index, 2 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_414_e3924,) = {
    if (noise_variable_543 != 0.0) {
        let noise_metadata_schedule_414_e3919: f64 = (noise_variable_246 * noise_variable_8);
        let noise_metadata_schedule_414_e3921: f64 = (noise_metadata_schedule_414_e3919 / params.p21);
        let noise_metadata_schedule_414_e3922: f64 = (noise_metadata_schedule_414_e3921).exp();
        (noise_metadata_schedule_414_e3922,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_414_e3924;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_415_e3930,) = {
    if (noise_variable_543 == 0.0) {
        let noise_metadata_schedule_415_e3928: f64 = (params.p147).exp();
        (noise_metadata_schedule_415_e3928,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_415_e3930;
        }
        if matches!(source_index, 2 | 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_416_e3945,) = {
    if (noise_variable_543 == 0.0) {
        let noise_metadata_schedule_416_e3937: f64 = (noise_variable_246 * noise_variable_8);
        let noise_metadata_schedule_416_e3939: f64 = (noise_metadata_schedule_416_e3937 / params.p21);
        let noise_metadata_schedule_416_e3941: f64 = (noise_metadata_schedule_416_e3939 - params.p147);
        let noise_metadata_schedule_416_e3942: f64 = (1.0 + noise_metadata_schedule_416_e3941);
        let noise_metadata_schedule_416_e3943: f64 = (noise_variable_295 * noise_metadata_schedule_416_e3942);
        (noise_metadata_schedule_416_e3943,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_416_e3945;
        }
        if matches!(source_index, 2 | 7) {
            let noise_metadata_schedule_417_e3949: f64 = (noise_variable_296 - 1.0);
            let noise_metadata_schedule_417_e3950: f64 = (noise_variable_38 * noise_metadata_schedule_417_e3949);
            noise_variable_160 = noise_metadata_schedule_417_e3950;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_418_e3953: f64 = (noise_variable_247 * noise_variable_8);
            let noise_metadata_schedule_418_e3955: f64 = (noise_metadata_schedule_418_e3953 / params.p23);
            let noise_metadata_schedule_418_e3957: f64 = if noise_metadata_schedule_418_e3955 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_544 = noise_metadata_schedule_418_e3957;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_419_e3966,) = {
    if (noise_variable_544 != 0.0) {
        let noise_metadata_schedule_419_e3961: f64 = (noise_variable_247 * noise_variable_8);
        let noise_metadata_schedule_419_e3963: f64 = (noise_metadata_schedule_419_e3961 / params.p23);
        let noise_metadata_schedule_419_e3964: f64 = (noise_metadata_schedule_419_e3963).exp();
        (noise_metadata_schedule_419_e3964,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_419_e3966;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_420_e3972,) = {
    if (noise_variable_544 == 0.0) {
        let noise_metadata_schedule_420_e3970: f64 = (params.p147).exp();
        (noise_metadata_schedule_420_e3970,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_420_e3972;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_421_e3987,) = {
    if (noise_variable_544 == 0.0) {
        let noise_metadata_schedule_421_e3979: f64 = (noise_variable_247 * noise_variable_8);
        let noise_metadata_schedule_421_e3981: f64 = (noise_metadata_schedule_421_e3979 / params.p23);
        let noise_metadata_schedule_421_e3983: f64 = (noise_metadata_schedule_421_e3981 - params.p147);
        let noise_metadata_schedule_421_e3984: f64 = (1.0 + noise_metadata_schedule_421_e3983);
        let noise_metadata_schedule_421_e3985: f64 = (noise_variable_295 * noise_metadata_schedule_421_e3984);
        (noise_metadata_schedule_421_e3985,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_421_e3987;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_422_e3991: f64 = (noise_variable_296 - 1.0);
            let noise_metadata_schedule_422_e3992: f64 = (noise_variable_46 * noise_metadata_schedule_422_e3991);
            noise_variable_162 = noise_metadata_schedule_422_e3992;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let noise_metadata_schedule_423_e3995: f64 = (noise_variable_249 * noise_variable_8);
            let noise_metadata_schedule_423_e3997: f64 = (noise_metadata_schedule_423_e3995 / params.p32);
            let noise_metadata_schedule_423_e3999: f64 = if noise_metadata_schedule_423_e3997 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_545 = noise_metadata_schedule_423_e3999;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_424_e4008,) = {
    if (noise_variable_545 != 0.0) {
        let noise_metadata_schedule_424_e4003: f64 = (noise_variable_249 * noise_variable_8);
        let noise_metadata_schedule_424_e4005: f64 = (noise_metadata_schedule_424_e4003 / params.p32);
        let noise_metadata_schedule_424_e4006: f64 = (noise_metadata_schedule_424_e4005).exp();
        (noise_metadata_schedule_424_e4006,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_424_e4008;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 9 | 10 | 15 | 16) {
            let (noise_metadata_schedule_425_e4014,) = {
    if (noise_variable_545 == 0.0) {
        let noise_metadata_schedule_425_e4012: f64 = (params.p147).exp();
        (noise_metadata_schedule_425_e4012,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_425_e4014;
        }
        if matches!(source_index, 7 | 8 | 9 | 10) {
            let (noise_metadata_schedule_426_e4029,) = {
    if (noise_variable_545 == 0.0) {
        let noise_metadata_schedule_426_e4021: f64 = (noise_variable_249 * noise_variable_8);
        let noise_metadata_schedule_426_e4023: f64 = (noise_metadata_schedule_426_e4021 / params.p32);
        let noise_metadata_schedule_426_e4025: f64 = (noise_metadata_schedule_426_e4023 - params.p147);
        let noise_metadata_schedule_426_e4026: f64 = (1.0 + noise_metadata_schedule_426_e4025);
        let noise_metadata_schedule_426_e4027: f64 = (noise_variable_295 * noise_metadata_schedule_426_e4026);
        (noise_metadata_schedule_426_e4027,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_426_e4029;
        }
        if matches!(source_index, 9 | 10) {
            let noise_metadata_schedule_427_e4033: f64 = (noise_variable_296 - 1.0);
            let noise_metadata_schedule_427_e4034: f64 = (noise_variable_39 * noise_metadata_schedule_427_e4033);
            noise_variable_161 = noise_metadata_schedule_427_e4034;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 15 | 16) {
            let noise_metadata_schedule_428_e4037: f64 = (noise_variable_247 * noise_variable_8);
            let noise_metadata_schedule_428_e4039: f64 = (noise_metadata_schedule_428_e4037 / params.p146);
            let noise_metadata_schedule_428_e4041: f64 = if noise_metadata_schedule_428_e4039 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_546 = noise_metadata_schedule_428_e4041;
        }
        if matches!(source_index, 7 | 8) {
            let (noise_metadata_schedule_429_e4050,) = {
    if (noise_variable_546 != 0.0) {
        let noise_metadata_schedule_429_e4045: f64 = (noise_variable_247 * noise_variable_8);
        let noise_metadata_schedule_429_e4047: f64 = (noise_metadata_schedule_429_e4045 / params.p146);
        let noise_metadata_schedule_429_e4048: f64 = (noise_metadata_schedule_429_e4047).exp();
        (noise_metadata_schedule_429_e4048,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_429_e4050;
        }
        if matches!(source_index, 1 | 2 | 7 | 8 | 15 | 16) {
            let (noise_metadata_schedule_430_e4056,) = {
    if (noise_variable_546 == 0.0) {
        let noise_metadata_schedule_430_e4054: f64 = (params.p147).exp();
        (noise_metadata_schedule_430_e4054,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_430_e4056;
        }
        if matches!(source_index, 7 | 8) {
            let (noise_metadata_schedule_431_e4071,) = {
    if (noise_variable_546 == 0.0) {
        let noise_metadata_schedule_431_e4063: f64 = (noise_variable_247 * noise_variable_8);
        let noise_metadata_schedule_431_e4065: f64 = (noise_metadata_schedule_431_e4063 / params.p146);
        let noise_metadata_schedule_431_e4067: f64 = (noise_metadata_schedule_431_e4065 - params.p147);
        let noise_metadata_schedule_431_e4068: f64 = (1.0 + noise_metadata_schedule_431_e4067);
        let noise_metadata_schedule_431_e4069: f64 = (noise_variable_295 * noise_metadata_schedule_431_e4068);
        (noise_metadata_schedule_431_e4069,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_431_e4071;
        }
        if matches!(source_index, 7 | 8) {
            let noise_metadata_schedule_432_e4075: f64 = (noise_variable_296 - 1.0);
            let noise_metadata_schedule_432_e4076: f64 = (noise_variable_47 * noise_metadata_schedule_432_e4075);
            noise_variable_163 = noise_metadata_schedule_432_e4076;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_433_e4087: f64 = if (((params.p34 > 0.0) && (params.p35 > 0.0)) && (noise_variable_246 < 0.0)) { 1.0 } else { 0.0 };
            noise_variable_547 = noise_metadata_schedule_433_e4087;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_434_e4093: f64 = (2.0 * noise_variable_59);
            let noise_metadata_schedule_434_e4094: f64 = (noise_variable_62 / noise_metadata_schedule_434_e4093);
            let noise_metadata_schedule_434_e4095: f64 = (1.0 - noise_metadata_schedule_434_e4094);
            let noise_metadata_schedule_434_e4096: f64 = (noise_variable_61 * noise_metadata_schedule_434_e4095);
            let noise_metadata_schedule_434_e4098: f64 = if noise_metadata_schedule_434_e4096 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_548 = noise_metadata_schedule_434_e4098;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_435_e4113,) = {
    if ((noise_variable_547 != 0.0) && (noise_variable_548 != 0.0)) {
        let noise_metadata_schedule_435_e4107: f64 = (2.0 * noise_variable_59);
        let noise_metadata_schedule_435_e4108: f64 = (noise_variable_62 / noise_metadata_schedule_435_e4107);
        let noise_metadata_schedule_435_e4109: f64 = (1.0 - noise_metadata_schedule_435_e4108);
        let noise_metadata_schedule_435_e4110: f64 = (noise_variable_61 * noise_metadata_schedule_435_e4109);
        let noise_metadata_schedule_435_e4111: f64 = (noise_metadata_schedule_435_e4110).exp();
        (noise_metadata_schedule_435_e4111,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_435_e4113;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_436_e4121,) = {
    if ((noise_variable_547 != 0.0) && (noise_variable_548 == 0.0)) {
        let noise_metadata_schedule_436_e4119: f64 = (params.p147).exp();
        (noise_metadata_schedule_436_e4119,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_436_e4121;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_437_e4142,) = {
    if ((noise_variable_547 != 0.0) && (noise_variable_548 == 0.0)) {
        let noise_metadata_schedule_437_e4133: f64 = (2.0 * noise_variable_59);
        let noise_metadata_schedule_437_e4134: f64 = (noise_variable_62 / noise_metadata_schedule_437_e4133);
        let noise_metadata_schedule_437_e4135: f64 = (1.0 - noise_metadata_schedule_437_e4134);
        let noise_metadata_schedule_437_e4136: f64 = (noise_variable_61 * noise_metadata_schedule_437_e4135);
        let noise_metadata_schedule_437_e4138: f64 = (noise_metadata_schedule_437_e4136 - params.p147);
        let noise_metadata_schedule_437_e4139: f64 = (1.0 + noise_metadata_schedule_437_e4138);
        let noise_metadata_schedule_437_e4140: f64 = (noise_variable_295 * noise_metadata_schedule_437_e4139);
        (noise_metadata_schedule_437_e4140,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_437_e4142;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_438_e4148,) = {
    if (noise_variable_547 != 0.0) {
        let noise_metadata_schedule_438_e4146: f64 = (noise_variable_246 * noise_variable_65);
        (noise_metadata_schedule_438_e4146,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_438_e4148;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_439_e4192,) = {
    if (noise_variable_547 != 0.0) {
        let noise_metadata_schedule_439_e4152: f64 = (noise_variable_275 * noise_variable_275);
        let noise_metadata_schedule_439_e4154: f64 = (noise_metadata_schedule_439_e4152 + 1e-30);
        let noise_metadata_schedule_439_e4155: f64 = (noise_metadata_schedule_439_e4154).sqrt();
        let noise_metadata_schedule_439_e4157: f64 = (-2.0);
        let noise_metadata_schedule_439_e4159: f64 = (noise_metadata_schedule_439_e4157 - params.p67);
        let noise_metadata_schedule_439_e4160: f64 = (noise_metadata_schedule_439_e4155).powf(noise_metadata_schedule_439_e4159);
        let noise_metadata_schedule_439_e4165: f64 = (params.p67 * params.p67);
        let noise_metadata_schedule_439_e4166: f64 = (1.0 - noise_metadata_schedule_439_e4165);
        let noise_metadata_schedule_439_e4169: f64 = (3.0 * noise_variable_275);
        let noise_metadata_schedule_439_e4172: f64 = (params.p67 - 1.0);
        let noise_metadata_schedule_439_e4173: f64 = (noise_metadata_schedule_439_e4169 * noise_metadata_schedule_439_e4172);
        let noise_metadata_schedule_439_e4174: f64 = (noise_metadata_schedule_439_e4166 - noise_metadata_schedule_439_e4173);
        let noise_metadata_schedule_439_e4175: f64 = (params.p67 * noise_metadata_schedule_439_e4174);
        let noise_metadata_schedule_439_e4178: f64 = (6.0 * noise_variable_275);
        let noise_metadata_schedule_439_e4180: f64 = (noise_metadata_schedule_439_e4178 * noise_variable_275);
        let noise_metadata_schedule_439_e4183: f64 = (params.p67 - 1.0);
        let noise_metadata_schedule_439_e4185: f64 = (noise_metadata_schedule_439_e4183 + noise_variable_275);
        let noise_metadata_schedule_439_e4186: f64 = (noise_metadata_schedule_439_e4180 * noise_metadata_schedule_439_e4185);
        let noise_metadata_schedule_439_e4187: f64 = (noise_metadata_schedule_439_e4175 - noise_metadata_schedule_439_e4186);
        let noise_metadata_schedule_439_e4188: f64 = (noise_metadata_schedule_439_e4160 * noise_metadata_schedule_439_e4187);
        let noise_metadata_schedule_439_e4190: f64 = (noise_metadata_schedule_439_e4188 * 0.16666666666666666);
        (noise_metadata_schedule_439_e4190,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_439_e4192;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_440_e4204,) = {
    if (noise_variable_547 != 0.0) {
        let noise_metadata_schedule_440_e4196: f64 = (noise_variable_246 * noise_variable_62);
        let noise_metadata_schedule_440_e4198: f64 = (noise_metadata_schedule_440_e4196 * noise_variable_61);
        let noise_metadata_schedule_440_e4201: f64 = (noise_variable_70 * noise_variable_60);
        let noise_metadata_schedule_440_e4202: f64 = (noise_metadata_schedule_440_e4198 / noise_metadata_schedule_440_e4201);
        (noise_metadata_schedule_440_e4202,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_440_e4204;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_441_e4207: f64 = (-0.001);
            let noise_metadata_schedule_441_e4208: f64 = if noise_variable_275 < noise_metadata_schedule_441_e4207 { 1.0 } else { 0.0 };
            noise_variable_549 = noise_metadata_schedule_441_e4208;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let noise_metadata_schedule_442_e4211: f64 = if noise_variable_275 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_550 = noise_metadata_schedule_442_e4211;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_443_e4220,) = {
    if (((noise_variable_547 != 0.0) && (noise_variable_549 != 0.0)) && (noise_variable_550 != 0.0)) {
        let noise_metadata_schedule_443_e4218: f64 = (noise_variable_275).exp();
        (noise_metadata_schedule_443_e4218,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_443_e4220;
        }
        if matches!(source_index, 1 | 2 | 15 | 16) {
            let (noise_metadata_schedule_444_e4230,) = {
    if (((noise_variable_547 != 0.0) && (noise_variable_549 != 0.0)) && (noise_variable_550 == 0.0)) {
        let noise_metadata_schedule_444_e4228: f64 = (params.p147).exp();
        (noise_metadata_schedule_444_e4228,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_444_e4230;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_445_e4245,) = {
    if (((noise_variable_547 != 0.0) && (noise_variable_549 != 0.0)) && (noise_variable_550 == 0.0)) {
        let noise_metadata_schedule_445_e4241: f64 = (noise_variable_275 - params.p147);
        let noise_metadata_schedule_445_e4242: f64 = (1.0 + noise_metadata_schedule_445_e4241);
        let noise_metadata_schedule_445_e4243: f64 = (noise_variable_295 * noise_metadata_schedule_445_e4242);
        (noise_metadata_schedule_445_e4243,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_445_e4245;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_446_e4260,) = {
    if ((noise_variable_547 != 0.0) && (noise_variable_549 != 0.0)) {
        let noise_metadata_schedule_446_e4250: f64 = (-noise_variable_246);
        let noise_metadata_schedule_446_e4254: f64 = (1.0 - noise_variable_91);
        let noise_metadata_schedule_446_e4256: f64 = (noise_metadata_schedule_446_e4254 / noise_variable_275);
        let noise_metadata_schedule_446_e4257: f64 = (1.0 + noise_metadata_schedule_446_e4256);
        let noise_metadata_schedule_446_e4258: f64 = (noise_metadata_schedule_446_e4250 * noise_metadata_schedule_446_e4257);
        (noise_metadata_schedule_446_e4258,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_446_e4260;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_447_e4283,) = {
    if ((noise_variable_547 != 0.0) && (noise_variable_549 == 0.0)) {
        let noise_metadata_schedule_447_e4267: f64 = (noise_variable_246 * 0.5);
        let noise_metadata_schedule_447_e4269: f64 = (noise_metadata_schedule_447_e4267 * noise_variable_275);
        let noise_metadata_schedule_447_e4273: f64 = (noise_variable_275 * 0.3333333333333333);
        let noise_metadata_schedule_447_e4277: f64 = (0.25 * noise_variable_275);
        let noise_metadata_schedule_447_e4278: f64 = (1.0 + noise_metadata_schedule_447_e4277);
        let noise_metadata_schedule_447_e4279: f64 = (noise_metadata_schedule_447_e4273 * noise_metadata_schedule_447_e4278);
        let noise_metadata_schedule_447_e4280: f64 = (1.0 + noise_metadata_schedule_447_e4279);
        let noise_metadata_schedule_447_e4281: f64 = (noise_metadata_schedule_447_e4269 * noise_metadata_schedule_447_e4280);
        (noise_metadata_schedule_447_e4281,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_447_e4283;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_448_e4299,) = {
    if (noise_variable_547 != 0.0) {
        let noise_metadata_schedule_448_e4287: f64 = (2.0 * noise_variable_58);
        let noise_metadata_schedule_448_e4289: f64 = (noise_metadata_schedule_448_e4287 * noise_variable_69);
        let noise_metadata_schedule_448_e4291: f64 = (noise_metadata_schedule_448_e4289 * noise_variable_59);
        let noise_metadata_schedule_448_e4293: f64 = (noise_metadata_schedule_448_e4291 * noise_variable_68);
        let noise_metadata_schedule_448_e4295: f64 = (noise_metadata_schedule_448_e4293 * noise_variable_65);
        let noise_metadata_schedule_448_e4297: f64 = (noise_metadata_schedule_448_e4295 * noise_variable_63);
        (noise_metadata_schedule_448_e4297,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_448_e4299;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_450_e4309,) = {
    if (noise_variable_547 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_450_e4309;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_451_e4320: f64 = if (((params.p36 > 0.0) && (params.p37 > 0.0)) && (noise_variable_244 < 0.0)) { 1.0 } else { 0.0 };
            noise_variable_551 = noise_metadata_schedule_451_e4320;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_452_e4332,) = {
    if (noise_variable_551 != 0.0) {
        let noise_metadata_schedule_452_e4325: f64 = (noise_variable_244 * noise_variable_67);
        let noise_metadata_schedule_452_e4326: f64 = (1.0 - noise_metadata_schedule_452_e4325);
        let noise_metadata_schedule_452_e4329: f64 = (1.0 - noise_variable_76);
        let noise_metadata_schedule_452_e4330: f64 = (noise_metadata_schedule_452_e4326).powf(noise_metadata_schedule_452_e4329);
        (noise_metadata_schedule_452_e4330,)
    } else {
        (noise_variable_77,)
    }
};
            noise_variable_77 = noise_metadata_schedule_452_e4332;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_453_e4338: f64 = (2.0 * noise_variable_77);
            let noise_metadata_schedule_453_e4339: f64 = (noise_variable_79 / noise_metadata_schedule_453_e4338);
            let noise_metadata_schedule_453_e4340: f64 = (1.0 - noise_metadata_schedule_453_e4339);
            let noise_metadata_schedule_453_e4341: f64 = (noise_variable_83 * noise_metadata_schedule_453_e4340);
            let noise_metadata_schedule_453_e4343: f64 = if noise_metadata_schedule_453_e4341 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_552 = noise_metadata_schedule_453_e4343;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_454_e4358,) = {
    if ((noise_variable_551 != 0.0) && (noise_variable_552 != 0.0)) {
        let noise_metadata_schedule_454_e4352: f64 = (2.0 * noise_variable_77);
        let noise_metadata_schedule_454_e4353: f64 = (noise_variable_79 / noise_metadata_schedule_454_e4352);
        let noise_metadata_schedule_454_e4354: f64 = (1.0 - noise_metadata_schedule_454_e4353);
        let noise_metadata_schedule_454_e4355: f64 = (noise_variable_83 * noise_metadata_schedule_454_e4354);
        let noise_metadata_schedule_454_e4356: f64 = (noise_metadata_schedule_454_e4355).exp();
        (noise_metadata_schedule_454_e4356,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_454_e4358;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_455_e4366,) = {
    if ((noise_variable_551 != 0.0) && (noise_variable_552 == 0.0)) {
        let noise_metadata_schedule_455_e4364: f64 = (params.p147).exp();
        (noise_metadata_schedule_455_e4364,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_455_e4366;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_456_e4387,) = {
    if ((noise_variable_551 != 0.0) && (noise_variable_552 == 0.0)) {
        let noise_metadata_schedule_456_e4378: f64 = (2.0 * noise_variable_77);
        let noise_metadata_schedule_456_e4379: f64 = (noise_variable_79 / noise_metadata_schedule_456_e4378);
        let noise_metadata_schedule_456_e4380: f64 = (1.0 - noise_metadata_schedule_456_e4379);
        let noise_metadata_schedule_456_e4381: f64 = (noise_variable_83 * noise_metadata_schedule_456_e4380);
        let noise_metadata_schedule_456_e4383: f64 = (noise_metadata_schedule_456_e4381 - params.p147);
        let noise_metadata_schedule_456_e4384: f64 = (1.0 + noise_metadata_schedule_456_e4383);
        let noise_metadata_schedule_456_e4385: f64 = (noise_variable_295 * noise_metadata_schedule_456_e4384);
        (noise_metadata_schedule_456_e4385,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_456_e4387;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_457_e4393,) = {
    if (noise_variable_551 != 0.0) {
        let noise_metadata_schedule_457_e4391: f64 = (noise_variable_244 * noise_variable_67);
        (noise_metadata_schedule_457_e4391,)
    } else {
        (noise_variable_277,)
    }
};
            noise_variable_277 = noise_metadata_schedule_457_e4393;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_458_e4437,) = {
    if (noise_variable_551 != 0.0) {
        let noise_metadata_schedule_458_e4397: f64 = (noise_variable_277 * noise_variable_277);
        let noise_metadata_schedule_458_e4399: f64 = (noise_metadata_schedule_458_e4397 + 1e-30);
        let noise_metadata_schedule_458_e4400: f64 = (noise_metadata_schedule_458_e4399).sqrt();
        let noise_metadata_schedule_458_e4402: f64 = (-2.0);
        let noise_metadata_schedule_458_e4404: f64 = (noise_metadata_schedule_458_e4402 - noise_variable_76);
        let noise_metadata_schedule_458_e4405: f64 = (noise_metadata_schedule_458_e4400).powf(noise_metadata_schedule_458_e4404);
        let noise_metadata_schedule_458_e4410: f64 = (noise_variable_76 * noise_variable_76);
        let noise_metadata_schedule_458_e4411: f64 = (1.0 - noise_metadata_schedule_458_e4410);
        let noise_metadata_schedule_458_e4414: f64 = (3.0 * noise_variable_277);
        let noise_metadata_schedule_458_e4417: f64 = (noise_variable_76 - 1.0);
        let noise_metadata_schedule_458_e4418: f64 = (noise_metadata_schedule_458_e4414 * noise_metadata_schedule_458_e4417);
        let noise_metadata_schedule_458_e4419: f64 = (noise_metadata_schedule_458_e4411 - noise_metadata_schedule_458_e4418);
        let noise_metadata_schedule_458_e4420: f64 = (noise_variable_76 * noise_metadata_schedule_458_e4419);
        let noise_metadata_schedule_458_e4423: f64 = (6.0 * noise_variable_277);
        let noise_metadata_schedule_458_e4425: f64 = (noise_metadata_schedule_458_e4423 * noise_variable_277);
        let noise_metadata_schedule_458_e4428: f64 = (noise_variable_76 - 1.0);
        let noise_metadata_schedule_458_e4430: f64 = (noise_metadata_schedule_458_e4428 + noise_variable_277);
        let noise_metadata_schedule_458_e4431: f64 = (noise_metadata_schedule_458_e4425 * noise_metadata_schedule_458_e4430);
        let noise_metadata_schedule_458_e4432: f64 = (noise_metadata_schedule_458_e4420 - noise_metadata_schedule_458_e4431);
        let noise_metadata_schedule_458_e4433: f64 = (noise_metadata_schedule_458_e4405 * noise_metadata_schedule_458_e4432);
        let noise_metadata_schedule_458_e4435: f64 = (noise_metadata_schedule_458_e4433 * 0.16666666666666666);
        (noise_metadata_schedule_458_e4435,)
    } else {
        (noise_variable_80,)
    }
};
            noise_variable_80 = noise_metadata_schedule_458_e4437;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_459_e4449,) = {
    if (noise_variable_551 != 0.0) {
        let noise_metadata_schedule_459_e4441: f64 = (noise_variable_244 * noise_variable_79);
        let noise_metadata_schedule_459_e4443: f64 = (noise_metadata_schedule_459_e4441 * noise_variable_83);
        let noise_metadata_schedule_459_e4446: f64 = (noise_variable_85 * noise_variable_80);
        let noise_metadata_schedule_459_e4447: f64 = (noise_metadata_schedule_459_e4443 / noise_metadata_schedule_459_e4446);
        (noise_metadata_schedule_459_e4447,)
    } else {
        (noise_variable_277,)
    }
};
            noise_variable_277 = noise_metadata_schedule_459_e4449;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_460_e4452: f64 = (-0.001);
            let noise_metadata_schedule_460_e4453: f64 = if noise_variable_277 < noise_metadata_schedule_460_e4452 { 1.0 } else { 0.0 };
            noise_variable_553 = noise_metadata_schedule_460_e4453;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let noise_metadata_schedule_461_e4456: f64 = if noise_variable_277 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_554 = noise_metadata_schedule_461_e4456;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_462_e4465,) = {
    if (((noise_variable_551 != 0.0) && (noise_variable_553 != 0.0)) && (noise_variable_554 != 0.0)) {
        let noise_metadata_schedule_462_e4463: f64 = (noise_variable_277).exp();
        (noise_metadata_schedule_462_e4463,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_462_e4465;
        }
        if matches!(source_index, 1 | 15 | 16) {
            let (noise_metadata_schedule_463_e4475,) = {
    if (((noise_variable_551 != 0.0) && (noise_variable_553 != 0.0)) && (noise_variable_554 == 0.0)) {
        let noise_metadata_schedule_463_e4473: f64 = (params.p147).exp();
        (noise_metadata_schedule_463_e4473,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_463_e4475;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_464_e4490,) = {
    if (((noise_variable_551 != 0.0) && (noise_variable_553 != 0.0)) && (noise_variable_554 == 0.0)) {
        let noise_metadata_schedule_464_e4486: f64 = (noise_variable_277 - params.p147);
        let noise_metadata_schedule_464_e4487: f64 = (1.0 + noise_metadata_schedule_464_e4486);
        let noise_metadata_schedule_464_e4488: f64 = (noise_variable_295 * noise_metadata_schedule_464_e4487);
        (noise_metadata_schedule_464_e4488,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_464_e4490;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_465_e4505,) = {
    if ((noise_variable_551 != 0.0) && (noise_variable_553 != 0.0)) {
        let noise_metadata_schedule_465_e4495: f64 = (-noise_variable_244);
        let noise_metadata_schedule_465_e4499: f64 = (1.0 - noise_variable_92);
        let noise_metadata_schedule_465_e4501: f64 = (noise_metadata_schedule_465_e4499 / noise_variable_277);
        let noise_metadata_schedule_465_e4502: f64 = (1.0 + noise_metadata_schedule_465_e4501);
        let noise_metadata_schedule_465_e4503: f64 = (noise_metadata_schedule_465_e4495 * noise_metadata_schedule_465_e4502);
        (noise_metadata_schedule_465_e4503,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_465_e4505;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_466_e4528,) = {
    if ((noise_variable_551 != 0.0) && (noise_variable_553 == 0.0)) {
        let noise_metadata_schedule_466_e4512: f64 = (noise_variable_244 * 0.5);
        let noise_metadata_schedule_466_e4514: f64 = (noise_metadata_schedule_466_e4512 * noise_variable_277);
        let noise_metadata_schedule_466_e4518: f64 = (noise_variable_277 * 0.3333333333333333);
        let noise_metadata_schedule_466_e4522: f64 = (0.25 * noise_variable_277);
        let noise_metadata_schedule_466_e4523: f64 = (1.0 + noise_metadata_schedule_466_e4522);
        let noise_metadata_schedule_466_e4524: f64 = (noise_metadata_schedule_466_e4518 * noise_metadata_schedule_466_e4523);
        let noise_metadata_schedule_466_e4525: f64 = (1.0 + noise_metadata_schedule_466_e4524);
        let noise_metadata_schedule_466_e4526: f64 = (noise_metadata_schedule_466_e4514 * noise_metadata_schedule_466_e4525);
        (noise_metadata_schedule_466_e4526,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_466_e4528;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_467_e4544,) = {
    if (noise_variable_551 != 0.0) {
        let noise_metadata_schedule_467_e4532: f64 = (2.0 * noise_variable_84);
        let noise_metadata_schedule_467_e4534: f64 = (noise_metadata_schedule_467_e4532 * noise_variable_81);
        let noise_metadata_schedule_467_e4536: f64 = (noise_metadata_schedule_467_e4534 * noise_variable_77);
        let noise_metadata_schedule_467_e4538: f64 = (noise_metadata_schedule_467_e4536 * noise_variable_78);
        let noise_metadata_schedule_467_e4540: f64 = (noise_metadata_schedule_467_e4538 * noise_variable_67);
        let noise_metadata_schedule_467_e4542: f64 = (noise_metadata_schedule_467_e4540 * noise_variable_89);
        (noise_metadata_schedule_467_e4542,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_467_e4544;
        }
        if matches!(source_index, 15 | 16) {
            let (noise_metadata_schedule_469_e4554,) = {
    if (noise_variable_551 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_469_e4554;
        }
        if matches!(source_index, 11 | 12) {
            let noise_metadata_schedule_474_e4581: f64 = (2.0 * noise_variable_43);
            let noise_metadata_schedule_474_e4584: f64 = (noise_variable_268 - 1.0);
            let noise_metadata_schedule_474_e4585: f64 = (noise_metadata_schedule_474_e4581 * noise_metadata_schedule_474_e4584);
            let noise_metadata_schedule_474_e4590: f64 = (4.0 * noise_variable_43);
            let noise_metadata_schedule_474_e4592: f64 = (noise_metadata_schedule_474_e4590 / noise_variable_37);
            let noise_metadata_schedule_474_e4594: f64 = (noise_metadata_schedule_474_e4592 * noise_variable_268);
            let noise_metadata_schedule_474_e4595: f64 = (1.0 + noise_metadata_schedule_474_e4594);
            let noise_metadata_schedule_474_e4596: f64 = (noise_metadata_schedule_474_e4595).sqrt();
            let noise_metadata_schedule_474_e4597: f64 = (1.0 + noise_metadata_schedule_474_e4596);
            let noise_metadata_schedule_474_e4598: f64 = (noise_metadata_schedule_474_e4585 / noise_metadata_schedule_474_e4597);
            noise_variable_164 = noise_metadata_schedule_474_e4598;
        }
        if matches!(source_index, 17 | 18) {
            let noise_metadata_schedule_475_e4601: f64 = if params.p8 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_555 = noise_metadata_schedule_475_e4601;
        }
        if matches!(source_index, 17) {
            let (noise_metadata_schedule_476_e4630,) = {
    if (noise_variable_555 != 0.0) {
        let noise_metadata_schedule_476_e4605: f64 = (params.p143 * 2.0);
        let noise_metadata_schedule_476_e4607: f64 = (noise_metadata_schedule_476_e4605 * noise_variable_104);
        let noise_metadata_schedule_476_e4610: f64 = (noise_variable_265 - noise_variable_256);
        let noise_metadata_schedule_476_e4611: f64 = (noise_metadata_schedule_476_e4607 * noise_metadata_schedule_476_e4610);
        let noise_metadata_schedule_476_e4617: f64 = (noise_variable_104 / noise_variable_106);
        let noise_metadata_schedule_476_e4618: f64 = (4.0 * noise_metadata_schedule_476_e4617);
        let noise_metadata_schedule_476_e4622: f64 = (params.p144 * noise_variable_256);
        let noise_metadata_schedule_476_e4623: f64 = (noise_variable_265 + noise_metadata_schedule_476_e4622);
        let noise_metadata_schedule_476_e4624: f64 = (noise_metadata_schedule_476_e4618 * noise_metadata_schedule_476_e4623);
        let noise_metadata_schedule_476_e4625: f64 = (1.0 + noise_metadata_schedule_476_e4624);
        let noise_metadata_schedule_476_e4626: f64 = (noise_metadata_schedule_476_e4625).sqrt();
        let noise_metadata_schedule_476_e4627: f64 = (1.0 + noise_metadata_schedule_476_e4626);
        let noise_metadata_schedule_476_e4628: f64 = (noise_metadata_schedule_476_e4611 / noise_metadata_schedule_476_e4627);
        (noise_metadata_schedule_476_e4628,)
    } else {
        (noise_variable_182,)
    }
};
            noise_variable_182 = noise_metadata_schedule_476_e4630;
        }
        if matches!(source_index, 18) {
            let (noise_metadata_schedule_477_e4661,) = {
    if (noise_variable_555 != 0.0) {
        let noise_metadata_schedule_477_e4634: f64 = (1.0 - params.p143);
        let noise_metadata_schedule_477_e4636: f64 = (noise_metadata_schedule_477_e4634 * 2.0);
        let noise_metadata_schedule_477_e4638: f64 = (noise_metadata_schedule_477_e4636 * noise_variable_104);
        let noise_metadata_schedule_477_e4641: f64 = (noise_variable_268 - noise_variable_258);
        let noise_metadata_schedule_477_e4642: f64 = (noise_metadata_schedule_477_e4638 * noise_metadata_schedule_477_e4641);
        let noise_metadata_schedule_477_e4648: f64 = (noise_variable_104 / noise_variable_106);
        let noise_metadata_schedule_477_e4649: f64 = (4.0 * noise_metadata_schedule_477_e4648);
        let noise_metadata_schedule_477_e4653: f64 = (params.p144 * noise_variable_258);
        let noise_metadata_schedule_477_e4654: f64 = (noise_variable_268 + noise_metadata_schedule_477_e4653);
        let noise_metadata_schedule_477_e4655: f64 = (noise_metadata_schedule_477_e4649 * noise_metadata_schedule_477_e4654);
        let noise_metadata_schedule_477_e4656: f64 = (1.0 + noise_metadata_schedule_477_e4655);
        let noise_metadata_schedule_477_e4657: f64 = (noise_metadata_schedule_477_e4656).sqrt();
        let noise_metadata_schedule_477_e4658: f64 = (1.0 + noise_metadata_schedule_477_e4657);
        let noise_metadata_schedule_477_e4659: f64 = (noise_metadata_schedule_477_e4642 / noise_metadata_schedule_477_e4658);
        (noise_metadata_schedule_477_e4659,)
    } else {
        (noise_variable_179,)
    }
};
            noise_variable_179 = noise_metadata_schedule_477_e4661;
        }
        if matches!(source_index, 17) {
            let (noise_metadata_schedule_478_e4687,) = {
    if (noise_variable_555 == 0.0) {
        let noise_metadata_schedule_478_e4666: f64 = (params.p143 * 2.0);
        let noise_metadata_schedule_478_e4668: f64 = (noise_metadata_schedule_478_e4666 * noise_variable_104);
        let noise_metadata_schedule_478_e4671: f64 = (noise_variable_265 - 1.0);
        let noise_metadata_schedule_478_e4672: f64 = (noise_metadata_schedule_478_e4668 * noise_metadata_schedule_478_e4671);
        let noise_metadata_schedule_478_e4678: f64 = (noise_variable_104 / noise_variable_106);
        let noise_metadata_schedule_478_e4679: f64 = (4.0 * noise_metadata_schedule_478_e4678);
        let noise_metadata_schedule_478_e4681: f64 = (noise_metadata_schedule_478_e4679 * noise_variable_265);
        let noise_metadata_schedule_478_e4682: f64 = (1.0 + noise_metadata_schedule_478_e4681);
        let noise_metadata_schedule_478_e4683: f64 = (noise_metadata_schedule_478_e4682).sqrt();
        let noise_metadata_schedule_478_e4684: f64 = (1.0 + noise_metadata_schedule_478_e4683);
        let noise_metadata_schedule_478_e4685: f64 = (noise_metadata_schedule_478_e4672 / noise_metadata_schedule_478_e4684);
        (noise_metadata_schedule_478_e4685,)
    } else {
        (noise_variable_182,)
    }
};
            noise_variable_182 = noise_metadata_schedule_478_e4687;
        }
        if matches!(source_index, 18) {
            let (noise_metadata_schedule_479_e4715,) = {
    if (noise_variable_555 == 0.0) {
        let noise_metadata_schedule_479_e4692: f64 = (1.0 - params.p143);
        let noise_metadata_schedule_479_e4694: f64 = (noise_metadata_schedule_479_e4692 * 2.0);
        let noise_metadata_schedule_479_e4696: f64 = (noise_metadata_schedule_479_e4694 * noise_variable_104);
        let noise_metadata_schedule_479_e4699: f64 = (noise_variable_268 - 1.0);
        let noise_metadata_schedule_479_e4700: f64 = (noise_metadata_schedule_479_e4696 * noise_metadata_schedule_479_e4699);
        let noise_metadata_schedule_479_e4706: f64 = (noise_variable_104 / noise_variable_106);
        let noise_metadata_schedule_479_e4707: f64 = (4.0 * noise_metadata_schedule_479_e4706);
        let noise_metadata_schedule_479_e4709: f64 = (noise_metadata_schedule_479_e4707 * noise_variable_268);
        let noise_metadata_schedule_479_e4710: f64 = (1.0 + noise_metadata_schedule_479_e4709);
        let noise_metadata_schedule_479_e4711: f64 = (noise_metadata_schedule_479_e4710).sqrt();
        let noise_metadata_schedule_479_e4712: f64 = (1.0 + noise_metadata_schedule_479_e4711);
        let noise_metadata_schedule_479_e4713: f64 = (noise_metadata_schedule_479_e4700 / noise_metadata_schedule_479_e4712);
        (noise_metadata_schedule_479_e4713,)
    } else {
        (noise_variable_179,)
    }
};
            noise_variable_179 = noise_metadata_schedule_479_e4715;
        }
        if matches!(source_index, 19) {
            noise_variable_180 = 0.0;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 18 | 19) {
            let noise_metadata_schedule_482_e4749: f64 = if ((params.p5 > 0.0) && (params.p33 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_556 = noise_metadata_schedule_482_e4749;
        }
        if matches!(source_index, 11 | 12) {
            let (noise_metadata_schedule_483_e4755,) = {
    if (noise_variable_556 != 0.0) {
        let noise_metadata_schedule_483_e4753: f64 = (noise_variable_164 * noise_variable_157);
        (noise_metadata_schedule_483_e4753,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_483_e4755;
        }
        if matches!(source_index, 18) {
            let (noise_metadata_schedule_484_e4761,) = {
    if (noise_variable_556 != 0.0) {
        let noise_metadata_schedule_484_e4759: f64 = (noise_variable_179 * noise_variable_157);
        (noise_metadata_schedule_484_e4759,)
    } else {
        (noise_variable_179,)
    }
};
            noise_variable_179 = noise_metadata_schedule_484_e4761;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_485_e4786,) = {
    if (noise_variable_556 != 0.0) {
        let noise_metadata_schedule_485_e4765: f64 = (params.p33 * 2.0);
        let noise_metadata_schedule_485_e4767: f64 = (noise_metadata_schedule_485_e4765 * noise_variable_43);
        let noise_metadata_schedule_485_e4770: f64 = (noise_variable_269 - 1.0);
        let noise_metadata_schedule_485_e4771: f64 = (noise_metadata_schedule_485_e4767 * noise_metadata_schedule_485_e4770);
        let noise_metadata_schedule_485_e4776: f64 = (4.0 * noise_variable_43);
        let noise_metadata_schedule_485_e4778: f64 = (noise_metadata_schedule_485_e4776 / noise_variable_37);
        let noise_metadata_schedule_485_e4780: f64 = (noise_metadata_schedule_485_e4778 * noise_variable_269);
        let noise_metadata_schedule_485_e4781: f64 = (1.0 + noise_metadata_schedule_485_e4780);
        let noise_metadata_schedule_485_e4782: f64 = (noise_metadata_schedule_485_e4781).sqrt();
        let noise_metadata_schedule_485_e4783: f64 = (1.0 + noise_metadata_schedule_485_e4782);
        let noise_metadata_schedule_485_e4784: f64 = (noise_metadata_schedule_485_e4771 / noise_metadata_schedule_485_e4783);
        (noise_metadata_schedule_485_e4784,)
    } else {
        (noise_variable_171,)
    }
};
            noise_variable_171 = noise_metadata_schedule_485_e4786;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let noise_metadata_schedule_486_e4789: f64 = if params.p8 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_557 = noise_metadata_schedule_486_e4789;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_487_e4824,) = {
    if ((noise_variable_556 != 0.0) && (noise_variable_557 != 0.0)) {
        let noise_metadata_schedule_487_e4795: f64 = (1.0 - params.p143);
        let noise_metadata_schedule_487_e4797: f64 = (noise_metadata_schedule_487_e4795 * params.p33);
        let noise_metadata_schedule_487_e4799: f64 = (noise_metadata_schedule_487_e4797 * 2.0);
        let noise_metadata_schedule_487_e4801: f64 = (noise_metadata_schedule_487_e4799 * noise_variable_104);
        let noise_metadata_schedule_487_e4804: f64 = (noise_variable_269 - noise_variable_257);
        let noise_metadata_schedule_487_e4805: f64 = (noise_metadata_schedule_487_e4801 * noise_metadata_schedule_487_e4804);
        let noise_metadata_schedule_487_e4810: f64 = (4.0 * noise_variable_104);
        let noise_metadata_schedule_487_e4812: f64 = (noise_metadata_schedule_487_e4810 / noise_variable_106);
        let noise_metadata_schedule_487_e4816: f64 = (params.p144 * noise_variable_257);
        let noise_metadata_schedule_487_e4817: f64 = (noise_variable_269 + noise_metadata_schedule_487_e4816);
        let noise_metadata_schedule_487_e4818: f64 = (noise_metadata_schedule_487_e4812 * noise_metadata_schedule_487_e4817);
        let noise_metadata_schedule_487_e4819: f64 = (1.0 + noise_metadata_schedule_487_e4818);
        let noise_metadata_schedule_487_e4820: f64 = (noise_metadata_schedule_487_e4819).sqrt();
        let noise_metadata_schedule_487_e4821: f64 = (1.0 + noise_metadata_schedule_487_e4820);
        let noise_metadata_schedule_487_e4822: f64 = (noise_metadata_schedule_487_e4805 / noise_metadata_schedule_487_e4821);
        (noise_metadata_schedule_487_e4822,)
    } else {
        (noise_variable_172,)
    }
};
            noise_variable_172 = noise_metadata_schedule_487_e4824;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_488_e4856,) = {
    if ((noise_variable_556 != 0.0) && (noise_variable_557 == 0.0)) {
        let noise_metadata_schedule_488_e4831: f64 = (1.0 - params.p143);
        let noise_metadata_schedule_488_e4833: f64 = (noise_metadata_schedule_488_e4831 * params.p33);
        let noise_metadata_schedule_488_e4835: f64 = (noise_metadata_schedule_488_e4833 * 2.0);
        let noise_metadata_schedule_488_e4837: f64 = (noise_metadata_schedule_488_e4835 * noise_variable_104);
        let noise_metadata_schedule_488_e4840: f64 = (noise_variable_269 - 1.0);
        let noise_metadata_schedule_488_e4841: f64 = (noise_metadata_schedule_488_e4837 * noise_metadata_schedule_488_e4840);
        let noise_metadata_schedule_488_e4846: f64 = (4.0 * noise_variable_104);
        let noise_metadata_schedule_488_e4848: f64 = (noise_metadata_schedule_488_e4846 / noise_variable_106);
        let noise_metadata_schedule_488_e4850: f64 = (noise_metadata_schedule_488_e4848 * noise_variable_269);
        let noise_metadata_schedule_488_e4851: f64 = (1.0 + noise_metadata_schedule_488_e4850);
        let noise_metadata_schedule_488_e4852: f64 = (noise_metadata_schedule_488_e4851).sqrt();
        let noise_metadata_schedule_488_e4853: f64 = (1.0 + noise_metadata_schedule_488_e4852);
        let noise_metadata_schedule_488_e4854: f64 = (noise_metadata_schedule_488_e4841 / noise_metadata_schedule_488_e4853);
        (noise_metadata_schedule_488_e4854,)
    } else {
        (noise_variable_172,)
    }
};
            noise_variable_172 = noise_metadata_schedule_488_e4856;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let noise_metadata_schedule_489_e4859: f64 = if params.p5 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_558 = noise_metadata_schedule_489_e4859;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_490_e4871,) = {
    if ((noise_variable_556 != 0.0) && (noise_variable_558 != 0.0)) {
        let noise_metadata_schedule_490_e4866: f64 = (noise_variable_43 + noise_variable_104);
        let noise_metadata_schedule_490_e4867: f64 = (params.p33 * noise_metadata_schedule_490_e4866);
        let noise_metadata_schedule_490_e4869: f64 = (noise_metadata_schedule_490_e4867 * noise_variable_32);
        (noise_metadata_schedule_490_e4869,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_490_e4871;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_491_e4884,) = {
    if ((noise_variable_556 != 0.0) && (noise_variable_558 != 0.0)) {
        let noise_metadata_schedule_491_e4879: f64 = (noise_variable_291 * noise_variable_8);
        let noise_metadata_schedule_491_e4880: f64 = (noise_metadata_schedule_491_e4879).ln();
        let noise_metadata_schedule_491_e4881: f64 = (2.0 - noise_metadata_schedule_491_e4880);
        let noise_metadata_schedule_491_e4882: f64 = (noise_variable_6 * noise_metadata_schedule_491_e4881);
        (noise_metadata_schedule_491_e4882,)
    } else {
        (noise_variable_173,)
    }
};
            noise_variable_173 = noise_metadata_schedule_491_e4884;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_492_e4892,) = {
    if ((noise_variable_556 != 0.0) && (noise_variable_558 != 0.0)) {
        let noise_metadata_schedule_492_e4890: f64 = (noise_variable_261 - noise_variable_173);
        (noise_metadata_schedule_492_e4890,)
    } else {
        (noise_variable_284,)
    }
};
            noise_variable_284 = noise_metadata_schedule_492_e4892;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_493_e4900,) = {
    if ((noise_variable_556 != 0.0) && (noise_variable_558 != 0.0)) {
        let noise_metadata_schedule_493_e4898: f64 = (0.11 * 0.11);
        (noise_metadata_schedule_493_e4898,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_493_e4900;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 19) {
            let (noise_metadata_schedule_494_e4908,) = {
    if ((noise_variable_556 != 0.0) && (noise_variable_558 != 0.0)) {
        let noise_metadata_schedule_494_e4906: f64 = (noise_variable_284 * noise_variable_284);
        (noise_metadata_schedule_494_e4906,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_494_e4908;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let noise_metadata_schedule_495_e4911: f64 = if noise_variable_284 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_559 = noise_metadata_schedule_495_e4911;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_496_e4928,) = {
    if (((noise_variable_556 != 0.0) && (noise_variable_558 != 0.0)) && (noise_variable_559 != 0.0)) {
        let noise_metadata_schedule_496_e4919: f64 = (0.5 * noise_variable_281);
        let noise_metadata_schedule_496_e4922: f64 = (noise_variable_282 + noise_variable_281);
        let noise_metadata_schedule_496_e4923: f64 = (noise_metadata_schedule_496_e4922).sqrt();
        let noise_metadata_schedule_496_e4925: f64 = (noise_metadata_schedule_496_e4923 - noise_variable_284);
        let noise_metadata_schedule_496_e4926: f64 = (noise_metadata_schedule_496_e4919 / noise_metadata_schedule_496_e4925);
        (noise_metadata_schedule_496_e4926,)
    } else {
        (noise_variable_174,)
    }
};
            noise_variable_174 = noise_metadata_schedule_496_e4928;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_497_e4944,) = {
    if (((noise_variable_556 != 0.0) && (noise_variable_558 != 0.0)) && (noise_variable_559 == 0.0)) {
        let noise_metadata_schedule_497_e4938: f64 = (noise_variable_282 + noise_variable_281);
        let noise_metadata_schedule_497_e4939: f64 = (noise_metadata_schedule_497_e4938).sqrt();
        let noise_metadata_schedule_497_e4941: f64 = (noise_metadata_schedule_497_e4939 + noise_variable_284);
        let noise_metadata_schedule_497_e4942: f64 = (0.5 * noise_metadata_schedule_497_e4941);
        (noise_metadata_schedule_497_e4942,)
    } else {
        (noise_variable_174,)
    }
};
            noise_variable_174 = noise_metadata_schedule_497_e4944;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_498_e4960,) = {
    if ((noise_variable_556 != 0.0) && (noise_variable_558 != 0.0)) {
        let noise_metadata_schedule_498_e4952: f64 = (noise_variable_171 + noise_variable_172);
        let noise_metadata_schedule_498_e4954: f64 = (noise_metadata_schedule_498_e4952 * noise_variable_32);
        let noise_metadata_schedule_498_e4955: f64 = (noise_variable_291 + noise_metadata_schedule_498_e4954);
        let noise_metadata_schedule_498_e4957: f64 = (noise_metadata_schedule_498_e4955 + noise_variable_174);
        let noise_metadata_schedule_498_e4958: f64 = (noise_variable_174 / noise_metadata_schedule_498_e4957);
        (noise_metadata_schedule_498_e4958,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_498_e4960;
        }
        if matches!(source_index, 13 | 14 | 19) {
            let (noise_metadata_schedule_502_e4988,) = {
    if ((noise_variable_556 != 0.0) && (noise_variable_558 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_502_e4988;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_503_e4994,) = {
    if (noise_variable_556 != 0.0) {
        let noise_metadata_schedule_503_e4992: f64 = (noise_variable_175 * noise_variable_171);
        (noise_metadata_schedule_503_e4992,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_503_e4994;
        }
        if matches!(source_index, 19) {
            let (noise_metadata_schedule_504_e5000,) = {
    if (noise_variable_556 != 0.0) {
        let noise_metadata_schedule_504_e4998: f64 = (noise_variable_175 * noise_variable_172);
        (noise_metadata_schedule_504_e4998,)
    } else {
        (noise_variable_180,)
    }
};
            noise_variable_180 = noise_metadata_schedule_504_e5000;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_505_e5003: f64 = if params.p84 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_560 = noise_metadata_schedule_505_e5003;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_506_e5009,) = {
    if (noise_variable_560 != 0.0) {
        let noise_metadata_schedule_506_e5007: f64 = (noise_variable_248 + noise_variable_244);
        (noise_metadata_schedule_506_e5007,)
    } else {
        (noise_variable_347,)
    }
};
            noise_variable_347 = noise_metadata_schedule_506_e5009;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_507_e5015,) = {
    if (noise_variable_560 != 0.0) {
        let noise_metadata_schedule_507_e5013: f64 = (1e-6 * 1e-6);
        (noise_metadata_schedule_507_e5013,)
    } else {
        (noise_variable_281,)
    }
};
            noise_variable_281 = noise_metadata_schedule_507_e5015;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_508_e5027,) = {
    if (noise_variable_560 != 0.0) {
        let noise_metadata_schedule_508_e5018: f64 = (-1.0);
        let noise_metadata_schedule_508_e5020: f64 = (noise_metadata_schedule_508_e5018 * noise_variable_347);
        let noise_metadata_schedule_508_e5022: f64 = (-1.0);
        let noise_metadata_schedule_508_e5023: f64 = (noise_metadata_schedule_508_e5020 * noise_metadata_schedule_508_e5022);
        let noise_metadata_schedule_508_e5025: f64 = (noise_metadata_schedule_508_e5023 * noise_variable_347);
        (noise_metadata_schedule_508_e5025,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_508_e5027;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_509_e5029: f64 = (-1.0);
            let noise_metadata_schedule_509_e5031: f64 = (noise_metadata_schedule_509_e5029 * noise_variable_347);
            let noise_metadata_schedule_509_e5033: f64 = if noise_metadata_schedule_509_e5031 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_561 = noise_metadata_schedule_509_e5033;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_510_e5051,) = {
    if ((noise_variable_560 != 0.0) && (noise_variable_561 != 0.0)) {
        let noise_metadata_schedule_510_e5039: f64 = (0.5 * noise_variable_281);
        let noise_metadata_schedule_510_e5042: f64 = (noise_variable_282 + noise_variable_281);
        let noise_metadata_schedule_510_e5043: f64 = (noise_metadata_schedule_510_e5042).sqrt();
        let noise_metadata_schedule_510_e5045: f64 = (-1.0);
        let noise_metadata_schedule_510_e5047: f64 = (noise_metadata_schedule_510_e5045 * noise_variable_347);
        let noise_metadata_schedule_510_e5048: f64 = (noise_metadata_schedule_510_e5043 - noise_metadata_schedule_510_e5047);
        let noise_metadata_schedule_510_e5049: f64 = (noise_metadata_schedule_510_e5039 / noise_metadata_schedule_510_e5048);
        (noise_metadata_schedule_510_e5049,)
    } else {
        (noise_variable_348,)
    }
};
            noise_variable_348 = noise_metadata_schedule_510_e5051;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_511_e5068,) = {
    if ((noise_variable_560 != 0.0) && (noise_variable_561 == 0.0)) {
        let noise_metadata_schedule_511_e5059: f64 = (noise_variable_282 + noise_variable_281);
        let noise_metadata_schedule_511_e5060: f64 = (noise_metadata_schedule_511_e5059).sqrt();
        let noise_metadata_schedule_511_e5062: f64 = (-1.0);
        let noise_metadata_schedule_511_e5064: f64 = (noise_metadata_schedule_511_e5062 * noise_variable_347);
        let noise_metadata_schedule_511_e5065: f64 = (noise_metadata_schedule_511_e5060 + noise_metadata_schedule_511_e5064);
        let noise_metadata_schedule_511_e5066: f64 = (0.5 * noise_metadata_schedule_511_e5065);
        (noise_metadata_schedule_511_e5066,)
    } else {
        (noise_variable_348,)
    }
};
            noise_variable_348 = noise_metadata_schedule_511_e5068;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_512_e5078,) = {
    if (noise_variable_560 != 0.0) {
        let noise_metadata_schedule_512_e5074: f64 = (noise_variable_343).powf(params.p82);
        let noise_metadata_schedule_512_e5075: f64 = (1.0 - noise_metadata_schedule_512_e5074);
        let noise_metadata_schedule_512_e5076: f64 = (1.0 / noise_metadata_schedule_512_e5075);
        (noise_metadata_schedule_512_e5076,)
    } else {
        (noise_variable_349,)
    }
};
            noise_variable_349 = noise_metadata_schedule_512_e5078;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_513_e5084,) = {
    if (noise_variable_560 != 0.0) {
        let noise_metadata_schedule_513_e5082: f64 = (noise_variable_343 * params.p81);
        (noise_metadata_schedule_513_e5082,)
    } else {
        (noise_variable_344,)
    }
};
            noise_variable_344 = noise_metadata_schedule_513_e5084;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_514_e5100,) = {
    if (noise_variable_560 != 0.0) {
        let noise_metadata_schedule_514_e5088: f64 = (noise_variable_349 * noise_variable_349);
        let noise_metadata_schedule_514_e5092: f64 = (params.p82 - 1.0);
        let noise_metadata_schedule_514_e5093: f64 = (noise_variable_343).powf(noise_metadata_schedule_514_e5092);
        let noise_metadata_schedule_514_e5094: f64 = (noise_metadata_schedule_514_e5088 * noise_metadata_schedule_514_e5093);
        let noise_metadata_schedule_514_e5096: f64 = (noise_metadata_schedule_514_e5094 * params.p82);
        let noise_metadata_schedule_514_e5098: f64 = (noise_metadata_schedule_514_e5096 / params.p81);
        (noise_metadata_schedule_514_e5098,)
    } else {
        (noise_variable_346,)
    }
};
            noise_variable_346 = noise_metadata_schedule_514_e5100;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let noise_metadata_schedule_515_e5103: f64 = if noise_variable_348 < noise_variable_344 { 1.0 } else { 0.0 };
            noise_variable_562 = noise_metadata_schedule_515_e5103;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_516_e5117,) = {
    if ((noise_variable_560 != 0.0) && (noise_variable_562 != 0.0)) {
        let noise_metadata_schedule_516_e5111: f64 = (noise_variable_348 / params.p81);
        let noise_metadata_schedule_516_e5113: f64 = (noise_metadata_schedule_516_e5111).powf(params.p82);
        let noise_metadata_schedule_516_e5114: f64 = (1.0 - noise_metadata_schedule_516_e5113);
        let noise_metadata_schedule_516_e5115: f64 = (1.0 / noise_metadata_schedule_516_e5114);
        (noise_metadata_schedule_516_e5115,)
    } else {
        (noise_variable_345,)
    }
};
            noise_variable_345 = noise_metadata_schedule_516_e5117;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_517_e5130,) = {
    if ((noise_variable_560 != 0.0) && (noise_variable_562 == 0.0)) {
        let noise_metadata_schedule_517_e5125: f64 = (noise_variable_348 - noise_variable_344);
        let noise_metadata_schedule_517_e5127: f64 = (noise_metadata_schedule_517_e5125 * noise_variable_346);
        let noise_metadata_schedule_517_e5128: f64 = (noise_variable_349 + noise_metadata_schedule_517_e5127);
        (noise_metadata_schedule_517_e5128,)
    } else {
        (noise_variable_345,)
    }
};
            noise_variable_345 = noise_metadata_schedule_517_e5130;
        }
        if matches!(source_index, 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16) {
            let (noise_metadata_schedule_518_e5135,) = {
    if (noise_variable_560 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_345,)
    }
};
            noise_variable_345 = noise_metadata_schedule_518_e5135;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_519_e5138: f64 = (noise_variable_82 * noise_variable_345);
            noise_variable_82 = noise_metadata_schedule_519_e5138;
        }
        if matches!(source_index, 11 | 12) {
            let noise_metadata_schedule_520_e5141: f64 = (noise_variable_164 * noise_variable_345);
            noise_variable_164 = noise_metadata_schedule_520_e5141;
        }
        if matches!(source_index, 9 | 10) {
            let noise_metadata_schedule_521_e5144: f64 = (noise_variable_161 * noise_variable_345);
            noise_variable_161 = noise_metadata_schedule_521_e5144;
        }
        if matches!(source_index, 13 | 14) {
            let noise_metadata_schedule_522_e5147: f64 = (noise_variable_176 * noise_variable_345);
            noise_variable_176 = noise_metadata_schedule_522_e5147;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_523_e5151: f64 = (noise_variable_138 / noise_variable_41);
            let noise_metadata_schedule_523_e5152: f64 = (1.0 + noise_metadata_schedule_523_e5151);
            let noise_metadata_schedule_523_e5155: f64 = (noise_variable_145 / noise_variable_40);
            let noise_metadata_schedule_523_e5156: f64 = (noise_metadata_schedule_523_e5152 + noise_metadata_schedule_523_e5155);
            noise_variable_183 = noise_metadata_schedule_523_e5156;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_524_e5159: f64 = (0.1 * 0.1);
            noise_variable_281 = noise_metadata_schedule_524_e5159;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_525_e5162: f64 = (noise_variable_183 * noise_variable_183);
            noise_variable_282 = noise_metadata_schedule_525_e5162;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_526_e5165: f64 = if noise_variable_183 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_563 = noise_metadata_schedule_526_e5165;
        }
        if matches!(source_index, 1 | 5) {
            let (noise_metadata_schedule_527_e5178,) = {
    if (noise_variable_563 != 0.0) {
        let noise_metadata_schedule_527_e5169: f64 = (0.5 * noise_variable_281);
        let noise_metadata_schedule_527_e5172: f64 = (noise_variable_282 + noise_variable_281);
        let noise_metadata_schedule_527_e5173: f64 = (noise_metadata_schedule_527_e5172).sqrt();
        let noise_metadata_schedule_527_e5175: f64 = (noise_metadata_schedule_527_e5173 - noise_variable_183);
        let noise_metadata_schedule_527_e5176: f64 = (noise_metadata_schedule_527_e5169 / noise_metadata_schedule_527_e5175);
        (noise_metadata_schedule_527_e5176,)
    } else {
        (noise_variable_184,)
    }
};
            noise_variable_184 = noise_metadata_schedule_527_e5178;
        }
        if matches!(source_index, 1 | 5) {
            let (noise_metadata_schedule_528_e5190,) = {
    if (noise_variable_563 == 0.0) {
        let noise_metadata_schedule_528_e5184: f64 = (noise_variable_282 + noise_variable_281);
        let noise_metadata_schedule_528_e5185: f64 = (noise_metadata_schedule_528_e5184).sqrt();
        let noise_metadata_schedule_528_e5187: f64 = (noise_metadata_schedule_528_e5185 + noise_variable_183);
        let noise_metadata_schedule_528_e5188: f64 = (0.5 * noise_metadata_schedule_528_e5187);
        (noise_metadata_schedule_528_e5188,)
    } else {
        (noise_variable_184,)
    }
};
            noise_variable_184 = noise_metadata_schedule_528_e5190;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_529_e5196: f64 = (noise_variable_149 + noise_variable_150);
            let noise_metadata_schedule_529_e5197: f64 = (0.5 * noise_metadata_schedule_529_e5196);
            let noise_metadata_schedule_529_e5198: f64 = (1.0 + noise_metadata_schedule_529_e5197);
            let noise_metadata_schedule_529_e5199: f64 = (noise_variable_184 * noise_metadata_schedule_529_e5198);
            noise_variable_185 = noise_metadata_schedule_529_e5199;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_530_e5202: f64 = (noise_variable_29 / noise_variable_185);
            noise_variable_187 = noise_metadata_schedule_530_e5202;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_531_e5205: f64 = if noise_variable_187 < noise_variable_340 { 1.0 } else { 0.0 };
            noise_variable_564 = noise_metadata_schedule_531_e5205;
        }
        if matches!(source_index, 1 | 5) {
            let (noise_metadata_schedule_532_e5209,) = {
    if (noise_variable_564 != 0.0) {
        (noise_variable_340,)
    } else {
        (noise_variable_187,)
    }
};
            noise_variable_187 = noise_metadata_schedule_532_e5209;
        }
        if matches!(source_index, 1 | 5) {
            let noise_metadata_schedule_533_e5212: f64 = (3.0 * noise_variable_187);
            noise_variable_186 = noise_metadata_schedule_533_e5212;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_535_e5226: f64 = if noise_variable_156 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_565 = noise_metadata_schedule_535_e5226;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_536_e5229: f64 = if params.p39 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_566 = noise_metadata_schedule_536_e5229;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_537_e5232: f64 = if noise_variable_244 < params.p44 { 1.0 } else { 0.0 };
            noise_variable_567 = noise_metadata_schedule_537_e5232;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_538_e5234: f64 = (-noise_variable_156);
            let noise_metadata_schedule_538_e5236: f64 = (noise_metadata_schedule_538_e5234 / params.p42);
            let noise_metadata_schedule_538_e5238: f64 = if noise_metadata_schedule_538_e5236 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_568 = noise_metadata_schedule_538_e5238;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_539_e5252,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 != 0.0)) && (noise_variable_567 != 0.0)) && (noise_variable_568 != 0.0)) {
        let noise_metadata_schedule_539_e5247: f64 = (-noise_variable_156);
        let noise_metadata_schedule_539_e5249: f64 = (noise_metadata_schedule_539_e5247 / params.p42);
        let noise_metadata_schedule_539_e5250: f64 = (noise_metadata_schedule_539_e5249).exp();
        (noise_metadata_schedule_539_e5250,)
    } else {
        (noise_variable_332,)
    }
};
            noise_variable_332 = noise_metadata_schedule_539_e5252;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_540_e5264,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 != 0.0)) && (noise_variable_567 != 0.0)) && (noise_variable_568 == 0.0)) {
        let noise_metadata_schedule_540_e5262: f64 = (params.p147).exp();
        (noise_metadata_schedule_540_e5262,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_540_e5264;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_541_e5284,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 != 0.0)) && (noise_variable_567 != 0.0)) && (noise_variable_568 == 0.0)) {
        let noise_metadata_schedule_541_e5276: f64 = (-noise_variable_156);
        let noise_metadata_schedule_541_e5278: f64 = (noise_metadata_schedule_541_e5276 / params.p42);
        let noise_metadata_schedule_541_e5280: f64 = (noise_metadata_schedule_541_e5278 - params.p147);
        let noise_metadata_schedule_541_e5281: f64 = (1.0 + noise_metadata_schedule_541_e5280);
        let noise_metadata_schedule_541_e5282: f64 = (noise_variable_295 * noise_metadata_schedule_541_e5281);
        (noise_metadata_schedule_541_e5282,)
    } else {
        (noise_variable_332,)
    }
};
            noise_variable_332 = noise_metadata_schedule_541_e5284;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_542_e5296,) = {
    if (((noise_variable_565 != 0.0) && (noise_variable_566 != 0.0)) && (noise_variable_567 != 0.0)) {
        let noise_metadata_schedule_542_e5292: f64 = (params.p44 - noise_variable_244);
        let noise_metadata_schedule_542_e5294: f64 = (noise_metadata_schedule_542_e5292 * noise_variable_332);
        (noise_metadata_schedule_542_e5294,)
    } else {
        (noise_variable_333,)
    }
};
            noise_variable_333 = noise_metadata_schedule_542_e5296;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_543_e5298: f64 = (-noise_variable_334);
            let noise_metadata_schedule_543_e5301: f64 = (noise_variable_333).powf(params.p41);
            let noise_metadata_schedule_543_e5302: f64 = (noise_metadata_schedule_543_e5298 * noise_metadata_schedule_543_e5301);
            let noise_metadata_schedule_543_e5304: f64 = if noise_metadata_schedule_543_e5302 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_569 = noise_metadata_schedule_543_e5304;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_544_e5320,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 != 0.0)) && (noise_variable_567 != 0.0)) && (noise_variable_569 != 0.0)) {
        let noise_metadata_schedule_544_e5313: f64 = (-noise_variable_334);
        let noise_metadata_schedule_544_e5316: f64 = (noise_variable_333).powf(params.p41);
        let noise_metadata_schedule_544_e5317: f64 = (noise_metadata_schedule_544_e5313 * noise_metadata_schedule_544_e5316);
        let noise_metadata_schedule_544_e5318: f64 = (noise_metadata_schedule_544_e5317).exp();
        (noise_metadata_schedule_544_e5318,)
    } else {
        (noise_variable_337,)
    }
};
            noise_variable_337 = noise_metadata_schedule_544_e5320;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_545_e5332,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 != 0.0)) && (noise_variable_567 != 0.0)) && (noise_variable_569 == 0.0)) {
        let noise_metadata_schedule_545_e5330: f64 = (params.p147).exp();
        (noise_metadata_schedule_545_e5330,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_545_e5332;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_546_e5354,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 != 0.0)) && (noise_variable_567 != 0.0)) && (noise_variable_569 == 0.0)) {
        let noise_metadata_schedule_546_e5344: f64 = (-noise_variable_334);
        let noise_metadata_schedule_546_e5347: f64 = (noise_variable_333).powf(params.p41);
        let noise_metadata_schedule_546_e5348: f64 = (noise_metadata_schedule_546_e5344 * noise_metadata_schedule_546_e5347);
        let noise_metadata_schedule_546_e5350: f64 = (noise_metadata_schedule_546_e5348 - params.p147);
        let noise_metadata_schedule_546_e5351: f64 = (1.0 + noise_metadata_schedule_546_e5350);
        let noise_metadata_schedule_546_e5352: f64 = (noise_variable_295 * noise_metadata_schedule_546_e5351);
        (noise_metadata_schedule_546_e5352,)
    } else {
        (noise_variable_337,)
    }
};
            noise_variable_337 = noise_metadata_schedule_546_e5354;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_547_e5368,) = {
    if (((noise_variable_565 != 0.0) && (noise_variable_566 != 0.0)) && (noise_variable_567 != 0.0)) {
        let noise_metadata_schedule_547_e5362: f64 = (params.p40 / noise_variable_334);
        let noise_metadata_schedule_547_e5364: f64 = (noise_metadata_schedule_547_e5362 * noise_variable_333);
        let noise_metadata_schedule_547_e5366: f64 = (noise_metadata_schedule_547_e5364 * noise_variable_337);
        (noise_metadata_schedule_547_e5366,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_547_e5368;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_548_e5371: f64 = if params.p39 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_570 = noise_metadata_schedule_548_e5371;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_549_e5374: f64 = if noise_variable_244 < noise_variable_16 { 1.0 } else { 0.0 };
            noise_variable_571 = noise_metadata_schedule_549_e5374;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_550_e5391,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) {
        let noise_metadata_schedule_550_e5385: f64 = (2.0 * params.p46);
        let noise_metadata_schedule_550_e5388: f64 = (params.p45 * params.p45);
        let noise_metadata_schedule_550_e5389: f64 = (noise_metadata_schedule_550_e5385 / noise_metadata_schedule_550_e5388);
        (noise_metadata_schedule_550_e5389,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_550_e5391;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_551_e5406,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) {
        let noise_metadata_schedule_551_e5402: f64 = (noise_variable_16 - noise_variable_244);
        let noise_metadata_schedule_551_e5404: f64 = (noise_metadata_schedule_551_e5402 / noise_variable_210);
        (noise_metadata_schedule_551_e5404,)
    } else {
        (noise_variable_280,)
    }
};
            noise_variable_280 = noise_metadata_schedule_551_e5406;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_552_e5422,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) {
        let noise_metadata_schedule_552_e5417: f64 = (2.0 * noise_variable_280);
        let noise_metadata_schedule_552_e5419: f64 = (noise_metadata_schedule_552_e5417 / noise_variable_196);
        let noise_metadata_schedule_552_e5420: f64 = (noise_metadata_schedule_552_e5419).sqrt();
        (noise_metadata_schedule_552_e5420,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_552_e5422;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_553_e5425: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_572 = noise_metadata_schedule_553_e5425;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_554_e5438,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) && (noise_variable_572 != 0.0)) {
        (params.p45,)
    } else {
        (noise_variable_198,)
    }
};
            noise_variable_198 = noise_metadata_schedule_554_e5438;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_555_e5456,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) && (noise_variable_572 == 0.0)) {
        let noise_metadata_schedule_555_e5453: f64 = (0.5 * noise_variable_122);
        let noise_metadata_schedule_555_e5454: f64 = (1.0 - noise_metadata_schedule_555_e5453);
        (noise_metadata_schedule_555_e5454,)
    } else {
        (noise_variable_123,)
    }
};
            noise_variable_123 = noise_metadata_schedule_555_e5456;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_556_e5474,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) && (noise_variable_572 == 0.0)) {
        let noise_metadata_schedule_556_e5470: f64 = (params.p45 * noise_variable_123);
        let noise_metadata_schedule_556_e5472: f64 = (noise_metadata_schedule_556_e5470 * noise_variable_123);
        (noise_metadata_schedule_556_e5472,)
    } else {
        (noise_variable_198,)
    }
};
            noise_variable_198 = noise_metadata_schedule_556_e5474;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_557_e5496,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) {
        let noise_metadata_schedule_557_e5485: f64 = (noise_variable_197 * noise_variable_198);
        let noise_metadata_schedule_557_e5488: f64 = (noise_variable_197 * noise_variable_197);
        let noise_metadata_schedule_557_e5491: f64 = (noise_variable_198 * noise_variable_198);
        let noise_metadata_schedule_557_e5492: f64 = (noise_metadata_schedule_557_e5488 + noise_metadata_schedule_557_e5491);
        let noise_metadata_schedule_557_e5493: f64 = (noise_metadata_schedule_557_e5492).sqrt();
        let noise_metadata_schedule_557_e5494: f64 = (noise_metadata_schedule_557_e5485 / noise_metadata_schedule_557_e5493);
        (noise_metadata_schedule_557_e5494,)
    } else {
        (noise_variable_199,)
    }
};
            noise_variable_199 = noise_metadata_schedule_557_e5496;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_558_e5511,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) {
        let noise_metadata_schedule_558_e5507: f64 = (noise_variable_16 - noise_variable_244);
        let noise_metadata_schedule_558_e5509: f64 = (noise_metadata_schedule_558_e5507 / noise_variable_199);
        (noise_metadata_schedule_558_e5509,)
    } else {
        (noise_variable_200,)
    }
};
            noise_variable_200 = noise_metadata_schedule_558_e5511;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_559_e5530,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) {
        let noise_metadata_schedule_559_e5523: f64 = (0.5 * noise_variable_199);
        let noise_metadata_schedule_559_e5525: f64 = (noise_metadata_schedule_559_e5523 * noise_variable_196);
        let noise_metadata_schedule_559_e5527: f64 = (noise_metadata_schedule_559_e5525 * noise_variable_210);
        let noise_metadata_schedule_559_e5528: f64 = (noise_variable_200 + noise_metadata_schedule_559_e5527);
        (noise_metadata_schedule_559_e5528,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_559_e5530;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_560_e5533: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_573 = noise_metadata_schedule_560_e5533;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_561_e5546,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) && (noise_variable_573 != 0.0)) {
        (noise_variable_201,)
    } else {
        (noise_variable_202,)
    }
};
            noise_variable_202 = noise_metadata_schedule_561_e5546;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_562_e5570,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) && (noise_variable_573 == 0.0)) {
        let noise_metadata_schedule_562_e5561: f64 = (2.0 * params.p47);
        let noise_metadata_schedule_562_e5565: f64 = (2.0 * noise_variable_122);
        let noise_metadata_schedule_562_e5566: f64 = (1.0 + noise_metadata_schedule_562_e5565);
        let noise_metadata_schedule_562_e5567: f64 = (noise_metadata_schedule_562_e5561 * noise_metadata_schedule_562_e5566);
        let noise_metadata_schedule_562_e5568: f64 = (1.0 + noise_metadata_schedule_562_e5567);
        (noise_metadata_schedule_562_e5568,)
    } else {
        (noise_variable_203,)
    }
};
            noise_variable_203 = noise_metadata_schedule_562_e5570;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_563_e5592,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) && (noise_variable_573 == 0.0)) {
        let noise_metadata_schedule_563_e5584: f64 = (1.0 + params.p47);
        let noise_metadata_schedule_563_e5588: f64 = (2.0 * params.p47);
        let noise_metadata_schedule_563_e5589: f64 = (1.0 + noise_metadata_schedule_563_e5588);
        let noise_metadata_schedule_563_e5590: f64 = (noise_metadata_schedule_563_e5584 / noise_metadata_schedule_563_e5589);
        (noise_metadata_schedule_563_e5590,)
    } else {
        (noise_variable_204,)
    }
};
            noise_variable_204 = noise_metadata_schedule_563_e5592;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_564_e5620,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) && (noise_variable_573 == 0.0)) {
        let noise_metadata_schedule_564_e5607: f64 = (0.5 * noise_variable_199);
        let noise_metadata_schedule_564_e5609: f64 = (noise_metadata_schedule_564_e5607 * noise_variable_196);
        let noise_metadata_schedule_564_e5614: f64 = (params.p62 * noise_variable_203);
        let noise_metadata_schedule_564_e5615: f64 = (noise_variable_156 / noise_metadata_schedule_564_e5614);
        let noise_metadata_schedule_564_e5616: f64 = (noise_variable_204 - noise_metadata_schedule_564_e5615);
        let noise_metadata_schedule_564_e5617: f64 = (noise_metadata_schedule_564_e5609 * noise_metadata_schedule_564_e5616);
        let noise_metadata_schedule_564_e5618: f64 = (noise_variable_200 - noise_metadata_schedule_564_e5617);
        (noise_metadata_schedule_564_e5618,)
    } else {
        (noise_variable_205,)
    }
};
            noise_variable_205 = noise_metadata_schedule_564_e5620;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_565_e5650,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) && (noise_variable_573 == 0.0)) {
        let noise_metadata_schedule_565_e5634: f64 = (noise_variable_205 - noise_variable_201);
        let noise_metadata_schedule_565_e5637: f64 = (noise_variable_205 - noise_variable_201);
        let noise_metadata_schedule_565_e5638: f64 = (noise_metadata_schedule_565_e5634 * noise_metadata_schedule_565_e5637);
        let noise_metadata_schedule_565_e5641: f64 = (0.1 * noise_variable_200);
        let noise_metadata_schedule_565_e5643: f64 = (noise_metadata_schedule_565_e5641 * noise_variable_200);
        let noise_metadata_schedule_565_e5645: f64 = (noise_metadata_schedule_565_e5643 * noise_variable_134);
        let noise_metadata_schedule_565_e5647: f64 = (noise_metadata_schedule_565_e5645 / params.p62);
        let noise_metadata_schedule_565_e5648: f64 = (noise_metadata_schedule_565_e5638 + noise_metadata_schedule_565_e5647);
        (noise_metadata_schedule_565_e5648,)
    } else {
        (noise_variable_280,)
    }
};
            noise_variable_280 = noise_metadata_schedule_565_e5650;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_566_e5671,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) && (noise_variable_573 == 0.0)) {
        let noise_metadata_schedule_566_e5665: f64 = (noise_variable_205 + noise_variable_201);
        let noise_metadata_schedule_566_e5667: f64 = (noise_variable_280).sqrt();
        let noise_metadata_schedule_566_e5668: f64 = (noise_metadata_schedule_566_e5665 + noise_metadata_schedule_566_e5667);
        let noise_metadata_schedule_566_e5669: f64 = (0.5 * noise_metadata_schedule_566_e5668);
        (noise_metadata_schedule_566_e5669,)
    } else {
        (noise_variable_202,)
    }
};
            noise_variable_202 = noise_metadata_schedule_566_e5671;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_567_e5686,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) {
        let noise_metadata_schedule_567_e5682: f64 = (noise_variable_202 - noise_variable_200);
        let noise_metadata_schedule_567_e5684: f64 = (noise_metadata_schedule_567_e5682 / noise_variable_202);
        (noise_metadata_schedule_567_e5684,)
    } else {
        (noise_variable_287,)
    }
};
            noise_variable_287 = noise_metadata_schedule_567_e5686;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_568_e5688: f64 = (noise_variable_287).abs();
            let noise_metadata_schedule_568_e5690: f64 = if noise_metadata_schedule_568_e5688 > 1e-7 { 1.0 } else { 0.0 };
            noise_variable_574 = noise_metadata_schedule_568_e5690;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_569_e5707,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) && (noise_variable_574 != 0.0)) {
        let noise_metadata_schedule_569_e5703: f64 = (0.5 * noise_variable_199);
        let noise_metadata_schedule_569_e5705: f64 = (noise_metadata_schedule_569_e5703 / noise_variable_287);
        (noise_metadata_schedule_569_e5705,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_569_e5707;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_570_e5744,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) && (noise_variable_574 != 0.0)) {
        let noise_metadata_schedule_570_e5720: f64 = (noise_variable_0 / noise_variable_98);
        let noise_metadata_schedule_570_e5722: f64 = (noise_metadata_schedule_570_e5720 * noise_variable_202);
        let noise_metadata_schedule_570_e5724: f64 = (noise_metadata_schedule_570_e5722 * noise_variable_206);
        let noise_metadata_schedule_570_e5726: f64 = (-noise_variable_98);
        let noise_metadata_schedule_570_e5728: f64 = (noise_metadata_schedule_570_e5726 / noise_variable_202);
        let noise_metadata_schedule_570_e5729: f64 = (noise_metadata_schedule_570_e5728).exp();
        let noise_metadata_schedule_570_e5731: f64 = (-noise_variable_98);
        let noise_metadata_schedule_570_e5733: f64 = (noise_metadata_schedule_570_e5731 / noise_variable_202);
        let noise_metadata_schedule_570_e5737: f64 = (noise_variable_198 / noise_variable_206);
        let noise_metadata_schedule_570_e5738: f64 = (1.0 + noise_metadata_schedule_570_e5737);
        let noise_metadata_schedule_570_e5739: f64 = (noise_metadata_schedule_570_e5733 * noise_metadata_schedule_570_e5738);
        let noise_metadata_schedule_570_e5740: f64 = (noise_metadata_schedule_570_e5739).exp();
        let noise_metadata_schedule_570_e5741: f64 = (noise_metadata_schedule_570_e5729 - noise_metadata_schedule_570_e5740);
        let noise_metadata_schedule_570_e5742: f64 = (noise_metadata_schedule_570_e5724 * noise_metadata_schedule_570_e5741);
        (noise_metadata_schedule_570_e5742,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_570_e5744;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_571_e5766,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 != 0.0)) && (noise_variable_571 != 0.0)) && (noise_variable_574 == 0.0)) {
        let noise_metadata_schedule_571_e5758: f64 = (noise_variable_0 * noise_variable_198);
        let noise_metadata_schedule_571_e5760: f64 = (-noise_variable_98);
        let noise_metadata_schedule_571_e5762: f64 = (noise_metadata_schedule_571_e5760 / noise_variable_202);
        let noise_metadata_schedule_571_e5763: f64 = (noise_metadata_schedule_571_e5762).exp();
        let noise_metadata_schedule_571_e5764: f64 = (noise_metadata_schedule_571_e5758 * noise_metadata_schedule_571_e5763);
        (noise_metadata_schedule_571_e5764,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_571_e5766;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_572_e5769: f64 = if params.p39 == 3.0 { 1.0 } else { 0.0 };
            noise_variable_575 = noise_metadata_schedule_572_e5769;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_573_e5772: f64 = if noise_variable_244 < params.p44 { 1.0 } else { 0.0 };
            noise_variable_576 = noise_metadata_schedule_573_e5772;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_574_e5800,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 == 0.0)) && (noise_variable_575 != 0.0)) && (noise_variable_576 != 0.0)) {
        let noise_metadata_schedule_574_e5786: f64 = (params.p44 - noise_variable_244);
        let noise_metadata_schedule_574_e5788: f64 = (noise_metadata_schedule_574_e5786).powf(params.p41);
        let noise_metadata_schedule_574_e5793: f64 = (params.p48 + noise_variable_156);
        let noise_metadata_schedule_574_e5794: f64 = (noise_variable_156 / noise_metadata_schedule_574_e5793);
        let noise_metadata_schedule_574_e5795: f64 = (1.0 - noise_metadata_schedule_574_e5794);
        let noise_metadata_schedule_574_e5797: f64 = (noise_metadata_schedule_574_e5795).powf(params.p49);
        let noise_metadata_schedule_574_e5798: f64 = (noise_metadata_schedule_574_e5788 * noise_metadata_schedule_574_e5797);
        (noise_metadata_schedule_574_e5798,)
    } else {
        (noise_variable_211,)
    }
};
            noise_variable_211 = noise_metadata_schedule_574_e5800;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_575_e5803: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_577 = noise_metadata_schedule_575_e5803;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_576_e5819,) = {
    if ((((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 == 0.0)) && (noise_variable_575 != 0.0)) && (noise_variable_576 != 0.0)) && (noise_variable_577 != 0.0)) {
        (noise_variable_211,)
    } else {
        (noise_variable_212,)
    }
};
            noise_variable_212 = noise_metadata_schedule_576_e5819;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_577_e5840,) = {
    if ((((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 == 0.0)) && (noise_variable_575 != 0.0)) && (noise_variable_576 != 0.0)) && (noise_variable_577 == 0.0)) {
        let noise_metadata_schedule_577_e5836: f64 = (noise_variable_156 - params.p52);
        let noise_metadata_schedule_577_e5838: f64 = (noise_metadata_schedule_577_e5836 / params.p48);
        (noise_metadata_schedule_577_e5838,)
    } else {
        (noise_variable_213,)
    }
};
            noise_variable_213 = noise_metadata_schedule_577_e5840;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_578_e5861,) = {
    if ((((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 == 0.0)) && (noise_variable_575 != 0.0)) && (noise_variable_576 != 0.0)) && (noise_variable_577 == 0.0)) {
        let noise_metadata_schedule_578_e5857: f64 = (noise_variable_213 - 1.0);
        let noise_metadata_schedule_578_e5859: f64 = (noise_metadata_schedule_578_e5857 / params.p51);
        (noise_metadata_schedule_578_e5859,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_578_e5861;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_579_e5864: f64 = if noise_variable_213 < 1.0 { 1.0 } else { 0.0 };
            noise_variable_578 = noise_metadata_schedule_579_e5864;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_580_e5891,) = {
    if (((((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 == 0.0)) && (noise_variable_575 != 0.0)) && (noise_variable_576 != 0.0)) && (noise_variable_577 == 0.0)) && (noise_variable_578 != 0.0)) {
        let noise_metadata_schedule_580_e5885: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_580_e5886: f64 = (1.0 + noise_metadata_schedule_580_e5885);
        let noise_metadata_schedule_580_e5887: f64 = (noise_metadata_schedule_580_e5886).ln();
        let noise_metadata_schedule_580_e5888: f64 = (params.p51 * noise_metadata_schedule_580_e5887);
        let noise_metadata_schedule_580_e5889: f64 = (1.0 + noise_metadata_schedule_580_e5888);
        (noise_metadata_schedule_580_e5889,)
    } else {
        (noise_variable_214,)
    }
};
            noise_variable_214 = noise_metadata_schedule_580_e5891;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_581_e5920,) = {
    if (((((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 == 0.0)) && (noise_variable_575 != 0.0)) && (noise_variable_576 != 0.0)) && (noise_variable_577 == 0.0)) && (noise_variable_578 == 0.0)) {
        let noise_metadata_schedule_581_e5913: f64 = (-noise_variable_279);
        let noise_metadata_schedule_581_e5914: f64 = (noise_metadata_schedule_581_e5913).exp();
        let noise_metadata_schedule_581_e5915: f64 = (1.0 + noise_metadata_schedule_581_e5914);
        let noise_metadata_schedule_581_e5916: f64 = (noise_metadata_schedule_581_e5915).ln();
        let noise_metadata_schedule_581_e5917: f64 = (params.p51 * noise_metadata_schedule_581_e5916);
        let noise_metadata_schedule_581_e5918: f64 = (noise_variable_213 + noise_metadata_schedule_581_e5917);
        (noise_metadata_schedule_581_e5918,)
    } else {
        (noise_variable_214,)
    }
};
            noise_variable_214 = noise_metadata_schedule_581_e5920;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_582_e5941,) = {
    if ((((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 == 0.0)) && (noise_variable_575 != 0.0)) && (noise_variable_576 != 0.0)) && (noise_variable_577 == 0.0)) {
        let noise_metadata_schedule_582_e5938: f64 = (noise_variable_214).powf(params.p50);
        let noise_metadata_schedule_582_e5939: f64 = (noise_variable_211 * noise_metadata_schedule_582_e5938);
        (noise_metadata_schedule_582_e5939,)
    } else {
        (noise_variable_212,)
    }
};
            noise_variable_212 = noise_metadata_schedule_582_e5941;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_583_e5943: f64 = (-noise_variable_334);
            let noise_metadata_schedule_583_e5945: f64 = (noise_metadata_schedule_583_e5943 * noise_variable_212);
            let noise_metadata_schedule_583_e5947: f64 = if noise_metadata_schedule_583_e5945 < params.p147 { 1.0 } else { 0.0 };
            noise_variable_579 = noise_metadata_schedule_583_e5947;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_584_e5967,) = {
    if ((((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 == 0.0)) && (noise_variable_575 != 0.0)) && (noise_variable_576 != 0.0)) && (noise_variable_579 != 0.0)) {
        let noise_metadata_schedule_584_e5962: f64 = (-noise_variable_334);
        let noise_metadata_schedule_584_e5964: f64 = (noise_metadata_schedule_584_e5962 * noise_variable_212);
        let noise_metadata_schedule_584_e5965: f64 = (noise_metadata_schedule_584_e5964).exp();
        (noise_metadata_schedule_584_e5965,)
    } else {
        (noise_variable_337,)
    }
};
            noise_variable_337 = noise_metadata_schedule_584_e5967;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_585_e5985,) = {
    if ((((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 == 0.0)) && (noise_variable_575 != 0.0)) && (noise_variable_576 != 0.0)) && (noise_variable_579 == 0.0)) {
        let noise_metadata_schedule_585_e5983: f64 = (params.p147).exp();
        (noise_metadata_schedule_585_e5983,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_585_e5985;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_586_e6011,) = {
    if ((((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 == 0.0)) && (noise_variable_575 != 0.0)) && (noise_variable_576 != 0.0)) && (noise_variable_579 == 0.0)) {
        let noise_metadata_schedule_586_e6003: f64 = (-noise_variable_334);
        let noise_metadata_schedule_586_e6005: f64 = (noise_metadata_schedule_586_e6003 * noise_variable_212);
        let noise_metadata_schedule_586_e6007: f64 = (noise_metadata_schedule_586_e6005 - params.p147);
        let noise_metadata_schedule_586_e6008: f64 = (1.0 + noise_metadata_schedule_586_e6007);
        let noise_metadata_schedule_586_e6009: f64 = (noise_variable_295 * noise_metadata_schedule_586_e6008);
        (noise_metadata_schedule_586_e6009,)
    } else {
        (noise_variable_337,)
    }
};
            noise_variable_337 = noise_metadata_schedule_586_e6011;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_587_e6033,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_566 == 0.0)) && (noise_variable_570 == 0.0)) && (noise_variable_575 != 0.0)) && (noise_variable_576 != 0.0)) {
        let noise_metadata_schedule_587_e6025: f64 = (params.p40 / noise_variable_334);
        let noise_metadata_schedule_587_e6028: f64 = (params.p44 - noise_variable_244);
        let noise_metadata_schedule_587_e6029: f64 = (noise_metadata_schedule_587_e6025 * noise_metadata_schedule_587_e6028);
        let noise_metadata_schedule_587_e6031: f64 = (noise_metadata_schedule_587_e6029 * noise_variable_337);
        (noise_metadata_schedule_587_e6031,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_587_e6033;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_588_e6036: f64 = if noise_variable_207 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_580 = noise_metadata_schedule_588_e6036;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_589_e6039: f64 = if params.p53 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_581 = noise_metadata_schedule_589_e6039;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_590_e6065,) = {
    if (((noise_variable_565 != 0.0) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) {
        let noise_metadata_schedule_590_e6049: f64 = (noise_variable_30 + noise_variable_186);
        let noise_metadata_schedule_590_e6050: f64 = (noise_variable_156 * noise_metadata_schedule_590_e6049);
        let noise_metadata_schedule_590_e6051: f64 = (noise_variable_6 / noise_metadata_schedule_590_e6050);
        let noise_metadata_schedule_590_e6054: f64 = (noise_variable_153 / noise_variable_35);
        let noise_metadata_schedule_590_e6056: f64 = (noise_metadata_schedule_590_e6054 * noise_variable_42);
        let noise_metadata_schedule_590_e6057: f64 = (noise_metadata_schedule_590_e6051 + noise_metadata_schedule_590_e6056);
        let noise_metadata_schedule_590_e6061: f64 = (noise_variable_30 + noise_variable_186);
        let noise_metadata_schedule_590_e6062: f64 = (noise_variable_28 / noise_metadata_schedule_590_e6061);
        let noise_metadata_schedule_590_e6063: f64 = (noise_metadata_schedule_590_e6057 + noise_metadata_schedule_590_e6062);
        (noise_metadata_schedule_590_e6063,)
    } else {
        (noise_variable_208,)
    }
};
            noise_variable_208 = noise_metadata_schedule_590_e6065;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_591_e6068: f64 = if params.p39 == 3.0 { 1.0 } else { 0.0 };
            noise_variable_582 = noise_metadata_schedule_591_e6068;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_592_e6082,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_582 != 0.0)) {
        let noise_metadata_schedule_592_e6078: f64 = (noise_variable_207 - noise_variable_208);
        let noise_metadata_schedule_592_e6080: f64 = (noise_metadata_schedule_592_e6078 / 1e-6);
        (noise_metadata_schedule_592_e6080,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_592_e6082;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_593_e6085: f64 = if noise_variable_207 < noise_variable_208 { 1.0 } else { 0.0 };
            noise_variable_583 = noise_metadata_schedule_593_e6085;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_594_e6105,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_582 != 0.0)) && (noise_variable_583 != 0.0)) {
        let noise_metadata_schedule_594_e6099: f64 = (noise_variable_279).exp();
        let noise_metadata_schedule_594_e6100: f64 = (1.0 + noise_metadata_schedule_594_e6099);
        let noise_metadata_schedule_594_e6101: f64 = (noise_metadata_schedule_594_e6100).ln();
        let noise_metadata_schedule_594_e6102: f64 = (1e-6 * noise_metadata_schedule_594_e6101);
        let noise_metadata_schedule_594_e6103: f64 = (noise_variable_207 - noise_metadata_schedule_594_e6102);
        (noise_metadata_schedule_594_e6103,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_594_e6105;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_595_e6127,) = {
    if (((((noise_variable_565 != 0.0) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_582 != 0.0)) && (noise_variable_583 == 0.0)) {
        let noise_metadata_schedule_595_e6120: f64 = (-noise_variable_279);
        let noise_metadata_schedule_595_e6121: f64 = (noise_metadata_schedule_595_e6120).exp();
        let noise_metadata_schedule_595_e6122: f64 = (1.0 + noise_metadata_schedule_595_e6121);
        let noise_metadata_schedule_595_e6123: f64 = (noise_metadata_schedule_595_e6122).ln();
        let noise_metadata_schedule_595_e6124: f64 = (1e-6 * noise_metadata_schedule_595_e6123);
        let noise_metadata_schedule_595_e6125: f64 = (noise_variable_208 - noise_metadata_schedule_595_e6124);
        (noise_metadata_schedule_595_e6125,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_595_e6127;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_596_e6139,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_582 != 0.0)) {
        let noise_metadata_schedule_596_e6137: f64 = (noise_variable_156 * noise_variable_207);
        (noise_metadata_schedule_596_e6137,)
    } else {
        (noise_variable_209,)
    }
};
            noise_variable_209 = noise_metadata_schedule_596_e6139;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_597_e6158,) = {
    if ((((noise_variable_565 != 0.0) && (noise_variable_580 != 0.0)) && (noise_variable_581 != 0.0)) && (noise_variable_582 == 0.0)) {
        let noise_metadata_schedule_597_e6150: f64 = (noise_variable_156 * noise_variable_207);
        let noise_metadata_schedule_597_e6152: f64 = (noise_metadata_schedule_597_e6150 * noise_variable_208);
        let noise_metadata_schedule_597_e6155: f64 = (noise_variable_207 + noise_variable_208);
        let noise_metadata_schedule_597_e6156: f64 = (noise_metadata_schedule_597_e6152 / noise_metadata_schedule_597_e6155);
        (noise_metadata_schedule_597_e6156,)
    } else {
        (noise_variable_209,)
    }
};
            noise_variable_209 = noise_metadata_schedule_597_e6158;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_598_e6169,) = {
    if (((noise_variable_565 != 0.0) && (noise_variable_580 != 0.0)) && (noise_variable_581 == 0.0)) {
        let noise_metadata_schedule_598_e6167: f64 = (noise_variable_156 * noise_variable_207);
        (noise_metadata_schedule_598_e6167,)
    } else {
        (noise_variable_209,)
    }
};
            noise_variable_209 = noise_metadata_schedule_598_e6169;
        }
        if matches!(source_index, 3 | 4 | 5 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27) {
            let noise_metadata_schedule_683_e7006: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_683_e7008: f64 = (noise_metadata_schedule_683_e7006 * noise_variable_2);
            noise_variable_302 = noise_metadata_schedule_683_e7008;
        }
        if matches!(source_index, 3) {
            let noise_metadata_schedule_684_e7011: f64 = (noise_variable_302 / noise_variable_28);
            noise_variable_303 = noise_metadata_schedule_684_e7011;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_685_e7014: f64 = (noise_variable_302 / noise_variable_30);
            noise_variable_304 = noise_metadata_schedule_685_e7014;
        }
        if matches!(source_index, 20 | 23 | 25 | 27) {
            let noise_metadata_schedule_686_e7017: f64 = (noise_variable_302 * noise_variable_108);
            noise_variable_305 = noise_metadata_schedule_686_e7017;
        }
        if matches!(source_index, 21 | 24) {
            let noise_metadata_schedule_687_e7020: f64 = (noise_variable_302 * noise_variable_109);
            noise_variable_306 = noise_metadata_schedule_687_e7020;
        }
        if matches!(source_index, 22 | 26) {
            let noise_metadata_schedule_688_e7023: f64 = (noise_variable_302 * noise_variable_110);
            noise_variable_307 = noise_metadata_schedule_688_e7023;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_689_e7026: f64 = (noise_variable_302 / noise_variable_186);
            let noise_metadata_schedule_689_e7029: f64 = (4.0 * noise_variable_267);
            let noise_metadata_schedule_689_e7031: f64 = (noise_metadata_schedule_689_e7029 + 5.0);
            let noise_metadata_schedule_689_e7032: f64 = (noise_metadata_schedule_689_e7026 * noise_metadata_schedule_689_e7031);
            let noise_metadata_schedule_689_e7034: f64 = (noise_metadata_schedule_689_e7032 * 0.3333333333333333);
            noise_variable_308 = noise_metadata_schedule_689_e7034;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_690_e7037: f64 = (noise_variable_155 + noise_variable_154);
            let noise_metadata_schedule_690_e7039: f64 = (noise_metadata_schedule_690_e7037 / noise_variable_153);
            noise_variable_327 = noise_metadata_schedule_690_e7039;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_691_e7042: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_691_e7044: f64 = (noise_variable_327).abs();
            let noise_metadata_schedule_691_e7045: f64 = (noise_metadata_schedule_691_e7042 * noise_metadata_schedule_691_e7044);
            noise_variable_309 = noise_metadata_schedule_691_e7045;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_692_e7048: f64 = if params.p130 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_600 = noise_metadata_schedule_692_e7048;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_693_e7055,) = {
    if (noise_variable_600 != 0.0) {
        let noise_metadata_schedule_693_e7052: f64 = (noise_variable_209 / noise_variable_327);
        let noise_metadata_schedule_693_e7053: f64 = (noise_metadata_schedule_693_e7052).abs();
        (noise_metadata_schedule_693_e7053,)
    } else {
        (noise_variable_328,)
    }
};
            noise_variable_328 = noise_metadata_schedule_693_e7055;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_694_e7060,) = {
    if (noise_variable_600 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_328,)
    }
};
            noise_variable_328 = noise_metadata_schedule_694_e7060;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_695_e7063: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_695_e7065: f64 = (noise_metadata_schedule_695_e7063 * noise_variable_209);
            let noise_metadata_schedule_695_e7068: f64 = (noise_variable_328 + 1.0);
            let noise_metadata_schedule_695_e7069: f64 = (noise_metadata_schedule_695_e7065 * noise_metadata_schedule_695_e7068);
            noise_variable_321 = noise_metadata_schedule_695_e7069;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_704_e7121: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_704_e7124: f64 = (noise_variable_158 + noise_variable_160);
            let noise_metadata_schedule_704_e7126: f64 = (noise_metadata_schedule_704_e7124 - noise_variable_57);
            let noise_metadata_schedule_704_e7128: f64 = (noise_metadata_schedule_704_e7126 + noise_variable_352);
            let noise_metadata_schedule_704_e7130: f64 = (noise_metadata_schedule_704_e7128 + noise_variable_351);
            let noise_metadata_schedule_704_e7131: f64 = (noise_metadata_schedule_704_e7130).abs();
            let noise_metadata_schedule_704_e7132: f64 = (noise_metadata_schedule_704_e7121 * noise_metadata_schedule_704_e7131);
            noise_variable_310 = noise_metadata_schedule_704_e7132;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_705_e7135: f64 = (noise_variable_158 + noise_variable_159);
            noise_variable_322 = noise_metadata_schedule_705_e7135;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_706_e7138: f64 = (noise_variable_322).abs();
            let noise_metadata_schedule_706_e7140: f64 = (noise_metadata_schedule_706_e7138).powf(params.p126);
            let noise_metadata_schedule_706_e7141: f64 = (params.p128 * noise_metadata_schedule_706_e7140);
            noise_variable_311 = noise_metadata_schedule_706_e7141;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_707_e7144: f64 = if noise_variable_322 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_604 = noise_metadata_schedule_707_e7144;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_708_e7149,) = {
    if (noise_variable_604 != 0.0) {
        let noise_metadata_schedule_708_e7147: f64 = (-noise_variable_311);
        (noise_metadata_schedule_708_e7147,)
    } else {
        (noise_variable_311,)
    }
};
            noise_variable_311 = noise_metadata_schedule_708_e7149;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_709_e7152: f64 = (noise_variable_160 + noise_variable_162);
            let noise_metadata_schedule_709_e7154: f64 = (noise_metadata_schedule_709_e7152 + noise_variable_163);
            noise_variable_323 = noise_metadata_schedule_709_e7154;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_710_e7157: f64 = (noise_variable_323).abs();
            let noise_metadata_schedule_710_e7159: f64 = (noise_metadata_schedule_710_e7157).powf(params.p127);
            let noise_metadata_schedule_710_e7160: f64 = (params.p129 * noise_metadata_schedule_710_e7159);
            noise_variable_312 = noise_metadata_schedule_710_e7160;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_711_e7163: f64 = if noise_variable_323 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_605 = noise_metadata_schedule_711_e7163;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_712_e7168,) = {
    if (noise_variable_605 != 0.0) {
        let noise_metadata_schedule_712_e7166: f64 = (-noise_variable_312);
        (noise_metadata_schedule_712_e7166,)
    } else {
        (noise_variable_312,)
    }
};
            noise_variable_312 = noise_metadata_schedule_712_e7168;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_713_e7171: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_713_e7174: f64 = (noise_variable_159 + noise_variable_162);
            let noise_metadata_schedule_713_e7176: f64 = (noise_metadata_schedule_713_e7174 + noise_variable_163);
            let noise_metadata_schedule_713_e7177: f64 = (noise_metadata_schedule_713_e7176).abs();
            let noise_metadata_schedule_713_e7178: f64 = (noise_metadata_schedule_713_e7171 * noise_metadata_schedule_713_e7177);
            noise_variable_313 = noise_metadata_schedule_713_e7178;
        }
        if matches!(source_index, 9) {
            let noise_metadata_schedule_714_e7181: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_714_e7183: f64 = (noise_variable_161).abs();
            let noise_metadata_schedule_714_e7184: f64 = (noise_metadata_schedule_714_e7181 * noise_metadata_schedule_714_e7183);
            noise_variable_314 = noise_metadata_schedule_714_e7184;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_715_e7187: f64 = (noise_variable_161).abs();
            let noise_metadata_schedule_715_e7189: f64 = (noise_metadata_schedule_715_e7187).powf(params.p126);
            let noise_metadata_schedule_715_e7190: f64 = (params.p128 * noise_metadata_schedule_715_e7189);
            noise_variable_315 = noise_metadata_schedule_715_e7190;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_716_e7193: f64 = if noise_variable_161 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_606 = noise_metadata_schedule_716_e7193;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_717_e7198,) = {
    if (noise_variable_606 != 0.0) {
        let noise_metadata_schedule_717_e7196: f64 = (-noise_variable_315);
        (noise_metadata_schedule_717_e7196,)
    } else {
        (noise_variable_315,)
    }
};
            noise_variable_315 = noise_metadata_schedule_717_e7198;
        }
        if matches!(source_index, 15 | 16) {
            let noise_metadata_schedule_718_e7201: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_718_e7203: f64 = (noise_variable_82).abs();
            let noise_metadata_schedule_718_e7204: f64 = (noise_metadata_schedule_718_e7201 * noise_metadata_schedule_718_e7203);
            noise_variable_316 = noise_metadata_schedule_718_e7204;
        }
        if matches!(source_index, 11) {
            let noise_metadata_schedule_719_e7207: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_719_e7209: f64 = (noise_variable_164).abs();
            let noise_metadata_schedule_719_e7210: f64 = (noise_metadata_schedule_719_e7207 * noise_metadata_schedule_719_e7209);
            noise_variable_317 = noise_metadata_schedule_719_e7210;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_720_e7215: f64 = (params.p5 * params.p33);
            let noise_metadata_schedule_720_e7216: f64 = (1.0 - noise_metadata_schedule_720_e7215);
            let noise_metadata_schedule_720_e7217: f64 = (params.p128 * noise_metadata_schedule_720_e7216);
            let noise_metadata_schedule_720_e7219: f64 = (noise_variable_164).abs();
            let noise_metadata_schedule_720_e7223: f64 = (params.p5 * params.p33);
            let noise_metadata_schedule_720_e7224: f64 = (1.0 - noise_metadata_schedule_720_e7223);
            let noise_metadata_schedule_720_e7225: f64 = (noise_metadata_schedule_720_e7219 / noise_metadata_schedule_720_e7224);
            let noise_metadata_schedule_720_e7227: f64 = (noise_metadata_schedule_720_e7225).powf(params.p126);
            let noise_metadata_schedule_720_e7228: f64 = (noise_metadata_schedule_720_e7217 * noise_metadata_schedule_720_e7227);
            noise_variable_319 = noise_metadata_schedule_720_e7228;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_721_e7231: f64 = if noise_variable_164 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_607 = noise_metadata_schedule_721_e7231;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_722_e7236,) = {
    if (noise_variable_607 != 0.0) {
        let noise_metadata_schedule_722_e7234: f64 = (-noise_variable_319);
        (noise_metadata_schedule_722_e7234,)
    } else {
        (noise_variable_319,)
    }
};
            noise_variable_319 = noise_metadata_schedule_722_e7236;
        }
        if matches!(source_index, 13) {
            let noise_metadata_schedule_723_e7239: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_723_e7241: f64 = (noise_variable_176).abs();
            let noise_metadata_schedule_723_e7242: f64 = (noise_metadata_schedule_723_e7239 * noise_metadata_schedule_723_e7241);
            let noise_metadata_schedule_723_e7244: f64 = (noise_metadata_schedule_723_e7242 * params.p5);
            noise_variable_318 = noise_metadata_schedule_723_e7244;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_724_e7247: f64 = if params.p33 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_608 = noise_metadata_schedule_724_e7247;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_725_e7251,) = {
    if (noise_variable_608 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_320,)
    }
};
            noise_variable_320 = noise_metadata_schedule_725_e7251;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_726_e7267,) = {
    if (noise_variable_608 == 0.0) {
        let noise_metadata_schedule_726_e7256: f64 = (params.p128 * params.p5);
        let noise_metadata_schedule_726_e7258: f64 = (noise_metadata_schedule_726_e7256 * params.p33);
        let noise_metadata_schedule_726_e7260: f64 = (noise_variable_176).abs();
        let noise_metadata_schedule_726_e7262: f64 = (noise_metadata_schedule_726_e7260 / params.p33);
        let noise_metadata_schedule_726_e7264: f64 = (noise_metadata_schedule_726_e7262).powf(params.p126);
        let noise_metadata_schedule_726_e7265: f64 = (noise_metadata_schedule_726_e7258 * noise_metadata_schedule_726_e7264);
        (noise_metadata_schedule_726_e7265,)
    } else {
        (noise_variable_320,)
    }
};
            noise_variable_320 = noise_metadata_schedule_726_e7267;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_727_e7270: f64 = if noise_variable_176 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_609 = noise_metadata_schedule_727_e7270;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_728_e7275,) = {
    if (noise_variable_609 != 0.0) {
        let noise_metadata_schedule_728_e7273: f64 = (-noise_variable_320);
        (noise_metadata_schedule_728_e7273,)
    } else {
        (noise_variable_320,)
    }
};
            noise_variable_320 = noise_metadata_schedule_728_e7275;
        }
        if matches!(source_index, 17) {
            let noise_metadata_schedule_729_e7278: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_729_e7280: f64 = (noise_variable_182).abs();
            let noise_metadata_schedule_729_e7281: f64 = (noise_metadata_schedule_729_e7278 * noise_metadata_schedule_729_e7280);
            noise_variable_324 = noise_metadata_schedule_729_e7281;
        }
        if matches!(source_index, 18) {
            let noise_metadata_schedule_730_e7284: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_730_e7286: f64 = (noise_variable_179).abs();
            let noise_metadata_schedule_730_e7287: f64 = (noise_metadata_schedule_730_e7284 * noise_metadata_schedule_730_e7286);
            noise_variable_325 = noise_metadata_schedule_730_e7287;
        }
        if matches!(source_index, 19) {
            let noise_metadata_schedule_731_e7290: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_731_e7292: f64 = (noise_variable_180).abs();
            let noise_metadata_schedule_731_e7293: f64 = (noise_metadata_schedule_731_e7290 * noise_metadata_schedule_731_e7292);
            noise_variable_326 = noise_metadata_schedule_731_e7293;
        }
        match source_index {
            0 => {
                let noise_0_psd_e8407: f64 = 1.0;
                let noise_0_psd_e388: f64 = (noise_variable_309 * params.p1);
                let noise_0_psd_e8408: f64 = (noise_0_psd_e8407 * noise_0_psd_e388);
                let psd = noise_0_psd_e8408;
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
                let noise_1_psd_e8410: f64 = 1.0;
                let noise_1_psd_e402: f64 = (noise_variable_321 * params.p1);
                let noise_1_psd_e8411: f64 = (noise_1_psd_e8410 * noise_1_psd_e402);
                let psd = noise_1_psd_e8411;
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
                let noise_2_psd_e8413: f64 = 1.0;
                let noise_2_psd_e407: f64 = (noise_variable_310 * params.p1);
                let noise_2_psd_e8414: f64 = (noise_2_psd_e8413 * noise_2_psd_e407);
                let psd = noise_2_psd_e8414;
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
                let noise_3_psd_e8416: f64 = 1.0;
                let noise_3_psd_e412: f64 = (noise_variable_303 * params.p1);
                let noise_3_psd_e8417: f64 = (noise_3_psd_e8416 * noise_3_psd_e412);
                let psd = noise_3_psd_e8417;
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
                let noise_4_psd_e8419: f64 = 1.0;
                let noise_4_psd_e417: f64 = (noise_variable_304 * params.p1);
                let noise_4_psd_e8420: f64 = (noise_4_psd_e8419 * noise_4_psd_e417);
                let psd = noise_4_psd_e8420;
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
                let noise_5_psd_e8422: f64 = 1.0;
                let noise_5_psd_e422: f64 = (noise_variable_308 * params.p1);
                let noise_5_psd_e8423: f64 = (noise_5_psd_e8422 * noise_5_psd_e422);
                let psd = noise_5_psd_e8423;
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
                let noise_6_psd_e8425: f64 = 1.0;
                let noise_6_psd_e427: f64 = (noise_variable_311 * params.p1);
                let noise_6_psd_e8426: f64 = (noise_6_psd_e8425 * noise_6_psd_e427);
                let psd = noise_6_psd_e8426;
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
                let noise_7_psd_e8428: f64 = 1.0;
                let noise_7_psd_e433: f64 = (noise_variable_312 * params.p1);
                let noise_7_psd_e8429: f64 = (noise_7_psd_e8428 * noise_7_psd_e433);
                let psd = noise_7_psd_e8429;
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
                let noise_8_psd_e8431: f64 = 1.0;
                let noise_8_psd_e439: f64 = (noise_variable_313 * params.p1);
                let noise_8_psd_e8432: f64 = (noise_8_psd_e8431 * noise_8_psd_e439);
                let psd = noise_8_psd_e8432;
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
                let noise_9_psd_e8434: f64 = 1.0;
                let noise_9_psd_e444: f64 = (noise_variable_314 * params.p1);
                let noise_9_psd_e8435: f64 = (noise_9_psd_e8434 * noise_9_psd_e444);
                let psd = noise_9_psd_e8435;
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
                let noise_10_psd_e8437: f64 = 1.0;
                let noise_10_psd_e449: f64 = (noise_variable_315 * params.p1);
                let noise_10_psd_e8438: f64 = (noise_10_psd_e8437 * noise_10_psd_e449);
                let psd = noise_10_psd_e8438;
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
                let noise_11_psd_e8440: f64 = 1.0;
                let noise_11_psd_e455: f64 = (noise_variable_317 * params.p1);
                let noise_11_psd_e8441: f64 = (noise_11_psd_e8440 * noise_11_psd_e455);
                let psd = noise_11_psd_e8441;
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
                let noise_12_psd_e8443: f64 = 1.0;
                let noise_12_psd_e460: f64 = (noise_variable_319 * params.p1);
                let noise_12_psd_e8444: f64 = (noise_12_psd_e8443 * noise_12_psd_e460);
                let psd = noise_12_psd_e8444;
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
                let noise_13_psd_e8446: f64 = 1.0;
                let noise_13_psd_e466: f64 = (noise_variable_318 * params.p1);
                let noise_13_psd_e8447: f64 = (noise_13_psd_e8446 * noise_13_psd_e466);
                let psd = noise_13_psd_e8447;
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
                let noise_14_psd_e8449: f64 = 1.0;
                let noise_14_psd_e471: f64 = (noise_variable_320 * params.p1);
                let noise_14_psd_e8450: f64 = (noise_14_psd_e8449 * noise_14_psd_e471);
                let psd = noise_14_psd_e8450;
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
                let noise_15_psd_e8452: f64 = 1.0;
                let noise_15_psd_e478: f64 = (noise_variable_316 * params.p1);
                let noise_15_psd_e8453: f64 = (noise_15_psd_e8452 * noise_15_psd_e478);
                let psd = noise_15_psd_e8453;
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
                let noise_16_psd_e8455: f64 = 1.0;
                let noise_16_psd_e487: f64 = (noise_variable_316 * params.p1);
                let noise_16_psd_e8456: f64 = (noise_16_psd_e8455 * noise_16_psd_e487);
                let psd = noise_16_psd_e8456;
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
                let noise_17_psd_e8458: f64 = 1.0;
                let noise_17_psd_e494: f64 = (noise_variable_324 * params.p1);
                let noise_17_psd_e8459: f64 = (noise_17_psd_e8458 * noise_17_psd_e494);
                let psd = noise_17_psd_e8459;
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
                let noise_18_psd_e8461: f64 = 1.0;
                let noise_18_psd_e499: f64 = (noise_variable_325 * params.p1);
                let noise_18_psd_e8462: f64 = (noise_18_psd_e8461 * noise_18_psd_e499);
                let psd = noise_18_psd_e8462;
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
                let noise_19_psd_e8464: f64 = 1.0;
                let noise_19_psd_e504: f64 = (noise_variable_326 * params.p1);
                let noise_19_psd_e8465: f64 = (noise_19_psd_e8464 * noise_19_psd_e504);
                let psd = noise_19_psd_e8465;
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
                let noise_20_psd_e8467: f64 = 1.0;
                let noise_20_psd_e512: f64 = (noise_variable_305 * params.p1);
                let noise_20_psd_e8468: f64 = (noise_20_psd_e8467 * noise_20_psd_e512);
                let psd = noise_20_psd_e8468;
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
                let noise_21_psd_e8470: f64 = 1.0;
                let noise_21_psd_e522: f64 = (noise_variable_306 * params.p1);
                let noise_21_psd_e8471: f64 = (noise_21_psd_e8470 * noise_21_psd_e522);
                let psd = noise_21_psd_e8471;
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
                let noise_22_psd_e8473: f64 = 1.0;
                let noise_22_psd_e532: f64 = (noise_variable_307 * params.p1);
                let noise_22_psd_e8474: f64 = (noise_22_psd_e8473 * noise_22_psd_e532);
                let psd = noise_22_psd_e8474;
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
                let noise_23_psd_e8476: f64 = 1.0;
                let noise_23_psd_e543: f64 = (noise_variable_305 * params.p1);
                let noise_23_psd_e8477: f64 = (noise_23_psd_e8476 * noise_23_psd_e543);
                let psd = noise_23_psd_e8477;
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
                let noise_24_psd_e8479: f64 = 1.0;
                let noise_24_psd_e554: f64 = (noise_variable_306 * params.p1);
                let noise_24_psd_e8480: f64 = (noise_24_psd_e8479 * noise_24_psd_e554);
                let psd = noise_24_psd_e8480;
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
                let noise_25_psd_e8482: f64 = 1.0;
                let noise_25_psd_e565: f64 = (noise_variable_305 * params.p1);
                let noise_25_psd_e8483: f64 = (noise_25_psd_e8482 * noise_25_psd_e565);
                let psd = noise_25_psd_e8483;
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
                let noise_26_psd_e8485: f64 = 1.0;
                let noise_26_psd_e576: f64 = (noise_variable_307 * params.p1);
                let noise_26_psd_e8486: f64 = (noise_26_psd_e8485 * noise_26_psd_e576);
                let psd = noise_26_psd_e8486;
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
                let noise_27_psd_e8488: f64 = 1.0;
                let noise_27_psd_e588: f64 = (noise_variable_305 * params.p1);
                let noise_27_psd_e8489: f64 = (noise_27_psd_e8488 * noise_27_psd_e588);
                let psd = noise_27_psd_e8489;
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

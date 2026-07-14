#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_S_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 8, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_D_S_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 8, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
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
        let noise_source_active = match source_index {
            0 => {
                params.p1 != 0.0
            }
            1 => {
                params.p1 != 0.0
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
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_1_e194: f64 = (11.7 * 8.8541879239442e-12);
            noise_variable_199 = noise_metadata_schedule_1_e194;
        }
        if matches!(source_index, 0 | 1) {
            noise_variable_157 = 0.0;
        }
        if matches!(source_index, 0 | 1) {
            noise_variable_6 = 0.0;
        }
        if matches!(source_index, 0 | 1) {
            noise_variable_175 = 0.0;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_6_e201: f64 = (noise_variable_199 / params.p13);
            noise_variable_31 = noise_metadata_schedule_6_e201;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_7_e204: f64 = (noise_variable_31 * params.p14);
            let noise_metadata_schedule_7_e205: f64 = (noise_metadata_schedule_7_e204).sqrt();
            noise_variable_34 = noise_metadata_schedule_7_e205;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_8_e208: f64 = (noise_variable_34 * params.p25);
            noise_variable_35 = noise_metadata_schedule_8_e208;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_9_e211: f64 = (3.0 * noise_variable_31);
            let noise_metadata_schedule_9_e213: f64 = (noise_metadata_schedule_9_e211 * params.p28);
            noise_variable_32 = noise_metadata_schedule_9_e213;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_10_e216: f64 = (noise_variable_31 * params.p29);
            noise_variable_33 = noise_metadata_schedule_10_e216;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_12_e223: f64 = (noise_variable_199 * params.p22);
            let noise_metadata_schedule_12_e224: f64 = (params.p13 / noise_metadata_schedule_12_e223);
            noise_variable_37 = noise_metadata_schedule_12_e224;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_13_e227: f64 = (params.p30 + params.p30);
            let noise_metadata_schedule_13_e229: f64 = (noise_metadata_schedule_13_e227 / params.p13);
            noise_variable_182 = noise_metadata_schedule_13_e229;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_14_e235,) = {
    if (params.p0 > 0.0) {
        (0.5,)
    } else {
        (0.3333333333333,)
    }
};
            noise_variable_39 = noise_metadata_schedule_14_e235;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_15_e238: f64 = (-1e21);
            let noise_metadata_schedule_15_e239: f64 = (-noise_metadata_schedule_15_e238);
            let noise_metadata_schedule_15_e240: f64 = if params.p3 == noise_metadata_schedule_15_e239 { 1.0 } else { 0.0 };
            noise_variable_238 = noise_metadata_schedule_15_e240;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_16_e246,) = {
    if (noise_variable_238 != 0.0) {
        let noise_metadata_schedule_16_e242: f64 = ctx.temperature();
        let noise_metadata_schedule_16_e244: f64 = (noise_metadata_schedule_16_e242 + params.p2);
        (noise_metadata_schedule_16_e244,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_16_e246;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_17_e253,) = {
    if (noise_variable_238 == 0.0) {
        let noise_metadata_schedule_17_e251: f64 = (params.p3 + 273.15);
        (noise_metadata_schedule_17_e251,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_17_e253;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_18_e256: f64 = (-1e21);
            let noise_metadata_schedule_18_e257: f64 = (-noise_metadata_schedule_18_e256);
            let noise_metadata_schedule_18_e258: f64 = if params.p4 == noise_metadata_schedule_18_e257 { 1.0 } else { 0.0 };
            noise_variable_239 = noise_metadata_schedule_18_e258;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_19_e264,) = {
    if (noise_variable_239 != 0.0) {
        let noise_metadata_schedule_19_e262: f64 = (25.0 + 273.15);
        (noise_metadata_schedule_19_e262,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_19_e264;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_20_e271,) = {
    if (noise_variable_239 == 0.0) {
        let noise_metadata_schedule_20_e269: f64 = (params.p4 + 273.15);
        (noise_metadata_schedule_20_e269,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_20_e271;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_21_e273: f64 = (noise_variable_49 * THERMAL_VOLTAGE_PER_K);
            noise_variable_17 = noise_metadata_schedule_21_e273;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_22_e276: f64 = (0.1 * noise_variable_17);
            noise_variable_25 = noise_metadata_schedule_22_e276;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_23_e279: f64 = (1.0 / noise_variable_17);
            noise_variable_24 = noise_metadata_schedule_23_e279;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_24_e282: f64 = (noise_variable_17 + noise_variable_17);
            noise_variable_26 = noise_metadata_schedule_24_e282;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_25_e285: f64 = (noise_variable_26 + noise_variable_26);
            noise_variable_27 = noise_metadata_schedule_25_e285;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_26_e288: f64 = (noise_variable_17 * noise_variable_17);
            noise_variable_28 = noise_metadata_schedule_26_e288;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_27_e291: f64 = (noise_variable_28 + noise_variable_28);
            noise_variable_29 = noise_metadata_schedule_27_e291;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_28_e294: f64 = (16.0 * noise_variable_28);
            noise_variable_30 = noise_metadata_schedule_28_e294;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_29_e298: f64 = (0.000702 * noise_variable_49);
            let noise_metadata_schedule_29_e300: f64 = (noise_metadata_schedule_29_e298 * noise_variable_49);
            let noise_metadata_schedule_29_e303: f64 = (noise_variable_49 + 1108.0);
            let noise_metadata_schedule_29_e304: f64 = (noise_metadata_schedule_29_e300 / noise_metadata_schedule_29_e303);
            let noise_metadata_schedule_29_e305: f64 = (1.16 - noise_metadata_schedule_29_e304);
            noise_variable_51 = noise_metadata_schedule_29_e305;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_30_e309: f64 = (0.000702 * noise_variable_55);
            let noise_metadata_schedule_30_e311: f64 = (noise_metadata_schedule_30_e309 * noise_variable_55);
            let noise_metadata_schedule_30_e314: f64 = (noise_variable_55 + 1108.0);
            let noise_metadata_schedule_30_e315: f64 = (noise_metadata_schedule_30_e311 / noise_metadata_schedule_30_e314);
            let noise_metadata_schedule_30_e316: f64 = (1.16 - noise_metadata_schedule_30_e315);
            noise_variable_52 = noise_metadata_schedule_30_e316;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_31_e319: f64 = (noise_variable_49 - noise_variable_55);
            noise_variable_53 = noise_metadata_schedule_31_e319;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_32_e322: f64 = (noise_variable_49 / noise_variable_55);
            noise_variable_54 = noise_metadata_schedule_32_e322;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_33_e326: f64 = (params.p16 * noise_variable_53);
            let noise_metadata_schedule_33_e327: f64 = (params.p15 - noise_metadata_schedule_33_e326);
            noise_variable_56 = noise_metadata_schedule_33_e327;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_34_e331: f64 = (noise_variable_54).powf(params.p20);
            let noise_metadata_schedule_34_e332: f64 = (params.p19 * noise_metadata_schedule_34_e331);
            noise_variable_58 = noise_metadata_schedule_34_e332;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_35_e336: f64 = (noise_variable_54).powf(params.p24);
            let noise_metadata_schedule_35_e337: f64 = (params.p23 * noise_metadata_schedule_35_e336);
            noise_variable_59 = noise_metadata_schedule_35_e337;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_37_e347: f64 = (params.p18 * noise_variable_54);
            let noise_metadata_schedule_37_e350: f64 = (3.0 * noise_variable_17);
            let noise_metadata_schedule_37_e352: f64 = (noise_variable_54).ln();
            let noise_metadata_schedule_37_e353: f64 = (noise_metadata_schedule_37_e350 * noise_metadata_schedule_37_e352);
            let noise_metadata_schedule_37_e354: f64 = (noise_metadata_schedule_37_e347 - noise_metadata_schedule_37_e353);
            let noise_metadata_schedule_37_e357: f64 = (noise_variable_52 * noise_variable_54);
            let noise_metadata_schedule_37_e358: f64 = (noise_metadata_schedule_37_e354 - noise_metadata_schedule_37_e357);
            let noise_metadata_schedule_37_e360: f64 = (noise_metadata_schedule_37_e358 + noise_variable_51);
            noise_variable_61 = noise_metadata_schedule_37_e360;
        }
        if matches!(source_index, 0 | 1) {
            noise_variable_0 = 0.2;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_39_e364: f64 = (noise_variable_61 - noise_variable_0);
            noise_variable_1 = noise_metadata_schedule_39_e364;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_40_e369: f64 = (noise_variable_1 * noise_variable_1);
            let noise_metadata_schedule_40_e372: f64 = (noise_variable_17 * noise_variable_17);
            let noise_metadata_schedule_40_e373: f64 = (noise_metadata_schedule_40_e369 + noise_metadata_schedule_40_e372);
            let noise_metadata_schedule_40_e374: f64 = (noise_metadata_schedule_40_e373).sqrt();
            let noise_metadata_schedule_40_e375: f64 = (noise_variable_1 + noise_metadata_schedule_40_e374);
            let noise_metadata_schedule_40_e376: f64 = (0.5 * noise_metadata_schedule_40_e375);
            let noise_metadata_schedule_40_e378: f64 = (noise_metadata_schedule_40_e376 + noise_variable_0);
            noise_variable_61 = noise_metadata_schedule_40_e378;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_41_e380: f64 = (noise_variable_61).sqrt();
            noise_variable_71 = noise_metadata_schedule_41_e380;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_42_e383: f64 = (1.0 / noise_variable_59);
            noise_variable_40 = noise_metadata_schedule_42_e383;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_43_e386: f64 = (noise_variable_34 * noise_variable_59);
            noise_variable_41 = noise_metadata_schedule_43_e386;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_46_e395: f64 = (params.p5 + params.p26);
            noise_variable_191 = noise_metadata_schedule_46_e395;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_47_e398: f64 = (params.p6 + params.p27);
            noise_variable_192 = noise_metadata_schedule_47_e398;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_48_e401: f64 = (noise_variable_59 * noise_variable_191);
            noise_variable_158 = noise_metadata_schedule_48_e401;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_49_e405: f64 = (0.5 * noise_variable_158);
            let noise_metadata_schedule_49_e407: f64 = (noise_metadata_schedule_49_e405 * noise_variable_24);
            let noise_metadata_schedule_49_e408: f64 = (noise_metadata_schedule_49_e407).ln();
            let noise_metadata_schedule_49_e410: f64 = (noise_metadata_schedule_49_e408 - 0.6);
            let noise_metadata_schedule_49_e411: f64 = (noise_variable_17 * noise_metadata_schedule_49_e410);
            noise_variable_173 = noise_metadata_schedule_49_e411;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_50_e415: f64 = (noise_variable_192 * noise_variable_191);
            let noise_metadata_schedule_50_e416: f64 = (noise_metadata_schedule_50_e415).sqrt();
            let noise_metadata_schedule_50_e417: f64 = (1.0 / noise_metadata_schedule_50_e416);
            noise_variable_48 = noise_metadata_schedule_50_e417;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_51_e420: f64 = if params.p0 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_240 = noise_metadata_schedule_51_e420;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_52_e435,) = {
    if (noise_variable_240 != 0.0) {
        let (noise_metadata_schedule_52_e433,) = {
            if (params.p38 != 1e-6) {
                let noise_metadata_schedule_52_e428: f64 = (params.p38 - 1e-6);
                let noise_metadata_schedule_52_e429: f64 = (noise_variable_48 * noise_metadata_schedule_52_e428);
                let noise_metadata_schedule_52_e431: f64 = (noise_metadata_schedule_52_e429 + noise_variable_56);
                (noise_metadata_schedule_52_e431,)
            } else {
                (noise_variable_56,)
            }
        };
        (noise_metadata_schedule_52_e433,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_52_e435;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_53_e452,) = {
    if (noise_variable_240 == 0.0) {
        let (noise_metadata_schedule_53_e450,) = {
            if (params.p38 != 1e-6) {
                let noise_metadata_schedule_53_e444: f64 = (1e-6 - params.p38);
                let noise_metadata_schedule_53_e445: f64 = (noise_variable_48 * noise_metadata_schedule_53_e444);
                let noise_metadata_schedule_53_e447: f64 = (noise_metadata_schedule_53_e445 - noise_variable_56);
                (noise_metadata_schedule_53_e447,)
            } else {
                let noise_metadata_schedule_53_e449: f64 = (-noise_variable_56);
                (noise_metadata_schedule_53_e449,)
            }
        };
        (noise_metadata_schedule_53_e450,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_53_e452;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_54_e467,) = {
    if (params.p39 != 1e-6) {
        let noise_metadata_schedule_54_e461: f64 = (params.p39 - 1e-6);
        let noise_metadata_schedule_54_e463: f64 = (noise_metadata_schedule_54_e461 * noise_variable_48);
        let noise_metadata_schedule_54_e464: f64 = (1.0 + noise_metadata_schedule_54_e463);
        let noise_metadata_schedule_54_e465: f64 = (noise_variable_58 * noise_metadata_schedule_54_e464);
        (noise_metadata_schedule_54_e465,)
    } else {
        (noise_variable_58,)
    }
};
            let noise_metadata_schedule_54_e468: f64 = (noise_variable_192 * noise_metadata_schedule_54_e467);
            noise_variable_50 = noise_metadata_schedule_54_e468;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_55_e480,) = {
    if (params.p40 != 1e-6) {
        let noise_metadata_schedule_55_e475: f64 = (params.p40 - 1e-6);
        let noise_metadata_schedule_55_e477: f64 = (noise_metadata_schedule_55_e475 * noise_variable_48);
        let noise_metadata_schedule_55_e478: f64 = (params.p17 + noise_metadata_schedule_55_e477);
        (noise_metadata_schedule_55_e478,)
    } else {
        (params.p17,)
    }
};
            noise_variable_62 = noise_metadata_schedule_55_e480;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_56_e483: f64 = (noise_variable_62 * noise_variable_71);
            noise_variable_153 = noise_metadata_schedule_56_e483;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_57_e486: f64 = if noise_variable_182 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_241 = noise_metadata_schedule_57_e486;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_58_e490,) = {
    if (noise_variable_241 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_183,)
    }
};
            noise_variable_183 = noise_metadata_schedule_58_e490;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_59_e503,) = {
    if (noise_variable_241 == 0.0) {
        let noise_metadata_schedule_59_e497: f64 = (params.p31 * params.p8);
        let noise_metadata_schedule_59_e498: f64 = (noise_variable_191 / noise_metadata_schedule_59_e497);
        let noise_metadata_schedule_59_e500: f64 = (noise_metadata_schedule_59_e498 - 0.1);
        let noise_metadata_schedule_59_e501: f64 = (0.28 * noise_metadata_schedule_59_e500);
        (noise_metadata_schedule_59_e501,)
    } else {
        (noise_variable_184,)
    }
};
            noise_variable_184 = noise_metadata_schedule_59_e503;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_60_e521,) = {
    if (noise_variable_241 == 0.0) {
        let noise_metadata_schedule_60_e512: f64 = (noise_variable_184 * noise_variable_184);
        let noise_metadata_schedule_60_e514: f64 = (noise_metadata_schedule_60_e512 + 0.001936);
        let noise_metadata_schedule_60_e515: f64 = (noise_metadata_schedule_60_e514).sqrt();
        let noise_metadata_schedule_60_e516: f64 = (noise_variable_184 + noise_metadata_schedule_60_e515);
        let noise_metadata_schedule_60_e517: f64 = (0.5 * noise_metadata_schedule_60_e516);
        let noise_metadata_schedule_60_e518: f64 = (1.0 + noise_metadata_schedule_60_e517);
        let noise_metadata_schedule_60_e519: f64 = (1.0 / noise_metadata_schedule_60_e518);
        (noise_metadata_schedule_60_e519,)
    } else {
        (noise_variable_242,)
    }
};
            noise_variable_242 = noise_metadata_schedule_60_e521;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_61_e530,) = {
    if (noise_variable_241 == 0.0) {
        let noise_metadata_schedule_61_e526: f64 = (noise_variable_182 * noise_variable_242);
        let noise_metadata_schedule_61_e528: f64 = (noise_metadata_schedule_61_e526 * noise_variable_242);
        (noise_metadata_schedule_61_e528,)
    } else {
        (noise_variable_183,)
    }
};
            noise_variable_183 = noise_metadata_schedule_61_e530;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_62_e533: f64 = (params.p0 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[3])));
            noise_variable_145 = noise_metadata_schedule_62_e533;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_63_e536: f64 = (params.p0 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[3])));
            noise_variable_147 = noise_metadata_schedule_63_e536;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_64_e539: f64 = (params.p0 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[3])));
            noise_variable_146 = noise_metadata_schedule_64_e539;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_65_e542: f64 = (noise_variable_146 - noise_variable_147);
            let noise_metadata_schedule_65_e544: f64 = if noise_metadata_schedule_65_e542 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_243 = noise_metadata_schedule_65_e544;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_67_e553,) = {
    if (noise_variable_243 != 0.0) {
        (noise_variable_147,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_67_e553;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_68_e557,) = {
    if (noise_variable_243 != 0.0) {
        (noise_variable_146,)
    } else {
        (noise_variable_147,)
    }
};
            noise_variable_147 = noise_metadata_schedule_68_e557;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_69_e561,) = {
    if (noise_variable_243 != 0.0) {
        (noise_variable_38,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_69_e561;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_71_e569: f64 = (noise_variable_145 - noise_variable_57);
            let noise_metadata_schedule_71_e571: f64 = (noise_metadata_schedule_71_e569 - noise_variable_183);
            let noise_metadata_schedule_71_e573: f64 = (noise_metadata_schedule_71_e571 + noise_variable_61);
            let noise_metadata_schedule_71_e575: f64 = (noise_metadata_schedule_71_e573 + noise_variable_153);
            noise_variable_143 = noise_metadata_schedule_71_e575;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_72_e578: f64 = (noise_variable_143 * noise_variable_143);
            let noise_metadata_schedule_72_e581: f64 = (2.0 * noise_variable_30);
            let noise_metadata_schedule_72_e582: f64 = (noise_metadata_schedule_72_e578 + noise_metadata_schedule_72_e581);
            let noise_metadata_schedule_72_e583: f64 = (noise_metadata_schedule_72_e582).sqrt();
            noise_variable_144 = noise_metadata_schedule_72_e583;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_73_e587: f64 = (noise_variable_143 + noise_variable_144);
            let noise_metadata_schedule_73_e588: f64 = (0.5 * noise_metadata_schedule_73_e587);
            noise_variable_3 = noise_metadata_schedule_73_e588;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_74_e591: f64 = (noise_variable_61 + noise_variable_147);
            noise_variable_70 = noise_metadata_schedule_74_e591;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_75_e594: f64 = (noise_variable_70 * noise_variable_70);
            let noise_metadata_schedule_75_e596: f64 = (noise_metadata_schedule_75_e594 + noise_variable_30);
            let noise_metadata_schedule_75_e597: f64 = (noise_metadata_schedule_75_e596).sqrt();
            noise_variable_76 = noise_metadata_schedule_75_e597;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_76_e601: f64 = (noise_variable_70 + noise_variable_76);
            let noise_metadata_schedule_76_e602: f64 = (0.5 * noise_metadata_schedule_76_e601);
            let noise_metadata_schedule_76_e603: f64 = (noise_metadata_schedule_76_e602).sqrt();
            noise_variable_74 = noise_metadata_schedule_76_e603;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_77_e606: f64 = (noise_variable_61 + noise_variable_146);
            noise_variable_69 = noise_metadata_schedule_77_e606;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_78_e609: f64 = (noise_variable_69 * noise_variable_69);
            let noise_metadata_schedule_78_e611: f64 = (noise_metadata_schedule_78_e609 + noise_variable_30);
            let noise_metadata_schedule_78_e612: f64 = (noise_metadata_schedule_78_e611).sqrt();
            noise_variable_75 = noise_metadata_schedule_78_e612;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_79_e616: f64 = (noise_variable_69 + noise_variable_75);
            let noise_metadata_schedule_79_e617: f64 = (0.5 * noise_metadata_schedule_79_e616);
            let noise_metadata_schedule_79_e618: f64 = (noise_metadata_schedule_79_e617).sqrt();
            noise_variable_73 = noise_metadata_schedule_79_e618;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_80_e621: f64 = (noise_variable_32 * params.p7);
            let noise_metadata_schedule_80_e623: f64 = (noise_metadata_schedule_80_e621 / noise_variable_192);
            noise_variable_45 = noise_metadata_schedule_80_e623;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_81_e626: f64 = (noise_variable_33 * params.p8);
            let noise_metadata_schedule_81_e628: f64 = (noise_metadata_schedule_81_e626 / noise_variable_191);
            noise_variable_46 = noise_metadata_schedule_81_e628;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_82_e632: f64 = (0.25 * noise_variable_62);
            let noise_metadata_schedule_82_e634: f64 = (noise_metadata_schedule_82_e632 * noise_variable_62);
            let noise_metadata_schedule_82_e635: f64 = (noise_variable_3 + noise_metadata_schedule_82_e634);
            let noise_metadata_schedule_82_e636: f64 = (noise_metadata_schedule_82_e635).sqrt();
            noise_variable_67 = noise_metadata_schedule_82_e636;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_83_e639: f64 = (noise_variable_3 - noise_variable_61);
            let noise_metadata_schedule_83_e644: f64 = (0.5 * noise_variable_62);
            let noise_metadata_schedule_83_e645: f64 = (noise_variable_67 - noise_metadata_schedule_83_e644);
            let noise_metadata_schedule_83_e646: f64 = (noise_variable_62 * noise_metadata_schedule_83_e645);
            let noise_metadata_schedule_83_e647: f64 = (noise_metadata_schedule_83_e639 - noise_metadata_schedule_83_e646);
            noise_variable_68 = noise_metadata_schedule_83_e647;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_84_e650: f64 = (noise_variable_68 + noise_variable_61);
            let noise_metadata_schedule_84_e652: f64 = (noise_metadata_schedule_84_e650 + noise_variable_25);
            let noise_metadata_schedule_84_e653: f64 = (noise_metadata_schedule_84_e652).sqrt();
            noise_variable_174 = noise_metadata_schedule_84_e653;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_85_e658: f64 = (noise_variable_74 + noise_variable_73);
            let noise_metadata_schedule_85_e659: f64 = (noise_variable_46 * noise_metadata_schedule_85_e658);
            let noise_metadata_schedule_85_e660: f64 = (noise_variable_62 - noise_metadata_schedule_85_e659);
            let noise_metadata_schedule_85_e663: f64 = (noise_variable_45 * noise_variable_174);
            let noise_metadata_schedule_85_e664: f64 = (noise_metadata_schedule_85_e660 + noise_metadata_schedule_85_e663);
            noise_variable_64 = noise_metadata_schedule_85_e664;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_86_e667: f64 = (noise_variable_64 * noise_variable_64);
            let noise_metadata_schedule_86_e669: f64 = (noise_metadata_schedule_86_e667 + noise_variable_25);
            let noise_metadata_schedule_86_e670: f64 = (noise_metadata_schedule_86_e669).sqrt();
            noise_variable_65 = noise_metadata_schedule_86_e670;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_87_e674: f64 = (noise_variable_64 + noise_variable_65);
            let noise_metadata_schedule_87_e675: f64 = (0.5 * noise_metadata_schedule_87_e674);
            noise_variable_4 = noise_metadata_schedule_87_e675;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_88_e679: f64 = (0.25 * noise_variable_4);
            let noise_metadata_schedule_88_e681: f64 = (noise_metadata_schedule_88_e679 * noise_variable_4);
            let noise_metadata_schedule_88_e682: f64 = (noise_variable_3 + noise_metadata_schedule_88_e681);
            let noise_metadata_schedule_88_e683: f64 = (noise_metadata_schedule_88_e682).sqrt();
            noise_variable_66 = noise_metadata_schedule_88_e683;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_89_e686: f64 = (noise_variable_3 - noise_variable_61);
            let noise_metadata_schedule_89_e691: f64 = (0.5 * noise_variable_4);
            let noise_metadata_schedule_89_e692: f64 = (noise_variable_66 - noise_metadata_schedule_89_e691);
            let noise_metadata_schedule_89_e693: f64 = (noise_variable_4 * noise_metadata_schedule_89_e692);
            let noise_metadata_schedule_89_e694: f64 = (noise_metadata_schedule_89_e686 - noise_metadata_schedule_89_e693);
            noise_variable_5 = noise_metadata_schedule_89_e694;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_90_e697: f64 = (noise_variable_5 - noise_variable_147);
            let noise_metadata_schedule_90_e699: f64 = (noise_metadata_schedule_90_e697 * noise_variable_24);
            noise_variable_0 = noise_metadata_schedule_90_e699;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_91_e702: f64 = (-0.35);
            let noise_metadata_schedule_91_e703: f64 = if noise_variable_0 > noise_metadata_schedule_91_e702 { 1.0 } else { 0.0 };
            noise_variable_244 = noise_metadata_schedule_91_e703;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_92_e716,) = {
    if (noise_variable_244 != 0.0) {
        let noise_metadata_schedule_92_e708: f64 = (1.3 + noise_variable_0);
        let noise_metadata_schedule_92_e711: f64 = (noise_variable_0 + 1.6);
        let noise_metadata_schedule_92_e712: f64 = (noise_metadata_schedule_92_e711).ln();
        let noise_metadata_schedule_92_e713: f64 = (noise_metadata_schedule_92_e708 - noise_metadata_schedule_92_e712);
        let noise_metadata_schedule_92_e714: f64 = (2.0 / noise_metadata_schedule_92_e713);
        (noise_metadata_schedule_92_e714,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_92_e716;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_93_e729,) = {
    if (noise_variable_244 != 0.0) {
        let noise_metadata_schedule_93_e720: f64 = (2.0 + noise_variable_196);
        let noise_metadata_schedule_93_e723: f64 = (1.0 + noise_variable_0);
        let noise_metadata_schedule_93_e725: f64 = (noise_variable_196).ln();
        let noise_metadata_schedule_93_e726: f64 = (noise_metadata_schedule_93_e723 + noise_metadata_schedule_93_e725);
        let noise_metadata_schedule_93_e727: f64 = (noise_metadata_schedule_93_e720 / noise_metadata_schedule_93_e726);
        (noise_metadata_schedule_93_e727,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_93_e729;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_94_e742,) = {
    if (noise_variable_244 != 0.0) {
        let noise_metadata_schedule_94_e733: f64 = (1.0 + noise_variable_0);
        let noise_metadata_schedule_94_e735: f64 = (noise_variable_197).ln();
        let noise_metadata_schedule_94_e736: f64 = (noise_metadata_schedule_94_e733 + noise_metadata_schedule_94_e735);
        let noise_metadata_schedule_94_e739: f64 = (2.0 + noise_variable_197);
        let noise_metadata_schedule_94_e740: f64 = (noise_metadata_schedule_94_e736 / noise_metadata_schedule_94_e739);
        (noise_metadata_schedule_94_e740,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_94_e742;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_95_e745: f64 = (-15.0);
            let noise_metadata_schedule_95_e746: f64 = if noise_variable_0 > noise_metadata_schedule_95_e745 { 1.0 } else { 0.0 };
            noise_variable_245 = noise_metadata_schedule_95_e746;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_96_e757,) = {
    if ((noise_variable_244 == 0.0) && (noise_variable_245 != 0.0)) {
        let noise_metadata_schedule_96_e753: f64 = (-noise_variable_0);
        let noise_metadata_schedule_96_e754: f64 = (noise_metadata_schedule_96_e753).exp();
        let noise_metadata_schedule_96_e755: f64 = (1.55 + noise_metadata_schedule_96_e754);
        (noise_metadata_schedule_96_e755,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_96_e757;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_97_e773,) = {
    if ((noise_variable_244 == 0.0) && (noise_variable_245 != 0.0)) {
        let noise_metadata_schedule_97_e764: f64 = (2.0 + noise_variable_196);
        let noise_metadata_schedule_97_e767: f64 = (1.0 + noise_variable_0);
        let noise_metadata_schedule_97_e769: f64 = (noise_variable_196).ln();
        let noise_metadata_schedule_97_e770: f64 = (noise_metadata_schedule_97_e767 + noise_metadata_schedule_97_e769);
        let noise_metadata_schedule_97_e771: f64 = (noise_metadata_schedule_97_e764 / noise_metadata_schedule_97_e770);
        (noise_metadata_schedule_97_e771,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_97_e773;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_98_e789,) = {
    if ((noise_variable_244 == 0.0) && (noise_variable_245 != 0.0)) {
        let noise_metadata_schedule_98_e780: f64 = (1.0 + noise_variable_0);
        let noise_metadata_schedule_98_e782: f64 = (noise_variable_197).ln();
        let noise_metadata_schedule_98_e783: f64 = (noise_metadata_schedule_98_e780 + noise_metadata_schedule_98_e782);
        let noise_metadata_schedule_98_e786: f64 = (2.0 + noise_variable_197);
        let noise_metadata_schedule_98_e787: f64 = (noise_metadata_schedule_98_e783 / noise_metadata_schedule_98_e786);
        (noise_metadata_schedule_98_e787,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_98_e789;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_99_e792: f64 = (-23.0);
            let noise_metadata_schedule_99_e793: f64 = if noise_variable_0 > noise_metadata_schedule_99_e792 { 1.0 } else { 0.0 };
            noise_variable_246 = noise_metadata_schedule_99_e793;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_100_e809,) = {
    if (((noise_variable_244 == 0.0) && (noise_variable_245 == 0.0)) && (noise_variable_246 != 0.0)) {
        let noise_metadata_schedule_100_e804: f64 = (-noise_variable_0);
        let noise_metadata_schedule_100_e805: f64 = (noise_metadata_schedule_100_e804).exp();
        let noise_metadata_schedule_100_e806: f64 = (2.0 + noise_metadata_schedule_100_e805);
        let noise_metadata_schedule_100_e807: f64 = (1.0 / noise_metadata_schedule_100_e806);
        (noise_metadata_schedule_100_e807,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_100_e809;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_101_e823,) = {
    if (((noise_variable_244 == 0.0) && (noise_variable_245 == 0.0)) && (noise_variable_246 == 0.0)) {
        let noise_metadata_schedule_101_e819: f64 = (noise_variable_0).exp();
        let noise_metadata_schedule_101_e821: f64 = (noise_metadata_schedule_101_e819 + 1e-64);
        (noise_metadata_schedule_101_e821,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_101_e823;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_102_e827: f64 = (1.0 + noise_variable_195);
            let noise_metadata_schedule_102_e828: f64 = (noise_variable_195 * noise_metadata_schedule_102_e827);
            noise_variable_7 = noise_metadata_schedule_102_e828;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_103_e830: f64 = (noise_variable_7).sqrt();
            noise_variable_87 = noise_metadata_schedule_103_e830;
        }
        if matches!(source_index, 1) {
            noise_variable_90 = noise_variable_195;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_105_e834: f64 = (noise_variable_17 / noise_variable_158);
            noise_variable_160 = noise_metadata_schedule_105_e834;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_106_e838: f64 = (noise_variable_87 * noise_variable_160);
            let noise_metadata_schedule_106_e839: f64 = (0.25 + noise_metadata_schedule_106_e838);
            let noise_metadata_schedule_106_e840: f64 = (noise_metadata_schedule_106_e839).sqrt();
            noise_variable_80 = noise_metadata_schedule_106_e840;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_107_e844: f64 = (noise_variable_80 - 0.5);
            let noise_metadata_schedule_107_e845: f64 = (noise_variable_158 * noise_metadata_schedule_107_e844);
            noise_variable_10 = noise_metadata_schedule_107_e845;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_108_e849: f64 = (noise_variable_146 - noise_variable_147);
            let noise_metadata_schedule_108_e850: f64 = (0.5 * noise_metadata_schedule_108_e849);
            noise_variable_77 = noise_metadata_schedule_108_e850;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_109_e856: f64 = (noise_variable_10 * noise_variable_24);
            let noise_metadata_schedule_109_e857: f64 = (noise_variable_87 - noise_metadata_schedule_109_e856);
            let noise_metadata_schedule_109_e858: f64 = (params.p25 * noise_metadata_schedule_109_e857);
            let noise_metadata_schedule_109_e860: f64 = (noise_metadata_schedule_109_e858 + 0.015625);
            let noise_metadata_schedule_109_e861: f64 = (noise_variable_30 * noise_metadata_schedule_109_e860);
            noise_variable_78 = noise_metadata_schedule_109_e861;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_110_e864: f64 = (noise_variable_10 * noise_variable_10);
            let noise_metadata_schedule_110_e866: f64 = (noise_metadata_schedule_110_e864 + noise_variable_78);
            let noise_metadata_schedule_110_e867: f64 = (noise_metadata_schedule_110_e866).sqrt();
            noise_variable_81 = noise_metadata_schedule_110_e867;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_111_e870: f64 = (noise_variable_77 - noise_variable_10);
            let noise_metadata_schedule_111_e873: f64 = (noise_variable_77 - noise_variable_10);
            let noise_metadata_schedule_111_e874: f64 = (noise_metadata_schedule_111_e870 * noise_metadata_schedule_111_e873);
            let noise_metadata_schedule_111_e876: f64 = (noise_metadata_schedule_111_e874 + noise_variable_78);
            let noise_metadata_schedule_111_e877: f64 = (noise_metadata_schedule_111_e876).sqrt();
            noise_variable_82 = noise_metadata_schedule_111_e877;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_112_e880: f64 = (noise_variable_81 - noise_variable_82);
            noise_variable_79 = noise_metadata_schedule_112_e880;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_113_e885: f64 = (noise_variable_7).ln();
            let noise_metadata_schedule_113_e886: f64 = (0.75 * noise_metadata_schedule_113_e885);
            let noise_metadata_schedule_113_e887: f64 = (noise_variable_87 - noise_metadata_schedule_113_e886);
            let noise_metadata_schedule_113_e889: f64 = (noise_metadata_schedule_113_e887 * noise_variable_160);
            let noise_metadata_schedule_113_e890: f64 = (0.25 + noise_metadata_schedule_113_e889);
            let noise_metadata_schedule_113_e891: f64 = (noise_metadata_schedule_113_e890).sqrt();
            noise_variable_83 = noise_metadata_schedule_113_e891;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_114_e895: f64 = (noise_variable_83 - 0.5);
            let noise_metadata_schedule_114_e896: f64 = (noise_variable_158 * noise_metadata_schedule_114_e895);
            let noise_metadata_schedule_114_e898: f64 = (noise_metadata_schedule_114_e896 + noise_variable_173);
            noise_variable_11 = noise_metadata_schedule_114_e898;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_115_e901: f64 = (noise_variable_77 - noise_variable_11);
            noise_variable_159 = noise_metadata_schedule_115_e901;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_116_e904: f64 = (noise_variable_11 * noise_variable_11);
            let noise_metadata_schedule_116_e906: f64 = (noise_metadata_schedule_116_e904 + noise_variable_78);
            let noise_metadata_schedule_116_e907: f64 = (noise_metadata_schedule_116_e906).sqrt();
            noise_variable_84 = noise_metadata_schedule_116_e907;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_117_e910: f64 = (noise_variable_159 * noise_variable_159);
            let noise_metadata_schedule_117_e912: f64 = (noise_metadata_schedule_117_e910 + noise_variable_78);
            let noise_metadata_schedule_117_e913: f64 = (noise_metadata_schedule_117_e912).sqrt();
            noise_variable_85 = noise_metadata_schedule_117_e913;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_118_e916: f64 = (noise_variable_5 - noise_variable_77);
            let noise_metadata_schedule_118_e918: f64 = (noise_metadata_schedule_118_e916 - noise_variable_147);
            let noise_metadata_schedule_118_e920: f64 = (noise_metadata_schedule_118_e918 - noise_variable_84);
            let noise_metadata_schedule_118_e922: f64 = (noise_metadata_schedule_118_e920 + noise_variable_85);
            let noise_metadata_schedule_118_e924: f64 = (noise_metadata_schedule_118_e922 * noise_variable_24);
            noise_variable_0 = noise_metadata_schedule_118_e924;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_119_e927: f64 = (-0.35);
            let noise_metadata_schedule_119_e928: f64 = if noise_variable_0 > noise_metadata_schedule_119_e927 { 1.0 } else { 0.0 };
            noise_variable_247 = noise_metadata_schedule_119_e928;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_120_e941,) = {
    if (noise_variable_247 != 0.0) {
        let noise_metadata_schedule_120_e933: f64 = (1.3 + noise_variable_0);
        let noise_metadata_schedule_120_e936: f64 = (noise_variable_0 + 1.6);
        let noise_metadata_schedule_120_e937: f64 = (noise_metadata_schedule_120_e936).ln();
        let noise_metadata_schedule_120_e938: f64 = (noise_metadata_schedule_120_e933 - noise_metadata_schedule_120_e937);
        let noise_metadata_schedule_120_e939: f64 = (2.0 / noise_metadata_schedule_120_e938);
        (noise_metadata_schedule_120_e939,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_120_e941;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_121_e954,) = {
    if (noise_variable_247 != 0.0) {
        let noise_metadata_schedule_121_e945: f64 = (2.0 + noise_variable_196);
        let noise_metadata_schedule_121_e948: f64 = (1.0 + noise_variable_0);
        let noise_metadata_schedule_121_e950: f64 = (noise_variable_196).ln();
        let noise_metadata_schedule_121_e951: f64 = (noise_metadata_schedule_121_e948 + noise_metadata_schedule_121_e950);
        let noise_metadata_schedule_121_e952: f64 = (noise_metadata_schedule_121_e945 / noise_metadata_schedule_121_e951);
        (noise_metadata_schedule_121_e952,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_121_e954;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_122_e967,) = {
    if (noise_variable_247 != 0.0) {
        let noise_metadata_schedule_122_e958: f64 = (1.0 + noise_variable_0);
        let noise_metadata_schedule_122_e960: f64 = (noise_variable_197).ln();
        let noise_metadata_schedule_122_e961: f64 = (noise_metadata_schedule_122_e958 + noise_metadata_schedule_122_e960);
        let noise_metadata_schedule_122_e964: f64 = (2.0 + noise_variable_197);
        let noise_metadata_schedule_122_e965: f64 = (noise_metadata_schedule_122_e961 / noise_metadata_schedule_122_e964);
        (noise_metadata_schedule_122_e965,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_122_e967;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_123_e970: f64 = (-15.0);
            let noise_metadata_schedule_123_e971: f64 = if noise_variable_0 > noise_metadata_schedule_123_e970 { 1.0 } else { 0.0 };
            noise_variable_248 = noise_metadata_schedule_123_e971;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_124_e982,) = {
    if ((noise_variable_247 == 0.0) && (noise_variable_248 != 0.0)) {
        let noise_metadata_schedule_124_e978: f64 = (-noise_variable_0);
        let noise_metadata_schedule_124_e979: f64 = (noise_metadata_schedule_124_e978).exp();
        let noise_metadata_schedule_124_e980: f64 = (1.55 + noise_metadata_schedule_124_e979);
        (noise_metadata_schedule_124_e980,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_124_e982;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_125_e998,) = {
    if ((noise_variable_247 == 0.0) && (noise_variable_248 != 0.0)) {
        let noise_metadata_schedule_125_e989: f64 = (2.0 + noise_variable_196);
        let noise_metadata_schedule_125_e992: f64 = (1.0 + noise_variable_0);
        let noise_metadata_schedule_125_e994: f64 = (noise_variable_196).ln();
        let noise_metadata_schedule_125_e995: f64 = (noise_metadata_schedule_125_e992 + noise_metadata_schedule_125_e994);
        let noise_metadata_schedule_125_e996: f64 = (noise_metadata_schedule_125_e989 / noise_metadata_schedule_125_e995);
        (noise_metadata_schedule_125_e996,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_125_e998;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_126_e1014,) = {
    if ((noise_variable_247 == 0.0) && (noise_variable_248 != 0.0)) {
        let noise_metadata_schedule_126_e1005: f64 = (1.0 + noise_variable_0);
        let noise_metadata_schedule_126_e1007: f64 = (noise_variable_197).ln();
        let noise_metadata_schedule_126_e1008: f64 = (noise_metadata_schedule_126_e1005 + noise_metadata_schedule_126_e1007);
        let noise_metadata_schedule_126_e1011: f64 = (2.0 + noise_variable_197);
        let noise_metadata_schedule_126_e1012: f64 = (noise_metadata_schedule_126_e1008 / noise_metadata_schedule_126_e1011);
        (noise_metadata_schedule_126_e1012,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_126_e1014;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_127_e1017: f64 = (-23.0);
            let noise_metadata_schedule_127_e1018: f64 = if noise_variable_0 > noise_metadata_schedule_127_e1017 { 1.0 } else { 0.0 };
            noise_variable_249 = noise_metadata_schedule_127_e1018;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_128_e1034,) = {
    if (((noise_variable_247 == 0.0) && (noise_variable_248 == 0.0)) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_128_e1029: f64 = (-noise_variable_0);
        let noise_metadata_schedule_128_e1030: f64 = (noise_metadata_schedule_128_e1029).exp();
        let noise_metadata_schedule_128_e1031: f64 = (2.0 + noise_metadata_schedule_128_e1030);
        let noise_metadata_schedule_128_e1032: f64 = (1.0 / noise_metadata_schedule_128_e1031);
        (noise_metadata_schedule_128_e1032,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_128_e1034;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_129_e1048,) = {
    if (((noise_variable_247 == 0.0) && (noise_variable_248 == 0.0)) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_129_e1044: f64 = (noise_variable_0).exp();
        let noise_metadata_schedule_129_e1046: f64 = (noise_metadata_schedule_129_e1044 + 1e-64);
        (noise_metadata_schedule_129_e1046,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_129_e1048;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_130_e1052: f64 = (1.0 + noise_variable_195);
            let noise_metadata_schedule_130_e1053: f64 = (noise_variable_195 * noise_metadata_schedule_130_e1052);
            noise_variable_9 = noise_metadata_schedule_130_e1053;
        }
        if matches!(source_index, 1) {
            noise_variable_92 = noise_variable_195;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_133_e1061: f64 = (noise_variable_77 - noise_variable_79);
            let noise_metadata_schedule_133_e1063: f64 = (noise_metadata_schedule_133_e1061 / noise_variable_41);
            let noise_metadata_schedule_133_e1064: f64 = (1.0 + noise_metadata_schedule_133_e1063);
            let noise_metadata_schedule_133_e1065: f64 = (noise_metadata_schedule_133_e1064).ln();
            let noise_metadata_schedule_133_e1066: f64 = (noise_variable_35 * noise_metadata_schedule_133_e1065);
            noise_variable_12 = noise_metadata_schedule_133_e1066;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_134_e1069: f64 = (noise_variable_191 - noise_variable_12);
            let noise_metadata_schedule_134_e1072: f64 = (noise_variable_77 + noise_variable_79);
            let noise_metadata_schedule_134_e1074: f64 = (noise_metadata_schedule_134_e1072 * noise_variable_40);
            let noise_metadata_schedule_134_e1075: f64 = (noise_metadata_schedule_134_e1069 + noise_metadata_schedule_134_e1074);
            noise_variable_155 = noise_metadata_schedule_134_e1075;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_135_e1078: f64 = (0.1 * noise_variable_191);
            noise_variable_154 = noise_metadata_schedule_135_e1078;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_136_e1081: f64 = (noise_variable_155 * noise_variable_155);
            let noise_metadata_schedule_136_e1084: f64 = (noise_variable_154 * noise_variable_154);
            let noise_metadata_schedule_136_e1085: f64 = (noise_metadata_schedule_136_e1081 + noise_metadata_schedule_136_e1084);
            let noise_metadata_schedule_136_e1086: f64 = (noise_metadata_schedule_136_e1085).sqrt();
            noise_variable_63 = noise_metadata_schedule_136_e1086;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_137_e1090: f64 = (noise_variable_155 + noise_variable_63);
            let noise_metadata_schedule_137_e1091: f64 = (0.5 * noise_metadata_schedule_137_e1090);
            noise_variable_13 = noise_metadata_schedule_137_e1091;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_138_e1094: f64 = (noise_variable_5 - noise_variable_146);
            let noise_metadata_schedule_138_e1096: f64 = (noise_metadata_schedule_138_e1094 * noise_variable_24);
            noise_variable_0 = noise_metadata_schedule_138_e1096;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_139_e1099: f64 = (-0.35);
            let noise_metadata_schedule_139_e1100: f64 = if noise_variable_0 > noise_metadata_schedule_139_e1099 { 1.0 } else { 0.0 };
            noise_variable_250 = noise_metadata_schedule_139_e1100;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_140_e1113,) = {
    if (noise_variable_250 != 0.0) {
        let noise_metadata_schedule_140_e1105: f64 = (1.3 + noise_variable_0);
        let noise_metadata_schedule_140_e1108: f64 = (noise_variable_0 + 1.6);
        let noise_metadata_schedule_140_e1109: f64 = (noise_metadata_schedule_140_e1108).ln();
        let noise_metadata_schedule_140_e1110: f64 = (noise_metadata_schedule_140_e1105 - noise_metadata_schedule_140_e1109);
        let noise_metadata_schedule_140_e1111: f64 = (2.0 / noise_metadata_schedule_140_e1110);
        (noise_metadata_schedule_140_e1111,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_140_e1113;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_141_e1126,) = {
    if (noise_variable_250 != 0.0) {
        let noise_metadata_schedule_141_e1117: f64 = (2.0 + noise_variable_196);
        let noise_metadata_schedule_141_e1120: f64 = (1.0 + noise_variable_0);
        let noise_metadata_schedule_141_e1122: f64 = (noise_variable_196).ln();
        let noise_metadata_schedule_141_e1123: f64 = (noise_metadata_schedule_141_e1120 + noise_metadata_schedule_141_e1122);
        let noise_metadata_schedule_141_e1124: f64 = (noise_metadata_schedule_141_e1117 / noise_metadata_schedule_141_e1123);
        (noise_metadata_schedule_141_e1124,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_141_e1126;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_142_e1139,) = {
    if (noise_variable_250 != 0.0) {
        let noise_metadata_schedule_142_e1130: f64 = (1.0 + noise_variable_0);
        let noise_metadata_schedule_142_e1132: f64 = (noise_variable_197).ln();
        let noise_metadata_schedule_142_e1133: f64 = (noise_metadata_schedule_142_e1130 + noise_metadata_schedule_142_e1132);
        let noise_metadata_schedule_142_e1136: f64 = (2.0 + noise_variable_197);
        let noise_metadata_schedule_142_e1137: f64 = (noise_metadata_schedule_142_e1133 / noise_metadata_schedule_142_e1136);
        (noise_metadata_schedule_142_e1137,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_142_e1139;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_143_e1142: f64 = (-15.0);
            let noise_metadata_schedule_143_e1143: f64 = if noise_variable_0 > noise_metadata_schedule_143_e1142 { 1.0 } else { 0.0 };
            noise_variable_251 = noise_metadata_schedule_143_e1143;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_144_e1154,) = {
    if ((noise_variable_250 == 0.0) && (noise_variable_251 != 0.0)) {
        let noise_metadata_schedule_144_e1150: f64 = (-noise_variable_0);
        let noise_metadata_schedule_144_e1151: f64 = (noise_metadata_schedule_144_e1150).exp();
        let noise_metadata_schedule_144_e1152: f64 = (1.55 + noise_metadata_schedule_144_e1151);
        (noise_metadata_schedule_144_e1152,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_144_e1154;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_145_e1170,) = {
    if ((noise_variable_250 == 0.0) && (noise_variable_251 != 0.0)) {
        let noise_metadata_schedule_145_e1161: f64 = (2.0 + noise_variable_196);
        let noise_metadata_schedule_145_e1164: f64 = (1.0 + noise_variable_0);
        let noise_metadata_schedule_145_e1166: f64 = (noise_variable_196).ln();
        let noise_metadata_schedule_145_e1167: f64 = (noise_metadata_schedule_145_e1164 + noise_metadata_schedule_145_e1166);
        let noise_metadata_schedule_145_e1168: f64 = (noise_metadata_schedule_145_e1161 / noise_metadata_schedule_145_e1167);
        (noise_metadata_schedule_145_e1168,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_145_e1170;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_146_e1186,) = {
    if ((noise_variable_250 == 0.0) && (noise_variable_251 != 0.0)) {
        let noise_metadata_schedule_146_e1177: f64 = (1.0 + noise_variable_0);
        let noise_metadata_schedule_146_e1179: f64 = (noise_variable_197).ln();
        let noise_metadata_schedule_146_e1180: f64 = (noise_metadata_schedule_146_e1177 + noise_metadata_schedule_146_e1179);
        let noise_metadata_schedule_146_e1183: f64 = (2.0 + noise_variable_197);
        let noise_metadata_schedule_146_e1184: f64 = (noise_metadata_schedule_146_e1180 / noise_metadata_schedule_146_e1183);
        (noise_metadata_schedule_146_e1184,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_146_e1186;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_147_e1189: f64 = (-23.0);
            let noise_metadata_schedule_147_e1190: f64 = if noise_variable_0 > noise_metadata_schedule_147_e1189 { 1.0 } else { 0.0 };
            noise_variable_252 = noise_metadata_schedule_147_e1190;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_148_e1206,) = {
    if (((noise_variable_250 == 0.0) && (noise_variable_251 == 0.0)) && (noise_variable_252 != 0.0)) {
        let noise_metadata_schedule_148_e1201: f64 = (-noise_variable_0);
        let noise_metadata_schedule_148_e1202: f64 = (noise_metadata_schedule_148_e1201).exp();
        let noise_metadata_schedule_148_e1203: f64 = (2.0 + noise_metadata_schedule_148_e1202);
        let noise_metadata_schedule_148_e1204: f64 = (1.0 / noise_metadata_schedule_148_e1203);
        (noise_metadata_schedule_148_e1204,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_148_e1206;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_149_e1220,) = {
    if (((noise_variable_250 == 0.0) && (noise_variable_251 == 0.0)) && (noise_variable_252 == 0.0)) {
        let noise_metadata_schedule_149_e1216: f64 = (noise_variable_0).exp();
        let noise_metadata_schedule_149_e1218: f64 = (noise_metadata_schedule_149_e1216 + 1e-64);
        (noise_metadata_schedule_149_e1218,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_149_e1220;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_150_e1224: f64 = (1.0 + noise_variable_195);
            let noise_metadata_schedule_150_e1225: f64 = (noise_variable_195 * noise_metadata_schedule_150_e1224);
            noise_variable_8 = noise_metadata_schedule_150_e1225;
        }
        if matches!(source_index, 1) {
            noise_variable_91 = noise_variable_195;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_153_e1231: f64 = (0.25 + noise_variable_7);
            noise_variable_95 = noise_metadata_schedule_153_e1231;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_154_e1234: f64 = (0.25 + noise_variable_8);
            noise_variable_96 = noise_metadata_schedule_154_e1234;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_155_e1236: f64 = (noise_variable_95).sqrt();
            noise_variable_93 = noise_metadata_schedule_155_e1236;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_156_e1238: f64 = (noise_variable_96).sqrt();
            noise_variable_94 = noise_metadata_schedule_156_e1238;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_157_e1241: f64 = (noise_variable_93 + noise_variable_94);
            let noise_metadata_schedule_157_e1244: f64 = (noise_variable_93 + noise_variable_94);
            let noise_metadata_schedule_157_e1245: f64 = (noise_metadata_schedule_157_e1241 * noise_metadata_schedule_157_e1244);
            noise_variable_99 = noise_metadata_schedule_157_e1245;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_158_e1248: f64 = (noise_variable_5 + noise_variable_61);
            let noise_metadata_schedule_158_e1250: f64 = (noise_metadata_schedule_158_e1248 + 1e-6);
            noise_variable_107 = noise_metadata_schedule_158_e1250;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_159_e1253: f64 = (noise_variable_107).sqrt();
            let noise_metadata_schedule_159_e1254: f64 = (2.0 * noise_metadata_schedule_159_e1253);
            noise_variable_108 = noise_metadata_schedule_159_e1254;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_160_e1257: f64 = (noise_variable_62 / noise_variable_108);
            noise_variable_111 = noise_metadata_schedule_160_e1257;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_161_e1261: f64 = (noise_variable_108 + noise_variable_62);
            let noise_metadata_schedule_161_e1262: f64 = (noise_variable_62 / noise_metadata_schedule_161_e1261);
            noise_variable_112 = noise_metadata_schedule_161_e1262;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_162_e1265: f64 = (1.0 + noise_variable_111);
            let noise_metadata_schedule_162_e1266: f64 = (-noise_metadata_schedule_162_e1265);
            let noise_metadata_schedule_162_e1268: f64 = (noise_metadata_schedule_162_e1266 * noise_variable_17);
            let noise_metadata_schedule_162_e1271: f64 = (0.66666666 + 0.66666666);
            let noise_metadata_schedule_162_e1275: f64 = (noise_variable_94 * noise_variable_93);
            let noise_metadata_schedule_162_e1276: f64 = (noise_variable_96 + noise_metadata_schedule_162_e1275);
            let noise_metadata_schedule_162_e1278: f64 = (noise_metadata_schedule_162_e1276 + noise_variable_95);
            let noise_metadata_schedule_162_e1279: f64 = (noise_metadata_schedule_162_e1271 * noise_metadata_schedule_162_e1278);
            let noise_metadata_schedule_162_e1282: f64 = (noise_variable_93 + noise_variable_94);
            let noise_metadata_schedule_162_e1283: f64 = (noise_metadata_schedule_162_e1279 / noise_metadata_schedule_162_e1282);
            let noise_metadata_schedule_162_e1285: f64 = (noise_metadata_schedule_162_e1283 - 1.0);
            let noise_metadata_schedule_162_e1286: f64 = (noise_metadata_schedule_162_e1268 * noise_metadata_schedule_162_e1285);
            noise_variable_100 = noise_metadata_schedule_162_e1286;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_163_e1288: f64 = (-0.5);
            let noise_metadata_schedule_163_e1290: f64 = (noise_metadata_schedule_163_e1288 * noise_variable_62);
            let noise_metadata_schedule_163_e1292: f64 = (noise_metadata_schedule_163_e1290 * noise_variable_108);
            let noise_metadata_schedule_163_e1295: f64 = (noise_variable_112 * noise_variable_100);
            let noise_metadata_schedule_163_e1296: f64 = (noise_metadata_schedule_163_e1292 - noise_metadata_schedule_163_e1295);
            noise_variable_101 = noise_metadata_schedule_163_e1296;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_164_e1299: f64 = if params.p22 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_253 = noise_metadata_schedule_164_e1299;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_165_e1308,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_165_e1303: f64 = (noise_variable_5 * noise_variable_5);
        let noise_metadata_schedule_165_e1305: f64 = (noise_metadata_schedule_165_e1303 + noise_variable_29);
        let noise_metadata_schedule_165_e1306: f64 = (noise_metadata_schedule_165_e1305).sqrt();
        (noise_metadata_schedule_165_e1306,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_165_e1308;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_166_e1316,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_166_e1313: f64 = (noise_variable_5 + noise_variable_175);
        let noise_metadata_schedule_166_e1314: f64 = (0.5 * noise_metadata_schedule_166_e1313);
        (noise_metadata_schedule_166_e1314,)
    } else {
        (noise_variable_6,)
    }
};
            noise_variable_6 = noise_metadata_schedule_166_e1316;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_167_e1324,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_167_e1321: f64 = (params.p21 * noise_variable_6);
        let noise_metadata_schedule_167_e1322: f64 = (1.0 + noise_metadata_schedule_167_e1321);
        (noise_metadata_schedule_167_e1322,)
    } else {
        (noise_variable_157,)
    }
};
            noise_variable_157 = noise_metadata_schedule_167_e1324;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_168_e1332,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_168_e1329: f64 = (noise_variable_13 * noise_variable_157);
        let noise_metadata_schedule_168_e1330: f64 = (noise_variable_50 / noise_metadata_schedule_168_e1329);
        (noise_metadata_schedule_168_e1330,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_168_e1332;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_169_e1336: f64 = (noise_variable_39 * noise_variable_100);
            let noise_metadata_schedule_169_e1337: f64 = (noise_variable_101 + noise_metadata_schedule_169_e1336);
            let noise_metadata_schedule_169_e1339: f64 = if noise_metadata_schedule_169_e1337 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_254 = noise_metadata_schedule_169_e1339;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_170_e1354,) = {
    if ((noise_variable_253 == 0.0) && (noise_variable_254 != 0.0)) {
        let noise_metadata_schedule_170_e1349: f64 = (noise_variable_39 * noise_variable_100);
        let noise_metadata_schedule_170_e1350: f64 = (noise_variable_101 + noise_metadata_schedule_170_e1349);
        let noise_metadata_schedule_170_e1351: f64 = (noise_variable_37 * noise_metadata_schedule_170_e1350);
        let noise_metadata_schedule_170_e1352: f64 = (1.0 + noise_metadata_schedule_170_e1351);
        (noise_metadata_schedule_170_e1352,)
    } else {
        (noise_variable_47,)
    }
};
            noise_variable_47 = noise_metadata_schedule_170_e1354;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_171_e1370,) = {
    if ((noise_variable_253 == 0.0) && (noise_variable_254 == 0.0)) {
        let noise_metadata_schedule_171_e1365: f64 = (noise_variable_39 * noise_variable_100);
        let noise_metadata_schedule_171_e1366: f64 = (noise_variable_101 + noise_metadata_schedule_171_e1365);
        let noise_metadata_schedule_171_e1367: f64 = (noise_variable_37 * noise_metadata_schedule_171_e1366);
        let noise_metadata_schedule_171_e1368: f64 = (1.0 - noise_metadata_schedule_171_e1367);
        (noise_metadata_schedule_171_e1368,)
    } else {
        (noise_variable_47,)
    }
};
            noise_variable_47 = noise_metadata_schedule_171_e1370;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_172_e1379,) = {
    if (noise_variable_253 == 0.0) {
        let noise_metadata_schedule_172_e1376: f64 = (noise_variable_37 * noise_variable_153);
        let noise_metadata_schedule_172_e1377: f64 = (1.0 + noise_metadata_schedule_172_e1376);
        (noise_metadata_schedule_172_e1377,)
    } else {
        (noise_variable_156,)
    }
};
            noise_variable_156 = noise_metadata_schedule_172_e1379;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_173_e1390,) = {
    if (noise_variable_253 == 0.0) {
        let noise_metadata_schedule_173_e1384: f64 = (noise_variable_50 * noise_variable_156);
        let noise_metadata_schedule_173_e1387: f64 = (noise_variable_13 * noise_variable_47);
        let noise_metadata_schedule_173_e1388: f64 = (noise_metadata_schedule_173_e1384 / noise_metadata_schedule_173_e1387);
        (noise_metadata_schedule_173_e1388,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_173_e1390;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_174_e1393: f64 = (noise_variable_61 + noise_variable_5);
            let noise_metadata_schedule_174_e1395: f64 = (noise_metadata_schedule_174_e1393 + noise_variable_27);
            let noise_metadata_schedule_174_e1396: f64 = (noise_metadata_schedule_174_e1395).sqrt();
            noise_variable_72 = noise_metadata_schedule_174_e1396;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_175_e1401: f64 = (2.0 * noise_variable_72);
            let noise_metadata_schedule_175_e1402: f64 = (noise_variable_62 / noise_metadata_schedule_175_e1401);
            let noise_metadata_schedule_175_e1403: f64 = (1.0 + noise_metadata_schedule_175_e1402);
            noise_variable_15 = noise_metadata_schedule_175_e1403;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_176_e1406: f64 = (noise_variable_7 - noise_variable_9);
            noise_variable_86 = noise_metadata_schedule_176_e1406;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_177_e1409: f64 = (noise_variable_29 * noise_variable_15);
            let noise_metadata_schedule_177_e1411: f64 = (noise_metadata_schedule_177_e1409 * noise_variable_14);
            noise_variable_16 = noise_metadata_schedule_177_e1411;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_181_e1431: f64 = (noise_variable_100).abs();
            let noise_metadata_schedule_181_e1432: f64 = (noise_variable_14 * noise_metadata_schedule_181_e1431);
            noise_variable_152 = noise_metadata_schedule_181_e1432;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_182_e1436: f64 = (noise_variable_65 + noise_variable_65);
            let noise_metadata_schedule_182_e1437: f64 = (noise_variable_4 / noise_metadata_schedule_182_e1436);
            noise_variable_0 = noise_metadata_schedule_182_e1437;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_183_e1440: f64 = (noise_variable_3 / noise_variable_144);
            noise_variable_1 = noise_metadata_schedule_183_e1440;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_186_e1459: f64 = (noise_variable_45 * noise_variable_0);
            let noise_metadata_schedule_186_e1463: f64 = (0.5 * noise_variable_62);
            let noise_metadata_schedule_186_e1464: f64 = (noise_variable_67 - noise_metadata_schedule_186_e1463);
            let noise_metadata_schedule_186_e1465: f64 = (noise_metadata_schedule_186_e1459 * noise_metadata_schedule_186_e1464);
            let noise_metadata_schedule_186_e1468: f64 = (noise_variable_67 * noise_variable_174);
            let noise_metadata_schedule_186_e1469: f64 = (noise_metadata_schedule_186_e1465 / noise_metadata_schedule_186_e1468);
            let noise_metadata_schedule_186_e1471: f64 = (noise_metadata_schedule_186_e1469 * noise_variable_1);
            noise_variable_162 = noise_metadata_schedule_186_e1471;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_187_e1474: f64 = (noise_variable_5 + noise_variable_61);
            let noise_metadata_schedule_187_e1476: f64 = (noise_metadata_schedule_187_e1474 / noise_variable_66);
            noise_variable_2 = noise_metadata_schedule_187_e1476;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_190_e1486: f64 = (-noise_variable_2);
            let noise_metadata_schedule_190_e1488: f64 = (noise_metadata_schedule_190_e1486 * noise_variable_162);
            let noise_metadata_schedule_190_e1493: f64 = (noise_variable_66 + noise_variable_66);
            let noise_metadata_schedule_190_e1494: f64 = (noise_variable_4 / noise_metadata_schedule_190_e1493);
            let noise_metadata_schedule_190_e1495: f64 = (1.0 - noise_metadata_schedule_190_e1494);
            let noise_metadata_schedule_190_e1497: f64 = (noise_metadata_schedule_190_e1495 * noise_variable_1);
            let noise_metadata_schedule_190_e1498: f64 = (noise_metadata_schedule_190_e1488 + noise_metadata_schedule_190_e1497);
            noise_variable_114 = noise_metadata_schedule_190_e1498;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_191_e1501: f64 = (noise_variable_90 * noise_variable_24);
            noise_variable_0 = noise_metadata_schedule_191_e1501;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_194_e1512: f64 = (noise_variable_0 * noise_variable_114);
            noise_variable_118 = noise_metadata_schedule_194_e1512;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_195_e1516: f64 = (4.0 * noise_variable_80);
            let noise_metadata_schedule_195_e1518: f64 = (noise_metadata_schedule_195_e1516 * noise_variable_87);
            let noise_metadata_schedule_195_e1519: f64 = (noise_variable_17 / noise_metadata_schedule_195_e1518);
            noise_variable_0 = noise_metadata_schedule_195_e1519;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_198_e1528: f64 = (noise_variable_0 * noise_variable_118);
            noise_variable_123 = noise_metadata_schedule_198_e1528;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_199_e1531: f64 = (noise_variable_27 + noise_variable_27);
            let noise_metadata_schedule_199_e1533: f64 = (noise_metadata_schedule_199_e1531 * params.p25);
            noise_variable_0 = noise_metadata_schedule_199_e1533;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_200_e1537: f64 = (noise_variable_87 + noise_variable_87);
            let noise_metadata_schedule_200_e1538: f64 = (noise_variable_17 / noise_metadata_schedule_200_e1537);
            noise_variable_1 = noise_metadata_schedule_200_e1538;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_203_e1556: f64 = (noise_variable_118 * noise_variable_1);
            let noise_metadata_schedule_203_e1558: f64 = (noise_metadata_schedule_203_e1556 - noise_variable_123);
            let noise_metadata_schedule_203_e1559: f64 = (noise_variable_0 * noise_metadata_schedule_203_e1558);
            noise_variable_126 = noise_metadata_schedule_203_e1559;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_204_e1562: f64 = (1.0 / noise_variable_81);
            noise_variable_0 = noise_metadata_schedule_204_e1562;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_205_e1565: f64 = (1.0 / noise_variable_82);
            noise_variable_1 = noise_metadata_schedule_205_e1565;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_206_e1568: f64 = (noise_variable_77 - noise_variable_10);
            noise_variable_2 = noise_metadata_schedule_206_e1568;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_209_e1606: f64 = (noise_variable_10 * noise_variable_123);
            let noise_metadata_schedule_209_e1608: f64 = (noise_metadata_schedule_209_e1606 + noise_variable_126);
            let noise_metadata_schedule_209_e1610: f64 = (noise_metadata_schedule_209_e1608 * noise_variable_0);
            let noise_metadata_schedule_209_e1613: f64 = (-noise_variable_123);
            let noise_metadata_schedule_209_e1614: f64 = (noise_variable_2 * noise_metadata_schedule_209_e1613);
            let noise_metadata_schedule_209_e1616: f64 = (noise_metadata_schedule_209_e1614 + noise_variable_126);
            let noise_metadata_schedule_209_e1618: f64 = (noise_metadata_schedule_209_e1616 * noise_variable_1);
            let noise_metadata_schedule_209_e1619: f64 = (noise_metadata_schedule_209_e1610 - noise_metadata_schedule_209_e1618);
            noise_variable_129 = noise_metadata_schedule_209_e1619;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_210_e1623: f64 = (noise_variable_87 - 1.5);
            let noise_metadata_schedule_210_e1624: f64 = (noise_variable_17 * noise_metadata_schedule_210_e1623);
            let noise_metadata_schedule_210_e1627: f64 = (4.0 * noise_variable_83);
            let noise_metadata_schedule_210_e1629: f64 = (noise_metadata_schedule_210_e1627 * noise_variable_7);
            let noise_metadata_schedule_210_e1630: f64 = (noise_metadata_schedule_210_e1624 / noise_metadata_schedule_210_e1629);
            noise_variable_0 = noise_metadata_schedule_210_e1630;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_213_e1639: f64 = (noise_variable_0 * noise_variable_118);
            noise_variable_132 = noise_metadata_schedule_213_e1639;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_214_e1642: f64 = (noise_variable_92 * noise_variable_24);
            noise_variable_0 = noise_metadata_schedule_214_e1642;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_215_e1645: f64 = (1.0 / noise_variable_84);
            noise_variable_1 = noise_metadata_schedule_215_e1645;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_216_e1648: f64 = (1.0 / noise_variable_85);
            noise_variable_2 = noise_metadata_schedule_216_e1648;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_219_e1700: f64 = (noise_variable_11 * noise_variable_132);
            let noise_metadata_schedule_219_e1702: f64 = (noise_metadata_schedule_219_e1700 + noise_variable_126);
            let noise_metadata_schedule_219_e1704: f64 = (noise_metadata_schedule_219_e1702 * noise_variable_1);
            let noise_metadata_schedule_219_e1705: f64 = (noise_variable_114 - noise_metadata_schedule_219_e1704);
            let noise_metadata_schedule_219_e1708: f64 = (-noise_variable_132);
            let noise_metadata_schedule_219_e1709: f64 = (noise_variable_159 * noise_metadata_schedule_219_e1708);
            let noise_metadata_schedule_219_e1711: f64 = (noise_metadata_schedule_219_e1709 + noise_variable_126);
            let noise_metadata_schedule_219_e1713: f64 = (noise_metadata_schedule_219_e1711 * noise_variable_2);
            let noise_metadata_schedule_219_e1714: f64 = (noise_metadata_schedule_219_e1705 + noise_metadata_schedule_219_e1713);
            let noise_metadata_schedule_219_e1715: f64 = (noise_variable_0 * noise_metadata_schedule_219_e1714);
            noise_variable_135 = noise_metadata_schedule_219_e1715;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_220_e1719: f64 = (noise_variable_41 + noise_variable_77);
            let noise_metadata_schedule_220_e1721: f64 = (noise_metadata_schedule_220_e1719 - noise_variable_79);
            let noise_metadata_schedule_220_e1722: f64 = (noise_variable_35 / noise_metadata_schedule_220_e1721);
            noise_variable_0 = noise_metadata_schedule_220_e1722;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_223_e1735: f64 = (-noise_variable_0);
            let noise_metadata_schedule_223_e1737: f64 = (noise_metadata_schedule_223_e1735 * noise_variable_129);
            noise_variable_168 = noise_metadata_schedule_223_e1737;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_224_e1740: f64 = (1.0 / noise_variable_63);
            noise_variable_0 = noise_metadata_schedule_224_e1740;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_227_e1764: f64 = (-noise_variable_168);
            let noise_metadata_schedule_227_e1767: f64 = (noise_variable_129 * noise_variable_40);
            let noise_metadata_schedule_227_e1768: f64 = (noise_metadata_schedule_227_e1764 + noise_metadata_schedule_227_e1767);
            let noise_metadata_schedule_227_e1769: f64 = (noise_variable_0 * noise_metadata_schedule_227_e1768);
            noise_variable_138 = noise_metadata_schedule_227_e1769;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_228_e1772: f64 = (noise_variable_91 * noise_variable_24);
            noise_variable_0 = noise_metadata_schedule_228_e1772;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_231_e1783: f64 = (noise_variable_0 * noise_variable_114);
            noise_variable_121 = noise_metadata_schedule_231_e1783;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_232_e1786: f64 = (1.0 + noise_variable_111);
            let noise_metadata_schedule_232_e1787: f64 = (-noise_metadata_schedule_232_e1786);
            let noise_metadata_schedule_232_e1789: f64 = (noise_metadata_schedule_232_e1787 * noise_variable_17);
            let noise_metadata_schedule_232_e1791: f64 = (noise_metadata_schedule_232_e1789 * 0.66666666);
            let noise_metadata_schedule_232_e1793: f64 = (noise_metadata_schedule_232_e1791 / noise_variable_99);
            noise_variable_0 = noise_metadata_schedule_232_e1793;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_233_e1798: f64 = (2.0 * noise_variable_94);
            let noise_metadata_schedule_233_e1799: f64 = (noise_variable_93 + noise_metadata_schedule_233_e1798);
            let noise_metadata_schedule_233_e1800: f64 = (noise_variable_0 * noise_metadata_schedule_233_e1799);
            noise_variable_1 = noise_metadata_schedule_233_e1800;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_234_e1805: f64 = (2.0 * noise_variable_93);
            let noise_metadata_schedule_234_e1806: f64 = (noise_variable_94 + noise_metadata_schedule_234_e1805);
            let noise_metadata_schedule_234_e1807: f64 = (noise_variable_0 * noise_metadata_schedule_234_e1806);
            noise_variable_2 = noise_metadata_schedule_234_e1807;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_235_e1809: f64 = (-noise_variable_111);
            let noise_metadata_schedule_235_e1811: f64 = (noise_metadata_schedule_235_e1809 * noise_variable_100);
            let noise_metadata_schedule_235_e1814: f64 = (2.0 + noise_variable_111);
            let noise_metadata_schedule_235_e1816: f64 = (noise_metadata_schedule_235_e1814 + noise_variable_111);
            let noise_metadata_schedule_235_e1818: f64 = (noise_metadata_schedule_235_e1816 * noise_variable_107);
            let noise_metadata_schedule_235_e1819: f64 = (noise_metadata_schedule_235_e1811 / noise_metadata_schedule_235_e1818);
            noise_variable_0 = noise_metadata_schedule_235_e1819;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_238_e1844: f64 = (noise_variable_0 * noise_variable_114);
            let noise_metadata_schedule_238_e1847: f64 = (noise_variable_1 * noise_variable_118);
            let noise_metadata_schedule_238_e1848: f64 = (noise_metadata_schedule_238_e1844 + noise_metadata_schedule_238_e1847);
            let noise_metadata_schedule_238_e1851: f64 = (noise_variable_2 * noise_variable_121);
            let noise_metadata_schedule_238_e1852: f64 = (noise_metadata_schedule_238_e1848 + noise_metadata_schedule_238_e1851);
            noise_variable_187 = noise_metadata_schedule_238_e1852;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_239_e1855: f64 = (1.0 + noise_variable_111);
            let noise_metadata_schedule_239_e1860: f64 = (1.0 + noise_variable_111);
            let noise_metadata_schedule_239_e1861: f64 = (2.0 * noise_metadata_schedule_239_e1860);
            let noise_metadata_schedule_239_e1863: f64 = (noise_metadata_schedule_239_e1861 * noise_variable_107);
            let noise_metadata_schedule_239_e1864: f64 = (noise_variable_100 / noise_metadata_schedule_239_e1863);
            let noise_metadata_schedule_239_e1865: f64 = (noise_metadata_schedule_239_e1855 - noise_metadata_schedule_239_e1864);
            noise_variable_0 = noise_metadata_schedule_239_e1865;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_242_e1883: f64 = (-noise_variable_112);
            let noise_metadata_schedule_242_e1886: f64 = (noise_variable_0 * noise_variable_114);
            let noise_metadata_schedule_242_e1888: f64 = (noise_metadata_schedule_242_e1886 + noise_variable_187);
            let noise_metadata_schedule_242_e1889: f64 = (noise_metadata_schedule_242_e1883 * noise_metadata_schedule_242_e1888);
            noise_variable_190 = noise_metadata_schedule_242_e1889;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_243_e1892: f64 = if params.p22 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_255 = noise_metadata_schedule_243_e1892;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_244_e1902,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_244_e1896: f64 = (params.p21 * noise_variable_6);
        let noise_metadata_schedule_244_e1899: f64 = (noise_variable_157 * noise_variable_175);
        let noise_metadata_schedule_244_e1900: f64 = (noise_metadata_schedule_244_e1896 / noise_metadata_schedule_244_e1899);
        (noise_metadata_schedule_244_e1900,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_244_e1902;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_247_e1920,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_247_e1918: f64 = (noise_variable_0 * noise_variable_114);
        (noise_metadata_schedule_247_e1918,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_247_e1920;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_250_e1941,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_250_e1937: f64 = (-noise_variable_138);
        let noise_metadata_schedule_250_e1939: f64 = (noise_metadata_schedule_250_e1937 - noise_variable_165);
        (noise_metadata_schedule_250_e1939,)
    } else {
        (noise_variable_141,)
    }
};
            noise_variable_141 = noise_metadata_schedule_250_e1941;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_251_e1948,) = {
    if (noise_variable_255 == 0.0) {
        let noise_metadata_schedule_251_e1946: f64 = (noise_variable_37 / noise_variable_47);
        (noise_metadata_schedule_251_e1946,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_251_e1948;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_254_e1990,) = {
    if (noise_variable_255 == 0.0) {
        let noise_metadata_schedule_254_e1980: f64 = (-noise_variable_138);
        let noise_metadata_schedule_254_e1985: f64 = (noise_variable_39 * noise_variable_187);
        let noise_metadata_schedule_254_e1986: f64 = (noise_variable_190 + noise_metadata_schedule_254_e1985);
        let noise_metadata_schedule_254_e1987: f64 = (noise_variable_0 * noise_metadata_schedule_254_e1986);
        let noise_metadata_schedule_254_e1988: f64 = (noise_metadata_schedule_254_e1980 + noise_metadata_schedule_254_e1987);
        (noise_metadata_schedule_254_e1988,)
    } else {
        (noise_variable_141,)
    }
};
            noise_variable_141 = noise_metadata_schedule_254_e1990;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_255_e1992: f64 = (-noise_variable_62);
            let noise_metadata_schedule_255_e1995: f64 = (4.0 * noise_variable_15);
            let noise_metadata_schedule_255_e1997: f64 = (noise_metadata_schedule_255_e1995 * noise_variable_72);
            let noise_metadata_schedule_255_e2000: f64 = (noise_variable_61 + noise_variable_5);
            let noise_metadata_schedule_255_e2002: f64 = (noise_metadata_schedule_255_e2000 + noise_variable_27);
            let noise_metadata_schedule_255_e2003: f64 = (noise_metadata_schedule_255_e1997 * noise_metadata_schedule_255_e2002);
            let noise_metadata_schedule_255_e2004: f64 = (noise_metadata_schedule_255_e1992 / noise_metadata_schedule_255_e2003);
            noise_variable_0 = noise_metadata_schedule_255_e2004;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_258_e2013: f64 = (noise_variable_0 * noise_variable_114);
            noise_variable_171 = noise_metadata_schedule_258_e2013;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_261_e2040: f64 = (noise_variable_171 + noise_variable_141);
            let noise_metadata_schedule_261_e2042: f64 = (noise_metadata_schedule_261_e2040 * noise_variable_86);
            let noise_metadata_schedule_261_e2044: f64 = (noise_metadata_schedule_261_e2042 + noise_variable_118);
            let noise_metadata_schedule_261_e2046: f64 = (noise_metadata_schedule_261_e2044 - noise_variable_135);
            let noise_metadata_schedule_261_e2047: f64 = (noise_variable_16 * noise_metadata_schedule_261_e2046);
            noise_variable_18 = noise_metadata_schedule_261_e2047;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_294_e2289,) = {
    if (params.p1 != 0.0) {
        let noise_metadata_schedule_294_e2283: f64 = (4.0 * 1.3806226e-23);
        let noise_metadata_schedule_294_e2285: f64 = (noise_metadata_schedule_294_e2283 * noise_variable_49);
        let noise_metadata_schedule_294_e2287: f64 = (noise_metadata_schedule_294_e2285 * noise_variable_152);
        (noise_metadata_schedule_294_e2287,)
    } else {
        (noise_variable_260,)
    }
};
            noise_variable_260 = noise_metadata_schedule_294_e2289;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_295_e2305,) = {
    if (params.p1 != 0.0) {
        let noise_metadata_schedule_295_e2293: f64 = (params.p42 * noise_variable_18);
        let noise_metadata_schedule_295_e2295: f64 = (noise_metadata_schedule_295_e2293 * noise_variable_18);
        let noise_metadata_schedule_295_e2298: f64 = (noise_variable_192 * params.p8);
        let noise_metadata_schedule_295_e2300: f64 = (noise_metadata_schedule_295_e2298 * noise_variable_191);
        let noise_metadata_schedule_295_e2302: f64 = (noise_metadata_schedule_295_e2300 * params.p13);
        let noise_metadata_schedule_295_e2303: f64 = (noise_metadata_schedule_295_e2295 / noise_metadata_schedule_295_e2302);
        (noise_metadata_schedule_295_e2303,)
    } else {
        (noise_variable_259,)
    }
};
            noise_variable_259 = noise_metadata_schedule_295_e2305;
        }
        match source_index {
            0 => {
                let noise_0_psd_e2951: f64 = 1.0;
                let noise_0_psd_e2952: f64 = (noise_0_psd_e2951 * noise_variable_260);
                let psd = noise_0_psd_e2952;
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
                let noise_1_psd_e2953: f64 = 1.0;
                let noise_1_psd_e2954: f64 = (noise_1_psd_e2953 * noise_variable_259);
                let psd = noise_1_psd_e2954;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
                let exponent: Option<f64> = Some(params.p41);
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
